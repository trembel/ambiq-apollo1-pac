#[doc = "Reader of register SRAMPWDINSLEEP"]
pub type R = crate::R<u32, super::SRAMPWDINSLEEP>;
#[doc = "Writer for register SRAMPWDINSLEEP"]
pub type W = crate::W<u32, super::SRAMPWDINSLEEP>;
#[doc = "Register SRAMPWDINSLEEP `reset()`'s with value 0"]
impl crate::ResetValue for super::SRAMPWDINSLEEP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Force SRAM Bank 7 to powerdown in deep sleep mode, causing the contents of the bank to be lost.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BANK7_A {
    #[doc = "0: SRAM Bank 7 normal operation."]
    NORMAL = 0,
    #[doc = "1: SRAM Bank 7 deep sleep."]
    PWRDN_IN_DEEPSLEEP = 1,
}
impl From<BANK7_A> for bool {
    #[inline(always)]
    fn from(variant: BANK7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BANK7`"]
pub type BANK7_R = crate::R<bool, BANK7_A>;
impl BANK7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BANK7_A {
        match self.bits {
            false => BANK7_A::NORMAL,
            true => BANK7_A::PWRDN_IN_DEEPSLEEP,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == BANK7_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `PWRDN_IN_DEEPSLEEP`"]
    #[inline(always)]
    pub fn is_pwrdn_in_deepsleep(&self) -> bool {
        *self == BANK7_A::PWRDN_IN_DEEPSLEEP
    }
}
#[doc = "Write proxy for field `BANK7`"]
pub struct BANK7_W<'a> {
    w: &'a mut W,
}
impl<'a> BANK7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BANK7_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "SRAM Bank 7 normal operation."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(BANK7_A::NORMAL)
    }
    #[doc = "SRAM Bank 7 deep sleep."]
    #[inline(always)]
    pub fn pwrdn_in_deepsleep(self) -> &'a mut W {
        self.variant(BANK7_A::PWRDN_IN_DEEPSLEEP)
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
#[doc = "Force SRAM Bank 6 to powerdown in deep sleep mode, causing the contents of the bank to be lost.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BANK6_A {
    #[doc = "0: SRAM Bank 6 normal operation."]
    NORMAL = 0,
    #[doc = "1: SRAM Bank 6 deep sleep."]
    PWRDN_IN_DEEPSLEEP = 1,
}
impl From<BANK6_A> for bool {
    #[inline(always)]
    fn from(variant: BANK6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BANK6`"]
pub type BANK6_R = crate::R<bool, BANK6_A>;
impl BANK6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BANK6_A {
        match self.bits {
            false => BANK6_A::NORMAL,
            true => BANK6_A::PWRDN_IN_DEEPSLEEP,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == BANK6_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `PWRDN_IN_DEEPSLEEP`"]
    #[inline(always)]
    pub fn is_pwrdn_in_deepsleep(&self) -> bool {
        *self == BANK6_A::PWRDN_IN_DEEPSLEEP
    }
}
#[doc = "Write proxy for field `BANK6`"]
pub struct BANK6_W<'a> {
    w: &'a mut W,
}
impl<'a> BANK6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BANK6_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "SRAM Bank 6 normal operation."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(BANK6_A::NORMAL)
    }
    #[doc = "SRAM Bank 6 deep sleep."]
    #[inline(always)]
    pub fn pwrdn_in_deepsleep(self) -> &'a mut W {
        self.variant(BANK6_A::PWRDN_IN_DEEPSLEEP)
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
#[doc = "Force SRAM Bank 5 to powerdown in deep sleep mode, causing the contents of the bank to be lost.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BANK5_A {
    #[doc = "0: SRAM Bank 5 normal operation."]
    NORMAL = 0,
    #[doc = "1: SRAM Bank 5 deep sleep."]
    PWRDN_IN_DEEPSLEEP = 1,
}
impl From<BANK5_A> for bool {
    #[inline(always)]
    fn from(variant: BANK5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BANK5`"]
pub type BANK5_R = crate::R<bool, BANK5_A>;
impl BANK5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BANK5_A {
        match self.bits {
            false => BANK5_A::NORMAL,
            true => BANK5_A::PWRDN_IN_DEEPSLEEP,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == BANK5_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `PWRDN_IN_DEEPSLEEP`"]
    #[inline(always)]
    pub fn is_pwrdn_in_deepsleep(&self) -> bool {
        *self == BANK5_A::PWRDN_IN_DEEPSLEEP
    }
}
#[doc = "Write proxy for field `BANK5`"]
pub struct BANK5_W<'a> {
    w: &'a mut W,
}
impl<'a> BANK5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BANK5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "SRAM Bank 5 normal operation."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(BANK5_A::NORMAL)
    }
    #[doc = "SRAM Bank 5 deep sleep."]
    #[inline(always)]
    pub fn pwrdn_in_deepsleep(self) -> &'a mut W {
        self.variant(BANK5_A::PWRDN_IN_DEEPSLEEP)
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
#[doc = "Force SRAM Bank 4 to powerdown in deep sleep mode, causing the contents of the bank to be lost.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BANK4_A {
    #[doc = "0: SRAM Bank 4 normal operation."]
    NORMAL = 0,
    #[doc = "1: SRAM Bank 4 deep sleep."]
    PWRDN_IN_DEEPSLEEP = 1,
}
impl From<BANK4_A> for bool {
    #[inline(always)]
    fn from(variant: BANK4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BANK4`"]
pub type BANK4_R = crate::R<bool, BANK4_A>;
impl BANK4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BANK4_A {
        match self.bits {
            false => BANK4_A::NORMAL,
            true => BANK4_A::PWRDN_IN_DEEPSLEEP,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == BANK4_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `PWRDN_IN_DEEPSLEEP`"]
    #[inline(always)]
    pub fn is_pwrdn_in_deepsleep(&self) -> bool {
        *self == BANK4_A::PWRDN_IN_DEEPSLEEP
    }
}
#[doc = "Write proxy for field `BANK4`"]
pub struct BANK4_W<'a> {
    w: &'a mut W,
}
impl<'a> BANK4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BANK4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "SRAM Bank 4 normal operation."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(BANK4_A::NORMAL)
    }
    #[doc = "SRAM Bank 4 deep sleep."]
    #[inline(always)]
    pub fn pwrdn_in_deepsleep(self) -> &'a mut W {
        self.variant(BANK4_A::PWRDN_IN_DEEPSLEEP)
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
#[doc = "Force SRAM Bank 3 to powerdown in deep sleep mode, causing the contents of the bank to be lost.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BANK3_A {
    #[doc = "0: SRAM Bank 3 normal operation."]
    NORMAL = 0,
    #[doc = "1: SRAM Bank 3 deep sleep."]
    PWRDN_IN_DEEPSLEEP = 1,
}
impl From<BANK3_A> for bool {
    #[inline(always)]
    fn from(variant: BANK3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BANK3`"]
pub type BANK3_R = crate::R<bool, BANK3_A>;
impl BANK3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BANK3_A {
        match self.bits {
            false => BANK3_A::NORMAL,
            true => BANK3_A::PWRDN_IN_DEEPSLEEP,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == BANK3_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `PWRDN_IN_DEEPSLEEP`"]
    #[inline(always)]
    pub fn is_pwrdn_in_deepsleep(&self) -> bool {
        *self == BANK3_A::PWRDN_IN_DEEPSLEEP
    }
}
#[doc = "Write proxy for field `BANK3`"]
pub struct BANK3_W<'a> {
    w: &'a mut W,
}
impl<'a> BANK3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BANK3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "SRAM Bank 3 normal operation."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(BANK3_A::NORMAL)
    }
    #[doc = "SRAM Bank 3 deep sleep."]
    #[inline(always)]
    pub fn pwrdn_in_deepsleep(self) -> &'a mut W {
        self.variant(BANK3_A::PWRDN_IN_DEEPSLEEP)
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
#[doc = "Force SRAM Bank 2 to powerdown in deep sleep mode, causing the contents of the bank to be lost.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BANK2_A {
    #[doc = "0: SRAM Bank 2 normal operation."]
    NORMAL = 0,
    #[doc = "1: SRAM Bank 2 deep sleep."]
    PWRDN_IN_DEEPSLEEP = 1,
}
impl From<BANK2_A> for bool {
    #[inline(always)]
    fn from(variant: BANK2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BANK2`"]
pub type BANK2_R = crate::R<bool, BANK2_A>;
impl BANK2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BANK2_A {
        match self.bits {
            false => BANK2_A::NORMAL,
            true => BANK2_A::PWRDN_IN_DEEPSLEEP,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == BANK2_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `PWRDN_IN_DEEPSLEEP`"]
    #[inline(always)]
    pub fn is_pwrdn_in_deepsleep(&self) -> bool {
        *self == BANK2_A::PWRDN_IN_DEEPSLEEP
    }
}
#[doc = "Write proxy for field `BANK2`"]
pub struct BANK2_W<'a> {
    w: &'a mut W,
}
impl<'a> BANK2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BANK2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "SRAM Bank 2 normal operation."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(BANK2_A::NORMAL)
    }
    #[doc = "SRAM Bank 2 deep sleep."]
    #[inline(always)]
    pub fn pwrdn_in_deepsleep(self) -> &'a mut W {
        self.variant(BANK2_A::PWRDN_IN_DEEPSLEEP)
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
#[doc = "Force SRAM Bank 1 to powerdown in deep sleep mode, causing the contents of the bank to be lost.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BANK1_A {
    #[doc = "0: SRAM Bank 1 normal operation."]
    NORMAL = 0,
    #[doc = "1: SRAM Bank 1 deep sleep."]
    PWRDN_IN_DEEPSLEEP = 1,
}
impl From<BANK1_A> for bool {
    #[inline(always)]
    fn from(variant: BANK1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BANK1`"]
pub type BANK1_R = crate::R<bool, BANK1_A>;
impl BANK1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BANK1_A {
        match self.bits {
            false => BANK1_A::NORMAL,
            true => BANK1_A::PWRDN_IN_DEEPSLEEP,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == BANK1_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `PWRDN_IN_DEEPSLEEP`"]
    #[inline(always)]
    pub fn is_pwrdn_in_deepsleep(&self) -> bool {
        *self == BANK1_A::PWRDN_IN_DEEPSLEEP
    }
}
#[doc = "Write proxy for field `BANK1`"]
pub struct BANK1_W<'a> {
    w: &'a mut W,
}
impl<'a> BANK1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BANK1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "SRAM Bank 1 normal operation."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(BANK1_A::NORMAL)
    }
    #[doc = "SRAM Bank 1 deep sleep."]
    #[inline(always)]
    pub fn pwrdn_in_deepsleep(self) -> &'a mut W {
        self.variant(BANK1_A::PWRDN_IN_DEEPSLEEP)
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
#[doc = "Force SRAM Bank 0 to powerdown in deep sleep mode, causing the contents of the bank to be lost.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BANK0_A {
    #[doc = "0: SRAM Bank 0 normal operation."]
    NORMAL = 0,
    #[doc = "1: SRAM Bank 0 deep sleep."]
    PWRDN_IN_DEEPSLEEP = 1,
}
impl From<BANK0_A> for bool {
    #[inline(always)]
    fn from(variant: BANK0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BANK0`"]
pub type BANK0_R = crate::R<bool, BANK0_A>;
impl BANK0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BANK0_A {
        match self.bits {
            false => BANK0_A::NORMAL,
            true => BANK0_A::PWRDN_IN_DEEPSLEEP,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == BANK0_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `PWRDN_IN_DEEPSLEEP`"]
    #[inline(always)]
    pub fn is_pwrdn_in_deepsleep(&self) -> bool {
        *self == BANK0_A::PWRDN_IN_DEEPSLEEP
    }
}
#[doc = "Write proxy for field `BANK0`"]
pub struct BANK0_W<'a> {
    w: &'a mut W,
}
impl<'a> BANK0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BANK0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "SRAM Bank 0 normal operation."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(BANK0_A::NORMAL)
    }
    #[doc = "SRAM Bank 0 deep sleep."]
    #[inline(always)]
    pub fn pwrdn_in_deepsleep(self) -> &'a mut W {
        self.variant(BANK0_A::PWRDN_IN_DEEPSLEEP)
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
    #[doc = "Bit 7 - Force SRAM Bank 7 to powerdown in deep sleep mode, causing the contents of the bank to be lost."]
    #[inline(always)]
    pub fn bank7(&self) -> BANK7_R {
        BANK7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Force SRAM Bank 6 to powerdown in deep sleep mode, causing the contents of the bank to be lost."]
    #[inline(always)]
    pub fn bank6(&self) -> BANK6_R {
        BANK6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Force SRAM Bank 5 to powerdown in deep sleep mode, causing the contents of the bank to be lost."]
    #[inline(always)]
    pub fn bank5(&self) -> BANK5_R {
        BANK5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Force SRAM Bank 4 to powerdown in deep sleep mode, causing the contents of the bank to be lost."]
    #[inline(always)]
    pub fn bank4(&self) -> BANK4_R {
        BANK4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Force SRAM Bank 3 to powerdown in deep sleep mode, causing the contents of the bank to be lost."]
    #[inline(always)]
    pub fn bank3(&self) -> BANK3_R {
        BANK3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Force SRAM Bank 2 to powerdown in deep sleep mode, causing the contents of the bank to be lost."]
    #[inline(always)]
    pub fn bank2(&self) -> BANK2_R {
        BANK2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Force SRAM Bank 1 to powerdown in deep sleep mode, causing the contents of the bank to be lost."]
    #[inline(always)]
    pub fn bank1(&self) -> BANK1_R {
        BANK1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Force SRAM Bank 0 to powerdown in deep sleep mode, causing the contents of the bank to be lost."]
    #[inline(always)]
    pub fn bank0(&self) -> BANK0_R {
        BANK0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - Force SRAM Bank 7 to powerdown in deep sleep mode, causing the contents of the bank to be lost."]
    #[inline(always)]
    pub fn bank7(&mut self) -> BANK7_W {
        BANK7_W { w: self }
    }
    #[doc = "Bit 6 - Force SRAM Bank 6 to powerdown in deep sleep mode, causing the contents of the bank to be lost."]
    #[inline(always)]
    pub fn bank6(&mut self) -> BANK6_W {
        BANK6_W { w: self }
    }
    #[doc = "Bit 5 - Force SRAM Bank 5 to powerdown in deep sleep mode, causing the contents of the bank to be lost."]
    #[inline(always)]
    pub fn bank5(&mut self) -> BANK5_W {
        BANK5_W { w: self }
    }
    #[doc = "Bit 4 - Force SRAM Bank 4 to powerdown in deep sleep mode, causing the contents of the bank to be lost."]
    #[inline(always)]
    pub fn bank4(&mut self) -> BANK4_W {
        BANK4_W { w: self }
    }
    #[doc = "Bit 3 - Force SRAM Bank 3 to powerdown in deep sleep mode, causing the contents of the bank to be lost."]
    #[inline(always)]
    pub fn bank3(&mut self) -> BANK3_W {
        BANK3_W { w: self }
    }
    #[doc = "Bit 2 - Force SRAM Bank 2 to powerdown in deep sleep mode, causing the contents of the bank to be lost."]
    #[inline(always)]
    pub fn bank2(&mut self) -> BANK2_W {
        BANK2_W { w: self }
    }
    #[doc = "Bit 1 - Force SRAM Bank 1 to powerdown in deep sleep mode, causing the contents of the bank to be lost."]
    #[inline(always)]
    pub fn bank1(&mut self) -> BANK1_W {
        BANK1_W { w: self }
    }
    #[doc = "Bit 0 - Force SRAM Bank 0 to powerdown in deep sleep mode, causing the contents of the bank to be lost."]
    #[inline(always)]
    pub fn bank0(&mut self) -> BANK0_W {
        BANK0_W { w: self }
    }
}
