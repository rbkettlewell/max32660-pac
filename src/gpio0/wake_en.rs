#[doc = "Register `WAKE_EN` reader"]
pub struct R(crate::R<WAKE_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WAKE_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WAKE_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WAKE_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WAKE_EN` writer"]
pub struct W(crate::W<WAKE_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WAKE_EN_SPEC>;
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
impl From<crate::W<WAKE_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WAKE_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Mask of all of the pins on the port.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum GPIO_WAKE_EN_A {
    #[doc = "0: PMU wakeup for this GPIO is disabled."]
    DIS = 0,
    #[doc = "1: PMU wakeup for this GPIO is enabled."]
    EN = 1,
}
impl From<GPIO_WAKE_EN_A> for u32 {
    #[inline(always)]
    fn from(variant: GPIO_WAKE_EN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `GPIO_WAKE_EN` reader - Mask of all of the pins on the port."]
pub struct GPIO_WAKE_EN_R(crate::FieldReader<u32, GPIO_WAKE_EN_A>);
impl GPIO_WAKE_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        GPIO_WAKE_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GPIO_WAKE_EN_A> {
        match self.bits {
            0 => Some(GPIO_WAKE_EN_A::DIS),
            1 => Some(GPIO_WAKE_EN_A::EN),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == GPIO_WAKE_EN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == GPIO_WAKE_EN_A::EN
    }
}
impl core::ops::Deref for GPIO_WAKE_EN_R {
    type Target = crate::FieldReader<u32, GPIO_WAKE_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO_WAKE_EN` writer - Mask of all of the pins on the port."]
pub struct GPIO_WAKE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_WAKE_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO_WAKE_EN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "PMU wakeup for this GPIO is disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO_WAKE_EN_A::DIS)
    }
    #[doc = "PMU wakeup for this GPIO is enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(GPIO_WAKE_EN_A::EN)
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
    pub fn gpio_wake_en(&self) -> GPIO_WAKE_EN_R {
        GPIO_WAKE_EN_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    pub fn gpio_wake_en(&mut self) -> GPIO_WAKE_EN_W {
        GPIO_WAKE_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO Wake Enable Register. Each bit in this register controls the PMU wakeup enable for the associated GPIO pin in this port.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wake_en](index.html) module"]
pub struct WAKE_EN_SPEC;
impl crate::RegisterSpec for WAKE_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wake_en::R](R) reader structure"]
impl crate::Readable for WAKE_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wake_en::W](W) writer structure"]
impl crate::Writable for WAKE_EN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WAKE_EN to value 0"]
impl crate::Resettable for WAKE_EN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
