TARGET := riscv64gc-unknown-none-elf
MODE := release
KERNEL_ELF := ./target/$(TARGET)/$(MODE)/os
KERNEL_BIN := $(KERNEL_ELF).bin
BOOTLOADER := ./bootloader/rustsbi-qemu.bin
export CPU_NUMS = 2
export LOG = INFO
USER_PATH := ./user/target/$(TARGET)/$(MODE)/
FS_IMG := $(USER_PATH)/fs.img
OTHER_PATH := /home/las/workstation/testsuits-for-oskernel-main/riscv-syscalls-testing/user/build/riscv64/
SDCARD := /dev/sdb

# board and bootloader
BOARD ?= qemu
SBI ?= rustsbi
BOOTLOADER := ./bootloader/$(SBI)-$(BOARD).bin
BOOTLOADER_SIZE = 131072 # 0x20000

# kernel entry
ifeq ($(BOARD), qemu)
	KERNEL_ENTRY := 0x80200000
else ifeq ($(BOARD), k210)
	KERNEL_ENTRY := 0x80020000
endif

# Run k210
K210_SERIALPORT := /dev/ttyUSB0
k210_BURNER := ./tools/kflash.py


all: switch-check fs-img
	@echo Platform: $(BOARD)
	@cp os/link-$(BOARD).ld os/link.ld
	@cd ./os && cargo build --release --features "board_$(BOARD)"
	@rm os/link.ld
	@rust-objcopy --binary-architecture=riscv64 $(KERNEL_ELF) \
		--strip-all \
		-O binary $(KERNEL_BIN)

# dev/zero永远输出0
sdcard: fs-img
	@echo "Are you sure write to $(SDCARD) ? [y/N] " && read ans && [ $${ans:-N} = y ]
	@sudo dd if=/dev/zero of=$(SDCARD) bs=1048576 count=16
	@sudo dd if=$(FS_IMG) of=$(SDCARD)

test:
	@cross test --target riscv64gc-unknown-linux-gnu

switch-check:
ifeq ($(BOARD), qemu)
	@(which last-qemu) || (touch last-qemu && make clean)
else ifeq ($(BOARD), k210)
	@(which last-k210) || (touch last-k210 && make clean)
endif

user:
	@cd ./user && python build.py && cargo build --release --features "board_$(BOARD)"

fs-img: user
	# @cd ./easy-fs-fuse && cargo run --release -- -s ../user/lib/src/bin/ -t ../user/target/$(TARGET)/$(MODE)/ -o $(OTHER_PATH)
	@cd ./easy-fs-fuse && cargo run --release -- -s ../user/lib/src/bin/ -t ../user/target/$(TARGET)/$(MODE)/

run: fs-img
ifeq ($(BOARD),qemu)
	@qemu-system-riscv64 \
		-machine virt \
		-nographic \
		-bios $(BOOTLOADER) \
		-device loader,file=$(KERNEL_BIN),addr=0x80200000 \
		-smp $(CPU_NUMS) \
		-drive file=$(FS_IMG),if=none,format=raw,id=x0 \
		-device virtio-blk-device,drive=x0,bus=virtio-mmio-bus.0
else
	@cp $(BOOTLOADER) $(BOOTLOADER).copy
	dd if=$(KERNEL_BIN) of=$(BOOTLOADER).copy bs=$(BOOTLOADER_SIZE) seek=1
	@mv $(BOOTLOADER).copy $(KERNEL_BIN)
	@sudo chmod 777 $(K210_SERIALPORT)
	python3 ./tools/kflash.py -p $(K210_SERIALPORT) -b 1500000 $(KERNEL_BIN)
	python3 -m serial.tools.miniterm --eol LF --dtr 0 --rts 0 --filter direct $(K210_SERIALPORT) 115200
endif

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

clean:
	@cargo clean

.PHONY: all test switch-check user run debug clean
