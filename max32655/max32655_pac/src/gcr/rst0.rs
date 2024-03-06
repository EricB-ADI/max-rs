#[doc = "Register `RST0` reader"]
pub struct R(crate::R<RST0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RST0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RST0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RST0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RST0` writer"]
pub struct W(crate::W<RST0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RST0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<RST0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RST0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMA` reader - DMA Reset."]
pub type DMA_R = crate::BitReader<RESET_A>;
#[doc = "DMA Reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RESET_A {
    #[doc = "0: Reset complete."]
    RESET_DONE = 0,
    #[doc = "1: Starts Reset or indicates reset in progress."]
    BUSY = 1,
}
impl From<RESET_A> for bool {
    #[inline(always)]
    fn from(variant: RESET_A) -> Self {
        variant as u8 != 0
    }
}
impl DMA_R {}
#[doc = "Field `DMA` writer - DMA Reset."]
pub type DMA_W<'a, const O: u8> = crate::BitWriter<'a, u32, RST0_SPEC, RESET_A, O>;
impl<'a, const O: u8> DMA_W<'a, O> {}
#[doc = "Field `WDT0` reader - Watchdog Timer 0 Reset."]
pub type WDT0_R = crate::BitReader<RESET_A>;
#[doc = "Watchdog Timer 0 Reset.\n\nValue on reset: 0"]

impl WDT0_R {}
#[doc = "Field `WDT0` writer - Watchdog Timer 0 Reset."]
pub type WDT0_W<'a, const O: u8> = crate::BitWriter<'a, u32, RST0_SPEC, RESET_A, O>;
impl<'a, const O: u8> WDT0_W<'a, O> {}
#[doc = "Field `GPIO0` reader - GPIO0 Reset. Setting this bit to 1 resets GPIO0 pins to their default states."]
pub type GPIO0_R = crate::BitReader<RESET_A>;
#[doc = "GPIO0 Reset. Setting this bit to 1 resets GPIO0 pins to their default states.\n\nValue on reset: 0"]

impl GPIO0_R {}
#[doc = "Field `GPIO0` writer - GPIO0 Reset. Setting this bit to 1 resets GPIO0 pins to their default states."]
pub type GPIO0_W<'a, const O: u8> = crate::BitWriter<'a, u32, RST0_SPEC, RESET_A, O>;
impl<'a, const O: u8> GPIO0_W<'a, O> {}
#[doc = "Field `GPIO1` reader - GPIO1 Reset. Setting this bit to 1 resets GPIO1 pins to their default states."]
pub type GPIO1_R = crate::BitReader<RESET_A>;
#[doc = "GPIO1 Reset. Setting this bit to 1 resets GPIO1 pins to their default states.\n\nValue on reset: 0"]

impl GPIO1_R {}
#[doc = "Field `GPIO1` writer - GPIO1 Reset. Setting this bit to 1 resets GPIO1 pins to their default states."]
pub type GPIO1_W<'a, const O: u8> = crate::BitWriter<'a, u32, RST0_SPEC, RESET_A, O>;
impl<'a, const O: u8> GPIO1_W<'a, O> {}
#[doc = "Field `TMR0` reader - Timer 0 Reset. Setting this bit to 1 resets Timer 0 blocks."]
pub type TMR0_R = crate::BitReader<RESET_A>;
#[doc = "Timer 0 Reset. Setting this bit to 1 resets Timer 0 blocks.\n\nValue on reset: 0"]

impl TMR0_R {}
#[doc = "Field `TMR0` writer - Timer 0 Reset. Setting this bit to 1 resets Timer 0 blocks."]
pub type TMR0_W<'a, const O: u8> = crate::BitWriter<'a, u32, RST0_SPEC, RESET_A, O>;
impl<'a, const O: u8> TMR0_W<'a, O> {}
#[doc = "Field `TMR1` reader - Timer 1 Reset. Setting this bit to 1 resets Timer 1 blocks."]
pub type TMR1_R = crate::BitReader<RESET_A>;
#[doc = "Timer 1 Reset. Setting this bit to 1 resets Timer 1 blocks.\n\nValue on reset: 0"]

