mod check;
mod listen;
mod run;
mod scan;

use std::env;

use clap::{ArgAction, Parser, Subcommand, ValueEnum};

use crate::ctx::Ctx;

#[enum_delegate::register]
pub trait Cmd {
    fn update_ctx(&self, _ctx: &mut Ctx) -> anyhow::Result<()> { Ok(()) }
    fn run(&self, _ctx: &mut Ctx) -> anyhow::Result<()> { Ok(()) }
    fn next_cmd(&self) -> Option<&dyn Cmd> { None }
}

impl<'a> dyn Cmd + 'a {
    pub fn walk_exec(&self, ctx: &mut Ctx) -> anyhow::Result<()> {
        self.update_ctx(ctx)?;
        self.run(ctx)?;
        if let Some(c) = self.next_cmd() {
            return c.walk_exec(ctx);
        }
        Ok(())
    }
}

/// Monitor USB events and execute actions
#[derive(Parser)]
#[command(version = env!("VERSION_WITH_GIT_HASH"))]
pub struct UsbWatch {
    /// Show verbose output
    #[arg(long, short, action = ArgAction::Count)]
    pub verbose: u8,

    #[command(subcommand)]
    pub cmd: UsbWatchCmd,
}

impl Cmd for UsbWatch {
    fn next_cmd(&self) -> Option<&dyn Cmd> { Some(&self.cmd) }
}

#[enum_delegate::implement(Cmd)]
#[derive(Subcommand)]
pub enum UsbWatchCmd {
    Listen(listen::UsbWatchListen),
    Run(run::UsbWatchRun),
    Check(check::UsbWatchCheck),
    Scan(scan::UsbWatchScan),
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
