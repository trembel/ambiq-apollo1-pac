#[doc = "Reader of register CMD"]
pub type R = crate::R<u32, super::CMD>;
#[doc = "Writer for register CMD"]
pub type W = crate::W<u32, super::CMD>;
#[doc = "Register CMD `reset()`'s with value 0"]
impl crate::ResetValue for super::CMD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "This register is the I/O Command.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum CMD_A {
    #[doc = "0: LSB bit position of the CMD LENGTH field."]
    POS_LENGTH = 0,
    #[doc = "8: LSB bit position of the CMD OFFSET field."]
    POS_OFFSET = 8,
    #[doc = "16: LSB bit position of the I2C CMD ADDRESS field."]
    POS_ADDRESS = 16,
    #[doc = "23: LSB bit position of the SPI CMD UPLNGTH field."]
    POS_UPLNGTH = 23,
    #[doc = "26: LSB bit position of the I2C CMD 10-bit field."]
    POS_10BIT = 26,
    #[doc = "27: LSB bit position of the CMD LSB-first field."]
    POS_LSB = 27,
    #[doc = "28: LSB bit position of the CMD CONTinue field."]
    POS_CONT = 28,
    #[doc = "29: LSB bit position of the CMD OPERation field."]
    POS_OPER = 29,
    #[doc = "255: LSB bit mask of the CMD LENGTH field."]
    MSK_LENGTH = 255,
    #[doc = "65280: LSB bit mask of the CMD OFFSET field."]
    MSK_OFFSET = 65280,
    #[doc = "16711680: LSB bit mask of the I2C CMD ADDRESS field."]
    MSK_ADDRESS = 16711680,
    #[doc = "458752: LSB bit mask of the SPI CMD CHANNEL field."]
    MSK_CHNL = 458752,
    #[doc = "125829120: LSB bit mask of the SPI CMD UPLNGTH field."]
    MSK_UPLNGTH = 125829120,
    #[doc = "67108864: LSB bit mask of the I2C CMD 10-bit field."]
    MSK_10BIT = 67108864,
    #[doc = "134217728: LSB bit mask of the CMD LSB-first field."]
    MSK_LSB = 134217728,
    #[doc = "268435456: LSB bit mask of the CMD CONTinue field."]
    MSK_CONT = 268435456,
    #[doc = "3758096384: LSB bit mask of the CMD OPERation field."]
    MSK_OPER = 3758096384,
}
impl From<CMD_A> for u32 {
    #[inline(always)]
    fn from(variant: CMD_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CMD`"]
pub type CMD_R = crate::R<u32, CMD_A>;
impl CMD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, CMD_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CMD_A::POS_LENGTH),
            8 => Val(CMD_A::POS_OFFSET),
            16 => Val(CMD_A::POS_ADDRESS),
            23 => Val(CMD_A::POS_UPLNGTH),
            26 => Val(CMD_A::POS_10BIT),
            27 => Val(CMD_A::POS_LSB),
            28 => Val(CMD_A::POS_CONT),
            29 => Val(CMD_A::POS_OPER),
            255 => Val(CMD_A::MSK_LENGTH),
            65280 => Val(CMD_A::MSK_OFFSET),
            16711680 => Val(CMD_A::MSK_ADDRESS),
            458752 => Val(CMD_A::MSK_CHNL),
            125829120 => Val(CMD_A::MSK_UPLNGTH),
            67108864 => Val(CMD_A::MSK_10BIT),
            134217728 => Val(CMD_A::MSK_LSB),
            268435456 => Val(CMD_A::MSK_CONT),
            3758096384 => Val(CMD_A::MSK_OPER),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `POS_LENGTH`"]
    #[inline(always)]
    pub fn is_pos_length(&self) -> bool {
        *self == CMD_A::POS_LENGTH
    }
    #[doc = "Checks if the value of the field is `POS_OFFSET`"]
    #[inline(always)]
    pub fn is_pos_offset(&self) -> bool {
        *self == CMD_A::POS_OFFSET
    }
    #[doc = "Checks if the value of the field is `POS_ADDRESS`"]
    #[inline(always)]
    pub fn is_pos_address(&self) -> bool {
        *self == CMD_A::POS_ADDRESS
    }
    #[doc = "Checks if the value of the field is `POS_UPLNGTH`"]
    #[inline(always)]
    pub fn is_pos_uplngth(&self) -> bool {
        *self == CMD_A::POS_UPLNGTH
    }
    #[doc = "Checks if the value of the field is `POS_10BIT`"]
    #[inline(always)]
    pub fn is_pos_10bit(&self) -> bool {
        *self == CMD_A::POS_10BIT
    }
    #[doc = "Checks if the value of the field is `POS_LSB`"]
    #[inline(always)]
    pub fn is_pos_lsb(&self) -> bool {
        *self == CMD_A::POS_LSB
    }
    #[doc = "Checks if the value of the field is `POS_CONT`"]
    #[inline(always)]
    pub fn is_pos_cont(&self) -> bool {
        *self == CMD_A::POS_CONT
    }
    #[doc = "Checks if the value of the field is `POS_OPER`"]
    #[inline(always)]
    pub fn is_pos_oper(&self) -> bool {
        *self == CMD_A::POS_OPER
    }
    #[doc = "Checks if the value of the field is `MSK_LENGTH`"]
    #[inline(always)]
    pub fn is_msk_length(&self) -> bool {
        *self == CMD_A::MSK_LENGTH
    }
    #[doc = "Checks if the value of the field is `MSK_OFFSET`"]
    #[inline(always)]
    pub fn is_msk_offset(&self) -> bool {
        *self == CMD_A::MSK_OFFSET
    }
    #[doc = "Checks if the value of the field is `MSK_ADDRESS`"]
    #[inline(always)]
    pub fn is_msk_address(&self) -> bool {
        *self == CMD_A::MSK_ADDRESS
    }
    #[doc = "Checks if the value of the field is `MSK_CHNL`"]
    #[inline(always)]
    pub fn is_msk_chnl(&self) -> bool {
        *self == CMD_A::MSK_CHNL
    }
    #[doc = "Checks if the value of the field is `MSK_UPLNGTH`"]
    #[inline(always)]
    pub fn is_msk_uplngth(&self) -> bool {
        *self == CMD_A::MSK_UPLNGTH
    }
    #[doc = "Checks if the value of the field is `MSK_10BIT`"]
    #[inline(always)]
    pub fn is_msk_10bit(&self) -> bool {
        *self == CMD_A::MSK_10BIT
    }
    #[doc = "Checks if the value of the field is `MSK_LSB`"]
    #[inline(always)]
    pub fn is_msk_lsb(&self) -> bool {
        *self == CMD_A::MSK_LSB
    }
    #[doc = "Checks if the value of the field is `MSK_CONT`"]
    #[inline(always)]
    pub fn is_msk_cont(&self) -> bool {
        *self == CMD_A::MSK_CONT
    }
    #[doc = "Checks if the value of the field is `MSK_OPER`"]
    #[inline(always)]
    pub fn is_msk_oper(&self) -> bool {
        *self == CMD_A::MSK_OPER
    }
}
#[doc = "Write proxy for field `CMD`"]
pub struct CMD_W<'a> {
    w: &'a mut W,
}
impl<'a> CMD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMD_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "LSB bit position of the CMD LENGTH field."]
    #[inline(always)]
    pub fn pos_length(self) -> &'a mut W {
        self.variant(CMD_A::POS_LENGTH)
    }
    #[doc = "LSB bit position of the CMD OFFSET field."]
    #[inline(always)]
    pub fn pos_offset(self) -> &'a mut W {
        self.variant(CMD_A::POS_OFFSET)
    }
    #[doc = "LSB bit position of the I2C CMD ADDRESS field."]
    #[inline(always)]
    pub fn pos_address(self) -> &'a mut W {
        self.variant(CMD_A::POS_ADDRESS)
    }
    #[doc = "LSB bit position of the SPI CMD UPLNGTH field."]
    #[inline(always)]
    pub fn pos_uplngth(self) -> &'a mut W {
        self.variant(CMD_A::POS_UPLNGTH)
    }
    #[doc = "LSB bit position of the I2C CMD 10-bit field."]
    #[inline(always)]
    pub fn pos_10bit(self) -> &'a mut W {
        self.variant(CMD_A::POS_10BIT)
    }
    #[doc = "LSB bit position of the CMD LSB-first field."]
    #[inline(always)]
    pub fn pos_lsb(self) -> &'a mut W {
        self.variant(CMD_A::POS_LSB)
    }
    #[doc = "LSB bit position of the CMD CONTinue field."]
    #[inline(always)]
    pub fn pos_cont(self) -> &'a mut W {
        self.variant(CMD_A::POS_CONT)
    }
    #[doc = "LSB bit position of the CMD OPERation field."]
    #[inline(always)]
    pub fn pos_oper(self) -> &'a mut W {
        self.variant(CMD_A::POS_OPER)
    }
    #[doc = "LSB bit mask of the CMD LENGTH field."]
    #[inline(always)]
    pub fn msk_length(self) -> &'a mut W {
        self.variant(CMD_A::MSK_LENGTH)
    }
    #[doc = "LSB bit mask of the CMD OFFSET field."]
    #[inline(always)]
    pub fn msk_offset(self) -> &'a mut W {
        self.variant(CMD_A::MSK_OFFSET)
    }
    #[doc = "LSB bit mask of the I2C CMD ADDRESS field."]
    #[inline(always)]
    pub fn msk_address(self) -> &'a mut W {
        self.variant(CMD_A::MSK_ADDRESS)
    }
    #[doc = "LSB bit mask of the SPI CMD CHANNEL field."]
    #[inline(always)]
    pub fn msk_chnl(self) -> &'a mut W {
        self.variant(CMD_A::MSK_CHNL)
    }
    #[doc = "LSB bit mask of the SPI CMD UPLNGTH field."]
    #[inline(always)]
    pub fn msk_uplngth(self) -> &'a mut W {
        self.variant(CMD_A::MSK_UPLNGTH)
    }
    #[doc = "LSB bit mask of the I2C CMD 10-bit field."]
    #[inline(always)]
    pub fn msk_10bit(self) -> &'a mut W {
        self.variant(CMD_A::MSK_10BIT)
    }
    #[doc = "LSB bit mask of the CMD LSB-first field."]
    #[inline(always)]
    pub fn msk_lsb(self) -> &'a mut W {
        self.variant(CMD_A::MSK_LSB)
    }
    #[doc = "LSB bit mask of the CMD CONTinue field."]
    #[inline(always)]
    pub fn msk_cont(self) -> &'a mut W {
        self.variant(CMD_A::MSK_CONT)
    }
    #[doc = "LSB bit mask of the CMD OPERation field."]
    #[inline(always)]
    pub fn msk_oper(self) -> &'a mut W {
        self.variant(CMD_A::MSK_OPER)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - This register is the I/O Command."]
    #[inline(always)]
    pub fn cmd(&self) -> CMD_R {
        CMD_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - This register is the I/O Command."]
    #[inline(always)]
    pub fn cmd(&mut self) -> CMD_W {
        CMD_W { w: self }
    }
}
