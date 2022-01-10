#[doc = "Register `BAUD1` reader"]
pub struct R(crate::R<BAUD1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BAUD1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BAUD1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BAUD1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BAUD1` writer"]
pub struct W(crate::W<BAUD1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BAUD1_SPEC>;
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
impl From<crate::W<BAUD1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BAUD1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DBAUD` reader - Decimal portion of baud rate divisor value. DIV = InputClock/(factor*Baud Rate Frequency). DDIV=(DIV-IDIV)*128."]
pub struct DBAUD_R(crate::FieldReader<u16, u16>);
impl DBAUD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        DBAUD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBAUD_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBAUD` writer - Decimal portion of baud rate divisor value. DIV = InputClock/(factor*Baud Rate Frequency). DDIV=(DIV-IDIV)*128."]
pub struct DBAUD_W<'a> {
    w: &'a mut W,
}
impl<'a> DBAUD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | (value as u32 & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - Decimal portion of baud rate divisor value. DIV = InputClock/(factor*Baud Rate Frequency). DDIV=(DIV-IDIV)*128."]
    #[inline(always)]
    pub fn dbaud(&self) -> DBAUD_R {
        DBAUD_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Decimal portion of baud rate divisor value. DIV = InputClock/(factor*Baud Rate Frequency). DDIV=(DIV-IDIV)*128."]
    #[inline(always)]
    pub fn dbaud(&mut self) -> DBAUD_W {
        DBAUD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Baud rate register. Decimal Setting.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [baud1](index.html) module"]
pub struct BAUD1_SPEC;
impl crate::RegisterSpec for BAUD1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [baud1::R](R) reader structure"]
impl crate::Readable for BAUD1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [baud1::W](W) writer structure"]
impl crate::Writable for BAUD1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BAUD1 to value 0"]
impl crate::Resettable for BAUD1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
