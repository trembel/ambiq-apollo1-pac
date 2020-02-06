#[doc = "Reader of register INTCLR"]
pub type R = crate::R<u32, super::INTCLR>;
#[doc = "Writer for register INTCLR"]
pub type W = crate::W<u32, super::INTCLR>;
#[doc = "Register INTCLR `reset()`'s with value 0"]
impl crate::ResetValue for super::INTCLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Window comparator voltage incursion interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WCINC_A {
    #[doc = "1: Window comparitor voltage incursion interrupt."]
    WCINCINT = 1,
}
impl From<WCINC_A> for bool {
    #[inline(always)]
    fn from(variant: WCINC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WCINC`"]
pub type WCINC_R = crate::R<bool, WCINC_A>;
impl WCINC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, WCINC_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(WCINC_A::WCINCINT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `WCINCINT`"]
    #[inline(always)]
    pub fn is_wcincint(&self) -> bool {
        *self == WCINC_A::WCINCINT
    }
}
#[doc = "Write proxy for field `WCINC`"]
pub struct WCINC_W<'a> {
    w: &'a mut W,
}
impl<'a> WCINC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WCINC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Window comparitor voltage incursion interrupt."]
    #[inline(always)]
    pub fn wcincint(self) -> &'a mut W {
        self.variant(WCINC_A::WCINCINT)
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
#[doc = "Window comparator voltage excursion interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WCEXC_A {
    #[doc = "1: Window comparitor voltage excursion interrupt."]
    WCEXCINT = 1,
}
impl From<WCEXC_A> for bool {
    #[inline(always)]
    fn from(variant: WCEXC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WCEXC`"]
pub type WCEXC_R = crate::R<bool, WCEXC_A>;
impl WCEXC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, WCEXC_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(WCEXC_A::WCEXCINT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `WCEXCINT`"]
    #[inline(always)]
    pub fn is_wcexcint(&self) -> bool {
        *self == WCEXC_A::WCEXCINT
    }
}
#[doc = "Write proxy for field `WCEXC`"]
pub struct WCEXC_W<'a> {
    w: &'a mut W,
}
impl<'a> WCEXC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WCEXC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Window comparitor voltage excursion interrupt."]
    #[inline(always)]
    pub fn wcexcint(self) -> &'a mut W {
        self.variant(WCEXC_A::WCEXCINT)
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
#[doc = "FIFO 100 percent full interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIFOOVR2_A {
    #[doc = "1: FIFO 100 percent full interrupt."]
    FIFOFULLINT = 1,
}
impl From<FIFOOVR2_A> for bool {
    #[inline(always)]
    fn from(variant: FIFOOVR2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FIFOOVR2`"]
pub type FIFOOVR2_R = crate::R<bool, FIFOOVR2_A>;
impl FIFOOVR2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, FIFOOVR2_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(FIFOOVR2_A::FIFOFULLINT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `FIFOFULLINT`"]
    #[inline(always)]
    pub fn is_fifofullint(&self) -> bool {
        *self == FIFOOVR2_A::FIFOFULLINT
    }
}
#[doc = "Write proxy for field `FIFOOVR2`"]
pub struct FIFOOVR2_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFOOVR2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FIFOOVR2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "FIFO 100 percent full interrupt."]
    #[inline(always)]
    pub fn fifofullint(self) -> &'a mut W {
        self.variant(FIFOOVR2_A::FIFOFULLINT)
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
#[doc = "FIFO 75 percent full interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIFOOVR1_A {
    #[doc = "1: FIFO 75 percent full interrupt."]
    FIFO75INT = 1,
}
impl From<FIFOOVR1_A> for bool {
    #[inline(always)]
    fn from(variant: FIFOOVR1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FIFOOVR1`"]
pub type FIFOOVR1_R = crate::R<bool, FIFOOVR1_A>;
impl FIFOOVR1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, FIFOOVR1_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(FIFOOVR1_A::FIFO75INT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `FIFO75INT`"]
    #[inline(always)]
    pub fn is_fifo75int(&self) -> bool {
        *self == FIFOOVR1_A::FIFO75INT
    }
}
#[doc = "Write proxy for field `FIFOOVR1`"]
pub struct FIFOOVR1_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFOOVR1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FIFOOVR1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "FIFO 75 percent full interrupt."]
    #[inline(always)]
    pub fn fifo75int(self) -> &'a mut W {
        self.variant(FIFOOVR1_A::FIFO75INT)
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
#[doc = "ADC scan complete interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCNCMP_A {
    #[doc = "1: ADC scan complete interrupt."]
    SCNCMPINT = 1,
}
impl From<SCNCMP_A> for bool {
    #[inline(always)]
    fn from(variant: SCNCMP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SCNCMP`"]
pub type SCNCMP_R = crate::R<bool, SCNCMP_A>;
impl SCNCMP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, SCNCMP_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(SCNCMP_A::SCNCMPINT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SCNCMPINT`"]
    #[inline(always)]
    pub fn is_scncmpint(&self) -> bool {
        *self == SCNCMP_A::SCNCMPINT
    }
}
#[doc = "Write proxy for field `SCNCMP`"]
pub struct SCNCMP_W<'a> {
    w: &'a mut W,
}
impl<'a> SCNCMP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SCNCMP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "ADC scan complete interrupt."]
    #[inline(always)]
    pub fn scncmpint(self) -> &'a mut W {
        self.variant(SCNCMP_A::SCNCMPINT)
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
#[doc = "ADC conversion complete interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CNVCMP_A {
    #[doc = "1: ADC conversion complete interrupt."]
    CNVCMPINT = 1,
}
impl From<CNVCMP_A> for bool {
    #[inline(always)]
    fn from(variant: CNVCMP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CNVCMP`"]
pub type CNVCMP_R = crate::R<bool, CNVCMP_A>;
impl CNVCMP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, CNVCMP_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(CNVCMP_A::CNVCMPINT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CNVCMPINT`"]
    #[inline(always)]
    pub fn is_cnvcmpint(&self) -> bool {
        *self == CNVCMP_A::CNVCMPINT
    }
}
#[doc = "Write proxy for field `CNVCMP`"]
pub struct CNVCMP_W<'a> {
    w: &'a mut W,
}
impl<'a> CNVCMP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CNVCMP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "ADC conversion complete interrupt."]
    #[inline(always)]
    pub fn cnvcmpint(self) -> &'a mut W {
        self.variant(CNVCMP_A::CNVCMPINT)
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
    #[doc = "Bit 5 - Window comparator voltage incursion interrupt."]
    #[inline(always)]
    pub fn wcinc(&self) -> WCINC_R {
        WCINC_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Window comparator voltage excursion interrupt."]
    #[inline(always)]
    pub fn wcexc(&self) -> WCEXC_R {
        WCEXC_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - FIFO 100 percent full interrupt."]
    #[inline(always)]
    pub fn fifoovr2(&self) -> FIFOOVR2_R {
        FIFOOVR2_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - FIFO 75 percent full interrupt."]
    #[inline(always)]
    pub fn fifoovr1(&self) -> FIFOOVR1_R {
        FIFOOVR1_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - ADC scan complete interrupt."]
    #[inline(always)]
    pub fn scncmp(&self) -> SCNCMP_R {
        SCNCMP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - ADC conversion complete interrupt."]
    #[inline(always)]
    pub fn cnvcmp(&self) -> CNVCMP_R {
        CNVCMP_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - Window comparator voltage incursion interrupt."]
    #[inline(always)]
    pub fn wcinc(&mut self) -> WCINC_W {
        WCINC_W { w: self }
    }
    #[doc = "Bit 4 - Window comparator voltage excursion interrupt."]
    #[inline(always)]
    pub fn wcexc(&mut self) -> WCEXC_W {
        WCEXC_W { w: self }
    }
    #[doc = "Bit 3 - FIFO 100 percent full interrupt."]
    #[inline(always)]
    pub fn fifoovr2(&mut self) -> FIFOOVR2_W {
        FIFOOVR2_W { w: self }
    }
    #[doc = "Bit 2 - FIFO 75 percent full interrupt."]
    #[inline(always)]
    pub fn fifoovr1(&mut self) -> FIFOOVR1_W {
        FIFOOVR1_W { w: self }
    }
    #[doc = "Bit 1 - ADC scan complete interrupt."]
    #[inline(always)]
    pub fn scncmp(&mut self) -> SCNCMP_W {
        SCNCMP_W { w: self }
    }
    #[doc = "Bit 0 - ADC conversion complete interrupt."]
    #[inline(always)]
    pub fn cnvcmp(&mut self) -> CNVCMP_W {
        CNVCMP_W { w: self }
    }
}