impl TMR1_R {}
#[doc = "Field `TMR1` writer - Timer 1 Reset. Setting this bit to 1 resets Timer 1 blocks."]
pub type TMR1_W<'a, const O: u8> = crate::BitWriter<'a, u32, RST0_SPEC, RESET_A, O>;
impl<'a, const O: u8> TMR1_W<'a, O> {}
#[doc = "Field `TMR2` reader - Timer 2 Reset. Setting this bit to 1 resets Timer 2 blocks."]
pub type TMR2_R = crate::BitReader<RESET_A>;
#[doc = "Timer 2 Reset. Setting this bit to 1 resets Timer 2 blocks.\n\nValue on reset: 0"]

impl TMR2_R {}
#[doc = "Field `TMR2` writer - Timer 2 Reset. Setting this bit to 1 resets Timer 2 blocks."]
pub type TMR2_W<'a, const O: u8> = crate::BitWriter<'a, u32, RST0_SPEC, RESET_A, O>;
impl<'a, const O: u8> TMR2_W<'a, O> {}
#[doc = "Field `TMR3` reader - Timer 3 Reset. Setting this bit to 1 resets Timer 3 blocks."]
pub type TMR3_R = crate::BitReader<RESET_A>;
#[doc = "Timer 3 Reset. Setting this bit to 1 resets Timer 3 blocks.\n\nValue on reset: 0"]

impl TMR3_R {}
#[doc = "Field `TMR3` writer - Timer 3 Reset. Setting this bit to 1 resets Timer 3 blocks."]
pub type TMR3_W<'a, const O: u8> = crate::BitWriter<'a, u32, RST0_SPEC, RESET_A, O>;
impl<'a, const O: u8> TMR3_W<'a, O> {}
#[doc = "Field `UART0` reader - UART 0 Reset. Setting this bit to 1 resets all UART 0 blocks."]
pub type UART0_R = crate::BitReader<RESET_A>;
#[doc = "UART 0 Reset. Setting this bit to 1 resets all UART 0 blocks.\n\nValue on reset: 0"]

impl UART0_R {}
#[doc = "Field `UART0` writer - UART 0 Reset. Setting this bit to 1 resets all UART 0 blocks."]
pub type UART0_W<'a, const O: u8> = crate::BitWriter<'a, u32, RST0_SPEC, RESET_A, O>;
impl<'a, const O: u8> UART0_W<'a, O> {}
#[doc = "Field `UART1` reader - UART 1 Reset. Setting this bit to 1 resets all UART 1 blocks."]
pub type UART1_R = crate::BitReader<RESET_A>;
#[doc = "UART 1 Reset. Setting this bit to 1 resets all UART 1 blocks.\n\nValue on reset: 0"]

impl UART1_R {}
#[doc = "Field `UART1` writer - UART 1 Reset. Setting this bit to 1 resets all UART 1 blocks."]
pub type UART1_W<'a, const O: u8> = crate::BitWriter<'a, u32, RST0_SPEC, RESET_A, O>;
impl<'a, const O: u8> UART1_W<'a, O> {}
#[doc = "Field `SPI1` reader - SPI 1 Reset. Setting this bit to 1 resets all SPI 1 blocks."]
pub type SPI1_R = crate::BitReader<RESET_A>;
#[doc = "SPI 1 Reset. Setting this bit to 1 resets all SPI 1 blocks.\n\nValue on reset: 0"]

impl SPI1_R {}
#[doc = "Field `SPI1` writer - SPI 1 Reset. Setting this bit to 1 resets all SPI 1 blocks."]
pub type SPI1_W<'a, const O: u8> = crate::BitWriter<'a, u32, RST0_SPEC, RESET_A, O>;
impl<'a, const O: u8> SPI1_W<'a, O> {}
#[doc = "Field `I2C0` reader - I2C 0 Reset."]
pub type I2C0_R = crate::BitReader<RESET_A>;
#[doc = "I2C 0 Reset.\n\nValue on reset: 0"]

