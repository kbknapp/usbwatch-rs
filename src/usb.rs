mod device;
mod port;

use std::str::FromStr;

use serde::{Serialize, Deserialize, ser::Serializer};
use clap::Clap;
use tokio_udev::EventType;

pub use device::{UsbDevice, UsbDevices};
pub use port::{UsbPort, UsbPorts};

#[derive(Clap, Copy, Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum UsbEvent {
    Add,
    Bind,
    Unbind,
    Remove,
    Change,
    Unknown,
    All,
}

impl From<tokio_udev::EventType> for UsbEvent {
    fn from(e: tokio_udev::EventType) -> Self {
        match e {
            EventType::Add => UsbEvent::Add,
            EventType::Remove => UsbEvent::Remove,
            EventType::Change => UsbEvent::Change,
            EventType::Unknown => UsbEvent::Unknown,
            EventType::Bind => UsbEvent::Bind,
            EventType::Unbind => UsbEvent::Unbind,
            _ => panic!("Unsupported event type") // @TODO maybe dont panic
        }
    }
}

impl FromStr for UsbEvent {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &*s.to_ascii_lowercase() {
            "add" => Ok(UsbEvent::Add),
            "remove" => Ok(UsbEvent::Remove),
            "all" => Ok(UsbEvent::All),
            _ => Err("Invalid event type"),
        }
    }
}

fn empty_if_none<S>(field: &Option<String>, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
    if let Some(s) = field {
        serializer.serialize_str(s)
    } else {
        serializer.serialize_str("")
    }
}
