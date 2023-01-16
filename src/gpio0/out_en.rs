#[doc = "Register `OUT_EN` reader"]
pub struct R(crate::R<OUT_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OUT_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OUT_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OUT_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OUT_EN` writer"]
pub struct W(crate::W<OUT_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OUT_EN_SPEC>;
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
impl From<crate::W<OUT_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OUT_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Mask of all of the pins on the port.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum GPIO_OUT_EN_A {
    #[doc = "0: GPIO Output Disable"]
    DIS = 0,
    #[doc = "1: GPIO Output Enable"]
    EN = 1,
}
impl From<GPIO_OUT_EN_A> for u32 {
    #[inline(always)]
    fn from(variant: GPIO_OUT_EN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `GPIO_OUT_EN` reader - Mask of all of the pins on the port."]
pub struct GPIO_OUT_EN_R(crate::FieldReader<u32, GPIO_OUT_EN_A>);
impl GPIO_OUT_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        GPIO_OUT_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GPIO_OUT_EN_A> {
        match self.bits {
            0 => Some(GPIO_OUT_EN_A::DIS),
            1 => Some(GPIO_OUT_EN_A::EN),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == GPIO_OUT_EN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == GPIO_OUT_EN_A::EN
    }
}
impl core::ops::Deref for GPIO_OUT_EN_R {
    type Target = crate::FieldReader<u32, GPIO_OUT_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO_OUT_EN` writer - Mask of all of the pins on the port."]
pub struct GPIO_OUT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_OUT_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO_OUT_EN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "GPIO Output Disable"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO_OUT_EN_A::DIS)
    }
    #[doc = "GPIO Output Enable"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(GPIO_OUT_EN_A::EN)
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
    pub fn gpio_out_en(&self) -> GPIO_OUT_EN_R {
        GPIO_OUT_EN_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    pub fn gpio_out_en(&mut self) -> GPIO_OUT_EN_W {
        GPIO_OUT_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO Output Enable Register. Each bit controls the GPIO_OUT_EN setting for one GPIO pin in the associated port.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [out_en](index.html) module"]
pub struct OUT_EN_SPEC;
impl crate::RegisterSpec for OUT_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [out_en::R](R) reader structure"]
impl crate::Readable for OUT_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [out_en::W](W) writer structure"]
impl crate::Writable for OUT_EN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OUT_EN to value 0"]
impl crate::Resettable for OUT_EN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
