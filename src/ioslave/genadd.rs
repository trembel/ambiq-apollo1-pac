#[doc = "Reader of register GENADD"]
pub type R = crate::R<u32, super::GENADD>;
#[doc = "Writer for register GENADD"]
pub type W = crate::W<u32, super::GENADD>;
#[doc = "Register GENADD `reset()`'s with value 0"]
impl crate::ResetValue for super::GENADD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `GADATA`"]
pub type GADATA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `GADATA`"]
pub struct GADATA_W<'a> {
    w: &'a mut W,
}
impl<'a> GADATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - The data supplied on the last General Address reference."]
    #[inline(always)]
    pub fn gadata(&self) -> GADATA_R {
        GADATA_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - The data supplied on the last General Address reference."]
    #[inline(always)]
    pub fn gadata(&mut self) -> GADATA_W {
        GADATA_W { w: self }
    }
}
