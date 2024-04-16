use std::path::PathBuf;

use clap::Args;
use tokio::{
    io::AsyncWriteExt,
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
}

impl Cmd for UsbWatchListen {
    fn update_ctx(&self, ctx: &mut Ctx) -> anyhow::Result<()> {
        if self.num_events == 0 {
            ctx.num_events = usize::MAX;
        } else {
            ctx.num_events = self.num_events;
        }

        Ok(())
    }

    fn run(&self, ctx: &mut Ctx) -> anyhow::Result<()> {
        tokio::runtime::Builder::new_current_thread()
            .enable_io()
            .build()
            .unwrap()
            .block_on(async {
                cli_debugln!("Creating signal listeners");
                let mut sigint = signal(SignalKind::interrupt()).unwrap();
                let mut sighup = signal(SignalKind::hangup()).unwrap();
                let mut end = false;

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
                    };

                    tokio::select! {
                        res = listener.run() => {
                            if let Err(err) = res {
                                cli_error!("listener failed; {}", err);
                            }
                        }
                        res = handler.run(self, ctx) => {
                            if let Err(err) = res {
                                cli_error!("handler failed; {}", err);
                            } else {
                                end = true;
                            }
                        }
                        _ = sighup.recv() => {
                            // SIGHUP has been received.
                            cli_eprintln!("SIGHUP received; reloading");
                        }
                        _ = sigint.recv() => {
                            // SIGINT has been received.
                            cli_eprintln!("SIGINT received; shutting down");
                            end = true;
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

                    if end {
                        break;
                    }
                }
            });

        Ok(())
    }
}

struct Handler {
    notify_shutdown: broadcast::Sender<()>,
    shutdown_complete_tx: mpsc::Sender<()>,
    shutdown_complete_rx: mpsc::Receiver<()>,
    udev_event_rx: broadcast::Receiver<UdevEvent>,
}

impl Handler {
    async fn run(
        &mut self,
        args: &UsbWatchListen,
        ctx: &Ctx,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let shutdown = Shutdown::new(self.notify_shutdown.subscribe());
        tokio::pin!(shutdown);

        let mut count = 0;
        let mut ports = vec![];
        let mut devices = vec![];

        while count < ctx.num_events && !shutdown.is_shutdown() {
            let event = tokio::select! {
                res = self.udev_event_rx.recv() => res?, // @TODO: add real error
                _ = shutdown.recv() => {
                    return Ok(());
                }
            };
            cli_debugln!("Received udev event");

            cli_debug!("Checking if event type qualifies for printing...");
            if args.event == event.event_kind || args.event == UsbEvent::All {
                cli_debugln!("Yes");
                if args.output.is_some() {
                    cli_println!("Recvied 1 event");
                    ports.push(event.port);
                    devices.push(event.device);
                } else {
                    print_event(event, args, ctx).await;
                }
                count += 1;
            } else {
                cli_debugln!("No");
            }
        }

        if let Some(path) = &args.output {
            cli_println!("Writing output to {}...", path.display());

            let mut f = String::new();

            f.push_str("---\n");
            if args.only == ForObject::Ports || args.only == ForObject::All {
                f.push_str("ports:\n");
                for port in ports {
                    f.push_str("  - ");
                    let yaml = serde_yaml::to_string(&port).unwrap();
                    for (i, line) in yaml.lines().skip(1).enumerate() {
                        if i != 0 {
                            f.push_str("    ");
                        }
                        f.push_str(line);
                        f.push('\n');
                    }
                }
            }
            if args.only == ForObject::Devices || args.only == ForObject::All {
                f.push_str("devices:\n");
                for device in devices {
                    f.push_str("  - ");
                    let yaml = serde_yaml::to_string(&device).unwrap();
                    for (i, line) in yaml.lines().skip(1).enumerate() {
                        if i != 0 {
                            f.push_str("    ");
                        }
                        f.push_str(line);
                        f.push('\n');
                    }
                }
            }
            f.push('\n');

            let mut file = tokio::fs::File::create(path).await?;
            file.write_all(f.as_bytes()).await?;
        }

        Ok(())
    }
}

async fn print_event(udev_dev: UdevEvent, args: &UsbWatchListen, ctx: &Ctx) {
    match ctx.format {
        OutFormat::Raw => {
            if args.only == ForObject::Ports || args.only == ForObject::All {
                cli_println!("{:#?}", udev_dev.port);
            }
            if args.only == ForObject::Devices || args.only == ForObject::All {
                cli_println!("{:#?}", udev_dev.device);
            }
        }
        OutFormat::Yaml => {
            if args.only == ForObject::Ports || args.only == ForObject::All {
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
            if args.only == ForObject::Devices || args.only == ForObject::All {
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
