#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Configuration Register"]
    pub cfg: CFG,
    #[doc = "0x04 - ADC Power Status"]
    pub stat: STAT,
    #[doc = "0x08 - Software trigger"]
    pub swt: SWT,
    #[doc = "0x0c - Slot 0 Configuration Register"]
    pub sl0cfg: SL0CFG,
    #[doc = "0x10 - Slot 1 Configuration Register"]
    pub sl1cfg: SL1CFG,
    #[doc = "0x14 - Slot 2 Configuration Register"]
    pub sl2cfg: SL2CFG,
    #[doc = "0x18 - Slot 3 Configuration Register"]
    pub sl3cfg: SL3CFG,
    #[doc = "0x1c - Slot 4 Configuration Register"]
    pub sl4cfg: SL4CFG,
    #[doc = "0x20 - Slot 5 Configuration Register"]
    pub sl5cfg: SL5CFG,
    #[doc = "0x24 - Slot 6 Configuration Register"]
    pub sl6cfg: SL6CFG,
    #[doc = "0x28 - Slot 7 Configuration Register"]
    pub sl7cfg: SL7CFG,
    #[doc = "0x2c - Window Comparator Limits Register"]
    pub wlim: WLIM,
    #[doc = "0x30 - FIFO Data and Valid Count Register"]
    pub fifo: FIFO,
    _reserved13: [u8; 460usize],
    #[doc = "0x200 - ADC Interrupt registers: Enable"]
    pub inten: INTEN,
    #[doc = "0x204 - ADC Interrupt registers: Status"]
    pub intstat: INTSTAT,
    #[doc = "0x208 - ADC Interrupt registers: Clear"]
    pub intclr: INTCLR,
    #[doc = "0x20c - ADC Interrupt registers: Set"]
    pub intset: INTSET,
}
#[doc = "Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub type CFG = crate::Reg<u32, _CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFG;
#[doc = "`read()` method returns [cfg::R](cfg::R) reader structure"]
impl crate::Readable for CFG {}
#[doc = "`write(|w| ..)` method takes [cfg::W](cfg::W) writer structure"]
impl crate::Writable for CFG {}
#[doc = "Configuration Register"]
pub mod cfg;
#[doc = "ADC Power Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stat](stat) module"]
pub type STAT = crate::Reg<u32, _STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STAT;
#[doc = "`read()` method returns [stat::R](stat::R) reader structure"]
impl crate::Readable for STAT {}
#[doc = "`write(|w| ..)` method takes [stat::W](stat::W) writer structure"]
impl crate::Writable for STAT {}
#[doc = "ADC Power Status"]
pub mod stat;
#[doc = "Software trigger\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swt](swt) module"]
pub type SWT = crate::Reg<u32, _SWT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SWT;
#[doc = "`read()` method returns [swt::R](swt::R) reader structure"]
impl crate::Readable for SWT {}
#[doc = "`write(|w| ..)` method takes [swt::W](swt::W) writer structure"]
impl crate::Writable for SWT {}
#[doc = "Software trigger"]
pub mod swt;
#[doc = "Slot 0 Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sl0cfg](sl0cfg) module"]
pub type SL0CFG = crate::Reg<u32, _SL0CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SL0CFG;
#[doc = "`read()` method returns [sl0cfg::R](sl0cfg::R) reader structure"]
impl crate::Readable for SL0CFG {}
#[doc = "`write(|w| ..)` method takes [sl0cfg::W](sl0cfg::W) writer structure"]
impl crate::Writable for SL0CFG {}
#[doc = "Slot 0 Configuration Register"]
pub mod sl0cfg;
#[doc = "Slot 1 Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sl1cfg](sl1cfg) module"]
pub type SL1CFG = crate::Reg<u32, _SL1CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SL1CFG;
#[doc = "`read()` method returns [sl1cfg::R](sl1cfg::R) reader structure"]
impl crate::Readable for SL1CFG {}
#[doc = "`write(|w| ..)` method takes [sl1cfg::W](sl1cfg::W) writer structure"]
impl crate::Writable for SL1CFG {}
#[doc = "Slot 1 Configuration Register"]
pub mod sl1cfg;
#[doc = "Slot 2 Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sl2cfg](sl2cfg) module"]
pub type SL2CFG = crate::Reg<u32, _SL2CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SL2CFG;
#[doc = "`read()` method returns [sl2cfg::R](sl2cfg::R) reader structure"]
impl crate::Readable for SL2CFG {}
#[doc = "`write(|w| ..)` method takes [sl2cfg::W](sl2cfg::W) writer structure"]
impl crate::Writable for SL2CFG {}
#[doc = "Slot 2 Configuration Register"]
pub mod sl2cfg;
#[doc = "Slot 3 Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sl3cfg](sl3cfg) module"]
pub type SL3CFG = crate::Reg<u32, _SL3CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SL3CFG;
#[doc = "`read()` method returns [sl3cfg::R](sl3cfg::R) reader structure"]
impl crate::Readable for SL3CFG {}
#[doc = "`write(|w| ..)` method takes [sl3cfg::W](sl3cfg::W) writer structure"]
impl crate::Writable for SL3CFG {}
#[doc = "Slot 3 Configuration Register"]
pub mod sl3cfg;
#[doc = "Slot 4 Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sl4cfg](sl4cfg) module"]
pub type SL4CFG = crate::Reg<u32, _SL4CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SL4CFG;
#[doc = "`read()` method returns [sl4cfg::R](sl4cfg::R) reader structure"]
impl crate::Readable for SL4CFG {}
#[doc = "`write(|w| ..)` method takes [sl4cfg::W](sl4cfg::W) writer structure"]
impl crate::Writable for SL4CFG {}
#[doc = "Slot 4 Configuration Register"]
pub mod sl4cfg;
#[doc = "Slot 5 Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sl5cfg](sl5cfg) module"]
pub type SL5CFG = crate::Reg<u32, _SL5CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SL5CFG;
#[doc = "`read()` method returns [sl5cfg::R](sl5cfg::R) reader structure"]
impl crate::Readable for SL5CFG {}
#[doc = "`write(|w| ..)` method takes [sl5cfg::W](sl5cfg::W) writer structure"]
impl crate::Writable for SL5CFG {}
#[doc = "Slot 5 Configuration Register"]
pub mod sl5cfg;
#[doc = "Slot 6 Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sl6cfg](sl6cfg) module"]
pub type SL6CFG = crate::Reg<u32, _SL6CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SL6CFG;
#[doc = "`read()` method returns [sl6cfg::R](sl6cfg::R) reader structure"]
impl crate::Readable for SL6CFG {}
#[doc = "`write(|w| ..)` method takes [sl6cfg::W](sl6cfg::W) writer structure"]
impl crate::Writable for SL6CFG {}
#[doc = "Slot 6 Configuration Register"]
pub mod sl6cfg;
#[doc = "Slot 7 Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sl7cfg](sl7cfg) module"]
pub type SL7CFG = crate::Reg<u32, _SL7CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SL7CFG;
#[doc = "`read()` method returns [sl7cfg::R](sl7cfg::R) reader structure"]
impl crate::Readable for SL7CFG {}
#[doc = "`write(|w| ..)` method takes [sl7cfg::W](sl7cfg::W) writer structure"]
impl crate::Writable for SL7CFG {}
#[doc = "Slot 7 Configuration Register"]
pub mod sl7cfg;
#[doc = "Window Comparator Limits Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wlim](wlim) module"]
pub type WLIM = crate::Reg<u32, _WLIM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WLIM;
#[doc = "`read()` method returns [wlim::R](wlim::R) reader structure"]
impl crate::Readable for WLIM {}
#[doc = "`write(|w| ..)` method takes [wlim::W](wlim::W) writer structure"]
impl crate::Writable for WLIM {}
#[doc = "Window Comparator Limits Register"]
pub mod wlim;
#[doc = "FIFO Data and Valid Count Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifo](fifo) module"]
pub type FIFO = crate::Reg<u32, _FIFO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFO;
#[doc = "`read()` method returns [fifo::R](fifo::R) reader structure"]
impl crate::Readable for FIFO {}
#[doc = "`write(|w| ..)` method takes [fifo::W](fifo::W) writer structure"]
impl crate::Writable for FIFO {}
#[doc = "FIFO Data and Valid Count Register"]
pub mod fifo;
#[doc = "ADC Interrupt registers: Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inten](inten) module"]
pub type INTEN = crate::Reg<u32, _INTEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTEN;
#[doc = "`read()` method returns [inten::R](inten::R) reader structure"]
impl crate::Readable for INTEN {}
#[doc = "`write(|w| ..)` method takes [inten::W](inten::W) writer structure"]
impl crate::Writable for INTEN {}
#[doc = "ADC Interrupt registers: Enable"]
pub mod inten;
#[doc = "ADC Interrupt registers: Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intstat](intstat) module"]
pub type INTSTAT = crate::Reg<u32, _INTSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTSTAT;
#[doc = "`read()` method returns [intstat::R](intstat::R) reader structure"]
impl crate::Readable for INTSTAT {}
#[doc = "`write(|w| ..)` method takes [intstat::W](intstat::W) writer structure"]
impl crate::Writable for INTSTAT {}
#[doc = "ADC Interrupt registers: Status"]
pub mod intstat;
#[doc = "ADC Interrupt registers: Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intclr](intclr) module"]
pub type INTCLR = crate::Reg<u32, _INTCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTCLR;
#[doc = "`read()` method returns [intclr::R](intclr::R) reader structure"]
impl crate::Readable for INTCLR {}
#[doc = "`write(|w| ..)` method takes [intclr::W](intclr::W) writer structure"]
impl crate::Writable for INTCLR {}
#[doc = "ADC Interrupt registers: Clear"]
pub mod intclr;
#[doc = "ADC Interrupt registers: Set\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intset](intset) module"]
pub type INTSET = crate::Reg<u32, _INTSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTSET;
#[doc = "`read()` method returns [intset::R](intset::R) reader structure"]
impl crate::Readable for INTSET {}
#[doc = "`write(|w| ..)` method takes [intset::W](intset::W) writer structure"]
impl crate::Writable for INTSET {}
#[doc = "ADC Interrupt registers: Set"]
pub mod intset;
