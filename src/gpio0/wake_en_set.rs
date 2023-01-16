#[doc = "Register `WAKE_EN_SET` reader"]
pub struct R(crate::R<WAKE_EN_SET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WAKE_EN_SET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WAKE_EN_SET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WAKE_EN_SET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WAKE_EN_SET` writer"]
pub struct W(crate::W<WAKE_EN_SET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WAKE_EN_SET_SPEC>;
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
impl From<crate::W<WAKE_EN_SET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WAKE_EN_SET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ALL` reader - Mask of all of the pins on the port."]
pub struct ALL_R(crate::FieldReader<u32, u32>);
impl ALL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        ALL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALL_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALL` writer - Mask of all of the pins on the port."]
pub struct ALL_W<'a> {
    w: &'a mut W,
}
impl<'a> ALL_W<'a> {
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
    pub fn all(&self) -> ALL_R {
        ALL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    pub fn all(&mut self) -> ALL_W {
        ALL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO Wake Enable Set. Writing a 1 to one or more bits in this register sets the bits in the same positions in GPIO_WAKE_EN to 1, without affecting other bits in that register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wake_en_set](index.html) module"]
pub struct WAKE_EN_SET_SPEC;
impl crate::RegisterSpec for WAKE_EN_SET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wake_en_set::R](R) reader structure"]
impl crate::Readable for WAKE_EN_SET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wake_en_set::W](W) writer structure"]
impl crate::Writable for WAKE_EN_SET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WAKE_EN_SET to value 0"]
impl crate::Resettable for WAKE_EN_SET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
