//! Interface to the [Linear Algebra PACKage][1].
//!
//! [1]: http://en.wikipedia.org/wiki/LAPACK

extern crate lapack_sys as ffi;
extern crate libc;

use libc::{c_char, c_int};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Jobu {
    A = b'A' as isize,
    S = b'S' as isize,
    O = b'O' as isize,
    N = b'N' as isize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Jobvt {
    A = b'A' as isize,
    S = b'S' as isize,
    O = b'O' as isize,
    N = b'N' as isize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Jobz {
    N = b'N' as isize,
    V = b'V' as isize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Uplo {
    U = b'U' as isize,
    L = b'L' as isize,
}

#[inline]
pub fn dsyev(jobz: Jobz, uplo: Uplo, n: usize, a: &mut [f64], lda: usize, w: &mut [f64],
             work: &mut [f64], lwork: usize, info: &mut isize) {

    unsafe {
        ffi::dsyev_(&(jobz as c_char), &(uplo as c_char), &(n as c_int), a.as_mut_ptr(),
                    &(lda as c_int), w.as_mut_ptr(), work.as_mut_ptr(), &(lwork as c_int),
                    info as *mut _ as *mut _);
    }
}

#[inline]
pub fn dgesvd(jobu: Jobu, jobvt: Jobvt, m: usize, n: usize, a: &mut [f64], lda: usize,
              s: &mut [f64], u: &mut [f64], ldu: usize, vt: &mut [f64], ldvt: usize,
              work: &mut [f64], lwork: usize, info: &mut isize) {

    unsafe {
        ffi::dgesvd_(&(jobu as c_char), &(jobvt as c_char), &(m as c_int), &(n as c_int),
                     a.as_mut_ptr(), &(lda as c_int), s.as_mut_ptr(), u.as_mut_ptr(),
                     &(ldu as c_int), vt.as_mut_ptr(), &(ldvt as c_int), work.as_mut_ptr(),
                     &(lwork as c_int), info as *mut _ as *mut _);
    }
}

#[inline]
pub fn dgetrf(m: usize, n: usize, a: &mut [f64], lda: usize, ipiv: &mut [i32], info: &mut isize) {
    unsafe {
        ffi::dgetrf_(&(m as c_int), &(n as c_int), a.as_mut_ptr(), &(lda as c_int),
                     ipiv.as_mut_ptr(), info as *mut _ as *mut _);
    }
}

#[inline]
pub fn dgetri(n: usize, a: &mut [f64], lda: usize, ipiv: &mut [i32], work: &mut [f64],
              lwork: usize, info: &mut isize) {

    unsafe {
        ffi::dgetri_(&(n as c_int), a.as_mut_ptr(), &(lda as c_int), ipiv.as_mut_ptr(),
                     work.as_mut_ptr(), &(lwork as c_int), info as *mut _ as *mut _);
    }
}
