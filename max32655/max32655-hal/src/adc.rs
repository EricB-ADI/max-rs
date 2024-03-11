use crate::pac;
use crate::Sys;

pub enum AdcError {
    Overflow,
}

pub enum Chan {
    AIN0,
    AIN1,
    AIN2,
    AIN3,
    AIN4,
    AIN5,
    AIN6,
    AIN7,
    VCOREA,
    VCOREb,
    VRXOUT,
    VTXOUT,
    VDDDA,
    VDDIO4 = 14,
    VDDIOH4,
    VREGI4,
}

pub enum DataAlignment {
    LSB,
    MSB,
}

pub enum RefSel {
    Bandgap,
    VDD2,
}

pub enum AdcFlags {}

pub trait Adc {
    fn init(&mut self);
    fn shutdown(&mut self);
    fn is_busy(&mut self) -> bool;
    fn enable_interrupt(&mut self);
    fn get_flags(&mut self) -> u16;
    fn clear_flags(&mut self, flags: u32);
    fn set_conversion_speed(&mut self, hz: u32);
    fn get_conversion_speed(&mut self) -> u32;
    fn set_data_alignment(&mut self, alignment: DataAlignment);
    fn get_data_alignment(&self) -> DataAlignment;
    fn set_ext_scale(&mut self);
    fn set_reference(&mut self, refsel: RefSel);
    fn set_channel(&mut self, channel: Chan);
    fn start_conversion(&self);
    fn get_conversion(&self) -> Result<u16, AdcError>;

    fn wait_until_not_busy(&mut self) {
        while self.is_busy() {}
    }
    fn read_sync(&mut self, channel: Chan) -> Result<u16, AdcError> {
        self.set_channel(channel);
        self.start_conversion();
        self.wait_until_not_busy();

        self.get_conversion()
    }
}

impl Adc for pac::ADC {
    fn init(&mut self) {
        Sys::periph_reset(Sys::PeriphRst::ADC);
        Sys::periph_clock_enable(Sys::PeriphClock::ADC);

        let ref_ready_bit = 1 << 17;
        let default_conversion_speed = 5847;

        self.set_conversion_speed(default_conversion_speed);
        self.clear_flags(ref_ready_bit);

        let startup_mask = 1 << 1 | 1 << 3;

        //enable adc and turn on reference buffer power
        self.ctrl
            .modify(|r, w| unsafe { w.bits(r.bits() | startup_mask) });

        while self.intr.read().ref_ready_if().bit_is_clear() {}

        self.clear_flags(ref_ready_bit);
    }
    fn shutdown(&mut self) {}
    fn is_busy(&mut self) -> bool {
        self.status.read().active().bit_is_set() && self.status.read().overflow().bit_is_clear()
    }
    fn enable_interrupt(&mut self) {}
    fn get_flags(&mut self) -> u16 {
        ((self.intr.read().bits() >> 16) & 0xFFFF) as u16
    }
    fn clear_flags(&mut self, flags: u32) {
        self.intr.modify(|r, w| unsafe { w.bits(r.bits() | flags) });
    }
    fn set_conversion_speed(&mut self, hz: u32) {
        _ = hz;
        self.ctrl
            .modify(|r, w| unsafe { w.bits(r.bits() | 1 << 11) })
    }
    fn get_conversion_speed(&mut self) -> u32 {
        0
    }

    fn get_data_alignment(&self) -> DataAlignment {
        if self.ctrl.read().data_align().bit_is_set() {
            DataAlignment::MSB
        } else {
            DataAlignment::LSB
        }
    }

    fn set_data_alignment(&mut self, alignment: DataAlignment) {
        match alignment {
            DataAlignment::LSB => self.ctrl.write(|w| w.data_align().clear_bit()),
            DataAlignment::MSB => self.ctrl.write(|w| w.data_align().set_bit()),
        }
    }

    fn set_ext_scale(&mut self) {
        todo!()
    }
    fn set_reference(&mut self, refsel: RefSel) {
        self.ctrl.write(|w| match refsel {
            RefSel::Bandgap => w.ref_sel().clear_bit(),
            RefSel::VDD2 => w.ref_sel().set_bit(),
        });
    }
    fn set_channel(&mut self, channel: Chan) {
        self.ctrl.write(|w| unsafe {
            w.ch_sel().bits(0);
            w.ch_sel().bits(channel as u8)
        });
    }
    fn start_conversion(&self) {
        self.ctrl.write(|w| w.start().set_bit());
    }
    fn get_conversion(&self) -> Result<u16, AdcError> {
        if self.status.read().overflow().bit_is_set() {
            return Err(AdcError::Overflow);
        }

        let data = self.data.read().bits() as u16;

        match self.get_data_alignment() {
            DataAlignment::LSB => Ok(data),
            DataAlignment::MSB => Ok(data >> 5),
        }
    }
}
