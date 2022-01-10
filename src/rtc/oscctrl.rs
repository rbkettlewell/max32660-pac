#[doc = "Register `OSCCTRL` reader"]
pub struct R(crate::R<OSCCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OSCCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OSCCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OSCCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OSCCTRL` writer"]
pub struct W(crate::W<OSCCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OSCCTRL_SPEC>;
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
impl From<crate::W<OSCCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OSCCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLITER_EN` reader - RTC Oscillator Filter Enable"]
pub struct FLITER_EN_R(crate::FieldReader<bool, bool>);
impl FLITER_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FLITER_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLITER_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLITER_EN` writer - RTC Oscillator Filter Enable"]
pub struct FLITER_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> FLITER_EN_W<'a> {
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
#[doc = "RTC Oscillator 4X Bias Current Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IBIAS_SEL_A {
    #[doc = "0: Selects 2X bias current for RTC oscillator"]
    _2X = 0,
    #[doc = "1: Selects 4X bias current for RTC oscillator"]
    _4X = 1,
}
impl From<IBIAS_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: IBIAS_SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IBIAS_SEL` reader - RTC Oscillator 4X Bias Current Select"]
pub struct IBIAS_SEL_R(crate::FieldReader<bool, IBIAS_SEL_A>);
impl IBIAS_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IBIAS_SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IBIAS_SEL_A {
        match self.bits {
            false => IBIAS_SEL_A::_2X,
            true => IBIAS_SEL_A::_4X,
        }
    }
    #[doc = "Checks if the value of the field is `_2X`"]
    #[inline(always)]
    pub fn is_2x(&self) -> bool {
        **self == IBIAS_SEL_A::_2X
    }
    #[doc = "Checks if the value of the field is `_4X`"]
    #[inline(always)]
    pub fn is_4x(&self) -> bool {
        **self == IBIAS_SEL_A::_4X
    }
}
impl core::ops::Deref for IBIAS_SEL_R {
    type Target = crate::FieldReader<bool, IBIAS_SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IBIAS_SEL` writer - RTC Oscillator 4X Bias Current Select"]
pub struct IBIAS_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> IBIAS_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IBIAS_SEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Selects 2X bias current for RTC oscillator"]
    #[inline(always)]
    pub fn _2x(self) -> &'a mut W {
        self.variant(IBIAS_SEL_A::_2X)
    }
    #[doc = "Selects 4X bias current for RTC oscillator"]
    #[inline(always)]
    pub fn _4x(self) -> &'a mut W {
        self.variant(IBIAS_SEL_A::_4X)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `HYST_EN` reader - RTC Oscillator Hysteresis Buffer Enable"]
pub struct HYST_EN_R(crate::FieldReader<bool, bool>);
impl HYST_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HYST_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HYST_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HYST_EN` writer - RTC Oscillator Hysteresis Buffer Enable"]
pub struct HYST_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> HYST_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `IBIAS_EN` reader - RTC Oscillator Bias Current Enable"]
pub struct IBIAS_EN_R(crate::FieldReader<bool, bool>);
impl IBIAS_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IBIAS_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IBIAS_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IBIAS_EN` writer - RTC Oscillator Bias Current Enable"]
pub struct IBIAS_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> IBIAS_EN_W<'a> {
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
#[doc = "Field `BYPASS` reader - RTC Crystal Bypass"]
pub struct BYPASS_R(crate::FieldReader<bool, bool>);
impl BYPASS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BYPASS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BYPASS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BYPASS` writer - RTC Crystal Bypass"]
pub struct BYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> BYPASS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `OUT32K` reader - RTC 32kHz Square Wave Output"]
pub struct OUT32K_R(crate::FieldReader<bool, bool>);
impl OUT32K_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OUT32K_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUT32K_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUT32K` writer - RTC 32kHz Square Wave Output"]
pub struct OUT32K_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT32K_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - RTC Oscillator Filter Enable"]
    #[inline(always)]
    pub fn fliter_en(&self) -> FLITER_EN_R {
        FLITER_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - RTC Oscillator 4X Bias Current Select"]
    #[inline(always)]
    pub fn ibias_sel(&self) -> IBIAS_SEL_R {
        IBIAS_SEL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - RTC Oscillator Hysteresis Buffer Enable"]
    #[inline(always)]
    pub fn hyst_en(&self) -> HYST_EN_R {
        HYST_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - RTC Oscillator Bias Current Enable"]
    #[inline(always)]
    pub fn ibias_en(&self) -> IBIAS_EN_R {
        IBIAS_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - RTC Crystal Bypass"]
    #[inline(always)]
    pub fn bypass(&self) -> BYPASS_R {
        BYPASS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - RTC 32kHz Square Wave Output"]
    #[inline(always)]
    pub fn out32k(&self) -> OUT32K_R {
        OUT32K_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RTC Oscillator Filter Enable"]
    #[inline(always)]
    pub fn fliter_en(&mut self) -> FLITER_EN_W {
        FLITER_EN_W { w: self }
    }
    #[doc = "Bit 1 - RTC Oscillator 4X Bias Current Select"]
    #[inline(always)]
    pub fn ibias_sel(&mut self) -> IBIAS_SEL_W {
        IBIAS_SEL_W { w: self }
    }
    #[doc = "Bit 2 - RTC Oscillator Hysteresis Buffer Enable"]
    #[inline(always)]
    pub fn hyst_en(&mut self) -> HYST_EN_W {
        HYST_EN_W { w: self }
    }
    #[doc = "Bit 3 - RTC Oscillator Bias Current Enable"]
    #[inline(always)]
    pub fn ibias_en(&mut self) -> IBIAS_EN_W {
        IBIAS_EN_W { w: self }
    }
    #[doc = "Bit 4 - RTC Crystal Bypass"]
    #[inline(always)]
    pub fn bypass(&mut self) -> BYPASS_W {
        BYPASS_W { w: self }
    }
    #[doc = "Bit 5 - RTC 32kHz Square Wave Output"]
    #[inline(always)]
    pub fn out32k(&mut self) -> OUT32K_W {
        OUT32K_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC Oscillator Control Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oscctrl](index.html) module"]
pub struct OSCCTRL_SPEC;
impl crate::RegisterSpec for OSCCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [oscctrl::R](R) reader structure"]
impl crate::Readable for OSCCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [oscctrl::W](W) writer structure"]
impl crate::Writable for OSCCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OSCCTRL to value 0"]
impl crate::Resettable for OSCCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
