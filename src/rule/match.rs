use std::{fmt::Debug, fs::File};

use serde::Serialize;
use tracing::{self, debug, trace};
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
    ignore_devices: Vec<usize>,
}

impl Match {
    fn new(event: UsbEvent) -> Self {
        Self {
            on: event,
            devices: Vec::new(),
            ports: Vec::new(),
            ignore_devices: Vec::new(),
        }
    }

    pub fn device_ignored(&self, device: &UsbDevice) -> bool {
        trace!(device = ?device, "Inside Match::device_ignored");
        for (i, dev) in self.devices.iter().enumerate() {
            if dev == device {
                debug!(ignored = ?self.ignore_devices.contains(&i), "Found device");
                return self.ignore_devices.contains(&i);
            }
        }

        false
    }

    pub fn matches_port(&self, port: &UsbPort) -> bool {
        trace!(port = ?port, "Inside Match::matches_port");
        trace!(ret = ?(self.ports.is_empty() || self.ports.contains(port)), "Returning");
        self.ports.is_empty() || self.ports.contains(port)
    }

    pub fn matches_device(&self, device: &UsbDevice) -> bool {
        trace!(device = ?device, "Inside Match::matches_device");
        trace!(ret = ?(self.devices.is_empty() || (self.devices.contains(device) && ! self.device_ignored(device))), "Returning");
        // If all devices are ignored, then our rule is an "any device except these ..."
        (self.devices.is_empty()
            || (self.devices.len() == self.ignore_devices.len() && !self.device_ignored(device)))
            || (self.devices.contains(device) && !self.device_ignored(device))
    }

    pub fn matches_usb_event(&self, event: &UsbEvent) -> bool {
        trace!(event = ?event, "Inside Match::matches_usb_event");
        trace!(ret = ?(&self.on == event), "Returning");
        &self.on == event
    }

    pub fn matches_udev_event(&self, event: &UdevEvent) -> bool {
        self.matches_usb_event(&event.event_kind)
            && self.matches_port(&event.port)
            && self.matches_device(&event.device)
    }
}

impl<'a> From<&'a Yaml> for Match {
    fn from(yaml: &'a Yaml) -> Self {
        trace!("Inside Match::from::<Yaml>");
        let mut m = if let Some(on_event) = yaml["on"].as_str() {
            Match::new(on_event.parse().unwrap())
        } else {
            todo!("Handle Match::from<Yaml> with no on key")
        };

        if let Some(devices) = yaml["devices"].as_vec() {
            trace!("Loading devices: array");
            let mut to_ignore: Vec<String> = Vec::new();
            for d in devices {
                if let Some(path) = d["include_devices"].as_str() {
                    debug!(path = ?path, "Including devices from path");
                    let file = File::open(path).unwrap();
                    let mut devs: UsbDevices = serde_yaml::from_reader(file).unwrap();
                    debug!(devices = ?devs, "Found devices");
                    m.devices.append(&mut devs.devices);
                } else if let Some(path) = d["exclude_devices"].as_str() {
                    debug!(path = ?path, "Excluding devices from path");
                    let file = File::open(path).unwrap();
                    let mut devs: UsbDevices = serde_yaml::from_reader(file).unwrap();
                    debug!(devices = ?devs, "Found devices");
                    let pre = m.devices.len();
                    let num_devices = devs.devices.len();
                    // Add the devices to be able to match against their info
                    m.devices.append(&mut devs.devices);
                    for i in pre..(pre + num_devices - 1) {
                        debug!(i = %i, "Ignoring device index");
                        m.ignore_devices.push(i);
                    }
                } else if d["name"].as_str().is_some() {
                    debug!(name = ?d, "Including device inline");
                    m.devices.push(UsbDevice::from(d));
                } else if let Some(name) = d.as_str() {
                    debug!(name = ?d, "Including device by name");
                    if name.starts_with('!') {
                        debug!("Device is to be ignored");
                        to_ignore.push(name.to_string());
                    } else {
                        m.devices.push(UsbDevice::new(name));
                    }
                } else {
                    todo!("Handle deserialize devices with bad key")
                }
            }
            for ignore_dev in to_ignore.into_iter() {
                if let Some(ignore_dev) = ignore_dev.strip_prefix('!') {
                    trace!(ignored_dev = %ignore_dev, "Ignoring device");
                    for (i, d) in m.devices.iter().enumerate() {
                        if let Some(name) = d.name.as_ref() {
                            if name == ignore_dev {
                                m.ignore_devices.push(i);
                                break;
                            }
                        }
                    }
                }
            }
        }

        if let Some(ports) = yaml["ports"].as_vec() {
            trace!("Loading ports: array");
            for p in ports {
                if let Some(path) = p["include_ports"].as_str() {
                    debug!(path = ?path, "Including port from path");
                    let file = File::open(path).unwrap();
                    let mut ports: UsbPorts = serde_yaml::from_reader(file).unwrap();
                    debug!(ports = ?ports, "Found ports");
                    m.ports.append(&mut ports.ports);
                } else if p["name"].as_str().is_some() {
                    debug!(name = ?p, "Including port inline");
                    m.ports.push(UsbPort::from(p));
                } else if let Some(name) = p.as_str() {
                    debug!(name = ?p, "Including port by name");
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
