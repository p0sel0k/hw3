use smarthome::{Home, HomeError, Room, SmartDevice};
use smarthome::{SmartSocket, SmartThermometer};

fn init_device(device_type: &str, name: String) -> Box<dyn SmartDevice> {
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

#[test]
fn add_device() {
    let mut home = init_home();
    let room = init_room("room".into());
    let socket = init_device("socket", "socket1".into());
    home.add_room(room);
    if let Err(HomeError::NoRoomInHoom(msg)) = home.add_device("room", socket) {
        panic!(
            "Can't add device in non-existed room\n ErrorMessage: {}",
            msg
        );
    }
}

#[test]
#[should_panic]
fn add_device_in_nonexisted_room() {
    let mut home = init_home();
    let thermo = init_device("thermo", "thermo1".into());
    if let Err(HomeError::NoRoomInHoom(msg)) = home.add_device("room", thermo) {
        panic!(
            "Can't add device in non-existed room\n ErrorMessage: {}",
            msg
        );
    }
}
