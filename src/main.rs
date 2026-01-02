// src/main.rs
#![no_std]
#![no_main]

mod exceptions;

use core::panic::PanicInfo;
use core::arch::global_asm;
use core::arch::asm;
use core::ptr::write_volatile;

// inject boot & exception vector table asm
global_asm!(include_str!("boot.s"));
global_asm!(include_str!("exceptions.s"));

const UART_DR : *mut u32 = 0x900_0000 as *mut u32;

// halt instruction
#[inline(always)]
pub(crate) fn halt() -> ! {
    loop {
        // aarch64 wait for event asm
        unsafe { asm!("wfe") }
    }
}

// write a byte to the UART interface 
pub(crate) fn uart_putc(byte: u8) {
    unsafe {
        write_volatile(UART_DR, byte as u32);
    }
}

// kernel space print
pub(crate) fn printk(s: &str) {
    for byte in s.bytes() {
        uart_putc(byte);
    }
}

// prints a u64 int as a string
pub(crate) fn printk_u64(mut n: u64) {
    if n == 0 {
        uart_putc(b'0');
        return;
    }

    // create buffer of 20 digits
    let mut buf = [0u8; 20];
    // digit index
    let mut i = 0usize;

    // extract digits (base 10 comp.)
    while n > 0 {
        let digit = (n % 10) as u8;
        buf[i] = b'0' + digit;
        n /= 10;
        i += 1;
    }

    // print digits to uart (in reverse)
    while i > 0 {
        i -= 1;
        uart_putc(buf[i]);
    }
}

// prints a u64 int as hex
pub(crate) fn printk_hex(v: u64) {
    printk("0x");
    for i in (0..16).rev() {
        let nib = ((v >> (i * 4)) & 0xF) as u8;
        uart_putc(if nib < 10 { b'0' + nib } else { b'a' + (nib - 10) });
    }
}

// Panic handler
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    printk("\n=== KERNEL PANIC ===\n");

    // print msg
    printk("msg: ");
    if let Some(msg) = info.message().as_str() {
        printk(msg);
        printk("\n");
    } else {
        printk("(non-str panic)\n");
    }

    // print location
    printk("at: ");
    if let Some(loc) = info.location() {
        printk(loc.file());
        printk(", line ");
        printk_u64(loc.line() as u64);
        printk(", col ");
        printk_u64(loc.column() as u64);
        printk("\n");
    } else {
        printk("(unknown)\n");
    }
    printk("====================\n");

    // stop execution
    halt();
}

// KERNEL MAIN FUNCTION
// no_mangle: ensures the symbol name stays "kmain" for the assmebly to find
// extern C: ensures the function uses the C calling convention
// -> !: the "never" type, stating that this function never returns to caller
#[unsafe(no_mangle)]
pub extern "C" fn kmain() -> ! {

    // say something to the user
    printk("=== FUNDAMENTAL OS ===\n");
    printk("Boot sequence complete.\n");

    // trigger exception
    unsafe { asm!("brk #0"); }

    // after init, the kernel enters an idle loop
    // TODO: jump to scheduler or shell
    loop {}
}



