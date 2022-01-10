#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Low Power Control Register."]
    pub lp_ctrl: crate::Reg<lp_ctrl::LP_CTRL_SPEC>,
    #[doc = "0x04 - Low Power Mode Wakeup Flags for GPIO0"]
    pub lp_wakefl: crate::Reg<lp_wakefl::LP_WAKEFL_SPEC>,
    #[doc = "0x08 - Low Power I/O Wakeup Enable Register 0. This register enables low power wakeup functionality for GPIO0."]
    pub lpwk_en: crate::Reg<lpwk_en::LPWK_EN_SPEC>,
    _reserved3: [u8; 0x34],
    #[doc = "0x40 - Low Power Memory Shutdown Control."]
    pub lpmemsd: crate::Reg<lpmemsd::LPMEMSD_SPEC>,
}
#[doc = "LP_CTRL register accessor: an alias for `Reg<LP_CTRL_SPEC>`"]
pub type LP_CTRL = crate::Reg<lp_ctrl::LP_CTRL_SPEC>;
#[doc = "Low Power Control Register."]
pub mod lp_ctrl;
#[doc = "LP_WAKEFL register accessor: an alias for `Reg<LP_WAKEFL_SPEC>`"]
pub type LP_WAKEFL = crate::Reg<lp_wakefl::LP_WAKEFL_SPEC>;
#[doc = "Low Power Mode Wakeup Flags for GPIO0"]
pub mod lp_wakefl;
#[doc = "LPWK_EN register accessor: an alias for `Reg<LPWK_EN_SPEC>`"]
pub type LPWK_EN = crate::Reg<lpwk_en::LPWK_EN_SPEC>;
#[doc = "Low Power I/O Wakeup Enable Register 0. This register enables low power wakeup functionality for GPIO0."]
pub mod lpwk_en;
#[doc = "LPMEMSD register accessor: an alias for `Reg<LPMEMSD_SPEC>`"]
pub type LPMEMSD = crate::Reg<lpmemsd::LPMEMSD_SPEC>;
#[doc = "Low Power Memory Shutdown Control."]
pub mod lpmemsd;
