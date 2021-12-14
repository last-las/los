OUTPUT_ARCH(riscv);
ENTRY(_start);
BASE_ADDRESS = 0x83500000;

SECTIONS
{
    . = BASE_ADDRESS;
    .text :
    {
        *(.text.entry);
        *(.text .text.*);
    }

    .rodata :
    {
        *(.rodata .rodata.*);
    }

    .data :
    {
        *(.data .data.*);
        *(.sdata .sdata.*);
    }

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
