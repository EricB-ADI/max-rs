#[doc = "Register `GPIO3_CTRL` reader"]
pub struct R(crate::R<GPIO3_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIO3_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIO3_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIO3_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIO3_CTRL` writer"]
pub struct W(crate::W<GPIO3_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIO3_CTRL_SPEC>;
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
impl From<crate::W<GPIO3_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIO3_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P30_DO` reader - GPIO3 Pin 0 Data Output."]
pub type P30_DO_R = crate::BitReader<bool>;
#[doc = "Field `P30_DO` writer - GPIO3 Pin 0 Data Output."]
pub type P30_DO_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO3_CTRL_SPEC, bool, O>;
#[doc = "Field `P30_OE` reader - GPIO3 Pin 0 Output Enable."]
pub type P30_OE_R = crate::BitReader<bool>;
#[doc = "Field `P30_OE` writer - GPIO3 Pin 0 Output Enable."]
pub type P30_OE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO3_CTRL_SPEC, bool, O>;
#[doc = "Field `P30_PE` reader - GPIO3 Pin 0 Pull-up Enable."]
pub type P30_PE_R = crate::BitReader<bool>;
#[doc = "Field `P30_PE` writer - GPIO3 Pin 0 Pull-up Enable."]
pub type P30_PE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO3_CTRL_SPEC, bool, O>;
#[doc = "Field `P30_IN` reader - GPIO3 Pin 0 Input Status."]
pub type P30_IN_R = crate::BitReader<bool>;
#[doc = "Field `P30_IN` writer - GPIO3 Pin 0 Input Status."]
pub type P30_IN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO3_CTRL_SPEC, bool, O>;
#[doc = "Field `P31_DO` reader - GPIO3 Pin 1 Data Output."]
pub type P31_DO_R = crate::BitReader<bool>;
#[doc = "Field `P31_DO` writer - GPIO3 Pin 1 Data Output."]
pub type P31_DO_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO3_CTRL_SPEC, bool, O>;
#[doc = "Field `P31_OE` reader - GPIO3 Pin 1 Output Enable."]
pub type P31_OE_R = crate::BitReader<bool>;
#[doc = "Field `P31_OE` writer - GPIO3 Pin 1 Output Enable."]
pub type P31_OE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO3_CTRL_SPEC, bool, O>;
#[doc = "Field `P31_PE` reader - GPIO3 Pin 1 Pull-up Enable."]
pub type P31_PE_R = crate::BitReader<bool>;
#[doc = "Field `P31_PE` writer - GPIO3 Pin 1 Pull-up Enable."]
pub type P31_PE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO3_CTRL_SPEC, bool, O>;
#[doc = "Field `P31_IN` reader - GPIO3 Pin 1 Input Status."]
pub type P31_IN_R = crate::BitReader<bool>;
#[doc = "Field `P31_IN` writer - GPIO3 Pin 1 Input Status."]
pub type P31_IN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO3_CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - GPIO3 Pin 0 Data Output."]
    #[inline(always)]
    pub fn p30_do(&self) -> P30_DO_R {
        P30_DO_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GPIO3 Pin 0 Output Enable."]
    #[inline(always)]
    pub fn p30_oe(&self) -> P30_OE_R {
        P30_OE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GPIO3 Pin 0 Pull-up Enable."]
    #[inline(always)]
    pub fn p30_pe(&self) -> P30_PE_R {
        P30_PE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GPIO3 Pin 0 Input Status."]
    #[inline(always)]
    pub fn p30_in(&self) -> P30_IN_R {
        P30_IN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - GPIO3 Pin 1 Data Output."]
    #[inline(always)]
    pub fn p31_do(&self) -> P31_DO_R {
        P31_DO_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - GPIO3 Pin 1 Output Enable."]
    #[inline(always)]
    pub fn p31_oe(&self) -> P31_OE_R {
        P31_OE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - GPIO3 Pin 1 Pull-up Enable."]
    #[inline(always)]
    pub fn p31_pe(&self) -> P31_PE_R {
        P31_PE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GPIO3 Pin 1 Input Status."]
    #[inline(always)]
    pub fn p31_in(&self) -> P31_IN_R {
        P31_IN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPIO3 Pin 0 Data Output."]
    #[inline(always)]
    #[must_use]
    pub fn p30_do(&mut self) -> P30_DO_W<0> {
        P30_DO_W::new(self)
    }
    #[doc = "Bit 1 - GPIO3 Pin 0 Output Enable."]
    #[inline(always)]
    #[must_use]
    pub fn p30_oe(&mut self) -> P30_OE_W<1> {
        P30_OE_W::new(self)
    }
    #[doc = "Bit 2 - GPIO3 Pin 0 Pull-up Enable."]
    #[inline(always)]
    #[must_use]
    pub fn p30_pe(&mut self) -> P30_PE_W<2> {
        P30_PE_W::new(self)
    }
    #[doc = "Bit 3 - GPIO3 Pin 0 Input Status."]
    #[inline(always)]
    #[must_use]
    pub fn p30_in(&mut self) -> P30_IN_W<3> {
        P30_IN_W::new(self)
    }
    #[doc = "Bit 4 - GPIO3 Pin 1 Data Output."]
    #[inline(always)]
    #[must_use]
    pub fn p31_do(&mut self) -> P31_DO_W<4> {
        P31_DO_W::new(self)
    }
    #[doc = "Bit 5 - GPIO3 Pin 1 Output Enable."]
    #[inline(always)]
    #[must_use]
    pub fn p31_oe(&mut self) -> P31_OE_W<5> {
        P31_OE_W::new(self)
    }
    #[doc = "Bit 6 - GPIO3 Pin 1 Pull-up Enable."]
    #[inline(always)]
    #[must_use]
    pub fn p31_pe(&mut self) -> P31_PE_W<6> {
        P31_PE_W::new(self)
    }
    #[doc = "Bit 7 - GPIO3 Pin 1 Input Status."]
    #[inline(always)]
    #[must_use]
    pub fn p31_in(&mut self) -> P31_IN_W<7> {
        P31_IN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO3 Pin Control Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio3_ctrl](index.html) module"]
pub struct GPIO3_CTRL_SPEC;
impl crate::RegisterSpec for GPIO3_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpio3_ctrl::R](R) reader structure"]
impl crate::Readable for GPIO3_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpio3_ctrl::W](W) writer structure"]
impl crate::Writable for GPIO3_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPIO3_CTRL to value 0"]
impl crate::Resettable for GPIO3_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
