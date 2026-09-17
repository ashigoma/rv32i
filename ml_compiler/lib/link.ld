OUTPUT_ARCH( "riscv" )
ENTRY( _start )

MEMORY
{
  RAM_MMIO (rwx) : ORIGIN = 0x20000000, LENGTH = 4K
  ROM      (rx)  : ORIGIN = 0x80000000, LENGTH = 64K
  RAM      (rwx) : ORIGIN = 0x90000000, LENGTH = 64K
}

SECTIONS
{ 
  .tohost 0x20000010 (NOLOAD) : {
    *(.tohost)
    *(.fromhost)
  } > RAM_MMIO

  .text : {
    *(.text.init)
  } > ROM

  _stack_top = ORIGIN(RAM) + LENGTH(RAM);
  _heap_bottom = ORIGIN(RAM);
}