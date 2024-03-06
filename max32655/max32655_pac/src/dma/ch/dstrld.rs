#[doc = "Register `DSTRLD` reader"]
pub struct R(crate::R<DSTRLD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DSTRLD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DSTRLD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DSTRLD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DSTRLD` writer"]
pub struct W(crate::W<DSTRLD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DSTRLD_SPEC>;
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
impl From<crate::W<DSTRLD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DSTRLD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDR` reader - Destination Address Reload Value."]
pub type ADDR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ADDR` writer - Destination Address Reload Value."]
pub type ADDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DSTRLD_SPEC, u32, u32, 31, O>;
impl R {
    #[doc = "Bits 0:30 - Destination Address Reload Value."]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new(self.bits & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:30 - Destination Address Reload Value."]
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
#[doc = "Destination Address Reload Value. The value of this register is loaded into DMA0_DST upon a count-to-zero condition.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dstrld](index.html) module"]
pub struct DSTRLD_SPEC;
impl crate::RegisterSpec for DSTRLD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dstrld::R](R) reader structure"]
impl crate::Readable for DSTRLD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dstrld::W](W) writer structure"]
impl crate::Writable for DSTRLD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DSTRLD to value 0"]
impl crate::Resettable for DSTRLD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
