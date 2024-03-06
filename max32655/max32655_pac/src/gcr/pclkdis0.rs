#[doc = "Register `PCLKDIS0` reader"]
pub struct R(crate::R<PCLKDIS0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCLKDIS0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCLKDIS0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCLKDIS0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCLKDIS0` writer"]
pub struct W(crate::W<PCLKDIS0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCLKDIS0_SPEC>;
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
impl From<crate::W<PCLKDIS0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCLKDIS0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPIO0` reader - GPIO0 Clock Disable."]
pub type GPIO0_R = crate::BitReader<GPIO0_A>;
#[doc = "GPIO0 Clock Disable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO0_A {
    #[doc = "0: enable it."]
    EN = 0,
    #[doc = "1: disable it."]
    DIS = 1,
}
impl From<GPIO0_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO0_A) -> Self {
        variant as u8 != 0
    }
}
impl GPIO0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO0_A {
        match self.bits {
            false => GPIO0_A::EN,
            true => GPIO0_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == GPIO0_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == GPIO0_A::DIS
    }
}
#[doc = "Field `GPIO0` writer - GPIO0 Clock Disable."]
pub type GPIO0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCLKDIS0_SPEC, GPIO0_A, O>;
impl<'a, const O: u8> GPIO0_W<'a, O> {
    #[doc = "enable it."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(GPIO0_A::EN)
    }
    #[doc = "disable it."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO0_A::DIS)
    }
}
#[doc = "Field `GPIO1` reader - GPIO1 Clock Disable."]
pub type GPIO1_R = crate::BitReader<GPIO1_A>;
#[doc = "GPIO1 Clock Disable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO1_A {
    #[doc = "0: enable it."]
    EN = 0,
    #[doc = "1: disable it."]
    DIS = 1,
}
impl From<GPIO1_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO1_A) -> Self {
        variant as u8 != 0
    }
}
impl GPIO1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO1_A {
        match self.bits {
            false => GPIO1_A::EN,
            true => GPIO1_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == GPIO1_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == GPIO1_A::DIS
    }
}
#[doc = "Field `GPIO1` writer - GPIO1 Clock Disable."]
pub type GPIO1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCLKDIS0_SPEC, GPIO1_A, O>;
impl<'a, const O: u8> GPIO1_W<'a, O> {
    #[doc = "enable it."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(GPIO1_A::EN)
    }
    #[doc = "disable it."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO1_A::DIS)
    }
}
#[doc = "Field `DMA` reader - DMA Clock Disable."]
pub type DMA_R = crate::BitReader<DMA_A>;
#[doc = "DMA Clock Disable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA_A {
    #[doc = "0: enable it."]
    EN = 0,
    #[doc = "1: disable it."]
    DIS = 1,
}
impl From<DMA_A> for bool {
    #[inline(always)]
    fn from(variant: DMA_A) -> Self {
        variant as u8 != 0
    }
}
impl DMA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA_A {
        match self.bits {
            false => DMA_A::EN,
            true => DMA_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == DMA_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == DMA_A::DIS
    }
}
#[doc = "Field `DMA` writer - DMA Clock Disable."]
pub type DMA_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCLKDIS0_SPEC, DMA_A, O>;
impl<'a, const O: u8> DMA_W<'a, O> {
    #[doc = "enable it."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(DMA_A::EN)
    }
    #[doc = "disable it."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(DMA_A::DIS)
    }
}
#[doc = "Field `SPI1` reader - SPI 1 Clock Disable."]
pub type SPI1_R = crate::BitReader<SPI1_A>;
#[doc = "SPI 1 Clock Disable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPI1_A {
    #[doc = "0: enable it."]
    EN = 0,
    #[doc = "1: disable it."]
    DIS = 1,
}
impl From<SPI1_A> for bool {
    #[inline(always)]
    fn from(variant: SPI1_A) -> Self {
        variant as u8 != 0
    }
}
impl SPI1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPI1_A {
        match self.bits {
            false => SPI1_A::EN,
            true => SPI1_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == SPI1_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == SPI1_A::DIS
    }
}
#[doc = "Field `SPI1` writer - SPI 1 Clock Disable."]
pub type SPI1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCLKDIS0_SPEC, SPI1_A, O>;
impl<'a, const O: u8> SPI1_W<'a, O> {
    #[doc = "enable it."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(SPI1_A::EN)
    }
    #[doc = "disable it."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(SPI1_A::DIS)
    }
}
#[doc = "Field `UART0` reader - UART 0 Clock Disable."]
pub type UART0_R = crate::BitReader<UART0_A>;
#[doc = "UART 0 Clock Disable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UART0_A {
    #[doc = "0: enable it."]
    EN = 0,
    #[doc = "1: disable it."]
    DIS = 1,
}
impl From<UART0_A> for bool {
    #[inline(always)]
    fn from(variant: UART0_A) -> Self {
        variant as u8 != 0
    }
}
impl UART0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UART0_A {
        match self.bits {
            false => UART0_A::EN,
            true => UART0_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == UART0_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == UART0_A::DIS
    }
}
#[doc = "Field `UART0` writer - UART 0 Clock Disable."]
pub type UART0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCLKDIS0_SPEC, UART0_A, O>;
impl<'a, const O: u8> UART0_W<'a, O> {
    #[doc = "enable it."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(UART0_A::EN)
    }
    #[doc = "disable it."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(UART0_A::DIS)
    }
}
#[doc = "Field `UART1` reader - UART 1 Clock Disable."]
pub type UART1_R = crate::BitReader<UART1_A>;
#[doc = "UART 1 Clock Disable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UART1_A {
    #[doc = "0: enable it."]
    EN = 0,
    #[doc = "1: disable it."]
    DIS = 1,
}
impl From<UART1_A> for bool {
    #[inline(always)]
    fn from(variant: UART1_A) -> Self {
        variant as u8 != 0
    }
}
impl UART1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UART1_A {
        match self.bits {
            false => UART1_A::EN,
            true => UART1_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == UART1_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == UART1_A::DIS
    }
}
#[doc = "Field `UART1` writer - UART 1 Clock Disable."]
pub type UART1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCLKDIS0_SPEC, UART1_A, O>;
impl<'a, const O: u8> UART1_W<'a, O> {
    #[doc = "enable it."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(UART1_A::EN)
    }
    #[doc = "disable it."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(UART1_A::DIS)
    }
}
#[doc = "Field `I2C0` reader - I2C 0 Clock Disable."]
pub type I2C0_R = crate::BitReader<I2C0_A>;
#[doc = "I2C 0 Clock Disable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C0_A {
    #[doc = "0: enable it."]
    EN = 0,
    #[doc = "1: disable it."]
    DIS = 1,
}
impl From<I2C0_A> for bool {
    #[inline(always)]
    fn from(variant: I2C0_A) -> Self {
        variant as u8 != 0
    }
}
impl I2C0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C0_A {
        match self.bits {
            false => I2C0_A::EN,
            true => I2C0_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == I2C0_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == I2C0_A::DIS
    }
}
#[doc = "Field `I2C0` writer - I2C 0 Clock Disable."]
pub type I2C0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCLKDIS0_SPEC, I2C0_A, O>;
impl<'a, const O: u8> I2C0_W<'a, O> {
    #[doc = "enable it."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(I2C0_A::EN)
    }
    #[doc = "disable it."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(I2C0_A::DIS)
    }
}
#[doc = "Field `TMR0` reader - Timer 0 Clock Disable."]
pub type TMR0_R = crate::BitReader<TMR0_A>;
#[doc = "Timer 0 Clock Disable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TMR0_A {
    #[doc = "0: enable it."]
    EN = 0,
    #[doc = "1: disable it."]
    DIS = 1,
}
impl From<TMR0_A> for bool {
    #[inline(always)]
    fn from(variant: TMR0_A) -> Self {
        variant as u8 != 0
    }
}
impl TMR0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMR0_A {
        match self.bits {
            false => TMR0_A::EN,
            true => TMR0_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == TMR0_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TMR0_A::DIS
    }
}
#[doc = "Field `TMR0` writer - Timer 0 Clock Disable."]
pub type TMR0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCLKDIS0_SPEC, TMR0_A, O>;
impl<'a, const O: u8> TMR0_W<'a, O> {
    #[doc = "enable it."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TMR0_A::EN)
    }
    #[doc = "disable it."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMR0_A::DIS)
    }
}
#[doc = "Field `TMR1` reader - Timer 1 Clock Disable."]
pub type TMR1_R = crate::BitReader<TMR1_A>;
#[doc = "Timer 1 Clock Disable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TMR1_A {
    #[doc = "0: enable it."]
    EN = 0,
    #[doc = "1: disable it."]
    DIS = 1,
}
impl From<TMR1_A> for bool {
    #[inline(always)]
    fn from(variant: TMR1_A) -> Self {
        variant as u8 != 0
    }
}
impl TMR1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMR1_A {
        match self.bits {
            false => TMR1_A::EN,
            true => TMR1_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == TMR1_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TMR1_A::DIS
    }
}
#[doc = "Field `TMR1` writer - Timer 1 Clock Disable."]
pub type TMR1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCLKDIS0_SPEC, TMR1_A, O>;
impl<'a, const O: u8> TMR1_W<'a, O> {
    #[doc = "enable it."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TMR1_A::EN)
    }
    #[doc = "disable it."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMR1_A::DIS)
    }
}
#[doc = "Field `TMR2` reader - Timer 2 Clock Disable."]
pub type TMR2_R = crate::BitReader<TMR2_A>;
#[doc = "Timer 2 Clock Disable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TMR2_A {
    #[doc = "0: enable it."]
    EN = 0,
    #[doc = "1: disable it."]
    DIS = 1,
}
impl From<TMR2_A> for bool {
    #[inline(always)]
    fn from(variant: TMR2_A) -> Self {
        variant as u8 != 0
    }
}
impl TMR2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMR2_A {
        match self.bits {
            false => TMR2_A::EN,
            true => TMR2_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == TMR2_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TMR2_A::DIS
    }
}
#[doc = "Field `TMR2` writer - Timer 2 Clock Disable."]
pub type TMR2_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCLKDIS0_SPEC, TMR2_A, O>;
impl<'a, const O: u8> TMR2_W<'a, O> {
    #[doc = "enable it."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TMR2_A::EN)
    }
    #[doc = "disable it."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMR2_A::DIS)
    }
}
#[doc = "Field `TMR3` reader - Timer 3 Clock Disable."]
pub type TMR3_R = crate::BitReader<TMR3_A>;
#[doc = "Timer 3 Clock Disable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TMR3_A {
    #[doc = "0: enable it."]
    EN = 0,
    #[doc = "1: disable it."]
    DIS = 1,
}
impl From<TMR3_A> for bool {
    #[inline(always)]
    fn from(variant: TMR3_A) -> Self {
        variant as u8 != 0
    }
}
impl TMR3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMR3_A {
        match self.bits {
            false => TMR3_A::EN,
            true => TMR3_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == TMR3_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TMR3_A::DIS
    }
}
#[doc = "Field `TMR3` writer - Timer 3 Clock Disable."]
pub type TMR3_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCLKDIS0_SPEC, TMR3_A, O>;
impl<'a, const O: u8> TMR3_W<'a, O> {
    #[doc = "enable it."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TMR3_A::EN)
    }
    #[doc = "disable it."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMR3_A::DIS)
    }
}
#[doc = "Field `ADC` reader - ADC Clock Disable."]
pub type ADC_R = crate::BitReader<ADC_A>;
#[doc = "ADC Clock Disable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC_A {
    #[doc = "0: enable it."]
    EN = 0,
    #[doc = "1: disable it."]
    DIS = 1,
}
impl From<ADC_A> for bool {
    #[inline(always)]
    fn from(variant: ADC_A) -> Self {
        variant as u8 != 0
    }
}
impl ADC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC_A {
        match self.bits {
            false => ADC_A::EN,
            true => ADC_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == ADC_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == ADC_A::DIS
    }
}
#[doc = "Field `ADC` writer - ADC Clock Disable."]
pub type ADC_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCLKDIS0_SPEC, ADC_A, O>;
impl<'a, const O: u8> ADC_W<'a, O> {
    #[doc = "enable it."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(ADC_A::EN)
    }
    #[doc = "disable it."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(ADC_A::DIS)
    }
}
#[doc = "Field `CNN` reader - CNN Clock Disable."]
pub type CNN_R = crate::BitReader<CNN_A>;
#[doc = "CNN Clock Disable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CNN_A {
    #[doc = "0: enable it."]
    EN = 0,
    #[doc = "1: disable it."]
    DIS = 1,
}
impl From<CNN_A> for bool {
    #[inline(always)]
    fn from(variant: CNN_A) -> Self {
        variant as u8 != 0
    }
}
impl CNN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CNN_A {
        match self.bits {
            false => CNN_A::EN,
            true => CNN_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == CNN_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == CNN_A::DIS
    }
}
#[doc = "Field `CNN` writer - CNN Clock Disable."]
pub type CNN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCLKDIS0_SPEC, CNN_A, O>;
impl<'a, const O: u8> CNN_W<'a, O> {
    #[doc = "enable it."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(CNN_A::EN)
    }
    #[doc = "disable it."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(CNN_A::DIS)
    }
}
#[doc = "Field `I2C1` reader - I2C 1 Clock Disable."]
pub type I2C1_R = crate::BitReader<I2C1_A>;
#[doc = "I2C 1 Clock Disable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C1_A {
    #[doc = "0: enable it."]
    EN = 0,
    #[doc = "1: disable it."]
    DIS = 1,
}
impl From<I2C1_A> for bool {
    #[inline(always)]
    fn from(variant: I2C1_A) -> Self {
        variant as u8 != 0
    }
}
impl I2C1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C1_A {
        match self.bits {
            false => I2C1_A::EN,
            true => I2C1_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == I2C1_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == I2C1_A::DIS
    }
}
#[doc = "Field `I2C1` writer - I2C 1 Clock Disable."]
pub type I2C1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCLKDIS0_SPEC, I2C1_A, O>;
impl<'a, const O: u8> I2C1_W<'a, O> {
    #[doc = "enable it."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(I2C1_A::EN)
    }
    #[doc = "disable it."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(I2C1_A::DIS)
    }
}
#[doc = "Field `PT` reader - Pluse Train Clock Disable."]
pub type PT_R = crate::BitReader<PT_A>;
#[doc = "Pluse Train Clock Disable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PT_A {
    #[doc = "0: enable it."]
    EN = 0,
    #[doc = "1: disable it."]
    DIS = 1,
}
impl From<PT_A> for bool {
    #[inline(always)]
    fn from(variant: PT_A) -> Self {
        variant as u8 != 0
    }
}
impl PT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PT_A {
        match self.bits {
            false => PT_A::EN,
            true => PT_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PT_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PT_A::DIS
    }
}
#[doc = "Field `PT` writer - Pluse Train Clock Disable."]
pub type PT_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCLKDIS0_SPEC, PT_A, O>;
impl<'a, const O: u8> PT_W<'a, O> {
    #[doc = "enable it."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PT_A::EN)
    }
    #[doc = "disable it."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PT_A::DIS)
    }
}
impl R {
    #[doc = "Bit 0 - GPIO0 Clock Disable."]
    #[inline(always)]
    pub fn gpio0(&self) -> GPIO0_R {
        GPIO0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GPIO1 Clock Disable."]
    #[inline(always)]
    pub fn gpio1(&self) -> GPIO1_R {
        GPIO1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 5 - DMA Clock Disable."]
    #[inline(always)]
    pub fn dma(&self) -> DMA_R {
        DMA_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SPI 1 Clock Disable."]
    #[inline(always)]
    pub fn spi1(&self) -> SPI1_R {
        SPI1_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - UART 0 Clock Disable."]
    #[inline(always)]
    pub fn uart0(&self) -> UART0_R {
        UART0_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - UART 1 Clock Disable."]
    #[inline(always)]
    pub fn uart1(&self) -> UART1_R {
        UART1_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 13 - I2C 0 Clock Disable."]
    #[inline(always)]
    pub fn i2c0(&self) -> I2C0_R {
        I2C0_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - Timer 0 Clock Disable."]
    #[inline(always)]
    pub fn tmr0(&self) -> TMR0_R {
        TMR0_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Timer 1 Clock Disable."]
    #[inline(always)]
    pub fn tmr1(&self) -> TMR1_R {
        TMR1_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Timer 2 Clock Disable."]
    #[inline(always)]
    pub fn tmr2(&self) -> TMR2_R {
        TMR2_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Timer 3 Clock Disable."]
    #[inline(always)]
    pub fn tmr3(&self) -> TMR3_R {
        TMR3_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 23 - ADC Clock Disable."]
    #[inline(always)]
    pub fn adc(&self) -> ADC_R {
        ADC_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 25 - CNN Clock Disable."]
    #[inline(always)]
    pub fn cnn(&self) -> CNN_R {
        CNN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 28 - I2C 1 Clock Disable."]
    #[inline(always)]
    pub fn i2c1(&self) -> I2C1_R {
        I2C1_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Pluse Train Clock Disable."]
    #[inline(always)]
    pub fn pt(&self) -> PT_R {
        PT_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPIO0 Clock Disable."]
    #[inline(always)]
    #[must_use]
    pub fn gpio0(&mut self) -> GPIO0_W<0> {
        GPIO0_W::new(self)
    }
    #[doc = "Bit 1 - GPIO1 Clock Disable."]
    #[inline(always)]
    #[must_use]
    pub fn gpio1(&mut self) -> GPIO1_W<1> {
        GPIO1_W::new(self)
    }
    #[doc = "Bit 5 - DMA Clock Disable."]
    #[inline(always)]
    #[must_use]
    pub fn dma(&mut self) -> DMA_W<5> {
        DMA_W::new(self)
    }
    #[doc = "Bit 6 - SPI 1 Clock Disable."]
    #[inline(always)]
    #[must_use]
    pub fn spi1(&mut self) -> SPI1_W<6> {
        SPI1_W::new(self)
    }
    #[doc = "Bit 9 - UART 0 Clock Disable."]
    #[inline(always)]
    #[must_use]
    pub fn uart0(&mut self) -> UART0_W<9> {
        UART0_W::new(self)
    }
    #[doc = "Bit 10 - UART 1 Clock Disable."]
    #[inline(always)]
    #[must_use]
    pub fn uart1(&mut self) -> UART1_W<10> {
        UART1_W::new(self)
    }
    #[doc = "Bit 13 - I2C 0 Clock Disable."]
    #[inline(always)]
    #[must_use]
    pub fn i2c0(&mut self) -> I2C0_W<13> {
        I2C0_W::new(self)
    }
    #[doc = "Bit 15 - Timer 0 Clock Disable."]
    #[inline(always)]
    #[must_use]
    pub fn tmr0(&mut self) -> TMR0_W<15> {
        TMR0_W::new(self)
    }
    #[doc = "Bit 16 - Timer 1 Clock Disable."]
    #[inline(always)]
    #[must_use]
    pub fn tmr1(&mut self) -> TMR1_W<16> {
        TMR1_W::new(self)
    }
    #[doc = "Bit 17 - Timer 2 Clock Disable."]
    #[inline(always)]
    #[must_use]
    pub fn tmr2(&mut self) -> TMR2_W<17> {
        TMR2_W::new(self)
    }
    #[doc = "Bit 18 - Timer 3 Clock Disable."]
    #[inline(always)]
    #[must_use]
    pub fn tmr3(&mut self) -> TMR3_W<18> {
        TMR3_W::new(self)
    }
    #[doc = "Bit 23 - ADC Clock Disable."]
    #[inline(always)]
    #[must_use]
    pub fn adc(&mut self) -> ADC_W<23> {
        ADC_W::new(self)
    }
    #[doc = "Bit 25 - CNN Clock Disable."]
    #[inline(always)]
    #[must_use]
    pub fn cnn(&mut self) -> CNN_W<25> {
        CNN_W::new(self)
    }
    #[doc = "Bit 28 - I2C 1 Clock Disable."]
    #[inline(always)]
    #[must_use]
    pub fn i2c1(&mut self) -> I2C1_W<28> {
        I2C1_W::new(self)
    }
    #[doc = "Bit 29 - Pluse Train Clock Disable."]
    #[inline(always)]
    #[must_use]
    pub fn pt(&mut self) -> PT_W<29> {
        PT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peripheral Clock Disable.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pclkdis0](index.html) module"]
pub struct PCLKDIS0_SPEC;
impl crate::RegisterSpec for PCLKDIS0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pclkdis0::R](R) reader structure"]
impl crate::Readable for PCLKDIS0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pclkdis0::W](W) writer structure"]
impl crate::Writable for PCLKDIS0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PCLKDIS0 to value 0"]
impl crate::Resettable for PCLKDIS0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
