.altmacro

.macro STORE_REG_ i
    sd s\i, 8*(\i+2)(sp)
.endm

.macro LOAD_REG_ i
    ld s\i, 8*(\i+2)(a0)
.endm

    .section .text
    .globl  __switch
    .globl  __record_sp
__switch: // a0: target task context pointer.
    // 1. save current task context onto the stack.
    addi t0, sp, 0
    addi sp, sp, -14*8
    sd ra, 0(sp)
    sd t0, 8(sp)

    .set n, 0
    .rept 12
        STORE_REG_ %n
        .set n, n+1
    .endr
    // 2. load target task context registers.
    ld ra, 0(a0)
    ld sp, 8(a0)

    .set n, 0
    .rept 12
        LOAD_REG_ %n
        .set n, n+1
    .endr
    ret

__record_sp: // a0: ptr of processor_task_context_ptr
    // save sp-sizeof(TaskContext) at the adress of processor_task_context_ptr.
    addi t0, sp, -14*8  // sizeof(TaskContext) == 14 * 8
    sd t0, 0(a0)
    ret