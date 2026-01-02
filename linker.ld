/* linker.ld */

/* tell the linker that the first instruction to execute 
 * is the "_start" label */
ENTRY(_start)

SECTIONS
{
  /* QEMU virtual machine RAM starts here
   * shift OS up by 0x80000 (512KB) to leave room for the device tree (DTB, QEMU specifc)
   * "." is the Location Counter */
  . = 0x40080000;

  /* code section */
  .text : {
    /* force our boot assembly to the front */
    KEEP(*(.text.boot))
    *(.text .text.*)
  }

  /* read-only data */
  .rodata : {
    *(.rodata .rodata.*)
  }

  /* initialized global vars */ 
  .data : {
    *(.data .data.*)
  }

  /* BSS (zero-initialized) section - this is cleared in assembly */
  .bss : {
    . = ALIGN(8) ; /* align to 8 byte boundary for assembly loop */
    __bss_start = .;
    *(.bss .bss.*)
    *(COMMON)
    . = ALIGN(8);
    __bss_end = .;
  }

  /* calculate size for the assembly "ldr w2, =__bss_size" instruction */
  __bss_size = (__bss_end - __bss_start) >> 3;

  /* stack section, aligned to 16 bytes */
  .stack (NOLOAD) : {
    . = ALIGN(16);
    __stack_bottom = .;
    . += 0x10000; /* reserve 64KB for the kernel */
    __stack_top = .;
  }

  /* exception vector table section, aligned to 2048 bytes */
  . = ALIGN(2048);
  .exceptions : {
    KEEP(*(.exceptions))
  }

  /* ignore sections we don't need for a simple kernel */
  /DISCARD/ : { *(.comment) *(.gnu*) *(.note*) *(.eh_frame) }
}

