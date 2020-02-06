#[doc = "Reader of register CTRL3"]
pub type R = crate::R<u32, super::CTRL3>;
#[doc = "Writer for register CTRL3"]
pub type W = crate::W<u32, super::CTRL3>;
#[doc = "Register CTRL3 `reset()`'s with value 0"]
impl crate::ResetValue for super::CTRL3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Counter/Timer A/B Link bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTLINK3_A {
    #[doc = "0: Use A0/B0 timers as two independent 16-bit timers (default)."]
    TWO_16BIT_TIMERS = 0,
    #[doc = "1: Link A3/B3 timers into a single 32-bit timer."]
    _32BIT_TIMER = 1,
}
impl From<CTLINK3_A> for bool {
    #[inline(always)]
    fn from(variant: CTLINK3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CTLINK3`"]
pub type CTLINK3_R = crate::R<bool, CTLINK3_A>;
impl CTLINK3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTLINK3_A {
        match self.bits {
            false => CTLINK3_A::TWO_16BIT_TIMERS,
            true => CTLINK3_A::_32BIT_TIMER,
        }
    }
    #[doc = "Checks if the value of the field is `TWO_16BIT_TIMERS`"]
    #[inline(always)]
    pub fn is_two_16bit_timers(&self) -> bool {
        *self == CTLINK3_A::TWO_16BIT_TIMERS
    }
    #[doc = "Checks if the value of the field is `_32BIT_TIMER`"]
    #[inline(always)]
    pub fn is_32bit_timer(&self) -> bool {
        *self == CTLINK3_A::_32BIT_TIMER
    }
}
#[doc = "Write proxy for field `CTLINK3`"]
pub struct CTLINK3_W<'a> {
    w: &'a mut W,
}
impl<'a> CTLINK3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTLINK3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Use A0/B0 timers as two independent 16-bit timers (default)."]
    #[inline(always)]
    pub fn two_16bit_timers(self) -> &'a mut W {
        self.variant(CTLINK3_A::TWO_16BIT_TIMERS)
    }
    #[doc = "Link A3/B3 timers into a single 32-bit timer."]
    #[inline(always)]
    pub fn _32bit_timer(self) -> &'a mut W {
        self.variant(CTLINK3_A::_32BIT_TIMER)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
#[doc = "Counter/Timer B3 output polarity.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB3POL_A {
    #[doc = "0: The polarity of the TMRPINB3 pin is the same as the timer output."]
    NORMAL = 0,
    #[doc = "1: The polarity of the TMRPINB3 pin is the inverse of the timer output."]
    INVERTED = 1,
}
impl From<TMRB3POL_A> for bool {
    #[inline(always)]
    fn from(variant: TMRB3POL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TMRB3POL`"]
pub type TMRB3POL_R = crate::R<bool, TMRB3POL_A>;
impl TMRB3POL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRB3POL_A {
        match self.bits {
            false => TMRB3POL_A::NORMAL,
            true => TMRB3POL_A::INVERTED,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == TMRB3POL_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `INVERTED`"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == TMRB3POL_A::INVERTED
    }
}
#[doc = "Write proxy for field `TMRB3POL`"]
pub struct TMRB3POL_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRB3POL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRB3POL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The polarity of the TMRPINB3 pin is the same as the timer output."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(TMRB3POL_A::NORMAL)
    }
    #[doc = "The polarity of the TMRPINB3 pin is the inverse of the timer output."]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut W {
        self.variant(TMRB3POL_A::INVERTED)
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Counter/Timer B3 Clear bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB3CLR_A {
    #[doc = "0: Allow counter/timer B3 to run."]
    RUN = 0,
    #[doc = "1: Holds counter/timer B3 at 0x0000."]
    CLEAR = 1,
}
impl From<TMRB3CLR_A> for bool {
    #[inline(always)]
    fn from(variant: TMRB3CLR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TMRB3CLR`"]
pub type TMRB3CLR_R = crate::R<bool, TMRB3CLR_A>;
impl TMRB3CLR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRB3CLR_A {
        match self.bits {
            false => TMRB3CLR_A::RUN,
            true => TMRB3CLR_A::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `RUN`"]
    #[inline(always)]
    pub fn is_run(&self) -> bool {
        *self == TMRB3CLR_A::RUN
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == TMRB3CLR_A::CLEAR
    }
}
#[doc = "Write proxy for field `TMRB3CLR`"]
pub struct TMRB3CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRB3CLR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRB3CLR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Allow counter/timer B3 to run."]
    #[inline(always)]
    pub fn run(self) -> &'a mut W {
        self.variant(TMRB3CLR_A::RUN)
    }
    #[doc = "Holds counter/timer B3 at 0x0000."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(TMRB3CLR_A::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Counter/Timer B3 Output Enable bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB3PE_A {
    #[doc = "0: Counter/Timer B holds the TMRPINB signal at the value TMRB3POL."]
    DIS = 0,
    #[doc = "1: Enable counter/timer B3 to generate a signal on TMRPINB."]
    EN = 1,
}
impl From<TMRB3PE_A> for bool {
    #[inline(always)]
    fn from(variant: TMRB3PE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TMRB3PE`"]
pub type TMRB3PE_R = crate::R<bool, TMRB3PE_A>;
impl TMRB3PE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRB3PE_A {
        match self.bits {
            false => TMRB3PE_A::DIS,
            true => TMRB3PE_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TMRB3PE_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == TMRB3PE_A::EN
    }
}
#[doc = "Write proxy for field `TMRB3PE`"]
pub struct TMRB3PE_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRB3PE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRB3PE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Counter/Timer B holds the TMRPINB signal at the value TMRB3POL."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRB3PE_A::DIS)
    }
    #[doc = "Enable counter/timer B3 to generate a signal on TMRPINB."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRB3PE_A::EN)
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Counter/Timer B3 Interrupt Enable bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB3IE_A {
    #[doc = "0: Disable counter/timer B3 from generating an interrupt."]
    DIS = 0,
    #[doc = "1: Enable counter/timer B3 to generate an interrupt."]
    EN = 1,
}
impl From<TMRB3IE_A> for bool {
    #[inline(always)]
    fn from(variant: TMRB3IE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TMRB3IE`"]
pub type TMRB3IE_R = crate::R<bool, TMRB3IE_A>;
impl TMRB3IE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRB3IE_A {
        match self.bits {
            false => TMRB3IE_A::DIS,
            true => TMRB3IE_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TMRB3IE_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == TMRB3IE_A::EN
    }
}
#[doc = "Write proxy for field `TMRB3IE`"]
pub struct TMRB3IE_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRB3IE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRB3IE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable counter/timer B3 from generating an interrupt."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRB3IE_A::DIS)
    }
    #[doc = "Enable counter/timer B3 to generate an interrupt."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRB3IE_A::EN)
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Counter/Timer B3 Function Select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TMRB3FN_A {
    #[doc = "0: Single count (output toggles and sticks).  Count to CMPR0B3, stop."]
    SINGLECOUNT = 0,
    #[doc = "1: Repeated count (periodic 1-clock-cycle-wide pulses).  Count to CMPR0B3, restart."]
    REPEATEDCOUNT = 1,
    #[doc = "2: Pulse once (aka one-shot).  Count to CMPR0B3, assert, count to CMPR1B, deassert, stop."]
    PULSE_ONCE = 2,
    #[doc = "3: Pulse continously.  Count to CMPR0B3, assert, count to CMPR1B3, deassert, restart."]
    PULSE_CONT = 3,
    #[doc = "4: Continuous run (aka Free Run).  Count continuously."]
    CONTINUOUS = 4,
}
impl From<TMRB3FN_A> for u8 {
    #[inline(always)]
    fn from(variant: TMRB3FN_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TMRB3FN`"]
pub type TMRB3FN_R = crate::R<u8, TMRB3FN_A>;
impl TMRB3FN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TMRB3FN_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TMRB3FN_A::SINGLECOUNT),
            1 => Val(TMRB3FN_A::REPEATEDCOUNT),
            2 => Val(TMRB3FN_A::PULSE_ONCE),
            3 => Val(TMRB3FN_A::PULSE_CONT),
            4 => Val(TMRB3FN_A::CONTINUOUS),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SINGLECOUNT`"]
    #[inline(always)]
    pub fn is_singlecount(&self) -> bool {
        *self == TMRB3FN_A::SINGLECOUNT
    }
    #[doc = "Checks if the value of the field is `REPEATEDCOUNT`"]
    #[inline(always)]
    pub fn is_repeatedcount(&self) -> bool {
        *self == TMRB3FN_A::REPEATEDCOUNT
    }
    #[doc = "Checks if the value of the field is `PULSE_ONCE`"]
    #[inline(always)]
    pub fn is_pulse_once(&self) -> bool {
        *self == TMRB3FN_A::PULSE_ONCE
    }
    #[doc = "Checks if the value of the field is `PULSE_CONT`"]
    #[inline(always)]
    pub fn is_pulse_cont(&self) -> bool {
        *self == TMRB3FN_A::PULSE_CONT
    }
    #[doc = "Checks if the value of the field is `CONTINUOUS`"]
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        *self == TMRB3FN_A::CONTINUOUS
    }
}
#[doc = "Write proxy for field `TMRB3FN`"]
pub struct TMRB3FN_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRB3FN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRB3FN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Single count (output toggles and sticks). Count to CMPR0B3, stop."]
    #[inline(always)]
    pub fn singlecount(self) -> &'a mut W {
        self.variant(TMRB3FN_A::SINGLECOUNT)
    }
    #[doc = "Repeated count (periodic 1-clock-cycle-wide pulses). Count to CMPR0B3, restart."]
    #[inline(always)]
    pub fn repeatedcount(self) -> &'a mut W {
        self.variant(TMRB3FN_A::REPEATEDCOUNT)
    }
    #[doc = "Pulse once (aka one-shot). Count to CMPR0B3, assert, count to CMPR1B, deassert, stop."]
    #[inline(always)]
    pub fn pulse_once(self) -> &'a mut W {
        self.variant(TMRB3FN_A::PULSE_ONCE)
    }
    #[doc = "Pulse continously. Count to CMPR0B3, assert, count to CMPR1B3, deassert, restart."]
    #[inline(always)]
    pub fn pulse_cont(self) -> &'a mut W {
        self.variant(TMRB3FN_A::PULSE_CONT)
    }
    #[doc = "Continuous run (aka Free Run). Count continuously."]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut W {
        self.variant(TMRB3FN_A::CONTINUOUS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 22)) | (((value as u32) & 0x07) << 22);
        self.w
    }
}
#[doc = "Counter/Timer B3 Clock Select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TMRB3CLK_A {
    #[doc = "0: Clock source is TMRPINB."]
    TMRPIN = 0,
    #[doc = "1: Clock source is the HFRC"]
    HFRC = 1,
    #[doc = "2: Clock source is HFRC / 8"]
    HFRC_DIV8 = 2,
    #[doc = "3: Clock source is HFRC / 128"]
    HFRC_DIV128 = 3,
    #[doc = "4: Clock source is HFRC / 512"]
    HFRC_DIV512 = 4,
    #[doc = "5: Clock source is HFRC / 2048"]
    HFRC_DIV2K = 5,
    #[doc = "6: Clock source is the XT (uncalibrated)."]
    XT = 6,
    #[doc = "7: Clock source is XT / 2"]
    XT_DIV2 = 7,
    #[doc = "8: Clock source is XT / 16"]
    XT_DIV16 = 8,
    #[doc = "9: Clock source is XT / 256"]
    XT_DIV256 = 9,
    #[doc = "10: Clock source is LFRC / 2"]
    LFRC_DIV2 = 10,
    #[doc = "11: Clock source is LFRC / 32"]
    LFRC_DIV32 = 11,
    #[doc = "12: Clock source is LFRC / 1024"]
    LFRC_DIV1K = 12,
    #[doc = "13: Clock source is LFRC / 16K"]
    LFRC = 13,
    #[doc = "14: Clock source is 100 Hz from the current RTC oscillator."]
    RTC_100HZ = 14,
    #[doc = "15: Clock source is HCLK."]
    HCLK = 15,
    #[doc = "16: Clock source is buck converter stream A."]
    BUCKA = 16,
}
impl From<TMRB3CLK_A> for u8 {
    #[inline(always)]
    fn from(variant: TMRB3CLK_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TMRB3CLK`"]
pub type TMRB3CLK_R = crate::R<u8, TMRB3CLK_A>;
impl TMRB3CLK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TMRB3CLK_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TMRB3CLK_A::TMRPIN),
            1 => Val(TMRB3CLK_A::HFRC),
            2 => Val(TMRB3CLK_A::HFRC_DIV8),
            3 => Val(TMRB3CLK_A::HFRC_DIV128),
            4 => Val(TMRB3CLK_A::HFRC_DIV512),
            5 => Val(TMRB3CLK_A::HFRC_DIV2K),
            6 => Val(TMRB3CLK_A::XT),
            7 => Val(TMRB3CLK_A::XT_DIV2),
            8 => Val(TMRB3CLK_A::XT_DIV16),
            9 => Val(TMRB3CLK_A::XT_DIV256),
            10 => Val(TMRB3CLK_A::LFRC_DIV2),
            11 => Val(TMRB3CLK_A::LFRC_DIV32),
            12 => Val(TMRB3CLK_A::LFRC_DIV1K),
            13 => Val(TMRB3CLK_A::LFRC),
            14 => Val(TMRB3CLK_A::RTC_100HZ),
            15 => Val(TMRB3CLK_A::HCLK),
            16 => Val(TMRB3CLK_A::BUCKA),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TMRPIN`"]
    #[inline(always)]
    pub fn is_tmrpin(&self) -> bool {
        *self == TMRB3CLK_A::TMRPIN
    }
    #[doc = "Checks if the value of the field is `HFRC`"]
    #[inline(always)]
    pub fn is_hfrc(&self) -> bool {
        *self == TMRB3CLK_A::HFRC
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV8`"]
    #[inline(always)]
    pub fn is_hfrc_div8(&self) -> bool {
        *self == TMRB3CLK_A::HFRC_DIV8
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV128`"]
    #[inline(always)]
    pub fn is_hfrc_div128(&self) -> bool {
        *self == TMRB3CLK_A::HFRC_DIV128
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV512`"]
    #[inline(always)]
    pub fn is_hfrc_div512(&self) -> bool {
        *self == TMRB3CLK_A::HFRC_DIV512
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV2K`"]
    #[inline(always)]
    pub fn is_hfrc_div2k(&self) -> bool {
        *self == TMRB3CLK_A::HFRC_DIV2K
    }
    #[doc = "Checks if the value of the field is `XT`"]
    #[inline(always)]
    pub fn is_xt(&self) -> bool {
        *self == TMRB3CLK_A::XT
    }
    #[doc = "Checks if the value of the field is `XT_DIV2`"]
    #[inline(always)]
    pub fn is_xt_div2(&self) -> bool {
        *self == TMRB3CLK_A::XT_DIV2
    }
    #[doc = "Checks if the value of the field is `XT_DIV16`"]
    #[inline(always)]
    pub fn is_xt_div16(&self) -> bool {
        *self == TMRB3CLK_A::XT_DIV16
    }
    #[doc = "Checks if the value of the field is `XT_DIV256`"]
    #[inline(always)]
    pub fn is_xt_div256(&self) -> bool {
        *self == TMRB3CLK_A::XT_DIV256
    }
    #[doc = "Checks if the value of the field is `LFRC_DIV2`"]
    #[inline(always)]
    pub fn is_lfrc_div2(&self) -> bool {
        *self == TMRB3CLK_A::LFRC_DIV2
    }
    #[doc = "Checks if the value of the field is `LFRC_DIV32`"]
    #[inline(always)]
    pub fn is_lfrc_div32(&self) -> bool {
        *self == TMRB3CLK_A::LFRC_DIV32
    }
    #[doc = "Checks if the value of the field is `LFRC_DIV1K`"]
    #[inline(always)]
    pub fn is_lfrc_div1k(&self) -> bool {
        *self == TMRB3CLK_A::LFRC_DIV1K
    }
    #[doc = "Checks if the value of the field is `LFRC`"]
    #[inline(always)]
    pub fn is_lfrc(&self) -> bool {
        *self == TMRB3CLK_A::LFRC
    }
    #[doc = "Checks if the value of the field is `RTC_100HZ`"]
    #[inline(always)]
    pub fn is_rtc_100hz(&self) -> bool {
        *self == TMRB3CLK_A::RTC_100HZ
    }
    #[doc = "Checks if the value of the field is `HCLK`"]
    #[inline(always)]
    pub fn is_hclk(&self) -> bool {
        *self == TMRB3CLK_A::HCLK
    }
    #[doc = "Checks if the value of the field is `BUCKA`"]
    #[inline(always)]
    pub fn is_bucka(&self) -> bool {
        *self == TMRB3CLK_A::BUCKA
    }
}
#[doc = "Write proxy for field `TMRB3CLK`"]
pub struct TMRB3CLK_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRB3CLK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRB3CLK_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Clock source is TMRPINB."]
    #[inline(always)]
    pub fn tmrpin(self) -> &'a mut W {
        self.variant(TMRB3CLK_A::TMRPIN)
    }
    #[doc = "Clock source is the HFRC"]
    #[inline(always)]
    pub fn hfrc(self) -> &'a mut W {
        self.variant(TMRB3CLK_A::HFRC)
    }
    #[doc = "Clock source is HFRC / 8"]
    #[inline(always)]
    pub fn hfrc_div8(self) -> &'a mut W {
        self.variant(TMRB3CLK_A::HFRC_DIV8)
    }
    #[doc = "Clock source is HFRC / 128"]
    #[inline(always)]
    pub fn hfrc_div128(self) -> &'a mut W {
        self.variant(TMRB3CLK_A::HFRC_DIV128)
    }
    #[doc = "Clock source is HFRC / 512"]
    #[inline(always)]
    pub fn hfrc_div512(self) -> &'a mut W {
        self.variant(TMRB3CLK_A::HFRC_DIV512)
    }
    #[doc = "Clock source is HFRC / 2048"]
    #[inline(always)]
    pub fn hfrc_div2k(self) -> &'a mut W {
        self.variant(TMRB3CLK_A::HFRC_DIV2K)
    }
    #[doc = "Clock source is the XT (uncalibrated)."]
    #[inline(always)]
    pub fn xt(self) -> &'a mut W {
        self.variant(TMRB3CLK_A::XT)
    }
    #[doc = "Clock source is XT / 2"]
    #[inline(always)]
    pub fn xt_div2(self) -> &'a mut W {
        self.variant(TMRB3CLK_A::XT_DIV2)
    }
    #[doc = "Clock source is XT / 16"]
    #[inline(always)]
    pub fn xt_div16(self) -> &'a mut W {
        self.variant(TMRB3CLK_A::XT_DIV16)
    }
    #[doc = "Clock source is XT / 256"]
    #[inline(always)]
    pub fn xt_div256(self) -> &'a mut W {
        self.variant(TMRB3CLK_A::XT_DIV256)
    }
    #[doc = "Clock source is LFRC / 2"]
    #[inline(always)]
    pub fn lfrc_div2(self) -> &'a mut W {
        self.variant(TMRB3CLK_A::LFRC_DIV2)
    }
    #[doc = "Clock source is LFRC / 32"]
    #[inline(always)]
    pub fn lfrc_div32(self) -> &'a mut W {
        self.variant(TMRB3CLK_A::LFRC_DIV32)
    }
    #[doc = "Clock source is LFRC / 1024"]
    #[inline(always)]
    pub fn lfrc_div1k(self) -> &'a mut W {
        self.variant(TMRB3CLK_A::LFRC_DIV1K)
    }
    #[doc = "Clock source is LFRC / 16K"]
    #[inline(always)]
    pub fn lfrc(self) -> &'a mut W {
        self.variant(TMRB3CLK_A::LFRC)
    }
    #[doc = "Clock source is 100 Hz from the current RTC oscillator."]
    #[inline(always)]
    pub fn rtc_100hz(self) -> &'a mut W {
        self.variant(TMRB3CLK_A::RTC_100HZ)
    }
    #[doc = "Clock source is HCLK."]
    #[inline(always)]
    pub fn hclk(self) -> &'a mut W {
        self.variant(TMRB3CLK_A::HCLK)
    }
    #[doc = "Clock source is buck converter stream A."]
    #[inline(always)]
    pub fn bucka(self) -> &'a mut W {
        self.variant(TMRB3CLK_A::BUCKA)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 17)) | (((value as u32) & 0x1f) << 17);
        self.w
    }
}
#[doc = "Counter/Timer B3 Enable bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB3EN_A {
    #[doc = "0: Counter/Timer B3 Disable."]
    DIS = 0,
    #[doc = "1: Counter/Timer B3 Enable."]
    EN = 1,
}
impl From<TMRB3EN_A> for bool {
    #[inline(always)]
    fn from(variant: TMRB3EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TMRB3EN`"]
pub type TMRB3EN_R = crate::R<bool, TMRB3EN_A>;
impl TMRB3EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRB3EN_A {
        match self.bits {
            false => TMRB3EN_A::DIS,
            true => TMRB3EN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TMRB3EN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == TMRB3EN_A::EN
    }
}
#[doc = "Write proxy for field `TMRB3EN`"]
pub struct TMRB3EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRB3EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRB3EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Counter/Timer B3 Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRB3EN_A::DIS)
    }
    #[doc = "Counter/Timer B3 Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRB3EN_A::EN)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `ADCEN`"]
pub type ADCEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADCEN`"]
pub struct ADCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Counter/Timer A3 output polarity.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA3POL_A {
    #[doc = "0: The polarity of the TMRPINA3 pin is the same as the timer output."]
    NORMAL = 0,
    #[doc = "1: The polarity of the TMRPINA3 pin is the inverse of the timer output."]
    INVERTED = 1,
}
impl From<TMRA3POL_A> for bool {
    #[inline(always)]
    fn from(variant: TMRA3POL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TMRA3POL`"]
pub type TMRA3POL_R = crate::R<bool, TMRA3POL_A>;
impl TMRA3POL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRA3POL_A {
        match self.bits {
            false => TMRA3POL_A::NORMAL,
            true => TMRA3POL_A::INVERTED,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == TMRA3POL_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `INVERTED`"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == TMRA3POL_A::INVERTED
    }
}
#[doc = "Write proxy for field `TMRA3POL`"]
pub struct TMRA3POL_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRA3POL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRA3POL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The polarity of the TMRPINA3 pin is the same as the timer output."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(TMRA3POL_A::NORMAL)
    }
    #[doc = "The polarity of the TMRPINA3 pin is the inverse of the timer output."]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut W {
        self.variant(TMRA3POL_A::INVERTED)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Counter/Timer A3 Clear bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA3CLR_A {
    #[doc = "1: Holds counter/timer A3 at 0x0000."]
    CLEAR = 1,
}
impl From<TMRA3CLR_A> for bool {
    #[inline(always)]
    fn from(variant: TMRA3CLR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TMRA3CLR`"]
pub type TMRA3CLR_R = crate::R<bool, TMRA3CLR_A>;
impl TMRA3CLR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, TMRA3CLR_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(TMRA3CLR_A::CLEAR),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == TMRA3CLR_A::CLEAR
    }
}
#[doc = "Write proxy for field `TMRA3CLR`"]
pub struct TMRA3CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRA3CLR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRA3CLR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Holds counter/timer A3 at 0x0000."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(TMRA3CLR_A::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Counter/Timer A3 Output Enable bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA3PE_A {
    #[doc = "0: Counter/Timer A holds the TMRPINA signal at the value TMRA3POL."]
    DIS = 0,
    #[doc = "1: Enable counter/timer A3 to generate a signal on TMRPINA."]
    EN = 1,
}
impl From<TMRA3PE_A> for bool {
    #[inline(always)]
    fn from(variant: TMRA3PE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TMRA3PE`"]
pub type TMRA3PE_R = crate::R<bool, TMRA3PE_A>;
impl TMRA3PE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRA3PE_A {
        match self.bits {
            false => TMRA3PE_A::DIS,
            true => TMRA3PE_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TMRA3PE_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == TMRA3PE_A::EN
    }
}
#[doc = "Write proxy for field `TMRA3PE`"]
pub struct TMRA3PE_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRA3PE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRA3PE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Counter/Timer A holds the TMRPINA signal at the value TMRA3POL."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRA3PE_A::DIS)
    }
    #[doc = "Enable counter/timer A3 to generate a signal on TMRPINA."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRA3PE_A::EN)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Counter/Timer A3 Interrupt Enable bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA3IE_A {
    #[doc = "0: Disable counter/timer A3 from generating an interrupt."]
    DIS = 0,
    #[doc = "1: Enable counter/timer A3 to generate an interrupt."]
    EN = 1,
}
impl From<TMRA3IE_A> for bool {
    #[inline(always)]
    fn from(variant: TMRA3IE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TMRA3IE`"]
pub type TMRA3IE_R = crate::R<bool, TMRA3IE_A>;
impl TMRA3IE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRA3IE_A {
        match self.bits {
            false => TMRA3IE_A::DIS,
            true => TMRA3IE_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TMRA3IE_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == TMRA3IE_A::EN
    }
}
#[doc = "Write proxy for field `TMRA3IE`"]
pub struct TMRA3IE_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRA3IE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRA3IE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable counter/timer A3 from generating an interrupt."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRA3IE_A::DIS)
    }
    #[doc = "Enable counter/timer A3 to generate an interrupt."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRA3IE_A::EN)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Counter/Timer A3 Function Select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TMRA3FN_A {
    #[doc = "0: Single count (output toggles and sticks).  Count to CMPR0A3, stop."]
    SINGLECOUNT = 0,
    #[doc = "1: Repeated count (periodic 1-clock-cycle-wide pulses).  Count to CMPR0A3, restart."]
    REPEATEDCOUNT = 1,
    #[doc = "2: Pulse once (aka one-shot).  Count to CMPR0A3, assert, count to CMPR1B, deassert, stop."]
    PULSE_ONCE = 2,
    #[doc = "3: Pulse continously.  Count to CMPR0A3, assert, count to CMPR1A3, deassert, restart."]
    PULSE_CONT = 3,
    #[doc = "4: Continuous run (aka Free Run).  Count continuously."]
    CONTINUOUS = 4,
}
impl From<TMRA3FN_A> for u8 {
    #[inline(always)]
    fn from(variant: TMRA3FN_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TMRA3FN`"]
pub type TMRA3FN_R = crate::R<u8, TMRA3FN_A>;
impl TMRA3FN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TMRA3FN_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TMRA3FN_A::SINGLECOUNT),
            1 => Val(TMRA3FN_A::REPEATEDCOUNT),
            2 => Val(TMRA3FN_A::PULSE_ONCE),
            3 => Val(TMRA3FN_A::PULSE_CONT),
            4 => Val(TMRA3FN_A::CONTINUOUS),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SINGLECOUNT`"]
    #[inline(always)]
    pub fn is_singlecount(&self) -> bool {
        *self == TMRA3FN_A::SINGLECOUNT
    }
    #[doc = "Checks if the value of the field is `REPEATEDCOUNT`"]
    #[inline(always)]
    pub fn is_repeatedcount(&self) -> bool {
        *self == TMRA3FN_A::REPEATEDCOUNT
    }
    #[doc = "Checks if the value of the field is `PULSE_ONCE`"]
    #[inline(always)]
    pub fn is_pulse_once(&self) -> bool {
        *self == TMRA3FN_A::PULSE_ONCE
    }
    #[doc = "Checks if the value of the field is `PULSE_CONT`"]
    #[inline(always)]
    pub fn is_pulse_cont(&self) -> bool {
        *self == TMRA3FN_A::PULSE_CONT
    }
    #[doc = "Checks if the value of the field is `CONTINUOUS`"]
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        *self == TMRA3FN_A::CONTINUOUS
    }
}
#[doc = "Write proxy for field `TMRA3FN`"]
pub struct TMRA3FN_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRA3FN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRA3FN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Single count (output toggles and sticks). Count to CMPR0A3, stop."]
    #[inline(always)]
    pub fn singlecount(self) -> &'a mut W {
        self.variant(TMRA3FN_A::SINGLECOUNT)
    }
    #[doc = "Repeated count (periodic 1-clock-cycle-wide pulses). Count to CMPR0A3, restart."]
    #[inline(always)]
    pub fn repeatedcount(self) -> &'a mut W {
        self.variant(TMRA3FN_A::REPEATEDCOUNT)
    }
    #[doc = "Pulse once (aka one-shot). Count to CMPR0A3, assert, count to CMPR1B, deassert, stop."]
    #[inline(always)]
    pub fn pulse_once(self) -> &'a mut W {
        self.variant(TMRA3FN_A::PULSE_ONCE)
    }
    #[doc = "Pulse continously. Count to CMPR0A3, assert, count to CMPR1A3, deassert, restart."]
    #[inline(always)]
    pub fn pulse_cont(self) -> &'a mut W {
        self.variant(TMRA3FN_A::PULSE_CONT)
    }
    #[doc = "Continuous run (aka Free Run). Count continuously."]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut W {
        self.variant(TMRA3FN_A::CONTINUOUS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 6)) | (((value as u32) & 0x07) << 6);
        self.w
    }
}
#[doc = "Counter/Timer A3 Clock Select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TMRA3CLK_A {
    #[doc = "0: Clock source is TMRPINA."]
    TMRPIN = 0,
    #[doc = "1: Clock source is the HFRC"]
    HFRC = 1,
    #[doc = "2: Clock source is HFRC / 8"]
    HFRC_DIV8 = 2,
    #[doc = "3: Clock source is HFRC / 128"]
    HFRC_DIV128 = 3,
    #[doc = "4: Clock source is HFRC / 512"]
    HFRC_DIV512 = 4,
    #[doc = "5: Clock source is HFRC / 2048"]
    HFRC_DIV2K = 5,
    #[doc = "6: Clock source is the XT (uncalibrated)."]
    XT = 6,
    #[doc = "7: Clock source is XT / 2"]
    XT_DIV2 = 7,
    #[doc = "8: Clock source is XT / 16"]
    XT_DIV16 = 8,
    #[doc = "9: Clock source is XT / 256"]
    XT_DIV256 = 9,
    #[doc = "10: Clock source is LFRC / 2"]
    LFRC_DIV2 = 10,
    #[doc = "11: Clock source is LFRC / 32"]
    LFRC_DIV32 = 11,
    #[doc = "12: Clock source is LFRC / 1024"]
    LFRC_DIV1K = 12,
    #[doc = "13: Clock source is LFRC / 16K"]
    LFRC = 13,
    #[doc = "14: Clock source is 100 Hz from the current RTC oscillator."]
    RTC_100HZ = 14,
    #[doc = "15: Clock source is HCLK."]
    HCLK = 15,
    #[doc = "16: Clock source is buck converter stream B."]
    BUCKB = 16,
}
impl From<TMRA3CLK_A> for u8 {
    #[inline(always)]
    fn from(variant: TMRA3CLK_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TMRA3CLK`"]
pub type TMRA3CLK_R = crate::R<u8, TMRA3CLK_A>;
impl TMRA3CLK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TMRA3CLK_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TMRA3CLK_A::TMRPIN),
            1 => Val(TMRA3CLK_A::HFRC),
            2 => Val(TMRA3CLK_A::HFRC_DIV8),
            3 => Val(TMRA3CLK_A::HFRC_DIV128),
            4 => Val(TMRA3CLK_A::HFRC_DIV512),
            5 => Val(TMRA3CLK_A::HFRC_DIV2K),
            6 => Val(TMRA3CLK_A::XT),
            7 => Val(TMRA3CLK_A::XT_DIV2),
            8 => Val(TMRA3CLK_A::XT_DIV16),
            9 => Val(TMRA3CLK_A::XT_DIV256),
            10 => Val(TMRA3CLK_A::LFRC_DIV2),
            11 => Val(TMRA3CLK_A::LFRC_DIV32),
            12 => Val(TMRA3CLK_A::LFRC_DIV1K),
            13 => Val(TMRA3CLK_A::LFRC),
            14 => Val(TMRA3CLK_A::RTC_100HZ),
            15 => Val(TMRA3CLK_A::HCLK),
            16 => Val(TMRA3CLK_A::BUCKB),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TMRPIN`"]
    #[inline(always)]
    pub fn is_tmrpin(&self) -> bool {
        *self == TMRA3CLK_A::TMRPIN
    }
    #[doc = "Checks if the value of the field is `HFRC`"]
    #[inline(always)]
    pub fn is_hfrc(&self) -> bool {
        *self == TMRA3CLK_A::HFRC
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV8`"]
    #[inline(always)]
    pub fn is_hfrc_div8(&self) -> bool {
        *self == TMRA3CLK_A::HFRC_DIV8
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV128`"]
    #[inline(always)]
    pub fn is_hfrc_div128(&self) -> bool {
        *self == TMRA3CLK_A::HFRC_DIV128
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV512`"]
    #[inline(always)]
    pub fn is_hfrc_div512(&self) -> bool {
        *self == TMRA3CLK_A::HFRC_DIV512
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV2K`"]
    #[inline(always)]
    pub fn is_hfrc_div2k(&self) -> bool {
        *self == TMRA3CLK_A::HFRC_DIV2K
    }
    #[doc = "Checks if the value of the field is `XT`"]
    #[inline(always)]
    pub fn is_xt(&self) -> bool {
        *self == TMRA3CLK_A::XT
    }
    #[doc = "Checks if the value of the field is `XT_DIV2`"]
    #[inline(always)]
    pub fn is_xt_div2(&self) -> bool {
        *self == TMRA3CLK_A::XT_DIV2
    }
    #[doc = "Checks if the value of the field is `XT_DIV16`"]
    #[inline(always)]
    pub fn is_xt_div16(&self) -> bool {
        *self == TMRA3CLK_A::XT_DIV16
    }
    #[doc = "Checks if the value of the field is `XT_DIV256`"]
    #[inline(always)]
    pub fn is_xt_div256(&self) -> bool {
        *self == TMRA3CLK_A::XT_DIV256
    }
    #[doc = "Checks if the value of the field is `LFRC_DIV2`"]
    #[inline(always)]
    pub fn is_lfrc_div2(&self) -> bool {
        *self == TMRA3CLK_A::LFRC_DIV2
    }
    #[doc = "Checks if the value of the field is `LFRC_DIV32`"]
    #[inline(always)]
    pub fn is_lfrc_div32(&self) -> bool {
        *self == TMRA3CLK_A::LFRC_DIV32
    }
    #[doc = "Checks if the value of the field is `LFRC_DIV1K`"]
    #[inline(always)]
    pub fn is_lfrc_div1k(&self) -> bool {
        *self == TMRA3CLK_A::LFRC_DIV1K
    }
    #[doc = "Checks if the value of the field is `LFRC`"]
    #[inline(always)]
    pub fn is_lfrc(&self) -> bool {
        *self == TMRA3CLK_A::LFRC
    }
    #[doc = "Checks if the value of the field is `RTC_100HZ`"]
    #[inline(always)]
    pub fn is_rtc_100hz(&self) -> bool {
        *self == TMRA3CLK_A::RTC_100HZ
    }
    #[doc = "Checks if the value of the field is `HCLK`"]
    #[inline(always)]
    pub fn is_hclk(&self) -> bool {
        *self == TMRA3CLK_A::HCLK
    }
    #[doc = "Checks if the value of the field is `BUCKB`"]
    #[inline(always)]
    pub fn is_buckb(&self) -> bool {
        *self == TMRA3CLK_A::BUCKB
    }
}
#[doc = "Write proxy for field `TMRA3CLK`"]
pub struct TMRA3CLK_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRA3CLK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRA3CLK_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Clock source is TMRPINA."]
    #[inline(always)]
    pub fn tmrpin(self) -> &'a mut W {
        self.variant(TMRA3CLK_A::TMRPIN)
    }
    #[doc = "Clock source is the HFRC"]
    #[inline(always)]
    pub fn hfrc(self) -> &'a mut W {
        self.variant(TMRA3CLK_A::HFRC)
    }
    #[doc = "Clock source is HFRC / 8"]
    #[inline(always)]
    pub fn hfrc_div8(self) -> &'a mut W {
        self.variant(TMRA3CLK_A::HFRC_DIV8)
    }
    #[doc = "Clock source is HFRC / 128"]
    #[inline(always)]
    pub fn hfrc_div128(self) -> &'a mut W {
        self.variant(TMRA3CLK_A::HFRC_DIV128)
    }
    #[doc = "Clock source is HFRC / 512"]
    #[inline(always)]
    pub fn hfrc_div512(self) -> &'a mut W {
        self.variant(TMRA3CLK_A::HFRC_DIV512)
    }
    #[doc = "Clock source is HFRC / 2048"]
    #[inline(always)]
    pub fn hfrc_div2k(self) -> &'a mut W {
        self.variant(TMRA3CLK_A::HFRC_DIV2K)
    }
    #[doc = "Clock source is the XT (uncalibrated)."]
    #[inline(always)]
    pub fn xt(self) -> &'a mut W {
        self.variant(TMRA3CLK_A::XT)
    }
    #[doc = "Clock source is XT / 2"]
    #[inline(always)]
    pub fn xt_div2(self) -> &'a mut W {
        self.variant(TMRA3CLK_A::XT_DIV2)
    }
    #[doc = "Clock source is XT / 16"]
    #[inline(always)]
    pub fn xt_div16(self) -> &'a mut W {
        self.variant(TMRA3CLK_A::XT_DIV16)
    }
    #[doc = "Clock source is XT / 256"]
    #[inline(always)]
    pub fn xt_div256(self) -> &'a mut W {
        self.variant(TMRA3CLK_A::XT_DIV256)
    }
    #[doc = "Clock source is LFRC / 2"]
    #[inline(always)]
    pub fn lfrc_div2(self) -> &'a mut W {
        self.variant(TMRA3CLK_A::LFRC_DIV2)
    }
    #[doc = "Clock source is LFRC / 32"]
    #[inline(always)]
    pub fn lfrc_div32(self) -> &'a mut W {
        self.variant(TMRA3CLK_A::LFRC_DIV32)
    }
    #[doc = "Clock source is LFRC / 1024"]
    #[inline(always)]
    pub fn lfrc_div1k(self) -> &'a mut W {
        self.variant(TMRA3CLK_A::LFRC_DIV1K)
    }
    #[doc = "Clock source is LFRC / 16K"]
    #[inline(always)]
    pub fn lfrc(self) -> &'a mut W {
        self.variant(TMRA3CLK_A::LFRC)
    }
    #[doc = "Clock source is 100 Hz from the current RTC oscillator."]
    #[inline(always)]
    pub fn rtc_100hz(self) -> &'a mut W {
        self.variant(TMRA3CLK_A::RTC_100HZ)
    }
    #[doc = "Clock source is HCLK."]
    #[inline(always)]
    pub fn hclk(self) -> &'a mut W {
        self.variant(TMRA3CLK_A::HCLK)
    }
    #[doc = "Clock source is buck converter stream B."]
    #[inline(always)]
    pub fn buckb(self) -> &'a mut W {
        self.variant(TMRA3CLK_A::BUCKB)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 1)) | (((value as u32) & 0x1f) << 1);
        self.w
    }
}
#[doc = "Counter/Timer A3 Enable bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA3EN_A {
    #[doc = "0: Counter/Timer A3 Disable."]
    DIS = 0,
    #[doc = "1: Counter/Timer A3 Enable."]
    EN = 1,
}
impl From<TMRA3EN_A> for bool {
    #[inline(always)]
    fn from(variant: TMRA3EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TMRA3EN`"]
pub type TMRA3EN_R = crate::R<bool, TMRA3EN_A>;
impl TMRA3EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRA3EN_A {
        match self.bits {
            false => TMRA3EN_A::DIS,
            true => TMRA3EN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TMRA3EN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == TMRA3EN_A::EN
    }
}
#[doc = "Write proxy for field `TMRA3EN`"]
pub struct TMRA3EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRA3EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRA3EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Counter/Timer A3 Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRA3EN_A::DIS)
    }
    #[doc = "Counter/Timer A3 Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRA3EN_A::EN)
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
    #[doc = "Bit 31 - Counter/Timer A/B Link bit."]
    #[inline(always)]
    pub fn ctlink3(&self) -> CTLINK3_R {
        CTLINK3_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Counter/Timer B3 output polarity."]
    #[inline(always)]
    pub fn tmrb3pol(&self) -> TMRB3POL_R {
        TMRB3POL_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Counter/Timer B3 Clear bit."]
    #[inline(always)]
    pub fn tmrb3clr(&self) -> TMRB3CLR_R {
        TMRB3CLR_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Counter/Timer B3 Output Enable bit."]
    #[inline(always)]
    pub fn tmrb3pe(&self) -> TMRB3PE_R {
        TMRB3PE_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Counter/Timer B3 Interrupt Enable bit."]
    #[inline(always)]
    pub fn tmrb3ie(&self) -> TMRB3IE_R {
        TMRB3IE_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bits 22:24 - Counter/Timer B3 Function Select."]
    #[inline(always)]
    pub fn tmrb3fn(&self) -> TMRB3FN_R {
        TMRB3FN_R::new(((self.bits >> 22) & 0x07) as u8)
    }
    #[doc = "Bits 17:21 - Counter/Timer B3 Clock Select."]
    #[inline(always)]
    pub fn tmrb3clk(&self) -> TMRB3CLK_R {
        TMRB3CLK_R::new(((self.bits >> 17) & 0x1f) as u8)
    }
    #[doc = "Bit 16 - Counter/Timer B3 Enable bit."]
    #[inline(always)]
    pub fn tmrb3en(&self) -> TMRB3EN_R {
        TMRB3EN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Special Timer A3 enable for ADC function."]
    #[inline(always)]
    pub fn adcen(&self) -> ADCEN_R {
        ADCEN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Counter/Timer A3 output polarity."]
    #[inline(always)]
    pub fn tmra3pol(&self) -> TMRA3POL_R {
        TMRA3POL_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Counter/Timer A3 Clear bit."]
    #[inline(always)]
    pub fn tmra3clr(&self) -> TMRA3CLR_R {
        TMRA3CLR_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Counter/Timer A3 Output Enable bit."]
    #[inline(always)]
    pub fn tmra3pe(&self) -> TMRA3PE_R {
        TMRA3PE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Counter/Timer A3 Interrupt Enable bit."]
    #[inline(always)]
    pub fn tmra3ie(&self) -> TMRA3IE_R {
        TMRA3IE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bits 6:8 - Counter/Timer A3 Function Select."]
    #[inline(always)]
    pub fn tmra3fn(&self) -> TMRA3FN_R {
        TMRA3FN_R::new(((self.bits >> 6) & 0x07) as u8)
    }
    #[doc = "Bits 1:5 - Counter/Timer A3 Clock Select."]
    #[inline(always)]
    pub fn tmra3clk(&self) -> TMRA3CLK_R {
        TMRA3CLK_R::new(((self.bits >> 1) & 0x1f) as u8)
    }
    #[doc = "Bit 0 - Counter/Timer A3 Enable bit."]
    #[inline(always)]
    pub fn tmra3en(&self) -> TMRA3EN_R {
        TMRA3EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - Counter/Timer A/B Link bit."]
    #[inline(always)]
    pub fn ctlink3(&mut self) -> CTLINK3_W {
        CTLINK3_W { w: self }
    }
    #[doc = "Bit 28 - Counter/Timer B3 output polarity."]
    #[inline(always)]
    pub fn tmrb3pol(&mut self) -> TMRB3POL_W {
        TMRB3POL_W { w: self }
    }
    #[doc = "Bit 27 - Counter/Timer B3 Clear bit."]
    #[inline(always)]
    pub fn tmrb3clr(&mut self) -> TMRB3CLR_W {
        TMRB3CLR_W { w: self }
    }
    #[doc = "Bit 26 - Counter/Timer B3 Output Enable bit."]
    #[inline(always)]
    pub fn tmrb3pe(&mut self) -> TMRB3PE_W {
        TMRB3PE_W { w: self }
    }
    #[doc = "Bit 25 - Counter/Timer B3 Interrupt Enable bit."]
    #[inline(always)]
    pub fn tmrb3ie(&mut self) -> TMRB3IE_W {
        TMRB3IE_W { w: self }
    }
    #[doc = "Bits 22:24 - Counter/Timer B3 Function Select."]
    #[inline(always)]
    pub fn tmrb3fn(&mut self) -> TMRB3FN_W {
        TMRB3FN_W { w: self }
    }
    #[doc = "Bits 17:21 - Counter/Timer B3 Clock Select."]
    #[inline(always)]
    pub fn tmrb3clk(&mut self) -> TMRB3CLK_W {
        TMRB3CLK_W { w: self }
    }
    #[doc = "Bit 16 - Counter/Timer B3 Enable bit."]
    #[inline(always)]
    pub fn tmrb3en(&mut self) -> TMRB3EN_W {
        TMRB3EN_W { w: self }
    }
    #[doc = "Bit 15 - Special Timer A3 enable for ADC function."]
    #[inline(always)]
    pub fn adcen(&mut self) -> ADCEN_W {
        ADCEN_W { w: self }
    }
    #[doc = "Bit 12 - Counter/Timer A3 output polarity."]
    #[inline(always)]
    pub fn tmra3pol(&mut self) -> TMRA3POL_W {
        TMRA3POL_W { w: self }
    }
    #[doc = "Bit 11 - Counter/Timer A3 Clear bit."]
    #[inline(always)]
    pub fn tmra3clr(&mut self) -> TMRA3CLR_W {
        TMRA3CLR_W { w: self }
    }
    #[doc = "Bit 10 - Counter/Timer A3 Output Enable bit."]
    #[inline(always)]
    pub fn tmra3pe(&mut self) -> TMRA3PE_W {
        TMRA3PE_W { w: self }
    }
    #[doc = "Bit 9 - Counter/Timer A3 Interrupt Enable bit."]
    #[inline(always)]
    pub fn tmra3ie(&mut self) -> TMRA3IE_W {
        TMRA3IE_W { w: self }
    }
    #[doc = "Bits 6:8 - Counter/Timer A3 Function Select."]
    #[inline(always)]
    pub fn tmra3fn(&mut self) -> TMRA3FN_W {
        TMRA3FN_W { w: self }
    }
    #[doc = "Bits 1:5 - Counter/Timer A3 Clock Select."]
    #[inline(always)]
    pub fn tmra3clk(&mut self) -> TMRA3CLK_W {
        TMRA3CLK_W { w: self }
    }
    #[doc = "Bit 0 - Counter/Timer A3 Enable bit."]
    #[inline(always)]
    pub fn tmra3en(&mut self) -> TMRA3EN_W {
        TMRA3EN_W { w: self }
    }
}
