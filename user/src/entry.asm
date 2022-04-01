.section .text.entry
.globl _start
_start:
    ld t1, 0(sp) # t1 = argc
    addi t1, t1, 2
    slli t1, t1, 3
    add a1, sp, t1
    addi a0, sp, 1
    call rust_start

