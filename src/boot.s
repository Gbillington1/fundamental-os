// src/boot.s

// ensure the linker puts this code at the very start of the binary
.section ".text.boot"

// execution starts here
.global _start

_start:
  // 1. Multicore sync
  // read the multiprocessor affinity register (MPIDR_EL1) 
  // to restrict execution to the core with ID 0
  // EL1 means "Exception Level 1", which is the kernel level
  mrs x0, MPIDR_EL1

  // extract the core id, which is affinity level 0 (bits 0-7)
  and x0, x0, #0xFF

  // if the core ID is 0, continue execution
  // else, pause execution
  cbz x0, primary_core

pause_exe:
  // Put non-primary cores into a low-power sleep loop
  wfe
  b pause_exe

primary_core:
  // enabled FP/SIMD access for floating point registers
  // set bits 20 and 21 of CPARCR_EL1 to 0b11
  mov x0, #(3 << 20) // enable EL0/EL1 access
  msr cpacr_el1, x0
  isb // barrier to ensure the change takes effect


  // 2. Initialize the stack pointer using linker symbol
  ldr x0, =__stack_top
  mov sp, x0

  // 3. Clear BSS section 
  // The symbols below are provided by the linker.ld script
  // __bss_start: The memory address where unitialized globals begin
  // __bss_size: The number of 8-byte blocks to clear
  ldr x1, =__bss_start
  ldr w2, =__bss_size

  // if BSS size is 0, jump straight to rust
  cbz w2, jump_to_rust

zero_loop:
  // Store zero (xzr) in the address held in x1, then increment x1 by 8
  str xzr, [x1], #8

  // decrement the counter (w2) and update CPU flags 
  subs w2, w2, #1 

  // if the counter hasn't reached zero, loop again
  bne zero_loop

jump_to_rust:
  // 3. Hand execution over to Rust
  // branch with link to "kmain"
  // calls rust function defined in main.rs
  bl kmain


halt:
  // for safety, if kmain ever returns, hang the CPU in a loop
  wfe
  b halt









