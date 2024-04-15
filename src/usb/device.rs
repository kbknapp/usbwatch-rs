use std::fmt::{self, Debug};

use serde::{Deserialize, Serialize};
use yaml_rust::Yaml;

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct UsbDevices {
    pub devices: Vec<UsbDevice>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct UsbDevice {
    #[serde(serialize_with = "super::empty_if_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    id_model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    id_model_enc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    id_model_from_database: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    id_model_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    id_serial: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    id_serial_short: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    id_vendor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    id_vendor_enc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    id_vendor_from_database: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    id_vendor_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    product: Option<String>,
}

impl UsbDevice {
    pub fn new<S: Into<String>>(name: S) -> Self {
        Self {
            name: Some(name.into()),
            ..Default::default()
        }
    }

    pub fn is_empty(&self) -> bool {
        self.id_model.is_none()
            && self.id_model_enc.is_none()
            && self.id_model_from_database.is_none()
            && self.id_model_id.is_none()
            && self.id_vendor.is_none()
            && self.id_vendor_enc.is_none()
            && self.id_vendor_from_database.is_none()
            && self.id_vendor_id.is_none()
            && self.id_serial.is_none()
            && self.id_serial_short.is_none()
            && self.product.is_none()
    }
}

impl fmt::Display for UsbDevice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(ref name) = self.name {
            write!(f, "Device {{ name: {} }}", name)
        } else if let Some(ref product) = self.product {
            write!(f, "Device {{ product: {} }}", product)
        } else if let Some(ref serial) = self.id_serial {
            write!(f, "Device {{ serial: {} }}", serial)
        } else if let Some(ref model) = self.id_model {
            write!(f, "Device {{ model: {} }}", model)
        } else {
            write!(f, "Device {{ unk }}")
        }
    }
}

impl<'a> From<&'a tokio_udev::Device> for UsbDevice {
    fn from(d: &tokio_udev::Device) -> Self {
        Self {
            id_model: d
                .property_value("ID_MODEL")
                .map(|v| v.to_string_lossy().to_string()),
            id_model_enc: d
                .property_value("ID_MODEL_ENC")
                .map(|v| v.to_string_lossy().to_string()),
            id_model_from_database: d
                .property_value("ID_MODEL_FROM_DATABASE")
                .map(|v| v.to_string_lossy().to_string()),
            id_model_id: d
                .property_value("ID_MODEL_ID")
                .map(|v| v.to_string_lossy().to_string()),
            id_serial: d
                .property_value("ID_SERIAL")
                .map(|v| v.to_string_lossy().to_string()),
            id_serial_short: d
                .property_value("ID_SERIAL_SHORT")
                .map(|v| v.to_string_lossy().to_string()),
            id_vendor: d
                .property_value("ID_VENDOR")
                .map(|v| v.to_string_lossy().to_string()),
            id_vendor_enc: d
                .property_value("ID_VENDOR_ENC")
                .map(|v| v.to_string_lossy().to_string()),
            id_vendor_from_database: d
                .property_value("ID_VENDOR_FROM_DATABASE")
                .map(|v| v.to_string_lossy().to_string()),
            id_vendor_id: d
                .property_value("ID_VENDOR_ID")
                .map(|v| v.to_string_lossy().to_string()),
            product: d
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
            cli_bail!("faild to parse YAML for Device; missing required 'name' key");
        };

        yaml_str!(device, yaml, id_model);
        yaml_str!(device, yaml, id_model_enc);
        yaml_str!(device, yaml, id_model_from_database);
        yaml_str!(device, yaml, id_model_id);
        yaml_str!(device, yaml, id_vendor);
        yaml_str!(device, yaml, id_vendor_enc);
        yaml_str!(device, yaml, id_vendor_from_database);
        yaml_str!(device, yaml, id_vendor_id);
        yaml_str!(device, yaml, id_serial);
        yaml_str!(device, yaml, id_serial_short);
        yaml_str!(device, yaml, product);

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
        cmp_ignore_none!(self, other, id_model);
        cmp_ignore_none!(self, other, id_model_enc);
        cmp_ignore_none!(self, other, id_model_from_database);
        cmp_ignore_none!(self, other, id_model_id);
        cmp_ignore_none!(self, other, id_vendor);
        cmp_ignore_none!(self, other, id_vendor_enc);
        cmp_ignore_none!(self, other, id_vendor_from_database);
        cmp_ignore_none!(self, other, id_vendor_id);
        cmp_ignore_none!(self, other, id_serial);
        cmp_ignore_none!(self, other, id_serial_short);
        cmp_ignore_none!(self, other, product);

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
            id_model: Some("bar".into()),
            id_model_enc: Some("baz".into()),
            id_model_from_database: Some("qux".into()),
            id_model_id: Some("foog".into()),
            id_serial: Some("food".into()),
            id_serial_short: Some("fool".into()),
            id_vendor: Some("foob".into()),
            id_vendor_enc: Some("foof".into()),
            id_vendor_from_database: Some("fooz".into()),
            id_vendor_id: Some("fooq".into()),
            product: Some("foom".into()),
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
            id_model: Some("bar".into()),
            id_model_enc: Some("baz".into()),
            id_model_from_database: Some("qux".into()),
            id_model_id: Some("foog".into()),
            id_serial: Some("food".into()),
            id_serial_short: Some("fool".into()),
            id_vendor: Some("foob".into()),
            id_vendor_enc: Some("foof".into()),
            id_vendor_from_database: Some("fooz".into()),
            id_vendor_id: Some("fooq".into()),
            product: Some("foom".into()),
        };

