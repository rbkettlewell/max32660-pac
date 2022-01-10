#[doc = "Register `THRESH_CTRL` reader"]
pub struct R(crate::R<THRESH_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<THRESH_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<THRESH_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<THRESH_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `THRESH_CTRL` writer"]
pub struct W(crate::W<THRESH_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<THRESH_CTRL_SPEC>;
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
impl From<crate::W<THRESH_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<THRESH_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RX_FIFO_THRESH` reader - RX FIFO Threshold Level.When the RX FIFO reaches this many bytes or higher, UARTn_INFTL.rx_fifo_level is set."]
pub struct RX_FIFO_THRESH_R(crate::FieldReader<u8, u8>);
impl RX_FIFO_THRESH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RX_FIFO_THRESH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_FIFO_THRESH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_FIFO_THRESH` writer - RX FIFO Threshold Level.When the RX FIFO reaches this many bytes or higher, UARTn_INFTL.rx_fifo_level is set."]
pub struct RX_FIFO_THRESH_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_FIFO_THRESH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
#[doc = "Field `TX_FIFO_THRESH` reader - TX FIFO Threshold Level. When the TX FIFO reaches this many bytes or higher, UARTn_INTFL.tx_fifo_level is set."]
pub struct TX_FIFO_THRESH_R(crate::FieldReader<u8, u8>);
impl TX_FIFO_THRESH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TX_FIFO_THRESH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_FIFO_THRESH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_FIFO_THRESH` writer - TX FIFO Threshold Level. When the TX FIFO reaches this many bytes or higher, UARTn_INTFL.tx_fifo_level is set."]
pub struct TX_FIFO_THRESH_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_FIFO_THRESH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | ((value as u32 & 0x3f) << 8);
        self.w
    }
}
#[doc = "Field `RTS_FIFO_THRESH` reader - RTS threshold control. When the RX FIFO reaches this many bytes or higher, the RTS output signal is deasserted, informing the transmitting UART to stop sending data to this UART."]
pub struct RTS_FIFO_THRESH_R(crate::FieldReader<u8, u8>);
impl RTS_FIFO_THRESH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RTS_FIFO_THRESH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTS_FIFO_THRESH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTS_FIFO_THRESH` writer - RTS threshold control. When the RX FIFO reaches this many bytes or higher, the RTS output signal is deasserted, informing the transmitting UART to stop sending data to this UART."]
pub struct RTS_FIFO_THRESH_W<'a> {
    w: &'a mut W,
}
impl<'a> RTS_FIFO_THRESH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | ((value as u32 & 0x3f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - RX FIFO Threshold Level.When the RX FIFO reaches this many bytes or higher, UARTn_INFTL.rx_fifo_level is set."]
    #[inline(always)]
    pub fn rx_fifo_thresh(&self) -> RX_FIFO_THRESH_R {
        RX_FIFO_THRESH_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - TX FIFO Threshold Level. When the TX FIFO reaches this many bytes or higher, UARTn_INTFL.tx_fifo_level is set."]
    #[inline(always)]
    pub fn tx_fifo_thresh(&self) -> TX_FIFO_THRESH_R {
        TX_FIFO_THRESH_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - RTS threshold control. When the RX FIFO reaches this many bytes or higher, the RTS output signal is deasserted, informing the transmitting UART to stop sending data to this UART."]
    #[inline(always)]
    pub fn rts_fifo_thresh(&self) -> RTS_FIFO_THRESH_R {
        RTS_FIFO_THRESH_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - RX FIFO Threshold Level.When the RX FIFO reaches this many bytes or higher, UARTn_INFTL.rx_fifo_level is set."]
    #[inline(always)]
    pub fn rx_fifo_thresh(&mut self) -> RX_FIFO_THRESH_W {
        RX_FIFO_THRESH_W { w: self }
    }
    #[doc = "Bits 8:13 - TX FIFO Threshold Level. When the TX FIFO reaches this many bytes or higher, UARTn_INTFL.tx_fifo_level is set."]
    #[inline(always)]
    pub fn tx_fifo_thresh(&mut self) -> TX_FIFO_THRESH_W {
        TX_FIFO_THRESH_W { w: self }
    }
    #[doc = "Bits 16:21 - RTS threshold control. When the RX FIFO reaches this many bytes or higher, the RTS output signal is deasserted, informing the transmitting UART to stop sending data to this UART."]
    #[inline(always)]
    pub fn rts_fifo_thresh(&mut self) -> RTS_FIFO_THRESH_W {
        RTS_FIFO_THRESH_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Threshold Control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [thresh_ctrl](index.html) module"]
pub struct THRESH_CTRL_SPEC;
impl crate::RegisterSpec for THRESH_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [thresh_ctrl::R](R) reader structure"]
impl crate::Readable for THRESH_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [thresh_ctrl::W](W) writer structure"]
impl crate::Writable for THRESH_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets THRESH_CTRL to value 0"]
impl crate::Resettable for THRESH_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
