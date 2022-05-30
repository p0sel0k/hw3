pub(crate) trait SmartDevice {
    fn name(&self) -> &str;
    fn print_state(&self);
}

pub(crate) struct SmartSocket {
    name: String,
    is_switched_on: bool,
    power: i32,
}

impl SmartSocket {
    pub(crate) fn new(name: String) -> Self {
        SmartSocket {
            name,
            is_switched_on: false,
            power: 220,
        }
    }

    pub(crate) fn switch_on(&mut self) {
        self.is_switched_on = true;
    }

    pub(crate) fn switch_off(&mut self) {
        self.is_switched_on = false;
    }

    pub(crate) fn power(&self) -> Option<i32> {
        if self.is_switched_on {
            Some(self.power)
        } else {
            None
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
            Some(p) => println!(">>>> Socket power is '{}'", p),
            None => println!(">>>> Socket is off"),
        }
    }
}

pub(crate) struct SmartThermometer {
    name: String,
    temperature: i32,
}

impl SmartThermometer {
    pub(crate) fn new(name: String) -> Self {
        SmartThermometer {
            name,
            temperature: 25,
        }
    }

    pub(crate) fn get_temperature(&self) -> i32 {
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
