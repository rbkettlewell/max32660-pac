#[doc = "Register `INT_EN_SET` reader"]
pub struct R(crate::R<INT_EN_SET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_EN_SET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_EN_SET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_EN_SET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INT_EN_SET` writer"]
pub struct W(crate::W<INT_EN_SET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_EN_SET_SPEC>;
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
impl From<crate::W<INT_EN_SET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_EN_SET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Mask of all of the pins on the port.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum GPIO_INT_EN_SET_A {
    #[doc = "0: No effect."]
    NO = 0,
    #[doc = "1: Set GPIO_INT_EN bit in this position to '1'"]
    SET = 1,
}
impl From<GPIO_INT_EN_SET_A> for u32 {
    #[inline(always)]
    fn from(variant: GPIO_INT_EN_SET_A) -> Self {
        variant as _
    }
}
#[doc = "Field `GPIO_INT_EN_SET` reader - Mask of all of the pins on the port."]
pub struct GPIO_INT_EN_SET_R(crate::FieldReader<u32, GPIO_INT_EN_SET_A>);
impl GPIO_INT_EN_SET_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        GPIO_INT_EN_SET_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GPIO_INT_EN_SET_A> {
        match self.bits {
            0 => Some(GPIO_INT_EN_SET_A::NO),
            1 => Some(GPIO_INT_EN_SET_A::SET),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        **self == GPIO_INT_EN_SET_A::NO
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        **self == GPIO_INT_EN_SET_A::SET
    }
}
impl core::ops::Deref for GPIO_INT_EN_SET_R {
    type Target = crate::FieldReader<u32, GPIO_INT_EN_SET_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO_INT_EN_SET` writer - Mask of all of the pins on the port."]
pub struct GPIO_INT_EN_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_INT_EN_SET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO_INT_EN_SET_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no(self) -> &'a mut W {
        self.variant(GPIO_INT_EN_SET_A::NO)
    }
    #[doc = "Set GPIO_INT_EN bit in this position to '1'"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(GPIO_INT_EN_SET_A::SET)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    pub fn gpio_int_en_set(&self) -> GPIO_INT_EN_SET_R {
        GPIO_INT_EN_SET_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    pub fn gpio_int_en_set(&mut self) -> GPIO_INT_EN_SET_W {
        GPIO_INT_EN_SET_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO Interrupt Enable Set. Writing a 1 to one or more bits in this register sets the bits in the same positions in GPIO_INT_EN to 1, without affecting other bits in that register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_en_set](index.html) module"]
pub struct INT_EN_SET_SPEC;
impl crate::RegisterSpec for INT_EN_SET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_en_set::R](R) reader structure"]
impl crate::Readable for INT_EN_SET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [int_en_set::W](W) writer structure"]
impl crate::Writable for INT_EN_SET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INT_EN_SET to value 0"]
impl crate::Resettable for INT_EN_SET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
