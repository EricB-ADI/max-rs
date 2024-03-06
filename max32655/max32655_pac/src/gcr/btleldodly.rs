#[doc = "Register `BTLELDODLY` reader"]
pub struct R(crate::R<BTLELDODLY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BTLELDODLY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BTLELDODLY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BTLELDODLY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BTLELDODLY` writer"]
pub struct W(crate::W<BTLELDODLY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BTLELDODLY_SPEC>;
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
impl From<crate::W<BTLELDODLY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BTLELDODLY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BYPDLYCNT` reader - Bypass Delay Count."]
pub type BYPDLYCNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BYPDLYCNT` writer - Bypass Delay Count."]
pub type BYPDLYCNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BTLELDODLY_SPEC, u8, u8, 8, O>;
#[doc = "Field `LDORXDLYCNT` reader - LDORX Delay Count."]
pub type LDORXDLYCNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `LDORXDLYCNT` writer - LDORX Delay Count."]
pub type LDORXDLYCNT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, BTLELDODLY_SPEC, u16, u16, 9, O>;
#[doc = "Field `LDOTXDLYCNT` reader - LDOTX Delay Count."]
pub type LDOTXDLYCNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `LDOTXDLYCNT` writer - LDOTX Delay Count."]
pub type LDOTXDLYCNT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, BTLELDODLY_SPEC, u16, u16, 9, O>;
impl R {
    #[doc = "Bits 0:7 - Bypass Delay Count."]
    #[inline(always)]
    pub fn bypdlycnt(&self) -> BYPDLYCNT_R {
        BYPDLYCNT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:16 - LDORX Delay Count."]
    #[inline(always)]
    pub fn ldorxdlycnt(&self) -> LDORXDLYCNT_R {
        LDORXDLYCNT_R::new(((self.bits >> 8) & 0x01ff) as u16)
    }
    #[doc = "Bits 20:28 - LDOTX Delay Count."]
    #[inline(always)]
    pub fn ldotxdlycnt(&self) -> LDOTXDLYCNT_R {
        LDOTXDLYCNT_R::new(((self.bits >> 20) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - Bypass Delay Count."]
    #[inline(always)]
    #[must_use]
    pub fn bypdlycnt(&mut self) -> BYPDLYCNT_W<0> {
        BYPDLYCNT_W::new(self)
    }
    #[doc = "Bits 8:16 - LDORX Delay Count."]
    #[inline(always)]
    #[must_use]
    pub fn ldorxdlycnt(&mut self) -> LDORXDLYCNT_W<8> {
        LDORXDLYCNT_W::new(self)
    }
    #[doc = "Bits 20:28 - LDOTX Delay Count."]
    #[inline(always)]
    #[must_use]
    pub fn ldotxdlycnt(&mut self) -> LDOTXDLYCNT_W<20> {
        LDOTXDLYCNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BTLE LDO Delay Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [btleldodly](index.html) module"]
pub struct BTLELDODLY_SPEC;
impl crate::RegisterSpec for BTLELDODLY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [btleldodly::R](R) reader structure"]
impl crate::Readable for BTLELDODLY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [btleldodly::W](W) writer structure"]
impl crate::Writable for BTLELDODLY_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BTLELDODLY to value 0"]
impl crate::Resettable for BTLELDODLY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
