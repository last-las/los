use alloc::vec::Vec;
use alloc::string::String;
use core::fmt::{Debug, Formatter};

/// a simplified version of the std::ffi::CString which is not available for no_std right now.
pub struct CString {
    inner: String
}

impl CString {
    pub fn new(mut string: String) -> Self {
        string.push('\0');
        Self {
            inner: string
        }
    }

    pub fn as_ptr(&self) -> * const u8 {
        self.inner.as_ptr()
    }

    pub fn as_bytes(&self) -> &[u8] {
        let len = self.inner.len();
        &self.inner.as_bytes()[0..len-1]
    }

    pub fn as_bytes_with_nul(&self) -> &[u8] {
        self.inner.as_bytes()
    }
}

impl From<&str> for CString {
    fn from(s: &str) -> Self {
        let string = String::from(s);
        Self::new(string)
    }
}

impl Debug for CString {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!("{}", self.inner))
    }
}

impl<'a> From<CStr<'a>> for CString {
    fn from(cstr: CStr) -> Self {
        let str = core::str::from_utf8(cstr.inner).unwrap();
        Self {
            inner: String::from(str),
        }
    }
}

/// a simplified version of the std::ffi::CStr which is not available for no_std right now.
pub struct CStr<'a> {
    inner: &'a [u8]
}

impl<'a> CStr<'a> {
    pub fn from_ptr(ptr: *const u8) -> Self {
        let len = unsafe { strlen(ptr) };
        let inner = unsafe {
            core::slice::from_raw_parts(ptr, len + 1) // include '\0'
        };

        Self {
            inner,
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        let len = self.inner.len();
        &self.inner[0..len-1]
    }

    pub fn as_bytes_with_nul(&self) -> &[u8] {
        self.inner
    }

    pub fn as_str(&self) -> &str {
        core::str::from_utf8(self.as_bytes()).unwrap()
    }
}

impl<'a> Into<String> for CStr<'a> {
    fn into(self) -> String {
        let len = self.inner.len();
        let str = core::str::from_utf8(&self.inner[..len-1]).unwrap();
        String::from(str)
    }
}

impl<'a> Debug for CStr<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!("{}", self.as_str()))
    }
}

/// `CStrArray` is used to describe structure like `char*` in C.
pub struct CStrArray {
    inner: Vec<*const u8>
}

impl CStrArray {
    pub fn copy_from_ptr(ptr: *const *const u8) -> CStrArray {
        let count = unsafe { strlen(ptr) };
        let c_str_array_slice = unsafe {
            core::slice::from_raw_parts(ptr, count + 1)
        };
        let inner = Vec::from(c_str_array_slice);
        Self {
            inner
        }
    }

    pub fn from_vec(mut v: Vec<*const u8>) -> CStrArray {
        v.push(0 as *const u8);
        Self {
            inner: v,
        }
    }

    pub fn as_ptr(&self) -> *const *const u8 {
        self.inner.as_ptr()
    }

    pub fn iter(&self) -> CStrArrayIter {
        CStrArrayIter::new(self.inner.as_ref())
    }
}

pub struct CStrArrayIter<'a> {
    inner_ref: &'a [*const u8],
    current: usize,
}

impl <'a> CStrArrayIter<'a> {
    pub fn new(inner_ref: &'a [*const u8]) -> Self {
        Self {
            inner_ref,
            current: 0,
        }
    }
}

impl <'a> Iterator for CStrArrayIter<'a> {
    type Item = *const u8;

    fn next(&mut self) -> Option<Self::Item> {
        let cur = self.current;
        if self.inner_ref[cur] as usize != 0 {
            self.current += 1;
            Some(self.inner_ref[cur])
        } else {
            None
        }
    }
}

unsafe fn strlen<T>(start: *const T) -> usize {
    let mut n = 0;
    while !is_null(start.offset(n as isize)) {
        n += 1;
    }
    n
}

unsafe fn is_null<T>(ptr:*const T) -> bool {
    let size = core::mem::size_of::<T>();
    let p = ptr as usize as *const u8;
    for i in 0..size {
        if p.offset(i as isize).read() != 0 {
            return false;
        }
    }
    return true;
}
