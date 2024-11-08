use core::fmt::{Display, Formatter};

/// System register type
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SystemRegType {
    /// System register OSDTRRX_EL1
    OSDTRRX_EL1 = 0x8002,
    /// System register DBGBVR0_EL1
    DBGBVR0_EL1 = 0x8004,
    /// System register DBGBCR0_EL1
    DBGBCR0_EL1 = 0x8005,
    /// System register DBGWVR0_EL1
    DBGWVR0_EL1 = 0x8006,
    /// System register DBGWCR0_EL1
    DBGWCR0_EL1 = 0x8007,
    /// System register DBGBVR1_EL1
    DBGBVR1_EL1 = 0x800c,
    /// System register DBGBCR1_EL1
    DBGBCR1_EL1 = 0x800d,
    /// System register DBGWVR1_EL1
    DBGWVR1_EL1 = 0x800e,
    /// System register DBGWCR1_EL1
    DBGWCR1_EL1 = 0x800f,
    /// System register MDCCINT_EL1
    MDCCINT_EL1 = 0x8010,
    /// System register MDSCR_EL1
    MDSCR_EL1 = 0x8012,
    /// System register DBGBVR2_EL1
    DBGBVR2_EL1 = 0x8014,
    /// System register DBGBCR2_EL1
    DBGBCR2_EL1 = 0x8015,
    /// System register DBGWVR2_EL1
    DBGWVR2_EL1 = 0x8016,
    /// System register DBGWCR2_EL1
    DBGWCR2_EL1 = 0x8017,
    /// System register OSDTRTX_EL1
    OSDTRTX_EL1 = 0x801a,
    /// System register DBGBVR3_EL1
    DBGBVR3_EL1 = 0x801c,
    /// System register DBGBCR3_EL1
    DBGBCR3_EL1 = 0x801d,
    /// System register DBGWVR3_EL1
    DBGWVR3_EL1 = 0x801e,
    /// System register DBGWCR3_EL1
    DBGWCR3_EL1 = 0x801f,
    /// System register DBGBVR4_EL1
    DBGBVR4_EL1 = 0x8024,
    /// System register DBGBCR4_EL1
    DBGBCR4_EL1 = 0x8025,
    /// System register DBGWVR4_EL1
    DBGWVR4_EL1 = 0x8026,
    /// System register DBGWCR4_EL1
    DBGWCR4_EL1 = 0x8027,
    /// System register DBGBVR5_EL1
    DBGBVR5_EL1 = 0x802c,
    /// System register DBGBCR5_EL1
    DBGBCR5_EL1 = 0x802d,
    /// System register DBGWVR5_EL1
    DBGWVR5_EL1 = 0x802e,
    /// System register DBGWCR5_EL1
    DBGWCR5_EL1 = 0x802f,
    /// System register OSECCR_EL1
    OSECCR_EL1 = 0x8032,
    /// System register DBGBVR6_EL1
    DBGBVR6_EL1 = 0x8034,
    /// System register DBGBCR6_EL1
    DBGBCR6_EL1 = 0x8035,
    /// System register DBGWVR6_EL1
    DBGWVR6_EL1 = 0x8036,
    /// System register DBGWCR6_EL1
    DBGWCR6_EL1 = 0x8037,
    /// System register DBGBVR7_EL1
    DBGBVR7_EL1 = 0x803c,
    /// System register DBGBCR7_EL1
    DBGBCR7_EL1 = 0x803d,
    /// System register DBGWVR7_EL1
    DBGWVR7_EL1 = 0x803e,
    /// System register DBGWCR7_EL1
    DBGWCR7_EL1 = 0x803f,
    /// System register DBGBVR8_EL1
    DBGBVR8_EL1 = 0x8044,
    /// System register DBGBCR8_EL1
    DBGBCR8_EL1 = 0x8045,
    /// System register DBGWVR8_EL1
    DBGWVR8_EL1 = 0x8046,
    /// System register DBGWCR8_EL1
    DBGWCR8_EL1 = 0x8047,
    /// System register DBGBVR9_EL1
    DBGBVR9_EL1 = 0x804c,
    /// System register DBGBCR9_EL1
    DBGBCR9_EL1 = 0x804d,
    /// System register DBGWVR9_EL1
    DBGWVR9_EL1 = 0x804e,
    /// System register DBGWCR9_EL1
    DBGWCR9_EL1 = 0x804f,
    /// System register DBGBVR10_EL1
    DBGBVR10_EL1 = 0x8054,
    /// System register DBGBCR10_EL1
    DBGBCR10_EL1 = 0x8055,
    /// System register DBGWVR10_EL1
    DBGWVR10_EL1 = 0x8056,
    /// System register DBGWCR10_EL1
    DBGWCR10_EL1 = 0x8057,
    /// System register DBGBVR11_EL1
    DBGBVR11_EL1 = 0x805c,
    /// System register DBGBCR11_EL1
    DBGBCR11_EL1 = 0x805d,
    /// System register DBGWVR11_EL1
    DBGWVR11_EL1 = 0x805e,
    /// System register DBGWCR11_EL1
    DBGWCR11_EL1 = 0x805f,
    /// System register DBGBVR12_EL1
    DBGBVR12_EL1 = 0x8064,
    /// System register DBGBCR12_EL1
    DBGBCR12_EL1 = 0x8065,
    /// System register DBGWVR12_EL1
    DBGWVR12_EL1 = 0x8066,
    /// System register DBGWCR12_EL1
    DBGWCR12_EL1 = 0x8067,
    /// System register DBGBVR13_EL1
    DBGBVR13_EL1 = 0x806c,
    /// System register DBGBCR13_EL1
    DBGBCR13_EL1 = 0x806d,
    /// System register DBGWVR13_EL1
    DBGWVR13_EL1 = 0x806e,
    /// System register DBGWCR13_EL1
    DBGWCR13_EL1 = 0x806f,
    /// System register DBGBVR14_EL1
    DBGBVR14_EL1 = 0x8074,
    /// System register DBGBCR14_EL1
    DBGBCR14_EL1 = 0x8075,
    /// System register DBGWVR14_EL1
    DBGWVR14_EL1 = 0x8076,
    /// System register DBGWCR14_EL1
    DBGWCR14_EL1 = 0x8077,
    /// System register DBGBVR15_EL1
    DBGBVR15_EL1 = 0x807c,
    /// System register DBGBCR15_EL1
    DBGBCR15_EL1 = 0x807d,
    /// System register DBGWVR15_EL1
    DBGWVR15_EL1 = 0x807e,
    /// System register DBGWCR15_EL1
    DBGWCR15_EL1 = 0x807f,
    /// System register OSLAR_EL1
    OSLAR_EL1 = 0x8084,
    /// System register OSDLR_EL1
    OSDLR_EL1 = 0x809c,
    /// System register DBGPRCR_EL1
    DBGPRCR_EL1 = 0x80a4,
    /// System register DBGCLAIMSET_EL1
    DBGCLAIMSET_EL1 = 0x83c6,
    /// System register DBGCLAIMCLR_EL1
    DBGCLAIMCLR_EL1 = 0x83ce,
    /// System register TRCTRACEIDR
    TRCTRACEIDR = 0x8801,
    /// System register TRCVICTLR
    TRCVICTLR = 0x8802,
    /// System register TRCSEQEVR0
    TRCSEQEVR0 = 0x8804,
    /// System register TRCCNTRLDVR0
    TRCCNTRLDVR0 = 0x8805,
    /// System register TRCIMSPEC0
    TRCIMSPEC0 = 0x8807,
    /// System register TRCPRGCTLR
    TRCPRGCTLR = 0x8808,
    /// System register TRCQCTLR
    TRCQCTLR = 0x8809,
    /// System register TRCVIIECTLR
    TRCVIIECTLR = 0x880a,
    /// System register TRCSEQEVR1
    TRCSEQEVR1 = 0x880c,
    /// System register TRCCNTRLDVR1
    TRCCNTRLDVR1 = 0x880d,
    /// System register TRCIMSPEC1
    TRCIMSPEC1 = 0x880f,
    /// System register TRCPROCSELR
    TRCPROCSELR = 0x8810,
    /// System register TRCVISSCTLR
    TRCVISSCTLR = 0x8812,
    /// System register TRCSEQEVR2
    TRCSEQEVR2 = 0x8814,
    /// System register TRCCNTRLDVR2
    TRCCNTRLDVR2 = 0x8815,
    /// System register TRCIMSPEC2
    TRCIMSPEC2 = 0x8817,
    /// System register TRCVIPCSSCTLR
    TRCVIPCSSCTLR = 0x881a,
    /// System register TRCCNTRLDVR3
    TRCCNTRLDVR3 = 0x881d,
    /// System register TRCIMSPEC3
    TRCIMSPEC3 = 0x881f,
    /// System register TRCCONFIGR
    TRCCONFIGR = 0x8820,
    /// System register TRCCNTCTLR0
    TRCCNTCTLR0 = 0x8825,
    /// System register TRCIMSPEC4
    TRCIMSPEC4 = 0x8827,
    /// System register TRCCNTCTLR1
    TRCCNTCTLR1 = 0x882d,
    /// System register TRCIMSPEC5
    TRCIMSPEC5 = 0x882f,
    /// System register TRCAUXCTLR
    TRCAUXCTLR = 0x8830,
    /// System register TRCSEQRSTEVR
    TRCSEQRSTEVR = 0x8834,
    /// System register TRCCNTCTLR2
    TRCCNTCTLR2 = 0x8835,
    /// System register TRCIMSPEC6
    TRCIMSPEC6 = 0x8837,
    /// System register TRCSEQSTR
    TRCSEQSTR = 0x883c,
    /// System register TRCCNTCTLR3
    TRCCNTCTLR3 = 0x883d,
    /// System register TRCIMSPEC7
    TRCIMSPEC7 = 0x883f,
    /// System register TRCEVENTCTL0R
    TRCEVENTCTL0R = 0x8840,
    /// System register TRCVDCTLR
    TRCVDCTLR = 0x8842,
    /// System register TRCEXTINSELR
    TRCEXTINSELR = 0x8844,
    /// System register TRCCNTVR0
    TRCCNTVR0 = 0x8845,
    /// System register TRCEVENTCTL1R
    TRCEVENTCTL1R = 0x8848,
    /// System register TRCVDSACCTLR
    TRCVDSACCTLR = 0x884a,
    /// System register TRCEXTINSELR1
    TRCEXTINSELR1 = 0x884c,
    /// System register TRCCNTVR1
    TRCCNTVR1 = 0x884d,
    /// System register TRCRSR
    TRCRSR = 0x8850,
    /// System register TRCVDARCCTLR
    TRCVDARCCTLR = 0x8852,
    /// System register TRCEXTINSELR2
    TRCEXTINSELR2 = 0x8854,
    /// System register TRCCNTVR2
    TRCCNTVR2 = 0x8855,
    /// System register TRCSTALLCTLR
    TRCSTALLCTLR = 0x8858,
    /// System register TRCEXTINSELR3
    TRCEXTINSELR3 = 0x885c,
    /// System register TRCCNTVR3
    TRCCNTVR3 = 0x885d,
    /// System register TRCTSCTLR
    TRCTSCTLR = 0x8860,
    /// System register TRCSYNCPR
    TRCSYNCPR = 0x8868,
    /// System register TRCCCCTLR
    TRCCCCTLR = 0x8870,
    /// System register TRCBBCTLR
    TRCBBCTLR = 0x8878,
    /// System register TRCRSCTLR16
    TRCRSCTLR16 = 0x8881,
    /// System register TRCSSCCR0
    TRCSSCCR0 = 0x8882,
    /// System register TRCSSPCICR0
    TRCSSPCICR0 = 0x8883,
    /// System register TRCOSLAR
    TRCOSLAR = 0x8884,
    /// System register TRCRSCTLR17
    TRCRSCTLR17 = 0x8889,
    /// System register TRCSSCCR1
    TRCSSCCR1 = 0x888a,
    /// System register TRCSSPCICR1
    TRCSSPCICR1 = 0x888b,
    /// System register TRCRSCTLR2
    TRCRSCTLR2 = 0x8890,
    /// System register TRCRSCTLR18
    TRCRSCTLR18 = 0x8891,
    /// System register TRCSSCCR2
    TRCSSCCR2 = 0x8892,
    /// System register TRCSSPCICR2
    TRCSSPCICR2 = 0x8893,
    /// System register TRCRSCTLR3
    TRCRSCTLR3 = 0x8898,
    /// System register TRCRSCTLR19
    TRCRSCTLR19 = 0x8899,
    /// System register TRCSSCCR3
    TRCSSCCR3 = 0x889a,
    /// System register TRCSSPCICR3
    TRCSSPCICR3 = 0x889b,
    /// System register TRCRSCTLR4
    TRCRSCTLR4 = 0x88a0,
    /// System register TRCRSCTLR20
    TRCRSCTLR20 = 0x88a1,
    /// System register TRCSSCCR4
    TRCSSCCR4 = 0x88a2,
    /// System register TRCSSPCICR4
    TRCSSPCICR4 = 0x88a3,
    /// System register TRCPDCR
    TRCPDCR = 0x88a4,
    /// System register TRCRSCTLR5
    TRCRSCTLR5 = 0x88a8,
    /// System register TRCRSCTLR21
    TRCRSCTLR21 = 0x88a9,
    /// System register TRCSSCCR5
    TRCSSCCR5 = 0x88aa,
    /// System register TRCSSPCICR5
    TRCSSPCICR5 = 0x88ab,
    /// System register TRCRSCTLR6
    TRCRSCTLR6 = 0x88b0,
    /// System register TRCRSCTLR22
    TRCRSCTLR22 = 0x88b1,
    /// System register TRCSSCCR6
    TRCSSCCR6 = 0x88b2,
    /// System register TRCSSPCICR6
    TRCSSPCICR6 = 0x88b3,
    /// System register TRCRSCTLR7
    TRCRSCTLR7 = 0x88b8,
    /// System register TRCRSCTLR23
    TRCRSCTLR23 = 0x88b9,
    /// System register TRCSSCCR7
    TRCSSCCR7 = 0x88ba,
    /// System register TRCSSPCICR7
    TRCSSPCICR7 = 0x88bb,
    /// System register TRCRSCTLR8
    TRCRSCTLR8 = 0x88c0,
    /// System register TRCRSCTLR24
    TRCRSCTLR24 = 0x88c1,
    /// System register TRCSSCSR0
    TRCSSCSR0 = 0x88c2,
    /// System register TRCRSCTLR9
    TRCRSCTLR9 = 0x88c8,
    /// System register TRCRSCTLR25
    TRCRSCTLR25 = 0x88c9,
    /// System register TRCSSCSR1
    TRCSSCSR1 = 0x88ca,
    /// System register TRCRSCTLR10
    TRCRSCTLR10 = 0x88d0,
    /// System register TRCRSCTLR26
    TRCRSCTLR26 = 0x88d1,
    /// System register TRCSSCSR2
    TRCSSCSR2 = 0x88d2,
    /// System register TRCRSCTLR11
    TRCRSCTLR11 = 0x88d8,
    /// System register TRCRSCTLR27
    TRCRSCTLR27 = 0x88d9,
    /// System register TRCSSCSR3
    TRCSSCSR3 = 0x88da,
    /// System register TRCRSCTLR12
    TRCRSCTLR12 = 0x88e0,
    /// System register TRCRSCTLR28
    TRCRSCTLR28 = 0x88e1,
    /// System register TRCSSCSR4
    TRCSSCSR4 = 0x88e2,
    /// System register TRCRSCTLR13
    TRCRSCTLR13 = 0x88e8,
    /// System register TRCRSCTLR29
    TRCRSCTLR29 = 0x88e9,
    /// System register TRCSSCSR5
    TRCSSCSR5 = 0x88ea,
    /// System register TRCRSCTLR14
    TRCRSCTLR14 = 0x88f0,
    /// System register TRCRSCTLR30
    TRCRSCTLR30 = 0x88f1,
    /// System register TRCSSCSR6
    TRCSSCSR6 = 0x88f2,
    /// System register TRCRSCTLR15
    TRCRSCTLR15 = 0x88f8,
    /// System register TRCRSCTLR31
    TRCRSCTLR31 = 0x88f9,
    /// System register TRCSSCSR7
    TRCSSCSR7 = 0x88fa,
    /// System register TRCACVR0
    TRCACVR0 = 0x8900,
    /// System register TRCACVR8
    TRCACVR8 = 0x8901,
    /// System register TRCACATR0
    TRCACATR0 = 0x8902,
    /// System register TRCACATR8
    TRCACATR8 = 0x8903,
    /// System register TRCDVCVR0
    TRCDVCVR0 = 0x8904,
    /// System register TRCDVCVR4
    TRCDVCVR4 = 0x8905,
    /// System register TRCDVCMR0
    TRCDVCMR0 = 0x8906,
    /// System register TRCDVCMR4
    TRCDVCMR4 = 0x8907,
    /// System register TRCACVR1
    TRCACVR1 = 0x8910,
    /// System register TRCACVR9
    TRCACVR9 = 0x8911,
    /// System register TRCACATR1
    TRCACATR1 = 0x8912,
    /// System register TRCACATR9
    TRCACATR9 = 0x8913,
    /// System register TRCACVR2
    TRCACVR2 = 0x8920,
    /// System register TRCACVR10
    TRCACVR10 = 0x8921,
    /// System register TRCACATR2
    TRCACATR2 = 0x8922,
    /// System register TRCACATR10
    TRCACATR10 = 0x8923,
    /// System register TRCDVCVR1
    TRCDVCVR1 = 0x8924,
    /// System register TRCDVCVR5
    TRCDVCVR5 = 0x8925,
    /// System register TRCDVCMR1
    TRCDVCMR1 = 0x8926,
    /// System register TRCDVCMR5
    TRCDVCMR5 = 0x8927,
    /// System register TRCACVR3
    TRCACVR3 = 0x8930,
    /// System register TRCACVR11
    TRCACVR11 = 0x8931,
    /// System register TRCACATR3
    TRCACATR3 = 0x8932,
    /// System register TRCACATR11
    TRCACATR11 = 0x8933,
    /// System register TRCACVR4
    TRCACVR4 = 0x8940,
    /// System register TRCACVR12
    TRCACVR12 = 0x8941,
    /// System register TRCACATR4
    TRCACATR4 = 0x8942,
    /// System register TRCACATR12
    TRCACATR12 = 0x8943,
    /// System register TRCDVCVR2
    TRCDVCVR2 = 0x8944,
    /// System register TRCDVCVR6
    TRCDVCVR6 = 0x8945,
    /// System register TRCDVCMR2
    TRCDVCMR2 = 0x8946,
    /// System register TRCDVCMR6
    TRCDVCMR6 = 0x8947,
    /// System register TRCACVR5
    TRCACVR5 = 0x8950,
    /// System register TRCACVR13
    TRCACVR13 = 0x8951,
    /// System register TRCACATR5
    TRCACATR5 = 0x8952,
    /// System register TRCACATR13
    TRCACATR13 = 0x8953,
    /// System register TRCACVR6
    TRCACVR6 = 0x8960,
    /// System register TRCACVR14
    TRCACVR14 = 0x8961,
    /// System register TRCACATR6
    TRCACATR6 = 0x8962,
    /// System register TRCACATR14
    TRCACATR14 = 0x8963,
    /// System register TRCDVCVR3
    TRCDVCVR3 = 0x8964,
    /// System register TRCDVCVR7
    TRCDVCVR7 = 0x8965,
    /// System register TRCDVCMR3
    TRCDVCMR3 = 0x8966,
    /// System register TRCDVCMR7
    TRCDVCMR7 = 0x8967,
    /// System register TRCACVR7
    TRCACVR7 = 0x8970,
    /// System register TRCACVR15
    TRCACVR15 = 0x8971,
    /// System register TRCACATR7
    TRCACATR7 = 0x8972,
    /// System register TRCACATR15
    TRCACATR15 = 0x8973,
    /// System register TRCCIDCVR0
    TRCCIDCVR0 = 0x8980,
    /// System register TRCVMIDCVR0
    TRCVMIDCVR0 = 0x8981,
    /// System register TRCCIDCCTLR0
    TRCCIDCCTLR0 = 0x8982,
    /// System register TRCCIDCCTLR1
    TRCCIDCCTLR1 = 0x898a,
    /// System register TRCCIDCVR1
    TRCCIDCVR1 = 0x8990,
    /// System register TRCVMIDCVR1
    TRCVMIDCVR1 = 0x8991,
    /// System register TRCVMIDCCTLR0
    TRCVMIDCCTLR0 = 0x8992,
    /// System register TRCVMIDCCTLR1
    TRCVMIDCCTLR1 = 0x899a,
    /// System register TRCCIDCVR2
    TRCCIDCVR2 = 0x89a0,
    /// System register TRCVMIDCVR2
    TRCVMIDCVR2 = 0x89a1,
    /// System register TRCCIDCVR3
    TRCCIDCVR3 = 0x89b0,
    /// System register TRCVMIDCVR3
    TRCVMIDCVR3 = 0x89b1,
    /// System register TRCCIDCVR4
    TRCCIDCVR4 = 0x89c0,
    /// System register TRCVMIDCVR4
    TRCVMIDCVR4 = 0x89c1,
    /// System register TRCCIDCVR5
    TRCCIDCVR5 = 0x89d0,
    /// System register TRCVMIDCVR5
    TRCVMIDCVR5 = 0x89d1,
    /// System register TRCCIDCVR6
    TRCCIDCVR6 = 0x89e0,
    /// System register TRCVMIDCVR6
    TRCVMIDCVR6 = 0x89e1,
    /// System register TRCCIDCVR7
    TRCCIDCVR7 = 0x89f0,
    /// System register TRCVMIDCVR7
    TRCVMIDCVR7 = 0x89f1,
    /// System register TRCITCTRL
    TRCITCTRL = 0x8b84,
    /// System register TRCCLAIMSET
    TRCCLAIMSET = 0x8bc6,
    /// System register TRCCLAIMCLR
    TRCCLAIMCLR = 0x8bce,
    /// System register TRCLAR
    TRCLAR = 0x8be6,
    /// System register TEECR32_EL1
    TEECR32_EL1 = 0x9000,
    /// System register TEEHBR32_EL1
    TEEHBR32_EL1 = 0x9080,
    /// System register DBGDTR_EL0
    DBGDTR_EL0 = 0x9820,
    /// System register DBGDTRTX_EL0
    DBGDTRTX_EL0 = 0x9828,
    /// System register DBGVCR32_EL2
    DBGVCR32_EL2 = 0xa038,
    /// System register SCTLR_EL1
    SCTLR_EL1 = 0xc080,
    /// System register ACTLR_EL1
    ACTLR_EL1 = 0xc081,
    /// System register CPACR_EL1
    CPACR_EL1 = 0xc082,
    /// System register RGSR_EL1
    RGSR_EL1 = 0xc085,
    /// System register GCR_EL1
    GCR_EL1 = 0xc086,
    /// System register TRFCR_EL1
    TRFCR_EL1 = 0xc091,
    /// System register TTBR0_EL1
    TTBR0_EL1 = 0xc100,
    /// System register TTBR1_EL1
    TTBR1_EL1 = 0xc101,
    /// System register TCR_EL1
    TCR_EL1 = 0xc102,
    /// System register APIAKEYLO_EL1
    APIAKEYLO_EL1 = 0xc108,
    /// System register APIAKEYHI_EL1
    APIAKEYHI_EL1 = 0xc109,
    /// System register APIBKEYLO_EL1
    APIBKEYLO_EL1 = 0xc10a,
    /// System register APIBKEYHI_EL1
    APIBKEYHI_EL1 = 0xc10b,
    /// System register APDAKEYLO_EL1
    APDAKEYLO_EL1 = 0xc110,
    /// System register APDAKEYHI_EL1
    APDAKEYHI_EL1 = 0xc111,
    /// System register APDBKEYLO_EL1
    APDBKEYLO_EL1 = 0xc112,
    /// System register APDBKEYHI_EL1
    APDBKEYHI_EL1 = 0xc113,
    /// System register APGAKEYLO_EL1
    APGAKEYLO_EL1 = 0xc118,
    /// System register APGAKEYHI_EL1
    APGAKEYHI_EL1 = 0xc119,
    /// System register SPSR_EL1
    SPSR_EL1 = 0xc200,
    /// System register ELR_EL1
    ELR_EL1 = 0xc201,
    /// System register SP_EL0
    SP_EL0 = 0xc208,
    /// System register SPSEL
    SPSEL = 0xc210,
    /// System register CURRENTEL
    CURRENTEL = 0xc212,
    /// System register PAN
    PAN = 0xc213,
    /// System register UAO
    UAO = 0xc214,
    /// System register ICC_PMR_EL1
    ICC_PMR_EL1 = 0xc230,
    /// System register AFSR0_EL1
    AFSR0_EL1 = 0xc288,
    /// System register AFSR1_EL1
    AFSR1_EL1 = 0xc289,
    /// System register ESR_EL1
    ESR_EL1 = 0xc290,
    /// System register ERRSELR_EL1
    ERRSELR_EL1 = 0xc299,
    /// System register ERXCTLR_EL1
    ERXCTLR_EL1 = 0xc2a1,
    /// System register ERXSTATUS_EL1
    ERXSTATUS_EL1 = 0xc2a2,
    /// System register ERXADDR_EL1
    ERXADDR_EL1 = 0xc2a3,
    /// System register ERXPFGCTL_EL1
    ERXPFGCTL_EL1 = 0xc2a5,
    /// System register ERXPFGCDN_EL1
    ERXPFGCDN_EL1 = 0xc2a6,
    /// System register ERXMISC0_EL1
    ERXMISC0_EL1 = 0xc2a8,
    /// System register ERXMISC1_EL1
    ERXMISC1_EL1 = 0xc2a9,
    /// System register ERXMISC2_EL1
    ERXMISC2_EL1 = 0xc2aa,
    /// System register ERXMISC3_EL1
    ERXMISC3_EL1 = 0xc2ab,
    /// System register ERXTS_EL1
    ERXTS_EL1 = 0xc2af,
    /// System register TFSR_EL1
    TFSR_EL1 = 0xc2b0,
    /// System register TFSRE0_EL1
    TFSRE0_EL1 = 0xc2b1,
    /// System register FAR_EL1
    FAR_EL1 = 0xc300,
    /// System register PAR_EL1
    PAR_EL1 = 0xc3a0,
    /// System register PMSCR_EL1
    PMSCR_EL1 = 0xc4c8,
    /// System register PMSICR_EL1
    PMSICR_EL1 = 0xc4ca,
    /// System register PMSIRR_EL1
    PMSIRR_EL1 = 0xc4cb,
    /// System register PMSFCR_EL1
    PMSFCR_EL1 = 0xc4cc,
    /// System register PMSEVFR_EL1
    PMSEVFR_EL1 = 0xc4cd,
    /// System register PMSLATFR_EL1
    PMSLATFR_EL1 = 0xc4ce,
    /// System register PMSIDR_EL1
    PMSIDR_EL1 = 0xc4cf,
    /// System register PMBLIMITR_EL1
    PMBLIMITR_EL1 = 0xc4d0,
    /// System register PMBPTR_EL1
    PMBPTR_EL1 = 0xc4d1,
    /// System register PMBSR_EL1
    PMBSR_EL1 = 0xc4d3,
    /// System register PMBIDR_EL1
    PMBIDR_EL1 = 0xc4d7,
    /// System register TRBLIMITR_EL1
    TRBLIMITR_EL1 = 0xc4d8,
    /// System register TRBPTR_EL1
    TRBPTR_EL1 = 0xc4d9,
    /// System register TRBBASER_EL1
    TRBBASER_EL1 = 0xc4da,
    /// System register TRBSR_EL1
    TRBSR_EL1 = 0xc4db,
    /// System register TRBMAR_EL1
    TRBMAR_EL1 = 0xc4dc,
    /// System register TRBTRG_EL1
    TRBTRG_EL1 = 0xc4de,
    /// System register PMINTENSET_EL1
    PMINTENSET_EL1 = 0xc4f1,
    /// System register PMINTENCLR_EL1
    PMINTENCLR_EL1 = 0xc4f2,
    /// System register PMMIR_EL1
    PMMIR_EL1 = 0xc4f6,
    /// System register MAIR_EL1
    MAIR_EL1 = 0xc510,
    /// System register AMAIR_EL1
    AMAIR_EL1 = 0xc518,
    /// System register LORSA_EL1
    LORSA_EL1 = 0xc520,
    /// System register LOREA_EL1
    LOREA_EL1 = 0xc521,
    /// System register LORN_EL1
    LORN_EL1 = 0xc522,
    /// System register LORC_EL1
    LORC_EL1 = 0xc523,
    /// System register MPAM1_EL1
    MPAM1_EL1 = 0xc528,
    /// System register MPAM0_EL1
    MPAM0_EL1 = 0xc529,
    /// System register VBAR_EL1
    VBAR_EL1 = 0xc600,
    /// System register RMR_EL1
    RMR_EL1 = 0xc602,
    /// System register DISR_EL1
    DISR_EL1 = 0xc609,
    /// System register ICC_EOIR0_EL1
    ICC_EOIR0_EL1 = 0xc641,
    /// System register ICC_BPR0_EL1
    ICC_BPR0_EL1 = 0xc643,
    /// System register ICC_AP0R0_EL1
    ICC_AP0R0_EL1 = 0xc644,
    /// System register ICC_AP0R1_EL1
    ICC_AP0R1_EL1 = 0xc645,
    /// System register ICC_AP0R2_EL1
    ICC_AP0R2_EL1 = 0xc646,
    /// System register ICC_AP0R3_EL1
    ICC_AP0R3_EL1 = 0xc647,
    /// System register ICC_AP1R0_EL1
    ICC_AP1R0_EL1 = 0xc648,
    /// System register ICC_AP1R1_EL1
    ICC_AP1R1_EL1 = 0xc649,
    /// System register ICC_AP1R2_EL1
    ICC_AP1R2_EL1 = 0xc64a,
    /// System register ICC_AP1R3_EL1
    ICC_AP1R3_EL1 = 0xc64b,
    /// System register ICC_DIR_EL1
    ICC_DIR_EL1 = 0xc659,
    /// System register ICC_SGI1R_EL1
    ICC_SGI1R_EL1 = 0xc65d,
    /// System register ICC_ASGI1R_EL1
    ICC_ASGI1R_EL1 = 0xc65e,
    /// System register ICC_SGI0R_EL1
    ICC_SGI0R_EL1 = 0xc65f,
    /// System register ICC_EOIR1_EL1
    ICC_EOIR1_EL1 = 0xc661,
    /// System register ICC_BPR1_EL1
    ICC_BPR1_EL1 = 0xc663,
    /// System register ICC_CTLR_EL1
    ICC_CTLR_EL1 = 0xc664,
    /// System register ICC_SRE_EL1
    ICC_SRE_EL1 = 0xc665,
    /// System register ICC_IGRPEN0_EL1
    ICC_IGRPEN0_EL1 = 0xc666,
    /// System register ICC_IGRPEN1_EL1
    ICC_IGRPEN1_EL1 = 0xc667,
    /// System register ICC_SEIEN_EL1
    ICC_SEIEN_EL1 = 0xc668,
    /// System register CONTEXTIDR_EL1
    CONTEXTIDR_EL1 = 0xc681,
    /// System register TPIDR_EL1
    TPIDR_EL1 = 0xc684,
    /// System register SCXTNUM_EL1
    SCXTNUM_EL1 = 0xc687,
    /// System register CNTKCTL_EL1
    CNTKCTL_EL1 = 0xc708,
    /// System register CSSELR_EL1
    CSSELR_EL1 = 0xd000,
    /// System register NZCV
    NZCV = 0xda10,
    /// System register DAIFSET
    DAIFSET = 0xda11,
    /// System register DIT
    DIT = 0xda15,
    /// System register SSBS
    SSBS = 0xda16,
    /// System register TCO
    TCO = 0xda17,
    /// System register FPCR
    FPCR = 0xda20,
    /// System register FPSR
    FPSR = 0xda21,
    /// System register DSPSR_EL0
    DSPSR_EL0 = 0xda28,
    /// System register DLR_EL0
    DLR_EL0 = 0xda29,
    /// System register PMCR_EL0
    PMCR_EL0 = 0xdce0,
    /// System register PMCNTENSET_EL0
    PMCNTENSET_EL0 = 0xdce1,
    /// System register PMCNTENCLR_EL0
    PMCNTENCLR_EL0 = 0xdce2,
    /// System register PMOVSCLR_EL0
    PMOVSCLR_EL0 = 0xdce3,
    /// System register PMSWINC_EL0
    PMSWINC_EL0 = 0xdce4,
    /// System register PMSELR_EL0
    PMSELR_EL0 = 0xdce5,
    /// System register PMCCNTR_EL0
    PMCCNTR_EL0 = 0xdce8,
    /// System register PMXEVTYPER_EL0
    PMXEVTYPER_EL0 = 0xdce9,
    /// System register PMXEVCNTR_EL0
    PMXEVCNTR_EL0 = 0xdcea,
    /// System register DAIFCLR
    DAIFCLR = 0xdced,
    /// System register PMUSERENR_EL0
    PMUSERENR_EL0 = 0xdcf0,
    /// System register PMOVSSET_EL0
    PMOVSSET_EL0 = 0xdcf3,
    /// System register TPIDR_EL0
    TPIDR_EL0 = 0xde82,
    /// System register TPIDRRO_EL0
    TPIDRRO_EL0 = 0xde83,
    /// System register SCXTNUM_EL0
    SCXTNUM_EL0 = 0xde87,
    /// System register AMCR_EL0
    AMCR_EL0 = 0xde90,
    /// System register AMUSERENR_EL0
    AMUSERENR_EL0 = 0xde93,
    /// System register AMCNTENCLR0_EL0
    AMCNTENCLR0_EL0 = 0xde94,
    /// System register AMCNTENSET0_EL0
    AMCNTENSET0_EL0 = 0xde95,
    /// System register AMCNTENCLR1_EL0
    AMCNTENCLR1_EL0 = 0xde98,
    /// System register AMCNTENSET1_EL0
    AMCNTENSET1_EL0 = 0xde99,
    /// System register AMEVCNTR00_EL0
    AMEVCNTR00_EL0 = 0xdea0,
    /// System register AMEVCNTR01_EL0
    AMEVCNTR01_EL0 = 0xdea1,
    /// System register AMEVCNTR02_EL0
    AMEVCNTR02_EL0 = 0xdea2,
    /// System register AMEVCNTR03_EL0
    AMEVCNTR03_EL0 = 0xdea3,
    /// System register AMEVCNTR10_EL0
    AMEVCNTR10_EL0 = 0xdee0,
    /// System register AMEVCNTR11_EL0
    AMEVCNTR11_EL0 = 0xdee1,
    /// System register AMEVCNTR12_EL0
    AMEVCNTR12_EL0 = 0xdee2,
    /// System register AMEVCNTR13_EL0
    AMEVCNTR13_EL0 = 0xdee3,
    /// System register AMEVCNTR14_EL0
    AMEVCNTR14_EL0 = 0xdee4,
    /// System register AMEVCNTR15_EL0
    AMEVCNTR15_EL0 = 0xdee5,
    /// System register AMEVCNTR16_EL0
    AMEVCNTR16_EL0 = 0xdee6,
    /// System register AMEVCNTR17_EL0
    AMEVCNTR17_EL0 = 0xdee7,
    /// System register AMEVCNTR18_EL0
    AMEVCNTR18_EL0 = 0xdee8,
    /// System register AMEVCNTR19_EL0
    AMEVCNTR19_EL0 = 0xdee9,
    /// System register AMEVCNTR110_EL0
    AMEVCNTR110_EL0 = 0xdeea,
    /// System register AMEVCNTR111_EL0
    AMEVCNTR111_EL0 = 0xdeeb,
    /// System register AMEVCNTR112_EL0
    AMEVCNTR112_EL0 = 0xdeec,
    /// System register AMEVCNTR113_EL0
    AMEVCNTR113_EL0 = 0xdeed,
    /// System register AMEVCNTR114_EL0
    AMEVCNTR114_EL0 = 0xdeee,
    /// System register AMEVCNTR115_EL0
    AMEVCNTR115_EL0 = 0xdeef,
    /// System register AMEVTYPER10_EL0
    AMEVTYPER10_EL0 = 0xdef0,
    /// System register AMEVTYPER11_EL0
    AMEVTYPER11_EL0 = 0xdef1,
    /// System register AMEVTYPER12_EL0
    AMEVTYPER12_EL0 = 0xdef2,
    /// System register AMEVTYPER13_EL0
    AMEVTYPER13_EL0 = 0xdef3,
    /// System register AMEVTYPER14_EL0
    AMEVTYPER14_EL0 = 0xdef4,
    /// System register AMEVTYPER15_EL0
    AMEVTYPER15_EL0 = 0xdef5,
    /// System register AMEVTYPER16_EL0
    AMEVTYPER16_EL0 = 0xdef6,
    /// System register AMEVTYPER17_EL0
    AMEVTYPER17_EL0 = 0xdef7,
    /// System register AMEVTYPER18_EL0
    AMEVTYPER18_EL0 = 0xdef8,
    /// System register AMEVTYPER19_EL0
    AMEVTYPER19_EL0 = 0xdef9,
    /// System register AMEVTYPER110_EL0
    AMEVTYPER110_EL0 = 0xdefa,
    /// System register AMEVTYPER111_EL0
    AMEVTYPER111_EL0 = 0xdefb,
    /// System register AMEVTYPER112_EL0
    AMEVTYPER112_EL0 = 0xdefc,
    /// System register AMEVTYPER113_EL0
    AMEVTYPER113_EL0 = 0xdefd,
    /// System register AMEVTYPER114_EL0
    AMEVTYPER114_EL0 = 0xdefe,
    /// System register AMEVTYPER115_EL0
    AMEVTYPER115_EL0 = 0xdeff,
    /// System register CNTFRQ_EL0
    CNTFRQ_EL0 = 0xdf00,
    /// System register CNTP_TVAL_EL0
    CNTP_TVAL_EL0 = 0xdf10,
    /// System register CNTP_CTL_EL0
    CNTP_CTL_EL0 = 0xdf11,
    /// System register CNTP_CVAL_EL0
    CNTP_CVAL_EL0 = 0xdf12,
    /// System register CNTV_TVAL_EL0
    CNTV_TVAL_EL0 = 0xdf18,
    /// System register CNTV_CTL_EL0
    CNTV_CTL_EL0 = 0xdf19,
    /// System register CNTV_CVAL_EL0
    CNTV_CVAL_EL0 = 0xdf1a,
    /// System register PMEVCNTR0_EL0
    PMEVCNTR0_EL0 = 0xdf40,
    /// System register PMEVCNTR1_EL0
    PMEVCNTR1_EL0 = 0xdf41,
    /// System register PMEVCNTR2_EL0
    PMEVCNTR2_EL0 = 0xdf42,
    /// System register PMEVCNTR3_EL0
    PMEVCNTR3_EL0 = 0xdf43,
    /// System register PMEVCNTR4_EL0
    PMEVCNTR4_EL0 = 0xdf44,
    /// System register PMEVCNTR5_EL0
    PMEVCNTR5_EL0 = 0xdf45,
    /// System register PMEVCNTR6_EL0
    PMEVCNTR6_EL0 = 0xdf46,
    /// System register PMEVCNTR7_EL0
    PMEVCNTR7_EL0 = 0xdf47,
    /// System register PMEVCNTR8_EL0
    PMEVCNTR8_EL0 = 0xdf48,
    /// System register PMEVCNTR9_EL0
    PMEVCNTR9_EL0 = 0xdf49,
    /// System register PMEVCNTR10_EL0
    PMEVCNTR10_EL0 = 0xdf4a,
    /// System register PMEVCNTR11_EL0
    PMEVCNTR11_EL0 = 0xdf4b,
    /// System register PMEVCNTR12_EL0
    PMEVCNTR12_EL0 = 0xdf4c,
    /// System register PMEVCNTR13_EL0
    PMEVCNTR13_EL0 = 0xdf4d,
    /// System register PMEVCNTR14_EL0
    PMEVCNTR14_EL0 = 0xdf4e,
    /// System register PMEVCNTR15_EL0
    PMEVCNTR15_EL0 = 0xdf4f,
    /// System register PMEVCNTR16_EL0
    PMEVCNTR16_EL0 = 0xdf50,
    /// System register PMEVCNTR17_EL0
    PMEVCNTR17_EL0 = 0xdf51,
    /// System register PMEVCNTR18_EL0
    PMEVCNTR18_EL0 = 0xdf52,
    /// System register PMEVCNTR19_EL0
    PMEVCNTR19_EL0 = 0xdf53,
    /// System register PMEVCNTR20_EL0
    PMEVCNTR20_EL0 = 0xdf54,
    /// System register PMEVCNTR21_EL0
    PMEVCNTR21_EL0 = 0xdf55,
    /// System register PMEVCNTR22_EL0
    PMEVCNTR22_EL0 = 0xdf56,
    /// System register PMEVCNTR23_EL0
    PMEVCNTR23_EL0 = 0xdf57,
    /// System register PMEVCNTR24_EL0
    PMEVCNTR24_EL0 = 0xdf58,
    /// System register PMEVCNTR25_EL0
    PMEVCNTR25_EL0 = 0xdf59,
    /// System register PMEVCNTR26_EL0
    PMEVCNTR26_EL0 = 0xdf5a,
    /// System register PMEVCNTR27_EL0
    PMEVCNTR27_EL0 = 0xdf5b,
    /// System register PMEVCNTR28_EL0
    PMEVCNTR28_EL0 = 0xdf5c,
    /// System register PMEVCNTR29_EL0
    PMEVCNTR29_EL0 = 0xdf5d,
    /// System register PMEVCNTR30_EL0
    PMEVCNTR30_EL0 = 0xdf5e,
    /// System register PMEVTYPER0_EL0
    PMEVTYPER0_EL0 = 0xdf60,
    /// System register PMEVTYPER1_EL0
    PMEVTYPER1_EL0 = 0xdf61,
    /// System register PMEVTYPER2_EL0
    PMEVTYPER2_EL0 = 0xdf62,
    /// System register PMEVTYPER3_EL0
    PMEVTYPER3_EL0 = 0xdf63,
    /// System register PMEVTYPER4_EL0
    PMEVTYPER4_EL0 = 0xdf64,
    /// System register PMEVTYPER5_EL0
    PMEVTYPER5_EL0 = 0xdf65,
    /// System register PMEVTYPER6_EL0
    PMEVTYPER6_EL0 = 0xdf66,
    /// System register PMEVTYPER7_EL0
    PMEVTYPER7_EL0 = 0xdf67,
    /// System register PMEVTYPER8_EL0
    PMEVTYPER8_EL0 = 0xdf68,
    /// System register PMEVTYPER9_EL0
    PMEVTYPER9_EL0 = 0xdf69,
    /// System register PMEVTYPER10_EL0
    PMEVTYPER10_EL0 = 0xdf6a,
    /// System register PMEVTYPER11_EL0
    PMEVTYPER11_EL0 = 0xdf6b,
    /// System register PMEVTYPER12_EL0
    PMEVTYPER12_EL0 = 0xdf6c,
    /// System register PMEVTYPER13_EL0
    PMEVTYPER13_EL0 = 0xdf6d,
    /// System register PMEVTYPER14_EL0
    PMEVTYPER14_EL0 = 0xdf6e,
    /// System register PMEVTYPER15_EL0
    PMEVTYPER15_EL0 = 0xdf6f,
    /// System register PMEVTYPER16_EL0
    PMEVTYPER16_EL0 = 0xdf70,
    /// System register PMEVTYPER17_EL0
    PMEVTYPER17_EL0 = 0xdf71,
    /// System register PMEVTYPER18_EL0
    PMEVTYPER18_EL0 = 0xdf72,
    /// System register PMEVTYPER19_EL0
    PMEVTYPER19_EL0 = 0xdf73,
    /// System register PMEVTYPER20_EL0
    PMEVTYPER20_EL0 = 0xdf74,
    /// System register PMEVTYPER21_EL0
    PMEVTYPER21_EL0 = 0xdf75,
    /// System register PMEVTYPER22_EL0
    PMEVTYPER22_EL0 = 0xdf76,
    /// System register PMEVTYPER23_EL0
    PMEVTYPER23_EL0 = 0xdf77,
    /// System register PMEVTYPER24_EL0
    PMEVTYPER24_EL0 = 0xdf78,
    /// System register PMEVTYPER25_EL0
    PMEVTYPER25_EL0 = 0xdf79,
    /// System register PMEVTYPER26_EL0
    PMEVTYPER26_EL0 = 0xdf7a,
    /// System register PMEVTYPER27_EL0
    PMEVTYPER27_EL0 = 0xdf7b,
    /// System register PMEVTYPER28_EL0
    PMEVTYPER28_EL0 = 0xdf7c,
    /// System register PMEVTYPER29_EL0
    PMEVTYPER29_EL0 = 0xdf7d,
    /// System register PMEVTYPER30_EL0
    PMEVTYPER30_EL0 = 0xdf7e,
    /// System register PMCCFILTR_EL0
    PMCCFILTR_EL0 = 0xdf7f,
    /// System register VPIDR_EL2
    VPIDR_EL2 = 0xe000,
    /// System register VMPIDR_EL2
    VMPIDR_EL2 = 0xe005,
    /// System register SCTLR_EL2
    SCTLR_EL2 = 0xe080,
    /// System register ACTLR_EL2
    ACTLR_EL2 = 0xe081,
    /// System register HCR_EL2
    HCR_EL2 = 0xe088,
    /// System register MDCR_EL2
    MDCR_EL2 = 0xe089,
    /// System register CPTR_EL2
    CPTR_EL2 = 0xe08a,
    /// System register HSTR_EL2
    HSTR_EL2 = 0xe08b,
    /// System register HACR_EL2
    HACR_EL2 = 0xe08f,
    /// System register TRFCR_EL2
    TRFCR_EL2 = 0xe091,
    /// System register SDER32_EL2
    SDER32_EL2 = 0xe099,
    /// System register TTBR0_EL2
    TTBR0_EL2 = 0xe100,
    /// System register TTBR1_EL2
    TTBR1_EL2 = 0xe101,
    /// System register TCR_EL2
    TCR_EL2 = 0xe102,
    /// System register VTTBR_EL2
    VTTBR_EL2 = 0xe108,
    /// System register VTCR_EL2
    VTCR_EL2 = 0xe10a,
    /// System register VNCR_EL2
    VNCR_EL2 = 0xe110,
    /// System register VSTTBR_EL2
    VSTTBR_EL2 = 0xe130,
    /// System register VSTCR_EL2
    VSTCR_EL2 = 0xe132,
    /// System register DACR32_EL2
    DACR32_EL2 = 0xe180,
    /// System register SPSR_EL2
    SPSR_EL2 = 0xe200,
    /// System register ELR_EL2
    ELR_EL2 = 0xe201,
    /// System register SP_EL1
    SP_EL1 = 0xe208,
    /// System register SPSR_IRQ
    SPSR_IRQ = 0xe218,
    /// System register SPSR_ABT
    SPSR_ABT = 0xe219,
    /// System register SPSR_UND
    SPSR_UND = 0xe21a,
    /// System register SPSR_FIQ
    SPSR_FIQ = 0xe21b,
    /// System register IFSR32_EL2
    IFSR32_EL2 = 0xe281,
    /// System register AFSR0_EL2
    AFSR0_EL2 = 0xe288,
    /// System register AFSR1_EL2
    AFSR1_EL2 = 0xe289,
    /// System register ESR_EL2
    ESR_EL2 = 0xe290,
    /// System register VSESR_EL2
    VSESR_EL2 = 0xe293,
    /// System register FPEXC32_EL2
    FPEXC32_EL2 = 0xe298,
    /// System register TFSR_EL2
    TFSR_EL2 = 0xe2b0,
    /// System register FAR_EL2
    FAR_EL2 = 0xe300,
    /// System register HPFAR_EL2
    HPFAR_EL2 = 0xe304,
    /// System register PMSCR_EL2
    PMSCR_EL2 = 0xe4c8,
    /// System register MAIR_EL2
    MAIR_EL2 = 0xe510,
    /// System register AMAIR_EL2
    AMAIR_EL2 = 0xe518,
    /// System register MPAMHCR_EL2
    MPAMHCR_EL2 = 0xe520,
    /// System register MPAMVPMV_EL2
    MPAMVPMV_EL2 = 0xe521,
    /// System register MPAM2_EL2
    MPAM2_EL2 = 0xe528,
    /// System register MPAMVPM0_EL2
    MPAMVPM0_EL2 = 0xe530,
    /// System register MPAMVPM1_EL2
    MPAMVPM1_EL2 = 0xe531,
    /// System register MPAMVPM2_EL2
    MPAMVPM2_EL2 = 0xe532,
    /// System register MPAMVPM3_EL2
    MPAMVPM3_EL2 = 0xe533,
    /// System register MPAMVPM4_EL2
    MPAMVPM4_EL2 = 0xe534,
    /// System register MPAMVPM5_EL2
    MPAMVPM5_EL2 = 0xe535,
    /// System register MPAMVPM6_EL2
    MPAMVPM6_EL2 = 0xe536,
    /// System register MPAMVPM7_EL2
    MPAMVPM7_EL2 = 0xe537,
    /// System register VBAR_EL2
    VBAR_EL2 = 0xe600,
    /// System register RMR_EL2
    RMR_EL2 = 0xe602,
    /// System register VDISR_EL2
    VDISR_EL2 = 0xe609,
    /// System register ICH_AP0R0_EL2
    ICH_AP0R0_EL2 = 0xe640,
    /// System register ICH_AP0R1_EL2
    ICH_AP0R1_EL2 = 0xe641,
    /// System register ICH_AP0R2_EL2
    ICH_AP0R2_EL2 = 0xe642,
    /// System register ICH_AP0R3_EL2
    ICH_AP0R3_EL2 = 0xe643,
    /// System register ICH_AP1R0_EL2
    ICH_AP1R0_EL2 = 0xe648,
    /// System register ICH_AP1R1_EL2
    ICH_AP1R1_EL2 = 0xe649,
    /// System register ICH_AP1R2_EL2
    ICH_AP1R2_EL2 = 0xe64a,
    /// System register ICH_AP1R3_EL2
    ICH_AP1R3_EL2 = 0xe64b,
    /// System register ICH_VSEIR_EL2
    ICH_VSEIR_EL2 = 0xe64c,
    /// System register ICC_SRE_EL2
    ICC_SRE_EL2 = 0xe64d,
    /// System register ICH_HCR_EL2
    ICH_HCR_EL2 = 0xe658,
    /// System register ICH_MISR_EL2
    ICH_MISR_EL2 = 0xe65a,
    /// System register ICH_VMCR_EL2
    ICH_VMCR_EL2 = 0xe65f,
    /// System register ICH_LR0_EL2
    ICH_LR0_EL2 = 0xe660,
    /// System register ICH_LR1_EL2
    ICH_LR1_EL2 = 0xe661,
    /// System register ICH_LR2_EL2
    ICH_LR2_EL2 = 0xe662,
    /// System register ICH_LR3_EL2
    ICH_LR3_EL2 = 0xe663,
    /// System register ICH_LR4_EL2
    ICH_LR4_EL2 = 0xe664,
    /// System register ICH_LR5_EL2
    ICH_LR5_EL2 = 0xe665,
    /// System register ICH_LR6_EL2
    ICH_LR6_EL2 = 0xe666,
    /// System register ICH_LR7_EL2
    ICH_LR7_EL2 = 0xe667,
    /// System register ICH_LR8_EL2
    ICH_LR8_EL2 = 0xe668,
    /// System register ICH_LR9_EL2
    ICH_LR9_EL2 = 0xe669,
    /// System register ICH_LR10_EL2
    ICH_LR10_EL2 = 0xe66a,
    /// System register ICH_LR11_EL2
    ICH_LR11_EL2 = 0xe66b,
    /// System register ICH_LR12_EL2
    ICH_LR12_EL2 = 0xe66c,
    /// System register ICH_LR13_EL2
    ICH_LR13_EL2 = 0xe66d,
    /// System register ICH_LR14_EL2
    ICH_LR14_EL2 = 0xe66e,
    /// System register ICH_LR15_EL2
    ICH_LR15_EL2 = 0xe66f,
    /// System register CONTEXTIDR_EL2
    CONTEXTIDR_EL2 = 0xe681,
    /// System register TPIDR_EL2
    TPIDR_EL2 = 0xe682,
    /// System register SCXTNUM_EL2
    SCXTNUM_EL2 = 0xe687,
    /// System register CNTVOFF_EL2
    CNTVOFF_EL2 = 0xe703,
    /// System register CNTHCTL_EL2
    CNTHCTL_EL2 = 0xe708,
    /// System register CNTHP_TVAL_EL2
    CNTHP_TVAL_EL2 = 0xe710,
    /// System register CNTHP_CTL_EL2
    CNTHP_CTL_EL2 = 0xe711,
    /// System register CNTHP_CVAL_EL2
    CNTHP_CVAL_EL2 = 0xe712,
    /// System register CNTHV_TVAL_EL2
    CNTHV_TVAL_EL2 = 0xe718,
    /// System register CNTHV_CTL_EL2
    CNTHV_CTL_EL2 = 0xe719,
    /// System register CNTHV_CVAL_EL2
    CNTHV_CVAL_EL2 = 0xe71a,
    /// System register CNTHVS_TVAL_EL2
    CNTHVS_TVAL_EL2 = 0xe720,
    /// System register CNTHVS_CTL_EL2
    CNTHVS_CTL_EL2 = 0xe721,
    /// System register CNTHVS_CVAL_EL2
    CNTHVS_CVAL_EL2 = 0xe722,
    /// System register CNTHPS_TVAL_EL2
    CNTHPS_TVAL_EL2 = 0xe728,
    /// System register CNTHPS_CTL_EL2
    CNTHPS_CTL_EL2 = 0xe729,
    /// System register CNTHPS_CVAL_EL2
    CNTHPS_CVAL_EL2 = 0xe72a,
    /// System register SCTLR_EL12
    SCTLR_EL12 = 0xe880,
    /// System register CPACR_EL12
    CPACR_EL12 = 0xe882,
    /// System register TRFCR_EL12
    TRFCR_EL12 = 0xe891,
    /// System register TTBR0_EL12
    TTBR0_EL12 = 0xe900,
    /// System register TTBR1_EL12
    TTBR1_EL12 = 0xe901,
    /// System register TCR_EL12
    TCR_EL12 = 0xe902,
    /// System register SPSR_EL12
    SPSR_EL12 = 0xea00,
    /// System register ELR_EL12
    ELR_EL12 = 0xea01,
    /// System register AFSR0_EL12
    AFSR0_EL12 = 0xea88,
    /// System register AFSR1_EL12
    AFSR1_EL12 = 0xea89,
    /// System register ESR_EL12
    ESR_EL12 = 0xea90,
    /// System register TFSR_EL12
    TFSR_EL12 = 0xeab0,
    /// System register FAR_EL12
    FAR_EL12 = 0xeb00,
    /// System register PMSCR_EL12
    PMSCR_EL12 = 0xecc8,
    /// System register MAIR_EL12
    MAIR_EL12 = 0xed10,
    /// System register AMAIR_EL12
    AMAIR_EL12 = 0xed18,
    /// System register MPAM1_EL12
    MPAM1_EL12 = 0xed28,
    /// System register VBAR_EL12
    VBAR_EL12 = 0xee00,
    /// System register CONTEXTIDR_EL12
    CONTEXTIDR_EL12 = 0xee81,
    /// System register SCXTNUM_EL12
    SCXTNUM_EL12 = 0xee87,
    /// System register CNTKCTL_EL12
    CNTKCTL_EL12 = 0xef08,
    /// System register CNTP_TVAL_EL02
    CNTP_TVAL_EL02 = 0xef10,
    /// System register CNTP_CTL_EL02
    CNTP_CTL_EL02 = 0xef11,
    /// System register CNTP_CVAL_EL02
    CNTP_CVAL_EL02 = 0xef12,
    /// System register CNTV_TVAL_EL02
    CNTV_TVAL_EL02 = 0xef18,
    /// System register CNTV_CTL_EL02
    CNTV_CTL_EL02 = 0xef19,
    /// System register CNTV_CVAL_EL02
    CNTV_CVAL_EL02 = 0xef1a,
    /// System register SCTLR_EL3
    SCTLR_EL3 = 0xf080,
    /// System register ACTLR_EL3
    ACTLR_EL3 = 0xf081,
    /// System register SCR_EL3
    SCR_EL3 = 0xf088,
    /// System register SDER32_EL3
    SDER32_EL3 = 0xf089,
    /// System register CPTR_EL3
    CPTR_EL3 = 0xf08a,
    /// System register MDCR_EL3
    MDCR_EL3 = 0xf099,
    /// System register TTBR0_EL3
    TTBR0_EL3 = 0xf100,
    /// System register TCR_EL3
    TCR_EL3 = 0xf102,
    /// System register SPSR_EL3
    SPSR_EL3 = 0xf200,
    /// System register ELR_EL3
    ELR_EL3 = 0xf201,
    /// System register SP_EL2
    SP_EL2 = 0xf208,
    /// System register AFSR0_EL3
    AFSR0_EL3 = 0xf288,
    /// System register AFSR1_EL3
    AFSR1_EL3 = 0xf289,
    /// System register ESR_EL3
    ESR_EL3 = 0xf290,
    /// System register TFSR_EL3
    TFSR_EL3 = 0xf2b0,
    /// System register FAR_EL3
    FAR_EL3 = 0xf300,
    /// System register MAIR_EL3
    MAIR_EL3 = 0xf510,
    /// System register AMAIR_EL3
    AMAIR_EL3 = 0xf518,
    /// System register MPAM3_EL3
    MPAM3_EL3 = 0xf528,
    /// System register VBAR_EL3
    VBAR_EL3 = 0xf600,
    /// System register RMR_EL3
    RMR_EL3 = 0xf602,
    /// System register ICC_CTLR_EL3
    ICC_CTLR_EL3 = 0xf664,
    /// System register ICC_SRE_EL3
    ICC_SRE_EL3 = 0xf665,
    /// System register ICC_IGRPEN1_EL3
    ICC_IGRPEN1_EL3 = 0xf667,
    /// System register TPIDR_EL3
    TPIDR_EL3 = 0xf682,
    /// System register SCXTNUM_EL3
    SCXTNUM_EL3 = 0xf687,
    /// System register CNTPS_TVAL_EL1
    CNTPS_TVAL_EL1 = 0xff10,
    /// System register CNTPS_CTL_EL1
    CNTPS_CTL_EL1 = 0xff11,
    /// System register CNTPS_CVAL_EL1
    CNTPS_CVAL_EL1 = 0xff12,
    /// System register PSTATE_SPSEL
    PSTATE_SPSEL = 0xff13,
}

