use clap::*;

#[macro_use]
mod macros;
mod usb_event;
mod cli;
mod cmds;
mod device;
mod port;

pub fn main() {
    // enable logging
    // see https://docs.rs/tracing for more info
    //tracing_subscriber::fmt::try_init()?;

    use cli::UsbWatchSubCmd::*;

    let args = cli::UsbWatchArgs::parse();

    match args.subcmd {
        Some(CreateDevice(a)) => cmds::create_device::run(a),
        Some(CreateRule(a)) => cmds::create_rule::run(a),
        Some(Listen(a)) => cmds::listen::run(a),
        Some(Run(a)) => cmds::run::run(a),
        None => todo!("Impl no subcommand")
    }
}
