#![crate_name = "lapack"]
#![crate_type = "rlib"]

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
