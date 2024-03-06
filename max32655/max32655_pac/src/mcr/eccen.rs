#[doc = "Register `ECCEN` reader"]
pub struct R(crate::R<ECCEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ECCEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ECCEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ECCEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ECCEN` writer"]
pub struct W(crate::W<ECCEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ECCEN_SPEC>;
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
impl From<crate::W<ECCEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ECCEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RAM0` reader - ECC System RAM0 Enable."]
pub type RAM0_R = crate::BitReader<RAM0_A>;
#[doc = "ECC System RAM0 Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RAM0_A {
    #[doc = "0: disabled."]
    DIS = 0,
    #[doc = "1: enabled."]
    EN = 1,
}
impl From<RAM0_A> for bool {
    #[inline(always)]
    fn from(variant: RAM0_A) -> Self {
        variant as u8 != 0
    }
}
impl RAM0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RAM0_A {
        match self.bits {
            false => RAM0_A::DIS,
            true => RAM0_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == RAM0_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == RAM0_A::EN
    }
}
#[doc = "Field `RAM0` writer - ECC System RAM0 Enable."]
pub type RAM0_W<'a, const O: u8> = crate::BitWriter<'a, u32, ECCEN_SPEC, RAM0_A, O>;
impl<'a, const O: u8> RAM0_W<'a, O> {
    #[doc = "disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(RAM0_A::DIS)
    }
    #[doc = "enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(RAM0_A::EN)
    }
}
impl R {
    #[doc = "Bit 0 - ECC System RAM0 Enable."]
    #[inline(always)]
    pub fn ram0(&self) -> RAM0_R {
        RAM0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ECC System RAM0 Enable."]
    #[inline(always)]
    #[must_use]
    pub fn ram0(&mut self) -> RAM0_W<0> {
        RAM0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ECC Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eccen](index.html) module"]
pub struct ECCEN_SPEC;
impl crate::RegisterSpec for ECCEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eccen::R](R) reader structure"]
impl crate::Readable for ECCEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eccen::W](W) writer structure"]
impl crate::Writable for ECCEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ECCEN to value 0"]
impl crate::Resettable for ECCEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
