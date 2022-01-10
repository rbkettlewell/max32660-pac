#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - System Control."]
    pub scon: crate::Reg<scon::SCON_SPEC>,
    #[doc = "0x04 - Reset."]
    pub rstr0: crate::Reg<rstr0::RSTR0_SPEC>,
    #[doc = "0x08 - Clock Control."]
    pub clkcn: crate::Reg<clkcn::CLKCN_SPEC>,
    #[doc = "0x0c - Power Management."]
    pub pm: crate::Reg<pm::PM_SPEC>,
    _reserved4: [u8; 0x08],
    #[doc = "0x18 - Peripheral Clock Divider."]
    pub pckdiv: crate::Reg<pckdiv::PCKDIV_SPEC>,
    _reserved5: [u8; 0x08],
    #[doc = "0x24 - Peripheral Clock Disable."]
    pub perckcn0: crate::Reg<perckcn0::PERCKCN0_SPEC>,
    #[doc = "0x28 - Memory Clock Control Register."]
    pub memckcn: crate::Reg<memckcn::MEMCKCN_SPEC>,
    #[doc = "0x2c - Memory Zeroize Control."]
    pub memzcn: crate::Reg<memzcn::MEMZCN_SPEC>,
    _reserved8: [u8; 0x04],
    #[doc = "0x34 - Smart Card Clock Control."]
    pub scck: crate::Reg<scck::SCCK_SPEC>,
    #[doc = "0x38 - Master Priority Control Register 0."]
    pub mpri0: crate::Reg<mpri0::MPRI0_SPEC>,
    #[doc = "0x3c - Mater Priority Control Register 1."]
    pub mpri1: crate::Reg<mpri1::MPRI1_SPEC>,
    #[doc = "0x40 - System Status Register."]
    pub sysst: crate::Reg<sysst::SYSST_SPEC>,
    #[doc = "0x44 - Reset 1."]
    pub rstr1: crate::Reg<rstr1::RSTR1_SPEC>,
    #[doc = "0x48 - Peripheral Clock Disable."]
    pub perckcn1: crate::Reg<perckcn1::PERCKCN1_SPEC>,
    #[doc = "0x4c - Event Enable Register."]
    pub evten: crate::Reg<evten::EVTEN_SPEC>,
    #[doc = "0x50 - Revision Register."]
    pub revision: crate::Reg<revision::REVISION_SPEC>,
    #[doc = "0x54 - System Status Interrupt Enable Register."]
    pub syssie: crate::Reg<syssie::SYSSIE_SPEC>,
}
#[doc = "SCON register accessor: an alias for `Reg<SCON_SPEC>`"]
pub type SCON = crate::Reg<scon::SCON_SPEC>;
#[doc = "System Control."]
pub mod scon;
#[doc = "RSTR0 register accessor: an alias for `Reg<RSTR0_SPEC>`"]
pub type RSTR0 = crate::Reg<rstr0::RSTR0_SPEC>;
#[doc = "Reset."]
pub mod rstr0;
#[doc = "CLKCN register accessor: an alias for `Reg<CLKCN_SPEC>`"]
pub type CLKCN = crate::Reg<clkcn::CLKCN_SPEC>;
#[doc = "Clock Control."]
pub mod clkcn;
#[doc = "PM register accessor: an alias for `Reg<PM_SPEC>`"]
pub type PM = crate::Reg<pm::PM_SPEC>;
#[doc = "Power Management."]
pub mod pm;
#[doc = "PCKDIV register accessor: an alias for `Reg<PCKDIV_SPEC>`"]
pub type PCKDIV = crate::Reg<pckdiv::PCKDIV_SPEC>;
#[doc = "Peripheral Clock Divider."]
pub mod pckdiv;
#[doc = "PERCKCN0 register accessor: an alias for `Reg<PERCKCN0_SPEC>`"]
pub type PERCKCN0 = crate::Reg<perckcn0::PERCKCN0_SPEC>;
#[doc = "Peripheral Clock Disable."]
pub mod perckcn0;
#[doc = "MEMCKCN register accessor: an alias for `Reg<MEMCKCN_SPEC>`"]
pub type MEMCKCN = crate::Reg<memckcn::MEMCKCN_SPEC>;
#[doc = "Memory Clock Control Register."]
pub mod memckcn;
#[doc = "MEMZCN register accessor: an alias for `Reg<MEMZCN_SPEC>`"]
pub type MEMZCN = crate::Reg<memzcn::MEMZCN_SPEC>;
#[doc = "Memory Zeroize Control."]
pub mod memzcn;
#[doc = "SCCK register accessor: an alias for `Reg<SCCK_SPEC>`"]
pub type SCCK = crate::Reg<scck::SCCK_SPEC>;
#[doc = "Smart Card Clock Control."]
pub mod scck;
#[doc = "MPRI0 register accessor: an alias for `Reg<MPRI0_SPEC>`"]
pub type MPRI0 = crate::Reg<mpri0::MPRI0_SPEC>;
#[doc = "Master Priority Control Register 0."]
pub mod mpri0;
#[doc = "MPRI1 register accessor: an alias for `Reg<MPRI1_SPEC>`"]
pub type MPRI1 = crate::Reg<mpri1::MPRI1_SPEC>;
#[doc = "Mater Priority Control Register 1."]
pub mod mpri1;
#[doc = "SYSST register accessor: an alias for `Reg<SYSST_SPEC>`"]
pub type SYSST = crate::Reg<sysst::SYSST_SPEC>;
#[doc = "System Status Register."]
pub mod sysst;
#[doc = "RSTR1 register accessor: an alias for `Reg<RSTR1_SPEC>`"]
pub type RSTR1 = crate::Reg<rstr1::RSTR1_SPEC>;
#[doc = "Reset 1."]
pub mod rstr1;
#[doc = "PERCKCN1 register accessor: an alias for `Reg<PERCKCN1_SPEC>`"]
pub type PERCKCN1 = crate::Reg<perckcn1::PERCKCN1_SPEC>;
#[doc = "Peripheral Clock Disable."]
pub mod perckcn1;
#[doc = "EVTEN register accessor: an alias for `Reg<EVTEN_SPEC>`"]
pub type EVTEN = crate::Reg<evten::EVTEN_SPEC>;
#[doc = "Event Enable Register."]
pub mod evten;
#[doc = "REVISION register accessor: an alias for `Reg<REVISION_SPEC>`"]
pub type REVISION = crate::Reg<revision::REVISION_SPEC>;
#[doc = "Revision Register."]
pub mod revision;
#[doc = "SYSSIE register accessor: an alias for `Reg<SYSSIE_SPEC>`"]
pub type SYSSIE = crate::Reg<syssie::SYSSIE_SPEC>;
#[doc = "System Status Interrupt Enable Register."]
pub mod syssie;
