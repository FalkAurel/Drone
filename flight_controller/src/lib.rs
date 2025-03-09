#![no_std]
#![feature(inline_const_pat, never_type, allocator_api, stmt_expr_attributes, pointer_is_aligned_to)]
#![allow(uncommon_codepoints)]

pub mod gy521;
pub mod math;
pub mod esc;
pub mod mem;
pub mod sync;

#[cfg(feature = "wifi")]
pub mod wifi;

pub extern crate alloc;
pub use alloc::*;
