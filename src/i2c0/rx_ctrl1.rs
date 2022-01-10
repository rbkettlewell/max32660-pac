#[doc = "Register `RX_CTRL1` reader"]
pub struct R(crate::R<RX_CTRL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_CTRL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_CTRL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_CTRL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RX_CTRL1` writer"]
pub struct W(crate::W<RX_CTRL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RX_CTRL1_SPEC>;
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
impl From<crate::W<RX_CTRL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RX_CTRL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RX_CNT` reader - Receive Count Bits. These bits define the number of bytes to be received in a transaction, except for the case RXCNT = 0. RXCNT = 0 means 256 bytes to be received in a transaction."]
pub struct RX_CNT_R(crate::FieldReader<u8, u8>);
impl RX_CNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RX_CNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_CNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_CNT` writer - Receive Count Bits. These bits define the number of bytes to be received in a transaction, except for the case RXCNT = 0. RXCNT = 0 means 256 bytes to be received in a transaction."]
pub struct RX_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `RX_FIFO` reader - Receive FIFO Count. These bits reflect the number of byte in the RX_FIFO. These bits are flushed when I2CEN = 0."]
pub struct RX_FIFO_R(crate::FieldReader<u8, u8>);
impl RX_FIFO_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RX_FIFO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_FIFO_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - Receive Count Bits. These bits define the number of bytes to be received in a transaction, except for the case RXCNT = 0. RXCNT = 0 means 256 bytes to be received in a transaction."]
    #[inline(always)]
    pub fn rx_cnt(&self) -> RX_CNT_R {
        RX_CNT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - Receive FIFO Count. These bits reflect the number of byte in the RX_FIFO. These bits are flushed when I2CEN = 0."]
    #[inline(always)]
    pub fn rx_fifo(&self) -> RX_FIFO_R {
        RX_FIFO_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Receive Count Bits. These bits define the number of bytes to be received in a transaction, except for the case RXCNT = 0. RXCNT = 0 means 256 bytes to be received in a transaction."]
    #[inline(always)]
    pub fn rx_cnt(&mut self) -> RX_CNT_W {
        RX_CNT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Receive Control Register 1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_ctrl1](index.html) module"]
pub struct RX_CTRL1_SPEC;
impl crate::RegisterSpec for RX_CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rx_ctrl1::R](R) reader structure"]
impl crate::Readable for RX_CTRL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rx_ctrl1::W](W) writer structure"]
impl crate::Writable for RX_CTRL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RX_CTRL1 to value 0"]
impl crate::Resettable for RX_CTRL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
