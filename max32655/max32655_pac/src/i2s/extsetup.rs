#[doc = "Register `EXTSETUP` reader"]
pub struct R(crate::R<EXTSETUP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTSETUP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTSETUP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTSETUP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXTSETUP` writer"]
pub struct W(crate::W<EXTSETUP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTSETUP_SPEC>;
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
impl From<crate::W<EXTSETUP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXTSETUP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EXT_BITS_WORD` reader - Word Length for ch_mode."]
pub type EXT_BITS_WORD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EXT_BITS_WORD` writer - Word Length for ch_mode."]
pub type EXT_BITS_WORD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EXTSETUP_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - Word Length for ch_mode."]
    #[inline(always)]
    pub fn ext_bits_word(&self) -> EXT_BITS_WORD_R {
        EXT_BITS_WORD_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Word Length for ch_mode."]
    #[inline(always)]
    #[must_use]
    pub fn ext_bits_word(&mut self) -> EXT_BITS_WORD_W<0> {
        EXT_BITS_WORD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ext Control.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extsetup](index.html) module"]
pub struct EXTSETUP_SPEC;
impl crate::RegisterSpec for EXTSETUP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [extsetup::R](R) reader structure"]
impl crate::Readable for EXTSETUP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [extsetup::W](W) writer structure"]
impl crate::Writable for EXTSETUP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EXTSETUP to value 0"]
impl crate::Resettable for EXTSETUP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
