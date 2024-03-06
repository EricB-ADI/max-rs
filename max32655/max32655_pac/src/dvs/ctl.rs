#[doc = "Register `CTL` reader"]
pub struct R(crate::R<CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTL` writer"]
pub struct W(crate::W<CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTL_SPEC>;
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
impl From<crate::W<CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MON_ENA` reader - Enable the DVS monitoring circuit"]
pub type MON_ENA_R = crate::BitReader<bool>;
#[doc = "Field `MON_ENA` writer - Enable the DVS monitoring circuit"]
pub type MON_ENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
#[doc = "Field `ADJ_ENA` reader - Enable the power supply adjustment based on measurements"]
pub type ADJ_ENA_R = crate::BitReader<bool>;
#[doc = "Field `ADJ_ENA` writer - Enable the power supply adjustment based on measurements"]
pub type ADJ_ENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
#[doc = "Field `PS_FB_DIS` reader - Power Supply Feedback Disable"]
pub type PS_FB_DIS_R = crate::BitReader<bool>;
#[doc = "Field `PS_FB_DIS` writer - Power Supply Feedback Disable"]
pub type PS_FB_DIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
#[doc = "Field `CTRL_TAP_ENA` reader - Use the TAP Select for automatic adjustment or monitoring"]
pub type CTRL_TAP_ENA_R = crate::BitReader<bool>;
#[doc = "Field `CTRL_TAP_ENA` writer - Use the TAP Select for automatic adjustment or monitoring"]
pub type CTRL_TAP_ENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
#[doc = "Field `PROP_DLY` reader - Additional delay to monitor lines"]
pub type PROP_DLY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PROP_DLY` writer - Additional delay to monitor lines"]
pub type PROP_DLY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTL_SPEC, u8, u8, 2, O>;
#[doc = "Field `MON_ONESHOT` reader - Measure delay once"]
pub type MON_ONESHOT_R = crate::BitReader<bool>;
#[doc = "Field `MON_ONESHOT` writer - Measure delay once"]
pub type MON_ONESHOT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
#[doc = "Field `GO_DIRECT` reader - Operate in automatic mode or move directly"]
pub type GO_DIRECT_R = crate::BitReader<bool>;
#[doc = "Field `GO_DIRECT` writer - Operate in automatic mode or move directly"]
pub type GO_DIRECT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
#[doc = "Field `DIRECT_REG` reader - Step incrementally to target voltage"]
pub type DIRECT_REG_R = crate::BitReader<bool>;
#[doc = "Field `DIRECT_REG` writer - Step incrementally to target voltage"]
pub type DIRECT_REG_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
#[doc = "Field `PRIME_ENA` reader - Include a delay line priming signal before monitoring"]
pub type PRIME_ENA_R = crate::BitReader<bool>;
#[doc = "Field `PRIME_ENA` writer - Include a delay line priming signal before monitoring"]
pub type PRIME_ENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
#[doc = "Field `LIMIT_IE` reader - Enable Limit Error Interrupt"]
pub type LIMIT_IE_R = crate::BitReader<bool>;
#[doc = "Field `LIMIT_IE` writer - Enable Limit Error Interrupt"]
pub type LIMIT_IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
#[doc = "Field `RANGE_IE` reader - Enable Range Error Interrupt"]
pub type RANGE_IE_R = crate::BitReader<bool>;
#[doc = "Field `RANGE_IE` writer - Enable Range Error Interrupt"]
pub type RANGE_IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
#[doc = "Field `ADJ_IE` reader - Enable Adjustment Error Interrupt"]
pub type ADJ_IE_R = crate::BitReader<bool>;
#[doc = "Field `ADJ_IE` writer - Enable Adjustment Error Interrupt"]
pub type ADJ_IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
#[doc = "Field `REF_SEL` reader - Select TAP used for voltage adjustment"]
pub type REF_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `REF_SEL` writer - Select TAP used for voltage adjustment"]
pub type REF_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTL_SPEC, u8, u8, 4, O>;
#[doc = "Field `INC_VAL` reader - Step size to increment voltage when in automatic mode"]
pub type INC_VAL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INC_VAL` writer - Step size to increment voltage when in automatic mode"]
pub type INC_VAL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTL_SPEC, u8, u8, 3, O>;
#[doc = "Field `DVS_PS_APB_DIS` reader - Prevent the application code from adjusting Vcore"]
pub type DVS_PS_APB_DIS_R = crate::BitReader<bool>;
#[doc = "Field `DVS_PS_APB_DIS` writer - Prevent the application code from adjusting Vcore"]
pub type DVS_PS_APB_DIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
#[doc = "Field `DVS_HI_RANGE_ANY` reader - Any high range signal from a delay line will cause a voltage adjustment"]
pub type DVS_HI_RANGE_ANY_R = crate::BitReader<bool>;
#[doc = "Field `DVS_HI_RANGE_ANY` writer - Any high range signal from a delay line will cause a voltage adjustment"]
pub type DVS_HI_RANGE_ANY_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
#[doc = "Field `FB_TO_IE` reader - Enable Voltage Adjustment Timeout Interrupt"]
pub type FB_TO_IE_R = crate::BitReader<bool>;
#[doc = "Field `FB_TO_IE` writer - Enable Voltage Adjustment Timeout Interrupt"]
pub type FB_TO_IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
#[doc = "Field `FC_LV_IE` reader - Enable Low Voltage Interrupt"]
pub type FC_LV_IE_R = crate::BitReader<bool>;
#[doc = "Field `FC_LV_IE` writer - Enable Low Voltage Interrupt"]
pub type FC_LV_IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
#[doc = "Field `PD_ACK_ENA` reader - Prevent DVS from ack'ing a request to enter a low power mode until in the idle state"]
pub type PD_ACK_ENA_R = crate::BitReader<bool>;
#[doc = "Field `PD_ACK_ENA` writer - Prevent DVS from ack'ing a request to enter a low power mode until in the idle state"]
pub type PD_ACK_ENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
#[doc = "Field `ADJ_ABORT` reader - Causes the DVS to enter the idle state immediately on a request to enter a low power mode"]
pub type ADJ_ABORT_R = crate::BitReader<bool>;
#[doc = "Field `ADJ_ABORT` writer - Causes the DVS to enter the idle state immediately on a request to enter a low power mode"]
pub type ADJ_ABORT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Enable the DVS monitoring circuit"]
    #[inline(always)]
    pub fn mon_ena(&self) -> MON_ENA_R {
        MON_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable the power supply adjustment based on measurements"]
    #[inline(always)]
    pub fn adj_ena(&self) -> ADJ_ENA_R {
        ADJ_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Power Supply Feedback Disable"]
    #[inline(always)]
    pub fn ps_fb_dis(&self) -> PS_FB_DIS_R {
        PS_FB_DIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Use the TAP Select for automatic adjustment or monitoring"]
    #[inline(always)]
    pub fn ctrl_tap_ena(&self) -> CTRL_TAP_ENA_R {
        CTRL_TAP_ENA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Additional delay to monitor lines"]
    #[inline(always)]
    pub fn prop_dly(&self) -> PROP_DLY_R {
        PROP_DLY_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Measure delay once"]
    #[inline(always)]
    pub fn mon_oneshot(&self) -> MON_ONESHOT_R {
        MON_ONESHOT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Operate in automatic mode or move directly"]
    #[inline(always)]
    pub fn go_direct(&self) -> GO_DIRECT_R {
        GO_DIRECT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Step incrementally to target voltage"]
    #[inline(always)]
    pub fn direct_reg(&self) -> DIRECT_REG_R {
        DIRECT_REG_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Include a delay line priming signal before monitoring"]
    #[inline(always)]
    pub fn prime_ena(&self) -> PRIME_ENA_R {
        PRIME_ENA_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable Limit Error Interrupt"]
    #[inline(always)]
    pub fn limit_ie(&self) -> LIMIT_IE_R {
        LIMIT_IE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable Range Error Interrupt"]
    #[inline(always)]
    pub fn range_ie(&self) -> RANGE_IE_R {
        RANGE_IE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable Adjustment Error Interrupt"]
    #[inline(always)]
    pub fn adj_ie(&self) -> ADJ_IE_R {
        ADJ_IE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:16 - Select TAP used for voltage adjustment"]
    #[inline(always)]
    pub fn ref_sel(&self) -> REF_SEL_R {
        REF_SEL_R::new(((self.bits >> 13) & 0x0f) as u8)
    }
    #[doc = "Bits 17:19 - Step size to increment voltage when in automatic mode"]
    #[inline(always)]
    pub fn inc_val(&self) -> INC_VAL_R {
        INC_VAL_R::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bit 20 - Prevent the application code from adjusting Vcore"]
    #[inline(always)]
    pub fn dvs_ps_apb_dis(&self) -> DVS_PS_APB_DIS_R {
        DVS_PS_APB_DIS_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Any high range signal from a delay line will cause a voltage adjustment"]
    #[inline(always)]
    pub fn dvs_hi_range_any(&self) -> DVS_HI_RANGE_ANY_R {
        DVS_HI_RANGE_ANY_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Enable Voltage Adjustment Timeout Interrupt"]
    #[inline(always)]
    pub fn fb_to_ie(&self) -> FB_TO_IE_R {
        FB_TO_IE_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Enable Low Voltage Interrupt"]
    #[inline(always)]
    pub fn fc_lv_ie(&self) -> FC_LV_IE_R {
        FC_LV_IE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Prevent DVS from ack'ing a request to enter a low power mode until in the idle state"]
    #[inline(always)]
    pub fn pd_ack_ena(&self) -> PD_ACK_ENA_R {
        PD_ACK_ENA_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Causes the DVS to enter the idle state immediately on a request to enter a low power mode"]
    #[inline(always)]
    pub fn adj_abort(&self) -> ADJ_ABORT_R {
        ADJ_ABORT_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable the DVS monitoring circuit"]
    #[inline(always)]
    #[must_use]
    pub fn mon_ena(&mut self) -> MON_ENA_W<0> {
        MON_ENA_W::new(self)
    }
    #[doc = "Bit 1 - Enable the power supply adjustment based on measurements"]
    #[inline(always)]
    #[must_use]
    pub fn adj_ena(&mut self) -> ADJ_ENA_W<1> {
        ADJ_ENA_W::new(self)
    }
    #[doc = "Bit 2 - Power Supply Feedback Disable"]
    #[inline(always)]
    #[must_use]
    pub fn ps_fb_dis(&mut self) -> PS_FB_DIS_W<2> {
        PS_FB_DIS_W::new(self)
    }
    #[doc = "Bit 3 - Use the TAP Select for automatic adjustment or monitoring"]
    #[inline(always)]
    #[must_use]
    pub fn ctrl_tap_ena(&mut self) -> CTRL_TAP_ENA_W<3> {
        CTRL_TAP_ENA_W::new(self)
    }
    #[doc = "Bits 4:5 - Additional delay to monitor lines"]
    #[inline(always)]
    #[must_use]
    pub fn prop_dly(&mut self) -> PROP_DLY_W<4> {
        PROP_DLY_W::new(self)
    }
    #[doc = "Bit 6 - Measure delay once"]
    #[inline(always)]
    #[must_use]
    pub fn mon_oneshot(&mut self) -> MON_ONESHOT_W<6> {
        MON_ONESHOT_W::new(self)
    }
    #[doc = "Bit 7 - Operate in automatic mode or move directly"]
    #[inline(always)]
    #[must_use]
    pub fn go_direct(&mut self) -> GO_DIRECT_W<7> {
        GO_DIRECT_W::new(self)
    }
    #[doc = "Bit 8 - Step incrementally to target voltage"]
    #[inline(always)]
    #[must_use]
    pub fn direct_reg(&mut self) -> DIRECT_REG_W<8> {
        DIRECT_REG_W::new(self)
    }
    #[doc = "Bit 9 - Include a delay line priming signal before monitoring"]
    #[inline(always)]
    #[must_use]
    pub fn prime_ena(&mut self) -> PRIME_ENA_W<9> {
        PRIME_ENA_W::new(self)
    }
    #[doc = "Bit 10 - Enable Limit Error Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn limit_ie(&mut self) -> LIMIT_IE_W<10> {
        LIMIT_IE_W::new(self)
    }
    #[doc = "Bit 11 - Enable Range Error Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn range_ie(&mut self) -> RANGE_IE_W<11> {
        RANGE_IE_W::new(self)
    }
    #[doc = "Bit 12 - Enable Adjustment Error Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn adj_ie(&mut self) -> ADJ_IE_W<12> {
        ADJ_IE_W::new(self)
    }
    #[doc = "Bits 13:16 - Select TAP used for voltage adjustment"]
    #[inline(always)]
    #[must_use]
    pub fn ref_sel(&mut self) -> REF_SEL_W<13> {
        REF_SEL_W::new(self)
    }
    #[doc = "Bits 17:19 - Step size to increment voltage when in automatic mode"]
    #[inline(always)]
    #[must_use]
    pub fn inc_val(&mut self) -> INC_VAL_W<17> {
        INC_VAL_W::new(self)
    }
    #[doc = "Bit 20 - Prevent the application code from adjusting Vcore"]
    #[inline(always)]
    #[must_use]
    pub fn dvs_ps_apb_dis(&mut self) -> DVS_PS_APB_DIS_W<20> {
        DVS_PS_APB_DIS_W::new(self)
    }
    #[doc = "Bit 21 - Any high range signal from a delay line will cause a voltage adjustment"]
    #[inline(always)]
    #[must_use]
    pub fn dvs_hi_range_any(&mut self) -> DVS_HI_RANGE_ANY_W<21> {
        DVS_HI_RANGE_ANY_W::new(self)
    }
    #[doc = "Bit 22 - Enable Voltage Adjustment Timeout Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn fb_to_ie(&mut self) -> FB_TO_IE_W<22> {
        FB_TO_IE_W::new(self)
    }
    #[doc = "Bit 23 - Enable Low Voltage Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn fc_lv_ie(&mut self) -> FC_LV_IE_W<23> {
        FC_LV_IE_W::new(self)
    }
    #[doc = "Bit 24 - Prevent DVS from ack'ing a request to enter a low power mode until in the idle state"]
    #[inline(always)]
    #[must_use]
    pub fn pd_ack_ena(&mut self) -> PD_ACK_ENA_W<24> {
        PD_ACK_ENA_W::new(self)
    }
    #[doc = "Bit 25 - Causes the DVS to enter the idle state immediately on a request to enter a low power mode"]
    #[inline(always)]
    #[must_use]
    pub fn adj_abort(&mut self) -> ADJ_ABORT_W<25> {
        ADJ_ABORT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl](index.html) module"]
pub struct CTL_SPEC;
impl crate::RegisterSpec for CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctl::R](R) reader structure"]
impl crate::Readable for CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctl::W](W) writer structure"]
impl crate::Writable for CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTL to value 0"]
impl crate::Resettable for CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
