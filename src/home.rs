use crate::devices::SmartDevice;
use std::collections::HashMap;

pub enum HomeError {
    NoRoomInHoom(String),
    NoDeviceInRoom(String),
    CantAddRoom,
}
pub struct Home {
    _name: String,
    rooms: HashMap<String, Room>,
}

impl Home {
    pub fn new(name: String) -> Self {
        let rooms = HashMap::new();
        Home { _name: name, rooms }
    }

    pub fn add_room(&mut self, room: Room) -> Result<(), HomeError> {
        match self.rooms.insert(room.name.clone(), room) {
            Some(_) => Ok(()),
            None => Err(HomeError::CantAddRoom),
        }
    }

    pub fn add_device(
        &mut self,
        room_name: &String,
        device: Box<dyn SmartDevice>,
    ) -> Result<(), HomeError> {
        let r = self.get_room(room_name)?;
        r.add_device(device);
        Ok(())
    }

    pub fn get_room(&mut self, name: &str) -> Result<&mut Room, HomeError> {
        for (room_name, room) in &mut self.rooms {
            if room_name == name {
                return Ok(room);
            }
        }
        Err(HomeError::NoRoomInHoom(format!(
            "There is no room: '{}' in home",
            name
        )))
    }

    pub fn remove_room(&mut self, name: &str) -> Result<Room, HomeError> {
        match self.rooms.remove(name) {
            Some(room) => return Ok(room),
            None => Err(HomeError::NoRoomInHoom(format!(
                "There is no room: '{}' in home",
                name
            ))),
        }
    }

    pub fn remove_device(
        &mut self,
        room_name: &String,
        device_name: &str,
    ) -> Option<Box<dyn SmartDevice>> {
        match self.get_room(room_name) {
            Ok(room) => return room.remove_device(device_name),
            Err(e) => {
                if let HomeError::NoRoomInHoom(msg) = e {
                    println!("{}", msg)
                }
            }
        }
        None
    }
    pub fn print_all_info(&self) {
        for (name, room) in &self.rooms {
            println!("Room {} info", name);
            room.devices_state();
        }
    }
}

pub struct Room {
    name: String,
    devices: HashMap<String, Box<dyn SmartDevice>>,
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

    fn _get_device(&mut self, name: &str) -> Option<&dyn SmartDevice> {
        for (device_name, device) in &self.devices {
            if device_name == name {
                return Some(device.as_ref());
            }
        }
        None
    }

    fn remove_device(&mut self, name: &str) -> Option<Box<dyn SmartDevice>> {
        match self.devices.remove(name) {
            Some(device) => return Some(device),
            None => println!(">> There is no '{}' in room: '{}'", name, self.name),
        }
        None
    }

    fn devices_state(&self) {
        for device in &self.devices {
            device.1.print_state();
        }
    }
}

#[cfg(test)]
mod test {}
