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