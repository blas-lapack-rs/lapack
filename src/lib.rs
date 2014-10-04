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
}

pub fn dgemv(trans: i8, m: i32, n: i32, alpha: f64, a: &[f64], lda: i32,
    x: &[f64], incx: i32, beta: f64, y: &mut[f64], incy: i32) {

    unsafe {
        dgemv_(&trans, &m, &n, &alpha, a.as_ptr(), &lda, x.as_ptr(), &incx, &beta, y.as_mut_ptr(), &incy);
    }
}

#[test]
fn test_dgemv() {
    let m: i32 = 2;
    let n: i32 = 3;

    let a = [1.0, 4.0, 2.0, 5.0, 3.0, 6.0];
    let x = [1.0, 2.0, 3.0];
    let mut y = [6.0, 8.0];
    let z = [20.0, 40.0];

    dgemv('N' as i8, m, n, 1.0, a, m, x, 1, 1.0, y, 1);

    for i in range(0u, y.len()) {
        assert_eq!(y[i], z[i]);
    }
}
