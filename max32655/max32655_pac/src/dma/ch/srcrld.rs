#[doc = "Register `SRCRLD` reader"]
pub struct R(crate::R<SRCRLD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRCRLD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRCRLD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRCRLD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SRCRLD` writer"]
pub struct W(crate::W<SRCRLD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRCRLD_SPEC>;
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
impl From<crate::W<SRCRLD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRCRLD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDR` reader - Source Address Reload Value."]
pub type ADDR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ADDR` writer - Source Address Reload Value."]
pub type ADDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SRCRLD_SPEC, u32, u32, 31, O>;
impl R {
    #[doc = "Bits 0:30 - Source Address Reload Value."]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new(self.bits & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:30 - Source Address Reload Value."]
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
#[doc = "Source Address Reload Value. The value of this register is loaded into DMA0_SRC upon a count-to-zero condition.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srcrld](index.html) module"]
pub struct SRCRLD_SPEC;
impl crate::RegisterSpec for SRCRLD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [srcrld::R](R) reader structure"]
impl crate::Readable for SRCRLD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [srcrld::W](W) writer structure"]
impl crate::Writable for SRCRLD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SRCRLD to value 0"]
impl crate::Resettable for SRCRLD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
