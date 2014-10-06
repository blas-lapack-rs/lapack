#![feature(macro_rules)]

extern crate libc;

use libc::{c_char, c_double, c_int};

#[link(name = "gfortran")]
#[link(name = "lapack", kind = "static")]
extern {
    fn dsyev_(jobz: *const c_char, uplo: *const c_char, n: *const c_int,
        a: *const c_double, lda: *const c_int, w: *mut c_double,
        work: *mut c_double, lwork: *const c_int, info: *mut c_int);
}

pub static EIG_VALS: i8 = 'N' as i8;
pub static EIG_VALS_VECS: i8 = 'V' as i8;

pub static UPPER: i8 = 'U' as i8;
pub static LOWER: i8 = 'L' as i8;

#[inline]
pub fn dsyev(jobz: i8, uplo: i8, n: i32, a: *const f64, lda: i32, w: *mut f64,
    work: *mut f64, lwork: i32, info: *mut i32) {

    unsafe {
        dsyev_(&jobz, &uplo, &n, a, &lda, w, work, &lwork, info);
    }
}

#[cfg(test)]
mod tests {
    macro_rules! assert_almost_equal(
        ($given:expr, $expected:expr) => ({
            assert_eq!($given.len(), $expected.len());
            for i in range(0u, $given.len()) {
                assert!(::std::num::abs($given[i] - $expected[i]) < 1e-8);
            }
        });
    )

    #[test]
    fn dsyev() {
        let n = 5;

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

        let mut w = Vec::from_elem(n as uint, 0.0);
        let mut work = vec![0.0];
        let mut lwork = -1;
        let mut info = 0;

        super::dsyev(super::EIG_VALS_VECS, super::UPPER, n, a.as_ptr(), n,
            w.as_mut_ptr(), work.as_mut_ptr(), lwork, &mut info);

        lwork = work[0] as i32;
        work = Vec::from_elem(lwork as uint, 0.0);

        super::dsyev(super::EIG_VALS_VECS, super::UPPER, n, a.as_ptr(), n,
            w.as_mut_ptr(), work.as_mut_ptr(), lwork, &mut info);

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
}
