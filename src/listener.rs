use std::sync::{Arc};
use std::{ffi::OsStr};
use std::future::Future;

use parking_lot::Mutex;
use tokio::sync::{broadcast, mpsc};
use tokio::signal::unix::{signal, SignalKind};
use tokio_stream::StreamExt;
use tracing::{debug, error, info, instrument};
use tokio_udev::{self, Device, EventType, Event};

use crate::{tokio_udev::DebugDevice, udev::UdevEvent, shutdown::Shutdown};

/// Udev listener state
#[derive(Debug)]
pub struct UdevListener {
    /// Broadcasts an event to all active channels.
    pub udev_event_tx: broadcast::Sender<UdevEvent>,
    pub shutdown: Shutdown,
    pub shutdown_complete_tx: mpsc::Sender<()>,
}

impl UdevListener {
    pub async fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {

        let mon = tokio_udev::MonitorBuilder::new().unwrap();
        let mut event_iter = mon.match_subsystem("usb")
                            .unwrap()
                            .listen()
                            .unwrap()
                            .filter(|e| e.is_ok())
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
                    let _ = self.udev_event_tx.send(e.into())?; // @TODO real error
                }
                Some(Err(err)) => {
                    error!(cause = ?err, "udev event error");
                }
                None => panic!("Nothing"), // @TODO return instead?
            }
        }

        Ok(())
    }
}
