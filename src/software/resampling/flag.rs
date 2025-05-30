use crate::ffi::*;
use libc::c_int;

bitflags::bitflags! {
    pub struct Flags: c_int {
        const FORCE = SWR_FLAG_RESAMPLE;
    }
}
