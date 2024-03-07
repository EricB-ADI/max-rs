// use crate::pac;

// use cortex_m::Peripherals;
// use crate::sys as Sys;

// fn delay_init()
// {
//     let mut syst = unsafe {
//         Peripherals::steal().SYST
//     };
//     let startick = syst.cvr.read();
//     // let ctrlsave = syst.

//     // syst.set_clock_source(clk_source);
//     // syst.enable_counter();

// }

// pub fn delay_us(usec: u32)
// {
//     let src = Sys::get_clock_source();

//     let freq : u32 = match src{
//         Sys::CoreClockSource::ISO => ,
//         Sys::CoreClockSource::Reserved => ,
//         Sys::CoreClockSource::ERFO => 32_000_000,
//         Sys::CoreClockSource::INRO => ,
//         Sys::CoreClockSource::IPO => 100_000_000,
//         Sys::CoreClockSource::IBRO => 7_372_800, 
//         Sys::CoreClockSource::ERTCO => ,
//         Sys::CoreClockSource::EXTCLK => ,
//     }


//     let sys_div = Sys::get_sys_clk_divider();

//     let base:u32 = 2;
//     let clk_f:u32 = freq / base.pow(sys_div as u32);

     


// }
// pub fn delay_ms(msec: u32)
// {
//     delay_us(1000 * msec);
// }
