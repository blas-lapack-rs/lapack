//! An interface to the [Linear Algebra PACKage][1].
//!
//! [1]: http://en.wikipedia.org/wiki/LAPACK

#![cfg_attr(test, feature(std_misc))]

#[cfg(test)]
#[macro_use]
extern crate assert;

extern crate "liblapack-sys" as raw;

pub enum Layout {
    RowMajor = raw::LAPACK_ROW_MAJOR as isize,
    ColumnMajor = raw::LAPACK_COL_MAJOR as isize,
}

pub enum Triangular {
    Upper = b'U' as isize,
    Lower = b'L' as isize,
}

pub enum Job {
    None = b'N' as isize,
    Vectors = b'V' as isize,
}

#[inline]
pub fn dsyev(layout: Layout, jobz: Job, uplo: Triangular, n: usize, a: &mut [f64],
             lda: usize, w: &mut [f64], work: &mut [f64], lwork: usize) -> i32 {

    unsafe {
        raw::LAPACKE_dsyev_work(layout as i32, jobz as i8, uplo as i8, n as i32,
                                a.as_mut_ptr(), lda as i32, w.as_mut_ptr(),
                                work.as_mut_ptr(), lwork as i32)
    }
}

#[cfg(test)]
mod tests {
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

        ::dsyev(::Layout::ColumnMajor, ::Job::Vectors, ::Triangular::Upper,
                n, &mut a, n, &mut w, &mut work, lwork);

        lwork = work[0] as usize;
        work = repeat(0.0).take(lwork).collect::<Vec<_>>();

        ::dsyev(::Layout::ColumnMajor, ::Job::Vectors, ::Triangular::Upper,
                n, &mut a, n, &mut w, &mut work, lwork);

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

        assert_close!(a, expected_a);
        assert_close!(w, expected_w);
    }
}
