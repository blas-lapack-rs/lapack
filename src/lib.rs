//! Interface to the [Linear Algebra PACKage][1].
//!
//! [1]: http://en.wikipedia.org/wiki/LAPACK

extern crate lapack_sys;
extern crate libc;
extern crate num_complex as num;

/// A complex number with 32-bit parts.
#[allow(non_camel_case_types)]
pub type c32 = num::Complex<f32>;

/// A complex number with 64-bit parts.
#[allow(non_camel_case_types)]
pub type c64 = num::Complex<f64>;

#[cfg(not(feature = "accelerate"))]
pub mod c;

pub mod fortran;
