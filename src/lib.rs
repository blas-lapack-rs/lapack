//! Interface to the [Linear Algebra PACKage][1].
//!
//! ## Example
//!
//! ```
//! let n = 3;
//!
//! let mut a = vec![3.0, 1.0, 1.0, 1.0, 3.0, 1.0, 1.0, 1.0, 3.0];
//! let mut w = vec![0.0; n];
//! let mut work = vec![0.0; 4 * n];
//! let mut info = 0;
//!
//! lapack::dsyev(b'V', b'U', n, &mut a, n, &mut w, &mut work, 4 * n, &mut info);
//!
//! for (one, another) in w.iter().zip(&[2.0, 2.0, 5.0]) {
//!     assert!((one - another).abs() < 1e-14);
//! }
//! ```
//!
//! [1]: http://en.wikipedia.org/wiki/LAPACK

extern crate complex;
extern crate lapack_sys as ffi;
extern crate libc;

use complex::{c32, c64};
use libc::{c_char, c_int};
use std::mem::transmute;

pub type Select2F32 = Option<extern "C" fn(*const f32, *const f32) -> i32>;
pub type Select3F32 = Option<extern "C" fn(*const f32, *const f32, *const f32) -> i32>;

pub type Select2F64 = Option<extern "C" fn(*const f64, *const f64) -> i32>;
pub type Select3F64 = Option<extern "C" fn(*const f64, *const f64, *const f64) -> i32>;

pub type Select1C32 = Option<extern "C" fn(*const c32) -> i32>;
pub type Select2C32 = Option<extern "C" fn(*const c32, *const c32) -> i32>;

pub type Select1C64 = Option<extern "C" fn(*const c64) -> i32>;
pub type Select2C64 = Option<extern "C" fn(*const c64, *const c64) -> i32>;

#[inline]
pub fn sgetrf(m: usize, n: usize, a: &mut [f32], lda: usize, ipiv: &mut [i32], info: &mut i32) {
    unsafe {
        ffi::sgetrf_(&(m as c_int), &(n as c_int), a.as_mut_ptr(), &(lda as c_int),
                     ipiv.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dgetrf(m: usize, n: usize, a: &mut [f64], lda: usize, ipiv: &mut [i32], info: &mut i32) {
    unsafe {
        ffi::dgetrf_(&(m as c_int), &(n as c_int), a.as_mut_ptr(), &(lda as c_int),
                     ipiv.as_mut_ptr(), info)
    }
}

#[inline]
pub fn cgetrf(m: usize, n: usize, a: &mut [c32], lda: usize, ipiv: &mut [i32], info: &mut i32) {
    unsafe {
        ffi::cgetrf_(&(m as c_int), &(n as c_int), a.as_mut_ptr() as *mut _, &(lda as c_int),
                     ipiv.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zgetrf(m: usize, n: usize, a: &mut [c64], lda: usize, ipiv: &mut [i32], info: &mut i32) {
    unsafe {
        ffi::zgetrf_(&(m as c_int), &(n as c_int), a.as_mut_ptr() as *mut _, &(lda as c_int),
                     ipiv.as_mut_ptr(), info)
    }
}

#[inline]
pub fn sgbtrf(m: usize, n: usize, kl: usize, ku: usize, ab: &mut [f32], ldab: usize,
              ipiv: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::sgbtrf_(&(m as c_int), &(n as c_int), &(kl as c_int), &(ku as c_int), ab.as_mut_ptr(),
                     &(ldab as c_int), ipiv.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dgbtrf(m: usize, n: usize, kl: usize, ku: usize, ab: &mut [f64], ldab: usize,
              ipiv: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::dgbtrf_(&(m as c_int), &(n as c_int), &(kl as c_int), &(ku as c_int), ab.as_mut_ptr(),
                     &(ldab as c_int), ipiv.as_mut_ptr(), info)
    }
}

#[inline]
pub fn cgbtrf(m: usize, n: usize, kl: usize, ku: usize, ab: &mut [c32], ldab: usize,
              ipiv: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::cgbtrf_(&(m as c_int), &(n as c_int), &(kl as c_int), &(ku as c_int),
                     ab.as_mut_ptr() as *mut _, &(ldab as c_int), ipiv.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zgbtrf(m: usize, n: usize, kl: usize, ku: usize, ab: &mut [c64], ldab: usize,
              ipiv: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::zgbtrf_(&(m as c_int), &(n as c_int), &(kl as c_int), &(ku as c_int),
                     ab.as_mut_ptr() as *mut _, &(ldab as c_int), ipiv.as_mut_ptr(), info)
    }
}

#[inline]
pub fn sgttrf(n: usize, dl: &mut [f32], d: &mut [f32], du: &mut [f32], du2: &mut [f32],
              ipiv: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::sgttrf_(&(n as c_int), dl.as_mut_ptr(), d.as_mut_ptr(), du.as_mut_ptr(),
                     du2.as_mut_ptr(), ipiv.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dgttrf(n: usize, dl: &mut [f64], d: &mut [f64], du: &mut [f64], du2: &mut [f64],
              ipiv: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::dgttrf_(&(n as c_int), dl.as_mut_ptr(), d.as_mut_ptr(), du.as_mut_ptr(),
                     du2.as_mut_ptr(), ipiv.as_mut_ptr(), info)
    }
}

#[inline]
pub fn cgttrf(n: usize, dl: &mut [c32], d: &mut [c32], du: &mut [c32], du2: &mut [c32],
              ipiv: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::cgttrf_(&(n as c_int), dl.as_mut_ptr() as *mut _, d.as_mut_ptr() as *mut _,
                     du.as_mut_ptr() as *mut _, du2.as_mut_ptr() as *mut _, ipiv.as_mut_ptr(),
                     info)
    }
}

#[inline]
pub fn zgttrf(n: usize, dl: &mut [c64], d: &mut [c64], du: &mut [c64], du2: &mut [c64],
              ipiv: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::zgttrf_(&(n as c_int), dl.as_mut_ptr() as *mut _, d.as_mut_ptr() as *mut _,
                     du.as_mut_ptr() as *mut _, du2.as_mut_ptr() as *mut _, ipiv.as_mut_ptr(),
                     info)
    }
}

#[inline]
pub fn spotrf(uplo: u8, n: usize, a: &mut [f32], lda: usize, info: &mut i32) {
    unsafe {
        ffi::spotrf_(&(uplo as c_char), &(n as c_int), a.as_mut_ptr(), &(lda as c_int), info)
    }
}

#[inline]
pub fn dpotrf(uplo: u8, n: usize, a: &mut [f64], lda: usize, info: &mut i32) {
    unsafe {
        ffi::dpotrf_(&(uplo as c_char), &(n as c_int), a.as_mut_ptr(), &(lda as c_int), info)
    }
}

#[inline]
pub fn cpotrf(uplo: u8, n: usize, a: &mut [c32], lda: usize, info: &mut i32) {
    unsafe {
        ffi::cpotrf_(&(uplo as c_char), &(n as c_int), a.as_mut_ptr() as *mut _, &(lda as c_int),
                     info)
    }
}

#[inline]
pub fn zpotrf(uplo: u8, n: usize, a: &mut [c64], lda: usize, info: &mut i32) {
    unsafe {
        ffi::zpotrf_(&(uplo as c_char), &(n as c_int), a.as_mut_ptr() as *mut _, &(lda as c_int),
                     info)
    }
}

#[inline]
pub fn spstrf(uplo: u8, n: usize, a: &mut [f32], lda: usize, piv: &mut [i32], rank: &mut u32,
              tol: &[f32], work: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::spstrf_(&(uplo as c_char), &(n as c_int), a.as_mut_ptr(), &(lda as c_int),
                     piv.as_mut_ptr(), rank as *mut _ as *mut _, tol.as_ptr(), work.as_mut_ptr(),
                     info)
    }
}

#[inline]
pub fn dpstrf(uplo: u8, n: usize, a: &mut [f64], lda: usize, piv: &mut [i32], rank: &mut u32,
              tol: &[f64], work: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::dpstrf_(&(uplo as c_char), &(n as c_int), a.as_mut_ptr(), &(lda as c_int),
                     piv.as_mut_ptr(), rank as *mut _ as *mut _, tol.as_ptr(), work.as_mut_ptr(),
                     info)
    }
}

#[inline]
pub fn cpstrf(uplo: u8, n: usize, a: &mut [c32], lda: usize, piv: &mut [i32], rank: &mut u32,
              tol: &[f32], work: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::cpstrf_(&(uplo as c_char), &(n as c_int), a.as_mut_ptr() as *mut _, &(lda as c_int),
                     piv.as_mut_ptr(), rank as *mut _ as *mut _, tol.as_ptr(), work.as_mut_ptr(),
                     info)
    }
}

#[inline]
pub fn zpstrf(uplo: u8, n: usize, a: &mut [c64], lda: usize, piv: &mut [i32], rank: &mut u32,
              tol: &[f64], work: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::zpstrf_(&(uplo as c_char), &(n as c_int), a.as_mut_ptr() as *mut _, &(lda as c_int),
                     piv.as_mut_ptr(), rank as *mut _ as *mut _, tol.as_ptr(), work.as_mut_ptr(),
                     info)
    }
}

#[inline]
pub fn spftrf(transr: u8, uplo: u8, n: usize, a: &mut [f32], info: &mut i32) {
    unsafe {
        ffi::spftrf_(&(transr as c_char), &(uplo as c_char), &(n as c_int), a.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dpftrf(transr: u8, uplo: u8, n: usize, a: &mut [f64], info: &mut i32) {
    unsafe {
        ffi::dpftrf_(&(transr as c_char), &(uplo as c_char), &(n as c_int), a.as_mut_ptr(), info)
    }
}

#[inline]
pub fn cpftrf(transr: u8, uplo: u8, n: usize, a: &mut [c32], info: &mut i32) {
    unsafe {
        ffi::cpftrf_(&(transr as c_char), &(uplo as c_char), &(n as c_int),
                     a.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn zpftrf(transr: u8, uplo: u8, n: usize, a: &mut [c64], info: &mut i32) {
    unsafe {
        ffi::zpftrf_(&(transr as c_char), &(uplo as c_char), &(n as c_int),
                     a.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn spptrf(uplo: u8, n: usize, ap: &mut [f32], info: &mut i32) {
    unsafe {
        ffi::spptrf_(&(uplo as c_char), &(n as c_int), ap.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dpptrf(uplo: u8, n: usize, ap: &mut [f64], info: &mut i32) {
    unsafe {
        ffi::dpptrf_(&(uplo as c_char), &(n as c_int), ap.as_mut_ptr(), info)
    }
}

#[inline]
pub fn cpptrf(uplo: u8, n: usize, ap: &mut [c32], info: &mut i32) {
    unsafe {
        ffi::cpptrf_(&(uplo as c_char), &(n as c_int), ap.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn zpptrf(uplo: u8, n: usize, ap: &mut [c64], info: &mut i32) {
    unsafe {
        ffi::zpptrf_(&(uplo as c_char), &(n as c_int), ap.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn spbtrf(uplo: u8, n: usize, kd: usize, ab: &mut [f32], ldab: usize, info: &mut i32) {
    unsafe {
        ffi::spbtrf_(&(uplo as c_char), &(n as c_int), &(kd as c_int), ab.as_mut_ptr(),
                     &(ldab as c_int), info)
    }
}

#[inline]
pub fn dpbtrf(uplo: u8, n: usize, kd: usize, ab: &mut [f64], ldab: usize, info: &mut i32) {
    unsafe {
        ffi::dpbtrf_(&(uplo as c_char), &(n as c_int), &(kd as c_int), ab.as_mut_ptr(),
                     &(ldab as c_int), info)
    }
}

#[inline]
pub fn cpbtrf(uplo: u8, n: usize, kd: usize, ab: &mut [c32], ldab: usize, info: &mut i32) {
    unsafe {
        ffi::cpbtrf_(&(uplo as c_char), &(n as c_int), &(kd as c_int), ab.as_mut_ptr() as *mut _,
                     &(ldab as c_int), info)
    }
}

#[inline]
pub fn zpbtrf(uplo: u8, n: usize, kd: usize, ab: &mut [c64], ldab: usize, info: &mut i32) {
    unsafe {
        ffi::zpbtrf_(&(uplo as c_char), &(n as c_int), &(kd as c_int), ab.as_mut_ptr() as *mut _,
                     &(ldab as c_int), info)
    }
}

#[inline]
pub fn spttrf(n: usize, d: &mut [f32], e: &mut [f32], info: &mut i32) {
    unsafe {
        ffi::spttrf_(&(n as c_int), d.as_mut_ptr(), e.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dpttrf(n: usize, d: &mut [f64], e: &mut [f64], info: &mut i32) {
    unsafe {
        ffi::dpttrf_(&(n as c_int), d.as_mut_ptr(), e.as_mut_ptr(), info)
    }
}

#[inline]
pub fn cpttrf(n: usize, d: &mut [f32], e: &mut [c32], info: &mut i32) {
    unsafe {
        ffi::cpttrf_(&(n as c_int), d.as_mut_ptr(), e.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn zpttrf(n: usize, d: &mut [f64], e: &mut [c64], info: &mut i32) {
    unsafe {
        ffi::zpttrf_(&(n as c_int), d.as_mut_ptr(), e.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn ssytrf(uplo: u8, n: usize, a: &mut [f32], lda: usize, ipiv: &mut [i32], work: &mut [f32],
              lwork: usize, info: &mut i32) {

    unsafe {
        ffi::ssytrf_(&(uplo as c_char), &(n as c_int), a.as_mut_ptr(), &(lda as c_int),
                     ipiv.as_mut_ptr(), work.as_mut_ptr(), &(lwork as c_int), info)
    }
}

#[inline]
pub fn dsytrf(uplo: u8, n: usize, a: &mut [f64], lda: usize, ipiv: &mut [i32], work: &mut [f64],
              lwork: usize, info: &mut i32) {

    unsafe {
        ffi::dsytrf_(&(uplo as c_char), &(n as c_int), a.as_mut_ptr(), &(lda as c_int),
                     ipiv.as_mut_ptr(), work.as_mut_ptr(), &(lwork as c_int), info)
    }
}

#[inline]
pub fn csytrf(uplo: u8, n: usize, a: &mut [c32], lda: usize, ipiv: &mut [i32], work: &mut [c32],
              lwork: usize, info: &mut i32) {

    unsafe {
        ffi::csytrf_(&(uplo as c_char), &(n as c_int), a.as_mut_ptr() as *mut _, &(lda as c_int),
                     ipiv.as_mut_ptr(), work.as_mut_ptr() as *mut _, &(lwork as c_int), info)
    }
}

#[inline]
pub fn zsytrf(uplo: u8, n: usize, a: &mut [c64], lda: usize, ipiv: &mut [i32], work: &mut [c64],
              lwork: usize, info: &mut i32) {

    unsafe {
        ffi::zsytrf_(&(uplo as c_char), &(n as c_int), a.as_mut_ptr() as *mut _, &(lda as c_int),
                     ipiv.as_mut_ptr(), work.as_mut_ptr() as *mut _, &(lwork as c_int), info)
    }
}

#[inline]
pub fn chetrf(uplo: u8, n: usize, a: &mut [c32], lda: usize, ipiv: &mut [i32], work: &mut [c32],
              lwork: usize, info: &mut i32) {

    unsafe {
        ffi::chetrf_(&(uplo as c_char), &(n as c_int), a.as_mut_ptr() as *mut _, &(lda as c_int),
                     ipiv.as_mut_ptr(), work.as_mut_ptr() as *mut _, &(lwork as c_int), info)
    }
}

#[inline]
pub fn zhetrf(uplo: u8, n: usize, a: &mut [c64], lda: usize, ipiv: &mut [i32], work: &mut [c64],
              lwork: usize, info: &mut i32) {

    unsafe {
        ffi::zhetrf_(&(uplo as c_char), &(n as c_int), a.as_mut_ptr() as *mut _, &(lda as c_int),
                     ipiv.as_mut_ptr(), work.as_mut_ptr() as *mut _, &(lwork as c_int), info)
    }
}

#[inline]
pub fn ssptrf(uplo: u8, n: usize, ap: &mut [f32], ipiv: &mut [i32], info: &mut i32) {
    unsafe {
        ffi::ssptrf_(&(uplo as c_char), &(n as c_int), ap.as_mut_ptr(), ipiv.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dsptrf(uplo: u8, n: usize, ap: &mut [f64], ipiv: &mut [i32], info: &mut i32) {
    unsafe {
        ffi::dsptrf_(&(uplo as c_char), &(n as c_int), ap.as_mut_ptr(), ipiv.as_mut_ptr(), info)
    }
}

#[inline]
pub fn csptrf(uplo: u8, n: usize, ap: &mut [c32], ipiv: &mut [i32], info: &mut i32) {
    unsafe {
        ffi::csptrf_(&(uplo as c_char), &(n as c_int), ap.as_mut_ptr() as *mut _,
                     ipiv.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zsptrf(uplo: u8, n: usize, ap: &mut [c64], ipiv: &mut [i32], info: &mut i32) {
    unsafe {
        ffi::zsptrf_(&(uplo as c_char), &(n as c_int), ap.as_mut_ptr() as *mut _,
                     ipiv.as_mut_ptr(), info)
    }
}

#[inline]
pub fn chptrf(uplo: u8, n: usize, ap: &mut [c32], ipiv: &mut [i32], info: &mut i32) {
    unsafe {
        ffi::chptrf_(&(uplo as c_char), &(n as c_int), ap.as_mut_ptr() as *mut _,
                     ipiv.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zhptrf(uplo: u8, n: usize, ap: &mut [c64], ipiv: &mut [i32], info: &mut i32) {
    unsafe {
        ffi::zhptrf_(&(uplo as c_char), &(n as c_int), ap.as_mut_ptr() as *mut _,
                     ipiv.as_mut_ptr(), info)
    }
}

#[inline]
pub fn sgetrs(trans: u8, n: usize, nrhs: usize, a: &[f32], lda: usize, ipiv: &[i32], b: &mut [f32],
              ldb: usize, info: &mut i32) {

    unsafe {
        ffi::sgetrs_(&(trans as c_char), &(n as c_int), &(nrhs as c_int), a.as_ptr(),
                     &(lda as c_int), ipiv.as_ptr(), b.as_mut_ptr(), &(ldb as c_int), info)
    }
}

#[inline]
pub fn dgetrs(trans: u8, n: usize, nrhs: usize, a: &[f64], lda: usize, ipiv: &[i32], b: &mut [f64],
              ldb: usize, info: &mut i32) {

    unsafe {
        ffi::dgetrs_(&(trans as c_char), &(n as c_int), &(nrhs as c_int), a.as_ptr(),
                     &(lda as c_int), ipiv.as_ptr(), b.as_mut_ptr(), &(ldb as c_int), info)
    }
}

#[inline]
pub fn cgetrs(trans: u8, n: usize, nrhs: usize, a: &[c32], lda: usize, ipiv: &[i32], b: &mut [c32],
              ldb: usize, info: &mut i32) {

    unsafe {
        ffi::cgetrs_(&(trans as c_char), &(n as c_int), &(nrhs as c_int), a.as_ptr() as *const _,
                     &(lda as c_int), ipiv.as_ptr(), b.as_mut_ptr() as *mut _, &(ldb as c_int),
                     info)
    }
}

#[inline]
pub fn zgetrs(trans: u8, n: usize, nrhs: usize, a: &[c64], lda: usize, ipiv: &[i32], b: &mut [c64],
              ldb: usize, info: &mut i32) {

    unsafe {
        ffi::zgetrs_(&(trans as c_char), &(n as c_int), &(nrhs as c_int), a.as_ptr() as *const _,
                     &(lda as c_int), ipiv.as_ptr(), b.as_mut_ptr() as *mut _, &(ldb as c_int),
                     info)
    }
}

#[inline]
pub fn sgbtrs(trans: u8, n: usize, kl: usize, ku: usize, nrhs: usize, ab: &[f32], ldab: usize,
              ipiv: &[i32], b: &mut [f32], ldb: usize, info: &mut i32) {

    unsafe {
        ffi::sgbtrs_(&(trans as c_char), &(n as c_int), &(kl as c_int), &(ku as c_int),
                     &(nrhs as c_int), ab.as_ptr(), &(ldab as c_int), ipiv.as_ptr(),
                     b.as_mut_ptr(), &(ldb as c_int), info)
    }
}

#[inline]
pub fn dgbtrs(trans: u8, n: usize, kl: usize, ku: usize, nrhs: usize, ab: &[f64], ldab: usize,
              ipiv: &[i32], b: &mut [f64], ldb: usize, info: &mut i32) {

    unsafe {
        ffi::dgbtrs_(&(trans as c_char), &(n as c_int), &(kl as c_int), &(ku as c_int),
                     &(nrhs as c_int), ab.as_ptr(), &(ldab as c_int), ipiv.as_ptr(),
                     b.as_mut_ptr(), &(ldb as c_int), info)
    }
}

#[inline]
pub fn cgbtrs(trans: u8, n: usize, kl: usize, ku: usize, nrhs: usize, ab: &[c32], ldab: usize,
              ipiv: &[i32], b: &mut [c32], ldb: usize, info: &mut i32) {

    unsafe {
        ffi::cgbtrs_(&(trans as c_char), &(n as c_int), &(kl as c_int), &(ku as c_int),
                     &(nrhs as c_int), ab.as_ptr() as *const _, &(ldab as c_int), ipiv.as_ptr(),
                     b.as_mut_ptr() as *mut _, &(ldb as c_int), info)
    }
}

#[inline]
pub fn zgbtrs(trans: u8, n: usize, kl: usize, ku: usize, nrhs: usize, ab: &[c64], ldab: usize,
              ipiv: &[i32], b: &mut [c64], ldb: usize, info: &mut i32) {

    unsafe {
        ffi::zgbtrs_(&(trans as c_char), &(n as c_int), &(kl as c_int), &(ku as c_int),
                     &(nrhs as c_int), ab.as_ptr() as *const _, &(ldab as c_int), ipiv.as_ptr(),
                     b.as_mut_ptr() as *mut _, &(ldb as c_int), info)
    }
}

#[inline]
pub fn sgttrs(trans: u8, n: usize, nrhs: usize, dl: &[f32], d: &[f32], du: &[f32], du2: &[f32],
              ipiv: &[i32], b: &mut [f32], ldb: usize, info: &mut i32) {

    unsafe {
        ffi::sgttrs_(&(trans as c_char), &(n as c_int), &(nrhs as c_int), dl.as_ptr(), d.as_ptr(),
                     du.as_ptr(), du2.as_ptr(), ipiv.as_ptr(), b.as_mut_ptr(), &(ldb as c_int),
                     info)
    }
}

#[inline]
pub fn dgttrs(trans: u8, n: usize, nrhs: usize, dl: &[f64], d: &[f64], du: &[f64], du2: &[f64],
              ipiv: &[i32], b: &mut [f64], ldb: usize, info: &mut i32) {

    unsafe {
        ffi::dgttrs_(&(trans as c_char), &(n as c_int), &(nrhs as c_int), dl.as_ptr(), d.as_ptr(),
                     du.as_ptr(), du2.as_ptr(), ipiv.as_ptr(), b.as_mut_ptr(), &(ldb as c_int),
                     info)
    }
}

#[inline]
pub fn cgttrs(trans: u8, n: usize, nrhs: usize, dl: &[c32], d: &[c32], du: &[c32], du2: &[c32],
              ipiv: &[i32], b: &mut [c32], ldb: usize, info: &mut i32) {

    unsafe {
        ffi::cgttrs_(&(trans as c_char), &(n as c_int), &(nrhs as c_int), dl.as_ptr() as *const _,
                     d.as_ptr() as *const _, du.as_ptr() as *const _, du2.as_ptr() as *const _,
                     ipiv.as_ptr(), b.as_mut_ptr() as *mut _, &(ldb as c_int), info)
    }
}

#[inline]
pub fn zgttrs(trans: u8, n: usize, nrhs: usize, dl: &[c64], d: &[c64], du: &[c64], du2: &[c64],
              ipiv: &[i32], b: &mut [c64], ldb: usize, info: &mut i32) {

    unsafe {
        ffi::zgttrs_(&(trans as c_char), &(n as c_int), &(nrhs as c_int), dl.as_ptr() as *const _,
                     d.as_ptr() as *const _, du.as_ptr() as *const _, du2.as_ptr() as *const _,
                     ipiv.as_ptr(), b.as_mut_ptr() as *mut _, &(ldb as c_int), info)
    }
}

#[inline]
pub fn spotrs(uplo: u8, n: usize, nrhs: usize, a: &[f32], lda: usize, b: &mut [f32], ldb: usize,
              info: &mut i32) {

    unsafe {
        ffi::spotrs_(&(uplo as c_char), &(n as c_int), &(nrhs as c_int), a.as_ptr(),
                     &(lda as c_int), b.as_mut_ptr(), &(ldb as c_int), info)
    }
}

#[inline]
pub fn dpotrs(uplo: u8, n: usize, nrhs: usize, a: &[f64], lda: usize, b: &mut [f64], ldb: usize,
              info: &mut i32) {

    unsafe {
        ffi::dpotrs_(&(uplo as c_char), &(n as c_int), &(nrhs as c_int), a.as_ptr(),
                     &(lda as c_int), b.as_mut_ptr(), &(ldb as c_int), info)
    }
}

#[inline]
pub fn cpotrs(uplo: u8, n: usize, nrhs: usize, a: &[c32], lda: usize, b: &mut [c32], ldb: usize,
              info: &mut i32) {

    unsafe {
        ffi::cpotrs_(&(uplo as c_char), &(n as c_int), &(nrhs as c_int), a.as_ptr() as *const _,
                     &(lda as c_int), b.as_mut_ptr() as *mut _, &(ldb as c_int), info)
    }
}

#[inline]
pub fn zpotrs(uplo: u8, n: usize, nrhs: usize, a: &[c64], lda: usize, b: &mut [c64], ldb: usize,
              info: &mut i32) {

    unsafe {
        ffi::zpotrs_(&(uplo as c_char), &(n as c_int), &(nrhs as c_int), a.as_ptr() as *const _,
                     &(lda as c_int), b.as_mut_ptr() as *mut _, &(ldb as c_int), info)
    }
}

#[inline]
pub fn spftrs(transr: u8, uplo: u8, n: usize, nrhs: usize, a: &[f32], b: &mut [f32], ldb: usize,
              info: &mut i32) {

    unsafe {
        ffi::spftrs_(&(transr as c_char), &(uplo as c_char), &(n as c_int), &(nrhs as c_int),
                     a.as_ptr(), b.as_mut_ptr(), &(ldb as c_int), info)
    }
}

#[inline]
pub fn dpftrs(transr: u8, uplo: u8, n: usize, nrhs: usize, a: &[f64], b: &mut [f64], ldb: usize,
              info: &mut i32) {

    unsafe {
        ffi::dpftrs_(&(transr as c_char), &(uplo as c_char), &(n as c_int), &(nrhs as c_int),
                     a.as_ptr(), b.as_mut_ptr(), &(ldb as c_int), info)
    }
}

#[inline]
pub fn cpftrs(transr: u8, uplo: u8, n: usize, nrhs: usize, a: &[c32], b: &mut [c32], ldb: usize,
              info: &mut i32) {

    unsafe {
        ffi::cpftrs_(&(transr as c_char), &(uplo as c_char), &(n as c_int), &(nrhs as c_int),
                     a.as_ptr() as *const _, b.as_mut_ptr() as *mut _, &(ldb as c_int), info)
    }
}

#[inline]
pub fn zpftrs(transr: u8, uplo: u8, n: usize, nrhs: usize, a: &[c64], b: &mut [c64], ldb: usize,
              info: &mut i32) {

    unsafe {
        ffi::zpftrs_(&(transr as c_char), &(uplo as c_char), &(n as c_int), &(nrhs as c_int),
                     a.as_ptr() as *const _, b.as_mut_ptr() as *mut _, &(ldb as c_int), info)
    }
}

#[inline]
pub fn spptrs(uplo: u8, n: usize, nrhs: usize, ap: &[f32], b: &mut [f32], ldb: usize,
              info: &mut i32) {

    unsafe {
        ffi::spptrs_(&(uplo as c_char), &(n as c_int), &(nrhs as c_int), ap.as_ptr(),
                     b.as_mut_ptr(), &(ldb as c_int), info)
    }
}

#[inline]
pub fn dpptrs(uplo: u8, n: usize, nrhs: usize, ap: &[f64], b: &mut [f64], ldb: usize,
              info: &mut i32) {

    unsafe {
        ffi::dpptrs_(&(uplo as c_char), &(n as c_int), &(nrhs as c_int), ap.as_ptr(),
                     b.as_mut_ptr(), &(ldb as c_int), info)
    }
}

#[inline]
pub fn cpptrs(uplo: u8, n: usize, nrhs: usize, ap: &[c32], b: &mut [c32], ldb: usize,
              info: &mut i32) {

    unsafe {
        ffi::cpptrs_(&(uplo as c_char), &(n as c_int), &(nrhs as c_int), ap.as_ptr() as *const _,
                     b.as_mut_ptr() as *mut _, &(ldb as c_int), info)
    }
}

#[inline]
pub fn zpptrs(uplo: u8, n: usize, nrhs: usize, ap: &[c64], b: &mut [c64], ldb: usize,
              info: &mut i32) {

    unsafe {
        ffi::zpptrs_(&(uplo as c_char), &(n as c_int), &(nrhs as c_int), ap.as_ptr() as *const _,
                     b.as_mut_ptr() as *mut _, &(ldb as c_int), info)
    }
}

#[inline]
pub fn spbtrs(uplo: u8, n: usize, kd: usize, nrhs: usize, ab: &[f32], ldab: usize, b: &mut [f32],
              ldb: usize, info: &mut i32) {

    unsafe {
        ffi::spbtrs_(&(uplo as c_char), &(n as c_int), &(kd as c_int), &(nrhs as c_int),
                     ab.as_ptr(), &(ldab as c_int), b.as_mut_ptr(), &(ldb as c_int), info)
    }
}

#[inline]
pub fn dpbtrs(uplo: u8, n: usize, kd: usize, nrhs: usize, ab: &[f64], ldab: usize, b: &mut [f64],
              ldb: usize, info: &mut i32) {

    unsafe {
        ffi::dpbtrs_(&(uplo as c_char), &(n as c_int), &(kd as c_int), &(nrhs as c_int),
                     ab.as_ptr(), &(ldab as c_int), b.as_mut_ptr(), &(ldb as c_int), info)
    }
}

#[inline]
pub fn cpbtrs(uplo: u8, n: usize, kd: usize, nrhs: usize, ab: &[c32], ldab: usize, b: &mut [c32],
              ldb: usize, info: &mut i32) {

    unsafe {
        ffi::cpbtrs_(&(uplo as c_char), &(n as c_int), &(kd as c_int), &(nrhs as c_int),
                     ab.as_ptr() as *const _, &(ldab as c_int), b.as_mut_ptr() as *mut _,
                     &(ldb as c_int), info)
    }
}

#[inline]
pub fn zpbtrs(uplo: u8, n: usize, kd: usize, nrhs: usize, ab: &[c64], ldab: usize, b: &mut [c64],
              ldb: usize, info: &mut i32) {

    unsafe {
        ffi::zpbtrs_(&(uplo as c_char), &(n as c_int), &(kd as c_int), &(nrhs as c_int),
                     ab.as_ptr() as *const _, &(ldab as c_int), b.as_mut_ptr() as *mut _,
                     &(ldb as c_int), info)
    }
}

#[inline]
pub fn spttrs(n: usize, nrhs: usize, d: &[f32], e: &[f32], b: &mut [f32], ldb: usize,
              info: &mut i32) {

    unsafe {
        ffi::spttrs_(&(n as c_int), &(nrhs as c_int), d.as_ptr(), e.as_ptr(), b.as_mut_ptr(),
                     &(ldb as c_int), info)
    }
}

#[inline]
pub fn dpttrs(n: usize, nrhs: usize, d: &[f64], e: &[f64], b: &mut [f64], ldb: usize,
              info: &mut i32) {

    unsafe {
        ffi::dpttrs_(&(n as c_int), &(nrhs as c_int), d.as_ptr(), e.as_ptr(), b.as_mut_ptr(),
                     &(ldb as c_int), info)
    }
}

#[inline]
pub fn cpttrs(uplo: u8, n: usize, nrhs: usize, d: &[f32], e: &[c32], b: &mut [c32], ldb: usize,
              info: &mut i32) {

    unsafe {
        ffi::cpttrs_(&(uplo as c_char), &(n as c_int), &(nrhs as c_int), d.as_ptr(),
                     e.as_ptr() as *const _, b.as_mut_ptr() as *mut _, &(ldb as c_int), info)
    }
}

#[inline]
pub fn zpttrs(uplo: u8, n: usize, nrhs: usize, d: &[f64], e: &[c64], b: &mut [c64], ldb: usize,
              info: &mut i32) {

    unsafe {
        ffi::zpttrs_(&(uplo as c_char), &(n as c_int), &(nrhs as c_int), d.as_ptr(),
                     e.as_ptr() as *const _, b.as_mut_ptr() as *mut _, &(ldb as c_int), info)
    }
}

#[inline]
pub fn ssytrs(uplo: u8, n: usize, nrhs: usize, a: &[f32], lda: usize, ipiv: &[i32], b: &mut [f32],
              ldb: usize, info: &mut i32) {

    unsafe {
        ffi::ssytrs_(&(uplo as c_char), &(n as c_int), &(nrhs as c_int), a.as_ptr(),
                     &(lda as c_int), ipiv.as_ptr(), b.as_mut_ptr(), &(ldb as c_int), info)
    }
}

#[inline]
pub fn dsytrs(uplo: u8, n: usize, nrhs: usize, a: &[f64], lda: usize, ipiv: &[i32], b: &mut [f64],
              ldb: usize, info: &mut i32) {

    unsafe {
        ffi::dsytrs_(&(uplo as c_char), &(n as c_int), &(nrhs as c_int), a.as_ptr(),
                     &(lda as c_int), ipiv.as_ptr(), b.as_mut_ptr(), &(ldb as c_int), info)
    }
}

#[inline]
pub fn csytrs(uplo: u8, n: usize, nrhs: usize, a: &[c32], lda: usize, ipiv: &[i32], b: &mut [c32],
              ldb: usize, info: &mut i32) {

    unsafe {
        ffi::csytrs_(&(uplo as c_char), &(n as c_int), &(nrhs as c_int), a.as_ptr() as *const _,
                     &(lda as c_int), ipiv.as_ptr(), b.as_mut_ptr() as *mut _, &(ldb as c_int),
                     info)
    }
}

#[inline]
pub fn zsytrs(uplo: u8, n: usize, nrhs: usize, a: &[c64], lda: usize, ipiv: &[i32], b: &mut [c64],
              ldb: usize, info: &mut i32) {

    unsafe {
        ffi::zsytrs_(&(uplo as c_char), &(n as c_int), &(nrhs as c_int), a.as_ptr() as *const _,
                     &(lda as c_int), ipiv.as_ptr(), b.as_mut_ptr() as *mut _, &(ldb as c_int),
                     info)
    }
}

#[inline]
pub fn chetrs(uplo: u8, n: usize, nrhs: usize, a: &[c32], lda: usize, ipiv: &[i32], b: &mut [c32],
              ldb: usize, info: &mut i32) {

    unsafe {
        ffi::chetrs_(&(uplo as c_char), &(n as c_int), &(nrhs as c_int), a.as_ptr() as *const _,
                     &(lda as c_int), ipiv.as_ptr(), b.as_mut_ptr() as *mut _, &(ldb as c_int),
                     info)
    }
}

#[inline]
pub fn zhetrs(uplo: u8, n: usize, nrhs: usize, a: &[c64], lda: usize, ipiv: &[i32], b: &mut [c64],
              ldb: usize, info: &mut i32) {

    unsafe {
        ffi::zhetrs_(&(uplo as c_char), &(n as c_int), &(nrhs as c_int), a.as_ptr() as *const _,
                     &(lda as c_int), ipiv.as_ptr(), b.as_mut_ptr() as *mut _, &(ldb as c_int),
                     info)
    }
}

#[inline]
pub fn ssptrs(uplo: u8, n: usize, nrhs: usize, ap: &[f32], ipiv: &[i32], b: &mut [f32], ldb: usize,
              info: &mut i32) {

    unsafe {
        ffi::ssptrs_(&(uplo as c_char), &(n as c_int), &(nrhs as c_int), ap.as_ptr(),
                     ipiv.as_ptr(), b.as_mut_ptr(), &(ldb as c_int), info)
    }
}

#[inline]
pub fn dsptrs(uplo: u8, n: usize, nrhs: usize, ap: &[f64], ipiv: &[i32], b: &mut [f64], ldb: usize,
              info: &mut i32) {

    unsafe {
        ffi::dsptrs_(&(uplo as c_char), &(n as c_int), &(nrhs as c_int), ap.as_ptr(),
                     ipiv.as_ptr(), b.as_mut_ptr(), &(ldb as c_int), info)
    }
}

#[inline]
pub fn csptrs(uplo: u8, n: usize, nrhs: usize, ap: &[c32], ipiv: &[i32], b: &mut [c32], ldb: usize,
              info: &mut i32) {

    unsafe {
        ffi::csptrs_(&(uplo as c_char), &(n as c_int), &(nrhs as c_int), ap.as_ptr() as *const _,
                     ipiv.as_ptr(), b.as_mut_ptr() as *mut _, &(ldb as c_int), info)
    }
}

#[inline]
pub fn zsptrs(uplo: u8, n: usize, nrhs: usize, ap: &[c64], ipiv: &[i32], b: &mut [c64], ldb: usize,
              info: &mut i32) {

    unsafe {
        ffi::zsptrs_(&(uplo as c_char), &(n as c_int), &(nrhs as c_int), ap.as_ptr() as *const _,
                     ipiv.as_ptr(), b.as_mut_ptr() as *mut _, &(ldb as c_int), info)
    }
}

#[inline]
pub fn chptrs(uplo: u8, n: usize, nrhs: usize, ap: &[c32], ipiv: &[i32], b: &mut [c32], ldb: usize,
              info: &mut i32) {

    unsafe {
        ffi::chptrs_(&(uplo as c_char), &(n as c_int), &(nrhs as c_int), ap.as_ptr() as *const _,
                     ipiv.as_ptr(), b.as_mut_ptr() as *mut _, &(ldb as c_int), info)
    }
}

#[inline]
pub fn zhptrs(uplo: u8, n: usize, nrhs: usize, ap: &[c64], ipiv: &[i32], b: &mut [c64], ldb: usize,
              info: &mut i32) {

    unsafe {
        ffi::zhptrs_(&(uplo as c_char), &(n as c_int), &(nrhs as c_int), ap.as_ptr() as *const _,
                     ipiv.as_ptr(), b.as_mut_ptr() as *mut _, &(ldb as c_int), info)
    }
}

#[inline]
pub fn strtrs(uplo: u8, trans: u8, diag: u8, n: usize, nrhs: usize, a: &[f32], lda: usize,
              b: &mut [f32], ldb: usize, info: &mut i32) {

    unsafe {
        ffi::strtrs_(&(uplo as c_char), &(trans as c_char), &(diag as c_char), &(n as c_int),
                     &(nrhs as c_int), a.as_ptr(), &(lda as c_int), b.as_mut_ptr(),
                     &(ldb as c_int), info)
    }
}

#[inline]
pub fn dtrtrs(uplo: u8, trans: u8, diag: u8, n: usize, nrhs: usize, a: &[f64], lda: usize,
              b: &mut [f64], ldb: usize, info: &mut i32) {

    unsafe {
        ffi::dtrtrs_(&(uplo as c_char), &(trans as c_char), &(diag as c_char), &(n as c_int),
                     &(nrhs as c_int), a.as_ptr(), &(lda as c_int), b.as_mut_ptr(),
                     &(ldb as c_int), info)
    }
}

#[inline]
pub fn ctrtrs(uplo: u8, trans: u8, diag: u8, n: usize, nrhs: usize, a: &[c32], lda: usize,
              b: &mut [c32], ldb: usize, info: &mut i32) {

    unsafe {
        ffi::ctrtrs_(&(uplo as c_char), &(trans as c_char), &(diag as c_char), &(n as c_int),
                     &(nrhs as c_int), a.as_ptr() as *const _, &(lda as c_int),
                     b.as_mut_ptr() as *mut _, &(ldb as c_int), info)
    }
}

#[inline]
pub fn ztrtrs(uplo: u8, trans: u8, diag: u8, n: usize, nrhs: usize, a: &[c64], lda: usize,
              b: &mut [c64], ldb: usize, info: &mut i32) {

    unsafe {
        ffi::ztrtrs_(&(uplo as c_char), &(trans as c_char), &(diag as c_char), &(n as c_int),
                     &(nrhs as c_int), a.as_ptr() as *const _, &(lda as c_int),
                     b.as_mut_ptr() as *mut _, &(ldb as c_int), info)
    }
}

#[inline]
pub fn stptrs(uplo: u8, trans: u8, diag: u8, n: usize, nrhs: usize, ap: &[f32], b: &mut [f32],
              ldb: usize, info: &mut i32) {

    unsafe {
        ffi::stptrs_(&(uplo as c_char), &(trans as c_char), &(diag as c_char), &(n as c_int),
                     &(nrhs as c_int), ap.as_ptr(), b.as_mut_ptr(), &(ldb as c_int), info)
    }
}

#[inline]
pub fn dtptrs(uplo: u8, trans: u8, diag: u8, n: usize, nrhs: usize, ap: &[f64], b: &mut [f64],
              ldb: usize, info: &mut i32) {

    unsafe {
        ffi::dtptrs_(&(uplo as c_char), &(trans as c_char), &(diag as c_char), &(n as c_int),
                     &(nrhs as c_int), ap.as_ptr(), b.as_mut_ptr(), &(ldb as c_int), info)
    }
}

#[inline]
pub fn ctptrs(uplo: u8, trans: u8, diag: u8, n: usize, nrhs: usize, ap: &[c32], b: &mut [c32],
              ldb: usize, info: &mut i32) {

    unsafe {
        ffi::ctptrs_(&(uplo as c_char), &(trans as c_char), &(diag as c_char), &(n as c_int),
                     &(nrhs as c_int), ap.as_ptr() as *const _, b.as_mut_ptr() as *mut _,
                     &(ldb as c_int), info)
    }
}

#[inline]
pub fn ztptrs(uplo: u8, trans: u8, diag: u8, n: usize, nrhs: usize, ap: &[c64], b: &mut [c64],
              ldb: usize, info: &mut i32) {

    unsafe {
        ffi::ztptrs_(&(uplo as c_char), &(trans as c_char), &(diag as c_char), &(n as c_int),
                     &(nrhs as c_int), ap.as_ptr() as *const _, b.as_mut_ptr() as *mut _,
                     &(ldb as c_int), info)
    }
}

#[inline]
pub fn stbtrs(uplo: u8, trans: u8, diag: u8, n: usize, kd: usize, nrhs: usize, ab: &[f32],
              ldab: usize, b: &mut [f32], ldb: usize, info: &mut i32) {

    unsafe {
        ffi::stbtrs_(&(uplo as c_char), &(trans as c_char), &(diag as c_char), &(n as c_int),
                     &(kd as c_int), &(nrhs as c_int), ab.as_ptr(), &(ldab as c_int),
                     b.as_mut_ptr(), &(ldb as c_int), info)
    }
}

#[inline]
pub fn dtbtrs(uplo: u8, trans: u8, diag: u8, n: usize, kd: usize, nrhs: usize, ab: &[f64],
              ldab: usize, b: &mut [f64], ldb: usize, info: &mut i32) {

    unsafe {
        ffi::dtbtrs_(&(uplo as c_char), &(trans as c_char), &(diag as c_char), &(n as c_int),
                     &(kd as c_int), &(nrhs as c_int), ab.as_ptr(), &(ldab as c_int),
                     b.as_mut_ptr(), &(ldb as c_int), info)
    }
}

#[inline]
pub fn ctbtrs(uplo: u8, trans: u8, diag: u8, n: usize, kd: usize, nrhs: usize, ab: &[c32],
              ldab: usize, b: &mut [c32], ldb: usize, info: &mut i32) {

    unsafe {
        ffi::ctbtrs_(&(uplo as c_char), &(trans as c_char), &(diag as c_char), &(n as c_int),
                     &(kd as c_int), &(nrhs as c_int), ab.as_ptr() as *const _, &(ldab as c_int),
                     b.as_mut_ptr() as *mut _, &(ldb as c_int), info)
    }
}

#[inline]
pub fn ztbtrs(uplo: u8, trans: u8, diag: u8, n: usize, kd: usize, nrhs: usize, ab: &[c64],
              ldab: usize, b: &mut [c64], ldb: usize, info: &mut i32) {

    unsafe {
        ffi::ztbtrs_(&(uplo as c_char), &(trans as c_char), &(diag as c_char), &(n as c_int),
                     &(kd as c_int), &(nrhs as c_int), ab.as_ptr() as *const _, &(ldab as c_int),
                     b.as_mut_ptr() as *mut _, &(ldb as c_int), info)
    }
}

#[inline]
pub fn sgecon(norm: u8, n: usize, a: &[f32], lda: usize, anorm: &[f32], rcond: &mut [f32],
              work: &mut [f32], iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::sgecon_(&(norm as c_char), &(n as c_int), a.as_ptr(), &(lda as c_int), anorm.as_ptr(),
                     rcond.as_mut_ptr(), work.as_mut_ptr(), iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dgecon(norm: u8, n: usize, a: &[f64], lda: usize, anorm: &[f64], rcond: &mut [f64],
              work: &mut [f64], iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::dgecon_(&(norm as c_char), &(n as c_int), a.as_ptr(), &(lda as c_int), anorm.as_ptr(),
                     rcond.as_mut_ptr(), work.as_mut_ptr(), iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn cgecon(norm: u8, n: usize, a: &[c32], lda: usize, anorm: &[f32], rcond: &mut [f32],
              work: &mut [c32], rwork: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::cgecon_(&(norm as c_char), &(n as c_int), a.as_ptr() as *const _, &(lda as c_int),
                     anorm.as_ptr(), rcond.as_mut_ptr(), work.as_mut_ptr() as *mut _,
                     rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zgecon(norm: u8, n: usize, a: &[c64], lda: usize, anorm: &[f64], rcond: &mut [f64],
              work: &mut [c64], rwork: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::zgecon_(&(norm as c_char), &(n as c_int), a.as_ptr() as *const _, &(lda as c_int),
                     anorm.as_ptr(), rcond.as_mut_ptr(), work.as_mut_ptr() as *mut _,
                     rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn sgbcon(norm: u8, n: usize, kl: usize, ku: usize, ab: &[f32], ldab: usize, ipiv: &[i32],
              anorm: &[f32], rcond: &mut [f32], work: &mut [f32], iwork: &mut [i32],
              info: &mut i32) {

    unsafe {
        ffi::sgbcon_(&(norm as c_char), &(n as c_int), &(kl as c_int), &(ku as c_int), ab.as_ptr(),
                     &(ldab as c_int), ipiv.as_ptr(), anorm.as_ptr(), rcond.as_mut_ptr(),
                     work.as_mut_ptr(), iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dgbcon(norm: u8, n: usize, kl: usize, ku: usize, ab: &[f64], ldab: usize, ipiv: &[i32],
              anorm: &[f64], rcond: &mut [f64], work: &mut [f64], iwork: &mut [i32],
              info: &mut i32) {

    unsafe {
        ffi::dgbcon_(&(norm as c_char), &(n as c_int), &(kl as c_int), &(ku as c_int), ab.as_ptr(),
                     &(ldab as c_int), ipiv.as_ptr(), anorm.as_ptr(), rcond.as_mut_ptr(),
                     work.as_mut_ptr(), iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn cgbcon(norm: u8, n: usize, kl: usize, ku: usize, ab: &[c32], ldab: usize, ipiv: &[i32],
              anorm: &[f32], rcond: &mut [f32], work: &mut [c32], rwork: &mut [f32],
              info: &mut i32) {

    unsafe {
        ffi::cgbcon_(&(norm as c_char), &(n as c_int), &(kl as c_int), &(ku as c_int),
                     ab.as_ptr() as *const _, &(ldab as c_int), ipiv.as_ptr(), anorm.as_ptr(),
                     rcond.as_mut_ptr(), work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zgbcon(norm: u8, n: usize, kl: usize, ku: usize, ab: &[c64], ldab: usize, ipiv: &[i32],
              anorm: &[f64], rcond: &mut [f64], work: &mut [c64], rwork: &mut [f64],
              info: &mut i32) {

    unsafe {
        ffi::zgbcon_(&(norm as c_char), &(n as c_int), &(kl as c_int), &(ku as c_int),
                     ab.as_ptr() as *const _, &(ldab as c_int), ipiv.as_ptr(), anorm.as_ptr(),
                     rcond.as_mut_ptr(), work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn sgtcon(norm: u8, n: usize, dl: &[f32], d: &[f32], du: &[f32], du2: &[f32], ipiv: &[i32],
              anorm: &[f32], rcond: &mut [f32], work: &mut [f32], iwork: &mut [i32],
              info: &mut i32) {

    unsafe {
        ffi::sgtcon_(&(norm as c_char), &(n as c_int), dl.as_ptr(), d.as_ptr(), du.as_ptr(),
                     du2.as_ptr(), ipiv.as_ptr(), anorm.as_ptr(), rcond.as_mut_ptr(),
                     work.as_mut_ptr(), iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dgtcon(norm: u8, n: usize, dl: &[f64], d: &[f64], du: &[f64], du2: &[f64], ipiv: &[i32],
              anorm: &[f64], rcond: &mut [f64], work: &mut [f64], iwork: &mut [i32],
              info: &mut i32) {

    unsafe {
        ffi::dgtcon_(&(norm as c_char), &(n as c_int), dl.as_ptr(), d.as_ptr(), du.as_ptr(),
                     du2.as_ptr(), ipiv.as_ptr(), anorm.as_ptr(), rcond.as_mut_ptr(),
                     work.as_mut_ptr(), iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn cgtcon(norm: u8, n: usize, dl: &[c32], d: &[c32], du: &[c32], du2: &[c32], ipiv: &[i32],
              anorm: &[f32], rcond: &mut [f32], work: &mut [c32], info: &mut i32) {

    unsafe {
        ffi::cgtcon_(&(norm as c_char), &(n as c_int), dl.as_ptr() as *const _,
                     d.as_ptr() as *const _, du.as_ptr() as *const _, du2.as_ptr() as *const _,
                     ipiv.as_ptr(), anorm.as_ptr(), rcond.as_mut_ptr(),
                     work.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn zgtcon(norm: u8, n: usize, dl: &[c64], d: &[c64], du: &[c64], du2: &[c64], ipiv: &[i32],
              anorm: &[f64], rcond: &mut [f64], work: &mut [c64], info: &mut i32) {

    unsafe {
        ffi::zgtcon_(&(norm as c_char), &(n as c_int), dl.as_ptr() as *const _,
                     d.as_ptr() as *const _, du.as_ptr() as *const _, du2.as_ptr() as *const _,
                     ipiv.as_ptr(), anorm.as_ptr(), rcond.as_mut_ptr(),
                     work.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn spocon(uplo: u8, n: usize, a: &[f32], lda: usize, anorm: &[f32], rcond: &mut [f32],
              work: &mut [f32], iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::spocon_(&(uplo as c_char), &(n as c_int), a.as_ptr(), &(lda as c_int), anorm.as_ptr(),
                     rcond.as_mut_ptr(), work.as_mut_ptr(), iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dpocon(uplo: u8, n: usize, a: &[f64], lda: usize, anorm: &[f64], rcond: &mut [f64],
              work: &mut [f64], iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::dpocon_(&(uplo as c_char), &(n as c_int), a.as_ptr(), &(lda as c_int), anorm.as_ptr(),
                     rcond.as_mut_ptr(), work.as_mut_ptr(), iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn cpocon(uplo: u8, n: usize, a: &[c32], lda: usize, anorm: &[f32], rcond: &mut [f32],
              work: &mut [c32], rwork: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::cpocon_(&(uplo as c_char), &(n as c_int), a.as_ptr() as *const _, &(lda as c_int),
                     anorm.as_ptr(), rcond.as_mut_ptr(), work.as_mut_ptr() as *mut _,
                     rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zpocon(uplo: u8, n: usize, a: &[c64], lda: usize, anorm: &[f64], rcond: &mut [f64],
              work: &mut [c64], rwork: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::zpocon_(&(uplo as c_char), &(n as c_int), a.as_ptr() as *const _, &(lda as c_int),
                     anorm.as_ptr(), rcond.as_mut_ptr(), work.as_mut_ptr() as *mut _,
                     rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn sppcon(uplo: u8, n: usize, ap: &[f32], anorm: &[f32], rcond: &mut [f32], work: &mut [f32],
              iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::sppcon_(&(uplo as c_char), &(n as c_int), ap.as_ptr(), anorm.as_ptr(),
                     rcond.as_mut_ptr(), work.as_mut_ptr(), iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dppcon(uplo: u8, n: usize, ap: &[f64], anorm: &[f64], rcond: &mut [f64], work: &mut [f64],
              iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::dppcon_(&(uplo as c_char), &(n as c_int), ap.as_ptr(), anorm.as_ptr(),
                     rcond.as_mut_ptr(), work.as_mut_ptr(), iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn cppcon(uplo: u8, n: usize, ap: &[c32], anorm: &[f32], rcond: &mut [f32], work: &mut [c32],
              rwork: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::cppcon_(&(uplo as c_char), &(n as c_int), ap.as_ptr() as *const _, anorm.as_ptr(),
                     rcond.as_mut_ptr(), work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zppcon(uplo: u8, n: usize, ap: &[c64], anorm: &[f64], rcond: &mut [f64], work: &mut [c64],
              rwork: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::zppcon_(&(uplo as c_char), &(n as c_int), ap.as_ptr() as *const _, anorm.as_ptr(),
                     rcond.as_mut_ptr(), work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn spbcon(uplo: u8, n: usize, kd: usize, ab: &[f32], ldab: usize, anorm: &[f32],
              rcond: &mut [f32], work: &mut [f32], iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::spbcon_(&(uplo as c_char), &(n as c_int), &(kd as c_int), ab.as_ptr(),
                     &(ldab as c_int), anorm.as_ptr(), rcond.as_mut_ptr(), work.as_mut_ptr(),
                     iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dpbcon(uplo: u8, n: usize, kd: usize, ab: &[f64], ldab: usize, anorm: &[f64],
              rcond: &mut [f64], work: &mut [f64], iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::dpbcon_(&(uplo as c_char), &(n as c_int), &(kd as c_int), ab.as_ptr(),
                     &(ldab as c_int), anorm.as_ptr(), rcond.as_mut_ptr(), work.as_mut_ptr(),
                     iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn cpbcon(uplo: u8, n: usize, kd: usize, ab: &[c32], ldab: usize, anorm: &[f32],
              rcond: &mut [f32], work: &mut [c32], rwork: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::cpbcon_(&(uplo as c_char), &(n as c_int), &(kd as c_int), ab.as_ptr() as *const _,
                     &(ldab as c_int), anorm.as_ptr(), rcond.as_mut_ptr(),
                     work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zpbcon(uplo: u8, n: usize, kd: usize, ab: &[c64], ldab: usize, anorm: &[f64],
              rcond: &mut [f64], work: &mut [c64], rwork: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::zpbcon_(&(uplo as c_char), &(n as c_int), &(kd as c_int), ab.as_ptr() as *const _,
                     &(ldab as c_int), anorm.as_ptr(), rcond.as_mut_ptr(),
                     work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn sptcon(n: usize, d: &[f32], e: &[f32], anorm: &[f32], rcond: &mut [f32], work: &mut [f32],
              info: &mut i32) {

    unsafe {
        ffi::sptcon_(&(n as c_int), d.as_ptr(), e.as_ptr(), anorm.as_ptr(), rcond.as_mut_ptr(),
                     work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dptcon(n: usize, d: &[f64], e: &[f64], anorm: &[f64], rcond: &mut [f64], work: &mut [f64],
              info: &mut i32) {

    unsafe {
        ffi::dptcon_(&(n as c_int), d.as_ptr(), e.as_ptr(), anorm.as_ptr(), rcond.as_mut_ptr(),
                     work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn cptcon(n: usize, d: &[f32], e: &[c32], anorm: &[f32], rcond: &mut [f32], rwork: &mut [f32],
              info: &mut i32) {

    unsafe {
        ffi::cptcon_(&(n as c_int), d.as_ptr(), e.as_ptr() as *const _, anorm.as_ptr(),
                     rcond.as_mut_ptr(), rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zptcon(n: usize, d: &[f64], e: &[c64], anorm: &[f64], rcond: &mut [f64], rwork: &mut [f64],
              info: &mut i32) {

    unsafe {
        ffi::zptcon_(&(n as c_int), d.as_ptr(), e.as_ptr() as *const _, anorm.as_ptr(),
                     rcond.as_mut_ptr(), rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn ssycon(uplo: u8, n: usize, a: &[f32], lda: usize, ipiv: &[i32], anorm: &[f32],
              rcond: &mut [f32], work: &mut [f32], iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::ssycon_(&(uplo as c_char), &(n as c_int), a.as_ptr(), &(lda as c_int), ipiv.as_ptr(),
                     anorm.as_ptr(), rcond.as_mut_ptr(), work.as_mut_ptr(), iwork.as_mut_ptr(),
                     info)
    }
}

#[inline]
pub fn dsycon(uplo: u8, n: usize, a: &[f64], lda: usize, ipiv: &[i32], anorm: &[f64],
              rcond: &mut [f64], work: &mut [f64], iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::dsycon_(&(uplo as c_char), &(n as c_int), a.as_ptr(), &(lda as c_int), ipiv.as_ptr(),
                     anorm.as_ptr(), rcond.as_mut_ptr(), work.as_mut_ptr(), iwork.as_mut_ptr(),
                     info)
    }
}

#[inline]
pub fn csycon(uplo: u8, n: usize, a: &[c32], lda: usize, ipiv: &[i32], anorm: &[f32],
              rcond: &mut [f32], work: &mut [c32], info: &mut i32) {

    unsafe {
        ffi::csycon_(&(uplo as c_char), &(n as c_int), a.as_ptr() as *const _, &(lda as c_int),
                     ipiv.as_ptr(), anorm.as_ptr(), rcond.as_mut_ptr(),
                     work.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn zsycon(uplo: u8, n: usize, a: &[c64], lda: usize, ipiv: &[i32], anorm: &[f64],
              rcond: &mut [f64], work: &mut [c64], info: &mut i32) {

    unsafe {
        ffi::zsycon_(&(uplo as c_char), &(n as c_int), a.as_ptr() as *const _, &(lda as c_int),
                     ipiv.as_ptr(), anorm.as_ptr(), rcond.as_mut_ptr(),
                     work.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn checon(uplo: u8, n: usize, a: &[c32], lda: usize, ipiv: &[i32], anorm: &[f32],
              rcond: &mut [f32], work: &mut [c32], info: &mut i32) {

    unsafe {
        ffi::checon_(&(uplo as c_char), &(n as c_int), a.as_ptr() as *const _, &(lda as c_int),
                     ipiv.as_ptr(), anorm.as_ptr(), rcond.as_mut_ptr(),
                     work.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn zhecon(uplo: u8, n: usize, a: &[c64], lda: usize, ipiv: &[i32], anorm: &[f64],
              rcond: &mut [f64], work: &mut [c64], info: &mut i32) {

    unsafe {
        ffi::zhecon_(&(uplo as c_char), &(n as c_int), a.as_ptr() as *const _, &(lda as c_int),
                     ipiv.as_ptr(), anorm.as_ptr(), rcond.as_mut_ptr(),
                     work.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn sspcon(uplo: u8, n: usize, ap: &[f32], ipiv: &[i32], anorm: &[f32], rcond: &mut [f32],
              work: &mut [f32], iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::sspcon_(&(uplo as c_char), &(n as c_int), ap.as_ptr(), ipiv.as_ptr(), anorm.as_ptr(),
                     rcond.as_mut_ptr(), work.as_mut_ptr(), iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dspcon(uplo: u8, n: usize, ap: &[f64], ipiv: &[i32], anorm: &[f64], rcond: &mut [f64],
              work: &mut [f64], iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::dspcon_(&(uplo as c_char), &(n as c_int), ap.as_ptr(), ipiv.as_ptr(), anorm.as_ptr(),
                     rcond.as_mut_ptr(), work.as_mut_ptr(), iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn cspcon(uplo: u8, n: usize, ap: &[c32], ipiv: &[i32], anorm: &[f32], rcond: &mut [f32],
              work: &mut [c32], info: &mut i32) {

    unsafe {
        ffi::cspcon_(&(uplo as c_char), &(n as c_int), ap.as_ptr() as *const _, ipiv.as_ptr(),
                     anorm.as_ptr(), rcond.as_mut_ptr(), work.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn zspcon(uplo: u8, n: usize, ap: &[c64], ipiv: &[i32], anorm: &[f64], rcond: &mut [f64],
              work: &mut [c64], info: &mut i32) {

    unsafe {
        ffi::zspcon_(&(uplo as c_char), &(n as c_int), ap.as_ptr() as *const _, ipiv.as_ptr(),
                     anorm.as_ptr(), rcond.as_mut_ptr(), work.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn chpcon(uplo: u8, n: usize, ap: &[c32], ipiv: &[i32], anorm: &[f32], rcond: &mut [f32],
              work: &mut [c32], info: &mut i32) {

    unsafe {
        ffi::chpcon_(&(uplo as c_char), &(n as c_int), ap.as_ptr() as *const _, ipiv.as_ptr(),
                     anorm.as_ptr(), rcond.as_mut_ptr(), work.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn zhpcon(uplo: u8, n: usize, ap: &[c64], ipiv: &[i32], anorm: &[f64], rcond: &mut [f64],
              work: &mut [c64], info: &mut i32) {

    unsafe {
        ffi::zhpcon_(&(uplo as c_char), &(n as c_int), ap.as_ptr() as *const _, ipiv.as_ptr(),
                     anorm.as_ptr(), rcond.as_mut_ptr(), work.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn strcon(norm: u8, uplo: u8, diag: u8, n: usize, a: &[f32], lda: usize, rcond: &mut [f32],
              work: &mut [f32], iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::strcon_(&(norm as c_char), &(uplo as c_char), &(diag as c_char), &(n as c_int),
                     a.as_ptr(), &(lda as c_int), rcond.as_mut_ptr(), work.as_mut_ptr(),
                     iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dtrcon(norm: u8, uplo: u8, diag: u8, n: usize, a: &[f64], lda: usize, rcond: &mut [f64],
              work: &mut [f64], iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::dtrcon_(&(norm as c_char), &(uplo as c_char), &(diag as c_char), &(n as c_int),
                     a.as_ptr(), &(lda as c_int), rcond.as_mut_ptr(), work.as_mut_ptr(),
                     iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn ctrcon(norm: u8, uplo: u8, diag: u8, n: usize, a: &[c32], lda: usize, rcond: &mut [f32],
              work: &mut [c32], rwork: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::ctrcon_(&(norm as c_char), &(uplo as c_char), &(diag as c_char), &(n as c_int),
                     a.as_ptr() as *const _, &(lda as c_int), rcond.as_mut_ptr(),
                     work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn ztrcon(norm: u8, uplo: u8, diag: u8, n: usize, a: &[c64], lda: usize, rcond: &mut [f64],
              work: &mut [c64], rwork: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::ztrcon_(&(norm as c_char), &(uplo as c_char), &(diag as c_char), &(n as c_int),
                     a.as_ptr() as *const _, &(lda as c_int), rcond.as_mut_ptr(),
                     work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn stpcon(norm: u8, uplo: u8, diag: u8, n: usize, ap: &[f32], rcond: &mut [f32],
              work: &mut [f32], iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::stpcon_(&(norm as c_char), &(uplo as c_char), &(diag as c_char), &(n as c_int),
                     ap.as_ptr(), rcond.as_mut_ptr(), work.as_mut_ptr(), iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dtpcon(norm: u8, uplo: u8, diag: u8, n: usize, ap: &[f64], rcond: &mut [f64],
              work: &mut [f64], iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::dtpcon_(&(norm as c_char), &(uplo as c_char), &(diag as c_char), &(n as c_int),
                     ap.as_ptr(), rcond.as_mut_ptr(), work.as_mut_ptr(), iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn ctpcon(norm: u8, uplo: u8, diag: u8, n: usize, ap: &[c32], rcond: &mut [f32],
              work: &mut [c32], rwork: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::ctpcon_(&(norm as c_char), &(uplo as c_char), &(diag as c_char), &(n as c_int),
                     ap.as_ptr() as *const _, rcond.as_mut_ptr(), work.as_mut_ptr() as *mut _,
                     rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn ztpcon(norm: u8, uplo: u8, diag: u8, n: usize, ap: &[c64], rcond: &mut [f64],
              work: &mut [c64], rwork: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::ztpcon_(&(norm as c_char), &(uplo as c_char), &(diag as c_char), &(n as c_int),
                     ap.as_ptr() as *const _, rcond.as_mut_ptr(), work.as_mut_ptr() as *mut _,
                     rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn stbcon(norm: u8, uplo: u8, diag: u8, n: usize, kd: usize, ab: &[f32], ldab: usize,
              rcond: &mut [f32], work: &mut [f32], iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::stbcon_(&(norm as c_char), &(uplo as c_char), &(diag as c_char), &(n as c_int),
                     &(kd as c_int), ab.as_ptr(), &(ldab as c_int), rcond.as_mut_ptr(),
                     work.as_mut_ptr(), iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dtbcon(norm: u8, uplo: u8, diag: u8, n: usize, kd: usize, ab: &[f64], ldab: usize,
              rcond: &mut [f64], work: &mut [f64], iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::dtbcon_(&(norm as c_char), &(uplo as c_char), &(diag as c_char), &(n as c_int),
                     &(kd as c_int), ab.as_ptr(), &(ldab as c_int), rcond.as_mut_ptr(),
                     work.as_mut_ptr(), iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn ctbcon(norm: u8, uplo: u8, diag: u8, n: usize, kd: usize, ab: &[c32], ldab: usize,
              rcond: &mut [f32], work: &mut [c32], rwork: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::ctbcon_(&(norm as c_char), &(uplo as c_char), &(diag as c_char), &(n as c_int),
                     &(kd as c_int), ab.as_ptr() as *const _, &(ldab as c_int), rcond.as_mut_ptr(),
                     work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn ztbcon(norm: u8, uplo: u8, diag: u8, n: usize, kd: usize, ab: &[c64], ldab: usize,
              rcond: &mut [f64], work: &mut [c64], rwork: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::ztbcon_(&(norm as c_char), &(uplo as c_char), &(diag as c_char), &(n as c_int),
                     &(kd as c_int), ab.as_ptr() as *const _, &(ldab as c_int), rcond.as_mut_ptr(),
                     work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn sgerfs(trans: u8, n: usize, nrhs: usize, a: &[f32], lda: usize, af: &[f32], ldaf: usize,
              ipiv: &[i32], b: &[f32], ldb: usize, x: &mut [f32], ldx: usize, ferr: &mut [f32],
              berr: &mut [f32], work: &mut [f32], iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::sgerfs_(&(trans as c_char), &(n as c_int), &(nrhs as c_int), a.as_ptr(),
                     &(lda as c_int), af.as_ptr(), &(ldaf as c_int), ipiv.as_ptr(), b.as_ptr(),
                     &(ldb as c_int), x.as_mut_ptr(), &(ldx as c_int), ferr.as_mut_ptr(),
                     berr.as_mut_ptr(), work.as_mut_ptr(), iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dgerfs(trans: u8, n: usize, nrhs: usize, a: &[f64], lda: usize, af: &[f64], ldaf: usize,
              ipiv: &[i32], b: &[f64], ldb: usize, x: &mut [f64], ldx: usize, ferr: &mut [f64],
              berr: &mut [f64], work: &mut [f64], iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::dgerfs_(&(trans as c_char), &(n as c_int), &(nrhs as c_int), a.as_ptr(),
                     &(lda as c_int), af.as_ptr(), &(ldaf as c_int), ipiv.as_ptr(), b.as_ptr(),
                     &(ldb as c_int), x.as_mut_ptr(), &(ldx as c_int), ferr.as_mut_ptr(),
                     berr.as_mut_ptr(), work.as_mut_ptr(), iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn cgerfs(trans: u8, n: usize, nrhs: usize, a: &[c32], lda: usize, af: &[c32], ldaf: usize,
              ipiv: &[i32], b: &[c32], ldb: usize, x: &mut [c32], ldx: usize, ferr: &mut [f32],
              berr: &mut [f32], work: &mut [c32], rwork: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::cgerfs_(&(trans as c_char), &(n as c_int), &(nrhs as c_int), a.as_ptr() as *const _,
                     &(lda as c_int), af.as_ptr() as *const _, &(ldaf as c_int), ipiv.as_ptr(),
                     b.as_ptr() as *const _, &(ldb as c_int), x.as_mut_ptr() as *mut _,
                     &(ldx as c_int), ferr.as_mut_ptr(), berr.as_mut_ptr(),
                     work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zgerfs(trans: u8, n: usize, nrhs: usize, a: &[c64], lda: usize, af: &[c64], ldaf: usize,
              ipiv: &[i32], b: &[c64], ldb: usize, x: &mut [c64], ldx: usize, ferr: &mut [f64],
              berr: &mut [f64], work: &mut [c64], rwork: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::zgerfs_(&(trans as c_char), &(n as c_int), &(nrhs as c_int), a.as_ptr() as *const _,
                     &(lda as c_int), af.as_ptr() as *const _, &(ldaf as c_int), ipiv.as_ptr(),
                     b.as_ptr() as *const _, &(ldb as c_int), x.as_mut_ptr() as *mut _,
                     &(ldx as c_int), ferr.as_mut_ptr(), berr.as_mut_ptr(),
                     work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn sgerfsx(trans: u8, equed: u8, n: usize, nrhs: usize, a: &[f32], lda: usize, af: &[f32],
               ldaf: usize, ipiv: &[i32], r: &[f32], c: &[f32], b: &[f32], ldb: usize,
               x: &mut [f32], ldx: usize, rcond: &mut [f32], berr: &mut [f32], n_err_bnds: &[i32],
               err_bnds_norm: &mut [f32], err_bnds_comp: &mut [f32], nparams: &[i32],
               params: &mut [f32], work: &mut [f32], iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::sgerfsx_(&(trans as c_char), &(equed as c_char), &(n as c_int), &(nrhs as c_int),
                      a.as_ptr(), &(lda as c_int), af.as_ptr(), &(ldaf as c_int), ipiv.as_ptr(),
                      r.as_ptr(), c.as_ptr(), b.as_ptr(), &(ldb as c_int), x.as_mut_ptr(),
                      &(ldx as c_int), rcond.as_mut_ptr(), berr.as_mut_ptr(), n_err_bnds.as_ptr(),
                      err_bnds_norm.as_mut_ptr(), err_bnds_comp.as_mut_ptr(), nparams.as_ptr(),
                      params.as_mut_ptr(), work.as_mut_ptr(), iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dgerfsx(trans: u8, equed: u8, n: usize, nrhs: usize, a: &[f64], lda: usize, af: &[f64],
               ldaf: usize, ipiv: &[i32], r: &[f64], c: &[f64], b: &[f64], ldb: usize,
               x: &mut [f64], ldx: usize, rcond: &mut [f64], berr: &mut [f64], n_err_bnds: &[i32],
               err_bnds_norm: &mut [f64], err_bnds_comp: &mut [f64], nparams: &[i32],
               params: &mut [f64], work: &mut [f64], iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::dgerfsx_(&(trans as c_char), &(equed as c_char), &(n as c_int), &(nrhs as c_int),
                      a.as_ptr(), &(lda as c_int), af.as_ptr(), &(ldaf as c_int), ipiv.as_ptr(),
                      r.as_ptr(), c.as_ptr(), b.as_ptr(), &(ldb as c_int), x.as_mut_ptr(),
                      &(ldx as c_int), rcond.as_mut_ptr(), berr.as_mut_ptr(), n_err_bnds.as_ptr(),
                      err_bnds_norm.as_mut_ptr(), err_bnds_comp.as_mut_ptr(), nparams.as_ptr(),
                      params.as_mut_ptr(), work.as_mut_ptr(), iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn cgerfsx(trans: u8, equed: u8, n: usize, nrhs: usize, a: &[c32], lda: usize, af: &[c32],
               ldaf: usize, ipiv: &[i32], r: &[f32], c: &[f32], b: &[c32], ldb: usize,
               x: &mut [c32], ldx: usize, rcond: &mut [f32], berr: &mut [f32], n_err_bnds: &[i32],
               err_bnds_norm: &mut [f32], err_bnds_comp: &mut [f32], nparams: &[i32],
               params: &mut [f32], work: &mut [c32], rwork: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::cgerfsx_(&(trans as c_char), &(equed as c_char), &(n as c_int), &(nrhs as c_int),
                      a.as_ptr() as *const _, &(lda as c_int), af.as_ptr() as *const _,
                      &(ldaf as c_int), ipiv.as_ptr(), r.as_ptr(), c.as_ptr(),
                      b.as_ptr() as *const _, &(ldb as c_int), x.as_mut_ptr() as *mut _,
                      &(ldx as c_int), rcond.as_mut_ptr(), berr.as_mut_ptr(), n_err_bnds.as_ptr(),
                      err_bnds_norm.as_mut_ptr(), err_bnds_comp.as_mut_ptr(), nparams.as_ptr(),
                      params.as_mut_ptr(), work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zgerfsx(trans: u8, equed: u8, n: usize, nrhs: usize, a: &[c64], lda: usize, af: &[c64],
               ldaf: usize, ipiv: &[i32], r: &[f64], c: &[f64], b: &[c64], ldb: usize,
               x: &mut [c64], ldx: usize, rcond: &mut [f64], berr: &mut [f64], n_err_bnds: &[i32],
               err_bnds_norm: &mut [f64], err_bnds_comp: &mut [f64], nparams: &[i32],
               params: &mut [f64], work: &mut [c64], rwork: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::zgerfsx_(&(trans as c_char), &(equed as c_char), &(n as c_int), &(nrhs as c_int),
                      a.as_ptr() as *const _, &(lda as c_int), af.as_ptr() as *const _,
                      &(ldaf as c_int), ipiv.as_ptr(), r.as_ptr(), c.as_ptr(),
                      b.as_ptr() as *const _, &(ldb as c_int), x.as_mut_ptr() as *mut _,
                      &(ldx as c_int), rcond.as_mut_ptr(), berr.as_mut_ptr(), n_err_bnds.as_ptr(),
                      err_bnds_norm.as_mut_ptr(), err_bnds_comp.as_mut_ptr(), nparams.as_ptr(),
                      params.as_mut_ptr(), work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn sgbrfs(trans: u8, n: usize, kl: usize, ku: usize, nrhs: usize, ab: &[f32], ldab: usize,
              afb: &[f32], ldafb: usize, ipiv: &[i32], b: &[f32], ldb: usize, x: &mut [f32],
              ldx: usize, ferr: &mut [f32], berr: &mut [f32], work: &mut [f32], iwork: &mut [i32],
              info: &mut i32) {

    unsafe {
        ffi::sgbrfs_(&(trans as c_char), &(n as c_int), &(kl as c_int), &(ku as c_int),
                     &(nrhs as c_int), ab.as_ptr(), &(ldab as c_int), afb.as_ptr(),
                     &(ldafb as c_int), ipiv.as_ptr(), b.as_ptr(), &(ldb as c_int), x.as_mut_ptr(),
                     &(ldx as c_int), ferr.as_mut_ptr(), berr.as_mut_ptr(), work.as_mut_ptr(),
                     iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dgbrfs(trans: u8, n: usize, kl: usize, ku: usize, nrhs: usize, ab: &[f64], ldab: usize,
              afb: &[f64], ldafb: usize, ipiv: &[i32], b: &[f64], ldb: usize, x: &mut [f64],
              ldx: usize, ferr: &mut [f64], berr: &mut [f64], work: &mut [f64], iwork: &mut [i32],
              info: &mut i32) {

    unsafe {
        ffi::dgbrfs_(&(trans as c_char), &(n as c_int), &(kl as c_int), &(ku as c_int),
                     &(nrhs as c_int), ab.as_ptr(), &(ldab as c_int), afb.as_ptr(),
                     &(ldafb as c_int), ipiv.as_ptr(), b.as_ptr(), &(ldb as c_int), x.as_mut_ptr(),
                     &(ldx as c_int), ferr.as_mut_ptr(), berr.as_mut_ptr(), work.as_mut_ptr(),
                     iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn cgbrfs(trans: u8, n: usize, kl: usize, ku: usize, nrhs: usize, ab: &[c32], ldab: usize,
              afb: &[c32], ldafb: usize, ipiv: &[i32], b: &[c32], ldb: usize, x: &mut [c32],
              ldx: usize, ferr: &mut [f32], berr: &mut [f32], work: &mut [c32], rwork: &mut [f32],
              info: &mut i32) {

    unsafe {
        ffi::cgbrfs_(&(trans as c_char), &(n as c_int), &(kl as c_int), &(ku as c_int),
                     &(nrhs as c_int), ab.as_ptr() as *const _, &(ldab as c_int),
                     afb.as_ptr() as *const _, &(ldafb as c_int), ipiv.as_ptr(),
                     b.as_ptr() as *const _, &(ldb as c_int), x.as_mut_ptr() as *mut _,
                     &(ldx as c_int), ferr.as_mut_ptr(), berr.as_mut_ptr(),
                     work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zgbrfs(trans: u8, n: usize, kl: usize, ku: usize, nrhs: usize, ab: &[c64], ldab: usize,
              afb: &[c64], ldafb: usize, ipiv: &[i32], b: &[c64], ldb: usize, x: &mut [c64],
              ldx: usize, ferr: &mut [f64], berr: &mut [f64], work: &mut [c64], rwork: &mut [f64],
              info: &mut i32) {

    unsafe {
        ffi::zgbrfs_(&(trans as c_char), &(n as c_int), &(kl as c_int), &(ku as c_int),
                     &(nrhs as c_int), ab.as_ptr() as *const _, &(ldab as c_int),
                     afb.as_ptr() as *const _, &(ldafb as c_int), ipiv.as_ptr(),
                     b.as_ptr() as *const _, &(ldb as c_int), x.as_mut_ptr() as *mut _,
                     &(ldx as c_int), ferr.as_mut_ptr(), berr.as_mut_ptr(),
                     work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn sgbrfsx(trans: u8, equed: u8, n: usize, kl: usize, ku: usize, nrhs: usize, ab: &[f32],
               ldab: usize, afb: &[f32], ldafb: usize, ipiv: &[i32], r: &[f32], c: &[f32],
               b: &[f32], ldb: usize, x: &mut [f32], ldx: usize, rcond: &mut [f32],
               berr: &mut [f32], n_err_bnds: &[i32], err_bnds_norm: &mut [f32],
               err_bnds_comp: &mut [f32], nparams: &[i32], params: &mut [f32], work: &mut [f32],
               iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::sgbrfsx_(&(trans as c_char), &(equed as c_char), &(n as c_int), &(kl as c_int),
                      &(ku as c_int), &(nrhs as c_int), ab.as_ptr(), &(ldab as c_int),
                      afb.as_ptr(), &(ldafb as c_int), ipiv.as_ptr(), r.as_ptr(), c.as_ptr(),
                      b.as_ptr(), &(ldb as c_int), x.as_mut_ptr(), &(ldx as c_int),
                      rcond.as_mut_ptr(), berr.as_mut_ptr(), n_err_bnds.as_ptr(),
                      err_bnds_norm.as_mut_ptr(), err_bnds_comp.as_mut_ptr(), nparams.as_ptr(),
                      params.as_mut_ptr(), work.as_mut_ptr(), iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dgbrfsx(trans: u8, equed: u8, n: usize, kl: usize, ku: usize, nrhs: usize, ab: &[f64],
               ldab: usize, afb: &[f64], ldafb: usize, ipiv: &[i32], r: &[f64], c: &[f64],
               b: &[f64], ldb: usize, x: &mut [f64], ldx: usize, rcond: &mut [f64],
               berr: &mut [f64], n_err_bnds: &[i32], err_bnds_norm: &mut [f64],
               err_bnds_comp: &mut [f64], nparams: &[i32], params: &mut [f64], work: &mut [f64],
               iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::dgbrfsx_(&(trans as c_char), &(equed as c_char), &(n as c_int), &(kl as c_int),
                      &(ku as c_int), &(nrhs as c_int), ab.as_ptr(), &(ldab as c_int),
                      afb.as_ptr(), &(ldafb as c_int), ipiv.as_ptr(), r.as_ptr(), c.as_ptr(),
                      b.as_ptr(), &(ldb as c_int), x.as_mut_ptr(), &(ldx as c_int),
                      rcond.as_mut_ptr(), berr.as_mut_ptr(), n_err_bnds.as_ptr(),
                      err_bnds_norm.as_mut_ptr(), err_bnds_comp.as_mut_ptr(), nparams.as_ptr(),
                      params.as_mut_ptr(), work.as_mut_ptr(), iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn cgbrfsx(trans: u8, equed: u8, n: usize, kl: usize, ku: usize, nrhs: usize, ab: &[c32],
               ldab: usize, afb: &[c32], ldafb: usize, ipiv: &[i32], r: &[f32], c: &[f32],
               b: &[c32], ldb: usize, x: &mut [c32], ldx: usize, rcond: &mut [f32],
               berr: &mut [f32], n_err_bnds: &[i32], err_bnds_norm: &mut [f32],
               err_bnds_comp: &mut [f32], nparams: &[i32], params: &mut [f32], work: &mut [c32],
               rwork: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::cgbrfsx_(&(trans as c_char), &(equed as c_char), &(n as c_int), &(kl as c_int),
                      &(ku as c_int), &(nrhs as c_int), ab.as_ptr() as *const _, &(ldab as c_int),
                      afb.as_ptr() as *const _, &(ldafb as c_int), ipiv.as_ptr(), r.as_ptr(),
                      c.as_ptr(), b.as_ptr() as *const _, &(ldb as c_int),
                      x.as_mut_ptr() as *mut _, &(ldx as c_int), rcond.as_mut_ptr(),
                      berr.as_mut_ptr(), n_err_bnds.as_ptr(), err_bnds_norm.as_mut_ptr(),
                      err_bnds_comp.as_mut_ptr(), nparams.as_ptr(), params.as_mut_ptr(),
                      work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zgbrfsx(trans: u8, equed: u8, n: usize, kl: usize, ku: usize, nrhs: usize, ab: &[c64],
               ldab: usize, afb: &[c64], ldafb: usize, ipiv: &[i32], r: &[f64], c: &[f64],
               b: &[c64], ldb: usize, x: &mut [c64], ldx: usize, rcond: &mut [f64],
               berr: &mut [f64], n_err_bnds: &[i32], err_bnds_norm: &mut [f64],
               err_bnds_comp: &mut [f64], nparams: &[i32], params: &mut [f64], work: &mut [c64],
               rwork: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::zgbrfsx_(&(trans as c_char), &(equed as c_char), &(n as c_int), &(kl as c_int),
                      &(ku as c_int), &(nrhs as c_int), ab.as_ptr() as *const _, &(ldab as c_int),
                      afb.as_ptr() as *const _, &(ldafb as c_int), ipiv.as_ptr(), r.as_ptr(),
                      c.as_ptr(), b.as_ptr() as *const _, &(ldb as c_int),
                      x.as_mut_ptr() as *mut _, &(ldx as c_int), rcond.as_mut_ptr(),
                      berr.as_mut_ptr(), n_err_bnds.as_ptr(), err_bnds_norm.as_mut_ptr(),
                      err_bnds_comp.as_mut_ptr(), nparams.as_ptr(), params.as_mut_ptr(),
                      work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn sgtrfs(trans: u8, n: usize, nrhs: usize, dl: &[f32], d: &[f32], du: &[f32], dlf: &[f32],
              df: &[f32], duf: &[f32], du2: &[f32], ipiv: &[i32], b: &[f32], ldb: usize,
              x: &mut [f32], ldx: usize, ferr: &mut [f32], berr: &mut [f32], work: &mut [f32],
              iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::sgtrfs_(&(trans as c_char), &(n as c_int), &(nrhs as c_int), dl.as_ptr(), d.as_ptr(),
                     du.as_ptr(), dlf.as_ptr(), df.as_ptr(), duf.as_ptr(), du2.as_ptr(),
                     ipiv.as_ptr(), b.as_ptr(), &(ldb as c_int), x.as_mut_ptr(), &(ldx as c_int),
                     ferr.as_mut_ptr(), berr.as_mut_ptr(), work.as_mut_ptr(), iwork.as_mut_ptr(),
                     info)
    }
}

#[inline]
pub fn dgtrfs(trans: u8, n: usize, nrhs: usize, dl: &[f64], d: &[f64], du: &[f64], dlf: &[f64],
              df: &[f64], duf: &[f64], du2: &[f64], ipiv: &[i32], b: &[f64], ldb: usize,
              x: &mut [f64], ldx: usize, ferr: &mut [f64], berr: &mut [f64], work: &mut [f64],
              iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::dgtrfs_(&(trans as c_char), &(n as c_int), &(nrhs as c_int), dl.as_ptr(), d.as_ptr(),
                     du.as_ptr(), dlf.as_ptr(), df.as_ptr(), duf.as_ptr(), du2.as_ptr(),
                     ipiv.as_ptr(), b.as_ptr(), &(ldb as c_int), x.as_mut_ptr(), &(ldx as c_int),
                     ferr.as_mut_ptr(), berr.as_mut_ptr(), work.as_mut_ptr(), iwork.as_mut_ptr(),
                     info)
    }
}

#[inline]
pub fn cgtrfs(trans: u8, n: usize, nrhs: usize, dl: &[c32], d: &[c32], du: &[c32], dlf: &[c32],
              df: &[c32], duf: &[c32], du2: &[c32], ipiv: &[i32], b: &[c32], ldb: usize,
              x: &mut [c32], ldx: usize, ferr: &mut [f32], berr: &mut [f32], work: &mut [c32],
              rwork: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::cgtrfs_(&(trans as c_char), &(n as c_int), &(nrhs as c_int), dl.as_ptr() as *const _,
                     d.as_ptr() as *const _, du.as_ptr() as *const _, dlf.as_ptr() as *const _,
                     df.as_ptr() as *const _, duf.as_ptr() as *const _, du2.as_ptr() as *const _,
                     ipiv.as_ptr(), b.as_ptr() as *const _, &(ldb as c_int),
                     x.as_mut_ptr() as *mut _, &(ldx as c_int), ferr.as_mut_ptr(),
                     berr.as_mut_ptr(), work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zgtrfs(trans: u8, n: usize, nrhs: usize, dl: &[c64], d: &[c64], du: &[c64], dlf: &[c64],
              df: &[c64], duf: &[c64], du2: &[c64], ipiv: &[i32], b: &[c64], ldb: usize,
              x: &mut [c64], ldx: usize, ferr: &mut [f64], berr: &mut [f64], work: &mut [c64],
              rwork: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::zgtrfs_(&(trans as c_char), &(n as c_int), &(nrhs as c_int), dl.as_ptr() as *const _,
                     d.as_ptr() as *const _, du.as_ptr() as *const _, dlf.as_ptr() as *const _,
                     df.as_ptr() as *const _, duf.as_ptr() as *const _, du2.as_ptr() as *const _,
                     ipiv.as_ptr(), b.as_ptr() as *const _, &(ldb as c_int),
                     x.as_mut_ptr() as *mut _, &(ldx as c_int), ferr.as_mut_ptr(),
                     berr.as_mut_ptr(), work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn sporfs(uplo: u8, n: usize, nrhs: usize, a: &[f32], lda: usize, af: &[f32], ldaf: usize,
              b: &[f32], ldb: usize, x: &mut [f32], ldx: usize, ferr: &mut [f32], berr: &mut [f32],
              work: &mut [f32], iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::sporfs_(&(uplo as c_char), &(n as c_int), &(nrhs as c_int), a.as_ptr(),
                     &(lda as c_int), af.as_ptr(), &(ldaf as c_int), b.as_ptr(), &(ldb as c_int),
                     x.as_mut_ptr(), &(ldx as c_int), ferr.as_mut_ptr(), berr.as_mut_ptr(),
                     work.as_mut_ptr(), iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dporfs(uplo: u8, n: usize, nrhs: usize, a: &[f64], lda: usize, af: &[f64], ldaf: usize,
              b: &[f64], ldb: usize, x: &mut [f64], ldx: usize, ferr: &mut [f64], berr: &mut [f64],
              work: &mut [f64], iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::dporfs_(&(uplo as c_char), &(n as c_int), &(nrhs as c_int), a.as_ptr(),
                     &(lda as c_int), af.as_ptr(), &(ldaf as c_int), b.as_ptr(), &(ldb as c_int),
                     x.as_mut_ptr(), &(ldx as c_int), ferr.as_mut_ptr(), berr.as_mut_ptr(),
                     work.as_mut_ptr(), iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn cporfs(uplo: u8, n: usize, nrhs: usize, a: &[c32], lda: usize, af: &[c32], ldaf: usize,
              b: &[c32], ldb: usize, x: &mut [c32], ldx: usize, ferr: &mut [f32], berr: &mut [f32],
              work: &mut [c32], rwork: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::cporfs_(&(uplo as c_char), &(n as c_int), &(nrhs as c_int), a.as_ptr() as *const _,
                     &(lda as c_int), af.as_ptr() as *const _, &(ldaf as c_int),
                     b.as_ptr() as *const _, &(ldb as c_int), x.as_mut_ptr() as *mut _,
                     &(ldx as c_int), ferr.as_mut_ptr(), berr.as_mut_ptr(),
                     work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zporfs(uplo: u8, n: usize, nrhs: usize, a: &[c64], lda: usize, af: &[c64], ldaf: usize,
              b: &[c64], ldb: usize, x: &mut [c64], ldx: usize, ferr: &mut [f64], berr: &mut [f64],
              work: &mut [c64], rwork: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::zporfs_(&(uplo as c_char), &(n as c_int), &(nrhs as c_int), a.as_ptr() as *const _,
                     &(lda as c_int), af.as_ptr() as *const _, &(ldaf as c_int),
                     b.as_ptr() as *const _, &(ldb as c_int), x.as_mut_ptr() as *mut _,
                     &(ldx as c_int), ferr.as_mut_ptr(), berr.as_mut_ptr(),
                     work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn sporfsx(uplo: u8, equed: u8, n: usize, nrhs: usize, a: &[f32], lda: usize, af: &[f32],
               ldaf: usize, s: &[f32], b: &[f32], ldb: usize, x: &mut [f32], ldx: usize,
               rcond: &mut [f32], berr: &mut [f32], n_err_bnds: &[i32], err_bnds_norm: &mut [f32],
               err_bnds_comp: &mut [f32], nparams: &[i32], params: &mut [f32], work: &mut [f32],
               iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::sporfsx_(&(uplo as c_char), &(equed as c_char), &(n as c_int), &(nrhs as c_int),
                      a.as_ptr(), &(lda as c_int), af.as_ptr(), &(ldaf as c_int), s.as_ptr(),
                      b.as_ptr(), &(ldb as c_int), x.as_mut_ptr(), &(ldx as c_int),
                      rcond.as_mut_ptr(), berr.as_mut_ptr(), n_err_bnds.as_ptr(),
                      err_bnds_norm.as_mut_ptr(), err_bnds_comp.as_mut_ptr(), nparams.as_ptr(),
                      params.as_mut_ptr(), work.as_mut_ptr(), iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dporfsx(uplo: u8, equed: u8, n: usize, nrhs: usize, a: &[f64], lda: usize, af: &[f64],
               ldaf: usize, s: &[f64], b: &[f64], ldb: usize, x: &mut [f64], ldx: usize,
               rcond: &mut [f64], berr: &mut [f64], n_err_bnds: &[i32], err_bnds_norm: &mut [f64],
               err_bnds_comp: &mut [f64], nparams: &[i32], params: &mut [f64], work: &mut [f64],
               iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::dporfsx_(&(uplo as c_char), &(equed as c_char), &(n as c_int), &(nrhs as c_int),
                      a.as_ptr(), &(lda as c_int), af.as_ptr(), &(ldaf as c_int), s.as_ptr(),
                      b.as_ptr(), &(ldb as c_int), x.as_mut_ptr(), &(ldx as c_int),
                      rcond.as_mut_ptr(), berr.as_mut_ptr(), n_err_bnds.as_ptr(),
                      err_bnds_norm.as_mut_ptr(), err_bnds_comp.as_mut_ptr(), nparams.as_ptr(),
                      params.as_mut_ptr(), work.as_mut_ptr(), iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn cporfsx(uplo: u8, equed: u8, n: usize, nrhs: usize, a: &[c32], lda: usize, af: &[c32],
               ldaf: usize, s: &[f32], b: &[c32], ldb: usize, x: &mut [c32], ldx: usize,
               rcond: &mut [f32], berr: &mut [f32], n_err_bnds: &[i32], err_bnds_norm: &mut [f32],
               err_bnds_comp: &mut [f32], nparams: &[i32], params: &mut [f32], work: &mut [c32],
               rwork: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::cporfsx_(&(uplo as c_char), &(equed as c_char), &(n as c_int), &(nrhs as c_int),
                      a.as_ptr() as *const _, &(lda as c_int), af.as_ptr() as *const _,
                      &(ldaf as c_int), s.as_ptr(), b.as_ptr() as *const _, &(ldb as c_int),
                      x.as_mut_ptr() as *mut _, &(ldx as c_int), rcond.as_mut_ptr(),
                      berr.as_mut_ptr(), n_err_bnds.as_ptr(), err_bnds_norm.as_mut_ptr(),
                      err_bnds_comp.as_mut_ptr(), nparams.as_ptr(), params.as_mut_ptr(),
                      work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zporfsx(uplo: u8, equed: u8, n: usize, nrhs: usize, a: &[c64], lda: usize, af: &[c64],
               ldaf: usize, s: &[f64], b: &[c64], ldb: usize, x: &mut [c64], ldx: usize,
               rcond: &mut [f64], berr: &mut [f64], n_err_bnds: &[i32], err_bnds_norm: &mut [f64],
               err_bnds_comp: &mut [f64], nparams: &[i32], params: &mut [f64], work: &mut [c64],
               rwork: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::zporfsx_(&(uplo as c_char), &(equed as c_char), &(n as c_int), &(nrhs as c_int),
                      a.as_ptr() as *const _, &(lda as c_int), af.as_ptr() as *const _,
                      &(ldaf as c_int), s.as_ptr(), b.as_ptr() as *const _, &(ldb as c_int),
                      x.as_mut_ptr() as *mut _, &(ldx as c_int), rcond.as_mut_ptr(),
                      berr.as_mut_ptr(), n_err_bnds.as_ptr(), err_bnds_norm.as_mut_ptr(),
                      err_bnds_comp.as_mut_ptr(), nparams.as_ptr(), params.as_mut_ptr(),
                      work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn spprfs(uplo: u8, n: usize, nrhs: usize, ap: &[f32], afp: &[f32], b: &[f32], ldb: usize,
              x: &mut [f32], ldx: usize, ferr: &mut [f32], berr: &mut [f32], work: &mut [f32],
              iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::spprfs_(&(uplo as c_char), &(n as c_int), &(nrhs as c_int), ap.as_ptr(), afp.as_ptr(),
                     b.as_ptr(), &(ldb as c_int), x.as_mut_ptr(), &(ldx as c_int),
                     ferr.as_mut_ptr(), berr.as_mut_ptr(), work.as_mut_ptr(), iwork.as_mut_ptr(),
                     info)
    }
}

#[inline]
pub fn dpprfs(uplo: u8, n: usize, nrhs: usize, ap: &[f64], afp: &[f64], b: &[f64], ldb: usize,
              x: &mut [f64], ldx: usize, ferr: &mut [f64], berr: &mut [f64], work: &mut [f64],
              iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::dpprfs_(&(uplo as c_char), &(n as c_int), &(nrhs as c_int), ap.as_ptr(), afp.as_ptr(),
                     b.as_ptr(), &(ldb as c_int), x.as_mut_ptr(), &(ldx as c_int),
                     ferr.as_mut_ptr(), berr.as_mut_ptr(), work.as_mut_ptr(), iwork.as_mut_ptr(),
                     info)
    }
}

#[inline]
pub fn cpprfs(uplo: u8, n: usize, nrhs: usize, ap: &[c32], afp: &[c32], b: &[c32], ldb: usize,
              x: &mut [c32], ldx: usize, ferr: &mut [f32], berr: &mut [f32], work: &mut [c32],
              rwork: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::cpprfs_(&(uplo as c_char), &(n as c_int), &(nrhs as c_int), ap.as_ptr() as *const _,
                     afp.as_ptr() as *const _, b.as_ptr() as *const _, &(ldb as c_int),
                     x.as_mut_ptr() as *mut _, &(ldx as c_int), ferr.as_mut_ptr(),
                     berr.as_mut_ptr(), work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zpprfs(uplo: u8, n: usize, nrhs: usize, ap: &[c64], afp: &[c64], b: &[c64], ldb: usize,
              x: &mut [c64], ldx: usize, ferr: &mut [f64], berr: &mut [f64], work: &mut [c64],
              rwork: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::zpprfs_(&(uplo as c_char), &(n as c_int), &(nrhs as c_int), ap.as_ptr() as *const _,
                     afp.as_ptr() as *const _, b.as_ptr() as *const _, &(ldb as c_int),
                     x.as_mut_ptr() as *mut _, &(ldx as c_int), ferr.as_mut_ptr(),
                     berr.as_mut_ptr(), work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn spbrfs(uplo: u8, n: usize, kd: usize, nrhs: usize, ab: &[f32], ldab: usize, afb: &[f32],
              ldafb: usize, b: &[f32], ldb: usize, x: &mut [f32], ldx: usize, ferr: &mut [f32],
              berr: &mut [f32], work: &mut [f32], iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::spbrfs_(&(uplo as c_char), &(n as c_int), &(kd as c_int), &(nrhs as c_int),
                     ab.as_ptr(), &(ldab as c_int), afb.as_ptr(), &(ldafb as c_int), b.as_ptr(),
                     &(ldb as c_int), x.as_mut_ptr(), &(ldx as c_int), ferr.as_mut_ptr(),
                     berr.as_mut_ptr(), work.as_mut_ptr(), iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dpbrfs(uplo: u8, n: usize, kd: usize, nrhs: usize, ab: &[f64], ldab: usize, afb: &[f64],
              ldafb: usize, b: &[f64], ldb: usize, x: &mut [f64], ldx: usize, ferr: &mut [f64],
              berr: &mut [f64], work: &mut [f64], iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::dpbrfs_(&(uplo as c_char), &(n as c_int), &(kd as c_int), &(nrhs as c_int),
                     ab.as_ptr(), &(ldab as c_int), afb.as_ptr(), &(ldafb as c_int), b.as_ptr(),
                     &(ldb as c_int), x.as_mut_ptr(), &(ldx as c_int), ferr.as_mut_ptr(),
                     berr.as_mut_ptr(), work.as_mut_ptr(), iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn cpbrfs(uplo: u8, n: usize, kd: usize, nrhs: usize, ab: &[c32], ldab: usize, afb: &[c32],
              ldafb: usize, b: &[c32], ldb: usize, x: &mut [c32], ldx: usize, ferr: &mut [f32],
              berr: &mut [f32], work: &mut [c32], rwork: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::cpbrfs_(&(uplo as c_char), &(n as c_int), &(kd as c_int), &(nrhs as c_int),
                     ab.as_ptr() as *const _, &(ldab as c_int), afb.as_ptr() as *const _,
                     &(ldafb as c_int), b.as_ptr() as *const _, &(ldb as c_int),
                     x.as_mut_ptr() as *mut _, &(ldx as c_int), ferr.as_mut_ptr(),
                     berr.as_mut_ptr(), work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zpbrfs(uplo: u8, n: usize, kd: usize, nrhs: usize, ab: &[c64], ldab: usize, afb: &[c64],
              ldafb: usize, b: &[c64], ldb: usize, x: &mut [c64], ldx: usize, ferr: &mut [f64],
              berr: &mut [f64], work: &mut [c64], rwork: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::zpbrfs_(&(uplo as c_char), &(n as c_int), &(kd as c_int), &(nrhs as c_int),
                     ab.as_ptr() as *const _, &(ldab as c_int), afb.as_ptr() as *const _,
                     &(ldafb as c_int), b.as_ptr() as *const _, &(ldb as c_int),
                     x.as_mut_ptr() as *mut _, &(ldx as c_int), ferr.as_mut_ptr(),
                     berr.as_mut_ptr(), work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn sptrfs(n: usize, nrhs: usize, d: &[f32], e: &[f32], df: &[f32], ef: &[f32], b: &[f32],
              ldb: usize, x: &mut [f32], ldx: usize, ferr: &mut [f32], berr: &mut [f32],
              work: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::sptrfs_(&(n as c_int), &(nrhs as c_int), d.as_ptr(), e.as_ptr(), df.as_ptr(),
                     ef.as_ptr(), b.as_ptr(), &(ldb as c_int), x.as_mut_ptr(), &(ldx as c_int),
                     ferr.as_mut_ptr(), berr.as_mut_ptr(), work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dptrfs(n: usize, nrhs: usize, d: &[f64], e: &[f64], df: &[f64], ef: &[f64], b: &[f64],
              ldb: usize, x: &mut [f64], ldx: usize, ferr: &mut [f64], berr: &mut [f64],
              work: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::dptrfs_(&(n as c_int), &(nrhs as c_int), d.as_ptr(), e.as_ptr(), df.as_ptr(),
                     ef.as_ptr(), b.as_ptr(), &(ldb as c_int), x.as_mut_ptr(), &(ldx as c_int),
                     ferr.as_mut_ptr(), berr.as_mut_ptr(), work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn cptrfs(uplo: u8, n: usize, nrhs: usize, d: &[f32], e: &[c32], df: &[f32], ef: &[c32],
              b: &[c32], ldb: usize, x: &mut [c32], ldx: usize, ferr: &mut [f32], berr: &mut [f32],
              work: &mut [c32], rwork: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::cptrfs_(&(uplo as c_char), &(n as c_int), &(nrhs as c_int), d.as_ptr(),
                     e.as_ptr() as *const _, df.as_ptr(), ef.as_ptr() as *const _,
                     b.as_ptr() as *const _, &(ldb as c_int), x.as_mut_ptr() as *mut _,
                     &(ldx as c_int), ferr.as_mut_ptr(), berr.as_mut_ptr(),
                     work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zptrfs(uplo: u8, n: usize, nrhs: usize, d: &[f64], e: &[c64], df: &[f64], ef: &[c64],
              b: &[c64], ldb: usize, x: &mut [c64], ldx: usize, ferr: &mut [f64], berr: &mut [f64],
              work: &mut [c64], rwork: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::zptrfs_(&(uplo as c_char), &(n as c_int), &(nrhs as c_int), d.as_ptr(),
                     e.as_ptr() as *const _, df.as_ptr(), ef.as_ptr() as *const _,
                     b.as_ptr() as *const _, &(ldb as c_int), x.as_mut_ptr() as *mut _,
                     &(ldx as c_int), ferr.as_mut_ptr(), berr.as_mut_ptr(),
                     work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn ssyrfs(uplo: u8, n: usize, nrhs: usize, a: &[f32], lda: usize, af: &[f32], ldaf: usize,
              ipiv: &[i32], b: &[f32], ldb: usize, x: &mut [f32], ldx: usize, ferr: &mut [f32],
              berr: &mut [f32], work: &mut [f32], iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::ssyrfs_(&(uplo as c_char), &(n as c_int), &(nrhs as c_int), a.as_ptr(),
                     &(lda as c_int), af.as_ptr(), &(ldaf as c_int), ipiv.as_ptr(), b.as_ptr(),
                     &(ldb as c_int), x.as_mut_ptr(), &(ldx as c_int), ferr.as_mut_ptr(),
                     berr.as_mut_ptr(), work.as_mut_ptr(), iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dsyrfs(uplo: u8, n: usize, nrhs: usize, a: &[f64], lda: usize, af: &[f64], ldaf: usize,
              ipiv: &[i32], b: &[f64], ldb: usize, x: &mut [f64], ldx: usize, ferr: &mut [f64],
              berr: &mut [f64], work: &mut [f64], iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::dsyrfs_(&(uplo as c_char), &(n as c_int), &(nrhs as c_int), a.as_ptr(),
                     &(lda as c_int), af.as_ptr(), &(ldaf as c_int), ipiv.as_ptr(), b.as_ptr(),
                     &(ldb as c_int), x.as_mut_ptr(), &(ldx as c_int), ferr.as_mut_ptr(),
                     berr.as_mut_ptr(), work.as_mut_ptr(), iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn csyrfs(uplo: u8, n: usize, nrhs: usize, a: &[c32], lda: usize, af: &[c32], ldaf: usize,
              ipiv: &[i32], b: &[c32], ldb: usize, x: &mut [c32], ldx: usize, ferr: &mut [f32],
              berr: &mut [f32], work: &mut [c32], rwork: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::csyrfs_(&(uplo as c_char), &(n as c_int), &(nrhs as c_int), a.as_ptr() as *const _,
                     &(lda as c_int), af.as_ptr() as *const _, &(ldaf as c_int), ipiv.as_ptr(),
                     b.as_ptr() as *const _, &(ldb as c_int), x.as_mut_ptr() as *mut _,
                     &(ldx as c_int), ferr.as_mut_ptr(), berr.as_mut_ptr(),
                     work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zsyrfs(uplo: u8, n: usize, nrhs: usize, a: &[c64], lda: usize, af: &[c64], ldaf: usize,
              ipiv: &[i32], b: &[c64], ldb: usize, x: &mut [c64], ldx: usize, ferr: &mut [f64],
              berr: &mut [f64], work: &mut [c64], rwork: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::zsyrfs_(&(uplo as c_char), &(n as c_int), &(nrhs as c_int), a.as_ptr() as *const _,
                     &(lda as c_int), af.as_ptr() as *const _, &(ldaf as c_int), ipiv.as_ptr(),
                     b.as_ptr() as *const _, &(ldb as c_int), x.as_mut_ptr() as *mut _,
                     &(ldx as c_int), ferr.as_mut_ptr(), berr.as_mut_ptr(),
                     work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn ssyrfsx(uplo: u8, equed: u8, n: usize, nrhs: usize, a: &[f32], lda: usize, af: &[f32],
               ldaf: usize, ipiv: &[i32], s: &[f32], b: &[f32], ldb: usize, x: &mut [f32],
               ldx: usize, rcond: &mut [f32], berr: &mut [f32], n_err_bnds: &[i32],
               err_bnds_norm: &mut [f32], err_bnds_comp: &mut [f32], nparams: &[i32],
               params: &mut [f32], work: &mut [f32], iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::ssyrfsx_(&(uplo as c_char), &(equed as c_char), &(n as c_int), &(nrhs as c_int),
                      a.as_ptr(), &(lda as c_int), af.as_ptr(), &(ldaf as c_int), ipiv.as_ptr(),
                      s.as_ptr(), b.as_ptr(), &(ldb as c_int), x.as_mut_ptr(), &(ldx as c_int),
                      rcond.as_mut_ptr(), berr.as_mut_ptr(), n_err_bnds.as_ptr(),
                      err_bnds_norm.as_mut_ptr(), err_bnds_comp.as_mut_ptr(), nparams.as_ptr(),
                      params.as_mut_ptr(), work.as_mut_ptr(), iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dsyrfsx(uplo: u8, equed: u8, n: usize, nrhs: usize, a: &[f64], lda: usize, af: &[f64],
               ldaf: usize, ipiv: &[i32], s: &[f64], b: &[f64], ldb: usize, x: &mut [f64],
               ldx: usize, rcond: &mut [f64], berr: &mut [f64], n_err_bnds: &[i32],
               err_bnds_norm: &mut [f64], err_bnds_comp: &mut [f64], nparams: &[i32],
               params: &mut [f64], work: &mut [f64], iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::dsyrfsx_(&(uplo as c_char), &(equed as c_char), &(n as c_int), &(nrhs as c_int),
                      a.as_ptr(), &(lda as c_int), af.as_ptr(), &(ldaf as c_int), ipiv.as_ptr(),
                      s.as_ptr(), b.as_ptr(), &(ldb as c_int), x.as_mut_ptr(), &(ldx as c_int),
                      rcond.as_mut_ptr(), berr.as_mut_ptr(), n_err_bnds.as_ptr(),
                      err_bnds_norm.as_mut_ptr(), err_bnds_comp.as_mut_ptr(), nparams.as_ptr(),
                      params.as_mut_ptr(), work.as_mut_ptr(), iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn csyrfsx(uplo: u8, equed: u8, n: usize, nrhs: usize, a: &[c32], lda: usize, af: &[c32],
               ldaf: usize, ipiv: &[i32], s: &[f32], b: &[c32], ldb: usize, x: &mut [c32],
               ldx: usize, rcond: &mut [f32], berr: &mut [f32], n_err_bnds: &[i32],
               err_bnds_norm: &mut [f32], err_bnds_comp: &mut [f32], nparams: &[i32],
               params: &mut [f32], work: &mut [c32], rwork: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::csyrfsx_(&(uplo as c_char), &(equed as c_char), &(n as c_int), &(nrhs as c_int),
                      a.as_ptr() as *const _, &(lda as c_int), af.as_ptr() as *const _,
                      &(ldaf as c_int), ipiv.as_ptr(), s.as_ptr(), b.as_ptr() as *const _,
                      &(ldb as c_int), x.as_mut_ptr() as *mut _, &(ldx as c_int),
                      rcond.as_mut_ptr(), berr.as_mut_ptr(), n_err_bnds.as_ptr(),
                      err_bnds_norm.as_mut_ptr(), err_bnds_comp.as_mut_ptr(), nparams.as_ptr(),
                      params.as_mut_ptr(), work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zsyrfsx(uplo: u8, equed: u8, n: usize, nrhs: usize, a: &[c64], lda: usize, af: &[c64],
               ldaf: usize, ipiv: &[i32], s: &[f64], b: &[c64], ldb: usize, x: &mut [c64],
               ldx: usize, rcond: &mut [f64], berr: &mut [f64], n_err_bnds: &[i32],
               err_bnds_norm: &mut [f64], err_bnds_comp: &mut [f64], nparams: &[i32],
               params: &mut [f64], work: &mut [c64], rwork: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::zsyrfsx_(&(uplo as c_char), &(equed as c_char), &(n as c_int), &(nrhs as c_int),
                      a.as_ptr() as *const _, &(lda as c_int), af.as_ptr() as *const _,
                      &(ldaf as c_int), ipiv.as_ptr(), s.as_ptr(), b.as_ptr() as *const _,
                      &(ldb as c_int), x.as_mut_ptr() as *mut _, &(ldx as c_int),
                      rcond.as_mut_ptr(), berr.as_mut_ptr(), n_err_bnds.as_ptr(),
                      err_bnds_norm.as_mut_ptr(), err_bnds_comp.as_mut_ptr(), nparams.as_ptr(),
                      params.as_mut_ptr(), work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn cherfs(uplo: u8, n: usize, nrhs: usize, a: &[c32], lda: usize, af: &[c32], ldaf: usize,
              ipiv: &[i32], b: &[c32], ldb: usize, x: &mut [c32], ldx: usize, ferr: &mut [f32],
              berr: &mut [f32], work: &mut [c32], rwork: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::cherfs_(&(uplo as c_char), &(n as c_int), &(nrhs as c_int), a.as_ptr() as *const _,
                     &(lda as c_int), af.as_ptr() as *const _, &(ldaf as c_int), ipiv.as_ptr(),
                     b.as_ptr() as *const _, &(ldb as c_int), x.as_mut_ptr() as *mut _,
                     &(ldx as c_int), ferr.as_mut_ptr(), berr.as_mut_ptr(),
                     work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zherfs(uplo: u8, n: usize, nrhs: usize, a: &[c64], lda: usize, af: &[c64], ldaf: usize,
              ipiv: &[i32], b: &[c64], ldb: usize, x: &mut [c64], ldx: usize, ferr: &mut [f64],
              berr: &mut [f64], work: &mut [c64], rwork: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::zherfs_(&(uplo as c_char), &(n as c_int), &(nrhs as c_int), a.as_ptr() as *const _,
                     &(lda as c_int), af.as_ptr() as *const _, &(ldaf as c_int), ipiv.as_ptr(),
                     b.as_ptr() as *const _, &(ldb as c_int), x.as_mut_ptr() as *mut _,
                     &(ldx as c_int), ferr.as_mut_ptr(), berr.as_mut_ptr(),
                     work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn cherfsx(uplo: u8, equed: u8, n: usize, nrhs: usize, a: &[c32], lda: usize, af: &[c32],
               ldaf: usize, ipiv: &[i32], s: &[f32], b: &[c32], ldb: usize, x: &mut [c32],
               ldx: usize, rcond: &mut [f32], berr: &mut [f32], n_err_bnds: &[i32],
               err_bnds_norm: &mut [f32], err_bnds_comp: &mut [f32], nparams: &[i32],
               params: &mut [f32], work: &mut [c32], rwork: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::cherfsx_(&(uplo as c_char), &(equed as c_char), &(n as c_int), &(nrhs as c_int),
                      a.as_ptr() as *const _, &(lda as c_int), af.as_ptr() as *const _,
                      &(ldaf as c_int), ipiv.as_ptr(), s.as_ptr(), b.as_ptr() as *const _,
                      &(ldb as c_int), x.as_mut_ptr() as *mut _, &(ldx as c_int),
                      rcond.as_mut_ptr(), berr.as_mut_ptr(), n_err_bnds.as_ptr(),
                      err_bnds_norm.as_mut_ptr(), err_bnds_comp.as_mut_ptr(), nparams.as_ptr(),
                      params.as_mut_ptr(), work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zherfsx(uplo: u8, equed: u8, n: usize, nrhs: usize, a: &[c64], lda: usize, af: &[c64],
               ldaf: usize, ipiv: &[i32], s: &[f64], b: &[c64], ldb: usize, x: &mut [c64],
               ldx: usize, rcond: &mut [f64], berr: &mut [f64], n_err_bnds: &[i32],
               err_bnds_norm: &mut [f64], err_bnds_comp: &mut [f64], nparams: &[i32],
               params: &mut [f64], work: &mut [c64], rwork: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::zherfsx_(&(uplo as c_char), &(equed as c_char), &(n as c_int), &(nrhs as c_int),
                      a.as_ptr() as *const _, &(lda as c_int), af.as_ptr() as *const _,
                      &(ldaf as c_int), ipiv.as_ptr(), s.as_ptr(), b.as_ptr() as *const _,
                      &(ldb as c_int), x.as_mut_ptr() as *mut _, &(ldx as c_int),
                      rcond.as_mut_ptr(), berr.as_mut_ptr(), n_err_bnds.as_ptr(),
                      err_bnds_norm.as_mut_ptr(), err_bnds_comp.as_mut_ptr(), nparams.as_ptr(),
                      params.as_mut_ptr(), work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn ssprfs(uplo: u8, n: usize, nrhs: usize, ap: &[f32], afp: &[f32], ipiv: &[i32], b: &[f32],
              ldb: usize, x: &mut [f32], ldx: usize, ferr: &mut [f32], berr: &mut [f32],
              work: &mut [f32], iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::ssprfs_(&(uplo as c_char), &(n as c_int), &(nrhs as c_int), ap.as_ptr(), afp.as_ptr(),
                     ipiv.as_ptr(), b.as_ptr(), &(ldb as c_int), x.as_mut_ptr(), &(ldx as c_int),
                     ferr.as_mut_ptr(), berr.as_mut_ptr(), work.as_mut_ptr(), iwork.as_mut_ptr(),
                     info)
    }
}

#[inline]
pub fn dsprfs(uplo: u8, n: usize, nrhs: usize, ap: &[f64], afp: &[f64], ipiv: &[i32], b: &[f64],
              ldb: usize, x: &mut [f64], ldx: usize, ferr: &mut [f64], berr: &mut [f64],
              work: &mut [f64], iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::dsprfs_(&(uplo as c_char), &(n as c_int), &(nrhs as c_int), ap.as_ptr(), afp.as_ptr(),
                     ipiv.as_ptr(), b.as_ptr(), &(ldb as c_int), x.as_mut_ptr(), &(ldx as c_int),
                     ferr.as_mut_ptr(), berr.as_mut_ptr(), work.as_mut_ptr(), iwork.as_mut_ptr(),
                     info)
    }
}

#[inline]
pub fn csprfs(uplo: u8, n: usize, nrhs: usize, ap: &[c32], afp: &[c32], ipiv: &[i32], b: &[c32],
              ldb: usize, x: &mut [c32], ldx: usize, ferr: &mut [f32], berr: &mut [f32],
              work: &mut [c32], rwork: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::csprfs_(&(uplo as c_char), &(n as c_int), &(nrhs as c_int), ap.as_ptr() as *const _,
                     afp.as_ptr() as *const _, ipiv.as_ptr(), b.as_ptr() as *const _,
                     &(ldb as c_int), x.as_mut_ptr() as *mut _, &(ldx as c_int), ferr.as_mut_ptr(),
                     berr.as_mut_ptr(), work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zsprfs(uplo: u8, n: usize, nrhs: usize, ap: &[c64], afp: &[c64], ipiv: &[i32], b: &[c64],
              ldb: usize, x: &mut [c64], ldx: usize, ferr: &mut [f64], berr: &mut [f64],
              work: &mut [c64], rwork: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::zsprfs_(&(uplo as c_char), &(n as c_int), &(nrhs as c_int), ap.as_ptr() as *const _,
                     afp.as_ptr() as *const _, ipiv.as_ptr(), b.as_ptr() as *const _,
                     &(ldb as c_int), x.as_mut_ptr() as *mut _, &(ldx as c_int), ferr.as_mut_ptr(),
                     berr.as_mut_ptr(), work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn chprfs(uplo: u8, n: usize, nrhs: usize, ap: &[c32], afp: &[c32], ipiv: &[i32], b: &[c32],
              ldb: usize, x: &mut [c32], ldx: usize, ferr: &mut [f32], berr: &mut [f32],
              work: &mut [c32], rwork: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::chprfs_(&(uplo as c_char), &(n as c_int), &(nrhs as c_int), ap.as_ptr() as *const _,
                     afp.as_ptr() as *const _, ipiv.as_ptr(), b.as_ptr() as *const _,
                     &(ldb as c_int), x.as_mut_ptr() as *mut _, &(ldx as c_int), ferr.as_mut_ptr(),
                     berr.as_mut_ptr(), work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zhprfs(uplo: u8, n: usize, nrhs: usize, ap: &[c64], afp: &[c64], ipiv: &[i32], b: &[c64],
              ldb: usize, x: &mut [c64], ldx: usize, ferr: &mut [f64], berr: &mut [f64],
              work: &mut [c64], rwork: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::zhprfs_(&(uplo as c_char), &(n as c_int), &(nrhs as c_int), ap.as_ptr() as *const _,
                     afp.as_ptr() as *const _, ipiv.as_ptr(), b.as_ptr() as *const _,
                     &(ldb as c_int), x.as_mut_ptr() as *mut _, &(ldx as c_int), ferr.as_mut_ptr(),
                     berr.as_mut_ptr(), work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn strrfs(uplo: u8, trans: u8, diag: u8, n: usize, nrhs: usize, a: &[f32], lda: usize,
              b: &[f32], ldb: usize, x: &[f32], ldx: usize, ferr: &mut [f32], berr: &mut [f32],
              work: &mut [f32], iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::strrfs_(&(uplo as c_char), &(trans as c_char), &(diag as c_char), &(n as c_int),
                     &(nrhs as c_int), a.as_ptr(), &(lda as c_int), b.as_ptr(), &(ldb as c_int),
                     x.as_ptr(), &(ldx as c_int), ferr.as_mut_ptr(), berr.as_mut_ptr(),
                     work.as_mut_ptr(), iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dtrrfs(uplo: u8, trans: u8, diag: u8, n: usize, nrhs: usize, a: &[f64], lda: usize,
              b: &[f64], ldb: usize, x: &[f64], ldx: usize, ferr: &mut [f64], berr: &mut [f64],
              work: &mut [f64], iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::dtrrfs_(&(uplo as c_char), &(trans as c_char), &(diag as c_char), &(n as c_int),
                     &(nrhs as c_int), a.as_ptr(), &(lda as c_int), b.as_ptr(), &(ldb as c_int),
                     x.as_ptr(), &(ldx as c_int), ferr.as_mut_ptr(), berr.as_mut_ptr(),
                     work.as_mut_ptr(), iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn ctrrfs(uplo: u8, trans: u8, diag: u8, n: usize, nrhs: usize, a: &[c32], lda: usize,
              b: &[c32], ldb: usize, x: &[c32], ldx: usize, ferr: &mut [f32], berr: &mut [f32],
              work: &mut [c32], rwork: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::ctrrfs_(&(uplo as c_char), &(trans as c_char), &(diag as c_char), &(n as c_int),
                     &(nrhs as c_int), a.as_ptr() as *const _, &(lda as c_int),
                     b.as_ptr() as *const _, &(ldb as c_int), x.as_ptr() as *const _,
                     &(ldx as c_int), ferr.as_mut_ptr(), berr.as_mut_ptr(),
                     work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn ztrrfs(uplo: u8, trans: u8, diag: u8, n: usize, nrhs: usize, a: &[c64], lda: usize,
              b: &[c64], ldb: usize, x: &[c64], ldx: usize, ferr: &mut [f64], berr: &mut [f64],
              work: &mut [c64], rwork: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::ztrrfs_(&(uplo as c_char), &(trans as c_char), &(diag as c_char), &(n as c_int),
                     &(nrhs as c_int), a.as_ptr() as *const _, &(lda as c_int),
                     b.as_ptr() as *const _, &(ldb as c_int), x.as_ptr() as *const _,
                     &(ldx as c_int), ferr.as_mut_ptr(), berr.as_mut_ptr(),
                     work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn stprfs(uplo: u8, trans: u8, diag: u8, n: usize, nrhs: usize, ap: &[f32], b: &[f32],
              ldb: usize, x: &[f32], ldx: usize, ferr: &mut [f32], berr: &mut [f32],
              work: &mut [f32], iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::stprfs_(&(uplo as c_char), &(trans as c_char), &(diag as c_char), &(n as c_int),
                     &(nrhs as c_int), ap.as_ptr(), b.as_ptr(), &(ldb as c_int), x.as_ptr(),
                     &(ldx as c_int), ferr.as_mut_ptr(), berr.as_mut_ptr(), work.as_mut_ptr(),
                     iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dtprfs(uplo: u8, trans: u8, diag: u8, n: usize, nrhs: usize, ap: &[f64], b: &[f64],
              ldb: usize, x: &[f64], ldx: usize, ferr: &mut [f64], berr: &mut [f64],
              work: &mut [f64], iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::dtprfs_(&(uplo as c_char), &(trans as c_char), &(diag as c_char), &(n as c_int),
                     &(nrhs as c_int), ap.as_ptr(), b.as_ptr(), &(ldb as c_int), x.as_ptr(),
                     &(ldx as c_int), ferr.as_mut_ptr(), berr.as_mut_ptr(), work.as_mut_ptr(),
                     iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn ctprfs(uplo: u8, trans: u8, diag: u8, n: usize, nrhs: usize, ap: &[c32], b: &[c32],
              ldb: usize, x: &[c32], ldx: usize, ferr: &mut [f32], berr: &mut [f32],
              work: &mut [c32], rwork: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::ctprfs_(&(uplo as c_char), &(trans as c_char), &(diag as c_char), &(n as c_int),
                     &(nrhs as c_int), ap.as_ptr() as *const _, b.as_ptr() as *const _,
                     &(ldb as c_int), x.as_ptr() as *const _, &(ldx as c_int), ferr.as_mut_ptr(),
                     berr.as_mut_ptr(), work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn ztprfs(uplo: u8, trans: u8, diag: u8, n: usize, nrhs: usize, ap: &[c64], b: &[c64],
              ldb: usize, x: &[c64], ldx: usize, ferr: &mut [f64], berr: &mut [f64],
              work: &mut [c64], rwork: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::ztprfs_(&(uplo as c_char), &(trans as c_char), &(diag as c_char), &(n as c_int),
                     &(nrhs as c_int), ap.as_ptr() as *const _, b.as_ptr() as *const _,
                     &(ldb as c_int), x.as_ptr() as *const _, &(ldx as c_int), ferr.as_mut_ptr(),
                     berr.as_mut_ptr(), work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn stbrfs(uplo: u8, trans: u8, diag: u8, n: usize, kd: usize, nrhs: usize, ab: &[f32],
              ldab: usize, b: &[f32], ldb: usize, x: &[f32], ldx: usize, ferr: &mut [f32],
              berr: &mut [f32], work: &mut [f32], iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::stbrfs_(&(uplo as c_char), &(trans as c_char), &(diag as c_char), &(n as c_int),
                     &(kd as c_int), &(nrhs as c_int), ab.as_ptr(), &(ldab as c_int), b.as_ptr(),
                     &(ldb as c_int), x.as_ptr(), &(ldx as c_int), ferr.as_mut_ptr(),
                     berr.as_mut_ptr(), work.as_mut_ptr(), iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dtbrfs(uplo: u8, trans: u8, diag: u8, n: usize, kd: usize, nrhs: usize, ab: &[f64],
              ldab: usize, b: &[f64], ldb: usize, x: &[f64], ldx: usize, ferr: &mut [f64],
              berr: &mut [f64], work: &mut [f64], iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::dtbrfs_(&(uplo as c_char), &(trans as c_char), &(diag as c_char), &(n as c_int),
                     &(kd as c_int), &(nrhs as c_int), ab.as_ptr(), &(ldab as c_int), b.as_ptr(),
                     &(ldb as c_int), x.as_ptr(), &(ldx as c_int), ferr.as_mut_ptr(),
                     berr.as_mut_ptr(), work.as_mut_ptr(), iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn ctbrfs(uplo: u8, trans: u8, diag: u8, n: usize, kd: usize, nrhs: usize, ab: &[c32],
              ldab: usize, b: &[c32], ldb: usize, x: &[c32], ldx: usize, ferr: &mut [f32],
              berr: &mut [f32], work: &mut [c32], rwork: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::ctbrfs_(&(uplo as c_char), &(trans as c_char), &(diag as c_char), &(n as c_int),
                     &(kd as c_int), &(nrhs as c_int), ab.as_ptr() as *const _, &(ldab as c_int),
                     b.as_ptr() as *const _, &(ldb as c_int), x.as_ptr() as *const _,
                     &(ldx as c_int), ferr.as_mut_ptr(), berr.as_mut_ptr(),
                     work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn ztbrfs(uplo: u8, trans: u8, diag: u8, n: usize, kd: usize, nrhs: usize, ab: &[c64],
              ldab: usize, b: &[c64], ldb: usize, x: &[c64], ldx: usize, ferr: &mut [f64],
              berr: &mut [f64], work: &mut [c64], rwork: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::ztbrfs_(&(uplo as c_char), &(trans as c_char), &(diag as c_char), &(n as c_int),
                     &(kd as c_int), &(nrhs as c_int), ab.as_ptr() as *const _, &(ldab as c_int),
                     b.as_ptr() as *const _, &(ldb as c_int), x.as_ptr() as *const _,
                     &(ldx as c_int), ferr.as_mut_ptr(), berr.as_mut_ptr(),
                     work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn sgetri(n: usize, a: &mut [f32], lda: usize, ipiv: &[i32], work: &mut [f32], lwork: usize,
              info: &mut i32) {

    unsafe {
        ffi::sgetri_(&(n as c_int), a.as_mut_ptr(), &(lda as c_int), ipiv.as_ptr(),
                     work.as_mut_ptr(), &(lwork as c_int), info)
    }
}

#[inline]
pub fn dgetri(n: usize, a: &mut [f64], lda: usize, ipiv: &[i32], work: &mut [f64], lwork: usize,
              info: &mut i32) {

    unsafe {
        ffi::dgetri_(&(n as c_int), a.as_mut_ptr(), &(lda as c_int), ipiv.as_ptr(),
                     work.as_mut_ptr(), &(lwork as c_int), info)
    }
}

#[inline]
pub fn cgetri(n: usize, a: &mut [c32], lda: usize, ipiv: &[i32], work: &mut [c32], lwork: usize,
              info: &mut i32) {

    unsafe {
        ffi::cgetri_(&(n as c_int), a.as_mut_ptr() as *mut _, &(lda as c_int), ipiv.as_ptr(),
                     work.as_mut_ptr() as *mut _, &(lwork as c_int), info)
    }
}

#[inline]
pub fn zgetri(n: usize, a: &mut [c64], lda: usize, ipiv: &[i32], work: &mut [c64], lwork: usize,
              info: &mut i32) {

    unsafe {
        ffi::zgetri_(&(n as c_int), a.as_mut_ptr() as *mut _, &(lda as c_int), ipiv.as_ptr(),
                     work.as_mut_ptr() as *mut _, &(lwork as c_int), info)
    }
}

#[inline]
pub fn spotri(uplo: u8, n: usize, a: &mut [f32], lda: usize, info: &mut i32) {
    unsafe {
        ffi::spotri_(&(uplo as c_char), &(n as c_int), a.as_mut_ptr(), &(lda as c_int), info)
    }
}

#[inline]
pub fn dpotri(uplo: u8, n: usize, a: &mut [f64], lda: usize, info: &mut i32) {
    unsafe {
        ffi::dpotri_(&(uplo as c_char), &(n as c_int), a.as_mut_ptr(), &(lda as c_int), info)
    }
}

#[inline]
pub fn cpotri(uplo: u8, n: usize, a: &mut [c32], lda: usize, info: &mut i32) {
    unsafe {
        ffi::cpotri_(&(uplo as c_char), &(n as c_int), a.as_mut_ptr() as *mut _, &(lda as c_int),
                     info)
    }
}

#[inline]
pub fn zpotri(uplo: u8, n: usize, a: &mut [c64], lda: usize, info: &mut i32) {
    unsafe {
        ffi::zpotri_(&(uplo as c_char), &(n as c_int), a.as_mut_ptr() as *mut _, &(lda as c_int),
                     info)
    }
}

#[inline]
pub fn spftri(transr: u8, uplo: u8, n: usize, a: &mut [f32], info: &mut i32) {
    unsafe {
        ffi::spftri_(&(transr as c_char), &(uplo as c_char), &(n as c_int), a.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dpftri(transr: u8, uplo: u8, n: usize, a: &mut [f64], info: &mut i32) {
    unsafe {
        ffi::dpftri_(&(transr as c_char), &(uplo as c_char), &(n as c_int), a.as_mut_ptr(), info)
    }
}

#[inline]
pub fn cpftri(transr: u8, uplo: u8, n: usize, a: &mut [c32], info: &mut i32) {
    unsafe {
        ffi::cpftri_(&(transr as c_char), &(uplo as c_char), &(n as c_int),
                     a.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn zpftri(transr: u8, uplo: u8, n: usize, a: &mut [c64], info: &mut i32) {
    unsafe {
        ffi::zpftri_(&(transr as c_char), &(uplo as c_char), &(n as c_int),
                     a.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn spptri(uplo: u8, n: usize, ap: &mut [f32], info: &mut i32) {
    unsafe {
        ffi::spptri_(&(uplo as c_char), &(n as c_int), ap.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dpptri(uplo: u8, n: usize, ap: &mut [f64], info: &mut i32) {
    unsafe {
        ffi::dpptri_(&(uplo as c_char), &(n as c_int), ap.as_mut_ptr(), info)
    }
}

#[inline]
pub fn cpptri(uplo: u8, n: usize, ap: &mut [c32], info: &mut i32) {
    unsafe {
        ffi::cpptri_(&(uplo as c_char), &(n as c_int), ap.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn zpptri(uplo: u8, n: usize, ap: &mut [c64], info: &mut i32) {
    unsafe {
        ffi::zpptri_(&(uplo as c_char), &(n as c_int), ap.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn ssytri(uplo: u8, n: usize, a: &mut [f32], lda: usize, ipiv: &[i32], work: &mut [f32],
              info: &mut i32) {

    unsafe {
        ffi::ssytri_(&(uplo as c_char), &(n as c_int), a.as_mut_ptr(), &(lda as c_int),
                     ipiv.as_ptr(), work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dsytri(uplo: u8, n: usize, a: &mut [f64], lda: usize, ipiv: &[i32], work: &mut [f64],
              info: &mut i32) {

    unsafe {
        ffi::dsytri_(&(uplo as c_char), &(n as c_int), a.as_mut_ptr(), &(lda as c_int),
                     ipiv.as_ptr(), work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn csytri(uplo: u8, n: usize, a: &mut [c32], lda: usize, ipiv: &[i32], work: &mut [c32],
              info: &mut i32) {

    unsafe {
        ffi::csytri_(&(uplo as c_char), &(n as c_int), a.as_mut_ptr() as *mut _, &(lda as c_int),
                     ipiv.as_ptr(), work.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn zsytri(uplo: u8, n: usize, a: &mut [c64], lda: usize, ipiv: &[i32], work: &mut [c64],
              info: &mut i32) {

    unsafe {
        ffi::zsytri_(&(uplo as c_char), &(n as c_int), a.as_mut_ptr() as *mut _, &(lda as c_int),
                     ipiv.as_ptr(), work.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn chetri(uplo: u8, n: usize, a: &mut [c32], lda: usize, ipiv: &[i32], work: &mut [c32],
              info: &mut i32) {

    unsafe {
        ffi::chetri_(&(uplo as c_char), &(n as c_int), a.as_mut_ptr() as *mut _, &(lda as c_int),
                     ipiv.as_ptr(), work.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn zhetri(uplo: u8, n: usize, a: &mut [c64], lda: usize, ipiv: &[i32], work: &mut [c64],
              info: &mut i32) {

    unsafe {
        ffi::zhetri_(&(uplo as c_char), &(n as c_int), a.as_mut_ptr() as *mut _, &(lda as c_int),
                     ipiv.as_ptr(), work.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn ssptri(uplo: u8, n: usize, ap: &mut [f32], ipiv: &[i32], work: &mut [f32], info: &mut i32) {
    unsafe {
        ffi::ssptri_(&(uplo as c_char), &(n as c_int), ap.as_mut_ptr(), ipiv.as_ptr(),
                     work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dsptri(uplo: u8, n: usize, ap: &mut [f64], ipiv: &[i32], work: &mut [f64], info: &mut i32) {
    unsafe {
        ffi::dsptri_(&(uplo as c_char), &(n as c_int), ap.as_mut_ptr(), ipiv.as_ptr(),
                     work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn csptri(uplo: u8, n: usize, ap: &mut [c32], ipiv: &[i32], work: &mut [c32], info: &mut i32) {
    unsafe {
        ffi::csptri_(&(uplo as c_char), &(n as c_int), ap.as_mut_ptr() as *mut _, ipiv.as_ptr(),
                     work.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn zsptri(uplo: u8, n: usize, ap: &mut [c64], ipiv: &[i32], work: &mut [c64], info: &mut i32) {
    unsafe {
        ffi::zsptri_(&(uplo as c_char), &(n as c_int), ap.as_mut_ptr() as *mut _, ipiv.as_ptr(),
                     work.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn chptri(uplo: u8, n: usize, ap: &mut [c32], ipiv: &[i32], work: &mut [c32], info: &mut i32) {
    unsafe {
        ffi::chptri_(&(uplo as c_char), &(n as c_int), ap.as_mut_ptr() as *mut _, ipiv.as_ptr(),
                     work.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn zhptri(uplo: u8, n: usize, ap: &mut [c64], ipiv: &[i32], work: &mut [c64], info: &mut i32) {
    unsafe {
        ffi::zhptri_(&(uplo as c_char), &(n as c_int), ap.as_mut_ptr() as *mut _, ipiv.as_ptr(),
                     work.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn strtri(uplo: u8, diag: u8, n: usize, a: &mut [f32], lda: usize, info: &mut i32) {
    unsafe {
        ffi::strtri_(&(uplo as c_char), &(diag as c_char), &(n as c_int), a.as_mut_ptr(),
                     &(lda as c_int), info)
    }
}

#[inline]
pub fn dtrtri(uplo: u8, diag: u8, n: usize, a: &mut [f64], lda: usize, info: &mut i32) {
    unsafe {
        ffi::dtrtri_(&(uplo as c_char), &(diag as c_char), &(n as c_int), a.as_mut_ptr(),
                     &(lda as c_int), info)
    }
}

#[inline]
pub fn ctrtri(uplo: u8, diag: u8, n: usize, a: &mut [c32], lda: usize, info: &mut i32) {
    unsafe {
        ffi::ctrtri_(&(uplo as c_char), &(diag as c_char), &(n as c_int), a.as_mut_ptr() as *mut _,
                     &(lda as c_int), info)
    }
}

#[inline]
pub fn ztrtri(uplo: u8, diag: u8, n: usize, a: &mut [c64], lda: usize, info: &mut i32) {
    unsafe {
        ffi::ztrtri_(&(uplo as c_char), &(diag as c_char), &(n as c_int), a.as_mut_ptr() as *mut _,
                     &(lda as c_int), info)
    }
}

#[inline]
pub fn stftri(transr: u8, uplo: u8, diag: u8, n: usize, a: &mut [f32], info: &mut i32) {
    unsafe {
        ffi::stftri_(&(transr as c_char), &(uplo as c_char), &(diag as c_char), &(n as c_int),
                     a.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dtftri(transr: u8, uplo: u8, diag: u8, n: usize, a: &mut [f64], info: &mut i32) {
    unsafe {
        ffi::dtftri_(&(transr as c_char), &(uplo as c_char), &(diag as c_char), &(n as c_int),
                     a.as_mut_ptr(), info)
    }
}

#[inline]
pub fn ctftri(transr: u8, uplo: u8, diag: u8, n: usize, a: &mut [c32], info: &mut i32) {
    unsafe {
        ffi::ctftri_(&(transr as c_char), &(uplo as c_char), &(diag as c_char), &(n as c_int),
                     a.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn ztftri(transr: u8, uplo: u8, diag: u8, n: usize, a: &mut [c64], info: &mut i32) {
    unsafe {
        ffi::ztftri_(&(transr as c_char), &(uplo as c_char), &(diag as c_char), &(n as c_int),
                     a.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn stptri(uplo: u8, diag: u8, n: usize, ap: &mut [f32], info: &mut i32) {
    unsafe {
        ffi::stptri_(&(uplo as c_char), &(diag as c_char), &(n as c_int), ap.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dtptri(uplo: u8, diag: u8, n: usize, ap: &mut [f64], info: &mut i32) {
    unsafe {
        ffi::dtptri_(&(uplo as c_char), &(diag as c_char), &(n as c_int), ap.as_mut_ptr(), info)
    }
}

#[inline]
pub fn ctptri(uplo: u8, diag: u8, n: usize, ap: &mut [c32], info: &mut i32) {
    unsafe {
        ffi::ctptri_(&(uplo as c_char), &(diag as c_char), &(n as c_int),
                     ap.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn ztptri(uplo: u8, diag: u8, n: usize, ap: &mut [c64], info: &mut i32) {
    unsafe {
        ffi::ztptri_(&(uplo as c_char), &(diag as c_char), &(n as c_int),
                     ap.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn sgeequ(m: usize, n: usize, a: &[f32], lda: usize, r: &mut [f32], c: &mut [f32],
              rowcnd: &mut [f32], colcnd: &mut [f32], amax: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::sgeequ_(&(m as c_int), &(n as c_int), a.as_ptr(), &(lda as c_int), r.as_mut_ptr(),
                     c.as_mut_ptr(), rowcnd.as_mut_ptr(), colcnd.as_mut_ptr(), amax.as_mut_ptr(),
                     info)
    }
}

#[inline]
pub fn dgeequ(m: usize, n: usize, a: &[f64], lda: usize, r: &mut [f64], c: &mut [f64],
              rowcnd: &mut [f64], colcnd: &mut [f64], amax: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::dgeequ_(&(m as c_int), &(n as c_int), a.as_ptr(), &(lda as c_int), r.as_mut_ptr(),
                     c.as_mut_ptr(), rowcnd.as_mut_ptr(), colcnd.as_mut_ptr(), amax.as_mut_ptr(),
                     info)
    }
}

#[inline]
pub fn cgeequ(m: usize, n: usize, a: &[c32], lda: usize, r: &mut [f32], c: &mut [f32],
              rowcnd: &mut [f32], colcnd: &mut [f32], amax: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::cgeequ_(&(m as c_int), &(n as c_int), a.as_ptr() as *const _, &(lda as c_int),
                     r.as_mut_ptr(), c.as_mut_ptr(), rowcnd.as_mut_ptr(), colcnd.as_mut_ptr(),
                     amax.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zgeequ(m: usize, n: usize, a: &[c64], lda: usize, r: &mut [f64], c: &mut [f64],
              rowcnd: &mut [f64], colcnd: &mut [f64], amax: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::zgeequ_(&(m as c_int), &(n as c_int), a.as_ptr() as *const _, &(lda as c_int),
                     r.as_mut_ptr(), c.as_mut_ptr(), rowcnd.as_mut_ptr(), colcnd.as_mut_ptr(),
                     amax.as_mut_ptr(), info)
    }
}

#[inline]
pub fn sgeequb(m: usize, n: usize, a: &[f32], lda: usize, r: &mut [f32], c: &mut [f32],
               rowcnd: &mut [f32], colcnd: &mut [f32], amax: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::sgeequb_(&(m as c_int), &(n as c_int), a.as_ptr(), &(lda as c_int), r.as_mut_ptr(),
                      c.as_mut_ptr(), rowcnd.as_mut_ptr(), colcnd.as_mut_ptr(), amax.as_mut_ptr(),
                      info)
    }
}

#[inline]
pub fn dgeequb(m: usize, n: usize, a: &[f64], lda: usize, r: &mut [f64], c: &mut [f64],
               rowcnd: &mut [f64], colcnd: &mut [f64], amax: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::dgeequb_(&(m as c_int), &(n as c_int), a.as_ptr(), &(lda as c_int), r.as_mut_ptr(),
                      c.as_mut_ptr(), rowcnd.as_mut_ptr(), colcnd.as_mut_ptr(), amax.as_mut_ptr(),
                      info)
    }
}

#[inline]
pub fn cgeequb(m: usize, n: usize, a: &[c32], lda: usize, r: &mut [f32], c: &mut [f32],
               rowcnd: &mut [f32], colcnd: &mut [f32], amax: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::cgeequb_(&(m as c_int), &(n as c_int), a.as_ptr() as *const _, &(lda as c_int),
                      r.as_mut_ptr(), c.as_mut_ptr(), rowcnd.as_mut_ptr(), colcnd.as_mut_ptr(),
                      amax.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zgeequb(m: usize, n: usize, a: &[c64], lda: usize, r: &mut [f64], c: &mut [f64],
               rowcnd: &mut [f64], colcnd: &mut [f64], amax: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::zgeequb_(&(m as c_int), &(n as c_int), a.as_ptr() as *const _, &(lda as c_int),
                      r.as_mut_ptr(), c.as_mut_ptr(), rowcnd.as_mut_ptr(), colcnd.as_mut_ptr(),
                      amax.as_mut_ptr(), info)
    }
}

#[inline]
pub fn sgbequ(m: usize, n: usize, kl: usize, ku: usize, ab: &[f32], ldab: usize, r: &mut [f32],
              c: &mut [f32], rowcnd: &mut [f32], colcnd: &mut [f32], amax: &mut [f32],
              info: &mut i32) {

    unsafe {
        ffi::sgbequ_(&(m as c_int), &(n as c_int), &(kl as c_int), &(ku as c_int), ab.as_ptr(),
                     &(ldab as c_int), r.as_mut_ptr(), c.as_mut_ptr(), rowcnd.as_mut_ptr(),
                     colcnd.as_mut_ptr(), amax.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dgbequ(m: usize, n: usize, kl: usize, ku: usize, ab: &[f64], ldab: usize, r: &mut [f64],
              c: &mut [f64], rowcnd: &mut [f64], colcnd: &mut [f64], amax: &mut [f64],
              info: &mut i32) {

    unsafe {
        ffi::dgbequ_(&(m as c_int), &(n as c_int), &(kl as c_int), &(ku as c_int), ab.as_ptr(),
                     &(ldab as c_int), r.as_mut_ptr(), c.as_mut_ptr(), rowcnd.as_mut_ptr(),
                     colcnd.as_mut_ptr(), amax.as_mut_ptr(), info)
    }
}

#[inline]
pub fn cgbequ(m: usize, n: usize, kl: usize, ku: usize, ab: &[c32], ldab: usize, r: &mut [f32],
              c: &mut [f32], rowcnd: &mut [f32], colcnd: &mut [f32], amax: &mut [f32],
              info: &mut i32) {

    unsafe {
        ffi::cgbequ_(&(m as c_int), &(n as c_int), &(kl as c_int), &(ku as c_int),
                     ab.as_ptr() as *const _, &(ldab as c_int), r.as_mut_ptr(), c.as_mut_ptr(),
                     rowcnd.as_mut_ptr(), colcnd.as_mut_ptr(), amax.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zgbequ(m: usize, n: usize, kl: usize, ku: usize, ab: &[c64], ldab: usize, r: &mut [f64],
              c: &mut [f64], rowcnd: &mut [f64], colcnd: &mut [f64], amax: &mut [f64],
              info: &mut i32) {

    unsafe {
        ffi::zgbequ_(&(m as c_int), &(n as c_int), &(kl as c_int), &(ku as c_int),
                     ab.as_ptr() as *const _, &(ldab as c_int), r.as_mut_ptr(), c.as_mut_ptr(),
                     rowcnd.as_mut_ptr(), colcnd.as_mut_ptr(), amax.as_mut_ptr(), info)
    }
}

#[inline]
pub fn sgbequb(m: usize, n: usize, kl: usize, ku: usize, ab: &[f32], ldab: usize, r: &mut [f32],
               c: &mut [f32], rowcnd: &mut [f32], colcnd: &mut [f32], amax: &mut [f32],
               info: &mut i32) {

    unsafe {
        ffi::sgbequb_(&(m as c_int), &(n as c_int), &(kl as c_int), &(ku as c_int), ab.as_ptr(),
                      &(ldab as c_int), r.as_mut_ptr(), c.as_mut_ptr(), rowcnd.as_mut_ptr(),
                      colcnd.as_mut_ptr(), amax.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dgbequb(m: usize, n: usize, kl: usize, ku: usize, ab: &[f64], ldab: usize, r: &mut [f64],
               c: &mut [f64], rowcnd: &mut [f64], colcnd: &mut [f64], amax: &mut [f64],
               info: &mut i32) {

    unsafe {
        ffi::dgbequb_(&(m as c_int), &(n as c_int), &(kl as c_int), &(ku as c_int), ab.as_ptr(),
                      &(ldab as c_int), r.as_mut_ptr(), c.as_mut_ptr(), rowcnd.as_mut_ptr(),
                      colcnd.as_mut_ptr(), amax.as_mut_ptr(), info)
    }
}

#[inline]
pub fn cgbequb(m: usize, n: usize, kl: usize, ku: usize, ab: &[c32], ldab: usize, r: &mut [f32],
               c: &mut [f32], rowcnd: &mut [f32], colcnd: &mut [f32], amax: &mut [f32],
               info: &mut i32) {

    unsafe {
        ffi::cgbequb_(&(m as c_int), &(n as c_int), &(kl as c_int), &(ku as c_int),
                      ab.as_ptr() as *const _, &(ldab as c_int), r.as_mut_ptr(), c.as_mut_ptr(),
                      rowcnd.as_mut_ptr(), colcnd.as_mut_ptr(), amax.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zgbequb(m: usize, n: usize, kl: usize, ku: usize, ab: &[c64], ldab: usize, r: &mut [f64],
               c: &mut [f64], rowcnd: &mut [f64], colcnd: &mut [f64], amax: &mut [f64],
               info: &mut i32) {

    unsafe {
        ffi::zgbequb_(&(m as c_int), &(n as c_int), &(kl as c_int), &(ku as c_int),
                      ab.as_ptr() as *const _, &(ldab as c_int), r.as_mut_ptr(), c.as_mut_ptr(),
                      rowcnd.as_mut_ptr(), colcnd.as_mut_ptr(), amax.as_mut_ptr(), info)
    }
}

#[inline]
pub fn spoequ(n: usize, a: &[f32], lda: usize, s: &mut [f32], scond: &mut [f32], amax: &mut [f32],
              info: &mut i32) {

    unsafe {
        ffi::spoequ_(&(n as c_int), a.as_ptr(), &(lda as c_int), s.as_mut_ptr(),
                     scond.as_mut_ptr(), amax.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dpoequ(n: usize, a: &[f64], lda: usize, s: &mut [f64], scond: &mut [f64], amax: &mut [f64],
              info: &mut i32) {

    unsafe {
        ffi::dpoequ_(&(n as c_int), a.as_ptr(), &(lda as c_int), s.as_mut_ptr(),
                     scond.as_mut_ptr(), amax.as_mut_ptr(), info)
    }
}

#[inline]
pub fn cpoequ(n: usize, a: &[c32], lda: usize, s: &mut [f32], scond: &mut [f32], amax: &mut [f32],
              info: &mut i32) {

    unsafe {
        ffi::cpoequ_(&(n as c_int), a.as_ptr() as *const _, &(lda as c_int), s.as_mut_ptr(),
                     scond.as_mut_ptr(), amax.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zpoequ(n: usize, a: &[c64], lda: usize, s: &mut [f64], scond: &mut [f64], amax: &mut [f64],
              info: &mut i32) {

    unsafe {
        ffi::zpoequ_(&(n as c_int), a.as_ptr() as *const _, &(lda as c_int), s.as_mut_ptr(),
                     scond.as_mut_ptr(), amax.as_mut_ptr(), info)
    }
}

#[inline]
pub fn spoequb(n: usize, a: &[f32], lda: usize, s: &mut [f32], scond: &mut [f32], amax: &mut [f32],
               info: &mut i32) {

    unsafe {
        ffi::spoequb_(&(n as c_int), a.as_ptr(), &(lda as c_int), s.as_mut_ptr(),
                      scond.as_mut_ptr(), amax.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dpoequb(n: usize, a: &[f64], lda: usize, s: &mut [f64], scond: &mut [f64], amax: &mut [f64],
               info: &mut i32) {

    unsafe {
        ffi::dpoequb_(&(n as c_int), a.as_ptr(), &(lda as c_int), s.as_mut_ptr(),
                      scond.as_mut_ptr(), amax.as_mut_ptr(), info)
    }
}

#[inline]
pub fn cpoequb(n: usize, a: &[c32], lda: usize, s: &mut [f32], scond: &mut [f32], amax: &mut [f32],
               info: &mut i32) {

    unsafe {
        ffi::cpoequb_(&(n as c_int), a.as_ptr() as *const _, &(lda as c_int), s.as_mut_ptr(),
                      scond.as_mut_ptr(), amax.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zpoequb(n: usize, a: &[c64], lda: usize, s: &mut [f64], scond: &mut [f64], amax: &mut [f64],
               info: &mut i32) {

    unsafe {
        ffi::zpoequb_(&(n as c_int), a.as_ptr() as *const _, &(lda as c_int), s.as_mut_ptr(),
                      scond.as_mut_ptr(), amax.as_mut_ptr(), info)
    }
}

#[inline]
pub fn sppequ(uplo: u8, n: usize, ap: &[f32], s: &mut [f32], scond: &mut [f32], amax: &mut [f32],
              info: &mut i32) {

    unsafe {
        ffi::sppequ_(&(uplo as c_char), &(n as c_int), ap.as_ptr(), s.as_mut_ptr(),
                     scond.as_mut_ptr(), amax.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dppequ(uplo: u8, n: usize, ap: &[f64], s: &mut [f64], scond: &mut [f64], amax: &mut [f64],
              info: &mut i32) {

    unsafe {
        ffi::dppequ_(&(uplo as c_char), &(n as c_int), ap.as_ptr(), s.as_mut_ptr(),
                     scond.as_mut_ptr(), amax.as_mut_ptr(), info)
    }
}

#[inline]
pub fn cppequ(uplo: u8, n: usize, ap: &[c32], s: &mut [f32], scond: &mut [f32], amax: &mut [f32],
              info: &mut i32) {

    unsafe {
        ffi::cppequ_(&(uplo as c_char), &(n as c_int), ap.as_ptr() as *const _, s.as_mut_ptr(),
                     scond.as_mut_ptr(), amax.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zppequ(uplo: u8, n: usize, ap: &[c64], s: &mut [f64], scond: &mut [f64], amax: &mut [f64],
              info: &mut i32) {

    unsafe {
        ffi::zppequ_(&(uplo as c_char), &(n as c_int), ap.as_ptr() as *const _, s.as_mut_ptr(),
                     scond.as_mut_ptr(), amax.as_mut_ptr(), info)
    }
}

#[inline]
pub fn spbequ(uplo: u8, n: usize, kd: usize, ab: &[f32], ldab: usize, s: &mut [f32],
              scond: &mut [f32], amax: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::spbequ_(&(uplo as c_char), &(n as c_int), &(kd as c_int), ab.as_ptr(),
                     &(ldab as c_int), s.as_mut_ptr(), scond.as_mut_ptr(), amax.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dpbequ(uplo: u8, n: usize, kd: usize, ab: &[f64], ldab: usize, s: &mut [f64],
              scond: &mut [f64], amax: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::dpbequ_(&(uplo as c_char), &(n as c_int), &(kd as c_int), ab.as_ptr(),
                     &(ldab as c_int), s.as_mut_ptr(), scond.as_mut_ptr(), amax.as_mut_ptr(), info)
    }
}

#[inline]
pub fn cpbequ(uplo: u8, n: usize, kd: usize, ab: &[c32], ldab: usize, s: &mut [f32],
              scond: &mut [f32], amax: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::cpbequ_(&(uplo as c_char), &(n as c_int), &(kd as c_int), ab.as_ptr() as *const _,
                     &(ldab as c_int), s.as_mut_ptr(), scond.as_mut_ptr(), amax.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zpbequ(uplo: u8, n: usize, kd: usize, ab: &[c64], ldab: usize, s: &mut [f64],
              scond: &mut [f64], amax: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::zpbequ_(&(uplo as c_char), &(n as c_int), &(kd as c_int), ab.as_ptr() as *const _,
                     &(ldab as c_int), s.as_mut_ptr(), scond.as_mut_ptr(), amax.as_mut_ptr(), info)
    }
}

#[inline]
pub fn ssyequb(uplo: u8, n: usize, a: &[f32], lda: usize, s: &mut [f32], scond: &mut [f32],
               amax: &mut [f32], work: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::ssyequb_(&(uplo as c_char), &(n as c_int), a.as_ptr(), &(lda as c_int),
                      s.as_mut_ptr(), scond.as_mut_ptr(), amax.as_mut_ptr(), work.as_mut_ptr(),
                      info)
    }
}

#[inline]
pub fn dsyequb(uplo: u8, n: usize, a: &[f64], lda: usize, s: &mut [f64], scond: &mut [f64],
               amax: &mut [f64], work: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::dsyequb_(&(uplo as c_char), &(n as c_int), a.as_ptr(), &(lda as c_int),
                      s.as_mut_ptr(), scond.as_mut_ptr(), amax.as_mut_ptr(), work.as_mut_ptr(),
                      info)
    }
}

#[inline]
pub fn csyequb(uplo: u8, n: usize, a: &[c32], lda: usize, s: &mut [f32], scond: &mut [f32],
               amax: &mut [f32], work: &mut [c32], info: &mut i32) {

    unsafe {
        ffi::csyequb_(&(uplo as c_char), &(n as c_int), a.as_ptr() as *const _, &(lda as c_int),
                      s.as_mut_ptr(), scond.as_mut_ptr(), amax.as_mut_ptr(),
                      work.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn zsyequb(uplo: u8, n: usize, a: &[c64], lda: usize, s: &mut [f64], scond: &mut [f64],
               amax: &mut [f64], work: &mut [c64], info: &mut i32) {

    unsafe {
        ffi::zsyequb_(&(uplo as c_char), &(n as c_int), a.as_ptr() as *const _, &(lda as c_int),
                      s.as_mut_ptr(), scond.as_mut_ptr(), amax.as_mut_ptr(),
                      work.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn cheequb(uplo: u8, n: usize, a: &[c32], lda: usize, s: &mut [f32], scond: &mut [f32],
               amax: &mut [f32], work: &mut [c32], info: &mut i32) {

    unsafe {
        ffi::cheequb_(&(uplo as c_char), &(n as c_int), a.as_ptr() as *const _, &(lda as c_int),
                      s.as_mut_ptr(), scond.as_mut_ptr(), amax.as_mut_ptr(),
                      work.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn zheequb(uplo: u8, n: usize, a: &[c64], lda: usize, s: &mut [f64], scond: &mut [f64],
               amax: &mut [f64], work: &mut [c64], info: &mut i32) {

    unsafe {
        ffi::zheequb_(&(uplo as c_char), &(n as c_int), a.as_ptr() as *const _, &(lda as c_int),
                      s.as_mut_ptr(), scond.as_mut_ptr(), amax.as_mut_ptr(),
                      work.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn sgesv(n: usize, nrhs: usize, a: &mut [f32], lda: usize, ipiv: &mut [i32], b: &mut [f32],
             ldb: usize, info: &mut i32) {

    unsafe {
        ffi::sgesv_(&(n as c_int), &(nrhs as c_int), a.as_mut_ptr(), &(lda as c_int),
                    ipiv.as_mut_ptr(), b.as_mut_ptr(), &(ldb as c_int), info)
    }
}

#[inline]
pub fn dgesv(n: usize, nrhs: usize, a: &mut [f64], lda: usize, ipiv: &mut [i32], b: &mut [f64],
             ldb: usize, info: &mut i32) {

    unsafe {
        ffi::dgesv_(&(n as c_int), &(nrhs as c_int), a.as_mut_ptr(), &(lda as c_int),
                    ipiv.as_mut_ptr(), b.as_mut_ptr(), &(ldb as c_int), info)
    }
}

#[inline]
pub fn cgesv(n: usize, nrhs: usize, a: &mut [c32], lda: usize, ipiv: &mut [i32], b: &mut [c32],
             ldb: usize, info: &mut i32) {

    unsafe {
        ffi::cgesv_(&(n as c_int), &(nrhs as c_int), a.as_mut_ptr() as *mut _, &(lda as c_int),
                    ipiv.as_mut_ptr(), b.as_mut_ptr() as *mut _, &(ldb as c_int), info)
    }
}

#[inline]
pub fn zgesv(n: usize, nrhs: usize, a: &mut [c64], lda: usize, ipiv: &mut [i32], b: &mut [c64],
             ldb: usize, info: &mut i32) {

    unsafe {
        ffi::zgesv_(&(n as c_int), &(nrhs as c_int), a.as_mut_ptr() as *mut _, &(lda as c_int),
                    ipiv.as_mut_ptr(), b.as_mut_ptr() as *mut _, &(ldb as c_int), info)
    }
}

#[inline]
pub fn dsgesv(n: usize, nrhs: usize, a: &mut [f64], lda: usize, ipiv: &mut [i32], b: &[f64],
              ldb: usize, x: &mut [f64], ldx: usize, work: &mut [f64], swork: &mut [f32],
              iter: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::dsgesv_(&(n as c_int), &(nrhs as c_int), a.as_mut_ptr(), &(lda as c_int),
                     ipiv.as_mut_ptr(), b.as_ptr(), &(ldb as c_int), x.as_mut_ptr(),
                     &(ldx as c_int), work.as_mut_ptr(), swork.as_mut_ptr(), iter.as_mut_ptr(),
                     info)
    }
}

#[inline]
pub fn zcgesv(n: usize, nrhs: usize, a: &mut [c64], lda: usize, ipiv: &mut [i32], b: &[c64],
              ldb: usize, x: &mut [c64], ldx: usize, work: &mut [c64], swork: &mut [c32],
              rwork: &mut [f64], iter: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::zcgesv_(&(n as c_int), &(nrhs as c_int), a.as_mut_ptr() as *mut _, &(lda as c_int),
                     ipiv.as_mut_ptr(), b.as_ptr() as *const _, &(ldb as c_int),
                     x.as_mut_ptr() as *mut _, &(ldx as c_int), work.as_mut_ptr() as *mut _,
                     swork.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), iter.as_mut_ptr(), info)
    }
}

#[inline]
pub fn sgesvx(fact: u8, trans: u8, n: usize, nrhs: usize, a: &mut [f32], lda: usize,
              af: &mut [f32], ldaf: usize, ipiv: &mut [i32], equed: &mut u8, r: &mut [f32],
              c: &mut [f32], b: &mut [f32], ldb: usize, x: &mut [f32], ldx: usize,
              rcond: &mut [f32], ferr: &mut [f32], berr: &mut [f32], work: &mut [f32],
              iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::sgesvx_(&(fact as c_char), &(trans as c_char), &(n as c_int), &(nrhs as c_int),
                     a.as_mut_ptr(), &(lda as c_int), af.as_mut_ptr(), &(ldaf as c_int),
                     ipiv.as_mut_ptr(), equed as *mut _ as *mut _, r.as_mut_ptr(), c.as_mut_ptr(),
                     b.as_mut_ptr(), &(ldb as c_int), x.as_mut_ptr(), &(ldx as c_int),
                     rcond.as_mut_ptr(), ferr.as_mut_ptr(), berr.as_mut_ptr(), work.as_mut_ptr(),
                     iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dgesvx(fact: u8, trans: u8, n: usize, nrhs: usize, a: &mut [f64], lda: usize,
              af: &mut [f64], ldaf: usize, ipiv: &mut [i32], equed: &mut u8, r: &mut [f64],
              c: &mut [f64], b: &mut [f64], ldb: usize, x: &mut [f64], ldx: usize,
              rcond: &mut [f64], ferr: &mut [f64], berr: &mut [f64], work: &mut [f64],
              iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::dgesvx_(&(fact as c_char), &(trans as c_char), &(n as c_int), &(nrhs as c_int),
                     a.as_mut_ptr(), &(lda as c_int), af.as_mut_ptr(), &(ldaf as c_int),
                     ipiv.as_mut_ptr(), equed as *mut _ as *mut _, r.as_mut_ptr(), c.as_mut_ptr(),
                     b.as_mut_ptr(), &(ldb as c_int), x.as_mut_ptr(), &(ldx as c_int),
                     rcond.as_mut_ptr(), ferr.as_mut_ptr(), berr.as_mut_ptr(), work.as_mut_ptr(),
                     iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn cgesvx(fact: u8, trans: u8, n: usize, nrhs: usize, a: &mut [c32], lda: usize,
              af: &mut [c32], ldaf: usize, ipiv: &mut [i32], equed: &mut u8, r: &mut [f32],
              c: &mut [f32], b: &mut [c32], ldb: usize, x: &mut [c32], ldx: usize,
              rcond: &mut [f32], ferr: &mut [f32], berr: &mut [f32], work: &mut [c32],
              rwork: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::cgesvx_(&(fact as c_char), &(trans as c_char), &(n as c_int), &(nrhs as c_int),
                     a.as_mut_ptr() as *mut _, &(lda as c_int), af.as_mut_ptr() as *mut _,
                     &(ldaf as c_int), ipiv.as_mut_ptr(), equed as *mut _ as *mut _,
                     r.as_mut_ptr(), c.as_mut_ptr(), b.as_mut_ptr() as *mut _, &(ldb as c_int),
                     x.as_mut_ptr() as *mut _, &(ldx as c_int), rcond.as_mut_ptr(),
                     ferr.as_mut_ptr(), berr.as_mut_ptr(), work.as_mut_ptr() as *mut _,
                     rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zgesvx(fact: u8, trans: u8, n: usize, nrhs: usize, a: &mut [c64], lda: usize,
              af: &mut [c64], ldaf: usize, ipiv: &mut [i32], equed: &mut u8, r: &mut [f64],
              c: &mut [f64], b: &mut [c64], ldb: usize, x: &mut [c64], ldx: usize,
              rcond: &mut [f64], ferr: &mut [f64], berr: &mut [f64], work: &mut [c64],
              rwork: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::zgesvx_(&(fact as c_char), &(trans as c_char), &(n as c_int), &(nrhs as c_int),
                     a.as_mut_ptr() as *mut _, &(lda as c_int), af.as_mut_ptr() as *mut _,
                     &(ldaf as c_int), ipiv.as_mut_ptr(), equed as *mut _ as *mut _,
                     r.as_mut_ptr(), c.as_mut_ptr(), b.as_mut_ptr() as *mut _, &(ldb as c_int),
                     x.as_mut_ptr() as *mut _, &(ldx as c_int), rcond.as_mut_ptr(),
                     ferr.as_mut_ptr(), berr.as_mut_ptr(), work.as_mut_ptr() as *mut _,
                     rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn sgesvxx(fact: u8, trans: u8, n: usize, nrhs: usize, a: &mut [f32], lda: usize,
               af: &mut [f32], ldaf: usize, ipiv: &mut [i32], equed: &mut u8, r: &mut [f32],
               c: &mut [f32], b: &mut [f32], ldb: usize, x: &mut [f32], ldx: usize,
               rcond: &mut [f32], rpvgrw: &mut [f32], berr: &mut [f32], n_err_bnds: &[i32],
               err_bnds_norm: &mut [f32], err_bnds_comp: &mut [f32], nparams: &[i32],
               params: &mut [f32], work: &mut [f32], iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::sgesvxx_(&(fact as c_char), &(trans as c_char), &(n as c_int), &(nrhs as c_int),
                      a.as_mut_ptr(), &(lda as c_int), af.as_mut_ptr(), &(ldaf as c_int),
                      ipiv.as_mut_ptr(), equed as *mut _ as *mut _, r.as_mut_ptr(), c.as_mut_ptr(),
                      b.as_mut_ptr(), &(ldb as c_int), x.as_mut_ptr(), &(ldx as c_int),
                      rcond.as_mut_ptr(), rpvgrw.as_mut_ptr(), berr.as_mut_ptr(),
                      n_err_bnds.as_ptr(), err_bnds_norm.as_mut_ptr(), err_bnds_comp.as_mut_ptr(),
                      nparams.as_ptr(), params.as_mut_ptr(), work.as_mut_ptr(), iwork.as_mut_ptr(),
                      info)
    }
}

#[inline]
pub fn dgesvxx(fact: u8, trans: u8, n: usize, nrhs: usize, a: &mut [f64], lda: usize,
               af: &mut [f64], ldaf: usize, ipiv: &mut [i32], equed: &mut u8, r: &mut [f64],
               c: &mut [f64], b: &mut [f64], ldb: usize, x: &mut [f64], ldx: usize,
               rcond: &mut [f64], rpvgrw: &mut [f64], berr: &mut [f64], n_err_bnds: &[i32],
               err_bnds_norm: &mut [f64], err_bnds_comp: &mut [f64], nparams: &[i32],
               params: &mut [f64], work: &mut [f64], iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::dgesvxx_(&(fact as c_char), &(trans as c_char), &(n as c_int), &(nrhs as c_int),
                      a.as_mut_ptr(), &(lda as c_int), af.as_mut_ptr(), &(ldaf as c_int),
                      ipiv.as_mut_ptr(), equed as *mut _ as *mut _, r.as_mut_ptr(), c.as_mut_ptr(),
                      b.as_mut_ptr(), &(ldb as c_int), x.as_mut_ptr(), &(ldx as c_int),
                      rcond.as_mut_ptr(), rpvgrw.as_mut_ptr(), berr.as_mut_ptr(),
                      n_err_bnds.as_ptr(), err_bnds_norm.as_mut_ptr(), err_bnds_comp.as_mut_ptr(),
                      nparams.as_ptr(), params.as_mut_ptr(), work.as_mut_ptr(), iwork.as_mut_ptr(),
                      info)
    }
}

#[inline]
pub fn cgesvxx(fact: u8, trans: u8, n: usize, nrhs: usize, a: &mut [c32], lda: usize,
               af: &mut [c32], ldaf: usize, ipiv: &mut [i32], equed: &mut u8, r: &mut [f32],
               c: &mut [f32], b: &mut [c32], ldb: usize, x: &mut [c32], ldx: usize,
               rcond: &mut [f32], rpvgrw: &mut [f32], berr: &mut [f32], n_err_bnds: &[i32],
               err_bnds_norm: &mut [f32], err_bnds_comp: &mut [f32], nparams: &[i32],
               params: &mut [f32], work: &mut [c32], rwork: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::cgesvxx_(&(fact as c_char), &(trans as c_char), &(n as c_int), &(nrhs as c_int),
                      a.as_mut_ptr() as *mut _, &(lda as c_int), af.as_mut_ptr() as *mut _,
                      &(ldaf as c_int), ipiv.as_mut_ptr(), equed as *mut _ as *mut _,
                      r.as_mut_ptr(), c.as_mut_ptr(), b.as_mut_ptr() as *mut _, &(ldb as c_int),
                      x.as_mut_ptr() as *mut _, &(ldx as c_int), rcond.as_mut_ptr(),
                      rpvgrw.as_mut_ptr(), berr.as_mut_ptr(), n_err_bnds.as_ptr(),
                      err_bnds_norm.as_mut_ptr(), err_bnds_comp.as_mut_ptr(), nparams.as_ptr(),
                      params.as_mut_ptr(), work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zgesvxx(fact: u8, trans: u8, n: usize, nrhs: usize, a: &mut [c64], lda: usize,
               af: &mut [c64], ldaf: usize, ipiv: &mut [i32], equed: &mut u8, r: &mut [f64],
               c: &mut [f64], b: &mut [c64], ldb: usize, x: &mut [c64], ldx: usize,
               rcond: &mut [f64], rpvgrw: &mut [f64], berr: &mut [f64], n_err_bnds: &[i32],
               err_bnds_norm: &mut [f64], err_bnds_comp: &mut [f64], nparams: &[i32],
               params: &mut [f64], work: &mut [c64], rwork: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::zgesvxx_(&(fact as c_char), &(trans as c_char), &(n as c_int), &(nrhs as c_int),
                      a.as_mut_ptr() as *mut _, &(lda as c_int), af.as_mut_ptr() as *mut _,
                      &(ldaf as c_int), ipiv.as_mut_ptr(), equed as *mut _ as *mut _,
                      r.as_mut_ptr(), c.as_mut_ptr(), b.as_mut_ptr() as *mut _, &(ldb as c_int),
                      x.as_mut_ptr() as *mut _, &(ldx as c_int), rcond.as_mut_ptr(),
                      rpvgrw.as_mut_ptr(), berr.as_mut_ptr(), n_err_bnds.as_ptr(),
                      err_bnds_norm.as_mut_ptr(), err_bnds_comp.as_mut_ptr(), nparams.as_ptr(),
                      params.as_mut_ptr(), work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn sgbsv(n: usize, kl: usize, ku: usize, nrhs: usize, ab: &mut [f32], ldab: usize,
             ipiv: &mut [i32], b: &mut [f32], ldb: usize, info: &mut i32) {

    unsafe {
        ffi::sgbsv_(&(n as c_int), &(kl as c_int), &(ku as c_int), &(nrhs as c_int),
                    ab.as_mut_ptr(), &(ldab as c_int), ipiv.as_mut_ptr(), b.as_mut_ptr(),
                    &(ldb as c_int), info)
    }
}

#[inline]
pub fn dgbsv(n: usize, kl: usize, ku: usize, nrhs: usize, ab: &mut [f64], ldab: usize,
             ipiv: &mut [i32], b: &mut [f64], ldb: usize, info: &mut i32) {

    unsafe {
        ffi::dgbsv_(&(n as c_int), &(kl as c_int), &(ku as c_int), &(nrhs as c_int),
                    ab.as_mut_ptr(), &(ldab as c_int), ipiv.as_mut_ptr(), b.as_mut_ptr(),
                    &(ldb as c_int), info)
    }
}

#[inline]
pub fn cgbsv(n: usize, kl: usize, ku: usize, nrhs: usize, ab: &mut [c32], ldab: usize,
             ipiv: &mut [i32], b: &mut [c32], ldb: usize, info: &mut i32) {

    unsafe {
        ffi::cgbsv_(&(n as c_int), &(kl as c_int), &(ku as c_int), &(nrhs as c_int),
                    ab.as_mut_ptr() as *mut _, &(ldab as c_int), ipiv.as_mut_ptr(),
                    b.as_mut_ptr() as *mut _, &(ldb as c_int), info)
    }
}

#[inline]
pub fn zgbsv(n: usize, kl: usize, ku: usize, nrhs: usize, ab: &mut [c64], ldab: usize,
             ipiv: &mut [i32], b: &mut [c64], ldb: usize, info: &mut i32) {

    unsafe {
        ffi::zgbsv_(&(n as c_int), &(kl as c_int), &(ku as c_int), &(nrhs as c_int),
                    ab.as_mut_ptr() as *mut _, &(ldab as c_int), ipiv.as_mut_ptr(),
                    b.as_mut_ptr() as *mut _, &(ldb as c_int), info)
    }
}

#[inline]
pub fn sgbsvx(fact: u8, trans: u8, n: usize, kl: usize, ku: usize, nrhs: usize, ab: &mut [f32],
              ldab: usize, afb: &mut [f32], ldafb: usize, ipiv: &mut [i32], equed: &mut u8,
              r: &mut [f32], c: &mut [f32], b: &mut [f32], ldb: usize, x: &mut [f32], ldx: usize,
              rcond: &mut [f32], ferr: &mut [f32], berr: &mut [f32], work: &mut [f32],
              iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::sgbsvx_(&(fact as c_char), &(trans as c_char), &(n as c_int), &(kl as c_int),
                     &(ku as c_int), &(nrhs as c_int), ab.as_mut_ptr(), &(ldab as c_int),
                     afb.as_mut_ptr(), &(ldafb as c_int), ipiv.as_mut_ptr(),
                     equed as *mut _ as *mut _, r.as_mut_ptr(), c.as_mut_ptr(), b.as_mut_ptr(),
                     &(ldb as c_int), x.as_mut_ptr(), &(ldx as c_int), rcond.as_mut_ptr(),
                     ferr.as_mut_ptr(), berr.as_mut_ptr(), work.as_mut_ptr(), iwork.as_mut_ptr(),
                     info)
    }
}

#[inline]
pub fn dgbsvx(fact: u8, trans: u8, n: usize, kl: usize, ku: usize, nrhs: usize, ab: &mut [f64],
              ldab: usize, afb: &mut [f64], ldafb: usize, ipiv: &mut [i32], equed: &mut u8,
              r: &mut [f64], c: &mut [f64], b: &mut [f64], ldb: usize, x: &mut [f64], ldx: usize,
              rcond: &mut [f64], ferr: &mut [f64], berr: &mut [f64], work: &mut [f64],
              iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::dgbsvx_(&(fact as c_char), &(trans as c_char), &(n as c_int), &(kl as c_int),
                     &(ku as c_int), &(nrhs as c_int), ab.as_mut_ptr(), &(ldab as c_int),
                     afb.as_mut_ptr(), &(ldafb as c_int), ipiv.as_mut_ptr(),
                     equed as *mut _ as *mut _, r.as_mut_ptr(), c.as_mut_ptr(), b.as_mut_ptr(),
                     &(ldb as c_int), x.as_mut_ptr(), &(ldx as c_int), rcond.as_mut_ptr(),
                     ferr.as_mut_ptr(), berr.as_mut_ptr(), work.as_mut_ptr(), iwork.as_mut_ptr(),
                     info)
    }
}

#[inline]
pub fn cgbsvx(fact: u8, trans: u8, n: usize, kl: usize, ku: usize, nrhs: usize, ab: &mut [c32],
              ldab: usize, afb: &mut [c32], ldafb: usize, ipiv: &mut [i32], equed: &mut u8,
              r: &mut [f32], c: &mut [f32], b: &mut [c32], ldb: usize, x: &mut [c32], ldx: usize,
              rcond: &mut [f32], ferr: &mut [f32], berr: &mut [f32], work: &mut [c32],
              rwork: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::cgbsvx_(&(fact as c_char), &(trans as c_char), &(n as c_int), &(kl as c_int),
                     &(ku as c_int), &(nrhs as c_int), ab.as_mut_ptr() as *mut _, &(ldab as c_int),
                     afb.as_mut_ptr() as *mut _, &(ldafb as c_int), ipiv.as_mut_ptr(),
                     equed as *mut _ as *mut _, r.as_mut_ptr(), c.as_mut_ptr(),
                     b.as_mut_ptr() as *mut _, &(ldb as c_int), x.as_mut_ptr() as *mut _,
                     &(ldx as c_int), rcond.as_mut_ptr(), ferr.as_mut_ptr(), berr.as_mut_ptr(),
                     work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zgbsvx(fact: u8, trans: u8, n: usize, kl: usize, ku: usize, nrhs: usize, ab: &mut [c64],
              ldab: usize, afb: &mut [c64], ldafb: usize, ipiv: &mut [i32], equed: &mut u8,
              r: &mut [f64], c: &mut [f64], b: &mut [c64], ldb: usize, x: &mut [c64], ldx: usize,
              rcond: &mut [f64], ferr: &mut [f64], berr: &mut [f64], work: &mut [c64],
              rwork: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::zgbsvx_(&(fact as c_char), &(trans as c_char), &(n as c_int), &(kl as c_int),
                     &(ku as c_int), &(nrhs as c_int), ab.as_mut_ptr() as *mut _, &(ldab as c_int),
                     afb.as_mut_ptr() as *mut _, &(ldafb as c_int), ipiv.as_mut_ptr(),
                     equed as *mut _ as *mut _, r.as_mut_ptr(), c.as_mut_ptr(),
                     b.as_mut_ptr() as *mut _, &(ldb as c_int), x.as_mut_ptr() as *mut _,
                     &(ldx as c_int), rcond.as_mut_ptr(), ferr.as_mut_ptr(), berr.as_mut_ptr(),
                     work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn sgbsvxx(fact: u8, trans: u8, n: usize, kl: usize, ku: usize, nrhs: usize, ab: &mut [f32],
               ldab: usize, afb: &mut [f32], ldafb: usize, ipiv: &mut [i32], equed: &mut u8,
               r: &mut [f32], c: &mut [f32], b: &mut [f32], ldb: usize, x: &mut [f32], ldx: usize,
               rcond: &mut [f32], rpvgrw: &mut [f32], berr: &mut [f32], n_err_bnds: &[i32],
               err_bnds_norm: &mut [f32], err_bnds_comp: &mut [f32], nparams: &[i32],
               params: &mut [f32], work: &mut [f32], iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::sgbsvxx_(&(fact as c_char), &(trans as c_char), &(n as c_int), &(kl as c_int),
                      &(ku as c_int), &(nrhs as c_int), ab.as_mut_ptr(), &(ldab as c_int),
                      afb.as_mut_ptr(), &(ldafb as c_int), ipiv.as_mut_ptr(),
                      equed as *mut _ as *mut _, r.as_mut_ptr(), c.as_mut_ptr(), b.as_mut_ptr(),
                      &(ldb as c_int), x.as_mut_ptr(), &(ldx as c_int), rcond.as_mut_ptr(),
                      rpvgrw.as_mut_ptr(), berr.as_mut_ptr(), n_err_bnds.as_ptr(),
                      err_bnds_norm.as_mut_ptr(), err_bnds_comp.as_mut_ptr(), nparams.as_ptr(),
                      params.as_mut_ptr(), work.as_mut_ptr(), iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dgbsvxx(fact: u8, trans: u8, n: usize, kl: usize, ku: usize, nrhs: usize, ab: &mut [f64],
               ldab: usize, afb: &mut [f64], ldafb: usize, ipiv: &mut [i32], equed: &mut u8,
               r: &mut [f64], c: &mut [f64], b: &mut [f64], ldb: usize, x: &mut [f64], ldx: usize,
               rcond: &mut [f64], rpvgrw: &mut [f64], berr: &mut [f64], n_err_bnds: &[i32],
               err_bnds_norm: &mut [f64], err_bnds_comp: &mut [f64], nparams: &[i32],
               params: &mut [f64], work: &mut [f64], iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::dgbsvxx_(&(fact as c_char), &(trans as c_char), &(n as c_int), &(kl as c_int),
                      &(ku as c_int), &(nrhs as c_int), ab.as_mut_ptr(), &(ldab as c_int),
                      afb.as_mut_ptr(), &(ldafb as c_int), ipiv.as_mut_ptr(),
                      equed as *mut _ as *mut _, r.as_mut_ptr(), c.as_mut_ptr(), b.as_mut_ptr(),
                      &(ldb as c_int), x.as_mut_ptr(), &(ldx as c_int), rcond.as_mut_ptr(),
                      rpvgrw.as_mut_ptr(), berr.as_mut_ptr(), n_err_bnds.as_ptr(),
                      err_bnds_norm.as_mut_ptr(), err_bnds_comp.as_mut_ptr(), nparams.as_ptr(),
                      params.as_mut_ptr(), work.as_mut_ptr(), iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn cgbsvxx(fact: u8, trans: u8, n: usize, kl: usize, ku: usize, nrhs: usize, ab: &mut [c32],
               ldab: usize, afb: &mut [c32], ldafb: usize, ipiv: &mut [i32], equed: &mut u8,
               r: &mut [f32], c: &mut [f32], b: &mut [c32], ldb: usize, x: &mut [c32], ldx: usize,
               rcond: &mut [f32], rpvgrw: &mut [f32], berr: &mut [f32], n_err_bnds: &[i32],
               err_bnds_norm: &mut [f32], err_bnds_comp: &mut [f32], nparams: &[i32],
               params: &mut [f32], work: &mut [c32], rwork: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::cgbsvxx_(&(fact as c_char), &(trans as c_char), &(n as c_int), &(kl as c_int),
                      &(ku as c_int), &(nrhs as c_int), ab.as_mut_ptr() as *mut _,
                      &(ldab as c_int), afb.as_mut_ptr() as *mut _, &(ldafb as c_int),
                      ipiv.as_mut_ptr(), equed as *mut _ as *mut _, r.as_mut_ptr(), c.as_mut_ptr(),
                      b.as_mut_ptr() as *mut _, &(ldb as c_int), x.as_mut_ptr() as *mut _,
                      &(ldx as c_int), rcond.as_mut_ptr(), rpvgrw.as_mut_ptr(), berr.as_mut_ptr(),
                      n_err_bnds.as_ptr(), err_bnds_norm.as_mut_ptr(), err_bnds_comp.as_mut_ptr(),
                      nparams.as_ptr(), params.as_mut_ptr(), work.as_mut_ptr() as *mut _,
                      rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zgbsvxx(fact: u8, trans: u8, n: usize, kl: usize, ku: usize, nrhs: usize, ab: &mut [c64],
               ldab: usize, afb: &mut [c64], ldafb: usize, ipiv: &mut [i32], equed: &mut u8,
               r: &mut [f64], c: &mut [f64], b: &mut [c64], ldb: usize, x: &mut [c64], ldx: usize,
               rcond: &mut [f64], rpvgrw: &mut [f64], berr: &mut [f64], n_err_bnds: &[i32],
               err_bnds_norm: &mut [f64], err_bnds_comp: &mut [f64], nparams: &[i32],
               params: &mut [f64], work: &mut [c64], rwork: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::zgbsvxx_(&(fact as c_char), &(trans as c_char), &(n as c_int), &(kl as c_int),
                      &(ku as c_int), &(nrhs as c_int), ab.as_mut_ptr() as *mut _,
                      &(ldab as c_int), afb.as_mut_ptr() as *mut _, &(ldafb as c_int),
                      ipiv.as_mut_ptr(), equed as *mut _ as *mut _, r.as_mut_ptr(), c.as_mut_ptr(),
                      b.as_mut_ptr() as *mut _, &(ldb as c_int), x.as_mut_ptr() as *mut _,
                      &(ldx as c_int), rcond.as_mut_ptr(), rpvgrw.as_mut_ptr(), berr.as_mut_ptr(),
                      n_err_bnds.as_ptr(), err_bnds_norm.as_mut_ptr(), err_bnds_comp.as_mut_ptr(),
                      nparams.as_ptr(), params.as_mut_ptr(), work.as_mut_ptr() as *mut _,
                      rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn sgtsv(n: usize, nrhs: usize, dl: &mut [f32], d: &mut [f32], du: &mut [f32], b: &mut [f32],
             ldb: usize, info: &mut i32) {

    unsafe {
        ffi::sgtsv_(&(n as c_int), &(nrhs as c_int), dl.as_mut_ptr(), d.as_mut_ptr(),
                    du.as_mut_ptr(), b.as_mut_ptr(), &(ldb as c_int), info)
    }
}

#[inline]
pub fn dgtsv(n: usize, nrhs: usize, dl: &mut [f64], d: &mut [f64], du: &mut [f64], b: &mut [f64],
             ldb: usize, info: &mut i32) {

    unsafe {
        ffi::dgtsv_(&(n as c_int), &(nrhs as c_int), dl.as_mut_ptr(), d.as_mut_ptr(),
                    du.as_mut_ptr(), b.as_mut_ptr(), &(ldb as c_int), info)
    }
}

#[inline]
pub fn cgtsv(n: usize, nrhs: usize, dl: &mut [c32], d: &mut [c32], du: &mut [c32], b: &mut [c32],
             ldb: usize, info: &mut i32) {

    unsafe {
        ffi::cgtsv_(&(n as c_int), &(nrhs as c_int), dl.as_mut_ptr() as *mut _,
                    d.as_mut_ptr() as *mut _, du.as_mut_ptr() as *mut _, b.as_mut_ptr() as *mut _,
                    &(ldb as c_int), info)
    }
}

#[inline]
pub fn zgtsv(n: usize, nrhs: usize, dl: &mut [c64], d: &mut [c64], du: &mut [c64], b: &mut [c64],
             ldb: usize, info: &mut i32) {

    unsafe {
        ffi::zgtsv_(&(n as c_int), &(nrhs as c_int), dl.as_mut_ptr() as *mut _,
                    d.as_mut_ptr() as *mut _, du.as_mut_ptr() as *mut _, b.as_mut_ptr() as *mut _,
                    &(ldb as c_int), info)
    }
}

#[inline]
pub fn sgtsvx(fact: u8, trans: u8, n: usize, nrhs: usize, dl: &[f32], d: &[f32], du: &[f32],
              dlf: &mut [f32], df: &mut [f32], duf: &mut [f32], du2: &mut [f32], ipiv: &mut [i32],
              b: &[f32], ldb: usize, x: &mut [f32], ldx: usize, rcond: &mut [f32],
              ferr: &mut [f32], berr: &mut [f32], work: &mut [f32], iwork: &mut [i32],
              info: &mut i32) {

    unsafe {
        ffi::sgtsvx_(&(fact as c_char), &(trans as c_char), &(n as c_int), &(nrhs as c_int),
                     dl.as_ptr(), d.as_ptr(), du.as_ptr(), dlf.as_mut_ptr(), df.as_mut_ptr(),
                     duf.as_mut_ptr(), du2.as_mut_ptr(), ipiv.as_mut_ptr(), b.as_ptr(),
                     &(ldb as c_int), x.as_mut_ptr(), &(ldx as c_int), rcond.as_mut_ptr(),
                     ferr.as_mut_ptr(), berr.as_mut_ptr(), work.as_mut_ptr(), iwork.as_mut_ptr(),
                     info)
    }
}

#[inline]
pub fn dgtsvx(fact: u8, trans: u8, n: usize, nrhs: usize, dl: &[f64], d: &[f64], du: &[f64],
              dlf: &mut [f64], df: &mut [f64], duf: &mut [f64], du2: &mut [f64], ipiv: &mut [i32],
              b: &[f64], ldb: usize, x: &mut [f64], ldx: usize, rcond: &mut [f64],
              ferr: &mut [f64], berr: &mut [f64], work: &mut [f64], iwork: &mut [i32],
              info: &mut i32) {

    unsafe {
        ffi::dgtsvx_(&(fact as c_char), &(trans as c_char), &(n as c_int), &(nrhs as c_int),
                     dl.as_ptr(), d.as_ptr(), du.as_ptr(), dlf.as_mut_ptr(), df.as_mut_ptr(),
                     duf.as_mut_ptr(), du2.as_mut_ptr(), ipiv.as_mut_ptr(), b.as_ptr(),
                     &(ldb as c_int), x.as_mut_ptr(), &(ldx as c_int), rcond.as_mut_ptr(),
                     ferr.as_mut_ptr(), berr.as_mut_ptr(), work.as_mut_ptr(), iwork.as_mut_ptr(),
                     info)
    }
}

#[inline]
pub fn cgtsvx(fact: u8, trans: u8, n: usize, nrhs: usize, dl: &[c32], d: &[c32], du: &[c32],
              dlf: &mut [c32], df: &mut [c32], duf: &mut [c32], du2: &mut [c32], ipiv: &mut [i32],
              b: &[c32], ldb: usize, x: &mut [c32], ldx: usize, rcond: &mut [f32],
              ferr: &mut [f32], berr: &mut [f32], work: &mut [c32], rwork: &mut [f32],
              info: &mut i32) {

    unsafe {
        ffi::cgtsvx_(&(fact as c_char), &(trans as c_char), &(n as c_int), &(nrhs as c_int),
                     dl.as_ptr() as *const _, d.as_ptr() as *const _, du.as_ptr() as *const _,
                     dlf.as_mut_ptr() as *mut _, df.as_mut_ptr() as *mut _,
                     duf.as_mut_ptr() as *mut _, du2.as_mut_ptr() as *mut _, ipiv.as_mut_ptr(),
                     b.as_ptr() as *const _, &(ldb as c_int), x.as_mut_ptr() as *mut _,
                     &(ldx as c_int), rcond.as_mut_ptr(), ferr.as_mut_ptr(), berr.as_mut_ptr(),
                     work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zgtsvx(fact: u8, trans: u8, n: usize, nrhs: usize, dl: &[c64], d: &[c64], du: &[c64],
              dlf: &mut [c64], df: &mut [c64], duf: &mut [c64], du2: &mut [c64], ipiv: &mut [i32],
              b: &[c64], ldb: usize, x: &mut [c64], ldx: usize, rcond: &mut [f64],
              ferr: &mut [f64], berr: &mut [f64], work: &mut [c64], rwork: &mut [f64],
              info: &mut i32) {

    unsafe {
        ffi::zgtsvx_(&(fact as c_char), &(trans as c_char), &(n as c_int), &(nrhs as c_int),
                     dl.as_ptr() as *const _, d.as_ptr() as *const _, du.as_ptr() as *const _,
                     dlf.as_mut_ptr() as *mut _, df.as_mut_ptr() as *mut _,
                     duf.as_mut_ptr() as *mut _, du2.as_mut_ptr() as *mut _, ipiv.as_mut_ptr(),
                     b.as_ptr() as *const _, &(ldb as c_int), x.as_mut_ptr() as *mut _,
                     &(ldx as c_int), rcond.as_mut_ptr(), ferr.as_mut_ptr(), berr.as_mut_ptr(),
                     work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn sposv(uplo: u8, n: usize, nrhs: usize, a: &mut [f32], lda: usize, b: &mut [f32], ldb: usize,
             info: &mut i32) {

    unsafe {
        ffi::sposv_(&(uplo as c_char), &(n as c_int), &(nrhs as c_int), a.as_mut_ptr(),
                    &(lda as c_int), b.as_mut_ptr(), &(ldb as c_int), info)
    }
}

#[inline]
pub fn dposv(uplo: u8, n: usize, nrhs: usize, a: &mut [f64], lda: usize, b: &mut [f64], ldb: usize,
             info: &mut i32) {

    unsafe {
        ffi::dposv_(&(uplo as c_char), &(n as c_int), &(nrhs as c_int), a.as_mut_ptr(),
                    &(lda as c_int), b.as_mut_ptr(), &(ldb as c_int), info)
    }
}

#[inline]
pub fn cposv(uplo: u8, n: usize, nrhs: usize, a: &mut [c32], lda: usize, b: &mut [c32], ldb: usize,
             info: &mut i32) {

    unsafe {
        ffi::cposv_(&(uplo as c_char), &(n as c_int), &(nrhs as c_int), a.as_mut_ptr() as *mut _,
                    &(lda as c_int), b.as_mut_ptr() as *mut _, &(ldb as c_int), info)
    }
}

#[inline]
pub fn zposv(uplo: u8, n: usize, nrhs: usize, a: &mut [c64], lda: usize, b: &mut [c64], ldb: usize,
             info: &mut i32) {

    unsafe {
        ffi::zposv_(&(uplo as c_char), &(n as c_int), &(nrhs as c_int), a.as_mut_ptr() as *mut _,
                    &(lda as c_int), b.as_mut_ptr() as *mut _, &(ldb as c_int), info)
    }
}

#[inline]
pub fn dsposv(uplo: u8, n: usize, nrhs: usize, a: &mut [f64], lda: usize, b: &[f64], ldb: usize,
              x: &mut [f64], ldx: usize, work: &mut [f64], swork: &mut [f32], iter: &mut [i32],
              info: &mut i32) {

    unsafe {
        ffi::dsposv_(&(uplo as c_char), &(n as c_int), &(nrhs as c_int), a.as_mut_ptr(),
                     &(lda as c_int), b.as_ptr(), &(ldb as c_int), x.as_mut_ptr(), &(ldx as c_int),
                     work.as_mut_ptr(), swork.as_mut_ptr(), iter.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zcposv(uplo: u8, n: usize, nrhs: usize, a: &mut [c64], lda: usize, b: &[c64], ldb: usize,
              x: &mut [c64], ldx: usize, work: &mut [c64], swork: &mut [c32], rwork: &mut [f64],
              iter: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::zcposv_(&(uplo as c_char), &(n as c_int), &(nrhs as c_int), a.as_mut_ptr() as *mut _,
                     &(lda as c_int), b.as_ptr() as *const _, &(ldb as c_int),
                     x.as_mut_ptr() as *mut _, &(ldx as c_int), work.as_mut_ptr() as *mut _,
                     swork.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), iter.as_mut_ptr(), info)
    }
}

#[inline]
pub fn sposvx(fact: u8, uplo: u8, n: usize, nrhs: usize, a: &mut [f32], lda: usize, af: &mut [f32],
              ldaf: usize, equed: &mut u8, s: &mut [f32], b: &mut [f32], ldb: usize, x: &mut [f32],
              ldx: usize, rcond: &mut [f32], ferr: &mut [f32], berr: &mut [f32], work: &mut [f32],
              iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::sposvx_(&(fact as c_char), &(uplo as c_char), &(n as c_int), &(nrhs as c_int),
                     a.as_mut_ptr(), &(lda as c_int), af.as_mut_ptr(), &(ldaf as c_int),
                     equed as *mut _ as *mut _, s.as_mut_ptr(), b.as_mut_ptr(), &(ldb as c_int),
                     x.as_mut_ptr(), &(ldx as c_int), rcond.as_mut_ptr(), ferr.as_mut_ptr(),
                     berr.as_mut_ptr(), work.as_mut_ptr(), iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dposvx(fact: u8, uplo: u8, n: usize, nrhs: usize, a: &mut [f64], lda: usize, af: &mut [f64],
              ldaf: usize, equed: &mut u8, s: &mut [f64], b: &mut [f64], ldb: usize, x: &mut [f64],
              ldx: usize, rcond: &mut [f64], ferr: &mut [f64], berr: &mut [f64], work: &mut [f64],
              iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::dposvx_(&(fact as c_char), &(uplo as c_char), &(n as c_int), &(nrhs as c_int),
                     a.as_mut_ptr(), &(lda as c_int), af.as_mut_ptr(), &(ldaf as c_int),
                     equed as *mut _ as *mut _, s.as_mut_ptr(), b.as_mut_ptr(), &(ldb as c_int),
                     x.as_mut_ptr(), &(ldx as c_int), rcond.as_mut_ptr(), ferr.as_mut_ptr(),
                     berr.as_mut_ptr(), work.as_mut_ptr(), iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn cposvx(fact: u8, uplo: u8, n: usize, nrhs: usize, a: &mut [c32], lda: usize, af: &mut [c32],
              ldaf: usize, equed: &mut u8, s: &mut [f32], b: &mut [c32], ldb: usize, x: &mut [c32],
              ldx: usize, rcond: &mut [f32], ferr: &mut [f32], berr: &mut [f32], work: &mut [c32],
              rwork: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::cposvx_(&(fact as c_char), &(uplo as c_char), &(n as c_int), &(nrhs as c_int),
                     a.as_mut_ptr() as *mut _, &(lda as c_int), af.as_mut_ptr() as *mut _,
                     &(ldaf as c_int), equed as *mut _ as *mut _, s.as_mut_ptr(),
                     b.as_mut_ptr() as *mut _, &(ldb as c_int), x.as_mut_ptr() as *mut _,
                     &(ldx as c_int), rcond.as_mut_ptr(), ferr.as_mut_ptr(), berr.as_mut_ptr(),
                     work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zposvx(fact: u8, uplo: u8, n: usize, nrhs: usize, a: &mut [c64], lda: usize, af: &mut [c64],
              ldaf: usize, equed: &mut u8, s: &mut [f64], b: &mut [c64], ldb: usize, x: &mut [c64],
              ldx: usize, rcond: &mut [f64], ferr: &mut [f64], berr: &mut [f64], work: &mut [c64],
              rwork: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::zposvx_(&(fact as c_char), &(uplo as c_char), &(n as c_int), &(nrhs as c_int),
                     a.as_mut_ptr() as *mut _, &(lda as c_int), af.as_mut_ptr() as *mut _,
                     &(ldaf as c_int), equed as *mut _ as *mut _, s.as_mut_ptr(),
                     b.as_mut_ptr() as *mut _, &(ldb as c_int), x.as_mut_ptr() as *mut _,
                     &(ldx as c_int), rcond.as_mut_ptr(), ferr.as_mut_ptr(), berr.as_mut_ptr(),
                     work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn sposvxx(fact: u8, uplo: u8, n: usize, nrhs: usize, a: &mut [f32], lda: usize,
               af: &mut [f32], ldaf: usize, equed: &mut u8, s: &mut [f32], b: &mut [f32],
               ldb: usize, x: &mut [f32], ldx: usize, rcond: &mut [f32], rpvgrw: &mut [f32],
               berr: &mut [f32], n_err_bnds: &[i32], err_bnds_norm: &mut [f32],
               err_bnds_comp: &mut [f32], nparams: &[i32], params: &mut [f32], work: &mut [f32],
               iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::sposvxx_(&(fact as c_char), &(uplo as c_char), &(n as c_int), &(nrhs as c_int),
                      a.as_mut_ptr(), &(lda as c_int), af.as_mut_ptr(), &(ldaf as c_int),
                      equed as *mut _ as *mut _, s.as_mut_ptr(), b.as_mut_ptr(), &(ldb as c_int),
                      x.as_mut_ptr(), &(ldx as c_int), rcond.as_mut_ptr(), rpvgrw.as_mut_ptr(),
                      berr.as_mut_ptr(), n_err_bnds.as_ptr(), err_bnds_norm.as_mut_ptr(),
                      err_bnds_comp.as_mut_ptr(), nparams.as_ptr(), params.as_mut_ptr(),
                      work.as_mut_ptr(), iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dposvxx(fact: u8, uplo: u8, n: usize, nrhs: usize, a: &mut [f64], lda: usize,
               af: &mut [f64], ldaf: usize, equed: &mut u8, s: &mut [f64], b: &mut [f64],
               ldb: usize, x: &mut [f64], ldx: usize, rcond: &mut [f64], rpvgrw: &mut [f64],
               berr: &mut [f64], n_err_bnds: &[i32], err_bnds_norm: &mut [f64],
               err_bnds_comp: &mut [f64], nparams: &[i32], params: &mut [f64], work: &mut [f64],
               iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::dposvxx_(&(fact as c_char), &(uplo as c_char), &(n as c_int), &(nrhs as c_int),
                      a.as_mut_ptr(), &(lda as c_int), af.as_mut_ptr(), &(ldaf as c_int),
                      equed as *mut _ as *mut _, s.as_mut_ptr(), b.as_mut_ptr(), &(ldb as c_int),
                      x.as_mut_ptr(), &(ldx as c_int), rcond.as_mut_ptr(), rpvgrw.as_mut_ptr(),
                      berr.as_mut_ptr(), n_err_bnds.as_ptr(), err_bnds_norm.as_mut_ptr(),
                      err_bnds_comp.as_mut_ptr(), nparams.as_ptr(), params.as_mut_ptr(),
                      work.as_mut_ptr(), iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn cposvxx(fact: u8, uplo: u8, n: usize, nrhs: usize, a: &mut [c32], lda: usize,
               af: &mut [c32], ldaf: usize, equed: &mut u8, s: &mut [f32], b: &mut [c32],
               ldb: usize, x: &mut [c32], ldx: usize, rcond: &mut [f32], rpvgrw: &mut [f32],
               berr: &mut [f32], n_err_bnds: &[i32], err_bnds_norm: &mut [f32],
               err_bnds_comp: &mut [f32], nparams: &[i32], params: &mut [f32], work: &mut [c32],
               rwork: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::cposvxx_(&(fact as c_char), &(uplo as c_char), &(n as c_int), &(nrhs as c_int),
                      a.as_mut_ptr() as *mut _, &(lda as c_int), af.as_mut_ptr() as *mut _,
                      &(ldaf as c_int), equed as *mut _ as *mut _, s.as_mut_ptr(),
                      b.as_mut_ptr() as *mut _, &(ldb as c_int), x.as_mut_ptr() as *mut _,
                      &(ldx as c_int), rcond.as_mut_ptr(), rpvgrw.as_mut_ptr(), berr.as_mut_ptr(),
                      n_err_bnds.as_ptr(), err_bnds_norm.as_mut_ptr(), err_bnds_comp.as_mut_ptr(),
                      nparams.as_ptr(), params.as_mut_ptr(), work.as_mut_ptr() as *mut _,
                      rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zposvxx(fact: u8, uplo: u8, n: usize, nrhs: usize, a: &mut [c64], lda: usize,
               af: &mut [c64], ldaf: usize, equed: &mut u8, s: &mut [f64], b: &mut [c64],
               ldb: usize, x: &mut [c64], ldx: usize, rcond: &mut [f64], rpvgrw: &mut [f64],
               berr: &mut [f64], n_err_bnds: &[i32], err_bnds_norm: &mut [f64],
               err_bnds_comp: &mut [f64], nparams: &[i32], params: &mut [f64], work: &mut [c64],
               rwork: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::zposvxx_(&(fact as c_char), &(uplo as c_char), &(n as c_int), &(nrhs as c_int),
                      a.as_mut_ptr() as *mut _, &(lda as c_int), af.as_mut_ptr() as *mut _,
                      &(ldaf as c_int), equed as *mut _ as *mut _, s.as_mut_ptr(),
                      b.as_mut_ptr() as *mut _, &(ldb as c_int), x.as_mut_ptr() as *mut _,
                      &(ldx as c_int), rcond.as_mut_ptr(), rpvgrw.as_mut_ptr(), berr.as_mut_ptr(),
                      n_err_bnds.as_ptr(), err_bnds_norm.as_mut_ptr(), err_bnds_comp.as_mut_ptr(),
                      nparams.as_ptr(), params.as_mut_ptr(), work.as_mut_ptr() as *mut _,
                      rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn sppsv(uplo: u8, n: usize, nrhs: usize, ap: &mut [f32], b: &mut [f32], ldb: usize,
             info: &mut i32) {

    unsafe {
        ffi::sppsv_(&(uplo as c_char), &(n as c_int), &(nrhs as c_int), ap.as_mut_ptr(),
                    b.as_mut_ptr(), &(ldb as c_int), info)
    }
}

#[inline]
pub fn dppsv(uplo: u8, n: usize, nrhs: usize, ap: &mut [f64], b: &mut [f64], ldb: usize,
             info: &mut i32) {

    unsafe {
        ffi::dppsv_(&(uplo as c_char), &(n as c_int), &(nrhs as c_int), ap.as_mut_ptr(),
                    b.as_mut_ptr(), &(ldb as c_int), info)
    }
}

#[inline]
pub fn cppsv(uplo: u8, n: usize, nrhs: usize, ap: &mut [c32], b: &mut [c32], ldb: usize,
             info: &mut i32) {

    unsafe {
        ffi::cppsv_(&(uplo as c_char), &(n as c_int), &(nrhs as c_int), ap.as_mut_ptr() as *mut _,
                    b.as_mut_ptr() as *mut _, &(ldb as c_int), info)
    }
}

#[inline]
pub fn zppsv(uplo: u8, n: usize, nrhs: usize, ap: &mut [c64], b: &mut [c64], ldb: usize,
             info: &mut i32) {

    unsafe {
        ffi::zppsv_(&(uplo as c_char), &(n as c_int), &(nrhs as c_int), ap.as_mut_ptr() as *mut _,
                    b.as_mut_ptr() as *mut _, &(ldb as c_int), info)
    }
}

#[inline]
pub fn sppsvx(fact: u8, uplo: u8, n: usize, nrhs: usize, ap: &mut [f32], afp: &mut [f32],
              equed: &mut u8, s: &mut [f32], b: &mut [f32], ldb: usize, x: &mut [f32], ldx: usize,
              rcond: &mut [f32], ferr: &mut [f32], berr: &mut [f32], work: &mut [f32],
              iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::sppsvx_(&(fact as c_char), &(uplo as c_char), &(n as c_int), &(nrhs as c_int),
                     ap.as_mut_ptr(), afp.as_mut_ptr(), equed as *mut _ as *mut _, s.as_mut_ptr(),
                     b.as_mut_ptr(), &(ldb as c_int), x.as_mut_ptr(), &(ldx as c_int),
                     rcond.as_mut_ptr(), ferr.as_mut_ptr(), berr.as_mut_ptr(), work.as_mut_ptr(),
                     iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dppsvx(fact: u8, uplo: u8, n: usize, nrhs: usize, ap: &mut [f64], afp: &mut [f64],
              equed: &mut u8, s: &mut [f64], b: &mut [f64], ldb: usize, x: &mut [f64], ldx: usize,
              rcond: &mut [f64], ferr: &mut [f64], berr: &mut [f64], work: &mut [f64],
              iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::dppsvx_(&(fact as c_char), &(uplo as c_char), &(n as c_int), &(nrhs as c_int),
                     ap.as_mut_ptr(), afp.as_mut_ptr(), equed as *mut _ as *mut _, s.as_mut_ptr(),
                     b.as_mut_ptr(), &(ldb as c_int), x.as_mut_ptr(), &(ldx as c_int),
                     rcond.as_mut_ptr(), ferr.as_mut_ptr(), berr.as_mut_ptr(), work.as_mut_ptr(),
                     iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn cppsvx(fact: u8, uplo: u8, n: usize, nrhs: usize, ap: &mut [c32], afp: &mut [c32],
              equed: &mut u8, s: &mut [f32], b: &mut [c32], ldb: usize, x: &mut [c32], ldx: usize,
              rcond: &mut [f32], ferr: &mut [f32], berr: &mut [f32], work: &mut [c32],
              rwork: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::cppsvx_(&(fact as c_char), &(uplo as c_char), &(n as c_int), &(nrhs as c_int),
                     ap.as_mut_ptr() as *mut _, afp.as_mut_ptr() as *mut _,
                     equed as *mut _ as *mut _, s.as_mut_ptr(), b.as_mut_ptr() as *mut _,
                     &(ldb as c_int), x.as_mut_ptr() as *mut _, &(ldx as c_int),
                     rcond.as_mut_ptr(), ferr.as_mut_ptr(), berr.as_mut_ptr(),
                     work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zppsvx(fact: u8, uplo: u8, n: usize, nrhs: usize, ap: &mut [c64], afp: &mut [c64],
              equed: &mut u8, s: &mut [f64], b: &mut [c64], ldb: usize, x: &mut [c64], ldx: usize,
              rcond: &mut [f64], ferr: &mut [f64], berr: &mut [f64], work: &mut [c64],
              rwork: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::zppsvx_(&(fact as c_char), &(uplo as c_char), &(n as c_int), &(nrhs as c_int),
                     ap.as_mut_ptr() as *mut _, afp.as_mut_ptr() as *mut _,
                     equed as *mut _ as *mut _, s.as_mut_ptr(), b.as_mut_ptr() as *mut _,
                     &(ldb as c_int), x.as_mut_ptr() as *mut _, &(ldx as c_int),
                     rcond.as_mut_ptr(), ferr.as_mut_ptr(), berr.as_mut_ptr(),
                     work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn spbsv(uplo: u8, n: usize, kd: usize, nrhs: usize, ab: &mut [f32], ldab: usize,
             b: &mut [f32], ldb: usize, info: &mut i32) {

    unsafe {
        ffi::spbsv_(&(uplo as c_char), &(n as c_int), &(kd as c_int), &(nrhs as c_int),
                    ab.as_mut_ptr(), &(ldab as c_int), b.as_mut_ptr(), &(ldb as c_int), info)
    }
}

#[inline]
pub fn dpbsv(uplo: u8, n: usize, kd: usize, nrhs: usize, ab: &mut [f64], ldab: usize,
             b: &mut [f64], ldb: usize, info: &mut i32) {

    unsafe {
        ffi::dpbsv_(&(uplo as c_char), &(n as c_int), &(kd as c_int), &(nrhs as c_int),
                    ab.as_mut_ptr(), &(ldab as c_int), b.as_mut_ptr(), &(ldb as c_int), info)
    }
}

#[inline]
pub fn cpbsv(uplo: u8, n: usize, kd: usize, nrhs: usize, ab: &mut [c32], ldab: usize,
             b: &mut [c32], ldb: usize, info: &mut i32) {

    unsafe {
        ffi::cpbsv_(&(uplo as c_char), &(n as c_int), &(kd as c_int), &(nrhs as c_int),
                    ab.as_mut_ptr() as *mut _, &(ldab as c_int), b.as_mut_ptr() as *mut _,
                    &(ldb as c_int), info)
    }
}

#[inline]
pub fn zpbsv(uplo: u8, n: usize, kd: usize, nrhs: usize, ab: &mut [c64], ldab: usize,
             b: &mut [c64], ldb: usize, info: &mut i32) {

    unsafe {
        ffi::zpbsv_(&(uplo as c_char), &(n as c_int), &(kd as c_int), &(nrhs as c_int),
                    ab.as_mut_ptr() as *mut _, &(ldab as c_int), b.as_mut_ptr() as *mut _,
                    &(ldb as c_int), info)
    }
}

#[inline]
pub fn spbsvx(fact: u8, uplo: u8, n: usize, kd: usize, nrhs: usize, ab: &mut [f32], ldab: usize,
              afb: &mut [f32], ldafb: usize, equed: &mut u8, s: &mut [f32], b: &mut [f32],
              ldb: usize, x: &mut [f32], ldx: usize, rcond: &mut [f32], ferr: &mut [f32],
              berr: &mut [f32], work: &mut [f32], iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::spbsvx_(&(fact as c_char), &(uplo as c_char), &(n as c_int), &(kd as c_int),
                     &(nrhs as c_int), ab.as_mut_ptr(), &(ldab as c_int), afb.as_mut_ptr(),
                     &(ldafb as c_int), equed as *mut _ as *mut _, s.as_mut_ptr(), b.as_mut_ptr(),
                     &(ldb as c_int), x.as_mut_ptr(), &(ldx as c_int), rcond.as_mut_ptr(),
                     ferr.as_mut_ptr(), berr.as_mut_ptr(), work.as_mut_ptr(), iwork.as_mut_ptr(),
                     info)
    }
}

#[inline]
pub fn dpbsvx(fact: u8, uplo: u8, n: usize, kd: usize, nrhs: usize, ab: &mut [f64], ldab: usize,
              afb: &mut [f64], ldafb: usize, equed: &mut u8, s: &mut [f64], b: &mut [f64],
              ldb: usize, x: &mut [f64], ldx: usize, rcond: &mut [f64], ferr: &mut [f64],
              berr: &mut [f64], work: &mut [f64], iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::dpbsvx_(&(fact as c_char), &(uplo as c_char), &(n as c_int), &(kd as c_int),
                     &(nrhs as c_int), ab.as_mut_ptr(), &(ldab as c_int), afb.as_mut_ptr(),
                     &(ldafb as c_int), equed as *mut _ as *mut _, s.as_mut_ptr(), b.as_mut_ptr(),
                     &(ldb as c_int), x.as_mut_ptr(), &(ldx as c_int), rcond.as_mut_ptr(),
                     ferr.as_mut_ptr(), berr.as_mut_ptr(), work.as_mut_ptr(), iwork.as_mut_ptr(),
                     info)
    }
}

#[inline]
pub fn cpbsvx(fact: u8, uplo: u8, n: usize, kd: usize, nrhs: usize, ab: &mut [c32], ldab: usize,
              afb: &mut [c32], ldafb: usize, equed: &mut u8, s: &mut [f32], b: &mut [c32],
              ldb: usize, x: &mut [c32], ldx: usize, rcond: &mut [f32], ferr: &mut [f32],
              berr: &mut [f32], work: &mut [c32], rwork: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::cpbsvx_(&(fact as c_char), &(uplo as c_char), &(n as c_int), &(kd as c_int),
                     &(nrhs as c_int), ab.as_mut_ptr() as *mut _, &(ldab as c_int),
                     afb.as_mut_ptr() as *mut _, &(ldafb as c_int), equed as *mut _ as *mut _,
                     s.as_mut_ptr(), b.as_mut_ptr() as *mut _, &(ldb as c_int),
                     x.as_mut_ptr() as *mut _, &(ldx as c_int), rcond.as_mut_ptr(),
                     ferr.as_mut_ptr(), berr.as_mut_ptr(), work.as_mut_ptr() as *mut _,
                     rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zpbsvx(fact: u8, uplo: u8, n: usize, kd: usize, nrhs: usize, ab: &mut [c64], ldab: usize,
              afb: &mut [c64], ldafb: usize, equed: &mut u8, s: &mut [f64], b: &mut [c64],
              ldb: usize, x: &mut [c64], ldx: usize, rcond: &mut [f64], ferr: &mut [f64],
              berr: &mut [f64], work: &mut [c64], rwork: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::zpbsvx_(&(fact as c_char), &(uplo as c_char), &(n as c_int), &(kd as c_int),
                     &(nrhs as c_int), ab.as_mut_ptr() as *mut _, &(ldab as c_int),
                     afb.as_mut_ptr() as *mut _, &(ldafb as c_int), equed as *mut _ as *mut _,
                     s.as_mut_ptr(), b.as_mut_ptr() as *mut _, &(ldb as c_int),
                     x.as_mut_ptr() as *mut _, &(ldx as c_int), rcond.as_mut_ptr(),
                     ferr.as_mut_ptr(), berr.as_mut_ptr(), work.as_mut_ptr() as *mut _,
                     rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn sptsv(n: usize, nrhs: usize, d: &mut [f32], e: &mut [f32], b: &mut [f32], ldb: usize,
             info: &mut i32) {

    unsafe {
        ffi::sptsv_(&(n as c_int), &(nrhs as c_int), d.as_mut_ptr(), e.as_mut_ptr(),
                    b.as_mut_ptr(), &(ldb as c_int), info)
    }
}

#[inline]
pub fn dptsv(n: usize, nrhs: usize, d: &mut [f64], e: &mut [f64], b: &mut [f64], ldb: usize,
             info: &mut i32) {

    unsafe {
        ffi::dptsv_(&(n as c_int), &(nrhs as c_int), d.as_mut_ptr(), e.as_mut_ptr(),
                    b.as_mut_ptr(), &(ldb as c_int), info)
    }
}

#[inline]
pub fn cptsv(n: usize, nrhs: usize, d: &mut [f32], e: &mut [c32], b: &mut [c32], ldb: usize,
             info: &mut i32) {

    unsafe {
        ffi::cptsv_(&(n as c_int), &(nrhs as c_int), d.as_mut_ptr(), e.as_mut_ptr() as *mut _,
                    b.as_mut_ptr() as *mut _, &(ldb as c_int), info)
    }
}

#[inline]
pub fn zptsv(n: usize, nrhs: usize, d: &mut [f64], e: &mut [c64], b: &mut [c64], ldb: usize,
             info: &mut i32) {

    unsafe {
        ffi::zptsv_(&(n as c_int), &(nrhs as c_int), d.as_mut_ptr(), e.as_mut_ptr() as *mut _,
                    b.as_mut_ptr() as *mut _, &(ldb as c_int), info)
    }
}

#[inline]
pub fn sptsvx(fact: u8, n: usize, nrhs: usize, d: &[f32], e: &[f32], df: &mut [f32],
              ef: &mut [f32], b: &[f32], ldb: usize, x: &mut [f32], ldx: usize, rcond: &mut [f32],
              ferr: &mut [f32], berr: &mut [f32], work: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::sptsvx_(&(fact as c_char), &(n as c_int), &(nrhs as c_int), d.as_ptr(), e.as_ptr(),
                     df.as_mut_ptr(), ef.as_mut_ptr(), b.as_ptr(), &(ldb as c_int), x.as_mut_ptr(),
                     &(ldx as c_int), rcond.as_mut_ptr(), ferr.as_mut_ptr(), berr.as_mut_ptr(),
                     work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dptsvx(fact: u8, n: usize, nrhs: usize, d: &[f64], e: &[f64], df: &mut [f64],
              ef: &mut [f64], b: &[f64], ldb: usize, x: &mut [f64], ldx: usize, rcond: &mut [f64],
              ferr: &mut [f64], berr: &mut [f64], work: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::dptsvx_(&(fact as c_char), &(n as c_int), &(nrhs as c_int), d.as_ptr(), e.as_ptr(),
                     df.as_mut_ptr(), ef.as_mut_ptr(), b.as_ptr(), &(ldb as c_int), x.as_mut_ptr(),
                     &(ldx as c_int), rcond.as_mut_ptr(), ferr.as_mut_ptr(), berr.as_mut_ptr(),
                     work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn cptsvx(fact: u8, n: usize, nrhs: usize, d: &[f32], e: &[c32], df: &mut [f32],
              ef: &mut [c32], b: &[c32], ldb: usize, x: &mut [c32], ldx: usize, rcond: &mut [f32],
              ferr: &mut [f32], berr: &mut [f32], work: &mut [c32], rwork: &mut [f32],
              info: &mut i32) {

    unsafe {
        ffi::cptsvx_(&(fact as c_char), &(n as c_int), &(nrhs as c_int), d.as_ptr(),
                     e.as_ptr() as *const _, df.as_mut_ptr(), ef.as_mut_ptr() as *mut _,
                     b.as_ptr() as *const _, &(ldb as c_int), x.as_mut_ptr() as *mut _,
                     &(ldx as c_int), rcond.as_mut_ptr(), ferr.as_mut_ptr(), berr.as_mut_ptr(),
                     work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zptsvx(fact: u8, n: usize, nrhs: usize, d: &[f64], e: &[c64], df: &mut [f64],
              ef: &mut [c64], b: &[c64], ldb: usize, x: &mut [c64], ldx: usize, rcond: &mut [f64],
              ferr: &mut [f64], berr: &mut [f64], work: &mut [c64], rwork: &mut [f64],
              info: &mut i32) {

    unsafe {
        ffi::zptsvx_(&(fact as c_char), &(n as c_int), &(nrhs as c_int), d.as_ptr(),
                     e.as_ptr() as *const _, df.as_mut_ptr(), ef.as_mut_ptr() as *mut _,
                     b.as_ptr() as *const _, &(ldb as c_int), x.as_mut_ptr() as *mut _,
                     &(ldx as c_int), rcond.as_mut_ptr(), ferr.as_mut_ptr(), berr.as_mut_ptr(),
                     work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn ssysv(uplo: u8, n: usize, nrhs: usize, a: &mut [f32], lda: usize, ipiv: &mut [i32],
             b: &mut [f32], ldb: usize, work: &mut [f32], lwork: usize, info: &mut i32) {

    unsafe {
        ffi::ssysv_(&(uplo as c_char), &(n as c_int), &(nrhs as c_int), a.as_mut_ptr(),
                    &(lda as c_int), ipiv.as_mut_ptr(), b.as_mut_ptr(), &(ldb as c_int),
                    work.as_mut_ptr(), &(lwork as c_int), info)
    }
}

#[inline]
pub fn dsysv(uplo: u8, n: usize, nrhs: usize, a: &mut [f64], lda: usize, ipiv: &mut [i32],
             b: &mut [f64], ldb: usize, work: &mut [f64], lwork: usize, info: &mut i32) {

    unsafe {
        ffi::dsysv_(&(uplo as c_char), &(n as c_int), &(nrhs as c_int), a.as_mut_ptr(),
                    &(lda as c_int), ipiv.as_mut_ptr(), b.as_mut_ptr(), &(ldb as c_int),
                    work.as_mut_ptr(), &(lwork as c_int), info)
    }
}

#[inline]
pub fn csysv(uplo: u8, n: usize, nrhs: usize, a: &mut [c32], lda: usize, ipiv: &mut [i32],
             b: &mut [c32], ldb: usize, work: &mut [c32], lwork: usize, info: &mut i32) {

    unsafe {
        ffi::csysv_(&(uplo as c_char), &(n as c_int), &(nrhs as c_int), a.as_mut_ptr() as *mut _,
                    &(lda as c_int), ipiv.as_mut_ptr(), b.as_mut_ptr() as *mut _, &(ldb as c_int),
                    work.as_mut_ptr() as *mut _, &(lwork as c_int), info)
    }
}

#[inline]
pub fn zsysv(uplo: u8, n: usize, nrhs: usize, a: &mut [c64], lda: usize, ipiv: &mut [i32],
             b: &mut [c64], ldb: usize, work: &mut [c64], lwork: usize, info: &mut i32) {

    unsafe {
        ffi::zsysv_(&(uplo as c_char), &(n as c_int), &(nrhs as c_int), a.as_mut_ptr() as *mut _,
                    &(lda as c_int), ipiv.as_mut_ptr(), b.as_mut_ptr() as *mut _, &(ldb as c_int),
                    work.as_mut_ptr() as *mut _, &(lwork as c_int), info)
    }
}

#[inline]
pub fn ssysvx(fact: u8, uplo: u8, n: usize, nrhs: usize, a: &[f32], lda: usize, af: &mut [f32],
              ldaf: usize, ipiv: &mut [i32], b: &[f32], ldb: usize, x: &mut [f32], ldx: usize,
              rcond: &mut [f32], ferr: &mut [f32], berr: &mut [f32], work: &mut [f32],
              lwork: usize, iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::ssysvx_(&(fact as c_char), &(uplo as c_char), &(n as c_int), &(nrhs as c_int),
                     a.as_ptr(), &(lda as c_int), af.as_mut_ptr(), &(ldaf as c_int),
                     ipiv.as_mut_ptr(), b.as_ptr(), &(ldb as c_int), x.as_mut_ptr(),
                     &(ldx as c_int), rcond.as_mut_ptr(), ferr.as_mut_ptr(), berr.as_mut_ptr(),
                     work.as_mut_ptr(), &(lwork as c_int), iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dsysvx(fact: u8, uplo: u8, n: usize, nrhs: usize, a: &[f64], lda: usize, af: &mut [f64],
              ldaf: usize, ipiv: &mut [i32], b: &[f64], ldb: usize, x: &mut [f64], ldx: usize,
              rcond: &mut [f64], ferr: &mut [f64], berr: &mut [f64], work: &mut [f64],
              lwork: usize, iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::dsysvx_(&(fact as c_char), &(uplo as c_char), &(n as c_int), &(nrhs as c_int),
                     a.as_ptr(), &(lda as c_int), af.as_mut_ptr(), &(ldaf as c_int),
                     ipiv.as_mut_ptr(), b.as_ptr(), &(ldb as c_int), x.as_mut_ptr(),
                     &(ldx as c_int), rcond.as_mut_ptr(), ferr.as_mut_ptr(), berr.as_mut_ptr(),
                     work.as_mut_ptr(), &(lwork as c_int), iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn csysvx(fact: u8, uplo: u8, n: usize, nrhs: usize, a: &[c32], lda: usize, af: &mut [c32],
              ldaf: usize, ipiv: &mut [i32], b: &[c32], ldb: usize, x: &mut [c32], ldx: usize,
              rcond: &mut [f32], ferr: &mut [f32], berr: &mut [f32], work: &mut [c32],
              lwork: usize, rwork: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::csysvx_(&(fact as c_char), &(uplo as c_char), &(n as c_int), &(nrhs as c_int),
                     a.as_ptr() as *const _, &(lda as c_int), af.as_mut_ptr() as *mut _,
                     &(ldaf as c_int), ipiv.as_mut_ptr(), b.as_ptr() as *const _, &(ldb as c_int),
                     x.as_mut_ptr() as *mut _, &(ldx as c_int), rcond.as_mut_ptr(),
                     ferr.as_mut_ptr(), berr.as_mut_ptr(), work.as_mut_ptr() as *mut _,
                     &(lwork as c_int), rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zsysvx(fact: u8, uplo: u8, n: usize, nrhs: usize, a: &[c64], lda: usize, af: &mut [c64],
              ldaf: usize, ipiv: &mut [i32], b: &[c64], ldb: usize, x: &mut [c64], ldx: usize,
              rcond: &mut [f64], ferr: &mut [f64], berr: &mut [f64], work: &mut [c64],
              lwork: usize, rwork: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::zsysvx_(&(fact as c_char), &(uplo as c_char), &(n as c_int), &(nrhs as c_int),
                     a.as_ptr() as *const _, &(lda as c_int), af.as_mut_ptr() as *mut _,
                     &(ldaf as c_int), ipiv.as_mut_ptr(), b.as_ptr() as *const _, &(ldb as c_int),
                     x.as_mut_ptr() as *mut _, &(ldx as c_int), rcond.as_mut_ptr(),
                     ferr.as_mut_ptr(), berr.as_mut_ptr(), work.as_mut_ptr() as *mut _,
                     &(lwork as c_int), rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dsysvxx(fact: u8, uplo: &mut u8, n: usize, nrhs: usize, a: &mut [f64], lda: usize,
               af: &mut [f64], ldaf: usize, ipiv: &mut [i32], equed: &mut u8, s: &mut [f64],
               b: &mut [f64], ldb: usize, x: &mut [f64], ldx: usize, rcond: &mut [f64],
               rpvgrw: &mut [f64], berr: &mut [f64], n_err_bnds: &[i32], err_bnds_norm: &mut [f64],
               err_bnds_comp: &mut [f64], nparams: &[i32], params: &mut [f64], work: &mut [f64],
               iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::dsysvxx_(&(fact as c_char), uplo as *mut _ as *mut _, &(n as c_int), &(nrhs as c_int),
                      a.as_mut_ptr(), &(lda as c_int), af.as_mut_ptr(), &(ldaf as c_int),
                      ipiv.as_mut_ptr(), equed as *mut _ as *mut _, s.as_mut_ptr(), b.as_mut_ptr(),
                      &(ldb as c_int), x.as_mut_ptr(), &(ldx as c_int), rcond.as_mut_ptr(),
                      rpvgrw.as_mut_ptr(), berr.as_mut_ptr(), n_err_bnds.as_ptr(),
                      err_bnds_norm.as_mut_ptr(), err_bnds_comp.as_mut_ptr(), nparams.as_ptr(),
                      params.as_mut_ptr(), work.as_mut_ptr(), iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn ssysvxx(fact: u8, uplo: &mut u8, n: usize, nrhs: usize, a: &mut [f32], lda: usize,
               af: &mut [f32], ldaf: usize, ipiv: &mut [i32], equed: &mut u8, s: &mut [f32],
               b: &mut [f32], ldb: usize, x: &mut [f32], ldx: usize, rcond: &mut [f32],
               rpvgrw: &mut [f32], berr: &mut [f32], n_err_bnds: &[i32], err_bnds_norm: &mut [f32],
               err_bnds_comp: &mut [f32], nparams: &[i32], params: &mut [f32], work: &mut [f32],
               iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::ssysvxx_(&(fact as c_char), uplo as *mut _ as *mut _, &(n as c_int), &(nrhs as c_int),
                      a.as_mut_ptr(), &(lda as c_int), af.as_mut_ptr(), &(ldaf as c_int),
                      ipiv.as_mut_ptr(), equed as *mut _ as *mut _, s.as_mut_ptr(), b.as_mut_ptr(),
                      &(ldb as c_int), x.as_mut_ptr(), &(ldx as c_int), rcond.as_mut_ptr(),
                      rpvgrw.as_mut_ptr(), berr.as_mut_ptr(), n_err_bnds.as_ptr(),
                      err_bnds_norm.as_mut_ptr(), err_bnds_comp.as_mut_ptr(), nparams.as_ptr(),
                      params.as_mut_ptr(), work.as_mut_ptr(), iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn csysvxx(fact: u8, uplo: &mut u8, n: usize, nrhs: usize, a: &mut [c32], lda: usize,
               af: &mut [c32], ldaf: usize, ipiv: &mut [i32], equed: &mut u8, s: &mut [f32],
               b: &mut [c32], ldb: usize, x: &mut [c32], ldx: usize, rcond: &mut [f32],
               rpvgrw: &mut [f32], berr: &mut [f32], n_err_bnds: &[i32], err_bnds_norm: &mut [f32],
               err_bnds_comp: &mut [f32], nparams: &[i32], params: &mut [f32], work: &mut [c32],
               rwork: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::csysvxx_(&(fact as c_char), uplo as *mut _ as *mut _, &(n as c_int), &(nrhs as c_int),
                      a.as_mut_ptr() as *mut _, &(lda as c_int), af.as_mut_ptr() as *mut _,
                      &(ldaf as c_int), ipiv.as_mut_ptr(), equed as *mut _ as *mut _,
                      s.as_mut_ptr(), b.as_mut_ptr() as *mut _, &(ldb as c_int),
                      x.as_mut_ptr() as *mut _, &(ldx as c_int), rcond.as_mut_ptr(),
                      rpvgrw.as_mut_ptr(), berr.as_mut_ptr(), n_err_bnds.as_ptr(),
                      err_bnds_norm.as_mut_ptr(), err_bnds_comp.as_mut_ptr(), nparams.as_ptr(),
                      params.as_mut_ptr(), work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zsysvxx(fact: u8, uplo: &mut u8, n: usize, nrhs: usize, a: &mut [c64], lda: usize,
               af: &mut [c64], ldaf: usize, ipiv: &mut [i32], equed: &mut u8, s: &mut [f64],
               b: &mut [c64], ldb: usize, x: &mut [c64], ldx: usize, rcond: &mut [f64],
               rpvgrw: &mut [f64], berr: &mut [f64], n_err_bnds: &[i32], err_bnds_norm: &mut [f64],
               err_bnds_comp: &mut [f64], nparams: &[i32], params: &mut [f64], work: &mut [c64],
               rwork: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::zsysvxx_(&(fact as c_char), uplo as *mut _ as *mut _, &(n as c_int), &(nrhs as c_int),
                      a.as_mut_ptr() as *mut _, &(lda as c_int), af.as_mut_ptr() as *mut _,
                      &(ldaf as c_int), ipiv.as_mut_ptr(), equed as *mut _ as *mut _,
                      s.as_mut_ptr(), b.as_mut_ptr() as *mut _, &(ldb as c_int),
                      x.as_mut_ptr() as *mut _, &(ldx as c_int), rcond.as_mut_ptr(),
                      rpvgrw.as_mut_ptr(), berr.as_mut_ptr(), n_err_bnds.as_ptr(),
                      err_bnds_norm.as_mut_ptr(), err_bnds_comp.as_mut_ptr(), nparams.as_ptr(),
                      params.as_mut_ptr(), work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn chesv(uplo: u8, n: usize, nrhs: usize, a: &mut [c32], lda: usize, ipiv: &mut [i32],
             b: &mut [c32], ldb: usize, work: &mut [c32], lwork: usize, info: &mut i32) {

    unsafe {
        ffi::chesv_(&(uplo as c_char), &(n as c_int), &(nrhs as c_int), a.as_mut_ptr() as *mut _,
                    &(lda as c_int), ipiv.as_mut_ptr(), b.as_mut_ptr() as *mut _, &(ldb as c_int),
                    work.as_mut_ptr() as *mut _, &(lwork as c_int), info)
    }
}

#[inline]
pub fn zhesv(uplo: u8, n: usize, nrhs: usize, a: &mut [c64], lda: usize, ipiv: &mut [i32],
             b: &mut [c64], ldb: usize, work: &mut [c64], lwork: usize, info: &mut i32) {

    unsafe {
        ffi::zhesv_(&(uplo as c_char), &(n as c_int), &(nrhs as c_int), a.as_mut_ptr() as *mut _,
                    &(lda as c_int), ipiv.as_mut_ptr(), b.as_mut_ptr() as *mut _, &(ldb as c_int),
                    work.as_mut_ptr() as *mut _, &(lwork as c_int), info)
    }
}

#[inline]
pub fn chesvx(fact: u8, uplo: u8, n: usize, nrhs: usize, a: &[c32], lda: usize, af: &mut [c32],
              ldaf: usize, ipiv: &mut [i32], b: &[c32], ldb: usize, x: &mut [c32], ldx: usize,
              rcond: &mut [f32], ferr: &mut [f32], berr: &mut [f32], work: &mut [c32],
              lwork: usize, rwork: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::chesvx_(&(fact as c_char), &(uplo as c_char), &(n as c_int), &(nrhs as c_int),
                     a.as_ptr() as *const _, &(lda as c_int), af.as_mut_ptr() as *mut _,
                     &(ldaf as c_int), ipiv.as_mut_ptr(), b.as_ptr() as *const _, &(ldb as c_int),
                     x.as_mut_ptr() as *mut _, &(ldx as c_int), rcond.as_mut_ptr(),
                     ferr.as_mut_ptr(), berr.as_mut_ptr(), work.as_mut_ptr() as *mut _,
                     &(lwork as c_int), rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zhesvx(fact: u8, uplo: u8, n: usize, nrhs: usize, a: &[c64], lda: usize, af: &mut [c64],
              ldaf: usize, ipiv: &mut [i32], b: &[c64], ldb: usize, x: &mut [c64], ldx: usize,
              rcond: &mut [f64], ferr: &mut [f64], berr: &mut [f64], work: &mut [c64],
              lwork: usize, rwork: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::zhesvx_(&(fact as c_char), &(uplo as c_char), &(n as c_int), &(nrhs as c_int),
                     a.as_ptr() as *const _, &(lda as c_int), af.as_mut_ptr() as *mut _,
                     &(ldaf as c_int), ipiv.as_mut_ptr(), b.as_ptr() as *const _, &(ldb as c_int),
                     x.as_mut_ptr() as *mut _, &(ldx as c_int), rcond.as_mut_ptr(),
                     ferr.as_mut_ptr(), berr.as_mut_ptr(), work.as_mut_ptr() as *mut _,
                     &(lwork as c_int), rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn chesvxx(fact: u8, uplo: &mut u8, n: usize, nrhs: usize, a: &mut [c32], lda: usize,
               af: &mut [c32], ldaf: usize, ipiv: &mut [i32], equed: &mut u8, s: &mut [f32],
               b: &mut [c32], ldb: usize, x: &mut [c32], ldx: usize, rcond: &mut [f32],
               rpvgrw: &mut [f32], berr: &mut [f32], n_err_bnds: &[i32], err_bnds_norm: &mut [f32],
               err_bnds_comp: &mut [f32], nparams: &[i32], params: &mut [f32], work: &mut [c32],
               rwork: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::chesvxx_(&(fact as c_char), uplo as *mut _ as *mut _, &(n as c_int), &(nrhs as c_int),
                      a.as_mut_ptr() as *mut _, &(lda as c_int), af.as_mut_ptr() as *mut _,
                      &(ldaf as c_int), ipiv.as_mut_ptr(), equed as *mut _ as *mut _,
                      s.as_mut_ptr(), b.as_mut_ptr() as *mut _, &(ldb as c_int),
                      x.as_mut_ptr() as *mut _, &(ldx as c_int), rcond.as_mut_ptr(),
                      rpvgrw.as_mut_ptr(), berr.as_mut_ptr(), n_err_bnds.as_ptr(),
                      err_bnds_norm.as_mut_ptr(), err_bnds_comp.as_mut_ptr(), nparams.as_ptr(),
                      params.as_mut_ptr(), work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zhesvxx(fact: u8, uplo: &mut u8, n: usize, nrhs: usize, a: &mut [c64], lda: usize,
               af: &mut [c64], ldaf: usize, ipiv: &mut [i32], equed: &mut u8, s: &mut [f64],
               b: &mut [c64], ldb: usize, x: &mut [c64], ldx: usize, rcond: &mut [f64],
               rpvgrw: &mut [f64], berr: &mut [f64], n_err_bnds: &[i32], err_bnds_norm: &mut [f64],
               err_bnds_comp: &mut [f64], nparams: &[i32], params: &mut [f64], work: &mut [c64],
               rwork: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::zhesvxx_(&(fact as c_char), uplo as *mut _ as *mut _, &(n as c_int), &(nrhs as c_int),
                      a.as_mut_ptr() as *mut _, &(lda as c_int), af.as_mut_ptr() as *mut _,
                      &(ldaf as c_int), ipiv.as_mut_ptr(), equed as *mut _ as *mut _,
                      s.as_mut_ptr(), b.as_mut_ptr() as *mut _, &(ldb as c_int),
                      x.as_mut_ptr() as *mut _, &(ldx as c_int), rcond.as_mut_ptr(),
                      rpvgrw.as_mut_ptr(), berr.as_mut_ptr(), n_err_bnds.as_ptr(),
                      err_bnds_norm.as_mut_ptr(), err_bnds_comp.as_mut_ptr(), nparams.as_ptr(),
                      params.as_mut_ptr(), work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn sspsv(uplo: u8, n: usize, nrhs: usize, ap: &mut [f32], ipiv: &mut [i32], b: &mut [f32],
             ldb: usize, info: &mut i32) {

    unsafe {
        ffi::sspsv_(&(uplo as c_char), &(n as c_int), &(nrhs as c_int), ap.as_mut_ptr(),
                    ipiv.as_mut_ptr(), b.as_mut_ptr(), &(ldb as c_int), info)
    }
}

#[inline]
pub fn dspsv(uplo: u8, n: usize, nrhs: usize, ap: &mut [f64], ipiv: &mut [i32], b: &mut [f64],
             ldb: usize, info: &mut i32) {

    unsafe {
        ffi::dspsv_(&(uplo as c_char), &(n as c_int), &(nrhs as c_int), ap.as_mut_ptr(),
                    ipiv.as_mut_ptr(), b.as_mut_ptr(), &(ldb as c_int), info)
    }
}

#[inline]
pub fn cspsv(uplo: u8, n: usize, nrhs: usize, ap: &mut [c32], ipiv: &mut [i32], b: &mut [c32],
             ldb: usize, info: &mut i32) {

    unsafe {
        ffi::cspsv_(&(uplo as c_char), &(n as c_int), &(nrhs as c_int), ap.as_mut_ptr() as *mut _,
                    ipiv.as_mut_ptr(), b.as_mut_ptr() as *mut _, &(ldb as c_int), info)
    }
}

#[inline]
pub fn zspsv(uplo: u8, n: usize, nrhs: usize, ap: &mut [c64], ipiv: &mut [i32], b: &mut [c64],
             ldb: usize, info: &mut i32) {

    unsafe {
        ffi::zspsv_(&(uplo as c_char), &(n as c_int), &(nrhs as c_int), ap.as_mut_ptr() as *mut _,
                    ipiv.as_mut_ptr(), b.as_mut_ptr() as *mut _, &(ldb as c_int), info)
    }
}

#[inline]
pub fn sspsvx(fact: u8, uplo: u8, n: usize, nrhs: usize, ap: &[f32], afp: &mut [f32],
              ipiv: &mut [i32], b: &[f32], ldb: usize, x: &mut [f32], ldx: usize,
              rcond: &mut [f32], ferr: &mut [f32], berr: &mut [f32], work: &mut [f32],
              iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::sspsvx_(&(fact as c_char), &(uplo as c_char), &(n as c_int), &(nrhs as c_int),
                     ap.as_ptr(), afp.as_mut_ptr(), ipiv.as_mut_ptr(), b.as_ptr(), &(ldb as c_int),
                     x.as_mut_ptr(), &(ldx as c_int), rcond.as_mut_ptr(), ferr.as_mut_ptr(),
                     berr.as_mut_ptr(), work.as_mut_ptr(), iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dspsvx(fact: u8, uplo: u8, n: usize, nrhs: usize, ap: &[f64], afp: &mut [f64],
              ipiv: &mut [i32], b: &[f64], ldb: usize, x: &mut [f64], ldx: usize,
              rcond: &mut [f64], ferr: &mut [f64], berr: &mut [f64], work: &mut [f64],
              iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::dspsvx_(&(fact as c_char), &(uplo as c_char), &(n as c_int), &(nrhs as c_int),
                     ap.as_ptr(), afp.as_mut_ptr(), ipiv.as_mut_ptr(), b.as_ptr(), &(ldb as c_int),
                     x.as_mut_ptr(), &(ldx as c_int), rcond.as_mut_ptr(), ferr.as_mut_ptr(),
                     berr.as_mut_ptr(), work.as_mut_ptr(), iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn cspsvx(fact: u8, uplo: u8, n: usize, nrhs: usize, ap: &[c32], afp: &mut [c32],
              ipiv: &mut [i32], b: &[c32], ldb: usize, x: &mut [c32], ldx: usize,
              rcond: &mut [f32], ferr: &mut [f32], berr: &mut [f32], work: &mut [c32],
              rwork: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::cspsvx_(&(fact as c_char), &(uplo as c_char), &(n as c_int), &(nrhs as c_int),
                     ap.as_ptr() as *const _, afp.as_mut_ptr() as *mut _, ipiv.as_mut_ptr(),
                     b.as_ptr() as *const _, &(ldb as c_int), x.as_mut_ptr() as *mut _,
                     &(ldx as c_int), rcond.as_mut_ptr(), ferr.as_mut_ptr(), berr.as_mut_ptr(),
                     work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zspsvx(fact: u8, uplo: u8, n: usize, nrhs: usize, ap: &[c64], afp: &mut [c64],
              ipiv: &mut [i32], b: &[c64], ldb: usize, x: &mut [c64], ldx: usize,
              rcond: &mut [f64], ferr: &mut [f64], berr: &mut [f64], work: &mut [c64],
              rwork: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::zspsvx_(&(fact as c_char), &(uplo as c_char), &(n as c_int), &(nrhs as c_int),
                     ap.as_ptr() as *const _, afp.as_mut_ptr() as *mut _, ipiv.as_mut_ptr(),
                     b.as_ptr() as *const _, &(ldb as c_int), x.as_mut_ptr() as *mut _,
                     &(ldx as c_int), rcond.as_mut_ptr(), ferr.as_mut_ptr(), berr.as_mut_ptr(),
                     work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn chpsv(uplo: u8, n: usize, nrhs: usize, ap: &mut [c32], ipiv: &mut [i32], b: &mut [c32],
             ldb: usize, info: &mut i32) {

    unsafe {
        ffi::chpsv_(&(uplo as c_char), &(n as c_int), &(nrhs as c_int), ap.as_mut_ptr() as *mut _,
                    ipiv.as_mut_ptr(), b.as_mut_ptr() as *mut _, &(ldb as c_int), info)
    }
}

#[inline]
pub fn zhpsv(uplo: u8, n: usize, nrhs: usize, ap: &mut [c64], ipiv: &mut [i32], b: &mut [c64],
             ldb: usize, info: &mut i32) {

    unsafe {
        ffi::zhpsv_(&(uplo as c_char), &(n as c_int), &(nrhs as c_int), ap.as_mut_ptr() as *mut _,
                    ipiv.as_mut_ptr(), b.as_mut_ptr() as *mut _, &(ldb as c_int), info)
    }
}

#[inline]
pub fn chpsvx(fact: u8, uplo: u8, n: usize, nrhs: usize, ap: &[c32], afp: &mut [c32],
              ipiv: &mut [i32], b: &[c32], ldb: usize, x: &mut [c32], ldx: usize,
              rcond: &mut [f32], ferr: &mut [f32], berr: &mut [f32], work: &mut [c32],
              rwork: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::chpsvx_(&(fact as c_char), &(uplo as c_char), &(n as c_int), &(nrhs as c_int),
                     ap.as_ptr() as *const _, afp.as_mut_ptr() as *mut _, ipiv.as_mut_ptr(),
                     b.as_ptr() as *const _, &(ldb as c_int), x.as_mut_ptr() as *mut _,
                     &(ldx as c_int), rcond.as_mut_ptr(), ferr.as_mut_ptr(), berr.as_mut_ptr(),
                     work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zhpsvx(fact: u8, uplo: u8, n: usize, nrhs: usize, ap: &[c64], afp: &mut [c64],
              ipiv: &mut [i32], b: &[c64], ldb: usize, x: &mut [c64], ldx: usize,
              rcond: &mut [f64], ferr: &mut [f64], berr: &mut [f64], work: &mut [c64],
              rwork: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::zhpsvx_(&(fact as c_char), &(uplo as c_char), &(n as c_int), &(nrhs as c_int),
                     ap.as_ptr() as *const _, afp.as_mut_ptr() as *mut _, ipiv.as_mut_ptr(),
                     b.as_ptr() as *const _, &(ldb as c_int), x.as_mut_ptr() as *mut _,
                     &(ldx as c_int), rcond.as_mut_ptr(), ferr.as_mut_ptr(), berr.as_mut_ptr(),
                     work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn sgeqrf(m: usize, n: usize, a: &mut [f32], lda: usize, tau: &mut [f32], work: &mut [f32],
              lwork: usize, info: &mut i32) {

    unsafe {
        ffi::sgeqrf_(&(m as c_int), &(n as c_int), a.as_mut_ptr(), &(lda as c_int),
                     tau.as_mut_ptr(), work.as_mut_ptr(), &(lwork as c_int), info)
    }
}

#[inline]
pub fn dgeqrf(m: usize, n: usize, a: &mut [f64], lda: usize, tau: &mut [f64], work: &mut [f64],
              lwork: usize, info: &mut i32) {

    unsafe {
        ffi::dgeqrf_(&(m as c_int), &(n as c_int), a.as_mut_ptr(), &(lda as c_int),
                     tau.as_mut_ptr(), work.as_mut_ptr(), &(lwork as c_int), info)
    }
}

#[inline]
pub fn cgeqrf(m: usize, n: usize, a: &mut [c32], lda: usize, tau: &mut [c32], work: &mut [c32],
              lwork: usize, info: &mut i32) {

    unsafe {
        ffi::cgeqrf_(&(m as c_int), &(n as c_int), a.as_mut_ptr() as *mut _, &(lda as c_int),
                     tau.as_mut_ptr() as *mut _, work.as_mut_ptr() as *mut _, &(lwork as c_int),
                     info)
    }
}

#[inline]
pub fn zgeqrf(m: usize, n: usize, a: &mut [c64], lda: usize, tau: &mut [c64], work: &mut [c64],
              lwork: usize, info: &mut i32) {

    unsafe {
        ffi::zgeqrf_(&(m as c_int), &(n as c_int), a.as_mut_ptr() as *mut _, &(lda as c_int),
                     tau.as_mut_ptr() as *mut _, work.as_mut_ptr() as *mut _, &(lwork as c_int),
                     info)
    }
}

#[inline]
pub fn sgeqpf(m: usize, n: usize, a: &mut [f32], lda: usize, jpvt: &mut [i32], tau: &mut [f32],
              work: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::sgeqpf_(&(m as c_int), &(n as c_int), a.as_mut_ptr(), &(lda as c_int),
                     jpvt.as_mut_ptr(), tau.as_mut_ptr(), work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dgeqpf(m: usize, n: usize, a: &mut [f64], lda: usize, jpvt: &mut [i32], tau: &mut [f64],
              work: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::dgeqpf_(&(m as c_int), &(n as c_int), a.as_mut_ptr(), &(lda as c_int),
                     jpvt.as_mut_ptr(), tau.as_mut_ptr(), work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn cgeqpf(m: usize, n: usize, a: &mut [c32], lda: usize, jpvt: &mut [i32], tau: &mut [c32],
              work: &mut [c32], rwork: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::cgeqpf_(&(m as c_int), &(n as c_int), a.as_mut_ptr() as *mut _, &(lda as c_int),
                     jpvt.as_mut_ptr(), tau.as_mut_ptr() as *mut _, work.as_mut_ptr() as *mut _,
                     rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zgeqpf(m: usize, n: usize, a: &mut [c64], lda: usize, jpvt: &mut [i32], tau: &mut [c64],
              work: &mut [c64], rwork: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::zgeqpf_(&(m as c_int), &(n as c_int), a.as_mut_ptr() as *mut _, &(lda as c_int),
                     jpvt.as_mut_ptr(), tau.as_mut_ptr() as *mut _, work.as_mut_ptr() as *mut _,
                     rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn sgeqp3(m: usize, n: usize, a: &mut [f32], lda: usize, jpvt: &mut [i32], tau: &mut [f32],
              work: &mut [f32], lwork: usize, info: &mut i32) {

    unsafe {
        ffi::sgeqp3_(&(m as c_int), &(n as c_int), a.as_mut_ptr(), &(lda as c_int),
                     jpvt.as_mut_ptr(), tau.as_mut_ptr(), work.as_mut_ptr(), &(lwork as c_int),
                     info)
    }
}

#[inline]
pub fn dgeqp3(m: usize, n: usize, a: &mut [f64], lda: usize, jpvt: &mut [i32], tau: &mut [f64],
              work: &mut [f64], lwork: usize, info: &mut i32) {

    unsafe {
        ffi::dgeqp3_(&(m as c_int), &(n as c_int), a.as_mut_ptr(), &(lda as c_int),
                     jpvt.as_mut_ptr(), tau.as_mut_ptr(), work.as_mut_ptr(), &(lwork as c_int),
                     info)
    }
}

#[inline]
pub fn cgeqp3(m: usize, n: usize, a: &mut [c32], lda: usize, jpvt: &mut [i32], tau: &mut [c32],
              work: &mut [c32], lwork: usize, rwork: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::cgeqp3_(&(m as c_int), &(n as c_int), a.as_mut_ptr() as *mut _, &(lda as c_int),
                     jpvt.as_mut_ptr(), tau.as_mut_ptr() as *mut _, work.as_mut_ptr() as *mut _,
                     &(lwork as c_int), rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zgeqp3(m: usize, n: usize, a: &mut [c64], lda: usize, jpvt: &mut [i32], tau: &mut [c64],
              work: &mut [c64], lwork: usize, rwork: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::zgeqp3_(&(m as c_int), &(n as c_int), a.as_mut_ptr() as *mut _, &(lda as c_int),
                     jpvt.as_mut_ptr(), tau.as_mut_ptr() as *mut _, work.as_mut_ptr() as *mut _,
                     &(lwork as c_int), rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn sorgqr(m: usize, n: usize, k: usize, a: &mut [f32], lda: usize, tau: &[f32],
              work: &mut [f32], lwork: usize, info: &mut i32) {

    unsafe {
        ffi::sorgqr_(&(m as c_int), &(n as c_int), &(k as c_int), a.as_mut_ptr(), &(lda as c_int),
                     tau.as_ptr(), work.as_mut_ptr(), &(lwork as c_int), info)
    }
}

#[inline]
pub fn dorgqr(m: usize, n: usize, k: usize, a: &mut [f64], lda: usize, tau: &[f64],
              work: &mut [f64], lwork: usize, info: &mut i32) {

    unsafe {
        ffi::dorgqr_(&(m as c_int), &(n as c_int), &(k as c_int), a.as_mut_ptr(), &(lda as c_int),
                     tau.as_ptr(), work.as_mut_ptr(), &(lwork as c_int), info)
    }
}

#[inline]
pub fn sormqr(side: u8, trans: u8, m: usize, n: usize, k: usize, a: &[f32], lda: usize,
              tau: &[f32], c: &mut [f32], ldc: usize, work: &mut [f32], lwork: usize,
              info: &mut i32) {

    unsafe {
        ffi::sormqr_(&(side as c_char), &(trans as c_char), &(m as c_int), &(n as c_int),
                     &(k as c_int), a.as_ptr(), &(lda as c_int), tau.as_ptr(), c.as_mut_ptr(),
                     &(ldc as c_int), work.as_mut_ptr(), &(lwork as c_int), info)
    }
}

#[inline]
pub fn dormqr(side: u8, trans: u8, m: usize, n: usize, k: usize, a: &[f64], lda: usize,
              tau: &[f64], c: &mut [f64], ldc: usize, work: &mut [f64], lwork: usize,
              info: &mut i32) {

    unsafe {
        ffi::dormqr_(&(side as c_char), &(trans as c_char), &(m as c_int), &(n as c_int),
                     &(k as c_int), a.as_ptr(), &(lda as c_int), tau.as_ptr(), c.as_mut_ptr(),
                     &(ldc as c_int), work.as_mut_ptr(), &(lwork as c_int), info)
    }
}

#[inline]
pub fn cungqr(m: usize, n: usize, k: usize, a: &mut [c32], lda: usize, tau: &[c32],
              work: &mut [c32], lwork: usize, info: &mut i32) {

    unsafe {
        ffi::cungqr_(&(m as c_int), &(n as c_int), &(k as c_int), a.as_mut_ptr() as *mut _,
                     &(lda as c_int), tau.as_ptr() as *const _, work.as_mut_ptr() as *mut _,
                     &(lwork as c_int), info)
    }
}

#[inline]
pub fn zungqr(m: usize, n: usize, k: usize, a: &mut [c64], lda: usize, tau: &[c64],
              work: &mut [c64], lwork: usize, info: &mut i32) {

    unsafe {
        ffi::zungqr_(&(m as c_int), &(n as c_int), &(k as c_int), a.as_mut_ptr() as *mut _,
                     &(lda as c_int), tau.as_ptr() as *const _, work.as_mut_ptr() as *mut _,
                     &(lwork as c_int), info)
    }
}

#[inline]
pub fn cunmqr(side: u8, trans: u8, m: usize, n: usize, k: usize, a: &[c32], lda: usize,
              tau: &[c32], c: &mut [c32], ldc: usize, work: &mut [c32], lwork: usize,
              info: &mut i32) {

    unsafe {
        ffi::cunmqr_(&(side as c_char), &(trans as c_char), &(m as c_int), &(n as c_int),
                     &(k as c_int), a.as_ptr() as *const _, &(lda as c_int),
                     tau.as_ptr() as *const _, c.as_mut_ptr() as *mut _, &(ldc as c_int),
                     work.as_mut_ptr() as *mut _, &(lwork as c_int), info)
    }
}

#[inline]
pub fn zunmqr(side: u8, trans: u8, m: usize, n: usize, k: usize, a: &[c64], lda: usize,
              tau: &[c64], c: &mut [c64], ldc: usize, work: &mut [c64], lwork: usize,
              info: &mut i32) {

    unsafe {
        ffi::zunmqr_(&(side as c_char), &(trans as c_char), &(m as c_int), &(n as c_int),
                     &(k as c_int), a.as_ptr() as *const _, &(lda as c_int),
                     tau.as_ptr() as *const _, c.as_mut_ptr() as *mut _, &(ldc as c_int),
                     work.as_mut_ptr() as *mut _, &(lwork as c_int), info)
    }
}

#[inline]
pub fn sgelqf(m: usize, n: usize, a: &mut [f32], lda: usize, tau: &mut [f32], work: &mut [f32],
              lwork: usize, info: &mut i32) {

    unsafe {
        ffi::sgelqf_(&(m as c_int), &(n as c_int), a.as_mut_ptr(), &(lda as c_int),
                     tau.as_mut_ptr(), work.as_mut_ptr(), &(lwork as c_int), info)
    }
}

#[inline]
pub fn dgelqf(m: usize, n: usize, a: &mut [f64], lda: usize, tau: &mut [f64], work: &mut [f64],
              lwork: usize, info: &mut i32) {

    unsafe {
        ffi::dgelqf_(&(m as c_int), &(n as c_int), a.as_mut_ptr(), &(lda as c_int),
                     tau.as_mut_ptr(), work.as_mut_ptr(), &(lwork as c_int), info)
    }
}

#[inline]
pub fn cgelqf(m: usize, n: usize, a: &mut [c32], lda: usize, tau: &mut [c32], work: &mut [c32],
              lwork: usize, info: &mut i32) {

    unsafe {
        ffi::cgelqf_(&(m as c_int), &(n as c_int), a.as_mut_ptr() as *mut _, &(lda as c_int),
                     tau.as_mut_ptr() as *mut _, work.as_mut_ptr() as *mut _, &(lwork as c_int),
                     info)
    }
}

#[inline]
pub fn zgelqf(m: usize, n: usize, a: &mut [c64], lda: usize, tau: &mut [c64], work: &mut [c64],
              lwork: usize, info: &mut i32) {

    unsafe {
        ffi::zgelqf_(&(m as c_int), &(n as c_int), a.as_mut_ptr() as *mut _, &(lda as c_int),
                     tau.as_mut_ptr() as *mut _, work.as_mut_ptr() as *mut _, &(lwork as c_int),
                     info)
    }
}

#[inline]
pub fn sorglq(m: usize, n: usize, k: usize, a: &mut [f32], lda: usize, tau: &[f32],
              work: &mut [f32], lwork: usize, info: &mut i32) {

    unsafe {
        ffi::sorglq_(&(m as c_int), &(n as c_int), &(k as c_int), a.as_mut_ptr(), &(lda as c_int),
                     tau.as_ptr(), work.as_mut_ptr(), &(lwork as c_int), info)
    }
}

#[inline]
pub fn dorglq(m: usize, n: usize, k: usize, a: &mut [f64], lda: usize, tau: &[f64],
              work: &mut [f64], lwork: usize, info: &mut i32) {

    unsafe {
        ffi::dorglq_(&(m as c_int), &(n as c_int), &(k as c_int), a.as_mut_ptr(), &(lda as c_int),
                     tau.as_ptr(), work.as_mut_ptr(), &(lwork as c_int), info)
    }
}

#[inline]
pub fn sormlq(side: u8, trans: u8, m: usize, n: usize, k: usize, a: &[f32], lda: usize,
              tau: &[f32], c: &mut [f32], ldc: usize, work: &mut [f32], lwork: usize,
              info: &mut i32) {

    unsafe {
        ffi::sormlq_(&(side as c_char), &(trans as c_char), &(m as c_int), &(n as c_int),
                     &(k as c_int), a.as_ptr(), &(lda as c_int), tau.as_ptr(), c.as_mut_ptr(),
                     &(ldc as c_int), work.as_mut_ptr(), &(lwork as c_int), info)
    }
}

#[inline]
pub fn dormlq(side: u8, trans: u8, m: usize, n: usize, k: usize, a: &[f64], lda: usize,
              tau: &[f64], c: &mut [f64], ldc: usize, work: &mut [f64], lwork: usize,
              info: &mut i32) {

    unsafe {
        ffi::dormlq_(&(side as c_char), &(trans as c_char), &(m as c_int), &(n as c_int),
                     &(k as c_int), a.as_ptr(), &(lda as c_int), tau.as_ptr(), c.as_mut_ptr(),
                     &(ldc as c_int), work.as_mut_ptr(), &(lwork as c_int), info)
    }
}

#[inline]
pub fn cunglq(m: usize, n: usize, k: usize, a: &mut [c32], lda: usize, tau: &[c32],
              work: &mut [c32], lwork: usize, info: &mut i32) {

    unsafe {
        ffi::cunglq_(&(m as c_int), &(n as c_int), &(k as c_int), a.as_mut_ptr() as *mut _,
                     &(lda as c_int), tau.as_ptr() as *const _, work.as_mut_ptr() as *mut _,
                     &(lwork as c_int), info)
    }
}

#[inline]
pub fn zunglq(m: usize, n: usize, k: usize, a: &mut [c64], lda: usize, tau: &[c64],
              work: &mut [c64], lwork: usize, info: &mut i32) {

    unsafe {
        ffi::zunglq_(&(m as c_int), &(n as c_int), &(k as c_int), a.as_mut_ptr() as *mut _,
                     &(lda as c_int), tau.as_ptr() as *const _, work.as_mut_ptr() as *mut _,
                     &(lwork as c_int), info)
    }
}

#[inline]
pub fn cunmlq(side: u8, trans: u8, m: usize, n: usize, k: usize, a: &[c32], lda: usize,
              tau: &[c32], c: &mut [c32], ldc: usize, work: &mut [c32], lwork: usize,
              info: &mut i32) {

    unsafe {
        ffi::cunmlq_(&(side as c_char), &(trans as c_char), &(m as c_int), &(n as c_int),
                     &(k as c_int), a.as_ptr() as *const _, &(lda as c_int),
                     tau.as_ptr() as *const _, c.as_mut_ptr() as *mut _, &(ldc as c_int),
                     work.as_mut_ptr() as *mut _, &(lwork as c_int), info)
    }
}

#[inline]
pub fn zunmlq(side: u8, trans: u8, m: usize, n: usize, k: usize, a: &[c64], lda: usize,
              tau: &[c64], c: &mut [c64], ldc: usize, work: &mut [c64], lwork: usize,
              info: &mut i32) {

    unsafe {
        ffi::zunmlq_(&(side as c_char), &(trans as c_char), &(m as c_int), &(n as c_int),
                     &(k as c_int), a.as_ptr() as *const _, &(lda as c_int),
                     tau.as_ptr() as *const _, c.as_mut_ptr() as *mut _, &(ldc as c_int),
                     work.as_mut_ptr() as *mut _, &(lwork as c_int), info)
    }
}

#[inline]
pub fn sgeqlf(m: usize, n: usize, a: &mut [f32], lda: usize, tau: &mut [f32], work: &mut [f32],
              lwork: usize, info: &mut i32) {

    unsafe {
        ffi::sgeqlf_(&(m as c_int), &(n as c_int), a.as_mut_ptr(), &(lda as c_int),
                     tau.as_mut_ptr(), work.as_mut_ptr(), &(lwork as c_int), info)
    }
}

#[inline]
pub fn dgeqlf(m: usize, n: usize, a: &mut [f64], lda: usize, tau: &mut [f64], work: &mut [f64],
              lwork: usize, info: &mut i32) {

    unsafe {
        ffi::dgeqlf_(&(m as c_int), &(n as c_int), a.as_mut_ptr(), &(lda as c_int),
                     tau.as_mut_ptr(), work.as_mut_ptr(), &(lwork as c_int), info)
    }
}

#[inline]
pub fn cgeqlf(m: usize, n: usize, a: &mut [c32], lda: usize, tau: &mut [c32], work: &mut [c32],
              lwork: usize, info: &mut i32) {

    unsafe {
        ffi::cgeqlf_(&(m as c_int), &(n as c_int), a.as_mut_ptr() as *mut _, &(lda as c_int),
                     tau.as_mut_ptr() as *mut _, work.as_mut_ptr() as *mut _, &(lwork as c_int),
                     info)
    }
}

#[inline]
pub fn zgeqlf(m: usize, n: usize, a: &mut [c64], lda: usize, tau: &mut [c64], work: &mut [c64],
              lwork: usize, info: &mut i32) {

    unsafe {
        ffi::zgeqlf_(&(m as c_int), &(n as c_int), a.as_mut_ptr() as *mut _, &(lda as c_int),
                     tau.as_mut_ptr() as *mut _, work.as_mut_ptr() as *mut _, &(lwork as c_int),
                     info)
    }
}

#[inline]
pub fn sorgql(m: usize, n: usize, k: usize, a: &mut [f32], lda: usize, tau: &[f32],
              work: &mut [f32], lwork: usize, info: &mut i32) {

    unsafe {
        ffi::sorgql_(&(m as c_int), &(n as c_int), &(k as c_int), a.as_mut_ptr(), &(lda as c_int),
                     tau.as_ptr(), work.as_mut_ptr(), &(lwork as c_int), info)
    }
}

#[inline]
pub fn dorgql(m: usize, n: usize, k: usize, a: &mut [f64], lda: usize, tau: &[f64],
              work: &mut [f64], lwork: usize, info: &mut i32) {

    unsafe {
        ffi::dorgql_(&(m as c_int), &(n as c_int), &(k as c_int), a.as_mut_ptr(), &(lda as c_int),
                     tau.as_ptr(), work.as_mut_ptr(), &(lwork as c_int), info)
    }
}

#[inline]
pub fn cungql(m: usize, n: usize, k: usize, a: &mut [c32], lda: usize, tau: &[c32],
              work: &mut [c32], lwork: usize, info: &mut i32) {

    unsafe {
        ffi::cungql_(&(m as c_int), &(n as c_int), &(k as c_int), a.as_mut_ptr() as *mut _,
                     &(lda as c_int), tau.as_ptr() as *const _, work.as_mut_ptr() as *mut _,
                     &(lwork as c_int), info)
    }
}

#[inline]
pub fn zungql(m: usize, n: usize, k: usize, a: &mut [c64], lda: usize, tau: &[c64],
              work: &mut [c64], lwork: usize, info: &mut i32) {

    unsafe {
        ffi::zungql_(&(m as c_int), &(n as c_int), &(k as c_int), a.as_mut_ptr() as *mut _,
                     &(lda as c_int), tau.as_ptr() as *const _, work.as_mut_ptr() as *mut _,
                     &(lwork as c_int), info)
    }
}

#[inline]
pub fn sormql(side: u8, trans: u8, m: usize, n: usize, k: usize, a: &[f32], lda: usize,
              tau: &[f32], c: &mut [f32], ldc: usize, work: &mut [f32], lwork: usize,
              info: &mut i32) {

    unsafe {
        ffi::sormql_(&(side as c_char), &(trans as c_char), &(m as c_int), &(n as c_int),
                     &(k as c_int), a.as_ptr(), &(lda as c_int), tau.as_ptr(), c.as_mut_ptr(),
                     &(ldc as c_int), work.as_mut_ptr(), &(lwork as c_int), info)
    }
}

#[inline]
pub fn dormql(side: u8, trans: u8, m: usize, n: usize, k: usize, a: &[f64], lda: usize,
              tau: &[f64], c: &mut [f64], ldc: usize, work: &mut [f64], lwork: usize,
              info: &mut i32) {

    unsafe {
        ffi::dormql_(&(side as c_char), &(trans as c_char), &(m as c_int), &(n as c_int),
                     &(k as c_int), a.as_ptr(), &(lda as c_int), tau.as_ptr(), c.as_mut_ptr(),
                     &(ldc as c_int), work.as_mut_ptr(), &(lwork as c_int), info)
    }
}

#[inline]
pub fn cunmql(side: u8, trans: u8, m: usize, n: usize, k: usize, a: &[c32], lda: usize,
              tau: &[c32], c: &mut [c32], ldc: usize, work: &mut [c32], lwork: usize,
              info: &mut i32) {

    unsafe {
        ffi::cunmql_(&(side as c_char), &(trans as c_char), &(m as c_int), &(n as c_int),
                     &(k as c_int), a.as_ptr() as *const _, &(lda as c_int),
                     tau.as_ptr() as *const _, c.as_mut_ptr() as *mut _, &(ldc as c_int),
                     work.as_mut_ptr() as *mut _, &(lwork as c_int), info)
    }
}

#[inline]
pub fn zunmql(side: u8, trans: u8, m: usize, n: usize, k: usize, a: &[c64], lda: usize,
              tau: &[c64], c: &mut [c64], ldc: usize, work: &mut [c64], lwork: usize,
              info: &mut i32) {

    unsafe {
        ffi::zunmql_(&(side as c_char), &(trans as c_char), &(m as c_int), &(n as c_int),
                     &(k as c_int), a.as_ptr() as *const _, &(lda as c_int),
                     tau.as_ptr() as *const _, c.as_mut_ptr() as *mut _, &(ldc as c_int),
                     work.as_mut_ptr() as *mut _, &(lwork as c_int), info)
    }
}

#[inline]
pub fn sgerqf(m: usize, n: usize, a: &mut [f32], lda: usize, tau: &mut [f32], work: &mut [f32],
              lwork: usize, info: &mut i32) {

    unsafe {
        ffi::sgerqf_(&(m as c_int), &(n as c_int), a.as_mut_ptr(), &(lda as c_int),
                     tau.as_mut_ptr(), work.as_mut_ptr(), &(lwork as c_int), info)
    }
}

#[inline]
pub fn dgerqf(m: usize, n: usize, a: &mut [f64], lda: usize, tau: &mut [f64], work: &mut [f64],
              lwork: usize, info: &mut i32) {

    unsafe {
        ffi::dgerqf_(&(m as c_int), &(n as c_int), a.as_mut_ptr(), &(lda as c_int),
                     tau.as_mut_ptr(), work.as_mut_ptr(), &(lwork as c_int), info)
    }
}

#[inline]
pub fn cgerqf(m: usize, n: usize, a: &mut [c32], lda: usize, tau: &mut [c32], work: &mut [c32],
              lwork: usize, info: &mut i32) {

    unsafe {
        ffi::cgerqf_(&(m as c_int), &(n as c_int), a.as_mut_ptr() as *mut _, &(lda as c_int),
                     tau.as_mut_ptr() as *mut _, work.as_mut_ptr() as *mut _, &(lwork as c_int),
                     info)
    }
}

#[inline]
pub fn zgerqf(m: usize, n: usize, a: &mut [c64], lda: usize, tau: &mut [c64], work: &mut [c64],
              lwork: usize, info: &mut i32) {

    unsafe {
        ffi::zgerqf_(&(m as c_int), &(n as c_int), a.as_mut_ptr() as *mut _, &(lda as c_int),
                     tau.as_mut_ptr() as *mut _, work.as_mut_ptr() as *mut _, &(lwork as c_int),
                     info)
    }
}

#[inline]
pub fn sorgrq(m: usize, n: usize, k: usize, a: &mut [f32], lda: usize, tau: &[f32],
              work: &mut [f32], lwork: usize, info: &mut i32) {

    unsafe {
        ffi::sorgrq_(&(m as c_int), &(n as c_int), &(k as c_int), a.as_mut_ptr(), &(lda as c_int),
                     tau.as_ptr(), work.as_mut_ptr(), &(lwork as c_int), info)
    }
}

#[inline]
pub fn dorgrq(m: usize, n: usize, k: usize, a: &mut [f64], lda: usize, tau: &[f64],
              work: &mut [f64], lwork: usize, info: &mut i32) {

    unsafe {
        ffi::dorgrq_(&(m as c_int), &(n as c_int), &(k as c_int), a.as_mut_ptr(), &(lda as c_int),
                     tau.as_ptr(), work.as_mut_ptr(), &(lwork as c_int), info)
    }
}

#[inline]
pub fn cungrq(m: usize, n: usize, k: usize, a: &mut [c32], lda: usize, tau: &[c32],
              work: &mut [c32], lwork: usize, info: &mut i32) {

    unsafe {
        ffi::cungrq_(&(m as c_int), &(n as c_int), &(k as c_int), a.as_mut_ptr() as *mut _,
                     &(lda as c_int), tau.as_ptr() as *const _, work.as_mut_ptr() as *mut _,
                     &(lwork as c_int), info)
    }
}

#[inline]
pub fn zungrq(m: usize, n: usize, k: usize, a: &mut [c64], lda: usize, tau: &[c64],
              work: &mut [c64], lwork: usize, info: &mut i32) {

    unsafe {
        ffi::zungrq_(&(m as c_int), &(n as c_int), &(k as c_int), a.as_mut_ptr() as *mut _,
                     &(lda as c_int), tau.as_ptr() as *const _, work.as_mut_ptr() as *mut _,
                     &(lwork as c_int), info)
    }
}

#[inline]
pub fn sormrq(side: u8, trans: u8, m: usize, n: usize, k: usize, a: &[f32], lda: usize,
              tau: &[f32], c: &mut [f32], ldc: usize, work: &mut [f32], lwork: usize,
              info: &mut i32) {

    unsafe {
        ffi::sormrq_(&(side as c_char), &(trans as c_char), &(m as c_int), &(n as c_int),
                     &(k as c_int), a.as_ptr(), &(lda as c_int), tau.as_ptr(), c.as_mut_ptr(),
                     &(ldc as c_int), work.as_mut_ptr(), &(lwork as c_int), info)
    }
}

#[inline]
pub fn dormrq(side: u8, trans: u8, m: usize, n: usize, k: usize, a: &[f64], lda: usize,
              tau: &[f64], c: &mut [f64], ldc: usize, work: &mut [f64], lwork: usize,
              info: &mut i32) {

    unsafe {
        ffi::dormrq_(&(side as c_char), &(trans as c_char), &(m as c_int), &(n as c_int),
                     &(k as c_int), a.as_ptr(), &(lda as c_int), tau.as_ptr(), c.as_mut_ptr(),
                     &(ldc as c_int), work.as_mut_ptr(), &(lwork as c_int), info)
    }
}

#[inline]
pub fn cunmrq(side: u8, trans: u8, m: usize, n: usize, k: usize, a: &[c32], lda: usize,
              tau: &[c32], c: &mut [c32], ldc: usize, work: &mut [c32], lwork: usize,
              info: &mut i32) {

    unsafe {
        ffi::cunmrq_(&(side as c_char), &(trans as c_char), &(m as c_int), &(n as c_int),
                     &(k as c_int), a.as_ptr() as *const _, &(lda as c_int),
                     tau.as_ptr() as *const _, c.as_mut_ptr() as *mut _, &(ldc as c_int),
                     work.as_mut_ptr() as *mut _, &(lwork as c_int), info)
    }
}

#[inline]
pub fn zunmrq(side: u8, trans: u8, m: usize, n: usize, k: usize, a: &[c64], lda: usize,
              tau: &[c64], c: &mut [c64], ldc: usize, work: &mut [c64], lwork: usize,
              info: &mut i32) {

    unsafe {
        ffi::zunmrq_(&(side as c_char), &(trans as c_char), &(m as c_int), &(n as c_int),
                     &(k as c_int), a.as_ptr() as *const _, &(lda as c_int),
                     tau.as_ptr() as *const _, c.as_mut_ptr() as *mut _, &(ldc as c_int),
                     work.as_mut_ptr() as *mut _, &(lwork as c_int), info)
    }
}

#[inline]
pub fn stzrzf(m: usize, n: usize, a: &mut [f32], lda: usize, tau: &mut [f32], work: &mut [f32],
              lwork: usize, info: &mut i32) {

    unsafe {
        ffi::stzrzf_(&(m as c_int), &(n as c_int), a.as_mut_ptr(), &(lda as c_int),
                     tau.as_mut_ptr(), work.as_mut_ptr(), &(lwork as c_int), info)
    }
}

#[inline]
pub fn dtzrzf(m: usize, n: usize, a: &mut [f64], lda: usize, tau: &mut [f64], work: &mut [f64],
              lwork: usize, info: &mut i32) {

    unsafe {
        ffi::dtzrzf_(&(m as c_int), &(n as c_int), a.as_mut_ptr(), &(lda as c_int),
                     tau.as_mut_ptr(), work.as_mut_ptr(), &(lwork as c_int), info)
    }
}

#[inline]
pub fn ctzrzf(m: usize, n: usize, a: &mut [c32], lda: usize, tau: &mut [c32], work: &mut [c32],
              lwork: usize, info: &mut i32) {

    unsafe {
        ffi::ctzrzf_(&(m as c_int), &(n as c_int), a.as_mut_ptr() as *mut _, &(lda as c_int),
                     tau.as_mut_ptr() as *mut _, work.as_mut_ptr() as *mut _, &(lwork as c_int),
                     info)
    }
}

#[inline]
pub fn ztzrzf(m: usize, n: usize, a: &mut [c64], lda: usize, tau: &mut [c64], work: &mut [c64],
              lwork: usize, info: &mut i32) {

    unsafe {
        ffi::ztzrzf_(&(m as c_int), &(n as c_int), a.as_mut_ptr() as *mut _, &(lda as c_int),
                     tau.as_mut_ptr() as *mut _, work.as_mut_ptr() as *mut _, &(lwork as c_int),
                     info)
    }
}

#[inline]
pub fn sormrz(side: u8, trans: u8, m: usize, n: usize, k: usize, l: &[i32], a: &[f32], lda: usize,
              tau: &[f32], c: &mut [f32], ldc: usize, work: &mut [f32], lwork: usize,
              info: &mut i32) {

    unsafe {
        ffi::sormrz_(&(side as c_char), &(trans as c_char), &(m as c_int), &(n as c_int),
                     &(k as c_int), l.as_ptr(), a.as_ptr(), &(lda as c_int), tau.as_ptr(),
                     c.as_mut_ptr(), &(ldc as c_int), work.as_mut_ptr(), &(lwork as c_int), info)
    }
}

#[inline]
pub fn dormrz(side: u8, trans: u8, m: usize, n: usize, k: usize, l: &[i32], a: &[f64], lda: usize,
              tau: &[f64], c: &mut [f64], ldc: usize, work: &mut [f64], lwork: usize,
              info: &mut i32) {

    unsafe {
        ffi::dormrz_(&(side as c_char), &(trans as c_char), &(m as c_int), &(n as c_int),
                     &(k as c_int), l.as_ptr(), a.as_ptr(), &(lda as c_int), tau.as_ptr(),
                     c.as_mut_ptr(), &(ldc as c_int), work.as_mut_ptr(), &(lwork as c_int), info)
    }
}

#[inline]
pub fn cunmrz(side: u8, trans: u8, m: usize, n: usize, k: usize, l: &[i32], a: &[c32], lda: usize,
              tau: &[c32], c: &mut [c32], ldc: usize, work: &mut [c32], lwork: usize,
              info: &mut i32) {

    unsafe {
        ffi::cunmrz_(&(side as c_char), &(trans as c_char), &(m as c_int), &(n as c_int),
                     &(k as c_int), l.as_ptr(), a.as_ptr() as *const _, &(lda as c_int),
                     tau.as_ptr() as *const _, c.as_mut_ptr() as *mut _, &(ldc as c_int),
                     work.as_mut_ptr() as *mut _, &(lwork as c_int), info)
    }
}

#[inline]
pub fn zunmrz(side: u8, trans: u8, m: usize, n: usize, k: usize, l: &[i32], a: &[c64], lda: usize,
              tau: &[c64], c: &mut [c64], ldc: usize, work: &mut [c64], lwork: usize,
              info: &mut i32) {

    unsafe {
        ffi::zunmrz_(&(side as c_char), &(trans as c_char), &(m as c_int), &(n as c_int),
                     &(k as c_int), l.as_ptr(), a.as_ptr() as *const _, &(lda as c_int),
                     tau.as_ptr() as *const _, c.as_mut_ptr() as *mut _, &(ldc as c_int),
                     work.as_mut_ptr() as *mut _, &(lwork as c_int), info)
    }
}

#[inline]
pub fn sggqrf(n: usize, m: usize, p: &[i32], a: &mut [f32], lda: usize, taua: &mut [f32],
              b: &mut [f32], ldb: usize, taub: &mut [f32], work: &mut [f32], lwork: usize,
              info: &mut i32) {

    unsafe {
        ffi::sggqrf_(&(n as c_int), &(m as c_int), p.as_ptr(), a.as_mut_ptr(), &(lda as c_int),
                     taua.as_mut_ptr(), b.as_mut_ptr(), &(ldb as c_int), taub.as_mut_ptr(),
                     work.as_mut_ptr(), &(lwork as c_int), info)
    }
}

#[inline]
pub fn dggqrf(n: usize, m: usize, p: &[i32], a: &mut [f64], lda: usize, taua: &mut [f64],
              b: &mut [f64], ldb: usize, taub: &mut [f64], work: &mut [f64], lwork: usize,
              info: &mut i32) {

    unsafe {
        ffi::dggqrf_(&(n as c_int), &(m as c_int), p.as_ptr(), a.as_mut_ptr(), &(lda as c_int),
                     taua.as_mut_ptr(), b.as_mut_ptr(), &(ldb as c_int), taub.as_mut_ptr(),
                     work.as_mut_ptr(), &(lwork as c_int), info)
    }
}

#[inline]
pub fn cggqrf(n: usize, m: usize, p: &[i32], a: &mut [c32], lda: usize, taua: &mut [c32],
              b: &mut [c32], ldb: usize, taub: &mut [c32], work: &mut [c32], lwork: usize,
              info: &mut i32) {

    unsafe {
        ffi::cggqrf_(&(n as c_int), &(m as c_int), p.as_ptr(), a.as_mut_ptr() as *mut _,
                     &(lda as c_int), taua.as_mut_ptr() as *mut _, b.as_mut_ptr() as *mut _,
                     &(ldb as c_int), taub.as_mut_ptr() as *mut _, work.as_mut_ptr() as *mut _,
                     &(lwork as c_int), info)
    }
}

#[inline]
pub fn zggqrf(n: usize, m: usize, p: &[i32], a: &mut [c64], lda: usize, taua: &mut [c64],
              b: &mut [c64], ldb: usize, taub: &mut [c64], work: &mut [c64], lwork: usize,
              info: &mut i32) {

    unsafe {
        ffi::zggqrf_(&(n as c_int), &(m as c_int), p.as_ptr(), a.as_mut_ptr() as *mut _,
                     &(lda as c_int), taua.as_mut_ptr() as *mut _, b.as_mut_ptr() as *mut _,
                     &(ldb as c_int), taub.as_mut_ptr() as *mut _, work.as_mut_ptr() as *mut _,
                     &(lwork as c_int), info)
    }
}

#[inline]
pub fn sggrqf(m: usize, p: &[i32], n: usize, a: &mut [f32], lda: usize, taua: &mut [f32],
              b: &mut [f32], ldb: usize, taub: &mut [f32], work: &mut [f32], lwork: usize,
              info: &mut i32) {

    unsafe {
        ffi::sggrqf_(&(m as c_int), p.as_ptr(), &(n as c_int), a.as_mut_ptr(), &(lda as c_int),
                     taua.as_mut_ptr(), b.as_mut_ptr(), &(ldb as c_int), taub.as_mut_ptr(),
                     work.as_mut_ptr(), &(lwork as c_int), info)
    }
}

#[inline]
pub fn dggrqf(m: usize, p: &[i32], n: usize, a: &mut [f64], lda: usize, taua: &mut [f64],
              b: &mut [f64], ldb: usize, taub: &mut [f64], work: &mut [f64], lwork: usize,
              info: &mut i32) {

    unsafe {
        ffi::dggrqf_(&(m as c_int), p.as_ptr(), &(n as c_int), a.as_mut_ptr(), &(lda as c_int),
                     taua.as_mut_ptr(), b.as_mut_ptr(), &(ldb as c_int), taub.as_mut_ptr(),
                     work.as_mut_ptr(), &(lwork as c_int), info)
    }
}

#[inline]
pub fn cggrqf(m: usize, p: &[i32], n: usize, a: &mut [c32], lda: usize, taua: &mut [c32],
              b: &mut [c32], ldb: usize, taub: &mut [c32], work: &mut [c32], lwork: usize,
              info: &mut i32) {

    unsafe {
        ffi::cggrqf_(&(m as c_int), p.as_ptr(), &(n as c_int), a.as_mut_ptr() as *mut _,
                     &(lda as c_int), taua.as_mut_ptr() as *mut _, b.as_mut_ptr() as *mut _,
                     &(ldb as c_int), taub.as_mut_ptr() as *mut _, work.as_mut_ptr() as *mut _,
                     &(lwork as c_int), info)
    }
}

#[inline]
pub fn zggrqf(m: usize, p: &[i32], n: usize, a: &mut [c64], lda: usize, taua: &mut [c64],
              b: &mut [c64], ldb: usize, taub: &mut [c64], work: &mut [c64], lwork: usize,
              info: &mut i32) {

    unsafe {
        ffi::zggrqf_(&(m as c_int), p.as_ptr(), &(n as c_int), a.as_mut_ptr() as *mut _,
                     &(lda as c_int), taua.as_mut_ptr() as *mut _, b.as_mut_ptr() as *mut _,
                     &(ldb as c_int), taub.as_mut_ptr() as *mut _, work.as_mut_ptr() as *mut _,
                     &(lwork as c_int), info)
    }
}

#[inline]
pub fn sgebrd(m: usize, n: usize, a: &mut [f32], lda: usize, d: &mut [f32], e: &mut [f32],
              tauq: &mut [f32], taup: &mut [f32], work: &mut [f32], lwork: usize, info: &mut i32) {

    unsafe {
        ffi::sgebrd_(&(m as c_int), &(n as c_int), a.as_mut_ptr(), &(lda as c_int), d.as_mut_ptr(),
                     e.as_mut_ptr(), tauq.as_mut_ptr(), taup.as_mut_ptr(), work.as_mut_ptr(),
                     &(lwork as c_int), info)
    }
}

#[inline]
pub fn dgebrd(m: usize, n: usize, a: &mut [f64], lda: usize, d: &mut [f64], e: &mut [f64],
              tauq: &mut [f64], taup: &mut [f64], work: &mut [f64], lwork: usize, info: &mut i32) {

    unsafe {
        ffi::dgebrd_(&(m as c_int), &(n as c_int), a.as_mut_ptr(), &(lda as c_int), d.as_mut_ptr(),
                     e.as_mut_ptr(), tauq.as_mut_ptr(), taup.as_mut_ptr(), work.as_mut_ptr(),
                     &(lwork as c_int), info)
    }
}

#[inline]
pub fn cgebrd(m: usize, n: usize, a: &mut [c32], lda: usize, d: &mut [f32], e: &mut [f32],
              tauq: &mut [c32], taup: &mut [c32], work: &mut [c32], lwork: usize, info: &mut i32) {

    unsafe {
        ffi::cgebrd_(&(m as c_int), &(n as c_int), a.as_mut_ptr() as *mut _, &(lda as c_int),
                     d.as_mut_ptr(), e.as_mut_ptr(), tauq.as_mut_ptr() as *mut _,
                     taup.as_mut_ptr() as *mut _, work.as_mut_ptr() as *mut _, &(lwork as c_int),
                     info)
    }
}

#[inline]
pub fn zgebrd(m: usize, n: usize, a: &mut [c64], lda: usize, d: &mut [f64], e: &mut [f64],
              tauq: &mut [c64], taup: &mut [c64], work: &mut [c64], lwork: usize, info: &mut i32) {

    unsafe {
        ffi::zgebrd_(&(m as c_int), &(n as c_int), a.as_mut_ptr() as *mut _, &(lda as c_int),
                     d.as_mut_ptr(), e.as_mut_ptr(), tauq.as_mut_ptr() as *mut _,
                     taup.as_mut_ptr() as *mut _, work.as_mut_ptr() as *mut _, &(lwork as c_int),
                     info)
    }
}

#[inline]
pub fn sgbbrd(vect: u8, m: usize, n: usize, ncc: &[i32], kl: usize, ku: usize, ab: &mut [f32],
              ldab: usize, d: &mut [f32], e: &mut [f32], q: &mut [f32], ldq: usize, pt: &mut [f32],
              ldpt: usize, c: &mut [f32], ldc: usize, work: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::sgbbrd_(&(vect as c_char), &(m as c_int), &(n as c_int), ncc.as_ptr(), &(kl as c_int),
                     &(ku as c_int), ab.as_mut_ptr(), &(ldab as c_int), d.as_mut_ptr(),
                     e.as_mut_ptr(), q.as_mut_ptr(), &(ldq as c_int), pt.as_mut_ptr(),
                     &(ldpt as c_int), c.as_mut_ptr(), &(ldc as c_int), work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dgbbrd(vect: u8, m: usize, n: usize, ncc: &[i32], kl: usize, ku: usize, ab: &mut [f64],
              ldab: usize, d: &mut [f64], e: &mut [f64], q: &mut [f64], ldq: usize, pt: &mut [f64],
              ldpt: usize, c: &mut [f64], ldc: usize, work: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::dgbbrd_(&(vect as c_char), &(m as c_int), &(n as c_int), ncc.as_ptr(), &(kl as c_int),
                     &(ku as c_int), ab.as_mut_ptr(), &(ldab as c_int), d.as_mut_ptr(),
                     e.as_mut_ptr(), q.as_mut_ptr(), &(ldq as c_int), pt.as_mut_ptr(),
                     &(ldpt as c_int), c.as_mut_ptr(), &(ldc as c_int), work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn cgbbrd(vect: u8, m: usize, n: usize, ncc: &[i32], kl: usize, ku: usize, ab: &mut [c32],
              ldab: usize, d: &mut [f32], e: &mut [f32], q: &mut [c32], ldq: usize, pt: &mut [c32],
              ldpt: usize, c: &mut [c32], ldc: usize, work: &mut [c32], rwork: &mut [f32],
              info: &mut i32) {

    unsafe {
        ffi::cgbbrd_(&(vect as c_char), &(m as c_int), &(n as c_int), ncc.as_ptr(), &(kl as c_int),
                     &(ku as c_int), ab.as_mut_ptr() as *mut _, &(ldab as c_int), d.as_mut_ptr(),
                     e.as_mut_ptr(), q.as_mut_ptr() as *mut _, &(ldq as c_int),
                     pt.as_mut_ptr() as *mut _, &(ldpt as c_int), c.as_mut_ptr() as *mut _,
                     &(ldc as c_int), work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zgbbrd(vect: u8, m: usize, n: usize, ncc: &[i32], kl: usize, ku: usize, ab: &mut [c64],
              ldab: usize, d: &mut [f64], e: &mut [f64], q: &mut [c64], ldq: usize, pt: &mut [c64],
              ldpt: usize, c: &mut [c64], ldc: usize, work: &mut [c64], rwork: &mut [f64],
              info: &mut i32) {

    unsafe {
        ffi::zgbbrd_(&(vect as c_char), &(m as c_int), &(n as c_int), ncc.as_ptr(), &(kl as c_int),
                     &(ku as c_int), ab.as_mut_ptr() as *mut _, &(ldab as c_int), d.as_mut_ptr(),
                     e.as_mut_ptr(), q.as_mut_ptr() as *mut _, &(ldq as c_int),
                     pt.as_mut_ptr() as *mut _, &(ldpt as c_int), c.as_mut_ptr() as *mut _,
                     &(ldc as c_int), work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn sorgbr(vect: u8, m: usize, n: usize, k: usize, a: &mut [f32], lda: usize, tau: &[f32],
              work: &mut [f32], lwork: usize, info: &mut i32) {

    unsafe {
        ffi::sorgbr_(&(vect as c_char), &(m as c_int), &(n as c_int), &(k as c_int),
                     a.as_mut_ptr(), &(lda as c_int), tau.as_ptr(), work.as_mut_ptr(),
                     &(lwork as c_int), info)
    }
}

#[inline]
pub fn dorgbr(vect: u8, m: usize, n: usize, k: usize, a: &mut [f64], lda: usize, tau: &[f64],
              work: &mut [f64], lwork: usize, info: &mut i32) {

    unsafe {
        ffi::dorgbr_(&(vect as c_char), &(m as c_int), &(n as c_int), &(k as c_int),
                     a.as_mut_ptr(), &(lda as c_int), tau.as_ptr(), work.as_mut_ptr(),
                     &(lwork as c_int), info)
    }
}

#[inline]
pub fn sormbr(vect: u8, side: u8, trans: u8, m: usize, n: usize, k: usize, a: &[f32], lda: usize,
              tau: &[f32], c: &mut [f32], ldc: usize, work: &mut [f32], lwork: usize,
              info: &mut i32) {

    unsafe {
        ffi::sormbr_(&(vect as c_char), &(side as c_char), &(trans as c_char), &(m as c_int),
                     &(n as c_int), &(k as c_int), a.as_ptr(), &(lda as c_int), tau.as_ptr(),
                     c.as_mut_ptr(), &(ldc as c_int), work.as_mut_ptr(), &(lwork as c_int), info)
    }
}

#[inline]
pub fn dormbr(vect: u8, side: u8, trans: u8, m: usize, n: usize, k: usize, a: &[f64], lda: usize,
              tau: &[f64], c: &mut [f64], ldc: usize, work: &mut [f64], lwork: usize,
              info: &mut i32) {

    unsafe {
        ffi::dormbr_(&(vect as c_char), &(side as c_char), &(trans as c_char), &(m as c_int),
                     &(n as c_int), &(k as c_int), a.as_ptr(), &(lda as c_int), tau.as_ptr(),
                     c.as_mut_ptr(), &(ldc as c_int), work.as_mut_ptr(), &(lwork as c_int), info)
    }
}

#[inline]
pub fn cungbr(vect: u8, m: usize, n: usize, k: usize, a: &mut [c32], lda: usize, tau: &[c32],
              work: &mut [c32], lwork: usize, info: &mut i32) {

    unsafe {
        ffi::cungbr_(&(vect as c_char), &(m as c_int), &(n as c_int), &(k as c_int),
                     a.as_mut_ptr() as *mut _, &(lda as c_int), tau.as_ptr() as *const _,
                     work.as_mut_ptr() as *mut _, &(lwork as c_int), info)
    }
}

#[inline]
pub fn zungbr(vect: u8, m: usize, n: usize, k: usize, a: &mut [c64], lda: usize, tau: &[c64],
              work: &mut [c64], lwork: usize, info: &mut i32) {

    unsafe {
        ffi::zungbr_(&(vect as c_char), &(m as c_int), &(n as c_int), &(k as c_int),
                     a.as_mut_ptr() as *mut _, &(lda as c_int), tau.as_ptr() as *const _,
                     work.as_mut_ptr() as *mut _, &(lwork as c_int), info)
    }
}

#[inline]
pub fn cunmbr(vect: u8, side: u8, trans: u8, m: usize, n: usize, k: usize, a: &[c32], lda: usize,
              tau: &[c32], c: &mut [c32], ldc: usize, work: &mut [c32], lwork: usize,
              info: &mut i32) {

    unsafe {
        ffi::cunmbr_(&(vect as c_char), &(side as c_char), &(trans as c_char), &(m as c_int),
                     &(n as c_int), &(k as c_int), a.as_ptr() as *const _, &(lda as c_int),
                     tau.as_ptr() as *const _, c.as_mut_ptr() as *mut _, &(ldc as c_int),
                     work.as_mut_ptr() as *mut _, &(lwork as c_int), info)
    }
}

#[inline]
pub fn zunmbr(vect: u8, side: u8, trans: u8, m: usize, n: usize, k: usize, a: &[c64], lda: usize,
              tau: &[c64], c: &mut [c64], ldc: usize, work: &mut [c64], lwork: usize,
              info: &mut i32) {

    unsafe {
        ffi::zunmbr_(&(vect as c_char), &(side as c_char), &(trans as c_char), &(m as c_int),
                     &(n as c_int), &(k as c_int), a.as_ptr() as *const _, &(lda as c_int),
                     tau.as_ptr() as *const _, c.as_mut_ptr() as *mut _, &(ldc as c_int),
                     work.as_mut_ptr() as *mut _, &(lwork as c_int), info)
    }
}

#[inline]
pub fn sbdsqr(uplo: u8, n: usize, ncvt: &[i32], nru: &[i32], ncc: &[i32], d: &mut [f32],
              e: &mut [f32], vt: &mut [f32], ldvt: usize, u: &mut [f32], ldu: usize, c: &mut [f32],
              ldc: usize, work: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::sbdsqr_(&(uplo as c_char), &(n as c_int), ncvt.as_ptr(), nru.as_ptr(), ncc.as_ptr(),
                     d.as_mut_ptr(), e.as_mut_ptr(), vt.as_mut_ptr(), &(ldvt as c_int),
                     u.as_mut_ptr(), &(ldu as c_int), c.as_mut_ptr(), &(ldc as c_int),
                     work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dbdsqr(uplo: u8, n: usize, ncvt: &[i32], nru: &[i32], ncc: &[i32], d: &mut [f64],
              e: &mut [f64], vt: &mut [f64], ldvt: usize, u: &mut [f64], ldu: usize, c: &mut [f64],
              ldc: usize, work: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::dbdsqr_(&(uplo as c_char), &(n as c_int), ncvt.as_ptr(), nru.as_ptr(), ncc.as_ptr(),
                     d.as_mut_ptr(), e.as_mut_ptr(), vt.as_mut_ptr(), &(ldvt as c_int),
                     u.as_mut_ptr(), &(ldu as c_int), c.as_mut_ptr(), &(ldc as c_int),
                     work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn cbdsqr(uplo: u8, n: usize, ncvt: &[i32], nru: &[i32], ncc: &[i32], d: &mut [f32],
              e: &mut [f32], vt: &mut [c32], ldvt: usize, u: &mut [c32], ldu: usize, c: &mut [c32],
              ldc: usize, rwork: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::cbdsqr_(&(uplo as c_char), &(n as c_int), ncvt.as_ptr(), nru.as_ptr(), ncc.as_ptr(),
                     d.as_mut_ptr(), e.as_mut_ptr(), vt.as_mut_ptr() as *mut _, &(ldvt as c_int),
                     u.as_mut_ptr() as *mut _, &(ldu as c_int), c.as_mut_ptr() as *mut _,
                     &(ldc as c_int), rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zbdsqr(uplo: u8, n: usize, ncvt: &[i32], nru: &[i32], ncc: &[i32], d: &mut [f64],
              e: &mut [f64], vt: &mut [c64], ldvt: usize, u: &mut [c64], ldu: usize, c: &mut [c64],
              ldc: usize, rwork: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::zbdsqr_(&(uplo as c_char), &(n as c_int), ncvt.as_ptr(), nru.as_ptr(), ncc.as_ptr(),
                     d.as_mut_ptr(), e.as_mut_ptr(), vt.as_mut_ptr() as *mut _, &(ldvt as c_int),
                     u.as_mut_ptr() as *mut _, &(ldu as c_int), c.as_mut_ptr() as *mut _,
                     &(ldc as c_int), rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn sbdsdc(uplo: u8, compq: u8, n: usize, d: &mut [f32], e: &mut [f32], u: &mut [f32],
              ldu: usize, vt: &mut [f32], ldvt: usize, q: &mut [f32], iq: &mut [i32],
              work: &mut [f32], iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::sbdsdc_(&(uplo as c_char), &(compq as c_char), &(n as c_int), d.as_mut_ptr(),
                     e.as_mut_ptr(), u.as_mut_ptr(), &(ldu as c_int), vt.as_mut_ptr(),
                     &(ldvt as c_int), q.as_mut_ptr(), iq.as_mut_ptr(), work.as_mut_ptr(),
                     iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dbdsdc(uplo: u8, compq: u8, n: usize, d: &mut [f64], e: &mut [f64], u: &mut [f64],
              ldu: usize, vt: &mut [f64], ldvt: usize, q: &mut [f64], iq: &mut [i32],
              work: &mut [f64], iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::dbdsdc_(&(uplo as c_char), &(compq as c_char), &(n as c_int), d.as_mut_ptr(),
                     e.as_mut_ptr(), u.as_mut_ptr(), &(ldu as c_int), vt.as_mut_ptr(),
                     &(ldvt as c_int), q.as_mut_ptr(), iq.as_mut_ptr(), work.as_mut_ptr(),
                     iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn ssytrd(uplo: u8, n: usize, a: &mut [f32], lda: usize, d: &mut [f32], e: &mut [f32],
              tau: &mut [f32], work: &mut [f32], lwork: usize, info: &mut i32) {

    unsafe {
        ffi::ssytrd_(&(uplo as c_char), &(n as c_int), a.as_mut_ptr(), &(lda as c_int),
                     d.as_mut_ptr(), e.as_mut_ptr(), tau.as_mut_ptr(), work.as_mut_ptr(),
                     &(lwork as c_int), info)
    }
}

#[inline]
pub fn dsytrd(uplo: u8, n: usize, a: &mut [f64], lda: usize, d: &mut [f64], e: &mut [f64],
              tau: &mut [f64], work: &mut [f64], lwork: usize, info: &mut i32) {

    unsafe {
        ffi::dsytrd_(&(uplo as c_char), &(n as c_int), a.as_mut_ptr(), &(lda as c_int),
                     d.as_mut_ptr(), e.as_mut_ptr(), tau.as_mut_ptr(), work.as_mut_ptr(),
                     &(lwork as c_int), info)
    }
}

#[inline]
pub fn sorgtr(uplo: u8, n: usize, a: &mut [f32], lda: usize, tau: &[f32], work: &mut [f32],
              lwork: usize, info: &mut i32) {

    unsafe {
        ffi::sorgtr_(&(uplo as c_char), &(n as c_int), a.as_mut_ptr(), &(lda as c_int),
                     tau.as_ptr(), work.as_mut_ptr(), &(lwork as c_int), info)
    }
}

#[inline]
pub fn dorgtr(uplo: u8, n: usize, a: &mut [f64], lda: usize, tau: &[f64], work: &mut [f64],
              lwork: usize, info: &mut i32) {

    unsafe {
        ffi::dorgtr_(&(uplo as c_char), &(n as c_int), a.as_mut_ptr(), &(lda as c_int),
                     tau.as_ptr(), work.as_mut_ptr(), &(lwork as c_int), info)
    }
}

#[inline]
pub fn sormtr(side: u8, uplo: u8, trans: u8, m: usize, n: usize, a: &[f32], lda: usize,
              tau: &[f32], c: &mut [f32], ldc: usize, work: &mut [f32], lwork: usize,
              info: &mut i32) {

    unsafe {
        ffi::sormtr_(&(side as c_char), &(uplo as c_char), &(trans as c_char), &(m as c_int),
                     &(n as c_int), a.as_ptr(), &(lda as c_int), tau.as_ptr(), c.as_mut_ptr(),
                     &(ldc as c_int), work.as_mut_ptr(), &(lwork as c_int), info)
    }
}

#[inline]
pub fn dormtr(side: u8, uplo: u8, trans: u8, m: usize, n: usize, a: &[f64], lda: usize,
              tau: &[f64], c: &mut [f64], ldc: usize, work: &mut [f64], lwork: usize,
              info: &mut i32) {

    unsafe {
        ffi::dormtr_(&(side as c_char), &(uplo as c_char), &(trans as c_char), &(m as c_int),
                     &(n as c_int), a.as_ptr(), &(lda as c_int), tau.as_ptr(), c.as_mut_ptr(),
                     &(ldc as c_int), work.as_mut_ptr(), &(lwork as c_int), info)
    }
}

#[inline]
pub fn chetrd(uplo: u8, n: usize, a: &mut [c32], lda: usize, d: &mut [f32], e: &mut [f32],
              tau: &mut [c32], work: &mut [c32], lwork: usize, info: &mut i32) {

    unsafe {
        ffi::chetrd_(&(uplo as c_char), &(n as c_int), a.as_mut_ptr() as *mut _, &(lda as c_int),
                     d.as_mut_ptr(), e.as_mut_ptr(), tau.as_mut_ptr() as *mut _,
                     work.as_mut_ptr() as *mut _, &(lwork as c_int), info)
    }
}

#[inline]
pub fn zhetrd(uplo: u8, n: usize, a: &mut [c64], lda: usize, d: &mut [f64], e: &mut [f64],
              tau: &mut [c64], work: &mut [c64], lwork: usize, info: &mut i32) {

    unsafe {
        ffi::zhetrd_(&(uplo as c_char), &(n as c_int), a.as_mut_ptr() as *mut _, &(lda as c_int),
                     d.as_mut_ptr(), e.as_mut_ptr(), tau.as_mut_ptr() as *mut _,
                     work.as_mut_ptr() as *mut _, &(lwork as c_int), info)
    }
}

#[inline]
pub fn cungtr(uplo: u8, n: usize, a: &mut [c32], lda: usize, tau: &[c32], work: &mut [c32],
              lwork: usize, info: &mut i32) {

    unsafe {
        ffi::cungtr_(&(uplo as c_char), &(n as c_int), a.as_mut_ptr() as *mut _, &(lda as c_int),
                     tau.as_ptr() as *const _, work.as_mut_ptr() as *mut _, &(lwork as c_int),
                     info)
    }
}

#[inline]
pub fn zungtr(uplo: u8, n: usize, a: &mut [c64], lda: usize, tau: &[c64], work: &mut [c64],
              lwork: usize, info: &mut i32) {

    unsafe {
        ffi::zungtr_(&(uplo as c_char), &(n as c_int), a.as_mut_ptr() as *mut _, &(lda as c_int),
                     tau.as_ptr() as *const _, work.as_mut_ptr() as *mut _, &(lwork as c_int),
                     info)
    }
}

#[inline]
pub fn cunmtr(side: u8, uplo: u8, trans: u8, m: usize, n: usize, a: &[c32], lda: usize,
              tau: &[c32], c: &mut [c32], ldc: usize, work: &mut [c32], lwork: usize,
              info: &mut i32) {

    unsafe {
        ffi::cunmtr_(&(side as c_char), &(uplo as c_char), &(trans as c_char), &(m as c_int),
                     &(n as c_int), a.as_ptr() as *const _, &(lda as c_int),
                     tau.as_ptr() as *const _, c.as_mut_ptr() as *mut _, &(ldc as c_int),
                     work.as_mut_ptr() as *mut _, &(lwork as c_int), info)
    }
}

#[inline]
pub fn zunmtr(side: u8, uplo: u8, trans: u8, m: usize, n: usize, a: &[c64], lda: usize,
              tau: &[c64], c: &mut [c64], ldc: usize, work: &mut [c64], lwork: usize,
              info: &mut i32) {

    unsafe {
        ffi::zunmtr_(&(side as c_char), &(uplo as c_char), &(trans as c_char), &(m as c_int),
                     &(n as c_int), a.as_ptr() as *const _, &(lda as c_int),
                     tau.as_ptr() as *const _, c.as_mut_ptr() as *mut _, &(ldc as c_int),
                     work.as_mut_ptr() as *mut _, &(lwork as c_int), info)
    }
}

#[inline]
pub fn ssptrd(uplo: u8, n: usize, ap: &mut [f32], d: &mut [f32], e: &mut [f32], tau: &mut [f32],
              info: &mut i32) {

    unsafe {
        ffi::ssptrd_(&(uplo as c_char), &(n as c_int), ap.as_mut_ptr(), d.as_mut_ptr(),
                     e.as_mut_ptr(), tau.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dsptrd(uplo: u8, n: usize, ap: &mut [f64], d: &mut [f64], e: &mut [f64], tau: &mut [f64],
              info: &mut i32) {

    unsafe {
        ffi::dsptrd_(&(uplo as c_char), &(n as c_int), ap.as_mut_ptr(), d.as_mut_ptr(),
                     e.as_mut_ptr(), tau.as_mut_ptr(), info)
    }
}

#[inline]
pub fn sopgtr(uplo: u8, n: usize, ap: &[f32], tau: &[f32], q: &mut [f32], ldq: usize,
              work: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::sopgtr_(&(uplo as c_char), &(n as c_int), ap.as_ptr(), tau.as_ptr(), q.as_mut_ptr(),
                     &(ldq as c_int), work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dopgtr(uplo: u8, n: usize, ap: &[f64], tau: &[f64], q: &mut [f64], ldq: usize,
              work: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::dopgtr_(&(uplo as c_char), &(n as c_int), ap.as_ptr(), tau.as_ptr(), q.as_mut_ptr(),
                     &(ldq as c_int), work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn sopmtr(side: u8, uplo: u8, trans: u8, m: usize, n: usize, ap: &[f32], tau: &[f32],
              c: &mut [f32], ldc: usize, work: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::sopmtr_(&(side as c_char), &(uplo as c_char), &(trans as c_char), &(m as c_int),
                     &(n as c_int), ap.as_ptr(), tau.as_ptr(), c.as_mut_ptr(), &(ldc as c_int),
                     work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dopmtr(side: u8, uplo: u8, trans: u8, m: usize, n: usize, ap: &[f64], tau: &[f64],
              c: &mut [f64], ldc: usize, work: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::dopmtr_(&(side as c_char), &(uplo as c_char), &(trans as c_char), &(m as c_int),
                     &(n as c_int), ap.as_ptr(), tau.as_ptr(), c.as_mut_ptr(), &(ldc as c_int),
                     work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn chptrd(uplo: u8, n: usize, ap: &mut [c32], d: &mut [f32], e: &mut [f32], tau: &mut [c32],
              info: &mut i32) {

    unsafe {
        ffi::chptrd_(&(uplo as c_char), &(n as c_int), ap.as_mut_ptr() as *mut _, d.as_mut_ptr(),
                     e.as_mut_ptr(), tau.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn zhptrd(uplo: u8, n: usize, ap: &mut [c64], d: &mut [f64], e: &mut [f64], tau: &mut [c64],
              info: &mut i32) {

    unsafe {
        ffi::zhptrd_(&(uplo as c_char), &(n as c_int), ap.as_mut_ptr() as *mut _, d.as_mut_ptr(),
                     e.as_mut_ptr(), tau.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn cupgtr(uplo: u8, n: usize, ap: &[c32], tau: &[c32], q: &mut [c32], ldq: usize,
              work: &mut [c32], info: &mut i32) {

    unsafe {
        ffi::cupgtr_(&(uplo as c_char), &(n as c_int), ap.as_ptr() as *const _,
                     tau.as_ptr() as *const _, q.as_mut_ptr() as *mut _, &(ldq as c_int),
                     work.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn zupgtr(uplo: u8, n: usize, ap: &[c64], tau: &[c64], q: &mut [c64], ldq: usize,
              work: &mut [c64], info: &mut i32) {

    unsafe {
        ffi::zupgtr_(&(uplo as c_char), &(n as c_int), ap.as_ptr() as *const _,
                     tau.as_ptr() as *const _, q.as_mut_ptr() as *mut _, &(ldq as c_int),
                     work.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn cupmtr(side: u8, uplo: u8, trans: u8, m: usize, n: usize, ap: &[c32], tau: &[c32],
              c: &mut [c32], ldc: usize, work: &mut [c32], info: &mut i32) {

    unsafe {
        ffi::cupmtr_(&(side as c_char), &(uplo as c_char), &(trans as c_char), &(m as c_int),
                     &(n as c_int), ap.as_ptr() as *const _, tau.as_ptr() as *const _,
                     c.as_mut_ptr() as *mut _, &(ldc as c_int), work.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn zupmtr(side: u8, uplo: u8, trans: u8, m: usize, n: usize, ap: &[c64], tau: &[c64],
              c: &mut [c64], ldc: usize, work: &mut [c64], info: &mut i32) {

    unsafe {
        ffi::zupmtr_(&(side as c_char), &(uplo as c_char), &(trans as c_char), &(m as c_int),
                     &(n as c_int), ap.as_ptr() as *const _, tau.as_ptr() as *const _,
                     c.as_mut_ptr() as *mut _, &(ldc as c_int), work.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn ssbtrd(vect: u8, uplo: u8, n: usize, kd: usize, ab: &mut [f32], ldab: usize, d: &mut [f32],
              e: &mut [f32], q: &mut [f32], ldq: usize, work: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::ssbtrd_(&(vect as c_char), &(uplo as c_char), &(n as c_int), &(kd as c_int),
                     ab.as_mut_ptr(), &(ldab as c_int), d.as_mut_ptr(), e.as_mut_ptr(),
                     q.as_mut_ptr(), &(ldq as c_int), work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dsbtrd(vect: u8, uplo: u8, n: usize, kd: usize, ab: &mut [f64], ldab: usize, d: &mut [f64],
              e: &mut [f64], q: &mut [f64], ldq: usize, work: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::dsbtrd_(&(vect as c_char), &(uplo as c_char), &(n as c_int), &(kd as c_int),
                     ab.as_mut_ptr(), &(ldab as c_int), d.as_mut_ptr(), e.as_mut_ptr(),
                     q.as_mut_ptr(), &(ldq as c_int), work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn chbtrd(vect: u8, uplo: u8, n: usize, kd: usize, ab: &mut [c32], ldab: usize, d: &mut [f32],
              e: &mut [f32], q: &mut [c32], ldq: usize, work: &mut [c32], info: &mut i32) {

    unsafe {
        ffi::chbtrd_(&(vect as c_char), &(uplo as c_char), &(n as c_int), &(kd as c_int),
                     ab.as_mut_ptr() as *mut _, &(ldab as c_int), d.as_mut_ptr(), e.as_mut_ptr(),
                     q.as_mut_ptr() as *mut _, &(ldq as c_int), work.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn zhbtrd(vect: u8, uplo: u8, n: usize, kd: usize, ab: &mut [c64], ldab: usize, d: &mut [f64],
              e: &mut [f64], q: &mut [c64], ldq: usize, work: &mut [c64], info: &mut i32) {

    unsafe {
        ffi::zhbtrd_(&(vect as c_char), &(uplo as c_char), &(n as c_int), &(kd as c_int),
                     ab.as_mut_ptr() as *mut _, &(ldab as c_int), d.as_mut_ptr(), e.as_mut_ptr(),
                     q.as_mut_ptr() as *mut _, &(ldq as c_int), work.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn ssterf(n: usize, d: &mut [f32], e: &mut [f32], info: &mut i32) {
    unsafe {
        ffi::ssterf_(&(n as c_int), d.as_mut_ptr(), e.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dsterf(n: usize, d: &mut [f64], e: &mut [f64], info: &mut i32) {
    unsafe {
        ffi::dsterf_(&(n as c_int), d.as_mut_ptr(), e.as_mut_ptr(), info)
    }
}

#[inline]
pub fn ssteqr(compz: u8, n: usize, d: &mut [f32], e: &mut [f32], z: &mut [f32], ldz: usize,
              work: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::ssteqr_(&(compz as c_char), &(n as c_int), d.as_mut_ptr(), e.as_mut_ptr(),
                     z.as_mut_ptr(), &(ldz as c_int), work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dsteqr(compz: u8, n: usize, d: &mut [f64], e: &mut [f64], z: &mut [f64], ldz: usize,
              work: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::dsteqr_(&(compz as c_char), &(n as c_int), d.as_mut_ptr(), e.as_mut_ptr(),
                     z.as_mut_ptr(), &(ldz as c_int), work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn csteqr(compz: u8, n: usize, d: &mut [f32], e: &mut [f32], z: &mut [c32], ldz: usize,
              work: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::csteqr_(&(compz as c_char), &(n as c_int), d.as_mut_ptr(), e.as_mut_ptr(),
                     z.as_mut_ptr() as *mut _, &(ldz as c_int), work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zsteqr(compz: u8, n: usize, d: &mut [f64], e: &mut [f64], z: &mut [c64], ldz: usize,
              work: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::zsteqr_(&(compz as c_char), &(n as c_int), d.as_mut_ptr(), e.as_mut_ptr(),
                     z.as_mut_ptr() as *mut _, &(ldz as c_int), work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn sstemr(jobz: u8, range: u8, n: usize, d: &mut [f32], e: &mut [f32], vl: &[f32], vu: &[f32],
              il: &[i32], iu: &[i32], m: &mut u32, w: &mut [f32], z: &mut [f32], ldz: usize,
              nzc: &[i32], isuppz: &mut [i32], tryrac: &mut [i32], work: &mut [f32], lwork: usize,
              iwork: &mut [i32], liwork: &[i32], info: &mut i32) {

    unsafe {
        ffi::sstemr_(&(jobz as c_char), &(range as c_char), &(n as c_int), d.as_mut_ptr(),
                     e.as_mut_ptr(), vl.as_ptr(), vu.as_ptr(), il.as_ptr(), iu.as_ptr(),
                     m as *mut _ as *mut _, w.as_mut_ptr(), z.as_mut_ptr(), &(ldz as c_int),
                     nzc.as_ptr(), isuppz.as_mut_ptr(), tryrac.as_mut_ptr(), work.as_mut_ptr(),
                     &(lwork as c_int), iwork.as_mut_ptr(), liwork.as_ptr(), info)
    }
}

#[inline]
pub fn dstemr(jobz: u8, range: u8, n: usize, d: &mut [f64], e: &mut [f64], vl: &[f64], vu: &[f64],
              il: &[i32], iu: &[i32], m: &mut u32, w: &mut [f64], z: &mut [f64], ldz: usize,
              nzc: &[i32], isuppz: &mut [i32], tryrac: &mut [i32], work: &mut [f64], lwork: usize,
              iwork: &mut [i32], liwork: &[i32], info: &mut i32) {

    unsafe {
        ffi::dstemr_(&(jobz as c_char), &(range as c_char), &(n as c_int), d.as_mut_ptr(),
                     e.as_mut_ptr(), vl.as_ptr(), vu.as_ptr(), il.as_ptr(), iu.as_ptr(),
                     m as *mut _ as *mut _, w.as_mut_ptr(), z.as_mut_ptr(), &(ldz as c_int),
                     nzc.as_ptr(), isuppz.as_mut_ptr(), tryrac.as_mut_ptr(), work.as_mut_ptr(),
                     &(lwork as c_int), iwork.as_mut_ptr(), liwork.as_ptr(), info)
    }
}

#[inline]
pub fn cstemr(jobz: u8, range: u8, n: usize, d: &mut [f32], e: &mut [f32], vl: &[f32], vu: &[f32],
              il: &[i32], iu: &[i32], m: &mut u32, w: &mut [f32], z: &mut [c32], ldz: usize,
              nzc: &[i32], isuppz: &mut [i32], tryrac: &mut [i32], work: &mut [f32], lwork: usize,
              iwork: &mut [i32], liwork: &[i32], info: &mut i32) {

    unsafe {
        ffi::cstemr_(&(jobz as c_char), &(range as c_char), &(n as c_int), d.as_mut_ptr(),
                     e.as_mut_ptr(), vl.as_ptr(), vu.as_ptr(), il.as_ptr(), iu.as_ptr(),
                     m as *mut _ as *mut _, w.as_mut_ptr(), z.as_mut_ptr() as *mut _,
                     &(ldz as c_int), nzc.as_ptr(), isuppz.as_mut_ptr(), tryrac.as_mut_ptr(),
                     work.as_mut_ptr(), &(lwork as c_int), iwork.as_mut_ptr(), liwork.as_ptr(),
                     info)
    }
}

#[inline]
pub fn zstemr(jobz: u8, range: u8, n: usize, d: &mut [f64], e: &mut [f64], vl: &[f64], vu: &[f64],
              il: &[i32], iu: &[i32], m: &mut u32, w: &mut [f64], z: &mut [c64], ldz: usize,
              nzc: &[i32], isuppz: &mut [i32], tryrac: &mut [i32], work: &mut [f64], lwork: usize,
              iwork: &mut [i32], liwork: &[i32], info: &mut i32) {

    unsafe {
        ffi::zstemr_(&(jobz as c_char), &(range as c_char), &(n as c_int), d.as_mut_ptr(),
                     e.as_mut_ptr(), vl.as_ptr(), vu.as_ptr(), il.as_ptr(), iu.as_ptr(),
                     m as *mut _ as *mut _, w.as_mut_ptr(), z.as_mut_ptr() as *mut _,
                     &(ldz as c_int), nzc.as_ptr(), isuppz.as_mut_ptr(), tryrac.as_mut_ptr(),
                     work.as_mut_ptr(), &(lwork as c_int), iwork.as_mut_ptr(), liwork.as_ptr(),
                     info)
    }
}

#[inline]
pub fn sstedc(compz: u8, n: usize, d: &mut [f32], e: &mut [f32], z: &mut [f32], ldz: usize,
              work: &mut [f32], lwork: usize, iwork: &mut [i32], liwork: &[i32], info: &mut i32) {

    unsafe {
        ffi::sstedc_(&(compz as c_char), &(n as c_int), d.as_mut_ptr(), e.as_mut_ptr(),
                     z.as_mut_ptr(), &(ldz as c_int), work.as_mut_ptr(), &(lwork as c_int),
                     iwork.as_mut_ptr(), liwork.as_ptr(), info)
    }
}

#[inline]
pub fn dstedc(compz: u8, n: usize, d: &mut [f64], e: &mut [f64], z: &mut [f64], ldz: usize,
              work: &mut [f64], lwork: usize, iwork: &mut [i32], liwork: &[i32], info: &mut i32) {

    unsafe {
        ffi::dstedc_(&(compz as c_char), &(n as c_int), d.as_mut_ptr(), e.as_mut_ptr(),
                     z.as_mut_ptr(), &(ldz as c_int), work.as_mut_ptr(), &(lwork as c_int),
                     iwork.as_mut_ptr(), liwork.as_ptr(), info)
    }
}

#[inline]
pub fn cstedc(compz: u8, n: usize, d: &mut [f32], e: &mut [f32], z: &mut [c32], ldz: usize,
              work: &mut [c32], lwork: usize, rwork: &mut [f32], lrwork: &[i32], iwork: &mut [i32],
              liwork: &[i32], info: &mut i32) {

    unsafe {
        ffi::cstedc_(&(compz as c_char), &(n as c_int), d.as_mut_ptr(), e.as_mut_ptr(),
                     z.as_mut_ptr() as *mut _, &(ldz as c_int), work.as_mut_ptr() as *mut _,
                     &(lwork as c_int), rwork.as_mut_ptr(), lrwork.as_ptr(), iwork.as_mut_ptr(),
                     liwork.as_ptr(), info)
    }
}

#[inline]
pub fn zstedc(compz: u8, n: usize, d: &mut [f64], e: &mut [f64], z: &mut [c64], ldz: usize,
              work: &mut [c64], lwork: usize, rwork: &mut [f64], lrwork: &[i32], iwork: &mut [i32],
              liwork: &[i32], info: &mut i32) {

    unsafe {
        ffi::zstedc_(&(compz as c_char), &(n as c_int), d.as_mut_ptr(), e.as_mut_ptr(),
                     z.as_mut_ptr() as *mut _, &(ldz as c_int), work.as_mut_ptr() as *mut _,
                     &(lwork as c_int), rwork.as_mut_ptr(), lrwork.as_ptr(), iwork.as_mut_ptr(),
                     liwork.as_ptr(), info)
    }
}

#[inline]
pub fn sstegr(jobz: u8, range: u8, n: usize, d: &mut [f32], e: &mut [f32], vl: &[f32], vu: &[f32],
              il: &[i32], iu: &[i32], abstol: &[f32], m: &mut u32, w: &mut [f32], z: &mut [f32],
              ldz: usize, isuppz: &mut [i32], work: &mut [f32], lwork: usize, iwork: &mut [i32],
              liwork: &[i32], info: &mut i32) {

    unsafe {
        ffi::sstegr_(&(jobz as c_char), &(range as c_char), &(n as c_int), d.as_mut_ptr(),
                     e.as_mut_ptr(), vl.as_ptr(), vu.as_ptr(), il.as_ptr(), iu.as_ptr(),
                     abstol.as_ptr(), m as *mut _ as *mut _, w.as_mut_ptr(), z.as_mut_ptr(),
                     &(ldz as c_int), isuppz.as_mut_ptr(), work.as_mut_ptr(), &(lwork as c_int),
                     iwork.as_mut_ptr(), liwork.as_ptr(), info)
    }
}

#[inline]
pub fn dstegr(jobz: u8, range: u8, n: usize, d: &mut [f64], e: &mut [f64], vl: &[f64], vu: &[f64],
              il: &[i32], iu: &[i32], abstol: &[f64], m: &mut u32, w: &mut [f64], z: &mut [f64],
              ldz: usize, isuppz: &mut [i32], work: &mut [f64], lwork: usize, iwork: &mut [i32],
              liwork: &[i32], info: &mut i32) {

    unsafe {
        ffi::dstegr_(&(jobz as c_char), &(range as c_char), &(n as c_int), d.as_mut_ptr(),
                     e.as_mut_ptr(), vl.as_ptr(), vu.as_ptr(), il.as_ptr(), iu.as_ptr(),
                     abstol.as_ptr(), m as *mut _ as *mut _, w.as_mut_ptr(), z.as_mut_ptr(),
                     &(ldz as c_int), isuppz.as_mut_ptr(), work.as_mut_ptr(), &(lwork as c_int),
                     iwork.as_mut_ptr(), liwork.as_ptr(), info)
    }
}

#[inline]
pub fn cstegr(jobz: u8, range: u8, n: usize, d: &mut [f32], e: &mut [f32], vl: &[f32], vu: &[f32],
              il: &[i32], iu: &[i32], abstol: &[f32], m: &mut u32, w: &mut [f32], z: &mut [c32],
              ldz: usize, isuppz: &mut [i32], work: &mut [f32], lwork: usize, iwork: &mut [i32],
              liwork: &[i32], info: &mut i32) {

    unsafe {
        ffi::cstegr_(&(jobz as c_char), &(range as c_char), &(n as c_int), d.as_mut_ptr(),
                     e.as_mut_ptr(), vl.as_ptr(), vu.as_ptr(), il.as_ptr(), iu.as_ptr(),
                     abstol.as_ptr(), m as *mut _ as *mut _, w.as_mut_ptr(),
                     z.as_mut_ptr() as *mut _, &(ldz as c_int), isuppz.as_mut_ptr(),
                     work.as_mut_ptr(), &(lwork as c_int), iwork.as_mut_ptr(), liwork.as_ptr(),
                     info)
    }
}

#[inline]
pub fn zstegr(jobz: u8, range: u8, n: usize, d: &mut [f64], e: &mut [f64], vl: &[f64], vu: &[f64],
              il: &[i32], iu: &[i32], abstol: &[f64], m: &mut u32, w: &mut [f64], z: &mut [c64],
              ldz: usize, isuppz: &mut [i32], work: &mut [f64], lwork: usize, iwork: &mut [i32],
              liwork: &[i32], info: &mut i32) {

    unsafe {
        ffi::zstegr_(&(jobz as c_char), &(range as c_char), &(n as c_int), d.as_mut_ptr(),
                     e.as_mut_ptr(), vl.as_ptr(), vu.as_ptr(), il.as_ptr(), iu.as_ptr(),
                     abstol.as_ptr(), m as *mut _ as *mut _, w.as_mut_ptr(),
                     z.as_mut_ptr() as *mut _, &(ldz as c_int), isuppz.as_mut_ptr(),
                     work.as_mut_ptr(), &(lwork as c_int), iwork.as_mut_ptr(), liwork.as_ptr(),
                     info)
    }
}

#[inline]
pub fn spteqr(compz: u8, n: usize, d: &mut [f32], e: &mut [f32], z: &mut [f32], ldz: usize,
              work: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::spteqr_(&(compz as c_char), &(n as c_int), d.as_mut_ptr(), e.as_mut_ptr(),
                     z.as_mut_ptr(), &(ldz as c_int), work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dpteqr(compz: u8, n: usize, d: &mut [f64], e: &mut [f64], z: &mut [f64], ldz: usize,
              work: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::dpteqr_(&(compz as c_char), &(n as c_int), d.as_mut_ptr(), e.as_mut_ptr(),
                     z.as_mut_ptr(), &(ldz as c_int), work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn cpteqr(compz: u8, n: usize, d: &mut [f32], e: &mut [f32], z: &mut [c32], ldz: usize,
              work: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::cpteqr_(&(compz as c_char), &(n as c_int), d.as_mut_ptr(), e.as_mut_ptr(),
                     z.as_mut_ptr() as *mut _, &(ldz as c_int), work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zpteqr(compz: u8, n: usize, d: &mut [f64], e: &mut [f64], z: &mut [c64], ldz: usize,
              work: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::zpteqr_(&(compz as c_char), &(n as c_int), d.as_mut_ptr(), e.as_mut_ptr(),
                     z.as_mut_ptr() as *mut _, &(ldz as c_int), work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn sstebz(range: u8, order: u8, n: usize, vl: &[f32], vu: &[f32], il: &[i32], iu: &[i32],
              abstol: &[f32], d: &[f32], e: &[f32], m: &mut u32, nsplit: &mut [i32], w: &mut [f32],
              iblock: &mut [i32], isplit: &mut [i32], work: &mut [f32], iwork: &mut [i32],
              info: &mut i32) {

    unsafe {
        ffi::sstebz_(&(range as c_char), &(order as c_char), &(n as c_int), vl.as_ptr(),
                     vu.as_ptr(), il.as_ptr(), iu.as_ptr(), abstol.as_ptr(), d.as_ptr(),
                     e.as_ptr(), m as *mut _ as *mut _, nsplit.as_mut_ptr(), w.as_mut_ptr(),
                     iblock.as_mut_ptr(), isplit.as_mut_ptr(), work.as_mut_ptr(),
                     iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dstebz(range: u8, order: u8, n: usize, vl: &[f64], vu: &[f64], il: &[i32], iu: &[i32],
              abstol: &[f64], d: &[f64], e: &[f64], m: &mut u32, nsplit: &mut [i32], w: &mut [f64],
              iblock: &mut [i32], isplit: &mut [i32], work: &mut [f64], iwork: &mut [i32],
              info: &mut i32) {

    unsafe {
        ffi::dstebz_(&(range as c_char), &(order as c_char), &(n as c_int), vl.as_ptr(),
                     vu.as_ptr(), il.as_ptr(), iu.as_ptr(), abstol.as_ptr(), d.as_ptr(),
                     e.as_ptr(), m as *mut _ as *mut _, nsplit.as_mut_ptr(), w.as_mut_ptr(),
                     iblock.as_mut_ptr(), isplit.as_mut_ptr(), work.as_mut_ptr(),
                     iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn sstein(n: usize, d: &[f32], e: &[f32], m: usize, w: &[f32], iblock: &[i32], isplit: &[i32],
              z: &mut [f32], ldz: usize, work: &mut [f32], iwork: &mut [i32], ifail: &mut [i32],
              info: &mut i32) {

    unsafe {
        ffi::sstein_(&(n as c_int), d.as_ptr(), e.as_ptr(), &(m as c_int), w.as_ptr(),
                     iblock.as_ptr(), isplit.as_ptr(), z.as_mut_ptr(), &(ldz as c_int),
                     work.as_mut_ptr(), iwork.as_mut_ptr(), ifail.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dstein(n: usize, d: &[f64], e: &[f64], m: usize, w: &[f64], iblock: &[i32], isplit: &[i32],
              z: &mut [f64], ldz: usize, work: &mut [f64], iwork: &mut [i32], ifail: &mut [i32],
              info: &mut i32) {

    unsafe {
        ffi::dstein_(&(n as c_int), d.as_ptr(), e.as_ptr(), &(m as c_int), w.as_ptr(),
                     iblock.as_ptr(), isplit.as_ptr(), z.as_mut_ptr(), &(ldz as c_int),
                     work.as_mut_ptr(), iwork.as_mut_ptr(), ifail.as_mut_ptr(), info)
    }
}

#[inline]
pub fn cstein(n: usize, d: &[f32], e: &[f32], m: usize, w: &[f32], iblock: &[i32], isplit: &[i32],
              z: &mut [c32], ldz: usize, work: &mut [f32], iwork: &mut [i32], ifail: &mut [i32],
              info: &mut i32) {

    unsafe {
        ffi::cstein_(&(n as c_int), d.as_ptr(), e.as_ptr(), &(m as c_int), w.as_ptr(),
                     iblock.as_ptr(), isplit.as_ptr(), z.as_mut_ptr() as *mut _, &(ldz as c_int),
                     work.as_mut_ptr(), iwork.as_mut_ptr(), ifail.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zstein(n: usize, d: &[f64], e: &[f64], m: usize, w: &[f64], iblock: &[i32], isplit: &[i32],
              z: &mut [c64], ldz: usize, work: &mut [f64], iwork: &mut [i32], ifail: &mut [i32],
              info: &mut i32) {

    unsafe {
        ffi::zstein_(&(n as c_int), d.as_ptr(), e.as_ptr(), &(m as c_int), w.as_ptr(),
                     iblock.as_ptr(), isplit.as_ptr(), z.as_mut_ptr() as *mut _, &(ldz as c_int),
                     work.as_mut_ptr(), iwork.as_mut_ptr(), ifail.as_mut_ptr(), info)
    }
}

#[inline]
pub fn sdisna(job: u8, m: usize, n: usize, d: &[f32], sep: &mut [f32], info: &mut i32) {
    unsafe {
        ffi::sdisna_(&(job as c_char), &(m as c_int), &(n as c_int), d.as_ptr(), sep.as_mut_ptr(),
                     info)
    }
}

#[inline]
pub fn ddisna(job: u8, m: usize, n: usize, d: &[f64], sep: &mut [f64], info: &mut i32) {
    unsafe {
        ffi::ddisna_(&(job as c_char), &(m as c_int), &(n as c_int), d.as_ptr(), sep.as_mut_ptr(),
                     info)
    }
}

#[inline]
pub fn ssygst(itype: &[i32], uplo: u8, n: usize, a: &mut [f32], lda: usize, b: &[f32], ldb: usize,
              info: &mut i32) {

    unsafe {
        ffi::ssygst_(itype.as_ptr(), &(uplo as c_char), &(n as c_int), a.as_mut_ptr(),
                     &(lda as c_int), b.as_ptr(), &(ldb as c_int), info)
    }
}

#[inline]
pub fn dsygst(itype: &[i32], uplo: u8, n: usize, a: &mut [f64], lda: usize, b: &[f64], ldb: usize,
              info: &mut i32) {

    unsafe {
        ffi::dsygst_(itype.as_ptr(), &(uplo as c_char), &(n as c_int), a.as_mut_ptr(),
                     &(lda as c_int), b.as_ptr(), &(ldb as c_int), info)
    }
}

#[inline]
pub fn chegst(itype: &[i32], uplo: u8, n: usize, a: &mut [c32], lda: usize, b: &[c32], ldb: usize,
              info: &mut i32) {

    unsafe {
        ffi::chegst_(itype.as_ptr(), &(uplo as c_char), &(n as c_int), a.as_mut_ptr() as *mut _,
                     &(lda as c_int), b.as_ptr() as *const _, &(ldb as c_int), info)
    }
}

#[inline]
pub fn zhegst(itype: &[i32], uplo: u8, n: usize, a: &mut [c64], lda: usize, b: &[c64], ldb: usize,
              info: &mut i32) {

    unsafe {
        ffi::zhegst_(itype.as_ptr(), &(uplo as c_char), &(n as c_int), a.as_mut_ptr() as *mut _,
                     &(lda as c_int), b.as_ptr() as *const _, &(ldb as c_int), info)
    }
}

#[inline]
pub fn sspgst(itype: &[i32], uplo: u8, n: usize, ap: &mut [f32], bp: &[f32], info: &mut i32) {
    unsafe {
        ffi::sspgst_(itype.as_ptr(), &(uplo as c_char), &(n as c_int), ap.as_mut_ptr(),
                     bp.as_ptr(), info)
    }
}

#[inline]
pub fn dspgst(itype: &[i32], uplo: u8, n: usize, ap: &mut [f64], bp: &[f64], info: &mut i32) {
    unsafe {
        ffi::dspgst_(itype.as_ptr(), &(uplo as c_char), &(n as c_int), ap.as_mut_ptr(),
                     bp.as_ptr(), info)
    }
}

#[inline]
pub fn chpgst(itype: &[i32], uplo: u8, n: usize, ap: &mut [c32], bp: &[c32], info: &mut i32) {
    unsafe {
        ffi::chpgst_(itype.as_ptr(), &(uplo as c_char), &(n as c_int), ap.as_mut_ptr() as *mut _,
                     bp.as_ptr() as *const _, info)
    }
}

#[inline]
pub fn zhpgst(itype: &[i32], uplo: u8, n: usize, ap: &mut [c64], bp: &[c64], info: &mut i32) {
    unsafe {
        ffi::zhpgst_(itype.as_ptr(), &(uplo as c_char), &(n as c_int), ap.as_mut_ptr() as *mut _,
                     bp.as_ptr() as *const _, info)
    }
}

#[inline]
pub fn ssbgst(vect: u8, uplo: u8, n: usize, ka: usize, kb: usize, ab: &mut [f32], ldab: usize,
              bb: &[f32], ldbb: usize, x: &mut [f32], ldx: usize, work: &mut [f32],
              info: &mut i32) {

    unsafe {
        ffi::ssbgst_(&(vect as c_char), &(uplo as c_char), &(n as c_int), &(ka as c_int),
                     &(kb as c_int), ab.as_mut_ptr(), &(ldab as c_int), bb.as_ptr(),
                     &(ldbb as c_int), x.as_mut_ptr(), &(ldx as c_int), work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dsbgst(vect: u8, uplo: u8, n: usize, ka: usize, kb: usize, ab: &mut [f64], ldab: usize,
              bb: &[f64], ldbb: usize, x: &mut [f64], ldx: usize, work: &mut [f64],
              info: &mut i32) {

    unsafe {
        ffi::dsbgst_(&(vect as c_char), &(uplo as c_char), &(n as c_int), &(ka as c_int),
                     &(kb as c_int), ab.as_mut_ptr(), &(ldab as c_int), bb.as_ptr(),
                     &(ldbb as c_int), x.as_mut_ptr(), &(ldx as c_int), work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn chbgst(vect: u8, uplo: u8, n: usize, ka: usize, kb: usize, ab: &mut [c32], ldab: usize,
              bb: &[c32], ldbb: usize, x: &mut [c32], ldx: usize, work: &mut [c32],
              rwork: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::chbgst_(&(vect as c_char), &(uplo as c_char), &(n as c_int), &(ka as c_int),
                     &(kb as c_int), ab.as_mut_ptr() as *mut _, &(ldab as c_int),
                     bb.as_ptr() as *const _, &(ldbb as c_int), x.as_mut_ptr() as *mut _,
                     &(ldx as c_int), work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zhbgst(vect: u8, uplo: u8, n: usize, ka: usize, kb: usize, ab: &mut [c64], ldab: usize,
              bb: &[c64], ldbb: usize, x: &mut [c64], ldx: usize, work: &mut [c64],
              rwork: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::zhbgst_(&(vect as c_char), &(uplo as c_char), &(n as c_int), &(ka as c_int),
                     &(kb as c_int), ab.as_mut_ptr() as *mut _, &(ldab as c_int),
                     bb.as_ptr() as *const _, &(ldbb as c_int), x.as_mut_ptr() as *mut _,
                     &(ldx as c_int), work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn spbstf(uplo: u8, n: usize, kd: usize, ab: &mut [f32], ldab: usize, info: &mut i32) {
    unsafe {
        ffi::spbstf_(&(uplo as c_char), &(n as c_int), &(kd as c_int), ab.as_mut_ptr(),
                     &(ldab as c_int), info)
    }
}

#[inline]
pub fn dpbstf(uplo: u8, n: usize, kd: usize, ab: &mut [f64], ldab: usize, info: &mut i32) {
    unsafe {
        ffi::dpbstf_(&(uplo as c_char), &(n as c_int), &(kd as c_int), ab.as_mut_ptr(),
                     &(ldab as c_int), info)
    }
}

#[inline]
pub fn cpbstf(uplo: u8, n: usize, kd: usize, ab: &mut [c32], ldab: usize, info: &mut i32) {
    unsafe {
        ffi::cpbstf_(&(uplo as c_char), &(n as c_int), &(kd as c_int), ab.as_mut_ptr() as *mut _,
                     &(ldab as c_int), info)
    }
}

#[inline]
pub fn zpbstf(uplo: u8, n: usize, kd: usize, ab: &mut [c64], ldab: usize, info: &mut i32) {
    unsafe {
        ffi::zpbstf_(&(uplo as c_char), &(n as c_int), &(kd as c_int), ab.as_mut_ptr() as *mut _,
                     &(ldab as c_int), info)
    }
}

#[inline]
pub fn sgehrd(n: usize, ilo: &[i32], ihi: &[i32], a: &mut [f32], lda: usize, tau: &mut [f32],
              work: &mut [f32], lwork: usize, info: &mut i32) {

    unsafe {
        ffi::sgehrd_(&(n as c_int), ilo.as_ptr(), ihi.as_ptr(), a.as_mut_ptr(), &(lda as c_int),
                     tau.as_mut_ptr(), work.as_mut_ptr(), &(lwork as c_int), info)
    }
}

#[inline]
pub fn dgehrd(n: usize, ilo: &[i32], ihi: &[i32], a: &mut [f64], lda: usize, tau: &mut [f64],
              work: &mut [f64], lwork: usize, info: &mut i32) {

    unsafe {
        ffi::dgehrd_(&(n as c_int), ilo.as_ptr(), ihi.as_ptr(), a.as_mut_ptr(), &(lda as c_int),
                     tau.as_mut_ptr(), work.as_mut_ptr(), &(lwork as c_int), info)
    }
}

#[inline]
pub fn cgehrd(n: usize, ilo: &[i32], ihi: &[i32], a: &mut [c32], lda: usize, tau: &mut [c32],
              work: &mut [c32], lwork: usize, info: &mut i32) {

    unsafe {
        ffi::cgehrd_(&(n as c_int), ilo.as_ptr(), ihi.as_ptr(), a.as_mut_ptr() as *mut _,
                     &(lda as c_int), tau.as_mut_ptr() as *mut _, work.as_mut_ptr() as *mut _,
                     &(lwork as c_int), info)
    }
}

#[inline]
pub fn zgehrd(n: usize, ilo: &[i32], ihi: &[i32], a: &mut [c64], lda: usize, tau: &mut [c64],
              work: &mut [c64], lwork: usize, info: &mut i32) {

    unsafe {
        ffi::zgehrd_(&(n as c_int), ilo.as_ptr(), ihi.as_ptr(), a.as_mut_ptr() as *mut _,
                     &(lda as c_int), tau.as_mut_ptr() as *mut _, work.as_mut_ptr() as *mut _,
                     &(lwork as c_int), info)
    }
}

#[inline]
pub fn sorghr(n: usize, ilo: &[i32], ihi: &[i32], a: &mut [f32], lda: usize, tau: &[f32],
              work: &mut [f32], lwork: usize, info: &mut i32) {

    unsafe {
        ffi::sorghr_(&(n as c_int), ilo.as_ptr(), ihi.as_ptr(), a.as_mut_ptr(), &(lda as c_int),
                     tau.as_ptr(), work.as_mut_ptr(), &(lwork as c_int), info)
    }
}

#[inline]
pub fn dorghr(n: usize, ilo: &[i32], ihi: &[i32], a: &mut [f64], lda: usize, tau: &[f64],
              work: &mut [f64], lwork: usize, info: &mut i32) {

    unsafe {
        ffi::dorghr_(&(n as c_int), ilo.as_ptr(), ihi.as_ptr(), a.as_mut_ptr(), &(lda as c_int),
                     tau.as_ptr(), work.as_mut_ptr(), &(lwork as c_int), info)
    }
}

#[inline]
pub fn sormhr(side: u8, trans: u8, m: usize, n: usize, ilo: &[i32], ihi: &[i32], a: &[f32],
              lda: usize, tau: &[f32], c: &mut [f32], ldc: usize, work: &mut [f32], lwork: usize,
              info: &mut i32) {

    unsafe {
        ffi::sormhr_(&(side as c_char), &(trans as c_char), &(m as c_int), &(n as c_int),
                     ilo.as_ptr(), ihi.as_ptr(), a.as_ptr(), &(lda as c_int), tau.as_ptr(),
                     c.as_mut_ptr(), &(ldc as c_int), work.as_mut_ptr(), &(lwork as c_int), info)
    }
}

#[inline]
pub fn dormhr(side: u8, trans: u8, m: usize, n: usize, ilo: &[i32], ihi: &[i32], a: &[f64],
              lda: usize, tau: &[f64], c: &mut [f64], ldc: usize, work: &mut [f64], lwork: usize,
              info: &mut i32) {

    unsafe {
        ffi::dormhr_(&(side as c_char), &(trans as c_char), &(m as c_int), &(n as c_int),
                     ilo.as_ptr(), ihi.as_ptr(), a.as_ptr(), &(lda as c_int), tau.as_ptr(),
                     c.as_mut_ptr(), &(ldc as c_int), work.as_mut_ptr(), &(lwork as c_int), info)
    }
}

#[inline]
pub fn cunghr(n: usize, ilo: &[i32], ihi: &[i32], a: &mut [c32], lda: usize, tau: &[c32],
              work: &mut [c32], lwork: usize, info: &mut i32) {

    unsafe {
        ffi::cunghr_(&(n as c_int), ilo.as_ptr(), ihi.as_ptr(), a.as_mut_ptr() as *mut _,
                     &(lda as c_int), tau.as_ptr() as *const _, work.as_mut_ptr() as *mut _,
                     &(lwork as c_int), info)
    }
}

#[inline]
pub fn zunghr(n: usize, ilo: &[i32], ihi: &[i32], a: &mut [c64], lda: usize, tau: &[c64],
              work: &mut [c64], lwork: usize, info: &mut i32) {

    unsafe {
        ffi::zunghr_(&(n as c_int), ilo.as_ptr(), ihi.as_ptr(), a.as_mut_ptr() as *mut _,
                     &(lda as c_int), tau.as_ptr() as *const _, work.as_mut_ptr() as *mut _,
                     &(lwork as c_int), info)
    }
}

#[inline]
pub fn cunmhr(side: u8, trans: u8, m: usize, n: usize, ilo: &[i32], ihi: &[i32], a: &[c32],
              lda: usize, tau: &[c32], c: &mut [c32], ldc: usize, work: &mut [c32], lwork: usize,
              info: &mut i32) {

    unsafe {
        ffi::cunmhr_(&(side as c_char), &(trans as c_char), &(m as c_int), &(n as c_int),
                     ilo.as_ptr(), ihi.as_ptr(), a.as_ptr() as *const _, &(lda as c_int),
                     tau.as_ptr() as *const _, c.as_mut_ptr() as *mut _, &(ldc as c_int),
                     work.as_mut_ptr() as *mut _, &(lwork as c_int), info)
    }
}

#[inline]
pub fn zunmhr(side: u8, trans: u8, m: usize, n: usize, ilo: &[i32], ihi: &[i32], a: &[c64],
              lda: usize, tau: &[c64], c: &mut [c64], ldc: usize, work: &mut [c64], lwork: usize,
              info: &mut i32) {

    unsafe {
        ffi::zunmhr_(&(side as c_char), &(trans as c_char), &(m as c_int), &(n as c_int),
                     ilo.as_ptr(), ihi.as_ptr(), a.as_ptr() as *const _, &(lda as c_int),
                     tau.as_ptr() as *const _, c.as_mut_ptr() as *mut _, &(ldc as c_int),
                     work.as_mut_ptr() as *mut _, &(lwork as c_int), info)
    }
}

#[inline]
pub fn sgebal(job: u8, n: usize, a: &mut [f32], lda: usize, ilo: &mut [i32], ihi: &mut [i32],
              scale: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::sgebal_(&(job as c_char), &(n as c_int), a.as_mut_ptr(), &(lda as c_int),
                     ilo.as_mut_ptr(), ihi.as_mut_ptr(), scale.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dgebal(job: u8, n: usize, a: &mut [f64], lda: usize, ilo: &mut [i32], ihi: &mut [i32],
              scale: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::dgebal_(&(job as c_char), &(n as c_int), a.as_mut_ptr(), &(lda as c_int),
                     ilo.as_mut_ptr(), ihi.as_mut_ptr(), scale.as_mut_ptr(), info)
    }
}

#[inline]
pub fn cgebal(job: u8, n: usize, a: &mut [c32], lda: usize, ilo: &mut [i32], ihi: &mut [i32],
              scale: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::cgebal_(&(job as c_char), &(n as c_int), a.as_mut_ptr() as *mut _, &(lda as c_int),
                     ilo.as_mut_ptr(), ihi.as_mut_ptr(), scale.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zgebal(job: u8, n: usize, a: &mut [c64], lda: usize, ilo: &mut [i32], ihi: &mut [i32],
              scale: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::zgebal_(&(job as c_char), &(n as c_int), a.as_mut_ptr() as *mut _, &(lda as c_int),
                     ilo.as_mut_ptr(), ihi.as_mut_ptr(), scale.as_mut_ptr(), info)
    }
}

#[inline]
pub fn sgebak(job: u8, side: u8, n: usize, ilo: &[i32], ihi: &[i32], scale: &[f32], m: usize,
              v: &mut [f32], ldv: usize, info: &mut i32) {

    unsafe {
        ffi::sgebak_(&(job as c_char), &(side as c_char), &(n as c_int), ilo.as_ptr(),
                     ihi.as_ptr(), scale.as_ptr(), &(m as c_int), v.as_mut_ptr(), &(ldv as c_int),
                     info)
    }
}

#[inline]
pub fn dgebak(job: u8, side: u8, n: usize, ilo: &[i32], ihi: &[i32], scale: &[f64], m: usize,
              v: &mut [f64], ldv: usize, info: &mut i32) {

    unsafe {
        ffi::dgebak_(&(job as c_char), &(side as c_char), &(n as c_int), ilo.as_ptr(),
                     ihi.as_ptr(), scale.as_ptr(), &(m as c_int), v.as_mut_ptr(), &(ldv as c_int),
                     info)
    }
}

#[inline]
pub fn cgebak(job: u8, side: u8, n: usize, ilo: &[i32], ihi: &[i32], scale: &[f32], m: usize,
              v: &mut [c32], ldv: usize, info: &mut i32) {

    unsafe {
        ffi::cgebak_(&(job as c_char), &(side as c_char), &(n as c_int), ilo.as_ptr(),
                     ihi.as_ptr(), scale.as_ptr(), &(m as c_int), v.as_mut_ptr() as *mut _,
                     &(ldv as c_int), info)
    }
}

#[inline]
pub fn zgebak(job: u8, side: u8, n: usize, ilo: &[i32], ihi: &[i32], scale: &[f64], m: usize,
              v: &mut [c64], ldv: usize, info: &mut i32) {

    unsafe {
        ffi::zgebak_(&(job as c_char), &(side as c_char), &(n as c_int), ilo.as_ptr(),
                     ihi.as_ptr(), scale.as_ptr(), &(m as c_int), v.as_mut_ptr() as *mut _,
                     &(ldv as c_int), info)
    }
}

#[inline]
pub fn shseqr(job: u8, compz: u8, n: usize, ilo: &[i32], ihi: &[i32], h: &mut [f32], ldh: usize,
              wr: &mut [f32], wi: &mut [f32], z: &mut [f32], ldz: usize, work: &mut [f32],
              lwork: usize, info: &mut i32) {

    unsafe {
        ffi::shseqr_(&(job as c_char), &(compz as c_char), &(n as c_int), ilo.as_ptr(),
                     ihi.as_ptr(), h.as_mut_ptr(), &(ldh as c_int), wr.as_mut_ptr(),
                     wi.as_mut_ptr(), z.as_mut_ptr(), &(ldz as c_int), work.as_mut_ptr(),
                     &(lwork as c_int), info)
    }
}

#[inline]
pub fn dhseqr(job: u8, compz: u8, n: usize, ilo: &[i32], ihi: &[i32], h: &mut [f64], ldh: usize,
              wr: &mut [f64], wi: &mut [f64], z: &mut [f64], ldz: usize, work: &mut [f64],
              lwork: usize, info: &mut i32) {

    unsafe {
        ffi::dhseqr_(&(job as c_char), &(compz as c_char), &(n as c_int), ilo.as_ptr(),
                     ihi.as_ptr(), h.as_mut_ptr(), &(ldh as c_int), wr.as_mut_ptr(),
                     wi.as_mut_ptr(), z.as_mut_ptr(), &(ldz as c_int), work.as_mut_ptr(),
                     &(lwork as c_int), info)
    }
}

#[inline]
pub fn chseqr(job: u8, compz: u8, n: usize, ilo: &[i32], ihi: &[i32], h: &mut [c32], ldh: usize,
              w: &mut [c32], z: &mut [c32], ldz: usize, work: &mut [c32], lwork: usize,
              info: &mut i32) {

    unsafe {
        ffi::chseqr_(&(job as c_char), &(compz as c_char), &(n as c_int), ilo.as_ptr(),
                     ihi.as_ptr(), h.as_mut_ptr() as *mut _, &(ldh as c_int),
                     w.as_mut_ptr() as *mut _, z.as_mut_ptr() as *mut _, &(ldz as c_int),
                     work.as_mut_ptr() as *mut _, &(lwork as c_int), info)
    }
}

#[inline]
pub fn zhseqr(job: u8, compz: u8, n: usize, ilo: &[i32], ihi: &[i32], h: &mut [c64], ldh: usize,
              w: &mut [c64], z: &mut [c64], ldz: usize, work: &mut [c64], lwork: usize,
              info: &mut i32) {

    unsafe {
        ffi::zhseqr_(&(job as c_char), &(compz as c_char), &(n as c_int), ilo.as_ptr(),
                     ihi.as_ptr(), h.as_mut_ptr() as *mut _, &(ldh as c_int),
                     w.as_mut_ptr() as *mut _, z.as_mut_ptr() as *mut _, &(ldz as c_int),
                     work.as_mut_ptr() as *mut _, &(lwork as c_int), info)
    }
}

#[inline]
pub fn shsein(side: u8, eigsrc: u8, initv: u8, select: &mut [i32], n: usize, h: &[f32], ldh: usize,
              wr: &mut [f32], wi: &[f32], vl: &mut [f32], ldvl: usize, vr: &mut [f32], ldvr: usize,
              mm: usize, m: &mut u32, work: &mut [f32], ifaill: &mut [i32], ifailr: &mut [i32],
              info: &mut i32) {

    unsafe {
        ffi::shsein_(&(side as c_char), &(eigsrc as c_char), &(initv as c_char),
                     select.as_mut_ptr(), &(n as c_int), h.as_ptr(), &(ldh as c_int),
                     wr.as_mut_ptr(), wi.as_ptr(), vl.as_mut_ptr(), &(ldvl as c_int),
                     vr.as_mut_ptr(), &(ldvr as c_int), &(mm as c_int), m as *mut _ as *mut _,
                     work.as_mut_ptr(), ifaill.as_mut_ptr(), ifailr.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dhsein(side: u8, eigsrc: u8, initv: u8, select: &mut [i32], n: usize, h: &[f64], ldh: usize,
              wr: &mut [f64], wi: &[f64], vl: &mut [f64], ldvl: usize, vr: &mut [f64], ldvr: usize,
              mm: usize, m: &mut u32, work: &mut [f64], ifaill: &mut [i32], ifailr: &mut [i32],
              info: &mut i32) {

    unsafe {
        ffi::dhsein_(&(side as c_char), &(eigsrc as c_char), &(initv as c_char),
                     select.as_mut_ptr(), &(n as c_int), h.as_ptr(), &(ldh as c_int),
                     wr.as_mut_ptr(), wi.as_ptr(), vl.as_mut_ptr(), &(ldvl as c_int),
                     vr.as_mut_ptr(), &(ldvr as c_int), &(mm as c_int), m as *mut _ as *mut _,
                     work.as_mut_ptr(), ifaill.as_mut_ptr(), ifailr.as_mut_ptr(), info)
    }
}

#[inline]
pub fn chsein(side: u8, eigsrc: u8, initv: u8, select: &[i32], n: usize, h: &[c32], ldh: usize,
              w: &mut [c32], vl: &mut [c32], ldvl: usize, vr: &mut [c32], ldvr: usize, mm: usize,
              m: &mut u32, work: &mut [c32], rwork: &mut [f32], ifaill: &mut [i32],
              ifailr: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::chsein_(&(side as c_char), &(eigsrc as c_char), &(initv as c_char), select.as_ptr(),
                     &(n as c_int), h.as_ptr() as *const _, &(ldh as c_int),
                     w.as_mut_ptr() as *mut _, vl.as_mut_ptr() as *mut _, &(ldvl as c_int),
                     vr.as_mut_ptr() as *mut _, &(ldvr as c_int), &(mm as c_int),
                     m as *mut _ as *mut _, work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(),
                     ifaill.as_mut_ptr(), ifailr.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zhsein(side: u8, eigsrc: u8, initv: u8, select: &[i32], n: usize, h: &[c64], ldh: usize,
              w: &mut [c64], vl: &mut [c64], ldvl: usize, vr: &mut [c64], ldvr: usize, mm: usize,
              m: &mut u32, work: &mut [c64], rwork: &mut [f64], ifaill: &mut [i32],
              ifailr: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::zhsein_(&(side as c_char), &(eigsrc as c_char), &(initv as c_char), select.as_ptr(),
                     &(n as c_int), h.as_ptr() as *const _, &(ldh as c_int),
                     w.as_mut_ptr() as *mut _, vl.as_mut_ptr() as *mut _, &(ldvl as c_int),
                     vr.as_mut_ptr() as *mut _, &(ldvr as c_int), &(mm as c_int),
                     m as *mut _ as *mut _, work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(),
                     ifaill.as_mut_ptr(), ifailr.as_mut_ptr(), info)
    }
}

#[inline]
pub fn strevc(side: u8, howmny: u8, select: &mut [i32], n: usize, t: &[f32], ldt: usize,
              vl: &mut [f32], ldvl: usize, vr: &mut [f32], ldvr: usize, mm: usize, m: &mut u32,
              work: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::strevc_(&(side as c_char), &(howmny as c_char), select.as_mut_ptr(), &(n as c_int),
                     t.as_ptr(), &(ldt as c_int), vl.as_mut_ptr(), &(ldvl as c_int),
                     vr.as_mut_ptr(), &(ldvr as c_int), &(mm as c_int), m as *mut _ as *mut _,
                     work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dtrevc(side: u8, howmny: u8, select: &mut [i32], n: usize, t: &[f64], ldt: usize,
              vl: &mut [f64], ldvl: usize, vr: &mut [f64], ldvr: usize, mm: usize, m: &mut u32,
              work: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::dtrevc_(&(side as c_char), &(howmny as c_char), select.as_mut_ptr(), &(n as c_int),
                     t.as_ptr(), &(ldt as c_int), vl.as_mut_ptr(), &(ldvl as c_int),
                     vr.as_mut_ptr(), &(ldvr as c_int), &(mm as c_int), m as *mut _ as *mut _,
                     work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn ctrevc(side: u8, howmny: u8, select: &[i32], n: usize, t: &mut [c32], ldt: usize,
              vl: &mut [c32], ldvl: usize, vr: &mut [c32], ldvr: usize, mm: usize, m: &mut u32,
              work: &mut [c32], rwork: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::ctrevc_(&(side as c_char), &(howmny as c_char), select.as_ptr(), &(n as c_int),
                     t.as_mut_ptr() as *mut _, &(ldt as c_int), vl.as_mut_ptr() as *mut _,
                     &(ldvl as c_int), vr.as_mut_ptr() as *mut _, &(ldvr as c_int), &(mm as c_int),
                     m as *mut _ as *mut _, work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn ztrevc(side: u8, howmny: u8, select: &[i32], n: usize, t: &mut [c64], ldt: usize,
              vl: &mut [c64], ldvl: usize, vr: &mut [c64], ldvr: usize, mm: usize, m: &mut u32,
              work: &mut [c64], rwork: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::ztrevc_(&(side as c_char), &(howmny as c_char), select.as_ptr(), &(n as c_int),
                     t.as_mut_ptr() as *mut _, &(ldt as c_int), vl.as_mut_ptr() as *mut _,
                     &(ldvl as c_int), vr.as_mut_ptr() as *mut _, &(ldvr as c_int), &(mm as c_int),
                     m as *mut _ as *mut _, work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn strsna(job: u8, howmny: u8, select: &[i32], n: usize, t: &[f32], ldt: usize, vl: &[f32],
              ldvl: usize, vr: &[f32], ldvr: usize, s: &mut [f32], sep: &mut [f32], mm: usize,
              m: &mut u32, work: &mut [f32], ldwork: usize, iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::strsna_(&(job as c_char), &(howmny as c_char), select.as_ptr(), &(n as c_int),
                     t.as_ptr(), &(ldt as c_int), vl.as_ptr(), &(ldvl as c_int), vr.as_ptr(),
                     &(ldvr as c_int), s.as_mut_ptr(), sep.as_mut_ptr(), &(mm as c_int),
                     m as *mut _ as *mut _, work.as_mut_ptr(), &(ldwork as c_int),
                     iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dtrsna(job: u8, howmny: u8, select: &[i32], n: usize, t: &[f64], ldt: usize, vl: &[f64],
              ldvl: usize, vr: &[f64], ldvr: usize, s: &mut [f64], sep: &mut [f64], mm: usize,
              m: &mut u32, work: &mut [f64], ldwork: usize, iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::dtrsna_(&(job as c_char), &(howmny as c_char), select.as_ptr(), &(n as c_int),
                     t.as_ptr(), &(ldt as c_int), vl.as_ptr(), &(ldvl as c_int), vr.as_ptr(),
                     &(ldvr as c_int), s.as_mut_ptr(), sep.as_mut_ptr(), &(mm as c_int),
                     m as *mut _ as *mut _, work.as_mut_ptr(), &(ldwork as c_int),
                     iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn ctrsna(job: u8, howmny: u8, select: &[i32], n: usize, t: &[c32], ldt: usize, vl: &[c32],
              ldvl: usize, vr: &[c32], ldvr: usize, s: &mut [f32], sep: &mut [f32], mm: usize,
              m: &mut u32, work: &mut [c32], ldwork: usize, rwork: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::ctrsna_(&(job as c_char), &(howmny as c_char), select.as_ptr(), &(n as c_int),
                     t.as_ptr() as *const _, &(ldt as c_int), vl.as_ptr() as *const _,
                     &(ldvl as c_int), vr.as_ptr() as *const _, &(ldvr as c_int), s.as_mut_ptr(),
                     sep.as_mut_ptr(), &(mm as c_int), m as *mut _ as *mut _,
                     work.as_mut_ptr() as *mut _, &(ldwork as c_int), rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn ztrsna(job: u8, howmny: u8, select: &[i32], n: usize, t: &[c64], ldt: usize, vl: &[c64],
              ldvl: usize, vr: &[c64], ldvr: usize, s: &mut [f64], sep: &mut [f64], mm: usize,
              m: &mut u32, work: &mut [c64], ldwork: usize, rwork: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::ztrsna_(&(job as c_char), &(howmny as c_char), select.as_ptr(), &(n as c_int),
                     t.as_ptr() as *const _, &(ldt as c_int), vl.as_ptr() as *const _,
                     &(ldvl as c_int), vr.as_ptr() as *const _, &(ldvr as c_int), s.as_mut_ptr(),
                     sep.as_mut_ptr(), &(mm as c_int), m as *mut _ as *mut _,
                     work.as_mut_ptr() as *mut _, &(ldwork as c_int), rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn strexc(compq: u8, n: usize, t: &mut [f32], ldt: usize, q: &mut [f32], ldq: usize,
              ifst: &mut [i32], ilst: &mut [i32], work: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::strexc_(&(compq as c_char), &(n as c_int), t.as_mut_ptr(), &(ldt as c_int),
                     q.as_mut_ptr(), &(ldq as c_int), ifst.as_mut_ptr(), ilst.as_mut_ptr(),
                     work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dtrexc(compq: u8, n: usize, t: &mut [f64], ldt: usize, q: &mut [f64], ldq: usize,
              ifst: &mut [i32], ilst: &mut [i32], work: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::dtrexc_(&(compq as c_char), &(n as c_int), t.as_mut_ptr(), &(ldt as c_int),
                     q.as_mut_ptr(), &(ldq as c_int), ifst.as_mut_ptr(), ilst.as_mut_ptr(),
                     work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn ctrexc(compq: u8, n: usize, t: &mut [c32], ldt: usize, q: &mut [c32], ldq: usize,
              ifst: &[i32], ilst: &[i32], info: &mut i32) {

    unsafe {
        ffi::ctrexc_(&(compq as c_char), &(n as c_int), t.as_mut_ptr() as *mut _, &(ldt as c_int),
                     q.as_mut_ptr() as *mut _, &(ldq as c_int), ifst.as_ptr(), ilst.as_ptr(), info)
    }
}

#[inline]
pub fn ztrexc(compq: u8, n: usize, t: &mut [c64], ldt: usize, q: &mut [c64], ldq: usize,
              ifst: &[i32], ilst: &[i32], info: &mut i32) {

    unsafe {
        ffi::ztrexc_(&(compq as c_char), &(n as c_int), t.as_mut_ptr() as *mut _, &(ldt as c_int),
                     q.as_mut_ptr() as *mut _, &(ldq as c_int), ifst.as_ptr(), ilst.as_ptr(), info)
    }
}

#[inline]
pub fn strsen(job: u8, compq: u8, select: &[i32], n: usize, t: &mut [f32], ldt: usize,
              q: &mut [f32], ldq: usize, wr: &mut [f32], wi: &mut [f32], m: &mut u32,
              s: &mut [f32], sep: &mut [f32], work: &mut [f32], lwork: usize, iwork: &mut [i32],
              liwork: &[i32], info: &mut i32) {

    unsafe {
        ffi::strsen_(&(job as c_char), &(compq as c_char), select.as_ptr(), &(n as c_int),
                     t.as_mut_ptr(), &(ldt as c_int), q.as_mut_ptr(), &(ldq as c_int),
                     wr.as_mut_ptr(), wi.as_mut_ptr(), m as *mut _ as *mut _, s.as_mut_ptr(),
                     sep.as_mut_ptr(), work.as_mut_ptr(), &(lwork as c_int), iwork.as_mut_ptr(),
                     liwork.as_ptr(), info)
    }
}

#[inline]
pub fn dtrsen(job: u8, compq: u8, select: &[i32], n: usize, t: &mut [f64], ldt: usize,
              q: &mut [f64], ldq: usize, wr: &mut [f64], wi: &mut [f64], m: &mut u32,
              s: &mut [f64], sep: &mut [f64], work: &mut [f64], lwork: usize, iwork: &mut [i32],
              liwork: &[i32], info: &mut i32) {

    unsafe {
        ffi::dtrsen_(&(job as c_char), &(compq as c_char), select.as_ptr(), &(n as c_int),
                     t.as_mut_ptr(), &(ldt as c_int), q.as_mut_ptr(), &(ldq as c_int),
                     wr.as_mut_ptr(), wi.as_mut_ptr(), m as *mut _ as *mut _, s.as_mut_ptr(),
                     sep.as_mut_ptr(), work.as_mut_ptr(), &(lwork as c_int), iwork.as_mut_ptr(),
                     liwork.as_ptr(), info)
    }
}

#[inline]
pub fn ctrsen(job: u8, compq: u8, select: &[i32], n: usize, t: &mut [c32], ldt: usize,
              q: &mut [c32], ldq: usize, w: &mut [c32], m: &mut u32, s: &mut [f32],
              sep: &mut [f32], work: &mut [c32], lwork: usize, info: &mut i32) {

    unsafe {
        ffi::ctrsen_(&(job as c_char), &(compq as c_char), select.as_ptr(), &(n as c_int),
                     t.as_mut_ptr() as *mut _, &(ldt as c_int), q.as_mut_ptr() as *mut _,
                     &(ldq as c_int), w.as_mut_ptr() as *mut _, m as *mut _ as *mut _,
                     s.as_mut_ptr(), sep.as_mut_ptr(), work.as_mut_ptr() as *mut _,
                     &(lwork as c_int), info)
    }
}

#[inline]
pub fn ztrsen(job: u8, compq: u8, select: &[i32], n: usize, t: &mut [c64], ldt: usize,
              q: &mut [c64], ldq: usize, w: &mut [c64], m: &mut u32, s: &mut [f64],
              sep: &mut [f64], work: &mut [c64], lwork: usize, info: &mut i32) {

    unsafe {
        ffi::ztrsen_(&(job as c_char), &(compq as c_char), select.as_ptr(), &(n as c_int),
                     t.as_mut_ptr() as *mut _, &(ldt as c_int), q.as_mut_ptr() as *mut _,
                     &(ldq as c_int), w.as_mut_ptr() as *mut _, m as *mut _ as *mut _,
                     s.as_mut_ptr(), sep.as_mut_ptr(), work.as_mut_ptr() as *mut _,
                     &(lwork as c_int), info)
    }
}

#[inline]
pub fn strsyl(trana: u8, tranb: u8, isgn: &[i32], m: usize, n: usize, a: &[f32], lda: usize,
              b: &[f32], ldb: usize, c: &mut [f32], ldc: usize, scale: &mut [f32],
              info: &mut i32) {

    unsafe {
        ffi::strsyl_(&(trana as c_char), &(tranb as c_char), isgn.as_ptr(), &(m as c_int),
                     &(n as c_int), a.as_ptr(), &(lda as c_int), b.as_ptr(), &(ldb as c_int),
                     c.as_mut_ptr(), &(ldc as c_int), scale.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dtrsyl(trana: u8, tranb: u8, isgn: &[i32], m: usize, n: usize, a: &[f64], lda: usize,
              b: &[f64], ldb: usize, c: &mut [f64], ldc: usize, scale: &mut [f64],
              info: &mut i32) {

    unsafe {
        ffi::dtrsyl_(&(trana as c_char), &(tranb as c_char), isgn.as_ptr(), &(m as c_int),
                     &(n as c_int), a.as_ptr(), &(lda as c_int), b.as_ptr(), &(ldb as c_int),
                     c.as_mut_ptr(), &(ldc as c_int), scale.as_mut_ptr(), info)
    }
}

#[inline]
pub fn ctrsyl(trana: u8, tranb: u8, isgn: &[i32], m: usize, n: usize, a: &[c32], lda: usize,
              b: &[c32], ldb: usize, c: &mut [c32], ldc: usize, scale: &mut [f32],
              info: &mut i32) {

    unsafe {
        ffi::ctrsyl_(&(trana as c_char), &(tranb as c_char), isgn.as_ptr(), &(m as c_int),
                     &(n as c_int), a.as_ptr() as *const _, &(lda as c_int),
                     b.as_ptr() as *const _, &(ldb as c_int), c.as_mut_ptr() as *mut _,
                     &(ldc as c_int), scale.as_mut_ptr(), info)
    }
}

#[inline]
pub fn ztrsyl(trana: u8, tranb: u8, isgn: &[i32], m: usize, n: usize, a: &[c64], lda: usize,
              b: &[c64], ldb: usize, c: &mut [c64], ldc: usize, scale: &mut [f64],
              info: &mut i32) {

    unsafe {
        ffi::ztrsyl_(&(trana as c_char), &(tranb as c_char), isgn.as_ptr(), &(m as c_int),
                     &(n as c_int), a.as_ptr() as *const _, &(lda as c_int),
                     b.as_ptr() as *const _, &(ldb as c_int), c.as_mut_ptr() as *mut _,
                     &(ldc as c_int), scale.as_mut_ptr(), info)
    }
}

#[inline]
pub fn sgghrd(compq: u8, compz: u8, n: usize, ilo: &[i32], ihi: &[i32], a: &mut [f32], lda: usize,
              b: &mut [f32], ldb: usize, q: &mut [f32], ldq: usize, z: &mut [f32], ldz: usize,
              info: &mut i32) {

    unsafe {
        ffi::sgghrd_(&(compq as c_char), &(compz as c_char), &(n as c_int), ilo.as_ptr(),
                     ihi.as_ptr(), a.as_mut_ptr(), &(lda as c_int), b.as_mut_ptr(),
                     &(ldb as c_int), q.as_mut_ptr(), &(ldq as c_int), z.as_mut_ptr(),
                     &(ldz as c_int), info)
    }
}

#[inline]
pub fn dgghrd(compq: u8, compz: u8, n: usize, ilo: &[i32], ihi: &[i32], a: &mut [f64], lda: usize,
              b: &mut [f64], ldb: usize, q: &mut [f64], ldq: usize, z: &mut [f64], ldz: usize,
              info: &mut i32) {

    unsafe {
        ffi::dgghrd_(&(compq as c_char), &(compz as c_char), &(n as c_int), ilo.as_ptr(),
                     ihi.as_ptr(), a.as_mut_ptr(), &(lda as c_int), b.as_mut_ptr(),
                     &(ldb as c_int), q.as_mut_ptr(), &(ldq as c_int), z.as_mut_ptr(),
                     &(ldz as c_int), info)
    }
}

#[inline]
pub fn cgghrd(compq: u8, compz: u8, n: usize, ilo: &[i32], ihi: &[i32], a: &mut [c32], lda: usize,
              b: &mut [c32], ldb: usize, q: &mut [c32], ldq: usize, z: &mut [c32], ldz: usize,
              info: &mut i32) {

    unsafe {
        ffi::cgghrd_(&(compq as c_char), &(compz as c_char), &(n as c_int), ilo.as_ptr(),
                     ihi.as_ptr(), a.as_mut_ptr() as *mut _, &(lda as c_int),
                     b.as_mut_ptr() as *mut _, &(ldb as c_int), q.as_mut_ptr() as *mut _,
                     &(ldq as c_int), z.as_mut_ptr() as *mut _, &(ldz as c_int), info)
    }
}

#[inline]
pub fn zgghrd(compq: u8, compz: u8, n: usize, ilo: &[i32], ihi: &[i32], a: &mut [c64], lda: usize,
              b: &mut [c64], ldb: usize, q: &mut [c64], ldq: usize, z: &mut [c64], ldz: usize,
              info: &mut i32) {

    unsafe {
        ffi::zgghrd_(&(compq as c_char), &(compz as c_char), &(n as c_int), ilo.as_ptr(),
                     ihi.as_ptr(), a.as_mut_ptr() as *mut _, &(lda as c_int),
                     b.as_mut_ptr() as *mut _, &(ldb as c_int), q.as_mut_ptr() as *mut _,
                     &(ldq as c_int), z.as_mut_ptr() as *mut _, &(ldz as c_int), info)
    }
}

#[inline]
pub fn sggbal(job: u8, n: usize, a: &mut [f32], lda: usize, b: &mut [f32], ldb: usize,
              ilo: &mut [i32], ihi: &mut [i32], lscale: &mut [f32], rscale: &mut [f32],
              work: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::sggbal_(&(job as c_char), &(n as c_int), a.as_mut_ptr(), &(lda as c_int),
                     b.as_mut_ptr(), &(ldb as c_int), ilo.as_mut_ptr(), ihi.as_mut_ptr(),
                     lscale.as_mut_ptr(), rscale.as_mut_ptr(), work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dggbal(job: u8, n: usize, a: &mut [f64], lda: usize, b: &mut [f64], ldb: usize,
              ilo: &mut [i32], ihi: &mut [i32], lscale: &mut [f64], rscale: &mut [f64],
              work: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::dggbal_(&(job as c_char), &(n as c_int), a.as_mut_ptr(), &(lda as c_int),
                     b.as_mut_ptr(), &(ldb as c_int), ilo.as_mut_ptr(), ihi.as_mut_ptr(),
                     lscale.as_mut_ptr(), rscale.as_mut_ptr(), work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn cggbal(job: u8, n: usize, a: &mut [c32], lda: usize, b: &mut [c32], ldb: usize,
              ilo: &mut [i32], ihi: &mut [i32], lscale: &mut [f32], rscale: &mut [f32],
              work: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::cggbal_(&(job as c_char), &(n as c_int), a.as_mut_ptr() as *mut _, &(lda as c_int),
                     b.as_mut_ptr() as *mut _, &(ldb as c_int), ilo.as_mut_ptr(), ihi.as_mut_ptr(),
                     lscale.as_mut_ptr(), rscale.as_mut_ptr(), work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zggbal(job: u8, n: usize, a: &mut [c64], lda: usize, b: &mut [c64], ldb: usize,
              ilo: &mut [i32], ihi: &mut [i32], lscale: &mut [f64], rscale: &mut [f64],
              work: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::zggbal_(&(job as c_char), &(n as c_int), a.as_mut_ptr() as *mut _, &(lda as c_int),
                     b.as_mut_ptr() as *mut _, &(ldb as c_int), ilo.as_mut_ptr(), ihi.as_mut_ptr(),
                     lscale.as_mut_ptr(), rscale.as_mut_ptr(), work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn sggbak(job: u8, side: u8, n: usize, ilo: &[i32], ihi: &[i32], lscale: &[f32],
              rscale: &[f32], m: usize, v: &mut [f32], ldv: usize, info: &mut i32) {

    unsafe {
        ffi::sggbak_(&(job as c_char), &(side as c_char), &(n as c_int), ilo.as_ptr(),
                     ihi.as_ptr(), lscale.as_ptr(), rscale.as_ptr(), &(m as c_int), v.as_mut_ptr(),
                     &(ldv as c_int), info)
    }
}

#[inline]
pub fn dggbak(job: u8, side: u8, n: usize, ilo: &[i32], ihi: &[i32], lscale: &[f64],
              rscale: &[f64], m: usize, v: &mut [f64], ldv: usize, info: &mut i32) {

    unsafe {
        ffi::dggbak_(&(job as c_char), &(side as c_char), &(n as c_int), ilo.as_ptr(),
                     ihi.as_ptr(), lscale.as_ptr(), rscale.as_ptr(), &(m as c_int), v.as_mut_ptr(),
                     &(ldv as c_int), info)
    }
}

#[inline]
pub fn cggbak(job: u8, side: u8, n: usize, ilo: &[i32], ihi: &[i32], lscale: &[f32],
              rscale: &[f32], m: usize, v: &mut [c32], ldv: usize, info: &mut i32) {

    unsafe {
        ffi::cggbak_(&(job as c_char), &(side as c_char), &(n as c_int), ilo.as_ptr(),
                     ihi.as_ptr(), lscale.as_ptr(), rscale.as_ptr(), &(m as c_int),
                     v.as_mut_ptr() as *mut _, &(ldv as c_int), info)
    }
}

#[inline]
pub fn zggbak(job: u8, side: u8, n: usize, ilo: &[i32], ihi: &[i32], lscale: &[f64],
              rscale: &[f64], m: usize, v: &mut [c64], ldv: usize, info: &mut i32) {

    unsafe {
        ffi::zggbak_(&(job as c_char), &(side as c_char), &(n as c_int), ilo.as_ptr(),
                     ihi.as_ptr(), lscale.as_ptr(), rscale.as_ptr(), &(m as c_int),
                     v.as_mut_ptr() as *mut _, &(ldv as c_int), info)
    }
}

#[inline]
pub fn shgeqz(job: u8, compq: u8, compz: u8, n: usize, ilo: &[i32], ihi: &[i32], h: &mut [f32],
              ldh: usize, t: &mut [f32], ldt: usize, alphar: &mut [f32], alphai: &mut [f32],
              beta: &mut [f32], q: &mut [f32], ldq: usize, z: &mut [f32], ldz: usize,
              work: &mut [f32], lwork: usize, info: &mut i32) {

    unsafe {
        ffi::shgeqz_(&(job as c_char), &(compq as c_char), &(compz as c_char), &(n as c_int),
                     ilo.as_ptr(), ihi.as_ptr(), h.as_mut_ptr(), &(ldh as c_int), t.as_mut_ptr(),
                     &(ldt as c_int), alphar.as_mut_ptr(), alphai.as_mut_ptr(), beta.as_mut_ptr(),
                     q.as_mut_ptr(), &(ldq as c_int), z.as_mut_ptr(), &(ldz as c_int),
                     work.as_mut_ptr(), &(lwork as c_int), info)
    }
}

#[inline]
pub fn dhgeqz(job: u8, compq: u8, compz: u8, n: usize, ilo: &[i32], ihi: &[i32], h: &mut [f64],
              ldh: usize, t: &mut [f64], ldt: usize, alphar: &mut [f64], alphai: &mut [f64],
              beta: &mut [f64], q: &mut [f64], ldq: usize, z: &mut [f64], ldz: usize,
              work: &mut [f64], lwork: usize, info: &mut i32) {

    unsafe {
        ffi::dhgeqz_(&(job as c_char), &(compq as c_char), &(compz as c_char), &(n as c_int),
                     ilo.as_ptr(), ihi.as_ptr(), h.as_mut_ptr(), &(ldh as c_int), t.as_mut_ptr(),
                     &(ldt as c_int), alphar.as_mut_ptr(), alphai.as_mut_ptr(), beta.as_mut_ptr(),
                     q.as_mut_ptr(), &(ldq as c_int), z.as_mut_ptr(), &(ldz as c_int),
                     work.as_mut_ptr(), &(lwork as c_int), info)
    }
}

#[inline]
pub fn chgeqz(job: u8, compq: u8, compz: u8, n: usize, ilo: &[i32], ihi: &[i32], h: &mut [c32],
              ldh: usize, t: &mut [c32], ldt: usize, alpha: &mut [c32], beta: &mut [c32],
              q: &mut [c32], ldq: usize, z: &mut [c32], ldz: usize, work: &mut [c32], lwork: usize,
              rwork: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::chgeqz_(&(job as c_char), &(compq as c_char), &(compz as c_char), &(n as c_int),
                     ilo.as_ptr(), ihi.as_ptr(), h.as_mut_ptr() as *mut _, &(ldh as c_int),
                     t.as_mut_ptr() as *mut _, &(ldt as c_int), alpha.as_mut_ptr() as *mut _,
                     beta.as_mut_ptr() as *mut _, q.as_mut_ptr() as *mut _, &(ldq as c_int),
                     z.as_mut_ptr() as *mut _, &(ldz as c_int), work.as_mut_ptr() as *mut _,
                     &(lwork as c_int), rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zhgeqz(job: u8, compq: u8, compz: u8, n: usize, ilo: &[i32], ihi: &[i32], h: &mut [c64],
              ldh: usize, t: &mut [c64], ldt: usize, alpha: &mut [c64], beta: &mut [c64],
              q: &mut [c64], ldq: usize, z: &mut [c64], ldz: usize, work: &mut [c64], lwork: usize,
              rwork: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::zhgeqz_(&(job as c_char), &(compq as c_char), &(compz as c_char), &(n as c_int),
                     ilo.as_ptr(), ihi.as_ptr(), h.as_mut_ptr() as *mut _, &(ldh as c_int),
                     t.as_mut_ptr() as *mut _, &(ldt as c_int), alpha.as_mut_ptr() as *mut _,
                     beta.as_mut_ptr() as *mut _, q.as_mut_ptr() as *mut _, &(ldq as c_int),
                     z.as_mut_ptr() as *mut _, &(ldz as c_int), work.as_mut_ptr() as *mut _,
                     &(lwork as c_int), rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn stgevc(side: u8, howmny: u8, select: &[i32], n: usize, s: &[f32], lds: usize, p: &[f32],
              ldp: usize, vl: &mut [f32], ldvl: usize, vr: &mut [f32], ldvr: usize, mm: usize,
              m: &mut u32, work: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::stgevc_(&(side as c_char), &(howmny as c_char), select.as_ptr(), &(n as c_int),
                     s.as_ptr(), &(lds as c_int), p.as_ptr(), &(ldp as c_int), vl.as_mut_ptr(),
                     &(ldvl as c_int), vr.as_mut_ptr(), &(ldvr as c_int), &(mm as c_int),
                     m as *mut _ as *mut _, work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dtgevc(side: u8, howmny: u8, select: &[i32], n: usize, s: &[f64], lds: usize, p: &[f64],
              ldp: usize, vl: &mut [f64], ldvl: usize, vr: &mut [f64], ldvr: usize, mm: usize,
              m: &mut u32, work: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::dtgevc_(&(side as c_char), &(howmny as c_char), select.as_ptr(), &(n as c_int),
                     s.as_ptr(), &(lds as c_int), p.as_ptr(), &(ldp as c_int), vl.as_mut_ptr(),
                     &(ldvl as c_int), vr.as_mut_ptr(), &(ldvr as c_int), &(mm as c_int),
                     m as *mut _ as *mut _, work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn ctgevc(side: u8, howmny: u8, select: &[i32], n: usize, s: &[c32], lds: usize, p: &[c32],
              ldp: usize, vl: &mut [c32], ldvl: usize, vr: &mut [c32], ldvr: usize, mm: usize,
              m: &mut u32, work: &mut [c32], rwork: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::ctgevc_(&(side as c_char), &(howmny as c_char), select.as_ptr(), &(n as c_int),
                     s.as_ptr() as *const _, &(lds as c_int), p.as_ptr() as *const _,
                     &(ldp as c_int), vl.as_mut_ptr() as *mut _, &(ldvl as c_int),
                     vr.as_mut_ptr() as *mut _, &(ldvr as c_int), &(mm as c_int),
                     m as *mut _ as *mut _, work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn ztgevc(side: u8, howmny: u8, select: &[i32], n: usize, s: &[c64], lds: usize, p: &[c64],
              ldp: usize, vl: &mut [c64], ldvl: usize, vr: &mut [c64], ldvr: usize, mm: usize,
              m: &mut u32, work: &mut [c64], rwork: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::ztgevc_(&(side as c_char), &(howmny as c_char), select.as_ptr(), &(n as c_int),
                     s.as_ptr() as *const _, &(lds as c_int), p.as_ptr() as *const _,
                     &(ldp as c_int), vl.as_mut_ptr() as *mut _, &(ldvl as c_int),
                     vr.as_mut_ptr() as *mut _, &(ldvr as c_int), &(mm as c_int),
                     m as *mut _ as *mut _, work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn stgexc(wantq: &[i32], wantz: &[i32], n: usize, a: &mut [f32], lda: usize, b: &mut [f32],
              ldb: usize, q: &mut [f32], ldq: usize, z: &mut [f32], ldz: usize, ifst: &mut [i32],
              ilst: &mut [i32], work: &mut [f32], lwork: usize, info: &mut i32) {

    unsafe {
        ffi::stgexc_(wantq.as_ptr(), wantz.as_ptr(), &(n as c_int), a.as_mut_ptr(),
                     &(lda as c_int), b.as_mut_ptr(), &(ldb as c_int), q.as_mut_ptr(),
                     &(ldq as c_int), z.as_mut_ptr(), &(ldz as c_int), ifst.as_mut_ptr(),
                     ilst.as_mut_ptr(), work.as_mut_ptr(), &(lwork as c_int), info)
    }
}

#[inline]
pub fn dtgexc(wantq: &[i32], wantz: &[i32], n: usize, a: &mut [f64], lda: usize, b: &mut [f64],
              ldb: usize, q: &mut [f64], ldq: usize, z: &mut [f64], ldz: usize, ifst: &mut [i32],
              ilst: &mut [i32], work: &mut [f64], lwork: usize, info: &mut i32) {

    unsafe {
        ffi::dtgexc_(wantq.as_ptr(), wantz.as_ptr(), &(n as c_int), a.as_mut_ptr(),
                     &(lda as c_int), b.as_mut_ptr(), &(ldb as c_int), q.as_mut_ptr(),
                     &(ldq as c_int), z.as_mut_ptr(), &(ldz as c_int), ifst.as_mut_ptr(),
                     ilst.as_mut_ptr(), work.as_mut_ptr(), &(lwork as c_int), info)
    }
}

#[inline]
pub fn ctgexc(wantq: &[i32], wantz: &[i32], n: usize, a: &mut [c32], lda: usize, b: &mut [c32],
              ldb: usize, q: &mut [c32], ldq: usize, z: &mut [c32], ldz: usize, ifst: &[i32],
              ilst: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::ctgexc_(wantq.as_ptr(), wantz.as_ptr(), &(n as c_int), a.as_mut_ptr() as *mut _,
                     &(lda as c_int), b.as_mut_ptr() as *mut _, &(ldb as c_int),
                     q.as_mut_ptr() as *mut _, &(ldq as c_int), z.as_mut_ptr() as *mut _,
                     &(ldz as c_int), ifst.as_ptr(), ilst.as_mut_ptr(), info)
    }
}

#[inline]
pub fn ztgexc(wantq: &[i32], wantz: &[i32], n: usize, a: &mut [c64], lda: usize, b: &mut [c64],
              ldb: usize, q: &mut [c64], ldq: usize, z: &mut [c64], ldz: usize, ifst: &[i32],
              ilst: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::ztgexc_(wantq.as_ptr(), wantz.as_ptr(), &(n as c_int), a.as_mut_ptr() as *mut _,
                     &(lda as c_int), b.as_mut_ptr() as *mut _, &(ldb as c_int),
                     q.as_mut_ptr() as *mut _, &(ldq as c_int), z.as_mut_ptr() as *mut _,
                     &(ldz as c_int), ifst.as_ptr(), ilst.as_mut_ptr(), info)
    }
}

#[inline]
pub fn stgsen(ijob: &[i32], wantq: &[i32], wantz: &[i32], select: &[i32], n: usize, a: &mut [f32],
              lda: usize, b: &mut [f32], ldb: usize, alphar: &mut [f32], alphai: &mut [f32],
              beta: &mut [f32], q: &mut [f32], ldq: usize, z: &mut [f32], ldz: usize, m: &mut u32,
              pl: &mut [f32], pr: &mut [f32], dif: &mut [f32], work: &mut [f32], lwork: usize,
              iwork: &mut [i32], liwork: &[i32], info: &mut i32) {

    unsafe {
        ffi::stgsen_(ijob.as_ptr(), wantq.as_ptr(), wantz.as_ptr(), select.as_ptr(), &(n as c_int),
                     a.as_mut_ptr(), &(lda as c_int), b.as_mut_ptr(), &(ldb as c_int),
                     alphar.as_mut_ptr(), alphai.as_mut_ptr(), beta.as_mut_ptr(), q.as_mut_ptr(),
                     &(ldq as c_int), z.as_mut_ptr(), &(ldz as c_int), m as *mut _ as *mut _,
                     pl.as_mut_ptr(), pr.as_mut_ptr(), dif.as_mut_ptr(), work.as_mut_ptr(),
                     &(lwork as c_int), iwork.as_mut_ptr(), liwork.as_ptr(), info)
    }
}

#[inline]
pub fn dtgsen(ijob: &[i32], wantq: &[i32], wantz: &[i32], select: &[i32], n: usize, a: &mut [f64],
              lda: usize, b: &mut [f64], ldb: usize, alphar: &mut [f64], alphai: &mut [f64],
              beta: &mut [f64], q: &mut [f64], ldq: usize, z: &mut [f64], ldz: usize, m: &mut u32,
              pl: &mut [f64], pr: &mut [f64], dif: &mut [f64], work: &mut [f64], lwork: usize,
              iwork: &mut [i32], liwork: &[i32], info: &mut i32) {

    unsafe {
        ffi::dtgsen_(ijob.as_ptr(), wantq.as_ptr(), wantz.as_ptr(), select.as_ptr(), &(n as c_int),
                     a.as_mut_ptr(), &(lda as c_int), b.as_mut_ptr(), &(ldb as c_int),
                     alphar.as_mut_ptr(), alphai.as_mut_ptr(), beta.as_mut_ptr(), q.as_mut_ptr(),
                     &(ldq as c_int), z.as_mut_ptr(), &(ldz as c_int), m as *mut _ as *mut _,
                     pl.as_mut_ptr(), pr.as_mut_ptr(), dif.as_mut_ptr(), work.as_mut_ptr(),
                     &(lwork as c_int), iwork.as_mut_ptr(), liwork.as_ptr(), info)
    }
}

#[inline]
pub fn ctgsen(ijob: &[i32], wantq: &[i32], wantz: &[i32], select: &[i32], n: usize, a: &mut [c32],
              lda: usize, b: &mut [c32], ldb: usize, alpha: &mut [c32], beta: &mut [c32],
              q: &mut [c32], ldq: usize, z: &mut [c32], ldz: usize, m: &mut u32, pl: &mut [f32],
              pr: &mut [f32], dif: &mut [f32], work: &mut [c32], lwork: usize, iwork: &mut [i32],
              liwork: &[i32], info: &mut i32) {

    unsafe {
        ffi::ctgsen_(ijob.as_ptr(), wantq.as_ptr(), wantz.as_ptr(), select.as_ptr(), &(n as c_int),
                     a.as_mut_ptr() as *mut _, &(lda as c_int), b.as_mut_ptr() as *mut _,
                     &(ldb as c_int), alpha.as_mut_ptr() as *mut _, beta.as_mut_ptr() as *mut _,
                     q.as_mut_ptr() as *mut _, &(ldq as c_int), z.as_mut_ptr() as *mut _,
                     &(ldz as c_int), m as *mut _ as *mut _, pl.as_mut_ptr(), pr.as_mut_ptr(),
                     dif.as_mut_ptr(), work.as_mut_ptr() as *mut _, &(lwork as c_int),
                     iwork.as_mut_ptr(), liwork.as_ptr(), info)
    }
}

#[inline]
pub fn ztgsen(ijob: &[i32], wantq: &[i32], wantz: &[i32], select: &[i32], n: usize, a: &mut [c64],
              lda: usize, b: &mut [c64], ldb: usize, alpha: &mut [c64], beta: &mut [c64],
              q: &mut [c64], ldq: usize, z: &mut [c64], ldz: usize, m: &mut u32, pl: &mut [f64],
              pr: &mut [f64], dif: &mut [f64], work: &mut [c64], lwork: usize, iwork: &mut [i32],
              liwork: &[i32], info: &mut i32) {

    unsafe {
        ffi::ztgsen_(ijob.as_ptr(), wantq.as_ptr(), wantz.as_ptr(), select.as_ptr(), &(n as c_int),
                     a.as_mut_ptr() as *mut _, &(lda as c_int), b.as_mut_ptr() as *mut _,
                     &(ldb as c_int), alpha.as_mut_ptr() as *mut _, beta.as_mut_ptr() as *mut _,
                     q.as_mut_ptr() as *mut _, &(ldq as c_int), z.as_mut_ptr() as *mut _,
                     &(ldz as c_int), m as *mut _ as *mut _, pl.as_mut_ptr(), pr.as_mut_ptr(),
                     dif.as_mut_ptr(), work.as_mut_ptr() as *mut _, &(lwork as c_int),
                     iwork.as_mut_ptr(), liwork.as_ptr(), info)
    }
}

#[inline]
pub fn stgsyl(trans: u8, ijob: &[i32], m: usize, n: usize, a: &[f32], lda: usize, b: &[f32],
              ldb: usize, c: &mut [f32], ldc: usize, d: &[f32], ldd: usize, e: &[f32], lde: usize,
              f: &mut [f32], ldf: usize, scale: &mut [f32], dif: &mut [f32], work: &mut [f32],
              lwork: usize, iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::stgsyl_(&(trans as c_char), ijob.as_ptr(), &(m as c_int), &(n as c_int), a.as_ptr(),
                     &(lda as c_int), b.as_ptr(), &(ldb as c_int), c.as_mut_ptr(), &(ldc as c_int),
                     d.as_ptr(), &(ldd as c_int), e.as_ptr(), &(lde as c_int), f.as_mut_ptr(),
                     &(ldf as c_int), scale.as_mut_ptr(), dif.as_mut_ptr(), work.as_mut_ptr(),
                     &(lwork as c_int), iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dtgsyl(trans: u8, ijob: &[i32], m: usize, n: usize, a: &[f64], lda: usize, b: &[f64],
              ldb: usize, c: &mut [f64], ldc: usize, d: &[f64], ldd: usize, e: &[f64], lde: usize,
              f: &mut [f64], ldf: usize, scale: &mut [f64], dif: &mut [f64], work: &mut [f64],
              lwork: usize, iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::dtgsyl_(&(trans as c_char), ijob.as_ptr(), &(m as c_int), &(n as c_int), a.as_ptr(),
                     &(lda as c_int), b.as_ptr(), &(ldb as c_int), c.as_mut_ptr(), &(ldc as c_int),
                     d.as_ptr(), &(ldd as c_int), e.as_ptr(), &(lde as c_int), f.as_mut_ptr(),
                     &(ldf as c_int), scale.as_mut_ptr(), dif.as_mut_ptr(), work.as_mut_ptr(),
                     &(lwork as c_int), iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn ctgsyl(trans: u8, ijob: &[i32], m: usize, n: usize, a: &[c32], lda: usize, b: &[c32],
              ldb: usize, c: &mut [c32], ldc: usize, d: &[c32], ldd: usize, e: &[c32], lde: usize,
              f: &mut [c32], ldf: usize, scale: &mut [f32], dif: &mut [f32], work: &mut [c32],
              lwork: usize, iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::ctgsyl_(&(trans as c_char), ijob.as_ptr(), &(m as c_int), &(n as c_int),
                     a.as_ptr() as *const _, &(lda as c_int), b.as_ptr() as *const _,
                     &(ldb as c_int), c.as_mut_ptr() as *mut _, &(ldc as c_int),
                     d.as_ptr() as *const _, &(ldd as c_int), e.as_ptr() as *const _,
                     &(lde as c_int), f.as_mut_ptr() as *mut _, &(ldf as c_int),
                     scale.as_mut_ptr(), dif.as_mut_ptr(), work.as_mut_ptr() as *mut _,
                     &(lwork as c_int), iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn ztgsyl(trans: u8, ijob: &[i32], m: usize, n: usize, a: &[c64], lda: usize, b: &[c64],
              ldb: usize, c: &mut [c64], ldc: usize, d: &[c64], ldd: usize, e: &[c64], lde: usize,
              f: &mut [c64], ldf: usize, scale: &mut [f64], dif: &mut [f64], work: &mut [c64],
              lwork: usize, iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::ztgsyl_(&(trans as c_char), ijob.as_ptr(), &(m as c_int), &(n as c_int),
                     a.as_ptr() as *const _, &(lda as c_int), b.as_ptr() as *const _,
                     &(ldb as c_int), c.as_mut_ptr() as *mut _, &(ldc as c_int),
                     d.as_ptr() as *const _, &(ldd as c_int), e.as_ptr() as *const _,
                     &(lde as c_int), f.as_mut_ptr() as *mut _, &(ldf as c_int),
                     scale.as_mut_ptr(), dif.as_mut_ptr(), work.as_mut_ptr() as *mut _,
                     &(lwork as c_int), iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn stgsna(job: u8, howmny: u8, select: &[i32], n: usize, a: &[f32], lda: usize, b: &[f32],
              ldb: usize, vl: &[f32], ldvl: usize, vr: &[f32], ldvr: usize, s: &mut [f32],
              dif: &mut [f32], mm: usize, m: &mut u32, work: &mut [f32], lwork: usize,
              iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::stgsna_(&(job as c_char), &(howmny as c_char), select.as_ptr(), &(n as c_int),
                     a.as_ptr(), &(lda as c_int), b.as_ptr(), &(ldb as c_int), vl.as_ptr(),
                     &(ldvl as c_int), vr.as_ptr(), &(ldvr as c_int), s.as_mut_ptr(),
                     dif.as_mut_ptr(), &(mm as c_int), m as *mut _ as *mut _, work.as_mut_ptr(),
                     &(lwork as c_int), iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dtgsna(job: u8, howmny: u8, select: &[i32], n: usize, a: &[f64], lda: usize, b: &[f64],
              ldb: usize, vl: &[f64], ldvl: usize, vr: &[f64], ldvr: usize, s: &mut [f64],
              dif: &mut [f64], mm: usize, m: &mut u32, work: &mut [f64], lwork: usize,
              iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::dtgsna_(&(job as c_char), &(howmny as c_char), select.as_ptr(), &(n as c_int),
                     a.as_ptr(), &(lda as c_int), b.as_ptr(), &(ldb as c_int), vl.as_ptr(),
                     &(ldvl as c_int), vr.as_ptr(), &(ldvr as c_int), s.as_mut_ptr(),
                     dif.as_mut_ptr(), &(mm as c_int), m as *mut _ as *mut _, work.as_mut_ptr(),
                     &(lwork as c_int), iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn ctgsna(job: u8, howmny: u8, select: &[i32], n: usize, a: &[c32], lda: usize, b: &[c32],
              ldb: usize, vl: &[c32], ldvl: usize, vr: &[c32], ldvr: usize, s: &mut [f32],
              dif: &mut [f32], mm: usize, m: &mut u32, work: &mut [c32], lwork: usize,
              iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::ctgsna_(&(job as c_char), &(howmny as c_char), select.as_ptr(), &(n as c_int),
                     a.as_ptr() as *const _, &(lda as c_int), b.as_ptr() as *const _,
                     &(ldb as c_int), vl.as_ptr() as *const _, &(ldvl as c_int),
                     vr.as_ptr() as *const _, &(ldvr as c_int), s.as_mut_ptr(), dif.as_mut_ptr(),
                     &(mm as c_int), m as *mut _ as *mut _, work.as_mut_ptr() as *mut _,
                     &(lwork as c_int), iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn ztgsna(job: u8, howmny: u8, select: &[i32], n: usize, a: &[c64], lda: usize, b: &[c64],
              ldb: usize, vl: &[c64], ldvl: usize, vr: &[c64], ldvr: usize, s: &mut [f64],
              dif: &mut [f64], mm: usize, m: &mut u32, work: &mut [c64], lwork: usize,
              iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::ztgsna_(&(job as c_char), &(howmny as c_char), select.as_ptr(), &(n as c_int),
                     a.as_ptr() as *const _, &(lda as c_int), b.as_ptr() as *const _,
                     &(ldb as c_int), vl.as_ptr() as *const _, &(ldvl as c_int),
                     vr.as_ptr() as *const _, &(ldvr as c_int), s.as_mut_ptr(), dif.as_mut_ptr(),
                     &(mm as c_int), m as *mut _ as *mut _, work.as_mut_ptr() as *mut _,
                     &(lwork as c_int), iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn sggsvp(jobu: u8, jobv: u8, jobq: u8, m: usize, p: &[i32], n: usize, a: &mut [f32],
              lda: usize, b: &mut [f32], ldb: usize, tola: &[f32], tolb: &[f32], k: &mut u32,
              l: &mut [i32], u: &mut [f32], ldu: usize, v: &mut [f32], ldv: usize, q: &mut [f32],
              ldq: usize, iwork: &mut [i32], tau: &mut [f32], work: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::sggsvp_(&(jobu as c_char), &(jobv as c_char), &(jobq as c_char), &(m as c_int),
                     p.as_ptr(), &(n as c_int), a.as_mut_ptr(), &(lda as c_int), b.as_mut_ptr(),
                     &(ldb as c_int), tola.as_ptr(), tolb.as_ptr(), k as *mut _ as *mut _,
                     l.as_mut_ptr(), u.as_mut_ptr(), &(ldu as c_int), v.as_mut_ptr(),
                     &(ldv as c_int), q.as_mut_ptr(), &(ldq as c_int), iwork.as_mut_ptr(),
                     tau.as_mut_ptr(), work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dggsvp(jobu: u8, jobv: u8, jobq: u8, m: usize, p: &[i32], n: usize, a: &mut [f64],
              lda: usize, b: &mut [f64], ldb: usize, tola: &[f64], tolb: &[f64], k: &mut u32,
              l: &mut [i32], u: &mut [f64], ldu: usize, v: &mut [f64], ldv: usize, q: &mut [f64],
              ldq: usize, iwork: &mut [i32], tau: &mut [f64], work: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::dggsvp_(&(jobu as c_char), &(jobv as c_char), &(jobq as c_char), &(m as c_int),
                     p.as_ptr(), &(n as c_int), a.as_mut_ptr(), &(lda as c_int), b.as_mut_ptr(),
                     &(ldb as c_int), tola.as_ptr(), tolb.as_ptr(), k as *mut _ as *mut _,
                     l.as_mut_ptr(), u.as_mut_ptr(), &(ldu as c_int), v.as_mut_ptr(),
                     &(ldv as c_int), q.as_mut_ptr(), &(ldq as c_int), iwork.as_mut_ptr(),
                     tau.as_mut_ptr(), work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn cggsvp(jobu: u8, jobv: u8, jobq: u8, m: usize, p: &[i32], n: usize, a: &mut [c32],
              lda: usize, b: &mut [c32], ldb: usize, tola: &[f32], tolb: &[f32], k: &mut u32,
              l: &mut [i32], u: &mut [c32], ldu: usize, v: &mut [c32], ldv: usize, q: &mut [c32],
              ldq: usize, iwork: &mut [i32], rwork: &mut [f32], tau: &mut [c32], work: &mut [c32],
              info: &mut i32) {

    unsafe {
        ffi::cggsvp_(&(jobu as c_char), &(jobv as c_char), &(jobq as c_char), &(m as c_int),
                     p.as_ptr(), &(n as c_int), a.as_mut_ptr() as *mut _, &(lda as c_int),
                     b.as_mut_ptr() as *mut _, &(ldb as c_int), tola.as_ptr(), tolb.as_ptr(),
                     k as *mut _ as *mut _, l.as_mut_ptr(), u.as_mut_ptr() as *mut _,
                     &(ldu as c_int), v.as_mut_ptr() as *mut _, &(ldv as c_int),
                     q.as_mut_ptr() as *mut _, &(ldq as c_int), iwork.as_mut_ptr(),
                     rwork.as_mut_ptr(), tau.as_mut_ptr() as *mut _, work.as_mut_ptr() as *mut _,
                     info)
    }
}

#[inline]
pub fn zggsvp(jobu: u8, jobv: u8, jobq: u8, m: usize, p: &[i32], n: usize, a: &mut [c64],
              lda: usize, b: &mut [c64], ldb: usize, tola: &[f64], tolb: &[f64], k: &mut u32,
              l: &mut [i32], u: &mut [c64], ldu: usize, v: &mut [c64], ldv: usize, q: &mut [c64],
              ldq: usize, iwork: &mut [i32], rwork: &mut [f64], tau: &mut [c64], work: &mut [c64],
              info: &mut i32) {

    unsafe {
        ffi::zggsvp_(&(jobu as c_char), &(jobv as c_char), &(jobq as c_char), &(m as c_int),
                     p.as_ptr(), &(n as c_int), a.as_mut_ptr() as *mut _, &(lda as c_int),
                     b.as_mut_ptr() as *mut _, &(ldb as c_int), tola.as_ptr(), tolb.as_ptr(),
                     k as *mut _ as *mut _, l.as_mut_ptr(), u.as_mut_ptr() as *mut _,
                     &(ldu as c_int), v.as_mut_ptr() as *mut _, &(ldv as c_int),
                     q.as_mut_ptr() as *mut _, &(ldq as c_int), iwork.as_mut_ptr(),
                     rwork.as_mut_ptr(), tau.as_mut_ptr() as *mut _, work.as_mut_ptr() as *mut _,
                     info)
    }
}

#[inline]
pub fn stgsja(jobu: u8, jobv: u8, jobq: u8, m: usize, p: &[i32], n: usize, k: usize, l: &[i32],
              a: &mut [f32], lda: usize, b: &mut [f32], ldb: usize, tola: &[f32], tolb: &[f32],
              alpha: &mut [f32], beta: &mut [f32], u: &mut [f32], ldu: usize, v: &mut [f32],
              ldv: usize, q: &mut [f32], ldq: usize, work: &mut [f32], ncycle: &mut [i32],
              info: &mut i32) {

    unsafe {
        ffi::stgsja_(&(jobu as c_char), &(jobv as c_char), &(jobq as c_char), &(m as c_int),
                     p.as_ptr(), &(n as c_int), &(k as c_int), l.as_ptr(), a.as_mut_ptr(),
                     &(lda as c_int), b.as_mut_ptr(), &(ldb as c_int), tola.as_ptr(),
                     tolb.as_ptr(), alpha.as_mut_ptr(), beta.as_mut_ptr(), u.as_mut_ptr(),
                     &(ldu as c_int), v.as_mut_ptr(), &(ldv as c_int), q.as_mut_ptr(),
                     &(ldq as c_int), work.as_mut_ptr(), ncycle.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dtgsja(jobu: u8, jobv: u8, jobq: u8, m: usize, p: &[i32], n: usize, k: usize, l: &[i32],
              a: &mut [f64], lda: usize, b: &mut [f64], ldb: usize, tola: &[f64], tolb: &[f64],
              alpha: &mut [f64], beta: &mut [f64], u: &mut [f64], ldu: usize, v: &mut [f64],
              ldv: usize, q: &mut [f64], ldq: usize, work: &mut [f64], ncycle: &mut [i32],
              info: &mut i32) {

    unsafe {
        ffi::dtgsja_(&(jobu as c_char), &(jobv as c_char), &(jobq as c_char), &(m as c_int),
                     p.as_ptr(), &(n as c_int), &(k as c_int), l.as_ptr(), a.as_mut_ptr(),
                     &(lda as c_int), b.as_mut_ptr(), &(ldb as c_int), tola.as_ptr(),
                     tolb.as_ptr(), alpha.as_mut_ptr(), beta.as_mut_ptr(), u.as_mut_ptr(),
                     &(ldu as c_int), v.as_mut_ptr(), &(ldv as c_int), q.as_mut_ptr(),
                     &(ldq as c_int), work.as_mut_ptr(), ncycle.as_mut_ptr(), info)
    }
}

#[inline]
pub fn ctgsja(jobu: u8, jobv: u8, jobq: u8, m: usize, p: &[i32], n: usize, k: usize, l: &[i32],
              a: &mut [c32], lda: usize, b: &mut [c32], ldb: usize, tola: &[f32], tolb: &[f32],
              alpha: &mut [f32], beta: &mut [f32], u: &mut [c32], ldu: usize, v: &mut [c32],
              ldv: usize, q: &mut [c32], ldq: usize, work: &mut [c32], ncycle: &mut [i32],
              info: &mut i32) {

    unsafe {
        ffi::ctgsja_(&(jobu as c_char), &(jobv as c_char), &(jobq as c_char), &(m as c_int),
                     p.as_ptr(), &(n as c_int), &(k as c_int), l.as_ptr(),
                     a.as_mut_ptr() as *mut _, &(lda as c_int), b.as_mut_ptr() as *mut _,
                     &(ldb as c_int), tola.as_ptr(), tolb.as_ptr(), alpha.as_mut_ptr(),
                     beta.as_mut_ptr(), u.as_mut_ptr() as *mut _, &(ldu as c_int),
                     v.as_mut_ptr() as *mut _, &(ldv as c_int), q.as_mut_ptr() as *mut _,
                     &(ldq as c_int), work.as_mut_ptr() as *mut _, ncycle.as_mut_ptr(), info)
    }
}

#[inline]
pub fn ztgsja(jobu: u8, jobv: u8, jobq: u8, m: usize, p: &[i32], n: usize, k: usize, l: &[i32],
              a: &mut [c64], lda: usize, b: &mut [c64], ldb: usize, tola: &[f64], tolb: &[f64],
              alpha: &mut [f64], beta: &mut [f64], u: &mut [c64], ldu: usize, v: &mut [c64],
              ldv: usize, q: &mut [c64], ldq: usize, work: &mut [c64], ncycle: &mut [i32],
              info: &mut i32) {

    unsafe {
        ffi::ztgsja_(&(jobu as c_char), &(jobv as c_char), &(jobq as c_char), &(m as c_int),
                     p.as_ptr(), &(n as c_int), &(k as c_int), l.as_ptr(),
                     a.as_mut_ptr() as *mut _, &(lda as c_int), b.as_mut_ptr() as *mut _,
                     &(ldb as c_int), tola.as_ptr(), tolb.as_ptr(), alpha.as_mut_ptr(),
                     beta.as_mut_ptr(), u.as_mut_ptr() as *mut _, &(ldu as c_int),
                     v.as_mut_ptr() as *mut _, &(ldv as c_int), q.as_mut_ptr() as *mut _,
                     &(ldq as c_int), work.as_mut_ptr() as *mut _, ncycle.as_mut_ptr(), info)
    }
}

#[inline]
pub fn sgels(trans: u8, m: usize, n: usize, nrhs: usize, a: &mut [f32], lda: usize, b: &mut [f32],
             ldb: usize, work: &mut [f32], lwork: usize, info: &mut i32) {

    unsafe {
        ffi::sgels_(&(trans as c_char), &(m as c_int), &(n as c_int), &(nrhs as c_int),
                    a.as_mut_ptr(), &(lda as c_int), b.as_mut_ptr(), &(ldb as c_int),
                    work.as_mut_ptr(), &(lwork as c_int), info)
    }
}

#[inline]
pub fn dgels(trans: u8, m: usize, n: usize, nrhs: usize, a: &mut [f64], lda: usize, b: &mut [f64],
             ldb: usize, work: &mut [f64], lwork: usize, info: &mut i32) {

    unsafe {
        ffi::dgels_(&(trans as c_char), &(m as c_int), &(n as c_int), &(nrhs as c_int),
                    a.as_mut_ptr(), &(lda as c_int), b.as_mut_ptr(), &(ldb as c_int),
                    work.as_mut_ptr(), &(lwork as c_int), info)
    }
}

#[inline]
pub fn cgels(trans: u8, m: usize, n: usize, nrhs: usize, a: &mut [c32], lda: usize, b: &mut [c32],
             ldb: usize, work: &mut [c32], lwork: usize, info: &mut i32) {

    unsafe {
        ffi::cgels_(&(trans as c_char), &(m as c_int), &(n as c_int), &(nrhs as c_int),
                    a.as_mut_ptr() as *mut _, &(lda as c_int), b.as_mut_ptr() as *mut _,
                    &(ldb as c_int), work.as_mut_ptr() as *mut _, &(lwork as c_int), info)
    }
}

#[inline]
pub fn zgels(trans: u8, m: usize, n: usize, nrhs: usize, a: &mut [c64], lda: usize, b: &mut [c64],
             ldb: usize, work: &mut [c64], lwork: usize, info: &mut i32) {

    unsafe {
        ffi::zgels_(&(trans as c_char), &(m as c_int), &(n as c_int), &(nrhs as c_int),
                    a.as_mut_ptr() as *mut _, &(lda as c_int), b.as_mut_ptr() as *mut _,
                    &(ldb as c_int), work.as_mut_ptr() as *mut _, &(lwork as c_int), info)
    }
}

#[inline]
pub fn sgelsy(m: usize, n: usize, nrhs: usize, a: &mut [f32], lda: usize, b: &mut [f32],
              ldb: usize, jpvt: &mut [i32], rcond: &[f32], rank: &mut u32, work: &mut [f32],
              lwork: usize, info: &mut i32) {

    unsafe {
        ffi::sgelsy_(&(m as c_int), &(n as c_int), &(nrhs as c_int), a.as_mut_ptr(),
                     &(lda as c_int), b.as_mut_ptr(), &(ldb as c_int), jpvt.as_mut_ptr(),
                     rcond.as_ptr(), rank as *mut _ as *mut _, work.as_mut_ptr(),
                     &(lwork as c_int), info)
    }
}

#[inline]
pub fn dgelsy(m: usize, n: usize, nrhs: usize, a: &mut [f64], lda: usize, b: &mut [f64],
              ldb: usize, jpvt: &mut [i32], rcond: &[f64], rank: &mut u32, work: &mut [f64],
              lwork: usize, info: &mut i32) {

    unsafe {
        ffi::dgelsy_(&(m as c_int), &(n as c_int), &(nrhs as c_int), a.as_mut_ptr(),
                     &(lda as c_int), b.as_mut_ptr(), &(ldb as c_int), jpvt.as_mut_ptr(),
                     rcond.as_ptr(), rank as *mut _ as *mut _, work.as_mut_ptr(),
                     &(lwork as c_int), info)
    }
}

#[inline]
pub fn cgelsy(m: usize, n: usize, nrhs: usize, a: &mut [c32], lda: usize, b: &mut [c32],
              ldb: usize, jpvt: &mut [i32], rcond: &[f32], rank: &mut u32, work: &mut [c32],
              lwork: usize, rwork: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::cgelsy_(&(m as c_int), &(n as c_int), &(nrhs as c_int), a.as_mut_ptr() as *mut _,
                     &(lda as c_int), b.as_mut_ptr() as *mut _, &(ldb as c_int), jpvt.as_mut_ptr(),
                     rcond.as_ptr(), rank as *mut _ as *mut _, work.as_mut_ptr() as *mut _,
                     &(lwork as c_int), rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zgelsy(m: usize, n: usize, nrhs: usize, a: &mut [c64], lda: usize, b: &mut [c64],
              ldb: usize, jpvt: &mut [i32], rcond: &[f64], rank: &mut u32, work: &mut [c64],
              lwork: usize, rwork: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::zgelsy_(&(m as c_int), &(n as c_int), &(nrhs as c_int), a.as_mut_ptr() as *mut _,
                     &(lda as c_int), b.as_mut_ptr() as *mut _, &(ldb as c_int), jpvt.as_mut_ptr(),
                     rcond.as_ptr(), rank as *mut _ as *mut _, work.as_mut_ptr() as *mut _,
                     &(lwork as c_int), rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn sgelss(m: usize, n: usize, nrhs: usize, a: &mut [f32], lda: usize, b: &mut [f32],
              ldb: usize, s: &mut [f32], rcond: &[f32], rank: &mut u32, work: &mut [f32],
              lwork: usize, info: &mut i32) {

    unsafe {
        ffi::sgelss_(&(m as c_int), &(n as c_int), &(nrhs as c_int), a.as_mut_ptr(),
                     &(lda as c_int), b.as_mut_ptr(), &(ldb as c_int), s.as_mut_ptr(),
                     rcond.as_ptr(), rank as *mut _ as *mut _, work.as_mut_ptr(),
                     &(lwork as c_int), info)
    }
}

#[inline]
pub fn dgelss(m: usize, n: usize, nrhs: usize, a: &mut [f64], lda: usize, b: &mut [f64],
              ldb: usize, s: &mut [f64], rcond: &[f64], rank: &mut u32, work: &mut [f64],
              lwork: usize, info: &mut i32) {

    unsafe {
        ffi::dgelss_(&(m as c_int), &(n as c_int), &(nrhs as c_int), a.as_mut_ptr(),
                     &(lda as c_int), b.as_mut_ptr(), &(ldb as c_int), s.as_mut_ptr(),
                     rcond.as_ptr(), rank as *mut _ as *mut _, work.as_mut_ptr(),
                     &(lwork as c_int), info)
    }
}

#[inline]
pub fn cgelss(m: usize, n: usize, nrhs: usize, a: &mut [c32], lda: usize, b: &mut [c32],
              ldb: usize, s: &mut [f32], rcond: &[f32], rank: &mut u32, work: &mut [c32],
              lwork: usize, rwork: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::cgelss_(&(m as c_int), &(n as c_int), &(nrhs as c_int), a.as_mut_ptr() as *mut _,
                     &(lda as c_int), b.as_mut_ptr() as *mut _, &(ldb as c_int), s.as_mut_ptr(),
                     rcond.as_ptr(), rank as *mut _ as *mut _, work.as_mut_ptr() as *mut _,
                     &(lwork as c_int), rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zgelss(m: usize, n: usize, nrhs: usize, a: &mut [c64], lda: usize, b: &mut [c64],
              ldb: usize, s: &mut [f64], rcond: &[f64], rank: &mut u32, work: &mut [c64],
              lwork: usize, rwork: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::zgelss_(&(m as c_int), &(n as c_int), &(nrhs as c_int), a.as_mut_ptr() as *mut _,
                     &(lda as c_int), b.as_mut_ptr() as *mut _, &(ldb as c_int), s.as_mut_ptr(),
                     rcond.as_ptr(), rank as *mut _ as *mut _, work.as_mut_ptr() as *mut _,
                     &(lwork as c_int), rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn sgelsd(m: usize, n: usize, nrhs: usize, a: &[f32], lda: usize, b: &mut [f32], ldb: usize,
              s: &mut [f32], rcond: &[f32], rank: &mut u32, work: &mut [f32], lwork: usize,
              iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::sgelsd_(&(m as c_int), &(n as c_int), &(nrhs as c_int), a.as_ptr(), &(lda as c_int),
                     b.as_mut_ptr(), &(ldb as c_int), s.as_mut_ptr(), rcond.as_ptr(),
                     rank as *mut _ as *mut _, work.as_mut_ptr(), &(lwork as c_int),
                     iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dgelsd(m: usize, n: usize, nrhs: usize, a: &[f64], lda: usize, b: &mut [f64], ldb: usize,
              s: &mut [f64], rcond: &[f64], rank: &mut u32, work: &mut [f64], lwork: usize,
              iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::dgelsd_(&(m as c_int), &(n as c_int), &(nrhs as c_int), a.as_ptr(), &(lda as c_int),
                     b.as_mut_ptr(), &(ldb as c_int), s.as_mut_ptr(), rcond.as_ptr(),
                     rank as *mut _ as *mut _, work.as_mut_ptr(), &(lwork as c_int),
                     iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn cgelsd(m: usize, n: usize, nrhs: usize, a: &mut [c32], lda: usize, b: &mut [c32],
              ldb: usize, s: &mut [f32], rcond: &[f32], rank: &mut u32, work: &mut [c32],
              lwork: usize, rwork: &mut [f32], iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::cgelsd_(&(m as c_int), &(n as c_int), &(nrhs as c_int), a.as_mut_ptr() as *mut _,
                     &(lda as c_int), b.as_mut_ptr() as *mut _, &(ldb as c_int), s.as_mut_ptr(),
                     rcond.as_ptr(), rank as *mut _ as *mut _, work.as_mut_ptr() as *mut _,
                     &(lwork as c_int), rwork.as_mut_ptr(), iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zgelsd(m: usize, n: usize, nrhs: usize, a: &[c64], lda: usize, b: &mut [c64], ldb: usize,
              s: &mut [f64], rcond: &[f64], rank: &mut u32, work: &mut [c64], lwork: usize,
              rwork: &mut [f64], iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::zgelsd_(&(m as c_int), &(n as c_int), &(nrhs as c_int), a.as_ptr() as *const _,
                     &(lda as c_int), b.as_mut_ptr() as *mut _, &(ldb as c_int), s.as_mut_ptr(),
                     rcond.as_ptr(), rank as *mut _ as *mut _, work.as_mut_ptr() as *mut _,
                     &(lwork as c_int), rwork.as_mut_ptr(), iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn sgglse(m: usize, n: usize, p: &[i32], a: &mut [f32], lda: usize, b: &mut [f32], ldb: usize,
              c: &mut [f32], d: &mut [f32], x: &mut [f32], work: &mut [f32], lwork: usize,
              info: &mut i32) {

    unsafe {
        ffi::sgglse_(&(m as c_int), &(n as c_int), p.as_ptr(), a.as_mut_ptr(), &(lda as c_int),
                     b.as_mut_ptr(), &(ldb as c_int), c.as_mut_ptr(), d.as_mut_ptr(),
                     x.as_mut_ptr(), work.as_mut_ptr(), &(lwork as c_int), info)
    }
}

#[inline]
pub fn dgglse(m: usize, n: usize, p: &[i32], a: &mut [f64], lda: usize, b: &mut [f64], ldb: usize,
              c: &mut [f64], d: &mut [f64], x: &mut [f64], work: &mut [f64], lwork: usize,
              info: &mut i32) {

    unsafe {
        ffi::dgglse_(&(m as c_int), &(n as c_int), p.as_ptr(), a.as_mut_ptr(), &(lda as c_int),
                     b.as_mut_ptr(), &(ldb as c_int), c.as_mut_ptr(), d.as_mut_ptr(),
                     x.as_mut_ptr(), work.as_mut_ptr(), &(lwork as c_int), info)
    }
}

#[inline]
pub fn cgglse(m: usize, n: usize, p: &[i32], a: &mut [c32], lda: usize, b: &mut [c32], ldb: usize,
              c: &mut [c32], d: &mut [c32], x: &mut [c32], work: &mut [c32], lwork: usize,
              info: &mut i32) {

    unsafe {
        ffi::cgglse_(&(m as c_int), &(n as c_int), p.as_ptr(), a.as_mut_ptr() as *mut _,
                     &(lda as c_int), b.as_mut_ptr() as *mut _, &(ldb as c_int),
                     c.as_mut_ptr() as *mut _, d.as_mut_ptr() as *mut _, x.as_mut_ptr() as *mut _,
                     work.as_mut_ptr() as *mut _, &(lwork as c_int), info)
    }
}

#[inline]
pub fn zgglse(m: usize, n: usize, p: &[i32], a: &mut [c64], lda: usize, b: &mut [c64], ldb: usize,
              c: &mut [c64], d: &mut [c64], x: &mut [c64], work: &mut [c64], lwork: usize,
              info: &mut i32) {

    unsafe {
        ffi::zgglse_(&(m as c_int), &(n as c_int), p.as_ptr(), a.as_mut_ptr() as *mut _,
                     &(lda as c_int), b.as_mut_ptr() as *mut _, &(ldb as c_int),
                     c.as_mut_ptr() as *mut _, d.as_mut_ptr() as *mut _, x.as_mut_ptr() as *mut _,
                     work.as_mut_ptr() as *mut _, &(lwork as c_int), info)
    }
}

#[inline]
pub fn sggglm(n: usize, m: usize, p: &[i32], a: &mut [f32], lda: usize, b: &mut [f32], ldb: usize,
              d: &mut [f32], x: &mut [f32], y: &mut [f32], work: &mut [f32], lwork: usize,
              info: &mut i32) {

    unsafe {
        ffi::sggglm_(&(n as c_int), &(m as c_int), p.as_ptr(), a.as_mut_ptr(), &(lda as c_int),
                     b.as_mut_ptr(), &(ldb as c_int), d.as_mut_ptr(), x.as_mut_ptr(),
                     y.as_mut_ptr(), work.as_mut_ptr(), &(lwork as c_int), info)
    }
}

#[inline]
pub fn dggglm(n: usize, m: usize, p: &[i32], a: &mut [f64], lda: usize, b: &mut [f64], ldb: usize,
              d: &mut [f64], x: &mut [f64], y: &mut [f64], work: &mut [f64], lwork: usize,
              info: &mut i32) {

    unsafe {
        ffi::dggglm_(&(n as c_int), &(m as c_int), p.as_ptr(), a.as_mut_ptr(), &(lda as c_int),
                     b.as_mut_ptr(), &(ldb as c_int), d.as_mut_ptr(), x.as_mut_ptr(),
                     y.as_mut_ptr(), work.as_mut_ptr(), &(lwork as c_int), info)
    }
}

#[inline]
pub fn cggglm(n: usize, m: usize, p: &[i32], a: &mut [c32], lda: usize, b: &mut [c32], ldb: usize,
              d: &mut [c32], x: &mut [c32], y: &mut [c32], work: &mut [c32], lwork: usize,
              info: &mut i32) {

    unsafe {
        ffi::cggglm_(&(n as c_int), &(m as c_int), p.as_ptr(), a.as_mut_ptr() as *mut _,
                     &(lda as c_int), b.as_mut_ptr() as *mut _, &(ldb as c_int),
                     d.as_mut_ptr() as *mut _, x.as_mut_ptr() as *mut _, y.as_mut_ptr() as *mut _,
                     work.as_mut_ptr() as *mut _, &(lwork as c_int), info)
    }
}

#[inline]
pub fn zggglm(n: usize, m: usize, p: &[i32], a: &mut [c64], lda: usize, b: &mut [c64], ldb: usize,
              d: &mut [c64], x: &mut [c64], y: &mut [c64], work: &mut [c64], lwork: usize,
              info: &mut i32) {

    unsafe {
        ffi::zggglm_(&(n as c_int), &(m as c_int), p.as_ptr(), a.as_mut_ptr() as *mut _,
                     &(lda as c_int), b.as_mut_ptr() as *mut _, &(ldb as c_int),
                     d.as_mut_ptr() as *mut _, x.as_mut_ptr() as *mut _, y.as_mut_ptr() as *mut _,
                     work.as_mut_ptr() as *mut _, &(lwork as c_int), info)
    }
}

#[inline]
pub fn ssyev(jobz: u8, uplo: u8, n: usize, a: &mut [f32], lda: usize, w: &mut [f32],
             work: &mut [f32], lwork: usize, info: &mut i32) {

    unsafe {
        ffi::ssyev_(&(jobz as c_char), &(uplo as c_char), &(n as c_int), a.as_mut_ptr(),
                    &(lda as c_int), w.as_mut_ptr(), work.as_mut_ptr(), &(lwork as c_int), info)
    }
}

#[inline]
pub fn dsyev(jobz: u8, uplo: u8, n: usize, a: &mut [f64], lda: usize, w: &mut [f64],
             work: &mut [f64], lwork: usize, info: &mut i32) {

    unsafe {
        ffi::dsyev_(&(jobz as c_char), &(uplo as c_char), &(n as c_int), a.as_mut_ptr(),
                    &(lda as c_int), w.as_mut_ptr(), work.as_mut_ptr(), &(lwork as c_int), info)
    }
}

#[inline]
pub fn cheev(jobz: u8, uplo: u8, n: usize, a: &mut [c32], lda: usize, w: &mut [f32],
             work: &mut [c32], lwork: usize, rwork: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::cheev_(&(jobz as c_char), &(uplo as c_char), &(n as c_int), a.as_mut_ptr() as *mut _,
                    &(lda as c_int), w.as_mut_ptr(), work.as_mut_ptr() as *mut _,
                    &(lwork as c_int), rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zheev(jobz: u8, uplo: u8, n: usize, a: &mut [c64], lda: usize, w: &mut [f64],
             work: &mut [c64], lwork: usize, rwork: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::zheev_(&(jobz as c_char), &(uplo as c_char), &(n as c_int), a.as_mut_ptr() as *mut _,
                    &(lda as c_int), w.as_mut_ptr(), work.as_mut_ptr() as *mut _,
                    &(lwork as c_int), rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn ssyevd(jobz: u8, uplo: u8, n: usize, a: &mut [f32], lda: usize, w: &mut [f32],
              work: &mut [f32], lwork: usize, iwork: &mut [i32], liwork: &[i32], info: &mut i32) {

    unsafe {
        ffi::ssyevd_(&(jobz as c_char), &(uplo as c_char), &(n as c_int), a.as_mut_ptr(),
                     &(lda as c_int), w.as_mut_ptr(), work.as_mut_ptr(), &(lwork as c_int),
                     iwork.as_mut_ptr(), liwork.as_ptr(), info)
    }
}

#[inline]
pub fn dsyevd(jobz: u8, uplo: u8, n: usize, a: &mut [f64], lda: usize, w: &mut [f64],
              work: &mut [f64], lwork: usize, iwork: &mut [i32], liwork: &[i32], info: &mut i32) {

    unsafe {
        ffi::dsyevd_(&(jobz as c_char), &(uplo as c_char), &(n as c_int), a.as_mut_ptr(),
                     &(lda as c_int), w.as_mut_ptr(), work.as_mut_ptr(), &(lwork as c_int),
                     iwork.as_mut_ptr(), liwork.as_ptr(), info)
    }
}

#[inline]
pub fn cheevd(jobz: u8, uplo: u8, n: usize, a: &mut [c32], lda: usize, w: &mut [f32],
              work: &mut [c32], lwork: usize, rwork: &mut [f32], lrwork: &[i32], iwork: &mut [i32],
              liwork: &[i32], info: &mut i32) {

    unsafe {
        ffi::cheevd_(&(jobz as c_char), &(uplo as c_char), &(n as c_int), a.as_mut_ptr() as *mut _,
                     &(lda as c_int), w.as_mut_ptr(), work.as_mut_ptr() as *mut _,
                     &(lwork as c_int), rwork.as_mut_ptr(), lrwork.as_ptr(), iwork.as_mut_ptr(),
                     liwork.as_ptr(), info)
    }
}

#[inline]
pub fn zheevd(jobz: u8, uplo: u8, n: usize, a: &mut [c64], lda: usize, w: &mut [f64],
              work: &mut [c64], lwork: usize, rwork: &mut [f64], lrwork: &[i32], iwork: &mut [i32],
              liwork: &[i32], info: &mut i32) {

    unsafe {
        ffi::zheevd_(&(jobz as c_char), &(uplo as c_char), &(n as c_int), a.as_mut_ptr() as *mut _,
                     &(lda as c_int), w.as_mut_ptr(), work.as_mut_ptr() as *mut _,
                     &(lwork as c_int), rwork.as_mut_ptr(), lrwork.as_ptr(), iwork.as_mut_ptr(),
                     liwork.as_ptr(), info)
    }
}

#[inline]
pub fn ssyevx(jobz: u8, range: u8, uplo: u8, n: usize, a: &mut [f32], lda: usize, vl: &[f32],
              vu: &[f32], il: &[i32], iu: &[i32], abstol: &[f32], m: &mut u32, w: &mut [f32],
              z: &mut [f32], ldz: usize, work: &mut [f32], lwork: usize, iwork: &mut [i32],
              ifail: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::ssyevx_(&(jobz as c_char), &(range as c_char), &(uplo as c_char), &(n as c_int),
                     a.as_mut_ptr(), &(lda as c_int), vl.as_ptr(), vu.as_ptr(), il.as_ptr(),
                     iu.as_ptr(), abstol.as_ptr(), m as *mut _ as *mut _, w.as_mut_ptr(),
                     z.as_mut_ptr(), &(ldz as c_int), work.as_mut_ptr(), &(lwork as c_int),
                     iwork.as_mut_ptr(), ifail.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dsyevx(jobz: u8, range: u8, uplo: u8, n: usize, a: &mut [f64], lda: usize, vl: &[f64],
              vu: &[f64], il: &[i32], iu: &[i32], abstol: &[f64], m: &mut u32, w: &mut [f64],
              z: &mut [f64], ldz: usize, work: &mut [f64], lwork: usize, iwork: &mut [i32],
              ifail: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::dsyevx_(&(jobz as c_char), &(range as c_char), &(uplo as c_char), &(n as c_int),
                     a.as_mut_ptr(), &(lda as c_int), vl.as_ptr(), vu.as_ptr(), il.as_ptr(),
                     iu.as_ptr(), abstol.as_ptr(), m as *mut _ as *mut _, w.as_mut_ptr(),
                     z.as_mut_ptr(), &(ldz as c_int), work.as_mut_ptr(), &(lwork as c_int),
                     iwork.as_mut_ptr(), ifail.as_mut_ptr(), info)
    }
}

#[inline]
pub fn cheevx(jobz: u8, range: u8, uplo: u8, n: usize, a: &mut [c32], lda: usize, vl: &[f32],
              vu: &[f32], il: &[i32], iu: &[i32], abstol: &[f32], m: &mut u32, w: &mut [f32],
              z: &mut [c32], ldz: usize, work: &mut [c32], lwork: usize, rwork: &mut [f32],
              iwork: &mut [i32], ifail: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::cheevx_(&(jobz as c_char), &(range as c_char), &(uplo as c_char), &(n as c_int),
                     a.as_mut_ptr() as *mut _, &(lda as c_int), vl.as_ptr(), vu.as_ptr(),
                     il.as_ptr(), iu.as_ptr(), abstol.as_ptr(), m as *mut _ as *mut _,
                     w.as_mut_ptr(), z.as_mut_ptr() as *mut _, &(ldz as c_int),
                     work.as_mut_ptr() as *mut _, &(lwork as c_int), rwork.as_mut_ptr(),
                     iwork.as_mut_ptr(), ifail.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zheevx(jobz: u8, range: u8, uplo: u8, n: usize, a: &mut [c64], lda: usize, vl: &[f64],
              vu: &[f64], il: &[i32], iu: &[i32], abstol: &[f64], m: &mut u32, w: &mut [f64],
              z: &mut [c64], ldz: usize, work: &mut [c64], lwork: usize, rwork: &mut [f64],
              iwork: &mut [i32], ifail: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::zheevx_(&(jobz as c_char), &(range as c_char), &(uplo as c_char), &(n as c_int),
                     a.as_mut_ptr() as *mut _, &(lda as c_int), vl.as_ptr(), vu.as_ptr(),
                     il.as_ptr(), iu.as_ptr(), abstol.as_ptr(), m as *mut _ as *mut _,
                     w.as_mut_ptr(), z.as_mut_ptr() as *mut _, &(ldz as c_int),
                     work.as_mut_ptr() as *mut _, &(lwork as c_int), rwork.as_mut_ptr(),
                     iwork.as_mut_ptr(), ifail.as_mut_ptr(), info)
    }
}

#[inline]
pub fn ssyevr(jobz: u8, range: u8, uplo: u8, n: usize, a: &mut [f32], lda: usize, vl: &[f32],
              vu: &[f32], il: &[i32], iu: &[i32], abstol: &[f32], m: &mut u32, w: &mut [f32],
              z: &mut [f32], ldz: usize, isuppz: &mut [i32], work: &mut [f32], lwork: usize,
              iwork: &mut [i32], liwork: &[i32], info: &mut i32) {

    unsafe {
        ffi::ssyevr_(&(jobz as c_char), &(range as c_char), &(uplo as c_char), &(n as c_int),
                     a.as_mut_ptr(), &(lda as c_int), vl.as_ptr(), vu.as_ptr(), il.as_ptr(),
                     iu.as_ptr(), abstol.as_ptr(), m as *mut _ as *mut _, w.as_mut_ptr(),
                     z.as_mut_ptr(), &(ldz as c_int), isuppz.as_mut_ptr(), work.as_mut_ptr(),
                     &(lwork as c_int), iwork.as_mut_ptr(), liwork.as_ptr(), info)
    }
}

#[inline]
pub fn dsyevr(jobz: u8, range: u8, uplo: u8, n: usize, a: &mut [f64], lda: usize, vl: &[f64],
              vu: &[f64], il: &[i32], iu: &[i32], abstol: &[f64], m: &mut u32, w: &mut [f64],
              z: &mut [f64], ldz: usize, isuppz: &mut [i32], work: &mut [f64], lwork: usize,
              iwork: &mut [i32], liwork: &[i32], info: &mut i32) {

    unsafe {
        ffi::dsyevr_(&(jobz as c_char), &(range as c_char), &(uplo as c_char), &(n as c_int),
                     a.as_mut_ptr(), &(lda as c_int), vl.as_ptr(), vu.as_ptr(), il.as_ptr(),
                     iu.as_ptr(), abstol.as_ptr(), m as *mut _ as *mut _, w.as_mut_ptr(),
                     z.as_mut_ptr(), &(ldz as c_int), isuppz.as_mut_ptr(), work.as_mut_ptr(),
                     &(lwork as c_int), iwork.as_mut_ptr(), liwork.as_ptr(), info)
    }
}

#[inline]
pub fn cheevr(jobz: u8, range: u8, uplo: u8, n: usize, a: &mut [c32], lda: usize, vl: &[f32],
              vu: &[f32], il: &[i32], iu: &[i32], abstol: &[f32], m: &mut u32, w: &mut [f32],
              z: &mut [c32], ldz: usize, isuppz: &mut [i32], work: &mut [c32], lwork: usize,
              rwork: &mut [f32], lrwork: &[i32], iwork: &mut [i32], liwork: &[i32],
              info: &mut i32) {

    unsafe {
        ffi::cheevr_(&(jobz as c_char), &(range as c_char), &(uplo as c_char), &(n as c_int),
                     a.as_mut_ptr() as *mut _, &(lda as c_int), vl.as_ptr(), vu.as_ptr(),
                     il.as_ptr(), iu.as_ptr(), abstol.as_ptr(), m as *mut _ as *mut _,
                     w.as_mut_ptr(), z.as_mut_ptr() as *mut _, &(ldz as c_int),
                     isuppz.as_mut_ptr(), work.as_mut_ptr() as *mut _, &(lwork as c_int),
                     rwork.as_mut_ptr(), lrwork.as_ptr(), iwork.as_mut_ptr(), liwork.as_ptr(),
                     info)
    }
}

#[inline]
pub fn zheevr(jobz: u8, range: u8, uplo: u8, n: usize, a: &mut [c64], lda: usize, vl: &[f64],
              vu: &[f64], il: &[i32], iu: &[i32], abstol: &[f64], m: &mut u32, w: &mut [f64],
              z: &mut [c64], ldz: usize, isuppz: &mut [i32], work: &mut [c64], lwork: usize,
              rwork: &mut [f64], lrwork: &[i32], iwork: &mut [i32], liwork: &[i32],
              info: &mut i32) {

    unsafe {
        ffi::zheevr_(&(jobz as c_char), &(range as c_char), &(uplo as c_char), &(n as c_int),
                     a.as_mut_ptr() as *mut _, &(lda as c_int), vl.as_ptr(), vu.as_ptr(),
                     il.as_ptr(), iu.as_ptr(), abstol.as_ptr(), m as *mut _ as *mut _,
                     w.as_mut_ptr(), z.as_mut_ptr() as *mut _, &(ldz as c_int),
                     isuppz.as_mut_ptr(), work.as_mut_ptr() as *mut _, &(lwork as c_int),
                     rwork.as_mut_ptr(), lrwork.as_ptr(), iwork.as_mut_ptr(), liwork.as_ptr(),
                     info)
    }
}

#[inline]
pub fn sspev(jobz: u8, uplo: u8, n: usize, ap: &mut [f32], w: &mut [f32], z: &mut [f32],
             ldz: usize, work: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::sspev_(&(jobz as c_char), &(uplo as c_char), &(n as c_int), ap.as_mut_ptr(),
                    w.as_mut_ptr(), z.as_mut_ptr(), &(ldz as c_int), work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dspev(jobz: u8, uplo: u8, n: usize, ap: &mut [f64], w: &mut [f64], z: &mut [f64],
             ldz: usize, work: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::dspev_(&(jobz as c_char), &(uplo as c_char), &(n as c_int), ap.as_mut_ptr(),
                    w.as_mut_ptr(), z.as_mut_ptr(), &(ldz as c_int), work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn chpev(jobz: u8, uplo: u8, n: usize, ap: &mut [c32], w: &mut [f32], z: &mut [c32],
             ldz: usize, work: &mut [c32], rwork: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::chpev_(&(jobz as c_char), &(uplo as c_char), &(n as c_int), ap.as_mut_ptr() as *mut _,
                    w.as_mut_ptr(), z.as_mut_ptr() as *mut _, &(ldz as c_int),
                    work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zhpev(jobz: u8, uplo: u8, n: usize, ap: &mut [c64], w: &mut [f64], z: &mut [c64],
             ldz: usize, work: &mut [c64], rwork: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::zhpev_(&(jobz as c_char), &(uplo as c_char), &(n as c_int), ap.as_mut_ptr() as *mut _,
                    w.as_mut_ptr(), z.as_mut_ptr() as *mut _, &(ldz as c_int),
                    work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn sspevd(jobz: u8, uplo: u8, n: usize, ap: &mut [f32], w: &mut [f32], z: &mut [f32],
              ldz: usize, work: &mut [f32], lwork: usize, iwork: &mut [i32], liwork: &[i32],
              info: &mut i32) {

    unsafe {
        ffi::sspevd_(&(jobz as c_char), &(uplo as c_char), &(n as c_int), ap.as_mut_ptr(),
                     w.as_mut_ptr(), z.as_mut_ptr(), &(ldz as c_int), work.as_mut_ptr(),
                     &(lwork as c_int), iwork.as_mut_ptr(), liwork.as_ptr(), info)
    }
}

#[inline]
pub fn dspevd(jobz: u8, uplo: u8, n: usize, ap: &mut [f64], w: &mut [f64], z: &mut [f64],
              ldz: usize, work: &mut [f64], lwork: usize, iwork: &mut [i32], liwork: &[i32],
              info: &mut i32) {

    unsafe {
        ffi::dspevd_(&(jobz as c_char), &(uplo as c_char), &(n as c_int), ap.as_mut_ptr(),
                     w.as_mut_ptr(), z.as_mut_ptr(), &(ldz as c_int), work.as_mut_ptr(),
                     &(lwork as c_int), iwork.as_mut_ptr(), liwork.as_ptr(), info)
    }
}

#[inline]
pub fn chpevd(jobz: u8, uplo: u8, n: usize, ap: &mut [c32], w: &mut [f32], z: &mut [c32],
              ldz: usize, work: &mut [c32], lwork: usize, rwork: &mut [f32], lrwork: &[i32],
              iwork: &mut [i32], liwork: &[i32], info: &mut i32) {

    unsafe {
        ffi::chpevd_(&(jobz as c_char), &(uplo as c_char), &(n as c_int),
                     ap.as_mut_ptr() as *mut _, w.as_mut_ptr(), z.as_mut_ptr() as *mut _,
                     &(ldz as c_int), work.as_mut_ptr() as *mut _, &(lwork as c_int),
                     rwork.as_mut_ptr(), lrwork.as_ptr(), iwork.as_mut_ptr(), liwork.as_ptr(),
                     info)
    }
}

#[inline]
pub fn zhpevd(jobz: u8, uplo: u8, n: usize, ap: &mut [c64], w: &mut [f64], z: &mut [c64],
              ldz: usize, work: &mut [c64], lwork: usize, rwork: &mut [f64], lrwork: &[i32],
              iwork: &mut [i32], liwork: &[i32], info: &mut i32) {

    unsafe {
        ffi::zhpevd_(&(jobz as c_char), &(uplo as c_char), &(n as c_int),
                     ap.as_mut_ptr() as *mut _, w.as_mut_ptr(), z.as_mut_ptr() as *mut _,
                     &(ldz as c_int), work.as_mut_ptr() as *mut _, &(lwork as c_int),
                     rwork.as_mut_ptr(), lrwork.as_ptr(), iwork.as_mut_ptr(), liwork.as_ptr(),
                     info)
    }
}

#[inline]
pub fn sspevx(jobz: u8, range: u8, uplo: u8, n: usize, ap: &mut [f32], vl: &[f32], vu: &[f32],
              il: &[i32], iu: &[i32], abstol: &[f32], m: &mut u32, w: &mut [f32], z: &mut [f32],
              ldz: usize, work: &mut [f32], iwork: &mut [i32], ifail: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::sspevx_(&(jobz as c_char), &(range as c_char), &(uplo as c_char), &(n as c_int),
                     ap.as_mut_ptr(), vl.as_ptr(), vu.as_ptr(), il.as_ptr(), iu.as_ptr(),
                     abstol.as_ptr(), m as *mut _ as *mut _, w.as_mut_ptr(), z.as_mut_ptr(),
                     &(ldz as c_int), work.as_mut_ptr(), iwork.as_mut_ptr(), ifail.as_mut_ptr(),
                     info)
    }
}

#[inline]
pub fn dspevx(jobz: u8, range: u8, uplo: u8, n: usize, ap: &mut [f64], vl: &[f64], vu: &[f64],
              il: &[i32], iu: &[i32], abstol: &[f64], m: &mut u32, w: &mut [f64], z: &mut [f64],
              ldz: usize, work: &mut [f64], iwork: &mut [i32], ifail: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::dspevx_(&(jobz as c_char), &(range as c_char), &(uplo as c_char), &(n as c_int),
                     ap.as_mut_ptr(), vl.as_ptr(), vu.as_ptr(), il.as_ptr(), iu.as_ptr(),
                     abstol.as_ptr(), m as *mut _ as *mut _, w.as_mut_ptr(), z.as_mut_ptr(),
                     &(ldz as c_int), work.as_mut_ptr(), iwork.as_mut_ptr(), ifail.as_mut_ptr(),
                     info)
    }
}

#[inline]
pub fn chpevx(jobz: u8, range: u8, uplo: u8, n: usize, ap: &mut [c32], vl: &[f32], vu: &[f32],
              il: &[i32], iu: &[i32], abstol: &[f32], m: &mut u32, w: &mut [f32], z: &mut [c32],
              ldz: usize, work: &mut [c32], rwork: &mut [f32], iwork: &mut [i32],
              ifail: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::chpevx_(&(jobz as c_char), &(range as c_char), &(uplo as c_char), &(n as c_int),
                     ap.as_mut_ptr() as *mut _, vl.as_ptr(), vu.as_ptr(), il.as_ptr(), iu.as_ptr(),
                     abstol.as_ptr(), m as *mut _ as *mut _, w.as_mut_ptr(),
                     z.as_mut_ptr() as *mut _, &(ldz as c_int), work.as_mut_ptr() as *mut _,
                     rwork.as_mut_ptr(), iwork.as_mut_ptr(), ifail.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zhpevx(jobz: u8, range: u8, uplo: u8, n: usize, ap: &mut [c64], vl: &[f64], vu: &[f64],
              il: &[i32], iu: &[i32], abstol: &[f64], m: &mut u32, w: &mut [f64], z: &mut [c64],
              ldz: usize, work: &mut [c64], rwork: &mut [f64], iwork: &mut [i32],
              ifail: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::zhpevx_(&(jobz as c_char), &(range as c_char), &(uplo as c_char), &(n as c_int),
                     ap.as_mut_ptr() as *mut _, vl.as_ptr(), vu.as_ptr(), il.as_ptr(), iu.as_ptr(),
                     abstol.as_ptr(), m as *mut _ as *mut _, w.as_mut_ptr(),
                     z.as_mut_ptr() as *mut _, &(ldz as c_int), work.as_mut_ptr() as *mut _,
                     rwork.as_mut_ptr(), iwork.as_mut_ptr(), ifail.as_mut_ptr(), info)
    }
}

#[inline]
pub fn ssbev(jobz: u8, uplo: u8, n: usize, kd: usize, ab: &mut [f32], ldab: usize, w: &mut [f32],
             z: &mut [f32], ldz: usize, work: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::ssbev_(&(jobz as c_char), &(uplo as c_char), &(n as c_int), &(kd as c_int),
                    ab.as_mut_ptr(), &(ldab as c_int), w.as_mut_ptr(), z.as_mut_ptr(),
                    &(ldz as c_int), work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dsbev(jobz: u8, uplo: u8, n: usize, kd: usize, ab: &mut [f64], ldab: usize, w: &mut [f64],
             z: &mut [f64], ldz: usize, work: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::dsbev_(&(jobz as c_char), &(uplo as c_char), &(n as c_int), &(kd as c_int),
                    ab.as_mut_ptr(), &(ldab as c_int), w.as_mut_ptr(), z.as_mut_ptr(),
                    &(ldz as c_int), work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn chbev(jobz: u8, uplo: u8, n: usize, kd: usize, ab: &mut [c32], ldab: usize, w: &mut [f32],
             z: &mut [c32], ldz: usize, work: &mut [c32], rwork: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::chbev_(&(jobz as c_char), &(uplo as c_char), &(n as c_int), &(kd as c_int),
                    ab.as_mut_ptr() as *mut _, &(ldab as c_int), w.as_mut_ptr(),
                    z.as_mut_ptr() as *mut _, &(ldz as c_int), work.as_mut_ptr() as *mut _,
                    rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zhbev(jobz: u8, uplo: u8, n: usize, kd: usize, ab: &mut [c64], ldab: usize, w: &mut [f64],
             z: &mut [c64], ldz: usize, work: &mut [c64], rwork: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::zhbev_(&(jobz as c_char), &(uplo as c_char), &(n as c_int), &(kd as c_int),
                    ab.as_mut_ptr() as *mut _, &(ldab as c_int), w.as_mut_ptr(),
                    z.as_mut_ptr() as *mut _, &(ldz as c_int), work.as_mut_ptr() as *mut _,
                    rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn ssbevd(jobz: u8, uplo: u8, n: usize, kd: usize, ab: &mut [f32], ldab: usize, w: &mut [f32],
              z: &mut [f32], ldz: usize, work: &mut [f32], lwork: usize, iwork: &mut [i32],
              liwork: &[i32], info: &mut i32) {

    unsafe {
        ffi::ssbevd_(&(jobz as c_char), &(uplo as c_char), &(n as c_int), &(kd as c_int),
                     ab.as_mut_ptr(), &(ldab as c_int), w.as_mut_ptr(), z.as_mut_ptr(),
                     &(ldz as c_int), work.as_mut_ptr(), &(lwork as c_int), iwork.as_mut_ptr(),
                     liwork.as_ptr(), info)
    }
}

#[inline]
pub fn dsbevd(jobz: u8, uplo: u8, n: usize, kd: usize, ab: &mut [f64], ldab: usize, w: &mut [f64],
              z: &mut [f64], ldz: usize, work: &mut [f64], lwork: usize, iwork: &mut [i32],
              liwork: &[i32], info: &mut i32) {

    unsafe {
        ffi::dsbevd_(&(jobz as c_char), &(uplo as c_char), &(n as c_int), &(kd as c_int),
                     ab.as_mut_ptr(), &(ldab as c_int), w.as_mut_ptr(), z.as_mut_ptr(),
                     &(ldz as c_int), work.as_mut_ptr(), &(lwork as c_int), iwork.as_mut_ptr(),
                     liwork.as_ptr(), info)
    }
}

#[inline]
pub fn chbevd(jobz: u8, uplo: u8, n: usize, kd: usize, ab: &mut [c32], ldab: usize, w: &mut [f32],
              z: &mut [c32], ldz: usize, work: &mut [c32], lwork: usize, rwork: &mut [f32],
              lrwork: &[i32], iwork: &mut [i32], liwork: &[i32], info: &mut i32) {

    unsafe {
        ffi::chbevd_(&(jobz as c_char), &(uplo as c_char), &(n as c_int), &(kd as c_int),
                     ab.as_mut_ptr() as *mut _, &(ldab as c_int), w.as_mut_ptr(),
                     z.as_mut_ptr() as *mut _, &(ldz as c_int), work.as_mut_ptr() as *mut _,
                     &(lwork as c_int), rwork.as_mut_ptr(), lrwork.as_ptr(), iwork.as_mut_ptr(),
                     liwork.as_ptr(), info)
    }
}

#[inline]
pub fn zhbevd(jobz: u8, uplo: u8, n: usize, kd: usize, ab: &mut [c64], ldab: usize, w: &mut [f64],
              z: &mut [c64], ldz: usize, work: &mut [c64], lwork: usize, rwork: &mut [f64],
              lrwork: &[i32], iwork: &mut [i32], liwork: &[i32], info: &mut i32) {

    unsafe {
        ffi::zhbevd_(&(jobz as c_char), &(uplo as c_char), &(n as c_int), &(kd as c_int),
                     ab.as_mut_ptr() as *mut _, &(ldab as c_int), w.as_mut_ptr(),
                     z.as_mut_ptr() as *mut _, &(ldz as c_int), work.as_mut_ptr() as *mut _,
                     &(lwork as c_int), rwork.as_mut_ptr(), lrwork.as_ptr(), iwork.as_mut_ptr(),
                     liwork.as_ptr(), info)
    }
}

#[inline]
pub fn ssbevx(jobz: u8, range: u8, uplo: u8, n: usize, kd: usize, ab: &mut [f32], ldab: usize,
              q: &mut [f32], ldq: usize, vl: &[f32], vu: &[f32], il: &[i32], iu: &[i32],
              abstol: &[f32], m: &mut u32, w: &mut [f32], z: &mut [f32], ldz: usize,
              work: &mut [f32], iwork: &mut [i32], ifail: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::ssbevx_(&(jobz as c_char), &(range as c_char), &(uplo as c_char), &(n as c_int),
                     &(kd as c_int), ab.as_mut_ptr(), &(ldab as c_int), q.as_mut_ptr(),
                     &(ldq as c_int), vl.as_ptr(), vu.as_ptr(), il.as_ptr(), iu.as_ptr(),
                     abstol.as_ptr(), m as *mut _ as *mut _, w.as_mut_ptr(), z.as_mut_ptr(),
                     &(ldz as c_int), work.as_mut_ptr(), iwork.as_mut_ptr(), ifail.as_mut_ptr(),
                     info)
    }
}

#[inline]
pub fn dsbevx(jobz: u8, range: u8, uplo: u8, n: usize, kd: usize, ab: &mut [f64], ldab: usize,
              q: &mut [f64], ldq: usize, vl: &[f64], vu: &[f64], il: &[i32], iu: &[i32],
              abstol: &[f64], m: &mut u32, w: &mut [f64], z: &mut [f64], ldz: usize,
              work: &mut [f64], iwork: &mut [i32], ifail: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::dsbevx_(&(jobz as c_char), &(range as c_char), &(uplo as c_char), &(n as c_int),
                     &(kd as c_int), ab.as_mut_ptr(), &(ldab as c_int), q.as_mut_ptr(),
                     &(ldq as c_int), vl.as_ptr(), vu.as_ptr(), il.as_ptr(), iu.as_ptr(),
                     abstol.as_ptr(), m as *mut _ as *mut _, w.as_mut_ptr(), z.as_mut_ptr(),
                     &(ldz as c_int), work.as_mut_ptr(), iwork.as_mut_ptr(), ifail.as_mut_ptr(),
                     info)
    }
}

#[inline]
pub fn chbevx(jobz: u8, range: u8, uplo: u8, n: usize, kd: usize, ab: &mut [c32], ldab: usize,
              q: &mut [c32], ldq: usize, vl: &[f32], vu: &[f32], il: &[i32], iu: &[i32],
              abstol: &[f32], m: &mut u32, w: &mut [f32], z: &mut [c32], ldz: usize,
              work: &mut [c32], rwork: &mut [f32], iwork: &mut [i32], ifail: &mut [i32],
              info: &mut i32) {

    unsafe {
        ffi::chbevx_(&(jobz as c_char), &(range as c_char), &(uplo as c_char), &(n as c_int),
                     &(kd as c_int), ab.as_mut_ptr() as *mut _, &(ldab as c_int),
                     q.as_mut_ptr() as *mut _, &(ldq as c_int), vl.as_ptr(), vu.as_ptr(),
                     il.as_ptr(), iu.as_ptr(), abstol.as_ptr(), m as *mut _ as *mut _,
                     w.as_mut_ptr(), z.as_mut_ptr() as *mut _, &(ldz as c_int),
                     work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), iwork.as_mut_ptr(),
                     ifail.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zhbevx(jobz: u8, range: u8, uplo: u8, n: usize, kd: usize, ab: &mut [c64], ldab: usize,
              q: &mut [c64], ldq: usize, vl: &[f64], vu: &[f64], il: &[i32], iu: &[i32],
              abstol: &[f64], m: &mut u32, w: &mut [f64], z: &mut [c64], ldz: usize,
              work: &mut [c64], rwork: &mut [f64], iwork: &mut [i32], ifail: &mut [i32],
              info: &mut i32) {

    unsafe {
        ffi::zhbevx_(&(jobz as c_char), &(range as c_char), &(uplo as c_char), &(n as c_int),
                     &(kd as c_int), ab.as_mut_ptr() as *mut _, &(ldab as c_int),
                     q.as_mut_ptr() as *mut _, &(ldq as c_int), vl.as_ptr(), vu.as_ptr(),
                     il.as_ptr(), iu.as_ptr(), abstol.as_ptr(), m as *mut _ as *mut _,
                     w.as_mut_ptr(), z.as_mut_ptr() as *mut _, &(ldz as c_int),
                     work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), iwork.as_mut_ptr(),
                     ifail.as_mut_ptr(), info)
    }
}

#[inline]
pub fn sstev(jobz: u8, n: usize, d: &mut [f32], e: &mut [f32], z: &mut [f32], ldz: usize,
             work: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::sstev_(&(jobz as c_char), &(n as c_int), d.as_mut_ptr(), e.as_mut_ptr(),
                    z.as_mut_ptr(), &(ldz as c_int), work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dstev(jobz: u8, n: usize, d: &mut [f64], e: &mut [f64], z: &mut [f64], ldz: usize,
             work: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::dstev_(&(jobz as c_char), &(n as c_int), d.as_mut_ptr(), e.as_mut_ptr(),
                    z.as_mut_ptr(), &(ldz as c_int), work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn sstevd(jobz: u8, n: usize, d: &mut [f32], e: &mut [f32], z: &mut [f32], ldz: usize,
              work: &mut [f32], lwork: usize, iwork: &mut [i32], liwork: &[i32], info: &mut i32) {

    unsafe {
        ffi::sstevd_(&(jobz as c_char), &(n as c_int), d.as_mut_ptr(), e.as_mut_ptr(),
                     z.as_mut_ptr(), &(ldz as c_int), work.as_mut_ptr(), &(lwork as c_int),
                     iwork.as_mut_ptr(), liwork.as_ptr(), info)
    }
}

#[inline]
pub fn dstevd(jobz: u8, n: usize, d: &mut [f64], e: &mut [f64], z: &mut [f64], ldz: usize,
              work: &mut [f64], lwork: usize, iwork: &mut [i32], liwork: &[i32], info: &mut i32) {

    unsafe {
        ffi::dstevd_(&(jobz as c_char), &(n as c_int), d.as_mut_ptr(), e.as_mut_ptr(),
                     z.as_mut_ptr(), &(ldz as c_int), work.as_mut_ptr(), &(lwork as c_int),
                     iwork.as_mut_ptr(), liwork.as_ptr(), info)
    }
}

#[inline]
pub fn sstevx(jobz: u8, range: u8, n: usize, d: &mut [f32], e: &mut [f32], vl: &[f32], vu: &[f32],
              il: &[i32], iu: &[i32], abstol: &[f32], m: &mut u32, w: &mut [f32], z: &mut [f32],
              ldz: usize, work: &mut [f32], iwork: &mut [i32], ifail: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::sstevx_(&(jobz as c_char), &(range as c_char), &(n as c_int), d.as_mut_ptr(),
                     e.as_mut_ptr(), vl.as_ptr(), vu.as_ptr(), il.as_ptr(), iu.as_ptr(),
                     abstol.as_ptr(), m as *mut _ as *mut _, w.as_mut_ptr(), z.as_mut_ptr(),
                     &(ldz as c_int), work.as_mut_ptr(), iwork.as_mut_ptr(), ifail.as_mut_ptr(),
                     info)
    }
}

#[inline]
pub fn dstevx(jobz: u8, range: u8, n: usize, d: &mut [f64], e: &mut [f64], vl: &[f64], vu: &[f64],
              il: &[i32], iu: &[i32], abstol: &[f64], m: &mut u32, w: &mut [f64], z: &mut [f64],
              ldz: usize, work: &mut [f64], iwork: &mut [i32], ifail: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::dstevx_(&(jobz as c_char), &(range as c_char), &(n as c_int), d.as_mut_ptr(),
                     e.as_mut_ptr(), vl.as_ptr(), vu.as_ptr(), il.as_ptr(), iu.as_ptr(),
                     abstol.as_ptr(), m as *mut _ as *mut _, w.as_mut_ptr(), z.as_mut_ptr(),
                     &(ldz as c_int), work.as_mut_ptr(), iwork.as_mut_ptr(), ifail.as_mut_ptr(),
                     info)
    }
}

#[inline]
pub fn sstevr(jobz: u8, range: u8, n: usize, d: &mut [f32], e: &mut [f32], vl: &[f32], vu: &[f32],
              il: &[i32], iu: &[i32], abstol: &[f32], m: &mut u32, w: &mut [f32], z: &mut [f32],
              ldz: usize, isuppz: &mut [i32], work: &mut [f32], lwork: usize, iwork: &mut [i32],
              liwork: &[i32], info: &mut i32) {

    unsafe {
        ffi::sstevr_(&(jobz as c_char), &(range as c_char), &(n as c_int), d.as_mut_ptr(),
                     e.as_mut_ptr(), vl.as_ptr(), vu.as_ptr(), il.as_ptr(), iu.as_ptr(),
                     abstol.as_ptr(), m as *mut _ as *mut _, w.as_mut_ptr(), z.as_mut_ptr(),
                     &(ldz as c_int), isuppz.as_mut_ptr(), work.as_mut_ptr(), &(lwork as c_int),
                     iwork.as_mut_ptr(), liwork.as_ptr(), info)
    }
}

#[inline]
pub fn dstevr(jobz: u8, range: u8, n: usize, d: &mut [f64], e: &mut [f64], vl: &[f64], vu: &[f64],
              il: &[i32], iu: &[i32], abstol: &[f64], m: &mut u32, w: &mut [f64], z: &mut [f64],
              ldz: usize, isuppz: &mut [i32], work: &mut [f64], lwork: usize, iwork: &mut [i32],
              liwork: &[i32], info: &mut i32) {

    unsafe {
        ffi::dstevr_(&(jobz as c_char), &(range as c_char), &(n as c_int), d.as_mut_ptr(),
                     e.as_mut_ptr(), vl.as_ptr(), vu.as_ptr(), il.as_ptr(), iu.as_ptr(),
                     abstol.as_ptr(), m as *mut _ as *mut _, w.as_mut_ptr(), z.as_mut_ptr(),
                     &(ldz as c_int), isuppz.as_mut_ptr(), work.as_mut_ptr(), &(lwork as c_int),
                     iwork.as_mut_ptr(), liwork.as_ptr(), info)
    }
}

#[inline]
pub fn sgees(jobvs: u8, sort: u8, select: Select2F32, n: usize, a: &mut [f32], lda: usize,
             sdim: &mut [i32], wr: &mut [f32], wi: &mut [f32], vs: &mut [f32], ldvs: usize,
             work: &mut [f32], lwork: usize, bwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::sgees_(&(jobvs as c_char), &(sort as c_char), transmute(select), &(n as c_int),
                    a.as_mut_ptr(), &(lda as c_int), sdim.as_mut_ptr(), wr.as_mut_ptr(),
                    wi.as_mut_ptr(), vs.as_mut_ptr(), &(ldvs as c_int), work.as_mut_ptr(),
                    &(lwork as c_int), bwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dgees(jobvs: u8, sort: u8, select: Select2F64, n: usize, a: &mut [f64], lda: usize,
             sdim: &mut [i32], wr: &mut [f64], wi: &mut [f64], vs: &mut [f64], ldvs: usize,
             work: &mut [f64], lwork: usize, bwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::dgees_(&(jobvs as c_char), &(sort as c_char), transmute(select), &(n as c_int),
                    a.as_mut_ptr(), &(lda as c_int), sdim.as_mut_ptr(), wr.as_mut_ptr(),
                    wi.as_mut_ptr(), vs.as_mut_ptr(), &(ldvs as c_int), work.as_mut_ptr(),
                    &(lwork as c_int), bwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn cgees(jobvs: u8, sort: u8, select: Select1C32, n: usize, a: &mut [c32], lda: usize,
             sdim: &mut [i32], w: &mut [c32], vs: &mut [c32], ldvs: usize, work: &mut [c32],
             lwork: usize, rwork: &mut [f32], bwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::cgees_(&(jobvs as c_char), &(sort as c_char), transmute(select), &(n as c_int),
                    a.as_mut_ptr() as *mut _, &(lda as c_int), sdim.as_mut_ptr(),
                    w.as_mut_ptr() as *mut _, vs.as_mut_ptr() as *mut _, &(ldvs as c_int),
                    work.as_mut_ptr() as *mut _, &(lwork as c_int), rwork.as_mut_ptr(),
                    bwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zgees(jobvs: u8, sort: u8, select: Select1C64, n: usize, a: &mut [c64], lda: usize,
             sdim: &mut [i32], w: &mut [c64], vs: &mut [c64], ldvs: usize, work: &mut [c64],
             lwork: usize, rwork: &mut [f64], bwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::zgees_(&(jobvs as c_char), &(sort as c_char), transmute(select), &(n as c_int),
                    a.as_mut_ptr() as *mut _, &(lda as c_int), sdim.as_mut_ptr(),
                    w.as_mut_ptr() as *mut _, vs.as_mut_ptr() as *mut _, &(ldvs as c_int),
                    work.as_mut_ptr() as *mut _, &(lwork as c_int), rwork.as_mut_ptr(),
                    bwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn sgeesx(jobvs: u8, sort: u8, select: Select2F32, sense: u8, n: usize, a: &mut [f32],
              lda: usize, sdim: &mut [i32], wr: &mut [f32], wi: &mut [f32], vs: &mut [f32],
              ldvs: usize, rconde: &mut [f32], rcondv: &mut [f32], work: &mut [f32], lwork: usize,
              iwork: &mut [i32], liwork: &[i32], bwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::sgeesx_(&(jobvs as c_char), &(sort as c_char), transmute(select), &(sense as c_char),
                     &(n as c_int), a.as_mut_ptr(), &(lda as c_int), sdim.as_mut_ptr(),
                     wr.as_mut_ptr(), wi.as_mut_ptr(), vs.as_mut_ptr(), &(ldvs as c_int),
                     rconde.as_mut_ptr(), rcondv.as_mut_ptr(), work.as_mut_ptr(),
                     &(lwork as c_int), iwork.as_mut_ptr(), liwork.as_ptr(), bwork.as_mut_ptr(),
                     info)
    }
}

#[inline]
pub fn dgeesx(jobvs: u8, sort: u8, select: Select2F64, sense: u8, n: usize, a: &mut [f64],
              lda: usize, sdim: &mut [i32], wr: &mut [f64], wi: &mut [f64], vs: &mut [f64],
              ldvs: usize, rconde: &mut [f64], rcondv: &mut [f64], work: &mut [f64], lwork: usize,
              iwork: &mut [i32], liwork: &[i32], bwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::dgeesx_(&(jobvs as c_char), &(sort as c_char), transmute(select), &(sense as c_char),
                     &(n as c_int), a.as_mut_ptr(), &(lda as c_int), sdim.as_mut_ptr(),
                     wr.as_mut_ptr(), wi.as_mut_ptr(), vs.as_mut_ptr(), &(ldvs as c_int),
                     rconde.as_mut_ptr(), rcondv.as_mut_ptr(), work.as_mut_ptr(),
                     &(lwork as c_int), iwork.as_mut_ptr(), liwork.as_ptr(), bwork.as_mut_ptr(),
                     info)
    }
}

#[inline]
pub fn cgeesx(jobvs: u8, sort: u8, select: Select1C32, sense: u8, n: usize, a: &mut [c32],
              lda: usize, sdim: &mut [i32], w: &mut [c32], vs: &mut [c32], ldvs: usize,
              rconde: &mut [f32], rcondv: &mut [f32], work: &mut [c32], lwork: usize,
              rwork: &mut [f32], bwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::cgeesx_(&(jobvs as c_char), &(sort as c_char), transmute(select), &(sense as c_char),
                     &(n as c_int), a.as_mut_ptr() as *mut _, &(lda as c_int), sdim.as_mut_ptr(),
                     w.as_mut_ptr() as *mut _, vs.as_mut_ptr() as *mut _, &(ldvs as c_int),
                     rconde.as_mut_ptr(), rcondv.as_mut_ptr(), work.as_mut_ptr() as *mut _,
                     &(lwork as c_int), rwork.as_mut_ptr(), bwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zgeesx(jobvs: u8, sort: u8, select: Select1C64, sense: u8, n: usize, a: &mut [c64],
              lda: usize, sdim: &mut [i32], w: &mut [c64], vs: &mut [c64], ldvs: usize,
              rconde: &mut [f64], rcondv: &mut [f64], work: &mut [c64], lwork: usize,
              rwork: &mut [f64], bwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::zgeesx_(&(jobvs as c_char), &(sort as c_char), transmute(select), &(sense as c_char),
                     &(n as c_int), a.as_mut_ptr() as *mut _, &(lda as c_int), sdim.as_mut_ptr(),
                     w.as_mut_ptr() as *mut _, vs.as_mut_ptr() as *mut _, &(ldvs as c_int),
                     rconde.as_mut_ptr(), rcondv.as_mut_ptr(), work.as_mut_ptr() as *mut _,
                     &(lwork as c_int), rwork.as_mut_ptr(), bwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn sgeev(jobvl: u8, jobvr: u8, n: usize, a: &mut [f32], lda: usize, wr: &mut [f32],
             wi: &mut [f32], vl: &mut [f32], ldvl: usize, vr: &mut [f32], ldvr: usize,
             work: &mut [f32], lwork: usize, info: &mut i32) {

    unsafe {
        ffi::sgeev_(&(jobvl as c_char), &(jobvr as c_char), &(n as c_int), a.as_mut_ptr(),
                    &(lda as c_int), wr.as_mut_ptr(), wi.as_mut_ptr(), vl.as_mut_ptr(),
                    &(ldvl as c_int), vr.as_mut_ptr(), &(ldvr as c_int), work.as_mut_ptr(),
                    &(lwork as c_int), info)
    }
}

#[inline]
pub fn dgeev(jobvl: u8, jobvr: u8, n: usize, a: &mut [f64], lda: usize, wr: &mut [f64],
             wi: &mut [f64], vl: &mut [f64], ldvl: usize, vr: &mut [f64], ldvr: usize,
             work: &mut [f64], lwork: usize, info: &mut i32) {

    unsafe {
        ffi::dgeev_(&(jobvl as c_char), &(jobvr as c_char), &(n as c_int), a.as_mut_ptr(),
                    &(lda as c_int), wr.as_mut_ptr(), wi.as_mut_ptr(), vl.as_mut_ptr(),
                    &(ldvl as c_int), vr.as_mut_ptr(), &(ldvr as c_int), work.as_mut_ptr(),
                    &(lwork as c_int), info)
    }
}

#[inline]
pub fn cgeev(jobvl: u8, jobvr: u8, n: usize, a: &mut [c32], lda: usize, w: &mut [c32],
             vl: &mut [c32], ldvl: usize, vr: &mut [c32], ldvr: usize, work: &mut [c32],
             lwork: usize, rwork: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::cgeev_(&(jobvl as c_char), &(jobvr as c_char), &(n as c_int),
                    a.as_mut_ptr() as *mut _, &(lda as c_int), w.as_mut_ptr() as *mut _,
                    vl.as_mut_ptr() as *mut _, &(ldvl as c_int), vr.as_mut_ptr() as *mut _,
                    &(ldvr as c_int), work.as_mut_ptr() as *mut _, &(lwork as c_int),
                    rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zgeev(jobvl: u8, jobvr: u8, n: usize, a: &mut [c64], lda: usize, w: &mut [c64],
             vl: &mut [c64], ldvl: usize, vr: &mut [c64], ldvr: usize, work: &mut [c64],
             lwork: usize, rwork: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::zgeev_(&(jobvl as c_char), &(jobvr as c_char), &(n as c_int),
                    a.as_mut_ptr() as *mut _, &(lda as c_int), w.as_mut_ptr() as *mut _,
                    vl.as_mut_ptr() as *mut _, &(ldvl as c_int), vr.as_mut_ptr() as *mut _,
                    &(ldvr as c_int), work.as_mut_ptr() as *mut _, &(lwork as c_int),
                    rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn sgeevx(balanc: u8, jobvl: u8, jobvr: u8, sense: u8, n: usize, a: &mut [f32], lda: usize,
              wr: &mut [f32], wi: &mut [f32], vl: &mut [f32], ldvl: usize, vr: &mut [f32],
              ldvr: usize, ilo: &mut [i32], ihi: &mut [i32], scale: &mut [f32], abnrm: &mut [f32],
              rconde: &mut [f32], rcondv: &mut [f32], work: &mut [f32], lwork: usize,
              iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::sgeevx_(&(balanc as c_char), &(jobvl as c_char), &(jobvr as c_char),
                     &(sense as c_char), &(n as c_int), a.as_mut_ptr(), &(lda as c_int),
                     wr.as_mut_ptr(), wi.as_mut_ptr(), vl.as_mut_ptr(), &(ldvl as c_int),
                     vr.as_mut_ptr(), &(ldvr as c_int), ilo.as_mut_ptr(), ihi.as_mut_ptr(),
                     scale.as_mut_ptr(), abnrm.as_mut_ptr(), rconde.as_mut_ptr(),
                     rcondv.as_mut_ptr(), work.as_mut_ptr(), &(lwork as c_int), iwork.as_mut_ptr(),
                     info)
    }
}

#[inline]
pub fn dgeevx(balanc: u8, jobvl: u8, jobvr: u8, sense: u8, n: usize, a: &mut [f64], lda: usize,
              wr: &mut [f64], wi: &mut [f64], vl: &mut [f64], ldvl: usize, vr: &mut [f64],
              ldvr: usize, ilo: &mut [i32], ihi: &mut [i32], scale: &mut [f64], abnrm: &mut [f64],
              rconde: &mut [f64], rcondv: &mut [f64], work: &mut [f64], lwork: usize,
              iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::dgeevx_(&(balanc as c_char), &(jobvl as c_char), &(jobvr as c_char),
                     &(sense as c_char), &(n as c_int), a.as_mut_ptr(), &(lda as c_int),
                     wr.as_mut_ptr(), wi.as_mut_ptr(), vl.as_mut_ptr(), &(ldvl as c_int),
                     vr.as_mut_ptr(), &(ldvr as c_int), ilo.as_mut_ptr(), ihi.as_mut_ptr(),
                     scale.as_mut_ptr(), abnrm.as_mut_ptr(), rconde.as_mut_ptr(),
                     rcondv.as_mut_ptr(), work.as_mut_ptr(), &(lwork as c_int), iwork.as_mut_ptr(),
                     info)
    }
}

#[inline]
pub fn cgeevx(balanc: u8, jobvl: u8, jobvr: u8, sense: u8, n: usize, a: &mut [c32], lda: usize,
              w: &mut [c32], vl: &mut [c32], ldvl: usize, vr: &mut [c32], ldvr: usize,
              ilo: &mut [i32], ihi: &mut [i32], scale: &mut [f32], abnrm: &mut [f32],
              rconde: &mut [f32], rcondv: &mut [f32], work: &mut [c32], lwork: usize,
              rwork: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::cgeevx_(&(balanc as c_char), &(jobvl as c_char), &(jobvr as c_char),
                     &(sense as c_char), &(n as c_int), a.as_mut_ptr() as *mut _, &(lda as c_int),
                     w.as_mut_ptr() as *mut _, vl.as_mut_ptr() as *mut _, &(ldvl as c_int),
                     vr.as_mut_ptr() as *mut _, &(ldvr as c_int), ilo.as_mut_ptr(),
                     ihi.as_mut_ptr(), scale.as_mut_ptr(), abnrm.as_mut_ptr(), rconde.as_mut_ptr(),
                     rcondv.as_mut_ptr(), work.as_mut_ptr() as *mut _, &(lwork as c_int),
                     rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zgeevx(balanc: u8, jobvl: u8, jobvr: u8, sense: u8, n: usize, a: &mut [c64], lda: usize,
              w: &mut [c64], vl: &mut [c64], ldvl: usize, vr: &mut [c64], ldvr: usize,
              ilo: &mut [i32], ihi: &mut [i32], scale: &mut [f64], abnrm: &mut [f64],
              rconde: &mut [f64], rcondv: &mut [f64], work: &mut [c64], lwork: usize,
              rwork: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::zgeevx_(&(balanc as c_char), &(jobvl as c_char), &(jobvr as c_char),
                     &(sense as c_char), &(n as c_int), a.as_mut_ptr() as *mut _, &(lda as c_int),
                     w.as_mut_ptr() as *mut _, vl.as_mut_ptr() as *mut _, &(ldvl as c_int),
                     vr.as_mut_ptr() as *mut _, &(ldvr as c_int), ilo.as_mut_ptr(),
                     ihi.as_mut_ptr(), scale.as_mut_ptr(), abnrm.as_mut_ptr(), rconde.as_mut_ptr(),
                     rcondv.as_mut_ptr(), work.as_mut_ptr() as *mut _, &(lwork as c_int),
                     rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn sgesvd(jobu: u8, jobvt: u8, m: usize, n: usize, a: &mut [f32], lda: usize, s: &mut [f32],
              u: &mut [f32], ldu: usize, vt: &mut [f32], ldvt: usize, work: &mut [f32],
              lwork: usize, info: &mut i32) {

    unsafe {
        ffi::sgesvd_(&(jobu as c_char), &(jobvt as c_char), &(m as c_int), &(n as c_int),
                     a.as_mut_ptr(), &(lda as c_int), s.as_mut_ptr(), u.as_mut_ptr(),
                     &(ldu as c_int), vt.as_mut_ptr(), &(ldvt as c_int), work.as_mut_ptr(),
                     &(lwork as c_int), info)
    }
}

#[inline]
pub fn dgesvd(jobu: u8, jobvt: u8, m: usize, n: usize, a: &mut [f64], lda: usize, s: &mut [f64],
              u: &mut [f64], ldu: usize, vt: &mut [f64], ldvt: usize, work: &mut [f64],
              lwork: usize, info: &mut i32) {

    unsafe {
        ffi::dgesvd_(&(jobu as c_char), &(jobvt as c_char), &(m as c_int), &(n as c_int),
                     a.as_mut_ptr(), &(lda as c_int), s.as_mut_ptr(), u.as_mut_ptr(),
                     &(ldu as c_int), vt.as_mut_ptr(), &(ldvt as c_int), work.as_mut_ptr(),
                     &(lwork as c_int), info)
    }
}

#[inline]
pub fn cgesvd(jobu: u8, jobvt: u8, m: usize, n: usize, a: &mut [c32], lda: usize, s: &mut [f32],
              u: &mut [c32], ldu: usize, vt: &mut [c32], ldvt: usize, work: &mut [c32],
              lwork: usize, rwork: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::cgesvd_(&(jobu as c_char), &(jobvt as c_char), &(m as c_int), &(n as c_int),
                     a.as_mut_ptr() as *mut _, &(lda as c_int), s.as_mut_ptr(),
                     u.as_mut_ptr() as *mut _, &(ldu as c_int), vt.as_mut_ptr() as *mut _,
                     &(ldvt as c_int), work.as_mut_ptr() as *mut _, &(lwork as c_int),
                     rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zgesvd(jobu: u8, jobvt: u8, m: usize, n: usize, a: &mut [c64], lda: usize, s: &mut [f64],
              u: &mut [c64], ldu: usize, vt: &mut [c64], ldvt: usize, work: &mut [c64],
              lwork: usize, rwork: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::zgesvd_(&(jobu as c_char), &(jobvt as c_char), &(m as c_int), &(n as c_int),
                     a.as_mut_ptr() as *mut _, &(lda as c_int), s.as_mut_ptr(),
                     u.as_mut_ptr() as *mut _, &(ldu as c_int), vt.as_mut_ptr() as *mut _,
                     &(ldvt as c_int), work.as_mut_ptr() as *mut _, &(lwork as c_int),
                     rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn sgesdd(jobz: u8, m: usize, n: usize, a: &mut [f32], lda: usize, s: &mut [f32],
              u: &mut [f32], ldu: usize, vt: &mut [f32], ldvt: usize, work: &mut [f32],
              lwork: usize, iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::sgesdd_(&(jobz as c_char), &(m as c_int), &(n as c_int), a.as_mut_ptr(),
                     &(lda as c_int), s.as_mut_ptr(), u.as_mut_ptr(), &(ldu as c_int),
                     vt.as_mut_ptr(), &(ldvt as c_int), work.as_mut_ptr(), &(lwork as c_int),
                     iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dgesdd(jobz: u8, m: usize, n: usize, a: &mut [f64], lda: usize, s: &mut [f64],
              u: &mut [f64], ldu: usize, vt: &mut [f64], ldvt: usize, work: &mut [f64],
              lwork: usize, iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::dgesdd_(&(jobz as c_char), &(m as c_int), &(n as c_int), a.as_mut_ptr(),
                     &(lda as c_int), s.as_mut_ptr(), u.as_mut_ptr(), &(ldu as c_int),
                     vt.as_mut_ptr(), &(ldvt as c_int), work.as_mut_ptr(), &(lwork as c_int),
                     iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn cgesdd(jobz: u8, m: usize, n: usize, a: &mut [c32], lda: usize, s: &mut [f32],
              u: &mut [c32], ldu: usize, vt: &mut [c32], ldvt: usize, work: &mut [c32],
              lwork: usize, rwork: &mut [f32], iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::cgesdd_(&(jobz as c_char), &(m as c_int), &(n as c_int), a.as_mut_ptr() as *mut _,
                     &(lda as c_int), s.as_mut_ptr(), u.as_mut_ptr() as *mut _, &(ldu as c_int),
                     vt.as_mut_ptr() as *mut _, &(ldvt as c_int), work.as_mut_ptr() as *mut _,
                     &(lwork as c_int), rwork.as_mut_ptr(), iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zgesdd(jobz: u8, m: usize, n: usize, a: &mut [c64], lda: usize, s: &mut [f64],
              u: &mut [c64], ldu: usize, vt: &mut [c64], ldvt: usize, work: &mut [c64],
              lwork: usize, rwork: &mut [f64], iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::zgesdd_(&(jobz as c_char), &(m as c_int), &(n as c_int), a.as_mut_ptr() as *mut _,
                     &(lda as c_int), s.as_mut_ptr(), u.as_mut_ptr() as *mut _, &(ldu as c_int),
                     vt.as_mut_ptr() as *mut _, &(ldvt as c_int), work.as_mut_ptr() as *mut _,
                     &(lwork as c_int), rwork.as_mut_ptr(), iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dgejsv(joba: &mut u8, jobu: &mut u8, jobv: &mut u8, jobr: &mut u8, jobt: &mut u8,
              jobp: &mut u8, m: usize, n: usize, a: &mut [f64], lda: usize, sva: &mut [f64],
              u: &mut [f64], ldu: usize, v: &mut [f64], ldv: usize, work: &mut [f64], lwork: usize,
              iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::dgejsv_(joba as *mut _ as *mut _, jobu as *mut _ as *mut _, jobv as *mut _ as *mut _,
                     jobr as *mut _ as *mut _, jobt as *mut _ as *mut _, jobp as *mut _ as *mut _,
                     &(m as c_int), &(n as c_int), a.as_mut_ptr(), &(lda as c_int),
                     sva.as_mut_ptr(), u.as_mut_ptr(), &(ldu as c_int), v.as_mut_ptr(),
                     &(ldv as c_int), work.as_mut_ptr(), &(lwork as c_int), iwork.as_mut_ptr(),
                     info)
    }
}

#[inline]
pub fn sgejsv(joba: &mut u8, jobu: &mut u8, jobv: &mut u8, jobr: &mut u8, jobt: &mut u8,
              jobp: &mut u8, m: usize, n: usize, a: &mut [f32], lda: usize, sva: &mut [f32],
              u: &mut [f32], ldu: usize, v: &mut [f32], ldv: usize, work: &mut [f32], lwork: usize,
              iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::sgejsv_(joba as *mut _ as *mut _, jobu as *mut _ as *mut _, jobv as *mut _ as *mut _,
                     jobr as *mut _ as *mut _, jobt as *mut _ as *mut _, jobp as *mut _ as *mut _,
                     &(m as c_int), &(n as c_int), a.as_mut_ptr(), &(lda as c_int),
                     sva.as_mut_ptr(), u.as_mut_ptr(), &(ldu as c_int), v.as_mut_ptr(),
                     &(ldv as c_int), work.as_mut_ptr(), &(lwork as c_int), iwork.as_mut_ptr(),
                     info)
    }
}

#[inline]
pub fn dgesvj(joba: u8, jobu: u8, jobv: u8, m: usize, n: usize, a: &mut [f64], lda: usize,
              sva: &mut [f64], mv: &[i32], v: &mut [f64], ldv: usize, work: &mut [f64],
              lwork: usize, info: &mut i32) {

    unsafe {
        ffi::dgesvj_(&(joba as c_char), &(jobu as c_char), &(jobv as c_char), &(m as c_int),
                     &(n as c_int), a.as_mut_ptr(), &(lda as c_int), sva.as_mut_ptr(), mv.as_ptr(),
                     v.as_mut_ptr(), &(ldv as c_int), work.as_mut_ptr(), &(lwork as c_int), info)
    }
}

#[inline]
pub fn sgesvj(joba: u8, jobu: u8, jobv: u8, m: usize, n: usize, a: &mut [f32], lda: usize,
              sva: &mut [f32], mv: &[i32], v: &mut [f32], ldv: usize, work: &mut [f32],
              lwork: usize, info: &mut i32) {

    unsafe {
        ffi::sgesvj_(&(joba as c_char), &(jobu as c_char), &(jobv as c_char), &(m as c_int),
                     &(n as c_int), a.as_mut_ptr(), &(lda as c_int), sva.as_mut_ptr(), mv.as_ptr(),
                     v.as_mut_ptr(), &(ldv as c_int), work.as_mut_ptr(), &(lwork as c_int), info)
    }
}

#[inline]
pub fn sggsvd(jobu: u8, jobv: u8, jobq: u8, m: usize, n: usize, p: &[i32], k: &mut u32,
              l: &mut [i32], a: &mut [f32], lda: usize, b: &mut [f32], ldb: usize,
              alpha: &mut [f32], beta: &mut [f32], u: &mut [f32], ldu: usize, v: &mut [f32],
              ldv: usize, q: &mut [f32], ldq: usize, work: &mut [f32], iwork: &mut [i32],
              info: &mut i32) {

    unsafe {
        ffi::sggsvd_(&(jobu as c_char), &(jobv as c_char), &(jobq as c_char), &(m as c_int),
                     &(n as c_int), p.as_ptr(), k as *mut _ as *mut _, l.as_mut_ptr(),
                     a.as_mut_ptr(), &(lda as c_int), b.as_mut_ptr(), &(ldb as c_int),
                     alpha.as_mut_ptr(), beta.as_mut_ptr(), u.as_mut_ptr(), &(ldu as c_int),
                     v.as_mut_ptr(), &(ldv as c_int), q.as_mut_ptr(), &(ldq as c_int),
                     work.as_mut_ptr(), iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dggsvd(jobu: u8, jobv: u8, jobq: u8, m: usize, n: usize, p: &[i32], k: &mut u32,
              l: &mut [i32], a: &mut [f64], lda: usize, b: &mut [f64], ldb: usize,
              alpha: &mut [f64], beta: &mut [f64], u: &mut [f64], ldu: usize, v: &mut [f64],
              ldv: usize, q: &mut [f64], ldq: usize, work: &mut [f64], iwork: &mut [i32],
              info: &mut i32) {

    unsafe {
        ffi::dggsvd_(&(jobu as c_char), &(jobv as c_char), &(jobq as c_char), &(m as c_int),
                     &(n as c_int), p.as_ptr(), k as *mut _ as *mut _, l.as_mut_ptr(),
                     a.as_mut_ptr(), &(lda as c_int), b.as_mut_ptr(), &(ldb as c_int),
                     alpha.as_mut_ptr(), beta.as_mut_ptr(), u.as_mut_ptr(), &(ldu as c_int),
                     v.as_mut_ptr(), &(ldv as c_int), q.as_mut_ptr(), &(ldq as c_int),
                     work.as_mut_ptr(), iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn cggsvd(jobu: u8, jobv: u8, jobq: u8, m: usize, n: usize, p: &[i32], k: &mut u32,
              l: &mut [i32], a: &mut [c32], lda: usize, b: &mut [c32], ldb: usize,
              alpha: &mut [f32], beta: &mut [f32], u: &mut [c32], ldu: usize, v: &mut [c32],
              ldv: usize, q: &mut [c32], ldq: usize, work: &mut [c32], rwork: &mut [f32],
              iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::cggsvd_(&(jobu as c_char), &(jobv as c_char), &(jobq as c_char), &(m as c_int),
                     &(n as c_int), p.as_ptr(), k as *mut _ as *mut _, l.as_mut_ptr(),
                     a.as_mut_ptr() as *mut _, &(lda as c_int), b.as_mut_ptr() as *mut _,
                     &(ldb as c_int), alpha.as_mut_ptr(), beta.as_mut_ptr(),
                     u.as_mut_ptr() as *mut _, &(ldu as c_int), v.as_mut_ptr() as *mut _,
                     &(ldv as c_int), q.as_mut_ptr() as *mut _, &(ldq as c_int),
                     work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zggsvd(jobu: u8, jobv: u8, jobq: u8, m: usize, n: usize, p: &[i32], k: &mut u32,
              l: &mut [i32], a: &mut [c64], lda: usize, b: &mut [c64], ldb: usize,
              alpha: &mut [f64], beta: &mut [f64], u: &mut [c64], ldu: usize, v: &mut [c64],
              ldv: usize, q: &mut [c64], ldq: usize, work: &mut [c64], rwork: &mut [f64],
              iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::zggsvd_(&(jobu as c_char), &(jobv as c_char), &(jobq as c_char), &(m as c_int),
                     &(n as c_int), p.as_ptr(), k as *mut _ as *mut _, l.as_mut_ptr(),
                     a.as_mut_ptr() as *mut _, &(lda as c_int), b.as_mut_ptr() as *mut _,
                     &(ldb as c_int), alpha.as_mut_ptr(), beta.as_mut_ptr(),
                     u.as_mut_ptr() as *mut _, &(ldu as c_int), v.as_mut_ptr() as *mut _,
                     &(ldv as c_int), q.as_mut_ptr() as *mut _, &(ldq as c_int),
                     work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(), iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn ssygv(itype: &[i32], jobz: u8, uplo: u8, n: usize, a: &mut [f32], lda: usize, b: &mut [f32],
             ldb: usize, w: &mut [f32], work: &mut [f32], lwork: usize, info: &mut i32) {

    unsafe {
        ffi::ssygv_(itype.as_ptr(), &(jobz as c_char), &(uplo as c_char), &(n as c_int),
                    a.as_mut_ptr(), &(lda as c_int), b.as_mut_ptr(), &(ldb as c_int),
                    w.as_mut_ptr(), work.as_mut_ptr(), &(lwork as c_int), info)
    }
}

#[inline]
pub fn dsygv(itype: &[i32], jobz: u8, uplo: u8, n: usize, a: &mut [f64], lda: usize, b: &mut [f64],
             ldb: usize, w: &mut [f64], work: &mut [f64], lwork: usize, info: &mut i32) {

    unsafe {
        ffi::dsygv_(itype.as_ptr(), &(jobz as c_char), &(uplo as c_char), &(n as c_int),
                    a.as_mut_ptr(), &(lda as c_int), b.as_mut_ptr(), &(ldb as c_int),
                    w.as_mut_ptr(), work.as_mut_ptr(), &(lwork as c_int), info)
    }
}

#[inline]
pub fn chegv(itype: &[i32], jobz: u8, uplo: u8, n: usize, a: &mut [c32], lda: usize, b: &mut [c32],
             ldb: usize, w: &mut [f32], work: &mut [c32], lwork: usize, rwork: &mut [f32],
             info: &mut i32) {

    unsafe {
        ffi::chegv_(itype.as_ptr(), &(jobz as c_char), &(uplo as c_char), &(n as c_int),
                    a.as_mut_ptr() as *mut _, &(lda as c_int), b.as_mut_ptr() as *mut _,
                    &(ldb as c_int), w.as_mut_ptr(), work.as_mut_ptr() as *mut _,
                    &(lwork as c_int), rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zhegv(itype: &[i32], jobz: u8, uplo: u8, n: usize, a: &mut [c64], lda: usize, b: &mut [c64],
             ldb: usize, w: &mut [f64], work: &mut [c64], lwork: usize, rwork: &mut [f64],
             info: &mut i32) {

    unsafe {
        ffi::zhegv_(itype.as_ptr(), &(jobz as c_char), &(uplo as c_char), &(n as c_int),
                    a.as_mut_ptr() as *mut _, &(lda as c_int), b.as_mut_ptr() as *mut _,
                    &(ldb as c_int), w.as_mut_ptr(), work.as_mut_ptr() as *mut _,
                    &(lwork as c_int), rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn ssygvd(itype: &[i32], jobz: u8, uplo: u8, n: usize, a: &mut [f32], lda: usize,
              b: &mut [f32], ldb: usize, w: &mut [f32], work: &mut [f32], lwork: usize,
              iwork: &mut [i32], liwork: &[i32], info: &mut i32) {

    unsafe {
        ffi::ssygvd_(itype.as_ptr(), &(jobz as c_char), &(uplo as c_char), &(n as c_int),
                     a.as_mut_ptr(), &(lda as c_int), b.as_mut_ptr(), &(ldb as c_int),
                     w.as_mut_ptr(), work.as_mut_ptr(), &(lwork as c_int), iwork.as_mut_ptr(),
                     liwork.as_ptr(), info)
    }
}

#[inline]
pub fn dsygvd(itype: &[i32], jobz: u8, uplo: u8, n: usize, a: &mut [f64], lda: usize,
              b: &mut [f64], ldb: usize, w: &mut [f64], work: &mut [f64], lwork: usize,
              iwork: &mut [i32], liwork: &[i32], info: &mut i32) {

    unsafe {
        ffi::dsygvd_(itype.as_ptr(), &(jobz as c_char), &(uplo as c_char), &(n as c_int),
                     a.as_mut_ptr(), &(lda as c_int), b.as_mut_ptr(), &(ldb as c_int),
                     w.as_mut_ptr(), work.as_mut_ptr(), &(lwork as c_int), iwork.as_mut_ptr(),
                     liwork.as_ptr(), info)
    }
}

#[inline]
pub fn chegvd(itype: &[i32], jobz: u8, uplo: u8, n: usize, a: &mut [c32], lda: usize,
              b: &mut [c32], ldb: usize, w: &mut [f32], work: &mut [c32], lwork: usize,
              rwork: &mut [f32], lrwork: &[i32], iwork: &mut [i32], liwork: &[i32],
              info: &mut i32) {

    unsafe {
        ffi::chegvd_(itype.as_ptr(), &(jobz as c_char), &(uplo as c_char), &(n as c_int),
                     a.as_mut_ptr() as *mut _, &(lda as c_int), b.as_mut_ptr() as *mut _,
                     &(ldb as c_int), w.as_mut_ptr(), work.as_mut_ptr() as *mut _,
                     &(lwork as c_int), rwork.as_mut_ptr(), lrwork.as_ptr(), iwork.as_mut_ptr(),
                     liwork.as_ptr(), info)
    }
}

#[inline]
pub fn zhegvd(itype: &[i32], jobz: u8, uplo: u8, n: usize, a: &mut [c64], lda: usize,
              b: &mut [c64], ldb: usize, w: &mut [f64], work: &mut [c64], lwork: usize,
              rwork: &mut [f64], lrwork: &[i32], iwork: &mut [i32], liwork: &[i32],
              info: &mut i32) {

    unsafe {
        ffi::zhegvd_(itype.as_ptr(), &(jobz as c_char), &(uplo as c_char), &(n as c_int),
                     a.as_mut_ptr() as *mut _, &(lda as c_int), b.as_mut_ptr() as *mut _,
                     &(ldb as c_int), w.as_mut_ptr(), work.as_mut_ptr() as *mut _,
                     &(lwork as c_int), rwork.as_mut_ptr(), lrwork.as_ptr(), iwork.as_mut_ptr(),
                     liwork.as_ptr(), info)
    }
}

#[inline]
pub fn ssygvx(itype: &[i32], jobz: u8, range: u8, uplo: u8, n: usize, a: &mut [f32], lda: usize,
              b: &mut [f32], ldb: usize, vl: &[f32], vu: &[f32], il: &[i32], iu: &[i32],
              abstol: &[f32], m: &mut u32, w: &mut [f32], z: &mut [f32], ldz: usize,
              work: &mut [f32], lwork: usize, iwork: &mut [i32], ifail: &mut [i32],
              info: &mut i32) {

    unsafe {
        ffi::ssygvx_(itype.as_ptr(), &(jobz as c_char), &(range as c_char), &(uplo as c_char),
                     &(n as c_int), a.as_mut_ptr(), &(lda as c_int), b.as_mut_ptr(),
                     &(ldb as c_int), vl.as_ptr(), vu.as_ptr(), il.as_ptr(), iu.as_ptr(),
                     abstol.as_ptr(), m as *mut _ as *mut _, w.as_mut_ptr(), z.as_mut_ptr(),
                     &(ldz as c_int), work.as_mut_ptr(), &(lwork as c_int), iwork.as_mut_ptr(),
                     ifail.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dsygvx(itype: &[i32], jobz: u8, range: u8, uplo: u8, n: usize, a: &mut [f64], lda: usize,
              b: &mut [f64], ldb: usize, vl: &[f64], vu: &[f64], il: &[i32], iu: &[i32],
              abstol: &[f64], m: &mut u32, w: &mut [f64], z: &mut [f64], ldz: usize,
              work: &mut [f64], lwork: usize, iwork: &mut [i32], ifail: &mut [i32],
              info: &mut i32) {

    unsafe {
        ffi::dsygvx_(itype.as_ptr(), &(jobz as c_char), &(range as c_char), &(uplo as c_char),
                     &(n as c_int), a.as_mut_ptr(), &(lda as c_int), b.as_mut_ptr(),
                     &(ldb as c_int), vl.as_ptr(), vu.as_ptr(), il.as_ptr(), iu.as_ptr(),
                     abstol.as_ptr(), m as *mut _ as *mut _, w.as_mut_ptr(), z.as_mut_ptr(),
                     &(ldz as c_int), work.as_mut_ptr(), &(lwork as c_int), iwork.as_mut_ptr(),
                     ifail.as_mut_ptr(), info)
    }
}

#[inline]
pub fn chegvx(itype: &[i32], jobz: u8, range: u8, uplo: u8, n: usize, a: &mut [c32], lda: usize,
              b: &mut [c32], ldb: usize, vl: &[f32], vu: &[f32], il: &[i32], iu: &[i32],
              abstol: &[f32], m: &mut u32, w: &mut [f32], z: &mut [c32], ldz: usize,
              work: &mut [c32], lwork: usize, rwork: &mut [f32], iwork: &mut [i32],
              ifail: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::chegvx_(itype.as_ptr(), &(jobz as c_char), &(range as c_char), &(uplo as c_char),
                     &(n as c_int), a.as_mut_ptr() as *mut _, &(lda as c_int),
                     b.as_mut_ptr() as *mut _, &(ldb as c_int), vl.as_ptr(), vu.as_ptr(),
                     il.as_ptr(), iu.as_ptr(), abstol.as_ptr(), m as *mut _ as *mut _,
                     w.as_mut_ptr(), z.as_mut_ptr() as *mut _, &(ldz as c_int),
                     work.as_mut_ptr() as *mut _, &(lwork as c_int), rwork.as_mut_ptr(),
                     iwork.as_mut_ptr(), ifail.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zhegvx(itype: &[i32], jobz: u8, range: u8, uplo: u8, n: usize, a: &mut [c64], lda: usize,
              b: &mut [c64], ldb: usize, vl: &[f64], vu: &[f64], il: &[i32], iu: &[i32],
              abstol: &[f64], m: &mut u32, w: &mut [f64], z: &mut [c64], ldz: usize,
              work: &mut [c64], lwork: usize, rwork: &mut [f64], iwork: &mut [i32],
              ifail: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::zhegvx_(itype.as_ptr(), &(jobz as c_char), &(range as c_char), &(uplo as c_char),
                     &(n as c_int), a.as_mut_ptr() as *mut _, &(lda as c_int),
                     b.as_mut_ptr() as *mut _, &(ldb as c_int), vl.as_ptr(), vu.as_ptr(),
                     il.as_ptr(), iu.as_ptr(), abstol.as_ptr(), m as *mut _ as *mut _,
                     w.as_mut_ptr(), z.as_mut_ptr() as *mut _, &(ldz as c_int),
                     work.as_mut_ptr() as *mut _, &(lwork as c_int), rwork.as_mut_ptr(),
                     iwork.as_mut_ptr(), ifail.as_mut_ptr(), info)
    }
}

#[inline]
pub fn sspgv(itype: &[i32], jobz: u8, uplo: u8, n: usize, ap: &mut [f32], bp: &mut [f32],
             w: &mut [f32], z: &mut [f32], ldz: usize, work: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::sspgv_(itype.as_ptr(), &(jobz as c_char), &(uplo as c_char), &(n as c_int),
                    ap.as_mut_ptr(), bp.as_mut_ptr(), w.as_mut_ptr(), z.as_mut_ptr(),
                    &(ldz as c_int), work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dspgv(itype: &[i32], jobz: u8, uplo: u8, n: usize, ap: &mut [f64], bp: &mut [f64],
             w: &mut [f64], z: &mut [f64], ldz: usize, work: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::dspgv_(itype.as_ptr(), &(jobz as c_char), &(uplo as c_char), &(n as c_int),
                    ap.as_mut_ptr(), bp.as_mut_ptr(), w.as_mut_ptr(), z.as_mut_ptr(),
                    &(ldz as c_int), work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn chpgv(itype: &[i32], jobz: u8, uplo: u8, n: usize, ap: &mut [c32], bp: &mut [c32],
             w: &mut [f32], z: &mut [c32], ldz: usize, work: &mut [c32], rwork: &mut [f32],
             info: &mut i32) {

    unsafe {
        ffi::chpgv_(itype.as_ptr(), &(jobz as c_char), &(uplo as c_char), &(n as c_int),
                    ap.as_mut_ptr() as *mut _, bp.as_mut_ptr() as *mut _, w.as_mut_ptr(),
                    z.as_mut_ptr() as *mut _, &(ldz as c_int), work.as_mut_ptr() as *mut _,
                    rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zhpgv(itype: &[i32], jobz: u8, uplo: u8, n: usize, ap: &mut [c64], bp: &mut [c64],
             w: &mut [f64], z: &mut [c64], ldz: usize, work: &mut [c64], rwork: &mut [f64],
             info: &mut i32) {

    unsafe {
        ffi::zhpgv_(itype.as_ptr(), &(jobz as c_char), &(uplo as c_char), &(n as c_int),
                    ap.as_mut_ptr() as *mut _, bp.as_mut_ptr() as *mut _, w.as_mut_ptr(),
                    z.as_mut_ptr() as *mut _, &(ldz as c_int), work.as_mut_ptr() as *mut _,
                    rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn sspgvd(itype: &[i32], jobz: u8, uplo: u8, n: usize, ap: &mut [f32], bp: &mut [f32],
              w: &mut [f32], z: &mut [f32], ldz: usize, work: &mut [f32], lwork: usize,
              iwork: &mut [i32], liwork: &[i32], info: &mut i32) {

    unsafe {
        ffi::sspgvd_(itype.as_ptr(), &(jobz as c_char), &(uplo as c_char), &(n as c_int),
                     ap.as_mut_ptr(), bp.as_mut_ptr(), w.as_mut_ptr(), z.as_mut_ptr(),
                     &(ldz as c_int), work.as_mut_ptr(), &(lwork as c_int), iwork.as_mut_ptr(),
                     liwork.as_ptr(), info)
    }
}

#[inline]
pub fn dspgvd(itype: &[i32], jobz: u8, uplo: u8, n: usize, ap: &mut [f64], bp: &mut [f64],
              w: &mut [f64], z: &mut [f64], ldz: usize, work: &mut [f64], lwork: usize,
              iwork: &mut [i32], liwork: &[i32], info: &mut i32) {

    unsafe {
        ffi::dspgvd_(itype.as_ptr(), &(jobz as c_char), &(uplo as c_char), &(n as c_int),
                     ap.as_mut_ptr(), bp.as_mut_ptr(), w.as_mut_ptr(), z.as_mut_ptr(),
                     &(ldz as c_int), work.as_mut_ptr(), &(lwork as c_int), iwork.as_mut_ptr(),
                     liwork.as_ptr(), info)
    }
}

#[inline]
pub fn chpgvd(itype: &[i32], jobz: u8, uplo: u8, n: usize, ap: &mut [c32], bp: &mut [c32],
              w: &mut [f32], z: &mut [c32], ldz: usize, work: &mut [c32], lwork: usize,
              rwork: &mut [f32], lrwork: &[i32], iwork: &mut [i32], liwork: &[i32],
              info: &mut i32) {

    unsafe {
        ffi::chpgvd_(itype.as_ptr(), &(jobz as c_char), &(uplo as c_char), &(n as c_int),
                     ap.as_mut_ptr() as *mut _, bp.as_mut_ptr() as *mut _, w.as_mut_ptr(),
                     z.as_mut_ptr() as *mut _, &(ldz as c_int), work.as_mut_ptr() as *mut _,
                     &(lwork as c_int), rwork.as_mut_ptr(), lrwork.as_ptr(), iwork.as_mut_ptr(),
                     liwork.as_ptr(), info)
    }
}

#[inline]
pub fn zhpgvd(itype: &[i32], jobz: u8, uplo: u8, n: usize, ap: &mut [c64], bp: &mut [c64],
              w: &mut [f64], z: &mut [c64], ldz: usize, work: &mut [c64], lwork: usize,
              rwork: &mut [f64], lrwork: &[i32], iwork: &mut [i32], liwork: &[i32],
              info: &mut i32) {

    unsafe {
        ffi::zhpgvd_(itype.as_ptr(), &(jobz as c_char), &(uplo as c_char), &(n as c_int),
                     ap.as_mut_ptr() as *mut _, bp.as_mut_ptr() as *mut _, w.as_mut_ptr(),
                     z.as_mut_ptr() as *mut _, &(ldz as c_int), work.as_mut_ptr() as *mut _,
                     &(lwork as c_int), rwork.as_mut_ptr(), lrwork.as_ptr(), iwork.as_mut_ptr(),
                     liwork.as_ptr(), info)
    }
}

#[inline]
pub fn sspgvx(itype: &[i32], jobz: u8, range: u8, uplo: u8, n: usize, ap: &mut [f32],
              bp: &mut [f32], vl: &[f32], vu: &[f32], il: &[i32], iu: &[i32], abstol: &[f32],
              m: &mut u32, w: &mut [f32], z: &mut [f32], ldz: usize, work: &mut [f32],
              iwork: &mut [i32], ifail: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::sspgvx_(itype.as_ptr(), &(jobz as c_char), &(range as c_char), &(uplo as c_char),
                     &(n as c_int), ap.as_mut_ptr(), bp.as_mut_ptr(), vl.as_ptr(), vu.as_ptr(),
                     il.as_ptr(), iu.as_ptr(), abstol.as_ptr(), m as *mut _ as *mut _,
                     w.as_mut_ptr(), z.as_mut_ptr(), &(ldz as c_int), work.as_mut_ptr(),
                     iwork.as_mut_ptr(), ifail.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dspgvx(itype: &[i32], jobz: u8, range: u8, uplo: u8, n: usize, ap: &mut [f64],
              bp: &mut [f64], vl: &[f64], vu: &[f64], il: &[i32], iu: &[i32], abstol: &[f64],
              m: &mut u32, w: &mut [f64], z: &mut [f64], ldz: usize, work: &mut [f64],
              iwork: &mut [i32], ifail: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::dspgvx_(itype.as_ptr(), &(jobz as c_char), &(range as c_char), &(uplo as c_char),
                     &(n as c_int), ap.as_mut_ptr(), bp.as_mut_ptr(), vl.as_ptr(), vu.as_ptr(),
                     il.as_ptr(), iu.as_ptr(), abstol.as_ptr(), m as *mut _ as *mut _,
                     w.as_mut_ptr(), z.as_mut_ptr(), &(ldz as c_int), work.as_mut_ptr(),
                     iwork.as_mut_ptr(), ifail.as_mut_ptr(), info)
    }
}

#[inline]
pub fn chpgvx(itype: &[i32], jobz: u8, range: u8, uplo: u8, n: usize, ap: &mut [c32],
              bp: &mut [c32], vl: &[f32], vu: &[f32], il: &[i32], iu: &[i32], abstol: &[f32],
              m: &mut u32, w: &mut [f32], z: &mut [c32], ldz: usize, work: &mut [c32],
              rwork: &mut [f32], iwork: &mut [i32], ifail: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::chpgvx_(itype.as_ptr(), &(jobz as c_char), &(range as c_char), &(uplo as c_char),
                     &(n as c_int), ap.as_mut_ptr() as *mut _, bp.as_mut_ptr() as *mut _,
                     vl.as_ptr(), vu.as_ptr(), il.as_ptr(), iu.as_ptr(), abstol.as_ptr(),
                     m as *mut _ as *mut _, w.as_mut_ptr(), z.as_mut_ptr() as *mut _,
                     &(ldz as c_int), work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(),
                     iwork.as_mut_ptr(), ifail.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zhpgvx(itype: &[i32], jobz: u8, range: u8, uplo: u8, n: usize, ap: &mut [c64],
              bp: &mut [c64], vl: &[f64], vu: &[f64], il: &[i32], iu: &[i32], abstol: &[f64],
              m: &mut u32, w: &mut [f64], z: &mut [c64], ldz: usize, work: &mut [c64],
              rwork: &mut [f64], iwork: &mut [i32], ifail: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::zhpgvx_(itype.as_ptr(), &(jobz as c_char), &(range as c_char), &(uplo as c_char),
                     &(n as c_int), ap.as_mut_ptr() as *mut _, bp.as_mut_ptr() as *mut _,
                     vl.as_ptr(), vu.as_ptr(), il.as_ptr(), iu.as_ptr(), abstol.as_ptr(),
                     m as *mut _ as *mut _, w.as_mut_ptr(), z.as_mut_ptr() as *mut _,
                     &(ldz as c_int), work.as_mut_ptr() as *mut _, rwork.as_mut_ptr(),
                     iwork.as_mut_ptr(), ifail.as_mut_ptr(), info)
    }
}

#[inline]
pub fn ssbgv(jobz: u8, uplo: u8, n: usize, ka: usize, kb: usize, ab: &mut [f32], ldab: usize,
             bb: &mut [f32], ldbb: usize, w: &mut [f32], z: &mut [f32], ldz: usize,
             work: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::ssbgv_(&(jobz as c_char), &(uplo as c_char), &(n as c_int), &(ka as c_int),
                    &(kb as c_int), ab.as_mut_ptr(), &(ldab as c_int), bb.as_mut_ptr(),
                    &(ldbb as c_int), w.as_mut_ptr(), z.as_mut_ptr(), &(ldz as c_int),
                    work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dsbgv(jobz: u8, uplo: u8, n: usize, ka: usize, kb: usize, ab: &mut [f64], ldab: usize,
             bb: &mut [f64], ldbb: usize, w: &mut [f64], z: &mut [f64], ldz: usize,
             work: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::dsbgv_(&(jobz as c_char), &(uplo as c_char), &(n as c_int), &(ka as c_int),
                    &(kb as c_int), ab.as_mut_ptr(), &(ldab as c_int), bb.as_mut_ptr(),
                    &(ldbb as c_int), w.as_mut_ptr(), z.as_mut_ptr(), &(ldz as c_int),
                    work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn chbgv(jobz: u8, uplo: u8, n: usize, ka: usize, kb: usize, ab: &mut [c32], ldab: usize,
             bb: &mut [c32], ldbb: usize, w: &mut [f32], z: &mut [c32], ldz: usize,
             work: &mut [c32], rwork: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::chbgv_(&(jobz as c_char), &(uplo as c_char), &(n as c_int), &(ka as c_int),
                    &(kb as c_int), ab.as_mut_ptr() as *mut _, &(ldab as c_int),
                    bb.as_mut_ptr() as *mut _, &(ldbb as c_int), w.as_mut_ptr(),
                    z.as_mut_ptr() as *mut _, &(ldz as c_int), work.as_mut_ptr() as *mut _,
                    rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zhbgv(jobz: u8, uplo: u8, n: usize, ka: usize, kb: usize, ab: &mut [c64], ldab: usize,
             bb: &mut [c64], ldbb: usize, w: &mut [f64], z: &mut [c64], ldz: usize,
             work: &mut [c64], rwork: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::zhbgv_(&(jobz as c_char), &(uplo as c_char), &(n as c_int), &(ka as c_int),
                    &(kb as c_int), ab.as_mut_ptr() as *mut _, &(ldab as c_int),
                    bb.as_mut_ptr() as *mut _, &(ldbb as c_int), w.as_mut_ptr(),
                    z.as_mut_ptr() as *mut _, &(ldz as c_int), work.as_mut_ptr() as *mut _,
                    rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn ssbgvd(jobz: u8, uplo: u8, n: usize, ka: usize, kb: usize, ab: &mut [f32], ldab: usize,
              bb: &mut [f32], ldbb: usize, w: &mut [f32], z: &mut [f32], ldz: usize,
              work: &mut [f32], lwork: usize, iwork: &mut [i32], liwork: &[i32], info: &mut i32) {

    unsafe {
        ffi::ssbgvd_(&(jobz as c_char), &(uplo as c_char), &(n as c_int), &(ka as c_int),
                     &(kb as c_int), ab.as_mut_ptr(), &(ldab as c_int), bb.as_mut_ptr(),
                     &(ldbb as c_int), w.as_mut_ptr(), z.as_mut_ptr(), &(ldz as c_int),
                     work.as_mut_ptr(), &(lwork as c_int), iwork.as_mut_ptr(), liwork.as_ptr(),
                     info)
    }
}

#[inline]
pub fn dsbgvd(jobz: u8, uplo: u8, n: usize, ka: usize, kb: usize, ab: &mut [f64], ldab: usize,
              bb: &mut [f64], ldbb: usize, w: &mut [f64], z: &mut [f64], ldz: usize,
              work: &mut [f64], lwork: usize, iwork: &mut [i32], liwork: &[i32], info: &mut i32) {

    unsafe {
        ffi::dsbgvd_(&(jobz as c_char), &(uplo as c_char), &(n as c_int), &(ka as c_int),
                     &(kb as c_int), ab.as_mut_ptr(), &(ldab as c_int), bb.as_mut_ptr(),
                     &(ldbb as c_int), w.as_mut_ptr(), z.as_mut_ptr(), &(ldz as c_int),
                     work.as_mut_ptr(), &(lwork as c_int), iwork.as_mut_ptr(), liwork.as_ptr(),
                     info)
    }
}

#[inline]
pub fn chbgvd(jobz: u8, uplo: u8, n: usize, ka: usize, kb: usize, ab: &mut [c32], ldab: usize,
              bb: &mut [c32], ldbb: usize, w: &mut [f32], z: &mut [c32], ldz: usize,
              work: &mut [c32], lwork: usize, rwork: &mut [f32], lrwork: &[i32], iwork: &mut [i32],
              liwork: &[i32], info: &mut i32) {

    unsafe {
        ffi::chbgvd_(&(jobz as c_char), &(uplo as c_char), &(n as c_int), &(ka as c_int),
                     &(kb as c_int), ab.as_mut_ptr() as *mut _, &(ldab as c_int),
                     bb.as_mut_ptr() as *mut _, &(ldbb as c_int), w.as_mut_ptr(),
                     z.as_mut_ptr() as *mut _, &(ldz as c_int), work.as_mut_ptr() as *mut _,
                     &(lwork as c_int), rwork.as_mut_ptr(), lrwork.as_ptr(), iwork.as_mut_ptr(),
                     liwork.as_ptr(), info)
    }
}

#[inline]
pub fn zhbgvd(jobz: u8, uplo: u8, n: usize, ka: usize, kb: usize, ab: &mut [c64], ldab: usize,
              bb: &mut [c64], ldbb: usize, w: &mut [f64], z: &mut [c64], ldz: usize,
              work: &mut [c64], lwork: usize, rwork: &mut [f64], lrwork: &[i32], iwork: &mut [i32],
              liwork: &[i32], info: &mut i32) {

    unsafe {
        ffi::zhbgvd_(&(jobz as c_char), &(uplo as c_char), &(n as c_int), &(ka as c_int),
                     &(kb as c_int), ab.as_mut_ptr() as *mut _, &(ldab as c_int),
                     bb.as_mut_ptr() as *mut _, &(ldbb as c_int), w.as_mut_ptr(),
                     z.as_mut_ptr() as *mut _, &(ldz as c_int), work.as_mut_ptr() as *mut _,
                     &(lwork as c_int), rwork.as_mut_ptr(), lrwork.as_ptr(), iwork.as_mut_ptr(),
                     liwork.as_ptr(), info)
    }
}

#[inline]
pub fn ssbgvx(jobz: u8, range: u8, uplo: u8, n: usize, ka: usize, kb: usize, ab: &mut [f32],
              ldab: usize, bb: &mut [f32], ldbb: usize, q: &mut [f32], ldq: usize, vl: &[f32],
              vu: &[f32], il: &[i32], iu: &[i32], abstol: &[f32], m: &mut u32, w: &mut [f32],
              z: &mut [f32], ldz: usize, work: &mut [f32], iwork: &mut [i32], ifail: &mut [i32],
              info: &mut i32) {

    unsafe {
        ffi::ssbgvx_(&(jobz as c_char), &(range as c_char), &(uplo as c_char), &(n as c_int),
                     &(ka as c_int), &(kb as c_int), ab.as_mut_ptr(), &(ldab as c_int),
                     bb.as_mut_ptr(), &(ldbb as c_int), q.as_mut_ptr(), &(ldq as c_int),
                     vl.as_ptr(), vu.as_ptr(), il.as_ptr(), iu.as_ptr(), abstol.as_ptr(),
                     m as *mut _ as *mut _, w.as_mut_ptr(), z.as_mut_ptr(), &(ldz as c_int),
                     work.as_mut_ptr(), iwork.as_mut_ptr(), ifail.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dsbgvx(jobz: u8, range: u8, uplo: u8, n: usize, ka: usize, kb: usize, ab: &mut [f64],
              ldab: usize, bb: &mut [f64], ldbb: usize, q: &mut [f64], ldq: usize, vl: &[f64],
              vu: &[f64], il: &[i32], iu: &[i32], abstol: &[f64], m: &mut u32, w: &mut [f64],
              z: &mut [f64], ldz: usize, work: &mut [f64], iwork: &mut [i32], ifail: &mut [i32],
              info: &mut i32) {

    unsafe {
        ffi::dsbgvx_(&(jobz as c_char), &(range as c_char), &(uplo as c_char), &(n as c_int),
                     &(ka as c_int), &(kb as c_int), ab.as_mut_ptr(), &(ldab as c_int),
                     bb.as_mut_ptr(), &(ldbb as c_int), q.as_mut_ptr(), &(ldq as c_int),
                     vl.as_ptr(), vu.as_ptr(), il.as_ptr(), iu.as_ptr(), abstol.as_ptr(),
                     m as *mut _ as *mut _, w.as_mut_ptr(), z.as_mut_ptr(), &(ldz as c_int),
                     work.as_mut_ptr(), iwork.as_mut_ptr(), ifail.as_mut_ptr(), info)
    }
}

#[inline]
pub fn chbgvx(jobz: u8, range: u8, uplo: u8, n: usize, ka: usize, kb: usize, ab: &mut [c32],
              ldab: usize, bb: &mut [c32], ldbb: usize, q: &mut [c32], ldq: usize, vl: &[f32],
              vu: &[f32], il: &[i32], iu: &[i32], abstol: &[f32], m: &mut u32, w: &mut [f32],
              z: &mut [c32], ldz: usize, work: &mut [c32], rwork: &mut [f32], iwork: &mut [i32],
              ifail: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::chbgvx_(&(jobz as c_char), &(range as c_char), &(uplo as c_char), &(n as c_int),
                     &(ka as c_int), &(kb as c_int), ab.as_mut_ptr() as *mut _, &(ldab as c_int),
                     bb.as_mut_ptr() as *mut _, &(ldbb as c_int), q.as_mut_ptr() as *mut _,
                     &(ldq as c_int), vl.as_ptr(), vu.as_ptr(), il.as_ptr(), iu.as_ptr(),
                     abstol.as_ptr(), m as *mut _ as *mut _, w.as_mut_ptr(),
                     z.as_mut_ptr() as *mut _, &(ldz as c_int), work.as_mut_ptr() as *mut _,
                     rwork.as_mut_ptr(), iwork.as_mut_ptr(), ifail.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zhbgvx(jobz: u8, range: u8, uplo: u8, n: usize, ka: usize, kb: usize, ab: &mut [c64],
              ldab: usize, bb: &mut [c64], ldbb: usize, q: &mut [c64], ldq: usize, vl: &[f64],
              vu: &[f64], il: &[i32], iu: &[i32], abstol: &[f64], m: &mut u32, w: &mut [f64],
              z: &mut [c64], ldz: usize, work: &mut [c64], rwork: &mut [f64], iwork: &mut [i32],
              ifail: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::zhbgvx_(&(jobz as c_char), &(range as c_char), &(uplo as c_char), &(n as c_int),
                     &(ka as c_int), &(kb as c_int), ab.as_mut_ptr() as *mut _, &(ldab as c_int),
                     bb.as_mut_ptr() as *mut _, &(ldbb as c_int), q.as_mut_ptr() as *mut _,
                     &(ldq as c_int), vl.as_ptr(), vu.as_ptr(), il.as_ptr(), iu.as_ptr(),
                     abstol.as_ptr(), m as *mut _ as *mut _, w.as_mut_ptr(),
                     z.as_mut_ptr() as *mut _, &(ldz as c_int), work.as_mut_ptr() as *mut _,
                     rwork.as_mut_ptr(), iwork.as_mut_ptr(), ifail.as_mut_ptr(), info)
    }
}

#[inline]
pub fn sgges(jobvsl: u8, jobvsr: u8, sort: u8, selctg: Select3F32, n: usize, a: &mut [f32],
             lda: usize, b: &mut [f32], ldb: usize, sdim: &mut [i32], alphar: &mut [f32],
             alphai: &mut [f32], beta: &mut [f32], vsl: &mut [f32], ldvsl: usize, vsr: &mut [f32],
             ldvsr: usize, work: &mut [f32], lwork: usize, bwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::sgges_(&(jobvsl as c_char), &(jobvsr as c_char), &(sort as c_char), transmute(selctg),
                    &(n as c_int), a.as_mut_ptr(), &(lda as c_int), b.as_mut_ptr(),
                    &(ldb as c_int), sdim.as_mut_ptr(), alphar.as_mut_ptr(), alphai.as_mut_ptr(),
                    beta.as_mut_ptr(), vsl.as_mut_ptr(), &(ldvsl as c_int), vsr.as_mut_ptr(),
                    &(ldvsr as c_int), work.as_mut_ptr(), &(lwork as c_int), bwork.as_mut_ptr(),
                    info)
    }
}

#[inline]
pub fn dgges(jobvsl: u8, jobvsr: u8, sort: u8, selctg: Select3F64, n: usize, a: &mut [f64],
             lda: usize, b: &mut [f64], ldb: usize, sdim: &mut [i32], alphar: &mut [f64],
             alphai: &mut [f64], beta: &mut [f64], vsl: &mut [f64], ldvsl: usize, vsr: &mut [f64],
             ldvsr: usize, work: &mut [f64], lwork: usize, bwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::dgges_(&(jobvsl as c_char), &(jobvsr as c_char), &(sort as c_char), transmute(selctg),
                    &(n as c_int), a.as_mut_ptr(), &(lda as c_int), b.as_mut_ptr(),
                    &(ldb as c_int), sdim.as_mut_ptr(), alphar.as_mut_ptr(), alphai.as_mut_ptr(),
                    beta.as_mut_ptr(), vsl.as_mut_ptr(), &(ldvsl as c_int), vsr.as_mut_ptr(),
                    &(ldvsr as c_int), work.as_mut_ptr(), &(lwork as c_int), bwork.as_mut_ptr(),
                    info)
    }
}

#[inline]
pub fn cgges(jobvsl: u8, jobvsr: u8, sort: u8, selctg: Select2C32, n: usize, a: &mut [c32],
             lda: usize, b: &mut [c32], ldb: usize, sdim: &mut [i32], alpha: &mut [c32],
             beta: &mut [c32], vsl: &mut [c32], ldvsl: usize, vsr: &mut [c32], ldvsr: usize,
             work: &mut [c32], lwork: usize, rwork: &mut [f32], bwork: &mut [i32],
             info: &mut i32) {

    unsafe {
        ffi::cgges_(&(jobvsl as c_char), &(jobvsr as c_char), &(sort as c_char), transmute(selctg),
                    &(n as c_int), a.as_mut_ptr() as *mut _, &(lda as c_int),
                    b.as_mut_ptr() as *mut _, &(ldb as c_int), sdim.as_mut_ptr(),
                    alpha.as_mut_ptr() as *mut _, beta.as_mut_ptr() as *mut _,
                    vsl.as_mut_ptr() as *mut _, &(ldvsl as c_int), vsr.as_mut_ptr() as *mut _,
                    &(ldvsr as c_int), work.as_mut_ptr() as *mut _, &(lwork as c_int),
                    rwork.as_mut_ptr(), bwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zgges(jobvsl: u8, jobvsr: u8, sort: u8, selctg: Select2C64, n: usize, a: &mut [c64],
             lda: usize, b: &mut [c64], ldb: usize, sdim: &mut [i32], alpha: &mut [c64],
             beta: &mut [c64], vsl: &mut [c64], ldvsl: usize, vsr: &mut [c64], ldvsr: usize,
             work: &mut [c64], lwork: usize, rwork: &mut [f64], bwork: &mut [i32],
             info: &mut i32) {

    unsafe {
        ffi::zgges_(&(jobvsl as c_char), &(jobvsr as c_char), &(sort as c_char), transmute(selctg),
                    &(n as c_int), a.as_mut_ptr() as *mut _, &(lda as c_int),
                    b.as_mut_ptr() as *mut _, &(ldb as c_int), sdim.as_mut_ptr(),
                    alpha.as_mut_ptr() as *mut _, beta.as_mut_ptr() as *mut _,
                    vsl.as_mut_ptr() as *mut _, &(ldvsl as c_int), vsr.as_mut_ptr() as *mut _,
                    &(ldvsr as c_int), work.as_mut_ptr() as *mut _, &(lwork as c_int),
                    rwork.as_mut_ptr(), bwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn sggesx(jobvsl: u8, jobvsr: u8, sort: u8, selctg: Select3F32, sense: u8, n: usize,
              a: &mut [f32], lda: usize, b: &mut [f32], ldb: usize, sdim: &mut [i32],
              alphar: &mut [f32], alphai: &mut [f32], beta: &mut [f32], vsl: &mut [f32],
              ldvsl: usize, vsr: &mut [f32], ldvsr: usize, rconde: &mut [f32], rcondv: &mut [f32],
              work: &mut [f32], lwork: usize, iwork: &mut [i32], liwork: &[i32], bwork: &mut [i32],
              info: &mut i32) {

    unsafe {
        ffi::sggesx_(&(jobvsl as c_char), &(jobvsr as c_char), &(sort as c_char),
                     transmute(selctg), &(sense as c_char), &(n as c_int), a.as_mut_ptr(),
                     &(lda as c_int), b.as_mut_ptr(), &(ldb as c_int), sdim.as_mut_ptr(),
                     alphar.as_mut_ptr(), alphai.as_mut_ptr(), beta.as_mut_ptr(), vsl.as_mut_ptr(),
                     &(ldvsl as c_int), vsr.as_mut_ptr(), &(ldvsr as c_int), rconde.as_mut_ptr(),
                     rcondv.as_mut_ptr(), work.as_mut_ptr(), &(lwork as c_int), iwork.as_mut_ptr(),
                     liwork.as_ptr(), bwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dggesx(jobvsl: u8, jobvsr: u8, sort: u8, selctg: Select3F64, sense: u8, n: usize,
              a: &mut [f64], lda: usize, b: &mut [f64], ldb: usize, sdim: &mut [i32],
              alphar: &mut [f64], alphai: &mut [f64], beta: &mut [f64], vsl: &mut [f64],
              ldvsl: usize, vsr: &mut [f64], ldvsr: usize, rconde: &mut [f64], rcondv: &mut [f64],
              work: &mut [f64], lwork: usize, iwork: &mut [i32], liwork: &[i32], bwork: &mut [i32],
              info: &mut i32) {

    unsafe {
        ffi::dggesx_(&(jobvsl as c_char), &(jobvsr as c_char), &(sort as c_char),
                     transmute(selctg), &(sense as c_char), &(n as c_int), a.as_mut_ptr(),
                     &(lda as c_int), b.as_mut_ptr(), &(ldb as c_int), sdim.as_mut_ptr(),
                     alphar.as_mut_ptr(), alphai.as_mut_ptr(), beta.as_mut_ptr(), vsl.as_mut_ptr(),
                     &(ldvsl as c_int), vsr.as_mut_ptr(), &(ldvsr as c_int), rconde.as_mut_ptr(),
                     rcondv.as_mut_ptr(), work.as_mut_ptr(), &(lwork as c_int), iwork.as_mut_ptr(),
                     liwork.as_ptr(), bwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn cggesx(jobvsl: u8, jobvsr: u8, sort: u8, selctg: Select2C32, sense: u8, n: usize,
              a: &mut [c32], lda: usize, b: &mut [c32], ldb: usize, sdim: &mut [i32],
              alpha: &mut [c32], beta: &mut [c32], vsl: &mut [c32], ldvsl: usize, vsr: &mut [c32],
              ldvsr: usize, rconde: &mut [f32], rcondv: &mut [f32], work: &mut [c32], lwork: usize,
              rwork: &mut [f32], iwork: &mut [i32], liwork: &[i32], bwork: &mut [i32],
              info: &mut i32) {

    unsafe {
        ffi::cggesx_(&(jobvsl as c_char), &(jobvsr as c_char), &(sort as c_char),
                     transmute(selctg), &(sense as c_char), &(n as c_int),
                     a.as_mut_ptr() as *mut _, &(lda as c_int), b.as_mut_ptr() as *mut _,
                     &(ldb as c_int), sdim.as_mut_ptr(), alpha.as_mut_ptr() as *mut _,
                     beta.as_mut_ptr() as *mut _, vsl.as_mut_ptr() as *mut _, &(ldvsl as c_int),
                     vsr.as_mut_ptr() as *mut _, &(ldvsr as c_int), rconde.as_mut_ptr(),
                     rcondv.as_mut_ptr(), work.as_mut_ptr() as *mut _, &(lwork as c_int),
                     rwork.as_mut_ptr(), iwork.as_mut_ptr(), liwork.as_ptr(), bwork.as_mut_ptr(),
                     info)
    }
}

#[inline]
pub fn zggesx(jobvsl: u8, jobvsr: u8, sort: u8, selctg: Select2C64, sense: u8, n: usize,
              a: &mut [c64], lda: usize, b: &mut [c64], ldb: usize, sdim: &mut [i32],
              alpha: &mut [c64], beta: &mut [c64], vsl: &mut [c64], ldvsl: usize, vsr: &mut [c64],
              ldvsr: usize, rconde: &mut [f64], rcondv: &mut [f64], work: &mut [c64], lwork: usize,
              rwork: &mut [f64], iwork: &mut [i32], liwork: &[i32], bwork: &mut [i32],
              info: &mut i32) {

    unsafe {
        ffi::zggesx_(&(jobvsl as c_char), &(jobvsr as c_char), &(sort as c_char),
                     transmute(selctg), &(sense as c_char), &(n as c_int),
                     a.as_mut_ptr() as *mut _, &(lda as c_int), b.as_mut_ptr() as *mut _,
                     &(ldb as c_int), sdim.as_mut_ptr(), alpha.as_mut_ptr() as *mut _,
                     beta.as_mut_ptr() as *mut _, vsl.as_mut_ptr() as *mut _, &(ldvsl as c_int),
                     vsr.as_mut_ptr() as *mut _, &(ldvsr as c_int), rconde.as_mut_ptr(),
                     rcondv.as_mut_ptr(), work.as_mut_ptr() as *mut _, &(lwork as c_int),
                     rwork.as_mut_ptr(), iwork.as_mut_ptr(), liwork.as_ptr(), bwork.as_mut_ptr(),
                     info)
    }
}

#[inline]
pub fn sggev(jobvl: u8, jobvr: u8, n: usize, a: &mut [f32], lda: usize, b: &mut [f32], ldb: usize,
             alphar: &mut [f32], alphai: &mut [f32], beta: &mut [f32], vl: &mut [f32], ldvl: usize,
             vr: &mut [f32], ldvr: usize, work: &mut [f32], lwork: usize, info: &mut i32) {

    unsafe {
        ffi::sggev_(&(jobvl as c_char), &(jobvr as c_char), &(n as c_int), a.as_mut_ptr(),
                    &(lda as c_int), b.as_mut_ptr(), &(ldb as c_int), alphar.as_mut_ptr(),
                    alphai.as_mut_ptr(), beta.as_mut_ptr(), vl.as_mut_ptr(), &(ldvl as c_int),
                    vr.as_mut_ptr(), &(ldvr as c_int), work.as_mut_ptr(), &(lwork as c_int), info)
    }
}

#[inline]
pub fn dggev(jobvl: u8, jobvr: u8, n: usize, a: &mut [f64], lda: usize, b: &mut [f64], ldb: usize,
             alphar: &mut [f64], alphai: &mut [f64], beta: &mut [f64], vl: &mut [f64], ldvl: usize,
             vr: &mut [f64], ldvr: usize, work: &mut [f64], lwork: usize, info: &mut i32) {

    unsafe {
        ffi::dggev_(&(jobvl as c_char), &(jobvr as c_char), &(n as c_int), a.as_mut_ptr(),
                    &(lda as c_int), b.as_mut_ptr(), &(ldb as c_int), alphar.as_mut_ptr(),
                    alphai.as_mut_ptr(), beta.as_mut_ptr(), vl.as_mut_ptr(), &(ldvl as c_int),
                    vr.as_mut_ptr(), &(ldvr as c_int), work.as_mut_ptr(), &(lwork as c_int), info)
    }
}

#[inline]
pub fn cggev(jobvl: u8, jobvr: u8, n: usize, a: &mut [c32], lda: usize, b: &mut [c32], ldb: usize,
             alpha: &mut [c32], beta: &mut [c32], vl: &mut [c32], ldvl: usize, vr: &mut [c32],
             ldvr: usize, work: &mut [c32], lwork: usize, rwork: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::cggev_(&(jobvl as c_char), &(jobvr as c_char), &(n as c_int),
                    a.as_mut_ptr() as *mut _, &(lda as c_int), b.as_mut_ptr() as *mut _,
                    &(ldb as c_int), alpha.as_mut_ptr() as *mut _, beta.as_mut_ptr() as *mut _,
                    vl.as_mut_ptr() as *mut _, &(ldvl as c_int), vr.as_mut_ptr() as *mut _,
                    &(ldvr as c_int), work.as_mut_ptr() as *mut _, &(lwork as c_int),
                    rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zggev(jobvl: u8, jobvr: u8, n: usize, a: &mut [c64], lda: usize, b: &mut [c64], ldb: usize,
             alpha: &mut [c64], beta: &mut [c64], vl: &mut [c64], ldvl: usize, vr: &mut [c64],
             ldvr: usize, work: &mut [c64], lwork: usize, rwork: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::zggev_(&(jobvl as c_char), &(jobvr as c_char), &(n as c_int),
                    a.as_mut_ptr() as *mut _, &(lda as c_int), b.as_mut_ptr() as *mut _,
                    &(ldb as c_int), alpha.as_mut_ptr() as *mut _, beta.as_mut_ptr() as *mut _,
                    vl.as_mut_ptr() as *mut _, &(ldvl as c_int), vr.as_mut_ptr() as *mut _,
                    &(ldvr as c_int), work.as_mut_ptr() as *mut _, &(lwork as c_int),
                    rwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn sggevx(balanc: u8, jobvl: u8, jobvr: u8, sense: u8, n: usize, a: &mut [f32], lda: usize,
              b: &mut [f32], ldb: usize, alphar: &mut [f32], alphai: &mut [f32], beta: &mut [f32],
              vl: &mut [f32], ldvl: usize, vr: &mut [f32], ldvr: usize, ilo: &mut [i32],
              ihi: &mut [i32], lscale: &mut [f32], rscale: &mut [f32], abnrm: &mut [f32],
              bbnrm: &mut [f32], rconde: &mut [f32], rcondv: &mut [f32], work: &mut [f32],
              lwork: usize, iwork: &mut [i32], bwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::sggevx_(&(balanc as c_char), &(jobvl as c_char), &(jobvr as c_char),
                     &(sense as c_char), &(n as c_int), a.as_mut_ptr(), &(lda as c_int),
                     b.as_mut_ptr(), &(ldb as c_int), alphar.as_mut_ptr(), alphai.as_mut_ptr(),
                     beta.as_mut_ptr(), vl.as_mut_ptr(), &(ldvl as c_int), vr.as_mut_ptr(),
                     &(ldvr as c_int), ilo.as_mut_ptr(), ihi.as_mut_ptr(), lscale.as_mut_ptr(),
                     rscale.as_mut_ptr(), abnrm.as_mut_ptr(), bbnrm.as_mut_ptr(),
                     rconde.as_mut_ptr(), rcondv.as_mut_ptr(), work.as_mut_ptr(),
                     &(lwork as c_int), iwork.as_mut_ptr(), bwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dggevx(balanc: u8, jobvl: u8, jobvr: u8, sense: u8, n: usize, a: &mut [f64], lda: usize,
              b: &mut [f64], ldb: usize, alphar: &mut [f64], alphai: &mut [f64], beta: &mut [f64],
              vl: &mut [f64], ldvl: usize, vr: &mut [f64], ldvr: usize, ilo: &mut [i32],
              ihi: &mut [i32], lscale: &mut [f64], rscale: &mut [f64], abnrm: &mut [f64],
              bbnrm: &mut [f64], rconde: &mut [f64], rcondv: &mut [f64], work: &mut [f64],
              lwork: usize, iwork: &mut [i32], bwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::dggevx_(&(balanc as c_char), &(jobvl as c_char), &(jobvr as c_char),
                     &(sense as c_char), &(n as c_int), a.as_mut_ptr(), &(lda as c_int),
                     b.as_mut_ptr(), &(ldb as c_int), alphar.as_mut_ptr(), alphai.as_mut_ptr(),
                     beta.as_mut_ptr(), vl.as_mut_ptr(), &(ldvl as c_int), vr.as_mut_ptr(),
                     &(ldvr as c_int), ilo.as_mut_ptr(), ihi.as_mut_ptr(), lscale.as_mut_ptr(),
                     rscale.as_mut_ptr(), abnrm.as_mut_ptr(), bbnrm.as_mut_ptr(),
                     rconde.as_mut_ptr(), rcondv.as_mut_ptr(), work.as_mut_ptr(),
                     &(lwork as c_int), iwork.as_mut_ptr(), bwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn cggevx(balanc: u8, jobvl: u8, jobvr: u8, sense: u8, n: usize, a: &mut [c32], lda: usize,
              b: &mut [c32], ldb: usize, alpha: &mut [c32], beta: &mut [c32], vl: &mut [c32],
              ldvl: usize, vr: &mut [c32], ldvr: usize, ilo: &mut [i32], ihi: &mut [i32],
              lscale: &mut [f32], rscale: &mut [f32], abnrm: &mut [f32], bbnrm: &mut [f32],
              rconde: &mut [f32], rcondv: &mut [f32], work: &mut [c32], lwork: usize,
              rwork: &mut [f32], iwork: &mut [i32], bwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::cggevx_(&(balanc as c_char), &(jobvl as c_char), &(jobvr as c_char),
                     &(sense as c_char), &(n as c_int), a.as_mut_ptr() as *mut _, &(lda as c_int),
                     b.as_mut_ptr() as *mut _, &(ldb as c_int), alpha.as_mut_ptr() as *mut _,
                     beta.as_mut_ptr() as *mut _, vl.as_mut_ptr() as *mut _, &(ldvl as c_int),
                     vr.as_mut_ptr() as *mut _, &(ldvr as c_int), ilo.as_mut_ptr(),
                     ihi.as_mut_ptr(), lscale.as_mut_ptr(), rscale.as_mut_ptr(),
                     abnrm.as_mut_ptr(), bbnrm.as_mut_ptr(), rconde.as_mut_ptr(),
                     rcondv.as_mut_ptr(), work.as_mut_ptr() as *mut _, &(lwork as c_int),
                     rwork.as_mut_ptr(), iwork.as_mut_ptr(), bwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zggevx(balanc: u8, jobvl: u8, jobvr: u8, sense: u8, n: usize, a: &mut [c64], lda: usize,
              b: &mut [c64], ldb: usize, alpha: &mut [c64], beta: &mut [c64], vl: &mut [c64],
              ldvl: usize, vr: &mut [c64], ldvr: usize, ilo: &mut [i32], ihi: &mut [i32],
              lscale: &mut [f64], rscale: &mut [f64], abnrm: &mut [f64], bbnrm: &mut [f64],
              rconde: &mut [f64], rcondv: &mut [f64], work: &mut [c64], lwork: usize,
              rwork: &mut [f64], iwork: &mut [i32], bwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::zggevx_(&(balanc as c_char), &(jobvl as c_char), &(jobvr as c_char),
                     &(sense as c_char), &(n as c_int), a.as_mut_ptr() as *mut _, &(lda as c_int),
                     b.as_mut_ptr() as *mut _, &(ldb as c_int), alpha.as_mut_ptr() as *mut _,
                     beta.as_mut_ptr() as *mut _, vl.as_mut_ptr() as *mut _, &(ldvl as c_int),
                     vr.as_mut_ptr() as *mut _, &(ldvr as c_int), ilo.as_mut_ptr(),
                     ihi.as_mut_ptr(), lscale.as_mut_ptr(), rscale.as_mut_ptr(),
                     abnrm.as_mut_ptr(), bbnrm.as_mut_ptr(), rconde.as_mut_ptr(),
                     rcondv.as_mut_ptr(), work.as_mut_ptr() as *mut _, &(lwork as c_int),
                     rwork.as_mut_ptr(), iwork.as_mut_ptr(), bwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn ssfrk(transr: u8, uplo: u8, trans: u8, n: usize, k: usize, alpha: &[f32], a: &[f32],
             lda: usize, beta: &[f32], c: &mut [f32]) {

    unsafe {
        ffi::ssfrk_(&(transr as c_char), &(uplo as c_char), &(trans as c_char), &(n as c_int),
                    &(k as c_int), alpha.as_ptr(), a.as_ptr(), &(lda as c_int), beta.as_ptr(),
                    c.as_mut_ptr())
    }
}

#[inline]
pub fn dsfrk(transr: u8, uplo: u8, trans: u8, n: usize, k: usize, alpha: &[f64], a: &[f64],
             lda: usize, beta: &[f64], c: &mut [f64]) {

    unsafe {
        ffi::dsfrk_(&(transr as c_char), &(uplo as c_char), &(trans as c_char), &(n as c_int),
                    &(k as c_int), alpha.as_ptr(), a.as_ptr(), &(lda as c_int), beta.as_ptr(),
                    c.as_mut_ptr())
    }
}

#[inline]
pub fn chfrk(transr: u8, uplo: u8, trans: u8, n: usize, k: usize, alpha: &[f32], a: &[c32],
             lda: usize, beta: &[f32], c: &mut [c32]) {

    unsafe {
        ffi::chfrk_(&(transr as c_char), &(uplo as c_char), &(trans as c_char), &(n as c_int),
                    &(k as c_int), alpha.as_ptr(), a.as_ptr() as *const _, &(lda as c_int),
                    beta.as_ptr(), c.as_mut_ptr() as *mut _)
    }
}

#[inline]
pub fn zhfrk(transr: u8, uplo: u8, trans: u8, n: usize, k: usize, alpha: &[f64], a: &[c64],
             lda: usize, beta: &[f64], c: &mut [c64]) {

    unsafe {
        ffi::zhfrk_(&(transr as c_char), &(uplo as c_char), &(trans as c_char), &(n as c_int),
                    &(k as c_int), alpha.as_ptr(), a.as_ptr() as *const _, &(lda as c_int),
                    beta.as_ptr(), c.as_mut_ptr() as *mut _)
    }
}

#[inline]
pub fn stfsm(transr: u8, side: u8, uplo: u8, trans: u8, diag: u8, m: usize, n: usize,
             alpha: &[f32], a: &[f32], b: &mut [f32], ldb: usize) {

    unsafe {
        ffi::stfsm_(&(transr as c_char), &(side as c_char), &(uplo as c_char), &(trans as c_char),
                    &(diag as c_char), &(m as c_int), &(n as c_int), alpha.as_ptr(), a.as_ptr(),
                    b.as_mut_ptr(), &(ldb as c_int))
    }
}

#[inline]
pub fn dtfsm(transr: u8, side: u8, uplo: u8, trans: u8, diag: u8, m: usize, n: usize,
             alpha: &[f64], a: &[f64], b: &mut [f64], ldb: usize) {

    unsafe {
        ffi::dtfsm_(&(transr as c_char), &(side as c_char), &(uplo as c_char), &(trans as c_char),
                    &(diag as c_char), &(m as c_int), &(n as c_int), alpha.as_ptr(), a.as_ptr(),
                    b.as_mut_ptr(), &(ldb as c_int))
    }
}

#[inline]
pub fn ctfsm(transr: u8, side: u8, uplo: u8, trans: u8, diag: u8, m: usize, n: usize,
             alpha: &[c32], a: &[c32], b: &mut [c32], ldb: usize) {

    unsafe {
        ffi::ctfsm_(&(transr as c_char), &(side as c_char), &(uplo as c_char), &(trans as c_char),
                    &(diag as c_char), &(m as c_int), &(n as c_int), alpha.as_ptr() as *const _,
                    a.as_ptr() as *const _, b.as_mut_ptr() as *mut _, &(ldb as c_int))
    }
}

#[inline]
pub fn ztfsm(transr: u8, side: u8, uplo: u8, trans: u8, diag: u8, m: usize, n: usize,
             alpha: &[c64], a: &[c64], b: &mut [c64], ldb: usize) {

    unsafe {
        ffi::ztfsm_(&(transr as c_char), &(side as c_char), &(uplo as c_char), &(trans as c_char),
                    &(diag as c_char), &(m as c_int), &(n as c_int), alpha.as_ptr() as *const _,
                    a.as_ptr() as *const _, b.as_mut_ptr() as *mut _, &(ldb as c_int))
    }
}

#[inline]
pub fn stfttp(transr: u8, uplo: u8, n: usize, arf: &[f32], ap: &mut [f32], info: &mut i32) {
    unsafe {
        ffi::stfttp_(&(transr as c_char), &(uplo as c_char), &(n as c_int), arf.as_ptr(),
                     ap.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dtfttp(transr: u8, uplo: u8, n: usize, arf: &[f64], ap: &mut [f64], info: &mut i32) {
    unsafe {
        ffi::dtfttp_(&(transr as c_char), &(uplo as c_char), &(n as c_int), arf.as_ptr(),
                     ap.as_mut_ptr(), info)
    }
}

#[inline]
pub fn ctfttp(transr: u8, uplo: u8, n: usize, arf: &[c32], ap: &mut [c32], info: &mut i32) {
    unsafe {
        ffi::ctfttp_(&(transr as c_char), &(uplo as c_char), &(n as c_int),
                     arf.as_ptr() as *const _, ap.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn ztfttp(transr: u8, uplo: u8, n: usize, arf: &[c64], ap: &mut [c64], info: &mut i32) {
    unsafe {
        ffi::ztfttp_(&(transr as c_char), &(uplo as c_char), &(n as c_int),
                     arf.as_ptr() as *const _, ap.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn stfttr(transr: u8, uplo: u8, n: usize, arf: &[f32], a: &mut [f32], lda: usize,
              info: &mut i32) {

    unsafe {
        ffi::stfttr_(&(transr as c_char), &(uplo as c_char), &(n as c_int), arf.as_ptr(),
                     a.as_mut_ptr(), &(lda as c_int), info)
    }
}

#[inline]
pub fn dtfttr(transr: u8, uplo: u8, n: usize, arf: &[f64], a: &mut [f64], lda: usize,
              info: &mut i32) {

    unsafe {
        ffi::dtfttr_(&(transr as c_char), &(uplo as c_char), &(n as c_int), arf.as_ptr(),
                     a.as_mut_ptr(), &(lda as c_int), info)
    }
}

#[inline]
pub fn ctfttr(transr: u8, uplo: u8, n: usize, arf: &[c32], a: &mut [c32], lda: usize,
              info: &mut i32) {

    unsafe {
        ffi::ctfttr_(&(transr as c_char), &(uplo as c_char), &(n as c_int),
                     arf.as_ptr() as *const _, a.as_mut_ptr() as *mut _, &(lda as c_int), info)
    }
}

#[inline]
pub fn ztfttr(transr: u8, uplo: u8, n: usize, arf: &[c64], a: &mut [c64], lda: usize,
              info: &mut i32) {

    unsafe {
        ffi::ztfttr_(&(transr as c_char), &(uplo as c_char), &(n as c_int),
                     arf.as_ptr() as *const _, a.as_mut_ptr() as *mut _, &(lda as c_int), info)
    }
}

#[inline]
pub fn stpttf(transr: u8, uplo: u8, n: usize, ap: &[f32], arf: &mut [f32], info: &mut i32) {
    unsafe {
        ffi::stpttf_(&(transr as c_char), &(uplo as c_char), &(n as c_int), ap.as_ptr(),
                     arf.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dtpttf(transr: u8, uplo: u8, n: usize, ap: &[f64], arf: &mut [f64], info: &mut i32) {
    unsafe {
        ffi::dtpttf_(&(transr as c_char), &(uplo as c_char), &(n as c_int), ap.as_ptr(),
                     arf.as_mut_ptr(), info)
    }
}

#[inline]
pub fn ctpttf(transr: u8, uplo: u8, n: usize, ap: &[c32], arf: &mut [c32], info: &mut i32) {
    unsafe {
        ffi::ctpttf_(&(transr as c_char), &(uplo as c_char), &(n as c_int),
                     ap.as_ptr() as *const _, arf.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn ztpttf(transr: u8, uplo: u8, n: usize, ap: &[c64], arf: &mut [c64], info: &mut i32) {
    unsafe {
        ffi::ztpttf_(&(transr as c_char), &(uplo as c_char), &(n as c_int),
                     ap.as_ptr() as *const _, arf.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn stpttr(uplo: u8, n: usize, ap: &[f32], a: &mut [f32], lda: usize, info: &mut i32) {
    unsafe {
        ffi::stpttr_(&(uplo as c_char), &(n as c_int), ap.as_ptr(), a.as_mut_ptr(),
                     &(lda as c_int), info)
    }
}

#[inline]
pub fn dtpttr(uplo: u8, n: usize, ap: &[f64], a: &mut [f64], lda: usize, info: &mut i32) {
    unsafe {
        ffi::dtpttr_(&(uplo as c_char), &(n as c_int), ap.as_ptr(), a.as_mut_ptr(),
                     &(lda as c_int), info)
    }
}

#[inline]
pub fn ctpttr(uplo: u8, n: usize, ap: &[c32], a: &mut [c32], lda: usize, info: &mut i32) {
    unsafe {
        ffi::ctpttr_(&(uplo as c_char), &(n as c_int), ap.as_ptr() as *const _,
                     a.as_mut_ptr() as *mut _, &(lda as c_int), info)
    }
}

#[inline]
pub fn ztpttr(uplo: u8, n: usize, ap: &[c64], a: &mut [c64], lda: usize, info: &mut i32) {
    unsafe {
        ffi::ztpttr_(&(uplo as c_char), &(n as c_int), ap.as_ptr() as *const _,
                     a.as_mut_ptr() as *mut _, &(lda as c_int), info)
    }
}

#[inline]
pub fn strttf(transr: u8, uplo: u8, n: usize, a: &[f32], lda: usize, arf: &mut [f32],
              info: &mut i32) {

    unsafe {
        ffi::strttf_(&(transr as c_char), &(uplo as c_char), &(n as c_int), a.as_ptr(),
                     &(lda as c_int), arf.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dtrttf(transr: u8, uplo: u8, n: usize, a: &[f64], lda: usize, arf: &mut [f64],
              info: &mut i32) {

    unsafe {
        ffi::dtrttf_(&(transr as c_char), &(uplo as c_char), &(n as c_int), a.as_ptr(),
                     &(lda as c_int), arf.as_mut_ptr(), info)
    }
}

#[inline]
pub fn ctrttf(transr: u8, uplo: u8, n: usize, a: &[c32], lda: usize, arf: &mut [c32],
              info: &mut i32) {

    unsafe {
        ffi::ctrttf_(&(transr as c_char), &(uplo as c_char), &(n as c_int), a.as_ptr() as *const _,
                     &(lda as c_int), arf.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn ztrttf(transr: u8, uplo: u8, n: usize, a: &[c64], lda: usize, arf: &mut [c64],
              info: &mut i32) {

    unsafe {
        ffi::ztrttf_(&(transr as c_char), &(uplo as c_char), &(n as c_int), a.as_ptr() as *const _,
                     &(lda as c_int), arf.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn strttp(uplo: u8, n: usize, a: &[f32], lda: usize, ap: &mut [f32], info: &mut i32) {
    unsafe {
        ffi::strttp_(&(uplo as c_char), &(n as c_int), a.as_ptr(), &(lda as c_int),
                     ap.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dtrttp(uplo: u8, n: usize, a: &[f64], lda: usize, ap: &mut [f64], info: &mut i32) {
    unsafe {
        ffi::dtrttp_(&(uplo as c_char), &(n as c_int), a.as_ptr(), &(lda as c_int),
                     ap.as_mut_ptr(), info)
    }
}

#[inline]
pub fn ctrttp(uplo: u8, n: usize, a: &[c32], lda: usize, ap: &mut [c32], info: &mut i32) {
    unsafe {
        ffi::ctrttp_(&(uplo as c_char), &(n as c_int), a.as_ptr() as *const _, &(lda as c_int),
                     ap.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn ztrttp(uplo: u8, n: usize, a: &[c64], lda: usize, ap: &mut [c64], info: &mut i32) {
    unsafe {
        ffi::ztrttp_(&(uplo as c_char), &(n as c_int), a.as_ptr() as *const _, &(lda as c_int),
                     ap.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn sgeqrfp(m: usize, n: usize, a: &mut [f32], lda: usize, tau: &mut [f32], work: &mut [f32],
               lwork: usize, info: &mut i32) {

    unsafe {
        ffi::sgeqrfp_(&(m as c_int), &(n as c_int), a.as_mut_ptr(), &(lda as c_int),
                      tau.as_mut_ptr(), work.as_mut_ptr(), &(lwork as c_int), info)
    }
}

#[inline]
pub fn dgeqrfp(m: usize, n: usize, a: &mut [f64], lda: usize, tau: &mut [f64], work: &mut [f64],
               lwork: usize, info: &mut i32) {

    unsafe {
        ffi::dgeqrfp_(&(m as c_int), &(n as c_int), a.as_mut_ptr(), &(lda as c_int),
                      tau.as_mut_ptr(), work.as_mut_ptr(), &(lwork as c_int), info)
    }
}

#[inline]
pub fn cgeqrfp(m: usize, n: usize, a: &mut [c32], lda: usize, tau: &mut [c32], work: &mut [c32],
               lwork: usize, info: &mut i32) {

    unsafe {
        ffi::cgeqrfp_(&(m as c_int), &(n as c_int), a.as_mut_ptr() as *mut _, &(lda as c_int),
                      tau.as_mut_ptr() as *mut _, work.as_mut_ptr() as *mut _, &(lwork as c_int),
                      info)
    }
}

#[inline]
pub fn zgeqrfp(m: usize, n: usize, a: &mut [c64], lda: usize, tau: &mut [c64], work: &mut [c64],
               lwork: usize, info: &mut i32) {

    unsafe {
        ffi::zgeqrfp_(&(m as c_int), &(n as c_int), a.as_mut_ptr() as *mut _, &(lda as c_int),
                      tau.as_mut_ptr() as *mut _, work.as_mut_ptr() as *mut _, &(lwork as c_int),
                      info)
    }
}

#[inline]
pub fn clacgv(n: usize, x: &mut [c32], incx: usize) {
    unsafe {
        ffi::clacgv_(&(n as c_int), x.as_mut_ptr() as *mut _, &(incx as c_int))
    }
}

#[inline]
pub fn zlacgv(n: usize, x: &mut [c64], incx: usize) {
    unsafe {
        ffi::zlacgv_(&(n as c_int), x.as_mut_ptr() as *mut _, &(incx as c_int))
    }
}

#[inline]
pub fn slarnv(idist: &[i32], iseed: &mut [i32], n: usize, x: &mut [f32]) {
    unsafe {
        ffi::slarnv_(idist.as_ptr(), iseed.as_mut_ptr(), &(n as c_int), x.as_mut_ptr())
    }
}

#[inline]
pub fn dlarnv(idist: &[i32], iseed: &mut [i32], n: usize, x: &mut [f64]) {
    unsafe {
        ffi::dlarnv_(idist.as_ptr(), iseed.as_mut_ptr(), &(n as c_int), x.as_mut_ptr())
    }
}

#[inline]
pub fn clarnv(idist: &[i32], iseed: &mut [i32], n: usize, x: &mut [c32]) {
    unsafe {
        ffi::clarnv_(idist.as_ptr(), iseed.as_mut_ptr(), &(n as c_int), x.as_mut_ptr() as *mut _)
    }
}

#[inline]
pub fn zlarnv(idist: &[i32], iseed: &mut [i32], n: usize, x: &mut [c64]) {
    unsafe {
        ffi::zlarnv_(idist.as_ptr(), iseed.as_mut_ptr(), &(n as c_int), x.as_mut_ptr() as *mut _)
    }
}

#[inline]
pub fn sgeqr2(m: usize, n: usize, a: &mut [f32], lda: usize, tau: &mut [f32], work: &mut [f32],
              info: &mut i32) {

    unsafe {
        ffi::sgeqr2_(&(m as c_int), &(n as c_int), a.as_mut_ptr(), &(lda as c_int),
                     tau.as_mut_ptr(), work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dgeqr2(m: usize, n: usize, a: &mut [f64], lda: usize, tau: &mut [f64], work: &mut [f64],
              info: &mut i32) {

    unsafe {
        ffi::dgeqr2_(&(m as c_int), &(n as c_int), a.as_mut_ptr(), &(lda as c_int),
                     tau.as_mut_ptr(), work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn cgeqr2(m: usize, n: usize, a: &mut [c32], lda: usize, tau: &mut [c32], work: &mut [c32],
              info: &mut i32) {

    unsafe {
        ffi::cgeqr2_(&(m as c_int), &(n as c_int), a.as_mut_ptr() as *mut _, &(lda as c_int),
                     tau.as_mut_ptr() as *mut _, work.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn zgeqr2(m: usize, n: usize, a: &mut [c64], lda: usize, tau: &mut [c64], work: &mut [c64],
              info: &mut i32) {

    unsafe {
        ffi::zgeqr2_(&(m as c_int), &(n as c_int), a.as_mut_ptr() as *mut _, &(lda as c_int),
                     tau.as_mut_ptr() as *mut _, work.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn slacn2(n: usize, v: &mut [f32], x: &mut [f32], isgn: &mut [i32], est: &mut [f32],
              kase: &mut u32, isave: &mut [i32]) {

    unsafe {
        ffi::slacn2_(&(n as c_int), v.as_mut_ptr(), x.as_mut_ptr(), isgn.as_mut_ptr(),
                     est.as_mut_ptr(), kase as *mut _ as *mut _, isave.as_mut_ptr())
    }
}

#[inline]
pub fn dlacn2(n: usize, v: &mut [f64], x: &mut [f64], isgn: &mut [i32], est: &mut [f64],
              kase: &mut u32, isave: &mut [i32]) {

    unsafe {
        ffi::dlacn2_(&(n as c_int), v.as_mut_ptr(), x.as_mut_ptr(), isgn.as_mut_ptr(),
                     est.as_mut_ptr(), kase as *mut _ as *mut _, isave.as_mut_ptr())
    }
}

#[inline]
pub fn clacn2(n: usize, v: &mut [c32], x: &mut [c32], est: &mut [f32], kase: &mut u32,
              isave: &mut [i32]) {

    unsafe {
        ffi::clacn2_(&(n as c_int), v.as_mut_ptr() as *mut _, x.as_mut_ptr() as *mut _,
                     est.as_mut_ptr(), kase as *mut _ as *mut _, isave.as_mut_ptr())
    }
}

#[inline]
pub fn zlacn2(n: usize, v: &mut [c64], x: &mut [c64], est: &mut [f64], kase: &mut u32,
              isave: &mut [i32]) {

    unsafe {
        ffi::zlacn2_(&(n as c_int), v.as_mut_ptr() as *mut _, x.as_mut_ptr() as *mut _,
                     est.as_mut_ptr(), kase as *mut _ as *mut _, isave.as_mut_ptr())
    }
}

#[inline]
pub fn slacpy(uplo: u8, m: usize, n: usize, a: &[f32], lda: usize, b: &mut [f32], ldb: usize) {
    unsafe {
        ffi::slacpy_(&(uplo as c_char), &(m as c_int), &(n as c_int), a.as_ptr(), &(lda as c_int),
                     b.as_mut_ptr(), &(ldb as c_int))
    }
}

#[inline]
pub fn dlacpy(uplo: u8, m: usize, n: usize, a: &[f64], lda: usize, b: &mut [f64], ldb: usize) {
    unsafe {
        ffi::dlacpy_(&(uplo as c_char), &(m as c_int), &(n as c_int), a.as_ptr(), &(lda as c_int),
                     b.as_mut_ptr(), &(ldb as c_int))
    }
}

#[inline]
pub fn clacpy(uplo: u8, m: usize, n: usize, a: &[c32], lda: usize, b: &mut [c32], ldb: usize) {
    unsafe {
        ffi::clacpy_(&(uplo as c_char), &(m as c_int), &(n as c_int), a.as_ptr() as *const _,
                     &(lda as c_int), b.as_mut_ptr() as *mut _, &(ldb as c_int))
    }
}

#[inline]
pub fn zlacpy(uplo: u8, m: usize, n: usize, a: &[c64], lda: usize, b: &mut [c64], ldb: usize) {
    unsafe {
        ffi::zlacpy_(&(uplo as c_char), &(m as c_int), &(n as c_int), a.as_ptr() as *const _,
                     &(lda as c_int), b.as_mut_ptr() as *mut _, &(ldb as c_int))
    }
}

#[inline]
pub fn clacp2(uplo: u8, m: usize, n: usize, a: &[f32], lda: usize, b: &mut [c32], ldb: usize) {
    unsafe {
        ffi::clacp2_(&(uplo as c_char), &(m as c_int), &(n as c_int), a.as_ptr(), &(lda as c_int),
                     b.as_mut_ptr() as *mut _, &(ldb as c_int))
    }
}

#[inline]
pub fn zlacp2(uplo: u8, m: usize, n: usize, a: &[f64], lda: usize, b: &mut [c64], ldb: usize) {
    unsafe {
        ffi::zlacp2_(&(uplo as c_char), &(m as c_int), &(n as c_int), a.as_ptr(), &(lda as c_int),
                     b.as_mut_ptr() as *mut _, &(ldb as c_int))
    }
}

#[inline]
pub fn sgetf2(m: usize, n: usize, a: &mut [f32], lda: usize, ipiv: &mut [i32], info: &mut i32) {
    unsafe {
        ffi::sgetf2_(&(m as c_int), &(n as c_int), a.as_mut_ptr(), &(lda as c_int),
                     ipiv.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dgetf2(m: usize, n: usize, a: &mut [f64], lda: usize, ipiv: &mut [i32], info: &mut i32) {
    unsafe {
        ffi::dgetf2_(&(m as c_int), &(n as c_int), a.as_mut_ptr(), &(lda as c_int),
                     ipiv.as_mut_ptr(), info)
    }
}

#[inline]
pub fn cgetf2(m: usize, n: usize, a: &mut [c32], lda: usize, ipiv: &mut [i32], info: &mut i32) {
    unsafe {
        ffi::cgetf2_(&(m as c_int), &(n as c_int), a.as_mut_ptr() as *mut _, &(lda as c_int),
                     ipiv.as_mut_ptr(), info)
    }
}

#[inline]
pub fn zgetf2(m: usize, n: usize, a: &mut [c64], lda: usize, ipiv: &mut [i32], info: &mut i32) {
    unsafe {
        ffi::zgetf2_(&(m as c_int), &(n as c_int), a.as_mut_ptr() as *mut _, &(lda as c_int),
                     ipiv.as_mut_ptr(), info)
    }
}

#[inline]
pub fn slaswp(n: usize, a: &mut [f32], lda: usize, k1: usize, k2: usize, ipiv: &[i32],
              incx: usize) {

    unsafe {
        ffi::slaswp_(&(n as c_int), a.as_mut_ptr(), &(lda as c_int), &(k1 as c_int),
                     &(k2 as c_int), ipiv.as_ptr(), &(incx as c_int))
    }
}

#[inline]
pub fn dlaswp(n: usize, a: &mut [f64], lda: usize, k1: usize, k2: usize, ipiv: &[i32],
              incx: usize) {

    unsafe {
        ffi::dlaswp_(&(n as c_int), a.as_mut_ptr(), &(lda as c_int), &(k1 as c_int),
                     &(k2 as c_int), ipiv.as_ptr(), &(incx as c_int))
    }
}

#[inline]
pub fn claswp(n: usize, a: &mut [c32], lda: usize, k1: usize, k2: usize, ipiv: &[i32],
              incx: usize) {

    unsafe {
        ffi::claswp_(&(n as c_int), a.as_mut_ptr() as *mut _, &(lda as c_int), &(k1 as c_int),
                     &(k2 as c_int), ipiv.as_ptr(), &(incx as c_int))
    }
}

#[inline]
pub fn zlaswp(n: usize, a: &mut [c64], lda: usize, k1: usize, k2: usize, ipiv: &[i32],
              incx: usize) {

    unsafe {
        ffi::zlaswp_(&(n as c_int), a.as_mut_ptr() as *mut _, &(lda as c_int), &(k1 as c_int),
                     &(k2 as c_int), ipiv.as_ptr(), &(incx as c_int))
    }
}

#[inline]
pub fn slange(norm: u8, m: usize, n: usize, a: &[f32], lda: usize, work: &mut [f32]) -> f32 {
    unsafe {
        ffi::slange_(&(norm as c_char), &(m as c_int), &(n as c_int), a.as_ptr(), &(lda as c_int),
                     work.as_mut_ptr()) as f32
    }
}

#[inline]
pub fn dlange(norm: u8, m: usize, n: usize, a: &[f64], lda: usize, work: &mut [f64]) -> f64 {
    unsafe {
        ffi::dlange_(&(norm as c_char), &(m as c_int), &(n as c_int), a.as_ptr(), &(lda as c_int),
                     work.as_mut_ptr()) as f64
    }
}

#[inline]
pub fn clange(norm: u8, m: usize, n: usize, a: &[c32], lda: usize, work: &mut [f32]) -> f32 {
    unsafe {
        ffi::clange_(&(norm as c_char), &(m as c_int), &(n as c_int), a.as_ptr() as *const _,
                     &(lda as c_int), work.as_mut_ptr()) as f32
    }
}

#[inline]
pub fn zlange(norm: u8, m: usize, n: usize, a: &[c64], lda: usize, work: &mut [f64]) -> f64 {
    unsafe {
        ffi::zlange_(&(norm as c_char), &(m as c_int), &(n as c_int), a.as_ptr() as *const _,
                     &(lda as c_int), work.as_mut_ptr()) as f64
    }
}

#[inline]
pub fn clanhe(norm: u8, uplo: u8, n: usize, a: &[c32], lda: usize, work: &mut [f32]) -> f32 {
    unsafe {
        ffi::clanhe_(&(norm as c_char), &(uplo as c_char), &(n as c_int), a.as_ptr() as *const _,
                     &(lda as c_int), work.as_mut_ptr()) as f32
    }
}

#[inline]
pub fn zlanhe(norm: u8, uplo: u8, n: usize, a: &[c64], lda: usize, work: &mut [f64]) -> f64 {
    unsafe {
        ffi::zlanhe_(&(norm as c_char), &(uplo as c_char), &(n as c_int), a.as_ptr() as *const _,
                     &(lda as c_int), work.as_mut_ptr()) as f64
    }
}

#[inline]
pub fn slansy(norm: u8, uplo: u8, n: usize, a: &[f32], lda: usize, work: &mut [f32]) -> f32 {
    unsafe {
        ffi::slansy_(&(norm as c_char), &(uplo as c_char), &(n as c_int), a.as_ptr(),
                     &(lda as c_int), work.as_mut_ptr()) as f32
    }
}

#[inline]
pub fn dlansy(norm: u8, uplo: u8, n: usize, a: &[f64], lda: usize, work: &mut [f64]) -> f64 {
    unsafe {
        ffi::dlansy_(&(norm as c_char), &(uplo as c_char), &(n as c_int), a.as_ptr(),
                     &(lda as c_int), work.as_mut_ptr()) as f64
    }
}

#[inline]
pub fn clansy(norm: u8, uplo: u8, n: usize, a: &[c32], lda: usize, work: &mut [f32]) -> f32 {
    unsafe {
        ffi::clansy_(&(norm as c_char), &(uplo as c_char), &(n as c_int), a.as_ptr() as *const _,
                     &(lda as c_int), work.as_mut_ptr()) as f32
    }
}

#[inline]
pub fn zlansy(norm: u8, uplo: u8, n: usize, a: &[c64], lda: usize, work: &mut [f64]) -> f64 {
    unsafe {
        ffi::zlansy_(&(norm as c_char), &(uplo as c_char), &(n as c_int), a.as_ptr() as *const _,
                     &(lda as c_int), work.as_mut_ptr()) as f64
    }
}

#[inline]
pub fn slantr(norm: u8, uplo: u8, diag: u8, m: usize, n: usize, a: &[f32], lda: usize,
              work: &mut [f32]) -> f32 {

    unsafe {
        ffi::slantr_(&(norm as c_char), &(uplo as c_char), &(diag as c_char), &(m as c_int),
                     &(n as c_int), a.as_ptr(), &(lda as c_int), work.as_mut_ptr()) as f32
    }
}

#[inline]
pub fn dlantr(norm: u8, uplo: u8, diag: u8, m: usize, n: usize, a: &[f64], lda: usize,
              work: &mut [f64]) -> f64 {

    unsafe {
        ffi::dlantr_(&(norm as c_char), &(uplo as c_char), &(diag as c_char), &(m as c_int),
                     &(n as c_int), a.as_ptr(), &(lda as c_int), work.as_mut_ptr()) as f64
    }
}

#[inline]
pub fn clantr(norm: u8, uplo: u8, diag: u8, m: usize, n: usize, a: &[c32], lda: usize,
              work: &mut [f32]) -> f32 {

    unsafe {
        ffi::clantr_(&(norm as c_char), &(uplo as c_char), &(diag as c_char), &(m as c_int),
                     &(n as c_int), a.as_ptr() as *const _, &(lda as c_int),
                     work.as_mut_ptr()) as f32
    }
}

#[inline]
pub fn zlantr(norm: u8, uplo: u8, diag: u8, m: usize, n: usize, a: &[c64], lda: usize,
              work: &mut [f64]) -> f64 {

    unsafe {
        ffi::zlantr_(&(norm as c_char), &(uplo as c_char), &(diag as c_char), &(m as c_int),
                     &(n as c_int), a.as_ptr() as *const _, &(lda as c_int),
                     work.as_mut_ptr()) as f64
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
pub fn sgelq2(m: usize, n: usize, a: &mut [f32], lda: usize, tau: &mut [f32], work: &mut [f32],
              info: &mut i32) {

    unsafe {
        ffi::sgelq2_(&(m as c_int), &(n as c_int), a.as_mut_ptr(), &(lda as c_int),
                     tau.as_mut_ptr(), work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dgelq2(m: usize, n: usize, a: &mut [f64], lda: usize, tau: &mut [f64], work: &mut [f64],
              info: &mut i32) {

    unsafe {
        ffi::dgelq2_(&(m as c_int), &(n as c_int), a.as_mut_ptr(), &(lda as c_int),
                     tau.as_mut_ptr(), work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn cgelq2(m: usize, n: usize, a: &mut [c32], lda: usize, tau: &mut [c32], work: &mut [c32],
              info: &mut i32) {

    unsafe {
        ffi::cgelq2_(&(m as c_int), &(n as c_int), a.as_mut_ptr() as *mut _, &(lda as c_int),
                     tau.as_mut_ptr() as *mut _, work.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn zgelq2(m: usize, n: usize, a: &mut [c64], lda: usize, tau: &mut [c64], work: &mut [c64],
              info: &mut i32) {

    unsafe {
        ffi::zgelq2_(&(m as c_int), &(n as c_int), a.as_mut_ptr() as *mut _, &(lda as c_int),
                     tau.as_mut_ptr() as *mut _, work.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn slarfb(side: u8, trans: u8, direct: u8, storev: u8, m: usize, n: usize, k: usize, v: &[f32],
              ldv: usize, t: &[f32], ldt: usize, c: &mut [f32], ldc: usize, work: &mut [f32],
              ldwork: usize) {

    unsafe {
        ffi::slarfb_(&(side as c_char), &(trans as c_char), &(direct as c_char),
                     &(storev as c_char), &(m as c_int), &(n as c_int), &(k as c_int), v.as_ptr(),
                     &(ldv as c_int), t.as_ptr(), &(ldt as c_int), c.as_mut_ptr(), &(ldc as c_int),
                     work.as_mut_ptr(), &(ldwork as c_int))
    }
}

#[inline]
pub fn dlarfb(side: u8, trans: u8, direct: u8, storev: u8, m: usize, n: usize, k: usize, v: &[f64],
              ldv: usize, t: &[f64], ldt: usize, c: &mut [f64], ldc: usize, work: &mut [f64],
              ldwork: usize) {

    unsafe {
        ffi::dlarfb_(&(side as c_char), &(trans as c_char), &(direct as c_char),
                     &(storev as c_char), &(m as c_int), &(n as c_int), &(k as c_int), v.as_ptr(),
                     &(ldv as c_int), t.as_ptr(), &(ldt as c_int), c.as_mut_ptr(), &(ldc as c_int),
                     work.as_mut_ptr(), &(ldwork as c_int))
    }
}

#[inline]
pub fn clarfb(side: u8, trans: u8, direct: u8, storev: u8, m: usize, n: usize, k: usize, v: &[c32],
              ldv: usize, t: &[c32], ldt: usize, c: &mut [c32], ldc: usize, work: &mut [c32],
              ldwork: usize) {

    unsafe {
        ffi::clarfb_(&(side as c_char), &(trans as c_char), &(direct as c_char),
                     &(storev as c_char), &(m as c_int), &(n as c_int), &(k as c_int),
                     v.as_ptr() as *const _, &(ldv as c_int), t.as_ptr() as *const _,
                     &(ldt as c_int), c.as_mut_ptr() as *mut _, &(ldc as c_int),
                     work.as_mut_ptr() as *mut _, &(ldwork as c_int))
    }
}

#[inline]
pub fn zlarfb(side: u8, trans: u8, direct: u8, storev: u8, m: usize, n: usize, k: usize, v: &[c64],
              ldv: usize, t: &[c64], ldt: usize, c: &mut [c64], ldc: usize, work: &mut [c64],
              ldwork: usize) {

    unsafe {
        ffi::zlarfb_(&(side as c_char), &(trans as c_char), &(direct as c_char),
                     &(storev as c_char), &(m as c_int), &(n as c_int), &(k as c_int),
                     v.as_ptr() as *const _, &(ldv as c_int), t.as_ptr() as *const _,
                     &(ldt as c_int), c.as_mut_ptr() as *mut _, &(ldc as c_int),
                     work.as_mut_ptr() as *mut _, &(ldwork as c_int))
    }
}

#[inline]
pub fn slarfg(n: usize, alpha: &mut [f32], x: &mut [f32], incx: usize, tau: &mut [f32]) {
    unsafe {
        ffi::slarfg_(&(n as c_int), alpha.as_mut_ptr(), x.as_mut_ptr(), &(incx as c_int),
                     tau.as_mut_ptr())
    }
}

#[inline]
pub fn dlarfg(n: usize, alpha: &mut [f64], x: &mut [f64], incx: usize, tau: &mut [f64]) {
    unsafe {
        ffi::dlarfg_(&(n as c_int), alpha.as_mut_ptr(), x.as_mut_ptr(), &(incx as c_int),
                     tau.as_mut_ptr())
    }
}

#[inline]
pub fn clarfg(n: usize, alpha: &mut [c32], x: &mut [c32], incx: usize, tau: &mut [c32]) {
    unsafe {
        ffi::clarfg_(&(n as c_int), alpha.as_mut_ptr() as *mut _, x.as_mut_ptr() as *mut _,
                     &(incx as c_int), tau.as_mut_ptr() as *mut _)
    }
}

#[inline]
pub fn zlarfg(n: usize, alpha: &mut [c64], x: &mut [c64], incx: usize, tau: &mut [c64]) {
    unsafe {
        ffi::zlarfg_(&(n as c_int), alpha.as_mut_ptr() as *mut _, x.as_mut_ptr() as *mut _,
                     &(incx as c_int), tau.as_mut_ptr() as *mut _)
    }
}

#[inline]
pub fn slarft(direct: u8, storev: u8, n: usize, k: usize, v: &mut [f32], ldv: usize, tau: &[f32],
              t: &mut [f32], ldt: usize) {

    unsafe {
        ffi::slarft_(&(direct as c_char), &(storev as c_char), &(n as c_int), &(k as c_int),
                     v.as_mut_ptr(), &(ldv as c_int), tau.as_ptr(), t.as_mut_ptr(),
                     &(ldt as c_int))
    }
}

#[inline]
pub fn dlarft(direct: u8, storev: u8, n: usize, k: usize, v: &mut [f64], ldv: usize, tau: &[f64],
              t: &mut [f64], ldt: usize) {

    unsafe {
        ffi::dlarft_(&(direct as c_char), &(storev as c_char), &(n as c_int), &(k as c_int),
                     v.as_mut_ptr(), &(ldv as c_int), tau.as_ptr(), t.as_mut_ptr(),
                     &(ldt as c_int))
    }
}

#[inline]
pub fn clarft(direct: u8, storev: u8, n: usize, k: usize, v: &mut [c32], ldv: usize, tau: &[c32],
              t: &mut [c32], ldt: usize) {

    unsafe {
        ffi::clarft_(&(direct as c_char), &(storev as c_char), &(n as c_int), &(k as c_int),
                     v.as_mut_ptr() as *mut _, &(ldv as c_int), tau.as_ptr() as *const _,
                     t.as_mut_ptr() as *mut _, &(ldt as c_int))
    }
}

#[inline]
pub fn zlarft(direct: u8, storev: u8, n: usize, k: usize, v: &mut [c64], ldv: usize, tau: &[c64],
              t: &mut [c64], ldt: usize) {

    unsafe {
        ffi::zlarft_(&(direct as c_char), &(storev as c_char), &(n as c_int), &(k as c_int),
                     v.as_mut_ptr() as *mut _, &(ldv as c_int), tau.as_ptr() as *const _,
                     t.as_mut_ptr() as *mut _, &(ldt as c_int))
    }
}

#[inline]
pub fn slarfx(side: u8, m: usize, n: usize, v: &[f32], tau: &[f32], c: &mut [f32], ldc: usize,
              work: &mut [f32]) {

    unsafe {
        ffi::slarfx_(&(side as c_char), &(m as c_int), &(n as c_int), v.as_ptr(), tau.as_ptr(),
                     c.as_mut_ptr(), &(ldc as c_int), work.as_mut_ptr())
    }
}

#[inline]
pub fn dlarfx(side: u8, m: usize, n: usize, v: &[f64], tau: &[f64], c: &mut [f64], ldc: usize,
              work: &mut [f64]) {

    unsafe {
        ffi::dlarfx_(&(side as c_char), &(m as c_int), &(n as c_int), v.as_ptr(), tau.as_ptr(),
                     c.as_mut_ptr(), &(ldc as c_int), work.as_mut_ptr())
    }
}

#[inline]
pub fn clarfx(side: u8, m: usize, n: usize, v: &[c32], tau: &[c32], c: &mut [c32], ldc: usize,
              work: &mut [c32]) {

    unsafe {
        ffi::clarfx_(&(side as c_char), &(m as c_int), &(n as c_int), v.as_ptr() as *const _,
                     tau.as_ptr() as *const _, c.as_mut_ptr() as *mut _, &(ldc as c_int),
                     work.as_mut_ptr() as *mut _)
    }
}

#[inline]
pub fn zlarfx(side: u8, m: usize, n: usize, v: &[c64], tau: &[c64], c: &mut [c64], ldc: usize,
              work: &mut [c64]) {

    unsafe {
        ffi::zlarfx_(&(side as c_char), &(m as c_int), &(n as c_int), v.as_ptr() as *const _,
                     tau.as_ptr() as *const _, c.as_mut_ptr() as *mut _, &(ldc as c_int),
                     work.as_mut_ptr() as *mut _)
    }
}

#[inline]
pub fn slatms(m: usize, n: usize, dist: u8, iseed: &mut [i32], sym: u8, d: &mut [f32],
              mode: &[i32], cond: &[f32], dmax: &[f32], kl: usize, ku: usize, pack: u8,
              a: &mut [f32], lda: usize, work: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::slatms_(&(m as c_int), &(n as c_int), &(dist as c_char), iseed.as_mut_ptr(),
                     &(sym as c_char), d.as_mut_ptr(), mode.as_ptr(), cond.as_ptr(), dmax.as_ptr(),
                     &(kl as c_int), &(ku as c_int), &(pack as c_char), a.as_mut_ptr(),
                     &(lda as c_int), work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dlatms(m: usize, n: usize, dist: u8, iseed: &mut [i32], sym: u8, d: &mut [f64],
              mode: &[i32], cond: &[f64], dmax: &[f64], kl: usize, ku: usize, pack: u8,
              a: &mut [f64], lda: usize, work: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::dlatms_(&(m as c_int), &(n as c_int), &(dist as c_char), iseed.as_mut_ptr(),
                     &(sym as c_char), d.as_mut_ptr(), mode.as_ptr(), cond.as_ptr(), dmax.as_ptr(),
                     &(kl as c_int), &(ku as c_int), &(pack as c_char), a.as_mut_ptr(),
                     &(lda as c_int), work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn clatms(m: usize, n: usize, dist: u8, iseed: &mut [i32], sym: u8, d: &mut [f32],
              mode: &[i32], cond: &[f32], dmax: &[f32], kl: usize, ku: usize, pack: u8,
              a: &mut [c32], lda: usize, work: &mut [c32], info: &mut i32) {

    unsafe {
        ffi::clatms_(&(m as c_int), &(n as c_int), &(dist as c_char), iseed.as_mut_ptr(),
                     &(sym as c_char), d.as_mut_ptr(), mode.as_ptr(), cond.as_ptr(), dmax.as_ptr(),
                     &(kl as c_int), &(ku as c_int), &(pack as c_char), a.as_mut_ptr() as *mut _,
                     &(lda as c_int), work.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn zlatms(m: usize, n: usize, dist: u8, iseed: &mut [i32], sym: u8, d: &mut [f64],
              mode: &[i32], cond: &[f64], dmax: &[f64], kl: usize, ku: usize, pack: u8,
              a: &mut [c64], lda: usize, work: &mut [c64], info: &mut i32) {

    unsafe {
        ffi::zlatms_(&(m as c_int), &(n as c_int), &(dist as c_char), iseed.as_mut_ptr(),
                     &(sym as c_char), d.as_mut_ptr(), mode.as_ptr(), cond.as_ptr(), dmax.as_ptr(),
                     &(kl as c_int), &(ku as c_int), &(pack as c_char), a.as_mut_ptr() as *mut _,
                     &(lda as c_int), work.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn dlag2s(m: usize, n: usize, a: &[f64], lda: usize, sa: &mut [f32], ldsa: usize,
              info: &mut i32) {

    unsafe {
        ffi::dlag2s_(&(m as c_int), &(n as c_int), a.as_ptr(), &(lda as c_int), sa.as_mut_ptr(),
                     &(ldsa as c_int), info)
    }
}

#[inline]
pub fn slag2d(m: usize, n: usize, sa: &[f32], ldsa: usize, a: &mut [f64], lda: usize,
              info: &mut i32) {

    unsafe {
        ffi::slag2d_(&(m as c_int), &(n as c_int), sa.as_ptr(), &(ldsa as c_int), a.as_mut_ptr(),
                     &(lda as c_int), info)
    }
}

#[inline]
pub fn zlag2c(m: usize, n: usize, a: &[c64], lda: usize, sa: &mut [c32], ldsa: usize,
              info: &mut i32) {

    unsafe {
        ffi::zlag2c_(&(m as c_int), &(n as c_int), a.as_ptr() as *const _, &(lda as c_int),
                     sa.as_mut_ptr() as *mut _, &(ldsa as c_int), info)
    }
}

#[inline]
pub fn clag2z(m: usize, n: usize, sa: &[c32], ldsa: usize, a: &mut [c64], lda: usize,
              info: &mut i32) {

    unsafe {
        ffi::clag2z_(&(m as c_int), &(n as c_int), sa.as_ptr() as *const _, &(ldsa as c_int),
                     a.as_mut_ptr() as *mut _, &(lda as c_int), info)
    }
}

#[inline]
pub fn slauum(uplo: u8, n: usize, a: &mut [f32], lda: usize, info: &mut i32) {
    unsafe {
        ffi::slauum_(&(uplo as c_char), &(n as c_int), a.as_mut_ptr(), &(lda as c_int), info)
    }
}

#[inline]
pub fn dlauum(uplo: u8, n: usize, a: &mut [f64], lda: usize, info: &mut i32) {
    unsafe {
        ffi::dlauum_(&(uplo as c_char), &(n as c_int), a.as_mut_ptr(), &(lda as c_int), info)
    }
}

#[inline]
pub fn clauum(uplo: u8, n: usize, a: &mut [c32], lda: usize, info: &mut i32) {
    unsafe {
        ffi::clauum_(&(uplo as c_char), &(n as c_int), a.as_mut_ptr() as *mut _, &(lda as c_int),
                     info)
    }
}

#[inline]
pub fn zlauum(uplo: u8, n: usize, a: &mut [c64], lda: usize, info: &mut i32) {
    unsafe {
        ffi::zlauum_(&(uplo as c_char), &(n as c_int), a.as_mut_ptr() as *mut _, &(lda as c_int),
                     info)
    }
}

#[inline]
pub fn slagge(m: usize, n: usize, kl: usize, ku: usize, d: &[f32], a: &mut [f32], lda: usize,
              iseed: &mut [i32], work: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::slagge_(&(m as c_int), &(n as c_int), &(kl as c_int), &(ku as c_int), d.as_ptr(),
                     a.as_mut_ptr(), &(lda as c_int), iseed.as_mut_ptr(), work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dlagge(m: usize, n: usize, kl: usize, ku: usize, d: &[f64], a: &mut [f64], lda: usize,
              iseed: &mut [i32], work: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::dlagge_(&(m as c_int), &(n as c_int), &(kl as c_int), &(ku as c_int), d.as_ptr(),
                     a.as_mut_ptr(), &(lda as c_int), iseed.as_mut_ptr(), work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn clagge(m: usize, n: usize, kl: usize, ku: usize, d: &[f32], a: &mut [c32], lda: usize,
              iseed: &mut [i32], work: &mut [c32], info: &mut i32) {

    unsafe {
        ffi::clagge_(&(m as c_int), &(n as c_int), &(kl as c_int), &(ku as c_int), d.as_ptr(),
                     a.as_mut_ptr() as *mut _, &(lda as c_int), iseed.as_mut_ptr(),
                     work.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn zlagge(m: usize, n: usize, kl: usize, ku: usize, d: &[f64], a: &mut [c64], lda: usize,
              iseed: &mut [i32], work: &mut [c64], info: &mut i32) {

    unsafe {
        ffi::zlagge_(&(m as c_int), &(n as c_int), &(kl as c_int), &(ku as c_int), d.as_ptr(),
                     a.as_mut_ptr() as *mut _, &(lda as c_int), iseed.as_mut_ptr(),
                     work.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn slaset(uplo: u8, m: usize, n: usize, alpha: &[f32], beta: &[f32], a: &mut [f32],
              lda: usize) {

    unsafe {
        ffi::slaset_(&(uplo as c_char), &(m as c_int), &(n as c_int), alpha.as_ptr(),
                     beta.as_ptr(), a.as_mut_ptr(), &(lda as c_int))
    }
}

#[inline]
pub fn dlaset(uplo: u8, m: usize, n: usize, alpha: &[f64], beta: &[f64], a: &mut [f64],
              lda: usize) {

    unsafe {
        ffi::dlaset_(&(uplo as c_char), &(m as c_int), &(n as c_int), alpha.as_ptr(),
                     beta.as_ptr(), a.as_mut_ptr(), &(lda as c_int))
    }
}

#[inline]
pub fn claset(uplo: u8, m: usize, n: usize, alpha: &[c32], beta: &[c32], a: &mut [c32],
              lda: usize) {

    unsafe {
        ffi::claset_(&(uplo as c_char), &(m as c_int), &(n as c_int), alpha.as_ptr() as *const _,
                     beta.as_ptr() as *const _, a.as_mut_ptr() as *mut _, &(lda as c_int))
    }
}

#[inline]
pub fn zlaset(uplo: u8, m: usize, n: usize, alpha: &[c64], beta: &[c64], a: &mut [c64],
              lda: usize) {

    unsafe {
        ffi::zlaset_(&(uplo as c_char), &(m as c_int), &(n as c_int), alpha.as_ptr() as *const _,
                     beta.as_ptr() as *const _, a.as_mut_ptr() as *mut _, &(lda as c_int))
    }
}

#[inline]
pub fn slasrt(id: u8, n: usize, d: &mut [f32], info: &mut i32) {
    unsafe {
        ffi::slasrt_(&(id as c_char), &(n as c_int), d.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dlasrt(id: u8, n: usize, d: &mut [f64], info: &mut i32) {
    unsafe {
        ffi::dlasrt_(&(id as c_char), &(n as c_int), d.as_mut_ptr(), info)
    }
}

#[inline]
pub fn claghe(n: usize, k: usize, d: &[f32], a: &mut [c32], lda: usize, iseed: &mut [i32],
              work: &mut [c32], info: &mut i32) {

    unsafe {
        ffi::claghe_(&(n as c_int), &(k as c_int), d.as_ptr(), a.as_mut_ptr() as *mut _,
                     &(lda as c_int), iseed.as_mut_ptr(), work.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn zlaghe(n: usize, k: usize, d: &[f64], a: &mut [c64], lda: usize, iseed: &mut [i32],
              work: &mut [c64], info: &mut i32) {

    unsafe {
        ffi::zlaghe_(&(n as c_int), &(k as c_int), d.as_ptr(), a.as_mut_ptr() as *mut _,
                     &(lda as c_int), iseed.as_mut_ptr(), work.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn slagsy(n: usize, k: usize, d: &[f32], a: &mut [f32], lda: usize, iseed: &mut [i32],
              work: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::slagsy_(&(n as c_int), &(k as c_int), d.as_ptr(), a.as_mut_ptr(), &(lda as c_int),
                     iseed.as_mut_ptr(), work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dlagsy(n: usize, k: usize, d: &[f64], a: &mut [f64], lda: usize, iseed: &mut [i32],
              work: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::dlagsy_(&(n as c_int), &(k as c_int), d.as_ptr(), a.as_mut_ptr(), &(lda as c_int),
                     iseed.as_mut_ptr(), work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn clagsy(n: usize, k: usize, d: &[f32], a: &mut [c32], lda: usize, iseed: &mut [i32],
              work: &mut [c32], info: &mut i32) {

    unsafe {
        ffi::clagsy_(&(n as c_int), &(k as c_int), d.as_ptr(), a.as_mut_ptr() as *mut _,
                     &(lda as c_int), iseed.as_mut_ptr(), work.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn zlagsy(n: usize, k: usize, d: &[f64], a: &mut [c64], lda: usize, iseed: &mut [i32],
              work: &mut [c64], info: &mut i32) {

    unsafe {
        ffi::zlagsy_(&(n as c_int), &(k as c_int), d.as_ptr(), a.as_mut_ptr() as *mut _,
                     &(lda as c_int), iseed.as_mut_ptr(), work.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn slapmr(forwrd: &[i32], m: usize, n: usize, x: &mut [f32], ldx: usize, k: &mut u32) {
    unsafe {
        ffi::slapmr_(forwrd.as_ptr(), &(m as c_int), &(n as c_int), x.as_mut_ptr(),
                     &(ldx as c_int), k as *mut _ as *mut _)
    }
}

#[inline]
pub fn dlapmr(forwrd: &[i32], m: usize, n: usize, x: &mut [f64], ldx: usize, k: &mut u32) {
    unsafe {
        ffi::dlapmr_(forwrd.as_ptr(), &(m as c_int), &(n as c_int), x.as_mut_ptr(),
                     &(ldx as c_int), k as *mut _ as *mut _)
    }
}

#[inline]
pub fn clapmr(forwrd: &[i32], m: usize, n: usize, x: &mut [c32], ldx: usize, k: &mut u32) {
    unsafe {
        ffi::clapmr_(forwrd.as_ptr(), &(m as c_int), &(n as c_int), x.as_mut_ptr() as *mut _,
                     &(ldx as c_int), k as *mut _ as *mut _)
    }
}

#[inline]
pub fn zlapmr(forwrd: &[i32], m: usize, n: usize, x: &mut [c64], ldx: usize, k: &mut u32) {
    unsafe {
        ffi::zlapmr_(forwrd.as_ptr(), &(m as c_int), &(n as c_int), x.as_mut_ptr() as *mut _,
                     &(ldx as c_int), k as *mut _ as *mut _)
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
pub fn sbbcsd(jobu1: u8, jobu2: u8, jobv1t: u8, jobv2t: u8, trans: u8, m: usize, p: &[i32],
              q: &[i32], theta: &mut [f32], phi: &mut [f32], u1: &mut [f32], ldu1: usize,
              u2: &mut [f32], ldu2: usize, v1t: &mut [f32], ldv1t: usize, v2t: &mut [f32],
              ldv2t: usize, b11d: &mut [f32], b11e: &mut [f32], b12d: &mut [f32], b12e: &mut [f32],
              b21d: &mut [f32], b21e: &mut [f32], b22d: &mut [f32], b22e: &mut [f32],
              work: &mut [f32], lwork: usize, info: &mut i32) {

    unsafe {
        ffi::sbbcsd_(&(jobu1 as c_char), &(jobu2 as c_char), &(jobv1t as c_char),
                     &(jobv2t as c_char), &(trans as c_char), &(m as c_int), p.as_ptr(),
                     q.as_ptr(), theta.as_mut_ptr(), phi.as_mut_ptr(), u1.as_mut_ptr(),
                     &(ldu1 as c_int), u2.as_mut_ptr(), &(ldu2 as c_int), v1t.as_mut_ptr(),
                     &(ldv1t as c_int), v2t.as_mut_ptr(), &(ldv2t as c_int), b11d.as_mut_ptr(),
                     b11e.as_mut_ptr(), b12d.as_mut_ptr(), b12e.as_mut_ptr(), b21d.as_mut_ptr(),
                     b21e.as_mut_ptr(), b22d.as_mut_ptr(), b22e.as_mut_ptr(), work.as_mut_ptr(),
                     &(lwork as c_int), info)
    }
}

#[inline]
pub fn dbbcsd(jobu1: u8, jobu2: u8, jobv1t: u8, jobv2t: u8, trans: u8, m: usize, p: &[i32],
              q: &[i32], theta: &mut [f64], phi: &mut [f64], u1: &mut [f64], ldu1: usize,
              u2: &mut [f64], ldu2: usize, v1t: &mut [f64], ldv1t: usize, v2t: &mut [f64],
              ldv2t: usize, b11d: &mut [f64], b11e: &mut [f64], b12d: &mut [f64], b12e: &mut [f64],
              b21d: &mut [f64], b21e: &mut [f64], b22d: &mut [f64], b22e: &mut [f64],
              work: &mut [f64], lwork: usize, info: &mut i32) {

    unsafe {
        ffi::dbbcsd_(&(jobu1 as c_char), &(jobu2 as c_char), &(jobv1t as c_char),
                     &(jobv2t as c_char), &(trans as c_char), &(m as c_int), p.as_ptr(),
                     q.as_ptr(), theta.as_mut_ptr(), phi.as_mut_ptr(), u1.as_mut_ptr(),
                     &(ldu1 as c_int), u2.as_mut_ptr(), &(ldu2 as c_int), v1t.as_mut_ptr(),
                     &(ldv1t as c_int), v2t.as_mut_ptr(), &(ldv2t as c_int), b11d.as_mut_ptr(),
                     b11e.as_mut_ptr(), b12d.as_mut_ptr(), b12e.as_mut_ptr(), b21d.as_mut_ptr(),
                     b21e.as_mut_ptr(), b22d.as_mut_ptr(), b22e.as_mut_ptr(), work.as_mut_ptr(),
                     &(lwork as c_int), info)
    }
}

#[inline]
pub fn cbbcsd(jobu1: u8, jobu2: u8, jobv1t: u8, jobv2t: u8, trans: u8, m: usize, p: &[i32],
              q: &[i32], theta: &mut [f32], phi: &mut [f32], u1: &mut [c32], ldu1: usize,
              u2: &mut [c32], ldu2: usize, v1t: &mut [c32], ldv1t: usize, v2t: &mut [c32],
              ldv2t: usize, b11d: &mut [f32], b11e: &mut [f32], b12d: &mut [f32], b12e: &mut [f32],
              b21d: &mut [f32], b21e: &mut [f32], b22d: &mut [f32], b22e: &mut [f32],
              rwork: &mut [f32], lrwork: &[i32], info: &mut i32) {

    unsafe {
        ffi::cbbcsd_(&(jobu1 as c_char), &(jobu2 as c_char), &(jobv1t as c_char),
                     &(jobv2t as c_char), &(trans as c_char), &(m as c_int), p.as_ptr(),
                     q.as_ptr(), theta.as_mut_ptr(), phi.as_mut_ptr(), u1.as_mut_ptr() as *mut _,
                     &(ldu1 as c_int), u2.as_mut_ptr() as *mut _, &(ldu2 as c_int),
                     v1t.as_mut_ptr() as *mut _, &(ldv1t as c_int), v2t.as_mut_ptr() as *mut _,
                     &(ldv2t as c_int), b11d.as_mut_ptr(), b11e.as_mut_ptr(), b12d.as_mut_ptr(),
                     b12e.as_mut_ptr(), b21d.as_mut_ptr(), b21e.as_mut_ptr(), b22d.as_mut_ptr(),
                     b22e.as_mut_ptr(), rwork.as_mut_ptr(), lrwork.as_ptr(), info)
    }
}

#[inline]
pub fn zbbcsd(jobu1: u8, jobu2: u8, jobv1t: u8, jobv2t: u8, trans: u8, m: usize, p: &[i32],
              q: &[i32], theta: &mut [f64], phi: &mut [f64], u1: &mut [c64], ldu1: usize,
              u2: &mut [c64], ldu2: usize, v1t: &mut [c64], ldv1t: usize, v2t: &mut [c64],
              ldv2t: usize, b11d: &mut [f64], b11e: &mut [f64], b12d: &mut [f64], b12e: &mut [f64],
              b21d: &mut [f64], b21e: &mut [f64], b22d: &mut [f64], b22e: &mut [f64],
              rwork: &mut [f64], lrwork: &[i32], info: &mut i32) {

    unsafe {
        ffi::zbbcsd_(&(jobu1 as c_char), &(jobu2 as c_char), &(jobv1t as c_char),
                     &(jobv2t as c_char), &(trans as c_char), &(m as c_int), p.as_ptr(),
                     q.as_ptr(), theta.as_mut_ptr(), phi.as_mut_ptr(), u1.as_mut_ptr() as *mut _,
                     &(ldu1 as c_int), u2.as_mut_ptr() as *mut _, &(ldu2 as c_int),
                     v1t.as_mut_ptr() as *mut _, &(ldv1t as c_int), v2t.as_mut_ptr() as *mut _,
                     &(ldv2t as c_int), b11d.as_mut_ptr(), b11e.as_mut_ptr(), b12d.as_mut_ptr(),
                     b12e.as_mut_ptr(), b21d.as_mut_ptr(), b21e.as_mut_ptr(), b22d.as_mut_ptr(),
                     b22e.as_mut_ptr(), rwork.as_mut_ptr(), lrwork.as_ptr(), info)
    }
}

#[inline]
pub fn cheswapr(uplo: u8, n: usize, a: &mut [c32], i1: &[i32], i2: &[i32]) {
    unsafe {
        ffi::cheswapr_(&(uplo as c_char), &(n as c_int), a.as_mut_ptr() as *mut _, i1.as_ptr(),
                       i2.as_ptr())
    }
}

#[inline]
pub fn zheswapr(uplo: u8, n: usize, a: &mut [c64], i1: &[i32], i2: &[i32]) {
    unsafe {
        ffi::zheswapr_(&(uplo as c_char), &(n as c_int), a.as_mut_ptr() as *mut _, i1.as_ptr(),
                       i2.as_ptr())
    }
}

#[inline]
pub fn chetri2(uplo: u8, n: usize, a: &mut [c32], lda: usize, ipiv: &[i32], work: &mut [c32],
               lwork: usize, info: &mut i32) {

    unsafe {
        ffi::chetri2_(&(uplo as c_char), &(n as c_int), a.as_mut_ptr() as *mut _, &(lda as c_int),
                      ipiv.as_ptr(), work.as_mut_ptr() as *mut _, &(lwork as c_int), info)
    }
}

#[inline]
pub fn zhetri2(uplo: u8, n: usize, a: &mut [c64], lda: usize, ipiv: &[i32], work: &mut [c64],
               lwork: usize, info: &mut i32) {

    unsafe {
        ffi::zhetri2_(&(uplo as c_char), &(n as c_int), a.as_mut_ptr() as *mut _, &(lda as c_int),
                      ipiv.as_ptr(), work.as_mut_ptr() as *mut _, &(lwork as c_int), info)
    }
}

#[inline]
pub fn chetri2x(uplo: u8, n: usize, a: &mut [c32], lda: usize, ipiv: &[i32], work: &mut [c32],
                nb: &[i32], info: &mut i32) {

    unsafe {
        ffi::chetri2x_(&(uplo as c_char), &(n as c_int), a.as_mut_ptr() as *mut _, &(lda as c_int),
                       ipiv.as_ptr(), work.as_mut_ptr() as *mut _, nb.as_ptr(), info)
    }
}

#[inline]
pub fn zhetri2x(uplo: u8, n: usize, a: &mut [c64], lda: usize, ipiv: &[i32], work: &mut [c64],
                nb: &[i32], info: &mut i32) {

    unsafe {
        ffi::zhetri2x_(&(uplo as c_char), &(n as c_int), a.as_mut_ptr() as *mut _, &(lda as c_int),
                       ipiv.as_ptr(), work.as_mut_ptr() as *mut _, nb.as_ptr(), info)
    }
}

#[inline]
pub fn chetrs2(uplo: u8, n: usize, nrhs: usize, a: &[c32], lda: usize, ipiv: &[i32], b: &mut [c32],
               ldb: usize, work: &mut [c32], info: &mut i32) {

    unsafe {
        ffi::chetrs2_(&(uplo as c_char), &(n as c_int), &(nrhs as c_int), a.as_ptr() as *const _,
                      &(lda as c_int), ipiv.as_ptr(), b.as_mut_ptr() as *mut _, &(ldb as c_int),
                      work.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn zhetrs2(uplo: u8, n: usize, nrhs: usize, a: &[c64], lda: usize, ipiv: &[i32], b: &mut [c64],
               ldb: usize, work: &mut [c64], info: &mut i32) {

    unsafe {
        ffi::zhetrs2_(&(uplo as c_char), &(n as c_int), &(nrhs as c_int), a.as_ptr() as *const _,
                      &(lda as c_int), ipiv.as_ptr(), b.as_mut_ptr() as *mut _, &(ldb as c_int),
                      work.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn ssyconv(uplo: u8, way: u8, n: usize, a: &mut [f32], lda: usize, ipiv: &[i32],
               work: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::ssyconv_(&(uplo as c_char), &(way as c_char), &(n as c_int), a.as_mut_ptr(),
                      &(lda as c_int), ipiv.as_ptr(), work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dsyconv(uplo: u8, way: u8, n: usize, a: &mut [f64], lda: usize, ipiv: &[i32],
               work: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::dsyconv_(&(uplo as c_char), &(way as c_char), &(n as c_int), a.as_mut_ptr(),
                      &(lda as c_int), ipiv.as_ptr(), work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn csyconv(uplo: u8, way: u8, n: usize, a: &mut [c32], lda: usize, ipiv: &[i32],
               work: &mut [c32], info: &mut i32) {

    unsafe {
        ffi::csyconv_(&(uplo as c_char), &(way as c_char), &(n as c_int), a.as_mut_ptr() as *mut _,
                      &(lda as c_int), ipiv.as_ptr(), work.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn zsyconv(uplo: u8, way: u8, n: usize, a: &mut [c64], lda: usize, ipiv: &[i32],
               work: &mut [c64], info: &mut i32) {

    unsafe {
        ffi::zsyconv_(&(uplo as c_char), &(way as c_char), &(n as c_int), a.as_mut_ptr() as *mut _,
                      &(lda as c_int), ipiv.as_ptr(), work.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn ssyswapr(uplo: u8, n: usize, a: &mut [f32], i1: &[i32], i2: &[i32]) {
    unsafe {
        ffi::ssyswapr_(&(uplo as c_char), &(n as c_int), a.as_mut_ptr(), i1.as_ptr(), i2.as_ptr())
    }
}

#[inline]
pub fn dsyswapr(uplo: u8, n: usize, a: &mut [f64], i1: &[i32], i2: &[i32]) {
    unsafe {
        ffi::dsyswapr_(&(uplo as c_char), &(n as c_int), a.as_mut_ptr(), i1.as_ptr(), i2.as_ptr())
    }
}

#[inline]
pub fn csyswapr(uplo: u8, n: usize, a: &mut [c32], i1: &[i32], i2: &[i32]) {
    unsafe {
        ffi::csyswapr_(&(uplo as c_char), &(n as c_int), a.as_mut_ptr() as *mut _, i1.as_ptr(),
                       i2.as_ptr())
    }
}

#[inline]
pub fn zsyswapr(uplo: u8, n: usize, a: &mut [c64], i1: &[i32], i2: &[i32]) {
    unsafe {
        ffi::zsyswapr_(&(uplo as c_char), &(n as c_int), a.as_mut_ptr() as *mut _, i1.as_ptr(),
                       i2.as_ptr())
    }
}

#[inline]
pub fn ssytri2(uplo: u8, n: usize, a: &mut [f32], lda: usize, ipiv: &[i32], work: &mut [c32],
               lwork: usize, info: &mut i32) {

    unsafe {
        ffi::ssytri2_(&(uplo as c_char), &(n as c_int), a.as_mut_ptr(), &(lda as c_int),
                      ipiv.as_ptr(), work.as_mut_ptr() as *mut _, &(lwork as c_int), info)
    }
}

#[inline]
pub fn dsytri2(uplo: u8, n: usize, a: &mut [f64], lda: usize, ipiv: &[i32], work: &mut [c64],
               lwork: usize, info: &mut i32) {

    unsafe {
        ffi::dsytri2_(&(uplo as c_char), &(n as c_int), a.as_mut_ptr(), &(lda as c_int),
                      ipiv.as_ptr(), work.as_mut_ptr() as *mut _, &(lwork as c_int), info)
    }
}

#[inline]
pub fn csytri2(uplo: u8, n: usize, a: &mut [c32], lda: usize, ipiv: &[i32], work: &mut [c32],
               lwork: usize, info: &mut i32) {

    unsafe {
        ffi::csytri2_(&(uplo as c_char), &(n as c_int), a.as_mut_ptr() as *mut _, &(lda as c_int),
                      ipiv.as_ptr(), work.as_mut_ptr() as *mut _, &(lwork as c_int), info)
    }
}

#[inline]
pub fn zsytri2(uplo: u8, n: usize, a: &mut [c64], lda: usize, ipiv: &[i32], work: &mut [c64],
               lwork: usize, info: &mut i32) {

    unsafe {
        ffi::zsytri2_(&(uplo as c_char), &(n as c_int), a.as_mut_ptr() as *mut _, &(lda as c_int),
                      ipiv.as_ptr(), work.as_mut_ptr() as *mut _, &(lwork as c_int), info)
    }
}

#[inline]
pub fn ssytri2x(uplo: u8, n: usize, a: &mut [f32], lda: usize, ipiv: &[i32], work: &mut [f32],
                nb: &[i32], info: &mut i32) {

    unsafe {
        ffi::ssytri2x_(&(uplo as c_char), &(n as c_int), a.as_mut_ptr(), &(lda as c_int),
                       ipiv.as_ptr(), work.as_mut_ptr(), nb.as_ptr(), info)
    }
}

#[inline]
pub fn dsytri2x(uplo: u8, n: usize, a: &mut [f64], lda: usize, ipiv: &[i32], work: &mut [f64],
                nb: &[i32], info: &mut i32) {

    unsafe {
        ffi::dsytri2x_(&(uplo as c_char), &(n as c_int), a.as_mut_ptr(), &(lda as c_int),
                       ipiv.as_ptr(), work.as_mut_ptr(), nb.as_ptr(), info)
    }
}

#[inline]
pub fn csytri2x(uplo: u8, n: usize, a: &mut [c32], lda: usize, ipiv: &[i32], work: &mut [c32],
                nb: &[i32], info: &mut i32) {

    unsafe {
        ffi::csytri2x_(&(uplo as c_char), &(n as c_int), a.as_mut_ptr() as *mut _, &(lda as c_int),
                       ipiv.as_ptr(), work.as_mut_ptr() as *mut _, nb.as_ptr(), info)
    }
}

#[inline]
pub fn zsytri2x(uplo: u8, n: usize, a: &mut [c64], lda: usize, ipiv: &[i32], work: &mut [c64],
                nb: &[i32], info: &mut i32) {

    unsafe {
        ffi::zsytri2x_(&(uplo as c_char), &(n as c_int), a.as_mut_ptr() as *mut _, &(lda as c_int),
                       ipiv.as_ptr(), work.as_mut_ptr() as *mut _, nb.as_ptr(), info)
    }
}

#[inline]
pub fn ssytrs2(uplo: u8, n: usize, nrhs: usize, a: &[f32], lda: usize, ipiv: &[i32], b: &mut [f32],
               ldb: usize, work: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::ssytrs2_(&(uplo as c_char), &(n as c_int), &(nrhs as c_int), a.as_ptr(),
                      &(lda as c_int), ipiv.as_ptr(), b.as_mut_ptr(), &(ldb as c_int),
                      work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dsytrs2(uplo: u8, n: usize, nrhs: usize, a: &[f64], lda: usize, ipiv: &[i32], b: &mut [f64],
               ldb: usize, work: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::dsytrs2_(&(uplo as c_char), &(n as c_int), &(nrhs as c_int), a.as_ptr(),
                      &(lda as c_int), ipiv.as_ptr(), b.as_mut_ptr(), &(ldb as c_int),
                      work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn csytrs2(uplo: u8, n: usize, nrhs: usize, a: &[c32], lda: usize, ipiv: &[i32], b: &mut [c32],
               ldb: usize, work: &mut [c32], info: &mut i32) {

    unsafe {
        ffi::csytrs2_(&(uplo as c_char), &(n as c_int), &(nrhs as c_int), a.as_ptr() as *const _,
                      &(lda as c_int), ipiv.as_ptr(), b.as_mut_ptr() as *mut _, &(ldb as c_int),
                      work.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn zsytrs2(uplo: u8, n: usize, nrhs: usize, a: &[c64], lda: usize, ipiv: &[i32], b: &mut [c64],
               ldb: usize, work: &mut [c64], info: &mut i32) {

    unsafe {
        ffi::zsytrs2_(&(uplo as c_char), &(n as c_int), &(nrhs as c_int), a.as_ptr() as *const _,
                      &(lda as c_int), ipiv.as_ptr(), b.as_mut_ptr() as *mut _, &(ldb as c_int),
                      work.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn cunbdb(trans: u8, signs: u8, m: usize, p: &[i32], q: &[i32], x11: &mut [c32], ldx11: usize,
              x12: &mut [c32], ldx12: usize, x21: &mut [c32], ldx21: usize, x22: &mut [c32],
              ldx22: usize, theta: &mut [f32], phi: &mut [f32], taup1: &mut [c32],
              taup2: &mut [c32], tauq1: &mut [c32], tauq2: &mut [c32], work: &mut [c32],
              lwork: usize, info: &mut i32) {

    unsafe {
        ffi::cunbdb_(&(trans as c_char), &(signs as c_char), &(m as c_int), p.as_ptr(), q.as_ptr(),
                     x11.as_mut_ptr() as *mut _, &(ldx11 as c_int), x12.as_mut_ptr() as *mut _,
                     &(ldx12 as c_int), x21.as_mut_ptr() as *mut _, &(ldx21 as c_int),
                     x22.as_mut_ptr() as *mut _, &(ldx22 as c_int), theta.as_mut_ptr(),
                     phi.as_mut_ptr(), taup1.as_mut_ptr() as *mut _, taup2.as_mut_ptr() as *mut _,
                     tauq1.as_mut_ptr() as *mut _, tauq2.as_mut_ptr() as *mut _,
                     work.as_mut_ptr() as *mut _, &(lwork as c_int), info)
    }
}

#[inline]
pub fn zunbdb(trans: u8, signs: u8, m: usize, p: &[i32], q: &[i32], x11: &mut [c64], ldx11: usize,
              x12: &mut [c64], ldx12: usize, x21: &mut [c64], ldx21: usize, x22: &mut [c64],
              ldx22: usize, theta: &mut [f64], phi: &mut [f64], taup1: &mut [c64],
              taup2: &mut [c64], tauq1: &mut [c64], tauq2: &mut [c64], work: &mut [c64],
              lwork: usize, info: &mut i32) {

    unsafe {
        ffi::zunbdb_(&(trans as c_char), &(signs as c_char), &(m as c_int), p.as_ptr(), q.as_ptr(),
                     x11.as_mut_ptr() as *mut _, &(ldx11 as c_int), x12.as_mut_ptr() as *mut _,
                     &(ldx12 as c_int), x21.as_mut_ptr() as *mut _, &(ldx21 as c_int),
                     x22.as_mut_ptr() as *mut _, &(ldx22 as c_int), theta.as_mut_ptr(),
                     phi.as_mut_ptr(), taup1.as_mut_ptr() as *mut _, taup2.as_mut_ptr() as *mut _,
                     tauq1.as_mut_ptr() as *mut _, tauq2.as_mut_ptr() as *mut _,
                     work.as_mut_ptr() as *mut _, &(lwork as c_int), info)
    }
}

#[inline]
pub fn cuncsd(jobu1: u8, jobu2: u8, jobv1t: u8, jobv2t: u8, trans: u8, signs: u8, m: usize,
              p: &[i32], q: &[i32], x11: &mut [c32], ldx11: &mut u32, x12: &mut [c32],
              ldx12: &mut u32, x21: &mut [c32], ldx21: &mut u32, x22: &mut [c32], ldx22: &mut u32,
              theta: &mut [f32], u1: &mut [c32], ldu1: usize, u2: &mut [c32], ldu2: usize,
              v1t: &mut [c32], ldv1t: usize, v2t: &mut [c32], ldv2t: usize, work: &mut [c32],
              lwork: usize, rwork: &mut [f32], lrwork: &[i32], iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::cuncsd_(&(jobu1 as c_char), &(jobu2 as c_char), &(jobv1t as c_char),
                     &(jobv2t as c_char), &(trans as c_char), &(signs as c_char), &(m as c_int),
                     p.as_ptr(), q.as_ptr(), x11.as_mut_ptr() as *mut _, ldx11 as *mut _ as *mut _,
                     x12.as_mut_ptr() as *mut _, ldx12 as *mut _ as *mut _,
                     x21.as_mut_ptr() as *mut _, ldx21 as *mut _ as *mut _,
                     x22.as_mut_ptr() as *mut _, ldx22 as *mut _ as *mut _, theta.as_mut_ptr(),
                     u1.as_mut_ptr() as *mut _, &(ldu1 as c_int), u2.as_mut_ptr() as *mut _,
                     &(ldu2 as c_int), v1t.as_mut_ptr() as *mut _, &(ldv1t as c_int),
                     v2t.as_mut_ptr() as *mut _, &(ldv2t as c_int), work.as_mut_ptr() as *mut _,
                     &(lwork as c_int), rwork.as_mut_ptr(), lrwork.as_ptr(), iwork.as_mut_ptr(),
                     info)
    }
}

#[inline]
pub fn zuncsd(jobu1: u8, jobu2: u8, jobv1t: u8, jobv2t: u8, trans: u8, signs: u8, m: usize,
              p: &[i32], q: &[i32], x11: &mut [c64], ldx11: &mut u32, x12: &mut [c64],
              ldx12: &mut u32, x21: &mut [c64], ldx21: &mut u32, x22: &mut [c64], ldx22: &mut u32,
              theta: &mut [f64], u1: &mut [c64], ldu1: usize, u2: &mut [c64], ldu2: usize,
              v1t: &mut [c64], ldv1t: usize, v2t: &mut [c64], ldv2t: usize, work: &mut [c64],
              lwork: usize, rwork: &mut [f64], lrwork: &[i32], iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::zuncsd_(&(jobu1 as c_char), &(jobu2 as c_char), &(jobv1t as c_char),
                     &(jobv2t as c_char), &(trans as c_char), &(signs as c_char), &(m as c_int),
                     p.as_ptr(), q.as_ptr(), x11.as_mut_ptr() as *mut _, ldx11 as *mut _ as *mut _,
                     x12.as_mut_ptr() as *mut _, ldx12 as *mut _ as *mut _,
                     x21.as_mut_ptr() as *mut _, ldx21 as *mut _ as *mut _,
                     x22.as_mut_ptr() as *mut _, ldx22 as *mut _ as *mut _, theta.as_mut_ptr(),
                     u1.as_mut_ptr() as *mut _, &(ldu1 as c_int), u2.as_mut_ptr() as *mut _,
                     &(ldu2 as c_int), v1t.as_mut_ptr() as *mut _, &(ldv1t as c_int),
                     v2t.as_mut_ptr() as *mut _, &(ldv2t as c_int), work.as_mut_ptr() as *mut _,
                     &(lwork as c_int), rwork.as_mut_ptr(), lrwork.as_ptr(), iwork.as_mut_ptr(),
                     info)
    }
}

#[inline]
pub fn sorbdb(trans: u8, signs: u8, m: usize, p: &[i32], q: &[i32], x11: &mut [f32], ldx11: usize,
              x12: &mut [f32], ldx12: usize, x21: &mut [f32], ldx21: usize, x22: &mut [f32],
              ldx22: usize, theta: &mut [f32], phi: &mut [f32], taup1: &mut [f32],
              taup2: &mut [f32], tauq1: &mut [f32], tauq2: &mut [f32], work: &mut [f32],
              lwork: usize, info: &mut i32) {

    unsafe {
        ffi::sorbdb_(&(trans as c_char), &(signs as c_char), &(m as c_int), p.as_ptr(), q.as_ptr(),
                     x11.as_mut_ptr(), &(ldx11 as c_int), x12.as_mut_ptr(), &(ldx12 as c_int),
                     x21.as_mut_ptr(), &(ldx21 as c_int), x22.as_mut_ptr(), &(ldx22 as c_int),
                     theta.as_mut_ptr(), phi.as_mut_ptr(), taup1.as_mut_ptr(), taup2.as_mut_ptr(),
                     tauq1.as_mut_ptr(), tauq2.as_mut_ptr(), work.as_mut_ptr(), &(lwork as c_int),
                     info)
    }
}

#[inline]
pub fn dorbdb(trans: u8, signs: u8, m: usize, p: &[i32], q: &[i32], x11: &mut [f64], ldx11: usize,
              x12: &mut [f64], ldx12: usize, x21: &mut [f64], ldx21: usize, x22: &mut [f64],
              ldx22: usize, theta: &mut [f64], phi: &mut [f64], taup1: &mut [f64],
              taup2: &mut [f64], tauq1: &mut [f64], tauq2: &mut [f64], work: &mut [f64],
              lwork: usize, info: &mut i32) {

    unsafe {
        ffi::dorbdb_(&(trans as c_char), &(signs as c_char), &(m as c_int), p.as_ptr(), q.as_ptr(),
                     x11.as_mut_ptr(), &(ldx11 as c_int), x12.as_mut_ptr(), &(ldx12 as c_int),
                     x21.as_mut_ptr(), &(ldx21 as c_int), x22.as_mut_ptr(), &(ldx22 as c_int),
                     theta.as_mut_ptr(), phi.as_mut_ptr(), taup1.as_mut_ptr(), taup2.as_mut_ptr(),
                     tauq1.as_mut_ptr(), tauq2.as_mut_ptr(), work.as_mut_ptr(), &(lwork as c_int),
                     info)
    }
}

#[inline]
pub fn sorcsd(jobu1: u8, jobu2: u8, jobv1t: u8, jobv2t: u8, trans: u8, signs: u8, m: usize,
              p: &[i32], q: &[i32], x11: &mut [f32], ldx11: usize, x12: &mut [f32], ldx12: usize,
              x21: &mut [f32], ldx21: usize, x22: &mut [f32], ldx22: usize, theta: &mut [f32],
              u1: &mut [f32], ldu1: usize, u2: &mut [f32], ldu2: usize, v1t: &mut [f32],
              ldv1t: usize, v2t: &mut [f32], ldv2t: usize, work: &mut [f32], lwork: usize,
              iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::sorcsd_(&(jobu1 as c_char), &(jobu2 as c_char), &(jobv1t as c_char),
                     &(jobv2t as c_char), &(trans as c_char), &(signs as c_char), &(m as c_int),
                     p.as_ptr(), q.as_ptr(), x11.as_mut_ptr(), &(ldx11 as c_int), x12.as_mut_ptr(),
                     &(ldx12 as c_int), x21.as_mut_ptr(), &(ldx21 as c_int), x22.as_mut_ptr(),
                     &(ldx22 as c_int), theta.as_mut_ptr(), u1.as_mut_ptr(), &(ldu1 as c_int),
                     u2.as_mut_ptr(), &(ldu2 as c_int), v1t.as_mut_ptr(), &(ldv1t as c_int),
                     v2t.as_mut_ptr(), &(ldv2t as c_int), work.as_mut_ptr(), &(lwork as c_int),
                     iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dorcsd(jobu1: u8, jobu2: u8, jobv1t: u8, jobv2t: u8, trans: u8, signs: u8, m: usize,
              p: &[i32], q: &[i32], x11: &mut [f64], ldx11: usize, x12: &mut [f64], ldx12: usize,
              x21: &mut [f64], ldx21: usize, x22: &mut [f64], ldx22: usize, theta: &mut [f64],
              u1: &mut [f64], ldu1: usize, u2: &mut [f64], ldu2: usize, v1t: &mut [f64],
              ldv1t: usize, v2t: &mut [f64], ldv2t: usize, work: &mut [f64], lwork: usize,
              iwork: &mut [i32], info: &mut i32) {

    unsafe {
        ffi::dorcsd_(&(jobu1 as c_char), &(jobu2 as c_char), &(jobv1t as c_char),
                     &(jobv2t as c_char), &(trans as c_char), &(signs as c_char), &(m as c_int),
                     p.as_ptr(), q.as_ptr(), x11.as_mut_ptr(), &(ldx11 as c_int), x12.as_mut_ptr(),
                     &(ldx12 as c_int), x21.as_mut_ptr(), &(ldx21 as c_int), x22.as_mut_ptr(),
                     &(ldx22 as c_int), theta.as_mut_ptr(), u1.as_mut_ptr(), &(ldu1 as c_int),
                     u2.as_mut_ptr(), &(ldu2 as c_int), v1t.as_mut_ptr(), &(ldv1t as c_int),
                     v2t.as_mut_ptr(), &(ldv2t as c_int), work.as_mut_ptr(), &(lwork as c_int),
                     iwork.as_mut_ptr(), info)
    }
}

#[inline]
pub fn sgemqrt(side: u8, trans: u8, m: usize, n: usize, k: usize, nb: &[i32], v: &[f32],
               ldv: usize, t: &[f32], ldt: usize, c: &mut [f32], ldc: usize, work: &mut [f32],
               info: &mut i32) {

    unsafe {
        ffi::sgemqrt_(&(side as c_char), &(trans as c_char), &(m as c_int), &(n as c_int),
                      &(k as c_int), nb.as_ptr(), v.as_ptr(), &(ldv as c_int), t.as_ptr(),
                      &(ldt as c_int), c.as_mut_ptr(), &(ldc as c_int), work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dgemqrt(side: u8, trans: u8, m: usize, n: usize, k: usize, nb: &[i32], v: &[f64],
               ldv: usize, t: &[f64], ldt: usize, c: &mut [f64], ldc: usize, work: &mut [f64],
               info: &mut i32) {

    unsafe {
        ffi::dgemqrt_(&(side as c_char), &(trans as c_char), &(m as c_int), &(n as c_int),
                      &(k as c_int), nb.as_ptr(), v.as_ptr(), &(ldv as c_int), t.as_ptr(),
                      &(ldt as c_int), c.as_mut_ptr(), &(ldc as c_int), work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn cgemqrt(side: u8, trans: u8, m: usize, n: usize, k: usize, nb: &[i32], v: &[c32],
               ldv: usize, t: &[c32], ldt: usize, c: &mut [c32], ldc: usize, work: &mut [c32],
               info: &mut i32) {

    unsafe {
        ffi::cgemqrt_(&(side as c_char), &(trans as c_char), &(m as c_int), &(n as c_int),
                      &(k as c_int), nb.as_ptr(), v.as_ptr() as *const _, &(ldv as c_int),
                      t.as_ptr() as *const _, &(ldt as c_int), c.as_mut_ptr() as *mut _,
                      &(ldc as c_int), work.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn zgemqrt(side: u8, trans: u8, m: usize, n: usize, k: usize, nb: &[i32], v: &[c64],
               ldv: usize, t: &[c64], ldt: usize, c: &mut [c64], ldc: usize, work: &mut [c64],
               info: &mut i32) {

    unsafe {
        ffi::zgemqrt_(&(side as c_char), &(trans as c_char), &(m as c_int), &(n as c_int),
                      &(k as c_int), nb.as_ptr(), v.as_ptr() as *const _, &(ldv as c_int),
                      t.as_ptr() as *const _, &(ldt as c_int), c.as_mut_ptr() as *mut _,
                      &(ldc as c_int), work.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn sgeqrt(m: usize, n: usize, nb: &[i32], a: &mut [f32], lda: usize, t: &mut [f32], ldt: usize,
              work: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::sgeqrt_(&(m as c_int), &(n as c_int), nb.as_ptr(), a.as_mut_ptr(), &(lda as c_int),
                     t.as_mut_ptr(), &(ldt as c_int), work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dgeqrt(m: usize, n: usize, nb: &[i32], a: &mut [f64], lda: usize, t: &mut [f64], ldt: usize,
              work: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::dgeqrt_(&(m as c_int), &(n as c_int), nb.as_ptr(), a.as_mut_ptr(), &(lda as c_int),
                     t.as_mut_ptr(), &(ldt as c_int), work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn cgeqrt(m: usize, n: usize, nb: &[i32], a: &mut [c32], lda: usize, t: &mut [c32], ldt: usize,
              work: &mut [c32], info: &mut i32) {

    unsafe {
        ffi::cgeqrt_(&(m as c_int), &(n as c_int), nb.as_ptr(), a.as_mut_ptr() as *mut _,
                     &(lda as c_int), t.as_mut_ptr() as *mut _, &(ldt as c_int),
                     work.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn zgeqrt(m: usize, n: usize, nb: &[i32], a: &mut [c64], lda: usize, t: &mut [c64], ldt: usize,
              work: &mut [c64], info: &mut i32) {

    unsafe {
        ffi::zgeqrt_(&(m as c_int), &(n as c_int), nb.as_ptr(), a.as_mut_ptr() as *mut _,
                     &(lda as c_int), t.as_mut_ptr() as *mut _, &(ldt as c_int),
                     work.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn sgeqrt2(m: usize, n: usize, a: &mut [f32], lda: usize, t: &mut [f32], ldt: usize,
               info: &mut i32) {

    unsafe {
        ffi::sgeqrt2_(&(m as c_int), &(n as c_int), a.as_mut_ptr(), &(lda as c_int),
                      t.as_mut_ptr(), &(ldt as c_int), info)
    }
}

#[inline]
pub fn dgeqrt2(m: usize, n: usize, a: &mut [f64], lda: usize, t: &mut [f64], ldt: usize,
               info: &mut i32) {

    unsafe {
        ffi::dgeqrt2_(&(m as c_int), &(n as c_int), a.as_mut_ptr(), &(lda as c_int),
                      t.as_mut_ptr(), &(ldt as c_int), info)
    }
}

#[inline]
pub fn cgeqrt2(m: usize, n: usize, a: &mut [c32], lda: usize, t: &mut [c32], ldt: usize,
               info: &mut i32) {

    unsafe {
        ffi::cgeqrt2_(&(m as c_int), &(n as c_int), a.as_mut_ptr() as *mut _, &(lda as c_int),
                      t.as_mut_ptr() as *mut _, &(ldt as c_int), info)
    }
}

#[inline]
pub fn zgeqrt2(m: usize, n: usize, a: &mut [c64], lda: usize, t: &mut [c64], ldt: usize,
               info: &mut i32) {

    unsafe {
        ffi::zgeqrt2_(&(m as c_int), &(n as c_int), a.as_mut_ptr() as *mut _, &(lda as c_int),
                      t.as_mut_ptr() as *mut _, &(ldt as c_int), info)
    }
}

#[inline]
pub fn sgeqrt3(m: usize, n: usize, a: &mut [f32], lda: usize, t: &mut [f32], ldt: usize,
               info: &mut i32) {

    unsafe {
        ffi::sgeqrt3_(&(m as c_int), &(n as c_int), a.as_mut_ptr(), &(lda as c_int),
                      t.as_mut_ptr(), &(ldt as c_int), info)
    }
}

#[inline]
pub fn dgeqrt3(m: usize, n: usize, a: &mut [f64], lda: usize, t: &mut [f64], ldt: usize,
               info: &mut i32) {

    unsafe {
        ffi::dgeqrt3_(&(m as c_int), &(n as c_int), a.as_mut_ptr(), &(lda as c_int),
                      t.as_mut_ptr(), &(ldt as c_int), info)
    }
}

#[inline]
pub fn cgeqrt3(m: usize, n: usize, a: &mut [c32], lda: usize, t: &mut [c32], ldt: usize,
               info: &mut i32) {

    unsafe {
        ffi::cgeqrt3_(&(m as c_int), &(n as c_int), a.as_mut_ptr() as *mut _, &(lda as c_int),
                      t.as_mut_ptr() as *mut _, &(ldt as c_int), info)
    }
}

#[inline]
pub fn zgeqrt3(m: usize, n: usize, a: &mut [c64], lda: usize, t: &mut [c64], ldt: usize,
               info: &mut i32) {

    unsafe {
        ffi::zgeqrt3_(&(m as c_int), &(n as c_int), a.as_mut_ptr() as *mut _, &(lda as c_int),
                      t.as_mut_ptr() as *mut _, &(ldt as c_int), info)
    }
}

#[inline]
pub fn stpmqrt(side: u8, trans: u8, m: usize, n: usize, k: usize, l: &[i32], nb: &[i32], v: &[f32],
               ldv: usize, t: &[f32], ldt: usize, a: &mut [f32], lda: usize, b: &mut [f32],
               ldb: usize, work: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::stpmqrt_(&(side as c_char), &(trans as c_char), &(m as c_int), &(n as c_int),
                      &(k as c_int), l.as_ptr(), nb.as_ptr(), v.as_ptr(), &(ldv as c_int),
                      t.as_ptr(), &(ldt as c_int), a.as_mut_ptr(), &(lda as c_int), b.as_mut_ptr(),
                      &(ldb as c_int), work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dtpmqrt(side: u8, trans: u8, m: usize, n: usize, k: usize, l: &[i32], nb: &[i32], v: &[f64],
               ldv: usize, t: &[f64], ldt: usize, a: &mut [f64], lda: usize, b: &mut [f64],
               ldb: usize, work: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::dtpmqrt_(&(side as c_char), &(trans as c_char), &(m as c_int), &(n as c_int),
                      &(k as c_int), l.as_ptr(), nb.as_ptr(), v.as_ptr(), &(ldv as c_int),
                      t.as_ptr(), &(ldt as c_int), a.as_mut_ptr(), &(lda as c_int), b.as_mut_ptr(),
                      &(ldb as c_int), work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn ctpmqrt(side: u8, trans: u8, m: usize, n: usize, k: usize, l: &[i32], nb: &[i32], v: &[c32],
               ldv: usize, t: &[c32], ldt: usize, a: &mut [c32], lda: usize, b: &mut [c32],
               ldb: usize, work: &mut [c32], info: &mut i32) {

    unsafe {
        ffi::ctpmqrt_(&(side as c_char), &(trans as c_char), &(m as c_int), &(n as c_int),
                      &(k as c_int), l.as_ptr(), nb.as_ptr(), v.as_ptr() as *const _,
                      &(ldv as c_int), t.as_ptr() as *const _, &(ldt as c_int),
                      a.as_mut_ptr() as *mut _, &(lda as c_int), b.as_mut_ptr() as *mut _,
                      &(ldb as c_int), work.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn ztpmqrt(side: u8, trans: u8, m: usize, n: usize, k: usize, l: &[i32], nb: &[i32], v: &[c64],
               ldv: usize, t: &[c64], ldt: usize, a: &mut [c64], lda: usize, b: &mut [c64],
               ldb: usize, work: &mut [c64], info: &mut i32) {

    unsafe {
        ffi::ztpmqrt_(&(side as c_char), &(trans as c_char), &(m as c_int), &(n as c_int),
                      &(k as c_int), l.as_ptr(), nb.as_ptr(), v.as_ptr() as *const _,
                      &(ldv as c_int), t.as_ptr() as *const _, &(ldt as c_int),
                      a.as_mut_ptr() as *mut _, &(lda as c_int), b.as_mut_ptr() as *mut _,
                      &(ldb as c_int), work.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn stpqrt(m: usize, n: usize, l: &[i32], nb: &[i32], a: &mut [f32], lda: usize, b: &mut [f32],
              ldb: usize, t: &mut [f32], ldt: usize, work: &mut [f32], info: &mut i32) {

    unsafe {
        ffi::stpqrt_(&(m as c_int), &(n as c_int), l.as_ptr(), nb.as_ptr(), a.as_mut_ptr(),
                     &(lda as c_int), b.as_mut_ptr(), &(ldb as c_int), t.as_mut_ptr(),
                     &(ldt as c_int), work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn dtpqrt(m: usize, n: usize, l: &[i32], nb: &[i32], a: &mut [f64], lda: usize, b: &mut [f64],
              ldb: usize, t: &mut [f64], ldt: usize, work: &mut [f64], info: &mut i32) {

    unsafe {
        ffi::dtpqrt_(&(m as c_int), &(n as c_int), l.as_ptr(), nb.as_ptr(), a.as_mut_ptr(),
                     &(lda as c_int), b.as_mut_ptr(), &(ldb as c_int), t.as_mut_ptr(),
                     &(ldt as c_int), work.as_mut_ptr(), info)
    }
}

#[inline]
pub fn ctpqrt(m: usize, n: usize, l: &[i32], nb: &[i32], a: &mut [c32], lda: usize, b: &mut [c32],
              ldb: usize, t: &mut [c32], ldt: usize, work: &mut [c32], info: &mut i32) {

    unsafe {
        ffi::ctpqrt_(&(m as c_int), &(n as c_int), l.as_ptr(), nb.as_ptr(),
                     a.as_mut_ptr() as *mut _, &(lda as c_int), b.as_mut_ptr() as *mut _,
                     &(ldb as c_int), t.as_mut_ptr() as *mut _, &(ldt as c_int),
                     work.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn ztpqrt(m: usize, n: usize, l: &[i32], nb: &[i32], a: &mut [c64], lda: usize, b: &mut [c64],
              ldb: usize, t: &mut [c64], ldt: usize, work: &mut [c64], info: &mut i32) {

    unsafe {
        ffi::ztpqrt_(&(m as c_int), &(n as c_int), l.as_ptr(), nb.as_ptr(),
                     a.as_mut_ptr() as *mut _, &(lda as c_int), b.as_mut_ptr() as *mut _,
                     &(ldb as c_int), t.as_mut_ptr() as *mut _, &(ldt as c_int),
                     work.as_mut_ptr() as *mut _, info)
    }
}

#[inline]
pub fn stpqrt2(m: usize, n: usize, l: &[i32], a: &mut [f32], lda: usize, b: &mut [f32], ldb: usize,
               t: &mut [f32], ldt: usize, info: &mut i32) {

    unsafe {
        ffi::stpqrt2_(&(m as c_int), &(n as c_int), l.as_ptr(), a.as_mut_ptr(), &(lda as c_int),
                      b.as_mut_ptr(), &(ldb as c_int), t.as_mut_ptr(), &(ldt as c_int), info)
    }
}

#[inline]
pub fn dtpqrt2(m: usize, n: usize, l: &[i32], a: &mut [f64], lda: usize, b: &mut [f64], ldb: usize,
               t: &mut [f64], ldt: usize, info: &mut i32) {

    unsafe {
        ffi::dtpqrt2_(&(m as c_int), &(n as c_int), l.as_ptr(), a.as_mut_ptr(), &(lda as c_int),
                      b.as_mut_ptr(), &(ldb as c_int), t.as_mut_ptr(), &(ldt as c_int), info)
    }
}

#[inline]
pub fn ctpqrt2(m: usize, n: usize, l: &[i32], a: &mut [c32], lda: usize, b: &mut [c32], ldb: usize,
               t: &mut [c32], ldt: usize, info: &mut i32) {

    unsafe {
        ffi::ctpqrt2_(&(m as c_int), &(n as c_int), l.as_ptr(), a.as_mut_ptr() as *mut _,
                      &(lda as c_int), b.as_mut_ptr() as *mut _, &(ldb as c_int),
                      t.as_mut_ptr() as *mut _, &(ldt as c_int), info)
    }
}

#[inline]
pub fn ztpqrt2(m: usize, n: usize, l: &[i32], a: &mut [c64], lda: usize, b: &mut [c64], ldb: usize,
               t: &mut [c64], ldt: usize, info: &mut i32) {

    unsafe {
        ffi::ztpqrt2_(&(m as c_int), &(n as c_int), l.as_ptr(), a.as_mut_ptr() as *mut _,
                      &(lda as c_int), b.as_mut_ptr() as *mut _, &(ldb as c_int),
                      t.as_mut_ptr() as *mut _, &(ldt as c_int), info)
    }
}

#[inline]
pub fn stprfb(side: u8, trans: u8, direct: u8, storev: u8, m: usize, n: usize, k: usize, l: &[i32],
              v: &[f32], ldv: usize, t: &[f32], ldt: usize, a: &mut [f32], lda: usize,
              b: &mut [f32], ldb: usize, work: &[f32], ldwork: usize) {

    unsafe {
        ffi::stprfb_(&(side as c_char), &(trans as c_char), &(direct as c_char),
                     &(storev as c_char), &(m as c_int), &(n as c_int), &(k as c_int), l.as_ptr(),
                     v.as_ptr(), &(ldv as c_int), t.as_ptr(), &(ldt as c_int), a.as_mut_ptr(),
                     &(lda as c_int), b.as_mut_ptr(), &(ldb as c_int), work.as_ptr(),
                     &(ldwork as c_int))
    }
}

#[inline]
pub fn dtprfb(side: u8, trans: u8, direct: u8, storev: u8, m: usize, n: usize, k: usize, l: &[i32],
              v: &[f64], ldv: usize, t: &[f64], ldt: usize, a: &mut [f64], lda: usize,
              b: &mut [f64], ldb: usize, work: &[f64], ldwork: usize) {

    unsafe {
        ffi::dtprfb_(&(side as c_char), &(trans as c_char), &(direct as c_char),
                     &(storev as c_char), &(m as c_int), &(n as c_int), &(k as c_int), l.as_ptr(),
                     v.as_ptr(), &(ldv as c_int), t.as_ptr(), &(ldt as c_int), a.as_mut_ptr(),
                     &(lda as c_int), b.as_mut_ptr(), &(ldb as c_int), work.as_ptr(),
                     &(ldwork as c_int))
    }
}

#[inline]
pub fn ctprfb(side: u8, trans: u8, direct: u8, storev: u8, m: usize, n: usize, k: usize, l: &[i32],
              v: &[c32], ldv: usize, t: &[c32], ldt: usize, a: &mut [c32], lda: usize,
              b: &mut [c32], ldb: usize, work: &[f32], ldwork: usize) {

    unsafe {
        ffi::ctprfb_(&(side as c_char), &(trans as c_char), &(direct as c_char),
                     &(storev as c_char), &(m as c_int), &(n as c_int), &(k as c_int), l.as_ptr(),
                     v.as_ptr() as *const _, &(ldv as c_int), t.as_ptr() as *const _,
                     &(ldt as c_int), a.as_mut_ptr() as *mut _, &(lda as c_int),
                     b.as_mut_ptr() as *mut _, &(ldb as c_int), work.as_ptr(), &(ldwork as c_int))
    }
}

#[inline]
pub fn ztprfb(side: u8, trans: u8, direct: u8, storev: u8, m: usize, n: usize, k: usize, l: &[i32],
              v: &[c64], ldv: usize, t: &[c64], ldt: usize, a: &mut [c64], lda: usize,
              b: &mut [c64], ldb: usize, work: &[f64], ldwork: usize) {

    unsafe {
        ffi::ztprfb_(&(side as c_char), &(trans as c_char), &(direct as c_char),
                     &(storev as c_char), &(m as c_int), &(n as c_int), &(k as c_int), l.as_ptr(),
                     v.as_ptr() as *const _, &(ldv as c_int), t.as_ptr() as *const _,
                     &(ldt as c_int), a.as_mut_ptr() as *mut _, &(lda as c_int),
                     b.as_mut_ptr() as *mut _, &(ldb as c_int), work.as_ptr(), &(ldwork as c_int))
    }
}

#[inline]
pub fn ssysv_rook(uplo: u8, n: usize, nrhs: usize, a: &mut [f32], lda: usize, ipiv: &mut [i32],
                  b: &mut [f32], ldb: usize, work: &mut [f32], lwork: usize, info: &mut i32) {

    unsafe {
        ffi::ssysv_rook_(&(uplo as c_char), &(n as c_int), &(nrhs as c_int), a.as_mut_ptr(),
                         &(lda as c_int), ipiv.as_mut_ptr(), b.as_mut_ptr(), &(ldb as c_int),
                         work.as_mut_ptr(), &(lwork as c_int), info)
    }
}

#[inline]
pub fn dsysv_rook(uplo: u8, n: usize, nrhs: usize, a: &mut [f64], lda: usize, ipiv: &mut [i32],
                  b: &mut [f64], ldb: usize, work: &mut [f64], lwork: usize, info: &mut i32) {

    unsafe {
        ffi::dsysv_rook_(&(uplo as c_char), &(n as c_int), &(nrhs as c_int), a.as_mut_ptr(),
                         &(lda as c_int), ipiv.as_mut_ptr(), b.as_mut_ptr(), &(ldb as c_int),
                         work.as_mut_ptr(), &(lwork as c_int), info)
    }
}

#[inline]
pub fn csysv_rook(uplo: u8, n: usize, nrhs: usize, a: &mut [c32], lda: usize, ipiv: &mut [i32],
                  b: &mut [c32], ldb: usize, work: &mut [c32], lwork: usize, info: &mut i32) {

    unsafe {
        ffi::csysv_rook_(&(uplo as c_char), &(n as c_int), &(nrhs as c_int),
                         a.as_mut_ptr() as *mut _, &(lda as c_int), ipiv.as_mut_ptr(),
                         b.as_mut_ptr() as *mut _, &(ldb as c_int), work.as_mut_ptr() as *mut _,
                         &(lwork as c_int), info)
    }
}

#[inline]
pub fn zsysv_rook(uplo: u8, n: usize, nrhs: usize, a: &mut [c64], lda: usize, ipiv: &mut [i32],
                  b: &mut [c64], ldb: usize, work: &mut [c64], lwork: usize, info: &mut i32) {

    unsafe {
        ffi::zsysv_rook_(&(uplo as c_char), &(n as c_int), &(nrhs as c_int),
                         a.as_mut_ptr() as *mut _, &(lda as c_int), ipiv.as_mut_ptr(),
                         b.as_mut_ptr() as *mut _, &(ldb as c_int), work.as_mut_ptr() as *mut _,
                         &(lwork as c_int), info)
    }
}

#[inline]
pub fn csyr(uplo: u8, n: usize, alpha: &[c32], x: &[c32], incx: usize, a: &mut [c32], lda: usize) {
    unsafe {
        ffi::csyr_(&(uplo as c_char), &(n as c_int), alpha.as_ptr() as *const _,
                   x.as_ptr() as *const _, &(incx as c_int), a.as_mut_ptr() as *mut _,
                   &(lda as c_int))
    }
}

#[inline]
pub fn zsyr(uplo: u8, n: usize, alpha: &[c64], x: &[c64], incx: usize, a: &mut [c64], lda: usize) {
    unsafe {
        ffi::zsyr_(&(uplo as c_char), &(n as c_int), alpha.as_ptr() as *const _,
                   x.as_ptr() as *const _, &(incx as c_int), a.as_mut_ptr() as *mut _,
                   &(lda as c_int))
    }
}

#[inline]
pub fn ilaver(vers_major: &mut u32, vers_minor: &mut u32, vers_patch: &mut u32) {
    unsafe {
        ffi::ilaver_(vers_major as *mut _ as *mut _, vers_minor as *mut _ as *mut _,
                     vers_patch as *mut _ as *mut _)
    }
}
