#![doc = "Peripheral access API for APOLLO1 microcontrollers (generated using svd2rust v0.17.0)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.17.0/svd2rust/#peripheral-api"]
#![deny(const_err)]
#![deny(dead_code)]
#![deny(improper_ctypes)]
#![deny(legacy_directory_ownership)]
#![deny(missing_docs)]
#![deny(no_mangle_generic_items)]
#![deny(non_shorthand_field_patterns)]
#![deny(overflowing_literals)]
#![deny(path_statements)]
#![deny(patterns_in_fns_without_body)]
#![deny(plugin_as_library)]
#![deny(private_in_public)]
#![deny(safe_extern_statics)]
#![deny(unconditional_recursion)]
#![deny(unions_with_drop_fields)]
#![deny(unused_allocation)]
#![deny(unused_comparisons)]
#![deny(unused_parens)]
#![deny(while_true)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]
extern crate bare_metal;
extern crate cortex_m;
#[cfg(feature = "rt")]
extern crate cortex_m_rt;
extern crate vcell;
use core::marker::PhantomData;
use core::ops::Deref;
#[doc = r"Number available in the NVIC for configuring priority"]
pub const NVIC_PRIO_BITS: u8 = 3;
#[cfg(feature = "rt")]
extern "C" {
    fn BROWNOUT();
    fn WDT();
    fn CLKGEN_RTC();
    fn VCOMP();
    fn IOSLAVE();
    fn IOSLAVEACC();
    fn IOMSTR0();
    fn IOMSTR1();
    fn ADC();
    fn GPIO();
    fn CTIMER();
    fn UART();
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
pub static __INTERRUPTS: [Vector; 12] = [
    Vector { _handler: BROWNOUT },
    Vector { _handler: WDT },
    Vector {
        _handler: CLKGEN_RTC,
    },
    Vector { _handler: VCOMP },
    Vector { _handler: IOSLAVE },
    Vector {
        _handler: IOSLAVEACC,
    },
    Vector { _handler: IOMSTR0 },
    Vector { _handler: IOMSTR1 },
    Vector { _handler: ADC },
    Vector { _handler: GPIO },
    Vector { _handler: CTIMER },
    Vector { _handler: UART },
];
#[doc = r"Enumeration of all the interrupts"]
#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum Interrupt {
    #[doc = "0 - BROWNOUT"]
    BROWNOUT = 0,
    #[doc = "1 - WDT"]
    WDT = 1,
    #[doc = "2 - CLKGEN_RTC"]
    CLKGEN_RTC = 2,
    #[doc = "3 - VCOMP"]
    VCOMP = 3,
    #[doc = "4 - IOSLAVE"]
    IOSLAVE = 4,
    #[doc = "5 - IOSLAVEACC"]
    IOSLAVEACC = 5,
    #[doc = "6 - IOMSTR0"]
    IOMSTR0 = 6,
    #[doc = "7 - IOMSTR1"]
    IOMSTR1 = 7,
    #[doc = "8 - ADC"]
    ADC = 8,
    #[doc = "9 - GPIO"]
    GPIO = 9,
    #[doc = "10 - CTIMER"]
    CTIMER = 10,
    #[doc = "11 - UART"]
    UART = 11,
}
unsafe impl bare_metal::Nr for Interrupt {
    #[inline(always)]
    fn nr(&self) -> u8 {
        *self as u8
    }
}
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
#[doc = "Analog Digital Converter Control"]
pub struct ADC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC {}
impl ADC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const adc::RegisterBlock {
        0x5000_8000 as *const _
    }
}
impl Deref for ADC {
    type Target = adc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*ADC::ptr() }
    }
}
#[doc = "Analog Digital Converter Control"]
pub mod adc;
#[doc = "Clock Generator"]
pub struct CLKGEN {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CLKGEN {}
impl CLKGEN {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const clkgen::RegisterBlock {
        0x4000_4000 as *const _
    }
}
impl Deref for CLKGEN {
    type Target = clkgen::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CLKGEN::ptr() }
    }
}
#[doc = "Clock Generator"]
pub mod clkgen;
#[doc = "Counter/Timer"]
pub struct CTIMER {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CTIMER {}
impl CTIMER {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ctimer::RegisterBlock {
        0x4000_8000 as *const _
    }
}
impl Deref for CTIMER {
    type Target = ctimer::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CTIMER::ptr() }
    }
}
#[doc = "Counter/Timer"]
pub mod ctimer;
#[doc = "General Purpose IO"]
pub struct GPIO {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIO {}
impl GPIO {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpio::RegisterBlock {
        0x4001_0000 as *const _
    }
}
impl Deref for GPIO {
    type Target = gpio::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIO::ptr() }
    }
}
#[doc = "General Purpose IO"]
pub mod gpio;
#[doc = "I2C/SPI Master"]
pub struct IOMSTR0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for IOMSTR0 {}
impl IOMSTR0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const iomstr0::RegisterBlock {
        0x5000_4000 as *const _
    }
}
impl Deref for IOMSTR0 {
    type Target = iomstr0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*IOMSTR0::ptr() }
    }
}
#[doc = "I2C/SPI Master"]
pub mod iomstr0;
#[doc = "I2C/SPI Master"]
pub struct IOMSTR1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for IOMSTR1 {}
impl IOMSTR1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const iomstr0::RegisterBlock {
        0x5000_5000 as *const _
    }
}
impl Deref for IOMSTR1 {
    type Target = iomstr0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*IOMSTR1::ptr() }
    }
}
#[doc = "I2C/SPI Slave"]
pub struct IOSLAVE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for IOSLAVE {}
impl IOSLAVE {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ioslave::RegisterBlock {
        0x5000_0000 as *const _
    }
}
impl Deref for IOSLAVE {
    type Target = ioslave::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*IOSLAVE::ptr() }
    }
}
#[doc = "I2C/SPI Slave"]
pub mod ioslave;
#[doc = "MCU Miscellaneous Control Logic"]
pub struct MCUCTRL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for MCUCTRL {}
impl MCUCTRL {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const mcuctrl::RegisterBlock {
        0x4002_0000 as *const _
    }
}
impl Deref for MCUCTRL {
    type Target = mcuctrl::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*MCUCTRL::ptr() }
    }
}
#[doc = "MCU Miscellaneous Control Logic"]
pub mod mcuctrl;
#[doc = "MCU Reset Generator"]
pub struct RSTGEN {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RSTGEN {}
impl RSTGEN {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rstgen::RegisterBlock {
        0x4000_0000 as *const _
    }
}
impl Deref for RSTGEN {
    type Target = rstgen::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*RSTGEN::ptr() }
    }
}
#[doc = "MCU Reset Generator"]
pub mod rstgen;
#[doc = "Real Time Clock"]
pub struct RTC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTC {}
impl RTC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rtc::RegisterBlock {
        0x4000_4040 as *const _
    }
}
impl Deref for RTC {
    type Target = rtc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*RTC::ptr() }
    }
}
#[doc = "Real Time Clock"]
pub mod rtc;
#[doc = "Serial UART"]
pub struct UART {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART {}
impl UART {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uart::RegisterBlock {
        0x4001_c000 as *const _
    }
}
impl Deref for UART {
    type Target = uart::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*UART::ptr() }
    }
}
#[doc = "Serial UART"]
pub mod uart;
#[doc = "Voltage Comparator"]
pub struct VCOMP {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for VCOMP {}
impl VCOMP {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const vcomp::RegisterBlock {
        0x4000_c000 as *const _
    }
}
impl Deref for VCOMP {
    type Target = vcomp::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*VCOMP::ptr() }
    }
}
#[doc = "Voltage Comparator"]
pub mod vcomp;
#[doc = "Watchdog Timer"]
pub struct WDT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WDT {}
impl WDT {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const wdt::RegisterBlock {
        0x4002_4000 as *const _
    }
}
impl Deref for WDT {
    type Target = wdt::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*WDT::ptr() }
    }
}
#[doc = "Watchdog Timer"]
pub mod wdt;
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r"All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "ADC"]
    pub ADC: ADC,
    #[doc = "CLKGEN"]
    pub CLKGEN: CLKGEN,
    #[doc = "CTIMER"]
    pub CTIMER: CTIMER,
    #[doc = "GPIO"]
    pub GPIO: GPIO,
    #[doc = "IOMSTR0"]
    pub IOMSTR0: IOMSTR0,
    #[doc = "IOMSTR1"]
    pub IOMSTR1: IOMSTR1,
    #[doc = "IOSLAVE"]
    pub IOSLAVE: IOSLAVE,
    #[doc = "MCUCTRL"]
    pub MCUCTRL: MCUCTRL,
    #[doc = "RSTGEN"]
    pub RSTGEN: RSTGEN,
    #[doc = "RTC"]
    pub RTC: RTC,
    #[doc = "UART"]
    pub UART: UART,
    #[doc = "VCOMP"]
    pub VCOMP: VCOMP,
    #[doc = "WDT"]
    pub WDT: WDT,
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
            ADC: ADC {
                _marker: PhantomData,
            },
            CLKGEN: CLKGEN {
                _marker: PhantomData,
            },
            CTIMER: CTIMER {
                _marker: PhantomData,
            },
            GPIO: GPIO {
                _marker: PhantomData,
            },
            IOMSTR0: IOMSTR0 {
                _marker: PhantomData,
            },
            IOMSTR1: IOMSTR1 {
                _marker: PhantomData,
            },
            IOSLAVE: IOSLAVE {
                _marker: PhantomData,
            },
            MCUCTRL: MCUCTRL {
                _marker: PhantomData,
            },
            RSTGEN: RSTGEN {
                _marker: PhantomData,
            },
            RTC: RTC {
                _marker: PhantomData,
            },
            UART: UART {
                _marker: PhantomData,
            },
            VCOMP: VCOMP {
                _marker: PhantomData,
            },
            WDT: WDT {
                _marker: PhantomData,
            },
        }
    }
}
