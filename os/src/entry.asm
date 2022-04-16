.section .text.entry
.globl _start
# a0 = hartid
_start:
    addi t0, a0, 1
    slli t0, t0, 16         # t0 = 4096 * 16 * (hartid + 1)
    lla t1, boot_stack
    add sp, t1, t0

    bgtz a0, end_clear_bss  # only hart0 invokes clear_bss
    lla t0, __bss_start
    lla t1, __bss_end
clear_bss:
    beq t0, t1, end_clear_bss
    sd zero, 0(t0)
    addi t0, t0, 8
    j clear_bss
end_clear_bss:

    # call enable_paging
    call enable_paging

.section .bss.stack
.globl boot_stack
boot_stack:
    .space 4096 * 16 * 2