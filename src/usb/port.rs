use std::fmt::{self, Debug};

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
    #[serde(
        rename = "ID_FOR_SEAT",
        skip_serializing_if = "Option::is_none",
        default
    )]
    id_for_seat: Option<String>,
    #[serde(rename = "ID_PATH", skip_serializing_if = "Option::is_none", default)]
    id_path: Option<String>,
    #[serde(
        rename = "ID_PATH_TAG",
        skip_serializing_if = "Option::is_none",
        default
    )]
    id_path_tag: Option<String>,
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
            && self.id_for_seat.is_none()
            && self.id_path.is_none()
            && self.id_path_tag.is_none()
    }
}

impl fmt::Display for UsbPort {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(ref name) = self.name {
            write!(f, "Port {{ name: {} }}", name)
        } else if let Some(ref name) = self.sysname {
            write!(f, "Port {{ sysname: {} }}", name)
        } else if let Some(ref path) = self.syspath {
            write!(f, "Port {{ syspath: {} }}", path)
        } else if let Some(ref path) = self.devpath {
            write!(f, "Port {{ devpath: {} }}", path)
        } else {
            write!(f, "Port {{ unk }}")
        }
    }
}

impl<'a> From<&'a tokio_udev::Device> for UsbPort {
    fn from(d: &tokio_udev::Device) -> Self {
        Self {
            syspath: Some(d.syspath().to_string_lossy().to_string()),
            devpath: Some(d.devpath().to_string_lossy().to_string()),
            sysname: Some(d.sysname().to_string_lossy().to_string()),
            sysnum: d.sysnum(),
            id_for_seat: d
                .property_value("ID_FOR_SEAT")
                .map(|v| v.to_string_lossy().to_string()),
            id_path: d
                .property_value("ID_PATH")
                .map(|v| v.to_string_lossy().to_string()),
            id_path_tag: d
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
        yaml_str!(port, yaml, id_for_seat);
        yaml_str!(port, yaml, id_path);
        yaml_str!(port, yaml, id_path_tag);

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
        cmp_ignore_none!(self, other, id_for_seat);
        cmp_ignore_none!(self, other, id_path);
        cmp_ignore_none!(self, other, id_path_tag);

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
            id_for_seat: Some("foog".into()),
            id_path: Some("fool".into()),
            id_path_tag: Some("foop".into()),
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
            id_for_seat: Some("foog".into()),
            id_path: Some("fool".into()),
            id_path_tag: Some("foop".into()),
        };

        // Less Specific
        let p2 = UsbPort {
            name: Some("baa".into()),
            syspath: Some("foo".into()),
            sysname: Some("baz".into()),
            id_for_seat: Some("foog".into()),
            id_path_tag: Some("foop".into()),
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
            id_for_seat: Some("foog".into()),
            id_path: Some("fool".into()),
            id_path_tag: Some("foop".into()),
        };

        // Less Specific
        let p2 = UsbPort {
            name: Some("baa".into()),
            syspath: Some("foo".into()), // name is ignored
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
            id_for_seat: Some("foog".into()),
            id_path: Some("fool".into()),
            id_path_tag: Some("foop".into()),
        };

        // Less Specific
        let p2 = UsbPort {
            name: Some("baa".into()),
            syspath: Some("foo".into()),
            sysname: Some("baz".into()),
            id_for_seat: Some("foog".into()),
            id_path_tag: Some("foop".into()),
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
            id_for_seat: Some("foog".into()),
            id_path: Some("fool".into()),
            id_path_tag: Some("foop".into()),
        };

        // Less Specific
        let p2 = UsbPort {
            name: Some("baa".into()),
            id_path: Some("fool".into()),
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
            id_for_seat: Some("foog".into()),
            id_path: Some("fool".into()),
            id_path_tag: Some("foop".into()),
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
            id_for_seat: Some("foog".into()),
            id_path: Some("fool".into()),
            id_path_tag: Some("foop".into()),
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
            id_for_seat: Some("foog".into()),
            id_path: Some("fool".into()),
            id_path_tag: Some("foop".into()),
        };

        let p2 = UsbPort {
            name: Some("baa".into()),
            syspath: Some("foo".into()),
            devpath: Some("bar".into()),
            sysname: Some("baz".into()),
            sysnum: Some(5),
            id_for_seat: Some("foog".into()),
            id_path: Some("fool".into()),
            id_path_tag: Some("foop".into()),
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
            id_for_seat: Some("foog".into()),
            id_path: Some("fool".into()),
            id_path_tag: Some("foop".into()),
        };

        let p2 = UsbPort {
            name: Some("baa".into()),
            syspath: Some("foo".into()),
            devpath: Some("bar".into()),
            sysname: Some("baz".into()),
            sysnum: Some(5),
            id_for_seat: Some("foog".into()),
            id_path: Some("fool".into()),
            id_path_tag: Some("foop".into()),
        };

        assert_eq!(p2, p1);
    }
}
