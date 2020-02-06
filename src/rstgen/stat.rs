#[doc = "Reader of register STAT"]
pub type R = crate::R<u32, super::STAT>;
#[doc = "Writer for register STAT"]
pub type W = crate::W<u32, super::STAT>;
#[doc = "Register STAT `reset()`'s with value 0"]
impl crate::ResetValue for super::STAT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WDRSTAT`"]
pub type WDRSTAT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WDRSTAT`"]
pub struct WDRSTAT_W<'a> {
    w: &'a mut W,
}
impl<'a> WDRSTAT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `DBGRSTAT`"]
pub type DBGRSTAT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DBGRSTAT`"]
pub struct DBGRSTAT_W<'a> {
    w: &'a mut W,
}
impl<'a> DBGRSTAT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `POIRSTAT`"]
pub type POIRSTAT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `POIRSTAT`"]
pub struct POIRSTAT_W<'a> {
    w: &'a mut W,
}
impl<'a> POIRSTAT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `SWRSTAT`"]
pub type SWRSTAT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWRSTAT`"]
pub struct SWRSTAT_W<'a> {
    w: &'a mut W,
}
impl<'a> SWRSTAT_W<'a> {
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
#[doc = "Reader of field `BORSTAT`"]
pub type BORSTAT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BORSTAT`"]
pub struct BORSTAT_W<'a> {
    w: &'a mut W,
}
impl<'a> BORSTAT_W<'a> {
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
#[doc = "Reader of field `PORSTAT`"]
pub type PORSTAT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PORSTAT`"]
pub struct PORSTAT_W<'a> {
    w: &'a mut W,
}
impl<'a> PORSTAT_W<'a> {
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
#[doc = "Reader of field `EXRSTAT`"]
pub type EXRSTAT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXRSTAT`"]
pub struct EXRSTAT_W<'a> {
    w: &'a mut W,
}
impl<'a> EXRSTAT_W<'a> {
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
    #[doc = "Bit 6 - Reset was initiated by a Watchdog Timer Reset."]
    #[inline(always)]
    pub fn wdrstat(&self) -> WDRSTAT_R {
        WDRSTAT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Reset was a initiated by Debugger Reset."]
    #[inline(always)]
    pub fn dbgrstat(&self) -> DBGRSTAT_R {
        DBGRSTAT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Reset was a initiated by Software POI Reset."]
    #[inline(always)]
    pub fn poirstat(&self) -> POIRSTAT_R {
        POIRSTAT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Reset was a initiated by SW POR or AIRCR Reset."]
    #[inline(always)]
    pub fn swrstat(&self) -> SWRSTAT_R {
        SWRSTAT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Reset was initiated by a Brown-Out Reset."]
    #[inline(always)]
    pub fn borstat(&self) -> BORSTAT_R {
        BORSTAT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Reset was initiated by a Power-On Reset."]
    #[inline(always)]
    pub fn porstat(&self) -> PORSTAT_R {
        PORSTAT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Reset was initiated by an External Reset."]
    #[inline(always)]
    pub fn exrstat(&self) -> EXRSTAT_R {
        EXRSTAT_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - Reset was initiated by a Watchdog Timer Reset."]
    #[inline(always)]
    pub fn wdrstat(&mut self) -> WDRSTAT_W {
        WDRSTAT_W { w: self }
    }
    #[doc = "Bit 5 - Reset was a initiated by Debugger Reset."]
    #[inline(always)]
    pub fn dbgrstat(&mut self) -> DBGRSTAT_W {
        DBGRSTAT_W { w: self }
    }
    #[doc = "Bit 4 - Reset was a initiated by Software POI Reset."]
    #[inline(always)]
    pub fn poirstat(&mut self) -> POIRSTAT_W {
        POIRSTAT_W { w: self }
    }
    #[doc = "Bit 3 - Reset was a initiated by SW POR or AIRCR Reset."]
    #[inline(always)]
    pub fn swrstat(&mut self) -> SWRSTAT_W {
        SWRSTAT_W { w: self }
    }
    #[doc = "Bit 2 - Reset was initiated by a Brown-Out Reset."]
    #[inline(always)]
    pub fn borstat(&mut self) -> BORSTAT_W {
        BORSTAT_W { w: self }
    }
    #[doc = "Bit 1 - Reset was initiated by a Power-On Reset."]
    #[inline(always)]
    pub fn porstat(&mut self) -> PORSTAT_W {
        PORSTAT_W { w: self }
    }
    #[doc = "Bit 0 - Reset was initiated by an External Reset."]
    #[inline(always)]
    pub fn exrstat(&mut self) -> EXRSTAT_W {
        EXRSTAT_W { w: self }
    }
}
