#[doc = "Register `RESTART` reader"]
pub struct R(crate::R<RESTART_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESTART_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RESTART_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RESTART_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RESTART` writer"]
pub struct W(crate::W<RESTART_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RESTART_SPEC>;
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
impl From<crate::W<RESTART_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RESTART_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pt_x_select` reader - Auto-Restart PT X Select"]
pub type PT_X_SELECT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pt_x_select` writer - Auto-Restart PT X Select"]
pub type PT_X_SELECT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RESTART_SPEC, u8, u8, 5, O>;
#[doc = "Field `on_pt_x_loop_exit` reader - Enable Auto-Restart on PT X Loop Exit"]
pub type ON_PT_X_LOOP_EXIT_R = crate::BitReader<bool>;
#[doc = "Field `on_pt_x_loop_exit` writer - Enable Auto-Restart on PT X Loop Exit"]
pub type ON_PT_X_LOOP_EXIT_W<'a, const O: u8> = crate::BitWriter<'a, u32, RESTART_SPEC, bool, O>;
#[doc = "Field `pt_y_select` reader - Auto-Restart PT Y Select"]
pub type PT_Y_SELECT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pt_y_select` writer - Auto-Restart PT Y Select"]
pub type PT_Y_SELECT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RESTART_SPEC, u8, u8, 5, O>;
#[doc = "Field `on_pt_y_loop_exit` reader - Enable Auto-Restart on PT Y Loop Exit"]
pub type ON_PT_Y_LOOP_EXIT_R = crate::BitReader<bool>;
#[doc = "Field `on_pt_y_loop_exit` writer - Enable Auto-Restart on PT Y Loop Exit"]
pub type ON_PT_Y_LOOP_EXIT_W<'a, const O: u8> = crate::BitWriter<'a, u32, RESTART_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:4 - Auto-Restart PT X Select"]
    #[inline(always)]
    pub fn pt_x_select(&self) -> PT_X_SELECT_R {
        PT_X_SELECT_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 7 - Enable Auto-Restart on PT X Loop Exit"]
    #[inline(always)]
    pub fn on_pt_x_loop_exit(&self) -> ON_PT_X_LOOP_EXIT_R {
        ON_PT_X_LOOP_EXIT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:12 - Auto-Restart PT Y Select"]
    #[inline(always)]
    pub fn pt_y_select(&self) -> PT_Y_SELECT_R {
        PT_Y_SELECT_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 15 - Enable Auto-Restart on PT Y Loop Exit"]
    #[inline(always)]
    pub fn on_pt_y_loop_exit(&self) -> ON_PT_Y_LOOP_EXIT_R {
        ON_PT_Y_LOOP_EXIT_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Auto-Restart PT X Select"]
    #[inline(always)]
    #[must_use]
    pub fn pt_x_select(&mut self) -> PT_X_SELECT_W<0> {
        PT_X_SELECT_W::new(self)
    }
    #[doc = "Bit 7 - Enable Auto-Restart on PT X Loop Exit"]
    #[inline(always)]
    #[must_use]
    pub fn on_pt_x_loop_exit(&mut self) -> ON_PT_X_LOOP_EXIT_W<7> {
        ON_PT_X_LOOP_EXIT_W::new(self)
    }
    #[doc = "Bits 8:12 - Auto-Restart PT Y Select"]
    #[inline(always)]
    #[must_use]
    pub fn pt_y_select(&mut self) -> PT_Y_SELECT_W<8> {
        PT_Y_SELECT_W::new(self)
    }
    #[doc = "Bit 15 - Enable Auto-Restart on PT Y Loop Exit"]
    #[inline(always)]
    #[must_use]
    pub fn on_pt_y_loop_exit(&mut self) -> ON_PT_Y_LOOP_EXIT_W<15> {
        ON_PT_Y_LOOP_EXIT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pulse Train Auto-Restart Configuration.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [restart](index.html) module"]
pub struct RESTART_SPEC;
impl crate::RegisterSpec for RESTART_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [restart::R](R) reader structure"]
impl crate::Readable for RESTART_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [restart::W](W) writer structure"]
impl crate::Writable for RESTART_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RESTART to value 0"]
impl crate::Resettable for RESTART_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
