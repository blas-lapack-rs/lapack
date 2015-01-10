//! An interface to the [Linear Algebra PACKage][1].
//!
//! [1]: http://www.netlib.org/lapack/

#![allow(non_snake_case, unstable)]

#[cfg(test)]
#[macro_use]
extern crate assert;

extern crate "liblapack-sys" as raw;

/// http://www.netlib.org/lapack/explore-html/dd/d4c/dsyev_8f.html
#[inline]
pub fn dsyev(JOBZ: u8, UPLO: u8, N: usize, A: &mut [f64], LDA: usize, W: &mut [f64],
             WORK: &mut [f64], LWORK: usize, INFO: *mut isize) {

    unsafe {
        raw::dsyev(&(JOBZ as i8), &(UPLO as i8), &(N as i32), A.as_mut_ptr(),
                   &(LDA as i32), W.as_mut_ptr(), WORK.as_mut_ptr(),
                   &(LWORK as i32), INFO as *mut i32);
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn dsyev() {
        use std::iter::repeat;

        let N = 5;

        let mut A = vec![
            0.162182308193243, 0.601981941401637, 0.450541598502498,
            0.825816977489547, 0.106652770180584, 0.601981941401637,
            0.262971284540144, 0.083821377996933, 0.538342435260057,
            0.961898080855054, 0.450541598502498, 0.083821377996933,
            0.228976968716819, 0.996134716626885, 0.004634224134067,
            0.825816977489547, 0.538342435260057, 0.996134716626885,
            0.078175528753184, 0.774910464711502, 0.106652770180584,
            0.961898080855054, 0.004634224134067, 0.774910464711502,
            0.817303220653433,
        ];

        let mut W = repeat(0.0).take(N).collect::<Vec<_>>();
        let mut WORK = vec![0.0];
        let mut LWORK = -1;
        let mut INFO = 0;

        ::dsyev(b'V', b'U', N, A.as_mut_slice(), N, W.as_mut_slice(),
                WORK.as_mut_slice(), LWORK, &mut INFO);

        LWORK = WORK[0] as usize;
        WORK = repeat(0.0).take(LWORK).collect::<Vec<_>>();

        ::dsyev(b'V', b'U', N, A.as_mut_slice(), N, W.as_mut_slice(),
                WORK.as_mut_slice(), LWORK, &mut INFO);

        assert_eq!(INFO, 0);

        let expected_A = vec![
            -0.350512137830478,  0.116468084895727, -0.435005782872646,
             0.750503447417042, -0.333303121372602,  0.462361750400701,
            -0.693041256027589, -0.409079614137348,  0.219801690292016,
             0.300427221556423, -0.638529696902280, -0.450088584675982,
             0.439292414730346,  0.201686466663592,  0.395025107611391,
            -0.330279135877552,  0.306798908225146, -0.590447291483772,
            -0.272545278020086,  0.611458248553625,  0.382829428829298,
             0.457628341015560,  0.319089342511548,  0.522946873930437,
             0.518388356797788,
        ];
        let expected_W = vec![
            -1.145487871954612, -0.676875725405419, -0.050275996742486,
             0.892450858666551,  2.529798046292787,
        ];

        assert_close!(A, expected_A);
        assert_close!(W, expected_W);
    }
}
