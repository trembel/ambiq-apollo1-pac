#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 256usize],
    #[doc = "0x100 - Current FIFO Pointer"]
    pub fifoptr: FIFOPTR,
    #[doc = "0x104 - FIFO Configuration"]
    pub fifocfg: FIFOCFG,
    #[doc = "0x108 - FIFO Threshold Configuration"]
    pub fifothr: FIFOTHR,
    #[doc = "0x10c - FIFO Update Status"]
    pub fupd: FUPD,
    #[doc = "0x110 - Overall FIFO Counter"]
    pub fifoctr: FIFOCTR,
    #[doc = "0x114 - Overall FIFO Counter Increment"]
    pub fifoinc: FIFOINC,
    #[doc = "0x118 - I/O Slave Configuration"]
    pub cfg: CFG,
    #[doc = "0x11c - I/O Slave Interrupt Priority Encode"]
    pub prenc: PRENC,
    #[doc = "0x120 - I/O Interrupt Control"]
    pub iointctl: IOINTCTL,
    #[doc = "0x124 - General Address Data"]
    pub genadd: GENADD,
    _reserved10: [u8; 216usize],
    #[doc = "0x200 - IO Slave Interrupts: Enable"]
    pub inten: INTEN,
    #[doc = "0x204 - IO Slave Interrupts: Status"]
    pub intstat: INTSTAT,
    #[doc = "0x208 - IO Slave Interrupts: Clear"]
    pub intclr: INTCLR,
    #[doc = "0x20c - IO Slave Interrupts: Set"]
    pub intset: INTSET,
    #[doc = "0x210 - Register Access Interrupts: Enable"]
    pub regaccinten: REGACCINTEN,
    #[doc = "0x214 - Register Access Interrupts: Status"]
    pub regaccintstat: REGACCINTSTAT,
    #[doc = "0x218 - Register Access Interrupts: Clear"]
    pub regaccintclr: REGACCINTCLR,
    #[doc = "0x21c - Register Access Interrupts: Set"]
    pub regaccintset: REGACCINTSET,
}
#[doc = "Current FIFO Pointer\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifoptr](fifoptr) module"]
pub type FIFOPTR = crate::Reg<u32, _FIFOPTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFOPTR;
#[doc = "`read()` method returns [fifoptr::R](fifoptr::R) reader structure"]
impl crate::Readable for FIFOPTR {}
#[doc = "`write(|w| ..)` method takes [fifoptr::W](fifoptr::W) writer structure"]
impl crate::Writable for FIFOPTR {}
#[doc = "Current FIFO Pointer"]
pub mod fifoptr;
#[doc = "FIFO Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifocfg](fifocfg) module"]
pub type FIFOCFG = crate::Reg<u32, _FIFOCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFOCFG;
#[doc = "`read()` method returns [fifocfg::R](fifocfg::R) reader structure"]
impl crate::Readable for FIFOCFG {}
#[doc = "`write(|w| ..)` method takes [fifocfg::W](fifocfg::W) writer structure"]
impl crate::Writable for FIFOCFG {}
#[doc = "FIFO Configuration"]
pub mod fifocfg;
#[doc = "FIFO Threshold Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifothr](fifothr) module"]
pub type FIFOTHR = crate::Reg<u32, _FIFOTHR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFOTHR;
#[doc = "`read()` method returns [fifothr::R](fifothr::R) reader structure"]
impl crate::Readable for FIFOTHR {}
#[doc = "`write(|w| ..)` method takes [fifothr::W](fifothr::W) writer structure"]
impl crate::Writable for FIFOTHR {}
#[doc = "FIFO Threshold Configuration"]
pub mod fifothr;
#[doc = "FIFO Update Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fupd](fupd) module"]
pub type FUPD = crate::Reg<u32, _FUPD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUPD;
#[doc = "`read()` method returns [fupd::R](fupd::R) reader structure"]
impl crate::Readable for FUPD {}
#[doc = "`write(|w| ..)` method takes [fupd::W](fupd::W) writer structure"]
impl crate::Writable for FUPD {}
#[doc = "FIFO Update Status"]
pub mod fupd;
#[doc = "Overall FIFO Counter\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifoctr](fifoctr) module"]
pub type FIFOCTR = crate::Reg<u32, _FIFOCTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFOCTR;
#[doc = "`read()` method returns [fifoctr::R](fifoctr::R) reader structure"]
impl crate::Readable for FIFOCTR {}
#[doc = "`write(|w| ..)` method takes [fifoctr::W](fifoctr::W) writer structure"]
impl crate::Writable for FIFOCTR {}
#[doc = "Overall FIFO Counter"]
pub mod fifoctr;
#[doc = "Overall FIFO Counter Increment\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifoinc](fifoinc) module"]
pub type FIFOINC = crate::Reg<u32, _FIFOINC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFOINC;
#[doc = "`read()` method returns [fifoinc::R](fifoinc::R) reader structure"]
impl crate::Readable for FIFOINC {}
#[doc = "`write(|w| ..)` method takes [fifoinc::W](fifoinc::W) writer structure"]
impl crate::Writable for FIFOINC {}
#[doc = "Overall FIFO Counter Increment"]
pub mod fifoinc;
#[doc = "I/O Slave Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub type CFG = crate::Reg<u32, _CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFG;
#[doc = "`read()` method returns [cfg::R](cfg::R) reader structure"]
impl crate::Readable for CFG {}
#[doc = "`write(|w| ..)` method takes [cfg::W](cfg::W) writer structure"]
impl crate::Writable for CFG {}
#[doc = "I/O Slave Configuration"]
pub mod cfg;
#[doc = "I/O Slave Interrupt Priority Encode\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prenc](prenc) module"]
pub type PRENC = crate::Reg<u32, _PRENC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRENC;
#[doc = "`read()` method returns [prenc::R](prenc::R) reader structure"]
impl crate::Readable for PRENC {}
#[doc = "`write(|w| ..)` method takes [prenc::W](prenc::W) writer structure"]
impl crate::Writable for PRENC {}
#[doc = "I/O Slave Interrupt Priority Encode"]
pub mod prenc;
#[doc = "I/O Interrupt Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iointctl](iointctl) module"]
pub type IOINTCTL = crate::Reg<u32, _IOINTCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOINTCTL;
#[doc = "`read()` method returns [iointctl::R](iointctl::R) reader structure"]
impl crate::Readable for IOINTCTL {}
#[doc = "`write(|w| ..)` method takes [iointctl::W](iointctl::W) writer structure"]
impl crate::Writable for IOINTCTL {}
#[doc = "I/O Interrupt Control"]
pub mod iointctl;
#[doc = "General Address Data\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [genadd](genadd) module"]
pub type GENADD = crate::Reg<u32, _GENADD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GENADD;
#[doc = "`read()` method returns [genadd::R](genadd::R) reader structure"]
impl crate::Readable for GENADD {}
#[doc = "`write(|w| ..)` method takes [genadd::W](genadd::W) writer structure"]
impl crate::Writable for GENADD {}
#[doc = "General Address Data"]
pub mod genadd;
#[doc = "IO Slave Interrupts: Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inten](inten) module"]
pub type INTEN = crate::Reg<u32, _INTEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTEN;
#[doc = "`read()` method returns [inten::R](inten::R) reader structure"]
impl crate::Readable for INTEN {}
#[doc = "`write(|w| ..)` method takes [inten::W](inten::W) writer structure"]
impl crate::Writable for INTEN {}
#[doc = "IO Slave Interrupts: Enable"]
pub mod inten;
#[doc = "IO Slave Interrupts: Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intstat](intstat) module"]
pub type INTSTAT = crate::Reg<u32, _INTSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTSTAT;
#[doc = "`read()` method returns [intstat::R](intstat::R) reader structure"]
impl crate::Readable for INTSTAT {}
#[doc = "`write(|w| ..)` method takes [intstat::W](intstat::W) writer structure"]
impl crate::Writable for INTSTAT {}
#[doc = "IO Slave Interrupts: Status"]
pub mod intstat;
#[doc = "IO Slave Interrupts: Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intclr](intclr) module"]
pub type INTCLR = crate::Reg<u32, _INTCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTCLR;
#[doc = "`read()` method returns [intclr::R](intclr::R) reader structure"]
impl crate::Readable for INTCLR {}
#[doc = "`write(|w| ..)` method takes [intclr::W](intclr::W) writer structure"]
impl crate::Writable for INTCLR {}
#[doc = "IO Slave Interrupts: Clear"]
pub mod intclr;
#[doc = "IO Slave Interrupts: Set\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intset](intset) module"]
pub type INTSET = crate::Reg<u32, _INTSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTSET;
#[doc = "`read()` method returns [intset::R](intset::R) reader structure"]
impl crate::Readable for INTSET {}
#[doc = "`write(|w| ..)` method takes [intset::W](intset::W) writer structure"]
impl crate::Writable for INTSET {}
#[doc = "IO Slave Interrupts: Set"]
pub mod intset;
#[doc = "Register Access Interrupts: Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [regaccinten](regaccinten) module"]
pub type REGACCINTEN = crate::Reg<u32, _REGACCINTEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REGACCINTEN;
#[doc = "`read()` method returns [regaccinten::R](regaccinten::R) reader structure"]
impl crate::Readable for REGACCINTEN {}
#[doc = "`write(|w| ..)` method takes [regaccinten::W](regaccinten::W) writer structure"]
impl crate::Writable for REGACCINTEN {}
#[doc = "Register Access Interrupts: Enable"]
pub mod regaccinten;
#[doc = "Register Access Interrupts: Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [regaccintstat](regaccintstat) module"]
pub type REGACCINTSTAT = crate::Reg<u32, _REGACCINTSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REGACCINTSTAT;
#[doc = "`read()` method returns [regaccintstat::R](regaccintstat::R) reader structure"]
impl crate::Readable for REGACCINTSTAT {}
#[doc = "`write(|w| ..)` method takes [regaccintstat::W](regaccintstat::W) writer structure"]
impl crate::Writable for REGACCINTSTAT {}
#[doc = "Register Access Interrupts: Status"]
pub mod regaccintstat;
#[doc = "Register Access Interrupts: Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [regaccintclr](regaccintclr) module"]
pub type REGACCINTCLR = crate::Reg<u32, _REGACCINTCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REGACCINTCLR;
#[doc = "`read()` method returns [regaccintclr::R](regaccintclr::R) reader structure"]
impl crate::Readable for REGACCINTCLR {}
#[doc = "`write(|w| ..)` method takes [regaccintclr::W](regaccintclr::W) writer structure"]
impl crate::Writable for REGACCINTCLR {}
#[doc = "Register Access Interrupts: Clear"]
pub mod regaccintclr;
#[doc = "Register Access Interrupts: Set\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [regaccintset](regaccintset) module"]
pub type REGACCINTSET = crate::Reg<u32, _REGACCINTSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REGACCINTSET;
#[doc = "`read()` method returns [regaccintset::R](regaccintset::R) reader structure"]
impl crate::Readable for REGACCINTSET {}
#[doc = "`write(|w| ..)` method takes [regaccintset::W](regaccintset::W) writer structure"]
impl crate::Writable for REGACCINTSET {}
#[doc = "Register Access Interrupts: Set"]
pub mod regaccintset;
