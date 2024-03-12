use crate::{pac, GCR::steal_gcr};
use cortex_m;

pub enum Ovr {
    V0_9,
    V1_0,
    V1_1,
}

pub enum PMMode {
    IPO,
    IBRO,
}

fn arm_set_sleepdeep(set: bool) {
    if (set) {
        unsafe { cortex_m::Peripherals::steal().SCB.set_sleepdeep() };
    } else {
        unsafe { cortex_m::Peripherals::steal().SCB.clear_sleepdeep() };
    }
}

pub fn enter_sleep_mode() {
    clear_wake_status();

    arm_set_sleepdeep(false);

    cortex_m::asm::wfi();
}
pub fn enter_low_power_mode() {
    clear_wake_status();

    unsafe {
        pac::Peripherals::steal()
            .MCR
            .ctrl
            .write(|w| w.ertco_en().set_bit())
    }

    arm_set_sleepdeep(true);
    steal_gcr().pm.write(|w| w.mode().upm());

    cortex_m::asm::wfi();
}
pub fn enter_micro_power_mode() {}
pub fn enter_standby_mode() {}
pub fn enter_backup_mode() {}

pub fn enter_power_down_mode() {}

pub fn enable_bandgap(enable: bool) {}
pub fn bandgap_is_on() -> bool {
    true
}

pub fn clear_wake_status() {}
pub fn enable_gpio_wakeup(enable: bool) {}
pub fn enable_rtc_alarm_wakeup(enable: bool) {}
pub fn enable_timer_wakeup(enable: bool) {}
pub fn enable_usb_wakeup(enable: bool) {}
pub fn enable_wut_alarm_wakeup(enable: bool) {}
pub fn enable_lpcmp_wakeup(enable: bool) {}
pub fn enable_ha0_wakeup(enable: bool) {}
pub fn enable_ha1_wakeup(enable: bool) {}

pub fn config_deep_sleep_clocks(mask: u32) {}

pub fn nfc_osc_bypass_enable(enable: bool) {}

pub fn nfc_osc_bypass_is_enabled() -> bool {
    true
}
