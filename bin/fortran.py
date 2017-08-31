#!/usr/bin/env python

from common import Function
import re

functions = """
    pub fn sgetrf_(m: *const lapack_int, n: *const lapack_int, a: *mut c_float,
                   lda: *const lapack_int, ipiv: *mut lapack_int, info: *mut lapack_int);
    pub fn dgetrf_(m: *const lapack_int, n: *const lapack_int, a: *mut c_double,
                   lda: *const lapack_int, ipiv: *mut lapack_int, info: *mut lapack_int);
    pub fn cgetrf_(m: *const lapack_int, n: *const lapack_int, a: *mut lapack_complex_float,
                   lda: *const lapack_int, ipiv: *mut lapack_int, info: *mut lapack_int);
    pub fn zgetrf_(m: *const lapack_int, n: *const lapack_int, a: *mut lapack_complex_double,
                   lda: *const lapack_int, ipiv: *mut lapack_int, info: *mut lapack_int);

    pub fn sgetrf2_(m: *const lapack_int, n: *const lapack_int, a: *mut c_float,
                    lda: *const lapack_int, ipiv: *mut lapack_int, info: *mut lapack_int);
    pub fn dgetrf2_(m: *const lapack_int, n: *const lapack_int, a: *mut c_double,
                    lda: *const lapack_int, ipiv: *mut lapack_int, info: *mut lapack_int);
    pub fn cgetrf2_(m: *const lapack_int, n: *const lapack_int, a: *mut lapack_complex_float,
                    lda: *const lapack_int, ipiv: *mut lapack_int, info: *mut lapack_int);
    pub fn zgetrf2_(m: *const lapack_int, n: *const lapack_int, a: *mut lapack_complex_double,
                    lda: *const lapack_int, ipiv: *mut lapack_int, info: *mut lapack_int);

    pub fn sgbtrf_(m: *const lapack_int, n: *const lapack_int, kl: *const lapack_int,
                   ku: *const lapack_int, ab: *mut c_float, ldab: *const lapack_int,
                   ipiv: *mut lapack_int, info: *mut lapack_int);
    pub fn dgbtrf_(m: *const lapack_int, n: *const lapack_int, kl: *const lapack_int,
                   ku: *const lapack_int, ab: *mut c_double, ldab: *const lapack_int,
                   ipiv: *mut lapack_int, info: *mut lapack_int);
    pub fn cgbtrf_(m: *const lapack_int, n: *const lapack_int, kl: *const lapack_int,
                   ku: *const lapack_int, ab: *mut lapack_complex_float,
                   ldab: *const lapack_int, ipiv: *mut lapack_int, info: *mut lapack_int);
    pub fn zgbtrf_(m: *const lapack_int, n: *const lapack_int, kl: *const lapack_int,
                   ku: *const lapack_int, ab: *mut lapack_complex_double,
                   ldab: *const lapack_int, ipiv: *mut lapack_int, info: *mut lapack_int);

    pub fn sgttrf_(n: *const lapack_int, dl: *mut c_float, d: *mut c_float, du: *mut c_float,
                   du2: *mut c_float, ipiv: *mut lapack_int, info: *mut lapack_int);
    pub fn dgttrf_(n: *const lapack_int, dl: *mut c_double, d: *mut c_double, du: *mut c_double,
                   du2: *mut c_double, ipiv: *mut lapack_int, info: *mut lapack_int);
    pub fn cgttrf_(n: *const lapack_int, dl: *mut lapack_complex_float,
                   d: *mut lapack_complex_float, du: *mut lapack_complex_float,
                   du2: *mut lapack_complex_float, ipiv: *mut lapack_int, info: *mut lapack_int);
    pub fn zgttrf_(n: *const lapack_int, dl: *mut lapack_complex_double,
                   d: *mut lapack_complex_double, du: *mut lapack_complex_double,
                   du2: *mut lapack_complex_double, ipiv: *mut lapack_int, info: *mut lapack_int);

    pub fn spotrf2_(uplo: *const c_char, n: *const lapack_int, a: *mut c_float,
                    lda: *const lapack_int, info: *mut lapack_int);
    pub fn dpotrf2_(uplo: *const c_char, n: *const lapack_int, a: *mut c_double,
                    lda: *const lapack_int, info: *mut lapack_int);
    pub fn cpotrf2_(uplo: *const c_char, n: *const lapack_int, a: *mut lapack_complex_float,
                    lda: *const lapack_int, info: *mut lapack_int);
    pub fn zpotrf2_(uplo: *const c_char, n: *const lapack_int, a: *mut lapack_complex_double,
                    lda: *const lapack_int, info: *mut lapack_int);

    pub fn spotrf_(uplo: *const c_char, n: *const lapack_int, a: *mut c_float,
                   lda: *const lapack_int, info: *mut lapack_int);
    pub fn dpotrf_(uplo: *const c_char, n: *const lapack_int, a: *mut c_double,
                   lda: *const lapack_int, info: *mut lapack_int);
    pub fn cpotrf_(uplo: *const c_char, n: *const lapack_int, a: *mut lapack_complex_float,
                   lda: *const lapack_int, info: *mut lapack_int);
    pub fn zpotrf_(uplo: *const c_char, n: *const lapack_int, a: *mut lapack_complex_double,
                   lda: *const lapack_int, info: *mut lapack_int);

    pub fn dpstrf_(uplo: *const c_char, n: *const lapack_int, a: *mut c_double,
                   lda: *const lapack_int, piv: *mut lapack_int, rank: *mut lapack_int,
                   tol: *const c_double, work: *mut c_double, info: *mut lapack_int);
    pub fn spstrf_(uplo: *const c_char, n: *const lapack_int, a: *mut c_float,
                   lda: *const lapack_int, piv: *mut lapack_int, rank: *mut lapack_int,
                   tol: *const c_float, work: *mut c_float, info: *mut lapack_int);
    pub fn zpstrf_(uplo: *const c_char, n: *const lapack_int, a: *mut lapack_complex_double,
                   lda: *const lapack_int, piv: *mut lapack_int, rank: *mut lapack_int,
                   tol: *const c_double, work: *mut c_double, info: *mut lapack_int);
    pub fn cpstrf_(uplo: *const c_char, n: *const lapack_int, a: *mut lapack_complex_float,
                   lda: *const lapack_int, piv: *mut lapack_int, rank: *mut lapack_int,
                   tol: *const c_float, work: *mut c_float, info: *mut lapack_int);

    pub fn dpftrf_(transr: *const c_char, uplo: *const c_char, n: *const lapack_int,
                   a: *mut c_double, info: *mut lapack_int);
    pub fn spftrf_(transr: *const c_char, uplo: *const c_char, n: *const lapack_int,
                   a: *mut c_float, info: *mut lapack_int);
    pub fn zpftrf_(transr: *const c_char, uplo: *const c_char, n: *const lapack_int,
                   a: *mut lapack_complex_double, info: *mut lapack_int);
    pub fn cpftrf_(transr: *const c_char, uplo: *const c_char, n: *const lapack_int,
                   a: *mut lapack_complex_float, info: *mut lapack_int);

    pub fn spptrf_(uplo: *const c_char, n: *const lapack_int, ap: *mut c_float,
                   info: *mut lapack_int);
    pub fn dpptrf_(uplo: *const c_char, n: *const lapack_int, ap: *mut c_double,
                   info: *mut lapack_int);
    pub fn cpptrf_(uplo: *const c_char, n: *const lapack_int, ap: *mut lapack_complex_float,
                   info: *mut lapack_int);
    pub fn zpptrf_(uplo: *const c_char, n: *const lapack_int, ap: *mut lapack_complex_double,
                   info: *mut lapack_int);

    pub fn spbtrf_(uplo: *const c_char, n: *const lapack_int, kd: *const lapack_int,
                   ab: *mut c_float, ldab: *const lapack_int, info: *mut lapack_int);
    pub fn dpbtrf_(uplo: *const c_char, n: *const lapack_int, kd: *const lapack_int,
                   ab: *mut c_double, ldab: *const lapack_int, info: *mut lapack_int);
    pub fn cpbtrf_(uplo: *const c_char, n: *const lapack_int, kd: *const lapack_int,
                   ab: *mut lapack_complex_float, ldab: *const lapack_int, info: *mut lapack_int);
    pub fn zpbtrf_(uplo: *const c_char, n: *const lapack_int, kd: *const lapack_int,
                   ab: *mut lapack_complex_double, ldab: *const lapack_int,
                   info: *mut lapack_int);

    pub fn spttrf_(n: *const lapack_int, d: *mut c_float, e: *mut c_float, info: *mut lapack_int);
    pub fn dpttrf_(n: *const lapack_int, d: *mut c_double, e: *mut c_double,
                   info: *mut lapack_int);
    pub fn cpttrf_(n: *const lapack_int, d: *mut c_float, e: *mut lapack_complex_float,
                   info: *mut lapack_int);
    pub fn zpttrf_(n: *const lapack_int, d: *mut c_double, e: *mut lapack_complex_double,
                   info: *mut lapack_int);

    pub fn ssytrf_(uplo: *const c_char, n: *const lapack_int, a: *mut c_float,
                   lda: *const lapack_int, ipiv: *mut lapack_int, work: *mut c_float,
                   lwork: *const lapack_int, info: *mut lapack_int);
    pub fn dsytrf_(uplo: *const c_char, n: *const lapack_int, a: *mut c_double,
                   lda: *const lapack_int, ipiv: *mut lapack_int, work: *mut c_double,
                   lwork: *const lapack_int, info: *mut lapack_int);
    pub fn csytrf_(uplo: *const c_char, n: *const lapack_int, a: *mut lapack_complex_float,
                   lda: *const lapack_int, ipiv: *mut lapack_int,
                   work: *mut lapack_complex_float, lwork: *const lapack_int,
                   info: *mut lapack_int);
    pub fn zsytrf_(uplo: *const c_char, n: *const lapack_int, a: *mut lapack_complex_double,
                   lda: *const lapack_int, ipiv: *mut lapack_int,
                   work: *mut lapack_complex_double, lwork: *const lapack_int,
                   info: *mut lapack_int);

    pub fn chetrf_(uplo: *const c_char, n: *const lapack_int, a: *mut lapack_complex_float,
                   lda: *const lapack_int, ipiv: *mut lapack_int,
                   work: *mut lapack_complex_float, lwork: *const lapack_int,
                   info: *mut lapack_int);
    pub fn zhetrf_(uplo: *const c_char, n: *const lapack_int, a: *mut lapack_complex_double,
                   lda: *const lapack_int, ipiv: *mut lapack_int,
                   work: *mut lapack_complex_double, lwork: *const lapack_int,
                   info: *mut lapack_int);

    pub fn ssptrf_(uplo: *const c_char, n: *const lapack_int, ap: *mut c_float,
                   ipiv: *mut lapack_int, info: *mut lapack_int);
    pub fn dsptrf_(uplo: *const c_char, n: *const lapack_int, ap: *mut c_double,
                   ipiv: *mut lapack_int, info: *mut lapack_int);
    pub fn csptrf_(uplo: *const c_char, n: *const lapack_int, ap: *mut lapack_complex_float,
                   ipiv: *mut lapack_int, info: *mut lapack_int);
    pub fn zsptrf_(uplo: *const c_char, n: *const lapack_int, ap: *mut lapack_complex_double,
                   ipiv: *mut lapack_int, info: *mut lapack_int);

    pub fn chptrf_(uplo: *const c_char, n: *const lapack_int, ap: *mut lapack_complex_float,
                   ipiv: *mut lapack_int, info: *mut lapack_int);
    pub fn zhptrf_(uplo: *const c_char, n: *const lapack_int, ap: *mut lapack_complex_double,
                   ipiv: *mut lapack_int, info: *mut lapack_int);

    pub fn sgetrs_(trans: *const c_char, n: *const lapack_int, nrhs: *const lapack_int,
                   a: *const c_float, lda: *const lapack_int, ipiv: *const lapack_int,
                   b: *mut c_float, ldb: *const lapack_int, info: *mut lapack_int);
    pub fn dgetrs_(trans: *const c_char, n: *const lapack_int, nrhs: *const lapack_int,
                   a: *const c_double, lda: *const lapack_int, ipiv: *const lapack_int,
                   b: *mut c_double, ldb: *const lapack_int, info: *mut lapack_int);
    pub fn cgetrs_(trans: *const c_char, n: *const lapack_int, nrhs: *const lapack_int,
                   a: *const lapack_complex_float, lda: *const lapack_int,
                   ipiv: *const lapack_int, b: *mut lapack_complex_float,
                   ldb: *const lapack_int, info: *mut lapack_int);
    pub fn zgetrs_(trans: *const c_char, n: *const lapack_int, nrhs: *const lapack_int,
                   a: *const lapack_complex_double, lda: *const lapack_int,
                   ipiv: *const lapack_int, b: *mut lapack_complex_double,
                   ldb: *const lapack_int, info: *mut lapack_int);

    pub fn sgbtrs_(trans: *const c_char, n: *const lapack_int, kl: *const lapack_int,
                   ku: *const lapack_int, nrhs: *const lapack_int, ab: *const c_float,
                   ldab: *const lapack_int, ipiv: *const lapack_int, b: *mut c_float,
                   ldb: *const lapack_int, info: *mut lapack_int);
    pub fn dgbtrs_(trans: *const c_char, n: *const lapack_int, kl: *const lapack_int,
                   ku: *const lapack_int, nrhs: *const lapack_int, ab: *const c_double,
                   ldab: *const lapack_int, ipiv: *const lapack_int, b: *mut c_double,
                   ldb: *const lapack_int, info: *mut lapack_int);
    pub fn cgbtrs_(trans: *const c_char, n: *const lapack_int, kl: *const lapack_int,
                   ku: *const lapack_int, nrhs: *const lapack_int,
                   ab: *const lapack_complex_float, ldab: *const lapack_int,
                   ipiv: *const lapack_int, b: *mut lapack_complex_float,
                   ldb: *const lapack_int, info: *mut lapack_int);
    pub fn zgbtrs_(trans: *const c_char, n: *const lapack_int, kl: *const lapack_int,
                   ku: *const lapack_int, nrhs: *const lapack_int,
                   ab: *const lapack_complex_double, ldab: *const lapack_int,
                   ipiv: *const lapack_int, b: *mut lapack_complex_double,
                   ldb: *const lapack_int, info: *mut lapack_int);

    pub fn sgttrs_(trans: *const c_char, n: *const lapack_int, nrhs: *const lapack_int,
                   dl: *const c_float, d: *const c_float, du: *const c_float,
                   du2: *const c_float, ipiv: *const lapack_int, b: *mut c_float,
                   ldb: *const lapack_int, info: *mut lapack_int);
    pub fn dgttrs_(trans: *const c_char, n: *const lapack_int, nrhs: *const lapack_int,
                   dl: *const c_double, d: *const c_double, du: *const c_double,
                   du2: *const c_double, ipiv: *const lapack_int, b: *mut c_double,
                   ldb: *const lapack_int, info: *mut lapack_int);
    pub fn cgttrs_(trans: *const c_char, n: *const lapack_int, nrhs: *const lapack_int,
                   dl: *const lapack_complex_float, d: *const lapack_complex_float,
                   du: *const lapack_complex_float, du2: *const lapack_complex_float,
                   ipiv: *const lapack_int, b: *mut lapack_complex_float,
                   ldb: *const lapack_int, info: *mut lapack_int);
    pub fn zgttrs_(trans: *const c_char, n: *const lapack_int, nrhs: *const lapack_int,
                   dl: *const lapack_complex_double, d: *const lapack_complex_double,
                   du: *const lapack_complex_double, du2: *const lapack_complex_double,
                   ipiv: *const lapack_int, b: *mut lapack_complex_double,
                   ldb: *const lapack_int, info: *mut lapack_int);

    pub fn spotrs_(uplo: *const c_char, n: *const lapack_int, nrhs: *const lapack_int,
                   a: *const c_float, lda: *const lapack_int, b: *mut c_float,
                   ldb: *const lapack_int, info: *mut lapack_int);
    pub fn dpotrs_(uplo: *const c_char, n: *const lapack_int, nrhs: *const lapack_int,
                   a: *const c_double, lda: *const lapack_int, b: *mut c_double,
                   ldb: *const lapack_int, info: *mut lapack_int);
    pub fn cpotrs_(uplo: *const c_char, n: *const lapack_int, nrhs: *const lapack_int,
                   a: *const lapack_complex_float, lda: *const lapack_int,
                   b: *mut lapack_complex_float, ldb: *const lapack_int, info: *mut lapack_int);
    pub fn zpotrs_(uplo: *const c_char, n: *const lapack_int, nrhs: *const lapack_int,
                   a: *const lapack_complex_double, lda: *const lapack_int,
                   b: *mut lapack_complex_double, ldb: *const lapack_int, info: *mut lapack_int);

    pub fn dpftrs_(transr: *const c_char, uplo: *const c_char, n: *const lapack_int,
                   nrhs: *const lapack_int, a: *const c_double, b: *mut c_double,
                   ldb: *const lapack_int, info: *mut lapack_int);
    pub fn spftrs_(transr: *const c_char, uplo: *const c_char, n: *const lapack_int,
                   nrhs: *const lapack_int, a: *const c_float, b: *mut c_float,
                   ldb: *const lapack_int, info: *mut lapack_int);
    pub fn zpftrs_(transr: *const c_char, uplo: *const c_char, n: *const lapack_int,
                   nrhs: *const lapack_int, a: *const lapack_complex_double,
                   b: *mut lapack_complex_double, ldb: *const lapack_int, info: *mut lapack_int);
    pub fn cpftrs_(transr: *const c_char, uplo: *const c_char, n: *const lapack_int,
                   nrhs: *const lapack_int, a: *const lapack_complex_float,
                   b: *mut lapack_complex_float, ldb: *const lapack_int, info: *mut lapack_int);

    pub fn spptrs_(uplo: *const c_char, n: *const lapack_int, nrhs: *const lapack_int,
                   ap: *const c_float, b: *mut c_float, ldb: *const lapack_int,
                   info: *mut lapack_int);
    pub fn dpptrs_(uplo: *const c_char, n: *const lapack_int, nrhs: *const lapack_int,
                   ap: *const c_double, b: *mut c_double, ldb: *const lapack_int,
                   info: *mut lapack_int);
    pub fn cpptrs_(uplo: *const c_char, n: *const lapack_int, nrhs: *const lapack_int,
                   ap: *const lapack_complex_float, b: *mut lapack_complex_float,
                   ldb: *const lapack_int, info: *mut lapack_int);
    pub fn zpptrs_(uplo: *const c_char, n: *const lapack_int, nrhs: *const lapack_int,
                   ap: *const lapack_complex_double, b: *mut lapack_complex_double,
                   ldb: *const lapack_int, info: *mut lapack_int);

    pub fn spbtrs_(uplo: *const c_char, n: *const lapack_int, kd: *const lapack_int,
                   nrhs: *const lapack_int, ab: *const c_float, ldab: *const lapack_int,
                   b: *mut c_float, ldb: *const lapack_int, info: *mut lapack_int);
    pub fn dpbtrs_(uplo: *const c_char, n: *const lapack_int, kd: *const lapack_int,
                   nrhs: *const lapack_int, ab: *const c_double, ldab: *const lapack_int,
                   b: *mut c_double, ldb: *const lapack_int, info: *mut lapack_int);
    pub fn cpbtrs_(uplo: *const c_char, n: *const lapack_int, kd: *const lapack_int,
                   nrhs: *const lapack_int, ab: *const lapack_complex_float,
                   ldab: *const lapack_int, b: *mut lapack_complex_float,
                   ldb: *const lapack_int, info: *mut lapack_int);
    pub fn zpbtrs_(uplo: *const c_char, n: *const lapack_int, kd: *const lapack_int,
                   nrhs: *const lapack_int, ab: *const lapack_complex_double,
                   ldab: *const lapack_int, b: *mut lapack_complex_double,
                   ldb: *const lapack_int, info: *mut lapack_int);

    pub fn spttrs_(n: *const lapack_int, nrhs: *const lapack_int, d: *const c_float,
                   e: *const c_float, b: *mut c_float, ldb: *const lapack_int,
                   info: *mut lapack_int);
    pub fn dpttrs_(n: *const lapack_int, nrhs: *const lapack_int, d: *const c_double,
                   e: *const c_double, b: *mut c_double, ldb: *const lapack_int,
                   info: *mut lapack_int);
    pub fn cpttrs_(uplo: *const c_char, n: *const lapack_int, nrhs: *const lapack_int,
                   d: *const c_float, e: *const lapack_complex_float,
                   b: *mut lapack_complex_float, ldb: *const lapack_int, info: *mut lapack_int);
    pub fn zpttrs_(uplo: *const c_char, n: *const lapack_int, nrhs: *const lapack_int,
                   d: *const c_double, e: *const lapack_complex_double,
                   b: *mut lapack_complex_double, ldb: *const lapack_int, info: *mut lapack_int);

    pub fn ssytrs_(uplo: *const c_char, n: *const lapack_int, nrhs: *const lapack_int,
                   a: *const c_float, lda: *const lapack_int, ipiv: *const lapack_int,
                   b: *mut c_float, ldb: *const lapack_int, info: *mut lapack_int);
    pub fn dsytrs_(uplo: *const c_char, n: *const lapack_int, nrhs: *const lapack_int,
                   a: *const c_double, lda: *const lapack_int, ipiv: *const lapack_int,
                   b: *mut c_double, ldb: *const lapack_int, info: *mut lapack_int);
    pub fn csytrs_(uplo: *const c_char, n: *const lapack_int, nrhs: *const lapack_int,
                   a: *const lapack_complex_float, lda: *const lapack_int,
                   ipiv: *const lapack_int, b: *mut lapack_complex_float,
                   ldb: *const lapack_int, info: *mut lapack_int);
    pub fn zsytrs_(uplo: *const c_char, n: *const lapack_int, nrhs: *const lapack_int,
                   a: *const lapack_complex_double, lda: *const lapack_int,
                   ipiv: *const lapack_int, b: *mut lapack_complex_double,
                   ldb: *const lapack_int, info: *mut lapack_int);

    pub fn chetrs_(uplo: *const c_char, n: *const lapack_int, nrhs: *const lapack_int,
                   a: *const lapack_complex_float, lda: *const lapack_int,
                   ipiv: *const lapack_int, b: *mut lapack_complex_float,
                   ldb: *const lapack_int, info: *mut lapack_int);
    pub fn zhetrs_(uplo: *const c_char, n: *const lapack_int, nrhs: *const lapack_int,
                   a: *const lapack_complex_double, lda: *const lapack_int,
                   ipiv: *const lapack_int, b: *mut lapack_complex_double,
                   ldb: *const lapack_int, info: *mut lapack_int);

    pub fn ssptrs_(uplo: *const c_char, n: *const lapack_int, nrhs: *const lapack_int,
                   ap: *const c_float, ipiv: *const lapack_int, b: *mut c_float,
                   ldb: *const lapack_int, info: *mut lapack_int);
    pub fn dsptrs_(uplo: *const c_char, n: *const lapack_int, nrhs: *const lapack_int,
                   ap: *const c_double, ipiv: *const lapack_int, b: *mut c_double,
                   ldb: *const lapack_int, info: *mut lapack_int);
    pub fn csptrs_(uplo: *const c_char, n: *const lapack_int, nrhs: *const lapack_int,
                   ap: *const lapack_complex_float, ipiv: *const lapack_int,
                   b: *mut lapack_complex_float, ldb: *const lapack_int, info: *mut lapack_int);
    pub fn zsptrs_(uplo: *const c_char, n: *const lapack_int, nrhs: *const lapack_int,
                   ap: *const lapack_complex_double, ipiv: *const lapack_int,
                   b: *mut lapack_complex_double, ldb: *const lapack_int, info: *mut lapack_int);

    pub fn chptrs_(uplo: *const c_char, n: *const lapack_int, nrhs: *const lapack_int,
                   ap: *const lapack_complex_float, ipiv: *const lapack_int,
                   b: *mut lapack_complex_float, ldb: *const lapack_int, info: *mut lapack_int);
    pub fn zhptrs_(uplo: *const c_char, n: *const lapack_int, nrhs: *const lapack_int,
                   ap: *const lapack_complex_double, ipiv: *const lapack_int,
                   b: *mut lapack_complex_double, ldb: *const lapack_int, info: *mut lapack_int);

    pub fn strtrs_(uplo: *const c_char, trans: *const c_char, diag: *const c_char,
                   n: *const lapack_int, nrhs: *const lapack_int, a: *const c_float,
                   lda: *const lapack_int, b: *mut c_float, ldb: *const lapack_int,
                   info: *mut lapack_int);
    pub fn dtrtrs_(uplo: *const c_char, trans: *const c_char, diag: *const c_char,
                   n: *const lapack_int, nrhs: *const lapack_int, a: *const c_double,
                   lda: *const lapack_int, b: *mut c_double, ldb: *const lapack_int,
                   info: *mut lapack_int);
    pub fn ctrtrs_(uplo: *const c_char, trans: *const c_char, diag: *const c_char,
                   n: *const lapack_int, nrhs: *const lapack_int,
                   a: *const lapack_complex_float, lda: *const lapack_int,
                   b: *mut lapack_complex_float, ldb: *const lapack_int, info: *mut lapack_int);
    pub fn ztrtrs_(uplo: *const c_char, trans: *const c_char, diag: *const c_char,
                   n: *const lapack_int, nrhs: *const lapack_int,
                   a: *const lapack_complex_double, lda: *const lapack_int,
                   b: *mut lapack_complex_double, ldb: *const lapack_int, info: *mut lapack_int);

    pub fn stptrs_(uplo: *const c_char, trans: *const c_char, diag: *const c_char,
                   n: *const lapack_int, nrhs: *const lapack_int, ap: *const c_float,
                   b: *mut c_float, ldb: *const lapack_int, info: *mut lapack_int);
    pub fn dtptrs_(uplo: *const c_char, trans: *const c_char, diag: *const c_char,
                   n: *const lapack_int, nrhs: *const lapack_int, ap: *const c_double,
                   b: *mut c_double, ldb: *const lapack_int, info: *mut lapack_int);
    pub fn ctptrs_(uplo: *const c_char, trans: *const c_char, diag: *const c_char,
                   n: *const lapack_int, nrhs: *const lapack_int,
                   ap: *const lapack_complex_float, b: *mut lapack_complex_float,
                   ldb: *const lapack_int, info: *mut lapack_int);
    pub fn ztptrs_(uplo: *const c_char, trans: *const c_char, diag: *const c_char,
                   n: *const lapack_int, nrhs: *const lapack_int,
                   ap: *const lapack_complex_double, b: *mut lapack_complex_double,
                   ldb: *const lapack_int, info: *mut lapack_int);

    pub fn stbtrs_(uplo: *const c_char, trans: *const c_char, diag: *const c_char,
                   n: *const lapack_int, kd: *const lapack_int, nrhs: *const lapack_int,
                   ab: *const c_float, ldab: *const lapack_int, b: *mut c_float,
                   ldb: *const lapack_int, info: *mut lapack_int);
    pub fn dtbtrs_(uplo: *const c_char, trans: *const c_char, diag: *const c_char,
                   n: *const lapack_int, kd: *const lapack_int, nrhs: *const lapack_int,
                   ab: *const c_double, ldab: *const lapack_int, b: *mut c_double,
                   ldb: *const lapack_int, info: *mut lapack_int);
    pub fn ctbtrs_(uplo: *const c_char, trans: *const c_char, diag: *const c_char,
                   n: *const lapack_int, kd: *const lapack_int, nrhs: *const lapack_int,
                   ab: *const lapack_complex_float, ldab: *const lapack_int,
                   b: *mut lapack_complex_float, ldb: *const lapack_int, info: *mut lapack_int);
    pub fn ztbtrs_(uplo: *const c_char, trans: *const c_char, diag: *const c_char,
                   n: *const lapack_int, kd: *const lapack_int, nrhs: *const lapack_int,
                   ab: *const lapack_complex_double, ldab: *const lapack_int,
                   b: *mut lapack_complex_double, ldb: *const lapack_int, info: *mut lapack_int);

    pub fn sgecon_(norm: *const c_char, n: *const lapack_int, a: *const c_float,
                   lda: *const lapack_int, anorm: *const c_float, rcond: *mut c_float,
                   work: *mut c_float, iwork: *mut lapack_int, info: *mut lapack_int);
    pub fn dgecon_(norm: *const c_char, n: *const lapack_int, a: *const c_double,
                   lda: *const lapack_int, anorm: *const c_double, rcond: *mut c_double,
                   work: *mut c_double, iwork: *mut lapack_int, info: *mut lapack_int);
    pub fn cgecon_(norm: *const c_char, n: *const lapack_int, a: *const lapack_complex_float,
                   lda: *const lapack_int, anorm: *const c_float, rcond: *mut c_float,
                   work: *mut lapack_complex_float, rwork: *mut c_float, info: *mut lapack_int);
    pub fn zgecon_(norm: *const c_char, n: *const lapack_int, a: *const lapack_complex_double,
                   lda: *const lapack_int, anorm: *const c_double, rcond: *mut c_double,
                   work: *mut lapack_complex_double, rwork: *mut c_double, info: *mut lapack_int);

    pub fn sgbcon_(norm: *const c_char, n: *const lapack_int, kl: *const lapack_int,
                   ku: *const lapack_int, ab: *const c_float, ldab: *const lapack_int,
                   ipiv: *const lapack_int, anorm: *const c_float, rcond: *mut c_float,
                   work: *mut c_float, iwork: *mut lapack_int, info: *mut lapack_int);
    pub fn dgbcon_(norm: *const c_char, n: *const lapack_int, kl: *const lapack_int,
                   ku: *const lapack_int, ab: *const c_double, ldab: *const lapack_int,
                   ipiv: *const lapack_int, anorm: *const c_double, rcond: *mut c_double,
                   work: *mut c_double, iwork: *mut lapack_int, info: *mut lapack_int);
    pub fn cgbcon_(norm: *const c_char, n: *const lapack_int, kl: *const lapack_int,
                   ku: *const lapack_int, ab: *const lapack_complex_float,
                   ldab: *const lapack_int, ipiv: *const lapack_int, anorm: *const c_float,
                   rcond: *mut c_float, work: *mut lapack_complex_float, rwork: *mut c_float,
                   info: *mut lapack_int);
    pub fn zgbcon_(norm: *const c_char, n: *const lapack_int, kl: *const lapack_int,
                   ku: *const lapack_int, ab: *const lapack_complex_double,
                   ldab: *const lapack_int, ipiv: *const lapack_int, anorm: *const c_double,
                   rcond: *mut c_double, work: *mut lapack_complex_double, rwork: *mut c_double,
                   info: *mut lapack_int);

    pub fn sgtcon_(norm: *const c_char, n: *const lapack_int, dl: *const c_float,
                   d: *const c_float, du: *const c_float, du2: *const c_float,
                   ipiv: *const lapack_int, anorm: *const c_float, rcond: *mut c_float,
                   work: *mut c_float, iwork: *mut lapack_int, info: *mut lapack_int);
    pub fn dgtcon_(norm: *const c_char, n: *const lapack_int, dl: *const c_double,
                   d: *const c_double, du: *const c_double, du2: *const c_double,
                   ipiv: *const lapack_int, anorm: *const c_double, rcond: *mut c_double,
                   work: *mut c_double, iwork: *mut lapack_int, info: *mut lapack_int);
    pub fn cgtcon_(norm: *const c_char, n: *const lapack_int, dl: *const lapack_complex_float,
                   d: *const lapack_complex_float, du: *const lapack_complex_float,
                   du2: *const lapack_complex_float, ipiv: *const lapack_int,
                   anorm: *const c_float, rcond: *mut c_float, work: *mut lapack_complex_float,
                   info: *mut lapack_int);
    pub fn zgtcon_(norm: *const c_char, n: *const lapack_int, dl: *const lapack_complex_double,
                   d: *const lapack_complex_double, du: *const lapack_complex_double,
                   du2: *const lapack_complex_double, ipiv: *const lapack_int,
                   anorm: *const c_double, rcond: *mut c_double,
                   work: *mut lapack_complex_double, info: *mut lapack_int);

    pub fn spocon_(uplo: *const c_char, n: *const lapack_int, a: *const c_float,
                   lda: *const lapack_int, anorm: *const c_float, rcond: *mut c_float,
                   work: *mut c_float, iwork: *mut lapack_int, info: *mut lapack_int);
    pub fn dpocon_(uplo: *const c_char, n: *const lapack_int, a: *const c_double,
                   lda: *const lapack_int, anorm: *const c_double, rcond: *mut c_double,
                   work: *mut c_double, iwork: *mut lapack_int, info: *mut lapack_int);
    pub fn cpocon_(uplo: *const c_char, n: *const lapack_int, a: *const lapack_complex_float,
                   lda: *const lapack_int, anorm: *const c_float, rcond: *mut c_float,
                   work: *mut lapack_complex_float, rwork: *mut c_float, info: *mut lapack_int);
    pub fn zpocon_(uplo: *const c_char, n: *const lapack_int, a: *const lapack_complex_double,
                   lda: *const lapack_int, anorm: *const c_double, rcond: *mut c_double,
                   work: *mut lapack_complex_double, rwork: *mut c_double, info: *mut lapack_int);

    pub fn sppcon_(uplo: *const c_char, n: *const lapack_int, ap: *const c_float,
                   anorm: *const c_float, rcond: *mut c_float, work: *mut c_float,
                   iwork: *mut lapack_int, info: *mut lapack_int);
    pub fn dppcon_(uplo: *const c_char, n: *const lapack_int, ap: *const c_double,
                   anorm: *const c_double, rcond: *mut c_double, work: *mut c_double,
                   iwork: *mut lapack_int, info: *mut lapack_int);
    pub fn cppcon_(uplo: *const c_char, n: *const lapack_int, ap: *const lapack_complex_float,
                   anorm: *const c_float, rcond: *mut c_float, work: *mut lapack_complex_float,
                   rwork: *mut c_float, info: *mut lapack_int);
    pub fn zppcon_(uplo: *const c_char, n: *const lapack_int, ap: *const lapack_complex_double,
                   anorm: *const c_double, rcond: *mut c_double,
                   work: *mut lapack_complex_double, rwork: *mut c_double, info: *mut lapack_int);

    pub fn spbcon_(uplo: *const c_char, n: *const lapack_int, kd: *const lapack_int,
                   ab: *const c_float, ldab: *const lapack_int, anorm: *const c_float,
                   rcond: *mut c_float, work: *mut c_float, iwork: *mut lapack_int,
                   info: *mut lapack_int);
    pub fn dpbcon_(uplo: *const c_char, n: *const lapack_int, kd: *const lapack_int,
                   ab: *const c_double, ldab: *const lapack_int, anorm: *const c_double,
                   rcond: *mut c_double, work: *mut c_double, iwork: *mut lapack_int,
                   info: *mut lapack_int);
    pub fn cpbcon_(uplo: *const c_char, n: *const lapack_int, kd: *const lapack_int,
                   ab: *const lapack_complex_float, ldab: *const lapack_int,
                   anorm: *const c_float, rcond: *mut c_float, work: *mut lapack_complex_float,
                   rwork: *mut c_float, info: *mut lapack_int);
    pub fn zpbcon_(uplo: *const c_char, n: *const lapack_int, kd: *const lapack_int,
                   ab: *const lapack_complex_double, ldab: *const lapack_int,
                   anorm: *const c_double, rcond: *mut c_double,
                   work: *mut lapack_complex_double, rwork: *mut c_double, info: *mut lapack_int);

    pub fn sptcon_(n: *const lapack_int, d: *const c_float, e: *const c_float,
                   anorm: *const c_float, rcond: *mut c_float, work: *mut c_float,
                   info: *mut lapack_int);
    pub fn dptcon_(n: *const lapack_int, d: *const c_double, e: *const c_double,
                   anorm: *const c_double, rcond: *mut c_double, work: *mut c_double,
                   info: *mut lapack_int);
    pub fn cptcon_(n: *const lapack_int, d: *const c_float, e: *const lapack_complex_float,
                   anorm: *const c_float, rcond: *mut c_float, work: *mut c_float,
                   info: *mut lapack_int);
    pub fn zptcon_(n: *const lapack_int, d: *const c_double, e: *const lapack_complex_double,
                   anorm: *const c_double, rcond: *mut c_double, work: *mut c_double,
                   info: *mut lapack_int);

    pub fn ssycon_(uplo: *const c_char, n: *const lapack_int, a: *const c_float,
                   lda: *const lapack_int, ipiv: *const lapack_int, anorm: *const c_float,
                   rcond: *mut c_float, work: *mut c_float, iwork: *mut lapack_int,
                   info: *mut lapack_int);
    pub fn dsycon_(uplo: *const c_char, n: *const lapack_int, a: *const c_double,
                   lda: *const lapack_int, ipiv: *const lapack_int, anorm: *const c_double,
                   rcond: *mut c_double, work: *mut c_double, iwork: *mut lapack_int,
                   info: *mut lapack_int);
    pub fn csycon_(uplo: *const c_char, n: *const lapack_int, a: *const lapack_complex_float,
                   lda: *const lapack_int, ipiv: *const lapack_int, anorm: *const c_float,
                   rcond: *mut c_float, work: *mut lapack_complex_float, info: *mut lapack_int);
    pub fn zsycon_(uplo: *const c_char, n: *const lapack_int, a: *const lapack_complex_double,
                   lda: *const lapack_int, ipiv: *const lapack_int, anorm: *const c_double,
                   rcond: *mut c_double, work: *mut lapack_complex_double, info: *mut lapack_int);

    pub fn checon_(uplo: *const c_char, n: *const lapack_int, a: *const lapack_complex_float,
                   lda: *const lapack_int, ipiv: *const lapack_int, anorm: *const c_float,
                   rcond: *mut c_float, work: *mut lapack_complex_float, info: *mut lapack_int);
    pub fn zhecon_(uplo: *const c_char, n: *const lapack_int, a: *const lapack_complex_double,
                   lda: *const lapack_int, ipiv: *const lapack_int, anorm: *const c_double,
                   rcond: *mut c_double, work: *mut lapack_complex_double, info: *mut lapack_int);

    pub fn sspcon_(uplo: *const c_char, n: *const lapack_int, ap: *const c_float,
                   ipiv: *const lapack_int, anorm: *const c_float, rcond: *mut c_float,
                   work: *mut c_float, iwork: *mut lapack_int, info: *mut lapack_int);
    pub fn dspcon_(uplo: *const c_char, n: *const lapack_int, ap: *const c_double,
                   ipiv: *const lapack_int, anorm: *const c_double, rcond: *mut c_double,
                   work: *mut c_double, iwork: *mut lapack_int, info: *mut lapack_int);
    pub fn cspcon_(uplo: *const c_char, n: *const lapack_int, ap: *const lapack_complex_float,
                   ipiv: *const lapack_int, anorm: *const c_float, rcond: *mut c_float,
                   work: *mut lapack_complex_float, info: *mut lapack_int);
    pub fn zspcon_(uplo: *const c_char, n: *const lapack_int, ap: *const lapack_complex_double,
                   ipiv: *const lapack_int, anorm: *const c_double, rcond: *mut c_double,
                   work: *mut lapack_complex_double, info: *mut lapack_int);

    pub fn chpcon_(uplo: *const c_char, n: *const lapack_int, ap: *const lapack_complex_float,
                   ipiv: *const lapack_int, anorm: *const c_float, rcond: *mut c_float,
                   work: *mut lapack_complex_float, info: *mut lapack_int);
    pub fn zhpcon_(uplo: *const c_char, n: *const lapack_int, ap: *const lapack_complex_double,
                   ipiv: *const lapack_int, anorm: *const c_double, rcond: *mut c_double,
                   work: *mut lapack_complex_double, info: *mut lapack_int);

    pub fn strcon_(norm: *const c_char, uplo: *const c_char, diag: *const c_char,
                   n: *const lapack_int, a: *const c_float, lda: *const lapack_int,
                   rcond: *mut c_float, work: *mut c_float, iwork: *mut lapack_int,
                   info: *mut lapack_int);
    pub fn dtrcon_(norm: *const c_char, uplo: *const c_char, diag: *const c_char,
                   n: *const lapack_int, a: *const c_double, lda: *const lapack_int,
                   rcond: *mut c_double, work: *mut c_double, iwork: *mut lapack_int,
                   info: *mut lapack_int);
    pub fn ctrcon_(norm: *const c_char, uplo: *const c_char, diag: *const c_char,
                   n: *const lapack_int, a: *const lapack_complex_float, lda: *const lapack_int,
                   rcond: *mut c_float, work: *mut lapack_complex_float, rwork: *mut c_float,
                   info: *mut lapack_int);
    pub fn ztrcon_(norm: *const c_char, uplo: *const c_char, diag: *const c_char,
                   n: *const lapack_int, a: *const lapack_complex_double,
                   lda: *const lapack_int, rcond: *mut c_double,
                   work: *mut lapack_complex_double, rwork: *mut c_double, info: *mut lapack_int);

    pub fn stpcon_(norm: *const c_char, uplo: *const c_char, diag: *const c_char,
                   n: *const lapack_int, ap: *const c_float, rcond: *mut c_float,
                   work: *mut c_float, iwork: *mut lapack_int, info: *mut lapack_int);
    pub fn dtpcon_(norm: *const c_char, uplo: *const c_char, diag: *const c_char,
                   n: *const lapack_int, ap: *const c_double, rcond: *mut c_double,
                   work: *mut c_double, iwork: *mut lapack_int, info: *mut lapack_int);
    pub fn ctpcon_(norm: *const c_char, uplo: *const c_char, diag: *const c_char,
                   n: *const lapack_int, ap: *const lapack_complex_float, rcond: *mut c_float,
                   work: *mut lapack_complex_float, rwork: *mut c_float, info: *mut lapack_int);
    pub fn ztpcon_(norm: *const c_char, uplo: *const c_char, diag: *const c_char,
                   n: *const lapack_int, ap: *const lapack_complex_double, rcond: *mut c_double,
                   work: *mut lapack_complex_double, rwork: *mut c_double, info: *mut lapack_int);

    pub fn stbcon_(norm: *const c_char, uplo: *const c_char, diag: *const c_char,
                   n: *const lapack_int, kd: *const lapack_int, ab: *const c_float,
                   ldab: *const lapack_int, rcond: *mut c_float, work: *mut c_float,
                   iwork: *mut lapack_int, info: *mut lapack_int);
    pub fn dtbcon_(norm: *const c_char, uplo: *const c_char, diag: *const c_char,
                   n: *const lapack_int, kd: *const lapack_int, ab: *const c_double,
                   ldab: *const lapack_int, rcond: *mut c_double, work: *mut c_double,
                   iwork: *mut lapack_int, info: *mut lapack_int);
    pub fn ctbcon_(norm: *const c_char, uplo: *const c_char, diag: *const c_char,
                   n: *const lapack_int, kd: *const lapack_int, ab: *const lapack_complex_float,
                   ldab: *const lapack_int, rcond: *mut c_float,
                   work: *mut lapack_complex_float, rwork: *mut c_float, info: *mut lapack_int);
    pub fn ztbcon_(norm: *const c_char, uplo: *const c_char, diag: *const c_char,
                   n: *const lapack_int, kd: *const lapack_int,
                   ab: *const lapack_complex_double, ldab: *const lapack_int,
                   rcond: *mut c_double, work: *mut lapack_complex_double, rwork: *mut c_double,
                   info: *mut lapack_int);

    pub fn sgerfs_(trans: *const c_char, n: *const lapack_int, nrhs: *const lapack_int,
                   a: *const c_float, lda: *const lapack_int, af: *const c_float,
                   ldaf: *const lapack_int, ipiv: *const lapack_int, b: *const c_float,
                   ldb: *const lapack_int, x: *mut c_float, ldx: *const lapack_int,
                   ferr: *mut c_float, berr: *mut c_float, work: *mut c_float,
                   iwork: *mut lapack_int, info: *mut lapack_int);
    pub fn dgerfs_(trans: *const c_char, n: *const lapack_int, nrhs: *const lapack_int,
                   a: *const c_double, lda: *const lapack_int, af: *const c_double,
                   ldaf: *const lapack_int, ipiv: *const lapack_int, b: *const c_double,
                   ldb: *const lapack_int, x: *mut c_double, ldx: *const lapack_int,
                   ferr: *mut c_double, berr: *mut c_double, work: *mut c_double,
                   iwork: *mut lapack_int, info: *mut lapack_int);
    pub fn cgerfs_(trans: *const c_char, n: *const lapack_int, nrhs: *const lapack_int,
                   a: *const lapack_complex_float, lda: *const lapack_int,
                   af: *const lapack_complex_float, ldaf: *const lapack_int,
                   ipiv: *const lapack_int, b: *const lapack_complex_float,
                   ldb: *const lapack_int, x: *mut lapack_complex_float, ldx: *const lapack_int,
                   ferr: *mut c_float, berr: *mut c_float, work: *mut lapack_complex_float,
                   rwork: *mut c_float, info: *mut lapack_int);
    pub fn zgerfs_(trans: *const c_char, n: *const lapack_int, nrhs: *const lapack_int,
                   a: *const lapack_complex_double, lda: *const lapack_int,
                   af: *const lapack_complex_double, ldaf: *const lapack_int,
                   ipiv: *const lapack_int, b: *const lapack_complex_double,
                   ldb: *const lapack_int, x: *mut lapack_complex_double,
                   ldx: *const lapack_int, ferr: *mut c_double, berr: *mut c_double,
                   work: *mut lapack_complex_double, rwork: *mut c_double, info: *mut lapack_int);

    pub fn dgerfsx_(trans: *const c_char, equed: *const c_char, n: *const lapack_int,
                    nrhs: *const lapack_int, a: *const c_double, lda: *const lapack_int,
                    af: *const c_double, ldaf: *const lapack_int, ipiv: *const lapack_int,
                    r: *const c_double, c: *const c_double, b: *const c_double,
                    ldb: *const lapack_int, x: *mut c_double, ldx: *const lapack_int,
                    rcond: *mut c_double, berr: *mut c_double, n_err_bnds: *const lapack_int,
                    err_bnds_norm: *mut c_double, err_bnds_comp: *mut c_double,
                    nparams: *const lapack_int, params: *mut c_double, work: *mut c_double,
                    iwork: *mut lapack_int, info: *mut lapack_int);
    pub fn sgerfsx_(trans: *const c_char, equed: *const c_char, n: *const lapack_int,
                    nrhs: *const lapack_int, a: *const c_float, lda: *const lapack_int,
                    af: *const c_float, ldaf: *const lapack_int, ipiv: *const lapack_int,
                    r: *const c_float, c: *const c_float, b: *const c_float,
                    ldb: *const lapack_int, x: *mut c_float, ldx: *const lapack_int,
                    rcond: *mut c_float, berr: *mut c_float, n_err_bnds: *const lapack_int,
                    err_bnds_norm: *mut c_float, err_bnds_comp: *mut c_float,
                    nparams: *const lapack_int, params: *mut c_float, work: *mut c_float,
                    iwork: *mut lapack_int, info: *mut lapack_int);
    pub fn zgerfsx_(trans: *const c_char, equed: *const c_char, n: *const lapack_int,
                    nrhs: *const lapack_int, a: *const lapack_complex_double,
                    lda: *const lapack_int, af: *const lapack_complex_double,
                    ldaf: *const lapack_int, ipiv: *const lapack_int, r: *const c_double,
                    c: *const c_double, b: *const lapack_complex_double, ldb: *const lapack_int,
                    x: *mut lapack_complex_double, ldx: *const lapack_int, rcond: *mut c_double,
                    berr: *mut c_double, n_err_bnds: *const lapack_int,
                    err_bnds_norm: *mut c_double, err_bnds_comp: *mut c_double,
                    nparams: *const lapack_int, params: *mut c_double,
                    work: *mut lapack_complex_double, rwork: *mut c_double,
                    info: *mut lapack_int);
    pub fn cgerfsx_(trans: *const c_char, equed: *const c_char, n: *const lapack_int,
                    nrhs: *const lapack_int, a: *const lapack_complex_float,
                    lda: *const lapack_int, af: *const lapack_complex_float,
                    ldaf: *const lapack_int, ipiv: *const lapack_int, r: *const c_float,
                    c: *const c_float, b: *const lapack_complex_float, ldb: *const lapack_int,
                    x: *mut lapack_complex_float, ldx: *const lapack_int, rcond: *mut c_float,
                    berr: *mut c_float, n_err_bnds: *const lapack_int,
                    err_bnds_norm: *mut c_float, err_bnds_comp: *mut c_float,
                    nparams: *const lapack_int, params: *mut c_float,
                    work: *mut lapack_complex_float, rwork: *mut c_float, info: *mut lapack_int);

    pub fn sgbrfs_(trans: *const c_char, n: *const lapack_int, kl: *const lapack_int,
                   ku: *const lapack_int, nrhs: *const lapack_int, ab: *const c_float,
                   ldab: *const lapack_int, afb: *const c_float, ldafb: *const lapack_int,
                   ipiv: *const lapack_int, b: *const c_float, ldb: *const lapack_int,
                   x: *mut c_float, ldx: *const lapack_int, ferr: *mut c_float,
                   berr: *mut c_float, work: *mut c_float, iwork: *mut lapack_int,
                   info: *mut lapack_int);
    pub fn dgbrfs_(trans: *const c_char, n: *const lapack_int, kl: *const lapack_int,
                   ku: *const lapack_int, nrhs: *const lapack_int, ab: *const c_double,
                   ldab: *const lapack_int, afb: *const c_double, ldafb: *const lapack_int,
                   ipiv: *const lapack_int, b: *const c_double, ldb: *const lapack_int,
                   x: *mut c_double, ldx: *const lapack_int, ferr: *mut c_double,
                   berr: *mut c_double, work: *mut c_double, iwork: *mut lapack_int,
                   info: *mut lapack_int);
    pub fn cgbrfs_(trans: *const c_char, n: *const lapack_int, kl: *const lapack_int,
                   ku: *const lapack_int, nrhs: *const lapack_int,
                   ab: *const lapack_complex_float, ldab: *const lapack_int,
                   afb: *const lapack_complex_float, ldafb: *const lapack_int,
                   ipiv: *const lapack_int, b: *const lapack_complex_float,
                   ldb: *const lapack_int, x: *mut lapack_complex_float, ldx: *const lapack_int,
                   ferr: *mut c_float, berr: *mut c_float, work: *mut lapack_complex_float,
                   rwork: *mut c_float, info: *mut lapack_int);
    pub fn zgbrfs_(trans: *const c_char, n: *const lapack_int, kl: *const lapack_int,
                   ku: *const lapack_int, nrhs: *const lapack_int,
                   ab: *const lapack_complex_double, ldab: *const lapack_int,
                   afb: *const lapack_complex_double, ldafb: *const lapack_int,
                   ipiv: *const lapack_int, b: *const lapack_complex_double,
                   ldb: *const lapack_int, x: *mut lapack_complex_double,
                   ldx: *const lapack_int, ferr: *mut c_double, berr: *mut c_double,
                   work: *mut lapack_complex_double, rwork: *mut c_double, info: *mut lapack_int);

    pub fn dgbrfsx_(trans: *const c_char, equed: *const c_char, n: *const lapack_int,
                    kl: *const lapack_int, ku: *const lapack_int, nrhs: *const lapack_int,
                    ab: *const c_double, ldab: *const lapack_int, afb: *const c_double,
                    ldafb: *const lapack_int, ipiv: *const lapack_int, r: *const c_double,
                    c: *const c_double, b: *const c_double, ldb: *const lapack_int,
                    x: *mut c_double, ldx: *const lapack_int, rcond: *mut c_double,
                    berr: *mut c_double, n_err_bnds: *const lapack_int,
                    err_bnds_norm: *mut c_double, err_bnds_comp: *mut c_double,
                    nparams: *const lapack_int, params: *mut c_double, work: *mut c_double,
                    iwork: *mut lapack_int, info: *mut lapack_int);
    pub fn sgbrfsx_(trans: *const c_char, equed: *const c_char, n: *const lapack_int,
                    kl: *const lapack_int, ku: *const lapack_int, nrhs: *const lapack_int,
                    ab: *const c_float, ldab: *const lapack_int, afb: *const c_float,
                    ldafb: *const lapack_int, ipiv: *const lapack_int, r: *const c_float,
                    c: *const c_float, b: *const c_float, ldb: *const lapack_int,
                    x: *mut c_float, ldx: *const lapack_int, rcond: *mut c_float,
                    berr: *mut c_float, n_err_bnds: *const lapack_int,
                    err_bnds_norm: *mut c_float, err_bnds_comp: *mut c_float,
                    nparams: *const lapack_int, params: *mut c_float, work: *mut c_float,
                    iwork: *mut lapack_int, info: *mut lapack_int);
    pub fn zgbrfsx_(trans: *const c_char, equed: *const c_char, n: *const lapack_int,
                    kl: *const lapack_int, ku: *const lapack_int, nrhs: *const lapack_int,
                    ab: *const lapack_complex_double, ldab: *const lapack_int,
                    afb: *const lapack_complex_double, ldafb: *const lapack_int,
                    ipiv: *const lapack_int, r: *const c_double, c: *const c_double,
                    b: *const lapack_complex_double, ldb: *const lapack_int,
                    x: *mut lapack_complex_double, ldx: *const lapack_int, rcond: *mut c_double,
                    berr: *mut c_double, n_err_bnds: *const lapack_int,
                    err_bnds_norm: *mut c_double, err_bnds_comp: *mut c_double,
                    nparams: *const lapack_int, params: *mut c_double,
                    work: *mut lapack_complex_double, rwork: *mut c_double,
                    info: *mut lapack_int);
    pub fn cgbrfsx_(trans: *const c_char, equed: *const c_char, n: *const lapack_int,
                    kl: *const lapack_int, ku: *const lapack_int, nrhs: *const lapack_int,
                    ab: *const lapack_complex_float, ldab: *const lapack_int,
                    afb: *const lapack_complex_float, ldafb: *const lapack_int,
                    ipiv: *const lapack_int, r: *const c_float, c: *const c_float,
                    b: *const lapack_complex_float, ldb: *const lapack_int,
                    x: *mut lapack_complex_float, ldx: *const lapack_int, rcond: *mut c_float,
                    berr: *mut c_float, n_err_bnds: *const lapack_int,
                    err_bnds_norm: *mut c_float, err_bnds_comp: *mut c_float,
                    nparams: *const lapack_int, params: *mut c_float,
                    work: *mut lapack_complex_float, rwork: *mut c_float, info: *mut lapack_int);

    pub fn sgtrfs_(trans: *const c_char, n: *const lapack_int, nrhs: *const lapack_int,
                   dl: *const c_float, d: *const c_float, du: *const c_float,
                   dlf: *const c_float, df: *const c_float, duf: *const c_float,
                   du2: *const c_float, ipiv: *const lapack_int, b: *const c_float,
                   ldb: *const lapack_int, x: *mut c_float, ldx: *const lapack_int,
                   ferr: *mut c_float, berr: *mut c_float, work: *mut c_float,
                   iwork: *mut lapack_int, info: *mut lapack_int);
    pub fn dgtrfs_(trans: *const c_char, n: *const lapack_int, nrhs: *const lapack_int,
                   dl: *const c_double, d: *const c_double, du: *const c_double,
                   dlf: *const c_double, df: *const c_double, duf: *const c_double,
                   du2: *const c_double, ipiv: *const lapack_int, b: *const c_double,
                   ldb: *const lapack_int, x: *mut c_double, ldx: *const lapack_int,
                   ferr: *mut c_double, berr: *mut c_double, work: *mut c_double,
                   iwork: *mut lapack_int, info: *mut lapack_int);
    pub fn cgtrfs_(trans: *const c_char, n: *const lapack_int, nrhs: *const lapack_int,
                   dl: *const lapack_complex_float, d: *const lapack_complex_float,
                   du: *const lapack_complex_float, dlf: *const lapack_complex_float,
                   df: *const lapack_complex_float, duf: *const lapack_complex_float,
                   du2: *const lapack_complex_float, ipiv: *const lapack_int,
                   b: *const lapack_complex_float, ldb: *const lapack_int,
                   x: *mut lapack_complex_float, ldx: *const lapack_int, ferr: *mut c_float,
                   berr: *mut c_float, work: *mut lapack_complex_float, rwork: *mut c_float,
                   info: *mut lapack_int);
    pub fn zgtrfs_(trans: *const c_char, n: *const lapack_int, nrhs: *const lapack_int,
                   dl: *const lapack_complex_double, d: *const lapack_complex_double,
                   du: *const lapack_complex_double, dlf: *const lapack_complex_double,
                   df: *const lapack_complex_double, duf: *const lapack_complex_double,
                   du2: *const lapack_complex_double, ipiv: *const lapack_int,
                   b: *const lapack_complex_double, ldb: *const lapack_int,
                   x: *mut lapack_complex_double, ldx: *const lapack_int, ferr: *mut c_double,
                   berr: *mut c_double, work: *mut lapack_complex_double, rwork: *mut c_double,
                   info: *mut lapack_int);

    pub fn sporfs_(uplo: *const c_char, n: *const lapack_int, nrhs: *const lapack_int,
                   a: *const c_float, lda: *const lapack_int, af: *const c_float,
                   ldaf: *const lapack_int, b: *const c_float, ldb: *const lapack_int,
                   x: *mut c_float, ldx: *const lapack_int, ferr: *mut c_float,
                   berr: *mut c_float, work: *mut c_float, iwork: *mut lapack_int,
                   info: *mut lapack_int);
    pub fn dporfs_(uplo: *const c_char, n: *const lapack_int, nrhs: *const lapack_int,
                   a: *const c_double, lda: *const lapack_int, af: *const c_double,
                   ldaf: *const lapack_int, b: *const c_double, ldb: *const lapack_int,
                   x: *mut c_double, ldx: *const lapack_int, ferr: *mut c_double,
                   berr: *mut c_double, work: *mut c_double, iwork: *mut lapack_int,
                   info: *mut lapack_int);
    pub fn cporfs_(uplo: *const c_char, n: *const lapack_int, nrhs: *const lapack_int,
                   a: *const lapack_complex_float, lda: *const lapack_int,
                   af: *const lapack_complex_float, ldaf: *const lapack_int,
                   b: *const lapack_complex_float, ldb: *const lapack_int,
                   x: *mut lapack_complex_float, ldx: *const lapack_int, ferr: *mut c_float,
                   berr: *mut c_float, work: *mut lapack_complex_float, rwork: *mut c_float,
                   info: *mut lapack_int);
    pub fn zporfs_(uplo: *const c_char, n: *const lapack_int, nrhs: *const lapack_int,
                   a: *const lapack_complex_double, lda: *const lapack_int,
                   af: *const lapack_complex_double, ldaf: *const lapack_int,
                   b: *const lapack_complex_double, ldb: *const lapack_int,
                   x: *mut lapack_complex_double, ldx: *const lapack_int, ferr: *mut c_double,
                   berr: *mut c_double, work: *mut lapack_complex_double, rwork: *mut c_double,
                   info: *mut lapack_int);

    pub fn dporfsx_(uplo: *const c_char, equed: *const c_char, n: *const lapack_int,
                    nrhs: *const lapack_int, a: *const c_double, lda: *const lapack_int,
                    af: *const c_double, ldaf: *const lapack_int, s: *const c_double,
                    b: *const c_double, ldb: *const lapack_int, x: *mut c_double,
                    ldx: *const lapack_int, rcond: *mut c_double, berr: *mut c_double,
                    n_err_bnds: *const lapack_int, err_bnds_norm: *mut c_double,
                    err_bnds_comp: *mut c_double, nparams: *const lapack_int,
                    params: *mut c_double, work: *mut c_double, iwork: *mut lapack_int,
                    info: *mut lapack_int);
    pub fn sporfsx_(uplo: *const c_char, equed: *const c_char, n: *const lapack_int,
                    nrhs: *const lapack_int, a: *const c_float, lda: *const lapack_int,
                    af: *const c_float, ldaf: *const lapack_int, s: *const c_float,
                    b: *const c_float, ldb: *const lapack_int, x: *mut c_float,
                    ldx: *const lapack_int, rcond: *mut c_float, berr: *mut c_float,
                    n_err_bnds: *const lapack_int, err_bnds_norm: *mut c_float,
                    err_bnds_comp: *mut c_float, nparams: *const lapack_int,
                    params: *mut c_float, work: *mut c_float, iwork: *mut lapack_int,
                    info: *mut lapack_int);
    pub fn zporfsx_(uplo: *const c_char, equed: *const c_char, n: *const lapack_int,
                    nrhs: *const lapack_int, a: *const lapack_complex_double,
                    lda: *const lapack_int, af: *const lapack_complex_double,
                    ldaf: *const lapack_int, s: *const c_double,
                    b: *const lapack_complex_double, ldb: *const lapack_int,
                    x: *mut lapack_complex_double, ldx: *const lapack_int, rcond: *mut c_double,
                    berr: *mut c_double, n_err_bnds: *const lapack_int,
                    err_bnds_norm: *mut c_double, err_bnds_comp: *mut c_double,
                    nparams: *const lapack_int, params: *mut c_double,
                    work: *mut lapack_complex_double, rwork: *mut c_double,
                    info: *mut lapack_int);
    pub fn cporfsx_(uplo: *const c_char, equed: *const c_char, n: *const lapack_int,
                    nrhs: *const lapack_int, a: *const lapack_complex_float,
                    lda: *const lapack_int, af: *const lapack_complex_float,
                    ldaf: *const lapack_int, s: *const c_float, b: *const lapack_complex_float,
                    ldb: *const lapack_int, x: *mut lapack_complex_float,
                    ldx: *const lapack_int, rcond: *mut c_float, berr: *mut c_float,
                    n_err_bnds: *const lapack_int, err_bnds_norm: *mut c_float,
                    err_bnds_comp: *mut c_float, nparams: *const lapack_int,
                    params: *mut c_float, work: *mut lapack_complex_float, rwork: *mut c_float,
                    info: *mut lapack_int);

    pub fn spprfs_(uplo: *const c_char, n: *const lapack_int, nrhs: *const lapack_int,
                   ap: *const c_float, afp: *const c_float, b: *const c_float,
                   ldb: *const lapack_int, x: *mut c_float, ldx: *const lapack_int,
                   ferr: *mut c_float, berr: *mut c_float, work: *mut c_float,
                   iwork: *mut lapack_int, info: *mut lapack_int);
    pub fn dpprfs_(uplo: *const c_char, n: *const lapack_int, nrhs: *const lapack_int,
                   ap: *const c_double, afp: *const c_double, b: *const c_double,
                   ldb: *const lapack_int, x: *mut c_double, ldx: *const lapack_int,
                   ferr: *mut c_double, berr: *mut c_double, work: *mut c_double,
                   iwork: *mut lapack_int, info: *mut lapack_int);
    pub fn cpprfs_(uplo: *const c_char, n: *const lapack_int, nrhs: *const lapack_int,
                   ap: *const lapack_complex_float, afp: *const lapack_complex_float,
                   b: *const lapack_complex_float, ldb: *const lapack_int,
                   x: *mut lapack_complex_float, ldx: *const lapack_int, ferr: *mut c_float,
                   berr: *mut c_float, work: *mut lapack_complex_float, rwork: *mut c_float,
                   info: *mut lapack_int);
    pub fn zpprfs_(uplo: *const c_char, n: *const lapack_int, nrhs: *const lapack_int,
                   ap: *const lapack_complex_double, afp: *const lapack_complex_double,
                   b: *const lapack_complex_double, ldb: *const lapack_int,
                   x: *mut lapack_complex_double, ldx: *const lapack_int, ferr: *mut c_double,
                   berr: *mut c_double, work: *mut lapack_complex_double, rwork: *mut c_double,
                   info: *mut lapack_int);

    pub fn spbrfs_(uplo: *const c_char, n: *const lapack_int, kd: *const lapack_int,
                   nrhs: *const lapack_int, ab: *const c_float, ldab: *const lapack_int,
                   afb: *const c_float, ldafb: *const lapack_int, b: *const c_float,
                   ldb: *const lapack_int, x: *mut c_float, ldx: *const lapack_int,
                   ferr: *mut c_float, berr: *mut c_float, work: *mut c_float,
                   iwork: *mut lapack_int, info: *mut lapack_int);
    pub fn dpbrfs_(uplo: *const c_char, n: *const lapack_int, kd: *const lapack_int,
                   nrhs: *const lapack_int, ab: *const c_double, ldab: *const lapack_int,
                   afb: *const c_double, ldafb: *const lapack_int, b: *const c_double,
                   ldb: *const lapack_int, x: *mut c_double, ldx: *const lapack_int,
                   ferr: *mut c_double, berr: *mut c_double, work: *mut c_double,
                   iwork: *mut lapack_int, info: *mut lapack_int);
    pub fn cpbrfs_(uplo: *const c_char, n: *const lapack_int, kd: *const lapack_int,
                   nrhs: *const lapack_int, ab: *const lapack_complex_float,
                   ldab: *const lapack_int, afb: *const lapack_complex_float,
                   ldafb: *const lapack_int, b: *const lapack_complex_float,
                   ldb: *const lapack_int, x: *mut lapack_complex_float, ldx: *const lapack_int,
                   ferr: *mut c_float, berr: *mut c_float, work: *mut lapack_complex_float,
                   rwork: *mut c_float, info: *mut lapack_int);
    pub fn zpbrfs_(uplo: *const c_char, n: *const lapack_int, kd: *const lapack_int,
                   nrhs: *const lapack_int, ab: *const lapack_complex_double,
                   ldab: *const lapack_int, afb: *const lapack_complex_double,
                   ldafb: *const lapack_int, b: *const lapack_complex_double,
                   ldb: *const lapack_int, x: *mut lapack_complex_double,
                   ldx: *const lapack_int, ferr: *mut c_double, berr: *mut c_double,
                   work: *mut lapack_complex_double, rwork: *mut c_double, info: *mut lapack_int);

    pub fn sptrfs_(n: *const lapack_int, nrhs: *const lapack_int, d: *const c_float,
                   e: *const c_float, df: *const c_float, ef: *const c_float, b: *const c_float,
                   ldb: *const lapack_int, x: *mut c_float, ldx: *const lapack_int,
                   ferr: *mut c_float, berr: *mut c_float, work: *mut c_float,
                   info: *mut lapack_int);
    pub fn dptrfs_(n: *const lapack_int, nrhs: *const lapack_int, d: *const c_double,
                   e: *const c_double, df: *const c_double, ef: *const c_double,
                   b: *const c_double, ldb: *const lapack_int, x: *mut c_double,
                   ldx: *const lapack_int, ferr: *mut c_double, berr: *mut c_double,
                   work: *mut c_double, info: *mut lapack_int);
    pub fn cptrfs_(uplo: *const c_char, n: *const lapack_int, nrhs: *const lapack_int,
                   d: *const c_float, e: *const lapack_complex_float, df: *const c_float,
                   ef: *const lapack_complex_float, b: *const lapack_complex_float,
                   ldb: *const lapack_int, x: *mut lapack_complex_float, ldx: *const lapack_int,
                   ferr: *mut c_float, berr: *mut c_float, work: *mut lapack_complex_float,
                   rwork: *mut c_float, info: *mut lapack_int);
    pub fn zptrfs_(uplo: *const c_char, n: *const lapack_int, nrhs: *const lapack_int,
                   d: *const c_double, e: *const lapack_complex_double, df: *const c_double,
                   ef: *const lapack_complex_double, b: *const lapack_complex_double,
                   ldb: *const lapack_int, x: *mut lapack_complex_double,
                   ldx: *const lapack_int, ferr: *mut c_double, berr: *mut c_double,
                   work: *mut lapack_complex_double, rwork: *mut c_double, info: *mut lapack_int);

    pub fn ssyrfs_(uplo: *const c_char, n: *const lapack_int, nrhs: *const lapack_int,
                   a: *const c_float, lda: *const lapack_int, af: *const c_float,
                   ldaf: *const lapack_int, ipiv: *const lapack_int, b: *const c_float,
                   ldb: *const lapack_int, x: *mut c_float, ldx: *const lapack_int,
                   ferr: *mut c_float, berr: *mut c_float, work: *mut c_float,
                   iwork: *mut lapack_int, info: *mut lapack_int);
    pub fn dsyrfs_(uplo: *const c_char, n: *const lapack_int, nrhs: *const lapack_int,
                   a: *const c_double, lda: *const lapack_int, af: *const c_double,
                   ldaf: *const lapack_int, ipiv: *const lapack_int, b: *const c_double,
                   ldb: *const lapack_int, x: *mut c_double, ldx: *const lapack_int,
                   ferr: *mut c_double, berr: *mut c_double, work: *mut c_double,
                   iwork: *mut lapack_int, info: *mut lapack_int);
    pub fn csyrfs_(uplo: *const c_char, n: *const lapack_int, nrhs: *const lapack_int,
                   a: *const lapack_complex_float, lda: *const lapack_int,
                   af: *const lapack_complex_float, ldaf: *const lapack_int,
                   ipiv: *const lapack_int, b: *const lapack_complex_float,
                   ldb: *const lapack_int, x: *mut lapack_complex_float, ldx: *const lapack_int,
                   ferr: *mut c_float, berr: *mut c_float, work: *mut lapack_complex_float,
                   rwork: *mut c_float, info: *mut lapack_int);
    pub fn zsyrfs_(uplo: *const c_char, n: *const lapack_int, nrhs: *const lapack_int,
                   a: *const lapack_complex_double, lda: *const lapack_int,
                   af: *const lapack_complex_double, ldaf: *const lapack_int,
                   ipiv: *const lapack_int, b: *const lapack_complex_double,
                   ldb: *const lapack_int, x: *mut lapack_complex_double,
                   ldx: *const lapack_int, ferr: *mut c_double, berr: *mut c_double,
                   work: *mut lapack_complex_double, rwork: *mut c_double, info: *mut lapack_int);

    pub fn dsyrfsx_(uplo: *const c_char, equed: *const c_char, n: *const lapack_int,
                    nrhs: *const lapack_int, a: *const c_double, lda: *const lapack_int,
                    af: *const c_double, ldaf: *const lapack_int, ipiv: *const lapack_int,
                    s: *const c_double, b: *const c_double, ldb: *const lapack_int,
                    x: *mut c_double, ldx: *const lapack_int, rcond: *mut c_double,
                    berr: *mut c_double, n_err_bnds: *const lapack_int,
                    err_bnds_norm: *mut c_double, err_bnds_comp: *mut c_double,
                    nparams: *const lapack_int, params: *mut c_double, work: *mut c_double,
                    iwork: *mut lapack_int, info: *mut lapack_int);
    pub fn ssyrfsx_(uplo: *const c_char, equed: *const c_char, n: *const lapack_int,
                    nrhs: *const lapack_int, a: *const c_float, lda: *const lapack_int,
                    af: *const c_float, ldaf: *const lapack_int, ipiv: *const lapack_int,
                    s: *const c_float, b: *const c_float, ldb: *const lapack_int,
                    x: *mut c_float, ldx: *const lapack_int, rcond: *mut c_float,
                    berr: *mut c_float, n_err_bnds: *const lapack_int,
                    err_bnds_norm: *mut c_float, err_bnds_comp: *mut c_float,
                    nparams: *const lapack_int, params: *mut c_float, work: *mut c_float,
                    iwork: *mut lapack_int, info: *mut lapack_int);
    pub fn zsyrfsx_(uplo: *const c_char, equed: *const c_char, n: *const lapack_int,
                    nrhs: *const lapack_int, a: *const lapack_complex_double,
                    lda: *const lapack_int, af: *const lapack_complex_double,
                    ldaf: *const lapack_int, ipiv: *const lapack_int, s: *const c_double,
                    b: *const lapack_complex_double, ldb: *const lapack_int,
                    x: *mut lapack_complex_double, ldx: *const lapack_int, rcond: *mut c_double,
                    berr: *mut c_double, n_err_bnds: *const lapack_int,
                    err_bnds_norm: *mut c_double, err_bnds_comp: *mut c_double,
                    nparams: *const lapack_int, params: *mut c_double,
                    work: *mut lapack_complex_double, rwork: *mut c_double,
                    info: *mut lapack_int);
    pub fn csyrfsx_(uplo: *const c_char, equed: *const c_char, n: *const lapack_int,
                    nrhs: *const lapack_int, a: *const lapack_complex_float,
                    lda: *const lapack_int, af: *const lapack_complex_float,
                    ldaf: *const lapack_int, ipiv: *const lapack_int, s: *const c_float,
                    b: *const lapack_complex_float, ldb: *const lapack_int,
                    x: *mut lapack_complex_float, ldx: *const lapack_int, rcond: *mut c_float,
                    berr: *mut c_float, n_err_bnds: *const lapack_int,
                    err_bnds_norm: *mut c_float, err_bnds_comp: *mut c_float,
                    nparams: *const lapack_int, params: *mut c_float,
                    work: *mut lapack_complex_float, rwork: *mut c_float, info: *mut lapack_int);

    pub fn cherfs_(uplo: *const c_char, n: *const lapack_int, nrhs: *const lapack_int,
                   a: *const lapack_complex_float, lda: *const lapack_int,
                   af: *const lapack_complex_float, ldaf: *const lapack_int,
                   ipiv: *const lapack_int, b: *const lapack_complex_float,
                   ldb: *const lapack_int, x: *mut lapack_complex_float, ldx: *const lapack_int,
                   ferr: *mut c_float, berr: *mut c_float, work: *mut lapack_complex_float,
                   rwork: *mut c_float, info: *mut lapack_int);
    pub fn zherfs_(uplo: *const c_char, n: *const lapack_int, nrhs: *const lapack_int,
                   a: *const lapack_complex_double, lda: *const lapack_int,
                   af: *const lapack_complex_double, ldaf: *const lapack_int,
                   ipiv: *const lapack_int, b: *const lapack_complex_double,
                   ldb: *const lapack_int, x: *mut lapack_complex_double,
                   ldx: *const lapack_int, ferr: *mut c_double, berr: *mut c_double,
                   work: *mut lapack_complex_double, rwork: *mut c_double, info: *mut lapack_int);

    pub fn zherfsx_(uplo: *const c_char, equed: *const c_char, n: *const lapack_int,
                    nrhs: *const lapack_int, a: *const lapack_complex_double,
                    lda: *const lapack_int, af: *const lapack_complex_double,
                    ldaf: *const lapack_int, ipiv: *const lapack_int, s: *const c_double,
                    b: *const lapack_complex_double, ldb: *const lapack_int,
                    x: *mut lapack_complex_double, ldx: *const lapack_int, rcond: *mut c_double,
                    berr: *mut c_double, n_err_bnds: *const lapack_int,
                    err_bnds_norm: *mut c_double, err_bnds_comp: *mut c_double,
                    nparams: *const lapack_int, params: *mut c_double,
                    work: *mut lapack_complex_double, rwork: *mut c_double,
                    info: *mut lapack_int);
    pub fn cherfsx_(uplo: *const c_char, equed: *const c_char, n: *const lapack_int,
                    nrhs: *const lapack_int, a: *const lapack_complex_float,
                    lda: *const lapack_int, af: *const lapack_complex_float,
                    ldaf: *const lapack_int, ipiv: *const lapack_int, s: *const c_float,
                    b: *const lapack_complex_float, ldb: *const lapack_int,
                    x: *mut lapack_complex_float, ldx: *const lapack_int, rcond: *mut c_float,
                    berr: *mut c_float, n_err_bnds: *const lapack_int,
                    err_bnds_norm: *mut c_float, err_bnds_comp: *mut c_float,
                    nparams: *const lapack_int, params: *mut c_float,
                    work: *mut lapack_complex_float, rwork: *mut c_float, info: *mut lapack_int);

    pub fn ssprfs_(uplo: *const c_char, n: *const lapack_int, nrhs: *const lapack_int,
                   ap: *const c_float, afp: *const c_float, ipiv: *const lapack_int,
                   b: *const c_float, ldb: *const lapack_int, x: *mut c_float,
                   ldx: *const lapack_int, ferr: *mut c_float, berr: *mut c_float,
                   work: *mut c_float, iwork: *mut lapack_int, info: *mut lapack_int);
    pub fn dsprfs_(uplo: *const c_char, n: *const lapack_int, nrhs: *const lapack_int,
                   ap: *const c_double, afp: *const c_double, ipiv: *const lapack_int,
                   b: *const c_double, ldb: *const lapack_int, x: *mut c_double,
                   ldx: *const lapack_int, ferr: *mut c_double, berr: *mut c_double,
                   work: *mut c_double, iwork: *mut lapack_int, info: *mut lapack_int);
    pub fn csprfs_(uplo: *const c_char, n: *const lapack_int, nrhs: *const lapack_int,
                   ap: *const lapack_complex_float, afp: *const lapack_complex_float,
                   ipiv: *const lapack_int, b: *const lapack_complex_float,
                   ldb: *const lapack_int, x: *mut lapack_complex_float, ldx: *const lapack_int,
                   ferr: *mut c_float, berr: *mut c_float, work: *mut lapack_complex_float,
                   rwork: *mut c_float, info: *mut lapack_int);
    pub fn zsprfs_(uplo: *const c_char, n: *const lapack_int, nrhs: *const lapack_int,
                   ap: *const lapack_complex_double, afp: *const lapack_complex_double,
                   ipiv: *const lapack_int, b: *const lapack_complex_double,
                   ldb: *const lapack_int, x: *mut lapack_complex_double,
                   ldx: *const lapack_int, ferr: *mut c_double, berr: *mut c_double,
                   work: *mut lapack_complex_double, rwork: *mut c_double, info: *mut lapack_int);

    pub fn chprfs_(uplo: *const c_char, n: *const lapack_int, nrhs: *const lapack_int,
                   ap: *const lapack_complex_float, afp: *const lapack_complex_float,
                   ipiv: *const lapack_int, b: *const lapack_complex_float,
                   ldb: *const lapack_int, x: *mut lapack_complex_float, ldx: *const lapack_int,
                   ferr: *mut c_float, berr: *mut c_float, work: *mut lapack_complex_float,
                   rwork: *mut c_float, info: *mut lapack_int);
    pub fn zhprfs_(uplo: *const c_char, n: *const lapack_int, nrhs: *const lapack_int,
                   ap: *const lapack_complex_double, afp: *const lapack_complex_double,
                   ipiv: *const lapack_int, b: *const lapack_complex_double,
                   ldb: *const lapack_int, x: *mut lapack_complex_double,
                   ldx: *const lapack_int, ferr: *mut c_double, berr: *mut c_double,
                   work: *mut lapack_complex_double, rwork: *mut c_double, info: *mut lapack_int);

    pub fn strrfs_(uplo: *const c_char, trans: *const c_char, diag: *const c_char,
                   n: *const lapack_int, nrhs: *const lapack_int, a: *const c_float,
                   lda: *const lapack_int, b: *const c_float, ldb: *const lapack_int,
                   x: *const c_float, ldx: *const lapack_int, ferr: *mut c_float,
                   berr: *mut c_float, work: *mut c_float, iwork: *mut lapack_int,
                   info: *mut lapack_int);
    pub fn dtrrfs_(uplo: *const c_char, trans: *const c_char, diag: *const c_char,
                   n: *const lapack_int, nrhs: *const lapack_int, a: *const c_double,
                   lda: *const lapack_int, b: *const c_double, ldb: *const lapack_int,
                   x: *const c_double, ldx: *const lapack_int, ferr: *mut c_double,
                   berr: *mut c_double, work: *mut c_double, iwork: *mut lapack_int,
                   info: *mut lapack_int);
    pub fn ctrrfs_(uplo: *const c_char, trans: *const c_char, diag: *const c_char,
                   n: *const lapack_int, nrhs: *const lapack_int,
                   a: *const lapack_complex_float, lda: *const lapack_int,
                   b: *const lapack_complex_float, ldb: *const lapack_int,
                   x: *const lapack_complex_float, ldx: *const lapack_int, ferr: *mut c_float,
                   berr: *mut c_float, work: *mut lapack_complex_float, rwork: *mut c_float,
                   info: *mut lapack_int);
    pub fn ztrrfs_(uplo: *const c_char, trans: *const c_char, diag: *const c_char,
                   n: *const lapack_int, nrhs: *const lapack_int,
                   a: *const lapack_complex_double, lda: *const lapack_int,
                   b: *const lapack_complex_double, ldb: *const lapack_int,
                   x: *const lapack_complex_double, ldx: *const lapack_int, ferr: *mut c_double,
                   berr: *mut c_double, work: *mut lapack_complex_double, rwork: *mut c_double,
                   info: *mut lapack_int);

    pub fn stprfs_(uplo: *const c_char, trans: *const c_char, diag: *const c_char,
                   n: *const lapack_int, nrhs: *const lapack_int, ap: *const c_float,
                   b: *const c_float, ldb: *const lapack_int, x: *const c_float,
                   ldx: *const lapack_int, ferr: *mut c_float, berr: *mut c_float,
                   work: *mut c_float, iwork: *mut lapack_int, info: *mut lapack_int);
    pub fn dtprfs_(uplo: *const c_char, trans: *const c_char, diag: *const c_char,
                   n: *const lapack_int, nrhs: *const lapack_int, ap: *const c_double,
                   b: *const c_double, ldb: *const lapack_int, x: *const c_double,
                   ldx: *const lapack_int, ferr: *mut c_double, berr: *mut c_double,
                   work: *mut c_double, iwork: *mut lapack_int, info: *mut lapack_int);
    pub fn ctprfs_(uplo: *const c_char, trans: *const c_char, diag: *const c_char,
                   n: *const lapack_int, nrhs: *const lapack_int,
                   ap: *const lapack_complex_float, b: *const lapack_complex_float,
                   ldb: *const lapack_int, x: *const lapack_complex_float,
                   ldx: *const lapack_int, ferr: *mut c_float, berr: *mut c_float,
                   work: *mut lapack_complex_float, rwork: *mut c_float, info: *mut lapack_int);
    pub fn ztprfs_(uplo: *const c_char, trans: *const c_char, diag: *const c_char,
                   n: *const lapack_int, nrhs: *const lapack_int,
                   ap: *const lapack_complex_double, b: *const lapack_complex_double,
                   ldb: *const lapack_int, x: *const lapack_complex_double,
                   ldx: *const lapack_int, ferr: *mut c_double, berr: *mut c_double,
                   work: *mut lapack_complex_double, rwork: *mut c_double, info: *mut lapack_int);

    pub fn stbrfs_(uplo: *const c_char, trans: *const c_char, diag: *const c_char,
                   n: *const lapack_int, kd: *const lapack_int, nrhs: *const lapack_int,
                   ab: *const c_float, ldab: *const lapack_int, b: *const c_float,
                   ldb: *const lapack_int, x: *const c_float, ldx: *const lapack_int,
                   ferr: *mut c_float, berr: *mut c_float, work: *mut c_float,
                   iwork: *mut lapack_int, info: *mut lapack_int);
    pub fn dtbrfs_(uplo: *const c_char, trans: *const c_char, diag: *const c_char,
                   n: *const lapack_int, kd: *const lapack_int, nrhs: *const lapack_int,
                   ab: *const c_double, ldab: *const lapack_int, b: *const c_double,
                   ldb: *const lapack_int, x: *const c_double, ldx: *const lapack_int,
                   ferr: *mut c_double, berr: *mut c_double, work: *mut c_double,
                   iwork: *mut lapack_int, info: *mut lapack_int);
    pub fn ctbrfs_(uplo: *const c_char, trans: *const c_char, diag: *const c_char,
                   n: *const lapack_int, kd: *const lapack_int, nrhs: *const lapack_int,
                   ab: *const lapack_complex_float, ldab: *const lapack_int,
                   b: *const lapack_complex_float, ldb: *const lapack_int,
                   x: *const lapack_complex_float, ldx: *const lapack_int, ferr: *mut c_float,
                   berr: *mut c_float, work: *mut lapack_complex_float, rwork: *mut c_float,
                   info: *mut lapack_int);
    pub fn ztbrfs_(uplo: *const c_char, trans: *const c_char, diag: *const c_char,
                   n: *const lapack_int, kd: *const lapack_int, nrhs: *const lapack_int,
                   ab: *const lapack_complex_double, ldab: *const lapack_int,
                   b: *const lapack_complex_double, ldb: *const lapack_int,
                   x: *const lapack_complex_double, ldx: *const lapack_int, ferr: *mut c_double,
                   berr: *mut c_double, work: *mut lapack_complex_double, rwork: *mut c_double,
                   info: *mut lapack_int);

    pub fn sgetri_(n: *const lapack_int, a: *mut c_float, lda: *const lapack_int,
                   ipiv: *const lapack_int, work: *mut c_float, lwork: *const lapack_int,
                   info: *mut lapack_int);
    pub fn dgetri_(n: *const lapack_int, a: *mut c_double, lda: *const lapack_int,
                   ipiv: *const lapack_int, work: *mut c_double, lwork: *const lapack_int,
                   info: *mut lapack_int);
    pub fn cgetri_(n: *const lapack_int, a: *mut lapack_complex_float, lda: *const lapack_int,
                   ipiv: *const lapack_int, work: *mut lapack_complex_float,
                   lwork: *const lapack_int, info: *mut lapack_int);
    pub fn zgetri_(n: *const lapack_int, a: *mut lapack_complex_double, lda: *const lapack_int,
                   ipiv: *const lapack_int, work: *mut lapack_complex_double,
                   lwork: *const lapack_int, info: *mut lapack_int);

    pub fn spotri_(uplo: *const c_char, n: *const lapack_int, a: *mut c_float,
                   lda: *const lapack_int, info: *mut lapack_int);
    pub fn dpotri_(uplo: *const c_char, n: *const lapack_int, a: *mut c_double,
                   lda: *const lapack_int, info: *mut lapack_int);
    pub fn cpotri_(uplo: *const c_char, n: *const lapack_int, a: *mut lapack_complex_float,
                   lda: *const lapack_int, info: *mut lapack_int);
    pub fn zpotri_(uplo: *const c_char, n: *const lapack_int, a: *mut lapack_complex_double,
                   lda: *const lapack_int, info: *mut lapack_int);

    pub fn dpftri_(transr: *const c_char, uplo: *const c_char, n: *const lapack_int,
                   a: *mut c_double, info: *mut lapack_int);
    pub fn spftri_(transr: *const c_char, uplo: *const c_char, n: *const lapack_int,
                   a: *mut c_float, info: *mut lapack_int);
    pub fn zpftri_(transr: *const c_char, uplo: *const c_char, n: *const lapack_int,
                   a: *mut lapack_complex_double, info: *mut lapack_int);
    pub fn cpftri_(transr: *const c_char, uplo: *const c_char, n: *const lapack_int,
                   a: *mut lapack_complex_float, info: *mut lapack_int);

    pub fn spptri_(uplo: *const c_char, n: *const lapack_int, ap: *mut c_float,
                   info: *mut lapack_int);
    pub fn dpptri_(uplo: *const c_char, n: *const lapack_int, ap: *mut c_double,
                   info: *mut lapack_int);
    pub fn cpptri_(uplo: *const c_char, n: *const lapack_int, ap: *mut lapack_complex_float,
                   info: *mut lapack_int);
    pub fn zpptri_(uplo: *const c_char, n: *const lapack_int, ap: *mut lapack_complex_double,
                   info: *mut lapack_int);

    pub fn ssytri_(uplo: *const c_char, n: *const lapack_int, a: *mut c_float,
                   lda: *const lapack_int, ipiv: *const lapack_int, work: *mut c_float,
                   info: *mut lapack_int);
    pub fn dsytri_(uplo: *const c_char, n: *const lapack_int, a: *mut c_double,
                   lda: *const lapack_int, ipiv: *const lapack_int, work: *mut c_double,
                   info: *mut lapack_int);
    pub fn csytri_(uplo: *const c_char, n: *const lapack_int, a: *mut lapack_complex_float,
                   lda: *const lapack_int, ipiv: *const lapack_int,
                   work: *mut lapack_complex_float, info: *mut lapack_int);
    pub fn zsytri_(uplo: *const c_char, n: *const lapack_int, a: *mut lapack_complex_double,
                   lda: *const lapack_int, ipiv: *const lapack_int,
                   work: *mut lapack_complex_double, info: *mut lapack_int);

    pub fn chetri_(uplo: *const c_char, n: *const lapack_int, a: *mut lapack_complex_float,
                   lda: *const lapack_int, ipiv: *const lapack_int,
                   work: *mut lapack_complex_float, info: *mut lapack_int);
    pub fn zhetri_(uplo: *const c_char, n: *const lapack_int, a: *mut lapack_complex_double,
                   lda: *const lapack_int, ipiv: *const lapack_int,
                   work: *mut lapack_complex_double, info: *mut lapack_int);

    pub fn ssptri_(uplo: *const c_char, n: *const lapack_int, ap: *mut c_float,
                   ipiv: *const lapack_int, work: *mut c_float, info: *mut lapack_int);
    pub fn dsptri_(uplo: *const c_char, n: *const lapack_int, ap: *mut c_double,
                   ipiv: *const lapack_int, work: *mut c_double, info: *mut lapack_int);
    pub fn csptri_(uplo: *const c_char, n: *const lapack_int, ap: *mut lapack_complex_float,
                   ipiv: *const lapack_int, work: *mut lapack_complex_float,
                   info: *mut lapack_int);
    pub fn zsptri_(uplo: *const c_char, n: *const lapack_int, ap: *mut lapack_complex_double,
                   ipiv: *const lapack_int, work: *mut lapack_complex_double,
                   info: *mut lapack_int);

    pub fn chptri_(uplo: *const c_char, n: *const lapack_int, ap: *mut lapack_complex_float,
                   ipiv: *const lapack_int, work: *mut lapack_complex_float,
                   info: *mut lapack_int);
    pub fn zhptri_(uplo: *const c_char, n: *const lapack_int, ap: *mut lapack_complex_double,
                   ipiv: *const lapack_int, work: *mut lapack_complex_double,
                   info: *mut lapack_int);

    pub fn strtri_(uplo: *const c_char, diag: *const c_char, n: *const lapack_int,
                   a: *mut c_float, lda: *const lapack_int, info: *mut lapack_int);
    pub fn dtrtri_(uplo: *const c_char, diag: *const c_char, n: *const lapack_int,
                   a: *mut c_double, lda: *const lapack_int, info: *mut lapack_int);
    pub fn ctrtri_(uplo: *const c_char, diag: *const c_char, n: *const lapack_int,
                   a: *mut lapack_complex_float, lda: *const lapack_int, info: *mut lapack_int);
    pub fn ztrtri_(uplo: *const c_char, diag: *const c_char, n: *const lapack_int,
                   a: *mut lapack_complex_double, lda: *const lapack_int, info: *mut lapack_int);

    pub fn dtftri_(transr: *const c_char, uplo: *const c_char, diag: *const c_char,
                   n: *const lapack_int, a: *mut c_double, info: *mut lapack_int);
    pub fn stftri_(transr: *const c_char, uplo: *const c_char, diag: *const c_char,
                   n: *const lapack_int, a: *mut c_float, info: *mut lapack_int);
    pub fn ztftri_(transr: *const c_char, uplo: *const c_char, diag: *const c_char,
                   n: *const lapack_int, a: *mut lapack_complex_double, info: *mut lapack_int);
    pub fn ctftri_(transr: *const c_char, uplo: *const c_char, diag: *const c_char,
                   n: *const lapack_int, a: *mut lapack_complex_float, info: *mut lapack_int);

    pub fn stptri_(uplo: *const c_char, diag: *const c_char, n: *const lapack_int,
                   ap: *mut c_float, info: *mut lapack_int);
    pub fn dtptri_(uplo: *const c_char, diag: *const c_char, n: *const lapack_int,
                   ap: *mut c_double, info: *mut lapack_int);
    pub fn ctptri_(uplo: *const c_char, diag: *const c_char, n: *const lapack_int,
                   ap: *mut lapack_complex_float, info: *mut lapack_int);
    pub fn ztptri_(uplo: *const c_char, diag: *const c_char, n: *const lapack_int,
                   ap: *mut lapack_complex_double, info: *mut lapack_int);

    pub fn sgeequ_(m: *const lapack_int, n: *const lapack_int, a: *const c_float,
                   lda: *const lapack_int, r: *mut c_float, c: *mut c_float,
                   rowcnd: *mut c_float, colcnd: *mut c_float, amax: *mut c_float,
                   info: *mut lapack_int);
    pub fn dgeequ_(m: *const lapack_int, n: *const lapack_int, a: *const c_double,
                   lda: *const lapack_int, r: *mut c_double, c: *mut c_double,
                   rowcnd: *mut c_double, colcnd: *mut c_double, amax: *mut c_double,
                   info: *mut lapack_int);
    pub fn cgeequ_(m: *const lapack_int, n: *const lapack_int, a: *const lapack_complex_float,
                   lda: *const lapack_int, r: *mut c_float, c: *mut c_float,
                   rowcnd: *mut c_float, colcnd: *mut c_float, amax: *mut c_float,
                   info: *mut lapack_int);
    pub fn zgeequ_(m: *const lapack_int, n: *const lapack_int, a: *const lapack_complex_double,
                   lda: *const lapack_int, r: *mut c_double, c: *mut c_double,
                   rowcnd: *mut c_double, colcnd: *mut c_double, amax: *mut c_double,
                   info: *mut lapack_int);

    pub fn dgeequb_(m: *const lapack_int, n: *const lapack_int, a: *const c_double,
                    lda: *const lapack_int, r: *mut c_double, c: *mut c_double,
                    rowcnd: *mut c_double, colcnd: *mut c_double, amax: *mut c_double,
                    info: *mut lapack_int);
    pub fn sgeequb_(m: *const lapack_int, n: *const lapack_int, a: *const c_float,
                    lda: *const lapack_int, r: *mut c_float, c: *mut c_float,
                    rowcnd: *mut c_float, colcnd: *mut c_float, amax: *mut c_float,
                    info: *mut lapack_int);
    pub fn zgeequb_(m: *const lapack_int, n: *const lapack_int, a: *const lapack_complex_double,
                    lda: *const lapack_int, r: *mut c_double, c: *mut c_double,
                    rowcnd: *mut c_double, colcnd: *mut c_double, amax: *mut c_double,
                    info: *mut lapack_int);
    pub fn cgeequb_(m: *const lapack_int, n: *const lapack_int, a: *const lapack_complex_float,
                    lda: *const lapack_int, r: *mut c_float, c: *mut c_float,
                    rowcnd: *mut c_float, colcnd: *mut c_float, amax: *mut c_float,
                    info: *mut lapack_int);

    pub fn sgbequ_(m: *const lapack_int, n: *const lapack_int, kl: *const lapack_int,
                   ku: *const lapack_int, ab: *const c_float, ldab: *const lapack_int,
                   r: *mut c_float, c: *mut c_float, rowcnd: *mut c_float, colcnd: *mut c_float,
                   amax: *mut c_float, info: *mut lapack_int);
    pub fn dgbequ_(m: *const lapack_int, n: *const lapack_int, kl: *const lapack_int,
                   ku: *const lapack_int, ab: *const c_double, ldab: *const lapack_int,
                   r: *mut c_double, c: *mut c_double, rowcnd: *mut c_double,
                   colcnd: *mut c_double, amax: *mut c_double, info: *mut lapack_int);
    pub fn cgbequ_(m: *const lapack_int, n: *const lapack_int, kl: *const lapack_int,
                   ku: *const lapack_int, ab: *const lapack_complex_float,
                   ldab: *const lapack_int, r: *mut c_float, c: *mut c_float,
                   rowcnd: *mut c_float, colcnd: *mut c_float, amax: *mut c_float,
                   info: *mut lapack_int);
    pub fn zgbequ_(m: *const lapack_int, n: *const lapack_int, kl: *const lapack_int,
                   ku: *const lapack_int, ab: *const lapack_complex_double,
                   ldab: *const lapack_int, r: *mut c_double, c: *mut c_double,
                   rowcnd: *mut c_double, colcnd: *mut c_double, amax: *mut c_double,
                   info: *mut lapack_int);

    pub fn dgbequb_(m: *const lapack_int, n: *const lapack_int, kl: *const lapack_int,
                    ku: *const lapack_int, ab: *const c_double, ldab: *const lapack_int,
                    r: *mut c_double, c: *mut c_double, rowcnd: *mut c_double,
                    colcnd: *mut c_double, amax: *mut c_double, info: *mut lapack_int);
    pub fn sgbequb_(m: *const lapack_int, n: *const lapack_int, kl: *const lapack_int,
                    ku: *const lapack_int, ab: *const c_float, ldab: *const lapack_int,
                    r: *mut c_float, c: *mut c_float, rowcnd: *mut c_float,
                    colcnd: *mut c_float, amax: *mut c_float, info: *mut lapack_int);
    pub fn zgbequb_(m: *const lapack_int, n: *const lapack_int, kl: *const lapack_int,
                    ku: *const lapack_int, ab: *const lapack_complex_double,
                    ldab: *const lapack_int, r: *mut c_double, c: *mut c_double,
                    rowcnd: *mut c_double, colcnd: *mut c_double, amax: *mut c_double,
                    info: *mut lapack_int);
    pub fn cgbequb_(m: *const lapack_int, n: *const lapack_int, kl: *const lapack_int,
                    ku: *const lapack_int, ab: *const lapack_complex_float,
                    ldab: *const lapack_int, r: *mut c_float, c: *mut c_float,
                    rowcnd: *mut c_float, colcnd: *mut c_float, amax: *mut c_float,
                    info: *mut lapack_int);

    pub fn spoequ_(n: *const lapack_int, a: *const c_float, lda: *const lapack_int,
                   s: *mut c_float, scond: *mut c_float, amax: *mut c_float,
                   info: *mut lapack_int);
    pub fn dpoequ_(n: *const lapack_int, a: *const c_double, lda: *const lapack_int,
                   s: *mut c_double, scond: *mut c_double, amax: *mut c_double,
                   info: *mut lapack_int);
    pub fn cpoequ_(n: *const lapack_int, a: *const lapack_complex_float, lda: *const lapack_int,
                   s: *mut c_float, scond: *mut c_float, amax: *mut c_float,
                   info: *mut lapack_int);
    pub fn zpoequ_(n: *const lapack_int, a: *const lapack_complex_double,
                   lda: *const lapack_int, s: *mut c_double, scond: *mut c_double,
                   amax: *mut c_double, info: *mut lapack_int);

    pub fn dpoequb_(n: *const lapack_int, a: *const c_double, lda: *const lapack_int,
                    s: *mut c_double, scond: *mut c_double, amax: *mut c_double,
                    info: *mut lapack_int);
    pub fn spoequb_(n: *const lapack_int, a: *const c_float, lda: *const lapack_int,
                    s: *mut c_float, scond: *mut c_float, amax: *mut c_float,
                    info: *mut lapack_int);
    pub fn zpoequb_(n: *const lapack_int, a: *const lapack_complex_double,
                    lda: *const lapack_int, s: *mut c_double, scond: *mut c_double,
                    amax: *mut c_double, info: *mut lapack_int);
    pub fn cpoequb_(n: *const lapack_int, a: *const lapack_complex_float,
                    lda: *const lapack_int, s: *mut c_float, scond: *mut c_float,
                    amax: *mut c_float, info: *mut lapack_int);

    pub fn sppequ_(uplo: *const c_char, n: *const lapack_int, ap: *const c_float,
                   s: *mut c_float, scond: *mut c_float, amax: *mut c_float,
                   info: *mut lapack_int);
    pub fn dppequ_(uplo: *const c_char, n: *const lapack_int, ap: *const c_double,
                   s: *mut c_double, scond: *mut c_double, amax: *mut c_double,
                   info: *mut lapack_int);
    pub fn cppequ_(uplo: *const c_char, n: *const lapack_int, ap: *const lapack_complex_float,
                   s: *mut c_float, scond: *mut c_float, amax: *mut c_float,
                   info: *mut lapack_int);
    pub fn zppequ_(uplo: *const c_char, n: *const lapack_int, ap: *const lapack_complex_double,
                   s: *mut c_double, scond: *mut c_double, amax: *mut c_double,
                   info: *mut lapack_int);

    pub fn spbequ_(uplo: *const c_char, n: *const lapack_int, kd: *const lapack_int,
                   ab: *const c_float, ldab: *const lapack_int, s: *mut c_float,
                   scond: *mut c_float, amax: *mut c_float, info: *mut lapack_int);
    pub fn dpbequ_(uplo: *const c_char, n: *const lapack_int, kd: *const lapack_int,
                   ab: *const c_double, ldab: *const lapack_int, s: *mut c_double,
                   scond: *mut c_double, amax: *mut c_double, info: *mut lapack_int);
    pub fn cpbequ_(uplo: *const c_char, n: *const lapack_int, kd: *const lapack_int,
                   ab: *const lapack_complex_float, ldab: *const lapack_int, s: *mut c_float,
                   scond: *mut c_float, amax: *mut c_float, info: *mut lapack_int);
    pub fn zpbequ_(uplo: *const c_char, n: *const lapack_int, kd: *const lapack_int,
                   ab: *const lapack_complex_double, ldab: *const lapack_int, s: *mut c_double,
                   scond: *mut c_double, amax: *mut c_double, info: *mut lapack_int);

    pub fn dsyequb_(uplo: *const c_char, n: *const lapack_int, a: *const c_double,
                    lda: *const lapack_int, s: *mut c_double, scond: *mut c_double,
                    amax: *mut c_double, work: *mut c_double, info: *mut lapack_int);
    pub fn ssyequb_(uplo: *const c_char, n: *const lapack_int, a: *const c_float,
                    lda: *const lapack_int, s: *mut c_float, scond: *mut c_float,
                    amax: *mut c_float, work: *mut c_float, info: *mut lapack_int);
    pub fn zsyequb_(uplo: *const c_char, n: *const lapack_int, a: *const lapack_complex_double,
                    lda: *const lapack_int, s: *mut c_double, scond: *mut c_double,
                    amax: *mut c_double, work: *mut lapack_complex_double, info: *mut lapack_int);
    pub fn csyequb_(uplo: *const c_char, n: *const lapack_int, a: *const lapack_complex_float,
                    lda: *const lapack_int, s: *mut c_float, scond: *mut c_float,
                    amax: *mut c_float, work: *mut lapack_complex_float, info: *mut lapack_int);

    pub fn zheequb_(uplo: *const c_char, n: *const lapack_int, a: *const lapack_complex_double,
                    lda: *const lapack_int, s: *mut c_double, scond: *mut c_double,
                    amax: *mut c_double, work: *mut lapack_complex_double, info: *mut lapack_int);
    pub fn cheequb_(uplo: *const c_char, n: *const lapack_int, a: *const lapack_complex_float,
                    lda: *const lapack_int, s: *mut c_float, scond: *mut c_float,
                    amax: *mut c_float, work: *mut lapack_complex_float, info: *mut lapack_int);

    pub fn sgesv_(n: *const lapack_int, nrhs: *const lapack_int, a: *mut c_float,
                  lda: *const lapack_int, ipiv: *mut lapack_int, b: *mut c_float,
                  ldb: *const lapack_int, info: *mut lapack_int);
    pub fn dgesv_(n: *const lapack_int, nrhs: *const lapack_int, a: *mut c_double,
                  lda: *const lapack_int, ipiv: *mut lapack_int, b: *mut c_double,
                  ldb: *const lapack_int, info: *mut lapack_int);
    pub fn cgesv_(n: *const lapack_int, nrhs: *const lapack_int, a: *mut lapack_complex_float,
                  lda: *const lapack_int, ipiv: *mut lapack_int, b: *mut lapack_complex_float,
                  ldb: *const lapack_int, info: *mut lapack_int);
    pub fn zgesv_(n: *const lapack_int, nrhs: *const lapack_int, a: *mut lapack_complex_double,
                  lda: *const lapack_int, ipiv: *mut lapack_int, b: *mut lapack_complex_double,
                  ldb: *const lapack_int, info: *mut lapack_int);

    pub fn dsgesv_(n: *const lapack_int, nrhs: *const lapack_int, a: *mut c_double,
                   lda: *const lapack_int, ipiv: *mut lapack_int, b: *const c_double,
                   ldb: *const lapack_int, x: *mut c_double, ldx: *const lapack_int,
                   work: *mut c_double, swork: *mut c_float, iter: *mut lapack_int,
                   info: *mut lapack_int);
    pub fn zcgesv_(n: *const lapack_int, nrhs: *const lapack_int, a: *mut lapack_complex_double,
                   lda: *const lapack_int, ipiv: *mut lapack_int,
                   b: *const lapack_complex_double, ldb: *const lapack_int,
                   x: *mut lapack_complex_double, ldx: *const lapack_int,
                   work: *mut lapack_complex_double, swork: *mut lapack_complex_float,
                   rwork: *mut c_double, iter: *mut lapack_int, info: *mut lapack_int);

    pub fn sgesvx_(fact: *const c_char, trans: *const c_char, n: *const lapack_int,
                   nrhs: *const lapack_int, a: *mut c_float, lda: *const lapack_int,
                   af: *mut c_float, ldaf: *const lapack_int, ipiv: *mut lapack_int,
                   equed: *mut c_char, r: *mut c_float, c: *mut c_float, b: *mut c_float,
                   ldb: *const lapack_int, x: *mut c_float, ldx: *const lapack_int,
                   rcond: *mut c_float, ferr: *mut c_float, berr: *mut c_float,
                   work: *mut c_float, iwork: *mut lapack_int, info: *mut lapack_int);
    pub fn dgesvx_(fact: *const c_char, trans: *const c_char, n: *const lapack_int,
                   nrhs: *const lapack_int, a: *mut c_double, lda: *const lapack_int,
                   af: *mut c_double, ldaf: *const lapack_int, ipiv: *mut lapack_int,
                   equed: *mut c_char, r: *mut c_double, c: *mut c_double, b: *mut c_double,
                   ldb: *const lapack_int, x: *mut c_double, ldx: *const lapack_int,
                   rcond: *mut c_double, ferr: *mut c_double, berr: *mut c_double,
                   work: *mut c_double, iwork: *mut lapack_int, info: *mut lapack_int);
    pub fn cgesvx_(fact: *const c_char, trans: *const c_char, n: *const lapack_int,
                   nrhs: *const lapack_int, a: *mut lapack_complex_float,
                   lda: *const lapack_int, af: *mut lapack_complex_float,
                   ldaf: *const lapack_int, ipiv: *mut lapack_int, equed: *mut c_char,
                   r: *mut c_float, c: *mut c_float, b: *mut lapack_complex_float,
                   ldb: *const lapack_int, x: *mut lapack_complex_float, ldx: *const lapack_int,
                   rcond: *mut c_float, ferr: *mut c_float, berr: *mut c_float,
                   work: *mut lapack_complex_float, rwork: *mut c_float, info: *mut lapack_int);
    pub fn zgesvx_(fact: *const c_char, trans: *const c_char, n: *const lapack_int,
                   nrhs: *const lapack_int, a: *mut lapack_complex_double,
                   lda: *const lapack_int, af: *mut lapack_complex_double,
                   ldaf: *const lapack_int, ipiv: *mut lapack_int, equed: *mut c_char,
                   r: *mut c_double, c: *mut c_double, b: *mut lapack_complex_double,
                   ldb: *const lapack_int, x: *mut lapack_complex_double,
                   ldx: *const lapack_int, rcond: *mut c_double, ferr: *mut c_double,
                   berr: *mut c_double, work: *mut lapack_complex_double, rwork: *mut c_double,
                   info: *mut lapack_int);

    pub fn dgesvxx_(fact: *const c_char, trans: *const c_char, n: *const lapack_int,
                    nrhs: *const lapack_int, a: *mut c_double, lda: *const lapack_int,
                    af: *mut c_double, ldaf: *const lapack_int, ipiv: *mut lapack_int,
                    equed: *mut c_char, r: *mut c_double, c: *mut c_double, b: *mut c_double,
                    ldb: *const lapack_int, x: *mut c_double, ldx: *const lapack_int,
                    rcond: *mut c_double, rpvgrw: *mut c_double, berr: *mut c_double,
                    n_err_bnds: *const lapack_int, err_bnds_norm: *mut c_double,
                    err_bnds_comp: *mut c_double, nparams: *const lapack_int,
                    params: *mut c_double, work: *mut c_double, iwork: *mut lapack_int,
                    info: *mut lapack_int);
    pub fn sgesvxx_(fact: *const c_char, trans: *const c_char, n: *const lapack_int,
                    nrhs: *const lapack_int, a: *mut c_float, lda: *const lapack_int,
                    af: *mut c_float, ldaf: *const lapack_int, ipiv: *mut lapack_int,
                    equed: *mut c_char, r: *mut c_float, c: *mut c_float, b: *mut c_float,
                    ldb: *const lapack_int, x: *mut c_float, ldx: *const lapack_int,
                    rcond: *mut c_float, rpvgrw: *mut c_float, berr: *mut c_float,
                    n_err_bnds: *const lapack_int, err_bnds_norm: *mut c_float,
                    err_bnds_comp: *mut c_float, nparams: *const lapack_int,
                    params: *mut c_float, work: *mut c_float, iwork: *mut lapack_int,
                    info: *mut lapack_int);
    pub fn zgesvxx_(fact: *const c_char, trans: *const c_char, n: *const lapack_int,
                    nrhs: *const lapack_int, a: *mut lapack_complex_double,
                    lda: *const lapack_int, af: *mut lapack_complex_double,
                    ldaf: *const lapack_int, ipiv: *mut lapack_int, equed: *mut c_char,
                    r: *mut c_double, c: *mut c_double, b: *mut lapack_complex_double,
                    ldb: *const lapack_int, x: *mut lapack_complex_double,
                    ldx: *const lapack_int, rcond: *mut c_double, rpvgrw: *mut c_double,
                    berr: *mut c_double, n_err_bnds: *const lapack_int,
                    err_bnds_norm: *mut c_double, err_bnds_comp: *mut c_double,
                    nparams: *const lapack_int, params: *mut c_double,
                    work: *mut lapack_complex_double, rwork: *mut c_double,
                    info: *mut lapack_int);
    pub fn cgesvxx_(fact: *const c_char, trans: *const c_char, n: *const lapack_int,
                    nrhs: *const lapack_int, a: *mut lapack_complex_float,
                    lda: *const lapack_int, af: *mut lapack_complex_float,
                    ldaf: *const lapack_int, ipiv: *mut lapack_int, equed: *mut c_char,
                    r: *mut c_float, c: *mut c_float, b: *mut lapack_complex_float,
                    ldb: *const lapack_int, x: *mut lapack_complex_float,
                    ldx: *const lapack_int, rcond: *mut c_float, rpvgrw: *mut c_float,
                    berr: *mut c_float, n_err_bnds: *const lapack_int,
                    err_bnds_norm: *mut c_float, err_bnds_comp: *mut c_float,
                    nparams: *const lapack_int, params: *mut c_float,
                    work: *mut lapack_complex_float, rwork: *mut c_float, info: *mut lapack_int);

    pub fn sgbsv_(n: *const lapack_int, kl: *const lapack_int, ku: *const lapack_int,
                  nrhs: *const lapack_int, ab: *mut c_float, ldab: *const lapack_int,
                  ipiv: *mut lapack_int, b: *mut c_float, ldb: *const lapack_int,
                  info: *mut lapack_int);
    pub fn dgbsv_(n: *const lapack_int, kl: *const lapack_int, ku: *const lapack_int,
                  nrhs: *const lapack_int, ab: *mut c_double, ldab: *const lapack_int,
                  ipiv: *mut lapack_int, b: *mut c_double, ldb: *const lapack_int,
                  info: *mut lapack_int);
    pub fn cgbsv_(n: *const lapack_int, kl: *const lapack_int, ku: *const lapack_int,
                  nrhs: *const lapack_int, ab: *mut lapack_complex_float,
                  ldab: *const lapack_int, ipiv: *mut lapack_int, b: *mut lapack_complex_float,
                  ldb: *const lapack_int, info: *mut lapack_int);
    pub fn zgbsv_(n: *const lapack_int, kl: *const lapack_int, ku: *const lapack_int,
                  nrhs: *const lapack_int, ab: *mut lapack_complex_double,
                  ldab: *const lapack_int, ipiv: *mut lapack_int, b: *mut lapack_complex_double,
                  ldb: *const lapack_int, info: *mut lapack_int);

    pub fn sgbsvx_(fact: *const c_char, trans: *const c_char, n: *const lapack_int,
                   kl: *const lapack_int, ku: *const lapack_int, nrhs: *const lapack_int,
                   ab: *mut c_float, ldab: *const lapack_int, afb: *mut c_float,
                   ldafb: *const lapack_int, ipiv: *mut lapack_int, equed: *mut c_char,
                   r: *mut c_float, c: *mut c_float, b: *mut c_float, ldb: *const lapack_int,
                   x: *mut c_float, ldx: *const lapack_int, rcond: *mut c_float,
                   ferr: *mut c_float, berr: *mut c_float, work: *mut c_float,
                   iwork: *mut lapack_int, info: *mut lapack_int);
    pub fn dgbsvx_(fact: *const c_char, trans: *const c_char, n: *const lapack_int,
                   kl: *const lapack_int, ku: *const lapack_int, nrhs: *const lapack_int,
                   ab: *mut c_double, ldab: *const lapack_int, afb: *mut c_double,
                   ldafb: *const lapack_int, ipiv: *mut lapack_int, equed: *mut c_char,
                   r: *mut c_double, c: *mut c_double, b: *mut c_double, ldb: *const lapack_int,
                   x: *mut c_double, ldx: *const lapack_int, rcond: *mut c_double,
                   ferr: *mut c_double, berr: *mut c_double, work: *mut c_double,
                   iwork: *mut lapack_int, info: *mut lapack_int);
    pub fn cgbsvx_(fact: *const c_char, trans: *const c_char, n: *const lapack_int,
                   kl: *const lapack_int, ku: *const lapack_int, nrhs: *const lapack_int,
                   ab: *mut lapack_complex_float, ldab: *const lapack_int,
                   afb: *mut lapack_complex_float, ldafb: *const lapack_int,
                   ipiv: *mut lapack_int, equed: *mut c_char, r: *mut c_float, c: *mut c_float,
                   b: *mut lapack_complex_float, ldb: *const lapack_int,
                   x: *mut lapack_complex_float, ldx: *const lapack_int, rcond: *mut c_float,
                   ferr: *mut c_float, berr: *mut c_float, work: *mut lapack_complex_float,
                   rwork: *mut c_float, info: *mut lapack_int);
    pub fn zgbsvx_(fact: *const c_char, trans: *const c_char, n: *const lapack_int,
                   kl: *const lapack_int, ku: *const lapack_int, nrhs: *const lapack_int,
                   ab: *mut lapack_complex_double, ldab: *const lapack_int,
                   afb: *mut lapack_complex_double, ldafb: *const lapack_int,
                   ipiv: *mut lapack_int, equed: *mut c_char, r: *mut c_double,
                   c: *mut c_double, b: *mut lapack_complex_double, ldb: *const lapack_int,
                   x: *mut lapack_complex_double, ldx: *const lapack_int, rcond: *mut c_double,
                   ferr: *mut c_double, berr: *mut c_double, work: *mut lapack_complex_double,
                   rwork: *mut c_double, info: *mut lapack_int);

    pub fn dgbsvxx_(fact: *const c_char, trans: *const c_char, n: *const lapack_int,
                    kl: *const lapack_int, ku: *const lapack_int, nrhs: *const lapack_int,
                    ab: *mut c_double, ldab: *const lapack_int, afb: *mut c_double,
                    ldafb: *const lapack_int, ipiv: *mut lapack_int, equed: *mut c_char,
                    r: *mut c_double, c: *mut c_double, b: *mut c_double,
                    ldb: *const lapack_int, x: *mut c_double, ldx: *const lapack_int,
                    rcond: *mut c_double, rpvgrw: *mut c_double, berr: *mut c_double,
                    n_err_bnds: *const lapack_int, err_bnds_norm: *mut c_double,
                    err_bnds_comp: *mut c_double, nparams: *const lapack_int,
                    params: *mut c_double, work: *mut c_double, iwork: *mut lapack_int,
                    info: *mut lapack_int);
    pub fn sgbsvxx_(fact: *const c_char, trans: *const c_char, n: *const lapack_int,
                    kl: *const lapack_int, ku: *const lapack_int, nrhs: *const lapack_int,
                    ab: *mut c_float, ldab: *const lapack_int, afb: *mut c_float,
                    ldafb: *const lapack_int, ipiv: *mut lapack_int, equed: *mut c_char,
                    r: *mut c_float, c: *mut c_float, b: *mut c_float, ldb: *const lapack_int,
                    x: *mut c_float, ldx: *const lapack_int, rcond: *mut c_float,
                    rpvgrw: *mut c_float, berr: *mut c_float, n_err_bnds: *const lapack_int,
                    err_bnds_norm: *mut c_float, err_bnds_comp: *mut c_float,
                    nparams: *const lapack_int, params: *mut c_float, work: *mut c_float,
                    iwork: *mut lapack_int, info: *mut lapack_int);
    pub fn zgbsvxx_(fact: *const c_char, trans: *const c_char, n: *const lapack_int,
                    kl: *const lapack_int, ku: *const lapack_int, nrhs: *const lapack_int,
                    ab: *mut lapack_complex_double, ldab: *const lapack_int,
                    afb: *mut lapack_complex_double, ldafb: *const lapack_int,
                    ipiv: *mut lapack_int, equed: *mut c_char, r: *mut c_double,
                    c: *mut c_double, b: *mut lapack_complex_double, ldb: *const lapack_int,
                    x: *mut lapack_complex_double, ldx: *const lapack_int, rcond: *mut c_double,
                    rpvgrw: *mut c_double, berr: *mut c_double, n_err_bnds: *const lapack_int,
                    err_bnds_norm: *mut c_double, err_bnds_comp: *mut c_double,
                    nparams: *const lapack_int, params: *mut c_double,
                    work: *mut lapack_complex_double, rwork: *mut c_double,
                    info: *mut lapack_int);
    pub fn cgbsvxx_(fact: *const c_char, trans: *const c_char, n: *const lapack_int,
                    kl: *const lapack_int, ku: *const lapack_int, nrhs: *const lapack_int,
                    ab: *mut lapack_complex_float, ldab: *const lapack_int,
                    afb: *mut lapack_complex_float, ldafb: *const lapack_int,
                    ipiv: *mut lapack_int, equed: *mut c_char, r: *mut c_float, c: *mut c_float,
                    b: *mut lapack_complex_float, ldb: *const lapack_int,
                    x: *mut lapack_complex_float, ldx: *const lapack_int, rcond: *mut c_float,
                    rpvgrw: *mut c_float, berr: *mut c_float, n_err_bnds: *const lapack_int,
                    err_bnds_norm: *mut c_float, err_bnds_comp: *mut c_float,
                    nparams: *const lapack_int, params: *mut c_float,
                    work: *mut lapack_complex_float, rwork: *mut c_float, info: *mut lapack_int);

    pub fn sgtsv_(n: *const lapack_int, nrhs: *const lapack_int, dl: *mut c_float,
                  d: *mut c_float, du: *mut c_float, b: *mut c_float, ldb: *const lapack_int,
                  info: *mut lapack_int);
    pub fn dgtsv_(n: *const lapack_int, nrhs: *const lapack_int, dl: *mut c_double,
                  d: *mut c_double, du: *mut c_double, b: *mut c_double, ldb: *const lapack_int,
                  info: *mut lapack_int);
    pub fn cgtsv_(n: *const lapack_int, nrhs: *const lapack_int, dl: *mut lapack_complex_float,
                  d: *mut lapack_complex_float, du: *mut lapack_complex_float,
                  b: *mut lapack_complex_float, ldb: *const lapack_int, info: *mut lapack_int);
    pub fn zgtsv_(n: *const lapack_int, nrhs: *const lapack_int, dl: *mut lapack_complex_double,
                  d: *mut lapack_complex_double, du: *mut lapack_complex_double,
                  b: *mut lapack_complex_double, ldb: *const lapack_int, info: *mut lapack_int);

    pub fn sgtsvx_(fact: *const c_char, trans: *const c_char, n: *const lapack_int,
                   nrhs: *const lapack_int, dl: *const c_float, d: *const c_float,
                   du: *const c_float, dlf: *mut c_float, df: *mut c_float, duf: *mut c_float,
                   du2: *mut c_float, ipiv: *mut lapack_int, b: *const c_float,
                   ldb: *const lapack_int, x: *mut c_float, ldx: *const lapack_int,
                   rcond: *mut c_float, ferr: *mut c_float, berr: *mut c_float,
                   work: *mut c_float, iwork: *mut lapack_int, info: *mut lapack_int);
    pub fn dgtsvx_(fact: *const c_char, trans: *const c_char, n: *const lapack_int,
                   nrhs: *const lapack_int, dl: *const c_double, d: *const c_double,
                   du: *const c_double, dlf: *mut c_double, df: *mut c_double,
                   duf: *mut c_double, du2: *mut c_double, ipiv: *mut lapack_int,
                   b: *const c_double, ldb: *const lapack_int, x: *mut c_double,
                   ldx: *const lapack_int, rcond: *mut c_double, ferr: *mut c_double,
                   berr: *mut c_double, work: *mut c_double, iwork: *mut lapack_int,
                   info: *mut lapack_int);
    pub fn cgtsvx_(fact: *const c_char, trans: *const c_char, n: *const lapack_int,
                   nrhs: *const lapack_int, dl: *const lapack_complex_float,
                   d: *const lapack_complex_float, du: *const lapack_complex_float,
                   dlf: *mut lapack_complex_float, df: *mut lapack_complex_float,
                   duf: *mut lapack_complex_float, du2: *mut lapack_complex_float,
                   ipiv: *mut lapack_int, b: *const lapack_complex_float,
                   ldb: *const lapack_int, x: *mut lapack_complex_float, ldx: *const lapack_int,
                   rcond: *mut c_float, ferr: *mut c_float, berr: *mut c_float,
                   work: *mut lapack_complex_float, rwork: *mut c_float, info: *mut lapack_int);
    pub fn zgtsvx_(fact: *const c_char, trans: *const c_char, n: *const lapack_int,
                   nrhs: *const lapack_int, dl: *const lapack_complex_double,
                   d: *const lapack_complex_double, du: *const lapack_complex_double,
                   dlf: *mut lapack_complex_double, df: *mut lapack_complex_double,
                   duf: *mut lapack_complex_double, du2: *mut lapack_complex_double,
                   ipiv: *mut lapack_int, b: *const lapack_complex_double,
                   ldb: *const lapack_int, x: *mut lapack_complex_double,
                   ldx: *const lapack_int, rcond: *mut c_double, ferr: *mut c_double,
                   berr: *mut c_double, work: *mut lapack_complex_double, rwork: *mut c_double,
                   info: *mut lapack_int);

    pub fn sposv_(uplo: *const c_char, n: *const lapack_int, nrhs: *const lapack_int,
                  a: *mut c_float, lda: *const lapack_int, b: *mut c_float,
                  ldb: *const lapack_int, info: *mut lapack_int);
    pub fn dposv_(uplo: *const c_char, n: *const lapack_int, nrhs: *const lapack_int,
                  a: *mut c_double, lda: *const lapack_int, b: *mut c_double,
                  ldb: *const lapack_int, info: *mut lapack_int);
    pub fn cposv_(uplo: *const c_char, n: *const lapack_int, nrhs: *const lapack_int,
                  a: *mut lapack_complex_float, lda: *const lapack_int,
                  b: *mut lapack_complex_float, ldb: *const lapack_int, info: *mut lapack_int);
    pub fn zposv_(uplo: *const c_char, n: *const lapack_int, nrhs: *const lapack_int,
                  a: *mut lapack_complex_double, lda: *const lapack_int,
                  b: *mut lapack_complex_double, ldb: *const lapack_int, info: *mut lapack_int);

    pub fn dsposv_(uplo: *const c_char, n: *const lapack_int, nrhs: *const lapack_int,
                   a: *mut c_double, lda: *const lapack_int, b: *const c_double,
                   ldb: *const lapack_int, x: *mut c_double, ldx: *const lapack_int,
                   work: *mut c_double, swork: *mut c_float, iter: *mut lapack_int,
                   info: *mut lapack_int);
    pub fn zcposv_(uplo: *const c_char, n: *const lapack_int, nrhs: *const lapack_int,
                   a: *mut lapack_complex_double, lda: *const lapack_int,
                   b: *const lapack_complex_double, ldb: *const lapack_int,
                   x: *mut lapack_complex_double, ldx: *const lapack_int,
                   work: *mut lapack_complex_double, swork: *mut lapack_complex_float,
                   rwork: *mut c_double, iter: *mut lapack_int, info: *mut lapack_int);

    pub fn sposvx_(fact: *const c_char, uplo: *const c_char, n: *const lapack_int,
                   nrhs: *const lapack_int, a: *mut c_float, lda: *const lapack_int,
                   af: *mut c_float, ldaf: *const lapack_int, equed: *mut c_char,
                   s: *mut c_float, b: *mut c_float, ldb: *const lapack_int, x: *mut c_float,
                   ldx: *const lapack_int, rcond: *mut c_float, ferr: *mut c_float,
                   berr: *mut c_float, work: *mut c_float, iwork: *mut lapack_int,
                   info: *mut lapack_int);
    pub fn dposvx_(fact: *const c_char, uplo: *const c_char, n: *const lapack_int,
                   nrhs: *const lapack_int, a: *mut c_double, lda: *const lapack_int,
                   af: *mut c_double, ldaf: *const lapack_int, equed: *mut c_char,
                   s: *mut c_double, b: *mut c_double, ldb: *const lapack_int, x: *mut c_double,
                   ldx: *const lapack_int, rcond: *mut c_double, ferr: *mut c_double,
                   berr: *mut c_double, work: *mut c_double, iwork: *mut lapack_int,
                   info: *mut lapack_int);
    pub fn cposvx_(fact: *const c_char, uplo: *const c_char, n: *const lapack_int,
                   nrhs: *const lapack_int, a: *mut lapack_complex_float,
                   lda: *const lapack_int, af: *mut lapack_complex_float,
                   ldaf: *const lapack_int, equed: *mut c_char, s: *mut c_float,
                   b: *mut lapack_complex_float, ldb: *const lapack_int,
                   x: *mut lapack_complex_float, ldx: *const lapack_int, rcond: *mut c_float,
                   ferr: *mut c_float, berr: *mut c_float, work: *mut lapack_complex_float,
                   rwork: *mut c_float, info: *mut lapack_int);
    pub fn zposvx_(fact: *const c_char, uplo: *const c_char, n: *const lapack_int,
                   nrhs: *const lapack_int, a: *mut lapack_complex_double,
                   lda: *const lapack_int, af: *mut lapack_complex_double,
                   ldaf: *const lapack_int, equed: *mut c_char, s: *mut c_double,
                   b: *mut lapack_complex_double, ldb: *const lapack_int,
                   x: *mut lapack_complex_double, ldx: *const lapack_int, rcond: *mut c_double,
                   ferr: *mut c_double, berr: *mut c_double, work: *mut lapack_complex_double,
                   rwork: *mut c_double, info: *mut lapack_int);

    pub fn dposvxx_(fact: *const c_char, uplo: *const c_char, n: *const lapack_int,
                    nrhs: *const lapack_int, a: *mut c_double, lda: *const lapack_int,
                    af: *mut c_double, ldaf: *const lapack_int, equed: *mut c_char,
                    s: *mut c_double, b: *mut c_double, ldb: *const lapack_int,
                    x: *mut c_double, ldx: *const lapack_int, rcond: *mut c_double,
                    rpvgrw: *mut c_double, berr: *mut c_double, n_err_bnds: *const lapack_int,
                    err_bnds_norm: *mut c_double, err_bnds_comp: *mut c_double,
                    nparams: *const lapack_int, params: *mut c_double, work: *mut c_double,
                    iwork: *mut lapack_int, info: *mut lapack_int);
    pub fn sposvxx_(fact: *const c_char, uplo: *const c_char, n: *const lapack_int,
                    nrhs: *const lapack_int, a: *mut c_float, lda: *const lapack_int,
                    af: *mut c_float, ldaf: *const lapack_int, equed: *mut c_char,
                    s: *mut c_float, b: *mut c_float, ldb: *const lapack_int, x: *mut c_float,
                    ldx: *const lapack_int, rcond: *mut c_float, rpvgrw: *mut c_float,
                    berr: *mut c_float, n_err_bnds: *const lapack_int,
                    err_bnds_norm: *mut c_float, err_bnds_comp: *mut c_float,
                    nparams: *const lapack_int, params: *mut c_float, work: *mut c_float,
                    iwork: *mut lapack_int, info: *mut lapack_int);
    pub fn zposvxx_(fact: *const c_char, uplo: *const c_char, n: *const lapack_int,
                    nrhs: *const lapack_int, a: *mut lapack_complex_double,
                    lda: *const lapack_int, af: *mut lapack_complex_double,
                    ldaf: *const lapack_int, equed: *mut c_char, s: *mut c_double,
                    b: *mut lapack_complex_double, ldb: *const lapack_int,
                    x: *mut lapack_complex_double, ldx: *const lapack_int, rcond: *mut c_double,
                    rpvgrw: *mut c_double, berr: *mut c_double, n_err_bnds: *const lapack_int,
                    err_bnds_norm: *mut c_double, err_bnds_comp: *mut c_double,
                    nparams: *const lapack_int, params: *mut c_double,
                    work: *mut lapack_complex_double, rwork: *mut c_double,
                    info: *mut lapack_int);
    pub fn cposvxx_(fact: *const c_char, uplo: *const c_char, n: *const lapack_int,
                    nrhs: *const lapack_int, a: *mut lapack_complex_float,
                    lda: *const lapack_int, af: *mut lapack_complex_float,
                    ldaf: *const lapack_int, equed: *mut c_char, s: *mut c_float,
                    b: *mut lapack_complex_float, ldb: *const lapack_int,
                    x: *mut lapack_complex_float, ldx: *const lapack_int, rcond: *mut c_float,
                    rpvgrw: *mut c_float, berr: *mut c_float, n_err_bnds: *const lapack_int,
                    err_bnds_norm: *mut c_float, err_bnds_comp: *mut c_float,
                    nparams: *const lapack_int, params: *mut c_float,
                    work: *mut lapack_complex_float, rwork: *mut c_float, info: *mut lapack_int);

    pub fn sppsv_(uplo: *const c_char, n: *const lapack_int, nrhs: *const lapack_int,
                  ap: *mut c_float, b: *mut c_float, ldb: *const lapack_int,
                  info: *mut lapack_int);
    pub fn dppsv_(uplo: *const c_char, n: *const lapack_int, nrhs: *const lapack_int,
                  ap: *mut c_double, b: *mut c_double, ldb: *const lapack_int,
                  info: *mut lapack_int);
    pub fn cppsv_(uplo: *const c_char, n: *const lapack_int, nrhs: *const lapack_int,
                  ap: *mut lapack_complex_float, b: *mut lapack_complex_float,
                  ldb: *const lapack_int, info: *mut lapack_int);
    pub fn zppsv_(uplo: *const c_char, n: *const lapack_int, nrhs: *const lapack_int,
                  ap: *mut lapack_complex_double, b: *mut lapack_complex_double,
                  ldb: *const lapack_int, info: *mut lapack_int);

    pub fn sppsvx_(fact: *const c_char, uplo: *const c_char, n: *const lapack_int,
                   nrhs: *const lapack_int, ap: *mut c_float, afp: *mut c_float,
                   equed: *mut c_char, s: *mut c_float, b: *mut c_float, ldb: *const lapack_int,
                   x: *mut c_float, ldx: *const lapack_int, rcond: *mut c_float,
                   ferr: *mut c_float, berr: *mut c_float, work: *mut c_float,
                   iwork: *mut lapack_int, info: *mut lapack_int);
    pub fn dppsvx_(fact: *const c_char, uplo: *const c_char, n: *const lapack_int,
                   nrhs: *const lapack_int, ap: *mut c_double, afp: *mut c_double,
                   equed: *mut c_char, s: *mut c_double, b: *mut c_double,
                   ldb: *const lapack_int, x: *mut c_double, ldx: *const lapack_int,
                   rcond: *mut c_double, ferr: *mut c_double, berr: *mut c_double,
                   work: *mut c_double, iwork: *mut lapack_int, info: *mut lapack_int);
    pub fn cppsvx_(fact: *const c_char, uplo: *const c_char, n: *const lapack_int,
                   nrhs: *const lapack_int, ap: *mut lapack_complex_float,
                   afp: *mut lapack_complex_float, equed: *mut c_char, s: *mut c_float,
                   b: *mut lapack_complex_float, ldb: *const lapack_int,
                   x: *mut lapack_complex_float, ldx: *const lapack_int, rcond: *mut c_float,
                   ferr: *mut c_float, berr: *mut c_float, work: *mut lapack_complex_float,
                   rwork: *mut c_float, info: *mut lapack_int);
    pub fn zppsvx_(fact: *const c_char, uplo: *const c_char, n: *const lapack_int,
                   nrhs: *const lapack_int, ap: *mut lapack_complex_double,
                   afp: *mut lapack_complex_double, equed: *mut c_char, s: *mut c_double,
                   b: *mut lapack_complex_double, ldb: *const lapack_int,
                   x: *mut lapack_complex_double, ldx: *const lapack_int, rcond: *mut c_double,
                   ferr: *mut c_double, berr: *mut c_double, work: *mut lapack_complex_double,
                   rwork: *mut c_double, info: *mut lapack_int);

    pub fn spbsv_(uplo: *const c_char, n: *const lapack_int, kd: *const lapack_int,
                  nrhs: *const lapack_int, ab: *mut c_float, ldab: *const lapack_int,
                  b: *mut c_float, ldb: *const lapack_int, info: *mut lapack_int);
    pub fn dpbsv_(uplo: *const c_char, n: *const lapack_int, kd: *const lapack_int,
                  nrhs: *const lapack_int, ab: *mut c_double, ldab: *const lapack_int,
                  b: *mut c_double, ldb: *const lapack_int, info: *mut lapack_int);
    pub fn cpbsv_(uplo: *const c_char, n: *const lapack_int, kd: *const lapack_int,
                  nrhs: *const lapack_int, ab: *mut lapack_complex_float,
                  ldab: *const lapack_int, b: *mut lapack_complex_float, ldb: *const lapack_int,
                  info: *mut lapack_int);
    pub fn zpbsv_(uplo: *const c_char, n: *const lapack_int, kd: *const lapack_int,
                  nrhs: *const lapack_int, ab: *mut lapack_complex_double,
                  ldab: *const lapack_int, b: *mut lapack_complex_double,
                  ldb: *const lapack_int, info: *mut lapack_int);

    pub fn spbsvx_(fact: *const c_char, uplo: *const c_char, n: *const lapack_int,
                   kd: *const lapack_int, nrhs: *const lapack_int, ab: *mut c_float,
                   ldab: *const lapack_int, afb: *mut c_float, ldafb: *const lapack_int,
                   equed: *mut c_char, s: *mut c_float, b: *mut c_float, ldb: *const lapack_int,
                   x: *mut c_float, ldx: *const lapack_int, rcond: *mut c_float,
                   ferr: *mut c_float, berr: *mut c_float, work: *mut c_float,
                   iwork: *mut lapack_int, info: *mut lapack_int);
    pub fn dpbsvx_(fact: *const c_char, uplo: *const c_char, n: *const lapack_int,
                   kd: *const lapack_int, nrhs: *const lapack_int, ab: *mut c_double,
                   ldab: *const lapack_int, afb: *mut c_double, ldafb: *const lapack_int,
                   equed: *mut c_char, s: *mut c_double, b: *mut c_double,
                   ldb: *const lapack_int, x: *mut c_double, ldx: *const lapack_int,
                   rcond: *mut c_double, ferr: *mut c_double, berr: *mut c_double,
                   work: *mut c_double, iwork: *mut lapack_int, info: *mut lapack_int);
    pub fn cpbsvx_(fact: *const c_char, uplo: *const c_char, n: *const lapack_int,
                   kd: *const lapack_int, nrhs: *const lapack_int,
                   ab: *mut lapack_complex_float, ldab: *const lapack_int,
                   afb: *mut lapack_complex_float, ldafb: *const lapack_int, equed: *mut c_char,
                   s: *mut c_float, b: *mut lapack_complex_float, ldb: *const lapack_int,
                   x: *mut lapack_complex_float, ldx: *const lapack_int, rcond: *mut c_float,
                   ferr: *mut c_float, berr: *mut c_float, work: *mut lapack_complex_float,
                   rwork: *mut c_float, info: *mut lapack_int);
    pub fn zpbsvx_(fact: *const c_char, uplo: *const c_char, n: *const lapack_int,
                   kd: *const lapack_int, nrhs: *const lapack_int,
                   ab: *mut lapack_complex_double, ldab: *const lapack_int,
                   afb: *mut lapack_complex_double, ldafb: *const lapack_int,
                   equed: *mut c_char, s: *mut c_double, b: *mut lapack_complex_double,
                   ldb: *const lapack_int, x: *mut lapack_complex_double,
                   ldx: *const lapack_int, rcond: *mut c_double, ferr: *mut c_double,
                   berr: *mut c_double, work: *mut lapack_complex_double, rwork: *mut c_double,
                   info: *mut lapack_int);

    pub fn sptsv_(n: *const lapack_int, nrhs: *const lapack_int, d: *mut c_float,
                  e: *mut c_float, b: *mut c_float, ldb: *const lapack_int,
                  info: *mut lapack_int);
    pub fn dptsv_(n: *const lapack_int, nrhs: *const lapack_int, d: *mut c_double,
                  e: *mut c_double, b: *mut c_double, ldb: *const lapack_int,
                  info: *mut lapack_int);
    pub fn cptsv_(n: *const lapack_int, nrhs: *const lapack_int, d: *mut c_float,
                  e: *mut lapack_complex_float, b: *mut lapack_complex_float,
                  ldb: *const lapack_int, info: *mut lapack_int);
    pub fn zptsv_(n: *const lapack_int, nrhs: *const lapack_int, d: *mut c_double,
                  e: *mut lapack_complex_double, b: *mut lapack_complex_double,
                  ldb: *const lapack_int, info: *mut lapack_int);

    pub fn sptsvx_(fact: *const c_char, n: *const lapack_int, nrhs: *const lapack_int,
                   d: *const c_float, e: *const c_float, df: *mut c_float, ef: *mut c_float,
                   b: *const c_float, ldb: *const lapack_int, x: *mut c_float,
                   ldx: *const lapack_int, rcond: *mut c_float, ferr: *mut c_float,
                   berr: *mut c_float, work: *mut c_float, info: *mut lapack_int);
    pub fn dptsvx_(fact: *const c_char, n: *const lapack_int, nrhs: *const lapack_int,
                   d: *const c_double, e: *const c_double, df: *mut c_double, ef: *mut c_double,
                   b: *const c_double, ldb: *const lapack_int, x: *mut c_double,
                   ldx: *const lapack_int, rcond: *mut c_double, ferr: *mut c_double,
                   berr: *mut c_double, work: *mut c_double, info: *mut lapack_int);
    pub fn cptsvx_(fact: *const c_char, n: *const lapack_int, nrhs: *const lapack_int,
                   d: *const c_float, e: *const lapack_complex_float, df: *mut c_float,
                   ef: *mut lapack_complex_float, b: *const lapack_complex_float,
                   ldb: *const lapack_int, x: *mut lapack_complex_float, ldx: *const lapack_int,
                   rcond: *mut c_float, ferr: *mut c_float, berr: *mut c_float,
                   work: *mut lapack_complex_float, rwork: *mut c_float, info: *mut lapack_int);
    pub fn zptsvx_(fact: *const c_char, n: *const lapack_int, nrhs: *const lapack_int,
                   d: *const c_double, e: *const lapack_complex_double, df: *mut c_double,
                   ef: *mut lapack_complex_double, b: *const lapack_complex_double,
                   ldb: *const lapack_int, x: *mut lapack_complex_double,
                   ldx: *const lapack_int, rcond: *mut c_double, ferr: *mut c_double,
                   berr: *mut c_double, work: *mut lapack_complex_double, rwork: *mut c_double,
                   info: *mut lapack_int);

    pub fn ssysv_(uplo: *const c_char, n: *const lapack_int, nrhs: *const lapack_int,
                  a: *mut c_float, lda: *const lapack_int, ipiv: *mut lapack_int,
                  b: *mut c_float, ldb: *const lapack_int, work: *mut c_float,
                  lwork: *const lapack_int, info: *mut lapack_int);
    pub fn dsysv_(uplo: *const c_char, n: *const lapack_int, nrhs: *const lapack_int,
                  a: *mut c_double, lda: *const lapack_int, ipiv: *mut lapack_int,
                  b: *mut c_double, ldb: *const lapack_int, work: *mut c_double,
                  lwork: *const lapack_int, info: *mut lapack_int);
    pub fn csysv_(uplo: *const c_char, n: *const lapack_int, nrhs: *const lapack_int,
                  a: *mut lapack_complex_float, lda: *const lapack_int, ipiv: *mut lapack_int,
                  b: *mut lapack_complex_float, ldb: *const lapack_int,
                  work: *mut lapack_complex_float, lwork: *const lapack_int,
                  info: *mut lapack_int);
    pub fn zsysv_(uplo: *const c_char, n: *const lapack_int, nrhs: *const lapack_int,
                  a: *mut lapack_complex_double, lda: *const lapack_int, ipiv: *mut lapack_int,
                  b: *mut lapack_complex_double, ldb: *const lapack_int,
                  work: *mut lapack_complex_double, lwork: *const lapack_int,
                  info: *mut lapack_int);

    pub fn ssysvx_(fact: *const c_char, uplo: *const c_char, n: *const lapack_int,
                   nrhs: *const lapack_int, a: *const c_float, lda: *const lapack_int,
                   af: *mut c_float, ldaf: *const lapack_int, ipiv: *mut lapack_int,
                   b: *const c_float, ldb: *const lapack_int, x: *mut c_float,
                   ldx: *const lapack_int, rcond: *mut c_float, ferr: *mut c_float,
                   berr: *mut c_float, work: *mut c_float, lwork: *const lapack_int,
                   iwork: *mut lapack_int, info: *mut lapack_int);
    pub fn dsysvx_(fact: *const c_char, uplo: *const c_char, n: *const lapack_int,
                   nrhs: *const lapack_int, a: *const c_double, lda: *const lapack_int,
                   af: *mut c_double, ldaf: *const lapack_int, ipiv: *mut lapack_int,
                   b: *const c_double, ldb: *const lapack_int, x: *mut c_double,
                   ldx: *const lapack_int, rcond: *mut c_double, ferr: *mut c_double,
                   berr: *mut c_double, work: *mut c_double, lwork: *const lapack_int,
                   iwork: *mut lapack_int, info: *mut lapack_int);
    pub fn csysvx_(fact: *const c_char, uplo: *const c_char, n: *const lapack_int,
                   nrhs: *const lapack_int, a: *const lapack_complex_float,
                   lda: *const lapack_int, af: *mut lapack_complex_float,
                   ldaf: *const lapack_int, ipiv: *mut lapack_int,
                   b: *const lapack_complex_float, ldb: *const lapack_int,
                   x: *mut lapack_complex_float, ldx: *const lapack_int, rcond: *mut c_float,
                   ferr: *mut c_float, berr: *mut c_float, work: *mut lapack_complex_float,
                   lwork: *const lapack_int, rwork: *mut c_float, info: *mut lapack_int);
    pub fn zsysvx_(fact: *const c_char, uplo: *const c_char, n: *const lapack_int,
                   nrhs: *const lapack_int, a: *const lapack_complex_double,
                   lda: *const lapack_int, af: *mut lapack_complex_double,
                   ldaf: *const lapack_int, ipiv: *mut lapack_int,
                   b: *const lapack_complex_double, ldb: *const lapack_int,
                   x: *mut lapack_complex_double, ldx: *const lapack_int, rcond: *mut c_double,
                   ferr: *mut c_double, berr: *mut c_double, work: *mut lapack_complex_double,
                   lwork: *const lapack_int, rwork: *mut c_double, info: *mut lapack_int);

    pub fn dsysvxx_(fact: *const c_char, uplo: *const c_char, n: *const lapack_int,
                    nrhs: *const lapack_int, a: *mut c_double, lda: *const lapack_int,
                    af: *mut c_double, ldaf: *const lapack_int, ipiv: *mut lapack_int,
                    equed: *mut c_char, s: *mut c_double, b: *mut c_double,
                    ldb: *const lapack_int, x: *mut c_double, ldx: *const lapack_int,
                    rcond: *mut c_double, rpvgrw: *mut c_double, berr: *mut c_double,
                    n_err_bnds: *const lapack_int, err_bnds_norm: *mut c_double,
                    err_bnds_comp: *mut c_double, nparams: *const lapack_int,
                    params: *mut c_double, work: *mut c_double, iwork: *mut lapack_int,
                    info: *mut lapack_int);
    pub fn ssysvxx_(fact: *const c_char, uplo: *const c_char, n: *const lapack_int,
                    nrhs: *const lapack_int, a: *mut c_float, lda: *const lapack_int,
                    af: *mut c_float, ldaf: *const lapack_int, ipiv: *mut lapack_int,
                    equed: *mut c_char, s: *mut c_float, b: *mut c_float,
                    ldb: *const lapack_int, x: *mut c_float, ldx: *const lapack_int,
                    rcond: *mut c_float, rpvgrw: *mut c_float, berr: *mut c_float,
                    n_err_bnds: *const lapack_int, err_bnds_norm: *mut c_float,
                    err_bnds_comp: *mut c_float, nparams: *const lapack_int,
                    params: *mut c_float, work: *mut c_float, iwork: *mut lapack_int,
                    info: *mut lapack_int);
    pub fn zsysvxx_(fact: *const c_char, uplo: *const c_char, n: *const lapack_int,
                    nrhs: *const lapack_int, a: *mut lapack_complex_double,
                    lda: *const lapack_int, af: *mut lapack_complex_double,
                    ldaf: *const lapack_int, ipiv: *mut lapack_int, equed: *mut c_char,
                    s: *mut c_double, b: *mut lapack_complex_double, ldb: *const lapack_int,
                    x: *mut lapack_complex_double, ldx: *const lapack_int, rcond: *mut c_double,
                    rpvgrw: *mut c_double, berr: *mut c_double, n_err_bnds: *const lapack_int,
                    err_bnds_norm: *mut c_double, err_bnds_comp: *mut c_double,
                    nparams: *const lapack_int, params: *mut c_double,
                    work: *mut lapack_complex_double, rwork: *mut c_double,
                    info: *mut lapack_int);
    pub fn csysvxx_(fact: *const c_char, uplo: *const c_char, n: *const lapack_int,
                    nrhs: *const lapack_int, a: *mut lapack_complex_float,
                    lda: *const lapack_int, af: *mut lapack_complex_float,
                    ldaf: *const lapack_int, ipiv: *mut lapack_int, equed: *mut c_char,
                    s: *mut c_float, b: *mut lapack_complex_float, ldb: *const lapack_int,
                    x: *mut lapack_complex_float, ldx: *const lapack_int, rcond: *mut c_float,
                    rpvgrw: *mut c_float, berr: *mut c_float, n_err_bnds: *const lapack_int,
                    err_bnds_norm: *mut c_float, err_bnds_comp: *mut c_float,
                    nparams: *const lapack_int, params: *mut c_float,
                    work: *mut lapack_complex_float, rwork: *mut c_float, info: *mut lapack_int);

    pub fn chesv_(uplo: *const c_char, n: *const lapack_int, nrhs: *const lapack_int,
                  a: *mut lapack_complex_float, lda: *const lapack_int, ipiv: *mut lapack_int,
                  b: *mut lapack_complex_float, ldb: *const lapack_int,
                  work: *mut lapack_complex_float, lwork: *const lapack_int,
                  info: *mut lapack_int);
    pub fn zhesv_(uplo: *const c_char, n: *const lapack_int, nrhs: *const lapack_int,
                  a: *mut lapack_complex_double, lda: *const lapack_int, ipiv: *mut lapack_int,
                  b: *mut lapack_complex_double, ldb: *const lapack_int,
                  work: *mut lapack_complex_double, lwork: *const lapack_int,
                  info: *mut lapack_int);

    pub fn chesvx_(fact: *const c_char, uplo: *const c_char, n: *const lapack_int,
                   nrhs: *const lapack_int, a: *const lapack_complex_float,
                   lda: *const lapack_int, af: *mut lapack_complex_float,
                   ldaf: *const lapack_int, ipiv: *mut lapack_int,
                   b: *const lapack_complex_float, ldb: *const lapack_int,
                   x: *mut lapack_complex_float, ldx: *const lapack_int, rcond: *mut c_float,
                   ferr: *mut c_float, berr: *mut c_float, work: *mut lapack_complex_float,
                   lwork: *const lapack_int, rwork: *mut c_float, info: *mut lapack_int);
    pub fn zhesvx_(fact: *const c_char, uplo: *const c_char, n: *const lapack_int,
                   nrhs: *const lapack_int, a: *const lapack_complex_double,
                   lda: *const lapack_int, af: *mut lapack_complex_double,
                   ldaf: *const lapack_int, ipiv: *mut lapack_int,
                   b: *const lapack_complex_double, ldb: *const lapack_int,
                   x: *mut lapack_complex_double, ldx: *const lapack_int, rcond: *mut c_double,
                   ferr: *mut c_double, berr: *mut c_double, work: *mut lapack_complex_double,
                   lwork: *const lapack_int, rwork: *mut c_double, info: *mut lapack_int);

    pub fn zhesvxx_(fact: *const c_char, uplo: *const c_char, n: *const lapack_int,
                    nrhs: *const lapack_int, a: *mut lapack_complex_double,
                    lda: *const lapack_int, af: *mut lapack_complex_double,
                    ldaf: *const lapack_int, ipiv: *mut lapack_int, equed: *mut c_char,
                    s: *mut c_double, b: *mut lapack_complex_double, ldb: *const lapack_int,
                    x: *mut lapack_complex_double, ldx: *const lapack_int, rcond: *mut c_double,
                    rpvgrw: *mut c_double, berr: *mut c_double, n_err_bnds: *const lapack_int,
                    err_bnds_norm: *mut c_double, err_bnds_comp: *mut c_double,
                    nparams: *const lapack_int, params: *mut c_double,
                    work: *mut lapack_complex_double, rwork: *mut c_double,
                    info: *mut lapack_int);
    pub fn chesvxx_(fact: *const c_char, uplo: *const c_char, n: *const lapack_int,
                    nrhs: *const lapack_int, a: *mut lapack_complex_float,
                    lda: *const lapack_int, af: *mut lapack_complex_float,
                    ldaf: *const lapack_int, ipiv: *mut lapack_int, equed: *mut c_char,
                    s: *mut c_float, b: *mut lapack_complex_float, ldb: *const lapack_int,
                    x: *mut lapack_complex_float, ldx: *const lapack_int, rcond: *mut c_float,
                    rpvgrw: *mut c_float, berr: *mut c_float, n_err_bnds: *const lapack_int,
                    err_bnds_norm: *mut c_float, err_bnds_comp: *mut c_float,
                    nparams: *const lapack_int, params: *mut c_float,
                    work: *mut lapack_complex_float, rwork: *mut c_float, info: *mut lapack_int);

    pub fn sspsv_(uplo: *const c_char, n: *const lapack_int, nrhs: *const lapack_int,
                  ap: *mut c_float, ipiv: *mut lapack_int, b: *mut c_float,
                  ldb: *const lapack_int, info: *mut lapack_int);
    pub fn dspsv_(uplo: *const c_char, n: *const lapack_int, nrhs: *const lapack_int,
                  ap: *mut c_double, ipiv: *mut lapack_int, b: *mut c_double,
                  ldb: *const lapack_int, info: *mut lapack_int);
    pub fn cspsv_(uplo: *const c_char, n: *const lapack_int, nrhs: *const lapack_int,
                  ap: *mut lapack_complex_float, ipiv: *mut lapack_int,
                  b: *mut lapack_complex_float, ldb: *const lapack_int, info: *mut lapack_int);
    pub fn zspsv_(uplo: *const c_char, n: *const lapack_int, nrhs: *const lapack_int,
                  ap: *mut lapack_complex_double, ipiv: *mut lapack_int,
                  b: *mut lapack_complex_double, ldb: *const lapack_int, info: *mut lapack_int);

    pub fn sspsvx_(fact: *const c_char, uplo: *const c_char, n: *const lapack_int,
                   nrhs: *const lapack_int, ap: *const c_float, afp: *mut c_float,
                   ipiv: *mut lapack_int, b: *const c_float, ldb: *const lapack_int,
                   x: *mut c_float, ldx: *const lapack_int, rcond: *mut c_float,
                   ferr: *mut c_float, berr: *mut c_float, work: *mut c_float,
                   iwork: *mut lapack_int, info: *mut lapack_int);
    pub fn dspsvx_(fact: *const c_char, uplo: *const c_char, n: *const lapack_int,
                   nrhs: *const lapack_int, ap: *const c_double, afp: *mut c_double,
                   ipiv: *mut lapack_int, b: *const c_double, ldb: *const lapack_int,
                   x: *mut c_double, ldx: *const lapack_int, rcond: *mut c_double,
                   ferr: *mut c_double, berr: *mut c_double, work: *mut c_double,
                   iwork: *mut lapack_int, info: *mut lapack_int);
    pub fn cspsvx_(fact: *const c_char, uplo: *const c_char, n: *const lapack_int,
                   nrhs: *const lapack_int, ap: *const lapack_complex_float,
                   afp: *mut lapack_complex_float, ipiv: *mut lapack_int,
                   b: *const lapack_complex_float, ldb: *const lapack_int,
                   x: *mut lapack_complex_float, ldx: *const lapack_int, rcond: *mut c_float,
                   ferr: *mut c_float, berr: *mut c_float, work: *mut lapack_complex_float,
                   rwork: *mut c_float, info: *mut lapack_int);
    pub fn zspsvx_(fact: *const c_char, uplo: *const c_char, n: *const lapack_int,
                   nrhs: *const lapack_int, ap: *const lapack_complex_double,
                   afp: *mut lapack_complex_double, ipiv: *mut lapack_int,
                   b: *const lapack_complex_double, ldb: *const lapack_int,
                   x: *mut lapack_complex_double, ldx: *const lapack_int, rcond: *mut c_double,
                   ferr: *mut c_double, berr: *mut c_double, work: *mut lapack_complex_double,
                   rwork: *mut c_double, info: *mut lapack_int);

    pub fn chpsv_(uplo: *const c_char, n: *const lapack_int, nrhs: *const lapack_int,
                  ap: *mut lapack_complex_float, ipiv: *mut lapack_int,
                  b: *mut lapack_complex_float, ldb: *const lapack_int, info: *mut lapack_int);
    pub fn zhpsv_(uplo: *const c_char, n: *const lapack_int, nrhs: *const lapack_int,
                  ap: *mut lapack_complex_double, ipiv: *mut lapack_int,
                  b: *mut lapack_complex_double, ldb: *const lapack_int, info: *mut lapack_int);

    pub fn chpsvx_(fact: *const c_char, uplo: *const c_char, n: *const lapack_int,
                   nrhs: *const lapack_int, ap: *const lapack_complex_float,
                   afp: *mut lapack_complex_float, ipiv: *mut lapack_int,
                   b: *const lapack_complex_float, ldb: *const lapack_int,
                   x: *mut lapack_complex_float, ldx: *const lapack_int, rcond: *mut c_float,
                   ferr: *mut c_float, berr: *mut c_float, work: *mut lapack_complex_float,
                   rwork: *mut c_float, info: *mut lapack_int);
    pub fn zhpsvx_(fact: *const c_char, uplo: *const c_char, n: *const lapack_int,
                   nrhs: *const lapack_int, ap: *const lapack_complex_double,
                   afp: *mut lapack_complex_double, ipiv: *mut lapack_int,
                   b: *const lapack_complex_double, ldb: *const lapack_int,
                   x: *mut lapack_complex_double, ldx: *const lapack_int, rcond: *mut c_double,
                   ferr: *mut c_double, berr: *mut c_double, work: *mut lapack_complex_double,
                   rwork: *mut c_double, info: *mut lapack_int);

    pub fn sgeqrf_(m: *const lapack_int, n: *const lapack_int, a: *mut c_float,
                   lda: *const lapack_int, tau: *mut c_float, work: *mut c_float,
                   lwork: *const lapack_int, info: *mut lapack_int);
    pub fn dgeqrf_(m: *const lapack_int, n: *const lapack_int, a: *mut c_double,
                   lda: *const lapack_int, tau: *mut c_double, work: *mut c_double,
                   lwork: *const lapack_int, info: *mut lapack_int);
    pub fn cgeqrf_(m: *const lapack_int, n: *const lapack_int, a: *mut lapack_complex_float,
                   lda: *const lapack_int, tau: *mut lapack_complex_float,
                   work: *mut lapack_complex_float, lwork: *const lapack_int,
                   info: *mut lapack_int);
    pub fn zgeqrf_(m: *const lapack_int, n: *const lapack_int, a: *mut lapack_complex_double,
                   lda: *const lapack_int, tau: *mut lapack_complex_double,
                   work: *mut lapack_complex_double, lwork: *const lapack_int,
                   info: *mut lapack_int);

    pub fn sgeqpf_(m: *const lapack_int, n: *const lapack_int, a: *mut c_float,
                   lda: *const lapack_int, jpvt: *mut lapack_int, tau: *mut c_float,
                   work: *mut c_float, info: *mut lapack_int);
    pub fn dgeqpf_(m: *const lapack_int, n: *const lapack_int, a: *mut c_double,
                   lda: *const lapack_int, jpvt: *mut lapack_int, tau: *mut c_double,
                   work: *mut c_double, info: *mut lapack_int);
    pub fn cgeqpf_(m: *const lapack_int, n: *const lapack_int, a: *mut lapack_complex_float,
                   lda: *const lapack_int, jpvt: *mut lapack_int,
                   tau: *mut lapack_complex_float, work: *mut lapack_complex_float,
                   rwork: *mut c_float, info: *mut lapack_int);
    pub fn zgeqpf_(m: *const lapack_int, n: *const lapack_int, a: *mut lapack_complex_double,
                   lda: *const lapack_int, jpvt: *mut lapack_int,
                   tau: *mut lapack_complex_double, work: *mut lapack_complex_double,
                   rwork: *mut c_double, info: *mut lapack_int);

    pub fn sgeqp3_(m: *const lapack_int, n: *const lapack_int, a: *mut c_float,
                   lda: *const lapack_int, jpvt: *mut lapack_int, tau: *mut c_float,
                   work: *mut c_float, lwork: *const lapack_int, info: *mut lapack_int);
    pub fn dgeqp3_(m: *const lapack_int, n: *const lapack_int, a: *mut c_double,
                   lda: *const lapack_int, jpvt: *mut lapack_int, tau: *mut c_double,
                   work: *mut c_double, lwork: *const lapack_int, info: *mut lapack_int);
    pub fn cgeqp3_(m: *const lapack_int, n: *const lapack_int, a: *mut lapack_complex_float,
                   lda: *const lapack_int, jpvt: *mut lapack_int,
                   tau: *mut lapack_complex_float, work: *mut lapack_complex_float,
                   lwork: *const lapack_int, rwork: *mut c_float, info: *mut lapack_int);
    pub fn zgeqp3_(m: *const lapack_int, n: *const lapack_int, a: *mut lapack_complex_double,
                   lda: *const lapack_int, jpvt: *mut lapack_int,
                   tau: *mut lapack_complex_double, work: *mut lapack_complex_double,
                   lwork: *const lapack_int, rwork: *mut c_double, info: *mut lapack_int);

    pub fn sorgqr_(m: *const lapack_int, n: *const lapack_int, k: *const lapack_int,
                   a: *mut c_float, lda: *const lapack_int, tau: *const c_float,
                   work: *mut c_float, lwork: *const lapack_int, info: *mut lapack_int);
    pub fn dorgqr_(m: *const lapack_int, n: *const lapack_int, k: *const lapack_int,
                   a: *mut c_double, lda: *const lapack_int, tau: *const c_double,
                   work: *mut c_double, lwork: *const lapack_int, info: *mut lapack_int);
    pub fn sormqr_(side: *const c_char, trans: *const c_char, m: *const lapack_int,
                   n: *const lapack_int, k: *const lapack_int, a: *const c_float,
                   lda: *const lapack_int, tau: *const c_float, c: *mut c_float,
                   ldc: *const lapack_int, work: *mut c_float, lwork: *const lapack_int,
                   info: *mut lapack_int);
    pub fn dormqr_(side: *const c_char, trans: *const c_char, m: *const lapack_int,
                   n: *const lapack_int, k: *const lapack_int, a: *const c_double,
                   lda: *const lapack_int, tau: *const c_double, c: *mut c_double,
                   ldc: *const lapack_int, work: *mut c_double, lwork: *const lapack_int,
                   info: *mut lapack_int);
    pub fn cungqr_(m: *const lapack_int, n: *const lapack_int, k: *const lapack_int,
                   a: *mut lapack_complex_float, lda: *const lapack_int,
                   tau: *const lapack_complex_float, work: *mut lapack_complex_float,
                   lwork: *const lapack_int, info: *mut lapack_int);
    pub fn zungqr_(m: *const lapack_int, n: *const lapack_int, k: *const lapack_int,
                   a: *mut lapack_complex_double, lda: *const lapack_int,
                   tau: *const lapack_complex_double, work: *mut lapack_complex_double,
                   lwork: *const lapack_int, info: *mut lapack_int);

    pub fn cunmqr_(side: *const c_char, trans: *const c_char, m: *const lapack_int,
                   n: *const lapack_int, k: *const lapack_int, a: *const lapack_complex_float,
                   lda: *const lapack_int, tau: *const lapack_complex_float,
                   c: *mut lapack_complex_float, ldc: *const lapack_int,
                   work: *mut lapack_complex_float, lwork: *const lapack_int,
                   info: *mut lapack_int);
    pub fn zunmqr_(side: *const c_char, trans: *const c_char, m: *const lapack_int,
                   n: *const lapack_int, k: *const lapack_int, a: *const lapack_complex_double,
                   lda: *const lapack_int, tau: *const lapack_complex_double,
                   c: *mut lapack_complex_double, ldc: *const lapack_int,
                   work: *mut lapack_complex_double, lwork: *const lapack_int,
                   info: *mut lapack_int);

    pub fn sgelqf_(m: *const lapack_int, n: *const lapack_int, a: *mut c_float,
                   lda: *const lapack_int, tau: *mut c_float, work: *mut c_float,
                   lwork: *const lapack_int, info: *mut lapack_int);
    pub fn dgelqf_(m: *const lapack_int, n: *const lapack_int, a: *mut c_double,
                   lda: *const lapack_int, tau: *mut c_double, work: *mut c_double,
                   lwork: *const lapack_int, info: *mut lapack_int);
    pub fn cgelqf_(m: *const lapack_int, n: *const lapack_int, a: *mut lapack_complex_float,
                   lda: *const lapack_int, tau: *mut lapack_complex_float,
                   work: *mut lapack_complex_float, lwork: *const lapack_int,
                   info: *mut lapack_int);
    pub fn zgelqf_(m: *const lapack_int, n: *const lapack_int, a: *mut lapack_complex_double,
                   lda: *const lapack_int, tau: *mut lapack_complex_double,
                   work: *mut lapack_complex_double, lwork: *const lapack_int,
                   info: *mut lapack_int);

    pub fn sorglq_(m: *const lapack_int, n: *const lapack_int, k: *const lapack_int,
                   a: *mut c_float, lda: *const lapack_int, tau: *const c_float,
                   work: *mut c_float, lwork: *const lapack_int, info: *mut lapack_int);
    pub fn dorglq_(m: *const lapack_int, n: *const lapack_int, k: *const lapack_int,
                   a: *mut c_double, lda: *const lapack_int, tau: *const c_double,
                   work: *mut c_double, lwork: *const lapack_int, info: *mut lapack_int);
    pub fn sormlq_(side: *const c_char, trans: *const c_char, m: *const lapack_int,
                   n: *const lapack_int, k: *const lapack_int, a: *const c_float,
                   lda: *const lapack_int, tau: *const c_float, c: *mut c_float,
                   ldc: *const lapack_int, work: *mut c_float, lwork: *const lapack_int,
                   info: *mut lapack_int);
    pub fn dormlq_(side: *const c_char, trans: *const c_char, m: *const lapack_int,
                   n: *const lapack_int, k: *const lapack_int, a: *const c_double,
                   lda: *const lapack_int, tau: *const c_double, c: *mut c_double,
                   ldc: *const lapack_int, work: *mut c_double, lwork: *const lapack_int,
                   info: *mut lapack_int);
    pub fn cunglq_(m: *const lapack_int, n: *const lapack_int, k: *const lapack_int,
                   a: *mut lapack_complex_float, lda: *const lapack_int,
                   tau: *const lapack_complex_float, work: *mut lapack_complex_float,
                   lwork: *const lapack_int, info: *mut lapack_int);
    pub fn zunglq_(m: *const lapack_int, n: *const lapack_int, k: *const lapack_int,
                   a: *mut lapack_complex_double, lda: *const lapack_int,
                   tau: *const lapack_complex_double, work: *mut lapack_complex_double,
                   lwork: *const lapack_int, info: *mut lapack_int);

    pub fn cunmlq_(side: *const c_char, trans: *const c_char, m: *const lapack_int,
                   n: *const lapack_int, k: *const lapack_int, a: *const lapack_complex_float,
                   lda: *const lapack_int, tau: *const lapack_complex_float,
                   c: *mut lapack_complex_float, ldc: *const lapack_int,
                   work: *mut lapack_complex_float, lwork: *const lapack_int,
                   info: *mut lapack_int);
    pub fn zunmlq_(side: *const c_char, trans: *const c_char, m: *const lapack_int,
                   n: *const lapack_int, k: *const lapack_int, a: *const lapack_complex_double,
                   lda: *const lapack_int, tau: *const lapack_complex_double,
                   c: *mut lapack_complex_double, ldc: *const lapack_int,
                   work: *mut lapack_complex_double, lwork: *const lapack_int,
                   info: *mut lapack_int);

    pub fn sgeqlf_(m: *const lapack_int, n: *const lapack_int, a: *mut c_float,
                   lda: *const lapack_int, tau: *mut c_float, work: *mut c_float,
                   lwork: *const lapack_int, info: *mut lapack_int);
    pub fn dgeqlf_(m: *const lapack_int, n: *const lapack_int, a: *mut c_double,
                   lda: *const lapack_int, tau: *mut c_double, work: *mut c_double,
                   lwork: *const lapack_int, info: *mut lapack_int);
    pub fn cgeqlf_(m: *const lapack_int, n: *const lapack_int, a: *mut lapack_complex_float,
                   lda: *const lapack_int, tau: *mut lapack_complex_float,
                   work: *mut lapack_complex_float, lwork: *const lapack_int,
                   info: *mut lapack_int);
    pub fn zgeqlf_(m: *const lapack_int, n: *const lapack_int, a: *mut lapack_complex_double,
                   lda: *const lapack_int, tau: *mut lapack_complex_double,
                   work: *mut lapack_complex_double, lwork: *const lapack_int,
                   info: *mut lapack_int);

    pub fn sorgql_(m: *const lapack_int, n: *const lapack_int, k: *const lapack_int,
                   a: *mut c_float, lda: *const lapack_int, tau: *const c_float,
                   work: *mut c_float, lwork: *const lapack_int, info: *mut lapack_int);
    pub fn dorgql_(m: *const lapack_int, n: *const lapack_int, k: *const lapack_int,
                   a: *mut c_double, lda: *const lapack_int, tau: *const c_double,
                   work: *mut c_double, lwork: *const lapack_int, info: *mut lapack_int);
    pub fn cungql_(m: *const lapack_int, n: *const lapack_int, k: *const lapack_int,
                   a: *mut lapack_complex_float, lda: *const lapack_int,
                   tau: *const lapack_complex_float, work: *mut lapack_complex_float,
                   lwork: *const lapack_int, info: *mut lapack_int);
    pub fn zungql_(m: *const lapack_int, n: *const lapack_int, k: *const lapack_int,
                   a: *mut lapack_complex_double, lda: *const lapack_int,
                   tau: *const lapack_complex_double, work: *mut lapack_complex_double,
                   lwork: *const lapack_int, info: *mut lapack_int);

    pub fn sormql_(side: *const c_char, trans: *const c_char, m: *const lapack_int,
                   n: *const lapack_int, k: *const lapack_int, a: *const c_float,
                   lda: *const lapack_int, tau: *const c_float, c: *mut c_float,
                   ldc: *const lapack_int, work: *mut c_float, lwork: *const lapack_int,
                   info: *mut lapack_int);
    pub fn dormql_(side: *const c_char, trans: *const c_char, m: *const lapack_int,
                   n: *const lapack_int, k: *const lapack_int, a: *const c_double,
                   lda: *const lapack_int, tau: *const c_double, c: *mut c_double,
                   ldc: *const lapack_int, work: *mut c_double, lwork: *const lapack_int,
                   info: *mut lapack_int);
    pub fn cunmql_(side: *const c_char, trans: *const c_char, m: *const lapack_int,
                   n: *const lapack_int, k: *const lapack_int, a: *const lapack_complex_float,
                   lda: *const lapack_int, tau: *const lapack_complex_float,
                   c: *mut lapack_complex_float, ldc: *const lapack_int,
                   work: *mut lapack_complex_float, lwork: *const lapack_int,
                   info: *mut lapack_int);
    pub fn zunmql_(side: *const c_char, trans: *const c_char, m: *const lapack_int,
                   n: *const lapack_int, k: *const lapack_int, a: *const lapack_complex_double,
                   lda: *const lapack_int, tau: *const lapack_complex_double,
                   c: *mut lapack_complex_double, ldc: *const lapack_int,
                   work: *mut lapack_complex_double, lwork: *const lapack_int,
                   info: *mut lapack_int);

    pub fn sgerqf_(m: *const lapack_int, n: *const lapack_int, a: *mut c_float,
                   lda: *const lapack_int, tau: *mut c_float, work: *mut c_float,
                   lwork: *const lapack_int, info: *mut lapack_int);
    pub fn dgerqf_(m: *const lapack_int, n: *const lapack_int, a: *mut c_double,
                   lda: *const lapack_int, tau: *mut c_double, work: *mut c_double,
                   lwork: *const lapack_int, info: *mut lapack_int);
    pub fn cgerqf_(m: *const lapack_int, n: *const lapack_int, a: *mut lapack_complex_float,
                   lda: *const lapack_int, tau: *mut lapack_complex_float,
                   work: *mut lapack_complex_float, lwork: *const lapack_int,
                   info: *mut lapack_int);
    pub fn zgerqf_(m: *const lapack_int, n: *const lapack_int, a: *mut lapack_complex_double,
                   lda: *const lapack_int, tau: *mut lapack_complex_double,
                   work: *mut lapack_complex_double, lwork: *const lapack_int,
                   info: *mut lapack_int);

    pub fn sorgrq_(m: *const lapack_int, n: *const lapack_int, k: *const lapack_int,
                   a: *mut c_float, lda: *const lapack_int, tau: *const c_float,
                   work: *mut c_float, lwork: *const lapack_int, info: *mut lapack_int);
    pub fn dorgrq_(m: *const lapack_int, n: *const lapack_int, k: *const lapack_int,
                   a: *mut c_double, lda: *const lapack_int, tau: *const c_double,
                   work: *mut c_double, lwork: *const lapack_int, info: *mut lapack_int);
    pub fn cungrq_(m: *const lapack_int, n: *const lapack_int, k: *const lapack_int,
                   a: *mut lapack_complex_float, lda: *const lapack_int,
                   tau: *const lapack_complex_float, work: *mut lapack_complex_float,
                   lwork: *const lapack_int, info: *mut lapack_int);
    pub fn zungrq_(m: *const lapack_int, n: *const lapack_int, k: *const lapack_int,
                   a: *mut lapack_complex_double, lda: *const lapack_int,
                   tau: *const lapack_complex_double, work: *mut lapack_complex_double,
                   lwork: *const lapack_int, info: *mut lapack_int);

    pub fn sormrq_(side: *const c_char, trans: *const c_char, m: *const lapack_int,
                   n: *const lapack_int, k: *const lapack_int, a: *const c_float,
                   lda: *const lapack_int, tau: *const c_float, c: *mut c_float,
                   ldc: *const lapack_int, work: *mut c_float, lwork: *const lapack_int,
                   info: *mut lapack_int);
    pub fn dormrq_(side: *const c_char, trans: *const c_char, m: *const lapack_int,
                   n: *const lapack_int, k: *const lapack_int, a: *const c_double,
                   lda: *const lapack_int, tau: *const c_double, c: *mut c_double,
                   ldc: *const lapack_int, work: *mut c_double, lwork: *const lapack_int,
                   info: *mut lapack_int);
    pub fn cunmrq_(side: *const c_char, trans: *const c_char, m: *const lapack_int,
                   n: *const lapack_int, k: *const lapack_int, a: *const lapack_complex_float,
                   lda: *const lapack_int, tau: *const lapack_complex_float,
                   c: *mut lapack_complex_float, ldc: *const lapack_int,
                   work: *mut lapack_complex_float, lwork: *const lapack_int,
                   info: *mut lapack_int);
    pub fn zunmrq_(side: *const c_char, trans: *const c_char, m: *const lapack_int,
                   n: *const lapack_int, k: *const lapack_int, a: *const lapack_complex_double,
                   lda: *const lapack_int, tau: *const lapack_complex_double,
                   c: *mut lapack_complex_double, ldc: *const lapack_int,
                   work: *mut lapack_complex_double, lwork: *const lapack_int,
                   info: *mut lapack_int);

    pub fn stzrzf_(m: *const lapack_int, n: *const lapack_int, a: *mut c_float,
                   lda: *const lapack_int, tau: *mut c_float, work: *mut c_float,
                   lwork: *const lapack_int, info: *mut lapack_int);
    pub fn dtzrzf_(m: *const lapack_int, n: *const lapack_int, a: *mut c_double,
                   lda: *const lapack_int, tau: *mut c_double, work: *mut c_double,
                   lwork: *const lapack_int, info: *mut lapack_int);
    pub fn ctzrzf_(m: *const lapack_int, n: *const lapack_int, a: *mut lapack_complex_float,
                   lda: *const lapack_int, tau: *mut lapack_complex_float,
                   work: *mut lapack_complex_float, lwork: *const lapack_int,
                   info: *mut lapack_int);
    pub fn ztzrzf_(m: *const lapack_int, n: *const lapack_int, a: *mut lapack_complex_double,
                   lda: *const lapack_int, tau: *mut lapack_complex_double,
                   work: *mut lapack_complex_double, lwork: *const lapack_int,
                   info: *mut lapack_int);

    pub fn sormrz_(side: *const c_char, trans: *const c_char, m: *const lapack_int,
                   n: *const lapack_int, k: *const lapack_int, l: *const lapack_int,
                   a: *const c_float, lda: *const lapack_int, tau: *const c_float,
                   c: *mut c_float, ldc: *const lapack_int, work: *mut c_float,
                   lwork: *const lapack_int, info: *mut lapack_int);
    pub fn dormrz_(side: *const c_char, trans: *const c_char, m: *const lapack_int,
                   n: *const lapack_int, k: *const lapack_int, l: *const lapack_int,
                   a: *const c_double, lda: *const lapack_int, tau: *const c_double,
                   c: *mut c_double, ldc: *const lapack_int, work: *mut c_double,
                   lwork: *const lapack_int, info: *mut lapack_int);
    pub fn cunmrz_(side: *const c_char, trans: *const c_char, m: *const lapack_int,
                   n: *const lapack_int, k: *const lapack_int, l: *const lapack_int,
                   a: *const lapack_complex_float, lda: *const lapack_int,
                   tau: *const lapack_complex_float, c: *mut lapack_complex_float,
                   ldc: *const lapack_int, work: *mut lapack_complex_float,
                   lwork: *const lapack_int, info: *mut lapack_int);
    pub fn zunmrz_(side: *const c_char, trans: *const c_char, m: *const lapack_int,
                   n: *const lapack_int, k: *const lapack_int, l: *const lapack_int,
                   a: *const lapack_complex_double, lda: *const lapack_int,
                   tau: *const lapack_complex_double, c: *mut lapack_complex_double,
                   ldc: *const lapack_int, work: *mut lapack_complex_double,
                   lwork: *const lapack_int, info: *mut lapack_int);

    pub fn sggqrf_(n: *const lapack_int, m: *const lapack_int, p: *const lapack_int,
                   a: *mut c_float, lda: *const lapack_int, taua: *mut c_float, b: *mut c_float,
                   ldb: *const lapack_int, taub: *mut c_float, work: *mut c_float,
                   lwork: *const lapack_int, info: *mut lapack_int);
    pub fn dggqrf_(n: *const lapack_int, m: *const lapack_int, p: *const lapack_int,
                   a: *mut c_double, lda: *const lapack_int, taua: *mut c_double,
                   b: *mut c_double, ldb: *const lapack_int, taub: *mut c_double,
                   work: *mut c_double, lwork: *const lapack_int, info: *mut lapack_int);
    pub fn cggqrf_(n: *const lapack_int, m: *const lapack_int, p: *const lapack_int,
                   a: *mut lapack_complex_float, lda: *const lapack_int,
                   taua: *mut lapack_complex_float, b: *mut lapack_complex_float,
                   ldb: *const lapack_int, taub: *mut lapack_complex_float,
                   work: *mut lapack_complex_float, lwork: *const lapack_int,
                   info: *mut lapack_int);
    pub fn zggqrf_(n: *const lapack_int, m: *const lapack_int, p: *const lapack_int,
                   a: *mut lapack_complex_double, lda: *const lapack_int,
                   taua: *mut lapack_complex_double, b: *mut lapack_complex_double,
                   ldb: *const lapack_int, taub: *mut lapack_complex_double,
                   work: *mut lapack_complex_double, lwork: *const lapack_int,
                   info: *mut lapack_int);

    pub fn sggrqf_(m: *const lapack_int, p: *const lapack_int, n: *const lapack_int,
                   a: *mut c_float, lda: *const lapack_int, taua: *mut c_float, b: *mut c_float,
                   ldb: *const lapack_int, taub: *mut c_float, work: *mut c_float,
                   lwork: *const lapack_int, info: *mut lapack_int);
    pub fn dggrqf_(m: *const lapack_int, p: *const lapack_int, n: *const lapack_int,
                   a: *mut c_double, lda: *const lapack_int, taua: *mut c_double,
                   b: *mut c_double, ldb: *const lapack_int, taub: *mut c_double,
                   work: *mut c_double, lwork: *const lapack_int, info: *mut lapack_int);
    pub fn cggrqf_(m: *const lapack_int, p: *const lapack_int, n: *const lapack_int,
                   a: *mut lapack_complex_float, lda: *const lapack_int,
                   taua: *mut lapack_complex_float, b: *mut lapack_complex_float,
                   ldb: *const lapack_int, taub: *mut lapack_complex_float,
                   work: *mut lapack_complex_float, lwork: *const lapack_int,
                   info: *mut lapack_int);
    pub fn zggrqf_(m: *const lapack_int, p: *const lapack_int, n: *const lapack_int,
                   a: *mut lapack_complex_double, lda: *const lapack_int,
                   taua: *mut lapack_complex_double, b: *mut lapack_complex_double,
                   ldb: *const lapack_int, taub: *mut lapack_complex_double,
                   work: *mut lapack_complex_double, lwork: *const lapack_int,
                   info: *mut lapack_int);

    pub fn sgebrd_(m: *const lapack_int, n: *const lapack_int, a: *mut c_float,
                   lda: *const lapack_int, d: *mut c_float, e: *mut c_float, tauq: *mut c_float,
                   taup: *mut c_float, work: *mut c_float, lwork: *const lapack_int,
                   info: *mut lapack_int);
    pub fn dgebrd_(m: *const lapack_int, n: *const lapack_int, a: *mut c_double,
                   lda: *const lapack_int, d: *mut c_double, e: *mut c_double,
                   tauq: *mut c_double, taup: *mut c_double, work: *mut c_double,
                   lwork: *const lapack_int, info: *mut lapack_int);
    pub fn cgebrd_(m: *const lapack_int, n: *const lapack_int, a: *mut lapack_complex_float,
                   lda: *const lapack_int, d: *mut c_float, e: *mut c_float,
                   tauq: *mut lapack_complex_float, taup: *mut lapack_complex_float,
                   work: *mut lapack_complex_float, lwork: *const lapack_int,
                   info: *mut lapack_int);
    pub fn zgebrd_(m: *const lapack_int, n: *const lapack_int, a: *mut lapack_complex_double,
                   lda: *const lapack_int, d: *mut c_double, e: *mut c_double,
                   tauq: *mut lapack_complex_double, taup: *mut lapack_complex_double,
                   work: *mut lapack_complex_double, lwork: *const lapack_int,
                   info: *mut lapack_int);

    pub fn sgbbrd_(vect: *const c_char, m: *const lapack_int, n: *const lapack_int,
                   ncc: *const lapack_int, kl: *const lapack_int, ku: *const lapack_int,
                   ab: *mut c_float, ldab: *const lapack_int, d: *mut c_float, e: *mut c_float,
                   q: *mut c_float, ldq: *const lapack_int, pt: *mut c_float,
                   ldpt: *const lapack_int, c: *mut c_float, ldc: *const lapack_int,
                   work: *mut c_float, info: *mut lapack_int);
    pub fn dgbbrd_(vect: *const c_char, m: *const lapack_int, n: *const lapack_int,
                   ncc: *const lapack_int, kl: *const lapack_int, ku: *const lapack_int,
                   ab: *mut c_double, ldab: *const lapack_int, d: *mut c_double,
                   e: *mut c_double, q: *mut c_double, ldq: *const lapack_int,
                   pt: *mut c_double, ldpt: *const lapack_int, c: *mut c_double,
                   ldc: *const lapack_int, work: *mut c_double, info: *mut lapack_int);
    pub fn cgbbrd_(vect: *const c_char, m: *const lapack_int, n: *const lapack_int,
                   ncc: *const lapack_int, kl: *const lapack_int, ku: *const lapack_int,
                   ab: *mut lapack_complex_float, ldab: *const lapack_int, d: *mut c_float,
                   e: *mut c_float, q: *mut lapack_complex_float, ldq: *const lapack_int,
                   pt: *mut lapack_complex_float, ldpt: *const lapack_int,
                   c: *mut lapack_complex_float, ldc: *const lapack_int,
                   work: *mut lapack_complex_float, rwork: *mut c_float, info: *mut lapack_int);
    pub fn zgbbrd_(vect: *const c_char, m: *const lapack_int, n: *const lapack_int,
                   ncc: *const lapack_int, kl: *const lapack_int, ku: *const lapack_int,
                   ab: *mut lapack_complex_double, ldab: *const lapack_int, d: *mut c_double,
                   e: *mut c_double, q: *mut lapack_complex_double, ldq: *const lapack_int,
                   pt: *mut lapack_complex_double, ldpt: *const lapack_int,
                   c: *mut lapack_complex_double, ldc: *const lapack_int,
                   work: *mut lapack_complex_double, rwork: *mut c_double, info: *mut lapack_int);

    pub fn sorgbr_(vect: *const c_char, m: *const lapack_int, n: *const lapack_int,
                   k: *const lapack_int, a: *mut c_float, lda: *const lapack_int,
                   tau: *const c_float, work: *mut c_float, lwork: *const lapack_int,
                   info: *mut lapack_int);
    pub fn dorgbr_(vect: *const c_char, m: *const lapack_int, n: *const lapack_int,
                   k: *const lapack_int, a: *mut c_double, lda: *const lapack_int,
                   tau: *const c_double, work: *mut c_double, lwork: *const lapack_int,
                   info: *mut lapack_int);
    pub fn sormbr_(vect: *const c_char, side: *const c_char, trans: *const c_char,
                   m: *const lapack_int, n: *const lapack_int, k: *const lapack_int,
                   a: *const c_float, lda: *const lapack_int, tau: *const c_float,
                   c: *mut c_float, ldc: *const lapack_int, work: *mut c_float,
                   lwork: *const lapack_int, info: *mut lapack_int);
    pub fn dormbr_(vect: *const c_char, side: *const c_char, trans: *const c_char,
                   m: *const lapack_int, n: *const lapack_int, k: *const lapack_int,
                   a: *const c_double, lda: *const lapack_int, tau: *const c_double,
                   c: *mut c_double, ldc: *const lapack_int, work: *mut c_double,
                   lwork: *const lapack_int, info: *mut lapack_int);
    pub fn cungbr_(vect: *const c_char, m: *const lapack_int, n: *const lapack_int,
                   k: *const lapack_int, a: *mut lapack_complex_float, lda: *const lapack_int,
                   tau: *const lapack_complex_float, work: *mut lapack_complex_float,
                   lwork: *const lapack_int, info: *mut lapack_int);
    pub fn zungbr_(vect: *const c_char, m: *const lapack_int, n: *const lapack_int,
                   k: *const lapack_int, a: *mut lapack_complex_double, lda: *const lapack_int,
                   tau: *const lapack_complex_double, work: *mut lapack_complex_double,
                   lwork: *const lapack_int, info: *mut lapack_int);

    pub fn cunmbr_(vect: *const c_char, side: *const c_char, trans: *const c_char,
                   m: *const lapack_int, n: *const lapack_int, k: *const lapack_int,
                   a: *const lapack_complex_float, lda: *const lapack_int,
                   tau: *const lapack_complex_float, c: *mut lapack_complex_float,
                   ldc: *const lapack_int, work: *mut lapack_complex_float,
                   lwork: *const lapack_int, info: *mut lapack_int);
    pub fn zunmbr_(vect: *const c_char, side: *const c_char, trans: *const c_char,
                   m: *const lapack_int, n: *const lapack_int, k: *const lapack_int,
                   a: *const lapack_complex_double, lda: *const lapack_int,
                   tau: *const lapack_complex_double, c: *mut lapack_complex_double,
                   ldc: *const lapack_int, work: *mut lapack_complex_double,
                   lwork: *const lapack_int, info: *mut lapack_int);

    pub fn sbdsqr_(uplo: *const c_char, n: *const lapack_int, ncvt: *const lapack_int,
                   nru: *const lapack_int, ncc: *const lapack_int, d: *mut c_float,
                   e: *mut c_float, vt: *mut c_float, ldvt: *const lapack_int, u: *mut c_float,
                   ldu: *const lapack_int, c: *mut c_float, ldc: *const lapack_int,
                   work: *mut c_float, info: *mut lapack_int);
    pub fn dbdsqr_(uplo: *const c_char, n: *const lapack_int, ncvt: *const lapack_int,
                   nru: *const lapack_int, ncc: *const lapack_int, d: *mut c_double,
                   e: *mut c_double, vt: *mut c_double, ldvt: *const lapack_int,
                   u: *mut c_double, ldu: *const lapack_int, c: *mut c_double,
                   ldc: *const lapack_int, work: *mut c_double, info: *mut lapack_int);
    pub fn cbdsqr_(uplo: *const c_char, n: *const lapack_int, ncvt: *const lapack_int,
                   nru: *const lapack_int, ncc: *const lapack_int, d: *mut c_float,
                   e: *mut c_float, vt: *mut lapack_complex_float, ldvt: *const lapack_int,
                   u: *mut lapack_complex_float, ldu: *const lapack_int,
                   c: *mut lapack_complex_float, ldc: *const lapack_int, work: *mut c_float,
                   info: *mut lapack_int);
    pub fn zbdsqr_(uplo: *const c_char, n: *const lapack_int, ncvt: *const lapack_int,
                   nru: *const lapack_int, ncc: *const lapack_int, d: *mut c_double,
                   e: *mut c_double, vt: *mut lapack_complex_double, ldvt: *const lapack_int,
                   u: *mut lapack_complex_double, ldu: *const lapack_int,
                   c: *mut lapack_complex_double, ldc: *const lapack_int, work: *mut c_double,
                   info: *mut lapack_int);

    pub fn sbdsdc_(uplo: *const c_char, compq: *const c_char, n: *const lapack_int,
                   d: *mut c_float, e: *mut c_float, u: *mut c_float, ldu: *const lapack_int,
                   vt: *mut c_float, ldvt: *const lapack_int, q: *mut c_float,
                   iq: *mut lapack_int, work: *mut c_float, iwork: *mut lapack_int,
                   info: *mut lapack_int);
    pub fn dbdsdc_(uplo: *const c_char, compq: *const c_char, n: *const lapack_int,
                   d: *mut c_double, e: *mut c_double, u: *mut c_double, ldu: *const lapack_int,
                   vt: *mut c_double, ldvt: *const lapack_int, q: *mut c_double,
                   iq: *mut lapack_int, work: *mut c_double, iwork: *mut lapack_int,
                   info: *mut lapack_int);

    pub fn sbdsvdx_(uplo: *const c_char, jobz: *const c_char, range: *const c_char,
                    n: *const lapack_int, d: *const c_float, e: *const c_float,
                    vl: *const c_float, vu: *const c_float, il: *const lapack_int,
                    iu: *const lapack_int, ns: *mut lapack_int, s: *mut c_float,
                    z: *mut c_float, ldz: *const lapack_int, work: *mut c_float,
                    iwork: *mut lapack_int, info: *mut lapack_int);
    pub fn dbdsvdx_(uplo: *const c_char, jobz: *const c_char, range: *const c_char,
                    n: *const lapack_int, d: *const c_double, e: *const c_double,
                    vl: *const c_double, vu: *const c_double, il: *const lapack_int,
                    iu: *const lapack_int, ns: *mut lapack_int, s: *mut c_double,
                    z: *mut c_double, ldz: *const lapack_int, work: *mut c_double,
                    iwork: *mut lapack_int, info: *mut lapack_int);

    pub fn ssytrd_(uplo: *const c_char, n: *const lapack_int, a: *mut c_float,
                   lda: *const lapack_int, d: *mut c_float, e: *mut c_float, tau: *mut c_float,
                   work: *mut c_float, lwork: *const lapack_int, info: *mut lapack_int);
    pub fn dsytrd_(uplo: *const c_char, n: *const lapack_int, a: *mut c_double,
                   lda: *const lapack_int, d: *mut c_double, e: *mut c_double,
                   tau: *mut c_double, work: *mut c_double, lwork: *const lapack_int,
                   info: *mut lapack_int);

    pub fn sorgtr_(uplo: *const c_char, n: *const lapack_int, a: *mut c_float,
                   lda: *const lapack_int, tau: *const c_float, work: *mut c_float,
                   lwork: *const lapack_int, info: *mut lapack_int);
    pub fn dorgtr_(uplo: *const c_char, n: *const lapack_int, a: *mut c_double,
                   lda: *const lapack_int, tau: *const c_double, work: *mut c_double,
                   lwork: *const lapack_int, info: *mut lapack_int);

    pub fn sormtr_(side: *const c_char, uplo: *const c_char, trans: *const c_char,
                   m: *const lapack_int, n: *const lapack_int, a: *const c_float,
                   lda: *const lapack_int, tau: *const c_float, c: *mut c_float,
                   ldc: *const lapack_int, work: *mut c_float, lwork: *const lapack_int,
                   info: *mut lapack_int);
    pub fn dormtr_(side: *const c_char, uplo: *const c_char, trans: *const c_char,
                   m: *const lapack_int, n: *const lapack_int, a: *const c_double,
                   lda: *const lapack_int, tau: *const c_double, c: *mut c_double,
                   ldc: *const lapack_int, work: *mut c_double, lwork: *const lapack_int,
                   info: *mut lapack_int);

    pub fn chetrd_(uplo: *const c_char, n: *const lapack_int, a: *mut lapack_complex_float,
                   lda: *const lapack_int, d: *mut c_float, e: *mut c_float,
                   tau: *mut lapack_complex_float, work: *mut lapack_complex_float,
                   lwork: *const lapack_int, info: *mut lapack_int);
    pub fn zhetrd_(uplo: *const c_char, n: *const lapack_int, a: *mut lapack_complex_double,
                   lda: *const lapack_int, d: *mut c_double, e: *mut c_double,
                   tau: *mut lapack_complex_double, work: *mut lapack_complex_double,
                   lwork: *const lapack_int, info: *mut lapack_int);

    pub fn cungtr_(uplo: *const c_char, n: *const lapack_int, a: *mut lapack_complex_float,
                   lda: *const lapack_int, tau: *const lapack_complex_float,
                   work: *mut lapack_complex_float, lwork: *const lapack_int,
                   info: *mut lapack_int);
    pub fn zungtr_(uplo: *const c_char, n: *const lapack_int, a: *mut lapack_complex_double,
                   lda: *const lapack_int, tau: *const lapack_complex_double,
                   work: *mut lapack_complex_double, lwork: *const lapack_int,
                   info: *mut lapack_int);

    pub fn cunmtr_(side: *const c_char, uplo: *const c_char, trans: *const c_char,
                   m: *const lapack_int, n: *const lapack_int, a: *const lapack_complex_float,
                   lda: *const lapack_int, tau: *const lapack_complex_float,
                   c: *mut lapack_complex_float, ldc: *const lapack_int,
                   work: *mut lapack_complex_float, lwork: *const lapack_int,
                   info: *mut lapack_int);
    pub fn zunmtr_(side: *const c_char, uplo: *const c_char, trans: *const c_char,
                   m: *const lapack_int, n: *const lapack_int, a: *const lapack_complex_double,
                   lda: *const lapack_int, tau: *const lapack_complex_double,
                   c: *mut lapack_complex_double, ldc: *const lapack_int,
                   work: *mut lapack_complex_double, lwork: *const lapack_int,
                   info: *mut lapack_int);

    pub fn ssptrd_(uplo: *const c_char, n: *const lapack_int, ap: *mut c_float, d: *mut c_float,
                   e: *mut c_float, tau: *mut c_float, info: *mut lapack_int);
    pub fn dsptrd_(uplo: *const c_char, n: *const lapack_int, ap: *mut c_double,
                   d: *mut c_double, e: *mut c_double, tau: *mut c_double, info: *mut lapack_int);

    pub fn sopgtr_(uplo: *const c_char, n: *const lapack_int, ap: *const c_float,
                   tau: *const c_float, q: *mut c_float, ldq: *const lapack_int,
                   work: *mut c_float, info: *mut lapack_int);
    pub fn dopgtr_(uplo: *const c_char, n: *const lapack_int, ap: *const c_double,
                   tau: *const c_double, q: *mut c_double, ldq: *const lapack_int,
                   work: *mut c_double, info: *mut lapack_int);

    pub fn sopmtr_(side: *const c_char, uplo: *const c_char, trans: *const c_char,
                   m: *const lapack_int, n: *const lapack_int, ap: *const c_float,
                   tau: *const c_float, c: *mut c_float, ldc: *const lapack_int,
                   work: *mut c_float, info: *mut lapack_int);
    pub fn dopmtr_(side: *const c_char, uplo: *const c_char, trans: *const c_char,
                   m: *const lapack_int, n: *const lapack_int, ap: *const c_double,
                   tau: *const c_double, c: *mut c_double, ldc: *const lapack_int,
                   work: *mut c_double, info: *mut lapack_int);

    pub fn chptrd_(uplo: *const c_char, n: *const lapack_int, ap: *mut lapack_complex_float,
                   d: *mut c_float, e: *mut c_float, tau: *mut lapack_complex_float,
                   info: *mut lapack_int);
    pub fn zhptrd_(uplo: *const c_char, n: *const lapack_int, ap: *mut lapack_complex_double,
                   d: *mut c_double, e: *mut c_double, tau: *mut lapack_complex_double,
                   info: *mut lapack_int);

    pub fn cupgtr_(uplo: *const c_char, n: *const lapack_int, ap: *const lapack_complex_float,
                   tau: *const lapack_complex_float, q: *mut lapack_complex_float,
                   ldq: *const lapack_int, work: *mut lapack_complex_float,
                   info: *mut lapack_int);
    pub fn zupgtr_(uplo: *const c_char, n: *const lapack_int, ap: *const lapack_complex_double,
                   tau: *const lapack_complex_double, q: *mut lapack_complex_double,
                   ldq: *const lapack_int, work: *mut lapack_complex_double,
                   info: *mut lapack_int);

    pub fn cupmtr_(side: *const c_char, uplo: *const c_char, trans: *const c_char,
                   m: *const lapack_int, n: *const lapack_int, ap: *const lapack_complex_float,
                   tau: *const lapack_complex_float, c: *mut lapack_complex_float,
                   ldc: *const lapack_int, work: *mut lapack_complex_float,
                   info: *mut lapack_int);
    pub fn zupmtr_(side: *const c_char, uplo: *const c_char, trans: *const c_char,
                   m: *const lapack_int, n: *const lapack_int, ap: *const lapack_complex_double,
                   tau: *const lapack_complex_double, c: *mut lapack_complex_double,
                   ldc: *const lapack_int, work: *mut lapack_complex_double,
                   info: *mut lapack_int);

    pub fn ssbtrd_(vect: *const c_char, uplo: *const c_char, n: *const lapack_int,
                   kd: *const lapack_int, ab: *mut c_float, ldab: *const lapack_int,
                   d: *mut c_float, e: *mut c_float, q: *mut c_float, ldq: *const lapack_int,
                   work: *mut c_float, info: *mut lapack_int);
    pub fn dsbtrd_(vect: *const c_char, uplo: *const c_char, n: *const lapack_int,
                   kd: *const lapack_int, ab: *mut c_double, ldab: *const lapack_int,
                   d: *mut c_double, e: *mut c_double, q: *mut c_double, ldq: *const lapack_int,
                   work: *mut c_double, info: *mut lapack_int);
    pub fn chbtrd_(vect: *const c_char, uplo: *const c_char, n: *const lapack_int,
                   kd: *const lapack_int, ab: *mut lapack_complex_float,
                   ldab: *const lapack_int, d: *mut c_float, e: *mut c_float,
                   q: *mut lapack_complex_float, ldq: *const lapack_int,
                   work: *mut lapack_complex_float, info: *mut lapack_int);
    pub fn zhbtrd_(vect: *const c_char, uplo: *const c_char, n: *const lapack_int,
                   kd: *const lapack_int, ab: *mut lapack_complex_double,
                   ldab: *const lapack_int, d: *mut c_double, e: *mut c_double,
                   q: *mut lapack_complex_double, ldq: *const lapack_int,
                   work: *mut lapack_complex_double, info: *mut lapack_int);

    pub fn ssterf_(n: *const lapack_int, d: *mut c_float, e: *mut c_float, info: *mut lapack_int);
    pub fn dsterf_(n: *const lapack_int, d: *mut c_double, e: *mut c_double,
                   info: *mut lapack_int);
    pub fn ssteqr_(compz: *const c_char, n: *const lapack_int, d: *mut c_float, e: *mut c_float,
                   z: *mut c_float, ldz: *const lapack_int, work: *mut c_float,
                   info: *mut lapack_int);
    pub fn dsteqr_(compz: *const c_char, n: *const lapack_int, d: *mut c_double,
                   e: *mut c_double, z: *mut c_double, ldz: *const lapack_int,
                   work: *mut c_double, info: *mut lapack_int);
    pub fn csteqr_(compz: *const c_char, n: *const lapack_int, d: *mut c_float, e: *mut c_float,
                   z: *mut lapack_complex_float, ldz: *const lapack_int, work: *mut c_float,
                   info: *mut lapack_int);
    pub fn zsteqr_(compz: *const c_char, n: *const lapack_int, d: *mut c_double,
                   e: *mut c_double, z: *mut lapack_complex_double, ldz: *const lapack_int,
                   work: *mut c_double, info: *mut lapack_int);

    pub fn sstemr_(jobz: *const c_char, range: *const c_char, n: *const lapack_int,
                   d: *mut c_float, e: *mut c_float, vl: *const c_float, vu: *const c_float,
                   il: *const lapack_int, iu: *const lapack_int, m: *mut lapack_int,
                   w: *mut c_float, z: *mut c_float, ldz: *const lapack_int,
                   nzc: *const lapack_int, isuppz: *mut lapack_int, tryrac: *mut lapack_logical,
                   work: *mut c_float, lwork: *const lapack_int, iwork: *mut lapack_int,
                   liwork: *const lapack_int, info: *mut lapack_int);
    pub fn dstemr_(jobz: *const c_char, range: *const c_char, n: *const lapack_int,
                   d: *mut c_double, e: *mut c_double, vl: *const c_double, vu: *const c_double,
                   il: *const lapack_int, iu: *const lapack_int, m: *mut lapack_int,
                   w: *mut c_double, z: *mut c_double, ldz: *const lapack_int,
                   nzc: *const lapack_int, isuppz: *mut lapack_int, tryrac: *mut lapack_logical,
                   work: *mut c_double, lwork: *const lapack_int, iwork: *mut lapack_int,
                   liwork: *const lapack_int, info: *mut lapack_int);
    pub fn cstemr_(jobz: *const c_char, range: *const c_char, n: *const lapack_int,
                   d: *mut c_float, e: *mut c_float, vl: *const c_float, vu: *const c_float,
                   il: *const lapack_int, iu: *const lapack_int, m: *mut lapack_int,
                   w: *mut c_float, z: *mut lapack_complex_float, ldz: *const lapack_int,
                   nzc: *const lapack_int, isuppz: *mut lapack_int, tryrac: *mut lapack_logical,
                   work: *mut c_float, lwork: *const lapack_int, iwork: *mut lapack_int,
                   liwork: *const lapack_int, info: *mut lapack_int);
    pub fn zstemr_(jobz: *const c_char, range: *const c_char, n: *const lapack_int,
                   d: *mut c_double, e: *mut c_double, vl: *const c_double, vu: *const c_double,
                   il: *const lapack_int, iu: *const lapack_int, m: *mut lapack_int,
                   w: *mut c_double, z: *mut lapack_complex_double, ldz: *const lapack_int,
                   nzc: *const lapack_int, isuppz: *mut lapack_int, tryrac: *mut lapack_logical,
                   work: *mut c_double, lwork: *const lapack_int, iwork: *mut lapack_int,
                   liwork: *const lapack_int, info: *mut lapack_int);

    pub fn sstedc_(compz: *const c_char, n: *const lapack_int, d: *mut c_float, e: *mut c_float,
                   z: *mut c_float, ldz: *const lapack_int, work: *mut c_float,
                   lwork: *const lapack_int, iwork: *mut lapack_int, liwork: *const lapack_int,
                   info: *mut lapack_int);
    pub fn dstedc_(compz: *const c_char, n: *const lapack_int, d: *mut c_double,
                   e: *mut c_double, z: *mut c_double, ldz: *const lapack_int,
                   work: *mut c_double, lwork: *const lapack_int, iwork: *mut lapack_int,
                   liwork: *const lapack_int, info: *mut lapack_int);
    pub fn cstedc_(compz: *const c_char, n: *const lapack_int, d: *mut c_float, e: *mut c_float,
                   z: *mut lapack_complex_float, ldz: *const lapack_int,
                   work: *mut lapack_complex_float, lwork: *const lapack_int,
                   rwork: *mut c_float, lrwork: *const lapack_int, iwork: *mut lapack_int,
                   liwork: *const lapack_int, info: *mut lapack_int);
    pub fn zstedc_(compz: *const c_char, n: *const lapack_int, d: *mut c_double,
                   e: *mut c_double, z: *mut lapack_complex_double, ldz: *const lapack_int,
                   work: *mut lapack_complex_double, lwork: *const lapack_int,
                   rwork: *mut c_double, lrwork: *const lapack_int, iwork: *mut lapack_int,
                   liwork: *const lapack_int, info: *mut lapack_int);

    pub fn sstegr_(jobz: *const c_char, range: *const c_char, n: *const lapack_int,
                   d: *mut c_float, e: *mut c_float, vl: *const c_float, vu: *const c_float,
                   il: *const lapack_int, iu: *const lapack_int, abstol: *const c_float,
                   m: *mut lapack_int, w: *mut c_float, z: *mut c_float, ldz: *const lapack_int,
                   isuppz: *mut lapack_int, work: *mut c_float, lwork: *const lapack_int,
                   iwork: *mut lapack_int, liwork: *const lapack_int, info: *mut lapack_int);
    pub fn dstegr_(jobz: *const c_char, range: *const c_char, n: *const lapack_int,
                   d: *mut c_double, e: *mut c_double, vl: *const c_double, vu: *const c_double,
                   il: *const lapack_int, iu: *const lapack_int, abstol: *const c_double,
                   m: *mut lapack_int, w: *mut c_double, z: *mut c_double,
                   ldz: *const lapack_int, isuppz: *mut lapack_int, work: *mut c_double,
                   lwork: *const lapack_int, iwork: *mut lapack_int, liwork: *const lapack_int,
                   info: *mut lapack_int);
    pub fn cstegr_(jobz: *const c_char, range: *const c_char, n: *const lapack_int,
                   d: *mut c_float, e: *mut c_float, vl: *const c_float, vu: *const c_float,
                   il: *const lapack_int, iu: *const lapack_int, abstol: *const c_float,
                   m: *mut lapack_int, w: *mut c_float, z: *mut lapack_complex_float,
                   ldz: *const lapack_int, isuppz: *mut lapack_int, work: *mut c_float,
                   lwork: *const lapack_int, iwork: *mut lapack_int, liwork: *const lapack_int,
                   info: *mut lapack_int);
    pub fn zstegr_(jobz: *const c_char, range: *const c_char, n: *const lapack_int,
                   d: *mut c_double, e: *mut c_double, vl: *const c_double, vu: *const c_double,
                   il: *const lapack_int, iu: *const lapack_int, abstol: *const c_double,
                   m: *mut lapack_int, w: *mut c_double, z: *mut lapack_complex_double,
                   ldz: *const lapack_int, isuppz: *mut lapack_int, work: *mut c_double,
                   lwork: *const lapack_int, iwork: *mut lapack_int, liwork: *const lapack_int,
                   info: *mut lapack_int);

    pub fn spteqr_(compz: *const c_char, n: *const lapack_int, d: *mut c_float, e: *mut c_float,
                   z: *mut c_float, ldz: *const lapack_int, work: *mut c_float,
                   info: *mut lapack_int);
    pub fn dpteqr_(compz: *const c_char, n: *const lapack_int, d: *mut c_double,
                   e: *mut c_double, z: *mut c_double, ldz: *const lapack_int,
                   work: *mut c_double, info: *mut lapack_int);
    pub fn cpteqr_(compz: *const c_char, n: *const lapack_int, d: *mut c_float, e: *mut c_float,
                   z: *mut lapack_complex_float, ldz: *const lapack_int, work: *mut c_float,
                   info: *mut lapack_int);
    pub fn zpteqr_(compz: *const c_char, n: *const lapack_int, d: *mut c_double,
                   e: *mut c_double, z: *mut lapack_complex_double, ldz: *const lapack_int,
                   work: *mut c_double, info: *mut lapack_int);

    pub fn sstebz_(range: *const c_char, order: *const c_char, n: *const lapack_int,
                   vl: *const c_float, vu: *const c_float, il: *const lapack_int,
                   iu: *const lapack_int, abstol: *const c_float, d: *const c_float,
                   e: *const c_float, m: *mut lapack_int, nsplit: *mut lapack_int,
                   w: *mut c_float, iblock: *mut lapack_int, isplit: *mut lapack_int,
                   work: *mut c_float, iwork: *mut lapack_int, info: *mut lapack_int);
    pub fn dstebz_(range: *const c_char, order: *const c_char, n: *const lapack_int,
                   vl: *const c_double, vu: *const c_double, il: *const lapack_int,
                   iu: *const lapack_int, abstol: *const c_double, d: *const c_double,
                   e: *const c_double, m: *mut lapack_int, nsplit: *mut lapack_int,
                   w: *mut c_double, iblock: *mut lapack_int, isplit: *mut lapack_int,
                   work: *mut c_double, iwork: *mut lapack_int, info: *mut lapack_int);

    pub fn sstein_(n: *const lapack_int, d: *const c_float, e: *const c_float,
                   m: *const lapack_int, w: *const c_float, iblock: *const lapack_int,
                   isplit: *const lapack_int, z: *mut c_float, ldz: *const lapack_int,
                   work: *mut c_float, iwork: *mut lapack_int, ifail: *mut lapack_int,
                   info: *mut lapack_int);
    pub fn dstein_(n: *const lapack_int, d: *const c_double, e: *const c_double,
                   m: *const lapack_int, w: *const c_double, iblock: *const lapack_int,
                   isplit: *const lapack_int, z: *mut c_double, ldz: *const lapack_int,
                   work: *mut c_double, iwork: *mut lapack_int, ifail: *mut lapack_int,
                   info: *mut lapack_int);
    pub fn cstein_(n: *const lapack_int, d: *const c_float, e: *const c_float,
                   m: *const lapack_int, w: *const c_float, iblock: *const lapack_int,
                   isplit: *const lapack_int, z: *mut lapack_complex_float,
                   ldz: *const lapack_int, work: *mut c_float, iwork: *mut lapack_int,
                   ifail: *mut lapack_int, info: *mut lapack_int);
    pub fn zstein_(n: *const lapack_int, d: *const c_double, e: *const c_double,
                   m: *const lapack_int, w: *const c_double, iblock: *const lapack_int,
                   isplit: *const lapack_int, z: *mut lapack_complex_double,
                   ldz: *const lapack_int, work: *mut c_double, iwork: *mut lapack_int,
                   ifail: *mut lapack_int, info: *mut lapack_int);

    pub fn sdisna_(job: *const c_char, m: *const lapack_int, n: *const lapack_int,
                   d: *const c_float, sep: *mut c_float, info: *mut lapack_int);
    pub fn ddisna_(job: *const c_char, m: *const lapack_int, n: *const lapack_int,
                   d: *const c_double, sep: *mut c_double, info: *mut lapack_int);

    pub fn ssygst_(itype: *const lapack_int, uplo: *const c_char, n: *const lapack_int,
                   a: *mut c_float, lda: *const lapack_int, b: *const c_float,
                   ldb: *const lapack_int, info: *mut lapack_int);
    pub fn dsygst_(itype: *const lapack_int, uplo: *const c_char, n: *const lapack_int,
                   a: *mut c_double, lda: *const lapack_int, b: *const c_double,
                   ldb: *const lapack_int, info: *mut lapack_int);
    pub fn chegst_(itype: *const lapack_int, uplo: *const c_char, n: *const lapack_int,
                   a: *mut lapack_complex_float, lda: *const lapack_int,
                   b: *const lapack_complex_float, ldb: *const lapack_int, info: *mut lapack_int);
    pub fn zhegst_(itype: *const lapack_int, uplo: *const c_char, n: *const lapack_int,
                   a: *mut lapack_complex_double, lda: *const lapack_int,
                   b: *const lapack_complex_double, ldb: *const lapack_int,
                   info: *mut lapack_int);

    pub fn sspgst_(itype: *const lapack_int, uplo: *const c_char, n: *const lapack_int,
                   ap: *mut c_float, bp: *const c_float, info: *mut lapack_int);
    pub fn dspgst_(itype: *const lapack_int, uplo: *const c_char, n: *const lapack_int,
                   ap: *mut c_double, bp: *const c_double, info: *mut lapack_int);
    pub fn chpgst_(itype: *const lapack_int, uplo: *const c_char, n: *const lapack_int,
                   ap: *mut lapack_complex_float, bp: *const lapack_complex_float,
                   info: *mut lapack_int);
    pub fn zhpgst_(itype: *const lapack_int, uplo: *const c_char, n: *const lapack_int,
                   ap: *mut lapack_complex_double, bp: *const lapack_complex_double,
                   info: *mut lapack_int);

    pub fn ssbgst_(vect: *const c_char, uplo: *const c_char, n: *const lapack_int,
                   ka: *const lapack_int, kb: *const lapack_int, ab: *mut c_float,
                   ldab: *const lapack_int, bb: *const c_float, ldbb: *const lapack_int,
                   x: *mut c_float, ldx: *const lapack_int, work: *mut c_float,
                   info: *mut lapack_int);
    pub fn dsbgst_(vect: *const c_char, uplo: *const c_char, n: *const lapack_int,
                   ka: *const lapack_int, kb: *const lapack_int, ab: *mut c_double,
                   ldab: *const lapack_int, bb: *const c_double, ldbb: *const lapack_int,
                   x: *mut c_double, ldx: *const lapack_int, work: *mut c_double,
                   info: *mut lapack_int);
    pub fn chbgst_(vect: *const c_char, uplo: *const c_char, n: *const lapack_int,
                   ka: *const lapack_int, kb: *const lapack_int, ab: *mut lapack_complex_float,
                   ldab: *const lapack_int, bb: *const lapack_complex_float,
                   ldbb: *const lapack_int, x: *mut lapack_complex_float,
                   ldx: *const lapack_int, work: *mut lapack_complex_float, rwork: *mut c_float,
                   info: *mut lapack_int);
    pub fn zhbgst_(vect: *const c_char, uplo: *const c_char, n: *const lapack_int,
                   ka: *const lapack_int, kb: *const lapack_int, ab: *mut lapack_complex_double,
                   ldab: *const lapack_int, bb: *const lapack_complex_double,
                   ldbb: *const lapack_int, x: *mut lapack_complex_double,
                   ldx: *const lapack_int, work: *mut lapack_complex_double,
                   rwork: *mut c_double, info: *mut lapack_int);

    pub fn spbstf_(uplo: *const c_char, n: *const lapack_int, kd: *const lapack_int,
                   ab: *mut c_float, ldab: *const lapack_int, info: *mut lapack_int);
    pub fn dpbstf_(uplo: *const c_char, n: *const lapack_int, kd: *const lapack_int,
                   ab: *mut c_double, ldab: *const lapack_int, info: *mut lapack_int);
    pub fn cpbstf_(uplo: *const c_char, n: *const lapack_int, kd: *const lapack_int,
                   ab: *mut lapack_complex_float, ldab: *const lapack_int, info: *mut lapack_int);
    pub fn zpbstf_(uplo: *const c_char, n: *const lapack_int, kd: *const lapack_int,
                   ab: *mut lapack_complex_double, ldab: *const lapack_int,
                   info: *mut lapack_int);

    pub fn sgehrd_(n: *const lapack_int, ilo: *const lapack_int, ihi: *const lapack_int,
                   a: *mut c_float, lda: *const lapack_int, tau: *mut c_float,
                   work: *mut c_float, lwork: *const lapack_int, info: *mut lapack_int);
    pub fn dgehrd_(n: *const lapack_int, ilo: *const lapack_int, ihi: *const lapack_int,
                   a: *mut c_double, lda: *const lapack_int, tau: *mut c_double,
                   work: *mut c_double, lwork: *const lapack_int, info: *mut lapack_int);
    pub fn cgehrd_(n: *const lapack_int, ilo: *const lapack_int, ihi: *const lapack_int,
                   a: *mut lapack_complex_float, lda: *const lapack_int,
                   tau: *mut lapack_complex_float, work: *mut lapack_complex_float,
                   lwork: *const lapack_int, info: *mut lapack_int);
    pub fn zgehrd_(n: *const lapack_int, ilo: *const lapack_int, ihi: *const lapack_int,
                   a: *mut lapack_complex_double, lda: *const lapack_int,
                   tau: *mut lapack_complex_double, work: *mut lapack_complex_double,
                   lwork: *const lapack_int, info: *mut lapack_int);

    pub fn sorghr_(n: *const lapack_int, ilo: *const lapack_int, ihi: *const lapack_int,
                   a: *mut c_float, lda: *const lapack_int, tau: *const c_float,
                   work: *mut c_float, lwork: *const lapack_int, info: *mut lapack_int);
    pub fn dorghr_(n: *const lapack_int, ilo: *const lapack_int, ihi: *const lapack_int,
                   a: *mut c_double, lda: *const lapack_int, tau: *const c_double,
                   work: *mut c_double, lwork: *const lapack_int, info: *mut lapack_int);

    pub fn sormhr_(side: *const c_char, trans: *const c_char, m: *const lapack_int,
                   n: *const lapack_int, ilo: *const lapack_int, ihi: *const lapack_int,
                   a: *const c_float, lda: *const lapack_int, tau: *const c_float,
                   c: *mut c_float, ldc: *const lapack_int, work: *mut c_float,
                   lwork: *const lapack_int, info: *mut lapack_int);
    pub fn dormhr_(side: *const c_char, trans: *const c_char, m: *const lapack_int,
                   n: *const lapack_int, ilo: *const lapack_int, ihi: *const lapack_int,
                   a: *const c_double, lda: *const lapack_int, tau: *const c_double,
                   c: *mut c_double, ldc: *const lapack_int, work: *mut c_double,
                   lwork: *const lapack_int, info: *mut lapack_int);

    pub fn cunghr_(n: *const lapack_int, ilo: *const lapack_int, ihi: *const lapack_int,
                   a: *mut lapack_complex_float, lda: *const lapack_int,
                   tau: *const lapack_complex_float, work: *mut lapack_complex_float,
                   lwork: *const lapack_int, info: *mut lapack_int);
    pub fn zunghr_(n: *const lapack_int, ilo: *const lapack_int, ihi: *const lapack_int,
                   a: *mut lapack_complex_double, lda: *const lapack_int,
                   tau: *const lapack_complex_double, work: *mut lapack_complex_double,
                   lwork: *const lapack_int, info: *mut lapack_int);

    pub fn cunmhr_(side: *const c_char, trans: *const c_char, m: *const lapack_int,
                   n: *const lapack_int, ilo: *const lapack_int, ihi: *const lapack_int,
                   a: *const lapack_complex_float, lda: *const lapack_int,
                   tau: *const lapack_complex_float, c: *mut lapack_complex_float,
                   ldc: *const lapack_int, work: *mut lapack_complex_float,
                   lwork: *const lapack_int, info: *mut lapack_int);
    pub fn zunmhr_(side: *const c_char, trans: *const c_char, m: *const lapack_int,
                   n: *const lapack_int, ilo: *const lapack_int, ihi: *const lapack_int,
                   a: *const lapack_complex_double, lda: *const lapack_int,
                   tau: *const lapack_complex_double, c: *mut lapack_complex_double,
                   ldc: *const lapack_int, work: *mut lapack_complex_double,
                   lwork: *const lapack_int, info: *mut lapack_int);

    pub fn sgebal_(job: *const c_char, n: *const lapack_int, a: *mut c_float,
                   lda: *const lapack_int, ilo: *mut lapack_int, ihi: *mut lapack_int,
                   scale: *mut c_float, info: *mut lapack_int);
    pub fn dgebal_(job: *const c_char, n: *const lapack_int, a: *mut c_double,
                   lda: *const lapack_int, ilo: *mut lapack_int, ihi: *mut lapack_int,
                   scale: *mut c_double, info: *mut lapack_int);
    pub fn cgebal_(job: *const c_char, n: *const lapack_int, a: *mut lapack_complex_float,
                   lda: *const lapack_int, ilo: *mut lapack_int, ihi: *mut lapack_int,
                   scale: *mut c_float, info: *mut lapack_int);
    pub fn zgebal_(job: *const c_char, n: *const lapack_int, a: *mut lapack_complex_double,
                   lda: *const lapack_int, ilo: *mut lapack_int, ihi: *mut lapack_int,
                   scale: *mut c_double, info: *mut lapack_int);

    pub fn sgebak_(job: *const c_char, side: *const c_char, n: *const lapack_int,
                   ilo: *const lapack_int, ihi: *const lapack_int, scale: *const c_float,
                   m: *const lapack_int, v: *mut c_float, ldv: *const lapack_int,
                   info: *mut lapack_int);
    pub fn dgebak_(job: *const c_char, side: *const c_char, n: *const lapack_int,
                   ilo: *const lapack_int, ihi: *const lapack_int, scale: *const c_double,
                   m: *const lapack_int, v: *mut c_double, ldv: *const lapack_int,
                   info: *mut lapack_int);
    pub fn cgebak_(job: *const c_char, side: *const c_char, n: *const lapack_int,
                   ilo: *const lapack_int, ihi: *const lapack_int, scale: *const c_float,
                   m: *const lapack_int, v: *mut lapack_complex_float, ldv: *const lapack_int,
                   info: *mut lapack_int);
    pub fn zgebak_(job: *const c_char, side: *const c_char, n: *const lapack_int,
                   ilo: *const lapack_int, ihi: *const lapack_int, scale: *const c_double,
                   m: *const lapack_int, v: *mut lapack_complex_double, ldv: *const lapack_int,
                   info: *mut lapack_int);

    pub fn shseqr_(job: *const c_char, compz: *const c_char, n: *const lapack_int,
                   ilo: *const lapack_int, ihi: *const lapack_int, h: *mut c_float,
                   ldh: *const lapack_int, wr: *mut c_float, wi: *mut c_float, z: *mut c_float,
                   ldz: *const lapack_int, work: *mut c_float, lwork: *const lapack_int,
                   info: *mut lapack_int);
    pub fn dhseqr_(job: *const c_char, compz: *const c_char, n: *const lapack_int,
                   ilo: *const lapack_int, ihi: *const lapack_int, h: *mut c_double,
                   ldh: *const lapack_int, wr: *mut c_double, wi: *mut c_double,
                   z: *mut c_double, ldz: *const lapack_int, work: *mut c_double,
                   lwork: *const lapack_int, info: *mut lapack_int);
    pub fn chseqr_(job: *const c_char, compz: *const c_char, n: *const lapack_int,
                   ilo: *const lapack_int, ihi: *const lapack_int, h: *mut lapack_complex_float,
                   ldh: *const lapack_int, w: *mut lapack_complex_float,
                   z: *mut lapack_complex_float, ldz: *const lapack_int,
                   work: *mut lapack_complex_float, lwork: *const lapack_int,
                   info: *mut lapack_int);
    pub fn zhseqr_(job: *const c_char, compz: *const c_char, n: *const lapack_int,
                   ilo: *const lapack_int, ihi: *const lapack_int,
                   h: *mut lapack_complex_double, ldh: *const lapack_int,
                   w: *mut lapack_complex_double, z: *mut lapack_complex_double,
                   ldz: *const lapack_int, work: *mut lapack_complex_double,
                   lwork: *const lapack_int, info: *mut lapack_int);

    pub fn shsein_(job: *const c_char, eigsrc: *const c_char, initv: *const c_char,
                   select: *mut lapack_logical, n: *const lapack_int, h: *const c_float,
                   ldh: *const lapack_int, wr: *mut c_float, wi: *const c_float,
                   vl: *mut c_float, ldvl: *const lapack_int, vr: *mut c_float,
                   ldvr: *const lapack_int, mm: *const lapack_int, m: *mut lapack_int,
                   work: *mut c_float, ifaill: *mut lapack_int, ifailr: *mut lapack_int,
                   info: *mut lapack_int);
    pub fn dhsein_(job: *const c_char, eigsrc: *const c_char, initv: *const c_char,
                   select: *mut lapack_logical, n: *const lapack_int, h: *const c_double,
                   ldh: *const lapack_int, wr: *mut c_double, wi: *const c_double,
                   vl: *mut c_double, ldvl: *const lapack_int, vr: *mut c_double,
                   ldvr: *const lapack_int, mm: *const lapack_int, m: *mut lapack_int,
                   work: *mut c_double, ifaill: *mut lapack_int, ifailr: *mut lapack_int,
                   info: *mut lapack_int);
    pub fn chsein_(job: *const c_char, eigsrc: *const c_char, initv: *const c_char,
                   select: *const lapack_logical, n: *const lapack_int,
                   h: *const lapack_complex_float, ldh: *const lapack_int,
                   w: *mut lapack_complex_float, vl: *mut lapack_complex_float,
                   ldvl: *const lapack_int, vr: *mut lapack_complex_float,
                   ldvr: *const lapack_int, mm: *const lapack_int, m: *mut lapack_int,
                   work: *mut lapack_complex_float, rwork: *mut c_float,
                   ifaill: *mut lapack_int, ifailr: *mut lapack_int, info: *mut lapack_int);
    pub fn zhsein_(job: *const c_char, eigsrc: *const c_char, initv: *const c_char,
                   select: *const lapack_logical, n: *const lapack_int,
                   h: *const lapack_complex_double, ldh: *const lapack_int,
                   w: *mut lapack_complex_double, vl: *mut lapack_complex_double,
                   ldvl: *const lapack_int, vr: *mut lapack_complex_double,
                   ldvr: *const lapack_int, mm: *const lapack_int, m: *mut lapack_int,
                   work: *mut lapack_complex_double, rwork: *mut c_double,
                   ifaill: *mut lapack_int, ifailr: *mut lapack_int, info: *mut lapack_int);

    pub fn strevc_(side: *const c_char, howmny: *const c_char, select: *mut lapack_logical,
                   n: *const lapack_int, t: *const c_float, ldt: *const lapack_int,
                   vl: *mut c_float, ldvl: *const lapack_int, vr: *mut c_float,
                   ldvr: *const lapack_int, mm: *const lapack_int, m: *mut lapack_int,
                   work: *mut c_float, info: *mut lapack_int);
    pub fn dtrevc_(side: *const c_char, howmny: *const c_char, select: *mut lapack_logical,
                   n: *const lapack_int, t: *const c_double, ldt: *const lapack_int,
                   vl: *mut c_double, ldvl: *const lapack_int, vr: *mut c_double,
                   ldvr: *const lapack_int, mm: *const lapack_int, m: *mut lapack_int,
                   work: *mut c_double, info: *mut lapack_int);
    pub fn ctrevc_(side: *const c_char, howmny: *const c_char, select: *const lapack_logical,
                   n: *const lapack_int, t: *mut lapack_complex_float, ldt: *const lapack_int,
                   vl: *mut lapack_complex_float, ldvl: *const lapack_int,
                   vr: *mut lapack_complex_float, ldvr: *const lapack_int,
                   mm: *const lapack_int, m: *mut lapack_int, work: *mut lapack_complex_float,
                   rwork: *mut c_float, info: *mut lapack_int);
    pub fn ztrevc_(side: *const c_char, howmny: *const c_char, select: *const lapack_logical,
                   n: *const lapack_int, t: *mut lapack_complex_double, ldt: *const lapack_int,
                   vl: *mut lapack_complex_double, ldvl: *const lapack_int,
                   vr: *mut lapack_complex_double, ldvr: *const lapack_int,
                   mm: *const lapack_int, m: *mut lapack_int, work: *mut lapack_complex_double,
                   rwork: *mut c_double, info: *mut lapack_int);

    pub fn strsna_(job: *const c_char, howmny: *const c_char, select: *const lapack_logical,
                   n: *const lapack_int, t: *const c_float, ldt: *const lapack_int,
                   vl: *const c_float, ldvl: *const lapack_int, vr: *const c_float,
                   ldvr: *const lapack_int, s: *mut c_float, sep: *mut c_float,
                   mm: *const lapack_int, m: *mut lapack_int, work: *mut c_float,
                   ldwork: *const lapack_int, iwork: *mut lapack_int, info: *mut lapack_int);
    pub fn dtrsna_(job: *const c_char, howmny: *const c_char, select: *const lapack_logical,
                   n: *const lapack_int, t: *const c_double, ldt: *const lapack_int,
                   vl: *const c_double, ldvl: *const lapack_int, vr: *const c_double,
                   ldvr: *const lapack_int, s: *mut c_double, sep: *mut c_double,
                   mm: *const lapack_int, m: *mut lapack_int, work: *mut c_double,
                   ldwork: *const lapack_int, iwork: *mut lapack_int, info: *mut lapack_int);
    pub fn ctrsna_(job: *const c_char, howmny: *const c_char, select: *const lapack_logical,
                   n: *const lapack_int, t: *const lapack_complex_float, ldt: *const lapack_int,
                   vl: *const lapack_complex_float, ldvl: *const lapack_int,
                   vr: *const lapack_complex_float, ldvr: *const lapack_int, s: *mut c_float,
                   sep: *mut c_float, mm: *const lapack_int, m: *mut lapack_int,
                   work: *mut lapack_complex_float, ldwork: *const lapack_int,
                   rwork: *mut c_float, info: *mut lapack_int);
    pub fn ztrsna_(job: *const c_char, howmny: *const c_char, select: *const lapack_logical,
                   n: *const lapack_int, t: *const lapack_complex_double,
                   ldt: *const lapack_int, vl: *const lapack_complex_double,
                   ldvl: *const lapack_int, vr: *const lapack_complex_double,
                   ldvr: *const lapack_int, s: *mut c_double, sep: *mut c_double,
                   mm: *const lapack_int, m: *mut lapack_int, work: *mut lapack_complex_double,
                   ldwork: *const lapack_int, rwork: *mut c_double, info: *mut lapack_int);

    pub fn strexc_(compq: *const c_char, n: *const lapack_int, t: *mut c_float,
                   ldt: *const lapack_int, q: *mut c_float, ldq: *const lapack_int,
                   ifst: *const lapack_int, ilst: *mut lapack_int, work: *mut c_float,
                   info: *mut lapack_int);
    pub fn dtrexc_(compq: *const c_char, n: *const lapack_int, t: *mut c_double,
                   ldt: *const lapack_int, q: *mut c_double, ldq: *const lapack_int,
                   ifst: *const lapack_int, ilst: *mut lapack_int, work: *mut c_double,
                   info: *mut lapack_int);
    pub fn ctrexc_(compq: *const c_char, n: *const lapack_int, t: *mut lapack_complex_float,
                   ldt: *const lapack_int, q: *mut lapack_complex_float, ldq: *const lapack_int,
                   ifst: *const lapack_int, ilst: *const lapack_int, info: *const lapack_int);
    pub fn ztrexc_(compq: *const c_char, n: *const lapack_int, t: *mut lapack_complex_double,
                   ldt: *const lapack_int, q: *mut lapack_complex_double,
                   ldq: *const lapack_int, ifst: *const lapack_int, ilst: *const lapack_int,
                   info: *const lapack_int);

    pub fn strsen_(job: *const c_char, compq: *const c_char, select: *const lapack_logical,
                   n: *const lapack_int, t: *mut c_float, ldt: *const lapack_int,
                   q: *mut c_float, ldq: *const lapack_int, wr: *mut c_float, wi: *mut c_float,
                   m: *mut lapack_int, s: *mut c_float, sep: *mut c_float, work: *mut c_float,
                   lwork: *const lapack_int, iwork: *mut lapack_int, liwork: *const lapack_int,
                   info: *mut lapack_int);
    pub fn dtrsen_(job: *const c_char, compq: *const c_char, select: *const lapack_logical,
                   n: *const lapack_int, t: *mut c_double, ldt: *const lapack_int,
                   q: *mut c_double, ldq: *const lapack_int, wr: *mut c_double,
                   wi: *mut c_double, m: *mut lapack_int, s: *mut c_double, sep: *mut c_double,
                   work: *mut c_double, lwork: *const lapack_int, iwork: *mut lapack_int,
                   liwork: *const lapack_int, info: *mut lapack_int);
    pub fn ctrsen_(job: *const c_char, compq: *const c_char, select: *const lapack_logical,
                   n: *const lapack_int, t: *mut lapack_complex_float, ldt: *const lapack_int,
                   q: *mut lapack_complex_float, ldq: *const lapack_int,
                   w: *mut lapack_complex_float, m: *mut lapack_int, s: *mut c_float,
                   sep: *mut c_float, work: *mut lapack_complex_float, lwork: *const lapack_int,
                   info: *mut lapack_int);
    pub fn ztrsen_(job: *const c_char, compq: *const c_char, select: *const lapack_logical,
                   n: *const lapack_int, t: *mut lapack_complex_double, ldt: *const lapack_int,
                   q: *mut lapack_complex_double, ldq: *const lapack_int,
                   w: *mut lapack_complex_double, m: *mut lapack_int, s: *mut c_double,
                   sep: *mut c_double, work: *mut lapack_complex_double,
                   lwork: *const lapack_int, info: *mut lapack_int);

    pub fn strsyl_(trana: *const c_char, tranb: *const c_char, isgn: *const lapack_int,
                   m: *const lapack_int, n: *const lapack_int, a: *const c_float,
                   lda: *const lapack_int, b: *const c_float, ldb: *const lapack_int,
                   c: *mut c_float, ldc: *const lapack_int, scale: *mut c_float,
                   info: *mut lapack_int);
    pub fn dtrsyl_(trana: *const c_char, tranb: *const c_char, isgn: *const lapack_int,
                   m: *const lapack_int, n: *const lapack_int, a: *const c_double,
                   lda: *const lapack_int, b: *const c_double, ldb: *const lapack_int,
                   c: *mut c_double, ldc: *const lapack_int, scale: *mut c_double,
                   info: *mut lapack_int);
    pub fn ctrsyl_(trana: *const c_char, tranb: *const c_char, isgn: *const lapack_int,
                   m: *const lapack_int, n: *const lapack_int, a: *const lapack_complex_float,
                   lda: *const lapack_int, b: *const lapack_complex_float,
                   ldb: *const lapack_int, c: *mut lapack_complex_float, ldc: *const lapack_int,
                   scale: *mut c_float, info: *mut lapack_int);
    pub fn ztrsyl_(trana: *const c_char, tranb: *const c_char, isgn: *const lapack_int,
                   m: *const lapack_int, n: *const lapack_int, a: *const lapack_complex_double,
                   lda: *const lapack_int, b: *const lapack_complex_double,
                   ldb: *const lapack_int, c: *mut lapack_complex_double,
                   ldc: *const lapack_int, scale: *mut c_double, info: *mut lapack_int);

    pub fn sgghrd_(compq: *const c_char, compz: *const c_char, n: *const lapack_int,
                   ilo: *const lapack_int, ihi: *const lapack_int, a: *mut c_float,
                   lda: *const lapack_int, b: *mut c_float, ldb: *const lapack_int,
                   q: *mut c_float, ldq: *const lapack_int, z: *mut c_float,
                   ldz: *const lapack_int, info: *mut lapack_int);
    pub fn dgghrd_(compq: *const c_char, compz: *const c_char, n: *const lapack_int,
                   ilo: *const lapack_int, ihi: *const lapack_int, a: *mut c_double,
                   lda: *const lapack_int, b: *mut c_double, ldb: *const lapack_int,
                   q: *mut c_double, ldq: *const lapack_int, z: *mut c_double,
                   ldz: *const lapack_int, info: *mut lapack_int);
    pub fn cgghrd_(compq: *const c_char, compz: *const c_char, n: *const lapack_int,
                   ilo: *const lapack_int, ihi: *const lapack_int, a: *mut lapack_complex_float,
                   lda: *const lapack_int, b: *mut lapack_complex_float, ldb: *const lapack_int,
                   q: *mut lapack_complex_float, ldq: *const lapack_int,
                   z: *mut lapack_complex_float, ldz: *const lapack_int, info: *mut lapack_int);
    pub fn zgghrd_(compq: *const c_char, compz: *const c_char, n: *const lapack_int,
                   ilo: *const lapack_int, ihi: *const lapack_int,
                   a: *mut lapack_complex_double, lda: *const lapack_int,
                   b: *mut lapack_complex_double, ldb: *const lapack_int,
                   q: *mut lapack_complex_double, ldq: *const lapack_int,
                   z: *mut lapack_complex_double, ldz: *const lapack_int, info: *mut lapack_int);

    pub fn sgghd3_(compq: *const c_char, compz: *const c_char, n: *const lapack_int,
                   ilo: *const lapack_int, ihi: *const lapack_int, a: *mut c_float,
                   lda: *const lapack_int, b: *mut c_float, ldb: *const lapack_int,
                   q: *mut c_float, ldq: *const lapack_int, z: *mut c_float,
                   ldz: *const lapack_int, work: *mut c_float, lwork: *const lapack_int,
                   info: *mut lapack_int);
    pub fn dgghd3_(compq: *const c_char, compz: *const c_char, n: *const lapack_int,
                   ilo: *const lapack_int, ihi: *const lapack_int, a: *mut c_double,
                   lda: *const lapack_int, b: *mut c_double, ldb: *const lapack_int,
                   q: *mut c_double, ldq: *const lapack_int, z: *mut c_double,
                   ldz: *const lapack_int, work: *mut c_double, lwork: *const lapack_int,
                   info: *mut lapack_int);
    pub fn cgghd3_(compq: *const c_char, compz: *const c_char, n: *const lapack_int,
                   ilo: *const lapack_int, ihi: *const lapack_int, a: *mut lapack_complex_float,
                   lda: *const lapack_int, b: *mut lapack_complex_float, ldb: *const lapack_int,
                   q: *mut lapack_complex_float, ldq: *const lapack_int,
                   z: *mut lapack_complex_float, ldz: *const lapack_int,
                   work: *mut lapack_complex_float, lwork: *const lapack_int,
                   info: *mut lapack_int);
    pub fn zgghd3_(compq: *const c_char, compz: *const c_char, n: *const lapack_int,
                   ilo: *const lapack_int, ihi: *const lapack_int,
                   a: *mut lapack_complex_double, lda: *const lapack_int,
                   b: *mut lapack_complex_double, ldb: *const lapack_int,
                   q: *mut lapack_complex_double, ldq: *const lapack_int,
                   z: *mut lapack_complex_double, ldz: *const lapack_int,
                   work: *mut lapack_complex_double, lwork: *const lapack_int,
                   info: *mut lapack_int);

    pub fn sggbal_(job: *const c_char, n: *const lapack_int, a: *mut c_float,
                   lda: *const lapack_int, b: *mut c_float, ldb: *const lapack_int,
                   ilo: *mut lapack_int, ihi: *mut lapack_int, lscale: *mut c_float,
                   rscale: *mut c_float, work: *mut c_float, info: *mut lapack_int);
    pub fn dggbal_(job: *const c_char, n: *const lapack_int, a: *mut c_double,
                   lda: *const lapack_int, b: *mut c_double, ldb: *const lapack_int,
                   ilo: *mut lapack_int, ihi: *mut lapack_int, lscale: *mut c_double,
                   rscale: *mut c_double, work: *mut c_double, info: *mut lapack_int);
    pub fn cggbal_(job: *const c_char, n: *const lapack_int, a: *mut lapack_complex_float,
                   lda: *const lapack_int, b: *mut lapack_complex_float, ldb: *const lapack_int,
                   ilo: *mut lapack_int, ihi: *mut lapack_int, lscale: *mut c_float,
                   rscale: *mut c_float, work: *mut c_float, info: *mut lapack_int);
    pub fn zggbal_(job: *const c_char, n: *const lapack_int, a: *mut lapack_complex_double,
                   lda: *const lapack_int, b: *mut lapack_complex_double,
                   ldb: *const lapack_int, ilo: *mut lapack_int, ihi: *mut lapack_int,
                   lscale: *mut c_double, rscale: *mut c_double, work: *mut c_double,
                   info: *mut lapack_int);

    pub fn sggbak_(job: *const c_char, side: *const c_char, n: *const lapack_int,
                   ilo: *const lapack_int, ihi: *const lapack_int, lscale: *const c_float,
                   rscale: *const c_float, m: *const lapack_int, v: *mut c_float,
                   ldv: *const lapack_int, info: *mut lapack_int);
    pub fn dggbak_(job: *const c_char, side: *const c_char, n: *const lapack_int,
                   ilo: *const lapack_int, ihi: *const lapack_int, lscale: *const c_double,
                   rscale: *const c_double, m: *const lapack_int, v: *mut c_double,
                   ldv: *const lapack_int, info: *mut lapack_int);
    pub fn cggbak_(job: *const c_char, side: *const c_char, n: *const lapack_int,
                   ilo: *const lapack_int, ihi: *const lapack_int, lscale: *const c_float,
                   rscale: *const c_float, m: *const lapack_int, v: *mut lapack_complex_float,
                   ldv: *const lapack_int, info: *mut lapack_int);
    pub fn zggbak_(job: *const c_char, side: *const c_char, n: *const lapack_int,
                   ilo: *const lapack_int, ihi: *const lapack_int, lscale: *const c_double,
                   rscale: *const c_double, m: *const lapack_int, v: *mut lapack_complex_double,
                   ldv: *const lapack_int, info: *mut lapack_int);

    pub fn shgeqz_(job: *const c_char, compq: *const c_char, compz: *const c_char,
                   n: *const lapack_int, ilo: *const lapack_int, ihi: *const lapack_int,
                   h: *mut c_float, ldh: *const lapack_int, t: *mut c_float,
                   ldt: *const lapack_int, alphar: *mut c_float, alphai: *mut c_float,
                   beta: *mut c_float, q: *mut c_float, ldq: *const lapack_int, z: *mut c_float,
                   ldz: *const lapack_int, work: *mut c_float, lwork: *const lapack_int,
                   info: *mut lapack_int);
    pub fn dhgeqz_(job: *const c_char, compq: *const c_char, compz: *const c_char,
                   n: *const lapack_int, ilo: *const lapack_int, ihi: *const lapack_int,
                   h: *mut c_double, ldh: *const lapack_int, t: *mut c_double,
                   ldt: *const lapack_int, alphar: *mut c_double, alphai: *mut c_double,
                   beta: *mut c_double, q: *mut c_double, ldq: *const lapack_int,
                   z: *mut c_double, ldz: *const lapack_int, work: *mut c_double,
                   lwork: *const lapack_int, info: *mut lapack_int);
    pub fn chgeqz_(job: *const c_char, compq: *const c_char, compz: *const c_char,
                   n: *const lapack_int, ilo: *const lapack_int, ihi: *const lapack_int,
                   h: *mut lapack_complex_float, ldh: *const lapack_int,
                   t: *mut lapack_complex_float, ldt: *const lapack_int,
                   alpha: *mut lapack_complex_float, beta: *mut lapack_complex_float,
                   q: *mut lapack_complex_float, ldq: *const lapack_int,
                   z: *mut lapack_complex_float, ldz: *const lapack_int,
                   work: *mut lapack_complex_float, lwork: *const lapack_int,
                   rwork: *mut c_float, info: *mut lapack_int);
    pub fn zhgeqz_(job: *const c_char, compq: *const c_char, compz: *const c_char,
                   n: *const lapack_int, ilo: *const lapack_int, ihi: *const lapack_int,
                   h: *mut lapack_complex_double, ldh: *const lapack_int,
                   t: *mut lapack_complex_double, ldt: *const lapack_int,
                   alpha: *mut lapack_complex_double, beta: *mut lapack_complex_double,
                   q: *mut lapack_complex_double, ldq: *const lapack_int,
                   z: *mut lapack_complex_double, ldz: *const lapack_int,
                   work: *mut lapack_complex_double, lwork: *const lapack_int,
                   rwork: *mut c_double, info: *mut lapack_int);

    pub fn stgevc_(side: *const c_char, howmny: *const c_char, select: *const lapack_logical,
                   n: *const lapack_int, s: *const c_float, lds: *const lapack_int,
                   p: *const c_float, ldp: *const lapack_int, vl: *mut c_float,
                   ldvl: *const lapack_int, vr: *mut c_float, ldvr: *const lapack_int,
                   mm: *const lapack_int, m: *mut lapack_int, work: *mut c_float,
                   info: *mut lapack_int);
    pub fn dtgevc_(side: *const c_char, howmny: *const c_char, select: *const lapack_logical,
                   n: *const lapack_int, s: *const c_double, lds: *const lapack_int,
                   p: *const c_double, ldp: *const lapack_int, vl: *mut c_double,
                   ldvl: *const lapack_int, vr: *mut c_double, ldvr: *const lapack_int,
                   mm: *const lapack_int, m: *mut lapack_int, work: *mut c_double,
                   info: *mut lapack_int);
    pub fn ctgevc_(side: *const c_char, howmny: *const c_char, select: *const lapack_logical,
                   n: *const lapack_int, s: *const lapack_complex_float, lds: *const lapack_int,
                   p: *const lapack_complex_float, ldp: *const lapack_int,
                   vl: *mut lapack_complex_float, ldvl: *const lapack_int,
                   vr: *mut lapack_complex_float, ldvr: *const lapack_int,
                   mm: *const lapack_int, m: *mut lapack_int, work: *mut lapack_complex_float,
                   rwork: *mut c_float, info: *mut lapack_int);
    pub fn ztgevc_(side: *const c_char, howmny: *const c_char, select: *const lapack_logical,
                   n: *const lapack_int, s: *const lapack_complex_double,
                   lds: *const lapack_int, p: *const lapack_complex_double,
                   ldp: *const lapack_int, vl: *mut lapack_complex_double,
                   ldvl: *const lapack_int, vr: *mut lapack_complex_double,
                   ldvr: *const lapack_int, mm: *const lapack_int, m: *mut lapack_int,
                   work: *mut lapack_complex_double, rwork: *mut c_double, info: *mut lapack_int);

    pub fn stgexc_(wantq: *const lapack_logical, wantz: *const lapack_logical,
                   n: *const lapack_int, a: *mut c_float, lda: *const lapack_int,
                   b: *mut c_float, ldb: *const lapack_int, q: *mut c_float,
                   ldq: *const lapack_int, z: *mut c_float, ldz: *const lapack_int,
                   ifst: *mut lapack_int, ilst: *mut lapack_int, work: *mut c_float,
                   lwork: *const lapack_int, info: *mut lapack_int);
    pub fn dtgexc_(wantq: *const lapack_logical, wantz: *const lapack_logical,
                   n: *const lapack_int, a: *mut c_double, lda: *const lapack_int,
                   b: *mut c_double, ldb: *const lapack_int, q: *mut c_double,
                   ldq: *const lapack_int, z: *mut c_double, ldz: *const lapack_int,
                   ifst: *mut lapack_int, ilst: *mut lapack_int, work: *mut c_double,
                   lwork: *const lapack_int, info: *mut lapack_int);
    pub fn ctgexc_(wantq: *const lapack_logical, wantz: *const lapack_logical,
                   n: *const lapack_int, a: *mut lapack_complex_float, lda: *const lapack_int,
                   b: *mut lapack_complex_float, ldb: *const lapack_int,
                   q: *mut lapack_complex_float, ldq: *const lapack_int,
                   z: *mut lapack_complex_float, ldz: *const lapack_int,
                   ifst: *const lapack_int, ilst: *mut lapack_int, info: *mut lapack_int);
    pub fn ztgexc_(wantq: *const lapack_logical, wantz: *const lapack_logical,
                   n: *const lapack_int, a: *mut lapack_complex_double, lda: *const lapack_int,
                   b: *mut lapack_complex_double, ldb: *const lapack_int,
                   q: *mut lapack_complex_double, ldq: *const lapack_int,
                   z: *mut lapack_complex_double, ldz: *const lapack_int,
                   ifst: *const lapack_int, ilst: *mut lapack_int, info: *mut lapack_int);

    pub fn stgsen_(ijob: *const lapack_int, wantq: *const lapack_logical,
                   wantz: *const lapack_logical, select: *const lapack_logical,
                   n: *const lapack_int, a: *mut c_float, lda: *const lapack_int,
                   b: *mut c_float, ldb: *const lapack_int, alphar: *mut c_float,
                   alphai: *mut c_float, beta: *mut c_float, q: *mut c_float,
                   ldq: *const lapack_int, z: *mut c_float, ldz: *const lapack_int,
                   m: *mut lapack_int, pl: *mut c_float, pr: *mut c_float, dif: *mut c_float,
                   work: *mut c_float, lwork: *const lapack_int, iwork: *mut lapack_int,
                   liwork: *const lapack_int, info: *mut lapack_int);
    pub fn dtgsen_(ijob: *const lapack_int, wantq: *const lapack_logical,
                   wantz: *const lapack_logical, select: *const lapack_logical,
                   n: *const lapack_int, a: *mut c_double, lda: *const lapack_int,
                   b: *mut c_double, ldb: *const lapack_int, alphar: *mut c_double,
                   alphai: *mut c_double, beta: *mut c_double, q: *mut c_double,
                   ldq: *const lapack_int, z: *mut c_double, ldz: *const lapack_int,
                   m: *mut lapack_int, pl: *mut c_double, pr: *mut c_double, dif: *mut c_double,
                   work: *mut c_double, lwork: *const lapack_int, iwork: *mut lapack_int,
                   liwork: *const lapack_int, info: *mut lapack_int);
    pub fn ctgsen_(ijob: *const lapack_int, wantq: *const lapack_logical,
                   wantz: *const lapack_logical, select: *const lapack_logical,
                   n: *const lapack_int, a: *mut lapack_complex_float, lda: *const lapack_int,
                   b: *mut lapack_complex_float, ldb: *const lapack_int,
                   alpha: *mut lapack_complex_float, beta: *mut lapack_complex_float,
                   q: *mut lapack_complex_float, ldq: *const lapack_int,
                   z: *mut lapack_complex_float, ldz: *const lapack_int, m: *mut lapack_int,
                   pl: *mut c_float, pr: *mut c_float, dif: *mut c_float,
                   work: *mut lapack_complex_float, lwork: *const lapack_int,
                   iwork: *mut lapack_int, liwork: *const lapack_int, info: *mut lapack_int);
    pub fn ztgsen_(ijob: *const lapack_int, wantq: *const lapack_logical,
                   wantz: *const lapack_logical, select: *const lapack_logical,
                   n: *const lapack_int, a: *mut lapack_complex_double, lda: *const lapack_int,
                   b: *mut lapack_complex_double, ldb: *const lapack_int,
                   alpha: *mut lapack_complex_double, beta: *mut lapack_complex_double,
                   q: *mut lapack_complex_double, ldq: *const lapack_int,
                   z: *mut lapack_complex_double, ldz: *const lapack_int, m: *mut lapack_int,
                   pl: *mut c_double, pr: *mut c_double, dif: *mut c_double,
                   work: *mut lapack_complex_double, lwork: *const lapack_int,
                   iwork: *mut lapack_int, liwork: *const lapack_int, info: *mut lapack_int);

    pub fn stgsyl_(trans: *const c_char, ijob: *const lapack_int, m: *const lapack_int,
                   n: *const lapack_int, a: *const c_float, lda: *const lapack_int,
                   b: *const c_float, ldb: *const lapack_int, c: *mut c_float,
                   ldc: *const lapack_int, d: *const c_float, ldd: *const lapack_int,
                   e: *const c_float, lde: *const lapack_int, f: *mut c_float,
                   ldf: *const lapack_int, scale: *mut c_float, dif: *mut c_float,
                   work: *mut c_float, lwork: *const lapack_int, iwork: *mut lapack_int,
                   info: *mut lapack_int);
    pub fn dtgsyl_(trans: *const c_char, ijob: *const lapack_int, m: *const lapack_int,
                   n: *const lapack_int, a: *const c_double, lda: *const lapack_int,
                   b: *const c_double, ldb: *const lapack_int, c: *mut c_double,
                   ldc: *const lapack_int, d: *const c_double, ldd: *const lapack_int,
                   e: *const c_double, lde: *const lapack_int, f: *mut c_double,
                   ldf: *const lapack_int, scale: *mut c_double, dif: *mut c_double,
                   work: *mut c_double, lwork: *const lapack_int, iwork: *mut lapack_int,
                   info: *mut lapack_int);
    pub fn ctgsyl_(trans: *const c_char, ijob: *const lapack_int, m: *const lapack_int,
                   n: *const lapack_int, a: *const lapack_complex_float, lda: *const lapack_int,
                   b: *const lapack_complex_float, ldb: *const lapack_int,
                   c: *mut lapack_complex_float, ldc: *const lapack_int,
                   d: *const lapack_complex_float, ldd: *const lapack_int,
                   e: *const lapack_complex_float, lde: *const lapack_int,
                   f: *mut lapack_complex_float, ldf: *const lapack_int, scale: *mut c_float,
                   dif: *mut c_float, work: *mut lapack_complex_float, lwork: *const lapack_int,
                   iwork: *mut lapack_int, info: *mut lapack_int);
    pub fn ztgsyl_(trans: *const c_char, ijob: *const lapack_int, m: *const lapack_int,
                   n: *const lapack_int, a: *const lapack_complex_double,
                   lda: *const lapack_int, b: *const lapack_complex_double,
                   ldb: *const lapack_int, c: *mut lapack_complex_double,
                   ldc: *const lapack_int, d: *const lapack_complex_double,
                   ldd: *const lapack_int, e: *const lapack_complex_double,
                   lde: *const lapack_int, f: *mut lapack_complex_double,
                   ldf: *const lapack_int, scale: *mut c_double, dif: *mut c_double,
                   work: *mut lapack_complex_double, lwork: *const lapack_int,
                   iwork: *mut lapack_int, info: *mut lapack_int);

    pub fn stgsna_(job: *const c_char, howmny: *const c_char, select: *const lapack_logical,
                   n: *const lapack_int, a: *const c_float, lda: *const lapack_int,
                   b: *const c_float, ldb: *const lapack_int, vl: *const c_float,
                   ldvl: *const lapack_int, vr: *const c_float, ldvr: *const lapack_int,
                   s: *mut c_float, dif: *mut c_float, mm: *const lapack_int,
                   m: *mut lapack_int, work: *mut c_float, lwork: *const lapack_int,
                   iwork: *mut lapack_int, info: *mut lapack_int);
    pub fn dtgsna_(job: *const c_char, howmny: *const c_char, select: *const lapack_logical,
                   n: *const lapack_int, a: *const c_double, lda: *const lapack_int,
                   b: *const c_double, ldb: *const lapack_int, vl: *const c_double,
                   ldvl: *const lapack_int, vr: *const c_double, ldvr: *const lapack_int,
                   s: *mut c_double, dif: *mut c_double, mm: *const lapack_int,
                   m: *mut lapack_int, work: *mut c_double, lwork: *const lapack_int,
                   iwork: *mut lapack_int, info: *mut lapack_int);
    pub fn ctgsna_(job: *const c_char, howmny: *const c_char, select: *const lapack_logical,
                   n: *const lapack_int, a: *const lapack_complex_float, lda: *const lapack_int,
                   b: *const lapack_complex_float, ldb: *const lapack_int,
                   vl: *const lapack_complex_float, ldvl: *const lapack_int,
                   vr: *const lapack_complex_float, ldvr: *const lapack_int, s: *mut c_float,
                   dif: *mut c_float, mm: *const lapack_int, m: *mut lapack_int,
                   work: *mut lapack_complex_float, lwork: *const lapack_int,
                   iwork: *mut lapack_int, info: *mut lapack_int);
    pub fn ztgsna_(job: *const c_char, howmny: *const c_char, select: *const lapack_logical,
                   n: *const lapack_int, a: *const lapack_complex_double,
                   lda: *const lapack_int, b: *const lapack_complex_double,
                   ldb: *const lapack_int, vl: *const lapack_complex_double,
                   ldvl: *const lapack_int, vr: *const lapack_complex_double,
                   ldvr: *const lapack_int, s: *mut c_double, dif: *mut c_double,
                   mm: *const lapack_int, m: *mut lapack_int, work: *mut lapack_complex_double,
                   lwork: *const lapack_int, iwork: *mut lapack_int, info: *mut lapack_int);

    pub fn sggsvp_(jobu: *const c_char, jobv: *const c_char, jobq: *const c_char,
                   m: *const lapack_int, p: *const lapack_int, n: *const lapack_int,
                   a: *mut c_float, lda: *const lapack_int, b: *mut c_float,
                   ldb: *const lapack_int, tola: *const c_float, tolb: *const c_float,
                   k: *mut lapack_int, l: *mut lapack_int, u: *mut c_float,
                   ldu: *const lapack_int, v: *mut c_float, ldv: *const lapack_int,
                   q: *mut c_float, ldq: *const lapack_int, iwork: *mut lapack_int,
                   tau: *mut c_float, work: *mut c_float, info: *mut lapack_int);
    pub fn dggsvp_(jobu: *const c_char, jobv: *const c_char, jobq: *const c_char,
                   m: *const lapack_int, p: *const lapack_int, n: *const lapack_int,
                   a: *mut c_double, lda: *const lapack_int, b: *mut c_double,
                   ldb: *const lapack_int, tola: *const c_double, tolb: *const c_double,
                   k: *mut lapack_int, l: *mut lapack_int, u: *mut c_double,
                   ldu: *const lapack_int, v: *mut c_double, ldv: *const lapack_int,
                   q: *mut c_double, ldq: *const lapack_int, iwork: *mut lapack_int,
                   tau: *mut c_double, work: *mut c_double, info: *mut lapack_int);
    pub fn cggsvp_(jobu: *const c_char, jobv: *const c_char, jobq: *const c_char,
                   m: *const lapack_int, p: *const lapack_int, n: *const lapack_int,
                   a: *mut lapack_complex_float, lda: *const lapack_int,
                   b: *mut lapack_complex_float, ldb: *const lapack_int, tola: *const c_float,
                   tolb: *const c_float, k: *mut lapack_int, l: *mut lapack_int,
                   u: *mut lapack_complex_float, ldu: *const lapack_int,
                   v: *mut lapack_complex_float, ldv: *const lapack_int,
                   q: *mut lapack_complex_float, ldq: *const lapack_int, iwork: *mut lapack_int,
                   rwork: *mut c_float, tau: *mut lapack_complex_float,
                   work: *mut lapack_complex_float, info: *mut lapack_int);
    pub fn zggsvp_(jobu: *const c_char, jobv: *const c_char, jobq: *const c_char,
                   m: *const lapack_int, p: *const lapack_int, n: *const lapack_int,
                   a: *mut lapack_complex_double, lda: *const lapack_int,
                   b: *mut lapack_complex_double, ldb: *const lapack_int, tola: *const c_double,
                   tolb: *const c_double, k: *mut lapack_int, l: *mut lapack_int,
                   u: *mut lapack_complex_double, ldu: *const lapack_int,
                   v: *mut lapack_complex_double, ldv: *const lapack_int,
                   q: *mut lapack_complex_double, ldq: *const lapack_int,
                   iwork: *mut lapack_int, rwork: *mut c_double,
                   tau: *mut lapack_complex_double, work: *mut lapack_complex_double,
                   info: *mut lapack_int);

    pub fn sggsvp3_(jobu: *const c_char, jobv: *const c_char, jobq: *const c_char,
                    m: *const lapack_int, p: *const lapack_int, n: *const lapack_int,
                    a: *mut c_float, lda: *const lapack_int, b: *mut c_float,
                    ldb: *const lapack_int, tola: *const c_float, tolb: *const c_float,
                    k: *mut lapack_int, l: *mut lapack_int, u: *mut c_float,
                    ldu: *const lapack_int, v: *mut c_float, ldv: *const lapack_int,
                    q: *mut c_float, ldq: *const lapack_int, iwork: *mut lapack_int,
                    tau: *mut c_float, work: *mut c_float, lwork: *const lapack_int,
                    info: *mut lapack_int);
    pub fn dggsvp3_(jobu: *const c_char, jobv: *const c_char, jobq: *const c_char,
                    m: *const lapack_int, p: *const lapack_int, n: *const lapack_int,
                    a: *mut c_double, lda: *const lapack_int, b: *mut c_double,
                    ldb: *const lapack_int, tola: *const c_double, tolb: *const c_double,
                    k: *mut lapack_int, l: *mut lapack_int, u: *mut c_double,
                    ldu: *const lapack_int, v: *mut c_double, ldv: *const lapack_int,
                    q: *mut c_double, ldq: *const lapack_int, iwork: *mut lapack_int,
                    tau: *mut c_double, work: *mut c_double, lwork: *const lapack_int,
                    info: *mut lapack_int);
    pub fn cggsvp3_(jobu: *const c_char, jobv: *const c_char, jobq: *const c_char,
                    m: *const lapack_int, p: *const lapack_int, n: *const lapack_int,
                    a: *mut lapack_complex_float, lda: *const lapack_int,
                    b: *mut lapack_complex_float, ldb: *const lapack_int, tola: *const c_float,
                    tolb: *const c_float, k: *mut lapack_int, l: *mut lapack_int,
                    u: *mut lapack_complex_float, ldu: *const lapack_int,
                    v: *mut lapack_complex_float, ldv: *const lapack_int,
                    q: *mut lapack_complex_float, ldq: *const lapack_int,
                    iwork: *mut lapack_int, rwork: *mut c_float, tau: *mut lapack_complex_float,
                    work: *mut lapack_complex_float, lwork: *const lapack_int,
                    info: *mut lapack_int);
    pub fn zggsvp3_(jobu: *const c_char, jobv: *const c_char, jobq: *const c_char,
                    m: *const lapack_int, p: *const lapack_int, n: *const lapack_int,
                    a: *mut lapack_complex_double, lda: *const lapack_int,
                    b: *mut lapack_complex_double, ldb: *const lapack_int,
                    tola: *const c_double, tolb: *const c_double, k: *mut lapack_int,
                    l: *mut lapack_int, u: *mut lapack_complex_double, ldu: *const lapack_int,
                    v: *mut lapack_complex_double, ldv: *const lapack_int,
                    q: *mut lapack_complex_double, ldq: *const lapack_int,
                    iwork: *mut lapack_int, rwork: *mut c_double,
                    tau: *mut lapack_complex_double, work: *mut lapack_complex_double,
                    lwork: *const lapack_int, info: *mut lapack_int);

    pub fn stgsja_(jobu: *const c_char, jobv: *const c_char, jobq: *const c_char,
                   m: *const lapack_int, p: *const lapack_int, n: *const lapack_int,
                   k: *const lapack_int, l: *const lapack_int, a: *mut c_float,
                   lda: *const lapack_int, b: *mut c_float, ldb: *const lapack_int,
                   tola: *const c_float, tolb: *const c_float, alpha: *mut c_float,
                   beta: *mut c_float, u: *mut c_float, ldu: *const lapack_int, v: *mut c_float,
                   ldv: *const lapack_int, q: *mut c_float, ldq: *const lapack_int,
                   work: *mut c_float, ncycle: *mut lapack_int, info: *mut lapack_int);
    pub fn dtgsja_(jobu: *const c_char, jobv: *const c_char, jobq: *const c_char,
                   m: *const lapack_int, p: *const lapack_int, n: *const lapack_int,
                   k: *const lapack_int, l: *const lapack_int, a: *mut c_double,
                   lda: *const lapack_int, b: *mut c_double, ldb: *const lapack_int,
                   tola: *const c_double, tolb: *const c_double, alpha: *mut c_double,
                   beta: *mut c_double, u: *mut c_double, ldu: *const lapack_int,
                   v: *mut c_double, ldv: *const lapack_int, q: *mut c_double,
                   ldq: *const lapack_int, work: *mut c_double, ncycle: *mut lapack_int,
                   info: *mut lapack_int);
    pub fn ctgsja_(jobu: *const c_char, jobv: *const c_char, jobq: *const c_char,
                   m: *const lapack_int, p: *const lapack_int, n: *const lapack_int,
                   k: *const lapack_int, l: *const lapack_int, a: *mut lapack_complex_float,
                   lda: *const lapack_int, b: *mut lapack_complex_float, ldb: *const lapack_int,
                   tola: *const c_float, tolb: *const c_float, alpha: *mut c_float,
                   beta: *mut c_float, u: *mut lapack_complex_float, ldu: *const lapack_int,
                   v: *mut lapack_complex_float, ldv: *const lapack_int,
                   q: *mut lapack_complex_float, ldq: *const lapack_int,
                   work: *mut lapack_complex_float, ncycle: *mut lapack_int,
                   info: *mut lapack_int);
    pub fn ztgsja_(jobu: *const c_char, jobv: *const c_char, jobq: *const c_char,
                   m: *const lapack_int, p: *const lapack_int, n: *const lapack_int,
                   k: *const lapack_int, l: *const lapack_int, a: *mut lapack_complex_double,
                   lda: *const lapack_int, b: *mut lapack_complex_double,
                   ldb: *const lapack_int, tola: *const c_double, tolb: *const c_double,
                   alpha: *mut c_double, beta: *mut c_double, u: *mut lapack_complex_double,
                   ldu: *const lapack_int, v: *mut lapack_complex_double,
                   ldv: *const lapack_int, q: *mut lapack_complex_double,
                   ldq: *const lapack_int, work: *mut lapack_complex_double,
                   ncycle: *mut lapack_int, info: *mut lapack_int);

    pub fn sgels_(trans: *const c_char, m: *const lapack_int, n: *const lapack_int,
                  nrhs: *const lapack_int, a: *mut c_float, lda: *const lapack_int,
                  b: *mut c_float, ldb: *const lapack_int, work: *mut c_float,
                  lwork: *const lapack_int, info: *mut lapack_int);
    pub fn dgels_(trans: *const c_char, m: *const lapack_int, n: *const lapack_int,
                  nrhs: *const lapack_int, a: *mut c_double, lda: *const lapack_int,
                  b: *mut c_double, ldb: *const lapack_int, work: *mut c_double,
                  lwork: *const lapack_int, info: *mut lapack_int);
    pub fn cgels_(trans: *const c_char, m: *const lapack_int, n: *const lapack_int,
                  nrhs: *const lapack_int, a: *mut lapack_complex_float, lda: *const lapack_int,
                  b: *mut lapack_complex_float, ldb: *const lapack_int,
                  work: *mut lapack_complex_float, lwork: *const lapack_int,
                  info: *mut lapack_int);
    pub fn zgels_(trans: *const c_char, m: *const lapack_int, n: *const lapack_int,
                  nrhs: *const lapack_int, a: *mut lapack_complex_double,
                  lda: *const lapack_int, b: *mut lapack_complex_double, ldb: *const lapack_int,
                  work: *mut lapack_complex_double, lwork: *const lapack_int,
                  info: *mut lapack_int);

    pub fn sgelsy_(m: *const lapack_int, n: *const lapack_int, nrhs: *const lapack_int,
                   a: *mut c_float, lda: *const lapack_int, b: *mut c_float,
                   ldb: *const lapack_int, jpvt: *mut lapack_int, rcond: *const c_float,
                   rank: *mut lapack_int, work: *mut c_float, lwork: *const lapack_int,
                   info: *mut lapack_int);
    pub fn dgelsy_(m: *const lapack_int, n: *const lapack_int, nrhs: *const lapack_int,
                   a: *mut c_double, lda: *const lapack_int, b: *mut c_double,
                   ldb: *const lapack_int, jpvt: *mut lapack_int, rcond: *const c_double,
                   rank: *mut lapack_int, work: *mut c_double, lwork: *const lapack_int,
                   info: *mut lapack_int);
    pub fn cgelsy_(m: *const lapack_int, n: *const lapack_int, nrhs: *const lapack_int,
                   a: *mut lapack_complex_float, lda: *const lapack_int,
                   b: *mut lapack_complex_float, ldb: *const lapack_int, jpvt: *mut lapack_int,
                   rcond: *const c_float, rank: *mut lapack_int,
                   work: *mut lapack_complex_float, lwork: *const lapack_int,
                   rwork: *mut c_float, info: *mut lapack_int);
    pub fn zgelsy_(m: *const lapack_int, n: *const lapack_int, nrhs: *const lapack_int,
                   a: *mut lapack_complex_double, lda: *const lapack_int,
                   b: *mut lapack_complex_double, ldb: *const lapack_int, jpvt: *mut lapack_int,
                   rcond: *const c_double, rank: *mut lapack_int,
                   work: *mut lapack_complex_double, lwork: *const lapack_int,
                   rwork: *mut c_double, info: *mut lapack_int);

    pub fn sgelss_(m: *const lapack_int, n: *const lapack_int, nrhs: *const lapack_int,
                   a: *mut c_float, lda: *const lapack_int, b: *mut c_float,
                   ldb: *const lapack_int, s: *mut c_float, rcond: *const c_float,
                   rank: *mut lapack_int, work: *mut c_float, lwork: *const lapack_int,
                   info: *mut lapack_int);
    pub fn dgelss_(m: *const lapack_int, n: *const lapack_int, nrhs: *const lapack_int,
                   a: *mut c_double, lda: *const lapack_int, b: *mut c_double,
                   ldb: *const lapack_int, s: *mut c_double, rcond: *const c_double,
                   rank: *mut lapack_int, work: *mut c_double, lwork: *const lapack_int,
                   info: *mut lapack_int);
    pub fn cgelss_(m: *const lapack_int, n: *const lapack_int, nrhs: *const lapack_int,
                   a: *mut lapack_complex_float, lda: *const lapack_int,
                   b: *mut lapack_complex_float, ldb: *const lapack_int, s: *mut c_float,
                   rcond: *const c_float, rank: *mut lapack_int,
                   work: *mut lapack_complex_float, lwork: *const lapack_int,
                   rwork: *mut c_float, info: *mut lapack_int);
    pub fn zgelss_(m: *const lapack_int, n: *const lapack_int, nrhs: *const lapack_int,
                   a: *mut lapack_complex_double, lda: *const lapack_int,
                   b: *mut lapack_complex_double, ldb: *const lapack_int, s: *mut c_double,
                   rcond: *const c_double, rank: *mut lapack_int,
                   work: *mut lapack_complex_double, lwork: *const lapack_int,
                   rwork: *mut c_double, info: *mut lapack_int);

    pub fn sgelsd_(m: *const lapack_int, n: *const lapack_int, nrhs: *const lapack_int,
                   a: *const c_float, lda: *const lapack_int, b: *mut c_float,
                   ldb: *const lapack_int, s: *mut c_float, rcond: *const c_float,
                   rank: *mut lapack_int, work: *mut c_float, lwork: *const lapack_int,
                   iwork: *mut lapack_int, info: *mut lapack_int);
    pub fn dgelsd_(m: *const lapack_int, n: *const lapack_int, nrhs: *const lapack_int,
                   a: *const c_double, lda: *const lapack_int, b: *mut c_double,
                   ldb: *const lapack_int, s: *mut c_double, rcond: *const c_double,
                   rank: *mut lapack_int, work: *mut c_double, lwork: *const lapack_int,
                   iwork: *mut lapack_int, info: *mut lapack_int);
    pub fn cgelsd_(m: *const lapack_int, n: *const lapack_int, nrhs: *const lapack_int,
                   a: *const lapack_complex_float, lda: *const lapack_int,
                   b: *mut lapack_complex_float, ldb: *const lapack_int, s: *mut c_float,
                   rcond: *const c_float, rank: *mut lapack_int,
                   work: *mut lapack_complex_float, lwork: *const lapack_int,
                   rwork: *mut c_float, iwork: *mut lapack_int, info: *mut lapack_int);
    pub fn zgelsd_(m: *const lapack_int, n: *const lapack_int, nrhs: *const lapack_int,
                   a: *const lapack_complex_double, lda: *const lapack_int,
                   b: *mut lapack_complex_double, ldb: *const lapack_int, s: *mut c_double,
                   rcond: *const c_double, rank: *mut lapack_int,
                   work: *mut lapack_complex_double, lwork: *const lapack_int,
                   rwork: *mut c_double, iwork: *mut lapack_int, info: *mut lapack_int);

    pub fn sgglse_(m: *const lapack_int, n: *const lapack_int, p: *const lapack_int,
                   a: *mut c_float, lda: *const lapack_int, b: *mut c_float,
                   ldb: *const lapack_int, c: *mut c_float, d: *mut c_float, x: *mut c_float,
                   work: *mut c_float, lwork: *const lapack_int, info: *mut lapack_int);
    pub fn dgglse_(m: *const lapack_int, n: *const lapack_int, p: *const lapack_int,
                   a: *mut c_double, lda: *const lapack_int, b: *mut c_double,
                   ldb: *const lapack_int, c: *mut c_double, d: *mut c_double, x: *mut c_double,
                   work: *mut c_double, lwork: *const lapack_int, info: *mut lapack_int);
    pub fn cgglse_(m: *const lapack_int, n: *const lapack_int, p: *const lapack_int,
                   a: *mut lapack_complex_float, lda: *const lapack_int,
                   b: *mut lapack_complex_float, ldb: *const lapack_int,
                   c: *mut lapack_complex_float, d: *mut lapack_complex_float,
                   x: *mut lapack_complex_float, work: *mut lapack_complex_float,
                   lwork: *const lapack_int, info: *mut lapack_int);
    pub fn zgglse_(m: *const lapack_int, n: *const lapack_int, p: *const lapack_int,
                   a: *mut lapack_complex_double, lda: *const lapack_int,
                   b: *mut lapack_complex_double, ldb: *const lapack_int,
                   c: *mut lapack_complex_double, d: *mut lapack_complex_double,
                   x: *mut lapack_complex_double, work: *mut lapack_complex_double,
                   lwork: *const lapack_int, info: *mut lapack_int);

    pub fn sggglm_(n: *const lapack_int, m: *const lapack_int, p: *const lapack_int,
                   a: *mut c_float, lda: *const lapack_int, b: *mut c_float,
                   ldb: *const lapack_int, d: *mut c_float, x: *mut c_float, y: *mut c_float,
                   work: *mut c_float, lwork: *const lapack_int, info: *mut lapack_int);
    pub fn dggglm_(n: *const lapack_int, m: *const lapack_int, p: *const lapack_int,
                   a: *mut c_double, lda: *const lapack_int, b: *mut c_double,
                   ldb: *const lapack_int, d: *mut c_double, x: *mut c_double, y: *mut c_double,
                   work: *mut c_double, lwork: *const lapack_int, info: *mut lapack_int);
    pub fn cggglm_(n: *const lapack_int, m: *const lapack_int, p: *const lapack_int,
                   a: *mut lapack_complex_float, lda: *const lapack_int,
                   b: *mut lapack_complex_float, ldb: *const lapack_int,
                   d: *mut lapack_complex_float, x: *mut lapack_complex_float,
                   y: *mut lapack_complex_float, work: *mut lapack_complex_float,
                   lwork: *const lapack_int, info: *mut lapack_int);
    pub fn zggglm_(n: *const lapack_int, m: *const lapack_int, p: *const lapack_int,
                   a: *mut lapack_complex_double, lda: *const lapack_int,
                   b: *mut lapack_complex_double, ldb: *const lapack_int,
                   d: *mut lapack_complex_double, x: *mut lapack_complex_double,
                   y: *mut lapack_complex_double, work: *mut lapack_complex_double,
                   lwork: *const lapack_int, info: *mut lapack_int);

    pub fn ssyev_(jobz: *const c_char, uplo: *const c_char, n: *const lapack_int,
                  a: *mut c_float, lda: *const lapack_int, w: *mut c_float, work: *mut c_float,
                  lwork: *const lapack_int, info: *mut lapack_int);
    pub fn dsyev_(jobz: *const c_char, uplo: *const c_char, n: *const lapack_int,
                  a: *mut c_double, lda: *const lapack_int, w: *mut c_double,
                  work: *mut c_double, lwork: *const lapack_int, info: *mut lapack_int);
    pub fn cheev_(jobz: *const c_char, uplo: *const c_char, n: *const lapack_int,
                  a: *mut lapack_complex_float, lda: *const lapack_int, w: *mut c_float,
                  work: *mut lapack_complex_float, lwork: *const lapack_int,
                  rwork: *mut c_float, info: *mut lapack_int);
    pub fn zheev_(jobz: *const c_char, uplo: *const c_char, n: *const lapack_int,
                  a: *mut lapack_complex_double, lda: *const lapack_int, w: *mut c_double,
                  work: *mut lapack_complex_double, lwork: *const lapack_int,
                  rwork: *mut c_double, info: *mut lapack_int);

    pub fn ssyevd_(jobz: *const c_char, uplo: *const c_char, n: *const lapack_int,
                   a: *mut c_float, lda: *const lapack_int, w: *mut c_float, work: *mut c_float,
                   lwork: *const lapack_int, iwork: *mut lapack_int, liwork: *const lapack_int,
                   info: *mut lapack_int);
    pub fn dsyevd_(jobz: *const c_char, uplo: *const c_char, n: *const lapack_int,
                   a: *mut c_double, lda: *const lapack_int, w: *mut c_double,
                   work: *mut c_double, lwork: *const lapack_int, iwork: *mut lapack_int,
                   liwork: *const lapack_int, info: *mut lapack_int);
    pub fn cheevd_(jobz: *const c_char, uplo: *const c_char, n: *const lapack_int,
                   a: *mut lapack_complex_float, lda: *const lapack_int, w: *mut c_float,
                   work: *mut lapack_complex_float, lwork: *const lapack_int,
                   rwork: *mut c_float, lrwork: *const lapack_int, iwork: *mut lapack_int,
                   liwork: *const lapack_int, info: *mut lapack_int);
    pub fn zheevd_(jobz: *const c_char, uplo: *const c_char, n: *const lapack_int,
                   a: *mut lapack_complex_double, lda: *const lapack_int, w: *mut c_double,
                   work: *mut lapack_complex_double, lwork: *const lapack_int,
                   rwork: *mut c_double, lrwork: *const lapack_int, iwork: *mut lapack_int,
                   liwork: *const lapack_int, info: *mut lapack_int);

    pub fn ssyevx_(jobz: *const c_char, range: *const c_char, uplo: *const c_char,
                   n: *const lapack_int, a: *mut c_float, lda: *const lapack_int,
                   vl: *const c_float, vu: *const c_float, il: *const lapack_int,
                   iu: *const lapack_int, abstol: *const c_float, m: *mut lapack_int,
                   w: *mut c_float, z: *mut c_float, ldz: *const lapack_int, work: *mut c_float,
                   lwork: *const lapack_int, iwork: *mut lapack_int, ifail: *mut lapack_int,
                   info: *mut lapack_int);
    pub fn dsyevx_(jobz: *const c_char, range: *const c_char, uplo: *const c_char,
                   n: *const lapack_int, a: *mut c_double, lda: *const lapack_int,
                   vl: *const c_double, vu: *const c_double, il: *const lapack_int,
                   iu: *const lapack_int, abstol: *const c_double, m: *mut lapack_int,
                   w: *mut c_double, z: *mut c_double, ldz: *const lapack_int,
                   work: *mut c_double, lwork: *const lapack_int, iwork: *mut lapack_int,
                   ifail: *mut lapack_int, info: *mut lapack_int);
    pub fn cheevx_(jobz: *const c_char, range: *const c_char, uplo: *const c_char,
                   n: *const lapack_int, a: *mut lapack_complex_float, lda: *const lapack_int,
                   vl: *const c_float, vu: *const c_float, il: *const lapack_int,
                   iu: *const lapack_int, abstol: *const c_float, m: *mut lapack_int,
                   w: *mut c_float, z: *mut lapack_complex_float, ldz: *const lapack_int,
                   work: *mut lapack_complex_float, lwork: *const lapack_int,
                   rwork: *mut c_float, iwork: *mut lapack_int, ifail: *mut lapack_int,
                   info: *mut lapack_int);
    pub fn zheevx_(jobz: *const c_char, range: *const c_char, uplo: *const c_char,
                   n: *const lapack_int, a: *mut lapack_complex_double, lda: *const lapack_int,
                   vl: *const c_double, vu: *const c_double, il: *const lapack_int,
                   iu: *const lapack_int, abstol: *const c_double, m: *mut lapack_int,
                   w: *mut c_double, z: *mut lapack_complex_double, ldz: *const lapack_int,
                   work: *mut lapack_complex_double, lwork: *const lapack_int,
                   rwork: *mut c_double, iwork: *mut lapack_int, ifail: *mut lapack_int,
                   info: *mut lapack_int);

    pub fn ssyevr_(jobz: *const c_char, range: *const c_char, uplo: *const c_char,
                   n: *const lapack_int, a: *mut c_float, lda: *const lapack_int,
                   vl: *const c_float, vu: *const c_float, il: *const lapack_int,
                   iu: *const lapack_int, abstol: *const c_float, m: *mut lapack_int,
                   w: *mut c_float, z: *mut c_float, ldz: *const lapack_int,
                   isuppz: *mut lapack_int, work: *mut c_float, lwork: *const lapack_int,
                   iwork: *mut lapack_int, liwork: *const lapack_int, info: *mut lapack_int);
    pub fn dsyevr_(jobz: *const c_char, range: *const c_char, uplo: *const c_char,
                   n: *const lapack_int, a: *mut c_double, lda: *const lapack_int,
                   vl: *const c_double, vu: *const c_double, il: *const lapack_int,
                   iu: *const lapack_int, abstol: *const c_double, m: *mut lapack_int,
                   w: *mut c_double, z: *mut c_double, ldz: *const lapack_int,
                   isuppz: *mut lapack_int, work: *mut c_double, lwork: *const lapack_int,
                   iwork: *mut lapack_int, liwork: *const lapack_int, info: *mut lapack_int);
    pub fn cheevr_(jobz: *const c_char, range: *const c_char, uplo: *const c_char,
                   n: *const lapack_int, a: *mut lapack_complex_float, lda: *const lapack_int,
                   vl: *const c_float, vu: *const c_float, il: *const lapack_int,
                   iu: *const lapack_int, abstol: *const c_float, m: *mut lapack_int,
                   w: *mut c_float, z: *mut lapack_complex_float, ldz: *const lapack_int,
                   isuppz: *mut lapack_int, work: *mut lapack_complex_float,
                   lwork: *const lapack_int, rwork: *mut c_float, lrwork: *const lapack_int,
                   iwork: *mut lapack_int, liwork: *const lapack_int, info: *mut lapack_int);
    pub fn zheevr_(jobz: *const c_char, range: *const c_char, uplo: *const c_char,
                   n: *const lapack_int, a: *mut lapack_complex_double, lda: *const lapack_int,
                   vl: *const c_double, vu: *const c_double, il: *const lapack_int,
                   iu: *const lapack_int, abstol: *const c_double, m: *mut lapack_int,
                   w: *mut c_double, z: *mut lapack_complex_double, ldz: *const lapack_int,
                   isuppz: *mut lapack_int, work: *mut lapack_complex_double,
                   lwork: *const lapack_int, rwork: *mut c_double, lrwork: *const lapack_int,
                   iwork: *mut lapack_int, liwork: *const lapack_int, info: *mut lapack_int);

    pub fn sspev_(jobz: *const c_char, uplo: *const c_char, n: *const lapack_int,
                  ap: *mut c_float, w: *mut c_float, z: *mut c_float, ldz: *const lapack_int,
                  work: *mut c_float, info: *mut lapack_int);
    pub fn dspev_(jobz: *const c_char, uplo: *const c_char, n: *const lapack_int,
                  ap: *mut c_double, w: *mut c_double, z: *mut c_double, ldz: *const lapack_int,
                  work: *mut c_double, info: *mut lapack_int);
    pub fn chpev_(jobz: *const c_char, uplo: *const c_char, n: *const lapack_int,
                  ap: *mut lapack_complex_float, w: *mut c_float, z: *mut lapack_complex_float,
                  ldz: *const lapack_int, work: *mut lapack_complex_float, rwork: *mut c_float,
                  info: *mut lapack_int);
    pub fn zhpev_(jobz: *const c_char, uplo: *const c_char, n: *const lapack_int,
                  ap: *mut lapack_complex_double, w: *mut c_double,
                  z: *mut lapack_complex_double, ldz: *const lapack_int,
                  work: *mut lapack_complex_double, rwork: *mut c_double, info: *mut lapack_int);

    pub fn sspevd_(jobz: *const c_char, uplo: *const c_char, n: *const lapack_int,
                   ap: *mut c_float, w: *mut c_float, z: *mut c_float, ldz: *const lapack_int,
                   work: *mut c_float, lwork: *const lapack_int, iwork: *mut lapack_int,
                   liwork: *const lapack_int, info: *mut lapack_int);
    pub fn dspevd_(jobz: *const c_char, uplo: *const c_char, n: *const lapack_int,
                   ap: *mut c_double, w: *mut c_double, z: *mut c_double,
                   ldz: *const lapack_int, work: *mut c_double, lwork: *const lapack_int,
                   iwork: *mut lapack_int, liwork: *const lapack_int, info: *mut lapack_int);
    pub fn chpevd_(jobz: *const c_char, uplo: *const c_char, n: *const lapack_int,
                   ap: *mut lapack_complex_float, w: *mut c_float, z: *mut lapack_complex_float,
                   ldz: *const lapack_int, work: *mut lapack_complex_float,
                   lwork: *const lapack_int, rwork: *mut c_float, lrwork: *const lapack_int,
                   iwork: *mut lapack_int, liwork: *const lapack_int, info: *mut lapack_int);
    pub fn zhpevd_(jobz: *const c_char, uplo: *const c_char, n: *const lapack_int,
                   ap: *mut lapack_complex_double, w: *mut c_double,
                   z: *mut lapack_complex_double, ldz: *const lapack_int,
                   work: *mut lapack_complex_double, lwork: *const lapack_int,
                   rwork: *mut c_double, lrwork: *const lapack_int, iwork: *mut lapack_int,
                   liwork: *const lapack_int, info: *mut lapack_int);

    pub fn sspevx_(jobz: *const c_char, range: *const c_char, uplo: *const c_char,
                   n: *const lapack_int, ap: *mut c_float, vl: *const c_float,
                   vu: *const c_float, il: *const lapack_int, iu: *const lapack_int,
                   abstol: *const c_float, m: *mut lapack_int, w: *mut c_float, z: *mut c_float,
                   ldz: *const lapack_int, work: *mut c_float, iwork: *mut lapack_int,
                   ifail: *mut lapack_int, info: *mut lapack_int);
    pub fn dspevx_(jobz: *const c_char, range: *const c_char, uplo: *const c_char,
                   n: *const lapack_int, ap: *mut c_double, vl: *const c_double,
                   vu: *const c_double, il: *const lapack_int, iu: *const lapack_int,
                   abstol: *const c_double, m: *mut lapack_int, w: *mut c_double,
                   z: *mut c_double, ldz: *const lapack_int, work: *mut c_double,
                   iwork: *mut lapack_int, ifail: *mut lapack_int, info: *mut lapack_int);
    pub fn chpevx_(jobz: *const c_char, range: *const c_char, uplo: *const c_char,
                   n: *const lapack_int, ap: *mut lapack_complex_float, vl: *const c_float,
                   vu: *const c_float, il: *const lapack_int, iu: *const lapack_int,
                   abstol: *const c_float, m: *mut lapack_int, w: *mut c_float,
                   z: *mut lapack_complex_float, ldz: *const lapack_int,
                   work: *mut lapack_complex_float, rwork: *mut c_float, iwork: *mut lapack_int,
                   ifail: *mut lapack_int, info: *mut lapack_int);
    pub fn zhpevx_(jobz: *const c_char, range: *const c_char, uplo: *const c_char,
                   n: *const lapack_int, ap: *mut lapack_complex_double, vl: *const c_double,
                   vu: *const c_double, il: *const lapack_int, iu: *const lapack_int,
                   abstol: *const c_double, m: *mut lapack_int, w: *mut c_double,
                   z: *mut lapack_complex_double, ldz: *const lapack_int,
                   work: *mut lapack_complex_double, rwork: *mut c_double,
                   iwork: *mut lapack_int, ifail: *mut lapack_int, info: *mut lapack_int);

    pub fn ssbev_(jobz: *const c_char, uplo: *const c_char, n: *const lapack_int,
                  kd: *const lapack_int, ab: *mut c_float, ldab: *const lapack_int,
                  w: *mut c_float, z: *mut c_float, ldz: *const lapack_int, work: *mut c_float,
                  info: *mut lapack_int);
    pub fn dsbev_(jobz: *const c_char, uplo: *const c_char, n: *const lapack_int,
                  kd: *const lapack_int, ab: *mut c_double, ldab: *const lapack_int,
                  w: *mut c_double, z: *mut c_double, ldz: *const lapack_int,
                  work: *mut c_double, info: *mut lapack_int);
    pub fn chbev_(jobz: *const c_char, uplo: *const c_char, n: *const lapack_int,
                  kd: *const lapack_int, ab: *mut lapack_complex_float, ldab: *const lapack_int,
                  w: *mut c_float, z: *mut lapack_complex_float, ldz: *const lapack_int,
                  work: *mut lapack_complex_float, rwork: *mut c_float, info: *mut lapack_int);
    pub fn zhbev_(jobz: *const c_char, uplo: *const c_char, n: *const lapack_int,
                  kd: *const lapack_int, ab: *mut lapack_complex_double,
                  ldab: *const lapack_int, w: *mut c_double, z: *mut lapack_complex_double,
                  ldz: *const lapack_int, work: *mut lapack_complex_double,
                  rwork: *mut c_double, info: *mut lapack_int);

    pub fn ssbevd_(jobz: *const c_char, uplo: *const c_char, n: *const lapack_int,
                   kd: *const lapack_int, ab: *mut c_float, ldab: *const lapack_int,
                   w: *mut c_float, z: *mut c_float, ldz: *const lapack_int, work: *mut c_float,
                   lwork: *const lapack_int, iwork: *mut lapack_int, liwork: *const lapack_int,
                   info: *mut lapack_int);
    pub fn dsbevd_(jobz: *const c_char, uplo: *const c_char, n: *const lapack_int,
                   kd: *const lapack_int, ab: *mut c_double, ldab: *const lapack_int,
                   w: *mut c_double, z: *mut c_double, ldz: *const lapack_int,
                   work: *mut c_double, lwork: *const lapack_int, iwork: *mut lapack_int,
                   liwork: *const lapack_int, info: *mut lapack_int);
    pub fn chbevd_(jobz: *const c_char, uplo: *const c_char, n: *const lapack_int,
                   kd: *const lapack_int, ab: *mut lapack_complex_float,
                   ldab: *const lapack_int, w: *mut c_float, z: *mut lapack_complex_float,
                   ldz: *const lapack_int, work: *mut lapack_complex_float,
                   lwork: *const lapack_int, rwork: *mut c_float, lrwork: *const lapack_int,
                   iwork: *mut lapack_int, liwork: *const lapack_int, info: *mut lapack_int);
    pub fn zhbevd_(jobz: *const c_char, uplo: *const c_char, n: *const lapack_int,
                   kd: *const lapack_int, ab: *mut lapack_complex_double,
                   ldab: *const lapack_int, w: *mut c_double, z: *mut lapack_complex_double,
                   ldz: *const lapack_int, work: *mut lapack_complex_double,
                   lwork: *const lapack_int, rwork: *mut c_double, lrwork: *const lapack_int,
                   iwork: *mut lapack_int, liwork: *const lapack_int, info: *mut lapack_int);

    pub fn ssbevx_(jobz: *const c_char, range: *const c_char, uplo: *const c_char,
                   n: *const lapack_int, kd: *const lapack_int, ab: *mut c_float,
                   ldab: *const lapack_int, q: *mut c_float, ldq: *const lapack_int,
                   vl: *const c_float, vu: *const c_float, il: *const lapack_int,
                   iu: *const lapack_int, abstol: *const c_float, m: *mut lapack_int,
                   w: *mut c_float, z: *mut c_float, ldz: *const lapack_int, work: *mut c_float,
                   iwork: *mut lapack_int, ifail: *mut lapack_int, info: *mut lapack_int);
    pub fn dsbevx_(jobz: *const c_char, range: *const c_char, uplo: *const c_char,
                   n: *const lapack_int, kd: *const lapack_int, ab: *mut c_double,
                   ldab: *const lapack_int, q: *mut c_double, ldq: *const lapack_int,
                   vl: *const c_double, vu: *const c_double, il: *const lapack_int,
                   iu: *const lapack_int, abstol: *const c_double, m: *mut lapack_int,
                   w: *mut c_double, z: *mut c_double, ldz: *const lapack_int,
                   work: *mut c_double, iwork: *mut lapack_int, ifail: *mut lapack_int,
                   info: *mut lapack_int);
    pub fn chbevx_(jobz: *const c_char, range: *const c_char, uplo: *const c_char,
                   n: *const lapack_int, kd: *const lapack_int, ab: *mut lapack_complex_float,
                   ldab: *const lapack_int, q: *mut lapack_complex_float,
                   ldq: *const lapack_int, vl: *const c_float, vu: *const c_float,
                   il: *const lapack_int, iu: *const lapack_int, abstol: *const c_float,
                   m: *mut lapack_int, w: *mut c_float, z: *mut lapack_complex_float,
                   ldz: *const lapack_int, work: *mut lapack_complex_float, rwork: *mut c_float,
                   iwork: *mut lapack_int, ifail: *mut lapack_int, info: *mut lapack_int);
    pub fn zhbevx_(jobz: *const c_char, range: *const c_char, uplo: *const c_char,
                   n: *const lapack_int, kd: *const lapack_int, ab: *mut lapack_complex_double,
                   ldab: *const lapack_int, q: *mut lapack_complex_double,
                   ldq: *const lapack_int, vl: *const c_double, vu: *const c_double,
                   il: *const lapack_int, iu: *const lapack_int, abstol: *const c_double,
                   m: *mut lapack_int, w: *mut c_double, z: *mut lapack_complex_double,
                   ldz: *const lapack_int, work: *mut lapack_complex_double,
                   rwork: *mut c_double, iwork: *mut lapack_int, ifail: *mut lapack_int,
                   info: *mut lapack_int);

    pub fn sstev_(jobz: *const c_char, n: *const lapack_int, d: *mut c_float, e: *mut c_float,
                  z: *mut c_float, ldz: *const lapack_int, work: *mut c_float,
                  info: *mut lapack_int);
    pub fn dstev_(jobz: *const c_char, n: *const lapack_int, d: *mut c_double, e: *mut c_double,
                  z: *mut c_double, ldz: *const lapack_int, work: *mut c_double,
                  info: *mut lapack_int);

    pub fn sstevd_(jobz: *const c_char, n: *const lapack_int, d: *mut c_float, e: *mut c_float,
                   z: *mut c_float, ldz: *const lapack_int, work: *mut c_float,
                   lwork: *const lapack_int, iwork: *mut lapack_int, liwork: *const lapack_int,
                   info: *mut lapack_int);
    pub fn dstevd_(jobz: *const c_char, n: *const lapack_int, d: *mut c_double,
                   e: *mut c_double, z: *mut c_double, ldz: *const lapack_int,
                   work: *mut c_double, lwork: *const lapack_int, iwork: *mut lapack_int,
                   liwork: *const lapack_int, info: *mut lapack_int);

    pub fn sstevx_(jobz: *const c_char, range: *const c_char, n: *const lapack_int,
                   d: *mut c_float, e: *mut c_float, vl: *const c_float, vu: *const c_float,
                   il: *const lapack_int, iu: *const lapack_int, abstol: *const c_float,
                   m: *mut lapack_int, w: *mut c_float, z: *mut c_float, ldz: *const lapack_int,
                   work: *mut c_float, iwork: *mut lapack_int, ifail: *mut lapack_int,
                   info: *mut lapack_int);
    pub fn dstevx_(jobz: *const c_char, range: *const c_char, n: *const lapack_int,
                   d: *mut c_double, e: *mut c_double, vl: *const c_double, vu: *const c_double,
                   il: *const lapack_int, iu: *const lapack_int, abstol: *const c_double,
                   m: *mut lapack_int, w: *mut c_double, z: *mut c_double,
                   ldz: *const lapack_int, work: *mut c_double, iwork: *mut lapack_int,
                   ifail: *mut lapack_int, info: *mut lapack_int);

    pub fn sstevr_(jobz: *const c_char, range: *const c_char, n: *const lapack_int,
                   d: *mut c_float, e: *mut c_float, vl: *const c_float, vu: *const c_float,
                   il: *const lapack_int, iu: *const lapack_int, abstol: *const c_float,
                   m: *mut lapack_int, w: *mut c_float, z: *mut c_float, ldz: *const lapack_int,
                   isuppz: *mut lapack_int, work: *mut c_float, lwork: *const lapack_int,
                   iwork: *mut lapack_int, liwork: *const lapack_int, info: *mut lapack_int);
    pub fn dstevr_(jobz: *const c_char, range: *const c_char, n: *const lapack_int,
                   d: *mut c_double, e: *mut c_double, vl: *const c_double, vu: *const c_double,
                   il: *const lapack_int, iu: *const lapack_int, abstol: *const c_double,
                   m: *mut lapack_int, w: *mut c_double, z: *mut c_double,
                   ldz: *const lapack_int, isuppz: *mut lapack_int, work: *mut c_double,
                   lwork: *const lapack_int, iwork: *mut lapack_int, liwork: *const lapack_int,
                   info: *mut lapack_int);

    pub fn sgees_(jobvs: *const c_char, sort: *const c_char, select: LAPACK_S_SELECT2,
                  n: *const lapack_int, a: *mut c_float, lda: *const lapack_int,
                  sdim: *mut lapack_int, wr: *mut c_float, wi: *mut c_float, vs: *mut c_float,
                  ldvs: *const lapack_int, work: *mut c_float, lwork: *const lapack_int,
                  bwork: *mut lapack_logical, info: *mut lapack_int);
    pub fn dgees_(jobvs: *const c_char, sort: *const c_char, select: LAPACK_D_SELECT2,
                  n: *const lapack_int, a: *mut c_double, lda: *const lapack_int,
                  sdim: *mut lapack_int, wr: *mut c_double, wi: *mut c_double,
                  vs: *mut c_double, ldvs: *const lapack_int, work: *mut c_double,
                  lwork: *const lapack_int, bwork: *mut lapack_logical, info: *mut lapack_int);
    pub fn cgees_(jobvs: *const c_char, sort: *const c_char, select: LAPACK_C_SELECT1,
                  n: *const lapack_int, a: *mut lapack_complex_float, lda: *const lapack_int,
                  sdim: *mut lapack_int, w: *mut lapack_complex_float,
                  vs: *mut lapack_complex_float, ldvs: *const lapack_int,
                  work: *mut lapack_complex_float, lwork: *const lapack_int,
                  rwork: *mut c_float, bwork: *mut lapack_logical, info: *mut lapack_int);
    pub fn zgees_(jobvs: *const c_char, sort: *const c_char, select: LAPACK_Z_SELECT1,
                  n: *const lapack_int, a: *mut lapack_complex_double, lda: *const lapack_int,
                  sdim: *mut lapack_int, w: *mut lapack_complex_double,
                  vs: *mut lapack_complex_double, ldvs: *const lapack_int,
                  work: *mut lapack_complex_double, lwork: *const lapack_int,
                  rwork: *mut c_double, bwork: *mut lapack_logical, info: *mut lapack_int);

    pub fn sgeesx_(jobvs: *const c_char, sort: *const c_char, select: LAPACK_S_SELECT2,
                   sense: *const c_char, n: *const lapack_int, a: *mut c_float,
                   lda: *const lapack_int, sdim: *mut lapack_int, wr: *mut c_float,
                   wi: *mut c_float, vs: *mut c_float, ldvs: *const lapack_int,
                   rconde: *mut c_float, rcondv: *mut c_float, work: *mut c_float,
                   lwork: *const lapack_int, iwork: *mut lapack_int, liwork: *const lapack_int,
                   bwork: *mut lapack_logical, info: *mut lapack_int);
    pub fn dgeesx_(jobvs: *const c_char, sort: *const c_char, select: LAPACK_D_SELECT2,
                   sense: *const c_char, n: *const lapack_int, a: *mut c_double,
                   lda: *const lapack_int, sdim: *mut lapack_int, wr: *mut c_double,
                   wi: *mut c_double, vs: *mut c_double, ldvs: *const lapack_int,
                   rconde: *mut c_double, rcondv: *mut c_double, work: *mut c_double,
                   lwork: *const lapack_int, iwork: *mut lapack_int, liwork: *const lapack_int,
                   bwork: *mut lapack_logical, info: *mut lapack_int);
    pub fn cgeesx_(jobvs: *const c_char, sort: *const c_char, select: LAPACK_C_SELECT1,
                   sense: *const c_char, n: *const lapack_int, a: *mut lapack_complex_float,
                   lda: *const lapack_int, sdim: *mut lapack_int, w: *mut lapack_complex_float,
                   vs: *mut lapack_complex_float, ldvs: *const lapack_int, rconde: *mut c_float,
                   rcondv: *mut c_float, work: *mut lapack_complex_float,
                   lwork: *const lapack_int, rwork: *mut c_float, bwork: *mut lapack_logical,
                   info: *mut lapack_int);
    pub fn zgeesx_(jobvs: *const c_char, sort: *const c_char, select: LAPACK_Z_SELECT1,
                   sense: *const c_char, n: *const lapack_int, a: *mut lapack_complex_double,
                   lda: *const lapack_int, sdim: *mut lapack_int, w: *mut lapack_complex_double,
                   vs: *mut lapack_complex_double, ldvs: *const lapack_int,
                   rconde: *mut c_double, rcondv: *mut c_double,
                   work: *mut lapack_complex_double, lwork: *const lapack_int,
                   rwork: *mut c_double, bwork: *mut lapack_logical, info: *mut lapack_int);

    pub fn sgeev_(jobvl: *const c_char, jobvr: *const c_char, n: *const lapack_int,
                  a: *mut c_float, lda: *const lapack_int, wr: *mut c_float, wi: *mut c_float,
                  vl: *mut c_float, ldvl: *const lapack_int, vr: *mut c_float,
                  ldvr: *const lapack_int, work: *mut c_float, lwork: *const lapack_int,
                  info: *mut lapack_int);
    pub fn dgeev_(jobvl: *const c_char, jobvr: *const c_char, n: *const lapack_int,
                  a: *mut c_double, lda: *const lapack_int, wr: *mut c_double,
                  wi: *mut c_double, vl: *mut c_double, ldvl: *const lapack_int,
                  vr: *mut c_double, ldvr: *const lapack_int, work: *mut c_double,
                  lwork: *const lapack_int, info: *mut lapack_int);
    pub fn cgeev_(jobvl: *const c_char, jobvr: *const c_char, n: *const lapack_int,
                  a: *mut lapack_complex_float, lda: *const lapack_int,
                  w: *mut lapack_complex_float, vl: *mut lapack_complex_float,
                  ldvl: *const lapack_int, vr: *mut lapack_complex_float,
                  ldvr: *const lapack_int, work: *mut lapack_complex_float,
                  lwork: *const lapack_int, rwork: *mut c_float, info: *mut lapack_int);
    pub fn zgeev_(jobvl: *const c_char, jobvr: *const c_char, n: *const lapack_int,
                  a: *mut lapack_complex_double, lda: *const lapack_int,
                  w: *mut lapack_complex_double, vl: *mut lapack_complex_double,
                  ldvl: *const lapack_int, vr: *mut lapack_complex_double,
                  ldvr: *const lapack_int, work: *mut lapack_complex_double,
                  lwork: *const lapack_int, rwork: *mut c_double, info: *mut lapack_int);

    pub fn sgeevx_(balanc: *const c_char, jobvl: *const c_char, jobvr: *const c_char,
                   sense: *const c_char, n: *const lapack_int, a: *mut c_float,
                   lda: *const lapack_int, wr: *mut c_float, wi: *mut c_float, vl: *mut c_float,
                   ldvl: *const lapack_int, vr: *mut c_float, ldvr: *const lapack_int,
                   ilo: *mut lapack_int, ihi: *mut lapack_int, scale: *mut c_float,
                   abnrm: *mut c_float, rconde: *mut c_float, rcondv: *mut c_float,
                   work: *mut c_float, lwork: *const lapack_int, iwork: *mut lapack_int,
                   info: *mut lapack_int);
    pub fn dgeevx_(balanc: *const c_char, jobvl: *const c_char, jobvr: *const c_char,
                   sense: *const c_char, n: *const lapack_int, a: *mut c_double,
                   lda: *const lapack_int, wr: *mut c_double, wi: *mut c_double,
                   vl: *mut c_double, ldvl: *const lapack_int, vr: *mut c_double,
                   ldvr: *const lapack_int, ilo: *mut lapack_int, ihi: *mut lapack_int,
                   scale: *mut c_double, abnrm: *mut c_double, rconde: *mut c_double,
                   rcondv: *mut c_double, work: *mut c_double, lwork: *const lapack_int,
                   iwork: *mut lapack_int, info: *mut lapack_int);
    pub fn cgeevx_(balanc: *const c_char, jobvl: *const c_char, jobvr: *const c_char,
                   sense: *const c_char, n: *const lapack_int, a: *mut lapack_complex_float,
                   lda: *const lapack_int, w: *mut lapack_complex_float,
                   vl: *mut lapack_complex_float, ldvl: *const lapack_int,
                   vr: *mut lapack_complex_float, ldvr: *const lapack_int, ilo: *mut lapack_int,
                   ihi: *mut lapack_int, scale: *mut c_float, abnrm: *mut c_float,
                   rconde: *mut c_float, rcondv: *mut c_float, work: *mut lapack_complex_float,
                   lwork: *const lapack_int, rwork: *mut c_float, info: *mut lapack_int);
    pub fn zgeevx_(balanc: *const c_char, jobvl: *const c_char, jobvr: *const c_char,
                   sense: *const c_char, n: *const lapack_int, a: *mut lapack_complex_double,
                   lda: *const lapack_int, w: *mut lapack_complex_double,
                   vl: *mut lapack_complex_double, ldvl: *const lapack_int,
                   vr: *mut lapack_complex_double, ldvr: *const lapack_int,
                   ilo: *mut lapack_int, ihi: *mut lapack_int, scale: *mut c_double,
                   abnrm: *mut c_double, rconde: *mut c_double, rcondv: *mut c_double,
                   work: *mut lapack_complex_double, lwork: *const lapack_int,
                   rwork: *mut c_double, info: *mut lapack_int);

    pub fn sgesvd_(jobu: *const c_char, jobvt: *const c_char, m: *const lapack_int,
                   n: *const lapack_int, a: *mut c_float, lda: *const lapack_int,
                   s: *mut c_float, u: *mut c_float, ldu: *const lapack_int, vt: *mut c_float,
                   ldvt: *const lapack_int, work: *mut c_float, lwork: *const lapack_int,
                   info: *mut lapack_int);
    pub fn dgesvd_(jobu: *const c_char, jobvt: *const c_char, m: *const lapack_int,
                   n: *const lapack_int, a: *mut c_double, lda: *const lapack_int,
                   s: *mut c_double, u: *mut c_double, ldu: *const lapack_int,
                   vt: *mut c_double, ldvt: *const lapack_int, work: *mut c_double,
                   lwork: *const lapack_int, info: *mut lapack_int);
    pub fn cgesvd_(jobu: *const c_char, jobvt: *const c_char, m: *const lapack_int,
                   n: *const lapack_int, a: *mut lapack_complex_float, lda: *const lapack_int,
                   s: *mut c_float, u: *mut lapack_complex_float, ldu: *const lapack_int,
                   vt: *mut lapack_complex_float, ldvt: *const lapack_int,
                   work: *mut lapack_complex_float, lwork: *const lapack_int,
                   rwork: *mut c_float, info: *mut lapack_int);
    pub fn zgesvd_(jobu: *const c_char, jobvt: *const c_char, m: *const lapack_int,
                   n: *const lapack_int, a: *mut lapack_complex_double, lda: *const lapack_int,
                   s: *mut c_double, u: *mut lapack_complex_double, ldu: *const lapack_int,
                   vt: *mut lapack_complex_double, ldvt: *const lapack_int,
                   work: *mut lapack_complex_double, lwork: *const lapack_int,
                   rwork: *mut c_double, info: *mut lapack_int);

    pub fn sgesvdx_(jobu: *const c_char, jobvt: *const c_char, range: *const c_char,
                    m: *const lapack_int, n: *const lapack_int, a: *mut c_float,
                    lda: *const lapack_int, vl: *const c_float, vu: *const c_float,
                    il: *const lapack_int, iu: *const lapack_int, ns: *mut lapack_int,
                    s: *mut c_float, u: *mut c_float, ldu: *const lapack_int, vt: *mut c_float,
                    ldvt: *const lapack_int, work: *mut c_float, lwork: *const lapack_int,
                    iwork: *mut lapack_int, info: *mut lapack_int);
    pub fn dgesvdx_(jobu: *const c_char, jobvt: *const c_char, range: *const c_char,
                    m: *const lapack_int, n: *const lapack_int, a: *mut c_double,
                    lda: *const lapack_int, vl: *const c_double, vu: *const c_double,
                    il: *const lapack_int, iu: *const lapack_int, ns: *mut lapack_int,
                    s: *mut c_double, u: *mut c_double, ldu: *const lapack_int,
                    vt: *mut c_double, ldvt: *const lapack_int, work: *mut c_double,
                    lwork: *const lapack_int, iwork: *mut lapack_int, info: *mut lapack_int);
    pub fn cgesvdx_(jobu: *const c_char, jobvt: *const c_char, range: *const c_char,
                    m: *const lapack_int, n: *const lapack_int, a: *mut lapack_complex_float,
                    lda: *const lapack_int, vl: *const c_float, vu: *const c_float,
                    il: *const lapack_int, iu: *const lapack_int, ns: *mut lapack_int,
                    s: *mut c_float, u: *mut lapack_complex_float, ldu: *const lapack_int,
                    vt: *mut lapack_complex_float, ldvt: *const lapack_int,
                    work: *mut lapack_complex_float, lwork: *const lapack_int,
                    rwork: *mut c_float, iwork: *mut lapack_int, info: *mut lapack_int);
    pub fn zgesvdx_(jobu: *const c_char, jobvt: *const c_char, range: *const c_char,
                    m: *const lapack_int, n: *const lapack_int, a: *mut lapack_complex_double,
                    lda: *const lapack_int, vl: *const c_double, vu: *const c_double,
                    il: *const lapack_int, iu: *const lapack_int, ns: *mut lapack_int,
                    s: *mut c_double, u: *mut lapack_complex_double, ldu: *const lapack_int,
                    vt: *mut lapack_complex_double, ldvt: *const lapack_int,
                    work: *mut lapack_complex_double, lwork: *const lapack_int,
                    rwork: *mut c_double, iwork: *mut lapack_int, info: *mut lapack_int);

    pub fn sgesdd_(jobz: *const c_char, m: *const lapack_int, n: *const lapack_int,
                   a: *mut c_float, lda: *const lapack_int, s: *mut c_float, u: *mut c_float,
                   ldu: *const lapack_int, vt: *mut c_float, ldvt: *const lapack_int,
                   work: *mut c_float, lwork: *const lapack_int, iwork: *mut lapack_int,
                   info: *mut lapack_int);
    pub fn dgesdd_(jobz: *const c_char, m: *const lapack_int, n: *const lapack_int,
                   a: *mut c_double, lda: *const lapack_int, s: *mut c_double, u: *mut c_double,
                   ldu: *const lapack_int, vt: *mut c_double, ldvt: *const lapack_int,
                   work: *mut c_double, lwork: *const lapack_int, iwork: *mut lapack_int,
                   info: *mut lapack_int);
    pub fn cgesdd_(jobz: *const c_char, m: *const lapack_int, n: *const lapack_int,
                   a: *mut lapack_complex_float, lda: *const lapack_int, s: *mut c_float,
                   u: *mut lapack_complex_float, ldu: *const lapack_int,
                   vt: *mut lapack_complex_float, ldvt: *const lapack_int,
                   work: *mut lapack_complex_float, lwork: *const lapack_int,
                   rwork: *mut c_float, iwork: *mut lapack_int, info: *mut lapack_int);
    pub fn zgesdd_(jobz: *const c_char, m: *const lapack_int, n: *const lapack_int,
                   a: *mut lapack_complex_double, lda: *const lapack_int, s: *mut c_double,
                   u: *mut lapack_complex_double, ldu: *const lapack_int,
                   vt: *mut lapack_complex_double, ldvt: *const lapack_int,
                   work: *mut lapack_complex_double, lwork: *const lapack_int,
                   rwork: *mut c_double, iwork: *mut lapack_int, info: *mut lapack_int);

    pub fn dgejsv_(joba: *const c_char, jobu: *const c_char, jobv: *const c_char,
                   jobr: *const c_char, jobt: *const c_char, jobp: *const c_char,
                   m: *const lapack_int, n: *const lapack_int, a: *mut c_double,
                   lda: *const lapack_int, sva: *mut c_double, u: *mut c_double,
                   ldu: *const lapack_int, v: *mut c_double, ldv: *const lapack_int,
                   work: *mut c_double, lwork: *const lapack_int, iwork: *mut lapack_int,
                   info: *mut lapack_int);
    pub fn sgejsv_(joba: *const c_char, jobu: *const c_char, jobv: *const c_char,
                   jobr: *const c_char, jobt: *const c_char, jobp: *const c_char,
                   m: *const lapack_int, n: *const lapack_int, a: *mut c_float,
                   lda: *const lapack_int, sva: *mut c_float, u: *mut c_float,
                   ldu: *const lapack_int, v: *mut c_float, ldv: *const lapack_int,
                   work: *mut c_float, lwork: *const lapack_int, iwork: *mut lapack_int,
                   info: *mut lapack_int);
    pub fn cgejsv_(joba: *const c_char, jobu: *const c_char, jobv: *const c_char,
                   jobr: *const c_char, jobt: *const c_char, jobp: *const c_char,
                   m: *const lapack_int, n: *const lapack_int, a: *mut lapack_complex_float,
                   lda: *const lapack_int, sva: *mut c_float, u: *mut lapack_complex_float,
                   ldu: *const lapack_int, v: *mut lapack_complex_float, ldv: *const lapack_int,
                   cwork: *mut lapack_complex_float, lwork: *const lapack_int,
                   work: *mut c_float, lrwork: *const lapack_int, iwork: *mut lapack_int,
                   info: *mut lapack_int);
    pub fn zgejsv_(joba: *const c_char, jobu: *const c_char, jobv: *const c_char,
                   jobr: *const c_char, jobt: *const c_char, jobp: *const c_char,
                   m: *const lapack_int, n: *const lapack_int, a: *mut lapack_complex_double,
                   lda: *const lapack_int, sva: *mut c_double, u: *mut lapack_complex_double,
                   ldu: *const lapack_int, v: *mut lapack_complex_double,
                   ldv: *const lapack_int, cwork: *mut lapack_complex_double,
                   lwork: *const lapack_int, work: *mut c_double, lrwork: *const lapack_int,
                   iwork: *mut lapack_int, info: *mut lapack_int);

    pub fn dgesvj_(joba: *const c_char, jobu: *const c_char, jobv: *const c_char,
                   m: *const lapack_int, n: *const lapack_int, a: *mut c_double,
                   lda: *const lapack_int, sva: *mut c_double, mv: *const lapack_int,
                   v: *mut c_double, ldv: *const lapack_int, work: *mut c_double,
                   lwork: *const lapack_int, info: *mut lapack_int);
    pub fn sgesvj_(joba: *const c_char, jobu: *const c_char, jobv: *const c_char,
                   m: *const lapack_int, n: *const lapack_int, a: *mut c_float,
                   lda: *const lapack_int, sva: *mut c_float, mv: *const lapack_int,
                   v: *mut c_float, ldv: *const lapack_int, work: *mut c_float,
                   lwork: *const lapack_int, info: *mut lapack_int);
    pub fn cgesvj_(joba: *const c_char, jobu: *const c_char, jobv: *const c_char,
                   m: *const lapack_int, n: *const lapack_int, a: *mut lapack_complex_float,
                   lda: *const lapack_int, sva: *mut c_float, mv: *const lapack_int,
                   v: *mut lapack_complex_float, ldv: *const lapack_int,
                   cwork: *mut lapack_complex_float, lwork: *const lapack_int,
                   rwork: *mut c_float, lrwork: *const lapack_int, info: *mut lapack_int);
    pub fn zgesvj_(joba: *const c_char, jobu: *const c_char, jobv: *const c_char,
                   m: *const lapack_int, n: *const lapack_int, a: *mut lapack_complex_double,
                   lda: *const lapack_int, sva: *mut c_double, mv: *const lapack_int,
                   v: *mut lapack_complex_double, ldv: *const lapack_int,
                   cwork: *mut lapack_complex_double, lwork: *const lapack_int,
                   rwork: *mut c_double, lrwork: *const lapack_int, info: *mut lapack_int);

    pub fn sggsvd_(jobu: *const c_char, jobv: *const c_char, jobq: *const c_char,
                   m: *const lapack_int, n: *const lapack_int, p: *const lapack_int,
                   k: *mut lapack_int, l: *mut lapack_int, a: *mut c_float,
                   lda: *const lapack_int, b: *mut c_float, ldb: *const lapack_int,
                   alpha: *mut c_float, beta: *mut c_float, u: *mut c_float,
                   ldu: *const lapack_int, v: *mut c_float, ldv: *const lapack_int,
                   q: *mut c_float, ldq: *const lapack_int, work: *mut c_float,
                   iwork: *mut lapack_int, info: *mut lapack_int);
    pub fn dggsvd_(jobu: *const c_char, jobv: *const c_char, jobq: *const c_char,
                   m: *const lapack_int, n: *const lapack_int, p: *const lapack_int,
                   k: *mut lapack_int, l: *mut lapack_int, a: *mut c_double,
                   lda: *const lapack_int, b: *mut c_double, ldb: *const lapack_int,
                   alpha: *mut c_double, beta: *mut c_double, u: *mut c_double,
                   ldu: *const lapack_int, v: *mut c_double, ldv: *const lapack_int,
                   q: *mut c_double, ldq: *const lapack_int, work: *mut c_double,
                   iwork: *mut lapack_int, info: *mut lapack_int);
    pub fn cggsvd_(jobu: *const c_char, jobv: *const c_char, jobq: *const c_char,
                   m: *const lapack_int, n: *const lapack_int, p: *const lapack_int,
                   k: *mut lapack_int, l: *mut lapack_int, a: *mut lapack_complex_float,
                   lda: *const lapack_int, b: *mut lapack_complex_float, ldb: *const lapack_int,
                   alpha: *mut c_float, beta: *mut c_float, u: *mut lapack_complex_float,
                   ldu: *const lapack_int, v: *mut lapack_complex_float, ldv: *const lapack_int,
                   q: *mut lapack_complex_float, ldq: *const lapack_int,
                   work: *mut lapack_complex_float, rwork: *mut c_float, iwork: *mut lapack_int,
                   info: *mut lapack_int);
    pub fn zggsvd_(jobu: *const c_char, jobv: *const c_char, jobq: *const c_char,
                   m: *const lapack_int, n: *const lapack_int, p: *const lapack_int,
                   k: *mut lapack_int, l: *mut lapack_int, a: *mut lapack_complex_double,
                   lda: *const lapack_int, b: *mut lapack_complex_double,
                   ldb: *const lapack_int, alpha: *mut c_double, beta: *mut c_double,
                   u: *mut lapack_complex_double, ldu: *const lapack_int,
                   v: *mut lapack_complex_double, ldv: *const lapack_int,
                   q: *mut lapack_complex_double, ldq: *const lapack_int,
                   work: *mut lapack_complex_double, rwork: *mut c_double,
                   iwork: *mut lapack_int, info: *mut lapack_int);

    pub fn sggsvd3_(jobu: *const c_char, jobv: *const c_char, jobq: *const c_char,
                    m: *const lapack_int, n: *const lapack_int, p: *const lapack_int,
                    k: *mut lapack_int, l: *mut lapack_int, a: *mut c_float,
                    lda: *const lapack_int, b: *mut c_float, ldb: *const lapack_int,
                    alpha: *mut c_float, beta: *mut c_float, u: *mut c_float,
                    ldu: *const lapack_int, v: *mut c_float, ldv: *const lapack_int,
                    q: *mut c_float, ldq: *const lapack_int, work: *mut c_float,
                    lwork: *const lapack_int, iwork: *mut lapack_int, info: *mut lapack_int);
    pub fn dggsvd3_(jobu: *const c_char, jobv: *const c_char, jobq: *const c_char,
                    m: *const lapack_int, n: *const lapack_int, p: *const lapack_int,
                    k: *mut lapack_int, l: *mut lapack_int, a: *mut c_double,
                    lda: *const lapack_int, b: *mut c_double, ldb: *const lapack_int,
                    alpha: *mut c_double, beta: *mut c_double, u: *mut c_double,
                    ldu: *const lapack_int, v: *mut c_double, ldv: *const lapack_int,
                    q: *mut c_double, ldq: *const lapack_int, work: *mut c_double,
                    lwork: *const lapack_int, iwork: *mut lapack_int, info: *mut lapack_int);
    pub fn cggsvd3_(jobu: *const c_char, jobv: *const c_char, jobq: *const c_char,
                    m: *const lapack_int, n: *const lapack_int, p: *const lapack_int,
                    k: *mut lapack_int, l: *mut lapack_int, a: *mut lapack_complex_float,
                    lda: *const lapack_int, b: *mut lapack_complex_float,
                    ldb: *const lapack_int, alpha: *mut c_float, beta: *mut c_float,
                    u: *mut lapack_complex_float, ldu: *const lapack_int,
                    v: *mut lapack_complex_float, ldv: *const lapack_int,
                    q: *mut lapack_complex_float, ldq: *const lapack_int,
                    work: *mut lapack_complex_float, lwork: *const lapack_int,
                    rwork: *mut c_float, iwork: *mut lapack_int, info: *mut lapack_int);
    pub fn zggsvd3_(jobu: *const c_char, jobv: *const c_char, jobq: *const c_char,
                    m: *const lapack_int, n: *const lapack_int, p: *const lapack_int,
                    k: *mut lapack_int, l: *mut lapack_int, a: *mut lapack_complex_double,
                    lda: *const lapack_int, b: *mut lapack_complex_double,
                    ldb: *const lapack_int, alpha: *mut c_double, beta: *mut c_double,
                    u: *mut lapack_complex_double, ldu: *const lapack_int,
                    v: *mut lapack_complex_double, ldv: *const lapack_int,
                    q: *mut lapack_complex_double, ldq: *const lapack_int,
                    work: *mut lapack_complex_double, lwork: *const lapack_int,
                    rwork: *mut c_double, iwork: *mut lapack_int, info: *mut lapack_int);

    pub fn ssygv_(itype: *const lapack_int, jobz: *const c_char, uplo: *const c_char,
                  n: *const lapack_int, a: *mut c_float, lda: *const lapack_int,
                  b: *mut c_float, ldb: *const lapack_int, w: *mut c_float, work: *mut c_float,
                  lwork: *const lapack_int, info: *mut lapack_int);
    pub fn dsygv_(itype: *const lapack_int, jobz: *const c_char, uplo: *const c_char,
                  n: *const lapack_int, a: *mut c_double, lda: *const lapack_int,
                  b: *mut c_double, ldb: *const lapack_int, w: *mut c_double,
                  work: *mut c_double, lwork: *const lapack_int, info: *mut lapack_int);
    pub fn chegv_(itype: *const lapack_int, jobz: *const c_char, uplo: *const c_char,
                  n: *const lapack_int, a: *mut lapack_complex_float, lda: *const lapack_int,
                  b: *mut lapack_complex_float, ldb: *const lapack_int, w: *mut c_float,
                  work: *mut lapack_complex_float, lwork: *const lapack_int,
                  rwork: *mut c_float, info: *mut lapack_int);
    pub fn zhegv_(itype: *const lapack_int, jobz: *const c_char, uplo: *const c_char,
                  n: *const lapack_int, a: *mut lapack_complex_double, lda: *const lapack_int,
                  b: *mut lapack_complex_double, ldb: *const lapack_int, w: *mut c_double,
                  work: *mut lapack_complex_double, lwork: *const lapack_int,
                  rwork: *mut c_double, info: *mut lapack_int);

    pub fn ssygvd_(itype: *const lapack_int, jobz: *const c_char, uplo: *const c_char,
                   n: *const lapack_int, a: *mut c_float, lda: *const lapack_int,
                   b: *mut c_float, ldb: *const lapack_int, w: *mut c_float, work: *mut c_float,
                   lwork: *const lapack_int, iwork: *mut lapack_int, liwork: *const lapack_int,
                   info: *mut lapack_int);
    pub fn dsygvd_(itype: *const lapack_int, jobz: *const c_char, uplo: *const c_char,
                   n: *const lapack_int, a: *mut c_double, lda: *const lapack_int,
                   b: *mut c_double, ldb: *const lapack_int, w: *mut c_double,
                   work: *mut c_double, lwork: *const lapack_int, iwork: *mut lapack_int,
                   liwork: *const lapack_int, info: *mut lapack_int);
    pub fn chegvd_(itype: *const lapack_int, jobz: *const c_char, uplo: *const c_char,
                   n: *const lapack_int, a: *mut lapack_complex_float, lda: *const lapack_int,
                   b: *mut lapack_complex_float, ldb: *const lapack_int, w: *mut c_float,
                   work: *mut lapack_complex_float, lwork: *const lapack_int,
                   rwork: *mut c_float, lrwork: *const lapack_int, iwork: *mut lapack_int,
                   liwork: *const lapack_int, info: *mut lapack_int);
    pub fn zhegvd_(itype: *const lapack_int, jobz: *const c_char, uplo: *const c_char,
                   n: *const lapack_int, a: *mut lapack_complex_double, lda: *const lapack_int,
                   b: *mut lapack_complex_double, ldb: *const lapack_int, w: *mut c_double,
                   work: *mut lapack_complex_double, lwork: *const lapack_int,
                   rwork: *mut c_double, lrwork: *const lapack_int, iwork: *mut lapack_int,
                   liwork: *const lapack_int, info: *mut lapack_int);

    pub fn ssygvx_(itype: *const lapack_int, jobz: *const c_char, range: *const c_char,
                   uplo: *const c_char, n: *const lapack_int, a: *mut c_float,
                   lda: *const lapack_int, b: *mut c_float, ldb: *const lapack_int,
                   vl: *const c_float, vu: *const c_float, il: *const lapack_int,
                   iu: *const lapack_int, abstol: *const c_float, m: *mut lapack_int,
                   w: *mut c_float, z: *mut c_float, ldz: *const lapack_int, work: *mut c_float,
                   lwork: *const lapack_int, iwork: *mut lapack_int, ifail: *mut lapack_int,
                   info: *mut lapack_int);
    pub fn dsygvx_(itype: *const lapack_int, jobz: *const c_char, range: *const c_char,
                   uplo: *const c_char, n: *const lapack_int, a: *mut c_double,
                   lda: *const lapack_int, b: *mut c_double, ldb: *const lapack_int,
                   vl: *const c_double, vu: *const c_double, il: *const lapack_int,
                   iu: *const lapack_int, abstol: *const c_double, m: *mut lapack_int,
                   w: *mut c_double, z: *mut c_double, ldz: *const lapack_int,
                   work: *mut c_double, lwork: *const lapack_int, iwork: *mut lapack_int,
                   ifail: *mut lapack_int, info: *mut lapack_int);
    pub fn chegvx_(itype: *const lapack_int, jobz: *const c_char, range: *const c_char,
                   uplo: *const c_char, n: *const lapack_int, a: *mut lapack_complex_float,
                   lda: *const lapack_int, b: *mut lapack_complex_float, ldb: *const lapack_int,
                   vl: *const c_float, vu: *const c_float, il: *const lapack_int,
                   iu: *const lapack_int, abstol: *const c_float, m: *mut lapack_int,
                   w: *mut c_float, z: *mut lapack_complex_float, ldz: *const lapack_int,
                   work: *mut lapack_complex_float, lwork: *const lapack_int,
                   rwork: *mut c_float, iwork: *mut lapack_int, ifail: *mut lapack_int,
                   info: *mut lapack_int);
    pub fn zhegvx_(itype: *const lapack_int, jobz: *const c_char, range: *const c_char,
                   uplo: *const c_char, n: *const lapack_int, a: *mut lapack_complex_double,
                   lda: *const lapack_int, b: *mut lapack_complex_double,
                   ldb: *const lapack_int, vl: *const c_double, vu: *const c_double,
                   il: *const lapack_int, iu: *const lapack_int, abstol: *const c_double,
                   m: *mut lapack_int, w: *mut c_double, z: *mut lapack_complex_double,
                   ldz: *const lapack_int, work: *mut lapack_complex_double,
                   lwork: *const lapack_int, rwork: *mut c_double, iwork: *mut lapack_int,
                   ifail: *mut lapack_int, info: *mut lapack_int);

    pub fn sspgv_(itype: *const lapack_int, jobz: *const c_char, uplo: *const c_char,
                  n: *const lapack_int, ap: *mut c_float, bp: *mut c_float, w: *mut c_float,
                  z: *mut c_float, ldz: *const lapack_int, work: *mut c_float,
                  info: *mut lapack_int);
    pub fn dspgv_(itype: *const lapack_int, jobz: *const c_char, uplo: *const c_char,
                  n: *const lapack_int, ap: *mut c_double, bp: *mut c_double, w: *mut c_double,
                  z: *mut c_double, ldz: *const lapack_int, work: *mut c_double,
                  info: *mut lapack_int);
    pub fn chpgv_(itype: *const lapack_int, jobz: *const c_char, uplo: *const c_char,
                  n: *const lapack_int, ap: *mut lapack_complex_float,
                  bp: *mut lapack_complex_float, w: *mut c_float, z: *mut lapack_complex_float,
                  ldz: *const lapack_int, work: *mut lapack_complex_float, rwork: *mut c_float,
                  info: *mut lapack_int);
    pub fn zhpgv_(itype: *const lapack_int, jobz: *const c_char, uplo: *const c_char,
                  n: *const lapack_int, ap: *mut lapack_complex_double,
                  bp: *mut lapack_complex_double, w: *mut c_double,
                  z: *mut lapack_complex_double, ldz: *const lapack_int,
                  work: *mut lapack_complex_double, rwork: *mut c_double, info: *mut lapack_int);

    pub fn sspgvd_(itype: *const lapack_int, jobz: *const c_char, uplo: *const c_char,
                   n: *const lapack_int, ap: *mut c_float, bp: *mut c_float, w: *mut c_float,
                   z: *mut c_float, ldz: *const lapack_int, work: *mut c_float,
                   lwork: *const lapack_int, iwork: *mut lapack_int, liwork: *const lapack_int,
                   info: *mut lapack_int);
    pub fn dspgvd_(itype: *const lapack_int, jobz: *const c_char, uplo: *const c_char,
                   n: *const lapack_int, ap: *mut c_double, bp: *mut c_double, w: *mut c_double,
                   z: *mut c_double, ldz: *const lapack_int, work: *mut c_double,
                   lwork: *const lapack_int, iwork: *mut lapack_int, liwork: *const lapack_int,
                   info: *mut lapack_int);
    pub fn chpgvd_(itype: *const lapack_int, jobz: *const c_char, uplo: *const c_char,
                   n: *const lapack_int, ap: *mut lapack_complex_float,
                   bp: *mut lapack_complex_float, w: *mut c_float, z: *mut lapack_complex_float,
                   ldz: *const lapack_int, work: *mut lapack_complex_float,
                   lwork: *const lapack_int, rwork: *mut c_float, lrwork: *const lapack_int,
                   iwork: *mut lapack_int, liwork: *const lapack_int, info: *mut lapack_int);
    pub fn zhpgvd_(itype: *const lapack_int, jobz: *const c_char, uplo: *const c_char,
                   n: *const lapack_int, ap: *mut lapack_complex_double,
                   bp: *mut lapack_complex_double, w: *mut c_double,
                   z: *mut lapack_complex_double, ldz: *const lapack_int,
                   work: *mut lapack_complex_double, lwork: *const lapack_int,
                   rwork: *mut c_double, lrwork: *const lapack_int, iwork: *mut lapack_int,
                   liwork: *const lapack_int, info: *mut lapack_int);

    pub fn sspgvx_(itype: *const lapack_int, jobz: *const c_char, range: *const c_char,
                   uplo: *const c_char, n: *const lapack_int, ap: *mut c_float,
                   bp: *mut c_float, vl: *const c_float, vu: *const c_float,
                   il: *const lapack_int, iu: *const lapack_int, abstol: *const c_float,
                   m: *mut lapack_int, w: *mut c_float, z: *mut c_float, ldz: *const lapack_int,
                   work: *mut c_float, iwork: *mut lapack_int, ifail: *mut lapack_int,
                   info: *mut lapack_int);
    pub fn dspgvx_(itype: *const lapack_int, jobz: *const c_char, range: *const c_char,
                   uplo: *const c_char, n: *const lapack_int, ap: *mut c_double,
                   bp: *mut c_double, vl: *const c_double, vu: *const c_double,
                   il: *const lapack_int, iu: *const lapack_int, abstol: *const c_double,
                   m: *mut lapack_int, w: *mut c_double, z: *mut c_double,
                   ldz: *const lapack_int, work: *mut c_double, iwork: *mut lapack_int,
                   ifail: *mut lapack_int, info: *mut lapack_int);
    pub fn chpgvx_(itype: *const lapack_int, jobz: *const c_char, range: *const c_char,
                   uplo: *const c_char, n: *const lapack_int, ap: *mut lapack_complex_float,
                   bp: *mut lapack_complex_float, vl: *const c_float, vu: *const c_float,
                   il: *const lapack_int, iu: *const lapack_int, abstol: *const c_float,
                   m: *mut lapack_int, w: *mut c_float, z: *mut lapack_complex_float,
                   ldz: *const lapack_int, work: *mut lapack_complex_float, rwork: *mut c_float,
                   iwork: *mut lapack_int, ifail: *mut lapack_int, info: *mut lapack_int);
    pub fn zhpgvx_(itype: *const lapack_int, jobz: *const c_char, range: *const c_char,
                   uplo: *const c_char, n: *const lapack_int, ap: *mut lapack_complex_double,
                   bp: *mut lapack_complex_double, vl: *const c_double, vu: *const c_double,
                   il: *const lapack_int, iu: *const lapack_int, abstol: *const c_double,
                   m: *mut lapack_int, w: *mut c_double, z: *mut lapack_complex_double,
                   ldz: *const lapack_int, work: *mut lapack_complex_double,
                   rwork: *mut c_double, iwork: *mut lapack_int, ifail: *mut lapack_int,
                   info: *mut lapack_int);

    pub fn ssbgv_(jobz: *const c_char, uplo: *const c_char, n: *const lapack_int,
                  ka: *const lapack_int, kb: *const lapack_int, ab: *mut c_float,
                  ldab: *const lapack_int, bb: *mut c_float, ldbb: *const lapack_int,
                  w: *mut c_float, z: *mut c_float, ldz: *const lapack_int, work: *mut c_float,
                  info: *mut lapack_int);
    pub fn dsbgv_(jobz: *const c_char, uplo: *const c_char, n: *const lapack_int,
                  ka: *const lapack_int, kb: *const lapack_int, ab: *mut c_double,
                  ldab: *const lapack_int, bb: *mut c_double, ldbb: *const lapack_int,
                  w: *mut c_double, z: *mut c_double, ldz: *const lapack_int,
                  work: *mut c_double, info: *mut lapack_int);
    pub fn chbgv_(jobz: *const c_char, uplo: *const c_char, n: *const lapack_int,
                  ka: *const lapack_int, kb: *const lapack_int, ab: *mut lapack_complex_float,
                  ldab: *const lapack_int, bb: *mut lapack_complex_float,
                  ldbb: *const lapack_int, w: *mut c_float, z: *mut lapack_complex_float,
                  ldz: *const lapack_int, work: *mut lapack_complex_float, rwork: *mut c_float,
                  info: *mut lapack_int);
    pub fn zhbgv_(jobz: *const c_char, uplo: *const c_char, n: *const lapack_int,
                  ka: *const lapack_int, kb: *const lapack_int, ab: *mut lapack_complex_double,
                  ldab: *const lapack_int, bb: *mut lapack_complex_double,
                  ldbb: *const lapack_int, w: *mut c_double, z: *mut lapack_complex_double,
                  ldz: *const lapack_int, work: *mut lapack_complex_double,
                  rwork: *mut c_double, info: *mut lapack_int);

    pub fn ssbgvd_(jobz: *const c_char, uplo: *const c_char, n: *const lapack_int,
                   ka: *const lapack_int, kb: *const lapack_int, ab: *mut c_float,
                   ldab: *const lapack_int, bb: *mut c_float, ldbb: *const lapack_int,
                   w: *mut c_float, z: *mut c_float, ldz: *const lapack_int, work: *mut c_float,
                   lwork: *const lapack_int, iwork: *mut lapack_int, liwork: *const lapack_int,
                   info: *mut lapack_int);
    pub fn dsbgvd_(jobz: *const c_char, uplo: *const c_char, n: *const lapack_int,
                   ka: *const lapack_int, kb: *const lapack_int, ab: *mut c_double,
                   ldab: *const lapack_int, bb: *mut c_double, ldbb: *const lapack_int,
                   w: *mut c_double, z: *mut c_double, ldz: *const lapack_int,
                   work: *mut c_double, lwork: *const lapack_int, iwork: *mut lapack_int,
                   liwork: *const lapack_int, info: *mut lapack_int);
    pub fn chbgvd_(jobz: *const c_char, uplo: *const c_char, n: *const lapack_int,
                   ka: *const lapack_int, kb: *const lapack_int, ab: *mut lapack_complex_float,
                   ldab: *const lapack_int, bb: *mut lapack_complex_float,
                   ldbb: *const lapack_int, w: *mut c_float, z: *mut lapack_complex_float,
                   ldz: *const lapack_int, work: *mut lapack_complex_float,
                   lwork: *const lapack_int, rwork: *mut c_float, lrwork: *const lapack_int,
                   iwork: *mut lapack_int, liwork: *const lapack_int, info: *mut lapack_int);
    pub fn zhbgvd_(jobz: *const c_char, uplo: *const c_char, n: *const lapack_int,
                   ka: *const lapack_int, kb: *const lapack_int, ab: *mut lapack_complex_double,
                   ldab: *const lapack_int, bb: *mut lapack_complex_double,
                   ldbb: *const lapack_int, w: *mut c_double, z: *mut lapack_complex_double,
                   ldz: *const lapack_int, work: *mut lapack_complex_double,
                   lwork: *const lapack_int, rwork: *mut c_double, lrwork: *const lapack_int,
                   iwork: *mut lapack_int, liwork: *const lapack_int, info: *mut lapack_int);

    pub fn ssbgvx_(jobz: *const c_char, range: *const c_char, uplo: *const c_char,
                   n: *const lapack_int, ka: *const lapack_int, kb: *const lapack_int,
                   ab: *mut c_float, ldab: *const lapack_int, bb: *mut c_float,
                   ldbb: *const lapack_int, q: *mut c_float, ldq: *const lapack_int,
                   vl: *const c_float, vu: *const c_float, il: *const lapack_int,
                   iu: *const lapack_int, abstol: *const c_float, m: *mut lapack_int,
                   w: *mut c_float, z: *mut c_float, ldz: *const lapack_int, work: *mut c_float,
                   iwork: *mut lapack_int, ifail: *mut lapack_int, info: *mut lapack_int);
    pub fn dsbgvx_(jobz: *const c_char, range: *const c_char, uplo: *const c_char,
                   n: *const lapack_int, ka: *const lapack_int, kb: *const lapack_int,
                   ab: *mut c_double, ldab: *const lapack_int, bb: *mut c_double,
                   ldbb: *const lapack_int, q: *mut c_double, ldq: *const lapack_int,
                   vl: *const c_double, vu: *const c_double, il: *const lapack_int,
                   iu: *const lapack_int, abstol: *const c_double, m: *mut lapack_int,
                   w: *mut c_double, z: *mut c_double, ldz: *const lapack_int,
                   work: *mut c_double, iwork: *mut lapack_int, ifail: *mut lapack_int,
                   info: *mut lapack_int);
    pub fn chbgvx_(jobz: *const c_char, range: *const c_char, uplo: *const c_char,
                   n: *const lapack_int, ka: *const lapack_int, kb: *const lapack_int,
                   ab: *mut lapack_complex_float, ldab: *const lapack_int,
                   bb: *mut lapack_complex_float, ldbb: *const lapack_int,
                   q: *mut lapack_complex_float, ldq: *const lapack_int, vl: *const c_float,
                   vu: *const c_float, il: *const lapack_int, iu: *const lapack_int,
                   abstol: *const c_float, m: *mut lapack_int, w: *mut c_float,
                   z: *mut lapack_complex_float, ldz: *const lapack_int,
                   work: *mut lapack_complex_float, rwork: *mut c_float, iwork: *mut lapack_int,
                   ifail: *mut lapack_int, info: *mut lapack_int);
    pub fn zhbgvx_(jobz: *const c_char, range: *const c_char, uplo: *const c_char,
                   n: *const lapack_int, ka: *const lapack_int, kb: *const lapack_int,
                   ab: *mut lapack_complex_double, ldab: *const lapack_int,
                   bb: *mut lapack_complex_double, ldbb: *const lapack_int,
                   q: *mut lapack_complex_double, ldq: *const lapack_int, vl: *const c_double,
                   vu: *const c_double, il: *const lapack_int, iu: *const lapack_int,
                   abstol: *const c_double, m: *mut lapack_int, w: *mut c_double,
                   z: *mut lapack_complex_double, ldz: *const lapack_int,
                   work: *mut lapack_complex_double, rwork: *mut c_double,
                   iwork: *mut lapack_int, ifail: *mut lapack_int, info: *mut lapack_int);

    pub fn sgges_(jobvsl: *const c_char, jobvsr: *const c_char, sort: *const c_char,
                  selctg: LAPACK_S_SELECT3, n: *const lapack_int, a: *mut c_float,
                  lda: *const lapack_int, b: *mut c_float, ldb: *const lapack_int,
                  sdim: *mut lapack_int, alphar: *mut c_float, alphai: *mut c_float,
                  beta: *mut c_float, vsl: *mut c_float, ldvsl: *const lapack_int,
                  vsr: *mut c_float, ldvsr: *const lapack_int, work: *mut c_float,
                  lwork: *const lapack_int, bwork: *mut lapack_logical, info: *mut lapack_int);
    pub fn dgges_(jobvsl: *const c_char, jobvsr: *const c_char, sort: *const c_char,
                  selctg: LAPACK_D_SELECT3, n: *const lapack_int, a: *mut c_double,
                  lda: *const lapack_int, b: *mut c_double, ldb: *const lapack_int,
                  sdim: *mut lapack_int, alphar: *mut c_double, alphai: *mut c_double,
                  beta: *mut c_double, vsl: *mut c_double, ldvsl: *const lapack_int,
                  vsr: *mut c_double, ldvsr: *const lapack_int, work: *mut c_double,
                  lwork: *const lapack_int, bwork: *mut lapack_logical, info: *mut lapack_int);
    pub fn cgges_(jobvsl: *const c_char, jobvsr: *const c_char, sort: *const c_char,
                  selctg: LAPACK_C_SELECT2, n: *const lapack_int, a: *mut lapack_complex_float,
                  lda: *const lapack_int, b: *mut lapack_complex_float, ldb: *const lapack_int,
                  sdim: *mut lapack_int, alpha: *mut lapack_complex_float,
                  beta: *mut lapack_complex_float, vsl: *mut lapack_complex_float,
                  ldvsl: *const lapack_int, vsr: *mut lapack_complex_float,
                  ldvsr: *const lapack_int, work: *mut lapack_complex_float,
                  lwork: *const lapack_int, rwork: *mut c_float, bwork: *mut lapack_logical,
                  info: *mut lapack_int);
    pub fn zgges_(jobvsl: *const c_char, jobvsr: *const c_char, sort: *const c_char,
                  selctg: LAPACK_Z_SELECT2, n: *const lapack_int, a: *mut lapack_complex_double,
                  lda: *const lapack_int, b: *mut lapack_complex_double, ldb: *const lapack_int,
                  sdim: *mut lapack_int, alpha: *mut lapack_complex_double,
                  beta: *mut lapack_complex_double, vsl: *mut lapack_complex_double,
                  ldvsl: *const lapack_int, vsr: *mut lapack_complex_double,
                  ldvsr: *const lapack_int, work: *mut lapack_complex_double,
                  lwork: *const lapack_int, rwork: *mut c_double, bwork: *mut lapack_logical,
                  info: *mut lapack_int);

    pub fn sgges3_(jobvsl: *const c_char, jobvsr: *const c_char, sort: *const c_char,
                   selctg: LAPACK_S_SELECT3, n: *const lapack_int, a: *mut c_float,
                   lda: *const lapack_int, b: *mut c_float, ldb: *const lapack_int,
                   sdim: *mut lapack_int, alphar: *mut c_float, alphai: *mut c_float,
                   beta: *mut c_float, vsl: *mut c_float, ldvsl: *const lapack_int,
                   vsr: *mut c_float, ldvsr: *const lapack_int, work: *mut c_float,
                   lwork: *const lapack_int, bwork: *mut lapack_logical, info: *mut lapack_int);
    pub fn dgges3_(jobvsl: *const c_char, jobvsr: *const c_char, sort: *const c_char,
                   selctg: LAPACK_D_SELECT3, n: *const lapack_int, a: *mut c_double,
                   lda: *const lapack_int, b: *mut c_double, ldb: *const lapack_int,
                   sdim: *mut lapack_int, alphar: *mut c_double, alphai: *mut c_double,
                   beta: *mut c_double, vsl: *mut c_double, ldvsl: *const lapack_int,
                   vsr: *mut c_double, ldvsr: *const lapack_int, work: *mut c_double,
                   lwork: *const lapack_int, bwork: *mut lapack_logical, info: *mut lapack_int);
    pub fn cgges3_(jobvsl: *const c_char, jobvsr: *const c_char, sort: *const c_char,
                   selctg: LAPACK_C_SELECT2, n: *const lapack_int, a: *mut lapack_complex_float,
                   lda: *const lapack_int, b: *mut lapack_complex_float, ldb: *const lapack_int,
                   sdim: *mut lapack_int, alpha: *mut lapack_complex_float,
                   beta: *mut lapack_complex_float, vsl: *mut lapack_complex_float,
                   ldvsl: *const lapack_int, vsr: *mut lapack_complex_float,
                   ldvsr: *const lapack_int, work: *mut lapack_complex_float,
                   lwork: *const lapack_int, rwork: *mut c_float, bwork: *mut lapack_logical,
                   info: *mut lapack_int);
    pub fn zgges3_(jobvsl: *const c_char, jobvsr: *const c_char, sort: *const c_char,
                   selctg: LAPACK_Z_SELECT2, n: *const lapack_int,
                   a: *mut lapack_complex_double, lda: *const lapack_int,
                   b: *mut lapack_complex_double, ldb: *const lapack_int, sdim: *mut lapack_int,
                   alpha: *mut lapack_complex_double, beta: *mut lapack_complex_double,
                   vsl: *mut lapack_complex_double, ldvsl: *const lapack_int,
                   vsr: *mut lapack_complex_double, ldvsr: *const lapack_int,
                   work: *mut lapack_complex_double, lwork: *const lapack_int,
                   rwork: *mut c_double, bwork: *mut lapack_logical, info: *mut lapack_int);

    pub fn sggesx_(jobvsl: *const c_char, jobvsr: *const c_char, sort: *const c_char,
                   selctg: LAPACK_S_SELECT3, sense: *const c_char, n: *const lapack_int,
                   a: *mut c_float, lda: *const lapack_int, b: *mut c_float,
                   ldb: *const lapack_int, sdim: *mut lapack_int, alphar: *mut c_float,
                   alphai: *mut c_float, beta: *mut c_float, vsl: *mut c_float,
                   ldvsl: *const lapack_int, vsr: *mut c_float, ldvsr: *const lapack_int,
                   rconde: *mut c_float, rcondv: *mut c_float, work: *mut c_float,
                   lwork: *const lapack_int, iwork: *mut lapack_int, liwork: *const lapack_int,
                   bwork: *mut lapack_logical, info: *mut lapack_int);
    pub fn dggesx_(jobvsl: *const c_char, jobvsr: *const c_char, sort: *const c_char,
                   selctg: LAPACK_D_SELECT3, sense: *const c_char, n: *const lapack_int,
                   a: *mut c_double, lda: *const lapack_int, b: *mut c_double,
                   ldb: *const lapack_int, sdim: *mut lapack_int, alphar: *mut c_double,
                   alphai: *mut c_double, beta: *mut c_double, vsl: *mut c_double,
                   ldvsl: *const lapack_int, vsr: *mut c_double, ldvsr: *const lapack_int,
                   rconde: *mut c_double, rcondv: *mut c_double, work: *mut c_double,
                   lwork: *const lapack_int, iwork: *mut lapack_int, liwork: *const lapack_int,
                   bwork: *mut lapack_logical, info: *mut lapack_int);
    pub fn cggesx_(jobvsl: *const c_char, jobvsr: *const c_char, sort: *const c_char,
                   selctg: LAPACK_C_SELECT2, sense: *const c_char, n: *const lapack_int,
                   a: *mut lapack_complex_float, lda: *const lapack_int,
                   b: *mut lapack_complex_float, ldb: *const lapack_int, sdim: *mut lapack_int,
                   alpha: *mut lapack_complex_float, beta: *mut lapack_complex_float,
                   vsl: *mut lapack_complex_float, ldvsl: *const lapack_int,
                   vsr: *mut lapack_complex_float, ldvsr: *const lapack_int,
                   rconde: *mut c_float, rcondv: *mut c_float, work: *mut lapack_complex_float,
                   lwork: *const lapack_int, rwork: *mut c_float, iwork: *mut lapack_int,
                   liwork: *const lapack_int, bwork: *mut lapack_logical, info: *mut lapack_int);
    pub fn zggesx_(jobvsl: *const c_char, jobvsr: *const c_char, sort: *const c_char,
                   selctg: LAPACK_Z_SELECT2, sense: *const c_char, n: *const lapack_int,
                   a: *mut lapack_complex_double, lda: *const lapack_int,
                   b: *mut lapack_complex_double, ldb: *const lapack_int, sdim: *mut lapack_int,
                   alpha: *mut lapack_complex_double, beta: *mut lapack_complex_double,
                   vsl: *mut lapack_complex_double, ldvsl: *const lapack_int,
                   vsr: *mut lapack_complex_double, ldvsr: *const lapack_int,
                   rconde: *mut c_double, rcondv: *mut c_double,
                   work: *mut lapack_complex_double, lwork: *const lapack_int,
                   rwork: *mut c_double, iwork: *mut lapack_int, liwork: *const lapack_int,
                   bwork: *mut lapack_logical, info: *mut lapack_int);

    pub fn sggev_(jobvl: *const c_char, jobvr: *const c_char, n: *const lapack_int,
                  a: *mut c_float, lda: *const lapack_int, b: *mut c_float,
                  ldb: *const lapack_int, alphar: *mut c_float, alphai: *mut c_float,
                  beta: *mut c_float, vl: *mut c_float, ldvl: *const lapack_int,
                  vr: *mut c_float, ldvr: *const lapack_int, work: *mut c_float,
                  lwork: *const lapack_int, info: *mut lapack_int);
    pub fn dggev_(jobvl: *const c_char, jobvr: *const c_char, n: *const lapack_int,
                  a: *mut c_double, lda: *const lapack_int, b: *mut c_double,
                  ldb: *const lapack_int, alphar: *mut c_double, alphai: *mut c_double,
                  beta: *mut c_double, vl: *mut c_double, ldvl: *const lapack_int,
                  vr: *mut c_double, ldvr: *const lapack_int, work: *mut c_double,
                  lwork: *const lapack_int, info: *mut lapack_int);
    pub fn cggev_(jobvl: *const c_char, jobvr: *const c_char, n: *const lapack_int,
                  a: *mut lapack_complex_float, lda: *const lapack_int,
                  b: *mut lapack_complex_float, ldb: *const lapack_int,
                  alpha: *mut lapack_complex_float, beta: *mut lapack_complex_float,
                  vl: *mut lapack_complex_float, ldvl: *const lapack_int,
                  vr: *mut lapack_complex_float, ldvr: *const lapack_int,
                  work: *mut lapack_complex_float, lwork: *const lapack_int,
                  rwork: *mut c_float, info: *mut lapack_int);
    pub fn zggev_(jobvl: *const c_char, jobvr: *const c_char, n: *const lapack_int,
                  a: *mut lapack_complex_double, lda: *const lapack_int,
                  b: *mut lapack_complex_double, ldb: *const lapack_int,
                  alpha: *mut lapack_complex_double, beta: *mut lapack_complex_double,
                  vl: *mut lapack_complex_double, ldvl: *const lapack_int,
                  vr: *mut lapack_complex_double, ldvr: *const lapack_int,
                  work: *mut lapack_complex_double, lwork: *const lapack_int,
                  rwork: *mut c_double, info: *mut lapack_int);

    pub fn sggev3_(jobvl: *const c_char, jobvr: *const c_char, n: *const lapack_int,
                   a: *mut c_float, lda: *const lapack_int, b: *mut c_float,
                   ldb: *const lapack_int, alphar: *mut c_float, alphai: *mut c_float,
                   beta: *mut c_float, vl: *mut c_float, ldvl: *const lapack_int,
                   vr: *mut c_float, ldvr: *const lapack_int, work: *mut c_float,
                   lwork: *const lapack_int, info: *mut lapack_int);
    pub fn dggev3_(jobvl: *const c_char, jobvr: *const c_char, n: *const lapack_int,
                   a: *mut c_double, lda: *const lapack_int, b: *mut c_double,
                   ldb: *const lapack_int, alphar: *mut c_double, alphai: *mut c_double,
                   beta: *mut c_double, vl: *mut c_double, ldvl: *const lapack_int,
                   vr: *mut c_double, ldvr: *const lapack_int, work: *mut c_double,
                   lwork: *const lapack_int, info: *mut lapack_int);
    pub fn cggev3_(jobvl: *const c_char, jobvr: *const c_char, n: *const lapack_int,
                   a: *mut lapack_complex_float, lda: *const lapack_int,
                   b: *mut lapack_complex_float, ldb: *const lapack_int,
                   alpha: *mut lapack_complex_float, beta: *mut lapack_complex_float,
                   vl: *mut lapack_complex_float, ldvl: *const lapack_int,
                   vr: *mut lapack_complex_float, ldvr: *const lapack_int,
                   work: *mut lapack_complex_float, lwork: *const lapack_int,
                   rwork: *mut c_float, info: *mut lapack_int);
    pub fn zggev3_(jobvl: *const c_char, jobvr: *const c_char, n: *const lapack_int,
                   a: *mut lapack_complex_double, lda: *const lapack_int,
                   b: *mut lapack_complex_double, ldb: *const lapack_int,
                   alpha: *mut lapack_complex_double, beta: *mut lapack_complex_double,
                   vl: *mut lapack_complex_double, ldvl: *const lapack_int,
                   vr: *mut lapack_complex_double, ldvr: *const lapack_int,
                   work: *mut lapack_complex_double, lwork: *const lapack_int,
                   rwork: *mut c_double, info: *mut lapack_int);

    pub fn sggevx_(balanc: *const c_char, jobvl: *const c_char, jobvr: *const c_char,
                   sense: *const c_char, n: *const lapack_int, a: *mut c_float,
                   lda: *const lapack_int, b: *mut c_float, ldb: *const lapack_int,
                   alphar: *mut c_float, alphai: *mut c_float, beta: *mut c_float,
                   vl: *mut c_float, ldvl: *const lapack_int, vr: *mut c_float,
                   ldvr: *const lapack_int, ilo: *mut lapack_int, ihi: *mut lapack_int,
                   lscale: *mut c_float, rscale: *mut c_float, abnrm: *mut c_float,
                   bbnrm: *mut c_float, rconde: *mut c_float, rcondv: *mut c_float,
                   work: *mut c_float, lwork: *const lapack_int, iwork: *mut lapack_int,
                   bwork: *mut lapack_logical, info: *mut lapack_int);
    pub fn dggevx_(balanc: *const c_char, jobvl: *const c_char, jobvr: *const c_char,
                   sense: *const c_char, n: *const lapack_int, a: *mut c_double,
                   lda: *const lapack_int, b: *mut c_double, ldb: *const lapack_int,
                   alphar: *mut c_double, alphai: *mut c_double, beta: *mut c_double,
                   vl: *mut c_double, ldvl: *const lapack_int, vr: *mut c_double,
                   ldvr: *const lapack_int, ilo: *mut lapack_int, ihi: *mut lapack_int,
                   lscale: *mut c_double, rscale: *mut c_double, abnrm: *mut c_double,
                   bbnrm: *mut c_double, rconde: *mut c_double, rcondv: *mut c_double,
                   work: *mut c_double, lwork: *const lapack_int, iwork: *mut lapack_int,
                   bwork: *mut lapack_logical, info: *mut lapack_int);
    pub fn cggevx_(balanc: *const c_char, jobvl: *const c_char, jobvr: *const c_char,
                   sense: *const c_char, n: *const lapack_int, a: *mut lapack_complex_float,
                   lda: *const lapack_int, b: *mut lapack_complex_float, ldb: *const lapack_int,
                   alpha: *mut lapack_complex_float, beta: *mut lapack_complex_float,
                   vl: *mut lapack_complex_float, ldvl: *const lapack_int,
                   vr: *mut lapack_complex_float, ldvr: *const lapack_int, ilo: *mut lapack_int,
                   ihi: *mut lapack_int, lscale: *mut c_float, rscale: *mut c_float,
                   abnrm: *mut c_float, bbnrm: *mut c_float, rconde: *mut c_float,
                   rcondv: *mut c_float, work: *mut lapack_complex_float,
                   lwork: *const lapack_int, rwork: *mut c_float, iwork: *mut lapack_int,
                   bwork: *mut lapack_logical, info: *mut lapack_int);
    pub fn zggevx_(balanc: *const c_char, jobvl: *const c_char, jobvr: *const c_char,
                   sense: *const c_char, n: *const lapack_int, a: *mut lapack_complex_double,
                   lda: *const lapack_int, b: *mut lapack_complex_double,
                   ldb: *const lapack_int, alpha: *mut lapack_complex_double,
                   beta: *mut lapack_complex_double, vl: *mut lapack_complex_double,
                   ldvl: *const lapack_int, vr: *mut lapack_complex_double,
                   ldvr: *const lapack_int, ilo: *mut lapack_int, ihi: *mut lapack_int,
                   lscale: *mut c_double, rscale: *mut c_double, abnrm: *mut c_double,
                   bbnrm: *mut c_double, rconde: *mut c_double, rcondv: *mut c_double,
                   work: *mut lapack_complex_double, lwork: *const lapack_int,
                   rwork: *mut c_double, iwork: *mut lapack_int, bwork: *mut lapack_logical,
                   info: *mut lapack_int);

    pub fn dsfrk_(transr: *const c_char, uplo: *const c_char, trans: *const c_char,
                  n: *const lapack_int, k: *const lapack_int, alpha: *const c_double,
                  a: *const c_double, lda: *const lapack_int, beta: *const c_double,
                  c: *mut c_double);
    pub fn ssfrk_(transr: *const c_char, uplo: *const c_char, trans: *const c_char,
                  n: *const lapack_int, k: *const lapack_int, alpha: *const c_float,
                  a: *const c_float, lda: *const lapack_int, beta: *const c_float,
                  c: *mut c_float);
    pub fn zhfrk_(transr: *const c_char, uplo: *const c_char, trans: *const c_char,
                  n: *const lapack_int, k: *const lapack_int, alpha: *const c_double,
                  a: *const lapack_complex_double, lda: *const lapack_int,
                  beta: *const c_double, c: *mut lapack_complex_double);
    pub fn chfrk_(transr: *const c_char, uplo: *const c_char, trans: *const c_char,
                  n: *const lapack_int, k: *const lapack_int, alpha: *const c_float,
                  a: *const lapack_complex_float, lda: *const lapack_int, beta: *const c_float,
                  c: *mut lapack_complex_float);

    pub fn dtfsm_(transr: *const c_char, side: *const c_char, uplo: *const c_char,
                  trans: *const c_char, diag: *const c_char, m: *const lapack_int,
                  n: *const lapack_int, alpha: *const c_double, a: *const c_double,
                  b: *mut c_double, ldb: *const lapack_int);
    pub fn stfsm_(transr: *const c_char, side: *const c_char, uplo: *const c_char,
                  trans: *const c_char, diag: *const c_char, m: *const lapack_int,
                  n: *const lapack_int, alpha: *const c_float, a: *const c_float,
                  b: *mut c_float, ldb: *const lapack_int);
    pub fn ztfsm_(transr: *const c_char, side: *const c_char, uplo: *const c_char,
                  trans: *const c_char, diag: *const c_char, m: *const lapack_int,
                  n: *const lapack_int, alpha: *const lapack_complex_double,
                  a: *const lapack_complex_double, b: *mut lapack_complex_double,
                  ldb: *const lapack_int);
    pub fn ctfsm_(transr: *const c_char, side: *const c_char, uplo: *const c_char,
                  trans: *const c_char, diag: *const c_char, m: *const lapack_int,
                  n: *const lapack_int, alpha: *const lapack_complex_float,
                  a: *const lapack_complex_float, b: *mut lapack_complex_float,
                  ldb: *const lapack_int);

    pub fn dtfttp_(transr: *const c_char, uplo: *const c_char, n: *const lapack_int,
                   arf: *const c_double, ap: *mut c_double, info: *mut lapack_int);
    pub fn stfttp_(transr: *const c_char, uplo: *const c_char, n: *const lapack_int,
                   arf: *const c_float, ap: *mut c_float, info: *mut lapack_int);
    pub fn ztfttp_(transr: *const c_char, uplo: *const c_char, n: *const lapack_int,
                   arf: *const lapack_complex_double, ap: *mut lapack_complex_double,
                   info: *mut lapack_int);
    pub fn ctfttp_(transr: *const c_char, uplo: *const c_char, n: *const lapack_int,
                   arf: *const lapack_complex_float, ap: *mut lapack_complex_float,
                   info: *mut lapack_int);

    pub fn dtfttr_(transr: *const c_char, uplo: *const c_char, n: *const lapack_int,
                   arf: *const c_double, a: *mut c_double, lda: *const lapack_int,
                   info: *mut lapack_int);
    pub fn stfttr_(transr: *const c_char, uplo: *const c_char, n: *const lapack_int,
                   arf: *const c_float, a: *mut c_float, lda: *const lapack_int,
                   info: *mut lapack_int);
    pub fn ztfttr_(transr: *const c_char, uplo: *const c_char, n: *const lapack_int,
                   arf: *const lapack_complex_double, a: *mut lapack_complex_double,
                   lda: *const lapack_int, info: *mut lapack_int);
    pub fn ctfttr_(transr: *const c_char, uplo: *const c_char, n: *const lapack_int,
                   arf: *const lapack_complex_float, a: *mut lapack_complex_float,
                   lda: *const lapack_int, info: *mut lapack_int);

    pub fn dtpttf_(transr: *const c_char, uplo: *const c_char, n: *const lapack_int,
                   ap: *const c_double, arf: *mut c_double, info: *mut lapack_int);
    pub fn stpttf_(transr: *const c_char, uplo: *const c_char, n: *const lapack_int,
                   ap: *const c_float, arf: *mut c_float, info: *mut lapack_int);
    pub fn ztpttf_(transr: *const c_char, uplo: *const c_char, n: *const lapack_int,
                   ap: *const lapack_complex_double, arf: *mut lapack_complex_double,
                   info: *mut lapack_int);
    pub fn ctpttf_(transr: *const c_char, uplo: *const c_char, n: *const lapack_int,
                   ap: *const lapack_complex_float, arf: *mut lapack_complex_float,
                   info: *mut lapack_int);

    pub fn dtpttr_(uplo: *const c_char, n: *const lapack_int, ap: *const c_double,
                   a: *mut c_double, lda: *const lapack_int, info: *mut lapack_int);
    pub fn stpttr_(uplo: *const c_char, n: *const lapack_int, ap: *const c_float,
                   a: *mut c_float, lda: *const lapack_int, info: *mut lapack_int);
    pub fn ztpttr_(uplo: *const c_char, n: *const lapack_int, ap: *const lapack_complex_double,
                   a: *mut lapack_complex_double, lda: *const lapack_int, info: *mut lapack_int);
    pub fn ctpttr_(uplo: *const c_char, n: *const lapack_int, ap: *const lapack_complex_float,
                   a: *mut lapack_complex_float, lda: *const lapack_int, info: *mut lapack_int);

    pub fn dtrttf_(transr: *const c_char, uplo: *const c_char, n: *const lapack_int,
                   a: *const c_double, lda: *const lapack_int, arf: *mut c_double,
                   info: *mut lapack_int);
    pub fn strttf_(transr: *const c_char, uplo: *const c_char, n: *const lapack_int,
                   a: *const c_float, lda: *const lapack_int, arf: *mut c_float,
                   info: *mut lapack_int);
    pub fn ztrttf_(transr: *const c_char, uplo: *const c_char, n: *const lapack_int,
                   a: *const lapack_complex_double, lda: *const lapack_int,
                   arf: *mut lapack_complex_double, info: *mut lapack_int);
    pub fn ctrttf_(transr: *const c_char, uplo: *const c_char, n: *const lapack_int,
                   a: *const lapack_complex_float, lda: *const lapack_int,
                   arf: *mut lapack_complex_float, info: *mut lapack_int);

    pub fn dtrttp_(uplo: *const c_char, n: *const lapack_int, a: *const c_double,
                   lda: *const lapack_int, ap: *mut c_double, info: *mut lapack_int);
    pub fn strttp_(uplo: *const c_char, n: *const lapack_int, a: *const c_float,
                   lda: *const lapack_int, ap: *mut c_float, info: *mut lapack_int);
    pub fn ztrttp_(uplo: *const c_char, n: *const lapack_int, a: *const lapack_complex_double,
                   lda: *const lapack_int, ap: *mut lapack_complex_double, info: *mut lapack_int);
    pub fn ctrttp_(uplo: *const c_char, n: *const lapack_int, a: *const lapack_complex_float,
                   lda: *const lapack_int, ap: *mut lapack_complex_float, info: *mut lapack_int);

    pub fn sgeqrfp_(m: *const lapack_int, n: *const lapack_int, a: *mut c_float,
                    lda: *const lapack_int, tau: *mut c_float, work: *mut c_float,
                    lwork: *const lapack_int, info: *mut lapack_int);
    pub fn dgeqrfp_(m: *const lapack_int, n: *const lapack_int, a: *mut c_double,
                    lda: *const lapack_int, tau: *mut c_double, work: *mut c_double,
                    lwork: *const lapack_int, info: *mut lapack_int);
    pub fn cgeqrfp_(m: *const lapack_int, n: *const lapack_int, a: *mut lapack_complex_float,
                    lda: *const lapack_int, tau: *mut lapack_complex_float,
                    work: *mut lapack_complex_float, lwork: *const lapack_int,
                    info: *mut lapack_int);
    pub fn zgeqrfp_(m: *const lapack_int, n: *const lapack_int, a: *mut lapack_complex_double,
                    lda: *const lapack_int, tau: *mut lapack_complex_double,
                    work: *mut lapack_complex_double, lwork: *const lapack_int,
                    info: *mut lapack_int);

    pub fn clacgv_(n: *const lapack_int, x: *mut lapack_complex_float, incx: *const lapack_int);
    pub fn zlacgv_(n: *const lapack_int, x: *mut lapack_complex_double, incx: *const lapack_int);

    pub fn slarnv_(idist: *const lapack_int, iseed: *mut lapack_int, n: *const lapack_int,
                   x: *mut c_float);
    pub fn dlarnv_(idist: *const lapack_int, iseed: *mut lapack_int, n: *const lapack_int,
                   x: *mut c_double);
    pub fn clarnv_(idist: *const lapack_int, iseed: *mut lapack_int, n: *const lapack_int,
                   x: *mut lapack_complex_float);
    pub fn zlarnv_(idist: *const lapack_int, iseed: *mut lapack_int, n: *const lapack_int,
                   x: *mut lapack_complex_double);

    pub fn sgeqr2_(m: *const lapack_int, n: *const lapack_int, a: *mut c_float,
                   lda: *const lapack_int, tau: *mut c_float, work: *mut c_float,
                   info: *mut lapack_int);
    pub fn dgeqr2_(m: *const lapack_int, n: *const lapack_int, a: *mut c_double,
                   lda: *const lapack_int, tau: *mut c_double, work: *mut c_double,
                   info: *mut lapack_int);
    pub fn cgeqr2_(m: *const lapack_int, n: *const lapack_int, a: *mut lapack_complex_float,
                   lda: *const lapack_int, tau: *mut lapack_complex_float,
                   work: *mut lapack_complex_float, info: *mut lapack_int);
    pub fn zgeqr2_(m: *const lapack_int, n: *const lapack_int, a: *mut lapack_complex_double,
                   lda: *const lapack_int, tau: *mut lapack_complex_double,
                   work: *mut lapack_complex_double, info: *mut lapack_int);

    pub fn slacn2_(n: *const lapack_int, v: *mut c_float, x: *mut c_float,
                   isgn: *mut lapack_int, est: *mut c_float, kase: *mut lapack_int,
                   isave: *mut lapack_int);
    pub fn dlacn2_(n: *const lapack_int, v: *mut c_double, x: *mut c_double,
                   isgn: *mut lapack_int, est: *mut c_double, kase: *mut lapack_int,
                   isave: *mut lapack_int);
    pub fn clacn2_(n: *const lapack_int, v: *mut lapack_complex_float,
                   x: *mut lapack_complex_float, est: *mut c_float, kase: *mut lapack_int,
                   isave: *mut lapack_int);
    pub fn zlacn2_(n: *const lapack_int, v: *mut lapack_complex_double,
                   x: *mut lapack_complex_double, est: *mut c_double, kase: *mut lapack_int,
                   isave: *mut lapack_int);

    pub fn slacpy_(uplo: *const c_char, m: *const lapack_int, n: *const lapack_int,
                   a: *const c_float, lda: *const lapack_int, b: *mut c_float,
                   ldb: *const lapack_int);
    pub fn dlacpy_(uplo: *const c_char, m: *const lapack_int, n: *const lapack_int,
                   a: *const c_double, lda: *const lapack_int, b: *mut c_double,
                   ldb: *const lapack_int);
    pub fn clacpy_(uplo: *const c_char, m: *const lapack_int, n: *const lapack_int,
                   a: *const lapack_complex_float, lda: *const lapack_int,
                   b: *mut lapack_complex_float, ldb: *const lapack_int);
    pub fn zlacpy_(uplo: *const c_char, m: *const lapack_int, n: *const lapack_int,
                   a: *const lapack_complex_double, lda: *const lapack_int,
                   b: *mut lapack_complex_double, ldb: *const lapack_int);

    pub fn clacp2_(uplo: *const c_char, m: *const lapack_int, n: *const lapack_int,
                   a: *const c_float, lda: *const lapack_int, b: *mut lapack_complex_float,
                   ldb: *const lapack_int);
    pub fn zlacp2_(uplo: *const c_char, m: *const lapack_int, n: *const lapack_int,
                   a: *const c_double, lda: *const lapack_int, b: *mut lapack_complex_double,
                   ldb: *const lapack_int);

    pub fn sgetf2_(m: *const lapack_int, n: *const lapack_int, a: *mut c_float,
                   lda: *const lapack_int, ipiv: *mut lapack_int, info: *mut lapack_int);
    pub fn dgetf2_(m: *const lapack_int, n: *const lapack_int, a: *mut c_double,
                   lda: *const lapack_int, ipiv: *mut lapack_int, info: *mut lapack_int);
    pub fn cgetf2_(m: *const lapack_int, n: *const lapack_int, a: *mut lapack_complex_float,
                   lda: *const lapack_int, ipiv: *mut lapack_int, info: *mut lapack_int);
    pub fn zgetf2_(m: *const lapack_int, n: *const lapack_int, a: *mut lapack_complex_double,
                   lda: *const lapack_int, ipiv: *mut lapack_int, info: *mut lapack_int);

    pub fn slaswp_(n: *const lapack_int, a: *mut c_float, lda: *const lapack_int,
                   k1: *const lapack_int, k2: *const lapack_int, ipiv: *const lapack_int,
                   incx: *const lapack_int);
    pub fn dlaswp_(n: *const lapack_int, a: *mut c_double, lda: *const lapack_int,
                   k1: *const lapack_int, k2: *const lapack_int, ipiv: *const lapack_int,
                   incx: *const lapack_int);
    pub fn claswp_(n: *const lapack_int, a: *mut lapack_complex_float, lda: *const lapack_int,
                   k1: *const lapack_int, k2: *const lapack_int, ipiv: *const lapack_int,
                   incx: *const lapack_int);
    pub fn zlaswp_(n: *const lapack_int, a: *mut lapack_complex_double, lda: *const lapack_int,
                   k1: *const lapack_int, k2: *const lapack_int, ipiv: *const lapack_int,
                   incx: *const lapack_int);

    pub fn slange_(norm: *const c_char, m: *const lapack_int, n: *const lapack_int,
                   a: *const c_float, lda: *const lapack_int, work: *mut c_float)
                   -> c_float;
    pub fn dlange_(norm: *const c_char, m: *const lapack_int, n: *const lapack_int,
                   a: *const c_double, lda: *const lapack_int, work: *mut c_double)
                   -> c_double;
    pub fn clange_(norm: *const c_char, m: *const lapack_int, n: *const lapack_int,
                   a: *const lapack_complex_float, lda: *const lapack_int, work: *mut c_float)
                   -> c_float;
    pub fn zlange_(norm: *const c_char, m: *const lapack_int, n: *const lapack_int,
                   a: *const lapack_complex_double, lda: *const lapack_int, work: *mut c_double)
                   -> c_double;

    pub fn clanhe_(norm: *const c_char, uplo: *const c_char, n: *const lapack_int,
                   a: *const lapack_complex_float, lda: *const lapack_int, work: *mut c_float)
                   -> c_float;
    pub fn zlanhe_(norm: *const c_char, uplo: *const c_char, n: *const lapack_int,
                   a: *const lapack_complex_double, lda: *const lapack_int, work: *mut c_double)
                   -> c_double;

    pub fn slansy_(norm: *const c_char, uplo: *const c_char, n: *const lapack_int,
                   a: *const c_float, lda: *const lapack_int, work: *mut c_float)
                   -> c_float;
    pub fn dlansy_(norm: *const c_char, uplo: *const c_char, n: *const lapack_int,
                   a: *const c_double, lda: *const lapack_int, work: *mut c_double)
                   -> c_double;
    pub fn clansy_(norm: *const c_char, uplo: *const c_char, n: *const lapack_int,
                   a: *const lapack_complex_float, lda: *const lapack_int, work: *mut c_float)
                   -> c_float;
    pub fn zlansy_(norm: *const c_char, uplo: *const c_char, n: *const lapack_int,
                   a: *const lapack_complex_double, lda: *const lapack_int, work: *mut c_double)
                   -> c_double;

    pub fn slantr_(norm: *const c_char, uplo: *const c_char, diag: *const c_char,
                   m: *const lapack_int, n: *const lapack_int, a: *const c_float,
                   lda: *const lapack_int, work: *mut c_float)
                   -> c_float;
    pub fn dlantr_(norm: *const c_char, uplo: *const c_char, diag: *const c_char,
                   m: *const lapack_int, n: *const lapack_int, a: *const c_double,
                   lda: *const lapack_int, work: *mut c_double)
                   -> c_double;
    pub fn clantr_(norm: *const c_char, uplo: *const c_char, diag: *const c_char,
                   m: *const lapack_int, n: *const lapack_int, a: *const lapack_complex_float,
                   lda: *const lapack_int, work: *mut c_float)
                   -> c_float;
    pub fn zlantr_(norm: *const c_char, uplo: *const c_char, diag: *const c_char,
                   m: *const lapack_int, n: *const lapack_int, a: *const lapack_complex_double,
                   lda: *const lapack_int, work: *mut c_double)
                   -> c_double;

    pub fn slamch_(cmach: *const c_char) -> c_float;
    pub fn dlamch_(cmach: *const c_char) -> c_double;

    pub fn sgelq2_(m: *const lapack_int, n: *const lapack_int, a: *mut c_float,
                   lda: *const lapack_int, tau: *mut c_float, work: *mut c_float,
                   info: *mut lapack_int);
    pub fn dgelq2_(m: *const lapack_int, n: *const lapack_int, a: *mut c_double,
                   lda: *const lapack_int, tau: *mut c_double, work: *mut c_double,
                   info: *mut lapack_int);
    pub fn cgelq2_(m: *const lapack_int, n: *const lapack_int, a: *mut lapack_complex_float,
                   lda: *const lapack_int, tau: *mut lapack_complex_float,
                   work: *mut lapack_complex_float, info: *mut lapack_int);
    pub fn zgelq2_(m: *const lapack_int, n: *const lapack_int, a: *mut lapack_complex_double,
                   lda: *const lapack_int, tau: *mut lapack_complex_double,
                   work: *mut lapack_complex_double, info: *mut lapack_int);

    pub fn slarfb_(side: *const c_char, trans: *const c_char, direct: *const c_char,
                   storev: *const c_char, m: *const lapack_int, n: *const lapack_int,
                   k: *const lapack_int, v: *const c_float, ldv: *const lapack_int,
                   t: *const c_float, ldt: *const lapack_int, c: *mut c_float,
                   ldc: *const lapack_int, work: *mut c_float, ldwork: *const lapack_int);
    pub fn dlarfb_(side: *const c_char, trans: *const c_char, direct: *const c_char,
                   storev: *const c_char, m: *const lapack_int, n: *const lapack_int,
                   k: *const lapack_int, v: *const c_double, ldv: *const lapack_int,
                   t: *const c_double, ldt: *const lapack_int, c: *mut c_double,
                   ldc: *const lapack_int, work: *mut c_double, ldwork: *const lapack_int);
    pub fn clarfb_(side: *const c_char, trans: *const c_char, direct: *const c_char,
                   storev: *const c_char, m: *const lapack_int, n: *const lapack_int,
                   k: *const lapack_int, v: *const lapack_complex_float, ldv: *const lapack_int,
                   t: *const lapack_complex_float, ldt: *const lapack_int,
                   c: *mut lapack_complex_float, ldc: *const lapack_int,
                   work: *mut lapack_complex_float, ldwork: *const lapack_int);
    pub fn zlarfb_(side: *const c_char, trans: *const c_char, direct: *const c_char,
                   storev: *const c_char, m: *const lapack_int, n: *const lapack_int,
                   k: *const lapack_int, v: *const lapack_complex_double,
                   ldv: *const lapack_int, t: *const lapack_complex_double,
                   ldt: *const lapack_int, c: *mut lapack_complex_double,
                   ldc: *const lapack_int, work: *mut lapack_complex_double,
                   ldwork: *const lapack_int);

    pub fn slarfg_(n: *const lapack_int, alpha: *mut c_float, x: *mut c_float,
                   incx: *const lapack_int, tau: *mut c_float);
    pub fn dlarfg_(n: *const lapack_int, alpha: *mut c_double, x: *mut c_double,
                   incx: *const lapack_int, tau: *mut c_double);
    pub fn clarfg_(n: *const lapack_int, alpha: *mut lapack_complex_float,
                   x: *mut lapack_complex_float, incx: *const lapack_int,
                   tau: *mut lapack_complex_float);
    pub fn zlarfg_(n: *const lapack_int, alpha: *mut lapack_complex_double,
                   x: *mut lapack_complex_double, incx: *const lapack_int,
                   tau: *mut lapack_complex_double);

    pub fn slarft_(direct: *const c_char, storev: *const c_char, n: *const lapack_int,
                   k: *const lapack_int, v: *const c_float, ldv: *const lapack_int,
                   tau: *const c_float, t: *mut c_float, ldt: *const lapack_int);
    pub fn dlarft_(direct: *const c_char, storev: *const c_char, n: *const lapack_int,
                   k: *const lapack_int, v: *const c_double, ldv: *const lapack_int,
                   tau: *const c_double, t: *mut c_double, ldt: *const lapack_int);
    pub fn clarft_(direct: *const c_char, storev: *const c_char, n: *const lapack_int,
                   k: *const lapack_int, v: *const lapack_complex_float, ldv: *const lapack_int,
                   tau: *const lapack_complex_float, t: *mut lapack_complex_float,
                   ldt: *const lapack_int);
    pub fn zlarft_(direct: *const c_char, storev: *const c_char, n: *const lapack_int,
                   k: *const lapack_int, v: *const lapack_complex_double,
                   ldv: *const lapack_int, tau: *const lapack_complex_double,
                   t: *mut lapack_complex_double, ldt: *const lapack_int);

    pub fn slarfx_(side: *const c_char, m: *const lapack_int, n: *const lapack_int,
                   v: *const c_float, tau: *const c_float, c: *mut c_float,
                   ldc: *const lapack_int, work: *mut c_float);
    pub fn dlarfx_(side: *const c_char, m: *const lapack_int, n: *const lapack_int,
                   v: *const c_double, tau: *const c_double, c: *mut c_double,
                   ldc: *const lapack_int, work: *mut c_double);
    pub fn clarfx_(side: *const c_char, m: *const lapack_int, n: *const lapack_int,
                   v: *const lapack_complex_float, tau: *const lapack_complex_float,
                   c: *mut lapack_complex_float, ldc: *const lapack_int,
                   work: *mut lapack_complex_float);
    pub fn zlarfx_(side: *const c_char, m: *const lapack_int, n: *const lapack_int,
                   v: *const lapack_complex_double, tau: *const lapack_complex_double,
                   c: *mut lapack_complex_double, ldc: *const lapack_int,
                   work: *mut lapack_complex_double);

    pub fn slatms_(m: *const lapack_int, n: *const lapack_int, dist: *const c_char,
                   iseed: *mut lapack_int, sym: *const c_char, d: *mut c_float,
                   mode: *const lapack_int, cond: *const c_float, dmax: *const c_float,
                   kl: *const lapack_int, ku: *const lapack_int, pack: *const c_char,
                   a: *mut c_float, lda: *const lapack_int, work: *mut c_float,
                   info: *mut lapack_int);
    pub fn dlatms_(m: *const lapack_int, n: *const lapack_int, dist: *const c_char,
                   iseed: *mut lapack_int, sym: *const c_char, d: *mut c_double,
                   mode: *const lapack_int, cond: *const c_double, dmax: *const c_double,
                   kl: *const lapack_int, ku: *const lapack_int, pack: *const c_char,
                   a: *mut c_double, lda: *const lapack_int, work: *mut c_double,
                   info: *mut lapack_int);
    pub fn clatms_(m: *const lapack_int, n: *const lapack_int, dist: *const c_char,
                   iseed: *mut lapack_int, sym: *const c_char, d: *mut c_float,
                   mode: *const lapack_int, cond: *const c_float, dmax: *const c_float,
                   kl: *const lapack_int, ku: *const lapack_int, pack: *const c_char,
                   a: *mut lapack_complex_float, lda: *const lapack_int,
                   work: *mut lapack_complex_float, info: *mut lapack_int);
    pub fn zlatms_(m: *const lapack_int, n: *const lapack_int, dist: *const c_char,
                   iseed: *mut lapack_int, sym: *const c_char, d: *mut c_double,
                   mode: *const lapack_int, cond: *const c_double, dmax: *const c_double,
                   kl: *const lapack_int, ku: *const lapack_int, pack: *const c_char,
                   a: *mut lapack_complex_double, lda: *const lapack_int,
                   work: *mut lapack_complex_double, info: *mut lapack_int);

    pub fn slag2d_(m: *const lapack_int, n: *const lapack_int, sa: *const c_float,
                   ldsa: *const lapack_int, a: *mut c_double, lda: *const lapack_int,
                   info: *mut lapack_int);
    pub fn dlag2s_(m: *const lapack_int, n: *const lapack_int, a: *const c_double,
                   lda: *const lapack_int, sa: *mut c_float, ldsa: *const lapack_int,
                   info: *mut lapack_int);
    pub fn clag2z_(m: *const lapack_int, n: *const lapack_int, sa: *const lapack_complex_float,
                   ldsa: *const lapack_int, a: *mut lapack_complex_double,
                   lda: *const lapack_int, info: *mut lapack_int);
    pub fn zlag2c_(m: *const lapack_int, n: *const lapack_int, a: *const lapack_complex_double,
                   lda: *const lapack_int, sa: *mut lapack_complex_float,
                   ldsa: *const lapack_int, info: *mut lapack_int);

    pub fn slauum_(uplo: *const c_char, n: *const lapack_int, a: *mut c_float,
                   lda: *const lapack_int, info: *mut lapack_int);
    pub fn dlauum_(uplo: *const c_char, n: *const lapack_int, a: *mut c_double,
                   lda: *const lapack_int, info: *mut lapack_int);
    pub fn clauum_(uplo: *const c_char, n: *const lapack_int, a: *mut lapack_complex_float,
                   lda: *const lapack_int, info: *mut lapack_int);
    pub fn zlauum_(uplo: *const c_char, n: *const lapack_int, a: *mut lapack_complex_double,
                   lda: *const lapack_int, info: *mut lapack_int);

    pub fn slagge_(m: *const lapack_int, n: *const lapack_int, kl: *const lapack_int,
                   ku: *const lapack_int, d: *const c_float, a: *mut c_float,
                   lda: *const lapack_int, iseed: *mut lapack_int, work: *mut c_float,
                   info: *mut lapack_int);
    pub fn dlagge_(m: *const lapack_int, n: *const lapack_int, kl: *const lapack_int,
                   ku: *const lapack_int, d: *const c_double, a: *mut c_double,
                   lda: *const lapack_int, iseed: *mut lapack_int, work: *mut c_double,
                   info: *mut lapack_int);
    pub fn clagge_(m: *const lapack_int, n: *const lapack_int, kl: *const lapack_int,
                   ku: *const lapack_int, d: *const c_float, a: *mut lapack_complex_float,
                   lda: *const lapack_int, iseed: *mut lapack_int,
                   work: *mut lapack_complex_float, info: *mut lapack_int);
    pub fn zlagge_(m: *const lapack_int, n: *const lapack_int, kl: *const lapack_int,
                   ku: *const lapack_int, d: *const c_double, a: *mut lapack_complex_double,
                   lda: *const lapack_int, iseed: *mut lapack_int,
                   work: *mut lapack_complex_double, info: *mut lapack_int);

    pub fn slascl_(_type: *const c_char, kl: *const lapack_int, ku: *const lapack_int,
                   cfrom: *const c_float, cto: *const c_float, m: *const lapack_int,
                   n: *const lapack_int, a: *mut c_float, lda: *const lapack_int,
                   info: *mut lapack_int);
    pub fn dlascl_(_type: *const c_char, kl: *const lapack_int, ku: *const lapack_int,
                   cfrom: *const c_double, cto: *const c_double, m: *const lapack_int,
                   n: *const lapack_int, a: *mut c_double, lda: *const lapack_int,
                   info: *mut lapack_int);
    pub fn clascl_(_type: *const c_char, kl: *const lapack_int, ku: *const lapack_int,
                   cfrom: *const c_float, cto: *const c_float, m: *const lapack_int,
                   n: *const lapack_int, a: *mut lapack_complex_float, lda: *const lapack_int,
                   info: *mut lapack_int);
    pub fn zlascl_(_type: *const c_char, kl: *const lapack_int, ku: *const lapack_int,
                   cfrom: *const c_double, cto: *const c_double, m: *const lapack_int,
                   n: *const lapack_int, a: *mut lapack_complex_double, lda: *const lapack_int,
                   info: *mut lapack_int);

    pub fn slaset_(uplo: *const c_char, m: *const lapack_int, n: *const lapack_int,
                   alpha: *const c_float, beta: *const c_float, a: *mut c_float,
                   lda: *const lapack_int);
    pub fn dlaset_(uplo: *const c_char, m: *const lapack_int, n: *const lapack_int,
                   alpha: *const c_double, beta: *const c_double, a: *mut c_double,
                   lda: *const lapack_int);
    pub fn claset_(uplo: *const c_char, m: *const lapack_int, n: *const lapack_int,
                   alpha: *const lapack_complex_float, beta: *const lapack_complex_float,
                   a: *mut lapack_complex_float, lda: *const lapack_int);
    pub fn zlaset_(uplo: *const c_char, m: *const lapack_int, n: *const lapack_int,
                   alpha: *const lapack_complex_double, beta: *const lapack_complex_double,
                   a: *mut lapack_complex_double, lda: *const lapack_int);

    pub fn slasrt_(id: *const c_char, n: *const lapack_int, d: *mut c_float,
                   info: *mut lapack_int);
    pub fn dlasrt_(id: *const c_char, n: *const lapack_int, d: *mut c_double,
                   info: *mut lapack_int);

    pub fn claghe_(n: *const lapack_int, k: *const lapack_int, d: *const c_float,
                   a: *mut lapack_complex_float, lda: *const lapack_int, iseed: *mut lapack_int,
                   work: *mut lapack_complex_float, info: *mut lapack_int);
    pub fn zlaghe_(n: *const lapack_int, k: *const lapack_int, d: *const c_double,
                   a: *mut lapack_complex_double, lda: *const lapack_int,
                   iseed: *mut lapack_int, work: *mut lapack_complex_double,
                   info: *mut lapack_int);

    pub fn slagsy_(n: *const lapack_int, k: *const lapack_int, d: *const c_float,
                   a: *mut c_float, lda: *const lapack_int, iseed: *mut lapack_int,
                   work: *mut c_float, info: *mut lapack_int);
    pub fn dlagsy_(n: *const lapack_int, k: *const lapack_int, d: *const c_double,
                   a: *mut c_double, lda: *const lapack_int, iseed: *mut lapack_int,
                   work: *mut c_double, info: *mut lapack_int);
    pub fn clagsy_(n: *const lapack_int, k: *const lapack_int, d: *const c_float,
                   a: *mut lapack_complex_float, lda: *const lapack_int, iseed: *mut lapack_int,
                   work: *mut lapack_complex_float, info: *mut lapack_int);
    pub fn zlagsy_(n: *const lapack_int, k: *const lapack_int, d: *const c_double,
                   a: *mut lapack_complex_double, lda: *const lapack_int,
                   iseed: *mut lapack_int, work: *mut lapack_complex_double,
                   info: *mut lapack_int);

    pub fn slapmr_(forwrd: *const lapack_logical, m: *const lapack_int, n: *const lapack_int,
                   x: *mut c_float, ldx: *const lapack_int, k: *mut lapack_int);
    pub fn dlapmr_(forwrd: *const lapack_logical, m: *const lapack_int, n: *const lapack_int,
                   x: *mut c_double, ldx: *const lapack_int, k: *mut lapack_int);
    pub fn clapmr_(forwrd: *const lapack_logical, m: *const lapack_int, n: *const lapack_int,
                   x: *mut lapack_complex_float, ldx: *const lapack_int, k: *mut lapack_int);
    pub fn zlapmr_(forwrd: *const lapack_logical, m: *const lapack_int, n: *const lapack_int,
                   x: *mut lapack_complex_double, ldx: *const lapack_int, k: *mut lapack_int);

    pub fn slapmt_(forwrd: *const lapack_logical, m: *const lapack_int, n: *const lapack_int,
                   x: *mut c_float, ldx: *const lapack_int, k: *mut lapack_int);
    pub fn dlapmt_(forwrd: *const lapack_logical, m: *const lapack_int, n: *const lapack_int,
                   x: *mut c_double, ldx: *const lapack_int, k: *mut lapack_int);
    pub fn clapmt_(forwrd: *const lapack_logical, m: *const lapack_int, n: *const lapack_int,
                   x: *mut lapack_complex_float, ldx: *const lapack_int, k: *mut lapack_int);
    pub fn zlapmt_(forwrd: *const lapack_logical, m: *const lapack_int, n: *const lapack_int,
                   x: *mut lapack_complex_double, ldx: *const lapack_int, k: *mut lapack_int);

    pub fn slapy2_(x: *const c_float, y: *const c_float) -> c_float;
    pub fn dlapy2_(x: *const c_double, y: *const c_double) -> c_double;

    pub fn slapy3_(x: *const c_float, y: *const c_float, z: *const c_float) -> c_float;
    pub fn dlapy3_(x: *const c_double, y: *const c_double, z: *const c_double) -> c_double;

    pub fn slartgp_(f: *const c_float, g: *const c_float, cs: *mut c_float, sn: *mut c_float,
                    r: *mut c_float);
    pub fn dlartgp_(f: *const c_double, g: *const c_double, cs: *mut c_double,
                    sn: *mut c_double, r: *mut c_double);

    pub fn slartgs_(x: *const c_float, y: *const c_float, sigma: *const c_float,
                    cs: *mut c_float, sn: *mut c_float);
    pub fn dlartgs_(x: *const c_double, y: *const c_double, sigma: *const c_double,
                    cs: *mut c_double, sn: *mut c_double);

    // Version 3.3.0
    pub fn cbbcsd_(jobu1: *const c_char, jobu2: *const c_char, jobv1t: *const c_char,
                   jobv2t: *const c_char, trans: *const c_char, m: *const lapack_int,
                   p: *const lapack_int, q: *const lapack_int, theta: *mut c_float,
                   phi: *mut c_float, u1: *mut lapack_complex_float, ldu1: *const lapack_int,
                   u2: *mut lapack_complex_float, ldu2: *const lapack_int,
                   v1t: *mut lapack_complex_float, ldv1t: *const lapack_int,
                   v2t: *mut lapack_complex_float, ldv2t: *const lapack_int, b11d: *mut c_float,
                   b11e: *mut c_float, b12d: *mut c_float, b12e: *mut c_float,
                   b21d: *mut c_float, b21e: *mut c_float, b22d: *mut c_float,
                   b22e: *mut c_float, rwork: *mut c_float, lrwork: *const lapack_int,
                   info: *mut lapack_int);
    pub fn cheswapr_(uplo: *const c_char, n: *const lapack_int, a: *mut lapack_complex_float,
                     i1: *const lapack_int, i2: *const lapack_int);
    pub fn chetri2_(uplo: *const c_char, n: *const lapack_int, a: *mut lapack_complex_float,
                    lda: *const lapack_int, ipiv: *const lapack_int,
                    work: *mut lapack_complex_float, lwork: *const lapack_int,
                    info: *mut lapack_int);
    pub fn chetri2x_(uplo: *const c_char, n: *const lapack_int, a: *mut lapack_complex_float,
                     lda: *const lapack_int, ipiv: *const lapack_int,
                     work: *mut lapack_complex_float, nb: *const lapack_int,
                     info: *mut lapack_int);
    pub fn chetrs2_(uplo: *const c_char, n: *const lapack_int, nrhs: *const lapack_int,
                    a: *const lapack_complex_float, lda: *const lapack_int,
                    ipiv: *const lapack_int, b: *mut lapack_complex_float,
                    ldb: *const lapack_int, work: *mut lapack_complex_float,
                    info: *mut lapack_int);
    pub fn csyconv_(uplo: *const c_char, way: *const c_char, n: *const lapack_int,
                    a: *mut lapack_complex_float, lda: *const lapack_int,
                    ipiv: *const lapack_int, work: *mut lapack_complex_float,
                    info: *mut lapack_int);
    pub fn csyswapr_(uplo: *const c_char, n: *const lapack_int, a: *mut lapack_complex_float,
                     i1: *const lapack_int, i2: *const lapack_int);
    pub fn csytri2_(uplo: *const c_char, n: *const lapack_int, a: *mut lapack_complex_float,
                    lda: *const lapack_int, ipiv: *const lapack_int,
                    work: *mut lapack_complex_float, lwork: *const lapack_int,
                    info: *mut lapack_int);
    pub fn csytri2x_(uplo: *const c_char, n: *const lapack_int, a: *mut lapack_complex_float,
                     lda: *const lapack_int, ipiv: *const lapack_int,
                     work: *mut lapack_complex_float, nb: *const lapack_int,
                     info: *mut lapack_int);
    pub fn csytrs2_(uplo: *const c_char, n: *const lapack_int, nrhs: *const lapack_int,
                    a: *const lapack_complex_float, lda: *const lapack_int,
                    ipiv: *const lapack_int, b: *mut lapack_complex_float,
                    ldb: *const lapack_int, work: *mut lapack_complex_float,
                    info: *mut lapack_int);
    pub fn cunbdb_(trans: *const c_char, signs: *const c_char, m: *const lapack_int,
                   p: *const lapack_int, q: *const lapack_int, x11: *mut lapack_complex_float,
                   ldx11: *const lapack_int, x12: *mut lapack_complex_float,
                   ldx12: *const lapack_int, x21: *mut lapack_complex_float,
                   ldx21: *const lapack_int, x22: *mut lapack_complex_float,
                   ldx22: *const lapack_int, theta: *mut c_float, phi: *mut c_float,
                   taup1: *mut lapack_complex_float, taup2: *mut lapack_complex_float,
                   tauq1: *mut lapack_complex_float, tauq2: *mut lapack_complex_float,
                   work: *mut lapack_complex_float, lwork: *const lapack_int,
                   info: *mut lapack_int);
    pub fn cuncsd_(jobu1: *const c_char, jobu2: *const c_char, jobv1t: *const c_char,
                   jobv2t: *const c_char, trans: *const c_char, signs: *const c_char,
                   m: *const lapack_int, p: *const lapack_int, q: *const lapack_int,
                   x11: *mut lapack_complex_float, ldx11: *const lapack_int,
                   x12: *mut lapack_complex_float, ldx12: *const lapack_int,
                   x21: *mut lapack_complex_float, ldx21: *const lapack_int,
                   x22: *mut lapack_complex_float, ldx22: *const lapack_int,
                   theta: *mut c_float, u1: *mut lapack_complex_float, ldu1: *const lapack_int,
                   u2: *mut lapack_complex_float, ldu2: *const lapack_int,
                   v1t: *mut lapack_complex_float, ldv1t: *const lapack_int,
                   v2t: *mut lapack_complex_float, ldv2t: *const lapack_int,
                   work: *mut lapack_complex_float, lwork: *const lapack_int,
                   rwork: *mut c_float, lrwork: *const lapack_int, iwork: *mut lapack_int,
                   info: *mut lapack_int);
    pub fn cuncsd2by1_(jobu1: *const c_char, jobu2: *const c_char, jobv1t: *const c_char,
                       m: *const lapack_int, p: *const lapack_int, q: *const lapack_int,
                       x11: *mut lapack_complex_float, ldx11: *const lapack_int,
                       x21: *mut lapack_complex_float, ldx21: *const lapack_int,
                       theta: *mut lapack_complex_float, u1: *mut lapack_complex_float,
                       ldu1: *const lapack_int, u2: *mut lapack_complex_float,
                       ldu2: *const lapack_int, v1t: *mut lapack_complex_float,
                       ldv1t: *const lapack_int, work: *mut lapack_complex_float,
                       lwork: *const lapack_int, rwork: *mut c_float, lrwork: *const lapack_int,
                       iwork: *mut lapack_int, info: *mut lapack_int);
    pub fn dbbcsd_(jobu1: *const c_char, jobu2: *const c_char, jobv1t: *const c_char,
                   jobv2t: *const c_char, trans: *const c_char, m: *const lapack_int,
                   p: *const lapack_int, q: *const lapack_int, theta: *mut c_double,
                   phi: *mut c_double, u1: *mut c_double, ldu1: *const lapack_int,
                   u2: *mut c_double, ldu2: *const lapack_int, v1t: *mut c_double,
                   ldv1t: *const lapack_int, v2t: *mut c_double, ldv2t: *const lapack_int,
                   b11d: *mut c_double, b11e: *mut c_double, b12d: *mut c_double,
                   b12e: *mut c_double, b21d: *mut c_double, b21e: *mut c_double,
                   b22d: *mut c_double, b22e: *mut c_double, work: *mut c_double,
                   lwork: *const lapack_int, info: *mut lapack_int);
    pub fn dorbdb_(trans: *const c_char, signs: *const c_char, m: *const lapack_int,
                   p: *const lapack_int, q: *const lapack_int, x11: *mut c_double,
                   ldx11: *const lapack_int, x12: *mut c_double, ldx12: *const lapack_int,
                   x21: *mut c_double, ldx21: *const lapack_int, x22: *mut c_double,
                   ldx22: *const lapack_int, theta: *mut c_double, phi: *mut c_double,
                   taup1: *mut c_double, taup2: *mut c_double, tauq1: *mut c_double,
                   tauq2: *mut c_double, work: *mut c_double, lwork: *const lapack_int,
                   info: *mut lapack_int);
    pub fn dorcsd_(jobu1: *const c_char, jobu2: *const c_char, jobv1t: *const c_char,
                   jobv2t: *const c_char, trans: *const c_char, signs: *const c_char,
                   m: *const lapack_int, p: *const lapack_int, q: *const lapack_int,
                   x11: *mut c_double, ldx11: *const lapack_int, x12: *mut c_double,
                   ldx12: *const lapack_int, x21: *mut c_double, ldx21: *const lapack_int,
                   x22: *mut c_double, ldx22: *const lapack_int, theta: *mut c_double,
                   u1: *mut c_double, ldu1: *const lapack_int, u2: *mut c_double,
                   ldu2: *const lapack_int, v1t: *mut c_double, ldv1t: *const lapack_int,
                   v2t: *mut c_double, ldv2t: *const lapack_int, work: *mut c_double,
                   lwork: *const lapack_int, iwork: *mut lapack_int, info: *mut lapack_int);
    pub fn dorcsd2by1_(jobu1: *const c_char, jobu2: *const c_char, jobv1t: *const c_char,
                       m: *const lapack_int, p: *const lapack_int, q: *const lapack_int,
                       x11: *mut c_double, ldx11: *const lapack_int, x21: *mut c_double,
                       ldx21: *const lapack_int, theta: *mut c_double, u1: *mut c_double,
                       ldu1: *const lapack_int, u2: *mut c_double, ldu2: *const lapack_int,
                       v1t: *mut c_double, ldv1t: *const lapack_int, work: *mut c_double,
                       lwork: *const lapack_int, iwork: *mut lapack_int, info: *mut lapack_int);
    pub fn dsyconv_(uplo: *const c_char, way: *const c_char, n: *const lapack_int,
                    a: *mut c_double, lda: *const lapack_int, ipiv: *const lapack_int,
                    work: *mut c_double, info: *mut lapack_int);
    pub fn dsyswapr_(uplo: *const c_char, n: *const lapack_int, a: *mut c_double,
                     i1: *const lapack_int, i2: *const lapack_int);
    pub fn dsytri2_(uplo: *const c_char, n: *const lapack_int, a: *mut c_double,
                    lda: *const lapack_int, ipiv: *const lapack_int,
                    work: *mut lapack_complex_double, lwork: *const lapack_int,
                    info: *mut lapack_int);
    pub fn dsytri2x_(uplo: *const c_char, n: *const lapack_int, a: *mut c_double,
                     lda: *const lapack_int, ipiv: *const lapack_int, work: *mut c_double,
                     nb: *const lapack_int, info: *mut lapack_int);
    pub fn dsytrs2_(uplo: *const c_char, n: *const lapack_int, nrhs: *const lapack_int,
                    a: *const c_double, lda: *const lapack_int, ipiv: *const lapack_int,
                    b: *mut c_double, ldb: *const lapack_int, work: *mut c_double,
                    info: *mut lapack_int);
    pub fn sbbcsd_(jobu1: *const c_char, jobu2: *const c_char, jobv1t: *const c_char,
                   jobv2t: *const c_char, trans: *const c_char, m: *const lapack_int,
                   p: *const lapack_int, q: *const lapack_int, theta: *mut c_float,
                   phi: *mut c_float, u1: *mut c_float, ldu1: *const lapack_int,
                   u2: *mut c_float, ldu2: *const lapack_int, v1t: *mut c_float,
                   ldv1t: *const lapack_int, v2t: *mut c_float, ldv2t: *const lapack_int,
                   b11d: *mut c_float, b11e: *mut c_float, b12d: *mut c_float,
                   b12e: *mut c_float, b21d: *mut c_float, b21e: *mut c_float,
                   b22d: *mut c_float, b22e: *mut c_float, work: *mut c_float,
                   lwork: *const lapack_int, info: *mut lapack_int);
    pub fn sorbdb_(trans: *const c_char, signs: *const c_char, m: *const lapack_int,
                   p: *const lapack_int, q: *const lapack_int, x11: *mut c_float,
                   ldx11: *const lapack_int, x12: *mut c_float, ldx12: *const lapack_int,
                   x21: *mut c_float, ldx21: *const lapack_int, x22: *mut c_float,
                   ldx22: *const lapack_int, theta: *mut c_float, phi: *mut c_float,
                   taup1: *mut c_float, taup2: *mut c_float, tauq1: *mut c_float,
                   tauq2: *mut c_float, work: *mut c_float, lwork: *const lapack_int,
                   info: *mut lapack_int);
    pub fn sorcsd_(jobu1: *const c_char, jobu2: *const c_char, jobv1t: *const c_char,
                   jobv2t: *const c_char, trans: *const c_char, signs: *const c_char,
                   m: *const lapack_int, p: *const lapack_int, q: *const lapack_int,
                   x11: *mut c_float, ldx11: *const lapack_int, x12: *mut c_float,
                   ldx12: *const lapack_int, x21: *mut c_float, ldx21: *const lapack_int,
                   x22: *mut c_float, ldx22: *const lapack_int, theta: *mut c_float,
                   u1: *mut c_float, ldu1: *const lapack_int, u2: *mut c_float,
                   ldu2: *const lapack_int, v1t: *mut c_float, ldv1t: *const lapack_int,
                   v2t: *mut c_float, ldv2t: *const lapack_int, work: *mut c_float,
                   lwork: *const lapack_int, iwork: *mut lapack_int, info: *mut lapack_int);
    pub fn sorcsd2by1_(jobu1: *const c_char, jobu2: *const c_char, jobv1t: *const c_char,
                       m: *const lapack_int, p: *const lapack_int, q: *const lapack_int,
                       x11: *mut c_float, ldx11: *const lapack_int, x21: *mut c_float,
                       ldx21: *const lapack_int, theta: *mut c_float, u1: *mut c_float,
                       ldu1: *const lapack_int, u2: *mut c_float, ldu2: *const lapack_int,
                       v1t: *mut c_float, ldv1t: *const lapack_int, work: *mut c_float,
                       lwork: *const lapack_int, iwork: *mut lapack_int, info: *mut lapack_int);
    pub fn ssyconv_(uplo: *const c_char, way: *const c_char, n: *const lapack_int,
                    a: *mut c_float, lda: *const lapack_int, ipiv: *const lapack_int,
                    work: *mut c_float, info: *mut lapack_int);
    pub fn ssyswapr_(uplo: *const c_char, n: *const lapack_int, a: *mut c_float,
                     i1: *const lapack_int, i2: *const lapack_int);
    pub fn ssytri2_(uplo: *const c_char, n: *const lapack_int, a: *mut c_float,
                    lda: *const lapack_int, ipiv: *const lapack_int,
                    work: *mut lapack_complex_float, lwork: *const lapack_int,
                    info: *mut lapack_int);
    pub fn ssytri2x_(uplo: *const c_char, n: *const lapack_int, a: *mut c_float,
                     lda: *const lapack_int, ipiv: *const lapack_int, work: *mut c_float,
                     nb: *const lapack_int, info: *mut lapack_int);
    pub fn ssytrs2_(uplo: *const c_char, n: *const lapack_int, nrhs: *const lapack_int,
                    a: *const c_float, lda: *const lapack_int, ipiv: *const lapack_int,
                    b: *mut c_float, ldb: *const lapack_int, work: *mut c_float,
                    info: *mut lapack_int);
    pub fn zbbcsd_(jobu1: *const c_char, jobu2: *const c_char, jobv1t: *const c_char,
                   jobv2t: *const c_char, trans: *const c_char, m: *const lapack_int,
                   p: *const lapack_int, q: *const lapack_int, theta: *mut c_double,
                   phi: *mut c_double, u1: *mut lapack_complex_double, ldu1: *const lapack_int,
                   u2: *mut lapack_complex_double, ldu2: *const lapack_int,
                   v1t: *mut lapack_complex_double, ldv1t: *const lapack_int,
                   v2t: *mut lapack_complex_double, ldv2t: *const lapack_int,
                   b11d: *mut c_double, b11e: *mut c_double, b12d: *mut c_double,
                   b12e: *mut c_double, b21d: *mut c_double, b21e: *mut c_double,
                   b22d: *mut c_double, b22e: *mut c_double, rwork: *mut c_double,
                   lrwork: *const lapack_int, info: *mut lapack_int);
    pub fn zheswapr_(uplo: *const c_char, n: *const lapack_int, a: *mut lapack_complex_double,
                     i1: *const lapack_int, i2: *const lapack_int);
    pub fn zhetri2_(uplo: *const c_char, n: *const lapack_int, a: *mut lapack_complex_double,
                    lda: *const lapack_int, ipiv: *const lapack_int,
                    work: *mut lapack_complex_double, lwork: *const lapack_int,
                    info: *mut lapack_int);
    pub fn zhetri2x_(uplo: *const c_char, n: *const lapack_int, a: *mut lapack_complex_double,
                     lda: *const lapack_int, ipiv: *const lapack_int,
                     work: *mut lapack_complex_double, nb: *const lapack_int,
                     info: *mut lapack_int);
    pub fn zhetrs2_(uplo: *const c_char, n: *const lapack_int, nrhs: *const lapack_int,
                    a: *const lapack_complex_double, lda: *const lapack_int,
                    ipiv: *const lapack_int, b: *mut lapack_complex_double,
                    ldb: *const lapack_int, work: *mut lapack_complex_double,
                    info: *mut lapack_int);
    pub fn zsyconv_(uplo: *const c_char, way: *const c_char, n: *const lapack_int,
                    a: *mut lapack_complex_double, lda: *const lapack_int,
                    ipiv: *const lapack_int, work: *mut lapack_complex_double,
                    info: *mut lapack_int);
    pub fn zsyswapr_(uplo: *const c_char, n: *const lapack_int, a: *mut lapack_complex_double,
                     i1: *const lapack_int, i2: *const lapack_int);
    pub fn zsytri2_(uplo: *const c_char, n: *const lapack_int, a: *mut lapack_complex_double,
                    lda: *const lapack_int, ipiv: *const lapack_int,
                    work: *mut lapack_complex_double, lwork: *const lapack_int,
                    info: *mut lapack_int);
    pub fn zsytri2x_(uplo: *const c_char, n: *const lapack_int, a: *mut lapack_complex_double,
                     lda: *const lapack_int, ipiv: *const lapack_int,
                     work: *mut lapack_complex_double, nb: *const lapack_int,
                     info: *mut lapack_int);
    pub fn zsytrs2_(uplo: *const c_char, n: *const lapack_int, nrhs: *const lapack_int,
                    a: *const lapack_complex_double, lda: *const lapack_int,
                    ipiv: *const lapack_int, b: *mut lapack_complex_double,
                    ldb: *const lapack_int, work: *mut lapack_complex_double,
                    info: *mut lapack_int);
    pub fn zunbdb_(trans: *const c_char, signs: *const c_char, m: *const lapack_int,
                   p: *const lapack_int, q: *const lapack_int, x11: *mut lapack_complex_double,
                   ldx11: *const lapack_int, x12: *mut lapack_complex_double,
                   ldx12: *const lapack_int, x21: *mut lapack_complex_double,
                   ldx21: *const lapack_int, x22: *mut lapack_complex_double,
                   ldx22: *const lapack_int, theta: *mut c_double, phi: *mut c_double,
                   taup1: *mut lapack_complex_double, taup2: *mut lapack_complex_double,
                   tauq1: *mut lapack_complex_double, tauq2: *mut lapack_complex_double,
                   work: *mut lapack_complex_double, lwork: *const lapack_int,
                   info: *mut lapack_int);
    pub fn zuncsd_(jobu1: *const c_char, jobu2: *const c_char, jobv1t: *const c_char,
                   jobv2t: *const c_char, trans: *const c_char, signs: *const c_char,
                   m: *const lapack_int, p: *const lapack_int, q: *const lapack_int,
                   x11: *mut lapack_complex_double, ldx11: *const lapack_int,
                   x12: *mut lapack_complex_double, ldx12: *const lapack_int,
                   x21: *mut lapack_complex_double, ldx21: *const lapack_int,
                   x22: *mut lapack_complex_double, ldx22: *const lapack_int,
                   theta: *mut c_double, u1: *mut lapack_complex_double,
                   ldu1: *const lapack_int, u2: *mut lapack_complex_double,
                   ldu2: *const lapack_int, v1t: *mut lapack_complex_double,
                   ldv1t: *const lapack_int, v2t: *mut lapack_complex_double,
                   ldv2t: *const lapack_int, work: *mut lapack_complex_double,
                   lwork: *const lapack_int, rwork: *mut c_double, lrwork: *const lapack_int,
                   iwork: *mut lapack_int, info: *mut lapack_int);
    pub fn zuncsd2by1_(jobu1: *const c_char, jobu2: *const c_char, jobv1t: *const c_char,
                       m: *const lapack_int, p: *const lapack_int, q: *const lapack_int,
                       x11: *mut lapack_complex_double, ldx11: *const lapack_int,
                       x21: *mut lapack_complex_double, ldx21: *const lapack_int,
                       theta: *mut lapack_complex_double, u1: *mut lapack_complex_double,
                       ldu1: *const lapack_int, u2: *mut lapack_complex_double,
                       ldu2: *const lapack_int, v1t: *mut lapack_complex_double,
                       ldv1t: *const lapack_int, work: *mut lapack_complex_double,
                       lwork: *const lapack_int, rwork: *mut c_double,
                       lrwork: *const lapack_int, iwork: *mut lapack_int, info: *mut lapack_int);

    // Version 3.4.0
    pub fn sgemqrt_(side: *const c_char, trans: *const c_char, m: *const lapack_int,
                    n: *const lapack_int, k: *const lapack_int, nb: *const lapack_int,
                    v: *const c_float, ldv: *const lapack_int, t: *const c_float,
                    ldt: *const lapack_int, c: *mut c_float, ldc: *const lapack_int,
                    work: *mut c_float, info: *mut lapack_int);
    pub fn dgemqrt_(side: *const c_char, trans: *const c_char, m: *const lapack_int,
                    n: *const lapack_int, k: *const lapack_int, nb: *const lapack_int,
                    v: *const c_double, ldv: *const lapack_int, t: *const c_double,
                    ldt: *const lapack_int, c: *mut c_double, ldc: *const lapack_int,
                    work: *mut c_double, info: *mut lapack_int);
    pub fn cgemqrt_(side: *const c_char, trans: *const c_char, m: *const lapack_int,
                    n: *const lapack_int, k: *const lapack_int, nb: *const lapack_int,
                    v: *const lapack_complex_float, ldv: *const lapack_int,
                    t: *const lapack_complex_float, ldt: *const lapack_int,
                    c: *mut lapack_complex_float, ldc: *const lapack_int,
                    work: *mut lapack_complex_float, info: *mut lapack_int);
    pub fn zgemqrt_(side: *const c_char, trans: *const c_char, m: *const lapack_int,
                    n: *const lapack_int, k: *const lapack_int, nb: *const lapack_int,
                    v: *const lapack_complex_double, ldv: *const lapack_int,
                    t: *const lapack_complex_double, ldt: *const lapack_int,
                    c: *mut lapack_complex_double, ldc: *const lapack_int,
                    work: *mut lapack_complex_double, info: *mut lapack_int);

    pub fn sgeqrt_(m: *const lapack_int, n: *const lapack_int, nb: *const lapack_int,
                   a: *mut c_float, lda: *const lapack_int, t: *mut c_float,
                   ldt: *const lapack_int, work: *mut c_float, info: *mut lapack_int);
    pub fn dgeqrt_(m: *const lapack_int, n: *const lapack_int, nb: *const lapack_int,
                   a: *mut c_double, lda: *const lapack_int, t: *mut c_double,
                   ldt: *const lapack_int, work: *mut c_double, info: *mut lapack_int);
    pub fn cgeqrt_(m: *const lapack_int, n: *const lapack_int, nb: *const lapack_int,
                   a: *mut lapack_complex_float, lda: *const lapack_int,
                   t: *mut lapack_complex_float, ldt: *const lapack_int,
                   work: *mut lapack_complex_float, info: *mut lapack_int);
    pub fn zgeqrt_(m: *const lapack_int, n: *const lapack_int, nb: *const lapack_int,
                   a: *mut lapack_complex_double, lda: *const lapack_int,
                   t: *mut lapack_complex_double, ldt: *const lapack_int,
                   work: *mut lapack_complex_double, info: *mut lapack_int);

    pub fn sgeqrt2_(m: *const lapack_int, n: *const lapack_int, a: *mut c_float,
                    lda: *const lapack_int, t: *mut c_float, ldt: *const lapack_int,
                    info: *mut lapack_int);
    pub fn dgeqrt2_(m: *const lapack_int, n: *const lapack_int, a: *mut c_double,
                    lda: *const lapack_int, t: *mut c_double, ldt: *const lapack_int,
                    info: *mut lapack_int);
    pub fn cgeqrt2_(m: *const lapack_int, n: *const lapack_int, a: *mut lapack_complex_float,
                    lda: *const lapack_int, t: *mut lapack_complex_float,
                    ldt: *const lapack_int, info: *mut lapack_int);
    pub fn zgeqrt2_(m: *const lapack_int, n: *const lapack_int, a: *mut lapack_complex_double,
                    lda: *const lapack_int, t: *mut lapack_complex_double,
                    ldt: *const lapack_int, info: *mut lapack_int);

    pub fn sgeqrt3_(m: *const lapack_int, n: *const lapack_int, a: *mut c_float,
                    lda: *const lapack_int, t: *mut c_float, ldt: *const lapack_int,
                    info: *mut lapack_int);
    pub fn dgeqrt3_(m: *const lapack_int, n: *const lapack_int, a: *mut c_double,
                    lda: *const lapack_int, t: *mut c_double, ldt: *const lapack_int,
                    info: *mut lapack_int);
    pub fn cgeqrt3_(m: *const lapack_int, n: *const lapack_int, a: *mut lapack_complex_float,
                    lda: *const lapack_int, t: *mut lapack_complex_float,
                    ldt: *const lapack_int, info: *mut lapack_int);
    pub fn zgeqrt3_(m: *const lapack_int, n: *const lapack_int, a: *mut lapack_complex_double,
                    lda: *const lapack_int, t: *mut lapack_complex_double,
                    ldt: *const lapack_int, info: *mut lapack_int);

    pub fn stpmqrt_(side: *const c_char, trans: *const c_char, m: *const lapack_int,
                    n: *const lapack_int, k: *const lapack_int, l: *const lapack_int,
                    nb: *const lapack_int, v: *const c_float, ldv: *const lapack_int,
                    t: *const c_float, ldt: *const lapack_int, a: *mut c_float,
                    lda: *const lapack_int, b: *mut c_float, ldb: *const lapack_int,
                    work: *mut c_float, info: *mut lapack_int);
    pub fn dtpmqrt_(side: *const c_char, trans: *const c_char, m: *const lapack_int,
                    n: *const lapack_int, k: *const lapack_int, l: *const lapack_int,
                    nb: *const lapack_int, v: *const c_double, ldv: *const lapack_int,
                    t: *const c_double, ldt: *const lapack_int, a: *mut c_double,
                    lda: *const lapack_int, b: *mut c_double, ldb: *const lapack_int,
                    work: *mut c_double, info: *mut lapack_int);
    pub fn ctpmqrt_(side: *const c_char, trans: *const c_char, m: *const lapack_int,
                    n: *const lapack_int, k: *const lapack_int, l: *const lapack_int,
                    nb: *const lapack_int, v: *const lapack_complex_float,
                    ldv: *const lapack_int, t: *const lapack_complex_float,
                    ldt: *const lapack_int, a: *mut lapack_complex_float,
                    lda: *const lapack_int, b: *mut lapack_complex_float,
                    ldb: *const lapack_int, work: *mut lapack_complex_float,
                    info: *mut lapack_int);
    pub fn ztpmqrt_(side: *const c_char, trans: *const c_char, m: *const lapack_int,
                    n: *const lapack_int, k: *const lapack_int, l: *const lapack_int,
                    nb: *const lapack_int, v: *const lapack_complex_double,
                    ldv: *const lapack_int, t: *const lapack_complex_double,
                    ldt: *const lapack_int, a: *mut lapack_complex_double,
                    lda: *const lapack_int, b: *mut lapack_complex_double,
                    ldb: *const lapack_int, work: *mut lapack_complex_double,
                    info: *mut lapack_int);

    pub fn stpqrt_(m: *const lapack_int, n: *const lapack_int, l: *const lapack_int,
                   nb: *const lapack_int, a: *mut c_float, lda: *const lapack_int,
                   b: *mut c_float, ldb: *const lapack_int, t: *mut c_float,
                   ldt: *const lapack_int, work: *mut c_float, info: *mut lapack_int);
    pub fn dtpqrt_(m: *const lapack_int, n: *const lapack_int, l: *const lapack_int,
                   nb: *const lapack_int, a: *mut c_double, lda: *const lapack_int,
                   b: *mut c_double, ldb: *const lapack_int, t: *mut c_double,
                   ldt: *const lapack_int, work: *mut c_double, info: *mut lapack_int);
    pub fn ctpqrt_(m: *const lapack_int, n: *const lapack_int, l: *const lapack_int,
                   nb: *const lapack_int, a: *mut lapack_complex_float, lda: *const lapack_int,
                   b: *mut lapack_complex_float, ldb: *const lapack_int,
                   t: *mut lapack_complex_float, ldt: *const lapack_int,
                   work: *mut lapack_complex_float, info: *mut lapack_int);
    pub fn ztpqrt_(m: *const lapack_int, n: *const lapack_int, l: *const lapack_int,
                   nb: *const lapack_int, a: *mut lapack_complex_double, lda: *const lapack_int,
                   b: *mut lapack_complex_double, ldb: *const lapack_int,
                   t: *mut lapack_complex_double, ldt: *const lapack_int,
                   work: *mut lapack_complex_double, info: *mut lapack_int);

    pub fn stpqrt2_(m: *const lapack_int, n: *const lapack_int, l: *const lapack_int,
                    a: *mut c_float, lda: *const lapack_int, b: *mut c_float,
                    ldb: *const lapack_int, t: *mut c_float, ldt: *const lapack_int,
                    info: *mut lapack_int);
    pub fn dtpqrt2_(m: *const lapack_int, n: *const lapack_int, l: *const lapack_int,
                    a: *mut c_double, lda: *const lapack_int, b: *mut c_double,
                    ldb: *const lapack_int, t: *mut c_double, ldt: *const lapack_int,
                    info: *mut lapack_int);
    pub fn ctpqrt2_(m: *const lapack_int, n: *const lapack_int, l: *const lapack_int,
                    a: *mut lapack_complex_float, lda: *const lapack_int,
                    b: *mut lapack_complex_float, ldb: *const lapack_int,
                    t: *mut lapack_complex_float, ldt: *const lapack_int, info: *mut lapack_int);
    pub fn ztpqrt2_(m: *const lapack_int, n: *const lapack_int, l: *const lapack_int,
                    a: *mut lapack_complex_double, lda: *const lapack_int,
                    b: *mut lapack_complex_double, ldb: *const lapack_int,
                    t: *mut lapack_complex_double, ldt: *const lapack_int, info: *mut lapack_int);

    pub fn stprfb_(side: *const c_char, trans: *const c_char, direct: *const c_char,
                   storev: *const c_char, m: *const lapack_int, n: *const lapack_int,
                   k: *const lapack_int, l: *const lapack_int, v: *const c_float,
                   ldv: *const lapack_int, t: *const c_float, ldt: *const lapack_int,
                   a: *mut c_float, lda: *const lapack_int, b: *mut c_float,
                   ldb: *const lapack_int, work: *const c_float, ldwork: *const lapack_int);
    pub fn dtprfb_(side: *const c_char, trans: *const c_char, direct: *const c_char,
                   storev: *const c_char, m: *const lapack_int, n: *const lapack_int,
                   k: *const lapack_int, l: *const lapack_int, v: *const c_double,
                   ldv: *const lapack_int, t: *const c_double, ldt: *const lapack_int,
                   a: *mut c_double, lda: *const lapack_int, b: *mut c_double,
                   ldb: *const lapack_int, work: *const c_double, ldwork: *const lapack_int);
    pub fn ctprfb_(side: *const c_char, trans: *const c_char, direct: *const c_char,
                   storev: *const c_char, m: *const lapack_int, n: *const lapack_int,
                   k: *const lapack_int, l: *const lapack_int, v: *const lapack_complex_float,
                   ldv: *const lapack_int, t: *const lapack_complex_float,
                   ldt: *const lapack_int, a: *mut lapack_complex_float, lda: *const lapack_int,
                   b: *mut lapack_complex_float, ldb: *const lapack_int,
                   work: *mut lapack_complex_float, ldwork: *const lapack_int);
    pub fn ztprfb_(side: *const c_char, trans: *const c_char, direct: *const c_char,
                   storev: *const c_char, m: *const lapack_int, n: *const lapack_int,
                   k: *const lapack_int, l: *const lapack_int, v: *const lapack_complex_double,
                   ldv: *const lapack_int, t: *const lapack_complex_double,
                   ldt: *const lapack_int, a: *mut lapack_complex_double,
                   lda: *const lapack_int, b: *mut lapack_complex_double,
                   ldb: *const lapack_int, work: *mut lapack_complex_double,
                   ldwork: *const lapack_int);

    // Version 3.5.0
    pub fn ssysv_rook_(uplo: *const c_char, n: *const lapack_int, nrhs: *const lapack_int,
                       a: *mut c_float, lda: *const lapack_int, ipiv: *mut lapack_int,
                       b: *mut c_float, ldb: *const lapack_int, work: *mut c_float,
                       lwork: *const lapack_int, info: *mut lapack_int);
    pub fn dsysv_rook_(uplo: *const c_char, n: *const lapack_int, nrhs: *const lapack_int,
                       a: *mut c_double, lda: *const lapack_int, ipiv: *mut lapack_int,
                       b: *mut c_double, ldb: *const lapack_int, work: *mut c_double,
                       lwork: *const lapack_int, info: *mut lapack_int);
    pub fn csysv_rook_(uplo: *const c_char, n: *const lapack_int, nrhs: *const lapack_int,
                       a: *mut lapack_complex_float, lda: *const lapack_int,
                       ipiv: *mut lapack_int, b: *mut lapack_complex_float,
                       ldb: *const lapack_int, work: *mut lapack_complex_float,
                       lwork: *const lapack_int, info: *mut lapack_int);
    pub fn zsysv_rook_(uplo: *const c_char, n: *const lapack_int, nrhs: *const lapack_int,
                       a: *mut lapack_complex_double, lda: *const lapack_int,
                       ipiv: *mut lapack_int, b: *mut lapack_complex_double,
                       ldb: *const lapack_int, work: *mut lapack_complex_double,
                       lwork: *const lapack_int, info: *mut lapack_int);

    pub fn ssytrf_rook_(uplo: *const c_char, n: *const lapack_int, a: *mut c_float,
                        lda: *const lapack_int, ipiv: *mut lapack_int, work: *mut c_float,
                        lwork: *const lapack_int, info: *mut lapack_int);
    pub fn dsytrf_rook_(uplo: *const c_char, n: *const lapack_int, a: *mut c_double,
                        lda: *const lapack_int, ipiv: *mut lapack_int, work: *mut c_double,
                        lwork: *const lapack_int, info: *mut lapack_int);
    pub fn csytrf_rook_(uplo: *const c_char, n: *const lapack_int, a: *mut lapack_complex_float,
                        lda: *const lapack_int, ipiv: *mut lapack_int,
                        work: *mut lapack_complex_float, lwork: *const lapack_int,
                        info: *mut lapack_int);
    pub fn zsytrf_rook_(uplo: *const c_char, n: *const lapack_int,
                        a: *mut lapack_complex_double, lda: *const lapack_int,
                        ipiv: *mut lapack_int, work: *mut lapack_complex_double,
                        lwork: *const lapack_int, info: *mut lapack_int);

    pub fn ssytrs_rook_(uplo: *const c_char, n: *const lapack_int, nrhs: *const lapack_int,
                        a: *const c_float, lda: *const lapack_int, ipiv: *const lapack_int,
                        b: *mut c_float, ldb: *const lapack_int, info: *mut lapack_int);
    pub fn dsytrs_rook_(uplo: *const c_char, n: *const lapack_int, nrhs: *const lapack_int,
                        a: *const c_double, lda: *const lapack_int, ipiv: *const lapack_int,
                        b: *mut c_double, ldb: *const lapack_int, info: *mut lapack_int);
    pub fn csytrs_rook_(uplo: *const c_char, n: *const lapack_int, nrhs: *const lapack_int,
                        a: *const lapack_complex_float, lda: *const lapack_int,
                        ipiv: *const lapack_int, b: *mut lapack_complex_float,
                        ldb: *const lapack_int, info: *mut lapack_int);
    pub fn zsytrs_rook_(uplo: *const c_char, n: *const lapack_int, nrhs: *const lapack_int,
                        a: *const lapack_complex_double, lda: *const lapack_int,
                        ipiv: *const lapack_int, b: *mut lapack_complex_double,
                        ldb: *const lapack_int, info: *mut lapack_int);

    pub fn chetrf_rook_(uplo: *const c_char, n: *const lapack_int, a: *mut lapack_complex_float,
                        lda: *const lapack_int, ipiv: *mut lapack_int,
                        work: *mut lapack_complex_float, lwork: *const lapack_int,
                        info: *mut lapack_int);
    pub fn zhetrf_rook_(uplo: *const c_char, n: *const lapack_int,
                        a: *mut lapack_complex_double, lda: *const lapack_int,
                        ipiv: *mut lapack_int, work: *mut lapack_complex_double,
                        lwork: *const lapack_int, info: *mut lapack_int);

    pub fn chetrs_rook_(uplo: *const c_char, n: *const lapack_int, nrhs: *const lapack_int,
                        a: *const lapack_complex_float, lda: *const lapack_int,
                        ipiv: *const lapack_int, b: *mut lapack_complex_float,
                        ldb: *const lapack_int, info: *mut lapack_int);
    pub fn zhetrs_rook_(uplo: *const c_char, n: *const lapack_int, nrhs: *const lapack_int,
                        a: *const lapack_complex_double, lda: *const lapack_int,
                        ipiv: *const lapack_int, b: *mut lapack_complex_double,
                        ldb: *const lapack_int, info: *mut lapack_int);

    pub fn csyr_(uplo: *const c_char, n: *const lapack_int, alpha: *const lapack_complex_float,
                 x: *const lapack_complex_float, incx: *const lapack_int,
                 a: *mut lapack_complex_float, lda: *const lapack_int);
    pub fn zsyr_(uplo: *const c_char, n: *const lapack_int, alpha: *const lapack_complex_double,
                 x: *const lapack_complex_double, incx: *const lapack_int,
                 a: *mut lapack_complex_double, lda: *const lapack_int);

    pub fn ilaver_(vers_major: *mut lapack_int, vers_minor: *mut lapack_int,
                   vers_patch: *mut lapack_int);
"""

select_re = re.compile("LAPACK_(\w)_SELECT(\d)")

def is_const(name, cty):
    return "*const" in cty

def is_mut(name, cty):
    return "*mut" in cty

def is_scalar(name, cty, f):
    return (
        "c_char" in cty or
        name in [
            "abnrm",
            "abstol",
            "amax",
            "anorm",
            "bbnrm",
            "colcnd",
            "dif",
            "ihi",
            "il",
            "ilo",
            "info",
            "iter",
            "iu",
            "l",
            "liwork",
            "lrwork",
            "lwork",
            "m",
            "mm",
            "n",
            "n_err_bnds",
            "nb",
            "nrhs",
            "q",
            "rank",
            "rcond",
            "rowcnd",
            "rpvgrw",
            "sdim",
            "tryrac",
            "vu",
        ] or
        not (
            'geev' in f.name or
            'tgsna' in f.name or
            'trsna' in f.name
        ) and name in [
            "vl",
            "vr",
        ] or
        not ('tgevc' in f.name) and name in [
            "p",
        ] or
        name.startswith("alpha") or
        name.startswith("beta") or
        name.startswith("ifail") or
        name.startswith("inc") or
        name.startswith("k") or
        name.startswith("ld") or
        name.startswith("tol") or
        name.startswith("vers")
    )

def translate_argument(name, cty, f):
    if is_const(name, cty):
        base = translate_type_base(cty, f)
        if is_scalar(name, cty, f):
            return base
        else:
            return "&[{}]".format(base)
    elif is_mut(name, cty):
        base = translate_type_base(cty, f)
        if is_scalar(name, cty, f):
            return "&mut {}".format(base)
        else:
            return "&mut [{}]".format(base)

    m = select_re.match(cty)
    if m is not None:
        if m.group(1) == 'S':
            return "Select{}F32".format(m.group(2))
        elif m.group(1) == 'D':
            return "Select{}F64".format(m.group(2))
        elif m.group(1) == 'C':
            return "Select{}C32".format(m.group(2))
        elif m.group(1) == 'Z':
            return "Select{}C64".format(m.group(2))

    assert False, "cannot translate `{}: {}`".format(name, cty)

def translate_type_base(cty, f):
    if "c_char" in cty:
        return "u8"
    elif "lapack_int" in cty or "lapack_logical" in cty:
        return "i32"
    elif "lapack_complex_double" in cty:
        return "c64"
    elif "lapack_complex_float" in cty:
        return "c32"
    elif "c_double" in cty:
        return "f64"
    elif "c_float" in cty:
        return "f32"

    assert False, "cannot translate `{}` in `{}`".format(cty, f.name)

def translate_body_argument(name, rty):
    if rty == "u8":
        return "&({} as c_char)".format(name)
    elif rty == "&mut u8":
        return "{} as *mut _ as *mut _".format(name)

    elif rty == "i32":
        return "&{}".format(name)
    elif rty == "&mut i32":
        return name
    elif rty == "&[i32]":
        return "{}.as_ptr()".format(name)
    elif rty == "&mut [i32]":
        return "{}.as_mut_ptr()".format(name)

    elif rty.startswith("f"):
        return "&{}".format(name)
    elif rty.startswith("&mut f"):
        return "{}".format(name)
    elif rty.startswith("&[f"):
        return "{}.as_ptr()".format(name)
    elif rty.startswith("&mut [f"):
        return "{}.as_mut_ptr()".format(name)

    elif rty.startswith("c"):
        return "&{} as *const _ as *const _".format(name)
    elif rty.startswith("&mut c"):
        return "{} as *mut _ as *mut _".format(name)
    elif rty.startswith("&[c"):
        return "{}.as_ptr() as *const _".format(name)
    elif rty.startswith("&mut [c"):
        return "{}.as_mut_ptr() as *mut _".format(name)

    if rty.startswith("Select"):
        return "transmute({})".format(name)

    assert False, "cannot translate `{}: {}`".format(name, rty)

def translate_return_type(cty):
    if cty == "c_float":
        return "f32"
    elif cty == "c_double":
        return "f64"

    assert False, "cannot translate `{}`".format(cty)

def format_header(f):
    args = format_header_arguments(f)
    if f.ret is None:
        return "pub unsafe fn {}({})".format(f.name, args)
    else:
        return "pub unsafe fn {}({}) -> {}".format(f.name, args, translate_return_type(f.ret))

def format_body(f):
    return "ffi::{}_({})".format(f.name, format_body_arguments(f))

def format_header_arguments(f):
    s = []
    for arg in f.args:
        s.append("{}: {}".format(arg[0], translate_argument(*arg, f=f)))
    return ", ".join(s)

def format_body_arguments(f):
    s = []
    for arg in f.args:
        rty = translate_argument(*arg, f=f)
        s.append(translate_body_argument(arg[0], rty))
    return ", ".join(s)

def prepare(code):
    lines = filter(lambda line: not re.match(r'^\s*//.*', line), code.split('\n'))
    lines = re.sub(r'\s+', ' ', "".join(lines)).strip().split(';')
    lines = filter(lambda line: not re.match(r'^\s*$', line), lines)
    return [Function.parse(line) for line in lines]

def do(functions):
    for f in functions:
        print("\n#[inline]")
        print(format_header(f) + " {")
        print("    " + format_body(f) + "\n}")

do(prepare(functions))
