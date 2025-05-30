use crate::ffi::*;
use libc::c_int;

bitflags::bitflags! {
    pub struct Flags: c_int {
        const FORCED = AV_SUBTITLE_FLAG_FORCED;
    }
}
