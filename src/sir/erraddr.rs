#[doc = "Register `ERRADDR` reader"]
pub struct R(crate::R<ERRADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ERRADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ERRADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ERRADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ERRADDR` reader - "]
pub struct ERRADDR_R(crate::FieldReader<u32, u32>);
impl ERRADDR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        ERRADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ERRADDR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn erraddr(&self) -> ERRADDR_R {
        ERRADDR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "Read-only field set by the SIB block if a CRC error occurs during the read of the OTP memory. Contains the failing address in OTP memory (when CRCERR equals 1).\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [erraddr](index.html) module"]
pub struct ERRADDR_SPEC;
impl crate::RegisterSpec for ERRADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [erraddr::R](R) reader structure"]
impl crate::Readable for ERRADDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ERRADDR to value 0"]
impl crate::Resettable for ERRADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
