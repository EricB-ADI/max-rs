#[doc = "Register `SNAPSHOT` reader"]
pub struct R(crate::R<SNAPSHOT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SNAPSHOT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SNAPSHOT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SNAPSHOT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SNAPSHOT` writer"]
pub struct W(crate::W<SNAPSHOT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SNAPSHOT_SPEC>;
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
impl From<crate::W<SNAPSHOT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SNAPSHOT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SNAPSHOT` reader - Snapshot Value."]
pub type SNAPSHOT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SNAPSHOT` writer - Snapshot Value."]
pub type SNAPSHOT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SNAPSHOT_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Snapshot Value."]
    #[inline(always)]
    pub fn snapshot(&self) -> SNAPSHOT_R {
        SNAPSHOT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Snapshot Value."]
    #[inline(always)]
    #[must_use]
    pub fn snapshot(&mut self) -> SNAPSHOT_W<0> {
        SNAPSHOT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Snapshot register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [snapshot](index.html) module"]
pub struct SNAPSHOT_SPEC;
impl crate::RegisterSpec for SNAPSHOT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [snapshot::R](R) reader structure"]
impl crate::Readable for SNAPSHOT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [snapshot::W](W) writer structure"]
impl crate::Writable for SNAPSHOT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SNAPSHOT to value 0"]
impl crate::Resettable for SNAPSHOT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
