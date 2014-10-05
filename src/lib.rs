#![crate_name = "lapack"]
#![crate_type = "rlib"]

#![feature(macro_rules)]

extern crate alloc;
extern crate core;
extern crate libc;

use libc::{c_char, c_double, c_int};

#[link(name = "gfortran")]
#[link(name = "lapack", kind = "static")]
extern {
    fn dgemv_(trans: *const c_char, m: *const c_int, n: *const c_int,
        alpha: *const c_double, a: *const c_double, lda: *const c_int,
        x: *const c_double, incx: *const c_int, beta: *const c_double,
        y: *mut c_double, incy: *const c_int);

    fn dgemm_(transa: *const c_char, transb: *const c_char, m: *const c_int,
        n: *const c_int, k: *const c_int, alpha: *const c_double,
        a: *const c_double, lda: *const c_int, b: *const c_double,
        ldb: *const c_int, beta: *const c_double, c: *mut c_double,
        ldc: *const c_int);

    fn dsyev_(jobz: *const c_char, uplo: *const c_char, n: *const c_int,
        a: *const c_double, lda: *const c_int, w: *mut c_double,
        work: *mut c_double, lwork: *const c_int, info: *mut c_int);
}

pub fn dgemv(trans: i8, m: i32, n: i32, alpha: f64, a: &[f64], lda: i32,
    x: &[f64], incx: i32, beta: f64, y: &mut[f64], incy: i32) {

    unsafe {
        dgemv_(&trans, &m, &n, &alpha, a.as_ptr(), &lda, x.as_ptr(), &incx,
            &beta, y.as_mut_ptr(), &incy);
    }
}

pub fn dgemm(transa: i8, transb: i8, m: i32, n: i32, k: i32, alpha: f64,
    a: &[f64], lda: i32, b: &[f64], ldb: i32, beta: f64, c: &mut[f64],
    ldc: i32) {

    unsafe {
        dgemm_(&transa, &transb, &m, &n, &k, &alpha, a.as_ptr(), &lda,
            b.as_ptr(), &ldb, &beta, c.as_mut_ptr(), &ldc);
    }
}

pub fn dsyev(jobz: i8, uplo: i8, n: i32, a: &[f64], lda: i32, w: &mut[f64],
    work: &mut[f64], lwork: i32, info: &mut i32) {

    unsafe {
        dsyev_(&jobz, &uplo, &n, a.as_ptr(), &lda, w.as_mut_ptr(),
            work.as_mut_ptr(), &lwork, info);
    }
}

macro_rules! assert_equal(
    ($given:expr , $expected:expr) => ({
        assert_eq!($given.len(), $expected.len());
        for i in range(0u, $given.len()) {
            assert_eq!($given[i], $expected[i]);
        }
    });
)

macro_rules! assert_almost_equal(
    ($given:expr , $expected:expr) => ({
        assert_eq!($given.len(), $expected.len());
        for i in range(0u, $given.len()) {
            assert!(std::num::abs($given[i] - $expected[i]) < 1e-8);
        }
    });
)

#[test]
fn dgemv_test() {
    let m: i32 = 2;
    let n: i32 = 3;

    let a = [1.0, 4.0, 2.0, 5.0, 3.0, 6.0];
    let x = [1.0, 2.0, 3.0];
    let mut y = [6.0, 8.0];

    dgemv('N' as i8, m, n, 1.0, a, m, x, 1, 1.0, y, 1);

    let expected_y = [20.0, 40.0];

    assert_equal!(y, expected_y);
}

#[test]
fn dgemm_test() {
    let m: i32 = 2;
    let n: i32 = 4;
    let k: i32 = 3;

    let a = [1.0, 4.0, 2.0, 5.0, 3.0, 6.0];
    let b = [1.0, 5.0, 9.0, 2.0, 6.0, 10.0, 3.0, 7.0, 11.0, 4.0, 8.0, 12.0];
    let mut c = [2.0, 7.0, 6.0, 2.0, 0.0, 7.0, 4.0, 2.0];

    dgemm('N' as i8, 'N' as i8, m, n, k, 1.0, a, m, b, k, 1.0, c, m);

    let expected_c = [40.0, 90.0, 50.0, 100.0, 50.0, 120.0, 60.0, 130.0];

    assert_equal!(c, expected_c);
}

#[test]
fn dsyev_test() {
    use std::mem;
    use alloc::heap;
    use core::raw;

    #[allow(non_uppercase_statics)]
    static n: i32 = 5;

    let a = [
        0.162182308193243,
        0.601981941401637,
        0.450541598502498,
        0.825816977489547,
        0.106652770180584,
        0.601981941401637,
        0.262971284540144,
        0.083821377996933,
        0.538342435260057,
        0.961898080855054,
        0.450541598502498,
        0.083821377996933,
        0.228976968716819,
        0.996134716626885,
        0.004634224134067,
        0.825816977489547,
        0.538342435260057,
        0.996134716626885,
        0.078175528753184,
        0.774910464711502,
        0.106652770180584,
        0.961898080855054,
        0.004634224134067,
        0.774910464711502,
        0.817303220653433,
    ];

    let mut w = box [0.0, ..(n as uint)];
    let mut crap = vec![0.0];
    let mut work = crap.as_mut_slice();
    let mut lwork = -1 as i32;
    let mut info = 0 as i32;

    dsyev('V' as i8, 'U' as i8, n, a, n, &mut *w, work, lwork, &mut info);

    lwork = work[0] as i32;

    let p = unsafe {
        heap::allocate(lwork as uint, mem::min_align_of::<f64>())
    };
    work = unsafe {
        mem::transmute(raw::Slice { data: p as *const f64, len: lwork as uint })
    };

    dsyev('V' as i8, 'U' as i8, n, a, n, &mut *w, work, lwork, &mut info);

    assert_eq!(info, 0);

    let expected_a = [
        -0.350512137830478,
         0.116468084895727,
        -0.435005782872646,
         0.750503447417042,
        -0.333303121372602,
         0.462361750400701,
        -0.693041256027589,
        -0.409079614137348,
         0.219801690292016,
         0.300427221556423,
        -0.638529696902280,
        -0.450088584675982,
         0.439292414730346,
         0.201686466663592,
         0.395025107611391,
        -0.330279135877552,
         0.306798908225146,
        -0.590447291483772,
        -0.272545278020086,
         0.611458248553625,
         0.382829428829298,
         0.457628341015560,
         0.319089342511548,
         0.522946873930437,
         0.518388356797788,
    ];

    let expected_w = [
        -1.145487871954612,
        -0.676875725405419,
        -0.050275996742486,
         0.892450858666551,
         2.529798046292787,
    ];

    assert_almost_equal!(a, expected_a)
    assert_almost_equal!(w, expected_w)
}
