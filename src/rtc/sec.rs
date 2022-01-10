#[doc = "Register `SEC` reader"]
pub struct R(crate::R<SEC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEC` writer"]
pub struct W(crate::W<SEC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEC_SPEC>;
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
impl From<crate::W<SEC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEC_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC Second Counter. This register contains the 32-bit second counter.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sec](index.html) module"]
pub struct SEC_SPEC;
impl crate::RegisterSpec for SEC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sec::R](R) reader structure"]
impl crate::Readable for SEC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sec::W](W) writer structure"]
impl crate::Writable for SEC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SEC to value 0"]
impl crate::Resettable for SEC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
