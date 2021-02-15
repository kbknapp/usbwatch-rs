use std::fs;


use yaml_rust::YamlLoader;

use crate::{
    port::{Port, Ports},
    device::{Device, Devices},
    cli::RunArgs,
    rule::{Rule, Rules},
    listen::listen,
};

pub fn run(a: RunArgs) {
    let buf = fs::read_to_string(a.rules).unwrap();
    let _rules = Rules::from(&YamlLoader::load_from_str(&*buf).unwrap()[0]);

    tokio::runtime::Builder::new_current_thread()
        .enable_io()
        .build()
        .unwrap()
        .block_on(listen())
}
