#[doc = "Reader of register WTA"]
pub type R = crate::R<u32, super::WTA>;
#[doc = "Writer for register WTA"]
pub type W = crate::W<u32, super::WTA>;
#[doc = "Register WTA `reset()`'s with value 0"]
impl crate::ResetValue for super::WTA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WTA`"]
pub type WTA_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `WTA`"]
pub struct WTA_W<'a> {
    w: &'a mut W,
}
impl<'a> WTA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - GPIO31-0 write data."]
    #[inline(always)]
    pub fn wta(&self) -> WTA_R {
        WTA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - GPIO31-0 write data."]
    #[inline(always)]
    pub fn wta(&mut self) -> WTA_W {
        WTA_W { w: self }
    }
}
