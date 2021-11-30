OUTPUT_ARCH(riscv);
ENTRY(_start);
BASE_ADDRESS = 0x80200000;

SECTIONS
{
    . = BASE_ADDRESS;
    .text :
    {
        *(.text.entry);
        *(.text .text.*);
    }

    srodata = .;
    .rodata :
    {
        *(.rodata .rodata.*);
    }
    erodata = .;

    sdata = .;
    .data :
    {
        *(.data .data.*);
        *(.sdata .sdata.*);
    }
    edata = .;

    sbss = .;
    .bss :
    {
        *(.bss .bss.*);
        *(.sbss .sbss.*);
    }
    ebss = .;

    /DISCARD/ :
    {
        *(.debug* .eh_frame);
    }

}