#[doc = "Register `BTLELDOCTRL` reader"]
pub struct R(crate::R<BTLELDOCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BTLELDOCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BTLELDOCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BTLELDOCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BTLELDOCTRL` writer"]
pub struct W(crate::W<BTLELDOCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BTLELDOCTRL_SPEC>;
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
impl From<crate::W<BTLELDOCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BTLELDOCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LDOTXEN` reader - LDOTX Enable."]
pub type LDOTXEN_R = crate::BitReader<bool>;
#[doc = "Field `LDOTXEN` writer - LDOTX Enable."]
pub type LDOTXEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, BTLELDOCTRL_SPEC, bool, O>;
#[doc = "Field `LDOTXPULLD` reader - LDOTX Pull Down."]
pub type LDOTXPULLD_R = crate::BitReader<bool>;
#[doc = "Field `LDOTXPULLD` writer - LDOTX Pull Down."]
pub type LDOTXPULLD_W<'a, const O: u8> = crate::BitWriter<'a, u32, BTLELDOCTRL_SPEC, bool, O>;
#[doc = "Field `LDOTXVSEL` reader - LDOTX Voltage Setting."]
pub type LDOTXVSEL_R = crate::FieldReader<u8, LDOTXVSEL_A>;
#[doc = "LDOTX Voltage Setting.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LDOTXVSEL_A {
    #[doc = "0: 0.7V"]
    _0_7 = 0,
    #[doc = "1: 0.85V"]
    _0_85 = 1,
    #[doc = "2: 0.9V"]
    _0_9 = 2,
    #[doc = "3: 1.1V"]
    _1_1 = 3,
}
impl From<LDOTXVSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: LDOTXVSEL_A) -> Self {
        variant as _
    }
}
impl LDOTXVSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LDOTXVSEL_A {
        match self.bits {
            0 => LDOTXVSEL_A::_0_7,
            1 => LDOTXVSEL_A::_0_85,
            2 => LDOTXVSEL_A::_0_9,
            3 => LDOTXVSEL_A::_1_1,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0_7`"]
    #[inline(always)]
    pub fn is_0_7(&self) -> bool {
        *self == LDOTXVSEL_A::_0_7
    }
    #[doc = "Checks if the value of the field is `_0_85`"]
    #[inline(always)]
    pub fn is_0_85(&self) -> bool {
        *self == LDOTXVSEL_A::_0_85
    }
    #[doc = "Checks if the value of the field is `_0_9`"]
    #[inline(always)]
    pub fn is_0_9(&self) -> bool {
        *self == LDOTXVSEL_A::_0_9
    }
    #[doc = "Checks if the value of the field is `_1_1`"]
    #[inline(always)]
    pub fn is_1_1(&self) -> bool {
        *self == LDOTXVSEL_A::_1_1
    }
}
#[doc = "Field `LDOTXVSEL` writer - LDOTX Voltage Setting."]
pub type LDOTXVSEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, BTLELDOCTRL_SPEC, u8, LDOTXVSEL_A, 2, O>;
impl<'a, const O: u8> LDOTXVSEL_W<'a, O> {
    #[doc = "0.7V"]
    #[inline(always)]
    pub fn _0_7(self) -> &'a mut W {
        self.variant(LDOTXVSEL_A::_0_7)
    }
    #[doc = "0.85V"]
    #[inline(always)]
    pub fn _0_85(self) -> &'a mut W {
        self.variant(LDOTXVSEL_A::_0_85)
    }
    #[doc = "0.9V"]
    #[inline(always)]
    pub fn _0_9(self) -> &'a mut W {
        self.variant(LDOTXVSEL_A::_0_9)
    }
    #[doc = "1.1V"]
    #[inline(always)]
    pub fn _1_1(self) -> &'a mut W {
        self.variant(LDOTXVSEL_A::_1_1)
    }
}
#[doc = "Field `LDORXEN` reader - LDORX Enable."]
pub type LDORXEN_R = crate::BitReader<bool>;
#[doc = "Field `LDORXEN` writer - LDORX Enable."]
pub type LDORXEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, BTLELDOCTRL_SPEC, bool, O>;
#[doc = "Field `LDORXPULLD` reader - LDOrX Pull Down."]
pub type LDORXPULLD_R = crate::BitReader<bool>;
#[doc = "Field `LDORXPULLD` writer - LDOrX Pull Down."]
pub type LDORXPULLD_W<'a, const O: u8> = crate::BitWriter<'a, u32, BTLELDOCTRL_SPEC, bool, O>;
#[doc = "Field `LDORXVSEL` reader - LDORX Voltage Setting."]
pub type LDORXVSEL_R = crate::FieldReader<u8, LDORXVSEL_A>;
#[doc = "LDORX Voltage Setting.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LDORXVSEL_A {
    #[doc = "0: 0.7V"]
    _0_7 = 0,
    #[doc = "1: 0.85V"]
    _0_85 = 1,
    #[doc = "2: 0.9V"]
    _0_9 = 2,
    #[doc = "3: 1.1V"]
    _1_1 = 3,
}
impl From<LDORXVSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: LDORXVSEL_A) -> Self {
        variant as _
    }
}
impl LDORXVSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LDORXVSEL_A {
        match self.bits {
            0 => LDORXVSEL_A::_0_7,
            1 => LDORXVSEL_A::_0_85,
            2 => LDORXVSEL_A::_0_9,
            3 => LDORXVSEL_A::_1_1,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0_7`"]
    #[inline(always)]
    pub fn is_0_7(&self) -> bool {
        *self == LDORXVSEL_A::_0_7
    }
    #[doc = "Checks if the value of the field is `_0_85`"]
    #[inline(always)]
    pub fn is_0_85(&self) -> bool {
        *self == LDORXVSEL_A::_0_85
    }
    #[doc = "Checks if the value of the field is `_0_9`"]
    #[inline(always)]
    pub fn is_0_9(&self) -> bool {
        *self == LDORXVSEL_A::_0_9
    }
    #[doc = "Checks if the value of the field is `_1_1`"]
    #[inline(always)]
    pub fn is_1_1(&self) -> bool {
        *self == LDORXVSEL_A::_1_1
    }
}
#[doc = "Field `LDORXVSEL` writer - LDORX Voltage Setting."]
pub type LDORXVSEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, BTLELDOCTRL_SPEC, u8, LDORXVSEL_A, 2, O>;
impl<'a, const O: u8> LDORXVSEL_W<'a, O> {
    #[doc = "0.7V"]
    #[inline(always)]
    pub fn _0_7(self) -> &'a mut W {
        self.variant(LDORXVSEL_A::_0_7)
    }
    #[doc = "0.85V"]
    #[inline(always)]
    pub fn _0_85(self) -> &'a mut W {
        self.variant(LDORXVSEL_A::_0_85)
    }
    #[doc = "0.9V"]
    #[inline(always)]
    pub fn _0_9(self) -> &'a mut W {
        self.variant(LDORXVSEL_A::_0_9)
    }
    #[doc = "1.1V"]
    #[inline(always)]
    pub fn _1_1(self) -> &'a mut W {
        self.variant(LDORXVSEL_A::_1_1)
    }
}
#[doc = "Field `LDORXBYP` reader - LDORX Bypass Enable."]
pub type LDORXBYP_R = crate::BitReader<bool>;
#[doc = "Field `LDORXBYP` writer - LDORX Bypass Enable."]
pub type LDORXBYP_W<'a, const O: u8> = crate::BitWriter<'a, u32, BTLELDOCTRL_SPEC, bool, O>;
#[doc = "Field `LDORXDISCH` reader - LDORX Discharge."]
pub type LDORXDISCH_R = crate::BitReader<bool>;
#[doc = "Field `LDORXDISCH` writer - LDORX Discharge."]
pub type LDORXDISCH_W<'a, const O: u8> = crate::BitWriter<'a, u32, BTLELDOCTRL_SPEC, bool, O>;
#[doc = "Field `LDOTXBYP` reader - LDOTX Bypass Enable."]
pub type LDOTXBYP_R = crate::BitReader<bool>;
#[doc = "Field `LDOTXBYP` writer - LDOTX Bypass Enable."]
pub type LDOTXBYP_W<'a, const O: u8> = crate::BitWriter<'a, u32, BTLELDOCTRL_SPEC, bool, O>;
#[doc = "Field `LDOTXDISCH` reader - LDOTX Discharge."]
pub type LDOTXDISCH_R = crate::BitReader<bool>;
#[doc = "Field `LDOTXDISCH` writer - LDOTX Discharge."]
pub type LDOTXDISCH_W<'a, const O: u8> = crate::BitWriter<'a, u32, BTLELDOCTRL_SPEC, bool, O>;
#[doc = "Field `LDOTXENDLY` reader - LDOTX Enable Delay."]
pub type LDOTXENDLY_R = crate::BitReader<bool>;
#[doc = "Field `LDOTXENDLY` writer - LDOTX Enable Delay."]
pub type LDOTXENDLY_W<'a, const O: u8> = crate::BitWriter<'a, u32, BTLELDOCTRL_SPEC, bool, O>;
#[doc = "Field `LDORXENDLY` reader - LDORX Enable Delay."]
pub type LDORXENDLY_R = crate::BitReader<bool>;
#[doc = "Field `LDORXENDLY` writer - LDORX Enable Delay."]
pub type LDORXENDLY_W<'a, const O: u8> = crate::BitWriter<'a, u32, BTLELDOCTRL_SPEC, bool, O>;
#[doc = "Field `LDORXBYPENENDLY` reader - LDORX Bypass Enable Delay."]
pub type LDORXBYPENENDLY_R = crate::BitReader<bool>;
#[doc = "Field `LDORXBYPENENDLY` writer - LDORX Bypass Enable Delay."]
pub type LDORXBYPENENDLY_W<'a, const O: u8> = crate::BitWriter<'a, u32, BTLELDOCTRL_SPEC, bool, O>;
#[doc = "Field `LDOTXBYPENENDLY` reader - LDOTX Bypass Enable Delay."]
pub type LDOTXBYPENENDLY_R = crate::BitReader<bool>;
#[doc = "Field `LDOTXBYPENENDLY` writer - LDOTX Bypass Enable Delay."]
pub type LDOTXBYPENENDLY_W<'a, const O: u8> = crate::BitWriter<'a, u32, BTLELDOCTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - LDOTX Enable."]
    #[inline(always)]
    pub fn ldotxen(&self) -> LDOTXEN_R {
        LDOTXEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LDOTX Pull Down."]
    #[inline(always)]
    pub fn ldotxpulld(&self) -> LDOTXPULLD_R {
        LDOTXPULLD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - LDOTX Voltage Setting."]
    #[inline(always)]
    pub fn ldotxvsel(&self) -> LDOTXVSEL_R {
        LDOTXVSEL_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - LDORX Enable."]
    #[inline(always)]
    pub fn ldorxen(&self) -> LDORXEN_R {
        LDORXEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - LDOrX Pull Down."]
    #[inline(always)]
    pub fn ldorxpulld(&self) -> LDORXPULLD_R {
        LDORXPULLD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - LDORX Voltage Setting."]
    #[inline(always)]
    pub fn ldorxvsel(&self) -> LDORXVSEL_R {
        LDORXVSEL_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - LDORX Bypass Enable."]
    #[inline(always)]
    pub fn ldorxbyp(&self) -> LDORXBYP_R {
        LDORXBYP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - LDORX Discharge."]
    #[inline(always)]
    pub fn ldorxdisch(&self) -> LDORXDISCH_R {
        LDORXDISCH_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - LDOTX Bypass Enable."]
    #[inline(always)]
    pub fn ldotxbyp(&self) -> LDOTXBYP_R {
        LDOTXBYP_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - LDOTX Discharge."]
    #[inline(always)]
    pub fn ldotxdisch(&self) -> LDOTXDISCH_R {
        LDOTXDISCH_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - LDOTX Enable Delay."]
    #[inline(always)]
    pub fn ldotxendly(&self) -> LDOTXENDLY_R {
        LDOTXENDLY_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - LDORX Enable Delay."]
    #[inline(always)]
    pub fn ldorxendly(&self) -> LDORXENDLY_R {
        LDORXENDLY_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - LDORX Bypass Enable Delay."]
    #[inline(always)]
    pub fn ldorxbypenendly(&self) -> LDORXBYPENENDLY_R {
        LDORXBYPENENDLY_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - LDOTX Bypass Enable Delay."]
    #[inline(always)]
    pub fn ldotxbypenendly(&self) -> LDOTXBYPENENDLY_R {
        LDOTXBYPENENDLY_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LDOTX Enable."]
    #[inline(always)]
    #[must_use]
    pub fn ldotxen(&mut self) -> LDOTXEN_W<0> {
        LDOTXEN_W::new(self)
    }
    #[doc = "Bit 1 - LDOTX Pull Down."]
    #[inline(always)]
    #[must_use]
    pub fn ldotxpulld(&mut self) -> LDOTXPULLD_W<1> {
        LDOTXPULLD_W::new(self)
    }
    #[doc = "Bits 2:3 - LDOTX Voltage Setting."]
    #[inline(always)]
    #[must_use]
    pub fn ldotxvsel(&mut self) -> LDOTXVSEL_W<2> {
        LDOTXVSEL_W::new(self)
    }
    #[doc = "Bit 4 - LDORX Enable."]
    #[inline(always)]
    #[must_use]
    pub fn ldorxen(&mut self) -> LDORXEN_W<4> {
        LDORXEN_W::new(self)
    }
    #[doc = "Bit 5 - LDOrX Pull Down."]
    #[inline(always)]
    #[must_use]
    pub fn ldorxpulld(&mut self) -> LDORXPULLD_W<5> {
        LDORXPULLD_W::new(self)
    }
    #[doc = "Bits 6:7 - LDORX Voltage Setting."]
    #[inline(always)]
    #[must_use]
    pub fn ldorxvsel(&mut self) -> LDORXVSEL_W<6> {
        LDORXVSEL_W::new(self)
    }
    #[doc = "Bit 8 - LDORX Bypass Enable."]
    #[inline(always)]
    #[must_use]
    pub fn ldorxbyp(&mut self) -> LDORXBYP_W<8> {
        LDORXBYP_W::new(self)
    }
    #[doc = "Bit 9 - LDORX Discharge."]
    #[inline(always)]
    #[must_use]
    pub fn ldorxdisch(&mut self) -> LDORXDISCH_W<9> {
        LDORXDISCH_W::new(self)
    }
    #[doc = "Bit 10 - LDOTX Bypass Enable."]
    #[inline(always)]
    #[must_use]
    pub fn ldotxbyp(&mut self) -> LDOTXBYP_W<10> {
        LDOTXBYP_W::new(self)
    }
    #[doc = "Bit 11 - LDOTX Discharge."]
    #[inline(always)]
    #[must_use]
    pub fn ldotxdisch(&mut self) -> LDOTXDISCH_W<11> {
        LDOTXDISCH_W::new(self)
    }
    #[doc = "Bit 12 - LDOTX Enable Delay."]
    #[inline(always)]
    #[must_use]
    pub fn ldotxendly(&mut self) -> LDOTXENDLY_W<12> {
        LDOTXENDLY_W::new(self)
    }
    #[doc = "Bit 13 - LDORX Enable Delay."]
    #[inline(always)]
    #[must_use]
    pub fn ldorxendly(&mut self) -> LDORXENDLY_W<13> {
        LDORXENDLY_W::new(self)
    }
    #[doc = "Bit 14 - LDORX Bypass Enable Delay."]
    #[inline(always)]
    #[must_use]
    pub fn ldorxbypenendly(&mut self) -> LDORXBYPENENDLY_W<14> {
        LDORXBYPENENDLY_W::new(self)
    }
    #[doc = "Bit 15 - LDOTX Bypass Enable Delay."]
    #[inline(always)]
    #[must_use]
    pub fn ldotxbypenendly(&mut self) -> LDOTXBYPENENDLY_W<15> {
        LDOTXBYPENENDLY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BTLE LDO Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [btleldoctrl](index.html) module"]
pub struct BTLELDOCTRL_SPEC;
impl crate::RegisterSpec for BTLELDOCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [btleldoctrl::R](R) reader structure"]
impl crate::Readable for BTLELDOCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [btleldoctrl::W](W) writer structure"]
impl crate::Writable for BTLELDOCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BTLELDOCTRL to value 0"]
impl crate::Resettable for BTLELDOCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
