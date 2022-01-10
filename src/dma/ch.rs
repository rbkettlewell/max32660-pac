#[doc = "CFG register accessor: an alias for `Reg<CFG_SPEC>`"]
pub type CFG = crate::Reg<cfg::CFG_SPEC>;
#[doc = "DMA Channel Configuration Register."]
pub mod cfg;
#[doc = "ST register accessor: an alias for `Reg<ST_SPEC>`"]
pub type ST = crate::Reg<st::ST_SPEC>;
#[doc = "DMA Channel Status Register."]
pub mod st;
#[doc = "SRC register accessor: an alias for `Reg<SRC_SPEC>`"]
pub type SRC = crate::Reg<src::SRC_SPEC>;
#[doc = "Source Device Address. If SRCINC=1, the counter bits are incremented by 1,2, or 4, depending on the data width of each AHB cycle. For peripheral transfers, some or all of the actual address bits are fixed. If SRCINC=0, this register remains constant. In the case where a count-to-zero condition occurs while RLDEN=1, the register is reloaded with the contents of DMA_SRC_RLD."]
pub mod src;
#[doc = "DST register accessor: an alias for `Reg<DST_SPEC>`"]
pub type DST = crate::Reg<dst::DST_SPEC>;
#[doc = "Destination Device Address. For peripheral transfers, some or all of the actual address bits are fixed. If DSTINC=1, this register is incremented on every AHB write out of the DMA FIFO. They are incremented by 1, 2, or 4, depending on the data width of each AHB cycle. In the case where a count-to-zero condition occurs while RLDEN=1, the register is reloaded with DMA_DST_RLD."]
pub mod dst;
#[doc = "CNT register accessor: an alias for `Reg<CNT_SPEC>`"]
pub type CNT = crate::Reg<cnt::CNT_SPEC>;
#[doc = "DMA Counter. The user loads this register with the number of bytes to transfer. This counter decreases on every AHB cycle into the DMA FIFO. The decrement will be 1, 2, or 4 depending on the data width of each AHB cycle. When the counter reaches 0, a count-to-zero condition is triggered."]
pub mod cnt;
#[doc = "SRC_RLD register accessor: an alias for `Reg<SRC_RLD_SPEC>`"]
pub type SRC_RLD = crate::Reg<src_rld::SRC_RLD_SPEC>;
#[doc = "Source Address Reload Value. The value of this register is loaded into DMA0_SRC upon a count-to-zero condition."]
pub mod src_rld;
#[doc = "DST_RLD register accessor: an alias for `Reg<DST_RLD_SPEC>`"]
pub type DST_RLD = crate::Reg<dst_rld::DST_RLD_SPEC>;
#[doc = "Destination Address Reload Value. The value of this register is loaded into DMA0_DST upon a count-to-zero condition."]
pub mod dst_rld;
#[doc = "CNT_RLD register accessor: an alias for `Reg<CNT_RLD_SPEC>`"]
pub type CNT_RLD = crate::Reg<cnt_rld::CNT_RLD_SPEC>;
#[doc = "DMA Channel Count Reload Register."]
pub mod cnt_rld;
