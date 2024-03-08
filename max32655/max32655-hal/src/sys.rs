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
pub enum PeriphRst {
    DMA,
    WDT0,
    GPIO0,
    GPIO1,
    TMR0,
    TMR1,
    TMR2,
    TMR3,
    UART0,
    UART1,
    SPI1,
    I2C0,
    RTC,
    BTLE,
    TRNG,
    ADC,
    UART2,
    SOFT,
    PERIPH,
    SYS,

    I2C1,
    PulseTrain,
    OWM,
    CRC,
    AES,
    SPI0,
    Semaphore,
    I2S,
    I2C2,
    DVS,
    SIMO,
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

fn steal_gcr() -> pac::GCR {
    unsafe { pac::Peripherals::steal().GCR }
}

pub fn periph_clock_disable(clk: PeriphClock) {
    let pclk = clk.values();
    let gcr = unsafe { pac::Peripherals::steal().GCR };

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
    let gcr = unsafe { pac::Peripherals::steal().GCR };
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
    let gcr = unsafe { pac::Peripherals::steal().GCR };

    match pclk.bank {
        PclkDisBankOption::Bank0 => gcr.pclkdis0.read().bits() & (1 << pclk.bit) == 0,
        PclkDisBankOption::Bank1 => gcr.pclkdis1.read().bits() & (1 << pclk.bit) != 1,
    }
}

pub fn get_revision() -> u32 {
    steal_gcr().revision.read().bits()
}

pub enum CoreClockSource {
    ISO,
    Reserved,
    ERFO,
    INRO,
    IPO,
    IBRO,
    ERTCO,
    EXTCLK,
}

pub fn get_clock_source() -> CoreClockSource {
    let gcr = steal_gcr();

    use CoreClockSource as src;

    match gcr.clkctrl.read().sysclk_sel().bits() {
        0 => src::ISO,
        1 => src::Reserved,
        2 => src::ERFO,
        3 => src::INRO,
        4 => src::IPO,
        5 => src::IBRO,
        6 => src::ERTCO,
        7 => src::EXTCLK,
        _ => src::ISO,
    }
}

pub fn get_sys_clk_divider() -> u8 {
    steal_gcr().clkctrl.read().sysclk_div().bits()
}

pub fn periph_reset(periph: PeriphRst) {
    let gcr = steal_gcr();

    let set_bit = |rst_bank: u8, bit: u8| {
        let mask = 1 << bit;
        if rst_bank == 1 {
            gcr.rst1.modify(|r, w| unsafe { w.bits(r.bits() | mask) });
        } else {
            gcr.rst0.modify(|r, w| unsafe { w.bits(r.bits() | mask) });
        }
    };

    match periph {
        PeriphRst::DMA => set_bit(0, 0),
        PeriphRst::WDT0 => set_bit(0, 1),
        PeriphRst::GPIO0 => set_bit(0, 2),
        PeriphRst::GPIO1 => set_bit(0, 3),
        PeriphRst::TMR0 => set_bit(0, 5),
        PeriphRst::TMR1 => set_bit(0, 6),
        PeriphRst::TMR2 => set_bit(0, 7),
        PeriphRst::TMR3 => set_bit(0, 8),
        PeriphRst::UART0 => set_bit(0, 11),
        PeriphRst::UART1 => set_bit(0, 12),
        PeriphRst::SPI1 => set_bit(0, 13),
        PeriphRst::I2C0 => set_bit(0, 16),
        PeriphRst::RTC => set_bit(0, 17),
        PeriphRst::BTLE => set_bit(0, 18),
        PeriphRst::TRNG => set_bit(0, 24),
        PeriphRst::ADC => set_bit(0, 26),
        PeriphRst::UART2 => set_bit(0, 28),
        PeriphRst::SOFT => set_bit(0, 29),
        PeriphRst::PERIPH => set_bit(0, 30),
        PeriphRst::SYS => set_bit(0, 21),

        PeriphRst::I2C1 => set_bit(1, 0),
        PeriphRst::PulseTrain => set_bit(1, 1),
        PeriphRst::OWM => set_bit(1, 7),
        PeriphRst::CRC => set_bit(1, 9),
        PeriphRst::AES => set_bit(1, 10),
        PeriphRst::SPI0 => set_bit(1, 11),
        PeriphRst::Semaphore => set_bit(1, 16),
        PeriphRst::I2S => set_bit(1, 19),
        PeriphRst::I2C2 => set_bit(1, 20),
        PeriphRst::DVS => set_bit(1, 24),
        PeriphRst::SIMO => set_bit(1, 25),
        PeriphRst::CPU1 => set_bit(1, 21),
    }
}
