#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Pad Configuration Register A"]
    pub padrega: PADREGA,
    #[doc = "0x04 - Pad Configuration Register B"]
    pub padregb: PADREGB,
    #[doc = "0x08 - Pad Configuration Register C"]
    pub padregc: PADREGC,
    #[doc = "0x0c - Pad Configuration Register D"]
    pub padregd: PADREGD,
    #[doc = "0x10 - Pad Configuration Register E"]
    pub padrege: PADREGE,
    #[doc = "0x14 - Pad Configuration Register F"]
    pub padregf: PADREGF,
    #[doc = "0x18 - Pad Configuration Register G"]
    pub padregg: PADREGG,
    #[doc = "0x1c - Pad Configuration Register H"]
    pub padregh: PADREGH,
    #[doc = "0x20 - Pad Configuration Register I"]
    pub padregi: PADREGI,
    #[doc = "0x24 - Pad Configuration Register J"]
    pub padregj: PADREGJ,
    #[doc = "0x28 - Pad Configuration Register K"]
    pub padregk: PADREGK,
    #[doc = "0x2c - Pad Configuration Register L"]
    pub padregl: PADREGL,
    #[doc = "0x30 - Pad Configuration Register M"]
    pub padregm: PADREGM,
    _reserved13: [u8; 12usize],
    #[doc = "0x40 - GPIO Configuration Register A"]
    pub cfga: CFGA,
    #[doc = "0x44 - GPIO Configuration Register B"]
    pub cfgb: CFGB,
    #[doc = "0x48 - GPIO Configuration Register C"]
    pub cfgc: CFGC,
    #[doc = "0x4c - GPIO Configuration Register D"]
    pub cfgd: CFGD,
    #[doc = "0x50 - GPIO Configuration Register E"]
    pub cfge: CFGE,
    #[doc = "0x54 - GPIO Configuration Register F"]
    pub cfgf: CFGF,
    #[doc = "0x58 - GPIO Configuration Register G"]
    pub cfgg: CFGG,
    _reserved20: [u8; 4usize],
    #[doc = "0x60 - Key Register for all pad configuration registers"]
    pub padkey: PADKEY,
    _reserved21: [u8; 28usize],
    #[doc = "0x80 - GPIO Input Register A"]
    pub rda: RDA,
    #[doc = "0x84 - GPIO Input Register B"]
    pub rdb: RDB,
    #[doc = "0x88 - GPIO Output Register A"]
    pub wta: WTA,
    #[doc = "0x8c - GPIO Output Register B"]
    pub wtb: WTB,
    #[doc = "0x90 - GPIO Output Register A Set"]
    pub wtsa: WTSA,
    #[doc = "0x94 - GPIO Output Register B Set"]
    pub wtsb: WTSB,
    #[doc = "0x98 - GPIO Output Register A Clear"]
    pub wtca: WTCA,
    #[doc = "0x9c - GPIO Output Register B Clear"]
    pub wtcb: WTCB,
    #[doc = "0xa0 - GPIO Enable Register A"]
    pub ena: ENA,
    #[doc = "0xa4 - GPIO Enable Register B"]
    pub enb: ENB,
    #[doc = "0xa8 - GPIO Enable Register A Set"]
    pub ensa: ENSA,
    #[doc = "0xac - GPIO Enable Register B Set"]
    pub ensb: ENSB,
    _reserved33: [u8; 4usize],
    #[doc = "0xb4 - GPIO Enable Register A Clear"]
    pub enca: ENCA,
    #[doc = "0xb8 - GPIO Enable Register B Clear"]
    pub encb: ENCB,
    _reserved35: [u8; 324usize],
    #[doc = "0x200 - GPIO Interrupt Registers 31-0: Enable"]
    pub int0en: INT0EN,
    #[doc = "0x204 - GPIO Interrupt Registers 31-0: Status"]
    pub int0stat: INT0STAT,
    #[doc = "0x208 - GPIO Interrupt Registers 31-0: Clear"]
    pub int0clr: INT0CLR,
    #[doc = "0x20c - GPIO Interrupt Registers 31-0: Set"]
    pub int0set: INT0SET,
    #[doc = "0x210 - GPIO Interrupt Registers 49-32: Enable"]
    pub int1en: INT1EN,
    #[doc = "0x214 - GPIO Interrupt Registers 49-32: Status"]
    pub int1stat: INT1STAT,
    #[doc = "0x218 - GPIO Interrupt Registers 49-32: Clear"]
    pub int1clr: INT1CLR,
    #[doc = "0x21c - GPIO Interrupt Registers 49-32: Set"]
    pub int1set: INT1SET,
}
#[doc = "Pad Configuration Register A\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [padrega](padrega) module"]
pub type PADREGA = crate::Reg<u32, _PADREGA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PADREGA;
#[doc = "`read()` method returns [padrega::R](padrega::R) reader structure"]
impl crate::Readable for PADREGA {}
#[doc = "`write(|w| ..)` method takes [padrega::W](padrega::W) writer structure"]
impl crate::Writable for PADREGA {}
#[doc = "Pad Configuration Register A"]
pub mod padrega;
#[doc = "Pad Configuration Register B\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [padregb](padregb) module"]
pub type PADREGB = crate::Reg<u32, _PADREGB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PADREGB;
#[doc = "`read()` method returns [padregb::R](padregb::R) reader structure"]
impl crate::Readable for PADREGB {}
#[doc = "`write(|w| ..)` method takes [padregb::W](padregb::W) writer structure"]
impl crate::Writable for PADREGB {}
#[doc = "Pad Configuration Register B"]
pub mod padregb;
#[doc = "Pad Configuration Register C\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [padregc](padregc) module"]
pub type PADREGC = crate::Reg<u32, _PADREGC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PADREGC;
#[doc = "`read()` method returns [padregc::R](padregc::R) reader structure"]
impl crate::Readable for PADREGC {}
#[doc = "`write(|w| ..)` method takes [padregc::W](padregc::W) writer structure"]
impl crate::Writable for PADREGC {}
#[doc = "Pad Configuration Register C"]
pub mod padregc;
#[doc = "Pad Configuration Register D\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [padregd](padregd) module"]
pub type PADREGD = crate::Reg<u32, _PADREGD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PADREGD;
#[doc = "`read()` method returns [padregd::R](padregd::R) reader structure"]
impl crate::Readable for PADREGD {}
#[doc = "`write(|w| ..)` method takes [padregd::W](padregd::W) writer structure"]
impl crate::Writable for PADREGD {}
#[doc = "Pad Configuration Register D"]
pub mod padregd;
#[doc = "Pad Configuration Register E\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [padrege](padrege) module"]
pub type PADREGE = crate::Reg<u32, _PADREGE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PADREGE;
#[doc = "`read()` method returns [padrege::R](padrege::R) reader structure"]
impl crate::Readable for PADREGE {}
#[doc = "`write(|w| ..)` method takes [padrege::W](padrege::W) writer structure"]
impl crate::Writable for PADREGE {}
#[doc = "Pad Configuration Register E"]
pub mod padrege;
#[doc = "Pad Configuration Register F\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [padregf](padregf) module"]
pub type PADREGF = crate::Reg<u32, _PADREGF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PADREGF;
#[doc = "`read()` method returns [padregf::R](padregf::R) reader structure"]
impl crate::Readable for PADREGF {}
#[doc = "`write(|w| ..)` method takes [padregf::W](padregf::W) writer structure"]
impl crate::Writable for PADREGF {}
#[doc = "Pad Configuration Register F"]
pub mod padregf;
#[doc = "Pad Configuration Register G\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [padregg](padregg) module"]
pub type PADREGG = crate::Reg<u32, _PADREGG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PADREGG;
#[doc = "`read()` method returns [padregg::R](padregg::R) reader structure"]
impl crate::Readable for PADREGG {}
#[doc = "`write(|w| ..)` method takes [padregg::W](padregg::W) writer structure"]
impl crate::Writable for PADREGG {}
#[doc = "Pad Configuration Register G"]
pub mod padregg;
#[doc = "Pad Configuration Register H\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [padregh](padregh) module"]
pub type PADREGH = crate::Reg<u32, _PADREGH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PADREGH;
#[doc = "`read()` method returns [padregh::R](padregh::R) reader structure"]
impl crate::Readable for PADREGH {}
#[doc = "`write(|w| ..)` method takes [padregh::W](padregh::W) writer structure"]
impl crate::Writable for PADREGH {}
#[doc = "Pad Configuration Register H"]
pub mod padregh;
#[doc = "Pad Configuration Register I\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [padregi](padregi) module"]
pub type PADREGI = crate::Reg<u32, _PADREGI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PADREGI;
#[doc = "`read()` method returns [padregi::R](padregi::R) reader structure"]
impl crate::Readable for PADREGI {}
#[doc = "`write(|w| ..)` method takes [padregi::W](padregi::W) writer structure"]
impl crate::Writable for PADREGI {}
#[doc = "Pad Configuration Register I"]
pub mod padregi;
#[doc = "Pad Configuration Register J\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [padregj](padregj) module"]
pub type PADREGJ = crate::Reg<u32, _PADREGJ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PADREGJ;
#[doc = "`read()` method returns [padregj::R](padregj::R) reader structure"]
impl crate::Readable for PADREGJ {}
#[doc = "`write(|w| ..)` method takes [padregj::W](padregj::W) writer structure"]
impl crate::Writable for PADREGJ {}
#[doc = "Pad Configuration Register J"]
pub mod padregj;
#[doc = "Pad Configuration Register K\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [padregk](padregk) module"]
pub type PADREGK = crate::Reg<u32, _PADREGK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PADREGK;
#[doc = "`read()` method returns [padregk::R](padregk::R) reader structure"]
impl crate::Readable for PADREGK {}
#[doc = "`write(|w| ..)` method takes [padregk::W](padregk::W) writer structure"]
impl crate::Writable for PADREGK {}
#[doc = "Pad Configuration Register K"]
pub mod padregk;
#[doc = "Pad Configuration Register L\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [padregl](padregl) module"]
pub type PADREGL = crate::Reg<u32, _PADREGL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PADREGL;
#[doc = "`read()` method returns [padregl::R](padregl::R) reader structure"]
impl crate::Readable for PADREGL {}
#[doc = "`write(|w| ..)` method takes [padregl::W](padregl::W) writer structure"]
impl crate::Writable for PADREGL {}
#[doc = "Pad Configuration Register L"]
pub mod padregl;
#[doc = "Pad Configuration Register M\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [padregm](padregm) module"]
pub type PADREGM = crate::Reg<u32, _PADREGM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PADREGM;
#[doc = "`read()` method returns [padregm::R](padregm::R) reader structure"]
impl crate::Readable for PADREGM {}
#[doc = "`write(|w| ..)` method takes [padregm::W](padregm::W) writer structure"]
impl crate::Writable for PADREGM {}
#[doc = "Pad Configuration Register M"]
pub mod padregm;
#[doc = "GPIO Configuration Register A\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfga](cfga) module"]
pub type CFGA = crate::Reg<u32, _CFGA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFGA;
#[doc = "`read()` method returns [cfga::R](cfga::R) reader structure"]
impl crate::Readable for CFGA {}
#[doc = "`write(|w| ..)` method takes [cfga::W](cfga::W) writer structure"]
impl crate::Writable for CFGA {}
#[doc = "GPIO Configuration Register A"]
pub mod cfga;
#[doc = "GPIO Configuration Register B\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgb](cfgb) module"]
pub type CFGB = crate::Reg<u32, _CFGB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFGB;
#[doc = "`read()` method returns [cfgb::R](cfgb::R) reader structure"]
impl crate::Readable for CFGB {}
#[doc = "`write(|w| ..)` method takes [cfgb::W](cfgb::W) writer structure"]
impl crate::Writable for CFGB {}
#[doc = "GPIO Configuration Register B"]
pub mod cfgb;
#[doc = "GPIO Configuration Register C\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgc](cfgc) module"]
pub type CFGC = crate::Reg<u32, _CFGC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFGC;
#[doc = "`read()` method returns [cfgc::R](cfgc::R) reader structure"]
impl crate::Readable for CFGC {}
#[doc = "`write(|w| ..)` method takes [cfgc::W](cfgc::W) writer structure"]
impl crate::Writable for CFGC {}
#[doc = "GPIO Configuration Register C"]
pub mod cfgc;
#[doc = "GPIO Configuration Register D\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgd](cfgd) module"]
pub type CFGD = crate::Reg<u32, _CFGD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFGD;
#[doc = "`read()` method returns [cfgd::R](cfgd::R) reader structure"]
impl crate::Readable for CFGD {}
#[doc = "`write(|w| ..)` method takes [cfgd::W](cfgd::W) writer structure"]
impl crate::Writable for CFGD {}
#[doc = "GPIO Configuration Register D"]
pub mod cfgd;
#[doc = "GPIO Configuration Register E\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfge](cfge) module"]
pub type CFGE = crate::Reg<u32, _CFGE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFGE;
#[doc = "`read()` method returns [cfge::R](cfge::R) reader structure"]
impl crate::Readable for CFGE {}
#[doc = "`write(|w| ..)` method takes [cfge::W](cfge::W) writer structure"]
impl crate::Writable for CFGE {}
#[doc = "GPIO Configuration Register E"]
pub mod cfge;
#[doc = "GPIO Configuration Register F\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgf](cfgf) module"]
pub type CFGF = crate::Reg<u32, _CFGF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFGF;
#[doc = "`read()` method returns [cfgf::R](cfgf::R) reader structure"]
impl crate::Readable for CFGF {}
#[doc = "`write(|w| ..)` method takes [cfgf::W](cfgf::W) writer structure"]
impl crate::Writable for CFGF {}
#[doc = "GPIO Configuration Register F"]
pub mod cfgf;
#[doc = "GPIO Configuration Register G\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgg](cfgg) module"]
pub type CFGG = crate::Reg<u32, _CFGG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFGG;
#[doc = "`read()` method returns [cfgg::R](cfgg::R) reader structure"]
impl crate::Readable for CFGG {}
#[doc = "`write(|w| ..)` method takes [cfgg::W](cfgg::W) writer structure"]
impl crate::Writable for CFGG {}
#[doc = "GPIO Configuration Register G"]
pub mod cfgg;
#[doc = "Key Register for all pad configuration registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [padkey](padkey) module"]
pub type PADKEY = crate::Reg<u32, _PADKEY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PADKEY;
#[doc = "`read()` method returns [padkey::R](padkey::R) reader structure"]
impl crate::Readable for PADKEY {}
#[doc = "`write(|w| ..)` method takes [padkey::W](padkey::W) writer structure"]
impl crate::Writable for PADKEY {}
#[doc = "Key Register for all pad configuration registers"]
pub mod padkey;
#[doc = "GPIO Input Register A\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rda](rda) module"]
pub type RDA = crate::Reg<u32, _RDA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RDA;
#[doc = "`read()` method returns [rda::R](rda::R) reader structure"]
impl crate::Readable for RDA {}
#[doc = "`write(|w| ..)` method takes [rda::W](rda::W) writer structure"]
impl crate::Writable for RDA {}
#[doc = "GPIO Input Register A"]
pub mod rda;
#[doc = "GPIO Input Register B\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rdb](rdb) module"]
pub type RDB = crate::Reg<u32, _RDB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RDB;
#[doc = "`read()` method returns [rdb::R](rdb::R) reader structure"]
impl crate::Readable for RDB {}
#[doc = "`write(|w| ..)` method takes [rdb::W](rdb::W) writer structure"]
impl crate::Writable for RDB {}
#[doc = "GPIO Input Register B"]
pub mod rdb;
#[doc = "GPIO Output Register A\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wta](wta) module"]
pub type WTA = crate::Reg<u32, _WTA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WTA;
#[doc = "`read()` method returns [wta::R](wta::R) reader structure"]
impl crate::Readable for WTA {}
#[doc = "`write(|w| ..)` method takes [wta::W](wta::W) writer structure"]
impl crate::Writable for WTA {}
#[doc = "GPIO Output Register A"]
pub mod wta;
#[doc = "GPIO Output Register B\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wtb](wtb) module"]
pub type WTB = crate::Reg<u32, _WTB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WTB;
#[doc = "`read()` method returns [wtb::R](wtb::R) reader structure"]
impl crate::Readable for WTB {}
#[doc = "`write(|w| ..)` method takes [wtb::W](wtb::W) writer structure"]
impl crate::Writable for WTB {}
#[doc = "GPIO Output Register B"]
pub mod wtb;
#[doc = "GPIO Output Register A Set\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wtsa](wtsa) module"]
pub type WTSA = crate::Reg<u32, _WTSA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WTSA;
#[doc = "`read()` method returns [wtsa::R](wtsa::R) reader structure"]
impl crate::Readable for WTSA {}
#[doc = "`write(|w| ..)` method takes [wtsa::W](wtsa::W) writer structure"]
impl crate::Writable for WTSA {}
#[doc = "GPIO Output Register A Set"]
pub mod wtsa;
#[doc = "GPIO Output Register B Set\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wtsb](wtsb) module"]
pub type WTSB = crate::Reg<u32, _WTSB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WTSB;
#[doc = "`read()` method returns [wtsb::R](wtsb::R) reader structure"]
impl crate::Readable for WTSB {}
#[doc = "`write(|w| ..)` method takes [wtsb::W](wtsb::W) writer structure"]
impl crate::Writable for WTSB {}
#[doc = "GPIO Output Register B Set"]
pub mod wtsb;
#[doc = "GPIO Output Register A Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wtca](wtca) module"]
pub type WTCA = crate::Reg<u32, _WTCA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WTCA;
#[doc = "`read()` method returns [wtca::R](wtca::R) reader structure"]
impl crate::Readable for WTCA {}
#[doc = "`write(|w| ..)` method takes [wtca::W](wtca::W) writer structure"]
impl crate::Writable for WTCA {}
#[doc = "GPIO Output Register A Clear"]
pub mod wtca;
#[doc = "GPIO Output Register B Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wtcb](wtcb) module"]
pub type WTCB = crate::Reg<u32, _WTCB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WTCB;
#[doc = "`read()` method returns [wtcb::R](wtcb::R) reader structure"]
impl crate::Readable for WTCB {}
#[doc = "`write(|w| ..)` method takes [wtcb::W](wtcb::W) writer structure"]
impl crate::Writable for WTCB {}
#[doc = "GPIO Output Register B Clear"]
pub mod wtcb;
#[doc = "GPIO Enable Register A\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ena](ena) module"]
pub type ENA = crate::Reg<u32, _ENA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ENA;
#[doc = "`read()` method returns [ena::R](ena::R) reader structure"]
impl crate::Readable for ENA {}
#[doc = "`write(|w| ..)` method takes [ena::W](ena::W) writer structure"]
impl crate::Writable for ENA {}
#[doc = "GPIO Enable Register A"]
pub mod ena;
#[doc = "GPIO Enable Register B\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [enb](enb) module"]
pub type ENB = crate::Reg<u32, _ENB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ENB;
#[doc = "`read()` method returns [enb::R](enb::R) reader structure"]
impl crate::Readable for ENB {}
#[doc = "`write(|w| ..)` method takes [enb::W](enb::W) writer structure"]
impl crate::Writable for ENB {}
#[doc = "GPIO Enable Register B"]
pub mod enb;
#[doc = "GPIO Enable Register A Set\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ensa](ensa) module"]
pub type ENSA = crate::Reg<u32, _ENSA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ENSA;
#[doc = "`read()` method returns [ensa::R](ensa::R) reader structure"]
impl crate::Readable for ENSA {}
#[doc = "`write(|w| ..)` method takes [ensa::W](ensa::W) writer structure"]
impl crate::Writable for ENSA {}
#[doc = "GPIO Enable Register A Set"]
pub mod ensa;
#[doc = "GPIO Enable Register B Set\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ensb](ensb) module"]
pub type ENSB = crate::Reg<u32, _ENSB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ENSB;
#[doc = "`read()` method returns [ensb::R](ensb::R) reader structure"]
impl crate::Readable for ENSB {}
#[doc = "`write(|w| ..)` method takes [ensb::W](ensb::W) writer structure"]
impl crate::Writable for ENSB {}
#[doc = "GPIO Enable Register B Set"]
pub mod ensb;
#[doc = "GPIO Enable Register A Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [enca](enca) module"]
pub type ENCA = crate::Reg<u32, _ENCA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ENCA;
#[doc = "`read()` method returns [enca::R](enca::R) reader structure"]
impl crate::Readable for ENCA {}
#[doc = "`write(|w| ..)` method takes [enca::W](enca::W) writer structure"]
impl crate::Writable for ENCA {}
#[doc = "GPIO Enable Register A Clear"]
pub mod enca;
#[doc = "GPIO Enable Register B Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [encb](encb) module"]
pub type ENCB = crate::Reg<u32, _ENCB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ENCB;
#[doc = "`read()` method returns [encb::R](encb::R) reader structure"]
impl crate::Readable for ENCB {}
#[doc = "`write(|w| ..)` method takes [encb::W](encb::W) writer structure"]
impl crate::Writable for ENCB {}
#[doc = "GPIO Enable Register B Clear"]
pub mod encb;
#[doc = "GPIO Interrupt Registers 31-0: Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int0en](int0en) module"]
pub type INT0EN = crate::Reg<u32, _INT0EN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT0EN;
#[doc = "`read()` method returns [int0en::R](int0en::R) reader structure"]
impl crate::Readable for INT0EN {}
#[doc = "`write(|w| ..)` method takes [int0en::W](int0en::W) writer structure"]
impl crate::Writable for INT0EN {}
#[doc = "GPIO Interrupt Registers 31-0: Enable"]
pub mod int0en;
#[doc = "GPIO Interrupt Registers 31-0: Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int0stat](int0stat) module"]
pub type INT0STAT = crate::Reg<u32, _INT0STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT0STAT;
#[doc = "`read()` method returns [int0stat::R](int0stat::R) reader structure"]
impl crate::Readable for INT0STAT {}
#[doc = "`write(|w| ..)` method takes [int0stat::W](int0stat::W) writer structure"]
impl crate::Writable for INT0STAT {}
#[doc = "GPIO Interrupt Registers 31-0: Status"]
pub mod int0stat;
#[doc = "GPIO Interrupt Registers 31-0: Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int0clr](int0clr) module"]
pub type INT0CLR = crate::Reg<u32, _INT0CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT0CLR;
#[doc = "`read()` method returns [int0clr::R](int0clr::R) reader structure"]
impl crate::Readable for INT0CLR {}
#[doc = "`write(|w| ..)` method takes [int0clr::W](int0clr::W) writer structure"]
impl crate::Writable for INT0CLR {}
#[doc = "GPIO Interrupt Registers 31-0: Clear"]
pub mod int0clr;
#[doc = "GPIO Interrupt Registers 31-0: Set\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int0set](int0set) module"]
pub type INT0SET = crate::Reg<u32, _INT0SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT0SET;
#[doc = "`read()` method returns [int0set::R](int0set::R) reader structure"]
impl crate::Readable for INT0SET {}
#[doc = "`write(|w| ..)` method takes [int0set::W](int0set::W) writer structure"]
impl crate::Writable for INT0SET {}
#[doc = "GPIO Interrupt Registers 31-0: Set"]
pub mod int0set;
#[doc = "GPIO Interrupt Registers 49-32: Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int1en](int1en) module"]
pub type INT1EN = crate::Reg<u32, _INT1EN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT1EN;
#[doc = "`read()` method returns [int1en::R](int1en::R) reader structure"]
impl crate::Readable for INT1EN {}
#[doc = "`write(|w| ..)` method takes [int1en::W](int1en::W) writer structure"]
impl crate::Writable for INT1EN {}
#[doc = "GPIO Interrupt Registers 49-32: Enable"]
pub mod int1en;
#[doc = "GPIO Interrupt Registers 49-32: Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int1stat](int1stat) module"]
pub type INT1STAT = crate::Reg<u32, _INT1STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT1STAT;
#[doc = "`read()` method returns [int1stat::R](int1stat::R) reader structure"]
impl crate::Readable for INT1STAT {}
#[doc = "`write(|w| ..)` method takes [int1stat::W](int1stat::W) writer structure"]
impl crate::Writable for INT1STAT {}
#[doc = "GPIO Interrupt Registers 49-32: Status"]
pub mod int1stat;
#[doc = "GPIO Interrupt Registers 49-32: Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int1clr](int1clr) module"]
pub type INT1CLR = crate::Reg<u32, _INT1CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT1CLR;
#[doc = "`read()` method returns [int1clr::R](int1clr::R) reader structure"]
impl crate::Readable for INT1CLR {}
#[doc = "`write(|w| ..)` method takes [int1clr::W](int1clr::W) writer structure"]
impl crate::Writable for INT1CLR {}
#[doc = "GPIO Interrupt Registers 49-32: Clear"]
pub mod int1clr;
#[doc = "GPIO Interrupt Registers 49-32: Set\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int1set](int1set) module"]
pub type INT1SET = crate::Reg<u32, _INT1SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT1SET;
#[doc = "`read()` method returns [int1set::R](int1set::R) reader structure"]
impl crate::Readable for INT1SET {}
#[doc = "`write(|w| ..)` method takes [int1set::W](int1set::W) writer structure"]
impl crate::Writable for INT1SET {}
#[doc = "GPIO Interrupt Registers 49-32: Set"]
pub mod int1set;
