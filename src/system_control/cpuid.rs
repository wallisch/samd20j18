#[doc = "Register `CPUID` reader"]
pub type R = crate::R<CPUID_SPEC>;
#[doc = "Field `REVISION` reader - Minor revision number"]
pub type REVISION_R = crate::FieldReader;
#[doc = "Field `PARTNO` reader - Processor Part Number, 0xC60=Cortex-M0+"]
pub type PARTNO_R = crate::FieldReader<u16>;
#[doc = "Field `ARCHITECTURE` reader - Processor Architecture, 0xC=ARMv6-M"]
pub type ARCHITECTURE_R = crate::FieldReader;
#[doc = "Field `VARIANT` reader - Major revision number"]
pub type VARIANT_R = crate::FieldReader;
#[doc = "Field `IMPLEMENTER` reader - Implementer code, ARM=0x41"]
pub type IMPLEMENTER_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Minor revision number"]
    #[inline(always)]
    pub fn revision(&self) -> REVISION_R {
        REVISION_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:15 - Processor Part Number, 0xC60=Cortex-M0+"]
    #[inline(always)]
    pub fn partno(&self) -> PARTNO_R {
        PARTNO_R::new(((self.bits >> 4) & 0x0fff) as u16)
    }
    #[doc = "Bits 16:19 - Processor Architecture, 0xC=ARMv6-M"]
    #[inline(always)]
    pub fn architecture(&self) -> ARCHITECTURE_R {
        ARCHITECTURE_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Major revision number"]
    #[inline(always)]
    pub fn variant(&self) -> VARIANT_R {
        VARIANT_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:31 - Implementer code, ARM=0x41"]
    #[inline(always)]
    pub fn implementer(&self) -> IMPLEMENTER_R {
        IMPLEMENTER_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "CPUID Base Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpuid::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CPUID_SPEC;
impl crate::RegisterSpec for CPUID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpuid::R`](R) reader structure"]
impl crate::Readable for CPUID_SPEC {}
#[doc = "`reset()` method sets CPUID to value 0x410c_c601"]
impl crate::Resettable for CPUID_SPEC {
    const RESET_VALUE: Self::Ux = 0x410c_c601;
}
