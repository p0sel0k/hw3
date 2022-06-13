use smarthome::{Home, Room};
use smarthome::{SmartSocket, SmartThermometer};

fn main() {
    let mut home = Home::new("home".to_string());

    let first_room_name = String::from("first");
    let second_room_name = String::from("second");

    let first_room = Room::new(first_room_name.clone());
    let second_room = Room::new(second_room_name.clone());

    home.add_room(first_room);
    home.add_room(second_room);

    let mut socket1 = SmartSocket::new("socket1".to_string());
    let mut socket2 = SmartSocket::new("socket2".to_string());
    let mut socket3 = SmartSocket::new("socket3".to_string());
    let thermometer1 = SmartThermometer::new("t1".to_string());
    let termometer2 = SmartThermometer::new("t2".to_string());

    socket1.switch_on();
    socket2.switch_on();
    socket3.switch_off();

    home.add_device(&first_room_name, Box::new(socket1));
    home.add_device(&first_room_name, Box::new(socket2));
    home.add_device(&first_room_name, Box::new(thermometer1));
    home.add_device(&first_room_name, Box::new(termometer2));
    home.add_device(&second_room_name, Box::new(socket3));

    home.remove_room(&first_room_name);
    home.remove_room("not_existed_room");
    home.remove_device(&"not_existed_room".to_string(), "socket2");
    home.remove_device(&second_room_name, "not_existed_socket");

    home.print_all_info();

    home.remove_device(&first_room_name, "socket2");
}
