#[doc = "Register `PAD_CFG2` reader"]
pub struct R(crate::R<PAD_CFG2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PAD_CFG2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PAD_CFG2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PAD_CFG2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PAD_CFG2` writer"]
pub struct W(crate::W<PAD_CFG2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PAD_CFG2_SPEC>;
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
impl From<crate::W<PAD_CFG2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PAD_CFG2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "The two bits in GPIO_PAD_CFG1 and GPIO_PAD_CFG2 for each GPIO pin work together to determine the pad mode when the GPIO is set to input mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum GPIO_PAD_CFG2_A {
    #[doc = "0: High Impedance."]
    IMPEDANCE = 0,
    #[doc = "1: Weak pull-up mode."]
    PU = 1,
    #[doc = "2: weak pull-down mode."]
    PD = 2,
}
impl From<GPIO_PAD_CFG2_A> for u32 {
    #[inline(always)]
    fn from(variant: GPIO_PAD_CFG2_A) -> Self {
        variant as _
    }
}
#[doc = "Field `GPIO_PAD_CFG2` reader - The two bits in GPIO_PAD_CFG1 and GPIO_PAD_CFG2 for each GPIO pin work together to determine the pad mode when the GPIO is set to input mode."]
pub struct GPIO_PAD_CFG2_R(crate::FieldReader<u32, GPIO_PAD_CFG2_A>);
impl GPIO_PAD_CFG2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        GPIO_PAD_CFG2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GPIO_PAD_CFG2_A> {
        match self.bits {
            0 => Some(GPIO_PAD_CFG2_A::IMPEDANCE),
            1 => Some(GPIO_PAD_CFG2_A::PU),
            2 => Some(GPIO_PAD_CFG2_A::PD),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `IMPEDANCE`"]
    #[inline(always)]
    pub fn is_impedance(&self) -> bool {
        **self == GPIO_PAD_CFG2_A::IMPEDANCE
    }
    #[doc = "Checks if the value of the field is `PU`"]
    #[inline(always)]
    pub fn is_pu(&self) -> bool {
        **self == GPIO_PAD_CFG2_A::PU
    }
    #[doc = "Checks if the value of the field is `PD`"]
    #[inline(always)]
    pub fn is_pd(&self) -> bool {
        **self == GPIO_PAD_CFG2_A::PD
    }
}
impl core::ops::Deref for GPIO_PAD_CFG2_R {
    type Target = crate::FieldReader<u32, GPIO_PAD_CFG2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO_PAD_CFG2` writer - The two bits in GPIO_PAD_CFG1 and GPIO_PAD_CFG2 for each GPIO pin work together to determine the pad mode when the GPIO is set to input mode."]
pub struct GPIO_PAD_CFG2_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_PAD_CFG2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO_PAD_CFG2_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "High Impedance."]
    #[inline(always)]
    pub fn impedance(self) -> &'a mut W {
        self.variant(GPIO_PAD_CFG2_A::IMPEDANCE)
    }
    #[doc = "Weak pull-up mode."]
    #[inline(always)]
    pub fn pu(self) -> &'a mut W {
        self.variant(GPIO_PAD_CFG2_A::PU)
    }
    #[doc = "weak pull-down mode."]
    #[inline(always)]
    pub fn pd(self) -> &'a mut W {
        self.variant(GPIO_PAD_CFG2_A::PD)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - The two bits in GPIO_PAD_CFG1 and GPIO_PAD_CFG2 for each GPIO pin work together to determine the pad mode when the GPIO is set to input mode."]
    #[inline(always)]
    pub fn gpio_pad_cfg2(&self) -> GPIO_PAD_CFG2_R {
        GPIO_PAD_CFG2_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - The two bits in GPIO_PAD_CFG1 and GPIO_PAD_CFG2 for each GPIO pin work together to determine the pad mode when the GPIO is set to input mode."]
    #[inline(always)]
    pub fn gpio_pad_cfg2(&mut self) -> GPIO_PAD_CFG2_W {
        GPIO_PAD_CFG2_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO Input Mode Config 2. Each bit in this register enables the weak pull-up for the associated GPIO pin in this port.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pad_cfg2](index.html) module"]
pub struct PAD_CFG2_SPEC;
impl crate::RegisterSpec for PAD_CFG2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pad_cfg2::R](R) reader structure"]
impl crate::Readable for PAD_CFG2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pad_cfg2::W](W) writer structure"]
impl crate::Writable for PAD_CFG2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PAD_CFG2 to value 0"]
impl crate::Resettable for PAD_CFG2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
