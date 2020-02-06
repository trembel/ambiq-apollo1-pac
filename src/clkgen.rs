#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - XT Oscillator Control"]
    pub calxt: CALXT,
    #[doc = "0x04 - RC Oscillator Control"]
    pub calrc: CALRC,
    #[doc = "0x08 - Autocalibration Counter"]
    pub acalctr: ACALCTR,
    #[doc = "0x0c - Oscillator Control"]
    pub octrl: OCTRL,
    #[doc = "0x10 - CLKOUT Frequency Select"]
    pub clkout: CLKOUT,
    #[doc = "0x14 - Key Register for Clock Control Register"]
    pub clkkey: CLKKEY,
    #[doc = "0x18 - HFRC Clock Control"]
    pub cctrl: CCTRL,
    #[doc = "0x1c - Clock Generator Status"]
    pub status: STATUS,
    #[doc = "0x20 - HFRC Adjustment"]
    pub hfadj: HFADJ,
    #[doc = "0x24 - HFADJ readback"]
    pub hfval: HFVAL,
    #[doc = "0x28 - Clock Enable Status"]
    pub clocken: CLOCKEN,
    #[doc = "0x2c - UART Enable"]
    pub uarten: UARTEN,
    _reserved12: [u8; 208usize],
    #[doc = "0x100 - CLKGEN Interrupt Register: Enable"]
    pub inten: INTEN,
    #[doc = "0x104 - CLKGEN Interrupt Register: Status"]
    pub intstat: INTSTAT,
    #[doc = "0x108 - CLKGEN Interrupt Register: Clear"]
    pub intclr: INTCLR,
    #[doc = "0x10c - CLKGEN Interrupt Register: Set"]
    pub intset: INTSET,
}
#[doc = "XT Oscillator Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [calxt](calxt) module"]
pub type CALXT = crate::Reg<u32, _CALXT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CALXT;
#[doc = "`read()` method returns [calxt::R](calxt::R) reader structure"]
impl crate::Readable for CALXT {}
#[doc = "`write(|w| ..)` method takes [calxt::W](calxt::W) writer structure"]
impl crate::Writable for CALXT {}
#[doc = "XT Oscillator Control"]
pub mod calxt;
#[doc = "RC Oscillator Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [calrc](calrc) module"]
pub type CALRC = crate::Reg<u32, _CALRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CALRC;
#[doc = "`read()` method returns [calrc::R](calrc::R) reader structure"]
impl crate::Readable for CALRC {}
#[doc = "`write(|w| ..)` method takes [calrc::W](calrc::W) writer structure"]
impl crate::Writable for CALRC {}
#[doc = "RC Oscillator Control"]
pub mod calrc;
#[doc = "Autocalibration Counter\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acalctr](acalctr) module"]
pub type ACALCTR = crate::Reg<u32, _ACALCTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACALCTR;
#[doc = "`read()` method returns [acalctr::R](acalctr::R) reader structure"]
impl crate::Readable for ACALCTR {}
#[doc = "`write(|w| ..)` method takes [acalctr::W](acalctr::W) writer structure"]
impl crate::Writable for ACALCTR {}
#[doc = "Autocalibration Counter"]
pub mod acalctr;
#[doc = "Oscillator Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [octrl](octrl) module"]
pub type OCTRL = crate::Reg<u32, _OCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OCTRL;
#[doc = "`read()` method returns [octrl::R](octrl::R) reader structure"]
impl crate::Readable for OCTRL {}
#[doc = "`write(|w| ..)` method takes [octrl::W](octrl::W) writer structure"]
impl crate::Writable for OCTRL {}
#[doc = "Oscillator Control"]
pub mod octrl;
#[doc = "CLKOUT Frequency Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkout](clkout) module"]
pub type CLKOUT = crate::Reg<u32, _CLKOUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLKOUT;
#[doc = "`read()` method returns [clkout::R](clkout::R) reader structure"]
impl crate::Readable for CLKOUT {}
#[doc = "`write(|w| ..)` method takes [clkout::W](clkout::W) writer structure"]
impl crate::Writable for CLKOUT {}
#[doc = "CLKOUT Frequency Select"]
pub mod clkout;
#[doc = "Key Register for Clock Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkkey](clkkey) module"]
pub type CLKKEY = crate::Reg<u32, _CLKKEY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLKKEY;
#[doc = "`read()` method returns [clkkey::R](clkkey::R) reader structure"]
impl crate::Readable for CLKKEY {}
#[doc = "`write(|w| ..)` method takes [clkkey::W](clkkey::W) writer structure"]
impl crate::Writable for CLKKEY {}
#[doc = "Key Register for Clock Control Register"]
pub mod clkkey;
#[doc = "HFRC Clock Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cctrl](cctrl) module"]
pub type CCTRL = crate::Reg<u32, _CCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCTRL;
#[doc = "`read()` method returns [cctrl::R](cctrl::R) reader structure"]
impl crate::Readable for CCTRL {}
#[doc = "`write(|w| ..)` method takes [cctrl::W](cctrl::W) writer structure"]
impl crate::Writable for CCTRL {}
#[doc = "HFRC Clock Control"]
pub mod cctrl;
#[doc = "Clock Generator Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](status) module"]
pub type STATUS = crate::Reg<u32, _STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUS;
#[doc = "`read()` method returns [status::R](status::R) reader structure"]
impl crate::Readable for STATUS {}
#[doc = "`write(|w| ..)` method takes [status::W](status::W) writer structure"]
impl crate::Writable for STATUS {}
#[doc = "Clock Generator Status"]
pub mod status;
#[doc = "HFRC Adjustment\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hfadj](hfadj) module"]
pub type HFADJ = crate::Reg<u32, _HFADJ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HFADJ;
#[doc = "`read()` method returns [hfadj::R](hfadj::R) reader structure"]
impl crate::Readable for HFADJ {}
#[doc = "`write(|w| ..)` method takes [hfadj::W](hfadj::W) writer structure"]
impl crate::Writable for HFADJ {}
#[doc = "HFRC Adjustment"]
pub mod hfadj;
#[doc = "HFADJ readback\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hfval](hfval) module"]
pub type HFVAL = crate::Reg<u32, _HFVAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HFVAL;
#[doc = "`read()` method returns [hfval::R](hfval::R) reader structure"]
impl crate::Readable for HFVAL {}
#[doc = "`write(|w| ..)` method takes [hfval::W](hfval::W) writer structure"]
impl crate::Writable for HFVAL {}
#[doc = "HFADJ readback"]
pub mod hfval;
#[doc = "Clock Enable Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clocken](clocken) module"]
pub type CLOCKEN = crate::Reg<u32, _CLOCKEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLOCKEN;
#[doc = "`read()` method returns [clocken::R](clocken::R) reader structure"]
impl crate::Readable for CLOCKEN {}
#[doc = "`write(|w| ..)` method takes [clocken::W](clocken::W) writer structure"]
impl crate::Writable for CLOCKEN {}
#[doc = "Clock Enable Status"]
pub mod clocken;
#[doc = "UART Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uarten](uarten) module"]
pub type UARTEN = crate::Reg<u32, _UARTEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UARTEN;
#[doc = "`read()` method returns [uarten::R](uarten::R) reader structure"]
impl crate::Readable for UARTEN {}
#[doc = "`write(|w| ..)` method takes [uarten::W](uarten::W) writer structure"]
impl crate::Writable for UARTEN {}
#[doc = "UART Enable"]
pub mod uarten;
#[doc = "CLKGEN Interrupt Register: Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inten](inten) module"]
pub type INTEN = crate::Reg<u32, _INTEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTEN;
#[doc = "`read()` method returns [inten::R](inten::R) reader structure"]
impl crate::Readable for INTEN {}
#[doc = "`write(|w| ..)` method takes [inten::W](inten::W) writer structure"]
impl crate::Writable for INTEN {}
#[doc = "CLKGEN Interrupt Register: Enable"]
pub mod inten;
#[doc = "CLKGEN Interrupt Register: Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intstat](intstat) module"]
pub type INTSTAT = crate::Reg<u32, _INTSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTSTAT;
#[doc = "`read()` method returns [intstat::R](intstat::R) reader structure"]
impl crate::Readable for INTSTAT {}
#[doc = "`write(|w| ..)` method takes [intstat::W](intstat::W) writer structure"]
impl crate::Writable for INTSTAT {}
#[doc = "CLKGEN Interrupt Register: Status"]
pub mod intstat;
#[doc = "CLKGEN Interrupt Register: Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intclr](intclr) module"]
pub type INTCLR = crate::Reg<u32, _INTCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTCLR;
#[doc = "`read()` method returns [intclr::R](intclr::R) reader structure"]
impl crate::Readable for INTCLR {}
#[doc = "`write(|w| ..)` method takes [intclr::W](intclr::W) writer structure"]
impl crate::Writable for INTCLR {}
#[doc = "CLKGEN Interrupt Register: Clear"]
pub mod intclr;
#[doc = "CLKGEN Interrupt Register: Set\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intset](intset) module"]
pub type INTSET = crate::Reg<u32, _INTSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTSET;
#[doc = "`read()` method returns [intset::R](intset::R) reader structure"]
impl crate::Readable for INTSET {}
#[doc = "`write(|w| ..)` method takes [intset::W](intset::W) writer structure"]
impl crate::Writable for INTSET {}
#[doc = "CLKGEN Interrupt Register: Set"]
pub mod intset;
