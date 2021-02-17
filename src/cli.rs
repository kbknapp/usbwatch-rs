use std::{str::FromStr, path::PathBuf};

use clap::Clap;
use serde::{Serialize, Deserialize};

use crate::usb::UsbEvent;

/// Monitor USB events and execute actions
#[derive(Clap)]
pub struct UsbWatchArgs {
    /// Show verbose output
    #[clap(long, short)]
    pub verbose: bool,

    #[clap(subcommand)]
    pub subcmd: Option<UsbWatchSubCmd>
}

#[derive(Clap)]
pub enum UsbWatchSubCmd {
    /// Create a port file
    CreatePort(CreatePortArgs),

    /// Create a device file
    CreateDevice(CreateDeviceArgs),

    /// Create a rule file
    CreateRule(CreateRuleArgs),

    /// Listen for events and display them to stdout
    Listen(ListenArgs),

    /// Begin matching against rules and running actions
    Run(RunArgs),

    /// List matched components from loaded rules
    #[clap(visible_aliases = &["test", "debug"])]
    Check(CheckArgs),
}

#[derive(Clap)] pub struct CreateDeviceArgs {}

#[derive(Clap)] pub struct CreatePortArgs {}

#[derive(Clap)]
pub struct CreateRuleArgs {
    /// USB Event activate the rule
    #[clap(long, short, arg_enum, default_value="All")]
    pub on: UsbEvent,
}

#[derive(Clap)]
pub struct ListenArgs {
    /// Only display KIND of objects
    #[clap(long, short, arg_enum, value_name = "KIND", default_value="All")]
    pub object: ListenForObject,
    /// Only display KIND of events
    #[clap(long, short, arg_enum, value_name = "KIND", default_value="all")]
    pub events: UsbEvent,
    /// Display output in format
    #[clap(long, short, arg_enum, value_name = "FORMAT", default_value="Raw")]
    pub format: ListenFormat,
}

#[derive(Clap)]
pub enum ListenForObject {
    Ports,
    Devices,
    All
}

    Raw,
    Yaml,
}

#[derive(Clap)]
pub struct RunArgs {
    /// Run in the background as a daemon
    #[clap(long, short)]
    pub daemon: bool,
    /// Rules file to use
    #[clap(long, short)]
    pub rules: PathBuf,
}

#[derive(Clap)]
pub struct CheckArgs {
    /// Rules file to use
    #[clap(long, short)]
    pub rules: Option<PathBuf>,
    /// Devices to match against
    #[clap(long, short)]
    pub devices: Option<PathBuf>,
    /// Ports to match against
    #[clap(long, short)]
    pub ports:  Option<PathBuf>,
}
