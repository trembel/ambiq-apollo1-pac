#[doc = "Reader of register UARTEN"]
pub type R = crate::R<u32, super::UARTEN>;
#[doc = "Writer for register UARTEN"]
pub type W = crate::W<u32, super::UARTEN>;
#[doc = "Register UARTEN `reset()`'s with value 0"]
impl crate::ResetValue for super::UARTEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "UART system clock control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UARTEN_A {
    #[doc = "0: Disable the UART system clock"]
    DIS = 0,
    #[doc = "1: Enable the UART system clock"]
    EN = 1,
}
impl From<UARTEN_A> for bool {
    #[inline(always)]
    fn from(variant: UARTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `UARTEN`"]
pub type UARTEN_R = crate::R<bool, UARTEN_A>;
impl UARTEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UARTEN_A {
        match self.bits {
            false => UARTEN_A::DIS,
            true => UARTEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == UARTEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == UARTEN_A::EN
    }
}
#[doc = "Write proxy for field `UARTEN`"]
pub struct UARTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> UARTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UARTEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable the UART system clock"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(UARTEN_A::DIS)
    }
    #[doc = "Enable the UART system clock"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(UARTEN_A::EN)
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
    #[doc = "Bit 0 - UART system clock control"]
    #[inline(always)]
    pub fn uarten(&self) -> UARTEN_R {
        UARTEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - UART system clock control"]
    #[inline(always)]
    pub fn uarten(&mut self) -> UARTEN_W {
        UARTEN_W { w: self }
    }
}
