#[doc = "Register `IPKA` reader"]
pub struct R(crate::R<IPKA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IPKA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IPKA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IPKA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IPKA` writer"]
pub struct W(crate::W<IPKA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IPKA_SPEC>;
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
impl From<crate::W<IPKA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IPKA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IPKSETA` reader - Voltage Regulator Peak Current Setting"]
pub type IPKSETA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IPKSETA` writer - Voltage Regulator Peak Current Setting"]
pub type IPKSETA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IPKA_SPEC, u8, u8, 4, O>;
#[doc = "Field `IPKSETB` reader - Voltage Regulator Peak Current Setting"]
pub type IPKSETB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IPKSETB` writer - Voltage Regulator Peak Current Setting"]
pub type IPKSETB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IPKA_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Voltage Regulator Peak Current Setting"]
    #[inline(always)]
    pub fn ipkseta(&self) -> IPKSETA_R {
        IPKSETA_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Voltage Regulator Peak Current Setting"]
    #[inline(always)]
    pub fn ipksetb(&self) -> IPKSETB_R {
        IPKSETB_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Voltage Regulator Peak Current Setting"]
    #[inline(always)]
    #[must_use]
    pub fn ipkseta(&mut self) -> IPKSETA_W<0> {
        IPKSETA_W::new(self)
    }
    #[doc = "Bits 4:7 - Voltage Regulator Peak Current Setting"]
    #[inline(always)]
    #[must_use]
    pub fn ipksetb(&mut self) -> IPKSETB_W<4> {
        IPKSETB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "High Side FET Peak Current VREGO_A/VREGO_B Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipka](index.html) module"]
pub struct IPKA_SPEC;
impl crate::RegisterSpec for IPKA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ipka::R](R) reader structure"]
impl crate::Readable for IPKA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ipka::W](W) writer structure"]
impl crate::Writable for IPKA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IPKA to value 0"]
impl crate::Resettable for IPKA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
