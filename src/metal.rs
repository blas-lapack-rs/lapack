//! Minimalistic wrappers for LAPACK routines.

use libc::{c_char, c_int};
use raw;

pub enum Jobu {
    /// All m columns of u are returned in array u.
    A = b'A' as isize,
    /// The first columns of u (the left singular vectors) are returned in array u.
    S = b'S' as isize,
    /// The first columns of u (the left singular vectors) are overwritten to array a.
    O = b'O' as isize,
    /// No columns of u (the left singular vectors) are computed.
    N = b'N' as isize,
}

pub enum Jobvt {
    /// All n rows of vt are returned in array vt.
    A = b'A' as isize,
    /// The first rows of vt (the right singular vectors) are returned in array vt.
    S = b'S' as isize,
    /// The first rows of vt (the right singular vectors) are overwritten to array a.
    O = b'O' as isize,
    /// No rows of vt (the right singular vectors) are computed.
    N = b'N' as isize,
}

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

#[inline]
pub fn dgesvd(jobu: Jobu, jobvt: Jobvt, m: usize, n: usize, a: &mut [f64], lda: usize,
              s: &mut [f64], u: &mut [f64], ldu: usize, vt: &mut [f64], ldvt: usize,
              work: &mut [f64], lwork: usize, info: &mut isize) {

    unsafe {
        raw::dgesvd_(&(jobu as c_char) as *const _ as *mut _,
                     &(jobvt as c_char) as *const _ as *mut _,
                     &(m as c_int) as *const _ as *mut _,
                     &(n as c_int) as *const _ as *mut _,
                     a.as_mut_ptr(),
                     &(lda as c_int) as *const _ as *mut _,
                     s.as_mut_ptr(),
                     u.as_mut_ptr(),
                     &(ldu as c_int) as *const _ as *mut _,
                     vt.as_mut_ptr(),
                     &(ldvt as c_int) as *const _ as *mut _,
                     work.as_mut_ptr(),
                     &(lwork as c_int) as *const _ as *mut _,
                     info as *mut _ as *mut _);
    }
}
