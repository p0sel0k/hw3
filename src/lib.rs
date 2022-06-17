mod devices;
mod home;

pub use devices::{DeviceError, SmartDevice, SmartSocket, SmartThermometer};
pub use home::{Home, HomeError, Room};