impl I2C0_R {}
#[doc = "Field `I2C0` writer - I2C 0 Reset."]
pub type I2C0_W<'a, const O: u8> = crate::BitWriter<'a, u32, RST0_SPEC, RESET_A, O>;
impl<'a, const O: u8> I2C0_W<'a, O> {}
#[doc = "Field `RTC` reader - Real Time Clock Reset."]
pub type RTC_R = crate::BitReader<RESET_A>;
#[doc = "Real Time Clock Reset.\n\nValue on reset: 0"]

impl RTC_R {}
#[doc = "Field `RTC` writer - Real Time Clock Reset."]
pub type RTC_W<'a, const O: u8> = crate::BitWriter<'a, u32, RST0_SPEC, RESET_A, O>;
impl<'a, const O: u8> RTC_W<'a, O> {}
#[doc = "Field `SMPHR` reader - Semaphore Reset."]
pub type SMPHR_R = crate::BitReader<RESET_A>;
#[doc = "Semaphore Reset.\n\nValue on reset: 0"]

impl SMPHR_R {}
#[doc = "Field `SMPHR` writer - Semaphore Reset."]
pub type SMPHR_W<'a, const O: u8> = crate::BitWriter<'a, u32, RST0_SPEC, RESET_A, O>;
impl<'a, const O: u8> SMPHR_W<'a, O> {}
#[doc = "Field `TRNG` reader - TRNG Reset. This reset is only available during the manufacture testing phase."]
pub type TRNG_R = crate::BitReader<RESET_A>;
#[doc = "TRNG Reset. This reset is only available during the manufacture testing phase.\n\nValue on reset: 0"]

impl TRNG_R {}
#[doc = "Field `TRNG` writer - TRNG Reset. This reset is only available during the manufacture testing phase."]
pub type TRNG_W<'a, const O: u8> = crate::BitWriter<'a, u32, RST0_SPEC, RESET_A, O>;
impl<'a, const O: u8> TRNG_W<'a, O> {}
#[doc = "Field `CNN` reader - CNN Reset."]
pub type CNN_R = crate::BitReader<RESET_A>;
#[doc = "CNN Reset.\n\nValue on reset: 0"]

impl CNN_R {}
#[doc = "Field `CNN` writer - CNN Reset."]
pub type CNN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RST0_SPEC, RESET_A, O>;
impl<'a, const O: u8> CNN_W<'a, O> {}
#[doc = "Field `ADC` reader - ADC Reset."]
pub type ADC_R = crate::BitReader<RESET_A>;
#[doc = "ADC Reset.\n\nValue on reset: 0"]

impl ADC_R {}
#[doc = "Field `ADC` writer - ADC Reset."]
pub type ADC_W<'a, const O: u8> = crate::BitWriter<'a, u32, RST0_SPEC, RESET_A, O>;
impl<'a, const O: u8> ADC_W<'a, O> {}
#[doc = "Field `UART2` reader - UART2 Reset. Setting this bit to 1 resets all UART 2 blocks."]
pub type UART2_R = crate::BitReader<RESET_A>;
#[doc = "UART2 Reset. Setting this bit to 1 resets all UART 2 blocks.\n\nValue on reset: 0"]

impl UART2_R {}
#[doc = "Field `UART2` writer - UART2 Reset. Setting this bit to 1 resets all UART 2 blocks."]
pub type UART2_W<'a, const O: u8> = crate::BitWriter<'a, u32, RST0_SPEC, RESET_A, O>;
impl<'a, const O: u8> UART2_W<'a, O> {}
#[doc = "Field `SOFT` reader - Soft Reset. Setting this bit to 1 resets everything except the CPU and the watchdog timer."]
pub type SOFT_R = crate::BitReader<RESET_A>;
#[doc = "Soft Reset. Setting this bit to 1 resets everything except the CPU and the watchdog timer.\n\nValue on reset: 0"]

