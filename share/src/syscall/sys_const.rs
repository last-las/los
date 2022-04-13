pub const SYSCALL_READ: usize = 63;
pub const _SYSCALL_READ: usize = KCALL_MASK | SYSCALL_READ;
pub const SYSCALL_WRITE: usize = 64;
pub const _SYSCALL_WRITE: usize = KCALL_MASK | SYSCALL_WRITE;
pub const SYSCALL_EXIT: usize = 93;
pub const SYSCALL_YIELD: usize = 124;
pub const SYSCALL_GET_PRIORITY: usize = 140;
pub const SYSCALL_SET_PRIORITY: usize = 141;
pub const SYSCALL_GET_TIME: usize = 169;
pub const SYSCALL_GETPID: usize = 172;
pub const SYSCALL_GETPPID: usize = 173;
pub const SYSCALL_BRK: usize = 214;
pub const SYSCALL_FORK: usize = 220;
pub const SYSCALL_EXEC: usize = 221;
pub const SYSCALL_WAITPID: usize = 260;
pub const SYSCALL_TEST: usize = 1234;

pub const DEBUG_FRAME_USAGE: usize = 1001;

pub const KCALL_MASK: usize = 0x1000;
pub const KCALL_SEND: usize = KCALL_MASK | 1;
pub const KCALL_RECEIVE: usize = KCALL_MASK | 2;
pub const KCALL_READ_DEV: usize = KCALL_MASK | 3;
pub const KCALL_WRITE_DEV: usize = KCALL_MASK | 4;
pub const KCALL_VIRT_COPY: usize = KCALL_MASK | 5;
