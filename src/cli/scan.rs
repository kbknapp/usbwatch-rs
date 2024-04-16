use std::ffi::OsStr;

use clap::Args;
use tokio_udev::Enumerator;

use crate::{
    cli::{Cmd, ForObject},
    ctx::Ctx,
    printer::OutFormat,
    usb::{UsbDevice, UsbPort},
};

/// Scan the currently attached devices and print their info
#[derive(Args, Copy, Clone, Debug)]
pub struct UsbWatchScan {
    /// Only display KIND of objects
    #[arg(
        long,
        short,
        value_enum,
        value_name = "KIND",
        default_value = "devices"
    )]
    pub only: ForObject,
}

impl Cmd for UsbWatchScan {
    fn run(&self, ctx: &mut Ctx) -> anyhow::Result<()> {
        let mut scanner = Enumerator::new().unwrap();
        scanner.match_subsystem("usb").unwrap();

        let mut ports = Vec::new();
        let mut devices = Vec::new();
        for dev in scanner
            .scan_devices()
            .unwrap()
            .filter(|d| Some(OsStr::new("usb_interface")) != d.devtype())
        {
            let usbdev = UsbDevice::from(&dev);
            let usbport = UsbPort::from(&dev);
            if !usbdev.is_empty() {
                devices.push(usbdev);
            }
            if !usbport.is_empty() {
                ports.push(usbport);
            }
        }

        match ctx.format {
            OutFormat::Raw => {
                if self.only == ForObject::Ports || self.only == ForObject::All {
                    cli_println!("{:#?}", ports);
                }
                if self.only == ForObject::Devices || self.only == ForObject::All {
                    cli_println!("{:#?}", devices);
                }
            }
            OutFormat::Yaml => {
                if self.only == ForObject::Ports || self.only == ForObject::All {
                    cli_println!("---\nports:");
                    for port in ports {
                        print!("  - ");
                        let yaml = serde_yaml::to_string(&port).unwrap();
                        for (i, line) in yaml.lines().skip(1).enumerate() {
                            if i == 0 {
                                cli_println!("{}", line);
                            } else {
                                cli_println!("    {}", line);
                            }
                        }
                    }
                }
                if self.only == ForObject::Devices || self.only == ForObject::All {
                    cli_println!("---\ndevices:");
                    for device in devices {
                        print!("  - ");
                        let yaml = serde_yaml::to_string(&device).unwrap();
                        for (i, line) in yaml.lines().skip(1).enumerate() {
                            if i == 0 {
                                cli_println!("{}", line);
                            } else {
                                cli_println!("    {}", line);
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }
}
