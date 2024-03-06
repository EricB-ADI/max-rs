#[doc = "Register `LOOP` reader"]
pub struct R(crate::R<LOOP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LOOP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LOOP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LOOP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LOOP` writer"]
pub struct W(crate::W<LOOP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LOOP_SPEC>;
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
impl From<crate::W<LOOP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LOOP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `count` reader - Number of loops for this pulse train to repeat."]
pub type COUNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `count` writer - Number of loops for this pulse train to repeat."]
pub type COUNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LOOP_SPEC, u16, u16, 16, O>;
#[doc = "Field `delay` reader - Delay between loops of the Pulse Train in PT Peripheral Clock cycles"]
pub type DELAY_R = crate::FieldReader<u16, u16>;
#[doc = "Field `delay` writer - Delay between loops of the Pulse Train in PT Peripheral Clock cycles"]
pub type DELAY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LOOP_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 0:15 - Number of loops for this pulse train to repeat."]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:27 - Delay between loops of the Pulse Train in PT Peripheral Clock cycles"]
    #[inline(always)]
    pub fn delay(&self) -> DELAY_R {
        DELAY_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Number of loops for this pulse train to repeat."]
    #[inline(always)]
    #[must_use]
    pub fn count(&mut self) -> COUNT_W<0> {
        COUNT_W::new(self)
    }
    #[doc = "Bits 16:27 - Delay between loops of the Pulse Train in PT Peripheral Clock cycles"]
    #[inline(always)]
    #[must_use]
    pub fn delay(&mut self) -> DELAY_W<16> {
        DELAY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pulse Train Loop Count\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [loop_](index.html) module"]
pub struct LOOP_SPEC;
impl crate::RegisterSpec for LOOP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [loop_::R](R) reader structure"]
impl crate::Readable for LOOP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [loop_::W](W) writer structure"]
impl crate::Writable for LOOP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LOOP to value 0"]
impl crate::Resettable for LOOP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
