#[doc = "Register `ECCDATA` reader"]
pub struct R(crate::R<ECCDATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ECCDATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ECCDATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ECCDATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ECCDATA` writer"]
pub struct W(crate::W<ECCDATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ECCDATA_SPEC>;
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
impl From<crate::W<ECCDATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ECCDATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EVEN` reader - Error Correction Code Odd Data."]
pub type EVEN_R = crate::FieldReader<u16, u16>;
#[doc = "Field `EVEN` writer - Error Correction Code Odd Data."]
pub type EVEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ECCDATA_SPEC, u16, u16, 9, O>;
#[doc = "Field `ODD` reader - Error Correction Code Even Data."]
pub type ODD_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ODD` writer - Error Correction Code Even Data."]
pub type ODD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ECCDATA_SPEC, u16, u16, 9, O>;
impl R {
    #[doc = "Bits 0:8 - Error Correction Code Odd Data."]
    #[inline(always)]
    pub fn even(&self) -> EVEN_R {
        EVEN_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 16:24 - Error Correction Code Even Data."]
    #[inline(always)]
    pub fn odd(&self) -> ODD_R {
        ODD_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - Error Correction Code Odd Data."]
    #[inline(always)]
    #[must_use]
    pub fn even(&mut self) -> EVEN_W<0> {
        EVEN_W::new(self)
    }
    #[doc = "Bits 16:24 - Error Correction Code Even Data."]
    #[inline(always)]
    #[must_use]
    pub fn odd(&mut self) -> ODD_W<16> {
        ODD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ECC Data Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eccdata](index.html) module"]
pub struct ECCDATA_SPEC;
impl crate::RegisterSpec for ECCDATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eccdata::R](R) reader structure"]
impl crate::Readable for ECCDATA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eccdata::W](W) writer structure"]
impl crate::Writable for ECCDATA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ECCDATA to value 0"]
impl crate::Resettable for ECCDATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
