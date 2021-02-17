
use tracing::{self, debug, warn, error, info, trace, instrument};
use tokio::sync::{broadcast, mpsc};
use tokio::signal::unix::{signal, SignalKind};

use crate::{
    cli::{OutFormat, ListenArgs, ListenForObject},
    listener::UdevListener,
    shutdown::Shutdown,
    udev::UdevEvent,
    usb::UsbEvent,
};

struct Handler {
    notify_shutdown: broadcast::Sender<()>,
    shutdown_complete_tx: mpsc::Sender<()>,
    shutdown_complete_rx: mpsc::Receiver<()>,
    udev_event_rx: broadcast::Receiver<UdevEvent>,
    args: ListenArgs,
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
                    return Ok(());
                }
            };
            info!("Received udev event");

            debug!("Checking if event type qualifies for printing");
            if self.args.events == event.event_kind || self.args.events == UsbEvent::All {
                debug!("Yes");
                self.print(event).await;
            } else {
                debug!("No");
            }
        }

        Ok(())
    }

    async fn print(&self, udev_dev: UdevEvent) {
        use OutFormat::*;

        match self.args.format {
            Raw => {
                if self.args.object == ListenForObject::Ports || self.args.object ==  ListenForObject::All {
                    println!("{:#?}", udev_dev.port);
                }
                if self.args.object == ListenForObject::Devices || self.args.object ==  ListenForObject::All {
                    println!("{:#?}", udev_dev.device);
                }
            },
            Yaml =>{
                if self.args.object == ListenForObject::Ports || self.args.object ==  ListenForObject::All {
                    println!("{}", serde_yaml::to_string(&udev_dev.port).unwrap());
                }
                if self.args.object == ListenForObject::Devices || self.args.object ==  ListenForObject::All {
                    println!("{}", serde_yaml::to_string(&udev_dev.device).unwrap());
                }
            },
        }
    }

}

#[tracing::instrument]
pub fn run(a: ListenArgs) {
    tokio::runtime::Builder::new_current_thread()
        .enable_io()
        .build()
        .unwrap()
        .block_on(async {
            debug!("Creating signal listeners");
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
                    shutdown_complete_tx: shutdown_complete_tx,
                    shutdown_complete_rx: shutdown_complete_rx,
                    udev_event_rx,
                    args: a,
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
                        // SIGHUP has been received.
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
