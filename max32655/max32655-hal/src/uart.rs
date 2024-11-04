use crate::pac;
use crate::MxcError;
use crate::Sys;

use crate::Sys::PeriphClockFreq;

#[derive(Clone, Copy)]
pub enum StopBits {
    One,
    Two,
}

pub enum Parity {
    Disable,
    Even0,
    Even1,
    Odd0,
    Odd1,
}

#[derive(PartialEq)]
pub enum BaudClock {
    External,
    PeriphClock,
    IBRO,
    ERTCO,
}

pub const FIFO_DEPTH: u8 = 6;

pub trait Uart {
    fn init(&self, baud: u32, clock: BaudClock) -> Result<(), MxcError::Error>;
    fn shutdown(&self);

    fn set_rx_threshold(&self, thresh: u8) -> Result<(), MxcError::Error>;
    fn get_rx_threshold(&self) -> u8;
    fn set_datasize(&self, datasize: u8) -> Result<(), MxcError::Error>;
    fn set_parity(&self, parity: Parity);
    fn set_stop_bits(&self, stop_bits: StopBits);
    fn get_frequency(&self) -> u32;
    fn set_frequency(&self, baud: u32, clock: BaudClock) -> Result<u32, MxcError::Error>;
    fn get_status(&self) -> u32;
    fn get_baud_clock(&self) -> Result<BaudClock, MxcError::Error>;

    fn write_byte(&self, data: u8) -> Result<(), MxcError::Error>;
    fn read_byte(&self) -> Result<u8, MxcError::Error>;
}

impl Uart for pac::UART {
    fn init(&self, baud: u32, clock: BaudClock) -> Result<(), MxcError::Error> {
        match clock {
            BaudClock::ERTCO => Sys::enable_core_clock(Sys::CoreClockSource::ERTCO)?,
            BaudClock::IBRO => Sys::enable_core_clock(Sys::CoreClockSource::IBRO)?,
            BaudClock::PeriphClock => Sys::periph_clock_enable(Sys::PeriphClock::UART0),
            _ => {}
        };

        // let mut gpio = unsafe {
        //     pac::Peripherals::steal().GPIO0
        // };
        // Need to update GPIO to configure I/O for UART

        self.set_rx_threshold(1)?;
        self.set_datasize(8)?;
        self.set_parity(Parity::Disable);
        self.set_stop_bits(StopBits::One);
        self.set_frequency(baud, clock)?;

        Ok(())
    }
    fn shutdown(&self) {}
    fn write_byte(&self, data: u8) -> Result<(), MxcError::Error> {
        if self.status.read().tx_full().bit_is_set() {
            return Err(MxcError::Error::Overflow);
        }

        self.fifo.write(|w| unsafe { w.data().bits(data) });

        Ok(())
    }
    fn read_byte(&self) -> Result<u8, MxcError::Error> {
        if self.status.read().rx_em().bit_is_set() {
            return Err(MxcError::Error::Underflow);
        }

        Ok(self.fifo.read().data().bits())
    }

    fn set_rx_threshold(&self, thresh: u8) -> Result<(), MxcError::Error> {
        if !(1..=FIFO_DEPTH).contains(&thresh) {
            return Err(MxcError::Error::BadParam);
        }

        self.ctrl.write(|w| unsafe { w.rx_thd_val().bits(thresh) });

        Ok(())
    }
    fn get_rx_threshold(&self) -> u8 {
        self.ctrl.read().rx_thd_val().bits()
    }
    fn set_datasize(&self, datasize: u8) -> Result<(), MxcError::Error> {
        if datasize < 5 || datasize > 8 {
            return Err(MxcError::Error::BadParam);
        }

        self.ctrl.write(|w| w.char_size().bits(datasize - 5));

        Ok(())
    }
    fn set_parity(&self, parity: Parity) {
        self.ctrl.write(|w| match parity {
            Parity::Disable => w.par_en().clear_bit(),
            Parity::Even0 => w
                .par_en()
                .clear_bit()
                .par_eo()
                .clear_bit()
                .par_md()
                .set_bit(),

            Parity::Even1 => w
                .par_en()
                .set_bit()
                .par_eo()
                .clear_bit()
                .par_md()
                .clear_bit(),
            Parity::Odd0 => w.par_en().set_bit().par_eo().set_bit().par_md().clear_bit(),
            Parity::Odd1 => w.par_en().set_bit().par_eo().set_bit().par_md().set_bit(),
        });
    }
    fn set_stop_bits(&self, stop_bits: StopBits) {
        self.ctrl.write(|w| match stop_bits {
            StopBits::One => w.stopbits().clear_bit(),
            StopBits::Two => w.stopbits().clear_bit(),
        })
    }
    fn get_baud_clock(&self) -> Result<BaudClock, MxcError::Error> {
        let src = if self.ctrl.read().bclksrc().is_external_clock() {
            BaudClock::External
        } else if self.ctrl.read().bclksrc().is_peripheral_clock() {
            BaudClock::PeriphClock
        } else if self.ctrl.read().bclksrc().is_clk2() {
            BaudClock::IBRO
        } else {
            return Err(MxcError::Error::Invalid);
        };

        Ok(src)
    }

    fn get_frequency(&self) -> u32 {
        let pclk = match self.get_baud_clock().unwrap() {
            BaudClock::PeriphClock => Sys::get_clock_source()
                .get_clock_freq()
                .into_periph_clock_freq(),
            BaudClock::IBRO => Sys::get_core_clock_freq(Sys::CoreClockSource::IBRO),
            BaudClock::ERTCO => Sys::get_core_clock_freq(Sys::CoreClockSource::IBRO),
            BaudClock::External => Sys::get_ext_clock_freq(),
        };

        pclk / self.clkdiv.read().bits()
    }
    fn set_frequency(&self, baud: u32, clock: BaudClock) -> Result<u32, MxcError::Error> {
        if clock == BaudClock::ERTCO {
            return Err(MxcError::Error::BadParam);
        }

        self.osr.write(|w| unsafe { w.bits(5) });

        let pclk_speed = Sys::get_clock_source()
            .get_clock_freq()
            .into_periph_clock_freq();

        let mut clk_div = pclk_speed / baud;
        let rem = pclk_speed % baud;

        let ctrl_code = match clock {
            BaudClock::PeriphClock => 0,
            BaudClock::IBRO => 2,
            _ => return Err(MxcError::Error::BadParam),
        };

        self.ctrl.write(|w| w.bclksrc().bits(ctrl_code));

        if clk_div == 0 || rem > baud / 2 {
            clk_div += 1
        }

        self.clkdiv.write(|w| unsafe { w.bits(clk_div) });

        Ok(self.get_frequency())
    }

    fn get_status(&self) -> u32 {
        self.status.read().bits()
    }
}
