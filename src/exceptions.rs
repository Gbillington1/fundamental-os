// src/exceptions.rs
// all code relating to exception vector table

use crate::{ printk, printk_u64, printk_hex };
use crate::syscall::handle_syscall;
use crate::gic; 

#[repr(C)]
pub(crate) struct TrapFrame {
    pub(crate) x: [u64; 31], // x0 through x30
    pub(crate) sp: u64, // stack pointer at exception entry
    pub(crate) elr_el1: u64,
    pub(crate) spsr_el1: u64,
    pub(crate) esr_el1: u64,
    pub(crate) far_el1: u64,
    pub(crate) vector_id: u64, // 0 through 15
    pub(crate) _padding: u64, // make total size 304 (aligned with asm)
}

// extracts the exception class (EC) field from ESR_EL1
fn esr_ec(esr: u64) -> u64 { (esr >> 26) & 0x3f }

// extracts the instruction specific syndrome (ISS) field from ESR_EL1
fn esr_iss(esr: u64) -> u64 { esr & 0x01ff_ffff }

// common EC values
const EC_SVC_A64: u64 = 0x15;
const EC_INSN_ABORT_CURR: u64 = 0x21;
const EC_DATA_ABORT_CURR: u64 = 0x25;
const EC_BRK_A64: u64 = 0x3C;

fn log_exception(tf: &mut TrapFrame, ec: u64, iss: u64) {
    printk("\n=== EXCEPTION ===\n");
    printk("Source Vector: "); printk_u64(tf.vector_id);

    // determine source of exception based on vector id
    match tf.vector_id {
        0..=3 => printk(" (Current EL with SP0)\n"),
        4..=7 => printk(" (Current EL with SPx)\n"),
        8..=11 => printk(" (Lower EL - AArch46)\n"),
        12..=15 => printk(" (Lower EL - Aarch32)\n"), 
        _ => printk(" (Unknown EC)\n"),
    }

    printk("Exception Class (EC): "); printk_hex(ec);
    printk(" | ISS: "); printk_hex(iss); printk("\n");

    // map EC to human name
    let description = match ec {
        EC_SVC_A64 => "SVC Instruction (System Call)",
        EC_INSN_ABORT_CURR => "Instruction Abort (Current EL)",
        EC_DATA_ABORT_CURR => "Data Abort (Current EL)",
        EC_BRK_A64 => "Breakpoint (BRK)",
        _ => "Unknown Syndrome",
    };

    printk("Description: "); printk(description); printk("\n");
    printk("Faulting PC (ELR): "); printk_hex(tf.elr_el1); printk("\n");

    // only print FAR if memory fault (abort)
    if ec == EC_DATA_ABORT_CURR || ec == EC_INSN_ABORT_CURR {
        printk("Fault Address (FAR): "); printk_hex(tf.far_el1); printk("\n");
    }
    
    printk("=================\n");
}

#[unsafe(no_mangle)]
pub extern "C" fn exception_handler(tf: *mut TrapFrame) {
    let tf = unsafe { &mut *tf };
    
    match tf.vector_id {
        // iqr interrupts (5 for current EL and 9 for lower EL)
        5 | 9 => {
            gic::handle_iqr(tf);
        }

        // synchronous execptions (crashes / syscalls)
        // 4 for current EL and 8 for lower EL
        4 | 8 => {
            handle_synchronous(tf);
        }

        _ => {
            printk("Unknown Vector ID: "); printk_u64(tf.vector_id); printk("\n");
            panic!("Unhandled exception vector");
        }
    }
     
}

fn handle_synchronous(tf: &mut TrapFrame) {
    let ec = esr_ec(tf.esr_el1);
    let iss = esr_iss(tf.esr_el1);

    // only log if its a crash
    if ec != EC_SVC_A64 {
        log_exception(tf, ec, iss);
    }

    // route sync exception
    match ec {
        // software breakpoint
        EC_BRK_A64 => {
            // skip the break instruction so we don't loop
            tf.elr_el1 += 4;
        }

        // system call
        EC_SVC_A64 => {
            handle_syscall(tf, iss);
            // skip svc instruction
            tf.elr_el1 += 4;
        }

        // fatal memory errors
        EC_DATA_ABORT_CURR | EC_INSN_ABORT_CURR => {
            let fault_type = if ec == EC_INSN_ABORT_CURR { "Instruction" } else { "Data" };
            printk(fault_type); printk(" abort at : "); printk_hex(tf.far_el1); printk("\n");
            panic!("Fatal memory exception");
            
        }

        _ => {
            panic!("Unhandled exception. Halting.");
        }
    }
}
