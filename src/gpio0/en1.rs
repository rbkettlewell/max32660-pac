#[doc = "Register `EN1` reader"]
pub struct R(crate::R<EN1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EN1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EN1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EN1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EN1` writer"]
pub struct W(crate::W<EN1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EN1_SPEC>;
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
impl From<crate::W<EN1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EN1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Mask of all of the pins on the port.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum GPIO_EN1_A {
    #[doc = "0: Primary function selected."]
    PRIMARY = 0,
    #[doc = "1: Secondary function selected."]
    SECONDARY = 1,
}
impl From<GPIO_EN1_A> for u32 {
    #[inline(always)]
    fn from(variant: GPIO_EN1_A) -> Self {
        variant as _
    }
}
#[doc = "Field `GPIO_EN1` reader - Mask of all of the pins on the port."]
pub struct GPIO_EN1_R(crate::FieldReader<u32, GPIO_EN1_A>);
impl GPIO_EN1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        GPIO_EN1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GPIO_EN1_A> {
        match self.bits {
            0 => Some(GPIO_EN1_A::PRIMARY),
            1 => Some(GPIO_EN1_A::SECONDARY),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PRIMARY`"]
    #[inline(always)]
    pub fn is_primary(&self) -> bool {
        **self == GPIO_EN1_A::PRIMARY
    }
    #[doc = "Checks if the value of the field is `SECONDARY`"]
    #[inline(always)]
    pub fn is_secondary(&self) -> bool {
        **self == GPIO_EN1_A::SECONDARY
    }
}
impl core::ops::Deref for GPIO_EN1_R {
    type Target = crate::FieldReader<u32, GPIO_EN1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO_EN1` writer - Mask of all of the pins on the port."]
pub struct GPIO_EN1_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_EN1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO_EN1_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Primary function selected."]
    #[inline(always)]
    pub fn primary(self) -> &'a mut W {
        self.variant(GPIO_EN1_A::PRIMARY)
    }
    #[doc = "Secondary function selected."]
    #[inline(always)]
    pub fn secondary(self) -> &'a mut W {
        self.variant(GPIO_EN1_A::SECONDARY)
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
    pub fn gpio_en1(&self) -> GPIO_EN1_R {
        GPIO_EN1_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    pub fn gpio_en1(&mut self) -> GPIO_EN1_W {
        GPIO_EN1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO Alternate Function Enable Register. Each bit in this register selects between primary/secondary functions for the associated GPIO pin in this port.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [en1](index.html) module"]
pub struct EN1_SPEC;
impl crate::RegisterSpec for EN1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [en1::R](R) reader structure"]
impl crate::Readable for EN1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [en1::W](W) writer structure"]
impl crate::Writable for EN1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EN1 to value 0"]
impl crate::Resettable for EN1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
