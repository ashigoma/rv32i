OUTPUT_ARCH( "riscv" )
ENTRY( _start )

SECTIONS
{
  . = 0x80000000;
  .text : {
    *(.text.init)
    *(.text .text.*)
  }

  .tohost : { 
    *(.tohost) 
  }

  .data : { *(.data .data.*) }
  .bss  : { *(.bss .bss.*) }
}