use crate::{devices::SmartDevice, SmartSocket, SmartThermometer};
use anyhow::{Context, Result};
use std::{
    collections::HashMap,
    io::{self, Write},
};
use thiserror::Error;
// use std::error::Error;
// use std::fmt::Display;

#[derive(Error, Debug)]
pub enum HomeError {
    #[error("No room in home")]
    NoRoomInHoom,

    #[error("No device in room")]
    NoDeviceInRoom,

    #[error("Can't add room")]
    CantAddRoom,

    #[error("Can't add device")]
    CantAddDevice,

    #[error("Can't read room from input")]
    CantInputRoom(#[source] io::Error),

    #[error("Can't read device from input")]
    CantInputDevice(#[source] io::Error),
}

#[derive(Default)]
pub struct Home {
    _name: String,
    rooms: HashMap<String, Room>,
}

impl Home {
    pub fn new(name: String) -> Self {
        let rooms = HashMap::new();
        Home { _name: name, rooms }
    }

    pub fn add_room(&mut self, room: Room) -> String {
        match self.rooms.insert(room.name.clone(), room) {
            Some(old_room) => format!("Existed room was updated: {:?}", old_room),
            None => "New room added".into(),
        }
    }

    pub fn input_new_room(&mut self) -> Result<String> {
        let mut room = String::new();
        print!("Input room name: ");
        io::stdout().flush()?;
        if let Err(err) = io::stdin().read_line(&mut room) {
            return Err(HomeError::CantInputRoom(err)).context("Can't read room name from stdin");
        }
        let room = room.trim();
        Ok(format!(
            "{}: {}",
            room,
            self.add_room(Room::new(room.into()))
        ))
    }

    pub fn add_device(
        &mut self,
        room_name: &str,
        device: Box<dyn SmartDevice>,
    ) -> Result<&dyn SmartDevice> {
        let r = self.get_room(room_name)?;
        let name = device.name().to_string();
        r.add_device(device);
        Ok(self
            .get_room(room_name)?
            .get_device(name.as_str())
            .ok_or(HomeError::NoDeviceInRoom)?)
    }

    pub fn input_new_device(&mut self) -> Result<&dyn SmartDevice> {
        let mut room = String::new();
        let mut device_name = String::new();
        let mut device_type = String::new();

        print!("Input room name: ");
        io::stdout().flush()?;
        if let Err(err) = io::stdin().read_line(&mut room) {
            return Err(HomeError::CantInputRoom(err)).context("Can't read room name from stdin");
        }

        print!("\nChoose device type: \n1) Smart Socket \n2) Smart Thermometer");
        io::stdout().flush()?;
        if let Err(err) = io::stdin().read_line(&mut device_type) {
            return Err(HomeError::CantInputDevice(err))
                .context("Can't read device type from stdin");
        }

        print!("\nInput device name: ");
        io::stdout().flush()?;
        if let Err(err) = io::stdin().read_line(&mut device_name) {
            return Err(HomeError::CantInputDevice(err))
                .context("Can't read device name from stdin");
        }

        let device_name = device_name.trim();
        let device: Option<Box<dyn SmartDevice>> = match device_type.trim() {
            "1" => Some(Box::new(SmartSocket::new(device_name.into()))),
            "2" => Some(Box::new(SmartThermometer::new(device_name.into()))),
            _ => None,
        };

        self.add_device(
            room.as_str().trim(),
            device
                .ok_or(HomeError::CantAddDevice)
                .context("Device is NONE")?,
        )
    }

    pub fn get_room(&mut self, name: &str) -> Result<&mut Room> {
        for (room_name, room) in &mut self.rooms {
            if room_name == name {
                return Ok(room);
            }
        }
        Err(HomeError::NoRoomInHoom).context(format!("room: {} doesn't exist in this home", name))
    }

    pub fn remove_room(&mut self, name: &str) -> Result<Room> {
        match self.rooms.remove(name) {
            Some(room) => Ok(room),
            None => Err(HomeError::NoRoomInHoom)
                .context(format!("room: {} doesn't exist in this home", name)),
        }
    }

    pub fn remove_room_by_name(&mut self) -> Result<Room> {
        let mut room = String::new();
        if let Err(err) = io::stdin().read_line(&mut room) {
            return Err(HomeError::CantInputRoom(err)).context("Can't read room name from stdin");
        }
        self.remove_room(room.trim())
    }

    pub fn remove_device(
        &mut self,
        room_name: &str,
        device_name: &str,
    ) -> Result<Box<dyn SmartDevice>> {
        self.get_room(room_name)?.remove_device(device_name)
    }

    pub fn remove_device_by_name(&mut self) -> Result<Box<dyn SmartDevice>> {
        let mut room = String::new();
        let mut device = String::new();
        if let Err(err) = io::stdin().read_line(&mut room) {
            return Err(HomeError::CantInputRoom(err)).context("Can't read room name from stdin");
        }
        if let Err(err) = io::stdin().read_line(&mut device) {
            return Err(HomeError::CantInputDevice(err))
                .context("Can't read device name from stdin");
        }
        self.remove_device(room.trim(), device.trim())
    }

    pub fn write_report<T: Write>(&self, mut writer: T) -> Result<T> {
        let mut home_report = String::from("\n---------Home Report--------\n");
        for (name, room) in &self.rooms {
            home_report.push_str(format!("Room {} info\n", name).as_str());
            home_report.push_str(room.devices_state()?.as_str());
        }
        writer
            .write_all(home_report.as_bytes())
            .context("Can't write report to current writer")?;
        Ok(writer)
    }
}

pub struct Room {
    name: String,
    devices: HashMap<String, Box<dyn SmartDevice>>,
}

impl std::fmt::Debug for Room {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Room").field("name", &self.name).finish()
    }
}

impl Room {
    pub fn new(name: String) -> Self {
        Room {
            devices: HashMap::new(),
            name,
        }
    }

