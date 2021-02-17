use std::sync::{Arc};
use std::{ffi::OsStr};
use std::future::Future;
use std::{path::PathBuf, ffi::OsString, fmt::{self, Debug}};
use std::io::{self, Write};
use std::os::unix::fs::PermissionsExt;
use std::process::Stdio;

use tracing::{self, debug, warn, error, info, trace, instrument};
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::process::Command;
use tempfile::NamedTempFile;
use parking_lot::Mutex;
use tokio::sync::{broadcast, mpsc};
use tokio::signal::unix::{signal, SignalKind};
use tokio_stream::StreamExt;
use tokio_udev::{self, EventType};

use crate::{
    cli::{RunArgs},
    listener::{UdevListener },
    tokio_udev::DebugDevice,
    shutdown::Shutdown,
    udev::UdevEvent,
    state::State,
    usb::UsbEvent,
};

struct Handler {
    notify_shutdown: broadcast::Sender<()>,
    shutdown_complete_tx: mpsc::Sender<()>,
    shutdown_complete_rx: mpsc::Receiver<()>,
    udev_event_rx: broadcast::Receiver<UdevEvent>,
    state: Arc<Mutex<State>>,
}


async fn exec(cmd: String, shell: PathBuf) -> Result<(), ()> {
    trace!("Inside exec");

    debug!("Executing command");
    let mut child = Command::new(&shell)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn child process");

    {
        let stdin = child.stdin.as_mut().expect("Failed to open stdin");
        stdin.write_all(cmd.as_bytes()).await.expect("Failed to write to stdin");
    }

    info!("Executing command");
    debug!("Waiting for child to exit");
    let status = child.wait().await.unwrap();
    if status.success() {
        info!("Command completed successfully");
    } else {
        info!("Command completed with error code {:?}", code = status.code());
    }
    Ok(())
}

impl Handler {
    async fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        trace!("Inside Handler::run");

        let shutdown = Shutdown::new(self.notify_shutdown.subscribe());
        tokio::pin!(shutdown);

        while !shutdown.is_shutdown() {
            let event = tokio::select! {
                res = self.udev_event_rx.recv() => res?, // @TODO: add real error
                _ = shutdown.recv() => {
                    info!("Shutting down handler");
                    return Ok(());
                }
            };

            info!(event = ?event.event_kind, "Received udev event");

            {
                debug!("Updating State");
                let mut s = self.state.lock();
                s.add_port(event.port.clone());
                if event.event_kind == UsbEvent::Add {
                    debug!("Adding");
                    s.add_and_slot_device(event.device.clone(), event.port.clone());
                } else if event.event_kind == UsbEvent::Remove {
                    debug!("Removing");
                    s.rm_and_unslot_device(event.device.clone());
                }

                for r in &s.rules {
                    if r.matches_udev_event(&event) {
                        info!(rule = ?r.name, "Found matching rule");
                        let cmd = r.command.clone();
                        let shell = r.command_shell.clone();
                        tokio::spawn(exec(cmd, shell));
                    }
                }
            }
        }

        Ok(())
    }
}

pub fn run(a: RunArgs) {
    tokio::runtime::Builder::new_current_thread()
        .enable_io()
        .build()
        .unwrap()
        .block_on(async {
            debug!("Creating signal listeners");
            //let (event_tx, event_rx) = broadcast::channel(32);
            let mut sigint = signal(SignalKind::interrupt()).unwrap();
            let mut sighup = signal(SignalKind::hangup()).unwrap();

            debug!("Creating blank State");
            let state = Arc::new(Mutex::new(State::new()));

            loop {

                let (udev_event_tx, udev_event_rx) = broadcast::channel(32); // 32 picked by fair diceroll
                let state = state.clone();
                {
                    let mut s = state.lock();
                    if let Some(ref p) = a.devices {
                        info!("Loading devices from {:?}", p);
                        s.devices_from_file(p);
                    }
                    if let Some(ref p) = a.ports{
                        info!("Loading ports from {:?}", p);
                        s.ports_from_file(p);
                    }
                    info!("Loading rules from {:?}", a.rules);
                    s.rules_from_file(a.rules.clone());
                }

                let (notify_shutdown, _) = broadcast::channel(1);
                let (shutdown_complete_tx, mut shutdown_complete_rx) = mpsc::channel(1);

                let mut listener = UdevListener {
                    shutdown: Shutdown::new(notify_shutdown.subscribe()),
                    shutdown_complete_tx: shutdown_complete_tx.clone(),
                    udev_event_tx: udev_event_tx.clone(),
                };

                let mut handler = Handler {
                    notify_shutdown,
                    shutdown_complete_tx: shutdown_complete_tx,
                    shutdown_complete_rx: shutdown_complete_rx,
                    udev_event_rx,
                    state: state.clone(),
                };

                tokio::select! {
                    res = listener.run() => {
                        if let Err(err) = res {
                            error!(cause = %err, "listener failed");
                        }
                    }
                    res = handler.run() => {
                        if let Err(err) = res {
                            error!(cause = %err, "handler failed");
                        }
                    }
                    _ = sighup.recv() => {
                        info!("SIGHUP received; reloading");
                    }
                    _ = sigint.recv() => {
                        // SIGINT has been received.
                        info!("SIGINT received; shutting down");
                        break;
                    }
                };

                let Handler {
                    mut shutdown_complete_rx,
                    shutdown_complete_tx,
                    notify_shutdown,
                    ..
                } = handler ;

                drop(notify_shutdown);
                drop(shutdown_complete_tx);

                let UdevListener {
                    shutdown_complete_tx,
                    shutdown,
                    ..
                } = listener;

                drop(shutdown);
                drop(shutdown_complete_tx);

                let _ = shutdown_complete_rx.recv().await;

            }
        });
}
