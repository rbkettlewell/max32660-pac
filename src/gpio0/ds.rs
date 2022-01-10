#[doc = "Register `DS` reader"]
pub struct R(crate::R<DS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DS` writer"]
pub struct W(crate::W<DS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DS_SPEC>;
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
impl From<crate::W<DS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Mask of all of the pins on the port.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum DS_A {
    #[doc = "0: GPIO port pin is in low-drive mode."]
    LD = 0,
    #[doc = "1: GPIO port pin is in high-drive mode."]
    HD = 1,
}
impl From<DS_A> for u32 {
    #[inline(always)]
    fn from(variant: DS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DS` reader - Mask of all of the pins on the port."]
pub struct DS_R(crate::FieldReader<u32, DS_A>);
impl DS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        DS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DS_A> {
        match self.bits {
            0 => Some(DS_A::LD),
            1 => Some(DS_A::HD),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `LD`"]
    #[inline(always)]
    pub fn is_ld(&self) -> bool {
        **self == DS_A::LD
    }
    #[doc = "Checks if the value of the field is `HD`"]
    #[inline(always)]
    pub fn is_hd(&self) -> bool {
        **self == DS_A::HD
    }
}
impl core::ops::Deref for DS_R {
    type Target = crate::FieldReader<u32, DS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DS` writer - Mask of all of the pins on the port."]
pub struct DS_W<'a> {
    w: &'a mut W,
}
impl<'a> DS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "GPIO port pin is in low-drive mode."]
    #[inline(always)]
    pub fn ld(self) -> &'a mut W {
        self.variant(DS_A::LD)
    }
    #[doc = "GPIO port pin is in high-drive mode."]
    #[inline(always)]
    pub fn hd(self) -> &'a mut W {
        self.variant(DS_A::HD)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    pub fn ds(&self) -> DS_R {
        DS_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    pub fn ds(&mut self) -> DS_W {
        DS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO Drive Strength Register. Each bit in this register selects the drive strength for the associated GPIO pin in this port. Refer to the Datasheet for sink/source current of GPIO pins in each mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ds](index.html) module"]
pub struct DS_SPEC;
impl crate::RegisterSpec for DS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ds::R](R) reader structure"]
impl crate::Readable for DS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ds::W](W) writer structure"]
impl crate::Writable for DS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DS to value 0"]
impl crate::Resettable for DS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
