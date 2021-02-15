use std::fmt::{self, Debug};

use tokio_udev::{self, Device, Property, Attribute};

pub struct DebugDevice {
    dev: Device,
}

impl DebugDevice {
    pub fn new(dev: Device) -> Self {
        Self {
            dev
        }
    }
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
            .field("properties", &self.dev.properties().map(|p| DebugProperty::new(p)).collect::<Vec<_>>())
            .field("attributes", &self.dev.attributes().map(|a| DebugAttribute::new(a)).collect::<Vec<_>>())
            .finish()
    }
}

pub struct DebugProperty {
    name: String,
    value: String,
}

impl DebugProperty {
    pub fn new(prop: Property) -> Self {
        Self {
            name: prop.name().to_string_lossy().to_string(),
            value: prop.value().to_string_lossy().to_string()
        }
    }
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

impl DebugAttribute {
    pub fn new(attr: Attribute) -> Self {
        Self {
            name: attr.name().to_string_lossy().to_string(),
            value: attr.value().map(|v| v.to_string_lossy().to_string())
        }
    }
}

impl fmt::Debug for DebugAttribute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Attribute")
            .field("name", &&*self.name)
            .field("value", &self.value)
            .finish()
    }
}
