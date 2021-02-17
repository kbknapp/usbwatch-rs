use tracing::{self, debug, warn, error, info, trace, instrument};

use crate::cli::CreateDeviceArgs;

#[tracing::instrument]
pub fn run(_cda: CreateDeviceArgs) {
    warn!("Not yet implemented");

}
