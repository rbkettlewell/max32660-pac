#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Count. This register stores the current timer count."]
    pub cnt: crate::Reg<cnt::CNT_SPEC>,
    #[doc = "0x04 - Compare. This register stores the compare value, which is used to set the maximum count value to initiate a reload of the timer to 0x0001."]
    pub cmp: crate::Reg<cmp::CMP_SPEC>,
    #[doc = "0x08 - PWM. This register stores the value that is compared to the current timer count."]
    pub pwm: crate::Reg<pwm::PWM_SPEC>,
    #[doc = "0x0c - Clear Interrupt. Writing a value (0 or 1) to a bit in this register clears the associated interrupt."]
    pub intr: crate::Reg<intr::INTR_SPEC>,
    #[doc = "0x10 - Timer Control Register."]
    pub cn: crate::Reg<cn::CN_SPEC>,
    #[doc = "0x14 - Timer Non-Overlapping Compare Register."]
    pub nolcmp: crate::Reg<nolcmp::NOLCMP_SPEC>,
}
#[doc = "CNT register accessor: an alias for `Reg<CNT_SPEC>`"]
pub type CNT = crate::Reg<cnt::CNT_SPEC>;
#[doc = "Count. This register stores the current timer count."]
pub mod cnt;
#[doc = "CMP register accessor: an alias for `Reg<CMP_SPEC>`"]
pub type CMP = crate::Reg<cmp::CMP_SPEC>;
#[doc = "Compare. This register stores the compare value, which is used to set the maximum count value to initiate a reload of the timer to 0x0001."]
pub mod cmp;
#[doc = "PWM register accessor: an alias for `Reg<PWM_SPEC>`"]
pub type PWM = crate::Reg<pwm::PWM_SPEC>;
#[doc = "PWM. This register stores the value that is compared to the current timer count."]
pub mod pwm;
#[doc = "INTR register accessor: an alias for `Reg<INTR_SPEC>`"]
pub type INTR = crate::Reg<intr::INTR_SPEC>;
#[doc = "Clear Interrupt. Writing a value (0 or 1) to a bit in this register clears the associated interrupt."]
pub mod intr;
#[doc = "CN register accessor: an alias for `Reg<CN_SPEC>`"]
pub type CN = crate::Reg<cn::CN_SPEC>;
#[doc = "Timer Control Register."]
pub mod cn;
#[doc = "NOLCMP register accessor: an alias for `Reg<NOLCMP_SPEC>`"]
pub type NOLCMP = crate::Reg<nolcmp::NOLCMP_SPEC>;
#[doc = "Timer Non-Overlapping Compare Register."]
pub mod nolcmp;
