pub mod encoder;
pub use self::encoder::Encoder;

pub mod video;
pub use self::video::Encoder as Video;

pub mod audio;
pub use self::audio::Encoder as Audio;

pub mod subtitle;
pub use self::subtitle::Encoder as Subtitle;

pub mod motion_estimation;
pub use self::motion_estimation::MotionEstimation;

#[cfg(not(feature = "ffmpeg_5_0"))]
pub mod prediction;
#[cfg(not(feature = "ffmpeg_5_0"))]
pub use self::prediction::Prediction;

pub mod comparison;
pub use self::comparison::Comparison;

pub mod decision;
pub use self::decision::Decision;

use std::ffi::CString;

use crate::codec::Context;
use crate::codec::Id;
use crate::ffi::*;
use crate::Codec;

pub fn new() -> Encoder {
    Context::new().encoder()
}

pub fn find(id: Id) -> Option<Codec> {
    unsafe {
        let ptr = avcodec_find_encoder(id.into());
        Codec::from_raw(ptr)
    }
}

pub fn find_by_name(name: &str) -> Option<Codec> {
    unsafe {
        let name = CString::new(name).unwrap();
        let ptr = avcodec_find_encoder_by_name(name.as_ptr());
        Codec::from_raw(ptr)
    }
}
