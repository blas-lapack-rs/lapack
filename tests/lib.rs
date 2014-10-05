#![feature(macro_rules)]
#![feature(unsafe_destructor)]

extern crate lapack;

extern crate alloc;
extern crate core;

use alloc::heap;
use core::raw;
use std::mem;

#[unsafe_no_drop_flag]
pub struct Vec<T> {
    len: uint,
    ptr: *mut T,
}

impl<T> Vec<T> {
    #[inline]
    pub fn new(len: uint) -> Vec<T> {
        let ptr = unsafe {
            heap::allocate(len * mem::size_of::<T>(), mem::min_align_of::<T>())
        };
        Vec { len: len, ptr: ptr as *mut T }
    }

    #[inline]
    pub fn as_ptr(&self) -> *const T {
        self.ptr as *const T
    }

    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.ptr
    }

    #[inline]
    pub fn as_mut_slice<'a>(&'a mut self) -> &'a mut [T] {
        unsafe {
            mem::transmute(raw::Slice {
                data: self.as_mut_ptr() as *const T,
                len: self.len,
            })
        }
    }
}

#[unsafe_destructor]
impl<T> Drop for Vec<T> {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            heap::deallocate(self.ptr as *mut u8,
                self.len * mem::size_of::<T>(), mem::min_align_of::<T>())
        }
    }
}

impl<T> Index<uint, T> for Vec<T> {
    #[inline]
    fn index<'a>(&'a self, index: &uint) -> &'a T {
        &self.as_slice()[*index]
    }
}

impl<T> Slice<T> for Vec<T> {
    #[inline]
    fn as_slice<'a>(&'a self) -> &'a [T] {
        unsafe {
            mem::transmute(raw::Slice {
                data: self.as_ptr(),
                len: self.len,
            })
        }
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
fn dgemv() {
    let m: i32 = 2;
    let n: i32 = 3;

    let a = [1.0, 4.0, 2.0, 5.0, 3.0, 6.0];
    let x = [1.0, 2.0, 3.0];
    let mut y = [6.0, 8.0];

    lapack::dgemv('N' as i8, m, n, 1.0, a, m, x, 1, 1.0, y, 1);

    let expected_y = [20.0, 40.0];

    assert_equal!(y, expected_y);
}

#[test]
fn dgemm() {
    let m: i32 = 2;
    let n: i32 = 4;
    let k: i32 = 3;

    let a = [1.0, 4.0, 2.0, 5.0, 3.0, 6.0];
    let b = [1.0, 5.0, 9.0, 2.0, 6.0, 10.0, 3.0, 7.0, 11.0, 4.0, 8.0, 12.0];
    let mut c = [2.0, 7.0, 6.0, 2.0, 0.0, 7.0, 4.0, 2.0];

    lapack::dgemm('N' as i8, 'N' as i8, m, n, k, 1.0, a, m, b, k, 1.0, c, m);

    let expected_c = [40.0, 90.0, 50.0, 100.0, 50.0, 120.0, 60.0, 130.0];

    assert_equal!(c, expected_c);
}

#[test]
fn dsyev() {
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
    let mut work = Vec::new(1);
    let mut lwork = -1 as i32;
    let mut info = 0 as i32;

    lapack::dsyev('V' as i8, 'U' as i8, n, a, n, &mut *w, work.as_mut_slice(), lwork, &mut info);

    lwork = work[0] as i32;
    work = Vec::new(lwork as uint);

    lapack::dsyev('V' as i8, 'U' as i8, n, a, n, &mut *w, work.as_mut_slice(), lwork, &mut info);

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
