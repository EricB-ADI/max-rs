#[doc = "Register `FIFO8[%s]` reader"]
pub struct R(crate::R<FIFO8_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FIFO8_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FIFO8_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FIFO8_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FIFO8[%s]` writer"]
pub struct W(crate::W<FIFO8_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FIFO8_SPEC>;
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
impl From<crate::W<FIFO8_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FIFO8_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA` reader - Read to pull from RX FIFO, write to put into TX FIFO."]
pub type DATA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA` writer - Read to pull from RX FIFO, write to put into TX FIFO."]
pub type DATA_W<'a, const O: u8> = crate::FieldWriter<'a, u8, FIFO8_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Read to pull from RX FIFO, write to put into TX FIFO."]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Read to pull from RX FIFO, write to put into TX FIFO."]
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
#[doc = "Register for reading and writing the FIFO.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifo8](index.html) module"]
pub struct FIFO8_SPEC;
impl crate::RegisterSpec for FIFO8_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [fifo8::R](R) reader structure"]
impl crate::Readable for FIFO8_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fifo8::W](W) writer structure"]
impl crate::Writable for FIFO8_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FIFO8[%s]
to value 0"]
impl crate::Resettable for FIFO8_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
