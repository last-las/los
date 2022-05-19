use alloc::rc::Rc;
use alloc::string::String;

pub struct SuperBlock {}

pub struct Inode {}

impl Inode {
    pub fn lookup() -> Rc<Dentry> {
        unimplemented!()
    }
}

pub struct File {}

impl File {
    pub fn new() -> Self {
        unimplemented!()
    }

    pub fn read(&self) {
        unimplemented!()
    }
}

pub struct Dentry {
    pub inode: Rc<Inode>,
    pub name: String,
}

pub struct VFSmount {}

pub struct NameIData {
    pub dentry: Rc<Dentry>,
    pub mnt: Rc<VFSmount>,
}