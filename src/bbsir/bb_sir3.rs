#[doc = "Register `BB_SIR3` reader"]
pub struct R(crate::R<BB_SIR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BB_SIR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BB_SIR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BB_SIR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "System Init. Configuration Register 3.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bb_sir3](index.html) module"]
pub struct BB_SIR3_SPEC;
impl crate::RegisterSpec for BB_SIR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bb_sir3::R](R) reader structure"]
impl crate::Readable for BB_SIR3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BB_SIR3 to value 0"]
impl crate::Resettable for BB_SIR3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
