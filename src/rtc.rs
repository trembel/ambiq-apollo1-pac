#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - RTC Counters Lower"]
    pub ctrlow: CTRLOW,
    #[doc = "0x04 - RTC Counters Upper"]
    pub ctrup: CTRUP,
    #[doc = "0x08 - RTC Alarms Lower"]
    pub almlow: ALMLOW,
    #[doc = "0x0c - RTC Alarms Upper"]
    pub almup: ALMUP,
    #[doc = "0x10 - RTC Control Register"]
    pub rtcctl: RTCCTL,
    _reserved5: [u8; 172usize],
    #[doc = "0xc0 - CLK_GEN Interrupt Register: Enable"]
    pub inten: INTEN,
    #[doc = "0xc4 - CLK_GEN Interrupt Register: Status"]
    pub intstat: INTSTAT,
    #[doc = "0xc8 - CLK_GEN Interrupt Register: Clear"]
    pub intclr: INTCLR,
    #[doc = "0xcc - CLK_GEN Interrupt Register: Set"]
    pub intset: INTSET,
}
#[doc = "RTC Counters Lower\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrlow](ctrlow) module"]
pub type CTRLOW = crate::Reg<u32, _CTRLOW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRLOW;
#[doc = "`read()` method returns [ctrlow::R](ctrlow::R) reader structure"]
impl crate::Readable for CTRLOW {}
#[doc = "`write(|w| ..)` method takes [ctrlow::W](ctrlow::W) writer structure"]
impl crate::Writable for CTRLOW {}
#[doc = "RTC Counters Lower"]
pub mod ctrlow;
#[doc = "RTC Counters Upper\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrup](ctrup) module"]
pub type CTRUP = crate::Reg<u32, _CTRUP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRUP;
#[doc = "`read()` method returns [ctrup::R](ctrup::R) reader structure"]
impl crate::Readable for CTRUP {}
#[doc = "`write(|w| ..)` method takes [ctrup::W](ctrup::W) writer structure"]
impl crate::Writable for CTRUP {}
#[doc = "RTC Counters Upper"]
pub mod ctrup;
#[doc = "RTC Alarms Lower\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [almlow](almlow) module"]
pub type ALMLOW = crate::Reg<u32, _ALMLOW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ALMLOW;
#[doc = "`read()` method returns [almlow::R](almlow::R) reader structure"]
impl crate::Readable for ALMLOW {}
#[doc = "`write(|w| ..)` method takes [almlow::W](almlow::W) writer structure"]
impl crate::Writable for ALMLOW {}
#[doc = "RTC Alarms Lower"]
pub mod almlow;
#[doc = "RTC Alarms Upper\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [almup](almup) module"]
pub type ALMUP = crate::Reg<u32, _ALMUP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ALMUP;
#[doc = "`read()` method returns [almup::R](almup::R) reader structure"]
impl crate::Readable for ALMUP {}
#[doc = "`write(|w| ..)` method takes [almup::W](almup::W) writer structure"]
impl crate::Writable for ALMUP {}
#[doc = "RTC Alarms Upper"]
pub mod almup;
#[doc = "RTC Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcctl](rtcctl) module"]
pub type RTCCTL = crate::Reg<u32, _RTCCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTCCTL;
#[doc = "`read()` method returns [rtcctl::R](rtcctl::R) reader structure"]
impl crate::Readable for RTCCTL {}
#[doc = "`write(|w| ..)` method takes [rtcctl::W](rtcctl::W) writer structure"]
impl crate::Writable for RTCCTL {}
#[doc = "RTC Control Register"]
pub mod rtcctl;
#[doc = "CLK_GEN Interrupt Register: Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inten](inten) module"]
pub type INTEN = crate::Reg<u32, _INTEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTEN;
#[doc = "`read()` method returns [inten::R](inten::R) reader structure"]
impl crate::Readable for INTEN {}
#[doc = "`write(|w| ..)` method takes [inten::W](inten::W) writer structure"]
impl crate::Writable for INTEN {}
#[doc = "CLK_GEN Interrupt Register: Enable"]
pub mod inten;
#[doc = "CLK_GEN Interrupt Register: Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intstat](intstat) module"]
pub type INTSTAT = crate::Reg<u32, _INTSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTSTAT;
#[doc = "`read()` method returns [intstat::R](intstat::R) reader structure"]
impl crate::Readable for INTSTAT {}
#[doc = "`write(|w| ..)` method takes [intstat::W](intstat::W) writer structure"]
impl crate::Writable for INTSTAT {}
#[doc = "CLK_GEN Interrupt Register: Status"]
pub mod intstat;
#[doc = "CLK_GEN Interrupt Register: Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intclr](intclr) module"]
pub type INTCLR = crate::Reg<u32, _INTCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTCLR;
#[doc = "`read()` method returns [intclr::R](intclr::R) reader structure"]
impl crate::Readable for INTCLR {}
#[doc = "`write(|w| ..)` method takes [intclr::W](intclr::W) writer structure"]
impl crate::Writable for INTCLR {}
#[doc = "CLK_GEN Interrupt Register: Clear"]
pub mod intclr;
#[doc = "CLK_GEN Interrupt Register: Set\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intset](intset) module"]
pub type INTSET = crate::Reg<u32, _INTSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTSET;
#[doc = "`read()` method returns [intset::R](intset::R) reader structure"]
impl crate::Readable for INTSET {}
#[doc = "`write(|w| ..)` method takes [intset::W](intset::W) writer structure"]
impl crate::Writable for INTSET {}
#[doc = "CLK_GEN Interrupt Register: Set"]
pub mod intset;
