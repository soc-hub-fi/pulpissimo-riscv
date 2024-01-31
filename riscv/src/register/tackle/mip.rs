use crate::register::mip::Mip;

impl Mip {
    /// ???: Unknown purpose ('DMA *something* event')
    ///
    /// TODO: if you know what this is, please update the `pulpissimo-riscv`
    /// crate with appropriate documentation.
    #[inline]
    pub fn dma_pe_evt(&self) -> bool {
        self.bits & (1 << 8) != 0
    }

    /// ???: Unknown purpose ('DMA *something* IRQ')
    ///
    /// TODO: if you know what this is, please update the `pulpissimo-riscv`
    /// crate with appropriate documentation.
    #[cfg(feature = "tackle")]
    #[inline]
    pub fn dma_pe_irq(&self) -> bool {
        self.bits & (1 << 9) != 0
    }

    /// APB timer low event
    #[inline]
    pub fn timer_lo_evt(&self) -> bool {
        self.bits & (1 << 10) != 0
    }

    /// APB timer high event
    #[inline]
    pub fn timer_hi_evt(&self) -> bool {
        self.bits & (1 << 11) != 0
    }

    /// PULP FPGA external interrupt signal
    #[inline]
    pub fn pf_ext_irq(&self) -> bool {
        self.bits & (1 << 12) != 0
    }

    /// Clock edge (rise or fall) event
    #[inline]
    pub fn clk_transit_evt(&self) -> bool {
        self.bits & (1 << 14) != 0
    }

    /// GPIO event
    #[inline]
    pub fn gpio_evt(&self) -> bool {
        self.bits & (1 << 15) != 0
    }

    /// Advanced timer event 0
    #[inline]
    pub fn adv_timer_evt_0(&self) -> bool {
        self.bits & (1 << 17) != 0
    }

    /// Advanced timer event 1
    #[inline]
    pub fn adv_timer_evt_1(&self) -> bool {
        self.bits & (1 << 18) != 0
    }

    /// Advanced timer event 2
    #[inline]
    pub fn adv_timer_evt_2(&self) -> bool {
        self.bits & (1 << 19) != 0
    }

    /// Advanced timer event 3
    #[inline]
    pub fn adv_timer_evt_3(&self) -> bool {
        self.bits & (1 << 20) != 0
    }

    /// SoC event FIFO
    ///
    /// Many events get muxed into this interrupt. User must check
    /// event/interrupt unit registers to find out the source.
    #[inline]
    pub fn soc_evt_fifo(&self) -> bool {
        self.bits & (1 << 26) != 0
    }

    /// Fabric controller error event
    #[inline]
    pub fn fc_err_evt(&self) -> bool {
        self.bits & (1 << 29) != 0
    }

    /// ???: Unknown purpose ('fabric controller *something*'). SoC event
    /// generator is the signal source, however. Apparently events can be routed
    /// to these lines from cluster or peripherals.
    ///
    /// TODO: if you know what this is, please update the `pulpissimo-riscv`
    /// crate with appropriate documentation.
    #[inline]
    pub fn fc_hp_evt_0(&self) -> bool {
        self.bits & (1 << 30) != 0
    }

    /// ???: Unknown purpose ('fabric controller *something*'). SoC event
    /// generator is the signal source, however. Apparently events can be routed
    /// to these lines from cluster or peripherals.
    ///
    /// TODO: if you know what this is, please update the `pulpissimo-riscv`
    /// crate with appropriate documentation.
    #[inline]
    pub fn fc_hp_evt_1(&self) -> bool {
        self.bits & (1 << 31) != 0
    }
}

read_csr_as!(Mip, 0x7d2);
set!(0x7d2);
clear!(0x7d2);

set_clear_csr!(
    /// ???: Unknown purpose ('DMA *something* event')
    , set_dma_pe_evt, clear_dma_pe_evt, 1 << 8
);
set_clear_csr!(
    /// ???: Unknown purpose ('DMA *something* IRQ')
    , set_dma_pe_irq, clear_dma_pe_irq, 1 << 9
);
set_clear_csr!(
    /// APB timer low event
    , set_timer_lo_evt, clear_timer_lo_evt, 1 << 10
);
set_clear_csr!(
    /// APB timer high event
    , set_timer_hi_evt, clear_timer_hi_evt, 1 << 11
);
set_clear_csr!(
    /// PULP FPGA external interrupt signal
    , set_pf_ext_irq, clear_pf_ext_irq, 1 << 12
);
set_clear_csr!(
    /// Clock edge (rise or fall) event
    , set_clk_transit_evt, clear_clk_transit_evt, 1 << 14
);
set_clear_csr!(
    /// GPIO event
    , set_gpio_evt, clear_gpio_evt, 1 << 15
);
set_clear_csr!(
    /// Advanced timer event 0
    , set_adv_timer_evt_0, clear_adv_timer_evt_0, 1 << 17
);
set_clear_csr!(
    /// Advanced timer event 1
    , set_adv_timer_evt_1, clear_adv_timer_evt_1, 1 << 18
);
set_clear_csr!(
    /// Advanced timer event 2
    , set_adv_timer_evt_2, clear_adv_timer_evt_2, 1 << 19
);
set_clear_csr!(
    /// Advanced timer event 3
    , set_adv_timer_evt_3, clear_adv_timer_evt_3, 1 << 20
);
set_clear_csr!(
    /// SoC event FIFO
    , set_soc_evt_fifo, clear_soc_evt_fifo, 1 << 26
);
set_clear_csr!(
    /// Fabric controller error event
    , set_fc_err_evt, clear_fc_err_evt, 1 << 29
);
set_clear_csr!(
    /// ???: Unknown purpose ('fabric controller *something*')
    , set_fc_hp_evt_0, clear_fc_hp_evt_0, 1 << 30
);
set_clear_csr!(
    /// ???: Unknown purpose ('fabric controller *something*')
    , set_fc_hp_evt_1, clear_fc_hp_evt_1, 1 << 31
);
