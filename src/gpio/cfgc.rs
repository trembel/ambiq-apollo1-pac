#[doc = "Reader of register CFGC"]
pub type R = crate::R<u32, super::CFGC>;
#[doc = "Writer for register CFGC"]
pub type W = crate::W<u32, super::CFGC>;
#[doc = "Register CFGC `reset()`'s with value 0x0011_0000"]
impl crate::ResetValue for super::CFGC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0011_0000
    }
}
#[doc = "GPIO23 interrupt direction.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO23INTD_A {
    #[doc = "0: Interrupt on low to high GPIO transition"]
    INTLH = 0,
    #[doc = "1: Interrupt on high to low GPIO transition"]
    INTHL = 1,
}
impl From<GPIO23INTD_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO23INTD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIO23INTD`"]
pub type GPIO23INTD_R = crate::R<bool, GPIO23INTD_A>;
impl GPIO23INTD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO23INTD_A {
        match self.bits {
            false => GPIO23INTD_A::INTLH,
            true => GPIO23INTD_A::INTHL,
        }
    }
    #[doc = "Checks if the value of the field is `INTLH`"]
    #[inline(always)]
    pub fn is_intlh(&self) -> bool {
        *self == GPIO23INTD_A::INTLH
    }
    #[doc = "Checks if the value of the field is `INTHL`"]
    #[inline(always)]
    pub fn is_inthl(&self) -> bool {
        *self == GPIO23INTD_A::INTHL
    }
}
#[doc = "Write proxy for field `GPIO23INTD`"]
pub struct GPIO23INTD_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO23INTD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO23INTD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt on low to high GPIO transition"]
    #[inline(always)]
    pub fn intlh(self) -> &'a mut W {
        self.variant(GPIO23INTD_A::INTLH)
    }
    #[doc = "Interrupt on high to low GPIO transition"]
    #[inline(always)]
    pub fn inthl(self) -> &'a mut W {
        self.variant(GPIO23INTD_A::INTHL)
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
#[doc = "GPIO23 output configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GPIO23OUTCFG_A {
    #[doc = "0: Output disabled"]
    DIS = 0,
    #[doc = "1: Output is push-pull"]
    PUSHPULL = 1,
    #[doc = "2: Output is open drain"]
    OD = 2,
    #[doc = "3: Output is tri-state"]
    TS = 3,
}
impl From<GPIO23OUTCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO23OUTCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `GPIO23OUTCFG`"]
pub type GPIO23OUTCFG_R = crate::R<u8, GPIO23OUTCFG_A>;
impl GPIO23OUTCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO23OUTCFG_A {
        match self.bits {
            0 => GPIO23OUTCFG_A::DIS,
            1 => GPIO23OUTCFG_A::PUSHPULL,
            2 => GPIO23OUTCFG_A::OD,
            3 => GPIO23OUTCFG_A::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == GPIO23OUTCFG_A::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == GPIO23OUTCFG_A::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        *self == GPIO23OUTCFG_A::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        *self == GPIO23OUTCFG_A::TS
    }
}
#[doc = "Write proxy for field `GPIO23OUTCFG`"]
pub struct GPIO23OUTCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO23OUTCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO23OUTCFG_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Output disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO23OUTCFG_A::DIS)
    }
    #[doc = "Output is push-pull"]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO23OUTCFG_A::PUSHPULL)
    }
    #[doc = "Output is open drain"]
    #[inline(always)]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO23OUTCFG_A::OD)
    }
    #[doc = "Output is tri-state"]
    #[inline(always)]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO23OUTCFG_A::TS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 29)) | (((value as u32) & 0x03) << 29);
        self.w
    }
}
#[doc = "GPIO23 input enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO23INCFG_A {
    #[doc = "0: Read the GPIO pin data"]
    READ = 0,
    #[doc = "1: Readback will always be zero"]
    RDZERO = 1,
}
impl From<GPIO23INCFG_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO23INCFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIO23INCFG`"]
pub type GPIO23INCFG_R = crate::R<bool, GPIO23INCFG_A>;
impl GPIO23INCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO23INCFG_A {
        match self.bits {
            false => GPIO23INCFG_A::READ,
            true => GPIO23INCFG_A::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        *self == GPIO23INCFG_A::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline(always)]
    pub fn is_rdzero(&self) -> bool {
        *self == GPIO23INCFG_A::RDZERO
    }
}
#[doc = "Write proxy for field `GPIO23INCFG`"]
pub struct GPIO23INCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO23INCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO23INCFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Read the GPIO pin data"]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO23INCFG_A::READ)
    }
    #[doc = "Readback will always be zero"]
    #[inline(always)]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO23INCFG_A::RDZERO)
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
#[doc = "GPIO22 interrupt direction.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO22INTD_A {
    #[doc = "0: Interrupt on low to high GPIO transition"]
    INTLH = 0,
    #[doc = "1: Interrupt on high to low GPIO transition"]
    INTHL = 1,
}
impl From<GPIO22INTD_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO22INTD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIO22INTD`"]
pub type GPIO22INTD_R = crate::R<bool, GPIO22INTD_A>;
impl GPIO22INTD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO22INTD_A {
        match self.bits {
            false => GPIO22INTD_A::INTLH,
            true => GPIO22INTD_A::INTHL,
        }
    }
    #[doc = "Checks if the value of the field is `INTLH`"]
    #[inline(always)]
    pub fn is_intlh(&self) -> bool {
        *self == GPIO22INTD_A::INTLH
    }
    #[doc = "Checks if the value of the field is `INTHL`"]
    #[inline(always)]
    pub fn is_inthl(&self) -> bool {
        *self == GPIO22INTD_A::INTHL
    }
}
#[doc = "Write proxy for field `GPIO22INTD`"]
pub struct GPIO22INTD_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO22INTD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO22INTD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt on low to high GPIO transition"]
    #[inline(always)]
    pub fn intlh(self) -> &'a mut W {
        self.variant(GPIO22INTD_A::INTLH)
    }
    #[doc = "Interrupt on high to low GPIO transition"]
    #[inline(always)]
    pub fn inthl(self) -> &'a mut W {
        self.variant(GPIO22INTD_A::INTHL)
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
#[doc = "GPIO22 output configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GPIO22OUTCFG_A {
    #[doc = "0: Output disabled"]
    DIS = 0,
    #[doc = "1: Output is push-pull"]
    PUSHPULL = 1,
    #[doc = "2: Output is open drain"]
    OD = 2,
    #[doc = "3: Output is tri-state"]
    TS = 3,
}
impl From<GPIO22OUTCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO22OUTCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `GPIO22OUTCFG`"]
pub type GPIO22OUTCFG_R = crate::R<u8, GPIO22OUTCFG_A>;
impl GPIO22OUTCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO22OUTCFG_A {
        match self.bits {
            0 => GPIO22OUTCFG_A::DIS,
            1 => GPIO22OUTCFG_A::PUSHPULL,
            2 => GPIO22OUTCFG_A::OD,
            3 => GPIO22OUTCFG_A::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == GPIO22OUTCFG_A::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == GPIO22OUTCFG_A::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        *self == GPIO22OUTCFG_A::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        *self == GPIO22OUTCFG_A::TS
    }
}
#[doc = "Write proxy for field `GPIO22OUTCFG`"]
pub struct GPIO22OUTCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO22OUTCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO22OUTCFG_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Output disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO22OUTCFG_A::DIS)
    }
    #[doc = "Output is push-pull"]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO22OUTCFG_A::PUSHPULL)
    }
    #[doc = "Output is open drain"]
    #[inline(always)]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO22OUTCFG_A::OD)
    }
    #[doc = "Output is tri-state"]
    #[inline(always)]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO22OUTCFG_A::TS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 25)) | (((value as u32) & 0x03) << 25);
        self.w
    }
}
#[doc = "GPIO22 input enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO22INCFG_A {
    #[doc = "0: Read the GPIO pin data"]
    READ = 0,
    #[doc = "1: Readback will always be zero"]
    RDZERO = 1,
}
impl From<GPIO22INCFG_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO22INCFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIO22INCFG`"]
pub type GPIO22INCFG_R = crate::R<bool, GPIO22INCFG_A>;
impl GPIO22INCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO22INCFG_A {
        match self.bits {
            false => GPIO22INCFG_A::READ,
            true => GPIO22INCFG_A::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        *self == GPIO22INCFG_A::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline(always)]
    pub fn is_rdzero(&self) -> bool {
        *self == GPIO22INCFG_A::RDZERO
    }
}
#[doc = "Write proxy for field `GPIO22INCFG`"]
pub struct GPIO22INCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO22INCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO22INCFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Read the GPIO pin data"]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO22INCFG_A::READ)
    }
    #[doc = "Readback will always be zero"]
    #[inline(always)]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO22INCFG_A::RDZERO)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "GPIO21 interrupt direction.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO21INTD_A {
    #[doc = "0: Interrupt on low to high GPIO transition"]
    INTLH = 0,
    #[doc = "1: Interrupt on high to low GPIO transition"]
    INTHL = 1,
}
impl From<GPIO21INTD_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO21INTD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIO21INTD`"]
pub type GPIO21INTD_R = crate::R<bool, GPIO21INTD_A>;
impl GPIO21INTD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO21INTD_A {
        match self.bits {
            false => GPIO21INTD_A::INTLH,
            true => GPIO21INTD_A::INTHL,
        }
    }
    #[doc = "Checks if the value of the field is `INTLH`"]
    #[inline(always)]
    pub fn is_intlh(&self) -> bool {
        *self == GPIO21INTD_A::INTLH
    }
    #[doc = "Checks if the value of the field is `INTHL`"]
    #[inline(always)]
    pub fn is_inthl(&self) -> bool {
        *self == GPIO21INTD_A::INTHL
    }
}
#[doc = "Write proxy for field `GPIO21INTD`"]
pub struct GPIO21INTD_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO21INTD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO21INTD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt on low to high GPIO transition"]
    #[inline(always)]
    pub fn intlh(self) -> &'a mut W {
        self.variant(GPIO21INTD_A::INTLH)
    }
    #[doc = "Interrupt on high to low GPIO transition"]
    #[inline(always)]
    pub fn inthl(self) -> &'a mut W {
        self.variant(GPIO21INTD_A::INTHL)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "GPIO21 output configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GPIO21OUTCFG_A {
    #[doc = "0: Output disabled"]
    DIS = 0,
    #[doc = "1: Output is push-pull"]
    PUSHPULL = 1,
    #[doc = "2: Output is open drain"]
    OD = 2,
    #[doc = "3: Output is tri-state"]
    TS = 3,
}
impl From<GPIO21OUTCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO21OUTCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `GPIO21OUTCFG`"]
pub type GPIO21OUTCFG_R = crate::R<u8, GPIO21OUTCFG_A>;
impl GPIO21OUTCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO21OUTCFG_A {
        match self.bits {
            0 => GPIO21OUTCFG_A::DIS,
            1 => GPIO21OUTCFG_A::PUSHPULL,
            2 => GPIO21OUTCFG_A::OD,
            3 => GPIO21OUTCFG_A::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == GPIO21OUTCFG_A::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == GPIO21OUTCFG_A::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        *self == GPIO21OUTCFG_A::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        *self == GPIO21OUTCFG_A::TS
    }
}
#[doc = "Write proxy for field `GPIO21OUTCFG`"]
pub struct GPIO21OUTCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO21OUTCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO21OUTCFG_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Output disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO21OUTCFG_A::DIS)
    }
    #[doc = "Output is push-pull"]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO21OUTCFG_A::PUSHPULL)
    }
    #[doc = "Output is open drain"]
    #[inline(always)]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO21OUTCFG_A::OD)
    }
    #[doc = "Output is tri-state"]
    #[inline(always)]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO21OUTCFG_A::TS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 21)) | (((value as u32) & 0x03) << 21);
        self.w
    }
}
#[doc = "GPIO21 input enable.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO21INCFG_A {
    #[doc = "0: Read the GPIO pin data"]
    READ = 0,
    #[doc = "1: Readback will always be zero"]
    RDZERO = 1,
}
impl From<GPIO21INCFG_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO21INCFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIO21INCFG`"]
pub type GPIO21INCFG_R = crate::R<bool, GPIO21INCFG_A>;
impl GPIO21INCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO21INCFG_A {
        match self.bits {
            false => GPIO21INCFG_A::READ,
            true => GPIO21INCFG_A::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        *self == GPIO21INCFG_A::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline(always)]
    pub fn is_rdzero(&self) -> bool {
        *self == GPIO21INCFG_A::RDZERO
    }
}
#[doc = "Write proxy for field `GPIO21INCFG`"]
pub struct GPIO21INCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO21INCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO21INCFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Read the GPIO pin data"]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO21INCFG_A::READ)
    }
    #[doc = "Readback will always be zero"]
    #[inline(always)]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO21INCFG_A::RDZERO)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "GPIO20 interrupt direction.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO20INTD_A {
    #[doc = "0: Interrupt on low to high GPIO transition"]
    INTLH = 0,
    #[doc = "1: Interrupt on high to low GPIO transition"]
    INTHL = 1,
}
impl From<GPIO20INTD_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO20INTD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIO20INTD`"]
pub type GPIO20INTD_R = crate::R<bool, GPIO20INTD_A>;
impl GPIO20INTD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO20INTD_A {
        match self.bits {
            false => GPIO20INTD_A::INTLH,
            true => GPIO20INTD_A::INTHL,
        }
    }
    #[doc = "Checks if the value of the field is `INTLH`"]
    #[inline(always)]
    pub fn is_intlh(&self) -> bool {
        *self == GPIO20INTD_A::INTLH
    }
    #[doc = "Checks if the value of the field is `INTHL`"]
    #[inline(always)]
    pub fn is_inthl(&self) -> bool {
        *self == GPIO20INTD_A::INTHL
    }
}
#[doc = "Write proxy for field `GPIO20INTD`"]
pub struct GPIO20INTD_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO20INTD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO20INTD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt on low to high GPIO transition"]
    #[inline(always)]
    pub fn intlh(self) -> &'a mut W {
        self.variant(GPIO20INTD_A::INTLH)
    }
    #[doc = "Interrupt on high to low GPIO transition"]
    #[inline(always)]
    pub fn inthl(self) -> &'a mut W {
        self.variant(GPIO20INTD_A::INTHL)
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
#[doc = "GPIO20 output configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GPIO20OUTCFG_A {
    #[doc = "0: Output disabled"]
    DIS = 0,
    #[doc = "1: Output is push-pull"]
    PUSHPULL = 1,
    #[doc = "2: Output is open drain"]
    OD = 2,
    #[doc = "3: Output is tri-state"]
    TS = 3,
}
impl From<GPIO20OUTCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO20OUTCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `GPIO20OUTCFG`"]
pub type GPIO20OUTCFG_R = crate::R<u8, GPIO20OUTCFG_A>;
impl GPIO20OUTCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO20OUTCFG_A {
        match self.bits {
            0 => GPIO20OUTCFG_A::DIS,
            1 => GPIO20OUTCFG_A::PUSHPULL,
            2 => GPIO20OUTCFG_A::OD,
            3 => GPIO20OUTCFG_A::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == GPIO20OUTCFG_A::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == GPIO20OUTCFG_A::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        *self == GPIO20OUTCFG_A::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        *self == GPIO20OUTCFG_A::TS
    }
}
#[doc = "Write proxy for field `GPIO20OUTCFG`"]
pub struct GPIO20OUTCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO20OUTCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO20OUTCFG_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Output disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO20OUTCFG_A::DIS)
    }
    #[doc = "Output is push-pull"]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO20OUTCFG_A::PUSHPULL)
    }
    #[doc = "Output is open drain"]
    #[inline(always)]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO20OUTCFG_A::OD)
    }
    #[doc = "Output is tri-state"]
    #[inline(always)]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO20OUTCFG_A::TS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 17)) | (((value as u32) & 0x03) << 17);
        self.w
    }
}
#[doc = "GPIO20 input enable.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO20INCFG_A {
    #[doc = "0: Read the GPIO pin data"]
    READ = 0,
    #[doc = "1: Readback will always be zero"]
    RDZERO = 1,
}
impl From<GPIO20INCFG_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO20INCFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIO20INCFG`"]
pub type GPIO20INCFG_R = crate::R<bool, GPIO20INCFG_A>;
impl GPIO20INCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO20INCFG_A {
        match self.bits {
            false => GPIO20INCFG_A::READ,
            true => GPIO20INCFG_A::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        *self == GPIO20INCFG_A::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline(always)]
    pub fn is_rdzero(&self) -> bool {
        *self == GPIO20INCFG_A::RDZERO
    }
}
#[doc = "Write proxy for field `GPIO20INCFG`"]
pub struct GPIO20INCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO20INCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO20INCFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Read the GPIO pin data"]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO20INCFG_A::READ)
    }
    #[doc = "Readback will always be zero"]
    #[inline(always)]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO20INCFG_A::RDZERO)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "GPIO19 interrupt direction.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO19INTD_A {
    #[doc = "0: Interrupt on low to high GPIO transition"]
    INTLH = 0,
    #[doc = "1: Interrupt on high to low GPIO transition"]
    INTHL = 1,
}
impl From<GPIO19INTD_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO19INTD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIO19INTD`"]
pub type GPIO19INTD_R = crate::R<bool, GPIO19INTD_A>;
impl GPIO19INTD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO19INTD_A {
        match self.bits {
            false => GPIO19INTD_A::INTLH,
            true => GPIO19INTD_A::INTHL,
        }
    }
    #[doc = "Checks if the value of the field is `INTLH`"]
    #[inline(always)]
    pub fn is_intlh(&self) -> bool {
        *self == GPIO19INTD_A::INTLH
    }
    #[doc = "Checks if the value of the field is `INTHL`"]
    #[inline(always)]
    pub fn is_inthl(&self) -> bool {
        *self == GPIO19INTD_A::INTHL
    }
}
#[doc = "Write proxy for field `GPIO19INTD`"]
pub struct GPIO19INTD_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO19INTD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO19INTD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt on low to high GPIO transition"]
    #[inline(always)]
    pub fn intlh(self) -> &'a mut W {
        self.variant(GPIO19INTD_A::INTLH)
    }
    #[doc = "Interrupt on high to low GPIO transition"]
    #[inline(always)]
    pub fn inthl(self) -> &'a mut W {
        self.variant(GPIO19INTD_A::INTHL)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "GPIO19 output configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GPIO19OUTCFG_A {
    #[doc = "0: Output disabled"]
    DIS = 0,
    #[doc = "1: Output is push-pull"]
    PUSHPULL = 1,
    #[doc = "2: Output is open drain"]
    OD = 2,
    #[doc = "3: Output is tri-state"]
    TS = 3,
}
impl From<GPIO19OUTCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO19OUTCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `GPIO19OUTCFG`"]
pub type GPIO19OUTCFG_R = crate::R<u8, GPIO19OUTCFG_A>;
impl GPIO19OUTCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO19OUTCFG_A {
        match self.bits {
            0 => GPIO19OUTCFG_A::DIS,
            1 => GPIO19OUTCFG_A::PUSHPULL,
            2 => GPIO19OUTCFG_A::OD,
            3 => GPIO19OUTCFG_A::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == GPIO19OUTCFG_A::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == GPIO19OUTCFG_A::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        *self == GPIO19OUTCFG_A::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        *self == GPIO19OUTCFG_A::TS
    }
}
#[doc = "Write proxy for field `GPIO19OUTCFG`"]
pub struct GPIO19OUTCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO19OUTCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO19OUTCFG_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Output disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO19OUTCFG_A::DIS)
    }
    #[doc = "Output is push-pull"]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO19OUTCFG_A::PUSHPULL)
    }
    #[doc = "Output is open drain"]
    #[inline(always)]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO19OUTCFG_A::OD)
    }
    #[doc = "Output is tri-state"]
    #[inline(always)]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO19OUTCFG_A::TS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 13)) | (((value as u32) & 0x03) << 13);
        self.w
    }
}
#[doc = "GPIO19 input enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO19INCFG_A {
    #[doc = "0: Read the GPIO pin data"]
    READ = 0,
    #[doc = "1: Readback will always be zero"]
    RDZERO = 1,
}
impl From<GPIO19INCFG_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO19INCFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIO19INCFG`"]
pub type GPIO19INCFG_R = crate::R<bool, GPIO19INCFG_A>;
impl GPIO19INCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO19INCFG_A {
        match self.bits {
            false => GPIO19INCFG_A::READ,
            true => GPIO19INCFG_A::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        *self == GPIO19INCFG_A::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline(always)]
    pub fn is_rdzero(&self) -> bool {
        *self == GPIO19INCFG_A::RDZERO
    }
}
#[doc = "Write proxy for field `GPIO19INCFG`"]
pub struct GPIO19INCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO19INCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO19INCFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Read the GPIO pin data"]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO19INCFG_A::READ)
    }
    #[doc = "Readback will always be zero"]
    #[inline(always)]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO19INCFG_A::RDZERO)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "GPIO18 interrupt direction.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO18INTD_A {
    #[doc = "0: Interrupt on low to high GPIO transition"]
    INTLH = 0,
    #[doc = "1: Interrupt on high to low GPIO transition"]
    INTHL = 1,
}
impl From<GPIO18INTD_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO18INTD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIO18INTD`"]
pub type GPIO18INTD_R = crate::R<bool, GPIO18INTD_A>;
impl GPIO18INTD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO18INTD_A {
        match self.bits {
            false => GPIO18INTD_A::INTLH,
            true => GPIO18INTD_A::INTHL,
        }
    }
    #[doc = "Checks if the value of the field is `INTLH`"]
    #[inline(always)]
    pub fn is_intlh(&self) -> bool {
        *self == GPIO18INTD_A::INTLH
    }
    #[doc = "Checks if the value of the field is `INTHL`"]
    #[inline(always)]
    pub fn is_inthl(&self) -> bool {
        *self == GPIO18INTD_A::INTHL
    }
}
#[doc = "Write proxy for field `GPIO18INTD`"]
pub struct GPIO18INTD_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO18INTD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO18INTD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt on low to high GPIO transition"]
    #[inline(always)]
    pub fn intlh(self) -> &'a mut W {
        self.variant(GPIO18INTD_A::INTLH)
    }
    #[doc = "Interrupt on high to low GPIO transition"]
    #[inline(always)]
    pub fn inthl(self) -> &'a mut W {
        self.variant(GPIO18INTD_A::INTHL)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "GPIO18 output configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GPIO18OUTCFG_A {
    #[doc = "0: Output disabled"]
    DIS = 0,
    #[doc = "1: Output is push-pull"]
    PUSHPULL = 1,
    #[doc = "2: Output is open drain"]
    OD = 2,
    #[doc = "3: Output is tri-state"]
    TS = 3,
}
impl From<GPIO18OUTCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO18OUTCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `GPIO18OUTCFG`"]
pub type GPIO18OUTCFG_R = crate::R<u8, GPIO18OUTCFG_A>;
impl GPIO18OUTCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO18OUTCFG_A {
        match self.bits {
            0 => GPIO18OUTCFG_A::DIS,
            1 => GPIO18OUTCFG_A::PUSHPULL,
            2 => GPIO18OUTCFG_A::OD,
            3 => GPIO18OUTCFG_A::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == GPIO18OUTCFG_A::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == GPIO18OUTCFG_A::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        *self == GPIO18OUTCFG_A::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        *self == GPIO18OUTCFG_A::TS
    }
}
#[doc = "Write proxy for field `GPIO18OUTCFG`"]
pub struct GPIO18OUTCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO18OUTCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO18OUTCFG_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Output disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO18OUTCFG_A::DIS)
    }
    #[doc = "Output is push-pull"]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO18OUTCFG_A::PUSHPULL)
    }
    #[doc = "Output is open drain"]
    #[inline(always)]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO18OUTCFG_A::OD)
    }
    #[doc = "Output is tri-state"]
    #[inline(always)]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO18OUTCFG_A::TS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 9)) | (((value as u32) & 0x03) << 9);
        self.w
    }
}
#[doc = "GPIO18 input enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO18INCFG_A {
    #[doc = "0: Read the GPIO pin data"]
    READ = 0,
    #[doc = "1: Readback will always be zero"]
    RDZERO = 1,
}
impl From<GPIO18INCFG_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO18INCFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIO18INCFG`"]
pub type GPIO18INCFG_R = crate::R<bool, GPIO18INCFG_A>;
impl GPIO18INCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO18INCFG_A {
        match self.bits {
            false => GPIO18INCFG_A::READ,
            true => GPIO18INCFG_A::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        *self == GPIO18INCFG_A::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline(always)]
    pub fn is_rdzero(&self) -> bool {
        *self == GPIO18INCFG_A::RDZERO
    }
}
#[doc = "Write proxy for field `GPIO18INCFG`"]
pub struct GPIO18INCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO18INCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO18INCFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Read the GPIO pin data"]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO18INCFG_A::READ)
    }
    #[doc = "Readback will always be zero"]
    #[inline(always)]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO18INCFG_A::RDZERO)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "GPIO17 interrupt direction.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO17INTD_A {
    #[doc = "0: Interrupt on low to high GPIO transition"]
    INTLH = 0,
    #[doc = "1: Interrupt on high to low GPIO transition"]
    INTHL = 1,
}
impl From<GPIO17INTD_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO17INTD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIO17INTD`"]
pub type GPIO17INTD_R = crate::R<bool, GPIO17INTD_A>;
impl GPIO17INTD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO17INTD_A {
        match self.bits {
            false => GPIO17INTD_A::INTLH,
            true => GPIO17INTD_A::INTHL,
        }
    }
    #[doc = "Checks if the value of the field is `INTLH`"]
    #[inline(always)]
    pub fn is_intlh(&self) -> bool {
        *self == GPIO17INTD_A::INTLH
    }
    #[doc = "Checks if the value of the field is `INTHL`"]
    #[inline(always)]
    pub fn is_inthl(&self) -> bool {
        *self == GPIO17INTD_A::INTHL
    }
}
#[doc = "Write proxy for field `GPIO17INTD`"]
pub struct GPIO17INTD_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO17INTD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO17INTD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt on low to high GPIO transition"]
    #[inline(always)]
    pub fn intlh(self) -> &'a mut W {
        self.variant(GPIO17INTD_A::INTLH)
    }
    #[doc = "Interrupt on high to low GPIO transition"]
    #[inline(always)]
    pub fn inthl(self) -> &'a mut W {
        self.variant(GPIO17INTD_A::INTHL)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "GPIO17 output configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GPIO17OUTCFG_A {
    #[doc = "0: Output disabled"]
    DIS = 0,
    #[doc = "1: Output is push-pull"]
    PUSHPULL = 1,
    #[doc = "2: Output is open drain"]
    OD = 2,
    #[doc = "3: Output is tri-state"]
    TS = 3,
}
impl From<GPIO17OUTCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO17OUTCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `GPIO17OUTCFG`"]
pub type GPIO17OUTCFG_R = crate::R<u8, GPIO17OUTCFG_A>;
impl GPIO17OUTCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO17OUTCFG_A {
        match self.bits {
            0 => GPIO17OUTCFG_A::DIS,
            1 => GPIO17OUTCFG_A::PUSHPULL,
            2 => GPIO17OUTCFG_A::OD,
            3 => GPIO17OUTCFG_A::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == GPIO17OUTCFG_A::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == GPIO17OUTCFG_A::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        *self == GPIO17OUTCFG_A::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        *self == GPIO17OUTCFG_A::TS
    }
}
#[doc = "Write proxy for field `GPIO17OUTCFG`"]
pub struct GPIO17OUTCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO17OUTCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO17OUTCFG_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Output disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO17OUTCFG_A::DIS)
    }
    #[doc = "Output is push-pull"]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO17OUTCFG_A::PUSHPULL)
    }
    #[doc = "Output is open drain"]
    #[inline(always)]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO17OUTCFG_A::OD)
    }
    #[doc = "Output is tri-state"]
    #[inline(always)]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO17OUTCFG_A::TS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | (((value as u32) & 0x03) << 5);
        self.w
    }
}
#[doc = "GPIO17 input enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO17INCFG_A {
    #[doc = "0: Read the GPIO pin data"]
    READ = 0,
    #[doc = "1: Readback will always be zero"]
    RDZERO = 1,
}
impl From<GPIO17INCFG_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO17INCFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIO17INCFG`"]
pub type GPIO17INCFG_R = crate::R<bool, GPIO17INCFG_A>;
impl GPIO17INCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO17INCFG_A {
        match self.bits {
            false => GPIO17INCFG_A::READ,
            true => GPIO17INCFG_A::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        *self == GPIO17INCFG_A::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline(always)]
    pub fn is_rdzero(&self) -> bool {
        *self == GPIO17INCFG_A::RDZERO
    }
}
#[doc = "Write proxy for field `GPIO17INCFG`"]
pub struct GPIO17INCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO17INCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO17INCFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Read the GPIO pin data"]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO17INCFG_A::READ)
    }
    #[doc = "Readback will always be zero"]
    #[inline(always)]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO17INCFG_A::RDZERO)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "GPIO16 interrupt direction.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO16INTD_A {
    #[doc = "0: Interrupt on low to high GPIO transition"]
    INTLH = 0,
    #[doc = "1: Interrupt on high to low GPIO transition"]
    INTHL = 1,
}
impl From<GPIO16INTD_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO16INTD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIO16INTD`"]
pub type GPIO16INTD_R = crate::R<bool, GPIO16INTD_A>;
impl GPIO16INTD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO16INTD_A {
        match self.bits {
            false => GPIO16INTD_A::INTLH,
            true => GPIO16INTD_A::INTHL,
        }
    }
    #[doc = "Checks if the value of the field is `INTLH`"]
    #[inline(always)]
    pub fn is_intlh(&self) -> bool {
        *self == GPIO16INTD_A::INTLH
    }
    #[doc = "Checks if the value of the field is `INTHL`"]
    #[inline(always)]
    pub fn is_inthl(&self) -> bool {
        *self == GPIO16INTD_A::INTHL
    }
}
#[doc = "Write proxy for field `GPIO16INTD`"]
pub struct GPIO16INTD_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO16INTD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO16INTD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt on low to high GPIO transition"]
    #[inline(always)]
    pub fn intlh(self) -> &'a mut W {
        self.variant(GPIO16INTD_A::INTLH)
    }
    #[doc = "Interrupt on high to low GPIO transition"]
    #[inline(always)]
    pub fn inthl(self) -> &'a mut W {
        self.variant(GPIO16INTD_A::INTHL)
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
#[doc = "GPIO16 output configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GPIO16OUTCFG_A {
    #[doc = "0: Output disabled"]
    DIS = 0,
    #[doc = "1: Output is push-pull"]
    PUSHPULL = 1,
    #[doc = "2: Output is open drain"]
    OD = 2,
    #[doc = "3: Output is tri-state"]
    TS = 3,
}
impl From<GPIO16OUTCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO16OUTCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `GPIO16OUTCFG`"]
pub type GPIO16OUTCFG_R = crate::R<u8, GPIO16OUTCFG_A>;
impl GPIO16OUTCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO16OUTCFG_A {
        match self.bits {
            0 => GPIO16OUTCFG_A::DIS,
            1 => GPIO16OUTCFG_A::PUSHPULL,
            2 => GPIO16OUTCFG_A::OD,
            3 => GPIO16OUTCFG_A::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == GPIO16OUTCFG_A::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == GPIO16OUTCFG_A::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        *self == GPIO16OUTCFG_A::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        *self == GPIO16OUTCFG_A::TS
    }
}
#[doc = "Write proxy for field `GPIO16OUTCFG`"]
pub struct GPIO16OUTCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO16OUTCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO16OUTCFG_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Output disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO16OUTCFG_A::DIS)
    }
    #[doc = "Output is push-pull"]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO16OUTCFG_A::PUSHPULL)
    }
    #[doc = "Output is open drain"]
    #[inline(always)]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO16OUTCFG_A::OD)
    }
    #[doc = "Output is tri-state"]
    #[inline(always)]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO16OUTCFG_A::TS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | (((value as u32) & 0x03) << 1);
        self.w
    }
}
#[doc = "GPIO16 input enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO16INCFG_A {
    #[doc = "0: Read the GPIO pin data"]
    READ = 0,
    #[doc = "1: Readback will always be zero"]
    RDZERO = 1,
}
impl From<GPIO16INCFG_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO16INCFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIO16INCFG`"]
pub type GPIO16INCFG_R = crate::R<bool, GPIO16INCFG_A>;
impl GPIO16INCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO16INCFG_A {
        match self.bits {
            false => GPIO16INCFG_A::READ,
            true => GPIO16INCFG_A::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        *self == GPIO16INCFG_A::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline(always)]
    pub fn is_rdzero(&self) -> bool {
        *self == GPIO16INCFG_A::RDZERO
    }
}
#[doc = "Write proxy for field `GPIO16INCFG`"]
pub struct GPIO16INCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO16INCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO16INCFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Read the GPIO pin data"]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO16INCFG_A::READ)
    }
    #[doc = "Readback will always be zero"]
    #[inline(always)]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO16INCFG_A::RDZERO)
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
    #[doc = "Bit 31 - GPIO23 interrupt direction."]
    #[inline(always)]
    pub fn gpio23intd(&self) -> GPIO23INTD_R {
        GPIO23INTD_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 29:30 - GPIO23 output configuration."]
    #[inline(always)]
    pub fn gpio23outcfg(&self) -> GPIO23OUTCFG_R {
        GPIO23OUTCFG_R::new(((self.bits >> 29) & 0x03) as u8)
    }
    #[doc = "Bit 28 - GPIO23 input enable."]
    #[inline(always)]
    pub fn gpio23incfg(&self) -> GPIO23INCFG_R {
        GPIO23INCFG_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - GPIO22 interrupt direction."]
    #[inline(always)]
    pub fn gpio22intd(&self) -> GPIO22INTD_R {
        GPIO22INTD_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bits 25:26 - GPIO22 output configuration."]
    #[inline(always)]
    pub fn gpio22outcfg(&self) -> GPIO22OUTCFG_R {
        GPIO22OUTCFG_R::new(((self.bits >> 25) & 0x03) as u8)
    }
    #[doc = "Bit 24 - GPIO22 input enable."]
    #[inline(always)]
    pub fn gpio22incfg(&self) -> GPIO22INCFG_R {
        GPIO22INCFG_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - GPIO21 interrupt direction."]
    #[inline(always)]
    pub fn gpio21intd(&self) -> GPIO21INTD_R {
        GPIO21INTD_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 21:22 - GPIO21 output configuration."]
    #[inline(always)]
    pub fn gpio21outcfg(&self) -> GPIO21OUTCFG_R {
        GPIO21OUTCFG_R::new(((self.bits >> 21) & 0x03) as u8)
    }
    #[doc = "Bit 20 - GPIO21 input enable."]
    #[inline(always)]
    pub fn gpio21incfg(&self) -> GPIO21INCFG_R {
        GPIO21INCFG_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - GPIO20 interrupt direction."]
    #[inline(always)]
    pub fn gpio20intd(&self) -> GPIO20INTD_R {
        GPIO20INTD_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bits 17:18 - GPIO20 output configuration."]
    #[inline(always)]
    pub fn gpio20outcfg(&self) -> GPIO20OUTCFG_R {
        GPIO20OUTCFG_R::new(((self.bits >> 17) & 0x03) as u8)
    }
    #[doc = "Bit 16 - GPIO20 input enable."]
    #[inline(always)]
    pub fn gpio20incfg(&self) -> GPIO20INCFG_R {
        GPIO20INCFG_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - GPIO19 interrupt direction."]
    #[inline(always)]
    pub fn gpio19intd(&self) -> GPIO19INTD_R {
        GPIO19INTD_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 13:14 - GPIO19 output configuration."]
    #[inline(always)]
    pub fn gpio19outcfg(&self) -> GPIO19OUTCFG_R {
        GPIO19OUTCFG_R::new(((self.bits >> 13) & 0x03) as u8)
    }
    #[doc = "Bit 12 - GPIO19 input enable."]
    #[inline(always)]
    pub fn gpio19incfg(&self) -> GPIO19INCFG_R {
        GPIO19INCFG_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - GPIO18 interrupt direction."]
    #[inline(always)]
    pub fn gpio18intd(&self) -> GPIO18INTD_R {
        GPIO18INTD_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 9:10 - GPIO18 output configuration."]
    #[inline(always)]
    pub fn gpio18outcfg(&self) -> GPIO18OUTCFG_R {
        GPIO18OUTCFG_R::new(((self.bits >> 9) & 0x03) as u8)
    }
    #[doc = "Bit 8 - GPIO18 input enable."]
    #[inline(always)]
    pub fn gpio18incfg(&self) -> GPIO18INCFG_R {
        GPIO18INCFG_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - GPIO17 interrupt direction."]
    #[inline(always)]
    pub fn gpio17intd(&self) -> GPIO17INTD_R {
        GPIO17INTD_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 5:6 - GPIO17 output configuration."]
    #[inline(always)]
    pub fn gpio17outcfg(&self) -> GPIO17OUTCFG_R {
        GPIO17OUTCFG_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bit 4 - GPIO17 input enable."]
    #[inline(always)]
    pub fn gpio17incfg(&self) -> GPIO17INCFG_R {
        GPIO17INCFG_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - GPIO16 interrupt direction."]
    #[inline(always)]
    pub fn gpio16intd(&self) -> GPIO16INTD_R {
        GPIO16INTD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - GPIO16 output configuration."]
    #[inline(always)]
    pub fn gpio16outcfg(&self) -> GPIO16OUTCFG_R {
        GPIO16OUTCFG_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bit 0 - GPIO16 input enable."]
    #[inline(always)]
    pub fn gpio16incfg(&self) -> GPIO16INCFG_R {
        GPIO16INCFG_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - GPIO23 interrupt direction."]
    #[inline(always)]
    pub fn gpio23intd(&mut self) -> GPIO23INTD_W {
        GPIO23INTD_W { w: self }
    }
    #[doc = "Bits 29:30 - GPIO23 output configuration."]
    #[inline(always)]
    pub fn gpio23outcfg(&mut self) -> GPIO23OUTCFG_W {
        GPIO23OUTCFG_W { w: self }
    }
    #[doc = "Bit 28 - GPIO23 input enable."]
    #[inline(always)]
    pub fn gpio23incfg(&mut self) -> GPIO23INCFG_W {
        GPIO23INCFG_W { w: self }
    }
    #[doc = "Bit 27 - GPIO22 interrupt direction."]
    #[inline(always)]
    pub fn gpio22intd(&mut self) -> GPIO22INTD_W {
        GPIO22INTD_W { w: self }
    }
    #[doc = "Bits 25:26 - GPIO22 output configuration."]
    #[inline(always)]
    pub fn gpio22outcfg(&mut self) -> GPIO22OUTCFG_W {
        GPIO22OUTCFG_W { w: self }
    }
    #[doc = "Bit 24 - GPIO22 input enable."]
    #[inline(always)]
    pub fn gpio22incfg(&mut self) -> GPIO22INCFG_W {
        GPIO22INCFG_W { w: self }
    }
    #[doc = "Bit 23 - GPIO21 interrupt direction."]
    #[inline(always)]
    pub fn gpio21intd(&mut self) -> GPIO21INTD_W {
        GPIO21INTD_W { w: self }
    }
    #[doc = "Bits 21:22 - GPIO21 output configuration."]
    #[inline(always)]
    pub fn gpio21outcfg(&mut self) -> GPIO21OUTCFG_W {
        GPIO21OUTCFG_W { w: self }
    }
    #[doc = "Bit 20 - GPIO21 input enable."]
    #[inline(always)]
    pub fn gpio21incfg(&mut self) -> GPIO21INCFG_W {
        GPIO21INCFG_W { w: self }
    }
    #[doc = "Bit 19 - GPIO20 interrupt direction."]
    #[inline(always)]
    pub fn gpio20intd(&mut self) -> GPIO20INTD_W {
        GPIO20INTD_W { w: self }
    }
    #[doc = "Bits 17:18 - GPIO20 output configuration."]
    #[inline(always)]
    pub fn gpio20outcfg(&mut self) -> GPIO20OUTCFG_W {
        GPIO20OUTCFG_W { w: self }
    }
    #[doc = "Bit 16 - GPIO20 input enable."]
    #[inline(always)]
    pub fn gpio20incfg(&mut self) -> GPIO20INCFG_W {
        GPIO20INCFG_W { w: self }
    }
    #[doc = "Bit 15 - GPIO19 interrupt direction."]
    #[inline(always)]
    pub fn gpio19intd(&mut self) -> GPIO19INTD_W {
        GPIO19INTD_W { w: self }
    }
    #[doc = "Bits 13:14 - GPIO19 output configuration."]
    #[inline(always)]
    pub fn gpio19outcfg(&mut self) -> GPIO19OUTCFG_W {
        GPIO19OUTCFG_W { w: self }
    }
    #[doc = "Bit 12 - GPIO19 input enable."]
    #[inline(always)]
    pub fn gpio19incfg(&mut self) -> GPIO19INCFG_W {
        GPIO19INCFG_W { w: self }
    }
    #[doc = "Bit 11 - GPIO18 interrupt direction."]
    #[inline(always)]
    pub fn gpio18intd(&mut self) -> GPIO18INTD_W {
        GPIO18INTD_W { w: self }
    }
    #[doc = "Bits 9:10 - GPIO18 output configuration."]
    #[inline(always)]
    pub fn gpio18outcfg(&mut self) -> GPIO18OUTCFG_W {
        GPIO18OUTCFG_W { w: self }
    }
    #[doc = "Bit 8 - GPIO18 input enable."]
    #[inline(always)]
    pub fn gpio18incfg(&mut self) -> GPIO18INCFG_W {
        GPIO18INCFG_W { w: self }
    }
    #[doc = "Bit 7 - GPIO17 interrupt direction."]
    #[inline(always)]
    pub fn gpio17intd(&mut self) -> GPIO17INTD_W {
        GPIO17INTD_W { w: self }
    }
    #[doc = "Bits 5:6 - GPIO17 output configuration."]
    #[inline(always)]
    pub fn gpio17outcfg(&mut self) -> GPIO17OUTCFG_W {
        GPIO17OUTCFG_W { w: self }
    }
    #[doc = "Bit 4 - GPIO17 input enable."]
    #[inline(always)]
    pub fn gpio17incfg(&mut self) -> GPIO17INCFG_W {
        GPIO17INCFG_W { w: self }
    }
    #[doc = "Bit 3 - GPIO16 interrupt direction."]
    #[inline(always)]
    pub fn gpio16intd(&mut self) -> GPIO16INTD_W {
        GPIO16INTD_W { w: self }
    }
    #[doc = "Bits 1:2 - GPIO16 output configuration."]
    #[inline(always)]
    pub fn gpio16outcfg(&mut self) -> GPIO16OUTCFG_W {
        GPIO16OUTCFG_W { w: self }
    }
    #[doc = "Bit 0 - GPIO16 input enable."]
    #[inline(always)]
    pub fn gpio16incfg(&mut self) -> GPIO16INCFG_W {
        GPIO16INCFG_W { w: self }
    }
}
