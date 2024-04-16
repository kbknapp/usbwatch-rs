mod device;
mod port;

use std::result::Result as StdResult;

use clap::ValueEnum;
use serde::{
    de::{self, Deserializer},
    ser::Serializer,
    Deserialize, Serialize,
};
use strum::{Display, EnumString};
use tokio_udev::EventType;

pub use device::{UsbDevice, UsbDevices};
pub use port::{UsbPort, UsbPorts};

#[derive(Default, EnumString, Display, ValueEnum, Copy, Clone, PartialEq, Eq, Debug, Serialize)]
#[strum(ascii_case_insensitive, serialize_all = "lowercase")]
pub enum UsbEvent {
    Add,
    Bind,
    Unbind,
    Remove,
    Change,
    Unknown,
    #[default]
    All,
}

impl<'de> Deserialize<'de> for UsbEvent {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> StdResult<Self, D::Error> {
        let s = <String>::deserialize(deserializer)?;
        UsbEvent::from_str(&s, true).map_err(de::Error::custom)
    }
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
        }
    }
}

fn empty_if_none<S>(field: &Option<String>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    if let Some(s) = field {
        serializer.serialize_str(s)
    } else {
        serializer.serialize_str("")
    }
}
