use crate::pac;
use crate::Sys;

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
    VDDIO4,
    VDDIOH4,
    VREGI4,
}
pub enum RefSel {
    Bandgap,
    VDD2,
}

pub enum AdcFlags {}

pub trait Adc {
    fn init(&mut self);
    fn shutdown(&mut self);
    fn is_busy(&mut self);
    fn enable_interrupt(&mut self);
    fn get_flags(&mut self) -> u32;
    fn clear_flags(&mut self, flags: u32);
    fn set_conversion_speed(&mut self, hz: u32);
    fn get_conversion_speed(&mut self) -> u32;
    fn set_data_alignment(&mut self, msb_justify: i32);
    fn set_ext_scale(&mut self);
    fn ref_select(&mut self, refsel: RefSel);
    fn read(&mut self, channel: Chan) -> u16;
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
    fn is_busy(&mut self) {}
    fn enable_interrupt(&mut self) {}
    fn get_flags(&mut self) -> u32 {
        0
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
    fn set_data_alignment(&mut self, msb_justify: i32) {
        let _ = msb_justify;
        todo!();
    }
    fn set_ext_scale(&mut self) {}
    fn ref_select(&mut self, refsel: RefSel) {
        self.ctrl.modify(|r, w| unsafe {
            let refsel_mask = 1 << 4;
            match refsel {
                RefSel::Bandgap => w.bits(r.bits() & !refsel_mask),
                RefSel::VDD2 => w.bits(r.bits() | refsel_mask),
            }
        });
    }
    fn read(&mut self, channel: Chan) -> u16 {
        let _ = channel;

        self.data.read().bits() as u16
    }
}
