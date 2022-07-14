pub use thiserror::Error;

#[derive(Error, Debug)]
pub enum DeviceError {
    #[error("Device is turned off")]
    DeviceIsTurnedOff,

    #[error("Return state error")]
    ReturnState,

    #[error("Can't switch on/off this device")]
    SwitchOnOffError,
}

pub trait SmartDevice {
    fn name(&self) -> &str;
    fn return_state(&self) -> Result<String, DeviceError>;
    fn print_state(&self) -> Result<String, DeviceError>;
    fn switch_device(&mut self) -> Result<String, DeviceError>;
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

    fn print_state(&self) -> Result<String, DeviceError> {
        match self.power() {
            Ok(p) => Ok(format!(
                ">> Socket name is: {}\n>>>> Socket power is '{}'\n",
                self.name(),
                p
            )),
            Err(e) => Err(e),
        }
    }

    fn return_state(&self) -> Result<String, DeviceError> {
        let str = format!("power: {}", self.power()?);
        Ok(str)
    }

    fn switch_device(&mut self) -> Result<String, DeviceError> {
        let result = if self.is_switched_on {
            self.switch_off();
            "On"
        } else {
            self.switch_on();
            "Off"
        };
        Ok(result.into())
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

    fn print_state(&self) -> Result<String, DeviceError> {
        Ok(format!(
            ">> Thermometer name is: {}\n>>>> Themperature is: {}\n",
            self.name,
            self.get_temperature(),
        ))
    }

    fn return_state(&self) -> Result<String, DeviceError> {
        let str = format!("temperature: {}", self.get_temperature());
        Ok(str)
    }

    fn switch_device(&mut self) -> Result<String, DeviceError> {
        Err(DeviceError::SwitchOnOffError)
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
