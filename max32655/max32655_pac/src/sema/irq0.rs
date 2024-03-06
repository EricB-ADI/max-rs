#[doc = "Register `irq0` reader"]
pub struct R(crate::R<IRQ0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IRQ0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IRQ0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IRQ0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `irq0` writer"]
pub struct W(crate::W<IRQ0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IRQ0_SPEC>;
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
impl From<crate::W<IRQ0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IRQ0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `en` reader - "]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `en` writer - "]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQ0_SPEC, bool, O>;
#[doc = "Field `cm4_irq` reader - "]
pub type CM4_IRQ_R = crate::BitReader<bool>;
#[doc = "Field `cm4_irq` writer - "]
pub type CM4_IRQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQ0_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn cm4_irq(&self) -> CM4_IRQ_R {
        CM4_IRQ_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn cm4_irq(&mut self) -> CM4_IRQ_W<16> {
        CM4_IRQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Semaphore IRQ0 register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irq0](index.html) module"]
pub struct IRQ0_SPEC;
impl crate::RegisterSpec for IRQ0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [irq0::R](R) reader structure"]
impl crate::Readable for IRQ0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [irq0::W](W) writer structure"]
impl crate::Writable for IRQ0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets irq0 to value 0"]
impl crate::Resettable for IRQ0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
