#[doc = "Register `INTR` reader"]
pub struct R(crate::R<INTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Channel Interrupt. To clear an interrupt, all active interrupt bits of the DMA_ST must be cleared. The interrupt bits are set only if their corresponding interrupt enable bits are set in DMA_CN.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH0_IPEND_A {
    #[doc = "0: No interrupt is pending."]
    INACTIVE = 0,
    #[doc = "1: An interrupt is pending."]
    PENDING = 1,
}
impl From<CH0_IPEND_A> for bool {
    #[inline(always)]
    fn from(variant: CH0_IPEND_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH0_IPEND` reader - Channel Interrupt. To clear an interrupt, all active interrupt bits of the DMA_ST must be cleared. The interrupt bits are set only if their corresponding interrupt enable bits are set in DMA_CN."]
pub struct CH0_IPEND_R(crate::FieldReader<bool, CH0_IPEND_A>);
impl CH0_IPEND_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH0_IPEND_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH0_IPEND_A {
        match self.bits {
            false => CH0_IPEND_A::INACTIVE,
            true => CH0_IPEND_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        **self == CH0_IPEND_A::INACTIVE
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        **self == CH0_IPEND_A::PENDING
    }
}
impl core::ops::Deref for CH0_IPEND_R {
    type Target = crate::FieldReader<bool, CH0_IPEND_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH1_IPEND` reader - "]
pub struct CH1_IPEND_R(crate::FieldReader<bool, bool>);
impl CH1_IPEND_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH1_IPEND_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH1_IPEND_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH2_IPEND` reader - "]
pub struct CH2_IPEND_R(crate::FieldReader<bool, bool>);
impl CH2_IPEND_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH2_IPEND_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH2_IPEND_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH3_IPEND` reader - "]
pub struct CH3_IPEND_R(crate::FieldReader<bool, bool>);
impl CH3_IPEND_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH3_IPEND_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH3_IPEND_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Channel Interrupt. To clear an interrupt, all active interrupt bits of the DMA_ST must be cleared. The interrupt bits are set only if their corresponding interrupt enable bits are set in DMA_CN."]
    #[inline(always)]
    pub fn ch0_ipend(&self) -> CH0_IPEND_R {
        CH0_IPEND_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ch1_ipend(&self) -> CH1_IPEND_R {
        CH1_IPEND_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ch2_ipend(&self) -> CH2_IPEND_R {
        CH2_IPEND_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ch3_ipend(&self) -> CH3_IPEND_R {
        CH3_IPEND_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
#[doc = "DMA Interrupt Register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr](index.html) module"]
pub struct INTR_SPEC;
impl crate::RegisterSpec for INTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr::R](R) reader structure"]
impl crate::Readable for INTR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INTR to value 0"]
impl crate::Resettable for INTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
