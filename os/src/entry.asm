    .section .text.entry
    .globl _start
# a0 = hartid
_start:
    addi t0, a0, 1
    slli t0, t0, 16         # t0 = 4096 * 16 * (hartid + 1)
    la t1, boot_stack
    add sp, t1, t0
    call rust_main

    .section .bss.stack
    .globl boot_stack
    .globl boot_stack_top
boot_stack:
    .space 4096 * 16 * 2