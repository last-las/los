use alloc::string::String;
use crate::ffi::CStr;
bitflags! {
    pub struct OpenFlag: u32 {
        const RDONLY = 0x0; // attention: RDONLY is zero, which means open_flag.contains(OpenFlag::RDONLY) is always true!
        const WRONLY = 0x1;
        const RDWR = 0x2;
        const CREAT = 0x40;
        const EXCL = 0x80;
        const TRUNC = 0x200;
        const APPEND = 0x400;
        const DIRECTORY = 0x10000;
    }
}

/// linux style Dirent.
#[repr(C)]
pub struct Dirent {
    /// Inode number.
    pub d_ino: u64,
    /// The distance from the start of the directory to the start of the next `Dirent`.
    ///
    /// (I don't understand the difference between `d_offset` and `d_reclen`. Right now it's value is always zero )
    pub d_offset: u64,
    /// The size of this entire `Dirent`.
    pub d_reclen: u16,
    /// File type.
    pub d_type: FileTypeFlag,
    /// Filename.
    pub d_name: *const u8,
}

/// Rust version Dirent.
pub struct RDirent {
    pub ino: u64,
    pub _type: FileTypeFlag,
    pub name: String,
}

impl From<Dirent> for RDirent {
    fn from(dirent: Dirent) -> Self {
        let cstr = CStr::from_ptr(dirent.d_name);
        let name = String::from(cstr.as_str());
        Self {
            ino: dirent.d_ino,
            _type: dirent.d_type,
            name,
        }
    }
}

bitflags! {
    pub struct FileTypeFlag: u8 {
        const DT_UNKNOWN = 0x0;
        const DT_FIFO = 0x1;
        const DT_CHR = 0x2;
        const DT_DIR = 0x4;
        const DT_BLK = 0x6;
        const DT_REG = 0x8;
        const DT_LNK = 0xa;
    }
}

impl FileTypeFlag {
    pub fn is_device(&self) -> bool {
        self.contains(FileTypeFlag::DT_BLK) ||
            self.contains(FileTypeFlag::DT_CHR)
    }
}

bitflags! {
    pub struct SEEKFlag: u32 {
        /// The file offset is set to offset bytes.
        const SET = 0x0;
        /// The file offset is set to its current location plus offset bytes.
        const CUR = 0x1;
        /// The file offset is set to the size of the file plus offset bytes.
        const END = 0x2;
    }
}

// TODO-FUTURE: update field size.
#[repr(C)]
pub struct Stat {
    pub dev: usize,
    pub ino: usize,
    pub mode: usize,
    pub nlink: usize,
    pub uid: usize,
    pub gid: usize,
    pub rdev: usize,
    // __pad
    pub size: usize,
    pub blksize: usize,
    // sec stuff...
}

impl Stat {
    pub fn empty() -> Self {
        Self {
            dev: 0,
            ino: 0,
            mode: 0,
            nlink: 0,
            uid: 0,
            gid: 0,
            rdev: 0,
            size: 0,
            blksize: 0,
        }
    }
}

pub const MAX_PATH_LENGTH: usize = 64;
pub const AT_FD_CWD: isize = -100;
pub const DIRENT_BUFFER_SZ: usize = 2048;

// for device driver
pub const RAM_MAJOR: u32 = 0;
pub const SDCARD_MAJOR: u32 = 1;
pub const VIRT_BLK_MAJOR: u32 = 1;
pub const CONSOLE_MAJOR: u32 = 3;