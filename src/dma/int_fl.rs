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
#[doc = "Channel Interrupt. To clear an interrupt, all active interrupt bits of the DMA_ST must be cleared. The interrupt bits are set only if their corresponding interrupt enable bits are set in DMA_CN.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum IPEND_A {
    #[doc = "0: No interrupt is pending."]
    INACTIVE = 0,
    #[doc = "1: An interrupt is pending."]
    PENDING = 1,
}
impl From<IPEND_A> for u8 {
    #[inline(always)]
    fn from(variant: IPEND_A) -> Self {
        variant as _
    }
}
#[doc = "Field `IPEND` reader - Channel Interrupt. To clear an interrupt, all active interrupt bits of the DMA_ST must be cleared. The interrupt bits are set only if their corresponding interrupt enable bits are set in DMA_CN."]
pub struct IPEND_R(crate::FieldReader<u8, IPEND_A>);
impl IPEND_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        IPEND_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<IPEND_A> {
        match self.bits {
            0 => Some(IPEND_A::INACTIVE),
            1 => Some(IPEND_A::PENDING),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        **self == IPEND_A::INACTIVE
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        **self == IPEND_A::PENDING
    }
}
impl core::ops::Deref for IPEND_R {
    type Target = crate::FieldReader<u8, IPEND_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:3 - Channel Interrupt. To clear an interrupt, all active interrupt bits of the DMA_ST must be cleared. The interrupt bits are set only if their corresponding interrupt enable bits are set in DMA_CN."]
    #[inline(always)]
    pub fn ipend(&self) -> IPEND_R {
        IPEND_R::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "DMA Interrupt Register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_fl](index.html) module"]
pub struct INT_FL_SPEC;
impl crate::RegisterSpec for INT_FL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_fl::R](R) reader structure"]
impl crate::Readable for INT_FL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INT_FL to value 0"]
impl crate::Resettable for INT_FL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
