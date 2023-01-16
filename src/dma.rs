#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DMA Control Register."]
    pub int_en: crate::Reg<int_en::INT_EN_SPEC>,
    #[doc = "0x04 - DMA Interrupt Register."]
    pub int_fl: crate::Reg<int_fl::INT_FL_SPEC>,
    _reserved2: [u8; 0xf8],
    #[doc = "0x100..0x180 - DMA Channel registers."]
    pub ch: [CH; 4],
}
#[doc = r"Register block"]
#[repr(C)]
pub struct CH {
    #[doc = "0x00 - DMA Channel Configuration Register."]
    pub cfg: crate::Reg<self::ch::cfg::CFG_SPEC>,
    #[doc = "0x04 - DMA Channel Status Register."]
    pub stat: crate::Reg<self::ch::stat::STAT_SPEC>,
    #[doc = "0x08 - Source Device Address. If SRCINC=1, the counter bits are incremented by 1,2, or 4, depending on the data width of each AHB cycle. For peripheral transfers, some or all of the actual address bits are fixed. If SRCINC=0, this register remains constant. In the case where a count-to-zero condition occurs while RLDEN=1, the register is reloaded with the contents of DMA_SRC_RLD."]
    pub src: crate::Reg<self::ch::src::SRC_SPEC>,
    #[doc = "0x0c - Destination Device Address. For peripheral transfers, some or all of the actual address bits are fixed. If DSTINC=1, this register is incremented on every AHB write out of the DMA FIFO. They are incremented by 1, 2, or 4, depending on the data width of each AHB cycle. In the case where a count-to-zero condition occurs while RLDEN=1, the register is reloaded with DMA_DST_RLD."]
    pub dst: crate::Reg<self::ch::dst::DST_SPEC>,
    #[doc = "0x10 - DMA Counter. The user loads this register with the number of bytes to transfer. This counter decreases on every AHB cycle into the DMA FIFO. The decrement will be 1, 2, or 4 depending on the data width of each AHB cycle. When the counter reaches 0, a count-to-zero condition is triggered."]
    pub cnt: crate::Reg<self::ch::cnt::CNT_SPEC>,
    #[doc = "0x14 - Source Address Reload Value. The value of this register is loaded into DMA0_SRC upon a count-to-zero condition."]
    pub src_rld: crate::Reg<self::ch::src_rld::SRC_RLD_SPEC>,
    #[doc = "0x18 - Destination Address Reload Value. The value of this register is loaded into DMA0_DST upon a count-to-zero condition."]
    pub dst_rld: crate::Reg<self::ch::dst_rld::DST_RLD_SPEC>,
    #[doc = "0x1c - DMA Channel Count Reload Register."]
    pub cnt_rld: crate::Reg<self::ch::cnt_rld::CNT_RLD_SPEC>,
}
#[doc = r"Register block"]
#[doc = "DMA Channel registers."]
pub mod ch;
#[doc = "INT_EN register accessor: an alias for `Reg<INT_EN_SPEC>`"]
pub type INT_EN = crate::Reg<int_en::INT_EN_SPEC>;
#[doc = "DMA Control Register."]
pub mod int_en;
#[doc = "INT_FL register accessor: an alias for `Reg<INT_FL_SPEC>`"]
pub type INT_FL = crate::Reg<int_fl::INT_FL_SPEC>;
#[doc = "DMA Interrupt Register."]
pub mod int_fl;
