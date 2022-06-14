use smarthome::{Home, Room};
use smarthome::{SmartSocket, SmartThermometer};

fn main() {
    //create new home instance
    let mut home = Home::new("home".to_string());

    let first_room_name = String::from("first");
    let second_room_name = String::from("second");

    let first_room = Room::new(first_room_name.clone());
    let second_room = Room::new(second_room_name.clone());

    //add new rooms in home and print log in console
    println!("{}: {}", &first_room_name, home.add_room(first_room));
    println!("{}: {}", &second_room_name, home.add_room(second_room));

    let mut socket1 = SmartSocket::new("socket1".to_string());
    let mut socket2 = SmartSocket::new("socket2".to_string());
    let mut socket3 = SmartSocket::new("socket3".to_string());
    let thermometer1 = SmartThermometer::new("t1".to_string());
    let termometer2 = SmartThermometer::new("t2".to_string());

    socket1.switch_on();
    socket2.switch_on();
    socket3.switch_off();

    //add devices in room
    match home.add_device(&first_room_name, Box::new(socket1)) {
        Ok(_) => println!("Socket1 has been added to room: '{}'", first_room_name),
        Err(err) => println!("Error: {}", err),
    }
    match home.add_device(&first_room_name, Box::new(socket2)) {
        Ok(_) => println!("Socket2 has been added to room: '{}'", first_room_name),
        Err(err) => println!("Error: {}", err),
    }
    match home.add_device(&first_room_name, Box::new(thermometer1)) {
        Ok(_) => println!("Thermometer1 has been added to room: '{}'", first_room_name),
        Err(err) => println!("Error: {}", err),
    }
    match home.add_device(&first_room_name, Box::new(termometer2)) {
        Ok(_) => println!("Thermometer2 has been added to room: '{}'", first_room_name),
        Err(err) => println!("Error: {}", err),
    }
    match home.add_device(&second_room_name, Box::new(socket3)) {
        Ok(_) => println!("Socket3 has been added to room: '{}'", second_room_name),
        Err(err) => println!("Error: {}", err),
    }
    match home.remove_room(&first_room_name) {
        Ok(_) => println!("Room: '{}' has been deleted", first_room_name),
        Err(err) => println!("Error: {}", err),
    }
    match home.remove_room("not_existed_room") {
        Ok(_) => println!("Room: 'not_existed_room' has been deleted"),
        Err(err) => println!("Error: {}", err),
    }

    //remove devices
    home.remove_device("not_existed_room", "socket2");
    home.remove_device(&second_room_name, "not_existed_socket");

    //print report about home (about existing devices)
    home.print_all_info();

    home.remove_device(&first_room_name, "socket2");
}
