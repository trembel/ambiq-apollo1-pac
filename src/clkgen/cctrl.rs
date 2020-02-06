#[doc = "Reader of register CCTRL"]
pub type R = crate::R<u32, super::CCTRL>;
#[doc = "Writer for register CCTRL"]
pub type W = crate::W<u32, super::CCTRL>;
#[doc = "Register CCTRL `reset()`'s with value 0x07"]
impl crate::ResetValue for super::CCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x07
    }
}
#[doc = "Flash Clock divisor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MEMSEL_A {
    #[doc = "0: Flash Clock is HFRC / 25"]
    HFRC_DIV25 = 0,
    #[doc = "1: Flash Clock is HFRC / 45"]
    HFRC_DIV45 = 1,
}
impl From<MEMSEL_A> for bool {
    #[inline(always)]
    fn from(variant: MEMSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MEMSEL`"]
pub type MEMSEL_R = crate::R<bool, MEMSEL_A>;
impl MEMSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MEMSEL_A {
        match self.bits {
            false => MEMSEL_A::HFRC_DIV25,
            true => MEMSEL_A::HFRC_DIV45,
        }
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV25`"]
    #[inline(always)]
    pub fn is_hfrc_div25(&self) -> bool {
        *self == MEMSEL_A::HFRC_DIV25
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV45`"]
    #[inline(always)]
    pub fn is_hfrc_div45(&self) -> bool {
        *self == MEMSEL_A::HFRC_DIV45
    }
}
#[doc = "Write proxy for field `MEMSEL`"]
pub struct MEMSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> MEMSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MEMSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Flash Clock is HFRC / 25"]
    #[inline(always)]
    pub fn hfrc_div25(self) -> &'a mut W {
        self.variant(MEMSEL_A::HFRC_DIV25)
    }
    #[doc = "Flash Clock is HFRC / 45"]
    #[inline(always)]
    pub fn hfrc_div45(self) -> &'a mut W {
        self.variant(MEMSEL_A::HFRC_DIV45)
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
#[doc = "Core Clock divisor\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CORESEL_A {
    #[doc = "0: Core Clock is HFRC"]
    HFRC = 0,
    #[doc = "1: Core Clock is HFRC / 2"]
    HFRC_DIV2 = 1,
    #[doc = "2: Core Clock is HFRC / 3"]
    HFRC_DIV3 = 2,
    #[doc = "3: Core Clock is HFRC / 4"]
    HFRC_DIV4 = 3,
    #[doc = "4: Core Clock is HFRC / 5"]
    HFRC_DIV5 = 4,
    #[doc = "5: Core Clock is HFRC / 6"]
    HFRC_DIV6 = 5,
    #[doc = "6: Core Clock is HFRC / 7"]
    HFRC_DIV7 = 6,
    #[doc = "7: Core Clock is HFRC / 8"]
    HFRC_DIV8 = 7,
}
impl From<CORESEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CORESEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CORESEL`"]
pub type CORESEL_R = crate::R<u8, CORESEL_A>;
impl CORESEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CORESEL_A {
        match self.bits {
            0 => CORESEL_A::HFRC,
            1 => CORESEL_A::HFRC_DIV2,
            2 => CORESEL_A::HFRC_DIV3,
            3 => CORESEL_A::HFRC_DIV4,
            4 => CORESEL_A::HFRC_DIV5,
            5 => CORESEL_A::HFRC_DIV6,
            6 => CORESEL_A::HFRC_DIV7,
            7 => CORESEL_A::HFRC_DIV8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `HFRC`"]
    #[inline(always)]
    pub fn is_hfrc(&self) -> bool {
        *self == CORESEL_A::HFRC
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV2`"]
    #[inline(always)]
    pub fn is_hfrc_div2(&self) -> bool {
        *self == CORESEL_A::HFRC_DIV2
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV3`"]
    #[inline(always)]
    pub fn is_hfrc_div3(&self) -> bool {
        *self == CORESEL_A::HFRC_DIV3
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV4`"]
    #[inline(always)]
    pub fn is_hfrc_div4(&self) -> bool {
        *self == CORESEL_A::HFRC_DIV4
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV5`"]
    #[inline(always)]
    pub fn is_hfrc_div5(&self) -> bool {
        *self == CORESEL_A::HFRC_DIV5
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV6`"]
    #[inline(always)]
    pub fn is_hfrc_div6(&self) -> bool {
        *self == CORESEL_A::HFRC_DIV6
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV7`"]
    #[inline(always)]
    pub fn is_hfrc_div7(&self) -> bool {
        *self == CORESEL_A::HFRC_DIV7
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV8`"]
    #[inline(always)]
    pub fn is_hfrc_div8(&self) -> bool {
        *self == CORESEL_A::HFRC_DIV8
    }
}
#[doc = "Write proxy for field `CORESEL`"]
pub struct CORESEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CORESEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CORESEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Core Clock is HFRC"]
    #[inline(always)]
    pub fn hfrc(self) -> &'a mut W {
        self.variant(CORESEL_A::HFRC)
    }
    #[doc = "Core Clock is HFRC / 2"]
    #[inline(always)]
    pub fn hfrc_div2(self) -> &'a mut W {
        self.variant(CORESEL_A::HFRC_DIV2)
    }
    #[doc = "Core Clock is HFRC / 3"]
    #[inline(always)]
    pub fn hfrc_div3(self) -> &'a mut W {
        self.variant(CORESEL_A::HFRC_DIV3)
    }
    #[doc = "Core Clock is HFRC / 4"]
    #[inline(always)]
    pub fn hfrc_div4(self) -> &'a mut W {
        self.variant(CORESEL_A::HFRC_DIV4)
    }
    #[doc = "Core Clock is HFRC / 5"]
    #[inline(always)]
    pub fn hfrc_div5(self) -> &'a mut W {
        self.variant(CORESEL_A::HFRC_DIV5)
    }
    #[doc = "Core Clock is HFRC / 6"]
    #[inline(always)]
    pub fn hfrc_div6(self) -> &'a mut W {
        self.variant(CORESEL_A::HFRC_DIV6)
    }
    #[doc = "Core Clock is HFRC / 7"]
    #[inline(always)]
    pub fn hfrc_div7(self) -> &'a mut W {
        self.variant(CORESEL_A::HFRC_DIV7)
    }
    #[doc = "Core Clock is HFRC / 8"]
    #[inline(always)]
    pub fn hfrc_div8(self) -> &'a mut W {
        self.variant(CORESEL_A::HFRC_DIV8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bit 3 - Flash Clock divisor"]
    #[inline(always)]
    pub fn memsel(&self) -> MEMSEL_R {
        MEMSEL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 0:2 - Core Clock divisor"]
    #[inline(always)]
    pub fn coresel(&self) -> CORESEL_R {
        CORESEL_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 3 - Flash Clock divisor"]
    #[inline(always)]
    pub fn memsel(&mut self) -> MEMSEL_W {
        MEMSEL_W { w: self }
    }
    #[doc = "Bits 0:2 - Core Clock divisor"]
    #[inline(always)]
    pub fn coresel(&mut self) -> CORESEL_W {
        CORESEL_W { w: self }
    }
}
