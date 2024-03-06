#[doc = "Register `CLKSEL` reader"]
pub struct R(crate::R<CLKSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLKSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLKSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLKSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLKSEL` writer"]
pub struct W(crate::W<CLKSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLKSEL_SPEC>;
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
impl From<crate::W<CLKSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLKSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SOURCE` reader - WWDT Clock Selection Register."]
pub type SOURCE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SOURCE` writer - WWDT Clock Selection Register."]
pub type SOURCE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CLKSEL_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:2 - WWDT Clock Selection Register."]
    #[inline(always)]
    pub fn source(&self) -> SOURCE_R {
        SOURCE_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - WWDT Clock Selection Register."]
    #[inline(always)]
    #[must_use]
    pub fn source(&mut self) -> SOURCE_W<0> {
        SOURCE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Windowed Watchdog Timer Clock Select Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clksel](index.html) module"]
pub struct CLKSEL_SPEC;
impl crate::RegisterSpec for CLKSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clksel::R](R) reader structure"]
impl crate::Readable for CLKSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clksel::W](W) writer structure"]
impl crate::Writable for CLKSEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLKSEL to value 0"]
impl crate::Resettable for CLKSEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
