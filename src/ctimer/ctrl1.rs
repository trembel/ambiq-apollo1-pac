#[doc = "Reader of register CTRL1"]
pub type R = crate::R<u32, super::CTRL1>;
#[doc = "Writer for register CTRL1"]
pub type W = crate::W<u32, super::CTRL1>;
#[doc = "Register CTRL1 `reset()`'s with value 0"]
impl crate::ResetValue for super::CTRL1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Counter/Timer A1/B1 Link bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTLINK1_A {
    #[doc = "0: Use A0/B0 timers as two independent 16-bit timers (default)."]
    TWO_16BIT_TIMERS = 0,
    #[doc = "1: Link A1/B1 timers into a single 32-bit timer."]
    _32BIT_TIMER = 1,
}
impl From<CTLINK1_A> for bool {
    #[inline(always)]
    fn from(variant: CTLINK1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CTLINK1`"]
pub type CTLINK1_R = crate::R<bool, CTLINK1_A>;
impl CTLINK1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTLINK1_A {
        match self.bits {
            false => CTLINK1_A::TWO_16BIT_TIMERS,
            true => CTLINK1_A::_32BIT_TIMER,
        }
    }
    #[doc = "Checks if the value of the field is `TWO_16BIT_TIMERS`"]
    #[inline(always)]
    pub fn is_two_16bit_timers(&self) -> bool {
        *self == CTLINK1_A::TWO_16BIT_TIMERS
    }
    #[doc = "Checks if the value of the field is `_32BIT_TIMER`"]
    #[inline(always)]
    pub fn is_32bit_timer(&self) -> bool {
        *self == CTLINK1_A::_32BIT_TIMER
    }
}
#[doc = "Write proxy for field `CTLINK1`"]
pub struct CTLINK1_W<'a> {
    w: &'a mut W,
}
impl<'a> CTLINK1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTLINK1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Use A0/B0 timers as two independent 16-bit timers (default)."]
    #[inline(always)]
    pub fn two_16bit_timers(self) -> &'a mut W {
        self.variant(CTLINK1_A::TWO_16BIT_TIMERS)
    }
    #[doc = "Link A1/B1 timers into a single 32-bit timer."]
    #[inline(always)]
    pub fn _32bit_timer(self) -> &'a mut W {
        self.variant(CTLINK1_A::_32BIT_TIMER)
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
#[doc = "Counter/Timer B1 output polarity.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB1POL_A {
    #[doc = "0: The polarity of the TMRPINB1 pin is the same as the timer output."]
    NORMAL = 0,
    #[doc = "1: The polarity of the TMRPINB1 pin is the inverse of the timer output."]
    INVERTED = 1,
}
impl From<TMRB1POL_A> for bool {
    #[inline(always)]
    fn from(variant: TMRB1POL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TMRB1POL`"]
pub type TMRB1POL_R = crate::R<bool, TMRB1POL_A>;
impl TMRB1POL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRB1POL_A {
        match self.bits {
            false => TMRB1POL_A::NORMAL,
            true => TMRB1POL_A::INVERTED,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == TMRB1POL_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `INVERTED`"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == TMRB1POL_A::INVERTED
    }
}
#[doc = "Write proxy for field `TMRB1POL`"]
pub struct TMRB1POL_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRB1POL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRB1POL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The polarity of the TMRPINB1 pin is the same as the timer output."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(TMRB1POL_A::NORMAL)
    }
    #[doc = "The polarity of the TMRPINB1 pin is the inverse of the timer output."]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut W {
        self.variant(TMRB1POL_A::INVERTED)
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
#[doc = "Counter/Timer B1 Clear bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB1CLR_A {
    #[doc = "0: Allow counter/timer B1 to run"]
    RUN = 0,
    #[doc = "1: Holds counter/timer B1 at 0x0000."]
    CLEAR = 1,
}
impl From<TMRB1CLR_A> for bool {
    #[inline(always)]
    fn from(variant: TMRB1CLR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TMRB1CLR`"]
pub type TMRB1CLR_R = crate::R<bool, TMRB1CLR_A>;
impl TMRB1CLR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRB1CLR_A {
        match self.bits {
            false => TMRB1CLR_A::RUN,
            true => TMRB1CLR_A::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `RUN`"]
    #[inline(always)]
    pub fn is_run(&self) -> bool {
        *self == TMRB1CLR_A::RUN
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == TMRB1CLR_A::CLEAR
    }
}
#[doc = "Write proxy for field `TMRB1CLR`"]
pub struct TMRB1CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRB1CLR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRB1CLR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Allow counter/timer B1 to run"]
    #[inline(always)]
    pub fn run(self) -> &'a mut W {
        self.variant(TMRB1CLR_A::RUN)
    }
    #[doc = "Holds counter/timer B1 at 0x0000."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(TMRB1CLR_A::CLEAR)
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
#[doc = "Counter/Timer B1 Output Enable bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB1PE_A {
    #[doc = "0: Counter/Timer B holds the TMRPINB signal at the value TMRB1POL."]
    DIS = 0,
    #[doc = "1: Enable counter/timer B1 to generate a signal on TMRPINB."]
    EN = 1,
}
impl From<TMRB1PE_A> for bool {
    #[inline(always)]
    fn from(variant: TMRB1PE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TMRB1PE`"]
pub type TMRB1PE_R = crate::R<bool, TMRB1PE_A>;
impl TMRB1PE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRB1PE_A {
        match self.bits {
            false => TMRB1PE_A::DIS,
            true => TMRB1PE_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TMRB1PE_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == TMRB1PE_A::EN
    }
}
#[doc = "Write proxy for field `TMRB1PE`"]
pub struct TMRB1PE_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRB1PE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRB1PE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Counter/Timer B holds the TMRPINB signal at the value TMRB1POL."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRB1PE_A::DIS)
    }
    #[doc = "Enable counter/timer B1 to generate a signal on TMRPINB."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRB1PE_A::EN)
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
#[doc = "Counter/Timer B1 Interrupt Enable bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB1IE_A {
    #[doc = "0: Disable counter/timer B1 from generating an interrupt."]
    DIS = 0,
    #[doc = "1: Enable counter/timer B1 to generate an interrupt."]
    EN = 1,
}
impl From<TMRB1IE_A> for bool {
    #[inline(always)]
    fn from(variant: TMRB1IE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TMRB1IE`"]
pub type TMRB1IE_R = crate::R<bool, TMRB1IE_A>;
impl TMRB1IE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRB1IE_A {
        match self.bits {
            false => TMRB1IE_A::DIS,
            true => TMRB1IE_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TMRB1IE_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == TMRB1IE_A::EN
    }
}
#[doc = "Write proxy for field `TMRB1IE`"]
pub struct TMRB1IE_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRB1IE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRB1IE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable counter/timer B1 from generating an interrupt."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRB1IE_A::DIS)
    }
    #[doc = "Enable counter/timer B1 to generate an interrupt."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRB1IE_A::EN)
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
#[doc = "Counter/Timer B1 Function Select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TMRB1FN_A {
    #[doc = "0: Single count (output toggles and sticks).  Count to CMPR0B1, stop."]
    SINGLECOUNT = 0,
    #[doc = "1: Repeated count (periodic 1-clock-cycle-wide pulses).  Count to CMPR0B1, restart."]
    REPEATEDCOUNT = 1,
    #[doc = "2: Pulse once (aka one-shot).  Count to CMPR0B1, assert, count to CMPR1B, deassert, stop."]
    PULSE_ONCE = 2,
    #[doc = "3: Pulse continously.  Count to CMPR0B1, assert, count to CMPR1B1, deassert, restart."]
    PULSE_CONT = 3,
    #[doc = "4: Continuous run (aka Free Run).  Count continuously."]
    CONTINUOUS = 4,
}
impl From<TMRB1FN_A> for u8 {
    #[inline(always)]
    fn from(variant: TMRB1FN_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TMRB1FN`"]
pub type TMRB1FN_R = crate::R<u8, TMRB1FN_A>;
impl TMRB1FN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TMRB1FN_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TMRB1FN_A::SINGLECOUNT),
            1 => Val(TMRB1FN_A::REPEATEDCOUNT),
            2 => Val(TMRB1FN_A::PULSE_ONCE),
            3 => Val(TMRB1FN_A::PULSE_CONT),
            4 => Val(TMRB1FN_A::CONTINUOUS),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SINGLECOUNT`"]
    #[inline(always)]
    pub fn is_singlecount(&self) -> bool {
        *self == TMRB1FN_A::SINGLECOUNT
    }
    #[doc = "Checks if the value of the field is `REPEATEDCOUNT`"]
    #[inline(always)]
    pub fn is_repeatedcount(&self) -> bool {
        *self == TMRB1FN_A::REPEATEDCOUNT
    }
    #[doc = "Checks if the value of the field is `PULSE_ONCE`"]
    #[inline(always)]
    pub fn is_pulse_once(&self) -> bool {
        *self == TMRB1FN_A::PULSE_ONCE
    }
    #[doc = "Checks if the value of the field is `PULSE_CONT`"]
    #[inline(always)]
    pub fn is_pulse_cont(&self) -> bool {
        *self == TMRB1FN_A::PULSE_CONT
    }
    #[doc = "Checks if the value of the field is `CONTINUOUS`"]
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        *self == TMRB1FN_A::CONTINUOUS
    }
}
#[doc = "Write proxy for field `TMRB1FN`"]
pub struct TMRB1FN_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRB1FN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRB1FN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Single count (output toggles and sticks). Count to CMPR0B1, stop."]
    #[inline(always)]
    pub fn singlecount(self) -> &'a mut W {
        self.variant(TMRB1FN_A::SINGLECOUNT)
    }
    #[doc = "Repeated count (periodic 1-clock-cycle-wide pulses). Count to CMPR0B1, restart."]
    #[inline(always)]
    pub fn repeatedcount(self) -> &'a mut W {
        self.variant(TMRB1FN_A::REPEATEDCOUNT)
    }
    #[doc = "Pulse once (aka one-shot). Count to CMPR0B1, assert, count to CMPR1B, deassert, stop."]
    #[inline(always)]
    pub fn pulse_once(self) -> &'a mut W {
        self.variant(TMRB1FN_A::PULSE_ONCE)
    }
    #[doc = "Pulse continously. Count to CMPR0B1, assert, count to CMPR1B1, deassert, restart."]
    #[inline(always)]
    pub fn pulse_cont(self) -> &'a mut W {
        self.variant(TMRB1FN_A::PULSE_CONT)
    }
    #[doc = "Continuous run (aka Free Run). Count continuously."]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut W {
        self.variant(TMRB1FN_A::CONTINUOUS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 22)) | (((value as u32) & 0x07) << 22);
        self.w
    }
}
#[doc = "Counter/Timer B1 Clock Select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TMRB1CLK_A {
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
impl From<TMRB1CLK_A> for u8 {
    #[inline(always)]
    fn from(variant: TMRB1CLK_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TMRB1CLK`"]
pub type TMRB1CLK_R = crate::R<u8, TMRB1CLK_A>;
impl TMRB1CLK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TMRB1CLK_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TMRB1CLK_A::TMRPIN),
            1 => Val(TMRB1CLK_A::HFRC),
            2 => Val(TMRB1CLK_A::HFRC_DIV8),
            3 => Val(TMRB1CLK_A::HFRC_DIV128),
            4 => Val(TMRB1CLK_A::HFRC_DIV512),
            5 => Val(TMRB1CLK_A::HFRC_DIV2K),
            6 => Val(TMRB1CLK_A::XT),
            7 => Val(TMRB1CLK_A::XT_DIV2),
            8 => Val(TMRB1CLK_A::XT_DIV16),
            9 => Val(TMRB1CLK_A::XT_DIV256),
            10 => Val(TMRB1CLK_A::LFRC_DIV2),
            11 => Val(TMRB1CLK_A::LFRC_DIV32),
            12 => Val(TMRB1CLK_A::LFRC_DIV1K),
            13 => Val(TMRB1CLK_A::LFRC),
            14 => Val(TMRB1CLK_A::RTC_100HZ),
            15 => Val(TMRB1CLK_A::HCLK),
            16 => Val(TMRB1CLK_A::BUCKB),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TMRPIN`"]
    #[inline(always)]
    pub fn is_tmrpin(&self) -> bool {
        *self == TMRB1CLK_A::TMRPIN
    }
    #[doc = "Checks if the value of the field is `HFRC`"]
    #[inline(always)]
    pub fn is_hfrc(&self) -> bool {
        *self == TMRB1CLK_A::HFRC
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV8`"]
    #[inline(always)]
    pub fn is_hfrc_div8(&self) -> bool {
        *self == TMRB1CLK_A::HFRC_DIV8
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV128`"]
    #[inline(always)]
    pub fn is_hfrc_div128(&self) -> bool {
        *self == TMRB1CLK_A::HFRC_DIV128
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV512`"]
    #[inline(always)]
    pub fn is_hfrc_div512(&self) -> bool {
        *self == TMRB1CLK_A::HFRC_DIV512
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV2K`"]
    #[inline(always)]
    pub fn is_hfrc_div2k(&self) -> bool {
        *self == TMRB1CLK_A::HFRC_DIV2K
    }
    #[doc = "Checks if the value of the field is `XT`"]
    #[inline(always)]
    pub fn is_xt(&self) -> bool {
        *self == TMRB1CLK_A::XT
    }
    #[doc = "Checks if the value of the field is `XT_DIV2`"]
    #[inline(always)]
    pub fn is_xt_div2(&self) -> bool {
        *self == TMRB1CLK_A::XT_DIV2
    }
    #[doc = "Checks if the value of the field is `XT_DIV16`"]
    #[inline(always)]
    pub fn is_xt_div16(&self) -> bool {
        *self == TMRB1CLK_A::XT_DIV16
    }
    #[doc = "Checks if the value of the field is `XT_DIV256`"]
    #[inline(always)]
    pub fn is_xt_div256(&self) -> bool {
        *self == TMRB1CLK_A::XT_DIV256
    }
    #[doc = "Checks if the value of the field is `LFRC_DIV2`"]
    #[inline(always)]
    pub fn is_lfrc_div2(&self) -> bool {
        *self == TMRB1CLK_A::LFRC_DIV2
    }
    #[doc = "Checks if the value of the field is `LFRC_DIV32`"]
    #[inline(always)]
    pub fn is_lfrc_div32(&self) -> bool {
        *self == TMRB1CLK_A::LFRC_DIV32
    }
    #[doc = "Checks if the value of the field is `LFRC_DIV1K`"]
    #[inline(always)]
    pub fn is_lfrc_div1k(&self) -> bool {
        *self == TMRB1CLK_A::LFRC_DIV1K
    }
    #[doc = "Checks if the value of the field is `LFRC`"]
    #[inline(always)]
    pub fn is_lfrc(&self) -> bool {
        *self == TMRB1CLK_A::LFRC
    }
    #[doc = "Checks if the value of the field is `RTC_100HZ`"]
    #[inline(always)]
    pub fn is_rtc_100hz(&self) -> bool {
        *self == TMRB1CLK_A::RTC_100HZ
    }
    #[doc = "Checks if the value of the field is `HCLK`"]
    #[inline(always)]
    pub fn is_hclk(&self) -> bool {
        *self == TMRB1CLK_A::HCLK
    }
    #[doc = "Checks if the value of the field is `BUCKB`"]
    #[inline(always)]
    pub fn is_buckb(&self) -> bool {
        *self == TMRB1CLK_A::BUCKB
    }
}
#[doc = "Write proxy for field `TMRB1CLK`"]
pub struct TMRB1CLK_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRB1CLK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRB1CLK_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Clock source is TMRPINB."]
    #[inline(always)]
    pub fn tmrpin(self) -> &'a mut W {
        self.variant(TMRB1CLK_A::TMRPIN)
    }
    #[doc = "Clock source is the HFRC"]
    #[inline(always)]
    pub fn hfrc(self) -> &'a mut W {
        self.variant(TMRB1CLK_A::HFRC)
    }
    #[doc = "Clock source is HFRC / 8"]
    #[inline(always)]
    pub fn hfrc_div8(self) -> &'a mut W {
        self.variant(TMRB1CLK_A::HFRC_DIV8)
    }
    #[doc = "Clock source is HFRC / 128"]
    #[inline(always)]
    pub fn hfrc_div128(self) -> &'a mut W {
        self.variant(TMRB1CLK_A::HFRC_DIV128)
    }
    #[doc = "Clock source is HFRC / 512"]
    #[inline(always)]
    pub fn hfrc_div512(self) -> &'a mut W {
        self.variant(TMRB1CLK_A::HFRC_DIV512)
    }
    #[doc = "Clock source is HFRC / 2048"]
    #[inline(always)]
    pub fn hfrc_div2k(self) -> &'a mut W {
        self.variant(TMRB1CLK_A::HFRC_DIV2K)
    }
    #[doc = "Clock source is the XT (uncalibrated)."]
    #[inline(always)]
    pub fn xt(self) -> &'a mut W {
        self.variant(TMRB1CLK_A::XT)
    }
    #[doc = "Clock source is XT / 2"]
    #[inline(always)]
    pub fn xt_div2(self) -> &'a mut W {
        self.variant(TMRB1CLK_A::XT_DIV2)
    }
    #[doc = "Clock source is XT / 16"]
    #[inline(always)]
    pub fn xt_div16(self) -> &'a mut W {
        self.variant(TMRB1CLK_A::XT_DIV16)
    }
    #[doc = "Clock source is XT / 256"]
    #[inline(always)]
    pub fn xt_div256(self) -> &'a mut W {
        self.variant(TMRB1CLK_A::XT_DIV256)
    }
    #[doc = "Clock source is LFRC / 2"]
    #[inline(always)]
    pub fn lfrc_div2(self) -> &'a mut W {
        self.variant(TMRB1CLK_A::LFRC_DIV2)
    }
    #[doc = "Clock source is LFRC / 32"]
    #[inline(always)]
    pub fn lfrc_div32(self) -> &'a mut W {
        self.variant(TMRB1CLK_A::LFRC_DIV32)
    }
    #[doc = "Clock source is LFRC / 1024"]
    #[inline(always)]
    pub fn lfrc_div1k(self) -> &'a mut W {
        self.variant(TMRB1CLK_A::LFRC_DIV1K)
    }
    #[doc = "Clock source is LFRC / 16K"]
    #[inline(always)]
    pub fn lfrc(self) -> &'a mut W {
        self.variant(TMRB1CLK_A::LFRC)
    }
    #[doc = "Clock source is 100 Hz from the current RTC oscillator."]
    #[inline(always)]
    pub fn rtc_100hz(self) -> &'a mut W {
        self.variant(TMRB1CLK_A::RTC_100HZ)
    }
    #[doc = "Clock source is HCLK."]
    #[inline(always)]
    pub fn hclk(self) -> &'a mut W {
        self.variant(TMRB1CLK_A::HCLK)
    }
    #[doc = "Clock source is buck converter stream B."]
    #[inline(always)]
    pub fn buckb(self) -> &'a mut W {
        self.variant(TMRB1CLK_A::BUCKB)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 17)) | (((value as u32) & 0x1f) << 17);
        self.w
    }
}
#[doc = "Counter/Timer B1 Enable bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB1EN_A {
    #[doc = "0: Counter/Timer B1 Disable."]
    DIS = 0,
    #[doc = "1: Counter/Timer B1 Enable."]
    EN = 1,
}
impl From<TMRB1EN_A> for bool {
    #[inline(always)]
    fn from(variant: TMRB1EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TMRB1EN`"]
pub type TMRB1EN_R = crate::R<bool, TMRB1EN_A>;
impl TMRB1EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRB1EN_A {
        match self.bits {
            false => TMRB1EN_A::DIS,
            true => TMRB1EN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TMRB1EN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == TMRB1EN_A::EN
    }
}
#[doc = "Write proxy for field `TMRB1EN`"]
pub struct TMRB1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRB1EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRB1EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Counter/Timer B1 Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRB1EN_A::DIS)
    }
    #[doc = "Counter/Timer B1 Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRB1EN_A::EN)
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
#[doc = "Counter/Timer A1 output polarity.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA1POL_A {
    #[doc = "0: The polarity of the TMRPINA1 pin is the same as the timer output."]
    NORMAL = 0,
    #[doc = "1: The polarity of the TMRPINA1 pin is the inverse of the timer output."]
    INVERTED = 1,
}
impl From<TMRA1POL_A> for bool {
    #[inline(always)]
    fn from(variant: TMRA1POL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TMRA1POL`"]
pub type TMRA1POL_R = crate::R<bool, TMRA1POL_A>;
impl TMRA1POL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRA1POL_A {
        match self.bits {
            false => TMRA1POL_A::NORMAL,
            true => TMRA1POL_A::INVERTED,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == TMRA1POL_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `INVERTED`"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == TMRA1POL_A::INVERTED
    }
}
#[doc = "Write proxy for field `TMRA1POL`"]
pub struct TMRA1POL_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRA1POL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRA1POL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The polarity of the TMRPINA1 pin is the same as the timer output."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(TMRA1POL_A::NORMAL)
    }
    #[doc = "The polarity of the TMRPINA1 pin is the inverse of the timer output."]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut W {
        self.variant(TMRA1POL_A::INVERTED)
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
#[doc = "Counter/Timer A1 Clear bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA1CLR_A {
    #[doc = "0: Allow counter/timer A1 to run"]
    RUN = 0,
    #[doc = "1: Holds counter/timer A1 at 0x0000."]
    CLEAR = 1,
}
impl From<TMRA1CLR_A> for bool {
    #[inline(always)]
    fn from(variant: TMRA1CLR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TMRA1CLR`"]
pub type TMRA1CLR_R = crate::R<bool, TMRA1CLR_A>;
impl TMRA1CLR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRA1CLR_A {
        match self.bits {
            false => TMRA1CLR_A::RUN,
            true => TMRA1CLR_A::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `RUN`"]
    #[inline(always)]
    pub fn is_run(&self) -> bool {
        *self == TMRA1CLR_A::RUN
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == TMRA1CLR_A::CLEAR
    }
}
#[doc = "Write proxy for field `TMRA1CLR`"]
pub struct TMRA1CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRA1CLR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRA1CLR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Allow counter/timer A1 to run"]
    #[inline(always)]
    pub fn run(self) -> &'a mut W {
        self.variant(TMRA1CLR_A::RUN)
    }
    #[doc = "Holds counter/timer A1 at 0x0000."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(TMRA1CLR_A::CLEAR)
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
#[doc = "Counter/Timer A1 Output Enable bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA1PE_A {
    #[doc = "0: Counter/Timer A holds the TMRPINA signal at the value TMRA1POL."]
    DIS = 0,
    #[doc = "1: Enable counter/timer A1 to generate a signal on TMRPINA."]
    EN = 1,
}
impl From<TMRA1PE_A> for bool {
    #[inline(always)]
    fn from(variant: TMRA1PE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TMRA1PE`"]
pub type TMRA1PE_R = crate::R<bool, TMRA1PE_A>;
impl TMRA1PE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRA1PE_A {
        match self.bits {
            false => TMRA1PE_A::DIS,
            true => TMRA1PE_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TMRA1PE_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == TMRA1PE_A::EN
    }
}
#[doc = "Write proxy for field `TMRA1PE`"]
pub struct TMRA1PE_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRA1PE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRA1PE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Counter/Timer A holds the TMRPINA signal at the value TMRA1POL."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRA1PE_A::DIS)
    }
    #[doc = "Enable counter/timer A1 to generate a signal on TMRPINA."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRA1PE_A::EN)
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
#[doc = "Counter/Timer A1 Interrupt Enable bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA1IE_A {
    #[doc = "0: Disable counter/timer A1 from generating an interrupt."]
    DIS = 0,
    #[doc = "1: Enable counter/timer A1 to generate an interrupt."]
    EN = 1,
}
impl From<TMRA1IE_A> for bool {
    #[inline(always)]
    fn from(variant: TMRA1IE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TMRA1IE`"]
pub type TMRA1IE_R = crate::R<bool, TMRA1IE_A>;
impl TMRA1IE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRA1IE_A {
        match self.bits {
            false => TMRA1IE_A::DIS,
            true => TMRA1IE_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TMRA1IE_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == TMRA1IE_A::EN
    }
}
#[doc = "Write proxy for field `TMRA1IE`"]
pub struct TMRA1IE_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRA1IE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRA1IE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable counter/timer A1 from generating an interrupt."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRA1IE_A::DIS)
    }
    #[doc = "Enable counter/timer A1 to generate an interrupt."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRA1IE_A::EN)
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
#[doc = "Counter/Timer A1 Function Select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TMRA1FN_A {
    #[doc = "0: Single count (output toggles and sticks).  Count to CMPR0A1, stop."]
    SINGLECOUNT = 0,
    #[doc = "1: Repeated count (periodic 1-clock-cycle-wide pulses).  Count to CMPR0A1, restart."]
    REPEATEDCOUNT = 1,
    #[doc = "2: Pulse once (aka one-shot).  Count to CMPR0A1, assert, count to CMPR1B, deassert, stop."]
    PULSE_ONCE = 2,
    #[doc = "3: Pulse continously.  Count to CMPR0A1, assert, count to CMPR1A1, deassert, restart."]
    PULSE_CONT = 3,
    #[doc = "4: Continuous run (aka Free Run).  Count continuously."]
    CONTINUOUS = 4,
}
impl From<TMRA1FN_A> for u8 {
    #[inline(always)]
    fn from(variant: TMRA1FN_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TMRA1FN`"]
pub type TMRA1FN_R = crate::R<u8, TMRA1FN_A>;
impl TMRA1FN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TMRA1FN_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TMRA1FN_A::SINGLECOUNT),
            1 => Val(TMRA1FN_A::REPEATEDCOUNT),
            2 => Val(TMRA1FN_A::PULSE_ONCE),
            3 => Val(TMRA1FN_A::PULSE_CONT),
            4 => Val(TMRA1FN_A::CONTINUOUS),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SINGLECOUNT`"]
    #[inline(always)]
    pub fn is_singlecount(&self) -> bool {
        *self == TMRA1FN_A::SINGLECOUNT
    }
    #[doc = "Checks if the value of the field is `REPEATEDCOUNT`"]
    #[inline(always)]
    pub fn is_repeatedcount(&self) -> bool {
        *self == TMRA1FN_A::REPEATEDCOUNT
    }
    #[doc = "Checks if the value of the field is `PULSE_ONCE`"]
    #[inline(always)]
    pub fn is_pulse_once(&self) -> bool {
        *self == TMRA1FN_A::PULSE_ONCE
    }
    #[doc = "Checks if the value of the field is `PULSE_CONT`"]
    #[inline(always)]
    pub fn is_pulse_cont(&self) -> bool {
        *self == TMRA1FN_A::PULSE_CONT
    }
    #[doc = "Checks if the value of the field is `CONTINUOUS`"]
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        *self == TMRA1FN_A::CONTINUOUS
    }
}
#[doc = "Write proxy for field `TMRA1FN`"]
pub struct TMRA1FN_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRA1FN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRA1FN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Single count (output toggles and sticks). Count to CMPR0A1, stop."]
    #[inline(always)]
    pub fn singlecount(self) -> &'a mut W {
        self.variant(TMRA1FN_A::SINGLECOUNT)
    }
    #[doc = "Repeated count (periodic 1-clock-cycle-wide pulses). Count to CMPR0A1, restart."]
    #[inline(always)]
    pub fn repeatedcount(self) -> &'a mut W {
        self.variant(TMRA1FN_A::REPEATEDCOUNT)
    }
    #[doc = "Pulse once (aka one-shot). Count to CMPR0A1, assert, count to CMPR1B, deassert, stop."]
    #[inline(always)]
    pub fn pulse_once(self) -> &'a mut W {
        self.variant(TMRA1FN_A::PULSE_ONCE)
    }
    #[doc = "Pulse continously. Count to CMPR0A1, assert, count to CMPR1A1, deassert, restart."]
    #[inline(always)]
    pub fn pulse_cont(self) -> &'a mut W {
        self.variant(TMRA1FN_A::PULSE_CONT)
    }
    #[doc = "Continuous run (aka Free Run). Count continuously."]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut W {
        self.variant(TMRA1FN_A::CONTINUOUS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 6)) | (((value as u32) & 0x07) << 6);
        self.w
    }
}
#[doc = "Counter/Timer A1 Clock Select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TMRA1CLK_A {
    #[doc = "0: Clock source is TMRPINA."]
    TMRPIN = 0,
    #[doc = "1: Clock source is the HFRC"]
    HFRC = 1,
    #[doc = "2: Clock source is the HFRC / 8"]
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
impl From<TMRA1CLK_A> for u8 {
    #[inline(always)]
    fn from(variant: TMRA1CLK_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TMRA1CLK`"]
pub type TMRA1CLK_R = crate::R<u8, TMRA1CLK_A>;
impl TMRA1CLK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TMRA1CLK_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TMRA1CLK_A::TMRPIN),
            1 => Val(TMRA1CLK_A::HFRC),
            2 => Val(TMRA1CLK_A::HFRC_DIV8),
            3 => Val(TMRA1CLK_A::HFRC_DIV128),
            4 => Val(TMRA1CLK_A::HFRC_DIV512),
            5 => Val(TMRA1CLK_A::HFRC_DIV2K),
            6 => Val(TMRA1CLK_A::XT),
            7 => Val(TMRA1CLK_A::XT_DIV2),
            8 => Val(TMRA1CLK_A::XT_DIV16),
            9 => Val(TMRA1CLK_A::XT_DIV256),
            10 => Val(TMRA1CLK_A::LFRC_DIV2),
            11 => Val(TMRA1CLK_A::LFRC_DIV32),
            12 => Val(TMRA1CLK_A::LFRC_DIV1K),
            13 => Val(TMRA1CLK_A::LFRC),
            14 => Val(TMRA1CLK_A::RTC_100HZ),
            15 => Val(TMRA1CLK_A::HCLK),
            16 => Val(TMRA1CLK_A::BUCKA),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TMRPIN`"]
    #[inline(always)]
    pub fn is_tmrpin(&self) -> bool {
        *self == TMRA1CLK_A::TMRPIN
    }
    #[doc = "Checks if the value of the field is `HFRC`"]
    #[inline(always)]
    pub fn is_hfrc(&self) -> bool {
        *self == TMRA1CLK_A::HFRC
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV8`"]
    #[inline(always)]
    pub fn is_hfrc_div8(&self) -> bool {
        *self == TMRA1CLK_A::HFRC_DIV8
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV128`"]
    #[inline(always)]
    pub fn is_hfrc_div128(&self) -> bool {
        *self == TMRA1CLK_A::HFRC_DIV128
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV512`"]
    #[inline(always)]
    pub fn is_hfrc_div512(&self) -> bool {
        *self == TMRA1CLK_A::HFRC_DIV512
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV2K`"]
    #[inline(always)]
    pub fn is_hfrc_div2k(&self) -> bool {
        *self == TMRA1CLK_A::HFRC_DIV2K
    }
    #[doc = "Checks if the value of the field is `XT`"]
    #[inline(always)]
    pub fn is_xt(&self) -> bool {
        *self == TMRA1CLK_A::XT
    }
    #[doc = "Checks if the value of the field is `XT_DIV2`"]
    #[inline(always)]
    pub fn is_xt_div2(&self) -> bool {
        *self == TMRA1CLK_A::XT_DIV2
    }
    #[doc = "Checks if the value of the field is `XT_DIV16`"]
    #[inline(always)]
    pub fn is_xt_div16(&self) -> bool {
        *self == TMRA1CLK_A::XT_DIV16
    }
    #[doc = "Checks if the value of the field is `XT_DIV256`"]
    #[inline(always)]
    pub fn is_xt_div256(&self) -> bool {
        *self == TMRA1CLK_A::XT_DIV256
    }
    #[doc = "Checks if the value of the field is `LFRC_DIV2`"]
    #[inline(always)]
    pub fn is_lfrc_div2(&self) -> bool {
        *self == TMRA1CLK_A::LFRC_DIV2
    }
    #[doc = "Checks if the value of the field is `LFRC_DIV32`"]
    #[inline(always)]
    pub fn is_lfrc_div32(&self) -> bool {
        *self == TMRA1CLK_A::LFRC_DIV32
    }
    #[doc = "Checks if the value of the field is `LFRC_DIV1K`"]
    #[inline(always)]
    pub fn is_lfrc_div1k(&self) -> bool {
        *self == TMRA1CLK_A::LFRC_DIV1K
    }
    #[doc = "Checks if the value of the field is `LFRC`"]
    #[inline(always)]
    pub fn is_lfrc(&self) -> bool {
        *self == TMRA1CLK_A::LFRC
    }
    #[doc = "Checks if the value of the field is `RTC_100HZ`"]
    #[inline(always)]
    pub fn is_rtc_100hz(&self) -> bool {
        *self == TMRA1CLK_A::RTC_100HZ
    }
    #[doc = "Checks if the value of the field is `HCLK`"]
    #[inline(always)]
    pub fn is_hclk(&self) -> bool {
        *self == TMRA1CLK_A::HCLK
    }
    #[doc = "Checks if the value of the field is `BUCKA`"]
    #[inline(always)]
    pub fn is_bucka(&self) -> bool {
        *self == TMRA1CLK_A::BUCKA
    }
}
#[doc = "Write proxy for field `TMRA1CLK`"]
pub struct TMRA1CLK_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRA1CLK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRA1CLK_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Clock source is TMRPINA."]
    #[inline(always)]
    pub fn tmrpin(self) -> &'a mut W {
        self.variant(TMRA1CLK_A::TMRPIN)
    }
    #[doc = "Clock source is the HFRC"]
    #[inline(always)]
    pub fn hfrc(self) -> &'a mut W {
        self.variant(TMRA1CLK_A::HFRC)
    }
    #[doc = "Clock source is the HFRC / 8"]
    #[inline(always)]
    pub fn hfrc_div8(self) -> &'a mut W {
        self.variant(TMRA1CLK_A::HFRC_DIV8)
    }
    #[doc = "Clock source is HFRC / 128"]
    #[inline(always)]
    pub fn hfrc_div128(self) -> &'a mut W {
        self.variant(TMRA1CLK_A::HFRC_DIV128)
    }
    #[doc = "Clock source is HFRC / 512"]
    #[inline(always)]
    pub fn hfrc_div512(self) -> &'a mut W {
        self.variant(TMRA1CLK_A::HFRC_DIV512)
    }
    #[doc = "Clock source is HFRC / 2048"]
    #[inline(always)]
    pub fn hfrc_div2k(self) -> &'a mut W {
        self.variant(TMRA1CLK_A::HFRC_DIV2K)
    }
    #[doc = "Clock source is the XT (uncalibrated)."]
    #[inline(always)]
    pub fn xt(self) -> &'a mut W {
        self.variant(TMRA1CLK_A::XT)
    }
    #[doc = "Clock source is XT / 2"]
    #[inline(always)]
    pub fn xt_div2(self) -> &'a mut W {
        self.variant(TMRA1CLK_A::XT_DIV2)
    }
    #[doc = "Clock source is XT / 16"]
    #[inline(always)]
    pub fn xt_div16(self) -> &'a mut W {
        self.variant(TMRA1CLK_A::XT_DIV16)
    }
    #[doc = "Clock source is XT / 256"]
    #[inline(always)]
    pub fn xt_div256(self) -> &'a mut W {
        self.variant(TMRA1CLK_A::XT_DIV256)
    }
    #[doc = "Clock source is LFRC / 2"]
    #[inline(always)]
    pub fn lfrc_div2(self) -> &'a mut W {
        self.variant(TMRA1CLK_A::LFRC_DIV2)
    }
    #[doc = "Clock source is LFRC / 32"]
    #[inline(always)]
    pub fn lfrc_div32(self) -> &'a mut W {
        self.variant(TMRA1CLK_A::LFRC_DIV32)
    }
    #[doc = "Clock source is LFRC / 1024"]
    #[inline(always)]
    pub fn lfrc_div1k(self) -> &'a mut W {
        self.variant(TMRA1CLK_A::LFRC_DIV1K)
    }
    #[doc = "Clock source is LFRC / 16K"]
    #[inline(always)]
    pub fn lfrc(self) -> &'a mut W {
        self.variant(TMRA1CLK_A::LFRC)
    }
    #[doc = "Clock source is 100 Hz from the current RTC oscillator."]
    #[inline(always)]
    pub fn rtc_100hz(self) -> &'a mut W {
        self.variant(TMRA1CLK_A::RTC_100HZ)
    }
    #[doc = "Clock source is HCLK."]
    #[inline(always)]
    pub fn hclk(self) -> &'a mut W {
        self.variant(TMRA1CLK_A::HCLK)
    }
    #[doc = "Clock source is buck converter stream A."]
    #[inline(always)]
    pub fn bucka(self) -> &'a mut W {
        self.variant(TMRA1CLK_A::BUCKA)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 1)) | (((value as u32) & 0x1f) << 1);
        self.w
    }
}
#[doc = "Counter/Timer A1 Enable bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA1EN_A {
    #[doc = "0: Counter/Timer A1 Disable."]
    DIS = 0,
    #[doc = "1: Counter/Timer A1 Enable."]
    EN = 1,
}
impl From<TMRA1EN_A> for bool {
    #[inline(always)]
    fn from(variant: TMRA1EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TMRA1EN`"]
pub type TMRA1EN_R = crate::R<bool, TMRA1EN_A>;
impl TMRA1EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRA1EN_A {
        match self.bits {
            false => TMRA1EN_A::DIS,
            true => TMRA1EN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TMRA1EN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == TMRA1EN_A::EN
    }
}
#[doc = "Write proxy for field `TMRA1EN`"]
pub struct TMRA1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRA1EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRA1EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Counter/Timer A1 Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRA1EN_A::DIS)
    }
    #[doc = "Counter/Timer A1 Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRA1EN_A::EN)
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
    #[doc = "Bit 31 - Counter/Timer A1/B1 Link bit."]
    #[inline(always)]
    pub fn ctlink1(&self) -> CTLINK1_R {
        CTLINK1_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Counter/Timer B1 output polarity."]
    #[inline(always)]
    pub fn tmrb1pol(&self) -> TMRB1POL_R {
        TMRB1POL_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Counter/Timer B1 Clear bit."]
    #[inline(always)]
    pub fn tmrb1clr(&self) -> TMRB1CLR_R {
        TMRB1CLR_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Counter/Timer B1 Output Enable bit."]
    #[inline(always)]
    pub fn tmrb1pe(&self) -> TMRB1PE_R {
        TMRB1PE_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Counter/Timer B1 Interrupt Enable bit."]
    #[inline(always)]
    pub fn tmrb1ie(&self) -> TMRB1IE_R {
        TMRB1IE_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bits 22:24 - Counter/Timer B1 Function Select."]
    #[inline(always)]
    pub fn tmrb1fn(&self) -> TMRB1FN_R {
        TMRB1FN_R::new(((self.bits >> 22) & 0x07) as u8)
    }
    #[doc = "Bits 17:21 - Counter/Timer B1 Clock Select."]
    #[inline(always)]
    pub fn tmrb1clk(&self) -> TMRB1CLK_R {
        TMRB1CLK_R::new(((self.bits >> 17) & 0x1f) as u8)
    }
    #[doc = "Bit 16 - Counter/Timer B1 Enable bit."]
    #[inline(always)]
    pub fn tmrb1en(&self) -> TMRB1EN_R {
        TMRB1EN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Counter/Timer A1 output polarity."]
    #[inline(always)]
    pub fn tmra1pol(&self) -> TMRA1POL_R {
        TMRA1POL_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Counter/Timer A1 Clear bit."]
    #[inline(always)]
    pub fn tmra1clr(&self) -> TMRA1CLR_R {
        TMRA1CLR_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Counter/Timer A1 Output Enable bit."]
    #[inline(always)]
    pub fn tmra1pe(&self) -> TMRA1PE_R {
        TMRA1PE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Counter/Timer A1 Interrupt Enable bit."]
    #[inline(always)]
    pub fn tmra1ie(&self) -> TMRA1IE_R {
        TMRA1IE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bits 6:8 - Counter/Timer A1 Function Select."]
    #[inline(always)]
    pub fn tmra1fn(&self) -> TMRA1FN_R {
        TMRA1FN_R::new(((self.bits >> 6) & 0x07) as u8)
    }
    #[doc = "Bits 1:5 - Counter/Timer A1 Clock Select."]
    #[inline(always)]
    pub fn tmra1clk(&self) -> TMRA1CLK_R {
        TMRA1CLK_R::new(((self.bits >> 1) & 0x1f) as u8)
    }
    #[doc = "Bit 0 - Counter/Timer A1 Enable bit."]
    #[inline(always)]
    pub fn tmra1en(&self) -> TMRA1EN_R {
        TMRA1EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - Counter/Timer A1/B1 Link bit."]
    #[inline(always)]
    pub fn ctlink1(&mut self) -> CTLINK1_W {
        CTLINK1_W { w: self }
    }
    #[doc = "Bit 28 - Counter/Timer B1 output polarity."]
    #[inline(always)]
    pub fn tmrb1pol(&mut self) -> TMRB1POL_W {
        TMRB1POL_W { w: self }
    }
    #[doc = "Bit 27 - Counter/Timer B1 Clear bit."]
    #[inline(always)]
    pub fn tmrb1clr(&mut self) -> TMRB1CLR_W {
        TMRB1CLR_W { w: self }
    }
    #[doc = "Bit 26 - Counter/Timer B1 Output Enable bit."]
    #[inline(always)]
    pub fn tmrb1pe(&mut self) -> TMRB1PE_W {
        TMRB1PE_W { w: self }
    }
    #[doc = "Bit 25 - Counter/Timer B1 Interrupt Enable bit."]
    #[inline(always)]
    pub fn tmrb1ie(&mut self) -> TMRB1IE_W {
        TMRB1IE_W { w: self }
    }
    #[doc = "Bits 22:24 - Counter/Timer B1 Function Select."]
    #[inline(always)]
    pub fn tmrb1fn(&mut self) -> TMRB1FN_W {
        TMRB1FN_W { w: self }
    }
    #[doc = "Bits 17:21 - Counter/Timer B1 Clock Select."]
    #[inline(always)]
    pub fn tmrb1clk(&mut self) -> TMRB1CLK_W {
        TMRB1CLK_W { w: self }
    }
    #[doc = "Bit 16 - Counter/Timer B1 Enable bit."]
    #[inline(always)]
    pub fn tmrb1en(&mut self) -> TMRB1EN_W {
        TMRB1EN_W { w: self }
    }
    #[doc = "Bit 12 - Counter/Timer A1 output polarity."]
    #[inline(always)]
    pub fn tmra1pol(&mut self) -> TMRA1POL_W {
        TMRA1POL_W { w: self }
    }
    #[doc = "Bit 11 - Counter/Timer A1 Clear bit."]
    #[inline(always)]
    pub fn tmra1clr(&mut self) -> TMRA1CLR_W {
        TMRA1CLR_W { w: self }
    }
    #[doc = "Bit 10 - Counter/Timer A1 Output Enable bit."]
    #[inline(always)]
    pub fn tmra1pe(&mut self) -> TMRA1PE_W {
        TMRA1PE_W { w: self }
    }
    #[doc = "Bit 9 - Counter/Timer A1 Interrupt Enable bit."]
    #[inline(always)]
    pub fn tmra1ie(&mut self) -> TMRA1IE_W {
        TMRA1IE_W { w: self }
    }
    #[doc = "Bits 6:8 - Counter/Timer A1 Function Select."]
    #[inline(always)]
    pub fn tmra1fn(&mut self) -> TMRA1FN_W {
        TMRA1FN_W { w: self }
    }
    #[doc = "Bits 1:5 - Counter/Timer A1 Clock Select."]
    #[inline(always)]
    pub fn tmra1clk(&mut self) -> TMRA1CLK_W {
        TMRA1CLK_W { w: self }
    }
    #[doc = "Bit 0 - Counter/Timer A1 Enable bit."]
    #[inline(always)]
    pub fn tmra1en(&mut self) -> TMRA1EN_W {
        TMRA1EN_W { w: self }
    }
}
