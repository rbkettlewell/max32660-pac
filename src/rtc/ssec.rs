#[doc = "Register `SSEC` reader"]
pub struct R(crate::R<SSEC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SSEC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SSEC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SSEC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SSEC` writer"]
pub struct W(crate::W<SSEC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SSEC_SPEC>;
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
impl From<crate::W<SSEC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SSEC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RTSS` reader - RTC Sub-second Counter."]
pub struct RTSS_R(crate::FieldReader<u8, u8>);
impl RTSS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RTSS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTSS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTSS` writer - RTC Sub-second Counter."]
pub struct RTSS_W<'a> {
    w: &'a mut W,
}
impl<'a> RTSS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - RTC Sub-second Counter."]
    #[inline(always)]
    pub fn rtss(&self) -> RTSS_R {
        RTSS_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - RTC Sub-second Counter."]
    #[inline(always)]
    pub fn rtss(&mut self) -> RTSS_W {
        RTSS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC Sub-second Counter. This counter increments at 256Hz. RTC_SEC is incremented when this register rolls over from 0xFF to 0x00.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssec](index.html) module"]
pub struct SSEC_SPEC;
impl crate::RegisterSpec for SSEC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ssec::R](R) reader structure"]
impl crate::Readable for SSEC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ssec::W](W) writer structure"]
impl crate::Writable for SSEC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SSEC to value 0"]
impl crate::Resettable for SSEC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
