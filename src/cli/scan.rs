use std::ffi::OsStr;

use clap::Args;
use tokio_udev::Enumerator;

use crate::{
    cli::{Cmd, ForObject, OutFormat},
    ctx::Ctx,
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
    pub scan_for: ForObject,

    /// Display output in format
    #[clap(
        long,
        short,
        value_enum,
        value_name = "FORMAT",
        default_value = "raw",
        alias = "output"
    )]
    pub format: OutFormat,
}

impl Cmd for UsbWatchScan {
    fn run(&self, _ctx: &mut Ctx) -> anyhow::Result<()> {
        use OutFormat::*;
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

        match self.format {
            Raw => {
                if self.scan_for == ForObject::Ports || self.scan_for == ForObject::All {
                    println!("{:#?}", ports);
                }
                if self.scan_for == ForObject::Devices || self.scan_for == ForObject::All {
                    println!("{:#?}", devices);
                }
            }
            Yaml => {
                if self.scan_for == ForObject::Ports || self.scan_for == ForObject::All {
                    println!("---\nports:");
                    for port in ports {
                        print!("  - ");
                        let yaml = serde_yaml::to_string(&port).unwrap();
                        for (i, line) in yaml.lines().skip(1).enumerate() {
                            if i == 0 {
                                println!("{}", line);
                            } else {
                                println!("    {}", line);
                            }
                        }
                    }
                }
                if self.scan_for == ForObject::Devices || self.scan_for == ForObject::All {
                    println!("---\ndevices:");
                    for device in devices {
                        print!("  - ");
                        let yaml = serde_yaml::to_string(&device).unwrap();
                        for (i, line) in yaml.lines().skip(1).enumerate() {
                            if i == 0 {
                                println!("{}", line);
                            } else {
                                println!("    {}", line);
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }
}
