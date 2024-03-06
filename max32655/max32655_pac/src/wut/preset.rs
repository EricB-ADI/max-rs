#[doc = "Register `PRESET` reader"]
pub struct R(crate::R<PRESET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRESET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRESET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRESET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRESET` writer"]
pub struct W(crate::W<PRESET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRESET_SPEC>;
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
impl From<crate::W<PRESET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRESET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRESET` reader - Preset Value."]
pub type PRESET_R = crate::FieldReader<u32, u32>;
#[doc = "Field `PRESET` writer - Preset Value."]
pub type PRESET_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PRESET_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Preset Value."]
    #[inline(always)]
    pub fn preset(&self) -> PRESET_R {
        PRESET_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Preset Value."]
    #[inline(always)]
    #[must_use]
    pub fn preset(&mut self) -> PRESET_W<0> {
        PRESET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Preset register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [preset](index.html) module"]
pub struct PRESET_SPEC;
impl crate::RegisterSpec for PRESET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [preset::R](R) reader structure"]
impl crate::Readable for PRESET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [preset::W](W) writer structure"]
impl crate::Writable for PRESET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRESET to value 0"]
impl crate::Resettable for PRESET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
