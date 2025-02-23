#![no_std]
#![feature(inline_const_pat, never_type)]
#![allow(uncommon_codepoints)]

pub mod gy521;
pub mod math;
pub mod esc;
pub mod mem;
pub mod pid;

pub extern crate alloc;
pub use alloc::*;
