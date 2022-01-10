#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - RFU"]
    pub rsv0: crate::Reg<rsv0::RSV0_SPEC>,
    _reserved1: [u8; 0x04],
    #[doc = "0x08 - System Init. Configuration Register 2."]
    pub bb_sir2: crate::Reg<bb_sir2::BB_SIR2_SPEC>,
    #[doc = "0x0c - System Init. Configuration Register 3."]
    pub bb_sir3: crate::Reg<bb_sir3::BB_SIR3_SPEC>,
}
#[doc = "rsv0 register accessor: an alias for `Reg<RSV0_SPEC>`"]
pub type RSV0 = crate::Reg<rsv0::RSV0_SPEC>;
#[doc = "RFU"]
pub mod rsv0;
#[doc = "BB_SIR2 register accessor: an alias for `Reg<BB_SIR2_SPEC>`"]
pub type BB_SIR2 = crate::Reg<bb_sir2::BB_SIR2_SPEC>;
#[doc = "System Init. Configuration Register 2."]
pub mod bb_sir2;
#[doc = "BB_SIR3 register accessor: an alias for `Reg<BB_SIR3_SPEC>`"]
pub type BB_SIR3 = crate::Reg<bb_sir3::BB_SIR3_SPEC>;
#[doc = "System Init. Configuration Register 3."]
pub mod bb_sir3;
