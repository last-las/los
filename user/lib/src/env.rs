use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use lazy_static::*;
use spin::Mutex;
use share::syscall::error::{SysError, EINVAL};
use core::ops::Add;
use core::str::from_utf8;
use alloc::vec::Vec;
use share::util::cvt_c_like_str_ptr_to_rust;
use share::ffi::{CStrArray, c_char, CString, CStr};

pub fn setenv(name: &str, value: &str, overwrite: bool) {
    ENV.lock().insert(name, value, overwrite);
}

pub fn unsetenv(name: &str) {
    ENV.lock().delete(name);
}

pub fn getenv<'a>(name: &str) -> Option<String> {
    ENV.lock().get(name)
}

pub fn get_args() -> Vec<String> {
    ARGS.lock().clone()
}

/// Create a collection of all environment variables' pointers, end with zero.
/// Normally this function is called when sys_exec is about to be invoked.
pub fn get_envp_copy() -> CStrArray {
    ENV.lock().get_envp_copy()
}

/// Read environment variables on the stack when the process is just created,
/// and copy them to the heap for better management.
pub fn parse_envp(envp: *const *const c_char) {
    let c_array_envp = CStrArray::copy_from_ptr(envp);
    ENV.lock().parse_envp(c_array_envp);
}

/// Read arguments on the stack when the process is just created,
/// and copy them to the heap for better management.
pub fn parse_argv(argv: *const *const c_char) {
    let c_array_argv = CStrArray::copy_from_ptr(argv);
    for arg_ptr in c_array_argv.iter() {
        let cstr = CStr::from_ptr(arg_ptr);
        ARGS.lock().push(cstr.into());
    }
}

lazy_static! {
    static ref ENV: Mutex<EnvironVariable> = Mutex::new(EnvironVariable::new());
    static ref ARGS: Mutex<Vec<String>> = Mutex::new(Vec::new());
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

    pub fn insert(&mut self, k: &str, v: &str, overwrite: bool) -> Result<(), SysError> {
        let key = String::from(k);
        let pair = Pair::new(k, v);

        if overwrite || !self.store.contains_key(&key) {
            self.store.insert(key, pair);
        }

        Ok(())
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

    pub fn get_envp_copy(&self) -> CStrArray {
        let mut v = Vec::new();
        for (_, pair) in self.store.iter() {
            v.push(pair.cstring.as_ptr());
        }

        CStrArray::from_vec(v)
    }

    pub fn parse_envp(&mut self, envs: CStrArray) -> Result<(), SysError> {
        for str_ptr in envs.iter() {
            let pair = Pair::from(str_ptr)?;
            let key = String::from(pair.key());
            self.store.insert(key, pair);
        }

        Ok(())
    }
}

pub struct Pair {
    cstring: CString,
    equal_index: usize,
}

impl Pair {
    pub fn new(key: &str, value: &str) -> Self {
        let mut string = String::from(key);
        let equal_index = string.len();
        string.push('=');
        string.push_str(value);

        Self {
            cstring: CString::new(string),
            equal_index,
        }
    }

    pub fn from(ptr:* const c_char) -> Result<Self, SysError> {
        let cstr = unsafe { CStr::from_ptr(ptr) };
        let cstring = unsafe {CString::from(cstr) };
        assert_ne!(cstring.as_ptr() as usize, ptr as usize);

        let result = cstring.as_bytes().iter().enumerate().find(|(_, byte)| {
            **byte == '=' as u8
        });
        if result.is_none() {
            return Err(SysError::new(EINVAL));
        }
        let equal_index = result.unwrap().0;

        Ok(Self {cstring, equal_index})
    }

    pub fn key(&self) -> &str{
        let (key, _) = self.cstring.as_bytes().split_at(self.equal_index);
        from_utf8(key).unwrap()
    }

    pub fn value(&self) -> &str{
        let (_, value) = self.cstring.as_bytes().split_at(self.equal_index + 1);
        from_utf8(&value[..value.len()]).unwrap()
    }
}