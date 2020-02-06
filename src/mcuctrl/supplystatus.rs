#[doc = "Reader of register SUPPLYSTATUS"]
pub type R = crate::R<u32, super::SUPPLYSTATUS>;
#[doc = "Writer for register SUPPLYSTATUS"]
pub type W = crate::W<u32, super::SUPPLYSTATUS>;
#[doc = "Register SUPPLYSTATUS `reset()`'s with value 0"]
impl crate::ResetValue for super::SUPPLYSTATUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Indicates whether the Core low-voltage domain is supplied from the LDO or the Buck.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COREBUCKON_A {
    #[doc = "0: Indicates the the LDO is supplying the Core low-voltage."]
    LDO = 0,
    #[doc = "1: Indicates the the Buck is supplying the Core low-voltage."]
    BUCK = 1,
}
impl From<COREBUCKON_A> for bool {
    #[inline(always)]
    fn from(variant: COREBUCKON_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `COREBUCKON`"]
pub type COREBUCKON_R = crate::R<bool, COREBUCKON_A>;
impl COREBUCKON_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COREBUCKON_A {
        match self.bits {
            false => COREBUCKON_A::LDO,
            true => COREBUCKON_A::BUCK,
        }
    }
    #[doc = "Checks if the value of the field is `LDO`"]
    #[inline(always)]
    pub fn is_ldo(&self) -> bool {
        *self == COREBUCKON_A::LDO
    }
    #[doc = "Checks if the value of the field is `BUCK`"]
    #[inline(always)]
    pub fn is_buck(&self) -> bool {
        *self == COREBUCKON_A::BUCK
    }
}
#[doc = "Write proxy for field `COREBUCKON`"]
pub struct COREBUCKON_W<'a> {
    w: &'a mut W,
}
impl<'a> COREBUCKON_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COREBUCKON_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Indicates the the LDO is supplying the Core low-voltage."]
    #[inline(always)]
    pub fn ldo(self) -> &'a mut W {
        self.variant(COREBUCKON_A::LDO)
    }
    #[doc = "Indicates the the Buck is supplying the Core low-voltage."]
    #[inline(always)]
    pub fn buck(self) -> &'a mut W {
        self.variant(COREBUCKON_A::BUCK)
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
#[doc = "Indicate whether the Memory power domain is supplied from the LDO or the Buck.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MEMBUCKON_A {
    #[doc = "0: Indicates the LDO is supplying the memory power domain."]
    LDO = 0,
    #[doc = "1: Indicates the Buck is supplying the memory power domain."]
    BUCK = 1,
}
impl From<MEMBUCKON_A> for bool {
    #[inline(always)]
    fn from(variant: MEMBUCKON_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MEMBUCKON`"]
pub type MEMBUCKON_R = crate::R<bool, MEMBUCKON_A>;
impl MEMBUCKON_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MEMBUCKON_A {
        match self.bits {
            false => MEMBUCKON_A::LDO,
            true => MEMBUCKON_A::BUCK,
        }
    }
    #[doc = "Checks if the value of the field is `LDO`"]
    #[inline(always)]
    pub fn is_ldo(&self) -> bool {
        *self == MEMBUCKON_A::LDO
    }
    #[doc = "Checks if the value of the field is `BUCK`"]
    #[inline(always)]
    pub fn is_buck(&self) -> bool {
        *self == MEMBUCKON_A::BUCK
    }
}
#[doc = "Write proxy for field `MEMBUCKON`"]
pub struct MEMBUCKON_W<'a> {
    w: &'a mut W,
}
impl<'a> MEMBUCKON_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MEMBUCKON_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Indicates the LDO is supplying the memory power domain."]
    #[inline(always)]
    pub fn ldo(self) -> &'a mut W {
        self.variant(MEMBUCKON_A::LDO)
    }
    #[doc = "Indicates the Buck is supplying the memory power domain."]
    #[inline(always)]
    pub fn buck(self) -> &'a mut W {
        self.variant(MEMBUCKON_A::BUCK)
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
    #[doc = "Bit 1 - Indicates whether the Core low-voltage domain is supplied from the LDO or the Buck."]
    #[inline(always)]
    pub fn corebuckon(&self) -> COREBUCKON_R {
        COREBUCKON_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Indicate whether the Memory power domain is supplied from the LDO or the Buck."]
    #[inline(always)]
    pub fn membuckon(&self) -> MEMBUCKON_R {
        MEMBUCKON_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Indicates whether the Core low-voltage domain is supplied from the LDO or the Buck."]
    #[inline(always)]
    pub fn corebuckon(&mut self) -> COREBUCKON_W {
        COREBUCKON_W { w: self }
    }
    #[doc = "Bit 0 - Indicate whether the Memory power domain is supplied from the LDO or the Buck."]
    #[inline(always)]
    pub fn membuckon(&mut self) -> MEMBUCKON_W {
        MEMBUCKON_W { w: self }
    }
}
