#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register."]
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    #[doc = "0x04 - Threshold Control register."]
    pub thresh_ctrl: crate::Reg<thresh_ctrl::THRESH_CTRL_SPEC>,
    #[doc = "0x08 - Status Register."]
    pub status: crate::Reg<status::STATUS_SPEC>,
    #[doc = "0x0c - Interrupt Enable Register."]
    pub int_en: crate::Reg<int_en::INT_EN_SPEC>,
    #[doc = "0x10 - Interrupt Status Flags."]
    pub int_fl: crate::Reg<int_fl::INT_FL_SPEC>,
    #[doc = "0x14 - Baud rate register. Integer portion."]
    pub baud0: crate::Reg<baud0::BAUD0_SPEC>,
    #[doc = "0x18 - Baud rate register. Decimal Setting."]
    pub baud1: crate::Reg<baud1::BAUD1_SPEC>,
    #[doc = "0x1c - FIFO Data buffer."]
    pub fifo: crate::Reg<fifo::FIFO_SPEC>,
    #[doc = "0x20 - DMA Configuration."]
    pub dma: crate::Reg<dma::DMA_SPEC>,
    #[doc = "0x24 - Transmit FIFO Status register."]
    pub tx_fifo: crate::Reg<tx_fifo::TX_FIFO_SPEC>,
}
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control Register."]
pub mod ctrl;
#[doc = "THRESH_CTRL register accessor: an alias for `Reg<THRESH_CTRL_SPEC>`"]
pub type THRESH_CTRL = crate::Reg<thresh_ctrl::THRESH_CTRL_SPEC>;
#[doc = "Threshold Control register."]
pub mod thresh_ctrl;
#[doc = "STATUS register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status Register."]
pub mod status;
#[doc = "INT_EN register accessor: an alias for `Reg<INT_EN_SPEC>`"]
pub type INT_EN = crate::Reg<int_en::INT_EN_SPEC>;
#[doc = "Interrupt Enable Register."]
pub mod int_en;
#[doc = "INT_FL register accessor: an alias for `Reg<INT_FL_SPEC>`"]
pub type INT_FL = crate::Reg<int_fl::INT_FL_SPEC>;
#[doc = "Interrupt Status Flags."]
pub mod int_fl;
#[doc = "BAUD0 register accessor: an alias for `Reg<BAUD0_SPEC>`"]
pub type BAUD0 = crate::Reg<baud0::BAUD0_SPEC>;
#[doc = "Baud rate register. Integer portion."]
pub mod baud0;
#[doc = "BAUD1 register accessor: an alias for `Reg<BAUD1_SPEC>`"]
pub type BAUD1 = crate::Reg<baud1::BAUD1_SPEC>;
#[doc = "Baud rate register. Decimal Setting."]
pub mod baud1;
#[doc = "FIFO register accessor: an alias for `Reg<FIFO_SPEC>`"]
pub type FIFO = crate::Reg<fifo::FIFO_SPEC>;
#[doc = "FIFO Data buffer."]
pub mod fifo;
#[doc = "DMA register accessor: an alias for `Reg<DMA_SPEC>`"]
pub type DMA = crate::Reg<dma::DMA_SPEC>;
#[doc = "DMA Configuration."]
pub mod dma;
#[doc = "TX_FIFO register accessor: an alias for `Reg<TX_FIFO_SPEC>`"]
pub type TX_FIFO = crate::Reg<tx_fifo::TX_FIFO_SPEC>;
#[doc = "Transmit FIFO Status register."]
pub mod tx_fifo;
