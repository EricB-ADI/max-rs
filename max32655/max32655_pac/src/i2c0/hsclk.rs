#[doc = "Register `HSCLK` reader"]
pub struct R(crate::R<HSCLK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HSCLK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HSCLK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HSCLK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HSCLK` writer"]
pub struct W(crate::W<HSCLK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HSCLK_SPEC>;
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
impl From<crate::W<HSCLK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HSCLK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LO` reader - Clock Low. This field sets the Hs-Mode clock low count. In Slave mode, this is the time SCL is held low after data is output on SDA."]
pub type LO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LO` writer - Clock Low. This field sets the Hs-Mode clock low count. In Slave mode, this is the time SCL is held low after data is output on SDA."]
pub type LO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HSCLK_SPEC, u8, u8, 8, O>;
#[doc = "Field `HI` reader - Clock High. This field sets the Hs-Mode clock high count. In Slave mode, this is the time SCL is held high after data is output on SDA"]
pub type HI_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HI` writer - Clock High. This field sets the Hs-Mode clock high count. In Slave mode, this is the time SCL is held high after data is output on SDA"]
pub type HI_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HSCLK_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Clock Low. This field sets the Hs-Mode clock low count. In Slave mode, this is the time SCL is held low after data is output on SDA."]
    #[inline(always)]
    pub fn lo(&self) -> LO_R {
        LO_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Clock High. This field sets the Hs-Mode clock high count. In Slave mode, this is the time SCL is held high after data is output on SDA"]
    #[inline(always)]
    pub fn hi(&self) -> HI_R {
        HI_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Clock Low. This field sets the Hs-Mode clock low count. In Slave mode, this is the time SCL is held low after data is output on SDA."]
    #[inline(always)]
    #[must_use]
    pub fn lo(&mut self) -> LO_W<0> {
        LO_W::new(self)
    }
    #[doc = "Bits 8:15 - Clock High. This field sets the Hs-Mode clock high count. In Slave mode, this is the time SCL is held high after data is output on SDA"]
    #[inline(always)]
    #[must_use]
    pub fn hi(&mut self) -> HI_W<8> {
        HI_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock high Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hsclk](index.html) module"]
pub struct HSCLK_SPEC;
impl crate::RegisterSpec for HSCLK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hsclk::R](R) reader structure"]
impl crate::Readable for HSCLK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hsclk::W](W) writer structure"]
impl crate::Writable for HSCLK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HSCLK to value 0"]
impl crate::Resettable for HSCLK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
