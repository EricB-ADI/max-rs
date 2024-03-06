#[doc = "Register `RESYNC` reader"]
pub struct R(crate::R<RESYNC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESYNC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RESYNC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RESYNC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RESYNC` writer"]
pub struct W(crate::W<RESYNC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RESYNC_SPEC>;
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
impl From<crate::W<RESYNC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RESYNC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pt0` reader - Resync control for PT0"]
pub type PT0_R = crate::BitReader<bool>;
#[doc = "Field `pt0` writer - Resync control for PT0"]
pub type PT0_W<'a, const O: u8> = crate::BitWriter<'a, u32, RESYNC_SPEC, bool, O>;
#[doc = "Field `pt1` reader - Resync control for PT1"]
pub type PT1_R = crate::BitReader<bool>;
#[doc = "Field `pt1` writer - Resync control for PT1"]
pub type PT1_W<'a, const O: u8> = crate::BitWriter<'a, u32, RESYNC_SPEC, bool, O>;
#[doc = "Field `pt2` reader - Resync control for PT2"]
pub type PT2_R = crate::BitReader<bool>;
#[doc = "Field `pt2` writer - Resync control for PT2"]
pub type PT2_W<'a, const O: u8> = crate::BitWriter<'a, u32, RESYNC_SPEC, bool, O>;
#[doc = "Field `pt3` reader - Resync control for PT3"]
pub type PT3_R = crate::BitReader<bool>;
#[doc = "Field `pt3` writer - Resync control for PT3"]
pub type PT3_W<'a, const O: u8> = crate::BitWriter<'a, u32, RESYNC_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Resync control for PT0"]
    #[inline(always)]
    pub fn pt0(&self) -> PT0_R {
        PT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Resync control for PT1"]
    #[inline(always)]
    pub fn pt1(&self) -> PT1_R {
        PT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Resync control for PT2"]
    #[inline(always)]
    pub fn pt2(&self) -> PT2_R {
        PT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Resync control for PT3"]
    #[inline(always)]
    pub fn pt3(&self) -> PT3_R {
        PT3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Resync control for PT0"]
    #[inline(always)]
    #[must_use]
    pub fn pt0(&mut self) -> PT0_W<0> {
        PT0_W::new(self)
    }
    #[doc = "Bit 1 - Resync control for PT1"]
    #[inline(always)]
    #[must_use]
    pub fn pt1(&mut self) -> PT1_W<1> {
        PT1_W::new(self)
    }
    #[doc = "Bit 2 - Resync control for PT2"]
    #[inline(always)]
    #[must_use]
    pub fn pt2(&mut self) -> PT2_W<2> {
        PT2_W::new(self)
    }
    #[doc = "Bit 3 - Resync control for PT3"]
    #[inline(always)]
    #[must_use]
    pub fn pt3(&mut self) -> PT3_W<3> {
        PT3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Global Resync (All Pulse Trains) Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [resync](index.html) module"]
pub struct RESYNC_SPEC;
impl crate::RegisterSpec for RESYNC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [resync::R](R) reader structure"]
impl crate::Readable for RESYNC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [resync::W](W) writer structure"]
impl crate::Writable for RESYNC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RESYNC to value 0"]
impl crate::Resettable for RESYNC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
