#[doc = "Reader of register PADREGL"]
pub type R = crate::R<u32, super::PADREGL>;
#[doc = "Writer for register PADREGL"]
pub type W = crate::W<u32, super::PADREGL>;
#[doc = "Register PADREGL `reset()`'s with value 0x1818_1818"]
impl crate::ResetValue for super::PADREGL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x1818_1818
    }
}
#[doc = "Pad 47 function select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD47FNCSEL_A {
    #[doc = "0: Pad disabled"]
    DIS = 0,
    #[doc = "1: Configure as the SPI channel 5 nCE signal from IOMSTR0"]
    M0NCE5 = 1,
    #[doc = "2: Configure as the input/output signal from CTIMER B2"]
    TCTB2 = 2,
    #[doc = "3: Configure as GPIO47"]
    GPIO47 = 3,
}
impl From<PAD47FNCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD47FNCSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PAD47FNCSEL`"]
pub type PAD47FNCSEL_R = crate::R<u8, PAD47FNCSEL_A>;
impl PAD47FNCSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD47FNCSEL_A {
        match self.bits {
            0 => PAD47FNCSEL_A::DIS,
            1 => PAD47FNCSEL_A::M0NCE5,
            2 => PAD47FNCSEL_A::TCTB2,
            3 => PAD47FNCSEL_A::GPIO47,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD47FNCSEL_A::DIS
    }
    #[doc = "Checks if the value of the field is `M0NCE5`"]
    #[inline(always)]
    pub fn is_m0n_ce5(&self) -> bool {
        *self == PAD47FNCSEL_A::M0NCE5
    }
    #[doc = "Checks if the value of the field is `TCTB2`"]
    #[inline(always)]
    pub fn is_tctb2(&self) -> bool {
        *self == PAD47FNCSEL_A::TCTB2
    }
    #[doc = "Checks if the value of the field is `GPIO47`"]
    #[inline(always)]
    pub fn is_gpio47(&self) -> bool {
        *self == PAD47FNCSEL_A::GPIO47
    }
}
#[doc = "Write proxy for field `PAD47FNCSEL`"]
pub struct PAD47FNCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD47FNCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD47FNCSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Pad disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD47FNCSEL_A::DIS)
    }
    #[doc = "Configure as the SPI channel 5 nCE signal from IOMSTR0"]
    #[inline(always)]
    pub fn m0n_ce5(self) -> &'a mut W {
        self.variant(PAD47FNCSEL_A::M0NCE5)
    }
    #[doc = "Configure as the input/output signal from CTIMER B2"]
    #[inline(always)]
    pub fn tctb2(self) -> &'a mut W {
        self.variant(PAD47FNCSEL_A::TCTB2)
    }
    #[doc = "Configure as GPIO47"]
    #[inline(always)]
    pub fn gpio47(self) -> &'a mut W {
        self.variant(PAD47FNCSEL_A::GPIO47)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 27)) | (((value as u32) & 0x03) << 27);
        self.w
    }
}
#[doc = "Pad 47 drive strentgh\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD47STRNG_A {
    #[doc = "0: Low drive strength"]
    LOW = 0,
    #[doc = "1: High drive strength"]
    HIGH = 1,
}
impl From<PAD47STRNG_A> for bool {
    #[inline(always)]
    fn from(variant: PAD47STRNG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD47STRNG`"]
