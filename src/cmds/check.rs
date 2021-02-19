use std::fs::{self, File};

use tracing::{self, warn};
use yaml_rust::YamlLoader;

use crate::{
    cli::CheckArgs,
    rule::Rules,
    usb::{UsbDevices, UsbPorts},
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
