#[doc = "Reader of register FAULTSTATUS"]
pub type R = crate::R<u32, super::FAULTSTATUS>;
#[doc = "Writer for register FAULTSTATUS"]
pub type W = crate::W<u32, super::FAULTSTATUS>;
#[doc = "Register FAULTSTATUS `reset()`'s with value 0"]
impl crate::ResetValue for super::FAULTSTATUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "SYS Bus Decoder Fault Detected bit. When set, a fault has been detected, and the SYSFAULTADDR register will contain the bus address which generated the fault.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYS_A {
    #[doc = "0: No bus fault has been detected."]
    NOFAULT = 0,
    #[doc = "1: Bus fault detected."]
    FAULT = 1,
}
impl From<SYS_A> for bool {
    #[inline(always)]
    fn from(variant: SYS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SYS`"]
pub type SYS_R = crate::R<bool, SYS_A>;
impl SYS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYS_A {
        match self.bits {
            false => SYS_A::NOFAULT,
            true => SYS_A::FAULT,
        }
    }
    #[doc = "Checks if the value of the field is `NOFAULT`"]
    #[inline(always)]
    pub fn is_nofault(&self) -> bool {
        *self == SYS_A::NOFAULT
    }
    #[doc = "Checks if the value of the field is `FAULT`"]
    #[inline(always)]
    pub fn is_fault(&self) -> bool {
        *self == SYS_A::FAULT
    }
}
#[doc = "Write proxy for field `SYS`"]
pub struct SYS_W<'a> {
    w: &'a mut W,
}
impl<'a> SYS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No bus fault has been detected."]
    #[inline(always)]
    pub fn nofault(self) -> &'a mut W {
        self.variant(SYS_A::NOFAULT)
    }
    #[doc = "Bus fault detected."]
    #[inline(always)]
    pub fn fault(self) -> &'a mut W {
        self.variant(SYS_A::FAULT)
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
#[doc = "DCODE Bus Decoder Fault Detected bit. When set, a fault has been detected, and the DCODEFAULTADDR register will contain the bus address which generated the fault.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCODE_A {
    #[doc = "0: No DCODE fault has been detected."]
    NOFAULT = 0,
    #[doc = "1: DCODE fault detected."]
    FAULT = 1,
}
impl From<DCODE_A> for bool {
    #[inline(always)]
    fn from(variant: DCODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DCODE`"]
pub type DCODE_R = crate::R<bool, DCODE_A>;
impl DCODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DCODE_A {
        match self.bits {
            false => DCODE_A::NOFAULT,
            true => DCODE_A::FAULT,
        }
    }
    #[doc = "Checks if the value of the field is `NOFAULT`"]
    #[inline(always)]
    pub fn is_nofault(&self) -> bool {
        *self == DCODE_A::NOFAULT
    }
    #[doc = "Checks if the value of the field is `FAULT`"]
    #[inline(always)]
    pub fn is_fault(&self) -> bool {
        *self == DCODE_A::FAULT
    }
}
#[doc = "Write proxy for field `DCODE`"]
pub struct DCODE_W<'a> {
    w: &'a mut W,
}
impl<'a> DCODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DCODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No DCODE fault has been detected."]
    #[inline(always)]
    pub fn nofault(self) -> &'a mut W {
        self.variant(DCODE_A::NOFAULT)
    }
    #[doc = "DCODE fault detected."]
    #[inline(always)]
    pub fn fault(self) -> &'a mut W {
        self.variant(DCODE_A::FAULT)
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
#[doc = "The ICODE Bus Decoder Fault Detected bit. When set, a fault has been detected, and the ICODEFAULTADDR register will contain the bus address which generated the fault.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICODE_A {
    #[doc = "0: No ICODE fault has been detected."]
    NOFAULT = 0,
    #[doc = "1: ICODE fault detected."]
    FAULT = 1,
}
impl From<ICODE_A> for bool {
    #[inline(always)]
    fn from(variant: ICODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ICODE`"]
pub type ICODE_R = crate::R<bool, ICODE_A>;
impl ICODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICODE_A {
        match self.bits {
            false => ICODE_A::NOFAULT,
            true => ICODE_A::FAULT,
        }
    }
    #[doc = "Checks if the value of the field is `NOFAULT`"]
    #[inline(always)]
    pub fn is_nofault(&self) -> bool {
        *self == ICODE_A::NOFAULT
    }
    #[doc = "Checks if the value of the field is `FAULT`"]
    #[inline(always)]
    pub fn is_fault(&self) -> bool {
        *self == ICODE_A::FAULT
    }
}
#[doc = "Write proxy for field `ICODE`"]
pub struct ICODE_W<'a> {
    w: &'a mut W,
}
impl<'a> ICODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ICODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No ICODE fault has been detected."]
    #[inline(always)]
    pub fn nofault(self) -> &'a mut W {
        self.variant(ICODE_A::NOFAULT)
    }
    #[doc = "ICODE fault detected."]
    #[inline(always)]
    pub fn fault(self) -> &'a mut W {
        self.variant(ICODE_A::FAULT)
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
    #[doc = "Bit 2 - SYS Bus Decoder Fault Detected bit. When set, a fault has been detected, and the SYSFAULTADDR register will contain the bus address which generated the fault."]
    #[inline(always)]
    pub fn sys(&self) -> SYS_R {
        SYS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - DCODE Bus Decoder Fault Detected bit. When set, a fault has been detected, and the DCODEFAULTADDR register will contain the bus address which generated the fault."]
    #[inline(always)]
    pub fn dcode(&self) -> DCODE_R {
        DCODE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - The ICODE Bus Decoder Fault Detected bit. When set, a fault has been detected, and the ICODEFAULTADDR register will contain the bus address which generated the fault."]
    #[inline(always)]
    pub fn icode(&self) -> ICODE_R {
        ICODE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - SYS Bus Decoder Fault Detected bit. When set, a fault has been detected, and the SYSFAULTADDR register will contain the bus address which generated the fault."]
    #[inline(always)]
    pub fn sys(&mut self) -> SYS_W {
        SYS_W { w: self }
    }
    #[doc = "Bit 1 - DCODE Bus Decoder Fault Detected bit. When set, a fault has been detected, and the DCODEFAULTADDR register will contain the bus address which generated the fault."]
    #[inline(always)]
    pub fn dcode(&mut self) -> DCODE_W {
        DCODE_W { w: self }
    }
    #[doc = "Bit 0 - The ICODE Bus Decoder Fault Detected bit. When set, a fault has been detected, and the ICODEFAULTADDR register will contain the bus address which generated the fault."]
    #[inline(always)]
    pub fn icode(&mut self) -> ICODE_W {
        ICODE_W { w: self }
    }
}
