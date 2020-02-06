#[doc = "Reader of register RSR"]
pub type R = crate::R<u32, super::RSR>;
#[doc = "Writer for register RSR"]
pub type W = crate::W<u32, super::RSR>;
#[doc = "Register RSR `reset()`'s with value 0"]
impl crate::ResetValue for super::RSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "This is the overrun error indicator.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OESTAT_A {
    #[doc = "0: No error on UART OESTAT, overrun error indicator."]
    NOERR = 0,
    #[doc = "1: Error on UART OESTAT, overrun error indicator."]
    ERR = 1,
}
impl From<OESTAT_A> for bool {
    #[inline(always)]
    fn from(variant: OESTAT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OESTAT`"]
pub type OESTAT_R = crate::R<bool, OESTAT_A>;
impl OESTAT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OESTAT_A {
        match self.bits {
            false => OESTAT_A::NOERR,
            true => OESTAT_A::ERR,
        }
    }
    #[doc = "Checks if the value of the field is `NOERR`"]
    #[inline(always)]
    pub fn is_noerr(&self) -> bool {
        *self == OESTAT_A::NOERR
    }
    #[doc = "Checks if the value of the field is `ERR`"]
    #[inline(always)]
    pub fn is_err(&self) -> bool {
        *self == OESTAT_A::ERR
    }
}
#[doc = "Write proxy for field `OESTAT`"]
pub struct OESTAT_W<'a> {
    w: &'a mut W,
}
impl<'a> OESTAT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OESTAT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No error on UART OESTAT, overrun error indicator."]
    #[inline(always)]
    pub fn noerr(self) -> &'a mut W {
        self.variant(OESTAT_A::NOERR)
    }
    #[doc = "Error on UART OESTAT, overrun error indicator."]
    #[inline(always)]
    pub fn err(self) -> &'a mut W {
        self.variant(OESTAT_A::ERR)
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
#[doc = "This is the break error indicator.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BESTAT_A {
    #[doc = "0: No error on UART BESTAT, break error indicator."]
    NOERR = 0,
    #[doc = "1: Error on UART BESTAT, break error indicator."]
    ERR = 1,
}
impl From<BESTAT_A> for bool {
    #[inline(always)]
    fn from(variant: BESTAT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BESTAT`"]
pub type BESTAT_R = crate::R<bool, BESTAT_A>;
impl BESTAT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BESTAT_A {
        match self.bits {
            false => BESTAT_A::NOERR,
            true => BESTAT_A::ERR,
        }
    }
    #[doc = "Checks if the value of the field is `NOERR`"]
    #[inline(always)]
    pub fn is_noerr(&self) -> bool {
        *self == BESTAT_A::NOERR
    }
    #[doc = "Checks if the value of the field is `ERR`"]
    #[inline(always)]
    pub fn is_err(&self) -> bool {
        *self == BESTAT_A::ERR
    }
}
#[doc = "Write proxy for field `BESTAT`"]
pub struct BESTAT_W<'a> {
    w: &'a mut W,
}
impl<'a> BESTAT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BESTAT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No error on UART BESTAT, break error indicator."]
    #[inline(always)]
    pub fn noerr(self) -> &'a mut W {
        self.variant(BESTAT_A::NOERR)
    }
    #[doc = "Error on UART BESTAT, break error indicator."]
    #[inline(always)]
    pub fn err(self) -> &'a mut W {
        self.variant(BESTAT_A::ERR)
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
#[doc = "This is the parity error indicator.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PESTAT_A {
    #[doc = "0: No error on UART PESTAT, parity error indicator."]
    NOERR = 0,
    #[doc = "1: Error on UART PESTAT, parity error indicator."]
    ERR = 1,
}
impl From<PESTAT_A> for bool {
    #[inline(always)]
    fn from(variant: PESTAT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PESTAT`"]
pub type PESTAT_R = crate::R<bool, PESTAT_A>;
impl PESTAT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PESTAT_A {
        match self.bits {
            false => PESTAT_A::NOERR,
            true => PESTAT_A::ERR,
        }
    }
    #[doc = "Checks if the value of the field is `NOERR`"]
    #[inline(always)]
    pub fn is_noerr(&self) -> bool {
        *self == PESTAT_A::NOERR
    }
    #[doc = "Checks if the value of the field is `ERR`"]
    #[inline(always)]
    pub fn is_err(&self) -> bool {
        *self == PESTAT_A::ERR
    }
}
#[doc = "Write proxy for field `PESTAT`"]
pub struct PESTAT_W<'a> {
    w: &'a mut W,
}
impl<'a> PESTAT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PESTAT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No error on UART PESTAT, parity error indicator."]
    #[inline(always)]
    pub fn noerr(self) -> &'a mut W {
        self.variant(PESTAT_A::NOERR)
    }
    #[doc = "Error on UART PESTAT, parity error indicator."]
    #[inline(always)]
    pub fn err(self) -> &'a mut W {
        self.variant(PESTAT_A::ERR)
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
#[doc = "This is the framing error indicator.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FESTAT_A {
    #[doc = "0: No error on UART FESTAT, framing error indicator."]
    NOERR = 0,
    #[doc = "1: Error on UART FESTAT, framing error indicator."]
    ERR = 1,
}
impl From<FESTAT_A> for bool {
    #[inline(always)]
    fn from(variant: FESTAT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FESTAT`"]
pub type FESTAT_R = crate::R<bool, FESTAT_A>;
impl FESTAT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FESTAT_A {
        match self.bits {
            false => FESTAT_A::NOERR,
            true => FESTAT_A::ERR,
        }
    }
    #[doc = "Checks if the value of the field is `NOERR`"]
    #[inline(always)]
    pub fn is_noerr(&self) -> bool {
        *self == FESTAT_A::NOERR
    }
    #[doc = "Checks if the value of the field is `ERR`"]
    #[inline(always)]
    pub fn is_err(&self) -> bool {
        *self == FESTAT_A::ERR
    }
}
#[doc = "Write proxy for field `FESTAT`"]
pub struct FESTAT_W<'a> {
    w: &'a mut W,
}
impl<'a> FESTAT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FESTAT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No error on UART FESTAT, framing error indicator."]
    #[inline(always)]
    pub fn noerr(self) -> &'a mut W {
        self.variant(FESTAT_A::NOERR)
    }
    #[doc = "Error on UART FESTAT, framing error indicator."]
    #[inline(always)]
    pub fn err(self) -> &'a mut W {
        self.variant(FESTAT_A::ERR)
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
    #[doc = "Bit 3 - This is the overrun error indicator."]
    #[inline(always)]
    pub fn oestat(&self) -> OESTAT_R {
        OESTAT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - This is the break error indicator."]
    #[inline(always)]
    pub fn bestat(&self) -> BESTAT_R {
        BESTAT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - This is the parity error indicator."]
    #[inline(always)]
    pub fn pestat(&self) -> PESTAT_R {
        PESTAT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - This is the framing error indicator."]
    #[inline(always)]
    pub fn festat(&self) -> FESTAT_R {
        FESTAT_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - This is the overrun error indicator."]
    #[inline(always)]
    pub fn oestat(&mut self) -> OESTAT_W {
        OESTAT_W { w: self }
    }
    #[doc = "Bit 2 - This is the break error indicator."]
    #[inline(always)]
    pub fn bestat(&mut self) -> BESTAT_W {
        BESTAT_W { w: self }
    }
    #[doc = "Bit 1 - This is the parity error indicator."]
    #[inline(always)]
    pub fn pestat(&mut self) -> PESTAT_W {
        PESTAT_W { w: self }
    }
    #[doc = "Bit 0 - This is the framing error indicator."]
    #[inline(always)]
    pub fn festat(&mut self) -> FESTAT_W {
        FESTAT_W { w: self }
    }
}
