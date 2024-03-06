#[doc = "Register `SRC` reader"]
pub struct R(crate::R<SRC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SRC` writer"]
pub struct W(crate::W<SRC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRC_SPEC>;
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
impl From<crate::W<SRC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDR` reader - "]
pub type ADDR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ADDR` writer - "]
pub type ADDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SRC_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> ADDR_W<0> {
        ADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Source Device Address. If SRCINC=1, the counter bits are incremented by 1,2, or 4, depending on the data width of each AHB cycle. For peripheral transfers, some or all of the actual address bits are fixed. If SRCINC=0, this register remains constant. In the case where a count-to-zero condition occurs while RLDEN=1, the register is reloaded with the contents of DMA_SRC_RLD.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [src](index.html) module"]
pub struct SRC_SPEC;
impl crate::RegisterSpec for SRC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [src::R](R) reader structure"]
impl crate::Readable for SRC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [src::W](W) writer structure"]
impl crate::Writable for SRC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SRC to value 0"]
impl crate::Resettable for SRC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
