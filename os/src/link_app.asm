
    .align 3
    .section .data
    .global _num_app
_num_app:
    .quad 24
 .quad app_0_start
 .quad app_1_start
 .quad app_2_start
 .quad app_3_start
 .quad app_4_start
 .quad app_5_start
 .quad app_6_start
 .quad app_7_start
 .quad app_8_start
 .quad app_9_start
 .quad app_10_start
 .quad app_11_start
 .quad app_12_start
 .quad app_13_start
 .quad app_14_start
 .quad app_15_start
 .quad app_16_start
 .quad app_17_start
 .quad app_18_start
 .quad app_19_start
 .quad app_20_start
 .quad app_21_start
 .quad app_22_start
 .quad app_23_start
 .quad app_23_end

    .global _app_names
_app_names:
     .string "00hello_world"
     .string "01power3"
     .string "02power5"
     .string "03sleep"
     .string "04ipc"
     .string "05brk"
     .string "06heap"
     .string "07fork"
     .string "08exec"
     .string "09waitpid1"
     .string "09waitpid2"
     .string "10env"
     .string "10env_child"
     .string "11getpid"
     .string "12setpriority"
     .string "13speed"
     .string "idle"
     .string "init"
     .string "rtc"
     .string "shell"
     .string "terminal"
     .string "test_terminal"
     .string "test_virtio"
     .string "virtio-blk"

        .align 3
        .section .data
        .global app_0_start
        .global app_0_end
    app_0_start:
        .incbin "./user/target/riscv64gc-unknown-none-elf/release/00hello_world"
    app_0_end:

        .align 3
        .section .data
        .global app_1_start
        .global app_1_end
    app_1_start:
        .incbin "./user/target/riscv64gc-unknown-none-elf/release/01power3"
    app_1_end:

        .align 3
        .section .data
        .global app_2_start
        .global app_2_end
    app_2_start:
        .incbin "./user/target/riscv64gc-unknown-none-elf/release/02power5"
    app_2_end:

        .align 3
        .section .data
        .global app_3_start
        .global app_3_end
    app_3_start:
        .incbin "./user/target/riscv64gc-unknown-none-elf/release/03sleep"
    app_3_end:

        .align 3
        .section .data
        .global app_4_start
        .global app_4_end
    app_4_start:
        .incbin "./user/target/riscv64gc-unknown-none-elf/release/04ipc"
    app_4_end:

        .align 3
        .section .data
        .global app_5_start
        .global app_5_end
    app_5_start:
        .incbin "./user/target/riscv64gc-unknown-none-elf/release/05brk"
    app_5_end:

        .align 3
        .section .data
        .global app_6_start
        .global app_6_end
    app_6_start:
        .incbin "./user/target/riscv64gc-unknown-none-elf/release/06heap"
    app_6_end:

        .align 3
        .section .data
        .global app_7_start
        .global app_7_end
    app_7_start:
        .incbin "./user/target/riscv64gc-unknown-none-elf/release/07fork"
    app_7_end:

        .align 3
        .section .data
        .global app_8_start
        .global app_8_end
    app_8_start:
        .incbin "./user/target/riscv64gc-unknown-none-elf/release/08exec"
    app_8_end:

        .align 3
        .section .data
        .global app_9_start
        .global app_9_end
    app_9_start:
        .incbin "./user/target/riscv64gc-unknown-none-elf/release/09waitpid1"
    app_9_end:

        .align 3
        .section .data
        .global app_10_start
        .global app_10_end
    app_10_start:
        .incbin "./user/target/riscv64gc-unknown-none-elf/release/09waitpid2"
    app_10_end:

        .align 3
        .section .data
        .global app_11_start
        .global app_11_end
    app_11_start:
        .incbin "./user/target/riscv64gc-unknown-none-elf/release/10env"
    app_11_end:

        .align 3
        .section .data
        .global app_12_start
        .global app_12_end
    app_12_start:
        .incbin "./user/target/riscv64gc-unknown-none-elf/release/10env_child"
    app_12_end:

        .align 3
        .section .data
        .global app_13_start
        .global app_13_end
    app_13_start:
        .incbin "./user/target/riscv64gc-unknown-none-elf/release/11getpid"
    app_13_end:

        .align 3
        .section .data
        .global app_14_start
        .global app_14_end
    app_14_start:
        .incbin "./user/target/riscv64gc-unknown-none-elf/release/12setpriority"
    app_14_end:

        .align 3
        .section .data
        .global app_15_start
        .global app_15_end
    app_15_start:
        .incbin "./user/target/riscv64gc-unknown-none-elf/release/13speed"
    app_15_end:

        .align 3
        .section .data
        .global app_16_start
        .global app_16_end
    app_16_start:
        .incbin "./user/target/riscv64gc-unknown-none-elf/release/idle"
    app_16_end:

        .align 3
        .section .data
        .global app_17_start
        .global app_17_end
    app_17_start:
        .incbin "./user/target/riscv64gc-unknown-none-elf/release/init"
    app_17_end:

        .align 3
        .section .data
        .global app_18_start
        .global app_18_end
    app_18_start:
        .incbin "./user/target/riscv64gc-unknown-none-elf/release/rtc"
    app_18_end:

        .align 3
        .section .data
        .global app_19_start
        .global app_19_end
    app_19_start:
        .incbin "./user/target/riscv64gc-unknown-none-elf/release/shell"
    app_19_end:

        .align 3
        .section .data
        .global app_20_start
        .global app_20_end
    app_20_start:
        .incbin "./user/target/riscv64gc-unknown-none-elf/release/terminal"
    app_20_end:

        .align 3
        .section .data
        .global app_21_start
        .global app_21_end
    app_21_start:
        .incbin "./user/target/riscv64gc-unknown-none-elf/release/test_terminal"
    app_21_end:

        .align 3
        .section .data
        .global app_22_start
        .global app_22_end
    app_22_start:
        .incbin "./user/target/riscv64gc-unknown-none-elf/release/test_virtio"
    app_22_end:

        .align 3
        .section .data
        .global app_23_start
        .global app_23_end
    app_23_start:
        .incbin "./user/target/riscv64gc-unknown-none-elf/release/virtio-blk"
    app_23_end:
