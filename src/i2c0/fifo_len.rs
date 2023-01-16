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
#[doc = "Register `FIFO_LEN` writer"]
pub struct W(crate::W<FIFO_LEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FIFO_LEN_SPEC>;
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
impl From<crate::W<FIFO_LEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FIFO_LEN_SPEC>) -> Self {
        W(writer)
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
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FIFO Configuration Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifo_len](index.html) module"]
pub struct FIFO_LEN_SPEC;
impl crate::RegisterSpec for FIFO_LEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fifo_len::R](R) reader structure"]
impl crate::Readable for FIFO_LEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fifo_len::W](W) writer structure"]
impl crate::Writable for FIFO_LEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FIFO_LEN to value 0"]
impl crate::Resettable for FIFO_LEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
