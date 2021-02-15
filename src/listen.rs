use std::{fs, ffi::OsStr};
use std::future::Future;

use futures_core::stream::Stream;
use tokio::sync::{broadcast, mpsc, Semaphore};
use tokio::time::{self, Duration};
use tokio::signal;
use tokio_stream::StreamExt;
use tracing::{debug, error, info, instrument};
use tokio_udev::{self, EventType};

use crate::{cli::ListenArgs, tokio_udev::DebugDevice};
/// Udev listener state. Created in the `run` call. It includes a `run` method
/// which performs the TCP listening and initialization of per-connection state.
#[derive(Debug)]
struct Listener {
    /// Broadcasts a shutdown signal to all active connections.
    ///
    /// The initial `shutdown` trigger is provided by the `run` caller. The
    /// server is responsible for gracefully shutting down active connections.
    /// When a connection task is spawned, it is passed a broadcast receiver
    /// handle. When a graceful shutdown is initiated, a `()` value is sent via
    /// the broadcast::Sender. Each active connection receives it, reaches a
    /// safe terminal state, and completes the task.
    notify_shutdown: broadcast::Sender<()>,

    /// Used as part of the graceful shutdown process to wait for client
    /// connections to complete processing.
    ///
    /// Tokio channels are closed once all `Sender` handles go out of scope.
    /// When a channel is closed, the receiver receives `None`. This is
    /// leveraged to detect all connection handlers completing. When a
    /// connection handler is initialized, it is assigned a clone of
    /// `shutdown_complete_tx`. When the listener shuts down, it drops the
    /// sender held by this `shutdown_complete_tx` field. Once all handler tasks
    /// complete, all clones of the `Sender` are also dropped. This results in
    /// `shutdown_complete_rx.recv()` completing with `None`. At this point, it
    /// is safe to exit the server process.
    shutdown_complete_rx: mpsc::Receiver<()>,
    shutdown_complete_tx: mpsc::Sender<()>,
}

impl Listener {
    async fn run(&mut self)  {
        let mon = tokio_udev::MonitorBuilder::new().unwrap();
        let mut listen = mon.match_subsystem("usb")
                            .unwrap()
                            .listen()
                            .unwrap()
                            .filter(|e| {
                                let d = e.as_ref().unwrap().device();
                                Some(OsStr::new("usb_interface")) != d.devtype()
                            })
                            .filter(|e| {
                                let et = e.as_ref().unwrap().event_type();
                                et == EventType::Add || et == EventType::Remove
                            });

        loop {
            match listen.next().await {
                Some(Ok(e)) => {
                    let dd = DebugDevice::new(e.device());
                    println!("event: {:?} for {:#?}", e.event_type(), dd);
                }
                Some(Err(err)) => {
                    error!(cause = ?err, "connection error");
                }
                None => (),
            }
        }
    }
}

pub async fn listen() {
    let shutdown = signal::ctrl_c();
    // When the `shutdown` future completes, we must send a shutdown
    // message to all active connections. We use a broadcast channel for this
    // purpose. The call below ignores the receiver of the broadcast pair, and when
    // a receiver is needed, the subscribe() method on the sender is used to create
    // one.
    let (notify_shutdown, _) = broadcast::channel(1);
    let (shutdown_complete_tx, shutdown_complete_rx) = mpsc::channel(1);

    // Initialize the listener state
    let mut listener = Listener {
        notify_shutdown,
        shutdown_complete_tx,
        shutdown_complete_rx,
    };

    tokio::select! {
        _ = listener.run() => {
            info!("shutting down");
        }
        _ = shutdown => {
            // The shutdown signal has been received.
            info!("shutting down");
        }
    }

    // Extract the `shutdown_complete` receiver and transmitter
    // explicitly drop `shutdown_transmitter`. This is important, as the
    // `.await` below would otherwise never complete.
    let Listener {
        mut shutdown_complete_rx,
        shutdown_complete_tx,
        notify_shutdown,
        ..
    } = listener;
    // When `notify_shutdown` is dropped, all tasks which have `subscribe`d will
    // receive the shutdown signal and can exit
    drop(notify_shutdown);
    // Drop final `Sender` so the `Receiver` below can complete
    drop(shutdown_complete_tx);

    // Wait for all active connections to finish processing. As the `Sender`
    // handle held by the listener has been dropped above, the only remaining
    // `Sender` instances are held by connection handler tasks. When those drop,
    // the `mpsc` channel will close and `recv()` will return `None`.
    let _ = shutdown_complete_rx.recv().await;
}
