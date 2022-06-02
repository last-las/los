TARGET := riscv64gc-unknown-none-elf
MODE := release
KERNEL_ELF := ./target/$(TARGET)/$(MODE)/os
KERNEL_BIN := $(KERNEL_ELF).bin
BOOTLOADER := ./bootloader/rustsbi-qemu.bin
export CPU_NUMS = 2
export LOG = INFO
USER_PATH := ./user/target/$(TARGET)/$(MODE)/
FS_IMG := $(USER_PATH)fs.img
OTHER_PATH := /home/las/workstation/testsuits-for-oskernel-main/riscv-syscalls-testing/user/build/riscv64/

all: user fs-img
	@cd ./os && cargo build --release
	@rust-objcopy --binary-architecture=riscv64 $(KERNEL_ELF) \
		--strip-all \
		-O binary $(KERNEL_BIN)

test:
	@cross test --target riscv64gc-unknown-linux-gnu

user:
	@cd ./user && python build.py && cargo build --release

fs-img:
	# @cd ./fat32-fuse && cargo run --release -- -s ../user/lib/src/bin/ -t ../user/target/$(TARGET)/$(MODE)/ -o $(OTHER_PATH)
	@dd if=/dev/zero of=$(USER_PATH)fs.img bs=512KB count=256 #k210 128MB
	@mkfs.vfat -F 32 $(USER_PATH)fs.img
	@cd ./fat32-fuse && cargo run --release -- -s ../user/lib/src/bin/ -t ../user/target/$(TARGET)/$(MODE)/
	#cd ./fat32-fuse && sh qemu_fs.sh

run:
	@qemu-system-riscv64 \
		-machine virt \
		-nographic \
		-bios $(BOOTLOADER) \
		-device loader,file=$(KERNEL_BIN),addr=0x80200000 \
		-smp $(CPU_NUMS) \
		-drive file=$(FS_IMG),if=none,format=raw,id=x0 \
		-device virtio-blk-device,drive=x0,bus=virtio-mmio-bus.0

debug:
	@echo "you should run command below in another terminal(same path):"
	@echo "riscv64-unknown-elf-gdb $(KERNEL_ELF)"
	@qemu-system-riscv64 \
	 	-machine virt \
	 	-nographic \
 		-bios $(BOOTLOADER) \
 		-device loader,file=$(KERNEL_BIN),addr=0x80200000 \
 		-s -S \
 		-smp $(CPU_NUMS) \
		-drive file=$(FS_IMG),if=none,format=raw,id=x0 \
		-device virtio-blk-device,drive=x0,bus=virtio-mmio-bus.0

.PHONY: user
