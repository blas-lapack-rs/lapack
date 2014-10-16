//! The library provides an interface to the [Linear Algebra PACKage][1].
//!
//! [1]: http://www.netlib.org/lapack/

extern crate libc;

use libc::{c_char, c_double, c_int};

#[link(name = "gfortran")]
#[link(name = "lapack", kind = "static")]
extern {
    fn dsyev_(jobz: *const c_char, uplo: *const c_char, n: *const c_int,
              a: *mut c_double, lda: *const c_int, w: *mut c_double,
              work: *mut c_double, lwork: *const c_int, info: *mut c_int);
}

/// Computes the eigenvalues and, optionally, the left and/or right
/// eigenvectors for symmetric matrices.
///
/// http://www.netlib.org/lapack/explore-html/dd/d4c/dsyev_8f.html
#[inline]
pub fn dsyev(jobz: u8, uplo: u8, n: uint, a: *mut f64, lda: uint, w: *mut f64,
             work: *mut f64, lwork: uint, info: *mut int) {

    unsafe {
        dsyev_(&(jobz as i8), &(uplo as i8), &(n as i32), a, &(lda as i32), w,
               work, &(lwork as i32), info as *mut i32);
    }
}
