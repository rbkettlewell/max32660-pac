#[doc = "Register `INT_STAT` reader"]
pub struct R(crate::R<INT_STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Mask of all of the pins on the port.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum GPIO_INT_STAT_A {
    #[doc = "0: No Interrupt is pending on this GPIO pin."]
    NO = 0,
    #[doc = "1: An Interrupt is pending on this GPIO pin."]
    PENDING = 1,
}
impl From<GPIO_INT_STAT_A> for u32 {
    #[inline(always)]
    fn from(variant: GPIO_INT_STAT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `GPIO_INT_STAT` reader - Mask of all of the pins on the port."]
pub struct GPIO_INT_STAT_R(crate::FieldReader<u32, GPIO_INT_STAT_A>);
impl GPIO_INT_STAT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        GPIO_INT_STAT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GPIO_INT_STAT_A> {
        match self.bits {
            0 => Some(GPIO_INT_STAT_A::NO),
            1 => Some(GPIO_INT_STAT_A::PENDING),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        **self == GPIO_INT_STAT_A::NO
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        **self == GPIO_INT_STAT_A::PENDING
    }
}
impl core::ops::Deref for GPIO_INT_STAT_R {
    type Target = crate::FieldReader<u32, GPIO_INT_STAT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    pub fn gpio_int_stat(&self) -> GPIO_INT_STAT_R {
        GPIO_INT_STAT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "GPIO Interrupt Status Register. Each bit in this register contains the pending interrupt status for the associated GPIO pin in this port.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_stat](index.html) module"]
pub struct INT_STAT_SPEC;
impl crate::RegisterSpec for INT_STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_stat::R](R) reader structure"]
impl crate::Readable for INT_STAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INT_STAT to value 0"]
impl crate::Resettable for INT_STAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
