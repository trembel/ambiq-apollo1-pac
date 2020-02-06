#[doc = "Reader of register PADREGM"]
pub type R = crate::R<u32, super::PADREGM>;
#[doc = "Writer for register PADREGM"]
pub type W = crate::W<u32, super::PADREGM>;
#[doc = "Register PADREGM `reset()`'s with value 0x1818"]
impl crate::ResetValue for super::PADREGM {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x1818
    }
}
#[doc = "Pad 49 function select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD49FNCSEL_A {
    #[doc = "0: Pad disabled"]
    DIS = 0,
    #[doc = "1: Configure as the SPI channel 7 nCE signal from IOMSTR0"]
    M0NCE7 = 1,
    #[doc = "2: Configure as the input/output signal from CTIMER B3"]
    TCTB3 = 2,
    #[doc = "3: Configure as GPIO49"]
    GPIO49 = 3,
}
impl From<PAD49FNCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD49FNCSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PAD49FNCSEL`"]
pub type PAD49FNCSEL_R = crate::R<u8, PAD49FNCSEL_A>;
impl PAD49FNCSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD49FNCSEL_A {
        match self.bits {
            0 => PAD49FNCSEL_A::DIS,
            1 => PAD49FNCSEL_A::M0NCE7,
            2 => PAD49FNCSEL_A::TCTB3,
            3 => PAD49FNCSEL_A::GPIO49,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD49FNCSEL_A::DIS
    }
    #[doc = "Checks if the value of the field is `M0NCE7`"]
    #[inline(always)]
    pub fn is_m0n_ce7(&self) -> bool {
        *self == PAD49FNCSEL_A::M0NCE7
    }
    #[doc = "Checks if the value of the field is `TCTB3`"]
    #[inline(always)]
    pub fn is_tctb3(&self) -> bool {
        *self == PAD49FNCSEL_A::TCTB3
    }
    #[doc = "Checks if the value of the field is `GPIO49`"]
    #[inline(always)]
    pub fn is_gpio49(&self) -> bool {
        *self == PAD49FNCSEL_A::GPIO49
    }
}
#[doc = "Write proxy for field `PAD49FNCSEL`"]
pub struct PAD49FNCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD49FNCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD49FNCSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Pad disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD49FNCSEL_A::DIS)
    }
    #[doc = "Configure as the SPI channel 7 nCE signal from IOMSTR0"]
    #[inline(always)]
    pub fn m0n_ce7(self) -> &'a mut W {
        self.variant(PAD49FNCSEL_A::M0NCE7)
    }
    #[doc = "Configure as the input/output signal from CTIMER B3"]
    #[inline(always)]
    pub fn tctb3(self) -> &'a mut W {
        self.variant(PAD49FNCSEL_A::TCTB3)
    }
    #[doc = "Configure as GPIO49"]
    #[inline(always)]
    pub fn gpio49(self) -> &'a mut W {
        self.variant(PAD49FNCSEL_A::GPIO49)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 11)) | (((value as u32) & 0x03) << 11);
        self.w
    }
}
#[doc = "Pad 49 drive strength\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD49STRNG_A {
    #[doc = "0: Low drive strength"]
    LOW = 0,
    #[doc = "1: High drive strength"]
    HIGH = 1,
}
impl From<PAD49STRNG_A> for bool {
    #[inline(always)]
    fn from(variant: PAD49STRNG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD49STRNG`"]
