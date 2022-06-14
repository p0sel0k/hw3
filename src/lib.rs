mod devices;
mod home;

pub use devices::{SmartSocket, SmartThermometer};
pub use home::{Home, HomeError, Room};
