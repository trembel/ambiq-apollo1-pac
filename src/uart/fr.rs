#[doc = "Reader of register FR"]
pub type R = crate::R<u32, super::FR>;
#[doc = "Writer for register FR"]
pub type W = crate::W<u32, super::FR>;
#[doc = "Register FR `reset()`'s with value 0"]
impl crate::ResetValue for super::FR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RI`"]
pub type RI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RI`"]
pub struct RI_W<'a> {
    w: &'a mut W,
}
impl<'a> RI_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "This bit holds the transmit FIFO empty indicator.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXFE_A {
    #[doc = "1: Transmit fifo is empty."]
    XMTFIFO_EMPTY = 1,
}
impl From<TXFE_A> for bool {
    #[inline(always)]
    fn from(variant: TXFE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TXFE`"]
pub type TXFE_R = crate::R<bool, TXFE_A>;
impl TXFE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, TXFE_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(TXFE_A::XMTFIFO_EMPTY),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `XMTFIFO_EMPTY`"]
    #[inline(always)]
    pub fn is_xmtfifo_empty(&self) -> bool {
        *self == TXFE_A::XMTFIFO_EMPTY
    }
}
#[doc = "Write proxy for field `TXFE`"]
pub struct TXFE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXFE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Transmit fifo is empty."]
    #[inline(always)]
    pub fn xmtfifo_empty(self) -> &'a mut W {
        self.variant(TXFE_A::XMTFIFO_EMPTY)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "This bit holds the receive FIFO full indicator.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXFF_A {
    #[doc = "1: Receive fifo is full."]
    RCVFIFO_FULL = 1,
}
impl From<RXFF_A> for bool {
    #[inline(always)]
    fn from(variant: RXFF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RXFF`"]
pub type RXFF_R = crate::R<bool, RXFF_A>;
impl RXFF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, RXFF_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(RXFF_A::RCVFIFO_FULL),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `RCVFIFO_FULL`"]
    #[inline(always)]
    pub fn is_rcvfifo_full(&self) -> bool {
        *self == RXFF_A::RCVFIFO_FULL
    }
}
#[doc = "Write proxy for field `RXFF`"]
pub struct RXFF_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXFF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Receive fifo is full."]
    #[inline(always)]
    pub fn rcvfifo_full(self) -> &'a mut W {
        self.variant(RXFF_A::RCVFIFO_FULL)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "This bit holds the transmit FIFO full indicator.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXFF_A {
    #[doc = "1: Transmit fifo is full."]
    XMTFIFO_FULL = 1,
}
impl From<TXFF_A> for bool {
    #[inline(always)]
    fn from(variant: TXFF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TXFF`"]
pub type TXFF_R = crate::R<bool, TXFF_A>;
impl TXFF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, TXFF_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(TXFF_A::XMTFIFO_FULL),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `XMTFIFO_FULL`"]
    #[inline(always)]
    pub fn is_xmtfifo_full(&self) -> bool {
        *self == TXFF_A::XMTFIFO_FULL
    }
}
#[doc = "Write proxy for field `TXFF`"]
pub struct TXFF_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXFF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Transmit fifo is full."]
    #[inline(always)]
    pub fn xmtfifo_full(self) -> &'a mut W {
        self.variant(TXFF_A::XMTFIFO_FULL)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "This bit holds the receive FIFO empty indicator.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXFE_A {
    #[doc = "1: Receive fifo is empty."]
    RCVFIFO_EMPTY = 1,
}
impl From<RXFE_A> for bool {
    #[inline(always)]
    fn from(variant: RXFE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RXFE`"]
pub type RXFE_R = crate::R<bool, RXFE_A>;
impl RXFE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, RXFE_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(RXFE_A::RCVFIFO_EMPTY),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `RCVFIFO_EMPTY`"]
    #[inline(always)]
    pub fn is_rcvfifo_empty(&self) -> bool {
        *self == RXFE_A::RCVFIFO_EMPTY
    }
}
#[doc = "Write proxy for field `RXFE`"]
pub struct RXFE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXFE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Receive fifo is empty."]
    #[inline(always)]
    pub fn rcvfifo_empty(self) -> &'a mut W {
        self.variant(RXFE_A::RCVFIFO_EMPTY)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "This bit holds the busy indicator.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUSY_A {
    #[doc = "1: UART busy indicator."]
    BUSY = 1,
}
impl From<BUSY_A> for bool {
    #[inline(always)]
    fn from(variant: BUSY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BUSY`"]
pub type BUSY_R = crate::R<bool, BUSY_A>;
impl BUSY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, BUSY_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(BUSY_A::BUSY),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == BUSY_A::BUSY
    }
}
#[doc = "Write proxy for field `BUSY`"]
pub struct BUSY_W<'a> {
    w: &'a mut W,
}
impl<'a> BUSY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BUSY_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "UART busy indicator."]
    #[inline(always)]
    pub fn busy(self) -> &'a mut W {
        self.variant(BUSY_A::BUSY)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "This bit holds the data carrier detect indicator.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCD_A {
    #[doc = "1: Data carrier detect detected."]
    DETECTED = 1,
}
impl From<DCD_A> for bool {
    #[inline(always)]
    fn from(variant: DCD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DCD`"]
pub type DCD_R = crate::R<bool, DCD_A>;
impl DCD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, DCD_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(DCD_A::DETECTED),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DETECTED`"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == DCD_A::DETECTED
    }
}
#[doc = "Write proxy for field `DCD`"]
pub struct DCD_W<'a> {
    w: &'a mut W,
}
impl<'a> DCD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DCD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Data carrier detect detected."]
    #[inline(always)]
    pub fn detected(self) -> &'a mut W {
        self.variant(DCD_A::DETECTED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "This bit holds the data set ready indicator.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSR_A {
    #[doc = "1: Data set ready."]
    READY = 1,
}
impl From<DSR_A> for bool {
    #[inline(always)]
    fn from(variant: DSR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DSR`"]
pub type DSR_R = crate::R<bool, DSR_A>;
impl DSR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, DSR_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(DSR_A::READY),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `READY`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == DSR_A::READY
    }
}
#[doc = "Write proxy for field `DSR`"]
pub struct DSR_W<'a> {
    w: &'a mut W,
}
impl<'a> DSR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DSR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Data set ready."]
    #[inline(always)]
    pub fn ready(self) -> &'a mut W {
        self.variant(DSR_A::READY)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "This bit holds the clear to send indicator.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTS_A {
    #[doc = "1: Clear to send is indicated."]
    CLEARTOSEND = 1,
}
impl From<CTS_A> for bool {
    #[inline(always)]
    fn from(variant: CTS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CTS`"]
pub type CTS_R = crate::R<bool, CTS_A>;
impl CTS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, CTS_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(CTS_A::CLEARTOSEND),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CLEARTOSEND`"]
    #[inline(always)]
    pub fn is_cleartosend(&self) -> bool {
        *self == CTS_A::CLEARTOSEND
    }
}
#[doc = "Write proxy for field `CTS`"]
pub struct CTS_W<'a> {
    w: &'a mut W,
}
impl<'a> CTS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clear to send is indicated."]
    #[inline(always)]
    pub fn cleartosend(self) -> &'a mut W {
        self.variant(CTS_A::CLEARTOSEND)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 8 - This bit holds the ring indicator."]
    #[inline(always)]
    pub fn ri(&self) -> RI_R {
        RI_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - This bit holds the transmit FIFO empty indicator."]
    #[inline(always)]
    pub fn txfe(&self) -> TXFE_R {
        TXFE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - This bit holds the receive FIFO full indicator."]
    #[inline(always)]
    pub fn rxff(&self) -> RXFF_R {
        RXFF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - This bit holds the transmit FIFO full indicator."]
    #[inline(always)]
    pub fn txff(&self) -> TXFF_R {
        TXFF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - This bit holds the receive FIFO empty indicator."]
    #[inline(always)]
    pub fn rxfe(&self) -> RXFE_R {
        RXFE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - This bit holds the busy indicator."]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - This bit holds the data carrier detect indicator."]
    #[inline(always)]
    pub fn dcd(&self) -> DCD_R {
        DCD_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - This bit holds the data set ready indicator."]
    #[inline(always)]
    pub fn dsr(&self) -> DSR_R {
        DSR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - This bit holds the clear to send indicator."]
    #[inline(always)]
    pub fn cts(&self) -> CTS_R {
        CTS_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - This bit holds the ring indicator."]
    #[inline(always)]
    pub fn ri(&mut self) -> RI_W {
        RI_W { w: self }
    }
    #[doc = "Bit 7 - This bit holds the transmit FIFO empty indicator."]
    #[inline(always)]
    pub fn txfe(&mut self) -> TXFE_W {
        TXFE_W { w: self }
    }
    #[doc = "Bit 6 - This bit holds the receive FIFO full indicator."]
    #[inline(always)]
    pub fn rxff(&mut self) -> RXFF_W {
        RXFF_W { w: self }
    }
    #[doc = "Bit 5 - This bit holds the transmit FIFO full indicator."]
    #[inline(always)]
    pub fn txff(&mut self) -> TXFF_W {
        TXFF_W { w: self }
    }
    #[doc = "Bit 4 - This bit holds the receive FIFO empty indicator."]
    #[inline(always)]
    pub fn rxfe(&mut self) -> RXFE_W {
        RXFE_W { w: self }
    }
    #[doc = "Bit 3 - This bit holds the busy indicator."]
    #[inline(always)]
    pub fn busy(&mut self) -> BUSY_W {
        BUSY_W { w: self }
    }
    #[doc = "Bit 2 - This bit holds the data carrier detect indicator."]
    #[inline(always)]
    pub fn dcd(&mut self) -> DCD_W {
        DCD_W { w: self }
    }
    #[doc = "Bit 1 - This bit holds the data set ready indicator."]
    #[inline(always)]
    pub fn dsr(&mut self) -> DSR_W {
        DSR_W { w: self }
    }
    #[doc = "Bit 0 - This bit holds the clear to send indicator."]
    #[inline(always)]
    pub fn cts(&mut self) -> CTS_W {
        CTS_W { w: self }
    }
}
