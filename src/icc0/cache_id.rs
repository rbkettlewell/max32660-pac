#[doc = "Register `CACHE_ID` reader"]
pub struct R(crate::R<CACHE_ID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CACHE_ID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CACHE_ID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CACHE_ID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RELNUM` reader - Release Number. Identifies the RTL release version."]
pub struct RELNUM_R(crate::FieldReader<u8, u8>);
impl RELNUM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RELNUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RELNUM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PARTNUM` reader - Part Number. This field reflects the value of C_ID_PART_NUMBER configuration parameter."]
pub struct PARTNUM_R(crate::FieldReader<u8, u8>);
impl PARTNUM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PARTNUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PARTNUM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCHID` reader - Cache ID. This field reflects the value of the C_ID_CACHEID configuration parameter."]
pub struct CCHID_R(crate::FieldReader<u8, u8>);
impl CCHID_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CCHID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCHID_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
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
    pub fn cchid(&self) -> CCHID_R {
        CCHID_R::new(((self.bits >> 10) & 0x3f) as u8)
    }
}
#[doc = "Cache ID Register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cache_id](index.html) module"]
pub struct CACHE_ID_SPEC;
impl crate::RegisterSpec for CACHE_ID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cache_id::R](R) reader structure"]
impl crate::Readable for CACHE_ID_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CACHE_ID to value 0"]
impl crate::Resettable for CACHE_ID_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
