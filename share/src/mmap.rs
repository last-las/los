bitflags! {
    pub struct Prot: u32 {
        const READ = 0x1;
        const WRITE = 0x2;
        const EXEC = 0x4;
    }
}

bitflags! {
    pub struct MMAPFlags: u32 {
        const FILE = 0x0;
        const SHARED = 0x1;
        const PRIVATE = 0x2;
        const ANONYMOUS = 0x10;
    }
}