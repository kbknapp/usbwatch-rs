use tracing::{self, debug, error, info, instrument, trace, warn};

use crate::cli::CreateDeviceArgs;

#[tracing::instrument]
pub fn run(_cda: CreateDeviceArgs) {
    warn!("Not yet implemented");
}
