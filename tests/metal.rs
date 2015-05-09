extern crate assert;

extern crate lapack;

use lapack::metal;

#[test]
fn dgesvd() {
    use std::iter::repeat;

    // Example from
    // http://en.wikipedia.org/wiki/Singular_value_decomposition#Example
    let m = 4;
    let n = 5;
    let mut a = vec![ // column major order
        1.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 4.0,
        0.0, 3.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0,
        2.0, 0.0, 0.0, 0.0,
    ];

    let lda = m;
    let ldu = m;
    let ldvt = n;


    let mut s = repeat(0.0).take(n).collect::<Vec<_>>();
    let mut u = repeat(0.0).take(ldu*m).collect::<Vec<_>>();
    let mut vt = repeat(0.0).take(ldvt*n).collect::<Vec<_>>();
    let mut work = vec![0.0];
    let mut lwork = -1;
    let mut info = 0;

    metal::dgesvd(metal::Jobu::A, metal::Jobvt::A, m, n, &mut a, lda, &mut s, &mut u, ldu, &mut vt,
                  ldvt, &mut work, lwork, &mut info);

    lwork = work[0] as usize;
    work = repeat(0.0).take(lwork).collect::<Vec<_>>();

    metal::dgesvd(metal::Jobu::A, metal::Jobvt::A, m, n, &mut a, lda, &mut s, &mut u, ldu, &mut vt,
                  ldvt, &mut work, lwork, &mut info);

    let expected_u = vec![ // column major order
        0.0, 0.0, 0.0, 1.0,
        0.0, 1.0, 0.0, 0.0,
        1.0, 0.0, 0.0, 0.0,
        0.0, 0.0, -1.0, 0.0,
    ];
    let expected_s = vec![
        4.0, 3.0, 5.0_f64.sqrt(), 0.0, 0.0,
    ];

    let expected_vt = vec![ // column major order
        0.0, 0.0, 0.2_f64.sqrt(), 0.0, -(0.8_f64).sqrt(),
        1.0, 0.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.8_f64.sqrt(), 0.0, 0.2_f64.sqrt(),
    ];

    assert::within(&u, &expected_u, 1e-14);
    assert::within(&s, &expected_s, 1e-14);
    assert::within(&vt, &expected_vt, 1e-14);
}

#[test]
fn dgetrf_and_dgetri() {

    // Compute the inverse of a square matrix.

    use std::iter::repeat;

    let m = 2;
    let n = 2;
    let mut a = vec![ // column major order
        1.0, 3.0,
        2.0, 4.0,
    ];
    let lda = m;

    let mut ipiv = vec![0];
    let mut info = 0;

    metal::dgetrf(m, n, &mut a, lda, &mut ipiv, &mut info);

    let lwork = n*n;
    let mut work = repeat(0.0).take(lwork).collect::<Vec<_>>();

    metal::dgetri(n, &mut a, lda, &mut ipiv, &mut work, lwork, &mut info);

    let expected_a = vec![ // column major order
        -2.0, 1.5,
        1.0, -0.5,
    ];
    assert::within(&a, &expected_a, 1e-14);
}

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

    metal::dsyev(metal::Jobz::V, metal::Uplo::U, n, &mut a, n, &mut w, &mut work, lwork,
                 &mut info);

    lwork = work[0] as usize;
    work = repeat(0.0).take(lwork).collect::<Vec<_>>();

    metal::dsyev(metal::Jobz::V, metal::Uplo::U, n, &mut a, n, &mut w, &mut work, lwork,
                 &mut info);

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
