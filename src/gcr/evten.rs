#[doc = "Register `EVTEN` reader"]
pub struct R(crate::R<EVTEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVTEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVTEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVTEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EVTEN` writer"]
pub struct W(crate::W<EVTEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EVTEN_SPEC>;
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
impl From<crate::W<EVTEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EVTEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMAEVENT` reader - Enable DMA event. When this bit is set, a DMA event will cause an RXEV event to wake the CPU from WFE sleep mode."]
pub struct DMAEVENT_R(crate::FieldReader<bool, bool>);
impl DMAEVENT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMAEVENT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMAEVENT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMAEVENT` writer - Enable DMA event. When this bit is set, a DMA event will cause an RXEV event to wake the CPU from WFE sleep mode."]
pub struct DMAEVENT_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAEVENT_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `RXEVENT` reader - Enable RXEV pin event. When this bit is set, a logic high of GPIO0\\[24\\]
will cause an RXEV event to wake the CPU from WFE sleep mode."]
pub struct RXEVENT_R(crate::FieldReader<bool, bool>);
impl RXEVENT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXEVENT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXEVENT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXEVENT` writer - Enable RXEV pin event. When this bit is set, a logic high of GPIO0\\[24\\]
will cause an RXEV event to wake the CPU from WFE sleep mode."]
pub struct RXEVENT_W<'a> {
    w: &'a mut W,
}
impl<'a> RXEVENT_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Enable DMA event. When this bit is set, a DMA event will cause an RXEV event to wake the CPU from WFE sleep mode."]
    #[inline(always)]
    pub fn dmaevent(&self) -> DMAEVENT_R {
        DMAEVENT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable RXEV pin event. When this bit is set, a logic high of GPIO0\\[24\\]
will cause an RXEV event to wake the CPU from WFE sleep mode."]
    #[inline(always)]
    pub fn rxevent(&self) -> RXEVENT_R {
        RXEVENT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable DMA event. When this bit is set, a DMA event will cause an RXEV event to wake the CPU from WFE sleep mode."]
    #[inline(always)]
    pub fn dmaevent(&mut self) -> DMAEVENT_W {
        DMAEVENT_W { w: self }
    }
    #[doc = "Bit 1 - Enable RXEV pin event. When this bit is set, a logic high of GPIO0\\[24\\]
will cause an RXEV event to wake the CPU from WFE sleep mode."]
    #[inline(always)]
    pub fn rxevent(&mut self) -> RXEVENT_W {
        RXEVENT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Event Enable Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evten](index.html) module"]
pub struct EVTEN_SPEC;
impl crate::RegisterSpec for EVTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [evten::R](R) reader structure"]
impl crate::Readable for EVTEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [evten::W](W) writer structure"]
impl crate::Writable for EVTEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EVTEN to value 0"]
impl crate::Resettable for EVTEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
