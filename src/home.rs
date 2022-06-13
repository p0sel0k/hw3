use crate::devices::SmartDevice;
use std::collections::HashMap;
pub struct Home {
    _name: String,
    rooms: HashMap<String, Room>,
}

impl Home {
    pub fn new(name: String) -> Self {
        let rooms = HashMap::new();
        Home { _name: name, rooms }
    }

    pub fn add_room(&mut self, room: Room) {
        self.rooms.insert(room.name.clone(), room);
        println!("The room has been succesfully added")
    }

    pub fn add_device(&mut self, room_name: &String, device: Box<dyn SmartDevice>) {
        match self.get_room(room_name) {
            Some(room) => {
                room.add_device(device);
                println!(
                    "The device has been succesfully added in room: '{}'",
                    room_name
                )
            }
            None => println!("There is no room: '{}' in home", room_name),
        }
    }

    pub fn get_room(&mut self, name: &str) -> Option<&mut Room> {
        for (room_name, room) in &mut self.rooms {
            if room_name == name {
                return Some(room);
            }
        }
        None
    }

    pub fn remove_room(&mut self, name: &str) -> Option<Room> {
        match self.rooms.remove(name) {
            Some(room) => return Some(room),
            None => println!("There is no room: '{}' in home", name),
        }
        None
    }

    pub fn remove_device(
        &mut self,
        room_name: &String,
        device_name: &str,
    ) -> Option<Box<dyn SmartDevice>> {
        match self.get_room(room_name) {
            Some(room) => return room.remove_device(device_name),
            None => println!("There is no room: '{}' in home", room_name),
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
