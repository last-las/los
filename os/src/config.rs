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
pub const UART_BASE_ADDRESS: usize = 0x1000_0000;
pub const VIRTIO0_START_ADDRESS: usize = 0x1000_1000;
