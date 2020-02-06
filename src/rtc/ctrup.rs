#[doc = "Reader of register CTRUP"]
pub type R = crate::R<u32, super::CTRUP>;
#[doc = "Writer for register CTRUP"]
pub type W = crate::W<u32, super::CTRUP>;
#[doc = "Register CTRUP `reset()`'s with value 0"]
impl crate::ResetValue for super::CTRUP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Counter read error status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTERR_A {
    #[doc = "0: No read error occurred"]
    NOERR = 0,
    #[doc = "1: Read error occurred"]
    RDERR = 1,
}
impl From<CTERR_A> for bool {
    #[inline(always)]
    fn from(variant: CTERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CTERR`"]
pub type CTERR_R = crate::R<bool, CTERR_A>;
impl CTERR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTERR_A {
        match self.bits {
            false => CTERR_A::NOERR,
            true => CTERR_A::RDERR,
        }
    }
    #[doc = "Checks if the value of the field is `NOERR`"]
    #[inline(always)]
    pub fn is_noerr(&self) -> bool {
        *self == CTERR_A::NOERR
    }
    #[doc = "Checks if the value of the field is `RDERR`"]
    #[inline(always)]
    pub fn is_rderr(&self) -> bool {
        *self == CTERR_A::RDERR
    }
}
#[doc = "Write proxy for field `CTERR`"]
pub struct CTERR_W<'a> {
    w: &'a mut W,
}
impl<'a> CTERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTERR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No read error occurred"]
    #[inline(always)]
    pub fn noerr(self) -> &'a mut W {
        self.variant(CTERR_A::NOERR)
    }
    #[doc = "Read error occurred"]
    #[inline(always)]
    pub fn rderr(self) -> &'a mut W {
        self.variant(CTERR_A::RDERR)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
#[doc = "Century enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CEB_A {
    #[doc = "0: Disable the Century bit from changing"]
    DIS = 0,
    #[doc = "1: Enable the Century bit to change"]
    EN = 1,
}
impl From<CEB_A> for bool {
    #[inline(always)]
    fn from(variant: CEB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CEB`"]
pub type CEB_R = crate::R<bool, CEB_A>;
impl CEB_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CEB_A {
        match self.bits {
            false => CEB_A::DIS,
            true => CEB_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == CEB_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == CEB_A::EN
    }
}
#[doc = "Write proxy for field `CEB`"]
pub struct CEB_W<'a> {
    w: &'a mut W,
}
impl<'a> CEB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CEB_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable the Century bit from changing"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(CEB_A::DIS)
    }
    #[doc = "Enable the Century bit to change"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(CEB_A::EN)
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Century\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CB_A {
    #[doc = "0: Century is 2000s"]
    _2000 = 0,
    #[doc = "1: Century is 1900s/2100s"]
    _1900_2100 = 1,
}
impl From<CB_A> for bool {
    #[inline(always)]
    fn from(variant: CB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CB`"]
pub type CB_R = crate::R<bool, CB_A>;
impl CB_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CB_A {
        match self.bits {
            false => CB_A::_2000,
            true => CB_A::_1900_2100,
        }
    }
    #[doc = "Checks if the value of the field is `_2000`"]
    #[inline(always)]
    pub fn is_2000(&self) -> bool {
        *self == CB_A::_2000
    }
    #[doc = "Checks if the value of the field is `_1900_2100`"]
    #[inline(always)]
    pub fn is_1900_2100(&self) -> bool {
        *self == CB_A::_1900_2100
    }
}
#[doc = "Write proxy for field `CB`"]
pub struct CB_W<'a> {
    w: &'a mut W,
}
impl<'a> CB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CB_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Century is 2000s"]
    #[inline(always)]
    pub fn _2000(self) -> &'a mut W {
        self.variant(CB_A::_2000)
    }
    #[doc = "Century is 1900s/2100s"]
    #[inline(always)]
    pub fn _1900_2100(self) -> &'a mut W {
        self.variant(CB_A::_1900_2100)
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Reader of field `CTRWKDY`"]
pub type CTRWKDY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CTRWKDY`"]
pub struct CTRWKDY_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRWKDY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | (((value as u32) & 0x07) << 24);
        self.w
    }
}
#[doc = "Reader of field `CTRYR`"]
pub type CTRYR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CTRYR`"]
pub struct CTRYR_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRYR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `CTRMO`"]
pub type CTRMO_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CTRMO`"]
pub struct CTRMO_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRMO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
#[doc = "Reader of field `CTRDATE`"]
pub type CTRDATE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CTRDATE`"]
pub struct CTRDATE_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRDATE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - Counter read error status"]
    #[inline(always)]
    pub fn cterr(&self) -> CTERR_R {
        CTERR_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Century enable"]
    #[inline(always)]
    pub fn ceb(&self) -> CEB_R {
        CEB_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Century"]
    #[inline(always)]
    pub fn cb(&self) -> CB_R {
        CB_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bits 24:26 - Weekdays Counter"]
    #[inline(always)]
    pub fn ctrwkdy(&self) -> CTRWKDY_R {
        CTRWKDY_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bits 16:23 - Years Counter"]
    #[inline(always)]
    pub fn ctryr(&self) -> CTRYR_R {
        CTRYR_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:12 - Months Counter"]
    #[inline(always)]
    pub fn ctrmo(&self) -> CTRMO_R {
        CTRMO_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 0:5 - Date Counter"]
    #[inline(always)]
    pub fn ctrdate(&self) -> CTRDATE_R {
        CTRDATE_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 31 - Counter read error status"]
    #[inline(always)]
    pub fn cterr(&mut self) -> CTERR_W {
        CTERR_W { w: self }
    }
    #[doc = "Bit 28 - Century enable"]
    #[inline(always)]
    pub fn ceb(&mut self) -> CEB_W {
        CEB_W { w: self }
    }
    #[doc = "Bit 27 - Century"]
    #[inline(always)]
    pub fn cb(&mut self) -> CB_W {
        CB_W { w: self }
    }
    #[doc = "Bits 24:26 - Weekdays Counter"]
    #[inline(always)]
    pub fn ctrwkdy(&mut self) -> CTRWKDY_W {
        CTRWKDY_W { w: self }
    }
    #[doc = "Bits 16:23 - Years Counter"]
    #[inline(always)]
    pub fn ctryr(&mut self) -> CTRYR_W {
        CTRYR_W { w: self }
    }
    #[doc = "Bits 8:12 - Months Counter"]
    #[inline(always)]
    pub fn ctrmo(&mut self) -> CTRMO_W {
        CTRMO_W { w: self }
    }
    #[doc = "Bits 0:5 - Date Counter"]
    #[inline(always)]
    pub fn ctrdate(&mut self) -> CTRDATE_W {
        CTRDATE_W { w: self }
    }
}
