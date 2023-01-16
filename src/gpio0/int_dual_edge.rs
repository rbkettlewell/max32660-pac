#[doc = "Register `INT_DUAL_EDGE` reader"]
pub struct R(crate::R<INT_DUAL_EDGE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_DUAL_EDGE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_DUAL_EDGE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_DUAL_EDGE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INT_DUAL_EDGE` writer"]
pub struct W(crate::W<INT_DUAL_EDGE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_DUAL_EDGE_SPEC>;
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
impl From<crate::W<INT_DUAL_EDGE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_DUAL_EDGE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Mask of all of the pins on the port.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum GPIO_INT_DUAL_EDGE_A {
    #[doc = "0: No Effect."]
    NO = 0,
    #[doc = "1: Dual Edge mode is enabled. If edge-triggered interrupts are enabled on this GPIO pin, then both rising and falling edges will trigger interrupts regardless of the GPIO_INT_POL setting."]
    EN = 1,
}
impl From<GPIO_INT_DUAL_EDGE_A> for u32 {
    #[inline(always)]
    fn from(variant: GPIO_INT_DUAL_EDGE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `GPIO_INT_DUAL_EDGE` reader - Mask of all of the pins on the port."]
pub struct GPIO_INT_DUAL_EDGE_R(crate::FieldReader<u32, GPIO_INT_DUAL_EDGE_A>);
impl GPIO_INT_DUAL_EDGE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        GPIO_INT_DUAL_EDGE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GPIO_INT_DUAL_EDGE_A> {
        match self.bits {
            0 => Some(GPIO_INT_DUAL_EDGE_A::NO),
            1 => Some(GPIO_INT_DUAL_EDGE_A::EN),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        **self == GPIO_INT_DUAL_EDGE_A::NO
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == GPIO_INT_DUAL_EDGE_A::EN
    }
}
impl core::ops::Deref for GPIO_INT_DUAL_EDGE_R {
    type Target = crate::FieldReader<u32, GPIO_INT_DUAL_EDGE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO_INT_DUAL_EDGE` writer - Mask of all of the pins on the port."]
pub struct GPIO_INT_DUAL_EDGE_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_INT_DUAL_EDGE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO_INT_DUAL_EDGE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No Effect."]
    #[inline(always)]
    pub fn no(self) -> &'a mut W {
        self.variant(GPIO_INT_DUAL_EDGE_A::NO)
    }
    #[doc = "Dual Edge mode is enabled. If edge-triggered interrupts are enabled on this GPIO pin, then both rising and falling edges will trigger interrupts regardless of the GPIO_INT_POL setting."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(GPIO_INT_DUAL_EDGE_A::EN)
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
    pub fn gpio_int_dual_edge(&self) -> GPIO_INT_DUAL_EDGE_R {
        GPIO_INT_DUAL_EDGE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    pub fn gpio_int_dual_edge(&mut self) -> GPIO_INT_DUAL_EDGE_W {
        GPIO_INT_DUAL_EDGE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO Interrupt Dual Edge Mode Register. Each bit in this register selects dual edge mode for the associated GPIO pin in this port.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_dual_edge](index.html) module"]
pub struct INT_DUAL_EDGE_SPEC;
impl crate::RegisterSpec for INT_DUAL_EDGE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_dual_edge::R](R) reader structure"]
impl crate::Readable for INT_DUAL_EDGE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [int_dual_edge::W](W) writer structure"]
impl crate::Writable for INT_DUAL_EDGE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INT_DUAL_EDGE to value 0"]
impl crate::Resettable for INT_DUAL_EDGE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
