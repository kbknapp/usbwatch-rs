#[macro_use]
extern crate bitflags;
use clap::*;

#[macro_use]
mod macros;
mod cli;
mod cmds;
mod listener;
mod rule;
mod state;
mod tokio_udev;
mod udev;
mod usb;
mod shutdown;

use tracing_subscriber;
use tracing::{debug, error, info, instrument};

pub fn main() {
    tracing_subscriber::fmt::init();

    use cli::UsbWatchSubCmd::*;

    info!("Parsing command line arguments");
    let args = cli::UsbWatchArgs::parse();

    match args.subcmd {
        Some(CreateDevice(a)) => cmds::create_device::run(a),
        Some(CreatePort(a)) => cmds::create_port::run(a),
        Some(CreateRule(a)) => cmds::create_rule::run(a),
        Some(Listen(a)) => cmds::listen::run(a),
        Some(Run(a)) => cmds::run::run(a),
        Some(Check(a)) => cmds::check::run(a),
        None => todo!("Impl no subcommand"),
    }
}
