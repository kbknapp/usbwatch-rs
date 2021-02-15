use std::{fs::{self, File}, io::Read};

use serde_yaml;
use yaml_rust::YamlLoader;

use crate::{
    port::{Port, Ports},
    device::{Device, Devices},
    cli::CheckArgs,
    rule::{Rule, Rules},

};

pub fn run(a: CheckArgs) {
    if let Some(path) = a.devices {
        let file = File::open(path).unwrap();
        let devices: Devices = serde_yaml::from_reader(file).unwrap();
        println!("{:#?}", devices);
    }

    if let Some(path) = a.ports {
        let file = File::open(path).unwrap();
        let ports: Ports = serde_yaml::from_reader(file).unwrap();
        println!("{:#?}", ports);
    }

    if let Some(path) = a.rules {
        let mut buf = fs::read_to_string(path).unwrap();
        let rules = Rules::from(&YamlLoader::load_from_str(&*buf).unwrap()[0]);
        println!("{:#?}", rules);
    }
}
