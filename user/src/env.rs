use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use lazy_static::*;
use spin::Mutex;
use share::syscall::error::SysError;
use core::ops::Add;
use core::str::from_utf8;
use alloc::vec::Vec;
use share::util::cvt_c_like_str_ptr_to_rust;

pub fn setenv(name: &str, value: &str, overwrite: bool) {
    ENV.lock().insert(name, value, overwrite);
}

pub fn unsetenv(name: &str) {
    ENV.lock().delete(name);
}

pub fn getenv<'a>(name: &str) -> Option<String> {
    ENV.lock().get(name)
}

pub fn cvt_c_like() -> Vec<usize> {
    ENV.lock().cvt_c_like()
}

lazy_static! {
    static ref ENV: Mutex<EnvironVariable> = Mutex::new(EnvironVariable::new());
}

pub struct EnvironVariable {
    store: BTreeMap<String, Pair>,
}

impl EnvironVariable {
    pub fn new() -> Self {
        Self {
            store: BTreeMap::new(),
        }
    }

    pub fn insert(&mut self, k: &str, v: &str, overwrite: bool) {
        let key = String::from(k);
        let pair = Pair::new(k, v);

        if overwrite || !self.store.contains_key(&key) {
            self.store.insert(key, pair);
        }
    }

    pub fn get(&self, k: &str) -> Option<String> {
        let key = String::from(k);

        self.store.get(&key).map(|pair: &Pair| {
            String::from(pair.value())
        })
    }

    pub fn delete(&mut self, key: &str) {
        let mut key = String::from(key);

        self.store.remove(&key);
    }

    pub fn cvt_c_like(&self) -> Vec<usize> {
        let mut v = Vec::new();
        for (_, pair) in self.store.iter() {
            v.push(pair.0.as_ptr() as usize);
        }
        v.push(0);

        v
    }

    pub fn from_c_like(&mut self, envs: &[usize]) {
        for &str_ptr in envs {
            if str_ptr == 0 {
                break;
            }

            let pair = Pair::from_c_like_ptr(str_ptr);
            let key = String::from(pair.key());
            self.store.insert(key, pair);
        }
    }
}

pub struct Pair(String);

impl Pair {
    pub fn new(key: &str, value: &str) -> Self {
        let mut string = String::from(key);
        string.push('=');
        string.push_str(value);
        string.push('\0');
        Self {
            0: string
        }
    }

    pub fn from_c_like_ptr(str_ptr: usize) -> Self {
        let str = cvt_c_like_str_ptr_to_rust(str_ptr);
        let mut string = String::from(str);
        string.push('\0');
        Self {
            0: string,
        }
    }

    pub fn key(&self) -> &str{
        let index = self.index_of_equal();
        let (key, _) = self.0.as_bytes().split_at(index);
        from_utf8(key).unwrap()
    }

    pub fn value(&self) -> &str{
        let index = self.index_of_equal();
        let (_, value) = self.0.as_bytes().split_at(index + 1);
        from_utf8(&value[..value.len()-1]).unwrap()
    }

    fn index_of_equal(&self) -> usize {
        let (index, _) = self.0.as_bytes().iter().enumerate().find(|(_, byte)| {
            **byte == '=' as u8
        }).unwrap();
        index
    }
}