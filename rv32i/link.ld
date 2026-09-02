OUTPUT_ARCH( "riscv" )
ENTRY( _start )

MEMORY
{
  RAM_MMIO (rwx) : ORIGIN = 0x20000000, LENGTH = 4K
  ROM      (rx)  : ORIGIN = 0x80000000, LENGTH = 4K
  RAM      (rwx) : ORIGIN = 0x90000000, LENGTH = 4K
}

SECTIONS
{ 
  .tohost 0x20000010 (NOLOAD) : {
    *(.tohost)
    *(.fromhost)
  } > RAM_MMIO

  .text : {
    *(.text.init)
    *(.text .text.*)
  } > ROM

  .rodata : {
    *(.rodata .rodata.*)
  } > ROM

  _sidata = LOADADDR(.data);
  .data : {
    _sdata = .;
    *(.data .data.*)
    _edata = .;
  } > RAM AT > ROM

  .bss : {
    _sbss = .;
    *(.bss .bss.*)
    *(COMMON)
    _ebss = .;
  } > RAM

  _stack_top = ORIGIN(RAM) + LENGTH(RAM);
}