#[doc = "Register `REG0` reader"]
pub struct R(crate::R<REG0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REG0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REG0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REG0` writer"]
pub struct W(crate::W<REG0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REG0_SPEC>;
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
impl From<crate::W<REG0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REG0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ISO_WUP` reader - ISO Warm Up Value."]
pub type ISO_WUP_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ISO_WUP` writer - ISO Warm Up Value."]
pub type ISO_WUP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, REG0_SPEC, u16, u16, 9, O>;
#[doc = "Field `IPO_WUP` reader - IPO Warm Up Value."]
pub type IPO_WUP_R = crate::FieldReader<u16, u16>;
#[doc = "Field `IPO_WUP` writer - IPO Warm Up Value."]
pub type IPO_WUP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, REG0_SPEC, u16, u16, 11, O>;
impl R {
    #[doc = "Bits 0:8 - ISO Warm Up Value."]
    #[inline(always)]
    pub fn iso_wup(&self) -> ISO_WUP_R {
        ISO_WUP_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 16:26 - IPO Warm Up Value."]
    #[inline(always)]
    pub fn ipo_wup(&self) -> IPO_WUP_R {
        IPO_WUP_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - ISO Warm Up Value."]
    #[inline(always)]
    #[must_use]
    pub fn iso_wup(&mut self) -> ISO_WUP_W<0> {
        ISO_WUP_W::new(self)
    }
    #[doc = "Bits 16:26 - IPO Warm Up Value."]
    #[inline(always)]
    #[must_use]
    pub fn ipo_wup(&mut self) -> IPO_WUP_W<16> {
        IPO_WUP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Register 0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reg0](index.html) module"]
pub struct REG0_SPEC;
impl crate::RegisterSpec for REG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [reg0::R](R) reader structure"]
impl crate::Readable for REG0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [reg0::W](W) writer structure"]
impl crate::Writable for REG0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets REG0 to value 0"]
impl crate::Resettable for REG0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
