use user_lib::syscall::{dev_read, dev_write};
use core::fmt::{Debug, Formatter};

#[derive(Debug, Clone, Default)]
pub struct ReadOnly<T: Copy + Debug>(Volatile<T>);

impl<T: Copy + Debug> ReadOnly<T> {
    pub fn read(&self) -> T {
        self.0.read()
    }
}

#[derive(Debug, Clone, Default)]
pub struct WriteOnly<T: Copy + Debug>(Volatile<T>);

impl<T: Copy + Debug> WriteOnly<T> {
    pub fn write(&mut self, value: T) {
        self.0.write(value);
    }
}

#[derive(Clone, Default)]
pub struct Volatile<T: Copy + Debug>(T);

impl<T: Copy + Debug> Debug for Volatile<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!("{:#?}", self.read()))
    }
}

impl<T: Copy + Debug> Volatile<T> {
    #[allow(unused)]
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