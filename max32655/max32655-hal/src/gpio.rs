use crate::pac;
use core::marker::PhantomData;
pub use embedded_hal::digital::PinState;

pub struct Input;
pub struct Output;

// use embedded_hal::digital;
#[derive(Debug, PartialEq, Eq)]
pub enum GpioError {
    InvalidPin,
    InvalidState,
}
pub enum Port {
    P0,
    P1,
    P2,
}

#[derive(Debug, PartialEq)]
pub enum Mode {
    Input,
    Output,
    ALT1,
    ALT2,
    ALT3,
    ALT4,
}
#[derive(Debug, PartialEq)]

pub enum Vssel {
    VDDIO,  //1.8v
    VDDIOH, //3.3v
}
#[derive(Debug, PartialEq)]

pub enum DriveStrength {
    DS0,
    DS1,
    DS2,
    DS3,
}
#[derive(Debug, PartialEq)]
pub enum Pad {
    NONE,
    PullUp,
    PullDown,
    WeakPullUp,
    WeakPullDown,
}
#[derive(Debug, PartialEq)]

pub struct PinConfig {
    mode: Mode,
    vssel: Vssel,
    drive_strength: DriveStrength,
    pad: Pad,
}
pub struct P0;
pub struct P1;
pub struct P2;
pub struct P3;
pub struct P4;
pub struct P5;
pub struct P6;
pub struct P7;
pub struct P8;
pub struct P9;
pub struct P10;
pub struct P11;
pub struct P12;
pub struct P13;
pub struct P14;
pub struct P15;
pub struct P16;
pub struct P17;
pub struct P18;
pub struct P19;
pub struct P20;
pub struct P21;
pub struct P22;
pub struct P23;
pub struct P24;
pub struct P25;
pub struct P26;
pub struct P27;
pub struct P28;
pub struct P29;
pub struct P30;
pub struct P31;
pub struct Pins {
    pub p0: P0,
    pub p1: P1,
    pub p2: P2,
    pub p3: P3,
    pub p4: P4,
    pub p5: P5,
    pub p6: P6,
    pub p7: P7,
    pub p8: P8,
    pub p9: P9,
    pub p10: P10,
    pub p11: P11,
    pub p12: P12,
    pub p13: P13,
    pub p14: P14,
    pub p15: P15,
    pub p16: P16,
    pub p17: P17,
    pub p18: P18,
    pub p19: P19,
    pub p20: P20,
    pub p21: P21,
    pub p22: P22,
    pub p23: P23,
    pub p24: P24,
    pub p25: P25,
    pub p26: P26,
    pub p27: P27,
    pub p28: P28,
    pub p29: P29,
    pub p30: P30,
    pub p31: P31,
}

// pub struct  Pin{
//     port: Port,
//     pin:u8
// }
pub struct Pin<const P: char, const N: u8, MODE = Input> {
    _mode: PhantomData<MODE>,
}
pub trait GpioExt {}

