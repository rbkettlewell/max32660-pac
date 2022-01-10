#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - External Sensor Control Register."]
    pub extscn: crate::Reg<extscn::EXTSCN_SPEC>,
    #[doc = "0x04 - Internal Sensor Control Register."]
    pub intscn: crate::Reg<intscn::INTSCN_SPEC>,
    #[doc = "0x08 - Security Alarm Register."]
    pub secalm: crate::Reg<secalm::SECALM_SPEC>,
    #[doc = "0x0c - Security Diagnostic Register."]
    pub secdiag: crate::Reg<secdiag::SECDIAG_SPEC>,
    #[doc = "0x10 - DRS Log RTC Value. This register contains the 32 bit value in the RTC second register when the last DRS event occurred."]
    pub dlrtc: crate::Reg<dlrtc::DLRTC_SPEC>,
    _reserved5: [u8; 0x20],
    #[doc = "0x34 - Security Monitor Status Register."]
    pub secst: crate::Reg<secst::SECST_SPEC>,
}
#[doc = "EXTSCN register accessor: an alias for `Reg<EXTSCN_SPEC>`"]
pub type EXTSCN = crate::Reg<extscn::EXTSCN_SPEC>;
#[doc = "External Sensor Control Register."]
pub mod extscn;
#[doc = "INTSCN register accessor: an alias for `Reg<INTSCN_SPEC>`"]
pub type INTSCN = crate::Reg<intscn::INTSCN_SPEC>;
#[doc = "Internal Sensor Control Register."]
pub mod intscn;
#[doc = "SECALM register accessor: an alias for `Reg<SECALM_SPEC>`"]
pub type SECALM = crate::Reg<secalm::SECALM_SPEC>;
#[doc = "Security Alarm Register."]
pub mod secalm;
#[doc = "SECDIAG register accessor: an alias for `Reg<SECDIAG_SPEC>`"]
pub type SECDIAG = crate::Reg<secdiag::SECDIAG_SPEC>;
#[doc = "Security Diagnostic Register."]
pub mod secdiag;
#[doc = "DLRTC register accessor: an alias for `Reg<DLRTC_SPEC>`"]
pub type DLRTC = crate::Reg<dlrtc::DLRTC_SPEC>;
#[doc = "DRS Log RTC Value. This register contains the 32 bit value in the RTC second register when the last DRS event occurred."]
pub mod dlrtc;
#[doc = "SECST register accessor: an alias for `Reg<SECST_SPEC>`"]
pub type SECST = crate::Reg<secst::SECST_SPEC>;
#[doc = "Security Monitor Status Register."]
pub mod secst;
