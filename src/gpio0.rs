#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GPIO Function Enable Register. Each bit controls the GPIO_EN setting for one GPIO pin on the associated port."]
    pub en: crate::Reg<en::EN_SPEC>,
    #[doc = "0x04 - GPIO Set Function Enable Register. Writing a 1 to one or more bits in this register sets the bits in the same positions in GPIO_EN to 1, without affecting other bits in that register."]
    pub en_set: crate::Reg<en_set::EN_SET_SPEC>,
    #[doc = "0x08 - GPIO Clear Function Enable Register. Writing a 1 to one or more bits in this register clears the bits in the same positions in GPIO_EN to 0, without affecting other bits in that register."]
    pub en_clr: crate::Reg<en_clr::EN_CLR_SPEC>,
    #[doc = "0x0c - GPIO Output Enable Register. Each bit controls the GPIO_OUT_EN setting for one GPIO pin in the associated port."]
    pub out_en: crate::Reg<out_en::OUT_EN_SPEC>,
    #[doc = "0x10 - GPIO Output Enable Set Function Enable Register. Writing a 1 to one or more bits in this register sets the bits in the same positions in GPIO_OUT_EN to 1, without affecting other bits in that register."]
    pub out_en_set: crate::Reg<out_en_set::OUT_EN_SET_SPEC>,
    #[doc = "0x14 - GPIO Output Enable Clear Function Enable Register. Writing a 1 to one or more bits in this register clears the bits in the same positions in GPIO_OUT_EN to 0, without affecting other bits in that register."]
    pub out_en_clr: crate::Reg<out_en_clr::OUT_EN_CLR_SPEC>,
    #[doc = "0x18 - GPIO Output Register. Each bit controls the GPIO_OUT setting for one pin in the associated port. This register can be written either directly, or by using the GPIO_OUT_SET and GPIO_OUT_CLR registers."]
    pub out: crate::Reg<out::OUT_SPEC>,
    #[doc = "0x1c - GPIO Output Set. Writing a 1 to one or more bits in this register sets the bits in the same positions in GPIO_OUT to 1, without affecting other bits in that register."]
    pub out_set: crate::Reg<out_set::OUT_SET_SPEC>,
    #[doc = "0x20 - GPIO Output Clear. Writing a 1 to one or more bits in this register clears the bits in the same positions in GPIO_OUT to 0, without affecting other bits in that register."]
    pub out_clr: crate::Reg<out_clr::OUT_CLR_SPEC>,
    #[doc = "0x24 - GPIO Input Register. Read-only register to read from the logic states of the GPIO pins on this port."]
    pub in_: crate::Reg<in_::IN_SPEC>,
    #[doc = "0x28 - GPIO Interrupt Mode Register. Each bit in this register controls the interrupt mode setting for the associated GPIO pin on this port."]
    pub int_mod: crate::Reg<int_mod::INT_MOD_SPEC>,
    #[doc = "0x2c - GPIO Interrupt Polarity Register. Each bit in this register controls the interrupt polarity setting for one GPIO pin in the associated port."]
    pub int_pol: crate::Reg<int_pol::INT_POL_SPEC>,
    _reserved12: [u8; 0x04],
    #[doc = "0x34 - GPIO Interrupt Enable Register. Each bit in this register controls the GPIO interrupt enable for the associated pin on the GPIO port."]
    pub int_en: crate::Reg<int_en::INT_EN_SPEC>,
    #[doc = "0x38 - GPIO Interrupt Enable Set. Writing a 1 to one or more bits in this register sets the bits in the same positions in GPIO_INT_EN to 1, without affecting other bits in that register."]
    pub int_en_set: crate::Reg<int_en_set::INT_EN_SET_SPEC>,
    #[doc = "0x3c - GPIO Interrupt Enable Clear. Writing a 1 to one or more bits in this register clears the bits in the same positions in GPIO_INT_EN to 0, without affecting other bits in that register."]
    pub int_en_clr: crate::Reg<int_en_clr::INT_EN_CLR_SPEC>,
    #[doc = "0x40 - GPIO Interrupt Status Register. Each bit in this register contains the pending interrupt status for the associated GPIO pin in this port."]
    pub int_stat: crate::Reg<int_stat::INT_STAT_SPEC>,
    _reserved16: [u8; 0x04],
    #[doc = "0x48 - GPIO Status Clear. Writing a 1 to one or more bits in this register clears the bits in the same positions in GPIO_INT_STAT to 0, without affecting other bits in that register."]
    pub int_clr: crate::Reg<int_clr::INT_CLR_SPEC>,
    #[doc = "0x4c - GPIO Wake Enable Register. Each bit in this register controls the PMU wakeup enable for the associated GPIO pin in this port."]
    pub wake_en: crate::Reg<wake_en::WAKE_EN_SPEC>,
    #[doc = "0x50 - GPIO Wake Enable Set. Writing a 1 to one or more bits in this register sets the bits in the same positions in GPIO_WAKE_EN to 1, without affecting other bits in that register."]
    pub wake_en_set: crate::Reg<wake_en_set::WAKE_EN_SET_SPEC>,
    #[doc = "0x54 - GPIO Wake Enable Clear. Writing a 1 to one or more bits in this register clears the bits in the same positions in GPIO_WAKE_EN to 0, without affecting other bits in that register."]
    pub wake_en_clr: crate::Reg<wake_en_clr::WAKE_EN_CLR_SPEC>,
    _reserved20: [u8; 0x04],
    #[doc = "0x5c - GPIO Interrupt Dual Edge Mode Register. Each bit in this register selects dual edge mode for the associated GPIO pin in this port."]
    pub int_dual_edge: crate::Reg<int_dual_edge::INT_DUAL_EDGE_SPEC>,
    #[doc = "0x60 - GPIO Input Mode Config 1. Each bit in this register enables the weak pull-up for the associated GPIO pin in this port."]
    pub pad_cfg1: crate::Reg<pad_cfg1::PAD_CFG1_SPEC>,
    #[doc = "0x64 - GPIO Input Mode Config 2. Each bit in this register enables the weak pull-up for the associated GPIO pin in this port."]
    pub pad_cfg2: crate::Reg<pad_cfg2::PAD_CFG2_SPEC>,
    #[doc = "0x68 - GPIO Alternate Function Enable Register. Each bit in this register selects between primary/secondary functions for the associated GPIO pin in this port."]
    pub en1: crate::Reg<en1::EN1_SPEC>,
    #[doc = "0x6c - GPIO Alternate Function Set. Writing a 1 to one or more bits in this register sets the bits in the same positions in GPIO_EN1 to 1, without affecting other bits in that register."]
    pub en1_set: crate::Reg<en1_set::EN1_SET_SPEC>,
    #[doc = "0x70 - GPIO Alternate Function Clear. Writing a 1 to one or more bits in this register clears the bits in the same positions in GPIO_EN1 to 0, without affecting other bits in that register."]
    pub en1_clr: crate::Reg<en1_clr::EN1_CLR_SPEC>,
    #[doc = "0x74 - GPIO Alternate Function Enable Register. Each bit in this register selects between primary/secondary functions for the associated GPIO pin in this port."]
    pub en2: crate::Reg<en2::EN2_SPEC>,
    #[doc = "0x78 - GPIO Alternate Function 2 Set. Writing a 1 to one or more bits in this register sets the bits in the same positions in GPIO_EN2 to 1, without affecting other bits in that register."]
    pub en2_set: crate::Reg<en2_set::EN2_SET_SPEC>,
    #[doc = "0x7c - GPIO Wake Alternate Function Clear. Writing a 1 to one or more bits in this register clears the bits in the same positions in GPIO_EN2 to 0, without affecting other bits in that register."]
    pub en2_clr: crate::Reg<en2_clr::EN2_CLR_SPEC>,
    _reserved29: [u8; 0x28],
    #[doc = "0xa8 - Input Hysteresis Enable Register"]
    pub is: crate::Reg<is::IS_SPEC>,
    #[doc = "0xac - Slew Rate Select Register."]
    pub sr: crate::Reg<sr::SR_SPEC>,
    #[doc = "0xb0 - GPIO Drive Strength Register. Each bit in this register selects the drive strength for the associated GPIO pin in this port. Refer to the Datasheet for sink/source current of GPIO pins in each mode."]
    pub ds: crate::Reg<ds::DS_SPEC>,
    #[doc = "0xb4 - GPIO Drive Strength 1 Register. Each bit in this register selects the drive strength for the associated GPIO pin in this port. Refer to the Datasheet for sink/source current of GPIO pins in each mode."]
    pub ds1: crate::Reg<ds1::DS1_SPEC>,
    #[doc = "0xb8 - GPIO Pull Select Mode."]
    pub ps: crate::Reg<ps::PS_SPEC>,
    _reserved34: [u8; 0x04],
    #[doc = "0xc0 - GPIO Voltage Select."]
    pub vssel: crate::Reg<vssel::VSSEL_SPEC>,
}
#[doc = "EN register accessor: an alias for `Reg<EN_SPEC>`"]
pub type EN = crate::Reg<en::EN_SPEC>;
#[doc = "GPIO Function Enable Register. Each bit controls the GPIO_EN setting for one GPIO pin on the associated port."]
pub mod en;
#[doc = "EN_SET register accessor: an alias for `Reg<EN_SET_SPEC>`"]
pub type EN_SET = crate::Reg<en_set::EN_SET_SPEC>;
#[doc = "GPIO Set Function Enable Register. Writing a 1 to one or more bits in this register sets the bits in the same positions in GPIO_EN to 1, without affecting other bits in that register."]
pub mod en_set;
#[doc = "EN_CLR register accessor: an alias for `Reg<EN_CLR_SPEC>`"]
pub type EN_CLR = crate::Reg<en_clr::EN_CLR_SPEC>;
#[doc = "GPIO Clear Function Enable Register. Writing a 1 to one or more bits in this register clears the bits in the same positions in GPIO_EN to 0, without affecting other bits in that register."]
pub mod en_clr;
#[doc = "OUT_EN register accessor: an alias for `Reg<OUT_EN_SPEC>`"]
pub type OUT_EN = crate::Reg<out_en::OUT_EN_SPEC>;
#[doc = "GPIO Output Enable Register. Each bit controls the GPIO_OUT_EN setting for one GPIO pin in the associated port."]
pub mod out_en;
#[doc = "OUT_EN_SET register accessor: an alias for `Reg<OUT_EN_SET_SPEC>`"]
pub type OUT_EN_SET = crate::Reg<out_en_set::OUT_EN_SET_SPEC>;
#[doc = "GPIO Output Enable Set Function Enable Register. Writing a 1 to one or more bits in this register sets the bits in the same positions in GPIO_OUT_EN to 1, without affecting other bits in that register."]
pub mod out_en_set;
#[doc = "OUT_EN_CLR register accessor: an alias for `Reg<OUT_EN_CLR_SPEC>`"]
pub type OUT_EN_CLR = crate::Reg<out_en_clr::OUT_EN_CLR_SPEC>;
#[doc = "GPIO Output Enable Clear Function Enable Register. Writing a 1 to one or more bits in this register clears the bits in the same positions in GPIO_OUT_EN to 0, without affecting other bits in that register."]
pub mod out_en_clr;
#[doc = "OUT register accessor: an alias for `Reg<OUT_SPEC>`"]
pub type OUT = crate::Reg<out::OUT_SPEC>;
#[doc = "GPIO Output Register. Each bit controls the GPIO_OUT setting for one pin in the associated port. This register can be written either directly, or by using the GPIO_OUT_SET and GPIO_OUT_CLR registers."]
pub mod out;
#[doc = "OUT_SET register accessor: an alias for `Reg<OUT_SET_SPEC>`"]
pub type OUT_SET = crate::Reg<out_set::OUT_SET_SPEC>;
#[doc = "GPIO Output Set. Writing a 1 to one or more bits in this register sets the bits in the same positions in GPIO_OUT to 1, without affecting other bits in that register."]
pub mod out_set;
#[doc = "OUT_CLR register accessor: an alias for `Reg<OUT_CLR_SPEC>`"]
pub type OUT_CLR = crate::Reg<out_clr::OUT_CLR_SPEC>;
#[doc = "GPIO Output Clear. Writing a 1 to one or more bits in this register clears the bits in the same positions in GPIO_OUT to 0, without affecting other bits in that register."]
pub mod out_clr;
#[doc = "IN register accessor: an alias for `Reg<IN_SPEC>`"]
pub type IN = crate::Reg<in_::IN_SPEC>;
#[doc = "GPIO Input Register. Read-only register to read from the logic states of the GPIO pins on this port."]
pub mod in_;
#[doc = "INT_MOD register accessor: an alias for `Reg<INT_MOD_SPEC>`"]
pub type INT_MOD = crate::Reg<int_mod::INT_MOD_SPEC>;
#[doc = "GPIO Interrupt Mode Register. Each bit in this register controls the interrupt mode setting for the associated GPIO pin on this port."]
pub mod int_mod;
#[doc = "INT_POL register accessor: an alias for `Reg<INT_POL_SPEC>`"]
pub type INT_POL = crate::Reg<int_pol::INT_POL_SPEC>;
#[doc = "GPIO Interrupt Polarity Register. Each bit in this register controls the interrupt polarity setting for one GPIO pin in the associated port."]
pub mod int_pol;
#[doc = "INT_EN register accessor: an alias for `Reg<INT_EN_SPEC>`"]
pub type INT_EN = crate::Reg<int_en::INT_EN_SPEC>;
#[doc = "GPIO Interrupt Enable Register. Each bit in this register controls the GPIO interrupt enable for the associated pin on the GPIO port."]
pub mod int_en;
#[doc = "INT_EN_SET register accessor: an alias for `Reg<INT_EN_SET_SPEC>`"]
pub type INT_EN_SET = crate::Reg<int_en_set::INT_EN_SET_SPEC>;
#[doc = "GPIO Interrupt Enable Set. Writing a 1 to one or more bits in this register sets the bits in the same positions in GPIO_INT_EN to 1, without affecting other bits in that register."]
pub mod int_en_set;
#[doc = "INT_EN_CLR register accessor: an alias for `Reg<INT_EN_CLR_SPEC>`"]
pub type INT_EN_CLR = crate::Reg<int_en_clr::INT_EN_CLR_SPEC>;
#[doc = "GPIO Interrupt Enable Clear. Writing a 1 to one or more bits in this register clears the bits in the same positions in GPIO_INT_EN to 0, without affecting other bits in that register."]
pub mod int_en_clr;
#[doc = "INT_STAT register accessor: an alias for `Reg<INT_STAT_SPEC>`"]
pub type INT_STAT = crate::Reg<int_stat::INT_STAT_SPEC>;
#[doc = "GPIO Interrupt Status Register. Each bit in this register contains the pending interrupt status for the associated GPIO pin in this port."]
pub mod int_stat;
#[doc = "INT_CLR register accessor: an alias for `Reg<INT_CLR_SPEC>`"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = "GPIO Status Clear. Writing a 1 to one or more bits in this register clears the bits in the same positions in GPIO_INT_STAT to 0, without affecting other bits in that register."]
pub mod int_clr;
#[doc = "WAKE_EN register accessor: an alias for `Reg<WAKE_EN_SPEC>`"]
pub type WAKE_EN = crate::Reg<wake_en::WAKE_EN_SPEC>;
#[doc = "GPIO Wake Enable Register. Each bit in this register controls the PMU wakeup enable for the associated GPIO pin in this port."]
pub mod wake_en;
#[doc = "WAKE_EN_SET register accessor: an alias for `Reg<WAKE_EN_SET_SPEC>`"]
pub type WAKE_EN_SET = crate::Reg<wake_en_set::WAKE_EN_SET_SPEC>;
#[doc = "GPIO Wake Enable Set. Writing a 1 to one or more bits in this register sets the bits in the same positions in GPIO_WAKE_EN to 1, without affecting other bits in that register."]
pub mod wake_en_set;
#[doc = "WAKE_EN_CLR register accessor: an alias for `Reg<WAKE_EN_CLR_SPEC>`"]
pub type WAKE_EN_CLR = crate::Reg<wake_en_clr::WAKE_EN_CLR_SPEC>;
#[doc = "GPIO Wake Enable Clear. Writing a 1 to one or more bits in this register clears the bits in the same positions in GPIO_WAKE_EN to 0, without affecting other bits in that register."]
pub mod wake_en_clr;
#[doc = "INT_DUAL_EDGE register accessor: an alias for `Reg<INT_DUAL_EDGE_SPEC>`"]
pub type INT_DUAL_EDGE = crate::Reg<int_dual_edge::INT_DUAL_EDGE_SPEC>;
#[doc = "GPIO Interrupt Dual Edge Mode Register. Each bit in this register selects dual edge mode for the associated GPIO pin in this port."]
pub mod int_dual_edge;
#[doc = "PAD_CFG1 register accessor: an alias for `Reg<PAD_CFG1_SPEC>`"]
pub type PAD_CFG1 = crate::Reg<pad_cfg1::PAD_CFG1_SPEC>;
#[doc = "GPIO Input Mode Config 1. Each bit in this register enables the weak pull-up for the associated GPIO pin in this port."]
pub mod pad_cfg1;
#[doc = "PAD_CFG2 register accessor: an alias for `Reg<PAD_CFG2_SPEC>`"]
pub type PAD_CFG2 = crate::Reg<pad_cfg2::PAD_CFG2_SPEC>;
#[doc = "GPIO Input Mode Config 2. Each bit in this register enables the weak pull-up for the associated GPIO pin in this port."]
pub mod pad_cfg2;
#[doc = "EN1 register accessor: an alias for `Reg<EN1_SPEC>`"]
pub type EN1 = crate::Reg<en1::EN1_SPEC>;
#[doc = "GPIO Alternate Function Enable Register. Each bit in this register selects between primary/secondary functions for the associated GPIO pin in this port."]
pub mod en1;
#[doc = "EN1_SET register accessor: an alias for `Reg<EN1_SET_SPEC>`"]
pub type EN1_SET = crate::Reg<en1_set::EN1_SET_SPEC>;
#[doc = "GPIO Alternate Function Set. Writing a 1 to one or more bits in this register sets the bits in the same positions in GPIO_EN1 to 1, without affecting other bits in that register."]
pub mod en1_set;
#[doc = "EN1_CLR register accessor: an alias for `Reg<EN1_CLR_SPEC>`"]
pub type EN1_CLR = crate::Reg<en1_clr::EN1_CLR_SPEC>;
#[doc = "GPIO Alternate Function Clear. Writing a 1 to one or more bits in this register clears the bits in the same positions in GPIO_EN1 to 0, without affecting other bits in that register."]
pub mod en1_clr;
#[doc = "EN2 register accessor: an alias for `Reg<EN2_SPEC>`"]
pub type EN2 = crate::Reg<en2::EN2_SPEC>;
#[doc = "GPIO Alternate Function Enable Register. Each bit in this register selects between primary/secondary functions for the associated GPIO pin in this port."]
pub mod en2;
#[doc = "EN2_SET register accessor: an alias for `Reg<EN2_SET_SPEC>`"]
pub type EN2_SET = crate::Reg<en2_set::EN2_SET_SPEC>;
#[doc = "GPIO Alternate Function 2 Set. Writing a 1 to one or more bits in this register sets the bits in the same positions in GPIO_EN2 to 1, without affecting other bits in that register."]
pub mod en2_set;
#[doc = "EN2_CLR register accessor: an alias for `Reg<EN2_CLR_SPEC>`"]
pub type EN2_CLR = crate::Reg<en2_clr::EN2_CLR_SPEC>;
#[doc = "GPIO Wake Alternate Function Clear. Writing a 1 to one or more bits in this register clears the bits in the same positions in GPIO_EN2 to 0, without affecting other bits in that register."]
pub mod en2_clr;
#[doc = "IS register accessor: an alias for `Reg<IS_SPEC>`"]
pub type IS = crate::Reg<is::IS_SPEC>;
#[doc = "Input Hysteresis Enable Register"]
pub mod is;
#[doc = "SR register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "Slew Rate Select Register."]
pub mod sr;
#[doc = "DS register accessor: an alias for `Reg<DS_SPEC>`"]
pub type DS = crate::Reg<ds::DS_SPEC>;
#[doc = "GPIO Drive Strength Register. Each bit in this register selects the drive strength for the associated GPIO pin in this port. Refer to the Datasheet for sink/source current of GPIO pins in each mode."]
pub mod ds;
#[doc = "DS1 register accessor: an alias for `Reg<DS1_SPEC>`"]
pub type DS1 = crate::Reg<ds1::DS1_SPEC>;
#[doc = "GPIO Drive Strength 1 Register. Each bit in this register selects the drive strength for the associated GPIO pin in this port. Refer to the Datasheet for sink/source current of GPIO pins in each mode."]
pub mod ds1;
#[doc = "PS register accessor: an alias for `Reg<PS_SPEC>`"]
pub type PS = crate::Reg<ps::PS_SPEC>;
#[doc = "GPIO Pull Select Mode."]
pub mod ps;
#[doc = "VSSEL register accessor: an alias for `Reg<VSSEL_SPEC>`"]
pub type VSSEL = crate::Reg<vssel::VSSEL_SPEC>;
#[doc = "GPIO Voltage Select."]
pub mod vssel;
