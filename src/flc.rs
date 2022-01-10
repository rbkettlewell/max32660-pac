#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Flash Write Address."]
    pub addr: crate::Reg<addr::ADDR_SPEC>,
    #[doc = "0x04 - Flash Clock Divide. The clock (PLL0) is divided by this value to generate a 1 MHz clock for Flash controller."]
    pub clkdiv: crate::Reg<clkdiv::CLKDIV_SPEC>,
    #[doc = "0x08 - Flash Control Register."]
    pub cn: crate::Reg<cn::CN_SPEC>,
    _reserved3: [u8; 0x18],
    #[doc = "0x24 - Flash Interrupt Register."]
    pub intr: crate::Reg<intr::INTR_SPEC>,
    _reserved4: [u8; 0x08],
    #[doc = "0x30..0x40 - Flash Write Data."]
    pub data: [crate::Reg<data::DATA_SPEC>; 4],
    #[doc = "0x40 - Access Control Register. Writing the ACNTL register with the following values in the order shown, allows read and write access to the system and user Information block: pflc-acntl = 0x3a7f5ca3; pflc-acntl = 0xa1e34f20; pflc-acntl = 0x9608b2c1. When unlocked, a write of any word will disable access to system and user information block. Readback of this register is always zero."]
    pub acntl: crate::Reg<acntl::ACNTL_SPEC>,
}
#[doc = "ADDR register accessor: an alias for `Reg<ADDR_SPEC>`"]
pub type ADDR = crate::Reg<addr::ADDR_SPEC>;
#[doc = "Flash Write Address."]
pub mod addr;
#[doc = "CLKDIV register accessor: an alias for `Reg<CLKDIV_SPEC>`"]
pub type CLKDIV = crate::Reg<clkdiv::CLKDIV_SPEC>;
#[doc = "Flash Clock Divide. The clock (PLL0) is divided by this value to generate a 1 MHz clock for Flash controller."]
pub mod clkdiv;
#[doc = "CN register accessor: an alias for `Reg<CN_SPEC>`"]
pub type CN = crate::Reg<cn::CN_SPEC>;
#[doc = "Flash Control Register."]
pub mod cn;
#[doc = "INTR register accessor: an alias for `Reg<INTR_SPEC>`"]
pub type INTR = crate::Reg<intr::INTR_SPEC>;
#[doc = "Flash Interrupt Register."]
pub mod intr;
#[doc = "DATA register accessor: an alias for `Reg<DATA_SPEC>`"]
pub type DATA = crate::Reg<data::DATA_SPEC>;
#[doc = "Flash Write Data."]
pub mod data;
#[doc = "ACNTL register accessor: an alias for `Reg<ACNTL_SPEC>`"]
pub type ACNTL = crate::Reg<acntl::ACNTL_SPEC>;
#[doc = "Access Control Register. Writing the ACNTL register with the following values in the order shown, allows read and write access to the system and user Information block: pflc-acntl = 0x3a7f5ca3; pflc-acntl = 0xa1e34f20; pflc-acntl = 0x9608b2c1. When unlocked, a write of any word will disable access to system and user information block. Readback of this register is always zero."]
pub mod acntl;
