//! Interface to the [Linear Algebra PACKage][lapack].
//!
//! ## Configuration
//!
//! The underlying implementation of LAPACK to compile, if needed, and link to
//! can be chosen among the following options:
//!
//! * Apple’s [Accelerate framework][accelerate] (macOS only),
//! * Netlib’s [reference implementation][netlib], and
//! * [OpenBLAS][openblas] (default).
//!
//! An implementation can be chosen using the package’s features as follows:
//!
//! ```toml
//! [dependencies]
//! # Apple’s Accelerate framework
//! lapack = { version = "0.14", default-features = false, features = ["accelerate"] }
//! # Netlib’s reference implementation
//! lapack = { version = "0.14", default-features = false, features = ["netlib"] }
//! # OpenBLAS
//! lapack = { version = "0.14", default-features = false, features = ["openblas"] }
//! # OpenBLAS
//! lapack = { version = "0.14" }
//! ```
//!
//! [accelerate]: https://developer.apple.com/reference/accelerate
//! [lapack]: https://en.wikipedia.org/wiki/LAPACK
//! [netlib]: http://www.netlib.org/lapack
//! [openblas]: http://www.openblas.net

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
