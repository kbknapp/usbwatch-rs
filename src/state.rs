use std::{
    collections::HashMap,
    fs::{self, File},
    path::Path,
};

use tracing::{debug, info, span, Level};
use yaml_rust::YamlLoader;

use crate::{
    rule::{Rule, Rules},
    usb::{UsbDevice, UsbDevices, UsbPort, UsbPorts},
};

#[derive(Default)]
pub struct State {
    ports: Vec<UsbPort>,
    devices: Vec<UsbDevice>,
    active_devices: Vec<usize>,
    // Port->Device
    slot_map: HashMap<usize, Option<usize>>,
    // Device->Port
    rev_slot_map: HashMap<usize, usize>,
    pub rules: Vec<Rule>,
}

impl State {
    pub fn new() -> Self { Self::default() }

    pub fn devices_from_file<P: AsRef<Path>>(&mut self, path: P) {
        let span = span!(Level::TRACE, "fn devices_from_file", file = ?path.as_ref());
        let _enter = span.enter();

        let file = File::open(path).unwrap();
        let devices: UsbDevices = serde_yaml::from_reader(file).unwrap();
        info!(num_devs= %devices.devices.len(), "Found Devices");
        for device in devices.devices.into_iter() {
            debug!(device = %device, "Adding Device");
            self.add_device(device);
        }
    }

    pub fn ports_from_file<P: AsRef<Path>>(&mut self, path: P) {
        let span = span!(Level::TRACE, "fn ports_from_file", file = ?path.as_ref());
        let _enter = span.enter();

        let file = File::open(path).unwrap();
        let ports: UsbPorts = serde_yaml::from_reader(file).unwrap();
        info!(num_ports= %ports.ports.len(), "Found Ports");
        for port in ports.ports.into_iter() {
            debug!(port = %port, "Adding Port");
            self.add_port(port);
        }
    }

    pub fn rules_from_file<P: AsRef<Path>>(&mut self, path: P) {
        let span = span!(Level::TRACE, "fn rules_from_file", file = ?path.as_ref());
        let _enter = span.enter();

        let buf = fs::read_to_string(path).unwrap();
        let rules = Rules::from(&YamlLoader::load_from_str(&buf).unwrap()[0]);
        info!(num_rules= %rules.rules.len(), "Found Rules");
        for rule in rules.rules.into_iter() {
            debug!(ruel = ?rule.name, "Adding Rule");
            self.rules.push(rule);
        }
    }

    pub fn add_port(&mut self, port: UsbPort) {
        let span = span!(Level::TRACE, "fn add_port", port = %port);
        let _enter = span.enter();

        for p in self.ports.iter() {
            if p == &port {
                debug!("Port already exists; returning");
                return;
            }
        }
        self.ports.push(port);
        debug!(key = self.ports.len(), "Slotting empty port");
        self.slot_map.entry(self.ports.len()).or_insert(None);
    }

    pub fn add_device(&mut self, device: UsbDevice) {
        let span = span!(Level::TRACE, "fn add_device", device = %device);
        let _enter = span.enter();

        if self.devices.contains(&device) {
            debug!("Device already exists; returning");
            return;
        }
        self.devices.push(device);
    }

    pub fn add_and_slot_device(&mut self, device: UsbDevice, port: UsbPort) {
        let span = span!(Level::TRACE, "fn add_and_slot_device", device = %device, port = %port);
        let _enter = span.enter();

        self.add_port(port.clone());
        self.add_device(device.clone());

        for (i, p) in self.ports.iter().enumerate() {
            debug!(i=i, port = %p, "Iter ports");
            if p == &port {
                debug!("Matched Port");
                for (j, d) in self.devices.iter().enumerate() {
                    debug!(j=j, device = %d, "Iter devices");
                    if d == &device {
                        debug!("Matched device");

                        debug!(
                            i = i,
                            j = j,
                            "Setting port slot {} to device index {}",
                            i,
                            j
                        );
                        *self.slot_map.entry(i).or_insert(Some(j)) = Some(j);
                        debug!(
                            i = i,
                            j = j,
                            "Setting reverse slot map device index {} to slot {}",
                            j,
                            i
                        );
                        *self.rev_slot_map.entry(j).or_insert(i) = i;
                        debug!("Activating device index {}", j);
                        self.active_devices.push(j);

                        debug!("Returning");
                        break;
                    }
                }
            }
        }
    }

    pub fn rm_and_unslot_device(&mut self, device: UsbDevice) {
        let span = span!(Level::TRACE, "fn rm_and_unslot_device", device = %device);
        let _enter = span.enter();

        for (i, d) in self.devices.iter().enumerate() {
            debug!(i=i, device = %d, "Iter devices");
            if d == &device {
                debug!("Matched device");
                if let Some(p) = self.rev_slot_map.get_mut(&i) {
                    debug!(
                        "Found port index {} via device reverse slot map index {}",
                        p, i
                    );
                    debug!("Setting slot map {} to None", p);
                    *self.slot_map.entry(*p).or_insert(None) = None;
                }
                let mut to_rem = None;
                for (j, idx) in self.active_devices.iter().enumerate() {
                    if *idx == i {
                        to_rem = Some(j);
                        break;
                    }
                }
                if let Some(idx) = to_rem {
                    debug!("Removing device index {} from active devices", idx);
                    self.active_devices.swap_remove(idx);
                }
                debug!("Returning");
                break;
            }
        }
    }
}
