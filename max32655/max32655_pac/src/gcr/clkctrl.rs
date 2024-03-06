#[doc = "Register `CLKCTRL` reader"]
pub struct R(crate::R<CLKCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLKCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLKCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLKCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLKCTRL` writer"]
pub struct W(crate::W<CLKCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLKCTRL_SPEC>;
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
impl From<crate::W<CLKCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLKCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSCLK_DIV` reader - Prescaler Select. This 3 bit field sets the system operating frequency by controlling the prescaler that divides the output of the PLL0."]
pub type SYSCLK_DIV_R = crate::FieldReader<u8, SYSCLK_DIV_A>;
#[doc = "Prescaler Select. This 3 bit field sets the system operating frequency by controlling the prescaler that divides the output of the PLL0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SYSCLK_DIV_A {
    #[doc = "0: Divide by 1."]
    DIV1 = 0,
    #[doc = "1: Divide by 2."]
    DIV2 = 1,
    #[doc = "2: Divide by 4."]
    DIV4 = 2,
    #[doc = "3: Divide by 8."]
    DIV8 = 3,
    #[doc = "4: Divide by 16."]
    DIV16 = 4,
    #[doc = "5: Divide by 32."]
    DIV32 = 5,
    #[doc = "6: Divide by 64."]
    DIV64 = 6,
    #[doc = "7: Divide by 128."]
    DIV128 = 7,
}
impl From<SYSCLK_DIV_A> for u8 {
    #[inline(always)]
    fn from(variant: SYSCLK_DIV_A) -> Self {
        variant as _
    }
}
impl SYSCLK_DIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYSCLK_DIV_A {
        match self.bits {
            0 => SYSCLK_DIV_A::DIV1,
            1 => SYSCLK_DIV_A::DIV2,
            2 => SYSCLK_DIV_A::DIV4,
            3 => SYSCLK_DIV_A::DIV8,
            4 => SYSCLK_DIV_A::DIV16,
            5 => SYSCLK_DIV_A::DIV32,
            6 => SYSCLK_DIV_A::DIV64,
            7 => SYSCLK_DIV_A::DIV128,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == SYSCLK_DIV_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == SYSCLK_DIV_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == SYSCLK_DIV_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == SYSCLK_DIV_A::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == SYSCLK_DIV_A::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV32`"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == SYSCLK_DIV_A::DIV32
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == SYSCLK_DIV_A::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV128`"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == SYSCLK_DIV_A::DIV128
    }
}
#[doc = "Field `SYSCLK_DIV` writer - Prescaler Select. This 3 bit field sets the system operating frequency by controlling the prescaler that divides the output of the PLL0."]
pub type SYSCLK_DIV_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CLKCTRL_SPEC, u8, SYSCLK_DIV_A, 3, O>;
impl<'a, const O: u8> SYSCLK_DIV_W<'a, O> {
    #[doc = "Divide by 1."]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(SYSCLK_DIV_A::DIV1)
    }
    #[doc = "Divide by 2."]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(SYSCLK_DIV_A::DIV2)
    }
    #[doc = "Divide by 4."]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(SYSCLK_DIV_A::DIV4)
    }
    #[doc = "Divide by 8."]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(SYSCLK_DIV_A::DIV8)
    }
    #[doc = "Divide by 16."]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(SYSCLK_DIV_A::DIV16)
    }
    #[doc = "Divide by 32."]
    #[inline(always)]
    pub fn div32(self) -> &'a mut W {
        self.variant(SYSCLK_DIV_A::DIV32)
    }
    #[doc = "Divide by 64."]
    #[inline(always)]
    pub fn div64(self) -> &'a mut W {
        self.variant(SYSCLK_DIV_A::DIV64)
    }
    #[doc = "Divide by 128."]
    #[inline(always)]
    pub fn div128(self) -> &'a mut W {
        self.variant(SYSCLK_DIV_A::DIV128)
    }
}
#[doc = "Field `SYSCLK_SEL` reader - Clock Source Select. This 3 bit field selects the source for the system clock."]
pub type SYSCLK_SEL_R = crate::FieldReader<u8, SYSCLK_SEL_A>;
#[doc = "Clock Source Select. This 3 bit field selects the source for the system clock.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SYSCLK_SEL_A {
    #[doc = "0: The internal 60 MHz oscillator is used for the system clock."]
    ISO = 0,
    #[doc = "2: 32MHz Crystal is used for the system clock."]
    ERFO = 2,
    #[doc = "3: 8 kHz LIRC is used for the system clock."]
    INRO = 3,
    #[doc = "4: The internal 96 MHz oscillator is used for the system clock."]
    IPO = 4,
    #[doc = "5: The internal 7.3725 MHz oscillator is used for the system clock."]
    IBRO = 5,
    #[doc = "6: 32 kHz is used for the system clock."]
    ERTCO = 6,
    #[doc = "7: External clock on GPIO0.30."]
    EXTCLK = 7,
}
impl From<SYSCLK_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SYSCLK_SEL_A) -> Self {
        variant as _
    }
}
impl SYSCLK_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SYSCLK_SEL_A> {
        match self.bits {
            0 => Some(SYSCLK_SEL_A::ISO),
            2 => Some(SYSCLK_SEL_A::ERFO),
            3 => Some(SYSCLK_SEL_A::INRO),
            4 => Some(SYSCLK_SEL_A::IPO),
            5 => Some(SYSCLK_SEL_A::IBRO),
            6 => Some(SYSCLK_SEL_A::ERTCO),
            7 => Some(SYSCLK_SEL_A::EXTCLK),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ISO`"]
    #[inline(always)]
    pub fn is_iso(&self) -> bool {
        *self == SYSCLK_SEL_A::ISO
    }
    #[doc = "Checks if the value of the field is `ERFO`"]
    #[inline(always)]
    pub fn is_erfo(&self) -> bool {
        *self == SYSCLK_SEL_A::ERFO
    }
    #[doc = "Checks if the value of the field is `INRO`"]
    #[inline(always)]
    pub fn is_inro(&self) -> bool {
        *self == SYSCLK_SEL_A::INRO
    }
    #[doc = "Checks if the value of the field is `IPO`"]
    #[inline(always)]
    pub fn is_ipo(&self) -> bool {
        *self == SYSCLK_SEL_A::IPO
    }
    #[doc = "Checks if the value of the field is `IBRO`"]
    #[inline(always)]
    pub fn is_ibro(&self) -> bool {
        *self == SYSCLK_SEL_A::IBRO
    }
    #[doc = "Checks if the value of the field is `ERTCO`"]
    #[inline(always)]
    pub fn is_ertco(&self) -> bool {
        *self == SYSCLK_SEL_A::ERTCO
    }
    #[doc = "Checks if the value of the field is `EXTCLK`"]
    #[inline(always)]
    pub fn is_extclk(&self) -> bool {
        *self == SYSCLK_SEL_A::EXTCLK
    }
}
#[doc = "Field `SYSCLK_SEL` writer - Clock Source Select. This 3 bit field selects the source for the system clock."]
pub type SYSCLK_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CLKCTRL_SPEC, u8, SYSCLK_SEL_A, 3, O>;
impl<'a, const O: u8> SYSCLK_SEL_W<'a, O> {
    #[doc = "The internal 60 MHz oscillator is used for the system clock."]
    #[inline(always)]
    pub fn iso(self) -> &'a mut W {
        self.variant(SYSCLK_SEL_A::ISO)
    }
    #[doc = "32MHz Crystal is used for the system clock."]
    #[inline(always)]
    pub fn erfo(self) -> &'a mut W {
        self.variant(SYSCLK_SEL_A::ERFO)
    }
    #[doc = "8 kHz LIRC is used for the system clock."]
    #[inline(always)]
    pub fn inro(self) -> &'a mut W {
        self.variant(SYSCLK_SEL_A::INRO)
    }
    #[doc = "The internal 96 MHz oscillator is used for the system clock."]
    #[inline(always)]
    pub fn ipo(self) -> &'a mut W {
        self.variant(SYSCLK_SEL_A::IPO)
    }
    #[doc = "The internal 7.3725 MHz oscillator is used for the system clock."]
    #[inline(always)]
    pub fn ibro(self) -> &'a mut W {
        self.variant(SYSCLK_SEL_A::IBRO)
    }
    #[doc = "32 kHz is used for the system clock."]
    #[inline(always)]
    pub fn ertco(self) -> &'a mut W {
        self.variant(SYSCLK_SEL_A::ERTCO)
    }
    #[doc = "External clock on GPIO0.30."]
    #[inline(always)]
    pub fn extclk(self) -> &'a mut W {
        self.variant(SYSCLK_SEL_A::EXTCLK)
    }
}
#[doc = "Field `SYSCLK_RDY` reader - Clock Ready. This read only bit reflects whether the currently selected system clock source is running."]
pub type SYSCLK_RDY_R = crate::BitReader<SYSCLK_RDY_A>;
#[doc = "Clock Ready. This read only bit reflects whether the currently selected system clock source is running.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYSCLK_RDY_A {
    #[doc = "0: Switchover to the new clock source (as selected by CLKSEL) has not yet occurred."]
    BUSY = 0,
    #[doc = "1: System clock running from CLKSEL clock source."]
    READY = 1,
}
impl From<SYSCLK_RDY_A> for bool {
    #[inline(always)]
    fn from(variant: SYSCLK_RDY_A) -> Self {
        variant as u8 != 0
    }
}
impl SYSCLK_RDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYSCLK_RDY_A {
        match self.bits {
            false => SYSCLK_RDY_A::BUSY,
            true => SYSCLK_RDY_A::READY,
        }
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == SYSCLK_RDY_A::BUSY
    }
    #[doc = "Checks if the value of the field is `READY`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == SYSCLK_RDY_A::READY
    }
}
#[doc = "Field `ERFO_EN` reader - 32MHz Crystal Oscillator Enable."]
pub type ERFO_EN_R = crate::BitReader<ERFO_EN_A>;
#[doc = "32MHz Crystal Oscillator Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERFO_EN_A {
    #[doc = "0: Is Disabled."]
    DIS = 0,
    #[doc = "1: Is Enabled."]
    EN = 1,
}
impl From<ERFO_EN_A> for bool {
    #[inline(always)]
    fn from(variant: ERFO_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl ERFO_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERFO_EN_A {
        match self.bits {
            false => ERFO_EN_A::DIS,
            true => ERFO_EN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == ERFO_EN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == ERFO_EN_A::EN
    }
}
#[doc = "Field `ERFO_EN` writer - 32MHz Crystal Oscillator Enable."]
pub type ERFO_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKCTRL_SPEC, ERFO_EN_A, O>;
impl<'a, const O: u8> ERFO_EN_W<'a, O> {
    #[doc = "Is Disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(ERFO_EN_A::DIS)
    }
    #[doc = "Is Enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(ERFO_EN_A::EN)
    }
}
#[doc = "Field `ERTCO_EN` reader - 32 kHz Crystal Oscillator Enable."]
pub type ERTCO_EN_R = crate::BitReader<ERTCO_EN_A>;
#[doc = "32 kHz Crystal Oscillator Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERTCO_EN_A {
    #[doc = "0: Is Disabled."]
    DIS = 0,
    #[doc = "1: Is Enabled."]
    EN = 1,
}
impl From<ERTCO_EN_A> for bool {
    #[inline(always)]
    fn from(variant: ERTCO_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl ERTCO_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERTCO_EN_A {
        match self.bits {
            false => ERTCO_EN_A::DIS,
            true => ERTCO_EN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == ERTCO_EN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == ERTCO_EN_A::EN
    }
}
#[doc = "Field `ERTCO_EN` writer - 32 kHz Crystal Oscillator Enable."]
pub type ERTCO_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKCTRL_SPEC, ERTCO_EN_A, O>;
impl<'a, const O: u8> ERTCO_EN_W<'a, O> {
    #[doc = "Is Disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(ERTCO_EN_A::DIS)
    }
    #[doc = "Is Enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(ERTCO_EN_A::EN)
    }
}
#[doc = "Field `ISO_EN` reader - 60 MHz High Frequency Internal Reference Clock Enable."]
pub type ISO_EN_R = crate::BitReader<ISO_EN_A>;
#[doc = "60 MHz High Frequency Internal Reference Clock Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ISO_EN_A {
    #[doc = "0: Is Disabled."]
    DIS = 0,
    #[doc = "1: Is Enabled."]
    EN = 1,
}
impl From<ISO_EN_A> for bool {
    #[inline(always)]
    fn from(variant: ISO_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl ISO_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISO_EN_A {
        match self.bits {
            false => ISO_EN_A::DIS,
            true => ISO_EN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == ISO_EN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == ISO_EN_A::EN
    }
}
#[doc = "Field `ISO_EN` writer - 60 MHz High Frequency Internal Reference Clock Enable."]
pub type ISO_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKCTRL_SPEC, ISO_EN_A, O>;
impl<'a, const O: u8> ISO_EN_W<'a, O> {
    #[doc = "Is Disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(ISO_EN_A::DIS)
    }
    #[doc = "Is Enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(ISO_EN_A::EN)
    }
}
#[doc = "Field `IPO_EN` reader - 100 MHz High Frequency Internal Reference Clock Enable."]
pub type IPO_EN_R = crate::BitReader<IPO_EN_A>;
#[doc = "100 MHz High Frequency Internal Reference Clock Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IPO_EN_A {
    #[doc = "0: Is Disabled."]
    DIS = 0,
    #[doc = "1: Is Enabled."]
    EN = 1,
}
impl From<IPO_EN_A> for bool {
    #[inline(always)]
    fn from(variant: IPO_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl IPO_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IPO_EN_A {
        match self.bits {
            false => IPO_EN_A::DIS,
            true => IPO_EN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == IPO_EN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == IPO_EN_A::EN
    }
}
#[doc = "Field `IPO_EN` writer - 100 MHz High Frequency Internal Reference Clock Enable."]
pub type IPO_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKCTRL_SPEC, IPO_EN_A, O>;
impl<'a, const O: u8> IPO_EN_W<'a, O> {
    #[doc = "Is Disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(IPO_EN_A::DIS)
    }
    #[doc = "Is Enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(IPO_EN_A::EN)
    }
}
#[doc = "Field `IBRO_EN` reader - 7.3725 MHz High Frequency Internal Reference Clock Enable."]
pub type IBRO_EN_R = crate::BitReader<IBRO_EN_A>;
#[doc = "7.3725 MHz High Frequency Internal Reference Clock Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IBRO_EN_A {
    #[doc = "0: Is Disabled."]
    DIS = 0,
    #[doc = "1: Is Enabled."]
    EN = 1,
}
impl From<IBRO_EN_A> for bool {
    #[inline(always)]
    fn from(variant: IBRO_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl IBRO_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IBRO_EN_A {
        match self.bits {
            false => IBRO_EN_A::DIS,
            true => IBRO_EN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == IBRO_EN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == IBRO_EN_A::EN
    }
}
#[doc = "Field `IBRO_EN` writer - 7.3725 MHz High Frequency Internal Reference Clock Enable."]
pub type IBRO_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKCTRL_SPEC, IBRO_EN_A, O>;
impl<'a, const O: u8> IBRO_EN_W<'a, O> {
    #[doc = "Is Disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(IBRO_EN_A::DIS)
    }
    #[doc = "Is Enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(IBRO_EN_A::EN)
    }
}
#[doc = "Field `IBRO_VS` reader - 7.3725 MHz High Frequency Internal Reference Clock Voltage Select. This register bit is used to select the power supply to the IBRO."]
pub type IBRO_VS_R = crate::BitReader<IBRO_VS_A>;
#[doc = "7.3725 MHz High Frequency Internal Reference Clock Voltage Select. This register bit is used to select the power supply to the IBRO.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IBRO_VS_A {
    #[doc = "0: VCore Supply"]
    VCOR = 0,
    #[doc = "1: Dedicated 1V regulated supply."]
    _1V = 1,
}
impl From<IBRO_VS_A> for bool {
    #[inline(always)]
    fn from(variant: IBRO_VS_A) -> Self {
        variant as u8 != 0
    }
}
impl IBRO_VS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IBRO_VS_A {
        match self.bits {
            false => IBRO_VS_A::VCOR,
            true => IBRO_VS_A::_1V,
        }
    }
    #[doc = "Checks if the value of the field is `VCOR`"]
    #[inline(always)]
    pub fn is_vcor(&self) -> bool {
        *self == IBRO_VS_A::VCOR
    }
    #[doc = "Checks if the value of the field is `_1V`"]
    #[inline(always)]
    pub fn is_1v(&self) -> bool {
        *self == IBRO_VS_A::_1V
    }
}
#[doc = "Field `IBRO_VS` writer - 7.3725 MHz High Frequency Internal Reference Clock Voltage Select. This register bit is used to select the power supply to the IBRO."]
pub type IBRO_VS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKCTRL_SPEC, IBRO_VS_A, O>;
impl<'a, const O: u8> IBRO_VS_W<'a, O> {
    #[doc = "VCore Supply"]
    #[inline(always)]
    pub fn vcor(self) -> &'a mut W {
        self.variant(IBRO_VS_A::VCOR)
    }
    #[doc = "Dedicated 1V regulated supply."]
    #[inline(always)]
    pub fn _1v(self) -> &'a mut W {
        self.variant(IBRO_VS_A::_1V)
    }
}
#[doc = "Field `ERFO_RDY` reader - 32MHz Crystal Oscillator Ready"]
pub type ERFO_RDY_R = crate::BitReader<ERFO_RDY_A>;
#[doc = "32MHz Crystal Oscillator Ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERFO_RDY_A {
    #[doc = "0: Is not Ready."]
    NOT = 0,
    #[doc = "1: Is Ready."]
    READY = 1,
}
impl From<ERFO_RDY_A> for bool {
    #[inline(always)]
    fn from(variant: ERFO_RDY_A) -> Self {
        variant as u8 != 0
    }
}
impl ERFO_RDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERFO_RDY_A {
        match self.bits {
            false => ERFO_RDY_A::NOT,
            true => ERFO_RDY_A::READY,
        }
    }
    #[doc = "Checks if the value of the field is `NOT`"]
    #[inline(always)]
    pub fn is_not(&self) -> bool {
        *self == ERFO_RDY_A::NOT
    }
    #[doc = "Checks if the value of the field is `READY`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == ERFO_RDY_A::READY
    }
}
#[doc = "Field `ERTCO_RDY` reader - 32 kHz Crystal Oscillator Ready"]
pub type ERTCO_RDY_R = crate::BitReader<ERTCO_RDY_A>;
#[doc = "32 kHz Crystal Oscillator Ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERTCO_RDY_A {
    #[doc = "0: Is not Ready."]
    NOT = 0,
    #[doc = "1: Is Ready."]
    READY = 1,
}
impl From<ERTCO_RDY_A> for bool {
    #[inline(always)]
    fn from(variant: ERTCO_RDY_A) -> Self {
        variant as u8 != 0
    }
}
impl ERTCO_RDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERTCO_RDY_A {
        match self.bits {
            false => ERTCO_RDY_A::NOT,
            true => ERTCO_RDY_A::READY,
        }
    }
    #[doc = "Checks if the value of the field is `NOT`"]
    #[inline(always)]
    pub fn is_not(&self) -> bool {
        *self == ERTCO_RDY_A::NOT
    }
    #[doc = "Checks if the value of the field is `READY`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == ERTCO_RDY_A::READY
    }
}
#[doc = "Field `ISO_RDY` reader - 60 MHz HIRC Ready."]
pub type ISO_RDY_R = crate::BitReader<ISO_RDY_A>;
#[doc = "60 MHz HIRC Ready.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ISO_RDY_A {
    #[doc = "0: Is not Ready."]
    NOT = 0,
    #[doc = "1: Is Ready."]
    READY = 1,
}
impl From<ISO_RDY_A> for bool {
    #[inline(always)]
    fn from(variant: ISO_RDY_A) -> Self {
        variant as u8 != 0
    }
}
impl ISO_RDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISO_RDY_A {
        match self.bits {
            false => ISO_RDY_A::NOT,
            true => ISO_RDY_A::READY,
        }
    }
    #[doc = "Checks if the value of the field is `NOT`"]
    #[inline(always)]
    pub fn is_not(&self) -> bool {
        *self == ISO_RDY_A::NOT
    }
    #[doc = "Checks if the value of the field is `READY`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == ISO_RDY_A::READY
    }
}
#[doc = "Field `IPO_RDY` reader - 100 MHz HIRC Ready."]
pub type IPO_RDY_R = crate::BitReader<IPO_RDY_A>;
#[doc = "100 MHz HIRC Ready.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IPO_RDY_A {
    #[doc = "0: Is not Ready."]
    NOT = 0,
    #[doc = "1: Is Ready."]
    READY = 1,
}
impl From<IPO_RDY_A> for bool {
    #[inline(always)]
    fn from(variant: IPO_RDY_A) -> Self {
        variant as u8 != 0
    }
}
impl IPO_RDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IPO_RDY_A {
        match self.bits {
            false => IPO_RDY_A::NOT,
            true => IPO_RDY_A::READY,
        }
    }
    #[doc = "Checks if the value of the field is `NOT`"]
    #[inline(always)]
    pub fn is_not(&self) -> bool {
        *self == IPO_RDY_A::NOT
    }
    #[doc = "Checks if the value of the field is `READY`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == IPO_RDY_A::READY
    }
}
#[doc = "Field `IBRO_RDY` reader - 7.3725 MHz HIRC Ready."]
pub type IBRO_RDY_R = crate::BitReader<IBRO_RDY_A>;
#[doc = "7.3725 MHz HIRC Ready.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IBRO_RDY_A {
    #[doc = "0: Is not Ready."]
    NOT = 0,
    #[doc = "1: Is Ready."]
    READY = 1,
}
impl From<IBRO_RDY_A> for bool {
    #[inline(always)]
    fn from(variant: IBRO_RDY_A) -> Self {
        variant as u8 != 0
    }
}
impl IBRO_RDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IBRO_RDY_A {
        match self.bits {
            false => IBRO_RDY_A::NOT,
            true => IBRO_RDY_A::READY,
        }
    }
    #[doc = "Checks if the value of the field is `NOT`"]
    #[inline(always)]
    pub fn is_not(&self) -> bool {
        *self == IBRO_RDY_A::NOT
    }
    #[doc = "Checks if the value of the field is `READY`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == IBRO_RDY_A::READY
    }
}
#[doc = "Field `INRO_RDY` reader - 8 kHz Low Frequency Reference Clock Ready."]
pub type INRO_RDY_R = crate::BitReader<INRO_RDY_A>;
#[doc = "8 kHz Low Frequency Reference Clock Ready.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INRO_RDY_A {
    #[doc = "0: Is not Ready."]
    NOT = 0,
    #[doc = "1: Is Ready."]
    READY = 1,
}
impl From<INRO_RDY_A> for bool {
    #[inline(always)]
    fn from(variant: INRO_RDY_A) -> Self {
        variant as u8 != 0
    }
}
impl INRO_RDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INRO_RDY_A {
        match self.bits {
            false => INRO_RDY_A::NOT,
            true => INRO_RDY_A::READY,
        }
    }
    #[doc = "Checks if the value of the field is `NOT`"]
    #[inline(always)]
    pub fn is_not(&self) -> bool {
        *self == INRO_RDY_A::NOT
    }
    #[doc = "Checks if the value of the field is `READY`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == INRO_RDY_A::READY
    }
}
impl R {
    #[doc = "Bits 6:8 - Prescaler Select. This 3 bit field sets the system operating frequency by controlling the prescaler that divides the output of the PLL0."]
    #[inline(always)]
    pub fn sysclk_div(&self) -> SYSCLK_DIV_R {
        SYSCLK_DIV_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11 - Clock Source Select. This 3 bit field selects the source for the system clock."]
    #[inline(always)]
    pub fn sysclk_sel(&self) -> SYSCLK_SEL_R {
        SYSCLK_SEL_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bit 13 - Clock Ready. This read only bit reflects whether the currently selected system clock source is running."]
    #[inline(always)]
    pub fn sysclk_rdy(&self) -> SYSCLK_RDY_R {
        SYSCLK_RDY_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - 32MHz Crystal Oscillator Enable."]
    #[inline(always)]
    pub fn erfo_en(&self) -> ERFO_EN_R {
        ERFO_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 32 kHz Crystal Oscillator Enable."]
    #[inline(always)]
    pub fn ertco_en(&self) -> ERTCO_EN_R {
        ERTCO_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 60 MHz High Frequency Internal Reference Clock Enable."]
    #[inline(always)]
    pub fn iso_en(&self) -> ISO_EN_R {
        ISO_EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 100 MHz High Frequency Internal Reference Clock Enable."]
    #[inline(always)]
    pub fn ipo_en(&self) -> IPO_EN_R {
        IPO_EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 7.3725 MHz High Frequency Internal Reference Clock Enable."]
    #[inline(always)]
    pub fn ibro_en(&self) -> IBRO_EN_R {
        IBRO_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 7.3725 MHz High Frequency Internal Reference Clock Voltage Select. This register bit is used to select the power supply to the IBRO."]
    #[inline(always)]
    pub fn ibro_vs(&self) -> IBRO_VS_R {
        IBRO_VS_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 24 - 32MHz Crystal Oscillator Ready"]
    #[inline(always)]
    pub fn erfo_rdy(&self) -> ERFO_RDY_R {
        ERFO_RDY_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 32 kHz Crystal Oscillator Ready"]
    #[inline(always)]
    pub fn ertco_rdy(&self) -> ERTCO_RDY_R {
        ERTCO_RDY_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 60 MHz HIRC Ready."]
    #[inline(always)]
    pub fn iso_rdy(&self) -> ISO_RDY_R {
        ISO_RDY_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - 100 MHz HIRC Ready."]
    #[inline(always)]
    pub fn ipo_rdy(&self) -> IPO_RDY_R {
        IPO_RDY_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - 7.3725 MHz HIRC Ready."]
    #[inline(always)]
    pub fn ibro_rdy(&self) -> IBRO_RDY_R {
        IBRO_RDY_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 8 kHz Low Frequency Reference Clock Ready."]
    #[inline(always)]
    pub fn inro_rdy(&self) -> INRO_RDY_R {
        INRO_RDY_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 6:8 - Prescaler Select. This 3 bit field sets the system operating frequency by controlling the prescaler that divides the output of the PLL0."]
    #[inline(always)]
    #[must_use]
    pub fn sysclk_div(&mut self) -> SYSCLK_DIV_W<6> {
        SYSCLK_DIV_W::new(self)
    }
    #[doc = "Bits 9:11 - Clock Source Select. This 3 bit field selects the source for the system clock."]
    #[inline(always)]
    #[must_use]
    pub fn sysclk_sel(&mut self) -> SYSCLK_SEL_W<9> {
        SYSCLK_SEL_W::new(self)
    }
    #[doc = "Bit 16 - 32MHz Crystal Oscillator Enable."]
    #[inline(always)]
    #[must_use]
    pub fn erfo_en(&mut self) -> ERFO_EN_W<16> {
        ERFO_EN_W::new(self)
    }
    #[doc = "Bit 17 - 32 kHz Crystal Oscillator Enable."]
    #[inline(always)]
    #[must_use]
    pub fn ertco_en(&mut self) -> ERTCO_EN_W<17> {
        ERTCO_EN_W::new(self)
    }
    #[doc = "Bit 18 - 60 MHz High Frequency Internal Reference Clock Enable."]
    #[inline(always)]
    #[must_use]
    pub fn iso_en(&mut self) -> ISO_EN_W<18> {
        ISO_EN_W::new(self)
    }
    #[doc = "Bit 19 - 100 MHz High Frequency Internal Reference Clock Enable."]
    #[inline(always)]
    #[must_use]
    pub fn ipo_en(&mut self) -> IPO_EN_W<19> {
        IPO_EN_W::new(self)
    }
    #[doc = "Bit 20 - 7.3725 MHz High Frequency Internal Reference Clock Enable."]
    #[inline(always)]
    #[must_use]
    pub fn ibro_en(&mut self) -> IBRO_EN_W<20> {
        IBRO_EN_W::new(self)
    }
    #[doc = "Bit 21 - 7.3725 MHz High Frequency Internal Reference Clock Voltage Select. This register bit is used to select the power supply to the IBRO."]
    #[inline(always)]
    #[must_use]
    pub fn ibro_vs(&mut self) -> IBRO_VS_W<21> {
        IBRO_VS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock Control.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkctrl](index.html) module"]
pub struct CLKCTRL_SPEC;
impl crate::RegisterSpec for CLKCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clkctrl::R](R) reader structure"]
impl crate::Readable for CLKCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clkctrl::W](W) writer structure"]
impl crate::Writable for CLKCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLKCTRL to value 0x08"]
impl crate::Resettable for CLKCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x08;
}
