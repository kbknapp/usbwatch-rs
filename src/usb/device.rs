use std::fmt::Debug;

use serde::{Deserialize, Serialize};
use yaml_rust::Yaml;

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct UsbDevices {
    pub devices: Vec<UsbDevice>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct UsbDevice {
    #[serde(serialize_with = "super::empty_if_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    ID_MODEL: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    ID_MODEL_ENC: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    ID_MODEL_FROM_DATABASE: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    ID_MODEL_ID: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    ID_SERIAL: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    ID_SERIAL_SHORT: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    ID_VENDOR: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    ID_VENDOR_ENC: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    ID_VENDOR_FROM_DATABASE: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    ID_VENDOR_ID: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    PRODUCT: Option<String>,
}

impl UsbDevice {
    pub fn new<S: Into<String>>(name: S) -> Self {
        Self {
            name: Some(name.into()),
            ..Default::default()
        }
    }

    pub fn is_empty(&self) -> bool {
        self.ID_MODEL.is_none()
            && self.ID_MODEL_ENC.is_none()
            && self.ID_MODEL_FROM_DATABASE.is_none()
            && self.ID_MODEL_ID.is_none()
            && self.ID_VENDOR.is_none()
            && self.ID_VENDOR_ENC.is_none()
            && self.ID_VENDOR_FROM_DATABASE.is_none()
            && self.ID_VENDOR_ID.is_none()
            && self.ID_SERIAL.is_none()
            && self.ID_SERIAL_SHORT.is_none()
            && self.PRODUCT.is_none()
    }
}

impl<'a> From<&'a tokio_udev::Device> for UsbDevice {
    fn from(d: &tokio_udev::Device) -> Self {
        Self {
            ID_MODEL: d
                .property_value("ID_MODEL")
                .map(|v| v.to_string_lossy().to_string()),
            ID_MODEL_ENC: d
                .property_value("ID_MODEL_ENC")
                .map(|v| v.to_string_lossy().to_string()),
            ID_MODEL_FROM_DATABASE: d
                .property_value("ID_MODEL_FROM_DATABASE")
                .map(|v| v.to_string_lossy().to_string()),
            ID_MODEL_ID: d
                .property_value("ID_MODEL_ID")
                .map(|v| v.to_string_lossy().to_string()),
            ID_SERIAL: d
                .property_value("ID_SERIAL")
                .map(|v| v.to_string_lossy().to_string()),
            ID_SERIAL_SHORT: d
                .property_value("ID_SERIAL_SHORT")
                .map(|v| v.to_string_lossy().to_string()),
            ID_VENDOR: d
                .property_value("ID_VENDOR")
                .map(|v| v.to_string_lossy().to_string()),
            ID_VENDOR_ENC: d
                .property_value("ID_VENDOR_ENC")
                .map(|v| v.to_string_lossy().to_string()),
            ID_VENDOR_FROM_DATABASE: d
                .property_value("ID_VENDOR_FROM_DATABASE")
                .map(|v| v.to_string_lossy().to_string()),
            ID_VENDOR_ID: d
                .property_value("ID_VENDOR_ID")
                .map(|v| v.to_string_lossy().to_string()),
            PRODUCT: d
                .property_value("PRODUCT")
                .map(|v| v.to_string_lossy().to_string()),
            ..Default::default()
        }
    }
}

impl<'a> From<&'a Yaml> for UsbDevice {
    fn from(yaml: &'a Yaml) -> Self {
        let mut device = if let Some(name) = yaml["name"].as_str() {
            UsbDevice::new(name)
        } else {
            todo!("Handle Device::from<Yaml> with no name key")
        };

        yaml_str!(device, &yaml, ID_MODEL);
        yaml_str!(device, &yaml, ID_MODEL_ENC);
        yaml_str!(device, &yaml, ID_MODEL_FROM_DATABASE);
        yaml_str!(device, &yaml, ID_MODEL_ID);
        yaml_str!(device, &yaml, ID_VENDOR);
        yaml_str!(device, &yaml, ID_VENDOR_ENC);
        yaml_str!(device, &yaml, ID_VENDOR_FROM_DATABASE);
        yaml_str!(device, &yaml, ID_VENDOR_ID);
        yaml_str!(device, &yaml, ID_SERIAL);
        yaml_str!(device, &yaml, ID_SERIAL_SHORT);
        yaml_str!(device, &yaml, PRODUCT);

        device
    }
}

impl PartialEq<UsbDevice> for UsbDevice {
    fn eq(&self, other: &UsbDevice) -> bool {
        // This impl ignores None values for either LHS or RHS

        // We don't want an empty device to always match
        match (self.is_empty(), other.is_empty()) {
            (true, true) => return true,
            (true, false) | (false, true) => return false,
            (false, false) => (),
        };

        // We don't compare name because it's always none from one side or the other
        cmp_ignore_none!(self, other, ID_MODEL);
        cmp_ignore_none!(self, other, ID_MODEL_ENC);
        cmp_ignore_none!(self, other, ID_MODEL_FROM_DATABASE);
        cmp_ignore_none!(self, other, ID_MODEL_ID);
        cmp_ignore_none!(self, other, ID_VENDOR);
        cmp_ignore_none!(self, other, ID_VENDOR_ENC);
        cmp_ignore_none!(self, other, ID_VENDOR_FROM_DATABASE);
        cmp_ignore_none!(self, other, ID_VENDOR_ID);
        cmp_ignore_none!(self, other, ID_SERIAL);
        cmp_ignore_none!(self, other, ID_SERIAL_SHORT);
        cmp_ignore_none!(self, other, PRODUCT);

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn device_eq_name_only_but_diff() {
        // More Specific
        let d1 = UsbDevice {
            name: Some("foo".into()),
            ID_MODEL: Some("bar".into()),
            ID_MODEL_ENC: Some("baz".into()),
            ID_MODEL_FROM_DATABASE: Some("qux".into()),
            ID_MODEL_ID: Some("foog".into()),
            ID_SERIAL: Some("food".into()),
            ID_SERIAL_SHORT: Some("fool".into()),
            ID_VENDOR: Some("foob".into()),
            ID_VENDOR_ENC: Some("foof".into()),
            ID_VENDOR_FROM_DATABASE: Some("fooz".into()),
            ID_VENDOR_ID: Some("fooq".into()),
            PRODUCT: Some("foom".into()),
        };

        // Less Specific
        let d2 = UsbDevice {
            name: Some("foodster".into()),
            ..Default::default()
        };

        assert!(d1 != d2);
    }
    #[test]
    fn device_eq() {
        // More Specific
        let d1 = UsbDevice {
            name: Some("foo".into()),
            ID_MODEL: Some("bar".into()),
            ID_MODEL_ENC: Some("baz".into()),
            ID_MODEL_FROM_DATABASE: Some("qux".into()),
            ID_MODEL_ID: Some("foog".into()),
            ID_SERIAL: Some("food".into()),
            ID_SERIAL_SHORT: Some("fool".into()),
            ID_VENDOR: Some("foob".into()),
            ID_VENDOR_ENC: Some("foof".into()),
            ID_VENDOR_FROM_DATABASE: Some("fooz".into()),
            ID_VENDOR_ID: Some("fooq".into()),
            PRODUCT: Some("foom".into()),
        };

        // Less Specific
        let d2 = UsbDevice {
            name: Some("foo".into()),
            ID_MODEL_ENC: Some("baz".into()),
            ID_MODEL_ID: Some("foog".into()),
            ID_SERIAL: Some("food".into()),
            ID_VENDOR_ENC: Some("foof".into()),
            ID_VENDOR_ID: Some("fooq".into()),
            ..Default::default()
        };

        assert_eq!(d1, d2);
    }

    #[test]
    fn device_eq_2() {
        // More Specific
        let d1 = UsbDevice {
            name: Some("foo".into()),
            ID_MODEL: Some("bar".into()),
            ID_MODEL_ENC: Some("baz".into()),
            ID_MODEL_FROM_DATABASE: Some("qux".into()),
            ID_MODEL_ID: Some("foog".into()),
            ID_SERIAL: Some("food".into()),
            ID_SERIAL_SHORT: Some("fool".into()),
            ID_VENDOR: Some("foob".into()),
            ID_VENDOR_ENC: Some("foof".into()),
            ID_VENDOR_FROM_DATABASE: Some("fooz".into()),
            ID_VENDOR_ID: Some("fooq".into()),
            PRODUCT: Some("foom".into()),
        };

        // Less Specific
        let d2 = UsbDevice {
            name: Some("foo".into()),
            ID_MODEL: Some("bar".into()),
            ..Default::default()
        };

        assert_eq!(d1, d2);
    }

    #[test]
    fn device_eq_rev() {
        // More Specific
        let d1 = UsbDevice {
            name: Some("foo".into()),
            ID_MODEL: Some("bar".into()),
            ID_MODEL_ENC: Some("baz".into()),
            ID_MODEL_FROM_DATABASE: Some("qux".into()),
            ID_MODEL_ID: Some("foog".into()),
            ID_SERIAL: Some("food".into()),
            ID_SERIAL_SHORT: Some("fool".into()),
            ID_VENDOR: Some("foob".into()),
            ID_VENDOR_ENC: Some("foof".into()),
            ID_VENDOR_FROM_DATABASE: Some("fooz".into()),
            ID_VENDOR_ID: Some("fooq".into()),
            PRODUCT: Some("foom".into()),
        };

        // Less Specific
        let d2 = UsbDevice {
            name: Some("foo".into()),
            ID_MODEL_ENC: Some("baz".into()),
            ID_MODEL_ID: Some("foog".into()),
            ID_SERIAL: Some("food".into()),
            ID_VENDOR_ENC: Some("foof".into()),
            ID_VENDOR_ID: Some("fooq".into()),
            ..Default::default()
        };

        assert_eq!(d2, d1);
    }

    #[test]
    fn device_eq_2_rev() {
        // More Specific
        let d1 = UsbDevice {
            name: Some("foo".into()),
            ID_MODEL: Some("bar".into()),
            ID_MODEL_ENC: Some("baz".into()),
            ID_MODEL_FROM_DATABASE: Some("qux".into()),
            ID_MODEL_ID: Some("foog".into()),
            ID_SERIAL: Some("food".into()),
            ID_SERIAL_SHORT: Some("fool".into()),
            ID_VENDOR: Some("foob".into()),
            ID_VENDOR_ENC: Some("foof".into()),
            ID_VENDOR_FROM_DATABASE: Some("fooz".into()),
            ID_VENDOR_ID: Some("fooq".into()),
            PRODUCT: Some("foom".into()),
        };

        // Less Specific
        let d2 = UsbDevice {
            name: Some("baa".into()),
            ID_SERIAL: Some("food".into()),
            ..Default::default()
        };

        assert_eq!(d2, d1);
    }

    #[test]
    fn device_eq_one_empty() {
        // More Specific
        let d1 = UsbDevice {
            name: Some("foo".into()),
            ID_MODEL: Some("bar".into()),
            ID_MODEL_ENC: Some("baz".into()),
            ID_MODEL_FROM_DATABASE: Some("qux".into()),
            ID_MODEL_ID: Some("foog".into()),
            ID_SERIAL: Some("food".into()),
            ID_SERIAL_SHORT: Some("fool".into()),
            ID_VENDOR: Some("foob".into()),
            ID_VENDOR_ENC: Some("foof".into()),
            ID_VENDOR_FROM_DATABASE: Some("fooz".into()),
            ID_VENDOR_ID: Some("fooq".into()),
            PRODUCT: Some("foom".into()),
        };

        // Less Specific
        let d2 = UsbDevice::default();

        assert!(d1 != d2);
    }

    #[test]
    fn device_eq_one_empty_rev() {
        let d1 = UsbDevice {
            name: Some("foo".into()),
            ID_MODEL: Some("bar".into()),
            ID_MODEL_ENC: Some("baz".into()),
            ID_MODEL_FROM_DATABASE: Some("qux".into()),
            ID_MODEL_ID: Some("foog".into()),
            ID_SERIAL: Some("food".into()),
            ID_SERIAL_SHORT: Some("fool".into()),
            ID_VENDOR: Some("foob".into()),
            ID_VENDOR_ENC: Some("foof".into()),
            ID_VENDOR_FROM_DATABASE: Some("fooz".into()),
            ID_VENDOR_ID: Some("fooq".into()),
            PRODUCT: Some("foom".into()),
        };

        let d2 = UsbDevice::default();

        assert!(d2 != d1);
    }

    #[test]
    fn device_eq_empty() {
        let d1 = UsbDevice::default();
        let d2 = UsbDevice::default();

        assert_eq!(d1, d2);
    }

    #[test]
    fn device_eq_empty_rev() {
        let d1 = UsbDevice::default();
        let d2 = UsbDevice::default();

        assert_eq!(d2, d1);
    }

    #[test]
    fn device_eq_match() {
        let d1 = UsbDevice {
            name: Some("foo".into()),
            ID_MODEL: Some("bar".into()),
            ID_MODEL_ENC: Some("baz".into()),
            ID_MODEL_FROM_DATABASE: Some("qux".into()),
            ID_MODEL_ID: Some("foog".into()),
            ID_SERIAL: Some("food".into()),
            ID_SERIAL_SHORT: Some("fool".into()),
            ID_VENDOR: Some("foob".into()),
            ID_VENDOR_ENC: Some("foof".into()),
            ID_VENDOR_FROM_DATABASE: Some("fooz".into()),
            ID_VENDOR_ID: Some("fooq".into()),
            PRODUCT: Some("foom".into()),
        };

        let d2 = UsbDevice {
            name: Some("foo".into()),
            ID_MODEL: Some("bar".into()),
            ID_MODEL_ENC: Some("baz".into()),
            ID_MODEL_FROM_DATABASE: Some("qux".into()),
            ID_MODEL_ID: Some("foog".into()),
            ID_SERIAL: Some("food".into()),
            ID_SERIAL_SHORT: Some("fool".into()),
            ID_VENDOR: Some("foob".into()),
            ID_VENDOR_ENC: Some("foof".into()),
            ID_VENDOR_FROM_DATABASE: Some("fooz".into()),
            ID_VENDOR_ID: Some("fooq".into()),
            PRODUCT: Some("foom".into()),
        };

        assert_eq!(d1, d2);
    }

    #[test]
    fn device_eq_match_rev() {
        let d1 = UsbDevice {
            name: Some("foo".into()),
            ID_MODEL: Some("bar".into()),
            ID_MODEL_ENC: Some("baz".into()),
            ID_MODEL_FROM_DATABASE: Some("qux".into()),
            ID_MODEL_ID: Some("foog".into()),
            ID_SERIAL: Some("food".into()),
            ID_SERIAL_SHORT: Some("fool".into()),
            ID_VENDOR: Some("foob".into()),
            ID_VENDOR_ENC: Some("foof".into()),
            ID_VENDOR_FROM_DATABASE: Some("fooz".into()),
            ID_VENDOR_ID: Some("fooq".into()),
            PRODUCT: Some("foom".into()),
        };

        let d2 = UsbDevice {
            name: Some("foo".into()),
            ID_MODEL: Some("bar".into()),
            ID_MODEL_ENC: Some("baz".into()),
            ID_MODEL_FROM_DATABASE: Some("qux".into()),
            ID_MODEL_ID: Some("foog".into()),
            ID_SERIAL: Some("food".into()),
            ID_SERIAL_SHORT: Some("fool".into()),
            ID_VENDOR: Some("foob".into()),
            ID_VENDOR_ENC: Some("foof".into()),
            ID_VENDOR_FROM_DATABASE: Some("fooz".into()),
            ID_VENDOR_ID: Some("fooq".into()),
            PRODUCT: Some("foom".into()),
        };

        assert_eq!(d2, d1);
    }
}
