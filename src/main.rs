use smarthome::{Home, HomeError, Room};
use smarthome::{SmartSocket, SmartThermometer};

fn main() {
    let mut home = Home::new("home".to_string());

    let first_room_name = String::from("first");
    let second_room_name = String::from("second");

    let first_room = Room::new(first_room_name.clone());
    let second_room = Room::new(second_room_name.clone());

    println!("{}: {}", &first_room_name, home.add_room(first_room));
    println!("{}: {}", &second_room_name, home.add_room(second_room));

    let mut socket1 = SmartSocket::new("socket1".to_string());
    let socket2 = SmartSocket::new("socket2".to_string());
    let mut socket3 = SmartSocket::new("socket3".to_string());
    let thermometer1 = SmartThermometer::new("t1".to_string());

    socket1.switch_on();
    socket3.switch_on();

    match home.add_device(&first_room_name, Box::new(socket1)) {
        Ok(_) => println!("Socket1 has been added to room: '{}'", first_room_name),
        Err(err) => match err {
            HomeError::NoRoomInHoom(_) => println!("No '{}' in home", first_room_name),
            _ => println!("Unknown ERROR!"),
        },
    }
    match home.add_device(&first_room_name, Box::new(socket2)) {
        Ok(_) => println!("Socket2 has been added to room: '{}'", first_room_name),
        Err(err) => match err {
            HomeError::NoRoomInHoom(_) => println!("No '{}' in home", first_room_name),
            _ => println!("Unknown ERROR!"),
        },
    }
    match home.add_device(&first_room_name, Box::new(thermometer1)) {
        Ok(_) => println!("Thermometer1 has been added to room: '{}'", first_room_name),
        Err(err) => match err {
            HomeError::NoRoomInHoom(_) => println!("No '{}' in home", first_room_name),
            _ => println!("Unknown ERROR!"),
        },
    }
    match home.add_device(&second_room_name, Box::new(socket3)) {
        Ok(_) => println!("Socket3 has been added to room: '{}'", second_room_name),
        Err(err) => match err {
            HomeError::NoRoomInHoom(_) => println!("No '{}' in home", second_room_name),
            _ => println!("Unknown ERROR!"),
        },
    }
    home.print_all_info();
}
