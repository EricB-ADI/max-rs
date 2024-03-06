#[doc = "Register `CLKLO` reader"]
pub struct R(crate::R<CLKLO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLKLO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLKLO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLKLO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLKLO` writer"]
pub struct W(crate::W<CLKLO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLKLO_SPEC>;
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
impl From<crate::W<CLKLO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLKLO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LO` reader - Clock low. In master mode, these bits define the SCL low period. In slave mode, these bits define the time SCL will be held low after data is outputted."]
pub type LO_R = crate::FieldReader<u16, u16>;
#[doc = "Field `LO` writer - Clock low. In master mode, these bits define the SCL low period. In slave mode, these bits define the time SCL will be held low after data is outputted."]
pub type LO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CLKLO_SPEC, u16, u16, 9, O>;
impl R {
    #[doc = "Bits 0:8 - Clock low. In master mode, these bits define the SCL low period. In slave mode, these bits define the time SCL will be held low after data is outputted."]
    #[inline(always)]
    pub fn lo(&self) -> LO_R {
        LO_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - Clock low. In master mode, these bits define the SCL low period. In slave mode, these bits define the time SCL will be held low after data is outputted."]
    #[inline(always)]
    #[must_use]
    pub fn lo(&mut self) -> LO_W<0> {
        LO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock Low Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clklo](index.html) module"]
pub struct CLKLO_SPEC;
impl crate::RegisterSpec for CLKLO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clklo::R](R) reader structure"]
impl crate::Readable for CLKLO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clklo::W](W) writer structure"]
impl crate::Writable for CLKLO_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLKLO to value 0"]
impl crate::Resettable for CLKLO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
