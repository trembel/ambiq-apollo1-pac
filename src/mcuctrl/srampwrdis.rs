#[doc = "Reader of register SRAMPWRDIS"]
pub type R = crate::R<u32, super::SRAMPWRDIS>;
#[doc = "Writer for register SRAMPWRDIS"]
pub type W = crate::W<u32, super::SRAMPWRDIS>;
#[doc = "Register SRAMPWRDIS `reset()`'s with value 0"]
impl crate::ResetValue for super::SRAMPWRDIS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Remove power from SRAM Bank 7 which will cause an access to its address space to generate a Hard Fault.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BANK7_A {
    #[doc = "1: Disable SRAM Bank 7."]
    DIS = 1,
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
    pub fn variant(&self) -> crate::Variant<bool, BANK7_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(BANK7_A::DIS),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == BANK7_A::DIS
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
    #[doc = "Disable SRAM Bank 7."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(BANK7_A::DIS)
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
#[doc = "Remove power from SRAM Bank 6 which will cause an access to its address space to generate a Hard Fault.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BANK6_A {
    #[doc = "1: Disable SRAM Bank 6."]
    DIS = 1,
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
    pub fn variant(&self) -> crate::Variant<bool, BANK6_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(BANK6_A::DIS),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == BANK6_A::DIS
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
    #[doc = "Disable SRAM Bank 6."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(BANK6_A::DIS)
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
#[doc = "Remove power from SRAM Bank 5 which will cause an access to its address space to generate a Hard Fault.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BANK5_A {
    #[doc = "1: Disable SRAM Bank 5."]
    DIS = 1,
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
    pub fn variant(&self) -> crate::Variant<bool, BANK5_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(BANK5_A::DIS),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == BANK5_A::DIS
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
    #[doc = "Disable SRAM Bank 5."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(BANK5_A::DIS)
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
#[doc = "Remove power from SRAM Bank 4 which will cause an access to its address space to generate a Hard Fault.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BANK4_A {
    #[doc = "1: Disable SRAM Bank 4."]
    DIS = 1,
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
    pub fn variant(&self) -> crate::Variant<bool, BANK4_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(BANK4_A::DIS),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == BANK4_A::DIS
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
    #[doc = "Disable SRAM Bank 4."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(BANK4_A::DIS)
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
#[doc = "Remove power from SRAM Bank 3 which will cause an access to its address space to generate a Hard Fault.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BANK3_A {
    #[doc = "1: Disable SRAM Bank 3."]
    DIS = 1,
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
    pub fn variant(&self) -> crate::Variant<bool, BANK3_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(BANK3_A::DIS),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == BANK3_A::DIS
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
    #[doc = "Disable SRAM Bank 3."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(BANK3_A::DIS)
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
#[doc = "Remove power from SRAM Bank 2 which will cause an access to its address space to generate a Hard Fault.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BANK2_A {
    #[doc = "1: Disable SRAM Bank 2."]
    DIS = 1,
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
    pub fn variant(&self) -> crate::Variant<bool, BANK2_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(BANK2_A::DIS),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == BANK2_A::DIS
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
    #[doc = "Disable SRAM Bank 2."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(BANK2_A::DIS)
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
#[doc = "Remove power from SRAM Bank 1 which will cause an access to its address space to generate a Hard Fault.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BANK1_A {
    #[doc = "1: Disable SRAM Bank 1."]
    DIS = 1,
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
    pub fn variant(&self) -> crate::Variant<bool, BANK1_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(BANK1_A::DIS),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == BANK1_A::DIS
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
    #[doc = "Disable SRAM Bank 1."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(BANK1_A::DIS)
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
#[doc = "Remove power from SRAM Bank 0 which will cause an access to its address space to generate a Hard Fault.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BANK0_A {
    #[doc = "1: Disable SRAM Bank 0."]
    DIS = 1,
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
    pub fn variant(&self) -> crate::Variant<bool, BANK0_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(BANK0_A::DIS),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == BANK0_A::DIS
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
    #[doc = "Disable SRAM Bank 0."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(BANK0_A::DIS)
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
    #[doc = "Bit 7 - Remove power from SRAM Bank 7 which will cause an access to its address space to generate a Hard Fault."]
    #[inline(always)]
    pub fn bank7(&self) -> BANK7_R {
        BANK7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Remove power from SRAM Bank 6 which will cause an access to its address space to generate a Hard Fault."]
    #[inline(always)]
    pub fn bank6(&self) -> BANK6_R {
        BANK6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Remove power from SRAM Bank 5 which will cause an access to its address space to generate a Hard Fault."]
    #[inline(always)]
    pub fn bank5(&self) -> BANK5_R {
        BANK5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Remove power from SRAM Bank 4 which will cause an access to its address space to generate a Hard Fault."]
    #[inline(always)]
    pub fn bank4(&self) -> BANK4_R {
        BANK4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Remove power from SRAM Bank 3 which will cause an access to its address space to generate a Hard Fault."]
    #[inline(always)]
    pub fn bank3(&self) -> BANK3_R {
        BANK3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Remove power from SRAM Bank 2 which will cause an access to its address space to generate a Hard Fault."]
    #[inline(always)]
    pub fn bank2(&self) -> BANK2_R {
        BANK2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Remove power from SRAM Bank 1 which will cause an access to its address space to generate a Hard Fault."]
    #[inline(always)]
    pub fn bank1(&self) -> BANK1_R {
        BANK1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Remove power from SRAM Bank 0 which will cause an access to its address space to generate a Hard Fault."]
    #[inline(always)]
    pub fn bank0(&self) -> BANK0_R {
        BANK0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - Remove power from SRAM Bank 7 which will cause an access to its address space to generate a Hard Fault."]
    #[inline(always)]
    pub fn bank7(&mut self) -> BANK7_W {
        BANK7_W { w: self }
    }
    #[doc = "Bit 6 - Remove power from SRAM Bank 6 which will cause an access to its address space to generate a Hard Fault."]
    #[inline(always)]
    pub fn bank6(&mut self) -> BANK6_W {
        BANK6_W { w: self }
    }
    #[doc = "Bit 5 - Remove power from SRAM Bank 5 which will cause an access to its address space to generate a Hard Fault."]
    #[inline(always)]
    pub fn bank5(&mut self) -> BANK5_W {
        BANK5_W { w: self }
    }
    #[doc = "Bit 4 - Remove power from SRAM Bank 4 which will cause an access to its address space to generate a Hard Fault."]
    #[inline(always)]
    pub fn bank4(&mut self) -> BANK4_W {
        BANK4_W { w: self }
    }
    #[doc = "Bit 3 - Remove power from SRAM Bank 3 which will cause an access to its address space to generate a Hard Fault."]
    #[inline(always)]
    pub fn bank3(&mut self) -> BANK3_W {
        BANK3_W { w: self }
    }
    #[doc = "Bit 2 - Remove power from SRAM Bank 2 which will cause an access to its address space to generate a Hard Fault."]
    #[inline(always)]
    pub fn bank2(&mut self) -> BANK2_W {
        BANK2_W { w: self }
    }
    #[doc = "Bit 1 - Remove power from SRAM Bank 1 which will cause an access to its address space to generate a Hard Fault."]
    #[inline(always)]
    pub fn bank1(&mut self) -> BANK1_W {
        BANK1_W { w: self }
    }
    #[doc = "Bit 0 - Remove power from SRAM Bank 0 which will cause an access to its address space to generate a Hard Fault."]
    #[inline(always)]
    pub fn bank0(&mut self) -> BANK0_W {
        BANK0_W { w: self }
    }
}
