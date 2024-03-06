#[doc = "Register `INTR` reader"]
pub struct R(crate::R<INTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTR` writer"]
pub struct W(crate::W<INTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTR_SPEC>;
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
impl From<crate::W<INTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `done_ie` reader - ADC Done Interrupt Enable"]
pub type DONE_IE_R = crate::BitReader<bool>;
#[doc = "Field `done_ie` writer - ADC Done Interrupt Enable"]
pub type DONE_IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `ref_ready_ie` reader - ADC Reference Ready Interrupt Enable"]
pub type REF_READY_IE_R = crate::BitReader<bool>;
#[doc = "Field `ref_ready_ie` writer - ADC Reference Ready Interrupt Enable"]
pub type REF_READY_IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `hi_limit_ie` reader - ADC Hi Limit Monitor Interrupt Enable"]
pub type HI_LIMIT_IE_R = crate::BitReader<bool>;
#[doc = "Field `hi_limit_ie` writer - ADC Hi Limit Monitor Interrupt Enable"]
pub type HI_LIMIT_IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `lo_limit_ie` reader - ADC Lo Limit Monitor Interrupt Enable"]
pub type LO_LIMIT_IE_R = crate::BitReader<bool>;
#[doc = "Field `lo_limit_ie` writer - ADC Lo Limit Monitor Interrupt Enable"]
pub type LO_LIMIT_IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `overflow_ie` reader - ADC Overflow Interrupt Enable"]
pub type OVERFLOW_IE_R = crate::BitReader<bool>;
#[doc = "Field `overflow_ie` writer - ADC Overflow Interrupt Enable"]
pub type OVERFLOW_IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `done_if` reader - ADC Done Interrupt Flag"]
pub type DONE_IF_R = crate::BitReader<bool>;
#[doc = "Field `done_if` writer - ADC Done Interrupt Flag"]
pub type DONE_IF_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `ref_ready_if` reader - ADC Reference Ready Interrupt Flag"]
pub type REF_READY_IF_R = crate::BitReader<bool>;
#[doc = "Field `ref_ready_if` writer - ADC Reference Ready Interrupt Flag"]
pub type REF_READY_IF_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `hi_limit_if` reader - ADC Hi Limit Monitor Interrupt Flag"]
pub type HI_LIMIT_IF_R = crate::BitReader<bool>;
#[doc = "Field `hi_limit_if` writer - ADC Hi Limit Monitor Interrupt Flag"]
pub type HI_LIMIT_IF_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `lo_limit_if` reader - ADC Lo Limit Monitor Interrupt Flag"]
pub type LO_LIMIT_IF_R = crate::BitReader<bool>;
#[doc = "Field `lo_limit_if` writer - ADC Lo Limit Monitor Interrupt Flag"]
pub type LO_LIMIT_IF_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `overflow_if` reader - ADC Overflow Interrupt Flag"]
pub type OVERFLOW_IF_R = crate::BitReader<bool>;
#[doc = "Field `overflow_if` writer - ADC Overflow Interrupt Flag"]
pub type OVERFLOW_IF_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `pending` reader - ADC Interrupt Pending Status"]
pub type PENDING_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - ADC Done Interrupt Enable"]
    #[inline(always)]
    pub fn done_ie(&self) -> DONE_IE_R {
        DONE_IE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADC Reference Ready Interrupt Enable"]
    #[inline(always)]
    pub fn ref_ready_ie(&self) -> REF_READY_IE_R {
        REF_READY_IE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ADC Hi Limit Monitor Interrupt Enable"]
    #[inline(always)]
    pub fn hi_limit_ie(&self) -> HI_LIMIT_IE_R {
        HI_LIMIT_IE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ADC Lo Limit Monitor Interrupt Enable"]
    #[inline(always)]
    pub fn lo_limit_ie(&self) -> LO_LIMIT_IE_R {
        LO_LIMIT_IE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ADC Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn overflow_ie(&self) -> OVERFLOW_IE_R {
        OVERFLOW_IE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 16 - ADC Done Interrupt Flag"]
    #[inline(always)]
    pub fn done_if(&self) -> DONE_IF_R {
        DONE_IF_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - ADC Reference Ready Interrupt Flag"]
    #[inline(always)]
    pub fn ref_ready_if(&self) -> REF_READY_IF_R {
        REF_READY_IF_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - ADC Hi Limit Monitor Interrupt Flag"]
    #[inline(always)]
    pub fn hi_limit_if(&self) -> HI_LIMIT_IF_R {
        HI_LIMIT_IF_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - ADC Lo Limit Monitor Interrupt Flag"]
    #[inline(always)]
    pub fn lo_limit_if(&self) -> LO_LIMIT_IF_R {
        LO_LIMIT_IF_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - ADC Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn overflow_if(&self) -> OVERFLOW_IF_R {
        OVERFLOW_IF_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 22 - ADC Interrupt Pending Status"]
    #[inline(always)]
    pub fn pending(&self) -> PENDING_R {
        PENDING_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADC Done Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn done_ie(&mut self) -> DONE_IE_W<0> {
        DONE_IE_W::new(self)
    }
    #[doc = "Bit 1 - ADC Reference Ready Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ref_ready_ie(&mut self) -> REF_READY_IE_W<1> {
        REF_READY_IE_W::new(self)
    }
    #[doc = "Bit 2 - ADC Hi Limit Monitor Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hi_limit_ie(&mut self) -> HI_LIMIT_IE_W<2> {
        HI_LIMIT_IE_W::new(self)
    }
    #[doc = "Bit 3 - ADC Lo Limit Monitor Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lo_limit_ie(&mut self) -> LO_LIMIT_IE_W<3> {
        LO_LIMIT_IE_W::new(self)
    }
    #[doc = "Bit 4 - ADC Overflow Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn overflow_ie(&mut self) -> OVERFLOW_IE_W<4> {
        OVERFLOW_IE_W::new(self)
    }
    #[doc = "Bit 16 - ADC Done Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn done_if(&mut self) -> DONE_IF_W<16> {
        DONE_IF_W::new(self)
    }
    #[doc = "Bit 17 - ADC Reference Ready Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ref_ready_if(&mut self) -> REF_READY_IF_W<17> {
        REF_READY_IF_W::new(self)
    }
    #[doc = "Bit 18 - ADC Hi Limit Monitor Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn hi_limit_if(&mut self) -> HI_LIMIT_IF_W<18> {
        HI_LIMIT_IF_W::new(self)
    }
    #[doc = "Bit 19 - ADC Lo Limit Monitor Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn lo_limit_if(&mut self) -> LO_LIMIT_IF_W<19> {
        LO_LIMIT_IF_W::new(self)
    }
    #[doc = "Bit 20 - ADC Overflow Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn overflow_if(&mut self) -> OVERFLOW_IF_W<20> {
        OVERFLOW_IF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Interrupt Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr](index.html) module"]
pub struct INTR_SPEC;
impl crate::RegisterSpec for INTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr::R](R) reader structure"]
impl crate::Readable for INTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intr::W](W) writer structure"]
impl crate::Writable for INTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x001f_0000;
}
#[doc = "`reset()` method sets INTR to value 0"]
impl crate::Resettable for INTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