impl Display for SystemRegType {
    /// Print system register name
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            SystemRegType::OSDTRRX_EL1 => write!(f, "OSDTRRX_EL1"),
            SystemRegType::DBGBVR0_EL1 => write!(f, "DBGBVR0_EL1"),
            SystemRegType::DBGBCR0_EL1 => write!(f, "DBGBCR0_EL1"),
            SystemRegType::DBGWVR0_EL1 => write!(f, "DBGWVR0_EL1"),
            SystemRegType::DBGWCR0_EL1 => write!(f, "DBGWCR0_EL1"),
            SystemRegType::DBGBVR1_EL1 => write!(f, "DBGBVR1_EL1"),
            SystemRegType::DBGBCR1_EL1 => write!(f, "DBGBCR1_EL1"),
            SystemRegType::DBGWVR1_EL1 => write!(f, "DBGWVR1_EL1"),
            SystemRegType::DBGWCR1_EL1 => write!(f, "DBGWCR1_EL1"),
            SystemRegType::MDCCINT_EL1 => write!(f, "MDCCINT_EL1"),
            SystemRegType::MDSCR_EL1 => write!(f, "MDSCR_EL1"),
            SystemRegType::DBGBVR2_EL1 => write!(f, "DBGBVR2_EL1"),
            SystemRegType::DBGBCR2_EL1 => write!(f, "DBGBCR2_EL1"),
            SystemRegType::DBGWVR2_EL1 => write!(f, "DBGWVR2_EL1"),
            SystemRegType::DBGWCR2_EL1 => write!(f, "DBGWCR2_EL1"),
            SystemRegType::OSDTRTX_EL1 => write!(f, "OSDTRTX_EL1"),
            SystemRegType::DBGBVR3_EL1 => write!(f, "DBGBVR3_EL1"),
            SystemRegType::DBGBCR3_EL1 => write!(f, "DBGBCR3_EL1"),
            SystemRegType::DBGWVR3_EL1 => write!(f, "DBGWVR3_EL1"),
            SystemRegType::DBGWCR3_EL1 => write!(f, "DBGWCR3_EL1"),
            SystemRegType::DBGBVR4_EL1 => write!(f, "DBGBVR4_EL1"),
            SystemRegType::DBGBCR4_EL1 => write!(f, "DBGBCR4_EL1"),
            SystemRegType::DBGWVR4_EL1 => write!(f, "DBGWVR4_EL1"),
            SystemRegType::DBGWCR4_EL1 => write!(f, "DBGWCR4_EL1"),
            SystemRegType::DBGBVR5_EL1 => write!(f, "DBGBVR5_EL1"),
            SystemRegType::DBGBCR5_EL1 => write!(f, "DBGBCR5_EL1"),
            SystemRegType::DBGWVR5_EL1 => write!(f, "DBGWVR5_EL1"),
            SystemRegType::DBGWCR5_EL1 => write!(f, "DBGWCR5_EL1"),
            SystemRegType::OSECCR_EL1 => write!(f, "OSECCR_EL1"),
            SystemRegType::DBGBVR6_EL1 => write!(f, "DBGBVR6_EL1"),
            SystemRegType::DBGBCR6_EL1 => write!(f, "DBGBCR6_EL1"),
            SystemRegType::DBGWVR6_EL1 => write!(f, "DBGWVR6_EL1"),
            SystemRegType::DBGWCR6_EL1 => write!(f, "DBGWCR6_EL1"),
            SystemRegType::DBGBVR7_EL1 => write!(f, "DBGBVR7_EL1"),
            SystemRegType::DBGBCR7_EL1 => write!(f, "DBGBCR7_EL1"),
            SystemRegType::DBGWVR7_EL1 => write!(f, "DBGWVR7_EL1"),
            SystemRegType::DBGWCR7_EL1 => write!(f, "DBGWCR7_EL1"),
            SystemRegType::DBGBVR8_EL1 => write!(f, "DBGBVR8_EL1"),
            SystemRegType::DBGBCR8_EL1 => write!(f, "DBGBCR8_EL1"),
            SystemRegType::DBGWVR8_EL1 => write!(f, "DBGWVR8_EL1"),
            SystemRegType::DBGWCR8_EL1 => write!(f, "DBGWCR8_EL1"),
            SystemRegType::DBGBVR9_EL1 => write!(f, "DBGBVR9_EL1"),
            SystemRegType::DBGBCR9_EL1 => write!(f, "DBGBCR9_EL1"),
            SystemRegType::DBGWVR9_EL1 => write!(f, "DBGWVR9_EL1"),
            SystemRegType::DBGWCR9_EL1 => write!(f, "DBGWCR9_EL1"),
            SystemRegType::DBGBVR10_EL1 => write!(f, "DBGBVR10_EL1"),
            SystemRegType::DBGBCR10_EL1 => write!(f, "DBGBCR10_EL1"),
            SystemRegType::DBGWVR10_EL1 => write!(f, "DBGWVR10_EL1"),
            SystemRegType::DBGWCR10_EL1 => write!(f, "DBGWCR10_EL1"),
            SystemRegType::DBGBVR11_EL1 => write!(f, "DBGBVR11_EL1"),
            SystemRegType::DBGBCR11_EL1 => write!(f, "DBGBCR11_EL1"),
            SystemRegType::DBGWVR11_EL1 => write!(f, "DBGWVR11_EL1"),
            SystemRegType::DBGWCR11_EL1 => write!(f, "DBGWCR11_EL1"),
            SystemRegType::DBGBVR12_EL1 => write!(f, "DBGBVR12_EL1"),
            SystemRegType::DBGBCR12_EL1 => write!(f, "DBGBCR12_EL1"),
            SystemRegType::DBGWVR12_EL1 => write!(f, "DBGWVR12_EL1"),
            SystemRegType::DBGWCR12_EL1 => write!(f, "DBGWCR12_EL1"),
            SystemRegType::DBGBVR13_EL1 => write!(f, "DBGBVR13_EL1"),
            SystemRegType::DBGBCR13_EL1 => write!(f, "DBGBCR13_EL1"),
            SystemRegType::DBGWVR13_EL1 => write!(f, "DBGWVR13_EL1"),
            SystemRegType::DBGWCR13_EL1 => write!(f, "DBGWCR13_EL1"),
            SystemRegType::DBGBVR14_EL1 => write!(f, "DBGBVR14_EL1"),
            SystemRegType::DBGBCR14_EL1 => write!(f, "DBGBCR14_EL1"),
            SystemRegType::DBGWVR14_EL1 => write!(f, "DBGWVR14_EL1"),
            SystemRegType::DBGWCR14_EL1 => write!(f, "DBGWCR14_EL1"),
            SystemRegType::DBGBVR15_EL1 => write!(f, "DBGBVR15_EL1"),
            SystemRegType::DBGBCR15_EL1 => write!(f, "DBGBCR15_EL1"),
            SystemRegType::DBGWVR15_EL1 => write!(f, "DBGWVR15_EL1"),
            SystemRegType::DBGWCR15_EL1 => write!(f, "DBGWCR15_EL1"),
            SystemRegType::OSLAR_EL1 => write!(f, "OSLAR_EL1"),
            SystemRegType::OSDLR_EL1 => write!(f, "OSDLR_EL1"),
            SystemRegType::DBGPRCR_EL1 => write!(f, "DBGPRCR_EL1"),
            SystemRegType::DBGCLAIMSET_EL1 => write!(f, "DBGCLAIMSET_EL1"),
            SystemRegType::DBGCLAIMCLR_EL1 => write!(f, "DBGCLAIMCLR_EL1"),
            SystemRegType::TRCTRACEIDR => write!(f, "TRCTRACEIDR"),
            SystemRegType::TRCVICTLR => write!(f, "TRCVICTLR"),
            SystemRegType::TRCSEQEVR0 => write!(f, "TRCSEQEVR0"),
            SystemRegType::TRCCNTRLDVR0 => write!(f, "TRCCNTRLDVR0"),
            SystemRegType::TRCIMSPEC0 => write!(f, "TRCIMSPEC0"),
            SystemRegType::TRCPRGCTLR => write!(f, "TRCPRGCTLR"),
            SystemRegType::TRCQCTLR => write!(f, "TRCQCTLR"),
            SystemRegType::TRCVIIECTLR => write!(f, "TRCVIIECTLR"),
            SystemRegType::TRCSEQEVR1 => write!(f, "TRCSEQEVR1"),
            SystemRegType::TRCCNTRLDVR1 => write!(f, "TRCCNTRLDVR1"),
            SystemRegType::TRCIMSPEC1 => write!(f, "TRCIMSPEC1"),
            SystemRegType::TRCPROCSELR => write!(f, "TRCPROCSELR"),
            SystemRegType::TRCVISSCTLR => write!(f, "TRCVISSCTLR"),
            SystemRegType::TRCSEQEVR2 => write!(f, "TRCSEQEVR2"),
            SystemRegType::TRCCNTRLDVR2 => write!(f, "TRCCNTRLDVR2"),
            SystemRegType::TRCIMSPEC2 => write!(f, "TRCIMSPEC2"),
            SystemRegType::TRCVIPCSSCTLR => write!(f, "TRCVIPCSSCTLR"),
            SystemRegType::TRCCNTRLDVR3 => write!(f, "TRCCNTRLDVR3"),
            SystemRegType::TRCIMSPEC3 => write!(f, "TRCIMSPEC3"),
            SystemRegType::TRCCONFIGR => write!(f, "TRCCONFIGR"),
            SystemRegType::TRCCNTCTLR0 => write!(f, "TRCCNTCTLR0"),
            SystemRegType::TRCIMSPEC4 => write!(f, "TRCIMSPEC4"),
            SystemRegType::TRCCNTCTLR1 => write!(f, "TRCCNTCTLR1"),
            SystemRegType::TRCIMSPEC5 => write!(f, "TRCIMSPEC5"),
            SystemRegType::TRCAUXCTLR => write!(f, "TRCAUXCTLR"),
            SystemRegType::TRCSEQRSTEVR => write!(f, "TRCSEQRSTEVR"),
            SystemRegType::TRCCNTCTLR2 => write!(f, "TRCCNTCTLR2"),
            SystemRegType::TRCIMSPEC6 => write!(f, "TRCIMSPEC6"),
            SystemRegType::TRCSEQSTR => write!(f, "TRCSEQSTR"),
            SystemRegType::TRCCNTCTLR3 => write!(f, "TRCCNTCTLR3"),
            SystemRegType::TRCIMSPEC7 => write!(f, "TRCIMSPEC7"),
            SystemRegType::TRCEVENTCTL0R => write!(f, "TRCEVENTCTL0R"),
            SystemRegType::TRCVDCTLR => write!(f, "TRCVDCTLR"),
            SystemRegType::TRCEXTINSELR => write!(f, "TRCEXTINSELR"),
            SystemRegType::TRCCNTVR0 => write!(f, "TRCCNTVR0"),
            SystemRegType::TRCEVENTCTL1R => write!(f, "TRCEVENTCTL1R"),
            SystemRegType::TRCVDSACCTLR => write!(f, "TRCVDSACCTLR"),
            SystemRegType::TRCEXTINSELR1 => write!(f, "TRCEXTINSELR1"),
            SystemRegType::TRCCNTVR1 => write!(f, "TRCCNTVR1"),
            SystemRegType::TRCRSR => write!(f, "TRCRSR"),
            SystemRegType::TRCVDARCCTLR => write!(f, "TRCVDARCCTLR"),
            SystemRegType::TRCEXTINSELR2 => write!(f, "TRCEXTINSELR2"),
            SystemRegType::TRCCNTVR2 => write!(f, "TRCCNTVR2"),
            SystemRegType::TRCSTALLCTLR => write!(f, "TRCSTALLCTLR"),
            SystemRegType::TRCEXTINSELR3 => write!(f, "TRCEXTINSELR3"),
            SystemRegType::TRCCNTVR3 => write!(f, "TRCCNTVR3"),
            SystemRegType::TRCTSCTLR => write!(f, "TRCTSCTLR"),
            SystemRegType::TRCSYNCPR => write!(f, "TRCSYNCPR"),
            SystemRegType::TRCCCCTLR => write!(f, "TRCCCCTLR"),
            SystemRegType::TRCBBCTLR => write!(f, "TRCBBCTLR"),
            SystemRegType::TRCRSCTLR16 => write!(f, "TRCRSCTLR16"),
            SystemRegType::TRCSSCCR0 => write!(f, "TRCSSCCR0"),
            SystemRegType::TRCSSPCICR0 => write!(f, "TRCSSPCICR0"),
            SystemRegType::TRCOSLAR => write!(f, "TRCOSLAR"),
            SystemRegType::TRCRSCTLR17 => write!(f, "TRCRSCTLR17"),
            SystemRegType::TRCSSCCR1 => write!(f, "TRCSSCCR1"),
            SystemRegType::TRCSSPCICR1 => write!(f, "TRCSSPCICR1"),
            SystemRegType::TRCRSCTLR2 => write!(f, "TRCRSCTLR2"),
            SystemRegType::TRCRSCTLR18 => write!(f, "TRCRSCTLR18"),
            SystemRegType::TRCSSCCR2 => write!(f, "TRCSSCCR2"),
            SystemRegType::TRCSSPCICR2 => write!(f, "TRCSSPCICR2"),
            SystemRegType::TRCRSCTLR3 => write!(f, "TRCRSCTLR3"),
            SystemRegType::TRCRSCTLR19 => write!(f, "TRCRSCTLR19"),
            SystemRegType::TRCSSCCR3 => write!(f, "TRCSSCCR3"),
            SystemRegType::TRCSSPCICR3 => write!(f, "TRCSSPCICR3"),
            SystemRegType::TRCRSCTLR4 => write!(f, "TRCRSCTLR4"),
            SystemRegType::TRCRSCTLR20 => write!(f, "TRCRSCTLR20"),
            SystemRegType::TRCSSCCR4 => write!(f, "TRCSSCCR4"),
            SystemRegType::TRCSSPCICR4 => write!(f, "TRCSSPCICR4"),
            SystemRegType::TRCPDCR => write!(f, "TRCPDCR"),
            SystemRegType::TRCRSCTLR5 => write!(f, "TRCRSCTLR5"),
            SystemRegType::TRCRSCTLR21 => write!(f, "TRCRSCTLR21"),
            SystemRegType::TRCSSCCR5 => write!(f, "TRCSSCCR5"),
            SystemRegType::TRCSSPCICR5 => write!(f, "TRCSSPCICR5"),
            SystemRegType::TRCRSCTLR6 => write!(f, "TRCRSCTLR6"),
            SystemRegType::TRCRSCTLR22 => write!(f, "TRCRSCTLR22"),
            SystemRegType::TRCSSCCR6 => write!(f, "TRCSSCCR6"),
            SystemRegType::TRCSSPCICR6 => write!(f, "TRCSSPCICR6"),
            SystemRegType::TRCRSCTLR7 => write!(f, "TRCRSCTLR7"),
            SystemRegType::TRCRSCTLR23 => write!(f, "TRCRSCTLR23"),
            SystemRegType::TRCSSCCR7 => write!(f, "TRCSSCCR7"),
            SystemRegType::TRCSSPCICR7 => write!(f, "TRCSSPCICR7"),
            SystemRegType::TRCRSCTLR8 => write!(f, "TRCRSCTLR8"),
            SystemRegType::TRCRSCTLR24 => write!(f, "TRCRSCTLR24"),
            SystemRegType::TRCSSCSR0 => write!(f, "TRCSSCSR0"),
            SystemRegType::TRCRSCTLR9 => write!(f, "TRCRSCTLR9"),
            SystemRegType::TRCRSCTLR25 => write!(f, "TRCRSCTLR25"),
            SystemRegType::TRCSSCSR1 => write!(f, "TRCSSCSR1"),
            SystemRegType::TRCRSCTLR10 => write!(f, "TRCRSCTLR10"),
            SystemRegType::TRCRSCTLR26 => write!(f, "TRCRSCTLR26"),
            SystemRegType::TRCSSCSR2 => write!(f, "TRCSSCSR2"),
            SystemRegType::TRCRSCTLR11 => write!(f, "TRCRSCTLR11"),
            SystemRegType::TRCRSCTLR27 => write!(f, "TRCRSCTLR27"),
            SystemRegType::TRCSSCSR3 => write!(f, "TRCSSCSR3"),
            SystemRegType::TRCRSCTLR12 => write!(f, "TRCRSCTLR12"),
            SystemRegType::TRCRSCTLR28 => write!(f, "TRCRSCTLR28"),
            SystemRegType::TRCSSCSR4 => write!(f, "TRCSSCSR4"),
            SystemRegType::TRCRSCTLR13 => write!(f, "TRCRSCTLR13"),
            SystemRegType::TRCRSCTLR29 => write!(f, "TRCRSCTLR29"),
            SystemRegType::TRCSSCSR5 => write!(f, "TRCSSCSR5"),
            SystemRegType::TRCRSCTLR14 => write!(f, "TRCRSCTLR14"),
            SystemRegType::TRCRSCTLR30 => write!(f, "TRCRSCTLR30"),
            SystemRegType::TRCSSCSR6 => write!(f, "TRCSSCSR6"),
            SystemRegType::TRCRSCTLR15 => write!(f, "TRCRSCTLR15"),
            SystemRegType::TRCRSCTLR31 => write!(f, "TRCRSCTLR31"),
            SystemRegType::TRCSSCSR7 => write!(f, "TRCSSCSR7"),
            SystemRegType::TRCACVR0 => write!(f, "TRCACVR0"),
            SystemRegType::TRCACVR8 => write!(f, "TRCACVR8"),
            SystemRegType::TRCACATR0 => write!(f, "TRCACATR0"),
            SystemRegType::TRCACATR8 => write!(f, "TRCACATR8"),
            SystemRegType::TRCDVCVR0 => write!(f, "TRCDVCVR0"),
            SystemRegType::TRCDVCVR4 => write!(f, "TRCDVCVR4"),
            SystemRegType::TRCDVCMR0 => write!(f, "TRCDVCMR0"),
            SystemRegType::TRCDVCMR4 => write!(f, "TRCDVCMR4"),
            SystemRegType::TRCACVR1 => write!(f, "TRCACVR1"),
            SystemRegType::TRCACVR9 => write!(f, "TRCACVR9"),
            SystemRegType::TRCACATR1 => write!(f, "TRCACATR1"),
            SystemRegType::TRCACATR9 => write!(f, "TRCACATR9"),
            SystemRegType::TRCACVR2 => write!(f, "TRCACVR2"),
            SystemRegType::TRCACVR10 => write!(f, "TRCACVR10"),
            SystemRegType::TRCACATR2 => write!(f, "TRCACATR2"),
            SystemRegType::TRCACATR10 => write!(f, "TRCACATR10"),
            SystemRegType::TRCDVCVR1 => write!(f, "TRCDVCVR1"),
            SystemRegType::TRCDVCVR5 => write!(f, "TRCDVCVR5"),
            SystemRegType::TRCDVCMR1 => write!(f, "TRCDVCMR1"),
            SystemRegType::TRCDVCMR5 => write!(f, "TRCDVCMR5"),
            SystemRegType::TRCACVR3 => write!(f, "TRCACVR3"),
            SystemRegType::TRCACVR11 => write!(f, "TRCACVR11"),
            SystemRegType::TRCACATR3 => write!(f, "TRCACATR3"),
            SystemRegType::TRCACATR11 => write!(f, "TRCACATR11"),
            SystemRegType::TRCACVR4 => write!(f, "TRCACVR4"),
            SystemRegType::TRCACVR12 => write!(f, "TRCACVR12"),
            SystemRegType::TRCACATR4 => write!(f, "TRCACATR4"),
            SystemRegType::TRCACATR12 => write!(f, "TRCACATR12"),
            SystemRegType::TRCDVCVR2 => write!(f, "TRCDVCVR2"),
            SystemRegType::TRCDVCVR6 => write!(f, "TRCDVCVR6"),
            SystemRegType::TRCDVCMR2 => write!(f, "TRCDVCMR2"),
            SystemRegType::TRCDVCMR6 => write!(f, "TRCDVCMR6"),
            SystemRegType::TRCACVR5 => write!(f, "TRCACVR5"),
            SystemRegType::TRCACVR13 => write!(f, "TRCACVR13"),
            SystemRegType::TRCACATR5 => write!(f, "TRCACATR5"),
            SystemRegType::TRCACATR13 => write!(f, "TRCACATR13"),
            SystemRegType::TRCACVR6 => write!(f, "TRCACVR6"),
            SystemRegType::TRCACVR14 => write!(f, "TRCACVR14"),
            SystemRegType::TRCACATR6 => write!(f, "TRCACATR6"),
            SystemRegType::TRCACATR14 => write!(f, "TRCACATR14"),
            SystemRegType::TRCDVCVR3 => write!(f, "TRCDVCVR3"),
            SystemRegType::TRCDVCVR7 => write!(f, "TRCDVCVR7"),
            SystemRegType::TRCDVCMR3 => write!(f, "TRCDVCMR3"),
            SystemRegType::TRCDVCMR7 => write!(f, "TRCDVCMR7"),
            SystemRegType::TRCACVR7 => write!(f, "TRCACVR7"),
            SystemRegType::TRCACVR15 => write!(f, "TRCACVR15"),
            SystemRegType::TRCACATR7 => write!(f, "TRCACATR7"),
            SystemRegType::TRCACATR15 => write!(f, "TRCACATR15"),
            SystemRegType::TRCCIDCVR0 => write!(f, "TRCCIDCVR0"),
            SystemRegType::TRCVMIDCVR0 => write!(f, "TRCVMIDCVR0"),
            SystemRegType::TRCCIDCCTLR0 => write!(f, "TRCCIDCCTLR0"),
            SystemRegType::TRCCIDCCTLR1 => write!(f, "TRCCIDCCTLR1"),
            SystemRegType::TRCCIDCVR1 => write!(f, "TRCCIDCVR1"),
            SystemRegType::TRCVMIDCVR1 => write!(f, "TRCVMIDCVR1"),
            SystemRegType::TRCVMIDCCTLR0 => write!(f, "TRCVMIDCCTLR0"),
            SystemRegType::TRCVMIDCCTLR1 => write!(f, "TRCVMIDCCTLR1"),
            SystemRegType::TRCCIDCVR2 => write!(f, "TRCCIDCVR2"),
            SystemRegType::TRCVMIDCVR2 => write!(f, "TRCVMIDCVR2"),
            SystemRegType::TRCCIDCVR3 => write!(f, "TRCCIDCVR3"),
            SystemRegType::TRCVMIDCVR3 => write!(f, "TRCVMIDCVR3"),
            SystemRegType::TRCCIDCVR4 => write!(f, "TRCCIDCVR4"),
            SystemRegType::TRCVMIDCVR4 => write!(f, "TRCVMIDCVR4"),
            SystemRegType::TRCCIDCVR5 => write!(f, "TRCCIDCVR5"),
            SystemRegType::TRCVMIDCVR5 => write!(f, "TRCVMIDCVR5"),
            SystemRegType::TRCCIDCVR6 => write!(f, "TRCCIDCVR6"),
            SystemRegType::TRCVMIDCVR6 => write!(f, "TRCVMIDCVR6"),
            SystemRegType::TRCCIDCVR7 => write!(f, "TRCCIDCVR7"),
            SystemRegType::TRCVMIDCVR7 => write!(f, "TRCVMIDCVR7"),
            SystemRegType::TRCITCTRL => write!(f, "TRCITCTRL"),
            SystemRegType::TRCCLAIMSET => write!(f, "TRCCLAIMSET"),
            SystemRegType::TRCCLAIMCLR => write!(f, "TRCCLAIMCLR"),
            SystemRegType::TRCLAR => write!(f, "TRCLAR"),
            SystemRegType::TEECR32_EL1 => write!(f, "TEECR32_EL1"),
            SystemRegType::TEEHBR32_EL1 => write!(f, "TEEHBR32_EL1"),
            SystemRegType::DBGDTR_EL0 => write!(f, "DBGDTR_EL0"),
            SystemRegType::DBGDTRTX_EL0 => write!(f, "DBGDTRTX_EL0"),
            SystemRegType::DBGVCR32_EL2 => write!(f, "DBGVCR32_EL2"),
            SystemRegType::SCTLR_EL1 => write!(f, "SCTLR_EL1"),
            SystemRegType::ACTLR_EL1 => write!(f, "ACTLR_EL1"),
            SystemRegType::CPACR_EL1 => write!(f, "CPACR_EL1"),
            SystemRegType::RGSR_EL1 => write!(f, "RGSR_EL1"),
            SystemRegType::GCR_EL1 => write!(f, "GCR_EL1"),
            SystemRegType::TRFCR_EL1 => write!(f, "TRFCR_EL1"),
            SystemRegType::TTBR0_EL1 => write!(f, "TTBR0_EL1"),
            SystemRegType::TTBR1_EL1 => write!(f, "TTBR1_EL1"),
            SystemRegType::TCR_EL1 => write!(f, "TCR_EL1"),
            SystemRegType::APIAKEYLO_EL1 => write!(f, "APIAKEYLO_EL1"),
            SystemRegType::APIAKEYHI_EL1 => write!(f, "APIAKEYHI_EL1"),
            SystemRegType::APIBKEYLO_EL1 => write!(f, "APIBKEYLO_EL1"),
            SystemRegType::APIBKEYHI_EL1 => write!(f, "APIBKEYHI_EL1"),
            SystemRegType::APDAKEYLO_EL1 => write!(f, "APDAKEYLO_EL1"),
            SystemRegType::APDAKEYHI_EL1 => write!(f, "APDAKEYHI_EL1"),
            SystemRegType::APDBKEYLO_EL1 => write!(f, "APDBKEYLO_EL1"),
            SystemRegType::APDBKEYHI_EL1 => write!(f, "APDBKEYHI_EL1"),
            SystemRegType::APGAKEYLO_EL1 => write!(f, "APGAKEYLO_EL1"),
            SystemRegType::APGAKEYHI_EL1 => write!(f, "APGAKEYHI_EL1"),
            SystemRegType::SPSR_EL1 => write!(f, "SPSR_EL1"),
            SystemRegType::ELR_EL1 => write!(f, "ELR_EL1"),
            SystemRegType::SP_EL0 => write!(f, "SP_EL0"),
            SystemRegType::SPSEL => write!(f, "SPSEL"),
            SystemRegType::CURRENTEL => write!(f, "CURRENTEL"),
            SystemRegType::PAN => write!(f, "PAN"),
            SystemRegType::UAO => write!(f, "UAO"),
            SystemRegType::ICC_PMR_EL1 => write!(f, "ICC_PMR_EL1"),
            SystemRegType::AFSR0_EL1 => write!(f, "AFSR0_EL1"),
            SystemRegType::AFSR1_EL1 => write!(f, "AFSR1_EL1"),
            SystemRegType::ESR_EL1 => write!(f, "ESR_EL1"),
            SystemRegType::ERRSELR_EL1 => write!(f, "ERRSELR_EL1"),
            SystemRegType::ERXCTLR_EL1 => write!(f, "ERXCTLR_EL1"),
            SystemRegType::ERXSTATUS_EL1 => write!(f, "ERXSTATUS_EL1"),
            SystemRegType::ERXADDR_EL1 => write!(f, "ERXADDR_EL1"),
            SystemRegType::ERXPFGCTL_EL1 => write!(f, "ERXPFGCTL_EL1"),
            SystemRegType::ERXPFGCDN_EL1 => write!(f, "ERXPFGCDN_EL1"),
            SystemRegType::ERXMISC0_EL1 => write!(f, "ERXMISC0_EL1"),
            SystemRegType::ERXMISC1_EL1 => write!(f, "ERXMISC1_EL1"),
            SystemRegType::ERXMISC2_EL1 => write!(f, "ERXMISC2_EL1"),
            SystemRegType::ERXMISC3_EL1 => write!(f, "ERXMISC3_EL1"),
            SystemRegType::ERXTS_EL1 => write!(f, "ERXTS_EL1"),
            SystemRegType::TFSR_EL1 => write!(f, "TFSR_EL1"),
            SystemRegType::TFSRE0_EL1 => write!(f, "TFSRE0_EL1"),
            SystemRegType::FAR_EL1 => write!(f, "FAR_EL1"),
            SystemRegType::PAR_EL1 => write!(f, "PAR_EL1"),
            SystemRegType::PMSCR_EL1 => write!(f, "PMSCR_EL1"),
            SystemRegType::PMSICR_EL1 => write!(f, "PMSICR_EL1"),
            SystemRegType::PMSIRR_EL1 => write!(f, "PMSIRR_EL1"),
            SystemRegType::PMSFCR_EL1 => write!(f, "PMSFCR_EL1"),
            SystemRegType::PMSEVFR_EL1 => write!(f, "PMSEVFR_EL1"),
            SystemRegType::PMSLATFR_EL1 => write!(f, "PMSLATFR_EL1"),
            SystemRegType::PMSIDR_EL1 => write!(f, "PMSIDR_EL1"),
            SystemRegType::PMBLIMITR_EL1 => write!(f, "PMBLIMITR_EL1"),
            SystemRegType::PMBPTR_EL1 => write!(f, "PMBPTR_EL1"),
            SystemRegType::PMBSR_EL1 => write!(f, "PMBSR_EL1"),
            SystemRegType::PMBIDR_EL1 => write!(f, "PMBIDR_EL1"),
            SystemRegType::TRBLIMITR_EL1 => write!(f, "TRBLIMITR_EL1"),
            SystemRegType::TRBPTR_EL1 => write!(f, "TRBPTR_EL1"),
            SystemRegType::TRBBASER_EL1 => write!(f, "TRBBASER_EL1"),
            SystemRegType::TRBSR_EL1 => write!(f, "TRBSR_EL1"),
            SystemRegType::TRBMAR_EL1 => write!(f, "TRBMAR_EL1"),
            SystemRegType::TRBTRG_EL1 => write!(f, "TRBTRG_EL1"),
            SystemRegType::PMINTENSET_EL1 => write!(f, "PMINTENSET_EL1"),
            SystemRegType::PMINTENCLR_EL1 => write!(f, "PMINTENCLR_EL1"),
            SystemRegType::PMMIR_EL1 => write!(f, "PMMIR_EL1"),
            SystemRegType::MAIR_EL1 => write!(f, "MAIR_EL1"),
            SystemRegType::AMAIR_EL1 => write!(f, "AMAIR_EL1"),
            SystemRegType::LORSA_EL1 => write!(f, "LORSA_EL1"),
            SystemRegType::LOREA_EL1 => write!(f, "LOREA_EL1"),
            SystemRegType::LORN_EL1 => write!(f, "LORN_EL1"),
            SystemRegType::LORC_EL1 => write!(f, "LORC_EL1"),
            SystemRegType::MPAM1_EL1 => write!(f, "MPAM1_EL1"),
            SystemRegType::MPAM0_EL1 => write!(f, "MPAM0_EL1"),
            SystemRegType::VBAR_EL1 => write!(f, "VBAR_EL1"),
            SystemRegType::RMR_EL1 => write!(f, "RMR_EL1"),
            SystemRegType::DISR_EL1 => write!(f, "DISR_EL1"),
            SystemRegType::ICC_EOIR0_EL1 => write!(f, "ICC_EOIR0_EL1"),
            SystemRegType::ICC_BPR0_EL1 => write!(f, "ICC_BPR0_EL1"),
            SystemRegType::ICC_AP0R0_EL1 => write!(f, "ICC_AP0R0_EL1"),
            SystemRegType::ICC_AP0R1_EL1 => write!(f, "ICC_AP0R1_EL1"),
            SystemRegType::ICC_AP0R2_EL1 => write!(f, "ICC_AP0R2_EL1"),
            SystemRegType::ICC_AP0R3_EL1 => write!(f, "ICC_AP0R3_EL1"),
            SystemRegType::ICC_AP1R0_EL1 => write!(f, "ICC_AP1R0_EL1"),
            SystemRegType::ICC_AP1R1_EL1 => write!(f, "ICC_AP1R1_EL1"),
            SystemRegType::ICC_AP1R2_EL1 => write!(f, "ICC_AP1R2_EL1"),
            SystemRegType::ICC_AP1R3_EL1 => write!(f, "ICC_AP1R3_EL1"),
            SystemRegType::ICC_DIR_EL1 => write!(f, "ICC_DIR_EL1"),
            SystemRegType::ICC_SGI1R_EL1 => write!(f, "ICC_SGI1R_EL1"),
            SystemRegType::ICC_ASGI1R_EL1 => write!(f, "ICC_ASGI1R_EL1"),
            SystemRegType::ICC_SGI0R_EL1 => write!(f, "ICC_SGI0R_EL1"),
            SystemRegType::ICC_EOIR1_EL1 => write!(f, "ICC_EOIR1_EL1"),
            SystemRegType::ICC_BPR1_EL1 => write!(f, "ICC_BPR1_EL1"),
            SystemRegType::ICC_CTLR_EL1 => write!(f, "ICC_CTLR_EL1"),
            SystemRegType::ICC_SRE_EL1 => write!(f, "ICC_SRE_EL1"),
            SystemRegType::ICC_IGRPEN0_EL1 => write!(f, "ICC_IGRPEN0_EL1"),
            SystemRegType::ICC_IGRPEN1_EL1 => write!(f, "ICC_IGRPEN1_EL1"),
            SystemRegType::ICC_SEIEN_EL1 => write!(f, "ICC_SEIEN_EL1"),
            SystemRegType::CONTEXTIDR_EL1 => write!(f, "CONTEXTIDR_EL1"),
            SystemRegType::TPIDR_EL1 => write!(f, "TPIDR_EL1"),
            SystemRegType::SCXTNUM_EL1 => write!(f, "SCXTNUM_EL1"),
            SystemRegType::CNTKCTL_EL1 => write!(f, "CNTKCTL_EL1"),
            SystemRegType::CSSELR_EL1 => write!(f, "CSSELR_EL1"),
            SystemRegType::NZCV => write!(f, "NZCV"),
            SystemRegType::DAIFSET => write!(f, "DAIFSET"),
            SystemRegType::DIT => write!(f, "DIT"),
            SystemRegType::SSBS => write!(f, "SSBS"),
            SystemRegType::TCO => write!(f, "TCO"),
            SystemRegType::FPCR => write!(f, "FPCR"),
            SystemRegType::FPSR => write!(f, "FPSR"),
            SystemRegType::DSPSR_EL0 => write!(f, "DSPSR_EL0"),
            SystemRegType::DLR_EL0 => write!(f, "DLR_EL0"),
            SystemRegType::PMCR_EL0 => write!(f, "PMCR_EL0"),
            SystemRegType::PMCNTENSET_EL0 => write!(f, "PMCNTENSET_EL0"),
            SystemRegType::PMCNTENCLR_EL0 => write!(f, "PMCNTENCLR_EL0"),
            SystemRegType::PMOVSCLR_EL0 => write!(f, "PMOVSCLR_EL0"),
            SystemRegType::PMSWINC_EL0 => write!(f, "PMSWINC_EL0"),
            SystemRegType::PMSELR_EL0 => write!(f, "PMSELR_EL0"),
            SystemRegType::PMCCNTR_EL0 => write!(f, "PMCCNTR_EL0"),
            SystemRegType::PMXEVTYPER_EL0 => write!(f, "PMXEVTYPER_EL0"),
            SystemRegType::PMXEVCNTR_EL0 => write!(f, "PMXEVCNTR_EL0"),
            SystemRegType::DAIFCLR => write!(f, "DAIFCLR"),
            SystemRegType::PMUSERENR_EL0 => write!(f, "PMUSERENR_EL0"),
            SystemRegType::PMOVSSET_EL0 => write!(f, "PMOVSSET_EL0"),
            SystemRegType::TPIDR_EL0 => write!(f, "TPIDR_EL0"),
            SystemRegType::TPIDRRO_EL0 => write!(f, "TPIDRRO_EL0"),
            SystemRegType::SCXTNUM_EL0 => write!(f, "SCXTNUM_EL0"),
            SystemRegType::AMCR_EL0 => write!(f, "AMCR_EL0"),
            SystemRegType::AMUSERENR_EL0 => write!(f, "AMUSERENR_EL0"),
            SystemRegType::AMCNTENCLR0_EL0 => write!(f, "AMCNTENCLR0_EL0"),
            SystemRegType::AMCNTENSET0_EL0 => write!(f, "AMCNTENSET0_EL0"),
            SystemRegType::AMCNTENCLR1_EL0 => write!(f, "AMCNTENCLR1_EL0"),
            SystemRegType::AMCNTENSET1_EL0 => write!(f, "AMCNTENSET1_EL0"),
            SystemRegType::AMEVCNTR00_EL0 => write!(f, "AMEVCNTR00_EL0"),
            SystemRegType::AMEVCNTR01_EL0 => write!(f, "AMEVCNTR01_EL0"),
            SystemRegType::AMEVCNTR02_EL0 => write!(f, "AMEVCNTR02_EL0"),
            SystemRegType::AMEVCNTR03_EL0 => write!(f, "AMEVCNTR03_EL0"),
            SystemRegType::AMEVCNTR10_EL0 => write!(f, "AMEVCNTR10_EL0"),
            SystemRegType::AMEVCNTR11_EL0 => write!(f, "AMEVCNTR11_EL0"),
            SystemRegType::AMEVCNTR12_EL0 => write!(f, "AMEVCNTR12_EL0"),
            SystemRegType::AMEVCNTR13_EL0 => write!(f, "AMEVCNTR13_EL0"),
            SystemRegType::AMEVCNTR14_EL0 => write!(f, "AMEVCNTR14_EL0"),
            SystemRegType::AMEVCNTR15_EL0 => write!(f, "AMEVCNTR15_EL0"),
            SystemRegType::AMEVCNTR16_EL0 => write!(f, "AMEVCNTR16_EL0"),
            SystemRegType::AMEVCNTR17_EL0 => write!(f, "AMEVCNTR17_EL0"),
            SystemRegType::AMEVCNTR18_EL0 => write!(f, "AMEVCNTR18_EL0"),
            SystemRegType::AMEVCNTR19_EL0 => write!(f, "AMEVCNTR19_EL0"),
            SystemRegType::AMEVCNTR110_EL0 => write!(f, "AMEVCNTR110_EL0"),
            SystemRegType::AMEVCNTR111_EL0 => write!(f, "AMEVCNTR111_EL0"),
            SystemRegType::AMEVCNTR112_EL0 => write!(f, "AMEVCNTR112_EL0"),
            SystemRegType::AMEVCNTR113_EL0 => write!(f, "AMEVCNTR113_EL0"),
            SystemRegType::AMEVCNTR114_EL0 => write!(f, "AMEVCNTR114_EL0"),
            SystemRegType::AMEVCNTR115_EL0 => write!(f, "AMEVCNTR115_EL0"),
            SystemRegType::AMEVTYPER10_EL0 => write!(f, "AMEVTYPER10_EL0"),
            SystemRegType::AMEVTYPER11_EL0 => write!(f, "AMEVTYPER11_EL0"),
            SystemRegType::AMEVTYPER12_EL0 => write!(f, "AMEVTYPER12_EL0"),
            SystemRegType::AMEVTYPER13_EL0 => write!(f, "AMEVTYPER13_EL0"),
            SystemRegType::AMEVTYPER14_EL0 => write!(f, "AMEVTYPER14_EL0"),
            SystemRegType::AMEVTYPER15_EL0 => write!(f, "AMEVTYPER15_EL0"),
            SystemRegType::AMEVTYPER16_EL0 => write!(f, "AMEVTYPER16_EL0"),
            SystemRegType::AMEVTYPER17_EL0 => write!(f, "AMEVTYPER17_EL0"),
            SystemRegType::AMEVTYPER18_EL0 => write!(f, "AMEVTYPER18_EL0"),
            SystemRegType::AMEVTYPER19_EL0 => write!(f, "AMEVTYPER19_EL0"),
            SystemRegType::AMEVTYPER110_EL0 => write!(f, "AMEVTYPER110_EL0"),
            SystemRegType::AMEVTYPER111_EL0 => write!(f, "AMEVTYPER111_EL0"),
            SystemRegType::AMEVTYPER112_EL0 => write!(f, "AMEVTYPER112_EL0"),
            SystemRegType::AMEVTYPER113_EL0 => write!(f, "AMEVTYPER113_EL0"),
            SystemRegType::AMEVTYPER114_EL0 => write!(f, "AMEVTYPER114_EL0"),
            SystemRegType::AMEVTYPER115_EL0 => write!(f, "AMEVTYPER115_EL0"),
            SystemRegType::CNTFRQ_EL0 => write!(f, "CNTFRQ_EL0"),
            SystemRegType::CNTP_TVAL_EL0 => write!(f, "CNTP_TVAL_EL0"),
            SystemRegType::CNTP_CTL_EL0 => write!(f, "CNTP_CTL_EL0"),
            SystemRegType::CNTP_CVAL_EL0 => write!(f, "CNTP_CVAL_EL0"),
            SystemRegType::CNTV_TVAL_EL0 => write!(f, "CNTV_TVAL_EL0"),
            SystemRegType::CNTV_CTL_EL0 => write!(f, "CNTV_CTL_EL0"),
            SystemRegType::CNTV_CVAL_EL0 => write!(f, "CNTV_CVAL_EL0"),
            SystemRegType::PMEVCNTR0_EL0 => write!(f, "PMEVCNTR0_EL0"),
            SystemRegType::PMEVCNTR1_EL0 => write!(f, "PMEVCNTR1_EL0"),
            SystemRegType::PMEVCNTR2_EL0 => write!(f, "PMEVCNTR2_EL0"),
            SystemRegType::PMEVCNTR3_EL0 => write!(f, "PMEVCNTR3_EL0"),
            SystemRegType::PMEVCNTR4_EL0 => write!(f, "PMEVCNTR4_EL0"),
            SystemRegType::PMEVCNTR5_EL0 => write!(f, "PMEVCNTR5_EL0"),
            SystemRegType::PMEVCNTR6_EL0 => write!(f, "PMEVCNTR6_EL0"),
            SystemRegType::PMEVCNTR7_EL0 => write!(f, "PMEVCNTR7_EL0"),
            SystemRegType::PMEVCNTR8_EL0 => write!(f, "PMEVCNTR8_EL0"),
            SystemRegType::PMEVCNTR9_EL0 => write!(f, "PMEVCNTR9_EL0"),
            SystemRegType::PMEVCNTR10_EL0 => write!(f, "PMEVCNTR10_EL0"),
            SystemRegType::PMEVCNTR11_EL0 => write!(f, "PMEVCNTR11_EL0"),
            SystemRegType::PMEVCNTR12_EL0 => write!(f, "PMEVCNTR12_EL0"),
            SystemRegType::PMEVCNTR13_EL0 => write!(f, "PMEVCNTR13_EL0"),
            SystemRegType::PMEVCNTR14_EL0 => write!(f, "PMEVCNTR14_EL0"),
            SystemRegType::PMEVCNTR15_EL0 => write!(f, "PMEVCNTR15_EL0"),
            SystemRegType::PMEVCNTR16_EL0 => write!(f, "PMEVCNTR16_EL0"),
            SystemRegType::PMEVCNTR17_EL0 => write!(f, "PMEVCNTR17_EL0"),
            SystemRegType::PMEVCNTR18_EL0 => write!(f, "PMEVCNTR18_EL0"),
            SystemRegType::PMEVCNTR19_EL0 => write!(f, "PMEVCNTR19_EL0"),
            SystemRegType::PMEVCNTR20_EL0 => write!(f, "PMEVCNTR20_EL0"),
            SystemRegType::PMEVCNTR21_EL0 => write!(f, "PMEVCNTR21_EL0"),
            SystemRegType::PMEVCNTR22_EL0 => write!(f, "PMEVCNTR22_EL0"),
            SystemRegType::PMEVCNTR23_EL0 => write!(f, "PMEVCNTR23_EL0"),
            SystemRegType::PMEVCNTR24_EL0 => write!(f, "PMEVCNTR24_EL0"),
            SystemRegType::PMEVCNTR25_EL0 => write!(f, "PMEVCNTR25_EL0"),
            SystemRegType::PMEVCNTR26_EL0 => write!(f, "PMEVCNTR26_EL0"),
            SystemRegType::PMEVCNTR27_EL0 => write!(f, "PMEVCNTR27_EL0"),
            SystemRegType::PMEVCNTR28_EL0 => write!(f, "PMEVCNTR28_EL0"),
            SystemRegType::PMEVCNTR29_EL0 => write!(f, "PMEVCNTR29_EL0"),
            SystemRegType::PMEVCNTR30_EL0 => write!(f, "PMEVCNTR30_EL0"),
            SystemRegType::PMEVTYPER0_EL0 => write!(f, "PMEVTYPER0_EL0"),
            SystemRegType::PMEVTYPER1_EL0 => write!(f, "PMEVTYPER1_EL0"),
            SystemRegType::PMEVTYPER2_EL0 => write!(f, "PMEVTYPER2_EL0"),
            SystemRegType::PMEVTYPER3_EL0 => write!(f, "PMEVTYPER3_EL0"),
            SystemRegType::PMEVTYPER4_EL0 => write!(f, "PMEVTYPER4_EL0"),
            SystemRegType::PMEVTYPER5_EL0 => write!(f, "PMEVTYPER5_EL0"),
            SystemRegType::PMEVTYPER6_EL0 => write!(f, "PMEVTYPER6_EL0"),
            SystemRegType::PMEVTYPER7_EL0 => write!(f, "PMEVTYPER7_EL0"),
            SystemRegType::PMEVTYPER8_EL0 => write!(f, "PMEVTYPER8_EL0"),
            SystemRegType::PMEVTYPER9_EL0 => write!(f, "PMEVTYPER9_EL0"),
            SystemRegType::PMEVTYPER10_EL0 => write!(f, "PMEVTYPER10_EL0"),
            SystemRegType::PMEVTYPER11_EL0 => write!(f, "PMEVTYPER11_EL0"),
            SystemRegType::PMEVTYPER12_EL0 => write!(f, "PMEVTYPER12_EL0"),
            SystemRegType::PMEVTYPER13_EL0 => write!(f, "PMEVTYPER13_EL0"),
            SystemRegType::PMEVTYPER14_EL0 => write!(f, "PMEVTYPER14_EL0"),
            SystemRegType::PMEVTYPER15_EL0 => write!(f, "PMEVTYPER15_EL0"),
            SystemRegType::PMEVTYPER16_EL0 => write!(f, "PMEVTYPER16_EL0"),
            SystemRegType::PMEVTYPER17_EL0 => write!(f, "PMEVTYPER17_EL0"),
            SystemRegType::PMEVTYPER18_EL0 => write!(f, "PMEVTYPER18_EL0"),
            SystemRegType::PMEVTYPER19_EL0 => write!(f, "PMEVTYPER19_EL0"),
            SystemRegType::PMEVTYPER20_EL0 => write!(f, "PMEVTYPER20_EL0"),
            SystemRegType::PMEVTYPER21_EL0 => write!(f, "PMEVTYPER21_EL0"),
            SystemRegType::PMEVTYPER22_EL0 => write!(f, "PMEVTYPER22_EL0"),
            SystemRegType::PMEVTYPER23_EL0 => write!(f, "PMEVTYPER23_EL0"),
            SystemRegType::PMEVTYPER24_EL0 => write!(f, "PMEVTYPER24_EL0"),
            SystemRegType::PMEVTYPER25_EL0 => write!(f, "PMEVTYPER25_EL0"),
            SystemRegType::PMEVTYPER26_EL0 => write!(f, "PMEVTYPER26_EL0"),
            SystemRegType::PMEVTYPER27_EL0 => write!(f, "PMEVTYPER27_EL0"),
            SystemRegType::PMEVTYPER28_EL0 => write!(f, "PMEVTYPER28_EL0"),
            SystemRegType::PMEVTYPER29_EL0 => write!(f, "PMEVTYPER29_EL0"),
            SystemRegType::PMEVTYPER30_EL0 => write!(f, "PMEVTYPER30_EL0"),
            SystemRegType::PMCCFILTR_EL0 => write!(f, "PMCCFILTR_EL0"),
            SystemRegType::VPIDR_EL2 => write!(f, "VPIDR_EL2"),
            SystemRegType::VMPIDR_EL2 => write!(f, "VMPIDR_EL2"),
            SystemRegType::SCTLR_EL2 => write!(f, "SCTLR_EL2"),
            SystemRegType::ACTLR_EL2 => write!(f, "ACTLR_EL2"),
            SystemRegType::HCR_EL2 => write!(f, "HCR_EL2"),
            SystemRegType::MDCR_EL2 => write!(f, "MDCR_EL2"),
            SystemRegType::CPTR_EL2 => write!(f, "CPTR_EL2"),
            SystemRegType::HSTR_EL2 => write!(f, "HSTR_EL2"),
            SystemRegType::HACR_EL2 => write!(f, "HACR_EL2"),
            SystemRegType::TRFCR_EL2 => write!(f, "TRFCR_EL2"),
            SystemRegType::SDER32_EL2 => write!(f, "SDER32_EL2"),
            SystemRegType::TTBR0_EL2 => write!(f, "TTBR0_EL2"),
            SystemRegType::TTBR1_EL2 => write!(f, "TTBR1_EL2"),
            SystemRegType::TCR_EL2 => write!(f, "TCR_EL2"),
            SystemRegType::VTTBR_EL2 => write!(f, "VTTBR_EL2"),
            SystemRegType::VTCR_EL2 => write!(f, "VTCR_EL2"),
            SystemRegType::VNCR_EL2 => write!(f, "VNCR_EL2"),
            SystemRegType::VSTTBR_EL2 => write!(f, "VSTTBR_EL2"),
            SystemRegType::VSTCR_EL2 => write!(f, "VSTCR_EL2"),
            SystemRegType::DACR32_EL2 => write!(f, "DACR32_EL2"),
            SystemRegType::SPSR_EL2 => write!(f, "SPSR_EL2"),
            SystemRegType::ELR_EL2 => write!(f, "ELR_EL2"),
            SystemRegType::SP_EL1 => write!(f, "SP_EL1"),
            SystemRegType::SPSR_IRQ => write!(f, "SPSR_IRQ"),
            SystemRegType::SPSR_ABT => write!(f, "SPSR_ABT"),
            SystemRegType::SPSR_UND => write!(f, "SPSR_UND"),
            SystemRegType::SPSR_FIQ => write!(f, "SPSR_FIQ"),
            SystemRegType::IFSR32_EL2 => write!(f, "IFSR32_EL2"),
            SystemRegType::AFSR0_EL2 => write!(f, "AFSR0_EL2"),
            SystemRegType::AFSR1_EL2 => write!(f, "AFSR1_EL2"),
            SystemRegType::ESR_EL2 => write!(f, "ESR_EL2"),
            SystemRegType::VSESR_EL2 => write!(f, "VSESR_EL2"),
            SystemRegType::FPEXC32_EL2 => write!(f, "FPEXC32_EL2"),
            SystemRegType::TFSR_EL2 => write!(f, "TFSR_EL2"),
            SystemRegType::FAR_EL2 => write!(f, "FAR_EL2"),
            SystemRegType::HPFAR_EL2 => write!(f, "HPFAR_EL2"),
            SystemRegType::PMSCR_EL2 => write!(f, "PMSCR_EL2"),
            SystemRegType::MAIR_EL2 => write!(f, "MAIR_EL2"),
            SystemRegType::AMAIR_EL2 => write!(f, "AMAIR_EL2"),
            SystemRegType::MPAMHCR_EL2 => write!(f, "MPAMHCR_EL2"),
            SystemRegType::MPAMVPMV_EL2 => write!(f, "MPAMVPMV_EL2"),
            SystemRegType::MPAM2_EL2 => write!(f, "MPAM2_EL2"),
            SystemRegType::MPAMVPM0_EL2 => write!(f, "MPAMVPM0_EL2"),
            SystemRegType::MPAMVPM1_EL2 => write!(f, "MPAMVPM1_EL2"),
            SystemRegType::MPAMVPM2_EL2 => write!(f, "MPAMVPM2_EL2"),
            SystemRegType::MPAMVPM3_EL2 => write!(f, "MPAMVPM3_EL2"),
            SystemRegType::MPAMVPM4_EL2 => write!(f, "MPAMVPM4_EL2"),
            SystemRegType::MPAMVPM5_EL2 => write!(f, "MPAMVPM5_EL2"),
            SystemRegType::MPAMVPM6_EL2 => write!(f, "MPAMVPM6_EL2"),
            SystemRegType::MPAMVPM7_EL2 => write!(f, "MPAMVPM7_EL2"),
            SystemRegType::VBAR_EL2 => write!(f, "VBAR_EL2"),
            SystemRegType::RMR_EL2 => write!(f, "RMR_EL2"),
            SystemRegType::VDISR_EL2 => write!(f, "VDISR_EL2"),
            SystemRegType::ICH_AP0R0_EL2 => write!(f, "ICH_AP0R0_EL2"),
            SystemRegType::ICH_AP0R1_EL2 => write!(f, "ICH_AP0R1_EL2"),
            SystemRegType::ICH_AP0R2_EL2 => write!(f, "ICH_AP0R2_EL2"),
            SystemRegType::ICH_AP0R3_EL2 => write!(f, "ICH_AP0R3_EL2"),
            SystemRegType::ICH_AP1R0_EL2 => write!(f, "ICH_AP1R0_EL2"),
            SystemRegType::ICH_AP1R1_EL2 => write!(f, "ICH_AP1R1_EL2"),
            SystemRegType::ICH_AP1R2_EL2 => write!(f, "ICH_AP1R2_EL2"),
            SystemRegType::ICH_AP1R3_EL2 => write!(f, "ICH_AP1R3_EL2"),
            SystemRegType::ICH_VSEIR_EL2 => write!(f, "ICH_VSEIR_EL2"),
            SystemRegType::ICC_SRE_EL2 => write!(f, "ICC_SRE_EL2"),
            SystemRegType::ICH_HCR_EL2 => write!(f, "ICH_HCR_EL2"),
            SystemRegType::ICH_MISR_EL2 => write!(f, "ICH_MISR_EL2"),
            SystemRegType::ICH_VMCR_EL2 => write!(f, "ICH_VMCR_EL2"),
            SystemRegType::ICH_LR0_EL2 => write!(f, "ICH_LR0_EL2"),
            SystemRegType::ICH_LR1_EL2 => write!(f, "ICH_LR1_EL2"),
            SystemRegType::ICH_LR2_EL2 => write!(f, "ICH_LR2_EL2"),
            SystemRegType::ICH_LR3_EL2 => write!(f, "ICH_LR3_EL2"),
            SystemRegType::ICH_LR4_EL2 => write!(f, "ICH_LR4_EL2"),
            SystemRegType::ICH_LR5_EL2 => write!(f, "ICH_LR5_EL2"),
            SystemRegType::ICH_LR6_EL2 => write!(f, "ICH_LR6_EL2"),
            SystemRegType::ICH_LR7_EL2 => write!(f, "ICH_LR7_EL2"),
            SystemRegType::ICH_LR8_EL2 => write!(f, "ICH_LR8_EL2"),
            SystemRegType::ICH_LR9_EL2 => write!(f, "ICH_LR9_EL2"),
            SystemRegType::ICH_LR10_EL2 => write!(f, "ICH_LR10_EL2"),
            SystemRegType::ICH_LR11_EL2 => write!(f, "ICH_LR11_EL2"),
            SystemRegType::ICH_LR12_EL2 => write!(f, "ICH_LR12_EL2"),
            SystemRegType::ICH_LR13_EL2 => write!(f, "ICH_LR13_EL2"),
            SystemRegType::ICH_LR14_EL2 => write!(f, "ICH_LR14_EL2"),
            SystemRegType::ICH_LR15_EL2 => write!(f, "ICH_LR15_EL2"),
            SystemRegType::CONTEXTIDR_EL2 => write!(f, "CONTEXTIDR_EL2"),
            SystemRegType::TPIDR_EL2 => write!(f, "TPIDR_EL2"),
            SystemRegType::SCXTNUM_EL2 => write!(f, "SCXTNUM_EL2"),
            SystemRegType::CNTVOFF_EL2 => write!(f, "CNTVOFF_EL2"),
            SystemRegType::CNTHCTL_EL2 => write!(f, "CNTHCTL_EL2"),
            SystemRegType::CNTHP_TVAL_EL2 => write!(f, "CNTHP_TVAL_EL2"),
            SystemRegType::CNTHP_CTL_EL2 => write!(f, "CNTHP_CTL_EL2"),
            SystemRegType::CNTHP_CVAL_EL2 => write!(f, "CNTHP_CVAL_EL2"),
            SystemRegType::CNTHV_TVAL_EL2 => write!(f, "CNTHV_TVAL_EL2"),
            SystemRegType::CNTHV_CTL_EL2 => write!(f, "CNTHV_CTL_EL2"),
            SystemRegType::CNTHV_CVAL_EL2 => write!(f, "CNTHV_CVAL_EL2"),
            SystemRegType::CNTHVS_TVAL_EL2 => write!(f, "CNTHVS_TVAL_EL2"),
            SystemRegType::CNTHVS_CTL_EL2 => write!(f, "CNTHVS_CTL_EL2"),
            SystemRegType::CNTHVS_CVAL_EL2 => write!(f, "CNTHVS_CVAL_EL2"),
            SystemRegType::CNTHPS_TVAL_EL2 => write!(f, "CNTHPS_TVAL_EL2"),
            SystemRegType::CNTHPS_CTL_EL2 => write!(f, "CNTHPS_CTL_EL2"),
            SystemRegType::CNTHPS_CVAL_EL2 => write!(f, "CNTHPS_CVAL_EL2"),
            SystemRegType::SCTLR_EL12 => write!(f, "SCTLR_EL12"),
            SystemRegType::CPACR_EL12 => write!(f, "CPACR_EL12"),
            SystemRegType::TRFCR_EL12 => write!(f, "TRFCR_EL12"),
            SystemRegType::TTBR0_EL12 => write!(f, "TTBR0_EL12"),
            SystemRegType::TTBR1_EL12 => write!(f, "TTBR1_EL12"),
            SystemRegType::TCR_EL12 => write!(f, "TCR_EL12"),
            SystemRegType::SPSR_EL12 => write!(f, "SPSR_EL12"),
            SystemRegType::ELR_EL12 => write!(f, "ELR_EL12"),
            SystemRegType::AFSR0_EL12 => write!(f, "AFSR0_EL12"),
            SystemRegType::AFSR1_EL12 => write!(f, "AFSR1_EL12"),
            SystemRegType::ESR_EL12 => write!(f, "ESR_EL12"),
            SystemRegType::TFSR_EL12 => write!(f, "TFSR_EL12"),
            SystemRegType::FAR_EL12 => write!(f, "FAR_EL12"),
            SystemRegType::PMSCR_EL12 => write!(f, "PMSCR_EL12"),
            SystemRegType::MAIR_EL12 => write!(f, "MAIR_EL12"),
            SystemRegType::AMAIR_EL12 => write!(f, "AMAIR_EL12"),
            SystemRegType::MPAM1_EL12 => write!(f, "MPAM1_EL12"),
            SystemRegType::VBAR_EL12 => write!(f, "VBAR_EL12"),
            SystemRegType::CONTEXTIDR_EL12 => write!(f, "CONTEXTIDR_EL12"),
            SystemRegType::SCXTNUM_EL12 => write!(f, "SCXTNUM_EL12"),
            SystemRegType::CNTKCTL_EL12 => write!(f, "CNTKCTL_EL12"),
            SystemRegType::CNTP_TVAL_EL02 => write!(f, "CNTP_TVAL_EL02"),
            SystemRegType::CNTP_CTL_EL02 => write!(f, "CNTP_CTL_EL02"),
            SystemRegType::CNTP_CVAL_EL02 => write!(f, "CNTP_CVAL_EL02"),
            SystemRegType::CNTV_TVAL_EL02 => write!(f, "CNTV_TVAL_EL02"),
            SystemRegType::CNTV_CTL_EL02 => write!(f, "CNTV_CTL_EL02"),
            SystemRegType::CNTV_CVAL_EL02 => write!(f, "CNTV_CVAL_EL02"),
            SystemRegType::SCTLR_EL3 => write!(f, "SCTLR_EL3"),
            SystemRegType::ACTLR_EL3 => write!(f, "ACTLR_EL3"),
            SystemRegType::SCR_EL3 => write!(f, "SCR_EL3"),
            SystemRegType::SDER32_EL3 => write!(f, "SDER32_EL3"),
            SystemRegType::CPTR_EL3 => write!(f, "CPTR_EL3"),
            SystemRegType::MDCR_EL3 => write!(f, "MDCR_EL3"),
            SystemRegType::TTBR0_EL3 => write!(f, "TTBR0_EL3"),
            SystemRegType::TCR_EL3 => write!(f, "TCR_EL3"),
            SystemRegType::SPSR_EL3 => write!(f, "SPSR_EL3"),
            SystemRegType::ELR_EL3 => write!(f, "ELR_EL3"),
            SystemRegType::SP_EL2 => write!(f, "SP_EL2"),
            SystemRegType::AFSR0_EL3 => write!(f, "AFSR0_EL3"),
            SystemRegType::AFSR1_EL3 => write!(f, "AFSR1_EL3"),
            SystemRegType::ESR_EL3 => write!(f, "ESR_EL3"),
            SystemRegType::TFSR_EL3 => write!(f, "TFSR_EL3"),
            SystemRegType::FAR_EL3 => write!(f, "FAR_EL3"),
            SystemRegType::MAIR_EL3 => write!(f, "MAIR_EL3"),
            SystemRegType::AMAIR_EL3 => write!(f, "AMAIR_EL3"),
            SystemRegType::MPAM3_EL3 => write!(f, "MPAM3_EL3"),
            SystemRegType::VBAR_EL3 => write!(f, "VBAR_EL3"),
            SystemRegType::RMR_EL3 => write!(f, "RMR_EL3"),
            SystemRegType::ICC_CTLR_EL3 => write!(f, "ICC_CTLR_EL3"),
            SystemRegType::ICC_SRE_EL3 => write!(f, "ICC_SRE_EL3"),
            SystemRegType::ICC_IGRPEN1_EL3 => write!(f, "ICC_IGRPEN1_EL3"),
            SystemRegType::TPIDR_EL3 => write!(f, "TPIDR_EL3"),
            SystemRegType::SCXTNUM_EL3 => write!(f, "SCXTNUM_EL3"),
            SystemRegType::CNTPS_TVAL_EL1 => write!(f, "CNTPS_TVAL_EL1"),
            SystemRegType::CNTPS_CTL_EL1 => write!(f, "CNTPS_CTL_EL1"),
            SystemRegType::CNTPS_CVAL_EL1 => write!(f, "CNTPS_CVAL_EL1"),
            SystemRegType::PSTATE_SPSEL => write!(f, "PSTATE_SPSEL"),
        }
    }
}

