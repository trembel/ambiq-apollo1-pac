#[doc = "Reader of register CLKCFG"]
pub type R = crate::R<u32, super::CLKCFG>;
#[doc = "Writer for register CLKCFG"]
pub type W = crate::W<u32, super::CLKCFG>;
#[doc = "Register CLKCFG `reset()`'s with value 0"]
impl crate::ResetValue for super::CLKCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TOTPER`"]
pub type TOTPER_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TOTPER`"]
pub struct TOTPER_W<'a> {
    w: &'a mut W,
}
impl<'a> TOTPER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
#[doc = "Reader of field `LOWPER`"]
pub type LOWPER_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LOWPER`"]
pub struct LOWPER_W<'a> {
    w: &'a mut W,
}
impl<'a> LOWPER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Enable clock division by TOTPER.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIVEN_A {
    #[doc = "0: Disable TOTPER division."]
    DIS = 0,
    #[doc = "1: Enable TOTPER division."]
    EN = 1,
}
impl From<DIVEN_A> for bool {
    #[inline(always)]
    fn from(variant: DIVEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIVEN`"]
pub type DIVEN_R = crate::R<bool, DIVEN_A>;
impl DIVEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIVEN_A {
        match self.bits {
            false => DIVEN_A::DIS,
            true => DIVEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == DIVEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == DIVEN_A::EN
    }
}
#[doc = "Write proxy for field `DIVEN`"]
pub struct DIVEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIVEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable TOTPER division."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(DIVEN_A::DIS)
    }
    #[doc = "Enable TOTPER division."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(DIVEN_A::EN)
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
#[doc = "Enable divide by 3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIV3_A {
    #[doc = "0: Select divide by 1."]
    DIS = 0,
    #[doc = "1: Select divide by 3."]
    EN = 1,
}
impl From<DIV3_A> for bool {
    #[inline(always)]
    fn from(variant: DIV3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIV3`"]
pub type DIV3_R = crate::R<bool, DIV3_A>;
impl DIV3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIV3_A {
        match self.bits {
            false => DIV3_A::DIS,
            true => DIV3_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == DIV3_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == DIV3_A::EN
    }
}
#[doc = "Write proxy for field `DIV3`"]
pub struct DIV3_W<'a> {
    w: &'a mut W,
}
impl<'a> DIV3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIV3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Select divide by 1."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(DIV3_A::DIS)
    }
    #[doc = "Select divide by 3."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(DIV3_A::EN)
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
#[doc = "Select the input clock frequency.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FSEL_A {
    #[doc = "0: Selects the HFRC / 64 as the input clock."]
    HFRC_DIV64 = 0,
    #[doc = "1: Selects the HFRC as the input clock."]
    HFRC = 1,
    #[doc = "2: Selects the HFRC / 2 as the input clock."]
    HFRC_DIV2 = 2,
    #[doc = "3: Selects the HFRC / 4 as the input clock."]
    HFRC_DIV4 = 3,
    #[doc = "4: Selects the HFRC / 8 as the input clock."]
    HFRC_DIV8 = 4,
    #[doc = "5: Selects the HFRC / 16 as the input clock."]
    HFRC_DIV16 = 5,
    #[doc = "6: Selects the HFRC / 32 as the input clock."]
    HFRC_DIV32 = 6,
}
impl From<FSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: FSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FSEL`"]
pub type FSEL_R = crate::R<u8, FSEL_A>;
impl FSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, FSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(FSEL_A::HFRC_DIV64),
            1 => Val(FSEL_A::HFRC),
            2 => Val(FSEL_A::HFRC_DIV2),
            3 => Val(FSEL_A::HFRC_DIV4),
            4 => Val(FSEL_A::HFRC_DIV8),
            5 => Val(FSEL_A::HFRC_DIV16),
            6 => Val(FSEL_A::HFRC_DIV32),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV64`"]
    #[inline(always)]
    pub fn is_hfrc_div64(&self) -> bool {
        *self == FSEL_A::HFRC_DIV64
    }
    #[doc = "Checks if the value of the field is `HFRC`"]
    #[inline(always)]
    pub fn is_hfrc(&self) -> bool {
        *self == FSEL_A::HFRC
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV2`"]
    #[inline(always)]
    pub fn is_hfrc_div2(&self) -> bool {
        *self == FSEL_A::HFRC_DIV2
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV4`"]
    #[inline(always)]
    pub fn is_hfrc_div4(&self) -> bool {
        *self == FSEL_A::HFRC_DIV4
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV8`"]
    #[inline(always)]
    pub fn is_hfrc_div8(&self) -> bool {
        *self == FSEL_A::HFRC_DIV8
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV16`"]
    #[inline(always)]
    pub fn is_hfrc_div16(&self) -> bool {
        *self == FSEL_A::HFRC_DIV16
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV32`"]
    #[inline(always)]
    pub fn is_hfrc_div32(&self) -> bool {
        *self == FSEL_A::HFRC_DIV32
    }
}
#[doc = "Write proxy for field `FSEL`"]
pub struct FSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> FSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Selects the HFRC / 64 as the input clock."]
    #[inline(always)]
    pub fn hfrc_div64(self) -> &'a mut W {
        self.variant(FSEL_A::HFRC_DIV64)
    }
    #[doc = "Selects the HFRC as the input clock."]
    #[inline(always)]
    pub fn hfrc(self) -> &'a mut W {
        self.variant(FSEL_A::HFRC)
    }
    #[doc = "Selects the HFRC / 2 as the input clock."]
    #[inline(always)]
    pub fn hfrc_div2(self) -> &'a mut W {
        self.variant(FSEL_A::HFRC_DIV2)
    }
    #[doc = "Selects the HFRC / 4 as the input clock."]
    #[inline(always)]
    pub fn hfrc_div4(self) -> &'a mut W {
        self.variant(FSEL_A::HFRC_DIV4)
    }
    #[doc = "Selects the HFRC / 8 as the input clock."]
    #[inline(always)]
    pub fn hfrc_div8(self) -> &'a mut W {
        self.variant(FSEL_A::HFRC_DIV8)
    }
    #[doc = "Selects the HFRC / 16 as the input clock."]
    #[inline(always)]
    pub fn hfrc_div16(self) -> &'a mut W {
        self.variant(FSEL_A::HFRC_DIV16)
    }
    #[doc = "Selects the HFRC / 32 as the input clock."]
    #[inline(always)]
    pub fn hfrc_div32(self) -> &'a mut W {
        self.variant(FSEL_A::HFRC_DIV32)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31 - Clock total count minus 1."]
    #[inline(always)]
    pub fn totper(&self) -> TOTPER_R {
        TOTPER_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Clock low count minus 1."]
    #[inline(always)]
    pub fn lowper(&self) -> LOWPER_R {
        LOWPER_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 12 - Enable clock division by TOTPER."]
    #[inline(always)]
    pub fn diven(&self) -> DIVEN_R {
        DIVEN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Enable divide by 3."]
    #[inline(always)]
    pub fn div3(&self) -> DIV3_R {
        DIV3_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 8:10 - Select the input clock frequency."]
    #[inline(always)]
    pub fn fsel(&self) -> FSEL_R {
        FSEL_R::new(((self.bits >> 8) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31 - Clock total count minus 1."]
    #[inline(always)]
    pub fn totper(&mut self) -> TOTPER_W {
        TOTPER_W { w: self }
    }
    #[doc = "Bits 16:23 - Clock low count minus 1."]
    #[inline(always)]
    pub fn lowper(&mut self) -> LOWPER_W {
        LOWPER_W { w: self }
    }
    #[doc = "Bit 12 - Enable clock division by TOTPER."]
    #[inline(always)]
    pub fn diven(&mut self) -> DIVEN_W {
        DIVEN_W { w: self }
    }
    #[doc = "Bit 11 - Enable divide by 3."]
    #[inline(always)]
    pub fn div3(&mut self) -> DIV3_W {
        DIV3_W { w: self }
    }
    #[doc = "Bits 8:10 - Select the input clock frequency."]
    #[inline(always)]
    pub fn fsel(&mut self) -> FSEL_W {
        FSEL_W { w: self }
    }
}
