#[doc = "Reader of register OCTRL"]
pub type R = crate::R<u32, super::OCTRL>;
#[doc = "Writer for register OCTRL"]
pub type W = crate::W<u32, super::OCTRL>;
#[doc = "Register OCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::OCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Autocalibration control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ACAL_A {
    #[doc = "0: Disable Autocalibration"]
    DIS = 0,
    #[doc = "2: Autocalibrate every 1024 seconds"]
    _1024SEC = 2,
    #[doc = "3: Autocalibrate every 512 seconds"]
    _512SEC = 3,
    #[doc = "6: Frequency measurement using XT"]
    XTFREQ = 6,
    #[doc = "7: Frequency measurement using external clock"]
    EXTFREQ = 7,
}
impl From<ACAL_A> for u8 {
    #[inline(always)]
    fn from(variant: ACAL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ACAL`"]
pub type ACAL_R = crate::R<u8, ACAL_A>;
impl ACAL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ACAL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(ACAL_A::DIS),
            2 => Val(ACAL_A::_1024SEC),
            3 => Val(ACAL_A::_512SEC),
            6 => Val(ACAL_A::XTFREQ),
            7 => Val(ACAL_A::EXTFREQ),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == ACAL_A::DIS
    }
    #[doc = "Checks if the value of the field is `_1024SEC`"]
    #[inline(always)]
    pub fn is_1024sec(&self) -> bool {
        *self == ACAL_A::_1024SEC
    }
    #[doc = "Checks if the value of the field is `_512SEC`"]
    #[inline(always)]
    pub fn is_512sec(&self) -> bool {
        *self == ACAL_A::_512SEC
    }
    #[doc = "Checks if the value of the field is `XTFREQ`"]
    #[inline(always)]
    pub fn is_xtfreq(&self) -> bool {
        *self == ACAL_A::XTFREQ
    }
    #[doc = "Checks if the value of the field is `EXTFREQ`"]
    #[inline(always)]
    pub fn is_extfreq(&self) -> bool {
        *self == ACAL_A::EXTFREQ
    }
}
#[doc = "Write proxy for field `ACAL`"]
pub struct ACAL_W<'a> {
    w: &'a mut W,
}
impl<'a> ACAL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACAL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Disable Autocalibration"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(ACAL_A::DIS)
    }
    #[doc = "Autocalibrate every 1024 seconds"]
    #[inline(always)]
    pub fn _1024sec(self) -> &'a mut W {
        self.variant(ACAL_A::_1024SEC)
    }
    #[doc = "Autocalibrate every 512 seconds"]
    #[inline(always)]
    pub fn _512sec(self) -> &'a mut W {
        self.variant(ACAL_A::_512SEC)
    }
    #[doc = "Frequency measurement using XT"]
    #[inline(always)]
    pub fn xtfreq(self) -> &'a mut W {
        self.variant(ACAL_A::XTFREQ)
    }
    #[doc = "Frequency measurement using external clock"]
    #[inline(always)]
    pub fn extfreq(self) -> &'a mut W {
        self.variant(ACAL_A::EXTFREQ)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Selects the RTC oscillator (1 => LFRC, 0 => XT)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OSEL_A {
    #[doc = "0: RTC uses the XT"]
    RTC_XT = 0,
    #[doc = "1: RTC uses the LFRC"]
    RTC_LFRC = 1,
}
impl From<OSEL_A> for bool {
    #[inline(always)]
    fn from(variant: OSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OSEL`"]
pub type OSEL_R = crate::R<bool, OSEL_A>;
impl OSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OSEL_A {
        match self.bits {
            false => OSEL_A::RTC_XT,
            true => OSEL_A::RTC_LFRC,
        }
    }
    #[doc = "Checks if the value of the field is `RTC_XT`"]
    #[inline(always)]
    pub fn is_rtc_xt(&self) -> bool {
        *self == OSEL_A::RTC_XT
    }
    #[doc = "Checks if the value of the field is `RTC_LFRC`"]
    #[inline(always)]
    pub fn is_rtc_lfrc(&self) -> bool {
        *self == OSEL_A::RTC_LFRC
    }
}
#[doc = "Write proxy for field `OSEL`"]
pub struct OSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> OSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "RTC uses the XT"]
    #[inline(always)]
    pub fn rtc_xt(self) -> &'a mut W {
        self.variant(OSEL_A::RTC_XT)
    }
    #[doc = "RTC uses the LFRC"]
    #[inline(always)]
    pub fn rtc_lfrc(self) -> &'a mut W {
        self.variant(OSEL_A::RTC_LFRC)
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
#[doc = "Oscillator switch on failure function\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FOS_A {
    #[doc = "0: Disable the oscillator switch on failure function"]
    DIS = 0,
    #[doc = "1: Enable the oscillator switch on failure function"]
    EN = 1,
}
impl From<FOS_A> for bool {
    #[inline(always)]
    fn from(variant: FOS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FOS`"]
pub type FOS_R = crate::R<bool, FOS_A>;
impl FOS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FOS_A {
        match self.bits {
            false => FOS_A::DIS,
            true => FOS_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == FOS_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == FOS_A::EN
    }
}
#[doc = "Write proxy for field `FOS`"]
pub struct FOS_W<'a> {
    w: &'a mut W,
}
impl<'a> FOS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FOS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable the oscillator switch on failure function"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(FOS_A::DIS)
    }
    #[doc = "Enable the oscillator switch on failure function"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(FOS_A::EN)
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
#[doc = "Stop the LFRC Oscillator to the RTC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STOPRC_A {
    #[doc = "0: Enable the LFRC Oscillator to drive the RTC"]
    EN = 0,
    #[doc = "1: Stop the LFRC Oscillator when driving the RTC"]
    STOP = 1,
}
impl From<STOPRC_A> for bool {
    #[inline(always)]
    fn from(variant: STOPRC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `STOPRC`"]
pub type STOPRC_R = crate::R<bool, STOPRC_A>;
impl STOPRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STOPRC_A {
        match self.bits {
            false => STOPRC_A::EN,
            true => STOPRC_A::STOP,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == STOPRC_A::EN
    }
    #[doc = "Checks if the value of the field is `STOP`"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == STOPRC_A::STOP
    }
}
#[doc = "Write proxy for field `STOPRC`"]
pub struct STOPRC_W<'a> {
    w: &'a mut W,
}
impl<'a> STOPRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STOPRC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable the LFRC Oscillator to drive the RTC"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(STOPRC_A::EN)
    }
    #[doc = "Stop the LFRC Oscillator when driving the RTC"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut W {
        self.variant(STOPRC_A::STOP)
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
#[doc = "Stop the XT Oscillator to the RTC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STOPXT_A {
    #[doc = "0: Enable the XT Oscillator to drive the RTC"]
    EN = 0,
    #[doc = "1: Stop the XT Oscillator when driving the RTC"]
    STOP = 1,
}
impl From<STOPXT_A> for bool {
    #[inline(always)]
    fn from(variant: STOPXT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `STOPXT`"]
pub type STOPXT_R = crate::R<bool, STOPXT_A>;
impl STOPXT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STOPXT_A {
        match self.bits {
            false => STOPXT_A::EN,
            true => STOPXT_A::STOP,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == STOPXT_A::EN
    }
    #[doc = "Checks if the value of the field is `STOP`"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == STOPXT_A::STOP
    }
}
#[doc = "Write proxy for field `STOPXT`"]
pub struct STOPXT_W<'a> {
    w: &'a mut W,
}
impl<'a> STOPXT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STOPXT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable the XT Oscillator to drive the RTC"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(STOPXT_A::EN)
    }
    #[doc = "Stop the XT Oscillator when driving the RTC"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut W {
        self.variant(STOPXT_A::STOP)
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
    #[doc = "Bits 8:10 - Autocalibration control"]
    #[inline(always)]
    pub fn acal(&self) -> ACAL_R {
        ACAL_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 7 - Selects the RTC oscillator (1 => LFRC, 0 => XT)"]
    #[inline(always)]
    pub fn osel(&self) -> OSEL_R {
        OSEL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Oscillator switch on failure function"]
    #[inline(always)]
    pub fn fos(&self) -> FOS_R {
        FOS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Stop the LFRC Oscillator to the RTC"]
    #[inline(always)]
    pub fn stoprc(&self) -> STOPRC_R {
        STOPRC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Stop the XT Oscillator to the RTC"]
    #[inline(always)]
    pub fn stopxt(&self) -> STOPXT_R {
        STOPXT_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 8:10 - Autocalibration control"]
    #[inline(always)]
    pub fn acal(&mut self) -> ACAL_W {
        ACAL_W { w: self }
    }
    #[doc = "Bit 7 - Selects the RTC oscillator (1 => LFRC, 0 => XT)"]
    #[inline(always)]
    pub fn osel(&mut self) -> OSEL_W {
        OSEL_W { w: self }
    }
    #[doc = "Bit 6 - Oscillator switch on failure function"]
    #[inline(always)]
    pub fn fos(&mut self) -> FOS_W {
        FOS_W { w: self }
    }
    #[doc = "Bit 1 - Stop the LFRC Oscillator to the RTC"]
    #[inline(always)]
    pub fn stoprc(&mut self) -> STOPRC_W {
        STOPRC_W { w: self }
    }
    #[doc = "Bit 0 - Stop the XT Oscillator to the RTC"]
    #[inline(always)]
    pub fn stopxt(&mut self) -> STOPXT_W {
        STOPXT_W { w: self }
    }
}
