use crate::register::mie::Mie;

impl Mie {
    // Interrupts from 0..16 are disconnected on Ballast SysCtrl (PULPissimo
    // Ibex). We use custom fast interrupts starting from 16.. instead.

    /// APB timer low event
    #[inline]
    pub fn timer_lo_evt(&self) -> bool {
        self.bits & (1 << 16) != 0
    }

    /// APB timer high event
    #[inline]
    pub fn timer_hi_evt(&self) -> bool {
        self.bits & (1 << 17) != 0
    }

    /// Clock edge (rise or fall) event
    #[inline]
    pub fn clk_transit_evt(&self) -> bool {
        self.bits & (1 << 18) != 0
    }

    /// GPIO event
    #[inline]
    pub fn gpio_evt(&self) -> bool {
        self.bits & (1 << 19) != 0
    }

    /// Advanced timer event 0
    #[inline]
    pub fn adv_timer_evt_0(&self) -> bool {
        self.bits & (1 << 20) != 0
    }

    /// Advanced timer event 1
    #[inline]
    pub fn adv_timer_evt_1(&self) -> bool {
        self.bits & (1 << 21) != 0
    }

    /// Advanced timer event 2
    #[inline]
    pub fn adv_timer_evt_2(&self) -> bool {
        self.bits & (1 << 22) != 0
    }

    /// Advanced timer event 3
    #[inline]
    pub fn adv_timer_evt_3(&self) -> bool {
        self.bits & (1 << 23) != 0
    }

    /// External interrupt 0
    ///
    /// ???: This may or may not be used on Ballast for "other subsystems".
    /// Maybe it can be connected via the top-level interrupt router? Please
    /// remove this function, or this comment block, once we know.
    #[inline]
    pub fn ext_0(&self) -> bool {
        self.bits & (1 << 24) != 0
    }

    /// External interrupt 1
    ///
    /// ???: This may or may not be used on Ballast for "other subsystems".
    /// Maybe it can be connected via the top-level interrupt router? Please
    /// remove this function, or this comment block, once we know.
    #[inline]
    pub fn ext_1(&self) -> bool {
        self.bits & (1 << 25) != 0
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
        self.bits & (1 << 27) != 0
    }

    /// ???: Unknown purpose ('fabric controller *something*'). SoC event
    /// generator is the signal source, however. Apparently events can be routed
    /// to these lines from cluster or peripherals.
    ///
    /// TODO: if you know what this is, please update the `pulpissimo-riscv`
    /// crate with appropriate documentation.
    #[inline]
    pub fn fc_hp_evt_0(&self) -> bool {
        self.bits & (1 << 28) != 0
    }

    /// ???: Unknown purpose ('fabric controller *something*'). SoC event
    /// generator is the signal source, however. Apparently events can be routed
    /// to these lines from cluster or peripherals.
    ///
    /// TODO: if you know what this is, please update the `pulpissimo-riscv`
    /// crate with appropriate documentation.
    #[inline]
    pub fn fc_hp_evt_1(&self) -> bool {
        self.bits & (1 << 29) != 0
    }

    /// External interrupt 2
    ///
    /// ???: This may or may not be used on Ballast for "other subsystems".
    /// Maybe it can be connected via the top-level interrupt router? Please
    /// remove this function, or this comment block, once we know.
    #[inline]
    pub fn ext_2(&self) -> bool {
        self.bits & (1 << 30) != 0
    }
}

set!(0x304);
clear!(0x304);

set_clear_csr!(
    /// APB timer low event
    , set_timer_lo_evt, clear_timer_lo_evt, 1 << 16
);
set_clear_csr!(
    /// APB timer high event
    , set_timer_hi_evt, clear_timer_hi_evt, 1 << 17
);
set_clear_csr!(
    /// Clock edge (rise or fall) event
    , set_clk_transit_evt, clear_clk_transit_evt, 1 << 18
);
set_clear_csr!(
    /// GPIO event
    , set_gpio_evt, clear_gpio_evt, 1 << 19
);
set_clear_csr!(
    /// Advanced timer event 0
    , set_adv_timer_evt_0, clear_adv_timer_evt_0, 1 << 20
);
set_clear_csr!(
    /// Advanced timer event 1
    , set_adv_timer_evt_1, clear_adv_timer_evt_1, 1 << 21
);
set_clear_csr!(
    /// Advanced timer event 2
    , set_adv_timer_evt_2, clear_adv_timer_evt_2, 1 << 22
);
set_clear_csr!(
    /// Advanced timer event 3
    , set_adv_timer_evt_3, clear_adv_timer_evt_3, 1 << 23
);
set_clear_csr!(
    /// External interrupt 0
    , set_ext_0, clear_ext_0, 1 << 24
);
set_clear_csr!(
    /// External interrupt 1
    , set_ext_1, clear_ext_1, 1 << 25
);
set_clear_csr!(
    /// SoC event FIFO
    , set_soc_evt_fifo, clear_soc_evt_fifo, 1 << 26
);
set_clear_csr!(
    /// Fabric controller error event
    , set_fc_err_evt, clear_fc_err_evt, 1 << 27
);
set_clear_csr!(
    /// ???: Unknown purpose ('fabric controller *something*')
    , set_fc_hp_evt_0, clear_fc_hp_evt_0, 1 << 28
);
set_clear_csr!(
    /// ???: Unknown purpose ('fabric controller *something*')
    , set_fc_hp_evt_1, clear_fc_hp_evt_1, 1 << 29
);
set_clear_csr!(
    /// External interrupt 2
    , set_ext_2, clear_ext_2, 1 << 30
);
