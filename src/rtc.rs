#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - RTC Second Counter. This register contains the 32-bit second counter."]
    pub sec: crate::Reg<sec::SEC_SPEC>,
    #[doc = "0x04 - RTC Sub-second Counter. This counter increments at 256Hz. RTC_SEC is incremented when this register rolls over from 0xFF to 0x00."]
    pub ssec: crate::Reg<ssec::SSEC_SPEC>,
    #[doc = "0x08 - Time-of-day Alarm."]
    pub ras: crate::Reg<ras::RAS_SPEC>,
    #[doc = "0x0c - RTC sub-second alarm. This register contains the reload value for the sub-second alarm."]
    pub rssa: crate::Reg<rssa::RSSA_SPEC>,
    #[doc = "0x10 - RTC Control Register."]
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    #[doc = "0x14 - RTC Trim Register."]
    pub trim: crate::Reg<trim::TRIM_SPEC>,
    #[doc = "0x18 - RTC Oscillator Control Register."]
    pub oscctrl: crate::Reg<oscctrl::OSCCTRL_SPEC>,
}
#[doc = "SEC register accessor: an alias for `Reg<SEC_SPEC>`"]
pub type SEC = crate::Reg<sec::SEC_SPEC>;
#[doc = "RTC Second Counter. This register contains the 32-bit second counter."]
pub mod sec;
#[doc = "SSEC register accessor: an alias for `Reg<SSEC_SPEC>`"]
pub type SSEC = crate::Reg<ssec::SSEC_SPEC>;
#[doc = "RTC Sub-second Counter. This counter increments at 256Hz. RTC_SEC is incremented when this register rolls over from 0xFF to 0x00."]
pub mod ssec;
#[doc = "RAS register accessor: an alias for `Reg<RAS_SPEC>`"]
pub type RAS = crate::Reg<ras::RAS_SPEC>;
#[doc = "Time-of-day Alarm."]
pub mod ras;
#[doc = "RSSA register accessor: an alias for `Reg<RSSA_SPEC>`"]
pub type RSSA = crate::Reg<rssa::RSSA_SPEC>;
#[doc = "RTC sub-second alarm. This register contains the reload value for the sub-second alarm."]
pub mod rssa;
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "RTC Control Register."]
pub mod ctrl;
#[doc = "TRIM register accessor: an alias for `Reg<TRIM_SPEC>`"]
pub type TRIM = crate::Reg<trim::TRIM_SPEC>;
#[doc = "RTC Trim Register."]
pub mod trim;
#[doc = "OSCCTRL register accessor: an alias for `Reg<OSCCTRL_SPEC>`"]
pub type OSCCTRL = crate::Reg<oscctrl::OSCCTRL_SPEC>;
#[doc = "RTC Oscillator Control Register."]
pub mod oscctrl;
