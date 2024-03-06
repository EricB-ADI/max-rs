#[doc = "Register `IPKB` reader"]
pub struct R(crate::R<IPKB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IPKB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IPKB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IPKB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IPKB` writer"]
pub struct W(crate::W<IPKB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IPKB_SPEC>;
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
impl From<crate::W<IPKB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IPKB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IPKSETC` reader - Voltage Regulator Peak Current Setting"]
pub type IPKSETC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IPKSETC` writer - Voltage Regulator Peak Current Setting"]
pub type IPKSETC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IPKB_SPEC, u8, u8, 4, O>;
#[doc = "Field `IPKSETD` reader - Voltage Regulator Peak Current Setting"]
pub type IPKSETD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IPKSETD` writer - Voltage Regulator Peak Current Setting"]
pub type IPKSETD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IPKB_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Voltage Regulator Peak Current Setting"]
    #[inline(always)]
    pub fn ipksetc(&self) -> IPKSETC_R {
        IPKSETC_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Voltage Regulator Peak Current Setting"]
    #[inline(always)]
    pub fn ipksetd(&self) -> IPKSETD_R {
        IPKSETD_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Voltage Regulator Peak Current Setting"]
    #[inline(always)]
    #[must_use]
    pub fn ipksetc(&mut self) -> IPKSETC_W<0> {
        IPKSETC_W::new(self)
    }
    #[doc = "Bits 4:7 - Voltage Regulator Peak Current Setting"]
    #[inline(always)]
    #[must_use]
    pub fn ipksetd(&mut self) -> IPKSETD_W<4> {
        IPKSETD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "High Side FET Peak Current VREGO_C/VREGO_D Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipkb](index.html) module"]
pub struct IPKB_SPEC;
impl crate::RegisterSpec for IPKB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ipkb::R](R) reader structure"]
impl crate::Readable for IPKB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ipkb::W](W) writer structure"]
impl crate::Writable for IPKB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IPKB to value 0"]
impl crate::Resettable for IPKB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
