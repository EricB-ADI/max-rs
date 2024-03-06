#[doc = "Register `OUTEN` reader"]
pub struct R(crate::R<OUTEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OUTEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OUTEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OUTEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OUTEN` writer"]
pub struct W(crate::W<OUTEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OUTEN_SPEC>;
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
impl From<crate::W<OUTEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OUTEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SQWOUT_EN` reader - Square Wave Output Enable."]
pub type SQWOUT_EN_R = crate::BitReader<bool>;
#[doc = "Field `SQWOUT_EN` writer - Square Wave Output Enable."]
pub type SQWOUT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, OUTEN_SPEC, bool, O>;
#[doc = "Field `PDOWN_OUT_EN` reader - Power Down Output Enable."]
pub type PDOWN_OUT_EN_R = crate::BitReader<bool>;
#[doc = "Field `PDOWN_OUT_EN` writer - Power Down Output Enable."]
pub type PDOWN_OUT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, OUTEN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Square Wave Output Enable."]
    #[inline(always)]
    pub fn sqwout_en(&self) -> SQWOUT_EN_R {
        SQWOUT_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Power Down Output Enable."]
    #[inline(always)]
    pub fn pdown_out_en(&self) -> PDOWN_OUT_EN_R {
        PDOWN_OUT_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Square Wave Output Enable."]
    #[inline(always)]
    #[must_use]
    pub fn sqwout_en(&mut self) -> SQWOUT_EN_W<0> {
        SQWOUT_EN_W::new(self)
    }
    #[doc = "Bit 1 - Power Down Output Enable."]
    #[inline(always)]
    #[must_use]
    pub fn pdown_out_en(&mut self) -> PDOWN_OUT_EN_W<1> {
        PDOWN_OUT_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Output Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [outen](index.html) module"]
pub struct OUTEN_SPEC;
impl crate::RegisterSpec for OUTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [outen::R](R) reader structure"]
impl crate::Readable for OUTEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [outen::W](W) writer structure"]
impl crate::Writable for OUTEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OUTEN to value 0"]
impl crate::Resettable for OUTEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
