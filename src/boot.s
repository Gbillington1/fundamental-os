// src/boot.s

// ensure the linker puts this code at the very start of the binary
.section ".text.boot"

// execution starts here
.global _start

_start:
  // read the multiprocessor affinity register (MPIDR_EL1) 
  // to restrict execution to the core with ID 0
  mrs x0, MPIDR_EL1

  // extract the core id, which is affinity level 0 (bits 0-7)
  and x0, x0, #0xFF

  // if the core ID is 0, continue execution
  // else, pause execution
  cbz x0, primary_core

// sleep non-primary cores
pause_exe:
  wfe
  b pause_exe

primary_core:
  // enable FP/SIMD access for floating point registers
  // set bits 20 and 21 of CPARCR_EL1 to 0b11
  mov x0, #(3 << 20) 
  msr cpacr_el1, x0
  isb 

  // move top of stack addr into stack pointer reg
  ldr x0, =__stack_top
  mov sp, x0

  // load execption vector table address into VBAR for level EL1
  ldr x0, =__exceptions
  msr VBAR_EL1, x0
  isb

  // load bss start & size into registers
  ldr x1, =__bss_start
  ldr w2, =__bss_size

  // if bss size is 0, jump straight to rust
  cbz w2, jump_to_rust

// else, zero bss section
zero_loop:
  // store zero (xzr) in the address held in x1, then increment x1 by 8
  str xzr, [x1], #8

  // decrement the counter (w2) and update CPU flags 
  subs w2, w2, #1 

  // if the counter hasn't reached zero, loop again
  bne zero_loop

jump_to_rust:
  // hand off to rust
  // branch with link to "kmain"
  bl kmain

halt:
  // for safety, if kmain ever returns, hang the CPU in a loop
  wfe
  b halt
