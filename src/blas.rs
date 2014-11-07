/// Perform a matrix-matrix multiplication followed by an addition.
///
/// The function performs one of the matrix-matrix operations
///
/// ```math
/// C := alpha * op(A) * op(B) + beta * C
/// ```
///
/// where `op(X)` is one of
///
/// ```math
/// op(X) = X or
/// op(X) = X^T,
/// ```
///
/// `alpha` and `beta` are scalars, and `A`, `B`, and `C` are matrices, with
/// `op(A)` an `m`-by-`k` matrix, `op(B)` a `k`-by-`n` matrix, and `C` an
/// `m`-by-`n` matrix.
///
/// http://www.netlib.org/lapack/explore-html/d7/d2b/dgemm_8f.html
#[inline]
pub fn dgemm(transa: u8, transb: u8, m: uint, n: uint, k: uint, alpha: f64, a: &[f64],
             lda: uint, b: &[f64], ldb: uint, beta: f64, c: &mut [f64], ldc: uint) {

    unsafe {
        ::raw::dgemm(&(transa as i8), &(transb as i8), &(m as i32), &(n as i32),
                     &(k as i32), &alpha, a.as_ptr(), &(lda as i32), b.as_ptr(),
                     &(ldb as i32), &beta, c.as_mut_ptr(), &(ldc as i32));
    }
}

/// Perform a matrix-vector multiplication followed by an addition.
///
/// The function performs one of the matrix-vector operations
///
/// ```math
/// y := alpha * A * x + beta * y or
/// y := alpha * A^T * x + beta * y
/// ```
///
/// where `alpha` and `beta` are scalars, `x` and `y` are vectors, and `A` is
/// an `m`-by-`n` matrix.
///
/// http://www.netlib.org/lapack/explore-html/dc/da8/dgemv_8f.html
#[inline]
pub fn dgemv(trans: u8, m: uint, n: uint, alpha: f64, a: &[f64], lda: uint,
             x: &[f64], incx: uint, beta: f64, y: &mut [f64], incy: uint) {

    unsafe {
        ::raw::dgemv(&(trans as i8), &(m as i32), &(n as i32), &alpha, a.as_ptr(),
                     &(lda as i32), x.as_ptr(), &(incx as i32), &beta,
                     y.as_mut_ptr(), &(incy as i32));
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn dgemm() {
        let (m, n, k) = (2, 4, 3);

        let a = vec![1.0, 4.0, 2.0, 5.0, 3.0, 6.0];
        let b = vec![1.0, 5.0, 9.0, 2.0, 6.0, 10.0, 3.0, 7.0, 11.0, 4.0, 8.0, 12.0];
        let mut c = vec![2.0, 7.0, 6.0, 2.0, 0.0, 7.0, 4.0, 2.0];

        ::dgemm(b'N', b'N', m, n, k, 1.0, a.as_slice(), m, b.as_slice(), k, 1.0,
                c.as_mut_slice(), m);

        let expected_c = vec![40.0, 90.0, 50.0, 100.0, 50.0, 120.0, 60.0, 130.0];
        assert_equal!(c, expected_c);
    }

    #[test]
    fn dgemv() {
        let (m, n) = (2, 3);

        let a = vec![1.0, 4.0, 2.0, 5.0, 3.0, 6.0];
        let x = vec![1.0, 2.0, 3.0];
        let mut y = vec![6.0, 8.0];

        ::dgemv(b'N', m, n, 1.0, a.as_slice(), m, x.as_slice(), 1, 1.0,
                y.as_mut_slice(), 1);

        let expected_y = vec![20.0, 40.0];
        assert_equal!(y, expected_y);
    }
}

#[cfg(test)]
mod bench {
    extern crate test;

    #[bench]
    fn dgemv_few_large(b: &mut test::Bencher) {
        let m = 1000;

        let a = Vec::from_elem(m * m, 1.0);
        let x = Vec::from_elem(m * 1, 1.0);
        let mut y = Vec::from_elem(m * 1, 1.0);

        b.iter(|| {
            ::dgemv(b'N', m, m, 1.0, a.as_slice(), m, x.as_slice(), 1, 1.0,
                    y.as_mut_slice(), 1)
        });
    }

    #[bench]
    fn dgemv_many_small(b: &mut test::Bencher) {
        let m = 20;

        let a = Vec::from_elem(m * m, 1.0);
        let x = Vec::from_elem(m * 1, 1.0);
        let mut y = Vec::from_elem(m * 1, 1.0);

        b.iter(|| {
            for _ in range(0u, 20000) {
                ::dgemv(b'N', m, m, 1.0, a.as_slice(), m, x.as_slice(), 1, 1.0,
                        y.as_mut_slice(), 1);
            }
        });
    }
}
