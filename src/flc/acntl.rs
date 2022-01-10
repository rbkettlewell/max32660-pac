#[doc = "Register `ACNTL` writer"]
pub struct W(crate::W<ACNTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ACNTL_SPEC>;
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
impl From<crate::W<ACNTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ACNTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ACNTL` writer - Access control."]
pub struct ACNTL_W<'a> {
    w: &'a mut W,
}
impl<'a> ACNTL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - Access control."]
    #[inline(always)]
    pub fn acntl(&mut self) -> ACNTL_W {
        ACNTL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Access Control Register. Writing the ACNTL register with the following values in the order shown, allows read and write access to the system and user Information block: pflc-acntl = 0x3a7f5ca3; pflc-acntl = 0xa1e34f20; pflc-acntl = 0x9608b2c1. When unlocked, a write of any word will disable access to system and user information block. Readback of this register is always zero.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acntl](index.html) module"]
pub struct ACNTL_SPEC;
impl crate::RegisterSpec for ACNTL_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [acntl::W](W) writer structure"]
impl crate::Writable for ACNTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ACNTL to value 0"]
impl crate::Resettable for ACNTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
