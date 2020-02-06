#[doc = "Reader of register CFGG"]
pub type R = crate::R<u32, super::CFGG>;
#[doc = "Writer for register CFGG"]
pub type W = crate::W<u32, super::CFGG>;
#[doc = "Register CFGG `reset()`'s with value 0"]
impl crate::ResetValue for super::CFGG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "GPIO49 interrupt direction.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO49INTD_A {
    #[doc = "0: Interrupt on low to high GPIO transition"]
    INTLH = 0,
    #[doc = "1: Interrupt on high to low GPIO transition"]
    INTHL = 1,
}
impl From<GPIO49INTD_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO49INTD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIO49INTD`"]
pub type GPIO49INTD_R = crate::R<bool, GPIO49INTD_A>;
impl GPIO49INTD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO49INTD_A {
        match self.bits {
            false => GPIO49INTD_A::INTLH,
            true => GPIO49INTD_A::INTHL,
        }
    }
    #[doc = "Checks if the value of the field is `INTLH`"]
    #[inline(always)]
    pub fn is_intlh(&self) -> bool {
        *self == GPIO49INTD_A::INTLH
    }
    #[doc = "Checks if the value of the field is `INTHL`"]
    #[inline(always)]
    pub fn is_inthl(&self) -> bool {
        *self == GPIO49INTD_A::INTHL
    }
}
#[doc = "Write proxy for field `GPIO49INTD`"]
pub struct GPIO49INTD_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO49INTD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO49INTD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt on low to high GPIO transition"]
    #[inline(always)]
    pub fn intlh(self) -> &'a mut W {
        self.variant(GPIO49INTD_A::INTLH)
    }
    #[doc = "Interrupt on high to low GPIO transition"]
    #[inline(always)]
    pub fn inthl(self) -> &'a mut W {
        self.variant(GPIO49INTD_A::INTHL)
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
#[doc = "GPIO49 output configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GPIO49OUTCFG_A {
    #[doc = "0: Output disabled"]
    DIS = 0,
    #[doc = "1: Output is push-pull"]
    PUSHPULL = 1,
    #[doc = "2: Output is open drain"]
    OD = 2,
    #[doc = "3: Output is tri-state"]
    TS = 3,
}
impl From<GPIO49OUTCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO49OUTCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `GPIO49OUTCFG`"]
pub type GPIO49OUTCFG_R = crate::R<u8, GPIO49OUTCFG_A>;
impl GPIO49OUTCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO49OUTCFG_A {
        match self.bits {
            0 => GPIO49OUTCFG_A::DIS,
            1 => GPIO49OUTCFG_A::PUSHPULL,
            2 => GPIO49OUTCFG_A::OD,
            3 => GPIO49OUTCFG_A::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == GPIO49OUTCFG_A::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == GPIO49OUTCFG_A::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        *self == GPIO49OUTCFG_A::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        *self == GPIO49OUTCFG_A::TS
    }
}
#[doc = "Write proxy for field `GPIO49OUTCFG`"]
pub struct GPIO49OUTCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO49OUTCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO49OUTCFG_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Output disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO49OUTCFG_A::DIS)
    }
    #[doc = "Output is push-pull"]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO49OUTCFG_A::PUSHPULL)
    }
    #[doc = "Output is open drain"]
    #[inline(always)]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO49OUTCFG_A::OD)
    }
    #[doc = "Output is tri-state"]
    #[inline(always)]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO49OUTCFG_A::TS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | (((value as u32) & 0x03) << 5);
        self.w
    }
}
#[doc = "GPIO49 input enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO49INCFG_A {
    #[doc = "0: Read the GPIO pin data"]
    READ = 0,
    #[doc = "1: Readback will always be zero"]
    RDZERO = 1,
}
impl From<GPIO49INCFG_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO49INCFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIO49INCFG`"]
pub type GPIO49INCFG_R = crate::R<bool, GPIO49INCFG_A>;
impl GPIO49INCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO49INCFG_A {
        match self.bits {
            false => GPIO49INCFG_A::READ,
            true => GPIO49INCFG_A::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        *self == GPIO49INCFG_A::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline(always)]
    pub fn is_rdzero(&self) -> bool {
        *self == GPIO49INCFG_A::RDZERO
    }
}
#[doc = "Write proxy for field `GPIO49INCFG`"]
pub struct GPIO49INCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO49INCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO49INCFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Read the GPIO pin data"]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO49INCFG_A::READ)
    }
    #[doc = "Readback will always be zero"]
    #[inline(always)]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO49INCFG_A::RDZERO)
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
#[doc = "GPIO48 interrupt direction.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO48INTD_A {
    #[doc = "0: Interrupt on low to high GPIO transition"]
    INTLH = 0,
    #[doc = "1: Interrupt on high to low GPIO transition"]
    INTHL = 1,
}
impl From<GPIO48INTD_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO48INTD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIO48INTD`"]
pub type GPIO48INTD_R = crate::R<bool, GPIO48INTD_A>;
impl GPIO48INTD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO48INTD_A {
        match self.bits {
            false => GPIO48INTD_A::INTLH,
            true => GPIO48INTD_A::INTHL,
        }
    }
    #[doc = "Checks if the value of the field is `INTLH`"]
    #[inline(always)]
    pub fn is_intlh(&self) -> bool {
        *self == GPIO48INTD_A::INTLH
    }
    #[doc = "Checks if the value of the field is `INTHL`"]
    #[inline(always)]
    pub fn is_inthl(&self) -> bool {
        *self == GPIO48INTD_A::INTHL
    }
}
#[doc = "Write proxy for field `GPIO48INTD`"]
pub struct GPIO48INTD_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO48INTD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO48INTD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt on low to high GPIO transition"]
    #[inline(always)]
    pub fn intlh(self) -> &'a mut W {
        self.variant(GPIO48INTD_A::INTLH)
    }
    #[doc = "Interrupt on high to low GPIO transition"]
    #[inline(always)]
    pub fn inthl(self) -> &'a mut W {
        self.variant(GPIO48INTD_A::INTHL)
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
#[doc = "GPIO48 output configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GPIO48OUTCFG_A {
    #[doc = "0: Output disabled"]
    DIS = 0,
    #[doc = "1: Output is push-pull"]
    PUSHPULL = 1,
    #[doc = "2: Output is open drain"]
    OD = 2,
    #[doc = "3: Output is tri-state"]
    TS = 3,
}
impl From<GPIO48OUTCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO48OUTCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `GPIO48OUTCFG`"]
pub type GPIO48OUTCFG_R = crate::R<u8, GPIO48OUTCFG_A>;
impl GPIO48OUTCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO48OUTCFG_A {
        match self.bits {
            0 => GPIO48OUTCFG_A::DIS,
            1 => GPIO48OUTCFG_A::PUSHPULL,
            2 => GPIO48OUTCFG_A::OD,
            3 => GPIO48OUTCFG_A::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == GPIO48OUTCFG_A::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == GPIO48OUTCFG_A::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        *self == GPIO48OUTCFG_A::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        *self == GPIO48OUTCFG_A::TS
    }
}
#[doc = "Write proxy for field `GPIO48OUTCFG`"]
pub struct GPIO48OUTCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO48OUTCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO48OUTCFG_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Output disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO48OUTCFG_A::DIS)
    }
    #[doc = "Output is push-pull"]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO48OUTCFG_A::PUSHPULL)
    }
    #[doc = "Output is open drain"]
    #[inline(always)]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO48OUTCFG_A::OD)
    }
    #[doc = "Output is tri-state"]
    #[inline(always)]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO48OUTCFG_A::TS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | (((value as u32) & 0x03) << 1);
        self.w
    }
}
#[doc = "GPIO48 input enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO48INCFG_A {
    #[doc = "0: Read the GPIO pin data"]
    READ = 0,
    #[doc = "1: Readback will always be zero"]
    RDZERO = 1,
}
impl From<GPIO48INCFG_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO48INCFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIO48INCFG`"]
pub type GPIO48INCFG_R = crate::R<bool, GPIO48INCFG_A>;
impl GPIO48INCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO48INCFG_A {
        match self.bits {
            false => GPIO48INCFG_A::READ,
            true => GPIO48INCFG_A::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        *self == GPIO48INCFG_A::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline(always)]
    pub fn is_rdzero(&self) -> bool {
        *self == GPIO48INCFG_A::RDZERO
    }
}
#[doc = "Write proxy for field `GPIO48INCFG`"]
pub struct GPIO48INCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO48INCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO48INCFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Read the GPIO pin data"]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO48INCFG_A::READ)
    }
    #[doc = "Readback will always be zero"]
    #[inline(always)]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO48INCFG_A::RDZERO)
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
    #[doc = "Bit 7 - GPIO49 interrupt direction."]
    #[inline(always)]
    pub fn gpio49intd(&self) -> GPIO49INTD_R {
        GPIO49INTD_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 5:6 - GPIO49 output configuration."]
    #[inline(always)]
    pub fn gpio49outcfg(&self) -> GPIO49OUTCFG_R {
        GPIO49OUTCFG_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bit 4 - GPIO49 input enable."]
    #[inline(always)]
    pub fn gpio49incfg(&self) -> GPIO49INCFG_R {
        GPIO49INCFG_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - GPIO48 interrupt direction."]
    #[inline(always)]
    pub fn gpio48intd(&self) -> GPIO48INTD_R {
        GPIO48INTD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - GPIO48 output configuration."]
    #[inline(always)]
    pub fn gpio48outcfg(&self) -> GPIO48OUTCFG_R {
        GPIO48OUTCFG_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bit 0 - GPIO48 input enable."]
    #[inline(always)]
    pub fn gpio48incfg(&self) -> GPIO48INCFG_R {
        GPIO48INCFG_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - GPIO49 interrupt direction."]
    #[inline(always)]
    pub fn gpio49intd(&mut self) -> GPIO49INTD_W {
        GPIO49INTD_W { w: self }
    }
    #[doc = "Bits 5:6 - GPIO49 output configuration."]
    #[inline(always)]
    pub fn gpio49outcfg(&mut self) -> GPIO49OUTCFG_W {
        GPIO49OUTCFG_W { w: self }
    }
    #[doc = "Bit 4 - GPIO49 input enable."]
    #[inline(always)]
    pub fn gpio49incfg(&mut self) -> GPIO49INCFG_W {
        GPIO49INCFG_W { w: self }
    }
    #[doc = "Bit 3 - GPIO48 interrupt direction."]
    #[inline(always)]
    pub fn gpio48intd(&mut self) -> GPIO48INTD_W {
        GPIO48INTD_W { w: self }
    }
    #[doc = "Bits 1:2 - GPIO48 output configuration."]
    #[inline(always)]
    pub fn gpio48outcfg(&mut self) -> GPIO48OUTCFG_W {
        GPIO48OUTCFG_W { w: self }
    }
    #[doc = "Bit 0 - GPIO48 input enable."]
    #[inline(always)]
    pub fn gpio48incfg(&mut self) -> GPIO48INCFG_W {
        GPIO48INCFG_W { w: self }
    }
}
