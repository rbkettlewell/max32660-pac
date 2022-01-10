#[doc = "Register `I2S_CTRL` reader"]
pub struct R(crate::R<I2S_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2S_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2S_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2S_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2S_CTRL` writer"]
pub struct W(crate::W<I2S_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2S_CTRL_SPEC>;
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
impl From<crate::W<I2S_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2S_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "I2S Mode Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2S_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<I2S_EN_A> for bool {
    #[inline(always)]
    fn from(variant: I2S_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2S_EN` reader - I2S Mode Enable."]
pub struct I2S_EN_R(crate::FieldReader<bool, I2S_EN_A>);
impl I2S_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        I2S_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2S_EN_A {
        match self.bits {
            false => I2S_EN_A::DISABLE,
            true => I2S_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == I2S_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == I2S_EN_A::ENABLE
    }
}
impl core::ops::Deref for I2S_EN_R {
    type Target = crate::FieldReader<bool, I2S_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2S_EN` writer - I2S Mode Enable."]
pub struct I2S_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2S_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(I2S_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(I2S_EN_A::ENABLE)
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
#[doc = "I2S Mute transmit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2S_MUTE_A {
    #[doc = "0: Normal Transmit."]
    NORMAL = 0,
    #[doc = "1: Transmit data is replaced with 0."]
    REPLACED = 1,
}
impl From<I2S_MUTE_A> for bool {
    #[inline(always)]
    fn from(variant: I2S_MUTE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2S_MUTE` reader - I2S Mute transmit."]
pub struct I2S_MUTE_R(crate::FieldReader<bool, I2S_MUTE_A>);
impl I2S_MUTE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        I2S_MUTE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2S_MUTE_A {
        match self.bits {
            false => I2S_MUTE_A::NORMAL,
            true => I2S_MUTE_A::REPLACED,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        **self == I2S_MUTE_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `REPLACED`"]
    #[inline(always)]
    pub fn is_replaced(&self) -> bool {
        **self == I2S_MUTE_A::REPLACED
    }
}
impl core::ops::Deref for I2S_MUTE_R {
    type Target = crate::FieldReader<bool, I2S_MUTE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2S_MUTE` writer - I2S Mute transmit."]
pub struct I2S_MUTE_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_MUTE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2S_MUTE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Normal Transmit."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(I2S_MUTE_A::NORMAL)
    }
    #[doc = "Transmit data is replaced with 0."]
    #[inline(always)]
    pub fn replaced(self) -> &'a mut W {
        self.variant(I2S_MUTE_A::REPLACED)
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
#[doc = "I2S Pause transmit/receive.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2S_PAUSE_A {
    #[doc = "0: Normal Transmit."]
    NORMAL = 0,
    #[doc = "1: Halt transmit and receive FIFO and DMA access, transmit 0's."]
    HALT = 1,
}
impl From<I2S_PAUSE_A> for bool {
    #[inline(always)]
    fn from(variant: I2S_PAUSE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2S_PAUSE` reader - I2S Pause transmit/receive."]
pub struct I2S_PAUSE_R(crate::FieldReader<bool, I2S_PAUSE_A>);
impl I2S_PAUSE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        I2S_PAUSE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2S_PAUSE_A {
        match self.bits {
            false => I2S_PAUSE_A::NORMAL,
            true => I2S_PAUSE_A::HALT,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        **self == I2S_PAUSE_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `HALT`"]
    #[inline(always)]
    pub fn is_halt(&self) -> bool {
        **self == I2S_PAUSE_A::HALT
    }
}
impl core::ops::Deref for I2S_PAUSE_R {
    type Target = crate::FieldReader<bool, I2S_PAUSE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2S_PAUSE` writer - I2S Pause transmit/receive."]
pub struct I2S_PAUSE_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_PAUSE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2S_PAUSE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Normal Transmit."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(I2S_PAUSE_A::NORMAL)
    }
    #[doc = "Halt transmit and receive FIFO and DMA access, transmit 0's."]
    #[inline(always)]
    pub fn halt(self) -> &'a mut W {
        self.variant(I2S_PAUSE_A::HALT)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "I2S Monophonic Audio Mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2S_MONO_A {
    #[doc = "0: Stereophonic audio."]
    STEREOPHONIC = 0,
    #[doc = "1: Monophonic audio format.Each transmit data word is replicated on both left/right channels. Receive data is taken from left channel, right channel receive data is ignored."]
    MONOPHONIC = 1,
}
impl From<I2S_MONO_A> for bool {
    #[inline(always)]
    fn from(variant: I2S_MONO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2S_MONO` reader - I2S Monophonic Audio Mode."]
pub struct I2S_MONO_R(crate::FieldReader<bool, I2S_MONO_A>);
impl I2S_MONO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        I2S_MONO_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2S_MONO_A {
        match self.bits {
            false => I2S_MONO_A::STEREOPHONIC,
            true => I2S_MONO_A::MONOPHONIC,
        }
    }
    #[doc = "Checks if the value of the field is `STEREOPHONIC`"]
    #[inline(always)]
    pub fn is_stereophonic(&self) -> bool {
        **self == I2S_MONO_A::STEREOPHONIC
    }
    #[doc = "Checks if the value of the field is `MONOPHONIC`"]
    #[inline(always)]
    pub fn is_monophonic(&self) -> bool {
        **self == I2S_MONO_A::MONOPHONIC
    }
}
impl core::ops::Deref for I2S_MONO_R {
    type Target = crate::FieldReader<bool, I2S_MONO_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2S_MONO` writer - I2S Monophonic Audio Mode."]
pub struct I2S_MONO_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_MONO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2S_MONO_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Stereophonic audio."]
    #[inline(always)]
    pub fn stereophonic(self) -> &'a mut W {
        self.variant(I2S_MONO_A::STEREOPHONIC)
    }
    #[doc = "Monophonic audio format.Each transmit data word is replicated on both left/right channels. Receive data is taken from left channel, right channel receive data is ignored."]
    #[inline(always)]
    pub fn monophonic(self) -> &'a mut W {
        self.variant(I2S_MONO_A::MONOPHONIC)
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
#[doc = "I2S Left Justify.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2S_LJ_A {
    #[doc = "0: Normal I2S audio protocol."]
    NORMAL = 0,
    #[doc = "1: Audio data is synchronized with SSEL."]
    REPLACED = 1,
}
impl From<I2S_LJ_A> for bool {
    #[inline(always)]
    fn from(variant: I2S_LJ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2S_LJ` reader - I2S Left Justify."]
pub struct I2S_LJ_R(crate::FieldReader<bool, I2S_LJ_A>);
impl I2S_LJ_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        I2S_LJ_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2S_LJ_A {
        match self.bits {
            false => I2S_LJ_A::NORMAL,
            true => I2S_LJ_A::REPLACED,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        **self == I2S_LJ_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `REPLACED`"]
    #[inline(always)]
    pub fn is_replaced(&self) -> bool {
        **self == I2S_LJ_A::REPLACED
    }
}
impl core::ops::Deref for I2S_LJ_R {
    type Target = crate::FieldReader<bool, I2S_LJ_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2S_LJ` writer - I2S Left Justify."]
pub struct I2S_LJ_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_LJ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2S_LJ_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Normal I2S audio protocol."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(I2S_LJ_A::NORMAL)
    }
    #[doc = "Audio data is synchronized with SSEL."]
    #[inline(always)]
    pub fn replaced(self) -> &'a mut W {
        self.variant(I2S_LJ_A::REPLACED)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - I2S Mode Enable."]
    #[inline(always)]
    pub fn i2s_en(&self) -> I2S_EN_R {
        I2S_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - I2S Mute transmit."]
    #[inline(always)]
    pub fn i2s_mute(&self) -> I2S_MUTE_R {
        I2S_MUTE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - I2S Pause transmit/receive."]
    #[inline(always)]
    pub fn i2s_pause(&self) -> I2S_PAUSE_R {
        I2S_PAUSE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - I2S Monophonic Audio Mode."]
    #[inline(always)]
    pub fn i2s_mono(&self) -> I2S_MONO_R {
        I2S_MONO_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - I2S Left Justify."]
    #[inline(always)]
    pub fn i2s_lj(&self) -> I2S_LJ_R {
        I2S_LJ_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2S Mode Enable."]
    #[inline(always)]
    pub fn i2s_en(&mut self) -> I2S_EN_W {
        I2S_EN_W { w: self }
    }
    #[doc = "Bit 1 - I2S Mute transmit."]
    #[inline(always)]
    pub fn i2s_mute(&mut self) -> I2S_MUTE_W {
        I2S_MUTE_W { w: self }
    }
    #[doc = "Bit 2 - I2S Pause transmit/receive."]
    #[inline(always)]
    pub fn i2s_pause(&mut self) -> I2S_PAUSE_W {
        I2S_PAUSE_W { w: self }
    }
    #[doc = "Bit 3 - I2S Monophonic Audio Mode."]
    #[inline(always)]
    pub fn i2s_mono(&mut self) -> I2S_MONO_W {
        I2S_MONO_W { w: self }
    }
    #[doc = "Bit 4 - I2S Left Justify."]
    #[inline(always)]
    pub fn i2s_lj(&mut self) -> I2S_LJ_W {
        I2S_LJ_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2S Control Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2s_ctrl](index.html) module"]
pub struct I2S_CTRL_SPEC;
impl crate::RegisterSpec for I2S_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2s_ctrl::R](R) reader structure"]
impl crate::Readable for I2S_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2s_ctrl::W](W) writer structure"]
impl crate::Writable for I2S_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets I2S_CTRL to value 0"]
impl crate::Resettable for I2S_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
