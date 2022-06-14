pub enum DeviceError {
    DeviceIsTurnedOff(&'static str),
    UnusedError,
}

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
            Err(e) => {
                if let DeviceError::DeviceIsTurnedOff(msg) = e {
                    println!(">>>> {}", msg)
                }
            }
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
mod test {}
