// src/gic.rs
// code to handle IQR interrupts via gic (Generic Interrupt Controller)

use crate::{ printk, printk_u64 };
use crate::exceptions::TrapFrame;

// distributor & CPU interface base addresses
const GICD_BASE: usize = 0x0800_0000;
const GICC_BASE: usize = 0x0801_0000;

// distributor registers
const GICD_CTLR:      usize = GICD_BASE + 0x000;
const GICD_ISENABLER: usize = GICD_BASE + 0x100;

// CPU interface registers
const GICC_CTLR: usize = GICC_BASE + 0x000;
// prirority mask
const GICC_PMR:  usize = GICC_BASE + 0x004;
// interrupt acknowledge
const GICC_IAR:  usize = GICC_BASE + 0x00C;
// end of interrupt
const GICC_EOIR: usize = GICC_BASE + 0x010;

// dispatch table
// function pointer type for handlers
type IrqHandler = fn();

// storage for up to 1024 interrupt handlers
// NOTE: needs spinlock when transitioning to multi-core
static mut IRQ_HANDLERS: [Option<IrqHandler>; 1024] = [None; 1024];

// func to register a driver's interrupt handler to an interrupt id
pub(crate) fn register_handler(id: u32, handler: IrqHandler) {
    if id < 1024 {
        unsafe { IRQ_HANDLERS[id as usize] = Some(handler); }
    }
}

// initialize the GIC, called in main.rs before enabling interrupts
pub(crate) unsafe fn init() {
    unsafe {
        // disable everything
        (GICD_CTLR as *mut u32).write_volatile(0);
        (GICC_CTLR as *mut u32).write_volatile(0);

        // CPU interface setup
        // set priority mask to 0xFF, allowing all priorities
        (GICC_PMR as *mut u32).write_volatile(0xFF);
        // enable group 1 interrupts
        (GICC_CTLR as *mut u32).write_volatile(1);

        // distributor setup
        // enable group 1 interrupts
        (GICD_CTLR as *mut u32).write_volatile(1);
    }
}

// enables a specific interrupt id
pub(crate) unsafe fn enable_interrupt(id: u32) {
    unsafe {
        // each reg holds 32 interrupts
        // reg i = id / 32 
        // bit i = id % 32 
        let reg_idx = (id / 32) as usize;
        let bit_idx = id % 32;

        // calc address (base + (4 bytes * index))
        let reg_addr = GICD_ISENABLER + (reg_idx * 4);

        // cast to poiner
        let reg_ptr = reg_addr as *mut u32;

        let val = reg_ptr.read_volatile();
        reg_ptr.write_volatile(val | (1 << bit_idx));
    }
}

// main interrupt dispatcher
// called from exception_handler
pub(crate) fn handle_iqr(_tf: &mut TrapFrame) {
    unsafe {
        // acknowledge interrupt by reading IAR
        let iar = (GICC_IAR as *mut u32).read_volatile();
        // get id from bits 0-9
        let irq_id = iar & 0x3FF; 

        // ignore ID 1023 (Spurious Interrupt)
        if irq_id == 1023 {
            return;
        }

        // dispatch by calling the registered interrupt handler
        if (irq_id as usize) < 1024 {
            match IRQ_HANDLERS[irq_id as usize] {
                Some(handler) => handler(),
                None =>  {
                    printk("Received unexpected IRQ: ");
                    printk_u64(irq_id as u64);
                    printk("\n");
                }
            }
        }

        // write end of interrupt
        // must write back exact value read from IAR
        (GICC_EOIR as *mut u32).write_volatile(iar);
    }
}
