//! An interface to the [Linear Algebra PACKage][1].
//!
//! [1]: http://en.wikipedia.org/wiki/LAPACK

extern crate libc;
extern crate liblapack_sys as raw;

pub mod metal;
