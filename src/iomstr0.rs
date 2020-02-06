#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - FIFO Access Port"]
    pub fifo: FIFO,
    _reserved1: [u8; 252usize],
    #[doc = "0x100 - Current FIFO Pointers"]
    pub fifoptr: FIFOPTR,
    #[doc = "0x104 - Transfer Length"]
    pub tlngth: TLNGTH,
    #[doc = "0x108 - FIFO Threshold Configuration"]
    pub fifothr: FIFOTHR,
    #[doc = "0x10c - I/O Clock Configuration"]
    pub clkcfg: CLKCFG,
    #[doc = "0x110 - Command Register"]
    pub cmd: CMD,
    #[doc = "0x114 - Command Repeat Register"]
    pub cmdrpt: CMDRPT,
    #[doc = "0x118 - Status Register"]
    pub status: STATUS,
    #[doc = "0x11c - I/O Master Configuration"]
    pub cfg: CFG,
    _reserved9: [u8; 224usize],
    #[doc = "0x200 - IO Master Interrupts: Enable"]
    pub inten: INTEN,
    #[doc = "0x204 - IO Master Interrupts: Status"]
    pub intstat: INTSTAT,
    #[doc = "0x208 - IO Master Interrupts: Clear"]
    pub intclr: INTCLR,
    #[doc = "0x20c - IO Master Interrupts: Set"]
    pub intset: INTSET,
}
#[doc = "FIFO Access Port\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifo](fifo) module"]
pub type FIFO = crate::Reg<u32, _FIFO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFO;
#[doc = "`read()` method returns [fifo::R](fifo::R) reader structure"]
impl crate::Readable for FIFO {}
#[doc = "`write(|w| ..)` method takes [fifo::W](fifo::W) writer structure"]
impl crate::Writable for FIFO {}
#[doc = "FIFO Access Port"]
pub mod fifo;
#[doc = "Current FIFO Pointers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifoptr](fifoptr) module"]
pub type FIFOPTR = crate::Reg<u32, _FIFOPTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFOPTR;
#[doc = "`read()` method returns [fifoptr::R](fifoptr::R) reader structure"]
impl crate::Readable for FIFOPTR {}
#[doc = "`write(|w| ..)` method takes [fifoptr::W](fifoptr::W) writer structure"]
impl crate::Writable for FIFOPTR {}
#[doc = "Current FIFO Pointers"]
pub mod fifoptr;
#[doc = "Transfer Length\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tlngth](tlngth) module"]
pub type TLNGTH = crate::Reg<u32, _TLNGTH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TLNGTH;
#[doc = "`read()` method returns [tlngth::R](tlngth::R) reader structure"]
impl crate::Readable for TLNGTH {}
#[doc = "`write(|w| ..)` method takes [tlngth::W](tlngth::W) writer structure"]
impl crate::Writable for TLNGTH {}
#[doc = "Transfer Length"]
pub mod tlngth;
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
#[doc = "I/O Clock Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkcfg](clkcfg) module"]
pub type CLKCFG = crate::Reg<u32, _CLKCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLKCFG;
#[doc = "`read()` method returns [clkcfg::R](clkcfg::R) reader structure"]
impl crate::Readable for CLKCFG {}
#[doc = "`write(|w| ..)` method takes [clkcfg::W](clkcfg::W) writer structure"]
impl crate::Writable for CLKCFG {}
#[doc = "I/O Clock Configuration"]
pub mod clkcfg;
#[doc = "Command Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd](cmd) module"]
pub type CMD = crate::Reg<u32, _CMD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMD;
#[doc = "`read()` method returns [cmd::R](cmd::R) reader structure"]
impl crate::Readable for CMD {}
#[doc = "`write(|w| ..)` method takes [cmd::W](cmd::W) writer structure"]
impl crate::Writable for CMD {}
#[doc = "Command Register"]
pub mod cmd;
#[doc = "Command Repeat Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmdrpt](cmdrpt) module"]
pub type CMDRPT = crate::Reg<u32, _CMDRPT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMDRPT;
#[doc = "`read()` method returns [cmdrpt::R](cmdrpt::R) reader structure"]
impl crate::Readable for CMDRPT {}
#[doc = "`write(|w| ..)` method takes [cmdrpt::W](cmdrpt::W) writer structure"]
impl crate::Writable for CMDRPT {}
#[doc = "Command Repeat Register"]
pub mod cmdrpt;
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](status) module"]
pub type STATUS = crate::Reg<u32, _STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUS;
#[doc = "`read()` method returns [status::R](status::R) reader structure"]
impl crate::Readable for STATUS {}
#[doc = "`write(|w| ..)` method takes [status::W](status::W) writer structure"]
impl crate::Writable for STATUS {}
#[doc = "Status Register"]
pub mod status;
#[doc = "I/O Master Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub type CFG = crate::Reg<u32, _CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFG;
#[doc = "`read()` method returns [cfg::R](cfg::R) reader structure"]
impl crate::Readable for CFG {}
#[doc = "`write(|w| ..)` method takes [cfg::W](cfg::W) writer structure"]
impl crate::Writable for CFG {}
#[doc = "I/O Master Configuration"]
pub mod cfg;
#[doc = "IO Master Interrupts: Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inten](inten) module"]
pub type INTEN = crate::Reg<u32, _INTEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTEN;
#[doc = "`read()` method returns [inten::R](inten::R) reader structure"]
impl crate::Readable for INTEN {}
#[doc = "`write(|w| ..)` method takes [inten::W](inten::W) writer structure"]
impl crate::Writable for INTEN {}
#[doc = "IO Master Interrupts: Enable"]
pub mod inten;
#[doc = "IO Master Interrupts: Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intstat](intstat) module"]
pub type INTSTAT = crate::Reg<u32, _INTSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTSTAT;
#[doc = "`read()` method returns [intstat::R](intstat::R) reader structure"]
impl crate::Readable for INTSTAT {}
#[doc = "`write(|w| ..)` method takes [intstat::W](intstat::W) writer structure"]
impl crate::Writable for INTSTAT {}
#[doc = "IO Master Interrupts: Status"]
pub mod intstat;
#[doc = "IO Master Interrupts: Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intclr](intclr) module"]
pub type INTCLR = crate::Reg<u32, _INTCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTCLR;
#[doc = "`read()` method returns [intclr::R](intclr::R) reader structure"]
impl crate::Readable for INTCLR {}
#[doc = "`write(|w| ..)` method takes [intclr::W](intclr::W) writer structure"]
impl crate::Writable for INTCLR {}
#[doc = "IO Master Interrupts: Clear"]
pub mod intclr;
#[doc = "IO Master Interrupts: Set\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intset](intset) module"]
pub type INTSET = crate::Reg<u32, _INTSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTSET;
#[doc = "`read()` method returns [intset::R](intset::R) reader structure"]
impl crate::Readable for INTSET {}
#[doc = "`write(|w| ..)` method takes [intset::W](intset::W) writer structure"]
impl crate::Writable for INTSET {}
#[doc = "IO Master Interrupts: Set"]
pub mod intset;
