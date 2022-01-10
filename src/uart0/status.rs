#[doc = "Register `STATUS` reader"]
pub struct R(crate::R<STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TX_BUSY` reader - Read-only flag indicating the UART transmit status."]
pub struct TX_BUSY_R(crate::FieldReader<bool, bool>);
impl TX_BUSY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_BUSY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_BUSY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_BUSY` reader - Read-only flag indicating the UARTreceiver status."]
pub struct RX_BUSY_R(crate::FieldReader<bool, bool>);
impl RX_BUSY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_BUSY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_BUSY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PARITY` reader - 9th Received bit state. This bit identifies the state of the 9th bit of received data. Only available for UART_CTRL.SIZE\\[1:0\\]=3."]
pub struct PARITY_R(crate::FieldReader<bool, bool>);
impl PARITY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PARITY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PARITY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BREAK` reader - Received BREAK status. BREAKS is cleared when UART_STAT register is read. Received data input is held in spacing (logic 0) state for longer than a full word transmission time (that is, the total time of Start bit + data bits + Parity + Stop bits)."]
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
#[doc = "Field `RX_EMPTY` reader - Read-only flag indicating the RX FIFO state."]
pub struct RX_EMPTY_R(crate::FieldReader<bool, bool>);
impl RX_EMPTY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_EMPTY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_EMPTY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_FULL` reader - Read-only flag indicating the RX FIFO state."]
pub struct RX_FULL_R(crate::FieldReader<bool, bool>);
impl RX_FULL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_FULL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_FULL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_EMPTY` reader - Read-only flag indicating the TX FIFO state."]
pub struct TX_EMPTY_R(crate::FieldReader<bool, bool>);
impl TX_EMPTY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_EMPTY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_EMPTY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_FULL` reader - Read-only flag indicating the TX FIFO state."]
pub struct TX_FULL_R(crate::FieldReader<bool, bool>);
impl TX_FULL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_FULL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_FULL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_FIFO_CNT` reader - Indicates the number of bytes currently in the RX FIFO."]
pub struct RX_FIFO_CNT_R(crate::FieldReader<u8, u8>);
impl RX_FIFO_CNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RX_FIFO_CNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_FIFO_CNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_FIFO_CNT` reader - Indicates the number of bytes currently in the TX FIFO."]
pub struct TX_FIFO_CNT_R(crate::FieldReader<u8, u8>);
impl TX_FIFO_CNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TX_FIFO_CNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_FIFO_CNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_TO` reader - RX Timeout status."]
pub struct RX_TO_R(crate::FieldReader<bool, bool>);
impl RX_TO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_TO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_TO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Read-only flag indicating the UART transmit status."]
    #[inline(always)]
    pub fn tx_busy(&self) -> TX_BUSY_R {
        TX_BUSY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Read-only flag indicating the UARTreceiver status."]
    #[inline(always)]
    pub fn rx_busy(&self) -> RX_BUSY_R {
        RX_BUSY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 9th Received bit state. This bit identifies the state of the 9th bit of received data. Only available for UART_CTRL.SIZE\\[1:0\\]=3."]
    #[inline(always)]
    pub fn parity(&self) -> PARITY_R {
        PARITY_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Received BREAK status. BREAKS is cleared when UART_STAT register is read. Received data input is held in spacing (logic 0) state for longer than a full word transmission time (that is, the total time of Start bit + data bits + Parity + Stop bits)."]
    #[inline(always)]
    pub fn break_(&self) -> BREAK_R {
        BREAK_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Read-only flag indicating the RX FIFO state."]
    #[inline(always)]
    pub fn rx_empty(&self) -> RX_EMPTY_R {
        RX_EMPTY_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Read-only flag indicating the RX FIFO state."]
    #[inline(always)]
    pub fn rx_full(&self) -> RX_FULL_R {
        RX_FULL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Read-only flag indicating the TX FIFO state."]
    #[inline(always)]
    pub fn tx_empty(&self) -> TX_EMPTY_R {
        TX_EMPTY_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Read-only flag indicating the TX FIFO state."]
    #[inline(always)]
    pub fn tx_full(&self) -> TX_FULL_R {
        TX_FULL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:13 - Indicates the number of bytes currently in the RX FIFO."]
    #[inline(always)]
    pub fn rx_fifo_cnt(&self) -> RX_FIFO_CNT_R {
        RX_FIFO_CNT_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - Indicates the number of bytes currently in the TX FIFO."]
    #[inline(always)]
    pub fn tx_fifo_cnt(&self) -> TX_FIFO_CNT_R {
        TX_FIFO_CNT_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 24 - RX Timeout status."]
    #[inline(always)]
    pub fn rx_to(&self) -> RX_TO_R {
        RX_TO_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
#[doc = "Status Register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [status::R](R) reader structure"]
impl crate::Readable for STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
