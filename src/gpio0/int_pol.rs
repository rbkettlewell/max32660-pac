#[doc = "Register `INT_POL` reader"]
pub struct R(crate::R<INT_POL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_POL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_POL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_POL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INT_POL` writer"]
pub struct W(crate::W<INT_POL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_POL_SPEC>;
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
impl From<crate::W<INT_POL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_POL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Mask of all of the pins on the port.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum GPIO_INT_POL_A {
    #[doc = "0: Interrupts are latched on a falling edge or low level condition for this pin."]
    FALLING = 0,
    #[doc = "1: Interrupts are latched on a rising edge or high condition for this pin."]
    RISING = 1,
}
impl From<GPIO_INT_POL_A> for u32 {
    #[inline(always)]
    fn from(variant: GPIO_INT_POL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `GPIO_INT_POL` reader - Mask of all of the pins on the port."]
pub struct GPIO_INT_POL_R(crate::FieldReader<u32, GPIO_INT_POL_A>);
impl GPIO_INT_POL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        GPIO_INT_POL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GPIO_INT_POL_A> {
        match self.bits {
            0 => Some(GPIO_INT_POL_A::FALLING),
            1 => Some(GPIO_INT_POL_A::RISING),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        **self == GPIO_INT_POL_A::FALLING
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        **self == GPIO_INT_POL_A::RISING
    }
}
impl core::ops::Deref for GPIO_INT_POL_R {
    type Target = crate::FieldReader<u32, GPIO_INT_POL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO_INT_POL` writer - Mask of all of the pins on the port."]
pub struct GPIO_INT_POL_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_INT_POL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO_INT_POL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Interrupts are latched on a falling edge or low level condition for this pin."]
    #[inline(always)]
    pub fn falling(self) -> &'a mut W {
        self.variant(GPIO_INT_POL_A::FALLING)
    }
    #[doc = "Interrupts are latched on a rising edge or high condition for this pin."]
    #[inline(always)]
    pub fn rising(self) -> &'a mut W {
        self.variant(GPIO_INT_POL_A::RISING)
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
    pub fn gpio_int_pol(&self) -> GPIO_INT_POL_R {
        GPIO_INT_POL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    pub fn gpio_int_pol(&mut self) -> GPIO_INT_POL_W {
        GPIO_INT_POL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO Interrupt Polarity Register. Each bit in this register controls the interrupt polarity setting for one GPIO pin in the associated port.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_pol](index.html) module"]
pub struct INT_POL_SPEC;
impl crate::RegisterSpec for INT_POL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_pol::R](R) reader structure"]
impl crate::Readable for INT_POL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [int_pol::W](W) writer structure"]
impl crate::Writable for INT_POL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INT_POL to value 0"]
impl crate::Resettable for INT_POL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
