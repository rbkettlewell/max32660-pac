#[doc = "Register `INT_EN1` reader"]
pub struct R(crate::R<INT_EN1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_EN1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_EN1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_EN1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INT_EN1` writer"]
pub struct W(crate::W<INT_EN1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_EN1_SPEC>;
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
impl From<crate::W<INT_EN1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_EN1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Receiver Overflow Interrupt Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_OVERFLOW_A {
    #[doc = "0: No Interrupt is Pending."]
    DIS = 0,
    #[doc = "1: An interrupt is pending."]
    EN = 1,
}
impl From<RX_OVERFLOW_A> for bool {
    #[inline(always)]
    fn from(variant: RX_OVERFLOW_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RX_OVERFLOW` reader - Receiver Overflow Interrupt Enable."]
pub struct RX_OVERFLOW_R(crate::FieldReader<bool, RX_OVERFLOW_A>);
impl RX_OVERFLOW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_OVERFLOW_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_OVERFLOW_A {
        match self.bits {
            false => RX_OVERFLOW_A::DIS,
            true => RX_OVERFLOW_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == RX_OVERFLOW_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == RX_OVERFLOW_A::EN
    }
}
impl core::ops::Deref for RX_OVERFLOW_R {
    type Target = crate::FieldReader<bool, RX_OVERFLOW_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_OVERFLOW` writer - Receiver Overflow Interrupt Enable."]
pub struct RX_OVERFLOW_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_OVERFLOW_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX_OVERFLOW_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No Interrupt is Pending."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(RX_OVERFLOW_A::DIS)
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(RX_OVERFLOW_A::EN)
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
#[doc = "Transmit Underflow Interrupt Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_UNDERFLOW_A {
    #[doc = "0: No Interrupt is Pending."]
    DIS = 0,
    #[doc = "1: An interrupt is pending."]
    EN = 1,
}
impl From<TX_UNDERFLOW_A> for bool {
    #[inline(always)]
    fn from(variant: TX_UNDERFLOW_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TX_UNDERFLOW` reader - Transmit Underflow Interrupt Enable."]
pub struct TX_UNDERFLOW_R(crate::FieldReader<bool, TX_UNDERFLOW_A>);
impl TX_UNDERFLOW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_UNDERFLOW_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_UNDERFLOW_A {
        match self.bits {
            false => TX_UNDERFLOW_A::DIS,
            true => TX_UNDERFLOW_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == TX_UNDERFLOW_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == TX_UNDERFLOW_A::EN
    }
}
impl core::ops::Deref for TX_UNDERFLOW_R {
    type Target = crate::FieldReader<bool, TX_UNDERFLOW_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_UNDERFLOW` writer - Transmit Underflow Interrupt Enable."]
pub struct TX_UNDERFLOW_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_UNDERFLOW_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TX_UNDERFLOW_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No Interrupt is Pending."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TX_UNDERFLOW_A::DIS)
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TX_UNDERFLOW_A::EN)
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
impl R {
    #[doc = "Bit 0 - Receiver Overflow Interrupt Enable."]
    #[inline(always)]
    pub fn rx_overflow(&self) -> RX_OVERFLOW_R {
        RX_OVERFLOW_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transmit Underflow Interrupt Enable."]
    #[inline(always)]
    pub fn tx_underflow(&self) -> TX_UNDERFLOW_R {
        TX_UNDERFLOW_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receiver Overflow Interrupt Enable."]
    #[inline(always)]
    pub fn rx_overflow(&mut self) -> RX_OVERFLOW_W {
        RX_OVERFLOW_W { w: self }
    }
    #[doc = "Bit 1 - Transmit Underflow Interrupt Enable."]
    #[inline(always)]
    pub fn tx_underflow(&mut self) -> TX_UNDERFLOW_W {
        TX_UNDERFLOW_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Staus Register 1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_en1](index.html) module"]
pub struct INT_EN1_SPEC;
impl crate::RegisterSpec for INT_EN1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_en1::R](R) reader structure"]
impl crate::Readable for INT_EN1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [int_en1::W](W) writer structure"]
impl crate::Writable for INT_EN1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INT_EN1 to value 0"]
impl crate::Resettable for INT_EN1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
