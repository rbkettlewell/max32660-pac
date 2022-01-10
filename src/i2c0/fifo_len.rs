#[doc = "Register `FIFO_LEN` reader"]
pub struct R(crate::R<FIFO_LEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FIFO_LEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FIFO_LEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FIFO_LEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RX_LEN` reader - Receive FIFO Length."]
pub struct RX_LEN_R(crate::FieldReader<u8, u8>);
impl RX_LEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RX_LEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_LEN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_LEN` reader - Transmit FIFO Length."]
pub struct TX_LEN_R(crate::FieldReader<u8, u8>);
impl TX_LEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TX_LEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_LEN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - Receive FIFO Length."]
    #[inline(always)]
    pub fn rx_len(&self) -> RX_LEN_R {
        RX_LEN_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Transmit FIFO Length."]
    #[inline(always)]
    pub fn tx_len(&self) -> TX_LEN_R {
        TX_LEN_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[doc = "FIFO Configuration Register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifo_len](index.html) module"]
pub struct FIFO_LEN_SPEC;
impl crate::RegisterSpec for FIFO_LEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fifo_len::R](R) reader structure"]
impl crate::Readable for FIFO_LEN_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FIFO_LEN to value 0"]
impl crate::Resettable for FIFO_LEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
