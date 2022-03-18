OUTPUT_ARCH(riscv64gc);
ENTRY(_start);
BASE_ADDRESS = 0x80200000;
KERNEL_OFFSET = 0xFFFFFFC000000000;
KERNEL_SIZE = 0x400000;

SECTIONS
{
    . = BASE_ADDRESS;
    . += KERNEL_OFFSET;

    __kernel_start = .;
    __text_start = .;
    .text : AT(ADDR(.text) - KERNEL_OFFSET)
    {
        *(.text.entry);
        *(.text.paging);
        . = ALIGN(4);
        *(.text.rust_main);
        *(.text .text.*);
    }
    . = ALIGN(4k);
    __text_end = .;

    __rodata_start = .;
    .rodata : AT(ADDR(.rodata) - KERNEL_OFFSET)
    {
        *(.rodata.user_stack);
        *(.rodata .rodata.*);
    }
    . = ALIGN(4k);
    __rodata_end = .;

    __data_start = .;
    .data : AT(ADDR(.data) - KERNEL_OFFSET)
    {
        *(.data .data.*);
        *(.sdata .sdata.*);
    }
    . = ALIGN(4k);
    __data_end = .;

    __bss_start = .;
    .bss : AT(ADDR(.bss) - KERNEL_OFFSET)
    {
        *(.bss.stack);
        *(.bss .bss.*);
        *(.sbss .sbss.*);
    }
    . = ALIGN(4K);
    __bss_end = .;
    __kernel_end = .;

    /DISCARD/ :
    {
        *(.debug* .eh_frame);
    }

}