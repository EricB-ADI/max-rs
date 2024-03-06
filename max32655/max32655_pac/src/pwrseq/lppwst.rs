#[doc = "Register `LPPWST` reader"]
pub struct R(crate::R<LPPWST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPPWST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPPWST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPPWST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LPPWST` writer"]
pub struct W(crate::W<LPPWST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LPPWST_SPEC>;
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
impl From<crate::W<LPPWST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LPPWST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AINCOMP0` reader - Analog Input Comparator Wakeup Flag."]
pub type AINCOMP0_R = crate::BitReader<bool>;
#[doc = "Field `AINCOMP0` writer - Analog Input Comparator Wakeup Flag."]
pub type AINCOMP0_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPPWST_SPEC, bool, O>;
#[doc = "Field `BACKUP` reader - Backup Mode Wakeup Flag."]
pub type BACKUP_R = crate::BitReader<bool>;
#[doc = "Field `BACKUP` writer - Backup Mode Wakeup Flag."]
pub type BACKUP_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPPWST_SPEC, bool, O>;
#[doc = "Field `RESET` reader - Reset Detected Wakeup Flag."]
pub type RESET_R = crate::BitReader<bool>;
#[doc = "Field `RESET` writer - Reset Detected Wakeup Flag."]
pub type RESET_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPPWST_SPEC, bool, O>;
impl R {
    #[doc = "Bit 4 - Analog Input Comparator Wakeup Flag."]
    #[inline(always)]
    pub fn aincomp0(&self) -> AINCOMP0_R {
        AINCOMP0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 16 - Backup Mode Wakeup Flag."]
    #[inline(always)]
    pub fn backup(&self) -> BACKUP_R {
        BACKUP_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Reset Detected Wakeup Flag."]
    #[inline(always)]
    pub fn reset(&self) -> RESET_R {
        RESET_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Analog Input Comparator Wakeup Flag."]
    #[inline(always)]
    #[must_use]
    pub fn aincomp0(&mut self) -> AINCOMP0_W<4> {
        AINCOMP0_W::new(self)
    }
    #[doc = "Bit 16 - Backup Mode Wakeup Flag."]
    #[inline(always)]
    #[must_use]
    pub fn backup(&mut self) -> BACKUP_W<16> {
        BACKUP_W::new(self)
    }
    #[doc = "Bit 17 - Reset Detected Wakeup Flag."]
    #[inline(always)]
    #[must_use]
    pub fn reset(&mut self) -> RESET_W<17> {
        RESET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Low Power Peripheral Wakeup Status Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lppwst](index.html) module"]
pub struct LPPWST_SPEC;
impl crate::RegisterSpec for LPPWST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lppwst::R](R) reader structure"]
impl crate::Readable for LPPWST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lppwst::W](W) writer structure"]
impl crate::Writable for LPPWST_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LPPWST to value 0"]
impl crate::Resettable for LPPWST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
