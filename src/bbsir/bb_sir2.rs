#[doc = "Register `BB_SIR2` reader"]
pub struct R(crate::R<BB_SIR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BB_SIR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BB_SIR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BB_SIR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "System Init. Configuration Register 2.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bb_sir2](index.html) module"]
pub struct BB_SIR2_SPEC;
impl crate::RegisterSpec for BB_SIR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bb_sir2::R](R) reader structure"]
impl crate::Readable for BB_SIR2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BB_SIR2 to value 0"]
impl crate::Resettable for BB_SIR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
