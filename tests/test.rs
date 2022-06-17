use smarthome::{Home, Room, SmartDevice};
use smarthome::{SmartSocket, SmartThermometer};

fn _init_device(device_type: &str, name: String) -> Box<dyn SmartDevice> {
    if device_type == "socket" {
        Box::new(SmartSocket::new(name))
    } else {
        Box::new(SmartThermometer::new(name))
    }
}

fn init_room(name: String) -> Room {
    Room::new(name)
}

fn init_home() -> Home {
    Home::new("home".to_string())
}

#[test]
fn add_room() {
    let mut home = init_home();
    let room = init_room("room".into());
    assert_eq!(home.add_room(room), "New room added");
}
