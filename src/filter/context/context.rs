use std::marker::PhantomData;

use super::{Sink, Source};
use crate::ffi::*;
use crate::{format, option, AsMutPtr, AsPtr};

#[cfg(feature = "ffmpeg_5_1")]
use crate::ChannelLayout;

#[cfg(not(feature = "ffmpeg_7_0"))]
use crate::ChannelLayoutMask;

pub struct Context<'a> {
    ptr: *mut AVFilterContext,

    _marker: PhantomData<&'a ()>,
}

impl<'a> Context<'a> {
    pub unsafe fn wrap(ptr: *mut AVFilterContext) -> Self {
        Context {
            ptr,
            _marker: PhantomData,
        }
    }

    pub unsafe fn as_ptr(&self) -> *const AVFilterContext {
        self.ptr as *const _
    }

    pub unsafe fn as_mut_ptr(&mut self) -> *mut AVFilterContext {
        self.ptr
    }
}

impl<'a> Context<'a> {
    pub fn source(&'a mut self) -> Source<'a> {
        unsafe { Source::wrap(self) }
    }

    pub fn sink(&'a mut self) -> Sink<'a> {
        unsafe { Sink::wrap(self) }
    }

    pub fn set_pixel_format(&mut self, value: format::Pixel) {
        let _ = option::Settable::set::<AVPixelFormat>(self, "pix_fmts", &value.into());
    }

    pub fn set_sample_format(&mut self, value: format::Sample) {
        let _ = option::Settable::set::<AVSampleFormat>(self, "sample_fmts", &value.into());
    }

    pub fn set_sample_rate(&mut self, value: u32) {
        let _ = option::Settable::set(self, "sample_rates", &i64::from(value));
    }

    #[cfg(not(feature = "ffmpeg_7_0"))]
    pub fn set_channel_layout(&mut self, value: ChannelLayoutMask) {
        let _ = option::Settable::set(self, "channel_layouts", &value.bits());
    }

    #[cfg(feature = "ffmpeg_5_1")]
    pub fn set_ch_layout(&mut self, value: ChannelLayout) {
        let _ = option::Settable::set_str(self, "channel_layouts", &value.description());
    }
}

impl<'a> AsPtr<AVFilterContext> for Context<'a> {
    fn as_ptr(&self) -> *const AVFilterContext {
        self.ptr as *const _
    }
}

impl<'a> AsMutPtr<AVFilterContext> for Context<'a> {
    fn as_mut_ptr(&mut self) -> *mut AVFilterContext {
        self.ptr as *mut _
    }
}

impl<'a> option::Settable<AVFilterContext> for Context<'a> {}
