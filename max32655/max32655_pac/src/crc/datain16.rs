#[doc = "Register `DATAIN16[%s]` reader"]
pub struct R(crate::R<DATAIN16_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATAIN16_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DATAIN16_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DATAIN16_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DATAIN16[%s]` writer"]
pub struct W(crate::W<DATAIN16_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DATAIN16_SPEC>;
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
impl From<crate::W<DATAIN16_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DATAIN16_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA` reader - CRC Data"]
pub type DATA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DATA` writer - CRC Data"]
pub type DATA_W<'a, const O: u8> = crate::FieldWriter<'a, u16, DATAIN16_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - CRC Data"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - CRC Data"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DATA_W<0> {
        DATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CRC Data Input\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [datain16](index.html) module"]
pub struct DATAIN16_SPEC;
impl crate::RegisterSpec for DATAIN16_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [datain16::R](R) reader structure"]
impl crate::Readable for DATAIN16_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [datain16::W](W) writer structure"]
impl crate::Writable for DATAIN16_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DATAIN16[%s]
to value 0"]
impl crate::Resettable for DATAIN16_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
