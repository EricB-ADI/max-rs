#[doc = "Register `INFO` reader"]
pub struct R(crate::R<INFO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INFO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INFO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INFO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RELNUM` reader - Release Number. Identifies the RTL release version."]
pub type RELNUM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PARTNUM` reader - Part Number. This field reflects the value of C_ID_PART_NUMBER configuration parameter."]
pub type PARTNUM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ID` reader - Cache ID. This field reflects the value of the C_ID_CACHEID configuration parameter."]
pub type ID_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:5 - Release Number. Identifies the RTL release version."]
    #[inline(always)]
    pub fn relnum(&self) -> RELNUM_R {
        RELNUM_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:9 - Part Number. This field reflects the value of C_ID_PART_NUMBER configuration parameter."]
    #[inline(always)]
    pub fn partnum(&self) -> PARTNUM_R {
        PARTNUM_R::new(((self.bits >> 6) & 0x0f) as u8)
    }
    #[doc = "Bits 10:15 - Cache ID. This field reflects the value of the C_ID_CACHEID configuration parameter."]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new(((self.bits >> 10) & 0x3f) as u8)
    }
}
#[doc = "Cache ID Register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [info](index.html) module"]
pub struct INFO_SPEC;
impl crate::RegisterSpec for INFO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [info::R](R) reader structure"]
impl crate::Readable for INFO_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INFO to value 0"]
impl crate::Resettable for INFO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
