TARGET := riscv64gc-unknown-none-elf
MODE := release
KERNEL_ELF := ./target/$(TARGET)/$(MODE)/os
KERNEL_BIN := $(KERNEL_ELF).bin
BOOTLOADER := ./bootloader/rustsbi-qemu.bin
export CPU_NUMS = 1
export LOG = INFO
USER_SRC := ./user/lib
USER_PATH := $(USER_SRC)/target/$(TARGET)/$(MODE)/

all: user
	@cd ./os && cargo build --release
	@rust-objcopy --binary-architecture=riscv64 $(KERNEL_ELF) \
		--strip-all \
		-O binary $(KERNEL_BIN)

test:
	@cross test --target riscv64gc-unknown-linux-gnu

user:
	@rm -rf $(USER_PATH)/deps
	@cd $(USER_SRC) && python build.py

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

.PHONY: user