impl SOFT_R {}
#[doc = "Field `SOFT` writer - Soft Reset. Setting this bit to 1 resets everything except the CPU and the watchdog timer."]
pub type SOFT_W<'a, const O: u8> = crate::BitWriter<'a, u32, RST0_SPEC, RESET_A, O>;
impl<'a, const O: u8> SOFT_W<'a, O> {}
#[doc = "Field `PERIPH` reader - Peripheral Reset. Setting this bit to 1 resets all peripherals. The CPU core, the watchdog timer, and all GPIO pins are unaffected by this reset."]
pub type PERIPH_R = crate::BitReader<RESET_A>;
#[doc = "Peripheral Reset. Setting this bit to 1 resets all peripherals. The CPU core, the watchdog timer, and all GPIO pins are unaffected by this reset.\n\nValue on reset: 0"]

impl PERIPH_R {}
#[doc = "Field `PERIPH` writer - Peripheral Reset. Setting this bit to 1 resets all peripherals. The CPU core, the watchdog timer, and all GPIO pins are unaffected by this reset."]
pub type PERIPH_W<'a, const O: u8> = crate::BitWriter<'a, u32, RST0_SPEC, RESET_A, O>;
impl<'a, const O: u8> PERIPH_W<'a, O> {}
#[doc = "Field `SYS` reader - System Reset. Setting this bit to 1 resets the CPU core and all peripherals, including the watchdog timer."]
pub type SYS_R = crate::BitReader<RESET_A>;
#[doc = "System Reset. Setting this bit to 1 resets the CPU core and all peripherals, including the watchdog timer.\n\nValue on reset: 0"]

