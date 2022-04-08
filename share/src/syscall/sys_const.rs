pub const SYSCALL_READ: usize = 63;
pub const SYSCALL_WRITE: usize = 64;
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

pub const KCALL_SEND: usize = 2001;
pub const KCALL_RECEIVE: usize = 2002;
pub const KCALL_READ_DEV: usize = 2003;
pub const KCALL_WRITE_DEV: usize = 2004;
