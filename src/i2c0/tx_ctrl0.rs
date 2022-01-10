#[doc = "Register `TX_CTRL0` reader"]
pub struct R(crate::R<TX_CTRL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TX_CTRL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TX_CTRL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TX_CTRL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TX_CTRL0` writer"]
pub struct W(crate::W<TX_CTRL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TX_CTRL0_SPEC>;
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
impl From<crate::W<TX_CTRL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TX_CTRL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX_PRELOAD` reader - Transmit FIFO Preaload Mode. Setting this bit will allow for high speed application to preload the transmit FIFO prior to Slave Address Match."]
pub struct TX_PRELOAD_R(crate::FieldReader<bool, bool>);
impl TX_PRELOAD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_PRELOAD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_PRELOAD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_PRELOAD` writer - Transmit FIFO Preaload Mode. Setting this bit will allow for high speed application to preload the transmit FIFO prior to Slave Address Match."]
pub struct TX_PRELOAD_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_PRELOAD_W<'a> {
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
#[doc = "Transmit FIFO Ready Manual Mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_READY_MODE_A {
    #[doc = "0: HW control of I2CTXRDY enabled."]
    EN = 0,
    #[doc = "1: HW control of I2CTXRDY disabled."]
    DIS = 1,
}
impl From<TX_READY_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: TX_READY_MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TX_READY_MODE` reader - Transmit FIFO Ready Manual Mode."]
pub struct TX_READY_MODE_R(crate::FieldReader<bool, TX_READY_MODE_A>);
impl TX_READY_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_READY_MODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_READY_MODE_A {
        match self.bits {
            false => TX_READY_MODE_A::EN,
            true => TX_READY_MODE_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == TX_READY_MODE_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == TX_READY_MODE_A::DIS
    }
}
impl core::ops::Deref for TX_READY_MODE_R {
    type Target = crate::FieldReader<bool, TX_READY_MODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_READY_MODE` writer - Transmit FIFO Ready Manual Mode."]
pub struct TX_READY_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_READY_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TX_READY_MODE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "HW control of I2CTXRDY enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TX_READY_MODE_A::EN)
    }
    #[doc = "HW control of I2CTXRDY disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TX_READY_MODE_A::DIS)
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
#[doc = "Transmit FIFO Flush. This bit is automatically cleared to 0 after the operation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_FLUSH_A {
    #[doc = "0: FIFO not flushed."]
    NOT_FLUSHED = 0,
    #[doc = "1: Flush TX_FIFO."]
    FLUSH = 1,
}
impl From<TX_FLUSH_A> for bool {
    #[inline(always)]
    fn from(variant: TX_FLUSH_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TX_FLUSH` reader - Transmit FIFO Flush. This bit is automatically cleared to 0 after the operation."]
pub struct TX_FLUSH_R(crate::FieldReader<bool, TX_FLUSH_A>);
impl TX_FLUSH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_FLUSH_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_FLUSH_A {
        match self.bits {
            false => TX_FLUSH_A::NOT_FLUSHED,
            true => TX_FLUSH_A::FLUSH,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_FLUSHED`"]
    #[inline(always)]
    pub fn is_not_flushed(&self) -> bool {
        **self == TX_FLUSH_A::NOT_FLUSHED
    }
    #[doc = "Checks if the value of the field is `FLUSH`"]
    #[inline(always)]
    pub fn is_flush(&self) -> bool {
        **self == TX_FLUSH_A::FLUSH
    }
}
impl core::ops::Deref for TX_FLUSH_R {
    type Target = crate::FieldReader<bool, TX_FLUSH_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_FLUSH` writer - Transmit FIFO Flush. This bit is automatically cleared to 0 after the operation."]
pub struct TX_FLUSH_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_FLUSH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TX_FLUSH_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "FIFO not flushed."]
    #[inline(always)]
    pub fn not_flushed(self) -> &'a mut W {
        self.variant(TX_FLUSH_A::NOT_FLUSHED)
    }
    #[doc = "Flush TX_FIFO."]
    #[inline(always)]
    pub fn flush(self) -> &'a mut W {
        self.variant(TX_FLUSH_A::FLUSH)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `TX_THRESH` reader - Transmit FIFO Threshold. These bits define the TX_FIFO interrupt threshold."]
pub struct TX_THRESH_R(crate::FieldReader<u8, u8>);
impl TX_THRESH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TX_THRESH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_THRESH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_THRESH` writer - Transmit FIFO Threshold. These bits define the TX_FIFO interrupt threshold."]
pub struct TX_THRESH_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_THRESH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Transmit FIFO Preaload Mode. Setting this bit will allow for high speed application to preload the transmit FIFO prior to Slave Address Match."]
    #[inline(always)]
    pub fn tx_preload(&self) -> TX_PRELOAD_R {
        TX_PRELOAD_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transmit FIFO Ready Manual Mode."]
    #[inline(always)]
    pub fn tx_ready_mode(&self) -> TX_READY_MODE_R {
        TX_READY_MODE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Transmit FIFO Flush. This bit is automatically cleared to 0 after the operation."]
    #[inline(always)]
    pub fn tx_flush(&self) -> TX_FLUSH_R {
        TX_FLUSH_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - Transmit FIFO Threshold. These bits define the TX_FIFO interrupt threshold."]
    #[inline(always)]
    pub fn tx_thresh(&self) -> TX_THRESH_R {
        TX_THRESH_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit FIFO Preaload Mode. Setting this bit will allow for high speed application to preload the transmit FIFO prior to Slave Address Match."]
    #[inline(always)]
    pub fn tx_preload(&mut self) -> TX_PRELOAD_W {
        TX_PRELOAD_W { w: self }
    }
    #[doc = "Bit 1 - Transmit FIFO Ready Manual Mode."]
    #[inline(always)]
    pub fn tx_ready_mode(&mut self) -> TX_READY_MODE_W {
        TX_READY_MODE_W { w: self }
    }
    #[doc = "Bit 7 - Transmit FIFO Flush. This bit is automatically cleared to 0 after the operation."]
    #[inline(always)]
    pub fn tx_flush(&mut self) -> TX_FLUSH_W {
        TX_FLUSH_W { w: self }
    }
    #[doc = "Bits 8:11 - Transmit FIFO Threshold. These bits define the TX_FIFO interrupt threshold."]
    #[inline(always)]
    pub fn tx_thresh(&mut self) -> TX_THRESH_W {
        TX_THRESH_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmit Control Register 0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_ctrl0](index.html) module"]
pub struct TX_CTRL0_SPEC;
impl crate::RegisterSpec for TX_CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tx_ctrl0::R](R) reader structure"]
impl crate::Readable for TX_CTRL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tx_ctrl0::W](W) writer structure"]
impl crate::Writable for TX_CTRL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TX_CTRL0 to value 0"]
impl crate::Resettable for TX_CTRL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
