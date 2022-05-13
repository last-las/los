TARGET := riscv64gc-unknown-none-elf
MODE := release
KERNEL_ELF := ./target/$(TARGET)/$(MODE)/os
KERNEL_BIN := $(KERNEL_ELF).bin
export CPU_NUMS = 1
export LOG = INFO
USER_PATH := ./user/target/$(TARGET)/$(MODE)/

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


all: switch-check user
	@echo Platform: $(BOARD)
	@cp os/link-$(BOARD).ld os/link.ld
	@cd ./os && cargo build --release --features "board_$(BOARD)"
	@rm os/link.ld
	@rust-objcopy --binary-architecture=riscv64 $(KERNEL_ELF) \
		--strip-all \
		-O binary $(KERNEL_BIN)

test:
	@cross test --target riscv64gc-unknown-linux-gnu

switch-check:
ifeq ($(BOARD), qemu)
	@(which last-qemu) || (rm last-k210 -f && touch last-qemu && make clean)
else ifeq ($(BOARD), k210)
	@(which last-k210) || (rm last-qemu -f && touch last-k210 && make clean)
endif

user:
	@rm -rf $(USER_PATH)/deps
	@cd ./user && python build.py && cargo build --release

run:
ifeq ($(BOARD),qemu)
	@qemu-system-riscv64 \
		-machine virt \
		-nographic \
		-bios $(BOOTLOADER) \
		-device loader,file=$(KERNEL_BIN),addr=0x80200000 \
		-smp $(CPU_NUMS)
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
 		-s -S

clean:
	@cargo clean

.PHONY: all test switch-check user run debug clean