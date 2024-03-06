use crate::pac;

pub trait Gpio {
    fn enable(&self, pin: u32);
    fn set_output(&self, pin: u32);
    fn set_input(&self, pin: u32) -> Result<(), GpioError>;
    fn set_high(&self, pin: u32);
    fn set_low(&self, pin: u32);
    fn is_high(&self, pin: u32) -> bool;
    fn is_low(&self, pin: u32) -> bool;
}



pub struct Port{
    
}
pub enum GpioError{
    InavlidPin
}

impl Gpio for pac::GPIO0 {
    fn enable(&self, pin: u32) {
        self.en0_set.modify(|_, w| unsafe { w.bits(pin) });
    }

    fn set_output(&self, pin: u32) {
        self.outen_set.modify(|_, w| unsafe { w.bits(pin) });
    }

    fn set_input(&self, pin: u32) -> Result<(), GpioError>{

        if pin > 31
        {
            return Err(GpioError::InavlidPin);
        }

        let mask = 1 << pin;
        self.outen_clr.write(|w| unsafe { w.bits(mask) });
        self.inten_set.write(|w| unsafe {
            w.bits(mask)
        });

        return Ok(());
    }

    fn set_high(&self, pin: u32) {
        self.out_set.write(|w| unsafe { w.bits(pin) });
    }

    fn set_low(&self, pin: u32) {
        self.out_clr.write(|w| unsafe { w.bits(pin) });
    }

    fn is_high(&self, pin: u32) -> bool {
        self.out.read().bits() & pin != 0
    }

    fn is_low(&self, pin: u32) -> bool {
        !self.is_high(pin)
    }
}
