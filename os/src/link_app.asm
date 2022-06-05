
    .align 3
    .section .data
    .global _num_app
_num_app:
    .quad 4
 .quad app_0_start
 .quad app_1_start
 .quad app_2_start
 .quad app_3_start
 .quad app_3_end

    .global _app_names
_app_names:
     .string "fs"
     .string "init"
     .string "terminal"
     .string "virtio-blk"

        .section .data
        .global app_0_start
        .global app_0_end
    app_0_start:
        .incbin "./user/target/riscv64gc-unknown-none-elf/release/fs"
    app_0_end:

        .section .data
        .global app_1_start
        .global app_1_end
    app_1_start:
        .incbin "./user/target/riscv64gc-unknown-none-elf/release/init"
    app_1_end:

        .section .data
        .global app_2_start
        .global app_2_end
    app_2_start:
        .incbin "./user/target/riscv64gc-unknown-none-elf/release/terminal"
    app_2_end:

        .section .data
        .global app_3_start
        .global app_3_end
    app_3_start:
        .incbin "./user/target/riscv64gc-unknown-none-elf/release/virtio-blk"
    app_3_end:
