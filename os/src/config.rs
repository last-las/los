pub const MAX_TASK_NUMBER: usize = 64;
pub const FRAME_SIZE: usize = 4096;
pub const KERNEL_MAPPING_OFFSET: usize = 0xFFFFFFC000000000;
// pub const KERNEL_MAPPING_OFFSET: usize = 0;
// pub const RAM_MAPPING_OFFSET: usize = 0x1000000000;
pub const RAM_MAPPING_OFFSET: usize = 0xFFFFFFD000000000;
pub const RAM_START_ADDRESS: usize = 0x80000000;
#[cfg(feature = "board_qemu")]
pub const RAM_SIZE: usize = 0x8_000_000;
#[cfg(feature = "board_k210")]
pub const RAM_SIZE: usize = 0x600_000;
pub const MAX_USER_ADDRESS: usize = 0x4_000_000_000;
pub const MMAP_START_ADDRESS: usize = 0x2_000_000_000;
#[cfg(feature = "board_qemu")]
pub const UART_BASE_ADDRESS: usize = 0x1000_0000;
#[cfg(feature = "board_k210")]
pub const UART_BASE_ADDRESS: usize = 0x3800_0000;
pub const VIRTIO0_START_ADDRESS: usize = 0x1000_1000;

// 外设地址
pub const RTC_BASE_ADDRESS: usize = 0x5046_0000;
pub const SYSCTL_ADDRESS: usize = 0x5044_0000;

pub const FPIOA_ADDRESS: usize = 0x502B_0000;
pub const GPIO_BASE_ADDR: usize = 0x50200000;
pub const GPIOHS_ADDRESS: usize = 0x3800_1000;
pub const DMAC_ADDRESS: usize = 0x5000_0000;
pub const SPI0_ADDRESS: usize = 0x5200_0000;
pub const SPI1_BASE_ADDR: usize = 0x53000000;
