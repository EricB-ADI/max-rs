#[doc = "Register `CNTRLD` reader"]
pub struct R(crate::R<CNTRLD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CNTRLD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CNTRLD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CNTRLD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CNTRLD` writer"]
pub struct W(crate::W<CNTRLD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CNTRLD_SPEC>;
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
impl From<crate::W<CNTRLD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CNTRLD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CNT` reader - Count Reload Value. The value of this register is loaded into DMA0_CNT upon a count-to-zero condition."]
pub type CNT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CNT` writer - Count Reload Value. The value of this register is loaded into DMA0_CNT upon a count-to-zero condition."]
pub type CNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CNTRLD_SPEC, u32, u32, 24, O>;
#[doc = "Field `EN` reader - Reload Enable. This bit should be set after the address reload registers have been programmed. This bit is automatically cleared to 0 when reload occurs."]
pub type EN_R = crate::BitReader<EN_A>;
#[doc = "Reload Enable. This bit should be set after the address reload registers have been programmed. This bit is automatically cleared to 0 when reload occurs.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EN_A {
    #[doc = "0: Disable."]
    DIS = 0,
    #[doc = "1: Enable."]
    EN = 1,
}
impl From<EN_A> for bool {
    #[inline(always)]
    fn from(variant: EN_A) -> Self {
        variant as u8 != 0
    }
}
impl EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN_A {
        match self.bits {
            false => EN_A::DIS,
            true => EN_A::EN,
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
#[doc = "Field `EN` writer - Reload Enable. This bit should be set after the address reload registers have been programmed. This bit is automatically cleared to 0 when reload occurs."]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CNTRLD_SPEC, EN_A, O>;
impl<'a, const O: u8> EN_W<'a, O> {
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(EN_A::DIS)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(EN_A::EN)
    }
}
impl R {
    #[doc = "Bits 0:23 - Count Reload Value. The value of this register is loaded into DMA0_CNT upon a count-to-zero condition."]
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bit 31 - Reload Enable. This bit should be set after the address reload registers have been programmed. This bit is automatically cleared to 0 when reload occurs."]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:23 - Count Reload Value. The value of this register is loaded into DMA0_CNT upon a count-to-zero condition."]
    #[inline(always)]
    #[must_use]
    pub fn cnt(&mut self) -> CNT_W<0> {
        CNT_W::new(self)
    }
    #[doc = "Bit 31 - Reload Enable. This bit should be set after the address reload registers have been programmed. This bit is automatically cleared to 0 when reload occurs."]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<31> {
        EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Channel Count Reload Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cntrld](index.html) module"]
pub struct CNTRLD_SPEC;
impl crate::RegisterSpec for CNTRLD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cntrld::R](R) reader structure"]
impl crate::Readable for CNTRLD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cntrld::W](W) writer structure"]
impl crate::Writable for CNTRLD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CNTRLD to value 0"]
impl crate::Resettable for CNTRLD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
