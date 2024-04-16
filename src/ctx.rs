use crate::printer::ColorChoice;

#[derive(Default)]
pub struct Ctx {
    pub verbose: u8,
    // Are we using trace logging, or human CLI output
    pub tracing: bool,
    pub color: ColorChoice,
}
