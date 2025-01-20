//! Internal utils, not related to `avutil`

use std::path::Path;
use std::{
    ffi::{CStr, CString},
    str::from_utf8_unchecked,
};

/// `ptr` must be non-null and valid.
/// Ensure that the returned lifetime is correctly bounded.
#[inline]
pub unsafe fn str_from_c_ptr<'s>(ptr: *const libc::c_char) -> &'s str {
    unsafe { from_utf8_unchecked(CStr::from_ptr(ptr).to_bytes()) }
}

/// `ptr` must be null or valid.
/// Ensure that the returned lifetime is correctly bounded.
#[inline]
pub unsafe fn optional_str_from_c_ptr<'s>(ptr: *const libc::c_char) -> Option<&'s str> {
    if ptr.is_null() {
        None
    } else {
        Some(str_from_c_ptr(ptr))
    }
}

// XXX: use to_cstring when stable
pub fn from_path<P: AsRef<Path>>(path: P) -> CString {
    // 只接受固定大小类型 ，可以获取所有权
    CString::new(path.as_ref().to_str().unwrap()).unwrap()
}

#[allow(dead_code)]
pub fn from_path_dyn<P: AsRef<Path> + ?Sized>(path: &P) -> CString {
    // 可以接受不定大小类型，必须通过引用使用
    // &P 实现了 AsRef<Path>，可以直接传递给 from_path
    from_path(path)
}
