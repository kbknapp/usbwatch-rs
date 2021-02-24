use std::path::PathBuf;

use clap::Clap;

use crate::usb::UsbEvent;

/// Monitor USB events and execute actions
#[derive(Clap)]
#[clap(version = env!("VERSION_WITH_GIT_HASH"))]
pub struct UsbWatchArgs {
    /// Show verbose output
    #[clap(long, short, parse(from_occurrences))]
    pub verbose: u8,

    #[clap(subcommand)]
    pub subcmd: Option<UsbWatchSubCmd>,
}

#[derive(Clap)]
pub enum UsbWatchSubCmd {
    /// Listen for events and display them to stdout
    Listen(ListenArgs),

    /// Begin matching against rules and running actions
    Run(RunArgs),

    /// List matched components from loaded rules
    #[clap(visible_aliases = &["test", "debug"])]
    Check(CheckArgs),
}

#[derive(Clap, Copy, Clone, Debug)]
pub struct ListenArgs {
    /// Only display KIND of objects
    #[clap(long, short, arg_enum, value_name = "KIND", default_value = "all")]
    pub listen_for: ForObject,
    /// Only display KIND of events
    #[clap(long, short, arg_enum, value_name = "KIND", default_value = "all")]
    pub events: UsbEvent,
    /// Display output in format
    #[clap(long, short, arg_enum, value_name = "FORMAT", default_value = "raw", alias="output")]
    pub format: OutFormat,
}

#[derive(Clap, Copy, Clone, Debug, PartialEq)]
pub enum ForObject {
    Ports,
    Devices,
    All,
}

#[derive(Clap, Copy, Clone, Debug, PartialEq)]
pub enum OutFormat {
    Raw,
    Yaml,
}

#[derive(Clap, Debug)]
pub struct RunArgs {
    /// Rules file to use
    #[clap(long, short)]
    pub rules: PathBuf,
    /// Devices to match against
    #[clap(long, short)]
    pub devices: Option<PathBuf>,
    /// Ports to match against
    #[clap(long, short)]
    pub ports: Option<PathBuf>,
}

#[derive(Clap, Debug)]
pub struct CheckArgs {
    /// Rules file to use
    #[clap(long, short)]
    pub rules: Option<PathBuf>,
    /// Devices to match against
    #[clap(long, short)]
    pub devices: Option<PathBuf>,
    /// Ports to match against
    #[clap(long, short)]
    pub ports: Option<PathBuf>,
}
