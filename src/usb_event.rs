use clap::Clap;

#[derive(Clap, Copy, Clone, PartialEq, Debug)]
pub enum UsbEvent {
    Connect,
    Disconnect,
    All
}
