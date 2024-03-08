#![no_std]

pub use max32655_pac as pac;

pub mod gpio;
pub use gpio as GPIO;

pub mod sys;
pub use sys as Sys;

pub mod delay;
pub use delay as Delay;

pub mod adc;
pub use adc as ADC;

pub mod aes;
pub use aes as AES;

pub mod crc;
pub use crc as CRC;

pub mod dma;
pub use dma as DMA;

pub mod flc;
pub use flc as FLC;

pub mod gcr;
pub use gcr as GCR;

pub mod i2c;
pub use i2c as I2C;

pub mod i2s;
pub use i2s as I2S;

pub mod icc;
pub use icc as ICC;

pub mod lp;
pub use lp as LP;

pub mod lpcmp;
pub use lpcmp as LPCMP;

pub mod owm;
pub use owm as OWM;

pub mod pt;
pub use pt as PT;

pub mod rtc;
pub use rtc as RTC;

pub mod sema;
pub use sema as SEMA;

pub mod simo;
pub use simo as SIMO;

pub mod spi;
pub use spi as SPI;

pub mod tmr;
pub use tmr as TMR;

pub mod uart;
pub use uart as UART;

pub mod wdt;
pub use wdt as WDT;

pub mod wut;
pub use wut as WUT;
