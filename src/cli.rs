use std::path::PathBuf;

use clap::{ArgAction, Args, Parser, Subcommand, ValueEnum};

use crate::usb::UsbEvent;

/// Monitor USB events and execute actions
#[derive(Parser)]
#[command(version = env!("VERSION_WITH_GIT_HASH"))]
pub struct UsbWatchArgs {
    /// Show verbose output
    #[arg(long, short, action = ArgAction::Count)]
    pub verbose: u8,

    #[command(subcommand)]
    pub subcmd: Option<UsbWatchSubCmd>,
}

#[derive(Subcommand)]
pub enum UsbWatchSubCmd {
    /// Listen for events and display them to stdout
    Listen(ListenArgs),

    /// Begin matching against rules and running actions
    Run(RunArgs),

    /// List matched components from loaded rules
    Check(CheckArgs),

    /// Scan the currently attached devices and print their info
    Scan(ScanArgs),
}

#[derive(Args, Copy, Clone, Debug)]
pub struct ScanArgs {
    /// Only display KIND of objects
    #[arg(
        long,
        short,
        value_enum,
        value_name = "KIND",
        default_value = "devices"
    )]
    pub scan_for: ForObject,
    /// Display output in format
    #[clap(
        long,
        short,
        value_enum,
        value_name = "FORMAT",
        default_value = "raw",
        alias = "output"
    )]
    pub format: OutFormat,
}

#[derive(Args, Copy, Clone, Debug)]
pub struct ListenArgs {
    /// Only display KIND of objects
    #[arg(long, short, value_enum, value_name = "KIND", default_value = "all")]
    pub listen_for: ForObject,
    /// Only display KIND of events
    #[arg(long, short, value_enum, value_name = "KIND", default_value = "all")]
    pub events: UsbEvent,
    /// Display output in format
    #[arg(
        long,
        short,
        value_enum,
        value_name = "FORMAT",
        default_value = "raw",
        alias = "output"
    )]
    pub format: OutFormat,
}

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq)]
pub enum ForObject {
    Ports,
    Devices,
    All,
}

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq)]
pub enum OutFormat {
    Raw,
    Yaml,
}

#[derive(Args, Debug)]
pub struct RunArgs {
    /// Rules file to use
    #[arg(long, short)]
    pub rules: PathBuf,
    /// Devices to match against
    #[arg(long, short)]
    pub devices: Option<PathBuf>,
    /// Ports to match against
    #[arg(long, short)]
    pub ports: Option<PathBuf>,
}

#[derive(Args, Debug)]
#[command(visible_aliases = &["test", "debug"])]
pub struct CheckArgs {
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
