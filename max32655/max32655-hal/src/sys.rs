use crate::pac;
pub enum PeriphClock {
    GPIO0,
    GPIO1,
    DMA,
    SPI1,
    UART0,
    UART1,
    I2C0,
    TMR0,
    TMR1,
    TMR2,
    TMR3,
    ADC,
    I2C1,
    PulseTrain,
    BTLE,
    UART2,
    TRNG,
    Semaphore,
    OWM,
    CRC,
    AES,
    SPI0,
    I2C2,
    I2S0,
    WDT0,
    CPU1,
}
enum PclkDisBankOption {
    Bank0,
    Bank1,
}
struct PclkDisBank {
    bank: PclkDisBankOption,
    bit: u8,
}
impl PclkDisBank {
    pub fn new(bank: PclkDisBankOption, bit: u8) -> PclkDisBank {
        PclkDisBank { bank, bit }
    }
}
impl PeriphClock {
    fn values(&self) -> PclkDisBank {
        use PclkDisBankOption as Bank;
        match self {
            PeriphClock::GPIO0 => PclkDisBank::new(Bank::Bank0, 0),
            PeriphClock::GPIO1 => PclkDisBank::new(Bank::Bank0, 1),
            PeriphClock::DMA => PclkDisBank::new(Bank::Bank0, 5),
            PeriphClock::SPI1 => PclkDisBank::new(Bank::Bank0, 6),
            PeriphClock::UART0 => PclkDisBank::new(Bank::Bank0, 9),
            PeriphClock::UART1 => PclkDisBank::new(Bank::Bank0, 10),
            PeriphClock::I2C0 => PclkDisBank::new(Bank::Bank0, 13),
            PeriphClock::TMR0 => PclkDisBank::new(Bank::Bank0, 15),
            PeriphClock::TMR1 => PclkDisBank::new(Bank::Bank0, 16),
            PeriphClock::TMR2 => PclkDisBank::new(Bank::Bank0, 17),
            PeriphClock::TMR3 => PclkDisBank::new(Bank::Bank0, 18),
            PeriphClock::ADC => PclkDisBank::new(Bank::Bank0, 23),
            PeriphClock::I2C1 => PclkDisBank::new(Bank::Bank0, 28),
            PeriphClock::PulseTrain => PclkDisBank::new(Bank::Bank0, 29),
            PeriphClock::BTLE => PclkDisBank::new(Bank::Bank1, 0),
            PeriphClock::UART2 => PclkDisBank::new(Bank::Bank1, 1),
            PeriphClock::TRNG => PclkDisBank::new(Bank::Bank1, 2),
            PeriphClock::Semaphore => PclkDisBank::new(Bank::Bank1, 9),
            PeriphClock::OWM => PclkDisBank::new(Bank::Bank1, 13),
            PeriphClock::CRC => PclkDisBank::new(Bank::Bank1, 14),
            PeriphClock::AES => PclkDisBank::new(Bank::Bank1, 15),
            PeriphClock::SPI0 => PclkDisBank::new(Bank::Bank1, 16),
            PeriphClock::I2S0 => PclkDisBank::new(Bank::Bank1, 23),
            PeriphClock::I2C2 => PclkDisBank::new(Bank::Bank1, 24),
            PeriphClock::WDT0 => PclkDisBank::new(Bank::Bank1, 27),
            PeriphClock::CPU1 => PclkDisBank::new(Bank::Bank1, 31),
        }
    }
}

pub fn periph_clock_disable(clk: PeriphClock) {
    let pclk = clk.values();
    let gcr = unsafe{pac::Peripherals::steal().GCR};

    match pclk.bank {
        PclkDisBankOption::Bank0 => gcr
            .pclkdis0
            .modify(|r, w| unsafe { w.bits(r.bits() | (1 << pclk.bit)) }),
        PclkDisBankOption::Bank1 => gcr
            .pclkdis1
            .modify(|r, w| unsafe { w.bits(r.bits() | (1 << pclk.bit)) }),
    }
}
pub fn periph_clock_enable(clk: PeriphClock) {
    let gcr = unsafe{pac::Peripherals::steal().GCR};
    let pclk = clk.values();
    let mask = !(1 << pclk.bit);


    match pclk.bank {
        PclkDisBankOption::Bank0 => gcr
            .pclkdis0
            .modify(|r, w| unsafe { w.bits(r.bits() & mask) }),
        PclkDisBankOption::Bank1 => gcr
            .pclkdis1
            .modify(|r, w| unsafe { w.bits(r.bits() & mask) }),
    }
}
pub fn periph_clock_is_enabled(clk: PeriphClock) -> bool {
    let pclk = clk.values();
    let gcr = unsafe{pac::Peripherals::steal().GCR};

    match pclk.bank {
        PclkDisBankOption::Bank0 => gcr.pclkdis0.read().bits() & (1 << pclk.bit) == 0,
        PclkDisBankOption::Bank1 => gcr.pclkdis1.read().bits() & (1 << pclk.bit) != 1,
    }
}

pub fn get_revision() -> u32
{
    let gcr = unsafe{pac::Peripherals::steal().GCR};
    return gcr.revision.read().bits();
}
