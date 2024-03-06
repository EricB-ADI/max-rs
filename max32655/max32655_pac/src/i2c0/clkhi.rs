#[doc = "Register `CLKHI` reader"]
pub struct R(crate::R<CLKHI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLKHI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLKHI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLKHI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLKHI` writer"]
pub struct W(crate::W<CLKHI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLKHI_SPEC>;
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
impl From<crate::W<CLKHI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLKHI_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HI` reader - Clock High. In master mode, these bits define the SCL high period."]
pub type HI_R = crate::FieldReader<u16, u16>;
#[doc = "Field `HI` writer - Clock High. In master mode, these bits define the SCL high period."]
pub type HI_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CLKHI_SPEC, u16, u16, 9, O>;
impl R {
    #[doc = "Bits 0:8 - Clock High. In master mode, these bits define the SCL high period."]
    #[inline(always)]
    pub fn hi(&self) -> HI_R {
        HI_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - Clock High. In master mode, these bits define the SCL high period."]
    #[inline(always)]
    #[must_use]
    pub fn hi(&mut self) -> HI_W<0> {
        HI_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock high Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkhi](index.html) module"]
pub struct CLKHI_SPEC;
impl crate::RegisterSpec for CLKHI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clkhi::R](R) reader structure"]
impl crate::Readable for CLKHI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clkhi::W](W) writer structure"]
impl crate::Writable for CLKHI_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLKHI to value 0"]
impl crate::Resettable for CLKHI_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
