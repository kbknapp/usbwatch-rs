use std::fmt;

use tokio_udev::{self, Device};

pub struct DebugDevice {
    dev: Device,
}

impl DebugDevice {
    #[allow(dead_code)]
    pub fn new(dev: Device) -> Self { Self { dev } }
}

impl fmt::Debug for DebugDevice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Device")
            .field("devnum", &self.dev.devnum())
            .field("syspath", &self.dev.syspath())
            .field("devnode", &self.dev.devnode())
            .field("devpath", &self.dev.devpath())
            //.field("parent", &DebugDevice::from(self.dev.parent()))
            .field("subsystem", &self.dev.subsystem())
            .field("sysname", &self.dev.sysname())
            .field("sysnum", &self.dev.sysnum())
            .field("devtype", &self.dev.devtype())
            .field("driver", &self.dev.driver())
            .field(
                "properties",
                &self
                    .dev
                    .properties()
                    .map(|e| {


        DebugProperty {
            name: e.name().to_string_lossy().to_string(),
            value: e.value().to_string_lossy().to_string(),
        }

                    })
                    .collect::<Vec<_>>(),
            )
            .field(
                "attributes",
                &self
                    .dev
                    .attributes()
                    .map(|e| {


        DebugAttribute {
            name: e.name().to_string_lossy().to_string(),
            value: Some(e.value().to_string_lossy().to_string()),
        }
                    })
                    .collect::<Vec<_>>(),
            )
            .finish()
    }
}

pub struct DebugProperty {
    name: String,
    value: String,
}

impl fmt::Debug for DebugProperty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Property")
            .field("name", &&*self.name)
            .field("value", &&*self.value)
            .finish()
    }
}

pub struct DebugAttribute {
    name: String,
    value: Option<String>,
}

impl fmt::Debug for DebugAttribute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Attribute")
            .field("name", &&*self.name)
            .field("value", &self.value)
            .finish()
    }
}
