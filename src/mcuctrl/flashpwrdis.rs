#[doc = "Reader of register FLASHPWRDIS"]
pub type R = crate::R<u32, super::FLASHPWRDIS>;
#[doc = "Writer for register FLASHPWRDIS"]
pub type W = crate::W<u32, super::FLASHPWRDIS>;
#[doc = "Register FLASHPWRDIS `reset()`'s with value 0"]
impl crate::ResetValue for super::FLASHPWRDIS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Remove power from Flash Bank 1 which will cause an access to its address space to generate a Hard Fault.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BANK1_A {
    #[doc = "1: Disable Flash instance 1."]
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
    #[doc = "Disable Flash instance 1."]
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
#[doc = "Remove power from Flash Bank 0 which will cause an access to its address space to generate a Hard Fault.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BANK0_A {
    #[doc = "1: Disable Flash instance 0."]
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
    #[doc = "Disable Flash instance 0."]
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
    #[doc = "Bit 1 - Remove power from Flash Bank 1 which will cause an access to its address space to generate a Hard Fault."]
    #[inline(always)]
    pub fn bank1(&self) -> BANK1_R {
        BANK1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Remove power from Flash Bank 0 which will cause an access to its address space to generate a Hard Fault."]
    #[inline(always)]
    pub fn bank0(&self) -> BANK0_R {
        BANK0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Remove power from Flash Bank 1 which will cause an access to its address space to generate a Hard Fault."]
    #[inline(always)]
    pub fn bank1(&mut self) -> BANK1_W {
        BANK1_W { w: self }
    }
    #[doc = "Bit 0 - Remove power from Flash Bank 0 which will cause an access to its address space to generate a Hard Fault."]
    #[inline(always)]
    pub fn bank0(&mut self) -> BANK0_W {
        BANK0_W { w: self }
    }
}
