#[doc = "Reader of register CFGD"]
pub type R = crate::R<u32, super::CFGD>;
#[doc = "Writer for register CFGD"]
pub type W = crate::W<u32, super::CFGD>;
#[doc = "Register CFGD `reset()`'s with value 0"]
impl crate::ResetValue for super::CFGD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "GPIO31 interrupt direction.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO31INTD_A {
    #[doc = "0: Interrupt on low to high GPIO transition"]
    INTLH = 0,
    #[doc = "1: Interrupt on high to low GPIO transition"]
    INTHL = 1,
}
impl From<GPIO31INTD_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO31INTD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIO31INTD`"]
pub type GPIO31INTD_R = crate::R<bool, GPIO31INTD_A>;
impl GPIO31INTD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO31INTD_A {
        match self.bits {
            false => GPIO31INTD_A::INTLH,
            true => GPIO31INTD_A::INTHL,
        }
    }
    #[doc = "Checks if the value of the field is `INTLH`"]
    #[inline(always)]
    pub fn is_intlh(&self) -> bool {
        *self == GPIO31INTD_A::INTLH
    }
    #[doc = "Checks if the value of the field is `INTHL`"]
    #[inline(always)]
    pub fn is_inthl(&self) -> bool {
        *self == GPIO31INTD_A::INTHL
    }
}
#[doc = "Write proxy for field `GPIO31INTD`"]
pub struct GPIO31INTD_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO31INTD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO31INTD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt on low to high GPIO transition"]
    #[inline(always)]
    pub fn intlh(self) -> &'a mut W {
        self.variant(GPIO31INTD_A::INTLH)
    }
    #[doc = "Interrupt on high to low GPIO transition"]
    #[inline(always)]
    pub fn inthl(self) -> &'a mut W {
        self.variant(GPIO31INTD_A::INTHL)
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
#[doc = "GPIO31 output configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GPIO31OUTCFG_A {
    #[doc = "0: Output disabled"]
    DIS = 0,
    #[doc = "1: Output is push-pull"]
    PUSHPULL = 1,
    #[doc = "2: Output is open drain"]
    OD = 2,
    #[doc = "3: Output is tri-state"]
    TS = 3,
}
impl From<GPIO31OUTCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO31OUTCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `GPIO31OUTCFG`"]
pub type GPIO31OUTCFG_R = crate::R<u8, GPIO31OUTCFG_A>;
impl GPIO31OUTCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO31OUTCFG_A {
        match self.bits {
            0 => GPIO31OUTCFG_A::DIS,
            1 => GPIO31OUTCFG_A::PUSHPULL,
            2 => GPIO31OUTCFG_A::OD,
            3 => GPIO31OUTCFG_A::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == GPIO31OUTCFG_A::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == GPIO31OUTCFG_A::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        *self == GPIO31OUTCFG_A::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        *self == GPIO31OUTCFG_A::TS
    }
}
#[doc = "Write proxy for field `GPIO31OUTCFG`"]
pub struct GPIO31OUTCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO31OUTCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO31OUTCFG_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Output disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO31OUTCFG_A::DIS)
    }
    #[doc = "Output is push-pull"]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO31OUTCFG_A::PUSHPULL)
    }
    #[doc = "Output is open drain"]
    #[inline(always)]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO31OUTCFG_A::OD)
    }
    #[doc = "Output is tri-state"]
    #[inline(always)]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO31OUTCFG_A::TS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 29)) | (((value as u32) & 0x03) << 29);
        self.w
    }
}
#[doc = "GPIO31 input enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO31INCFG_A {
    #[doc = "0: Read the GPIO pin data"]
    READ = 0,
    #[doc = "1: Readback will always be zero"]
    RDZERO = 1,
}
impl From<GPIO31INCFG_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO31INCFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIO31INCFG`"]
pub type GPIO31INCFG_R = crate::R<bool, GPIO31INCFG_A>;
impl GPIO31INCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO31INCFG_A {
        match self.bits {
            false => GPIO31INCFG_A::READ,
            true => GPIO31INCFG_A::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        *self == GPIO31INCFG_A::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline(always)]
    pub fn is_rdzero(&self) -> bool {
        *self == GPIO31INCFG_A::RDZERO
    }
}
#[doc = "Write proxy for field `GPIO31INCFG`"]
pub struct GPIO31INCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO31INCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO31INCFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Read the GPIO pin data"]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO31INCFG_A::READ)
    }
    #[doc = "Readback will always be zero"]
    #[inline(always)]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO31INCFG_A::RDZERO)
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
#[doc = "GPIO30 interrupt direction.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO30INTD_A {
    #[doc = "0: Interrupt on low to high GPIO transition"]
    INTLH = 0,
    #[doc = "1: Interrupt on high to low GPIO transition"]
    INTHL = 1,
}
impl From<GPIO30INTD_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO30INTD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIO30INTD`"]
pub type GPIO30INTD_R = crate::R<bool, GPIO30INTD_A>;
impl GPIO30INTD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO30INTD_A {
        match self.bits {
            false => GPIO30INTD_A::INTLH,
            true => GPIO30INTD_A::INTHL,
        }
    }
    #[doc = "Checks if the value of the field is `INTLH`"]
    #[inline(always)]
    pub fn is_intlh(&self) -> bool {
        *self == GPIO30INTD_A::INTLH
    }
    #[doc = "Checks if the value of the field is `INTHL`"]
    #[inline(always)]
    pub fn is_inthl(&self) -> bool {
        *self == GPIO30INTD_A::INTHL
    }
}
#[doc = "Write proxy for field `GPIO30INTD`"]
pub struct GPIO30INTD_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO30INTD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO30INTD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt on low to high GPIO transition"]
    #[inline(always)]
    pub fn intlh(self) -> &'a mut W {
        self.variant(GPIO30INTD_A::INTLH)
    }
    #[doc = "Interrupt on high to low GPIO transition"]
    #[inline(always)]
    pub fn inthl(self) -> &'a mut W {
        self.variant(GPIO30INTD_A::INTHL)
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
#[doc = "GPIO30 output configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GPIO30OUTCFG_A {
    #[doc = "0: Output disabled"]
    DIS = 0,
    #[doc = "1: Output is push-pull"]
    PUSHPULL = 1,
    #[doc = "2: Output is open drain"]
    OD = 2,
    #[doc = "3: Output is tri-state"]
    TS = 3,
}
impl From<GPIO30OUTCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO30OUTCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `GPIO30OUTCFG`"]
pub type GPIO30OUTCFG_R = crate::R<u8, GPIO30OUTCFG_A>;
impl GPIO30OUTCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO30OUTCFG_A {
        match self.bits {
            0 => GPIO30OUTCFG_A::DIS,
            1 => GPIO30OUTCFG_A::PUSHPULL,
            2 => GPIO30OUTCFG_A::OD,
            3 => GPIO30OUTCFG_A::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == GPIO30OUTCFG_A::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == GPIO30OUTCFG_A::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        *self == GPIO30OUTCFG_A::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        *self == GPIO30OUTCFG_A::TS
    }
}
#[doc = "Write proxy for field `GPIO30OUTCFG`"]
pub struct GPIO30OUTCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO30OUTCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO30OUTCFG_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Output disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO30OUTCFG_A::DIS)
    }
    #[doc = "Output is push-pull"]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO30OUTCFG_A::PUSHPULL)
    }
    #[doc = "Output is open drain"]
    #[inline(always)]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO30OUTCFG_A::OD)
    }
    #[doc = "Output is tri-state"]
    #[inline(always)]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO30OUTCFG_A::TS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 25)) | (((value as u32) & 0x03) << 25);
        self.w
    }
}
#[doc = "GPIO30 input enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO30INCFG_A {
    #[doc = "0: Read the GPIO pin data"]
    READ = 0,
    #[doc = "1: Readback will always be zero"]
    RDZERO = 1,
}
impl From<GPIO30INCFG_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO30INCFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIO30INCFG`"]
pub type GPIO30INCFG_R = crate::R<bool, GPIO30INCFG_A>;
impl GPIO30INCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO30INCFG_A {
        match self.bits {
            false => GPIO30INCFG_A::READ,
            true => GPIO30INCFG_A::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        *self == GPIO30INCFG_A::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline(always)]
    pub fn is_rdzero(&self) -> bool {
        *self == GPIO30INCFG_A::RDZERO
    }
}
#[doc = "Write proxy for field `GPIO30INCFG`"]
pub struct GPIO30INCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO30INCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO30INCFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Read the GPIO pin data"]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO30INCFG_A::READ)
    }
    #[doc = "Readback will always be zero"]
    #[inline(always)]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO30INCFG_A::RDZERO)
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
#[doc = "GPIO29 interrupt direction.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO29INTD_A {
    #[doc = "0: Interrupt on low to high GPIO transition"]
    INTLH = 0,
    #[doc = "1: Interrupt on high to low GPIO transition"]
    INTHL = 1,
}
impl From<GPIO29INTD_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO29INTD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIO29INTD`"]
pub type GPIO29INTD_R = crate::R<bool, GPIO29INTD_A>;
impl GPIO29INTD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO29INTD_A {
        match self.bits {
            false => GPIO29INTD_A::INTLH,
            true => GPIO29INTD_A::INTHL,
        }
    }
    #[doc = "Checks if the value of the field is `INTLH`"]
    #[inline(always)]
    pub fn is_intlh(&self) -> bool {
        *self == GPIO29INTD_A::INTLH
    }
    #[doc = "Checks if the value of the field is `INTHL`"]
    #[inline(always)]
    pub fn is_inthl(&self) -> bool {
        *self == GPIO29INTD_A::INTHL
    }
}
#[doc = "Write proxy for field `GPIO29INTD`"]
pub struct GPIO29INTD_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO29INTD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO29INTD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt on low to high GPIO transition"]
    #[inline(always)]
    pub fn intlh(self) -> &'a mut W {
        self.variant(GPIO29INTD_A::INTLH)
    }
    #[doc = "Interrupt on high to low GPIO transition"]
    #[inline(always)]
    pub fn inthl(self) -> &'a mut W {
        self.variant(GPIO29INTD_A::INTHL)
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
#[doc = "GPIO29 output configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GPIO29OUTCFG_A {
    #[doc = "0: Output disabled"]
    DIS = 0,
    #[doc = "1: Output is push-pull"]
    PUSHPULL = 1,
    #[doc = "2: Output is open drain"]
    OD = 2,
    #[doc = "3: Output is tri-state"]
    TS = 3,
}
impl From<GPIO29OUTCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO29OUTCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `GPIO29OUTCFG`"]
pub type GPIO29OUTCFG_R = crate::R<u8, GPIO29OUTCFG_A>;
impl GPIO29OUTCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO29OUTCFG_A {
        match self.bits {
            0 => GPIO29OUTCFG_A::DIS,
            1 => GPIO29OUTCFG_A::PUSHPULL,
            2 => GPIO29OUTCFG_A::OD,
            3 => GPIO29OUTCFG_A::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == GPIO29OUTCFG_A::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == GPIO29OUTCFG_A::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        *self == GPIO29OUTCFG_A::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        *self == GPIO29OUTCFG_A::TS
    }
}
#[doc = "Write proxy for field `GPIO29OUTCFG`"]
pub struct GPIO29OUTCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO29OUTCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO29OUTCFG_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Output disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO29OUTCFG_A::DIS)
    }
    #[doc = "Output is push-pull"]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO29OUTCFG_A::PUSHPULL)
    }
    #[doc = "Output is open drain"]
    #[inline(always)]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO29OUTCFG_A::OD)
    }
    #[doc = "Output is tri-state"]
    #[inline(always)]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO29OUTCFG_A::TS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 21)) | (((value as u32) & 0x03) << 21);
        self.w
    }
}
#[doc = "GPIO29 input enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO29INCFG_A {
    #[doc = "0: Read the GPIO pin data"]
    READ = 0,
    #[doc = "1: Readback will always be zero"]
    RDZERO = 1,
}
impl From<GPIO29INCFG_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO29INCFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIO29INCFG`"]
pub type GPIO29INCFG_R = crate::R<bool, GPIO29INCFG_A>;
impl GPIO29INCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO29INCFG_A {
        match self.bits {
            false => GPIO29INCFG_A::READ,
            true => GPIO29INCFG_A::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        *self == GPIO29INCFG_A::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline(always)]
    pub fn is_rdzero(&self) -> bool {
        *self == GPIO29INCFG_A::RDZERO
    }
}
#[doc = "Write proxy for field `GPIO29INCFG`"]
pub struct GPIO29INCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO29INCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO29INCFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Read the GPIO pin data"]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO29INCFG_A::READ)
    }
    #[doc = "Readback will always be zero"]
    #[inline(always)]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO29INCFG_A::RDZERO)
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
#[doc = "GPIO28 interrupt direction.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO28INTD_A {
    #[doc = "0: Interrupt on low to high GPIO transition"]
    INTLH = 0,
    #[doc = "1: Interrupt on high to low GPIO transition"]
    INTHL = 1,
}
impl From<GPIO28INTD_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO28INTD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIO28INTD`"]
pub type GPIO28INTD_R = crate::R<bool, GPIO28INTD_A>;
impl GPIO28INTD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO28INTD_A {
        match self.bits {
            false => GPIO28INTD_A::INTLH,
            true => GPIO28INTD_A::INTHL,
        }
    }
    #[doc = "Checks if the value of the field is `INTLH`"]
    #[inline(always)]
    pub fn is_intlh(&self) -> bool {
        *self == GPIO28INTD_A::INTLH
    }
    #[doc = "Checks if the value of the field is `INTHL`"]
    #[inline(always)]
    pub fn is_inthl(&self) -> bool {
        *self == GPIO28INTD_A::INTHL
    }
}
#[doc = "Write proxy for field `GPIO28INTD`"]
pub struct GPIO28INTD_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO28INTD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO28INTD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt on low to high GPIO transition"]
    #[inline(always)]
    pub fn intlh(self) -> &'a mut W {
        self.variant(GPIO28INTD_A::INTLH)
    }
    #[doc = "Interrupt on high to low GPIO transition"]
    #[inline(always)]
    pub fn inthl(self) -> &'a mut W {
        self.variant(GPIO28INTD_A::INTHL)
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
#[doc = "GPIO28 output configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GPIO28OUTCFG_A {
    #[doc = "0: Output disabled"]
    DIS = 0,
    #[doc = "1: Output is push-pull"]
    PUSHPULL = 1,
    #[doc = "2: Output is open drain"]
    OD = 2,
    #[doc = "3: Output is tri-state"]
    TS = 3,
}
impl From<GPIO28OUTCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO28OUTCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `GPIO28OUTCFG`"]
pub type GPIO28OUTCFG_R = crate::R<u8, GPIO28OUTCFG_A>;
impl GPIO28OUTCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO28OUTCFG_A {
        match self.bits {
            0 => GPIO28OUTCFG_A::DIS,
            1 => GPIO28OUTCFG_A::PUSHPULL,
            2 => GPIO28OUTCFG_A::OD,
            3 => GPIO28OUTCFG_A::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == GPIO28OUTCFG_A::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == GPIO28OUTCFG_A::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        *self == GPIO28OUTCFG_A::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        *self == GPIO28OUTCFG_A::TS
    }
}
#[doc = "Write proxy for field `GPIO28OUTCFG`"]
pub struct GPIO28OUTCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO28OUTCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO28OUTCFG_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Output disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO28OUTCFG_A::DIS)
    }
    #[doc = "Output is push-pull"]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO28OUTCFG_A::PUSHPULL)
    }
    #[doc = "Output is open drain"]
    #[inline(always)]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO28OUTCFG_A::OD)
    }
    #[doc = "Output is tri-state"]
    #[inline(always)]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO28OUTCFG_A::TS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 17)) | (((value as u32) & 0x03) << 17);
        self.w
    }
}
#[doc = "GPIO28 input enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO28INCFG_A {
    #[doc = "0: Read the GPIO pin data"]
    READ = 0,
    #[doc = "1: Readback will always be zero"]
    RDZERO = 1,
}
impl From<GPIO28INCFG_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO28INCFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIO28INCFG`"]
pub type GPIO28INCFG_R = crate::R<bool, GPIO28INCFG_A>;
impl GPIO28INCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO28INCFG_A {
        match self.bits {
            false => GPIO28INCFG_A::READ,
            true => GPIO28INCFG_A::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        *self == GPIO28INCFG_A::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline(always)]
    pub fn is_rdzero(&self) -> bool {
        *self == GPIO28INCFG_A::RDZERO
    }
}
#[doc = "Write proxy for field `GPIO28INCFG`"]
pub struct GPIO28INCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO28INCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO28INCFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Read the GPIO pin data"]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO28INCFG_A::READ)
    }
    #[doc = "Readback will always be zero"]
    #[inline(always)]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO28INCFG_A::RDZERO)
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
#[doc = "GPIO27 interrupt direction.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO27INTD_A {
    #[doc = "0: Interrupt on low to high GPIO transition"]
    INTLH = 0,
    #[doc = "1: Interrupt on high to low GPIO transition"]
    INTHL = 1,
}
impl From<GPIO27INTD_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO27INTD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIO27INTD`"]
pub type GPIO27INTD_R = crate::R<bool, GPIO27INTD_A>;
impl GPIO27INTD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO27INTD_A {
        match self.bits {
            false => GPIO27INTD_A::INTLH,
            true => GPIO27INTD_A::INTHL,
        }
    }
    #[doc = "Checks if the value of the field is `INTLH`"]
    #[inline(always)]
    pub fn is_intlh(&self) -> bool {
        *self == GPIO27INTD_A::INTLH
    }
    #[doc = "Checks if the value of the field is `INTHL`"]
    #[inline(always)]
    pub fn is_inthl(&self) -> bool {
        *self == GPIO27INTD_A::INTHL
    }
}
#[doc = "Write proxy for field `GPIO27INTD`"]
pub struct GPIO27INTD_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO27INTD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO27INTD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt on low to high GPIO transition"]
    #[inline(always)]
    pub fn intlh(self) -> &'a mut W {
        self.variant(GPIO27INTD_A::INTLH)
    }
    #[doc = "Interrupt on high to low GPIO transition"]
    #[inline(always)]
    pub fn inthl(self) -> &'a mut W {
        self.variant(GPIO27INTD_A::INTHL)
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
#[doc = "GPIO27 output configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GPIO27OUTCFG_A {
    #[doc = "0: Output disabled"]
    DIS = 0,
    #[doc = "1: Output is push-pull"]
    PUSHPULL = 1,
    #[doc = "2: Output is open drain"]
    OD = 2,
    #[doc = "3: Output is tri-state"]
    TS = 3,
}
impl From<GPIO27OUTCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO27OUTCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `GPIO27OUTCFG`"]
pub type GPIO27OUTCFG_R = crate::R<u8, GPIO27OUTCFG_A>;
impl GPIO27OUTCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO27OUTCFG_A {
        match self.bits {
            0 => GPIO27OUTCFG_A::DIS,
            1 => GPIO27OUTCFG_A::PUSHPULL,
            2 => GPIO27OUTCFG_A::OD,
            3 => GPIO27OUTCFG_A::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == GPIO27OUTCFG_A::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == GPIO27OUTCFG_A::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        *self == GPIO27OUTCFG_A::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        *self == GPIO27OUTCFG_A::TS
    }
}
#[doc = "Write proxy for field `GPIO27OUTCFG`"]
pub struct GPIO27OUTCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO27OUTCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO27OUTCFG_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Output disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO27OUTCFG_A::DIS)
    }
    #[doc = "Output is push-pull"]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO27OUTCFG_A::PUSHPULL)
    }
    #[doc = "Output is open drain"]
    #[inline(always)]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO27OUTCFG_A::OD)
    }
    #[doc = "Output is tri-state"]
    #[inline(always)]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO27OUTCFG_A::TS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 13)) | (((value as u32) & 0x03) << 13);
        self.w
    }
}
#[doc = "GPIO27 input enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO27INCFG_A {
    #[doc = "0: Read the GPIO pin data"]
    READ = 0,
    #[doc = "1: Readback will always be zero"]
    RDZERO = 1,
}
impl From<GPIO27INCFG_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO27INCFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIO27INCFG`"]
pub type GPIO27INCFG_R = crate::R<bool, GPIO27INCFG_A>;
impl GPIO27INCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO27INCFG_A {
        match self.bits {
            false => GPIO27INCFG_A::READ,
            true => GPIO27INCFG_A::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        *self == GPIO27INCFG_A::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline(always)]
    pub fn is_rdzero(&self) -> bool {
        *self == GPIO27INCFG_A::RDZERO
    }
}
#[doc = "Write proxy for field `GPIO27INCFG`"]
pub struct GPIO27INCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO27INCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO27INCFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Read the GPIO pin data"]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO27INCFG_A::READ)
    }
    #[doc = "Readback will always be zero"]
    #[inline(always)]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO27INCFG_A::RDZERO)
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
#[doc = "GPIO26 interrupt direction.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO26INTD_A {
    #[doc = "0: Interrupt on low to high GPIO transition"]
    INTLH = 0,
    #[doc = "1: Interrupt on high to low GPIO transition"]
    INTHL = 1,
}
impl From<GPIO26INTD_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO26INTD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIO26INTD`"]
pub type GPIO26INTD_R = crate::R<bool, GPIO26INTD_A>;
impl GPIO26INTD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO26INTD_A {
        match self.bits {
            false => GPIO26INTD_A::INTLH,
            true => GPIO26INTD_A::INTHL,
        }
    }
    #[doc = "Checks if the value of the field is `INTLH`"]
    #[inline(always)]
    pub fn is_intlh(&self) -> bool {
        *self == GPIO26INTD_A::INTLH
    }
    #[doc = "Checks if the value of the field is `INTHL`"]
    #[inline(always)]
    pub fn is_inthl(&self) -> bool {
        *self == GPIO26INTD_A::INTHL
    }
}
#[doc = "Write proxy for field `GPIO26INTD`"]
pub struct GPIO26INTD_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO26INTD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO26INTD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt on low to high GPIO transition"]
    #[inline(always)]
    pub fn intlh(self) -> &'a mut W {
        self.variant(GPIO26INTD_A::INTLH)
    }
    #[doc = "Interrupt on high to low GPIO transition"]
    #[inline(always)]
    pub fn inthl(self) -> &'a mut W {
        self.variant(GPIO26INTD_A::INTHL)
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
#[doc = "GPIO26 output configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GPIO26OUTCFG_A {
    #[doc = "0: Output disabled"]
    DIS = 0,
    #[doc = "1: Output is push-pull"]
    PUSHPULL = 1,
    #[doc = "2: Output is open drain"]
    OD = 2,
    #[doc = "3: Output is tri-state"]
    TS = 3,
}
impl From<GPIO26OUTCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO26OUTCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `GPIO26OUTCFG`"]
pub type GPIO26OUTCFG_R = crate::R<u8, GPIO26OUTCFG_A>;
impl GPIO26OUTCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO26OUTCFG_A {
        match self.bits {
            0 => GPIO26OUTCFG_A::DIS,
            1 => GPIO26OUTCFG_A::PUSHPULL,
            2 => GPIO26OUTCFG_A::OD,
            3 => GPIO26OUTCFG_A::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == GPIO26OUTCFG_A::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == GPIO26OUTCFG_A::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        *self == GPIO26OUTCFG_A::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        *self == GPIO26OUTCFG_A::TS
    }
}
#[doc = "Write proxy for field `GPIO26OUTCFG`"]
pub struct GPIO26OUTCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO26OUTCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO26OUTCFG_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Output disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO26OUTCFG_A::DIS)
    }
    #[doc = "Output is push-pull"]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO26OUTCFG_A::PUSHPULL)
    }
    #[doc = "Output is open drain"]
    #[inline(always)]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO26OUTCFG_A::OD)
    }
    #[doc = "Output is tri-state"]
    #[inline(always)]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO26OUTCFG_A::TS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 9)) | (((value as u32) & 0x03) << 9);
        self.w
    }
}
#[doc = "GPIO26 input enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO26INCFG_A {
    #[doc = "0: Read the GPIO pin data"]
    READ = 0,
    #[doc = "1: Readback will always be zero"]
    RDZERO = 1,
}
impl From<GPIO26INCFG_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO26INCFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIO26INCFG`"]
pub type GPIO26INCFG_R = crate::R<bool, GPIO26INCFG_A>;
impl GPIO26INCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO26INCFG_A {
        match self.bits {
            false => GPIO26INCFG_A::READ,
            true => GPIO26INCFG_A::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        *self == GPIO26INCFG_A::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline(always)]
    pub fn is_rdzero(&self) -> bool {
        *self == GPIO26INCFG_A::RDZERO
    }
}
#[doc = "Write proxy for field `GPIO26INCFG`"]
pub struct GPIO26INCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO26INCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO26INCFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Read the GPIO pin data"]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO26INCFG_A::READ)
    }
    #[doc = "Readback will always be zero"]
    #[inline(always)]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO26INCFG_A::RDZERO)
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
#[doc = "GPIO25 interrupt direction.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO25INTD_A {
    #[doc = "0: Interrupt on low to high GPIO transition"]
    INTLH = 0,
    #[doc = "1: Interrupt on high to low GPIO transition"]
    INTHL = 1,
}
impl From<GPIO25INTD_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO25INTD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIO25INTD`"]
pub type GPIO25INTD_R = crate::R<bool, GPIO25INTD_A>;
impl GPIO25INTD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO25INTD_A {
        match self.bits {
            false => GPIO25INTD_A::INTLH,
            true => GPIO25INTD_A::INTHL,
        }
    }
    #[doc = "Checks if the value of the field is `INTLH`"]
    #[inline(always)]
    pub fn is_intlh(&self) -> bool {
        *self == GPIO25INTD_A::INTLH
    }
    #[doc = "Checks if the value of the field is `INTHL`"]
    #[inline(always)]
    pub fn is_inthl(&self) -> bool {
        *self == GPIO25INTD_A::INTHL
    }
}
#[doc = "Write proxy for field `GPIO25INTD`"]
pub struct GPIO25INTD_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO25INTD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO25INTD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt on low to high GPIO transition"]
    #[inline(always)]
    pub fn intlh(self) -> &'a mut W {
        self.variant(GPIO25INTD_A::INTLH)
    }
    #[doc = "Interrupt on high to low GPIO transition"]
    #[inline(always)]
    pub fn inthl(self) -> &'a mut W {
        self.variant(GPIO25INTD_A::INTHL)
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
#[doc = "GPIO25 output configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GPIO25OUTCFG_A {
    #[doc = "0: Output disabled"]
    DIS = 0,
    #[doc = "1: Output is push-pull"]
    PUSHPULL = 1,
    #[doc = "2: Output is open drain"]
    OD = 2,
    #[doc = "3: Output is tri-state"]
    TS = 3,
}
impl From<GPIO25OUTCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO25OUTCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `GPIO25OUTCFG`"]
pub type GPIO25OUTCFG_R = crate::R<u8, GPIO25OUTCFG_A>;
impl GPIO25OUTCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO25OUTCFG_A {
        match self.bits {
            0 => GPIO25OUTCFG_A::DIS,
            1 => GPIO25OUTCFG_A::PUSHPULL,
            2 => GPIO25OUTCFG_A::OD,
            3 => GPIO25OUTCFG_A::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == GPIO25OUTCFG_A::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == GPIO25OUTCFG_A::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        *self == GPIO25OUTCFG_A::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        *self == GPIO25OUTCFG_A::TS
    }
}
#[doc = "Write proxy for field `GPIO25OUTCFG`"]
pub struct GPIO25OUTCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO25OUTCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO25OUTCFG_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Output disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO25OUTCFG_A::DIS)
    }
    #[doc = "Output is push-pull"]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO25OUTCFG_A::PUSHPULL)
    }
    #[doc = "Output is open drain"]
    #[inline(always)]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO25OUTCFG_A::OD)
    }
    #[doc = "Output is tri-state"]
    #[inline(always)]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO25OUTCFG_A::TS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | (((value as u32) & 0x03) << 5);
        self.w
    }
}
#[doc = "GPIO25 input enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO25INCFG_A {
    #[doc = "0: Read the GPIO pin data"]
    READ = 0,
    #[doc = "1: Readback will always be zero"]
    RDZERO = 1,
}
impl From<GPIO25INCFG_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO25INCFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIO25INCFG`"]
pub type GPIO25INCFG_R = crate::R<bool, GPIO25INCFG_A>;
impl GPIO25INCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO25INCFG_A {
        match self.bits {
            false => GPIO25INCFG_A::READ,
            true => GPIO25INCFG_A::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        *self == GPIO25INCFG_A::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline(always)]
    pub fn is_rdzero(&self) -> bool {
        *self == GPIO25INCFG_A::RDZERO
    }
}
#[doc = "Write proxy for field `GPIO25INCFG`"]
pub struct GPIO25INCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO25INCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO25INCFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Read the GPIO pin data"]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO25INCFG_A::READ)
    }
    #[doc = "Readback will always be zero"]
    #[inline(always)]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO25INCFG_A::RDZERO)
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
#[doc = "GPIO24 interrupt direction.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO24INTD_A {
    #[doc = "0: Interrupt on low to high GPIO transition"]
    INTLH = 0,
    #[doc = "1: Interrupt on high to low GPIO transition"]
    INTHL = 1,
}
impl From<GPIO24INTD_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO24INTD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIO24INTD`"]
pub type GPIO24INTD_R = crate::R<bool, GPIO24INTD_A>;
impl GPIO24INTD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO24INTD_A {
        match self.bits {
            false => GPIO24INTD_A::INTLH,
            true => GPIO24INTD_A::INTHL,
        }
    }
    #[doc = "Checks if the value of the field is `INTLH`"]
    #[inline(always)]
    pub fn is_intlh(&self) -> bool {
        *self == GPIO24INTD_A::INTLH
    }
    #[doc = "Checks if the value of the field is `INTHL`"]
    #[inline(always)]
    pub fn is_inthl(&self) -> bool {
        *self == GPIO24INTD_A::INTHL
    }
}
#[doc = "Write proxy for field `GPIO24INTD`"]
pub struct GPIO24INTD_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO24INTD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO24INTD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt on low to high GPIO transition"]
    #[inline(always)]
    pub fn intlh(self) -> &'a mut W {
        self.variant(GPIO24INTD_A::INTLH)
    }
    #[doc = "Interrupt on high to low GPIO transition"]
    #[inline(always)]
    pub fn inthl(self) -> &'a mut W {
        self.variant(GPIO24INTD_A::INTHL)
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
#[doc = "GPIO24 output configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GPIO24OUTCFG_A {
    #[doc = "0: Output disabled"]
    DIS = 0,
    #[doc = "1: Output is push-pull"]
    PUSHPULL = 1,
    #[doc = "2: Output is open drain"]
    OD = 2,
    #[doc = "3: Output is tri-state"]
    TS = 3,
}
impl From<GPIO24OUTCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO24OUTCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `GPIO24OUTCFG`"]
pub type GPIO24OUTCFG_R = crate::R<u8, GPIO24OUTCFG_A>;
impl GPIO24OUTCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO24OUTCFG_A {
        match self.bits {
            0 => GPIO24OUTCFG_A::DIS,
            1 => GPIO24OUTCFG_A::PUSHPULL,
            2 => GPIO24OUTCFG_A::OD,
            3 => GPIO24OUTCFG_A::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == GPIO24OUTCFG_A::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == GPIO24OUTCFG_A::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        *self == GPIO24OUTCFG_A::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        *self == GPIO24OUTCFG_A::TS
    }
}
#[doc = "Write proxy for field `GPIO24OUTCFG`"]
pub struct GPIO24OUTCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO24OUTCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO24OUTCFG_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Output disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO24OUTCFG_A::DIS)
    }
    #[doc = "Output is push-pull"]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO24OUTCFG_A::PUSHPULL)
    }
    #[doc = "Output is open drain"]
    #[inline(always)]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO24OUTCFG_A::OD)
    }
    #[doc = "Output is tri-state"]
    #[inline(always)]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO24OUTCFG_A::TS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | (((value as u32) & 0x03) << 1);
        self.w
    }
}
#[doc = "GPIO24 input enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO24INCFG_A {
    #[doc = "0: Read the GPIO pin data"]
    READ = 0,
    #[doc = "1: Readback will always be zero"]
    RDZERO = 1,
}
impl From<GPIO24INCFG_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO24INCFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIO24INCFG`"]
pub type GPIO24INCFG_R = crate::R<bool, GPIO24INCFG_A>;
impl GPIO24INCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO24INCFG_A {
        match self.bits {
            false => GPIO24INCFG_A::READ,
            true => GPIO24INCFG_A::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        *self == GPIO24INCFG_A::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline(always)]
    pub fn is_rdzero(&self) -> bool {
        *self == GPIO24INCFG_A::RDZERO
    }
}
#[doc = "Write proxy for field `GPIO24INCFG`"]
pub struct GPIO24INCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO24INCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO24INCFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Read the GPIO pin data"]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO24INCFG_A::READ)
    }
    #[doc = "Readback will always be zero"]
    #[inline(always)]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO24INCFG_A::RDZERO)
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
    #[doc = "Bit 31 - GPIO31 interrupt direction."]
    #[inline(always)]
    pub fn gpio31intd(&self) -> GPIO31INTD_R {
        GPIO31INTD_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 29:30 - GPIO31 output configuration."]
    #[inline(always)]
    pub fn gpio31outcfg(&self) -> GPIO31OUTCFG_R {
        GPIO31OUTCFG_R::new(((self.bits >> 29) & 0x03) as u8)
    }
    #[doc = "Bit 28 - GPIO31 input enable."]
    #[inline(always)]
    pub fn gpio31incfg(&self) -> GPIO31INCFG_R {
        GPIO31INCFG_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - GPIO30 interrupt direction."]
    #[inline(always)]
    pub fn gpio30intd(&self) -> GPIO30INTD_R {
        GPIO30INTD_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bits 25:26 - GPIO30 output configuration."]
    #[inline(always)]
    pub fn gpio30outcfg(&self) -> GPIO30OUTCFG_R {
        GPIO30OUTCFG_R::new(((self.bits >> 25) & 0x03) as u8)
    }
    #[doc = "Bit 24 - GPIO30 input enable."]
    #[inline(always)]
    pub fn gpio30incfg(&self) -> GPIO30INCFG_R {
        GPIO30INCFG_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - GPIO29 interrupt direction."]
    #[inline(always)]
    pub fn gpio29intd(&self) -> GPIO29INTD_R {
        GPIO29INTD_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 21:22 - GPIO29 output configuration."]
    #[inline(always)]
    pub fn gpio29outcfg(&self) -> GPIO29OUTCFG_R {
        GPIO29OUTCFG_R::new(((self.bits >> 21) & 0x03) as u8)
    }
    #[doc = "Bit 20 - GPIO29 input enable."]
    #[inline(always)]
    pub fn gpio29incfg(&self) -> GPIO29INCFG_R {
        GPIO29INCFG_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - GPIO28 interrupt direction."]
    #[inline(always)]
    pub fn gpio28intd(&self) -> GPIO28INTD_R {
        GPIO28INTD_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bits 17:18 - GPIO28 output configuration."]
    #[inline(always)]
    pub fn gpio28outcfg(&self) -> GPIO28OUTCFG_R {
        GPIO28OUTCFG_R::new(((self.bits >> 17) & 0x03) as u8)
    }
    #[doc = "Bit 16 - GPIO28 input enable."]
    #[inline(always)]
    pub fn gpio28incfg(&self) -> GPIO28INCFG_R {
        GPIO28INCFG_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - GPIO27 interrupt direction."]
    #[inline(always)]
    pub fn gpio27intd(&self) -> GPIO27INTD_R {
        GPIO27INTD_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 13:14 - GPIO27 output configuration."]
    #[inline(always)]
    pub fn gpio27outcfg(&self) -> GPIO27OUTCFG_R {
        GPIO27OUTCFG_R::new(((self.bits >> 13) & 0x03) as u8)
    }
    #[doc = "Bit 12 - GPIO27 input enable."]
    #[inline(always)]
    pub fn gpio27incfg(&self) -> GPIO27INCFG_R {
        GPIO27INCFG_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - GPIO26 interrupt direction."]
    #[inline(always)]
    pub fn gpio26intd(&self) -> GPIO26INTD_R {
        GPIO26INTD_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 9:10 - GPIO26 output configuration."]
    #[inline(always)]
    pub fn gpio26outcfg(&self) -> GPIO26OUTCFG_R {
        GPIO26OUTCFG_R::new(((self.bits >> 9) & 0x03) as u8)
    }
    #[doc = "Bit 8 - GPIO26 input enable."]
    #[inline(always)]
    pub fn gpio26incfg(&self) -> GPIO26INCFG_R {
        GPIO26INCFG_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - GPIO25 interrupt direction."]
    #[inline(always)]
    pub fn gpio25intd(&self) -> GPIO25INTD_R {
        GPIO25INTD_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 5:6 - GPIO25 output configuration."]
    #[inline(always)]
    pub fn gpio25outcfg(&self) -> GPIO25OUTCFG_R {
        GPIO25OUTCFG_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bit 4 - GPIO25 input enable."]
    #[inline(always)]
    pub fn gpio25incfg(&self) -> GPIO25INCFG_R {
        GPIO25INCFG_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - GPIO24 interrupt direction."]
    #[inline(always)]
    pub fn gpio24intd(&self) -> GPIO24INTD_R {
        GPIO24INTD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - GPIO24 output configuration."]
    #[inline(always)]
    pub fn gpio24outcfg(&self) -> GPIO24OUTCFG_R {
        GPIO24OUTCFG_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bit 0 - GPIO24 input enable."]
    #[inline(always)]
    pub fn gpio24incfg(&self) -> GPIO24INCFG_R {
        GPIO24INCFG_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - GPIO31 interrupt direction."]
    #[inline(always)]
    pub fn gpio31intd(&mut self) -> GPIO31INTD_W {
        GPIO31INTD_W { w: self }
    }
    #[doc = "Bits 29:30 - GPIO31 output configuration."]
    #[inline(always)]
    pub fn gpio31outcfg(&mut self) -> GPIO31OUTCFG_W {
        GPIO31OUTCFG_W { w: self }
    }
    #[doc = "Bit 28 - GPIO31 input enable."]
    #[inline(always)]
    pub fn gpio31incfg(&mut self) -> GPIO31INCFG_W {
        GPIO31INCFG_W { w: self }
    }
    #[doc = "Bit 27 - GPIO30 interrupt direction."]
    #[inline(always)]
    pub fn gpio30intd(&mut self) -> GPIO30INTD_W {
        GPIO30INTD_W { w: self }
    }
    #[doc = "Bits 25:26 - GPIO30 output configuration."]
    #[inline(always)]
    pub fn gpio30outcfg(&mut self) -> GPIO30OUTCFG_W {
        GPIO30OUTCFG_W { w: self }
    }
    #[doc = "Bit 24 - GPIO30 input enable."]
    #[inline(always)]
    pub fn gpio30incfg(&mut self) -> GPIO30INCFG_W {
        GPIO30INCFG_W { w: self }
    }
    #[doc = "Bit 23 - GPIO29 interrupt direction."]
    #[inline(always)]
    pub fn gpio29intd(&mut self) -> GPIO29INTD_W {
        GPIO29INTD_W { w: self }
    }
    #[doc = "Bits 21:22 - GPIO29 output configuration."]
    #[inline(always)]
    pub fn gpio29outcfg(&mut self) -> GPIO29OUTCFG_W {
        GPIO29OUTCFG_W { w: self }
    }
    #[doc = "Bit 20 - GPIO29 input enable."]
    #[inline(always)]
    pub fn gpio29incfg(&mut self) -> GPIO29INCFG_W {
        GPIO29INCFG_W { w: self }
    }
    #[doc = "Bit 19 - GPIO28 interrupt direction."]
    #[inline(always)]
    pub fn gpio28intd(&mut self) -> GPIO28INTD_W {
        GPIO28INTD_W { w: self }
    }
    #[doc = "Bits 17:18 - GPIO28 output configuration."]
    #[inline(always)]
    pub fn gpio28outcfg(&mut self) -> GPIO28OUTCFG_W {
        GPIO28OUTCFG_W { w: self }
    }
    #[doc = "Bit 16 - GPIO28 input enable."]
    #[inline(always)]
    pub fn gpio28incfg(&mut self) -> GPIO28INCFG_W {
        GPIO28INCFG_W { w: self }
    }
    #[doc = "Bit 15 - GPIO27 interrupt direction."]
    #[inline(always)]
    pub fn gpio27intd(&mut self) -> GPIO27INTD_W {
        GPIO27INTD_W { w: self }
    }
    #[doc = "Bits 13:14 - GPIO27 output configuration."]
    #[inline(always)]
    pub fn gpio27outcfg(&mut self) -> GPIO27OUTCFG_W {
        GPIO27OUTCFG_W { w: self }
    }
    #[doc = "Bit 12 - GPIO27 input enable."]
    #[inline(always)]
    pub fn gpio27incfg(&mut self) -> GPIO27INCFG_W {
        GPIO27INCFG_W { w: self }
    }
    #[doc = "Bit 11 - GPIO26 interrupt direction."]
    #[inline(always)]
    pub fn gpio26intd(&mut self) -> GPIO26INTD_W {
        GPIO26INTD_W { w: self }
    }
    #[doc = "Bits 9:10 - GPIO26 output configuration."]
    #[inline(always)]
    pub fn gpio26outcfg(&mut self) -> GPIO26OUTCFG_W {
        GPIO26OUTCFG_W { w: self }
    }
    #[doc = "Bit 8 - GPIO26 input enable."]
    #[inline(always)]
    pub fn gpio26incfg(&mut self) -> GPIO26INCFG_W {
        GPIO26INCFG_W { w: self }
    }
    #[doc = "Bit 7 - GPIO25 interrupt direction."]
    #[inline(always)]
    pub fn gpio25intd(&mut self) -> GPIO25INTD_W {
        GPIO25INTD_W { w: self }
    }
    #[doc = "Bits 5:6 - GPIO25 output configuration."]
    #[inline(always)]
    pub fn gpio25outcfg(&mut self) -> GPIO25OUTCFG_W {
        GPIO25OUTCFG_W { w: self }
    }
    #[doc = "Bit 4 - GPIO25 input enable."]
    #[inline(always)]
    pub fn gpio25incfg(&mut self) -> GPIO25INCFG_W {
        GPIO25INCFG_W { w: self }
    }
    #[doc = "Bit 3 - GPIO24 interrupt direction."]
    #[inline(always)]
    pub fn gpio24intd(&mut self) -> GPIO24INTD_W {
        GPIO24INTD_W { w: self }
    }
    #[doc = "Bits 1:2 - GPIO24 output configuration."]
    #[inline(always)]
    pub fn gpio24outcfg(&mut self) -> GPIO24OUTCFG_W {
        GPIO24OUTCFG_W { w: self }
    }
    #[doc = "Bit 0 - GPIO24 input enable."]
    #[inline(always)]
    pub fn gpio24incfg(&mut self) -> GPIO24INCFG_W {
        GPIO24INCFG_W { w: self }
    }
}
