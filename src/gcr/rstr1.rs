#[doc = "Register `RSTR1` reader"]
pub struct R(crate::R<RSTR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RSTR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RSTR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RSTR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RSTR1` writer"]
pub struct W(crate::W<RSTR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RSTR1_SPEC>;
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
impl From<crate::W<RSTR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RSTR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "I2C1 Reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C1_A {
    #[doc = "0: Reset complete."]
    RESET_DONE = 0,
    #[doc = "1: Reset in progress."]
    BUSY = 1,
}
impl From<I2C1_A> for bool {
    #[inline(always)]
    fn from(variant: I2C1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C1` reader - I2C1 Reset."]
pub struct I2C1_R(crate::FieldReader<bool, I2C1_A>);
impl I2C1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        I2C1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C1_A {
        match self.bits {
            false => I2C1_A::RESET_DONE,
            true => I2C1_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `RESET_DONE`"]
    #[inline(always)]
    pub fn is_reset_done(&self) -> bool {
        **self == I2C1_A::RESET_DONE
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        **self == I2C1_A::BUSY
    }
}
impl core::ops::Deref for I2C1_R {
    type Target = crate::FieldReader<bool, I2C1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "I2C1 Reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C1_AW {
    #[doc = "0: Reserved. Do not use."]
    RFU = 0,
    #[doc = "1: Starts reset operation."]
    RESET = 1,
}
impl From<I2C1_AW> for bool {
    #[inline(always)]
    fn from(variant: I2C1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C1` writer - I2C1 Reset."]
pub struct I2C1_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C1_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reserved. Do not use."]
    #[inline(always)]
    pub fn rfu(self) -> &'a mut W {
        self.variant(I2C1_AW::RFU)
    }
    #[doc = "Starts reset operation."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(I2C1_AW::RESET)
    }
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
impl R {
    #[doc = "Bit 0 - I2C1 Reset."]
    #[inline(always)]
    pub fn i2c1(&self) -> I2C1_R {
        I2C1_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2C1 Reset."]
    #[inline(always)]
    pub fn i2c1(&mut self) -> I2C1_W {
        I2C1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Reset 1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rstr1](index.html) module"]
pub struct RSTR1_SPEC;
impl crate::RegisterSpec for RSTR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rstr1::R](R) reader structure"]
impl crate::Readable for RSTR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rstr1::W](W) writer structure"]
impl crate::Writable for RSTR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RSTR1 to value 0"]
impl crate::Resettable for RSTR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
