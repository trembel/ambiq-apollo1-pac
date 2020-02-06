#[doc = "Reader of register CFGF"]
pub type R = crate::R<u32, super::CFGF>;
#[doc = "Writer for register CFGF"]
pub type W = crate::W<u32, super::CFGF>;
#[doc = "Register CFGF `reset()`'s with value 0"]
impl crate::ResetValue for super::CFGF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "GPIO47 interrupt direction.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO47INTD_A {
    #[doc = "0: Interrupt on low to high GPIO transition"]
    INTLH = 0,
    #[doc = "1: Interrupt on high to low GPIO transition"]
    INTHL = 1,
}
impl From<GPIO47INTD_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO47INTD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIO47INTD`"]
pub type GPIO47INTD_R = crate::R<bool, GPIO47INTD_A>;
impl GPIO47INTD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO47INTD_A {
        match self.bits {
            false => GPIO47INTD_A::INTLH,
            true => GPIO47INTD_A::INTHL,
        }
    }
    #[doc = "Checks if the value of the field is `INTLH`"]
    #[inline(always)]
    pub fn is_intlh(&self) -> bool {
        *self == GPIO47INTD_A::INTLH
    }
    #[doc = "Checks if the value of the field is `INTHL`"]
    #[inline(always)]
    pub fn is_inthl(&self) -> bool {
        *self == GPIO47INTD_A::INTHL
    }
}
#[doc = "Write proxy for field `GPIO47INTD`"]
pub struct GPIO47INTD_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO47INTD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO47INTD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt on low to high GPIO transition"]
    #[inline(always)]
    pub fn intlh(self) -> &'a mut W {
        self.variant(GPIO47INTD_A::INTLH)
    }
    #[doc = "Interrupt on high to low GPIO transition"]
    #[inline(always)]
    pub fn inthl(self) -> &'a mut W {
        self.variant(GPIO47INTD_A::INTHL)
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
#[doc = "GPIO47 output configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GPIO47OUTCFG_A {
    #[doc = "0: Output disabled"]
    DIS = 0,
    #[doc = "1: Output is push-pull"]
    PUSHPULL = 1,
    #[doc = "2: Output is open drain"]
    OD = 2,
    #[doc = "3: Output is tri-state"]
    TS = 3,
}
impl From<GPIO47OUTCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO47OUTCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `GPIO47OUTCFG`"]
pub type GPIO47OUTCFG_R = crate::R<u8, GPIO47OUTCFG_A>;
impl GPIO47OUTCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO47OUTCFG_A {
        match self.bits {
            0 => GPIO47OUTCFG_A::DIS,
            1 => GPIO47OUTCFG_A::PUSHPULL,
            2 => GPIO47OUTCFG_A::OD,
            3 => GPIO47OUTCFG_A::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == GPIO47OUTCFG_A::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == GPIO47OUTCFG_A::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        *self == GPIO47OUTCFG_A::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        *self == GPIO47OUTCFG_A::TS
    }
}
#[doc = "Write proxy for field `GPIO47OUTCFG`"]
pub struct GPIO47OUTCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO47OUTCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO47OUTCFG_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Output disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO47OUTCFG_A::DIS)
    }
    #[doc = "Output is push-pull"]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO47OUTCFG_A::PUSHPULL)
    }
    #[doc = "Output is open drain"]
    #[inline(always)]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO47OUTCFG_A::OD)
    }
    #[doc = "Output is tri-state"]
    #[inline(always)]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO47OUTCFG_A::TS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 29)) | (((value as u32) & 0x03) << 29);
        self.w
    }
}
#[doc = "GPIO47 input enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO47INCFG_A {
    #[doc = "0: Read the GPIO pin data"]
    READ = 0,
    #[doc = "1: Readback will always be zero"]
    RDZERO = 1,
}
impl From<GPIO47INCFG_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO47INCFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIO47INCFG`"]
pub type GPIO47INCFG_R = crate::R<bool, GPIO47INCFG_A>;
impl GPIO47INCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO47INCFG_A {
        match self.bits {
            false => GPIO47INCFG_A::READ,
            true => GPIO47INCFG_A::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        *self == GPIO47INCFG_A::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline(always)]
    pub fn is_rdzero(&self) -> bool {
        *self == GPIO47INCFG_A::RDZERO
    }
}
#[doc = "Write proxy for field `GPIO47INCFG`"]
pub struct GPIO47INCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO47INCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO47INCFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Read the GPIO pin data"]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO47INCFG_A::READ)
    }
    #[doc = "Readback will always be zero"]
    #[inline(always)]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO47INCFG_A::RDZERO)
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
#[doc = "GPIO46 interrupt direction.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO46INTD_A {
    #[doc = "0: Interrupt on low to high GPIO transition"]
    INTLH = 0,
    #[doc = "1: Interrupt on high to low GPIO transition"]
    INTHL = 1,
}
impl From<GPIO46INTD_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO46INTD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIO46INTD`"]
pub type GPIO46INTD_R = crate::R<bool, GPIO46INTD_A>;
impl GPIO46INTD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO46INTD_A {
        match self.bits {
            false => GPIO46INTD_A::INTLH,
            true => GPIO46INTD_A::INTHL,
        }
    }
    #[doc = "Checks if the value of the field is `INTLH`"]
    #[inline(always)]
    pub fn is_intlh(&self) -> bool {
        *self == GPIO46INTD_A::INTLH
    }
    #[doc = "Checks if the value of the field is `INTHL`"]
    #[inline(always)]
    pub fn is_inthl(&self) -> bool {
        *self == GPIO46INTD_A::INTHL
    }
}
#[doc = "Write proxy for field `GPIO46INTD`"]
pub struct GPIO46INTD_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO46INTD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO46INTD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt on low to high GPIO transition"]
    #[inline(always)]
    pub fn intlh(self) -> &'a mut W {
        self.variant(GPIO46INTD_A::INTLH)
    }
    #[doc = "Interrupt on high to low GPIO transition"]
    #[inline(always)]
    pub fn inthl(self) -> &'a mut W {
        self.variant(GPIO46INTD_A::INTHL)
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
#[doc = "GPIO46 output configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GPIO46OUTCFG_A {
    #[doc = "0: Output disabled"]
    DIS = 0,
    #[doc = "1: Output is push-pull"]
    PUSHPULL = 1,
    #[doc = "2: Output is open drain"]
    OD = 2,
    #[doc = "3: Output is tri-state"]
    TS = 3,
}
impl From<GPIO46OUTCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO46OUTCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `GPIO46OUTCFG`"]
pub type GPIO46OUTCFG_R = crate::R<u8, GPIO46OUTCFG_A>;
impl GPIO46OUTCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO46OUTCFG_A {
        match self.bits {
            0 => GPIO46OUTCFG_A::DIS,
            1 => GPIO46OUTCFG_A::PUSHPULL,
            2 => GPIO46OUTCFG_A::OD,
            3 => GPIO46OUTCFG_A::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == GPIO46OUTCFG_A::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == GPIO46OUTCFG_A::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        *self == GPIO46OUTCFG_A::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        *self == GPIO46OUTCFG_A::TS
    }
}
#[doc = "Write proxy for field `GPIO46OUTCFG`"]
pub struct GPIO46OUTCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO46OUTCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO46OUTCFG_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Output disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO46OUTCFG_A::DIS)
    }
    #[doc = "Output is push-pull"]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO46OUTCFG_A::PUSHPULL)
    }
    #[doc = "Output is open drain"]
    #[inline(always)]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO46OUTCFG_A::OD)
    }
    #[doc = "Output is tri-state"]
    #[inline(always)]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO46OUTCFG_A::TS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 25)) | (((value as u32) & 0x03) << 25);
        self.w
    }
}
#[doc = "GPIO46 input enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO46INCFG_A {
    #[doc = "0: Read the GPIO pin data"]
    READ = 0,
    #[doc = "1: Readback will always be zero"]
    RDZERO = 1,
}
impl From<GPIO46INCFG_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO46INCFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIO46INCFG`"]
pub type GPIO46INCFG_R = crate::R<bool, GPIO46INCFG_A>;
impl GPIO46INCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO46INCFG_A {
        match self.bits {
            false => GPIO46INCFG_A::READ,
            true => GPIO46INCFG_A::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        *self == GPIO46INCFG_A::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline(always)]
    pub fn is_rdzero(&self) -> bool {
        *self == GPIO46INCFG_A::RDZERO
    }
}
#[doc = "Write proxy for field `GPIO46INCFG`"]
pub struct GPIO46INCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO46INCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO46INCFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Read the GPIO pin data"]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO46INCFG_A::READ)
    }
    #[doc = "Readback will always be zero"]
    #[inline(always)]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO46INCFG_A::RDZERO)
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
#[doc = "GPIO45 interrupt direction.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO45INTD_A {
    #[doc = "0: Interrupt on low to high GPIO transition"]
    INTLH = 0,
    #[doc = "1: Interrupt on high to low GPIO transition"]
    INTHL = 1,
}
impl From<GPIO45INTD_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO45INTD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIO45INTD`"]
pub type GPIO45INTD_R = crate::R<bool, GPIO45INTD_A>;
impl GPIO45INTD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO45INTD_A {
        match self.bits {
            false => GPIO45INTD_A::INTLH,
            true => GPIO45INTD_A::INTHL,
        }
    }
    #[doc = "Checks if the value of the field is `INTLH`"]
    #[inline(always)]
    pub fn is_intlh(&self) -> bool {
        *self == GPIO45INTD_A::INTLH
    }
    #[doc = "Checks if the value of the field is `INTHL`"]
    #[inline(always)]
    pub fn is_inthl(&self) -> bool {
        *self == GPIO45INTD_A::INTHL
    }
}
#[doc = "Write proxy for field `GPIO45INTD`"]
pub struct GPIO45INTD_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO45INTD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO45INTD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt on low to high GPIO transition"]
    #[inline(always)]
    pub fn intlh(self) -> &'a mut W {
        self.variant(GPIO45INTD_A::INTLH)
    }
    #[doc = "Interrupt on high to low GPIO transition"]
    #[inline(always)]
    pub fn inthl(self) -> &'a mut W {
        self.variant(GPIO45INTD_A::INTHL)
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
#[doc = "GPIO45 output configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GPIO45OUTCFG_A {
    #[doc = "0: Output disabled"]
    DIS = 0,
    #[doc = "1: Output is push-pull"]
    PUSHPULL = 1,
    #[doc = "2: Output is open drain"]
    OD = 2,
    #[doc = "3: Output is tri-state"]
    TS = 3,
}
impl From<GPIO45OUTCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO45OUTCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `GPIO45OUTCFG`"]
pub type GPIO45OUTCFG_R = crate::R<u8, GPIO45OUTCFG_A>;
impl GPIO45OUTCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO45OUTCFG_A {
        match self.bits {
            0 => GPIO45OUTCFG_A::DIS,
            1 => GPIO45OUTCFG_A::PUSHPULL,
            2 => GPIO45OUTCFG_A::OD,
            3 => GPIO45OUTCFG_A::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == GPIO45OUTCFG_A::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == GPIO45OUTCFG_A::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        *self == GPIO45OUTCFG_A::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        *self == GPIO45OUTCFG_A::TS
    }
}
#[doc = "Write proxy for field `GPIO45OUTCFG`"]
pub struct GPIO45OUTCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO45OUTCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO45OUTCFG_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Output disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO45OUTCFG_A::DIS)
    }
    #[doc = "Output is push-pull"]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO45OUTCFG_A::PUSHPULL)
    }
    #[doc = "Output is open drain"]
    #[inline(always)]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO45OUTCFG_A::OD)
    }
    #[doc = "Output is tri-state"]
    #[inline(always)]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO45OUTCFG_A::TS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 21)) | (((value as u32) & 0x03) << 21);
        self.w
    }
}
#[doc = "GPIO45 input enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO45INCFG_A {
    #[doc = "0: Read the GPIO pin data"]
    READ = 0,
    #[doc = "1: Readback will always be zero"]
    RDZERO = 1,
}
impl From<GPIO45INCFG_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO45INCFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIO45INCFG`"]
pub type GPIO45INCFG_R = crate::R<bool, GPIO45INCFG_A>;
impl GPIO45INCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO45INCFG_A {
        match self.bits {
            false => GPIO45INCFG_A::READ,
            true => GPIO45INCFG_A::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        *self == GPIO45INCFG_A::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline(always)]
    pub fn is_rdzero(&self) -> bool {
        *self == GPIO45INCFG_A::RDZERO
    }
}
#[doc = "Write proxy for field `GPIO45INCFG`"]
pub struct GPIO45INCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO45INCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO45INCFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Read the GPIO pin data"]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO45INCFG_A::READ)
    }
    #[doc = "Readback will always be zero"]
    #[inline(always)]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO45INCFG_A::RDZERO)
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
#[doc = "GPIO44 interrupt direction.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO44INTD_A {
    #[doc = "0: Interrupt on low to high GPIO transition"]
    INTLH = 0,
    #[doc = "1: Interrupt on high to low GPIO transition"]
    INTHL = 1,
}
impl From<GPIO44INTD_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO44INTD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIO44INTD`"]
pub type GPIO44INTD_R = crate::R<bool, GPIO44INTD_A>;
impl GPIO44INTD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO44INTD_A {
        match self.bits {
            false => GPIO44INTD_A::INTLH,
            true => GPIO44INTD_A::INTHL,
        }
    }
    #[doc = "Checks if the value of the field is `INTLH`"]
    #[inline(always)]
    pub fn is_intlh(&self) -> bool {
        *self == GPIO44INTD_A::INTLH
    }
    #[doc = "Checks if the value of the field is `INTHL`"]
    #[inline(always)]
    pub fn is_inthl(&self) -> bool {
        *self == GPIO44INTD_A::INTHL
    }
}
#[doc = "Write proxy for field `GPIO44INTD`"]
pub struct GPIO44INTD_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO44INTD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO44INTD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt on low to high GPIO transition"]
    #[inline(always)]
    pub fn intlh(self) -> &'a mut W {
        self.variant(GPIO44INTD_A::INTLH)
    }
    #[doc = "Interrupt on high to low GPIO transition"]
    #[inline(always)]
    pub fn inthl(self) -> &'a mut W {
        self.variant(GPIO44INTD_A::INTHL)
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
#[doc = "GPIO44 output configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GPIO44OUTCFG_A {
    #[doc = "0: Output disabled"]
    DIS = 0,
    #[doc = "1: Output is push-pull"]
    PUSHPULL = 1,
    #[doc = "2: Output is open drain"]
    OD = 2,
    #[doc = "3: Output is tri-state"]
    TS = 3,
}
impl From<GPIO44OUTCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO44OUTCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `GPIO44OUTCFG`"]
pub type GPIO44OUTCFG_R = crate::R<u8, GPIO44OUTCFG_A>;
impl GPIO44OUTCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO44OUTCFG_A {
        match self.bits {
            0 => GPIO44OUTCFG_A::DIS,
            1 => GPIO44OUTCFG_A::PUSHPULL,
            2 => GPIO44OUTCFG_A::OD,
            3 => GPIO44OUTCFG_A::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == GPIO44OUTCFG_A::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == GPIO44OUTCFG_A::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        *self == GPIO44OUTCFG_A::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        *self == GPIO44OUTCFG_A::TS
    }
}
#[doc = "Write proxy for field `GPIO44OUTCFG`"]
pub struct GPIO44OUTCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO44OUTCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO44OUTCFG_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Output disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO44OUTCFG_A::DIS)
    }
    #[doc = "Output is push-pull"]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO44OUTCFG_A::PUSHPULL)
    }
    #[doc = "Output is open drain"]
    #[inline(always)]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO44OUTCFG_A::OD)
    }
    #[doc = "Output is tri-state"]
    #[inline(always)]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO44OUTCFG_A::TS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 17)) | (((value as u32) & 0x03) << 17);
        self.w
    }
}
#[doc = "GPIO44 input enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO44INCFG_A {
    #[doc = "0: Read the GPIO pin data"]
    READ = 0,
    #[doc = "1: Readback will always be zero"]
    RDZERO = 1,
}
impl From<GPIO44INCFG_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO44INCFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIO44INCFG`"]
pub type GPIO44INCFG_R = crate::R<bool, GPIO44INCFG_A>;
impl GPIO44INCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO44INCFG_A {
        match self.bits {
            false => GPIO44INCFG_A::READ,
            true => GPIO44INCFG_A::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        *self == GPIO44INCFG_A::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline(always)]
    pub fn is_rdzero(&self) -> bool {
        *self == GPIO44INCFG_A::RDZERO
    }
}
#[doc = "Write proxy for field `GPIO44INCFG`"]
pub struct GPIO44INCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO44INCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO44INCFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Read the GPIO pin data"]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO44INCFG_A::READ)
    }
    #[doc = "Readback will always be zero"]
    #[inline(always)]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO44INCFG_A::RDZERO)
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
#[doc = "GPIO43 interrupt direction.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO43INTD_A {
    #[doc = "0: Interrupt on low to high GPIO transition"]
    INTLH = 0,
    #[doc = "1: Interrupt on high to low GPIO transition"]
    INTHL = 1,
}
impl From<GPIO43INTD_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO43INTD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIO43INTD`"]
pub type GPIO43INTD_R = crate::R<bool, GPIO43INTD_A>;
impl GPIO43INTD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO43INTD_A {
        match self.bits {
            false => GPIO43INTD_A::INTLH,
            true => GPIO43INTD_A::INTHL,
        }
    }
    #[doc = "Checks if the value of the field is `INTLH`"]
    #[inline(always)]
    pub fn is_intlh(&self) -> bool {
        *self == GPIO43INTD_A::INTLH
    }
    #[doc = "Checks if the value of the field is `INTHL`"]
    #[inline(always)]
    pub fn is_inthl(&self) -> bool {
        *self == GPIO43INTD_A::INTHL
    }
}
#[doc = "Write proxy for field `GPIO43INTD`"]
pub struct GPIO43INTD_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO43INTD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO43INTD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt on low to high GPIO transition"]
    #[inline(always)]
    pub fn intlh(self) -> &'a mut W {
        self.variant(GPIO43INTD_A::INTLH)
    }
    #[doc = "Interrupt on high to low GPIO transition"]
    #[inline(always)]
    pub fn inthl(self) -> &'a mut W {
        self.variant(GPIO43INTD_A::INTHL)
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
#[doc = "GPIO43 output configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GPIO43OUTCFG_A {
    #[doc = "0: Output disabled"]
    DIS = 0,
    #[doc = "1: Output is push-pull"]
    PUSHPULL = 1,
    #[doc = "2: Output is open drain"]
    OD = 2,
    #[doc = "3: Output is tri-state"]
    TS = 3,
}
impl From<GPIO43OUTCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO43OUTCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `GPIO43OUTCFG`"]
pub type GPIO43OUTCFG_R = crate::R<u8, GPIO43OUTCFG_A>;
impl GPIO43OUTCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO43OUTCFG_A {
        match self.bits {
            0 => GPIO43OUTCFG_A::DIS,
            1 => GPIO43OUTCFG_A::PUSHPULL,
            2 => GPIO43OUTCFG_A::OD,
            3 => GPIO43OUTCFG_A::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == GPIO43OUTCFG_A::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == GPIO43OUTCFG_A::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        *self == GPIO43OUTCFG_A::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        *self == GPIO43OUTCFG_A::TS
    }
}
#[doc = "Write proxy for field `GPIO43OUTCFG`"]
pub struct GPIO43OUTCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO43OUTCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO43OUTCFG_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Output disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO43OUTCFG_A::DIS)
    }
    #[doc = "Output is push-pull"]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO43OUTCFG_A::PUSHPULL)
    }
    #[doc = "Output is open drain"]
    #[inline(always)]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO43OUTCFG_A::OD)
    }
    #[doc = "Output is tri-state"]
    #[inline(always)]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO43OUTCFG_A::TS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 13)) | (((value as u32) & 0x03) << 13);
        self.w
    }
}
#[doc = "GPIO43 input enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO43INCFG_A {
    #[doc = "0: Read the GPIO pin data"]
    READ = 0,
    #[doc = "1: Readback will always be zero"]
    RDZERO = 1,
}
impl From<GPIO43INCFG_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO43INCFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIO43INCFG`"]
pub type GPIO43INCFG_R = crate::R<bool, GPIO43INCFG_A>;
impl GPIO43INCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO43INCFG_A {
        match self.bits {
            false => GPIO43INCFG_A::READ,
            true => GPIO43INCFG_A::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        *self == GPIO43INCFG_A::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline(always)]
    pub fn is_rdzero(&self) -> bool {
        *self == GPIO43INCFG_A::RDZERO
    }
}
#[doc = "Write proxy for field `GPIO43INCFG`"]
pub struct GPIO43INCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO43INCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO43INCFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Read the GPIO pin data"]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO43INCFG_A::READ)
    }
    #[doc = "Readback will always be zero"]
    #[inline(always)]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO43INCFG_A::RDZERO)
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
#[doc = "GPIO42 interrupt direction.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO42INTD_A {
    #[doc = "0: Interrupt on low to high GPIO transition"]
    INTLH = 0,
    #[doc = "1: Interrupt on high to low GPIO transition"]
    INTHL = 1,
}
impl From<GPIO42INTD_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO42INTD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIO42INTD`"]
pub type GPIO42INTD_R = crate::R<bool, GPIO42INTD_A>;
impl GPIO42INTD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO42INTD_A {
        match self.bits {
            false => GPIO42INTD_A::INTLH,
            true => GPIO42INTD_A::INTHL,
        }
    }
    #[doc = "Checks if the value of the field is `INTLH`"]
    #[inline(always)]
    pub fn is_intlh(&self) -> bool {
        *self == GPIO42INTD_A::INTLH
    }
    #[doc = "Checks if the value of the field is `INTHL`"]
    #[inline(always)]
    pub fn is_inthl(&self) -> bool {
        *self == GPIO42INTD_A::INTHL
    }
}
#[doc = "Write proxy for field `GPIO42INTD`"]
pub struct GPIO42INTD_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO42INTD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO42INTD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt on low to high GPIO transition"]
    #[inline(always)]
    pub fn intlh(self) -> &'a mut W {
        self.variant(GPIO42INTD_A::INTLH)
    }
    #[doc = "Interrupt on high to low GPIO transition"]
    #[inline(always)]
    pub fn inthl(self) -> &'a mut W {
        self.variant(GPIO42INTD_A::INTHL)
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
#[doc = "GPIO42 output configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GPIO42OUTCFG_A {
    #[doc = "0: Output disabled"]
    DIS = 0,
    #[doc = "1: Output is push-pull"]
    PUSHPULL = 1,
    #[doc = "2: Output is open drain"]
    OD = 2,
    #[doc = "3: Output is tri-state"]
    TS = 3,
}
impl From<GPIO42OUTCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO42OUTCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `GPIO42OUTCFG`"]
pub type GPIO42OUTCFG_R = crate::R<u8, GPIO42OUTCFG_A>;
impl GPIO42OUTCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO42OUTCFG_A {
        match self.bits {
            0 => GPIO42OUTCFG_A::DIS,
            1 => GPIO42OUTCFG_A::PUSHPULL,
            2 => GPIO42OUTCFG_A::OD,
            3 => GPIO42OUTCFG_A::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == GPIO42OUTCFG_A::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == GPIO42OUTCFG_A::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        *self == GPIO42OUTCFG_A::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        *self == GPIO42OUTCFG_A::TS
    }
}
#[doc = "Write proxy for field `GPIO42OUTCFG`"]
pub struct GPIO42OUTCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO42OUTCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO42OUTCFG_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Output disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO42OUTCFG_A::DIS)
    }
    #[doc = "Output is push-pull"]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO42OUTCFG_A::PUSHPULL)
    }
    #[doc = "Output is open drain"]
    #[inline(always)]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO42OUTCFG_A::OD)
    }
    #[doc = "Output is tri-state"]
    #[inline(always)]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO42OUTCFG_A::TS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 9)) | (((value as u32) & 0x03) << 9);
        self.w
    }
}
#[doc = "GPIO42 input enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO42INCFG_A {
    #[doc = "0: Read the GPIO pin data"]
    READ = 0,
    #[doc = "1: Readback will always be zero"]
    RDZERO = 1,
}
impl From<GPIO42INCFG_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO42INCFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIO42INCFG`"]
pub type GPIO42INCFG_R = crate::R<bool, GPIO42INCFG_A>;
impl GPIO42INCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO42INCFG_A {
        match self.bits {
            false => GPIO42INCFG_A::READ,
            true => GPIO42INCFG_A::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        *self == GPIO42INCFG_A::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline(always)]
    pub fn is_rdzero(&self) -> bool {
        *self == GPIO42INCFG_A::RDZERO
    }
}
#[doc = "Write proxy for field `GPIO42INCFG`"]
pub struct GPIO42INCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO42INCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO42INCFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Read the GPIO pin data"]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO42INCFG_A::READ)
    }
    #[doc = "Readback will always be zero"]
    #[inline(always)]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO42INCFG_A::RDZERO)
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
#[doc = "GPIO41 interrupt direction.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO41INTD_A {
    #[doc = "0: Interrupt on low to high GPIO transition"]
    INTLH = 0,
    #[doc = "1: Interrupt on high to low GPIO transition"]
    INTHL = 1,
}
impl From<GPIO41INTD_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO41INTD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIO41INTD`"]
pub type GPIO41INTD_R = crate::R<bool, GPIO41INTD_A>;
impl GPIO41INTD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO41INTD_A {
        match self.bits {
            false => GPIO41INTD_A::INTLH,
            true => GPIO41INTD_A::INTHL,
        }
    }
    #[doc = "Checks if the value of the field is `INTLH`"]
    #[inline(always)]
    pub fn is_intlh(&self) -> bool {
        *self == GPIO41INTD_A::INTLH
    }
    #[doc = "Checks if the value of the field is `INTHL`"]
    #[inline(always)]
    pub fn is_inthl(&self) -> bool {
        *self == GPIO41INTD_A::INTHL
    }
}
#[doc = "Write proxy for field `GPIO41INTD`"]
pub struct GPIO41INTD_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO41INTD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO41INTD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt on low to high GPIO transition"]
    #[inline(always)]
    pub fn intlh(self) -> &'a mut W {
        self.variant(GPIO41INTD_A::INTLH)
    }
    #[doc = "Interrupt on high to low GPIO transition"]
    #[inline(always)]
    pub fn inthl(self) -> &'a mut W {
        self.variant(GPIO41INTD_A::INTHL)
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
#[doc = "GPIO41 output configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GPIO41OUTCFG_A {
    #[doc = "0: Output disabled"]
    DIS = 0,
    #[doc = "1: Output is push-pull"]
    PUSHPULL = 1,
    #[doc = "2: Output is open drain"]
    OD = 2,
    #[doc = "3: Output is tri-state"]
    TS = 3,
}
impl From<GPIO41OUTCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO41OUTCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `GPIO41OUTCFG`"]
pub type GPIO41OUTCFG_R = crate::R<u8, GPIO41OUTCFG_A>;
impl GPIO41OUTCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO41OUTCFG_A {
        match self.bits {
            0 => GPIO41OUTCFG_A::DIS,
            1 => GPIO41OUTCFG_A::PUSHPULL,
            2 => GPIO41OUTCFG_A::OD,
            3 => GPIO41OUTCFG_A::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == GPIO41OUTCFG_A::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == GPIO41OUTCFG_A::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        *self == GPIO41OUTCFG_A::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        *self == GPIO41OUTCFG_A::TS
    }
}
#[doc = "Write proxy for field `GPIO41OUTCFG`"]
pub struct GPIO41OUTCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO41OUTCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO41OUTCFG_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Output disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO41OUTCFG_A::DIS)
    }
    #[doc = "Output is push-pull"]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO41OUTCFG_A::PUSHPULL)
    }
    #[doc = "Output is open drain"]
    #[inline(always)]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO41OUTCFG_A::OD)
    }
    #[doc = "Output is tri-state"]
    #[inline(always)]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO41OUTCFG_A::TS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | (((value as u32) & 0x03) << 5);
        self.w
    }
}
#[doc = "GPIO41 input enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO41INCFG_A {
    #[doc = "0: Read the GPIO pin data"]
    READ = 0,
    #[doc = "1: Readback will always be zero"]
    RDZERO = 1,
}
impl From<GPIO41INCFG_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO41INCFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIO41INCFG`"]
pub type GPIO41INCFG_R = crate::R<bool, GPIO41INCFG_A>;
impl GPIO41INCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO41INCFG_A {
        match self.bits {
            false => GPIO41INCFG_A::READ,
            true => GPIO41INCFG_A::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        *self == GPIO41INCFG_A::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline(always)]
    pub fn is_rdzero(&self) -> bool {
        *self == GPIO41INCFG_A::RDZERO
    }
}
#[doc = "Write proxy for field `GPIO41INCFG`"]
pub struct GPIO41INCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO41INCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO41INCFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Read the GPIO pin data"]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO41INCFG_A::READ)
    }
    #[doc = "Readback will always be zero"]
    #[inline(always)]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO41INCFG_A::RDZERO)
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
#[doc = "GPIO40 interrupt direction.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO40INTD_A {
    #[doc = "0: Interrupt on low to high GPIO transition"]
    INTLH = 0,
    #[doc = "1: Interrupt on high to low GPIO transition"]
    INTHL = 1,
}
impl From<GPIO40INTD_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO40INTD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIO40INTD`"]
pub type GPIO40INTD_R = crate::R<bool, GPIO40INTD_A>;
impl GPIO40INTD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO40INTD_A {
        match self.bits {
            false => GPIO40INTD_A::INTLH,
            true => GPIO40INTD_A::INTHL,
        }
    }
    #[doc = "Checks if the value of the field is `INTLH`"]
    #[inline(always)]
    pub fn is_intlh(&self) -> bool {
        *self == GPIO40INTD_A::INTLH
    }
    #[doc = "Checks if the value of the field is `INTHL`"]
    #[inline(always)]
    pub fn is_inthl(&self) -> bool {
        *self == GPIO40INTD_A::INTHL
    }
}
#[doc = "Write proxy for field `GPIO40INTD`"]
pub struct GPIO40INTD_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO40INTD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO40INTD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt on low to high GPIO transition"]
    #[inline(always)]
    pub fn intlh(self) -> &'a mut W {
        self.variant(GPIO40INTD_A::INTLH)
    }
    #[doc = "Interrupt on high to low GPIO transition"]
    #[inline(always)]
    pub fn inthl(self) -> &'a mut W {
        self.variant(GPIO40INTD_A::INTHL)
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
#[doc = "GPIO40 output configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GPIO40OUTCFG_A {
    #[doc = "0: Output disabled"]
    DIS = 0,
    #[doc = "1: Output is push-pull"]
    PUSHPULL = 1,
    #[doc = "2: Output is open drain"]
    OD = 2,
    #[doc = "3: Output is tri-state"]
    TS = 3,
}
impl From<GPIO40OUTCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO40OUTCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `GPIO40OUTCFG`"]
pub type GPIO40OUTCFG_R = crate::R<u8, GPIO40OUTCFG_A>;
impl GPIO40OUTCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO40OUTCFG_A {
        match self.bits {
            0 => GPIO40OUTCFG_A::DIS,
            1 => GPIO40OUTCFG_A::PUSHPULL,
            2 => GPIO40OUTCFG_A::OD,
            3 => GPIO40OUTCFG_A::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == GPIO40OUTCFG_A::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == GPIO40OUTCFG_A::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        *self == GPIO40OUTCFG_A::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        *self == GPIO40OUTCFG_A::TS
    }
}
#[doc = "Write proxy for field `GPIO40OUTCFG`"]
pub struct GPIO40OUTCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO40OUTCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO40OUTCFG_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Output disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO40OUTCFG_A::DIS)
    }
    #[doc = "Output is push-pull"]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO40OUTCFG_A::PUSHPULL)
    }
    #[doc = "Output is open drain"]
    #[inline(always)]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO40OUTCFG_A::OD)
    }
    #[doc = "Output is tri-state"]
    #[inline(always)]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO40OUTCFG_A::TS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | (((value as u32) & 0x03) << 1);
        self.w
    }
}
#[doc = "GPIO40 input enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO40INCFG_A {
    #[doc = "0: Read the GPIO pin data"]
    READ = 0,
    #[doc = "1: Readback will always be zero"]
    RDZERO = 1,
}
impl From<GPIO40INCFG_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO40INCFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIO40INCFG`"]
pub type GPIO40INCFG_R = crate::R<bool, GPIO40INCFG_A>;
impl GPIO40INCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO40INCFG_A {
        match self.bits {
            false => GPIO40INCFG_A::READ,
            true => GPIO40INCFG_A::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        *self == GPIO40INCFG_A::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline(always)]
    pub fn is_rdzero(&self) -> bool {
        *self == GPIO40INCFG_A::RDZERO
    }
}
#[doc = "Write proxy for field `GPIO40INCFG`"]
pub struct GPIO40INCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO40INCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO40INCFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Read the GPIO pin data"]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO40INCFG_A::READ)
    }
    #[doc = "Readback will always be zero"]
    #[inline(always)]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO40INCFG_A::RDZERO)
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
    #[doc = "Bit 31 - GPIO47 interrupt direction."]
    #[inline(always)]
    pub fn gpio47intd(&self) -> GPIO47INTD_R {
        GPIO47INTD_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 29:30 - GPIO47 output configuration."]
    #[inline(always)]
    pub fn gpio47outcfg(&self) -> GPIO47OUTCFG_R {
        GPIO47OUTCFG_R::new(((self.bits >> 29) & 0x03) as u8)
    }
    #[doc = "Bit 28 - GPIO47 input enable."]
    #[inline(always)]
    pub fn gpio47incfg(&self) -> GPIO47INCFG_R {
        GPIO47INCFG_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - GPIO46 interrupt direction."]
    #[inline(always)]
    pub fn gpio46intd(&self) -> GPIO46INTD_R {
        GPIO46INTD_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bits 25:26 - GPIO46 output configuration."]
    #[inline(always)]
    pub fn gpio46outcfg(&self) -> GPIO46OUTCFG_R {
        GPIO46OUTCFG_R::new(((self.bits >> 25) & 0x03) as u8)
    }
    #[doc = "Bit 24 - GPIO46 input enable."]
    #[inline(always)]
    pub fn gpio46incfg(&self) -> GPIO46INCFG_R {
        GPIO46INCFG_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - GPIO45 interrupt direction."]
    #[inline(always)]
    pub fn gpio45intd(&self) -> GPIO45INTD_R {
        GPIO45INTD_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 21:22 - GPIO45 output configuration."]
    #[inline(always)]
    pub fn gpio45outcfg(&self) -> GPIO45OUTCFG_R {
        GPIO45OUTCFG_R::new(((self.bits >> 21) & 0x03) as u8)
    }
    #[doc = "Bit 20 - GPIO45 input enable."]
    #[inline(always)]
    pub fn gpio45incfg(&self) -> GPIO45INCFG_R {
        GPIO45INCFG_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - GPIO44 interrupt direction."]
    #[inline(always)]
    pub fn gpio44intd(&self) -> GPIO44INTD_R {
        GPIO44INTD_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bits 17:18 - GPIO44 output configuration."]
    #[inline(always)]
    pub fn gpio44outcfg(&self) -> GPIO44OUTCFG_R {
        GPIO44OUTCFG_R::new(((self.bits >> 17) & 0x03) as u8)
    }
    #[doc = "Bit 16 - GPIO44 input enable."]
    #[inline(always)]
    pub fn gpio44incfg(&self) -> GPIO44INCFG_R {
        GPIO44INCFG_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - GPIO43 interrupt direction."]
    #[inline(always)]
    pub fn gpio43intd(&self) -> GPIO43INTD_R {
        GPIO43INTD_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 13:14 - GPIO43 output configuration."]
    #[inline(always)]
    pub fn gpio43outcfg(&self) -> GPIO43OUTCFG_R {
        GPIO43OUTCFG_R::new(((self.bits >> 13) & 0x03) as u8)
    }
    #[doc = "Bit 12 - GPIO43 input enable."]
    #[inline(always)]
    pub fn gpio43incfg(&self) -> GPIO43INCFG_R {
        GPIO43INCFG_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - GPIO42 interrupt direction."]
    #[inline(always)]
    pub fn gpio42intd(&self) -> GPIO42INTD_R {
        GPIO42INTD_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 9:10 - GPIO42 output configuration."]
    #[inline(always)]
    pub fn gpio42outcfg(&self) -> GPIO42OUTCFG_R {
        GPIO42OUTCFG_R::new(((self.bits >> 9) & 0x03) as u8)
    }
    #[doc = "Bit 8 - GPIO42 input enable."]
    #[inline(always)]
    pub fn gpio42incfg(&self) -> GPIO42INCFG_R {
        GPIO42INCFG_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - GPIO41 interrupt direction."]
    #[inline(always)]
    pub fn gpio41intd(&self) -> GPIO41INTD_R {
        GPIO41INTD_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 5:6 - GPIO41 output configuration."]
    #[inline(always)]
    pub fn gpio41outcfg(&self) -> GPIO41OUTCFG_R {
        GPIO41OUTCFG_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bit 4 - GPIO41 input enable."]
    #[inline(always)]
    pub fn gpio41incfg(&self) -> GPIO41INCFG_R {
        GPIO41INCFG_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - GPIO40 interrupt direction."]
    #[inline(always)]
    pub fn gpio40intd(&self) -> GPIO40INTD_R {
        GPIO40INTD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - GPIO40 output configuration."]
    #[inline(always)]
    pub fn gpio40outcfg(&self) -> GPIO40OUTCFG_R {
        GPIO40OUTCFG_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bit 0 - GPIO40 input enable."]
    #[inline(always)]
    pub fn gpio40incfg(&self) -> GPIO40INCFG_R {
        GPIO40INCFG_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - GPIO47 interrupt direction."]
    #[inline(always)]
    pub fn gpio47intd(&mut self) -> GPIO47INTD_W {
        GPIO47INTD_W { w: self }
    }
    #[doc = "Bits 29:30 - GPIO47 output configuration."]
    #[inline(always)]
    pub fn gpio47outcfg(&mut self) -> GPIO47OUTCFG_W {
        GPIO47OUTCFG_W { w: self }
    }
    #[doc = "Bit 28 - GPIO47 input enable."]
    #[inline(always)]
    pub fn gpio47incfg(&mut self) -> GPIO47INCFG_W {
        GPIO47INCFG_W { w: self }
    }
    #[doc = "Bit 27 - GPIO46 interrupt direction."]
    #[inline(always)]
    pub fn gpio46intd(&mut self) -> GPIO46INTD_W {
        GPIO46INTD_W { w: self }
    }
    #[doc = "Bits 25:26 - GPIO46 output configuration."]
    #[inline(always)]
    pub fn gpio46outcfg(&mut self) -> GPIO46OUTCFG_W {
        GPIO46OUTCFG_W { w: self }
    }
    #[doc = "Bit 24 - GPIO46 input enable."]
    #[inline(always)]
    pub fn gpio46incfg(&mut self) -> GPIO46INCFG_W {
        GPIO46INCFG_W { w: self }
    }
    #[doc = "Bit 23 - GPIO45 interrupt direction."]
    #[inline(always)]
    pub fn gpio45intd(&mut self) -> GPIO45INTD_W {
        GPIO45INTD_W { w: self }
    }
    #[doc = "Bits 21:22 - GPIO45 output configuration."]
    #[inline(always)]
    pub fn gpio45outcfg(&mut self) -> GPIO45OUTCFG_W {
        GPIO45OUTCFG_W { w: self }
    }
    #[doc = "Bit 20 - GPIO45 input enable."]
    #[inline(always)]
    pub fn gpio45incfg(&mut self) -> GPIO45INCFG_W {
        GPIO45INCFG_W { w: self }
    }
    #[doc = "Bit 19 - GPIO44 interrupt direction."]
    #[inline(always)]
    pub fn gpio44intd(&mut self) -> GPIO44INTD_W {
        GPIO44INTD_W { w: self }
    }
    #[doc = "Bits 17:18 - GPIO44 output configuration."]
    #[inline(always)]
    pub fn gpio44outcfg(&mut self) -> GPIO44OUTCFG_W {
        GPIO44OUTCFG_W { w: self }
    }
    #[doc = "Bit 16 - GPIO44 input enable."]
    #[inline(always)]
    pub fn gpio44incfg(&mut self) -> GPIO44INCFG_W {
        GPIO44INCFG_W { w: self }
    }
    #[doc = "Bit 15 - GPIO43 interrupt direction."]
    #[inline(always)]
    pub fn gpio43intd(&mut self) -> GPIO43INTD_W {
        GPIO43INTD_W { w: self }
    }
    #[doc = "Bits 13:14 - GPIO43 output configuration."]
    #[inline(always)]
    pub fn gpio43outcfg(&mut self) -> GPIO43OUTCFG_W {
        GPIO43OUTCFG_W { w: self }
    }
    #[doc = "Bit 12 - GPIO43 input enable."]
    #[inline(always)]
    pub fn gpio43incfg(&mut self) -> GPIO43INCFG_W {
        GPIO43INCFG_W { w: self }
    }
    #[doc = "Bit 11 - GPIO42 interrupt direction."]
    #[inline(always)]
    pub fn gpio42intd(&mut self) -> GPIO42INTD_W {
        GPIO42INTD_W { w: self }
    }
    #[doc = "Bits 9:10 - GPIO42 output configuration."]
    #[inline(always)]
    pub fn gpio42outcfg(&mut self) -> GPIO42OUTCFG_W {
        GPIO42OUTCFG_W { w: self }
    }
    #[doc = "Bit 8 - GPIO42 input enable."]
    #[inline(always)]
    pub fn gpio42incfg(&mut self) -> GPIO42INCFG_W {
        GPIO42INCFG_W { w: self }
    }
    #[doc = "Bit 7 - GPIO41 interrupt direction."]
    #[inline(always)]
    pub fn gpio41intd(&mut self) -> GPIO41INTD_W {
        GPIO41INTD_W { w: self }
    }
    #[doc = "Bits 5:6 - GPIO41 output configuration."]
    #[inline(always)]
    pub fn gpio41outcfg(&mut self) -> GPIO41OUTCFG_W {
        GPIO41OUTCFG_W { w: self }
    }
    #[doc = "Bit 4 - GPIO41 input enable."]
    #[inline(always)]
    pub fn gpio41incfg(&mut self) -> GPIO41INCFG_W {
        GPIO41INCFG_W { w: self }
    }
    #[doc = "Bit 3 - GPIO40 interrupt direction."]
    #[inline(always)]
    pub fn gpio40intd(&mut self) -> GPIO40INTD_W {
        GPIO40INTD_W { w: self }
    }
    #[doc = "Bits 1:2 - GPIO40 output configuration."]
    #[inline(always)]
    pub fn gpio40outcfg(&mut self) -> GPIO40OUTCFG_W {
        GPIO40OUTCFG_W { w: self }
    }
    #[doc = "Bit 0 - GPIO40 input enable."]
    #[inline(always)]
    pub fn gpio40incfg(&mut self) -> GPIO40INCFG_W {
        GPIO40INCFG_W { w: self }
    }
}
