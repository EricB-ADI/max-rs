#[doc = "Register `PCLKDIS1` reader"]
pub struct R(crate::R<PCLKDIS1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCLKDIS1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCLKDIS1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCLKDIS1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCLKDIS1` writer"]
pub struct W(crate::W<PCLKDIS1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCLKDIS1_SPEC>;
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
impl From<crate::W<PCLKDIS1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCLKDIS1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BTLE` reader - Bluetooth Clock Disable."]
pub type BTLE_R = crate::BitReader<bool>;
#[doc = "Field `BTLE` writer - Bluetooth Clock Disable."]
pub type BTLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCLKDIS1_SPEC, bool, O>;
#[doc = "Field `UART2` reader - UART2 Clock Disable."]
pub type UART2_R = crate::BitReader<UART2_A>;
#[doc = "UART2 Clock Disable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UART2_A {
    #[doc = "0: Enable."]
    EN = 0,
    #[doc = "1: Disable."]
    DIS = 1,
}
impl From<UART2_A> for bool {
    #[inline(always)]
    fn from(variant: UART2_A) -> Self {
        variant as u8 != 0
    }
}
impl UART2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UART2_A {
        match self.bits {
            false => UART2_A::EN,
            true => UART2_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == UART2_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == UART2_A::DIS
    }
}
#[doc = "Field `UART2` writer - UART2 Clock Disable."]
pub type UART2_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCLKDIS1_SPEC, UART2_A, O>;
impl<'a, const O: u8> UART2_W<'a, O> {
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(UART2_A::EN)
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(UART2_A::DIS)
    }
}
#[doc = "Field `TRNG` reader - TRNG Clock Disable."]
pub type TRNG_R = crate::BitReader<TRNG_A>;
#[doc = "TRNG Clock Disable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRNG_A {
    #[doc = "0: Enable."]
    EN = 0,
    #[doc = "1: Disable."]
    DIS = 1,
}
impl From<TRNG_A> for bool {
    #[inline(always)]
    fn from(variant: TRNG_A) -> Self {
        variant as u8 != 0
    }
}
impl TRNG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRNG_A {
        match self.bits {
            false => TRNG_A::EN,
            true => TRNG_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == TRNG_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TRNG_A::DIS
    }
}
#[doc = "Field `TRNG` writer - TRNG Clock Disable."]
pub type TRNG_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCLKDIS1_SPEC, TRNG_A, O>;
impl<'a, const O: u8> TRNG_W<'a, O> {
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TRNG_A::EN)
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TRNG_A::DIS)
    }
}
#[doc = "Field `SMPHR` reader - SMPHR Clock Disable."]
pub type SMPHR_R = crate::BitReader<SMPHR_A>;
#[doc = "SMPHR Clock Disable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMPHR_A {
    #[doc = "0: Enable."]
    EN = 0,
    #[doc = "1: Disable."]
    DIS = 1,
}
impl From<SMPHR_A> for bool {
    #[inline(always)]
    fn from(variant: SMPHR_A) -> Self {
        variant as u8 != 0
    }
}
impl SMPHR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMPHR_A {
        match self.bits {
            false => SMPHR_A::EN,
            true => SMPHR_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == SMPHR_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == SMPHR_A::DIS
    }
}
#[doc = "Field `SMPHR` writer - SMPHR Clock Disable."]
pub type SMPHR_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCLKDIS1_SPEC, SMPHR_A, O>;
impl<'a, const O: u8> SMPHR_W<'a, O> {
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(SMPHR_A::EN)
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(SMPHR_A::DIS)
    }
}
#[doc = "Field `OWM` reader - One-Wire Clock Disable."]
pub type OWM_R = crate::BitReader<OWM_A>;
#[doc = "One-Wire Clock Disable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OWM_A {
    #[doc = "0: Enable."]
    EN = 0,
    #[doc = "1: Disable."]
    DIS = 1,
}
impl From<OWM_A> for bool {
    #[inline(always)]
    fn from(variant: OWM_A) -> Self {
        variant as u8 != 0
    }
}
impl OWM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OWM_A {
        match self.bits {
            false => OWM_A::EN,
            true => OWM_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == OWM_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == OWM_A::DIS
    }
}
#[doc = "Field `OWM` writer - One-Wire Clock Disable."]
pub type OWM_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCLKDIS1_SPEC, OWM_A, O>;
impl<'a, const O: u8> OWM_W<'a, O> {
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(OWM_A::EN)
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(OWM_A::DIS)
    }
}
#[doc = "Field `CRC` reader - CRC Clock Disable."]
pub type CRC_R = crate::BitReader<CRC_A>;
#[doc = "CRC Clock Disable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRC_A {
    #[doc = "0: Enable."]
    EN = 0,
    #[doc = "1: Disable."]
    DIS = 1,
}
impl From<CRC_A> for bool {
    #[inline(always)]
    fn from(variant: CRC_A) -> Self {
        variant as u8 != 0
    }
}
impl CRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRC_A {
        match self.bits {
            false => CRC_A::EN,
            true => CRC_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == CRC_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == CRC_A::DIS
    }
}
#[doc = "Field `CRC` writer - CRC Clock Disable."]
pub type CRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCLKDIS1_SPEC, CRC_A, O>;
impl<'a, const O: u8> CRC_W<'a, O> {
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(CRC_A::EN)
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(CRC_A::DIS)
    }
}
#[doc = "Field `AES` reader - AES Clock Disable."]
pub type AES_R = crate::BitReader<AES_A>;
#[doc = "AES Clock Disable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AES_A {
    #[doc = "0: Enable."]
    EN = 0,
    #[doc = "1: Disable."]
    DIS = 1,
}
impl From<AES_A> for bool {
    #[inline(always)]
    fn from(variant: AES_A) -> Self {
        variant as u8 != 0
    }
}
impl AES_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AES_A {
        match self.bits {
            false => AES_A::EN,
            true => AES_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == AES_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == AES_A::DIS
    }
}
#[doc = "Field `AES` writer - AES Clock Disable."]
pub type AES_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCLKDIS1_SPEC, AES_A, O>;
impl<'a, const O: u8> AES_W<'a, O> {
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(AES_A::EN)
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(AES_A::DIS)
    }
}
#[doc = "Field `SPI0` reader - SPI 0 Clock Disable."]
pub type SPI0_R = crate::BitReader<SPI0_A>;
#[doc = "SPI 0 Clock Disable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPI0_A {
    #[doc = "0: Enable."]
    EN = 0,
    #[doc = "1: Disable."]
    DIS = 1,
}
impl From<SPI0_A> for bool {
    #[inline(always)]
    fn from(variant: SPI0_A) -> Self {
        variant as u8 != 0
    }
}
impl SPI0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPI0_A {
        match self.bits {
            false => SPI0_A::EN,
            true => SPI0_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == SPI0_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == SPI0_A::DIS
    }
}
#[doc = "Field `SPI0` writer - SPI 0 Clock Disable."]
pub type SPI0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCLKDIS1_SPEC, SPI0_A, O>;
impl<'a, const O: u8> SPI0_W<'a, O> {
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(SPI0_A::EN)
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(SPI0_A::DIS)
    }
}
#[doc = "Field `PCIF` reader - Parallel Camera Interface Clock Disable."]
pub type PCIF_R = crate::BitReader<PCIF_A>;
#[doc = "Parallel Camera Interface Clock Disable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PCIF_A {
    #[doc = "0: Enable."]
    EN = 0,
    #[doc = "1: Disable."]
    DIS = 1,
}
impl From<PCIF_A> for bool {
    #[inline(always)]
    fn from(variant: PCIF_A) -> Self {
        variant as u8 != 0
    }
}
impl PCIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCIF_A {
        match self.bits {
            false => PCIF_A::EN,
            true => PCIF_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PCIF_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PCIF_A::DIS
    }
}
#[doc = "Field `PCIF` writer - Parallel Camera Interface Clock Disable."]
pub type PCIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCLKDIS1_SPEC, PCIF_A, O>;
impl<'a, const O: u8> PCIF_W<'a, O> {
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PCIF_A::EN)
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PCIF_A::DIS)
    }
}
#[doc = "Field `I2S` reader - I2S Clock Disable."]
pub type I2S_R = crate::BitReader<I2S_A>;
#[doc = "I2S Clock Disable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2S_A {
    #[doc = "0: Enable."]
    EN = 0,
    #[doc = "1: Disable."]
    DIS = 1,
}
impl From<I2S_A> for bool {
    #[inline(always)]
    fn from(variant: I2S_A) -> Self {
        variant as u8 != 0
    }
}
impl I2S_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2S_A {
        match self.bits {
            false => I2S_A::EN,
            true => I2S_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == I2S_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == I2S_A::DIS
    }
}
#[doc = "Field `I2S` writer - I2S Clock Disable."]
pub type I2S_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCLKDIS1_SPEC, I2S_A, O>;
impl<'a, const O: u8> I2S_W<'a, O> {
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(I2S_A::EN)
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(I2S_A::DIS)
    }
}
#[doc = "Field `I2C2` reader - I2C2 Clock Disable."]
pub type I2C2_R = crate::BitReader<I2C2_A>;
#[doc = "I2C2 Clock Disable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C2_A {
    #[doc = "0: Enable."]
    EN = 0,
    #[doc = "1: Disable."]
    DIS = 1,
}
impl From<I2C2_A> for bool {
    #[inline(always)]
    fn from(variant: I2C2_A) -> Self {
        variant as u8 != 0
    }
}
impl I2C2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C2_A {
        match self.bits {
            false => I2C2_A::EN,
            true => I2C2_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == I2C2_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == I2C2_A::DIS
    }
}
#[doc = "Field `I2C2` writer - I2C2 Clock Disable."]
pub type I2C2_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCLKDIS1_SPEC, I2C2_A, O>;
impl<'a, const O: u8> I2C2_W<'a, O> {
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(I2C2_A::EN)
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(I2C2_A::DIS)
    }
}
#[doc = "Field `WDT0` reader - Watch Dog Timer 0 Clock Disable."]
pub type WDT0_R = crate::BitReader<WDT0_A>;
#[doc = "Watch Dog Timer 0 Clock Disable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WDT0_A {
    #[doc = "0: Enable."]
    EN = 0,
    #[doc = "1: Disable."]
    DIS = 1,
}
impl From<WDT0_A> for bool {
    #[inline(always)]
    fn from(variant: WDT0_A) -> Self {
        variant as u8 != 0
    }
}
impl WDT0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDT0_A {
        match self.bits {
            false => WDT0_A::EN,
            true => WDT0_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == WDT0_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == WDT0_A::DIS
    }
}
#[doc = "Field `WDT0` writer - Watch Dog Timer 0 Clock Disable."]
pub type WDT0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCLKDIS1_SPEC, WDT0_A, O>;
impl<'a, const O: u8> WDT0_W<'a, O> {
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(WDT0_A::EN)
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(WDT0_A::DIS)
    }
}
#[doc = "Field `CPU1` reader - CPU1 Clock Disable."]
pub type CPU1_R = crate::BitReader<CPU1_A>;
#[doc = "CPU1 Clock Disable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPU1_A {
    #[doc = "0: Enable."]
    EN = 0,
    #[doc = "1: Disable."]
    DIS = 1,
}
impl From<CPU1_A> for bool {
    #[inline(always)]
    fn from(variant: CPU1_A) -> Self {
        variant as u8 != 0
    }
}
impl CPU1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPU1_A {
        match self.bits {
            false => CPU1_A::EN,
            true => CPU1_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == CPU1_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == CPU1_A::DIS
    }
}
#[doc = "Field `CPU1` writer - CPU1 Clock Disable."]
pub type CPU1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCLKDIS1_SPEC, CPU1_A, O>;
impl<'a, const O: u8> CPU1_W<'a, O> {
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(CPU1_A::EN)
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(CPU1_A::DIS)
    }
}
impl R {
    #[doc = "Bit 0 - Bluetooth Clock Disable."]
    #[inline(always)]
    pub fn btle(&self) -> BTLE_R {
        BTLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - UART2 Clock Disable."]
    #[inline(always)]
    pub fn uart2(&self) -> UART2_R {
        UART2_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TRNG Clock Disable."]
    #[inline(always)]
    pub fn trng(&self) -> TRNG_R {
        TRNG_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 9 - SMPHR Clock Disable."]
    #[inline(always)]
    pub fn smphr(&self) -> SMPHR_R {
        SMPHR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 13 - One-Wire Clock Disable."]
    #[inline(always)]
    pub fn owm(&self) -> OWM_R {
        OWM_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - CRC Clock Disable."]
    #[inline(always)]
    pub fn crc(&self) -> CRC_R {
        CRC_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - AES Clock Disable."]
    #[inline(always)]
    pub fn aes(&self) -> AES_R {
        AES_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - SPI 0 Clock Disable."]
    #[inline(always)]
    pub fn spi0(&self) -> SPI0_R {
        SPI0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - Parallel Camera Interface Clock Disable."]
    #[inline(always)]
    pub fn pcif(&self) -> PCIF_R {
        PCIF_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 23 - I2S Clock Disable."]
    #[inline(always)]
    pub fn i2s(&self) -> I2S_R {
        I2S_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - I2C2 Clock Disable."]
    #[inline(always)]
    pub fn i2c2(&self) -> I2C2_R {
        I2C2_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 27 - Watch Dog Timer 0 Clock Disable."]
    #[inline(always)]
    pub fn wdt0(&self) -> WDT0_R {
        WDT0_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 31 - CPU1 Clock Disable."]
    #[inline(always)]
    pub fn cpu1(&self) -> CPU1_R {
        CPU1_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Bluetooth Clock Disable."]
    #[inline(always)]
    #[must_use]
    pub fn btle(&mut self) -> BTLE_W<0> {
        BTLE_W::new(self)
    }
    #[doc = "Bit 1 - UART2 Clock Disable."]
    #[inline(always)]
    #[must_use]
    pub fn uart2(&mut self) -> UART2_W<1> {
        UART2_W::new(self)
    }
    #[doc = "Bit 2 - TRNG Clock Disable."]
    #[inline(always)]
    #[must_use]
    pub fn trng(&mut self) -> TRNG_W<2> {
        TRNG_W::new(self)
    }
    #[doc = "Bit 9 - SMPHR Clock Disable."]
    #[inline(always)]
    #[must_use]
    pub fn smphr(&mut self) -> SMPHR_W<9> {
        SMPHR_W::new(self)
    }
    #[doc = "Bit 13 - One-Wire Clock Disable."]
    #[inline(always)]
    #[must_use]
    pub fn owm(&mut self) -> OWM_W<13> {
        OWM_W::new(self)
    }
    #[doc = "Bit 14 - CRC Clock Disable."]
    #[inline(always)]
    #[must_use]
    pub fn crc(&mut self) -> CRC_W<14> {
        CRC_W::new(self)
    }
    #[doc = "Bit 15 - AES Clock Disable."]
    #[inline(always)]
    #[must_use]
    pub fn aes(&mut self) -> AES_W<15> {
        AES_W::new(self)
    }
    #[doc = "Bit 16 - SPI 0 Clock Disable."]
    #[inline(always)]
    #[must_use]
    pub fn spi0(&mut self) -> SPI0_W<16> {
        SPI0_W::new(self)
    }
    #[doc = "Bit 18 - Parallel Camera Interface Clock Disable."]
    #[inline(always)]
    #[must_use]
    pub fn pcif(&mut self) -> PCIF_W<18> {
        PCIF_W::new(self)
    }
    #[doc = "Bit 23 - I2S Clock Disable."]
    #[inline(always)]
    #[must_use]
    pub fn i2s(&mut self) -> I2S_W<23> {
        I2S_W::new(self)
    }
    #[doc = "Bit 24 - I2C2 Clock Disable."]
    #[inline(always)]
    #[must_use]
    pub fn i2c2(&mut self) -> I2C2_W<24> {
        I2C2_W::new(self)
    }
    #[doc = "Bit 27 - Watch Dog Timer 0 Clock Disable."]
    #[inline(always)]
    #[must_use]
    pub fn wdt0(&mut self) -> WDT0_W<27> {
        WDT0_W::new(self)
    }
    #[doc = "Bit 31 - CPU1 Clock Disable."]
    #[inline(always)]
    #[must_use]
    pub fn cpu1(&mut self) -> CPU1_W<31> {
        CPU1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peripheral Clock Disable.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pclkdis1](index.html) module"]
pub struct PCLKDIS1_SPEC;
impl crate::RegisterSpec for PCLKDIS1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pclkdis1::R](R) reader structure"]
impl crate::Readable for PCLKDIS1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pclkdis1::W](W) writer structure"]
impl crate::Writable for PCLKDIS1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PCLKDIS1 to value 0"]
impl crate::Resettable for PCLKDIS1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
