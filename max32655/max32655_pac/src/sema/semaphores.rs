#[doc = "Register `SEMAPHORES[%s]` reader"]
pub struct R(crate::R<SEMAPHORES_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEMAPHORES_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEMAPHORES_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEMAPHORES_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEMAPHORES[%s]` writer"]
pub struct W(crate::W<SEMAPHORES_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEMAPHORES_SPEC>;
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
impl From<crate::W<SEMAPHORES_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEMAPHORES_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `sema` reader - "]
pub type SEMA_R = crate::BitReader<bool>;
#[doc = "Field `sema` writer - "]
pub type SEMA_W<'a, const O: u8> = crate::BitWriter<'a, u32, SEMAPHORES_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sema(&self) -> SEMA_R {
        SEMA_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn sema(&mut self) -> SEMA_W<0> {
        SEMA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Read to test and set, returns prior value. Write 0 to clear semaphore.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [semaphores](index.html) module"]
pub struct SEMAPHORES_SPEC;
impl crate::RegisterSpec for SEMAPHORES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [semaphores::R](R) reader structure"]
impl crate::Readable for SEMAPHORES_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [semaphores::W](W) writer structure"]
impl crate::Writable for SEMAPHORES_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEMAPHORES[%s]
to value 0"]
impl crate::Resettable for SEMAPHORES_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
