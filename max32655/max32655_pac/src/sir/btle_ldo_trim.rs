#[doc = "Register `BTLE_LDO_TRIM` reader"]
pub struct R(crate::R<BTLE_LDO_TRIM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BTLE_LDO_TRIM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BTLE_LDO_TRIM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BTLE_LDO_TRIM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BTLE_LDO_TRIM` writer"]
pub struct W(crate::W<BTLE_LDO_TRIM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BTLE_LDO_TRIM_SPEC>;
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
impl From<crate::W<BTLE_LDO_TRIM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BTLE_LDO_TRIM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX` reader - TX LDO trim value."]
pub type TX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TX` writer - TX LDO trim value."]
pub type TX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BTLE_LDO_TRIM_SPEC, u8, u8, 5, O>;
#[doc = "Field `RX` reader - RX LDO trim value."]
pub type RX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RX` writer - RX LDO trim value."]
pub type RX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BTLE_LDO_TRIM_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - TX LDO trim value."]
    #[inline(always)]
    pub fn tx(&self) -> TX_R {
        TX_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - RX LDO trim value."]
    #[inline(always)]
    pub fn rx(&self) -> RX_R {
        RX_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - TX LDO trim value."]
    #[inline(always)]
    #[must_use]
    pub fn tx(&mut self) -> TX_W<0> {
        TX_W::new(self)
    }
    #[doc = "Bits 8:12 - RX LDO trim value."]
    #[inline(always)]
    #[must_use]
    pub fn rx(&mut self) -> RX_W<8> {
        RX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BTLE LDO Trim register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [btle_ldo_trim](index.html) module"]
pub struct BTLE_LDO_TRIM_SPEC;
impl crate::RegisterSpec for BTLE_LDO_TRIM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [btle_ldo_trim::R](R) reader structure"]
impl crate::Readable for BTLE_LDO_TRIM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [btle_ldo_trim::W](W) writer structure"]
impl crate::Writable for BTLE_LDO_TRIM_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BTLE_LDO_TRIM to value 0"]
impl crate::Resettable for BTLE_LDO_TRIM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
