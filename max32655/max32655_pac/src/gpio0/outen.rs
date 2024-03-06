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
#[doc = "Field `EN` reader - Mask of all of the pins on the port."]
pub type EN_R = crate::FieldReader<u32, EN_A>;
#[doc = "Mask of all of the pins on the port.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum EN_A {
    #[doc = "0: GPIO Output Disable"]
    DIS = 0,
    #[doc = "1: GPIO Output Enable"]
    EN = 1,
}
impl From<EN_A> for u32 {
    #[inline(always)]
    fn from(variant: EN_A) -> Self {
        variant as _
    }
}
impl EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EN_A> {
        match self.bits {
            0 => Some(EN_A::DIS),
            1 => Some(EN_A::EN),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == EN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == EN_A::EN
    }
}
#[doc = "Field `EN` writer - Mask of all of the pins on the port."]
pub type EN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OUTEN_SPEC, u32, EN_A, 32, O>;
impl<'a, const O: u8> EN_W<'a, O> {
    #[doc = "GPIO Output Disable"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(EN_A::DIS)
    }
    #[doc = "GPIO Output Enable"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(EN_A::EN)
    }
}
impl R {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO Output Enable Register. Each bit controls the GPIO_OUT_EN setting for one GPIO pin in the associated port.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [outen](index.html) module"]
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
