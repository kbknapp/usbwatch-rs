#![allow(non_snake_case)]
#![warn(rust_2018_idioms, future_incompatible)]

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

use std::env;

use clap::*;

pub fn main() {
    use cli::UsbWatchSubCmd::*;

    let args = cli::UsbWatchArgs::parse();

    match args.verbose {
        0 => (),
        1 => env::set_var("RUST_LOG", "usbwatch=info"),
        2 => env::set_var("RUST_LOG", "usbwatch=debug"),
        _ => env::set_var("RUST_LOG", "usbwatch=trace"),
    }

    tracing_subscriber::fmt::init();

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
