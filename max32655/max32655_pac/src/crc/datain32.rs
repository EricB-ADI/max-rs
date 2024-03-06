#[doc = "Register `DATAIN32` reader"]
pub struct R(crate::R<DATAIN32_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATAIN32_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DATAIN32_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DATAIN32_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DATAIN32` writer"]
pub struct W(crate::W<DATAIN32_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DATAIN32_SPEC>;
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
impl From<crate::W<DATAIN32_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DATAIN32_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA` reader - CRC Data"]
pub type DATA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DATA` writer - CRC Data"]
pub type DATA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DATAIN32_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - CRC Data"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CRC Data"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DATA_W<0> {
        DATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CRC Data Input\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [datain32](index.html) module"]
pub struct DATAIN32_SPEC;
impl crate::RegisterSpec for DATAIN32_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [datain32::R](R) reader structure"]
impl crate::Readable for DATAIN32_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [datain32::W](W) writer structure"]
impl crate::Writable for DATAIN32_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DATAIN32 to value 0"]
impl crate::Resettable for DATAIN32_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
