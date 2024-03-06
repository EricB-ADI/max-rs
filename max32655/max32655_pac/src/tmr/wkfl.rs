#[doc = "Register `WKFL` reader"]
pub struct R(crate::R<WKFL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WKFL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WKFL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WKFL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WKFL` writer"]
pub struct W(crate::W<WKFL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WKFL_SPEC>;
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
impl From<crate::W<WKFL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WKFL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `A` reader - Wake-Up Flag for Timer A"]
pub type A_R = crate::BitReader<bool>;
#[doc = "Field `A` writer - Wake-Up Flag for Timer A"]
pub type A_W<'a, const O: u8> = crate::BitWriter<'a, u32, WKFL_SPEC, bool, O>;
#[doc = "Field `B` reader - Wake-Up Flag for Timer B"]
pub type B_R = crate::BitReader<bool>;
#[doc = "Field `B` writer - Wake-Up Flag for Timer B"]
pub type B_W<'a, const O: u8> = crate::BitWriter<'a, u32, WKFL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Wake-Up Flag for Timer A"]
    #[inline(always)]
    pub fn a(&self) -> A_R {
        A_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16 - Wake-Up Flag for Timer B"]
    #[inline(always)]
    pub fn b(&self) -> B_R {
        B_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Wake-Up Flag for Timer A"]
    #[inline(always)]
    #[must_use]
    pub fn a(&mut self) -> A_W<0> {
        A_W::new(self)
    }
    #[doc = "Bit 16 - Wake-Up Flag for Timer B"]
    #[inline(always)]
    #[must_use]
    pub fn b(&mut self) -> B_W<16> {
        B_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer Wakeup Status Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wkfl](index.html) module"]
pub struct WKFL_SPEC;
impl crate::RegisterSpec for WKFL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wkfl::R](R) reader structure"]
impl crate::Readable for WKFL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wkfl::W](W) writer structure"]
impl crate::Writable for WKFL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WKFL to value 0"]
impl crate::Resettable for WKFL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
