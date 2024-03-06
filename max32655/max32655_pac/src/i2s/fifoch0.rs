#[doc = "Register `FIFOCH0` reader"]
pub struct R(crate::R<FIFOCH0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FIFOCH0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FIFOCH0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FIFOCH0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FIFOCH0` writer"]
pub struct W(crate::W<FIFOCH0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FIFOCH0_SPEC>;
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
impl From<crate::W<FIFOCH0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FIFOCH0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA` reader - Load/unload location for TX and RX FIFO buffers."]
pub type DATA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DATA` writer - Load/unload location for TX and RX FIFO buffers."]
pub type DATA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FIFOCH0_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Load/unload location for TX and RX FIFO buffers."]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Load/unload location for TX and RX FIFO buffers."]
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
#[doc = "I2S Fifo.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifoch0](index.html) module"]
pub struct FIFOCH0_SPEC;
impl crate::RegisterSpec for FIFOCH0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fifoch0::R](R) reader structure"]
impl crate::Readable for FIFOCH0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fifoch0::W](W) writer structure"]
impl crate::Writable for FIFOCH0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FIFOCH0 to value 0"]
impl crate::Resettable for FIFOCH0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
