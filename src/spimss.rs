#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_data8: [u8; 0x02],
    _reserved1: [u8; 0x02],
    #[doc = "0x04 - SPI Control Register."]
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    #[doc = "0x08 - SPI Status Register."]
    pub status: crate::Reg<status::STATUS_SPEC>,
    #[doc = "0x0c - SPI Mode Register."]
    pub mod_: crate::Reg<mod_::MOD_SPEC>,
    _reserved4: [u8; 0x04],
    #[doc = "0x14 - Baud Rate Reload Value. The SPI Baud Rate register is a 16-bit reload value for the SPI Baud Rate Generator. The reload value must be greater than or equal to 0002H for proper SPI operation (maximum baud rate is PCLK frequency divided by 4)."]
    pub brg: crate::Reg<brg::BRG_SPEC>,
    #[doc = "0x18 - SPI DMA Register."]
    pub dma: crate::Reg<dma::DMA_SPEC>,
    #[doc = "0x1c - I2S Control Register."]
    pub i2s_ctrl: crate::Reg<i2s_ctrl::I2S_CTRL_SPEC>,
}
impl RegisterBlock {
    #[doc = "0x00 - SPI Data 8-bit access"]
    #[inline(always)]
    pub fn data8(&self) -> &[crate::Reg<data8::DATA8_SPEC>; 2] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(0usize)
                as *const [crate::Reg<data8::DATA8_SPEC>; 2])
        }
    }
    #[doc = "0x00 - SPI 16-bit Data Access"]
    #[inline(always)]
    pub fn data16(&self) -> &crate::Reg<data16::DATA16_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(0usize)
                as *const crate::Reg<data16::DATA16_SPEC>)
        }
    }
}
#[doc = "DATA16 register accessor: an alias for `Reg<DATA16_SPEC>`"]
pub type DATA16 = crate::Reg<data16::DATA16_SPEC>;
#[doc = "SPI 16-bit Data Access"]
pub mod data16;
#[doc = "DATA8 register accessor: an alias for `Reg<DATA8_SPEC>`"]
pub type DATA8 = crate::Reg<data8::DATA8_SPEC>;
#[doc = "SPI Data 8-bit access"]
pub mod data8;
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "SPI Control Register."]
pub mod ctrl;
#[doc = "STATUS register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "SPI Status Register."]
pub mod status;
#[doc = "MOD register accessor: an alias for `Reg<MOD_SPEC>`"]
pub type MOD = crate::Reg<mod_::MOD_SPEC>;
#[doc = "SPI Mode Register."]
pub mod mod_;
#[doc = "BRG register accessor: an alias for `Reg<BRG_SPEC>`"]
pub type BRG = crate::Reg<brg::BRG_SPEC>;
#[doc = "Baud Rate Reload Value. The SPI Baud Rate register is a 16-bit reload value for the SPI Baud Rate Generator. The reload value must be greater than or equal to 0002H for proper SPI operation (maximum baud rate is PCLK frequency divided by 4)."]
pub mod brg;
#[doc = "DMA register accessor: an alias for `Reg<DMA_SPEC>`"]
pub type DMA = crate::Reg<dma::DMA_SPEC>;
#[doc = "SPI DMA Register."]
pub mod dma;
#[doc = "I2S_CTRL register accessor: an alias for `Reg<I2S_CTRL_SPEC>`"]
pub type I2S_CTRL = crate::Reg<i2s_ctrl::I2S_CTRL_SPEC>;
#[doc = "I2S Control Register."]
pub mod i2s_ctrl;
