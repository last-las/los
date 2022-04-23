bitflags! {
    pub struct OpenFlag: u32 {
        const RDONLY = 0x0;
        const WRONLY = 0x1;
        const RDWR = 0x2;
        const CREAT = 0x40;
        const EXCL = 0x80;
        const TRUNC = 0x200;
        const APPEND = 0x400;
        const DIRECTORY = 0x10000;
    }
}

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