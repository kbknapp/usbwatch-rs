#[macro_use]
mod macros;
mod cli;
mod ctx;
mod listener;
mod rule;
mod shutdown;
mod state;
mod tokio_udev;
mod udev;
mod usb;

use std::env;

use clap::*;
use tracing::error;

use crate::{
    cli::{Cmd, UsbWatch},
    ctx::Ctx,
};

fn main() {
    let args = UsbWatch::parse();

    match args.verbose {
        0 => (),
        1 => env::set_var("RUST_LOG", "usbwatch=info"),
        2 => env::set_var("RUST_LOG", "usbwatch=debug"),
        _ => env::set_var("RUST_LOG", "usbwatch=trace"),
    }

    tracing_subscriber::fmt::init();

    let mut ctx = Ctx;
    let cmd: &dyn Cmd = &args;

    if let Err(e) = cmd.walk_exec(&mut ctx) {
        error!("{e}");
        eprintln!("error: {e}");
        std::process::exit(1);
    }
}
