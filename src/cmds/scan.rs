use std::ffi::OsStr;

use tokio_udev::Enumerator;

use crate::{
    cli::{ForObject, OutFormat, ScanArgs},
    usb::{UsbDevice, UsbPort},
};

pub fn run(a: ScanArgs) {
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

    match a.format {
        Raw => {
            if a.scan_for == ForObject::Ports || a.scan_for == ForObject::All {
                println!("{:#?}", ports);
            }
            if a.scan_for == ForObject::Devices || a.scan_for == ForObject::All {
                println!("{:#?}", devices);
            }
        }
        Yaml => {
            if a.scan_for == ForObject::Ports || a.scan_for == ForObject::All {
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
            if a.scan_for == ForObject::Devices || a.scan_for == ForObject::All {
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
}
