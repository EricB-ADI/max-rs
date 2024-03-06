#[doc = "Register `CNT` reader"]
pub struct R(crate::R<CNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CNT` writer"]
pub struct W(crate::W<CNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CNT_SPEC>;
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
impl From<crate::W<CNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CNT` reader - DMA Counter."]
pub type CNT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CNT` writer - DMA Counter."]
pub type CNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CNT_SPEC, u32, u32, 24, O>;
impl R {
    #[doc = "Bits 0:23 - DMA Counter."]
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - DMA Counter."]
    #[inline(always)]
    #[must_use]
    pub fn cnt(&mut self) -> CNT_W<0> {
        CNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Counter. The user loads this register with the number of bytes to transfer. This counter decreases on every AHB cycle into the DMA FIFO. The decrement will be 1, 2, or 4 depending on the data width of each AHB cycle. When the counter reaches 0, a count-to-zero condition is triggered.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cnt](index.html) module"]
pub struct CNT_SPEC;
impl crate::RegisterSpec for CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cnt::R](R) reader structure"]
impl crate::Readable for CNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cnt::W](W) writer structure"]
impl crate::Writable for CNT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CNT to value 0"]
impl crate::Resettable for CNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
