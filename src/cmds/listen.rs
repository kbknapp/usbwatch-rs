










use crate::{cli::ListenArgs, listen};

pub fn run(_la: ListenArgs) {
    tokio::runtime::Builder::new_current_thread()
        .enable_io()
        .build()
        .unwrap()
        .block_on(listen::listen())
}

