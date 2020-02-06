#[doc = "Reader of register BANDGAPEN"]
pub type R = crate::R<u32, super::BANDGAPEN>;
#[doc = "Writer for register BANDGAPEN"]
pub type W = crate::W<u32, super::BANDGAPEN>;
#[doc = "Register BANDGAPEN `reset()`'s with value 0"]
impl crate::ResetValue for super::BANDGAPEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Bandgap Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BGPEN_A {
    #[doc = "0: Bandgap disable."]
    DIS = 0,
    #[doc = "1: Bandgap enable."]
    EN = 1,
}
impl From<BGPEN_A> for bool {
    #[inline(always)]
    fn from(variant: BGPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BGPEN`"]
pub type BGPEN_R = crate::R<bool, BGPEN_A>;
impl BGPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BGPEN_A {
        match self.bits {
            false => BGPEN_A::DIS,
            true => BGPEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == BGPEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == BGPEN_A::EN
    }
}
#[doc = "Write proxy for field `BGPEN`"]
pub struct BGPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BGPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BGPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bandgap disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(BGPEN_A::DIS)
    }
    #[doc = "Bandgap enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(BGPEN_A::EN)
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
    #[doc = "Bit 0 - Bandgap Enable"]
    #[inline(always)]
    pub fn bgpen(&self) -> BGPEN_R {
        BGPEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Bandgap Enable"]
    #[inline(always)]
    pub fn bgpen(&mut self) -> BGPEN_W {
        BGPEN_W { w: self }
    }
}
