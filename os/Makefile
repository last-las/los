TARGET := riscv64gc-unknown-none-elf
MODE := release
KERNEL_ELF := target/$(TARGET)/$(MODE)/os
KERNEL_BIN := $(KERNEL_ELF).bin
BOOTLOADER := ../bootloader/rustsbi-qemu.bin
# BOOTLOADER := ../bootloader/fw_payload.bin
export CPU_NUMS = 1
export LOG = INFO
# USER_PATH := ../user/target/$(TARGET)/$(MODE)/
# APPLICATION := $(foreach app,$(shell ls ../user/src/bin/),$(patsubst %.rs, %, $(app)))

all: user
	@cargo build --release
	@rust-objcopy --binary-architecture=riscv64 $(KERNEL_ELF) \
		--strip-all \
		-O binary $(KERNEL_BIN)

test: user
	@cargo build --features test --release
	@rust-objcopy --binary-architecture=riscv64 $(KERNEL_ELF) \
		--strip-all \
		-O binary $(KERNEL_BIN)

user:
	@cd ../user && make build

run:
	@qemu-system-riscv64 \
		-machine virt \
		-nographic \
		-bios $(BOOTLOADER) \
		-device loader,file=$(KERNEL_BIN),addr=0x80200000 \
		-smp $(CPU_NUMS)

debug:
	@echo "you should run command below in another terminal(same path):"
	@echo "riscv64-unknown-elf-gdb $(KERNEL_ELF)"
	@qemu-system-riscv64 \
	 	-machine virt \
	 	-nographic \
 		-bios $(BOOTLOADER) \
 		-device loader,file=$(KERNEL_BIN),addr=0x80200000 \
 		-s -S