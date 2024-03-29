#[doc = "Register `DATAIN8[%s]` reader"]
pub struct R(crate::R<DATAIN8_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATAIN8_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DATAIN8_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DATAIN8_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DATAIN8[%s]` writer"]
pub struct W(crate::W<DATAIN8_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DATAIN8_SPEC>;
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
impl From<crate::W<DATAIN8_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DATAIN8_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA` reader - CRC Data"]
pub type DATA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA` writer - CRC Data"]
pub type DATA_W<'a, const O: u8> = crate::FieldWriter<'a, u8, DATAIN8_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - CRC Data"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - CRC Data"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DATA_W<0> {
        DATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CRC Data Input\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [datain8](index.html) module"]
pub struct DATAIN8_SPEC;
impl crate::RegisterSpec for DATAIN8_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [datain8::R](R) reader structure"]
impl crate::Readable for DATAIN8_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [datain8::W](W) writer structure"]
impl crate::Writable for DATAIN8_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DATAIN8[%s]
to value 0"]
impl crate::Resettable for DATAIN8_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
