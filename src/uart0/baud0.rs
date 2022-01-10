#[doc = "Register `BAUD0` reader"]
pub struct R(crate::R<BAUD0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BAUD0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BAUD0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BAUD0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BAUD0` writer"]
pub struct W(crate::W<BAUD0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BAUD0_SPEC>;
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
impl From<crate::W<BAUD0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BAUD0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IBAUD` reader - Integer portion of baud rate divisor value. IBAUD = InputClock / (factor * Baud Rate Frequency)."]
pub struct IBAUD_R(crate::FieldReader<u16, u16>);
impl IBAUD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        IBAUD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IBAUD_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IBAUD` writer - Integer portion of baud rate divisor value. IBAUD = InputClock / (factor * Baud Rate Frequency)."]
pub struct IBAUD_W<'a> {
    w: &'a mut W,
}
impl<'a> IBAUD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | (value as u32 & 0x0fff);
        self.w
    }
}
#[doc = "FACTOR must be chosen to have IDIV>0. factor used in calculation = 128 >> FACTOR.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FACTOR_A {
    #[doc = "0: Baud Factor 128"]
    _128 = 0,
    #[doc = "1: Baud Factor 64"]
    _64 = 1,
    #[doc = "2: Baud Factor 32"]
    _32 = 2,
    #[doc = "3: Baud Factor 16"]
    _16 = 3,
}
impl From<FACTOR_A> for u8 {
    #[inline(always)]
    fn from(variant: FACTOR_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FACTOR` reader - FACTOR must be chosen to have IDIV>0. factor used in calculation = 128 >> FACTOR."]
pub struct FACTOR_R(crate::FieldReader<u8, FACTOR_A>);
impl FACTOR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FACTOR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FACTOR_A {
        match self.bits {
            0 => FACTOR_A::_128,
            1 => FACTOR_A::_64,
            2 => FACTOR_A::_32,
            3 => FACTOR_A::_16,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_128`"]
    #[inline(always)]
    pub fn is_128(&self) -> bool {
        **self == FACTOR_A::_128
    }
    #[doc = "Checks if the value of the field is `_64`"]
    #[inline(always)]
    pub fn is_64(&self) -> bool {
        **self == FACTOR_A::_64
    }
    #[doc = "Checks if the value of the field is `_32`"]
    #[inline(always)]
    pub fn is_32(&self) -> bool {
        **self == FACTOR_A::_32
    }
    #[doc = "Checks if the value of the field is `_16`"]
    #[inline(always)]
    pub fn is_16(&self) -> bool {
        **self == FACTOR_A::_16
    }
}
impl core::ops::Deref for FACTOR_R {
    type Target = crate::FieldReader<u8, FACTOR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FACTOR` writer - FACTOR must be chosen to have IDIV>0. factor used in calculation = 128 >> FACTOR."]
pub struct FACTOR_W<'a> {
    w: &'a mut W,
}
impl<'a> FACTOR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FACTOR_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Baud Factor 128"]
    #[inline(always)]
    pub fn _128(self) -> &'a mut W {
        self.variant(FACTOR_A::_128)
    }
    #[doc = "Baud Factor 64"]
    #[inline(always)]
    pub fn _64(self) -> &'a mut W {
        self.variant(FACTOR_A::_64)
    }
    #[doc = "Baud Factor 32"]
    #[inline(always)]
    pub fn _32(self) -> &'a mut W {
        self.variant(FACTOR_A::_32)
    }
    #[doc = "Baud Factor 16"]
    #[inline(always)]
    pub fn _16(self) -> &'a mut W {
        self.variant(FACTOR_A::_16)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - Integer portion of baud rate divisor value. IBAUD = InputClock / (factor * Baud Rate Frequency)."]
    #[inline(always)]
    pub fn ibaud(&self) -> IBAUD_R {
        IBAUD_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:17 - FACTOR must be chosen to have IDIV>0. factor used in calculation = 128 >> FACTOR."]
    #[inline(always)]
    pub fn factor(&self) -> FACTOR_R {
        FACTOR_R::new(((self.bits >> 16) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11 - Integer portion of baud rate divisor value. IBAUD = InputClock / (factor * Baud Rate Frequency)."]
    #[inline(always)]
    pub fn ibaud(&mut self) -> IBAUD_W {
        IBAUD_W { w: self }
    }
    #[doc = "Bits 16:17 - FACTOR must be chosen to have IDIV>0. factor used in calculation = 128 >> FACTOR."]
    #[inline(always)]
    pub fn factor(&mut self) -> FACTOR_W {
        FACTOR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Baud rate register. Integer portion.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [baud0](index.html) module"]
pub struct BAUD0_SPEC;
impl crate::RegisterSpec for BAUD0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [baud0::R](R) reader structure"]
impl crate::Readable for BAUD0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [baud0::W](W) writer structure"]
impl crate::Writable for BAUD0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BAUD0 to value 0"]
impl crate::Resettable for BAUD0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
