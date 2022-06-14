use std::error::Error;
use std::fmt::Display;

#[derive(Debug)]
pub enum DeviceError {
    DeviceIsTurnedOff(&'static str),
    UnusedError,
}

impl Display for DeviceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DeviceError::DeviceIsTurnedOff(msg) => {
                write!(f, "Device Is Turned Off \n>>>> Cause: {}", msg)
            }
            DeviceError::UnusedError => write!(f, "UnusedError"),
        }
    }
}

impl Error for DeviceError {}

pub trait SmartDevice {
    fn name(&self) -> &str;
    fn print_state(&self);
}

pub struct SmartSocket {
    name: String,
    is_switched_on: bool,
    power: i32,
}

impl SmartSocket {
    pub fn new(name: String) -> Self {
        SmartSocket {
            name,
            is_switched_on: false,
            power: 220,
        }
    }

    pub fn switch_on(&mut self) {
        self.is_switched_on = true;
    }

    pub fn switch_off(&mut self) {
        self.is_switched_on = false;
    }

    pub fn power(&self) -> Result<i32, DeviceError> {
        if self.is_switched_on {
            Ok(self.power)
        } else {
            Err(DeviceError::DeviceIsTurnedOff("Socket is off"))
        }
    }
}

impl SmartDevice for SmartSocket {
    fn name(&self) -> &str {
        &self.name
    }

    fn print_state(&self) {
        println!(">> Socket name is: {}", self.name());
        match self.power() {
            Ok(p) => println!(">>>> Socket power is '{}'", p),
            Err(e) => println!(">>>> Error: {}", e),
        }
    }
}

pub struct SmartThermometer {
    name: String,
    temperature: i32,
}

impl SmartThermometer {
    pub fn new(name: String) -> Self {
        SmartThermometer {
            name,
            temperature: 25,
        }
    }

    pub fn get_temperature(&self) -> i32 {
        self.temperature
    }
}

impl SmartDevice for SmartThermometer {
    fn name(&self) -> &str {
        &self.name
    }

    fn print_state(&self) {
        println!(">> Thermometer name is: {}", self.name());
        println!(">>>> Themperature is: {}", self.get_temperature());
    }
}

#[cfg(test)]
mod test {
    use crate::SmartSocket;

    fn init_smart_socket() -> SmartSocket {
        SmartSocket::new("test_device".into())
    }

    #[test]
    fn get_socket_power() {
        let mut socket = init_smart_socket();
        socket.switch_on();
        match socket.power() {
            Ok(power) => assert_eq!(power, 220),
            Err(_) => panic!("Socket is turned on"),
        }
    }

    #[test]
    #[should_panic]
    fn get_turned_off_socket_power() {
        let socket = init_smart_socket();
        match socket.power() {
            Ok(power) => assert_eq!(power, 220),
            Err(_) => panic!("Socket is turned on"),
        }
    }
}
