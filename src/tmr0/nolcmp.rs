#[doc = "Register `NOLCMP` reader"]
pub struct R(crate::R<NOLCMP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NOLCMP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NOLCMP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NOLCMP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NOLCMP` writer"]
pub struct W(crate::W<NOLCMP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NOLCMP_SPEC>;
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
impl From<crate::W<NOLCMP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NOLCMP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NOLLCMP` reader - Non-overlapping Low Compare. The 8-bit timer count value of non-overlapping time between falling edge of PWM output 0A and next rising edge of PWM output 0A'."]
pub struct NOLLCMP_R(crate::FieldReader<u8, u8>);
impl NOLLCMP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        NOLLCMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NOLLCMP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NOLLCMP` writer - Non-overlapping Low Compare. The 8-bit timer count value of non-overlapping time between falling edge of PWM output 0A and next rising edge of PWM output 0A'."]
pub struct NOLLCMP_W<'a> {
    w: &'a mut W,
}
impl<'a> NOLLCMP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `NOLHCMP` reader - Non-overlapping High Compare. The 8-bit timer count value of non-overlapping time between falling edge of PWM output 0A' and next rising edge of PWM output 0A."]
pub struct NOLHCMP_R(crate::FieldReader<u8, u8>);
impl NOLHCMP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        NOLHCMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NOLHCMP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NOLHCMP` writer - Non-overlapping High Compare. The 8-bit timer count value of non-overlapping time between falling edge of PWM output 0A' and next rising edge of PWM output 0A."]
pub struct NOLHCMP_W<'a> {
    w: &'a mut W,
}
impl<'a> NOLHCMP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Non-overlapping Low Compare. The 8-bit timer count value of non-overlapping time between falling edge of PWM output 0A and next rising edge of PWM output 0A'."]
    #[inline(always)]
    pub fn nollcmp(&self) -> NOLLCMP_R {
        NOLLCMP_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Non-overlapping High Compare. The 8-bit timer count value of non-overlapping time between falling edge of PWM output 0A' and next rising edge of PWM output 0A."]
    #[inline(always)]
    pub fn nolhcmp(&self) -> NOLHCMP_R {
        NOLHCMP_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Non-overlapping Low Compare. The 8-bit timer count value of non-overlapping time between falling edge of PWM output 0A and next rising edge of PWM output 0A'."]
    #[inline(always)]
    pub fn nollcmp(&mut self) -> NOLLCMP_W {
        NOLLCMP_W { w: self }
    }
    #[doc = "Bits 8:15 - Non-overlapping High Compare. The 8-bit timer count value of non-overlapping time between falling edge of PWM output 0A' and next rising edge of PWM output 0A."]
    #[inline(always)]
    pub fn nolhcmp(&mut self) -> NOLHCMP_W {
        NOLHCMP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer Non-Overlapping Compare Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nolcmp](index.html) module"]
pub struct NOLCMP_SPEC;
impl crate::RegisterSpec for NOLCMP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nolcmp::R](R) reader structure"]
impl crate::Readable for NOLCMP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nolcmp::W](W) writer structure"]
impl crate::Writable for NOLCMP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets NOLCMP to value 0"]
impl crate::Resettable for NOLCMP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
