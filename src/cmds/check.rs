use std::{fs::{self, File}};

use serde_yaml;
use yaml_rust::YamlLoader;
use tracing::{self, debug, warn, error, info, trace, instrument};

use crate::{
    usb::{UsbDevice, UsbDevices, UsbPort, UsbPorts},
    cli::CheckArgs,
    rule::{Rule, Rules},

};

#[tracing::instrument]
pub fn run(a: CheckArgs) {
    warn!("Not fully implemented");

    if let Some(path) = a.devices {
        let file = File::open(path).unwrap();
        let devices: UsbDevices = serde_yaml::from_reader(file).unwrap();
        println!("{:#?}", devices);
    }

    if let Some(path) = a.ports {
        let file = File::open(path).unwrap();
        let ports: UsbPorts = serde_yaml::from_reader(file).unwrap();
        println!("{:#?}", ports);
    }

    if let Some(path) = a.rules {
        let buf = fs::read_to_string(path).unwrap();
        let rules = Rules::from(&YamlLoader::load_from_str(&*buf).unwrap()[0]);
        println!("{:#?}", rules);
    }
}
