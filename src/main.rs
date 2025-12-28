// src/main.rs
#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::arch::global_asm;

// inject assembly from boot.sh into the compilation unit
global_asm!(include_str!("boot.s"));

// UART (serial port) abstraction
// QEMU's virtual machine has the UART serial port mapped to 0x900_0000
struct Uart {
    base_address: *mut u8,
}

impl Uart {
    // initialize the UART driver by casting the physical
    // address to a raw pointer
    pub fn new(address: usize) -> Self {
        Uart {
            base_address: address as *mut u8,
        }
    }

    // write a single byte to the hardware
    pub fn write_byte(&self, byte: u8) {
        // MMIO Note: using write_volatile to let the complier know
        // that this mem write has side effects (printing to screen)
        // and it shouldn't be optimized
        unsafe {
            core::ptr::write_volatile(self.base_address, byte);
        }
    }

    // iterate over a string slice and
    // write each byte to the UART
    pub fn write_string(&self, s: &str) {
        for byte in s.bytes() {
            self.write_byte(byte);
        }
    }
}

// KERNEL MAIN FUNCTION
// no_mangle: ensures the symbol name stays "kmain" for the assmebly to find
// extern C: ensures the function uses the C calling convention
// -> !: the "never" type, stating that this function never returns to caller
#[unsafe(no_mangle)]
pub extern "C" fn kmain() -> ! {
    // init our UART driver with the QEMU virtual hardware address
    let uart = Uart::new(0x0900_0000);

    // say something to the user
    uart.write_string("FundamentalOS: Boot sequence complete.\n");
    uart.write_string("Hello Graham, the kernel is now in control.\n");

    // after init, the kernel enters an idle loop
    // TODO: jump to scheduler or shell
    loop {}
}

// Panic handler
// since we have no OS to catch crashes, we must define what happens
// if the rust code panics
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // for now, if we panic, we just hang the system
    // TODO: use Uart struct to print the error message here
    loop {}
}