pub type PAD47STRNG_R = crate::R<bool, PAD47STRNG_A>;
impl PAD47STRNG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD47STRNG_A {
        match self.bits {
            false => PAD47STRNG_A::LOW,
            true => PAD47STRNG_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == PAD47STRNG_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == PAD47STRNG_A::HIGH
    }
}
#[doc = "Write proxy for field `PAD47STRNG`"]
pub struct PAD47STRNG_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD47STRNG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD47STRNG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Low drive strength"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD47STRNG_A::LOW)
    }
    #[doc = "High drive strength"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD47STRNG_A::HIGH)
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Pad 47 input enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD47INPEN_A {
    #[doc = "0: Pad input disabled"]
    DIS = 0,
    #[doc = "1: Pad input enabled"]
    EN = 1,
}
impl From<PAD47INPEN_A> for bool {
    #[inline(always)]
    fn from(variant: PAD47INPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD47INPEN`"]
pub type PAD47INPEN_R = crate::R<bool, PAD47INPEN_A>;
impl PAD47INPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD47INPEN_A {
        match self.bits {
            false => PAD47INPEN_A::DIS,
            true => PAD47INPEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD47INPEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD47INPEN_A::EN
    }
}
#[doc = "Write proxy for field `PAD47INPEN`"]
pub struct PAD47INPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD47INPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD47INPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pad input disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD47INPEN_A::DIS)
    }
    #[doc = "Pad input enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD47INPEN_A::EN)
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Pad 47 pullup enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD47PULL_A {
    #[doc = "0: Pullup disabled"]
    DIS = 0,
    #[doc = "1: Pullup enabled"]
    EN = 1,
}
impl From<PAD47PULL_A> for bool {
    #[inline(always)]
    fn from(variant: PAD47PULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD47PULL`"]
pub type PAD47PULL_R = crate::R<bool, PAD47PULL_A>;
impl PAD47PULL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD47PULL_A {
        match self.bits {
            false => PAD47PULL_A::DIS,
            true => PAD47PULL_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD47PULL_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD47PULL_A::EN
    }
}
#[doc = "Write proxy for field `PAD47PULL`"]
pub struct PAD47PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD47PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD47PULL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD47PULL_A::DIS)
    }
    #[doc = "Pullup enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD47PULL_A::EN)
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
#[doc = "Pad 46 function select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD46FNCSEL_A {
    #[doc = "0: Pad disabled"]
    DIS = 0,
    #[doc = "1: Configure as the SPI channel 4 nCE signal from IOMSTR0"]
    M0NCE4 = 1,
    #[doc = "2: Configure as the input/output signal from CTIMER A2"]
    TCTA2 = 2,
    #[doc = "3: Configure as GPIO46"]
    GPIO46 = 3,
}
impl From<PAD46FNCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD46FNCSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PAD46FNCSEL`"]
pub type PAD46FNCSEL_R = crate::R<u8, PAD46FNCSEL_A>;
impl PAD46FNCSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD46FNCSEL_A {
        match self.bits {
            0 => PAD46FNCSEL_A::DIS,
            1 => PAD46FNCSEL_A::M0NCE4,
            2 => PAD46FNCSEL_A::TCTA2,
            3 => PAD46FNCSEL_A::GPIO46,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD46FNCSEL_A::DIS
    }
    #[doc = "Checks if the value of the field is `M0NCE4`"]
    #[inline(always)]
    pub fn is_m0n_ce4(&self) -> bool {
        *self == PAD46FNCSEL_A::M0NCE4
    }
    #[doc = "Checks if the value of the field is `TCTA2`"]
    #[inline(always)]
    pub fn is_tcta2(&self) -> bool {
        *self == PAD46FNCSEL_A::TCTA2
    }
    #[doc = "Checks if the value of the field is `GPIO46`"]
    #[inline(always)]
    pub fn is_gpio46(&self) -> bool {
        *self == PAD46FNCSEL_A::GPIO46
    }
}
#[doc = "Write proxy for field `PAD46FNCSEL`"]
pub struct PAD46FNCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD46FNCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD46FNCSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Pad disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD46FNCSEL_A::DIS)
    }
    #[doc = "Configure as the SPI channel 4 nCE signal from IOMSTR0"]
    #[inline(always)]
    pub fn m0n_ce4(self) -> &'a mut W {
        self.variant(PAD46FNCSEL_A::M0NCE4)
    }
    #[doc = "Configure as the input/output signal from CTIMER A2"]
    #[inline(always)]
    pub fn tcta2(self) -> &'a mut W {
        self.variant(PAD46FNCSEL_A::TCTA2)
    }
    #[doc = "Configure as GPIO46"]
    #[inline(always)]
    pub fn gpio46(self) -> &'a mut W {
        self.variant(PAD46FNCSEL_A::GPIO46)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 19)) | (((value as u32) & 0x03) << 19);
        self.w
    }
}
#[doc = "Pad 46 drive strength\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD46STRNG_A {
    #[doc = "0: Low drive strength"]
    LOW = 0,
    #[doc = "1: High drive strength"]
    HIGH = 1,
}
impl From<PAD46STRNG_A> for bool {
    #[inline(always)]
    fn from(variant: PAD46STRNG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD46STRNG`"]
