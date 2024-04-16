#[macro_use]
mod macros;
mod cli;
mod ctx;
mod listener;
mod log;
mod printer;
mod rule;
mod shutdown;
mod state;
mod tokio_udev;
mod udev;
mod usb;

use clap::*;

use crate::{
    cli::{Cmd, UsbWatch},
    ctx::Ctx,
    log::LogLevel,
};

fn main() {
    let args = UsbWatch::parse();

    // Normally, this would be in the Seaplane::run method, however setting up
    // logging has to happen super early in the process lifetime
    match args.verbose {
        0 => match args.quiet {
            0 => log::LOG_LEVEL.set(LogLevel::Info).unwrap(),
            1 => log::LOG_LEVEL.set(LogLevel::Warn).unwrap(),
            2 => log::LOG_LEVEL.set(LogLevel::Error).unwrap(),
            _ => log::LOG_LEVEL.set(LogLevel::Off).unwrap(),
        },
        1 => log::LOG_LEVEL.set(LogLevel::Debug).unwrap(),
        _ => log::LOG_LEVEL.set(LogLevel::Trace).unwrap(),
    }

    let mut ctx = Ctx::default();
    let cmd: &dyn Cmd = &args;

    if let Err(e) = cmd.walk_exec(&mut ctx) {
        if ctx.tracing {
            tracing::error!("{e}");
        } else {
            eprintln!("error: {e}");
        }
        std::process::exit(1);
    }
}
