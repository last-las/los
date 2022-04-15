use user_lib::syscall::{dev_read, dev_write};

#[derive(Debug, Clone, Default)]
pub struct ReadOnly<T: Copy>(Volatile<T>);

impl<T: Copy> ReadOnly<T> {
    pub fn read(&self) -> T {
        self.0.read()
    }
}

#[derive(Debug, Clone, Default)]
pub struct WriteOnly<T: Copy>(Volatile<T>);

impl<T: Copy> WriteOnly<T> {
    pub fn write(&mut self, value: T) {
        self.0.write(value);
    }
}

#[derive(Debug, Clone, Default)]
pub struct Volatile<T: Copy>(T);

impl<T: Copy> Volatile<T> {
    pub fn new(value: T) -> Volatile<T>{
        Volatile(value)
    }

    pub fn read(&self) -> T {
        let size = core::mem::size_of::<T>();
        assert!(size <= core::mem::size_of::<usize>());
        let ret = dev_read(&self.0 as *const _ as usize, size).unwrap();
        unsafe {
            (&ret as *const _ as usize as *const T).read_volatile()
        }
    }

    pub fn write(&mut self, value: T) {
        let size = core::mem::size_of::<T>();
        let value =unsafe {
            match size {
                1 => {
                    (&value as *const _ as usize as *const u8).read_volatile() as usize
                },
                2 => {
                    (&value as *const _ as usize as *const u16).read_volatile() as usize
                },
                4 => {
                    (&value as *const _ as usize as *const u32).read_volatile() as usize
                },
                8 => {
                    (&value as *const _ as usize as *const u64).read_volatile() as usize
                },
                _ => panic!("Wrong size:{}", size),
            }
        };
        dev_write(&mut self.0 as *mut _ as usize, value, size).unwrap();
    }
}