use std::ffi::OsStr;
use std::future::Future;

use futures_core::stream::Stream;
use tokio::sync::{broadcast, mpsc, Semaphore};
use tokio::time::{self, Duration};
use tokio::signal;
use tokio_stream::StreamExt;
use tracing::{debug, error, info, instrument};
use tokio_udev::{self, EventType};

use crate::{cli::ListenArgs, tokio_udev::DebugDevice, listen};

pub fn run(la: ListenArgs) {
    tokio::runtime::Builder::new_current_thread()
        .enable_io()
        .build()
        .unwrap()
        .block_on(listen::listen())
}

