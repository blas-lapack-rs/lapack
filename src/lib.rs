//! An interface to the [Linear Algebra PACKage][1] including the [Basic Linear
//! Algebra Subprograms][2].
//!
//! [1]: http://www.netlib.org/lapack/
//! [2]: http://www.netlib.org/blas/

#![feature(phase)]

extern crate "liblapack-sys" as raw;

pub use blas::dgemm;
pub use blas::dgemv;
pub use lapack::dsyev;

mod blas;
mod lapack;

#[cfg(test)]
mod test {
    #[phase(plugin)] extern crate assert;
}
