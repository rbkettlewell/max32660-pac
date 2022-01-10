#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Function Control Register 0."]
    pub bbfcr0: crate::Reg<bbfcr0::BBFCR0_SPEC>,
}
#[doc = "BBFCR0 register accessor: an alias for `Reg<BBFCR0_SPEC>`"]
pub type BBFCR0 = crate::Reg<bbfcr0::BBFCR0_SPEC>;
#[doc = "Function Control Register 0."]
pub mod bbfcr0;