pub type PAD46STRNG_R = crate::R<bool, PAD46STRNG_A>;
impl PAD46STRNG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD46STRNG_A {
        match self.bits {
            false => PAD46STRNG_A::LOW,
            true => PAD46STRNG_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == PAD46STRNG_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == PAD46STRNG_A::HIGH
    }
}
#[doc = "Write proxy for field `PAD46STRNG`"]
pub struct PAD46STRNG_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD46STRNG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD46STRNG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Low drive strength"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD46STRNG_A::LOW)
    }
    #[doc = "High drive strength"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD46STRNG_A::HIGH)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Pad 46 input enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD46INPEN_A {
    #[doc = "0: Pad input disabled"]
    DIS = 0,
    #[doc = "1: Pad input enabled"]
    EN = 1,
}
impl From<PAD46INPEN_A> for bool {
    #[inline(always)]
    fn from(variant: PAD46INPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD46INPEN`"]
pub type PAD46INPEN_R = crate::R<bool, PAD46INPEN_A>;
impl PAD46INPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD46INPEN_A {
        match self.bits {
            false => PAD46INPEN_A::DIS,
            true => PAD46INPEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD46INPEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD46INPEN_A::EN
    }
}
#[doc = "Write proxy for field `PAD46INPEN`"]
pub struct PAD46INPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD46INPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD46INPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pad input disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD46INPEN_A::DIS)
    }
    #[doc = "Pad input enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD46INPEN_A::EN)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Pad 46 pullup enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD46PULL_A {
    #[doc = "0: Pullup disabled"]
    DIS = 0,
    #[doc = "1: Pullup enabled"]
    EN = 1,
}
impl From<PAD46PULL_A> for bool {
    #[inline(always)]
    fn from(variant: PAD46PULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD46PULL`"]
pub type PAD46PULL_R = crate::R<bool, PAD46PULL_A>;
impl PAD46PULL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD46PULL_A {
        match self.bits {
            false => PAD46PULL_A::DIS,
            true => PAD46PULL_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD46PULL_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD46PULL_A::EN
    }
}
#[doc = "Write proxy for field `PAD46PULL`"]
pub struct PAD46PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD46PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD46PULL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD46PULL_A::DIS)
    }
    #[doc = "Pullup enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD46PULL_A::EN)
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
#[doc = "Pad 45 function select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD45FNCSEL_A {
    #[doc = "0: Pad disabled"]
    DIS = 0,
    #[doc = "1: Configure as the SPI channel 3 nCE signal from IOMSTR0"]
    M0NCE3 = 1,
    #[doc = "2: Configure as the input/output signal from CTIMER B1"]
    TCTB1 = 2,
    #[doc = "3: Configure as GPIO45"]
    GPIO45 = 3,
}
impl From<PAD45FNCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD45FNCSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PAD45FNCSEL`"]
pub type PAD45FNCSEL_R = crate::R<u8, PAD45FNCSEL_A>;
impl PAD45FNCSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD45FNCSEL_A {
        match self.bits {
            0 => PAD45FNCSEL_A::DIS,
            1 => PAD45FNCSEL_A::M0NCE3,
            2 => PAD45FNCSEL_A::TCTB1,
            3 => PAD45FNCSEL_A::GPIO45,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD45FNCSEL_A::DIS
    }
    #[doc = "Checks if the value of the field is `M0NCE3`"]
    #[inline(always)]
    pub fn is_m0n_ce3(&self) -> bool {
        *self == PAD45FNCSEL_A::M0NCE3
    }
    #[doc = "Checks if the value of the field is `TCTB1`"]
    #[inline(always)]
    pub fn is_tctb1(&self) -> bool {
        *self == PAD45FNCSEL_A::TCTB1
    }
    #[doc = "Checks if the value of the field is `GPIO45`"]
    #[inline(always)]
    pub fn is_gpio45(&self) -> bool {
        *self == PAD45FNCSEL_A::GPIO45
    }
}
#[doc = "Write proxy for field `PAD45FNCSEL`"]
pub struct PAD45FNCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD45FNCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD45FNCSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Pad disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD45FNCSEL_A::DIS)
    }
    #[doc = "Configure as the SPI channel 3 nCE signal from IOMSTR0"]
    #[inline(always)]
    pub fn m0n_ce3(self) -> &'a mut W {
        self.variant(PAD45FNCSEL_A::M0NCE3)
    }
    #[doc = "Configure as the input/output signal from CTIMER B1"]
    #[inline(always)]
    pub fn tctb1(self) -> &'a mut W {
        self.variant(PAD45FNCSEL_A::TCTB1)
    }
    #[doc = "Configure as GPIO45"]
    #[inline(always)]
    pub fn gpio45(self) -> &'a mut W {
        self.variant(PAD45FNCSEL_A::GPIO45)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 11)) | (((value as u32) & 0x03) << 11);
        self.w
    }
}
#[doc = "Pad 45 drive strength\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD45STRNG_A {
    #[doc = "0: Low drive strength"]
    LOW = 0,
    #[doc = "1: High drive strength"]
    HIGH = 1,
}
impl From<PAD45STRNG_A> for bool {
    #[inline(always)]
    fn from(variant: PAD45STRNG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD45STRNG`"]
