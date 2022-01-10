#[doc = "Register `CN` reader"]
pub struct R(crate::R<CN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CN` writer"]
pub struct W(crate::W<CN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CN_SPEC>;
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
impl From<crate::W<CN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Channel 0 Interrupt Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH0_IEN_A {
    #[doc = "0: Disable."]
    DIS = 0,
    #[doc = "1: Enable."]
    EN = 1,
}
impl From<CH0_IEN_A> for bool {
    #[inline(always)]
    fn from(variant: CH0_IEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH0_IEN` reader - Channel 0 Interrupt Enable."]
pub struct CH0_IEN_R(crate::FieldReader<bool, CH0_IEN_A>);
impl CH0_IEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH0_IEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH0_IEN_A {
        match self.bits {
            false => CH0_IEN_A::DIS,
            true => CH0_IEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == CH0_IEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == CH0_IEN_A::EN
    }
}
impl core::ops::Deref for CH0_IEN_R {
    type Target = crate::FieldReader<bool, CH0_IEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH0_IEN` writer - Channel 0 Interrupt Enable."]
pub struct CH0_IEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0_IEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH0_IEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(CH0_IEN_A::DIS)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(CH0_IEN_A::EN)
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
#[doc = "Field `CH1_IEN` reader - Channel 1 Interrupt Enable."]
pub struct CH1_IEN_R(crate::FieldReader<bool, bool>);
impl CH1_IEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH1_IEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH1_IEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH1_IEN` writer - Channel 1 Interrupt Enable."]
pub struct CH1_IEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1_IEN_W<'a> {
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
#[doc = "Field `CH2_IEN` reader - Channel 2 Interrupt Enable."]
pub struct CH2_IEN_R(crate::FieldReader<bool, bool>);
impl CH2_IEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH2_IEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH2_IEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH2_IEN` writer - Channel 2 Interrupt Enable."]
pub struct CH2_IEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2_IEN_W<'a> {
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
#[doc = "Field `CH3_IEN` reader - Channel 3 Interrupt Enable."]
pub struct CH3_IEN_R(crate::FieldReader<bool, bool>);
impl CH3_IEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH3_IEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH3_IEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH3_IEN` writer - Channel 3 Interrupt Enable."]
pub struct CH3_IEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CH3_IEN_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Channel 0 Interrupt Enable."]
    #[inline(always)]
    pub fn ch0_ien(&self) -> CH0_IEN_R {
        CH0_IEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Channel 1 Interrupt Enable."]
    #[inline(always)]
    pub fn ch1_ien(&self) -> CH1_IEN_R {
        CH1_IEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Channel 2 Interrupt Enable."]
    #[inline(always)]
    pub fn ch2_ien(&self) -> CH2_IEN_R {
        CH2_IEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Channel 3 Interrupt Enable."]
    #[inline(always)]
    pub fn ch3_ien(&self) -> CH3_IEN_R {
        CH3_IEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel 0 Interrupt Enable."]
    #[inline(always)]
    pub fn ch0_ien(&mut self) -> CH0_IEN_W {
        CH0_IEN_W { w: self }
    }
    #[doc = "Bit 1 - Channel 1 Interrupt Enable."]
    #[inline(always)]
    pub fn ch1_ien(&mut self) -> CH1_IEN_W {
        CH1_IEN_W { w: self }
    }
    #[doc = "Bit 2 - Channel 2 Interrupt Enable."]
    #[inline(always)]
    pub fn ch2_ien(&mut self) -> CH2_IEN_W {
        CH2_IEN_W { w: self }
    }
    #[doc = "Bit 3 - Channel 3 Interrupt Enable."]
    #[inline(always)]
    pub fn ch3_ien(&mut self) -> CH3_IEN_W {
        CH3_IEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Control Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cn](index.html) module"]
pub struct CN_SPEC;
impl crate::RegisterSpec for CN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cn::R](R) reader structure"]
impl crate::Readable for CN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cn::W](W) writer structure"]
impl crate::Writable for CN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CN to value 0"]
impl crate::Resettable for CN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
