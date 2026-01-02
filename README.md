# FundamentalOS 

A bare-metal 64-bit ARM (AArch64) operating system written in Rust. Applying the fundamentals of OS theory to a real project.

## Hardware
- architecture: aarch64
- machine: qemu virt
- cpu: cortex-a57
- ram base: 0x40000000
- kernel load: 0x40080000
- uart mmio: 0x09000000

## Progress
- [x] boot loader
- [x] stack setup
- [x] printing with uart
- [x] rust kmain entry
- [x] exception vector table
  - [x] rust exception handler
  - [ ] restore registers before exception returns
- [x] rust panic handler
- [ ] interrupt controller (gic)
- [ ] page frame allocator
- [ ] mmu and page tables
- [ ] heap allocator
- [ ] context switching
- [ ] scheduler
- [ ] device tree parser
- [ ] virtio disk driver
