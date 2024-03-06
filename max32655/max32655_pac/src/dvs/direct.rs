#[doc = "Register `DIRECT` reader"]
pub struct R(crate::R<DIRECT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIRECT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIRECT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIRECT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIRECT` writer"]
pub struct W(crate::W<DIRECT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIRECT_SPEC>;
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
impl From<crate::W<DIRECT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIRECT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VOLTAGE` reader - Sets the target power supply value"]
pub type VOLTAGE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VOLTAGE` writer - Sets the target power supply value"]
pub type VOLTAGE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DIRECT_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:6 - Sets the target power supply value"]
    #[inline(always)]
    pub fn voltage(&self) -> VOLTAGE_R {
        VOLTAGE_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Sets the target power supply value"]
    #[inline(always)]
    #[must_use]
    pub fn voltage(&mut self) -> VOLTAGE_W<0> {
        VOLTAGE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Direct control of target voltage\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [direct](index.html) module"]
pub struct DIRECT_SPEC;
impl crate::RegisterSpec for DIRECT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [direct::R](R) reader structure"]
impl crate::Readable for DIRECT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [direct::W](W) writer structure"]
impl crate::Writable for DIRECT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIRECT to value 0"]
impl crate::Resettable for DIRECT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