        // Less Specific
        let d2 = UsbDevice {
            name: Some("foo".into()),
            id_model_enc: Some("baz".into()),
            id_model_id: Some("foog".into()),
            id_serial: Some("food".into()),
            id_vendor_enc: Some("foof".into()),
            id_vendor_id: Some("fooq".into()),
            ..Default::default()
        };

        assert_eq!(d1, d2);
    }

    #[test]
    fn device_eq_2() {
        // More Specific
        let d1 = UsbDevice {
            name: Some("foo".into()),
            id_model: Some("bar".into()),
            id_model_enc: Some("baz".into()),
            id_model_from_database: Some("qux".into()),
            id_model_id: Some("foog".into()),
            id_serial: Some("food".into()),
            id_serial_short: Some("fool".into()),
            id_vendor: Some("foob".into()),
            id_vendor_enc: Some("foof".into()),
            id_vendor_from_database: Some("fooz".into()),
            id_vendor_id: Some("fooq".into()),
            product: Some("foom".into()),
        };

        // Less Specific
        let d2 = UsbDevice {
            name: Some("foo".into()),
            id_model: Some("bar".into()),
            ..Default::default()
        };

        assert_eq!(d1, d2);
    }

    #[test]
    fn device_eq_rev() {
        // More Specific
        let d1 = UsbDevice {
            name: Some("foo".into()),
            id_model: Some("bar".into()),
            id_model_enc: Some("baz".into()),
            id_model_from_database: Some("qux".into()),
            id_model_id: Some("foog".into()),
            id_serial: Some("food".into()),
            id_serial_short: Some("fool".into()),
            id_vendor: Some("foob".into()),
            id_vendor_enc: Some("foof".into()),
            id_vendor_from_database: Some("fooz".into()),
            id_vendor_id: Some("fooq".into()),
            product: Some("foom".into()),
        };

        // Less Specific
        let d2 = UsbDevice {
            name: Some("foo".into()),
            id_model_enc: Some("baz".into()),
            id_model_id: Some("foog".into()),
            id_serial: Some("food".into()),
            id_vendor_enc: Some("foof".into()),
            id_vendor_id: Some("fooq".into()),
            ..Default::default()
        };

        assert_eq!(d2, d1);
    }

    #[test]
    fn device_eq_2_rev() {
        // More Specific
        let d1 = UsbDevice {
            name: Some("foo".into()),
            id_model: Some("bar".into()),
            id_model_enc: Some("baz".into()),
            id_model_from_database: Some("qux".into()),
            id_model_id: Some("foog".into()),
            id_serial: Some("food".into()),
            id_serial_short: Some("fool".into()),
            id_vendor: Some("foob".into()),
            id_vendor_enc: Some("foof".into()),
            id_vendor_from_database: Some("fooz".into()),
            id_vendor_id: Some("fooq".into()),
            product: Some("foom".into()),
        };

        // Less Specific
        let d2 = UsbDevice {
            name: Some("baa".into()),
            id_serial: Some("food".into()),
            ..Default::default()
        };

        assert_eq!(d2, d1);
    }

    #[test]
    fn device_eq_one_empty() {
        // More Specific
        let d1 = UsbDevice {
            name: Some("foo".into()),
            id_model: Some("bar".into()),
            id_model_enc: Some("baz".into()),
            id_model_from_database: Some("qux".into()),
            id_model_id: Some("foog".into()),
            id_serial: Some("food".into()),
            id_serial_short: Some("fool".into()),
            id_vendor: Some("foob".into()),
            id_vendor_enc: Some("foof".into()),
            id_vendor_from_database: Some("fooz".into()),
            id_vendor_id: Some("fooq".into()),
            product: Some("foom".into()),
        };

        // Less Specific
        let d2 = UsbDevice::default();

        assert!(d1 != d2);
    }

    #[test]
    fn device_eq_one_empty_rev() {
        let d1 = UsbDevice {
            name: Some("foo".into()),
            id_model: Some("bar".into()),
            id_model_enc: Some("baz".into()),
            id_model_from_database: Some("qux".into()),
            id_model_id: Some("foog".into()),
            id_serial: Some("food".into()),
            id_serial_short: Some("fool".into()),
            id_vendor: Some("foob".into()),
            id_vendor_enc: Some("foof".into()),
            id_vendor_from_database: Some("fooz".into()),
            id_vendor_id: Some("fooq".into()),
            product: Some("foom".into()),
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
            id_model: Some("bar".into()),
            id_model_enc: Some("baz".into()),
            id_model_from_database: Some("qux".into()),
            id_model_id: Some("foog".into()),
            id_serial: Some("food".into()),
            id_serial_short: Some("fool".into()),
            id_vendor: Some("foob".into()),
            id_vendor_enc: Some("foof".into()),
            id_vendor_from_database: Some("fooz".into()),
            id_vendor_id: Some("fooq".into()),
            product: Some("foom".into()),
        };

        let d2 = UsbDevice {
            name: Some("foo".into()),
            id_model: Some("bar".into()),
            id_model_enc: Some("baz".into()),
            id_model_from_database: Some("qux".into()),
            id_model_id: Some("foog".into()),
            id_serial: Some("food".into()),
            id_serial_short: Some("fool".into()),
            id_vendor: Some("foob".into()),
            id_vendor_enc: Some("foof".into()),
            id_vendor_from_database: Some("fooz".into()),
            id_vendor_id: Some("fooq".into()),
            product: Some("foom".into()),
        };

        assert_eq!(d1, d2);
    }

    #[test]
    fn device_eq_match_rev() {
        let d1 = UsbDevice {
            name: Some("foo".into()),
            id_model: Some("bar".into()),
            id_model_enc: Some("baz".into()),
            id_model_from_database: Some("qux".into()),
            id_model_id: Some("foog".into()),
            id_serial: Some("food".into()),
            id_serial_short: Some("fool".into()),
            id_vendor: Some("foob".into()),
            id_vendor_enc: Some("foof".into()),
            id_vendor_from_database: Some("fooz".into()),
            id_vendor_id: Some("fooq".into()),
            product: Some("foom".into()),
        };

        let d2 = UsbDevice {
            name: Some("foo".into()),
            id_model: Some("bar".into()),
            id_model_enc: Some("baz".into()),
            id_model_from_database: Some("qux".into()),
            id_model_id: Some("foog".into()),
            id_serial: Some("food".into()),
            id_serial_short: Some("fool".into()),
            id_vendor: Some("foob".into()),
            id_vendor_enc: Some("foof".into()),
            id_vendor_from_database: Some("fooz".into()),
            id_vendor_id: Some("fooq".into()),
            product: Some("foom".into()),
        };

        assert_eq!(d2, d1);
    }
}
