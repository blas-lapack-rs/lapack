//! The Fortran interface.
//!
//! ## Example
//!
//! ```
//! use lapack::fortran::*;
//!
//! let n = 3;
//! let mut a = vec![3.0, 1.0, 1.0, 1.0, 3.0, 1.0, 1.0, 1.0, 3.0];
//! let mut w = vec![0.0; n as usize];
//! let mut work = vec![0.0; 4 * n as usize];
//! let lwork = 4 * n;
//! let mut info = 0;
//!
//! dsyev(b'V', b'U', n, &mut a, n, &mut w, &mut work, lwork, &mut info);
//!
//! for (one, another) in w.iter().zip(&[2.0, 2.0, 5.0]) {
//!     assert!((one - another).abs() < 1e-14);
//! }
//! ```

use lapack_sys::fortran as ffi;

include!("common.rs");

#[inline]
pub fn sgetrf(m: i32, n: i32, a: &mut [f32], lda: i32, ipiv: &mut [i32], info: &mut i32) {
    unsafe {
        ffi::sgetrf_(&m, &n, a.as_mut_ptr(), &lda, ipiv.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dgetrf(m: i32, n: i32, a: &mut [f64], lda: i32, ipiv: &mut [i32], info: &mut i32) {
    unsafe {
        ffi::dgetrf_(&m, &n, a.as_mut_ptr(), &lda, ipiv.as_mut_ptr(), info)
    }
}

#[inline]
pub fn cgetrf(m: i32, n: i32, a: &mut [c32], lda: i32, ipiv: &mut [i32], info: &mut i32) {
    unsafe {
        ffi::cgetrf_(&m, &n, a.as_mut_ptr() as *mut _, &lda, ipiv.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zgetrf(m: i32, n: i32, a: &mut [c64], lda: i32, ipiv: &mut [i32], info: &mut i32) {
    unsafe {
        ffi::zgetrf_(&m, &n, a.as_mut_ptr() as *mut _, &lda, ipiv.as_mut_ptr(), info)
    }
}

#[inline]
pub fn sgetrf2(m: i32, n: i32, a: &mut [f32], lda: i32, ipiv: &mut [i32], info: &mut i32) {
    unsafe {
        ffi::sgetrf2_(&m, &n, a.as_mut_ptr(), &lda, ipiv.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dgetrf2(m: i32, n: i32, a: &mut [f64], lda: i32, ipiv: &mut [i32], info: &mut i32) {
    unsafe {
        ffi::dgetrf2_(&m, &n, a.as_mut_ptr(), &lda, ipiv.as_mut_ptr(), info)
    }
}

#[inline]
pub fn cgetrf2(m: i32, n: i32, a: &mut [c32], lda: i32, ipiv: &mut [i32], info: &mut i32) {
    unsafe {
        ffi::cgetrf2_(&m, &n, a.as_mut_ptr() as *mut _, &lda, ipiv.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zgetrf2(m: i32, n: i32, a: &mut [c64], lda: i32, ipiv: &mut [i32], info: &mut i32) {
    unsafe {
        ffi::zgetrf2_(&m, &n, a.as_mut_ptr() as *mut _, &lda, ipiv.as_mut_ptr(), info)
    }
}

#[inline]
pub fn sgbtrf(m: i32, n: i32, kl: i32, ku: i32, ab: &mut [f32], ldab: i32, ipiv: &mut [i32],
              info: &mut i32) {

    unsafe {
        ffi::sgbtrf_(&m, &n, &kl, &ku, ab.as_mut_ptr(), &ldab, ipiv.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dgbtrf(m: i32, n: i32, kl: i32, ku: i32, ab: &mut [f64], ldab: i32, ipiv: &mut [i32],
              info: &mut i32) {

    unsafe {
        ffi::dgbtrf_(&m, &n, &kl, &ku, ab.as_mut_ptr(), &ldab, ipiv.as_mut_ptr(), info)
    }
}

#[inline]
pub fn cgbtrf(m: i32, n: i32, kl: i32, ku: i32, ab: &mut [c32], ldab: i32, ipiv: &mut [i32],
              info: &mut i32) {

    unsafe {
        ffi::cgbtrf_(&m, &n, &kl, &ku, ab.as_mut_ptr() as *mut _, &ldab, ipiv.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zgbtrf(m: i32, n: i32, kl: i32, ku: i32, ab: &mut [c64], ldab: i32, ipiv: &mut [i32],
              info: &mut i32) {

    unsafe {
        ffi::zgbtrf_(&m, &n, &kl, &ku, ab.as_mut_ptr() as *mut _, &ldab, ipiv.as_mut_ptr(), info)
    }
}

#[inline]
pub fn sgttrf(n: i32, dl: &mut [f32], d: &mut [f32], du: &mut [f32], du2: &mut [f32],
              ipiv: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::sgttrf_(&n, dl.as_mut_ptr(), d.as_mut_ptr(), du.as_mut_ptr(), du2.as_mut_ptr(),
                     ipiv.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dgttrf(n: i32, dl: &mut [f64], d: &mut [f64], du: &mut [f64], du2: &mut [f64],
              ipiv: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::dgttrf_(&n, dl.as_mut_ptr(), d.as_mut_ptr(), du.as_mut_ptr(), du2.as_mut_ptr(),
                     ipiv.as_mut_ptr(), info)
    }
}

#[inline]
pub fn cgttrf(n: i32, dl: &mut [c32], d: &mut [c32], du: &mut [c32], du2: &mut [c32],
              ipiv: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::cgttrf_(&n, dl.as_mut_ptr() as *mut _, d.as_mut_ptr() as *mut _,
                     du.as_mut_ptr() as *mut _, du2.as_mut_ptr() as *mut _, ipiv.as_mut_ptr(),
                     info)
    }
}

#[inline]
pub fn zgttrf(n: i32, dl: &mut [c64], d: &mut [c64], du: &mut [c64], du2: &mut [c64],
              ipiv: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::zgttrf_(&n, dl.as_mut_ptr() as *mut _, d.as_mut_ptr() as *mut _,
                     du.as_mut_ptr() as *mut _, du2.as_mut_ptr() as *mut _, ipiv.as_mut_ptr(),
                     info)
    }
}

#[inline]
pub fn spotrf2(uplo: u8, n: i32, a: &mut [f32], lda: i32, info: &mut i32) {
    unsafe {
        ffi::spotrf2_(&(uplo as c_char), &n, a.as_mut_ptr(), &lda, info)
    }
}

#[inline]
pub fn dpotrf2(uplo: u8, n: i32, a: &mut [f64], lda: i32, info: &mut i32) {
    unsafe {
        ffi::dpotrf2_(&(uplo as c_char), &n, a.as_mut_ptr(), &lda, info)
    }
}

#[inline]
pub fn cpotrf2(uplo: u8, n: i32, a: &mut [c32], lda: i32, info: &mut i32) {
    unsafe {
        ffi::cpotrf2_(&(uplo as c_char), &n, a.as_mut_ptr() as *mut _, &lda, info)
    }
}

#[inline]
pub fn zpotrf2(uplo: u8, n: i32, a: &mut [c64], lda: i32, info: &mut i32) {
    unsafe {
        ffi::zpotrf2_(&(uplo as c_char), &n, a.as_mut_ptr() as *mut _, &lda, info)
    }
}

#[inline]
pub fn spotrf(uplo: u8, n: i32, a: &mut [f32], lda: i32, info: &mut i32) {
    unsafe {
        ffi::spotrf_(&(uplo as c_char), &n, a.as_mut_ptr(), &lda, info)
    }
}

#[inline]
pub fn dpotrf(uplo: u8, n: i32, a: &mut [f64], lda: i32, info: &mut i32) {
    unsafe {
        ffi::dpotrf_(&(uplo as c_char), &n, a.as_mut_ptr(), &lda, info)
    }
}

#[inline]
pub fn cpotrf(uplo: u8, n: i32, a: &mut [c32], lda: i32, info: &mut i32) {
    unsafe {
        ffi::cpotrf_(&(uplo as c_char), &n, a.as_mut_ptr() as *mut _, &lda, info)
    }
}

#[inline]
pub fn zpotrf(uplo: u8, n: i32, a: &mut [c64], lda: i32, info: &mut i32) {
    unsafe {
        ffi::zpotrf_(&(uplo as c_char), &n, a.as_mut_ptr() as *mut _, &lda, info)
    }
}

#[inline]
pub fn dpstrf(uplo: u8, n: i32, a: &mut [f64], lda: i32, piv: &mut [i32], rank: &mut i32, tol: f64,
              work: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::dpstrf_(&(uplo as c_char), &n, a.as_mut_ptr(), &lda, piv.as_mut_ptr(), rank, &tol,
                     work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn spstrf(uplo: u8, n: i32, a: &mut [f32], lda: i32, piv: &mut [i32], rank: &mut i32, tol: f32,
              work: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::spstrf_(&(uplo as c_char), &n, a.as_mut_ptr(), &lda, piv.as_mut_ptr(), rank, &tol,
                     work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zpstrf(uplo: u8, n: i32, a: &mut [c64], lda: i32, piv: &mut [i32], rank: &mut i32, tol: f64,
              work: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::zpstrf_(&(uplo as c_char), &n, a.as_mut_ptr() as *mut _, &lda, piv.as_mut_ptr(), rank,
                     &tol, work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn cpstrf(uplo: u8, n: i32, a: &mut [c32], lda: i32, piv: &mut [i32], rank: &mut i32, tol: f32,
              work: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::cpstrf_(&(uplo as c_char), &n, a.as_mut_ptr() as *mut _, &lda, piv.as_mut_ptr(), rank,
                     &tol, work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dpftrf(transr: u8, uplo: u8, n: i32, a: &mut [f64], info: &mut i32) {
    unsafe {
        ffi::dpftrf_(&(transr as c_char), &(uplo as c_char), &n, a.as_mut_ptr(), info)
    }
}

#[inline]
pub fn spftrf(transr: u8, uplo: u8, n: i32, a: &mut [f32], info: &mut i32) {
    unsafe {
        ffi::spftrf_(&(transr as c_char), &(uplo as c_char), &n, a.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zpftrf(transr: u8, uplo: u8, n: i32, a: &mut [c64], info: &mut i32) {
    unsafe {
        ffi::zpftrf_(&(transr as c_char), &(uplo as c_char), &n, a.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn cpftrf(transr: u8, uplo: u8, n: i32, a: &mut [c32], info: &mut i32) {
    unsafe {
        ffi::cpftrf_(&(transr as c_char), &(uplo as c_char), &n, a.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn spptrf(uplo: u8, n: i32, ap: &mut [f32], info: &mut i32) {
    unsafe {
        ffi::spptrf_(&(uplo as c_char), &n, ap.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dpptrf(uplo: u8, n: i32, ap: &mut [f64], info: &mut i32) {
    unsafe {
        ffi::dpptrf_(&(uplo as c_char), &n, ap.as_mut_ptr(), info)
    }
}

#[inline]
pub fn cpptrf(uplo: u8, n: i32, ap: &mut [c32], info: &mut i32) {
    unsafe {
        ffi::cpptrf_(&(uplo as c_char), &n, ap.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn zpptrf(uplo: u8, n: i32, ap: &mut [c64], info: &mut i32) {
    unsafe {
        ffi::zpptrf_(&(uplo as c_char), &n, ap.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn spbtrf(uplo: u8, n: i32, kd: i32, ab: &mut [f32], ldab: i32, info: &mut i32) {
    unsafe {
        ffi::spbtrf_(&(uplo as c_char), &n, &kd, ab.as_mut_ptr(), &ldab, info)
    }
}

#[inline]
pub fn dpbtrf(uplo: u8, n: i32, kd: i32, ab: &mut [f64], ldab: i32, info: &mut i32) {
    unsafe {
        ffi::dpbtrf_(&(uplo as c_char), &n, &kd, ab.as_mut_ptr(), &ldab, info)
    }
}

#[inline]
pub fn cpbtrf(uplo: u8, n: i32, kd: i32, ab: &mut [c32], ldab: i32, info: &mut i32) {
    unsafe {
        ffi::cpbtrf_(&(uplo as c_char), &n, &kd, ab.as_mut_ptr() as *mut _, &ldab, info)
    }
}

#[inline]
pub fn zpbtrf(uplo: u8, n: i32, kd: i32, ab: &mut [c64], ldab: i32, info: &mut i32) {
    unsafe {
        ffi::zpbtrf_(&(uplo as c_char), &n, &kd, ab.as_mut_ptr() as *mut _, &ldab, info)
    }
}

#[inline]
pub fn spttrf(n: i32, d: &mut [f32], e: &mut [f32], info: &mut i32) {
    unsafe {
        ffi::spttrf_(&n, d.as_mut_ptr(), e.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dpttrf(n: i32, d: &mut [f64], e: &mut [f64], info: &mut i32) {
    unsafe {
        ffi::dpttrf_(&n, d.as_mut_ptr(), e.as_mut_ptr(), info)
    }
}

#[inline]
pub fn cpttrf(n: i32, d: &mut [f32], e: &mut [c32], info: &mut i32) {
    unsafe {
        ffi::cpttrf_(&n, d.as_mut_ptr(), e.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn zpttrf(n: i32, d: &mut [f64], e: &mut [c64], info: &mut i32) {
    unsafe {
        ffi::zpttrf_(&n, d.as_mut_ptr(), e.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn ssytrf(uplo: u8, n: i32, a: &mut [f32], lda: i32, ipiv: &mut [i32], work: &mut [f32],
              lwork: i32, info: &mut i32) {

    unsafe {
        ffi::ssytrf_(&(uplo as c_char), &n, a.as_mut_ptr(), &lda, ipiv.as_mut_ptr(),
                     work.as_mut_ptr(), &lwork, info)
    }
}

#[inline]
pub fn dsytrf(uplo: u8, n: i32, a: &mut [f64], lda: i32, ipiv: &mut [i32], work: &mut [f64],
              lwork: i32, info: &mut i32) {

    unsafe {
        ffi::dsytrf_(&(uplo as c_char), &n, a.as_mut_ptr(), &lda, ipiv.as_mut_ptr(),
                     work.as_mut_ptr(), &lwork, info)
    }
}

#[inline]
pub fn csytrf(uplo: u8, n: i32, a: &mut [c32], lda: i32, ipiv: &mut [i32], work: &mut [c32],
              lwork: i32, info: &mut i32) {

    unsafe {
        ffi::csytrf_(&(uplo as c_char), &n, a.as_mut_ptr() as *mut _, &lda, ipiv.as_mut_ptr(),
                     work.as_mut_ptr() as *mut _, &lwork, info)
    }
}

#[inline]
pub fn zsytrf(uplo: u8, n: i32, a: &mut [c64], lda: i32, ipiv: &mut [i32], work: &mut [c64],
              lwork: i32, info: &mut i32) {

    unsafe {
        ffi::zsytrf_(&(uplo as c_char), &n, a.as_mut_ptr() as *mut _, &lda, ipiv.as_mut_ptr(),
                     work.as_mut_ptr() as *mut _, &lwork, info)
    }
}

#[inline]
pub fn chetrf(uplo: u8, n: i32, a: &mut [c32], lda: i32, ipiv: &mut [i32], work: &mut [c32],
              lwork: i32, info: &mut i32) {

    unsafe {
        ffi::chetrf_(&(uplo as c_char), &n, a.as_mut_ptr() as *mut _, &lda, ipiv.as_mut_ptr(),
                     work.as_mut_ptr() as *mut _, &lwork, info)
    }
}

#[inline]
pub fn zhetrf(uplo: u8, n: i32, a: &mut [c64], lda: i32, ipiv: &mut [i32], work: &mut [c64],
              lwork: i32, info: &mut i32) {

    unsafe {
        ffi::zhetrf_(&(uplo as c_char), &n, a.as_mut_ptr() as *mut _, &lda, ipiv.as_mut_ptr(),
                     work.as_mut_ptr() as *mut _, &lwork, info)
    }
}

#[inline]
pub fn ssptrf(uplo: u8, n: i32, ap: &mut [f32], ipiv: &mut [i32], info: &mut i32) {
    unsafe {
        ffi::ssptrf_(&(uplo as c_char), &n, ap.as_mut_ptr(), ipiv.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dsptrf(uplo: u8, n: i32, ap: &mut [f64], ipiv: &mut [i32], info: &mut i32) {
    unsafe {
        ffi::dsptrf_(&(uplo as c_char), &n, ap.as_mut_ptr(), ipiv.as_mut_ptr(), info)
    }
}

#[inline]
pub fn csptrf(uplo: u8, n: i32, ap: &mut [c32], ipiv: &mut [i32], info: &mut i32) {
    unsafe {
        ffi::csptrf_(&(uplo as c_char), &n, ap.as_mut_ptr() as *mut _, ipiv.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zsptrf(uplo: u8, n: i32, ap: &mut [c64], ipiv: &mut [i32], info: &mut i32) {
    unsafe {
        ffi::zsptrf_(&(uplo as c_char), &n, ap.as_mut_ptr() as *mut _, ipiv.as_mut_ptr(), info)
    }
}

#[inline]
pub fn chptrf(uplo: u8, n: i32, ap: &mut [c32], ipiv: &mut [i32], info: &mut i32) {
    unsafe {
        ffi::chptrf_(&(uplo as c_char), &n, ap.as_mut_ptr() as *mut _, ipiv.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zhptrf(uplo: u8, n: i32, ap: &mut [c64], ipiv: &mut [i32], info: &mut i32) {
    unsafe {
        ffi::zhptrf_(&(uplo as c_char), &n, ap.as_mut_ptr() as *mut _, ipiv.as_mut_ptr(), info)
    }
}

#[inline]
pub fn sgetrs(trans: u8, n: i32, nrhs: i32, a: &[f32], lda: i32, ipiv: &[i32], b: &mut [f32],
              ldb: i32, info: &mut i32) {

    unsafe {
        ffi::sgetrs_(&(trans as c_char), &n, &nrhs, a.as_ptr(), &lda, ipiv.as_ptr(),
                     b.as_mut_ptr(), &ldb, info)
    }
}

#[inline]
pub fn dgetrs(trans: u8, n: i32, nrhs: i32, a: &[f64], lda: i32, ipiv: &[i32], b: &mut [f64],
              ldb: i32, info: &mut i32) {

    unsafe {
        ffi::dgetrs_(&(trans as c_char), &n, &nrhs, a.as_ptr(), &lda, ipiv.as_ptr(),
                     b.as_mut_ptr(), &ldb, info)
    }
}

#[inline]
pub fn cgetrs(trans: u8, n: i32, nrhs: i32, a: &[c32], lda: i32, ipiv: &[i32], b: &mut [c32],
              ldb: i32, info: &mut i32) {

    unsafe {
        ffi::cgetrs_(&(trans as c_char), &n, &nrhs, a.as_ptr() as *const _, &lda, ipiv.as_ptr(),
                     b.as_mut_ptr() as *mut _, &ldb, info)
    }
}

#[inline]
pub fn zgetrs(trans: u8, n: i32, nrhs: i32, a: &[c64], lda: i32, ipiv: &[i32], b: &mut [c64],
              ldb: i32, info: &mut i32) {

    unsafe {
        ffi::zgetrs_(&(trans as c_char), &n, &nrhs, a.as_ptr() as *const _, &lda, ipiv.as_ptr(),
                     b.as_mut_ptr() as *mut _, &ldb, info)
    }
}

#[inline]
pub fn sgbtrs(trans: u8, n: i32, kl: i32, ku: i32, nrhs: i32, ab: &[f32], ldab: i32, ipiv: &[i32],
              b: &mut [f32], ldb: i32, info: &mut i32) {

    unsafe {
        ffi::sgbtrs_(&(trans as c_char), &n, &kl, &ku, &nrhs, ab.as_ptr(), &ldab, ipiv.as_ptr(),
                     b.as_mut_ptr(), &ldb, info)
    }
}

#[inline]
pub fn dgbtrs(trans: u8, n: i32, kl: i32, ku: i32, nrhs: i32, ab: &[f64], ldab: i32, ipiv: &[i32],
              b: &mut [f64], ldb: i32, info: &mut i32) {

    unsafe {
        ffi::dgbtrs_(&(trans as c_char), &n, &kl, &ku, &nrhs, ab.as_ptr(), &ldab, ipiv.as_ptr(),
                     b.as_mut_ptr(), &ldb, info)
    }
}

#[inline]
pub fn cgbtrs(trans: u8, n: i32, kl: i32, ku: i32, nrhs: i32, ab: &[c32], ldab: i32, ipiv: &[i32],
              b: &mut [c32], ldb: i32, info: &mut i32) {

    unsafe {
        ffi::cgbtrs_(&(trans as c_char), &n, &kl, &ku, &nrhs, ab.as_ptr() as *const _, &ldab,
                     ipiv.as_ptr(), b.as_mut_ptr() as *mut _, &ldb, info)
    }
}

#[inline]
pub fn zgbtrs(trans: u8, n: i32, kl: i32, ku: i32, nrhs: i32, ab: &[c64], ldab: i32, ipiv: &[i32],
              b: &mut [c64], ldb: i32, info: &mut i32) {

    unsafe {
        ffi::zgbtrs_(&(trans as c_char), &n, &kl, &ku, &nrhs, ab.as_ptr() as *const _, &ldab,
                     ipiv.as_ptr(), b.as_mut_ptr() as *mut _, &ldb, info)
    }
}

#[inline]
pub fn sgttrs(trans: u8, n: i32, nrhs: i32, dl: &[f32], d: &[f32], du: &[f32], du2: &[f32],
              ipiv: &[i32], b: &mut [f32], ldb: i32, info: &mut i32) {

    unsafe {
        ffi::sgttrs_(&(trans as c_char), &n, &nrhs, dl.as_ptr(), d.as_ptr(), du.as_ptr(),
                     du2.as_ptr(), ipiv.as_ptr(), b.as_mut_ptr(), &ldb, info)
    }
}

#[inline]
pub fn dgttrs(trans: u8, n: i32, nrhs: i32, dl: &[f64], d: &[f64], du: &[f64], du2: &[f64],
              ipiv: &[i32], b: &mut [f64], ldb: i32, info: &mut i32) {

    unsafe {
        ffi::dgttrs_(&(trans as c_char), &n, &nrhs, dl.as_ptr(), d.as_ptr(), du.as_ptr(),
                     du2.as_ptr(), ipiv.as_ptr(), b.as_mut_ptr(), &ldb, info)
    }
}

#[inline]
pub fn cgttrs(trans: u8, n: i32, nrhs: i32, dl: &[c32], d: &[c32], du: &[c32], du2: &[c32],
              ipiv: &[i32], b: &mut [c32], ldb: i32, info: &mut i32) {

    unsafe {
        ffi::cgttrs_(&(trans as c_char), &n, &nrhs, dl.as_ptr() as *const _,
                     d.as_ptr() as *const _, du.as_ptr() as *const _, du2.as_ptr() as *const _,
                     ipiv.as_ptr(), b.as_mut_ptr() as *mut _, &ldb, info)
    }
}

#[inline]
pub fn zgttrs(trans: u8, n: i32, nrhs: i32, dl: &[c64], d: &[c64], du: &[c64], du2: &[c64],
              ipiv: &[i32], b: &mut [c64], ldb: i32, info: &mut i32) {

    unsafe {
        ffi::zgttrs_(&(trans as c_char), &n, &nrhs, dl.as_ptr() as *const _,
                     d.as_ptr() as *const _, du.as_ptr() as *const _, du2.as_ptr() as *const _,
                     ipiv.as_ptr(), b.as_mut_ptr() as *mut _, &ldb, info)
    }
}

#[inline]
pub fn spotrs(uplo: u8, n: i32, nrhs: i32, a: &[f32], lda: i32, b: &mut [f32], ldb: i32,
              info: &mut i32) {

    unsafe {
        ffi::spotrs_(&(uplo as c_char), &n, &nrhs, a.as_ptr(), &lda, b.as_mut_ptr(), &ldb, info)
    }
}

#[inline]
pub fn dpotrs(uplo: u8, n: i32, nrhs: i32, a: &[f64], lda: i32, b: &mut [f64], ldb: i32,
              info: &mut i32) {

    unsafe {
        ffi::dpotrs_(&(uplo as c_char), &n, &nrhs, a.as_ptr(), &lda, b.as_mut_ptr(), &ldb, info)
    }
}

#[inline]
pub fn cpotrs(uplo: u8, n: i32, nrhs: i32, a: &[c32], lda: i32, b: &mut [c32], ldb: i32,
              info: &mut i32) {

    unsafe {
        ffi::cpotrs_(&(uplo as c_char), &n, &nrhs, a.as_ptr() as *const _, &lda,
                     b.as_mut_ptr() as *mut _, &ldb, info)
    }
}

#[inline]
pub fn zpotrs(uplo: u8, n: i32, nrhs: i32, a: &[c64], lda: i32, b: &mut [c64], ldb: i32,
              info: &mut i32) {

    unsafe {
        ffi::zpotrs_(&(uplo as c_char), &n, &nrhs, a.as_ptr() as *const _, &lda,
                     b.as_mut_ptr() as *mut _, &ldb, info)
    }
}

#[inline]
pub fn dpftrs(transr: u8, uplo: u8, n: i32, nrhs: i32, a: &[f64], b: &mut [f64], ldb: i32,
              info: &mut i32) {

    unsafe {
        ffi::dpftrs_(&(transr as c_char), &(uplo as c_char), &n, &nrhs, a.as_ptr(), b.as_mut_ptr(),
                     &ldb, info)
    }
}

#[inline]
pub fn spftrs(transr: u8, uplo: u8, n: i32, nrhs: i32, a: &[f32], b: &mut [f32], ldb: i32,
              info: &mut i32) {

    unsafe {
        ffi::spftrs_(&(transr as c_char), &(uplo as c_char), &n, &nrhs, a.as_ptr(), b.as_mut_ptr(),
                     &ldb, info)
    }
}

#[inline]
pub fn zpftrs(transr: u8, uplo: u8, n: i32, nrhs: i32, a: &[c64], b: &mut [c64], ldb: i32,
              info: &mut i32) {

    unsafe {
        ffi::zpftrs_(&(transr as c_char), &(uplo as c_char), &n, &nrhs, a.as_ptr() as *const _,
                     b.as_mut_ptr() as *mut _, &ldb, info)
    }
}

#[inline]
pub fn cpftrs(transr: u8, uplo: u8, n: i32, nrhs: i32, a: &[c32], b: &mut [c32], ldb: i32,
              info: &mut i32) {

    unsafe {
        ffi::cpftrs_(&(transr as c_char), &(uplo as c_char), &n, &nrhs, a.as_ptr() as *const _,
                     b.as_mut_ptr() as *mut _, &ldb, info)
    }
}

#[inline]
pub fn spptrs(uplo: u8, n: i32, nrhs: i32, ap: &[f32], b: &mut [f32], ldb: i32, info: &mut i32) {
    unsafe {
        ffi::spptrs_(&(uplo as c_char), &n, &nrhs, ap.as_ptr(), b.as_mut_ptr(), &ldb, info)
    }
}

#[inline]
pub fn dpptrs(uplo: u8, n: i32, nrhs: i32, ap: &[f64], b: &mut [f64], ldb: i32, info: &mut i32) {
    unsafe {
        ffi::dpptrs_(&(uplo as c_char), &n, &nrhs, ap.as_ptr(), b.as_mut_ptr(), &ldb, info)
    }
}

#[inline]
pub fn cpptrs(uplo: u8, n: i32, nrhs: i32, ap: &[c32], b: &mut [c32], ldb: i32, info: &mut i32) {
    unsafe {
        ffi::cpptrs_(&(uplo as c_char), &n, &nrhs, ap.as_ptr() as *const _,
                     b.as_mut_ptr() as *mut _, &ldb, info)
    }
}

#[inline]
pub fn zpptrs(uplo: u8, n: i32, nrhs: i32, ap: &[c64], b: &mut [c64], ldb: i32, info: &mut i32) {
    unsafe {
        ffi::zpptrs_(&(uplo as c_char), &n, &nrhs, ap.as_ptr() as *const _,
                     b.as_mut_ptr() as *mut _, &ldb, info)
    }
}

#[inline]
pub fn spbtrs(uplo: u8, n: i32, kd: i32, nrhs: i32, ab: &[f32], ldab: i32, b: &mut [f32], ldb: i32,
              info: &mut i32) {

    unsafe {
        ffi::spbtrs_(&(uplo as c_char), &n, &kd, &nrhs, ab.as_ptr(), &ldab, b.as_mut_ptr(), &ldb,
                     info)
    }
}

#[inline]
pub fn dpbtrs(uplo: u8, n: i32, kd: i32, nrhs: i32, ab: &[f64], ldab: i32, b: &mut [f64], ldb: i32,
              info: &mut i32) {

    unsafe {
        ffi::dpbtrs_(&(uplo as c_char), &n, &kd, &nrhs, ab.as_ptr(), &ldab, b.as_mut_ptr(), &ldb,
                     info)
    }
}

#[inline]
pub fn cpbtrs(uplo: u8, n: i32, kd: i32, nrhs: i32, ab: &[c32], ldab: i32, b: &mut [c32], ldb: i32,
              info: &mut i32) {

    unsafe {
        ffi::cpbtrs_(&(uplo as c_char), &n, &kd, &nrhs, ab.as_ptr() as *const _, &ldab,
                     b.as_mut_ptr() as *mut _, &ldb, info)
    }
}

#[inline]
pub fn zpbtrs(uplo: u8, n: i32, kd: i32, nrhs: i32, ab: &[c64], ldab: i32, b: &mut [c64], ldb: i32,
              info: &mut i32) {

    unsafe {
        ffi::zpbtrs_(&(uplo as c_char), &n, &kd, &nrhs, ab.as_ptr() as *const _, &ldab,
                     b.as_mut_ptr() as *mut _, &ldb, info)
    }
}

#[inline]
pub fn spttrs(n: i32, nrhs: i32, d: &[f32], e: &[f32], b: &mut [f32], ldb: i32, info: &mut i32) {
    unsafe {
        ffi::spttrs_(&n, &nrhs, d.as_ptr(), e.as_ptr(), b.as_mut_ptr(), &ldb, info)
    }
}

#[inline]
pub fn dpttrs(n: i32, nrhs: i32, d: &[f64], e: &[f64], b: &mut [f64], ldb: i32, info: &mut i32) {
    unsafe {
        ffi::dpttrs_(&n, &nrhs, d.as_ptr(), e.as_ptr(), b.as_mut_ptr(), &ldb, info)
    }
}

#[inline]
pub fn cpttrs(uplo: u8, n: i32, nrhs: i32, d: &[f32], e: &[c32], b: &mut [c32], ldb: i32,
              info: &mut i32) {

    unsafe {
        ffi::cpttrs_(&(uplo as c_char), &n, &nrhs, d.as_ptr(), e.as_ptr() as *const _,
                     b.as_mut_ptr() as *mut _, &ldb, info)
    }
}

#[inline]
pub fn zpttrs(uplo: u8, n: i32, nrhs: i32, d: &[f64], e: &[c64], b: &mut [c64], ldb: i32,
              info: &mut i32) {

    unsafe {
        ffi::zpttrs_(&(uplo as c_char), &n, &nrhs, d.as_ptr(), e.as_ptr() as *const _,
                     b.as_mut_ptr() as *mut _, &ldb, info)
    }
}

#[inline]
pub fn ssytrs(uplo: u8, n: i32, nrhs: i32, a: &[f32], lda: i32, ipiv: &[i32], b: &mut [f32],
              ldb: i32, info: &mut i32) {

    unsafe {
        ffi::ssytrs_(&(uplo as c_char), &n, &nrhs, a.as_ptr(), &lda, ipiv.as_ptr(), b.as_mut_ptr(),
                     &ldb, info)
    }
}

#[inline]
pub fn dsytrs(uplo: u8, n: i32, nrhs: i32, a: &[f64], lda: i32, ipiv: &[i32], b: &mut [f64],
              ldb: i32, info: &mut i32) {

    unsafe {
        ffi::dsytrs_(&(uplo as c_char), &n, &nrhs, a.as_ptr(), &lda, ipiv.as_ptr(), b.as_mut_ptr(),
                     &ldb, info)
    }
}

#[inline]
pub fn csytrs(uplo: u8, n: i32, nrhs: i32, a: &[c32], lda: i32, ipiv: &[i32], b: &mut [c32],
              ldb: i32, info: &mut i32) {

    unsafe {
        ffi::csytrs_(&(uplo as c_char), &n, &nrhs, a.as_ptr() as *const _, &lda, ipiv.as_ptr(),
                     b.as_mut_ptr() as *mut _, &ldb, info)
    }
}

#[inline]
pub fn zsytrs(uplo: u8, n: i32, nrhs: i32, a: &[c64], lda: i32, ipiv: &[i32], b: &mut [c64],
              ldb: i32, info: &mut i32) {

    unsafe {
        ffi::zsytrs_(&(uplo as c_char), &n, &nrhs, a.as_ptr() as *const _, &lda, ipiv.as_ptr(),
                     b.as_mut_ptr() as *mut _, &ldb, info)
    }
}

#[inline]
pub fn chetrs(uplo: u8, n: i32, nrhs: i32, a: &[c32], lda: i32, ipiv: &[i32], b: &mut [c32],
              ldb: i32, info: &mut i32) {

    unsafe {
        ffi::chetrs_(&(uplo as c_char), &n, &nrhs, a.as_ptr() as *const _, &lda, ipiv.as_ptr(),
                     b.as_mut_ptr() as *mut _, &ldb, info)
    }
}

#[inline]
pub fn zhetrs(uplo: u8, n: i32, nrhs: i32, a: &[c64], lda: i32, ipiv: &[i32], b: &mut [c64],
              ldb: i32, info: &mut i32) {

    unsafe {
        ffi::zhetrs_(&(uplo as c_char), &n, &nrhs, a.as_ptr() as *const _, &lda, ipiv.as_ptr(),
                     b.as_mut_ptr() as *mut _, &ldb, info)
    }
}

#[inline]
pub fn ssptrs(uplo: u8, n: i32, nrhs: i32, ap: &[f32], ipiv: &[i32], b: &mut [f32], ldb: i32,
              info: &mut i32) {

    unsafe {
        ffi::ssptrs_(&(uplo as c_char), &n, &nrhs, ap.as_ptr(), ipiv.as_ptr(), b.as_mut_ptr(),
                     &ldb, info)
    }
}

#[inline]
pub fn dsptrs(uplo: u8, n: i32, nrhs: i32, ap: &[f64], ipiv: &[i32], b: &mut [f64], ldb: i32,
              info: &mut i32) {

    unsafe {
        ffi::dsptrs_(&(uplo as c_char), &n, &nrhs, ap.as_ptr(), ipiv.as_ptr(), b.as_mut_ptr(),
                     &ldb, info)
    }
}

#[inline]
pub fn csptrs(uplo: u8, n: i32, nrhs: i32, ap: &[c32], ipiv: &[i32], b: &mut [c32], ldb: i32,
              info: &mut i32) {

    unsafe {
        ffi::csptrs_(&(uplo as c_char), &n, &nrhs, ap.as_ptr() as *const _, ipiv.as_ptr(),
                     b.as_mut_ptr() as *mut _, &ldb, info)
    }
}

#[inline]
pub fn zsptrs(uplo: u8, n: i32, nrhs: i32, ap: &[c64], ipiv: &[i32], b: &mut [c64], ldb: i32,
              info: &mut i32) {

    unsafe {
        ffi::zsptrs_(&(uplo as c_char), &n, &nrhs, ap.as_ptr() as *const _, ipiv.as_ptr(),
                     b.as_mut_ptr() as *mut _, &ldb, info)
    }
}

#[inline]
pub fn chptrs(uplo: u8, n: i32, nrhs: i32, ap: &[c32], ipiv: &[i32], b: &mut [c32], ldb: i32,
              info: &mut i32) {

    unsafe {
        ffi::chptrs_(&(uplo as c_char), &n, &nrhs, ap.as_ptr() as *const _, ipiv.as_ptr(),
                     b.as_mut_ptr() as *mut _, &ldb, info)
    }
}

#[inline]
pub fn zhptrs(uplo: u8, n: i32, nrhs: i32, ap: &[c64], ipiv: &[i32], b: &mut [c64], ldb: i32,
              info: &mut i32) {

    unsafe {
        ffi::zhptrs_(&(uplo as c_char), &n, &nrhs, ap.as_ptr() as *const _, ipiv.as_ptr(),
                     b.as_mut_ptr() as *mut _, &ldb, info)
    }
}

#[inline]
pub fn strtrs(uplo: u8, trans: u8, diag: u8, n: i32, nrhs: i32, a: &[f32], lda: i32, b: &mut [f32],
              ldb: i32, info: &mut i32) {

    unsafe {
        ffi::strtrs_(&(uplo as c_char), &(trans as c_char), &(diag as c_char), &n, &nrhs,
                     a.as_ptr(), &lda, b.as_mut_ptr(), &ldb, info)
    }
}

#[inline]
pub fn dtrtrs(uplo: u8, trans: u8, diag: u8, n: i32, nrhs: i32, a: &[f64], lda: i32, b: &mut [f64],
              ldb: i32, info: &mut i32) {

    unsafe {
        ffi::dtrtrs_(&(uplo as c_char), &(trans as c_char), &(diag as c_char), &n, &nrhs,
                     a.as_ptr(), &lda, b.as_mut_ptr(), &ldb, info)
    }
}

#[inline]
pub fn ctrtrs(uplo: u8, trans: u8, diag: u8, n: i32, nrhs: i32, a: &[c32], lda: i32, b: &mut [c32],
              ldb: i32, info: &mut i32) {

    unsafe {
        ffi::ctrtrs_(&(uplo as c_char), &(trans as c_char), &(diag as c_char), &n, &nrhs,
                     a.as_ptr() as *const _, &lda, b.as_mut_ptr() as *mut _, &ldb, info)
    }
}

#[inline]
pub fn ztrtrs(uplo: u8, trans: u8, diag: u8, n: i32, nrhs: i32, a: &[c64], lda: i32, b: &mut [c64],
              ldb: i32, info: &mut i32) {

    unsafe {
        ffi::ztrtrs_(&(uplo as c_char), &(trans as c_char), &(diag as c_char), &n, &nrhs,
                     a.as_ptr() as *const _, &lda, b.as_mut_ptr() as *mut _, &ldb, info)
    }
}

#[inline]
pub fn stptrs(uplo: u8, trans: u8, diag: u8, n: i32, nrhs: i32, ap: &[f32], b: &mut [f32],
              ldb: i32, info: &mut i32) {

    unsafe {
        ffi::stptrs_(&(uplo as c_char), &(trans as c_char), &(diag as c_char), &n, &nrhs,
                     ap.as_ptr(), b.as_mut_ptr(), &ldb, info)
    }
}

#[inline]
pub fn dtptrs(uplo: u8, trans: u8, diag: u8, n: i32, nrhs: i32, ap: &[f64], b: &mut [f64],
              ldb: i32, info: &mut i32) {

    unsafe {
        ffi::dtptrs_(&(uplo as c_char), &(trans as c_char), &(diag as c_char), &n, &nrhs,
                     ap.as_ptr(), b.as_mut_ptr(), &ldb, info)
    }
}

#[inline]
pub fn ctptrs(uplo: u8, trans: u8, diag: u8, n: i32, nrhs: i32, ap: &[c32], b: &mut [c32],
              ldb: i32, info: &mut i32) {

    unsafe {
        ffi::ctptrs_(&(uplo as c_char), &(trans as c_char), &(diag as c_char), &n, &nrhs,
                     ap.as_ptr() as *const _, b.as_mut_ptr() as *mut _, &ldb, info)
    }
}

#[inline]
pub fn ztptrs(uplo: u8, trans: u8, diag: u8, n: i32, nrhs: i32, ap: &[c64], b: &mut [c64],
              ldb: i32, info: &mut i32) {

    unsafe {
        ffi::ztptrs_(&(uplo as c_char), &(trans as c_char), &(diag as c_char), &n, &nrhs,
                     ap.as_ptr() as *const _, b.as_mut_ptr() as *mut _, &ldb, info)
    }
}

#[inline]
pub fn stbtrs(uplo: u8, trans: u8, diag: u8, n: i32, kd: i32, nrhs: i32, ab: &[f32], ldab: i32,
              b: &mut [f32], ldb: i32, info: &mut i32) {

    unsafe {
        ffi::stbtrs_(&(uplo as c_char), &(trans as c_char), &(diag as c_char), &n, &kd, &nrhs,
                     ab.as_ptr(), &ldab, b.as_mut_ptr(), &ldb, info)
    }
}

#[inline]
pub fn dtbtrs(uplo: u8, trans: u8, diag: u8, n: i32, kd: i32, nrhs: i32, ab: &[f64], ldab: i32,
              b: &mut [f64], ldb: i32, info: &mut i32) {

    unsafe {
        ffi::dtbtrs_(&(uplo as c_char), &(trans as c_char), &(diag as c_char), &n, &kd, &nrhs,
                     ab.as_ptr(), &ldab, b.as_mut_ptr(), &ldb, info)
    }
}

#[inline]
pub fn ctbtrs(uplo: u8, trans: u8, diag: u8, n: i32, kd: i32, nrhs: i32, ab: &[c32], ldab: i32,
              b: &mut [c32], ldb: i32, info: &mut i32) {

    unsafe {
        ffi::ctbtrs_(&(uplo as c_char), &(trans as c_char), &(diag as c_char), &n, &kd, &nrhs,
                     ab.as_ptr() as *const _, &ldab, b.as_mut_ptr() as *mut _, &ldb, info)
    }
}

#[inline]
pub fn ztbtrs(uplo: u8, trans: u8, diag: u8, n: i32, kd: i32, nrhs: i32, ab: &[c64], ldab: i32,
              b: &mut [c64], ldb: i32, info: &mut i32) {

    unsafe {
        ffi::ztbtrs_(&(uplo as c_char), &(trans as c_char), &(diag as c_char), &n, &kd, &nrhs,
                     ab.as_ptr() as *const _, &ldab, b.as_mut_ptr() as *mut _, &ldb, info)
    }
}

#[inline]
pub fn sgecon(norm: u8, n: i32, a: &[f32], lda: i32, anorm: f32, rcond: &mut f32, work: &mut [f32],
              iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::sgecon_(&(norm as c_char), &n, a.as_ptr(), &lda, &anorm, rcond, work.as_mut_ptr(),
                     iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dgecon(norm: u8, n: i32, a: &[f64], lda: i32, anorm: f64, rcond: &mut f64, work: &mut [f64],
              iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::dgecon_(&(norm as c_char), &n, a.as_ptr(), &lda, &anorm, rcond, work.as_mut_ptr(),
                     iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn cgecon(norm: u8, n: i32, a: &[c32], lda: i32, anorm: f32, rcond: &mut f32, work: &mut [c32],
              rwork: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::cgecon_(&(norm as c_char), &n, a.as_ptr() as *const _, &lda, &anorm, rcond,
                     work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zgecon(norm: u8, n: i32, a: &[c64], lda: i32, anorm: f64, rcond: &mut f64, work: &mut [c64],
              rwork: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::zgecon_(&(norm as c_char), &n, a.as_ptr() as *const _, &lda, &anorm, rcond,
                     work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn sgbcon(norm: u8, n: i32, kl: i32, ku: i32, ab: &[f32], ldab: i32, ipiv: &[i32], anorm: f32,
              rcond: &mut f32, work: &mut [f32], iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::sgbcon_(&(norm as c_char), &n, &kl, &ku, ab.as_ptr(), &ldab, ipiv.as_ptr(), &anorm,
                     rcond, work.as_mut_ptr(), iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dgbcon(norm: u8, n: i32, kl: i32, ku: i32, ab: &[f64], ldab: i32, ipiv: &[i32], anorm: f64,
              rcond: &mut f64, work: &mut [f64], iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::dgbcon_(&(norm as c_char), &n, &kl, &ku, ab.as_ptr(), &ldab, ipiv.as_ptr(), &anorm,
                     rcond, work.as_mut_ptr(), iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn cgbcon(norm: u8, n: i32, kl: i32, ku: i32, ab: &[c32], ldab: i32, ipiv: &[i32], anorm: f32,
              rcond: &mut f32, work: &mut [c32], rwork: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::cgbcon_(&(norm as c_char), &n, &kl, &ku, ab.as_ptr() as *const _, &ldab,
                     ipiv.as_ptr(), &anorm, rcond, work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(),
                     info)
    }
}

#[inline]
pub fn zgbcon(norm: u8, n: i32, kl: i32, ku: i32, ab: &[c64], ldab: i32, ipiv: &[i32], anorm: f64,
              rcond: &mut f64, work: &mut [c64], rwork: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::zgbcon_(&(norm as c_char), &n, &kl, &ku, ab.as_ptr() as *const _, &ldab,
                     ipiv.as_ptr(), &anorm, rcond, work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(),
                     info)
    }
}

#[inline]
pub fn sgtcon(norm: u8, n: i32, dl: &[f32], d: &[f32], du: &[f32], du2: &[f32], ipiv: &[i32],
              anorm: f32, rcond: &mut f32, work: &mut [f32], iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::sgtcon_(&(norm as c_char), &n, dl.as_ptr(), d.as_ptr(), du.as_ptr(), du2.as_ptr(),
                     ipiv.as_ptr(), &anorm, rcond, work.as_mut_ptr(), iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dgtcon(norm: u8, n: i32, dl: &[f64], d: &[f64], du: &[f64], du2: &[f64], ipiv: &[i32],
              anorm: f64, rcond: &mut f64, work: &mut [f64], iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::dgtcon_(&(norm as c_char), &n, dl.as_ptr(), d.as_ptr(), du.as_ptr(), du2.as_ptr(),
                     ipiv.as_ptr(), &anorm, rcond, work.as_mut_ptr(), iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn cgtcon(norm: u8, n: i32, dl: &[c32], d: &[c32], du: &[c32], du2: &[c32], ipiv: &[i32],
              anorm: f32, rcond: &mut f32, work: &mut [c32], info: &mut i32) {

    unsafe {
        ffi::cgtcon_(&(norm as c_char), &n, dl.as_ptr() as *const _, d.as_ptr() as *const _,
                     du.as_ptr() as *const _, du2.as_ptr() as *const _, ipiv.as_ptr(), &anorm,
                     rcond, work.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn zgtcon(norm: u8, n: i32, dl: &[c64], d: &[c64], du: &[c64], du2: &[c64], ipiv: &[i32],
              anorm: f64, rcond: &mut f64, work: &mut [c64], info: &mut i32) {

    unsafe {
        ffi::zgtcon_(&(norm as c_char), &n, dl.as_ptr() as *const _, d.as_ptr() as *const _,
                     du.as_ptr() as *const _, du2.as_ptr() as *const _, ipiv.as_ptr(), &anorm,
                     rcond, work.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn spocon(uplo: u8, n: i32, a: &[f32], lda: i32, anorm: f32, rcond: &mut f32, work: &mut [f32],
              iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::spocon_(&(uplo as c_char), &n, a.as_ptr(), &lda, &anorm, rcond, work.as_mut_ptr(),
                     iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dpocon(uplo: u8, n: i32, a: &[f64], lda: i32, anorm: f64, rcond: &mut f64, work: &mut [f64],
              iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::dpocon_(&(uplo as c_char), &n, a.as_ptr(), &lda, &anorm, rcond, work.as_mut_ptr(),
                     iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn cpocon(uplo: u8, n: i32, a: &[c32], lda: i32, anorm: f32, rcond: &mut f32, work: &mut [c32],
              rwork: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::cpocon_(&(uplo as c_char), &n, a.as_ptr() as *const _, &lda, &anorm, rcond,
                     work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zpocon(uplo: u8, n: i32, a: &[c64], lda: i32, anorm: f64, rcond: &mut f64, work: &mut [c64],
              rwork: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::zpocon_(&(uplo as c_char), &n, a.as_ptr() as *const _, &lda, &anorm, rcond,
                     work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn sppcon(uplo: u8, n: i32, ap: &[f32], anorm: f32, rcond: &mut f32, work: &mut [f32],
              iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::sppcon_(&(uplo as c_char), &n, ap.as_ptr(), &anorm, rcond, work.as_mut_ptr(),
                     iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dppcon(uplo: u8, n: i32, ap: &[f64], anorm: f64, rcond: &mut f64, work: &mut [f64],
              iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::dppcon_(&(uplo as c_char), &n, ap.as_ptr(), &anorm, rcond, work.as_mut_ptr(),
                     iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn cppcon(uplo: u8, n: i32, ap: &[c32], anorm: f32, rcond: &mut f32, work: &mut [c32],
              rwork: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::cppcon_(&(uplo as c_char), &n, ap.as_ptr() as *const _, &anorm, rcond,
                     work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zppcon(uplo: u8, n: i32, ap: &[c64], anorm: f64, rcond: &mut f64, work: &mut [c64],
              rwork: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::zppcon_(&(uplo as c_char), &n, ap.as_ptr() as *const _, &anorm, rcond,
                     work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn spbcon(uplo: u8, n: i32, kd: i32, ab: &[f32], ldab: i32, anorm: f32, rcond: &mut f32,
              work: &mut [f32], iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::spbcon_(&(uplo as c_char), &n, &kd, ab.as_ptr(), &ldab, &anorm, rcond,
                     work.as_mut_ptr(), iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dpbcon(uplo: u8, n: i32, kd: i32, ab: &[f64], ldab: i32, anorm: f64, rcond: &mut f64,
              work: &mut [f64], iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::dpbcon_(&(uplo as c_char), &n, &kd, ab.as_ptr(), &ldab, &anorm, rcond,
                     work.as_mut_ptr(), iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn cpbcon(uplo: u8, n: i32, kd: i32, ab: &[c32], ldab: i32, anorm: f32, rcond: &mut f32,
              work: &mut [c32], rwork: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::cpbcon_(&(uplo as c_char), &n, &kd, ab.as_ptr() as *const _, &ldab, &anorm, rcond,
                     work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zpbcon(uplo: u8, n: i32, kd: i32, ab: &[c64], ldab: i32, anorm: f64, rcond: &mut f64,
              work: &mut [c64], rwork: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::zpbcon_(&(uplo as c_char), &n, &kd, ab.as_ptr() as *const _, &ldab, &anorm, rcond,
                     work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn sptcon(n: i32, d: &[f32], e: &[f32], anorm: f32, rcond: &mut f32, work: &mut [f32],
              info: &mut i32) {

    unsafe {
        ffi::sptcon_(&n, d.as_ptr(), e.as_ptr(), &anorm, rcond, work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dptcon(n: i32, d: &[f64], e: &[f64], anorm: f64, rcond: &mut f64, work: &mut [f64],
              info: &mut i32) {

    unsafe {
        ffi::dptcon_(&n, d.as_ptr(), e.as_ptr(), &anorm, rcond, work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn cptcon(n: i32, d: &[f32], e: &[c32], anorm: f32, rcond: &mut f32, work: &mut [f32],
              info: &mut i32) {

    unsafe {
        ffi::cptcon_(&n, d.as_ptr(), e.as_ptr() as *const _, &anorm, rcond, work.as_mut_ptr(),
                     info)
    }
}

#[inline]
pub fn zptcon(n: i32, d: &[f64], e: &[c64], anorm: f64, rcond: &mut f64, work: &mut [f64],
              info: &mut i32) {

    unsafe {
        ffi::zptcon_(&n, d.as_ptr(), e.as_ptr() as *const _, &anorm, rcond, work.as_mut_ptr(),
                     info)
    }
}

#[inline]
pub fn ssycon(uplo: u8, n: i32, a: &[f32], lda: i32, ipiv: &[i32], anorm: f32, rcond: &mut f32,
              work: &mut [f32], iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::ssycon_(&(uplo as c_char), &n, a.as_ptr(), &lda, ipiv.as_ptr(), &anorm, rcond,
                     work.as_mut_ptr(), iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dsycon(uplo: u8, n: i32, a: &[f64], lda: i32, ipiv: &[i32], anorm: f64, rcond: &mut f64,
              work: &mut [f64], iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::dsycon_(&(uplo as c_char), &n, a.as_ptr(), &lda, ipiv.as_ptr(), &anorm, rcond,
                     work.as_mut_ptr(), iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn csycon(uplo: u8, n: i32, a: &[c32], lda: i32, ipiv: &[i32], anorm: f32, rcond: &mut f32,
              work: &mut [c32], info: &mut i32) {

    unsafe {
        ffi::csycon_(&(uplo as c_char), &n, a.as_ptr() as *const _, &lda, ipiv.as_ptr(), &anorm,
                     rcond, work.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn zsycon(uplo: u8, n: i32, a: &[c64], lda: i32, ipiv: &[i32], anorm: f64, rcond: &mut f64,
              work: &mut [c64], info: &mut i32) {

    unsafe {
        ffi::zsycon_(&(uplo as c_char), &n, a.as_ptr() as *const _, &lda, ipiv.as_ptr(), &anorm,
                     rcond, work.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn checon(uplo: u8, n: i32, a: &[c32], lda: i32, ipiv: &[i32], anorm: f32, rcond: &mut f32,
              work: &mut [c32], info: &mut i32) {

    unsafe {
        ffi::checon_(&(uplo as c_char), &n, a.as_ptr() as *const _, &lda, ipiv.as_ptr(), &anorm,
                     rcond, work.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn zhecon(uplo: u8, n: i32, a: &[c64], lda: i32, ipiv: &[i32], anorm: f64, rcond: &mut f64,
              work: &mut [c64], info: &mut i32) {

    unsafe {
        ffi::zhecon_(&(uplo as c_char), &n, a.as_ptr() as *const _, &lda, ipiv.as_ptr(), &anorm,
                     rcond, work.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn sspcon(uplo: u8, n: i32, ap: &[f32], ipiv: &[i32], anorm: f32, rcond: &mut f32,
              work: &mut [f32], iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::sspcon_(&(uplo as c_char), &n, ap.as_ptr(), ipiv.as_ptr(), &anorm, rcond,
                     work.as_mut_ptr(), iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dspcon(uplo: u8, n: i32, ap: &[f64], ipiv: &[i32], anorm: f64, rcond: &mut f64,
              work: &mut [f64], iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::dspcon_(&(uplo as c_char), &n, ap.as_ptr(), ipiv.as_ptr(), &anorm, rcond,
                     work.as_mut_ptr(), iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn cspcon(uplo: u8, n: i32, ap: &[c32], ipiv: &[i32], anorm: f32, rcond: &mut f32,
              work: &mut [c32], info: &mut i32) {

    unsafe {
        ffi::cspcon_(&(uplo as c_char), &n, ap.as_ptr() as *const _, ipiv.as_ptr(), &anorm, rcond,
                     work.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn zspcon(uplo: u8, n: i32, ap: &[c64], ipiv: &[i32], anorm: f64, rcond: &mut f64,
              work: &mut [c64], info: &mut i32) {

    unsafe {
        ffi::zspcon_(&(uplo as c_char), &n, ap.as_ptr() as *const _, ipiv.as_ptr(), &anorm, rcond,
                     work.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn chpcon(uplo: u8, n: i32, ap: &[c32], ipiv: &[i32], anorm: f32, rcond: &mut f32,
              work: &mut [c32], info: &mut i32) {

    unsafe {
        ffi::chpcon_(&(uplo as c_char), &n, ap.as_ptr() as *const _, ipiv.as_ptr(), &anorm, rcond,
                     work.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn zhpcon(uplo: u8, n: i32, ap: &[c64], ipiv: &[i32], anorm: f64, rcond: &mut f64,
              work: &mut [c64], info: &mut i32) {

    unsafe {
        ffi::zhpcon_(&(uplo as c_char), &n, ap.as_ptr() as *const _, ipiv.as_ptr(), &anorm, rcond,
                     work.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn strcon(norm: u8, uplo: u8, diag: u8, n: i32, a: &[f32], lda: i32, rcond: &mut f32,
              work: &mut [f32], iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::strcon_(&(norm as c_char), &(uplo as c_char), &(diag as c_char), &n, a.as_ptr(), &lda,
                     rcond, work.as_mut_ptr(), iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dtrcon(norm: u8, uplo: u8, diag: u8, n: i32, a: &[f64], lda: i32, rcond: &mut f64,
              work: &mut [f64], iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::dtrcon_(&(norm as c_char), &(uplo as c_char), &(diag as c_char), &n, a.as_ptr(), &lda,
                     rcond, work.as_mut_ptr(), iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn ctrcon(norm: u8, uplo: u8, diag: u8, n: i32, a: &[c32], lda: i32, rcond: &mut f32,
              work: &mut [c32], rwork: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::ctrcon_(&(norm as c_char), &(uplo as c_char), &(diag as c_char), &n,
                     a.as_ptr() as *const _, &lda, rcond, work.as_mut_ptr() as *mut _,
                     rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn ztrcon(norm: u8, uplo: u8, diag: u8, n: i32, a: &[c64], lda: i32, rcond: &mut f64,
              work: &mut [c64], rwork: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::ztrcon_(&(norm as c_char), &(uplo as c_char), &(diag as c_char), &n,
                     a.as_ptr() as *const _, &lda, rcond, work.as_mut_ptr() as *mut _,
                     rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn stpcon(norm: u8, uplo: u8, diag: u8, n: i32, ap: &[f32], rcond: &mut f32, work: &mut [f32],
              iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::stpcon_(&(norm as c_char), &(uplo as c_char), &(diag as c_char), &n, ap.as_ptr(),
                     rcond, work.as_mut_ptr(), iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dtpcon(norm: u8, uplo: u8, diag: u8, n: i32, ap: &[f64], rcond: &mut f64, work: &mut [f64],
              iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::dtpcon_(&(norm as c_char), &(uplo as c_char), &(diag as c_char), &n, ap.as_ptr(),
                     rcond, work.as_mut_ptr(), iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn ctpcon(norm: u8, uplo: u8, diag: u8, n: i32, ap: &[c32], rcond: &mut f32, work: &mut [c32],
              rwork: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::ctpcon_(&(norm as c_char), &(uplo as c_char), &(diag as c_char), &n,
                     ap.as_ptr() as *const _, rcond, work.as_mut_ptr() as *mut _,
                     rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn ztpcon(norm: u8, uplo: u8, diag: u8, n: i32, ap: &[c64], rcond: &mut f64, work: &mut [c64],
              rwork: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::ztpcon_(&(norm as c_char), &(uplo as c_char), &(diag as c_char), &n,
                     ap.as_ptr() as *const _, rcond, work.as_mut_ptr() as *mut _,
                     rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn stbcon(norm: u8, uplo: u8, diag: u8, n: i32, kd: i32, ab: &[f32], ldab: i32,
              rcond: &mut f32, work: &mut [f32], iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::stbcon_(&(norm as c_char), &(uplo as c_char), &(diag as c_char), &n, &kd, ab.as_ptr(),
                     &ldab, rcond, work.as_mut_ptr(), iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dtbcon(norm: u8, uplo: u8, diag: u8, n: i32, kd: i32, ab: &[f64], ldab: i32,
              rcond: &mut f64, work: &mut [f64], iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::dtbcon_(&(norm as c_char), &(uplo as c_char), &(diag as c_char), &n, &kd, ab.as_ptr(),
                     &ldab, rcond, work.as_mut_ptr(), iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn ctbcon(norm: u8, uplo: u8, diag: u8, n: i32, kd: i32, ab: &[c32], ldab: i32,
              rcond: &mut f32, work: &mut [c32], rwork: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::ctbcon_(&(norm as c_char), &(uplo as c_char), &(diag as c_char), &n, &kd,
                     ab.as_ptr() as *const _, &ldab, rcond, work.as_mut_ptr() as *mut _,
                     rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn ztbcon(norm: u8, uplo: u8, diag: u8, n: i32, kd: i32, ab: &[c64], ldab: i32,
              rcond: &mut f64, work: &mut [c64], rwork: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::ztbcon_(&(norm as c_char), &(uplo as c_char), &(diag as c_char), &n, &kd,
                     ab.as_ptr() as *const _, &ldab, rcond, work.as_mut_ptr() as *mut _,
                     rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn sgerfs(trans: u8, n: i32, nrhs: i32, a: &[f32], lda: i32, af: &[f32], ldaf: i32,
              ipiv: &[i32], b: &[f32], ldb: i32, x: &mut [f32], ldx: i32, ferr: &mut [f32],
              berr: &mut [f32], work: &mut [f32], iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::sgerfs_(&(trans as c_char), &n, &nrhs, a.as_ptr(), &lda, af.as_ptr(), &ldaf,
                     ipiv.as_ptr(), b.as_ptr(), &ldb, x.as_mut_ptr(), &ldx, ferr.as_mut_ptr(),
                     berr.as_mut_ptr(), work.as_mut_ptr(), iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dgerfs(trans: u8, n: i32, nrhs: i32, a: &[f64], lda: i32, af: &[f64], ldaf: i32,
              ipiv: &[i32], b: &[f64], ldb: i32, x: &mut [f64], ldx: i32, ferr: &mut [f64],
              berr: &mut [f64], work: &mut [f64], iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::dgerfs_(&(trans as c_char), &n, &nrhs, a.as_ptr(), &lda, af.as_ptr(), &ldaf,
                     ipiv.as_ptr(), b.as_ptr(), &ldb, x.as_mut_ptr(), &ldx, ferr.as_mut_ptr(),
                     berr.as_mut_ptr(), work.as_mut_ptr(), iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn cgerfs(trans: u8, n: i32, nrhs: i32, a: &[c32], lda: i32, af: &[c32], ldaf: i32,
              ipiv: &[i32], b: &[c32], ldb: i32, x: &mut [c32], ldx: i32, ferr: &mut [f32],
              berr: &mut [f32], work: &mut [c32], rwork: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::cgerfs_(&(trans as c_char), &n, &nrhs, a.as_ptr() as *const _, &lda,
                     af.as_ptr() as *const _, &ldaf, ipiv.as_ptr(), b.as_ptr() as *const _, &ldb,
                     x.as_mut_ptr() as *mut _, &ldx, ferr.as_mut_ptr(), berr.as_mut_ptr(),
                     work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zgerfs(trans: u8, n: i32, nrhs: i32, a: &[c64], lda: i32, af: &[c64], ldaf: i32,
              ipiv: &[i32], b: &[c64], ldb: i32, x: &mut [c64], ldx: i32, ferr: &mut [f64],
              berr: &mut [f64], work: &mut [c64], rwork: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::zgerfs_(&(trans as c_char), &n, &nrhs, a.as_ptr() as *const _, &lda,
                     af.as_ptr() as *const _, &ldaf, ipiv.as_ptr(), b.as_ptr() as *const _, &ldb,
                     x.as_mut_ptr() as *mut _, &ldx, ferr.as_mut_ptr(), berr.as_mut_ptr(),
                     work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dgerfsx(trans: u8, equed: u8, n: i32, nrhs: i32, a: &[f64], lda: i32, af: &[f64], ldaf: i32,
               ipiv: &[i32], r: &[f64], c: &[f64], b: &[f64], ldb: i32, x: &mut [f64], ldx: i32,
               rcond: &mut f64, berr: &mut [f64], n_err_bnds: i32, err_bnds_norm: &mut [f64],
               err_bnds_comp: &mut [f64], nparams: &[i32], params: &mut [f64], work: &mut [f64],
               iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::dgerfsx_(&(trans as c_char), &(equed as c_char), &n, &nrhs, a.as_ptr(), &lda,
                      af.as_ptr(), &ldaf, ipiv.as_ptr(), r.as_ptr(), c.as_ptr(), b.as_ptr(), &ldb,
                      x.as_mut_ptr(), &ldx, rcond, berr.as_mut_ptr(), &n_err_bnds,
                      err_bnds_norm.as_mut_ptr(), err_bnds_comp.as_mut_ptr(), nparams.as_ptr(),
                      params.as_mut_ptr(), work.as_mut_ptr(), iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn sgerfsx(trans: u8, equed: u8, n: i32, nrhs: i32, a: &[f32], lda: i32, af: &[f32], ldaf: i32,
               ipiv: &[i32], r: &[f32], c: &[f32], b: &[f32], ldb: i32, x: &mut [f32], ldx: i32,
               rcond: &mut f32, berr: &mut [f32], n_err_bnds: i32, err_bnds_norm: &mut [f32],
               err_bnds_comp: &mut [f32], nparams: &[i32], params: &mut [f32], work: &mut [f32],
               iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::sgerfsx_(&(trans as c_char), &(equed as c_char), &n, &nrhs, a.as_ptr(), &lda,
                      af.as_ptr(), &ldaf, ipiv.as_ptr(), r.as_ptr(), c.as_ptr(), b.as_ptr(), &ldb,
                      x.as_mut_ptr(), &ldx, rcond, berr.as_mut_ptr(), &n_err_bnds,
                      err_bnds_norm.as_mut_ptr(), err_bnds_comp.as_mut_ptr(), nparams.as_ptr(),
                      params.as_mut_ptr(), work.as_mut_ptr(), iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zgerfsx(trans: u8, equed: u8, n: i32, nrhs: i32, a: &[c64], lda: i32, af: &[c64], ldaf: i32,
               ipiv: &[i32], r: &[f64], c: &[f64], b: &[c64], ldb: i32, x: &mut [c64], ldx: i32,
               rcond: &mut f64, berr: &mut [f64], n_err_bnds: i32, err_bnds_norm: &mut [f64],
               err_bnds_comp: &mut [f64], nparams: &[i32], params: &mut [f64], work: &mut [c64],
               rwork: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::zgerfsx_(&(trans as c_char), &(equed as c_char), &n, &nrhs, a.as_ptr() as *const _,
                      &lda, af.as_ptr() as *const _, &ldaf, ipiv.as_ptr(), r.as_ptr(), c.as_ptr(),
                      b.as_ptr() as *const _, &ldb, x.as_mut_ptr() as *mut _, &ldx, rcond,
                      berr.as_mut_ptr(), &n_err_bnds, err_bnds_norm.as_mut_ptr(),
                      err_bnds_comp.as_mut_ptr(), nparams.as_ptr(), params.as_mut_ptr(),
                      work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn cgerfsx(trans: u8, equed: u8, n: i32, nrhs: i32, a: &[c32], lda: i32, af: &[c32], ldaf: i32,
               ipiv: &[i32], r: &[f32], c: &[f32], b: &[c32], ldb: i32, x: &mut [c32], ldx: i32,
               rcond: &mut f32, berr: &mut [f32], n_err_bnds: i32, err_bnds_norm: &mut [f32],
               err_bnds_comp: &mut [f32], nparams: &[i32], params: &mut [f32], work: &mut [c32],
               rwork: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::cgerfsx_(&(trans as c_char), &(equed as c_char), &n, &nrhs, a.as_ptr() as *const _,
                      &lda, af.as_ptr() as *const _, &ldaf, ipiv.as_ptr(), r.as_ptr(), c.as_ptr(),
                      b.as_ptr() as *const _, &ldb, x.as_mut_ptr() as *mut _, &ldx, rcond,
                      berr.as_mut_ptr(), &n_err_bnds, err_bnds_norm.as_mut_ptr(),
                      err_bnds_comp.as_mut_ptr(), nparams.as_ptr(), params.as_mut_ptr(),
                      work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn sgbrfs(trans: u8, n: i32, kl: i32, ku: i32, nrhs: i32, ab: &[f32], ldab: i32, afb: &[f32],
              ldafb: i32, ipiv: &[i32], b: &[f32], ldb: i32, x: &mut [f32], ldx: i32,
              ferr: &mut [f32], berr: &mut [f32], work: &mut [f32], iwork: &mut [i32],
              info: &mut i32) {

    unsafe {
        ffi::sgbrfs_(&(trans as c_char), &n, &kl, &ku, &nrhs, ab.as_ptr(), &ldab, afb.as_ptr(),
                     &ldafb, ipiv.as_ptr(), b.as_ptr(), &ldb, x.as_mut_ptr(), &ldx,
                     ferr.as_mut_ptr(), berr.as_mut_ptr(), work.as_mut_ptr(), iwork.as_mut_ptr(),
                     info)
    }
}

#[inline]
pub fn dgbrfs(trans: u8, n: i32, kl: i32, ku: i32, nrhs: i32, ab: &[f64], ldab: i32, afb: &[f64],
              ldafb: i32, ipiv: &[i32], b: &[f64], ldb: i32, x: &mut [f64], ldx: i32,
              ferr: &mut [f64], berr: &mut [f64], work: &mut [f64], iwork: &mut [i32],
              info: &mut i32) {

    unsafe {
        ffi::dgbrfs_(&(trans as c_char), &n, &kl, &ku, &nrhs, ab.as_ptr(), &ldab, afb.as_ptr(),
                     &ldafb, ipiv.as_ptr(), b.as_ptr(), &ldb, x.as_mut_ptr(), &ldx,
                     ferr.as_mut_ptr(), berr.as_mut_ptr(), work.as_mut_ptr(), iwork.as_mut_ptr(),
                     info)
    }
}

#[inline]
pub fn cgbrfs(trans: u8, n: i32, kl: i32, ku: i32, nrhs: i32, ab: &[c32], ldab: i32, afb: &[c32],
              ldafb: i32, ipiv: &[i32], b: &[c32], ldb: i32, x: &mut [c32], ldx: i32,
              ferr: &mut [f32], berr: &mut [f32], work: &mut [c32], rwork: &mut [f32],
              info: &mut i32) {

    unsafe {
        ffi::cgbrfs_(&(trans as c_char), &n, &kl, &ku, &nrhs, ab.as_ptr() as *const _, &ldab,
                     afb.as_ptr() as *const _, &ldafb, ipiv.as_ptr(), b.as_ptr() as *const _, &ldb,
                     x.as_mut_ptr() as *mut _, &ldx, ferr.as_mut_ptr(), berr.as_mut_ptr(),
                     work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zgbrfs(trans: u8, n: i32, kl: i32, ku: i32, nrhs: i32, ab: &[c64], ldab: i32, afb: &[c64],
              ldafb: i32, ipiv: &[i32], b: &[c64], ldb: i32, x: &mut [c64], ldx: i32,
              ferr: &mut [f64], berr: &mut [f64], work: &mut [c64], rwork: &mut [f64],
              info: &mut i32) {

    unsafe {
        ffi::zgbrfs_(&(trans as c_char), &n, &kl, &ku, &nrhs, ab.as_ptr() as *const _, &ldab,
                     afb.as_ptr() as *const _, &ldafb, ipiv.as_ptr(), b.as_ptr() as *const _, &ldb,
                     x.as_mut_ptr() as *mut _, &ldx, ferr.as_mut_ptr(), berr.as_mut_ptr(),
                     work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dgbrfsx(trans: u8, equed: u8, n: i32, kl: i32, ku: i32, nrhs: i32, ab: &[f64], ldab: i32,
               afb: &[f64], ldafb: i32, ipiv: &[i32], r: &[f64], c: &[f64], b: &[f64], ldb: i32,
               x: &mut [f64], ldx: i32, rcond: &mut f64, berr: &mut [f64], n_err_bnds: i32,
               err_bnds_norm: &mut [f64], err_bnds_comp: &mut [f64], nparams: &[i32],
               params: &mut [f64], work: &mut [f64], iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::dgbrfsx_(&(trans as c_char), &(equed as c_char), &n, &kl, &ku, &nrhs, ab.as_ptr(),
                      &ldab, afb.as_ptr(), &ldafb, ipiv.as_ptr(), r.as_ptr(), c.as_ptr(),
                      b.as_ptr(), &ldb, x.as_mut_ptr(), &ldx, rcond, berr.as_mut_ptr(),
                      &n_err_bnds, err_bnds_norm.as_mut_ptr(), err_bnds_comp.as_mut_ptr(),
                      nparams.as_ptr(), params.as_mut_ptr(), work.as_mut_ptr(), iwork.as_mut_ptr(),
                      info)
    }
}

#[inline]
pub fn sgbrfsx(trans: u8, equed: u8, n: i32, kl: i32, ku: i32, nrhs: i32, ab: &[f32], ldab: i32,
               afb: &[f32], ldafb: i32, ipiv: &[i32], r: &[f32], c: &[f32], b: &[f32], ldb: i32,
               x: &mut [f32], ldx: i32, rcond: &mut f32, berr: &mut [f32], n_err_bnds: i32,
               err_bnds_norm: &mut [f32], err_bnds_comp: &mut [f32], nparams: &[i32],
               params: &mut [f32], work: &mut [f32], iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::sgbrfsx_(&(trans as c_char), &(equed as c_char), &n, &kl, &ku, &nrhs, ab.as_ptr(),
                      &ldab, afb.as_ptr(), &ldafb, ipiv.as_ptr(), r.as_ptr(), c.as_ptr(),
                      b.as_ptr(), &ldb, x.as_mut_ptr(), &ldx, rcond, berr.as_mut_ptr(),
                      &n_err_bnds, err_bnds_norm.as_mut_ptr(), err_bnds_comp.as_mut_ptr(),
                      nparams.as_ptr(), params.as_mut_ptr(), work.as_mut_ptr(), iwork.as_mut_ptr(),
                      info)
    }
}

#[inline]
pub fn zgbrfsx(trans: u8, equed: u8, n: i32, kl: i32, ku: i32, nrhs: i32, ab: &[c64], ldab: i32,
               afb: &[c64], ldafb: i32, ipiv: &[i32], r: &[f64], c: &[f64], b: &[c64], ldb: i32,
               x: &mut [c64], ldx: i32, rcond: &mut f64, berr: &mut [f64], n_err_bnds: i32,
               err_bnds_norm: &mut [f64], err_bnds_comp: &mut [f64], nparams: &[i32],
               params: &mut [f64], work: &mut [c64], rwork: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::zgbrfsx_(&(trans as c_char), &(equed as c_char), &n, &kl, &ku, &nrhs,
                      ab.as_ptr() as *const _, &ldab, afb.as_ptr() as *const _, &ldafb,
                      ipiv.as_ptr(), r.as_ptr(), c.as_ptr(), b.as_ptr() as *const _, &ldb,
                      x.as_mut_ptr() as *mut _, &ldx, rcond, berr.as_mut_ptr(), &n_err_bnds,
                      err_bnds_norm.as_mut_ptr(), err_bnds_comp.as_mut_ptr(), nparams.as_ptr(),
                      params.as_mut_ptr(), work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn cgbrfsx(trans: u8, equed: u8, n: i32, kl: i32, ku: i32, nrhs: i32, ab: &[c32], ldab: i32,
               afb: &[c32], ldafb: i32, ipiv: &[i32], r: &[f32], c: &[f32], b: &[c32], ldb: i32,
               x: &mut [c32], ldx: i32, rcond: &mut f32, berr: &mut [f32], n_err_bnds: i32,
               err_bnds_norm: &mut [f32], err_bnds_comp: &mut [f32], nparams: &[i32],
               params: &mut [f32], work: &mut [c32], rwork: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::cgbrfsx_(&(trans as c_char), &(equed as c_char), &n, &kl, &ku, &nrhs,
                      ab.as_ptr() as *const _, &ldab, afb.as_ptr() as *const _, &ldafb,
                      ipiv.as_ptr(), r.as_ptr(), c.as_ptr(), b.as_ptr() as *const _, &ldb,
                      x.as_mut_ptr() as *mut _, &ldx, rcond, berr.as_mut_ptr(), &n_err_bnds,
                      err_bnds_norm.as_mut_ptr(), err_bnds_comp.as_mut_ptr(), nparams.as_ptr(),
                      params.as_mut_ptr(), work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn sgtrfs(trans: u8, n: i32, nrhs: i32, dl: &[f32], d: &[f32], du: &[f32], dlf: &[f32],
              df: &[f32], duf: &[f32], du2: &[f32], ipiv: &[i32], b: &[f32], ldb: i32,
              x: &mut [f32], ldx: i32, ferr: &mut [f32], berr: &mut [f32], work: &mut [f32],
              iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::sgtrfs_(&(trans as c_char), &n, &nrhs, dl.as_ptr(), d.as_ptr(), du.as_ptr(),
                     dlf.as_ptr(), df.as_ptr(), duf.as_ptr(), du2.as_ptr(), ipiv.as_ptr(),
                     b.as_ptr(), &ldb, x.as_mut_ptr(), &ldx, ferr.as_mut_ptr(), berr.as_mut_ptr(),
                     work.as_mut_ptr(), iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dgtrfs(trans: u8, n: i32, nrhs: i32, dl: &[f64], d: &[f64], du: &[f64], dlf: &[f64],
              df: &[f64], duf: &[f64], du2: &[f64], ipiv: &[i32], b: &[f64], ldb: i32,
              x: &mut [f64], ldx: i32, ferr: &mut [f64], berr: &mut [f64], work: &mut [f64],
              iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::dgtrfs_(&(trans as c_char), &n, &nrhs, dl.as_ptr(), d.as_ptr(), du.as_ptr(),
                     dlf.as_ptr(), df.as_ptr(), duf.as_ptr(), du2.as_ptr(), ipiv.as_ptr(),
                     b.as_ptr(), &ldb, x.as_mut_ptr(), &ldx, ferr.as_mut_ptr(), berr.as_mut_ptr(),
                     work.as_mut_ptr(), iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn cgtrfs(trans: u8, n: i32, nrhs: i32, dl: &[c32], d: &[c32], du: &[c32], dlf: &[c32],
              df: &[c32], duf: &[c32], du2: &[c32], ipiv: &[i32], b: &[c32], ldb: i32,
              x: &mut [c32], ldx: i32, ferr: &mut [f32], berr: &mut [f32], work: &mut [c32],
              rwork: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::cgtrfs_(&(trans as c_char), &n, &nrhs, dl.as_ptr() as *const _,
                     d.as_ptr() as *const _, du.as_ptr() as *const _, dlf.as_ptr() as *const _,
                     df.as_ptr() as *const _, duf.as_ptr() as *const _, du2.as_ptr() as *const _,
                     ipiv.as_ptr(), b.as_ptr() as *const _, &ldb, x.as_mut_ptr() as *mut _, &ldx,
                     ferr.as_mut_ptr(), berr.as_mut_ptr(), work.as_mut_ptr() as *mut _,
                     rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zgtrfs(trans: u8, n: i32, nrhs: i32, dl: &[c64], d: &[c64], du: &[c64], dlf: &[c64],
              df: &[c64], duf: &[c64], du2: &[c64], ipiv: &[i32], b: &[c64], ldb: i32,
              x: &mut [c64], ldx: i32, ferr: &mut [f64], berr: &mut [f64], work: &mut [c64],
              rwork: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::zgtrfs_(&(trans as c_char), &n, &nrhs, dl.as_ptr() as *const _,
                     d.as_ptr() as *const _, du.as_ptr() as *const _, dlf.as_ptr() as *const _,
                     df.as_ptr() as *const _, duf.as_ptr() as *const _, du2.as_ptr() as *const _,
                     ipiv.as_ptr(), b.as_ptr() as *const _, &ldb, x.as_mut_ptr() as *mut _, &ldx,
                     ferr.as_mut_ptr(), berr.as_mut_ptr(), work.as_mut_ptr() as *mut _,
                     rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn sporfs(uplo: u8, n: i32, nrhs: i32, a: &[f32], lda: i32, af: &[f32], ldaf: i32, b: &[f32],
              ldb: i32, x: &mut [f32], ldx: i32, ferr: &mut [f32], berr: &mut [f32],
              work: &mut [f32], iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::sporfs_(&(uplo as c_char), &n, &nrhs, a.as_ptr(), &lda, af.as_ptr(), &ldaf,
                     b.as_ptr(), &ldb, x.as_mut_ptr(), &ldx, ferr.as_mut_ptr(), berr.as_mut_ptr(),
                     work.as_mut_ptr(), iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dporfs(uplo: u8, n: i32, nrhs: i32, a: &[f64], lda: i32, af: &[f64], ldaf: i32, b: &[f64],
              ldb: i32, x: &mut [f64], ldx: i32, ferr: &mut [f64], berr: &mut [f64],
              work: &mut [f64], iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::dporfs_(&(uplo as c_char), &n, &nrhs, a.as_ptr(), &lda, af.as_ptr(), &ldaf,
                     b.as_ptr(), &ldb, x.as_mut_ptr(), &ldx, ferr.as_mut_ptr(), berr.as_mut_ptr(),
                     work.as_mut_ptr(), iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn cporfs(uplo: u8, n: i32, nrhs: i32, a: &[c32], lda: i32, af: &[c32], ldaf: i32, b: &[c32],
              ldb: i32, x: &mut [c32], ldx: i32, ferr: &mut [f32], berr: &mut [f32],
              work: &mut [c32], rwork: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::cporfs_(&(uplo as c_char), &n, &nrhs, a.as_ptr() as *const _, &lda,
                     af.as_ptr() as *const _, &ldaf, b.as_ptr() as *const _, &ldb,
                     x.as_mut_ptr() as *mut _, &ldx, ferr.as_mut_ptr(), berr.as_mut_ptr(),
                     work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zporfs(uplo: u8, n: i32, nrhs: i32, a: &[c64], lda: i32, af: &[c64], ldaf: i32, b: &[c64],
              ldb: i32, x: &mut [c64], ldx: i32, ferr: &mut [f64], berr: &mut [f64],
              work: &mut [c64], rwork: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::zporfs_(&(uplo as c_char), &n, &nrhs, a.as_ptr() as *const _, &lda,
                     af.as_ptr() as *const _, &ldaf, b.as_ptr() as *const _, &ldb,
                     x.as_mut_ptr() as *mut _, &ldx, ferr.as_mut_ptr(), berr.as_mut_ptr(),
                     work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dporfsx(uplo: u8, equed: u8, n: i32, nrhs: i32, a: &[f64], lda: i32, af: &[f64], ldaf: i32,
               s: &[f64], b: &[f64], ldb: i32, x: &mut [f64], ldx: i32, rcond: &mut f64,
               berr: &mut [f64], n_err_bnds: i32, err_bnds_norm: &mut [f64],
               err_bnds_comp: &mut [f64], nparams: &[i32], params: &mut [f64], work: &mut [f64],
               iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::dporfsx_(&(uplo as c_char), &(equed as c_char), &n, &nrhs, a.as_ptr(), &lda,
                      af.as_ptr(), &ldaf, s.as_ptr(), b.as_ptr(), &ldb, x.as_mut_ptr(), &ldx,
                      rcond, berr.as_mut_ptr(), &n_err_bnds, err_bnds_norm.as_mut_ptr(),
                      err_bnds_comp.as_mut_ptr(), nparams.as_ptr(), params.as_mut_ptr(),
                      work.as_mut_ptr(), iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn sporfsx(uplo: u8, equed: u8, n: i32, nrhs: i32, a: &[f32], lda: i32, af: &[f32], ldaf: i32,
               s: &[f32], b: &[f32], ldb: i32, x: &mut [f32], ldx: i32, rcond: &mut f32,
               berr: &mut [f32], n_err_bnds: i32, err_bnds_norm: &mut [f32],
               err_bnds_comp: &mut [f32], nparams: &[i32], params: &mut [f32], work: &mut [f32],
               iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::sporfsx_(&(uplo as c_char), &(equed as c_char), &n, &nrhs, a.as_ptr(), &lda,
                      af.as_ptr(), &ldaf, s.as_ptr(), b.as_ptr(), &ldb, x.as_mut_ptr(), &ldx,
                      rcond, berr.as_mut_ptr(), &n_err_bnds, err_bnds_norm.as_mut_ptr(),
                      err_bnds_comp.as_mut_ptr(), nparams.as_ptr(), params.as_mut_ptr(),
                      work.as_mut_ptr(), iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zporfsx(uplo: u8, equed: u8, n: i32, nrhs: i32, a: &[c64], lda: i32, af: &[c64], ldaf: i32,
               s: &[f64], b: &[c64], ldb: i32, x: &mut [c64], ldx: i32, rcond: &mut f64,
               berr: &mut [f64], n_err_bnds: i32, err_bnds_norm: &mut [f64],
               err_bnds_comp: &mut [f64], nparams: &[i32], params: &mut [f64], work: &mut [c64],
               rwork: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::zporfsx_(&(uplo as c_char), &(equed as c_char), &n, &nrhs, a.as_ptr() as *const _,
                      &lda, af.as_ptr() as *const _, &ldaf, s.as_ptr(), b.as_ptr() as *const _,
                      &ldb, x.as_mut_ptr() as *mut _, &ldx, rcond, berr.as_mut_ptr(), &n_err_bnds,
                      err_bnds_norm.as_mut_ptr(), err_bnds_comp.as_mut_ptr(), nparams.as_ptr(),
                      params.as_mut_ptr(), work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn cporfsx(uplo: u8, equed: u8, n: i32, nrhs: i32, a: &[c32], lda: i32, af: &[c32], ldaf: i32,
               s: &[f32], b: &[c32], ldb: i32, x: &mut [c32], ldx: i32, rcond: &mut f32,
               berr: &mut [f32], n_err_bnds: i32, err_bnds_norm: &mut [f32],
               err_bnds_comp: &mut [f32], nparams: &[i32], params: &mut [f32], work: &mut [c32],
               rwork: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::cporfsx_(&(uplo as c_char), &(equed as c_char), &n, &nrhs, a.as_ptr() as *const _,
                      &lda, af.as_ptr() as *const _, &ldaf, s.as_ptr(), b.as_ptr() as *const _,
                      &ldb, x.as_mut_ptr() as *mut _, &ldx, rcond, berr.as_mut_ptr(), &n_err_bnds,
                      err_bnds_norm.as_mut_ptr(), err_bnds_comp.as_mut_ptr(), nparams.as_ptr(),
                      params.as_mut_ptr(), work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn spprfs(uplo: u8, n: i32, nrhs: i32, ap: &[f32], afp: &[f32], b: &[f32], ldb: i32,
              x: &mut [f32], ldx: i32, ferr: &mut [f32], berr: &mut [f32], work: &mut [f32],
              iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::spprfs_(&(uplo as c_char), &n, &nrhs, ap.as_ptr(), afp.as_ptr(), b.as_ptr(), &ldb,
                     x.as_mut_ptr(), &ldx, ferr.as_mut_ptr(), berr.as_mut_ptr(), work.as_mut_ptr(),
                     iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dpprfs(uplo: u8, n: i32, nrhs: i32, ap: &[f64], afp: &[f64], b: &[f64], ldb: i32,
              x: &mut [f64], ldx: i32, ferr: &mut [f64], berr: &mut [f64], work: &mut [f64],
              iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::dpprfs_(&(uplo as c_char), &n, &nrhs, ap.as_ptr(), afp.as_ptr(), b.as_ptr(), &ldb,
                     x.as_mut_ptr(), &ldx, ferr.as_mut_ptr(), berr.as_mut_ptr(), work.as_mut_ptr(),
                     iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn cpprfs(uplo: u8, n: i32, nrhs: i32, ap: &[c32], afp: &[c32], b: &[c32], ldb: i32,
              x: &mut [c32], ldx: i32, ferr: &mut [f32], berr: &mut [f32], work: &mut [c32],
              rwork: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::cpprfs_(&(uplo as c_char), &n, &nrhs, ap.as_ptr() as *const _,
                     afp.as_ptr() as *const _, b.as_ptr() as *const _, &ldb,
                     x.as_mut_ptr() as *mut _, &ldx, ferr.as_mut_ptr(), berr.as_mut_ptr(),
                     work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zpprfs(uplo: u8, n: i32, nrhs: i32, ap: &[c64], afp: &[c64], b: &[c64], ldb: i32,
              x: &mut [c64], ldx: i32, ferr: &mut [f64], berr: &mut [f64], work: &mut [c64],
              rwork: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::zpprfs_(&(uplo as c_char), &n, &nrhs, ap.as_ptr() as *const _,
                     afp.as_ptr() as *const _, b.as_ptr() as *const _, &ldb,
                     x.as_mut_ptr() as *mut _, &ldx, ferr.as_mut_ptr(), berr.as_mut_ptr(),
                     work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn spbrfs(uplo: u8, n: i32, kd: i32, nrhs: i32, ab: &[f32], ldab: i32, afb: &[f32], ldafb: i32,
              b: &[f32], ldb: i32, x: &mut [f32], ldx: i32, ferr: &mut [f32], berr: &mut [f32],
              work: &mut [f32], iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::spbrfs_(&(uplo as c_char), &n, &kd, &nrhs, ab.as_ptr(), &ldab, afb.as_ptr(), &ldafb,
                     b.as_ptr(), &ldb, x.as_mut_ptr(), &ldx, ferr.as_mut_ptr(), berr.as_mut_ptr(),
                     work.as_mut_ptr(), iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dpbrfs(uplo: u8, n: i32, kd: i32, nrhs: i32, ab: &[f64], ldab: i32, afb: &[f64], ldafb: i32,
              b: &[f64], ldb: i32, x: &mut [f64], ldx: i32, ferr: &mut [f64], berr: &mut [f64],
              work: &mut [f64], iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::dpbrfs_(&(uplo as c_char), &n, &kd, &nrhs, ab.as_ptr(), &ldab, afb.as_ptr(), &ldafb,
                     b.as_ptr(), &ldb, x.as_mut_ptr(), &ldx, ferr.as_mut_ptr(), berr.as_mut_ptr(),
                     work.as_mut_ptr(), iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn cpbrfs(uplo: u8, n: i32, kd: i32, nrhs: i32, ab: &[c32], ldab: i32, afb: &[c32], ldafb: i32,
              b: &[c32], ldb: i32, x: &mut [c32], ldx: i32, ferr: &mut [f32], berr: &mut [f32],
              work: &mut [c32], rwork: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::cpbrfs_(&(uplo as c_char), &n, &kd, &nrhs, ab.as_ptr() as *const _, &ldab,
                     afb.as_ptr() as *const _, &ldafb, b.as_ptr() as *const _, &ldb,
                     x.as_mut_ptr() as *mut _, &ldx, ferr.as_mut_ptr(), berr.as_mut_ptr(),
                     work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zpbrfs(uplo: u8, n: i32, kd: i32, nrhs: i32, ab: &[c64], ldab: i32, afb: &[c64], ldafb: i32,
              b: &[c64], ldb: i32, x: &mut [c64], ldx: i32, ferr: &mut [f64], berr: &mut [f64],
              work: &mut [c64], rwork: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::zpbrfs_(&(uplo as c_char), &n, &kd, &nrhs, ab.as_ptr() as *const _, &ldab,
                     afb.as_ptr() as *const _, &ldafb, b.as_ptr() as *const _, &ldb,
                     x.as_mut_ptr() as *mut _, &ldx, ferr.as_mut_ptr(), berr.as_mut_ptr(),
                     work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn sptrfs(n: i32, nrhs: i32, d: &[f32], e: &[f32], df: &[f32], ef: &[f32], b: &[f32], ldb: i32,
              x: &mut [f32], ldx: i32, ferr: &mut [f32], berr: &mut [f32], work: &mut [f32],
              info: &mut i32) {

    unsafe {
        ffi::sptrfs_(&n, &nrhs, d.as_ptr(), e.as_ptr(), df.as_ptr(), ef.as_ptr(), b.as_ptr(), &ldb,
                     x.as_mut_ptr(), &ldx, ferr.as_mut_ptr(), berr.as_mut_ptr(), work.as_mut_ptr(),
                     info)
    }
}

#[inline]
pub fn dptrfs(n: i32, nrhs: i32, d: &[f64], e: &[f64], df: &[f64], ef: &[f64], b: &[f64], ldb: i32,
              x: &mut [f64], ldx: i32, ferr: &mut [f64], berr: &mut [f64], work: &mut [f64],
              info: &mut i32) {

    unsafe {
        ffi::dptrfs_(&n, &nrhs, d.as_ptr(), e.as_ptr(), df.as_ptr(), ef.as_ptr(), b.as_ptr(), &ldb,
                     x.as_mut_ptr(), &ldx, ferr.as_mut_ptr(), berr.as_mut_ptr(), work.as_mut_ptr(),
                     info)
    }
}

#[inline]
pub fn cptrfs(uplo: u8, n: i32, nrhs: i32, d: &[f32], e: &[c32], df: &[f32], ef: &[c32], b: &[c32],
              ldb: i32, x: &mut [c32], ldx: i32, ferr: &mut [f32], berr: &mut [f32],
              work: &mut [c32], rwork: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::cptrfs_(&(uplo as c_char), &n, &nrhs, d.as_ptr(), e.as_ptr() as *const _, df.as_ptr(),
                     ef.as_ptr() as *const _, b.as_ptr() as *const _, &ldb,
                     x.as_mut_ptr() as *mut _, &ldx, ferr.as_mut_ptr(), berr.as_mut_ptr(),
                     work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zptrfs(uplo: u8, n: i32, nrhs: i32, d: &[f64], e: &[c64], df: &[f64], ef: &[c64], b: &[c64],
              ldb: i32, x: &mut [c64], ldx: i32, ferr: &mut [f64], berr: &mut [f64],
              work: &mut [c64], rwork: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::zptrfs_(&(uplo as c_char), &n, &nrhs, d.as_ptr(), e.as_ptr() as *const _, df.as_ptr(),
                     ef.as_ptr() as *const _, b.as_ptr() as *const _, &ldb,
                     x.as_mut_ptr() as *mut _, &ldx, ferr.as_mut_ptr(), berr.as_mut_ptr(),
                     work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn ssyrfs(uplo: u8, n: i32, nrhs: i32, a: &[f32], lda: i32, af: &[f32], ldaf: i32,
              ipiv: &[i32], b: &[f32], ldb: i32, x: &mut [f32], ldx: i32, ferr: &mut [f32],
              berr: &mut [f32], work: &mut [f32], iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::ssyrfs_(&(uplo as c_char), &n, &nrhs, a.as_ptr(), &lda, af.as_ptr(), &ldaf,
                     ipiv.as_ptr(), b.as_ptr(), &ldb, x.as_mut_ptr(), &ldx, ferr.as_mut_ptr(),
                     berr.as_mut_ptr(), work.as_mut_ptr(), iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dsyrfs(uplo: u8, n: i32, nrhs: i32, a: &[f64], lda: i32, af: &[f64], ldaf: i32,
              ipiv: &[i32], b: &[f64], ldb: i32, x: &mut [f64], ldx: i32, ferr: &mut [f64],
              berr: &mut [f64], work: &mut [f64], iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::dsyrfs_(&(uplo as c_char), &n, &nrhs, a.as_ptr(), &lda, af.as_ptr(), &ldaf,
                     ipiv.as_ptr(), b.as_ptr(), &ldb, x.as_mut_ptr(), &ldx, ferr.as_mut_ptr(),
                     berr.as_mut_ptr(), work.as_mut_ptr(), iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn csyrfs(uplo: u8, n: i32, nrhs: i32, a: &[c32], lda: i32, af: &[c32], ldaf: i32,
              ipiv: &[i32], b: &[c32], ldb: i32, x: &mut [c32], ldx: i32, ferr: &mut [f32],
              berr: &mut [f32], work: &mut [c32], rwork: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::csyrfs_(&(uplo as c_char), &n, &nrhs, a.as_ptr() as *const _, &lda,
                     af.as_ptr() as *const _, &ldaf, ipiv.as_ptr(), b.as_ptr() as *const _, &ldb,
                     x.as_mut_ptr() as *mut _, &ldx, ferr.as_mut_ptr(), berr.as_mut_ptr(),
                     work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zsyrfs(uplo: u8, n: i32, nrhs: i32, a: &[c64], lda: i32, af: &[c64], ldaf: i32,
              ipiv: &[i32], b: &[c64], ldb: i32, x: &mut [c64], ldx: i32, ferr: &mut [f64],
              berr: &mut [f64], work: &mut [c64], rwork: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::zsyrfs_(&(uplo as c_char), &n, &nrhs, a.as_ptr() as *const _, &lda,
                     af.as_ptr() as *const _, &ldaf, ipiv.as_ptr(), b.as_ptr() as *const _, &ldb,
                     x.as_mut_ptr() as *mut _, &ldx, ferr.as_mut_ptr(), berr.as_mut_ptr(),
                     work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dsyrfsx(uplo: u8, equed: u8, n: i32, nrhs: i32, a: &[f64], lda: i32, af: &[f64], ldaf: i32,
               ipiv: &[i32], s: &[f64], b: &[f64], ldb: i32, x: &mut [f64], ldx: i32,
               rcond: &mut f64, berr: &mut [f64], n_err_bnds: i32, err_bnds_norm: &mut [f64],
               err_bnds_comp: &mut [f64], nparams: &[i32], params: &mut [f64], work: &mut [f64],
               iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::dsyrfsx_(&(uplo as c_char), &(equed as c_char), &n, &nrhs, a.as_ptr(), &lda,
                      af.as_ptr(), &ldaf, ipiv.as_ptr(), s.as_ptr(), b.as_ptr(), &ldb,
                      x.as_mut_ptr(), &ldx, rcond, berr.as_mut_ptr(), &n_err_bnds,
                      err_bnds_norm.as_mut_ptr(), err_bnds_comp.as_mut_ptr(), nparams.as_ptr(),
                      params.as_mut_ptr(), work.as_mut_ptr(), iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn ssyrfsx(uplo: u8, equed: u8, n: i32, nrhs: i32, a: &[f32], lda: i32, af: &[f32], ldaf: i32,
               ipiv: &[i32], s: &[f32], b: &[f32], ldb: i32, x: &mut [f32], ldx: i32,
               rcond: &mut f32, berr: &mut [f32], n_err_bnds: i32, err_bnds_norm: &mut [f32],
               err_bnds_comp: &mut [f32], nparams: &[i32], params: &mut [f32], work: &mut [f32],
               iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::ssyrfsx_(&(uplo as c_char), &(equed as c_char), &n, &nrhs, a.as_ptr(), &lda,
                      af.as_ptr(), &ldaf, ipiv.as_ptr(), s.as_ptr(), b.as_ptr(), &ldb,
                      x.as_mut_ptr(), &ldx, rcond, berr.as_mut_ptr(), &n_err_bnds,
                      err_bnds_norm.as_mut_ptr(), err_bnds_comp.as_mut_ptr(), nparams.as_ptr(),
                      params.as_mut_ptr(), work.as_mut_ptr(), iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zsyrfsx(uplo: u8, equed: u8, n: i32, nrhs: i32, a: &[c64], lda: i32, af: &[c64], ldaf: i32,
               ipiv: &[i32], s: &[f64], b: &[c64], ldb: i32, x: &mut [c64], ldx: i32,
               rcond: &mut f64, berr: &mut [f64], n_err_bnds: i32, err_bnds_norm: &mut [f64],
               err_bnds_comp: &mut [f64], nparams: &[i32], params: &mut [f64], work: &mut [c64],
               rwork: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::zsyrfsx_(&(uplo as c_char), &(equed as c_char), &n, &nrhs, a.as_ptr() as *const _,
                      &lda, af.as_ptr() as *const _, &ldaf, ipiv.as_ptr(), s.as_ptr(),
                      b.as_ptr() as *const _, &ldb, x.as_mut_ptr() as *mut _, &ldx, rcond,
                      berr.as_mut_ptr(), &n_err_bnds, err_bnds_norm.as_mut_ptr(),
                      err_bnds_comp.as_mut_ptr(), nparams.as_ptr(), params.as_mut_ptr(),
                      work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn csyrfsx(uplo: u8, equed: u8, n: i32, nrhs: i32, a: &[c32], lda: i32, af: &[c32], ldaf: i32,
               ipiv: &[i32], s: &[f32], b: &[c32], ldb: i32, x: &mut [c32], ldx: i32,
               rcond: &mut f32, berr: &mut [f32], n_err_bnds: i32, err_bnds_norm: &mut [f32],
               err_bnds_comp: &mut [f32], nparams: &[i32], params: &mut [f32], work: &mut [c32],
               rwork: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::csyrfsx_(&(uplo as c_char), &(equed as c_char), &n, &nrhs, a.as_ptr() as *const _,
                      &lda, af.as_ptr() as *const _, &ldaf, ipiv.as_ptr(), s.as_ptr(),
                      b.as_ptr() as *const _, &ldb, x.as_mut_ptr() as *mut _, &ldx, rcond,
                      berr.as_mut_ptr(), &n_err_bnds, err_bnds_norm.as_mut_ptr(),
                      err_bnds_comp.as_mut_ptr(), nparams.as_ptr(), params.as_mut_ptr(),
                      work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn cherfs(uplo: u8, n: i32, nrhs: i32, a: &[c32], lda: i32, af: &[c32], ldaf: i32,
              ipiv: &[i32], b: &[c32], ldb: i32, x: &mut [c32], ldx: i32, ferr: &mut [f32],
              berr: &mut [f32], work: &mut [c32], rwork: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::cherfs_(&(uplo as c_char), &n, &nrhs, a.as_ptr() as *const _, &lda,
                     af.as_ptr() as *const _, &ldaf, ipiv.as_ptr(), b.as_ptr() as *const _, &ldb,
                     x.as_mut_ptr() as *mut _, &ldx, ferr.as_mut_ptr(), berr.as_mut_ptr(),
                     work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zherfs(uplo: u8, n: i32, nrhs: i32, a: &[c64], lda: i32, af: &[c64], ldaf: i32,
              ipiv: &[i32], b: &[c64], ldb: i32, x: &mut [c64], ldx: i32, ferr: &mut [f64],
              berr: &mut [f64], work: &mut [c64], rwork: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::zherfs_(&(uplo as c_char), &n, &nrhs, a.as_ptr() as *const _, &lda,
                     af.as_ptr() as *const _, &ldaf, ipiv.as_ptr(), b.as_ptr() as *const _, &ldb,
                     x.as_mut_ptr() as *mut _, &ldx, ferr.as_mut_ptr(), berr.as_mut_ptr(),
                     work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zherfsx(uplo: u8, equed: u8, n: i32, nrhs: i32, a: &[c64], lda: i32, af: &[c64], ldaf: i32,
               ipiv: &[i32], s: &[f64], b: &[c64], ldb: i32, x: &mut [c64], ldx: i32,
               rcond: &mut f64, berr: &mut [f64], n_err_bnds: i32, err_bnds_norm: &mut [f64],
               err_bnds_comp: &mut [f64], nparams: &[i32], params: &mut [f64], work: &mut [c64],
               rwork: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::zherfsx_(&(uplo as c_char), &(equed as c_char), &n, &nrhs, a.as_ptr() as *const _,
                      &lda, af.as_ptr() as *const _, &ldaf, ipiv.as_ptr(), s.as_ptr(),
                      b.as_ptr() as *const _, &ldb, x.as_mut_ptr() as *mut _, &ldx, rcond,
                      berr.as_mut_ptr(), &n_err_bnds, err_bnds_norm.as_mut_ptr(),
                      err_bnds_comp.as_mut_ptr(), nparams.as_ptr(), params.as_mut_ptr(),
                      work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn cherfsx(uplo: u8, equed: u8, n: i32, nrhs: i32, a: &[c32], lda: i32, af: &[c32], ldaf: i32,
               ipiv: &[i32], s: &[f32], b: &[c32], ldb: i32, x: &mut [c32], ldx: i32,
               rcond: &mut f32, berr: &mut [f32], n_err_bnds: i32, err_bnds_norm: &mut [f32],
               err_bnds_comp: &mut [f32], nparams: &[i32], params: &mut [f32], work: &mut [c32],
               rwork: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::cherfsx_(&(uplo as c_char), &(equed as c_char), &n, &nrhs, a.as_ptr() as *const _,
                      &lda, af.as_ptr() as *const _, &ldaf, ipiv.as_ptr(), s.as_ptr(),
                      b.as_ptr() as *const _, &ldb, x.as_mut_ptr() as *mut _, &ldx, rcond,
                      berr.as_mut_ptr(), &n_err_bnds, err_bnds_norm.as_mut_ptr(),
                      err_bnds_comp.as_mut_ptr(), nparams.as_ptr(), params.as_mut_ptr(),
                      work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn ssprfs(uplo: u8, n: i32, nrhs: i32, ap: &[f32], afp: &[f32], ipiv: &[i32], b: &[f32],
              ldb: i32, x: &mut [f32], ldx: i32, ferr: &mut [f32], berr: &mut [f32],
              work: &mut [f32], iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::ssprfs_(&(uplo as c_char), &n, &nrhs, ap.as_ptr(), afp.as_ptr(), ipiv.as_ptr(),
                     b.as_ptr(), &ldb, x.as_mut_ptr(), &ldx, ferr.as_mut_ptr(), berr.as_mut_ptr(),
                     work.as_mut_ptr(), iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dsprfs(uplo: u8, n: i32, nrhs: i32, ap: &[f64], afp: &[f64], ipiv: &[i32], b: &[f64],
              ldb: i32, x: &mut [f64], ldx: i32, ferr: &mut [f64], berr: &mut [f64],
              work: &mut [f64], iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::dsprfs_(&(uplo as c_char), &n, &nrhs, ap.as_ptr(), afp.as_ptr(), ipiv.as_ptr(),
                     b.as_ptr(), &ldb, x.as_mut_ptr(), &ldx, ferr.as_mut_ptr(), berr.as_mut_ptr(),
                     work.as_mut_ptr(), iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn csprfs(uplo: u8, n: i32, nrhs: i32, ap: &[c32], afp: &[c32], ipiv: &[i32], b: &[c32],
              ldb: i32, x: &mut [c32], ldx: i32, ferr: &mut [f32], berr: &mut [f32],
              work: &mut [c32], rwork: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::csprfs_(&(uplo as c_char), &n, &nrhs, ap.as_ptr() as *const _,
                     afp.as_ptr() as *const _, ipiv.as_ptr(), b.as_ptr() as *const _, &ldb,
                     x.as_mut_ptr() as *mut _, &ldx, ferr.as_mut_ptr(), berr.as_mut_ptr(),
                     work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zsprfs(uplo: u8, n: i32, nrhs: i32, ap: &[c64], afp: &[c64], ipiv: &[i32], b: &[c64],
              ldb: i32, x: &mut [c64], ldx: i32, ferr: &mut [f64], berr: &mut [f64],
              work: &mut [c64], rwork: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::zsprfs_(&(uplo as c_char), &n, &nrhs, ap.as_ptr() as *const _,
                     afp.as_ptr() as *const _, ipiv.as_ptr(), b.as_ptr() as *const _, &ldb,
                     x.as_mut_ptr() as *mut _, &ldx, ferr.as_mut_ptr(), berr.as_mut_ptr(),
                     work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn chprfs(uplo: u8, n: i32, nrhs: i32, ap: &[c32], afp: &[c32], ipiv: &[i32], b: &[c32],
              ldb: i32, x: &mut [c32], ldx: i32, ferr: &mut [f32], berr: &mut [f32],
              work: &mut [c32], rwork: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::chprfs_(&(uplo as c_char), &n, &nrhs, ap.as_ptr() as *const _,
                     afp.as_ptr() as *const _, ipiv.as_ptr(), b.as_ptr() as *const _, &ldb,
                     x.as_mut_ptr() as *mut _, &ldx, ferr.as_mut_ptr(), berr.as_mut_ptr(),
                     work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zhprfs(uplo: u8, n: i32, nrhs: i32, ap: &[c64], afp: &[c64], ipiv: &[i32], b: &[c64],
              ldb: i32, x: &mut [c64], ldx: i32, ferr: &mut [f64], berr: &mut [f64],
              work: &mut [c64], rwork: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::zhprfs_(&(uplo as c_char), &n, &nrhs, ap.as_ptr() as *const _,
                     afp.as_ptr() as *const _, ipiv.as_ptr(), b.as_ptr() as *const _, &ldb,
                     x.as_mut_ptr() as *mut _, &ldx, ferr.as_mut_ptr(), berr.as_mut_ptr(),
                     work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn strrfs(uplo: u8, trans: u8, diag: u8, n: i32, nrhs: i32, a: &[f32], lda: i32, b: &[f32],
              ldb: i32, x: &[f32], ldx: i32, ferr: &mut [f32], berr: &mut [f32], work: &mut [f32],
              iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::strrfs_(&(uplo as c_char), &(trans as c_char), &(diag as c_char), &n, &nrhs,
                     a.as_ptr(), &lda, b.as_ptr(), &ldb, x.as_ptr(), &ldx, ferr.as_mut_ptr(),
                     berr.as_mut_ptr(), work.as_mut_ptr(), iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dtrrfs(uplo: u8, trans: u8, diag: u8, n: i32, nrhs: i32, a: &[f64], lda: i32, b: &[f64],
              ldb: i32, x: &[f64], ldx: i32, ferr: &mut [f64], berr: &mut [f64], work: &mut [f64],
              iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::dtrrfs_(&(uplo as c_char), &(trans as c_char), &(diag as c_char), &n, &nrhs,
                     a.as_ptr(), &lda, b.as_ptr(), &ldb, x.as_ptr(), &ldx, ferr.as_mut_ptr(),
                     berr.as_mut_ptr(), work.as_mut_ptr(), iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn ctrrfs(uplo: u8, trans: u8, diag: u8, n: i32, nrhs: i32, a: &[c32], lda: i32, b: &[c32],
              ldb: i32, x: &[c32], ldx: i32, ferr: &mut [f32], berr: &mut [f32], work: &mut [c32],
              rwork: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::ctrrfs_(&(uplo as c_char), &(trans as c_char), &(diag as c_char), &n, &nrhs,
                     a.as_ptr() as *const _, &lda, b.as_ptr() as *const _, &ldb,
                     x.as_ptr() as *const _, &ldx, ferr.as_mut_ptr(), berr.as_mut_ptr(),
                     work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn ztrrfs(uplo: u8, trans: u8, diag: u8, n: i32, nrhs: i32, a: &[c64], lda: i32, b: &[c64],
              ldb: i32, x: &[c64], ldx: i32, ferr: &mut [f64], berr: &mut [f64], work: &mut [c64],
              rwork: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::ztrrfs_(&(uplo as c_char), &(trans as c_char), &(diag as c_char), &n, &nrhs,
                     a.as_ptr() as *const _, &lda, b.as_ptr() as *const _, &ldb,
                     x.as_ptr() as *const _, &ldx, ferr.as_mut_ptr(), berr.as_mut_ptr(),
                     work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn stprfs(uplo: u8, trans: u8, diag: u8, n: i32, nrhs: i32, ap: &[f32], b: &[f32], ldb: i32,
              x: &[f32], ldx: i32, ferr: &mut [f32], berr: &mut [f32], work: &mut [f32],
              iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::stprfs_(&(uplo as c_char), &(trans as c_char), &(diag as c_char), &n, &nrhs,
                     ap.as_ptr(), b.as_ptr(), &ldb, x.as_ptr(), &ldx, ferr.as_mut_ptr(),
                     berr.as_mut_ptr(), work.as_mut_ptr(), iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dtprfs(uplo: u8, trans: u8, diag: u8, n: i32, nrhs: i32, ap: &[f64], b: &[f64], ldb: i32,
              x: &[f64], ldx: i32, ferr: &mut [f64], berr: &mut [f64], work: &mut [f64],
              iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::dtprfs_(&(uplo as c_char), &(trans as c_char), &(diag as c_char), &n, &nrhs,
                     ap.as_ptr(), b.as_ptr(), &ldb, x.as_ptr(), &ldx, ferr.as_mut_ptr(),
                     berr.as_mut_ptr(), work.as_mut_ptr(), iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn ctprfs(uplo: u8, trans: u8, diag: u8, n: i32, nrhs: i32, ap: &[c32], b: &[c32], ldb: i32,
              x: &[c32], ldx: i32, ferr: &mut [f32], berr: &mut [f32], work: &mut [c32],
              rwork: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::ctprfs_(&(uplo as c_char), &(trans as c_char), &(diag as c_char), &n, &nrhs,
                     ap.as_ptr() as *const _, b.as_ptr() as *const _, &ldb, x.as_ptr() as *const _,
                     &ldx, ferr.as_mut_ptr(), berr.as_mut_ptr(), work.as_mut_ptr() as *mut _,
                     rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn ztprfs(uplo: u8, trans: u8, diag: u8, n: i32, nrhs: i32, ap: &[c64], b: &[c64], ldb: i32,
              x: &[c64], ldx: i32, ferr: &mut [f64], berr: &mut [f64], work: &mut [c64],
              rwork: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::ztprfs_(&(uplo as c_char), &(trans as c_char), &(diag as c_char), &n, &nrhs,
                     ap.as_ptr() as *const _, b.as_ptr() as *const _, &ldb, x.as_ptr() as *const _,
                     &ldx, ferr.as_mut_ptr(), berr.as_mut_ptr(), work.as_mut_ptr() as *mut _,
                     rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn stbrfs(uplo: u8, trans: u8, diag: u8, n: i32, kd: i32, nrhs: i32, ab: &[f32], ldab: i32,
              b: &[f32], ldb: i32, x: &[f32], ldx: i32, ferr: &mut [f32], berr: &mut [f32],
              work: &mut [f32], iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::stbrfs_(&(uplo as c_char), &(trans as c_char), &(diag as c_char), &n, &kd, &nrhs,
                     ab.as_ptr(), &ldab, b.as_ptr(), &ldb, x.as_ptr(), &ldx, ferr.as_mut_ptr(),
                     berr.as_mut_ptr(), work.as_mut_ptr(), iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dtbrfs(uplo: u8, trans: u8, diag: u8, n: i32, kd: i32, nrhs: i32, ab: &[f64], ldab: i32,
              b: &[f64], ldb: i32, x: &[f64], ldx: i32, ferr: &mut [f64], berr: &mut [f64],
              work: &mut [f64], iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::dtbrfs_(&(uplo as c_char), &(trans as c_char), &(diag as c_char), &n, &kd, &nrhs,
                     ab.as_ptr(), &ldab, b.as_ptr(), &ldb, x.as_ptr(), &ldx, ferr.as_mut_ptr(),
                     berr.as_mut_ptr(), work.as_mut_ptr(), iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn ctbrfs(uplo: u8, trans: u8, diag: u8, n: i32, kd: i32, nrhs: i32, ab: &[c32], ldab: i32,
              b: &[c32], ldb: i32, x: &[c32], ldx: i32, ferr: &mut [f32], berr: &mut [f32],
              work: &mut [c32], rwork: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::ctbrfs_(&(uplo as c_char), &(trans as c_char), &(diag as c_char), &n, &kd, &nrhs,
                     ab.as_ptr() as *const _, &ldab, b.as_ptr() as *const _, &ldb,
                     x.as_ptr() as *const _, &ldx, ferr.as_mut_ptr(), berr.as_mut_ptr(),
                     work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn ztbrfs(uplo: u8, trans: u8, diag: u8, n: i32, kd: i32, nrhs: i32, ab: &[c64], ldab: i32,
              b: &[c64], ldb: i32, x: &[c64], ldx: i32, ferr: &mut [f64], berr: &mut [f64],
              work: &mut [c64], rwork: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::ztbrfs_(&(uplo as c_char), &(trans as c_char), &(diag as c_char), &n, &kd, &nrhs,
                     ab.as_ptr() as *const _, &ldab, b.as_ptr() as *const _, &ldb,
                     x.as_ptr() as *const _, &ldx, ferr.as_mut_ptr(), berr.as_mut_ptr(),
                     work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn sgetri(n: i32, a: &mut [f32], lda: i32, ipiv: &[i32], work: &mut [f32], lwork: i32,
              info: &mut i32) {

    unsafe {
        ffi::sgetri_(&n, a.as_mut_ptr(), &lda, ipiv.as_ptr(), work.as_mut_ptr(), &lwork, info)
    }
}

#[inline]
pub fn dgetri(n: i32, a: &mut [f64], lda: i32, ipiv: &[i32], work: &mut [f64], lwork: i32,
              info: &mut i32) {

    unsafe {
        ffi::dgetri_(&n, a.as_mut_ptr(), &lda, ipiv.as_ptr(), work.as_mut_ptr(), &lwork, info)
    }
}

#[inline]
pub fn cgetri(n: i32, a: &mut [c32], lda: i32, ipiv: &[i32], work: &mut [c32], lwork: i32,
              info: &mut i32) {

    unsafe {
        ffi::cgetri_(&n, a.as_mut_ptr() as *mut _, &lda, ipiv.as_ptr(),
                     work.as_mut_ptr() as *mut _, &lwork, info)
    }
}

#[inline]
pub fn zgetri(n: i32, a: &mut [c64], lda: i32, ipiv: &[i32], work: &mut [c64], lwork: i32,
              info: &mut i32) {

    unsafe {
        ffi::zgetri_(&n, a.as_mut_ptr() as *mut _, &lda, ipiv.as_ptr(),
                     work.as_mut_ptr() as *mut _, &lwork, info)
    }
}

#[inline]
pub fn spotri(uplo: u8, n: i32, a: &mut [f32], lda: i32, info: &mut i32) {
    unsafe {
        ffi::spotri_(&(uplo as c_char), &n, a.as_mut_ptr(), &lda, info)
    }
}

#[inline]
pub fn dpotri(uplo: u8, n: i32, a: &mut [f64], lda: i32, info: &mut i32) {
    unsafe {
        ffi::dpotri_(&(uplo as c_char), &n, a.as_mut_ptr(), &lda, info)
    }
}

#[inline]
pub fn cpotri(uplo: u8, n: i32, a: &mut [c32], lda: i32, info: &mut i32) {
    unsafe {
        ffi::cpotri_(&(uplo as c_char), &n, a.as_mut_ptr() as *mut _, &lda, info)
    }
}

#[inline]
pub fn zpotri(uplo: u8, n: i32, a: &mut [c64], lda: i32, info: &mut i32) {
    unsafe {
        ffi::zpotri_(&(uplo as c_char), &n, a.as_mut_ptr() as *mut _, &lda, info)
    }
}

#[inline]
pub fn dpftri(transr: u8, uplo: u8, n: i32, a: &mut [f64], info: &mut i32) {
    unsafe {
        ffi::dpftri_(&(transr as c_char), &(uplo as c_char), &n, a.as_mut_ptr(), info)
    }
}

#[inline]
pub fn spftri(transr: u8, uplo: u8, n: i32, a: &mut [f32], info: &mut i32) {
    unsafe {
        ffi::spftri_(&(transr as c_char), &(uplo as c_char), &n, a.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zpftri(transr: u8, uplo: u8, n: i32, a: &mut [c64], info: &mut i32) {
    unsafe {
        ffi::zpftri_(&(transr as c_char), &(uplo as c_char), &n, a.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn cpftri(transr: u8, uplo: u8, n: i32, a: &mut [c32], info: &mut i32) {
    unsafe {
        ffi::cpftri_(&(transr as c_char), &(uplo as c_char), &n, a.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn spptri(uplo: u8, n: i32, ap: &mut [f32], info: &mut i32) {
    unsafe {
        ffi::spptri_(&(uplo as c_char), &n, ap.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dpptri(uplo: u8, n: i32, ap: &mut [f64], info: &mut i32) {
    unsafe {
        ffi::dpptri_(&(uplo as c_char), &n, ap.as_mut_ptr(), info)
    }
}

#[inline]
pub fn cpptri(uplo: u8, n: i32, ap: &mut [c32], info: &mut i32) {
    unsafe {
        ffi::cpptri_(&(uplo as c_char), &n, ap.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn zpptri(uplo: u8, n: i32, ap: &mut [c64], info: &mut i32) {
    unsafe {
        ffi::zpptri_(&(uplo as c_char), &n, ap.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn ssytri(uplo: u8, n: i32, a: &mut [f32], lda: i32, ipiv: &[i32], work: &mut [f32],
              info: &mut i32) {

    unsafe {
        ffi::ssytri_(&(uplo as c_char), &n, a.as_mut_ptr(), &lda, ipiv.as_ptr(), work.as_mut_ptr(),
                     info)
    }
}

#[inline]
pub fn dsytri(uplo: u8, n: i32, a: &mut [f64], lda: i32, ipiv: &[i32], work: &mut [f64],
              info: &mut i32) {

    unsafe {
        ffi::dsytri_(&(uplo as c_char), &n, a.as_mut_ptr(), &lda, ipiv.as_ptr(), work.as_mut_ptr(),
                     info)
    }
}

#[inline]
pub fn csytri(uplo: u8, n: i32, a: &mut [c32], lda: i32, ipiv: &[i32], work: &mut [c32],
              info: &mut i32) {

    unsafe {
        ffi::csytri_(&(uplo as c_char), &n, a.as_mut_ptr() as *mut _, &lda, ipiv.as_ptr(),
                     work.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn zsytri(uplo: u8, n: i32, a: &mut [c64], lda: i32, ipiv: &[i32], work: &mut [c64],
              info: &mut i32) {

    unsafe {
        ffi::zsytri_(&(uplo as c_char), &n, a.as_mut_ptr() as *mut _, &lda, ipiv.as_ptr(),
                     work.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn chetri(uplo: u8, n: i32, a: &mut [c32], lda: i32, ipiv: &[i32], work: &mut [c32],
              info: &mut i32) {

    unsafe {
        ffi::chetri_(&(uplo as c_char), &n, a.as_mut_ptr() as *mut _, &lda, ipiv.as_ptr(),
                     work.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn zhetri(uplo: u8, n: i32, a: &mut [c64], lda: i32, ipiv: &[i32], work: &mut [c64],
              info: &mut i32) {

    unsafe {
        ffi::zhetri_(&(uplo as c_char), &n, a.as_mut_ptr() as *mut _, &lda, ipiv.as_ptr(),
                     work.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn ssptri(uplo: u8, n: i32, ap: &mut [f32], ipiv: &[i32], work: &mut [f32], info: &mut i32) {
    unsafe {
        ffi::ssptri_(&(uplo as c_char), &n, ap.as_mut_ptr(), ipiv.as_ptr(), work.as_mut_ptr(),
                     info)
    }
}

#[inline]
pub fn dsptri(uplo: u8, n: i32, ap: &mut [f64], ipiv: &[i32], work: &mut [f64], info: &mut i32) {
    unsafe {
        ffi::dsptri_(&(uplo as c_char), &n, ap.as_mut_ptr(), ipiv.as_ptr(), work.as_mut_ptr(),
                     info)
    }
}

#[inline]
pub fn csptri(uplo: u8, n: i32, ap: &mut [c32], ipiv: &[i32], work: &mut [c32], info: &mut i32) {
    unsafe {
        ffi::csptri_(&(uplo as c_char), &n, ap.as_mut_ptr() as *mut _, ipiv.as_ptr(),
                     work.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn zsptri(uplo: u8, n: i32, ap: &mut [c64], ipiv: &[i32], work: &mut [c64], info: &mut i32) {
    unsafe {
        ffi::zsptri_(&(uplo as c_char), &n, ap.as_mut_ptr() as *mut _, ipiv.as_ptr(),
                     work.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn chptri(uplo: u8, n: i32, ap: &mut [c32], ipiv: &[i32], work: &mut [c32], info: &mut i32) {
    unsafe {
        ffi::chptri_(&(uplo as c_char), &n, ap.as_mut_ptr() as *mut _, ipiv.as_ptr(),
                     work.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn zhptri(uplo: u8, n: i32, ap: &mut [c64], ipiv: &[i32], work: &mut [c64], info: &mut i32) {
    unsafe {
        ffi::zhptri_(&(uplo as c_char), &n, ap.as_mut_ptr() as *mut _, ipiv.as_ptr(),
                     work.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn strtri(uplo: u8, diag: u8, n: i32, a: &mut [f32], lda: i32, info: &mut i32) {
    unsafe {
        ffi::strtri_(&(uplo as c_char), &(diag as c_char), &n, a.as_mut_ptr(), &lda, info)
    }
}

#[inline]
pub fn dtrtri(uplo: u8, diag: u8, n: i32, a: &mut [f64], lda: i32, info: &mut i32) {
    unsafe {
        ffi::dtrtri_(&(uplo as c_char), &(diag as c_char), &n, a.as_mut_ptr(), &lda, info)
    }
}

#[inline]
pub fn ctrtri(uplo: u8, diag: u8, n: i32, a: &mut [c32], lda: i32, info: &mut i32) {
    unsafe {
        ffi::ctrtri_(&(uplo as c_char), &(diag as c_char), &n, a.as_mut_ptr() as *mut _, &lda,
                     info)
    }
}

#[inline]
pub fn ztrtri(uplo: u8, diag: u8, n: i32, a: &mut [c64], lda: i32, info: &mut i32) {
    unsafe {
        ffi::ztrtri_(&(uplo as c_char), &(diag as c_char), &n, a.as_mut_ptr() as *mut _, &lda,
                     info)
    }
}

#[inline]
pub fn dtftri(transr: u8, uplo: u8, diag: u8, n: i32, a: &mut [f64], info: &mut i32) {
    unsafe {
        ffi::dtftri_(&(transr as c_char), &(uplo as c_char), &(diag as c_char), &n, a.as_mut_ptr(),
                     info)
    }
}

#[inline]
pub fn stftri(transr: u8, uplo: u8, diag: u8, n: i32, a: &mut [f32], info: &mut i32) {
    unsafe {
        ffi::stftri_(&(transr as c_char), &(uplo as c_char), &(diag as c_char), &n, a.as_mut_ptr(),
                     info)
    }
}

#[inline]
pub fn ztftri(transr: u8, uplo: u8, diag: u8, n: i32, a: &mut [c64], info: &mut i32) {
    unsafe {
        ffi::ztftri_(&(transr as c_char), &(uplo as c_char), &(diag as c_char), &n,
                     a.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn ctftri(transr: u8, uplo: u8, diag: u8, n: i32, a: &mut [c32], info: &mut i32) {
    unsafe {
        ffi::ctftri_(&(transr as c_char), &(uplo as c_char), &(diag as c_char), &n,
                     a.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn stptri(uplo: u8, diag: u8, n: i32, ap: &mut [f32], info: &mut i32) {
    unsafe {
        ffi::stptri_(&(uplo as c_char), &(diag as c_char), &n, ap.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dtptri(uplo: u8, diag: u8, n: i32, ap: &mut [f64], info: &mut i32) {
    unsafe {
        ffi::dtptri_(&(uplo as c_char), &(diag as c_char), &n, ap.as_mut_ptr(), info)
    }
}

#[inline]
pub fn ctptri(uplo: u8, diag: u8, n: i32, ap: &mut [c32], info: &mut i32) {
    unsafe {
        ffi::ctptri_(&(uplo as c_char), &(diag as c_char), &n, ap.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn ztptri(uplo: u8, diag: u8, n: i32, ap: &mut [c64], info: &mut i32) {
    unsafe {
        ffi::ztptri_(&(uplo as c_char), &(diag as c_char), &n, ap.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn sgeequ(m: i32, n: i32, a: &[f32], lda: i32, r: &mut [f32], c: &mut [f32],
              rowcnd: &mut [f32], colcnd: &mut [f32], amax: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::sgeequ_(&m, &n, a.as_ptr(), &lda, r.as_mut_ptr(), c.as_mut_ptr(), rowcnd.as_mut_ptr(),
                     colcnd.as_mut_ptr(), amax.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dgeequ(m: i32, n: i32, a: &[f64], lda: i32, r: &mut [f64], c: &mut [f64],
              rowcnd: &mut [f64], colcnd: &mut [f64], amax: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::dgeequ_(&m, &n, a.as_ptr(), &lda, r.as_mut_ptr(), c.as_mut_ptr(), rowcnd.as_mut_ptr(),
                     colcnd.as_mut_ptr(), amax.as_mut_ptr(), info)
    }
}

#[inline]
pub fn cgeequ(m: i32, n: i32, a: &[c32], lda: i32, r: &mut [f32], c: &mut [f32],
              rowcnd: &mut [f32], colcnd: &mut [f32], amax: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::cgeequ_(&m, &n, a.as_ptr() as *const _, &lda, r.as_mut_ptr(), c.as_mut_ptr(),
                     rowcnd.as_mut_ptr(), colcnd.as_mut_ptr(), amax.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zgeequ(m: i32, n: i32, a: &[c64], lda: i32, r: &mut [f64], c: &mut [f64],
              rowcnd: &mut [f64], colcnd: &mut [f64], amax: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::zgeequ_(&m, &n, a.as_ptr() as *const _, &lda, r.as_mut_ptr(), c.as_mut_ptr(),
                     rowcnd.as_mut_ptr(), colcnd.as_mut_ptr(), amax.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dgeequb(m: i32, n: i32, a: &[f64], lda: i32, r: &mut [f64], c: &mut [f64],
               rowcnd: &mut [f64], colcnd: &mut [f64], amax: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::dgeequb_(&m, &n, a.as_ptr(), &lda, r.as_mut_ptr(), c.as_mut_ptr(),
                      rowcnd.as_mut_ptr(), colcnd.as_mut_ptr(), amax.as_mut_ptr(), info)
    }
}

#[inline]
pub fn sgeequb(m: i32, n: i32, a: &[f32], lda: i32, r: &mut [f32], c: &mut [f32],
               rowcnd: &mut [f32], colcnd: &mut [f32], amax: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::sgeequb_(&m, &n, a.as_ptr(), &lda, r.as_mut_ptr(), c.as_mut_ptr(),
                      rowcnd.as_mut_ptr(), colcnd.as_mut_ptr(), amax.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zgeequb(m: i32, n: i32, a: &[c64], lda: i32, r: &mut [f64], c: &mut [f64],
               rowcnd: &mut [f64], colcnd: &mut [f64], amax: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::zgeequb_(&m, &n, a.as_ptr() as *const _, &lda, r.as_mut_ptr(), c.as_mut_ptr(),
                      rowcnd.as_mut_ptr(), colcnd.as_mut_ptr(), amax.as_mut_ptr(), info)
    }
}

#[inline]
pub fn cgeequb(m: i32, n: i32, a: &[c32], lda: i32, r: &mut [f32], c: &mut [f32],
               rowcnd: &mut [f32], colcnd: &mut [f32], amax: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::cgeequb_(&m, &n, a.as_ptr() as *const _, &lda, r.as_mut_ptr(), c.as_mut_ptr(),
                      rowcnd.as_mut_ptr(), colcnd.as_mut_ptr(), amax.as_mut_ptr(), info)
    }
}

#[inline]
pub fn sgbequ(m: i32, n: i32, kl: i32, ku: i32, ab: &[f32], ldab: i32, r: &mut [f32],
              c: &mut [f32], rowcnd: &mut [f32], colcnd: &mut [f32], amax: &mut [f32],
              info: &mut i32) {

    unsafe {
        ffi::sgbequ_(&m, &n, &kl, &ku, ab.as_ptr(), &ldab, r.as_mut_ptr(), c.as_mut_ptr(),
                     rowcnd.as_mut_ptr(), colcnd.as_mut_ptr(), amax.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dgbequ(m: i32, n: i32, kl: i32, ku: i32, ab: &[f64], ldab: i32, r: &mut [f64],
              c: &mut [f64], rowcnd: &mut [f64], colcnd: &mut [f64], amax: &mut [f64],
              info: &mut i32) {

    unsafe {
        ffi::dgbequ_(&m, &n, &kl, &ku, ab.as_ptr(), &ldab, r.as_mut_ptr(), c.as_mut_ptr(),
                     rowcnd.as_mut_ptr(), colcnd.as_mut_ptr(), amax.as_mut_ptr(), info)
    }
}

#[inline]
pub fn cgbequ(m: i32, n: i32, kl: i32, ku: i32, ab: &[c32], ldab: i32, r: &mut [f32],
              c: &mut [f32], rowcnd: &mut [f32], colcnd: &mut [f32], amax: &mut [f32],
              info: &mut i32) {

    unsafe {
        ffi::cgbequ_(&m, &n, &kl, &ku, ab.as_ptr() as *const _, &ldab, r.as_mut_ptr(),
                     c.as_mut_ptr(), rowcnd.as_mut_ptr(), colcnd.as_mut_ptr(), amax.as_mut_ptr(),
                     info)
    }
}

#[inline]
pub fn zgbequ(m: i32, n: i32, kl: i32, ku: i32, ab: &[c64], ldab: i32, r: &mut [f64],
              c: &mut [f64], rowcnd: &mut [f64], colcnd: &mut [f64], amax: &mut [f64],
              info: &mut i32) {

    unsafe {
        ffi::zgbequ_(&m, &n, &kl, &ku, ab.as_ptr() as *const _, &ldab, r.as_mut_ptr(),
                     c.as_mut_ptr(), rowcnd.as_mut_ptr(), colcnd.as_mut_ptr(), amax.as_mut_ptr(),
                     info)
    }
}

#[inline]
pub fn dgbequb(m: i32, n: i32, kl: i32, ku: i32, ab: &[f64], ldab: i32, r: &mut [f64],
               c: &mut [f64], rowcnd: &mut [f64], colcnd: &mut [f64], amax: &mut [f64],
               info: &mut i32) {

    unsafe {
        ffi::dgbequb_(&m, &n, &kl, &ku, ab.as_ptr(), &ldab, r.as_mut_ptr(), c.as_mut_ptr(),
                      rowcnd.as_mut_ptr(), colcnd.as_mut_ptr(), amax.as_mut_ptr(), info)
    }
}

#[inline]
pub fn sgbequb(m: i32, n: i32, kl: i32, ku: i32, ab: &[f32], ldab: i32, r: &mut [f32],
               c: &mut [f32], rowcnd: &mut [f32], colcnd: &mut [f32], amax: &mut [f32],
               info: &mut i32) {

    unsafe {
        ffi::sgbequb_(&m, &n, &kl, &ku, ab.as_ptr(), &ldab, r.as_mut_ptr(), c.as_mut_ptr(),
                      rowcnd.as_mut_ptr(), colcnd.as_mut_ptr(), amax.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zgbequb(m: i32, n: i32, kl: i32, ku: i32, ab: &[c64], ldab: i32, r: &mut [f64],
               c: &mut [f64], rowcnd: &mut [f64], colcnd: &mut [f64], amax: &mut [f64],
               info: &mut i32) {

    unsafe {
        ffi::zgbequb_(&m, &n, &kl, &ku, ab.as_ptr() as *const _, &ldab, r.as_mut_ptr(),
                      c.as_mut_ptr(), rowcnd.as_mut_ptr(), colcnd.as_mut_ptr(), amax.as_mut_ptr(),
                      info)
    }
}

#[inline]
pub fn cgbequb(m: i32, n: i32, kl: i32, ku: i32, ab: &[c32], ldab: i32, r: &mut [f32],
               c: &mut [f32], rowcnd: &mut [f32], colcnd: &mut [f32], amax: &mut [f32],
               info: &mut i32) {

    unsafe {
        ffi::cgbequb_(&m, &n, &kl, &ku, ab.as_ptr() as *const _, &ldab, r.as_mut_ptr(),
                      c.as_mut_ptr(), rowcnd.as_mut_ptr(), colcnd.as_mut_ptr(), amax.as_mut_ptr(),
                      info)
    }
}

#[inline]
pub fn spoequ(n: i32, a: &[f32], lda: i32, s: &mut [f32], scond: &mut [f32], amax: &mut [f32],
              info: &mut i32) {

    unsafe {
        ffi::spoequ_(&n, a.as_ptr(), &lda, s.as_mut_ptr(), scond.as_mut_ptr(), amax.as_mut_ptr(),
                     info)
    }
}

#[inline]
pub fn dpoequ(n: i32, a: &[f64], lda: i32, s: &mut [f64], scond: &mut [f64], amax: &mut [f64],
              info: &mut i32) {

    unsafe {
        ffi::dpoequ_(&n, a.as_ptr(), &lda, s.as_mut_ptr(), scond.as_mut_ptr(), amax.as_mut_ptr(),
                     info)
    }
}

#[inline]
pub fn cpoequ(n: i32, a: &[c32], lda: i32, s: &mut [f32], scond: &mut [f32], amax: &mut [f32],
              info: &mut i32) {

    unsafe {
        ffi::cpoequ_(&n, a.as_ptr() as *const _, &lda, s.as_mut_ptr(), scond.as_mut_ptr(),
                     amax.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zpoequ(n: i32, a: &[c64], lda: i32, s: &mut [f64], scond: &mut [f64], amax: &mut [f64],
              info: &mut i32) {

    unsafe {
        ffi::zpoequ_(&n, a.as_ptr() as *const _, &lda, s.as_mut_ptr(), scond.as_mut_ptr(),
                     amax.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dpoequb(n: i32, a: &[f64], lda: i32, s: &mut [f64], scond: &mut [f64], amax: &mut [f64],
               info: &mut i32) {

    unsafe {
        ffi::dpoequb_(&n, a.as_ptr(), &lda, s.as_mut_ptr(), scond.as_mut_ptr(), amax.as_mut_ptr(),
                      info)
    }
}

#[inline]
pub fn spoequb(n: i32, a: &[f32], lda: i32, s: &mut [f32], scond: &mut [f32], amax: &mut [f32],
               info: &mut i32) {

    unsafe {
        ffi::spoequb_(&n, a.as_ptr(), &lda, s.as_mut_ptr(), scond.as_mut_ptr(), amax.as_mut_ptr(),
                      info)
    }
}

#[inline]
pub fn zpoequb(n: i32, a: &[c64], lda: i32, s: &mut [f64], scond: &mut [f64], amax: &mut [f64],
               info: &mut i32) {

    unsafe {
        ffi::zpoequb_(&n, a.as_ptr() as *const _, &lda, s.as_mut_ptr(), scond.as_mut_ptr(),
                      amax.as_mut_ptr(), info)
    }
}

#[inline]
pub fn cpoequb(n: i32, a: &[c32], lda: i32, s: &mut [f32], scond: &mut [f32], amax: &mut [f32],
               info: &mut i32) {

    unsafe {
        ffi::cpoequb_(&n, a.as_ptr() as *const _, &lda, s.as_mut_ptr(), scond.as_mut_ptr(),
                      amax.as_mut_ptr(), info)
    }
}

#[inline]
pub fn sppequ(uplo: u8, n: i32, ap: &[f32], s: &mut [f32], scond: &mut [f32], amax: &mut [f32],
              info: &mut i32) {

    unsafe {
        ffi::sppequ_(&(uplo as c_char), &n, ap.as_ptr(), s.as_mut_ptr(), scond.as_mut_ptr(),
                     amax.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dppequ(uplo: u8, n: i32, ap: &[f64], s: &mut [f64], scond: &mut [f64], amax: &mut [f64],
              info: &mut i32) {

    unsafe {
        ffi::dppequ_(&(uplo as c_char), &n, ap.as_ptr(), s.as_mut_ptr(), scond.as_mut_ptr(),
                     amax.as_mut_ptr(), info)
    }
}

#[inline]
pub fn cppequ(uplo: u8, n: i32, ap: &[c32], s: &mut [f32], scond: &mut [f32], amax: &mut [f32],
              info: &mut i32) {

    unsafe {
        ffi::cppequ_(&(uplo as c_char), &n, ap.as_ptr() as *const _, s.as_mut_ptr(),
                     scond.as_mut_ptr(), amax.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zppequ(uplo: u8, n: i32, ap: &[c64], s: &mut [f64], scond: &mut [f64], amax: &mut [f64],
              info: &mut i32) {

    unsafe {
        ffi::zppequ_(&(uplo as c_char), &n, ap.as_ptr() as *const _, s.as_mut_ptr(),
                     scond.as_mut_ptr(), amax.as_mut_ptr(), info)
    }
}

#[inline]
pub fn spbequ(uplo: u8, n: i32, kd: i32, ab: &[f32], ldab: i32, s: &mut [f32], scond: &mut [f32],
              amax: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::spbequ_(&(uplo as c_char), &n, &kd, ab.as_ptr(), &ldab, s.as_mut_ptr(),
                     scond.as_mut_ptr(), amax.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dpbequ(uplo: u8, n: i32, kd: i32, ab: &[f64], ldab: i32, s: &mut [f64], scond: &mut [f64],
              amax: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::dpbequ_(&(uplo as c_char), &n, &kd, ab.as_ptr(), &ldab, s.as_mut_ptr(),
                     scond.as_mut_ptr(), amax.as_mut_ptr(), info)
    }
}

#[inline]
pub fn cpbequ(uplo: u8, n: i32, kd: i32, ab: &[c32], ldab: i32, s: &mut [f32], scond: &mut [f32],
              amax: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::cpbequ_(&(uplo as c_char), &n, &kd, ab.as_ptr() as *const _, &ldab, s.as_mut_ptr(),
                     scond.as_mut_ptr(), amax.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zpbequ(uplo: u8, n: i32, kd: i32, ab: &[c64], ldab: i32, s: &mut [f64], scond: &mut [f64],
              amax: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::zpbequ_(&(uplo as c_char), &n, &kd, ab.as_ptr() as *const _, &ldab, s.as_mut_ptr(),
                     scond.as_mut_ptr(), amax.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dsyequb(uplo: u8, n: i32, a: &[f64], lda: i32, s: &mut [f64], scond: &mut [f64],
               amax: &mut [f64], work: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::dsyequb_(&(uplo as c_char), &n, a.as_ptr(), &lda, s.as_mut_ptr(), scond.as_mut_ptr(),
                      amax.as_mut_ptr(), work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn ssyequb(uplo: u8, n: i32, a: &[f32], lda: i32, s: &mut [f32], scond: &mut [f32],
               amax: &mut [f32], work: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::ssyequb_(&(uplo as c_char), &n, a.as_ptr(), &lda, s.as_mut_ptr(), scond.as_mut_ptr(),
                      amax.as_mut_ptr(), work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zsyequb(uplo: u8, n: i32, a: &[c64], lda: i32, s: &mut [f64], scond: &mut [f64],
               amax: &mut [f64], work: &mut [c64], info: &mut i32) {

    unsafe {
        ffi::zsyequb_(&(uplo as c_char), &n, a.as_ptr() as *const _, &lda, s.as_mut_ptr(),
                      scond.as_mut_ptr(), amax.as_mut_ptr(), work.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn csyequb(uplo: u8, n: i32, a: &[c32], lda: i32, s: &mut [f32], scond: &mut [f32],
               amax: &mut [f32], work: &mut [c32], info: &mut i32) {

    unsafe {
        ffi::csyequb_(&(uplo as c_char), &n, a.as_ptr() as *const _, &lda, s.as_mut_ptr(),
                      scond.as_mut_ptr(), amax.as_mut_ptr(), work.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn zheequb(uplo: u8, n: i32, a: &[c64], lda: i32, s: &mut [f64], scond: &mut [f64],
               amax: &mut [f64], work: &mut [c64], info: &mut i32) {

    unsafe {
        ffi::zheequb_(&(uplo as c_char), &n, a.as_ptr() as *const _, &lda, s.as_mut_ptr(),
                      scond.as_mut_ptr(), amax.as_mut_ptr(), work.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn cheequb(uplo: u8, n: i32, a: &[c32], lda: i32, s: &mut [f32], scond: &mut [f32],
               amax: &mut [f32], work: &mut [c32], info: &mut i32) {

    unsafe {
        ffi::cheequb_(&(uplo as c_char), &n, a.as_ptr() as *const _, &lda, s.as_mut_ptr(),
                      scond.as_mut_ptr(), amax.as_mut_ptr(), work.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn sgesv(n: i32, nrhs: i32, a: &mut [f32], lda: i32, ipiv: &mut [i32], b: &mut [f32], ldb: i32,
             info: &mut i32) {

    unsafe {
        ffi::sgesv_(&n, &nrhs, a.as_mut_ptr(), &lda, ipiv.as_mut_ptr(), b.as_mut_ptr(), &ldb, info)
    }
}

#[inline]
pub fn dgesv(n: i32, nrhs: i32, a: &mut [f64], lda: i32, ipiv: &mut [i32], b: &mut [f64], ldb: i32,
             info: &mut i32) {

    unsafe {
        ffi::dgesv_(&n, &nrhs, a.as_mut_ptr(), &lda, ipiv.as_mut_ptr(), b.as_mut_ptr(), &ldb, info)
    }
}

#[inline]
pub fn cgesv(n: i32, nrhs: i32, a: &mut [c32], lda: i32, ipiv: &mut [i32], b: &mut [c32], ldb: i32,
             info: &mut i32) {

    unsafe {
        ffi::cgesv_(&n, &nrhs, a.as_mut_ptr() as *mut _, &lda, ipiv.as_mut_ptr(),
                    b.as_mut_ptr() as *mut _, &ldb, info)
    }
}

#[inline]
pub fn zgesv(n: i32, nrhs: i32, a: &mut [c64], lda: i32, ipiv: &mut [i32], b: &mut [c64], ldb: i32,
             info: &mut i32) {

    unsafe {
        ffi::zgesv_(&n, &nrhs, a.as_mut_ptr() as *mut _, &lda, ipiv.as_mut_ptr(),
                    b.as_mut_ptr() as *mut _, &ldb, info)
    }
}

#[inline]
pub fn dsgesv(n: i32, nrhs: i32, a: &mut [f64], lda: i32, ipiv: &mut [i32], b: &[f64], ldb: i32,
              x: &mut [f64], ldx: i32, work: &mut [f64], swork: &mut [f32], iter: &mut [i32],
              info: &mut i32) {

    unsafe {
        ffi::dsgesv_(&n, &nrhs, a.as_mut_ptr(), &lda, ipiv.as_mut_ptr(), b.as_ptr(), &ldb,
                     x.as_mut_ptr(), &ldx, work.as_mut_ptr(), swork.as_mut_ptr(),
                     iter.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zcgesv(n: i32, nrhs: i32, a: &mut [c64], lda: i32, ipiv: &mut [i32], b: &[c64], ldb: i32,
              x: &mut [c64], ldx: i32, work: &mut [c64], swork: &mut [c32], rwork: &mut [f64],
              iter: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::zcgesv_(&n, &nrhs, a.as_mut_ptr() as *mut _, &lda, ipiv.as_mut_ptr(),
                     b.as_ptr() as *const _, &ldb, x.as_mut_ptr() as *mut _, &ldx,
                     work.as_mut_ptr() as *mut _, swork.as_mut_ptr() as *mut _, rwork.as_mut_ptr(),
                     iter.as_mut_ptr(), info)
    }
}

#[inline]
pub fn sgesvx(fact: u8, trans: u8, n: i32, nrhs: i32, a: &mut [f32], lda: i32, af: &mut [f32],
              ldaf: i32, ipiv: &mut [i32], equed: &mut u8, r: &mut [f32], c: &mut [f32],
              b: &mut [f32], ldb: i32, x: &mut [f32], ldx: i32, rcond: &mut f32, ferr: &mut [f32],
              berr: &mut [f32], work: &mut [f32], iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::sgesvx_(&(fact as c_char), &(trans as c_char), &n, &nrhs, a.as_mut_ptr(), &lda,
                     af.as_mut_ptr(), &ldaf, ipiv.as_mut_ptr(), equed as *mut _ as *mut _,
                     r.as_mut_ptr(), c.as_mut_ptr(), b.as_mut_ptr(), &ldb, x.as_mut_ptr(), &ldx,
                     rcond, ferr.as_mut_ptr(), berr.as_mut_ptr(), work.as_mut_ptr(),
                     iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dgesvx(fact: u8, trans: u8, n: i32, nrhs: i32, a: &mut [f64], lda: i32, af: &mut [f64],
              ldaf: i32, ipiv: &mut [i32], equed: &mut u8, r: &mut [f64], c: &mut [f64],
              b: &mut [f64], ldb: i32, x: &mut [f64], ldx: i32, rcond: &mut f64, ferr: &mut [f64],
              berr: &mut [f64], work: &mut [f64], iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::dgesvx_(&(fact as c_char), &(trans as c_char), &n, &nrhs, a.as_mut_ptr(), &lda,
                     af.as_mut_ptr(), &ldaf, ipiv.as_mut_ptr(), equed as *mut _ as *mut _,
                     r.as_mut_ptr(), c.as_mut_ptr(), b.as_mut_ptr(), &ldb, x.as_mut_ptr(), &ldx,
                     rcond, ferr.as_mut_ptr(), berr.as_mut_ptr(), work.as_mut_ptr(),
                     iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn cgesvx(fact: u8, trans: u8, n: i32, nrhs: i32, a: &mut [c32], lda: i32, af: &mut [c32],
              ldaf: i32, ipiv: &mut [i32], equed: &mut u8, r: &mut [f32], c: &mut [f32],
              b: &mut [c32], ldb: i32, x: &mut [c32], ldx: i32, rcond: &mut f32, ferr: &mut [f32],
              berr: &mut [f32], work: &mut [c32], rwork: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::cgesvx_(&(fact as c_char), &(trans as c_char), &n, &nrhs, a.as_mut_ptr() as *mut _,
                     &lda, af.as_mut_ptr() as *mut _, &ldaf, ipiv.as_mut_ptr(),
                     equed as *mut _ as *mut _, r.as_mut_ptr(), c.as_mut_ptr(),
                     b.as_mut_ptr() as *mut _, &ldb, x.as_mut_ptr() as *mut _, &ldx, rcond,
                     ferr.as_mut_ptr(), berr.as_mut_ptr(), work.as_mut_ptr() as *mut _,
                     rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zgesvx(fact: u8, trans: u8, n: i32, nrhs: i32, a: &mut [c64], lda: i32, af: &mut [c64],
              ldaf: i32, ipiv: &mut [i32], equed: &mut u8, r: &mut [f64], c: &mut [f64],
              b: &mut [c64], ldb: i32, x: &mut [c64], ldx: i32, rcond: &mut f64, ferr: &mut [f64],
              berr: &mut [f64], work: &mut [c64], rwork: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::zgesvx_(&(fact as c_char), &(trans as c_char), &n, &nrhs, a.as_mut_ptr() as *mut _,
                     &lda, af.as_mut_ptr() as *mut _, &ldaf, ipiv.as_mut_ptr(),
                     equed as *mut _ as *mut _, r.as_mut_ptr(), c.as_mut_ptr(),
                     b.as_mut_ptr() as *mut _, &ldb, x.as_mut_ptr() as *mut _, &ldx, rcond,
                     ferr.as_mut_ptr(), berr.as_mut_ptr(), work.as_mut_ptr() as *mut _,
                     rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dgesvxx(fact: u8, trans: u8, n: i32, nrhs: i32, a: &mut [f64], lda: i32, af: &mut [f64],
               ldaf: i32, ipiv: &mut [i32], equed: &mut u8, r: &mut [f64], c: &mut [f64],
               b: &mut [f64], ldb: i32, x: &mut [f64], ldx: i32, rcond: &mut f64,
               rpvgrw: &mut [f64], berr: &mut [f64], n_err_bnds: i32, err_bnds_norm: &mut [f64],
               err_bnds_comp: &mut [f64], nparams: &[i32], params: &mut [f64], work: &mut [f64],
               iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::dgesvxx_(&(fact as c_char), &(trans as c_char), &n, &nrhs, a.as_mut_ptr(), &lda,
                      af.as_mut_ptr(), &ldaf, ipiv.as_mut_ptr(), equed as *mut _ as *mut _,
                      r.as_mut_ptr(), c.as_mut_ptr(), b.as_mut_ptr(), &ldb, x.as_mut_ptr(), &ldx,
                      rcond, rpvgrw.as_mut_ptr(), berr.as_mut_ptr(), &n_err_bnds,
                      err_bnds_norm.as_mut_ptr(), err_bnds_comp.as_mut_ptr(), nparams.as_ptr(),
                      params.as_mut_ptr(), work.as_mut_ptr(), iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn sgesvxx(fact: u8, trans: u8, n: i32, nrhs: i32, a: &mut [f32], lda: i32, af: &mut [f32],
               ldaf: i32, ipiv: &mut [i32], equed: &mut u8, r: &mut [f32], c: &mut [f32],
               b: &mut [f32], ldb: i32, x: &mut [f32], ldx: i32, rcond: &mut f32,
               rpvgrw: &mut [f32], berr: &mut [f32], n_err_bnds: i32, err_bnds_norm: &mut [f32],
               err_bnds_comp: &mut [f32], nparams: &[i32], params: &mut [f32], work: &mut [f32],
               iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::sgesvxx_(&(fact as c_char), &(trans as c_char), &n, &nrhs, a.as_mut_ptr(), &lda,
                      af.as_mut_ptr(), &ldaf, ipiv.as_mut_ptr(), equed as *mut _ as *mut _,
                      r.as_mut_ptr(), c.as_mut_ptr(), b.as_mut_ptr(), &ldb, x.as_mut_ptr(), &ldx,
                      rcond, rpvgrw.as_mut_ptr(), berr.as_mut_ptr(), &n_err_bnds,
                      err_bnds_norm.as_mut_ptr(), err_bnds_comp.as_mut_ptr(), nparams.as_ptr(),
                      params.as_mut_ptr(), work.as_mut_ptr(), iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zgesvxx(fact: u8, trans: u8, n: i32, nrhs: i32, a: &mut [c64], lda: i32, af: &mut [c64],
               ldaf: i32, ipiv: &mut [i32], equed: &mut u8, r: &mut [f64], c: &mut [f64],
               b: &mut [c64], ldb: i32, x: &mut [c64], ldx: i32, rcond: &mut f64,
               rpvgrw: &mut [f64], berr: &mut [f64], n_err_bnds: i32, err_bnds_norm: &mut [f64],
               err_bnds_comp: &mut [f64], nparams: &[i32], params: &mut [f64], work: &mut [c64],
               rwork: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::zgesvxx_(&(fact as c_char), &(trans as c_char), &n, &nrhs, a.as_mut_ptr() as *mut _,
                      &lda, af.as_mut_ptr() as *mut _, &ldaf, ipiv.as_mut_ptr(),
                      equed as *mut _ as *mut _, r.as_mut_ptr(), c.as_mut_ptr(),
                      b.as_mut_ptr() as *mut _, &ldb, x.as_mut_ptr() as *mut _, &ldx, rcond,
                      rpvgrw.as_mut_ptr(), berr.as_mut_ptr(), &n_err_bnds,
                      err_bnds_norm.as_mut_ptr(), err_bnds_comp.as_mut_ptr(), nparams.as_ptr(),
                      params.as_mut_ptr(), work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn cgesvxx(fact: u8, trans: u8, n: i32, nrhs: i32, a: &mut [c32], lda: i32, af: &mut [c32],
               ldaf: i32, ipiv: &mut [i32], equed: &mut u8, r: &mut [f32], c: &mut [f32],
               b: &mut [c32], ldb: i32, x: &mut [c32], ldx: i32, rcond: &mut f32,
               rpvgrw: &mut [f32], berr: &mut [f32], n_err_bnds: i32, err_bnds_norm: &mut [f32],
               err_bnds_comp: &mut [f32], nparams: &[i32], params: &mut [f32], work: &mut [c32],
               rwork: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::cgesvxx_(&(fact as c_char), &(trans as c_char), &n, &nrhs, a.as_mut_ptr() as *mut _,
                      &lda, af.as_mut_ptr() as *mut _, &ldaf, ipiv.as_mut_ptr(),
                      equed as *mut _ as *mut _, r.as_mut_ptr(), c.as_mut_ptr(),
                      b.as_mut_ptr() as *mut _, &ldb, x.as_mut_ptr() as *mut _, &ldx, rcond,
                      rpvgrw.as_mut_ptr(), berr.as_mut_ptr(), &n_err_bnds,
                      err_bnds_norm.as_mut_ptr(), err_bnds_comp.as_mut_ptr(), nparams.as_ptr(),
                      params.as_mut_ptr(), work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn sgbsv(n: i32, kl: i32, ku: i32, nrhs: i32, ab: &mut [f32], ldab: i32, ipiv: &mut [i32],
             b: &mut [f32], ldb: i32, info: &mut i32) {

    unsafe {
        ffi::sgbsv_(&n, &kl, &ku, &nrhs, ab.as_mut_ptr(), &ldab, ipiv.as_mut_ptr(), b.as_mut_ptr(),
                    &ldb, info)
    }
}

#[inline]
pub fn dgbsv(n: i32, kl: i32, ku: i32, nrhs: i32, ab: &mut [f64], ldab: i32, ipiv: &mut [i32],
             b: &mut [f64], ldb: i32, info: &mut i32) {

    unsafe {
        ffi::dgbsv_(&n, &kl, &ku, &nrhs, ab.as_mut_ptr(), &ldab, ipiv.as_mut_ptr(), b.as_mut_ptr(),
                    &ldb, info)
    }
}

#[inline]
pub fn cgbsv(n: i32, kl: i32, ku: i32, nrhs: i32, ab: &mut [c32], ldab: i32, ipiv: &mut [i32],
             b: &mut [c32], ldb: i32, info: &mut i32) {

    unsafe {
        ffi::cgbsv_(&n, &kl, &ku, &nrhs, ab.as_mut_ptr() as *mut _, &ldab, ipiv.as_mut_ptr(),
                    b.as_mut_ptr() as *mut _, &ldb, info)
    }
}

#[inline]
pub fn zgbsv(n: i32, kl: i32, ku: i32, nrhs: i32, ab: &mut [c64], ldab: i32, ipiv: &mut [i32],
             b: &mut [c64], ldb: i32, info: &mut i32) {

    unsafe {
        ffi::zgbsv_(&n, &kl, &ku, &nrhs, ab.as_mut_ptr() as *mut _, &ldab, ipiv.as_mut_ptr(),
                    b.as_mut_ptr() as *mut _, &ldb, info)
    }
}

#[inline]
pub fn sgbsvx(fact: u8, trans: u8, n: i32, kl: i32, ku: i32, nrhs: i32, ab: &mut [f32], ldab: i32,
              afb: &mut [f32], ldafb: i32, ipiv: &mut [i32], equed: &mut u8, r: &mut [f32],
              c: &mut [f32], b: &mut [f32], ldb: i32, x: &mut [f32], ldx: i32, rcond: &mut f32,
              ferr: &mut [f32], berr: &mut [f32], work: &mut [f32], iwork: &mut [i32],
              info: &mut i32) {

    unsafe {
        ffi::sgbsvx_(&(fact as c_char), &(trans as c_char), &n, &kl, &ku, &nrhs, ab.as_mut_ptr(),
                     &ldab, afb.as_mut_ptr(), &ldafb, ipiv.as_mut_ptr(), equed as *mut _ as *mut _,
                     r.as_mut_ptr(), c.as_mut_ptr(), b.as_mut_ptr(), &ldb, x.as_mut_ptr(), &ldx,
                     rcond, ferr.as_mut_ptr(), berr.as_mut_ptr(), work.as_mut_ptr(),
                     iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dgbsvx(fact: u8, trans: u8, n: i32, kl: i32, ku: i32, nrhs: i32, ab: &mut [f64], ldab: i32,
              afb: &mut [f64], ldafb: i32, ipiv: &mut [i32], equed: &mut u8, r: &mut [f64],
              c: &mut [f64], b: &mut [f64], ldb: i32, x: &mut [f64], ldx: i32, rcond: &mut f64,
              ferr: &mut [f64], berr: &mut [f64], work: &mut [f64], iwork: &mut [i32],
              info: &mut i32) {

    unsafe {
        ffi::dgbsvx_(&(fact as c_char), &(trans as c_char), &n, &kl, &ku, &nrhs, ab.as_mut_ptr(),
                     &ldab, afb.as_mut_ptr(), &ldafb, ipiv.as_mut_ptr(), equed as *mut _ as *mut _,
                     r.as_mut_ptr(), c.as_mut_ptr(), b.as_mut_ptr(), &ldb, x.as_mut_ptr(), &ldx,
                     rcond, ferr.as_mut_ptr(), berr.as_mut_ptr(), work.as_mut_ptr(),
                     iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn cgbsvx(fact: u8, trans: u8, n: i32, kl: i32, ku: i32, nrhs: i32, ab: &mut [c32], ldab: i32,
              afb: &mut [c32], ldafb: i32, ipiv: &mut [i32], equed: &mut u8, r: &mut [f32],
              c: &mut [f32], b: &mut [c32], ldb: i32, x: &mut [c32], ldx: i32, rcond: &mut f32,
              ferr: &mut [f32], berr: &mut [f32], work: &mut [c32], rwork: &mut [f32],
              info: &mut i32) {

    unsafe {
        ffi::cgbsvx_(&(fact as c_char), &(trans as c_char), &n, &kl, &ku, &nrhs,
                     ab.as_mut_ptr() as *mut _, &ldab, afb.as_mut_ptr() as *mut _, &ldafb,
                     ipiv.as_mut_ptr(), equed as *mut _ as *mut _, r.as_mut_ptr(), c.as_mut_ptr(),
                     b.as_mut_ptr() as *mut _, &ldb, x.as_mut_ptr() as *mut _, &ldx, rcond,
                     ferr.as_mut_ptr(), berr.as_mut_ptr(), work.as_mut_ptr() as *mut _,
                     rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zgbsvx(fact: u8, trans: u8, n: i32, kl: i32, ku: i32, nrhs: i32, ab: &mut [c64], ldab: i32,
              afb: &mut [c64], ldafb: i32, ipiv: &mut [i32], equed: &mut u8, r: &mut [f64],
              c: &mut [f64], b: &mut [c64], ldb: i32, x: &mut [c64], ldx: i32, rcond: &mut f64,
              ferr: &mut [f64], berr: &mut [f64], work: &mut [c64], rwork: &mut [f64],
              info: &mut i32) {

    unsafe {
        ffi::zgbsvx_(&(fact as c_char), &(trans as c_char), &n, &kl, &ku, &nrhs,
                     ab.as_mut_ptr() as *mut _, &ldab, afb.as_mut_ptr() as *mut _, &ldafb,
                     ipiv.as_mut_ptr(), equed as *mut _ as *mut _, r.as_mut_ptr(), c.as_mut_ptr(),
                     b.as_mut_ptr() as *mut _, &ldb, x.as_mut_ptr() as *mut _, &ldx, rcond,
                     ferr.as_mut_ptr(), berr.as_mut_ptr(), work.as_mut_ptr() as *mut _,
                     rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dgbsvxx(fact: u8, trans: u8, n: i32, kl: i32, ku: i32, nrhs: i32, ab: &mut [f64], ldab: i32,
               afb: &mut [f64], ldafb: i32, ipiv: &mut [i32], equed: &mut u8, r: &mut [f64],
               c: &mut [f64], b: &mut [f64], ldb: i32, x: &mut [f64], ldx: i32, rcond: &mut f64,
               rpvgrw: &mut [f64], berr: &mut [f64], n_err_bnds: i32, err_bnds_norm: &mut [f64],
               err_bnds_comp: &mut [f64], nparams: &[i32], params: &mut [f64], work: &mut [f64],
               iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::dgbsvxx_(&(fact as c_char), &(trans as c_char), &n, &kl, &ku, &nrhs, ab.as_mut_ptr(),
                      &ldab, afb.as_mut_ptr(), &ldafb, ipiv.as_mut_ptr(),
                      equed as *mut _ as *mut _, r.as_mut_ptr(), c.as_mut_ptr(), b.as_mut_ptr(),
                      &ldb, x.as_mut_ptr(), &ldx, rcond, rpvgrw.as_mut_ptr(), berr.as_mut_ptr(),
                      &n_err_bnds, err_bnds_norm.as_mut_ptr(), err_bnds_comp.as_mut_ptr(),
                      nparams.as_ptr(), params.as_mut_ptr(), work.as_mut_ptr(), iwork.as_mut_ptr(),
                      info)
    }
}

#[inline]
pub fn sgbsvxx(fact: u8, trans: u8, n: i32, kl: i32, ku: i32, nrhs: i32, ab: &mut [f32], ldab: i32,
               afb: &mut [f32], ldafb: i32, ipiv: &mut [i32], equed: &mut u8, r: &mut [f32],
               c: &mut [f32], b: &mut [f32], ldb: i32, x: &mut [f32], ldx: i32, rcond: &mut f32,
               rpvgrw: &mut [f32], berr: &mut [f32], n_err_bnds: i32, err_bnds_norm: &mut [f32],
               err_bnds_comp: &mut [f32], nparams: &[i32], params: &mut [f32], work: &mut [f32],
               iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::sgbsvxx_(&(fact as c_char), &(trans as c_char), &n, &kl, &ku, &nrhs, ab.as_mut_ptr(),
                      &ldab, afb.as_mut_ptr(), &ldafb, ipiv.as_mut_ptr(),
                      equed as *mut _ as *mut _, r.as_mut_ptr(), c.as_mut_ptr(), b.as_mut_ptr(),
                      &ldb, x.as_mut_ptr(), &ldx, rcond, rpvgrw.as_mut_ptr(), berr.as_mut_ptr(),
                      &n_err_bnds, err_bnds_norm.as_mut_ptr(), err_bnds_comp.as_mut_ptr(),
                      nparams.as_ptr(), params.as_mut_ptr(), work.as_mut_ptr(), iwork.as_mut_ptr(),
                      info)
    }
}

#[inline]
pub fn zgbsvxx(fact: u8, trans: u8, n: i32, kl: i32, ku: i32, nrhs: i32, ab: &mut [c64], ldab: i32,
               afb: &mut [c64], ldafb: i32, ipiv: &mut [i32], equed: &mut u8, r: &mut [f64],
               c: &mut [f64], b: &mut [c64], ldb: i32, x: &mut [c64], ldx: i32, rcond: &mut f64,
               rpvgrw: &mut [f64], berr: &mut [f64], n_err_bnds: i32, err_bnds_norm: &mut [f64],
               err_bnds_comp: &mut [f64], nparams: &[i32], params: &mut [f64], work: &mut [c64],
               rwork: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::zgbsvxx_(&(fact as c_char), &(trans as c_char), &n, &kl, &ku, &nrhs,
                      ab.as_mut_ptr() as *mut _, &ldab, afb.as_mut_ptr() as *mut _, &ldafb,
                      ipiv.as_mut_ptr(), equed as *mut _ as *mut _, r.as_mut_ptr(), c.as_mut_ptr(),
                      b.as_mut_ptr() as *mut _, &ldb, x.as_mut_ptr() as *mut _, &ldx, rcond,
                      rpvgrw.as_mut_ptr(), berr.as_mut_ptr(), &n_err_bnds,
                      err_bnds_norm.as_mut_ptr(), err_bnds_comp.as_mut_ptr(), nparams.as_ptr(),
                      params.as_mut_ptr(), work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn cgbsvxx(fact: u8, trans: u8, n: i32, kl: i32, ku: i32, nrhs: i32, ab: &mut [c32], ldab: i32,
               afb: &mut [c32], ldafb: i32, ipiv: &mut [i32], equed: &mut u8, r: &mut [f32],
               c: &mut [f32], b: &mut [c32], ldb: i32, x: &mut [c32], ldx: i32, rcond: &mut f32,
               rpvgrw: &mut [f32], berr: &mut [f32], n_err_bnds: i32, err_bnds_norm: &mut [f32],
               err_bnds_comp: &mut [f32], nparams: &[i32], params: &mut [f32], work: &mut [c32],
               rwork: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::cgbsvxx_(&(fact as c_char), &(trans as c_char), &n, &kl, &ku, &nrhs,
                      ab.as_mut_ptr() as *mut _, &ldab, afb.as_mut_ptr() as *mut _, &ldafb,
                      ipiv.as_mut_ptr(), equed as *mut _ as *mut _, r.as_mut_ptr(), c.as_mut_ptr(),
                      b.as_mut_ptr() as *mut _, &ldb, x.as_mut_ptr() as *mut _, &ldx, rcond,
                      rpvgrw.as_mut_ptr(), berr.as_mut_ptr(), &n_err_bnds,
                      err_bnds_norm.as_mut_ptr(), err_bnds_comp.as_mut_ptr(), nparams.as_ptr(),
                      params.as_mut_ptr(), work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn sgtsv(n: i32, nrhs: i32, dl: &mut [f32], d: &mut [f32], du: &mut [f32], b: &mut [f32],
             ldb: i32, info: &mut i32) {

    unsafe {
        ffi::sgtsv_(&n, &nrhs, dl.as_mut_ptr(), d.as_mut_ptr(), du.as_mut_ptr(), b.as_mut_ptr(),
                    &ldb, info)
    }
}

#[inline]
pub fn dgtsv(n: i32, nrhs: i32, dl: &mut [f64], d: &mut [f64], du: &mut [f64], b: &mut [f64],
             ldb: i32, info: &mut i32) {

    unsafe {
        ffi::dgtsv_(&n, &nrhs, dl.as_mut_ptr(), d.as_mut_ptr(), du.as_mut_ptr(), b.as_mut_ptr(),
                    &ldb, info)
    }
}

#[inline]
pub fn cgtsv(n: i32, nrhs: i32, dl: &mut [c32], d: &mut [c32], du: &mut [c32], b: &mut [c32],
             ldb: i32, info: &mut i32) {

    unsafe {
        ffi::cgtsv_(&n, &nrhs, dl.as_mut_ptr() as *mut _, d.as_mut_ptr() as *mut _,
                    du.as_mut_ptr() as *mut _, b.as_mut_ptr() as *mut _, &ldb, info)
    }
}

#[inline]
pub fn zgtsv(n: i32, nrhs: i32, dl: &mut [c64], d: &mut [c64], du: &mut [c64], b: &mut [c64],
             ldb: i32, info: &mut i32) {

    unsafe {
        ffi::zgtsv_(&n, &nrhs, dl.as_mut_ptr() as *mut _, d.as_mut_ptr() as *mut _,
                    du.as_mut_ptr() as *mut _, b.as_mut_ptr() as *mut _, &ldb, info)
    }
}

#[inline]
pub fn sgtsvx(fact: u8, trans: u8, n: i32, nrhs: i32, dl: &[f32], d: &[f32], du: &[f32],
              dlf: &mut [f32], df: &mut [f32], duf: &mut [f32], du2: &mut [f32], ipiv: &mut [i32],
              b: &[f32], ldb: i32, x: &mut [f32], ldx: i32, rcond: &mut f32, ferr: &mut [f32],
              berr: &mut [f32], work: &mut [f32], iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::sgtsvx_(&(fact as c_char), &(trans as c_char), &n, &nrhs, dl.as_ptr(), d.as_ptr(),
                     du.as_ptr(), dlf.as_mut_ptr(), df.as_mut_ptr(), duf.as_mut_ptr(),
                     du2.as_mut_ptr(), ipiv.as_mut_ptr(), b.as_ptr(), &ldb, x.as_mut_ptr(), &ldx,
                     rcond, ferr.as_mut_ptr(), berr.as_mut_ptr(), work.as_mut_ptr(),
                     iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dgtsvx(fact: u8, trans: u8, n: i32, nrhs: i32, dl: &[f64], d: &[f64], du: &[f64],
              dlf: &mut [f64], df: &mut [f64], duf: &mut [f64], du2: &mut [f64], ipiv: &mut [i32],
              b: &[f64], ldb: i32, x: &mut [f64], ldx: i32, rcond: &mut f64, ferr: &mut [f64],
              berr: &mut [f64], work: &mut [f64], iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::dgtsvx_(&(fact as c_char), &(trans as c_char), &n, &nrhs, dl.as_ptr(), d.as_ptr(),
                     du.as_ptr(), dlf.as_mut_ptr(), df.as_mut_ptr(), duf.as_mut_ptr(),
                     du2.as_mut_ptr(), ipiv.as_mut_ptr(), b.as_ptr(), &ldb, x.as_mut_ptr(), &ldx,
                     rcond, ferr.as_mut_ptr(), berr.as_mut_ptr(), work.as_mut_ptr(),
                     iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn cgtsvx(fact: u8, trans: u8, n: i32, nrhs: i32, dl: &[c32], d: &[c32], du: &[c32],
              dlf: &mut [c32], df: &mut [c32], duf: &mut [c32], du2: &mut [c32], ipiv: &mut [i32],
              b: &[c32], ldb: i32, x: &mut [c32], ldx: i32, rcond: &mut f32, ferr: &mut [f32],
              berr: &mut [f32], work: &mut [c32], rwork: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::cgtsvx_(&(fact as c_char), &(trans as c_char), &n, &nrhs, dl.as_ptr() as *const _,
                     d.as_ptr() as *const _, du.as_ptr() as *const _, dlf.as_mut_ptr() as *mut _,
                     df.as_mut_ptr() as *mut _, duf.as_mut_ptr() as *mut _,
                     du2.as_mut_ptr() as *mut _, ipiv.as_mut_ptr(), b.as_ptr() as *const _, &ldb,
                     x.as_mut_ptr() as *mut _, &ldx, rcond, ferr.as_mut_ptr(), berr.as_mut_ptr(),
                     work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zgtsvx(fact: u8, trans: u8, n: i32, nrhs: i32, dl: &[c64], d: &[c64], du: &[c64],
              dlf: &mut [c64], df: &mut [c64], duf: &mut [c64], du2: &mut [c64], ipiv: &mut [i32],
              b: &[c64], ldb: i32, x: &mut [c64], ldx: i32, rcond: &mut f64, ferr: &mut [f64],
              berr: &mut [f64], work: &mut [c64], rwork: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::zgtsvx_(&(fact as c_char), &(trans as c_char), &n, &nrhs, dl.as_ptr() as *const _,
                     d.as_ptr() as *const _, du.as_ptr() as *const _, dlf.as_mut_ptr() as *mut _,
                     df.as_mut_ptr() as *mut _, duf.as_mut_ptr() as *mut _,
                     du2.as_mut_ptr() as *mut _, ipiv.as_mut_ptr(), b.as_ptr() as *const _, &ldb,
                     x.as_mut_ptr() as *mut _, &ldx, rcond, ferr.as_mut_ptr(), berr.as_mut_ptr(),
                     work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn sposv(uplo: u8, n: i32, nrhs: i32, a: &mut [f32], lda: i32, b: &mut [f32], ldb: i32,
             info: &mut i32) {

    unsafe {
        ffi::sposv_(&(uplo as c_char), &n, &nrhs, a.as_mut_ptr(), &lda, b.as_mut_ptr(), &ldb, info)
    }
}

#[inline]
pub fn dposv(uplo: u8, n: i32, nrhs: i32, a: &mut [f64], lda: i32, b: &mut [f64], ldb: i32,
             info: &mut i32) {

    unsafe {
        ffi::dposv_(&(uplo as c_char), &n, &nrhs, a.as_mut_ptr(), &lda, b.as_mut_ptr(), &ldb, info)
    }
}

#[inline]
pub fn cposv(uplo: u8, n: i32, nrhs: i32, a: &mut [c32], lda: i32, b: &mut [c32], ldb: i32,
             info: &mut i32) {

    unsafe {
        ffi::cposv_(&(uplo as c_char), &n, &nrhs, a.as_mut_ptr() as *mut _, &lda,
                    b.as_mut_ptr() as *mut _, &ldb, info)
    }
}

#[inline]
pub fn zposv(uplo: u8, n: i32, nrhs: i32, a: &mut [c64], lda: i32, b: &mut [c64], ldb: i32,
             info: &mut i32) {

    unsafe {
        ffi::zposv_(&(uplo as c_char), &n, &nrhs, a.as_mut_ptr() as *mut _, &lda,
                    b.as_mut_ptr() as *mut _, &ldb, info)
    }
}

#[inline]
pub fn dsposv(uplo: u8, n: i32, nrhs: i32, a: &mut [f64], lda: i32, b: &[f64], ldb: i32,
              x: &mut [f64], ldx: i32, work: &mut [f64], swork: &mut [f32], iter: &mut [i32],
              info: &mut i32) {

    unsafe {
        ffi::dsposv_(&(uplo as c_char), &n, &nrhs, a.as_mut_ptr(), &lda, b.as_ptr(), &ldb,
                     x.as_mut_ptr(), &ldx, work.as_mut_ptr(), swork.as_mut_ptr(),
                     iter.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zcposv(uplo: u8, n: i32, nrhs: i32, a: &mut [c64], lda: i32, b: &[c64], ldb: i32,
              x: &mut [c64], ldx: i32, work: &mut [c64], swork: &mut [c32], rwork: &mut [f64],
              iter: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::zcposv_(&(uplo as c_char), &n, &nrhs, a.as_mut_ptr() as *mut _, &lda,
                     b.as_ptr() as *const _, &ldb, x.as_mut_ptr() as *mut _, &ldx,
                     work.as_mut_ptr() as *mut _, swork.as_mut_ptr() as *mut _, rwork.as_mut_ptr(),
                     iter.as_mut_ptr(), info)
    }
}

#[inline]
pub fn sposvx(fact: u8, uplo: u8, n: i32, nrhs: i32, a: &mut [f32], lda: i32, af: &mut [f32],
              ldaf: i32, equed: &mut u8, s: &mut [f32], b: &mut [f32], ldb: i32, x: &mut [f32],
              ldx: i32, rcond: &mut f32, ferr: &mut [f32], berr: &mut [f32], work: &mut [f32],
              iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::sposvx_(&(fact as c_char), &(uplo as c_char), &n, &nrhs, a.as_mut_ptr(), &lda,
                     af.as_mut_ptr(), &ldaf, equed as *mut _ as *mut _, s.as_mut_ptr(),
                     b.as_mut_ptr(), &ldb, x.as_mut_ptr(), &ldx, rcond, ferr.as_mut_ptr(),
                     berr.as_mut_ptr(), work.as_mut_ptr(), iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dposvx(fact: u8, uplo: u8, n: i32, nrhs: i32, a: &mut [f64], lda: i32, af: &mut [f64],
              ldaf: i32, equed: &mut u8, s: &mut [f64], b: &mut [f64], ldb: i32, x: &mut [f64],
              ldx: i32, rcond: &mut f64, ferr: &mut [f64], berr: &mut [f64], work: &mut [f64],
              iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::dposvx_(&(fact as c_char), &(uplo as c_char), &n, &nrhs, a.as_mut_ptr(), &lda,
                     af.as_mut_ptr(), &ldaf, equed as *mut _ as *mut _, s.as_mut_ptr(),
                     b.as_mut_ptr(), &ldb, x.as_mut_ptr(), &ldx, rcond, ferr.as_mut_ptr(),
                     berr.as_mut_ptr(), work.as_mut_ptr(), iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn cposvx(fact: u8, uplo: u8, n: i32, nrhs: i32, a: &mut [c32], lda: i32, af: &mut [c32],
              ldaf: i32, equed: &mut u8, s: &mut [f32], b: &mut [c32], ldb: i32, x: &mut [c32],
              ldx: i32, rcond: &mut f32, ferr: &mut [f32], berr: &mut [f32], work: &mut [c32],
              rwork: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::cposvx_(&(fact as c_char), &(uplo as c_char), &n, &nrhs, a.as_mut_ptr() as *mut _,
                     &lda, af.as_mut_ptr() as *mut _, &ldaf, equed as *mut _ as *mut _,
                     s.as_mut_ptr(), b.as_mut_ptr() as *mut _, &ldb, x.as_mut_ptr() as *mut _,
                     &ldx, rcond, ferr.as_mut_ptr(), berr.as_mut_ptr(),
                     work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zposvx(fact: u8, uplo: u8, n: i32, nrhs: i32, a: &mut [c64], lda: i32, af: &mut [c64],
              ldaf: i32, equed: &mut u8, s: &mut [f64], b: &mut [c64], ldb: i32, x: &mut [c64],
              ldx: i32, rcond: &mut f64, ferr: &mut [f64], berr: &mut [f64], work: &mut [c64],
              rwork: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::zposvx_(&(fact as c_char), &(uplo as c_char), &n, &nrhs, a.as_mut_ptr() as *mut _,
                     &lda, af.as_mut_ptr() as *mut _, &ldaf, equed as *mut _ as *mut _,
                     s.as_mut_ptr(), b.as_mut_ptr() as *mut _, &ldb, x.as_mut_ptr() as *mut _,
                     &ldx, rcond, ferr.as_mut_ptr(), berr.as_mut_ptr(),
                     work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dposvxx(fact: u8, uplo: u8, n: i32, nrhs: i32, a: &mut [f64], lda: i32, af: &mut [f64],
               ldaf: i32, equed: &mut u8, s: &mut [f64], b: &mut [f64], ldb: i32, x: &mut [f64],
               ldx: i32, rcond: &mut f64, rpvgrw: &mut [f64], berr: &mut [f64], n_err_bnds: i32,
               err_bnds_norm: &mut [f64], err_bnds_comp: &mut [f64], nparams: &[i32],
               params: &mut [f64], work: &mut [f64], iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::dposvxx_(&(fact as c_char), &(uplo as c_char), &n, &nrhs, a.as_mut_ptr(), &lda,
                      af.as_mut_ptr(), &ldaf, equed as *mut _ as *mut _, s.as_mut_ptr(),
                      b.as_mut_ptr(), &ldb, x.as_mut_ptr(), &ldx, rcond, rpvgrw.as_mut_ptr(),
                      berr.as_mut_ptr(), &n_err_bnds, err_bnds_norm.as_mut_ptr(),
                      err_bnds_comp.as_mut_ptr(), nparams.as_ptr(), params.as_mut_ptr(),
                      work.as_mut_ptr(), iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn sposvxx(fact: u8, uplo: u8, n: i32, nrhs: i32, a: &mut [f32], lda: i32, af: &mut [f32],
               ldaf: i32, equed: &mut u8, s: &mut [f32], b: &mut [f32], ldb: i32, x: &mut [f32],
               ldx: i32, rcond: &mut f32, rpvgrw: &mut [f32], berr: &mut [f32], n_err_bnds: i32,
               err_bnds_norm: &mut [f32], err_bnds_comp: &mut [f32], nparams: &[i32],
               params: &mut [f32], work: &mut [f32], iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::sposvxx_(&(fact as c_char), &(uplo as c_char), &n, &nrhs, a.as_mut_ptr(), &lda,
                      af.as_mut_ptr(), &ldaf, equed as *mut _ as *mut _, s.as_mut_ptr(),
                      b.as_mut_ptr(), &ldb, x.as_mut_ptr(), &ldx, rcond, rpvgrw.as_mut_ptr(),
                      berr.as_mut_ptr(), &n_err_bnds, err_bnds_norm.as_mut_ptr(),
                      err_bnds_comp.as_mut_ptr(), nparams.as_ptr(), params.as_mut_ptr(),
                      work.as_mut_ptr(), iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zposvxx(fact: u8, uplo: u8, n: i32, nrhs: i32, a: &mut [c64], lda: i32, af: &mut [c64],
               ldaf: i32, equed: &mut u8, s: &mut [f64], b: &mut [c64], ldb: i32, x: &mut [c64],
               ldx: i32, rcond: &mut f64, rpvgrw: &mut [f64], berr: &mut [f64], n_err_bnds: i32,
               err_bnds_norm: &mut [f64], err_bnds_comp: &mut [f64], nparams: &[i32],
               params: &mut [f64], work: &mut [c64], rwork: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::zposvxx_(&(fact as c_char), &(uplo as c_char), &n, &nrhs, a.as_mut_ptr() as *mut _,
                      &lda, af.as_mut_ptr() as *mut _, &ldaf, equed as *mut _ as *mut _,
                      s.as_mut_ptr(), b.as_mut_ptr() as *mut _, &ldb, x.as_mut_ptr() as *mut _,
                      &ldx, rcond, rpvgrw.as_mut_ptr(), berr.as_mut_ptr(), &n_err_bnds,
                      err_bnds_norm.as_mut_ptr(), err_bnds_comp.as_mut_ptr(), nparams.as_ptr(),
                      params.as_mut_ptr(), work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn cposvxx(fact: u8, uplo: u8, n: i32, nrhs: i32, a: &mut [c32], lda: i32, af: &mut [c32],
               ldaf: i32, equed: &mut u8, s: &mut [f32], b: &mut [c32], ldb: i32, x: &mut [c32],
               ldx: i32, rcond: &mut f32, rpvgrw: &mut [f32], berr: &mut [f32], n_err_bnds: i32,
               err_bnds_norm: &mut [f32], err_bnds_comp: &mut [f32], nparams: &[i32],
               params: &mut [f32], work: &mut [c32], rwork: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::cposvxx_(&(fact as c_char), &(uplo as c_char), &n, &nrhs, a.as_mut_ptr() as *mut _,
                      &lda, af.as_mut_ptr() as *mut _, &ldaf, equed as *mut _ as *mut _,
                      s.as_mut_ptr(), b.as_mut_ptr() as *mut _, &ldb, x.as_mut_ptr() as *mut _,
                      &ldx, rcond, rpvgrw.as_mut_ptr(), berr.as_mut_ptr(), &n_err_bnds,
                      err_bnds_norm.as_mut_ptr(), err_bnds_comp.as_mut_ptr(), nparams.as_ptr(),
                      params.as_mut_ptr(), work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn sppsv(uplo: u8, n: i32, nrhs: i32, ap: &mut [f32], b: &mut [f32], ldb: i32,
             info: &mut i32) {

    unsafe {
        ffi::sppsv_(&(uplo as c_char), &n, &nrhs, ap.as_mut_ptr(), b.as_mut_ptr(), &ldb, info)
    }
}

#[inline]
pub fn dppsv(uplo: u8, n: i32, nrhs: i32, ap: &mut [f64], b: &mut [f64], ldb: i32,
             info: &mut i32) {

    unsafe {
        ffi::dppsv_(&(uplo as c_char), &n, &nrhs, ap.as_mut_ptr(), b.as_mut_ptr(), &ldb, info)
    }
}

#[inline]
pub fn cppsv(uplo: u8, n: i32, nrhs: i32, ap: &mut [c32], b: &mut [c32], ldb: i32,
             info: &mut i32) {

    unsafe {
        ffi::cppsv_(&(uplo as c_char), &n, &nrhs, ap.as_mut_ptr() as *mut _,
                    b.as_mut_ptr() as *mut _, &ldb, info)
    }
}

#[inline]
pub fn zppsv(uplo: u8, n: i32, nrhs: i32, ap: &mut [c64], b: &mut [c64], ldb: i32,
             info: &mut i32) {

    unsafe {
        ffi::zppsv_(&(uplo as c_char), &n, &nrhs, ap.as_mut_ptr() as *mut _,
                    b.as_mut_ptr() as *mut _, &ldb, info)
    }
}

#[inline]
pub fn sppsvx(fact: u8, uplo: u8, n: i32, nrhs: i32, ap: &mut [f32], afp: &mut [f32],
              equed: &mut u8, s: &mut [f32], b: &mut [f32], ldb: i32, x: &mut [f32], ldx: i32,
              rcond: &mut f32, ferr: &mut [f32], berr: &mut [f32], work: &mut [f32],
              iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::sppsvx_(&(fact as c_char), &(uplo as c_char), &n, &nrhs, ap.as_mut_ptr(),
                     afp.as_mut_ptr(), equed as *mut _ as *mut _, s.as_mut_ptr(), b.as_mut_ptr(),
                     &ldb, x.as_mut_ptr(), &ldx, rcond, ferr.as_mut_ptr(), berr.as_mut_ptr(),
                     work.as_mut_ptr(), iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dppsvx(fact: u8, uplo: u8, n: i32, nrhs: i32, ap: &mut [f64], afp: &mut [f64],
              equed: &mut u8, s: &mut [f64], b: &mut [f64], ldb: i32, x: &mut [f64], ldx: i32,
              rcond: &mut f64, ferr: &mut [f64], berr: &mut [f64], work: &mut [f64],
              iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::dppsvx_(&(fact as c_char), &(uplo as c_char), &n, &nrhs, ap.as_mut_ptr(),
                     afp.as_mut_ptr(), equed as *mut _ as *mut _, s.as_mut_ptr(), b.as_mut_ptr(),
                     &ldb, x.as_mut_ptr(), &ldx, rcond, ferr.as_mut_ptr(), berr.as_mut_ptr(),
                     work.as_mut_ptr(), iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn cppsvx(fact: u8, uplo: u8, n: i32, nrhs: i32, ap: &mut [c32], afp: &mut [c32],
              equed: &mut u8, s: &mut [f32], b: &mut [c32], ldb: i32, x: &mut [c32], ldx: i32,
              rcond: &mut f32, ferr: &mut [f32], berr: &mut [f32], work: &mut [c32],
              rwork: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::cppsvx_(&(fact as c_char), &(uplo as c_char), &n, &nrhs, ap.as_mut_ptr() as *mut _,
                     afp.as_mut_ptr() as *mut _, equed as *mut _ as *mut _, s.as_mut_ptr(),
                     b.as_mut_ptr() as *mut _, &ldb, x.as_mut_ptr() as *mut _, &ldx, rcond,
                     ferr.as_mut_ptr(), berr.as_mut_ptr(), work.as_mut_ptr() as *mut _,
                     rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zppsvx(fact: u8, uplo: u8, n: i32, nrhs: i32, ap: &mut [c64], afp: &mut [c64],
              equed: &mut u8, s: &mut [f64], b: &mut [c64], ldb: i32, x: &mut [c64], ldx: i32,
              rcond: &mut f64, ferr: &mut [f64], berr: &mut [f64], work: &mut [c64],
              rwork: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::zppsvx_(&(fact as c_char), &(uplo as c_char), &n, &nrhs, ap.as_mut_ptr() as *mut _,
                     afp.as_mut_ptr() as *mut _, equed as *mut _ as *mut _, s.as_mut_ptr(),
                     b.as_mut_ptr() as *mut _, &ldb, x.as_mut_ptr() as *mut _, &ldx, rcond,
                     ferr.as_mut_ptr(), berr.as_mut_ptr(), work.as_mut_ptr() as *mut _,
                     rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn spbsv(uplo: u8, n: i32, kd: i32, nrhs: i32, ab: &mut [f32], ldab: i32, b: &mut [f32],
             ldb: i32, info: &mut i32) {

    unsafe {
        ffi::spbsv_(&(uplo as c_char), &n, &kd, &nrhs, ab.as_mut_ptr(), &ldab, b.as_mut_ptr(),
                    &ldb, info)
    }
}

#[inline]
pub fn dpbsv(uplo: u8, n: i32, kd: i32, nrhs: i32, ab: &mut [f64], ldab: i32, b: &mut [f64],
             ldb: i32, info: &mut i32) {

    unsafe {
        ffi::dpbsv_(&(uplo as c_char), &n, &kd, &nrhs, ab.as_mut_ptr(), &ldab, b.as_mut_ptr(),
                    &ldb, info)
    }
}

#[inline]
pub fn cpbsv(uplo: u8, n: i32, kd: i32, nrhs: i32, ab: &mut [c32], ldab: i32, b: &mut [c32],
             ldb: i32, info: &mut i32) {

    unsafe {
        ffi::cpbsv_(&(uplo as c_char), &n, &kd, &nrhs, ab.as_mut_ptr() as *mut _, &ldab,
                    b.as_mut_ptr() as *mut _, &ldb, info)
    }
}

#[inline]
pub fn zpbsv(uplo: u8, n: i32, kd: i32, nrhs: i32, ab: &mut [c64], ldab: i32, b: &mut [c64],
             ldb: i32, info: &mut i32) {

    unsafe {
        ffi::zpbsv_(&(uplo as c_char), &n, &kd, &nrhs, ab.as_mut_ptr() as *mut _, &ldab,
                    b.as_mut_ptr() as *mut _, &ldb, info)
    }
}

#[inline]
pub fn spbsvx(fact: u8, uplo: u8, n: i32, kd: i32, nrhs: i32, ab: &mut [f32], ldab: i32,
              afb: &mut [f32], ldafb: i32, equed: &mut u8, s: &mut [f32], b: &mut [f32], ldb: i32,
              x: &mut [f32], ldx: i32, rcond: &mut f32, ferr: &mut [f32], berr: &mut [f32],
              work: &mut [f32], iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::spbsvx_(&(fact as c_char), &(uplo as c_char), &n, &kd, &nrhs, ab.as_mut_ptr(), &ldab,
                     afb.as_mut_ptr(), &ldafb, equed as *mut _ as *mut _, s.as_mut_ptr(),
                     b.as_mut_ptr(), &ldb, x.as_mut_ptr(), &ldx, rcond, ferr.as_mut_ptr(),
                     berr.as_mut_ptr(), work.as_mut_ptr(), iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dpbsvx(fact: u8, uplo: u8, n: i32, kd: i32, nrhs: i32, ab: &mut [f64], ldab: i32,
              afb: &mut [f64], ldafb: i32, equed: &mut u8, s: &mut [f64], b: &mut [f64], ldb: i32,
              x: &mut [f64], ldx: i32, rcond: &mut f64, ferr: &mut [f64], berr: &mut [f64],
              work: &mut [f64], iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::dpbsvx_(&(fact as c_char), &(uplo as c_char), &n, &kd, &nrhs, ab.as_mut_ptr(), &ldab,
                     afb.as_mut_ptr(), &ldafb, equed as *mut _ as *mut _, s.as_mut_ptr(),
                     b.as_mut_ptr(), &ldb, x.as_mut_ptr(), &ldx, rcond, ferr.as_mut_ptr(),
                     berr.as_mut_ptr(), work.as_mut_ptr(), iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn cpbsvx(fact: u8, uplo: u8, n: i32, kd: i32, nrhs: i32, ab: &mut [c32], ldab: i32,
              afb: &mut [c32], ldafb: i32, equed: &mut u8, s: &mut [f32], b: &mut [c32], ldb: i32,
              x: &mut [c32], ldx: i32, rcond: &mut f32, ferr: &mut [f32], berr: &mut [f32],
              work: &mut [c32], rwork: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::cpbsvx_(&(fact as c_char), &(uplo as c_char), &n, &kd, &nrhs,
                     ab.as_mut_ptr() as *mut _, &ldab, afb.as_mut_ptr() as *mut _, &ldafb,
                     equed as *mut _ as *mut _, s.as_mut_ptr(), b.as_mut_ptr() as *mut _, &ldb,
                     x.as_mut_ptr() as *mut _, &ldx, rcond, ferr.as_mut_ptr(), berr.as_mut_ptr(),
                     work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zpbsvx(fact: u8, uplo: u8, n: i32, kd: i32, nrhs: i32, ab: &mut [c64], ldab: i32,
              afb: &mut [c64], ldafb: i32, equed: &mut u8, s: &mut [f64], b: &mut [c64], ldb: i32,
              x: &mut [c64], ldx: i32, rcond: &mut f64, ferr: &mut [f64], berr: &mut [f64],
              work: &mut [c64], rwork: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::zpbsvx_(&(fact as c_char), &(uplo as c_char), &n, &kd, &nrhs,
                     ab.as_mut_ptr() as *mut _, &ldab, afb.as_mut_ptr() as *mut _, &ldafb,
                     equed as *mut _ as *mut _, s.as_mut_ptr(), b.as_mut_ptr() as *mut _, &ldb,
                     x.as_mut_ptr() as *mut _, &ldx, rcond, ferr.as_mut_ptr(), berr.as_mut_ptr(),
                     work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn sptsv(n: i32, nrhs: i32, d: &mut [f32], e: &mut [f32], b: &mut [f32], ldb: i32,
             info: &mut i32) {

    unsafe {
        ffi::sptsv_(&n, &nrhs, d.as_mut_ptr(), e.as_mut_ptr(), b.as_mut_ptr(), &ldb, info)
    }
}

#[inline]
pub fn dptsv(n: i32, nrhs: i32, d: &mut [f64], e: &mut [f64], b: &mut [f64], ldb: i32,
             info: &mut i32) {

    unsafe {
        ffi::dptsv_(&n, &nrhs, d.as_mut_ptr(), e.as_mut_ptr(), b.as_mut_ptr(), &ldb, info)
    }
}

#[inline]
pub fn cptsv(n: i32, nrhs: i32, d: &mut [f32], e: &mut [c32], b: &mut [c32], ldb: i32,
             info: &mut i32) {

    unsafe {
        ffi::cptsv_(&n, &nrhs, d.as_mut_ptr(), e.as_mut_ptr() as *mut _, b.as_mut_ptr() as *mut _,
                    &ldb, info)
    }
}

#[inline]
pub fn zptsv(n: i32, nrhs: i32, d: &mut [f64], e: &mut [c64], b: &mut [c64], ldb: i32,
             info: &mut i32) {

    unsafe {
        ffi::zptsv_(&n, &nrhs, d.as_mut_ptr(), e.as_mut_ptr() as *mut _, b.as_mut_ptr() as *mut _,
                    &ldb, info)
    }
}

#[inline]
pub fn sptsvx(fact: u8, n: i32, nrhs: i32, d: &[f32], e: &[f32], df: &mut [f32], ef: &mut [f32],
              b: &[f32], ldb: i32, x: &mut [f32], ldx: i32, rcond: &mut f32, ferr: &mut [f32],
              berr: &mut [f32], work: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::sptsvx_(&(fact as c_char), &n, &nrhs, d.as_ptr(), e.as_ptr(), df.as_mut_ptr(),
                     ef.as_mut_ptr(), b.as_ptr(), &ldb, x.as_mut_ptr(), &ldx, rcond,
                     ferr.as_mut_ptr(), berr.as_mut_ptr(), work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dptsvx(fact: u8, n: i32, nrhs: i32, d: &[f64], e: &[f64], df: &mut [f64], ef: &mut [f64],
              b: &[f64], ldb: i32, x: &mut [f64], ldx: i32, rcond: &mut f64, ferr: &mut [f64],
              berr: &mut [f64], work: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::dptsvx_(&(fact as c_char), &n, &nrhs, d.as_ptr(), e.as_ptr(), df.as_mut_ptr(),
                     ef.as_mut_ptr(), b.as_ptr(), &ldb, x.as_mut_ptr(), &ldx, rcond,
                     ferr.as_mut_ptr(), berr.as_mut_ptr(), work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn cptsvx(fact: u8, n: i32, nrhs: i32, d: &[f32], e: &[c32], df: &mut [f32], ef: &mut [c32],
              b: &[c32], ldb: i32, x: &mut [c32], ldx: i32, rcond: &mut f32, ferr: &mut [f32],
              berr: &mut [f32], work: &mut [c32], rwork: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::cptsvx_(&(fact as c_char), &n, &nrhs, d.as_ptr(), e.as_ptr() as *const _,
                     df.as_mut_ptr(), ef.as_mut_ptr() as *mut _, b.as_ptr() as *const _, &ldb,
                     x.as_mut_ptr() as *mut _, &ldx, rcond, ferr.as_mut_ptr(), berr.as_mut_ptr(),
                     work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zptsvx(fact: u8, n: i32, nrhs: i32, d: &[f64], e: &[c64], df: &mut [f64], ef: &mut [c64],
              b: &[c64], ldb: i32, x: &mut [c64], ldx: i32, rcond: &mut f64, ferr: &mut [f64],
              berr: &mut [f64], work: &mut [c64], rwork: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::zptsvx_(&(fact as c_char), &n, &nrhs, d.as_ptr(), e.as_ptr() as *const _,
                     df.as_mut_ptr(), ef.as_mut_ptr() as *mut _, b.as_ptr() as *const _, &ldb,
                     x.as_mut_ptr() as *mut _, &ldx, rcond, ferr.as_mut_ptr(), berr.as_mut_ptr(),
                     work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn ssysv(uplo: u8, n: i32, nrhs: i32, a: &mut [f32], lda: i32, ipiv: &mut [i32], b: &mut [f32],
             ldb: i32, work: &mut [f32], lwork: i32, info: &mut i32) {

    unsafe {
        ffi::ssysv_(&(uplo as c_char), &n, &nrhs, a.as_mut_ptr(), &lda, ipiv.as_mut_ptr(),
                    b.as_mut_ptr(), &ldb, work.as_mut_ptr(), &lwork, info)
    }
}

#[inline]
pub fn dsysv(uplo: u8, n: i32, nrhs: i32, a: &mut [f64], lda: i32, ipiv: &mut [i32], b: &mut [f64],
             ldb: i32, work: &mut [f64], lwork: i32, info: &mut i32) {

    unsafe {
        ffi::dsysv_(&(uplo as c_char), &n, &nrhs, a.as_mut_ptr(), &lda, ipiv.as_mut_ptr(),
                    b.as_mut_ptr(), &ldb, work.as_mut_ptr(), &lwork, info)
    }
}

#[inline]
pub fn csysv(uplo: u8, n: i32, nrhs: i32, a: &mut [c32], lda: i32, ipiv: &mut [i32], b: &mut [c32],
             ldb: i32, work: &mut [c32], lwork: i32, info: &mut i32) {

    unsafe {
        ffi::csysv_(&(uplo as c_char), &n, &nrhs, a.as_mut_ptr() as *mut _, &lda,
                    ipiv.as_mut_ptr(), b.as_mut_ptr() as *mut _, &ldb, work.as_mut_ptr() as *mut _,
                    &lwork, info)
    }
}

#[inline]
pub fn zsysv(uplo: u8, n: i32, nrhs: i32, a: &mut [c64], lda: i32, ipiv: &mut [i32], b: &mut [c64],
             ldb: i32, work: &mut [c64], lwork: i32, info: &mut i32) {

    unsafe {
        ffi::zsysv_(&(uplo as c_char), &n, &nrhs, a.as_mut_ptr() as *mut _, &lda,
                    ipiv.as_mut_ptr(), b.as_mut_ptr() as *mut _, &ldb, work.as_mut_ptr() as *mut _,
                    &lwork, info)
    }
}

#[inline]
pub fn ssysvx(fact: u8, uplo: u8, n: i32, nrhs: i32, a: &[f32], lda: i32, af: &mut [f32],
              ldaf: i32, ipiv: &mut [i32], b: &[f32], ldb: i32, x: &mut [f32], ldx: i32,
              rcond: &mut f32, ferr: &mut [f32], berr: &mut [f32], work: &mut [f32], lwork: i32,
              iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::ssysvx_(&(fact as c_char), &(uplo as c_char), &n, &nrhs, a.as_ptr(), &lda,
                     af.as_mut_ptr(), &ldaf, ipiv.as_mut_ptr(), b.as_ptr(), &ldb, x.as_mut_ptr(),
                     &ldx, rcond, ferr.as_mut_ptr(), berr.as_mut_ptr(), work.as_mut_ptr(), &lwork,
                     iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dsysvx(fact: u8, uplo: u8, n: i32, nrhs: i32, a: &[f64], lda: i32, af: &mut [f64],
              ldaf: i32, ipiv: &mut [i32], b: &[f64], ldb: i32, x: &mut [f64], ldx: i32,
              rcond: &mut f64, ferr: &mut [f64], berr: &mut [f64], work: &mut [f64], lwork: i32,
              iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::dsysvx_(&(fact as c_char), &(uplo as c_char), &n, &nrhs, a.as_ptr(), &lda,
                     af.as_mut_ptr(), &ldaf, ipiv.as_mut_ptr(), b.as_ptr(), &ldb, x.as_mut_ptr(),
                     &ldx, rcond, ferr.as_mut_ptr(), berr.as_mut_ptr(), work.as_mut_ptr(), &lwork,
                     iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn csysvx(fact: u8, uplo: u8, n: i32, nrhs: i32, a: &[c32], lda: i32, af: &mut [c32],
              ldaf: i32, ipiv: &mut [i32], b: &[c32], ldb: i32, x: &mut [c32], ldx: i32,
              rcond: &mut f32, ferr: &mut [f32], berr: &mut [f32], work: &mut [c32], lwork: i32,
              rwork: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::csysvx_(&(fact as c_char), &(uplo as c_char), &n, &nrhs, a.as_ptr() as *const _, &lda,
                     af.as_mut_ptr() as *mut _, &ldaf, ipiv.as_mut_ptr(), b.as_ptr() as *const _,
                     &ldb, x.as_mut_ptr() as *mut _, &ldx, rcond, ferr.as_mut_ptr(),
                     berr.as_mut_ptr(), work.as_mut_ptr() as *mut _, &lwork, rwork.as_mut_ptr(),
                     info)
    }
}

#[inline]
pub fn zsysvx(fact: u8, uplo: u8, n: i32, nrhs: i32, a: &[c64], lda: i32, af: &mut [c64],
              ldaf: i32, ipiv: &mut [i32], b: &[c64], ldb: i32, x: &mut [c64], ldx: i32,
              rcond: &mut f64, ferr: &mut [f64], berr: &mut [f64], work: &mut [c64], lwork: i32,
              rwork: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::zsysvx_(&(fact as c_char), &(uplo as c_char), &n, &nrhs, a.as_ptr() as *const _, &lda,
                     af.as_mut_ptr() as *mut _, &ldaf, ipiv.as_mut_ptr(), b.as_ptr() as *const _,
                     &ldb, x.as_mut_ptr() as *mut _, &ldx, rcond, ferr.as_mut_ptr(),
                     berr.as_mut_ptr(), work.as_mut_ptr() as *mut _, &lwork, rwork.as_mut_ptr(),
                     info)
    }
}

#[inline]
pub fn dsysvxx(fact: u8, uplo: u8, n: i32, nrhs: i32, a: &mut [f64], lda: i32, af: &mut [f64],
               ldaf: i32, ipiv: &mut [i32], equed: &mut u8, s: &mut [f64], b: &mut [f64], ldb: i32,
               x: &mut [f64], ldx: i32, rcond: &mut f64, rpvgrw: &mut [f64], berr: &mut [f64],
               n_err_bnds: i32, err_bnds_norm: &mut [f64], err_bnds_comp: &mut [f64],
               nparams: &[i32], params: &mut [f64], work: &mut [f64], iwork: &mut [i32],
               info: &mut i32) {

    unsafe {
        ffi::dsysvxx_(&(fact as c_char), &(uplo as c_char), &n, &nrhs, a.as_mut_ptr(), &lda,
                      af.as_mut_ptr(), &ldaf, ipiv.as_mut_ptr(), equed as *mut _ as *mut _,
                      s.as_mut_ptr(), b.as_mut_ptr(), &ldb, x.as_mut_ptr(), &ldx, rcond,
                      rpvgrw.as_mut_ptr(), berr.as_mut_ptr(), &n_err_bnds,
                      err_bnds_norm.as_mut_ptr(), err_bnds_comp.as_mut_ptr(), nparams.as_ptr(),
                      params.as_mut_ptr(), work.as_mut_ptr(), iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn ssysvxx(fact: u8, uplo: u8, n: i32, nrhs: i32, a: &mut [f32], lda: i32, af: &mut [f32],
               ldaf: i32, ipiv: &mut [i32], equed: &mut u8, s: &mut [f32], b: &mut [f32], ldb: i32,
               x: &mut [f32], ldx: i32, rcond: &mut f32, rpvgrw: &mut [f32], berr: &mut [f32],
               n_err_bnds: i32, err_bnds_norm: &mut [f32], err_bnds_comp: &mut [f32],
               nparams: &[i32], params: &mut [f32], work: &mut [f32], iwork: &mut [i32],
               info: &mut i32) {

    unsafe {
        ffi::ssysvxx_(&(fact as c_char), &(uplo as c_char), &n, &nrhs, a.as_mut_ptr(), &lda,
                      af.as_mut_ptr(), &ldaf, ipiv.as_mut_ptr(), equed as *mut _ as *mut _,
                      s.as_mut_ptr(), b.as_mut_ptr(), &ldb, x.as_mut_ptr(), &ldx, rcond,
                      rpvgrw.as_mut_ptr(), berr.as_mut_ptr(), &n_err_bnds,
                      err_bnds_norm.as_mut_ptr(), err_bnds_comp.as_mut_ptr(), nparams.as_ptr(),
                      params.as_mut_ptr(), work.as_mut_ptr(), iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zsysvxx(fact: u8, uplo: u8, n: i32, nrhs: i32, a: &mut [c64], lda: i32, af: &mut [c64],
               ldaf: i32, ipiv: &mut [i32], equed: &mut u8, s: &mut [f64], b: &mut [c64], ldb: i32,
               x: &mut [c64], ldx: i32, rcond: &mut f64, rpvgrw: &mut [f64], berr: &mut [f64],
               n_err_bnds: i32, err_bnds_norm: &mut [f64], err_bnds_comp: &mut [f64],
               nparams: &[i32], params: &mut [f64], work: &mut [c64], rwork: &mut [f64],
               info: &mut i32) {

    unsafe {
        ffi::zsysvxx_(&(fact as c_char), &(uplo as c_char), &n, &nrhs, a.as_mut_ptr() as *mut _,
                      &lda, af.as_mut_ptr() as *mut _, &ldaf, ipiv.as_mut_ptr(),
                      equed as *mut _ as *mut _, s.as_mut_ptr(), b.as_mut_ptr() as *mut _, &ldb,
                      x.as_mut_ptr() as *mut _, &ldx, rcond, rpvgrw.as_mut_ptr(),
                      berr.as_mut_ptr(), &n_err_bnds, err_bnds_norm.as_mut_ptr(),
                      err_bnds_comp.as_mut_ptr(), nparams.as_ptr(), params.as_mut_ptr(),
                      work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn csysvxx(fact: u8, uplo: u8, n: i32, nrhs: i32, a: &mut [c32], lda: i32, af: &mut [c32],
               ldaf: i32, ipiv: &mut [i32], equed: &mut u8, s: &mut [f32], b: &mut [c32], ldb: i32,
               x: &mut [c32], ldx: i32, rcond: &mut f32, rpvgrw: &mut [f32], berr: &mut [f32],
               n_err_bnds: i32, err_bnds_norm: &mut [f32], err_bnds_comp: &mut [f32],
               nparams: &[i32], params: &mut [f32], work: &mut [c32], rwork: &mut [f32],
               info: &mut i32) {

    unsafe {
        ffi::csysvxx_(&(fact as c_char), &(uplo as c_char), &n, &nrhs, a.as_mut_ptr() as *mut _,
                      &lda, af.as_mut_ptr() as *mut _, &ldaf, ipiv.as_mut_ptr(),
                      equed as *mut _ as *mut _, s.as_mut_ptr(), b.as_mut_ptr() as *mut _, &ldb,
                      x.as_mut_ptr() as *mut _, &ldx, rcond, rpvgrw.as_mut_ptr(),
                      berr.as_mut_ptr(), &n_err_bnds, err_bnds_norm.as_mut_ptr(),
                      err_bnds_comp.as_mut_ptr(), nparams.as_ptr(), params.as_mut_ptr(),
                      work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn chesv(uplo: u8, n: i32, nrhs: i32, a: &mut [c32], lda: i32, ipiv: &mut [i32], b: &mut [c32],
             ldb: i32, work: &mut [c32], lwork: i32, info: &mut i32) {

    unsafe {
        ffi::chesv_(&(uplo as c_char), &n, &nrhs, a.as_mut_ptr() as *mut _, &lda,
                    ipiv.as_mut_ptr(), b.as_mut_ptr() as *mut _, &ldb, work.as_mut_ptr() as *mut _,
                    &lwork, info)
    }
}

#[inline]
pub fn zhesv(uplo: u8, n: i32, nrhs: i32, a: &mut [c64], lda: i32, ipiv: &mut [i32], b: &mut [c64],
             ldb: i32, work: &mut [c64], lwork: i32, info: &mut i32) {

    unsafe {
        ffi::zhesv_(&(uplo as c_char), &n, &nrhs, a.as_mut_ptr() as *mut _, &lda,
                    ipiv.as_mut_ptr(), b.as_mut_ptr() as *mut _, &ldb, work.as_mut_ptr() as *mut _,
                    &lwork, info)
    }
}

#[inline]
pub fn chesvx(fact: u8, uplo: u8, n: i32, nrhs: i32, a: &[c32], lda: i32, af: &mut [c32],
              ldaf: i32, ipiv: &mut [i32], b: &[c32], ldb: i32, x: &mut [c32], ldx: i32,
              rcond: &mut f32, ferr: &mut [f32], berr: &mut [f32], work: &mut [c32], lwork: i32,
              rwork: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::chesvx_(&(fact as c_char), &(uplo as c_char), &n, &nrhs, a.as_ptr() as *const _, &lda,
                     af.as_mut_ptr() as *mut _, &ldaf, ipiv.as_mut_ptr(), b.as_ptr() as *const _,
                     &ldb, x.as_mut_ptr() as *mut _, &ldx, rcond, ferr.as_mut_ptr(),
                     berr.as_mut_ptr(), work.as_mut_ptr() as *mut _, &lwork, rwork.as_mut_ptr(),
                     info)
    }
}

#[inline]
pub fn zhesvx(fact: u8, uplo: u8, n: i32, nrhs: i32, a: &[c64], lda: i32, af: &mut [c64],
              ldaf: i32, ipiv: &mut [i32], b: &[c64], ldb: i32, x: &mut [c64], ldx: i32,
              rcond: &mut f64, ferr: &mut [f64], berr: &mut [f64], work: &mut [c64], lwork: i32,
              rwork: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::zhesvx_(&(fact as c_char), &(uplo as c_char), &n, &nrhs, a.as_ptr() as *const _, &lda,
                     af.as_mut_ptr() as *mut _, &ldaf, ipiv.as_mut_ptr(), b.as_ptr() as *const _,
                     &ldb, x.as_mut_ptr() as *mut _, &ldx, rcond, ferr.as_mut_ptr(),
                     berr.as_mut_ptr(), work.as_mut_ptr() as *mut _, &lwork, rwork.as_mut_ptr(),
                     info)
    }
}

#[inline]
pub fn zhesvxx(fact: u8, uplo: u8, n: i32, nrhs: i32, a: &mut [c64], lda: i32, af: &mut [c64],
               ldaf: i32, ipiv: &mut [i32], equed: &mut u8, s: &mut [f64], b: &mut [c64], ldb: i32,
               x: &mut [c64], ldx: i32, rcond: &mut f64, rpvgrw: &mut [f64], berr: &mut [f64],
               n_err_bnds: i32, err_bnds_norm: &mut [f64], err_bnds_comp: &mut [f64],
               nparams: &[i32], params: &mut [f64], work: &mut [c64], rwork: &mut [f64],
               info: &mut i32) {

    unsafe {
        ffi::zhesvxx_(&(fact as c_char), &(uplo as c_char), &n, &nrhs, a.as_mut_ptr() as *mut _,
                      &lda, af.as_mut_ptr() as *mut _, &ldaf, ipiv.as_mut_ptr(),
                      equed as *mut _ as *mut _, s.as_mut_ptr(), b.as_mut_ptr() as *mut _, &ldb,
                      x.as_mut_ptr() as *mut _, &ldx, rcond, rpvgrw.as_mut_ptr(),
                      berr.as_mut_ptr(), &n_err_bnds, err_bnds_norm.as_mut_ptr(),
                      err_bnds_comp.as_mut_ptr(), nparams.as_ptr(), params.as_mut_ptr(),
                      work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn chesvxx(fact: u8, uplo: u8, n: i32, nrhs: i32, a: &mut [c32], lda: i32, af: &mut [c32],
               ldaf: i32, ipiv: &mut [i32], equed: &mut u8, s: &mut [f32], b: &mut [c32], ldb: i32,
               x: &mut [c32], ldx: i32, rcond: &mut f32, rpvgrw: &mut [f32], berr: &mut [f32],
               n_err_bnds: i32, err_bnds_norm: &mut [f32], err_bnds_comp: &mut [f32],
               nparams: &[i32], params: &mut [f32], work: &mut [c32], rwork: &mut [f32],
               info: &mut i32) {

    unsafe {
        ffi::chesvxx_(&(fact as c_char), &(uplo as c_char), &n, &nrhs, a.as_mut_ptr() as *mut _,
                      &lda, af.as_mut_ptr() as *mut _, &ldaf, ipiv.as_mut_ptr(),
                      equed as *mut _ as *mut _, s.as_mut_ptr(), b.as_mut_ptr() as *mut _, &ldb,
                      x.as_mut_ptr() as *mut _, &ldx, rcond, rpvgrw.as_mut_ptr(),
                      berr.as_mut_ptr(), &n_err_bnds, err_bnds_norm.as_mut_ptr(),
                      err_bnds_comp.as_mut_ptr(), nparams.as_ptr(), params.as_mut_ptr(),
                      work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn sspsv(uplo: u8, n: i32, nrhs: i32, ap: &mut [f32], ipiv: &mut [i32], b: &mut [f32],
             ldb: i32, info: &mut i32) {

    unsafe {
        ffi::sspsv_(&(uplo as c_char), &n, &nrhs, ap.as_mut_ptr(), ipiv.as_mut_ptr(),
                    b.as_mut_ptr(), &ldb, info)
    }
}

#[inline]
pub fn dspsv(uplo: u8, n: i32, nrhs: i32, ap: &mut [f64], ipiv: &mut [i32], b: &mut [f64],
             ldb: i32, info: &mut i32) {

    unsafe {
        ffi::dspsv_(&(uplo as c_char), &n, &nrhs, ap.as_mut_ptr(), ipiv.as_mut_ptr(),
                    b.as_mut_ptr(), &ldb, info)
    }
}

#[inline]
pub fn cspsv(uplo: u8, n: i32, nrhs: i32, ap: &mut [c32], ipiv: &mut [i32], b: &mut [c32],
             ldb: i32, info: &mut i32) {

    unsafe {
        ffi::cspsv_(&(uplo as c_char), &n, &nrhs, ap.as_mut_ptr() as *mut _, ipiv.as_mut_ptr(),
                    b.as_mut_ptr() as *mut _, &ldb, info)
    }
}

#[inline]
pub fn zspsv(uplo: u8, n: i32, nrhs: i32, ap: &mut [c64], ipiv: &mut [i32], b: &mut [c64],
             ldb: i32, info: &mut i32) {

    unsafe {
        ffi::zspsv_(&(uplo as c_char), &n, &nrhs, ap.as_mut_ptr() as *mut _, ipiv.as_mut_ptr(),
                    b.as_mut_ptr() as *mut _, &ldb, info)
    }
}

#[inline]
pub fn sspsvx(fact: u8, uplo: u8, n: i32, nrhs: i32, ap: &[f32], afp: &mut [f32], ipiv: &mut [i32],
              b: &[f32], ldb: i32, x: &mut [f32], ldx: i32, rcond: &mut f32, ferr: &mut [f32],
              berr: &mut [f32], work: &mut [f32], iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::sspsvx_(&(fact as c_char), &(uplo as c_char), &n, &nrhs, ap.as_ptr(),
                     afp.as_mut_ptr(), ipiv.as_mut_ptr(), b.as_ptr(), &ldb, x.as_mut_ptr(), &ldx,
                     rcond, ferr.as_mut_ptr(), berr.as_mut_ptr(), work.as_mut_ptr(),
                     iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dspsvx(fact: u8, uplo: u8, n: i32, nrhs: i32, ap: &[f64], afp: &mut [f64], ipiv: &mut [i32],
              b: &[f64], ldb: i32, x: &mut [f64], ldx: i32, rcond: &mut f64, ferr: &mut [f64],
              berr: &mut [f64], work: &mut [f64], iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::dspsvx_(&(fact as c_char), &(uplo as c_char), &n, &nrhs, ap.as_ptr(),
                     afp.as_mut_ptr(), ipiv.as_mut_ptr(), b.as_ptr(), &ldb, x.as_mut_ptr(), &ldx,
                     rcond, ferr.as_mut_ptr(), berr.as_mut_ptr(), work.as_mut_ptr(),
                     iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn cspsvx(fact: u8, uplo: u8, n: i32, nrhs: i32, ap: &[c32], afp: &mut [c32], ipiv: &mut [i32],
              b: &[c32], ldb: i32, x: &mut [c32], ldx: i32, rcond: &mut f32, ferr: &mut [f32],
              berr: &mut [f32], work: &mut [c32], rwork: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::cspsvx_(&(fact as c_char), &(uplo as c_char), &n, &nrhs, ap.as_ptr() as *const _,
                     afp.as_mut_ptr() as *mut _, ipiv.as_mut_ptr(), b.as_ptr() as *const _, &ldb,
                     x.as_mut_ptr() as *mut _, &ldx, rcond, ferr.as_mut_ptr(), berr.as_mut_ptr(),
                     work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zspsvx(fact: u8, uplo: u8, n: i32, nrhs: i32, ap: &[c64], afp: &mut [c64], ipiv: &mut [i32],
              b: &[c64], ldb: i32, x: &mut [c64], ldx: i32, rcond: &mut f64, ferr: &mut [f64],
              berr: &mut [f64], work: &mut [c64], rwork: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::zspsvx_(&(fact as c_char), &(uplo as c_char), &n, &nrhs, ap.as_ptr() as *const _,
                     afp.as_mut_ptr() as *mut _, ipiv.as_mut_ptr(), b.as_ptr() as *const _, &ldb,
                     x.as_mut_ptr() as *mut _, &ldx, rcond, ferr.as_mut_ptr(), berr.as_mut_ptr(),
                     work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn chpsv(uplo: u8, n: i32, nrhs: i32, ap: &mut [c32], ipiv: &mut [i32], b: &mut [c32],
             ldb: i32, info: &mut i32) {

    unsafe {
        ffi::chpsv_(&(uplo as c_char), &n, &nrhs, ap.as_mut_ptr() as *mut _, ipiv.as_mut_ptr(),
                    b.as_mut_ptr() as *mut _, &ldb, info)
    }
}

#[inline]
pub fn zhpsv(uplo: u8, n: i32, nrhs: i32, ap: &mut [c64], ipiv: &mut [i32], b: &mut [c64],
             ldb: i32, info: &mut i32) {

    unsafe {
        ffi::zhpsv_(&(uplo as c_char), &n, &nrhs, ap.as_mut_ptr() as *mut _, ipiv.as_mut_ptr(),
                    b.as_mut_ptr() as *mut _, &ldb, info)
    }
}

#[inline]
pub fn chpsvx(fact: u8, uplo: u8, n: i32, nrhs: i32, ap: &[c32], afp: &mut [c32], ipiv: &mut [i32],
              b: &[c32], ldb: i32, x: &mut [c32], ldx: i32, rcond: &mut f32, ferr: &mut [f32],
              berr: &mut [f32], work: &mut [c32], rwork: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::chpsvx_(&(fact as c_char), &(uplo as c_char), &n, &nrhs, ap.as_ptr() as *const _,
                     afp.as_mut_ptr() as *mut _, ipiv.as_mut_ptr(), b.as_ptr() as *const _, &ldb,
                     x.as_mut_ptr() as *mut _, &ldx, rcond, ferr.as_mut_ptr(), berr.as_mut_ptr(),
                     work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zhpsvx(fact: u8, uplo: u8, n: i32, nrhs: i32, ap: &[c64], afp: &mut [c64], ipiv: &mut [i32],
              b: &[c64], ldb: i32, x: &mut [c64], ldx: i32, rcond: &mut f64, ferr: &mut [f64],
              berr: &mut [f64], work: &mut [c64], rwork: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::zhpsvx_(&(fact as c_char), &(uplo as c_char), &n, &nrhs, ap.as_ptr() as *const _,
                     afp.as_mut_ptr() as *mut _, ipiv.as_mut_ptr(), b.as_ptr() as *const _, &ldb,
                     x.as_mut_ptr() as *mut _, &ldx, rcond, ferr.as_mut_ptr(), berr.as_mut_ptr(),
                     work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn sgeqrf(m: i32, n: i32, a: &mut [f32], lda: i32, tau: &mut [f32], work: &mut [f32],
              lwork: i32, info: &mut i32) {

    unsafe {
        ffi::sgeqrf_(&m, &n, a.as_mut_ptr(), &lda, tau.as_mut_ptr(), work.as_mut_ptr(), &lwork,
                     info)
    }
}

#[inline]
pub fn dgeqrf(m: i32, n: i32, a: &mut [f64], lda: i32, tau: &mut [f64], work: &mut [f64],
              lwork: i32, info: &mut i32) {

    unsafe {
        ffi::dgeqrf_(&m, &n, a.as_mut_ptr(), &lda, tau.as_mut_ptr(), work.as_mut_ptr(), &lwork,
                     info)
    }
}

#[inline]
pub fn cgeqrf(m: i32, n: i32, a: &mut [c32], lda: i32, tau: &mut [c32], work: &mut [c32],
              lwork: i32, info: &mut i32) {

    unsafe {
        ffi::cgeqrf_(&m, &n, a.as_mut_ptr() as *mut _, &lda, tau.as_mut_ptr() as *mut _,
                     work.as_mut_ptr() as *mut _, &lwork, info)
    }
}

#[inline]
pub fn zgeqrf(m: i32, n: i32, a: &mut [c64], lda: i32, tau: &mut [c64], work: &mut [c64],
              lwork: i32, info: &mut i32) {

    unsafe {
        ffi::zgeqrf_(&m, &n, a.as_mut_ptr() as *mut _, &lda, tau.as_mut_ptr() as *mut _,
                     work.as_mut_ptr() as *mut _, &lwork, info)
    }
}

#[inline]
pub fn sgeqpf(m: i32, n: i32, a: &mut [f32], lda: i32, jpvt: &mut [i32], tau: &mut [f32],
              work: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::sgeqpf_(&m, &n, a.as_mut_ptr(), &lda, jpvt.as_mut_ptr(), tau.as_mut_ptr(),
                     work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dgeqpf(m: i32, n: i32, a: &mut [f64], lda: i32, jpvt: &mut [i32], tau: &mut [f64],
              work: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::dgeqpf_(&m, &n, a.as_mut_ptr(), &lda, jpvt.as_mut_ptr(), tau.as_mut_ptr(),
                     work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn cgeqpf(m: i32, n: i32, a: &mut [c32], lda: i32, jpvt: &mut [i32], tau: &mut [c32],
              work: &mut [c32], rwork: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::cgeqpf_(&m, &n, a.as_mut_ptr() as *mut _, &lda, jpvt.as_mut_ptr(),
                     tau.as_mut_ptr() as *mut _, work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(),
                     info)
    }
}

#[inline]
pub fn zgeqpf(m: i32, n: i32, a: &mut [c64], lda: i32, jpvt: &mut [i32], tau: &mut [c64],
              work: &mut [c64], rwork: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::zgeqpf_(&m, &n, a.as_mut_ptr() as *mut _, &lda, jpvt.as_mut_ptr(),
                     tau.as_mut_ptr() as *mut _, work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(),
                     info)
    }
}

#[inline]
pub fn sgeqp3(m: i32, n: i32, a: &mut [f32], lda: i32, jpvt: &mut [i32], tau: &mut [f32],
              work: &mut [f32], lwork: i32, info: &mut i32) {

    unsafe {
        ffi::sgeqp3_(&m, &n, a.as_mut_ptr(), &lda, jpvt.as_mut_ptr(), tau.as_mut_ptr(),
                     work.as_mut_ptr(), &lwork, info)
    }
}

#[inline]
pub fn dgeqp3(m: i32, n: i32, a: &mut [f64], lda: i32, jpvt: &mut [i32], tau: &mut [f64],
              work: &mut [f64], lwork: i32, info: &mut i32) {

    unsafe {
        ffi::dgeqp3_(&m, &n, a.as_mut_ptr(), &lda, jpvt.as_mut_ptr(), tau.as_mut_ptr(),
                     work.as_mut_ptr(), &lwork, info)
    }
}

#[inline]
pub fn cgeqp3(m: i32, n: i32, a: &mut [c32], lda: i32, jpvt: &mut [i32], tau: &mut [c32],
              work: &mut [c32], lwork: i32, rwork: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::cgeqp3_(&m, &n, a.as_mut_ptr() as *mut _, &lda, jpvt.as_mut_ptr(),
                     tau.as_mut_ptr() as *mut _, work.as_mut_ptr() as *mut _, &lwork,
                     rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zgeqp3(m: i32, n: i32, a: &mut [c64], lda: i32, jpvt: &mut [i32], tau: &mut [c64],
              work: &mut [c64], lwork: i32, rwork: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::zgeqp3_(&m, &n, a.as_mut_ptr() as *mut _, &lda, jpvt.as_mut_ptr(),
                     tau.as_mut_ptr() as *mut _, work.as_mut_ptr() as *mut _, &lwork,
                     rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn sorgqr(m: i32, n: i32, k: i32, a: &mut [f32], lda: i32, tau: &[f32], work: &mut [f32],
              lwork: i32, info: &mut i32) {

    unsafe {
        ffi::sorgqr_(&m, &n, &k, a.as_mut_ptr(), &lda, tau.as_ptr(), work.as_mut_ptr(), &lwork,
                     info)
    }
}

#[inline]
pub fn dorgqr(m: i32, n: i32, k: i32, a: &mut [f64], lda: i32, tau: &[f64], work: &mut [f64],
              lwork: i32, info: &mut i32) {

    unsafe {
        ffi::dorgqr_(&m, &n, &k, a.as_mut_ptr(), &lda, tau.as_ptr(), work.as_mut_ptr(), &lwork,
                     info)
    }
}

#[inline]
pub fn sormqr(side: u8, trans: u8, m: i32, n: i32, k: i32, a: &[f32], lda: i32, tau: &[f32],
              c: &mut [f32], ldc: i32, work: &mut [f32], lwork: i32, info: &mut i32) {

    unsafe {
        ffi::sormqr_(&(side as c_char), &(trans as c_char), &m, &n, &k, a.as_ptr(), &lda,
                     tau.as_ptr(), c.as_mut_ptr(), &ldc, work.as_mut_ptr(), &lwork, info)
    }
}

#[inline]
pub fn dormqr(side: u8, trans: u8, m: i32, n: i32, k: i32, a: &[f64], lda: i32, tau: &[f64],
              c: &mut [f64], ldc: i32, work: &mut [f64], lwork: i32, info: &mut i32) {

    unsafe {
        ffi::dormqr_(&(side as c_char), &(trans as c_char), &m, &n, &k, a.as_ptr(), &lda,
                     tau.as_ptr(), c.as_mut_ptr(), &ldc, work.as_mut_ptr(), &lwork, info)
    }
}

#[inline]
pub fn cungqr(m: i32, n: i32, k: i32, a: &mut [c32], lda: i32, tau: &[c32], work: &mut [c32],
              lwork: i32, info: &mut i32) {

    unsafe {
        ffi::cungqr_(&m, &n, &k, a.as_mut_ptr() as *mut _, &lda, tau.as_ptr() as *const _,
                     work.as_mut_ptr() as *mut _, &lwork, info)
    }
}

#[inline]
pub fn zungqr(m: i32, n: i32, k: i32, a: &mut [c64], lda: i32, tau: &[c64], work: &mut [c64],
              lwork: i32, info: &mut i32) {

    unsafe {
        ffi::zungqr_(&m, &n, &k, a.as_mut_ptr() as *mut _, &lda, tau.as_ptr() as *const _,
                     work.as_mut_ptr() as *mut _, &lwork, info)
    }
}

#[inline]
pub fn cunmqr(side: u8, trans: u8, m: i32, n: i32, k: i32, a: &[c32], lda: i32, tau: &[c32],
              c: &mut [c32], ldc: i32, work: &mut [c32], lwork: i32, info: &mut i32) {

    unsafe {
        ffi::cunmqr_(&(side as c_char), &(trans as c_char), &m, &n, &k, a.as_ptr() as *const _,
                     &lda, tau.as_ptr() as *const _, c.as_mut_ptr() as *mut _, &ldc,
                     work.as_mut_ptr() as *mut _, &lwork, info)
    }
}

#[inline]
pub fn zunmqr(side: u8, trans: u8, m: i32, n: i32, k: i32, a: &[c64], lda: i32, tau: &[c64],
              c: &mut [c64], ldc: i32, work: &mut [c64], lwork: i32, info: &mut i32) {

    unsafe {
        ffi::zunmqr_(&(side as c_char), &(trans as c_char), &m, &n, &k, a.as_ptr() as *const _,
                     &lda, tau.as_ptr() as *const _, c.as_mut_ptr() as *mut _, &ldc,
                     work.as_mut_ptr() as *mut _, &lwork, info)
    }
}

#[inline]
pub fn sgelqf(m: i32, n: i32, a: &mut [f32], lda: i32, tau: &mut [f32], work: &mut [f32],
              lwork: i32, info: &mut i32) {

    unsafe {
        ffi::sgelqf_(&m, &n, a.as_mut_ptr(), &lda, tau.as_mut_ptr(), work.as_mut_ptr(), &lwork,
                     info)
    }
}

#[inline]
pub fn dgelqf(m: i32, n: i32, a: &mut [f64], lda: i32, tau: &mut [f64], work: &mut [f64],
              lwork: i32, info: &mut i32) {

    unsafe {
        ffi::dgelqf_(&m, &n, a.as_mut_ptr(), &lda, tau.as_mut_ptr(), work.as_mut_ptr(), &lwork,
                     info)
    }
}

#[inline]
pub fn cgelqf(m: i32, n: i32, a: &mut [c32], lda: i32, tau: &mut [c32], work: &mut [c32],
              lwork: i32, info: &mut i32) {

    unsafe {
        ffi::cgelqf_(&m, &n, a.as_mut_ptr() as *mut _, &lda, tau.as_mut_ptr() as *mut _,
                     work.as_mut_ptr() as *mut _, &lwork, info)
    }
}

#[inline]
pub fn zgelqf(m: i32, n: i32, a: &mut [c64], lda: i32, tau: &mut [c64], work: &mut [c64],
              lwork: i32, info: &mut i32) {

    unsafe {
        ffi::zgelqf_(&m, &n, a.as_mut_ptr() as *mut _, &lda, tau.as_mut_ptr() as *mut _,
                     work.as_mut_ptr() as *mut _, &lwork, info)
    }
}

#[inline]
pub fn sorglq(m: i32, n: i32, k: i32, a: &mut [f32], lda: i32, tau: &[f32], work: &mut [f32],
              lwork: i32, info: &mut i32) {

    unsafe {
        ffi::sorglq_(&m, &n, &k, a.as_mut_ptr(), &lda, tau.as_ptr(), work.as_mut_ptr(), &lwork,
                     info)
    }
}

#[inline]
pub fn dorglq(m: i32, n: i32, k: i32, a: &mut [f64], lda: i32, tau: &[f64], work: &mut [f64],
              lwork: i32, info: &mut i32) {

    unsafe {
        ffi::dorglq_(&m, &n, &k, a.as_mut_ptr(), &lda, tau.as_ptr(), work.as_mut_ptr(), &lwork,
                     info)
    }
}

#[inline]
pub fn sormlq(side: u8, trans: u8, m: i32, n: i32, k: i32, a: &[f32], lda: i32, tau: &[f32],
              c: &mut [f32], ldc: i32, work: &mut [f32], lwork: i32, info: &mut i32) {

    unsafe {
        ffi::sormlq_(&(side as c_char), &(trans as c_char), &m, &n, &k, a.as_ptr(), &lda,
                     tau.as_ptr(), c.as_mut_ptr(), &ldc, work.as_mut_ptr(), &lwork, info)
    }
}

#[inline]
pub fn dormlq(side: u8, trans: u8, m: i32, n: i32, k: i32, a: &[f64], lda: i32, tau: &[f64],
              c: &mut [f64], ldc: i32, work: &mut [f64], lwork: i32, info: &mut i32) {

    unsafe {
        ffi::dormlq_(&(side as c_char), &(trans as c_char), &m, &n, &k, a.as_ptr(), &lda,
                     tau.as_ptr(), c.as_mut_ptr(), &ldc, work.as_mut_ptr(), &lwork, info)
    }
}

#[inline]
pub fn cunglq(m: i32, n: i32, k: i32, a: &mut [c32], lda: i32, tau: &[c32], work: &mut [c32],
              lwork: i32, info: &mut i32) {

    unsafe {
        ffi::cunglq_(&m, &n, &k, a.as_mut_ptr() as *mut _, &lda, tau.as_ptr() as *const _,
                     work.as_mut_ptr() as *mut _, &lwork, info)
    }
}

#[inline]
pub fn zunglq(m: i32, n: i32, k: i32, a: &mut [c64], lda: i32, tau: &[c64], work: &mut [c64],
              lwork: i32, info: &mut i32) {

    unsafe {
        ffi::zunglq_(&m, &n, &k, a.as_mut_ptr() as *mut _, &lda, tau.as_ptr() as *const _,
                     work.as_mut_ptr() as *mut _, &lwork, info)
    }
}

#[inline]
pub fn cunmlq(side: u8, trans: u8, m: i32, n: i32, k: i32, a: &[c32], lda: i32, tau: &[c32],
              c: &mut [c32], ldc: i32, work: &mut [c32], lwork: i32, info: &mut i32) {

    unsafe {
        ffi::cunmlq_(&(side as c_char), &(trans as c_char), &m, &n, &k, a.as_ptr() as *const _,
                     &lda, tau.as_ptr() as *const _, c.as_mut_ptr() as *mut _, &ldc,
                     work.as_mut_ptr() as *mut _, &lwork, info)
    }
}

#[inline]
pub fn zunmlq(side: u8, trans: u8, m: i32, n: i32, k: i32, a: &[c64], lda: i32, tau: &[c64],
              c: &mut [c64], ldc: i32, work: &mut [c64], lwork: i32, info: &mut i32) {

    unsafe {
        ffi::zunmlq_(&(side as c_char), &(trans as c_char), &m, &n, &k, a.as_ptr() as *const _,
                     &lda, tau.as_ptr() as *const _, c.as_mut_ptr() as *mut _, &ldc,
                     work.as_mut_ptr() as *mut _, &lwork, info)
    }
}

#[inline]
pub fn sgeqlf(m: i32, n: i32, a: &mut [f32], lda: i32, tau: &mut [f32], work: &mut [f32],
              lwork: i32, info: &mut i32) {

    unsafe {
        ffi::sgeqlf_(&m, &n, a.as_mut_ptr(), &lda, tau.as_mut_ptr(), work.as_mut_ptr(), &lwork,
                     info)
    }
}

#[inline]
pub fn dgeqlf(m: i32, n: i32, a: &mut [f64], lda: i32, tau: &mut [f64], work: &mut [f64],
              lwork: i32, info: &mut i32) {

    unsafe {
        ffi::dgeqlf_(&m, &n, a.as_mut_ptr(), &lda, tau.as_mut_ptr(), work.as_mut_ptr(), &lwork,
                     info)
    }
}

#[inline]
pub fn cgeqlf(m: i32, n: i32, a: &mut [c32], lda: i32, tau: &mut [c32], work: &mut [c32],
              lwork: i32, info: &mut i32) {

    unsafe {
        ffi::cgeqlf_(&m, &n, a.as_mut_ptr() as *mut _, &lda, tau.as_mut_ptr() as *mut _,
                     work.as_mut_ptr() as *mut _, &lwork, info)
    }
}

#[inline]
pub fn zgeqlf(m: i32, n: i32, a: &mut [c64], lda: i32, tau: &mut [c64], work: &mut [c64],
              lwork: i32, info: &mut i32) {

    unsafe {
        ffi::zgeqlf_(&m, &n, a.as_mut_ptr() as *mut _, &lda, tau.as_mut_ptr() as *mut _,
                     work.as_mut_ptr() as *mut _, &lwork, info)
    }
}

#[inline]
pub fn sorgql(m: i32, n: i32, k: i32, a: &mut [f32], lda: i32, tau: &[f32], work: &mut [f32],
              lwork: i32, info: &mut i32) {

    unsafe {
        ffi::sorgql_(&m, &n, &k, a.as_mut_ptr(), &lda, tau.as_ptr(), work.as_mut_ptr(), &lwork,
                     info)
    }
}

#[inline]
pub fn dorgql(m: i32, n: i32, k: i32, a: &mut [f64], lda: i32, tau: &[f64], work: &mut [f64],
              lwork: i32, info: &mut i32) {

    unsafe {
        ffi::dorgql_(&m, &n, &k, a.as_mut_ptr(), &lda, tau.as_ptr(), work.as_mut_ptr(), &lwork,
                     info)
    }
}

#[inline]
pub fn cungql(m: i32, n: i32, k: i32, a: &mut [c32], lda: i32, tau: &[c32], work: &mut [c32],
              lwork: i32, info: &mut i32) {

    unsafe {
        ffi::cungql_(&m, &n, &k, a.as_mut_ptr() as *mut _, &lda, tau.as_ptr() as *const _,
                     work.as_mut_ptr() as *mut _, &lwork, info)
    }
}

#[inline]
pub fn zungql(m: i32, n: i32, k: i32, a: &mut [c64], lda: i32, tau: &[c64], work: &mut [c64],
              lwork: i32, info: &mut i32) {

    unsafe {
        ffi::zungql_(&m, &n, &k, a.as_mut_ptr() as *mut _, &lda, tau.as_ptr() as *const _,
                     work.as_mut_ptr() as *mut _, &lwork, info)
    }
}

#[inline]
pub fn sormql(side: u8, trans: u8, m: i32, n: i32, k: i32, a: &[f32], lda: i32, tau: &[f32],
              c: &mut [f32], ldc: i32, work: &mut [f32], lwork: i32, info: &mut i32) {

    unsafe {
        ffi::sormql_(&(side as c_char), &(trans as c_char), &m, &n, &k, a.as_ptr(), &lda,
                     tau.as_ptr(), c.as_mut_ptr(), &ldc, work.as_mut_ptr(), &lwork, info)
    }
}

#[inline]
pub fn dormql(side: u8, trans: u8, m: i32, n: i32, k: i32, a: &[f64], lda: i32, tau: &[f64],
              c: &mut [f64], ldc: i32, work: &mut [f64], lwork: i32, info: &mut i32) {

    unsafe {
        ffi::dormql_(&(side as c_char), &(trans as c_char), &m, &n, &k, a.as_ptr(), &lda,
                     tau.as_ptr(), c.as_mut_ptr(), &ldc, work.as_mut_ptr(), &lwork, info)
    }
}

#[inline]
pub fn cunmql(side: u8, trans: u8, m: i32, n: i32, k: i32, a: &[c32], lda: i32, tau: &[c32],
              c: &mut [c32], ldc: i32, work: &mut [c32], lwork: i32, info: &mut i32) {

    unsafe {
        ffi::cunmql_(&(side as c_char), &(trans as c_char), &m, &n, &k, a.as_ptr() as *const _,
                     &lda, tau.as_ptr() as *const _, c.as_mut_ptr() as *mut _, &ldc,
                     work.as_mut_ptr() as *mut _, &lwork, info)
    }
}

#[inline]
pub fn zunmql(side: u8, trans: u8, m: i32, n: i32, k: i32, a: &[c64], lda: i32, tau: &[c64],
              c: &mut [c64], ldc: i32, work: &mut [c64], lwork: i32, info: &mut i32) {

    unsafe {
        ffi::zunmql_(&(side as c_char), &(trans as c_char), &m, &n, &k, a.as_ptr() as *const _,
                     &lda, tau.as_ptr() as *const _, c.as_mut_ptr() as *mut _, &ldc,
                     work.as_mut_ptr() as *mut _, &lwork, info)
    }
}

#[inline]
pub fn sgerqf(m: i32, n: i32, a: &mut [f32], lda: i32, tau: &mut [f32], work: &mut [f32],
              lwork: i32, info: &mut i32) {

    unsafe {
        ffi::sgerqf_(&m, &n, a.as_mut_ptr(), &lda, tau.as_mut_ptr(), work.as_mut_ptr(), &lwork,
                     info)
    }
}

#[inline]
pub fn dgerqf(m: i32, n: i32, a: &mut [f64], lda: i32, tau: &mut [f64], work: &mut [f64],
              lwork: i32, info: &mut i32) {

    unsafe {
        ffi::dgerqf_(&m, &n, a.as_mut_ptr(), &lda, tau.as_mut_ptr(), work.as_mut_ptr(), &lwork,
                     info)
    }
}

#[inline]
pub fn cgerqf(m: i32, n: i32, a: &mut [c32], lda: i32, tau: &mut [c32], work: &mut [c32],
              lwork: i32, info: &mut i32) {

    unsafe {
        ffi::cgerqf_(&m, &n, a.as_mut_ptr() as *mut _, &lda, tau.as_mut_ptr() as *mut _,
                     work.as_mut_ptr() as *mut _, &lwork, info)
    }
}

#[inline]
pub fn zgerqf(m: i32, n: i32, a: &mut [c64], lda: i32, tau: &mut [c64], work: &mut [c64],
              lwork: i32, info: &mut i32) {

    unsafe {
        ffi::zgerqf_(&m, &n, a.as_mut_ptr() as *mut _, &lda, tau.as_mut_ptr() as *mut _,
                     work.as_mut_ptr() as *mut _, &lwork, info)
    }
}

#[inline]
pub fn sorgrq(m: i32, n: i32, k: i32, a: &mut [f32], lda: i32, tau: &[f32], work: &mut [f32],
              lwork: i32, info: &mut i32) {

    unsafe {
        ffi::sorgrq_(&m, &n, &k, a.as_mut_ptr(), &lda, tau.as_ptr(), work.as_mut_ptr(), &lwork,
                     info)
    }
}

#[inline]
pub fn dorgrq(m: i32, n: i32, k: i32, a: &mut [f64], lda: i32, tau: &[f64], work: &mut [f64],
              lwork: i32, info: &mut i32) {

    unsafe {
        ffi::dorgrq_(&m, &n, &k, a.as_mut_ptr(), &lda, tau.as_ptr(), work.as_mut_ptr(), &lwork,
                     info)
    }
}

#[inline]
pub fn cungrq(m: i32, n: i32, k: i32, a: &mut [c32], lda: i32, tau: &[c32], work: &mut [c32],
              lwork: i32, info: &mut i32) {

    unsafe {
        ffi::cungrq_(&m, &n, &k, a.as_mut_ptr() as *mut _, &lda, tau.as_ptr() as *const _,
                     work.as_mut_ptr() as *mut _, &lwork, info)
    }
}

#[inline]
pub fn zungrq(m: i32, n: i32, k: i32, a: &mut [c64], lda: i32, tau: &[c64], work: &mut [c64],
              lwork: i32, info: &mut i32) {

    unsafe {
        ffi::zungrq_(&m, &n, &k, a.as_mut_ptr() as *mut _, &lda, tau.as_ptr() as *const _,
                     work.as_mut_ptr() as *mut _, &lwork, info)
    }
}

#[inline]
pub fn sormrq(side: u8, trans: u8, m: i32, n: i32, k: i32, a: &[f32], lda: i32, tau: &[f32],
              c: &mut [f32], ldc: i32, work: &mut [f32], lwork: i32, info: &mut i32) {

    unsafe {
        ffi::sormrq_(&(side as c_char), &(trans as c_char), &m, &n, &k, a.as_ptr(), &lda,
                     tau.as_ptr(), c.as_mut_ptr(), &ldc, work.as_mut_ptr(), &lwork, info)
    }
}

#[inline]
pub fn dormrq(side: u8, trans: u8, m: i32, n: i32, k: i32, a: &[f64], lda: i32, tau: &[f64],
              c: &mut [f64], ldc: i32, work: &mut [f64], lwork: i32, info: &mut i32) {

    unsafe {
        ffi::dormrq_(&(side as c_char), &(trans as c_char), &m, &n, &k, a.as_ptr(), &lda,
                     tau.as_ptr(), c.as_mut_ptr(), &ldc, work.as_mut_ptr(), &lwork, info)
    }
}

#[inline]
pub fn cunmrq(side: u8, trans: u8, m: i32, n: i32, k: i32, a: &[c32], lda: i32, tau: &[c32],
              c: &mut [c32], ldc: i32, work: &mut [c32], lwork: i32, info: &mut i32) {

    unsafe {
        ffi::cunmrq_(&(side as c_char), &(trans as c_char), &m, &n, &k, a.as_ptr() as *const _,
                     &lda, tau.as_ptr() as *const _, c.as_mut_ptr() as *mut _, &ldc,
                     work.as_mut_ptr() as *mut _, &lwork, info)
    }
}

#[inline]
pub fn zunmrq(side: u8, trans: u8, m: i32, n: i32, k: i32, a: &[c64], lda: i32, tau: &[c64],
              c: &mut [c64], ldc: i32, work: &mut [c64], lwork: i32, info: &mut i32) {

    unsafe {
        ffi::zunmrq_(&(side as c_char), &(trans as c_char), &m, &n, &k, a.as_ptr() as *const _,
                     &lda, tau.as_ptr() as *const _, c.as_mut_ptr() as *mut _, &ldc,
                     work.as_mut_ptr() as *mut _, &lwork, info)
    }
}

#[inline]
pub fn stzrzf(m: i32, n: i32, a: &mut [f32], lda: i32, tau: &mut [f32], work: &mut [f32],
              lwork: i32, info: &mut i32) {

    unsafe {
        ffi::stzrzf_(&m, &n, a.as_mut_ptr(), &lda, tau.as_mut_ptr(), work.as_mut_ptr(), &lwork,
                     info)
    }
}

#[inline]
pub fn dtzrzf(m: i32, n: i32, a: &mut [f64], lda: i32, tau: &mut [f64], work: &mut [f64],
              lwork: i32, info: &mut i32) {

    unsafe {
        ffi::dtzrzf_(&m, &n, a.as_mut_ptr(), &lda, tau.as_mut_ptr(), work.as_mut_ptr(), &lwork,
                     info)
    }
}

#[inline]
pub fn ctzrzf(m: i32, n: i32, a: &mut [c32], lda: i32, tau: &mut [c32], work: &mut [c32],
              lwork: i32, info: &mut i32) {

    unsafe {
        ffi::ctzrzf_(&m, &n, a.as_mut_ptr() as *mut _, &lda, tau.as_mut_ptr() as *mut _,
                     work.as_mut_ptr() as *mut _, &lwork, info)
    }
}

#[inline]
pub fn ztzrzf(m: i32, n: i32, a: &mut [c64], lda: i32, tau: &mut [c64], work: &mut [c64],
              lwork: i32, info: &mut i32) {

    unsafe {
        ffi::ztzrzf_(&m, &n, a.as_mut_ptr() as *mut _, &lda, tau.as_mut_ptr() as *mut _,
                     work.as_mut_ptr() as *mut _, &lwork, info)
    }
}

#[inline]
pub fn sormrz(side: u8, trans: u8, m: i32, n: i32, k: i32, l: i32, a: &[f32], lda: i32,
              tau: &[f32], c: &mut [f32], ldc: i32, work: &mut [f32], lwork: i32, info: &mut i32) {

    unsafe {
        ffi::sormrz_(&(side as c_char), &(trans as c_char), &m, &n, &k, &l, a.as_ptr(), &lda,
                     tau.as_ptr(), c.as_mut_ptr(), &ldc, work.as_mut_ptr(), &lwork, info)
    }
}

#[inline]
pub fn dormrz(side: u8, trans: u8, m: i32, n: i32, k: i32, l: i32, a: &[f64], lda: i32,
              tau: &[f64], c: &mut [f64], ldc: i32, work: &mut [f64], lwork: i32, info: &mut i32) {

    unsafe {
        ffi::dormrz_(&(side as c_char), &(trans as c_char), &m, &n, &k, &l, a.as_ptr(), &lda,
                     tau.as_ptr(), c.as_mut_ptr(), &ldc, work.as_mut_ptr(), &lwork, info)
    }
}

#[inline]
pub fn cunmrz(side: u8, trans: u8, m: i32, n: i32, k: i32, l: i32, a: &[c32], lda: i32,
              tau: &[c32], c: &mut [c32], ldc: i32, work: &mut [c32], lwork: i32, info: &mut i32) {

    unsafe {
        ffi::cunmrz_(&(side as c_char), &(trans as c_char), &m, &n, &k, &l, a.as_ptr() as *const _,
                     &lda, tau.as_ptr() as *const _, c.as_mut_ptr() as *mut _, &ldc,
                     work.as_mut_ptr() as *mut _, &lwork, info)
    }
}

#[inline]
pub fn zunmrz(side: u8, trans: u8, m: i32, n: i32, k: i32, l: i32, a: &[c64], lda: i32,
              tau: &[c64], c: &mut [c64], ldc: i32, work: &mut [c64], lwork: i32, info: &mut i32) {

    unsafe {
        ffi::zunmrz_(&(side as c_char), &(trans as c_char), &m, &n, &k, &l, a.as_ptr() as *const _,
                     &lda, tau.as_ptr() as *const _, c.as_mut_ptr() as *mut _, &ldc,
                     work.as_mut_ptr() as *mut _, &lwork, info)
    }
}

#[inline]
pub fn sggqrf(n: i32, m: i32, p: i32, a: &mut [f32], lda: i32, taua: &mut [f32], b: &mut [f32],
              ldb: i32, taub: &mut [f32], work: &mut [f32], lwork: i32, info: &mut i32) {

    unsafe {
        ffi::sggqrf_(&n, &m, &p, a.as_mut_ptr(), &lda, taua.as_mut_ptr(), b.as_mut_ptr(), &ldb,
                     taub.as_mut_ptr(), work.as_mut_ptr(), &lwork, info)
    }
}

#[inline]
pub fn dggqrf(n: i32, m: i32, p: i32, a: &mut [f64], lda: i32, taua: &mut [f64], b: &mut [f64],
              ldb: i32, taub: &mut [f64], work: &mut [f64], lwork: i32, info: &mut i32) {

    unsafe {
        ffi::dggqrf_(&n, &m, &p, a.as_mut_ptr(), &lda, taua.as_mut_ptr(), b.as_mut_ptr(), &ldb,
                     taub.as_mut_ptr(), work.as_mut_ptr(), &lwork, info)
    }
}

#[inline]
pub fn cggqrf(n: i32, m: i32, p: i32, a: &mut [c32], lda: i32, taua: &mut [c32], b: &mut [c32],
              ldb: i32, taub: &mut [c32], work: &mut [c32], lwork: i32, info: &mut i32) {

    unsafe {
        ffi::cggqrf_(&n, &m, &p, a.as_mut_ptr() as *mut _, &lda, taua.as_mut_ptr() as *mut _,
                     b.as_mut_ptr() as *mut _, &ldb, taub.as_mut_ptr() as *mut _,
                     work.as_mut_ptr() as *mut _, &lwork, info)
    }
}

#[inline]
pub fn zggqrf(n: i32, m: i32, p: i32, a: &mut [c64], lda: i32, taua: &mut [c64], b: &mut [c64],
              ldb: i32, taub: &mut [c64], work: &mut [c64], lwork: i32, info: &mut i32) {

    unsafe {
        ffi::zggqrf_(&n, &m, &p, a.as_mut_ptr() as *mut _, &lda, taua.as_mut_ptr() as *mut _,
                     b.as_mut_ptr() as *mut _, &ldb, taub.as_mut_ptr() as *mut _,
                     work.as_mut_ptr() as *mut _, &lwork, info)
    }
}

#[inline]
pub fn sggrqf(m: i32, p: i32, n: i32, a: &mut [f32], lda: i32, taua: &mut [f32], b: &mut [f32],
              ldb: i32, taub: &mut [f32], work: &mut [f32], lwork: i32, info: &mut i32) {

    unsafe {
        ffi::sggrqf_(&m, &p, &n, a.as_mut_ptr(), &lda, taua.as_mut_ptr(), b.as_mut_ptr(), &ldb,
                     taub.as_mut_ptr(), work.as_mut_ptr(), &lwork, info)
    }
}

#[inline]
pub fn dggrqf(m: i32, p: i32, n: i32, a: &mut [f64], lda: i32, taua: &mut [f64], b: &mut [f64],
              ldb: i32, taub: &mut [f64], work: &mut [f64], lwork: i32, info: &mut i32) {

    unsafe {
        ffi::dggrqf_(&m, &p, &n, a.as_mut_ptr(), &lda, taua.as_mut_ptr(), b.as_mut_ptr(), &ldb,
                     taub.as_mut_ptr(), work.as_mut_ptr(), &lwork, info)
    }
}

#[inline]
pub fn cggrqf(m: i32, p: i32, n: i32, a: &mut [c32], lda: i32, taua: &mut [c32], b: &mut [c32],
              ldb: i32, taub: &mut [c32], work: &mut [c32], lwork: i32, info: &mut i32) {

    unsafe {
        ffi::cggrqf_(&m, &p, &n, a.as_mut_ptr() as *mut _, &lda, taua.as_mut_ptr() as *mut _,
                     b.as_mut_ptr() as *mut _, &ldb, taub.as_mut_ptr() as *mut _,
                     work.as_mut_ptr() as *mut _, &lwork, info)
    }
}

#[inline]
pub fn zggrqf(m: i32, p: i32, n: i32, a: &mut [c64], lda: i32, taua: &mut [c64], b: &mut [c64],
              ldb: i32, taub: &mut [c64], work: &mut [c64], lwork: i32, info: &mut i32) {

    unsafe {
        ffi::zggrqf_(&m, &p, &n, a.as_mut_ptr() as *mut _, &lda, taua.as_mut_ptr() as *mut _,
                     b.as_mut_ptr() as *mut _, &ldb, taub.as_mut_ptr() as *mut _,
                     work.as_mut_ptr() as *mut _, &lwork, info)
    }
}

#[inline]
pub fn sgebrd(m: i32, n: i32, a: &mut [f32], lda: i32, d: &mut [f32], e: &mut [f32],
              tauq: &mut [f32], taup: &mut [f32], work: &mut [f32], lwork: i32, info: &mut i32) {

    unsafe {
        ffi::sgebrd_(&m, &n, a.as_mut_ptr(), &lda, d.as_mut_ptr(), e.as_mut_ptr(),
                     tauq.as_mut_ptr(), taup.as_mut_ptr(), work.as_mut_ptr(), &lwork, info)
    }
}

#[inline]
pub fn dgebrd(m: i32, n: i32, a: &mut [f64], lda: i32, d: &mut [f64], e: &mut [f64],
              tauq: &mut [f64], taup: &mut [f64], work: &mut [f64], lwork: i32, info: &mut i32) {

    unsafe {
        ffi::dgebrd_(&m, &n, a.as_mut_ptr(), &lda, d.as_mut_ptr(), e.as_mut_ptr(),
                     tauq.as_mut_ptr(), taup.as_mut_ptr(), work.as_mut_ptr(), &lwork, info)
    }
}

#[inline]
pub fn cgebrd(m: i32, n: i32, a: &mut [c32], lda: i32, d: &mut [f32], e: &mut [f32],
              tauq: &mut [c32], taup: &mut [c32], work: &mut [c32], lwork: i32, info: &mut i32) {

    unsafe {
        ffi::cgebrd_(&m, &n, a.as_mut_ptr() as *mut _, &lda, d.as_mut_ptr(), e.as_mut_ptr(),
                     tauq.as_mut_ptr() as *mut _, taup.as_mut_ptr() as *mut _,
                     work.as_mut_ptr() as *mut _, &lwork, info)
    }
}

#[inline]
pub fn zgebrd(m: i32, n: i32, a: &mut [c64], lda: i32, d: &mut [f64], e: &mut [f64],
              tauq: &mut [c64], taup: &mut [c64], work: &mut [c64], lwork: i32, info: &mut i32) {

    unsafe {
        ffi::zgebrd_(&m, &n, a.as_mut_ptr() as *mut _, &lda, d.as_mut_ptr(), e.as_mut_ptr(),
                     tauq.as_mut_ptr() as *mut _, taup.as_mut_ptr() as *mut _,
                     work.as_mut_ptr() as *mut _, &lwork, info)
    }
}

#[inline]
pub fn sgbbrd(vect: u8, m: i32, n: i32, ncc: &[i32], kl: i32, ku: i32, ab: &mut [f32], ldab: i32,
              d: &mut [f32], e: &mut [f32], q: &mut f32, ldq: i32, pt: &mut [f32], ldpt: i32,
              c: &mut [f32], ldc: i32, work: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::sgbbrd_(&(vect as c_char), &m, &n, ncc.as_ptr(), &kl, &ku, ab.as_mut_ptr(), &ldab,
                     d.as_mut_ptr(), e.as_mut_ptr(), q, &ldq, pt.as_mut_ptr(), &ldpt,
                     c.as_mut_ptr(), &ldc, work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dgbbrd(vect: u8, m: i32, n: i32, ncc: &[i32], kl: i32, ku: i32, ab: &mut [f64], ldab: i32,
              d: &mut [f64], e: &mut [f64], q: &mut f64, ldq: i32, pt: &mut [f64], ldpt: i32,
              c: &mut [f64], ldc: i32, work: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::dgbbrd_(&(vect as c_char), &m, &n, ncc.as_ptr(), &kl, &ku, ab.as_mut_ptr(), &ldab,
                     d.as_mut_ptr(), e.as_mut_ptr(), q, &ldq, pt.as_mut_ptr(), &ldpt,
                     c.as_mut_ptr(), &ldc, work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn cgbbrd(vect: u8, m: i32, n: i32, ncc: &[i32], kl: i32, ku: i32, ab: &mut [c32], ldab: i32,
              d: &mut [f32], e: &mut [f32], q: &mut c32, ldq: i32, pt: &mut [c32], ldpt: i32,
              c: &mut [c32], ldc: i32, work: &mut [c32], rwork: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::cgbbrd_(&(vect as c_char), &m, &n, ncc.as_ptr(), &kl, &ku, ab.as_mut_ptr() as *mut _,
                     &ldab, d.as_mut_ptr(), e.as_mut_ptr(), q as *mut _ as *mut _, &ldq,
                     pt.as_mut_ptr() as *mut _, &ldpt, c.as_mut_ptr() as *mut _, &ldc,
                     work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zgbbrd(vect: u8, m: i32, n: i32, ncc: &[i32], kl: i32, ku: i32, ab: &mut [c64], ldab: i32,
              d: &mut [f64], e: &mut [f64], q: &mut c64, ldq: i32, pt: &mut [c64], ldpt: i32,
              c: &mut [c64], ldc: i32, work: &mut [c64], rwork: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::zgbbrd_(&(vect as c_char), &m, &n, ncc.as_ptr(), &kl, &ku, ab.as_mut_ptr() as *mut _,
                     &ldab, d.as_mut_ptr(), e.as_mut_ptr(), q as *mut _ as *mut _, &ldq,
                     pt.as_mut_ptr() as *mut _, &ldpt, c.as_mut_ptr() as *mut _, &ldc,
                     work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn sorgbr(vect: u8, m: i32, n: i32, k: i32, a: &mut [f32], lda: i32, tau: &[f32],
              work: &mut [f32], lwork: i32, info: &mut i32) {

    unsafe {
        ffi::sorgbr_(&(vect as c_char), &m, &n, &k, a.as_mut_ptr(), &lda, tau.as_ptr(),
                     work.as_mut_ptr(), &lwork, info)
    }
}

#[inline]
pub fn dorgbr(vect: u8, m: i32, n: i32, k: i32, a: &mut [f64], lda: i32, tau: &[f64],
              work: &mut [f64], lwork: i32, info: &mut i32) {

    unsafe {
        ffi::dorgbr_(&(vect as c_char), &m, &n, &k, a.as_mut_ptr(), &lda, tau.as_ptr(),
                     work.as_mut_ptr(), &lwork, info)
    }
}

#[inline]
pub fn sormbr(vect: u8, side: u8, trans: u8, m: i32, n: i32, k: i32, a: &[f32], lda: i32,
              tau: &[f32], c: &mut [f32], ldc: i32, work: &mut [f32], lwork: i32, info: &mut i32) {

    unsafe {
        ffi::sormbr_(&(vect as c_char), &(side as c_char), &(trans as c_char), &m, &n, &k,
                     a.as_ptr(), &lda, tau.as_ptr(), c.as_mut_ptr(), &ldc, work.as_mut_ptr(),
                     &lwork, info)
    }
}

#[inline]
pub fn dormbr(vect: u8, side: u8, trans: u8, m: i32, n: i32, k: i32, a: &[f64], lda: i32,
              tau: &[f64], c: &mut [f64], ldc: i32, work: &mut [f64], lwork: i32, info: &mut i32) {

    unsafe {
        ffi::dormbr_(&(vect as c_char), &(side as c_char), &(trans as c_char), &m, &n, &k,
                     a.as_ptr(), &lda, tau.as_ptr(), c.as_mut_ptr(), &ldc, work.as_mut_ptr(),
                     &lwork, info)
    }
}

#[inline]
pub fn cungbr(vect: u8, m: i32, n: i32, k: i32, a: &mut [c32], lda: i32, tau: &[c32],
              work: &mut [c32], lwork: i32, info: &mut i32) {

    unsafe {
        ffi::cungbr_(&(vect as c_char), &m, &n, &k, a.as_mut_ptr() as *mut _, &lda,
                     tau.as_ptr() as *const _, work.as_mut_ptr() as *mut _, &lwork, info)
    }
}

#[inline]
pub fn zungbr(vect: u8, m: i32, n: i32, k: i32, a: &mut [c64], lda: i32, tau: &[c64],
              work: &mut [c64], lwork: i32, info: &mut i32) {

    unsafe {
        ffi::zungbr_(&(vect as c_char), &m, &n, &k, a.as_mut_ptr() as *mut _, &lda,
                     tau.as_ptr() as *const _, work.as_mut_ptr() as *mut _, &lwork, info)
    }
}

#[inline]
pub fn cunmbr(vect: u8, side: u8, trans: u8, m: i32, n: i32, k: i32, a: &[c32], lda: i32,
              tau: &[c32], c: &mut [c32], ldc: i32, work: &mut [c32], lwork: i32, info: &mut i32) {

    unsafe {
        ffi::cunmbr_(&(vect as c_char), &(side as c_char), &(trans as c_char), &m, &n, &k,
                     a.as_ptr() as *const _, &lda, tau.as_ptr() as *const _,
                     c.as_mut_ptr() as *mut _, &ldc, work.as_mut_ptr() as *mut _, &lwork, info)
    }
}

#[inline]
pub fn zunmbr(vect: u8, side: u8, trans: u8, m: i32, n: i32, k: i32, a: &[c64], lda: i32,
              tau: &[c64], c: &mut [c64], ldc: i32, work: &mut [c64], lwork: i32, info: &mut i32) {

    unsafe {
        ffi::zunmbr_(&(vect as c_char), &(side as c_char), &(trans as c_char), &m, &n, &k,
                     a.as_ptr() as *const _, &lda, tau.as_ptr() as *const _,
                     c.as_mut_ptr() as *mut _, &ldc, work.as_mut_ptr() as *mut _, &lwork, info)
    }
}

#[inline]
pub fn sbdsqr(uplo: u8, n: i32, ncvt: &[i32], nru: &[i32], ncc: &[i32], d: &mut [f32],
              e: &mut [f32], vt: &mut [f32], ldvt: i32, u: &mut [f32], ldu: i32, c: &mut [f32],
              ldc: i32, work: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::sbdsqr_(&(uplo as c_char), &n, ncvt.as_ptr(), nru.as_ptr(), ncc.as_ptr(),
                     d.as_mut_ptr(), e.as_mut_ptr(), vt.as_mut_ptr(), &ldvt, u.as_mut_ptr(), &ldu,
                     c.as_mut_ptr(), &ldc, work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dbdsqr(uplo: u8, n: i32, ncvt: &[i32], nru: &[i32], ncc: &[i32], d: &mut [f64],
              e: &mut [f64], vt: &mut [f64], ldvt: i32, u: &mut [f64], ldu: i32, c: &mut [f64],
              ldc: i32, work: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::dbdsqr_(&(uplo as c_char), &n, ncvt.as_ptr(), nru.as_ptr(), ncc.as_ptr(),
                     d.as_mut_ptr(), e.as_mut_ptr(), vt.as_mut_ptr(), &ldvt, u.as_mut_ptr(), &ldu,
                     c.as_mut_ptr(), &ldc, work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn cbdsqr(uplo: u8, n: i32, ncvt: &[i32], nru: &[i32], ncc: &[i32], d: &mut [f32],
              e: &mut [f32], vt: &mut [c32], ldvt: i32, u: &mut [c32], ldu: i32, c: &mut [c32],
              ldc: i32, work: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::cbdsqr_(&(uplo as c_char), &n, ncvt.as_ptr(), nru.as_ptr(), ncc.as_ptr(),
                     d.as_mut_ptr(), e.as_mut_ptr(), vt.as_mut_ptr() as *mut _, &ldvt,
                     u.as_mut_ptr() as *mut _, &ldu, c.as_mut_ptr() as *mut _, &ldc,
                     work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zbdsqr(uplo: u8, n: i32, ncvt: &[i32], nru: &[i32], ncc: &[i32], d: &mut [f64],
              e: &mut [f64], vt: &mut [c64], ldvt: i32, u: &mut [c64], ldu: i32, c: &mut [c64],
              ldc: i32, work: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::zbdsqr_(&(uplo as c_char), &n, ncvt.as_ptr(), nru.as_ptr(), ncc.as_ptr(),
                     d.as_mut_ptr(), e.as_mut_ptr(), vt.as_mut_ptr() as *mut _, &ldvt,
                     u.as_mut_ptr() as *mut _, &ldu, c.as_mut_ptr() as *mut _, &ldc,
                     work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn sbdsdc(uplo: u8, compq: u8, n: i32, d: &mut [f32], e: &mut [f32], u: &mut [f32], ldu: i32,
              vt: &mut [f32], ldvt: i32, q: &mut f32, iq: &mut [i32], work: &mut [f32],
              iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::sbdsdc_(&(uplo as c_char), &(compq as c_char), &n, d.as_mut_ptr(), e.as_mut_ptr(),
                     u.as_mut_ptr(), &ldu, vt.as_mut_ptr(), &ldvt, q, iq.as_mut_ptr(),
                     work.as_mut_ptr(), iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dbdsdc(uplo: u8, compq: u8, n: i32, d: &mut [f64], e: &mut [f64], u: &mut [f64], ldu: i32,
              vt: &mut [f64], ldvt: i32, q: &mut f64, iq: &mut [i32], work: &mut [f64],
              iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::dbdsdc_(&(uplo as c_char), &(compq as c_char), &n, d.as_mut_ptr(), e.as_mut_ptr(),
                     u.as_mut_ptr(), &ldu, vt.as_mut_ptr(), &ldvt, q, iq.as_mut_ptr(),
                     work.as_mut_ptr(), iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn sbdsvdx(uplo: u8, jobz: u8, range: u8, n: i32, d: &[f32], e: &[f32], vl: f32, vu: i32,
               il: i32, iu: i32, ns: &mut [i32], s: &mut [f32], z: &mut [f32], ldz: i32,
               work: &mut [f32], iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::sbdsvdx_(&(uplo as c_char), &(jobz as c_char), &(range as c_char), &n, d.as_ptr(),
                      e.as_ptr(), &vl, &vu, &il, &iu, ns.as_mut_ptr(), s.as_mut_ptr(),
                      z.as_mut_ptr(), &ldz, work.as_mut_ptr(), iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dbdsvdx(uplo: u8, jobz: u8, range: u8, n: i32, d: &[f64], e: &[f64], vl: f64, vu: i32,
               il: i32, iu: i32, ns: &mut [i32], s: &mut [f64], z: &mut [f64], ldz: i32,
               work: &mut [f64], iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::dbdsvdx_(&(uplo as c_char), &(jobz as c_char), &(range as c_char), &n, d.as_ptr(),
                      e.as_ptr(), &vl, &vu, &il, &iu, ns.as_mut_ptr(), s.as_mut_ptr(),
                      z.as_mut_ptr(), &ldz, work.as_mut_ptr(), iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn ssytrd(uplo: u8, n: i32, a: &mut [f32], lda: i32, d: &mut [f32], e: &mut [f32],
              tau: &mut [f32], work: &mut [f32], lwork: i32, info: &mut i32) {

    unsafe {
        ffi::ssytrd_(&(uplo as c_char), &n, a.as_mut_ptr(), &lda, d.as_mut_ptr(), e.as_mut_ptr(),
                     tau.as_mut_ptr(), work.as_mut_ptr(), &lwork, info)
    }
}

#[inline]
pub fn dsytrd(uplo: u8, n: i32, a: &mut [f64], lda: i32, d: &mut [f64], e: &mut [f64],
              tau: &mut [f64], work: &mut [f64], lwork: i32, info: &mut i32) {

    unsafe {
        ffi::dsytrd_(&(uplo as c_char), &n, a.as_mut_ptr(), &lda, d.as_mut_ptr(), e.as_mut_ptr(),
                     tau.as_mut_ptr(), work.as_mut_ptr(), &lwork, info)
    }
}

#[inline]
pub fn sorgtr(uplo: u8, n: i32, a: &mut [f32], lda: i32, tau: &[f32], work: &mut [f32], lwork: i32,
              info: &mut i32) {

    unsafe {
        ffi::sorgtr_(&(uplo as c_char), &n, a.as_mut_ptr(), &lda, tau.as_ptr(), work.as_mut_ptr(),
                     &lwork, info)
    }
}

#[inline]
pub fn dorgtr(uplo: u8, n: i32, a: &mut [f64], lda: i32, tau: &[f64], work: &mut [f64], lwork: i32,
              info: &mut i32) {

    unsafe {
        ffi::dorgtr_(&(uplo as c_char), &n, a.as_mut_ptr(), &lda, tau.as_ptr(), work.as_mut_ptr(),
                     &lwork, info)
    }
}

#[inline]
pub fn sormtr(side: u8, uplo: u8, trans: u8, m: i32, n: i32, a: &[f32], lda: i32, tau: &[f32],
              c: &mut [f32], ldc: i32, work: &mut [f32], lwork: i32, info: &mut i32) {

    unsafe {
        ffi::sormtr_(&(side as c_char), &(uplo as c_char), &(trans as c_char), &m, &n, a.as_ptr(),
                     &lda, tau.as_ptr(), c.as_mut_ptr(), &ldc, work.as_mut_ptr(), &lwork, info)
    }
}

#[inline]
pub fn dormtr(side: u8, uplo: u8, trans: u8, m: i32, n: i32, a: &[f64], lda: i32, tau: &[f64],
              c: &mut [f64], ldc: i32, work: &mut [f64], lwork: i32, info: &mut i32) {

    unsafe {
        ffi::dormtr_(&(side as c_char), &(uplo as c_char), &(trans as c_char), &m, &n, a.as_ptr(),
                     &lda, tau.as_ptr(), c.as_mut_ptr(), &ldc, work.as_mut_ptr(), &lwork, info)
    }
}

#[inline]
pub fn chetrd(uplo: u8, n: i32, a: &mut [c32], lda: i32, d: &mut [f32], e: &mut [f32],
              tau: &mut [c32], work: &mut [c32], lwork: i32, info: &mut i32) {

    unsafe {
        ffi::chetrd_(&(uplo as c_char), &n, a.as_mut_ptr() as *mut _, &lda, d.as_mut_ptr(),
                     e.as_mut_ptr(), tau.as_mut_ptr() as *mut _, work.as_mut_ptr() as *mut _,
                     &lwork, info)
    }
}

#[inline]
pub fn zhetrd(uplo: u8, n: i32, a: &mut [c64], lda: i32, d: &mut [f64], e: &mut [f64],
              tau: &mut [c64], work: &mut [c64], lwork: i32, info: &mut i32) {

    unsafe {
        ffi::zhetrd_(&(uplo as c_char), &n, a.as_mut_ptr() as *mut _, &lda, d.as_mut_ptr(),
                     e.as_mut_ptr(), tau.as_mut_ptr() as *mut _, work.as_mut_ptr() as *mut _,
                     &lwork, info)
    }
}

#[inline]
pub fn cungtr(uplo: u8, n: i32, a: &mut [c32], lda: i32, tau: &[c32], work: &mut [c32], lwork: i32,
              info: &mut i32) {

    unsafe {
        ffi::cungtr_(&(uplo as c_char), &n, a.as_mut_ptr() as *mut _, &lda,
                     tau.as_ptr() as *const _, work.as_mut_ptr() as *mut _, &lwork, info)
    }
}

#[inline]
pub fn zungtr(uplo: u8, n: i32, a: &mut [c64], lda: i32, tau: &[c64], work: &mut [c64], lwork: i32,
              info: &mut i32) {

    unsafe {
        ffi::zungtr_(&(uplo as c_char), &n, a.as_mut_ptr() as *mut _, &lda,
                     tau.as_ptr() as *const _, work.as_mut_ptr() as *mut _, &lwork, info)
    }
}

#[inline]
pub fn cunmtr(side: u8, uplo: u8, trans: u8, m: i32, n: i32, a: &[c32], lda: i32, tau: &[c32],
              c: &mut [c32], ldc: i32, work: &mut [c32], lwork: i32, info: &mut i32) {

    unsafe {
        ffi::cunmtr_(&(side as c_char), &(uplo as c_char), &(trans as c_char), &m, &n,
                     a.as_ptr() as *const _, &lda, tau.as_ptr() as *const _,
                     c.as_mut_ptr() as *mut _, &ldc, work.as_mut_ptr() as *mut _, &lwork, info)
    }
}

#[inline]
pub fn zunmtr(side: u8, uplo: u8, trans: u8, m: i32, n: i32, a: &[c64], lda: i32, tau: &[c64],
              c: &mut [c64], ldc: i32, work: &mut [c64], lwork: i32, info: &mut i32) {

    unsafe {
        ffi::zunmtr_(&(side as c_char), &(uplo as c_char), &(trans as c_char), &m, &n,
                     a.as_ptr() as *const _, &lda, tau.as_ptr() as *const _,
                     c.as_mut_ptr() as *mut _, &ldc, work.as_mut_ptr() as *mut _, &lwork, info)
    }
}

#[inline]
pub fn ssptrd(uplo: u8, n: i32, ap: &mut [f32], d: &mut [f32], e: &mut [f32], tau: &mut [f32],
              info: &mut i32) {

    unsafe {
        ffi::ssptrd_(&(uplo as c_char), &n, ap.as_mut_ptr(), d.as_mut_ptr(), e.as_mut_ptr(),
                     tau.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dsptrd(uplo: u8, n: i32, ap: &mut [f64], d: &mut [f64], e: &mut [f64], tau: &mut [f64],
              info: &mut i32) {

    unsafe {
        ffi::dsptrd_(&(uplo as c_char), &n, ap.as_mut_ptr(), d.as_mut_ptr(), e.as_mut_ptr(),
                     tau.as_mut_ptr(), info)
    }
}

#[inline]
pub fn sopgtr(uplo: u8, n: i32, ap: &[f32], tau: &[f32], q: &mut f32, ldq: i32, work: &mut [f32],
              info: &mut i32) {

    unsafe {
        ffi::sopgtr_(&(uplo as c_char), &n, ap.as_ptr(), tau.as_ptr(), q, &ldq, work.as_mut_ptr(),
                     info)
    }
}

#[inline]
pub fn dopgtr(uplo: u8, n: i32, ap: &[f64], tau: &[f64], q: &mut f64, ldq: i32, work: &mut [f64],
              info: &mut i32) {

    unsafe {
        ffi::dopgtr_(&(uplo as c_char), &n, ap.as_ptr(), tau.as_ptr(), q, &ldq, work.as_mut_ptr(),
                     info)
    }
}

#[inline]
pub fn sopmtr(side: u8, uplo: u8, trans: u8, m: i32, n: i32, ap: &[f32], tau: &[f32],
              c: &mut [f32], ldc: i32, work: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::sopmtr_(&(side as c_char), &(uplo as c_char), &(trans as c_char), &m, &n, ap.as_ptr(),
                     tau.as_ptr(), c.as_mut_ptr(), &ldc, work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dopmtr(side: u8, uplo: u8, trans: u8, m: i32, n: i32, ap: &[f64], tau: &[f64],
              c: &mut [f64], ldc: i32, work: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::dopmtr_(&(side as c_char), &(uplo as c_char), &(trans as c_char), &m, &n, ap.as_ptr(),
                     tau.as_ptr(), c.as_mut_ptr(), &ldc, work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn chptrd(uplo: u8, n: i32, ap: &mut [c32], d: &mut [f32], e: &mut [f32], tau: &mut [c32],
              info: &mut i32) {

    unsafe {
        ffi::chptrd_(&(uplo as c_char), &n, ap.as_mut_ptr() as *mut _, d.as_mut_ptr(),
                     e.as_mut_ptr(), tau.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn zhptrd(uplo: u8, n: i32, ap: &mut [c64], d: &mut [f64], e: &mut [f64], tau: &mut [c64],
              info: &mut i32) {

    unsafe {
        ffi::zhptrd_(&(uplo as c_char), &n, ap.as_mut_ptr() as *mut _, d.as_mut_ptr(),
                     e.as_mut_ptr(), tau.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn cupgtr(uplo: u8, n: i32, ap: &[c32], tau: &[c32], q: &mut c32, ldq: i32, work: &mut [c32],
              info: &mut i32) {

    unsafe {
        ffi::cupgtr_(&(uplo as c_char), &n, ap.as_ptr() as *const _, tau.as_ptr() as *const _,
                     q as *mut _ as *mut _, &ldq, work.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn zupgtr(uplo: u8, n: i32, ap: &[c64], tau: &[c64], q: &mut c64, ldq: i32, work: &mut [c64],
              info: &mut i32) {

    unsafe {
        ffi::zupgtr_(&(uplo as c_char), &n, ap.as_ptr() as *const _, tau.as_ptr() as *const _,
                     q as *mut _ as *mut _, &ldq, work.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn cupmtr(side: u8, uplo: u8, trans: u8, m: i32, n: i32, ap: &[c32], tau: &[c32],
              c: &mut [c32], ldc: i32, work: &mut [c32], info: &mut i32) {

    unsafe {
        ffi::cupmtr_(&(side as c_char), &(uplo as c_char), &(trans as c_char), &m, &n,
                     ap.as_ptr() as *const _, tau.as_ptr() as *const _, c.as_mut_ptr() as *mut _,
                     &ldc, work.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn zupmtr(side: u8, uplo: u8, trans: u8, m: i32, n: i32, ap: &[c64], tau: &[c64],
              c: &mut [c64], ldc: i32, work: &mut [c64], info: &mut i32) {

    unsafe {
        ffi::zupmtr_(&(side as c_char), &(uplo as c_char), &(trans as c_char), &m, &n,
                     ap.as_ptr() as *const _, tau.as_ptr() as *const _, c.as_mut_ptr() as *mut _,
                     &ldc, work.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn ssbtrd(vect: u8, uplo: u8, n: i32, kd: i32, ab: &mut [f32], ldab: i32, d: &mut [f32],
              e: &mut [f32], q: &mut f32, ldq: i32, work: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::ssbtrd_(&(vect as c_char), &(uplo as c_char), &n, &kd, ab.as_mut_ptr(), &ldab,
                     d.as_mut_ptr(), e.as_mut_ptr(), q, &ldq, work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dsbtrd(vect: u8, uplo: u8, n: i32, kd: i32, ab: &mut [f64], ldab: i32, d: &mut [f64],
              e: &mut [f64], q: &mut f64, ldq: i32, work: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::dsbtrd_(&(vect as c_char), &(uplo as c_char), &n, &kd, ab.as_mut_ptr(), &ldab,
                     d.as_mut_ptr(), e.as_mut_ptr(), q, &ldq, work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn chbtrd(vect: u8, uplo: u8, n: i32, kd: i32, ab: &mut [c32], ldab: i32, d: &mut [f32],
              e: &mut [f32], q: &mut c32, ldq: i32, work: &mut [c32], info: &mut i32) {

    unsafe {
        ffi::chbtrd_(&(vect as c_char), &(uplo as c_char), &n, &kd, ab.as_mut_ptr() as *mut _,
                     &ldab, d.as_mut_ptr(), e.as_mut_ptr(), q as *mut _ as *mut _, &ldq,
                     work.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn zhbtrd(vect: u8, uplo: u8, n: i32, kd: i32, ab: &mut [c64], ldab: i32, d: &mut [f64],
              e: &mut [f64], q: &mut c64, ldq: i32, work: &mut [c64], info: &mut i32) {

    unsafe {
        ffi::zhbtrd_(&(vect as c_char), &(uplo as c_char), &n, &kd, ab.as_mut_ptr() as *mut _,
                     &ldab, d.as_mut_ptr(), e.as_mut_ptr(), q as *mut _ as *mut _, &ldq,
                     work.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn ssterf(n: i32, d: &mut [f32], e: &mut [f32], info: &mut i32) {
    unsafe {
        ffi::ssterf_(&n, d.as_mut_ptr(), e.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dsterf(n: i32, d: &mut [f64], e: &mut [f64], info: &mut i32) {
    unsafe {
        ffi::dsterf_(&n, d.as_mut_ptr(), e.as_mut_ptr(), info)
    }
}

#[inline]
pub fn ssteqr(compz: u8, n: i32, d: &mut [f32], e: &mut [f32], z: &mut [f32], ldz: i32,
              work: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::ssteqr_(&(compz as c_char), &n, d.as_mut_ptr(), e.as_mut_ptr(), z.as_mut_ptr(), &ldz,
                     work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dsteqr(compz: u8, n: i32, d: &mut [f64], e: &mut [f64], z: &mut [f64], ldz: i32,
              work: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::dsteqr_(&(compz as c_char), &n, d.as_mut_ptr(), e.as_mut_ptr(), z.as_mut_ptr(), &ldz,
                     work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn csteqr(compz: u8, n: i32, d: &mut [f32], e: &mut [f32], z: &mut [c32], ldz: i32,
              work: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::csteqr_(&(compz as c_char), &n, d.as_mut_ptr(), e.as_mut_ptr(),
                     z.as_mut_ptr() as *mut _, &ldz, work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zsteqr(compz: u8, n: i32, d: &mut [f64], e: &mut [f64], z: &mut [c64], ldz: i32,
              work: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::zsteqr_(&(compz as c_char), &n, d.as_mut_ptr(), e.as_mut_ptr(),
                     z.as_mut_ptr() as *mut _, &ldz, work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn sstemr(jobz: u8, range: u8, n: i32, d: &mut [f32], e: &mut [f32], vl: f32, vu: f32, il: i32,
              iu: i32, m: &mut i32, w: &mut [f32], z: &mut [f32], ldz: i32, nzc: &[i32],
              isuppz: &mut [i32], tryrac: &mut i32, work: &mut [f32], lwork: i32,
              iwork: &mut [i32], liwork: i32, info: &mut i32) {

    unsafe {
        ffi::sstemr_(&(jobz as c_char), &(range as c_char), &n, d.as_mut_ptr(), e.as_mut_ptr(),
                     &vl, &vu, &il, &iu, m, w.as_mut_ptr(), z.as_mut_ptr(), &ldz, nzc.as_ptr(),
                     isuppz.as_mut_ptr(), tryrac, work.as_mut_ptr(), &lwork, iwork.as_mut_ptr(),
                     &liwork, info)
    }
}

#[inline]
pub fn dstemr(jobz: u8, range: u8, n: i32, d: &mut [f64], e: &mut [f64], vl: f64, vu: f64, il: i32,
              iu: i32, m: &mut i32, w: &mut [f64], z: &mut [f64], ldz: i32, nzc: &[i32],
              isuppz: &mut [i32], tryrac: &mut i32, work: &mut [f64], lwork: i32,
              iwork: &mut [i32], liwork: i32, info: &mut i32) {

    unsafe {
        ffi::dstemr_(&(jobz as c_char), &(range as c_char), &n, d.as_mut_ptr(), e.as_mut_ptr(),
                     &vl, &vu, &il, &iu, m, w.as_mut_ptr(), z.as_mut_ptr(), &ldz, nzc.as_ptr(),
                     isuppz.as_mut_ptr(), tryrac, work.as_mut_ptr(), &lwork, iwork.as_mut_ptr(),
                     &liwork, info)
    }
}

#[inline]
pub fn cstemr(jobz: u8, range: u8, n: i32, d: &mut [f32], e: &mut [f32], vl: f32, vu: f32, il: i32,
              iu: i32, m: &mut i32, w: &mut [f32], z: &mut [c32], ldz: i32, nzc: &[i32],
              isuppz: &mut [i32], tryrac: &mut i32, work: &mut [f32], lwork: i32,
              iwork: &mut [i32], liwork: i32, info: &mut i32) {

    unsafe {
        ffi::cstemr_(&(jobz as c_char), &(range as c_char), &n, d.as_mut_ptr(), e.as_mut_ptr(),
                     &vl, &vu, &il, &iu, m, w.as_mut_ptr(), z.as_mut_ptr() as *mut _, &ldz,
                     nzc.as_ptr(), isuppz.as_mut_ptr(), tryrac, work.as_mut_ptr(), &lwork,
                     iwork.as_mut_ptr(), &liwork, info)
    }
}

#[inline]
pub fn zstemr(jobz: u8, range: u8, n: i32, d: &mut [f64], e: &mut [f64], vl: f64, vu: f64, il: i32,
              iu: i32, m: &mut i32, w: &mut [f64], z: &mut [c64], ldz: i32, nzc: &[i32],
              isuppz: &mut [i32], tryrac: &mut i32, work: &mut [f64], lwork: i32,
              iwork: &mut [i32], liwork: i32, info: &mut i32) {

    unsafe {
        ffi::zstemr_(&(jobz as c_char), &(range as c_char), &n, d.as_mut_ptr(), e.as_mut_ptr(),
                     &vl, &vu, &il, &iu, m, w.as_mut_ptr(), z.as_mut_ptr() as *mut _, &ldz,
                     nzc.as_ptr(), isuppz.as_mut_ptr(), tryrac, work.as_mut_ptr(), &lwork,
                     iwork.as_mut_ptr(), &liwork, info)
    }
}

#[inline]
pub fn sstedc(compz: u8, n: i32, d: &mut [f32], e: &mut [f32], z: &mut [f32], ldz: i32,
              work: &mut [f32], lwork: i32, iwork: &mut [i32], liwork: i32, info: &mut i32) {

    unsafe {
        ffi::sstedc_(&(compz as c_char), &n, d.as_mut_ptr(), e.as_mut_ptr(), z.as_mut_ptr(), &ldz,
                     work.as_mut_ptr(), &lwork, iwork.as_mut_ptr(), &liwork, info)
    }
}

#[inline]
pub fn dstedc(compz: u8, n: i32, d: &mut [f64], e: &mut [f64], z: &mut [f64], ldz: i32,
              work: &mut [f64], lwork: i32, iwork: &mut [i32], liwork: i32, info: &mut i32) {

    unsafe {
        ffi::dstedc_(&(compz as c_char), &n, d.as_mut_ptr(), e.as_mut_ptr(), z.as_mut_ptr(), &ldz,
                     work.as_mut_ptr(), &lwork, iwork.as_mut_ptr(), &liwork, info)
    }
}

#[inline]
pub fn cstedc(compz: u8, n: i32, d: &mut [f32], e: &mut [f32], z: &mut [c32], ldz: i32,
              work: &mut [c32], lwork: i32, rwork: &mut [f32], lrwork: i32, iwork: &mut [i32],
              liwork: i32, info: &mut i32) {

    unsafe {
        ffi::cstedc_(&(compz as c_char), &n, d.as_mut_ptr(), e.as_mut_ptr(),
                     z.as_mut_ptr() as *mut _, &ldz, work.as_mut_ptr() as *mut _, &lwork,
                     rwork.as_mut_ptr(), &lrwork, iwork.as_mut_ptr(), &liwork, info)
    }
}

#[inline]
pub fn zstedc(compz: u8, n: i32, d: &mut [f64], e: &mut [f64], z: &mut [c64], ldz: i32,
              work: &mut [c64], lwork: i32, rwork: &mut [f64], lrwork: i32, iwork: &mut [i32],
              liwork: i32, info: &mut i32) {

    unsafe {
        ffi::zstedc_(&(compz as c_char), &n, d.as_mut_ptr(), e.as_mut_ptr(),
                     z.as_mut_ptr() as *mut _, &ldz, work.as_mut_ptr() as *mut _, &lwork,
                     rwork.as_mut_ptr(), &lrwork, iwork.as_mut_ptr(), &liwork, info)
    }
}

#[inline]
pub fn sstegr(jobz: u8, range: u8, n: i32, d: &mut [f32], e: &mut [f32], vl: f32, vu: f32, il: i32,
              iu: i32, abstol: f32, m: &mut i32, w: &mut [f32], z: &mut [f32], ldz: i32,
              isuppz: &mut [i32], work: &mut [f32], lwork: i32, iwork: &mut [i32], liwork: i32,
              info: &mut i32) {

    unsafe {
        ffi::sstegr_(&(jobz as c_char), &(range as c_char), &n, d.as_mut_ptr(), e.as_mut_ptr(),
                     &vl, &vu, &il, &iu, &abstol, m, w.as_mut_ptr(), z.as_mut_ptr(), &ldz,
                     isuppz.as_mut_ptr(), work.as_mut_ptr(), &lwork, iwork.as_mut_ptr(), &liwork,
                     info)
    }
}

#[inline]
pub fn dstegr(jobz: u8, range: u8, n: i32, d: &mut [f64], e: &mut [f64], vl: f64, vu: f64, il: i32,
              iu: i32, abstol: f64, m: &mut i32, w: &mut [f64], z: &mut [f64], ldz: i32,
              isuppz: &mut [i32], work: &mut [f64], lwork: i32, iwork: &mut [i32], liwork: i32,
              info: &mut i32) {

    unsafe {
        ffi::dstegr_(&(jobz as c_char), &(range as c_char), &n, d.as_mut_ptr(), e.as_mut_ptr(),
                     &vl, &vu, &il, &iu, &abstol, m, w.as_mut_ptr(), z.as_mut_ptr(), &ldz,
                     isuppz.as_mut_ptr(), work.as_mut_ptr(), &lwork, iwork.as_mut_ptr(), &liwork,
                     info)
    }
}

#[inline]
pub fn cstegr(jobz: u8, range: u8, n: i32, d: &mut [f32], e: &mut [f32], vl: f32, vu: f32, il: i32,
              iu: i32, abstol: f32, m: &mut i32, w: &mut [f32], z: &mut [c32], ldz: i32,
              isuppz: &mut [i32], work: &mut [f32], lwork: i32, iwork: &mut [i32], liwork: i32,
              info: &mut i32) {

    unsafe {
        ffi::cstegr_(&(jobz as c_char), &(range as c_char), &n, d.as_mut_ptr(), e.as_mut_ptr(),
                     &vl, &vu, &il, &iu, &abstol, m, w.as_mut_ptr(), z.as_mut_ptr() as *mut _,
                     &ldz, isuppz.as_mut_ptr(), work.as_mut_ptr(), &lwork, iwork.as_mut_ptr(),
                     &liwork, info)
    }
}

#[inline]
pub fn zstegr(jobz: u8, range: u8, n: i32, d: &mut [f64], e: &mut [f64], vl: f64, vu: f64, il: i32,
              iu: i32, abstol: f64, m: &mut i32, w: &mut [f64], z: &mut [c64], ldz: i32,
              isuppz: &mut [i32], work: &mut [f64], lwork: i32, iwork: &mut [i32], liwork: i32,
              info: &mut i32) {

    unsafe {
        ffi::zstegr_(&(jobz as c_char), &(range as c_char), &n, d.as_mut_ptr(), e.as_mut_ptr(),
                     &vl, &vu, &il, &iu, &abstol, m, w.as_mut_ptr(), z.as_mut_ptr() as *mut _,
                     &ldz, isuppz.as_mut_ptr(), work.as_mut_ptr(), &lwork, iwork.as_mut_ptr(),
                     &liwork, info)
    }
}

#[inline]
pub fn spteqr(compz: u8, n: i32, d: &mut [f32], e: &mut [f32], z: &mut [f32], ldz: i32,
              work: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::spteqr_(&(compz as c_char), &n, d.as_mut_ptr(), e.as_mut_ptr(), z.as_mut_ptr(), &ldz,
                     work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dpteqr(compz: u8, n: i32, d: &mut [f64], e: &mut [f64], z: &mut [f64], ldz: i32,
              work: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::dpteqr_(&(compz as c_char), &n, d.as_mut_ptr(), e.as_mut_ptr(), z.as_mut_ptr(), &ldz,
                     work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn cpteqr(compz: u8, n: i32, d: &mut [f32], e: &mut [f32], z: &mut [c32], ldz: i32,
              work: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::cpteqr_(&(compz as c_char), &n, d.as_mut_ptr(), e.as_mut_ptr(),
                     z.as_mut_ptr() as *mut _, &ldz, work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zpteqr(compz: u8, n: i32, d: &mut [f64], e: &mut [f64], z: &mut [c64], ldz: i32,
              work: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::zpteqr_(&(compz as c_char), &n, d.as_mut_ptr(), e.as_mut_ptr(),
                     z.as_mut_ptr() as *mut _, &ldz, work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn sstebz(range: u8, order: u8, n: i32, vl: f32, vu: f32, il: i32, iu: i32, abstol: f32,
              d: &[f32], e: &[f32], m: &mut i32, nsplit: &mut [i32], w: &mut [f32],
              iblock: &mut [i32], isplit: &mut [i32], work: &mut [f32], iwork: &mut [i32],
              info: &mut i32) {

    unsafe {
        ffi::sstebz_(&(range as c_char), &(order as c_char), &n, &vl, &vu, &il, &iu, &abstol,
                     d.as_ptr(), e.as_ptr(), m, nsplit.as_mut_ptr(), w.as_mut_ptr(),
                     iblock.as_mut_ptr(), isplit.as_mut_ptr(), work.as_mut_ptr(),
                     iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dstebz(range: u8, order: u8, n: i32, vl: f64, vu: f64, il: i32, iu: i32, abstol: f64,
              d: &[f64], e: &[f64], m: &mut i32, nsplit: &mut [i32], w: &mut [f64],
              iblock: &mut [i32], isplit: &mut [i32], work: &mut [f64], iwork: &mut [i32],
              info: &mut i32) {

    unsafe {
        ffi::dstebz_(&(range as c_char), &(order as c_char), &n, &vl, &vu, &il, &iu, &abstol,
                     d.as_ptr(), e.as_ptr(), m, nsplit.as_mut_ptr(), w.as_mut_ptr(),
                     iblock.as_mut_ptr(), isplit.as_mut_ptr(), work.as_mut_ptr(),
                     iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn sstein(n: i32, d: &[f32], e: &[f32], m: i32, w: &[f32], iblock: &[i32], isplit: &[i32],
              z: &mut [f32], ldz: i32, work: &mut [f32], iwork: &mut [i32], ifail: &mut i32,
              info: &mut i32) {

    unsafe {
        ffi::sstein_(&n, d.as_ptr(), e.as_ptr(), &m, w.as_ptr(), iblock.as_ptr(), isplit.as_ptr(),
                     z.as_mut_ptr(), &ldz, work.as_mut_ptr(), iwork.as_mut_ptr(), ifail, info)
    }
}

#[inline]
pub fn dstein(n: i32, d: &[f64], e: &[f64], m: i32, w: &[f64], iblock: &[i32], isplit: &[i32],
              z: &mut [f64], ldz: i32, work: &mut [f64], iwork: &mut [i32], ifail: &mut i32,
              info: &mut i32) {

    unsafe {
        ffi::dstein_(&n, d.as_ptr(), e.as_ptr(), &m, w.as_ptr(), iblock.as_ptr(), isplit.as_ptr(),
                     z.as_mut_ptr(), &ldz, work.as_mut_ptr(), iwork.as_mut_ptr(), ifail, info)
    }
}

#[inline]
pub fn cstein(n: i32, d: &[f32], e: &[f32], m: i32, w: &[f32], iblock: &[i32], isplit: &[i32],
              z: &mut [c32], ldz: i32, work: &mut [f32], iwork: &mut [i32], ifail: &mut i32,
              info: &mut i32) {

    unsafe {
        ffi::cstein_(&n, d.as_ptr(), e.as_ptr(), &m, w.as_ptr(), iblock.as_ptr(), isplit.as_ptr(),
                     z.as_mut_ptr() as *mut _, &ldz, work.as_mut_ptr(), iwork.as_mut_ptr(), ifail,
                     info)
    }
}

#[inline]
pub fn zstein(n: i32, d: &[f64], e: &[f64], m: i32, w: &[f64], iblock: &[i32], isplit: &[i32],
              z: &mut [c64], ldz: i32, work: &mut [f64], iwork: &mut [i32], ifail: &mut i32,
              info: &mut i32) {

    unsafe {
        ffi::zstein_(&n, d.as_ptr(), e.as_ptr(), &m, w.as_ptr(), iblock.as_ptr(), isplit.as_ptr(),
                     z.as_mut_ptr() as *mut _, &ldz, work.as_mut_ptr(), iwork.as_mut_ptr(), ifail,
                     info)
    }
}

#[inline]
pub fn sdisna(job: u8, m: i32, n: i32, d: &[f32], sep: &mut [f32], info: &mut i32) {
    unsafe {
        ffi::sdisna_(&(job as c_char), &m, &n, d.as_ptr(), sep.as_mut_ptr(), info)
    }
}

#[inline]
pub fn ddisna(job: u8, m: i32, n: i32, d: &[f64], sep: &mut [f64], info: &mut i32) {
    unsafe {
        ffi::ddisna_(&(job as c_char), &m, &n, d.as_ptr(), sep.as_mut_ptr(), info)
    }
}

#[inline]
pub fn ssygst(itype: &[i32], uplo: u8, n: i32, a: &mut [f32], lda: i32, b: &[f32], ldb: i32,
              info: &mut i32) {

    unsafe {
        ffi::ssygst_(itype.as_ptr(), &(uplo as c_char), &n, a.as_mut_ptr(), &lda, b.as_ptr(), &ldb,
                     info)
    }
}

#[inline]
pub fn dsygst(itype: &[i32], uplo: u8, n: i32, a: &mut [f64], lda: i32, b: &[f64], ldb: i32,
              info: &mut i32) {

    unsafe {
        ffi::dsygst_(itype.as_ptr(), &(uplo as c_char), &n, a.as_mut_ptr(), &lda, b.as_ptr(), &ldb,
                     info)
    }
}

#[inline]
pub fn chegst(itype: &[i32], uplo: u8, n: i32, a: &mut [c32], lda: i32, b: &[c32], ldb: i32,
              info: &mut i32) {

    unsafe {
        ffi::chegst_(itype.as_ptr(), &(uplo as c_char), &n, a.as_mut_ptr() as *mut _, &lda,
                     b.as_ptr() as *const _, &ldb, info)
    }
}

#[inline]
pub fn zhegst(itype: &[i32], uplo: u8, n: i32, a: &mut [c64], lda: i32, b: &[c64], ldb: i32,
              info: &mut i32) {

    unsafe {
        ffi::zhegst_(itype.as_ptr(), &(uplo as c_char), &n, a.as_mut_ptr() as *mut _, &lda,
                     b.as_ptr() as *const _, &ldb, info)
    }
}

#[inline]
pub fn sspgst(itype: &[i32], uplo: u8, n: i32, ap: &mut [f32], bp: &[f32], info: &mut i32) {
    unsafe {
        ffi::sspgst_(itype.as_ptr(), &(uplo as c_char), &n, ap.as_mut_ptr(), bp.as_ptr(), info)
    }
}

#[inline]
pub fn dspgst(itype: &[i32], uplo: u8, n: i32, ap: &mut [f64], bp: &[f64], info: &mut i32) {
    unsafe {
        ffi::dspgst_(itype.as_ptr(), &(uplo as c_char), &n, ap.as_mut_ptr(), bp.as_ptr(), info)
    }
}

#[inline]
pub fn chpgst(itype: &[i32], uplo: u8, n: i32, ap: &mut [c32], bp: &[c32], info: &mut i32) {
    unsafe {
        ffi::chpgst_(itype.as_ptr(), &(uplo as c_char), &n, ap.as_mut_ptr() as *mut _,
                     bp.as_ptr() as *const _, info)
    }
}

#[inline]
pub fn zhpgst(itype: &[i32], uplo: u8, n: i32, ap: &mut [c64], bp: &[c64], info: &mut i32) {
    unsafe {
        ffi::zhpgst_(itype.as_ptr(), &(uplo as c_char), &n, ap.as_mut_ptr() as *mut _,
                     bp.as_ptr() as *const _, info)
    }
}

#[inline]
pub fn ssbgst(vect: u8, uplo: u8, n: i32, ka: i32, kb: i32, ab: &mut [f32], ldab: i32, bb: &[f32],
              ldbb: i32, x: &mut [f32], ldx: i32, work: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::ssbgst_(&(vect as c_char), &(uplo as c_char), &n, &ka, &kb, ab.as_mut_ptr(), &ldab,
                     bb.as_ptr(), &ldbb, x.as_mut_ptr(), &ldx, work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dsbgst(vect: u8, uplo: u8, n: i32, ka: i32, kb: i32, ab: &mut [f64], ldab: i32, bb: &[f64],
              ldbb: i32, x: &mut [f64], ldx: i32, work: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::dsbgst_(&(vect as c_char), &(uplo as c_char), &n, &ka, &kb, ab.as_mut_ptr(), &ldab,
                     bb.as_ptr(), &ldbb, x.as_mut_ptr(), &ldx, work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn chbgst(vect: u8, uplo: u8, n: i32, ka: i32, kb: i32, ab: &mut [c32], ldab: i32, bb: &[c32],
              ldbb: i32, x: &mut [c32], ldx: i32, work: &mut [c32], rwork: &mut [f32],
              info: &mut i32) {

    unsafe {
        ffi::chbgst_(&(vect as c_char), &(uplo as c_char), &n, &ka, &kb, ab.as_mut_ptr() as *mut _,
                     &ldab, bb.as_ptr() as *const _, &ldbb, x.as_mut_ptr() as *mut _, &ldx,
                     work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zhbgst(vect: u8, uplo: u8, n: i32, ka: i32, kb: i32, ab: &mut [c64], ldab: i32, bb: &[c64],
              ldbb: i32, x: &mut [c64], ldx: i32, work: &mut [c64], rwork: &mut [f64],
              info: &mut i32) {

    unsafe {
        ffi::zhbgst_(&(vect as c_char), &(uplo as c_char), &n, &ka, &kb, ab.as_mut_ptr() as *mut _,
                     &ldab, bb.as_ptr() as *const _, &ldbb, x.as_mut_ptr() as *mut _, &ldx,
                     work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn spbstf(uplo: u8, n: i32, kd: i32, ab: &mut [f32], ldab: i32, info: &mut i32) {
    unsafe {
        ffi::spbstf_(&(uplo as c_char), &n, &kd, ab.as_mut_ptr(), &ldab, info)
    }
}

#[inline]
pub fn dpbstf(uplo: u8, n: i32, kd: i32, ab: &mut [f64], ldab: i32, info: &mut i32) {
    unsafe {
        ffi::dpbstf_(&(uplo as c_char), &n, &kd, ab.as_mut_ptr(), &ldab, info)
    }
}

#[inline]
pub fn cpbstf(uplo: u8, n: i32, kd: i32, ab: &mut [c32], ldab: i32, info: &mut i32) {
    unsafe {
        ffi::cpbstf_(&(uplo as c_char), &n, &kd, ab.as_mut_ptr() as *mut _, &ldab, info)
    }
}

#[inline]
pub fn zpbstf(uplo: u8, n: i32, kd: i32, ab: &mut [c64], ldab: i32, info: &mut i32) {
    unsafe {
        ffi::zpbstf_(&(uplo as c_char), &n, &kd, ab.as_mut_ptr() as *mut _, &ldab, info)
    }
}

#[inline]
pub fn sgehrd(n: i32, ilo: i32, ihi: i32, a: &mut [f32], lda: i32, tau: &mut [f32],
              work: &mut [f32], lwork: i32, info: &mut i32) {

    unsafe {
        ffi::sgehrd_(&n, &ilo, &ihi, a.as_mut_ptr(), &lda, tau.as_mut_ptr(), work.as_mut_ptr(),
                     &lwork, info)
    }
}

#[inline]
pub fn dgehrd(n: i32, ilo: i32, ihi: i32, a: &mut [f64], lda: i32, tau: &mut [f64],
              work: &mut [f64], lwork: i32, info: &mut i32) {

    unsafe {
        ffi::dgehrd_(&n, &ilo, &ihi, a.as_mut_ptr(), &lda, tau.as_mut_ptr(), work.as_mut_ptr(),
                     &lwork, info)
    }
}

#[inline]
pub fn cgehrd(n: i32, ilo: i32, ihi: i32, a: &mut [c32], lda: i32, tau: &mut [c32],
              work: &mut [c32], lwork: i32, info: &mut i32) {

    unsafe {
        ffi::cgehrd_(&n, &ilo, &ihi, a.as_mut_ptr() as *mut _, &lda, tau.as_mut_ptr() as *mut _,
                     work.as_mut_ptr() as *mut _, &lwork, info)
    }
}

#[inline]
pub fn zgehrd(n: i32, ilo: i32, ihi: i32, a: &mut [c64], lda: i32, tau: &mut [c64],
              work: &mut [c64], lwork: i32, info: &mut i32) {

    unsafe {
        ffi::zgehrd_(&n, &ilo, &ihi, a.as_mut_ptr() as *mut _, &lda, tau.as_mut_ptr() as *mut _,
                     work.as_mut_ptr() as *mut _, &lwork, info)
    }
}

#[inline]
pub fn sorghr(n: i32, ilo: i32, ihi: i32, a: &mut [f32], lda: i32, tau: &[f32], work: &mut [f32],
              lwork: i32, info: &mut i32) {

    unsafe {
        ffi::sorghr_(&n, &ilo, &ihi, a.as_mut_ptr(), &lda, tau.as_ptr(), work.as_mut_ptr(), &lwork,
                     info)
    }
}

#[inline]
pub fn dorghr(n: i32, ilo: i32, ihi: i32, a: &mut [f64], lda: i32, tau: &[f64], work: &mut [f64],
              lwork: i32, info: &mut i32) {

    unsafe {
        ffi::dorghr_(&n, &ilo, &ihi, a.as_mut_ptr(), &lda, tau.as_ptr(), work.as_mut_ptr(), &lwork,
                     info)
    }
}

#[inline]
pub fn sormhr(side: u8, trans: u8, m: i32, n: i32, ilo: i32, ihi: i32, a: &[f32], lda: i32,
              tau: &[f32], c: &mut [f32], ldc: i32, work: &mut [f32], lwork: i32, info: &mut i32) {

    unsafe {
        ffi::sormhr_(&(side as c_char), &(trans as c_char), &m, &n, &ilo, &ihi, a.as_ptr(), &lda,
                     tau.as_ptr(), c.as_mut_ptr(), &ldc, work.as_mut_ptr(), &lwork, info)
    }
}

#[inline]
pub fn dormhr(side: u8, trans: u8, m: i32, n: i32, ilo: i32, ihi: i32, a: &[f64], lda: i32,
              tau: &[f64], c: &mut [f64], ldc: i32, work: &mut [f64], lwork: i32, info: &mut i32) {

    unsafe {
        ffi::dormhr_(&(side as c_char), &(trans as c_char), &m, &n, &ilo, &ihi, a.as_ptr(), &lda,
                     tau.as_ptr(), c.as_mut_ptr(), &ldc, work.as_mut_ptr(), &lwork, info)
    }
}

#[inline]
pub fn cunghr(n: i32, ilo: i32, ihi: i32, a: &mut [c32], lda: i32, tau: &[c32], work: &mut [c32],
              lwork: i32, info: &mut i32) {

    unsafe {
        ffi::cunghr_(&n, &ilo, &ihi, a.as_mut_ptr() as *mut _, &lda, tau.as_ptr() as *const _,
                     work.as_mut_ptr() as *mut _, &lwork, info)
    }
}

#[inline]
pub fn zunghr(n: i32, ilo: i32, ihi: i32, a: &mut [c64], lda: i32, tau: &[c64], work: &mut [c64],
              lwork: i32, info: &mut i32) {

    unsafe {
        ffi::zunghr_(&n, &ilo, &ihi, a.as_mut_ptr() as *mut _, &lda, tau.as_ptr() as *const _,
                     work.as_mut_ptr() as *mut _, &lwork, info)
    }
}

#[inline]
pub fn cunmhr(side: u8, trans: u8, m: i32, n: i32, ilo: i32, ihi: i32, a: &[c32], lda: i32,
              tau: &[c32], c: &mut [c32], ldc: i32, work: &mut [c32], lwork: i32, info: &mut i32) {

    unsafe {
        ffi::cunmhr_(&(side as c_char), &(trans as c_char), &m, &n, &ilo, &ihi,
                     a.as_ptr() as *const _, &lda, tau.as_ptr() as *const _,
                     c.as_mut_ptr() as *mut _, &ldc, work.as_mut_ptr() as *mut _, &lwork, info)
    }
}

#[inline]
pub fn zunmhr(side: u8, trans: u8, m: i32, n: i32, ilo: i32, ihi: i32, a: &[c64], lda: i32,
              tau: &[c64], c: &mut [c64], ldc: i32, work: &mut [c64], lwork: i32, info: &mut i32) {

    unsafe {
        ffi::zunmhr_(&(side as c_char), &(trans as c_char), &m, &n, &ilo, &ihi,
                     a.as_ptr() as *const _, &lda, tau.as_ptr() as *const _,
                     c.as_mut_ptr() as *mut _, &ldc, work.as_mut_ptr() as *mut _, &lwork, info)
    }
}

#[inline]
pub fn sgebal(job: u8, n: i32, a: &mut [f32], lda: i32, ilo: &mut i32, ihi: &mut i32,
              scale: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::sgebal_(&(job as c_char), &n, a.as_mut_ptr(), &lda, ilo, ihi, scale.as_mut_ptr(),
                     info)
    }
}

#[inline]
pub fn dgebal(job: u8, n: i32, a: &mut [f64], lda: i32, ilo: &mut i32, ihi: &mut i32,
              scale: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::dgebal_(&(job as c_char), &n, a.as_mut_ptr(), &lda, ilo, ihi, scale.as_mut_ptr(),
                     info)
    }
}

#[inline]
pub fn cgebal(job: u8, n: i32, a: &mut [c32], lda: i32, ilo: &mut i32, ihi: &mut i32,
              scale: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::cgebal_(&(job as c_char), &n, a.as_mut_ptr() as *mut _, &lda, ilo, ihi,
                     scale.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zgebal(job: u8, n: i32, a: &mut [c64], lda: i32, ilo: &mut i32, ihi: &mut i32,
              scale: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::zgebal_(&(job as c_char), &n, a.as_mut_ptr() as *mut _, &lda, ilo, ihi,
                     scale.as_mut_ptr(), info)
    }
}

#[inline]
pub fn sgebak(job: u8, side: u8, n: i32, ilo: i32, ihi: i32, scale: &[f32], m: i32, v: &mut [f32],
              ldv: i32, info: &mut i32) {

    unsafe {
        ffi::sgebak_(&(job as c_char), &(side as c_char), &n, &ilo, &ihi, scale.as_ptr(), &m,
                     v.as_mut_ptr(), &ldv, info)
    }
}

#[inline]
pub fn dgebak(job: u8, side: u8, n: i32, ilo: i32, ihi: i32, scale: &[f64], m: i32, v: &mut [f64],
              ldv: i32, info: &mut i32) {

    unsafe {
        ffi::dgebak_(&(job as c_char), &(side as c_char), &n, &ilo, &ihi, scale.as_ptr(), &m,
                     v.as_mut_ptr(), &ldv, info)
    }
}

#[inline]
pub fn cgebak(job: u8, side: u8, n: i32, ilo: i32, ihi: i32, scale: &[f32], m: i32, v: &mut [c32],
              ldv: i32, info: &mut i32) {

    unsafe {
        ffi::cgebak_(&(job as c_char), &(side as c_char), &n, &ilo, &ihi, scale.as_ptr(), &m,
                     v.as_mut_ptr() as *mut _, &ldv, info)
    }
}

#[inline]
pub fn zgebak(job: u8, side: u8, n: i32, ilo: i32, ihi: i32, scale: &[f64], m: i32, v: &mut [c64],
              ldv: i32, info: &mut i32) {

    unsafe {
        ffi::zgebak_(&(job as c_char), &(side as c_char), &n, &ilo, &ihi, scale.as_ptr(), &m,
                     v.as_mut_ptr() as *mut _, &ldv, info)
    }
}

#[inline]
pub fn shseqr(job: u8, compz: u8, n: i32, ilo: i32, ihi: i32, h: &mut [f32], ldh: i32,
              wr: &mut [f32], wi: &mut [f32], z: &mut [f32], ldz: i32, work: &mut [f32],
              lwork: i32, info: &mut i32) {

    unsafe {
        ffi::shseqr_(&(job as c_char), &(compz as c_char), &n, &ilo, &ihi, h.as_mut_ptr(), &ldh,
                     wr.as_mut_ptr(), wi.as_mut_ptr(), z.as_mut_ptr(), &ldz, work.as_mut_ptr(),
                     &lwork, info)
    }
}

#[inline]
pub fn dhseqr(job: u8, compz: u8, n: i32, ilo: i32, ihi: i32, h: &mut [f64], ldh: i32,
              wr: &mut [f64], wi: &mut [f64], z: &mut [f64], ldz: i32, work: &mut [f64],
              lwork: i32, info: &mut i32) {

    unsafe {
        ffi::dhseqr_(&(job as c_char), &(compz as c_char), &n, &ilo, &ihi, h.as_mut_ptr(), &ldh,
                     wr.as_mut_ptr(), wi.as_mut_ptr(), z.as_mut_ptr(), &ldz, work.as_mut_ptr(),
                     &lwork, info)
    }
}

#[inline]
pub fn chseqr(job: u8, compz: u8, n: i32, ilo: i32, ihi: i32, h: &mut [c32], ldh: i32,
              w: &mut [c32], z: &mut [c32], ldz: i32, work: &mut [c32], lwork: i32,
              info: &mut i32) {

    unsafe {
        ffi::chseqr_(&(job as c_char), &(compz as c_char), &n, &ilo, &ihi,
                     h.as_mut_ptr() as *mut _, &ldh, w.as_mut_ptr() as *mut _,
                     z.as_mut_ptr() as *mut _, &ldz, work.as_mut_ptr() as *mut _, &lwork, info)
    }
}

#[inline]
pub fn zhseqr(job: u8, compz: u8, n: i32, ilo: i32, ihi: i32, h: &mut [c64], ldh: i32,
              w: &mut [c64], z: &mut [c64], ldz: i32, work: &mut [c64], lwork: i32,
              info: &mut i32) {

    unsafe {
        ffi::zhseqr_(&(job as c_char), &(compz as c_char), &n, &ilo, &ihi,
                     h.as_mut_ptr() as *mut _, &ldh, w.as_mut_ptr() as *mut _,
                     z.as_mut_ptr() as *mut _, &ldz, work.as_mut_ptr() as *mut _, &lwork, info)
    }
}

#[inline]
pub fn shsein(job: u8, eigsrc: u8, initv: u8, select: &mut [i32], n: i32, h: &[f32], ldh: i32,
              wr: &mut [f32], wi: &[f32], vl: &mut f32, ldvl: i32, vr: &mut f32, ldvr: i32,
              mm: i32, m: &mut i32, work: &mut [f32], ifaill: &mut i32, ifailr: &mut i32,
              info: &mut i32) {

    unsafe {
        ffi::shsein_(&(job as c_char), &(eigsrc as c_char), &(initv as c_char),
                     select.as_mut_ptr(), &n, h.as_ptr(), &ldh, wr.as_mut_ptr(), wi.as_ptr(), vl,
                     &ldvl, vr, &ldvr, &mm, m, work.as_mut_ptr(), ifaill, ifailr, info)
    }
}

#[inline]
pub fn dhsein(job: u8, eigsrc: u8, initv: u8, select: &mut [i32], n: i32, h: &[f64], ldh: i32,
              wr: &mut [f64], wi: &[f64], vl: &mut f64, ldvl: i32, vr: &mut f64, ldvr: i32,
              mm: i32, m: &mut i32, work: &mut [f64], ifaill: &mut i32, ifailr: &mut i32,
              info: &mut i32) {

    unsafe {
        ffi::dhsein_(&(job as c_char), &(eigsrc as c_char), &(initv as c_char),
                     select.as_mut_ptr(), &n, h.as_ptr(), &ldh, wr.as_mut_ptr(), wi.as_ptr(), vl,
                     &ldvl, vr, &ldvr, &mm, m, work.as_mut_ptr(), ifaill, ifailr, info)
    }
}

#[inline]
pub fn chsein(job: u8, eigsrc: u8, initv: u8, select: &[i32], n: i32, h: &[c32], ldh: i32,
              w: &mut [c32], vl: &mut c32, ldvl: i32, vr: &mut c32, ldvr: i32, mm: i32,
              m: &mut i32, work: &mut [c32], rwork: &mut [f32], ifaill: &mut i32, ifailr: &mut i32,
              info: &mut i32) {

    unsafe {
        ffi::chsein_(&(job as c_char), &(eigsrc as c_char), &(initv as c_char), select.as_ptr(),
                     &n, h.as_ptr() as *const _, &ldh, w.as_mut_ptr() as *mut _,
                     vl as *mut _ as *mut _, &ldvl, vr as *mut _ as *mut _, &ldvr, &mm, m,
                     work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), ifaill, ifailr, info)
    }
}

#[inline]
pub fn zhsein(job: u8, eigsrc: u8, initv: u8, select: &[i32], n: i32, h: &[c64], ldh: i32,
              w: &mut [c64], vl: &mut c64, ldvl: i32, vr: &mut c64, ldvr: i32, mm: i32,
              m: &mut i32, work: &mut [c64], rwork: &mut [f64], ifaill: &mut i32, ifailr: &mut i32,
              info: &mut i32) {

    unsafe {
        ffi::zhsein_(&(job as c_char), &(eigsrc as c_char), &(initv as c_char), select.as_ptr(),
                     &n, h.as_ptr() as *const _, &ldh, w.as_mut_ptr() as *mut _,
                     vl as *mut _ as *mut _, &ldvl, vr as *mut _ as *mut _, &ldvr, &mm, m,
                     work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), ifaill, ifailr, info)
    }
}

#[inline]
pub fn strevc(side: u8, howmny: u8, select: &mut [i32], n: i32, t: &[f32], ldt: i32, vl: &mut f32,
              ldvl: i32, vr: &mut f32, ldvr: i32, mm: i32, m: &mut i32, work: &mut [f32],
              info: &mut i32) {

    unsafe {
        ffi::strevc_(&(side as c_char), &(howmny as c_char), select.as_mut_ptr(), &n, t.as_ptr(),
                     &ldt, vl, &ldvl, vr, &ldvr, &mm, m, work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dtrevc(side: u8, howmny: u8, select: &mut [i32], n: i32, t: &[f64], ldt: i32, vl: &mut f64,
              ldvl: i32, vr: &mut f64, ldvr: i32, mm: i32, m: &mut i32, work: &mut [f64],
              info: &mut i32) {

    unsafe {
        ffi::dtrevc_(&(side as c_char), &(howmny as c_char), select.as_mut_ptr(), &n, t.as_ptr(),
                     &ldt, vl, &ldvl, vr, &ldvr, &mm, m, work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn ctrevc(side: u8, howmny: u8, select: &[i32], n: i32, t: &mut [c32], ldt: i32, vl: &mut c32,
              ldvl: i32, vr: &mut c32, ldvr: i32, mm: i32, m: &mut i32, work: &mut [c32],
              rwork: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::ctrevc_(&(side as c_char), &(howmny as c_char), select.as_ptr(), &n,
                     t.as_mut_ptr() as *mut _, &ldt, vl as *mut _ as *mut _, &ldvl,
                     vr as *mut _ as *mut _, &ldvr, &mm, m, work.as_mut_ptr() as *mut _,
                     rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn ztrevc(side: u8, howmny: u8, select: &[i32], n: i32, t: &mut [c64], ldt: i32, vl: &mut c64,
              ldvl: i32, vr: &mut c64, ldvr: i32, mm: i32, m: &mut i32, work: &mut [c64],
              rwork: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::ztrevc_(&(side as c_char), &(howmny as c_char), select.as_ptr(), &n,
                     t.as_mut_ptr() as *mut _, &ldt, vl as *mut _ as *mut _, &ldvl,
                     vr as *mut _ as *mut _, &ldvr, &mm, m, work.as_mut_ptr() as *mut _,
                     rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn strsna(job: u8, howmny: u8, select: &[i32], n: i32, t: &[f32], ldt: i32, vl: f32, ldvl: i32,
              vr: f32, ldvr: i32, s: &mut [f32], sep: &mut [f32], mm: i32, m: &mut i32,
              work: &mut [f32], ldwork: i32, iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::strsna_(&(job as c_char), &(howmny as c_char), select.as_ptr(), &n, t.as_ptr(), &ldt,
                     &vl, &ldvl, &vr, &ldvr, s.as_mut_ptr(), sep.as_mut_ptr(), &mm, m,
                     work.as_mut_ptr(), &ldwork, iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dtrsna(job: u8, howmny: u8, select: &[i32], n: i32, t: &[f64], ldt: i32, vl: f64, ldvl: i32,
              vr: f64, ldvr: i32, s: &mut [f64], sep: &mut [f64], mm: i32, m: &mut i32,
              work: &mut [f64], ldwork: i32, iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::dtrsna_(&(job as c_char), &(howmny as c_char), select.as_ptr(), &n, t.as_ptr(), &ldt,
                     &vl, &ldvl, &vr, &ldvr, s.as_mut_ptr(), sep.as_mut_ptr(), &mm, m,
                     work.as_mut_ptr(), &ldwork, iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn ctrsna(job: u8, howmny: u8, select: &[i32], n: i32, t: &[c32], ldt: i32, vl: c32, ldvl: i32,
              vr: c32, ldvr: i32, s: &mut [f32], sep: &mut [f32], mm: i32, m: &mut i32,
              work: &mut [c32], ldwork: i32, rwork: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::ctrsna_(&(job as c_char), &(howmny as c_char), select.as_ptr(), &n,
                     t.as_ptr() as *const _, &ldt, &vl as *const _ as *const _, &ldvl,
                     &vr as *const _ as *const _, &ldvr, s.as_mut_ptr(), sep.as_mut_ptr(), &mm, m,
                     work.as_mut_ptr() as *mut _, &ldwork, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn ztrsna(job: u8, howmny: u8, select: &[i32], n: i32, t: &[c64], ldt: i32, vl: c64, ldvl: i32,
              vr: c64, ldvr: i32, s: &mut [f64], sep: &mut [f64], mm: i32, m: &mut i32,
              work: &mut [c64], ldwork: i32, rwork: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::ztrsna_(&(job as c_char), &(howmny as c_char), select.as_ptr(), &n,
                     t.as_ptr() as *const _, &ldt, &vl as *const _ as *const _, &ldvl,
                     &vr as *const _ as *const _, &ldvr, s.as_mut_ptr(), sep.as_mut_ptr(), &mm, m,
                     work.as_mut_ptr() as *mut _, &ldwork, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn strexc(compq: u8, n: i32, t: &mut [f32], ldt: i32, q: &mut f32, ldq: i32, ifst: &[i32],
              ilst: &mut [i32], work: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::strexc_(&(compq as c_char), &n, t.as_mut_ptr(), &ldt, q, &ldq, ifst.as_ptr(),
                     ilst.as_mut_ptr(), work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dtrexc(compq: u8, n: i32, t: &mut [f64], ldt: i32, q: &mut f64, ldq: i32, ifst: &[i32],
              ilst: &mut [i32], work: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::dtrexc_(&(compq as c_char), &n, t.as_mut_ptr(), &ldt, q, &ldq, ifst.as_ptr(),
                     ilst.as_mut_ptr(), work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn ctrexc(compq: u8, n: i32, t: &mut [c32], ldt: i32, q: &mut c32, ldq: i32, ifst: &[i32],
              ilst: &[i32], info: i32) {

    unsafe {
        ffi::ctrexc_(&(compq as c_char), &n, t.as_mut_ptr() as *mut _, &ldt, q as *mut _ as *mut _,
                     &ldq, ifst.as_ptr(), ilst.as_ptr(), &info)
    }
}

#[inline]
pub fn ztrexc(compq: u8, n: i32, t: &mut [c64], ldt: i32, q: &mut c64, ldq: i32, ifst: &[i32],
              ilst: &[i32], info: i32) {

    unsafe {
        ffi::ztrexc_(&(compq as c_char), &n, t.as_mut_ptr() as *mut _, &ldt, q as *mut _ as *mut _,
                     &ldq, ifst.as_ptr(), ilst.as_ptr(), &info)
    }
}

#[inline]
pub fn strsen(job: u8, compq: u8, select: &[i32], n: i32, t: &mut [f32], ldt: i32, q: &mut f32,
              ldq: i32, wr: &mut [f32], wi: &mut [f32], m: &mut i32, s: &mut [f32],
              sep: &mut [f32], work: &mut [f32], lwork: i32, iwork: &mut [i32], liwork: i32,
              info: &mut i32) {

    unsafe {
        ffi::strsen_(&(job as c_char), &(compq as c_char), select.as_ptr(), &n, t.as_mut_ptr(),
                     &ldt, q, &ldq, wr.as_mut_ptr(), wi.as_mut_ptr(), m, s.as_mut_ptr(),
                     sep.as_mut_ptr(), work.as_mut_ptr(), &lwork, iwork.as_mut_ptr(), &liwork,
                     info)
    }
}

#[inline]
pub fn dtrsen(job: u8, compq: u8, select: &[i32], n: i32, t: &mut [f64], ldt: i32, q: &mut f64,
              ldq: i32, wr: &mut [f64], wi: &mut [f64], m: &mut i32, s: &mut [f64],
              sep: &mut [f64], work: &mut [f64], lwork: i32, iwork: &mut [i32], liwork: i32,
              info: &mut i32) {

    unsafe {
        ffi::dtrsen_(&(job as c_char), &(compq as c_char), select.as_ptr(), &n, t.as_mut_ptr(),
                     &ldt, q, &ldq, wr.as_mut_ptr(), wi.as_mut_ptr(), m, s.as_mut_ptr(),
                     sep.as_mut_ptr(), work.as_mut_ptr(), &lwork, iwork.as_mut_ptr(), &liwork,
                     info)
    }
}

#[inline]
pub fn ctrsen(job: u8, compq: u8, select: &[i32], n: i32, t: &mut [c32], ldt: i32, q: &mut c32,
              ldq: i32, w: &mut [c32], m: &mut i32, s: &mut [f32], sep: &mut [f32],
              work: &mut [c32], lwork: i32, info: &mut i32) {

    unsafe {
        ffi::ctrsen_(&(job as c_char), &(compq as c_char), select.as_ptr(), &n,
                     t.as_mut_ptr() as *mut _, &ldt, q as *mut _ as *mut _, &ldq,
                     w.as_mut_ptr() as *mut _, m, s.as_mut_ptr(), sep.as_mut_ptr(),
                     work.as_mut_ptr() as *mut _, &lwork, info)
    }
}

#[inline]
pub fn ztrsen(job: u8, compq: u8, select: &[i32], n: i32, t: &mut [c64], ldt: i32, q: &mut c64,
              ldq: i32, w: &mut [c64], m: &mut i32, s: &mut [f64], sep: &mut [f64],
              work: &mut [c64], lwork: i32, info: &mut i32) {

    unsafe {
        ffi::ztrsen_(&(job as c_char), &(compq as c_char), select.as_ptr(), &n,
                     t.as_mut_ptr() as *mut _, &ldt, q as *mut _ as *mut _, &ldq,
                     w.as_mut_ptr() as *mut _, m, s.as_mut_ptr(), sep.as_mut_ptr(),
                     work.as_mut_ptr() as *mut _, &lwork, info)
    }
}

#[inline]
pub fn strsyl(trana: u8, tranb: u8, isgn: &[i32], m: i32, n: i32, a: &[f32], lda: i32, b: &[f32],
              ldb: i32, c: &mut [f32], ldc: i32, scale: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::strsyl_(&(trana as c_char), &(tranb as c_char), isgn.as_ptr(), &m, &n, a.as_ptr(),
                     &lda, b.as_ptr(), &ldb, c.as_mut_ptr(), &ldc, scale.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dtrsyl(trana: u8, tranb: u8, isgn: &[i32], m: i32, n: i32, a: &[f64], lda: i32, b: &[f64],
              ldb: i32, c: &mut [f64], ldc: i32, scale: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::dtrsyl_(&(trana as c_char), &(tranb as c_char), isgn.as_ptr(), &m, &n, a.as_ptr(),
                     &lda, b.as_ptr(), &ldb, c.as_mut_ptr(), &ldc, scale.as_mut_ptr(), info)
    }
}

#[inline]
pub fn ctrsyl(trana: u8, tranb: u8, isgn: &[i32], m: i32, n: i32, a: &[c32], lda: i32, b: &[c32],
              ldb: i32, c: &mut [c32], ldc: i32, scale: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::ctrsyl_(&(trana as c_char), &(tranb as c_char), isgn.as_ptr(), &m, &n,
                     a.as_ptr() as *const _, &lda, b.as_ptr() as *const _, &ldb,
                     c.as_mut_ptr() as *mut _, &ldc, scale.as_mut_ptr(), info)
    }
}

#[inline]
pub fn ztrsyl(trana: u8, tranb: u8, isgn: &[i32], m: i32, n: i32, a: &[c64], lda: i32, b: &[c64],
              ldb: i32, c: &mut [c64], ldc: i32, scale: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::ztrsyl_(&(trana as c_char), &(tranb as c_char), isgn.as_ptr(), &m, &n,
                     a.as_ptr() as *const _, &lda, b.as_ptr() as *const _, &ldb,
                     c.as_mut_ptr() as *mut _, &ldc, scale.as_mut_ptr(), info)
    }
}

#[inline]
pub fn sgghrd(compq: u8, compz: u8, n: i32, ilo: i32, ihi: i32, a: &mut [f32], lda: i32,
              b: &mut [f32], ldb: i32, q: &mut f32, ldq: i32, z: &mut [f32], ldz: i32,
              info: &mut i32) {

    unsafe {
        ffi::sgghrd_(&(compq as c_char), &(compz as c_char), &n, &ilo, &ihi, a.as_mut_ptr(), &lda,
                     b.as_mut_ptr(), &ldb, q, &ldq, z.as_mut_ptr(), &ldz, info)
    }
}

#[inline]
pub fn dgghrd(compq: u8, compz: u8, n: i32, ilo: i32, ihi: i32, a: &mut [f64], lda: i32,
              b: &mut [f64], ldb: i32, q: &mut f64, ldq: i32, z: &mut [f64], ldz: i32,
              info: &mut i32) {

    unsafe {
        ffi::dgghrd_(&(compq as c_char), &(compz as c_char), &n, &ilo, &ihi, a.as_mut_ptr(), &lda,
                     b.as_mut_ptr(), &ldb, q, &ldq, z.as_mut_ptr(), &ldz, info)
    }
}

#[inline]
pub fn cgghrd(compq: u8, compz: u8, n: i32, ilo: i32, ihi: i32, a: &mut [c32], lda: i32,
              b: &mut [c32], ldb: i32, q: &mut c32, ldq: i32, z: &mut [c32], ldz: i32,
              info: &mut i32) {

    unsafe {
        ffi::cgghrd_(&(compq as c_char), &(compz as c_char), &n, &ilo, &ihi,
                     a.as_mut_ptr() as *mut _, &lda, b.as_mut_ptr() as *mut _, &ldb,
                     q as *mut _ as *mut _, &ldq, z.as_mut_ptr() as *mut _, &ldz, info)
    }
}

#[inline]
pub fn zgghrd(compq: u8, compz: u8, n: i32, ilo: i32, ihi: i32, a: &mut [c64], lda: i32,
              b: &mut [c64], ldb: i32, q: &mut c64, ldq: i32, z: &mut [c64], ldz: i32,
              info: &mut i32) {

    unsafe {
        ffi::zgghrd_(&(compq as c_char), &(compz as c_char), &n, &ilo, &ihi,
                     a.as_mut_ptr() as *mut _, &lda, b.as_mut_ptr() as *mut _, &ldb,
                     q as *mut _ as *mut _, &ldq, z.as_mut_ptr() as *mut _, &ldz, info)
    }
}

#[inline]
pub fn sgghd3(compq: u8, compz: u8, n: i32, ilo: i32, ihi: i32, a: &mut [f32], lda: i32,
              b: &mut [f32], ldb: i32, q: &mut f32, ldq: i32, z: &mut [f32], ldz: i32,
              work: &mut [f32], lwork: i32, info: &mut i32) {

    unsafe {
        ffi::sgghd3_(&(compq as c_char), &(compz as c_char), &n, &ilo, &ihi, a.as_mut_ptr(), &lda,
                     b.as_mut_ptr(), &ldb, q, &ldq, z.as_mut_ptr(), &ldz, work.as_mut_ptr(),
                     &lwork, info)
    }
}

#[inline]
pub fn dgghd3(compq: u8, compz: u8, n: i32, ilo: i32, ihi: i32, a: &mut [f64], lda: i32,
              b: &mut [f64], ldb: i32, q: &mut f64, ldq: i32, z: &mut [f64], ldz: i32,
              work: &mut [f64], lwork: i32, info: &mut i32) {

    unsafe {
        ffi::dgghd3_(&(compq as c_char), &(compz as c_char), &n, &ilo, &ihi, a.as_mut_ptr(), &lda,
                     b.as_mut_ptr(), &ldb, q, &ldq, z.as_mut_ptr(), &ldz, work.as_mut_ptr(),
                     &lwork, info)
    }
}

#[inline]
pub fn cgghd3(compq: u8, compz: u8, n: i32, ilo: i32, ihi: i32, a: &mut [c32], lda: i32,
              b: &mut [c32], ldb: i32, q: &mut c32, ldq: i32, z: &mut [c32], ldz: i32,
              work: &mut [c32], lwork: i32, info: &mut i32) {

    unsafe {
        ffi::cgghd3_(&(compq as c_char), &(compz as c_char), &n, &ilo, &ihi,
                     a.as_mut_ptr() as *mut _, &lda, b.as_mut_ptr() as *mut _, &ldb,
                     q as *mut _ as *mut _, &ldq, z.as_mut_ptr() as *mut _, &ldz,
                     work.as_mut_ptr() as *mut _, &lwork, info)
    }
}

#[inline]
pub fn zgghd3(compq: u8, compz: u8, n: i32, ilo: i32, ihi: i32, a: &mut [c64], lda: i32,
              b: &mut [c64], ldb: i32, q: &mut c64, ldq: i32, z: &mut [c64], ldz: i32,
              work: &mut [c64], lwork: i32, info: &mut i32) {

    unsafe {
        ffi::zgghd3_(&(compq as c_char), &(compz as c_char), &n, &ilo, &ihi,
                     a.as_mut_ptr() as *mut _, &lda, b.as_mut_ptr() as *mut _, &ldb,
                     q as *mut _ as *mut _, &ldq, z.as_mut_ptr() as *mut _, &ldz,
                     work.as_mut_ptr() as *mut _, &lwork, info)
    }
}

#[inline]
pub fn sggbal(job: u8, n: i32, a: &mut [f32], lda: i32, b: &mut [f32], ldb: i32, ilo: &mut i32,
              ihi: &mut i32, lscale: &mut [f32], rscale: &mut [f32], work: &mut [f32],
              info: &mut i32) {

    unsafe {
        ffi::sggbal_(&(job as c_char), &n, a.as_mut_ptr(), &lda, b.as_mut_ptr(), &ldb, ilo, ihi,
                     lscale.as_mut_ptr(), rscale.as_mut_ptr(), work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dggbal(job: u8, n: i32, a: &mut [f64], lda: i32, b: &mut [f64], ldb: i32, ilo: &mut i32,
              ihi: &mut i32, lscale: &mut [f64], rscale: &mut [f64], work: &mut [f64],
              info: &mut i32) {

    unsafe {
        ffi::dggbal_(&(job as c_char), &n, a.as_mut_ptr(), &lda, b.as_mut_ptr(), &ldb, ilo, ihi,
                     lscale.as_mut_ptr(), rscale.as_mut_ptr(), work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn cggbal(job: u8, n: i32, a: &mut [c32], lda: i32, b: &mut [c32], ldb: i32, ilo: &mut i32,
              ihi: &mut i32, lscale: &mut [f32], rscale: &mut [f32], work: &mut [f32],
              info: &mut i32) {

    unsafe {
        ffi::cggbal_(&(job as c_char), &n, a.as_mut_ptr() as *mut _, &lda,
                     b.as_mut_ptr() as *mut _, &ldb, ilo, ihi, lscale.as_mut_ptr(),
                     rscale.as_mut_ptr(), work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zggbal(job: u8, n: i32, a: &mut [c64], lda: i32, b: &mut [c64], ldb: i32, ilo: &mut i32,
              ihi: &mut i32, lscale: &mut [f64], rscale: &mut [f64], work: &mut [f64],
              info: &mut i32) {

    unsafe {
        ffi::zggbal_(&(job as c_char), &n, a.as_mut_ptr() as *mut _, &lda,
                     b.as_mut_ptr() as *mut _, &ldb, ilo, ihi, lscale.as_mut_ptr(),
                     rscale.as_mut_ptr(), work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn sggbak(job: u8, side: u8, n: i32, ilo: i32, ihi: i32, lscale: &[f32], rscale: &[f32],
              m: i32, v: &mut [f32], ldv: i32, info: &mut i32) {

    unsafe {
        ffi::sggbak_(&(job as c_char), &(side as c_char), &n, &ilo, &ihi, lscale.as_ptr(),
                     rscale.as_ptr(), &m, v.as_mut_ptr(), &ldv, info)
    }
}

#[inline]
pub fn dggbak(job: u8, side: u8, n: i32, ilo: i32, ihi: i32, lscale: &[f64], rscale: &[f64],
              m: i32, v: &mut [f64], ldv: i32, info: &mut i32) {

    unsafe {
        ffi::dggbak_(&(job as c_char), &(side as c_char), &n, &ilo, &ihi, lscale.as_ptr(),
                     rscale.as_ptr(), &m, v.as_mut_ptr(), &ldv, info)
    }
}

#[inline]
pub fn cggbak(job: u8, side: u8, n: i32, ilo: i32, ihi: i32, lscale: &[f32], rscale: &[f32],
              m: i32, v: &mut [c32], ldv: i32, info: &mut i32) {

    unsafe {
        ffi::cggbak_(&(job as c_char), &(side as c_char), &n, &ilo, &ihi, lscale.as_ptr(),
                     rscale.as_ptr(), &m, v.as_mut_ptr() as *mut _, &ldv, info)
    }
}

#[inline]
pub fn zggbak(job: u8, side: u8, n: i32, ilo: i32, ihi: i32, lscale: &[f64], rscale: &[f64],
              m: i32, v: &mut [c64], ldv: i32, info: &mut i32) {

    unsafe {
        ffi::zggbak_(&(job as c_char), &(side as c_char), &n, &ilo, &ihi, lscale.as_ptr(),
                     rscale.as_ptr(), &m, v.as_mut_ptr() as *mut _, &ldv, info)
    }
}

#[inline]
pub fn shgeqz(job: u8, compq: u8, compz: u8, n: i32, ilo: i32, ihi: i32, h: &mut [f32], ldh: i32,
              t: &mut [f32], ldt: i32, alphar: &mut f32, alphai: &mut f32, beta: &mut f32,
              q: &mut f32, ldq: i32, z: &mut [f32], ldz: i32, work: &mut [f32], lwork: i32,
              info: &mut i32) {

    unsafe {
        ffi::shgeqz_(&(job as c_char), &(compq as c_char), &(compz as c_char), &n, &ilo, &ihi,
                     h.as_mut_ptr(), &ldh, t.as_mut_ptr(), &ldt, alphar, alphai, beta, q, &ldq,
                     z.as_mut_ptr(), &ldz, work.as_mut_ptr(), &lwork, info)
    }
}

#[inline]
pub fn dhgeqz(job: u8, compq: u8, compz: u8, n: i32, ilo: i32, ihi: i32, h: &mut [f64], ldh: i32,
              t: &mut [f64], ldt: i32, alphar: &mut f64, alphai: &mut f64, beta: &mut f64,
              q: &mut f64, ldq: i32, z: &mut [f64], ldz: i32, work: &mut [f64], lwork: i32,
              info: &mut i32) {

    unsafe {
        ffi::dhgeqz_(&(job as c_char), &(compq as c_char), &(compz as c_char), &n, &ilo, &ihi,
                     h.as_mut_ptr(), &ldh, t.as_mut_ptr(), &ldt, alphar, alphai, beta, q, &ldq,
                     z.as_mut_ptr(), &ldz, work.as_mut_ptr(), &lwork, info)
    }
}

#[inline]
pub fn chgeqz(job: u8, compq: u8, compz: u8, n: i32, ilo: i32, ihi: i32, h: &mut [c32], ldh: i32,
              t: &mut [c32], ldt: i32, alpha: &mut c32, beta: &mut c32, q: &mut c32, ldq: i32,
              z: &mut [c32], ldz: i32, work: &mut [c32], lwork: i32, rwork: &mut [f32],
              info: &mut i32) {

    unsafe {
        ffi::chgeqz_(&(job as c_char), &(compq as c_char), &(compz as c_char), &n, &ilo, &ihi,
                     h.as_mut_ptr() as *mut _, &ldh, t.as_mut_ptr() as *mut _, &ldt,
                     alpha as *mut _ as *mut _, beta as *mut _ as *mut _, q as *mut _ as *mut _,
                     &ldq, z.as_mut_ptr() as *mut _, &ldz, work.as_mut_ptr() as *mut _, &lwork,
                     rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zhgeqz(job: u8, compq: u8, compz: u8, n: i32, ilo: i32, ihi: i32, h: &mut [c64], ldh: i32,
              t: &mut [c64], ldt: i32, alpha: &mut c64, beta: &mut c64, q: &mut c64, ldq: i32,
              z: &mut [c64], ldz: i32, work: &mut [c64], lwork: i32, rwork: &mut [f64],
              info: &mut i32) {

    unsafe {
        ffi::zhgeqz_(&(job as c_char), &(compq as c_char), &(compz as c_char), &n, &ilo, &ihi,
                     h.as_mut_ptr() as *mut _, &ldh, t.as_mut_ptr() as *mut _, &ldt,
                     alpha as *mut _ as *mut _, beta as *mut _ as *mut _, q as *mut _ as *mut _,
                     &ldq, z.as_mut_ptr() as *mut _, &ldz, work.as_mut_ptr() as *mut _, &lwork,
                     rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn stgevc(side: u8, howmny: u8, select: &[i32], n: i32, s: &[f32], lds: i32, p: f32, ldp: i32,
              vl: &mut f32, ldvl: i32, vr: &mut f32, ldvr: i32, mm: i32, m: &mut i32,
              work: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::stgevc_(&(side as c_char), &(howmny as c_char), select.as_ptr(), &n, s.as_ptr(), &lds,
                     &p, &ldp, vl, &ldvl, vr, &ldvr, &mm, m, work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dtgevc(side: u8, howmny: u8, select: &[i32], n: i32, s: &[f64], lds: i32, p: f64, ldp: i32,
              vl: &mut f64, ldvl: i32, vr: &mut f64, ldvr: i32, mm: i32, m: &mut i32,
              work: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::dtgevc_(&(side as c_char), &(howmny as c_char), select.as_ptr(), &n, s.as_ptr(), &lds,
                     &p, &ldp, vl, &ldvl, vr, &ldvr, &mm, m, work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn ctgevc(side: u8, howmny: u8, select: &[i32], n: i32, s: &[c32], lds: i32, p: c32, ldp: i32,
              vl: &mut c32, ldvl: i32, vr: &mut c32, ldvr: i32, mm: i32, m: &mut i32,
              work: &mut [c32], rwork: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::ctgevc_(&(side as c_char), &(howmny as c_char), select.as_ptr(), &n,
                     s.as_ptr() as *const _, &lds, &p as *const _ as *const _, &ldp,
                     vl as *mut _ as *mut _, &ldvl, vr as *mut _ as *mut _, &ldvr, &mm, m,
                     work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn ztgevc(side: u8, howmny: u8, select: &[i32], n: i32, s: &[c64], lds: i32, p: c64, ldp: i32,
              vl: &mut c64, ldvl: i32, vr: &mut c64, ldvr: i32, mm: i32, m: &mut i32,
              work: &mut [c64], rwork: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::ztgevc_(&(side as c_char), &(howmny as c_char), select.as_ptr(), &n,
                     s.as_ptr() as *const _, &lds, &p as *const _ as *const _, &ldp,
                     vl as *mut _ as *mut _, &ldvl, vr as *mut _ as *mut _, &ldvr, &mm, m,
                     work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn stgexc(wantq: &[i32], wantz: &[i32], n: i32, a: &mut [f32], lda: i32, b: &mut [f32],
              ldb: i32, q: &mut f32, ldq: i32, z: &mut [f32], ldz: i32, ifst: &mut [i32],
              ilst: &mut [i32], work: &mut [f32], lwork: i32, info: &mut i32) {

    unsafe {
        ffi::stgexc_(wantq.as_ptr(), wantz.as_ptr(), &n, a.as_mut_ptr(), &lda, b.as_mut_ptr(),
                     &ldb, q, &ldq, z.as_mut_ptr(), &ldz, ifst.as_mut_ptr(), ilst.as_mut_ptr(),
                     work.as_mut_ptr(), &lwork, info)
    }
}

#[inline]
pub fn dtgexc(wantq: &[i32], wantz: &[i32], n: i32, a: &mut [f64], lda: i32, b: &mut [f64],
              ldb: i32, q: &mut f64, ldq: i32, z: &mut [f64], ldz: i32, ifst: &mut [i32],
              ilst: &mut [i32], work: &mut [f64], lwork: i32, info: &mut i32) {

    unsafe {
        ffi::dtgexc_(wantq.as_ptr(), wantz.as_ptr(), &n, a.as_mut_ptr(), &lda, b.as_mut_ptr(),
                     &ldb, q, &ldq, z.as_mut_ptr(), &ldz, ifst.as_mut_ptr(), ilst.as_mut_ptr(),
                     work.as_mut_ptr(), &lwork, info)
    }
}

#[inline]
pub fn ctgexc(wantq: &[i32], wantz: &[i32], n: i32, a: &mut [c32], lda: i32, b: &mut [c32],
              ldb: i32, q: &mut c32, ldq: i32, z: &mut [c32], ldz: i32, ifst: &[i32],
              ilst: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::ctgexc_(wantq.as_ptr(), wantz.as_ptr(), &n, a.as_mut_ptr() as *mut _, &lda,
                     b.as_mut_ptr() as *mut _, &ldb, q as *mut _ as *mut _, &ldq,
                     z.as_mut_ptr() as *mut _, &ldz, ifst.as_ptr(), ilst.as_mut_ptr(), info)
    }
}

#[inline]
pub fn ztgexc(wantq: &[i32], wantz: &[i32], n: i32, a: &mut [c64], lda: i32, b: &mut [c64],
              ldb: i32, q: &mut c64, ldq: i32, z: &mut [c64], ldz: i32, ifst: &[i32],
              ilst: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::ztgexc_(wantq.as_ptr(), wantz.as_ptr(), &n, a.as_mut_ptr() as *mut _, &lda,
                     b.as_mut_ptr() as *mut _, &ldb, q as *mut _ as *mut _, &ldq,
                     z.as_mut_ptr() as *mut _, &ldz, ifst.as_ptr(), ilst.as_mut_ptr(), info)
    }
}

#[inline]
pub fn stgsen(ijob: &[i32], wantq: &[i32], wantz: &[i32], select: &[i32], n: i32, a: &mut [f32],
              lda: i32, b: &mut [f32], ldb: i32, alphar: &mut f32, alphai: &mut f32,
              beta: &mut f32, q: &mut f32, ldq: i32, z: &mut [f32], ldz: i32, m: &mut i32,
              pl: &mut [f32], pr: &mut [f32], dif: &mut [f32], work: &mut [f32], lwork: i32,
              iwork: &mut [i32], liwork: i32, info: &mut i32) {

    unsafe {
        ffi::stgsen_(ijob.as_ptr(), wantq.as_ptr(), wantz.as_ptr(), select.as_ptr(), &n,
                     a.as_mut_ptr(), &lda, b.as_mut_ptr(), &ldb, alphar, alphai, beta, q, &ldq,
                     z.as_mut_ptr(), &ldz, m, pl.as_mut_ptr(), pr.as_mut_ptr(), dif.as_mut_ptr(),
                     work.as_mut_ptr(), &lwork, iwork.as_mut_ptr(), &liwork, info)
    }
}

#[inline]
pub fn dtgsen(ijob: &[i32], wantq: &[i32], wantz: &[i32], select: &[i32], n: i32, a: &mut [f64],
              lda: i32, b: &mut [f64], ldb: i32, alphar: &mut f64, alphai: &mut f64,
              beta: &mut f64, q: &mut f64, ldq: i32, z: &mut [f64], ldz: i32, m: &mut i32,
              pl: &mut [f64], pr: &mut [f64], dif: &mut [f64], work: &mut [f64], lwork: i32,
              iwork: &mut [i32], liwork: i32, info: &mut i32) {

    unsafe {
        ffi::dtgsen_(ijob.as_ptr(), wantq.as_ptr(), wantz.as_ptr(), select.as_ptr(), &n,
                     a.as_mut_ptr(), &lda, b.as_mut_ptr(), &ldb, alphar, alphai, beta, q, &ldq,
                     z.as_mut_ptr(), &ldz, m, pl.as_mut_ptr(), pr.as_mut_ptr(), dif.as_mut_ptr(),
                     work.as_mut_ptr(), &lwork, iwork.as_mut_ptr(), &liwork, info)
    }
}

#[inline]
pub fn ctgsen(ijob: &[i32], wantq: &[i32], wantz: &[i32], select: &[i32], n: i32, a: &mut [c32],
              lda: i32, b: &mut [c32], ldb: i32, alpha: &mut c32, beta: &mut c32, q: &mut c32,
              ldq: i32, z: &mut [c32], ldz: i32, m: &mut i32, pl: &mut [f32], pr: &mut [f32],
              dif: &mut [f32], work: &mut [c32], lwork: i32, iwork: &mut [i32], liwork: i32,
              info: &mut i32) {

    unsafe {
        ffi::ctgsen_(ijob.as_ptr(), wantq.as_ptr(), wantz.as_ptr(), select.as_ptr(), &n,
                     a.as_mut_ptr() as *mut _, &lda, b.as_mut_ptr() as *mut _, &ldb,
                     alpha as *mut _ as *mut _, beta as *mut _ as *mut _, q as *mut _ as *mut _,
                     &ldq, z.as_mut_ptr() as *mut _, &ldz, m, pl.as_mut_ptr(), pr.as_mut_ptr(),
                     dif.as_mut_ptr(), work.as_mut_ptr() as *mut _, &lwork, iwork.as_mut_ptr(),
                     &liwork, info)
    }
}

#[inline]
pub fn ztgsen(ijob: &[i32], wantq: &[i32], wantz: &[i32], select: &[i32], n: i32, a: &mut [c64],
              lda: i32, b: &mut [c64], ldb: i32, alpha: &mut c64, beta: &mut c64, q: &mut c64,
              ldq: i32, z: &mut [c64], ldz: i32, m: &mut i32, pl: &mut [f64], pr: &mut [f64],
              dif: &mut [f64], work: &mut [c64], lwork: i32, iwork: &mut [i32], liwork: i32,
              info: &mut i32) {

    unsafe {
        ffi::ztgsen_(ijob.as_ptr(), wantq.as_ptr(), wantz.as_ptr(), select.as_ptr(), &n,
                     a.as_mut_ptr() as *mut _, &lda, b.as_mut_ptr() as *mut _, &ldb,
                     alpha as *mut _ as *mut _, beta as *mut _ as *mut _, q as *mut _ as *mut _,
                     &ldq, z.as_mut_ptr() as *mut _, &ldz, m, pl.as_mut_ptr(), pr.as_mut_ptr(),
                     dif.as_mut_ptr(), work.as_mut_ptr() as *mut _, &lwork, iwork.as_mut_ptr(),
                     &liwork, info)
    }
}

#[inline]
pub fn stgsyl(trans: u8, ijob: &[i32], m: i32, n: i32, a: &[f32], lda: i32, b: &[f32], ldb: i32,
              c: &mut [f32], ldc: i32, d: &[f32], ldd: i32, e: &[f32], lde: i32, f: &mut [f32],
              ldf: i32, scale: &mut [f32], dif: &mut [f32], work: &mut [f32], lwork: i32,
              iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::stgsyl_(&(trans as c_char), ijob.as_ptr(), &m, &n, a.as_ptr(), &lda, b.as_ptr(), &ldb,
                     c.as_mut_ptr(), &ldc, d.as_ptr(), &ldd, e.as_ptr(), &lde, f.as_mut_ptr(),
                     &ldf, scale.as_mut_ptr(), dif.as_mut_ptr(), work.as_mut_ptr(), &lwork,
                     iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dtgsyl(trans: u8, ijob: &[i32], m: i32, n: i32, a: &[f64], lda: i32, b: &[f64], ldb: i32,
              c: &mut [f64], ldc: i32, d: &[f64], ldd: i32, e: &[f64], lde: i32, f: &mut [f64],
              ldf: i32, scale: &mut [f64], dif: &mut [f64], work: &mut [f64], lwork: i32,
              iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::dtgsyl_(&(trans as c_char), ijob.as_ptr(), &m, &n, a.as_ptr(), &lda, b.as_ptr(), &ldb,
                     c.as_mut_ptr(), &ldc, d.as_ptr(), &ldd, e.as_ptr(), &lde, f.as_mut_ptr(),
                     &ldf, scale.as_mut_ptr(), dif.as_mut_ptr(), work.as_mut_ptr(), &lwork,
                     iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn ctgsyl(trans: u8, ijob: &[i32], m: i32, n: i32, a: &[c32], lda: i32, b: &[c32], ldb: i32,
              c: &mut [c32], ldc: i32, d: &[c32], ldd: i32, e: &[c32], lde: i32, f: &mut [c32],
              ldf: i32, scale: &mut [f32], dif: &mut [f32], work: &mut [c32], lwork: i32,
              iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::ctgsyl_(&(trans as c_char), ijob.as_ptr(), &m, &n, a.as_ptr() as *const _, &lda,
                     b.as_ptr() as *const _, &ldb, c.as_mut_ptr() as *mut _, &ldc,
                     d.as_ptr() as *const _, &ldd, e.as_ptr() as *const _, &lde,
                     f.as_mut_ptr() as *mut _, &ldf, scale.as_mut_ptr(), dif.as_mut_ptr(),
                     work.as_mut_ptr() as *mut _, &lwork, iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn ztgsyl(trans: u8, ijob: &[i32], m: i32, n: i32, a: &[c64], lda: i32, b: &[c64], ldb: i32,
              c: &mut [c64], ldc: i32, d: &[c64], ldd: i32, e: &[c64], lde: i32, f: &mut [c64],
              ldf: i32, scale: &mut [f64], dif: &mut [f64], work: &mut [c64], lwork: i32,
              iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::ztgsyl_(&(trans as c_char), ijob.as_ptr(), &m, &n, a.as_ptr() as *const _, &lda,
                     b.as_ptr() as *const _, &ldb, c.as_mut_ptr() as *mut _, &ldc,
                     d.as_ptr() as *const _, &ldd, e.as_ptr() as *const _, &lde,
                     f.as_mut_ptr() as *mut _, &ldf, scale.as_mut_ptr(), dif.as_mut_ptr(),
                     work.as_mut_ptr() as *mut _, &lwork, iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn stgsna(job: u8, howmny: u8, select: &[i32], n: i32, a: &[f32], lda: i32, b: &[f32],
              ldb: i32, vl: f32, ldvl: i32, vr: f32, ldvr: i32, s: &mut [f32], dif: &mut [f32],
              mm: i32, m: &mut i32, work: &mut [f32], lwork: i32, iwork: &mut [i32],
              info: &mut i32) {

    unsafe {
        ffi::stgsna_(&(job as c_char), &(howmny as c_char), select.as_ptr(), &n, a.as_ptr(), &lda,
                     b.as_ptr(), &ldb, &vl, &ldvl, &vr, &ldvr, s.as_mut_ptr(), dif.as_mut_ptr(),
                     &mm, m, work.as_mut_ptr(), &lwork, iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dtgsna(job: u8, howmny: u8, select: &[i32], n: i32, a: &[f64], lda: i32, b: &[f64],
              ldb: i32, vl: f64, ldvl: i32, vr: f64, ldvr: i32, s: &mut [f64], dif: &mut [f64],
              mm: i32, m: &mut i32, work: &mut [f64], lwork: i32, iwork: &mut [i32],
              info: &mut i32) {

    unsafe {
        ffi::dtgsna_(&(job as c_char), &(howmny as c_char), select.as_ptr(), &n, a.as_ptr(), &lda,
                     b.as_ptr(), &ldb, &vl, &ldvl, &vr, &ldvr, s.as_mut_ptr(), dif.as_mut_ptr(),
                     &mm, m, work.as_mut_ptr(), &lwork, iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn ctgsna(job: u8, howmny: u8, select: &[i32], n: i32, a: &[c32], lda: i32, b: &[c32],
              ldb: i32, vl: c32, ldvl: i32, vr: c32, ldvr: i32, s: &mut [f32], dif: &mut [f32],
              mm: i32, m: &mut i32, work: &mut [c32], lwork: i32, iwork: &mut [i32],
              info: &mut i32) {

    unsafe {
        ffi::ctgsna_(&(job as c_char), &(howmny as c_char), select.as_ptr(), &n,
                     a.as_ptr() as *const _, &lda, b.as_ptr() as *const _, &ldb,
                     &vl as *const _ as *const _, &ldvl, &vr as *const _ as *const _, &ldvr,
                     s.as_mut_ptr(), dif.as_mut_ptr(), &mm, m, work.as_mut_ptr() as *mut _, &lwork,
                     iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn ztgsna(job: u8, howmny: u8, select: &[i32], n: i32, a: &[c64], lda: i32, b: &[c64],
              ldb: i32, vl: c64, ldvl: i32, vr: c64, ldvr: i32, s: &mut [f64], dif: &mut [f64],
              mm: i32, m: &mut i32, work: &mut [c64], lwork: i32, iwork: &mut [i32],
              info: &mut i32) {

    unsafe {
        ffi::ztgsna_(&(job as c_char), &(howmny as c_char), select.as_ptr(), &n,
                     a.as_ptr() as *const _, &lda, b.as_ptr() as *const _, &ldb,
                     &vl as *const _ as *const _, &ldvl, &vr as *const _ as *const _, &ldvr,
                     s.as_mut_ptr(), dif.as_mut_ptr(), &mm, m, work.as_mut_ptr() as *mut _, &lwork,
                     iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn sggsvp(jobu: u8, jobv: u8, jobq: u8, m: i32, p: i32, n: i32, a: &mut [f32], lda: i32,
              b: &mut [f32], ldb: i32, tola: f32, tolb: f32, k: &mut i32, l: &mut i32,
              u: &mut [f32], ldu: i32, v: &mut [f32], ldv: i32, q: &mut f32, ldq: i32,
              iwork: &mut [i32], tau: &mut [f32], work: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::sggsvp_(&(jobu as c_char), &(jobv as c_char), &(jobq as c_char), &m, &p, &n,
                     a.as_mut_ptr(), &lda, b.as_mut_ptr(), &ldb, &tola, &tolb, k, l,
                     u.as_mut_ptr(), &ldu, v.as_mut_ptr(), &ldv, q, &ldq, iwork.as_mut_ptr(),
                     tau.as_mut_ptr(), work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dggsvp(jobu: u8, jobv: u8, jobq: u8, m: i32, p: i32, n: i32, a: &mut [f64], lda: i32,
              b: &mut [f64], ldb: i32, tola: f64, tolb: f64, k: &mut i32, l: &mut i32,
              u: &mut [f64], ldu: i32, v: &mut [f64], ldv: i32, q: &mut f64, ldq: i32,
              iwork: &mut [i32], tau: &mut [f64], work: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::dggsvp_(&(jobu as c_char), &(jobv as c_char), &(jobq as c_char), &m, &p, &n,
                     a.as_mut_ptr(), &lda, b.as_mut_ptr(), &ldb, &tola, &tolb, k, l,
                     u.as_mut_ptr(), &ldu, v.as_mut_ptr(), &ldv, q, &ldq, iwork.as_mut_ptr(),
                     tau.as_mut_ptr(), work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn cggsvp(jobu: u8, jobv: u8, jobq: u8, m: i32, p: i32, n: i32, a: &mut [c32], lda: i32,
              b: &mut [c32], ldb: i32, tola: f32, tolb: f32, k: &mut i32, l: &mut i32,
              u: &mut [c32], ldu: i32, v: &mut [c32], ldv: i32, q: &mut c32, ldq: i32,
              iwork: &mut [i32], rwork: &mut [f32], tau: &mut [c32], work: &mut [c32],
              info: &mut i32) {

    unsafe {
        ffi::cggsvp_(&(jobu as c_char), &(jobv as c_char), &(jobq as c_char), &m, &p, &n,
                     a.as_mut_ptr() as *mut _, &lda, b.as_mut_ptr() as *mut _, &ldb, &tola, &tolb,
                     k, l, u.as_mut_ptr() as *mut _, &ldu, v.as_mut_ptr() as *mut _, &ldv,
                     q as *mut _ as *mut _, &ldq, iwork.as_mut_ptr(), rwork.as_mut_ptr(),
                     tau.as_mut_ptr() as *mut _, work.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn zggsvp(jobu: u8, jobv: u8, jobq: u8, m: i32, p: i32, n: i32, a: &mut [c64], lda: i32,
              b: &mut [c64], ldb: i32, tola: f64, tolb: f64, k: &mut i32, l: &mut i32,
              u: &mut [c64], ldu: i32, v: &mut [c64], ldv: i32, q: &mut c64, ldq: i32,
              iwork: &mut [i32], rwork: &mut [f64], tau: &mut [c64], work: &mut [c64],
              info: &mut i32) {

    unsafe {
        ffi::zggsvp_(&(jobu as c_char), &(jobv as c_char), &(jobq as c_char), &m, &p, &n,
                     a.as_mut_ptr() as *mut _, &lda, b.as_mut_ptr() as *mut _, &ldb, &tola, &tolb,
                     k, l, u.as_mut_ptr() as *mut _, &ldu, v.as_mut_ptr() as *mut _, &ldv,
                     q as *mut _ as *mut _, &ldq, iwork.as_mut_ptr(), rwork.as_mut_ptr(),
                     tau.as_mut_ptr() as *mut _, work.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn sggsvp3(jobu: u8, jobv: u8, jobq: u8, m: i32, p: i32, n: i32, a: &mut [f32], lda: i32,
               b: &mut [f32], ldb: i32, tola: f32, tolb: f32, k: &mut i32, l: &mut i32,
               u: &mut [f32], ldu: i32, v: &mut [f32], ldv: i32, q: &mut f32, ldq: i32,
               iwork: &mut [i32], tau: &mut [f32], work: &mut [f32], lwork: i32, info: &mut i32) {

    unsafe {
        ffi::sggsvp3_(&(jobu as c_char), &(jobv as c_char), &(jobq as c_char), &m, &p, &n,
                      a.as_mut_ptr(), &lda, b.as_mut_ptr(), &ldb, &tola, &tolb, k, l,
                      u.as_mut_ptr(), &ldu, v.as_mut_ptr(), &ldv, q, &ldq, iwork.as_mut_ptr(),
                      tau.as_mut_ptr(), work.as_mut_ptr(), &lwork, info)
    }
}

#[inline]
pub fn dggsvp3(jobu: u8, jobv: u8, jobq: u8, m: i32, p: i32, n: i32, a: &mut [f64], lda: i32,
               b: &mut [f64], ldb: i32, tola: f64, tolb: f64, k: &mut i32, l: &mut i32,
               u: &mut [f64], ldu: i32, v: &mut [f64], ldv: i32, q: &mut f64, ldq: i32,
               iwork: &mut [i32], tau: &mut [f64], work: &mut [f64], lwork: i32, info: &mut i32) {

    unsafe {
        ffi::dggsvp3_(&(jobu as c_char), &(jobv as c_char), &(jobq as c_char), &m, &p, &n,
                      a.as_mut_ptr(), &lda, b.as_mut_ptr(), &ldb, &tola, &tolb, k, l,
                      u.as_mut_ptr(), &ldu, v.as_mut_ptr(), &ldv, q, &ldq, iwork.as_mut_ptr(),
                      tau.as_mut_ptr(), work.as_mut_ptr(), &lwork, info)
    }
}

#[inline]
pub fn cggsvp3(jobu: u8, jobv: u8, jobq: u8, m: i32, p: i32, n: i32, a: &mut [c32], lda: i32,
               b: &mut [c32], ldb: i32, tola: f32, tolb: f32, k: &mut i32, l: &mut i32,
               u: &mut [c32], ldu: i32, v: &mut [c32], ldv: i32, q: &mut c32, ldq: i32,
               iwork: &mut [i32], rwork: &mut [f32], tau: &mut [c32], work: &mut [c32], lwork: i32,
               info: &mut i32) {

    unsafe {
        ffi::cggsvp3_(&(jobu as c_char), &(jobv as c_char), &(jobq as c_char), &m, &p, &n,
                      a.as_mut_ptr() as *mut _, &lda, b.as_mut_ptr() as *mut _, &ldb, &tola, &tolb,
                      k, l, u.as_mut_ptr() as *mut _, &ldu, v.as_mut_ptr() as *mut _, &ldv,
                      q as *mut _ as *mut _, &ldq, iwork.as_mut_ptr(), rwork.as_mut_ptr(),
                      tau.as_mut_ptr() as *mut _, work.as_mut_ptr() as *mut _, &lwork, info)
    }
}

#[inline]
pub fn zggsvp3(jobu: u8, jobv: u8, jobq: u8, m: i32, p: i32, n: i32, a: &mut [c64], lda: i32,
               b: &mut [c64], ldb: i32, tola: f64, tolb: f64, k: &mut i32, l: &mut i32,
               u: &mut [c64], ldu: i32, v: &mut [c64], ldv: i32, q: &mut c64, ldq: i32,
               iwork: &mut [i32], rwork: &mut [f64], tau: &mut [c64], work: &mut [c64], lwork: i32,
               info: &mut i32) {

    unsafe {
        ffi::zggsvp3_(&(jobu as c_char), &(jobv as c_char), &(jobq as c_char), &m, &p, &n,
                      a.as_mut_ptr() as *mut _, &lda, b.as_mut_ptr() as *mut _, &ldb, &tola, &tolb,
                      k, l, u.as_mut_ptr() as *mut _, &ldu, v.as_mut_ptr() as *mut _, &ldv,
                      q as *mut _ as *mut _, &ldq, iwork.as_mut_ptr(), rwork.as_mut_ptr(),
                      tau.as_mut_ptr() as *mut _, work.as_mut_ptr() as *mut _, &lwork, info)
    }
}

#[inline]
pub fn stgsja(jobu: u8, jobv: u8, jobq: u8, m: i32, p: i32, n: i32, k: i32, l: i32, a: &mut [f32],
              lda: i32, b: &mut [f32], ldb: i32, tola: f32, tolb: f32, alpha: &mut f32,
              beta: &mut f32, u: &mut [f32], ldu: i32, v: &mut [f32], ldv: i32, q: &mut f32,
              ldq: i32, work: &mut [f32], ncycle: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::stgsja_(&(jobu as c_char), &(jobv as c_char), &(jobq as c_char), &m, &p, &n, &k, &l,
                     a.as_mut_ptr(), &lda, b.as_mut_ptr(), &ldb, &tola, &tolb, alpha, beta,
                     u.as_mut_ptr(), &ldu, v.as_mut_ptr(), &ldv, q, &ldq, work.as_mut_ptr(),
                     ncycle.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dtgsja(jobu: u8, jobv: u8, jobq: u8, m: i32, p: i32, n: i32, k: i32, l: i32, a: &mut [f64],
              lda: i32, b: &mut [f64], ldb: i32, tola: f64, tolb: f64, alpha: &mut f64,
              beta: &mut f64, u: &mut [f64], ldu: i32, v: &mut [f64], ldv: i32, q: &mut f64,
              ldq: i32, work: &mut [f64], ncycle: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::dtgsja_(&(jobu as c_char), &(jobv as c_char), &(jobq as c_char), &m, &p, &n, &k, &l,
                     a.as_mut_ptr(), &lda, b.as_mut_ptr(), &ldb, &tola, &tolb, alpha, beta,
                     u.as_mut_ptr(), &ldu, v.as_mut_ptr(), &ldv, q, &ldq, work.as_mut_ptr(),
                     ncycle.as_mut_ptr(), info)
    }
}

#[inline]
pub fn ctgsja(jobu: u8, jobv: u8, jobq: u8, m: i32, p: i32, n: i32, k: i32, l: i32, a: &mut [c32],
              lda: i32, b: &mut [c32], ldb: i32, tola: f32, tolb: f32, alpha: &mut f32,
              beta: &mut f32, u: &mut [c32], ldu: i32, v: &mut [c32], ldv: i32, q: &mut c32,
              ldq: i32, work: &mut [c32], ncycle: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::ctgsja_(&(jobu as c_char), &(jobv as c_char), &(jobq as c_char), &m, &p, &n, &k, &l,
                     a.as_mut_ptr() as *mut _, &lda, b.as_mut_ptr() as *mut _, &ldb, &tola, &tolb,
                     alpha, beta, u.as_mut_ptr() as *mut _, &ldu, v.as_mut_ptr() as *mut _, &ldv,
                     q as *mut _ as *mut _, &ldq, work.as_mut_ptr() as *mut _, ncycle.as_mut_ptr(),
                     info)
    }
}

#[inline]
pub fn ztgsja(jobu: u8, jobv: u8, jobq: u8, m: i32, p: i32, n: i32, k: i32, l: i32, a: &mut [c64],
              lda: i32, b: &mut [c64], ldb: i32, tola: f64, tolb: f64, alpha: &mut f64,
              beta: &mut f64, u: &mut [c64], ldu: i32, v: &mut [c64], ldv: i32, q: &mut c64,
              ldq: i32, work: &mut [c64], ncycle: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::ztgsja_(&(jobu as c_char), &(jobv as c_char), &(jobq as c_char), &m, &p, &n, &k, &l,
                     a.as_mut_ptr() as *mut _, &lda, b.as_mut_ptr() as *mut _, &ldb, &tola, &tolb,
                     alpha, beta, u.as_mut_ptr() as *mut _, &ldu, v.as_mut_ptr() as *mut _, &ldv,
                     q as *mut _ as *mut _, &ldq, work.as_mut_ptr() as *mut _, ncycle.as_mut_ptr(),
                     info)
    }
}

#[inline]
pub fn sgels(trans: u8, m: i32, n: i32, nrhs: i32, a: &mut [f32], lda: i32, b: &mut [f32],
             ldb: i32, work: &mut [f32], lwork: i32, info: &mut i32) {

    unsafe {
        ffi::sgels_(&(trans as c_char), &m, &n, &nrhs, a.as_mut_ptr(), &lda, b.as_mut_ptr(), &ldb,
                    work.as_mut_ptr(), &lwork, info)
    }
}

#[inline]
pub fn dgels(trans: u8, m: i32, n: i32, nrhs: i32, a: &mut [f64], lda: i32, b: &mut [f64],
             ldb: i32, work: &mut [f64], lwork: i32, info: &mut i32) {

    unsafe {
        ffi::dgels_(&(trans as c_char), &m, &n, &nrhs, a.as_mut_ptr(), &lda, b.as_mut_ptr(), &ldb,
                    work.as_mut_ptr(), &lwork, info)
    }
}

#[inline]
pub fn cgels(trans: u8, m: i32, n: i32, nrhs: i32, a: &mut [c32], lda: i32, b: &mut [c32],
             ldb: i32, work: &mut [c32], lwork: i32, info: &mut i32) {

    unsafe {
        ffi::cgels_(&(trans as c_char), &m, &n, &nrhs, a.as_mut_ptr() as *mut _, &lda,
                    b.as_mut_ptr() as *mut _, &ldb, work.as_mut_ptr() as *mut _, &lwork, info)
    }
}

#[inline]
pub fn zgels(trans: u8, m: i32, n: i32, nrhs: i32, a: &mut [c64], lda: i32, b: &mut [c64],
             ldb: i32, work: &mut [c64], lwork: i32, info: &mut i32) {

    unsafe {
        ffi::zgels_(&(trans as c_char), &m, &n, &nrhs, a.as_mut_ptr() as *mut _, &lda,
                    b.as_mut_ptr() as *mut _, &ldb, work.as_mut_ptr() as *mut _, &lwork, info)
    }
}

#[inline]
pub fn sgelsy(m: i32, n: i32, nrhs: i32, a: &mut [f32], lda: i32, b: &mut [f32], ldb: i32,
              jpvt: &mut [i32], rcond: f32, rank: &mut i32, work: &mut [f32], lwork: i32,
              info: &mut i32) {

    unsafe {
        ffi::sgelsy_(&m, &n, &nrhs, a.as_mut_ptr(), &lda, b.as_mut_ptr(), &ldb, jpvt.as_mut_ptr(),
                     &rcond, rank, work.as_mut_ptr(), &lwork, info)
    }
}

#[inline]
pub fn dgelsy(m: i32, n: i32, nrhs: i32, a: &mut [f64], lda: i32, b: &mut [f64], ldb: i32,
              jpvt: &mut [i32], rcond: f64, rank: &mut i32, work: &mut [f64], lwork: i32,
              info: &mut i32) {

    unsafe {
        ffi::dgelsy_(&m, &n, &nrhs, a.as_mut_ptr(), &lda, b.as_mut_ptr(), &ldb, jpvt.as_mut_ptr(),
                     &rcond, rank, work.as_mut_ptr(), &lwork, info)
    }
}

#[inline]
pub fn cgelsy(m: i32, n: i32, nrhs: i32, a: &mut [c32], lda: i32, b: &mut [c32], ldb: i32,
              jpvt: &mut [i32], rcond: f32, rank: &mut i32, work: &mut [c32], lwork: i32,
              rwork: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::cgelsy_(&m, &n, &nrhs, a.as_mut_ptr() as *mut _, &lda, b.as_mut_ptr() as *mut _, &ldb,
                     jpvt.as_mut_ptr(), &rcond, rank, work.as_mut_ptr() as *mut _, &lwork,
                     rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zgelsy(m: i32, n: i32, nrhs: i32, a: &mut [c64], lda: i32, b: &mut [c64], ldb: i32,
              jpvt: &mut [i32], rcond: f64, rank: &mut i32, work: &mut [c64], lwork: i32,
              rwork: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::zgelsy_(&m, &n, &nrhs, a.as_mut_ptr() as *mut _, &lda, b.as_mut_ptr() as *mut _, &ldb,
                     jpvt.as_mut_ptr(), &rcond, rank, work.as_mut_ptr() as *mut _, &lwork,
                     rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn sgelss(m: i32, n: i32, nrhs: i32, a: &mut [f32], lda: i32, b: &mut [f32], ldb: i32,
              s: &mut [f32], rcond: f32, rank: &mut i32, work: &mut [f32], lwork: i32,
              info: &mut i32) {

    unsafe {
        ffi::sgelss_(&m, &n, &nrhs, a.as_mut_ptr(), &lda, b.as_mut_ptr(), &ldb, s.as_mut_ptr(),
                     &rcond, rank, work.as_mut_ptr(), &lwork, info)
    }
}

#[inline]
pub fn dgelss(m: i32, n: i32, nrhs: i32, a: &mut [f64], lda: i32, b: &mut [f64], ldb: i32,
              s: &mut [f64], rcond: f64, rank: &mut i32, work: &mut [f64], lwork: i32,
              info: &mut i32) {

    unsafe {
        ffi::dgelss_(&m, &n, &nrhs, a.as_mut_ptr(), &lda, b.as_mut_ptr(), &ldb, s.as_mut_ptr(),
                     &rcond, rank, work.as_mut_ptr(), &lwork, info)
    }
}

#[inline]
pub fn cgelss(m: i32, n: i32, nrhs: i32, a: &mut [c32], lda: i32, b: &mut [c32], ldb: i32,
              s: &mut [f32], rcond: f32, rank: &mut i32, work: &mut [c32], lwork: i32,
              rwork: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::cgelss_(&m, &n, &nrhs, a.as_mut_ptr() as *mut _, &lda, b.as_mut_ptr() as *mut _, &ldb,
                     s.as_mut_ptr(), &rcond, rank, work.as_mut_ptr() as *mut _, &lwork,
                     rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zgelss(m: i32, n: i32, nrhs: i32, a: &mut [c64], lda: i32, b: &mut [c64], ldb: i32,
              s: &mut [f64], rcond: f64, rank: &mut i32, work: &mut [c64], lwork: i32,
              rwork: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::zgelss_(&m, &n, &nrhs, a.as_mut_ptr() as *mut _, &lda, b.as_mut_ptr() as *mut _, &ldb,
                     s.as_mut_ptr(), &rcond, rank, work.as_mut_ptr() as *mut _, &lwork,
                     rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn sgelsd(m: i32, n: i32, nrhs: i32, a: &[f32], lda: i32, b: &mut [f32], ldb: i32,
              s: &mut [f32], rcond: f32, rank: &mut i32, work: &mut [f32], lwork: i32,
              iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::sgelsd_(&m, &n, &nrhs, a.as_ptr(), &lda, b.as_mut_ptr(), &ldb, s.as_mut_ptr(), &rcond,
                     rank, work.as_mut_ptr(), &lwork, iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dgelsd(m: i32, n: i32, nrhs: i32, a: &[f64], lda: i32, b: &mut [f64], ldb: i32,
              s: &mut [f64], rcond: f64, rank: &mut i32, work: &mut [f64], lwork: i32,
              iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::dgelsd_(&m, &n, &nrhs, a.as_ptr(), &lda, b.as_mut_ptr(), &ldb, s.as_mut_ptr(), &rcond,
                     rank, work.as_mut_ptr(), &lwork, iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn cgelsd(m: i32, n: i32, nrhs: i32, a: &[c32], lda: i32, b: &mut [c32], ldb: i32,
              s: &mut [f32], rcond: f32, rank: &mut i32, work: &mut [c32], lwork: i32,
              rwork: &mut [f32], iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::cgelsd_(&m, &n, &nrhs, a.as_ptr() as *const _, &lda, b.as_mut_ptr() as *mut _, &ldb,
                     s.as_mut_ptr(), &rcond, rank, work.as_mut_ptr() as *mut _, &lwork,
                     rwork.as_mut_ptr(), iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zgelsd(m: i32, n: i32, nrhs: i32, a: &[c64], lda: i32, b: &mut [c64], ldb: i32,
              s: &mut [f64], rcond: f64, rank: &mut i32, work: &mut [c64], lwork: i32,
              rwork: &mut [f64], iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::zgelsd_(&m, &n, &nrhs, a.as_ptr() as *const _, &lda, b.as_mut_ptr() as *mut _, &ldb,
                     s.as_mut_ptr(), &rcond, rank, work.as_mut_ptr() as *mut _, &lwork,
                     rwork.as_mut_ptr(), iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn sgglse(m: i32, n: i32, p: i32, a: &mut [f32], lda: i32, b: &mut [f32], ldb: i32,
              c: &mut [f32], d: &mut [f32], x: &mut [f32], work: &mut [f32], lwork: i32,
              info: &mut i32) {

    unsafe {
        ffi::sgglse_(&m, &n, &p, a.as_mut_ptr(), &lda, b.as_mut_ptr(), &ldb, c.as_mut_ptr(),
                     d.as_mut_ptr(), x.as_mut_ptr(), work.as_mut_ptr(), &lwork, info)
    }
}

#[inline]
pub fn dgglse(m: i32, n: i32, p: i32, a: &mut [f64], lda: i32, b: &mut [f64], ldb: i32,
              c: &mut [f64], d: &mut [f64], x: &mut [f64], work: &mut [f64], lwork: i32,
              info: &mut i32) {

    unsafe {
        ffi::dgglse_(&m, &n, &p, a.as_mut_ptr(), &lda, b.as_mut_ptr(), &ldb, c.as_mut_ptr(),
                     d.as_mut_ptr(), x.as_mut_ptr(), work.as_mut_ptr(), &lwork, info)
    }
}

#[inline]
pub fn cgglse(m: i32, n: i32, p: i32, a: &mut [c32], lda: i32, b: &mut [c32], ldb: i32,
              c: &mut [c32], d: &mut [c32], x: &mut [c32], work: &mut [c32], lwork: i32,
              info: &mut i32) {

    unsafe {
        ffi::cgglse_(&m, &n, &p, a.as_mut_ptr() as *mut _, &lda, b.as_mut_ptr() as *mut _, &ldb,
                     c.as_mut_ptr() as *mut _, d.as_mut_ptr() as *mut _, x.as_mut_ptr() as *mut _,
                     work.as_mut_ptr() as *mut _, &lwork, info)
    }
}

#[inline]
pub fn zgglse(m: i32, n: i32, p: i32, a: &mut [c64], lda: i32, b: &mut [c64], ldb: i32,
              c: &mut [c64], d: &mut [c64], x: &mut [c64], work: &mut [c64], lwork: i32,
              info: &mut i32) {

    unsafe {
        ffi::zgglse_(&m, &n, &p, a.as_mut_ptr() as *mut _, &lda, b.as_mut_ptr() as *mut _, &ldb,
                     c.as_mut_ptr() as *mut _, d.as_mut_ptr() as *mut _, x.as_mut_ptr() as *mut _,
                     work.as_mut_ptr() as *mut _, &lwork, info)
    }
}

#[inline]
pub fn sggglm(n: i32, m: i32, p: i32, a: &mut [f32], lda: i32, b: &mut [f32], ldb: i32,
              d: &mut [f32], x: &mut [f32], y: &mut [f32], work: &mut [f32], lwork: i32,
              info: &mut i32) {

    unsafe {
        ffi::sggglm_(&n, &m, &p, a.as_mut_ptr(), &lda, b.as_mut_ptr(), &ldb, d.as_mut_ptr(),
                     x.as_mut_ptr(), y.as_mut_ptr(), work.as_mut_ptr(), &lwork, info)
    }
}

#[inline]
pub fn dggglm(n: i32, m: i32, p: i32, a: &mut [f64], lda: i32, b: &mut [f64], ldb: i32,
              d: &mut [f64], x: &mut [f64], y: &mut [f64], work: &mut [f64], lwork: i32,
              info: &mut i32) {

    unsafe {
        ffi::dggglm_(&n, &m, &p, a.as_mut_ptr(), &lda, b.as_mut_ptr(), &ldb, d.as_mut_ptr(),
                     x.as_mut_ptr(), y.as_mut_ptr(), work.as_mut_ptr(), &lwork, info)
    }
}

#[inline]
pub fn cggglm(n: i32, m: i32, p: i32, a: &mut [c32], lda: i32, b: &mut [c32], ldb: i32,
              d: &mut [c32], x: &mut [c32], y: &mut [c32], work: &mut [c32], lwork: i32,
              info: &mut i32) {

    unsafe {
        ffi::cggglm_(&n, &m, &p, a.as_mut_ptr() as *mut _, &lda, b.as_mut_ptr() as *mut _, &ldb,
                     d.as_mut_ptr() as *mut _, x.as_mut_ptr() as *mut _, y.as_mut_ptr() as *mut _,
                     work.as_mut_ptr() as *mut _, &lwork, info)
    }
}

#[inline]
pub fn zggglm(n: i32, m: i32, p: i32, a: &mut [c64], lda: i32, b: &mut [c64], ldb: i32,
              d: &mut [c64], x: &mut [c64], y: &mut [c64], work: &mut [c64], lwork: i32,
              info: &mut i32) {

    unsafe {
        ffi::zggglm_(&n, &m, &p, a.as_mut_ptr() as *mut _, &lda, b.as_mut_ptr() as *mut _, &ldb,
                     d.as_mut_ptr() as *mut _, x.as_mut_ptr() as *mut _, y.as_mut_ptr() as *mut _,
                     work.as_mut_ptr() as *mut _, &lwork, info)
    }
}

#[inline]
pub fn ssyev(jobz: u8, uplo: u8, n: i32, a: &mut [f32], lda: i32, w: &mut [f32], work: &mut [f32],
             lwork: i32, info: &mut i32) {

    unsafe {
        ffi::ssyev_(&(jobz as c_char), &(uplo as c_char), &n, a.as_mut_ptr(), &lda, w.as_mut_ptr(),
                    work.as_mut_ptr(), &lwork, info)
    }
}

#[inline]
pub fn dsyev(jobz: u8, uplo: u8, n: i32, a: &mut [f64], lda: i32, w: &mut [f64], work: &mut [f64],
             lwork: i32, info: &mut i32) {

    unsafe {
        ffi::dsyev_(&(jobz as c_char), &(uplo as c_char), &n, a.as_mut_ptr(), &lda, w.as_mut_ptr(),
                    work.as_mut_ptr(), &lwork, info)
    }
}

#[inline]
pub fn cheev(jobz: u8, uplo: u8, n: i32, a: &mut [c32], lda: i32, w: &mut [f32], work: &mut [c32],
             lwork: i32, rwork: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::cheev_(&(jobz as c_char), &(uplo as c_char), &n, a.as_mut_ptr() as *mut _, &lda,
                    w.as_mut_ptr(), work.as_mut_ptr() as *mut _, &lwork, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zheev(jobz: u8, uplo: u8, n: i32, a: &mut [c64], lda: i32, w: &mut [f64], work: &mut [c64],
             lwork: i32, rwork: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::zheev_(&(jobz as c_char), &(uplo as c_char), &n, a.as_mut_ptr() as *mut _, &lda,
                    w.as_mut_ptr(), work.as_mut_ptr() as *mut _, &lwork, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn ssyevd(jobz: u8, uplo: u8, n: i32, a: &mut [f32], lda: i32, w: &mut [f32], work: &mut [f32],
              lwork: i32, iwork: &mut [i32], liwork: i32, info: &mut i32) {

    unsafe {
        ffi::ssyevd_(&(jobz as c_char), &(uplo as c_char), &n, a.as_mut_ptr(), &lda,
                     w.as_mut_ptr(), work.as_mut_ptr(), &lwork, iwork.as_mut_ptr(), &liwork, info)
    }
}

#[inline]
pub fn dsyevd(jobz: u8, uplo: u8, n: i32, a: &mut [f64], lda: i32, w: &mut [f64], work: &mut [f64],
              lwork: i32, iwork: &mut [i32], liwork: i32, info: &mut i32) {

    unsafe {
        ffi::dsyevd_(&(jobz as c_char), &(uplo as c_char), &n, a.as_mut_ptr(), &lda,
                     w.as_mut_ptr(), work.as_mut_ptr(), &lwork, iwork.as_mut_ptr(), &liwork, info)
    }
}

#[inline]
pub fn cheevd(jobz: u8, uplo: u8, n: i32, a: &mut [c32], lda: i32, w: &mut [f32], work: &mut [c32],
              lwork: i32, rwork: &mut [f32], lrwork: i32, iwork: &mut [i32], liwork: i32,
              info: &mut i32) {

    unsafe {
        ffi::cheevd_(&(jobz as c_char), &(uplo as c_char), &n, a.as_mut_ptr() as *mut _, &lda,
                     w.as_mut_ptr(), work.as_mut_ptr() as *mut _, &lwork, rwork.as_mut_ptr(),
                     &lrwork, iwork.as_mut_ptr(), &liwork, info)
    }
}

#[inline]
pub fn zheevd(jobz: u8, uplo: u8, n: i32, a: &mut [c64], lda: i32, w: &mut [f64], work: &mut [c64],
              lwork: i32, rwork: &mut [f64], lrwork: i32, iwork: &mut [i32], liwork: i32,
              info: &mut i32) {

    unsafe {
        ffi::zheevd_(&(jobz as c_char), &(uplo as c_char), &n, a.as_mut_ptr() as *mut _, &lda,
                     w.as_mut_ptr(), work.as_mut_ptr() as *mut _, &lwork, rwork.as_mut_ptr(),
                     &lrwork, iwork.as_mut_ptr(), &liwork, info)
    }
}

#[inline]
pub fn ssyevx(jobz: u8, range: u8, uplo: u8, n: i32, a: &mut [f32], lda: i32, vl: f32, vu: f32,
              il: i32, iu: i32, abstol: f32, m: &mut i32, w: &mut [f32], z: &mut [f32], ldz: i32,
              work: &mut [f32], lwork: i32, iwork: &mut [i32], ifail: &mut i32, info: &mut i32) {

    unsafe {
        ffi::ssyevx_(&(jobz as c_char), &(range as c_char), &(uplo as c_char), &n, a.as_mut_ptr(),
                     &lda, &vl, &vu, &il, &iu, &abstol, m, w.as_mut_ptr(), z.as_mut_ptr(), &ldz,
                     work.as_mut_ptr(), &lwork, iwork.as_mut_ptr(), ifail, info)
    }
}

#[inline]
pub fn dsyevx(jobz: u8, range: u8, uplo: u8, n: i32, a: &mut [f64], lda: i32, vl: f64, vu: f64,
              il: i32, iu: i32, abstol: f64, m: &mut i32, w: &mut [f64], z: &mut [f64], ldz: i32,
              work: &mut [f64], lwork: i32, iwork: &mut [i32], ifail: &mut i32, info: &mut i32) {

    unsafe {
        ffi::dsyevx_(&(jobz as c_char), &(range as c_char), &(uplo as c_char), &n, a.as_mut_ptr(),
                     &lda, &vl, &vu, &il, &iu, &abstol, m, w.as_mut_ptr(), z.as_mut_ptr(), &ldz,
                     work.as_mut_ptr(), &lwork, iwork.as_mut_ptr(), ifail, info)
    }
}

#[inline]
pub fn cheevx(jobz: u8, range: u8, uplo: u8, n: i32, a: &mut [c32], lda: i32, vl: f32, vu: f32,
              il: i32, iu: i32, abstol: f32, m: &mut i32, w: &mut [f32], z: &mut [c32], ldz: i32,
              work: &mut [c32], lwork: i32, rwork: &mut [f32], iwork: &mut [i32], ifail: &mut i32,
              info: &mut i32) {

    unsafe {
        ffi::cheevx_(&(jobz as c_char), &(range as c_char), &(uplo as c_char), &n,
                     a.as_mut_ptr() as *mut _, &lda, &vl, &vu, &il, &iu, &abstol, m,
                     w.as_mut_ptr(), z.as_mut_ptr() as *mut _, &ldz, work.as_mut_ptr() as *mut _,
                     &lwork, rwork.as_mut_ptr(), iwork.as_mut_ptr(), ifail, info)
    }
}

#[inline]
pub fn zheevx(jobz: u8, range: u8, uplo: u8, n: i32, a: &mut [c64], lda: i32, vl: f64, vu: f64,
              il: i32, iu: i32, abstol: f64, m: &mut i32, w: &mut [f64], z: &mut [c64], ldz: i32,
              work: &mut [c64], lwork: i32, rwork: &mut [f64], iwork: &mut [i32], ifail: &mut i32,
              info: &mut i32) {

    unsafe {
        ffi::zheevx_(&(jobz as c_char), &(range as c_char), &(uplo as c_char), &n,
                     a.as_mut_ptr() as *mut _, &lda, &vl, &vu, &il, &iu, &abstol, m,
                     w.as_mut_ptr(), z.as_mut_ptr() as *mut _, &ldz, work.as_mut_ptr() as *mut _,
                     &lwork, rwork.as_mut_ptr(), iwork.as_mut_ptr(), ifail, info)
    }
}

#[inline]
pub fn ssyevr(jobz: u8, range: u8, uplo: u8, n: i32, a: &mut [f32], lda: i32, vl: f32, vu: f32,
              il: i32, iu: i32, abstol: f32, m: &mut i32, w: &mut [f32], z: &mut [f32], ldz: i32,
              isuppz: &mut [i32], work: &mut [f32], lwork: i32, iwork: &mut [i32], liwork: i32,
              info: &mut i32) {

    unsafe {
        ffi::ssyevr_(&(jobz as c_char), &(range as c_char), &(uplo as c_char), &n, a.as_mut_ptr(),
                     &lda, &vl, &vu, &il, &iu, &abstol, m, w.as_mut_ptr(), z.as_mut_ptr(), &ldz,
                     isuppz.as_mut_ptr(), work.as_mut_ptr(), &lwork, iwork.as_mut_ptr(), &liwork,
                     info)
    }
}

#[inline]
pub fn dsyevr(jobz: u8, range: u8, uplo: u8, n: i32, a: &mut [f64], lda: i32, vl: f64, vu: f64,
              il: i32, iu: i32, abstol: f64, m: &mut i32, w: &mut [f64], z: &mut [f64], ldz: i32,
              isuppz: &mut [i32], work: &mut [f64], lwork: i32, iwork: &mut [i32], liwork: i32,
              info: &mut i32) {

    unsafe {
        ffi::dsyevr_(&(jobz as c_char), &(range as c_char), &(uplo as c_char), &n, a.as_mut_ptr(),
                     &lda, &vl, &vu, &il, &iu, &abstol, m, w.as_mut_ptr(), z.as_mut_ptr(), &ldz,
                     isuppz.as_mut_ptr(), work.as_mut_ptr(), &lwork, iwork.as_mut_ptr(), &liwork,
                     info)
    }
}

#[inline]
pub fn cheevr(jobz: u8, range: u8, uplo: u8, n: i32, a: &mut [c32], lda: i32, vl: f32, vu: f32,
              il: i32, iu: i32, abstol: f32, m: &mut i32, w: &mut [f32], z: &mut [c32], ldz: i32,
              isuppz: &mut [i32], work: &mut [c32], lwork: i32, rwork: &mut [f32], lrwork: i32,
              iwork: &mut [i32], liwork: i32, info: &mut i32) {

    unsafe {
        ffi::cheevr_(&(jobz as c_char), &(range as c_char), &(uplo as c_char), &n,
                     a.as_mut_ptr() as *mut _, &lda, &vl, &vu, &il, &iu, &abstol, m,
                     w.as_mut_ptr(), z.as_mut_ptr() as *mut _, &ldz, isuppz.as_mut_ptr(),
                     work.as_mut_ptr() as *mut _, &lwork, rwork.as_mut_ptr(), &lrwork,
                     iwork.as_mut_ptr(), &liwork, info)
    }
}

#[inline]
pub fn zheevr(jobz: u8, range: u8, uplo: u8, n: i32, a: &mut [c64], lda: i32, vl: f64, vu: f64,
              il: i32, iu: i32, abstol: f64, m: &mut i32, w: &mut [f64], z: &mut [c64], ldz: i32,
              isuppz: &mut [i32], work: &mut [c64], lwork: i32, rwork: &mut [f64], lrwork: i32,
              iwork: &mut [i32], liwork: i32, info: &mut i32) {

    unsafe {
        ffi::zheevr_(&(jobz as c_char), &(range as c_char), &(uplo as c_char), &n,
                     a.as_mut_ptr() as *mut _, &lda, &vl, &vu, &il, &iu, &abstol, m,
                     w.as_mut_ptr(), z.as_mut_ptr() as *mut _, &ldz, isuppz.as_mut_ptr(),
                     work.as_mut_ptr() as *mut _, &lwork, rwork.as_mut_ptr(), &lrwork,
                     iwork.as_mut_ptr(), &liwork, info)
    }
}

#[inline]
pub fn sspev(jobz: u8, uplo: u8, n: i32, ap: &mut [f32], w: &mut [f32], z: &mut [f32], ldz: i32,
             work: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::sspev_(&(jobz as c_char), &(uplo as c_char), &n, ap.as_mut_ptr(), w.as_mut_ptr(),
                    z.as_mut_ptr(), &ldz, work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dspev(jobz: u8, uplo: u8, n: i32, ap: &mut [f64], w: &mut [f64], z: &mut [f64], ldz: i32,
             work: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::dspev_(&(jobz as c_char), &(uplo as c_char), &n, ap.as_mut_ptr(), w.as_mut_ptr(),
                    z.as_mut_ptr(), &ldz, work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn chpev(jobz: u8, uplo: u8, n: i32, ap: &mut [c32], w: &mut [f32], z: &mut [c32], ldz: i32,
             work: &mut [c32], rwork: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::chpev_(&(jobz as c_char), &(uplo as c_char), &n, ap.as_mut_ptr() as *mut _,
                    w.as_mut_ptr(), z.as_mut_ptr() as *mut _, &ldz, work.as_mut_ptr() as *mut _,
                    rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zhpev(jobz: u8, uplo: u8, n: i32, ap: &mut [c64], w: &mut [f64], z: &mut [c64], ldz: i32,
             work: &mut [c64], rwork: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::zhpev_(&(jobz as c_char), &(uplo as c_char), &n, ap.as_mut_ptr() as *mut _,
                    w.as_mut_ptr(), z.as_mut_ptr() as *mut _, &ldz, work.as_mut_ptr() as *mut _,
                    rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn sspevd(jobz: u8, uplo: u8, n: i32, ap: &mut [f32], w: &mut [f32], z: &mut [f32], ldz: i32,
              work: &mut [f32], lwork: i32, iwork: &mut [i32], liwork: i32, info: &mut i32) {

    unsafe {
        ffi::sspevd_(&(jobz as c_char), &(uplo as c_char), &n, ap.as_mut_ptr(), w.as_mut_ptr(),
                     z.as_mut_ptr(), &ldz, work.as_mut_ptr(), &lwork, iwork.as_mut_ptr(), &liwork,
                     info)
    }
}

#[inline]
pub fn dspevd(jobz: u8, uplo: u8, n: i32, ap: &mut [f64], w: &mut [f64], z: &mut [f64], ldz: i32,
              work: &mut [f64], lwork: i32, iwork: &mut [i32], liwork: i32, info: &mut i32) {

    unsafe {
        ffi::dspevd_(&(jobz as c_char), &(uplo as c_char), &n, ap.as_mut_ptr(), w.as_mut_ptr(),
                     z.as_mut_ptr(), &ldz, work.as_mut_ptr(), &lwork, iwork.as_mut_ptr(), &liwork,
                     info)
    }
}

#[inline]
pub fn chpevd(jobz: u8, uplo: u8, n: i32, ap: &mut [c32], w: &mut [f32], z: &mut [c32], ldz: i32,
              work: &mut [c32], lwork: i32, rwork: &mut [f32], lrwork: i32, iwork: &mut [i32],
              liwork: i32, info: &mut i32) {

    unsafe {
        ffi::chpevd_(&(jobz as c_char), &(uplo as c_char), &n, ap.as_mut_ptr() as *mut _,
                     w.as_mut_ptr(), z.as_mut_ptr() as *mut _, &ldz, work.as_mut_ptr() as *mut _,
                     &lwork, rwork.as_mut_ptr(), &lrwork, iwork.as_mut_ptr(), &liwork, info)
    }
}

#[inline]
pub fn zhpevd(jobz: u8, uplo: u8, n: i32, ap: &mut [c64], w: &mut [f64], z: &mut [c64], ldz: i32,
              work: &mut [c64], lwork: i32, rwork: &mut [f64], lrwork: i32, iwork: &mut [i32],
              liwork: i32, info: &mut i32) {

    unsafe {
        ffi::zhpevd_(&(jobz as c_char), &(uplo as c_char), &n, ap.as_mut_ptr() as *mut _,
                     w.as_mut_ptr(), z.as_mut_ptr() as *mut _, &ldz, work.as_mut_ptr() as *mut _,
                     &lwork, rwork.as_mut_ptr(), &lrwork, iwork.as_mut_ptr(), &liwork, info)
    }
}

#[inline]
pub fn sspevx(jobz: u8, range: u8, uplo: u8, n: i32, ap: &mut [f32], vl: f32, vu: f32, il: i32,
              iu: i32, abstol: f32, m: &mut i32, w: &mut [f32], z: &mut [f32], ldz: i32,
              work: &mut [f32], iwork: &mut [i32], ifail: &mut i32, info: &mut i32) {

    unsafe {
        ffi::sspevx_(&(jobz as c_char), &(range as c_char), &(uplo as c_char), &n, ap.as_mut_ptr(),
                     &vl, &vu, &il, &iu, &abstol, m, w.as_mut_ptr(), z.as_mut_ptr(), &ldz,
                     work.as_mut_ptr(), iwork.as_mut_ptr(), ifail, info)
    }
}

#[inline]
pub fn dspevx(jobz: u8, range: u8, uplo: u8, n: i32, ap: &mut [f64], vl: f64, vu: f64, il: i32,
              iu: i32, abstol: f64, m: &mut i32, w: &mut [f64], z: &mut [f64], ldz: i32,
              work: &mut [f64], iwork: &mut [i32], ifail: &mut i32, info: &mut i32) {

    unsafe {
        ffi::dspevx_(&(jobz as c_char), &(range as c_char), &(uplo as c_char), &n, ap.as_mut_ptr(),
                     &vl, &vu, &il, &iu, &abstol, m, w.as_mut_ptr(), z.as_mut_ptr(), &ldz,
                     work.as_mut_ptr(), iwork.as_mut_ptr(), ifail, info)
    }
}

#[inline]
pub fn chpevx(jobz: u8, range: u8, uplo: u8, n: i32, ap: &mut [c32], vl: f32, vu: f32, il: i32,
              iu: i32, abstol: f32, m: &mut i32, w: &mut [f32], z: &mut [c32], ldz: i32,
              work: &mut [c32], rwork: &mut [f32], iwork: &mut [i32], ifail: &mut i32,
              info: &mut i32) {

    unsafe {
        ffi::chpevx_(&(jobz as c_char), &(range as c_char), &(uplo as c_char), &n,
                     ap.as_mut_ptr() as *mut _, &vl, &vu, &il, &iu, &abstol, m, w.as_mut_ptr(),
                     z.as_mut_ptr() as *mut _, &ldz, work.as_mut_ptr() as *mut _,
                     rwork.as_mut_ptr(), iwork.as_mut_ptr(), ifail, info)
    }
}

#[inline]
pub fn zhpevx(jobz: u8, range: u8, uplo: u8, n: i32, ap: &mut [c64], vl: f64, vu: f64, il: i32,
              iu: i32, abstol: f64, m: &mut i32, w: &mut [f64], z: &mut [c64], ldz: i32,
              work: &mut [c64], rwork: &mut [f64], iwork: &mut [i32], ifail: &mut i32,
              info: &mut i32) {

    unsafe {
        ffi::zhpevx_(&(jobz as c_char), &(range as c_char), &(uplo as c_char), &n,
                     ap.as_mut_ptr() as *mut _, &vl, &vu, &il, &iu, &abstol, m, w.as_mut_ptr(),
                     z.as_mut_ptr() as *mut _, &ldz, work.as_mut_ptr() as *mut _,
                     rwork.as_mut_ptr(), iwork.as_mut_ptr(), ifail, info)
    }
}

#[inline]
pub fn ssbev(jobz: u8, uplo: u8, n: i32, kd: i32, ab: &mut [f32], ldab: i32, w: &mut [f32],
             z: &mut [f32], ldz: i32, work: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::ssbev_(&(jobz as c_char), &(uplo as c_char), &n, &kd, ab.as_mut_ptr(), &ldab,
                    w.as_mut_ptr(), z.as_mut_ptr(), &ldz, work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dsbev(jobz: u8, uplo: u8, n: i32, kd: i32, ab: &mut [f64], ldab: i32, w: &mut [f64],
             z: &mut [f64], ldz: i32, work: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::dsbev_(&(jobz as c_char), &(uplo as c_char), &n, &kd, ab.as_mut_ptr(), &ldab,
                    w.as_mut_ptr(), z.as_mut_ptr(), &ldz, work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn chbev(jobz: u8, uplo: u8, n: i32, kd: i32, ab: &mut [c32], ldab: i32, w: &mut [f32],
             z: &mut [c32], ldz: i32, work: &mut [c32], rwork: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::chbev_(&(jobz as c_char), &(uplo as c_char), &n, &kd, ab.as_mut_ptr() as *mut _,
                    &ldab, w.as_mut_ptr(), z.as_mut_ptr() as *mut _, &ldz,
                    work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zhbev(jobz: u8, uplo: u8, n: i32, kd: i32, ab: &mut [c64], ldab: i32, w: &mut [f64],
             z: &mut [c64], ldz: i32, work: &mut [c64], rwork: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::zhbev_(&(jobz as c_char), &(uplo as c_char), &n, &kd, ab.as_mut_ptr() as *mut _,
                    &ldab, w.as_mut_ptr(), z.as_mut_ptr() as *mut _, &ldz,
                    work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn ssbevd(jobz: u8, uplo: u8, n: i32, kd: i32, ab: &mut [f32], ldab: i32, w: &mut [f32],
              z: &mut [f32], ldz: i32, work: &mut [f32], lwork: i32, iwork: &mut [i32],
              liwork: i32, info: &mut i32) {

    unsafe {
        ffi::ssbevd_(&(jobz as c_char), &(uplo as c_char), &n, &kd, ab.as_mut_ptr(), &ldab,
                     w.as_mut_ptr(), z.as_mut_ptr(), &ldz, work.as_mut_ptr(), &lwork,
                     iwork.as_mut_ptr(), &liwork, info)
    }
}

#[inline]
pub fn dsbevd(jobz: u8, uplo: u8, n: i32, kd: i32, ab: &mut [f64], ldab: i32, w: &mut [f64],
              z: &mut [f64], ldz: i32, work: &mut [f64], lwork: i32, iwork: &mut [i32],
              liwork: i32, info: &mut i32) {

    unsafe {
        ffi::dsbevd_(&(jobz as c_char), &(uplo as c_char), &n, &kd, ab.as_mut_ptr(), &ldab,
                     w.as_mut_ptr(), z.as_mut_ptr(), &ldz, work.as_mut_ptr(), &lwork,
                     iwork.as_mut_ptr(), &liwork, info)
    }
}

#[inline]
pub fn chbevd(jobz: u8, uplo: u8, n: i32, kd: i32, ab: &mut [c32], ldab: i32, w: &mut [f32],
              z: &mut [c32], ldz: i32, work: &mut [c32], lwork: i32, rwork: &mut [f32],
              lrwork: i32, iwork: &mut [i32], liwork: i32, info: &mut i32) {

    unsafe {
        ffi::chbevd_(&(jobz as c_char), &(uplo as c_char), &n, &kd, ab.as_mut_ptr() as *mut _,
                     &ldab, w.as_mut_ptr(), z.as_mut_ptr() as *mut _, &ldz,
                     work.as_mut_ptr() as *mut _, &lwork, rwork.as_mut_ptr(), &lrwork,
                     iwork.as_mut_ptr(), &liwork, info)
    }
}

#[inline]
pub fn zhbevd(jobz: u8, uplo: u8, n: i32, kd: i32, ab: &mut [c64], ldab: i32, w: &mut [f64],
              z: &mut [c64], ldz: i32, work: &mut [c64], lwork: i32, rwork: &mut [f64],
              lrwork: i32, iwork: &mut [i32], liwork: i32, info: &mut i32) {

    unsafe {
        ffi::zhbevd_(&(jobz as c_char), &(uplo as c_char), &n, &kd, ab.as_mut_ptr() as *mut _,
                     &ldab, w.as_mut_ptr(), z.as_mut_ptr() as *mut _, &ldz,
                     work.as_mut_ptr() as *mut _, &lwork, rwork.as_mut_ptr(), &lrwork,
                     iwork.as_mut_ptr(), &liwork, info)
    }
}

#[inline]
pub fn ssbevx(jobz: u8, range: u8, uplo: u8, n: i32, kd: i32, ab: &mut [f32], ldab: i32,
              q: &mut f32, ldq: i32, vl: f32, vu: f32, il: i32, iu: i32, abstol: f32, m: &mut i32,
              w: &mut [f32], z: &mut [f32], ldz: i32, work: &mut [f32], iwork: &mut [i32],
              ifail: &mut i32, info: &mut i32) {

    unsafe {
        ffi::ssbevx_(&(jobz as c_char), &(range as c_char), &(uplo as c_char), &n, &kd,
                     ab.as_mut_ptr(), &ldab, q, &ldq, &vl, &vu, &il, &iu, &abstol, m,
                     w.as_mut_ptr(), z.as_mut_ptr(), &ldz, work.as_mut_ptr(), iwork.as_mut_ptr(),
                     ifail, info)
    }
}

#[inline]
pub fn dsbevx(jobz: u8, range: u8, uplo: u8, n: i32, kd: i32, ab: &mut [f64], ldab: i32,
              q: &mut f64, ldq: i32, vl: f64, vu: f64, il: i32, iu: i32, abstol: f64, m: &mut i32,
              w: &mut [f64], z: &mut [f64], ldz: i32, work: &mut [f64], iwork: &mut [i32],
              ifail: &mut i32, info: &mut i32) {

    unsafe {
        ffi::dsbevx_(&(jobz as c_char), &(range as c_char), &(uplo as c_char), &n, &kd,
                     ab.as_mut_ptr(), &ldab, q, &ldq, &vl, &vu, &il, &iu, &abstol, m,
                     w.as_mut_ptr(), z.as_mut_ptr(), &ldz, work.as_mut_ptr(), iwork.as_mut_ptr(),
                     ifail, info)
    }
}

#[inline]
pub fn chbevx(jobz: u8, range: u8, uplo: u8, n: i32, kd: i32, ab: &mut [c32], ldab: i32,
              q: &mut c32, ldq: i32, vl: f32, vu: f32, il: i32, iu: i32, abstol: f32, m: &mut i32,
              w: &mut [f32], z: &mut [c32], ldz: i32, work: &mut [c32], rwork: &mut [f32],
              iwork: &mut [i32], ifail: &mut i32, info: &mut i32) {

    unsafe {
        ffi::chbevx_(&(jobz as c_char), &(range as c_char), &(uplo as c_char), &n, &kd,
                     ab.as_mut_ptr() as *mut _, &ldab, q as *mut _ as *mut _, &ldq, &vl, &vu, &il,
                     &iu, &abstol, m, w.as_mut_ptr(), z.as_mut_ptr() as *mut _, &ldz,
                     work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), iwork.as_mut_ptr(), ifail,
                     info)
    }
}

#[inline]
pub fn zhbevx(jobz: u8, range: u8, uplo: u8, n: i32, kd: i32, ab: &mut [c64], ldab: i32,
              q: &mut c64, ldq: i32, vl: f64, vu: f64, il: i32, iu: i32, abstol: f64, m: &mut i32,
              w: &mut [f64], z: &mut [c64], ldz: i32, work: &mut [c64], rwork: &mut [f64],
              iwork: &mut [i32], ifail: &mut i32, info: &mut i32) {

    unsafe {
        ffi::zhbevx_(&(jobz as c_char), &(range as c_char), &(uplo as c_char), &n, &kd,
                     ab.as_mut_ptr() as *mut _, &ldab, q as *mut _ as *mut _, &ldq, &vl, &vu, &il,
                     &iu, &abstol, m, w.as_mut_ptr(), z.as_mut_ptr() as *mut _, &ldz,
                     work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), iwork.as_mut_ptr(), ifail,
                     info)
    }
}

#[inline]
pub fn sstev(jobz: u8, n: i32, d: &mut [f32], e: &mut [f32], z: &mut [f32], ldz: i32,
             work: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::sstev_(&(jobz as c_char), &n, d.as_mut_ptr(), e.as_mut_ptr(), z.as_mut_ptr(), &ldz,
                    work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dstev(jobz: u8, n: i32, d: &mut [f64], e: &mut [f64], z: &mut [f64], ldz: i32,
             work: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::dstev_(&(jobz as c_char), &n, d.as_mut_ptr(), e.as_mut_ptr(), z.as_mut_ptr(), &ldz,
                    work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn sstevd(jobz: u8, n: i32, d: &mut [f32], e: &mut [f32], z: &mut [f32], ldz: i32,
              work: &mut [f32], lwork: i32, iwork: &mut [i32], liwork: i32, info: &mut i32) {

    unsafe {
        ffi::sstevd_(&(jobz as c_char), &n, d.as_mut_ptr(), e.as_mut_ptr(), z.as_mut_ptr(), &ldz,
                     work.as_mut_ptr(), &lwork, iwork.as_mut_ptr(), &liwork, info)
    }
}

#[inline]
pub fn dstevd(jobz: u8, n: i32, d: &mut [f64], e: &mut [f64], z: &mut [f64], ldz: i32,
              work: &mut [f64], lwork: i32, iwork: &mut [i32], liwork: i32, info: &mut i32) {

    unsafe {
        ffi::dstevd_(&(jobz as c_char), &n, d.as_mut_ptr(), e.as_mut_ptr(), z.as_mut_ptr(), &ldz,
                     work.as_mut_ptr(), &lwork, iwork.as_mut_ptr(), &liwork, info)
    }
}

#[inline]
pub fn sstevx(jobz: u8, range: u8, n: i32, d: &mut [f32], e: &mut [f32], vl: f32, vu: f32, il: i32,
              iu: i32, abstol: f32, m: &mut i32, w: &mut [f32], z: &mut [f32], ldz: i32,
              work: &mut [f32], iwork: &mut [i32], ifail: &mut i32, info: &mut i32) {

    unsafe {
        ffi::sstevx_(&(jobz as c_char), &(range as c_char), &n, d.as_mut_ptr(), e.as_mut_ptr(),
                     &vl, &vu, &il, &iu, &abstol, m, w.as_mut_ptr(), z.as_mut_ptr(), &ldz,
                     work.as_mut_ptr(), iwork.as_mut_ptr(), ifail, info)
    }
}

#[inline]
pub fn dstevx(jobz: u8, range: u8, n: i32, d: &mut [f64], e: &mut [f64], vl: f64, vu: f64, il: i32,
              iu: i32, abstol: f64, m: &mut i32, w: &mut [f64], z: &mut [f64], ldz: i32,
              work: &mut [f64], iwork: &mut [i32], ifail: &mut i32, info: &mut i32) {

    unsafe {
        ffi::dstevx_(&(jobz as c_char), &(range as c_char), &n, d.as_mut_ptr(), e.as_mut_ptr(),
                     &vl, &vu, &il, &iu, &abstol, m, w.as_mut_ptr(), z.as_mut_ptr(), &ldz,
                     work.as_mut_ptr(), iwork.as_mut_ptr(), ifail, info)
    }
}

#[inline]
pub fn sstevr(jobz: u8, range: u8, n: i32, d: &mut [f32], e: &mut [f32], vl: f32, vu: f32, il: i32,
              iu: i32, abstol: f32, m: &mut i32, w: &mut [f32], z: &mut [f32], ldz: i32,
              isuppz: &mut [i32], work: &mut [f32], lwork: i32, iwork: &mut [i32], liwork: i32,
              info: &mut i32) {

    unsafe {
        ffi::sstevr_(&(jobz as c_char), &(range as c_char), &n, d.as_mut_ptr(), e.as_mut_ptr(),
                     &vl, &vu, &il, &iu, &abstol, m, w.as_mut_ptr(), z.as_mut_ptr(), &ldz,
                     isuppz.as_mut_ptr(), work.as_mut_ptr(), &lwork, iwork.as_mut_ptr(), &liwork,
                     info)
    }
}

#[inline]
pub fn dstevr(jobz: u8, range: u8, n: i32, d: &mut [f64], e: &mut [f64], vl: f64, vu: f64, il: i32,
              iu: i32, abstol: f64, m: &mut i32, w: &mut [f64], z: &mut [f64], ldz: i32,
              isuppz: &mut [i32], work: &mut [f64], lwork: i32, iwork: &mut [i32], liwork: i32,
              info: &mut i32) {

    unsafe {
        ffi::dstevr_(&(jobz as c_char), &(range as c_char), &n, d.as_mut_ptr(), e.as_mut_ptr(),
                     &vl, &vu, &il, &iu, &abstol, m, w.as_mut_ptr(), z.as_mut_ptr(), &ldz,
                     isuppz.as_mut_ptr(), work.as_mut_ptr(), &lwork, iwork.as_mut_ptr(), &liwork,
                     info)
    }
}

#[inline]
pub fn sgees(jobvs: u8, sort: u8, select: Select2F32, n: i32, a: &mut [f32], lda: i32,
             sdim: &mut [i32], wr: &mut [f32], wi: &mut [f32], vs: &mut [f32], ldvs: i32,
             work: &mut [f32], lwork: i32, bwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::sgees_(&(jobvs as c_char), &(sort as c_char), transmute(select), &n, a.as_mut_ptr(),
                    &lda, sdim.as_mut_ptr(), wr.as_mut_ptr(), wi.as_mut_ptr(), vs.as_mut_ptr(),
                    &ldvs, work.as_mut_ptr(), &lwork, bwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dgees(jobvs: u8, sort: u8, select: Select2F64, n: i32, a: &mut [f64], lda: i32,
             sdim: &mut [i32], wr: &mut [f64], wi: &mut [f64], vs: &mut [f64], ldvs: i32,
             work: &mut [f64], lwork: i32, bwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::dgees_(&(jobvs as c_char), &(sort as c_char), transmute(select), &n, a.as_mut_ptr(),
                    &lda, sdim.as_mut_ptr(), wr.as_mut_ptr(), wi.as_mut_ptr(), vs.as_mut_ptr(),
                    &ldvs, work.as_mut_ptr(), &lwork, bwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn cgees(jobvs: u8, sort: u8, select: Select1C32, n: i32, a: &mut [c32], lda: i32,
             sdim: &mut [i32], w: &mut [c32], vs: &mut [c32], ldvs: i32, work: &mut [c32],
             lwork: i32, rwork: &mut [f32], bwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::cgees_(&(jobvs as c_char), &(sort as c_char), transmute(select), &n,
                    a.as_mut_ptr() as *mut _, &lda, sdim.as_mut_ptr(), w.as_mut_ptr() as *mut _,
                    vs.as_mut_ptr() as *mut _, &ldvs, work.as_mut_ptr() as *mut _, &lwork,
                    rwork.as_mut_ptr(), bwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zgees(jobvs: u8, sort: u8, select: Select1C64, n: i32, a: &mut [c64], lda: i32,
             sdim: &mut [i32], w: &mut [c64], vs: &mut [c64], ldvs: i32, work: &mut [c64],
             lwork: i32, rwork: &mut [f64], bwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::zgees_(&(jobvs as c_char), &(sort as c_char), transmute(select), &n,
                    a.as_mut_ptr() as *mut _, &lda, sdim.as_mut_ptr(), w.as_mut_ptr() as *mut _,
                    vs.as_mut_ptr() as *mut _, &ldvs, work.as_mut_ptr() as *mut _, &lwork,
                    rwork.as_mut_ptr(), bwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn sgeesx(jobvs: u8, sort: u8, select: Select2F32, sense: u8, n: i32, a: &mut [f32], lda: i32,
              sdim: &mut [i32], wr: &mut [f32], wi: &mut [f32], vs: &mut [f32], ldvs: i32,
              rconde: &mut [f32], rcondv: &mut [f32], work: &mut [f32], lwork: i32,
              iwork: &mut [i32], liwork: i32, bwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::sgeesx_(&(jobvs as c_char), &(sort as c_char), transmute(select), &(sense as c_char),
                     &n, a.as_mut_ptr(), &lda, sdim.as_mut_ptr(), wr.as_mut_ptr(), wi.as_mut_ptr(),
                     vs.as_mut_ptr(), &ldvs, rconde.as_mut_ptr(), rcondv.as_mut_ptr(),
                     work.as_mut_ptr(), &lwork, iwork.as_mut_ptr(), &liwork, bwork.as_mut_ptr(),
                     info)
    }
}

#[inline]
pub fn dgeesx(jobvs: u8, sort: u8, select: Select2F64, sense: u8, n: i32, a: &mut [f64], lda: i32,
              sdim: &mut [i32], wr: &mut [f64], wi: &mut [f64], vs: &mut [f64], ldvs: i32,
              rconde: &mut [f64], rcondv: &mut [f64], work: &mut [f64], lwork: i32,
              iwork: &mut [i32], liwork: i32, bwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::dgeesx_(&(jobvs as c_char), &(sort as c_char), transmute(select), &(sense as c_char),
                     &n, a.as_mut_ptr(), &lda, sdim.as_mut_ptr(), wr.as_mut_ptr(), wi.as_mut_ptr(),
                     vs.as_mut_ptr(), &ldvs, rconde.as_mut_ptr(), rcondv.as_mut_ptr(),
                     work.as_mut_ptr(), &lwork, iwork.as_mut_ptr(), &liwork, bwork.as_mut_ptr(),
                     info)
    }
}

#[inline]
pub fn cgeesx(jobvs: u8, sort: u8, select: Select1C32, sense: u8, n: i32, a: &mut [c32], lda: i32,
              sdim: &mut [i32], w: &mut [c32], vs: &mut [c32], ldvs: i32, rconde: &mut [f32],
              rcondv: &mut [f32], work: &mut [c32], lwork: i32, rwork: &mut [f32],
              bwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::cgeesx_(&(jobvs as c_char), &(sort as c_char), transmute(select), &(sense as c_char),
                     &n, a.as_mut_ptr() as *mut _, &lda, sdim.as_mut_ptr(),
                     w.as_mut_ptr() as *mut _, vs.as_mut_ptr() as *mut _, &ldvs,
                     rconde.as_mut_ptr(), rcondv.as_mut_ptr(), work.as_mut_ptr() as *mut _, &lwork,
                     rwork.as_mut_ptr(), bwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zgeesx(jobvs: u8, sort: u8, select: Select1C64, sense: u8, n: i32, a: &mut [c64], lda: i32,
              sdim: &mut [i32], w: &mut [c64], vs: &mut [c64], ldvs: i32, rconde: &mut [f64],
              rcondv: &mut [f64], work: &mut [c64], lwork: i32, rwork: &mut [f64],
              bwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::zgeesx_(&(jobvs as c_char), &(sort as c_char), transmute(select), &(sense as c_char),
                     &n, a.as_mut_ptr() as *mut _, &lda, sdim.as_mut_ptr(),
                     w.as_mut_ptr() as *mut _, vs.as_mut_ptr() as *mut _, &ldvs,
                     rconde.as_mut_ptr(), rcondv.as_mut_ptr(), work.as_mut_ptr() as *mut _, &lwork,
                     rwork.as_mut_ptr(), bwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn sgeev(jobvl: u8, jobvr: u8, n: i32, a: &mut [f32], lda: i32, wr: &mut [f32], wi: &mut [f32],
             vl: &mut f32, ldvl: i32, vr: &mut f32, ldvr: i32, work: &mut [f32], lwork: i32,
             info: &mut i32) {

    unsafe {
        ffi::sgeev_(&(jobvl as c_char), &(jobvr as c_char), &n, a.as_mut_ptr(), &lda,
                    wr.as_mut_ptr(), wi.as_mut_ptr(), vl, &ldvl, vr, &ldvr, work.as_mut_ptr(),
                    &lwork, info)
    }
}

#[inline]
pub fn dgeev(jobvl: u8, jobvr: u8, n: i32, a: &mut [f64], lda: i32, wr: &mut [f64], wi: &mut [f64],
             vl: &mut f64, ldvl: i32, vr: &mut f64, ldvr: i32, work: &mut [f64], lwork: i32,
             info: &mut i32) {

    unsafe {
        ffi::dgeev_(&(jobvl as c_char), &(jobvr as c_char), &n, a.as_mut_ptr(), &lda,
                    wr.as_mut_ptr(), wi.as_mut_ptr(), vl, &ldvl, vr, &ldvr, work.as_mut_ptr(),
                    &lwork, info)
    }
}

#[inline]
pub fn cgeev(jobvl: u8, jobvr: u8, n: i32, a: &mut [c32], lda: i32, w: &mut [c32], vl: &mut c32,
             ldvl: i32, vr: &mut c32, ldvr: i32, work: &mut [c32], lwork: i32, rwork: &mut [f32],
             info: &mut i32) {

    unsafe {
        ffi::cgeev_(&(jobvl as c_char), &(jobvr as c_char), &n, a.as_mut_ptr() as *mut _, &lda,
                    w.as_mut_ptr() as *mut _, vl as *mut _ as *mut _, &ldvl,
                    vr as *mut _ as *mut _, &ldvr, work.as_mut_ptr() as *mut _, &lwork,
                    rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zgeev(jobvl: u8, jobvr: u8, n: i32, a: &mut [c64], lda: i32, w: &mut [c64], vl: &mut c64,
             ldvl: i32, vr: &mut c64, ldvr: i32, work: &mut [c64], lwork: i32, rwork: &mut [f64],
             info: &mut i32) {

    unsafe {
        ffi::zgeev_(&(jobvl as c_char), &(jobvr as c_char), &n, a.as_mut_ptr() as *mut _, &lda,
                    w.as_mut_ptr() as *mut _, vl as *mut _ as *mut _, &ldvl,
                    vr as *mut _ as *mut _, &ldvr, work.as_mut_ptr() as *mut _, &lwork,
                    rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn sgeevx(balanc: u8, jobvl: u8, jobvr: u8, sense: u8, n: i32, a: &mut [f32], lda: i32,
              wr: &mut [f32], wi: &mut [f32], vl: &mut f32, ldvl: i32, vr: &mut f32, ldvr: i32,
              ilo: &mut i32, ihi: &mut i32, scale: &mut [f32], abnrm: &mut [f32],
              rconde: &mut [f32], rcondv: &mut [f32], work: &mut [f32], lwork: i32,
              iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::sgeevx_(&(balanc as c_char), &(jobvl as c_char), &(jobvr as c_char),
                     &(sense as c_char), &n, a.as_mut_ptr(), &lda, wr.as_mut_ptr(),
                     wi.as_mut_ptr(), vl, &ldvl, vr, &ldvr, ilo, ihi, scale.as_mut_ptr(),
                     abnrm.as_mut_ptr(), rconde.as_mut_ptr(), rcondv.as_mut_ptr(),
                     work.as_mut_ptr(), &lwork, iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dgeevx(balanc: u8, jobvl: u8, jobvr: u8, sense: u8, n: i32, a: &mut [f64], lda: i32,
              wr: &mut [f64], wi: &mut [f64], vl: &mut f64, ldvl: i32, vr: &mut f64, ldvr: i32,
              ilo: &mut i32, ihi: &mut i32, scale: &mut [f64], abnrm: &mut [f64],
              rconde: &mut [f64], rcondv: &mut [f64], work: &mut [f64], lwork: i32,
              iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::dgeevx_(&(balanc as c_char), &(jobvl as c_char), &(jobvr as c_char),
                     &(sense as c_char), &n, a.as_mut_ptr(), &lda, wr.as_mut_ptr(),
                     wi.as_mut_ptr(), vl, &ldvl, vr, &ldvr, ilo, ihi, scale.as_mut_ptr(),
                     abnrm.as_mut_ptr(), rconde.as_mut_ptr(), rcondv.as_mut_ptr(),
                     work.as_mut_ptr(), &lwork, iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn cgeevx(balanc: u8, jobvl: u8, jobvr: u8, sense: u8, n: i32, a: &mut [c32], lda: i32,
              w: &mut [c32], vl: &mut c32, ldvl: i32, vr: &mut c32, ldvr: i32, ilo: &mut i32,
              ihi: &mut i32, scale: &mut [f32], abnrm: &mut [f32], rconde: &mut [f32],
              rcondv: &mut [f32], work: &mut [c32], lwork: i32, rwork: &mut [f32],
              info: &mut i32) {

    unsafe {
        ffi::cgeevx_(&(balanc as c_char), &(jobvl as c_char), &(jobvr as c_char),
                     &(sense as c_char), &n, a.as_mut_ptr() as *mut _, &lda,
                     w.as_mut_ptr() as *mut _, vl as *mut _ as *mut _, &ldvl,
                     vr as *mut _ as *mut _, &ldvr, ilo, ihi, scale.as_mut_ptr(),
                     abnrm.as_mut_ptr(), rconde.as_mut_ptr(), rcondv.as_mut_ptr(),
                     work.as_mut_ptr() as *mut _, &lwork, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zgeevx(balanc: u8, jobvl: u8, jobvr: u8, sense: u8, n: i32, a: &mut [c64], lda: i32,
              w: &mut [c64], vl: &mut c64, ldvl: i32, vr: &mut c64, ldvr: i32, ilo: &mut i32,
              ihi: &mut i32, scale: &mut [f64], abnrm: &mut [f64], rconde: &mut [f64],
              rcondv: &mut [f64], work: &mut [c64], lwork: i32, rwork: &mut [f64],
              info: &mut i32) {

    unsafe {
        ffi::zgeevx_(&(balanc as c_char), &(jobvl as c_char), &(jobvr as c_char),
                     &(sense as c_char), &n, a.as_mut_ptr() as *mut _, &lda,
                     w.as_mut_ptr() as *mut _, vl as *mut _ as *mut _, &ldvl,
                     vr as *mut _ as *mut _, &ldvr, ilo, ihi, scale.as_mut_ptr(),
                     abnrm.as_mut_ptr(), rconde.as_mut_ptr(), rcondv.as_mut_ptr(),
                     work.as_mut_ptr() as *mut _, &lwork, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn sgesvd(jobu: u8, jobvt: u8, m: i32, n: i32, a: &mut [f32], lda: i32, s: &mut [f32],
              u: &mut [f32], ldu: i32, vt: &mut [f32], ldvt: i32, work: &mut [f32], lwork: i32,
              info: &mut i32) {

    unsafe {
        ffi::sgesvd_(&(jobu as c_char), &(jobvt as c_char), &m, &n, a.as_mut_ptr(), &lda,
                     s.as_mut_ptr(), u.as_mut_ptr(), &ldu, vt.as_mut_ptr(), &ldvt,
                     work.as_mut_ptr(), &lwork, info)
    }
}

#[inline]
pub fn dgesvd(jobu: u8, jobvt: u8, m: i32, n: i32, a: &mut [f64], lda: i32, s: &mut [f64],
              u: &mut [f64], ldu: i32, vt: &mut [f64], ldvt: i32, work: &mut [f64], lwork: i32,
              info: &mut i32) {

    unsafe {
        ffi::dgesvd_(&(jobu as c_char), &(jobvt as c_char), &m, &n, a.as_mut_ptr(), &lda,
                     s.as_mut_ptr(), u.as_mut_ptr(), &ldu, vt.as_mut_ptr(), &ldvt,
                     work.as_mut_ptr(), &lwork, info)
    }
}

#[inline]
pub fn cgesvd(jobu: u8, jobvt: u8, m: i32, n: i32, a: &mut [c32], lda: i32, s: &mut [f32],
              u: &mut [c32], ldu: i32, vt: &mut [c32], ldvt: i32, work: &mut [c32], lwork: i32,
              rwork: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::cgesvd_(&(jobu as c_char), &(jobvt as c_char), &m, &n, a.as_mut_ptr() as *mut _, &lda,
                     s.as_mut_ptr(), u.as_mut_ptr() as *mut _, &ldu, vt.as_mut_ptr() as *mut _,
                     &ldvt, work.as_mut_ptr() as *mut _, &lwork, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zgesvd(jobu: u8, jobvt: u8, m: i32, n: i32, a: &mut [c64], lda: i32, s: &mut [f64],
              u: &mut [c64], ldu: i32, vt: &mut [c64], ldvt: i32, work: &mut [c64], lwork: i32,
              rwork: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::zgesvd_(&(jobu as c_char), &(jobvt as c_char), &m, &n, a.as_mut_ptr() as *mut _, &lda,
                     s.as_mut_ptr(), u.as_mut_ptr() as *mut _, &ldu, vt.as_mut_ptr() as *mut _,
                     &ldvt, work.as_mut_ptr() as *mut _, &lwork, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn sgesvdx(jobu: u8, jobvt: u8, range: u8, m: i32, n: i32, a: &mut [f32], lda: i32, vl: f32,
               vu: f32, il: i32, iu: i32, ns: &mut [i32], s: &mut [f32], u: &mut [f32], ldu: i32,
               vt: &mut [f32], ldvt: i32, work: &mut [f32], lwork: i32, iwork: &mut [i32],
               info: &mut i32) {

    unsafe {
        ffi::sgesvdx_(&(jobu as c_char), &(jobvt as c_char), &(range as c_char), &m, &n,
                      a.as_mut_ptr(), &lda, &vl, &vu, &il, &iu, ns.as_mut_ptr(), s.as_mut_ptr(),
                      u.as_mut_ptr(), &ldu, vt.as_mut_ptr(), &ldvt, work.as_mut_ptr(), &lwork,
                      iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dgesvdx(jobu: u8, jobvt: u8, range: u8, m: i32, n: i32, a: &mut [f64], lda: i32, vl: f64,
               vu: f64, il: i32, iu: i32, ns: &mut [i32], s: &mut [f64], u: &mut [f64], ldu: i32,
               vt: &mut [f64], ldvt: i32, work: &mut [f64], lwork: i32, iwork: &mut [i32],
               info: &mut i32) {

    unsafe {
        ffi::dgesvdx_(&(jobu as c_char), &(jobvt as c_char), &(range as c_char), &m, &n,
                      a.as_mut_ptr(), &lda, &vl, &vu, &il, &iu, ns.as_mut_ptr(), s.as_mut_ptr(),
                      u.as_mut_ptr(), &ldu, vt.as_mut_ptr(), &ldvt, work.as_mut_ptr(), &lwork,
                      iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn cgesvdx(jobu: u8, jobvt: u8, range: u8, m: i32, n: i32, a: &mut [c32], lda: i32, vl: f32,
               vu: f32, il: i32, iu: i32, ns: &mut [i32], s: &mut [f32], u: &mut [c32], ldu: i32,
               vt: &mut [c32], ldvt: i32, work: &mut [c32], lwork: i32, rwork: &mut [f32],
               iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::cgesvdx_(&(jobu as c_char), &(jobvt as c_char), &(range as c_char), &m, &n,
                      a.as_mut_ptr() as *mut _, &lda, &vl, &vu, &il, &iu, ns.as_mut_ptr(),
                      s.as_mut_ptr(), u.as_mut_ptr() as *mut _, &ldu, vt.as_mut_ptr() as *mut _,
                      &ldvt, work.as_mut_ptr() as *mut _, &lwork, rwork.as_mut_ptr(),
                      iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zgesvdx(jobu: u8, jobvt: u8, range: u8, m: i32, n: i32, a: &mut [c64], lda: i32, vl: f64,
               vu: f64, il: i32, iu: i32, ns: &mut [i32], s: &mut [f64], u: &mut [c64], ldu: i32,
               vt: &mut [c64], ldvt: i32, work: &mut [c64], lwork: i32, rwork: &mut [f64],
               iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::zgesvdx_(&(jobu as c_char), &(jobvt as c_char), &(range as c_char), &m, &n,
                      a.as_mut_ptr() as *mut _, &lda, &vl, &vu, &il, &iu, ns.as_mut_ptr(),
                      s.as_mut_ptr(), u.as_mut_ptr() as *mut _, &ldu, vt.as_mut_ptr() as *mut _,
                      &ldvt, work.as_mut_ptr() as *mut _, &lwork, rwork.as_mut_ptr(),
                      iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn sgesdd(jobz: u8, m: i32, n: i32, a: &mut [f32], lda: i32, s: &mut [f32], u: &mut [f32],
              ldu: i32, vt: &mut [f32], ldvt: i32, work: &mut [f32], lwork: i32, iwork: &mut [i32],
              info: &mut i32) {

    unsafe {
        ffi::sgesdd_(&(jobz as c_char), &m, &n, a.as_mut_ptr(), &lda, s.as_mut_ptr(),
                     u.as_mut_ptr(), &ldu, vt.as_mut_ptr(), &ldvt, work.as_mut_ptr(), &lwork,
                     iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dgesdd(jobz: u8, m: i32, n: i32, a: &mut [f64], lda: i32, s: &mut [f64], u: &mut [f64],
              ldu: i32, vt: &mut [f64], ldvt: i32, work: &mut [f64], lwork: i32, iwork: &mut [i32],
              info: &mut i32) {

    unsafe {
        ffi::dgesdd_(&(jobz as c_char), &m, &n, a.as_mut_ptr(), &lda, s.as_mut_ptr(),
                     u.as_mut_ptr(), &ldu, vt.as_mut_ptr(), &ldvt, work.as_mut_ptr(), &lwork,
                     iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn cgesdd(jobz: u8, m: i32, n: i32, a: &mut [c32], lda: i32, s: &mut [f32], u: &mut [c32],
              ldu: i32, vt: &mut [c32], ldvt: i32, work: &mut [c32], lwork: i32, rwork: &mut [f32],
              iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::cgesdd_(&(jobz as c_char), &m, &n, a.as_mut_ptr() as *mut _, &lda, s.as_mut_ptr(),
                     u.as_mut_ptr() as *mut _, &ldu, vt.as_mut_ptr() as *mut _, &ldvt,
                     work.as_mut_ptr() as *mut _, &lwork, rwork.as_mut_ptr(), iwork.as_mut_ptr(),
                     info)
    }
}

#[inline]
pub fn zgesdd(jobz: u8, m: i32, n: i32, a: &mut [c64], lda: i32, s: &mut [f64], u: &mut [c64],
              ldu: i32, vt: &mut [c64], ldvt: i32, work: &mut [c64], lwork: i32, rwork: &mut [f64],
              iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::zgesdd_(&(jobz as c_char), &m, &n, a.as_mut_ptr() as *mut _, &lda, s.as_mut_ptr(),
                     u.as_mut_ptr() as *mut _, &ldu, vt.as_mut_ptr() as *mut _, &ldvt,
                     work.as_mut_ptr() as *mut _, &lwork, rwork.as_mut_ptr(), iwork.as_mut_ptr(),
                     info)
    }
}

#[inline]
pub fn dgejsv(joba: u8, jobu: u8, jobv: u8, jobr: u8, jobt: u8, jobp: u8, m: i32, n: i32,
              a: &mut [f64], lda: i32, sva: &mut [f64], u: &mut [f64], ldu: i32, v: &mut [f64],
              ldv: i32, work: &mut [f64], lwork: i32, iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::dgejsv_(&(joba as c_char), &(jobu as c_char), &(jobv as c_char), &(jobr as c_char),
                     &(jobt as c_char), &(jobp as c_char), &m, &n, a.as_mut_ptr(), &lda,
                     sva.as_mut_ptr(), u.as_mut_ptr(), &ldu, v.as_mut_ptr(), &ldv,
                     work.as_mut_ptr(), &lwork, iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn sgejsv(joba: u8, jobu: u8, jobv: u8, jobr: u8, jobt: u8, jobp: u8, m: i32, n: i32,
              a: &mut [f32], lda: i32, sva: &mut [f32], u: &mut [f32], ldu: i32, v: &mut [f32],
              ldv: i32, work: &mut [f32], lwork: i32, iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::sgejsv_(&(joba as c_char), &(jobu as c_char), &(jobv as c_char), &(jobr as c_char),
                     &(jobt as c_char), &(jobp as c_char), &m, &n, a.as_mut_ptr(), &lda,
                     sva.as_mut_ptr(), u.as_mut_ptr(), &ldu, v.as_mut_ptr(), &ldv,
                     work.as_mut_ptr(), &lwork, iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn cgejsv(joba: u8, jobu: u8, jobv: u8, jobr: u8, jobt: u8, jobp: u8, m: i32, n: i32,
              a: &mut [c32], lda: i32, sva: &mut [f32], u: &mut [c32], ldu: i32, v: &mut [c32],
              ldv: i32, cwork: &mut [c32], lwork: i32, work: &mut [f32], lrwork: i32,
              iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::cgejsv_(&(joba as c_char), &(jobu as c_char), &(jobv as c_char), &(jobr as c_char),
                     &(jobt as c_char), &(jobp as c_char), &m, &n, a.as_mut_ptr() as *mut _, &lda,
                     sva.as_mut_ptr(), u.as_mut_ptr() as *mut _, &ldu, v.as_mut_ptr() as *mut _,
                     &ldv, cwork.as_mut_ptr() as *mut _, &lwork, work.as_mut_ptr(), &lrwork,
                     iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zgejsv(joba: u8, jobu: u8, jobv: u8, jobr: u8, jobt: u8, jobp: u8, m: i32, n: i32,
              a: &mut [c64], lda: i32, sva: &mut [f64], u: &mut [c64], ldu: i32, v: &mut [c64],
              ldv: i32, cwork: &mut [c64], lwork: i32, work: &mut [f64], lrwork: i32,
              iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::zgejsv_(&(joba as c_char), &(jobu as c_char), &(jobv as c_char), &(jobr as c_char),
                     &(jobt as c_char), &(jobp as c_char), &m, &n, a.as_mut_ptr() as *mut _, &lda,
                     sva.as_mut_ptr(), u.as_mut_ptr() as *mut _, &ldu, v.as_mut_ptr() as *mut _,
                     &ldv, cwork.as_mut_ptr() as *mut _, &lwork, work.as_mut_ptr(), &lrwork,
                     iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dgesvj(joba: u8, jobu: u8, jobv: u8, m: i32, n: i32, a: &mut [f64], lda: i32,
              sva: &mut [f64], mv: &[i32], v: &mut [f64], ldv: i32, work: &mut [f64], lwork: i32,
              info: &mut i32) {

    unsafe {
        ffi::dgesvj_(&(joba as c_char), &(jobu as c_char), &(jobv as c_char), &m, &n,
                     a.as_mut_ptr(), &lda, sva.as_mut_ptr(), mv.as_ptr(), v.as_mut_ptr(), &ldv,
                     work.as_mut_ptr(), &lwork, info)
    }
}

#[inline]
pub fn sgesvj(joba: u8, jobu: u8, jobv: u8, m: i32, n: i32, a: &mut [f32], lda: i32,
              sva: &mut [f32], mv: &[i32], v: &mut [f32], ldv: i32, work: &mut [f32], lwork: i32,
              info: &mut i32) {

    unsafe {
        ffi::sgesvj_(&(joba as c_char), &(jobu as c_char), &(jobv as c_char), &m, &n,
                     a.as_mut_ptr(), &lda, sva.as_mut_ptr(), mv.as_ptr(), v.as_mut_ptr(), &ldv,
                     work.as_mut_ptr(), &lwork, info)
    }
}

#[inline]
pub fn cgesvj(joba: u8, jobu: u8, jobv: u8, m: i32, n: i32, a: &mut [c32], lda: i32,
              sva: &mut [f32], mv: &[i32], v: &mut [c32], ldv: i32, cwork: &mut [c32], lwork: i32,
              rwork: &mut [f32], lrwork: i32, info: &mut i32) {

    unsafe {
        ffi::cgesvj_(&(joba as c_char), &(jobu as c_char), &(jobv as c_char), &m, &n,
                     a.as_mut_ptr() as *mut _, &lda, sva.as_mut_ptr(), mv.as_ptr(),
                     v.as_mut_ptr() as *mut _, &ldv, cwork.as_mut_ptr() as *mut _, &lwork,
                     rwork.as_mut_ptr(), &lrwork, info)
    }
}

#[inline]
pub fn zgesvj(joba: u8, jobu: u8, jobv: u8, m: i32, n: i32, a: &mut [c64], lda: i32,
              sva: &mut [f64], mv: &[i32], v: &mut [c64], ldv: i32, cwork: &mut [c64], lwork: i32,
              rwork: &mut [f64], lrwork: i32, info: &mut i32) {

    unsafe {
        ffi::zgesvj_(&(joba as c_char), &(jobu as c_char), &(jobv as c_char), &m, &n,
                     a.as_mut_ptr() as *mut _, &lda, sva.as_mut_ptr(), mv.as_ptr(),
                     v.as_mut_ptr() as *mut _, &ldv, cwork.as_mut_ptr() as *mut _, &lwork,
                     rwork.as_mut_ptr(), &lrwork, info)
    }
}

#[inline]
pub fn sggsvd(jobu: u8, jobv: u8, jobq: u8, m: i32, n: i32, p: i32, k: &mut i32, l: &mut i32,
              a: &mut [f32], lda: i32, b: &mut [f32], ldb: i32, alpha: &mut f32, beta: &mut f32,
              u: &mut [f32], ldu: i32, v: &mut [f32], ldv: i32, q: &mut f32, ldq: i32,
              work: &mut [f32], iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::sggsvd_(&(jobu as c_char), &(jobv as c_char), &(jobq as c_char), &m, &n, &p, k, l,
                     a.as_mut_ptr(), &lda, b.as_mut_ptr(), &ldb, alpha, beta, u.as_mut_ptr(), &ldu,
                     v.as_mut_ptr(), &ldv, q, &ldq, work.as_mut_ptr(), iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dggsvd(jobu: u8, jobv: u8, jobq: u8, m: i32, n: i32, p: i32, k: &mut i32, l: &mut i32,
              a: &mut [f64], lda: i32, b: &mut [f64], ldb: i32, alpha: &mut f64, beta: &mut f64,
              u: &mut [f64], ldu: i32, v: &mut [f64], ldv: i32, q: &mut f64, ldq: i32,
              work: &mut [f64], iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::dggsvd_(&(jobu as c_char), &(jobv as c_char), &(jobq as c_char), &m, &n, &p, k, l,
                     a.as_mut_ptr(), &lda, b.as_mut_ptr(), &ldb, alpha, beta, u.as_mut_ptr(), &ldu,
                     v.as_mut_ptr(), &ldv, q, &ldq, work.as_mut_ptr(), iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn cggsvd(jobu: u8, jobv: u8, jobq: u8, m: i32, n: i32, p: i32, k: &mut i32, l: &mut i32,
              a: &mut [c32], lda: i32, b: &mut [c32], ldb: i32, alpha: &mut f32, beta: &mut f32,
              u: &mut [c32], ldu: i32, v: &mut [c32], ldv: i32, q: &mut c32, ldq: i32,
              work: &mut [c32], rwork: &mut [f32], iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::cggsvd_(&(jobu as c_char), &(jobv as c_char), &(jobq as c_char), &m, &n, &p, k, l,
                     a.as_mut_ptr() as *mut _, &lda, b.as_mut_ptr() as *mut _, &ldb, alpha, beta,
                     u.as_mut_ptr() as *mut _, &ldu, v.as_mut_ptr() as *mut _, &ldv,
                     q as *mut _ as *mut _, &ldq, work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(),
                     iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zggsvd(jobu: u8, jobv: u8, jobq: u8, m: i32, n: i32, p: i32, k: &mut i32, l: &mut i32,
              a: &mut [c64], lda: i32, b: &mut [c64], ldb: i32, alpha: &mut f64, beta: &mut f64,
              u: &mut [c64], ldu: i32, v: &mut [c64], ldv: i32, q: &mut c64, ldq: i32,
              work: &mut [c64], rwork: &mut [f64], iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::zggsvd_(&(jobu as c_char), &(jobv as c_char), &(jobq as c_char), &m, &n, &p, k, l,
                     a.as_mut_ptr() as *mut _, &lda, b.as_mut_ptr() as *mut _, &ldb, alpha, beta,
                     u.as_mut_ptr() as *mut _, &ldu, v.as_mut_ptr() as *mut _, &ldv,
                     q as *mut _ as *mut _, &ldq, work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(),
                     iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn sggsvd3(jobu: u8, jobv: u8, jobq: u8, m: i32, n: i32, p: i32, k: &mut i32, l: &mut i32,
               a: &mut [f32], lda: i32, b: &mut [f32], ldb: i32, alpha: &mut f32, beta: &mut f32,
               u: &mut [f32], ldu: i32, v: &mut [f32], ldv: i32, q: &mut f32, ldq: i32,
               work: &mut [f32], lwork: i32, iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::sggsvd3_(&(jobu as c_char), &(jobv as c_char), &(jobq as c_char), &m, &n, &p, k, l,
                      a.as_mut_ptr(), &lda, b.as_mut_ptr(), &ldb, alpha, beta, u.as_mut_ptr(),
                      &ldu, v.as_mut_ptr(), &ldv, q, &ldq, work.as_mut_ptr(), &lwork,
                      iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dggsvd3(jobu: u8, jobv: u8, jobq: u8, m: i32, n: i32, p: i32, k: &mut i32, l: &mut i32,
               a: &mut [f64], lda: i32, b: &mut [f64], ldb: i32, alpha: &mut f64, beta: &mut f64,
               u: &mut [f64], ldu: i32, v: &mut [f64], ldv: i32, q: &mut f64, ldq: i32,
               work: &mut [f64], lwork: i32, iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::dggsvd3_(&(jobu as c_char), &(jobv as c_char), &(jobq as c_char), &m, &n, &p, k, l,
                      a.as_mut_ptr(), &lda, b.as_mut_ptr(), &ldb, alpha, beta, u.as_mut_ptr(),
                      &ldu, v.as_mut_ptr(), &ldv, q, &ldq, work.as_mut_ptr(), &lwork,
                      iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn cggsvd3(jobu: u8, jobv: u8, jobq: u8, m: i32, n: i32, p: i32, k: &mut i32, l: &mut i32,
               a: &mut [c32], lda: i32, b: &mut [c32], ldb: i32, alpha: &mut f32, beta: &mut f32,
               u: &mut [c32], ldu: i32, v: &mut [c32], ldv: i32, q: &mut c32, ldq: i32,
               work: &mut [c32], lwork: i32, rwork: &mut [f32], iwork: &mut [i32],
               info: &mut i32) {

    unsafe {
        ffi::cggsvd3_(&(jobu as c_char), &(jobv as c_char), &(jobq as c_char), &m, &n, &p, k, l,
                      a.as_mut_ptr() as *mut _, &lda, b.as_mut_ptr() as *mut _, &ldb, alpha, beta,
                      u.as_mut_ptr() as *mut _, &ldu, v.as_mut_ptr() as *mut _, &ldv,
                      q as *mut _ as *mut _, &ldq, work.as_mut_ptr() as *mut _, &lwork,
                      rwork.as_mut_ptr(), iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zggsvd3(jobu: u8, jobv: u8, jobq: u8, m: i32, n: i32, p: i32, k: &mut i32, l: &mut i32,
               a: &mut [c64], lda: i32, b: &mut [c64], ldb: i32, alpha: &mut f64, beta: &mut f64,
               u: &mut [c64], ldu: i32, v: &mut [c64], ldv: i32, q: &mut c64, ldq: i32,
               work: &mut [c64], lwork: i32, rwork: &mut [f64], iwork: &mut [i32],
               info: &mut i32) {

    unsafe {
        ffi::zggsvd3_(&(jobu as c_char), &(jobv as c_char), &(jobq as c_char), &m, &n, &p, k, l,
                      a.as_mut_ptr() as *mut _, &lda, b.as_mut_ptr() as *mut _, &ldb, alpha, beta,
                      u.as_mut_ptr() as *mut _, &ldu, v.as_mut_ptr() as *mut _, &ldv,
                      q as *mut _ as *mut _, &ldq, work.as_mut_ptr() as *mut _, &lwork,
                      rwork.as_mut_ptr(), iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn ssygv(itype: &[i32], jobz: u8, uplo: u8, n: i32, a: &mut [f32], lda: i32, b: &mut [f32],
             ldb: i32, w: &mut [f32], work: &mut [f32], lwork: i32, info: &mut i32) {

    unsafe {
        ffi::ssygv_(itype.as_ptr(), &(jobz as c_char), &(uplo as c_char), &n, a.as_mut_ptr(), &lda,
                    b.as_mut_ptr(), &ldb, w.as_mut_ptr(), work.as_mut_ptr(), &lwork, info)
    }
}

#[inline]
pub fn dsygv(itype: &[i32], jobz: u8, uplo: u8, n: i32, a: &mut [f64], lda: i32, b: &mut [f64],
             ldb: i32, w: &mut [f64], work: &mut [f64], lwork: i32, info: &mut i32) {

    unsafe {
        ffi::dsygv_(itype.as_ptr(), &(jobz as c_char), &(uplo as c_char), &n, a.as_mut_ptr(), &lda,
                    b.as_mut_ptr(), &ldb, w.as_mut_ptr(), work.as_mut_ptr(), &lwork, info)
    }
}

#[inline]
pub fn chegv(itype: &[i32], jobz: u8, uplo: u8, n: i32, a: &mut [c32], lda: i32, b: &mut [c32],
             ldb: i32, w: &mut [f32], work: &mut [c32], lwork: i32, rwork: &mut [f32],
             info: &mut i32) {

    unsafe {
        ffi::chegv_(itype.as_ptr(), &(jobz as c_char), &(uplo as c_char), &n,
                    a.as_mut_ptr() as *mut _, &lda, b.as_mut_ptr() as *mut _, &ldb, w.as_mut_ptr(),
                    work.as_mut_ptr() as *mut _, &lwork, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zhegv(itype: &[i32], jobz: u8, uplo: u8, n: i32, a: &mut [c64], lda: i32, b: &mut [c64],
             ldb: i32, w: &mut [f64], work: &mut [c64], lwork: i32, rwork: &mut [f64],
             info: &mut i32) {

    unsafe {
        ffi::zhegv_(itype.as_ptr(), &(jobz as c_char), &(uplo as c_char), &n,
                    a.as_mut_ptr() as *mut _, &lda, b.as_mut_ptr() as *mut _, &ldb, w.as_mut_ptr(),
                    work.as_mut_ptr() as *mut _, &lwork, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn ssygvd(itype: &[i32], jobz: u8, uplo: u8, n: i32, a: &mut [f32], lda: i32, b: &mut [f32],
              ldb: i32, w: &mut [f32], work: &mut [f32], lwork: i32, iwork: &mut [i32],
              liwork: i32, info: &mut i32) {

    unsafe {
        ffi::ssygvd_(itype.as_ptr(), &(jobz as c_char), &(uplo as c_char), &n, a.as_mut_ptr(),
                     &lda, b.as_mut_ptr(), &ldb, w.as_mut_ptr(), work.as_mut_ptr(), &lwork,
                     iwork.as_mut_ptr(), &liwork, info)
    }
}

#[inline]
pub fn dsygvd(itype: &[i32], jobz: u8, uplo: u8, n: i32, a: &mut [f64], lda: i32, b: &mut [f64],
              ldb: i32, w: &mut [f64], work: &mut [f64], lwork: i32, iwork: &mut [i32],
              liwork: i32, info: &mut i32) {

    unsafe {
        ffi::dsygvd_(itype.as_ptr(), &(jobz as c_char), &(uplo as c_char), &n, a.as_mut_ptr(),
                     &lda, b.as_mut_ptr(), &ldb, w.as_mut_ptr(), work.as_mut_ptr(), &lwork,
                     iwork.as_mut_ptr(), &liwork, info)
    }
}

#[inline]
pub fn chegvd(itype: &[i32], jobz: u8, uplo: u8, n: i32, a: &mut [c32], lda: i32, b: &mut [c32],
              ldb: i32, w: &mut [f32], work: &mut [c32], lwork: i32, rwork: &mut [f32],
              lrwork: i32, iwork: &mut [i32], liwork: i32, info: &mut i32) {

    unsafe {
        ffi::chegvd_(itype.as_ptr(), &(jobz as c_char), &(uplo as c_char), &n,
                     a.as_mut_ptr() as *mut _, &lda, b.as_mut_ptr() as *mut _, &ldb,
                     w.as_mut_ptr(), work.as_mut_ptr() as *mut _, &lwork, rwork.as_mut_ptr(),
                     &lrwork, iwork.as_mut_ptr(), &liwork, info)
    }
}

#[inline]
pub fn zhegvd(itype: &[i32], jobz: u8, uplo: u8, n: i32, a: &mut [c64], lda: i32, b: &mut [c64],
              ldb: i32, w: &mut [f64], work: &mut [c64], lwork: i32, rwork: &mut [f64],
              lrwork: i32, iwork: &mut [i32], liwork: i32, info: &mut i32) {

    unsafe {
        ffi::zhegvd_(itype.as_ptr(), &(jobz as c_char), &(uplo as c_char), &n,
                     a.as_mut_ptr() as *mut _, &lda, b.as_mut_ptr() as *mut _, &ldb,
                     w.as_mut_ptr(), work.as_mut_ptr() as *mut _, &lwork, rwork.as_mut_ptr(),
                     &lrwork, iwork.as_mut_ptr(), &liwork, info)
    }
}

#[inline]
pub fn ssygvx(itype: &[i32], jobz: u8, range: u8, uplo: u8, n: i32, a: &mut [f32], lda: i32,
              b: &mut [f32], ldb: i32, vl: f32, vu: f32, il: i32, iu: i32, abstol: f32,
              m: &mut i32, w: &mut [f32], z: &mut [f32], ldz: i32, work: &mut [f32], lwork: i32,
              iwork: &mut [i32], ifail: &mut i32, info: &mut i32) {

    unsafe {
        ffi::ssygvx_(itype.as_ptr(), &(jobz as c_char), &(range as c_char), &(uplo as c_char), &n,
                     a.as_mut_ptr(), &lda, b.as_mut_ptr(), &ldb, &vl, &vu, &il, &iu, &abstol, m,
                     w.as_mut_ptr(), z.as_mut_ptr(), &ldz, work.as_mut_ptr(), &lwork,
                     iwork.as_mut_ptr(), ifail, info)
    }
}

#[inline]
pub fn dsygvx(itype: &[i32], jobz: u8, range: u8, uplo: u8, n: i32, a: &mut [f64], lda: i32,
              b: &mut [f64], ldb: i32, vl: f64, vu: f64, il: i32, iu: i32, abstol: f64,
              m: &mut i32, w: &mut [f64], z: &mut [f64], ldz: i32, work: &mut [f64], lwork: i32,
              iwork: &mut [i32], ifail: &mut i32, info: &mut i32) {

    unsafe {
        ffi::dsygvx_(itype.as_ptr(), &(jobz as c_char), &(range as c_char), &(uplo as c_char), &n,
                     a.as_mut_ptr(), &lda, b.as_mut_ptr(), &ldb, &vl, &vu, &il, &iu, &abstol, m,
                     w.as_mut_ptr(), z.as_mut_ptr(), &ldz, work.as_mut_ptr(), &lwork,
                     iwork.as_mut_ptr(), ifail, info)
    }
}

#[inline]
pub fn chegvx(itype: &[i32], jobz: u8, range: u8, uplo: u8, n: i32, a: &mut [c32], lda: i32,
              b: &mut [c32], ldb: i32, vl: f32, vu: f32, il: i32, iu: i32, abstol: f32,
              m: &mut i32, w: &mut [f32], z: &mut [c32], ldz: i32, work: &mut [c32], lwork: i32,
              rwork: &mut [f32], iwork: &mut [i32], ifail: &mut i32, info: &mut i32) {

    unsafe {
        ffi::chegvx_(itype.as_ptr(), &(jobz as c_char), &(range as c_char), &(uplo as c_char), &n,
                     a.as_mut_ptr() as *mut _, &lda, b.as_mut_ptr() as *mut _, &ldb, &vl, &vu, &il,
                     &iu, &abstol, m, w.as_mut_ptr(), z.as_mut_ptr() as *mut _, &ldz,
                     work.as_mut_ptr() as *mut _, &lwork, rwork.as_mut_ptr(), iwork.as_mut_ptr(),
                     ifail, info)
    }
}

#[inline]
pub fn zhegvx(itype: &[i32], jobz: u8, range: u8, uplo: u8, n: i32, a: &mut [c64], lda: i32,
              b: &mut [c64], ldb: i32, vl: f64, vu: f64, il: i32, iu: i32, abstol: f64,
              m: &mut i32, w: &mut [f64], z: &mut [c64], ldz: i32, work: &mut [c64], lwork: i32,
              rwork: &mut [f64], iwork: &mut [i32], ifail: &mut i32, info: &mut i32) {

    unsafe {
        ffi::zhegvx_(itype.as_ptr(), &(jobz as c_char), &(range as c_char), &(uplo as c_char), &n,
                     a.as_mut_ptr() as *mut _, &lda, b.as_mut_ptr() as *mut _, &ldb, &vl, &vu, &il,
                     &iu, &abstol, m, w.as_mut_ptr(), z.as_mut_ptr() as *mut _, &ldz,
                     work.as_mut_ptr() as *mut _, &lwork, rwork.as_mut_ptr(), iwork.as_mut_ptr(),
                     ifail, info)
    }
}

#[inline]
pub fn sspgv(itype: &[i32], jobz: u8, uplo: u8, n: i32, ap: &mut [f32], bp: &mut [f32],
             w: &mut [f32], z: &mut [f32], ldz: i32, work: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::sspgv_(itype.as_ptr(), &(jobz as c_char), &(uplo as c_char), &n, ap.as_mut_ptr(),
                    bp.as_mut_ptr(), w.as_mut_ptr(), z.as_mut_ptr(), &ldz, work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dspgv(itype: &[i32], jobz: u8, uplo: u8, n: i32, ap: &mut [f64], bp: &mut [f64],
             w: &mut [f64], z: &mut [f64], ldz: i32, work: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::dspgv_(itype.as_ptr(), &(jobz as c_char), &(uplo as c_char), &n, ap.as_mut_ptr(),
                    bp.as_mut_ptr(), w.as_mut_ptr(), z.as_mut_ptr(), &ldz, work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn chpgv(itype: &[i32], jobz: u8, uplo: u8, n: i32, ap: &mut [c32], bp: &mut [c32],
             w: &mut [f32], z: &mut [c32], ldz: i32, work: &mut [c32], rwork: &mut [f32],
             info: &mut i32) {

    unsafe {
        ffi::chpgv_(itype.as_ptr(), &(jobz as c_char), &(uplo as c_char), &n,
                    ap.as_mut_ptr() as *mut _, bp.as_mut_ptr() as *mut _, w.as_mut_ptr(),
                    z.as_mut_ptr() as *mut _, &ldz, work.as_mut_ptr() as *mut _,
                    rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zhpgv(itype: &[i32], jobz: u8, uplo: u8, n: i32, ap: &mut [c64], bp: &mut [c64],
             w: &mut [f64], z: &mut [c64], ldz: i32, work: &mut [c64], rwork: &mut [f64],
             info: &mut i32) {

    unsafe {
        ffi::zhpgv_(itype.as_ptr(), &(jobz as c_char), &(uplo as c_char), &n,
                    ap.as_mut_ptr() as *mut _, bp.as_mut_ptr() as *mut _, w.as_mut_ptr(),
                    z.as_mut_ptr() as *mut _, &ldz, work.as_mut_ptr() as *mut _,
                    rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn sspgvd(itype: &[i32], jobz: u8, uplo: u8, n: i32, ap: &mut [f32], bp: &mut [f32],
              w: &mut [f32], z: &mut [f32], ldz: i32, work: &mut [f32], lwork: i32,
              iwork: &mut [i32], liwork: i32, info: &mut i32) {

    unsafe {
        ffi::sspgvd_(itype.as_ptr(), &(jobz as c_char), &(uplo as c_char), &n, ap.as_mut_ptr(),
                     bp.as_mut_ptr(), w.as_mut_ptr(), z.as_mut_ptr(), &ldz, work.as_mut_ptr(),
                     &lwork, iwork.as_mut_ptr(), &liwork, info)
    }
}

#[inline]
pub fn dspgvd(itype: &[i32], jobz: u8, uplo: u8, n: i32, ap: &mut [f64], bp: &mut [f64],
              w: &mut [f64], z: &mut [f64], ldz: i32, work: &mut [f64], lwork: i32,
              iwork: &mut [i32], liwork: i32, info: &mut i32) {

    unsafe {
        ffi::dspgvd_(itype.as_ptr(), &(jobz as c_char), &(uplo as c_char), &n, ap.as_mut_ptr(),
                     bp.as_mut_ptr(), w.as_mut_ptr(), z.as_mut_ptr(), &ldz, work.as_mut_ptr(),
                     &lwork, iwork.as_mut_ptr(), &liwork, info)
    }
}

#[inline]
pub fn chpgvd(itype: &[i32], jobz: u8, uplo: u8, n: i32, ap: &mut [c32], bp: &mut [c32],
              w: &mut [f32], z: &mut [c32], ldz: i32, work: &mut [c32], lwork: i32,
              rwork: &mut [f32], lrwork: i32, iwork: &mut [i32], liwork: i32, info: &mut i32) {

    unsafe {
        ffi::chpgvd_(itype.as_ptr(), &(jobz as c_char), &(uplo as c_char), &n,
                     ap.as_mut_ptr() as *mut _, bp.as_mut_ptr() as *mut _, w.as_mut_ptr(),
                     z.as_mut_ptr() as *mut _, &ldz, work.as_mut_ptr() as *mut _, &lwork,
                     rwork.as_mut_ptr(), &lrwork, iwork.as_mut_ptr(), &liwork, info)
    }
}

#[inline]
pub fn zhpgvd(itype: &[i32], jobz: u8, uplo: u8, n: i32, ap: &mut [c64], bp: &mut [c64],
              w: &mut [f64], z: &mut [c64], ldz: i32, work: &mut [c64], lwork: i32,
              rwork: &mut [f64], lrwork: i32, iwork: &mut [i32], liwork: i32, info: &mut i32) {

    unsafe {
        ffi::zhpgvd_(itype.as_ptr(), &(jobz as c_char), &(uplo as c_char), &n,
                     ap.as_mut_ptr() as *mut _, bp.as_mut_ptr() as *mut _, w.as_mut_ptr(),
                     z.as_mut_ptr() as *mut _, &ldz, work.as_mut_ptr() as *mut _, &lwork,
                     rwork.as_mut_ptr(), &lrwork, iwork.as_mut_ptr(), &liwork, info)
    }
}

#[inline]
pub fn sspgvx(itype: &[i32], jobz: u8, range: u8, uplo: u8, n: i32, ap: &mut [f32], bp: &mut [f32],
              vl: f32, vu: f32, il: i32, iu: i32, abstol: f32, m: &mut i32, w: &mut [f32],
              z: &mut [f32], ldz: i32, work: &mut [f32], iwork: &mut [i32], ifail: &mut i32,
              info: &mut i32) {

    unsafe {
        ffi::sspgvx_(itype.as_ptr(), &(jobz as c_char), &(range as c_char), &(uplo as c_char), &n,
                     ap.as_mut_ptr(), bp.as_mut_ptr(), &vl, &vu, &il, &iu, &abstol, m,
                     w.as_mut_ptr(), z.as_mut_ptr(), &ldz, work.as_mut_ptr(), iwork.as_mut_ptr(),
                     ifail, info)
    }
}

#[inline]
pub fn dspgvx(itype: &[i32], jobz: u8, range: u8, uplo: u8, n: i32, ap: &mut [f64], bp: &mut [f64],
              vl: f64, vu: f64, il: i32, iu: i32, abstol: f64, m: &mut i32, w: &mut [f64],
              z: &mut [f64], ldz: i32, work: &mut [f64], iwork: &mut [i32], ifail: &mut i32,
              info: &mut i32) {

    unsafe {
        ffi::dspgvx_(itype.as_ptr(), &(jobz as c_char), &(range as c_char), &(uplo as c_char), &n,
                     ap.as_mut_ptr(), bp.as_mut_ptr(), &vl, &vu, &il, &iu, &abstol, m,
                     w.as_mut_ptr(), z.as_mut_ptr(), &ldz, work.as_mut_ptr(), iwork.as_mut_ptr(),
                     ifail, info)
    }
}

#[inline]
pub fn chpgvx(itype: &[i32], jobz: u8, range: u8, uplo: u8, n: i32, ap: &mut [c32], bp: &mut [c32],
              vl: f32, vu: f32, il: i32, iu: i32, abstol: f32, m: &mut i32, w: &mut [f32],
              z: &mut [c32], ldz: i32, work: &mut [c32], rwork: &mut [f32], iwork: &mut [i32],
              ifail: &mut i32, info: &mut i32) {

    unsafe {
        ffi::chpgvx_(itype.as_ptr(), &(jobz as c_char), &(range as c_char), &(uplo as c_char), &n,
                     ap.as_mut_ptr() as *mut _, bp.as_mut_ptr() as *mut _, &vl, &vu, &il, &iu,
                     &abstol, m, w.as_mut_ptr(), z.as_mut_ptr() as *mut _, &ldz,
                     work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), iwork.as_mut_ptr(), ifail,
                     info)
    }
}

#[inline]
pub fn zhpgvx(itype: &[i32], jobz: u8, range: u8, uplo: u8, n: i32, ap: &mut [c64], bp: &mut [c64],
              vl: f64, vu: f64, il: i32, iu: i32, abstol: f64, m: &mut i32, w: &mut [f64],
              z: &mut [c64], ldz: i32, work: &mut [c64], rwork: &mut [f64], iwork: &mut [i32],
              ifail: &mut i32, info: &mut i32) {

    unsafe {
        ffi::zhpgvx_(itype.as_ptr(), &(jobz as c_char), &(range as c_char), &(uplo as c_char), &n,
                     ap.as_mut_ptr() as *mut _, bp.as_mut_ptr() as *mut _, &vl, &vu, &il, &iu,
                     &abstol, m, w.as_mut_ptr(), z.as_mut_ptr() as *mut _, &ldz,
                     work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), iwork.as_mut_ptr(), ifail,
                     info)
    }
}

#[inline]
pub fn ssbgv(jobz: u8, uplo: u8, n: i32, ka: i32, kb: i32, ab: &mut [f32], ldab: i32,
             bb: &mut [f32], ldbb: i32, w: &mut [f32], z: &mut [f32], ldz: i32, work: &mut [f32],
             info: &mut i32) {

    unsafe {
        ffi::ssbgv_(&(jobz as c_char), &(uplo as c_char), &n, &ka, &kb, ab.as_mut_ptr(), &ldab,
                    bb.as_mut_ptr(), &ldbb, w.as_mut_ptr(), z.as_mut_ptr(), &ldz,
                    work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dsbgv(jobz: u8, uplo: u8, n: i32, ka: i32, kb: i32, ab: &mut [f64], ldab: i32,
             bb: &mut [f64], ldbb: i32, w: &mut [f64], z: &mut [f64], ldz: i32, work: &mut [f64],
             info: &mut i32) {

    unsafe {
        ffi::dsbgv_(&(jobz as c_char), &(uplo as c_char), &n, &ka, &kb, ab.as_mut_ptr(), &ldab,
                    bb.as_mut_ptr(), &ldbb, w.as_mut_ptr(), z.as_mut_ptr(), &ldz,
                    work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn chbgv(jobz: u8, uplo: u8, n: i32, ka: i32, kb: i32, ab: &mut [c32], ldab: i32,
             bb: &mut [c32], ldbb: i32, w: &mut [f32], z: &mut [c32], ldz: i32, work: &mut [c32],
             rwork: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::chbgv_(&(jobz as c_char), &(uplo as c_char), &n, &ka, &kb, ab.as_mut_ptr() as *mut _,
                    &ldab, bb.as_mut_ptr() as *mut _, &ldbb, w.as_mut_ptr(),
                    z.as_mut_ptr() as *mut _, &ldz, work.as_mut_ptr() as *mut _,
                    rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zhbgv(jobz: u8, uplo: u8, n: i32, ka: i32, kb: i32, ab: &mut [c64], ldab: i32,
             bb: &mut [c64], ldbb: i32, w: &mut [f64], z: &mut [c64], ldz: i32, work: &mut [c64],
             rwork: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::zhbgv_(&(jobz as c_char), &(uplo as c_char), &n, &ka, &kb, ab.as_mut_ptr() as *mut _,
                    &ldab, bb.as_mut_ptr() as *mut _, &ldbb, w.as_mut_ptr(),
                    z.as_mut_ptr() as *mut _, &ldz, work.as_mut_ptr() as *mut _,
                    rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn ssbgvd(jobz: u8, uplo: u8, n: i32, ka: i32, kb: i32, ab: &mut [f32], ldab: i32,
              bb: &mut [f32], ldbb: i32, w: &mut [f32], z: &mut [f32], ldz: i32, work: &mut [f32],
              lwork: i32, iwork: &mut [i32], liwork: i32, info: &mut i32) {

    unsafe {
        ffi::ssbgvd_(&(jobz as c_char), &(uplo as c_char), &n, &ka, &kb, ab.as_mut_ptr(), &ldab,
                     bb.as_mut_ptr(), &ldbb, w.as_mut_ptr(), z.as_mut_ptr(), &ldz,
                     work.as_mut_ptr(), &lwork, iwork.as_mut_ptr(), &liwork, info)
    }
}

#[inline]
pub fn dsbgvd(jobz: u8, uplo: u8, n: i32, ka: i32, kb: i32, ab: &mut [f64], ldab: i32,
              bb: &mut [f64], ldbb: i32, w: &mut [f64], z: &mut [f64], ldz: i32, work: &mut [f64],
              lwork: i32, iwork: &mut [i32], liwork: i32, info: &mut i32) {

    unsafe {
        ffi::dsbgvd_(&(jobz as c_char), &(uplo as c_char), &n, &ka, &kb, ab.as_mut_ptr(), &ldab,
                     bb.as_mut_ptr(), &ldbb, w.as_mut_ptr(), z.as_mut_ptr(), &ldz,
                     work.as_mut_ptr(), &lwork, iwork.as_mut_ptr(), &liwork, info)
    }
}

#[inline]
pub fn chbgvd(jobz: u8, uplo: u8, n: i32, ka: i32, kb: i32, ab: &mut [c32], ldab: i32,
              bb: &mut [c32], ldbb: i32, w: &mut [f32], z: &mut [c32], ldz: i32, work: &mut [c32],
              lwork: i32, rwork: &mut [f32], lrwork: i32, iwork: &mut [i32], liwork: i32,
              info: &mut i32) {

    unsafe {
        ffi::chbgvd_(&(jobz as c_char), &(uplo as c_char), &n, &ka, &kb, ab.as_mut_ptr() as *mut _,
                     &ldab, bb.as_mut_ptr() as *mut _, &ldbb, w.as_mut_ptr(),
                     z.as_mut_ptr() as *mut _, &ldz, work.as_mut_ptr() as *mut _, &lwork,
                     rwork.as_mut_ptr(), &lrwork, iwork.as_mut_ptr(), &liwork, info)
    }
}

#[inline]
pub fn zhbgvd(jobz: u8, uplo: u8, n: i32, ka: i32, kb: i32, ab: &mut [c64], ldab: i32,
              bb: &mut [c64], ldbb: i32, w: &mut [f64], z: &mut [c64], ldz: i32, work: &mut [c64],
              lwork: i32, rwork: &mut [f64], lrwork: i32, iwork: &mut [i32], liwork: i32,
              info: &mut i32) {

    unsafe {
        ffi::zhbgvd_(&(jobz as c_char), &(uplo as c_char), &n, &ka, &kb, ab.as_mut_ptr() as *mut _,
                     &ldab, bb.as_mut_ptr() as *mut _, &ldbb, w.as_mut_ptr(),
                     z.as_mut_ptr() as *mut _, &ldz, work.as_mut_ptr() as *mut _, &lwork,
                     rwork.as_mut_ptr(), &lrwork, iwork.as_mut_ptr(), &liwork, info)
    }
}

#[inline]
pub fn ssbgvx(jobz: u8, range: u8, uplo: u8, n: i32, ka: i32, kb: i32, ab: &mut [f32], ldab: i32,
              bb: &mut [f32], ldbb: i32, q: &mut f32, ldq: i32, vl: f32, vu: f32, il: i32, iu: i32,
              abstol: f32, m: &mut i32, w: &mut [f32], z: &mut [f32], ldz: i32, work: &mut [f32],
              iwork: &mut [i32], ifail: &mut i32, info: &mut i32) {

    unsafe {
        ffi::ssbgvx_(&(jobz as c_char), &(range as c_char), &(uplo as c_char), &n, &ka, &kb,
                     ab.as_mut_ptr(), &ldab, bb.as_mut_ptr(), &ldbb, q, &ldq, &vl, &vu, &il, &iu,
                     &abstol, m, w.as_mut_ptr(), z.as_mut_ptr(), &ldz, work.as_mut_ptr(),
                     iwork.as_mut_ptr(), ifail, info)
    }
}

#[inline]
pub fn dsbgvx(jobz: u8, range: u8, uplo: u8, n: i32, ka: i32, kb: i32, ab: &mut [f64], ldab: i32,
              bb: &mut [f64], ldbb: i32, q: &mut f64, ldq: i32, vl: f64, vu: f64, il: i32, iu: i32,
              abstol: f64, m: &mut i32, w: &mut [f64], z: &mut [f64], ldz: i32, work: &mut [f64],
              iwork: &mut [i32], ifail: &mut i32, info: &mut i32) {

    unsafe {
        ffi::dsbgvx_(&(jobz as c_char), &(range as c_char), &(uplo as c_char), &n, &ka, &kb,
                     ab.as_mut_ptr(), &ldab, bb.as_mut_ptr(), &ldbb, q, &ldq, &vl, &vu, &il, &iu,
                     &abstol, m, w.as_mut_ptr(), z.as_mut_ptr(), &ldz, work.as_mut_ptr(),
                     iwork.as_mut_ptr(), ifail, info)
    }
}

#[inline]
pub fn chbgvx(jobz: u8, range: u8, uplo: u8, n: i32, ka: i32, kb: i32, ab: &mut [c32], ldab: i32,
              bb: &mut [c32], ldbb: i32, q: &mut c32, ldq: i32, vl: f32, vu: f32, il: i32, iu: i32,
              abstol: f32, m: &mut i32, w: &mut [f32], z: &mut [c32], ldz: i32, work: &mut [c32],
              rwork: &mut [f32], iwork: &mut [i32], ifail: &mut i32, info: &mut i32) {

    unsafe {
        ffi::chbgvx_(&(jobz as c_char), &(range as c_char), &(uplo as c_char), &n, &ka, &kb,
                     ab.as_mut_ptr() as *mut _, &ldab, bb.as_mut_ptr() as *mut _, &ldbb,
                     q as *mut _ as *mut _, &ldq, &vl, &vu, &il, &iu, &abstol, m, w.as_mut_ptr(),
                     z.as_mut_ptr() as *mut _, &ldz, work.as_mut_ptr() as *mut _,
                     rwork.as_mut_ptr(), iwork.as_mut_ptr(), ifail, info)
    }
}

#[inline]
pub fn zhbgvx(jobz: u8, range: u8, uplo: u8, n: i32, ka: i32, kb: i32, ab: &mut [c64], ldab: i32,
              bb: &mut [c64], ldbb: i32, q: &mut c64, ldq: i32, vl: f64, vu: f64, il: i32, iu: i32,
              abstol: f64, m: &mut i32, w: &mut [f64], z: &mut [c64], ldz: i32, work: &mut [c64],
              rwork: &mut [f64], iwork: &mut [i32], ifail: &mut i32, info: &mut i32) {

    unsafe {
        ffi::zhbgvx_(&(jobz as c_char), &(range as c_char), &(uplo as c_char), &n, &ka, &kb,
                     ab.as_mut_ptr() as *mut _, &ldab, bb.as_mut_ptr() as *mut _, &ldbb,
                     q as *mut _ as *mut _, &ldq, &vl, &vu, &il, &iu, &abstol, m, w.as_mut_ptr(),
                     z.as_mut_ptr() as *mut _, &ldz, work.as_mut_ptr() as *mut _,
                     rwork.as_mut_ptr(), iwork.as_mut_ptr(), ifail, info)
    }
}

#[inline]
pub fn sgges(jobvsl: u8, jobvsr: u8, sort: u8, selctg: Select3F32, n: i32, a: &mut [f32], lda: i32,
             b: &mut [f32], ldb: i32, sdim: &mut [i32], alphar: &mut f32, alphai: &mut f32,
             beta: &mut f32, vsl: &mut [f32], ldvsl: i32, vsr: &mut [f32], ldvsr: i32,
             work: &mut [f32], lwork: i32, bwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::sgges_(&(jobvsl as c_char), &(jobvsr as c_char), &(sort as c_char), transmute(selctg),
                    &n, a.as_mut_ptr(), &lda, b.as_mut_ptr(), &ldb, sdim.as_mut_ptr(), alphar,
                    alphai, beta, vsl.as_mut_ptr(), &ldvsl, vsr.as_mut_ptr(), &ldvsr,
                    work.as_mut_ptr(), &lwork, bwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dgges(jobvsl: u8, jobvsr: u8, sort: u8, selctg: Select3F64, n: i32, a: &mut [f64], lda: i32,
             b: &mut [f64], ldb: i32, sdim: &mut [i32], alphar: &mut f64, alphai: &mut f64,
             beta: &mut f64, vsl: &mut [f64], ldvsl: i32, vsr: &mut [f64], ldvsr: i32,
             work: &mut [f64], lwork: i32, bwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::dgges_(&(jobvsl as c_char), &(jobvsr as c_char), &(sort as c_char), transmute(selctg),
                    &n, a.as_mut_ptr(), &lda, b.as_mut_ptr(), &ldb, sdim.as_mut_ptr(), alphar,
                    alphai, beta, vsl.as_mut_ptr(), &ldvsl, vsr.as_mut_ptr(), &ldvsr,
                    work.as_mut_ptr(), &lwork, bwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn cgges(jobvsl: u8, jobvsr: u8, sort: u8, selctg: Select2C32, n: i32, a: &mut [c32], lda: i32,
             b: &mut [c32], ldb: i32, sdim: &mut [i32], alpha: &mut c32, beta: &mut c32,
             vsl: &mut [c32], ldvsl: i32, vsr: &mut [c32], ldvsr: i32, work: &mut [c32],
             lwork: i32, rwork: &mut [f32], bwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::cgges_(&(jobvsl as c_char), &(jobvsr as c_char), &(sort as c_char), transmute(selctg),
                    &n, a.as_mut_ptr() as *mut _, &lda, b.as_mut_ptr() as *mut _, &ldb,
                    sdim.as_mut_ptr(), alpha as *mut _ as *mut _, beta as *mut _ as *mut _,
                    vsl.as_mut_ptr() as *mut _, &ldvsl, vsr.as_mut_ptr() as *mut _, &ldvsr,
                    work.as_mut_ptr() as *mut _, &lwork, rwork.as_mut_ptr(), bwork.as_mut_ptr(),
                    info)
    }
}

#[inline]
pub fn zgges(jobvsl: u8, jobvsr: u8, sort: u8, selctg: Select2C64, n: i32, a: &mut [c64], lda: i32,
             b: &mut [c64], ldb: i32, sdim: &mut [i32], alpha: &mut c64, beta: &mut c64,
             vsl: &mut [c64], ldvsl: i32, vsr: &mut [c64], ldvsr: i32, work: &mut [c64],
             lwork: i32, rwork: &mut [f64], bwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::zgges_(&(jobvsl as c_char), &(jobvsr as c_char), &(sort as c_char), transmute(selctg),
                    &n, a.as_mut_ptr() as *mut _, &lda, b.as_mut_ptr() as *mut _, &ldb,
                    sdim.as_mut_ptr(), alpha as *mut _ as *mut _, beta as *mut _ as *mut _,
                    vsl.as_mut_ptr() as *mut _, &ldvsl, vsr.as_mut_ptr() as *mut _, &ldvsr,
                    work.as_mut_ptr() as *mut _, &lwork, rwork.as_mut_ptr(), bwork.as_mut_ptr(),
                    info)
    }
}

#[inline]
pub fn sgges3(jobvsl: u8, jobvsr: u8, sort: u8, selctg: Select3F32, n: i32, a: &mut [f32],
              lda: i32, b: &mut [f32], ldb: i32, sdim: &mut [i32], alphar: &mut f32,
              alphai: &mut f32, beta: &mut f32, vsl: &mut [f32], ldvsl: i32, vsr: &mut [f32],
              ldvsr: i32, work: &mut [f32], lwork: i32, bwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::sgges3_(&(jobvsl as c_char), &(jobvsr as c_char), &(sort as c_char),
                     transmute(selctg), &n, a.as_mut_ptr(), &lda, b.as_mut_ptr(), &ldb,
                     sdim.as_mut_ptr(), alphar, alphai, beta, vsl.as_mut_ptr(), &ldvsl,
                     vsr.as_mut_ptr(), &ldvsr, work.as_mut_ptr(), &lwork, bwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dgges3(jobvsl: u8, jobvsr: u8, sort: u8, selctg: Select3F64, n: i32, a: &mut [f64],
              lda: i32, b: &mut [f64], ldb: i32, sdim: &mut [i32], alphar: &mut f64,
              alphai: &mut f64, beta: &mut f64, vsl: &mut [f64], ldvsl: i32, vsr: &mut [f64],
              ldvsr: i32, work: &mut [f64], lwork: i32, bwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::dgges3_(&(jobvsl as c_char), &(jobvsr as c_char), &(sort as c_char),
                     transmute(selctg), &n, a.as_mut_ptr(), &lda, b.as_mut_ptr(), &ldb,
                     sdim.as_mut_ptr(), alphar, alphai, beta, vsl.as_mut_ptr(), &ldvsl,
                     vsr.as_mut_ptr(), &ldvsr, work.as_mut_ptr(), &lwork, bwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn cgges3(jobvsl: u8, jobvsr: u8, sort: u8, selctg: Select2C32, n: i32, a: &mut [c32],
              lda: i32, b: &mut [c32], ldb: i32, sdim: &mut [i32], alpha: &mut c32, beta: &mut c32,
              vsl: &mut [c32], ldvsl: i32, vsr: &mut [c32], ldvsr: i32, work: &mut [c32],
              lwork: i32, rwork: &mut [f32], bwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::cgges3_(&(jobvsl as c_char), &(jobvsr as c_char), &(sort as c_char),
                     transmute(selctg), &n, a.as_mut_ptr() as *mut _, &lda,
                     b.as_mut_ptr() as *mut _, &ldb, sdim.as_mut_ptr(), alpha as *mut _ as *mut _,
                     beta as *mut _ as *mut _, vsl.as_mut_ptr() as *mut _, &ldvsl,
                     vsr.as_mut_ptr() as *mut _, &ldvsr, work.as_mut_ptr() as *mut _, &lwork,
                     rwork.as_mut_ptr(), bwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zgges3(jobvsl: u8, jobvsr: u8, sort: u8, selctg: Select2C64, n: i32, a: &mut [c64],
              lda: i32, b: &mut [c64], ldb: i32, sdim: &mut [i32], alpha: &mut c64, beta: &mut c64,
              vsl: &mut [c64], ldvsl: i32, vsr: &mut [c64], ldvsr: i32, work: &mut [c64],
              lwork: i32, rwork: &mut [f64], bwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::zgges3_(&(jobvsl as c_char), &(jobvsr as c_char), &(sort as c_char),
                     transmute(selctg), &n, a.as_mut_ptr() as *mut _, &lda,
                     b.as_mut_ptr() as *mut _, &ldb, sdim.as_mut_ptr(), alpha as *mut _ as *mut _,
                     beta as *mut _ as *mut _, vsl.as_mut_ptr() as *mut _, &ldvsl,
                     vsr.as_mut_ptr() as *mut _, &ldvsr, work.as_mut_ptr() as *mut _, &lwork,
                     rwork.as_mut_ptr(), bwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn sggesx(jobvsl: u8, jobvsr: u8, sort: u8, selctg: Select3F32, sense: u8, n: i32,
              a: &mut [f32], lda: i32, b: &mut [f32], ldb: i32, sdim: &mut [i32], alphar: &mut f32,
              alphai: &mut f32, beta: &mut f32, vsl: &mut [f32], ldvsl: i32, vsr: &mut [f32],
              ldvsr: i32, rconde: &mut [f32], rcondv: &mut [f32], work: &mut [f32], lwork: i32,
              iwork: &mut [i32], liwork: i32, bwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::sggesx_(&(jobvsl as c_char), &(jobvsr as c_char), &(sort as c_char),
                     transmute(selctg), &(sense as c_char), &n, a.as_mut_ptr(), &lda,
                     b.as_mut_ptr(), &ldb, sdim.as_mut_ptr(), alphar, alphai, beta,
                     vsl.as_mut_ptr(), &ldvsl, vsr.as_mut_ptr(), &ldvsr, rconde.as_mut_ptr(),
                     rcondv.as_mut_ptr(), work.as_mut_ptr(), &lwork, iwork.as_mut_ptr(), &liwork,
                     bwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dggesx(jobvsl: u8, jobvsr: u8, sort: u8, selctg: Select3F64, sense: u8, n: i32,
              a: &mut [f64], lda: i32, b: &mut [f64], ldb: i32, sdim: &mut [i32], alphar: &mut f64,
              alphai: &mut f64, beta: &mut f64, vsl: &mut [f64], ldvsl: i32, vsr: &mut [f64],
              ldvsr: i32, rconde: &mut [f64], rcondv: &mut [f64], work: &mut [f64], lwork: i32,
              iwork: &mut [i32], liwork: i32, bwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::dggesx_(&(jobvsl as c_char), &(jobvsr as c_char), &(sort as c_char),
                     transmute(selctg), &(sense as c_char), &n, a.as_mut_ptr(), &lda,
                     b.as_mut_ptr(), &ldb, sdim.as_mut_ptr(), alphar, alphai, beta,
                     vsl.as_mut_ptr(), &ldvsl, vsr.as_mut_ptr(), &ldvsr, rconde.as_mut_ptr(),
                     rcondv.as_mut_ptr(), work.as_mut_ptr(), &lwork, iwork.as_mut_ptr(), &liwork,
                     bwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn cggesx(jobvsl: u8, jobvsr: u8, sort: u8, selctg: Select2C32, sense: u8, n: i32,
              a: &mut [c32], lda: i32, b: &mut [c32], ldb: i32, sdim: &mut [i32], alpha: &mut c32,
              beta: &mut c32, vsl: &mut [c32], ldvsl: i32, vsr: &mut [c32], ldvsr: i32,
              rconde: &mut [f32], rcondv: &mut [f32], work: &mut [c32], lwork: i32,
              rwork: &mut [f32], iwork: &mut [i32], liwork: i32, bwork: &mut [i32],
              info: &mut i32) {

    unsafe {
        ffi::cggesx_(&(jobvsl as c_char), &(jobvsr as c_char), &(sort as c_char),
                     transmute(selctg), &(sense as c_char), &n, a.as_mut_ptr() as *mut _, &lda,
                     b.as_mut_ptr() as *mut _, &ldb, sdim.as_mut_ptr(), alpha as *mut _ as *mut _,
                     beta as *mut _ as *mut _, vsl.as_mut_ptr() as *mut _, &ldvsl,
                     vsr.as_mut_ptr() as *mut _, &ldvsr, rconde.as_mut_ptr(), rcondv.as_mut_ptr(),
                     work.as_mut_ptr() as *mut _, &lwork, rwork.as_mut_ptr(), iwork.as_mut_ptr(),
                     &liwork, bwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zggesx(jobvsl: u8, jobvsr: u8, sort: u8, selctg: Select2C64, sense: u8, n: i32,
              a: &mut [c64], lda: i32, b: &mut [c64], ldb: i32, sdim: &mut [i32], alpha: &mut c64,
              beta: &mut c64, vsl: &mut [c64], ldvsl: i32, vsr: &mut [c64], ldvsr: i32,
              rconde: &mut [f64], rcondv: &mut [f64], work: &mut [c64], lwork: i32,
              rwork: &mut [f64], iwork: &mut [i32], liwork: i32, bwork: &mut [i32],
              info: &mut i32) {

    unsafe {
        ffi::zggesx_(&(jobvsl as c_char), &(jobvsr as c_char), &(sort as c_char),
                     transmute(selctg), &(sense as c_char), &n, a.as_mut_ptr() as *mut _, &lda,
                     b.as_mut_ptr() as *mut _, &ldb, sdim.as_mut_ptr(), alpha as *mut _ as *mut _,
                     beta as *mut _ as *mut _, vsl.as_mut_ptr() as *mut _, &ldvsl,
                     vsr.as_mut_ptr() as *mut _, &ldvsr, rconde.as_mut_ptr(), rcondv.as_mut_ptr(),
                     work.as_mut_ptr() as *mut _, &lwork, rwork.as_mut_ptr(), iwork.as_mut_ptr(),
                     &liwork, bwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn sggev(jobvl: u8, jobvr: u8, n: i32, a: &mut [f32], lda: i32, b: &mut [f32], ldb: i32,
             alphar: &mut f32, alphai: &mut f32, beta: &mut f32, vl: &mut f32, ldvl: i32,
             vr: &mut f32, ldvr: i32, work: &mut [f32], lwork: i32, info: &mut i32) {

    unsafe {
        ffi::sggev_(&(jobvl as c_char), &(jobvr as c_char), &n, a.as_mut_ptr(), &lda,
                    b.as_mut_ptr(), &ldb, alphar, alphai, beta, vl, &ldvl, vr, &ldvr,
                    work.as_mut_ptr(), &lwork, info)
    }
}

#[inline]
pub fn dggev(jobvl: u8, jobvr: u8, n: i32, a: &mut [f64], lda: i32, b: &mut [f64], ldb: i32,
             alphar: &mut f64, alphai: &mut f64, beta: &mut f64, vl: &mut f64, ldvl: i32,
             vr: &mut f64, ldvr: i32, work: &mut [f64], lwork: i32, info: &mut i32) {

    unsafe {
        ffi::dggev_(&(jobvl as c_char), &(jobvr as c_char), &n, a.as_mut_ptr(), &lda,
                    b.as_mut_ptr(), &ldb, alphar, alphai, beta, vl, &ldvl, vr, &ldvr,
                    work.as_mut_ptr(), &lwork, info)
    }
}

#[inline]
pub fn cggev(jobvl: u8, jobvr: u8, n: i32, a: &mut [c32], lda: i32, b: &mut [c32], ldb: i32,
             alpha: &mut c32, beta: &mut c32, vl: &mut c32, ldvl: i32, vr: &mut c32, ldvr: i32,
             work: &mut [c32], lwork: i32, rwork: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::cggev_(&(jobvl as c_char), &(jobvr as c_char), &n, a.as_mut_ptr() as *mut _, &lda,
                    b.as_mut_ptr() as *mut _, &ldb, alpha as *mut _ as *mut _,
                    beta as *mut _ as *mut _, vl as *mut _ as *mut _, &ldvl,
                    vr as *mut _ as *mut _, &ldvr, work.as_mut_ptr() as *mut _, &lwork,
                    rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zggev(jobvl: u8, jobvr: u8, n: i32, a: &mut [c64], lda: i32, b: &mut [c64], ldb: i32,
             alpha: &mut c64, beta: &mut c64, vl: &mut c64, ldvl: i32, vr: &mut c64, ldvr: i32,
             work: &mut [c64], lwork: i32, rwork: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::zggev_(&(jobvl as c_char), &(jobvr as c_char), &n, a.as_mut_ptr() as *mut _, &lda,
                    b.as_mut_ptr() as *mut _, &ldb, alpha as *mut _ as *mut _,
                    beta as *mut _ as *mut _, vl as *mut _ as *mut _, &ldvl,
                    vr as *mut _ as *mut _, &ldvr, work.as_mut_ptr() as *mut _, &lwork,
                    rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn sggev3(jobvl: u8, jobvr: u8, n: i32, a: &mut [f32], lda: i32, b: &mut [f32], ldb: i32,
              alphar: &mut f32, alphai: &mut f32, beta: &mut f32, vl: &mut f32, ldvl: i32,
              vr: &mut f32, ldvr: i32, work: &mut [f32], lwork: i32, info: &mut i32) {

    unsafe {
        ffi::sggev3_(&(jobvl as c_char), &(jobvr as c_char), &n, a.as_mut_ptr(), &lda,
                     b.as_mut_ptr(), &ldb, alphar, alphai, beta, vl, &ldvl, vr, &ldvr,
                     work.as_mut_ptr(), &lwork, info)
    }
}

#[inline]
pub fn dggev3(jobvl: u8, jobvr: u8, n: i32, a: &mut [f64], lda: i32, b: &mut [f64], ldb: i32,
              alphar: &mut f64, alphai: &mut f64, beta: &mut f64, vl: &mut f64, ldvl: i32,
              vr: &mut f64, ldvr: i32, work: &mut [f64], lwork: i32, info: &mut i32) {

    unsafe {
        ffi::dggev3_(&(jobvl as c_char), &(jobvr as c_char), &n, a.as_mut_ptr(), &lda,
                     b.as_mut_ptr(), &ldb, alphar, alphai, beta, vl, &ldvl, vr, &ldvr,
                     work.as_mut_ptr(), &lwork, info)
    }
}

#[inline]
pub fn cggev3(jobvl: u8, jobvr: u8, n: i32, a: &mut [c32], lda: i32, b: &mut [c32], ldb: i32,
              alpha: &mut c32, beta: &mut c32, vl: &mut c32, ldvl: i32, vr: &mut c32, ldvr: i32,
              work: &mut [c32], lwork: i32, rwork: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::cggev3_(&(jobvl as c_char), &(jobvr as c_char), &n, a.as_mut_ptr() as *mut _, &lda,
                     b.as_mut_ptr() as *mut _, &ldb, alpha as *mut _ as *mut _,
                     beta as *mut _ as *mut _, vl as *mut _ as *mut _, &ldvl,
                     vr as *mut _ as *mut _, &ldvr, work.as_mut_ptr() as *mut _, &lwork,
                     rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zggev3(jobvl: u8, jobvr: u8, n: i32, a: &mut [c64], lda: i32, b: &mut [c64], ldb: i32,
              alpha: &mut c64, beta: &mut c64, vl: &mut c64, ldvl: i32, vr: &mut c64, ldvr: i32,
              work: &mut [c64], lwork: i32, rwork: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::zggev3_(&(jobvl as c_char), &(jobvr as c_char), &n, a.as_mut_ptr() as *mut _, &lda,
                     b.as_mut_ptr() as *mut _, &ldb, alpha as *mut _ as *mut _,
                     beta as *mut _ as *mut _, vl as *mut _ as *mut _, &ldvl,
                     vr as *mut _ as *mut _, &ldvr, work.as_mut_ptr() as *mut _, &lwork,
                     rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn sggevx(balanc: u8, jobvl: u8, jobvr: u8, sense: u8, n: i32, a: &mut [f32], lda: i32,
              b: &mut [f32], ldb: i32, alphar: &mut f32, alphai: &mut f32, beta: &mut f32,
              vl: &mut f32, ldvl: i32, vr: &mut f32, ldvr: i32, ilo: &mut i32, ihi: &mut i32,
              lscale: &mut [f32], rscale: &mut [f32], abnrm: &mut [f32], bbnrm: &mut [f32],
              rconde: &mut [f32], rcondv: &mut [f32], work: &mut [f32], lwork: i32,
              iwork: &mut [i32], bwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::sggevx_(&(balanc as c_char), &(jobvl as c_char), &(jobvr as c_char),
                     &(sense as c_char), &n, a.as_mut_ptr(), &lda, b.as_mut_ptr(), &ldb, alphar,
                     alphai, beta, vl, &ldvl, vr, &ldvr, ilo, ihi, lscale.as_mut_ptr(),
                     rscale.as_mut_ptr(), abnrm.as_mut_ptr(), bbnrm.as_mut_ptr(),
                     rconde.as_mut_ptr(), rcondv.as_mut_ptr(), work.as_mut_ptr(), &lwork,
                     iwork.as_mut_ptr(), bwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dggevx(balanc: u8, jobvl: u8, jobvr: u8, sense: u8, n: i32, a: &mut [f64], lda: i32,
              b: &mut [f64], ldb: i32, alphar: &mut f64, alphai: &mut f64, beta: &mut f64,
              vl: &mut f64, ldvl: i32, vr: &mut f64, ldvr: i32, ilo: &mut i32, ihi: &mut i32,
              lscale: &mut [f64], rscale: &mut [f64], abnrm: &mut [f64], bbnrm: &mut [f64],
              rconde: &mut [f64], rcondv: &mut [f64], work: &mut [f64], lwork: i32,
              iwork: &mut [i32], bwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::dggevx_(&(balanc as c_char), &(jobvl as c_char), &(jobvr as c_char),
                     &(sense as c_char), &n, a.as_mut_ptr(), &lda, b.as_mut_ptr(), &ldb, alphar,
                     alphai, beta, vl, &ldvl, vr, &ldvr, ilo, ihi, lscale.as_mut_ptr(),
                     rscale.as_mut_ptr(), abnrm.as_mut_ptr(), bbnrm.as_mut_ptr(),
                     rconde.as_mut_ptr(), rcondv.as_mut_ptr(), work.as_mut_ptr(), &lwork,
                     iwork.as_mut_ptr(), bwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn cggevx(balanc: u8, jobvl: u8, jobvr: u8, sense: u8, n: i32, a: &mut [c32], lda: i32,
              b: &mut [c32], ldb: i32, alpha: &mut c32, beta: &mut c32, vl: &mut c32, ldvl: i32,
              vr: &mut c32, ldvr: i32, ilo: &mut i32, ihi: &mut i32, lscale: &mut [f32],
              rscale: &mut [f32], abnrm: &mut [f32], bbnrm: &mut [f32], rconde: &mut [f32],
              rcondv: &mut [f32], work: &mut [c32], lwork: i32, rwork: &mut [f32],
              iwork: &mut [i32], bwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::cggevx_(&(balanc as c_char), &(jobvl as c_char), &(jobvr as c_char),
                     &(sense as c_char), &n, a.as_mut_ptr() as *mut _, &lda,
                     b.as_mut_ptr() as *mut _, &ldb, alpha as *mut _ as *mut _,
                     beta as *mut _ as *mut _, vl as *mut _ as *mut _, &ldvl,
                     vr as *mut _ as *mut _, &ldvr, ilo, ihi, lscale.as_mut_ptr(),
                     rscale.as_mut_ptr(), abnrm.as_mut_ptr(), bbnrm.as_mut_ptr(),
                     rconde.as_mut_ptr(), rcondv.as_mut_ptr(), work.as_mut_ptr() as *mut _, &lwork,
                     rwork.as_mut_ptr(), iwork.as_mut_ptr(), bwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zggevx(balanc: u8, jobvl: u8, jobvr: u8, sense: u8, n: i32, a: &mut [c64], lda: i32,
              b: &mut [c64], ldb: i32, alpha: &mut c64, beta: &mut c64, vl: &mut c64, ldvl: i32,
              vr: &mut c64, ldvr: i32, ilo: &mut i32, ihi: &mut i32, lscale: &mut [f64],
              rscale: &mut [f64], abnrm: &mut [f64], bbnrm: &mut [f64], rconde: &mut [f64],
              rcondv: &mut [f64], work: &mut [c64], lwork: i32, rwork: &mut [f64],
              iwork: &mut [i32], bwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::zggevx_(&(balanc as c_char), &(jobvl as c_char), &(jobvr as c_char),
                     &(sense as c_char), &n, a.as_mut_ptr() as *mut _, &lda,
                     b.as_mut_ptr() as *mut _, &ldb, alpha as *mut _ as *mut _,
                     beta as *mut _ as *mut _, vl as *mut _ as *mut _, &ldvl,
                     vr as *mut _ as *mut _, &ldvr, ilo, ihi, lscale.as_mut_ptr(),
                     rscale.as_mut_ptr(), abnrm.as_mut_ptr(), bbnrm.as_mut_ptr(),
                     rconde.as_mut_ptr(), rcondv.as_mut_ptr(), work.as_mut_ptr() as *mut _, &lwork,
                     rwork.as_mut_ptr(), iwork.as_mut_ptr(), bwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dsfrk(transr: u8, uplo: u8, trans: u8, n: i32, k: i32, alpha: f64, a: &[f64], lda: i32,
             beta: f64, c: &mut [f64]) {

    unsafe {
        ffi::dsfrk_(&(transr as c_char), &(uplo as c_char), &(trans as c_char), &n, &k, &alpha,
                    a.as_ptr(), &lda, &beta, c.as_mut_ptr())
    }
}

#[inline]
pub fn ssfrk(transr: u8, uplo: u8, trans: u8, n: i32, k: i32, alpha: f32, a: &[f32], lda: i32,
             beta: f32, c: &mut [f32]) {

    unsafe {
        ffi::ssfrk_(&(transr as c_char), &(uplo as c_char), &(trans as c_char), &n, &k, &alpha,
                    a.as_ptr(), &lda, &beta, c.as_mut_ptr())
    }
}

#[inline]
pub fn zhfrk(transr: u8, uplo: u8, trans: u8, n: i32, k: i32, alpha: f64, a: &[c64], lda: i32,
             beta: f64, c: &mut [c64]) {

    unsafe {
        ffi::zhfrk_(&(transr as c_char), &(uplo as c_char), &(trans as c_char), &n, &k, &alpha,
                    a.as_ptr() as *const _, &lda, &beta, c.as_mut_ptr() as *mut _)
    }
}

#[inline]
pub fn chfrk(transr: u8, uplo: u8, trans: u8, n: i32, k: i32, alpha: f32, a: &[c32], lda: i32,
             beta: f32, c: &mut [c32]) {

    unsafe {
        ffi::chfrk_(&(transr as c_char), &(uplo as c_char), &(trans as c_char), &n, &k, &alpha,
                    a.as_ptr() as *const _, &lda, &beta, c.as_mut_ptr() as *mut _)
    }
}

#[inline]
pub fn dtfsm(transr: u8, side: u8, uplo: u8, trans: u8, diag: u8, m: i32, n: i32, alpha: f64,
             a: &[f64], b: &mut [f64], ldb: i32) {

    unsafe {
        ffi::dtfsm_(&(transr as c_char), &(side as c_char), &(uplo as c_char), &(trans as c_char),
                    &(diag as c_char), &m, &n, &alpha, a.as_ptr(), b.as_mut_ptr(), &ldb)
    }
}

#[inline]
pub fn stfsm(transr: u8, side: u8, uplo: u8, trans: u8, diag: u8, m: i32, n: i32, alpha: f32,
             a: &[f32], b: &mut [f32], ldb: i32) {

    unsafe {
        ffi::stfsm_(&(transr as c_char), &(side as c_char), &(uplo as c_char), &(trans as c_char),
                    &(diag as c_char), &m, &n, &alpha, a.as_ptr(), b.as_mut_ptr(), &ldb)
    }
}

#[inline]
pub fn ztfsm(transr: u8, side: u8, uplo: u8, trans: u8, diag: u8, m: i32, n: i32, alpha: c64,
             a: &[c64], b: &mut [c64], ldb: i32) {

    unsafe {
        ffi::ztfsm_(&(transr as c_char), &(side as c_char), &(uplo as c_char), &(trans as c_char),
                    &(diag as c_char), &m, &n, &alpha as *const _ as *const _,
                    a.as_ptr() as *const _, b.as_mut_ptr() as *mut _, &ldb)
    }
}

#[inline]
pub fn ctfsm(transr: u8, side: u8, uplo: u8, trans: u8, diag: u8, m: i32, n: i32, alpha: c32,
             a: &[c32], b: &mut [c32], ldb: i32) {

    unsafe {
        ffi::ctfsm_(&(transr as c_char), &(side as c_char), &(uplo as c_char), &(trans as c_char),
                    &(diag as c_char), &m, &n, &alpha as *const _ as *const _,
                    a.as_ptr() as *const _, b.as_mut_ptr() as *mut _, &ldb)
    }
}

#[inline]
pub fn dtfttp(transr: u8, uplo: u8, n: i32, arf: &[f64], ap: &mut [f64], info: &mut i32) {
    unsafe {
        ffi::dtfttp_(&(transr as c_char), &(uplo as c_char), &n, arf.as_ptr(), ap.as_mut_ptr(),
                     info)
    }
}

#[inline]
pub fn stfttp(transr: u8, uplo: u8, n: i32, arf: &[f32], ap: &mut [f32], info: &mut i32) {
    unsafe {
        ffi::stfttp_(&(transr as c_char), &(uplo as c_char), &n, arf.as_ptr(), ap.as_mut_ptr(),
                     info)
    }
}

#[inline]
pub fn ztfttp(transr: u8, uplo: u8, n: i32, arf: &[c64], ap: &mut [c64], info: &mut i32) {
    unsafe {
        ffi::ztfttp_(&(transr as c_char), &(uplo as c_char), &n, arf.as_ptr() as *const _,
                     ap.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn ctfttp(transr: u8, uplo: u8, n: i32, arf: &[c32], ap: &mut [c32], info: &mut i32) {
    unsafe {
        ffi::ctfttp_(&(transr as c_char), &(uplo as c_char), &n, arf.as_ptr() as *const _,
                     ap.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn dtfttr(transr: u8, uplo: u8, n: i32, arf: &[f64], a: &mut [f64], lda: i32, info: &mut i32) {
    unsafe {
        ffi::dtfttr_(&(transr as c_char), &(uplo as c_char), &n, arf.as_ptr(), a.as_mut_ptr(),
                     &lda, info)
    }
}

#[inline]
pub fn stfttr(transr: u8, uplo: u8, n: i32, arf: &[f32], a: &mut [f32], lda: i32, info: &mut i32) {
    unsafe {
        ffi::stfttr_(&(transr as c_char), &(uplo as c_char), &n, arf.as_ptr(), a.as_mut_ptr(),
                     &lda, info)
    }
}

#[inline]
pub fn ztfttr(transr: u8, uplo: u8, n: i32, arf: &[c64], a: &mut [c64], lda: i32, info: &mut i32) {
    unsafe {
        ffi::ztfttr_(&(transr as c_char), &(uplo as c_char), &n, arf.as_ptr() as *const _,
                     a.as_mut_ptr() as *mut _, &lda, info)
    }
}

#[inline]
pub fn ctfttr(transr: u8, uplo: u8, n: i32, arf: &[c32], a: &mut [c32], lda: i32, info: &mut i32) {
    unsafe {
        ffi::ctfttr_(&(transr as c_char), &(uplo as c_char), &n, arf.as_ptr() as *const _,
                     a.as_mut_ptr() as *mut _, &lda, info)
    }
}

#[inline]
pub fn dtpttf(transr: u8, uplo: u8, n: i32, ap: &[f64], arf: &mut [f64], info: &mut i32) {
    unsafe {
        ffi::dtpttf_(&(transr as c_char), &(uplo as c_char), &n, ap.as_ptr(), arf.as_mut_ptr(),
                     info)
    }
}

#[inline]
pub fn stpttf(transr: u8, uplo: u8, n: i32, ap: &[f32], arf: &mut [f32], info: &mut i32) {
    unsafe {
        ffi::stpttf_(&(transr as c_char), &(uplo as c_char), &n, ap.as_ptr(), arf.as_mut_ptr(),
                     info)
    }
}

#[inline]
pub fn ztpttf(transr: u8, uplo: u8, n: i32, ap: &[c64], arf: &mut [c64], info: &mut i32) {
    unsafe {
        ffi::ztpttf_(&(transr as c_char), &(uplo as c_char), &n, ap.as_ptr() as *const _,
                     arf.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn ctpttf(transr: u8, uplo: u8, n: i32, ap: &[c32], arf: &mut [c32], info: &mut i32) {
    unsafe {
        ffi::ctpttf_(&(transr as c_char), &(uplo as c_char), &n, ap.as_ptr() as *const _,
                     arf.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn dtpttr(uplo: u8, n: i32, ap: &[f64], a: &mut [f64], lda: i32, info: &mut i32) {
    unsafe {
        ffi::dtpttr_(&(uplo as c_char), &n, ap.as_ptr(), a.as_mut_ptr(), &lda, info)
    }
}

#[inline]
pub fn stpttr(uplo: u8, n: i32, ap: &[f32], a: &mut [f32], lda: i32, info: &mut i32) {
    unsafe {
        ffi::stpttr_(&(uplo as c_char), &n, ap.as_ptr(), a.as_mut_ptr(), &lda, info)
    }
}

#[inline]
pub fn ztpttr(uplo: u8, n: i32, ap: &[c64], a: &mut [c64], lda: i32, info: &mut i32) {
    unsafe {
        ffi::ztpttr_(&(uplo as c_char), &n, ap.as_ptr() as *const _, a.as_mut_ptr() as *mut _,
                     &lda, info)
    }
}

#[inline]
pub fn ctpttr(uplo: u8, n: i32, ap: &[c32], a: &mut [c32], lda: i32, info: &mut i32) {
    unsafe {
        ffi::ctpttr_(&(uplo as c_char), &n, ap.as_ptr() as *const _, a.as_mut_ptr() as *mut _,
                     &lda, info)
    }
}

#[inline]
pub fn dtrttf(transr: u8, uplo: u8, n: i32, a: &[f64], lda: i32, arf: &mut [f64], info: &mut i32) {
    unsafe {
        ffi::dtrttf_(&(transr as c_char), &(uplo as c_char), &n, a.as_ptr(), &lda,
                     arf.as_mut_ptr(), info)
    }
}

#[inline]
pub fn strttf(transr: u8, uplo: u8, n: i32, a: &[f32], lda: i32, arf: &mut [f32], info: &mut i32) {
    unsafe {
        ffi::strttf_(&(transr as c_char), &(uplo as c_char), &n, a.as_ptr(), &lda,
                     arf.as_mut_ptr(), info)
    }
}

#[inline]
pub fn ztrttf(transr: u8, uplo: u8, n: i32, a: &[c64], lda: i32, arf: &mut [c64], info: &mut i32) {
    unsafe {
        ffi::ztrttf_(&(transr as c_char), &(uplo as c_char), &n, a.as_ptr() as *const _, &lda,
                     arf.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn ctrttf(transr: u8, uplo: u8, n: i32, a: &[c32], lda: i32, arf: &mut [c32], info: &mut i32) {
    unsafe {
        ffi::ctrttf_(&(transr as c_char), &(uplo as c_char), &n, a.as_ptr() as *const _, &lda,
                     arf.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn dtrttp(uplo: u8, n: i32, a: &[f64], lda: i32, ap: &mut [f64], info: &mut i32) {
    unsafe {
        ffi::dtrttp_(&(uplo as c_char), &n, a.as_ptr(), &lda, ap.as_mut_ptr(), info)
    }
}

#[inline]
pub fn strttp(uplo: u8, n: i32, a: &[f32], lda: i32, ap: &mut [f32], info: &mut i32) {
    unsafe {
        ffi::strttp_(&(uplo as c_char), &n, a.as_ptr(), &lda, ap.as_mut_ptr(), info)
    }
}

#[inline]
pub fn ztrttp(uplo: u8, n: i32, a: &[c64], lda: i32, ap: &mut [c64], info: &mut i32) {
    unsafe {
        ffi::ztrttp_(&(uplo as c_char), &n, a.as_ptr() as *const _, &lda,
                     ap.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn ctrttp(uplo: u8, n: i32, a: &[c32], lda: i32, ap: &mut [c32], info: &mut i32) {
    unsafe {
        ffi::ctrttp_(&(uplo as c_char), &n, a.as_ptr() as *const _, &lda,
                     ap.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn sgeqrfp(m: i32, n: i32, a: &mut [f32], lda: i32, tau: &mut [f32], work: &mut [f32],
               lwork: i32, info: &mut i32) {

    unsafe {
        ffi::sgeqrfp_(&m, &n, a.as_mut_ptr(), &lda, tau.as_mut_ptr(), work.as_mut_ptr(), &lwork,
                      info)
    }
}

#[inline]
pub fn dgeqrfp(m: i32, n: i32, a: &mut [f64], lda: i32, tau: &mut [f64], work: &mut [f64],
               lwork: i32, info: &mut i32) {

    unsafe {
        ffi::dgeqrfp_(&m, &n, a.as_mut_ptr(), &lda, tau.as_mut_ptr(), work.as_mut_ptr(), &lwork,
                      info)
    }
}

#[inline]
pub fn cgeqrfp(m: i32, n: i32, a: &mut [c32], lda: i32, tau: &mut [c32], work: &mut [c32],
               lwork: i32, info: &mut i32) {

    unsafe {
        ffi::cgeqrfp_(&m, &n, a.as_mut_ptr() as *mut _, &lda, tau.as_mut_ptr() as *mut _,
                      work.as_mut_ptr() as *mut _, &lwork, info)
    }
}

#[inline]
pub fn zgeqrfp(m: i32, n: i32, a: &mut [c64], lda: i32, tau: &mut [c64], work: &mut [c64],
               lwork: i32, info: &mut i32) {

    unsafe {
        ffi::zgeqrfp_(&m, &n, a.as_mut_ptr() as *mut _, &lda, tau.as_mut_ptr() as *mut _,
                      work.as_mut_ptr() as *mut _, &lwork, info)
    }
}

#[inline]
pub fn clacgv(n: i32, x: &mut [c32], incx: i32) {
    unsafe {
        ffi::clacgv_(&n, x.as_mut_ptr() as *mut _, &incx)
    }
}

#[inline]
pub fn zlacgv(n: i32, x: &mut [c64], incx: i32) {
    unsafe {
        ffi::zlacgv_(&n, x.as_mut_ptr() as *mut _, &incx)
    }
}

#[inline]
pub fn slarnv(idist: &[i32], iseed: &mut [i32], n: i32, x: &mut [f32]) {
    unsafe {
        ffi::slarnv_(idist.as_ptr(), iseed.as_mut_ptr(), &n, x.as_mut_ptr())
    }
}

#[inline]
pub fn dlarnv(idist: &[i32], iseed: &mut [i32], n: i32, x: &mut [f64]) {
    unsafe {
        ffi::dlarnv_(idist.as_ptr(), iseed.as_mut_ptr(), &n, x.as_mut_ptr())
    }
}

#[inline]
pub fn clarnv(idist: &[i32], iseed: &mut [i32], n: i32, x: &mut [c32]) {
    unsafe {
        ffi::clarnv_(idist.as_ptr(), iseed.as_mut_ptr(), &n, x.as_mut_ptr() as *mut _)
    }
}

#[inline]
pub fn zlarnv(idist: &[i32], iseed: &mut [i32], n: i32, x: &mut [c64]) {
    unsafe {
        ffi::zlarnv_(idist.as_ptr(), iseed.as_mut_ptr(), &n, x.as_mut_ptr() as *mut _)
    }
}

#[inline]
pub fn sgeqr2(m: i32, n: i32, a: &mut [f32], lda: i32, tau: &mut [f32], work: &mut [f32],
              info: &mut i32) {

    unsafe {
        ffi::sgeqr2_(&m, &n, a.as_mut_ptr(), &lda, tau.as_mut_ptr(), work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dgeqr2(m: i32, n: i32, a: &mut [f64], lda: i32, tau: &mut [f64], work: &mut [f64],
              info: &mut i32) {

    unsafe {
        ffi::dgeqr2_(&m, &n, a.as_mut_ptr(), &lda, tau.as_mut_ptr(), work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn cgeqr2(m: i32, n: i32, a: &mut [c32], lda: i32, tau: &mut [c32], work: &mut [c32],
              info: &mut i32) {

    unsafe {
        ffi::cgeqr2_(&m, &n, a.as_mut_ptr() as *mut _, &lda, tau.as_mut_ptr() as *mut _,
                     work.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn zgeqr2(m: i32, n: i32, a: &mut [c64], lda: i32, tau: &mut [c64], work: &mut [c64],
              info: &mut i32) {

    unsafe {
        ffi::zgeqr2_(&m, &n, a.as_mut_ptr() as *mut _, &lda, tau.as_mut_ptr() as *mut _,
                     work.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn slacn2(n: i32, v: &mut [f32], x: &mut [f32], isgn: &mut [i32], est: &mut [f32],
              kase: &mut i32, isave: &mut [i32]) {

    unsafe {
        ffi::slacn2_(&n, v.as_mut_ptr(), x.as_mut_ptr(), isgn.as_mut_ptr(), est.as_mut_ptr(), kase,
                     isave.as_mut_ptr())
    }
}

#[inline]
pub fn dlacn2(n: i32, v: &mut [f64], x: &mut [f64], isgn: &mut [i32], est: &mut [f64],
              kase: &mut i32, isave: &mut [i32]) {

    unsafe {
        ffi::dlacn2_(&n, v.as_mut_ptr(), x.as_mut_ptr(), isgn.as_mut_ptr(), est.as_mut_ptr(), kase,
                     isave.as_mut_ptr())
    }
}

#[inline]
pub fn clacn2(n: i32, v: &mut [c32], x: &mut [c32], est: &mut [f32], kase: &mut i32,
              isave: &mut [i32]) {

    unsafe {
        ffi::clacn2_(&n, v.as_mut_ptr() as *mut _, x.as_mut_ptr() as *mut _, est.as_mut_ptr(),
                     kase, isave.as_mut_ptr())
    }
}

#[inline]
pub fn zlacn2(n: i32, v: &mut [c64], x: &mut [c64], est: &mut [f64], kase: &mut i32,
              isave: &mut [i32]) {

    unsafe {
        ffi::zlacn2_(&n, v.as_mut_ptr() as *mut _, x.as_mut_ptr() as *mut _, est.as_mut_ptr(),
                     kase, isave.as_mut_ptr())
    }
}

#[inline]
pub fn slacpy(uplo: u8, m: i32, n: i32, a: &[f32], lda: i32, b: &mut [f32], ldb: i32) {
    unsafe {
        ffi::slacpy_(&(uplo as c_char), &m, &n, a.as_ptr(), &lda, b.as_mut_ptr(), &ldb)
    }
}

#[inline]
pub fn dlacpy(uplo: u8, m: i32, n: i32, a: &[f64], lda: i32, b: &mut [f64], ldb: i32) {
    unsafe {
        ffi::dlacpy_(&(uplo as c_char), &m, &n, a.as_ptr(), &lda, b.as_mut_ptr(), &ldb)
    }
}

#[inline]
pub fn clacpy(uplo: u8, m: i32, n: i32, a: &[c32], lda: i32, b: &mut [c32], ldb: i32) {
    unsafe {
        ffi::clacpy_(&(uplo as c_char), &m, &n, a.as_ptr() as *const _, &lda,
                     b.as_mut_ptr() as *mut _, &ldb)
    }
}

#[inline]
pub fn zlacpy(uplo: u8, m: i32, n: i32, a: &[c64], lda: i32, b: &mut [c64], ldb: i32) {
    unsafe {
        ffi::zlacpy_(&(uplo as c_char), &m, &n, a.as_ptr() as *const _, &lda,
                     b.as_mut_ptr() as *mut _, &ldb)
    }
}

#[inline]
pub fn clacp2(uplo: u8, m: i32, n: i32, a: &[f32], lda: i32, b: &mut [c32], ldb: i32) {
    unsafe {
        ffi::clacp2_(&(uplo as c_char), &m, &n, a.as_ptr(), &lda, b.as_mut_ptr() as *mut _, &ldb)
    }
}

#[inline]
pub fn zlacp2(uplo: u8, m: i32, n: i32, a: &[f64], lda: i32, b: &mut [c64], ldb: i32) {
    unsafe {
        ffi::zlacp2_(&(uplo as c_char), &m, &n, a.as_ptr(), &lda, b.as_mut_ptr() as *mut _, &ldb)
    }
}

#[inline]
pub fn sgetf2(m: i32, n: i32, a: &mut [f32], lda: i32, ipiv: &mut [i32], info: &mut i32) {
    unsafe {
        ffi::sgetf2_(&m, &n, a.as_mut_ptr(), &lda, ipiv.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dgetf2(m: i32, n: i32, a: &mut [f64], lda: i32, ipiv: &mut [i32], info: &mut i32) {
    unsafe {
        ffi::dgetf2_(&m, &n, a.as_mut_ptr(), &lda, ipiv.as_mut_ptr(), info)
    }
}

#[inline]
pub fn cgetf2(m: i32, n: i32, a: &mut [c32], lda: i32, ipiv: &mut [i32], info: &mut i32) {
    unsafe {
        ffi::cgetf2_(&m, &n, a.as_mut_ptr() as *mut _, &lda, ipiv.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zgetf2(m: i32, n: i32, a: &mut [c64], lda: i32, ipiv: &mut [i32], info: &mut i32) {
    unsafe {
        ffi::zgetf2_(&m, &n, a.as_mut_ptr() as *mut _, &lda, ipiv.as_mut_ptr(), info)
    }
}

#[inline]
pub fn slaswp(n: i32, a: &mut [f32], lda: i32, k1: i32, k2: i32, ipiv: &[i32], incx: i32) {
    unsafe {
        ffi::slaswp_(&n, a.as_mut_ptr(), &lda, &k1, &k2, ipiv.as_ptr(), &incx)
    }
}

#[inline]
pub fn dlaswp(n: i32, a: &mut [f64], lda: i32, k1: i32, k2: i32, ipiv: &[i32], incx: i32) {
    unsafe {
        ffi::dlaswp_(&n, a.as_mut_ptr(), &lda, &k1, &k2, ipiv.as_ptr(), &incx)
    }
}

#[inline]
pub fn claswp(n: i32, a: &mut [c32], lda: i32, k1: i32, k2: i32, ipiv: &[i32], incx: i32) {
    unsafe {
        ffi::claswp_(&n, a.as_mut_ptr() as *mut _, &lda, &k1, &k2, ipiv.as_ptr(), &incx)
    }
}

#[inline]
pub fn zlaswp(n: i32, a: &mut [c64], lda: i32, k1: i32, k2: i32, ipiv: &[i32], incx: i32) {
    unsafe {
        ffi::zlaswp_(&n, a.as_mut_ptr() as *mut _, &lda, &k1, &k2, ipiv.as_ptr(), &incx)
    }
}

#[inline]
pub fn slange(norm: u8, m: i32, n: i32, a: &[f32], lda: i32, work: &mut [f32]) -> f32 {
    unsafe {
        ffi::slange_(&(norm as c_char), &m, &n, a.as_ptr(), &lda, work.as_mut_ptr()) as f32
    }
}

#[inline]
pub fn dlange(norm: u8, m: i32, n: i32, a: &[f64], lda: i32, work: &mut [f64]) -> f64 {
    unsafe {
        ffi::dlange_(&(norm as c_char), &m, &n, a.as_ptr(), &lda, work.as_mut_ptr()) as f64
    }
}

#[inline]
pub fn clange(norm: u8, m: i32, n: i32, a: &[c32], lda: i32, work: &mut [f32]) -> f32 {
    unsafe {
        ffi::clange_(&(norm as c_char), &m, &n, a.as_ptr() as *const _, &lda,
                     work.as_mut_ptr()) as f32
    }
}

#[inline]
pub fn zlange(norm: u8, m: i32, n: i32, a: &[c64], lda: i32, work: &mut [f64]) -> f64 {
    unsafe {
        ffi::zlange_(&(norm as c_char), &m, &n, a.as_ptr() as *const _, &lda,
                     work.as_mut_ptr()) as f64
    }
}

#[inline]
pub fn clanhe(norm: u8, uplo: u8, n: i32, a: &[c32], lda: i32, work: &mut [f32]) -> f32 {
    unsafe {
        ffi::clanhe_(&(norm as c_char), &(uplo as c_char), &n, a.as_ptr() as *const _, &lda,
                     work.as_mut_ptr()) as f32
    }
}

#[inline]
pub fn zlanhe(norm: u8, uplo: u8, n: i32, a: &[c64], lda: i32, work: &mut [f64]) -> f64 {
    unsafe {
        ffi::zlanhe_(&(norm as c_char), &(uplo as c_char), &n, a.as_ptr() as *const _, &lda,
                     work.as_mut_ptr()) as f64
    }
}

#[inline]
pub fn slansy(norm: u8, uplo: u8, n: i32, a: &[f32], lda: i32, work: &mut [f32]) -> f32 {
    unsafe {
        ffi::slansy_(&(norm as c_char), &(uplo as c_char), &n, a.as_ptr(), &lda,
                     work.as_mut_ptr()) as f32
    }
}

#[inline]
pub fn dlansy(norm: u8, uplo: u8, n: i32, a: &[f64], lda: i32, work: &mut [f64]) -> f64 {
    unsafe {
        ffi::dlansy_(&(norm as c_char), &(uplo as c_char), &n, a.as_ptr(), &lda,
                     work.as_mut_ptr()) as f64
    }
}

#[inline]
pub fn clansy(norm: u8, uplo: u8, n: i32, a: &[c32], lda: i32, work: &mut [f32]) -> f32 {
    unsafe {
        ffi::clansy_(&(norm as c_char), &(uplo as c_char), &n, a.as_ptr() as *const _, &lda,
                     work.as_mut_ptr()) as f32
    }
}

#[inline]
pub fn zlansy(norm: u8, uplo: u8, n: i32, a: &[c64], lda: i32, work: &mut [f64]) -> f64 {
    unsafe {
        ffi::zlansy_(&(norm as c_char), &(uplo as c_char), &n, a.as_ptr() as *const _, &lda,
                     work.as_mut_ptr()) as f64
    }
}

#[inline]
pub fn slantr(norm: u8, uplo: u8, diag: u8, m: i32, n: i32, a: &[f32], lda: i32,
              work: &mut [f32]) -> f32 {

    unsafe {
        ffi::slantr_(&(norm as c_char), &(uplo as c_char), &(diag as c_char), &m, &n, a.as_ptr(),
                     &lda, work.as_mut_ptr()) as f32
    }
}

#[inline]
pub fn dlantr(norm: u8, uplo: u8, diag: u8, m: i32, n: i32, a: &[f64], lda: i32,
              work: &mut [f64]) -> f64 {

    unsafe {
        ffi::dlantr_(&(norm as c_char), &(uplo as c_char), &(diag as c_char), &m, &n, a.as_ptr(),
                     &lda, work.as_mut_ptr()) as f64
    }
}

#[inline]
pub fn clantr(norm: u8, uplo: u8, diag: u8, m: i32, n: i32, a: &[c32], lda: i32,
              work: &mut [f32]) -> f32 {

    unsafe {
        ffi::clantr_(&(norm as c_char), &(uplo as c_char), &(diag as c_char), &m, &n,
                     a.as_ptr() as *const _, &lda, work.as_mut_ptr()) as f32
    }
}

#[inline]
pub fn zlantr(norm: u8, uplo: u8, diag: u8, m: i32, n: i32, a: &[c64], lda: i32,
              work: &mut [f64]) -> f64 {

    unsafe {
        ffi::zlantr_(&(norm as c_char), &(uplo as c_char), &(diag as c_char), &m, &n,
                     a.as_ptr() as *const _, &lda, work.as_mut_ptr()) as f64
    }
}

#[inline]
pub fn slamch(cmach: u8) -> f32 {
    unsafe {
        ffi::slamch_(&(cmach as c_char)) as f32
    }
}

#[inline]
pub fn dlamch(cmach: u8) -> f64 {
    unsafe {
        ffi::dlamch_(&(cmach as c_char)) as f64
    }
}

#[inline]
pub fn sgelq2(m: i32, n: i32, a: &mut [f32], lda: i32, tau: &mut [f32], work: &mut [f32],
              info: &mut i32) {

    unsafe {
        ffi::sgelq2_(&m, &n, a.as_mut_ptr(), &lda, tau.as_mut_ptr(), work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dgelq2(m: i32, n: i32, a: &mut [f64], lda: i32, tau: &mut [f64], work: &mut [f64],
              info: &mut i32) {

    unsafe {
        ffi::dgelq2_(&m, &n, a.as_mut_ptr(), &lda, tau.as_mut_ptr(), work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn cgelq2(m: i32, n: i32, a: &mut [c32], lda: i32, tau: &mut [c32], work: &mut [c32],
              info: &mut i32) {

    unsafe {
        ffi::cgelq2_(&m, &n, a.as_mut_ptr() as *mut _, &lda, tau.as_mut_ptr() as *mut _,
                     work.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn zgelq2(m: i32, n: i32, a: &mut [c64], lda: i32, tau: &mut [c64], work: &mut [c64],
              info: &mut i32) {

    unsafe {
        ffi::zgelq2_(&m, &n, a.as_mut_ptr() as *mut _, &lda, tau.as_mut_ptr() as *mut _,
                     work.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn slarfb(side: u8, trans: u8, direct: u8, storev: u8, m: i32, n: i32, k: i32, v: &[f32],
              ldv: i32, t: &[f32], ldt: i32, c: &mut [f32], ldc: i32, work: &mut [f32],
              ldwork: i32) {

    unsafe {
        ffi::slarfb_(&(side as c_char), &(trans as c_char), &(direct as c_char),
                     &(storev as c_char), &m, &n, &k, v.as_ptr(), &ldv, t.as_ptr(), &ldt,
                     c.as_mut_ptr(), &ldc, work.as_mut_ptr(), &ldwork)
    }
}

#[inline]
pub fn dlarfb(side: u8, trans: u8, direct: u8, storev: u8, m: i32, n: i32, k: i32, v: &[f64],
              ldv: i32, t: &[f64], ldt: i32, c: &mut [f64], ldc: i32, work: &mut [f64],
              ldwork: i32) {

    unsafe {
        ffi::dlarfb_(&(side as c_char), &(trans as c_char), &(direct as c_char),
                     &(storev as c_char), &m, &n, &k, v.as_ptr(), &ldv, t.as_ptr(), &ldt,
                     c.as_mut_ptr(), &ldc, work.as_mut_ptr(), &ldwork)
    }
}

#[inline]
pub fn clarfb(side: u8, trans: u8, direct: u8, storev: u8, m: i32, n: i32, k: i32, v: &[c32],
              ldv: i32, t: &[c32], ldt: i32, c: &mut [c32], ldc: i32, work: &mut [c32],
              ldwork: i32) {

    unsafe {
        ffi::clarfb_(&(side as c_char), &(trans as c_char), &(direct as c_char),
                     &(storev as c_char), &m, &n, &k, v.as_ptr() as *const _, &ldv,
                     t.as_ptr() as *const _, &ldt, c.as_mut_ptr() as *mut _, &ldc,
                     work.as_mut_ptr() as *mut _, &ldwork)
    }
}

#[inline]
pub fn zlarfb(side: u8, trans: u8, direct: u8, storev: u8, m: i32, n: i32, k: i32, v: &[c64],
              ldv: i32, t: &[c64], ldt: i32, c: &mut [c64], ldc: i32, work: &mut [c64],
              ldwork: i32) {

    unsafe {
        ffi::zlarfb_(&(side as c_char), &(trans as c_char), &(direct as c_char),
                     &(storev as c_char), &m, &n, &k, v.as_ptr() as *const _, &ldv,
                     t.as_ptr() as *const _, &ldt, c.as_mut_ptr() as *mut _, &ldc,
                     work.as_mut_ptr() as *mut _, &ldwork)
    }
}

#[inline]
pub fn slarfg(n: i32, alpha: &mut f32, x: &mut [f32], incx: i32, tau: &mut [f32]) {
    unsafe {
        ffi::slarfg_(&n, alpha, x.as_mut_ptr(), &incx, tau.as_mut_ptr())
    }
}

#[inline]
pub fn dlarfg(n: i32, alpha: &mut f64, x: &mut [f64], incx: i32, tau: &mut [f64]) {
    unsafe {
        ffi::dlarfg_(&n, alpha, x.as_mut_ptr(), &incx, tau.as_mut_ptr())
    }
}

#[inline]
pub fn clarfg(n: i32, alpha: &mut c32, x: &mut [c32], incx: i32, tau: &mut [c32]) {
    unsafe {
        ffi::clarfg_(&n, alpha as *mut _ as *mut _, x.as_mut_ptr() as *mut _, &incx,
                     tau.as_mut_ptr() as *mut _)
    }
}

#[inline]
pub fn zlarfg(n: i32, alpha: &mut c64, x: &mut [c64], incx: i32, tau: &mut [c64]) {
    unsafe {
        ffi::zlarfg_(&n, alpha as *mut _ as *mut _, x.as_mut_ptr() as *mut _, &incx,
                     tau.as_mut_ptr() as *mut _)
    }
}

#[inline]
pub fn slarft(direct: u8, storev: u8, n: i32, k: i32, v: &[f32], ldv: i32, tau: &[f32],
              t: &mut [f32], ldt: i32) {

    unsafe {
        ffi::slarft_(&(direct as c_char), &(storev as c_char), &n, &k, v.as_ptr(), &ldv,
                     tau.as_ptr(), t.as_mut_ptr(), &ldt)
    }
}

#[inline]
pub fn dlarft(direct: u8, storev: u8, n: i32, k: i32, v: &[f64], ldv: i32, tau: &[f64],
              t: &mut [f64], ldt: i32) {

    unsafe {
        ffi::dlarft_(&(direct as c_char), &(storev as c_char), &n, &k, v.as_ptr(), &ldv,
                     tau.as_ptr(), t.as_mut_ptr(), &ldt)
    }
}

#[inline]
pub fn clarft(direct: u8, storev: u8, n: i32, k: i32, v: &[c32], ldv: i32, tau: &[c32],
              t: &mut [c32], ldt: i32) {

    unsafe {
        ffi::clarft_(&(direct as c_char), &(storev as c_char), &n, &k, v.as_ptr() as *const _,
                     &ldv, tau.as_ptr() as *const _, t.as_mut_ptr() as *mut _, &ldt)
    }
}

#[inline]
pub fn zlarft(direct: u8, storev: u8, n: i32, k: i32, v: &[c64], ldv: i32, tau: &[c64],
              t: &mut [c64], ldt: i32) {

    unsafe {
        ffi::zlarft_(&(direct as c_char), &(storev as c_char), &n, &k, v.as_ptr() as *const _,
                     &ldv, tau.as_ptr() as *const _, t.as_mut_ptr() as *mut _, &ldt)
    }
}

#[inline]
pub fn slarfx(side: u8, m: i32, n: i32, v: &[f32], tau: &[f32], c: &mut [f32], ldc: i32,
              work: &mut [f32]) {

    unsafe {
        ffi::slarfx_(&(side as c_char), &m, &n, v.as_ptr(), tau.as_ptr(), c.as_mut_ptr(), &ldc,
                     work.as_mut_ptr())
    }
}

#[inline]
pub fn dlarfx(side: u8, m: i32, n: i32, v: &[f64], tau: &[f64], c: &mut [f64], ldc: i32,
              work: &mut [f64]) {

    unsafe {
        ffi::dlarfx_(&(side as c_char), &m, &n, v.as_ptr(), tau.as_ptr(), c.as_mut_ptr(), &ldc,
                     work.as_mut_ptr())
    }
}

#[inline]
pub fn clarfx(side: u8, m: i32, n: i32, v: &[c32], tau: &[c32], c: &mut [c32], ldc: i32,
              work: &mut [c32]) {

    unsafe {
        ffi::clarfx_(&(side as c_char), &m, &n, v.as_ptr() as *const _, tau.as_ptr() as *const _,
                     c.as_mut_ptr() as *mut _, &ldc, work.as_mut_ptr() as *mut _)
    }
}

#[inline]
pub fn zlarfx(side: u8, m: i32, n: i32, v: &[c64], tau: &[c64], c: &mut [c64], ldc: i32,
              work: &mut [c64]) {

    unsafe {
        ffi::zlarfx_(&(side as c_char), &m, &n, v.as_ptr() as *const _, tau.as_ptr() as *const _,
                     c.as_mut_ptr() as *mut _, &ldc, work.as_mut_ptr() as *mut _)
    }
}

#[inline]
pub fn slatms(m: i32, n: i32, dist: u8, iseed: &mut [i32], sym: u8, d: &mut [f32], mode: &[i32],
              cond: &[f32], dmax: &[f32], kl: i32, ku: i32, pack: u8, a: &mut [f32], lda: i32,
              work: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::slatms_(&m, &n, &(dist as c_char), iseed.as_mut_ptr(), &(sym as c_char),
                     d.as_mut_ptr(), mode.as_ptr(), cond.as_ptr(), dmax.as_ptr(), &kl, &ku,
                     &(pack as c_char), a.as_mut_ptr(), &lda, work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dlatms(m: i32, n: i32, dist: u8, iseed: &mut [i32], sym: u8, d: &mut [f64], mode: &[i32],
              cond: &[f64], dmax: &[f64], kl: i32, ku: i32, pack: u8, a: &mut [f64], lda: i32,
              work: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::dlatms_(&m, &n, &(dist as c_char), iseed.as_mut_ptr(), &(sym as c_char),
                     d.as_mut_ptr(), mode.as_ptr(), cond.as_ptr(), dmax.as_ptr(), &kl, &ku,
                     &(pack as c_char), a.as_mut_ptr(), &lda, work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn clatms(m: i32, n: i32, dist: u8, iseed: &mut [i32], sym: u8, d: &mut [f32], mode: &[i32],
              cond: &[f32], dmax: &[f32], kl: i32, ku: i32, pack: u8, a: &mut [c32], lda: i32,
              work: &mut [c32], info: &mut i32) {

    unsafe {
        ffi::clatms_(&m, &n, &(dist as c_char), iseed.as_mut_ptr(), &(sym as c_char),
                     d.as_mut_ptr(), mode.as_ptr(), cond.as_ptr(), dmax.as_ptr(), &kl, &ku,
                     &(pack as c_char), a.as_mut_ptr() as *mut _, &lda,
                     work.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn zlatms(m: i32, n: i32, dist: u8, iseed: &mut [i32], sym: u8, d: &mut [f64], mode: &[i32],
              cond: &[f64], dmax: &[f64], kl: i32, ku: i32, pack: u8, a: &mut [c64], lda: i32,
              work: &mut [c64], info: &mut i32) {

    unsafe {
        ffi::zlatms_(&m, &n, &(dist as c_char), iseed.as_mut_ptr(), &(sym as c_char),
                     d.as_mut_ptr(), mode.as_ptr(), cond.as_ptr(), dmax.as_ptr(), &kl, &ku,
                     &(pack as c_char), a.as_mut_ptr() as *mut _, &lda,
                     work.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn slag2d(m: i32, n: i32, sa: &[f32], ldsa: i32, a: &mut [f64], lda: i32, info: &mut i32) {
    unsafe {
        ffi::slag2d_(&m, &n, sa.as_ptr(), &ldsa, a.as_mut_ptr(), &lda, info)
    }
}

#[inline]
pub fn dlag2s(m: i32, n: i32, a: &[f64], lda: i32, sa: &mut [f32], ldsa: i32, info: &mut i32) {
    unsafe {
        ffi::dlag2s_(&m, &n, a.as_ptr(), &lda, sa.as_mut_ptr(), &ldsa, info)
    }
}

#[inline]
pub fn clag2z(m: i32, n: i32, sa: &[c32], ldsa: i32, a: &mut [c64], lda: i32, info: &mut i32) {
    unsafe {
        ffi::clag2z_(&m, &n, sa.as_ptr() as *const _, &ldsa, a.as_mut_ptr() as *mut _, &lda, info)
    }
}

#[inline]
pub fn zlag2c(m: i32, n: i32, a: &[c64], lda: i32, sa: &mut [c32], ldsa: i32, info: &mut i32) {
    unsafe {
        ffi::zlag2c_(&m, &n, a.as_ptr() as *const _, &lda, sa.as_mut_ptr() as *mut _, &ldsa, info)
    }
}

#[inline]
pub fn slauum(uplo: u8, n: i32, a: &mut [f32], lda: i32, info: &mut i32) {
    unsafe {
        ffi::slauum_(&(uplo as c_char), &n, a.as_mut_ptr(), &lda, info)
    }
}

#[inline]
pub fn dlauum(uplo: u8, n: i32, a: &mut [f64], lda: i32, info: &mut i32) {
    unsafe {
        ffi::dlauum_(&(uplo as c_char), &n, a.as_mut_ptr(), &lda, info)
    }
}

#[inline]
pub fn clauum(uplo: u8, n: i32, a: &mut [c32], lda: i32, info: &mut i32) {
    unsafe {
        ffi::clauum_(&(uplo as c_char), &n, a.as_mut_ptr() as *mut _, &lda, info)
    }
}

#[inline]
pub fn zlauum(uplo: u8, n: i32, a: &mut [c64], lda: i32, info: &mut i32) {
    unsafe {
        ffi::zlauum_(&(uplo as c_char), &n, a.as_mut_ptr() as *mut _, &lda, info)
    }
}

#[inline]
pub fn slagge(m: i32, n: i32, kl: i32, ku: i32, d: &[f32], a: &mut [f32], lda: i32,
              iseed: &mut [i32], work: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::slagge_(&m, &n, &kl, &ku, d.as_ptr(), a.as_mut_ptr(), &lda, iseed.as_mut_ptr(),
                     work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dlagge(m: i32, n: i32, kl: i32, ku: i32, d: &[f64], a: &mut [f64], lda: i32,
              iseed: &mut [i32], work: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::dlagge_(&m, &n, &kl, &ku, d.as_ptr(), a.as_mut_ptr(), &lda, iseed.as_mut_ptr(),
                     work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn clagge(m: i32, n: i32, kl: i32, ku: i32, d: &[f32], a: &mut [c32], lda: i32,
              iseed: &mut [i32], work: &mut [c32], info: &mut i32) {

    unsafe {
        ffi::clagge_(&m, &n, &kl, &ku, d.as_ptr(), a.as_mut_ptr() as *mut _, &lda,
                     iseed.as_mut_ptr(), work.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn zlagge(m: i32, n: i32, kl: i32, ku: i32, d: &[f64], a: &mut [c64], lda: i32,
              iseed: &mut [i32], work: &mut [c64], info: &mut i32) {

    unsafe {
        ffi::zlagge_(&m, &n, &kl, &ku, d.as_ptr(), a.as_mut_ptr() as *mut _, &lda,
                     iseed.as_mut_ptr(), work.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn slascl(_type: u8, kl: i32, ku: i32, cfrom: &[f32], cto: &[f32], m: i32, n: i32,
              a: &mut [f32], lda: i32, info: &mut i32) {

    unsafe {
        ffi::slascl_(&(_type as c_char), &kl, &ku, cfrom.as_ptr(), cto.as_ptr(), &m, &n,
                     a.as_mut_ptr(), &lda, info)
    }
}

#[inline]
pub fn dlascl(_type: u8, kl: i32, ku: i32, cfrom: &[f64], cto: &[f64], m: i32, n: i32,
              a: &mut [f64], lda: i32, info: &mut i32) {

    unsafe {
        ffi::dlascl_(&(_type as c_char), &kl, &ku, cfrom.as_ptr(), cto.as_ptr(), &m, &n,
                     a.as_mut_ptr(), &lda, info)
    }
}

#[inline]
pub fn clascl(_type: u8, kl: i32, ku: i32, cfrom: &[f32], cto: &[f32], m: i32, n: i32,
              a: &mut [c32], lda: i32, info: &mut i32) {

    unsafe {
        ffi::clascl_(&(_type as c_char), &kl, &ku, cfrom.as_ptr(), cto.as_ptr(), &m, &n,
                     a.as_mut_ptr() as *mut _, &lda, info)
    }
}

#[inline]
pub fn zlascl(_type: u8, kl: i32, ku: i32, cfrom: &[f64], cto: &[f64], m: i32, n: i32,
              a: &mut [c64], lda: i32, info: &mut i32) {

    unsafe {
        ffi::zlascl_(&(_type as c_char), &kl, &ku, cfrom.as_ptr(), cto.as_ptr(), &m, &n,
                     a.as_mut_ptr() as *mut _, &lda, info)
    }
}

#[inline]
pub fn slaset(uplo: u8, m: i32, n: i32, alpha: f32, beta: f32, a: &mut [f32], lda: i32) {
    unsafe {
        ffi::slaset_(&(uplo as c_char), &m, &n, &alpha, &beta, a.as_mut_ptr(), &lda)
    }
}

#[inline]
pub fn dlaset(uplo: u8, m: i32, n: i32, alpha: f64, beta: f64, a: &mut [f64], lda: i32) {
    unsafe {
        ffi::dlaset_(&(uplo as c_char), &m, &n, &alpha, &beta, a.as_mut_ptr(), &lda)
    }
}

#[inline]
pub fn claset(uplo: u8, m: i32, n: i32, alpha: c32, beta: c32, a: &mut [c32], lda: i32) {
    unsafe {
        ffi::claset_(&(uplo as c_char), &m, &n, &alpha as *const _ as *const _,
                     &beta as *const _ as *const _, a.as_mut_ptr() as *mut _, &lda)
    }
}

#[inline]
pub fn zlaset(uplo: u8, m: i32, n: i32, alpha: c64, beta: c64, a: &mut [c64], lda: i32) {
    unsafe {
        ffi::zlaset_(&(uplo as c_char), &m, &n, &alpha as *const _ as *const _,
                     &beta as *const _ as *const _, a.as_mut_ptr() as *mut _, &lda)
    }
}

#[inline]
pub fn slasrt(id: u8, n: i32, d: &mut [f32], info: &mut i32) {
    unsafe {
        ffi::slasrt_(&(id as c_char), &n, d.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dlasrt(id: u8, n: i32, d: &mut [f64], info: &mut i32) {
    unsafe {
        ffi::dlasrt_(&(id as c_char), &n, d.as_mut_ptr(), info)
    }
}

#[inline]
pub fn claghe(n: i32, k: i32, d: &[f32], a: &mut [c32], lda: i32, iseed: &mut [i32],
              work: &mut [c32], info: &mut i32) {

    unsafe {
        ffi::claghe_(&n, &k, d.as_ptr(), a.as_mut_ptr() as *mut _, &lda, iseed.as_mut_ptr(),
                     work.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn zlaghe(n: i32, k: i32, d: &[f64], a: &mut [c64], lda: i32, iseed: &mut [i32],
              work: &mut [c64], info: &mut i32) {

    unsafe {
        ffi::zlaghe_(&n, &k, d.as_ptr(), a.as_mut_ptr() as *mut _, &lda, iseed.as_mut_ptr(),
                     work.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn slagsy(n: i32, k: i32, d: &[f32], a: &mut [f32], lda: i32, iseed: &mut [i32],
              work: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::slagsy_(&n, &k, d.as_ptr(), a.as_mut_ptr(), &lda, iseed.as_mut_ptr(),
                     work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dlagsy(n: i32, k: i32, d: &[f64], a: &mut [f64], lda: i32, iseed: &mut [i32],
              work: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::dlagsy_(&n, &k, d.as_ptr(), a.as_mut_ptr(), &lda, iseed.as_mut_ptr(),
                     work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn clagsy(n: i32, k: i32, d: &[f32], a: &mut [c32], lda: i32, iseed: &mut [i32],
              work: &mut [c32], info: &mut i32) {

    unsafe {
        ffi::clagsy_(&n, &k, d.as_ptr(), a.as_mut_ptr() as *mut _, &lda, iseed.as_mut_ptr(),
                     work.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn zlagsy(n: i32, k: i32, d: &[f64], a: &mut [c64], lda: i32, iseed: &mut [i32],
              work: &mut [c64], info: &mut i32) {

    unsafe {
        ffi::zlagsy_(&n, &k, d.as_ptr(), a.as_mut_ptr() as *mut _, &lda, iseed.as_mut_ptr(),
                     work.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn slapmr(forwrd: &[i32], m: i32, n: i32, x: &mut [f32], ldx: i32, k: &mut i32) {
    unsafe {
        ffi::slapmr_(forwrd.as_ptr(), &m, &n, x.as_mut_ptr(), &ldx, k)
    }
}

#[inline]
pub fn dlapmr(forwrd: &[i32], m: i32, n: i32, x: &mut [f64], ldx: i32, k: &mut i32) {
    unsafe {
        ffi::dlapmr_(forwrd.as_ptr(), &m, &n, x.as_mut_ptr(), &ldx, k)
    }
}

#[inline]
pub fn clapmr(forwrd: &[i32], m: i32, n: i32, x: &mut [c32], ldx: i32, k: &mut i32) {
    unsafe {
        ffi::clapmr_(forwrd.as_ptr(), &m, &n, x.as_mut_ptr() as *mut _, &ldx, k)
    }
}

#[inline]
pub fn zlapmr(forwrd: &[i32], m: i32, n: i32, x: &mut [c64], ldx: i32, k: &mut i32) {
    unsafe {
        ffi::zlapmr_(forwrd.as_ptr(), &m, &n, x.as_mut_ptr() as *mut _, &ldx, k)
    }
}

#[inline]
pub fn slapmt(forwrd: &[i32], m: i32, n: i32, x: &mut [f32], ldx: i32, k: &mut i32) {
    unsafe {
        ffi::slapmt_(forwrd.as_ptr(), &m, &n, x.as_mut_ptr(), &ldx, k)
    }
}

#[inline]
pub fn dlapmt(forwrd: &[i32], m: i32, n: i32, x: &mut [f64], ldx: i32, k: &mut i32) {
    unsafe {
        ffi::dlapmt_(forwrd.as_ptr(), &m, &n, x.as_mut_ptr(), &ldx, k)
    }
}

#[inline]
pub fn clapmt(forwrd: &[i32], m: i32, n: i32, x: &mut [c32], ldx: i32, k: &mut i32) {
    unsafe {
        ffi::clapmt_(forwrd.as_ptr(), &m, &n, x.as_mut_ptr() as *mut _, &ldx, k)
    }
}

#[inline]
pub fn zlapmt(forwrd: &[i32], m: i32, n: i32, x: &mut [c64], ldx: i32, k: &mut i32) {
    unsafe {
        ffi::zlapmt_(forwrd.as_ptr(), &m, &n, x.as_mut_ptr() as *mut _, &ldx, k)
    }
}

#[inline]
pub fn slapy2(x: &[f32], y: &[f32]) -> f32 {
    unsafe {
        ffi::slapy2_(x.as_ptr(), y.as_ptr()) as f32
    }
}

#[inline]
pub fn dlapy2(x: &[f64], y: &[f64]) -> f64 {
    unsafe {
        ffi::dlapy2_(x.as_ptr(), y.as_ptr()) as f64
    }
}

#[inline]
pub fn slapy3(x: &[f32], y: &[f32], z: &[f32]) -> f32 {
    unsafe {
        ffi::slapy3_(x.as_ptr(), y.as_ptr(), z.as_ptr()) as f32
    }
}

#[inline]
pub fn dlapy3(x: &[f64], y: &[f64], z: &[f64]) -> f64 {
    unsafe {
        ffi::dlapy3_(x.as_ptr(), y.as_ptr(), z.as_ptr()) as f64
    }
}

#[inline]
pub fn slartgp(f: &[f32], g: &[f32], cs: &mut [f32], sn: &mut [f32], r: &mut [f32]) {
    unsafe {
        ffi::slartgp_(f.as_ptr(), g.as_ptr(), cs.as_mut_ptr(), sn.as_mut_ptr(), r.as_mut_ptr())
    }
}

#[inline]
pub fn dlartgp(f: &[f64], g: &[f64], cs: &mut [f64], sn: &mut [f64], r: &mut [f64]) {
    unsafe {
        ffi::dlartgp_(f.as_ptr(), g.as_ptr(), cs.as_mut_ptr(), sn.as_mut_ptr(), r.as_mut_ptr())
    }
}

#[inline]
pub fn slartgs(x: &[f32], y: &[f32], sigma: &[f32], cs: &mut [f32], sn: &mut [f32]) {
    unsafe {
        ffi::slartgs_(x.as_ptr(), y.as_ptr(), sigma.as_ptr(), cs.as_mut_ptr(), sn.as_mut_ptr())
    }
}

#[inline]
pub fn dlartgs(x: &[f64], y: &[f64], sigma: &[f64], cs: &mut [f64], sn: &mut [f64]) {
    unsafe {
        ffi::dlartgs_(x.as_ptr(), y.as_ptr(), sigma.as_ptr(), cs.as_mut_ptr(), sn.as_mut_ptr())
    }
}

#[inline]
pub fn cbbcsd(jobu1: u8, jobu2: u8, jobv1t: u8, jobv2t: u8, trans: u8, m: i32, p: i32, q: i32,
              theta: &mut [f32], phi: &mut [f32], u1: &mut [c32], ldu1: i32, u2: &mut [c32],
              ldu2: i32, v1t: &mut [c32], ldv1t: i32, v2t: &mut [c32], ldv2t: i32,
              b11d: &mut [f32], b11e: &mut [f32], b12d: &mut [f32], b12e: &mut [f32],
              b21d: &mut [f32], b21e: &mut [f32], b22d: &mut [f32], b22e: &mut [f32],
              rwork: &mut [f32], lrwork: i32, info: &mut i32) {

    unsafe {
        ffi::cbbcsd_(&(jobu1 as c_char), &(jobu2 as c_char), &(jobv1t as c_char),
                     &(jobv2t as c_char), &(trans as c_char), &m, &p, &q, theta.as_mut_ptr(),
                     phi.as_mut_ptr(), u1.as_mut_ptr() as *mut _, &ldu1, u2.as_mut_ptr() as *mut _,
                     &ldu2, v1t.as_mut_ptr() as *mut _, &ldv1t, v2t.as_mut_ptr() as *mut _, &ldv2t,
                     b11d.as_mut_ptr(), b11e.as_mut_ptr(), b12d.as_mut_ptr(), b12e.as_mut_ptr(),
                     b21d.as_mut_ptr(), b21e.as_mut_ptr(), b22d.as_mut_ptr(), b22e.as_mut_ptr(),
                     rwork.as_mut_ptr(), &lrwork, info)
    }
}

#[inline]
pub fn cheswapr(uplo: u8, n: i32, a: &mut [c32], i1: &[i32], i2: &[i32]) {
    unsafe {
        ffi::cheswapr_(&(uplo as c_char), &n, a.as_mut_ptr() as *mut _, i1.as_ptr(), i2.as_ptr())
    }
}

#[inline]
pub fn chetri2(uplo: u8, n: i32, a: &mut [c32], lda: i32, ipiv: &[i32], work: &mut [c32],
               lwork: i32, info: &mut i32) {

    unsafe {
        ffi::chetri2_(&(uplo as c_char), &n, a.as_mut_ptr() as *mut _, &lda, ipiv.as_ptr(),
                      work.as_mut_ptr() as *mut _, &lwork, info)
    }
}

#[inline]
pub fn chetri2x(uplo: u8, n: i32, a: &mut [c32], lda: i32, ipiv: &[i32], work: &mut [c32], nb: i32,
                info: &mut i32) {

    unsafe {
        ffi::chetri2x_(&(uplo as c_char), &n, a.as_mut_ptr() as *mut _, &lda, ipiv.as_ptr(),
                       work.as_mut_ptr() as *mut _, &nb, info)
    }
}

#[inline]
pub fn chetrs2(uplo: u8, n: i32, nrhs: i32, a: &[c32], lda: i32, ipiv: &[i32], b: &mut [c32],
               ldb: i32, work: &mut [c32], info: &mut i32) {

    unsafe {
        ffi::chetrs2_(&(uplo as c_char), &n, &nrhs, a.as_ptr() as *const _, &lda, ipiv.as_ptr(),
                      b.as_mut_ptr() as *mut _, &ldb, work.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn csyconv(uplo: u8, way: u8, n: i32, a: &mut [c32], lda: i32, ipiv: &[i32], work: &mut [c32],
               info: &mut i32) {

    unsafe {
        ffi::csyconv_(&(uplo as c_char), &(way as c_char), &n, a.as_mut_ptr() as *mut _, &lda,
                      ipiv.as_ptr(), work.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn csyswapr(uplo: u8, n: i32, a: &mut [c32], i1: &[i32], i2: &[i32]) {
    unsafe {
        ffi::csyswapr_(&(uplo as c_char), &n, a.as_mut_ptr() as *mut _, i1.as_ptr(), i2.as_ptr())
    }
}

#[inline]
pub fn csytri2(uplo: u8, n: i32, a: &mut [c32], lda: i32, ipiv: &[i32], work: &mut [c32],
               lwork: i32, info: &mut i32) {

    unsafe {
        ffi::csytri2_(&(uplo as c_char), &n, a.as_mut_ptr() as *mut _, &lda, ipiv.as_ptr(),
                      work.as_mut_ptr() as *mut _, &lwork, info)
    }
}

#[inline]
pub fn csytri2x(uplo: u8, n: i32, a: &mut [c32], lda: i32, ipiv: &[i32], work: &mut [c32], nb: i32,
                info: &mut i32) {

    unsafe {
        ffi::csytri2x_(&(uplo as c_char), &n, a.as_mut_ptr() as *mut _, &lda, ipiv.as_ptr(),
                       work.as_mut_ptr() as *mut _, &nb, info)
    }
}

#[inline]
pub fn csytrs2(uplo: u8, n: i32, nrhs: i32, a: &[c32], lda: i32, ipiv: &[i32], b: &mut [c32],
               ldb: i32, work: &mut [c32], info: &mut i32) {

    unsafe {
        ffi::csytrs2_(&(uplo as c_char), &n, &nrhs, a.as_ptr() as *const _, &lda, ipiv.as_ptr(),
                      b.as_mut_ptr() as *mut _, &ldb, work.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn cunbdb(trans: u8, signs: u8, m: i32, p: i32, q: i32, x11: &mut [c32], ldx11: i32,
              x12: &mut [c32], ldx12: i32, x21: &mut [c32], ldx21: i32, x22: &mut [c32],
              ldx22: i32, theta: &mut [f32], phi: &mut [f32], taup1: &mut [c32], taup2: &mut [c32],
              tauq1: &mut [c32], tauq2: &mut [c32], work: &mut [c32], lwork: i32, info: &mut i32) {

    unsafe {
        ffi::cunbdb_(&(trans as c_char), &(signs as c_char), &m, &p, &q,
                     x11.as_mut_ptr() as *mut _, &ldx11, x12.as_mut_ptr() as *mut _, &ldx12,
                     x21.as_mut_ptr() as *mut _, &ldx21, x22.as_mut_ptr() as *mut _, &ldx22,
                     theta.as_mut_ptr(), phi.as_mut_ptr(), taup1.as_mut_ptr() as *mut _,
                     taup2.as_mut_ptr() as *mut _, tauq1.as_mut_ptr() as *mut _,
                     tauq2.as_mut_ptr() as *mut _, work.as_mut_ptr() as *mut _, &lwork, info)
    }
}

#[inline]
pub fn cuncsd(jobu1: u8, jobu2: u8, jobv1t: u8, jobv2t: u8, trans: u8, signs: u8, m: i32, p: i32,
              q: i32, x11: &mut [c32], ldx11: i32, x12: &mut [c32], ldx12: i32, x21: &mut [c32],
              ldx21: i32, x22: &mut [c32], ldx22: i32, theta: &mut [f32], u1: &mut [c32],
              ldu1: i32, u2: &mut [c32], ldu2: i32, v1t: &mut [c32], ldv1t: i32, v2t: &mut [c32],
              ldv2t: i32, work: &mut [c32], lwork: i32, rwork: &mut [f32], lrwork: i32,
              iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::cuncsd_(&(jobu1 as c_char), &(jobu2 as c_char), &(jobv1t as c_char),
                     &(jobv2t as c_char), &(trans as c_char), &(signs as c_char), &m, &p, &q,
                     x11.as_mut_ptr() as *mut _, &ldx11, x12.as_mut_ptr() as *mut _, &ldx12,
                     x21.as_mut_ptr() as *mut _, &ldx21, x22.as_mut_ptr() as *mut _, &ldx22,
                     theta.as_mut_ptr(), u1.as_mut_ptr() as *mut _, &ldu1,
                     u2.as_mut_ptr() as *mut _, &ldu2, v1t.as_mut_ptr() as *mut _, &ldv1t,
                     v2t.as_mut_ptr() as *mut _, &ldv2t, work.as_mut_ptr() as *mut _, &lwork,
                     rwork.as_mut_ptr(), &lrwork, iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn cuncsd2by1(jobu1: u8, jobu2: u8, jobv1t: u8, m: i32, p: i32, q: i32, x11: &mut [c32],
                  ldx11: i32, x21: &mut [c32], ldx21: i32, theta: &mut [c32], u1: &mut [c32],
                  ldu1: i32, u2: &mut [c32], ldu2: i32, v1t: &mut [c32], ldv1t: i32,
                  work: &mut [c32], lwork: i32, rwork: &mut [f32], lrwork: i32, iwork: &mut [i32],
                  info: &mut i32) {

    unsafe {
        ffi::cuncsd2by1_(&(jobu1 as c_char), &(jobu2 as c_char), &(jobv1t as c_char), &m, &p, &q,
                         x11.as_mut_ptr() as *mut _, &ldx11, x21.as_mut_ptr() as *mut _, &ldx21,
                         theta.as_mut_ptr() as *mut _, u1.as_mut_ptr() as *mut _, &ldu1,
                         u2.as_mut_ptr() as *mut _, &ldu2, v1t.as_mut_ptr() as *mut _, &ldv1t,
                         work.as_mut_ptr() as *mut _, &lwork, rwork.as_mut_ptr(), &lrwork,
                         iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dbbcsd(jobu1: u8, jobu2: u8, jobv1t: u8, jobv2t: u8, trans: u8, m: i32, p: i32, q: i32,
              theta: &mut [f64], phi: &mut [f64], u1: &mut [f64], ldu1: i32, u2: &mut [f64],
              ldu2: i32, v1t: &mut [f64], ldv1t: i32, v2t: &mut [f64], ldv2t: i32,
              b11d: &mut [f64], b11e: &mut [f64], b12d: &mut [f64], b12e: &mut [f64],
              b21d: &mut [f64], b21e: &mut [f64], b22d: &mut [f64], b22e: &mut [f64],
              work: &mut [f64], lwork: i32, info: &mut i32) {

    unsafe {
        ffi::dbbcsd_(&(jobu1 as c_char), &(jobu2 as c_char), &(jobv1t as c_char),
                     &(jobv2t as c_char), &(trans as c_char), &m, &p, &q, theta.as_mut_ptr(),
                     phi.as_mut_ptr(), u1.as_mut_ptr(), &ldu1, u2.as_mut_ptr(), &ldu2,
                     v1t.as_mut_ptr(), &ldv1t, v2t.as_mut_ptr(), &ldv2t, b11d.as_mut_ptr(),
                     b11e.as_mut_ptr(), b12d.as_mut_ptr(), b12e.as_mut_ptr(), b21d.as_mut_ptr(),
                     b21e.as_mut_ptr(), b22d.as_mut_ptr(), b22e.as_mut_ptr(), work.as_mut_ptr(),
                     &lwork, info)
    }
}

#[inline]
pub fn dorbdb(trans: u8, signs: u8, m: i32, p: i32, q: i32, x11: &mut [f64], ldx11: i32,
              x12: &mut [f64], ldx12: i32, x21: &mut [f64], ldx21: i32, x22: &mut [f64],
              ldx22: i32, theta: &mut [f64], phi: &mut [f64], taup1: &mut [f64], taup2: &mut [f64],
              tauq1: &mut [f64], tauq2: &mut [f64], work: &mut [f64], lwork: i32, info: &mut i32) {

    unsafe {
        ffi::dorbdb_(&(trans as c_char), &(signs as c_char), &m, &p, &q, x11.as_mut_ptr(), &ldx11,
                     x12.as_mut_ptr(), &ldx12, x21.as_mut_ptr(), &ldx21, x22.as_mut_ptr(), &ldx22,
                     theta.as_mut_ptr(), phi.as_mut_ptr(), taup1.as_mut_ptr(), taup2.as_mut_ptr(),
                     tauq1.as_mut_ptr(), tauq2.as_mut_ptr(), work.as_mut_ptr(), &lwork, info)
    }
}

#[inline]
pub fn dorcsd(jobu1: u8, jobu2: u8, jobv1t: u8, jobv2t: u8, trans: u8, signs: u8, m: i32, p: i32,
              q: i32, x11: &mut [f64], ldx11: i32, x12: &mut [f64], ldx12: i32, x21: &mut [f64],
              ldx21: i32, x22: &mut [f64], ldx22: i32, theta: &mut [f64], u1: &mut [f64],
              ldu1: i32, u2: &mut [f64], ldu2: i32, v1t: &mut [f64], ldv1t: i32, v2t: &mut [f64],
              ldv2t: i32, work: &mut [f64], lwork: i32, iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::dorcsd_(&(jobu1 as c_char), &(jobu2 as c_char), &(jobv1t as c_char),
                     &(jobv2t as c_char), &(trans as c_char), &(signs as c_char), &m, &p, &q,
                     x11.as_mut_ptr(), &ldx11, x12.as_mut_ptr(), &ldx12, x21.as_mut_ptr(), &ldx21,
                     x22.as_mut_ptr(), &ldx22, theta.as_mut_ptr(), u1.as_mut_ptr(), &ldu1,
                     u2.as_mut_ptr(), &ldu2, v1t.as_mut_ptr(), &ldv1t, v2t.as_mut_ptr(), &ldv2t,
                     work.as_mut_ptr(), &lwork, iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dorcsd2by1(jobu1: u8, jobu2: u8, jobv1t: u8, m: i32, p: i32, q: i32, x11: &mut [f64],
                  ldx11: i32, x21: &mut [f64], ldx21: i32, theta: &mut [f64], u1: &mut [f64],
                  ldu1: i32, u2: &mut [f64], ldu2: i32, v1t: &mut [f64], ldv1t: i32,
                  work: &mut [f64], lwork: i32, iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::dorcsd2by1_(&(jobu1 as c_char), &(jobu2 as c_char), &(jobv1t as c_char), &m, &p, &q,
                         x11.as_mut_ptr(), &ldx11, x21.as_mut_ptr(), &ldx21, theta.as_mut_ptr(),
                         u1.as_mut_ptr(), &ldu1, u2.as_mut_ptr(), &ldu2, v1t.as_mut_ptr(), &ldv1t,
                         work.as_mut_ptr(), &lwork, iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dsyconv(uplo: u8, way: u8, n: i32, a: &mut [f64], lda: i32, ipiv: &[i32], work: &mut [f64],
               info: &mut i32) {

    unsafe {
        ffi::dsyconv_(&(uplo as c_char), &(way as c_char), &n, a.as_mut_ptr(), &lda, ipiv.as_ptr(),
                      work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dsyswapr(uplo: u8, n: i32, a: &mut [f64], i1: &[i32], i2: &[i32]) {
    unsafe {
        ffi::dsyswapr_(&(uplo as c_char), &n, a.as_mut_ptr(), i1.as_ptr(), i2.as_ptr())
    }
}

#[inline]
pub fn dsytri2(uplo: u8, n: i32, a: &mut [f64], lda: i32, ipiv: &[i32], work: &mut [c64],
               lwork: i32, info: &mut i32) {

    unsafe {
        ffi::dsytri2_(&(uplo as c_char), &n, a.as_mut_ptr(), &lda, ipiv.as_ptr(),
                      work.as_mut_ptr() as *mut _, &lwork, info)
    }
}

#[inline]
pub fn dsytri2x(uplo: u8, n: i32, a: &mut [f64], lda: i32, ipiv: &[i32], work: &mut [f64], nb: i32,
                info: &mut i32) {

    unsafe {
        ffi::dsytri2x_(&(uplo as c_char), &n, a.as_mut_ptr(), &lda, ipiv.as_ptr(),
                       work.as_mut_ptr(), &nb, info)
    }
}

#[inline]
pub fn dsytrs2(uplo: u8, n: i32, nrhs: i32, a: &[f64], lda: i32, ipiv: &[i32], b: &mut [f64],
               ldb: i32, work: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::dsytrs2_(&(uplo as c_char), &n, &nrhs, a.as_ptr(), &lda, ipiv.as_ptr(),
                      b.as_mut_ptr(), &ldb, work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn sbbcsd(jobu1: u8, jobu2: u8, jobv1t: u8, jobv2t: u8, trans: u8, m: i32, p: i32, q: i32,
              theta: &mut [f32], phi: &mut [f32], u1: &mut [f32], ldu1: i32, u2: &mut [f32],
              ldu2: i32, v1t: &mut [f32], ldv1t: i32, v2t: &mut [f32], ldv2t: i32,
              b11d: &mut [f32], b11e: &mut [f32], b12d: &mut [f32], b12e: &mut [f32],
              b21d: &mut [f32], b21e: &mut [f32], b22d: &mut [f32], b22e: &mut [f32],
              work: &mut [f32], lwork: i32, info: &mut i32) {

    unsafe {
        ffi::sbbcsd_(&(jobu1 as c_char), &(jobu2 as c_char), &(jobv1t as c_char),
                     &(jobv2t as c_char), &(trans as c_char), &m, &p, &q, theta.as_mut_ptr(),
                     phi.as_mut_ptr(), u1.as_mut_ptr(), &ldu1, u2.as_mut_ptr(), &ldu2,
                     v1t.as_mut_ptr(), &ldv1t, v2t.as_mut_ptr(), &ldv2t, b11d.as_mut_ptr(),
                     b11e.as_mut_ptr(), b12d.as_mut_ptr(), b12e.as_mut_ptr(), b21d.as_mut_ptr(),
                     b21e.as_mut_ptr(), b22d.as_mut_ptr(), b22e.as_mut_ptr(), work.as_mut_ptr(),
                     &lwork, info)
    }
}

#[inline]
pub fn sorbdb(trans: u8, signs: u8, m: i32, p: i32, q: i32, x11: &mut [f32], ldx11: i32,
              x12: &mut [f32], ldx12: i32, x21: &mut [f32], ldx21: i32, x22: &mut [f32],
              ldx22: i32, theta: &mut [f32], phi: &mut [f32], taup1: &mut [f32], taup2: &mut [f32],
              tauq1: &mut [f32], tauq2: &mut [f32], work: &mut [f32], lwork: i32, info: &mut i32) {

    unsafe {
        ffi::sorbdb_(&(trans as c_char), &(signs as c_char), &m, &p, &q, x11.as_mut_ptr(), &ldx11,
                     x12.as_mut_ptr(), &ldx12, x21.as_mut_ptr(), &ldx21, x22.as_mut_ptr(), &ldx22,
                     theta.as_mut_ptr(), phi.as_mut_ptr(), taup1.as_mut_ptr(), taup2.as_mut_ptr(),
                     tauq1.as_mut_ptr(), tauq2.as_mut_ptr(), work.as_mut_ptr(), &lwork, info)
    }
}

#[inline]
pub fn sorcsd(jobu1: u8, jobu2: u8, jobv1t: u8, jobv2t: u8, trans: u8, signs: u8, m: i32, p: i32,
              q: i32, x11: &mut [f32], ldx11: i32, x12: &mut [f32], ldx12: i32, x21: &mut [f32],
              ldx21: i32, x22: &mut [f32], ldx22: i32, theta: &mut [f32], u1: &mut [f32],
              ldu1: i32, u2: &mut [f32], ldu2: i32, v1t: &mut [f32], ldv1t: i32, v2t: &mut [f32],
              ldv2t: i32, work: &mut [f32], lwork: i32, iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::sorcsd_(&(jobu1 as c_char), &(jobu2 as c_char), &(jobv1t as c_char),
                     &(jobv2t as c_char), &(trans as c_char), &(signs as c_char), &m, &p, &q,
                     x11.as_mut_ptr(), &ldx11, x12.as_mut_ptr(), &ldx12, x21.as_mut_ptr(), &ldx21,
                     x22.as_mut_ptr(), &ldx22, theta.as_mut_ptr(), u1.as_mut_ptr(), &ldu1,
                     u2.as_mut_ptr(), &ldu2, v1t.as_mut_ptr(), &ldv1t, v2t.as_mut_ptr(), &ldv2t,
                     work.as_mut_ptr(), &lwork, iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn sorcsd2by1(jobu1: u8, jobu2: u8, jobv1t: u8, m: i32, p: i32, q: i32, x11: &mut [f32],
                  ldx11: i32, x21: &mut [f32], ldx21: i32, theta: &mut [f32], u1: &mut [f32],
                  ldu1: i32, u2: &mut [f32], ldu2: i32, v1t: &mut [f32], ldv1t: i32,
                  work: &mut [f32], lwork: i32, iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::sorcsd2by1_(&(jobu1 as c_char), &(jobu2 as c_char), &(jobv1t as c_char), &m, &p, &q,
                         x11.as_mut_ptr(), &ldx11, x21.as_mut_ptr(), &ldx21, theta.as_mut_ptr(),
                         u1.as_mut_ptr(), &ldu1, u2.as_mut_ptr(), &ldu2, v1t.as_mut_ptr(), &ldv1t,
                         work.as_mut_ptr(), &lwork, iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn ssyconv(uplo: u8, way: u8, n: i32, a: &mut [f32], lda: i32, ipiv: &[i32], work: &mut [f32],
               info: &mut i32) {

    unsafe {
        ffi::ssyconv_(&(uplo as c_char), &(way as c_char), &n, a.as_mut_ptr(), &lda, ipiv.as_ptr(),
                      work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn ssyswapr(uplo: u8, n: i32, a: &mut [f32], i1: &[i32], i2: &[i32]) {
    unsafe {
        ffi::ssyswapr_(&(uplo as c_char), &n, a.as_mut_ptr(), i1.as_ptr(), i2.as_ptr())
    }
}

#[inline]
pub fn ssytri2(uplo: u8, n: i32, a: &mut [f32], lda: i32, ipiv: &[i32], work: &mut [c32],
               lwork: i32, info: &mut i32) {

    unsafe {
        ffi::ssytri2_(&(uplo as c_char), &n, a.as_mut_ptr(), &lda, ipiv.as_ptr(),
                      work.as_mut_ptr() as *mut _, &lwork, info)
    }
}

#[inline]
pub fn ssytri2x(uplo: u8, n: i32, a: &mut [f32], lda: i32, ipiv: &[i32], work: &mut [f32], nb: i32,
                info: &mut i32) {

    unsafe {
        ffi::ssytri2x_(&(uplo as c_char), &n, a.as_mut_ptr(), &lda, ipiv.as_ptr(),
                       work.as_mut_ptr(), &nb, info)
    }
}

#[inline]
pub fn ssytrs2(uplo: u8, n: i32, nrhs: i32, a: &[f32], lda: i32, ipiv: &[i32], b: &mut [f32],
               ldb: i32, work: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::ssytrs2_(&(uplo as c_char), &n, &nrhs, a.as_ptr(), &lda, ipiv.as_ptr(),
                      b.as_mut_ptr(), &ldb, work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zbbcsd(jobu1: u8, jobu2: u8, jobv1t: u8, jobv2t: u8, trans: u8, m: i32, p: i32, q: i32,
              theta: &mut [f64], phi: &mut [f64], u1: &mut [c64], ldu1: i32, u2: &mut [c64],
              ldu2: i32, v1t: &mut [c64], ldv1t: i32, v2t: &mut [c64], ldv2t: i32,
              b11d: &mut [f64], b11e: &mut [f64], b12d: &mut [f64], b12e: &mut [f64],
              b21d: &mut [f64], b21e: &mut [f64], b22d: &mut [f64], b22e: &mut [f64],
              rwork: &mut [f64], lrwork: i32, info: &mut i32) {

    unsafe {
        ffi::zbbcsd_(&(jobu1 as c_char), &(jobu2 as c_char), &(jobv1t as c_char),
                     &(jobv2t as c_char), &(trans as c_char), &m, &p, &q, theta.as_mut_ptr(),
                     phi.as_mut_ptr(), u1.as_mut_ptr() as *mut _, &ldu1, u2.as_mut_ptr() as *mut _,
                     &ldu2, v1t.as_mut_ptr() as *mut _, &ldv1t, v2t.as_mut_ptr() as *mut _, &ldv2t,
                     b11d.as_mut_ptr(), b11e.as_mut_ptr(), b12d.as_mut_ptr(), b12e.as_mut_ptr(),
                     b21d.as_mut_ptr(), b21e.as_mut_ptr(), b22d.as_mut_ptr(), b22e.as_mut_ptr(),
                     rwork.as_mut_ptr(), &lrwork, info)
    }
}

#[inline]
pub fn zheswapr(uplo: u8, n: i32, a: &mut [c64], i1: &[i32], i2: &[i32]) {
    unsafe {
        ffi::zheswapr_(&(uplo as c_char), &n, a.as_mut_ptr() as *mut _, i1.as_ptr(), i2.as_ptr())
    }
}

#[inline]
pub fn zhetri2(uplo: u8, n: i32, a: &mut [c64], lda: i32, ipiv: &[i32], work: &mut [c64],
               lwork: i32, info: &mut i32) {

    unsafe {
        ffi::zhetri2_(&(uplo as c_char), &n, a.as_mut_ptr() as *mut _, &lda, ipiv.as_ptr(),
                      work.as_mut_ptr() as *mut _, &lwork, info)
    }
}

#[inline]
pub fn zhetri2x(uplo: u8, n: i32, a: &mut [c64], lda: i32, ipiv: &[i32], work: &mut [c64], nb: i32,
                info: &mut i32) {

    unsafe {
        ffi::zhetri2x_(&(uplo as c_char), &n, a.as_mut_ptr() as *mut _, &lda, ipiv.as_ptr(),
                       work.as_mut_ptr() as *mut _, &nb, info)
    }
}

#[inline]
pub fn zhetrs2(uplo: u8, n: i32, nrhs: i32, a: &[c64], lda: i32, ipiv: &[i32], b: &mut [c64],
               ldb: i32, work: &mut [c64], info: &mut i32) {

    unsafe {
        ffi::zhetrs2_(&(uplo as c_char), &n, &nrhs, a.as_ptr() as *const _, &lda, ipiv.as_ptr(),
                      b.as_mut_ptr() as *mut _, &ldb, work.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn zsyconv(uplo: u8, way: u8, n: i32, a: &mut [c64], lda: i32, ipiv: &[i32], work: &mut [c64],
               info: &mut i32) {

    unsafe {
        ffi::zsyconv_(&(uplo as c_char), &(way as c_char), &n, a.as_mut_ptr() as *mut _, &lda,
                      ipiv.as_ptr(), work.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn zsyswapr(uplo: u8, n: i32, a: &mut [c64], i1: &[i32], i2: &[i32]) {
    unsafe {
        ffi::zsyswapr_(&(uplo as c_char), &n, a.as_mut_ptr() as *mut _, i1.as_ptr(), i2.as_ptr())
    }
}

#[inline]
pub fn zsytri2(uplo: u8, n: i32, a: &mut [c64], lda: i32, ipiv: &[i32], work: &mut [c64],
               lwork: i32, info: &mut i32) {

    unsafe {
        ffi::zsytri2_(&(uplo as c_char), &n, a.as_mut_ptr() as *mut _, &lda, ipiv.as_ptr(),
                      work.as_mut_ptr() as *mut _, &lwork, info)
    }
}

#[inline]
pub fn zsytri2x(uplo: u8, n: i32, a: &mut [c64], lda: i32, ipiv: &[i32], work: &mut [c64], nb: i32,
                info: &mut i32) {

    unsafe {
        ffi::zsytri2x_(&(uplo as c_char), &n, a.as_mut_ptr() as *mut _, &lda, ipiv.as_ptr(),
                       work.as_mut_ptr() as *mut _, &nb, info)
    }
}

#[inline]
pub fn zsytrs2(uplo: u8, n: i32, nrhs: i32, a: &[c64], lda: i32, ipiv: &[i32], b: &mut [c64],
               ldb: i32, work: &mut [c64], info: &mut i32) {

    unsafe {
        ffi::zsytrs2_(&(uplo as c_char), &n, &nrhs, a.as_ptr() as *const _, &lda, ipiv.as_ptr(),
                      b.as_mut_ptr() as *mut _, &ldb, work.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn zunbdb(trans: u8, signs: u8, m: i32, p: i32, q: i32, x11: &mut [c64], ldx11: i32,
              x12: &mut [c64], ldx12: i32, x21: &mut [c64], ldx21: i32, x22: &mut [c64],
              ldx22: i32, theta: &mut [f64], phi: &mut [f64], taup1: &mut [c64], taup2: &mut [c64],
              tauq1: &mut [c64], tauq2: &mut [c64], work: &mut [c64], lwork: i32, info: &mut i32) {

    unsafe {
        ffi::zunbdb_(&(trans as c_char), &(signs as c_char), &m, &p, &q,
                     x11.as_mut_ptr() as *mut _, &ldx11, x12.as_mut_ptr() as *mut _, &ldx12,
                     x21.as_mut_ptr() as *mut _, &ldx21, x22.as_mut_ptr() as *mut _, &ldx22,
                     theta.as_mut_ptr(), phi.as_mut_ptr(), taup1.as_mut_ptr() as *mut _,
                     taup2.as_mut_ptr() as *mut _, tauq1.as_mut_ptr() as *mut _,
                     tauq2.as_mut_ptr() as *mut _, work.as_mut_ptr() as *mut _, &lwork, info)
    }
}

#[inline]
pub fn zuncsd(jobu1: u8, jobu2: u8, jobv1t: u8, jobv2t: u8, trans: u8, signs: u8, m: i32, p: i32,
              q: i32, x11: &mut [c64], ldx11: i32, x12: &mut [c64], ldx12: i32, x21: &mut [c64],
              ldx21: i32, x22: &mut [c64], ldx22: i32, theta: &mut [f64], u1: &mut [c64],
              ldu1: i32, u2: &mut [c64], ldu2: i32, v1t: &mut [c64], ldv1t: i32, v2t: &mut [c64],
              ldv2t: i32, work: &mut [c64], lwork: i32, rwork: &mut [f64], lrwork: i32,
              iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::zuncsd_(&(jobu1 as c_char), &(jobu2 as c_char), &(jobv1t as c_char),
                     &(jobv2t as c_char), &(trans as c_char), &(signs as c_char), &m, &p, &q,
                     x11.as_mut_ptr() as *mut _, &ldx11, x12.as_mut_ptr() as *mut _, &ldx12,
                     x21.as_mut_ptr() as *mut _, &ldx21, x22.as_mut_ptr() as *mut _, &ldx22,
                     theta.as_mut_ptr(), u1.as_mut_ptr() as *mut _, &ldu1,
                     u2.as_mut_ptr() as *mut _, &ldu2, v1t.as_mut_ptr() as *mut _, &ldv1t,
                     v2t.as_mut_ptr() as *mut _, &ldv2t, work.as_mut_ptr() as *mut _, &lwork,
                     rwork.as_mut_ptr(), &lrwork, iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zuncsd2by1(jobu1: u8, jobu2: u8, jobv1t: u8, m: i32, p: i32, q: i32, x11: &mut [c64],
                  ldx11: i32, x21: &mut [c64], ldx21: i32, theta: &mut [c64], u1: &mut [c64],
                  ldu1: i32, u2: &mut [c64], ldu2: i32, v1t: &mut [c64], ldv1t: i32,
                  work: &mut [c64], lwork: i32, rwork: &mut [f64], lrwork: i32, iwork: &mut [i32],
                  info: &mut i32) {

    unsafe {
        ffi::zuncsd2by1_(&(jobu1 as c_char), &(jobu2 as c_char), &(jobv1t as c_char), &m, &p, &q,
                         x11.as_mut_ptr() as *mut _, &ldx11, x21.as_mut_ptr() as *mut _, &ldx21,
                         theta.as_mut_ptr() as *mut _, u1.as_mut_ptr() as *mut _, &ldu1,
                         u2.as_mut_ptr() as *mut _, &ldu2, v1t.as_mut_ptr() as *mut _, &ldv1t,
                         work.as_mut_ptr() as *mut _, &lwork, rwork.as_mut_ptr(), &lrwork,
                         iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn sgemqrt(side: u8, trans: u8, m: i32, n: i32, k: i32, nb: i32, v: &[f32], ldv: i32,
               t: &[f32], ldt: i32, c: &mut [f32], ldc: i32, work: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::sgemqrt_(&(side as c_char), &(trans as c_char), &m, &n, &k, &nb, v.as_ptr(), &ldv,
                      t.as_ptr(), &ldt, c.as_mut_ptr(), &ldc, work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dgemqrt(side: u8, trans: u8, m: i32, n: i32, k: i32, nb: i32, v: &[f64], ldv: i32,
               t: &[f64], ldt: i32, c: &mut [f64], ldc: i32, work: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::dgemqrt_(&(side as c_char), &(trans as c_char), &m, &n, &k, &nb, v.as_ptr(), &ldv,
                      t.as_ptr(), &ldt, c.as_mut_ptr(), &ldc, work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn cgemqrt(side: u8, trans: u8, m: i32, n: i32, k: i32, nb: i32, v: &[c32], ldv: i32,
               t: &[c32], ldt: i32, c: &mut [c32], ldc: i32, work: &mut [c32], info: &mut i32) {

    unsafe {
        ffi::cgemqrt_(&(side as c_char), &(trans as c_char), &m, &n, &k, &nb,
                      v.as_ptr() as *const _, &ldv, t.as_ptr() as *const _, &ldt,
                      c.as_mut_ptr() as *mut _, &ldc, work.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn zgemqrt(side: u8, trans: u8, m: i32, n: i32, k: i32, nb: i32, v: &[c64], ldv: i32,
               t: &[c64], ldt: i32, c: &mut [c64], ldc: i32, work: &mut [c64], info: &mut i32) {

    unsafe {
        ffi::zgemqrt_(&(side as c_char), &(trans as c_char), &m, &n, &k, &nb,
                      v.as_ptr() as *const _, &ldv, t.as_ptr() as *const _, &ldt,
                      c.as_mut_ptr() as *mut _, &ldc, work.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn sgeqrt(m: i32, n: i32, nb: i32, a: &mut [f32], lda: i32, t: &mut [f32], ldt: i32,
              work: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::sgeqrt_(&m, &n, &nb, a.as_mut_ptr(), &lda, t.as_mut_ptr(), &ldt, work.as_mut_ptr(),
                     info)
    }
}

#[inline]
pub fn dgeqrt(m: i32, n: i32, nb: i32, a: &mut [f64], lda: i32, t: &mut [f64], ldt: i32,
              work: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::dgeqrt_(&m, &n, &nb, a.as_mut_ptr(), &lda, t.as_mut_ptr(), &ldt, work.as_mut_ptr(),
                     info)
    }
}

#[inline]
pub fn cgeqrt(m: i32, n: i32, nb: i32, a: &mut [c32], lda: i32, t: &mut [c32], ldt: i32,
              work: &mut [c32], info: &mut i32) {

    unsafe {
        ffi::cgeqrt_(&m, &n, &nb, a.as_mut_ptr() as *mut _, &lda, t.as_mut_ptr() as *mut _, &ldt,
                     work.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn zgeqrt(m: i32, n: i32, nb: i32, a: &mut [c64], lda: i32, t: &mut [c64], ldt: i32,
              work: &mut [c64], info: &mut i32) {

    unsafe {
        ffi::zgeqrt_(&m, &n, &nb, a.as_mut_ptr() as *mut _, &lda, t.as_mut_ptr() as *mut _, &ldt,
                     work.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn sgeqrt2(m: i32, n: i32, a: &mut [f32], lda: i32, t: &mut [f32], ldt: i32, info: &mut i32) {
    unsafe {
        ffi::sgeqrt2_(&m, &n, a.as_mut_ptr(), &lda, t.as_mut_ptr(), &ldt, info)
    }
}

#[inline]
pub fn dgeqrt2(m: i32, n: i32, a: &mut [f64], lda: i32, t: &mut [f64], ldt: i32, info: &mut i32) {
    unsafe {
        ffi::dgeqrt2_(&m, &n, a.as_mut_ptr(), &lda, t.as_mut_ptr(), &ldt, info)
    }
}

#[inline]
pub fn cgeqrt2(m: i32, n: i32, a: &mut [c32], lda: i32, t: &mut [c32], ldt: i32, info: &mut i32) {
    unsafe {
        ffi::cgeqrt2_(&m, &n, a.as_mut_ptr() as *mut _, &lda, t.as_mut_ptr() as *mut _, &ldt, info)
    }
}

#[inline]
pub fn zgeqrt2(m: i32, n: i32, a: &mut [c64], lda: i32, t: &mut [c64], ldt: i32, info: &mut i32) {
    unsafe {
        ffi::zgeqrt2_(&m, &n, a.as_mut_ptr() as *mut _, &lda, t.as_mut_ptr() as *mut _, &ldt, info)
    }
}

#[inline]
pub fn sgeqrt3(m: i32, n: i32, a: &mut [f32], lda: i32, t: &mut [f32], ldt: i32, info: &mut i32) {
    unsafe {
        ffi::sgeqrt3_(&m, &n, a.as_mut_ptr(), &lda, t.as_mut_ptr(), &ldt, info)
    }
}

#[inline]
pub fn dgeqrt3(m: i32, n: i32, a: &mut [f64], lda: i32, t: &mut [f64], ldt: i32, info: &mut i32) {
    unsafe {
        ffi::dgeqrt3_(&m, &n, a.as_mut_ptr(), &lda, t.as_mut_ptr(), &ldt, info)
    }
}

#[inline]
pub fn cgeqrt3(m: i32, n: i32, a: &mut [c32], lda: i32, t: &mut [c32], ldt: i32, info: &mut i32) {
    unsafe {
        ffi::cgeqrt3_(&m, &n, a.as_mut_ptr() as *mut _, &lda, t.as_mut_ptr() as *mut _, &ldt, info)
    }
}

#[inline]
pub fn zgeqrt3(m: i32, n: i32, a: &mut [c64], lda: i32, t: &mut [c64], ldt: i32, info: &mut i32) {
    unsafe {
        ffi::zgeqrt3_(&m, &n, a.as_mut_ptr() as *mut _, &lda, t.as_mut_ptr() as *mut _, &ldt, info)
    }
}

#[inline]
pub fn stpmqrt(side: u8, trans: u8, m: i32, n: i32, k: i32, l: i32, nb: i32, v: &[f32], ldv: i32,
               t: &[f32], ldt: i32, a: &mut [f32], lda: i32, b: &mut [f32], ldb: i32,
               work: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::stpmqrt_(&(side as c_char), &(trans as c_char), &m, &n, &k, &l, &nb, v.as_ptr(), &ldv,
                      t.as_ptr(), &ldt, a.as_mut_ptr(), &lda, b.as_mut_ptr(), &ldb,
                      work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dtpmqrt(side: u8, trans: u8, m: i32, n: i32, k: i32, l: i32, nb: i32, v: &[f64], ldv: i32,
               t: &[f64], ldt: i32, a: &mut [f64], lda: i32, b: &mut [f64], ldb: i32,
               work: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::dtpmqrt_(&(side as c_char), &(trans as c_char), &m, &n, &k, &l, &nb, v.as_ptr(), &ldv,
                      t.as_ptr(), &ldt, a.as_mut_ptr(), &lda, b.as_mut_ptr(), &ldb,
                      work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn ctpmqrt(side: u8, trans: u8, m: i32, n: i32, k: i32, l: i32, nb: i32, v: &[c32], ldv: i32,
               t: &[c32], ldt: i32, a: &mut [c32], lda: i32, b: &mut [c32], ldb: i32,
               work: &mut [c32], info: &mut i32) {

    unsafe {
        ffi::ctpmqrt_(&(side as c_char), &(trans as c_char), &m, &n, &k, &l, &nb,
                      v.as_ptr() as *const _, &ldv, t.as_ptr() as *const _, &ldt,
                      a.as_mut_ptr() as *mut _, &lda, b.as_mut_ptr() as *mut _, &ldb,
                      work.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn ztpmqrt(side: u8, trans: u8, m: i32, n: i32, k: i32, l: i32, nb: i32, v: &[c64], ldv: i32,
               t: &[c64], ldt: i32, a: &mut [c64], lda: i32, b: &mut [c64], ldb: i32,
               work: &mut [c64], info: &mut i32) {

    unsafe {
        ffi::ztpmqrt_(&(side as c_char), &(trans as c_char), &m, &n, &k, &l, &nb,
                      v.as_ptr() as *const _, &ldv, t.as_ptr() as *const _, &ldt,
                      a.as_mut_ptr() as *mut _, &lda, b.as_mut_ptr() as *mut _, &ldb,
                      work.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn stpqrt(m: i32, n: i32, l: i32, nb: i32, a: &mut [f32], lda: i32, b: &mut [f32], ldb: i32,
              t: &mut [f32], ldt: i32, work: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::stpqrt_(&m, &n, &l, &nb, a.as_mut_ptr(), &lda, b.as_mut_ptr(), &ldb, t.as_mut_ptr(),
                     &ldt, work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dtpqrt(m: i32, n: i32, l: i32, nb: i32, a: &mut [f64], lda: i32, b: &mut [f64], ldb: i32,
              t: &mut [f64], ldt: i32, work: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::dtpqrt_(&m, &n, &l, &nb, a.as_mut_ptr(), &lda, b.as_mut_ptr(), &ldb, t.as_mut_ptr(),
                     &ldt, work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn ctpqrt(m: i32, n: i32, l: i32, nb: i32, a: &mut [c32], lda: i32, b: &mut [c32], ldb: i32,
              t: &mut [c32], ldt: i32, work: &mut [c32], info: &mut i32) {

    unsafe {
        ffi::ctpqrt_(&m, &n, &l, &nb, a.as_mut_ptr() as *mut _, &lda, b.as_mut_ptr() as *mut _,
                     &ldb, t.as_mut_ptr() as *mut _, &ldt, work.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn ztpqrt(m: i32, n: i32, l: i32, nb: i32, a: &mut [c64], lda: i32, b: &mut [c64], ldb: i32,
              t: &mut [c64], ldt: i32, work: &mut [c64], info: &mut i32) {

    unsafe {
        ffi::ztpqrt_(&m, &n, &l, &nb, a.as_mut_ptr() as *mut _, &lda, b.as_mut_ptr() as *mut _,
                     &ldb, t.as_mut_ptr() as *mut _, &ldt, work.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn stpqrt2(m: i32, n: i32, l: i32, a: &mut [f32], lda: i32, b: &mut [f32], ldb: i32,
               t: &mut [f32], ldt: i32, info: &mut i32) {

    unsafe {
        ffi::stpqrt2_(&m, &n, &l, a.as_mut_ptr(), &lda, b.as_mut_ptr(), &ldb, t.as_mut_ptr(), &ldt,
                      info)
    }
}

#[inline]
pub fn dtpqrt2(m: i32, n: i32, l: i32, a: &mut [f64], lda: i32, b: &mut [f64], ldb: i32,
               t: &mut [f64], ldt: i32, info: &mut i32) {

    unsafe {
        ffi::dtpqrt2_(&m, &n, &l, a.as_mut_ptr(), &lda, b.as_mut_ptr(), &ldb, t.as_mut_ptr(), &ldt,
                      info)
    }
}

#[inline]
pub fn ctpqrt2(m: i32, n: i32, l: i32, a: &mut [c32], lda: i32, b: &mut [c32], ldb: i32,
               t: &mut [c32], ldt: i32, info: &mut i32) {

    unsafe {
        ffi::ctpqrt2_(&m, &n, &l, a.as_mut_ptr() as *mut _, &lda, b.as_mut_ptr() as *mut _, &ldb,
                      t.as_mut_ptr() as *mut _, &ldt, info)
    }
}

#[inline]
pub fn ztpqrt2(m: i32, n: i32, l: i32, a: &mut [c64], lda: i32, b: &mut [c64], ldb: i32,
               t: &mut [c64], ldt: i32, info: &mut i32) {

    unsafe {
        ffi::ztpqrt2_(&m, &n, &l, a.as_mut_ptr() as *mut _, &lda, b.as_mut_ptr() as *mut _, &ldb,
                      t.as_mut_ptr() as *mut _, &ldt, info)
    }
}

#[inline]
pub fn stprfb(side: u8, trans: u8, direct: u8, storev: u8, m: i32, n: i32, k: i32, l: i32,
              v: &[f32], ldv: i32, t: &[f32], ldt: i32, a: &mut [f32], lda: i32, b: &mut [f32],
              ldb: i32, work: &[f32], ldwork: i32) {

    unsafe {
        ffi::stprfb_(&(side as c_char), &(trans as c_char), &(direct as c_char),
                     &(storev as c_char), &m, &n, &k, &l, v.as_ptr(), &ldv, t.as_ptr(), &ldt,
                     a.as_mut_ptr(), &lda, b.as_mut_ptr(), &ldb, work.as_ptr(), &ldwork)
    }
}

#[inline]
pub fn dtprfb(side: u8, trans: u8, direct: u8, storev: u8, m: i32, n: i32, k: i32, l: i32,
              v: &[f64], ldv: i32, t: &[f64], ldt: i32, a: &mut [f64], lda: i32, b: &mut [f64],
              ldb: i32, work: &[f64], ldwork: i32) {

    unsafe {
        ffi::dtprfb_(&(side as c_char), &(trans as c_char), &(direct as c_char),
                     &(storev as c_char), &m, &n, &k, &l, v.as_ptr(), &ldv, t.as_ptr(), &ldt,
                     a.as_mut_ptr(), &lda, b.as_mut_ptr(), &ldb, work.as_ptr(), &ldwork)
    }
}

#[inline]
pub fn ctprfb(side: u8, trans: u8, direct: u8, storev: u8, m: i32, n: i32, k: i32, l: i32,
              v: &[c32], ldv: i32, t: &[c32], ldt: i32, a: &mut [c32], lda: i32, b: &mut [c32],
              ldb: i32, work: &mut [c32], ldwork: i32) {

    unsafe {
        ffi::ctprfb_(&(side as c_char), &(trans as c_char), &(direct as c_char),
                     &(storev as c_char), &m, &n, &k, &l, v.as_ptr() as *const _, &ldv,
                     t.as_ptr() as *const _, &ldt, a.as_mut_ptr() as *mut _, &lda,
                     b.as_mut_ptr() as *mut _, &ldb, work.as_mut_ptr() as *mut _, &ldwork)
    }
}

#[inline]
pub fn ztprfb(side: u8, trans: u8, direct: u8, storev: u8, m: i32, n: i32, k: i32, l: i32,
              v: &[c64], ldv: i32, t: &[c64], ldt: i32, a: &mut [c64], lda: i32, b: &mut [c64],
              ldb: i32, work: &mut [c64], ldwork: i32) {

    unsafe {
        ffi::ztprfb_(&(side as c_char), &(trans as c_char), &(direct as c_char),
                     &(storev as c_char), &m, &n, &k, &l, v.as_ptr() as *const _, &ldv,
                     t.as_ptr() as *const _, &ldt, a.as_mut_ptr() as *mut _, &lda,
                     b.as_mut_ptr() as *mut _, &ldb, work.as_mut_ptr() as *mut _, &ldwork)
    }
}

#[inline]
pub fn ssysv_rook(uplo: u8, n: i32, nrhs: i32, a: &mut [f32], lda: i32, ipiv: &mut [i32],
                  b: &mut [f32], ldb: i32, work: &mut [f32], lwork: i32, info: &mut i32) {

    unsafe {
        ffi::ssysv_rook_(&(uplo as c_char), &n, &nrhs, a.as_mut_ptr(), &lda, ipiv.as_mut_ptr(),
                         b.as_mut_ptr(), &ldb, work.as_mut_ptr(), &lwork, info)
    }
}

#[inline]
pub fn dsysv_rook(uplo: u8, n: i32, nrhs: i32, a: &mut [f64], lda: i32, ipiv: &mut [i32],
                  b: &mut [f64], ldb: i32, work: &mut [f64], lwork: i32, info: &mut i32) {

    unsafe {
        ffi::dsysv_rook_(&(uplo as c_char), &n, &nrhs, a.as_mut_ptr(), &lda, ipiv.as_mut_ptr(),
                         b.as_mut_ptr(), &ldb, work.as_mut_ptr(), &lwork, info)
    }
}

#[inline]
pub fn csysv_rook(uplo: u8, n: i32, nrhs: i32, a: &mut [c32], lda: i32, ipiv: &mut [i32],
                  b: &mut [c32], ldb: i32, work: &mut [c32], lwork: i32, info: &mut i32) {

    unsafe {
        ffi::csysv_rook_(&(uplo as c_char), &n, &nrhs, a.as_mut_ptr() as *mut _, &lda,
                         ipiv.as_mut_ptr(), b.as_mut_ptr() as *mut _, &ldb,
                         work.as_mut_ptr() as *mut _, &lwork, info)
    }
}

#[inline]
pub fn zsysv_rook(uplo: u8, n: i32, nrhs: i32, a: &mut [c64], lda: i32, ipiv: &mut [i32],
                  b: &mut [c64], ldb: i32, work: &mut [c64], lwork: i32, info: &mut i32) {

    unsafe {
        ffi::zsysv_rook_(&(uplo as c_char), &n, &nrhs, a.as_mut_ptr() as *mut _, &lda,
                         ipiv.as_mut_ptr(), b.as_mut_ptr() as *mut _, &ldb,
                         work.as_mut_ptr() as *mut _, &lwork, info)
    }
}

#[inline]
pub fn ssytrf_rook(uplo: u8, n: i32, a: &mut [f32], lda: i32, ipiv: &mut [i32], work: &mut [f32],
                   lwork: i32, info: &mut i32) {

    unsafe {
        ffi::ssytrf_rook_(&(uplo as c_char), &n, a.as_mut_ptr(), &lda, ipiv.as_mut_ptr(),
                          work.as_mut_ptr(), &lwork, info)
    }
}

#[inline]
pub fn dsytrf_rook(uplo: u8, n: i32, a: &mut [f64], lda: i32, ipiv: &mut [i32], work: &mut [f64],
                   lwork: i32, info: &mut i32) {

    unsafe {
        ffi::dsytrf_rook_(&(uplo as c_char), &n, a.as_mut_ptr(), &lda, ipiv.as_mut_ptr(),
                          work.as_mut_ptr(), &lwork, info)
    }
}

#[inline]
pub fn csytrf_rook(uplo: u8, n: i32, a: &mut [c32], lda: i32, ipiv: &mut [i32], work: &mut [c32],
                   lwork: i32, info: &mut i32) {

    unsafe {
        ffi::csytrf_rook_(&(uplo as c_char), &n, a.as_mut_ptr() as *mut _, &lda, ipiv.as_mut_ptr(),
                          work.as_mut_ptr() as *mut _, &lwork, info)
    }
}

#[inline]
pub fn zsytrf_rook(uplo: u8, n: i32, a: &mut [c64], lda: i32, ipiv: &mut [i32], work: &mut [c64],
                   lwork: i32, info: &mut i32) {

    unsafe {
        ffi::zsytrf_rook_(&(uplo as c_char), &n, a.as_mut_ptr() as *mut _, &lda, ipiv.as_mut_ptr(),
                          work.as_mut_ptr() as *mut _, &lwork, info)
    }
}

#[inline]
pub fn ssytrs_rook(uplo: u8, n: i32, nrhs: i32, a: &[f32], lda: i32, ipiv: &[i32], b: &mut [f32],
                   ldb: i32, info: &mut i32) {

    unsafe {
        ffi::ssytrs_rook_(&(uplo as c_char), &n, &nrhs, a.as_ptr(), &lda, ipiv.as_ptr(),
                          b.as_mut_ptr(), &ldb, info)
    }
}

#[inline]
pub fn dsytrs_rook(uplo: u8, n: i32, nrhs: i32, a: &[f64], lda: i32, ipiv: &[i32], b: &mut [f64],
                   ldb: i32, info: &mut i32) {

    unsafe {
        ffi::dsytrs_rook_(&(uplo as c_char), &n, &nrhs, a.as_ptr(), &lda, ipiv.as_ptr(),
                          b.as_mut_ptr(), &ldb, info)
    }
}

#[inline]
pub fn csytrs_rook(uplo: u8, n: i32, nrhs: i32, a: &[c32], lda: i32, ipiv: &[i32], b: &mut [c32],
                   ldb: i32, info: &mut i32) {

    unsafe {
        ffi::csytrs_rook_(&(uplo as c_char), &n, &nrhs, a.as_ptr() as *const _, &lda,
                          ipiv.as_ptr(), b.as_mut_ptr() as *mut _, &ldb, info)
    }
}

#[inline]
pub fn zsytrs_rook(uplo: u8, n: i32, nrhs: i32, a: &[c64], lda: i32, ipiv: &[i32], b: &mut [c64],
                   ldb: i32, info: &mut i32) {

    unsafe {
        ffi::zsytrs_rook_(&(uplo as c_char), &n, &nrhs, a.as_ptr() as *const _, &lda,
                          ipiv.as_ptr(), b.as_mut_ptr() as *mut _, &ldb, info)
    }
}

#[inline]
pub fn chetrf_rook(uplo: u8, n: i32, a: &mut [c32], lda: i32, ipiv: &mut [i32], work: &mut [c32],
                   lwork: i32, info: &mut i32) {

    unsafe {
        ffi::chetrf_rook_(&(uplo as c_char), &n, a.as_mut_ptr() as *mut _, &lda, ipiv.as_mut_ptr(),
                          work.as_mut_ptr() as *mut _, &lwork, info)
    }
}

#[inline]
pub fn zhetrf_rook(uplo: u8, n: i32, a: &mut [c64], lda: i32, ipiv: &mut [i32], work: &mut [c64],
                   lwork: i32, info: &mut i32) {

    unsafe {
        ffi::zhetrf_rook_(&(uplo as c_char), &n, a.as_mut_ptr() as *mut _, &lda, ipiv.as_mut_ptr(),
                          work.as_mut_ptr() as *mut _, &lwork, info)
    }
}

#[inline]
pub fn chetrs_rook(uplo: u8, n: i32, nrhs: i32, a: &[c32], lda: i32, ipiv: &[i32], b: &mut [c32],
                   ldb: i32, info: &mut i32) {

    unsafe {
        ffi::chetrs_rook_(&(uplo as c_char), &n, &nrhs, a.as_ptr() as *const _, &lda,
                          ipiv.as_ptr(), b.as_mut_ptr() as *mut _, &ldb, info)
    }
}

#[inline]
pub fn zhetrs_rook(uplo: u8, n: i32, nrhs: i32, a: &[c64], lda: i32, ipiv: &[i32], b: &mut [c64],
                   ldb: i32, info: &mut i32) {

    unsafe {
        ffi::zhetrs_rook_(&(uplo as c_char), &n, &nrhs, a.as_ptr() as *const _, &lda,
                          ipiv.as_ptr(), b.as_mut_ptr() as *mut _, &ldb, info)
    }
}

#[inline]
pub fn csyr(uplo: u8, n: i32, alpha: c32, x: &[c32], incx: i32, a: &mut [c32], lda: i32) {
    unsafe {
        ffi::csyr_(&(uplo as c_char), &n, &alpha as *const _ as *const _, x.as_ptr() as *const _,
                   &incx, a.as_mut_ptr() as *mut _, &lda)
    }
}

#[inline]
pub fn zsyr(uplo: u8, n: i32, alpha: c64, x: &[c64], incx: i32, a: &mut [c64], lda: i32) {
    unsafe {
        ffi::zsyr_(&(uplo as c_char), &n, &alpha as *const _ as *const _, x.as_ptr() as *const _,
                   &incx, a.as_mut_ptr() as *mut _, &lda)
    }
}

#[inline]
pub fn ilaver(vers_major: &mut i32, vers_minor: &mut i32, vers_patch: &mut i32) {
    unsafe {
        ffi::ilaver_(vers_major, vers_minor, vers_patch)
    }
}
