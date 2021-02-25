use std::fmt::Debug;

use serde::{Deserialize, Serialize};
use yaml_rust::Yaml;

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct UsbPorts {
    pub ports: Vec<UsbPort>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct UsbPort {
    #[serde(serialize_with = "super::empty_if_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    syspath: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    devpath: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    sysname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    sysnum: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    ID_FOR_SEAT: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    ID_PATH: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    ID_PATH_TAG: Option<String>,
}

impl UsbPort {
    pub fn new<S: Into<String>>(name: S) -> Self {
        Self {
            name: Some(name.into()),
            ..Default::default()
        }
    }

    pub fn is_empty(&self) -> bool {
        self.syspath.is_none()
            && self.devpath.is_none()
            && self.sysname.is_none()
            && self.sysnum.is_none()
            && self.ID_FOR_SEAT.is_none()
            && self.ID_PATH.is_none()
            && self.ID_PATH_TAG.is_none()
    }
}

impl<'a> From<&'a tokio_udev::Device> for UsbPort {
    fn from(d: &tokio_udev::Device) -> Self {
        Self {
            syspath: Some(d.syspath().to_string_lossy().to_string()),
            devpath: Some(d.devpath().to_string_lossy().to_string()),
            sysname: Some(d.sysname().to_string_lossy().to_string()),
            sysnum: d.sysnum(),
            ID_FOR_SEAT: d
                .property_value("ID_FOR_SEAT")
                .map(|v| v.to_string_lossy().to_string()),
            ID_PATH: d
                .property_value("ID_PATH")
                .map(|v| v.to_string_lossy().to_string()),
            ID_PATH_TAG: d
                .property_value("ID_PATH_TAG")
                .map(|v| v.to_string_lossy().to_string()),
            ..Default::default()
        }
    }
}

impl<'a> From<&'a Yaml> for UsbPort {
    fn from(yaml: &'a Yaml) -> Self {
        let mut port = if let Some(name) = yaml["name"].as_str() {
            UsbPort::new(name)
        } else {
            todo!("Handle Port::from<Yaml> with no name key")
        };

        yaml_str!(port, yaml, syspath);
        yaml_str!(port, yaml, devpath);
        yaml_str!(port, yaml, sysname);
        yaml_str!(port, yaml, ID_FOR_SEAT);
        yaml_str!(port, yaml, ID_PATH);
        yaml_str!(port, yaml, ID_PATH_TAG);

        if let Some(v) = yaml["sysnum"].as_i64() {
            port.sysnum = Some(v as usize);
        }

        port
    }
}

impl PartialEq<UsbPort> for UsbPort {
    fn eq(&self, other: &UsbPort) -> bool {
        // This impl ignores None values for either LHS or RHS

        // We don't want an empty port to always match
        match (self.is_empty(), other.is_empty()) {
            (true, true) => return true,
            (true, false) | (false, true) => return false,
            (false, false) => (),
        };

        // We don't compare name because it's always none from one side or the other
        cmp_ignore_none!(self, other, syspath);
        cmp_ignore_none!(self, other, devpath);
        cmp_ignore_none!(self, other, sysname);
        cmp_ignore_none!(self, other, sysnum);
        cmp_ignore_none!(self, other, ID_FOR_SEAT);
        cmp_ignore_none!(self, other, ID_PATH);
        cmp_ignore_none!(self, other, ID_PATH_TAG);

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn port_eq_name_only_but_diff() {
        // More Specific
        let p1 = UsbPort {
            name: Some("baa".into()),
            syspath: Some("foo".into()),
            devpath: Some("bar".into()),
            sysname: Some("baz".into()),
            sysnum: Some(5),
            ID_FOR_SEAT: Some("foog".into()),
            ID_PATH: Some("fool".into()),
            ID_PATH_TAG: Some("foop".into()),
        };

        // Less Specific
        let p2 = UsbPort {
            name: Some("baagle".into()),
            ..Default::default()
        };

        assert!(p1 != p2);
    }

    #[test]
    fn port_eq() {
        // More Specific
        let p1 = UsbPort {
            name: Some("baa".into()),
            syspath: Some("foo".into()),
            devpath: Some("bar".into()),
            sysname: Some("baz".into()),
            sysnum: Some(5),
            ID_FOR_SEAT: Some("foog".into()),
            ID_PATH: Some("fool".into()),
            ID_PATH_TAG: Some("foop".into()),
        };

        // Less Specific
        let p2 = UsbPort {
            name: Some("baa".into()),
            syspath: Some("foo".into()),
            sysname: Some("baz".into()),
            ID_FOR_SEAT: Some("foog".into()),
            ID_PATH_TAG: Some("foop".into()),
            ..Default::default()
        };

        assert_eq!(p1, p2);
    }

    #[test]
    fn port_eq_2() {
        // More Specific
        let p1 = UsbPort {
            name: Some("baa".into()),
            syspath: Some("foo".into()),
            devpath: Some("bar".into()),
            sysname: Some("baz".into()),
            sysnum: Some(5),
            ID_FOR_SEAT: Some("foog".into()),
            ID_PATH: Some("fool".into()),
            ID_PATH_TAG: Some("foop".into()),
        };

        // Less Specific
        let p2 = UsbPort {
            name: Some("baa".into()),
            ..Default::default()
        };

        assert_eq!(p1, p2);
    }

    #[test]
    fn port_eq_rev() {
        // More Specific
        let p1 = UsbPort {
            name: Some("baa".into()),
            syspath: Some("foo".into()),
            devpath: Some("bar".into()),
            sysname: Some("baz".into()),
            sysnum: Some(5),
            ID_FOR_SEAT: Some("foog".into()),
            ID_PATH: Some("fool".into()),
            ID_PATH_TAG: Some("foop".into()),
        };

        // Less Specific
        let p2 = UsbPort {
            name: Some("baa".into()),
            syspath: Some("foo".into()),
            sysname: Some("baz".into()),
            ID_FOR_SEAT: Some("foog".into()),
            ID_PATH_TAG: Some("foop".into()),
            ..Default::default()
        };

        assert_eq!(p2, p1);
    }

    #[test]
    fn port_eq_2_rev() {
        // More Specific
        let p1 = UsbPort {
            name: Some("baa".into()),
            syspath: Some("foo".into()),
            devpath: Some("bar".into()),
            sysname: Some("baz".into()),
            sysnum: Some(5),
            ID_FOR_SEAT: Some("foog".into()),
            ID_PATH: Some("fool".into()),
            ID_PATH_TAG: Some("foop".into()),
        };

        // Less Specific
        let p2 = UsbPort {
            name: Some("baa".into()),
            ID_PATH: Some("fool".into()),
            ..Default::default()
        };

        assert_eq!(p2, p1);
    }

    #[test]
    fn port_eq_one_empty() {
        // More Specific
        let p1 = UsbPort {
            name: Some("baa".into()),
            syspath: Some("foo".into()),
            devpath: Some("bar".into()),
            sysname: Some("baz".into()),
            sysnum: Some(5),
            ID_FOR_SEAT: Some("foog".into()),
            ID_PATH: Some("fool".into()),
            ID_PATH_TAG: Some("foop".into()),
        };

        // Less Specific
        let p2 = UsbPort::default();

        assert!(p1 != p2);
    }

    #[test]
    fn port_eq_one_empty_rev() {
        let p1 = UsbPort {
            name: Some("baa".into()),
            syspath: Some("foo".into()),
            devpath: Some("bar".into()),
            sysname: Some("baz".into()),
            sysnum: Some(5),
            ID_FOR_SEAT: Some("foog".into()),
            ID_PATH: Some("fool".into()),
            ID_PATH_TAG: Some("foop".into()),
        };

        let p2 = UsbPort::default();

        assert!(p2 != p1);
    }

    #[test]
    fn port_eq_empty() {
        let p1 = UsbPort::default();
        let p2 = UsbPort::default();

        assert_eq!(p1, p2);
    }

    #[test]
    fn port_eq_empty_rev() {
        let p1 = UsbPort::default();
        let p2 = UsbPort::default();

        assert_eq!(p2, p1);
    }

    #[test]
    fn port_eq_match() {
        let p1 = UsbPort {
            name: Some("baa".into()),
            syspath: Some("foo".into()),
            devpath: Some("bar".into()),
            sysname: Some("baz".into()),
            sysnum: Some(5),
            ID_FOR_SEAT: Some("foog".into()),
            ID_PATH: Some("fool".into()),
            ID_PATH_TAG: Some("foop".into()),
        };

        let p2 = UsbPort {
            name: Some("baa".into()),
            syspath: Some("foo".into()),
            devpath: Some("bar".into()),
            sysname: Some("baz".into()),
            sysnum: Some(5),
            ID_FOR_SEAT: Some("foog".into()),
            ID_PATH: Some("fool".into()),
            ID_PATH_TAG: Some("foop".into()),
        };

        assert_eq!(p1, p2);
    }

    #[test]
    fn port_eq_match_rev() {
        let p1 = UsbPort {
            name: Some("baa".into()),
            syspath: Some("foo".into()),
            devpath: Some("bar".into()),
            sysname: Some("baz".into()),
            sysnum: Some(5),
            ID_FOR_SEAT: Some("foog".into()),
            ID_PATH: Some("fool".into()),
            ID_PATH_TAG: Some("foop".into()),
        };

        let p2 = UsbPort {
            name: Some("baa".into()),
            syspath: Some("foo".into()),
            devpath: Some("bar".into()),
            sysname: Some("baz".into()),
            sysnum: Some(5),
            ID_FOR_SEAT: Some("foog".into()),
            ID_PATH: Some("fool".into()),
            ID_PATH_TAG: Some("foop".into()),
        };

        assert_eq!(p2, p1);
    }
}
