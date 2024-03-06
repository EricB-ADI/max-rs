#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ODHT` reader - Start On-Demand health test."]
pub type ODHT_R = crate::BitReader<bool>;
#[doc = "Field `ODHT` writer - Start On-Demand health test."]
pub type ODHT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `RND_IE` reader - To enable IRQ generation when a new 32-bit Random number is ready."]
pub type RND_IE_R = crate::BitReader<RND_IE_A>;
#[doc = "To enable IRQ generation when a new 32-bit Random number is ready.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RND_IE_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<RND_IE_A> for bool {
    #[inline(always)]
    fn from(variant: RND_IE_A) -> Self {
        variant as u8 != 0
    }
}
impl RND_IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RND_IE_A {
        match self.bits {
            false => RND_IE_A::DISABLE,
            true => RND_IE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RND_IE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RND_IE_A::ENABLE
    }
}
#[doc = "Field `RND_IE` writer - To enable IRQ generation when a new 32-bit Random number is ready."]
pub type RND_IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, RND_IE_A, O>;
impl<'a, const O: u8> RND_IE_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RND_IE_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RND_IE_A::ENABLE)
    }
}
#[doc = "Field `HEALTH_EN` reader - Enable IRQ generation when a health test fails."]
pub type HEALTH_EN_R = crate::BitReader<bool>;
#[doc = "Field `HEALTH_EN` writer - Enable IRQ generation when a health test fails."]
pub type HEALTH_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `KEYGEN` reader - AES Key Generate. When enabled, the key for securing NVSRAM is generated and transferred to the secure key register automatically without user visibility or intervention. This bit is cleared by hardware once the key has been transferred to the secure key register."]
pub type KEYGEN_R = crate::BitReader<bool>;
#[doc = "Field `KEYGEN` writer - AES Key Generate. When enabled, the key for securing NVSRAM is generated and transferred to the secure key register automatically without user visibility or intervention. This bit is cleared by hardware once the key has been transferred to the secure key register."]
pub type KEYGEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `KEYWIPE` reader - To wipe the Battery Backed key."]
pub type KEYWIPE_R = crate::BitReader<bool>;
#[doc = "Field `KEYWIPE` writer - To wipe the Battery Backed key."]
pub type KEYWIPE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Start On-Demand health test."]
    #[inline(always)]
    pub fn odht(&self) -> ODHT_R {
        ODHT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - To enable IRQ generation when a new 32-bit Random number is ready."]
    #[inline(always)]
    pub fn rnd_ie(&self) -> RND_IE_R {
        RND_IE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable IRQ generation when a health test fails."]
    #[inline(always)]
    pub fn health_en(&self) -> HEALTH_EN_R {
        HEALTH_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - AES Key Generate. When enabled, the key for securing NVSRAM is generated and transferred to the secure key register automatically without user visibility or intervention. This bit is cleared by hardware once the key has been transferred to the secure key register."]
    #[inline(always)]
    pub fn keygen(&self) -> KEYGEN_R {
        KEYGEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 15 - To wipe the Battery Backed key."]
    #[inline(always)]
    pub fn keywipe(&self) -> KEYWIPE_R {
        KEYWIPE_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Start On-Demand health test."]
    #[inline(always)]
    #[must_use]
    pub fn odht(&mut self) -> ODHT_W<0> {
        ODHT_W::new(self)
    }
    #[doc = "Bit 1 - To enable IRQ generation when a new 32-bit Random number is ready."]
    #[inline(always)]
    #[must_use]
    pub fn rnd_ie(&mut self) -> RND_IE_W<1> {
        RND_IE_W::new(self)
    }
    #[doc = "Bit 2 - Enable IRQ generation when a health test fails."]
    #[inline(always)]
    #[must_use]
    pub fn health_en(&mut self) -> HEALTH_EN_W<2> {
        HEALTH_EN_W::new(self)
    }
    #[doc = "Bit 3 - AES Key Generate. When enabled, the key for securing NVSRAM is generated and transferred to the secure key register automatically without user visibility or intervention. This bit is cleared by hardware once the key has been transferred to the secure key register."]
    #[inline(always)]
    #[must_use]
    pub fn keygen(&mut self) -> KEYGEN_W<3> {
        KEYGEN_W::new(self)
    }
    #[doc = "Bit 15 - To wipe the Battery Backed key."]
    #[inline(always)]
    #[must_use]
    pub fn keywipe(&mut self) -> KEYWIPE_W<15> {
        KEYWIPE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TRNG Control Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL to value 0x03"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x03;
}
