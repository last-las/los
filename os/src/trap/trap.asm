.altmacro
.macro LOAD_REG i
    ld x\i, 8*\i(sp)
.endm

.macro STORE_REG i
    sd x\i, 8*\i(sp)
.endm

    .section .text
    .globl __enter_user_mode
    .globl __from_user_mode

__from_user_mode:
    # switch to kernel sp
    csrrw sp, sscratch, sp   # now sp-> kernel_stack, sscratch-> user_stack

    # decrease stack size
    addi sp, sp, -34*8

    # store sstatus, sepc, user_stack
    csrr t0, sstatus
    csrr t1, sepc
    csrr t2, sscratch
    sd t0, 32*8(sp)
    sd t1, 33*8(sp)
    sd t2, 2*8(sp)

    # store x[0,1,3,5-31], x2/sp is stored before, x4/tp won't set.
    sd x0, 0(sp)
    sd x1, 8(sp)
    sd x3, 8*3(sp)

    .set n, 5
    .rept 27
        STORE_REG %n
        .set n, n+1
    .endr

    # jump to trap_handler, this address should be set in stvec CSR.
    call trap_handler

__enter_user_mode:
    # load sstatus,sepc
    ld t0, 32*8(sp)
    ld t1, 33*8(sp)
    csrw sstatus, t0
    csrw sepc, t1

    # load x[0,1,3,5-31], x2/sp will be load later, x4/tp won't load due to it's storing hart id.
    ld x0, 0(sp)
    ld x1, 8(sp)
    ld x3, 3*8(sp)

    .set n, 5
    .rept 27
        LOAD_REG %n
        .set n, n+1
    .endr

    # increase stack size.
    addi sp, sp, 34*8

    # record current kernel sp
    csrw sscratch, sp

    # load sp/x2
    ld sp, 2*8-34*8(sp)

    # enter user mode
    # 0x54
    sret
