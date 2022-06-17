use thiserror::Error;

#[derive(Error, Debug)]
pub enum DeviceError {
    #[error("Device is turned off")]
    DeviceIsTurnedOff,

    #[error("Unused Error")]
    UnusedError,
}

pub trait SmartDevice {
    fn name(&self) -> &str;
    fn print_state(&self) -> Result<(), DeviceError>;
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
            Err(DeviceError::DeviceIsTurnedOff)
        }
    }
}

impl SmartDevice for SmartSocket {
    fn name(&self) -> &str {
        &self.name
    }

    fn print_state(&self) -> Result<(), DeviceError> {
        println!(">> Socket name is: {}", self.name());
        match self.power() {
            Ok(p) => {
                println!(">>>> Socket power is '{}'", p);
                Ok(())
            }
            Err(e) => Err(e),
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

    fn print_state(&self) -> Result<(), DeviceError> {
        println!(">> Thermometer name is: {}", self.name());
        println!(">>>> Themperature is: {}", self.get_temperature());
        Ok(())
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
