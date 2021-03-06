use crate::bindings;
use crate::traits::*;
use crate::{Error, Result};

pub(crate) trait FutharkType {
    type RustType: Default;
    const DIM: usize;
    
    unsafe fn shape<C>(ctx: C, ptr: *const Self) -> *mut i64
    where
        C: Into<*mut bindings::futhark_context>;
    unsafe fn values<C>(ctx: C, ptr: *mut Self, dst: *mut Self::RustType)
    where
        C: Into<*mut bindings::futhark_context>;
    unsafe fn free<C>(ctx: C, ptr: *mut Self)
    where
        C: Into<*mut bindings::futhark_context>;
}
