#![allow(non_snake_case)]
#![warn(rust_2018_idioms, future_incompatible)]

use clap::*;

#[macro_use]
mod macros;
mod cli;
mod cmds;
mod listener;
mod rule;
mod shutdown;
mod state;
mod tokio_udev;
mod udev;
mod usb;

use tracing::info;

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
