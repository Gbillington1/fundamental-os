# FundamentalOS 

A bare-metal 64-bit ARM (AArch64) operating system written in Rust.

## Hardware
- architecture: aarch64
- machine: qemu virt
- cpu: cortex-a57
- ram base: 0x40000000
- kernel load: 0x40080000
- uart mmio: 0x09000000

## Progress
- [x] multi-core parking
- [x] stack setup
- [x] fpu/simd enablement
- [x] uart polling driver
- [x] rust kmain entry
- [ ] exception vector table
- [ ] rust panic handler
- [ ] interrupt controller (gic)
- [ ] page frame allocator
- [ ] mmu and page tables
- [ ] heap allocator
- [ ] context switching
- [ ] scheduler
- [ ] device tree parser
- [ ] virtio disk driver
