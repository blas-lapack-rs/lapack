extern crate libc;

use libc::{c_char, c_double, c_int};

#[link(name = "gfortran")]
extern {
    pub fn dsyev_(jobz: *const c_char, uplo: *const c_char, n: *const c_int,
                  a: *mut c_double, lda: *const c_int, w: *mut c_double,
                  work: *mut c_double, lwork: *const c_int, info: *mut c_int);
}
