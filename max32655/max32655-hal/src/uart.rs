use crate::common;
use crate::pac;
use crate::MxcError;
use crate::Sys;
use crate::Sys::CoreClockSource;
use crate::GPIO::GpioPort;

#[derive(Clone, Copy)]
pub enum StopBits {
    One,
    Two,
}

pub const FIFO_DEPTH: u8 = 6;

pub trait Uart {
    fn init(&self, baud: u32, clock: Sys::CoreClockSource) -> Result<(), MxcError::Error>;
    fn shutdown(&self);

    fn set_rx_threshold(&self, thresh: u8) -> Result<(), MxcError::Error>;
    fn get_rx_threshold(&self) -> u8;
    fn set_datasize(&self, datasize: u8) -> Result<(), MxcError::Error>;
    fn set_parity(&self, enable: bool) -> Result<(), MxcError::Error>;
    fn set_stop_bits(&self, stop_bits: StopBits) -> Result<(), MxcError::Error>;
    fn set_frequency(&self, freq: u32, clock: Sys::CoreClockSource) -> Result<(), MxcError::Error>;
    fn get_status(&self) -> u32;

    fn write_byte(&self, data: u8) -> Result<(), MxcError::Error>;
    fn read_byte(&self) -> Result<u8, MxcError::Error>;
}

impl Uart for pac::UART {
    fn init(&self, baud: u32, clock: Sys::CoreClockSource) -> Result<(), MxcError::Error> {
        use Sys::CoreClockSource as clksrc;
        match clock {
            clksrc::ERTCO | clksrc::IBRO => Sys::enable_core_clock(clock)?,
            _ => {}
        };

        // let mut gpio = unsafe {
        //     pac::Peripherals::steal().GPIO0
        // };
        // Need to update GPIO to configure I/O for UART

        Sys::periph_clock_enable(Sys::PeriphClock::UART0);

        self.set_rx_threshold(1)?;
        self.set_datasize(8)?;
        self.set_parity(false)?;
        self.set_stop_bits(StopBits::One)?;

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
        Ok(())
    }
    fn set_parity(&self, enable: bool) -> Result<(), MxcError::Error> {
        Ok(())
    }
    fn set_stop_bits(&self, stop_bits: StopBits) -> Result<(), MxcError::Error> {
        Ok(())
    }
    fn set_frequency(&self, freq: u32, clock: CoreClockSource) -> Result<(), MxcError::Error> {
        Ok(())
    }

    fn get_status(&self) -> u32 {
        self.status.read().bits()
    }
}
