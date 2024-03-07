#![no_std]

pub use max32655_pac as pac;

pub mod gpio;
pub use gpio as GPIO;

pub mod sys;
pub use sys as SYS;
