#[doc = "Reader of register STAT"]
pub type R = crate::R<u32, super::STAT>;
#[doc = "Writer for register STAT"]
pub type W = crate::W<u32, super::STAT>;
#[doc = "Register STAT `reset()`'s with value 0"]
impl crate::ResetValue for super::STAT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "This bit indicates the power down state of the voltage comparator.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWDSTAT_A {
    #[doc = "1: The voltage comparator is powered down."]
    POWERED_DOWN = 1,
}
impl From<PWDSTAT_A> for bool {
    #[inline(always)]
    fn from(variant: PWDSTAT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PWDSTAT`"]
pub type PWDSTAT_R = crate::R<bool, PWDSTAT_A>;
impl PWDSTAT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, PWDSTAT_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(PWDSTAT_A::POWERED_DOWN),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `POWERED_DOWN`"]
    #[inline(always)]
    pub fn is_powered_down(&self) -> bool {
        *self == PWDSTAT_A::POWERED_DOWN
    }
}
#[doc = "Write proxy for field `PWDSTAT`"]
pub struct PWDSTAT_W<'a> {
    w: &'a mut W,
}
impl<'a> PWDSTAT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWDSTAT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The voltage comparator is powered down."]
    #[inline(always)]
    pub fn powered_down(self) -> &'a mut W {
        self.variant(PWDSTAT_A::POWERED_DOWN)
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
#[doc = "This bit is 1 if the positive input of the comparator is greater than the negative input.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMPOUT_A {
    #[doc = "0: The negative input of the comparator is greater than the positive input."]
    VOUT_LOW = 0,
    #[doc = "1: The positive input of the comparator is greater than the negative input."]
    VOUT_HIGH = 1,
}
impl From<CMPOUT_A> for bool {
    #[inline(always)]
    fn from(variant: CMPOUT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CMPOUT`"]
pub type CMPOUT_R = crate::R<bool, CMPOUT_A>;
impl CMPOUT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPOUT_A {
        match self.bits {
            false => CMPOUT_A::VOUT_LOW,
            true => CMPOUT_A::VOUT_HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `VOUT_LOW`"]
    #[inline(always)]
    pub fn is_vout_low(&self) -> bool {
        *self == CMPOUT_A::VOUT_LOW
    }
    #[doc = "Checks if the value of the field is `VOUT_HIGH`"]
    #[inline(always)]
    pub fn is_vout_high(&self) -> bool {
        *self == CMPOUT_A::VOUT_HIGH
    }
}
#[doc = "Write proxy for field `CMPOUT`"]
pub struct CMPOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPOUT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMPOUT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The negative input of the comparator is greater than the positive input."]
    #[inline(always)]
    pub fn vout_low(self) -> &'a mut W {
        self.variant(CMPOUT_A::VOUT_LOW)
    }
    #[doc = "The positive input of the comparator is greater than the negative input."]
    #[inline(always)]
    pub fn vout_high(self) -> &'a mut W {
        self.variant(CMPOUT_A::VOUT_HIGH)
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
    #[doc = "Bit 1 - This bit indicates the power down state of the voltage comparator."]
    #[inline(always)]
    pub fn pwdstat(&self) -> PWDSTAT_R {
        PWDSTAT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - This bit is 1 if the positive input of the comparator is greater than the negative input."]
    #[inline(always)]
    pub fn cmpout(&self) -> CMPOUT_R {
        CMPOUT_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - This bit indicates the power down state of the voltage comparator."]
    #[inline(always)]
    pub fn pwdstat(&mut self) -> PWDSTAT_W {
        PWDSTAT_W { w: self }
    }
    #[doc = "Bit 0 - This bit is 1 if the positive input of the comparator is greater than the negative input."]
    #[inline(always)]
    pub fn cmpout(&mut self) -> CMPOUT_W {
        CMPOUT_W { w: self }
    }
}
