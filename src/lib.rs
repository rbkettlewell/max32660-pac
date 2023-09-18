#![doc = "Peripheral access API for MAX32660 microcontrollers (generated using svd2rust v0.21.0 ( ))\n\nYou can find an overview of the generated API [here].\n\nAPI features to be included in the [next]
svd2rust release can be generated by cloning the svd2rust [repository], checking out the above commit, and running `cargo doc --open`.\n\n[here]: https://docs.rs/svd2rust/0.21.0/svd2rust/#peripheral-api\n[next]: https://github.com/rust-embedded/svd2rust/blob/master/CHANGELOG.md#unreleased\n[repository]: https://github.com/rust-embedded/svd2rust"]
#![deny(const_err)]
#![deny(dead_code)]
#![deny(improper_ctypes)]
#![deny(missing_docs)]
#![deny(no_mangle_generic_items)]
#![deny(non_shorthand_field_patterns)]
#![deny(overflowing_literals)]
#![deny(path_statements)]
#![deny(patterns_in_fns_without_body)]
#![deny(private_in_public)]
#![deny(unconditional_recursion)]
#![deny(unused_allocation)]
#![deny(unused_comparisons)]
#![deny(unused_parens)]
#![deny(while_true)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]
use core::marker::PhantomData;
use core::ops::Deref;
#[doc = r"Number available in the NVIC for configuring priority"]
pub const NVIC_PRIO_BITS: u8 = 3;
#[cfg(feature = "rt")]
pub use self::Interrupt as interrupt;
pub use cortex_m::peripheral::Peripherals as CorePeripherals;
pub use cortex_m::peripheral::{CBP, CPUID, DCB, DWT, FPB, FPU, ITM, MPU, NVIC, SCB, SYST, TPIU};
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[allow(unused_imports)]
use generic::*;
#[doc = r"Common register and bit access and modify traits"]
pub mod generic;
#[cfg(feature = "rt")]
extern "C" {
    fn WDT0();
    fn RTC();
    fn TMR0();
    fn TMR1();
    fn TMR2();
    fn I2C0();
    fn UART0();
    fn UART1();
    fn SPI0();
    fn FLASH_CONTROLLER();
    fn GPIO0();
    fn DMA0();
    fn DMA1();
    fn DMA2();
    fn DMA3();
    fn I2C1();
}
#[doc(hidden)]
pub union Vector {
    _handler: unsafe extern "C" fn(),
    _reserved: u32,
}
#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static __INTERRUPTS: [Vector; 37] = [
    Vector { _reserved: 0 },
    Vector { _handler: WDT0 },
    Vector { _reserved: 0 },
    Vector { _handler: RTC },
    Vector { _reserved: 0 },
    Vector { _handler: TMR0 },
    Vector { _handler: TMR1 },
    Vector { _handler: TMR2 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: I2C0 },
    Vector { _handler: UART0 },
    Vector { _handler: UART1 },
    Vector { _handler: SPI0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector {
        _handler: FLASH_CONTROLLER,
    },
    Vector { _handler: GPIO0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: DMA0 },
    Vector { _handler: DMA1 },
    Vector { _handler: DMA2 },
    Vector { _handler: DMA3 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: I2C1 },
];
#[doc = r"Enumeration of all the interrupts."]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Interrupt {
    #[doc = "1 - WDT0"]
    WDT0 = 1,
    #[doc = "3 - RTC interrupt."]
    RTC = 3,
    #[doc = "5 - TMR0 IRQ"]
    TMR0 = 5,
    #[doc = "6 - TMR1 IRQ"]
    TMR1 = 6,
    #[doc = "7 - TMR2 IRQ"]
    TMR2 = 7,
    #[doc = "13 - I2C0 IRQ"]
    I2C0 = 13,
    #[doc = "14 - UART0 IRQ"]
    UART0 = 14,
    #[doc = "15 - UART1 IRQ"]
    UART1 = 15,
    #[doc = "16 - SPI0"]
    SPI0 = 16,
    #[doc = "23 - Flash Controller interrupt."]
    FLASH_CONTROLLER = 23,
    #[doc = "24 - GPIO0 interrupt."]
    GPIO0 = 24,
    #[doc = "28 - DMA0"]
    DMA0 = 28,
    #[doc = "29 - DMA1"]
    DMA1 = 29,
    #[doc = "30 - DMA2"]
    DMA2 = 30,
    #[doc = "31 - DMA3"]
    DMA3 = 31,
    #[doc = "36 - I2C1 IRQ"]
    I2C1 = 36,
}
unsafe impl cortex_m::interrupt::InterruptNumber for Interrupt {
    #[inline(always)]
    fn number(self) -> u16 {
        self as u16
    }
}
#[doc = "Battery-Backed Function Control."]
pub struct BBFC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for BBFC {}
impl BBFC {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const bbfc::RegisterBlock = 0x4000_5800 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const bbfc::RegisterBlock {
        Self::PTR
    }
}
impl Deref for BBFC {
    type Target = bbfc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for BBFC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BBFC").finish()
    }
}
#[doc = "Battery-Backed Function Control."]
pub mod bbfc;
#[doc = "Battery-Backed Registers."]
pub struct BBSIR {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for BBSIR {}
impl BBSIR {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const bbsir::RegisterBlock = 0x4000_5400 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const bbsir::RegisterBlock {
        Self::PTR
    }
}
impl Deref for BBSIR {
    type Target = bbsir::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for BBSIR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BBSIR").finish()
    }
}
#[doc = "Battery-Backed Registers."]
pub mod bbsir;
#[doc = "DMA Controller Fully programmable, chaining capable DMA channels."]
pub struct DMA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DMA {}
impl DMA {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const dma::RegisterBlock = 0x4002_8000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dma::RegisterBlock {
        Self::PTR
    }
}
impl Deref for DMA {
    type Target = dma::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for DMA {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA").finish()
    }
}
#[doc = "DMA Controller Fully programmable, chaining capable DMA channels."]
pub mod dma;
#[doc = "Flash Memory Control."]
pub struct FLC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FLC {}
impl FLC {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const flc::RegisterBlock = 0x4002_9000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const flc::RegisterBlock {
        Self::PTR
    }
}
impl Deref for FLC {
    type Target = flc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for FLC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLC").finish()
    }
}
#[doc = "Flash Memory Control."]
pub mod flc;
#[doc = "Global Control Registers."]
pub struct GCR {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GCR {}
impl GCR {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const gcr::RegisterBlock = 0x4000_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gcr::RegisterBlock {
        Self::PTR
    }
}
impl Deref for GCR {
    type Target = gcr::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for GCR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GCR").finish()
    }
}
#[doc = "Global Control Registers."]
pub mod gcr;
#[doc = "Individual I/O for each GPIO"]
pub struct GPIO0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIO0 {}
impl GPIO0 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const gpio0::RegisterBlock = 0x4000_8000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpio0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for GPIO0 {
    type Target = gpio0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for GPIO0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIO0").finish()
    }
}
#[doc = "Individual I/O for each GPIO"]
pub mod gpio0;
#[doc = "Inter-Integrated Circuit."]
pub struct I2C0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C0 {}
impl I2C0 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const i2c0::RegisterBlock = 0x4001_d000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2c0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for I2C0 {
    type Target = i2c0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for I2C0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2C0").finish()
    }
}
#[doc = "Inter-Integrated Circuit."]
pub mod i2c0;
#[doc = "Inter-Integrated Circuit. 1"]
pub struct I2C1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C1 {}
impl I2C1 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const i2c0::RegisterBlock = 0x4001_e000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2c0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for I2C1 {
    type Target = i2c0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for I2C1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2C1").finish()
    }
}
#[doc = "Inter-Integrated Circuit. 1"]
pub use i2c0 as i2c1;
#[doc = "Instruction Cache Controller Registers"]
pub struct ICC0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ICC0 {}
impl ICC0 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const icc0::RegisterBlock = 0x4002_a000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const icc0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for ICC0 {
    type Target = icc0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for ICC0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICC0").finish()
    }
}
#[doc = "Instruction Cache Controller Registers"]
pub mod icc0;
#[doc = "Instruction Cache Controller Registers 1"]
pub struct ICC1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ICC1 {}
impl ICC1 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const icc0::RegisterBlock = 0x4002_f000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const icc0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for ICC1 {
    type Target = icc0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for ICC1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICC1").finish()
    }
}
#[doc = "Instruction Cache Controller Registers 1"]
pub use icc0 as icc1;
#[doc = "Power Sequencer / Low Power Control Register."]
pub struct PWRSEQ {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PWRSEQ {}
impl PWRSEQ {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const pwrseq::RegisterBlock = 0x4000_6800 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pwrseq::RegisterBlock {
        Self::PTR
    }
}
impl Deref for PWRSEQ {
    type Target = pwrseq::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for PWRSEQ {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWRSEQ").finish()
    }
}
#[doc = "Power Sequencer / Low Power Control Register."]
pub mod pwrseq;
#[doc = "Real Time Clock and Alarm."]
pub struct RTC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTC {}
impl RTC {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const rtc::RegisterBlock = 0x4000_6000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rtc::RegisterBlock {
        Self::PTR
    }
}
impl Deref for RTC {
    type Target = rtc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for RTC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RTC").finish()
    }
}
#[doc = "Real Time Clock and Alarm."]
pub mod rtc;
#[doc = "System Initialization Registers."]
pub struct SIR {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SIR {}
impl SIR {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const sir::RegisterBlock = 0x4000_0400 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sir::RegisterBlock {
        Self::PTR
    }
}
impl Deref for SIR {
    type Target = sir::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for SIR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SIR").finish()
    }
}
#[doc = "System Initialization Registers."]
pub mod sir;
#[doc = "The Security Monitor block used to monitor system threat conditions."]
pub struct SMON {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SMON {}
impl SMON {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const smon::RegisterBlock = 0x4000_4000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const smon::RegisterBlock {
        Self::PTR
    }
}
impl Deref for SMON {
    type Target = smon::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for SMON {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMON").finish()
    }
}
#[doc = "The Security Monitor block used to monitor system threat conditions."]
pub mod smon;
#[doc = "SPI peripheral."]
pub struct SPI17Y {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI17Y {}
impl SPI17Y {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const spi17y::RegisterBlock = 0x4004_6000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spi17y::RegisterBlock {
        Self::PTR
    }
}
impl Deref for SPI17Y {
    type Target = spi17y::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for SPI17Y {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI17Y").finish()
    }
}
#[doc = "SPI peripheral."]
pub mod spi17y;
#[doc = "Serial Peripheral Interface."]
pub struct SPIMSS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPIMSS {}
impl SPIMSS {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const spimss::RegisterBlock = 0x4001_9000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spimss::RegisterBlock {
        Self::PTR
    }
}
impl Deref for SPIMSS {
    type Target = spimss::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for SPIMSS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPIMSS").finish()
    }
}
#[doc = "Serial Peripheral Interface."]
pub mod spimss;
#[doc = "32-bit reloadable timer that can be used for timing and event counting."]
pub struct TMR0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TMR0 {}
impl TMR0 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const tmr0::RegisterBlock = 0x4001_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tmr0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for TMR0 {
    type Target = tmr0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for TMR0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TMR0").finish()
    }
}
#[doc = "32-bit reloadable timer that can be used for timing and event counting."]
pub mod tmr0;
#[doc = "32-bit reloadable timer that can be used for timing and event counting. 1"]
pub struct TMR1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TMR1 {}
impl TMR1 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const tmr0::RegisterBlock = 0x4001_1000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tmr0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for TMR1 {
    type Target = tmr0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for TMR1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TMR1").finish()
    }
}
#[doc = "32-bit reloadable timer that can be used for timing and event counting. 1"]
pub use tmr0 as tmr1;
#[doc = "32-bit reloadable timer that can be used for timing and event counting. 2"]
pub struct TMR2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TMR2 {}
impl TMR2 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const tmr0::RegisterBlock = 0x4001_2000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tmr0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for TMR2 {
    type Target = tmr0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for TMR2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TMR2").finish()
    }
}
#[doc = "32-bit reloadable timer that can be used for timing and event counting. 2"]
pub use tmr0 as tmr2;
#[doc = "UART"]
pub struct UART0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART0 {}
impl UART0 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const uart0::RegisterBlock = 0x4004_2000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uart0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for UART0 {
    type Target = uart0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for UART0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UART0").finish()
    }
}
#[doc = "UART"]
pub mod uart0;
#[doc = "UART 1"]
pub struct UART1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART1 {}
impl UART1 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const uart0::RegisterBlock = 0x4004_3000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uart0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for UART1 {
    type Target = uart0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for UART1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UART1").finish()
    }
}
#[doc = "UART 1"]
pub use uart0 as uart1;
#[doc = "Watchdog Timer 0"]
pub struct WDT0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WDT0 {}
impl WDT0 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const wdt0::RegisterBlock = 0x4000_3000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const wdt0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for WDT0 {
    type Target = wdt0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for WDT0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WDT0").finish()
    }
}
#[doc = "Watchdog Timer 0"]
pub mod wdt0;
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r"All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "BBFC"]
    pub BBFC: BBFC,
    #[doc = "BBSIR"]
    pub BBSIR: BBSIR,
    #[doc = "DMA"]
    pub DMA: DMA,
    #[doc = "FLC"]
    pub FLC: FLC,
    #[doc = "GCR"]
    pub GCR: GCR,
    #[doc = "GPIO0"]
    pub GPIO0: GPIO0,
    #[doc = "I2C0"]
    pub I2C0: I2C0,
    #[doc = "I2C1"]
    pub I2C1: I2C1,
    #[doc = "ICC0"]
    pub ICC0: ICC0,
    #[doc = "ICC1"]
    pub ICC1: ICC1,
    #[doc = "PWRSEQ"]
    pub PWRSEQ: PWRSEQ,
    #[doc = "RTC"]
    pub RTC: RTC,
    #[doc = "SIR"]
    pub SIR: SIR,
    #[doc = "SMON"]
    pub SMON: SMON,
    #[doc = "SPI17Y"]
    pub SPI17Y: SPI17Y,
    #[doc = "SPIMSS"]
    pub SPIMSS: SPIMSS,
    #[doc = "TMR0"]
    pub TMR0: TMR0,
    #[doc = "TMR1"]
    pub TMR1: TMR1,
    #[doc = "TMR2"]
    pub TMR2: TMR2,
    #[doc = "UART0"]
    pub UART0: UART0,
    #[doc = "UART1"]
    pub UART1: UART1,
    #[doc = "WDT0"]
    pub WDT0: WDT0,
}
impl Peripherals {
    #[doc = r"Returns all the peripherals *once*"]
    #[inline]
    pub fn take() -> Option<Self> {
        cortex_m::interrupt::free(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                None
            } else {
                Some(unsafe { Peripherals::steal() })
            }
        })
    }
    #[doc = r"Unchecked version of `Peripherals::take`"]
    #[inline]
    pub unsafe fn steal() -> Self {
        DEVICE_PERIPHERALS = true;
        Peripherals {
            BBFC: BBFC {
                _marker: PhantomData,
            },
            BBSIR: BBSIR {
                _marker: PhantomData,
            },
            DMA: DMA {
                _marker: PhantomData,
            },
            FLC: FLC {
                _marker: PhantomData,
            },
            GCR: GCR {
                _marker: PhantomData,
            },
            GPIO0: GPIO0 {
                _marker: PhantomData,
            },
            I2C0: I2C0 {
                _marker: PhantomData,
            },
            I2C1: I2C1 {
                _marker: PhantomData,
            },
            ICC0: ICC0 {
                _marker: PhantomData,
            },
            ICC1: ICC1 {
                _marker: PhantomData,
            },
            PWRSEQ: PWRSEQ {
                _marker: PhantomData,
            },
            RTC: RTC {
                _marker: PhantomData,
            },
            SIR: SIR {
                _marker: PhantomData,
            },
            SMON: SMON {
                _marker: PhantomData,
            },
            SPI17Y: SPI17Y {
                _marker: PhantomData,
            },
            SPIMSS: SPIMSS {
                _marker: PhantomData,
            },
            TMR0: TMR0 {
                _marker: PhantomData,
            },
            TMR1: TMR1 {
                _marker: PhantomData,
            },
            TMR2: TMR2 {
                _marker: PhantomData,
            },
            UART0: UART0 {
                _marker: PhantomData,
            },
            UART1: UART1 {
                _marker: PhantomData,
            },
            WDT0: WDT0 {
                _marker: PhantomData,
            },
        }
    }
}
