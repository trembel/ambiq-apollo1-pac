#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - UART Data Register"]
    pub dr: DR,
    #[doc = "0x04 - UART Status Register"]
    pub rsr: RSR,
    _reserved2: [u8; 16usize],
    #[doc = "0x18 - Flag Register"]
    pub fr: FR,
    _reserved3: [u8; 4usize],
    #[doc = "0x20 - IrDA Counter"]
    pub ilpr: ILPR,
    #[doc = "0x24 - Integer Baud Rate Divisor"]
    pub ibrd: IBRD,
    #[doc = "0x28 - Fractional Baud Rate Divisor"]
    pub fbrd: FBRD,
    #[doc = "0x2c - Line Control High"]
    pub lcrh: LCRH,
    #[doc = "0x30 - Control Register"]
    pub cr: CR,
    #[doc = "0x34 - FIFO Interrupt Level Select"]
    pub ifls: IFLS,
    #[doc = "0x38 - Interrupt Enable"]
    pub ier: IER,
    #[doc = "0x3c - Interrupt Status"]
    pub ies: IES,
    #[doc = "0x40 - Masked Interrupt Status"]
    pub mis: MIS,
    #[doc = "0x44 - Interrupt Clear"]
    pub iec: IEC,
}
#[doc = "UART Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dr](dr) module"]
pub type DR = crate::Reg<u32, _DR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DR;
#[doc = "`read()` method returns [dr::R](dr::R) reader structure"]
impl crate::Readable for DR {}
#[doc = "`write(|w| ..)` method takes [dr::W](dr::W) writer structure"]
impl crate::Writable for DR {}
#[doc = "UART Data Register"]
pub mod dr;
#[doc = "UART Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rsr](rsr) module"]
pub type RSR = crate::Reg<u32, _RSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSR;
#[doc = "`read()` method returns [rsr::R](rsr::R) reader structure"]
impl crate::Readable for RSR {}
#[doc = "`write(|w| ..)` method takes [rsr::W](rsr::W) writer structure"]
impl crate::Writable for RSR {}
#[doc = "UART Status Register"]
pub mod rsr;
#[doc = "Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fr](fr) module"]
pub type FR = crate::Reg<u32, _FR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FR;
#[doc = "`read()` method returns [fr::R](fr::R) reader structure"]
impl crate::Readable for FR {}
#[doc = "`write(|w| ..)` method takes [fr::W](fr::W) writer structure"]
impl crate::Writable for FR {}
#[doc = "Flag Register"]
pub mod fr;
#[doc = "IrDA Counter\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ilpr](ilpr) module"]
pub type ILPR = crate::Reg<u32, _ILPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ILPR;
#[doc = "`read()` method returns [ilpr::R](ilpr::R) reader structure"]
impl crate::Readable for ILPR {}
#[doc = "`write(|w| ..)` method takes [ilpr::W](ilpr::W) writer structure"]
impl crate::Writable for ILPR {}
#[doc = "IrDA Counter"]
pub mod ilpr;
#[doc = "Integer Baud Rate Divisor\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ibrd](ibrd) module"]
pub type IBRD = crate::Reg<u32, _IBRD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IBRD;
#[doc = "`read()` method returns [ibrd::R](ibrd::R) reader structure"]
impl crate::Readable for IBRD {}
#[doc = "`write(|w| ..)` method takes [ibrd::W](ibrd::W) writer structure"]
impl crate::Writable for IBRD {}
#[doc = "Integer Baud Rate Divisor"]
pub mod ibrd;
#[doc = "Fractional Baud Rate Divisor\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fbrd](fbrd) module"]
pub type FBRD = crate::Reg<u32, _FBRD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FBRD;
#[doc = "`read()` method returns [fbrd::R](fbrd::R) reader structure"]
impl crate::Readable for FBRD {}
#[doc = "`write(|w| ..)` method takes [fbrd::W](fbrd::W) writer structure"]
impl crate::Writable for FBRD {}
#[doc = "Fractional Baud Rate Divisor"]
pub mod fbrd;
#[doc = "Line Control High\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcrh](lcrh) module"]
pub type LCRH = crate::Reg<u32, _LCRH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCRH;
#[doc = "`read()` method returns [lcrh::R](lcrh::R) reader structure"]
impl crate::Readable for LCRH {}
#[doc = "`write(|w| ..)` method takes [lcrh::W](lcrh::W) writer structure"]
impl crate::Writable for LCRH {}
#[doc = "Line Control High"]
pub mod lcrh;
#[doc = "Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](cr) module"]
pub type CR = crate::Reg<u32, _CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR;
#[doc = "`read()` method returns [cr::R](cr::R) reader structure"]
impl crate::Readable for CR {}
#[doc = "`write(|w| ..)` method takes [cr::W](cr::W) writer structure"]
impl crate::Writable for CR {}
#[doc = "Control Register"]
pub mod cr;
#[doc = "FIFO Interrupt Level Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ifls](ifls) module"]
pub type IFLS = crate::Reg<u32, _IFLS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IFLS;
#[doc = "`read()` method returns [ifls::R](ifls::R) reader structure"]
impl crate::Readable for IFLS {}
#[doc = "`write(|w| ..)` method takes [ifls::W](ifls::W) writer structure"]
impl crate::Writable for IFLS {}
#[doc = "FIFO Interrupt Level Select"]
pub mod ifls;
#[doc = "Interrupt Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ier](ier) module"]
pub type IER = crate::Reg<u32, _IER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IER;
#[doc = "`read()` method returns [ier::R](ier::R) reader structure"]
impl crate::Readable for IER {}
#[doc = "`write(|w| ..)` method takes [ier::W](ier::W) writer structure"]
impl crate::Writable for IER {}
#[doc = "Interrupt Enable"]
pub mod ier;
#[doc = "Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ies](ies) module"]
pub type IES = crate::Reg<u32, _IES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IES;
#[doc = "`read()` method returns [ies::R](ies::R) reader structure"]
impl crate::Readable for IES {}
#[doc = "`write(|w| ..)` method takes [ies::W](ies::W) writer structure"]
impl crate::Writable for IES {}
#[doc = "Interrupt Status"]
pub mod ies;
#[doc = "Masked Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mis](mis) module"]
pub type MIS = crate::Reg<u32, _MIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MIS;
#[doc = "`read()` method returns [mis::R](mis::R) reader structure"]
impl crate::Readable for MIS {}
#[doc = "`write(|w| ..)` method takes [mis::W](mis::W) writer structure"]
impl crate::Writable for MIS {}
#[doc = "Masked Interrupt Status"]
pub mod mis;
#[doc = "Interrupt Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iec](iec) module"]
pub type IEC = crate::Reg<u32, _IEC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IEC;
#[doc = "`read()` method returns [iec::R](iec::R) reader structure"]
impl crate::Readable for IEC {}
#[doc = "`write(|w| ..)` method takes [iec::W](iec::W) writer structure"]
impl crate::Writable for IEC {}
#[doc = "Interrupt Clear"]
pub mod iec;
