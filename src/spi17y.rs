#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_data8: [u8; 0x04],
    #[doc = "0x04 - Register for controlling SPI peripheral."]
    pub ctrl0: crate::Reg<ctrl0::CTRL0_SPEC>,
    #[doc = "0x08 - Register for controlling SPI peripheral."]
    pub ctrl1: crate::Reg<ctrl1::CTRL1_SPEC>,
    #[doc = "0x0c - Register for controlling SPI peripheral."]
    pub ctrl2: crate::Reg<ctrl2::CTRL2_SPEC>,
    #[doc = "0x10 - Register for controlling SPI peripheral/Slave Select Timing."]
    pub ss_time: crate::Reg<ss_time::SS_TIME_SPEC>,
    #[doc = "0x14 - Register for controlling SPI clock rate."]
    pub clk_cfg: crate::Reg<clk_cfg::CLK_CFG_SPEC>,
    _reserved6: [u8; 0x04],
    #[doc = "0x1c - Register for controlling DMA."]
    pub dma: crate::Reg<dma::DMA_SPEC>,
    #[doc = "0x20 - Register for reading and clearing interrupt flags. All bits are write 1 to clear."]
    pub int_fl: crate::Reg<int_fl::INT_FL_SPEC>,
    #[doc = "0x24 - Register for enabling interrupts."]
    pub int_en: crate::Reg<int_en::INT_EN_SPEC>,
    #[doc = "0x28 - Register for wake up flags. All bits in this register are write 1 to clear."]
    pub wake_fl: crate::Reg<wake_fl::WAKE_FL_SPEC>,
    #[doc = "0x2c - Register for wake up enable."]
    pub wake_en: crate::Reg<wake_en::WAKE_EN_SPEC>,
    #[doc = "0x30 - SPI Status register."]
    pub stat: crate::Reg<stat::STAT_SPEC>,
}
impl RegisterBlock {
    #[doc = "0x00 - Register for reading and writing the FIFO."]
    #[inline(always)]
    pub fn data8(&self) -> &[crate::Reg<data8::DATA8_SPEC>; 4] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(0usize)
                as *const [crate::Reg<data8::DATA8_SPEC>; 4])
        }
    }
    #[doc = "0x00 - Register for reading and writing the FIFO."]
    #[inline(always)]
    pub fn data16(&self) -> &[crate::Reg<data16::DATA16_SPEC>; 2] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(0usize)
                as *const [crate::Reg<data16::DATA16_SPEC>; 2])
        }
    }
    #[doc = "0x00 - Register for reading and writing the FIFO."]
    #[inline(always)]
    pub fn data32(&self) -> &crate::Reg<data32::DATA32_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(0usize)
                as *const crate::Reg<data32::DATA32_SPEC>)
        }
    }
}
#[doc = "DATA32 register accessor: an alias for `Reg<DATA32_SPEC>`"]
pub type DATA32 = crate::Reg<data32::DATA32_SPEC>;
#[doc = "Register for reading and writing the FIFO."]
pub mod data32;
#[doc = "DATA16 register accessor: an alias for `Reg<DATA16_SPEC>`"]
pub type DATA16 = crate::Reg<data16::DATA16_SPEC>;
#[doc = "Register for reading and writing the FIFO."]
pub mod data16;
#[doc = "DATA8 register accessor: an alias for `Reg<DATA8_SPEC>`"]
pub type DATA8 = crate::Reg<data8::DATA8_SPEC>;
#[doc = "Register for reading and writing the FIFO."]
pub mod data8;
#[doc = "CTRL0 register accessor: an alias for `Reg<CTRL0_SPEC>`"]
pub type CTRL0 = crate::Reg<ctrl0::CTRL0_SPEC>;
#[doc = "Register for controlling SPI peripheral."]
pub mod ctrl0;
#[doc = "CTRL1 register accessor: an alias for `Reg<CTRL1_SPEC>`"]
pub type CTRL1 = crate::Reg<ctrl1::CTRL1_SPEC>;
#[doc = "Register for controlling SPI peripheral."]
pub mod ctrl1;
#[doc = "CTRL2 register accessor: an alias for `Reg<CTRL2_SPEC>`"]
pub type CTRL2 = crate::Reg<ctrl2::CTRL2_SPEC>;
#[doc = "Register for controlling SPI peripheral."]
pub mod ctrl2;
#[doc = "SS_TIME register accessor: an alias for `Reg<SS_TIME_SPEC>`"]
pub type SS_TIME = crate::Reg<ss_time::SS_TIME_SPEC>;
#[doc = "Register for controlling SPI peripheral/Slave Select Timing."]
pub mod ss_time;
#[doc = "CLK_CFG register accessor: an alias for `Reg<CLK_CFG_SPEC>`"]
pub type CLK_CFG = crate::Reg<clk_cfg::CLK_CFG_SPEC>;
#[doc = "Register for controlling SPI clock rate."]
pub mod clk_cfg;
#[doc = "DMA register accessor: an alias for `Reg<DMA_SPEC>`"]
pub type DMA = crate::Reg<dma::DMA_SPEC>;
#[doc = "Register for controlling DMA."]
pub mod dma;
#[doc = "INT_FL register accessor: an alias for `Reg<INT_FL_SPEC>`"]
pub type INT_FL = crate::Reg<int_fl::INT_FL_SPEC>;
#[doc = "Register for reading and clearing interrupt flags. All bits are write 1 to clear."]
pub mod int_fl;
#[doc = "INT_EN register accessor: an alias for `Reg<INT_EN_SPEC>`"]
pub type INT_EN = crate::Reg<int_en::INT_EN_SPEC>;
#[doc = "Register for enabling interrupts."]
pub mod int_en;
#[doc = "WAKE_FL register accessor: an alias for `Reg<WAKE_FL_SPEC>`"]
pub type WAKE_FL = crate::Reg<wake_fl::WAKE_FL_SPEC>;
#[doc = "Register for wake up flags. All bits in this register are write 1 to clear."]
pub mod wake_fl;
#[doc = "WAKE_EN register accessor: an alias for `Reg<WAKE_EN_SPEC>`"]
pub type WAKE_EN = crate::Reg<wake_en::WAKE_EN_SPEC>;
#[doc = "Register for wake up enable."]
pub mod wake_en;
#[doc = "STAT register accessor: an alias for `Reg<STAT_SPEC>`"]
pub type STAT = crate::Reg<stat::STAT_SPEC>;
#[doc = "SPI Status register."]
pub mod stat;
