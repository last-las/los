.altmacro

.macro STORE_REG_ i
    sd s\i, 8*(\i+2)(a0)
.endm

.macro LOAD_REG_ i
    ld s\i, 8*(\i+2)(a1)
.endm

    .section .text
    .globl  __switch
__switch: // a0: current context pointer; a1: target context pointer.
    // save current task context into a0, when a0 equals zero we skip this block.
    beqz a0, load_context
    sd ra, 0(a0)
    sd sp, 8(a0)
    .set n, 0
    .rept 12
        STORE_REG_ %n
        .set n, n+1
    .endr
load_context:
    // load target task context with a1.
    ld ra, 0(a1)
    ld sp, 8(a1)
    .set n, 0
    .rept 12
        LOAD_REG_ %n
        .set n, n+1
    .endr

    ret