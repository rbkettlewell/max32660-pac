#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DMA Control Register."]
    pub cn: crate::Reg<cn::CN_SPEC>,
    #[doc = "0x04 - DMA Interrupt Register."]
    pub intr: crate::Reg<intr::INTR_SPEC>,
    _reserved2: [u8; 0xf8],
    _reserved_2_ch0: [u8; 0x012c],
}
impl RegisterBlock {
    #[doc = "0x100..0x220 - DMA Channel registers."]
    #[inline(always)]
    pub fn ch0(&self) -> &CH {
        unsafe { &*(((self as *const Self) as *const u8).add(256usize) as *const CH) }
    }
    #[doc = "0x104..0x224 - DMA Channel registers."]
    #[inline(always)]
    pub fn ch1(&self) -> &CH {
        unsafe { &*(((self as *const Self) as *const u8).add(260usize) as *const CH) }
    }
    #[doc = "0x108..0x228 - DMA Channel registers."]
    #[inline(always)]
    pub fn ch2(&self) -> &CH {
        unsafe { &*(((self as *const Self) as *const u8).add(264usize) as *const CH) }
    }
    #[doc = "0x10c..0x22c - DMA Channel registers."]
    #[inline(always)]
    pub fn ch3(&self) -> &CH {
        unsafe { &*(((self as *const Self) as *const u8).add(268usize) as *const CH) }
    }
}
#[doc = r"Register block"]
#[repr(C)]
pub struct CH {
    _reserved0: [u8; 0x0100],
    #[doc = "0x100 - DMA Channel Configuration Register."]
    pub cfg: crate::Reg<self::ch::cfg::CFG_SPEC>,
    #[doc = "0x104 - DMA Channel Status Register."]
    pub st: crate::Reg<self::ch::st::ST_SPEC>,
    #[doc = "0x108 - Source Device Address. If SRCINC=1, the counter bits are incremented by 1,2, or 4, depending on the data width of each AHB cycle. For peripheral transfers, some or all of the actual address bits are fixed. If SRCINC=0, this register remains constant. In the case where a count-to-zero condition occurs while RLDEN=1, the register is reloaded with the contents of DMA_SRC_RLD."]
    pub src: crate::Reg<self::ch::src::SRC_SPEC>,
    #[doc = "0x10c - Destination Device Address. For peripheral transfers, some or all of the actual address bits are fixed. If DSTINC=1, this register is incremented on every AHB write out of the DMA FIFO. They are incremented by 1, 2, or 4, depending on the data width of each AHB cycle. In the case where a count-to-zero condition occurs while RLDEN=1, the register is reloaded with DMA_DST_RLD."]
    pub dst: crate::Reg<self::ch::dst::DST_SPEC>,
    #[doc = "0x110 - DMA Counter. The user loads this register with the number of bytes to transfer. This counter decreases on every AHB cycle into the DMA FIFO. The decrement will be 1, 2, or 4 depending on the data width of each AHB cycle. When the counter reaches 0, a count-to-zero condition is triggered."]
    pub cnt: crate::Reg<self::ch::cnt::CNT_SPEC>,
    #[doc = "0x114 - Source Address Reload Value. The value of this register is loaded into DMA0_SRC upon a count-to-zero condition."]
    pub src_rld: crate::Reg<self::ch::src_rld::SRC_RLD_SPEC>,
    #[doc = "0x118 - Destination Address Reload Value. The value of this register is loaded into DMA0_DST upon a count-to-zero condition."]
    pub dst_rld: crate::Reg<self::ch::dst_rld::DST_RLD_SPEC>,
    #[doc = "0x11c - DMA Channel Count Reload Register."]
    pub cnt_rld: crate::Reg<self::ch::cnt_rld::CNT_RLD_SPEC>,
}
#[doc = r"Register block"]
#[doc = "DMA Channel registers."]
pub mod ch;
#[doc = "CN register accessor: an alias for `Reg<CN_SPEC>`"]
pub type CN = crate::Reg<cn::CN_SPEC>;
#[doc = "DMA Control Register."]
pub mod cn;
#[doc = "INTR register accessor: an alias for `Reg<INTR_SPEC>`"]
pub type INTR = crate::Reg<intr::INTR_SPEC>;
#[doc = "DMA Interrupt Register."]
pub mod intr;
