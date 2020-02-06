#[doc = "Reader of register CFGB"]
pub type R = crate::R<u32, super::CFGB>;
#[doc = "Writer for register CFGB"]
pub type W = crate::W<u32, super::CFGB>;
#[doc = "Register CFGB `reset()`'s with value 0"]
impl crate::ResetValue for super::CFGB {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "GPIO15 interrupt direction.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO15INTD_A {
    #[doc = "0: Interrupt on low to high GPIO transition"]
    INTLH = 0,
    #[doc = "1: Interrupt on high to low GPIO transition"]
    INTHL = 1,
}
impl From<GPIO15INTD_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO15INTD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIO15INTD`"]
pub type GPIO15INTD_R = crate::R<bool, GPIO15INTD_A>;
impl GPIO15INTD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO15INTD_A {
        match self.bits {
            false => GPIO15INTD_A::INTLH,
            true => GPIO15INTD_A::INTHL,
        }
    }
    #[doc = "Checks if the value of the field is `INTLH`"]
    #[inline(always)]
    pub fn is_intlh(&self) -> bool {
        *self == GPIO15INTD_A::INTLH
    }
    #[doc = "Checks if the value of the field is `INTHL`"]
    #[inline(always)]
    pub fn is_inthl(&self) -> bool {
        *self == GPIO15INTD_A::INTHL
    }
}
#[doc = "Write proxy for field `GPIO15INTD`"]
pub struct GPIO15INTD_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO15INTD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO15INTD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt on low to high GPIO transition"]
    #[inline(always)]
    pub fn intlh(self) -> &'a mut W {
        self.variant(GPIO15INTD_A::INTLH)
    }
    #[doc = "Interrupt on high to low GPIO transition"]
    #[inline(always)]
    pub fn inthl(self) -> &'a mut W {
        self.variant(GPIO15INTD_A::INTHL)
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
#[doc = "GPIO15 output configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GPIO15OUTCFG_A {
    #[doc = "0: Output disabled"]
    DIS = 0,
    #[doc = "1: Output is push-pull"]
    PUSHPULL = 1,
    #[doc = "2: Output is open drain"]
    OD = 2,
    #[doc = "3: Output is tri-state"]
    TS = 3,
}
impl From<GPIO15OUTCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO15OUTCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `GPIO15OUTCFG`"]
pub type GPIO15OUTCFG_R = crate::R<u8, GPIO15OUTCFG_A>;
impl GPIO15OUTCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO15OUTCFG_A {
        match self.bits {
            0 => GPIO15OUTCFG_A::DIS,
            1 => GPIO15OUTCFG_A::PUSHPULL,
            2 => GPIO15OUTCFG_A::OD,
            3 => GPIO15OUTCFG_A::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == GPIO15OUTCFG_A::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == GPIO15OUTCFG_A::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        *self == GPIO15OUTCFG_A::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        *self == GPIO15OUTCFG_A::TS
    }
}
#[doc = "Write proxy for field `GPIO15OUTCFG`"]
pub struct GPIO15OUTCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO15OUTCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO15OUTCFG_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Output disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO15OUTCFG_A::DIS)
    }
    #[doc = "Output is push-pull"]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO15OUTCFG_A::PUSHPULL)
    }
    #[doc = "Output is open drain"]
    #[inline(always)]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO15OUTCFG_A::OD)
    }
    #[doc = "Output is tri-state"]
    #[inline(always)]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO15OUTCFG_A::TS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 29)) | (((value as u32) & 0x03) << 29);
        self.w
    }
}
#[doc = "GPIO15 input enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO15INCFG_A {
    #[doc = "0: Read the GPIO pin data"]
    READ = 0,
    #[doc = "1: Readback will always be zero"]
    RDZERO = 1,
}
impl From<GPIO15INCFG_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO15INCFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIO15INCFG`"]
pub type GPIO15INCFG_R = crate::R<bool, GPIO15INCFG_A>;
impl GPIO15INCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO15INCFG_A {
        match self.bits {
            false => GPIO15INCFG_A::READ,
            true => GPIO15INCFG_A::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        *self == GPIO15INCFG_A::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline(always)]
    pub fn is_rdzero(&self) -> bool {
        *self == GPIO15INCFG_A::RDZERO
    }
}
#[doc = "Write proxy for field `GPIO15INCFG`"]
pub struct GPIO15INCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO15INCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO15INCFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Read the GPIO pin data"]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO15INCFG_A::READ)
    }
    #[doc = "Readback will always be zero"]
    #[inline(always)]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO15INCFG_A::RDZERO)
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
#[doc = "GPIO14 interrupt direction.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO14INTD_A {
    #[doc = "0: Interrupt on low to high GPIO transition"]
    INTLH = 0,
    #[doc = "1: Interrupt on high to low GPIO transition"]
    INTHL = 1,
}
impl From<GPIO14INTD_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO14INTD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIO14INTD`"]
pub type GPIO14INTD_R = crate::R<bool, GPIO14INTD_A>;
impl GPIO14INTD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO14INTD_A {
        match self.bits {
            false => GPIO14INTD_A::INTLH,
            true => GPIO14INTD_A::INTHL,
        }
    }
    #[doc = "Checks if the value of the field is `INTLH`"]
    #[inline(always)]
    pub fn is_intlh(&self) -> bool {
        *self == GPIO14INTD_A::INTLH
    }
    #[doc = "Checks if the value of the field is `INTHL`"]
    #[inline(always)]
    pub fn is_inthl(&self) -> bool {
        *self == GPIO14INTD_A::INTHL
    }
}
#[doc = "Write proxy for field `GPIO14INTD`"]
pub struct GPIO14INTD_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO14INTD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO14INTD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt on low to high GPIO transition"]
    #[inline(always)]
    pub fn intlh(self) -> &'a mut W {
        self.variant(GPIO14INTD_A::INTLH)
    }
    #[doc = "Interrupt on high to low GPIO transition"]
    #[inline(always)]
    pub fn inthl(self) -> &'a mut W {
        self.variant(GPIO14INTD_A::INTHL)
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
#[doc = "GPIO14 output configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GPIO14OUTCFG_A {
    #[doc = "0: Output disabled"]
    DIS = 0,
    #[doc = "1: Output is push-pull"]
    PUSHPULL = 1,
    #[doc = "2: Output is open drain"]
    OD = 2,
    #[doc = "3: Output is tri-state"]
    TS = 3,
}
impl From<GPIO14OUTCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO14OUTCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `GPIO14OUTCFG`"]
pub type GPIO14OUTCFG_R = crate::R<u8, GPIO14OUTCFG_A>;
impl GPIO14OUTCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO14OUTCFG_A {
        match self.bits {
            0 => GPIO14OUTCFG_A::DIS,
            1 => GPIO14OUTCFG_A::PUSHPULL,
            2 => GPIO14OUTCFG_A::OD,
            3 => GPIO14OUTCFG_A::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == GPIO14OUTCFG_A::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == GPIO14OUTCFG_A::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        *self == GPIO14OUTCFG_A::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        *self == GPIO14OUTCFG_A::TS
    }
}
#[doc = "Write proxy for field `GPIO14OUTCFG`"]
pub struct GPIO14OUTCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO14OUTCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO14OUTCFG_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Output disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO14OUTCFG_A::DIS)
    }
    #[doc = "Output is push-pull"]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO14OUTCFG_A::PUSHPULL)
    }
    #[doc = "Output is open drain"]
    #[inline(always)]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO14OUTCFG_A::OD)
    }
    #[doc = "Output is tri-state"]
    #[inline(always)]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO14OUTCFG_A::TS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 25)) | (((value as u32) & 0x03) << 25);
        self.w
    }
}
#[doc = "GPIO14 input enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO14INCFG_A {
    #[doc = "0: Read the GPIO pin data"]
    READ = 0,
    #[doc = "1: Readback will always be zero"]
    RDZERO = 1,
}
impl From<GPIO14INCFG_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO14INCFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIO14INCFG`"]
pub type GPIO14INCFG_R = crate::R<bool, GPIO14INCFG_A>;
impl GPIO14INCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO14INCFG_A {
        match self.bits {
            false => GPIO14INCFG_A::READ,
            true => GPIO14INCFG_A::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        *self == GPIO14INCFG_A::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline(always)]
    pub fn is_rdzero(&self) -> bool {
        *self == GPIO14INCFG_A::RDZERO
    }
}
#[doc = "Write proxy for field `GPIO14INCFG`"]
pub struct GPIO14INCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO14INCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO14INCFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Read the GPIO pin data"]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO14INCFG_A::READ)
    }
    #[doc = "Readback will always be zero"]
    #[inline(always)]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO14INCFG_A::RDZERO)
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
#[doc = "GPIO13 interrupt direction.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO13INTD_A {
    #[doc = "0: Interrupt on low to high GPIO transition"]
    INTLH = 0,
    #[doc = "1: Interrupt on high to low GPIO transition"]
    INTHL = 1,
}
impl From<GPIO13INTD_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO13INTD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIO13INTD`"]
pub type GPIO13INTD_R = crate::R<bool, GPIO13INTD_A>;
impl GPIO13INTD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO13INTD_A {
        match self.bits {
            false => GPIO13INTD_A::INTLH,
            true => GPIO13INTD_A::INTHL,
        }
    }
    #[doc = "Checks if the value of the field is `INTLH`"]
    #[inline(always)]
    pub fn is_intlh(&self) -> bool {
        *self == GPIO13INTD_A::INTLH
    }
    #[doc = "Checks if the value of the field is `INTHL`"]
    #[inline(always)]
    pub fn is_inthl(&self) -> bool {
        *self == GPIO13INTD_A::INTHL
    }
}
#[doc = "Write proxy for field `GPIO13INTD`"]
pub struct GPIO13INTD_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO13INTD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO13INTD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt on low to high GPIO transition"]
    #[inline(always)]
    pub fn intlh(self) -> &'a mut W {
        self.variant(GPIO13INTD_A::INTLH)
    }
    #[doc = "Interrupt on high to low GPIO transition"]
    #[inline(always)]
    pub fn inthl(self) -> &'a mut W {
        self.variant(GPIO13INTD_A::INTHL)
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
#[doc = "GPIO13 output configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GPIO13OUTCFG_A {
    #[doc = "0: Output disabled"]
    DIS = 0,
    #[doc = "1: Output is push-pull"]
    PUSHPULL = 1,
    #[doc = "2: Output is open drain"]
    OD = 2,
    #[doc = "3: Output is tri-state"]
    TS = 3,
}
impl From<GPIO13OUTCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO13OUTCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `GPIO13OUTCFG`"]
pub type GPIO13OUTCFG_R = crate::R<u8, GPIO13OUTCFG_A>;
impl GPIO13OUTCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO13OUTCFG_A {
        match self.bits {
            0 => GPIO13OUTCFG_A::DIS,
            1 => GPIO13OUTCFG_A::PUSHPULL,
            2 => GPIO13OUTCFG_A::OD,
            3 => GPIO13OUTCFG_A::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == GPIO13OUTCFG_A::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == GPIO13OUTCFG_A::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        *self == GPIO13OUTCFG_A::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        *self == GPIO13OUTCFG_A::TS
    }
}
#[doc = "Write proxy for field `GPIO13OUTCFG`"]
pub struct GPIO13OUTCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO13OUTCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO13OUTCFG_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Output disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO13OUTCFG_A::DIS)
    }
    #[doc = "Output is push-pull"]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO13OUTCFG_A::PUSHPULL)
    }
    #[doc = "Output is open drain"]
    #[inline(always)]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO13OUTCFG_A::OD)
    }
    #[doc = "Output is tri-state"]
    #[inline(always)]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO13OUTCFG_A::TS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 21)) | (((value as u32) & 0x03) << 21);
        self.w
    }
}
#[doc = "GPIO13 input enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO13INCFG_A {
    #[doc = "0: Read the GPIO pin data"]
    READ = 0,
    #[doc = "1: Readback will always be zero"]
    RDZERO = 1,
}
impl From<GPIO13INCFG_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO13INCFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIO13INCFG`"]
pub type GPIO13INCFG_R = crate::R<bool, GPIO13INCFG_A>;
impl GPIO13INCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO13INCFG_A {
        match self.bits {
            false => GPIO13INCFG_A::READ,
            true => GPIO13INCFG_A::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        *self == GPIO13INCFG_A::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline(always)]
    pub fn is_rdzero(&self) -> bool {
        *self == GPIO13INCFG_A::RDZERO
    }
}
#[doc = "Write proxy for field `GPIO13INCFG`"]
pub struct GPIO13INCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO13INCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO13INCFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Read the GPIO pin data"]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO13INCFG_A::READ)
    }
    #[doc = "Readback will always be zero"]
    #[inline(always)]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO13INCFG_A::RDZERO)
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
#[doc = "GPIO12 interrupt direction.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO12INTD_A {
    #[doc = "0: Interrupt on low to high GPIO transition"]
    INTLH = 0,
    #[doc = "1: Interrupt on high to low GPIO transition"]
    INTHL = 1,
}
impl From<GPIO12INTD_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO12INTD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIO12INTD`"]
pub type GPIO12INTD_R = crate::R<bool, GPIO12INTD_A>;
impl GPIO12INTD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO12INTD_A {
        match self.bits {
            false => GPIO12INTD_A::INTLH,
            true => GPIO12INTD_A::INTHL,
        }
    }
    #[doc = "Checks if the value of the field is `INTLH`"]
    #[inline(always)]
    pub fn is_intlh(&self) -> bool {
        *self == GPIO12INTD_A::INTLH
    }
    #[doc = "Checks if the value of the field is `INTHL`"]
    #[inline(always)]
    pub fn is_inthl(&self) -> bool {
        *self == GPIO12INTD_A::INTHL
    }
}
#[doc = "Write proxy for field `GPIO12INTD`"]
pub struct GPIO12INTD_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO12INTD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO12INTD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt on low to high GPIO transition"]
    #[inline(always)]
    pub fn intlh(self) -> &'a mut W {
        self.variant(GPIO12INTD_A::INTLH)
    }
    #[doc = "Interrupt on high to low GPIO transition"]
    #[inline(always)]
    pub fn inthl(self) -> &'a mut W {
        self.variant(GPIO12INTD_A::INTHL)
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
#[doc = "GPIO12 output configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GPIO12OUTCFG_A {
    #[doc = "0: Output disabled"]
    DIS = 0,
    #[doc = "1: Output is push-pull"]
    PUSHPULL = 1,
    #[doc = "2: Output is open drain"]
    OD = 2,
    #[doc = "3: Output is tri-state"]
    TS = 3,
}
impl From<GPIO12OUTCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO12OUTCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `GPIO12OUTCFG`"]
pub type GPIO12OUTCFG_R = crate::R<u8, GPIO12OUTCFG_A>;
impl GPIO12OUTCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO12OUTCFG_A {
        match self.bits {
            0 => GPIO12OUTCFG_A::DIS,
            1 => GPIO12OUTCFG_A::PUSHPULL,
            2 => GPIO12OUTCFG_A::OD,
            3 => GPIO12OUTCFG_A::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == GPIO12OUTCFG_A::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == GPIO12OUTCFG_A::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        *self == GPIO12OUTCFG_A::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        *self == GPIO12OUTCFG_A::TS
    }
}
#[doc = "Write proxy for field `GPIO12OUTCFG`"]
pub struct GPIO12OUTCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO12OUTCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO12OUTCFG_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Output disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO12OUTCFG_A::DIS)
    }
    #[doc = "Output is push-pull"]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO12OUTCFG_A::PUSHPULL)
    }
    #[doc = "Output is open drain"]
    #[inline(always)]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO12OUTCFG_A::OD)
    }
    #[doc = "Output is tri-state"]
    #[inline(always)]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO12OUTCFG_A::TS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 17)) | (((value as u32) & 0x03) << 17);
        self.w
    }
}
#[doc = "GPIO12 input enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO12INCFG_A {
    #[doc = "0: Read the GPIO pin data"]
    READ = 0,
    #[doc = "1: Readback will always be zero"]
    RDZERO = 1,
}
impl From<GPIO12INCFG_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO12INCFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIO12INCFG`"]
pub type GPIO12INCFG_R = crate::R<bool, GPIO12INCFG_A>;
impl GPIO12INCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO12INCFG_A {
        match self.bits {
            false => GPIO12INCFG_A::READ,
            true => GPIO12INCFG_A::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        *self == GPIO12INCFG_A::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline(always)]
    pub fn is_rdzero(&self) -> bool {
        *self == GPIO12INCFG_A::RDZERO
    }
}
#[doc = "Write proxy for field `GPIO12INCFG`"]
pub struct GPIO12INCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO12INCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO12INCFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Read the GPIO pin data"]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO12INCFG_A::READ)
    }
    #[doc = "Readback will always be zero"]
    #[inline(always)]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO12INCFG_A::RDZERO)
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
#[doc = "GPIO11 interrupt direction.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO11INTD_A {
    #[doc = "0: Interrupt on low to high GPIO transition"]
    INTLH = 0,
    #[doc = "1: Interrupt on high to low GPIO transition"]
    INTHL = 1,
}
impl From<GPIO11INTD_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO11INTD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIO11INTD`"]
pub type GPIO11INTD_R = crate::R<bool, GPIO11INTD_A>;
impl GPIO11INTD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO11INTD_A {
        match self.bits {
            false => GPIO11INTD_A::INTLH,
            true => GPIO11INTD_A::INTHL,
        }
    }
    #[doc = "Checks if the value of the field is `INTLH`"]
    #[inline(always)]
    pub fn is_intlh(&self) -> bool {
        *self == GPIO11INTD_A::INTLH
    }
    #[doc = "Checks if the value of the field is `INTHL`"]
    #[inline(always)]
    pub fn is_inthl(&self) -> bool {
        *self == GPIO11INTD_A::INTHL
    }
}
#[doc = "Write proxy for field `GPIO11INTD`"]
pub struct GPIO11INTD_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO11INTD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO11INTD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt on low to high GPIO transition"]
    #[inline(always)]
    pub fn intlh(self) -> &'a mut W {
        self.variant(GPIO11INTD_A::INTLH)
    }
    #[doc = "Interrupt on high to low GPIO transition"]
    #[inline(always)]
    pub fn inthl(self) -> &'a mut W {
        self.variant(GPIO11INTD_A::INTHL)
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
#[doc = "GPIO11 output configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GPIO11OUTCFG_A {
    #[doc = "0: Output disabled"]
    DIS = 0,
    #[doc = "1: Output is push-pull"]
    PUSHPULL = 1,
    #[doc = "2: Output is open drain"]
    OD = 2,
    #[doc = "3: Output is tri-state"]
    TS = 3,
}
impl From<GPIO11OUTCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO11OUTCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `GPIO11OUTCFG`"]
pub type GPIO11OUTCFG_R = crate::R<u8, GPIO11OUTCFG_A>;
impl GPIO11OUTCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO11OUTCFG_A {
        match self.bits {
            0 => GPIO11OUTCFG_A::DIS,
            1 => GPIO11OUTCFG_A::PUSHPULL,
            2 => GPIO11OUTCFG_A::OD,
            3 => GPIO11OUTCFG_A::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == GPIO11OUTCFG_A::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == GPIO11OUTCFG_A::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        *self == GPIO11OUTCFG_A::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        *self == GPIO11OUTCFG_A::TS
    }
}
#[doc = "Write proxy for field `GPIO11OUTCFG`"]
pub struct GPIO11OUTCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO11OUTCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO11OUTCFG_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Output disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO11OUTCFG_A::DIS)
    }
    #[doc = "Output is push-pull"]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO11OUTCFG_A::PUSHPULL)
    }
    #[doc = "Output is open drain"]
    #[inline(always)]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO11OUTCFG_A::OD)
    }
    #[doc = "Output is tri-state"]
    #[inline(always)]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO11OUTCFG_A::TS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 13)) | (((value as u32) & 0x03) << 13);
        self.w
    }
}
#[doc = "GPIO11 input enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO11INCFG_A {
    #[doc = "0: Read the GPIO pin data"]
    READ = 0,
    #[doc = "1: Readback will always be zero"]
    RDZERO = 1,
}
impl From<GPIO11INCFG_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO11INCFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIO11INCFG`"]
pub type GPIO11INCFG_R = crate::R<bool, GPIO11INCFG_A>;
impl GPIO11INCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO11INCFG_A {
        match self.bits {
            false => GPIO11INCFG_A::READ,
            true => GPIO11INCFG_A::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        *self == GPIO11INCFG_A::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline(always)]
    pub fn is_rdzero(&self) -> bool {
        *self == GPIO11INCFG_A::RDZERO
    }
}
#[doc = "Write proxy for field `GPIO11INCFG`"]
pub struct GPIO11INCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO11INCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO11INCFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Read the GPIO pin data"]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO11INCFG_A::READ)
    }
    #[doc = "Readback will always be zero"]
    #[inline(always)]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO11INCFG_A::RDZERO)
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
#[doc = "GPIO10 interrupt direction.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO10INTD_A {
    #[doc = "0: Interrupt on low to high GPIO transition"]
    INTLH = 0,
    #[doc = "1: Interrupt on high to low GPIO transition"]
    INTHL = 1,
}
impl From<GPIO10INTD_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO10INTD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIO10INTD`"]
pub type GPIO10INTD_R = crate::R<bool, GPIO10INTD_A>;
impl GPIO10INTD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO10INTD_A {
        match self.bits {
            false => GPIO10INTD_A::INTLH,
            true => GPIO10INTD_A::INTHL,
        }
    }
    #[doc = "Checks if the value of the field is `INTLH`"]
    #[inline(always)]
    pub fn is_intlh(&self) -> bool {
        *self == GPIO10INTD_A::INTLH
    }
    #[doc = "Checks if the value of the field is `INTHL`"]
    #[inline(always)]
    pub fn is_inthl(&self) -> bool {
        *self == GPIO10INTD_A::INTHL
    }
}
#[doc = "Write proxy for field `GPIO10INTD`"]
pub struct GPIO10INTD_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO10INTD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO10INTD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt on low to high GPIO transition"]
    #[inline(always)]
    pub fn intlh(self) -> &'a mut W {
        self.variant(GPIO10INTD_A::INTLH)
    }
    #[doc = "Interrupt on high to low GPIO transition"]
    #[inline(always)]
    pub fn inthl(self) -> &'a mut W {
        self.variant(GPIO10INTD_A::INTHL)
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
#[doc = "GPIO10 output configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GPIO10OUTCFG_A {
    #[doc = "0: Output disabled"]
    DIS = 0,
    #[doc = "1: Output is push-pull"]
    PUSHPULL = 1,
    #[doc = "2: Output is open drain"]
    OD = 2,
    #[doc = "3: Output is tri-state"]
    TS = 3,
}
impl From<GPIO10OUTCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO10OUTCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `GPIO10OUTCFG`"]
pub type GPIO10OUTCFG_R = crate::R<u8, GPIO10OUTCFG_A>;
impl GPIO10OUTCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO10OUTCFG_A {
        match self.bits {
            0 => GPIO10OUTCFG_A::DIS,
            1 => GPIO10OUTCFG_A::PUSHPULL,
            2 => GPIO10OUTCFG_A::OD,
            3 => GPIO10OUTCFG_A::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == GPIO10OUTCFG_A::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == GPIO10OUTCFG_A::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        *self == GPIO10OUTCFG_A::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        *self == GPIO10OUTCFG_A::TS
    }
}
#[doc = "Write proxy for field `GPIO10OUTCFG`"]
pub struct GPIO10OUTCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO10OUTCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO10OUTCFG_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Output disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO10OUTCFG_A::DIS)
    }
    #[doc = "Output is push-pull"]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO10OUTCFG_A::PUSHPULL)
    }
    #[doc = "Output is open drain"]
    #[inline(always)]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO10OUTCFG_A::OD)
    }
    #[doc = "Output is tri-state"]
    #[inline(always)]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO10OUTCFG_A::TS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 9)) | (((value as u32) & 0x03) << 9);
        self.w
    }
}
#[doc = "GPIO10 input enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO10INCFG_A {
    #[doc = "0: Read the GPIO pin data"]
    READ = 0,
    #[doc = "1: Readback will always be zero"]
    RDZERO = 1,
}
impl From<GPIO10INCFG_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO10INCFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIO10INCFG`"]
pub type GPIO10INCFG_R = crate::R<bool, GPIO10INCFG_A>;
impl GPIO10INCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO10INCFG_A {
        match self.bits {
            false => GPIO10INCFG_A::READ,
            true => GPIO10INCFG_A::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        *self == GPIO10INCFG_A::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline(always)]
    pub fn is_rdzero(&self) -> bool {
        *self == GPIO10INCFG_A::RDZERO
    }
}
#[doc = "Write proxy for field `GPIO10INCFG`"]
pub struct GPIO10INCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO10INCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO10INCFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Read the GPIO pin data"]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO10INCFG_A::READ)
    }
    #[doc = "Readback will always be zero"]
    #[inline(always)]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO10INCFG_A::RDZERO)
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
#[doc = "GPIO9 interrupt direction.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO9INTD_A {
    #[doc = "0: Interrupt on low to high GPIO transition"]
    INTLH = 0,
    #[doc = "1: Interrupt on high to low GPIO transition"]
    INTHL = 1,
}
impl From<GPIO9INTD_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO9INTD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIO9INTD`"]
pub type GPIO9INTD_R = crate::R<bool, GPIO9INTD_A>;
impl GPIO9INTD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO9INTD_A {
        match self.bits {
            false => GPIO9INTD_A::INTLH,
            true => GPIO9INTD_A::INTHL,
        }
    }
    #[doc = "Checks if the value of the field is `INTLH`"]
    #[inline(always)]
    pub fn is_intlh(&self) -> bool {
        *self == GPIO9INTD_A::INTLH
    }
    #[doc = "Checks if the value of the field is `INTHL`"]
    #[inline(always)]
    pub fn is_inthl(&self) -> bool {
        *self == GPIO9INTD_A::INTHL
    }
}
#[doc = "Write proxy for field `GPIO9INTD`"]
pub struct GPIO9INTD_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO9INTD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO9INTD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt on low to high GPIO transition"]
    #[inline(always)]
    pub fn intlh(self) -> &'a mut W {
        self.variant(GPIO9INTD_A::INTLH)
    }
    #[doc = "Interrupt on high to low GPIO transition"]
    #[inline(always)]
    pub fn inthl(self) -> &'a mut W {
        self.variant(GPIO9INTD_A::INTHL)
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
#[doc = "GPIO9 output configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GPIO9OUTCFG_A {
    #[doc = "0: Output disabled"]
    DIS = 0,
    #[doc = "1: Output is push-pull"]
    PUSHPULL = 1,
    #[doc = "2: Output is open drain"]
    OD = 2,
    #[doc = "3: Output is tri-state"]
    TS = 3,
}
impl From<GPIO9OUTCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO9OUTCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `GPIO9OUTCFG`"]
pub type GPIO9OUTCFG_R = crate::R<u8, GPIO9OUTCFG_A>;
impl GPIO9OUTCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO9OUTCFG_A {
        match self.bits {
            0 => GPIO9OUTCFG_A::DIS,
            1 => GPIO9OUTCFG_A::PUSHPULL,
            2 => GPIO9OUTCFG_A::OD,
            3 => GPIO9OUTCFG_A::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == GPIO9OUTCFG_A::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == GPIO9OUTCFG_A::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        *self == GPIO9OUTCFG_A::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        *self == GPIO9OUTCFG_A::TS
    }
}
#[doc = "Write proxy for field `GPIO9OUTCFG`"]
pub struct GPIO9OUTCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO9OUTCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO9OUTCFG_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Output disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO9OUTCFG_A::DIS)
    }
    #[doc = "Output is push-pull"]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO9OUTCFG_A::PUSHPULL)
    }
    #[doc = "Output is open drain"]
    #[inline(always)]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO9OUTCFG_A::OD)
    }
    #[doc = "Output is tri-state"]
    #[inline(always)]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO9OUTCFG_A::TS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | (((value as u32) & 0x03) << 5);
        self.w
    }
}
#[doc = "GPIO9 input enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO9INCFG_A {
    #[doc = "0: Read the GPIO pin data"]
    READ = 0,
    #[doc = "1: Readback will always be zero"]
    RDZERO = 1,
}
impl From<GPIO9INCFG_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO9INCFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIO9INCFG`"]
pub type GPIO9INCFG_R = crate::R<bool, GPIO9INCFG_A>;
impl GPIO9INCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO9INCFG_A {
        match self.bits {
            false => GPIO9INCFG_A::READ,
            true => GPIO9INCFG_A::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        *self == GPIO9INCFG_A::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline(always)]
    pub fn is_rdzero(&self) -> bool {
        *self == GPIO9INCFG_A::RDZERO
    }
}
#[doc = "Write proxy for field `GPIO9INCFG`"]
pub struct GPIO9INCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO9INCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO9INCFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Read the GPIO pin data"]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO9INCFG_A::READ)
    }
    #[doc = "Readback will always be zero"]
    #[inline(always)]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO9INCFG_A::RDZERO)
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
#[doc = "GPIO8 interrupt direction.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO8INTD_A {
    #[doc = "0: Interrupt on low to high GPIO transition"]
    INTLH = 0,
    #[doc = "1: Interrupt on high to low GPIO transition"]
    INTHL = 1,
}
impl From<GPIO8INTD_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO8INTD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIO8INTD`"]
pub type GPIO8INTD_R = crate::R<bool, GPIO8INTD_A>;
impl GPIO8INTD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO8INTD_A {
        match self.bits {
            false => GPIO8INTD_A::INTLH,
            true => GPIO8INTD_A::INTHL,
        }
    }
    #[doc = "Checks if the value of the field is `INTLH`"]
    #[inline(always)]
    pub fn is_intlh(&self) -> bool {
        *self == GPIO8INTD_A::INTLH
    }
    #[doc = "Checks if the value of the field is `INTHL`"]
    #[inline(always)]
    pub fn is_inthl(&self) -> bool {
        *self == GPIO8INTD_A::INTHL
    }
}
#[doc = "Write proxy for field `GPIO8INTD`"]
pub struct GPIO8INTD_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO8INTD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO8INTD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt on low to high GPIO transition"]
    #[inline(always)]
    pub fn intlh(self) -> &'a mut W {
        self.variant(GPIO8INTD_A::INTLH)
    }
    #[doc = "Interrupt on high to low GPIO transition"]
    #[inline(always)]
    pub fn inthl(self) -> &'a mut W {
        self.variant(GPIO8INTD_A::INTHL)
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
#[doc = "GPIO8 output configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GPIO8OUTCFG_A {
    #[doc = "0: Output disabled"]
    DIS = 0,
    #[doc = "1: Output is push-pull"]
    PUSHPULL = 1,
    #[doc = "2: Output is open drain"]
    OD = 2,
    #[doc = "3: Output is tri-state"]
    TS = 3,
}
impl From<GPIO8OUTCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO8OUTCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `GPIO8OUTCFG`"]
pub type GPIO8OUTCFG_R = crate::R<u8, GPIO8OUTCFG_A>;
impl GPIO8OUTCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO8OUTCFG_A {
        match self.bits {
            0 => GPIO8OUTCFG_A::DIS,
            1 => GPIO8OUTCFG_A::PUSHPULL,
            2 => GPIO8OUTCFG_A::OD,
            3 => GPIO8OUTCFG_A::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == GPIO8OUTCFG_A::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == GPIO8OUTCFG_A::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        *self == GPIO8OUTCFG_A::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        *self == GPIO8OUTCFG_A::TS
    }
}
#[doc = "Write proxy for field `GPIO8OUTCFG`"]
pub struct GPIO8OUTCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO8OUTCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO8OUTCFG_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Output disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO8OUTCFG_A::DIS)
    }
    #[doc = "Output is push-pull"]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO8OUTCFG_A::PUSHPULL)
    }
    #[doc = "Output is open drain"]
    #[inline(always)]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO8OUTCFG_A::OD)
    }
    #[doc = "Output is tri-state"]
    #[inline(always)]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO8OUTCFG_A::TS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | (((value as u32) & 0x03) << 1);
        self.w
    }
}
#[doc = "GPIO8 input enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO8INCFG_A {
    #[doc = "0: Read the GPIO pin data"]
    READ = 0,
    #[doc = "1: Readback will always be zero"]
    RDZERO = 1,
}
impl From<GPIO8INCFG_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO8INCFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIO8INCFG`"]
pub type GPIO8INCFG_R = crate::R<bool, GPIO8INCFG_A>;
impl GPIO8INCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO8INCFG_A {
        match self.bits {
            false => GPIO8INCFG_A::READ,
            true => GPIO8INCFG_A::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        *self == GPIO8INCFG_A::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline(always)]
    pub fn is_rdzero(&self) -> bool {
        *self == GPIO8INCFG_A::RDZERO
    }
}
#[doc = "Write proxy for field `GPIO8INCFG`"]
pub struct GPIO8INCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO8INCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO8INCFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Read the GPIO pin data"]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO8INCFG_A::READ)
    }
    #[doc = "Readback will always be zero"]
    #[inline(always)]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO8INCFG_A::RDZERO)
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
    #[doc = "Bit 31 - GPIO15 interrupt direction."]
    #[inline(always)]
    pub fn gpio15intd(&self) -> GPIO15INTD_R {
        GPIO15INTD_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 29:30 - GPIO15 output configuration."]
    #[inline(always)]
    pub fn gpio15outcfg(&self) -> GPIO15OUTCFG_R {
        GPIO15OUTCFG_R::new(((self.bits >> 29) & 0x03) as u8)
    }
    #[doc = "Bit 28 - GPIO15 input enable."]
    #[inline(always)]
    pub fn gpio15incfg(&self) -> GPIO15INCFG_R {
        GPIO15INCFG_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - GPIO14 interrupt direction."]
    #[inline(always)]
    pub fn gpio14intd(&self) -> GPIO14INTD_R {
        GPIO14INTD_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bits 25:26 - GPIO14 output configuration."]
    #[inline(always)]
    pub fn gpio14outcfg(&self) -> GPIO14OUTCFG_R {
        GPIO14OUTCFG_R::new(((self.bits >> 25) & 0x03) as u8)
    }
    #[doc = "Bit 24 - GPIO14 input enable."]
    #[inline(always)]
    pub fn gpio14incfg(&self) -> GPIO14INCFG_R {
        GPIO14INCFG_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - GPIO13 interrupt direction."]
    #[inline(always)]
    pub fn gpio13intd(&self) -> GPIO13INTD_R {
        GPIO13INTD_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 21:22 - GPIO13 output configuration."]
    #[inline(always)]
    pub fn gpio13outcfg(&self) -> GPIO13OUTCFG_R {
        GPIO13OUTCFG_R::new(((self.bits >> 21) & 0x03) as u8)
    }
    #[doc = "Bit 20 - GPIO13 input enable."]
    #[inline(always)]
    pub fn gpio13incfg(&self) -> GPIO13INCFG_R {
        GPIO13INCFG_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - GPIO12 interrupt direction."]
    #[inline(always)]
    pub fn gpio12intd(&self) -> GPIO12INTD_R {
        GPIO12INTD_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bits 17:18 - GPIO12 output configuration."]
    #[inline(always)]
    pub fn gpio12outcfg(&self) -> GPIO12OUTCFG_R {
        GPIO12OUTCFG_R::new(((self.bits >> 17) & 0x03) as u8)
    }
    #[doc = "Bit 16 - GPIO12 input enable."]
    #[inline(always)]
    pub fn gpio12incfg(&self) -> GPIO12INCFG_R {
        GPIO12INCFG_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - GPIO11 interrupt direction."]
    #[inline(always)]
    pub fn gpio11intd(&self) -> GPIO11INTD_R {
        GPIO11INTD_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 13:14 - GPIO11 output configuration."]
    #[inline(always)]
    pub fn gpio11outcfg(&self) -> GPIO11OUTCFG_R {
        GPIO11OUTCFG_R::new(((self.bits >> 13) & 0x03) as u8)
    }
    #[doc = "Bit 12 - GPIO11 input enable."]
    #[inline(always)]
    pub fn gpio11incfg(&self) -> GPIO11INCFG_R {
        GPIO11INCFG_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - GPIO10 interrupt direction."]
    #[inline(always)]
    pub fn gpio10intd(&self) -> GPIO10INTD_R {
        GPIO10INTD_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 9:10 - GPIO10 output configuration."]
    #[inline(always)]
    pub fn gpio10outcfg(&self) -> GPIO10OUTCFG_R {
        GPIO10OUTCFG_R::new(((self.bits >> 9) & 0x03) as u8)
    }
    #[doc = "Bit 8 - GPIO10 input enable."]
    #[inline(always)]
    pub fn gpio10incfg(&self) -> GPIO10INCFG_R {
        GPIO10INCFG_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - GPIO9 interrupt direction."]
    #[inline(always)]
    pub fn gpio9intd(&self) -> GPIO9INTD_R {
        GPIO9INTD_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 5:6 - GPIO9 output configuration."]
    #[inline(always)]
    pub fn gpio9outcfg(&self) -> GPIO9OUTCFG_R {
        GPIO9OUTCFG_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bit 4 - GPIO9 input enable."]
    #[inline(always)]
    pub fn gpio9incfg(&self) -> GPIO9INCFG_R {
        GPIO9INCFG_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - GPIO8 interrupt direction."]
    #[inline(always)]
    pub fn gpio8intd(&self) -> GPIO8INTD_R {
        GPIO8INTD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - GPIO8 output configuration."]
    #[inline(always)]
    pub fn gpio8outcfg(&self) -> GPIO8OUTCFG_R {
        GPIO8OUTCFG_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bit 0 - GPIO8 input enable."]
    #[inline(always)]
    pub fn gpio8incfg(&self) -> GPIO8INCFG_R {
        GPIO8INCFG_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - GPIO15 interrupt direction."]
    #[inline(always)]
    pub fn gpio15intd(&mut self) -> GPIO15INTD_W {
        GPIO15INTD_W { w: self }
    }
    #[doc = "Bits 29:30 - GPIO15 output configuration."]
    #[inline(always)]
    pub fn gpio15outcfg(&mut self) -> GPIO15OUTCFG_W {
        GPIO15OUTCFG_W { w: self }
    }
    #[doc = "Bit 28 - GPIO15 input enable."]
    #[inline(always)]
    pub fn gpio15incfg(&mut self) -> GPIO15INCFG_W {
        GPIO15INCFG_W { w: self }
    }
    #[doc = "Bit 27 - GPIO14 interrupt direction."]
    #[inline(always)]
    pub fn gpio14intd(&mut self) -> GPIO14INTD_W {
        GPIO14INTD_W { w: self }
    }
    #[doc = "Bits 25:26 - GPIO14 output configuration."]
    #[inline(always)]
    pub fn gpio14outcfg(&mut self) -> GPIO14OUTCFG_W {
        GPIO14OUTCFG_W { w: self }
    }
    #[doc = "Bit 24 - GPIO14 input enable."]
    #[inline(always)]
    pub fn gpio14incfg(&mut self) -> GPIO14INCFG_W {
        GPIO14INCFG_W { w: self }
    }
    #[doc = "Bit 23 - GPIO13 interrupt direction."]
    #[inline(always)]
    pub fn gpio13intd(&mut self) -> GPIO13INTD_W {
        GPIO13INTD_W { w: self }
    }
    #[doc = "Bits 21:22 - GPIO13 output configuration."]
    #[inline(always)]
    pub fn gpio13outcfg(&mut self) -> GPIO13OUTCFG_W {
        GPIO13OUTCFG_W { w: self }
    }
    #[doc = "Bit 20 - GPIO13 input enable."]
    #[inline(always)]
    pub fn gpio13incfg(&mut self) -> GPIO13INCFG_W {
        GPIO13INCFG_W { w: self }
    }
    #[doc = "Bit 19 - GPIO12 interrupt direction."]
    #[inline(always)]
    pub fn gpio12intd(&mut self) -> GPIO12INTD_W {
        GPIO12INTD_W { w: self }
    }
    #[doc = "Bits 17:18 - GPIO12 output configuration."]
    #[inline(always)]
    pub fn gpio12outcfg(&mut self) -> GPIO12OUTCFG_W {
        GPIO12OUTCFG_W { w: self }
    }
    #[doc = "Bit 16 - GPIO12 input enable."]
    #[inline(always)]
    pub fn gpio12incfg(&mut self) -> GPIO12INCFG_W {
        GPIO12INCFG_W { w: self }
    }
    #[doc = "Bit 15 - GPIO11 interrupt direction."]
    #[inline(always)]
    pub fn gpio11intd(&mut self) -> GPIO11INTD_W {
        GPIO11INTD_W { w: self }
    }
    #[doc = "Bits 13:14 - GPIO11 output configuration."]
    #[inline(always)]
    pub fn gpio11outcfg(&mut self) -> GPIO11OUTCFG_W {
        GPIO11OUTCFG_W { w: self }
    }
    #[doc = "Bit 12 - GPIO11 input enable."]
    #[inline(always)]
    pub fn gpio11incfg(&mut self) -> GPIO11INCFG_W {
        GPIO11INCFG_W { w: self }
    }
    #[doc = "Bit 11 - GPIO10 interrupt direction."]
    #[inline(always)]
    pub fn gpio10intd(&mut self) -> GPIO10INTD_W {
        GPIO10INTD_W { w: self }
    }
    #[doc = "Bits 9:10 - GPIO10 output configuration."]
    #[inline(always)]
    pub fn gpio10outcfg(&mut self) -> GPIO10OUTCFG_W {
        GPIO10OUTCFG_W { w: self }
    }
    #[doc = "Bit 8 - GPIO10 input enable."]
    #[inline(always)]
    pub fn gpio10incfg(&mut self) -> GPIO10INCFG_W {
        GPIO10INCFG_W { w: self }
    }
    #[doc = "Bit 7 - GPIO9 interrupt direction."]
    #[inline(always)]
    pub fn gpio9intd(&mut self) -> GPIO9INTD_W {
        GPIO9INTD_W { w: self }
    }
    #[doc = "Bits 5:6 - GPIO9 output configuration."]
    #[inline(always)]
    pub fn gpio9outcfg(&mut self) -> GPIO9OUTCFG_W {
        GPIO9OUTCFG_W { w: self }
    }
    #[doc = "Bit 4 - GPIO9 input enable."]
    #[inline(always)]
    pub fn gpio9incfg(&mut self) -> GPIO9INCFG_W {
        GPIO9INCFG_W { w: self }
    }
    #[doc = "Bit 3 - GPIO8 interrupt direction."]
    #[inline(always)]
    pub fn gpio8intd(&mut self) -> GPIO8INTD_W {
        GPIO8INTD_W { w: self }
    }
    #[doc = "Bits 1:2 - GPIO8 output configuration."]
    #[inline(always)]
    pub fn gpio8outcfg(&mut self) -> GPIO8OUTCFG_W {
        GPIO8OUTCFG_W { w: self }
    }
    #[doc = "Bit 0 - GPIO8 input enable."]
    #[inline(always)]
    pub fn gpio8incfg(&mut self) -> GPIO8INCFG_W {
        GPIO8INCFG_W { w: self }
    }
}
