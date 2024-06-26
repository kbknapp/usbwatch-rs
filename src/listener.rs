use std::ffi::OsStr;

use tokio::sync::{broadcast, mpsc};
use tokio_stream::StreamExt;
use tokio_udev::{AsyncMonitorSocket, EventType};
use tracing::error;

use crate::{shutdown::Shutdown, udev::UdevEvent};

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
        let mut event_iter = AsyncMonitorSocket::new(
            tokio_udev::MonitorBuilder::new()
                .unwrap()
                .match_subsystem("usb")
                .unwrap()
                .listen()
                .unwrap(),
        )
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
                    let _ = self.udev_event_tx.send(e.into())?; // @TODO real
                                                                // error
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
