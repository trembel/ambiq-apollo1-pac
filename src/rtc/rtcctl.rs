#[doc = "Reader of register RTCCTL"]
pub type R = crate::R<u32, super::RTCCTL>;
#[doc = "Writer for register RTCCTL"]
pub type W = crate::W<u32, super::RTCCTL>;
#[doc = "Register RTCCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::RTCCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Hours Counter mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HR1224_A {
    #[doc = "0: Hours in 24 hour mode"]
    _24HR = 0,
    #[doc = "1: Hours in 12 hour mode"]
    _12HR = 1,
}
impl From<HR1224_A> for bool {
    #[inline(always)]
    fn from(variant: HR1224_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HR1224`"]
pub type HR1224_R = crate::R<bool, HR1224_A>;
impl HR1224_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HR1224_A {
        match self.bits {
            false => HR1224_A::_24HR,
            true => HR1224_A::_12HR,
        }
    }
    #[doc = "Checks if the value of the field is `_24HR`"]
    #[inline(always)]
    pub fn is_24hr(&self) -> bool {
        *self == HR1224_A::_24HR
    }
    #[doc = "Checks if the value of the field is `_12HR`"]
    #[inline(always)]
    pub fn is_12hr(&self) -> bool {
        *self == HR1224_A::_12HR
    }
}
#[doc = "Write proxy for field `HR1224`"]
pub struct HR1224_W<'a> {
    w: &'a mut W,
}
impl<'a> HR1224_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HR1224_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Hours in 24 hour mode"]
    #[inline(always)]
    pub fn _24hr(self) -> &'a mut W {
        self.variant(HR1224_A::_24HR)
    }
    #[doc = "Hours in 12 hour mode"]
    #[inline(always)]
    pub fn _12hr(self) -> &'a mut W {
        self.variant(HR1224_A::_12HR)
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
#[doc = "RTC input clock control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSTOP_A {
    #[doc = "0: Allow the RTC input clock to run"]
    RUN = 0,
    #[doc = "1: Stop the RTC input clock"]
    STOP = 1,
}
impl From<RSTOP_A> for bool {
    #[inline(always)]
    fn from(variant: RSTOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RSTOP`"]
pub type RSTOP_R = crate::R<bool, RSTOP_A>;
impl RSTOP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RSTOP_A {
        match self.bits {
            false => RSTOP_A::RUN,
            true => RSTOP_A::STOP,
        }
    }
    #[doc = "Checks if the value of the field is `RUN`"]
    #[inline(always)]
    pub fn is_run(&self) -> bool {
        *self == RSTOP_A::RUN
    }
    #[doc = "Checks if the value of the field is `STOP`"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == RSTOP_A::STOP
    }
}
#[doc = "Write proxy for field `RSTOP`"]
pub struct RSTOP_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTOP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RSTOP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Allow the RTC input clock to run"]
    #[inline(always)]
    pub fn run(self) -> &'a mut W {
        self.variant(RSTOP_A::RUN)
    }
    #[doc = "Stop the RTC input clock"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut W {
        self.variant(RSTOP_A::STOP)
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
#[doc = "Alarm repeat interval\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RPT_A {
    #[doc = "0: Alarm interrupt disabled"]
    DIS = 0,
    #[doc = "1: Interrupt every year"]
    YEAR = 1,
    #[doc = "2: Interrupt every month"]
    MONTH = 2,
    #[doc = "3: Interrupt every week"]
    WEEK = 3,
    #[doc = "4: Interrupt every day"]
    DAY = 4,
    #[doc = "5: Interrupt every hour"]
    HR = 5,
    #[doc = "6: Interrupt every minute"]
    MIN = 6,
    #[doc = "7: Interrupt every second/10th/100th"]
    SEC = 7,
}
impl From<RPT_A> for u8 {
    #[inline(always)]
    fn from(variant: RPT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RPT`"]
pub type RPT_R = crate::R<u8, RPT_A>;
impl RPT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RPT_A {
        match self.bits {
            0 => RPT_A::DIS,
            1 => RPT_A::YEAR,
            2 => RPT_A::MONTH,
            3 => RPT_A::WEEK,
            4 => RPT_A::DAY,
            5 => RPT_A::HR,
            6 => RPT_A::MIN,
            7 => RPT_A::SEC,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == RPT_A::DIS
    }
    #[doc = "Checks if the value of the field is `YEAR`"]
    #[inline(always)]
    pub fn is_year(&self) -> bool {
        *self == RPT_A::YEAR
    }
    #[doc = "Checks if the value of the field is `MONTH`"]
    #[inline(always)]
    pub fn is_month(&self) -> bool {
        *self == RPT_A::MONTH
    }
    #[doc = "Checks if the value of the field is `WEEK`"]
    #[inline(always)]
    pub fn is_week(&self) -> bool {
        *self == RPT_A::WEEK
    }
    #[doc = "Checks if the value of the field is `DAY`"]
    #[inline(always)]
    pub fn is_day(&self) -> bool {
        *self == RPT_A::DAY
    }
    #[doc = "Checks if the value of the field is `HR`"]
    #[inline(always)]
    pub fn is_hr(&self) -> bool {
        *self == RPT_A::HR
    }
    #[doc = "Checks if the value of the field is `MIN`"]
    #[inline(always)]
    pub fn is_min(&self) -> bool {
        *self == RPT_A::MIN
    }
    #[doc = "Checks if the value of the field is `SEC`"]
    #[inline(always)]
    pub fn is_sec(&self) -> bool {
        *self == RPT_A::SEC
    }
}
#[doc = "Write proxy for field `RPT`"]
pub struct RPT_W<'a> {
    w: &'a mut W,
}
impl<'a> RPT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RPT_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Alarm interrupt disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(RPT_A::DIS)
    }
    #[doc = "Interrupt every year"]
    #[inline(always)]
    pub fn year(self) -> &'a mut W {
        self.variant(RPT_A::YEAR)
    }
    #[doc = "Interrupt every month"]
    #[inline(always)]
    pub fn month(self) -> &'a mut W {
        self.variant(RPT_A::MONTH)
    }
    #[doc = "Interrupt every week"]
    #[inline(always)]
    pub fn week(self) -> &'a mut W {
        self.variant(RPT_A::WEEK)
    }
    #[doc = "Interrupt every day"]
    #[inline(always)]
    pub fn day(self) -> &'a mut W {
        self.variant(RPT_A::DAY)
    }
    #[doc = "Interrupt every hour"]
    #[inline(always)]
    pub fn hr(self) -> &'a mut W {
        self.variant(RPT_A::HR)
    }
    #[doc = "Interrupt every minute"]
    #[inline(always)]
    pub fn min(self) -> &'a mut W {
        self.variant(RPT_A::MIN)
    }
    #[doc = "Interrupt every second/10th/100th"]
    #[inline(always)]
    pub fn sec(self) -> &'a mut W {
        self.variant(RPT_A::SEC)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 1)) | (((value as u32) & 0x07) << 1);
        self.w
    }
}
#[doc = "Counter write control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WRTC_A {
    #[doc = "0: Counter writes are disabled"]
    DIS = 0,
    #[doc = "1: Counter writes are enabled"]
    EN = 1,
}
impl From<WRTC_A> for bool {
    #[inline(always)]
    fn from(variant: WRTC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WRTC`"]
pub type WRTC_R = crate::R<bool, WRTC_A>;
impl WRTC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WRTC_A {
        match self.bits {
            false => WRTC_A::DIS,
            true => WRTC_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == WRTC_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == WRTC_A::EN
    }
}
#[doc = "Write proxy for field `WRTC`"]
pub struct WRTC_W<'a> {
    w: &'a mut W,
}
impl<'a> WRTC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WRTC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Counter writes are disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(WRTC_A::DIS)
    }
    #[doc = "Counter writes are enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(WRTC_A::EN)
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
    #[doc = "Bit 5 - Hours Counter mode"]
    #[inline(always)]
    pub fn hr1224(&self) -> HR1224_R {
        HR1224_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - RTC input clock control"]
    #[inline(always)]
    pub fn rstop(&self) -> RSTOP_R {
        RSTOP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 1:3 - Alarm repeat interval"]
    #[inline(always)]
    pub fn rpt(&self) -> RPT_R {
        RPT_R::new(((self.bits >> 1) & 0x07) as u8)
    }
    #[doc = "Bit 0 - Counter write control"]
    #[inline(always)]
    pub fn wrtc(&self) -> WRTC_R {
        WRTC_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - Hours Counter mode"]
    #[inline(always)]
    pub fn hr1224(&mut self) -> HR1224_W {
        HR1224_W { w: self }
    }
    #[doc = "Bit 4 - RTC input clock control"]
    #[inline(always)]
    pub fn rstop(&mut self) -> RSTOP_W {
        RSTOP_W { w: self }
    }
    #[doc = "Bits 1:3 - Alarm repeat interval"]
    #[inline(always)]
    pub fn rpt(&mut self) -> RPT_W {
        RPT_W { w: self }
    }
    #[doc = "Bit 0 - Counter write control"]
    #[inline(always)]
    pub fn wrtc(&mut self) -> WRTC_W {
        WRTC_W { w: self }
    }
}
