#[doc = "Register `DLRTC` reader"]
pub struct R(crate::R<DLRTC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DLRTC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DLRTC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DLRTC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DLRTC` reader - DRS Log RTC Value. This register contains the 32 bit value in the RTC second register when the last DRS event occured."]
pub struct DLRTC_R(crate::FieldReader<u32, u32>);
impl DLRTC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        DLRTC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DLRTC_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - DRS Log RTC Value. This register contains the 32 bit value in the RTC second register when the last DRS event occured."]
    #[inline(always)]
    pub fn dlrtc(&self) -> DLRTC_R {
        DLRTC_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "DRS Log RTC Value. This register contains the 32 bit value in the RTC second register when the last DRS event occurred.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dlrtc](index.html) module"]
pub struct DLRTC_SPEC;
impl crate::RegisterSpec for DLRTC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dlrtc::R](R) reader structure"]
impl crate::Readable for DLRTC_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DLRTC to value 0"]
impl crate::Resettable for DLRTC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
