#[doc = "Reader of register CHIP_INFO"]
pub type R = crate::R<u32, super::CHIP_INFO>;
#[doc = "Writer for register CHIP_INFO"]
pub type W = crate::W<u32, super::CHIP_INFO>;
#[doc = "Register CHIP_INFO `reset()`'s with value 0x0141_114b"]
impl crate::ResetValue for super::CHIP_INFO {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0141_114b
    }
}
#[doc = "Device class.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CLASS_A {
    #[doc = "1: APOLLO"]
    APOLLO = 1,
}
impl From<CLASS_A> for u8 {
    #[inline(always)]
    fn from(variant: CLASS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CLASS`"]
pub type CLASS_R = crate::R<u8, CLASS_A>;
impl CLASS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CLASS_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(CLASS_A::APOLLO),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `APOLLO`"]
    #[inline(always)]
    pub fn is_apollo(&self) -> bool {
        *self == CLASS_A::APOLLO
    }
}
#[doc = "Write proxy for field `CLASS`"]
pub struct CLASS_W<'a> {
    w: &'a mut W,
}
impl<'a> CLASS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLASS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "APOLLO"]
    #[inline(always)]
    pub fn apollo(self) -> &'a mut W {
        self.variant(CLASS_A::APOLLO)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
#[doc = "Device flash size.\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FLASH_A {
    #[doc = "3: 256K of available flash."]
    _256K = 3,
    #[doc = "4: 512K of available flash."]
    _512K = 4,
}
impl From<FLASH_A> for u8 {
    #[inline(always)]
    fn from(variant: FLASH_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FLASH`"]
pub type FLASH_R = crate::R<u8, FLASH_A>;
impl FLASH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, FLASH_A> {
        use crate::Variant::*;
        match self.bits {
            3 => Val(FLASH_A::_256K),
            4 => Val(FLASH_A::_512K),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_256K`"]
    #[inline(always)]
    pub fn is_256k(&self) -> bool {
        *self == FLASH_A::_256K
    }
    #[doc = "Checks if the value of the field is `_512K`"]
    #[inline(always)]
    pub fn is_512k(&self) -> bool {
        *self == FLASH_A::_512K
    }
}
#[doc = "Write proxy for field `FLASH`"]
pub struct FLASH_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLASH_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "256K of available flash."]
    #[inline(always)]
    pub fn _256k(self) -> &'a mut W {
        self.variant(FLASH_A::_256K)
    }
    #[doc = "512K of available flash."]
    #[inline(always)]
    pub fn _512k(self) -> &'a mut W {
        self.variant(FLASH_A::_512K)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "Device RAM size.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RAM_A {
    #[doc = "0: 32K of available SRAM."]
    _32K = 0,
    #[doc = "1: 64K of available SRAM."]
    _64K = 1,
}
impl From<RAM_A> for u8 {
    #[inline(always)]
    fn from(variant: RAM_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RAM`"]
pub type RAM_R = crate::R<u8, RAM_A>;
impl RAM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, RAM_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(RAM_A::_32K),
            1 => Val(RAM_A::_64K),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_32K`"]
    #[inline(always)]
    pub fn is_32k(&self) -> bool {
        *self == RAM_A::_32K
    }
    #[doc = "Checks if the value of the field is `_64K`"]
    #[inline(always)]
    pub fn is_64k(&self) -> bool {
        *self == RAM_A::_64K
    }
}
#[doc = "Write proxy for field `RAM`"]
pub struct RAM_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RAM_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "32K of available SRAM."]
    #[inline(always)]
    pub fn _32k(self) -> &'a mut W {
        self.variant(RAM_A::_32K)
    }
    #[doc = "64K of available SRAM."]
    #[inline(always)]
    pub fn _64k(self) -> &'a mut W {
        self.variant(RAM_A::_64K)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `MAJORREV`"]
pub type MAJORREV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MAJORREV`"]
pub struct MAJORREV_W<'a> {
    w: &'a mut W,
}
impl<'a> MAJORREV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Reader of field `MINORREV`"]
pub type MINORREV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MINORREV`"]
pub struct MINORREV_W<'a> {
    w: &'a mut W,
}
impl<'a> MINORREV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Device package type.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PKG_A {
    #[doc = "2: Ball grid array."]
    BGA = 2,
    #[doc = "3: Chip-scale package."]
    CSP = 3,
}
impl From<PKG_A> for u8 {
    #[inline(always)]
    fn from(variant: PKG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PKG`"]
pub type PKG_R = crate::R<u8, PKG_A>;
impl PKG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PKG_A> {
        use crate::Variant::*;
        match self.bits {
            2 => Val(PKG_A::BGA),
            3 => Val(PKG_A::CSP),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `BGA`"]
    #[inline(always)]
    pub fn is_bga(&self) -> bool {
        *self == PKG_A::BGA
    }
    #[doc = "Checks if the value of the field is `CSP`"]
    #[inline(always)]
    pub fn is_csp(&self) -> bool {
        *self == PKG_A::CSP
    }
}
#[doc = "Write proxy for field `PKG`"]
pub struct PKG_W<'a> {
    w: &'a mut W,
}
impl<'a> PKG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PKG_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Ball grid array."]
    #[inline(always)]
    pub fn bga(self) -> &'a mut W {
        self.variant(PKG_A::BGA)
    }
    #[doc = "Chip-scale package."]
    #[inline(always)]
    pub fn csp(self) -> &'a mut W {
        self.variant(PKG_A::CSP)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Number of pins.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PINS_A {
    #[doc = "1: 41 package pins total."]
    _41PINS = 1,
}
impl From<PINS_A> for u8 {
    #[inline(always)]
    fn from(variant: PINS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PINS`"]
pub type PINS_R = crate::R<u8, PINS_A>;
impl PINS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PINS_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(PINS_A::_41PINS),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_41PINS`"]
    #[inline(always)]
    pub fn is_41pins(&self) -> bool {
        *self == PINS_A::_41PINS
    }
}
#[doc = "Write proxy for field `PINS`"]
pub struct PINS_W<'a> {
    w: &'a mut W,
}
impl<'a> PINS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PINS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "41 package pins total."]
    #[inline(always)]
    pub fn _41pins(self) -> &'a mut W {
        self.variant(PINS_A::_41PINS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | (((value as u32) & 0x07) << 3);
        self.w
    }
}
#[doc = "Device temperature range.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TEMP_A {
    #[doc = "0: Commercial temperature range."]
    COMMERCIAL = 0,
}
impl From<TEMP_A> for u8 {
    #[inline(always)]
    fn from(variant: TEMP_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TEMP`"]
pub type TEMP_R = crate::R<u8, TEMP_A>;
impl TEMP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TEMP_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TEMP_A::COMMERCIAL),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `COMMERCIAL`"]
    #[inline(always)]
    pub fn is_commercial(&self) -> bool {
        *self == TEMP_A::COMMERCIAL
    }
}
#[doc = "Write proxy for field `TEMP`"]
pub struct TEMP_W<'a> {
    w: &'a mut W,
}
impl<'a> TEMP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TEMP_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Commercial temperature range."]
    #[inline(always)]
    pub fn commercial(self) -> &'a mut W {
        self.variant(TEMP_A::COMMERCIAL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | (((value as u32) & 0x03) << 1);
        self.w
    }
}
#[doc = "Device qualified.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum QUAL_A {
    #[doc = "0: Prototype device."]
    PROTOTYPE = 0,
    #[doc = "1: Fully qualified device."]
    QUALIFIED = 1,
}
impl From<QUAL_A> for bool {
    #[inline(always)]
    fn from(variant: QUAL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `QUAL`"]
pub type QUAL_R = crate::R<bool, QUAL_A>;
impl QUAL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> QUAL_A {
        match self.bits {
            false => QUAL_A::PROTOTYPE,
            true => QUAL_A::QUALIFIED,
        }
    }
    #[doc = "Checks if the value of the field is `PROTOTYPE`"]
    #[inline(always)]
    pub fn is_prototype(&self) -> bool {
        *self == QUAL_A::PROTOTYPE
    }
    #[doc = "Checks if the value of the field is `QUALIFIED`"]
    #[inline(always)]
    pub fn is_qualified(&self) -> bool {
        *self == QUAL_A::QUALIFIED
    }
}
#[doc = "Write proxy for field `QUAL`"]
pub struct QUAL_W<'a> {
    w: &'a mut W,
}
impl<'a> QUAL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: QUAL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Prototype device."]
    #[inline(always)]
    pub fn prototype(self) -> &'a mut W {
        self.variant(QUAL_A::PROTOTYPE)
    }
    #[doc = "Fully qualified device."]
    #[inline(always)]
    pub fn qualified(self) -> &'a mut W {
        self.variant(QUAL_A::QUALIFIED)
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
    #[doc = "Bits 24:31 - Device class."]
    #[inline(always)]
    pub fn class(&self) -> CLASS_R {
        CLASS_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 20:23 - Device flash size."]
    #[inline(always)]
    pub fn flash(&self) -> FLASH_R {
        FLASH_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Device RAM size."]
    #[inline(always)]
    pub fn ram(&self) -> RAM_R {
        RAM_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Major device revision number."]
    #[inline(always)]
    pub fn majorrev(&self) -> MAJORREV_R {
        MAJORREV_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Minor device revision number."]
    #[inline(always)]
    pub fn minorrev(&self) -> MINORREV_R {
        MINORREV_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 6:7 - Device package type."]
    #[inline(always)]
    pub fn pkg(&self) -> PKG_R {
        PKG_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 3:5 - Number of pins."]
    #[inline(always)]
    pub fn pins(&self) -> PINS_R {
        PINS_R::new(((self.bits >> 3) & 0x07) as u8)
    }
    #[doc = "Bits 1:2 - Device temperature range."]
    #[inline(always)]
    pub fn temp(&self) -> TEMP_R {
        TEMP_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bit 0 - Device qualified."]
    #[inline(always)]
    pub fn qual(&self) -> QUAL_R {
        QUAL_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 24:31 - Device class."]
    #[inline(always)]
    pub fn class(&mut self) -> CLASS_W {
        CLASS_W { w: self }
    }
    #[doc = "Bits 20:23 - Device flash size."]
    #[inline(always)]
    pub fn flash(&mut self) -> FLASH_W {
        FLASH_W { w: self }
    }
    #[doc = "Bits 16:19 - Device RAM size."]
    #[inline(always)]
    pub fn ram(&mut self) -> RAM_W {
        RAM_W { w: self }
    }
    #[doc = "Bits 12:15 - Major device revision number."]
    #[inline(always)]
    pub fn majorrev(&mut self) -> MAJORREV_W {
        MAJORREV_W { w: self }
    }
    #[doc = "Bits 8:11 - Minor device revision number."]
    #[inline(always)]
    pub fn minorrev(&mut self) -> MINORREV_W {
        MINORREV_W { w: self }
    }
    #[doc = "Bits 6:7 - Device package type."]
    #[inline(always)]
    pub fn pkg(&mut self) -> PKG_W {
        PKG_W { w: self }
    }
    #[doc = "Bits 3:5 - Number of pins."]
    #[inline(always)]
    pub fn pins(&mut self) -> PINS_W {
        PINS_W { w: self }
    }
    #[doc = "Bits 1:2 - Device temperature range."]
    #[inline(always)]
    pub fn temp(&mut self) -> TEMP_W {
        TEMP_W { w: self }
    }
    #[doc = "Bit 0 - Device qualified."]
    #[inline(always)]
    pub fn qual(&mut self) -> QUAL_W {
        QUAL_W { w: self }
    }
}
