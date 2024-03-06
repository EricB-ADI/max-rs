#[doc = "Register `ERFOKS` reader"]
pub struct R(crate::R<ERFOKS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ERFOKS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ERFOKS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ERFOKS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ERFOKS` writer"]
pub struct W(crate::W<ERFOKS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ERFOKS_SPEC>;
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
impl From<crate::W<ERFOKS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ERFOKS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `KSERFO_CNT` reader - Kick Start ERFO Counter."]
pub type KSERFO_CNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `KSERFO_CNT` writer - Kick Start ERFO Counter."]
pub type KSERFO_CNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ERFOKS_SPEC, u8, u8, 7, O>;
#[doc = "Field `KSERFO_EN` reader - Kick Start ERFO Enable."]
pub type KSERFO_EN_R = crate::BitReader<bool>;
#[doc = "Field `KSERFO_EN` writer - Kick Start ERFO Enable."]
pub type KSERFO_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERFOKS_SPEC, bool, O>;
#[doc = "Field `KSERFODRIVER` reader - Kick Start ERFO Driver."]
pub type KSERFODRIVER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `KSERFODRIVER` writer - Kick Start ERFO Driver."]
pub type KSERFODRIVER_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ERFOKS_SPEC, u8, u8, 3, O>;
#[doc = "Field `KSERFO2X` reader - Kick Start ERFO 2X Pulse."]
pub type KSERFO2X_R = crate::BitReader<bool>;
#[doc = "Field `KSERFO2X` writer - Kick Start ERFO 2X Pulse."]
pub type KSERFO2X_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERFOKS_SPEC, bool, O>;
#[doc = "Field `KSCLKSEL` reader - Kick Start Clock Select for ERFO."]
pub type KSCLKSEL_R = crate::FieldReader<u8, KSCLKSEL_A>;
#[doc = "Kick Start Clock Select for ERFO.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum KSCLKSEL_A {
    #[doc = "2: ISO."]
    ISO = 2,
    #[doc = "3: IPO."]
    IPO = 3,
}
impl From<KSCLKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: KSCLKSEL_A) -> Self {
        variant as _
    }
}
impl KSCLKSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<KSCLKSEL_A> {
        match self.bits {
            2 => Some(KSCLKSEL_A::ISO),
            3 => Some(KSCLKSEL_A::IPO),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ISO`"]
    #[inline(always)]
    pub fn is_iso(&self) -> bool {
        *self == KSCLKSEL_A::ISO
    }
    #[doc = "Checks if the value of the field is `IPO`"]
    #[inline(always)]
    pub fn is_ipo(&self) -> bool {
        *self == KSCLKSEL_A::IPO
    }
}
#[doc = "Field `KSCLKSEL` writer - Kick Start Clock Select for ERFO."]
pub type KSCLKSEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ERFOKS_SPEC, u8, KSCLKSEL_A, 2, O>;
impl<'a, const O: u8> KSCLKSEL_W<'a, O> {
    #[doc = "ISO."]
    #[inline(always)]
    pub fn iso(self) -> &'a mut W {
        self.variant(KSCLKSEL_A::ISO)
    }
    #[doc = "IPO."]
    #[inline(always)]
    pub fn ipo(self) -> &'a mut W {
        self.variant(KSCLKSEL_A::IPO)
    }
}
impl R {
    #[doc = "Bits 0:6 - Kick Start ERFO Counter."]
    #[inline(always)]
    pub fn kserfo_cnt(&self) -> KSERFO_CNT_R {
        KSERFO_CNT_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - Kick Start ERFO Enable."]
    #[inline(always)]
    pub fn kserfo_en(&self) -> KSERFO_EN_R {
        KSERFO_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Kick Start ERFO Driver."]
    #[inline(always)]
    pub fn kserfodriver(&self) -> KSERFODRIVER_R {
        KSERFODRIVER_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - Kick Start ERFO 2X Pulse."]
    #[inline(always)]
    pub fn kserfo2x(&self) -> KSERFO2X_R {
        KSERFO2X_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Kick Start Clock Select for ERFO."]
    #[inline(always)]
    pub fn ksclksel(&self) -> KSCLKSEL_R {
        KSCLKSEL_R::new(((self.bits >> 12) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Kick Start ERFO Counter."]
    #[inline(always)]
    #[must_use]
    pub fn kserfo_cnt(&mut self) -> KSERFO_CNT_W<0> {
        KSERFO_CNT_W::new(self)
    }
    #[doc = "Bit 7 - Kick Start ERFO Enable."]
    #[inline(always)]
    #[must_use]
    pub fn kserfo_en(&mut self) -> KSERFO_EN_W<7> {
        KSERFO_EN_W::new(self)
    }
    #[doc = "Bits 8:10 - Kick Start ERFO Driver."]
    #[inline(always)]
    #[must_use]
    pub fn kserfodriver(&mut self) -> KSERFODRIVER_W<8> {
        KSERFODRIVER_W::new(self)
    }
    #[doc = "Bit 11 - Kick Start ERFO 2X Pulse."]
    #[inline(always)]
    #[must_use]
    pub fn kserfo2x(&mut self) -> KSERFO2X_W<11> {
        KSERFO2X_W::new(self)
    }
    #[doc = "Bits 12:13 - Kick Start Clock Select for ERFO."]
    #[inline(always)]
    #[must_use]
    pub fn ksclksel(&mut self) -> KSCLKSEL_W<12> {
        KSCLKSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ERFO Kick Start Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [erfoks](index.html) module"]
pub struct ERFOKS_SPEC;
impl crate::RegisterSpec for ERFOKS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [erfoks::R](R) reader structure"]
impl crate::Readable for ERFOKS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [erfoks::W](W) writer structure"]
impl crate::Writable for ERFOKS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ERFOKS to value 0"]
impl crate::Resettable for ERFOKS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