impl From<usize> for SystemRegType {
    fn from(value: usize) -> Self {
        match value {
            0x8002 => Self::OSDTRRX_EL1,
            0x8004 => Self::DBGBVR0_EL1,
            0x8005 => Self::DBGBCR0_EL1,
            0x8006 => Self::DBGWVR0_EL1,
            0x8007 => Self::DBGWCR0_EL1,
            0x800c => Self::DBGBVR1_EL1,
            0x800d => Self::DBGBCR1_EL1,
            0x800e => Self::DBGWVR1_EL1,
            0x800f => Self::DBGWCR1_EL1,
            0x8010 => Self::MDCCINT_EL1,
            0x8012 => Self::MDSCR_EL1,
            0x8014 => Self::DBGBVR2_EL1,
            0x8015 => Self::DBGBCR2_EL1,
            0x8016 => Self::DBGWVR2_EL1,
            0x8017 => Self::DBGWCR2_EL1,
            0x801a => Self::OSDTRTX_EL1,
            0x801c => Self::DBGBVR3_EL1,
            0x801d => Self::DBGBCR3_EL1,
            0x801e => Self::DBGWVR3_EL1,
            0x801f => Self::DBGWCR3_EL1,
            0x8024 => Self::DBGBVR4_EL1,
            0x8025 => Self::DBGBCR4_EL1,
            0x8026 => Self::DBGWVR4_EL1,
            0x8027 => Self::DBGWCR4_EL1,
            0x802c => Self::DBGBVR5_EL1,
            0x802d => Self::DBGBCR5_EL1,
            0x802e => Self::DBGWVR5_EL1,
            0x802f => Self::DBGWCR5_EL1,
            0x8032 => Self::OSECCR_EL1,
            0x8034 => Self::DBGBVR6_EL1,
            0x8035 => Self::DBGBCR6_EL1,
            0x8036 => Self::DBGWVR6_EL1,
            0x8037 => Self::DBGWCR6_EL1,
            0x803c => Self::DBGBVR7_EL1,
            0x803d => Self::DBGBCR7_EL1,
            0x803e => Self::DBGWVR7_EL1,
            0x803f => Self::DBGWCR7_EL1,
            0x8044 => Self::DBGBVR8_EL1,
            0x8045 => Self::DBGBCR8_EL1,
            0x8046 => Self::DBGWVR8_EL1,
            0x8047 => Self::DBGWCR8_EL1,
            0x804c => Self::DBGBVR9_EL1,
            0x804d => Self::DBGBCR9_EL1,
            0x804e => Self::DBGWVR9_EL1,
            0x804f => Self::DBGWCR9_EL1,
            0x8054 => Self::DBGBVR10_EL1,
            0x8055 => Self::DBGBCR10_EL1,
            0x8056 => Self::DBGWVR10_EL1,
            0x8057 => Self::DBGWCR10_EL1,
            0x805c => Self::DBGBVR11_EL1,
            0x805d => Self::DBGBCR11_EL1,
            0x805e => Self::DBGWVR11_EL1,
            0x805f => Self::DBGWCR11_EL1,
            0x8064 => Self::DBGBVR12_EL1,
            0x8065 => Self::DBGBCR12_EL1,
            0x8066 => Self::DBGWVR12_EL1,
            0x8067 => Self::DBGWCR12_EL1,
            0x806c => Self::DBGBVR13_EL1,
            0x806d => Self::DBGBCR13_EL1,
            0x806e => Self::DBGWVR13_EL1,
            0x806f => Self::DBGWCR13_EL1,
            0x8074 => Self::DBGBVR14_EL1,
            0x8075 => Self::DBGBCR14_EL1,
            0x8076 => Self::DBGWVR14_EL1,
            0x8077 => Self::DBGWCR14_EL1,
            0x807c => Self::DBGBVR15_EL1,
            0x807d => Self::DBGBCR15_EL1,
            0x807e => Self::DBGWVR15_EL1,
            0x807f => Self::DBGWCR15_EL1,
            0x8084 => Self::OSLAR_EL1,
            0x809c => Self::OSDLR_EL1,
            0x80a4 => Self::DBGPRCR_EL1,
            0x83c6 => Self::DBGCLAIMSET_EL1,
            0x83ce => Self::DBGCLAIMCLR_EL1,
            0x8801 => Self::TRCTRACEIDR,
            0x8802 => Self::TRCVICTLR,
            0x8804 => Self::TRCSEQEVR0,
            0x8805 => Self::TRCCNTRLDVR0,
            0x8807 => Self::TRCIMSPEC0,
            0x8808 => Self::TRCPRGCTLR,
            0x8809 => Self::TRCQCTLR,
            0x880a => Self::TRCVIIECTLR,
            0x880c => Self::TRCSEQEVR1,
            0x880d => Self::TRCCNTRLDVR1,
            0x880f => Self::TRCIMSPEC1,
            0x8810 => Self::TRCPROCSELR,
            0x8812 => Self::TRCVISSCTLR,
            0x8814 => Self::TRCSEQEVR2,
            0x8815 => Self::TRCCNTRLDVR2,
            0x8817 => Self::TRCIMSPEC2,
            0x881a => Self::TRCVIPCSSCTLR,
            0x881d => Self::TRCCNTRLDVR3,
            0x881f => Self::TRCIMSPEC3,
            0x8820 => Self::TRCCONFIGR,
            0x8825 => Self::TRCCNTCTLR0,
            0x8827 => Self::TRCIMSPEC4,
            0x882d => Self::TRCCNTCTLR1,
            0x882f => Self::TRCIMSPEC5,
            0x8830 => Self::TRCAUXCTLR,
            0x8834 => Self::TRCSEQRSTEVR,
            0x8835 => Self::TRCCNTCTLR2,
            0x8837 => Self::TRCIMSPEC6,
            0x883c => Self::TRCSEQSTR,
            0x883d => Self::TRCCNTCTLR3,
            0x883f => Self::TRCIMSPEC7,
            0x8840 => Self::TRCEVENTCTL0R,
            0x8842 => Self::TRCVDCTLR,
            0x8844 => Self::TRCEXTINSELR,
            0x8845 => Self::TRCCNTVR0,
            0x8848 => Self::TRCEVENTCTL1R,
            0x884a => Self::TRCVDSACCTLR,
            0x884c => Self::TRCEXTINSELR1,
            0x884d => Self::TRCCNTVR1,
            0x8850 => Self::TRCRSR,
            0x8852 => Self::TRCVDARCCTLR,
            0x8854 => Self::TRCEXTINSELR2,
            0x8855 => Self::TRCCNTVR2,
            0x8858 => Self::TRCSTALLCTLR,
            0x885c => Self::TRCEXTINSELR3,
            0x885d => Self::TRCCNTVR3,
            0x8860 => Self::TRCTSCTLR,
            0x8868 => Self::TRCSYNCPR,
            0x8870 => Self::TRCCCCTLR,
            0x8878 => Self::TRCBBCTLR,
            0x8881 => Self::TRCRSCTLR16,
            0x8882 => Self::TRCSSCCR0,
            0x8883 => Self::TRCSSPCICR0,
            0x8884 => Self::TRCOSLAR,
            0x8889 => Self::TRCRSCTLR17,
            0x888a => Self::TRCSSCCR1,
            0x888b => Self::TRCSSPCICR1,
            0x8890 => Self::TRCRSCTLR2,
            0x8891 => Self::TRCRSCTLR18,
            0x8892 => Self::TRCSSCCR2,
            0x8893 => Self::TRCSSPCICR2,
            0x8898 => Self::TRCRSCTLR3,
            0x8899 => Self::TRCRSCTLR19,
            0x889a => Self::TRCSSCCR3,
            0x889b => Self::TRCSSPCICR3,
            0x88a0 => Self::TRCRSCTLR4,
            0x88a1 => Self::TRCRSCTLR20,
            0x88a2 => Self::TRCSSCCR4,
            0x88a3 => Self::TRCSSPCICR4,
            0x88a4 => Self::TRCPDCR,
            0x88a8 => Self::TRCRSCTLR5,
            0x88a9 => Self::TRCRSCTLR21,
            0x88aa => Self::TRCSSCCR5,
            0x88ab => Self::TRCSSPCICR5,
            0x88b0 => Self::TRCRSCTLR6,
            0x88b1 => Self::TRCRSCTLR22,
            0x88b2 => Self::TRCSSCCR6,
            0x88b3 => Self::TRCSSPCICR6,
            0x88b8 => Self::TRCRSCTLR7,
            0x88b9 => Self::TRCRSCTLR23,
            0x88ba => Self::TRCSSCCR7,
            0x88bb => Self::TRCSSPCICR7,
            0x88c0 => Self::TRCRSCTLR8,
            0x88c1 => Self::TRCRSCTLR24,
            0x88c2 => Self::TRCSSCSR0,
            0x88c8 => Self::TRCRSCTLR9,
            0x88c9 => Self::TRCRSCTLR25,
            0x88ca => Self::TRCSSCSR1,
            0x88d0 => Self::TRCRSCTLR10,
            0x88d1 => Self::TRCRSCTLR26,
            0x88d2 => Self::TRCSSCSR2,
            0x88d8 => Self::TRCRSCTLR11,
            0x88d9 => Self::TRCRSCTLR27,
            0x88da => Self::TRCSSCSR3,
            0x88e0 => Self::TRCRSCTLR12,
            0x88e1 => Self::TRCRSCTLR28,
            0x88e2 => Self::TRCSSCSR4,
            0x88e8 => Self::TRCRSCTLR13,
            0x88e9 => Self::TRCRSCTLR29,
            0x88ea => Self::TRCSSCSR5,
            0x88f0 => Self::TRCRSCTLR14,
            0x88f1 => Self::TRCRSCTLR30,
            0x88f2 => Self::TRCSSCSR6,
            0x88f8 => Self::TRCRSCTLR15,
            0x88f9 => Self::TRCRSCTLR31,
            0x88fa => Self::TRCSSCSR7,
            0x8900 => Self::TRCACVR0,
            0x8901 => Self::TRCACVR8,
            0x8902 => Self::TRCACATR0,
            0x8903 => Self::TRCACATR8,
            0x8904 => Self::TRCDVCVR0,
            0x8905 => Self::TRCDVCVR4,
            0x8906 => Self::TRCDVCMR0,
            0x8907 => Self::TRCDVCMR4,
            0x8910 => Self::TRCACVR1,
            0x8911 => Self::TRCACVR9,
            0x8912 => Self::TRCACATR1,
            0x8913 => Self::TRCACATR9,
            0x8920 => Self::TRCACVR2,
            0x8921 => Self::TRCACVR10,
            0x8922 => Self::TRCACATR2,
            0x8923 => Self::TRCACATR10,
            0x8924 => Self::TRCDVCVR1,
            0x8925 => Self::TRCDVCVR5,
            0x8926 => Self::TRCDVCMR1,
            0x8927 => Self::TRCDVCMR5,
            0x8930 => Self::TRCACVR3,
            0x8931 => Self::TRCACVR11,
            0x8932 => Self::TRCACATR3,
            0x8933 => Self::TRCACATR11,
            0x8940 => Self::TRCACVR4,
            0x8941 => Self::TRCACVR12,
            0x8942 => Self::TRCACATR4,
            0x8943 => Self::TRCACATR12,
            0x8944 => Self::TRCDVCVR2,
            0x8945 => Self::TRCDVCVR6,
            0x8946 => Self::TRCDVCMR2,
            0x8947 => Self::TRCDVCMR6,
            0x8950 => Self::TRCACVR5,
            0x8951 => Self::TRCACVR13,
            0x8952 => Self::TRCACATR5,
            0x8953 => Self::TRCACATR13,
            0x8960 => Self::TRCACVR6,
            0x8961 => Self::TRCACVR14,
            0x8962 => Self::TRCACATR6,
            0x8963 => Self::TRCACATR14,
            0x8964 => Self::TRCDVCVR3,
            0x8965 => Self::TRCDVCVR7,
            0x8966 => Self::TRCDVCMR3,
            0x8967 => Self::TRCDVCMR7,
            0x8970 => Self::TRCACVR7,
            0x8971 => Self::TRCACVR15,
            0x8972 => Self::TRCACATR7,
            0x8973 => Self::TRCACATR15,
            0x8980 => Self::TRCCIDCVR0,
            0x8981 => Self::TRCVMIDCVR0,
            0x8982 => Self::TRCCIDCCTLR0,
            0x898a => Self::TRCCIDCCTLR1,
            0x8990 => Self::TRCCIDCVR1,
            0x8991 => Self::TRCVMIDCVR1,
            0x8992 => Self::TRCVMIDCCTLR0,
            0x899a => Self::TRCVMIDCCTLR1,
            0x89a0 => Self::TRCCIDCVR2,
            0x89a1 => Self::TRCVMIDCVR2,
            0x89b0 => Self::TRCCIDCVR3,
            0x89b1 => Self::TRCVMIDCVR3,
            0x89c0 => Self::TRCCIDCVR4,
            0x89c1 => Self::TRCVMIDCVR4,
            0x89d0 => Self::TRCCIDCVR5,
            0x89d1 => Self::TRCVMIDCVR5,
            0x89e0 => Self::TRCCIDCVR6,
            0x89e1 => Self::TRCVMIDCVR6,
            0x89f0 => Self::TRCCIDCVR7,
            0x89f1 => Self::TRCVMIDCVR7,
            0x8b84 => Self::TRCITCTRL,
            0x8bc6 => Self::TRCCLAIMSET,
            0x8bce => Self::TRCCLAIMCLR,
            0x8be6 => Self::TRCLAR,
            0x9000 => Self::TEECR32_EL1,
            0x9080 => Self::TEEHBR32_EL1,
            0x9820 => Self::DBGDTR_EL0,
            0x9828 => Self::DBGDTRTX_EL0,
            0xa038 => Self::DBGVCR32_EL2,
            0xc080 => Self::SCTLR_EL1,
            0xc081 => Self::ACTLR_EL1,
            0xc082 => Self::CPACR_EL1,
            0xc085 => Self::RGSR_EL1,
            0xc086 => Self::GCR_EL1,
            0xc091 => Self::TRFCR_EL1,
            0xc100 => Self::TTBR0_EL1,
            0xc101 => Self::TTBR1_EL1,
            0xc102 => Self::TCR_EL1,
            0xc108 => Self::APIAKEYLO_EL1,
            0xc109 => Self::APIAKEYHI_EL1,
            0xc10a => Self::APIBKEYLO_EL1,
            0xc10b => Self::APIBKEYHI_EL1,
            0xc110 => Self::APDAKEYLO_EL1,
            0xc111 => Self::APDAKEYHI_EL1,
            0xc112 => Self::APDBKEYLO_EL1,
            0xc113 => Self::APDBKEYHI_EL1,
            0xc118 => Self::APGAKEYLO_EL1,
            0xc119 => Self::APGAKEYHI_EL1,
            0xc200 => Self::SPSR_EL1,
            0xc201 => Self::ELR_EL1,
            0xc208 => Self::SP_EL0,
            0xc210 => Self::SPSEL,
            0xc212 => Self::CURRENTEL,
            0xc213 => Self::PAN,
            0xc214 => Self::UAO,
            0xc230 => Self::ICC_PMR_EL1,
            0xc288 => Self::AFSR0_EL1,
            0xc289 => Self::AFSR1_EL1,
            0xc290 => Self::ESR_EL1,
            0xc299 => Self::ERRSELR_EL1,
            0xc2a1 => Self::ERXCTLR_EL1,
            0xc2a2 => Self::ERXSTATUS_EL1,
            0xc2a3 => Self::ERXADDR_EL1,
            0xc2a5 => Self::ERXPFGCTL_EL1,
            0xc2a6 => Self::ERXPFGCDN_EL1,
            0xc2a8 => Self::ERXMISC0_EL1,
            0xc2a9 => Self::ERXMISC1_EL1,
            0xc2aa => Self::ERXMISC2_EL1,
            0xc2ab => Self::ERXMISC3_EL1,
            0xc2af => Self::ERXTS_EL1,
            0xc2b0 => Self::TFSR_EL1,
            0xc2b1 => Self::TFSRE0_EL1,
            0xc300 => Self::FAR_EL1,
            0xc3a0 => Self::PAR_EL1,
            0xc4c8 => Self::PMSCR_EL1,
            0xc4ca => Self::PMSICR_EL1,
            0xc4cb => Self::PMSIRR_EL1,
            0xc4cc => Self::PMSFCR_EL1,
            0xc4cd => Self::PMSEVFR_EL1,
            0xc4ce => Self::PMSLATFR_EL1,
            0xc4cf => Self::PMSIDR_EL1,
            0xc4d0 => Self::PMBLIMITR_EL1,
            0xc4d1 => Self::PMBPTR_EL1,
            0xc4d3 => Self::PMBSR_EL1,
            0xc4d7 => Self::PMBIDR_EL1,
            0xc4d8 => Self::TRBLIMITR_EL1,
            0xc4d9 => Self::TRBPTR_EL1,
            0xc4da => Self::TRBBASER_EL1,
            0xc4db => Self::TRBSR_EL1,
            0xc4dc => Self::TRBMAR_EL1,
            0xc4de => Self::TRBTRG_EL1,
            0xc4f1 => Self::PMINTENSET_EL1,
            0xc4f2 => Self::PMINTENCLR_EL1,
            0xc4f6 => Self::PMMIR_EL1,
            0xc510 => Self::MAIR_EL1,
            0xc518 => Self::AMAIR_EL1,
            0xc520 => Self::LORSA_EL1,
            0xc521 => Self::LOREA_EL1,
            0xc522 => Self::LORN_EL1,
            0xc523 => Self::LORC_EL1,
            0xc528 => Self::MPAM1_EL1,
            0xc529 => Self::MPAM0_EL1,
            0xc600 => Self::VBAR_EL1,
            0xc602 => Self::RMR_EL1,
            0xc609 => Self::DISR_EL1,
            0xc641 => Self::ICC_EOIR0_EL1,
            0xc643 => Self::ICC_BPR0_EL1,
            0xc644 => Self::ICC_AP0R0_EL1,
            0xc645 => Self::ICC_AP0R1_EL1,
            0xc646 => Self::ICC_AP0R2_EL1,
            0xc647 => Self::ICC_AP0R3_EL1,
            0xc648 => Self::ICC_AP1R0_EL1,
            0xc649 => Self::ICC_AP1R1_EL1,
            0xc64a => Self::ICC_AP1R2_EL1,
            0xc64b => Self::ICC_AP1R3_EL1,
            0xc659 => Self::ICC_DIR_EL1,
            0xc65d => Self::ICC_SGI1R_EL1,
            0xc65e => Self::ICC_ASGI1R_EL1,
            0xc65f => Self::ICC_SGI0R_EL1,
            0xc661 => Self::ICC_EOIR1_EL1,
            0xc663 => Self::ICC_BPR1_EL1,
            0xc664 => Self::ICC_CTLR_EL1,
            0xc665 => Self::ICC_SRE_EL1,
            0xc666 => Self::ICC_IGRPEN0_EL1,
            0xc667 => Self::ICC_IGRPEN1_EL1,
            0xc668 => Self::ICC_SEIEN_EL1,
            0xc681 => Self::CONTEXTIDR_EL1,
            0xc684 => Self::TPIDR_EL1,
            0xc687 => Self::SCXTNUM_EL1,
            0xc708 => Self::CNTKCTL_EL1,
            0xd000 => Self::CSSELR_EL1,
            0xda10 => Self::NZCV,
            0xda11 => Self::DAIFSET,
            0xda15 => Self::DIT,
            0xda16 => Self::SSBS,
            0xda17 => Self::TCO,
            0xda20 => Self::FPCR,
            0xda21 => Self::FPSR,
            0xda28 => Self::DSPSR_EL0,
            0xda29 => Self::DLR_EL0,
            0xdce0 => Self::PMCR_EL0,
            0xdce1 => Self::PMCNTENSET_EL0,
            0xdce2 => Self::PMCNTENCLR_EL0,
            0xdce3 => Self::PMOVSCLR_EL0,
            0xdce4 => Self::PMSWINC_EL0,
            0xdce5 => Self::PMSELR_EL0,
            0xdce8 => Self::PMCCNTR_EL0,
            0xdce9 => Self::PMXEVTYPER_EL0,
            0xdcea => Self::PMXEVCNTR_EL0,
            0xdced => Self::DAIFCLR,
            0xdcf0 => Self::PMUSERENR_EL0,
            0xdcf3 => Self::PMOVSSET_EL0,
            0xde82 => Self::TPIDR_EL0,
            0xde83 => Self::TPIDRRO_EL0,
            0xde87 => Self::SCXTNUM_EL0,
            0xde90 => Self::AMCR_EL0,
            0xde93 => Self::AMUSERENR_EL0,
            0xde94 => Self::AMCNTENCLR0_EL0,
            0xde95 => Self::AMCNTENSET0_EL0,
            0xde98 => Self::AMCNTENCLR1_EL0,
            0xde99 => Self::AMCNTENSET1_EL0,
            0xdea0 => Self::AMEVCNTR00_EL0,
            0xdea1 => Self::AMEVCNTR01_EL0,
            0xdea2 => Self::AMEVCNTR02_EL0,
            0xdea3 => Self::AMEVCNTR03_EL0,
            0xdee0 => Self::AMEVCNTR10_EL0,
            0xdee1 => Self::AMEVCNTR11_EL0,
            0xdee2 => Self::AMEVCNTR12_EL0,
            0xdee3 => Self::AMEVCNTR13_EL0,
            0xdee4 => Self::AMEVCNTR14_EL0,
            0xdee5 => Self::AMEVCNTR15_EL0,
            0xdee6 => Self::AMEVCNTR16_EL0,
            0xdee7 => Self::AMEVCNTR17_EL0,
            0xdee8 => Self::AMEVCNTR18_EL0,
            0xdee9 => Self::AMEVCNTR19_EL0,
            0xdeea => Self::AMEVCNTR110_EL0,
            0xdeeb => Self::AMEVCNTR111_EL0,
            0xdeec => Self::AMEVCNTR112_EL0,
            0xdeed => Self::AMEVCNTR113_EL0,
            0xdeee => Self::AMEVCNTR114_EL0,
            0xdeef => Self::AMEVCNTR115_EL0,
            0xdef0 => Self::AMEVTYPER10_EL0,
            0xdef1 => Self::AMEVTYPER11_EL0,
            0xdef2 => Self::AMEVTYPER12_EL0,
            0xdef3 => Self::AMEVTYPER13_EL0,
            0xdef4 => Self::AMEVTYPER14_EL0,
            0xdef5 => Self::AMEVTYPER15_EL0,
            0xdef6 => Self::AMEVTYPER16_EL0,
            0xdef7 => Self::AMEVTYPER17_EL0,
            0xdef8 => Self::AMEVTYPER18_EL0,
            0xdef9 => Self::AMEVTYPER19_EL0,
            0xdefa => Self::AMEVTYPER110_EL0,
            0xdefb => Self::AMEVTYPER111_EL0,
            0xdefc => Self::AMEVTYPER112_EL0,
            0xdefd => Self::AMEVTYPER113_EL0,
            0xdefe => Self::AMEVTYPER114_EL0,
            0xdeff => Self::AMEVTYPER115_EL0,
            0xdf00 => Self::CNTFRQ_EL0,
            0xdf10 => Self::CNTP_TVAL_EL0,
            0xdf11 => Self::CNTP_CTL_EL0,
            0xdf12 => Self::CNTP_CVAL_EL0,
            0xdf18 => Self::CNTV_TVAL_EL0,
            0xdf19 => Self::CNTV_CTL_EL0,
            0xdf1a => Self::CNTV_CVAL_EL0,
            0xdf40 => Self::PMEVCNTR0_EL0,
            0xdf41 => Self::PMEVCNTR1_EL0,
            0xdf42 => Self::PMEVCNTR2_EL0,
            0xdf43 => Self::PMEVCNTR3_EL0,
            0xdf44 => Self::PMEVCNTR4_EL0,
            0xdf45 => Self::PMEVCNTR5_EL0,
            0xdf46 => Self::PMEVCNTR6_EL0,
            0xdf47 => Self::PMEVCNTR7_EL0,
            0xdf48 => Self::PMEVCNTR8_EL0,
            0xdf49 => Self::PMEVCNTR9_EL0,
            0xdf4a => Self::PMEVCNTR10_EL0,
            0xdf4b => Self::PMEVCNTR11_EL0,
            0xdf4c => Self::PMEVCNTR12_EL0,
            0xdf4d => Self::PMEVCNTR13_EL0,
            0xdf4e => Self::PMEVCNTR14_EL0,
            0xdf4f => Self::PMEVCNTR15_EL0,
            0xdf50 => Self::PMEVCNTR16_EL0,
            0xdf51 => Self::PMEVCNTR17_EL0,
            0xdf52 => Self::PMEVCNTR18_EL0,
            0xdf53 => Self::PMEVCNTR19_EL0,
            0xdf54 => Self::PMEVCNTR20_EL0,
            0xdf55 => Self::PMEVCNTR21_EL0,
            0xdf56 => Self::PMEVCNTR22_EL0,
            0xdf57 => Self::PMEVCNTR23_EL0,
            0xdf58 => Self::PMEVCNTR24_EL0,
            0xdf59 => Self::PMEVCNTR25_EL0,
            0xdf5a => Self::PMEVCNTR26_EL0,
            0xdf5b => Self::PMEVCNTR27_EL0,
            0xdf5c => Self::PMEVCNTR28_EL0,
            0xdf5d => Self::PMEVCNTR29_EL0,
            0xdf5e => Self::PMEVCNTR30_EL0,
            0xdf60 => Self::PMEVTYPER0_EL0,
            0xdf61 => Self::PMEVTYPER1_EL0,
            0xdf62 => Self::PMEVTYPER2_EL0,
            0xdf63 => Self::PMEVTYPER3_EL0,
            0xdf64 => Self::PMEVTYPER4_EL0,
            0xdf65 => Self::PMEVTYPER5_EL0,
            0xdf66 => Self::PMEVTYPER6_EL0,
            0xdf67 => Self::PMEVTYPER7_EL0,
            0xdf68 => Self::PMEVTYPER8_EL0,
            0xdf69 => Self::PMEVTYPER9_EL0,
            0xdf6a => Self::PMEVTYPER10_EL0,
            0xdf6b => Self::PMEVTYPER11_EL0,
            0xdf6c => Self::PMEVTYPER12_EL0,
            0xdf6d => Self::PMEVTYPER13_EL0,
            0xdf6e => Self::PMEVTYPER14_EL0,
            0xdf6f => Self::PMEVTYPER15_EL0,
            0xdf70 => Self::PMEVTYPER16_EL0,
            0xdf71 => Self::PMEVTYPER17_EL0,
            0xdf72 => Self::PMEVTYPER18_EL0,
            0xdf73 => Self::PMEVTYPER19_EL0,
            0xdf74 => Self::PMEVTYPER20_EL0,
            0xdf75 => Self::PMEVTYPER21_EL0,
            0xdf76 => Self::PMEVTYPER22_EL0,
            0xdf77 => Self::PMEVTYPER23_EL0,
            0xdf78 => Self::PMEVTYPER24_EL0,
            0xdf79 => Self::PMEVTYPER25_EL0,
            0xdf7a => Self::PMEVTYPER26_EL0,
            0xdf7b => Self::PMEVTYPER27_EL0,
            0xdf7c => Self::PMEVTYPER28_EL0,
            0xdf7d => Self::PMEVTYPER29_EL0,
            0xdf7e => Self::PMEVTYPER30_EL0,
            0xdf7f => Self::PMCCFILTR_EL0,
            0xe000 => Self::VPIDR_EL2,
            0xe005 => Self::VMPIDR_EL2,
            0xe080 => Self::SCTLR_EL2,
            0xe081 => Self::ACTLR_EL2,
            0xe088 => Self::HCR_EL2,
            0xe089 => Self::MDCR_EL2,
            0xe08a => Self::CPTR_EL2,
            0xe08b => Self::HSTR_EL2,
            0xe08f => Self::HACR_EL2,
            0xe091 => Self::TRFCR_EL2,
            0xe099 => Self::SDER32_EL2,
            0xe100 => Self::TTBR0_EL2,
            0xe101 => Self::TTBR1_EL2,
            0xe102 => Self::TCR_EL2,
            0xe108 => Self::VTTBR_EL2,
            0xe10a => Self::VTCR_EL2,
            0xe110 => Self::VNCR_EL2,
            0xe130 => Self::VSTTBR_EL2,
            0xe132 => Self::VSTCR_EL2,
            0xe180 => Self::DACR32_EL2,
            0xe200 => Self::SPSR_EL2,
            0xe201 => Self::ELR_EL2,
            0xe208 => Self::SP_EL1,
            0xe218 => Self::SPSR_IRQ,
            0xe219 => Self::SPSR_ABT,
            0xe21a => Self::SPSR_UND,
            0xe21b => Self::SPSR_FIQ,
            0xe281 => Self::IFSR32_EL2,
            0xe288 => Self::AFSR0_EL2,
            0xe289 => Self::AFSR1_EL2,
            0xe290 => Self::ESR_EL2,
            0xe293 => Self::VSESR_EL2,
            0xe298 => Self::FPEXC32_EL2,
            0xe2b0 => Self::TFSR_EL2,
            0xe300 => Self::FAR_EL2,
            0xe304 => Self::HPFAR_EL2,
            0xe4c8 => Self::PMSCR_EL2,
            0xe510 => Self::MAIR_EL2,
            0xe518 => Self::AMAIR_EL2,
            0xe520 => Self::MPAMHCR_EL2,
            0xe521 => Self::MPAMVPMV_EL2,
            0xe528 => Self::MPAM2_EL2,
            0xe530 => Self::MPAMVPM0_EL2,
            0xe531 => Self::MPAMVPM1_EL2,
            0xe532 => Self::MPAMVPM2_EL2,
            0xe533 => Self::MPAMVPM3_EL2,
            0xe534 => Self::MPAMVPM4_EL2,
            0xe535 => Self::MPAMVPM5_EL2,
            0xe536 => Self::MPAMVPM6_EL2,
            0xe537 => Self::MPAMVPM7_EL2,
            0xe600 => Self::VBAR_EL2,
            0xe602 => Self::RMR_EL2,
            0xe609 => Self::VDISR_EL2,
            0xe640 => Self::ICH_AP0R0_EL2,
            0xe641 => Self::ICH_AP0R1_EL2,
            0xe642 => Self::ICH_AP0R2_EL2,
            0xe643 => Self::ICH_AP0R3_EL2,
            0xe648 => Self::ICH_AP1R0_EL2,
            0xe649 => Self::ICH_AP1R1_EL2,
            0xe64a => Self::ICH_AP1R2_EL2,
            0xe64b => Self::ICH_AP1R3_EL2,
            0xe64c => Self::ICH_VSEIR_EL2,
            0xe64d => Self::ICC_SRE_EL2,
            0xe658 => Self::ICH_HCR_EL2,
            0xe65a => Self::ICH_MISR_EL2,
            0xe65f => Self::ICH_VMCR_EL2,
            0xe660 => Self::ICH_LR0_EL2,
            0xe661 => Self::ICH_LR1_EL2,
            0xe662 => Self::ICH_LR2_EL2,
            0xe663 => Self::ICH_LR3_EL2,
            0xe664 => Self::ICH_LR4_EL2,
            0xe665 => Self::ICH_LR5_EL2,
            0xe666 => Self::ICH_LR6_EL2,
            0xe667 => Self::ICH_LR7_EL2,
            0xe668 => Self::ICH_LR8_EL2,
            0xe669 => Self::ICH_LR9_EL2,
            0xe66a => Self::ICH_LR10_EL2,
            0xe66b => Self::ICH_LR11_EL2,
            0xe66c => Self::ICH_LR12_EL2,
            0xe66d => Self::ICH_LR13_EL2,
            0xe66e => Self::ICH_LR14_EL2,
            0xe66f => Self::ICH_LR15_EL2,
            0xe681 => Self::CONTEXTIDR_EL2,
            0xe682 => Self::TPIDR_EL2,
            0xe687 => Self::SCXTNUM_EL2,
            0xe703 => Self::CNTVOFF_EL2,
            0xe708 => Self::CNTHCTL_EL2,
            0xe710 => Self::CNTHP_TVAL_EL2,
            0xe711 => Self::CNTHP_CTL_EL2,
            0xe712 => Self::CNTHP_CVAL_EL2,
            0xe718 => Self::CNTHV_TVAL_EL2,
            0xe719 => Self::CNTHV_CTL_EL2,
            0xe71a => Self::CNTHV_CVAL_EL2,
            0xe720 => Self::CNTHVS_TVAL_EL2,
            0xe721 => Self::CNTHVS_CTL_EL2,
            0xe722 => Self::CNTHVS_CVAL_EL2,
            0xe728 => Self::CNTHPS_TVAL_EL2,
            0xe729 => Self::CNTHPS_CTL_EL2,
            0xe72a => Self::CNTHPS_CVAL_EL2,
            0xe880 => Self::SCTLR_EL12,
            0xe882 => Self::CPACR_EL12,
            0xe891 => Self::TRFCR_EL12,
            0xe900 => Self::TTBR0_EL12,
            0xe901 => Self::TTBR1_EL12,
            0xe902 => Self::TCR_EL12,
            0xea00 => Self::SPSR_EL12,
            0xea01 => Self::ELR_EL12,
            0xea88 => Self::AFSR0_EL12,
            0xea89 => Self::AFSR1_EL12,
            0xea90 => Self::ESR_EL12,
            0xeab0 => Self::TFSR_EL12,
            0xeb00 => Self::FAR_EL12,
            0xecc8 => Self::PMSCR_EL12,
            0xed10 => Self::MAIR_EL12,
            0xed18 => Self::AMAIR_EL12,
            0xed28 => Self::MPAM1_EL12,
            0xee00 => Self::VBAR_EL12,
            0xee81 => Self::CONTEXTIDR_EL12,
            0xee87 => Self::SCXTNUM_EL12,
            0xef08 => Self::CNTKCTL_EL12,
            0xef10 => Self::CNTP_TVAL_EL02,
            0xef11 => Self::CNTP_CTL_EL02,
            0xef12 => Self::CNTP_CVAL_EL02,
            0xef18 => Self::CNTV_TVAL_EL02,
            0xef19 => Self::CNTV_CTL_EL02,
            0xef1a => Self::CNTV_CVAL_EL02,
            0xf080 => Self::SCTLR_EL3,
            0xf081 => Self::ACTLR_EL3,
            0xf088 => Self::SCR_EL3,
            0xf089 => Self::SDER32_EL3,
            0xf08a => Self::CPTR_EL3,
            0xf099 => Self::MDCR_EL3,
            0xf100 => Self::TTBR0_EL3,
            0xf102 => Self::TCR_EL3,
            0xf200 => Self::SPSR_EL3,
            0xf201 => Self::ELR_EL3,
            0xf208 => Self::SP_EL2,
            0xf288 => Self::AFSR0_EL3,
            0xf289 => Self::AFSR1_EL3,
            0xf290 => Self::ESR_EL3,
            0xf2b0 => Self::TFSR_EL3,
            0xf300 => Self::FAR_EL3,
            0xf510 => Self::MAIR_EL3,
            0xf518 => Self::AMAIR_EL3,
            0xf528 => Self::MPAM3_EL3,
            0xf600 => Self::VBAR_EL3,
            0xf602 => Self::RMR_EL3,
            0xf664 => Self::ICC_CTLR_EL3,
            0xf665 => Self::ICC_SRE_EL3,
            0xf667 => Self::ICC_IGRPEN1_EL3,
            0xf682 => Self::TPIDR_EL3,
            0xf687 => Self::SCXTNUM_EL3,
            0xff10 => Self::CNTPS_TVAL_EL1,
            0xff11 => Self::CNTPS_CTL_EL1,
            0xff12 => Self::CNTPS_CVAL_EL1,
            0xff13 => Self::PSTATE_SPSEL,

            _ => panic!("Invalid system register value"),
        }
    }
}
