#[doc = "Reader of register CTRL0"]
pub type R = crate::R<u32, super::CTRL0>;
#[doc = "Writer for register CTRL0"]
pub type W = crate::W<u32, super::CTRL0>;
#[doc = "Register CTRL0 `reset()`'s with value 0"]
impl crate::ResetValue for super::CTRL0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Counter/Timer A0/B0 Link bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTLINK0_A {
    #[doc = "0: Use A0/B0 timers as two independent 16-bit timers (default)."]
    TWO_16BIT_TIMERS = 0,
    #[doc = "1: Link A0/B0 timers into a single 32-bit timer."]
    _32BIT_TIMER = 1,
}
impl From<CTLINK0_A> for bool {
    #[inline(always)]
    fn from(variant: CTLINK0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CTLINK0`"]
pub type CTLINK0_R = crate::R<bool, CTLINK0_A>;
impl CTLINK0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTLINK0_A {
        match self.bits {
            false => CTLINK0_A::TWO_16BIT_TIMERS,
            true => CTLINK0_A::_32BIT_TIMER,
        }
    }
    #[doc = "Checks if the value of the field is `TWO_16BIT_TIMERS`"]
    #[inline(always)]
    pub fn is_two_16bit_timers(&self) -> bool {
        *self == CTLINK0_A::TWO_16BIT_TIMERS
    }
    #[doc = "Checks if the value of the field is `_32BIT_TIMER`"]
    #[inline(always)]
    pub fn is_32bit_timer(&self) -> bool {
        *self == CTLINK0_A::_32BIT_TIMER
    }
}
#[doc = "Write proxy for field `CTLINK0`"]
pub struct CTLINK0_W<'a> {
    w: &'a mut W,
}
impl<'a> CTLINK0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTLINK0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Use A0/B0 timers as two independent 16-bit timers (default)."]
    #[inline(always)]
    pub fn two_16bit_timers(self) -> &'a mut W {
        self.variant(CTLINK0_A::TWO_16BIT_TIMERS)
    }
    #[doc = "Link A0/B0 timers into a single 32-bit timer."]
    #[inline(always)]
    pub fn _32bit_timer(self) -> &'a mut W {
        self.variant(CTLINK0_A::_32BIT_TIMER)
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
#[doc = "Counter/Timer B0 output polarity.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB0POL_A {
    #[doc = "0: The polarity of the TMRPINB0 pin is the same as the timer output."]
    NORMAL = 0,
    #[doc = "1: The polarity of the TMRPINB0 pin is the inverse of the timer output."]
    INVERTED = 1,
}
impl From<TMRB0POL_A> for bool {
    #[inline(always)]
    fn from(variant: TMRB0POL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TMRB0POL`"]
pub type TMRB0POL_R = crate::R<bool, TMRB0POL_A>;
impl TMRB0POL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRB0POL_A {
        match self.bits {
            false => TMRB0POL_A::NORMAL,
            true => TMRB0POL_A::INVERTED,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == TMRB0POL_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `INVERTED`"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == TMRB0POL_A::INVERTED
    }
}
#[doc = "Write proxy for field `TMRB0POL`"]
pub struct TMRB0POL_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRB0POL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRB0POL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The polarity of the TMRPINB0 pin is the same as the timer output."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(TMRB0POL_A::NORMAL)
    }
    #[doc = "The polarity of the TMRPINB0 pin is the inverse of the timer output."]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut W {
        self.variant(TMRB0POL_A::INVERTED)
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
#[doc = "Counter/Timer B0 Clear bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB0CLR_A {
    #[doc = "0: Allow counter/timer B0 to run"]
    RUN = 0,
    #[doc = "1: Holds counter/timer B0 at 0x0000."]
    CLEAR = 1,
}
impl From<TMRB0CLR_A> for bool {
    #[inline(always)]
    fn from(variant: TMRB0CLR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TMRB0CLR`"]
pub type TMRB0CLR_R = crate::R<bool, TMRB0CLR_A>;
impl TMRB0CLR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRB0CLR_A {
        match self.bits {
            false => TMRB0CLR_A::RUN,
            true => TMRB0CLR_A::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `RUN`"]
    #[inline(always)]
    pub fn is_run(&self) -> bool {
        *self == TMRB0CLR_A::RUN
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == TMRB0CLR_A::CLEAR
    }
}
#[doc = "Write proxy for field `TMRB0CLR`"]
pub struct TMRB0CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRB0CLR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRB0CLR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Allow counter/timer B0 to run"]
    #[inline(always)]
    pub fn run(self) -> &'a mut W {
        self.variant(TMRB0CLR_A::RUN)
    }
    #[doc = "Holds counter/timer B0 at 0x0000."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(TMRB0CLR_A::CLEAR)
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
#[doc = "Counter/Timer B0 Output Enable bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB0PE_A {
    #[doc = "0: Counter/Timer B holds the TMRPINB signal at the value TMRB0POL."]
    DIS = 0,
    #[doc = "1: Enable counter/timer B0 to generate a signal on TMRPINB."]
    EN = 1,
}
impl From<TMRB0PE_A> for bool {
    #[inline(always)]
    fn from(variant: TMRB0PE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TMRB0PE`"]
pub type TMRB0PE_R = crate::R<bool, TMRB0PE_A>;
impl TMRB0PE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRB0PE_A {
        match self.bits {
            false => TMRB0PE_A::DIS,
            true => TMRB0PE_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TMRB0PE_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == TMRB0PE_A::EN
    }
}
#[doc = "Write proxy for field `TMRB0PE`"]
pub struct TMRB0PE_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRB0PE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRB0PE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Counter/Timer B holds the TMRPINB signal at the value TMRB0POL."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRB0PE_A::DIS)
    }
    #[doc = "Enable counter/timer B0 to generate a signal on TMRPINB."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRB0PE_A::EN)
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
#[doc = "Counter/Timer B0 Interrupt Enable bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB0IE_A {
    #[doc = "0: Disable counter/timer B0 from generating an interrupt."]
    DIS = 0,
    #[doc = "1: Enable counter/timer B0 to generate an interrupt."]
    EN = 1,
}
impl From<TMRB0IE_A> for bool {
    #[inline(always)]
    fn from(variant: TMRB0IE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TMRB0IE`"]
pub type TMRB0IE_R = crate::R<bool, TMRB0IE_A>;
impl TMRB0IE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRB0IE_A {
        match self.bits {
            false => TMRB0IE_A::DIS,
            true => TMRB0IE_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TMRB0IE_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == TMRB0IE_A::EN
    }
}
#[doc = "Write proxy for field `TMRB0IE`"]
pub struct TMRB0IE_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRB0IE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRB0IE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable counter/timer B0 from generating an interrupt."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRB0IE_A::DIS)
    }
    #[doc = "Enable counter/timer B0 to generate an interrupt."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRB0IE_A::EN)
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
#[doc = "Counter/Timer B0 Function Select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TMRB0FN_A {
    #[doc = "0: Single count (output toggles and sticks).  Count to CMPR0B0, stop."]
    SINGLECOUNT = 0,
    #[doc = "1: Repeated count (periodic 1-clock-cycle-wide pulses).  Count to CMPR0B0, restart."]
    REPEATEDCOUNT = 1,
    #[doc = "2: Pulse once (aka one-shot).  Count to CMPR0B0, assert, count to CMPR1B, deassert, stop."]
    PULSE_ONCE = 2,
    #[doc = "3: Pulse continously.  Count to CMPR0B0, assert, count to CMPR1B0, deassert, restart."]
    PULSE_CONT = 3,
    #[doc = "4: Continuous run (aka Free Run).  Count continuously."]
    CONTINUOUS = 4,
}
impl From<TMRB0FN_A> for u8 {
    #[inline(always)]
    fn from(variant: TMRB0FN_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TMRB0FN`"]
pub type TMRB0FN_R = crate::R<u8, TMRB0FN_A>;
impl TMRB0FN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TMRB0FN_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TMRB0FN_A::SINGLECOUNT),
            1 => Val(TMRB0FN_A::REPEATEDCOUNT),
            2 => Val(TMRB0FN_A::PULSE_ONCE),
            3 => Val(TMRB0FN_A::PULSE_CONT),
            4 => Val(TMRB0FN_A::CONTINUOUS),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SINGLECOUNT`"]
    #[inline(always)]
    pub fn is_singlecount(&self) -> bool {
        *self == TMRB0FN_A::SINGLECOUNT
    }
    #[doc = "Checks if the value of the field is `REPEATEDCOUNT`"]
    #[inline(always)]
    pub fn is_repeatedcount(&self) -> bool {
        *self == TMRB0FN_A::REPEATEDCOUNT
    }
    #[doc = "Checks if the value of the field is `PULSE_ONCE`"]
    #[inline(always)]
    pub fn is_pulse_once(&self) -> bool {
        *self == TMRB0FN_A::PULSE_ONCE
    }
    #[doc = "Checks if the value of the field is `PULSE_CONT`"]
    #[inline(always)]
    pub fn is_pulse_cont(&self) -> bool {
        *self == TMRB0FN_A::PULSE_CONT
    }
    #[doc = "Checks if the value of the field is `CONTINUOUS`"]
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        *self == TMRB0FN_A::CONTINUOUS
    }
}
#[doc = "Write proxy for field `TMRB0FN`"]
pub struct TMRB0FN_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRB0FN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRB0FN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Single count (output toggles and sticks). Count to CMPR0B0, stop."]
    #[inline(always)]
    pub fn singlecount(self) -> &'a mut W {
        self.variant(TMRB0FN_A::SINGLECOUNT)
    }
    #[doc = "Repeated count (periodic 1-clock-cycle-wide pulses). Count to CMPR0B0, restart."]
    #[inline(always)]
    pub fn repeatedcount(self) -> &'a mut W {
        self.variant(TMRB0FN_A::REPEATEDCOUNT)
    }
    #[doc = "Pulse once (aka one-shot). Count to CMPR0B0, assert, count to CMPR1B, deassert, stop."]
    #[inline(always)]
    pub fn pulse_once(self) -> &'a mut W {
        self.variant(TMRB0FN_A::PULSE_ONCE)
    }
    #[doc = "Pulse continously. Count to CMPR0B0, assert, count to CMPR1B0, deassert, restart."]
    #[inline(always)]
    pub fn pulse_cont(self) -> &'a mut W {
        self.variant(TMRB0FN_A::PULSE_CONT)
    }
    #[doc = "Continuous run (aka Free Run). Count continuously."]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut W {
        self.variant(TMRB0FN_A::CONTINUOUS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 22)) | (((value as u32) & 0x07) << 22);
        self.w
    }
}
#[doc = "Counter/Timer B0 Clock Select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TMRB0CLK_A {
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
    #[doc = "16: Clock source is buck converter stream B."]
    BUCKB = 16,
}
impl From<TMRB0CLK_A> for u8 {
    #[inline(always)]
    fn from(variant: TMRB0CLK_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TMRB0CLK`"]
pub type TMRB0CLK_R = crate::R<u8, TMRB0CLK_A>;
impl TMRB0CLK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TMRB0CLK_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TMRB0CLK_A::TMRPIN),
            1 => Val(TMRB0CLK_A::HFRC),
            2 => Val(TMRB0CLK_A::HFRC_DIV8),
            3 => Val(TMRB0CLK_A::HFRC_DIV128),
            4 => Val(TMRB0CLK_A::HFRC_DIV512),
            5 => Val(TMRB0CLK_A::HFRC_DIV2K),
            6 => Val(TMRB0CLK_A::XT),
            7 => Val(TMRB0CLK_A::XT_DIV2),
            8 => Val(TMRB0CLK_A::XT_DIV16),
            9 => Val(TMRB0CLK_A::XT_DIV256),
            10 => Val(TMRB0CLK_A::LFRC_DIV2),
            11 => Val(TMRB0CLK_A::LFRC_DIV32),
            12 => Val(TMRB0CLK_A::LFRC_DIV1K),
            13 => Val(TMRB0CLK_A::LFRC),
            14 => Val(TMRB0CLK_A::RTC_100HZ),
            15 => Val(TMRB0CLK_A::HCLK),
            16 => Val(TMRB0CLK_A::BUCKB),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TMRPIN`"]
    #[inline(always)]
    pub fn is_tmrpin(&self) -> bool {
        *self == TMRB0CLK_A::TMRPIN
    }
    #[doc = "Checks if the value of the field is `HFRC`"]
    #[inline(always)]
    pub fn is_hfrc(&self) -> bool {
        *self == TMRB0CLK_A::HFRC
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV8`"]
    #[inline(always)]
    pub fn is_hfrc_div8(&self) -> bool {
        *self == TMRB0CLK_A::HFRC_DIV8
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV128`"]
    #[inline(always)]
    pub fn is_hfrc_div128(&self) -> bool {
        *self == TMRB0CLK_A::HFRC_DIV128
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV512`"]
    #[inline(always)]
    pub fn is_hfrc_div512(&self) -> bool {
        *self == TMRB0CLK_A::HFRC_DIV512
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV2K`"]
    #[inline(always)]
    pub fn is_hfrc_div2k(&self) -> bool {
        *self == TMRB0CLK_A::HFRC_DIV2K
    }
    #[doc = "Checks if the value of the field is `XT`"]
    #[inline(always)]
    pub fn is_xt(&self) -> bool {
        *self == TMRB0CLK_A::XT
    }
    #[doc = "Checks if the value of the field is `XT_DIV2`"]
    #[inline(always)]
    pub fn is_xt_div2(&self) -> bool {
        *self == TMRB0CLK_A::XT_DIV2
    }
    #[doc = "Checks if the value of the field is `XT_DIV16`"]
    #[inline(always)]
    pub fn is_xt_div16(&self) -> bool {
        *self == TMRB0CLK_A::XT_DIV16
    }
    #[doc = "Checks if the value of the field is `XT_DIV256`"]
    #[inline(always)]
    pub fn is_xt_div256(&self) -> bool {
        *self == TMRB0CLK_A::XT_DIV256
    }
    #[doc = "Checks if the value of the field is `LFRC_DIV2`"]
    #[inline(always)]
    pub fn is_lfrc_div2(&self) -> bool {
        *self == TMRB0CLK_A::LFRC_DIV2
    }
    #[doc = "Checks if the value of the field is `LFRC_DIV32`"]
    #[inline(always)]
    pub fn is_lfrc_div32(&self) -> bool {
        *self == TMRB0CLK_A::LFRC_DIV32
    }
    #[doc = "Checks if the value of the field is `LFRC_DIV1K`"]
    #[inline(always)]
    pub fn is_lfrc_div1k(&self) -> bool {
        *self == TMRB0CLK_A::LFRC_DIV1K
    }
    #[doc = "Checks if the value of the field is `LFRC`"]
    #[inline(always)]
    pub fn is_lfrc(&self) -> bool {
        *self == TMRB0CLK_A::LFRC
    }
    #[doc = "Checks if the value of the field is `RTC_100HZ`"]
    #[inline(always)]
    pub fn is_rtc_100hz(&self) -> bool {
        *self == TMRB0CLK_A::RTC_100HZ
    }
    #[doc = "Checks if the value of the field is `HCLK`"]
    #[inline(always)]
    pub fn is_hclk(&self) -> bool {
        *self == TMRB0CLK_A::HCLK
    }
    #[doc = "Checks if the value of the field is `BUCKB`"]
    #[inline(always)]
    pub fn is_buckb(&self) -> bool {
        *self == TMRB0CLK_A::BUCKB
    }
}
#[doc = "Write proxy for field `TMRB0CLK`"]
pub struct TMRB0CLK_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRB0CLK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRB0CLK_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Clock source is TMRPINB."]
    #[inline(always)]
    pub fn tmrpin(self) -> &'a mut W {
        self.variant(TMRB0CLK_A::TMRPIN)
    }
    #[doc = "Clock source is the HFRC"]
    #[inline(always)]
    pub fn hfrc(self) -> &'a mut W {
        self.variant(TMRB0CLK_A::HFRC)
    }
    #[doc = "Clock source is HFRC / 8"]
    #[inline(always)]
    pub fn hfrc_div8(self) -> &'a mut W {
        self.variant(TMRB0CLK_A::HFRC_DIV8)
    }
    #[doc = "Clock source is HFRC / 128"]
    #[inline(always)]
    pub fn hfrc_div128(self) -> &'a mut W {
        self.variant(TMRB0CLK_A::HFRC_DIV128)
    }
    #[doc = "Clock source is HFRC / 512"]
    #[inline(always)]
    pub fn hfrc_div512(self) -> &'a mut W {
        self.variant(TMRB0CLK_A::HFRC_DIV512)
    }
    #[doc = "Clock source is HFRC / 2048"]
    #[inline(always)]
    pub fn hfrc_div2k(self) -> &'a mut W {
        self.variant(TMRB0CLK_A::HFRC_DIV2K)
    }
    #[doc = "Clock source is the XT (uncalibrated)."]
    #[inline(always)]
    pub fn xt(self) -> &'a mut W {
        self.variant(TMRB0CLK_A::XT)
    }
    #[doc = "Clock source is XT / 2"]
    #[inline(always)]
    pub fn xt_div2(self) -> &'a mut W {
        self.variant(TMRB0CLK_A::XT_DIV2)
    }
    #[doc = "Clock source is XT / 16"]
    #[inline(always)]
    pub fn xt_div16(self) -> &'a mut W {
        self.variant(TMRB0CLK_A::XT_DIV16)
    }
    #[doc = "Clock source is XT / 256"]
    #[inline(always)]
    pub fn xt_div256(self) -> &'a mut W {
        self.variant(TMRB0CLK_A::XT_DIV256)
    }
    #[doc = "Clock source is LFRC / 2"]
    #[inline(always)]
    pub fn lfrc_div2(self) -> &'a mut W {
        self.variant(TMRB0CLK_A::LFRC_DIV2)
    }
    #[doc = "Clock source is LFRC / 32"]
    #[inline(always)]
    pub fn lfrc_div32(self) -> &'a mut W {
        self.variant(TMRB0CLK_A::LFRC_DIV32)
    }
    #[doc = "Clock source is LFRC / 1024"]
    #[inline(always)]
    pub fn lfrc_div1k(self) -> &'a mut W {
        self.variant(TMRB0CLK_A::LFRC_DIV1K)
    }
    #[doc = "Clock source is LFRC / 16K"]
    #[inline(always)]
    pub fn lfrc(self) -> &'a mut W {
        self.variant(TMRB0CLK_A::LFRC)
    }
    #[doc = "Clock source is 100 Hz from the current RTC oscillator."]
    #[inline(always)]
    pub fn rtc_100hz(self) -> &'a mut W {
        self.variant(TMRB0CLK_A::RTC_100HZ)
    }
    #[doc = "Clock source is HCLK."]
    #[inline(always)]
    pub fn hclk(self) -> &'a mut W {
        self.variant(TMRB0CLK_A::HCLK)
    }
    #[doc = "Clock source is buck converter stream B."]
    #[inline(always)]
    pub fn buckb(self) -> &'a mut W {
        self.variant(TMRB0CLK_A::BUCKB)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 17)) | (((value as u32) & 0x1f) << 17);
        self.w
    }
}
#[doc = "Counter/Timer B0 Enable bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB0EN_A {
    #[doc = "0: Counter/Timer B0 Disable."]
    DIS = 0,
    #[doc = "1: Counter/Timer B0 Enable."]
    EN = 1,
}
impl From<TMRB0EN_A> for bool {
    #[inline(always)]
    fn from(variant: TMRB0EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TMRB0EN`"]
pub type TMRB0EN_R = crate::R<bool, TMRB0EN_A>;
impl TMRB0EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRB0EN_A {
        match self.bits {
            false => TMRB0EN_A::DIS,
            true => TMRB0EN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TMRB0EN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == TMRB0EN_A::EN
    }
}
#[doc = "Write proxy for field `TMRB0EN`"]
pub struct TMRB0EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRB0EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRB0EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Counter/Timer B0 Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRB0EN_A::DIS)
    }
    #[doc = "Counter/Timer B0 Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRB0EN_A::EN)
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
#[doc = "Counter/Timer A0 output polarity.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA0POL_A {
    #[doc = "0: The polarity of the TMRPINA0 pin is the same as the timer output."]
    NORMAL = 0,
    #[doc = "1: The polarity of the TMRPINA0 pin is the inverse of the timer output."]
    INVERTED = 1,
}
impl From<TMRA0POL_A> for bool {
    #[inline(always)]
    fn from(variant: TMRA0POL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TMRA0POL`"]
pub type TMRA0POL_R = crate::R<bool, TMRA0POL_A>;
impl TMRA0POL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRA0POL_A {
        match self.bits {
            false => TMRA0POL_A::NORMAL,
            true => TMRA0POL_A::INVERTED,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == TMRA0POL_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `INVERTED`"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == TMRA0POL_A::INVERTED
    }
}
#[doc = "Write proxy for field `TMRA0POL`"]
pub struct TMRA0POL_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRA0POL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRA0POL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The polarity of the TMRPINA0 pin is the same as the timer output."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(TMRA0POL_A::NORMAL)
    }
    #[doc = "The polarity of the TMRPINA0 pin is the inverse of the timer output."]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut W {
        self.variant(TMRA0POL_A::INVERTED)
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
#[doc = "Counter/Timer A0 Clear bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA0CLR_A {
    #[doc = "0: Allow counter/timer A0 to run"]
    RUN = 0,
    #[doc = "1: Holds counter/timer A0 at 0x0000."]
    CLEAR = 1,
}
impl From<TMRA0CLR_A> for bool {
    #[inline(always)]
    fn from(variant: TMRA0CLR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TMRA0CLR`"]
pub type TMRA0CLR_R = crate::R<bool, TMRA0CLR_A>;
impl TMRA0CLR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRA0CLR_A {
        match self.bits {
            false => TMRA0CLR_A::RUN,
            true => TMRA0CLR_A::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `RUN`"]
    #[inline(always)]
    pub fn is_run(&self) -> bool {
        *self == TMRA0CLR_A::RUN
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == TMRA0CLR_A::CLEAR
    }
}
#[doc = "Write proxy for field `TMRA0CLR`"]
pub struct TMRA0CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRA0CLR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRA0CLR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Allow counter/timer A0 to run"]
    #[inline(always)]
    pub fn run(self) -> &'a mut W {
        self.variant(TMRA0CLR_A::RUN)
    }
    #[doc = "Holds counter/timer A0 at 0x0000."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(TMRA0CLR_A::CLEAR)
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
#[doc = "Counter/Timer A0 Output Enable bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA0PE_A {
    #[doc = "0: Counter/Timer A holds the TMRPINA signal at the value TMRA0POL."]
    DIS = 0,
    #[doc = "1: Enable counter/timer B0 to generate a signal on TMRPINB."]
    EN = 1,
}
impl From<TMRA0PE_A> for bool {
    #[inline(always)]
    fn from(variant: TMRA0PE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TMRA0PE`"]
pub type TMRA0PE_R = crate::R<bool, TMRA0PE_A>;
impl TMRA0PE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRA0PE_A {
        match self.bits {
            false => TMRA0PE_A::DIS,
            true => TMRA0PE_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TMRA0PE_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == TMRA0PE_A::EN
    }
}
#[doc = "Write proxy for field `TMRA0PE`"]
pub struct TMRA0PE_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRA0PE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRA0PE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Counter/Timer A holds the TMRPINA signal at the value TMRA0POL."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRA0PE_A::DIS)
    }
    #[doc = "Enable counter/timer B0 to generate a signal on TMRPINB."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRA0PE_A::EN)
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
#[doc = "Counter/Timer A0 Interrupt Enable bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA0IE_A {
    #[doc = "0: Disable counter/timer A0 from generating an interrupt."]
    DIS = 0,
    #[doc = "1: Enable counter/timer A0 to generate an interrupt."]
    EN = 1,
}
impl From<TMRA0IE_A> for bool {
    #[inline(always)]
    fn from(variant: TMRA0IE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TMRA0IE`"]
pub type TMRA0IE_R = crate::R<bool, TMRA0IE_A>;
impl TMRA0IE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRA0IE_A {
        match self.bits {
            false => TMRA0IE_A::DIS,
            true => TMRA0IE_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TMRA0IE_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == TMRA0IE_A::EN
    }
}
#[doc = "Write proxy for field `TMRA0IE`"]
pub struct TMRA0IE_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRA0IE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRA0IE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable counter/timer A0 from generating an interrupt."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRA0IE_A::DIS)
    }
    #[doc = "Enable counter/timer A0 to generate an interrupt."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRA0IE_A::EN)
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
#[doc = "Counter/Timer A0 Function Select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TMRA0FN_A {
    #[doc = "0: Single count (output toggles and sticks).  Count to CMPR0A0, stop."]
    SINGLECOUNT = 0,
    #[doc = "1: Repeated count (periodic 1-clock-cycle-wide pulses).  Count to CMPR0A0, restart."]
    REPEATEDCOUNT = 1,
    #[doc = "2: Pulse once (aka one-shot).  Count to CMPR0A0, assert, count to CMPR1B, deassert, stop."]
    PULSE_ONCE = 2,
    #[doc = "3: Pulse continously.  Count to CMPR0A0, assert, count to CMPR1A0, deassert, restart."]
    PULSE_CONT = 3,
    #[doc = "4: Continuous run (aka Free Run).  Count continuously."]
    CONTINUOUS = 4,
}
impl From<TMRA0FN_A> for u8 {
    #[inline(always)]
    fn from(variant: TMRA0FN_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TMRA0FN`"]
pub type TMRA0FN_R = crate::R<u8, TMRA0FN_A>;
impl TMRA0FN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TMRA0FN_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TMRA0FN_A::SINGLECOUNT),
            1 => Val(TMRA0FN_A::REPEATEDCOUNT),
            2 => Val(TMRA0FN_A::PULSE_ONCE),
            3 => Val(TMRA0FN_A::PULSE_CONT),
            4 => Val(TMRA0FN_A::CONTINUOUS),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SINGLECOUNT`"]
    #[inline(always)]
    pub fn is_singlecount(&self) -> bool {
        *self == TMRA0FN_A::SINGLECOUNT
    }
    #[doc = "Checks if the value of the field is `REPEATEDCOUNT`"]
    #[inline(always)]
    pub fn is_repeatedcount(&self) -> bool {
        *self == TMRA0FN_A::REPEATEDCOUNT
    }
    #[doc = "Checks if the value of the field is `PULSE_ONCE`"]
    #[inline(always)]
    pub fn is_pulse_once(&self) -> bool {
        *self == TMRA0FN_A::PULSE_ONCE
    }
    #[doc = "Checks if the value of the field is `PULSE_CONT`"]
    #[inline(always)]
    pub fn is_pulse_cont(&self) -> bool {
        *self == TMRA0FN_A::PULSE_CONT
    }
    #[doc = "Checks if the value of the field is `CONTINUOUS`"]
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        *self == TMRA0FN_A::CONTINUOUS
    }
}
#[doc = "Write proxy for field `TMRA0FN`"]
pub struct TMRA0FN_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRA0FN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRA0FN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Single count (output toggles and sticks). Count to CMPR0A0, stop."]
    #[inline(always)]
    pub fn singlecount(self) -> &'a mut W {
        self.variant(TMRA0FN_A::SINGLECOUNT)
    }
    #[doc = "Repeated count (periodic 1-clock-cycle-wide pulses). Count to CMPR0A0, restart."]
    #[inline(always)]
    pub fn repeatedcount(self) -> &'a mut W {
        self.variant(TMRA0FN_A::REPEATEDCOUNT)
    }
    #[doc = "Pulse once (aka one-shot). Count to CMPR0A0, assert, count to CMPR1B, deassert, stop."]
    #[inline(always)]
    pub fn pulse_once(self) -> &'a mut W {
        self.variant(TMRA0FN_A::PULSE_ONCE)
    }
    #[doc = "Pulse continously. Count to CMPR0A0, assert, count to CMPR1A0, deassert, restart."]
    #[inline(always)]
    pub fn pulse_cont(self) -> &'a mut W {
        self.variant(TMRA0FN_A::PULSE_CONT)
    }
    #[doc = "Continuous run (aka Free Run). Count continuously."]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut W {
        self.variant(TMRA0FN_A::CONTINUOUS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 6)) | (((value as u32) & 0x07) << 6);
        self.w
    }
}
#[doc = "Counter/Timer A0 Clock Select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TMRA0CLK_A {
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
    #[doc = "16: Clock source is buck converter stream A."]
    BUCKA = 16,
}
impl From<TMRA0CLK_A> for u8 {
    #[inline(always)]
    fn from(variant: TMRA0CLK_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TMRA0CLK`"]
pub type TMRA0CLK_R = crate::R<u8, TMRA0CLK_A>;
impl TMRA0CLK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TMRA0CLK_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TMRA0CLK_A::TMRPIN),
            1 => Val(TMRA0CLK_A::HFRC),
            2 => Val(TMRA0CLK_A::HFRC_DIV8),
            3 => Val(TMRA0CLK_A::HFRC_DIV128),
            4 => Val(TMRA0CLK_A::HFRC_DIV512),
            5 => Val(TMRA0CLK_A::HFRC_DIV2K),
            6 => Val(TMRA0CLK_A::XT),
            7 => Val(TMRA0CLK_A::XT_DIV2),
            8 => Val(TMRA0CLK_A::XT_DIV16),
            9 => Val(TMRA0CLK_A::XT_DIV256),
            10 => Val(TMRA0CLK_A::LFRC_DIV2),
            11 => Val(TMRA0CLK_A::LFRC_DIV32),
            12 => Val(TMRA0CLK_A::LFRC_DIV1K),
            13 => Val(TMRA0CLK_A::LFRC),
            14 => Val(TMRA0CLK_A::RTC_100HZ),
            15 => Val(TMRA0CLK_A::HCLK),
            16 => Val(TMRA0CLK_A::BUCKA),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TMRPIN`"]
    #[inline(always)]
    pub fn is_tmrpin(&self) -> bool {
        *self == TMRA0CLK_A::TMRPIN
    }
    #[doc = "Checks if the value of the field is `HFRC`"]
    #[inline(always)]
    pub fn is_hfrc(&self) -> bool {
        *self == TMRA0CLK_A::HFRC
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV8`"]
    #[inline(always)]
    pub fn is_hfrc_div8(&self) -> bool {
        *self == TMRA0CLK_A::HFRC_DIV8
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV128`"]
    #[inline(always)]
    pub fn is_hfrc_div128(&self) -> bool {
        *self == TMRA0CLK_A::HFRC_DIV128
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV512`"]
    #[inline(always)]
    pub fn is_hfrc_div512(&self) -> bool {
        *self == TMRA0CLK_A::HFRC_DIV512
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV2K`"]
    #[inline(always)]
    pub fn is_hfrc_div2k(&self) -> bool {
        *self == TMRA0CLK_A::HFRC_DIV2K
    }
    #[doc = "Checks if the value of the field is `XT`"]
    #[inline(always)]
    pub fn is_xt(&self) -> bool {
        *self == TMRA0CLK_A::XT
    }
    #[doc = "Checks if the value of the field is `XT_DIV2`"]
    #[inline(always)]
    pub fn is_xt_div2(&self) -> bool {
        *self == TMRA0CLK_A::XT_DIV2
    }
    #[doc = "Checks if the value of the field is `XT_DIV16`"]
    #[inline(always)]
    pub fn is_xt_div16(&self) -> bool {
        *self == TMRA0CLK_A::XT_DIV16
    }
    #[doc = "Checks if the value of the field is `XT_DIV256`"]
    #[inline(always)]
    pub fn is_xt_div256(&self) -> bool {
        *self == TMRA0CLK_A::XT_DIV256
    }
    #[doc = "Checks if the value of the field is `LFRC_DIV2`"]
    #[inline(always)]
    pub fn is_lfrc_div2(&self) -> bool {
        *self == TMRA0CLK_A::LFRC_DIV2
    }
    #[doc = "Checks if the value of the field is `LFRC_DIV32`"]
    #[inline(always)]
    pub fn is_lfrc_div32(&self) -> bool {
        *self == TMRA0CLK_A::LFRC_DIV32
    }
    #[doc = "Checks if the value of the field is `LFRC_DIV1K`"]
    #[inline(always)]
    pub fn is_lfrc_div1k(&self) -> bool {
        *self == TMRA0CLK_A::LFRC_DIV1K
    }
    #[doc = "Checks if the value of the field is `LFRC`"]
    #[inline(always)]
    pub fn is_lfrc(&self) -> bool {
        *self == TMRA0CLK_A::LFRC
    }
    #[doc = "Checks if the value of the field is `RTC_100HZ`"]
    #[inline(always)]
    pub fn is_rtc_100hz(&self) -> bool {
        *self == TMRA0CLK_A::RTC_100HZ
    }
    #[doc = "Checks if the value of the field is `HCLK`"]
    #[inline(always)]
    pub fn is_hclk(&self) -> bool {
        *self == TMRA0CLK_A::HCLK
    }
    #[doc = "Checks if the value of the field is `BUCKA`"]
    #[inline(always)]
    pub fn is_bucka(&self) -> bool {
        *self == TMRA0CLK_A::BUCKA
    }
}
#[doc = "Write proxy for field `TMRA0CLK`"]
pub struct TMRA0CLK_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRA0CLK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRA0CLK_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Clock source is TMRPINA."]
    #[inline(always)]
    pub fn tmrpin(self) -> &'a mut W {
        self.variant(TMRA0CLK_A::TMRPIN)
    }
    #[doc = "Clock source is the HFRC"]
    #[inline(always)]
    pub fn hfrc(self) -> &'a mut W {
        self.variant(TMRA0CLK_A::HFRC)
    }
    #[doc = "Clock source is HFRC / 8"]
    #[inline(always)]
    pub fn hfrc_div8(self) -> &'a mut W {
        self.variant(TMRA0CLK_A::HFRC_DIV8)
    }
    #[doc = "Clock source is HFRC / 128"]
    #[inline(always)]
    pub fn hfrc_div128(self) -> &'a mut W {
        self.variant(TMRA0CLK_A::HFRC_DIV128)
    }
    #[doc = "Clock source is HFRC / 512"]
    #[inline(always)]
    pub fn hfrc_div512(self) -> &'a mut W {
        self.variant(TMRA0CLK_A::HFRC_DIV512)
    }
    #[doc = "Clock source is HFRC / 2048"]
    #[inline(always)]
    pub fn hfrc_div2k(self) -> &'a mut W {
        self.variant(TMRA0CLK_A::HFRC_DIV2K)
    }
    #[doc = "Clock source is the XT (uncalibrated)."]
    #[inline(always)]
    pub fn xt(self) -> &'a mut W {
        self.variant(TMRA0CLK_A::XT)
    }
    #[doc = "Clock source is XT / 2"]
    #[inline(always)]
    pub fn xt_div2(self) -> &'a mut W {
        self.variant(TMRA0CLK_A::XT_DIV2)
    }
    #[doc = "Clock source is XT / 16"]
    #[inline(always)]
    pub fn xt_div16(self) -> &'a mut W {
        self.variant(TMRA0CLK_A::XT_DIV16)
    }
    #[doc = "Clock source is XT / 256"]
    #[inline(always)]
    pub fn xt_div256(self) -> &'a mut W {
        self.variant(TMRA0CLK_A::XT_DIV256)
    }
    #[doc = "Clock source is LFRC / 2"]
    #[inline(always)]
    pub fn lfrc_div2(self) -> &'a mut W {
        self.variant(TMRA0CLK_A::LFRC_DIV2)
    }
    #[doc = "Clock source is LFRC / 32"]
    #[inline(always)]
    pub fn lfrc_div32(self) -> &'a mut W {
        self.variant(TMRA0CLK_A::LFRC_DIV32)
    }
    #[doc = "Clock source is LFRC / 1024"]
    #[inline(always)]
    pub fn lfrc_div1k(self) -> &'a mut W {
        self.variant(TMRA0CLK_A::LFRC_DIV1K)
    }
    #[doc = "Clock source is LFRC / 16K"]
    #[inline(always)]
    pub fn lfrc(self) -> &'a mut W {
        self.variant(TMRA0CLK_A::LFRC)
    }
    #[doc = "Clock source is 100 Hz from the current RTC oscillator."]
    #[inline(always)]
    pub fn rtc_100hz(self) -> &'a mut W {
        self.variant(TMRA0CLK_A::RTC_100HZ)
    }
    #[doc = "Clock source is HCLK."]
    #[inline(always)]
    pub fn hclk(self) -> &'a mut W {
        self.variant(TMRA0CLK_A::HCLK)
    }
    #[doc = "Clock source is buck converter stream A."]
    #[inline(always)]
    pub fn bucka(self) -> &'a mut W {
        self.variant(TMRA0CLK_A::BUCKA)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 1)) | (((value as u32) & 0x1f) << 1);
        self.w
    }
}
#[doc = "Counter/Timer A0 Enable bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA0EN_A {
    #[doc = "0: Counter/Timer A0 Disable."]
    DIS = 0,
    #[doc = "1: Counter/Timer A0 Enable."]
    EN = 1,
}
impl From<TMRA0EN_A> for bool {
    #[inline(always)]
    fn from(variant: TMRA0EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TMRA0EN`"]
pub type TMRA0EN_R = crate::R<bool, TMRA0EN_A>;
impl TMRA0EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRA0EN_A {
        match self.bits {
            false => TMRA0EN_A::DIS,
            true => TMRA0EN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TMRA0EN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == TMRA0EN_A::EN
    }
}
#[doc = "Write proxy for field `TMRA0EN`"]
pub struct TMRA0EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRA0EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRA0EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Counter/Timer A0 Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRA0EN_A::DIS)
    }
    #[doc = "Counter/Timer A0 Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRA0EN_A::EN)
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
    #[doc = "Bit 31 - Counter/Timer A0/B0 Link bit."]
    #[inline(always)]
    pub fn ctlink0(&self) -> CTLINK0_R {
        CTLINK0_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Counter/Timer B0 output polarity."]
    #[inline(always)]
    pub fn tmrb0pol(&self) -> TMRB0POL_R {
        TMRB0POL_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Counter/Timer B0 Clear bit."]
    #[inline(always)]
    pub fn tmrb0clr(&self) -> TMRB0CLR_R {
        TMRB0CLR_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Counter/Timer B0 Output Enable bit."]
    #[inline(always)]
    pub fn tmrb0pe(&self) -> TMRB0PE_R {
        TMRB0PE_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Counter/Timer B0 Interrupt Enable bit."]
    #[inline(always)]
    pub fn tmrb0ie(&self) -> TMRB0IE_R {
        TMRB0IE_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bits 22:24 - Counter/Timer B0 Function Select."]
    #[inline(always)]
    pub fn tmrb0fn(&self) -> TMRB0FN_R {
        TMRB0FN_R::new(((self.bits >> 22) & 0x07) as u8)
    }
    #[doc = "Bits 17:21 - Counter/Timer B0 Clock Select."]
    #[inline(always)]
    pub fn tmrb0clk(&self) -> TMRB0CLK_R {
        TMRB0CLK_R::new(((self.bits >> 17) & 0x1f) as u8)
    }
    #[doc = "Bit 16 - Counter/Timer B0 Enable bit."]
    #[inline(always)]
    pub fn tmrb0en(&self) -> TMRB0EN_R {
        TMRB0EN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Counter/Timer A0 output polarity."]
    #[inline(always)]
    pub fn tmra0pol(&self) -> TMRA0POL_R {
        TMRA0POL_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Counter/Timer A0 Clear bit."]
    #[inline(always)]
    pub fn tmra0clr(&self) -> TMRA0CLR_R {
        TMRA0CLR_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Counter/Timer A0 Output Enable bit."]
    #[inline(always)]
    pub fn tmra0pe(&self) -> TMRA0PE_R {
        TMRA0PE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Counter/Timer A0 Interrupt Enable bit."]
    #[inline(always)]
    pub fn tmra0ie(&self) -> TMRA0IE_R {
        TMRA0IE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bits 6:8 - Counter/Timer A0 Function Select."]
    #[inline(always)]
    pub fn tmra0fn(&self) -> TMRA0FN_R {
        TMRA0FN_R::new(((self.bits >> 6) & 0x07) as u8)
    }
    #[doc = "Bits 1:5 - Counter/Timer A0 Clock Select."]
    #[inline(always)]
    pub fn tmra0clk(&self) -> TMRA0CLK_R {
        TMRA0CLK_R::new(((self.bits >> 1) & 0x1f) as u8)
    }
    #[doc = "Bit 0 - Counter/Timer A0 Enable bit."]
    #[inline(always)]
    pub fn tmra0en(&self) -> TMRA0EN_R {
        TMRA0EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - Counter/Timer A0/B0 Link bit."]
    #[inline(always)]
    pub fn ctlink0(&mut self) -> CTLINK0_W {
        CTLINK0_W { w: self }
    }
    #[doc = "Bit 28 - Counter/Timer B0 output polarity."]
    #[inline(always)]
    pub fn tmrb0pol(&mut self) -> TMRB0POL_W {
        TMRB0POL_W { w: self }
    }
    #[doc = "Bit 27 - Counter/Timer B0 Clear bit."]
    #[inline(always)]
    pub fn tmrb0clr(&mut self) -> TMRB0CLR_W {
        TMRB0CLR_W { w: self }
    }
    #[doc = "Bit 26 - Counter/Timer B0 Output Enable bit."]
    #[inline(always)]
    pub fn tmrb0pe(&mut self) -> TMRB0PE_W {
        TMRB0PE_W { w: self }
    }
    #[doc = "Bit 25 - Counter/Timer B0 Interrupt Enable bit."]
    #[inline(always)]
    pub fn tmrb0ie(&mut self) -> TMRB0IE_W {
        TMRB0IE_W { w: self }
    }
    #[doc = "Bits 22:24 - Counter/Timer B0 Function Select."]
    #[inline(always)]
    pub fn tmrb0fn(&mut self) -> TMRB0FN_W {
        TMRB0FN_W { w: self }
    }
    #[doc = "Bits 17:21 - Counter/Timer B0 Clock Select."]
    #[inline(always)]
    pub fn tmrb0clk(&mut self) -> TMRB0CLK_W {
        TMRB0CLK_W { w: self }
    }
    #[doc = "Bit 16 - Counter/Timer B0 Enable bit."]
    #[inline(always)]
    pub fn tmrb0en(&mut self) -> TMRB0EN_W {
        TMRB0EN_W { w: self }
    }
    #[doc = "Bit 12 - Counter/Timer A0 output polarity."]
    #[inline(always)]
    pub fn tmra0pol(&mut self) -> TMRA0POL_W {
        TMRA0POL_W { w: self }
    }
    #[doc = "Bit 11 - Counter/Timer A0 Clear bit."]
    #[inline(always)]
    pub fn tmra0clr(&mut self) -> TMRA0CLR_W {
        TMRA0CLR_W { w: self }
    }
    #[doc = "Bit 10 - Counter/Timer A0 Output Enable bit."]
    #[inline(always)]
    pub fn tmra0pe(&mut self) -> TMRA0PE_W {
        TMRA0PE_W { w: self }
    }
    #[doc = "Bit 9 - Counter/Timer A0 Interrupt Enable bit."]
    #[inline(always)]
    pub fn tmra0ie(&mut self) -> TMRA0IE_W {
        TMRA0IE_W { w: self }
    }
    #[doc = "Bits 6:8 - Counter/Timer A0 Function Select."]
    #[inline(always)]
    pub fn tmra0fn(&mut self) -> TMRA0FN_W {
        TMRA0FN_W { w: self }
    }
    #[doc = "Bits 1:5 - Counter/Timer A0 Clock Select."]
    #[inline(always)]
    pub fn tmra0clk(&mut self) -> TMRA0CLK_W {
        TMRA0CLK_W { w: self }
    }
    #[doc = "Bit 0 - Counter/Timer A0 Enable bit."]
    #[inline(always)]
    pub fn tmra0en(&mut self) -> TMRA0EN_W {
        TMRA0EN_W { w: self }
    }
}
