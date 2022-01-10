#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register0."]
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    #[doc = "0x04 - Status Register."]
    pub status: crate::Reg<status::STATUS_SPEC>,
    #[doc = "0x08 - Interrupt Status Register."]
    pub int_fl0: crate::Reg<int_fl0::INT_FL0_SPEC>,
    #[doc = "0x0c - Interrupt Enable Register."]
    pub int_en0: crate::Reg<int_en0::INT_EN0_SPEC>,
    #[doc = "0x10 - Interrupt Status Register 1."]
    pub int_fl1: crate::Reg<int_fl1::INT_FL1_SPEC>,
    #[doc = "0x14 - Interrupt Staus Register 1."]
    pub int_en1: crate::Reg<int_en1::INT_EN1_SPEC>,
    #[doc = "0x18 - FIFO Configuration Register."]
    pub fifo_len: crate::Reg<fifo_len::FIFO_LEN_SPEC>,
    #[doc = "0x1c - Receive Control Register 0."]
    pub rx_ctrl0: crate::Reg<rx_ctrl0::RX_CTRL0_SPEC>,
    #[doc = "0x20 - Receive Control Register 1."]
    pub rx_ctrl1: crate::Reg<rx_ctrl1::RX_CTRL1_SPEC>,
    #[doc = "0x24 - Transmit Control Register 0."]
    pub tx_ctrl0: crate::Reg<tx_ctrl0::TX_CTRL0_SPEC>,
    #[doc = "0x28 - Transmit Control Register 1."]
    pub tx_ctrl1: crate::Reg<tx_ctrl1::TX_CTRL1_SPEC>,
    #[doc = "0x2c - Data Register."]
    pub fifo: crate::Reg<fifo::FIFO_SPEC>,
    #[doc = "0x30 - Master Control Register."]
    pub master_ctrl: crate::Reg<master_ctrl::MASTER_CTRL_SPEC>,
    #[doc = "0x34 - Clock Low Register."]
    pub clk_lo: crate::Reg<clk_lo::CLK_LO_SPEC>,
    #[doc = "0x38 - Clock high Register."]
    pub clk_hi: crate::Reg<clk_hi::CLK_HI_SPEC>,
    #[doc = "0x3c - HS-Mode Clock Control Register"]
    pub hs_clk: crate::Reg<hs_clk::HS_CLK_SPEC>,
    #[doc = "0x40 - Timeout Register"]
    pub timeout: crate::Reg<timeout::TIMEOUT_SPEC>,
    #[doc = "0x44 - Slave Address Register."]
    pub slave_addr: crate::Reg<slave_addr::SLAVE_ADDR_SPEC>,
    #[doc = "0x48 - DMA Register."]
    pub dma: crate::Reg<dma::DMA_SPEC>,
}
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control Register0."]
pub mod ctrl;
#[doc = "STATUS register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status Register."]
pub mod status;
#[doc = "INT_FL0 register accessor: an alias for `Reg<INT_FL0_SPEC>`"]
pub type INT_FL0 = crate::Reg<int_fl0::INT_FL0_SPEC>;
#[doc = "Interrupt Status Register."]
pub mod int_fl0;
#[doc = "INT_EN0 register accessor: an alias for `Reg<INT_EN0_SPEC>`"]
pub type INT_EN0 = crate::Reg<int_en0::INT_EN0_SPEC>;
#[doc = "Interrupt Enable Register."]
pub mod int_en0;
#[doc = "INT_FL1 register accessor: an alias for `Reg<INT_FL1_SPEC>`"]
pub type INT_FL1 = crate::Reg<int_fl1::INT_FL1_SPEC>;
#[doc = "Interrupt Status Register 1."]
pub mod int_fl1;
#[doc = "INT_EN1 register accessor: an alias for `Reg<INT_EN1_SPEC>`"]
pub type INT_EN1 = crate::Reg<int_en1::INT_EN1_SPEC>;
#[doc = "Interrupt Staus Register 1."]
pub mod int_en1;
#[doc = "FIFO_LEN register accessor: an alias for `Reg<FIFO_LEN_SPEC>`"]
pub type FIFO_LEN = crate::Reg<fifo_len::FIFO_LEN_SPEC>;
#[doc = "FIFO Configuration Register."]
pub mod fifo_len;
#[doc = "RX_CTRL0 register accessor: an alias for `Reg<RX_CTRL0_SPEC>`"]
pub type RX_CTRL0 = crate::Reg<rx_ctrl0::RX_CTRL0_SPEC>;
#[doc = "Receive Control Register 0."]
pub mod rx_ctrl0;
#[doc = "RX_CTRL1 register accessor: an alias for `Reg<RX_CTRL1_SPEC>`"]
pub type RX_CTRL1 = crate::Reg<rx_ctrl1::RX_CTRL1_SPEC>;
#[doc = "Receive Control Register 1."]
pub mod rx_ctrl1;
#[doc = "TX_CTRL0 register accessor: an alias for `Reg<TX_CTRL0_SPEC>`"]
pub type TX_CTRL0 = crate::Reg<tx_ctrl0::TX_CTRL0_SPEC>;
#[doc = "Transmit Control Register 0."]
pub mod tx_ctrl0;
#[doc = "TX_CTRL1 register accessor: an alias for `Reg<TX_CTRL1_SPEC>`"]
pub type TX_CTRL1 = crate::Reg<tx_ctrl1::TX_CTRL1_SPEC>;
#[doc = "Transmit Control Register 1."]
pub mod tx_ctrl1;
#[doc = "FIFO register accessor: an alias for `Reg<FIFO_SPEC>`"]
pub type FIFO = crate::Reg<fifo::FIFO_SPEC>;
#[doc = "Data Register."]
pub mod fifo;
#[doc = "MASTER_CTRL register accessor: an alias for `Reg<MASTER_CTRL_SPEC>`"]
pub type MASTER_CTRL = crate::Reg<master_ctrl::MASTER_CTRL_SPEC>;
#[doc = "Master Control Register."]
pub mod master_ctrl;
#[doc = "CLK_LO register accessor: an alias for `Reg<CLK_LO_SPEC>`"]
pub type CLK_LO = crate::Reg<clk_lo::CLK_LO_SPEC>;
#[doc = "Clock Low Register."]
pub mod clk_lo;
#[doc = "CLK_HI register accessor: an alias for `Reg<CLK_HI_SPEC>`"]
pub type CLK_HI = crate::Reg<clk_hi::CLK_HI_SPEC>;
#[doc = "Clock high Register."]
pub mod clk_hi;
#[doc = "HS_CLK register accessor: an alias for `Reg<HS_CLK_SPEC>`"]
pub type HS_CLK = crate::Reg<hs_clk::HS_CLK_SPEC>;
#[doc = "HS-Mode Clock Control Register"]
pub mod hs_clk;
#[doc = "TIMEOUT register accessor: an alias for `Reg<TIMEOUT_SPEC>`"]
pub type TIMEOUT = crate::Reg<timeout::TIMEOUT_SPEC>;
#[doc = "Timeout Register"]
pub mod timeout;
#[doc = "SLAVE_ADDR register accessor: an alias for `Reg<SLAVE_ADDR_SPEC>`"]
pub type SLAVE_ADDR = crate::Reg<slave_addr::SLAVE_ADDR_SPEC>;
#[doc = "Slave Address Register."]
pub mod slave_addr;
#[doc = "DMA register accessor: an alias for `Reg<DMA_SPEC>`"]
pub type DMA = crate::Reg<dma::DMA_SPEC>;
#[doc = "DMA Register."]
pub mod dma;
