#[doc = "Register `RX_CTRL0` reader"]
pub struct R(crate::R<RX_CTRL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_CTRL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_CTRL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_CTRL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RX_CTRL0` writer"]
pub struct W(crate::W<RX_CTRL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RX_CTRL0_SPEC>;
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
impl From<crate::W<RX_CTRL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RX_CTRL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Do Not Respond.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DNR_A {
    #[doc = "0: Always respond to address match."]
    RESPOND = 0,
    #[doc = "1: Do not respond to address match when RX_FIFO is not empty."]
    NOT_RESPOND_RX_FIFO_EMPTY = 1,
}
impl From<DNR_A> for bool {
    #[inline(always)]
    fn from(variant: DNR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DNR` reader - Do Not Respond."]
pub struct DNR_R(crate::FieldReader<bool, DNR_A>);
impl DNR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DNR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DNR_A {
        match self.bits {
            false => DNR_A::RESPOND,
            true => DNR_A::NOT_RESPOND_RX_FIFO_EMPTY,
        }
    }
    #[doc = "Checks if the value of the field is `RESPOND`"]
    #[inline(always)]
    pub fn is_respond(&self) -> bool {
        **self == DNR_A::RESPOND
    }
    #[doc = "Checks if the value of the field is `NOT_RESPOND_RX_FIFO_EMPTY`"]
    #[inline(always)]
    pub fn is_not_respond_rx_fifo_empty(&self) -> bool {
        **self == DNR_A::NOT_RESPOND_RX_FIFO_EMPTY
    }
}
impl core::ops::Deref for DNR_R {
    type Target = crate::FieldReader<bool, DNR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DNR` writer - Do Not Respond."]
pub struct DNR_W<'a> {
    w: &'a mut W,
}
impl<'a> DNR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DNR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Always respond to address match."]
    #[inline(always)]
    pub fn respond(self) -> &'a mut W {
        self.variant(DNR_A::RESPOND)
    }
    #[doc = "Do not respond to address match when RX_FIFO is not empty."]
    #[inline(always)]
    pub fn not_respond_rx_fifo_empty(self) -> &'a mut W {
        self.variant(DNR_A::NOT_RESPOND_RX_FIFO_EMPTY)
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
#[doc = "Receive FIFO Flush. This bit is automatically cleared to 0 after the operation. Setting this bit to 1 will affect RX_FIFO status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_FLUSH_A {
    #[doc = "0: FIFO not flushed."]
    NOT_FLUSHED = 0,
    #[doc = "1: Flush RX_FIFO."]
    FLUSH = 1,
}
impl From<RX_FLUSH_A> for bool {
    #[inline(always)]
    fn from(variant: RX_FLUSH_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RX_FLUSH` reader - Receive FIFO Flush. This bit is automatically cleared to 0 after the operation. Setting this bit to 1 will affect RX_FIFO status."]
pub struct RX_FLUSH_R(crate::FieldReader<bool, RX_FLUSH_A>);
impl RX_FLUSH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_FLUSH_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_FLUSH_A {
        match self.bits {
            false => RX_FLUSH_A::NOT_FLUSHED,
            true => RX_FLUSH_A::FLUSH,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_FLUSHED`"]
    #[inline(always)]
    pub fn is_not_flushed(&self) -> bool {
        **self == RX_FLUSH_A::NOT_FLUSHED
    }
    #[doc = "Checks if the value of the field is `FLUSH`"]
    #[inline(always)]
    pub fn is_flush(&self) -> bool {
        **self == RX_FLUSH_A::FLUSH
    }
}
impl core::ops::Deref for RX_FLUSH_R {
    type Target = crate::FieldReader<bool, RX_FLUSH_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_FLUSH` writer - Receive FIFO Flush. This bit is automatically cleared to 0 after the operation. Setting this bit to 1 will affect RX_FIFO status."]
pub struct RX_FLUSH_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_FLUSH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX_FLUSH_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "FIFO not flushed."]
    #[inline(always)]
    pub fn not_flushed(self) -> &'a mut W {
        self.variant(RX_FLUSH_A::NOT_FLUSHED)
    }
    #[doc = "Flush RX_FIFO."]
    #[inline(always)]
    pub fn flush(self) -> &'a mut W {
        self.variant(RX_FLUSH_A::FLUSH)
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
#[doc = "Field `RX_THRESH` reader - Receive FIFO Threshold. These bits define the RX_FIFO interrupt threshold."]
pub struct RX_THRESH_R(crate::FieldReader<u8, u8>);
impl RX_THRESH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RX_THRESH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_THRESH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_THRESH` writer - Receive FIFO Threshold. These bits define the RX_FIFO interrupt threshold."]
pub struct RX_THRESH_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_THRESH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Do Not Respond."]
    #[inline(always)]
    pub fn dnr(&self) -> DNR_R {
        DNR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 7 - Receive FIFO Flush. This bit is automatically cleared to 0 after the operation. Setting this bit to 1 will affect RX_FIFO status."]
    #[inline(always)]
    pub fn rx_flush(&self) -> RX_FLUSH_R {
        RX_FLUSH_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - Receive FIFO Threshold. These bits define the RX_FIFO interrupt threshold."]
    #[inline(always)]
    pub fn rx_thresh(&self) -> RX_THRESH_R {
        RX_THRESH_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Do Not Respond."]
    #[inline(always)]
    pub fn dnr(&mut self) -> DNR_W {
        DNR_W { w: self }
    }
    #[doc = "Bit 7 - Receive FIFO Flush. This bit is automatically cleared to 0 after the operation. Setting this bit to 1 will affect RX_FIFO status."]
    #[inline(always)]
    pub fn rx_flush(&mut self) -> RX_FLUSH_W {
        RX_FLUSH_W { w: self }
    }
    #[doc = "Bits 8:11 - Receive FIFO Threshold. These bits define the RX_FIFO interrupt threshold."]
    #[inline(always)]
    pub fn rx_thresh(&mut self) -> RX_THRESH_W {
        RX_THRESH_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Receive Control Register 0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_ctrl0](index.html) module"]
pub struct RX_CTRL0_SPEC;
impl crate::RegisterSpec for RX_CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rx_ctrl0::R](R) reader structure"]
impl crate::Readable for RX_CTRL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rx_ctrl0::W](W) writer structure"]
impl crate::Writable for RX_CTRL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RX_CTRL0 to value 0"]
impl crate::Resettable for RX_CTRL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
