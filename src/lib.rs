//! An interface to the [Linear Algebra PACKage][1].
//!
//! [1]: http://en.wikipedia.org/wiki/LAPACK

#[cfg(test)]
extern crate assert;

extern crate libc;
extern crate liblapack_sys as raw;

use libc::{c_char, c_int};

pub enum Jobz {
    /// Compute eigenvalues only.
    N = b'N' as isize,
    /// Compute eigenvalues and eigenvectors.
    V = b'V' as isize,
}

pub enum Uplo {
    /// Upper triangles are stored.
    U = b'U' as isize,
    /// Lower triangles are stored.
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

#[cfg(test)]
mod tests {
    use assert;

    #[test]
    fn dsyev() {
        use std::iter::repeat;

        let n = 5;

        let mut a = vec![
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

        let mut w = repeat(0.0).take(n).collect::<Vec<_>>();
        let mut work = vec![0.0];
        let mut lwork = -1;
        let mut info = 0;

        ::dsyev(::Jobz::V, ::Uplo::U, n, &mut a, n, &mut w, &mut work, lwork, &mut info);

        lwork = work[0] as usize;
        work = repeat(0.0).take(lwork).collect::<Vec<_>>();

        ::dsyev(::Jobz::V, ::Uplo::U, n, &mut a, n, &mut w, &mut work, lwork, &mut info);

        let expected_a = vec![
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
        let expected_w = vec![
            -1.145487871954612, -0.676875725405419, -0.050275996742486,
             0.892450858666551,  2.529798046292787,
        ];

        assert::within(&a, &expected_a, 1e-14);
        assert::within(&w, &expected_w, 1e-14);
    }
}
