// src/exceptions.rs
// all code relating to exception vector table

use crate::{ printk, printk_u64, printk_hex, halt };

#[repr(C)]
pub struct TrapFrame {
    pub x: [u64; 31], // x0 through x30
    pub sp: u64, // stack pointer at exception entry
    pub elr_el1: u64,
    pub spsr_el1: u64,
    pub esr_el1: u64,
    pub far_el1: u64,
    pub vector_id: u64, // 0 through 15
}

// extracts the exception class (EC) field from ESR_EL1
fn esr_ec(esr: u64) -> u64 { (esr >> 26) & 0x3f }

// extracts the instruction specific syndrome (ISS) field from ESR_EL1
fn esr_iss(esr: u64) -> u64 { esr & 0x01ff_ffff }

// common EC values
const EC_SVC_A64: u64 = 0b010101;
const EC_INSN_ABORT_CURR: u64 = 0b100001;
const EC_DATA_ABORT_CURR: u64 = 0b100101;
const EC_BRK_A64: u64 = 0b111100;

#[unsafe(no_mangle)]
pub extern "C" fn exception_handler(tf: *mut TrapFrame) -> ! {
    let tf = unsafe { &*tf };

    let ec = esr_ec(tf.esr_el1);
    let iss = esr_iss(tf.esr_el1);

    printk("\n=== EXCEPTION ===\n");
    printk("vector_id: "); printk_u64(tf.vector_id); printk("\n");

    printk("ESR_EL1: "); printk_hex(tf.esr_el1); printk("\n");
    printk("    EC: "); printk_u64(ec); printk(" ");
    printk("ISS: "); printk_hex(iss); printk("\n");

    printk("    Type: ");
    match ec {
        EC_BRK_A64 => printk("BRK (breakpoint)\n"),
        EC_SVC_A64 => printk("SVC (syscall)\n"),
        EC_DATA_ABORT_CURR => printk("Data Abort (current EL)\n"),
        EC_INSN_ABORT_CURR => printk("Instruction Abort (current EL)\n"),
        _ => printk("Unknown/Unhandled\n"),
    }
    
    printk("ELR_EL1: "); printk_hex(tf.elr_el1); printk("\n");
    printk("FAR_EL1: "); printk_hex(tf.far_el1); printk("\n");
    printk("SPSR_EL1: "); printk_hex(tf.spsr_el1); printk("\n");
    printk("=================\n");

    halt();
}
