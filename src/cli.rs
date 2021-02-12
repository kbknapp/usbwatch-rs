use clap::Clap;

use crate::usb_event::UsbEvent;

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
    CreateDevice(CreateDeviceArgs),
    CreateRule(CreateRuleArgs),
    Listen(ListenArgs),
    Run(RunArgs),
}

#[derive(Clap)]
pub struct CreateDeviceArgs {
}

#[derive(Clap)]
pub struct CreateRuleArgs {
    /// USB Event activate the rule
    #[clap(long, short, arg_enum, default_value="All")]
    pub on: UsbEvent,
}

#[derive(Clap)]
pub struct ListenArgs {
}

#[derive(Clap)]
pub struct RunArgs {
    /// Run in the background as a daemon
    #[clap(long, short)]
    pub daemon: bool,
}

