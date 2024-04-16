mod check;
mod listen;
mod rule;
mod run;
mod scan;

use std::env;

use clap::{ArgAction, Parser, Subcommand, ValueEnum};

use crate::{
    ctx::Ctx,
    printer::{ColorChoice, OutFormat, Printer},
};

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
    /// Display more verbose output
    ///
    /// More uses displays more verbose output
    ///     -v:  Display debug info
    ///     -vv: Display trace info
    #[arg(long, short, action = ArgAction::Count, global = true)]
    pub verbose: u8,

    /// Suppress output at a specific level and below
    ///
    /// More uses suppresses higher levels of output
    ///     -q:   Only display WARN messages and above
    ///     -qq:  Only display ERROR messages
    ///     -qqq: Suppress all output
    #[arg(long, short, action = ArgAction::Count, global = true)]
    pub quiet: u8,

    /// Should the output include color?
    #[arg(long, value_enum, value_name = "WHEN", default_value = "auto", overrides_with_all = ["color", "no_color"], global = true)]
    pub color: ColorChoice,

    /// Do not color output (alias for --color=never)
    #[arg(long, overrides_with_all = ["color", "no_color"], global = true)]
    pub no_color: bool,

    /// Display output in format
    #[arg(
        short = 'F',
        long,
        value_enum,
        value_name = "FORMAT",
        default_value = "yaml",
        global = true
    )]
    pub format: OutFormat,

    #[command(subcommand)]
    pub cmd: UsbWatchCmd,
}

impl Cmd for UsbWatch {
    fn update_ctx(&self, ctx: &mut Ctx) -> anyhow::Result<()> {
        ctx.verbose = self.verbose;
        ctx.format = self.format;

        Ok(())
    }

    fn run(&self, ctx: &mut Ctx) -> anyhow::Result<()> {
        // Initialize the printer now that we have all the color choices
        Printer::init(ctx.color);

        Ok(())
    }

    fn next_cmd(&self) -> Option<&dyn Cmd> { Some(&self.cmd) }
}

#[enum_delegate::implement(Cmd)]
#[derive(Subcommand)]
pub enum UsbWatchCmd {
    Listen(listen::UsbWatchListen),
    Run(run::UsbWatchRun),
    Check(check::UsbWatchCheck),
    Scan(scan::UsbWatchScan),
    CreateRule(rule::UsbWatchCreateRule),
}

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq)]
pub enum ForObject {
    Ports,
    Devices,
    All,
}
