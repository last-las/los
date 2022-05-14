// filesystem syscall.
pub const SYSCALL_LSEEK: usize = 8;
pub const SYSCALL_GETCWD: usize = 17;
pub const SYSCALL_DUP: usize = 23;
pub const SYSCALL_DUP3: usize = 24;
pub const SYSCALL_MKDIRAT: usize = 34;
pub const SYSCALL_UNMOUNT: usize = 39;
pub const SYSCALL_MOUNT: usize = 40;
pub const SYSCALL_CHDIR: usize = 49;
pub const SYSCALL_OPEN: usize = 56;
pub const SYSCALL_CLOSE: usize = 57;
pub const SYSCALL_GETDENTS: usize = 61;
pub const SYSCALL_READ: usize = 63;
pub const SYSCALL_WRITE: usize = 64;
pub const SYSCALL_FSTAT: usize = 80;
pub const SYSCALL_UNLINK: usize = 83;
pub const SYSCALL_RMDIR: usize = 84;

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
pub const DEBUG_SCHEDULE_RECORD_ENABLE: usize = 1002;
pub const DEBUG_SCHEDULE_RECORD_PRINT: usize = 1003;

pub const KCALL_MASK: usize = 0x1000;
pub const KCALL_SEND: usize = KCALL_MASK | 1;
pub const KCALL_RECEIVE: usize = KCALL_MASK | 2;
pub const KCALL_READ_DEV: usize = KCALL_MASK | 3;
pub const KCALL_WRITE_DEV: usize = KCALL_MASK | 4;
pub const KCALL_VIRT_COPY: usize = KCALL_MASK | 5;
pub const KCALL_CONTINUOUS_ALLOC: usize = KCALL_MASK | 6;
pub const KCALL_VIRT_TO_PHYS: usize = KCALL_MASK | 7;
pub const KCALL_COPY_C_PATH: usize = KCALL_MASK | 8;
pub const KCALL_SBI_READ: usize = KCALL_MASK | 9;
pub const KCALL_TERMINAL_READ: usize = KCALL_MASK | 10;
pub const KCALL_SBI_WRITE: usize = KCALL_MASK | 11;
pub const KCALL_TERMINAL_WRITE: usize = KCALL_MASK | 12;
