// src/exceptions.s

// save registers using trap frame
.section ".text", "ax"

// trap frame consts
.equ TF_SIZE, 304
.equ TF_X_OFF, 0
.equ TF_SP_OFF, 248
.equ TF_ELR_OFF, 256
.equ TF_SPSR_OFF, 264
.equ TF_ESR_OFF, 272
.equ TF_FAR_OFF, 280
.equ TF_VID_OFF, 288

// save all x0 - x30 to sp, plus SP/ELR/SPSR/ESR/FAR and vector id
.macro SAVE_AND_CALL id
  // allocate space on the stack for the TrapFrame
  sub sp, sp, #TF_SIZE

  // save general purpose regs. as pairs, then x30 on its own
  stp x0, x1, [sp, #(TF_X_OFF + 0)]
  stp x2, x3, [sp, #(TF_X_OFF + 16)]
  stp x4, x5, [sp, #(TF_X_OFF + 32)]
  stp x6, x7, [sp, #(TF_X_OFF + 48)]
  stp x8, x9, [sp, #(TF_X_OFF + 64)]
  stp x10, x11, [sp, #(TF_X_OFF + 80)]
  stp x12, x13, [sp, #(TF_X_OFF + 96)]
  stp x14, x15, [sp, #(TF_X_OFF + 112)]
  stp x16, x17, [sp, #(TF_X_OFF + 128)]
  stp x18, x19, [sp, #(TF_X_OFF + 144)]
  stp x20, x21, [sp, #(TF_X_OFF + 160)]
  stp x22, x23, [sp, #(TF_X_OFF + 176)]
  stp x24, x25, [sp, #(TF_X_OFF + 192)]
  stp x26, x27, [sp, #(TF_X_OFF + 208)]
  stp x28, x29, [sp, #(TF_X_OFF + 224)]
  str x30, [sp, #(TF_X_OFF + 240)]

  // save original sp (pre-alloc)
  add x0, sp, #TF_SIZE
  str x0, [sp, #TF_SP_OFF]

  // save system registers
  mrs x0, ELR_EL1
  str x0, [sp, #TF_ELR_OFF]

  mrs x0, SPSR_EL1 
  str x0, [sp, #TF_SPSR_OFF]

  mrs x0, ESR_EL1
  str x0, [sp, #TF_ESR_OFF]

  mrs x0, FAR_EL1
  str x0, [sp, #TF_FAR_OFF]

  // store vector id
  mov x0, #\id
  str x0, [sp, #TF_VID_OFF]

  // call rust exception handler
  mov x0, sp
  bl exception_handler

  // restore system registers
  ldr x0, [sp, #TF_ELR_OFF]
  msr ELR_EL1, x0
  
  ldr x0, [sp, #TF_SPSR_OFF]
  msr SPSR_EL1, x0

  // restore general purpose registers
  ldp x0, x1, [sp, #(TF_X_OFF + 0)]
  ldp x2, x3, [sp, #(TF_X_OFF + 16)]
  ldp x4, x5, [sp, #(TF_X_OFF + 32)]
  ldp x6, x7, [sp, #(TF_X_OFF + 48)]
  ldp x8, x9, [sp, #(TF_X_OFF + 64)]
  ldp x10, x11, [sp, #(TF_X_OFF + 80)]
  ldp x12, x13, [sp, #(TF_X_OFF + 96)]
  ldp x14, x15, [sp, #(TF_X_OFF + 112)]
  ldp x16, x17, [sp, #(TF_X_OFF + 128)]
  ldp x18, x19, [sp, #(TF_X_OFF + 144)]
  ldp x20, x21, [sp, #(TF_X_OFF + 160)]
  ldp x22, x23, [sp, #(TF_X_OFF + 176)]
  ldp x24, x25, [sp, #(TF_X_OFF + 192)]
  ldp x26, x27, [sp, #(TF_X_OFF + 208)]
  ldp x28, x29, [sp, #(TF_X_OFF + 224)]
  ldr x30, [sp, #(TF_X_OFF + 240)]

  // pop TrapFrame off the stack
  add sp, sp, #TF_SIZE

  // return to address saved in ELR_EL1
  eret

.endm

// define 16 handlers first in the text section
.irp n,0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15
  .global vec_\n
vec_\n:
  SAVE_AND_CALL \n
.endr

// define exception vectors table
.section ".exceptions", "ax"
.align 11 // 2^11 alignment
.global __exceptions

// macro to define table entry
.macro ENTRY label
  b \label
  .rept 31 
    nop
  .endr
.endm

__exceptions:

.irp n,0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15
  ENTRY vec_\n
.endr

// TODO: Restore registers before returning from exception (eret)
