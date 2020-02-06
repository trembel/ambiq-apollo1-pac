#[doc = "Reader of register INT0CLR"]
pub type R = crate::R<u32, super::INT0CLR>;
#[doc = "Writer for register INT0CLR"]
pub type W = crate::W<u32, super::INT0CLR>;
#[doc = "Register INT0CLR `reset()`'s with value 0"]
impl crate::ResetValue for super::INT0CLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `GPIO31`"]
pub type GPIO31_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO31`"]
pub struct GPIO31_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO31_W<'a> {
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
#[doc = "Reader of field `GPIO30`"]
pub type GPIO30_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO30`"]
pub struct GPIO30_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO30_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `GPIO29`"]
pub type GPIO29_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO29`"]
pub struct GPIO29_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO29_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `GPIO28`"]
pub type GPIO28_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO28`"]
pub struct GPIO28_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO28_W<'a> {
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
#[doc = "Reader of field `GPIO27`"]
pub type GPIO27_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO27`"]
pub struct GPIO27_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO27_W<'a> {
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
#[doc = "Reader of field `GPIO26`"]
pub type GPIO26_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO26`"]
pub struct GPIO26_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO26_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `GPIO25`"]
pub type GPIO25_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO25`"]
pub struct GPIO25_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO25_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `GPIO24`"]
pub type GPIO24_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO24`"]
pub struct GPIO24_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO24_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `GPIO23`"]
pub type GPIO23_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO23`"]
pub struct GPIO23_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO23_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `GPIO22`"]
pub type GPIO22_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO22`"]
pub struct GPIO22_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO22_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `GPIO21`"]
pub type GPIO21_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO21`"]
pub struct GPIO21_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO21_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `GPIO20`"]
pub type GPIO20_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO20`"]
pub struct GPIO20_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO20_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `GPIO19`"]
pub type GPIO19_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO19`"]
pub struct GPIO19_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO19_W<'a> {
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
#[doc = "Reader of field `GPIO18`"]
pub type GPIO18_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO18`"]
pub struct GPIO18_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO18_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `GPIO17`"]
pub type GPIO17_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO17`"]
pub struct GPIO17_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO17_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `GPIO16`"]
pub type GPIO16_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO16`"]
pub struct GPIO16_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO16_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `GPIO15`"]
pub type GPIO15_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO15`"]
pub struct GPIO15_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO15_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `GPIO14`"]
pub type GPIO14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO14`"]
pub struct GPIO14_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO14_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `GPIO13`"]
pub type GPIO13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO13`"]
pub struct GPIO13_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO13_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `GPIO12`"]
pub type GPIO12_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO12`"]
pub struct GPIO12_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO12_W<'a> {
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
#[doc = "Reader of field `GPIO11`"]
pub type GPIO11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO11`"]
pub struct GPIO11_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO11_W<'a> {
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
#[doc = "Reader of field `GPIO10`"]
pub type GPIO10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO10`"]
pub struct GPIO10_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO10_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `GPIO9`"]
pub type GPIO9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO9`"]
pub struct GPIO9_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO9_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `GPIO8`"]
pub type GPIO8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO8`"]
pub struct GPIO8_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO8_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `GPIO7`"]
pub type GPIO7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO7`"]
pub struct GPIO7_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO7_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `GPIO6`"]
pub type GPIO6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO6`"]
pub struct GPIO6_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO6_W<'a> {
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
#[doc = "Reader of field `GPIO5`"]
pub type GPIO5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO5`"]
pub struct GPIO5_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO5_W<'a> {
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
#[doc = "Reader of field `GPIO4`"]
pub type GPIO4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO4`"]
pub struct GPIO4_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO4_W<'a> {
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
#[doc = "Reader of field `GPIO3`"]
pub type GPIO3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO3`"]
pub struct GPIO3_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO3_W<'a> {
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
#[doc = "Reader of field `GPIO2`"]
pub type GPIO2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO2`"]
pub struct GPIO2_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO2_W<'a> {
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
#[doc = "Reader of field `GPIO1`"]
pub type GPIO1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO1`"]
pub struct GPIO1_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO1_W<'a> {
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
#[doc = "Reader of field `GPIO0`"]
pub type GPIO0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO0`"]
pub struct GPIO0_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO0_W<'a> {
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
    #[doc = "Bit 31 - GPIO31 interrupt."]
    #[inline(always)]
    pub fn gpio31(&self) -> GPIO31_R {
        GPIO31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - GPIO30 interrupt."]
    #[inline(always)]
    pub fn gpio30(&self) -> GPIO30_R {
        GPIO30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - GPIO29 interrupt."]
    #[inline(always)]
    pub fn gpio29(&self) -> GPIO29_R {
        GPIO29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - GPIO28 interrupt."]
    #[inline(always)]
    pub fn gpio28(&self) -> GPIO28_R {
        GPIO28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - GPIO27 interrupt."]
    #[inline(always)]
    pub fn gpio27(&self) -> GPIO27_R {
        GPIO27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26 - GPIO26 interrupt."]
    #[inline(always)]
    pub fn gpio26(&self) -> GPIO26_R {
        GPIO26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - GPIO25 interrupt."]
    #[inline(always)]
    pub fn gpio25(&self) -> GPIO25_R {
        GPIO25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - GPIO24 interrupt."]
    #[inline(always)]
    pub fn gpio24(&self) -> GPIO24_R {
        GPIO24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - GPIO23 interrupt."]
    #[inline(always)]
    pub fn gpio23(&self) -> GPIO23_R {
        GPIO23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - GPIO22 interrupt."]
    #[inline(always)]
    pub fn gpio22(&self) -> GPIO22_R {
        GPIO22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - GPIO21 interrupt."]
    #[inline(always)]
    pub fn gpio21(&self) -> GPIO21_R {
        GPIO21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - GPIO20 interrupt."]
    #[inline(always)]
    pub fn gpio20(&self) -> GPIO20_R {
        GPIO20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - GPIO19 interrupt."]
    #[inline(always)]
    pub fn gpio19(&self) -> GPIO19_R {
        GPIO19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - GPIO18interrupt."]
    #[inline(always)]
    pub fn gpio18(&self) -> GPIO18_R {
        GPIO18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - GPIO17 interrupt."]
    #[inline(always)]
    pub fn gpio17(&self) -> GPIO17_R {
        GPIO17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - GPIO16 interrupt."]
    #[inline(always)]
    pub fn gpio16(&self) -> GPIO16_R {
        GPIO16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - GPIO15 interrupt."]
    #[inline(always)]
    pub fn gpio15(&self) -> GPIO15_R {
        GPIO15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - GPIO14 interrupt."]
    #[inline(always)]
    pub fn gpio14(&self) -> GPIO14_R {
        GPIO14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - GPIO13 interrupt."]
    #[inline(always)]
    pub fn gpio13(&self) -> GPIO13_R {
        GPIO13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - GPIO12 interrupt."]
    #[inline(always)]
    pub fn gpio12(&self) -> GPIO12_R {
        GPIO12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - GPIO11 interrupt."]
    #[inline(always)]
    pub fn gpio11(&self) -> GPIO11_R {
        GPIO11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - GPIO10 interrupt."]
    #[inline(always)]
    pub fn gpio10(&self) -> GPIO10_R {
        GPIO10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - GPIO9 interrupt."]
    #[inline(always)]
    pub fn gpio9(&self) -> GPIO9_R {
        GPIO9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - GPIO8 interrupt."]
    #[inline(always)]
    pub fn gpio8(&self) -> GPIO8_R {
        GPIO8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - GPIO7 interrupt."]
    #[inline(always)]
    pub fn gpio7(&self) -> GPIO7_R {
        GPIO7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - GPIO6 interrupt."]
    #[inline(always)]
    pub fn gpio6(&self) -> GPIO6_R {
        GPIO6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - GPIO5 interrupt."]
    #[inline(always)]
    pub fn gpio5(&self) -> GPIO5_R {
        GPIO5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - GPIO4 interrupt."]
    #[inline(always)]
    pub fn gpio4(&self) -> GPIO4_R {
        GPIO4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - GPIO3 interrupt."]
    #[inline(always)]
    pub fn gpio3(&self) -> GPIO3_R {
        GPIO3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - GPIO2 interrupt."]
    #[inline(always)]
    pub fn gpio2(&self) -> GPIO2_R {
        GPIO2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - GPIO1 interrupt."]
    #[inline(always)]
    pub fn gpio1(&self) -> GPIO1_R {
        GPIO1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - GPIO0 interrupt."]
    #[inline(always)]
    pub fn gpio0(&self) -> GPIO0_R {
        GPIO0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - GPIO31 interrupt."]
    #[inline(always)]
    pub fn gpio31(&mut self) -> GPIO31_W {
        GPIO31_W { w: self }
    }
    #[doc = "Bit 30 - GPIO30 interrupt."]
    #[inline(always)]
    pub fn gpio30(&mut self) -> GPIO30_W {
        GPIO30_W { w: self }
    }
    #[doc = "Bit 29 - GPIO29 interrupt."]
    #[inline(always)]
    pub fn gpio29(&mut self) -> GPIO29_W {
        GPIO29_W { w: self }
    }
    #[doc = "Bit 28 - GPIO28 interrupt."]
    #[inline(always)]
    pub fn gpio28(&mut self) -> GPIO28_W {
        GPIO28_W { w: self }
    }
    #[doc = "Bit 27 - GPIO27 interrupt."]
    #[inline(always)]
    pub fn gpio27(&mut self) -> GPIO27_W {
        GPIO27_W { w: self }
    }
    #[doc = "Bit 26 - GPIO26 interrupt."]
    #[inline(always)]
    pub fn gpio26(&mut self) -> GPIO26_W {
        GPIO26_W { w: self }
    }
    #[doc = "Bit 25 - GPIO25 interrupt."]
    #[inline(always)]
    pub fn gpio25(&mut self) -> GPIO25_W {
        GPIO25_W { w: self }
    }
    #[doc = "Bit 24 - GPIO24 interrupt."]
    #[inline(always)]
    pub fn gpio24(&mut self) -> GPIO24_W {
        GPIO24_W { w: self }
    }
    #[doc = "Bit 23 - GPIO23 interrupt."]
    #[inline(always)]
    pub fn gpio23(&mut self) -> GPIO23_W {
        GPIO23_W { w: self }
    }
    #[doc = "Bit 22 - GPIO22 interrupt."]
    #[inline(always)]
    pub fn gpio22(&mut self) -> GPIO22_W {
        GPIO22_W { w: self }
    }
    #[doc = "Bit 21 - GPIO21 interrupt."]
    #[inline(always)]
    pub fn gpio21(&mut self) -> GPIO21_W {
        GPIO21_W { w: self }
    }
    #[doc = "Bit 20 - GPIO20 interrupt."]
    #[inline(always)]
    pub fn gpio20(&mut self) -> GPIO20_W {
        GPIO20_W { w: self }
    }
    #[doc = "Bit 19 - GPIO19 interrupt."]
    #[inline(always)]
    pub fn gpio19(&mut self) -> GPIO19_W {
        GPIO19_W { w: self }
    }
    #[doc = "Bit 18 - GPIO18interrupt."]
    #[inline(always)]
    pub fn gpio18(&mut self) -> GPIO18_W {
        GPIO18_W { w: self }
    }
    #[doc = "Bit 17 - GPIO17 interrupt."]
    #[inline(always)]
    pub fn gpio17(&mut self) -> GPIO17_W {
        GPIO17_W { w: self }
    }
    #[doc = "Bit 16 - GPIO16 interrupt."]
    #[inline(always)]
    pub fn gpio16(&mut self) -> GPIO16_W {
        GPIO16_W { w: self }
    }
    #[doc = "Bit 15 - GPIO15 interrupt."]
    #[inline(always)]
    pub fn gpio15(&mut self) -> GPIO15_W {
        GPIO15_W { w: self }
    }
    #[doc = "Bit 14 - GPIO14 interrupt."]
    #[inline(always)]
    pub fn gpio14(&mut self) -> GPIO14_W {
        GPIO14_W { w: self }
    }
    #[doc = "Bit 13 - GPIO13 interrupt."]
    #[inline(always)]
    pub fn gpio13(&mut self) -> GPIO13_W {
        GPIO13_W { w: self }
    }
    #[doc = "Bit 12 - GPIO12 interrupt."]
    #[inline(always)]
    pub fn gpio12(&mut self) -> GPIO12_W {
        GPIO12_W { w: self }
    }
    #[doc = "Bit 11 - GPIO11 interrupt."]
    #[inline(always)]
    pub fn gpio11(&mut self) -> GPIO11_W {
        GPIO11_W { w: self }
    }
    #[doc = "Bit 10 - GPIO10 interrupt."]
    #[inline(always)]
    pub fn gpio10(&mut self) -> GPIO10_W {
        GPIO10_W { w: self }
    }
    #[doc = "Bit 9 - GPIO9 interrupt."]
    #[inline(always)]
    pub fn gpio9(&mut self) -> GPIO9_W {
        GPIO9_W { w: self }
    }
    #[doc = "Bit 8 - GPIO8 interrupt."]
    #[inline(always)]
    pub fn gpio8(&mut self) -> GPIO8_W {
        GPIO8_W { w: self }
    }
    #[doc = "Bit 7 - GPIO7 interrupt."]
    #[inline(always)]
    pub fn gpio7(&mut self) -> GPIO7_W {
        GPIO7_W { w: self }
    }
    #[doc = "Bit 6 - GPIO6 interrupt."]
    #[inline(always)]
    pub fn gpio6(&mut self) -> GPIO6_W {
        GPIO6_W { w: self }
    }
    #[doc = "Bit 5 - GPIO5 interrupt."]
    #[inline(always)]
    pub fn gpio5(&mut self) -> GPIO5_W {
        GPIO5_W { w: self }
    }
    #[doc = "Bit 4 - GPIO4 interrupt."]
    #[inline(always)]
    pub fn gpio4(&mut self) -> GPIO4_W {
        GPIO4_W { w: self }
    }
    #[doc = "Bit 3 - GPIO3 interrupt."]
    #[inline(always)]
    pub fn gpio3(&mut self) -> GPIO3_W {
        GPIO3_W { w: self }
    }
    #[doc = "Bit 2 - GPIO2 interrupt."]
    #[inline(always)]
    pub fn gpio2(&mut self) -> GPIO2_W {
        GPIO2_W { w: self }
    }
    #[doc = "Bit 1 - GPIO1 interrupt."]
    #[inline(always)]
    pub fn gpio1(&mut self) -> GPIO1_W {
        GPIO1_W { w: self }
    }
    #[doc = "Bit 0 - GPIO0 interrupt."]
    #[inline(always)]
    pub fn gpio0(&mut self) -> GPIO0_W {
        GPIO0_W { w: self }
    }
}
