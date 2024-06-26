mod r#match;

use std::{fmt::Debug, path::PathBuf};

use serde::Serialize;
use tracing::{debug, span, Level};
use yaml_rust::Yaml;

use crate::udev::UdevEvent;

use r#match::Match;

#[derive(Serialize, Debug, PartialEq)]
pub struct Rules {
    pub rules: Vec<Rule>,
}

impl<'a> From<&'a Yaml> for Rules {
    fn from(yaml: &'a Yaml) -> Self {
        let mut rules = Vec::new();
        if let Some(yaml_rules) = yaml["rules"].as_vec() {
            for r in yaml_rules {
                rules.push(Rule::from(r));
            }
        }

        Self { rules }
    }
}

#[derive(Serialize, PartialEq, Debug)]
pub struct Rule {
    pub name: String,
    r#match: Match,
    pub command_shell: PathBuf,
    pub command: String,
}

impl Rule {
    pub fn matches_udev_event(&self, event: &UdevEvent) -> bool {
        let span = span!(Level::TRACE, "fn matches_udev_event", rule = %self.name);
        let _enter = span.enter();

        self.r#match.matches_udev_event(event)
    }
}

impl<'a> From<&'a Yaml> for Rule {
    fn from(yaml: &'a Yaml) -> Self {
        let span = span!(Level::TRACE, "fn From::<Yaml>");
        let _enter = span.enter();

        let name: String = if let Some(name) = yaml["name"].as_str() {
            debug!(name = %name, "Building Rule");
            name.into()
        } else {
            todo!("Handle Rule::from<Yaml> with no name key")
        };

        let yaml_match = &yaml["match"];
        let m = if !yaml_match.is_badvalue() {
            Match::from(yaml_match)
        } else {
            cli_bail!("failed to parse YAML for Rule; missing required 'match' key");
        };

        let command_shell = if let Some(s) = yaml["command_shell"].as_str() {
            PathBuf::from(s)
        } else {
            PathBuf::from("/bin/sh")
        };

        let command: String = if let Some(c) = yaml["command"].as_str() {
            c.into()
        } else {
            cli_bail!("failed to parse YAML for Rule; missing required 'command' key");
        };

        Rule {
            name,
            r#match: m,
            command_shell,
            command,
        }
    }
}
