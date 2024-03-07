#![no_std]
#![no_main]

use hal::SYS::PeriphClock;
// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
                     // use panic_abort as _; // requires nightly
                     // use panic_itm as _; // logs messages over ITM; requires ITM support
                     // use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use cortex_m_rt::entry;
use hal::SYS;
use max32655_hal as hal;
use max32655_pac::Peripherals;

#[entry]
fn main() -> ! {
    let pin: u32 = 1 << 24;
    let per = Peripherals::take().unwrap();
    let gpio0 = per.GPIO0;

    let rev = SYS::get_revision();
    assert!(rev == 0xb1 || rev == 0xa1);

    SYS::periph_clock_enable(PeriphClock::GPIO0);

    //enable gpio and configure at output
    gpio0.en0_set.write(|w| unsafe { w.bits(pin) });

    gpio0.outen_set.write(|w| unsafe { w.bits(pin) });

    gpio0
        .padctrl0
        .modify(|r, w| unsafe { w.bits(r.bits() | pin) });
    gpio0
        .padctrl1
        .modify(|r, w| unsafe { w.bits(r.bits() | pin) });

    loop {
        gpio0.out_set.write(|w| unsafe { w.bits(pin) });
        gpio0.out_clr.write(|w| unsafe { w.bits(pin) });
    }
}