pub trait GpioPort {
    fn split(&self) -> Pins {
        Pins {
            p0: P0,
            p1: P1,
            p2: P2,
            p3: P3,
            p4: P4,
            p5: P5,
            p6: P6,
            p7: P7,
            p8: P8,
            p9: P9,
            p10: P10,
            p11: P11,
            p12: P12,
            p13: P13,
            p14: P14,
            p15: P15,
            p16: P16,
            p17: P17,
            p18: P18,
            p19: P19,
            p20: P20,
            p21: P21,
            p22: P22,
            p23: P23,
            p24: P24,
            p25: P25,
            p26: P26,
            p27: P27,
            p28: P28,
            p29: P29,
            p30: P30,
            p31: P31,
        }
    }

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

// mod sealed {
//     pub trait Sealed {}
// }

// pub trait PinState: sealed::Sealed {}
// pub trait OutputState: sealed::Sealed {}
// pub trait InputState: sealed::Sealed {
//     // ...
// }

// pub struct Output<S: OutputState> {
//     _p: PhantomData<S>,
// }

// impl<S: OutputState> PinState for Output<S> {}
// impl<S: OutputState> sealed::Sealed for Output<S> {}

// pub struct PushPull;
// pub struct OpenDrain;

// impl OutputState for PushPull {}
// impl OutputState for OpenDrain {}
// impl sealed::Sealed for PushPull {}
// impl sealed::Sealed for OpenDrain {}

// pub struct Input<S: InputState> {
//     _p: PhantomData<S>,
// }

// impl<S: InputState> PinState for Input<S> {}
// impl<S: InputState> sealed::Sealed for Input<S> {}

// pub struct Floating;
// pub struct PullUp;
// pub struct PullDown;

// impl InputState for Floating {}
// impl InputState for PullUp {}
// impl InputState for PullDown {}
// impl sealed::Sealed for Floating {}
// impl sealed::Sealed for PullUp {}
// impl sealed::Sealed for PullDown {}

// pub struct PA1<S: PinState> {
//     _p: PhantomData<S>,
// }

// impl<S: PinState> PA1<S> {
//     pub fn into_input<N: InputState>(self, input: N) -> PA1<Input<N>> {

//         todo!()
//     }

//     pub fn into_output<N: OutputState>(self, output: N) -> PA1<Output<N>> {
//         todo!()
//     }

//     pub fn with_input_state<N: InputState, R>(
//         &mut self,
//         input: N,
//         f: impl FnOnce(&mut PA1<N>) -> R,
//     ) -> R {
//         todo!()
//     }

//     pub fn with_output_state<N: OutputState, R>(
//         &mut self,
//         output: N,
//         f: impl FnOnce(&mut PA1<N>) -> R,
//     ) -> R {
//         todo!()
//     }
// }

impl<const P: char, const N: u8, MODE> Pin<P, N, MODE> {
    /// Set the output of the pin regardless of its mode.
    /// Primarily used to set the output value of the pin
    /// before changing its mode to an output to avoid
    /// a short spike of an incorrect value
    #[inline(always)]
    fn _set_state(&mut self, state: PinState) {
        match state {
            PinState::High => self._set_high(),
            PinState::Low => self._set_low(),
        }
    }
    #[inline(always)]
    fn _set_high(&mut self) {
        // NOTE(unsafe) atomic write to a stateless register
        unsafe { (*gpiox::<P>()).out_set.write(|w| w.bits(1 << N)) }
    }
    #[inline(always)]
    fn _set_low(&mut self) {
        // NOTE(unsafe) atomic write to a stateless register
        unsafe { (*gpiox::<P>()).out_clr.write(|w| w.bits(1 << N)) }
    }
    #[inline(always)]
    fn _is_set_low(&self) -> bool {
        // NOTE(unsafe) atomic read with no side effects
        unsafe { (*gpiox::<P>()).in_.read().bits() & (1 << N) == 0 }
    }
    #[inline(always)]
    fn _is_low(&self) -> bool {
        // NOTE(unsafe) atomic read with no side effects
        unsafe { (*gpiox::<P>()).in_.read().bits() & (1 << N) == 0 }
    }
}

// impl<const P: char, const N: u8, MODE> Pin<P, N, Output<MODE>> {
//     /// Drives the pin high
//     #[inline(always)]
//     pub fn set_high(&mut self) {
//         self._set_high()
//     }

//     /// Drives the pin low
//     #[inline(always)]
//     pub fn set_low(&mut self) {
//         self._set_low()
//     }

//     /// Is the pin in drive high or low mode?
//     #[inline(always)]
//     pub fn get_state(&self) -> PinState {
//         if self.is_set_low() {
//             PinState::Low
//         } else {
//             PinState::High
//         }
//     }

//     /// Drives the pin high or low depending on the provided value
//     #[inline(always)]
//     pub fn set_state(&mut self, state: PinState) {
//         match state {
//             PinState::Low => self.set_low(),
//             PinState::High => self.set_high(),
//         }
//     }

//     /// Is the pin in drive high mode?
//     #[inline(always)]
//     pub fn is_set_high(&self) -> bool {
//         !self.is_set_low()
//     }

//     /// Is the pin in drive low mode?
//     #[inline(always)]
//     pub fn is_set_low(&self) -> bool {
//         self._is_set_low()
//     }

//     /// Toggle pin output
//     #[inline(always)]
//     pub fn toggle(&mut self) {
//         if self.is_set_low() {
//             self.set_high()
//         } else {
//             self.set_low()
//         }
//     }
// }
#[allow(dead_code)]
const fn gpiox<const P: char>() -> *const crate::pac::gpio0::RegisterBlock {
    match P {
        '0' => crate::pac::GPIO0::ptr(),
        '1' => crate::pac::GPIO1::ptr() as _,
        '2' => crate::pac::GPIO2::ptr() as _,
        _ => panic!("Unknown GPIO port"),
    }
}
