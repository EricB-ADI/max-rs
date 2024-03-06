#[doc = "Register `URVCTRL` reader"]
pub struct R(crate::R<URVCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<URVCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<URVCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<URVCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `URVCTRL` writer"]
pub struct W(crate::W<URVCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<URVCTRL_SPEC>;
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
impl From<crate::W<URVCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<URVCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MEMSEL` reader - RAM2, RAM3 exclusive ownership."]
pub type MEMSEL_R = crate::BitReader<bool>;
#[doc = "Field `MEMSEL` writer - RAM2, RAM3 exclusive ownership."]
pub type MEMSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, URVCTRL_SPEC, bool, O>;
#[doc = "Field `IFLUSHEN` reader - URV instruction flush enable."]
pub type IFLUSHEN_R = crate::BitReader<bool>;
#[doc = "Field `IFLUSHEN` writer - URV instruction flush enable."]
pub type IFLUSHEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, URVCTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - RAM2, RAM3 exclusive ownership."]
    #[inline(always)]
    pub fn memsel(&self) -> MEMSEL_R {
        MEMSEL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - URV instruction flush enable."]
    #[inline(always)]
    pub fn iflushen(&self) -> IFLUSHEN_R {
        IFLUSHEN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RAM2, RAM3 exclusive ownership."]
    #[inline(always)]
    #[must_use]
    pub fn memsel(&mut self) -> MEMSEL_W<0> {
        MEMSEL_W::new(self)
    }
    #[doc = "Bit 1 - URV instruction flush enable."]
    #[inline(always)]
    #[must_use]
    pub fn iflushen(&mut self) -> IFLUSHEN_W<1> {
        IFLUSHEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RISC-V Control Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [urvctrl](index.html) module"]
pub struct URVCTRL_SPEC;
impl crate::RegisterSpec for URVCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [urvctrl::R](R) reader structure"]
impl crate::Readable for URVCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [urvctrl::W](W) writer structure"]
impl crate::Writable for URVCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets URVCTRL to value 0"]
impl crate::Resettable for URVCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
