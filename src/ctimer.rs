#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Counter/Timer Register"]
    pub tmr0: TMR0,
    #[doc = "0x04 - Counter/Timer A0 Compare Registers"]
    pub cmpra0: CMPRA0,
    #[doc = "0x08 - Counter/Timer B0 Compare Registers"]
    pub cmprb0: CMPRB0,
    #[doc = "0x0c - Counter/Timer Control"]
    pub ctrl0: CTRL0,
    #[doc = "0x10 - Counter/Timer Register"]
    pub tmr1: TMR1,
    #[doc = "0x14 - Counter/Timer A1 Compare Registers"]
    pub cmpra1: CMPRA1,
    #[doc = "0x18 - Counter/Timer B1 Compare Registers"]
    pub cmprb1: CMPRB1,
    #[doc = "0x1c - Counter/Timer Control"]
    pub ctrl1: CTRL1,
    #[doc = "0x20 - Counter/Timer Register"]
    pub tmr2: TMR2,
    #[doc = "0x24 - Counter/Timer A2 Compare Registers"]
    pub cmpra2: CMPRA2,
    #[doc = "0x28 - Counter/Timer B2 Compare Registers"]
    pub cmprb2: CMPRB2,
    #[doc = "0x2c - Counter/Timer Control"]
    pub ctrl2: CTRL2,
    #[doc = "0x30 - Counter/Timer Register"]
    pub tmr3: TMR3,
    #[doc = "0x34 - Counter/Timer A3 Compare Registers"]
    pub cmpra3: CMPRA3,
    #[doc = "0x38 - Counter/Timer B3 Compare Registers"]
    pub cmprb3: CMPRB3,
    #[doc = "0x3c - Counter/Timer Control"]
    pub ctrl3: CTRL3,
    _reserved16: [u8; 448usize],
    #[doc = "0x200 - Counter/Timer Interrupts: Enable"]
    pub inten: INTEN,
    #[doc = "0x204 - Counter/Timer Interrupts: Status"]
    pub intstat: INTSTAT,
    #[doc = "0x208 - Counter/Timer Interrupts: Clear"]
    pub intclr: INTCLR,
    #[doc = "0x20c - Counter/Timer Interrupts: Set"]
    pub intset: INTSET,
}
#[doc = "Counter/Timer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tmr0](tmr0) module"]
pub type TMR0 = crate::Reg<u32, _TMR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TMR0;
#[doc = "`read()` method returns [tmr0::R](tmr0::R) reader structure"]
impl crate::Readable for TMR0 {}
#[doc = "`write(|w| ..)` method takes [tmr0::W](tmr0::W) writer structure"]
impl crate::Writable for TMR0 {}
#[doc = "Counter/Timer Register"]
pub mod tmr0;
#[doc = "Counter/Timer A0 Compare Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmpra0](cmpra0) module"]
pub type CMPRA0 = crate::Reg<u32, _CMPRA0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMPRA0;
#[doc = "`read()` method returns [cmpra0::R](cmpra0::R) reader structure"]
impl crate::Readable for CMPRA0 {}
#[doc = "`write(|w| ..)` method takes [cmpra0::W](cmpra0::W) writer structure"]
impl crate::Writable for CMPRA0 {}
#[doc = "Counter/Timer A0 Compare Registers"]
pub mod cmpra0;
#[doc = "Counter/Timer B0 Compare Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmprb0](cmprb0) module"]
pub type CMPRB0 = crate::Reg<u32, _CMPRB0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMPRB0;
#[doc = "`read()` method returns [cmprb0::R](cmprb0::R) reader structure"]
impl crate::Readable for CMPRB0 {}
#[doc = "`write(|w| ..)` method takes [cmprb0::W](cmprb0::W) writer structure"]
impl crate::Writable for CMPRB0 {}
#[doc = "Counter/Timer B0 Compare Registers"]
pub mod cmprb0;
#[doc = "Counter/Timer Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl0](ctrl0) module"]
pub type CTRL0 = crate::Reg<u32, _CTRL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL0;
#[doc = "`read()` method returns [ctrl0::R](ctrl0::R) reader structure"]
impl crate::Readable for CTRL0 {}
#[doc = "`write(|w| ..)` method takes [ctrl0::W](ctrl0::W) writer structure"]
impl crate::Writable for CTRL0 {}
#[doc = "Counter/Timer Control"]
pub mod ctrl0;
#[doc = "Counter/Timer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tmr1](tmr1) module"]
pub type TMR1 = crate::Reg<u32, _TMR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TMR1;
#[doc = "`read()` method returns [tmr1::R](tmr1::R) reader structure"]
impl crate::Readable for TMR1 {}
#[doc = "`write(|w| ..)` method takes [tmr1::W](tmr1::W) writer structure"]
impl crate::Writable for TMR1 {}
#[doc = "Counter/Timer Register"]
pub mod tmr1;
#[doc = "Counter/Timer A1 Compare Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmpra1](cmpra1) module"]
pub type CMPRA1 = crate::Reg<u32, _CMPRA1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMPRA1;
#[doc = "`read()` method returns [cmpra1::R](cmpra1::R) reader structure"]
impl crate::Readable for CMPRA1 {}
#[doc = "`write(|w| ..)` method takes [cmpra1::W](cmpra1::W) writer structure"]
impl crate::Writable for CMPRA1 {}
#[doc = "Counter/Timer A1 Compare Registers"]
pub mod cmpra1;
#[doc = "Counter/Timer B1 Compare Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmprb1](cmprb1) module"]
pub type CMPRB1 = crate::Reg<u32, _CMPRB1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMPRB1;
#[doc = "`read()` method returns [cmprb1::R](cmprb1::R) reader structure"]
impl crate::Readable for CMPRB1 {}
#[doc = "`write(|w| ..)` method takes [cmprb1::W](cmprb1::W) writer structure"]
impl crate::Writable for CMPRB1 {}
#[doc = "Counter/Timer B1 Compare Registers"]
pub mod cmprb1;
#[doc = "Counter/Timer Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl1](ctrl1) module"]
pub type CTRL1 = crate::Reg<u32, _CTRL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL1;
#[doc = "`read()` method returns [ctrl1::R](ctrl1::R) reader structure"]
impl crate::Readable for CTRL1 {}
#[doc = "`write(|w| ..)` method takes [ctrl1::W](ctrl1::W) writer structure"]
impl crate::Writable for CTRL1 {}
#[doc = "Counter/Timer Control"]
pub mod ctrl1;
#[doc = "Counter/Timer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tmr2](tmr2) module"]
pub type TMR2 = crate::Reg<u32, _TMR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TMR2;
#[doc = "`read()` method returns [tmr2::R](tmr2::R) reader structure"]
impl crate::Readable for TMR2 {}
#[doc = "`write(|w| ..)` method takes [tmr2::W](tmr2::W) writer structure"]
impl crate::Writable for TMR2 {}
#[doc = "Counter/Timer Register"]
pub mod tmr2;
#[doc = "Counter/Timer A2 Compare Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmpra2](cmpra2) module"]
pub type CMPRA2 = crate::Reg<u32, _CMPRA2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMPRA2;
#[doc = "`read()` method returns [cmpra2::R](cmpra2::R) reader structure"]
impl crate::Readable for CMPRA2 {}
#[doc = "`write(|w| ..)` method takes [cmpra2::W](cmpra2::W) writer structure"]
impl crate::Writable for CMPRA2 {}
#[doc = "Counter/Timer A2 Compare Registers"]
pub mod cmpra2;
#[doc = "Counter/Timer B2 Compare Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmprb2](cmprb2) module"]
pub type CMPRB2 = crate::Reg<u32, _CMPRB2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMPRB2;
#[doc = "`read()` method returns [cmprb2::R](cmprb2::R) reader structure"]
impl crate::Readable for CMPRB2 {}
#[doc = "`write(|w| ..)` method takes [cmprb2::W](cmprb2::W) writer structure"]
impl crate::Writable for CMPRB2 {}
#[doc = "Counter/Timer B2 Compare Registers"]
pub mod cmprb2;
#[doc = "Counter/Timer Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl2](ctrl2) module"]
pub type CTRL2 = crate::Reg<u32, _CTRL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL2;
#[doc = "`read()` method returns [ctrl2::R](ctrl2::R) reader structure"]
impl crate::Readable for CTRL2 {}
#[doc = "`write(|w| ..)` method takes [ctrl2::W](ctrl2::W) writer structure"]
impl crate::Writable for CTRL2 {}
#[doc = "Counter/Timer Control"]
pub mod ctrl2;
#[doc = "Counter/Timer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tmr3](tmr3) module"]
pub type TMR3 = crate::Reg<u32, _TMR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TMR3;
#[doc = "`read()` method returns [tmr3::R](tmr3::R) reader structure"]
impl crate::Readable for TMR3 {}
#[doc = "`write(|w| ..)` method takes [tmr3::W](tmr3::W) writer structure"]
impl crate::Writable for TMR3 {}
#[doc = "Counter/Timer Register"]
pub mod tmr3;
#[doc = "Counter/Timer A3 Compare Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmpra3](cmpra3) module"]
pub type CMPRA3 = crate::Reg<u32, _CMPRA3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMPRA3;
#[doc = "`read()` method returns [cmpra3::R](cmpra3::R) reader structure"]
impl crate::Readable for CMPRA3 {}
#[doc = "`write(|w| ..)` method takes [cmpra3::W](cmpra3::W) writer structure"]
impl crate::Writable for CMPRA3 {}
#[doc = "Counter/Timer A3 Compare Registers"]
pub mod cmpra3;
#[doc = "Counter/Timer B3 Compare Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmprb3](cmprb3) module"]
pub type CMPRB3 = crate::Reg<u32, _CMPRB3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMPRB3;
#[doc = "`read()` method returns [cmprb3::R](cmprb3::R) reader structure"]
impl crate::Readable for CMPRB3 {}
#[doc = "`write(|w| ..)` method takes [cmprb3::W](cmprb3::W) writer structure"]
impl crate::Writable for CMPRB3 {}
#[doc = "Counter/Timer B3 Compare Registers"]
pub mod cmprb3;
#[doc = "Counter/Timer Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl3](ctrl3) module"]
pub type CTRL3 = crate::Reg<u32, _CTRL3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL3;
#[doc = "`read()` method returns [ctrl3::R](ctrl3::R) reader structure"]
impl crate::Readable for CTRL3 {}
#[doc = "`write(|w| ..)` method takes [ctrl3::W](ctrl3::W) writer structure"]
impl crate::Writable for CTRL3 {}
#[doc = "Counter/Timer Control"]
pub mod ctrl3;
#[doc = "Counter/Timer Interrupts: Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inten](inten) module"]
pub type INTEN = crate::Reg<u32, _INTEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTEN;
#[doc = "`read()` method returns [inten::R](inten::R) reader structure"]
impl crate::Readable for INTEN {}
#[doc = "`write(|w| ..)` method takes [inten::W](inten::W) writer structure"]
impl crate::Writable for INTEN {}
#[doc = "Counter/Timer Interrupts: Enable"]
pub mod inten;
#[doc = "Counter/Timer Interrupts: Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intstat](intstat) module"]
pub type INTSTAT = crate::Reg<u32, _INTSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTSTAT;
#[doc = "`read()` method returns [intstat::R](intstat::R) reader structure"]
impl crate::Readable for INTSTAT {}
#[doc = "`write(|w| ..)` method takes [intstat::W](intstat::W) writer structure"]
impl crate::Writable for INTSTAT {}
#[doc = "Counter/Timer Interrupts: Status"]
pub mod intstat;
#[doc = "Counter/Timer Interrupts: Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intclr](intclr) module"]
pub type INTCLR = crate::Reg<u32, _INTCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTCLR;
#[doc = "`read()` method returns [intclr::R](intclr::R) reader structure"]
impl crate::Readable for INTCLR {}
#[doc = "`write(|w| ..)` method takes [intclr::W](intclr::W) writer structure"]
impl crate::Writable for INTCLR {}
#[doc = "Counter/Timer Interrupts: Clear"]
pub mod intclr;
#[doc = "Counter/Timer Interrupts: Set\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intset](intset) module"]
pub type INTSET = crate::Reg<u32, _INTSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTSET;
#[doc = "`read()` method returns [intset::R](intset::R) reader structure"]
impl crate::Readable for INTSET {}
#[doc = "`write(|w| ..)` method takes [intset::W](intset::W) writer structure"]
impl crate::Writable for INTSET {}
#[doc = "Counter/Timer Interrupts: Set"]
pub mod intset;
