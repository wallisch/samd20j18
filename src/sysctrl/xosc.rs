#[doc = "Register `XOSC` reader"]
pub type R = crate::R<XOSC_SPEC>;
#[doc = "Register `XOSC` writer"]
pub type W = crate::W<XOSC_SPEC>;
#[doc = "Field `ENABLE` reader - Oscillator Enable"]
pub type ENABLE_R = crate::BitReader;
#[doc = "Field `ENABLE` writer - Oscillator Enable"]
pub type ENABLE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `XTALEN` reader - Crystal Oscillator Enable"]
pub type XTALEN_R = crate::BitReader;
#[doc = "Field `XTALEN` writer - Crystal Oscillator Enable"]
pub type XTALEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RUNSTDBY` reader - Run in Standby"]
pub type RUNSTDBY_R = crate::BitReader;
#[doc = "Field `RUNSTDBY` writer - Run in Standby"]
pub type RUNSTDBY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ONDEMAND` reader - On Demand Control"]
pub type ONDEMAND_R = crate::BitReader;
#[doc = "Field `ONDEMAND` writer - On Demand Control"]
pub type ONDEMAND_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GAIN` reader - Oscillator Gain"]
pub type GAIN_R = crate::FieldReader<GAINSELECT_A>;
#[doc = "Oscillator Gain\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GAINSELECT_A {
    #[doc = "0: 2MHz"]
    _0 = 0,
    #[doc = "1: 4MHz"]
    _1 = 1,
    #[doc = "2: 8MHz"]
    _2 = 2,
    #[doc = "3: 16MHz"]
    _3 = 3,
    #[doc = "4: 30MHz"]
    _4 = 4,
}
impl From<GAINSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: GAINSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for GAINSELECT_A {
    type Ux = u8;
}
impl GAIN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<GAINSELECT_A> {
        match self.bits {
            0 => Some(GAINSELECT_A::_0),
            1 => Some(GAINSELECT_A::_1),
            2 => Some(GAINSELECT_A::_2),
            3 => Some(GAINSELECT_A::_3),
            4 => Some(GAINSELECT_A::_4),
            _ => None,
        }
    }
    #[doc = "2MHz"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == GAINSELECT_A::_0
    }
    #[doc = "4MHz"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == GAINSELECT_A::_1
    }
    #[doc = "8MHz"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        *self == GAINSELECT_A::_2
    }
    #[doc = "16MHz"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        *self == GAINSELECT_A::_3
    }
    #[doc = "30MHz"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        *self == GAINSELECT_A::_4
    }
}
#[doc = "Field `GAIN` writer - Oscillator Gain"]
pub type GAIN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O, GAINSELECT_A>;
impl<'a, REG, const O: u8> GAIN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "2MHz"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(GAINSELECT_A::_0)
    }
    #[doc = "4MHz"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(GAINSELECT_A::_1)
    }
    #[doc = "8MHz"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut crate::W<REG> {
        self.variant(GAINSELECT_A::_2)
    }
    #[doc = "16MHz"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut crate::W<REG> {
        self.variant(GAINSELECT_A::_3)
    }
    #[doc = "30MHz"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut crate::W<REG> {
        self.variant(GAINSELECT_A::_4)
    }
}
#[doc = "Field `AMPGC` reader - Automatic Amplitude Gain Control"]
pub type AMPGC_R = crate::BitReader;
#[doc = "Field `AMPGC` writer - Automatic Amplitude Gain Control"]
pub type AMPGC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `STARTUP` reader - Start-Up Time"]
pub type STARTUP_R = crate::FieldReader<STARTUPSELECT_A>;
#[doc = "Start-Up Time\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum STARTUPSELECT_A {
    #[doc = "0: 31 us"]
    CYCLE1 = 0,
    #[doc = "1: 61 us"]
    CYCLE2 = 1,
    #[doc = "2: 122 us"]
    CYCLE4 = 2,
    #[doc = "3: 244 us"]
    CYCLE8 = 3,
    #[doc = "4: 488 us"]
    CYCLE16 = 4,
    #[doc = "5: 977 us"]
    CYCLE32 = 5,
    #[doc = "6: 1953 us"]
    CYCLE64 = 6,
    #[doc = "7: 3906 us"]
    CYCLE128 = 7,
    #[doc = "8: 7813 us"]
    CYCLE256 = 8,
    #[doc = "9: 15625 us"]
    CYCLE512 = 9,
    #[doc = "10: 31250 us"]
    CYCLE1024 = 10,
    #[doc = "11: 62500 us"]
    CYCLE2048 = 11,
    #[doc = "12: 125000 us"]
    CYCLE4096 = 12,
    #[doc = "13: 250000 us"]
    CYCLE8192 = 13,
    #[doc = "14: 500000 us"]
    CYCLE16384 = 14,
    #[doc = "15: 1000000 us"]
    CYCLE32768 = 15,
}
impl From<STARTUPSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: STARTUPSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for STARTUPSELECT_A {
    type Ux = u8;
}
impl STARTUP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> STARTUPSELECT_A {
        match self.bits {
            0 => STARTUPSELECT_A::CYCLE1,
            1 => STARTUPSELECT_A::CYCLE2,
            2 => STARTUPSELECT_A::CYCLE4,
            3 => STARTUPSELECT_A::CYCLE8,
            4 => STARTUPSELECT_A::CYCLE16,
            5 => STARTUPSELECT_A::CYCLE32,
            6 => STARTUPSELECT_A::CYCLE64,
            7 => STARTUPSELECT_A::CYCLE128,
            8 => STARTUPSELECT_A::CYCLE256,
            9 => STARTUPSELECT_A::CYCLE512,
            10 => STARTUPSELECT_A::CYCLE1024,
            11 => STARTUPSELECT_A::CYCLE2048,
            12 => STARTUPSELECT_A::CYCLE4096,
            13 => STARTUPSELECT_A::CYCLE8192,
            14 => STARTUPSELECT_A::CYCLE16384,
            15 => STARTUPSELECT_A::CYCLE32768,
            _ => unreachable!(),
        }
    }
    #[doc = "31 us"]
    #[inline(always)]
    pub fn is_cycle1(&self) -> bool {
        *self == STARTUPSELECT_A::CYCLE1
    }
    #[doc = "61 us"]
    #[inline(always)]
    pub fn is_cycle2(&self) -> bool {
        *self == STARTUPSELECT_A::CYCLE2
    }
    #[doc = "122 us"]
    #[inline(always)]
    pub fn is_cycle4(&self) -> bool {
        *self == STARTUPSELECT_A::CYCLE4
    }
    #[doc = "244 us"]
    #[inline(always)]
    pub fn is_cycle8(&self) -> bool {
        *self == STARTUPSELECT_A::CYCLE8
    }
    #[doc = "488 us"]
    #[inline(always)]
    pub fn is_cycle16(&self) -> bool {
        *self == STARTUPSELECT_A::CYCLE16
    }
    #[doc = "977 us"]
    #[inline(always)]
    pub fn is_cycle32(&self) -> bool {
        *self == STARTUPSELECT_A::CYCLE32
    }
    #[doc = "1953 us"]
    #[inline(always)]
    pub fn is_cycle64(&self) -> bool {
        *self == STARTUPSELECT_A::CYCLE64
    }
    #[doc = "3906 us"]
    #[inline(always)]
    pub fn is_cycle128(&self) -> bool {
        *self == STARTUPSELECT_A::CYCLE128
    }
    #[doc = "7813 us"]
    #[inline(always)]
    pub fn is_cycle256(&self) -> bool {
        *self == STARTUPSELECT_A::CYCLE256
    }
    #[doc = "15625 us"]
    #[inline(always)]
    pub fn is_cycle512(&self) -> bool {
        *self == STARTUPSELECT_A::CYCLE512
    }
    #[doc = "31250 us"]
    #[inline(always)]
    pub fn is_cycle1024(&self) -> bool {
        *self == STARTUPSELECT_A::CYCLE1024
    }
    #[doc = "62500 us"]
    #[inline(always)]
    pub fn is_cycle2048(&self) -> bool {
        *self == STARTUPSELECT_A::CYCLE2048
    }
    #[doc = "125000 us"]
    #[inline(always)]
    pub fn is_cycle4096(&self) -> bool {
        *self == STARTUPSELECT_A::CYCLE4096
    }
    #[doc = "250000 us"]
    #[inline(always)]
    pub fn is_cycle8192(&self) -> bool {
        *self == STARTUPSELECT_A::CYCLE8192
    }
    #[doc = "500000 us"]
    #[inline(always)]
    pub fn is_cycle16384(&self) -> bool {
        *self == STARTUPSELECT_A::CYCLE16384
    }
    #[doc = "1000000 us"]
    #[inline(always)]
    pub fn is_cycle32768(&self) -> bool {
        *self == STARTUPSELECT_A::CYCLE32768
    }
}
#[doc = "Field `STARTUP` writer - Start-Up Time"]
pub type STARTUP_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 4, O, STARTUPSELECT_A>;
impl<'a, REG, const O: u8> STARTUP_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "31 us"]
    #[inline(always)]
    pub fn cycle1(self) -> &'a mut crate::W<REG> {
        self.variant(STARTUPSELECT_A::CYCLE1)
    }
    #[doc = "61 us"]
    #[inline(always)]
    pub fn cycle2(self) -> &'a mut crate::W<REG> {
        self.variant(STARTUPSELECT_A::CYCLE2)
    }
    #[doc = "122 us"]
    #[inline(always)]
    pub fn cycle4(self) -> &'a mut crate::W<REG> {
        self.variant(STARTUPSELECT_A::CYCLE4)
    }
    #[doc = "244 us"]
    #[inline(always)]
    pub fn cycle8(self) -> &'a mut crate::W<REG> {
        self.variant(STARTUPSELECT_A::CYCLE8)
    }
    #[doc = "488 us"]
    #[inline(always)]
    pub fn cycle16(self) -> &'a mut crate::W<REG> {
        self.variant(STARTUPSELECT_A::CYCLE16)
    }
    #[doc = "977 us"]
    #[inline(always)]
    pub fn cycle32(self) -> &'a mut crate::W<REG> {
        self.variant(STARTUPSELECT_A::CYCLE32)
    }
    #[doc = "1953 us"]
    #[inline(always)]
    pub fn cycle64(self) -> &'a mut crate::W<REG> {
        self.variant(STARTUPSELECT_A::CYCLE64)
    }
    #[doc = "3906 us"]
    #[inline(always)]
    pub fn cycle128(self) -> &'a mut crate::W<REG> {
        self.variant(STARTUPSELECT_A::CYCLE128)
    }
    #[doc = "7813 us"]
    #[inline(always)]
    pub fn cycle256(self) -> &'a mut crate::W<REG> {
        self.variant(STARTUPSELECT_A::CYCLE256)
    }
    #[doc = "15625 us"]
    #[inline(always)]
    pub fn cycle512(self) -> &'a mut crate::W<REG> {
        self.variant(STARTUPSELECT_A::CYCLE512)
    }
    #[doc = "31250 us"]
    #[inline(always)]
    pub fn cycle1024(self) -> &'a mut crate::W<REG> {
        self.variant(STARTUPSELECT_A::CYCLE1024)
    }
    #[doc = "62500 us"]
    #[inline(always)]
    pub fn cycle2048(self) -> &'a mut crate::W<REG> {
        self.variant(STARTUPSELECT_A::CYCLE2048)
    }
    #[doc = "125000 us"]
    #[inline(always)]
    pub fn cycle4096(self) -> &'a mut crate::W<REG> {
        self.variant(STARTUPSELECT_A::CYCLE4096)
    }
    #[doc = "250000 us"]
    #[inline(always)]
    pub fn cycle8192(self) -> &'a mut crate::W<REG> {
        self.variant(STARTUPSELECT_A::CYCLE8192)
    }
    #[doc = "500000 us"]
    #[inline(always)]
    pub fn cycle16384(self) -> &'a mut crate::W<REG> {
        self.variant(STARTUPSELECT_A::CYCLE16384)
    }
    #[doc = "1000000 us"]
    #[inline(always)]
    pub fn cycle32768(self) -> &'a mut crate::W<REG> {
        self.variant(STARTUPSELECT_A::CYCLE32768)
    }
}
impl R {
    #[doc = "Bit 1 - Oscillator Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Crystal Oscillator Enable"]
    #[inline(always)]
    pub fn xtalen(&self) -> XTALEN_R {
        XTALEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 6 - Run in Standby"]
    #[inline(always)]
    pub fn runstdby(&self) -> RUNSTDBY_R {
        RUNSTDBY_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - On Demand Control"]
    #[inline(always)]
    pub fn ondemand(&self) -> ONDEMAND_R {
        ONDEMAND_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Oscillator Gain"]
    #[inline(always)]
    pub fn gain(&self) -> GAIN_R {
        GAIN_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - Automatic Amplitude Gain Control"]
    #[inline(always)]
    pub fn ampgc(&self) -> AMPGC_R {
        AMPGC_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:15 - Start-Up Time"]
    #[inline(always)]
    pub fn startup(&self) -> STARTUP_R {
        STARTUP_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Oscillator Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<XOSC_SPEC, 1> {
        ENABLE_W::new(self)
    }
    #[doc = "Bit 2 - Crystal Oscillator Enable"]
    #[inline(always)]
    #[must_use]
    pub fn xtalen(&mut self) -> XTALEN_W<XOSC_SPEC, 2> {
        XTALEN_W::new(self)
    }
    #[doc = "Bit 6 - Run in Standby"]
    #[inline(always)]
    #[must_use]
    pub fn runstdby(&mut self) -> RUNSTDBY_W<XOSC_SPEC, 6> {
        RUNSTDBY_W::new(self)
    }
    #[doc = "Bit 7 - On Demand Control"]
    #[inline(always)]
    #[must_use]
    pub fn ondemand(&mut self) -> ONDEMAND_W<XOSC_SPEC, 7> {
        ONDEMAND_W::new(self)
    }
    #[doc = "Bits 8:10 - Oscillator Gain"]
    #[inline(always)]
    #[must_use]
    pub fn gain(&mut self) -> GAIN_W<XOSC_SPEC, 8> {
        GAIN_W::new(self)
    }
    #[doc = "Bit 11 - Automatic Amplitude Gain Control"]
    #[inline(always)]
    #[must_use]
    pub fn ampgc(&mut self) -> AMPGC_W<XOSC_SPEC, 11> {
        AMPGC_W::new(self)
    }
    #[doc = "Bits 12:15 - Start-Up Time"]
    #[inline(always)]
    #[must_use]
    pub fn startup(&mut self) -> STARTUP_W<XOSC_SPEC, 12> {
        STARTUP_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "External Multipurpose Crystal Oscillator (XOSC) Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`xosc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`xosc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct XOSC_SPEC;
impl crate::RegisterSpec for XOSC_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`xosc::R`](R) reader structure"]
impl crate::Readable for XOSC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`xosc::W`](W) writer structure"]
impl crate::Writable for XOSC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets XOSC to value 0x80"]
impl crate::Resettable for XOSC_SPEC {
    const RESET_VALUE: Self::Ux = 0x80;
}