pub type PAD49STRNG_R = crate::R<bool, PAD49STRNG_A>;
impl PAD49STRNG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD49STRNG_A {
        match self.bits {
            false => PAD49STRNG_A::LOW,
            true => PAD49STRNG_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == PAD49STRNG_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == PAD49STRNG_A::HIGH
    }
}
#[doc = "Write proxy for field `PAD49STRNG`"]
pub struct PAD49STRNG_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD49STRNG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD49STRNG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Low drive strength"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD49STRNG_A::LOW)
    }
    #[doc = "High drive strength"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD49STRNG_A::HIGH)
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
#[doc = "Pad 49 input enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD49INPEN_A {
    #[doc = "0: Pad input disabled"]
    DIS = 0,
    #[doc = "1: Pad input enabled"]
    EN = 1,
}
impl From<PAD49INPEN_A> for bool {
    #[inline(always)]
    fn from(variant: PAD49INPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD49INPEN`"]
pub type PAD49INPEN_R = crate::R<bool, PAD49INPEN_A>;
impl PAD49INPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD49INPEN_A {
        match self.bits {
            false => PAD49INPEN_A::DIS,
            true => PAD49INPEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD49INPEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD49INPEN_A::EN
    }
}
#[doc = "Write proxy for field `PAD49INPEN`"]
pub struct PAD49INPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD49INPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD49INPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pad input disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD49INPEN_A::DIS)
    }
    #[doc = "Pad input enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD49INPEN_A::EN)
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
#[doc = "Pad 49 pullup enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD49PULL_A {
    #[doc = "0: Pullup disabled"]
    DIS = 0,
    #[doc = "1: Pullup enabled"]
    EN = 1,
}
impl From<PAD49PULL_A> for bool {
    #[inline(always)]
    fn from(variant: PAD49PULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD49PULL`"]
pub type PAD49PULL_R = crate::R<bool, PAD49PULL_A>;
impl PAD49PULL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD49PULL_A {
        match self.bits {
            false => PAD49PULL_A::DIS,
            true => PAD49PULL_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD49PULL_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD49PULL_A::EN
    }
}
#[doc = "Write proxy for field `PAD49PULL`"]
pub struct PAD49PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD49PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD49PULL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD49PULL_A::DIS)
    }
    #[doc = "Pullup enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD49PULL_A::EN)
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
#[doc = "Pad 48 function select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD48FNCSEL_A {
    #[doc = "0: Pad disabled"]
    DIS = 0,
    #[doc = "1: Configure as the SPI channel 6 nCE signal from IOMSTR0"]
    M0NCE6 = 1,
    #[doc = "2: Configure as the input/output signal from CTIMER A3"]
    TCTA3 = 2,
    #[doc = "3: Configure as GPIO48"]
    GPIO48 = 3,
}
impl From<PAD48FNCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD48FNCSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PAD48FNCSEL`"]
pub type PAD48FNCSEL_R = crate::R<u8, PAD48FNCSEL_A>;
impl PAD48FNCSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD48FNCSEL_A {
        match self.bits {
            0 => PAD48FNCSEL_A::DIS,
            1 => PAD48FNCSEL_A::M0NCE6,
            2 => PAD48FNCSEL_A::TCTA3,
            3 => PAD48FNCSEL_A::GPIO48,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD48FNCSEL_A::DIS
    }
    #[doc = "Checks if the value of the field is `M0NCE6`"]
    #[inline(always)]
    pub fn is_m0n_ce6(&self) -> bool {
        *self == PAD48FNCSEL_A::M0NCE6
    }
    #[doc = "Checks if the value of the field is `TCTA3`"]
    #[inline(always)]
    pub fn is_tcta3(&self) -> bool {
        *self == PAD48FNCSEL_A::TCTA3
    }
    #[doc = "Checks if the value of the field is `GPIO48`"]
    #[inline(always)]
    pub fn is_gpio48(&self) -> bool {
        *self == PAD48FNCSEL_A::GPIO48
    }
}
#[doc = "Write proxy for field `PAD48FNCSEL`"]
pub struct PAD48FNCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD48FNCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD48FNCSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Pad disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD48FNCSEL_A::DIS)
    }
    #[doc = "Configure as the SPI channel 6 nCE signal from IOMSTR0"]
    #[inline(always)]
    pub fn m0n_ce6(self) -> &'a mut W {
        self.variant(PAD48FNCSEL_A::M0NCE6)
    }
    #[doc = "Configure as the input/output signal from CTIMER A3"]
    #[inline(always)]
    pub fn tcta3(self) -> &'a mut W {
        self.variant(PAD48FNCSEL_A::TCTA3)
    }
    #[doc = "Configure as GPIO48"]
    #[inline(always)]
    pub fn gpio48(self) -> &'a mut W {
        self.variant(PAD48FNCSEL_A::GPIO48)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | (((value as u32) & 0x03) << 3);
        self.w
    }
}
#[doc = "Pad 48 drive strength\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD48STRNG_A {
    #[doc = "0: Low drive strength"]
    LOW = 0,
    #[doc = "1: High drive strength"]
    HIGH = 1,
}
impl From<PAD48STRNG_A> for bool {
    #[inline(always)]
    fn from(variant: PAD48STRNG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD48STRNG`"]
