#[doc = "Reader of register HFADJ"]
pub type R = crate::R<u32, super::HFADJ>;
#[doc = "Writer for register HFADJ"]
pub type W = crate::W<u32, super::HFADJ>;
#[doc = "Register HFADJ `reset()`'s with value 0"]
impl crate::ResetValue for super::HFADJ {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "XT warmup period for HFRC adjustment\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HFWARMUP_A {
    #[doc = "0: Autoadjust XT warmup period = 1-2 seconds"]
    _1SEC = 0,
    #[doc = "1: Autoadjust XT warmup period = 2-4 seconds"]
    _2SEC = 1,
}
impl From<HFWARMUP_A> for bool {
    #[inline(always)]
    fn from(variant: HFWARMUP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HFWARMUP`"]
pub type HFWARMUP_R = crate::R<bool, HFWARMUP_A>;
impl HFWARMUP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HFWARMUP_A {
        match self.bits {
            false => HFWARMUP_A::_1SEC,
            true => HFWARMUP_A::_2SEC,
        }
    }
    #[doc = "Checks if the value of the field is `_1SEC`"]
    #[inline(always)]
    pub fn is_1sec(&self) -> bool {
        *self == HFWARMUP_A::_1SEC
    }
    #[doc = "Checks if the value of the field is `_2SEC`"]
    #[inline(always)]
    pub fn is_2sec(&self) -> bool {
        *self == HFWARMUP_A::_2SEC
    }
}
#[doc = "Write proxy for field `HFWARMUP`"]
pub struct HFWARMUP_W<'a> {
    w: &'a mut W,
}
impl<'a> HFWARMUP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HFWARMUP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Autoadjust XT warmup period = 1-2 seconds"]
    #[inline(always)]
    pub fn _1sec(self) -> &'a mut W {
        self.variant(HFWARMUP_A::_1SEC)
    }
    #[doc = "Autoadjust XT warmup period = 2-4 seconds"]
    #[inline(always)]
    pub fn _2sec(self) -> &'a mut W {
        self.variant(HFWARMUP_A::_2SEC)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `HFXTADJ`"]
pub type HFXTADJ_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `HFXTADJ`"]
pub struct HFXTADJ_W<'a> {
    w: &'a mut W,
}
impl<'a> HFXTADJ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff << 8)) | (((value as u32) & 0x07ff) << 8);
        self.w
    }
}
#[doc = "Repeat period for HFRC adjustment\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum HFADJCK_A {
    #[doc = "0: Autoadjust repeat period = 4 seconds"]
    _4SEC = 0,
    #[doc = "1: Autoadjust repeat period = 16 seconds"]
    _16SEC = 1,
    #[doc = "2: Autoadjust repeat period = 32 seconds"]
    _32SEC = 2,
    #[doc = "3: Autoadjust repeat period = 64 seconds"]
    _64SEC = 3,
    #[doc = "4: Autoadjust repeat period = 128 seconds"]
    _128SEC = 4,
    #[doc = "5: Autoadjust repeat period = 256 seconds"]
    _256SEC = 5,
    #[doc = "6: Autoadjust repeat period = 512 seconds"]
    _512SEC = 6,
    #[doc = "7: Autoadjust repeat period = 1024 seconds"]
    _1024SEC = 7,
}
impl From<HFADJCK_A> for u8 {
    #[inline(always)]
    fn from(variant: HFADJCK_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `HFADJCK`"]
pub type HFADJCK_R = crate::R<u8, HFADJCK_A>;
impl HFADJCK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HFADJCK_A {
        match self.bits {
            0 => HFADJCK_A::_4SEC,
            1 => HFADJCK_A::_16SEC,
            2 => HFADJCK_A::_32SEC,
            3 => HFADJCK_A::_64SEC,
            4 => HFADJCK_A::_128SEC,
            5 => HFADJCK_A::_256SEC,
            6 => HFADJCK_A::_512SEC,
            7 => HFADJCK_A::_1024SEC,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_4SEC`"]
    #[inline(always)]
    pub fn is_4sec(&self) -> bool {
        *self == HFADJCK_A::_4SEC
    }
    #[doc = "Checks if the value of the field is `_16SEC`"]
    #[inline(always)]
    pub fn is_16sec(&self) -> bool {
        *self == HFADJCK_A::_16SEC
    }
    #[doc = "Checks if the value of the field is `_32SEC`"]
    #[inline(always)]
    pub fn is_32sec(&self) -> bool {
        *self == HFADJCK_A::_32SEC
    }
    #[doc = "Checks if the value of the field is `_64SEC`"]
    #[inline(always)]
    pub fn is_64sec(&self) -> bool {
        *self == HFADJCK_A::_64SEC
    }
    #[doc = "Checks if the value of the field is `_128SEC`"]
    #[inline(always)]
    pub fn is_128sec(&self) -> bool {
        *self == HFADJCK_A::_128SEC
    }
    #[doc = "Checks if the value of the field is `_256SEC`"]
    #[inline(always)]
    pub fn is_256sec(&self) -> bool {
        *self == HFADJCK_A::_256SEC
    }
    #[doc = "Checks if the value of the field is `_512SEC`"]
    #[inline(always)]
    pub fn is_512sec(&self) -> bool {
        *self == HFADJCK_A::_512SEC
    }
    #[doc = "Checks if the value of the field is `_1024SEC`"]
    #[inline(always)]
    pub fn is_1024sec(&self) -> bool {
        *self == HFADJCK_A::_1024SEC
    }
}
#[doc = "Write proxy for field `HFADJCK`"]
pub struct HFADJCK_W<'a> {
    w: &'a mut W,
}
impl<'a> HFADJCK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HFADJCK_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Autoadjust repeat period = 4 seconds"]
    #[inline(always)]
    pub fn _4sec(self) -> &'a mut W {
        self.variant(HFADJCK_A::_4SEC)
    }
    #[doc = "Autoadjust repeat period = 16 seconds"]
    #[inline(always)]
    pub fn _16sec(self) -> &'a mut W {
        self.variant(HFADJCK_A::_16SEC)
    }
    #[doc = "Autoadjust repeat period = 32 seconds"]
    #[inline(always)]
    pub fn _32sec(self) -> &'a mut W {
        self.variant(HFADJCK_A::_32SEC)
    }
    #[doc = "Autoadjust repeat period = 64 seconds"]
    #[inline(always)]
    pub fn _64sec(self) -> &'a mut W {
        self.variant(HFADJCK_A::_64SEC)
    }
    #[doc = "Autoadjust repeat period = 128 seconds"]
    #[inline(always)]
    pub fn _128sec(self) -> &'a mut W {
        self.variant(HFADJCK_A::_128SEC)
    }
    #[doc = "Autoadjust repeat period = 256 seconds"]
    #[inline(always)]
    pub fn _256sec(self) -> &'a mut W {
        self.variant(HFADJCK_A::_256SEC)
    }
    #[doc = "Autoadjust repeat period = 512 seconds"]
    #[inline(always)]
    pub fn _512sec(self) -> &'a mut W {
        self.variant(HFADJCK_A::_512SEC)
    }
    #[doc = "Autoadjust repeat period = 1024 seconds"]
    #[inline(always)]
    pub fn _1024sec(self) -> &'a mut W {
        self.variant(HFADJCK_A::_1024SEC)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 1)) | (((value as u32) & 0x07) << 1);
        self.w
    }
}
#[doc = "HFRC adjustment control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HFADJEN_A {
    #[doc = "0: Disable the HFRC adjustment"]
    DIS = 0,
    #[doc = "1: Enable the HFRC adjustment"]
    EN = 1,
}
impl From<HFADJEN_A> for bool {
    #[inline(always)]
    fn from(variant: HFADJEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HFADJEN`"]
pub type HFADJEN_R = crate::R<bool, HFADJEN_A>;
impl HFADJEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HFADJEN_A {
        match self.bits {
            false => HFADJEN_A::DIS,
            true => HFADJEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == HFADJEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == HFADJEN_A::EN
    }
}
#[doc = "Write proxy for field `HFADJEN`"]
pub struct HFADJEN_W<'a> {
    w: &'a mut W,
}
impl<'a> HFADJEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HFADJEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable the HFRC adjustment"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(HFADJEN_A::DIS)
    }
    #[doc = "Enable the HFRC adjustment"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(HFADJEN_A::EN)
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
    #[doc = "Bit 19 - XT warmup period for HFRC adjustment"]
    #[inline(always)]
    pub fn hfwarmup(&self) -> HFWARMUP_R {
        HFWARMUP_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bits 8:18 - Target HFRC adjustment value."]
    #[inline(always)]
    pub fn hfxtadj(&self) -> HFXTADJ_R {
        HFXTADJ_R::new(((self.bits >> 8) & 0x07ff) as u16)
    }
    #[doc = "Bits 1:3 - Repeat period for HFRC adjustment"]
    #[inline(always)]
    pub fn hfadjck(&self) -> HFADJCK_R {
        HFADJCK_R::new(((self.bits >> 1) & 0x07) as u8)
    }
    #[doc = "Bit 0 - HFRC adjustment control"]
    #[inline(always)]
    pub fn hfadjen(&self) -> HFADJEN_R {
        HFADJEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 19 - XT warmup period for HFRC adjustment"]
    #[inline(always)]
    pub fn hfwarmup(&mut self) -> HFWARMUP_W {
        HFWARMUP_W { w: self }
    }
    #[doc = "Bits 8:18 - Target HFRC adjustment value."]
    #[inline(always)]
    pub fn hfxtadj(&mut self) -> HFXTADJ_W {
        HFXTADJ_W { w: self }
    }
    #[doc = "Bits 1:3 - Repeat period for HFRC adjustment"]
    #[inline(always)]
    pub fn hfadjck(&mut self) -> HFADJCK_W {
        HFADJCK_W { w: self }
    }
    #[doc = "Bit 0 - HFRC adjustment control"]
    #[inline(always)]
    pub fn hfadjen(&mut self) -> HFADJEN_W {
        HFADJEN_W { w: self }
    }
}
