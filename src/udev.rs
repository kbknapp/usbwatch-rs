use serde::Serialize;

use crate::usb::{UsbDevice, UsbEvent, UsbPort};

#[derive(Clone, PartialEq, Debug, Serialize)]
pub struct UdevEvent {
    pub event_kind: UsbEvent,
    pub device: UsbDevice,
    pub port: UsbPort,
}

impl UdevEvent {
    pub fn new(event_kind: UsbEvent) -> Self {
        Self {
            event_kind,
            device: UsbDevice::default(),
            port: UsbPort::default(),
        }
    }
}

impl From<tokio_udev::Event> for UdevEvent {
    fn from(e: tokio_udev::Event) -> Self {
        let d = e.device();
        Self {
            event_kind: e.event_type().into(),
            device: UsbDevice::from(&d),
            port: UsbPort::from(&d),
        }
    }
}
