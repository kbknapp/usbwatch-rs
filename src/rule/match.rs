use std::{fs::File, ffi::OsString, fmt::{self, Debug}};

use serde::{Deserialize, Serialize, de::Deserializer};
use yaml_rust::Yaml;

use crate::{
    udev::UdevEvent,
    usb::{UsbDevice, UsbDevices, UsbEvent, UsbPort, UsbPorts},
};

#[derive(Serialize, PartialEq, Debug)]
pub struct Match {
    on: UsbEvent,
    devices: Vec<UsbDevice>,
    ports: Vec<UsbPort>,
}

impl Match {
    fn new(event: UsbEvent) -> Self {
        Self {
            on: event,
            devices: Vec::new(),
            ports: Vec::new(),
        }
    }
}

impl<'a> From<&'a Yaml> for Match {
    fn from(yaml: &'a Yaml) -> Self {
        let mut m = if let Some(on_event) = yaml["on"].as_str() {
            Match::new(on_event.parse().unwrap())
        } else {
            todo!("Handle Match::from<Yaml> with no on key")
        };

        if let Some(devices) = yaml["devices"].as_vec() {
            for d in devices {
                if let Some(path) = d["include_devices"].as_str() {
                    let file = File::open(path).unwrap();
                    let mut devs: UsbDevices = serde_yaml::from_reader(file).unwrap();
                    m.devices.append(&mut devs.devices);
                } else if d["name"].as_str().is_some() {
                    m.devices.push(UsbDevice::from(d));
                } else if let Some(name) = d.as_str() {
                    m.devices.push(UsbDevice::new(name));
                    // @TODO: will need to handle lookup of name / merge
                } else {
                    todo!("Handle deserialize devices with bad key")
                }
            }
        }

        if let Some(ports) = yaml["ports"].as_vec() {
            for p in ports {
                if let Some(path) = p["include_ports"].as_str() {
                    let file = File::open(path).unwrap();
                    let mut ports: UsbPorts = serde_yaml::from_reader(file).unwrap();
                    m.ports.append(&mut ports.ports);
                } else if p["name"].as_str().is_some() {
                    m.ports.push(UsbPort::from(p));
                } else if let Some(name) = p.as_str() {
                    m.ports.push(UsbPort::new(name));
                    // @TODO: will need to handle lookup of name / merge
                } else {
                    todo!("Handle deserialize ports with bad key")
                }
            }
        }

        m
    }
}
