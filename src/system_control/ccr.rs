#[doc = "Register `CCR` reader"]
pub type R = crate::R<CCR_SPEC>;
#[doc = "Field `UNALIGN_TRP` reader - Unaligned accesses generates a Hard Fault"]
pub type UNALIGN_TRP_R = crate::BitReader<UNALIGN_TRPSELECT_A>;
#[doc = "Unaligned accesses generates a Hard Fault\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UNALIGN_TRPSELECT_A {
    #[doc = "0: Do not trap unaligned halfword and word accesses"]
    VALUE_0 = 0,
    #[doc = "1: Trap unaligned halfword and word accesses"]
    VALUE_1 = 1,
}
impl From<UNALIGN_TRPSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: UNALIGN_TRPSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl UNALIGN_TRP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UNALIGN_TRPSELECT_A {
        match self.bits {
            false => UNALIGN_TRPSELECT_A::VALUE_0,
            true => UNALIGN_TRPSELECT_A::VALUE_1,
        }
    }
    #[doc = "Do not trap unaligned halfword and word accesses"]
    #[inline(always)]
    pub fn is_value_0(&self) -> bool {
        *self == UNALIGN_TRPSELECT_A::VALUE_0
    }
    #[doc = "Trap unaligned halfword and word accesses"]
    #[inline(always)]
    pub fn is_value_1(&self) -> bool {
        *self == UNALIGN_TRPSELECT_A::VALUE_1
    }
}
#[doc = "Field `STKALIGN` reader - Stack 8-byte aligned on exception entry"]
pub type STKALIGN_R = crate::BitReader<STKALIGNSELECT_A>;
#[doc = "Stack 8-byte aligned on exception entry\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STKALIGNSELECT_A {
    #[doc = "0: 4-byte aligned"]
    VALUE_0 = 0,
    #[doc = "1: 8-byte aligned"]
    VALUE_1 = 1,
}
impl From<STKALIGNSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: STKALIGNSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl STKALIGN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> STKALIGNSELECT_A {
        match self.bits {
            false => STKALIGNSELECT_A::VALUE_0,
            true => STKALIGNSELECT_A::VALUE_1,
        }
    }
    #[doc = "4-byte aligned"]
    #[inline(always)]
    pub fn is_value_0(&self) -> bool {
        *self == STKALIGNSELECT_A::VALUE_0
    }
    #[doc = "8-byte aligned"]
    #[inline(always)]
    pub fn is_value_1(&self) -> bool {
        *self == STKALIGNSELECT_A::VALUE_1
    }
}
impl R {
    #[doc = "Bit 3 - Unaligned accesses generates a Hard Fault"]
    #[inline(always)]
    pub fn unalign_trp(&self) -> UNALIGN_TRP_R {
        UNALIGN_TRP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 9 - Stack 8-byte aligned on exception entry"]
    #[inline(always)]
    pub fn stkalign(&self) -> STKALIGN_R {
        STKALIGN_R::new(((self.bits >> 9) & 1) != 0)
    }
}
#[doc = "Configuration and Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCR_SPEC;
impl crate::RegisterSpec for CCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccr::R`](R) reader structure"]
impl crate::Readable for CCR_SPEC {}
#[doc = "`reset()` method sets CCR to value 0x0204"]
impl crate::Resettable for CCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0204;
}
