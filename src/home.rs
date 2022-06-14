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

    pub fn add_room(&mut self, room: Room) -> String {
        match self.rooms.insert(room.name.clone(), room) {
            Some(old_room) => format!("Existed room was updated: {:?}", old_room),
            None => "New room added".into(),
        }
    }

    pub fn add_device(
        &mut self,
        room_name: &str,
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
            Some(room) => Ok(room),
            None => Err(HomeError::NoRoomInHoom(format!(
                "There is no room: '{}' in home",
                name
            ))),
        }
    }

    pub fn remove_device(
        &mut self,
        room_name: &str,
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
        println!("\n---------Home Report--------");
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
            Some(_) => (),
            None => panic!("Can't remove device!!!"),
        }
    }
    #[test]
    fn remove_nonexisting_device() {
        let mut room = create_room();
        let device = Box::new(create_device());
        room.add_device(device);
        match room.remove_device("non-existent_device") {
            Some(_) => panic!("Can't remove non-existent device!!!"),
            None => (),
        }
    }
}
