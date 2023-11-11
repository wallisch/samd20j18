#[doc = "Register `CLKCTRL` reader"]
pub type R = crate::R<CLKCTRL_SPEC>;
#[doc = "Register `CLKCTRL` writer"]
pub type W = crate::W<CLKCTRL_SPEC>;
#[doc = "Field `ID` reader - Generic Clock Selection ID"]
pub type ID_R = crate::FieldReader<IDSELECT_A>;
#[doc = "Generic Clock Selection ID\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IDSELECT_A {
    #[doc = "0: DFLL48M Reference"]
    DFLL48M = 0,
    #[doc = "1: WDT"]
    WDT = 1,
    #[doc = "2: RTC"]
    RTC = 2,
    #[doc = "3: EIC"]
    EIC = 3,
    #[doc = "4: EVSYS_CHANNEL_0"]
    EVSYS_CHANNEL_0 = 4,
    #[doc = "5: EVSYS_CHANNEL_1"]
    EVSYS_CHANNEL_1 = 5,
    #[doc = "6: EVSYS_CHANNEL_2"]
    EVSYS_CHANNEL_2 = 6,
    #[doc = "7: EVSYS_CHANNEL_3"]
    EVSYS_CHANNEL_3 = 7,
    #[doc = "8: EVSYS_CHANNEL_4"]
    EVSYS_CHANNEL_4 = 8,
    #[doc = "9: EVSYS_CHANNEL_5"]
    EVSYS_CHANNEL_5 = 9,
    #[doc = "10: EVSYS_CHANNEL_6"]
    EVSYS_CHANNEL_6 = 10,
    #[doc = "11: EVSYS_CHANNEL_7"]
    EVSYS_CHANNEL_7 = 11,
    #[doc = "12: SERCOMx_SLOW"]
    SERCOMX_SLOW = 12,
    #[doc = "13: SERCOM0_CORE"]
    SERCOM0_CORE = 13,
    #[doc = "14: SERCOM1_CORE"]
    SERCOM1_CORE = 14,
    #[doc = "15: SERCOM2_CORE"]
    SERCOM2_CORE = 15,
    #[doc = "16: SERCOM3_CORE"]
    SERCOM3_CORE = 16,
    #[doc = "17: SERCOM4_CORE"]
    SERCOM4_CORE = 17,
    #[doc = "18: SERCOM5_CORE"]
    SERCOM5_CORE = 18,
    #[doc = "19: TC0,TC1"]
    TC0_TC1 = 19,
    #[doc = "20: TC2,TC3"]
    TC2_TC3 = 20,
    #[doc = "21: TC4,TC5"]
    TC4_TC5 = 21,
    #[doc = "22: TC6,TC7"]
    TC6_TC7 = 22,
    #[doc = "23: ADC"]
    ADC = 23,
    #[doc = "24: AC_DIG"]
    AC_DIG = 24,
    #[doc = "25: AC_ANA"]
    AC_ANA = 25,
    #[doc = "26: DAC"]
    DAC = 26,
}
impl From<IDSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: IDSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for IDSELECT_A {
    type Ux = u8;
}
impl ID_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<IDSELECT_A> {
        match self.bits {
            0 => Some(IDSELECT_A::DFLL48M),
            1 => Some(IDSELECT_A::WDT),
            2 => Some(IDSELECT_A::RTC),
            3 => Some(IDSELECT_A::EIC),
            4 => Some(IDSELECT_A::EVSYS_CHANNEL_0),
            5 => Some(IDSELECT_A::EVSYS_CHANNEL_1),
            6 => Some(IDSELECT_A::EVSYS_CHANNEL_2),
            7 => Some(IDSELECT_A::EVSYS_CHANNEL_3),
            8 => Some(IDSELECT_A::EVSYS_CHANNEL_4),
            9 => Some(IDSELECT_A::EVSYS_CHANNEL_5),
            10 => Some(IDSELECT_A::EVSYS_CHANNEL_6),
            11 => Some(IDSELECT_A::EVSYS_CHANNEL_7),
            12 => Some(IDSELECT_A::SERCOMX_SLOW),
            13 => Some(IDSELECT_A::SERCOM0_CORE),
            14 => Some(IDSELECT_A::SERCOM1_CORE),
            15 => Some(IDSELECT_A::SERCOM2_CORE),
            16 => Some(IDSELECT_A::SERCOM3_CORE),
            17 => Some(IDSELECT_A::SERCOM4_CORE),
            18 => Some(IDSELECT_A::SERCOM5_CORE),
            19 => Some(IDSELECT_A::TC0_TC1),
            20 => Some(IDSELECT_A::TC2_TC3),
            21 => Some(IDSELECT_A::TC4_TC5),
            22 => Some(IDSELECT_A::TC6_TC7),
            23 => Some(IDSELECT_A::ADC),
            24 => Some(IDSELECT_A::AC_DIG),
            25 => Some(IDSELECT_A::AC_ANA),
            26 => Some(IDSELECT_A::DAC),
            _ => None,
        }
    }
    #[doc = "DFLL48M Reference"]
    #[inline(always)]
    pub fn is_dfll48m(&self) -> bool {
        *self == IDSELECT_A::DFLL48M
    }
    #[doc = "WDT"]
    #[inline(always)]
    pub fn is_wdt(&self) -> bool {
        *self == IDSELECT_A::WDT
    }
    #[doc = "RTC"]
    #[inline(always)]
    pub fn is_rtc(&self) -> bool {
        *self == IDSELECT_A::RTC
    }
    #[doc = "EIC"]
    #[inline(always)]
    pub fn is_eic(&self) -> bool {
        *self == IDSELECT_A::EIC
    }
    #[doc = "EVSYS_CHANNEL_0"]
    #[inline(always)]
    pub fn is_evsys_channel_0(&self) -> bool {
        *self == IDSELECT_A::EVSYS_CHANNEL_0
    }
    #[doc = "EVSYS_CHANNEL_1"]
    #[inline(always)]
    pub fn is_evsys_channel_1(&self) -> bool {
        *self == IDSELECT_A::EVSYS_CHANNEL_1
    }
    #[doc = "EVSYS_CHANNEL_2"]
    #[inline(always)]
    pub fn is_evsys_channel_2(&self) -> bool {
        *self == IDSELECT_A::EVSYS_CHANNEL_2
    }
    #[doc = "EVSYS_CHANNEL_3"]
    #[inline(always)]
    pub fn is_evsys_channel_3(&self) -> bool {
        *self == IDSELECT_A::EVSYS_CHANNEL_3
    }
    #[doc = "EVSYS_CHANNEL_4"]
    #[inline(always)]
    pub fn is_evsys_channel_4(&self) -> bool {
        *self == IDSELECT_A::EVSYS_CHANNEL_4
    }
    #[doc = "EVSYS_CHANNEL_5"]
    #[inline(always)]
    pub fn is_evsys_channel_5(&self) -> bool {
        *self == IDSELECT_A::EVSYS_CHANNEL_5
    }
    #[doc = "EVSYS_CHANNEL_6"]
    #[inline(always)]
    pub fn is_evsys_channel_6(&self) -> bool {
        *self == IDSELECT_A::EVSYS_CHANNEL_6
    }
    #[doc = "EVSYS_CHANNEL_7"]
    #[inline(always)]
    pub fn is_evsys_channel_7(&self) -> bool {
        *self == IDSELECT_A::EVSYS_CHANNEL_7
    }
    #[doc = "SERCOMx_SLOW"]
    #[inline(always)]
    pub fn is_sercomx_slow(&self) -> bool {
        *self == IDSELECT_A::SERCOMX_SLOW
    }
    #[doc = "SERCOM0_CORE"]
    #[inline(always)]
    pub fn is_sercom0_core(&self) -> bool {
        *self == IDSELECT_A::SERCOM0_CORE
    }
    #[doc = "SERCOM1_CORE"]
    #[inline(always)]
    pub fn is_sercom1_core(&self) -> bool {
        *self == IDSELECT_A::SERCOM1_CORE
    }
    #[doc = "SERCOM2_CORE"]
    #[inline(always)]
    pub fn is_sercom2_core(&self) -> bool {
        *self == IDSELECT_A::SERCOM2_CORE
    }
    #[doc = "SERCOM3_CORE"]
    #[inline(always)]
    pub fn is_sercom3_core(&self) -> bool {
        *self == IDSELECT_A::SERCOM3_CORE
    }
    #[doc = "SERCOM4_CORE"]
    #[inline(always)]
    pub fn is_sercom4_core(&self) -> bool {
        *self == IDSELECT_A::SERCOM4_CORE
    }
    #[doc = "SERCOM5_CORE"]
    #[inline(always)]
    pub fn is_sercom5_core(&self) -> bool {
        *self == IDSELECT_A::SERCOM5_CORE
    }
    #[doc = "TC0,TC1"]
    #[inline(always)]
    pub fn is_tc0_tc1(&self) -> bool {
        *self == IDSELECT_A::TC0_TC1
    }
    #[doc = "TC2,TC3"]
    #[inline(always)]
    pub fn is_tc2_tc3(&self) -> bool {
        *self == IDSELECT_A::TC2_TC3
    }
    #[doc = "TC4,TC5"]
    #[inline(always)]
    pub fn is_tc4_tc5(&self) -> bool {
        *self == IDSELECT_A::TC4_TC5
    }
    #[doc = "TC6,TC7"]
    #[inline(always)]
    pub fn is_tc6_tc7(&self) -> bool {
        *self == IDSELECT_A::TC6_TC7
    }
    #[doc = "ADC"]
    #[inline(always)]
    pub fn is_adc(&self) -> bool {
        *self == IDSELECT_A::ADC
    }
    #[doc = "AC_DIG"]
    #[inline(always)]
    pub fn is_ac_dig(&self) -> bool {
        *self == IDSELECT_A::AC_DIG
    }
    #[doc = "AC_ANA"]
    #[inline(always)]
    pub fn is_ac_ana(&self) -> bool {
        *self == IDSELECT_A::AC_ANA
    }
    #[doc = "DAC"]
    #[inline(always)]
    pub fn is_dac(&self) -> bool {
        *self == IDSELECT_A::DAC
    }
}
#[doc = "Field `ID` writer - Generic Clock Selection ID"]
pub type ID_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O, IDSELECT_A>;
impl<'a, REG, const O: u8> ID_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DFLL48M Reference"]
    #[inline(always)]
    pub fn dfll48m(self) -> &'a mut crate::W<REG> {
        self.variant(IDSELECT_A::DFLL48M)
    }
    #[doc = "WDT"]
    #[inline(always)]
    pub fn wdt(self) -> &'a mut crate::W<REG> {
        self.variant(IDSELECT_A::WDT)
    }
    #[doc = "RTC"]
    #[inline(always)]
    pub fn rtc(self) -> &'a mut crate::W<REG> {
        self.variant(IDSELECT_A::RTC)
    }
    #[doc = "EIC"]
    #[inline(always)]
    pub fn eic(self) -> &'a mut crate::W<REG> {
        self.variant(IDSELECT_A::EIC)
    }
    #[doc = "EVSYS_CHANNEL_0"]
    #[inline(always)]
    pub fn evsys_channel_0(self) -> &'a mut crate::W<REG> {
        self.variant(IDSELECT_A::EVSYS_CHANNEL_0)
    }
    #[doc = "EVSYS_CHANNEL_1"]
    #[inline(always)]
    pub fn evsys_channel_1(self) -> &'a mut crate::W<REG> {
        self.variant(IDSELECT_A::EVSYS_CHANNEL_1)
    }
    #[doc = "EVSYS_CHANNEL_2"]
    #[inline(always)]
    pub fn evsys_channel_2(self) -> &'a mut crate::W<REG> {
        self.variant(IDSELECT_A::EVSYS_CHANNEL_2)
    }
    #[doc = "EVSYS_CHANNEL_3"]
    #[inline(always)]
    pub fn evsys_channel_3(self) -> &'a mut crate::W<REG> {
        self.variant(IDSELECT_A::EVSYS_CHANNEL_3)
    }
    #[doc = "EVSYS_CHANNEL_4"]
    #[inline(always)]
    pub fn evsys_channel_4(self) -> &'a mut crate::W<REG> {
        self.variant(IDSELECT_A::EVSYS_CHANNEL_4)
    }
    #[doc = "EVSYS_CHANNEL_5"]
    #[inline(always)]
    pub fn evsys_channel_5(self) -> &'a mut crate::W<REG> {
        self.variant(IDSELECT_A::EVSYS_CHANNEL_5)
    }
    #[doc = "EVSYS_CHANNEL_6"]
    #[inline(always)]
    pub fn evsys_channel_6(self) -> &'a mut crate::W<REG> {
        self.variant(IDSELECT_A::EVSYS_CHANNEL_6)
    }
    #[doc = "EVSYS_CHANNEL_7"]
    #[inline(always)]
    pub fn evsys_channel_7(self) -> &'a mut crate::W<REG> {
        self.variant(IDSELECT_A::EVSYS_CHANNEL_7)
    }
    #[doc = "SERCOMx_SLOW"]
    #[inline(always)]
    pub fn sercomx_slow(self) -> &'a mut crate::W<REG> {
        self.variant(IDSELECT_A::SERCOMX_SLOW)
    }
    #[doc = "SERCOM0_CORE"]
    #[inline(always)]
    pub fn sercom0_core(self) -> &'a mut crate::W<REG> {
        self.variant(IDSELECT_A::SERCOM0_CORE)
    }
    #[doc = "SERCOM1_CORE"]
    #[inline(always)]
    pub fn sercom1_core(self) -> &'a mut crate::W<REG> {
        self.variant(IDSELECT_A::SERCOM1_CORE)
    }
    #[doc = "SERCOM2_CORE"]
    #[inline(always)]
    pub fn sercom2_core(self) -> &'a mut crate::W<REG> {
        self.variant(IDSELECT_A::SERCOM2_CORE)
    }
    #[doc = "SERCOM3_CORE"]
    #[inline(always)]
    pub fn sercom3_core(self) -> &'a mut crate::W<REG> {
        self.variant(IDSELECT_A::SERCOM3_CORE)
    }
    #[doc = "SERCOM4_CORE"]
    #[inline(always)]
    pub fn sercom4_core(self) -> &'a mut crate::W<REG> {
        self.variant(IDSELECT_A::SERCOM4_CORE)
    }
    #[doc = "SERCOM5_CORE"]
    #[inline(always)]
    pub fn sercom5_core(self) -> &'a mut crate::W<REG> {
        self.variant(IDSELECT_A::SERCOM5_CORE)
    }
    #[doc = "TC0,TC1"]
    #[inline(always)]
    pub fn tc0_tc1(self) -> &'a mut crate::W<REG> {
        self.variant(IDSELECT_A::TC0_TC1)
    }
    #[doc = "TC2,TC3"]
    #[inline(always)]
    pub fn tc2_tc3(self) -> &'a mut crate::W<REG> {
        self.variant(IDSELECT_A::TC2_TC3)
    }
    #[doc = "TC4,TC5"]
    #[inline(always)]
    pub fn tc4_tc5(self) -> &'a mut crate::W<REG> {
        self.variant(IDSELECT_A::TC4_TC5)
    }
    #[doc = "TC6,TC7"]
    #[inline(always)]
    pub fn tc6_tc7(self) -> &'a mut crate::W<REG> {
        self.variant(IDSELECT_A::TC6_TC7)
    }
    #[doc = "ADC"]
    #[inline(always)]
    pub fn adc(self) -> &'a mut crate::W<REG> {
        self.variant(IDSELECT_A::ADC)
    }
    #[doc = "AC_DIG"]
    #[inline(always)]
    pub fn ac_dig(self) -> &'a mut crate::W<REG> {
        self.variant(IDSELECT_A::AC_DIG)
    }
    #[doc = "AC_ANA"]
    #[inline(always)]
    pub fn ac_ana(self) -> &'a mut crate::W<REG> {
        self.variant(IDSELECT_A::AC_ANA)
    }
    #[doc = "DAC"]
    #[inline(always)]
    pub fn dac(self) -> &'a mut crate::W<REG> {
        self.variant(IDSELECT_A::DAC)
    }
}
#[doc = "Field `GEN` reader - Generic Clock Generator"]
pub type GEN_R = crate::FieldReader<GENSELECT_A>;
#[doc = "Generic Clock Generator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GENSELECT_A {
    #[doc = "0: Generic clock generator 0"]
    GCLK0 = 0,
    #[doc = "1: Generic clock generator 1"]
    GCLK1 = 1,
    #[doc = "2: Generic clock generator 2"]
    GCLK2 = 2,
    #[doc = "3: Generic clock generator 3"]
    GCLK3 = 3,
    #[doc = "4: Generic clock generator 4"]
    GCLK4 = 4,
    #[doc = "5: Generic clock generator 5"]
    GCLK5 = 5,
    #[doc = "6: Generic clock generator 6"]
    GCLK6 = 6,
    #[doc = "7: Generic clock generator 7"]
    GCLK7 = 7,
}
impl From<GENSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: GENSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for GENSELECT_A {
    type Ux = u8;
}
impl GEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<GENSELECT_A> {
        match self.bits {
            0 => Some(GENSELECT_A::GCLK0),
            1 => Some(GENSELECT_A::GCLK1),
            2 => Some(GENSELECT_A::GCLK2),
            3 => Some(GENSELECT_A::GCLK3),
            4 => Some(GENSELECT_A::GCLK4),
            5 => Some(GENSELECT_A::GCLK5),
            6 => Some(GENSELECT_A::GCLK6),
            7 => Some(GENSELECT_A::GCLK7),
            _ => None,
        }
    }
    #[doc = "Generic clock generator 0"]
    #[inline(always)]
    pub fn is_gclk0(&self) -> bool {
        *self == GENSELECT_A::GCLK0
    }
    #[doc = "Generic clock generator 1"]
    #[inline(always)]
    pub fn is_gclk1(&self) -> bool {
        *self == GENSELECT_A::GCLK1
    }
    #[doc = "Generic clock generator 2"]
    #[inline(always)]
    pub fn is_gclk2(&self) -> bool {
        *self == GENSELECT_A::GCLK2
    }
    #[doc = "Generic clock generator 3"]
    #[inline(always)]
    pub fn is_gclk3(&self) -> bool {
        *self == GENSELECT_A::GCLK3
    }
    #[doc = "Generic clock generator 4"]
    #[inline(always)]
    pub fn is_gclk4(&self) -> bool {
        *self == GENSELECT_A::GCLK4
    }
    #[doc = "Generic clock generator 5"]
    #[inline(always)]
    pub fn is_gclk5(&self) -> bool {
        *self == GENSELECT_A::GCLK5
    }
    #[doc = "Generic clock generator 6"]
    #[inline(always)]
    pub fn is_gclk6(&self) -> bool {
        *self == GENSELECT_A::GCLK6
    }
    #[doc = "Generic clock generator 7"]
    #[inline(always)]
    pub fn is_gclk7(&self) -> bool {
        *self == GENSELECT_A::GCLK7
    }
}
#[doc = "Field `GEN` writer - Generic Clock Generator"]
pub type GEN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O, GENSELECT_A>;
impl<'a, REG, const O: u8> GEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Generic clock generator 0"]
    #[inline(always)]
    pub fn gclk0(self) -> &'a mut crate::W<REG> {
        self.variant(GENSELECT_A::GCLK0)
    }
    #[doc = "Generic clock generator 1"]
    #[inline(always)]
    pub fn gclk1(self) -> &'a mut crate::W<REG> {
        self.variant(GENSELECT_A::GCLK1)
    }
    #[doc = "Generic clock generator 2"]
    #[inline(always)]
    pub fn gclk2(self) -> &'a mut crate::W<REG> {
        self.variant(GENSELECT_A::GCLK2)
    }
    #[doc = "Generic clock generator 3"]
    #[inline(always)]
    pub fn gclk3(self) -> &'a mut crate::W<REG> {
        self.variant(GENSELECT_A::GCLK3)
    }
    #[doc = "Generic clock generator 4"]
    #[inline(always)]
    pub fn gclk4(self) -> &'a mut crate::W<REG> {
        self.variant(GENSELECT_A::GCLK4)
    }
    #[doc = "Generic clock generator 5"]
    #[inline(always)]
    pub fn gclk5(self) -> &'a mut crate::W<REG> {
        self.variant(GENSELECT_A::GCLK5)
    }
    #[doc = "Generic clock generator 6"]
    #[inline(always)]
    pub fn gclk6(self) -> &'a mut crate::W<REG> {
        self.variant(GENSELECT_A::GCLK6)
    }
    #[doc = "Generic clock generator 7"]
    #[inline(always)]
    pub fn gclk7(self) -> &'a mut crate::W<REG> {
        self.variant(GENSELECT_A::GCLK7)
    }
}
#[doc = "Field `CLKEN` reader - Clock Enable"]
pub type CLKEN_R = crate::BitReader;
#[doc = "Field `CLKEN` writer - Clock Enable"]
pub type CLKEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WRTLOCK` reader - Write Lock"]
pub type WRTLOCK_R = crate::BitReader;
#[doc = "Field `WRTLOCK` writer - Write Lock"]
pub type WRTLOCK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:5 - Generic Clock Selection ID"]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:11 - Generic Clock Generator"]
    #[inline(always)]
    pub fn gen(&self) -> GEN_R {
        GEN_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 14 - Clock Enable"]
    #[inline(always)]
    pub fn clken(&self) -> CLKEN_R {
        CLKEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Write Lock"]
    #[inline(always)]
    pub fn wrtlock(&self) -> WRTLOCK_R {
        WRTLOCK_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Generic Clock Selection ID"]
    #[inline(always)]
    #[must_use]
    pub fn id(&mut self) -> ID_W<CLKCTRL_SPEC, 0> {
        ID_W::new(self)
    }
    #[doc = "Bits 8:11 - Generic Clock Generator"]
    #[inline(always)]
    #[must_use]
    pub fn gen(&mut self) -> GEN_W<CLKCTRL_SPEC, 8> {
        GEN_W::new(self)
    }
    #[doc = "Bit 14 - Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn clken(&mut self) -> CLKEN_W<CLKCTRL_SPEC, 14> {
        CLKEN_W::new(self)
    }
    #[doc = "Bit 15 - Write Lock"]
    #[inline(always)]
    #[must_use]
    pub fn wrtlock(&mut self) -> WRTLOCK_W<CLKCTRL_SPEC, 15> {
        WRTLOCK_W::new(self)
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
#[doc = "Generic Clock Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLKCTRL_SPEC;
impl crate::RegisterSpec for CLKCTRL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`clkctrl::R`](R) reader structure"]
impl crate::Readable for CLKCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clkctrl::W`](W) writer structure"]
impl crate::Writable for CLKCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLKCTRL to value 0"]
impl crate::Resettable for CLKCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
