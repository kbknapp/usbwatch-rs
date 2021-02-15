use std::{ffi::OsString, fmt::{self, Debug}};

use serde::{Deserialize, Serialize};
use yaml_rust::Yaml;

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct Ports {
    pub ports: Vec<Port>
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Default)]
pub struct Port {
    name: String,
    #[serde(default)]
    syspath: Option<String>,
    #[serde(default)]
    devpath: Option<String>,
    #[serde(default)]
    sysname: Option<String>,
    #[serde(default)]
    sysnum: Option<i64>,
    #[serde(default)]
    DEVPATH: Option<String>,
    #[serde(default)]
    ID_FOR_SEAT: Option<String>,
    #[serde(default)]
    ID_PATH: Option<String>,
    #[serde(default)]
    ID_PATH_TAG: Option<String>,
}

impl Port {
    pub fn new<S: Into<String>>(name: S) -> Self {
        Self {
            name: name.into(),
            ..Default::default()
        }
    }
}

impl<'a> From<&'a Yaml> for Port {
    fn from(mut yaml: &'a Yaml) -> Self {
        let mut port = if let Some(name) = yaml["name"].as_str() {
            Port::new(name)
        } else {
            todo!("Handle Port::from<Yaml> with no name key")
        };

        yaml_str!(port, yaml, syspath);
        yaml_str!(port, yaml, devpath);
        yaml_str!(port, yaml, sysname);
        yaml_str!(port, yaml, DEVPATH);
        yaml_str!(port, yaml, ID_FOR_SEAT);
        yaml_str!(port, yaml, ID_PATH);
        yaml_str!(port, yaml, ID_PATH_TAG);

        if let Some(v) = yaml["sysnum"].as_i64() {
            port.sysnum = Some(v);
        }

        port
    }
}
