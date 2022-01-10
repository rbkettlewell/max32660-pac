#[doc = "Register `TX_CTRL1` reader"]
pub struct R(crate::R<TX_CTRL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TX_CTRL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TX_CTRL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TX_CTRL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TX_CTRL1` writer"]
pub struct W(crate::W<TX_CTRL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TX_CTRL1_SPEC>;
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
impl From<crate::W<TX_CTRL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TX_CTRL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX_READY` reader - Transmit FIFO Preload Ready."]
pub struct TX_READY_R(crate::FieldReader<bool, bool>);
impl TX_READY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_READY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_READY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_READY` writer - Transmit FIFO Preload Ready."]
pub struct TX_READY_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_READY_W<'a> {
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
#[doc = "Transmit Last. This bit is used in slave mod only. Do not use when preloading (cleared by hardware).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_LAST_A {
    #[doc = "0: Hold SCL low on TX_FIFO empty."]
    HOLD_SCL_LOW = 0,
    #[doc = "1: End transaction on TX_FIFO empty."]
    END_TRANSACTION = 1,
}
impl From<TX_LAST_A> for bool {
    #[inline(always)]
    fn from(variant: TX_LAST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TX_LAST` reader - Transmit Last. This bit is used in slave mod only. Do not use when preloading (cleared by hardware)."]
pub struct TX_LAST_R(crate::FieldReader<bool, TX_LAST_A>);
impl TX_LAST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_LAST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_LAST_A {
        match self.bits {
            false => TX_LAST_A::HOLD_SCL_LOW,
            true => TX_LAST_A::END_TRANSACTION,
        }
    }
    #[doc = "Checks if the value of the field is `HOLD_SCL_LOW`"]
    #[inline(always)]
    pub fn is_hold_scl_low(&self) -> bool {
        **self == TX_LAST_A::HOLD_SCL_LOW
    }
    #[doc = "Checks if the value of the field is `END_TRANSACTION`"]
    #[inline(always)]
    pub fn is_end_transaction(&self) -> bool {
        **self == TX_LAST_A::END_TRANSACTION
    }
}
impl core::ops::Deref for TX_LAST_R {
    type Target = crate::FieldReader<bool, TX_LAST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_LAST` writer - Transmit Last. This bit is used in slave mod only. Do not use when preloading (cleared by hardware)."]
pub struct TX_LAST_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_LAST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TX_LAST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Hold SCL low on TX_FIFO empty."]
    #[inline(always)]
    pub fn hold_scl_low(self) -> &'a mut W {
        self.variant(TX_LAST_A::HOLD_SCL_LOW)
    }
    #[doc = "End transaction on TX_FIFO empty."]
    #[inline(always)]
    pub fn end_transaction(self) -> &'a mut W {
        self.variant(TX_LAST_A::END_TRANSACTION)
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
#[doc = "Field `TX_FIFO` reader - Transmit FIFO Count. These bits reflect the number of bytes in the TX_FIFO."]
pub struct TX_FIFO_R(crate::FieldReader<u8, u8>);
impl TX_FIFO_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TX_FIFO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_FIFO_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Transmit FIFO Preload Ready."]
    #[inline(always)]
    pub fn tx_ready(&self) -> TX_READY_R {
        TX_READY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transmit Last. This bit is used in slave mod only. Do not use when preloading (cleared by hardware)."]
    #[inline(always)]
    pub fn tx_last(&self) -> TX_LAST_R {
        TX_LAST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - Transmit FIFO Count. These bits reflect the number of bytes in the TX_FIFO."]
    #[inline(always)]
    pub fn tx_fifo(&self) -> TX_FIFO_R {
        TX_FIFO_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit FIFO Preload Ready."]
    #[inline(always)]
    pub fn tx_ready(&mut self) -> TX_READY_W {
        TX_READY_W { w: self }
    }
    #[doc = "Bit 1 - Transmit Last. This bit is used in slave mod only. Do not use when preloading (cleared by hardware)."]
    #[inline(always)]
    pub fn tx_last(&mut self) -> TX_LAST_W {
        TX_LAST_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmit Control Register 1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_ctrl1](index.html) module"]
pub struct TX_CTRL1_SPEC;
impl crate::RegisterSpec for TX_CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tx_ctrl1::R](R) reader structure"]
impl crate::Readable for TX_CTRL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tx_ctrl1::W](W) writer structure"]
impl crate::Writable for TX_CTRL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TX_CTRL1 to value 0"]
impl crate::Resettable for TX_CTRL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