pub type PAD45STRNG_R = crate::R<bool, PAD45STRNG_A>;
impl PAD45STRNG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD45STRNG_A {
        match self.bits {
            false => PAD45STRNG_A::LOW,
            true => PAD45STRNG_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == PAD45STRNG_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == PAD45STRNG_A::HIGH
    }
}
#[doc = "Write proxy for field `PAD45STRNG`"]
pub struct PAD45STRNG_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD45STRNG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD45STRNG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Low drive strength"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD45STRNG_A::LOW)
    }
    #[doc = "High drive strength"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD45STRNG_A::HIGH)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Pad 45 input enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD45INPEN_A {
    #[doc = "0: Pad input disabled"]
    DIS = 0,
    #[doc = "1: Pad input enabled"]
    EN = 1,
}
impl From<PAD45INPEN_A> for bool {
    #[inline(always)]
    fn from(variant: PAD45INPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD45INPEN`"]
pub type PAD45INPEN_R = crate::R<bool, PAD45INPEN_A>;
impl PAD45INPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD45INPEN_A {
        match self.bits {
            false => PAD45INPEN_A::DIS,
            true => PAD45INPEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD45INPEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD45INPEN_A::EN
    }
}
#[doc = "Write proxy for field `PAD45INPEN`"]
pub struct PAD45INPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD45INPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD45INPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pad input disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD45INPEN_A::DIS)
    }
    #[doc = "Pad input enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD45INPEN_A::EN)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Pad 45 pullup enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD45PULL_A {
    #[doc = "0: Pullup disabled"]
    DIS = 0,
    #[doc = "1: Pullup enabled"]
    EN = 1,
}
impl From<PAD45PULL_A> for bool {
    #[inline(always)]
    fn from(variant: PAD45PULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD45PULL`"]
pub type PAD45PULL_R = crate::R<bool, PAD45PULL_A>;
impl PAD45PULL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD45PULL_A {
        match self.bits {
            false => PAD45PULL_A::DIS,
            true => PAD45PULL_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD45PULL_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD45PULL_A::EN
    }
}
#[doc = "Write proxy for field `PAD45PULL`"]
pub struct PAD45PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD45PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD45PULL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD45PULL_A::DIS)
    }
    #[doc = "Pullup enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD45PULL_A::EN)
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
#[doc = "Pad 44 function select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD44FNCSEL_A {
    #[doc = "0: Configure as the ADC Trigger 7 signal"]
    TRIG7 = 0,
    #[doc = "1: Configure as the SPI channel 2 nCE signal from IOMSTR0"]
    M0NCE2 = 1,
    #[doc = "2: Configure as the input/output signal from CTIMER A1"]
    TCTA1 = 2,
    #[doc = "3: Configure as GPIO44"]
    GPIO44 = 3,
}
impl From<PAD44FNCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD44FNCSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PAD44FNCSEL`"]
pub type PAD44FNCSEL_R = crate::R<u8, PAD44FNCSEL_A>;
impl PAD44FNCSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD44FNCSEL_A {
        match self.bits {
            0 => PAD44FNCSEL_A::TRIG7,
            1 => PAD44FNCSEL_A::M0NCE2,
            2 => PAD44FNCSEL_A::TCTA1,
            3 => PAD44FNCSEL_A::GPIO44,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TRIG7`"]
    #[inline(always)]
    pub fn is_trig7(&self) -> bool {
        *self == PAD44FNCSEL_A::TRIG7
    }
    #[doc = "Checks if the value of the field is `M0NCE2`"]
    #[inline(always)]
    pub fn is_m0n_ce2(&self) -> bool {
        *self == PAD44FNCSEL_A::M0NCE2
    }
    #[doc = "Checks if the value of the field is `TCTA1`"]
    #[inline(always)]
    pub fn is_tcta1(&self) -> bool {
        *self == PAD44FNCSEL_A::TCTA1
    }
    #[doc = "Checks if the value of the field is `GPIO44`"]
    #[inline(always)]
    pub fn is_gpio44(&self) -> bool {
        *self == PAD44FNCSEL_A::GPIO44
    }
}
#[doc = "Write proxy for field `PAD44FNCSEL`"]
pub struct PAD44FNCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD44FNCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD44FNCSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Configure as the ADC Trigger 7 signal"]
    #[inline(always)]
    pub fn trig7(self) -> &'a mut W {
        self.variant(PAD44FNCSEL_A::TRIG7)
    }
    #[doc = "Configure as the SPI channel 2 nCE signal from IOMSTR0"]
    #[inline(always)]
    pub fn m0n_ce2(self) -> &'a mut W {
        self.variant(PAD44FNCSEL_A::M0NCE2)
    }
    #[doc = "Configure as the input/output signal from CTIMER A1"]
    #[inline(always)]
    pub fn tcta1(self) -> &'a mut W {
        self.variant(PAD44FNCSEL_A::TCTA1)
    }
    #[doc = "Configure as GPIO44"]
    #[inline(always)]
    pub fn gpio44(self) -> &'a mut W {
        self.variant(PAD44FNCSEL_A::GPIO44)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | (((value as u32) & 0x03) << 3);
        self.w
    }
}
#[doc = "Pad 44 drive strength\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD44STRNG_A {
    #[doc = "0: Low drive strength"]
    LOW = 0,
    #[doc = "1: High drive strength"]
    HIGH = 1,
}
impl From<PAD44STRNG_A> for bool {
    #[inline(always)]
    fn from(variant: PAD44STRNG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD44STRNG`"]
pub type PAD44STRNG_R = crate::R<bool, PAD44STRNG_A>;
impl PAD44STRNG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD44STRNG_A {
        match self.bits {
            false => PAD44STRNG_A::LOW,
            true => PAD44STRNG_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == PAD44STRNG_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == PAD44STRNG_A::HIGH
    }
}
#[doc = "Write proxy for field `PAD44STRNG`"]
pub struct PAD44STRNG_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD44STRNG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD44STRNG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Low drive strength"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD44STRNG_A::LOW)
    }
    #[doc = "High drive strength"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD44STRNG_A::HIGH)
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
#[doc = "Pad 44 input enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD44INPEN_A {
    #[doc = "0: Pad input disabled"]
    DIS = 0,
    #[doc = "1: Pad input enabled"]
    EN = 1,
}
impl From<PAD44INPEN_A> for bool {
    #[inline(always)]
    fn from(variant: PAD44INPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD44INPEN`"]
pub type PAD44INPEN_R = crate::R<bool, PAD44INPEN_A>;
impl PAD44INPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD44INPEN_A {
        match self.bits {
            false => PAD44INPEN_A::DIS,
            true => PAD44INPEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD44INPEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD44INPEN_A::EN
    }
}
#[doc = "Write proxy for field `PAD44INPEN`"]
pub struct PAD44INPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD44INPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD44INPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pad input disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD44INPEN_A::DIS)
    }
    #[doc = "Pad input enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD44INPEN_A::EN)
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
#[doc = "Pad 44 pullup enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD44PULL_A {
    #[doc = "0: Pullup disabled"]
    DIS = 0,
    #[doc = "1: Pullup enabled"]
    EN = 1,
}
impl From<PAD44PULL_A> for bool {
    #[inline(always)]
    fn from(variant: PAD44PULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD44PULL`"]
pub type PAD44PULL_R = crate::R<bool, PAD44PULL_A>;
impl PAD44PULL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD44PULL_A {
        match self.bits {
            false => PAD44PULL_A::DIS,
            true => PAD44PULL_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD44PULL_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD44PULL_A::EN
    }
}
#[doc = "Write proxy for field `PAD44PULL`"]
pub struct PAD44PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD44PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD44PULL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD44PULL_A::DIS)
    }
    #[doc = "Pullup enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD44PULL_A::EN)
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
    #[doc = "Bits 27:28 - Pad 47 function select"]
    #[inline(always)]
    pub fn pad47fncsel(&self) -> PAD47FNCSEL_R {
        PAD47FNCSEL_R::new(((self.bits >> 27) & 0x03) as u8)
    }
    #[doc = "Bit 26 - Pad 47 drive strentgh"]
    #[inline(always)]
    pub fn pad47strng(&self) -> PAD47STRNG_R {
        PAD47STRNG_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Pad 47 input enable"]
    #[inline(always)]
    pub fn pad47inpen(&self) -> PAD47INPEN_R {
        PAD47INPEN_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Pad 47 pullup enable"]
    #[inline(always)]
    pub fn pad47pull(&self) -> PAD47PULL_R {
        PAD47PULL_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 19:20 - Pad 46 function select"]
    #[inline(always)]
    pub fn pad46fncsel(&self) -> PAD46FNCSEL_R {
        PAD46FNCSEL_R::new(((self.bits >> 19) & 0x03) as u8)
    }
    #[doc = "Bit 18 - Pad 46 drive strength"]
    #[inline(always)]
    pub fn pad46strng(&self) -> PAD46STRNG_R {
        PAD46STRNG_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Pad 46 input enable"]
    #[inline(always)]
    pub fn pad46inpen(&self) -> PAD46INPEN_R {
        PAD46INPEN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Pad 46 pullup enable"]
    #[inline(always)]
    pub fn pad46pull(&self) -> PAD46PULL_R {
        PAD46PULL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 11:12 - Pad 45 function select"]
    #[inline(always)]
    pub fn pad45fncsel(&self) -> PAD45FNCSEL_R {
        PAD45FNCSEL_R::new(((self.bits >> 11) & 0x03) as u8)
    }
    #[doc = "Bit 10 - Pad 45 drive strength"]
    #[inline(always)]
    pub fn pad45strng(&self) -> PAD45STRNG_R {
        PAD45STRNG_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Pad 45 input enable"]
    #[inline(always)]
    pub fn pad45inpen(&self) -> PAD45INPEN_R {
        PAD45INPEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Pad 45 pullup enable"]
    #[inline(always)]
    pub fn pad45pull(&self) -> PAD45PULL_R {
        PAD45PULL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 3:4 - Pad 44 function select"]
    #[inline(always)]
    pub fn pad44fncsel(&self) -> PAD44FNCSEL_R {
        PAD44FNCSEL_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bit 2 - Pad 44 drive strength"]
    #[inline(always)]
    pub fn pad44strng(&self) -> PAD44STRNG_R {
        PAD44STRNG_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Pad 44 input enable"]
    #[inline(always)]
    pub fn pad44inpen(&self) -> PAD44INPEN_R {
        PAD44INPEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Pad 44 pullup enable"]
    #[inline(always)]
    pub fn pad44pull(&self) -> PAD44PULL_R {
        PAD44PULL_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 27:28 - Pad 47 function select"]
    #[inline(always)]
    pub fn pad47fncsel(&mut self) -> PAD47FNCSEL_W {
        PAD47FNCSEL_W { w: self }
    }
    #[doc = "Bit 26 - Pad 47 drive strentgh"]
    #[inline(always)]
    pub fn pad47strng(&mut self) -> PAD47STRNG_W {
        PAD47STRNG_W { w: self }
    }
    #[doc = "Bit 25 - Pad 47 input enable"]
    #[inline(always)]
    pub fn pad47inpen(&mut self) -> PAD47INPEN_W {
        PAD47INPEN_W { w: self }
    }
    #[doc = "Bit 24 - Pad 47 pullup enable"]
    #[inline(always)]
    pub fn pad47pull(&mut self) -> PAD47PULL_W {
        PAD47PULL_W { w: self }
    }
    #[doc = "Bits 19:20 - Pad 46 function select"]
    #[inline(always)]
    pub fn pad46fncsel(&mut self) -> PAD46FNCSEL_W {
        PAD46FNCSEL_W { w: self }
    }
    #[doc = "Bit 18 - Pad 46 drive strength"]
    #[inline(always)]
    pub fn pad46strng(&mut self) -> PAD46STRNG_W {
        PAD46STRNG_W { w: self }
    }
    #[doc = "Bit 17 - Pad 46 input enable"]
    #[inline(always)]
    pub fn pad46inpen(&mut self) -> PAD46INPEN_W {
        PAD46INPEN_W { w: self }
    }
    #[doc = "Bit 16 - Pad 46 pullup enable"]
    #[inline(always)]
    pub fn pad46pull(&mut self) -> PAD46PULL_W {
        PAD46PULL_W { w: self }
    }
    #[doc = "Bits 11:12 - Pad 45 function select"]
    #[inline(always)]
    pub fn pad45fncsel(&mut self) -> PAD45FNCSEL_W {
        PAD45FNCSEL_W { w: self }
    }
    #[doc = "Bit 10 - Pad 45 drive strength"]
    #[inline(always)]
    pub fn pad45strng(&mut self) -> PAD45STRNG_W {
        PAD45STRNG_W { w: self }
    }
    #[doc = "Bit 9 - Pad 45 input enable"]
    #[inline(always)]
    pub fn pad45inpen(&mut self) -> PAD45INPEN_W {
        PAD45INPEN_W { w: self }
    }
    #[doc = "Bit 8 - Pad 45 pullup enable"]
    #[inline(always)]
    pub fn pad45pull(&mut self) -> PAD45PULL_W {
        PAD45PULL_W { w: self }
    }
    #[doc = "Bits 3:4 - Pad 44 function select"]
    #[inline(always)]
    pub fn pad44fncsel(&mut self) -> PAD44FNCSEL_W {
        PAD44FNCSEL_W { w: self }
    }
    #[doc = "Bit 2 - Pad 44 drive strength"]
    #[inline(always)]
    pub fn pad44strng(&mut self) -> PAD44STRNG_W {
        PAD44STRNG_W { w: self }
    }
    #[doc = "Bit 1 - Pad 44 input enable"]
    #[inline(always)]
    pub fn pad44inpen(&mut self) -> PAD44INPEN_W {
        PAD44INPEN_W { w: self }
    }
    #[doc = "Bit 0 - Pad 44 pullup enable"]
    #[inline(always)]
    pub fn pad44pull(&mut self) -> PAD44PULL_W {
        PAD44PULL_W { w: self }
    }
}
