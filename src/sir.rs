#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - System Initialization Status Register."]
    pub sistat: crate::Reg<sistat::SISTAT_SPEC>,
    #[doc = "0x04 - Read-only field set by the SIB block if a CRC error occurs during the read of the OTP memory. Contains the failing address in OTP memory (when CRCERR equals 1)."]
    pub erraddr: crate::Reg<erraddr::ERRADDR_SPEC>,
    _reserved2: [u8; 0xf8],
    #[doc = "0x100 - funcstat register."]
    pub fstat: crate::Reg<fstat::FSTAT_SPEC>,
    #[doc = "0x104 - secfuncstat register."]
    pub sfstat: crate::Reg<sfstat::SFSTAT_SPEC>,
}
#[doc = "SISTAT register accessor: an alias for `Reg<SISTAT_SPEC>`"]
pub type SISTAT = crate::Reg<sistat::SISTAT_SPEC>;
#[doc = "System Initialization Status Register."]
pub mod sistat;
#[doc = "ERRADDR register accessor: an alias for `Reg<ERRADDR_SPEC>`"]
pub type ERRADDR = crate::Reg<erraddr::ERRADDR_SPEC>;
#[doc = "Read-only field set by the SIB block if a CRC error occurs during the read of the OTP memory. Contains the failing address in OTP memory (when CRCERR equals 1)."]
pub mod erraddr;
#[doc = "FSTAT register accessor: an alias for `Reg<FSTAT_SPEC>`"]
pub type FSTAT = crate::Reg<fstat::FSTAT_SPEC>;
#[doc = "funcstat register."]
pub mod fstat;
#[doc = "SFSTAT register accessor: an alias for `Reg<SFSTAT_SPEC>`"]
pub type SFSTAT = crate::Reg<sfstat::SFSTAT_SPEC>;
#[doc = "secfuncstat register."]
pub mod sfstat;
