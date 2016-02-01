use libc::c_char;
use std::mem::transmute;

use {c32, c64};

pub type Select2F32 = Option<extern "C" fn(*const f32, *const f32) -> i32>;
pub type Select3F32 = Option<extern "C" fn(*const f32, *const f32, *const f32) -> i32>;

pub type Select2F64 = Option<extern "C" fn(*const f64, *const f64) -> i32>;
pub type Select3F64 = Option<extern "C" fn(*const f64, *const f64, *const f64) -> i32>;

pub type Select1C32 = Option<extern "C" fn(*const c32) -> i32>;
pub type Select2C32 = Option<extern "C" fn(*const c32, *const c32) -> i32>;

pub type Select1C64 = Option<extern "C" fn(*const c64) -> i32>;
pub type Select2C64 = Option<extern "C" fn(*const c64, *const c64) -> i32>;
