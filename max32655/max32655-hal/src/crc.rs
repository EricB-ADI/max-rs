use crate::pac;
use crate::Sys;
use crate::common::BitOrder;

pub trait Crc {
    fn init(&mut self);
    fn shutdown(&mut self);
    fn set_bit_order(&self, order: BitOrder);
    fn get_bit_order(&self) -> BitOrder;
    fn swap_data_in(&self, order: BitOrder);
    fn swap_data_out(&self, order: BitOrder);
    fn set_polynomial(&self, poly: u32);
    fn get_polynomail(&self) -> u32;
    fn get_result(&self) -> u32;
    fn compute(&self, buf: &[u32]) -> u32;
}

impl Crc for pac::CRC {
    fn init(&mut self) {
        Sys::periph_clock_enable(Sys::PeriphClock::CRC);

        self.ctrl.write(|w| unsafe { w.bits(0) });

        self.val.write(|w| unsafe { w.bits(u32::MAX) })
    }

    fn shutdown(&mut self) {
        self.ctrl.write(|w| w.en().clear_bit());
    }

    fn set_bit_order(&self, order: BitOrder) {
        self.ctrl.write(|w| match order {
            BitOrder::LSB => w.msb().clear_bit(),
            BitOrder::MSB => w.msb().set_bit(),
        })
    }
    fn get_bit_order(&self) -> BitOrder {
        if self.ctrl.read().msb().bit_is_set() {
            BitOrder::MSB
        } else {
            BitOrder::LSB
        }
    }

    fn swap_data_in(&self, order: BitOrder) {
        self.ctrl.write(|w| match order {
            BitOrder::LSB => w.byte_swap_in().clear_bit(),
            BitOrder::MSB => w.byte_swap_in().set_bit(),
        })
    }
    fn swap_data_out(&self, order: BitOrder) {
        self.ctrl.write(|w| match order {
            BitOrder::LSB => w.byte_swap_out().clear_bit(),
            BitOrder::MSB => w.byte_swap_out().set_bit(),
        })
    }

    fn set_polynomial(&self, poly: u32) {
        self.poly.write(|w| unsafe { w.bits(poly) })
    }
    fn get_polynomail(&self) -> u32 {
        self.poly.read().bits()
    }

    fn get_result(&self) -> u32 {
        self.val.read().bits()
    }

    fn compute(&self, buf: &[u32]) -> u32 {
        self.ctrl.write(|w| w.en().set_bit());

        for &data in buf {
            self.datain32().write(|w| unsafe { w.bits(data) });

            while self.ctrl.read().busy().bit_is_set() {}
        }

        self.get_result()
    }
}
