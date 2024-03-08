#![no_std]
#![no_main]

use cortex_m_rt::entry;
use hal::{Sys, GPIO::GpioPort};
use max32655_hal as hal;
use max32655_pac::Peripherals;
use panic_halt as _;

#[entry]
fn main() -> ! {
    let led_pin: u8 = 24;
    let btn_pin: u8 = 18;

    let per = Peripherals::take().unwrap();
    let mut gpio0 = per.GPIO0;

    Sys::periph_reset(Sys::PeriphRst::GPIO0);
    Sys::periph_clock_enable(Sys::PeriphClock::GPIO0);

    gpio0.enable_output(true, led_pin).unwrap();
    gpio0.enable_input(true, btn_pin).unwrap();

    loop {
        let input = gpio0.read(btn_pin).unwrap();

        if input {
            gpio0.set_high(led_pin).unwrap();
        } else {
            gpio0.set_low(led_pin).unwrap();
        }
    }
}
