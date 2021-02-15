mod r#match;

use std::{path::PathBuf, ffi::OsString, fmt::{self, Debug}};

use serde::{Serialize};
use yaml_rust::Yaml;

use r#match::Match;

#[derive(Serialize, Debug, PartialEq)]
pub struct Rules {
    rules: Vec<Rule>,
}

impl<'a> From<&'a Yaml> for Rules {
    fn from(yaml: &'a Yaml) -> Self {
        let mut rules = Vec::new();
        if let Some(yaml_rules) = yaml["rules"].as_vec() {
            for r in yaml_rules {
                rules.push(Rule::from(r));
            }
        }

        Self {
            rules,
        }
    }
}

#[derive(Serialize, PartialEq, Debug)]
pub struct Rule {
    name: String,
    r#match: Match,
    command_shell: PathBuf,
    command: String
}

impl<'a> From<&'a Yaml> for Rule {
    fn from(yaml: &'a Yaml) -> Self {
        let name: String = if let Some(name) = yaml["name"].as_str() {
            name.into()
        } else {
            todo!("Handle Rule::from<Yaml> with no name key")
        };

        let yaml_match = &yaml["match"];
        let m = if !yaml_match.is_badvalue() {
            Match::from(yaml_match)
        } else {
            todo!("handle no match key when deserializing rule from yaml")
        };

        let command_shell = if let Some(s) = yaml["command_shell"].as_str() {
            PathBuf::from(s)
        } else {
            // @TODO un-hardcode /bin/bash
            PathBuf::from("/bin/bash")
        };

        let command: String = if let Some(c) = yaml["command"].as_str() {
            c.into()
        } else {
            todo!("Handle no command key in Rule::from<Yaml>")
        };

        Rule {
            name,
            r#match: m,
            command_shell,
            command
        }
    }
}
