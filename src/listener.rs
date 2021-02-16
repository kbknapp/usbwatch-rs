use std::sync::Arc;
use std::{ffi::OsStr};
use std::future::Future;

use tokio::sync::{broadcast, mpsc};
use tokio::signal::unix::{signal, SignalKind};
use tokio_stream::StreamExt;
use tracing::{debug, error, info, instrument};
use tokio_udev::{self, Device, EventType, Event};

use crate::{tokio_udev::DebugDevice, shutdown::Shutdown};

/// Udev listener state
#[derive(Debug)]
pub struct UdevListener {
    /// Broadcasts an event to all active channels.
    udev_event_tx: broadcast::Sender<Arc<tokio_udev::Device>>,
    shutdown: Shutdown,
    shutdown_complete: mpsc::Sender<()>,
}

impl UdevListener {
    pub async fn run(&mut self) -> Result<(),()> {
        let mon = tokio_udev::MonitorBuilder::new().unwrap();
        let mut event_iter = mon.match_subsystem("usb")
                            .unwrap()
                            .listen()
                            .unwrap()
                            .filter(|e| {
                                let et = e.as_ref().unwrap().event_type();
                                et == EventType::Add || et == EventType::Remove
                            })
                            .filter(|e| {
                                let d = e.as_ref().unwrap().device();
                                Some(OsStr::new("usb_interface")) != d.devtype()
                            });

        while !self.shutdown.is_shutdown() {
            let event = tokio::select! {
                res = event_iter.next() => res,
                _ = self.shutdown.recv() => return Ok(()),
            };

            match event {
                Some(Ok(e)) => {
                    let _ = self.udev_event_tx.send(Arc::new(e.device())).map_err(|_| ())?; // @TODO real error
                }
                Some(Err(err)) => {
                    error!(cause = ?err, "udev event error");
                }
                None => (), // @TODO return instead?
            }
        }

        Ok(())
    }
}

// pub async fn listen(event_handler: broadcast::Sender<Device>, filters: ListenerFilter) {
//     // When the `shutdown` future completes, we must send a shutdown
//     // message to all active connections. We use a broadcast channel for this
//     // purpose. The call below ignores the receiver of the broadcast pair, and when
//     // a receiver is needed, the subscribe() method on the sender is used to create
//     // one.
//     let (notify_shutdown, _) = broadcast::channel(1);
//     let (shutdown_complete_tx, shutdown_complete_rx) = mpsc::channel(1);

//     // Initialize the listener state
//     let mut listener = UdevListener {
//         event_tx: event_handler,
//         notify_shutdown,
//         shutdown_complete_tx,
//         shutdown_complete_rx,
//     };

//     tokio::select! {
//         _ = listener.run(filters) => {
//             info!("shutting down");
//         }
//         _ = shutdown => {
//             // The shutdown signal has been received.
//             info!("shutting down");
//         }
//     }

//     // Extract the `shutdown_complete` receiver and transmitter
//     // explicitly drop `shutdown_transmitter`. This is important, as the
//     // `.await` below would otherwise never complete.
//     let UdevListener {
//         mut shutdown_complete_rx,
//         shutdown_complete_tx,
//         notify_shutdown,
//         ..
//     } = listener;
//     // When `notify_shutdown` is dropped, all tasks which have `subscribe`d will
//     // receive the shutdown signal and can exit
//     drop(notify_shutdown);
//     // Drop final `Sender` so the `Receiver` below can complete
//     drop(shutdown_complete_tx);

//     // Wait for all active connections to finish processing. As the `Sender`
//     // handle held by the listener has been dropped above, the only remaining
//     // `Sender` instances are held by connection handler tasks. When those drop,
//     // the `mpsc` channel will close and `recv()` will return `None`.
//     let _ = shutdown_complete_rx.recv().await;
// }

// bitflags! {
//     pub struct ListenerFilter: u8 {
//         const NONE = 0;
//         const NO_USB_INTERFACE = 1;
//         const ONLY_ADD_EVENTS = 1 << 1;
//         const ONLY_REM_EVENTS = 1 << 2;
//         const ALL_EVENTS = Self::ONLY_ADD_EVENTS.bits | Self::ONLY_REM_EVENTS.bits;
//     }
// }

// impl ListenerFilter {

//     fn filter_fn(&self) -> for<'r> fn(&'r Result<tokio_udev::Event, std::io::Error>) -> bool {
//         match *self {
//             ListenerFilter::NONE => {
//                 |e| {
//                     true
//                 }
//             },
//             ListenerFilter::NO_USB_INTERFACE => {
//                 |e| {
//                     let d = e.as_ref().unwrap().device();
//                     Some(OsStr::new("usb_interface")) != d.devtype()
//                 }
//             },
//             ListenerFilter::ALL_EVENTS => {
//                 |e| {
//                     let et = e.as_ref().unwrap().event_type();
//                     et == EventType::Add || et == EventType::Remove
//                 }
//             }
//             ListenerFilter::ONLY_ADD_EVENTS => {
//                 |e| {
//                     let et = e.as_ref().unwrap().event_type();
//                     et == EventType::Add
//                 }
//             }
//             ListenerFilter::ONLY_REM_EVENTS => {
//                 |e| {
//                     let et = e.as_ref().unwrap().event_type();
//                     et == EventType::Remove
//                 }
//             }
//             _ => panic!("Invalid ListenerFilter")
//         }
//     }
// }

// impl Iterator for ListenerFilter {
//     type Item = ListenerFilter;

//     fn next(&mut self) -> Option<Self::Item> {
//         if *self & ListenerFilter::NONE == ListenerFilter::NONE {
//             return None;
//         }
//         macro_rules! handle_bitflag {
//             ($_self:ident, $variant:ident) => {
//                 if *$_self & ListenerFilter::$variant == ListenerFilter::$variant {
//                     $_self.remove(ListenerFilter::$variant);
//                     return Some(ListenerFilter::$variant);
//                 }
//             }
//         }

//         handle_bitflag!(self, NO_USB_INTERFACE);
//         handle_bitflag!(self, ALL_EVENTS);
//         handle_bitflag!(self, ONLY_ADD_EVENTS);
//         handle_bitflag!(self, ONLY_REM_EVENTS);

//         None
//     }
// }
