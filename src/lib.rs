extern crate libc;

use libc::{c_char, c_double, c_int};

#[link(name = "gfortran")]
#[link(name = "lapack", kind = "static")]
extern {
    fn dsyev_(jobz: *const c_char, uplo: *const c_char, n: *const c_int,
        a: *const c_double, lda: *const c_int, w: *mut c_double,
        work: *mut c_double, lwork: *const c_int, info: *mut c_int);
}

/// Computes the eigenvalues and, optionally, the left and/or right
/// eigenvectors for symmetric matrices.
///
/// http://www.netlib.org/lapack/explore-html/dd/d4c/dsyev_8f.html
#[inline]
pub fn dsyev(jobz: i8, uplo: i8, n: i32, a: *const f64, lda: i32, w: *mut f64,
    work: *mut f64, lwork: i32, info: *mut i32) {

    unsafe {
        dsyev_(&jobz, &uplo, &n, a, &lda, w, work, &lwork, info);
    }
}
