#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Cache ID Register."]
    pub cache_id: crate::Reg<cache_id::CACHE_ID_SPEC>,
    #[doc = "0x04 - Memory Configuration Register."]
    pub memcfg: crate::Reg<memcfg::MEMCFG_SPEC>,
    _reserved2: [u8; 0xf8],
    #[doc = "0x100 - Cache Control and Status Register."]
    pub cache_ctrl: crate::Reg<cache_ctrl::CACHE_CTRL_SPEC>,
    _reserved3: [u8; 0x05fc],
    #[doc = "0x700 - Invalidate All Registers."]
    pub invalidate: crate::Reg<invalidate::INVALIDATE_SPEC>,
}
#[doc = "CACHE_ID register accessor: an alias for `Reg<CACHE_ID_SPEC>`"]
pub type CACHE_ID = crate::Reg<cache_id::CACHE_ID_SPEC>;
#[doc = "Cache ID Register."]
pub mod cache_id;
#[doc = "MEMCFG register accessor: an alias for `Reg<MEMCFG_SPEC>`"]
pub type MEMCFG = crate::Reg<memcfg::MEMCFG_SPEC>;
#[doc = "Memory Configuration Register."]
pub mod memcfg;
#[doc = "CACHE_CTRL register accessor: an alias for `Reg<CACHE_CTRL_SPEC>`"]
pub type CACHE_CTRL = crate::Reg<cache_ctrl::CACHE_CTRL_SPEC>;
#[doc = "Cache Control and Status Register."]
pub mod cache_ctrl;
#[doc = "INVALIDATE register accessor: an alias for `Reg<INVALIDATE_SPEC>`"]
pub type INVALIDATE = crate::Reg<invalidate::INVALIDATE_SPEC>;
#[doc = "Invalidate All Registers."]
pub mod invalidate;
