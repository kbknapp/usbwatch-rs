use std::path::PathBuf;

use clap::Args;
use tokio::{
    signal::unix::{signal, SignalKind},
    sync::{broadcast, mpsc},
};

use crate::{
    cli::{Cmd, ForObject},
    ctx::Ctx,
    listener::UdevListener,
    printer::OutFormat,
    shutdown::Shutdown,
    udev::UdevEvent,
    usb::UsbEvent,
};

/// Listen for events and display them to stdout
#[derive(Args, Clone, Debug)]
pub struct UsbWatchListen {
    /// Only display KIND of objects
    #[arg(long, short, value_enum, value_name = "KIND", default_value = "all")]
    pub only: ForObject,

    /// Only display KIND of events
    #[arg(long, short, value_enum, value_name = "KIND", default_value = "all")]
    pub event: UsbEvent,

    /// Save the even information to a file at the following path
    #[arg(long, short = 'O', value_name = "PATH")]
    pub output: Option<PathBuf>,

    /// Only listen for N events and exit (0 is infinite)
    #[arg(long, short, value_name = "N", default_value = "0")]
    pub num_events: usize,

    /// Display output in format
    #[arg(
        short = 'F',
        long,
        value_enum,
        value_name = "FORMAT",
        default_value = "raw"
    )]
    pub format: OutFormat,
}

impl Cmd for UsbWatchListen {
    fn run(&self, _ctx: &mut Ctx) -> anyhow::Result<()> {
        tokio::runtime::Builder::new_current_thread()
            .enable_io()
            .build()
            .unwrap()
            .block_on(async {
                cli_debugln!("Creating signal listeners");
                let mut sigint = signal(SignalKind::interrupt()).unwrap();
                let mut sighup = signal(SignalKind::hangup()).unwrap();

                loop {
                    let (udev_event_tx, udev_event_rx) = broadcast::channel(32); // 32 picked by fair diceroll
                    let (notify_shutdown, _) = broadcast::channel(1);
                    let (shutdown_complete_tx, shutdown_complete_rx) = mpsc::channel(1);

                    let mut listener = UdevListener {
                        shutdown: Shutdown::new(notify_shutdown.subscribe()),
                        shutdown_complete_tx: shutdown_complete_tx.clone(),
                        udev_event_tx: udev_event_tx.clone(),
                    };

                    let mut handler = Handler {
                        notify_shutdown,
                        shutdown_complete_tx,
                        shutdown_complete_rx,
                        udev_event_rx,
                        args: self,
                    };

                    tokio::select! {
                        res = listener.run() => {
                            if let Err(err) = res {
                                cli_error!("listener failed; {}", err);
                            }
                        }
                        res = handler.run() => {
                            if let Err(err) = res {
                                cli_error!("handler failed; {}", err);
                            }
                        }
                        _ = sighup.recv() => {
                            // SIGHUP has been received.
                            cli_println!("SIGHUP received; reloading");
                        }
                        _ = sigint.recv() => {
                            // SIGINT has been received.
                            cli_println!("SIGINT received; shutting down");
                            break;
                        }
                    };

                    let Handler {
                        mut shutdown_complete_rx,
                        shutdown_complete_tx,
                        notify_shutdown,
                        ..
                    } = handler;

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

        Ok(())
    }
}

struct Handler<'a> {
    notify_shutdown: broadcast::Sender<()>,
    shutdown_complete_tx: mpsc::Sender<()>,
    shutdown_complete_rx: mpsc::Receiver<()>,
    udev_event_rx: broadcast::Receiver<UdevEvent>,
    args: &'a UsbWatchListen,
}

impl<'a> Handler<'a> {
    async fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let shutdown = Shutdown::new(self.notify_shutdown.subscribe());
        tokio::pin!(shutdown);

        while !shutdown.is_shutdown() {
            let event = tokio::select! {
                res = self.udev_event_rx.recv() => res?, // @TODO: add real error
                _ = shutdown.recv() => {
                    return Ok(());
                }
            };
            cli_println!("Received udev event");

            cli_debug!("Checking if event type qualifies for printing...");
            if self.args.event == event.event_kind || self.args.event == UsbEvent::All {
                cli_debugln!("Yes");
                self.print(event).await;
            } else {
                cli_debugln!("No");
            }
        }

        Ok(())
    }

    async fn print(&self, udev_dev: UdevEvent) {
        match self.args.format {
            OutFormat::Raw => {
                if self.args.only == ForObject::Ports || self.args.only == ForObject::All {
                    cli_println!("{:#?}", udev_dev.port);
                }
                if self.args.only == ForObject::Devices || self.args.only == ForObject::All {
                    cli_println!("{:#?}", udev_dev.device);
                }
            }
            OutFormat::Yaml => {
                if self.args.only == ForObject::Ports || self.args.only == ForObject::All {
                    cli_print!("---\nports:\n  - ");
                    let yaml = serde_yaml::to_string(&udev_dev.port).unwrap();
                    for (i, line) in yaml.lines().skip(1).enumerate() {
                        if i == 0 {
                            cli_println!("{}", line);
                        } else {
                            cli_println!("    {}", line);
                        }
                    }
                }
                if self.args.only == ForObject::Devices || self.args.only == ForObject::All {
                    cli_print!("---\ndevices:\n  - ");
                    let yaml = serde_yaml::to_string(&udev_dev.device).unwrap();
                    for (i, line) in yaml.lines().skip(1).enumerate() {
                        if i == 0 {
                            cli_println!("{}", line);
                        } else {
                            cli_println!("    {}", line);
                        }
                    }
                }
            }
        }
    }
}
