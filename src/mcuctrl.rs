#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Chip Information Register"]
    pub chip_info: CHIP_INFO,
    #[doc = "0x04 - Unique Chip ID 0"]
    pub chipid0: CHIPID0,
    #[doc = "0x08 - Unique Chip ID 1"]
    pub chipid1: CHIPID1,
    #[doc = "0x0c - Chip Revision"]
    pub chiprev: CHIPREV,
    #[doc = "0x10 - Memory and Core Voltage Supply Source Select Register"]
    pub supplysrc: SUPPLYSRC,
    #[doc = "0x14 - Memory and Core Voltage Supply Source Status Register"]
    pub supplystatus: SUPPLYSTATUS,
    _reserved6: [u8; 228usize],
    #[doc = "0xfc - Band Gap Enable"]
    pub bandgapen: BANDGAPEN,
    _reserved7: [u8; 64usize],
    #[doc = "0x140 - Powerdown an SRAM Bank in Deep Sleep mode"]
    pub srampwdinsleep: SRAMPWDINSLEEP,
    #[doc = "0x144 - Disables individual banks of the SRAM array"]
    pub srampwrdis: SRAMPWRDIS,
    #[doc = "0x148 - Disables individual banks of the Flash array"]
    pub flashpwrdis: FLASHPWRDIS,
    _reserved10: [u8; 116usize],
    #[doc = "0x1c0 - ICODE bus address which was present when a bus fault occurred."]
    pub icodefaultaddr: ICODEFAULTADDR,
    #[doc = "0x1c4 - DCODE bus address which was present when a bus fault occurred."]
    pub dcodefaultaddr: DCODEFAULTADDR,
    #[doc = "0x1c8 - System bus address which was present when a bus fault occurred."]
    pub sysfaultaddr: SYSFAULTADDR,
    #[doc = "0x1cc - Reflects the status of the bus decoders' fault detection. Any write to this register will clear all of the status bits within the register."]
    pub faultstatus: FAULTSTATUS,
    #[doc = "0x1d0 - Enable the fault capture registers"]
    pub faultcaptureen: FAULTCAPTUREEN,
    _reserved15: [u8; 124usize],
    #[doc = "0x250 - TPIU Control Register. Determines the clock enable and frequency for the M4's TPIU interface."]
    pub tpiuctrl: TPIUCTRL,
}
#[doc = "Chip Information Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chip_info](chip_info) module"]
pub type CHIP_INFO = crate::Reg<u32, _CHIP_INFO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHIP_INFO;
#[doc = "`read()` method returns [chip_info::R](chip_info::R) reader structure"]
impl crate::Readable for CHIP_INFO {}
#[doc = "`write(|w| ..)` method takes [chip_info::W](chip_info::W) writer structure"]
impl crate::Writable for CHIP_INFO {}
#[doc = "Chip Information Register"]
pub mod chip_info;
#[doc = "Unique Chip ID 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chipid0](chipid0) module"]
pub type CHIPID0 = crate::Reg<u32, _CHIPID0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHIPID0;
#[doc = "`read()` method returns [chipid0::R](chipid0::R) reader structure"]
impl crate::Readable for CHIPID0 {}
#[doc = "`write(|w| ..)` method takes [chipid0::W](chipid0::W) writer structure"]
impl crate::Writable for CHIPID0 {}
#[doc = "Unique Chip ID 0"]
pub mod chipid0;
#[doc = "Unique Chip ID 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chipid1](chipid1) module"]
pub type CHIPID1 = crate::Reg<u32, _CHIPID1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHIPID1;
#[doc = "`read()` method returns [chipid1::R](chipid1::R) reader structure"]
impl crate::Readable for CHIPID1 {}
#[doc = "`write(|w| ..)` method takes [chipid1::W](chipid1::W) writer structure"]
impl crate::Writable for CHIPID1 {}
#[doc = "Unique Chip ID 1"]
pub mod chipid1;
#[doc = "Chip Revision\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chiprev](chiprev) module"]
pub type CHIPREV = crate::Reg<u32, _CHIPREV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHIPREV;
#[doc = "`read()` method returns [chiprev::R](chiprev::R) reader structure"]
impl crate::Readable for CHIPREV {}
#[doc = "`write(|w| ..)` method takes [chiprev::W](chiprev::W) writer structure"]
impl crate::Writable for CHIPREV {}
#[doc = "Chip Revision"]
pub mod chiprev;
#[doc = "Memory and Core Voltage Supply Source Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [supplysrc](supplysrc) module"]
pub type SUPPLYSRC = crate::Reg<u32, _SUPPLYSRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SUPPLYSRC;
#[doc = "`read()` method returns [supplysrc::R](supplysrc::R) reader structure"]
impl crate::Readable for SUPPLYSRC {}
#[doc = "`write(|w| ..)` method takes [supplysrc::W](supplysrc::W) writer structure"]
impl crate::Writable for SUPPLYSRC {}
#[doc = "Memory and Core Voltage Supply Source Select Register"]
pub mod supplysrc;
#[doc = "Memory and Core Voltage Supply Source Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [supplystatus](supplystatus) module"]
pub type SUPPLYSTATUS = crate::Reg<u32, _SUPPLYSTATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SUPPLYSTATUS;
#[doc = "`read()` method returns [supplystatus::R](supplystatus::R) reader structure"]
impl crate::Readable for SUPPLYSTATUS {}
#[doc = "`write(|w| ..)` method takes [supplystatus::W](supplystatus::W) writer structure"]
impl crate::Writable for SUPPLYSTATUS {}
#[doc = "Memory and Core Voltage Supply Source Status Register"]
pub mod supplystatus;
#[doc = "Band Gap Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bandgapen](bandgapen) module"]
pub type BANDGAPEN = crate::Reg<u32, _BANDGAPEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BANDGAPEN;
#[doc = "`read()` method returns [bandgapen::R](bandgapen::R) reader structure"]
impl crate::Readable for BANDGAPEN {}
#[doc = "`write(|w| ..)` method takes [bandgapen::W](bandgapen::W) writer structure"]
impl crate::Writable for BANDGAPEN {}
#[doc = "Band Gap Enable"]
pub mod bandgapen;
#[doc = "Powerdown an SRAM Bank in Deep Sleep mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srampwdinsleep](srampwdinsleep) module"]
pub type SRAMPWDINSLEEP = crate::Reg<u32, _SRAMPWDINSLEEP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRAMPWDINSLEEP;
#[doc = "`read()` method returns [srampwdinsleep::R](srampwdinsleep::R) reader structure"]
impl crate::Readable for SRAMPWDINSLEEP {}
#[doc = "`write(|w| ..)` method takes [srampwdinsleep::W](srampwdinsleep::W) writer structure"]
impl crate::Writable for SRAMPWDINSLEEP {}
#[doc = "Powerdown an SRAM Bank in Deep Sleep mode"]
pub mod srampwdinsleep;
#[doc = "Disables individual banks of the SRAM array\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srampwrdis](srampwrdis) module"]
pub type SRAMPWRDIS = crate::Reg<u32, _SRAMPWRDIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRAMPWRDIS;
#[doc = "`read()` method returns [srampwrdis::R](srampwrdis::R) reader structure"]
impl crate::Readable for SRAMPWRDIS {}
#[doc = "`write(|w| ..)` method takes [srampwrdis::W](srampwrdis::W) writer structure"]
impl crate::Writable for SRAMPWRDIS {}
#[doc = "Disables individual banks of the SRAM array"]
pub mod srampwrdis;
#[doc = "Disables individual banks of the Flash array\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flashpwrdis](flashpwrdis) module"]
pub type FLASHPWRDIS = crate::Reg<u32, _FLASHPWRDIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLASHPWRDIS;
#[doc = "`read()` method returns [flashpwrdis::R](flashpwrdis::R) reader structure"]
impl crate::Readable for FLASHPWRDIS {}
#[doc = "`write(|w| ..)` method takes [flashpwrdis::W](flashpwrdis::W) writer structure"]
impl crate::Writable for FLASHPWRDIS {}
#[doc = "Disables individual banks of the Flash array"]
pub mod flashpwrdis;
#[doc = "ICODE bus address which was present when a bus fault occurred.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icodefaultaddr](icodefaultaddr) module"]
pub type ICODEFAULTADDR = crate::Reg<u32, _ICODEFAULTADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICODEFAULTADDR;
#[doc = "`read()` method returns [icodefaultaddr::R](icodefaultaddr::R) reader structure"]
impl crate::Readable for ICODEFAULTADDR {}
#[doc = "`write(|w| ..)` method takes [icodefaultaddr::W](icodefaultaddr::W) writer structure"]
impl crate::Writable for ICODEFAULTADDR {}
#[doc = "ICODE bus address which was present when a bus fault occurred."]
pub mod icodefaultaddr;
#[doc = "DCODE bus address which was present when a bus fault occurred.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcodefaultaddr](dcodefaultaddr) module"]
pub type DCODEFAULTADDR = crate::Reg<u32, _DCODEFAULTADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCODEFAULTADDR;
#[doc = "`read()` method returns [dcodefaultaddr::R](dcodefaultaddr::R) reader structure"]
impl crate::Readable for DCODEFAULTADDR {}
#[doc = "`write(|w| ..)` method takes [dcodefaultaddr::W](dcodefaultaddr::W) writer structure"]
impl crate::Writable for DCODEFAULTADDR {}
#[doc = "DCODE bus address which was present when a bus fault occurred."]
pub mod dcodefaultaddr;
#[doc = "System bus address which was present when a bus fault occurred.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sysfaultaddr](sysfaultaddr) module"]
pub type SYSFAULTADDR = crate::Reg<u32, _SYSFAULTADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSFAULTADDR;
#[doc = "`read()` method returns [sysfaultaddr::R](sysfaultaddr::R) reader structure"]
impl crate::Readable for SYSFAULTADDR {}
#[doc = "`write(|w| ..)` method takes [sysfaultaddr::W](sysfaultaddr::W) writer structure"]
impl crate::Writable for SYSFAULTADDR {}
#[doc = "System bus address which was present when a bus fault occurred."]
pub mod sysfaultaddr;
#[doc = "Reflects the status of the bus decoders' fault detection. Any write to this register will clear all of the status bits within the register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [faultstatus](faultstatus) module"]
pub type FAULTSTATUS = crate::Reg<u32, _FAULTSTATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FAULTSTATUS;
#[doc = "`read()` method returns [faultstatus::R](faultstatus::R) reader structure"]
impl crate::Readable for FAULTSTATUS {}
#[doc = "`write(|w| ..)` method takes [faultstatus::W](faultstatus::W) writer structure"]
impl crate::Writable for FAULTSTATUS {}
#[doc = "Reflects the status of the bus decoders' fault detection. Any write to this register will clear all of the status bits within the register."]
pub mod faultstatus;
#[doc = "Enable the fault capture registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [faultcaptureen](faultcaptureen) module"]
pub type FAULTCAPTUREEN = crate::Reg<u32, _FAULTCAPTUREEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FAULTCAPTUREEN;
#[doc = "`read()` method returns [faultcaptureen::R](faultcaptureen::R) reader structure"]
impl crate::Readable for FAULTCAPTUREEN {}
#[doc = "`write(|w| ..)` method takes [faultcaptureen::W](faultcaptureen::W) writer structure"]
impl crate::Writable for FAULTCAPTUREEN {}
#[doc = "Enable the fault capture registers"]
pub mod faultcaptureen;
#[doc = "TPIU Control Register. Determines the clock enable and frequency for the M4's TPIU interface.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tpiuctrl](tpiuctrl) module"]
pub type TPIUCTRL = crate::Reg<u32, _TPIUCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TPIUCTRL;
#[doc = "`read()` method returns [tpiuctrl::R](tpiuctrl::R) reader structure"]
impl crate::Readable for TPIUCTRL {}
#[doc = "`write(|w| ..)` method takes [tpiuctrl::W](tpiuctrl::W) writer structure"]
impl crate::Writable for TPIUCTRL {}
#[doc = "TPIU Control Register. Determines the clock enable and frequency for the M4's TPIU interface."]
pub mod tpiuctrl;
