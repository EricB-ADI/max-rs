#[doc = "Register `STAT` reader"]
pub struct R(crate::R<STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STAT` writer"]
pub struct W(crate::W<STAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STAT_SPEC>;
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
impl From<crate::W<STAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DVS_STATE` reader - State machine state"]
pub type DVS_STATE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DVS_STATE` writer - State machine state"]
pub type DVS_STATE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, STAT_SPEC, u8, u8, 4, O>;
#[doc = "Field `ADJ_UP_ENA` reader - DVS Raising voltage"]
pub type ADJ_UP_ENA_R = crate::BitReader<bool>;
#[doc = "Field `ADJ_UP_ENA` writer - DVS Raising voltage"]
pub type ADJ_UP_ENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAT_SPEC, bool, O>;
#[doc = "Field `ADJ_DWN_ENA` reader - DVS Lowering voltage"]
pub type ADJ_DWN_ENA_R = crate::BitReader<bool>;
#[doc = "Field `ADJ_DWN_ENA` writer - DVS Lowering voltage"]
pub type ADJ_DWN_ENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAT_SPEC, bool, O>;
#[doc = "Field `ADJ_ACTIVE` reader - Adjustment to a Direct Voltage"]
pub type ADJ_ACTIVE_R = crate::BitReader<bool>;
#[doc = "Field `ADJ_ACTIVE` writer - Adjustment to a Direct Voltage"]
pub type ADJ_ACTIVE_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAT_SPEC, bool, O>;
#[doc = "Field `CTR_TAP_OK` reader - Tap Enabled and the Tap is withing Hi/Low limits"]
pub type CTR_TAP_OK_R = crate::BitReader<bool>;
#[doc = "Field `CTR_TAP_OK` writer - Tap Enabled and the Tap is withing Hi/Low limits"]
pub type CTR_TAP_OK_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAT_SPEC, bool, O>;
#[doc = "Field `CTR_TAP_SEL` reader - Status of selected center tap delay line detect output"]
pub type CTR_TAP_SEL_R = crate::BitReader<bool>;
#[doc = "Field `CTR_TAP_SEL` writer - Status of selected center tap delay line detect output"]
pub type CTR_TAP_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAT_SPEC, bool, O>;
#[doc = "Field `SLOW_TRIP_DET` reader - Provides the current combined status of all selected Low Range delay lines"]
pub type SLOW_TRIP_DET_R = crate::BitReader<bool>;
#[doc = "Field `SLOW_TRIP_DET` writer - Provides the current combined status of all selected Low Range delay lines"]
pub type SLOW_TRIP_DET_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAT_SPEC, bool, O>;
#[doc = "Field `FAST_TRIP_DET` reader - Provides the current combined status of all selected High Range delay lines"]
pub type FAST_TRIP_DET_R = crate::BitReader<bool>;
#[doc = "Field `FAST_TRIP_DET` writer - Provides the current combined status of all selected High Range delay lines"]
pub type FAST_TRIP_DET_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAT_SPEC, bool, O>;
#[doc = "Field `PS_IN_RANGE` reader - Indicates if the power supply is in range"]
pub type PS_IN_RANGE_R = crate::BitReader<bool>;
#[doc = "Field `PS_IN_RANGE` writer - Indicates if the power supply is in range"]
pub type PS_IN_RANGE_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAT_SPEC, bool, O>;
#[doc = "Field `PS_VCNTR` reader - Voltage Count value sent to the power supply"]
pub type PS_VCNTR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PS_VCNTR` writer - Voltage Count value sent to the power supply"]
pub type PS_VCNTR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, STAT_SPEC, u8, u8, 7, O>;
#[doc = "Field `MON_DLY_OK` reader - Indicates the monitor delay count is at 0"]
pub type MON_DLY_OK_R = crate::BitReader<bool>;
#[doc = "Field `MON_DLY_OK` writer - Indicates the monitor delay count is at 0"]
pub type MON_DLY_OK_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAT_SPEC, bool, O>;
#[doc = "Field `ADJ_DLY_OK` reader - Indicates the adjustment delay count is at 0"]
pub type ADJ_DLY_OK_R = crate::BitReader<bool>;
#[doc = "Field `ADJ_DLY_OK` writer - Indicates the adjustment delay count is at 0"]
pub type ADJ_DLY_OK_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAT_SPEC, bool, O>;
#[doc = "Field `LO_LIMIT_DET` reader - Power supply voltage counter is at low limit"]
pub type LO_LIMIT_DET_R = crate::BitReader<bool>;
#[doc = "Field `LO_LIMIT_DET` writer - Power supply voltage counter is at low limit"]
pub type LO_LIMIT_DET_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAT_SPEC, bool, O>;
#[doc = "Field `HI_LIMIT_DET` reader - Power supply voltage counter is at high limit"]
pub type HI_LIMIT_DET_R = crate::BitReader<bool>;
#[doc = "Field `HI_LIMIT_DET` writer - Power supply voltage counter is at high limit"]
pub type HI_LIMIT_DET_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAT_SPEC, bool, O>;
#[doc = "Field `VALID_TAP` reader - At least one delay line has been enabled"]
pub type VALID_TAP_R = crate::BitReader<bool>;
#[doc = "Field `VALID_TAP` writer - At least one delay line has been enabled"]
pub type VALID_TAP_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAT_SPEC, bool, O>;
#[doc = "Field `LIMIT_ERR` reader - Interrupt flag that indicates a voltage count is at/beyond manufacturer limits"]
pub type LIMIT_ERR_R = crate::BitReader<bool>;
#[doc = "Field `LIMIT_ERR` writer - Interrupt flag that indicates a voltage count is at/beyond manufacturer limits"]
pub type LIMIT_ERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAT_SPEC, bool, O>;
#[doc = "Field `RANGE_ERR` reader - Interrupt flag that indicates a tap has an invalid value"]
pub type RANGE_ERR_R = crate::BitReader<bool>;
#[doc = "Field `RANGE_ERR` writer - Interrupt flag that indicates a tap has an invalid value"]
pub type RANGE_ERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAT_SPEC, bool, O>;
#[doc = "Field `ADJ_ERR` reader - Interrupt flag that indicates up and down adjustment requested simultaneously"]
pub type ADJ_ERR_R = crate::BitReader<bool>;
#[doc = "Field `ADJ_ERR` writer - Interrupt flag that indicates up and down adjustment requested simultaneously"]
pub type ADJ_ERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAT_SPEC, bool, O>;
#[doc = "Field `REF_SEL_ERR` reader - Indicates the ref select register bit is out of range"]
pub type REF_SEL_ERR_R = crate::BitReader<bool>;
#[doc = "Field `REF_SEL_ERR` writer - Indicates the ref select register bit is out of range"]
pub type REF_SEL_ERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAT_SPEC, bool, O>;
#[doc = "Field `FB_TO_ERR` reader - Interrupt flag that indicates a timeout while adjusting the voltage"]
pub type FB_TO_ERR_R = crate::BitReader<bool>;
#[doc = "Field `FB_TO_ERR` writer - Interrupt flag that indicates a timeout while adjusting the voltage"]
pub type FB_TO_ERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAT_SPEC, bool, O>;
#[doc = "Field `FB_TO_ERR_S` reader - Interrupt flag that mirror FB_TO_ERR and is write one clear"]
pub type FB_TO_ERR_S_R = crate::BitReader<bool>;
#[doc = "Field `FB_TO_ERR_S` writer - Interrupt flag that mirror FB_TO_ERR and is write one clear"]
pub type FB_TO_ERR_S_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAT_SPEC, bool, O>;
#[doc = "Field `FC_LV_DET_INT` reader - Interrupt flag that indicates the power supply voltage requested is below the low threshold"]
pub type FC_LV_DET_INT_R = crate::BitReader<bool>;
#[doc = "Field `FC_LV_DET_INT` writer - Interrupt flag that indicates the power supply voltage requested is below the low threshold"]
pub type FC_LV_DET_INT_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAT_SPEC, bool, O>;
#[doc = "Field `FC_LV_DET_S` reader - Interrupt flag that mirrors FC_LV_DET_INT"]
pub type FC_LV_DET_S_R = crate::BitReader<bool>;
#[doc = "Field `FC_LV_DET_S` writer - Interrupt flag that mirrors FC_LV_DET_INT"]
pub type FC_LV_DET_S_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAT_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3 - State machine state"]
    #[inline(always)]
    pub fn dvs_state(&self) -> DVS_STATE_R {
        DVS_STATE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - DVS Raising voltage"]
    #[inline(always)]
    pub fn adj_up_ena(&self) -> ADJ_UP_ENA_R {
        ADJ_UP_ENA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DVS Lowering voltage"]
    #[inline(always)]
    pub fn adj_dwn_ena(&self) -> ADJ_DWN_ENA_R {
        ADJ_DWN_ENA_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Adjustment to a Direct Voltage"]
    #[inline(always)]
    pub fn adj_active(&self) -> ADJ_ACTIVE_R {
        ADJ_ACTIVE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Tap Enabled and the Tap is withing Hi/Low limits"]
    #[inline(always)]
    pub fn ctr_tap_ok(&self) -> CTR_TAP_OK_R {
        CTR_TAP_OK_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Status of selected center tap delay line detect output"]
    #[inline(always)]
    pub fn ctr_tap_sel(&self) -> CTR_TAP_SEL_R {
        CTR_TAP_SEL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Provides the current combined status of all selected Low Range delay lines"]
    #[inline(always)]
    pub fn slow_trip_det(&self) -> SLOW_TRIP_DET_R {
        SLOW_TRIP_DET_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Provides the current combined status of all selected High Range delay lines"]
    #[inline(always)]
    pub fn fast_trip_det(&self) -> FAST_TRIP_DET_R {
        FAST_TRIP_DET_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Indicates if the power supply is in range"]
    #[inline(always)]
    pub fn ps_in_range(&self) -> PS_IN_RANGE_R {
        PS_IN_RANGE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:18 - Voltage Count value sent to the power supply"]
    #[inline(always)]
    pub fn ps_vcntr(&self) -> PS_VCNTR_R {
        PS_VCNTR_R::new(((self.bits >> 12) & 0x7f) as u8)
    }
    #[doc = "Bit 19 - Indicates the monitor delay count is at 0"]
    #[inline(always)]
    pub fn mon_dly_ok(&self) -> MON_DLY_OK_R {
        MON_DLY_OK_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Indicates the adjustment delay count is at 0"]
    #[inline(always)]
    pub fn adj_dly_ok(&self) -> ADJ_DLY_OK_R {
        ADJ_DLY_OK_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Power supply voltage counter is at low limit"]
    #[inline(always)]
    pub fn lo_limit_det(&self) -> LO_LIMIT_DET_R {
        LO_LIMIT_DET_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Power supply voltage counter is at high limit"]
    #[inline(always)]
    pub fn hi_limit_det(&self) -> HI_LIMIT_DET_R {
        HI_LIMIT_DET_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - At least one delay line has been enabled"]
    #[inline(always)]
    pub fn valid_tap(&self) -> VALID_TAP_R {
        VALID_TAP_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Interrupt flag that indicates a voltage count is at/beyond manufacturer limits"]
    #[inline(always)]
    pub fn limit_err(&self) -> LIMIT_ERR_R {
        LIMIT_ERR_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Interrupt flag that indicates a tap has an invalid value"]
    #[inline(always)]
    pub fn range_err(&self) -> RANGE_ERR_R {
        RANGE_ERR_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Interrupt flag that indicates up and down adjustment requested simultaneously"]
    #[inline(always)]
    pub fn adj_err(&self) -> ADJ_ERR_R {
        ADJ_ERR_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Indicates the ref select register bit is out of range"]
    #[inline(always)]
    pub fn ref_sel_err(&self) -> REF_SEL_ERR_R {
        REF_SEL_ERR_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Interrupt flag that indicates a timeout while adjusting the voltage"]
    #[inline(always)]
    pub fn fb_to_err(&self) -> FB_TO_ERR_R {
        FB_TO_ERR_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Interrupt flag that mirror FB_TO_ERR and is write one clear"]
    #[inline(always)]
    pub fn fb_to_err_s(&self) -> FB_TO_ERR_S_R {
        FB_TO_ERR_S_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Interrupt flag that indicates the power supply voltage requested is below the low threshold"]
    #[inline(always)]
    pub fn fc_lv_det_int(&self) -> FC_LV_DET_INT_R {
        FC_LV_DET_INT_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Interrupt flag that mirrors FC_LV_DET_INT"]
    #[inline(always)]
    pub fn fc_lv_det_s(&self) -> FC_LV_DET_S_R {
        FC_LV_DET_S_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - State machine state"]
    #[inline(always)]
    #[must_use]
    pub fn dvs_state(&mut self) -> DVS_STATE_W<0> {
        DVS_STATE_W::new(self)
    }
    #[doc = "Bit 4 - DVS Raising voltage"]
    #[inline(always)]
    #[must_use]
    pub fn adj_up_ena(&mut self) -> ADJ_UP_ENA_W<4> {
        ADJ_UP_ENA_W::new(self)
    }
    #[doc = "Bit 5 - DVS Lowering voltage"]
    #[inline(always)]
    #[must_use]
    pub fn adj_dwn_ena(&mut self) -> ADJ_DWN_ENA_W<5> {
        ADJ_DWN_ENA_W::new(self)
    }
    #[doc = "Bit 6 - Adjustment to a Direct Voltage"]
    #[inline(always)]
    #[must_use]
    pub fn adj_active(&mut self) -> ADJ_ACTIVE_W<6> {
        ADJ_ACTIVE_W::new(self)
    }
    #[doc = "Bit 7 - Tap Enabled and the Tap is withing Hi/Low limits"]
    #[inline(always)]
    #[must_use]
    pub fn ctr_tap_ok(&mut self) -> CTR_TAP_OK_W<7> {
        CTR_TAP_OK_W::new(self)
    }
    #[doc = "Bit 8 - Status of selected center tap delay line detect output"]
    #[inline(always)]
    #[must_use]
    pub fn ctr_tap_sel(&mut self) -> CTR_TAP_SEL_W<8> {
        CTR_TAP_SEL_W::new(self)
    }
    #[doc = "Bit 9 - Provides the current combined status of all selected Low Range delay lines"]
    #[inline(always)]
    #[must_use]
    pub fn slow_trip_det(&mut self) -> SLOW_TRIP_DET_W<9> {
        SLOW_TRIP_DET_W::new(self)
    }
    #[doc = "Bit 10 - Provides the current combined status of all selected High Range delay lines"]
    #[inline(always)]
    #[must_use]
    pub fn fast_trip_det(&mut self) -> FAST_TRIP_DET_W<10> {
        FAST_TRIP_DET_W::new(self)
    }
    #[doc = "Bit 11 - Indicates if the power supply is in range"]
    #[inline(always)]
    #[must_use]
    pub fn ps_in_range(&mut self) -> PS_IN_RANGE_W<11> {
        PS_IN_RANGE_W::new(self)
    }
    #[doc = "Bits 12:18 - Voltage Count value sent to the power supply"]
    #[inline(always)]
    #[must_use]
    pub fn ps_vcntr(&mut self) -> PS_VCNTR_W<12> {
        PS_VCNTR_W::new(self)
    }
    #[doc = "Bit 19 - Indicates the monitor delay count is at 0"]
    #[inline(always)]
    #[must_use]
    pub fn mon_dly_ok(&mut self) -> MON_DLY_OK_W<19> {
        MON_DLY_OK_W::new(self)
    }
    #[doc = "Bit 20 - Indicates the adjustment delay count is at 0"]
    #[inline(always)]
    #[must_use]
    pub fn adj_dly_ok(&mut self) -> ADJ_DLY_OK_W<20> {
        ADJ_DLY_OK_W::new(self)
    }
    #[doc = "Bit 21 - Power supply voltage counter is at low limit"]
    #[inline(always)]
    #[must_use]
    pub fn lo_limit_det(&mut self) -> LO_LIMIT_DET_W<21> {
        LO_LIMIT_DET_W::new(self)
    }
    #[doc = "Bit 22 - Power supply voltage counter is at high limit"]
    #[inline(always)]
    #[must_use]
    pub fn hi_limit_det(&mut self) -> HI_LIMIT_DET_W<22> {
        HI_LIMIT_DET_W::new(self)
    }
    #[doc = "Bit 23 - At least one delay line has been enabled"]
    #[inline(always)]
    #[must_use]
    pub fn valid_tap(&mut self) -> VALID_TAP_W<23> {
        VALID_TAP_W::new(self)
    }
    #[doc = "Bit 24 - Interrupt flag that indicates a voltage count is at/beyond manufacturer limits"]
    #[inline(always)]
    #[must_use]
    pub fn limit_err(&mut self) -> LIMIT_ERR_W<24> {
        LIMIT_ERR_W::new(self)
    }
    #[doc = "Bit 25 - Interrupt flag that indicates a tap has an invalid value"]
    #[inline(always)]
    #[must_use]
    pub fn range_err(&mut self) -> RANGE_ERR_W<25> {
        RANGE_ERR_W::new(self)
    }
    #[doc = "Bit 26 - Interrupt flag that indicates up and down adjustment requested simultaneously"]
    #[inline(always)]
    #[must_use]
    pub fn adj_err(&mut self) -> ADJ_ERR_W<26> {
        ADJ_ERR_W::new(self)
    }
    #[doc = "Bit 27 - Indicates the ref select register bit is out of range"]
    #[inline(always)]
    #[must_use]
    pub fn ref_sel_err(&mut self) -> REF_SEL_ERR_W<27> {
        REF_SEL_ERR_W::new(self)
    }
    #[doc = "Bit 28 - Interrupt flag that indicates a timeout while adjusting the voltage"]
    #[inline(always)]
    #[must_use]
    pub fn fb_to_err(&mut self) -> FB_TO_ERR_W<28> {
        FB_TO_ERR_W::new(self)
    }
    #[doc = "Bit 29 - Interrupt flag that mirror FB_TO_ERR and is write one clear"]
    #[inline(always)]
    #[must_use]
    pub fn fb_to_err_s(&mut self) -> FB_TO_ERR_S_W<29> {
        FB_TO_ERR_S_W::new(self)
    }
    #[doc = "Bit 30 - Interrupt flag that indicates the power supply voltage requested is below the low threshold"]
    #[inline(always)]
    #[must_use]
    pub fn fc_lv_det_int(&mut self) -> FC_LV_DET_INT_W<30> {
        FC_LV_DET_INT_W::new(self)
    }
    #[doc = "Bit 31 - Interrupt flag that mirrors FC_LV_DET_INT"]
    #[inline(always)]
    #[must_use]
    pub fn fc_lv_det_s(&mut self) -> FC_LV_DET_S_W<31> {
        FC_LV_DET_S_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Status Fields\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stat](index.html) module"]
pub struct STAT_SPEC;
impl crate::RegisterSpec for STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stat::R](R) reader structure"]
impl crate::Readable for STAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [stat::W](W) writer structure"]
impl crate::Writable for STAT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STAT to value 0"]
impl crate::Resettable for STAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
