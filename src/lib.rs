//! An interface to the [Linear Algebra PACKage][1].
//!
//! [1]: http://en.wikipedia.org/wiki/LAPACK

extern crate libc;
extern crate liblapack_sys as raw;

use libc::{c_char, c_int};

pub enum Jobz {
    /// Compute eigenvalues only.
    N = b'N' as isize,
    /// Compute eigenvalues and eigenvectors.
    V = b'V' as isize,
}

pub enum Uplo {
    /// Upper triangles are stored.
    U = b'U' as isize,
    /// Lower triangles are stored.
    L = b'L' as isize,
}

#[inline]
pub fn dsyev(jobz: Jobz, uplo: Uplo, n: usize, a: &mut [f64], lda: usize, w: &mut [f64],
             work: &mut [f64], lwork: usize, info: &mut isize) {

    unsafe {
        raw::dsyev_(&(jobz as c_char) as *const _ as *mut _,
                    &(uplo as c_char) as *const _ as *mut _,
                    &(n as c_int) as *const _ as *mut _,
                    a.as_mut_ptr(),
                    &(lda as c_int) as *const _ as *mut _,
                    w.as_mut_ptr(),
                    work.as_mut_ptr(),
                    &(lwork as c_int) as *const _ as *mut _,
                    info as *mut _ as *mut _);
    }
}
