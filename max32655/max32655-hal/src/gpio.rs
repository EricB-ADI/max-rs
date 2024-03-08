use crate::pac;

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

#[derive(Debug, PartialEq, Eq)]
pub enum GpioError {
    InvalidPin,
    InvalidState,
}

pub trait GpioPort {
    fn enable_output(&mut self, en: bool, pin: u8) -> Result<(), GpioError>;
    fn enable_input(&mut self, en: bool, pin: u8) -> Result<(), GpioError>;

    fn set_high(&mut self, pin: u8) -> Result<(), GpioError>;
    fn set_low(&mut self, pin: u8) -> Result<(), GpioError>;
    fn read(&mut self, pin: u8) -> Result<bool, GpioError>;
}

impl GpioPort for pac::GPIO0 {
    fn enable_output(&mut self, en: bool, pin: u8) -> Result<(), GpioError> {
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

        self.padctrl0
            .modify(|r, w| unsafe { w.bits(r.bits() & !(1 << pin)) });

        self.padctrl1
            .modify(|r, w| unsafe { w.bits(r.bits() & !(1 << pin)) });

        Ok(())
    }

    fn enable_input(&mut self, en: bool, pin: u8) -> Result<(), GpioError> {
        if pin > 31 {
            return Err(GpioError::InvalidPin);
        }

        let mask = 1 << pin;

        if en {
            self.en0_set.write(|w| unsafe { w.bits(mask) });
            self.inen.modify(|r, w| unsafe { w.bits(r.bits() | mask) });
        } else {
            self.en0_clr.write(|w| unsafe { w.bits(mask) });
            self.inen.modify(|r, w| unsafe { w.bits(r.bits() & !mask) });
        }

        self.padctrl1
            .modify(|r, w| unsafe { w.bits(r.bits() & !mask) });

        self.padctrl0
            .modify(|r, w| unsafe { w.bits(r.bits() | mask) });

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

        let input = (self.in_.read().bits() >> pin) & 1;

        Ok(input != 0)
    }
}

impl GpioPort for pac::GPIO1 {
    fn enable_output(&mut self, en: bool, pin: u8) -> Result<(), GpioError> {
        if pin > 31 {
            return Err(GpioError::InvalidPin);
        }

        if en {
            self.en1_set.write(|w| unsafe { w.bits(1 << pin) });
            self.outen_set.write(|w| unsafe { w.bits(1 << pin) });
        } else {
            self.en1_clr.write(|w| unsafe { w.bits(1 << pin) });
            self.outen_clr.write(|w| unsafe { w.bits(1 << pin) });
        }

        self.padctrl0
            .modify(|r, w| unsafe { w.bits(r.bits() & !(1 << pin)) });

        self.padctrl1
            .modify(|r, w| unsafe { w.bits(r.bits() & !(1 << pin)) });

        Ok(())
    }

    fn enable_input(&mut self, en: bool, pin: u8) -> Result<(), GpioError> {
        if pin > 31 {
            return Err(GpioError::InvalidPin);
        }

        let mask = 1 << pin;

        if en {
            self.en1_set.write(|w| unsafe { w.bits(mask) });
            self.inen.modify(|r, w| unsafe { w.bits(r.bits() | mask) });
        } else {
            self.en1_clr.write(|w| unsafe { w.bits(mask) });
            self.inen.modify(|r, w| unsafe { w.bits(r.bits() & !mask) });
        }

        self.padctrl1
            .modify(|r, w| unsafe { w.bits(r.bits() & !mask) });

        self.padctrl0
            .modify(|r, w| unsafe { w.bits(r.bits() | mask) });

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

        let input = (self.in_.read().bits() >> pin) & 1;

        Ok(input != 0)
    }
}

impl GpioPort for pac::GPIO2 {
    fn enable_output(&mut self, en: bool, pin: u8) -> Result<(), GpioError> {
        if pin > 31 {
            return Err(GpioError::InvalidPin);
        }

        if en {
            self.en2_set.write(|w| unsafe { w.bits(1 << pin) });
            self.outen_set.write(|w| unsafe { w.bits(1 << pin) });
        } else {
            self.en2_clr.write(|w| unsafe { w.bits(1 << pin) });
            self.outen_clr.write(|w| unsafe { w.bits(1 << pin) });
        }

        self.padctrl0
            .modify(|r, w| unsafe { w.bits(r.bits() & !(1 << pin)) });

        self.padctrl1
            .modify(|r, w| unsafe { w.bits(r.bits() & !(1 << pin)) });

        Ok(())
    }

    fn enable_input(&mut self, en: bool, pin: u8) -> Result<(), GpioError> {
        if pin > 31 {
            return Err(GpioError::InvalidPin);
        }

        let mask = 1 << pin;

        if en {
            self.en2_set.write(|w| unsafe { w.bits(mask) });
            self.inen.modify(|r, w| unsafe { w.bits(r.bits() | mask) });
        } else {
            self.en2_clr.write(|w| unsafe { w.bits(mask) });
            self.inen.modify(|r, w| unsafe { w.bits(r.bits() & !mask) });
        }

        self.padctrl1
            .modify(|r, w| unsafe { w.bits(r.bits() & !mask) });

        self.padctrl0
            .modify(|r, w| unsafe { w.bits(r.bits() | mask) });

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

        let input = (self.in_.read().bits() >> pin) & 1;

        Ok(input != 0)
    }
}
