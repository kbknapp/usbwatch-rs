use std::path::PathBuf;

use clap::Args;
use serde::Serialize;

use crate::{
    cli::{Cmd},
    ctx::Ctx,
    usb::UsbEvent,
};

fn default_shell(p: &PathBuf) -> bool { p == &PathBuf::from("/bin/sh") }

/// Create a rule
#[derive(Args, Clone, Debug)]
pub struct UsbWatchCreateRule {
    /// Name of the rule
    #[arg(long, short, value_name = "STR")]
    pub name: String,

    /// Paths to device files that will be matched
    #[arg(long, short, value_name = "PATH")]
    pub devices_file: Vec<PathBuf>,

    /// Paths to ports files that will be matched
    #[arg(long, short, value_name = "PATH")]
    pub ports_file: Vec<PathBuf>,

    /// Events to match on
    #[arg(long, short, value_enum, value_name = "KIND", default_value = "all")]
    pub on: UsbEvent,

    /// The command or script to execute on rule match
    ///
    /// Will be run using the command-shell with a
    /// '-c' argument
    #[arg(long, short = 'x', value_name = "STR")]
    pub execute: String,

    /// The shell to use to execute the command(s)
    ///
    /// The shell provided here must use a `-c "cmds"` argument.
    #[arg(long, short, default_value = "/bin/sh")]
    pub shell: PathBuf,
}

impl Cmd for UsbWatchCreateRule {
    fn run(&self, _ctx: &mut Ctx) -> anyhow::Result<()> {
        #[derive(Serialize, PartialEq, Debug)]
        struct CliRule {
            pub name: String,
            #[serde(skip_serializing_if = "default_shell")]
            pub command_shell: PathBuf,
            pub command: String,
            r#match: CliMatch,
        }
        #[derive(Serialize, PartialEq, Debug)]
        pub struct CliMatch {
            on: UsbEvent,
            #[serde(skip_serializing_if = "Vec::is_empty")]
            devices: Vec<IncludeDevices>,
            #[serde(skip_serializing_if = "Vec::is_empty")]
            ports: Vec<IncludePorts>,
            #[serde(skip_serializing_if = "Vec::is_empty")]
            ignore_devices: Vec<usize>,
        }
        #[derive(Serialize, PartialEq, Debug)]
        pub struct IncludeDevices {
            include_devices: PathBuf,
        }
        #[derive(Serialize, PartialEq, Debug)]
        pub struct IncludePorts {
            include_ports: PathBuf,
        }

        let r = CliRule {
            name: self.name.clone(),
            r#match: CliMatch {
                on: self.on,
                devices: self
                    .devices_file
                    .iter()
                    .map(|pb| IncludeDevices {
                        include_devices: pb.to_path_buf(),
                    })
                    .collect(),
                ports: self
                    .ports_file
                    .iter()
                    .map(|pb| IncludePorts {
                        include_ports: pb.to_path_buf(),
                    })
                    .collect(),
                ignore_devices: vec![],
            },
            command_shell: self.shell.clone(),
            command: self.execute.clone(),
        };
        let yaml = serde_yaml::to_string(&r).unwrap();

        cli_print!("---\nrules:\n  - ");
        for (i, line) in yaml.lines().enumerate() {
            if i != 0 {
                cli_print!("    ");
            }
            cli_println!("{line}");
        }

        Ok(())
    }
}
