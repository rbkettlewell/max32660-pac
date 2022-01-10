#[doc = "Register `PERCKCN1` reader"]
pub struct R(crate::R<PERCKCN1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PERCKCN1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PERCKCN1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PERCKCN1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PERCKCN1` writer"]
pub struct W(crate::W<PERCKCN1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PERCKCN1_SPEC>;
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
impl From<crate::W<PERCKCN1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PERCKCN1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Secure Flash Controller Disable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLCD_A {
    #[doc = "0: Enable."]
    EN = 0,
    #[doc = "1: Disable."]
    DIS = 1,
}
impl From<FLCD_A> for bool {
    #[inline(always)]
    fn from(variant: FLCD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLCD` reader - Secure Flash Controller Disable."]
pub struct FLCD_R(crate::FieldReader<bool, FLCD_A>);
impl FLCD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FLCD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLCD_A {
        match self.bits {
            false => FLCD_A::EN,
            true => FLCD_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == FLCD_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == FLCD_A::DIS
    }
}
impl core::ops::Deref for FLCD_R {
    type Target = crate::FieldReader<bool, FLCD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLCD` writer - Secure Flash Controller Disable."]
pub struct FLCD_W<'a> {
    w: &'a mut W,
}
impl<'a> FLCD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLCD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(FLCD_A::EN)
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(FLCD_A::DIS)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "ICache Clock Disable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICACHED_A {
    #[doc = "0: Enable."]
    EN = 0,
    #[doc = "1: Disable."]
    DIS = 1,
}
impl From<ICACHED_A> for bool {
    #[inline(always)]
    fn from(variant: ICACHED_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ICACHED` reader - ICache Clock Disable."]
pub struct ICACHED_R(crate::FieldReader<bool, ICACHED_A>);
impl ICACHED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ICACHED_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICACHED_A {
        match self.bits {
            false => ICACHED_A::EN,
            true => ICACHED_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == ICACHED_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == ICACHED_A::DIS
    }
}
impl core::ops::Deref for ICACHED_R {
    type Target = crate::FieldReader<bool, ICACHED_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ICACHED` writer - ICache Clock Disable."]
pub struct ICACHED_W<'a> {
    w: &'a mut W,
}
impl<'a> ICACHED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ICACHED_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(ICACHED_A::EN)
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(ICACHED_A::DIS)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
impl R {
    #[doc = "Bit 3 - Secure Flash Controller Disable."]
    #[inline(always)]
    pub fn flcd(&self) -> FLCD_R {
        FLCD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 11 - ICache Clock Disable."]
    #[inline(always)]
    pub fn icached(&self) -> ICACHED_R {
        ICACHED_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Secure Flash Controller Disable."]
    #[inline(always)]
    pub fn flcd(&mut self) -> FLCD_W {
        FLCD_W { w: self }
    }
    #[doc = "Bit 11 - ICache Clock Disable."]
    #[inline(always)]
    pub fn icached(&mut self) -> ICACHED_W {
        ICACHED_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peripheral Clock Disable.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [perckcn1](index.html) module"]
pub struct PERCKCN1_SPEC;
impl crate::RegisterSpec for PERCKCN1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [perckcn1::R](R) reader structure"]
impl crate::Readable for PERCKCN1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [perckcn1::W](W) writer structure"]
impl crate::Writable for PERCKCN1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PERCKCN1 to value 0"]
impl crate::Resettable for PERCKCN1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
