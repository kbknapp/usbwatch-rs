use crate::printer::{ColorChoice, OutFormat};

#[derive(Default)]
pub struct Ctx {
    pub verbose: u8,
    // Are we using trace logging, or human CLI output
    pub tracing: bool,
    pub format: OutFormat,
    pub color: ColorChoice,
    pub num_events: usize,
}
