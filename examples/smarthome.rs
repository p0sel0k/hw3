use std::fs::File;

use anyhow::Result;
use smarthome::{Home, Room};
use smarthome::{SmartSocket, SmartThermometer};

fn main() -> Result<()> {
    //create new home instance
    let mut home = Home::new("home".into());

    let first_room_name = String::from("first");
    let second_room_name = String::from("second");
    println!("{}", home.input_new_room()?);

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
    socket3.switch_on();

    home.input_new_device()?;

    //add devices in room
    home.add_device(&first_room_name, Box::new(socket1))?;
    home.add_device(&first_room_name, Box::new(socket2))?;
    home.add_device(&first_room_name, Box::new(thermometer1))?;
    home.add_device(&first_room_name, Box::new(termometer2))?;
    home.add_device(&second_room_name, Box::new(socket3))?;

    // let _removed_room = home.remove_room(&first_room_name)?;
    // let _nonexisted_room = home.remove_room("not_existed_room")?;

    //remove devices
    // let _socket_in_nonexisted_room = home.remove_device("not_existed_room", "socket2")?;
    // let _nonexisted_socket = home.remove_device(&second_room_name, "not_existed_socket")?;

    //print report about home (about existing devices)
    let file = File::create("./home_report.txt")?;
    home.write_report(file)?;

    Ok(())
}
