// src/timer.rs

use crate::gic;
use crate::{ printk, printk_u64 };
use core::arch::asm;

// timer interrupt ID is fixed by aarch64 to 30
const ARCH_TIMER_IRQ_ID: u32 = 30;

// setup timer and register it with the GIC
pub(crate) fn init() {
    unsafe {
        // register handler with the GIC
        gic::register_handler(ARCH_TIMER_IRQ_ID, timer_handler);

        // enable the interrupt line in the GIC
        gic::enable_interrupt(ARCH_TIMER_IRQ_ID);

        // configure ARM generic timer 
        // read frequency to determine a second
        let freq: u64;
        asm!("mrs {}, cntfrq_el0", out(reg) freq);
        printk("Timer Frequency: "); printk_u64(freq); printk("\n");

        // set timer to fire in 1 sec
        set_next_trigger(freq);

        // enable the timer and unmask the interrupt bit
        asm!("msr cntp_ctl_el0, {}", in(reg) 1_u64);
    }
}

// sets the timer to fire "ticks" from now
unsafe fn set_next_trigger(ticks: u64) {
    unsafe {
        // writing to tval_el0 sets the countdown
        asm!("msr cntp_tval_el0, {}", in(reg) ticks);
    }
}

// timer interrupt handler
fn timer_handler() {
    printk("TICK!\n");

    // must reset timer to prevent infinite loop
    unsafe {
        let freq: u64;
        asm!("mrs {}, cntfrq_el0", out(reg) freq);

        // schedule next tick 1 sec later 
        set_next_trigger(freq);
    }
}
