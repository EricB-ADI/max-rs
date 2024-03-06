#[doc = "Register `IPO_MTRIM` reader"]
pub struct R(crate::R<IPO_MTRIM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IPO_MTRIM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IPO_MTRIM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IPO_MTRIM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IPO_MTRIM` writer"]
pub struct W(crate::W<IPO_MTRIM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IPO_MTRIM_SPEC>;
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
impl From<crate::W<IPO_MTRIM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IPO_MTRIM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MTRIM` reader - Manual Trim Value."]
pub type MTRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MTRIM` writer - Manual Trim Value."]
pub type MTRIM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IPO_MTRIM_SPEC, u8, u8, 8, O>;
#[doc = "Field `TRIM_RANGE` reader - Trim Range Select."]
pub type TRIM_RANGE_R = crate::BitReader<bool>;
#[doc = "Field `TRIM_RANGE` writer - Trim Range Select."]
pub type TRIM_RANGE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IPO_MTRIM_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:7 - Manual Trim Value."]
    #[inline(always)]
    pub fn mtrim(&self) -> MTRIM_R {
        MTRIM_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Trim Range Select."]
    #[inline(always)]
    pub fn trim_range(&self) -> TRIM_RANGE_R {
        TRIM_RANGE_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Manual Trim Value."]
    #[inline(always)]
    #[must_use]
    pub fn mtrim(&mut self) -> MTRIM_W<0> {
        MTRIM_W::new(self)
    }
    #[doc = "Bit 8 - Trim Range Select."]
    #[inline(always)]
    #[must_use]
    pub fn trim_range(&mut self) -> TRIM_RANGE_W<8> {
        TRIM_RANGE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IPO Manual Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipo_mtrim](index.html) module"]
pub struct IPO_MTRIM_SPEC;
impl crate::RegisterSpec for IPO_MTRIM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ipo_mtrim::R](R) reader structure"]
impl crate::Readable for IPO_MTRIM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ipo_mtrim::W](W) writer structure"]
impl crate::Writable for IPO_MTRIM_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IPO_MTRIM to value 0"]
impl crate::Resettable for IPO_MTRIM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
