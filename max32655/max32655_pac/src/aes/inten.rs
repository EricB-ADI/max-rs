#[doc = "Register `INTEN` reader"]
pub struct R(crate::R<INTEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTEN` writer"]
pub struct W(crate::W<INTEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTEN_SPEC>;
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
impl From<crate::W<INTEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DONE` reader - AES Done Interrupt Enable"]
pub type DONE_R = crate::BitReader<bool>;
#[doc = "Field `DONE` writer - AES Done Interrupt Enable"]
pub type DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, O>;
#[doc = "Field `KEY_CHANGE` reader - External AES Key Changed Interrupt Enable"]
pub type KEY_CHANGE_R = crate::BitReader<bool>;
#[doc = "Field `KEY_CHANGE` writer - External AES Key Changed Interrupt Enable"]
pub type KEY_CHANGE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, O>;
#[doc = "Field `KEY_ZERO` reader - External AES Key Zero Interrupt Enable"]
pub type KEY_ZERO_R = crate::BitReader<bool>;
#[doc = "Field `KEY_ZERO` writer - External AES Key Zero Interrupt Enable"]
pub type KEY_ZERO_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, O>;
#[doc = "Field `OV` reader - Data Output FIFO Overrun Interrupt Enable"]
pub type OV_R = crate::BitReader<bool>;
#[doc = "Field `OV` writer - Data Output FIFO Overrun Interrupt Enable"]
pub type OV_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, O>;
#[doc = "Field `KEY_ONE` reader - KEY_ONE"]
pub type KEY_ONE_R = crate::BitReader<bool>;
#[doc = "Field `KEY_ONE` writer - KEY_ONE"]
pub type KEY_ONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - AES Done Interrupt Enable"]
    #[inline(always)]
    pub fn done(&self) -> DONE_R {
        DONE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - External AES Key Changed Interrupt Enable"]
    #[inline(always)]
    pub fn key_change(&self) -> KEY_CHANGE_R {
        KEY_CHANGE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - External AES Key Zero Interrupt Enable"]
    #[inline(always)]
    pub fn key_zero(&self) -> KEY_ZERO_R {
        KEY_ZERO_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Data Output FIFO Overrun Interrupt Enable"]
    #[inline(always)]
    pub fn ov(&self) -> OV_R {
        OV_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - KEY_ONE"]
    #[inline(always)]
    pub fn key_one(&self) -> KEY_ONE_R {
        KEY_ONE_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AES Done Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn done(&mut self) -> DONE_W<0> {
        DONE_W::new(self)
    }
    #[doc = "Bit 1 - External AES Key Changed Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn key_change(&mut self) -> KEY_CHANGE_W<1> {
        KEY_CHANGE_W::new(self)
    }
    #[doc = "Bit 2 - External AES Key Zero Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn key_zero(&mut self) -> KEY_ZERO_W<2> {
        KEY_ZERO_W::new(self)
    }
    #[doc = "Bit 3 - Data Output FIFO Overrun Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ov(&mut self) -> OV_W<3> {
        OV_W::new(self)
    }
    #[doc = "Bit 4 - KEY_ONE"]
    #[inline(always)]
    #[must_use]
    pub fn key_one(&mut self) -> KEY_ONE_W<4> {
        KEY_ONE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AES Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inten](index.html) module"]
pub struct INTEN_SPEC;
impl crate::RegisterSpec for INTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [inten::R](R) reader structure"]
impl crate::Readable for INTEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [inten::W](W) writer structure"]
impl crate::Writable for INTEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTEN to value 0"]
impl crate::Resettable for INTEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
