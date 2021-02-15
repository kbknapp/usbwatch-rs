use std::{ffi::OsString, fmt::{self, Debug}};

use serde::{Deserialize, Serialize};
use yaml_rust::Yaml;

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct Devices {
    pub devices: Vec<Device>,
}

#[derive(Deserialize, Serialize, PartialEq, Default, Debug)]
pub struct Device {
    name: String,
    #[serde(default)]
    ID_MODEL: Option<String>,
    #[serde(default)]
    ID_MODEL_ENC: Option<String>,
    #[serde(default)]
    ID_MODEL_FROM_DATABASE: Option<String>,
    #[serde(default)]
    ID_MODEL_ID: Option<String>,
    #[serde(default)]
    ID_SERIAL: Option<String>,
    #[serde(default)]
    ID_SERIAL_SHORT: Option<String>,
    #[serde(default)]
    ID_VENDOR: Option<String>,
    #[serde(default)]
    ID_VENDOR_ENC: Option<String>,
    #[serde(default)]
    ID_VENDOR_FROM_DATABASE: Option<String>,
    #[serde(default)]
    ID_VENDOR_ID: Option<String>,
    #[serde(default)]
    PRODUCT: Option<String>,
}

impl Device {
    pub fn new<S: Into<String>>(name: S) -> Self {
        Self {
            name: name.into(),
            ..Default::default()
        }
    }
}

impl<'a> From<&'a Yaml> for Device {
    fn from(mut yaml: &'a Yaml) -> Self {
        let mut device = if let Some(name) = yaml["name"].as_str() {
            Device::new(name)
        } else {
            todo!("Handle Device::from<Yaml> with no name key")
        };

        yaml_str!(device, &yaml, ID_MODEL);
        yaml_str!(device, &yaml, ID_MODEL_ENC);
        yaml_str!(device, &yaml, ID_MODEL_FROM_DATABASE);
        yaml_str!(device, &yaml, ID_MODEL_ID);
        yaml_str!(device, &yaml, ID_VENDOR);
        yaml_str!(device, &yaml, ID_VENDOR_ENC);
        yaml_str!(device, &yaml, ID_VENDOR_FROM_DATABASE);
        yaml_str!(device, &yaml, ID_VENDOR_ID);
        yaml_str!(device, &yaml, ID_SERIAL);
        yaml_str!(device, &yaml, ID_SERIAL_SHORT);
        yaml_str!(device, &yaml, PRODUCT);

        device
    }
}
