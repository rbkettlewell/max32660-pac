#[doc = "Register `INT_FL` reader"]
pub struct R(crate::R<INT_FL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_FL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_FL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_FL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INT_FL` writer"]
pub struct W(crate::W<INT_FL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_FL_SPEC>;
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
impl From<crate::W<INT_FL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_FL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RX_FRAME_ERROR` reader - FLAG for RX Frame Error Interrupt."]
pub struct RX_FRAME_ERROR_R(crate::FieldReader<bool, bool>);
impl RX_FRAME_ERROR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_FRAME_ERROR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_FRAME_ERROR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_FRAME_ERROR` writer - FLAG for RX Frame Error Interrupt."]
pub struct RX_FRAME_ERROR_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_FRAME_ERROR_W<'a> {
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
#[doc = "Field `RX_PARITY_ERROR` reader - FLAG for RX Parity Error interrupt."]
pub struct RX_PARITY_ERROR_R(crate::FieldReader<bool, bool>);
impl RX_PARITY_ERROR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_PARITY_ERROR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_PARITY_ERROR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_PARITY_ERROR` writer - FLAG for RX Parity Error interrupt."]
pub struct RX_PARITY_ERROR_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_PARITY_ERROR_W<'a> {
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
#[doc = "Field `CTS_CHANGE` reader - FLAG for CTS signal change interrupt."]
pub struct CTS_CHANGE_R(crate::FieldReader<bool, bool>);
impl CTS_CHANGE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CTS_CHANGE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTS_CHANGE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTS_CHANGE` writer - FLAG for CTS signal change interrupt."]
pub struct CTS_CHANGE_W<'a> {
    w: &'a mut W,
}
impl<'a> CTS_CHANGE_W<'a> {
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
#[doc = "Field `RX_OVERRUN` reader - FLAG for RX FIFO Overrun interrupt."]
pub struct RX_OVERRUN_R(crate::FieldReader<bool, bool>);
impl RX_OVERRUN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_OVERRUN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_OVERRUN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_OVERRUN` writer - FLAG for RX FIFO Overrun interrupt."]
pub struct RX_OVERRUN_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_OVERRUN_W<'a> {
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
#[doc = "Field `RX_FIFO_THRESH` reader - FLAG for interrupt when RX FIFO reaches the number of bytes configured by the RXTHD field."]
pub struct RX_FIFO_THRESH_R(crate::FieldReader<bool, bool>);
impl RX_FIFO_THRESH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_FIFO_THRESH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_FIFO_THRESH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_FIFO_THRESH` writer - FLAG for interrupt when RX FIFO reaches the number of bytes configured by the RXTHD field."]
pub struct RX_FIFO_THRESH_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_FIFO_THRESH_W<'a> {
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
#[doc = "Field `TX_FIFO_ALMOST_EMPTY` reader - FLAG for interrupt when TX FIFO has only one byte remaining."]
pub struct TX_FIFO_ALMOST_EMPTY_R(crate::FieldReader<bool, bool>);
impl TX_FIFO_ALMOST_EMPTY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_FIFO_ALMOST_EMPTY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_FIFO_ALMOST_EMPTY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_FIFO_ALMOST_EMPTY` writer - FLAG for interrupt when TX FIFO has only one byte remaining."]
pub struct TX_FIFO_ALMOST_EMPTY_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_FIFO_ALMOST_EMPTY_W<'a> {
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
#[doc = "Field `TX_FIFO_THRESH` reader - FLAG for interrupt when TX FIFO reaches the number of bytes configured by the TXTHD field."]
pub struct TX_FIFO_THRESH_R(crate::FieldReader<bool, bool>);
impl TX_FIFO_THRESH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_FIFO_THRESH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_FIFO_THRESH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_FIFO_THRESH` writer - FLAG for interrupt when TX FIFO reaches the number of bytes configured by the TXTHD field."]
pub struct TX_FIFO_THRESH_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_FIFO_THRESH_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `BREAK` reader - FLAG for received BREAK character interrupt."]
pub struct BREAK_R(crate::FieldReader<bool, bool>);
impl BREAK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BREAK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BREAK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BREAK` writer - FLAG for received BREAK character interrupt."]
pub struct BREAK_W<'a> {
    w: &'a mut W,
}
impl<'a> BREAK_W<'a> {
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
#[doc = "Field `RX_TIMEOUT` reader - FLAG for RX Timeout Interrupt. Trigger if there is no RX communication during n UART characters (n=UART_CN.RXTO)."]
pub struct RX_TIMEOUT_R(crate::FieldReader<bool, bool>);
impl RX_TIMEOUT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_TIMEOUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_TIMEOUT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_TIMEOUT` writer - FLAG for RX Timeout Interrupt. Trigger if there is no RX communication during n UART characters (n=UART_CN.RXTO)."]
pub struct RX_TIMEOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_TIMEOUT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `LAST_BREAK` reader - FLAG for Last break character interrupt."]
pub struct LAST_BREAK_R(crate::FieldReader<bool, bool>);
impl LAST_BREAK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LAST_BREAK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LAST_BREAK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LAST_BREAK` writer - FLAG for Last break character interrupt."]
pub struct LAST_BREAK_W<'a> {
    w: &'a mut W,
}
impl<'a> LAST_BREAK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - FLAG for RX Frame Error Interrupt."]
    #[inline(always)]
    pub fn rx_frame_error(&self) -> RX_FRAME_ERROR_R {
        RX_FRAME_ERROR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - FLAG for RX Parity Error interrupt."]
    #[inline(always)]
    pub fn rx_parity_error(&self) -> RX_PARITY_ERROR_R {
        RX_PARITY_ERROR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - FLAG for CTS signal change interrupt."]
    #[inline(always)]
    pub fn cts_change(&self) -> CTS_CHANGE_R {
        CTS_CHANGE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - FLAG for RX FIFO Overrun interrupt."]
    #[inline(always)]
    pub fn rx_overrun(&self) -> RX_OVERRUN_R {
        RX_OVERRUN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - FLAG for interrupt when RX FIFO reaches the number of bytes configured by the RXTHD field."]
    #[inline(always)]
    pub fn rx_fifo_thresh(&self) -> RX_FIFO_THRESH_R {
        RX_FIFO_THRESH_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - FLAG for interrupt when TX FIFO has only one byte remaining."]
    #[inline(always)]
    pub fn tx_fifo_almost_empty(&self) -> TX_FIFO_ALMOST_EMPTY_R {
        TX_FIFO_ALMOST_EMPTY_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - FLAG for interrupt when TX FIFO reaches the number of bytes configured by the TXTHD field."]
    #[inline(always)]
    pub fn tx_fifo_thresh(&self) -> TX_FIFO_THRESH_R {
        TX_FIFO_THRESH_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - FLAG for received BREAK character interrupt."]
    #[inline(always)]
    pub fn break_(&self) -> BREAK_R {
        BREAK_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - FLAG for RX Timeout Interrupt. Trigger if there is no RX communication during n UART characters (n=UART_CN.RXTO)."]
    #[inline(always)]
    pub fn rx_timeout(&self) -> RX_TIMEOUT_R {
        RX_TIMEOUT_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - FLAG for Last break character interrupt."]
    #[inline(always)]
    pub fn last_break(&self) -> LAST_BREAK_R {
        LAST_BREAK_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FLAG for RX Frame Error Interrupt."]
    #[inline(always)]
    pub fn rx_frame_error(&mut self) -> RX_FRAME_ERROR_W {
        RX_FRAME_ERROR_W { w: self }
    }
    #[doc = "Bit 1 - FLAG for RX Parity Error interrupt."]
    #[inline(always)]
    pub fn rx_parity_error(&mut self) -> RX_PARITY_ERROR_W {
        RX_PARITY_ERROR_W { w: self }
    }
    #[doc = "Bit 2 - FLAG for CTS signal change interrupt."]
    #[inline(always)]
    pub fn cts_change(&mut self) -> CTS_CHANGE_W {
        CTS_CHANGE_W { w: self }
    }
    #[doc = "Bit 3 - FLAG for RX FIFO Overrun interrupt."]
    #[inline(always)]
    pub fn rx_overrun(&mut self) -> RX_OVERRUN_W {
        RX_OVERRUN_W { w: self }
    }
    #[doc = "Bit 4 - FLAG for interrupt when RX FIFO reaches the number of bytes configured by the RXTHD field."]
    #[inline(always)]
    pub fn rx_fifo_thresh(&mut self) -> RX_FIFO_THRESH_W {
        RX_FIFO_THRESH_W { w: self }
    }
    #[doc = "Bit 5 - FLAG for interrupt when TX FIFO has only one byte remaining."]
    #[inline(always)]
    pub fn tx_fifo_almost_empty(&mut self) -> TX_FIFO_ALMOST_EMPTY_W {
        TX_FIFO_ALMOST_EMPTY_W { w: self }
    }
    #[doc = "Bit 6 - FLAG for interrupt when TX FIFO reaches the number of bytes configured by the TXTHD field."]
    #[inline(always)]
    pub fn tx_fifo_thresh(&mut self) -> TX_FIFO_THRESH_W {
        TX_FIFO_THRESH_W { w: self }
    }
    #[doc = "Bit 7 - FLAG for received BREAK character interrupt."]
    #[inline(always)]
    pub fn break_(&mut self) -> BREAK_W {
        BREAK_W { w: self }
    }
    #[doc = "Bit 8 - FLAG for RX Timeout Interrupt. Trigger if there is no RX communication during n UART characters (n=UART_CN.RXTO)."]
    #[inline(always)]
    pub fn rx_timeout(&mut self) -> RX_TIMEOUT_W {
        RX_TIMEOUT_W { w: self }
    }
    #[doc = "Bit 9 - FLAG for Last break character interrupt."]
    #[inline(always)]
    pub fn last_break(&mut self) -> LAST_BREAK_W {
        LAST_BREAK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Status Flags.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_fl](index.html) module"]
pub struct INT_FL_SPEC;
impl crate::RegisterSpec for INT_FL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_fl::R](R) reader structure"]
impl crate::Readable for INT_FL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [int_fl::W](W) writer structure"]
impl crate::Writable for INT_FL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INT_FL to value 0"]
impl crate::Resettable for INT_FL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
