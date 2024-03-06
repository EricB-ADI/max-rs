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
#[doc = "Field `LO` reader - Low duty cycle control. In timer mode, reload\\[7:0\\]."]
pub type LO_R = crate::FieldReader<u8, LO_A>;
#[doc = "Low duty cycle control. In timer mode, reload\\[7:0\\].\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LO_A {
    #[doc = "0: Duty cycle control of serial clock generation is disabled."]
    DIS = 0,
}
impl From<LO_A> for u8 {
    #[inline(always)]
    fn from(variant: LO_A) -> Self {
        variant as _
    }
}
impl LO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LO_A> {
        match self.bits {
            0 => Some(LO_A::DIS),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == LO_A::DIS
    }
}
#[doc = "Field `LO` writer - Low duty cycle control. In timer mode, reload\\[7:0\\]."]
pub type LO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CLKCTRL_SPEC, u8, LO_A, 8, O>;
impl<'a, const O: u8> LO_W<'a, O> {
    #[doc = "Duty cycle control of serial clock generation is disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(LO_A::DIS)
    }
}
#[doc = "Field `HI` reader - High duty cycle control. In timer mode, reload\\[15:8\\]."]
pub type HI_R = crate::FieldReader<u8, HI_A>;
#[doc = "High duty cycle control. In timer mode, reload\\[15:8\\].\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HI_A {
    #[doc = "0: Duty cycle control of serial clock generation is disabled."]
    DIS = 0,
}
impl From<HI_A> for u8 {
    #[inline(always)]
    fn from(variant: HI_A) -> Self {
        variant as _
    }
}
impl HI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<HI_A> {
        match self.bits {
            0 => Some(HI_A::DIS),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == HI_A::DIS
    }
}
#[doc = "Field `HI` writer - High duty cycle control. In timer mode, reload\\[15:8\\]."]
pub type HI_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CLKCTRL_SPEC, u8, HI_A, 8, O>;
impl<'a, const O: u8> HI_W<'a, O> {
    #[doc = "Duty cycle control of serial clock generation is disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(HI_A::DIS)
    }
}
#[doc = "Field `CLKDIV` reader - System Clock scale factor. Scales the AMBA clock by 2^SCALE before generating serial clock."]
pub type CLKDIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CLKDIV` writer - System Clock scale factor. Scales the AMBA clock by 2^SCALE before generating serial clock."]
pub type CLKDIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CLKCTRL_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:7 - Low duty cycle control. In timer mode, reload\\[7:0\\]."]
    #[inline(always)]
    pub fn lo(&self) -> LO_R {
        LO_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - High duty cycle control. In timer mode, reload\\[15:8\\]."]
    #[inline(always)]
    pub fn hi(&self) -> HI_R {
        HI_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - System Clock scale factor. Scales the AMBA clock by 2^SCALE before generating serial clock."]
    #[inline(always)]
    pub fn clkdiv(&self) -> CLKDIV_R {
        CLKDIV_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Low duty cycle control. In timer mode, reload\\[7:0\\]."]
    #[inline(always)]
    #[must_use]
    pub fn lo(&mut self) -> LO_W<0> {
        LO_W::new(self)
    }
    #[doc = "Bits 8:15 - High duty cycle control. In timer mode, reload\\[15:8\\]."]
    #[inline(always)]
    #[must_use]
    pub fn hi(&mut self) -> HI_W<8> {
        HI_W::new(self)
    }
    #[doc = "Bits 16:19 - System Clock scale factor. Scales the AMBA clock by 2^SCALE before generating serial clock."]
    #[inline(always)]
    #[must_use]
    pub fn clkdiv(&mut self) -> CLKDIV_W<16> {
        CLKDIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Register for controlling SPI clock rate.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkctrl](index.html) module"]
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
#[doc = "`reset()` method sets CLKCTRL to value 0"]
impl crate::Resettable for CLKCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
