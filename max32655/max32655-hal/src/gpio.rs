use crate::pac;
use crate::Sys;
// use embedded_hal::digital;

pub enum PortNum {
    P0,
    P1,
    P2,
}

#[derive(PartialEq)]
pub enum Mode {
    AF1,
    AF2,
    Input,
    Output,
}

pub enum GpioError {
    InvalidPin,
    InvalidState,
}


// pub struct Gpio<T>{
//     port : T
// }

// impl <T> Gpio<T> {
//     pub fn new(port : PortNum)-> {
//         Gpio{port}
//     }
// }


pub trait GpioPort {
    fn enable(&mut self, en: bool, pin: u8) -> Result<(), GpioError>;
    fn set_high(&mut self, pin: u8) -> Result<(), GpioError>;
    fn set_low(&mut self, pin: u8) -> Result<(), GpioError>;
    fn read(&mut self, pin: u8) -> Result<bool, GpioError>;
}

impl GpioPort for pac::GPIO0 {
    fn enable(&mut self, en: bool, pin: u8) -> Result<(), GpioError> {
        if pin > 31 {
            return Err(GpioError::InvalidPin);
        }

        if en {
            self.en0_set.write(|w| unsafe { w.bits(1 << pin) });
            self.outen_set.write(|w| unsafe { w.bits(1 << pin) });

        } else {
            self.en0_clr.write(|w| unsafe { w.bits(1 << pin) });
            self.outen_clr.write(|w| unsafe { w.bits(1 << pin) });
        }


        self
        .padctrl0
        .modify(|r, w| unsafe { w.bits(r.bits() | (1 << pin)) });
        
        self
        .padctrl1
        .modify(|r, w| unsafe { w.bits(r.bits() | (1 << pin)) });

        Ok(())
    }
    fn set_high(&mut self, pin: u8) -> Result<(), GpioError> {
        if pin > 31 {
            return Err(GpioError::InvalidPin);
        }

        self.out_set.write(|w| unsafe { w.bits(1 << pin) });

        Ok(())
    }
    fn set_low(&mut self, pin: u8) -> Result<(), GpioError> {
        if pin > 31 {
            return Err(GpioError::InvalidPin);
        }

        self.out_clr.write(|w| unsafe { w.bits(1 << pin) });

        Ok(())
    }

    fn read(&mut self, pin: u8) -> Result<bool, GpioError> {
        if pin > 31 {
            return Err(GpioError::InvalidPin);
        }

        Ok(self.in_.read().bits() & (1 << pin) != 0)
    }
}



// pub struct GPIO<T> {
//     port: T,
// }

// impl<T> GPIO<T>
// where
//     T: GpioPort,
// {
//     pub fn new(port : T) -> Self {
//         GPIO { port }
//     }
// }