    fn add_device(&mut self, device: Box<dyn SmartDevice>) {
        self.devices.insert(device.name().to_string(), device);
    }

    pub fn get_device(&mut self, name: &str) -> Option<&mut dyn SmartDevice> {
        for (device_name, device) in &mut self.devices {
            if device_name == name {
                return Some(device.as_mut());
            }
        }
        None
    }

    fn remove_device(&mut self, name: &str) -> Result<Box<dyn SmartDevice>> {
        match self.devices.remove(name) {
            Some(device) => Ok(device),
            None => Err(HomeError::NoDeviceInRoom)
                .context(format!("There is no '{}' in room '{}'", name, &self.name)),
        }
    }

    fn devices_state(&self) -> Result<String> {
        let mut room_report = String::new();
        for (name, device) in &self.devices {
            room_report.push_str(
                device
                    .print_state()
                    .context(format!("Can't  use device: '{}'", name))?
                    .as_str(),
            )
        }
        Ok(room_report)
    }
}

#[cfg(test)]
mod test {
    use super::{Home, Room};
    use crate::SmartSocket;

    fn create_device() -> SmartSocket {
        SmartSocket::new("test_device".into())
    }

    fn create_home() -> Home {
        Home::new("test_home".into())
    }

    fn create_room() -> Room {
        Room::new("test_room".into())
    }

    #[test]
    fn add_room() {
        let mut home = create_home();
        let room = create_room();
        assert_eq!(home.add_room(room), "New room added");
    }

    #[test]
    fn remove_existing_device() {
        let mut room = create_room();
        let device = Box::new(create_device());
        room.add_device(device);
        match room.remove_device("test_device") {
            Ok(_) => (),
            Err(e) => panic!("Error: {}", e),
        }
    }

    #[test]
    fn remove_nonexisting_device() {
        let mut room = create_room();
        let device = Box::new(create_device());
        room.add_device(device);
        if room.remove_device("non-existent_device").is_ok() {
            panic!("Can't remove non-existent device!!!")
        }
    }
}
