#[doc = "Register `SZ` reader"]
pub struct R(crate::R<SZ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SZ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SZ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SZ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CCH` reader - Cache Size. Indicates total size in Kbytes of cache."]
pub type CCH_R = crate::FieldReader<u16, u16>;
#[doc = "Field `MEM` reader - Main Memory Size. Indicates the total size, in units of 128 Kbytes, of code memory accessible to the cache controller."]
pub type MEM_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Cache Size. Indicates total size in Kbytes of cache."]
    #[inline(always)]
    pub fn cch(&self) -> CCH_R {
        CCH_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Main Memory Size. Indicates the total size, in units of 128 Kbytes, of code memory accessible to the cache controller."]
    #[inline(always)]
    pub fn mem(&self) -> MEM_R {
        MEM_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Memory Configuration Register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sz](index.html) module"]
pub struct SZ_SPEC;
impl crate::RegisterSpec for SZ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sz::R](R) reader structure"]
impl crate::Readable for SZ_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SZ to value 0x0008_0008"]
impl crate::Resettable for SZ_SPEC {
    const RESET_VALUE: Self::Ux = 0x0008_0008;
}