impl SYS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RESET_A {
        match self.bits {
            false => RESET_A::RESET_DONE,
            true => RESET_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `RESET_DONE`"]
    #[inline(always)]
    pub fn is_reset_done(&self) -> bool {
        *self == RESET_A::RESET_DONE
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == RESET_A::BUSY
    }
}
#[doc = "Field `SYS` writer - System Reset. Setting this bit to 1 resets the CPU core and all peripherals, including the watchdog timer."]
pub type SYS_W<'a, const O: u8> = crate::BitWriter<'a, u32, RST0_SPEC, RESET_A, O>;
impl<'a, const O: u8> SYS_W<'a, O> {
    #[doc = "Reset complete."]
    #[inline(always)]
    pub fn reset_done(self) -> &'a mut W {
        self.variant(RESET_A::RESET_DONE)
    }
    #[doc = "Starts Reset or indicates reset in progress."]
    #[inline(always)]
    pub fn busy(self) -> &'a mut W {
        self.variant(RESET_A::BUSY)
    }
}
impl R {
    #[doc = "Bit 0 - DMA Reset."]
    #[inline(always)]
    pub fn dma(&self) -> DMA_R {
        DMA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Watchdog Timer 0 Reset."]
    #[inline(always)]
    pub fn wdt0(&self) -> WDT0_R {
        WDT0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GPIO0 Reset. Setting this bit to 1 resets GPIO0 pins to their default states."]
    #[inline(always)]
    pub fn gpio0(&self) -> GPIO0_R {
        GPIO0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GPIO1 Reset. Setting this bit to 1 resets GPIO1 pins to their default states."]
    #[inline(always)]
    pub fn gpio1(&self) -> GPIO1_R {
        GPIO1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Timer 0 Reset. Setting this bit to 1 resets Timer 0 blocks."]
    #[inline(always)]
    pub fn tmr0(&self) -> TMR0_R {
        TMR0_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Timer 1 Reset. Setting this bit to 1 resets Timer 1 blocks."]
    #[inline(always)]
    pub fn tmr1(&self) -> TMR1_R {
        TMR1_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Timer 2 Reset. Setting this bit to 1 resets Timer 2 blocks."]
    #[inline(always)]
    pub fn tmr2(&self) -> TMR2_R {
        TMR2_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Timer 3 Reset. Setting this bit to 1 resets Timer 3 blocks."]
    #[inline(always)]
    pub fn tmr3(&self) -> TMR3_R {
        TMR3_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - UART 0 Reset. Setting this bit to 1 resets all UART 0 blocks."]
    #[inline(always)]
    pub fn uart0(&self) -> UART0_R {
        UART0_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - UART 1 Reset. Setting this bit to 1 resets all UART 1 blocks."]
    #[inline(always)]
    pub fn uart1(&self) -> UART1_R {
        UART1_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SPI 1 Reset. Setting this bit to 1 resets all SPI 1 blocks."]
    #[inline(always)]
    pub fn spi1(&self) -> SPI1_R {
        SPI1_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - I2C 0 Reset."]
    #[inline(always)]
    pub fn i2c0(&self) -> I2C0_R {
        I2C0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Real Time Clock Reset."]
    #[inline(always)]
    pub fn rtc(&self) -> RTC_R {
        RTC_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 22 - Semaphore Reset."]
    #[inline(always)]
    pub fn smphr(&self) -> SMPHR_R {
        SMPHR_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - TRNG Reset. This reset is only available during the manufacture testing phase."]
    #[inline(always)]
    pub fn trng(&self) -> TRNG_R {
        TRNG_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - CNN Reset."]
    #[inline(always)]
    pub fn cnn(&self) -> CNN_R {
        CNN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - ADC Reset."]
    #[inline(always)]
    pub fn adc(&self) -> ADC_R {
        ADC_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - UART2 Reset. Setting this bit to 1 resets all UART 2 blocks."]
    #[inline(always)]
    pub fn uart2(&self) -> UART2_R {
        UART2_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Soft Reset. Setting this bit to 1 resets everything except the CPU and the watchdog timer."]
    #[inline(always)]
    pub fn soft(&self) -> SOFT_R {
        SOFT_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Peripheral Reset. Setting this bit to 1 resets all peripherals. The CPU core, the watchdog timer, and all GPIO pins are unaffected by this reset."]
    #[inline(always)]
    pub fn periph(&self) -> PERIPH_R {
        PERIPH_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - System Reset. Setting this bit to 1 resets the CPU core and all peripherals, including the watchdog timer."]
    #[inline(always)]
    pub fn sys(&self) -> SYS_R {
        SYS_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA Reset."]
    #[inline(always)]
    #[must_use]
    pub fn dma(&mut self) -> DMA_W<0> {
        DMA_W::new(self)
    }
    #[doc = "Bit 1 - Watchdog Timer 0 Reset."]
    #[inline(always)]
    #[must_use]
    pub fn wdt0(&mut self) -> WDT0_W<1> {
        WDT0_W::new(self)
    }
    #[doc = "Bit 2 - GPIO0 Reset. Setting this bit to 1 resets GPIO0 pins to their default states."]
    #[inline(always)]
    #[must_use]
    pub fn gpio0(&mut self) -> GPIO0_W<2> {
        GPIO0_W::new(self)
    }
    #[doc = "Bit 3 - GPIO1 Reset. Setting this bit to 1 resets GPIO1 pins to their default states."]
    #[inline(always)]
    #[must_use]
    pub fn gpio1(&mut self) -> GPIO1_W<3> {
        GPIO1_W::new(self)
    }
    #[doc = "Bit 5 - Timer 0 Reset. Setting this bit to 1 resets Timer 0 blocks."]
    #[inline(always)]
    #[must_use]
    pub fn tmr0(&mut self) -> TMR0_W<5> {
        TMR0_W::new(self)
    }
    #[doc = "Bit 6 - Timer 1 Reset. Setting this bit to 1 resets Timer 1 blocks."]
    #[inline(always)]
    #[must_use]
    pub fn tmr1(&mut self) -> TMR1_W<6> {
        TMR1_W::new(self)
    }
    #[doc = "Bit 7 - Timer 2 Reset. Setting this bit to 1 resets Timer 2 blocks."]
    #[inline(always)]
    #[must_use]
    pub fn tmr2(&mut self) -> TMR2_W<7> {
        TMR2_W::new(self)
    }
    #[doc = "Bit 8 - Timer 3 Reset. Setting this bit to 1 resets Timer 3 blocks."]
    #[inline(always)]
    #[must_use]
    pub fn tmr3(&mut self) -> TMR3_W<8> {
        TMR3_W::new(self)
    }
    #[doc = "Bit 11 - UART 0 Reset. Setting this bit to 1 resets all UART 0 blocks."]
    #[inline(always)]
    #[must_use]
    pub fn uart0(&mut self) -> UART0_W<11> {
        UART0_W::new(self)
    }
    #[doc = "Bit 12 - UART 1 Reset. Setting this bit to 1 resets all UART 1 blocks."]
    #[inline(always)]
    #[must_use]
    pub fn uart1(&mut self) -> UART1_W<12> {
        UART1_W::new(self)
    }
    #[doc = "Bit 13 - SPI 1 Reset. Setting this bit to 1 resets all SPI 1 blocks."]
    #[inline(always)]
    #[must_use]
    pub fn spi1(&mut self) -> SPI1_W<13> {
        SPI1_W::new(self)
    }
    #[doc = "Bit 16 - I2C 0 Reset."]
    #[inline(always)]
    #[must_use]
    pub fn i2c0(&mut self) -> I2C0_W<16> {
        I2C0_W::new(self)
    }
    #[doc = "Bit 17 - Real Time Clock Reset."]
    #[inline(always)]
    #[must_use]
    pub fn rtc(&mut self) -> RTC_W<17> {
        RTC_W::new(self)
    }
    #[doc = "Bit 22 - Semaphore Reset."]
    #[inline(always)]
    #[must_use]
    pub fn smphr(&mut self) -> SMPHR_W<22> {
        SMPHR_W::new(self)
    }
    #[doc = "Bit 24 - TRNG Reset. This reset is only available during the manufacture testing phase."]
    #[inline(always)]
    #[must_use]
    pub fn trng(&mut self) -> TRNG_W<24> {
        TRNG_W::new(self)
    }
    #[doc = "Bit 25 - CNN Reset."]
    #[inline(always)]
    #[must_use]
    pub fn cnn(&mut self) -> CNN_W<25> {
        CNN_W::new(self)
    }
    #[doc = "Bit 26 - ADC Reset."]
    #[inline(always)]
    #[must_use]
    pub fn adc(&mut self) -> ADC_W<26> {
        ADC_W::new(self)
    }
    #[doc = "Bit 28 - UART2 Reset. Setting this bit to 1 resets all UART 2 blocks."]
    #[inline(always)]
    #[must_use]
    pub fn uart2(&mut self) -> UART2_W<28> {
        UART2_W::new(self)
    }
    #[doc = "Bit 29 - Soft Reset. Setting this bit to 1 resets everything except the CPU and the watchdog timer."]
    #[inline(always)]
    #[must_use]
    pub fn soft(&mut self) -> SOFT_W<29> {
        SOFT_W::new(self)
    }
    #[doc = "Bit 30 - Peripheral Reset. Setting this bit to 1 resets all peripherals. The CPU core, the watchdog timer, and all GPIO pins are unaffected by this reset."]
    #[inline(always)]
    #[must_use]
    pub fn periph(&mut self) -> PERIPH_W<30> {
        PERIPH_W::new(self)
    }
    #[doc = "Bit 31 - System Reset. Setting this bit to 1 resets the CPU core and all peripherals, including the watchdog timer."]
    #[inline(always)]
    #[must_use]
    pub fn sys(&mut self) -> SYS_W<31> {
        SYS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Reset.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rst0](index.html) module"]
pub struct RST0_SPEC;
impl crate::RegisterSpec for RST0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rst0::R](R) reader structure"]
impl crate::Readable for RST0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rst0::W](W) writer structure"]
impl crate::Writable for RST0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RST0 to value 0"]
impl crate::Resettable for RST0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
