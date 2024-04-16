use std::{
    fs::{self, File},
    path::PathBuf,
};

use clap::Args;
use tracing::warn;
use yaml_rust::YamlLoader;

use crate::{
    cli::Cmd,
    ctx::Ctx,
    rule::Rules,
    usb::{UsbDevices, UsbPorts},
};

/// List matched components from loaded rules
#[derive(Args, Debug)]
pub struct UsbWatchCheck {
    /// Rules file to use
    #[arg(long, short)]
    pub rules: Option<PathBuf>,

    /// Devices to match against
    #[arg(long, short)]
    pub devices: Option<PathBuf>,

    /// Ports to match against
    #[arg(long, short)]
    pub ports: Option<PathBuf>,
}

impl Cmd for UsbWatchCheck {
    fn run(&self, _ctx: &mut Ctx) -> anyhow::Result<()> {
        warn!("Not fully implemented");

        if let Some(path) = &self.devices {
            let file = File::open(path).unwrap();
            let devices: UsbDevices = serde_yaml::from_reader(file).unwrap();
            cli_println!("{:#?}", devices);
        }

        if let Some(path) = &self.ports {
            let file = File::open(path).unwrap();
            let ports: UsbPorts = serde_yaml::from_reader(file).unwrap();
            cli_println!("{:#?}", ports);
        }

        if let Some(path) = &self.rules {
            let buf = fs::read_to_string(path).unwrap();
            let rules = Rules::from(&YamlLoader::load_from_str(&buf).unwrap()[0]);
            cli_println!("{:#?}", rules);
        }
        Ok(())
    }
}
