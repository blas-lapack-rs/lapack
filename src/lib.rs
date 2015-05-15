//! An interface to the [Linear Algebra PACKage][1].
//!
//! [1]: http://en.wikipedia.org/wiki/LAPACK

extern crate libc;
extern crate liblapack_sys as raw;

use libc::{c_char, c_int};

#[derive(Clone, Copy)]
pub enum Jobu {
    A = b'A' as isize,
    S = b'S' as isize,
    O = b'O' as isize,
    N = b'N' as isize,
}

#[derive(Clone, Copy)]
pub enum Jobvt {
    A = b'A' as isize,
    S = b'S' as isize,
    O = b'O' as isize,
    N = b'N' as isize,
}

#[derive(Clone, Copy)]
pub enum Jobz {
    N = b'N' as isize,
    V = b'V' as isize,
}

#[derive(Clone, Copy)]
pub enum Uplo {
    U = b'U' as isize,
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

#[inline]
pub fn dgetrf(m: usize, n: usize, a: &mut [f64], lda: usize, ipiv: &mut [i32], info: &mut isize) {
    unsafe {
        raw::dgetrf_(&(m as c_int) as *const _ as *mut _,
                     &(n as c_int) as *const _ as *mut _,
                     a.as_mut_ptr(),
                     &(lda as c_int) as *const _ as *mut _,
                     ipiv.as_mut_ptr(),
                     info as *mut _ as *mut _);
    }
}

#[inline]
pub fn dgetri(n: usize, a: &mut [f64], lda: usize, ipiv: &mut [i32], work: &mut [f64],
              lwork: usize, info: &mut isize) {

    unsafe {
        raw::dgetri_(&(n as c_int) as *const _ as *mut _,
                     a.as_mut_ptr(),
                     &(lda as c_int) as *const _ as *mut _,
                     ipiv.as_mut_ptr(),
                     work.as_mut_ptr(),
                     &(lwork as c_int) as *const _ as *mut _,
                     info as *mut _ as *mut _);
    }
}
