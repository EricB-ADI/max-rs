#[doc = "Register `FIFO32` reader"]
pub struct R(crate::R<FIFO32_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FIFO32_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FIFO32_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FIFO32_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FIFO32` writer"]
pub struct W(crate::W<FIFO32_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FIFO32_SPEC>;
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
impl From<crate::W<FIFO32_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FIFO32_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA` reader - Read to pull from RX FIFO, write to put into TX FIFO."]
pub type DATA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DATA` writer - Read to pull from RX FIFO, write to put into TX FIFO."]
pub type DATA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FIFO32_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Read to pull from RX FIFO, write to put into TX FIFO."]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Read to pull from RX FIFO, write to put into TX FIFO."]
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
#[doc = "Register for reading and writing the FIFO.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifo32](index.html) module"]
pub struct FIFO32_SPEC;
impl crate::RegisterSpec for FIFO32_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fifo32::R](R) reader structure"]
impl crate::Readable for FIFO32_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fifo32::W](W) writer structure"]
impl crate::Writable for FIFO32_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FIFO32 to value 0"]
impl crate::Resettable for FIFO32_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