pub type PAD48STRNG_R = crate::R<bool, PAD48STRNG_A>;
impl PAD48STRNG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD48STRNG_A {
        match self.bits {
            false => PAD48STRNG_A::LOW,
            true => PAD48STRNG_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == PAD48STRNG_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == PAD48STRNG_A::HIGH
    }
}
#[doc = "Write proxy for field `PAD48STRNG`"]
pub struct PAD48STRNG_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD48STRNG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD48STRNG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Low drive strength"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD48STRNG_A::LOW)
    }
    #[doc = "High drive strength"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD48STRNG_A::HIGH)
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
#[doc = "Pad 48 input enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD48INPEN_A {
    #[doc = "0: Pad input disabled"]
    DIS = 0,
    #[doc = "1: Pad input enabled"]
    EN = 1,
}
impl From<PAD48INPEN_A> for bool {
    #[inline(always)]
    fn from(variant: PAD48INPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD48INPEN`"]
pub type PAD48INPEN_R = crate::R<bool, PAD48INPEN_A>;
impl PAD48INPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD48INPEN_A {
        match self.bits {
            false => PAD48INPEN_A::DIS,
            true => PAD48INPEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD48INPEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD48INPEN_A::EN
    }
}
#[doc = "Write proxy for field `PAD48INPEN`"]
pub struct PAD48INPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD48INPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD48INPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pad input disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD48INPEN_A::DIS)
    }
    #[doc = "Pad input enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD48INPEN_A::EN)
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
#[doc = "Pad 48 pullup enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD48PULL_A {
    #[doc = "0: Pullup disabled"]
    DIS = 0,
    #[doc = "1: Pullup enabled"]
    EN = 1,
}
impl From<PAD48PULL_A> for bool {
    #[inline(always)]
    fn from(variant: PAD48PULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD48PULL`"]
pub type PAD48PULL_R = crate::R<bool, PAD48PULL_A>;
impl PAD48PULL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD48PULL_A {
        match self.bits {
            false => PAD48PULL_A::DIS,
            true => PAD48PULL_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD48PULL_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD48PULL_A::EN
    }
}
#[doc = "Write proxy for field `PAD48PULL`"]
pub struct PAD48PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD48PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD48PULL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD48PULL_A::DIS)
    }
    #[doc = "Pullup enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD48PULL_A::EN)
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
    #[doc = "Bits 11:12 - Pad 49 function select"]
    #[inline(always)]
    pub fn pad49fncsel(&self) -> PAD49FNCSEL_R {
        PAD49FNCSEL_R::new(((self.bits >> 11) & 0x03) as u8)
    }
    #[doc = "Bit 10 - Pad 49 drive strength"]
    #[inline(always)]
    pub fn pad49strng(&self) -> PAD49STRNG_R {
        PAD49STRNG_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Pad 49 input enable"]
    #[inline(always)]
    pub fn pad49inpen(&self) -> PAD49INPEN_R {
        PAD49INPEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Pad 49 pullup enable"]
    #[inline(always)]
    pub fn pad49pull(&self) -> PAD49PULL_R {
        PAD49PULL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 3:4 - Pad 48 function select"]
    #[inline(always)]
    pub fn pad48fncsel(&self) -> PAD48FNCSEL_R {
        PAD48FNCSEL_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bit 2 - Pad 48 drive strength"]
    #[inline(always)]
    pub fn pad48strng(&self) -> PAD48STRNG_R {
        PAD48STRNG_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Pad 48 input enable"]
    #[inline(always)]
    pub fn pad48inpen(&self) -> PAD48INPEN_R {
        PAD48INPEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Pad 48 pullup enable"]
    #[inline(always)]
    pub fn pad48pull(&self) -> PAD48PULL_R {
        PAD48PULL_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 11:12 - Pad 49 function select"]
    #[inline(always)]
    pub fn pad49fncsel(&mut self) -> PAD49FNCSEL_W {
        PAD49FNCSEL_W { w: self }
    }
    #[doc = "Bit 10 - Pad 49 drive strength"]
    #[inline(always)]
    pub fn pad49strng(&mut self) -> PAD49STRNG_W {
        PAD49STRNG_W { w: self }
    }
    #[doc = "Bit 9 - Pad 49 input enable"]
    #[inline(always)]
    pub fn pad49inpen(&mut self) -> PAD49INPEN_W {
        PAD49INPEN_W { w: self }
    }
    #[doc = "Bit 8 - Pad 49 pullup enable"]
    #[inline(always)]
    pub fn pad49pull(&mut self) -> PAD49PULL_W {
        PAD49PULL_W { w: self }
    }
    #[doc = "Bits 3:4 - Pad 48 function select"]
    #[inline(always)]
    pub fn pad48fncsel(&mut self) -> PAD48FNCSEL_W {
        PAD48FNCSEL_W { w: self }
    }
    #[doc = "Bit 2 - Pad 48 drive strength"]
    #[inline(always)]
    pub fn pad48strng(&mut self) -> PAD48STRNG_W {
        PAD48STRNG_W { w: self }
    }
    #[doc = "Bit 1 - Pad 48 input enable"]
    #[inline(always)]
    pub fn pad48inpen(&mut self) -> PAD48INPEN_W {
        PAD48INPEN_W { w: self }
    }
    #[doc = "Bit 0 - Pad 48 pullup enable"]
    #[inline(always)]
    pub fn pad48pull(&mut self) -> PAD48PULL_W {
        PAD48PULL_W { w: self }
    }
}
