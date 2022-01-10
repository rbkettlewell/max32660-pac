#[doc = "Register `LP_WAKEFL` reader"]
pub struct R(crate::R<LP_WAKEFL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LP_WAKEFL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LP_WAKEFL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LP_WAKEFL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LP_WAKEFL` writer"]
pub struct W(crate::W<LP_WAKEFL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LP_WAKEFL_SPEC>;
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
impl From<crate::W<LP_WAKEFL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LP_WAKEFL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WAKEST` reader - Wakeup IRQ flags (write ones to clear). One or more of these bits will be set when the corresponding dedicated GPIO pin(s) transition(s) from low to high or high to low. If GPIO wakeup source is selected, using PM.GPIOWKEN register, and the corresponding bit is also selected in LPWKEN register, an interrupt will be gnerated to wake up the CPU from a low power mode."]
pub struct WAKEST_R(crate::FieldReader<u16, u16>);
impl WAKEST_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        WAKEST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WAKEST_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WAKEST` writer - Wakeup IRQ flags (write ones to clear). One or more of these bits will be set when the corresponding dedicated GPIO pin(s) transition(s) from low to high or high to low. If GPIO wakeup source is selected, using PM.GPIOWKEN register, and the corresponding bit is also selected in LPWKEN register, an interrupt will be gnerated to wake up the CPU from a low power mode."]
pub struct WAKEST_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKEST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff) | (value as u32 & 0x3fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:13 - Wakeup IRQ flags (write ones to clear). One or more of these bits will be set when the corresponding dedicated GPIO pin(s) transition(s) from low to high or high to low. If GPIO wakeup source is selected, using PM.GPIOWKEN register, and the corresponding bit is also selected in LPWKEN register, an interrupt will be gnerated to wake up the CPU from a low power mode."]
    #[inline(always)]
    pub fn wakest(&self) -> WAKEST_R {
        WAKEST_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - Wakeup IRQ flags (write ones to clear). One or more of these bits will be set when the corresponding dedicated GPIO pin(s) transition(s) from low to high or high to low. If GPIO wakeup source is selected, using PM.GPIOWKEN register, and the corresponding bit is also selected in LPWKEN register, an interrupt will be gnerated to wake up the CPU from a low power mode."]
    #[inline(always)]
    pub fn wakest(&mut self) -> WAKEST_W {
        WAKEST_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Low Power Mode Wakeup Flags for GPIO0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lp_wakefl](index.html) module"]
pub struct LP_WAKEFL_SPEC;
impl crate::RegisterSpec for LP_WAKEFL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lp_wakefl::R](R) reader structure"]
impl crate::Readable for LP_WAKEFL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lp_wakefl::W](W) writer structure"]
impl crate::Writable for LP_WAKEFL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LP_WAKEFL to value 0"]
impl crate::Resettable for LP_WAKEFL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
