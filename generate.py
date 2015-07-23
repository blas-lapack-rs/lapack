import re

definitions = """
pub fn sgetrf_(m: *const c_int, n: *const c_int, a: *mut c_float, lda: *const c_int,
               ipiv: *mut c_int, info: *mut c_int);
pub fn dgetrf_(m: *const c_int, n: *const c_int, a: *mut c_double, lda: *const c_int,
               ipiv: *mut c_int, info: *mut c_int);
pub fn cgetrf_(m: *const c_int, n: *const c_int, a: *mut complex_float, lda: *const c_int,
               ipiv: *mut c_int, info: *mut c_int);
pub fn zgetrf_(m: *const c_int, n: *const c_int, a: *mut complex_double, lda: *const c_int,
               ipiv: *mut c_int, info: *mut c_int);

pub fn sgbtrf_(m: *const c_int, n: *const c_int, kl: *const c_int, ku: *const c_int,
               ab: *mut c_float, ldab: *const c_int, ipiv: *mut c_int, info: *mut c_int);
pub fn dgbtrf_(m: *const c_int, n: *const c_int, kl: *const c_int, ku: *const c_int,
               ab: *mut c_double, ldab: *const c_int, ipiv: *mut c_int, info: *mut c_int);
pub fn cgbtrf_(m: *const c_int, n: *const c_int, kl: *const c_int, ku: *const c_int,
               ab: *mut complex_float, ldab: *const c_int, ipiv: *mut c_int, info: *mut c_int);
pub fn zgbtrf_(m: *const c_int, n: *const c_int, kl: *const c_int, ku: *const c_int,
               ab: *mut complex_double, ldab: *const c_int, ipiv: *mut c_int,
               info: *mut c_int);

pub fn sgttrf_(n: *const c_int, dl: *mut c_float, d: *mut c_float, du: *mut c_float,
               du2: *mut c_float, ipiv: *mut c_int, info: *mut c_int);
pub fn dgttrf_(n: *const c_int, dl: *mut c_double, d: *mut c_double, du: *mut c_double,
               du2: *mut c_double, ipiv: *mut c_int, info: *mut c_int);
pub fn cgttrf_(n: *const c_int, dl: *mut complex_float, d: *mut complex_float,
               du: *mut complex_float, du2: *mut complex_float, ipiv: *mut c_int,
               info: *mut c_int);
pub fn zgttrf_(n: *const c_int, dl: *mut complex_double, d: *mut complex_double,
               du: *mut complex_double, du2: *mut complex_double, ipiv: *mut c_int,
               info: *mut c_int);

pub fn spotrf_(uplo: *const c_char, n: *const c_int, a: *mut c_float, lda: *const c_int,
               info: *mut c_int);
pub fn dpotrf_(uplo: *const c_char, n: *const c_int, a: *mut c_double, lda: *const c_int,
               info: *mut c_int);
pub fn cpotrf_(uplo: *const c_char, n: *const c_int, a: *mut complex_float, lda: *const c_int,
               info: *mut c_int);
pub fn zpotrf_(uplo: *const c_char, n: *const c_int, a: *mut complex_double, lda: *const c_int,
               info: *mut c_int);

pub fn spstrf_(uplo: *const c_char, n: *const c_int, a: *mut c_float, lda: *const c_int,
               piv: *mut c_int, rank: *mut c_int, tol: *const c_float, work: *mut c_float,
               info: *mut c_int);
pub fn dpstrf_(uplo: *const c_char, n: *const c_int, a: *mut c_double, lda: *const c_int,
               piv: *mut c_int, rank: *mut c_int, tol: *const c_double, work: *mut c_double,
               info: *mut c_int);
pub fn cpstrf_(uplo: *const c_char, n: *const c_int, a: *mut complex_float, lda: *const c_int,
               piv: *mut c_int, rank: *mut c_int, tol: *const c_float, work: *mut c_float,
               info: *mut c_int);
pub fn zpstrf_(uplo: *const c_char, n: *const c_int, a: *mut complex_double, lda: *const c_int,
               piv: *mut c_int, rank: *mut c_int, tol: *const c_double, work: *mut c_double,
               info: *mut c_int);

pub fn spftrf_(transr: *const c_char, uplo: *const c_char, n: *const c_int, a: *mut c_float,
               info: *mut c_int);
pub fn dpftrf_(transr: *const c_char, uplo: *const c_char, n: *const c_int, a: *mut c_double,
               info: *mut c_int);
pub fn cpftrf_(transr: *const c_char, uplo: *const c_char, n: *const c_int,
               a: *mut complex_float, info: *mut c_int);
pub fn zpftrf_(transr: *const c_char, uplo: *const c_char, n: *const c_int,
               a: *mut complex_double, info: *mut c_int);

pub fn spptrf_(uplo: *const c_char, n: *const c_int, ap: *mut c_float, info: *mut c_int);
pub fn dpptrf_(uplo: *const c_char, n: *const c_int, ap: *mut c_double, info: *mut c_int);
pub fn cpptrf_(uplo: *const c_char, n: *const c_int, ap: *mut complex_float, info: *mut c_int);
pub fn zpptrf_(uplo: *const c_char, n: *const c_int, ap: *mut complex_double,
               info: *mut c_int);

pub fn spbtrf_(uplo: *const c_char, n: *const c_int, kd: *const c_int, ab: *mut c_float,
               ldab: *const c_int, info: *mut c_int);
pub fn dpbtrf_(uplo: *const c_char, n: *const c_int, kd: *const c_int, ab: *mut c_double,
               ldab: *const c_int, info: *mut c_int);
pub fn cpbtrf_(uplo: *const c_char, n: *const c_int, kd: *const c_int, ab: *mut complex_float,
               ldab: *const c_int, info: *mut c_int);
pub fn zpbtrf_(uplo: *const c_char, n: *const c_int, kd: *const c_int, ab: *mut complex_double,
               ldab: *const c_int, info: *mut c_int);

pub fn spttrf_(n: *const c_int, d: *mut c_float, e: *mut c_float, info: *mut c_int);
pub fn dpttrf_(n: *const c_int, d: *mut c_double, e: *mut c_double, info: *mut c_int);
pub fn cpttrf_(n: *const c_int, d: *mut c_float, e: *mut complex_float, info: *mut c_int);
pub fn zpttrf_(n: *const c_int, d: *mut c_double, e: *mut complex_double, info: *mut c_int);

pub fn ssytrf_(uplo: *const c_char, n: *const c_int, a: *mut c_float, lda: *const c_int,
               ipiv: *mut c_int, work: *mut c_float, lwork: *const c_int, info: *mut c_int);
pub fn dsytrf_(uplo: *const c_char, n: *const c_int, a: *mut c_double, lda: *const c_int,
               ipiv: *mut c_int, work: *mut c_double, lwork: *const c_int, info: *mut c_int);
pub fn csytrf_(uplo: *const c_char, n: *const c_int, a: *mut complex_float, lda: *const c_int,
               ipiv: *mut c_int, work: *mut complex_float, lwork: *const c_int,
               info: *mut c_int);
pub fn zsytrf_(uplo: *const c_char, n: *const c_int, a: *mut complex_double, lda: *const c_int,
               ipiv: *mut c_int, work: *mut complex_double, lwork: *const c_int,
               info: *mut c_int);

pub fn chetrf_(uplo: *const c_char, n: *const c_int, a: *mut complex_float, lda: *const c_int,
               ipiv: *mut c_int, work: *mut complex_float, lwork: *const c_int,
               info: *mut c_int);
pub fn zhetrf_(uplo: *const c_char, n: *const c_int, a: *mut complex_double, lda: *const c_int,
               ipiv: *mut c_int, work: *mut complex_double, lwork: *const c_int,
               info: *mut c_int);

pub fn ssptrf_(uplo: *const c_char, n: *const c_int, ap: *mut c_float, ipiv: *mut c_int,
               info: *mut c_int);
pub fn dsptrf_(uplo: *const c_char, n: *const c_int, ap: *mut c_double, ipiv: *mut c_int,
               info: *mut c_int);
pub fn csptrf_(uplo: *const c_char, n: *const c_int, ap: *mut complex_float, ipiv: *mut c_int,
               info: *mut c_int);
pub fn zsptrf_(uplo: *const c_char, n: *const c_int, ap: *mut complex_double, ipiv: *mut c_int,
               info: *mut c_int);

pub fn chptrf_(uplo: *const c_char, n: *const c_int, ap: *mut complex_float, ipiv: *mut c_int,
               info: *mut c_int);
pub fn zhptrf_(uplo: *const c_char, n: *const c_int, ap: *mut complex_double, ipiv: *mut c_int,
               info: *mut c_int);

pub fn sgetrs_(trans: *const c_char, n: *const c_int, nrhs: *const c_int, a: *const c_float,
               lda: *const c_int, ipiv: *const c_int, b: *mut c_float, ldb: *const c_int,
               info: *mut c_int);
pub fn dgetrs_(trans: *const c_char, n: *const c_int, nrhs: *const c_int, a: *const c_double,
               lda: *const c_int, ipiv: *const c_int, b: *mut c_double, ldb: *const c_int,
               info: *mut c_int);
pub fn cgetrs_(trans: *const c_char, n: *const c_int, nrhs: *const c_int,
               a: *const complex_float, lda: *const c_int, ipiv: *const c_int,
               b: *mut complex_float, ldb: *const c_int, info: *mut c_int);
pub fn zgetrs_(trans: *const c_char, n: *const c_int, nrhs: *const c_int,
               a: *const complex_double, lda: *const c_int, ipiv: *const c_int,
               b: *mut complex_double, ldb: *const c_int, info: *mut c_int);

pub fn sgbtrs_(trans: *const c_char, n: *const c_int, kl: *const c_int, ku: *const c_int,
               nrhs: *const c_int, ab: *const c_float, ldab: *const c_int, ipiv: *const c_int,
               b: *mut c_float, ldb: *const c_int, info: *mut c_int);
pub fn dgbtrs_(trans: *const c_char, n: *const c_int, kl: *const c_int, ku: *const c_int,
               nrhs: *const c_int, ab: *const c_double, ldab: *const c_int, ipiv: *const c_int,
               b: *mut c_double, ldb: *const c_int, info: *mut c_int);
pub fn cgbtrs_(trans: *const c_char, n: *const c_int, kl: *const c_int, ku: *const c_int,
               nrhs: *const c_int, ab: *const complex_float, ldab: *const c_int,
               ipiv: *const c_int, b: *mut complex_float, ldb: *const c_int, info: *mut c_int);
pub fn zgbtrs_(trans: *const c_char, n: *const c_int, kl: *const c_int, ku: *const c_int,
               nrhs: *const c_int, ab: *const complex_double, ldab: *const c_int,
               ipiv: *const c_int, b: *mut complex_double, ldb: *const c_int,
               info: *mut c_int);

pub fn sgttrs_(trans: *const c_char, n: *const c_int, nrhs: *const c_int, dl: *const c_float,
               d: *const c_float, du: *const c_float, du2: *const c_float, ipiv: *const c_int,
               b: *mut c_float, ldb: *const c_int, info: *mut c_int);
pub fn dgttrs_(trans: *const c_char, n: *const c_int, nrhs: *const c_int, dl: *const c_double,
               d: *const c_double, du: *const c_double, du2: *const c_double,
               ipiv: *const c_int, b: *mut c_double, ldb: *const c_int, info: *mut c_int);
pub fn cgttrs_(trans: *const c_char, n: *const c_int, nrhs: *const c_int,
               dl: *const complex_float, d: *const complex_float, du: *const complex_float,
               du2: *const complex_float, ipiv: *const c_int, b: *mut complex_float,
               ldb: *const c_int, info: *mut c_int);
pub fn zgttrs_(trans: *const c_char, n: *const c_int, nrhs: *const c_int,
               dl: *const complex_double, d: *const complex_double, du: *const complex_double,
               du2: *const complex_double, ipiv: *const c_int, b: *mut complex_double,
               ldb: *const c_int, info: *mut c_int);

pub fn spotrs_(uplo: *const c_char, n: *const c_int, nrhs: *const c_int, a: *const c_float,
               lda: *const c_int, b: *mut c_float, ldb: *const c_int, info: *mut c_int);
pub fn dpotrs_(uplo: *const c_char, n: *const c_int, nrhs: *const c_int, a: *const c_double,
               lda: *const c_int, b: *mut c_double, ldb: *const c_int, info: *mut c_int);
pub fn cpotrs_(uplo: *const c_char, n: *const c_int, nrhs: *const c_int,
               a: *const complex_float, lda: *const c_int, b: *mut complex_float,
               ldb: *const c_int, info: *mut c_int);
pub fn zpotrs_(uplo: *const c_char, n: *const c_int, nrhs: *const c_int,
               a: *const complex_double, lda: *const c_int, b: *mut complex_double,
               ldb: *const c_int, info: *mut c_int);

pub fn spftrs_(transr: *const c_char, uplo: *const c_char, n: *const c_int, nrhs: *const c_int,
               a: *const c_float, b: *mut c_float, ldb: *const c_int, info: *mut c_int);
pub fn dpftrs_(transr: *const c_char, uplo: *const c_char, n: *const c_int, nrhs: *const c_int,
               a: *const c_double, b: *mut c_double, ldb: *const c_int, info: *mut c_int);
pub fn cpftrs_(transr: *const c_char, uplo: *const c_char, n: *const c_int, nrhs: *const c_int,
               a: *const complex_float, b: *mut complex_float, ldb: *const c_int,
               info: *mut c_int);
pub fn zpftrs_(transr: *const c_char, uplo: *const c_char, n: *const c_int, nrhs: *const c_int,
               a: *const complex_double, b: *mut complex_double, ldb: *const c_int,
               info: *mut c_int);

pub fn spptrs_(uplo: *const c_char, n: *const c_int, nrhs: *const c_int, ap: *const c_float,
               b: *mut c_float, ldb: *const c_int, info: *mut c_int);
pub fn dpptrs_(uplo: *const c_char, n: *const c_int, nrhs: *const c_int, ap: *const c_double,
               b: *mut c_double, ldb: *const c_int, info: *mut c_int);
pub fn cpptrs_(uplo: *const c_char, n: *const c_int, nrhs: *const c_int,
               ap: *const complex_float, b: *mut complex_float, ldb: *const c_int,
               info: *mut c_int);
pub fn zpptrs_(uplo: *const c_char, n: *const c_int, nrhs: *const c_int,
               ap: *const complex_double, b: *mut complex_double, ldb: *const c_int,
               info: *mut c_int);

pub fn spbtrs_(uplo: *const c_char, n: *const c_int, kd: *const c_int, nrhs: *const c_int,
               ab: *const c_float, ldab: *const c_int, b: *mut c_float, ldb: *const c_int,
               info: *mut c_int);
pub fn dpbtrs_(uplo: *const c_char, n: *const c_int, kd: *const c_int, nrhs: *const c_int,
               ab: *const c_double, ldab: *const c_int, b: *mut c_double, ldb: *const c_int,
               info: *mut c_int);
pub fn cpbtrs_(uplo: *const c_char, n: *const c_int, kd: *const c_int, nrhs: *const c_int,
               ab: *const complex_float, ldab: *const c_int, b: *mut complex_float,
               ldb: *const c_int, info: *mut c_int);
pub fn zpbtrs_(uplo: *const c_char, n: *const c_int, kd: *const c_int, nrhs: *const c_int,
               ab: *const complex_double, ldab: *const c_int, b: *mut complex_double,
               ldb: *const c_int, info: *mut c_int);

pub fn spttrs_(n: *const c_int, nrhs: *const c_int, d: *const c_float, e: *const c_float,
               b: *mut c_float, ldb: *const c_int, info: *mut c_int);
pub fn dpttrs_(n: *const c_int, nrhs: *const c_int, d: *const c_double, e: *const c_double,
               b: *mut c_double, ldb: *const c_int, info: *mut c_int);
pub fn cpttrs_(uplo: *const c_char, n: *const c_int, nrhs: *const c_int, d: *const c_float,
               e: *const complex_float, b: *mut complex_float, ldb: *const c_int,
               info: *mut c_int);
pub fn zpttrs_(uplo: *const c_char, n: *const c_int, nrhs: *const c_int, d: *const c_double,
               e: *const complex_double, b: *mut complex_double, ldb: *const c_int,
               info: *mut c_int);

pub fn ssytrs_(uplo: *const c_char, n: *const c_int, nrhs: *const c_int, a: *const c_float,
               lda: *const c_int, ipiv: *const c_int, b: *mut c_float, ldb: *const c_int,
               info: *mut c_int);
pub fn dsytrs_(uplo: *const c_char, n: *const c_int, nrhs: *const c_int, a: *const c_double,
               lda: *const c_int, ipiv: *const c_int, b: *mut c_double, ldb: *const c_int,
               info: *mut c_int);
pub fn csytrs_(uplo: *const c_char, n: *const c_int, nrhs: *const c_int,
               a: *const complex_float, lda: *const c_int, ipiv: *const c_int,
               b: *mut complex_float, ldb: *const c_int, info: *mut c_int);
pub fn zsytrs_(uplo: *const c_char, n: *const c_int, nrhs: *const c_int,
               a: *const complex_double, lda: *const c_int, ipiv: *const c_int,
               b: *mut complex_double, ldb: *const c_int, info: *mut c_int);

pub fn chetrs_(uplo: *const c_char, n: *const c_int, nrhs: *const c_int,
               a: *const complex_float, lda: *const c_int, ipiv: *const c_int,
               b: *mut complex_float, ldb: *const c_int, info: *mut c_int);
pub fn zhetrs_(uplo: *const c_char, n: *const c_int, nrhs: *const c_int,
               a: *const complex_double, lda: *const c_int, ipiv: *const c_int,
               b: *mut complex_double, ldb: *const c_int, info: *mut c_int);

pub fn ssptrs_(uplo: *const c_char, n: *const c_int, nrhs: *const c_int, ap: *const c_float,
               ipiv: *const c_int, b: *mut c_float, ldb: *const c_int, info: *mut c_int);
pub fn dsptrs_(uplo: *const c_char, n: *const c_int, nrhs: *const c_int, ap: *const c_double,
               ipiv: *const c_int, b: *mut c_double, ldb: *const c_int, info: *mut c_int);
pub fn csptrs_(uplo: *const c_char, n: *const c_int, nrhs: *const c_int,
               ap: *const complex_float, ipiv: *const c_int, b: *mut complex_float,
               ldb: *const c_int, info: *mut c_int);
pub fn zsptrs_(uplo: *const c_char, n: *const c_int, nrhs: *const c_int,
               ap: *const complex_double, ipiv: *const c_int, b: *mut complex_double,
               ldb: *const c_int, info: *mut c_int);

pub fn chptrs_(uplo: *const c_char, n: *const c_int, nrhs: *const c_int,
               ap: *const complex_float, ipiv: *const c_int, b: *mut complex_float,
               ldb: *const c_int, info: *mut c_int);
pub fn zhptrs_(uplo: *const c_char, n: *const c_int, nrhs: *const c_int,
               ap: *const complex_double, ipiv: *const c_int, b: *mut complex_double,
               ldb: *const c_int, info: *mut c_int);

pub fn strtrs_(uplo: *const c_char, trans: *const c_char, diag: *const c_char, n: *const c_int,
               nrhs: *const c_int, a: *const c_float, lda: *const c_int, b: *mut c_float,
               ldb: *const c_int, info: *mut c_int);
pub fn dtrtrs_(uplo: *const c_char, trans: *const c_char, diag: *const c_char, n: *const c_int,
               nrhs: *const c_int, a: *const c_double, lda: *const c_int, b: *mut c_double,
               ldb: *const c_int, info: *mut c_int);
pub fn ctrtrs_(uplo: *const c_char, trans: *const c_char, diag: *const c_char, n: *const c_int,
               nrhs: *const c_int, a: *const complex_float, lda: *const c_int,
               b: *mut complex_float, ldb: *const c_int, info: *mut c_int);
pub fn ztrtrs_(uplo: *const c_char, trans: *const c_char, diag: *const c_char, n: *const c_int,
               nrhs: *const c_int, a: *const complex_double, lda: *const c_int,
               b: *mut complex_double, ldb: *const c_int, info: *mut c_int);

pub fn stptrs_(uplo: *const c_char, trans: *const c_char, diag: *const c_char, n: *const c_int,
               nrhs: *const c_int, ap: *const c_float, b: *mut c_float, ldb: *const c_int,
               info: *mut c_int);
pub fn dtptrs_(uplo: *const c_char, trans: *const c_char, diag: *const c_char, n: *const c_int,
               nrhs: *const c_int, ap: *const c_double, b: *mut c_double, ldb: *const c_int,
               info: *mut c_int);
pub fn ctptrs_(uplo: *const c_char, trans: *const c_char, diag: *const c_char, n: *const c_int,
               nrhs: *const c_int, ap: *const complex_float, b: *mut complex_float,
               ldb: *const c_int, info: *mut c_int);
pub fn ztptrs_(uplo: *const c_char, trans: *const c_char, diag: *const c_char, n: *const c_int,
               nrhs: *const c_int, ap: *const complex_double, b: *mut complex_double,
               ldb: *const c_int, info: *mut c_int);

pub fn stbtrs_(uplo: *const c_char, trans: *const c_char, diag: *const c_char, n: *const c_int,
               kd: *const c_int, nrhs: *const c_int, ab: *const c_float, ldab: *const c_int,
               b: *mut c_float, ldb: *const c_int, info: *mut c_int);
pub fn dtbtrs_(uplo: *const c_char, trans: *const c_char, diag: *const c_char, n: *const c_int,
               kd: *const c_int, nrhs: *const c_int, ab: *const c_double, ldab: *const c_int,
               b: *mut c_double, ldb: *const c_int, info: *mut c_int);
pub fn ctbtrs_(uplo: *const c_char, trans: *const c_char, diag: *const c_char, n: *const c_int,
               kd: *const c_int, nrhs: *const c_int, ab: *const complex_float,
               ldab: *const c_int, b: *mut complex_float, ldb: *const c_int, info: *mut c_int);
pub fn ztbtrs_(uplo: *const c_char, trans: *const c_char, diag: *const c_char, n: *const c_int,
               kd: *const c_int, nrhs: *const c_int, ab: *const complex_double,
               ldab: *const c_int, b: *mut complex_double, ldb: *const c_int,
               info: *mut c_int);

pub fn sgecon_(norm: *const c_char, n: *const c_int, a: *const c_float, lda: *const c_int,
               anorm: *const c_float, rcond: *mut c_float, work: *mut c_float,
               iwork: *mut c_int, info: *mut c_int);
pub fn dgecon_(norm: *const c_char, n: *const c_int, a: *const c_double, lda: *const c_int,
               anorm: *const c_double, rcond: *mut c_double, work: *mut c_double,
               iwork: *mut c_int, info: *mut c_int);
pub fn cgecon_(norm: *const c_char, n: *const c_int, a: *const complex_float,
               lda: *const c_int, anorm: *const c_float, rcond: *mut c_float,
               work: *mut complex_float, rwork: *mut c_float, info: *mut c_int);
pub fn zgecon_(norm: *const c_char, n: *const c_int, a: *const complex_double,
               lda: *const c_int, anorm: *const c_double, rcond: *mut c_double,
               work: *mut complex_double, rwork: *mut c_double, info: *mut c_int);

pub fn sgbcon_(norm: *const c_char, n: *const c_int, kl: *const c_int, ku: *const c_int,
               ab: *const c_float, ldab: *const c_int, ipiv: *const c_int,
               anorm: *const c_float, rcond: *mut c_float, work: *mut c_float,
               iwork: *mut c_int, info: *mut c_int);
pub fn dgbcon_(norm: *const c_char, n: *const c_int, kl: *const c_int, ku: *const c_int,
               ab: *const c_double, ldab: *const c_int, ipiv: *const c_int,
               anorm: *const c_double, rcond: *mut c_double, work: *mut c_double,
               iwork: *mut c_int, info: *mut c_int);
pub fn cgbcon_(norm: *const c_char, n: *const c_int, kl: *const c_int, ku: *const c_int,
               ab: *const complex_float, ldab: *const c_int, ipiv: *const c_int,
               anorm: *const c_float, rcond: *mut c_float, work: *mut complex_float,
               rwork: *mut c_float, info: *mut c_int);
pub fn zgbcon_(norm: *const c_char, n: *const c_int, kl: *const c_int, ku: *const c_int,
               ab: *const complex_double, ldab: *const c_int, ipiv: *const c_int,
               anorm: *const c_double, rcond: *mut c_double, work: *mut complex_double,
               rwork: *mut c_double, info: *mut c_int);

pub fn sgtcon_(norm: *const c_char, n: *const c_int, dl: *const c_float, d: *const c_float,
               du: *const c_float, du2: *const c_float, ipiv: *const c_int,
               anorm: *const c_float, rcond: *mut c_float, work: *mut c_float,
               iwork: *mut c_int, info: *mut c_int);
pub fn dgtcon_(norm: *const c_char, n: *const c_int, dl: *const c_double, d: *const c_double,
               du: *const c_double, du2: *const c_double, ipiv: *const c_int,
               anorm: *const c_double, rcond: *mut c_double, work: *mut c_double,
               iwork: *mut c_int, info: *mut c_int);
pub fn cgtcon_(norm: *const c_char, n: *const c_int, dl: *const complex_float,
               d: *const complex_float, du: *const complex_float, du2: *const complex_float,
               ipiv: *const c_int, anorm: *const c_float, rcond: *mut c_float,
               work: *mut complex_float, info: *mut c_int);
pub fn zgtcon_(norm: *const c_char, n: *const c_int, dl: *const complex_double,
               d: *const complex_double, du: *const complex_double, du2: *const complex_double,
               ipiv: *const c_int, anorm: *const c_double, rcond: *mut c_double,
               work: *mut complex_double, info: *mut c_int);

pub fn spocon_(uplo: *const c_char, n: *const c_int, a: *const c_float, lda: *const c_int,
               anorm: *const c_float, rcond: *mut c_float, work: *mut c_float,
               iwork: *mut c_int, info: *mut c_int);
pub fn dpocon_(uplo: *const c_char, n: *const c_int, a: *const c_double, lda: *const c_int,
               anorm: *const c_double, rcond: *mut c_double, work: *mut c_double,
               iwork: *mut c_int, info: *mut c_int);
pub fn cpocon_(uplo: *const c_char, n: *const c_int, a: *const complex_float,
               lda: *const c_int, anorm: *const c_float, rcond: *mut c_float,
               work: *mut complex_float, rwork: *mut c_float, info: *mut c_int);
pub fn zpocon_(uplo: *const c_char, n: *const c_int, a: *const complex_double,
               lda: *const c_int, anorm: *const c_double, rcond: *mut c_double,
               work: *mut complex_double, rwork: *mut c_double, info: *mut c_int);

pub fn sppcon_(uplo: *const c_char, n: *const c_int, ap: *const c_float, anorm: *const c_float,
               rcond: *mut c_float, work: *mut c_float, iwork: *mut c_int, info: *mut c_int);
pub fn dppcon_(uplo: *const c_char, n: *const c_int, ap: *const c_double,
               anorm: *const c_double, rcond: *mut c_double, work: *mut c_double, iwork: *mut c_int,
               info: *mut c_int);
pub fn cppcon_(uplo: *const c_char, n: *const c_int, ap: *const complex_float,
               anorm: *const c_float, rcond: *mut c_float, work: *mut complex_float,
               rwork: *mut c_float, info: *mut c_int);
pub fn zppcon_(uplo: *const c_char, n: *const c_int, ap: *const complex_double,
               anorm: *const c_double, rcond: *mut c_double, work: *mut complex_double,
               rwork: *mut c_double, info: *mut c_int);

pub fn spbcon_(uplo: *const c_char, n: *const c_int, kd: *const c_int, ab: *const c_float,
               ldab: *const c_int, anorm: *const c_float, rcond: *mut c_float,
               work: *mut c_float, iwork: *mut c_int, info: *mut c_int);
pub fn dpbcon_(uplo: *const c_char, n: *const c_int, kd: *const c_int, ab: *const c_double,
               ldab: *const c_int, anorm: *const c_double, rcond: *mut c_double,
               work: *mut c_double, iwork: *mut c_int, info: *mut c_int);
pub fn cpbcon_(uplo: *const c_char, n: *const c_int, kd: *const c_int,
               ab: *const complex_float, ldab: *const c_int, anorm: *const c_float,
               rcond: *mut c_float, work: *mut complex_float, rwork: *mut c_float,
               info: *mut c_int);
pub fn zpbcon_(uplo: *const c_char, n: *const c_int, kd: *const c_int,
               ab: *const complex_double, ldab: *const c_int, anorm: *const c_double,
               rcond: *mut c_double, work: *mut complex_double, rwork: *mut c_double,
               info: *mut c_int);

pub fn sptcon_(n: *const c_int, d: *const c_float, e: *const c_float, anorm: *const c_float,
               rcond: *mut c_float, work: *mut c_float, info: *mut c_int);
pub fn dptcon_(n: *const c_int, d: *const c_double, e: *const c_double, anorm: *const c_double,
               rcond: *mut c_double, work: *mut c_double, info: *mut c_int);
pub fn cptcon_(n: *const c_int, d: *const c_float, e: *const complex_float,
               anorm: *const c_float, rcond: *mut c_float, rwork: *mut c_float, info: *mut c_int);
pub fn zptcon_(n: *const c_int, d: *const c_double, e: *const complex_double,
               anorm: *const c_double, rcond: *mut c_double, rwork: *mut c_double,
               info: *mut c_int);

pub fn ssycon_(uplo: *const c_char, n: *const c_int, a: *const c_float, lda: *const c_int,
               ipiv: *const c_int, anorm: *const c_float, rcond: *mut c_float,
               work: *mut c_float, iwork: *mut c_int, info: *mut c_int);
pub fn dsycon_(uplo: *const c_char, n: *const c_int, a: *const c_double, lda: *const c_int,
               ipiv: *const c_int, anorm: *const c_double, rcond: *mut c_double,
               work: *mut c_double, iwork: *mut c_int, info: *mut c_int);
pub fn csycon_(uplo: *const c_char, n: *const c_int, a: *const complex_float,
               lda: *const c_int, ipiv: *const c_int, anorm: *const c_float,
               rcond: *mut c_float, work: *mut complex_float, info: *mut c_int);
pub fn zsycon_(uplo: *const c_char, n: *const c_int, a: *const complex_double,
               lda: *const c_int, ipiv: *const c_int, anorm: *const c_double,
               rcond: *mut c_double, work: *mut complex_double, info: *mut c_int);

pub fn checon_(uplo: *const c_char, n: *const c_int, a: *const complex_float,
               lda: *const c_int, ipiv: *const c_int, anorm: *const c_float,
               rcond: *mut c_float, work: *mut complex_float, info: *mut c_int);
pub fn zhecon_(uplo: *const c_char, n: *const c_int, a: *const complex_double,
               lda: *const c_int, ipiv: *const c_int, anorm: *const c_double,
               rcond: *mut c_double, work: *mut complex_double, info: *mut c_int);

pub fn sspcon_(uplo: *const c_char, n: *const c_int, ap: *const c_float, ipiv: *const c_int,
               anorm: *const c_float, rcond: *mut c_float, work: *mut c_float,
               iwork: *mut c_int, info: *mut c_int);
pub fn dspcon_(uplo: *const c_char, n: *const c_int, ap: *const c_double, ipiv: *const c_int,
               anorm: *const c_double, rcond: *mut c_double, work: *mut c_double,
               iwork: *mut c_int, info: *mut c_int);
pub fn cspcon_(uplo: *const c_char, n: *const c_int, ap: *const complex_float,
               ipiv: *const c_int, anorm: *const c_float, rcond: *mut c_float,
               work: *mut complex_float, info: *mut c_int);
pub fn zspcon_(uplo: *const c_char, n: *const c_int, ap: *const complex_double,
               ipiv: *const c_int, anorm: *const c_double, rcond: *mut c_double,
               work: *mut complex_double, info: *mut c_int);

pub fn chpcon_(uplo: *const c_char, n: *const c_int, ap: *const complex_float,
               ipiv: *const c_int, anorm: *const c_float, rcond: *mut c_float,
               work: *mut complex_float, info: *mut c_int);
pub fn zhpcon_(uplo: *const c_char, n: *const c_int, ap: *const complex_double,
               ipiv: *const c_int, anorm: *const c_double, rcond: *mut c_double,
               work: *mut complex_double, info: *mut c_int);

pub fn strcon_(norm: *const c_char, uplo: *const c_char, diag: *const c_char, n: *const c_int,
               a: *const c_float, lda: *const c_int, rcond: *mut c_float, work: *mut c_float,
               iwork: *mut c_int, info: *mut c_int);
pub fn dtrcon_(norm: *const c_char, uplo: *const c_char, diag: *const c_char, n: *const c_int,
               a: *const c_double, lda: *const c_int, rcond: *mut c_double,
               work: *mut c_double, iwork: *mut c_int, info: *mut c_int);
pub fn ctrcon_(norm: *const c_char, uplo: *const c_char, diag: *const c_char, n: *const c_int,
               a: *const complex_float, lda: *const c_int, rcond: *mut c_float,
               work: *mut complex_float, rwork: *mut c_float, info: *mut c_int);
pub fn ztrcon_(norm: *const c_char, uplo: *const c_char, diag: *const c_char, n: *const c_int,
               a: *const complex_double, lda: *const c_int, rcond: *mut c_double,
               work: *mut complex_double, rwork: *mut c_double, info: *mut c_int);

pub fn stpcon_(norm: *const c_char, uplo: *const c_char, diag: *const c_char, n: *const c_int,
               ap: *const c_float, rcond: *mut c_float, work: *mut c_float, iwork: *mut c_int,
               info: *mut c_int);
pub fn dtpcon_(norm: *const c_char, uplo: *const c_char, diag: *const c_char, n: *const c_int,
               ap: *const c_double, rcond: *mut c_double, work: *mut c_double,
               iwork: *mut c_int, info: *mut c_int);
pub fn ctpcon_(norm: *const c_char, uplo: *const c_char, diag: *const c_char, n: *const c_int,
               ap: *const complex_float, rcond: *mut c_float, work: *mut complex_float,
               rwork: *mut c_float, info: *mut c_int);
pub fn ztpcon_(norm: *const c_char, uplo: *const c_char, diag: *const c_char, n: *const c_int,
               ap: *const complex_double, rcond: *mut c_double, work: *mut complex_double,
               rwork: *mut c_double, info: *mut c_int);

pub fn stbcon_(norm: *const c_char, uplo: *const c_char, diag: *const c_char, n: *const c_int,
               kd: *const c_int, ab: *const c_float, ldab: *const c_int, rcond: *mut c_float,
               work: *mut c_float, iwork: *mut c_int, info: *mut c_int);
pub fn dtbcon_(norm: *const c_char, uplo: *const c_char, diag: *const c_char, n: *const c_int,
               kd: *const c_int, ab: *const c_double, ldab: *const c_int, rcond: *mut c_double,
               work: *mut c_double, iwork: *mut c_int, info: *mut c_int);
pub fn ctbcon_(norm: *const c_char, uplo: *const c_char, diag: *const c_char, n: *const c_int,
               kd: *const c_int, ab: *const complex_float, ldab: *const c_int,
               rcond: *mut c_float, work: *mut complex_float, rwork: *mut c_float,
               info: *mut c_int);
pub fn ztbcon_(norm: *const c_char, uplo: *const c_char, diag: *const c_char, n: *const c_int,
               kd: *const c_int, ab: *const complex_double, ldab: *const c_int,
               rcond: *mut c_double, work: *mut complex_double, rwork: *mut c_double,
               info: *mut c_int);

pub fn sgerfs_(trans: *const c_char, n: *const c_int, nrhs: *const c_int, a: *const c_float,
               lda: *const c_int, af: *const c_float, ldaf: *const c_int, ipiv: *const c_int,
               b: *const c_float, ldb: *const c_int, x: *mut c_float, ldx: *const c_int,
               ferr: *mut c_float, berr: *mut c_float, work: *mut c_float, iwork: *mut c_int,
               info: *mut c_int);
pub fn dgerfs_(trans: *const c_char, n: *const c_int, nrhs: *const c_int, a: *const c_double,
               lda: *const c_int, af: *const c_double, ldaf: *const c_int, ipiv: *const c_int,
               b: *const c_double, ldb: *const c_int, x: *mut c_double, ldx: *const c_int,
               ferr: *mut c_double, berr: *mut c_double, work: *mut c_double,
               iwork: *mut c_int, info: *mut c_int);
pub fn cgerfs_(trans: *const c_char, n: *const c_int, nrhs: *const c_int,
               a: *const complex_float, lda: *const c_int, af: *const complex_float,
               ldaf: *const c_int, ipiv: *const c_int, b: *const complex_float,
               ldb: *const c_int, x: *mut complex_float, ldx: *const c_int, ferr: *mut c_float,
               berr: *mut c_float, work: *mut complex_float, rwork: *mut c_float,
               info: *mut c_int);
pub fn zgerfs_(trans: *const c_char, n: *const c_int, nrhs: *const c_int,
               a: *const complex_double, lda: *const c_int, af: *const complex_double,
               ldaf: *const c_int, ipiv: *const c_int, b: *const complex_double,
               ldb: *const c_int, x: *mut complex_double, ldx: *const c_int,
               ferr: *mut c_double, berr: *mut c_double, work: *mut complex_double,
               rwork: *mut c_double, info: *mut c_int);

pub fn sgerfsx_(trans: *const c_char, equed: *const c_char, n: *const c_int,
                nrhs: *const c_int, a: *const c_float, lda: *const c_int, af: *const c_float,
                ldaf: *const c_int, ipiv: *const c_int, r: *const c_float, c: *const c_float,
                b: *const c_float, ldb: *const c_int, x: *mut c_float, ldx: *const c_int,
                rcond: *mut c_float, berr: *mut c_float, n_err_bnds: *const c_int,
                err_bnds_norm: *mut c_float, err_bnds_comp: *mut c_float,
                nparams: *const c_int, params: *mut c_float, work: *mut c_float,
                iwork: *mut c_int, info: *mut c_int);
pub fn dgerfsx_(trans: *const c_char, equed: *const c_char, n: *const c_int,
                nrhs: *const c_int, a: *const c_double, lda: *const c_int, af: *const c_double,
                ldaf: *const c_int, ipiv: *const c_int, r: *const c_double, c: *const c_double,
                b: *const c_double, ldb: *const c_int, x: *mut c_double, ldx: *const c_int,
                rcond: *mut c_double, berr: *mut c_double, n_err_bnds: *const c_int,
                err_bnds_norm: *mut c_double, err_bnds_comp: *mut c_double,
                nparams: *const c_int, params: *mut c_double, work: *mut c_double,
                iwork: *mut c_int, info: *mut c_int);
pub fn cgerfsx_(trans: *const c_char, equed: *const c_char, n: *const c_int,
                nrhs: *const c_int, a: *const complex_float, lda: *const c_int,
                af: *const complex_float, ldaf: *const c_int, ipiv: *const c_int,
                r: *const c_float, c: *const c_float, b: *const complex_float,
                ldb: *const c_int, x: *mut complex_float, ldx: *const c_int,
                rcond: *mut c_float, berr: *mut c_float, n_err_bnds: *const c_int,
                err_bnds_norm: *mut c_float, err_bnds_comp: *mut c_float,
                nparams: *const c_int, params: *mut c_float, work: *mut complex_float,
                rwork: *mut c_float, info: *mut c_int);
pub fn zgerfsx_(trans: *const c_char, equed: *const c_char, n: *const c_int,
                nrhs: *const c_int, a: *const complex_double, lda: *const c_int,
                af: *const complex_double, ldaf: *const c_int, ipiv: *const c_int,
                r: *const c_double, c: *const c_double, b: *const complex_double,
                ldb: *const c_int, x: *mut complex_double, ldx: *const c_int,
                rcond: *mut c_double, berr: *mut c_double, n_err_bnds: *const c_int,
                err_bnds_norm: *mut c_double, err_bnds_comp: *mut c_double,
                nparams: *const c_int, params: *mut c_double, work: *mut complex_double,
                rwork: *mut c_double, info: *mut c_int);

pub fn sgbrfs_(trans: *const c_char, n: *const c_int, kl: *const c_int, ku: *const c_int,
               nrhs: *const c_int, ab: *const c_float, ldab: *const c_int, afb: *const c_float,
               ldafb: *const c_int, ipiv: *const c_int, b: *const c_float, ldb: *const c_int,
               x: *mut c_float, ldx: *const c_int, ferr: *mut c_float, berr: *mut c_float,
               work: *mut c_float, iwork: *mut c_int, info: *mut c_int);
pub fn dgbrfs_(trans: *const c_char, n: *const c_int, kl: *const c_int, ku: *const c_int,
               nrhs: *const c_int, ab: *const c_double, ldab: *const c_int,
               afb: *const c_double, ldafb: *const c_int, ipiv: *const c_int,
               b: *const c_double, ldb: *const c_int, x: *mut c_double, ldx: *const c_int,
               ferr: *mut c_double, berr: *mut c_double, work: *mut c_double,
               iwork: *mut c_int, info: *mut c_int);
pub fn cgbrfs_(trans: *const c_char, n: *const c_int, kl: *const c_int, ku: *const c_int,
               nrhs: *const c_int, ab: *const complex_float, ldab: *const c_int,
               afb: *const complex_float, ldafb: *const c_int, ipiv: *const c_int,
               b: *const complex_float, ldb: *const c_int, x: *mut complex_float,
               ldx: *const c_int, ferr: *mut c_float, berr: *mut c_float,
               work: *mut complex_float, rwork: *mut c_float, info: *mut c_int);
pub fn zgbrfs_(trans: *const c_char, n: *const c_int, kl: *const c_int, ku: *const c_int,
               nrhs: *const c_int, ab: *const complex_double, ldab: *const c_int,
               afb: *const complex_double, ldafb: *const c_int, ipiv: *const c_int,
               b: *const complex_double, ldb: *const c_int, x: *mut complex_double,
               ldx: *const c_int, ferr: *mut c_double, berr: *mut c_double,
               work: *mut complex_double, rwork: *mut c_double, info: *mut c_int);

pub fn sgbrfsx_(trans: *const c_char, equed: *const c_char, n: *const c_int, kl: *const c_int,
                ku: *const c_int, nrhs: *const c_int, ab: *const c_float, ldab: *const c_int,
                afb: *const c_float, ldafb: *const c_int, ipiv: *const c_int,
                r: *const c_float, c: *const c_float, b: *const c_float, ldb: *const c_int,
                x: *mut c_float, ldx: *const c_int, rcond: *mut c_float, berr: *mut c_float,
                n_err_bnds: *const c_int, err_bnds_norm: *mut c_float,
                err_bnds_comp: *mut c_float, nparams: *const c_int, params: *mut c_float,
                work: *mut c_float, iwork: *mut c_int, info: *mut c_int);
pub fn dgbrfsx_(trans: *const c_char, equed: *const c_char, n: *const c_int, kl: *const c_int,
                ku: *const c_int, nrhs: *const c_int, ab: *const c_double, ldab: *const c_int,
                afb: *const c_double, ldafb: *const c_int, ipiv: *const c_int,
                r: *const c_double, c: *const c_double, b: *const c_double, ldb: *const c_int,
                x: *mut c_double, ldx: *const c_int, rcond: *mut c_double, berr: *mut c_double,
                n_err_bnds: *const c_int, err_bnds_norm: *mut c_double,
                err_bnds_comp: *mut c_double, nparams: *const c_int, params: *mut c_double,
                work: *mut c_double, iwork: *mut c_int, info: *mut c_int);
pub fn cgbrfsx_(trans: *const c_char, equed: *const c_char, n: *const c_int, kl: *const c_int,
                ku: *const c_int, nrhs: *const c_int, ab: *const complex_float,
                ldab: *const c_int, afb: *const complex_float, ldafb: *const c_int,
                ipiv: *const c_int, r: *const c_float, c: *const c_float,
                b: *const complex_float, ldb: *const c_int, x: *mut complex_float,
                ldx: *const c_int, rcond: *mut c_float, berr: *mut c_float,
                n_err_bnds: *const c_int, err_bnds_norm: *mut c_float,
                err_bnds_comp: *mut c_float, nparams: *const c_int, params: *mut c_float,
                work: *mut complex_float, rwork: *mut c_float, info: *mut c_int);
pub fn zgbrfsx_(trans: *const c_char, equed: *const c_char, n: *const c_int, kl: *const c_int,
                ku: *const c_int, nrhs: *const c_int, ab: *const complex_double,
                ldab: *const c_int, afb: *const complex_double, ldafb: *const c_int,
                ipiv: *const c_int, r: *const c_double, c: *const c_double,
                b: *const complex_double, ldb: *const c_int, x: *mut complex_double,
                ldx: *const c_int, rcond: *mut c_double, berr: *mut c_double,
                n_err_bnds: *const c_int, err_bnds_norm: *mut c_double,
                err_bnds_comp: *mut c_double, nparams: *const c_int, params: *mut c_double,
                work: *mut complex_double, rwork: *mut c_double, info: *mut c_int);

pub fn sgtrfs_(trans: *const c_char, n: *const c_int, nrhs: *const c_int, dl: *const c_float,
               d: *const c_float, du: *const c_float, dlf: *const c_float, df: *const c_float,
               duf: *const c_float, du2: *const c_float, ipiv: *const c_int, b: *const c_float,
               ldb: *const c_int, x: *mut c_float, ldx: *const c_int, ferr: *mut c_float,
               berr: *mut c_float, work: *mut c_float, iwork: *mut c_int, info: *mut c_int);
pub fn dgtrfs_(trans: *const c_char, n: *const c_int, nrhs: *const c_int, dl: *const c_double,
               d: *const c_double, du: *const c_double, dlf: *const c_double,
               df: *const c_double, duf: *const c_double, du2: *const c_double,
               ipiv: *const c_int, b: *const c_double, ldb: *const c_int, x: *mut c_double,
               ldx: *const c_int, ferr: *mut c_double, berr: *mut c_double,
               work: *mut c_double, iwork: *mut c_int, info: *mut c_int);
pub fn cgtrfs_(trans: *const c_char, n: *const c_int, nrhs: *const c_int,
               dl: *const complex_float, d: *const complex_float, du: *const complex_float,
               dlf: *const complex_float, df: *const complex_float, duf: *const complex_float,
               du2: *const complex_float, ipiv: *const c_int, b: *const complex_float,
               ldb: *const c_int, x: *mut complex_float, ldx: *const c_int, ferr: *mut c_float,
               berr: *mut c_float, work: *mut complex_float, rwork: *mut c_float,
               info: *mut c_int);
pub fn zgtrfs_(trans: *const c_char, n: *const c_int, nrhs: *const c_int,
               dl: *const complex_double, d: *const complex_double, du: *const complex_double,
               dlf: *const complex_double, df: *const complex_double,
               duf: *const complex_double, du2: *const complex_double, ipiv: *const c_int,
               b: *const complex_double, ldb: *const c_int, x: *mut complex_double,
               ldx: *const c_int, ferr: *mut c_double, berr: *mut c_double,
               work: *mut complex_double, rwork: *mut c_double, info: *mut c_int);

pub fn sporfs_(uplo: *const c_char, n: *const c_int, nrhs: *const c_int, a: *const c_float,
               lda: *const c_int, af: *const c_float, ldaf: *const c_int, b: *const c_float,
               ldb: *const c_int, x: *mut c_float, ldx: *const c_int, ferr: *mut c_float,
               berr: *mut c_float, work: *mut c_float, iwork: *mut c_int, info: *mut c_int);
pub fn dporfs_(uplo: *const c_char, n: *const c_int, nrhs: *const c_int, a: *const c_double,
               lda: *const c_int, af: *const c_double, ldaf: *const c_int, b: *const c_double,
               ldb: *const c_int, x: *mut c_double, ldx: *const c_int, ferr: *mut c_double,
               berr: *mut c_double, work: *mut c_double, iwork: *mut c_int, info: *mut c_int);
pub fn cporfs_(uplo: *const c_char, n: *const c_int, nrhs: *const c_int,
               a: *const complex_float, lda: *const c_int, af: *const complex_float,
               ldaf: *const c_int, b: *const complex_float, ldb: *const c_int,
               x: *mut complex_float, ldx: *const c_int, ferr: *mut c_float,
               berr: *mut c_float, work: *mut complex_float, rwork: *mut c_float,
               info: *mut c_int);
pub fn zporfs_(uplo: *const c_char, n: *const c_int, nrhs: *const c_int,
               a: *const complex_double, lda: *const c_int, af: *const complex_double,
               ldaf: *const c_int, b: *const complex_double, ldb: *const c_int,
               x: *mut complex_double, ldx: *const c_int, ferr: *mut c_double,
               berr: *mut c_double, work: *mut complex_double, rwork: *mut c_double,
               info: *mut c_int);

pub fn sporfsx_(uplo: *const c_char, equed: *const c_char, n: *const c_int, nrhs: *const c_int,
                a: *const c_float, lda: *const c_int, af: *const c_float, ldaf: *const c_int,
                s: *const c_float, b: *const c_float, ldb: *const c_int, x: *mut c_float,
                ldx: *const c_int, rcond: *mut c_float, berr: *mut c_float,
                n_err_bnds: *const c_int, err_bnds_norm: *mut c_float,
                err_bnds_comp: *mut c_float, nparams: *const c_int, params: *mut c_float,
                work: *mut c_float, iwork: *mut c_int, info: *mut c_int);
pub fn dporfsx_(uplo: *const c_char, equed: *const c_char, n: *const c_int, nrhs: *const c_int,
                a: *const c_double, lda: *const c_int, af: *const c_double, ldaf: *const c_int,
                s: *const c_double, b: *const c_double, ldb: *const c_int, x: *mut c_double,
                ldx: *const c_int, rcond: *mut c_double, berr: *mut c_double,
                n_err_bnds: *const c_int, err_bnds_norm: *mut c_double,
                err_bnds_comp: *mut c_double, nparams: *const c_int, params: *mut c_double,
                work: *mut c_double, iwork: *mut c_int, info: *mut c_int);
pub fn cporfsx_(uplo: *const c_char, equed: *const c_char, n: *const c_int, nrhs: *const c_int,
                a: *const complex_float, lda: *const c_int, af: *const complex_float,
                ldaf: *const c_int, s: *const c_float, b: *const complex_float,
                ldb: *const c_int, x: *mut complex_float, ldx: *const c_int,
                rcond: *mut c_float, berr: *mut c_float, n_err_bnds: *const c_int,
                err_bnds_norm: *mut c_float, err_bnds_comp: *mut c_float,
                nparams: *const c_int, params: *mut c_float, work: *mut complex_float,
                rwork: *mut c_float, info: *mut c_int);
pub fn zporfsx_(uplo: *const c_char, equed: *const c_char, n: *const c_int, nrhs: *const c_int,
                a: *const complex_double, lda: *const c_int, af: *const complex_double,
                ldaf: *const c_int, s: *const c_double, b: *const complex_double,
                ldb: *const c_int, x: *mut complex_double, ldx: *const c_int,
                rcond: *mut c_double, berr: *mut c_double, n_err_bnds: *const c_int,
                err_bnds_norm: *mut c_double, err_bnds_comp: *mut c_double,
                nparams: *const c_int, params: *mut c_double, work: *mut complex_double,
                rwork: *mut c_double, info: *mut c_int);

pub fn spprfs_(uplo: *const c_char, n: *const c_int, nrhs: *const c_int, ap: *const c_float,
               afp: *const c_float, b: *const c_float, ldb: *const c_int, x: *mut c_float,
               ldx: *const c_int, ferr: *mut c_float, berr: *mut c_float, work: *mut c_float,
               iwork: *mut c_int, info: *mut c_int);
pub fn dpprfs_(uplo: *const c_char, n: *const c_int, nrhs: *const c_int, ap: *const c_double,
               afp: *const c_double, b: *const c_double, ldb: *const c_int, x: *mut c_double,
               ldx: *const c_int, ferr: *mut c_double, berr: *mut c_double,
               work: *mut c_double, iwork: *mut c_int, info: *mut c_int);
pub fn cpprfs_(uplo: *const c_char, n: *const c_int, nrhs: *const c_int,
               ap: *const complex_float, afp: *const complex_float, b: *const complex_float,
               ldb: *const c_int, x: *mut complex_float, ldx: *const c_int, ferr: *mut c_float,
               berr: *mut c_float, work: *mut complex_float, rwork: *mut c_float,
               info: *mut c_int);
pub fn zpprfs_(uplo: *const c_char, n: *const c_int, nrhs: *const c_int,
               ap: *const complex_double, afp: *const complex_double, b: *const complex_double,
               ldb: *const c_int, x: *mut complex_double, ldx: *const c_int,
               ferr: *mut c_double, berr: *mut c_double, work: *mut complex_double,
               rwork: *mut c_double, info: *mut c_int);

pub fn spbrfs_(uplo: *const c_char, n: *const c_int, kd: *const c_int, nrhs: *const c_int,
               ab: *const c_float, ldab: *const c_int, afb: *const c_float,
               ldafb: *const c_int, b: *const c_float, ldb: *const c_int, x: *mut c_float,
               ldx: *const c_int, ferr: *mut c_float, berr: *mut c_float, work: *mut c_float,
               iwork: *mut c_int, info: *mut c_int);
pub fn dpbrfs_(uplo: *const c_char, n: *const c_int, kd: *const c_int, nrhs: *const c_int,
               ab: *const c_double, ldab: *const c_int, afb: *const c_double,
               ldafb: *const c_int, b: *const c_double, ldb: *const c_int, x: *mut c_double,
               ldx: *const c_int, ferr: *mut c_double, berr: *mut c_double,
               work: *mut c_double, iwork: *mut c_int, info: *mut c_int);
pub fn cpbrfs_(uplo: *const c_char, n: *const c_int, kd: *const c_int, nrhs: *const c_int,
               ab: *const complex_float, ldab: *const c_int, afb: *const complex_float,
               ldafb: *const c_int, b: *const complex_float, ldb: *const c_int,
               x: *mut complex_float, ldx: *const c_int, ferr: *mut c_float,
               berr: *mut c_float, work: *mut complex_float, rwork: *mut c_float,
               info: *mut c_int);
pub fn zpbrfs_(uplo: *const c_char, n: *const c_int, kd: *const c_int, nrhs: *const c_int,
               ab: *const complex_double, ldab: *const c_int, afb: *const complex_double,
               ldafb: *const c_int, b: *const complex_double, ldb: *const c_int,
               x: *mut complex_double, ldx: *const c_int, ferr: *mut c_double,
               berr: *mut c_double, work: *mut complex_double, rwork: *mut c_double,
               info: *mut c_int);

pub fn sptrfs_(n: *const c_int, nrhs: *const c_int, d: *const c_float, e: *const c_float,
               df: *const c_float, ef: *const c_float, b: *const c_float, ldb: *const c_int,
               x: *mut c_float, ldx: *const c_int, ferr: *mut c_float, berr: *mut c_float,
               work: *mut c_float, info: *mut c_int);
pub fn dptrfs_(n: *const c_int, nrhs: *const c_int, d: *const c_double, e: *const c_double,
               df: *const c_double, ef: *const c_double, b: *const c_double, ldb: *const c_int,
               x: *mut c_double, ldx: *const c_int, ferr: *mut c_double, berr: *mut c_double,
               work: *mut c_double, info: *mut c_int);
pub fn cptrfs_(uplo: *const c_char, n: *const c_int, nrhs: *const c_int, d: *const c_float,
               e: *const complex_float, df: *const c_float, ef: *const complex_float,
               b: *const complex_float, ldb: *const c_int, x: *mut complex_float,
               ldx: *const c_int, ferr: *mut c_float, berr: *mut c_float,
               work: *mut complex_float, rwork: *mut c_float, info: *mut c_int);
pub fn zptrfs_(uplo: *const c_char, n: *const c_int, nrhs: *const c_int, d: *const c_double,
               e: *const complex_double, df: *const c_double, ef: *const complex_double,
               b: *const complex_double, ldb: *const c_int, x: *mut complex_double,
               ldx: *const c_int, ferr: *mut c_double, berr: *mut c_double,
               work: *mut complex_double, rwork: *mut c_double, info: *mut c_int);

pub fn ssyrfs_(uplo: *const c_char, n: *const c_int, nrhs: *const c_int, a: *const c_float,
               lda: *const c_int, af: *const c_float, ldaf: *const c_int, ipiv: *const c_int,
               b: *const c_float, ldb: *const c_int, x: *mut c_float, ldx: *const c_int,
               ferr: *mut c_float, berr: *mut c_float, work: *mut c_float, iwork: *mut c_int,
               info: *mut c_int);
pub fn dsyrfs_(uplo: *const c_char, n: *const c_int, nrhs: *const c_int, a: *const c_double,
               lda: *const c_int, af: *const c_double, ldaf: *const c_int, ipiv: *const c_int,
               b: *const c_double, ldb: *const c_int, x: *mut c_double, ldx: *const c_int,
               ferr: *mut c_double, berr: *mut c_double, work: *mut c_double,
               iwork: *mut c_int, info: *mut c_int);
pub fn csyrfs_(uplo: *const c_char, n: *const c_int, nrhs: *const c_int,
               a: *const complex_float, lda: *const c_int, af: *const complex_float,
               ldaf: *const c_int, ipiv: *const c_int, b: *const complex_float,
               ldb: *const c_int, x: *mut complex_float, ldx: *const c_int, ferr: *mut c_float,
               berr: *mut c_float, work: *mut complex_float, rwork: *mut c_float,
               info: *mut c_int);
pub fn zsyrfs_(uplo: *const c_char, n: *const c_int, nrhs: *const c_int,
               a: *const complex_double, lda: *const c_int, af: *const complex_double,
               ldaf: *const c_int, ipiv: *const c_int, b: *const complex_double,
               ldb: *const c_int, x: *mut complex_double, ldx: *const c_int,
               ferr: *mut c_double, berr: *mut c_double, work: *mut complex_double,
               rwork: *mut c_double, info: *mut c_int);

pub fn ssyrfsx_(uplo: *const c_char, equed: *const c_char, n: *const c_int, nrhs: *const c_int,
                a: *const c_float, lda: *const c_int, af: *const c_float, ldaf: *const c_int,
                ipiv: *const c_int, s: *const c_float, b: *const c_float, ldb: *const c_int,
                x: *mut c_float, ldx: *const c_int, rcond: *mut c_float, berr: *mut c_float,
                n_err_bnds: *const c_int, err_bnds_norm: *mut c_float,
                err_bnds_comp: *mut c_float, nparams: *const c_int, params: *mut c_float,
                work: *mut c_float, iwork: *mut c_int, info: *mut c_int);
pub fn dsyrfsx_(uplo: *const c_char, equed: *const c_char, n: *const c_int, nrhs: *const c_int,
                a: *const c_double, lda: *const c_int, af: *const c_double, ldaf: *const c_int,
                ipiv: *const c_int, s: *const c_double, b: *const c_double, ldb: *const c_int,
                x: *mut c_double, ldx: *const c_int, rcond: *mut c_double, berr: *mut c_double,
                n_err_bnds: *const c_int, err_bnds_norm: *mut c_double,
                err_bnds_comp: *mut c_double, nparams: *const c_int, params: *mut c_double,
                work: *mut c_double, iwork: *mut c_int, info: *mut c_int);
pub fn csyrfsx_(uplo: *const c_char, equed: *const c_char, n: *const c_int, nrhs: *const c_int,
                a: *const complex_float, lda: *const c_int, af: *const complex_float,
                ldaf: *const c_int, ipiv: *const c_int, s: *const c_float,
                b: *const complex_float, ldb: *const c_int, x: *mut complex_float,
                ldx: *const c_int, rcond: *mut c_float, berr: *mut c_float,
                n_err_bnds: *const c_int, err_bnds_norm: *mut c_float,
                err_bnds_comp: *mut c_float, nparams: *const c_int, params: *mut c_float,
                work: *mut complex_float, rwork: *mut c_float, info: *mut c_int);
pub fn zsyrfsx_(uplo: *const c_char, equed: *const c_char, n: *const c_int, nrhs: *const c_int,
                a: *const complex_double, lda: *const c_int, af: *const complex_double,
                ldaf: *const c_int, ipiv: *const c_int, s: *const c_double,
                b: *const complex_double, ldb: *const c_int, x: *mut complex_double,
                ldx: *const c_int, rcond: *mut c_double, berr: *mut c_double,
                n_err_bnds: *const c_int, err_bnds_norm: *mut c_double,
                err_bnds_comp: *mut c_double, nparams: *const c_int, params: *mut c_double,
                work: *mut complex_double, rwork: *mut c_double, info: *mut c_int);

pub fn cherfs_(uplo: *const c_char, n: *const c_int, nrhs: *const c_int,
               a: *const complex_float, lda: *const c_int, af: *const complex_float,
               ldaf: *const c_int, ipiv: *const c_int, b: *const complex_float,
               ldb: *const c_int, x: *mut complex_float, ldx: *const c_int, ferr: *mut c_float,
               berr: *mut c_float, work: *mut complex_float, rwork: *mut c_float,
               info: *mut c_int);
pub fn zherfs_(uplo: *const c_char, n: *const c_int, nrhs: *const c_int,
               a: *const complex_double, lda: *const c_int, af: *const complex_double,
               ldaf: *const c_int, ipiv: *const c_int, b: *const complex_double,
               ldb: *const c_int, x: *mut complex_double, ldx: *const c_int,
               ferr: *mut c_double, berr: *mut c_double, work: *mut complex_double,
               rwork: *mut c_double, info: *mut c_int);

pub fn cherfsx_(uplo: *const c_char, equed: *const c_char, n: *const c_int, nrhs: *const c_int,
                a: *const complex_float, lda: *const c_int, af: *const complex_float,
                ldaf: *const c_int, ipiv: *const c_int, s: *const c_float,
                b: *const complex_float, ldb: *const c_int, x: *mut complex_float,
                ldx: *const c_int, rcond: *mut c_float, berr: *mut c_float,
                n_err_bnds: *const c_int, err_bnds_norm: *mut c_float,
                err_bnds_comp: *mut c_float, nparams: *const c_int, params: *mut c_float,
                work: *mut complex_float, rwork: *mut c_float, info: *mut c_int);
pub fn zherfsx_(uplo: *const c_char, equed: *const c_char, n: *const c_int, nrhs: *const c_int,
                a: *const complex_double, lda: *const c_int, af: *const complex_double,
                ldaf: *const c_int, ipiv: *const c_int, s: *const c_double,
                b: *const complex_double, ldb: *const c_int, x: *mut complex_double,
                ldx: *const c_int, rcond: *mut c_double, berr: *mut c_double,
                n_err_bnds: *const c_int, err_bnds_norm: *mut c_double,
                err_bnds_comp: *mut c_double, nparams: *const c_int, params: *mut c_double,
                work: *mut complex_double, rwork: *mut c_double, info: *mut c_int);

pub fn ssprfs_(uplo: *const c_char, n: *const c_int, nrhs: *const c_int, ap: *const c_float,
               afp: *const c_float, ipiv: *const c_int, b: *const c_float, ldb: *const c_int,
               x: *mut c_float, ldx: *const c_int, ferr: *mut c_float, berr: *mut c_float,
               work: *mut c_float, iwork: *mut c_int, info: *mut c_int);
pub fn dsprfs_(uplo: *const c_char, n: *const c_int, nrhs: *const c_int, ap: *const c_double,
               afp: *const c_double, ipiv: *const c_int, b: *const c_double, ldb: *const c_int,
               x: *mut c_double, ldx: *const c_int, ferr: *mut c_double, berr: *mut c_double,
               work: *mut c_double, iwork: *mut c_int, info: *mut c_int);
pub fn csprfs_(uplo: *const c_char, n: *const c_int, nrhs: *const c_int,
               ap: *const complex_float, afp: *const complex_float, ipiv: *const c_int,
               b: *const complex_float, ldb: *const c_int, x: *mut complex_float,
               ldx: *const c_int, ferr: *mut c_float, berr: *mut c_float,
               work: *mut complex_float, rwork: *mut c_float, info: *mut c_int);
pub fn zsprfs_(uplo: *const c_char, n: *const c_int, nrhs: *const c_int,
               ap: *const complex_double, afp: *const complex_double, ipiv: *const c_int,
               b: *const complex_double, ldb: *const c_int, x: *mut complex_double,
               ldx: *const c_int, ferr: *mut c_double, berr: *mut c_double,
               work: *mut complex_double, rwork: *mut c_double, info: *mut c_int);

pub fn chprfs_(uplo: *const c_char, n: *const c_int, nrhs: *const c_int,
               ap: *const complex_float, afp: *const complex_float, ipiv: *const c_int,
               b: *const complex_float, ldb: *const c_int, x: *mut complex_float,
               ldx: *const c_int, ferr: *mut c_float, berr: *mut c_float,
               work: *mut complex_float, rwork: *mut c_float, info: *mut c_int);
pub fn zhprfs_(uplo: *const c_char, n: *const c_int, nrhs: *const c_int,
               ap: *const complex_double, afp: *const complex_double, ipiv: *const c_int,
               b: *const complex_double, ldb: *const c_int, x: *mut complex_double,
               ldx: *const c_int, ferr: *mut c_double, berr: *mut c_double,
               work: *mut complex_double, rwork: *mut c_double, info: *mut c_int);

pub fn strrfs_(uplo: *const c_char, trans: *const c_char, diag: *const c_char, n: *const c_int,
               nrhs: *const c_int, a: *const c_float, lda: *const c_int, b: *const c_float,
               ldb: *const c_int, x: *const c_float, ldx: *const c_int, ferr: *mut c_float,
               berr: *mut c_float, work: *mut c_float, iwork: *mut c_int, info: *mut c_int);
pub fn dtrrfs_(uplo: *const c_char, trans: *const c_char, diag: *const c_char, n: *const c_int,
               nrhs: *const c_int, a: *const c_double, lda: *const c_int, b: *const c_double,
               ldb: *const c_int, x: *const c_double, ldx: *const c_int, ferr: *mut c_double,
               berr: *mut c_double, work: *mut c_double, iwork: *mut c_int, info: *mut c_int);
pub fn ctrrfs_(uplo: *const c_char, trans: *const c_char, diag: *const c_char, n: *const c_int,
               nrhs: *const c_int, a: *const complex_float, lda: *const c_int,
               b: *const complex_float, ldb: *const c_int, x: *const complex_float,
               ldx: *const c_int, ferr: *mut c_float, berr: *mut c_float,
               work: *mut complex_float, rwork: *mut c_float, info: *mut c_int);
pub fn ztrrfs_(uplo: *const c_char, trans: *const c_char, diag: *const c_char, n: *const c_int,
               nrhs: *const c_int, a: *const complex_double, lda: *const c_int,
               b: *const complex_double, ldb: *const c_int, x: *const complex_double,
               ldx: *const c_int, ferr: *mut c_double, berr: *mut c_double,
               work: *mut complex_double, rwork: *mut c_double, info: *mut c_int);

pub fn stprfs_(uplo: *const c_char, trans: *const c_char, diag: *const c_char, n: *const c_int,
               nrhs: *const c_int, ap: *const c_float, b: *const c_float, ldb: *const c_int,
               x: *const c_float, ldx: *const c_int, ferr: *mut c_float, berr: *mut c_float,
               work: *mut c_float, iwork: *mut c_int, info: *mut c_int);
pub fn dtprfs_(uplo: *const c_char, trans: *const c_char, diag: *const c_char, n: *const c_int,
               nrhs: *const c_int, ap: *const c_double, b: *const c_double, ldb: *const c_int,
               x: *const c_double, ldx: *const c_int, ferr: *mut c_double, berr: *mut c_double,
               work: *mut c_double, iwork: *mut c_int, info: *mut c_int);
pub fn ctprfs_(uplo: *const c_char, trans: *const c_char, diag: *const c_char, n: *const c_int,
               nrhs: *const c_int, ap: *const complex_float, b: *const complex_float,
               ldb: *const c_int, x: *const complex_float, ldx: *const c_int,
               ferr: *mut c_float, berr: *mut c_float, work: *mut complex_float,
               rwork: *mut c_float, info: *mut c_int);
pub fn ztprfs_(uplo: *const c_char, trans: *const c_char, diag: *const c_char, n: *const c_int,
               nrhs: *const c_int, ap: *const complex_double, b: *const complex_double,
               ldb: *const c_int, x: *const complex_double, ldx: *const c_int,
               ferr: *mut c_double, berr: *mut c_double, work: *mut complex_double,
               rwork: *mut c_double, info: *mut c_int);

pub fn stbrfs_(uplo: *const c_char, trans: *const c_char, diag: *const c_char, n: *const c_int,
               kd: *const c_int, nrhs: *const c_int, ab: *const c_float, ldab: *const c_int,
               b: *const c_float, ldb: *const c_int, x: *const c_float, ldx: *const c_int,
               ferr: *mut c_float, berr: *mut c_float, work: *mut c_float, iwork: *mut c_int,
               info: *mut c_int);
pub fn dtbrfs_(uplo: *const c_char, trans: *const c_char, diag: *const c_char, n: *const c_int,
               kd: *const c_int, nrhs: *const c_int, ab: *const c_double, ldab: *const c_int,
               b: *const c_double, ldb: *const c_int, x: *const c_double, ldx: *const c_int,
               ferr: *mut c_double, berr: *mut c_double, work: *mut c_double,
               iwork: *mut c_int, info: *mut c_int);
pub fn ctbrfs_(uplo: *const c_char, trans: *const c_char, diag: *const c_char, n: *const c_int,
               kd: *const c_int, nrhs: *const c_int, ab: *const complex_float,
               ldab: *const c_int, b: *const complex_float, ldb: *const c_int,
               x: *const complex_float, ldx: *const c_int, ferr: *mut c_float,
               berr: *mut c_float, work: *mut complex_float, rwork: *mut c_float,
               info: *mut c_int);
pub fn ztbrfs_(uplo: *const c_char, trans: *const c_char, diag: *const c_char, n: *const c_int,
               kd: *const c_int, nrhs: *const c_int, ab: *const complex_double,
               ldab: *const c_int, b: *const complex_double, ldb: *const c_int,
               x: *const complex_double, ldx: *const c_int, ferr: *mut c_double,
               berr: *mut c_double, work: *mut complex_double, rwork: *mut c_double,
               info: *mut c_int);

pub fn sgetri_(n: *const c_int, a: *mut c_float, lda: *const c_int, ipiv: *const c_int,
               work: *mut c_float, lwork: *const c_int, info: *mut c_int);
pub fn dgetri_(n: *const c_int, a: *mut c_double, lda: *const c_int, ipiv: *const c_int,
               work: *mut c_double, lwork: *const c_int, info: *mut c_int);
pub fn cgetri_(n: *const c_int, a: *mut complex_float, lda: *const c_int, ipiv: *const c_int,
               work: *mut complex_float, lwork: *const c_int, info: *mut c_int);
pub fn zgetri_(n: *const c_int, a: *mut complex_double, lda: *const c_int, ipiv: *const c_int,
               work: *mut complex_double, lwork: *const c_int, info: *mut c_int);

pub fn spotri_(uplo: *const c_char, n: *const c_int, a: *mut c_float, lda: *const c_int,
               info: *mut c_int);
pub fn dpotri_(uplo: *const c_char, n: *const c_int, a: *mut c_double, lda: *const c_int,
               info: *mut c_int);
pub fn cpotri_(uplo: *const c_char, n: *const c_int, a: *mut complex_float, lda: *const c_int,
               info: *mut c_int);
pub fn zpotri_(uplo: *const c_char, n: *const c_int, a: *mut complex_double, lda: *const c_int,
               info: *mut c_int);

pub fn spftri_(transr: *const c_char, uplo: *const c_char, n: *const c_int, a: *mut c_float,
               info: *mut c_int);
pub fn dpftri_(transr: *const c_char, uplo: *const c_char, n: *const c_int, a: *mut c_double,
               info: *mut c_int);
pub fn cpftri_(transr: *const c_char, uplo: *const c_char, n: *const c_int,
               a: *mut complex_float, info: *mut c_int);
pub fn zpftri_(transr: *const c_char, uplo: *const c_char, n: *const c_int,
               a: *mut complex_double, info: *mut c_int);

pub fn spptri_(uplo: *const c_char, n: *const c_int, ap: *mut c_float, info: *mut c_int);
pub fn dpptri_(uplo: *const c_char, n: *const c_int, ap: *mut c_double, info: *mut c_int);
pub fn cpptri_(uplo: *const c_char, n: *const c_int, ap: *mut complex_float, info: *mut c_int);
pub fn zpptri_(uplo: *const c_char, n: *const c_int, ap: *mut complex_double,
               info: *mut c_int);

pub fn ssytri_(uplo: *const c_char, n: *const c_int, a: *mut c_float, lda: *const c_int,
               ipiv: *const c_int, work: *mut c_float, info: *mut c_int);
pub fn dsytri_(uplo: *const c_char, n: *const c_int, a: *mut c_double, lda: *const c_int,
               ipiv: *const c_int, work: *mut c_double, info: *mut c_int);
pub fn csytri_(uplo: *const c_char, n: *const c_int, a: *mut complex_float, lda: *const c_int,
               ipiv: *const c_int, work: *mut complex_float, info: *mut c_int);
pub fn zsytri_(uplo: *const c_char, n: *const c_int, a: *mut complex_double, lda: *const c_int,
               ipiv: *const c_int, work: *mut complex_double, info: *mut c_int);

pub fn chetri_(uplo: *const c_char, n: *const c_int, a: *mut complex_float, lda: *const c_int,
               ipiv: *const c_int, work: *mut complex_float, info: *mut c_int);
pub fn zhetri_(uplo: *const c_char, n: *const c_int, a: *mut complex_double, lda: *const c_int,
               ipiv: *const c_int, work: *mut complex_double, info: *mut c_int);

pub fn ssptri_(uplo: *const c_char, n: *const c_int, ap: *mut c_float, ipiv: *const c_int,
               work: *mut c_float, info: *mut c_int);
pub fn dsptri_(uplo: *const c_char, n: *const c_int, ap: *mut c_double, ipiv: *const c_int,
               work: *mut c_double, info: *mut c_int);
pub fn csptri_(uplo: *const c_char, n: *const c_int, ap: *mut complex_float,
               ipiv: *const c_int, work: *mut complex_float, info: *mut c_int);
pub fn zsptri_(uplo: *const c_char, n: *const c_int, ap: *mut complex_double,
               ipiv: *const c_int, work: *mut complex_double, info: *mut c_int);

pub fn chptri_(uplo: *const c_char, n: *const c_int, ap: *mut complex_float,
               ipiv: *const c_int, work: *mut complex_float, info: *mut c_int);
pub fn zhptri_(uplo: *const c_char, n: *const c_int, ap: *mut complex_double,
               ipiv: *const c_int, work: *mut complex_double, info: *mut c_int);

pub fn strtri_(uplo: *const c_char, diag: *const c_char, n: *const c_int, a: *mut c_float,
               lda: *const c_int, info: *mut c_int);
pub fn dtrtri_(uplo: *const c_char, diag: *const c_char, n: *const c_int, a: *mut c_double,
               lda: *const c_int, info: *mut c_int);
pub fn ctrtri_(uplo: *const c_char, diag: *const c_char, n: *const c_int,
               a: *mut complex_float, lda: *const c_int, info: *mut c_int);
pub fn ztrtri_(uplo: *const c_char, diag: *const c_char, n: *const c_int,
               a: *mut complex_double, lda: *const c_int, info: *mut c_int);

pub fn stftri_(transr: *const c_char, uplo: *const c_char, diag: *const c_char,
               n: *const c_int, a: *mut c_float, info: *mut c_int);
pub fn dtftri_(transr: *const c_char, uplo: *const c_char, diag: *const c_char,
               n: *const c_int, a: *mut c_double, info: *mut c_int);
pub fn ctftri_(transr: *const c_char, uplo: *const c_char, diag: *const c_char,
               n: *const c_int, a: *mut complex_float, info: *mut c_int);
pub fn ztftri_(transr: *const c_char, uplo: *const c_char, diag: *const c_char,
               n: *const c_int, a: *mut complex_double, info: *mut c_int);

pub fn stptri_(uplo: *const c_char, diag: *const c_char, n: *const c_int, ap: *mut c_float,
               info: *mut c_int);
pub fn dtptri_(uplo: *const c_char, diag: *const c_char, n: *const c_int, ap: *mut c_double,
               info: *mut c_int);
pub fn ctptri_(uplo: *const c_char, diag: *const c_char, n: *const c_int,
               ap: *mut complex_float, info: *mut c_int);
pub fn ztptri_(uplo: *const c_char, diag: *const c_char, n: *const c_int,
               ap: *mut complex_double, info: *mut c_int);

pub fn sgeequ_(m: *const c_int, n: *const c_int, a: *const c_float, lda: *const c_int,
               r: *mut c_float, c: *mut c_float, rowcnd: *mut c_float, colcnd: *mut c_float,
               amax: *mut c_float, info: *mut c_int);
pub fn dgeequ_(m: *const c_int, n: *const c_int, a: *const c_double, lda: *const c_int,
               r: *mut c_double, c: *mut c_double, rowcnd: *mut c_double,
               colcnd: *mut c_double, amax: *mut c_double, info: *mut c_int);
pub fn cgeequ_(m: *const c_int, n: *const c_int, a: *const complex_float, lda: *const c_int,
               r: *mut c_float, c: *mut c_float, rowcnd: *mut c_float, colcnd: *mut c_float,
               amax: *mut c_float, info: *mut c_int);
pub fn zgeequ_(m: *const c_int, n: *const c_int, a: *const complex_double, lda: *const c_int,
               r: *mut c_double, c: *mut c_double, rowcnd: *mut c_double,
               colcnd: *mut c_double, amax: *mut c_double, info: *mut c_int);

pub fn sgeequb_(m: *const c_int, n: *const c_int, a: *const c_float, lda: *const c_int,
                r: *mut c_float, c: *mut c_float, rowcnd: *mut c_float, colcnd: *mut c_float,
                amax: *mut c_float, info: *mut c_int);
pub fn dgeequb_(m: *const c_int, n: *const c_int, a: *const c_double, lda: *const c_int,
                r: *mut c_double, c: *mut c_double, rowcnd: *mut c_double,
                colcnd: *mut c_double, amax: *mut c_double, info: *mut c_int);
pub fn cgeequb_(m: *const c_int, n: *const c_int, a: *const complex_float, lda: *const c_int,
                r: *mut c_float, c: *mut c_float, rowcnd: *mut c_float, colcnd: *mut c_float,
                amax: *mut c_float, info: *mut c_int);
pub fn zgeequb_(m: *const c_int, n: *const c_int, a: *const complex_double, lda: *const c_int,
                r: *mut c_double, c: *mut c_double, rowcnd: *mut c_double,
                colcnd: *mut c_double, amax: *mut c_double, info: *mut c_int);

pub fn sgbequ_(m: *const c_int, n: *const c_int, kl: *const c_int, ku: *const c_int,
               ab: *const c_float, ldab: *const c_int, r: *mut c_float, c: *mut c_float,
               rowcnd: *mut c_float, colcnd: *mut c_float, amax: *mut c_float,
               info: *mut c_int);
pub fn dgbequ_(m: *const c_int, n: *const c_int, kl: *const c_int, ku: *const c_int,
               ab: *const c_double, ldab: *const c_int, r: *mut c_double, c: *mut c_double,
               rowcnd: *mut c_double, colcnd: *mut c_double, amax: *mut c_double,
               info: *mut c_int);
pub fn cgbequ_(m: *const c_int, n: *const c_int, kl: *const c_int, ku: *const c_int,
               ab: *const complex_float, ldab: *const c_int, r: *mut c_float, c: *mut c_float,
               rowcnd: *mut c_float, colcnd: *mut c_float, amax: *mut c_float,
               info: *mut c_int);
pub fn zgbequ_(m: *const c_int, n: *const c_int, kl: *const c_int, ku: *const c_int,
               ab: *const complex_double, ldab: *const c_int, r: *mut c_double,
               c: *mut c_double, rowcnd: *mut c_double, colcnd: *mut c_double,
               amax: *mut c_double, info: *mut c_int);

pub fn sgbequb_(m: *const c_int, n: *const c_int, kl: *const c_int, ku: *const c_int,
                ab: *const c_float, ldab: *const c_int, r: *mut c_float, c: *mut c_float,
                rowcnd: *mut c_float, colcnd: *mut c_float, amax: *mut c_float,
                info: *mut c_int);
pub fn dgbequb_(m: *const c_int, n: *const c_int, kl: *const c_int, ku: *const c_int,
                ab: *const c_double, ldab: *const c_int, r: *mut c_double, c: *mut c_double,
                rowcnd: *mut c_double, colcnd: *mut c_double, amax: *mut c_double,
                info: *mut c_int);
pub fn cgbequb_(m: *const c_int, n: *const c_int, kl: *const c_int, ku: *const c_int,
                ab: *const complex_float, ldab: *const c_int, r: *mut c_float, c: *mut c_float,
                rowcnd: *mut c_float, colcnd: *mut c_float, amax: *mut c_float,
                info: *mut c_int);
pub fn zgbequb_(m: *const c_int, n: *const c_int, kl: *const c_int, ku: *const c_int,
                ab: *const complex_double, ldab: *const c_int, r: *mut c_double,
                c: *mut c_double, rowcnd: *mut c_double, colcnd: *mut c_double,
                amax: *mut c_double, info: *mut c_int);

pub fn spoequ_(n: *const c_int, a: *const c_float, lda: *const c_int, s: *mut c_float,
               scond: *mut c_float, amax: *mut c_float, info: *mut c_int);
pub fn dpoequ_(n: *const c_int, a: *const c_double, lda: *const c_int, s: *mut c_double,
               scond: *mut c_double, amax: *mut c_double, info: *mut c_int);
pub fn cpoequ_(n: *const c_int, a: *const complex_float, lda: *const c_int, s: *mut c_float,
               scond: *mut c_float, amax: *mut c_float, info: *mut c_int);
pub fn zpoequ_(n: *const c_int, a: *const complex_double, lda: *const c_int, s: *mut c_double,
               scond: *mut c_double, amax: *mut c_double, info: *mut c_int);

pub fn spoequb_(n: *const c_int, a: *const c_float, lda: *const c_int, s: *mut c_float,
                scond: *mut c_float, amax: *mut c_float, info: *mut c_int);
pub fn dpoequb_(n: *const c_int, a: *const c_double, lda: *const c_int, s: *mut c_double,
                scond: *mut c_double, amax: *mut c_double, info: *mut c_int);
pub fn cpoequb_(n: *const c_int, a: *const complex_float, lda: *const c_int, s: *mut c_float,
                scond: *mut c_float, amax: *mut c_float, info: *mut c_int);
pub fn zpoequb_(n: *const c_int, a: *const complex_double, lda: *const c_int, s: *mut c_double,
                scond: *mut c_double, amax: *mut c_double, info: *mut c_int);

pub fn sppequ_(uplo: *const c_char, n: *const c_int, ap: *const c_float, s: *mut c_float,
               scond: *mut c_float, amax: *mut c_float, info: *mut c_int);
pub fn dppequ_(uplo: *const c_char, n: *const c_int, ap: *const c_double, s: *mut c_double,
               scond: *mut c_double, amax: *mut c_double, info: *mut c_int);
pub fn cppequ_(uplo: *const c_char, n: *const c_int, ap: *const complex_float, s: *mut c_float,
               scond: *mut c_float, amax: *mut c_float, info: *mut c_int);
pub fn zppequ_(uplo: *const c_char, n: *const c_int, ap: *const complex_double,
               s: *mut c_double, scond: *mut c_double, amax: *mut c_double, info: *mut c_int);

pub fn spbequ_(uplo: *const c_char, n: *const c_int, kd: *const c_int, ab: *const c_float,
               ldab: *const c_int, s: *mut c_float, scond: *mut c_float, amax: *mut c_float,
               info: *mut c_int);
pub fn dpbequ_(uplo: *const c_char, n: *const c_int, kd: *const c_int, ab: *const c_double,
               ldab: *const c_int, s: *mut c_double, scond: *mut c_double, amax: *mut c_double,
               info: *mut c_int);
pub fn cpbequ_(uplo: *const c_char, n: *const c_int, kd: *const c_int,
               ab: *const complex_float, ldab: *const c_int, s: *mut c_float,
               scond: *mut c_float, amax: *mut c_float, info: *mut c_int);
pub fn zpbequ_(uplo: *const c_char, n: *const c_int, kd: *const c_int,
               ab: *const complex_double, ldab: *const c_int, s: *mut c_double,
               scond: *mut c_double, amax: *mut c_double, info: *mut c_int);

pub fn ssyequb_(uplo: *const c_char, n: *const c_int, a: *const c_float, lda: *const c_int,
                s: *mut c_float, scond: *mut c_float, amax: *mut c_float, work: *mut c_float,
                info: *mut c_int);
pub fn dsyequb_(uplo: *const c_char, n: *const c_int, a: *const c_double, lda: *const c_int,
                s: *mut c_double, scond: *mut c_double, amax: *mut c_double,
                work: *mut c_double, info: *mut c_int);
pub fn csyequb_(uplo: *const c_char, n: *const c_int, a: *const complex_float,
                lda: *const c_int, s: *mut c_float, scond: *mut c_float, amax: *mut c_float,
                work: *mut complex_float, info: *mut c_int);
pub fn zsyequb_(uplo: *const c_char, n: *const c_int, a: *const complex_double,
                lda: *const c_int, s: *mut c_double, scond: *mut c_double, amax: *mut c_double,
                work: *mut complex_double, info: *mut c_int);

pub fn cheequb_(uplo: *const c_char, n: *const c_int, a: *const complex_float,
                lda: *const c_int, s: *mut c_float, scond: *mut c_float, amax: *mut c_float,
                work: *mut complex_float, info: *mut c_int);
pub fn zheequb_(uplo: *const c_char, n: *const c_int, a: *const complex_double,
                lda: *const c_int, s: *mut c_double, scond: *mut c_double, amax: *mut c_double,
                work: *mut complex_double, info: *mut c_int);

pub fn sgesv_(n: *const c_int, nrhs: *const c_int, a: *mut c_float, lda: *const c_int,
              ipiv: *mut c_int, b: *mut c_float, ldb: *const c_int, info: *mut c_int);
pub fn dgesv_(n: *const c_int, nrhs: *const c_int, a: *mut c_double, lda: *const c_int,
              ipiv: *mut c_int, b: *mut c_double, ldb: *const c_int, info: *mut c_int);
pub fn cgesv_(n: *const c_int, nrhs: *const c_int, a: *mut complex_float, lda: *const c_int,
              ipiv: *mut c_int, b: *mut complex_float, ldb: *const c_int, info: *mut c_int);
pub fn zgesv_(n: *const c_int, nrhs: *const c_int, a: *mut complex_double, lda: *const c_int,
              ipiv: *mut c_int, b: *mut complex_double, ldb: *const c_int, info: *mut c_int);

pub fn dsgesv_(n: *const c_int, nrhs: *const c_int, a: *mut c_double, lda: *const c_int,
               ipiv: *mut c_int, b: *const c_double, ldb: *const c_int, x: *mut c_double,
               ldx: *const c_int, work: *mut c_double, swork: *mut c_float, iter: *mut c_int,
               info: *mut c_int);
pub fn zcgesv_(n: *const c_int, nrhs: *const c_int, a: *mut complex_double, lda: *const c_int,
               ipiv: *mut c_int, b: *const complex_double, ldb: *const c_int,
               x: *mut complex_double, ldx: *const c_int, work: *mut complex_double,
               swork: *mut complex_float, rwork: *mut c_double, iter: *mut c_int,
               info: *mut c_int);

pub fn sgesvx_(fact: *const c_char, trans: *const c_char, n: *const c_int, nrhs: *const c_int,
               a: *mut c_float, lda: *const c_int, af: *mut c_float, ldaf: *const c_int,
               ipiv: *mut c_int, equed: *mut c_char , r: *mut c_float, c: *mut c_float,
               b: *mut c_float, ldb: *const c_int, x: *mut c_float, ldx: *const c_int,
               rcond: *mut c_float, ferr: *mut c_float, berr: *mut c_float, work: *mut c_float,
               iwork: *mut c_int, info: *mut c_int);
pub fn dgesvx_(fact: *const c_char, trans: *const c_char, n: *const c_int, nrhs: *const c_int,
               a: *mut c_double, lda: *const c_int, af: *mut c_double, ldaf: *const c_int,
               ipiv: *mut c_int, equed: *mut c_char , r: *mut c_double, c: *mut c_double,
               b: *mut c_double, ldb: *const c_int, x: *mut c_double, ldx: *const c_int,
               rcond: *mut c_double, ferr: *mut c_double, berr: *mut c_double,
               work: *mut c_double, iwork: *mut c_int, info: *mut c_int);
pub fn cgesvx_(fact: *const c_char, trans: *const c_char, n: *const c_int, nrhs: *const c_int,
               a: *mut complex_float, lda: *const c_int, af: *mut complex_float,
               ldaf: *const c_int, ipiv: *mut c_int, equed: *mut c_char , r: *mut c_float,
               c: *mut c_float, b: *mut complex_float, ldb: *const c_int,
               x: *mut complex_float, ldx: *const c_int, rcond: *mut c_float,
               ferr: *mut c_float, berr: *mut c_float, work: *mut complex_float,
               rwork: *mut c_float, info: *mut c_int);
pub fn zgesvx_(fact: *const c_char, trans: *const c_char, n: *const c_int, nrhs: *const c_int,
               a: *mut complex_double, lda: *const c_int, af: *mut complex_double,
               ldaf: *const c_int, ipiv: *mut c_int, equed: *mut c_char, r: *mut c_double,
               c: *mut c_double, b: *mut complex_double, ldb: *const c_int,
               x: *mut complex_double, ldx: *const c_int, rcond: *mut c_double,
               ferr: *mut c_double, berr: *mut c_double, work: *mut complex_double,
               rwork: *mut c_double, info: *mut c_int);

pub fn sgesvxx_(fact: *const c_char, trans: *const c_char, n: *const c_int, nrhs: *const c_int,
                a: *mut c_float, lda: *const c_int, af: *mut c_float, ldaf: *const c_int,
                ipiv: *mut c_int, equed: *mut c_char, r: *mut c_float, c: *mut c_float,
                b: *mut c_float, ldb: *const c_int, x: *mut c_float, ldx: *const c_int,
                rcond: *mut c_float, rpvgrw: *mut c_float, berr: *mut c_float,
                n_err_bnds: *const c_int, err_bnds_norm: *mut c_float,
                err_bnds_comp: *mut c_float, nparams: *const c_int, params: *mut c_float,
                work: *mut c_float, iwork: *mut c_int, info: *mut c_int);
pub fn dgesvxx_(fact: *const c_char, trans: *const c_char, n: *const c_int, nrhs: *const c_int,
                a: *mut c_double, lda: *const c_int, af: *mut c_double, ldaf: *const c_int,
                ipiv: *mut c_int, equed: *mut c_char, r: *mut c_double, c: *mut c_double,
                b: *mut c_double, ldb: *const c_int, x: *mut c_double, ldx: *const c_int,
                rcond: *mut c_double, rpvgrw: *mut c_double, berr: *mut c_double,
                n_err_bnds: *const c_int, err_bnds_norm: *mut c_double,
                err_bnds_comp: *mut c_double, nparams: *const c_int, params: *mut c_double,
                work: *mut c_double, iwork: *mut c_int, info: *mut c_int);
pub fn cgesvxx_(fact: *const c_char, trans: *const c_char, n: *const c_int, nrhs: *const c_int,
                a: *mut complex_float, lda: *const c_int, af: *mut complex_float,
                ldaf: *const c_int, ipiv: *mut c_int, equed: *mut c_char, r: *mut c_float,
                c: *mut c_float, b: *mut complex_float, ldb: *const c_int,
                x: *mut complex_float, ldx: *const c_int, rcond: *mut c_float,
                rpvgrw: *mut c_float, berr: *mut c_float, n_err_bnds: *const c_int,
                err_bnds_norm: *mut c_float, err_bnds_comp: *mut c_float,
                nparams: *const c_int, params: *mut c_float, work: *mut complex_float,
                rwork: *mut c_float, info: *mut c_int);
pub fn zgesvxx_(fact: *const c_char, trans: *const c_char, n: *const c_int, nrhs: *const c_int,
                a: *mut complex_double, lda: *const c_int, af: *mut complex_double,
                ldaf: *const c_int, ipiv: *mut c_int, equed: *mut c_char, r: *mut c_double,
                c: *mut c_double, b: *mut complex_double, ldb: *const c_int,
                x: *mut complex_double, ldx: *const c_int, rcond: *mut c_double,
                rpvgrw: *mut c_double, berr: *mut c_double, n_err_bnds: *const c_int,
                err_bnds_norm: *mut c_double, err_bnds_comp: *mut c_double,
                nparams: *const c_int, params: *mut c_double, work: *mut complex_double,
                rwork: *mut c_double, info: *mut c_int);

pub fn sgbsv_(n: *const c_int, kl: *const c_int, ku: *const c_int, nrhs: *const c_int,
              ab: *mut c_float, ldab: *const c_int, ipiv: *mut c_int, b: *mut c_float,
              ldb: *const c_int, info: *mut c_int);
pub fn dgbsv_(n: *const c_int, kl: *const c_int, ku: *const c_int, nrhs: *const c_int,
              ab: *mut c_double, ldab: *const c_int, ipiv: *mut c_int, b: *mut c_double,
              ldb: *const c_int, info: *mut c_int);
pub fn cgbsv_(n: *const c_int, kl: *const c_int, ku: *const c_int, nrhs: *const c_int,
              ab: *mut complex_float, ldab: *const c_int, ipiv: *mut c_int,
              b: *mut complex_float, ldb: *const c_int, info: *mut c_int);
pub fn zgbsv_(n: *const c_int, kl: *const c_int, ku: *const c_int, nrhs: *const c_int,
              ab: *mut complex_double, ldab: *const c_int, ipiv: *mut c_int,
              b: *mut complex_double, ldb: *const c_int, info: *mut c_int);

pub fn sgbsvx_(fact: *const c_char, trans: *const c_char, n: *const c_int, kl: *const c_int,
               ku: *const c_int, nrhs: *const c_int, ab: *mut c_float, ldab: *const c_int,
               afb: *mut c_float, ldafb: *const c_int, ipiv: *mut c_int, equed: *mut c_char,
               r: *mut c_float, c: *mut c_float, b: *mut c_float, ldb: *const c_int,
               x: *mut c_float, ldx: *const c_int, rcond: *mut c_float, ferr: *mut c_float,
               berr: *mut c_float, work: *mut c_float, iwork: *mut c_int, info: *mut c_int);
pub fn dgbsvx_(fact: *const c_char, trans: *const c_char, n: *const c_int, kl: *const c_int,
               ku: *const c_int, nrhs: *const c_int, ab: *mut c_double, ldab: *const c_int,
               afb: *mut c_double, ldafb: *const c_int, ipiv: *mut c_int, equed: *mut c_char,
               r: *mut c_double, c: *mut c_double, b: *mut c_double, ldb: *const c_int,
               x: *mut c_double, ldx: *const c_int, rcond: *mut c_double, ferr: *mut c_double,
               berr: *mut c_double, work: *mut c_double, iwork: *mut c_int, info: *mut c_int);
pub fn cgbsvx_(fact: *const c_char, trans: *const c_char, n: *const c_int, kl: *const c_int,
               ku: *const c_int, nrhs: *const c_int, ab: *mut complex_float,
               ldab: *const c_int, afb: *mut complex_float, ldafb: *const c_int,
               ipiv: *mut c_int, equed: *mut c_char, r: *mut c_float, c: *mut c_float,
               b: *mut complex_float, ldb: *const c_int, x: *mut complex_float,
               ldx: *const c_int, rcond: *mut c_float, ferr: *mut c_float, berr: *mut c_float,
               work: *mut complex_float, rwork: *mut c_float, info: *mut c_int);
pub fn zgbsvx_(fact: *const c_char, trans: *const c_char, n: *const c_int, kl: *const c_int,
               ku: *const c_int, nrhs: *const c_int, ab: *mut complex_double,
               ldab: *const c_int, afb: *mut complex_double, ldafb: *const c_int,
               ipiv: *mut c_int, equed: *mut c_char, r: *mut c_double, c: *mut c_double,
               b: *mut complex_double, ldb: *const c_int, x: *mut complex_double,
               ldx: *const c_int, rcond: *mut c_double, ferr: *mut c_double,
               berr: *mut c_double, work: *mut complex_double, rwork: *mut c_double,
               info: *mut c_int);

pub fn sgbsvxx_(fact: *const c_char, trans: *const c_char, n: *const c_int, kl: *const c_int,
                ku: *const c_int, nrhs: *const c_int, ab: *mut c_float, ldab: *const c_int,
                afb: *mut c_float, ldafb: *const c_int, ipiv: *mut c_int, equed: *mut c_char,
                r: *mut c_float, c: *mut c_float, b: *mut c_float, ldb: *const c_int,
                x: *mut c_float, ldx: *const c_int, rcond: *mut c_float, rpvgrw: *mut c_float,
                berr: *mut c_float, n_err_bnds: *const c_int, err_bnds_norm: *mut c_float,
                err_bnds_comp: *mut c_float, nparams: *const c_int, params: *mut c_float,
                work: *mut c_float, iwork: *mut c_int, info: *mut c_int);
pub fn dgbsvxx_(fact: *const c_char, trans: *const c_char, n: *const c_int, kl: *const c_int,
                ku: *const c_int, nrhs: *const c_int, ab: *mut c_double, ldab: *const c_int,
                afb: *mut c_double, ldafb: *const c_int, ipiv: *mut c_int, equed: *mut c_char,
                r: *mut c_double, c: *mut c_double, b: *mut c_double, ldb: *const c_int,
                x: *mut c_double, ldx: *const c_int, rcond: *mut c_double,
                rpvgrw: *mut c_double, berr: *mut c_double, n_err_bnds: *const c_int,
                err_bnds_norm: *mut c_double, err_bnds_comp: *mut c_double,
                nparams: *const c_int, params: *mut c_double, work: *mut c_double,
                iwork: *mut c_int, info: *mut c_int);
pub fn cgbsvxx_(fact: *const c_char, trans: *const c_char, n: *const c_int, kl: *const c_int,
                ku: *const c_int, nrhs: *const c_int, ab: *mut complex_float,
                ldab: *const c_int, afb: *mut complex_float, ldafb: *const c_int,
                ipiv: *mut c_int, equed: *mut c_char, r: *mut c_float, c: *mut c_float,
                b: *mut complex_float, ldb: *const c_int, x: *mut complex_float,
                ldx: *const c_int, rcond: *mut c_float, rpvgrw: *mut c_float,
                berr: *mut c_float, n_err_bnds: *const c_int, err_bnds_norm: *mut c_float,
                err_bnds_comp: *mut c_float, nparams: *const c_int, params: *mut c_float,
                work: *mut complex_float, rwork: *mut c_float, info: *mut c_int);
pub fn zgbsvxx_(fact: *const c_char, trans: *const c_char, n: *const c_int, kl: *const c_int,
                ku: *const c_int, nrhs: *const c_int, ab: *mut complex_double,
                ldab: *const c_int, afb: *mut complex_double, ldafb: *const c_int,
                ipiv: *mut c_int, equed: *mut c_char, r: *mut c_double, c: *mut c_double,
                b: *mut complex_double, ldb: *const c_int, x: *mut complex_double,
                ldx: *const c_int, rcond: *mut c_double, rpvgrw: *mut c_double,
                berr: *mut c_double, n_err_bnds: *const c_int, err_bnds_norm: *mut c_double,
                err_bnds_comp: *mut c_double, nparams: *const c_int, params: *mut c_double,
                work: *mut complex_double, rwork: *mut c_double, info: *mut c_int);

pub fn sgtsv_(n: *const c_int, nrhs: *const c_int, dl: *mut c_float, d: *mut c_float,
              du: *mut c_float, b: *mut c_float, ldb: *const c_int, info: *mut c_int);
pub fn dgtsv_(n: *const c_int, nrhs: *const c_int, dl: *mut c_double, d: *mut c_double,
              du: *mut c_double, b: *mut c_double, ldb: *const c_int, info: *mut c_int);
pub fn cgtsv_(n: *const c_int, nrhs: *const c_int, dl: *mut complex_float,
              d: *mut complex_float, du: *mut complex_float, b: *mut complex_float,
              ldb: *const c_int, info: *mut c_int);
pub fn zgtsv_(n: *const c_int, nrhs: *const c_int, dl: *mut complex_double,
              d: *mut complex_double, du: *mut complex_double, b: *mut complex_double,
              ldb: *const c_int, info: *mut c_int);

pub fn sgtsvx_(fact: *const c_char, trans: *const c_char, n: *const c_int, nrhs: *const c_int,
               dl: *const c_float, d: *const c_float, du: *const c_float, dlf: *mut c_float,
               df: *mut c_float, duf: *mut c_float, du2: *mut c_float, ipiv: *mut c_int,
               b: *const c_float, ldb: *const c_int, x: *mut c_float, ldx: *const c_int,
               rcond: *mut c_float, ferr: *mut c_float, berr: *mut c_float, work: *mut c_float,
               iwork: *mut c_int, info: *mut c_int);
pub fn dgtsvx_(fact: *const c_char, trans: *const c_char, n: *const c_int, nrhs: *const c_int,
               dl: *const c_double, d: *const c_double, du: *const c_double,
               dlf: *mut c_double, df: *mut c_double, duf: *mut c_double, du2: *mut c_double,
               ipiv: *mut c_int, b: *const c_double, ldb: *const c_int, x: *mut c_double,
               ldx: *const c_int, rcond: *mut c_double, ferr: *mut c_double,
               berr: *mut c_double, work: *mut c_double, iwork: *mut c_int, info: *mut c_int);
pub fn cgtsvx_(fact: *const c_char, trans: *const c_char, n: *const c_int, nrhs: *const c_int,
               dl: *const complex_float, d: *const complex_float, du: *const complex_float,
               dlf: *mut complex_float, df: *mut complex_float, duf: *mut complex_float,
               du2: *mut complex_float, ipiv: *mut c_int, b: *const complex_float,
               ldb: *const c_int, x: *mut complex_float, ldx: *const c_int,
               rcond: *mut c_float, ferr: *mut c_float, berr: *mut c_float,
               work: *mut complex_float, rwork: *mut c_float, info: *mut c_int);
pub fn zgtsvx_(fact: *const c_char, trans: *const c_char, n: *const c_int, nrhs: *const c_int,
               dl: *const complex_double, d: *const complex_double, du: *const complex_double,
               dlf: *mut complex_double, df: *mut complex_double, duf: *mut complex_double,
               du2: *mut complex_double, ipiv: *mut c_int, b: *const complex_double,
               ldb: *const c_int, x: *mut complex_double, ldx: *const c_int,
               rcond: *mut c_double, ferr: *mut c_double, berr: *mut c_double,
               work: *mut complex_double, rwork: *mut c_double, info: *mut c_int);

pub fn sposv_(uplo: *const c_char, n: *const c_int, nrhs: *const c_int, a: *mut c_float,
              lda: *const c_int, b: *mut c_float, ldb: *const c_int, info: *mut c_int);
pub fn dposv_(uplo: *const c_char, n: *const c_int, nrhs: *const c_int, a: *mut c_double,
              lda: *const c_int, b: *mut c_double, ldb: *const c_int, info: *mut c_int);
pub fn cposv_(uplo: *const c_char, n: *const c_int, nrhs: *const c_int, a: *mut complex_float,
              lda: *const c_int, b: *mut complex_float, ldb: *const c_int, info: *mut c_int);
pub fn zposv_(uplo: *const c_char, n: *const c_int, nrhs: *const c_int, a: *mut complex_double,
              lda: *const c_int, b: *mut complex_double, ldb: *const c_int, info: *mut c_int);

pub fn dsposv_(uplo: *const c_char, n: *const c_int, nrhs: *const c_int, a: *mut c_double,
               lda: *const c_int, b: *const c_double, ldb: *const c_int, x: *mut c_double,
               ldx: *const c_int, work: *mut c_double, swork: *mut c_float, iter: *mut c_int,
               info: *mut c_int);
pub fn zcposv_(uplo: *const c_char, n: *const c_int, nrhs: *const c_int,
               a: *mut complex_double, lda: *const c_int, b: *const complex_double,
               ldb: *const c_int, x: *mut complex_double, ldx: *const c_int,
               work: *mut complex_double, swork: *mut complex_float, rwork: *mut c_double,
               iter: *mut c_int, info: *mut c_int);

pub fn sposvx_(fact: *const c_char, uplo: *const c_char, n: *const c_int, nrhs: *const c_int,
               a: *mut c_float, lda: *const c_int, af: *mut c_float, ldaf: *const c_int,
               equed: *mut c_char, s: *mut c_float, b: *mut c_float, ldb: *const c_int,
               x: *mut c_float, ldx: *const c_int, rcond: *mut c_float, ferr: *mut c_float,
               berr: *mut c_float, work: *mut c_float, iwork: *mut c_int, info: *mut c_int);
pub fn dposvx_(fact: *const c_char, uplo: *const c_char, n: *const c_int, nrhs: *const c_int,
               a: *mut c_double, lda: *const c_int, af: *mut c_double, ldaf: *const c_int,
               equed: *mut c_char, s: *mut c_double, b: *mut c_double, ldb: *const c_int,
               x: *mut c_double, ldx: *const c_int, rcond: *mut c_double, ferr: *mut c_double,
               berr: *mut c_double, work: *mut c_double, iwork: *mut c_int, info: *mut c_int);
pub fn cposvx_(fact: *const c_char, uplo: *const c_char, n: *const c_int, nrhs: *const c_int,
               a: *mut complex_float, lda: *const c_int, af: *mut complex_float,
               ldaf: *const c_int, equed: *mut c_char, s: *mut c_float, b: *mut complex_float,
               ldb: *const c_int, x: *mut complex_float, ldx: *const c_int,
               rcond: *mut c_float, ferr: *mut c_float, berr: *mut c_float,
               work: *mut complex_float, rwork: *mut c_float, info: *mut c_int);
pub fn zposvx_(fact: *const c_char, uplo: *const c_char, n: *const c_int, nrhs: *const c_int,
               a: *mut complex_double, lda: *const c_int, af: *mut complex_double,
               ldaf: *const c_int, equed: *mut c_char, s: *mut c_double,
               b: *mut complex_double, ldb: *const c_int, x: *mut complex_double,
               ldx: *const c_int, rcond: *mut c_double, ferr: *mut c_double,
               berr: *mut c_double, work: *mut complex_double, rwork: *mut c_double,
               info: *mut c_int);

pub fn sposvxx_(fact: *const c_char, uplo: *const c_char, n: *const c_int, nrhs: *const c_int,
                a: *mut c_float, lda: *const c_int, af: *mut c_float, ldaf: *const c_int,
                equed: *mut c_char, s: *mut c_float, b: *mut c_float, ldb: *const c_int,
                x: *mut c_float, ldx: *const c_int, rcond: *mut c_float, rpvgrw: *mut c_float,
                berr: *mut c_float, n_err_bnds: *const c_int, err_bnds_norm: *mut c_float,
                err_bnds_comp: *mut c_float, nparams: *const c_int, params: *mut c_float,
                work: *mut c_float, iwork: *mut c_int, info: *mut c_int);
pub fn dposvxx_(fact: *const c_char, uplo: *const c_char, n: *const c_int, nrhs: *const c_int,
                a: *mut c_double, lda: *const c_int, af: *mut c_double, ldaf: *const c_int,
                equed: *mut c_char, s: *mut c_double, b: *mut c_double, ldb: *const c_int,
                x: *mut c_double, ldx: *const c_int, rcond: *mut c_double,
                rpvgrw: *mut c_double, berr: *mut c_double, n_err_bnds: *const c_int,
                err_bnds_norm: *mut c_double, err_bnds_comp: *mut c_double,
                nparams: *const c_int, params: *mut c_double, work: *mut c_double,
                iwork: *mut c_int, info: *mut c_int);
pub fn cposvxx_(fact: *const c_char, uplo: *const c_char, n: *const c_int, nrhs: *const c_int,
                a: *mut complex_float, lda: *const c_int, af: *mut complex_float,
                ldaf: *const c_int, equed: *mut c_char, s: *mut c_float, b: *mut complex_float,
                ldb: *const c_int, x: *mut complex_float, ldx: *const c_int,
                rcond: *mut c_float, rpvgrw: *mut c_float, berr: *mut c_float,
                n_err_bnds: *const c_int, err_bnds_norm: *mut c_float,
                err_bnds_comp: *mut c_float, nparams: *const c_int, params: *mut c_float,
                work: *mut complex_float, rwork: *mut c_float, info: *mut c_int);
pub fn zposvxx_(fact: *const c_char, uplo: *const c_char, n: *const c_int, nrhs: *const c_int,
                a: *mut complex_double, lda: *const c_int, af: *mut complex_double,
                ldaf: *const c_int, equed: *mut c_char, s: *mut c_double,
                b: *mut complex_double, ldb: *const c_int, x: *mut complex_double,
                ldx: *const c_int, rcond: *mut c_double, rpvgrw: *mut c_double,
                berr: *mut c_double, n_err_bnds: *const c_int, err_bnds_norm: *mut c_double,
                err_bnds_comp: *mut c_double, nparams: *const c_int, params: *mut c_double,
                work: *mut complex_double, rwork: *mut c_double, info: *mut c_int);

pub fn sppsv_(uplo: *const c_char, n: *const c_int, nrhs: *const c_int, ap: *mut c_float,
              b: *mut c_float, ldb: *const c_int, info: *mut c_int);
pub fn dppsv_(uplo: *const c_char, n: *const c_int, nrhs: *const c_int, ap: *mut c_double,
              b: *mut c_double, ldb: *const c_int, info: *mut c_int);
pub fn cppsv_(uplo: *const c_char, n: *const c_int, nrhs: *const c_int, ap: *mut complex_float,
              b: *mut complex_float, ldb: *const c_int, info: *mut c_int);
pub fn zppsv_(uplo: *const c_char, n: *const c_int, nrhs: *const c_int,
              ap: *mut complex_double, b: *mut complex_double, ldb: *const c_int,
              info: *mut c_int);

pub fn sppsvx_(fact: *const c_char, uplo: *const c_char, n: *const c_int, nrhs: *const c_int,
               ap: *mut c_float, afp: *mut c_float, equed: *mut c_char, s: *mut c_float,
               b: *mut c_float, ldb: *const c_int, x: *mut c_float, ldx: *const c_int,
               rcond: *mut c_float, ferr: *mut c_float, berr: *mut c_float, work: *mut c_float,
               iwork: *mut c_int, info: *mut c_int);
pub fn dppsvx_(fact: *const c_char, uplo: *const c_char, n: *const c_int, nrhs: *const c_int,
               ap: *mut c_double, afp: *mut c_double, equed: *mut c_char, s: *mut c_double,
               b: *mut c_double, ldb: *const c_int, x: *mut c_double, ldx: *const c_int,
               rcond: *mut c_double, ferr: *mut c_double, berr: *mut c_double,
               work: *mut c_double, iwork: *mut c_int, info: *mut c_int);
pub fn cppsvx_(fact: *const c_char, uplo: *const c_char, n: *const c_int, nrhs: *const c_int,
               ap: *mut complex_float, afp: *mut complex_float, equed: *mut c_char,
               s: *mut c_float, b: *mut complex_float, ldb: *const c_int,
               x: *mut complex_float, ldx: *const c_int, rcond: *mut c_float,
               ferr: *mut c_float, berr: *mut c_float, work: *mut complex_float,
               rwork: *mut c_float, info: *mut c_int);
pub fn zppsvx_(fact: *const c_char, uplo: *const c_char, n: *const c_int, nrhs: *const c_int,
               ap: *mut complex_double, afp: *mut complex_double, equed: *mut c_char,
               s: *mut c_double, b: *mut complex_double, ldb: *const c_int,
               x: *mut complex_double, ldx: *const c_int, rcond: *mut c_double,
               ferr: *mut c_double, berr: *mut c_double, work: *mut complex_double,
               rwork: *mut c_double, info: *mut c_int);

pub fn spbsv_(uplo: *const c_char, n: *const c_int, kd: *const c_int, nrhs: *const c_int,
              ab: *mut c_float, ldab: *const c_int, b: *mut c_float, ldb: *const c_int,
              info: *mut c_int);
pub fn dpbsv_(uplo: *const c_char, n: *const c_int, kd: *const c_int, nrhs: *const c_int,
              ab: *mut c_double, ldab: *const c_int, b: *mut c_double, ldb: *const c_int,
              info: *mut c_int);
pub fn cpbsv_(uplo: *const c_char, n: *const c_int, kd: *const c_int, nrhs: *const c_int,
              ab: *mut complex_float, ldab: *const c_int, b: *mut complex_float,
              ldb: *const c_int, info: *mut c_int);
pub fn zpbsv_(uplo: *const c_char, n: *const c_int, kd: *const c_int, nrhs: *const c_int,
              ab: *mut complex_double, ldab: *const c_int, b: *mut complex_double,
              ldb: *const c_int, info: *mut c_int);

pub fn spbsvx_(fact: *const c_char, uplo: *const c_char, n: *const c_int, kd: *const c_int,
               nrhs: *const c_int, ab: *mut c_float, ldab: *const c_int, afb: *mut c_float,
               ldafb: *const c_int, equed: *mut c_char, s: *mut c_float, b: *mut c_float,
               ldb: *const c_int, x: *mut c_float, ldx: *const c_int, rcond: *mut c_float,
               ferr: *mut c_float, berr: *mut c_float, work: *mut c_float, iwork: *mut c_int,
               info: *mut c_int);
pub fn dpbsvx_(fact: *const c_char, uplo: *const c_char, n: *const c_int, kd: *const c_int,
               nrhs: *const c_int, ab: *mut c_double, ldab: *const c_int, afb: *mut c_double,
               ldafb: *const c_int, equed: *mut c_char, s: *mut c_double, b: *mut c_double,
               ldb: *const c_int, x: *mut c_double, ldx: *const c_int, rcond: *mut c_double,
               ferr: *mut c_double, berr: *mut c_double, work: *mut c_double,
               iwork: *mut c_int, info: *mut c_int);
pub fn cpbsvx_(fact: *const c_char, uplo: *const c_char, n: *const c_int, kd: *const c_int,
               nrhs: *const c_int, ab: *mut complex_float, ldab: *const c_int,
               afb: *mut complex_float, ldafb: *const c_int, equed: *mut c_char,
               s: *mut c_float, b: *mut complex_float, ldb: *const c_int,
               x: *mut complex_float, ldx: *const c_int, rcond: *mut c_float,
               ferr: *mut c_float, berr: *mut c_float, work: *mut complex_float,
               rwork: *mut c_float, info: *mut c_int);
pub fn zpbsvx_(fact: *const c_char, uplo: *const c_char, n: *const c_int, kd: *const c_int,
               nrhs: *const c_int, ab: *mut complex_double, ldab: *const c_int,
               afb: *mut complex_double, ldafb: *const c_int, equed: *mut c_char,
               s: *mut c_double, b: *mut complex_double, ldb: *const c_int,
               x: *mut complex_double, ldx: *const c_int, rcond: *mut c_double,
               ferr: *mut c_double, berr: *mut c_double, work: *mut complex_double,
               rwork: *mut c_double, info: *mut c_int);

pub fn sptsv_(n: *const c_int, nrhs: *const c_int, d: *mut c_float, e: *mut c_float,
              b: *mut c_float, ldb: *const c_int, info: *mut c_int);
pub fn dptsv_(n: *const c_int, nrhs: *const c_int, d: *mut c_double, e: *mut c_double,
              b: *mut c_double, ldb: *const c_int, info: *mut c_int);
pub fn cptsv_(n: *const c_int, nrhs: *const c_int, d: *mut c_float, e: *mut complex_float,
              b: *mut complex_float, ldb: *const c_int, info: *mut c_int);
pub fn zptsv_(n: *const c_int, nrhs: *const c_int, d: *mut c_double, e: *mut complex_double,
              b: *mut complex_double, ldb: *const c_int, info: *mut c_int);

pub fn sptsvx_(fact: *const c_char, n: *const c_int, nrhs: *const c_int, d: *const c_float,
               e: *const c_float, df: *mut c_float, ef: *mut c_float, b: *const c_float,
               ldb: *const c_int, x: *mut c_float, ldx: *const c_int, rcond: *mut c_float,
               ferr: *mut c_float, berr: *mut c_float, work: *mut c_float, info: *mut c_int);
pub fn dptsvx_(fact: *const c_char, n: *const c_int, nrhs: *const c_int, d: *const c_double,
               e: *const c_double, df: *mut c_double, ef: *mut c_double, b: *const c_double,
               ldb: *const c_int, x: *mut c_double, ldx: *const c_int, rcond: *mut c_double,
               ferr: *mut c_double, berr: *mut c_double, work: *mut c_double,
               info: *mut c_int);
pub fn cptsvx_(fact: *const c_char, n: *const c_int, nrhs: *const c_int, d: *const c_float,
               e: *const complex_float, df: *mut c_float, ef: *mut complex_float,
               b: *const complex_float, ldb: *const c_int, x: *mut complex_float,
               ldx: *const c_int, rcond: *mut c_float, ferr: *mut c_float, berr: *mut c_float,
               work: *mut complex_float, rwork: *mut c_float, info: *mut c_int);
pub fn zptsvx_(fact: *const c_char, n: *const c_int, nrhs: *const c_int, d: *const c_double,
               e: *const complex_double, df: *mut c_double, ef: *mut complex_double,
               b: *const complex_double, ldb: *const c_int, x: *mut complex_double,
               ldx: *const c_int, rcond: *mut c_double, ferr: *mut c_double,
               berr: *mut c_double, work: *mut complex_double, rwork: *mut c_double,
               info: *mut c_int);

pub fn ssysv_(uplo: *const c_char, n: *const c_int, nrhs: *const c_int, a: *mut c_float,
              lda: *const c_int, ipiv: *mut c_int, b: *mut c_float, ldb: *const c_int,
              work: *mut c_float, lwork: *const c_int, info: *mut c_int);
pub fn dsysv_(uplo: *const c_char, n: *const c_int, nrhs: *const c_int, a: *mut c_double,
              lda: *const c_int, ipiv: *mut c_int, b: *mut c_double, ldb: *const c_int,
              work: *mut c_double, lwork: *const c_int, info: *mut c_int);
pub fn csysv_(uplo: *const c_char, n: *const c_int, nrhs: *const c_int, a: *mut complex_float,
              lda: *const c_int, ipiv: *mut c_int, b: *mut complex_float, ldb: *const c_int,
              work: *mut complex_float, lwork: *const c_int, info: *mut c_int);
pub fn zsysv_(uplo: *const c_char, n: *const c_int, nrhs: *const c_int, a: *mut complex_double,
              lda: *const c_int, ipiv: *mut c_int, b: *mut complex_double, ldb: *const c_int,
              work: *mut complex_double, lwork: *const c_int, info: *mut c_int);

pub fn ssysvx_(fact: *const c_char, uplo: *const c_char, n: *const c_int, nrhs: *const c_int,
               a: *const c_float, lda: *const c_int, af: *mut c_float, ldaf: *const c_int,
               ipiv: *mut c_int, b: *const c_float, ldb: *const c_int, x: *mut c_float,
               ldx: *const c_int, rcond: *mut c_float, ferr: *mut c_float, berr: *mut c_float,
               work: *mut c_float, lwork: *const c_int, iwork: *mut c_int, info: *mut c_int);
pub fn dsysvx_(fact: *const c_char, uplo: *const c_char, n: *const c_int, nrhs: *const c_int,
               a: *const c_double, lda: *const c_int, af: *mut c_double, ldaf: *const c_int,
               ipiv: *mut c_int, b: *const c_double, ldb: *const c_int, x: *mut c_double,
               ldx: *const c_int, rcond: *mut c_double, ferr: *mut c_double,
               berr: *mut c_double, work: *mut c_double, lwork: *const c_int,
               iwork: *mut c_int, info: *mut c_int);
pub fn csysvx_(fact: *const c_char, uplo: *const c_char, n: *const c_int, nrhs: *const c_int,
               a: *const complex_float, lda: *const c_int, af: *mut complex_float,
               ldaf: *const c_int, ipiv: *mut c_int, b: *const complex_float,
               ldb: *const c_int, x: *mut complex_float, ldx: *const c_int,
               rcond: *mut c_float, ferr: *mut c_float, berr: *mut c_float,
               work: *mut complex_float, lwork: *const c_int, rwork: *mut c_float,
               info: *mut c_int);
pub fn zsysvx_(fact: *const c_char, uplo: *const c_char, n: *const c_int, nrhs: *const c_int,
               a: *const complex_double, lda: *const c_int, af: *mut complex_double,
               ldaf: *const c_int, ipiv: *mut c_int, b: *const complex_double,
               ldb: *const c_int, x: *mut complex_double, ldx: *const c_int,
               rcond: *mut c_double, ferr: *mut c_double, berr: *mut c_double,
               work: *mut complex_double, lwork: *const c_int, rwork: *mut c_double,
               info: *mut c_int);

pub fn dsysvxx_(fact: *const c_char, uplo: *mut c_char, n: *const c_int, nrhs: *const c_int,
                a: *mut c_double, lda: *const c_int, af: *mut c_double, ldaf: *const c_int,
                ipiv: *mut c_int, equed: *mut c_char, s: *mut c_double, b: *mut c_double,
                ldb: *const c_int, x: *mut c_double, ldx: *const c_int, rcond: *mut c_double,
                rpvgrw: *mut c_double, berr: *mut c_double, n_err_bnds: *const c_int,
                err_bnds_norm: *mut c_double, err_bnds_comp: *mut c_double,
                nparams: *const c_int, params: *mut c_double, work: *mut c_double,
                iwork: *mut c_int, info: *mut c_int);
pub fn ssysvxx_(fact: *const c_char, uplo: *mut c_char, n: *const c_int, nrhs: *const c_int,
                a: *mut c_float, lda: *const c_int, af: *mut c_float, ldaf: *const c_int,
                ipiv: *mut c_int, equed: *mut c_char, s: *mut c_float, b: *mut c_float,
                ldb: *const c_int, x: *mut c_float, ldx: *const c_int, rcond: *mut c_float,
                rpvgrw: *mut c_float, berr: *mut c_float, n_err_bnds: *const c_int,
                err_bnds_norm: *mut c_float, err_bnds_comp: *mut c_float,
                nparams: *const c_int, params: *mut c_float, work: *mut c_float,
                iwork: *mut c_int, info: *mut c_int);
pub fn csysvxx_(fact: *const c_char, uplo: *mut c_char, n: *const c_int, nrhs: *const c_int,
                a: *mut complex_float, lda: *const c_int, af: *mut complex_float,
                ldaf: *const c_int, ipiv: *mut c_int, equed: *mut c_char, s: *mut c_float,
                b: *mut complex_float, ldb: *const c_int, x: *mut complex_float,
                ldx: *const c_int, rcond: *mut c_float, rpvgrw: *mut c_float,
                berr: *mut c_float, n_err_bnds: *const c_int, err_bnds_norm: *mut c_float,
                err_bnds_comp: *mut c_float, nparams: *const c_int, params: *mut c_float,
                work: *mut complex_float, rwork: *mut c_float, info: *mut c_int);
pub fn zsysvxx_(fact: *const c_char, uplo: *mut c_char, n: *const c_int, nrhs: *const c_int,
                a: *mut complex_double, lda: *const c_int, af: *mut complex_double,
                ldaf: *const c_int, ipiv: *mut c_int, equed: *mut c_char, s: *mut c_double,
                b: *mut complex_double, ldb: *const c_int, x: *mut complex_double,
                ldx: *const c_int, rcond: *mut c_double, rpvgrw: *mut c_double,
                berr: *mut c_double, n_err_bnds: *const c_int, err_bnds_norm: *mut c_double,
                err_bnds_comp: *mut c_double, nparams: *const c_int, params: *mut c_double,
                work: *mut complex_double, rwork: *mut c_double, info: *mut c_int);

pub fn chesv_(uplo: *const c_char, n: *const c_int, nrhs: *const c_int, a: *mut complex_float,
              lda: *const c_int, ipiv: *mut c_int, b: *mut complex_float, ldb: *const c_int,
              work: *mut complex_float, lwork: *const c_int, info: *mut c_int);
pub fn zhesv_(uplo: *const c_char, n: *const c_int, nrhs: *const c_int, a: *mut complex_double,
              lda: *const c_int, ipiv: *mut c_int, b: *mut complex_double, ldb: *const c_int,
              work: *mut complex_double, lwork: *const c_int, info: *mut c_int);

pub fn chesvx_(fact: *const c_char, uplo: *const c_char, n: *const c_int, nrhs: *const c_int,
               a: *const complex_float, lda: *const c_int, af: *mut complex_float,
               ldaf: *const c_int, ipiv: *mut c_int, b: *const complex_float,
               ldb: *const c_int, x: *mut complex_float, ldx: *const c_int,
               rcond: *mut c_float, ferr: *mut c_float, berr: *mut c_float,
               work: *mut complex_float, lwork: *const c_int, rwork: *mut c_float,
               info: *mut c_int);
pub fn zhesvx_(fact: *const c_char, uplo: *const c_char, n: *const c_int, nrhs: *const c_int,
               a: *const complex_double, lda: *const c_int, af: *mut complex_double,
               ldaf: *const c_int, ipiv: *mut c_int, b: *const complex_double,
               ldb: *const c_int, x: *mut complex_double, ldx: *const c_int,
               rcond: *mut c_double, ferr: *mut c_double, berr: *mut c_double,
               work: *mut complex_double, lwork: *const c_int, rwork: *mut c_double,
               info: *mut c_int);

pub fn chesvxx_(fact: *const c_char, uplo: *mut c_char, n: *const c_int, nrhs: *const c_int,
                a: *mut complex_float, lda: *const c_int, af: *mut complex_float,
                ldaf: *const c_int, ipiv: *mut c_int, equed: *mut c_char, s: *mut c_float,
                b: *mut complex_float, ldb: *const c_int, x: *mut complex_float,
                ldx: *const c_int, rcond: *mut c_float, rpvgrw: *mut c_float,
                berr: *mut c_float, n_err_bnds: *const c_int, err_bnds_norm: *mut c_float,
                err_bnds_comp: *mut c_float, nparams: *const c_int, params: *mut c_float,
                work: *mut complex_float, rwork: *mut c_float, info: *mut c_int);
pub fn zhesvxx_(fact: *const c_char, uplo: *mut c_char, n: *const c_int, nrhs: *const c_int,
                a: *mut complex_double, lda: *const c_int, af: *mut complex_double,
                ldaf: *const c_int, ipiv: *mut c_int, equed: *mut c_char, s: *mut c_double,
                b: *mut complex_double, ldb: *const c_int, x: *mut complex_double,
                ldx: *const c_int, rcond: *mut c_double, rpvgrw: *mut c_double,
                berr: *mut c_double, n_err_bnds: *const c_int, err_bnds_norm: *mut c_double,
                err_bnds_comp: *mut c_double, nparams: *const c_int, params: *mut c_double,
                work: *mut complex_double, rwork: *mut c_double, info: *mut c_int);

pub fn sspsv_(uplo: *const c_char, n: *const c_int, nrhs: *const c_int, ap: *mut c_float,
              ipiv: *mut c_int, b: *mut c_float, ldb: *const c_int, info: *mut c_int);
pub fn dspsv_(uplo: *const c_char, n: *const c_int, nrhs: *const c_int, ap: *mut c_double,
              ipiv: *mut c_int, b: *mut c_double, ldb: *const c_int, info: *mut c_int);
pub fn cspsv_(uplo: *const c_char, n: *const c_int, nrhs: *const c_int, ap: *mut complex_float,
              ipiv: *mut c_int, b: *mut complex_float, ldb: *const c_int, info: *mut c_int);
pub fn zspsv_(uplo: *const c_char, n: *const c_int, nrhs: *const c_int,
              ap: *mut complex_double, ipiv: *mut c_int, b: *mut complex_double,
              ldb: *const c_int, info: *mut c_int);

pub fn sspsvx_(fact: *const c_char, uplo: *const c_char, n: *const c_int, nrhs: *const c_int,
               ap: *const c_float, afp: *mut c_float, ipiv: *mut c_int, b: *const c_float,
               ldb: *const c_int, x: *mut c_float, ldx: *const c_int, rcond: *mut c_float,
               ferr: *mut c_float, berr: *mut c_float, work: *mut c_float, iwork: *mut c_int,
               info: *mut c_int);
pub fn dspsvx_(fact: *const c_char, uplo: *const c_char, n: *const c_int, nrhs: *const c_int,
               ap: *const c_double, afp: *mut c_double, ipiv: *mut c_int, b: *const c_double,
               ldb: *const c_int, x: *mut c_double, ldx: *const c_int, rcond: *mut c_double,
               ferr: *mut c_double, berr: *mut c_double, work: *mut c_double,
               iwork: *mut c_int, info: *mut c_int);
pub fn cspsvx_(fact: *const c_char, uplo: *const c_char, n: *const c_int, nrhs: *const c_int,
               ap: *const complex_float, afp: *mut complex_float, ipiv: *mut c_int,
               b: *const complex_float, ldb: *const c_int, x: *mut complex_float,
               ldx: *const c_int, rcond: *mut c_float, ferr: *mut c_float, berr: *mut c_float,
               work: *mut complex_float, rwork: *mut c_float, info: *mut c_int);
pub fn zspsvx_(fact: *const c_char, uplo: *const c_char, n: *const c_int, nrhs: *const c_int,
               ap: *const complex_double, afp: *mut complex_double, ipiv: *mut c_int,
               b: *const complex_double, ldb: *const c_int, x: *mut complex_double,
               ldx: *const c_int, rcond: *mut c_double, ferr: *mut c_double,
               berr: *mut c_double, work: *mut complex_double, rwork: *mut c_double,
               info: *mut c_int);

pub fn chpsv_(uplo: *const c_char, n: *const c_int, nrhs: *const c_int, ap: *mut complex_float,
              ipiv: *mut c_int, b: *mut complex_float, ldb: *const c_int, info: *mut c_int);
pub fn zhpsv_(uplo: *const c_char, n: *const c_int, nrhs: *const c_int,
              ap: *mut complex_double, ipiv: *mut c_int, b: *mut complex_double,
              ldb: *const c_int, info: *mut c_int);

pub fn chpsvx_(fact: *const c_char, uplo: *const c_char, n: *const c_int, nrhs: *const c_int,
               ap: *const complex_float, afp: *mut complex_float, ipiv: *mut c_int,
               b: *const complex_float, ldb: *const c_int, x: *mut complex_float,
               ldx: *const c_int, rcond: *mut c_float, ferr: *mut c_float, berr: *mut c_float,
               work: *mut complex_float, rwork: *mut c_float, info: *mut c_int);
pub fn zhpsvx_(fact: *const c_char, uplo: *const c_char, n: *const c_int, nrhs: *const c_int,
               ap: *const complex_double, afp: *mut complex_double, ipiv: *mut c_int,
               b: *const complex_double, ldb: *const c_int, x: *mut complex_double,
               ldx: *const c_int, rcond: *mut c_double, ferr: *mut c_double,
               berr: *mut c_double, work: *mut complex_double, rwork: *mut c_double,
               info: *mut c_int);

pub fn sgeqrf_(m: *const c_int, n: *const c_int, a: *mut c_float, lda: *const c_int,
               tau: *mut c_float, work: *mut c_float, lwork: *const c_int, info: *mut c_int);
pub fn dgeqrf_(m: *const c_int, n: *const c_int, a: *mut c_double, lda: *const c_int,
               tau: *mut c_double, work: *mut c_double, lwork: *const c_int, info: *mut c_int);
pub fn cgeqrf_(m: *const c_int, n: *const c_int, a: *mut complex_float, lda: *const c_int,
               tau: *mut complex_float, work: *mut complex_float, lwork: *const c_int,
               info: *mut c_int);
pub fn zgeqrf_(m: *const c_int, n: *const c_int, a: *mut complex_double, lda: *const c_int,
               tau: *mut complex_double, work: *mut complex_double, lwork: *const c_int,
               info: *mut c_int);

pub fn sgeqpf_(m: *const c_int, n: *const c_int, a: *mut c_float, lda: *const c_int,
               jpvt: *mut c_int, tau: *mut c_float, work: *mut c_float, info: *mut c_int);
pub fn dgeqpf_(m: *const c_int, n: *const c_int, a: *mut c_double, lda: *const c_int,
               jpvt: *mut c_int, tau: *mut c_double, work: *mut c_double, info: *mut c_int);
pub fn cgeqpf_(m: *const c_int, n: *const c_int, a: *mut complex_float, lda: *const c_int,
               jpvt: *mut c_int, tau: *mut complex_float, work: *mut complex_float,
               rwork: *mut c_float, info: *mut c_int);
pub fn zgeqpf_(m: *const c_int, n: *const c_int, a: *mut complex_double, lda: *const c_int,
               jpvt: *mut c_int, tau: *mut complex_double, work: *mut complex_double,
               rwork: *mut c_double, info: *mut c_int);

pub fn sgeqp3_(m: *const c_int, n: *const c_int, a: *mut c_float, lda: *const c_int,
               jpvt: *mut c_int, tau: *mut c_float, work: *mut c_float, lwork: *const c_int,
               info: *mut c_int);
pub fn dgeqp3_(m: *const c_int, n: *const c_int, a: *mut c_double, lda: *const c_int,
               jpvt: *mut c_int, tau: *mut c_double, work: *mut c_double, lwork: *const c_int,
               info: *mut c_int);
pub fn cgeqp3_(m: *const c_int, n: *const c_int, a: *mut complex_float, lda: *const c_int,
               jpvt: *mut c_int, tau: *mut complex_float, work: *mut complex_float,
               lwork: *const c_int, rwork: *mut c_float, info: *mut c_int);
pub fn zgeqp3_(m: *const c_int, n: *const c_int, a: *mut complex_double, lda: *const c_int,
               jpvt: *mut c_int, tau: *mut complex_double, work: *mut complex_double,
               lwork: *const c_int, rwork: *mut c_double, info: *mut c_int);

pub fn sorgqr_(m: *const c_int, n: *const c_int, k: *const c_int, a: *mut c_float,
               lda: *const c_int, tau: *const c_float, work: *mut c_float, lwork: *const c_int,
               info: *mut c_int);
pub fn dorgqr_(m: *const c_int, n: *const c_int, k: *const c_int, a: *mut c_double,
               lda: *const c_int, tau: *const c_double, work: *mut c_double,
               lwork: *const c_int, info: *mut c_int);

pub fn sormqr_(side: *const c_char, trans: *const c_char, m: *const c_int, n: *const c_int,
               k: *const c_int, a: *const c_float, lda: *const c_int, tau: *const c_float,
               c: *mut c_float, ldc: *const c_int, work: *mut c_float, lwork: *const c_int,
               info: *mut c_int);
pub fn dormqr_(side: *const c_char, trans: *const c_char, m: *const c_int, n: *const c_int,
               k: *const c_int, a: *const c_double, lda: *const c_int, tau: *const c_double,
               c: *mut c_double, ldc: *const c_int, work: *mut c_double, lwork: *const c_int,
               info: *mut c_int);

pub fn cungqr_(m: *const c_int, n: *const c_int, k: *const c_int, a: *mut complex_float,
               lda: *const c_int, tau: *const complex_float, work: *mut complex_float,
               lwork: *const c_int, info: *mut c_int);
pub fn zungqr_(m: *const c_int, n: *const c_int, k: *const c_int, a: *mut complex_double,
               lda: *const c_int, tau: *const complex_double, work: *mut complex_double,
               lwork: *const c_int, info: *mut c_int);

pub fn cunmqr_(side: *const c_char, trans: *const c_char, m: *const c_int, n: *const c_int,
               k: *const c_int, a: *const complex_float, lda: *const c_int,
               tau: *const complex_float, c: *mut complex_float, ldc: *const c_int,
               work: *mut complex_float, lwork: *const c_int, info: *mut c_int);
pub fn zunmqr_(side: *const c_char, trans: *const c_char, m: *const c_int, n: *const c_int,
               k: *const c_int, a: *const complex_double, lda: *const c_int,
               tau: *const complex_double, c: *mut complex_double, ldc: *const c_int,
               work: *mut complex_double, lwork: *const c_int, info: *mut c_int);

pub fn sgelqf_(m: *const c_int, n: *const c_int, a: *mut c_float, lda: *const c_int,
               tau: *mut c_float, work: *mut c_float, lwork: *const c_int, info: *mut c_int);
pub fn dgelqf_(m: *const c_int, n: *const c_int, a: *mut c_double, lda: *const c_int,
               tau: *mut c_double, work: *mut c_double, lwork: *const c_int, info: *mut c_int);
pub fn cgelqf_(m: *const c_int, n: *const c_int, a: *mut complex_float, lda: *const c_int,
               tau: *mut complex_float, work: *mut complex_float, lwork: *const c_int,
               info: *mut c_int);
pub fn zgelqf_(m: *const c_int, n: *const c_int, a: *mut complex_double, lda: *const c_int,
               tau: *mut complex_double, work: *mut complex_double, lwork: *const c_int,
               info: *mut c_int);

pub fn sorglq_(m: *const c_int, n: *const c_int, k: *const c_int, a: *mut c_float,
               lda: *const c_int, tau: *const c_float, work: *mut c_float, lwork: *const c_int,
               info: *mut c_int);
pub fn dorglq_(m: *const c_int, n: *const c_int, k: *const c_int, a: *mut c_double,
               lda: *const c_int, tau: *const c_double, work: *mut c_double,
               lwork: *const c_int, info: *mut c_int);

pub fn sormlq_(side: *const c_char, trans: *const c_char, m: *const c_int, n: *const c_int,
               k: *const c_int, a: *const c_float, lda: *const c_int, tau: *const c_float,
               c: *mut c_float, ldc: *const c_int, work: *mut c_float, lwork: *const c_int,
               info: *mut c_int);
pub fn dormlq_(side: *const c_char, trans: *const c_char, m: *const c_int, n: *const c_int,
               k: *const c_int, a: *const c_double, lda: *const c_int, tau: *const c_double,
               c: *mut c_double, ldc: *const c_int, work: *mut c_double, lwork: *const c_int,
               info: *mut c_int);

pub fn cunglq_(m: *const c_int, n: *const c_int, k: *const c_int, a: *mut complex_float,
               lda: *const c_int, tau: *const complex_float, work: *mut complex_float,
               lwork: *const c_int, info: *mut c_int);
pub fn zunglq_(m: *const c_int, n: *const c_int, k: *const c_int, a: *mut complex_double,
               lda: *const c_int, tau: *const complex_double, work: *mut complex_double,
               lwork: *const c_int, info: *mut c_int);

pub fn cunmlq_(side: *const c_char, trans: *const c_char, m: *const c_int, n: *const c_int,
               k: *const c_int, a: *const complex_float, lda: *const c_int,
               tau: *const complex_float, c: *mut complex_float, ldc: *const c_int,
               work: *mut complex_float, lwork: *const c_int, info: *mut c_int);
pub fn zunmlq_(side: *const c_char, trans: *const c_char, m: *const c_int, n: *const c_int,
               k: *const c_int, a: *const complex_double, lda: *const c_int,
               tau: *const complex_double, c: *mut complex_double, ldc: *const c_int,
               work: *mut complex_double, lwork: *const c_int, info: *mut c_int);

pub fn sgeqlf_(m: *const c_int, n: *const c_int, a: *mut c_float, lda: *const c_int,
               tau: *mut c_float, work: *mut c_float, lwork: *const c_int, info: *mut c_int);
pub fn dgeqlf_(m: *const c_int, n: *const c_int, a: *mut c_double, lda: *const c_int,
               tau: *mut c_double, work: *mut c_double, lwork: *const c_int, info: *mut c_int);
pub fn cgeqlf_(m: *const c_int, n: *const c_int, a: *mut complex_float, lda: *const c_int,
               tau: *mut complex_float, work: *mut complex_float, lwork: *const c_int,
               info: *mut c_int);
pub fn zgeqlf_(m: *const c_int, n: *const c_int, a: *mut complex_double, lda: *const c_int,
               tau: *mut complex_double, work: *mut complex_double, lwork: *const c_int,
               info: *mut c_int);

pub fn sorgql_(m: *const c_int, n: *const c_int, k: *const c_int, a: *mut c_float,
               lda: *const c_int, tau: *const c_float, work: *mut c_float, lwork: *const c_int,
               info: *mut c_int);
pub fn dorgql_(m: *const c_int, n: *const c_int, k: *const c_int, a: *mut c_double,
               lda: *const c_int, tau: *const c_double, work: *mut c_double,
               lwork: *const c_int, info: *mut c_int);
pub fn cungql_(m: *const c_int, n: *const c_int, k: *const c_int, a: *mut complex_float,
               lda: *const c_int, tau: *const complex_float, work: *mut complex_float,
               lwork: *const c_int, info: *mut c_int);
pub fn zungql_(m: *const c_int, n: *const c_int, k: *const c_int, a: *mut complex_double,
               lda: *const c_int, tau: *const complex_double, work: *mut complex_double,
               lwork: *const c_int, info: *mut c_int);

pub fn sormql_(side: *const c_char, trans: *const c_char, m: *const c_int, n: *const c_int,
               k: *const c_int, a: *const c_float, lda: *const c_int, tau: *const c_float,
               c: *mut c_float, ldc: *const c_int, work: *mut c_float, lwork: *const c_int,
               info: *mut c_int);
pub fn dormql_(side: *const c_char, trans: *const c_char, m: *const c_int, n: *const c_int,
               k: *const c_int, a: *const c_double, lda: *const c_int, tau: *const c_double,
               c: *mut c_double, ldc: *const c_int, work: *mut c_double, lwork: *const c_int,
               info: *mut c_int);
pub fn cunmql_(side: *const c_char, trans: *const c_char, m: *const c_int, n: *const c_int,
               k: *const c_int, a: *const complex_float, lda: *const c_int,
               tau: *const complex_float, c: *mut complex_float, ldc: *const c_int,
               work: *mut complex_float, lwork: *const c_int, info: *mut c_int);
pub fn zunmql_(side: *const c_char, trans: *const c_char, m: *const c_int, n: *const c_int,
               k: *const c_int, a: *const complex_double, lda: *const c_int,
               tau: *const complex_double, c: *mut complex_double, ldc: *const c_int,
               work: *mut complex_double, lwork: *const c_int, info: *mut c_int);

pub fn sgerqf_(m: *const c_int, n: *const c_int, a: *mut c_float, lda: *const c_int,
               tau: *mut c_float, work: *mut c_float, lwork: *const c_int, info: *mut c_int);
pub fn dgerqf_(m: *const c_int, n: *const c_int, a: *mut c_double, lda: *const c_int,
               tau: *mut c_double, work: *mut c_double, lwork: *const c_int, info: *mut c_int);
pub fn cgerqf_(m: *const c_int, n: *const c_int, a: *mut complex_float, lda: *const c_int,
               tau: *mut complex_float, work: *mut complex_float, lwork: *const c_int,
               info: *mut c_int);
pub fn zgerqf_(m: *const c_int, n: *const c_int, a: *mut complex_double, lda: *const c_int,
               tau: *mut complex_double, work: *mut complex_double, lwork: *const c_int,
               info: *mut c_int);

pub fn sorgrq_(m: *const c_int, n: *const c_int, k: *const c_int, a: *mut c_float,
               lda: *const c_int, tau: *const c_float, work: *mut c_float, lwork: *const c_int,
               info: *mut c_int);
pub fn dorgrq_(m: *const c_int, n: *const c_int, k: *const c_int, a: *mut c_double,
               lda: *const c_int, tau: *const c_double, work: *mut c_double,
               lwork: *const c_int, info: *mut c_int);
pub fn cungrq_(m: *const c_int, n: *const c_int, k: *const c_int, a: *mut complex_float,
               lda: *const c_int, tau: *const complex_float, work: *mut complex_float,
               lwork: *const c_int, info: *mut c_int);
pub fn zungrq_(m: *const c_int, n: *const c_int, k: *const c_int, a: *mut complex_double,
               lda: *const c_int, tau: *const complex_double, work: *mut complex_double,
               lwork: *const c_int, info: *mut c_int);

pub fn sormrq_(side: *const c_char, trans: *const c_char, m: *const c_int, n: *const c_int,
               k: *const c_int, a: *const c_float, lda: *const c_int, tau: *const c_float,
               c: *mut c_float, ldc: *const c_int, work: *mut c_float, lwork: *const c_int,
               info: *mut c_int);
pub fn dormrq_(side: *const c_char, trans: *const c_char, m: *const c_int, n: *const c_int,
               k: *const c_int, a: *const c_double, lda: *const c_int, tau: *const c_double,
               c: *mut c_double, ldc: *const c_int, work: *mut c_double, lwork: *const c_int,
               info: *mut c_int);
pub fn cunmrq_(side: *const c_char, trans: *const c_char, m: *const c_int, n: *const c_int,
               k: *const c_int, a: *const complex_float, lda: *const c_int,
               tau: *const complex_float, c: *mut complex_float, ldc: *const c_int,
               work: *mut complex_float, lwork: *const c_int, info: *mut c_int);
pub fn zunmrq_(side: *const c_char, trans: *const c_char, m: *const c_int, n: *const c_int,
               k: *const c_int, a: *const complex_double, lda: *const c_int,
               tau: *const complex_double, c: *mut complex_double, ldc: *const c_int,
               work: *mut complex_double, lwork: *const c_int, info: *mut c_int);

pub fn stzrzf_(m: *const c_int, n: *const c_int, a: *mut c_float, lda: *const c_int,
               tau: *mut c_float, work: *mut c_float, lwork: *const c_int, info: *mut c_int);
pub fn dtzrzf_(m: *const c_int, n: *const c_int, a: *mut c_double, lda: *const c_int,
               tau: *mut c_double, work: *mut c_double, lwork: *const c_int, info: *mut c_int);
pub fn ctzrzf_(m: *const c_int, n: *const c_int, a: *mut complex_float, lda: *const c_int,
               tau: *mut complex_float, work: *mut complex_float, lwork: *const c_int,
               info: *mut c_int);
pub fn ztzrzf_(m: *const c_int, n: *const c_int, a: *mut complex_double, lda: *const c_int,
               tau: *mut complex_double, work: *mut complex_double, lwork: *const c_int,
               info: *mut c_int);

pub fn sormrz_(side: *const c_char, trans: *const c_char, m: *const c_int, n: *const c_int,
               k: *const c_int, l: *const c_int, a: *const c_float, lda: *const c_int,
               tau: *const c_float, c: *mut c_float, ldc: *const c_int, work: *mut c_float,
               lwork: *const c_int, info: *mut c_int);
pub fn dormrz_(side: *const c_char, trans: *const c_char, m: *const c_int, n: *const c_int,
               k: *const c_int, l: *const c_int, a: *const c_double, lda: *const c_int,
               tau: *const c_double, c: *mut c_double, ldc: *const c_int, work: *mut c_double,
               lwork: *const c_int, info: *mut c_int);
pub fn cunmrz_(side: *const c_char, trans: *const c_char, m: *const c_int, n: *const c_int,
               k: *const c_int, l: *const c_int, a: *const complex_float, lda: *const c_int,
               tau: *const complex_float, c: *mut complex_float, ldc: *const c_int,
               work: *mut complex_float, lwork: *const c_int, info: *mut c_int);
pub fn zunmrz_(side: *const c_char, trans: *const c_char, m: *const c_int, n: *const c_int,
               k: *const c_int, l: *const c_int, a: *const complex_double, lda: *const c_int,
               tau: *const complex_double, c: *mut complex_double, ldc: *const c_int,
               work: *mut complex_double, lwork: *const c_int, info: *mut c_int);

pub fn sggqrf_(n: *const c_int, m: *const c_int, p: *const c_int, a: *mut c_float,
               lda: *const c_int, taua: *mut c_float, b: *mut c_float, ldb: *const c_int,
               taub: *mut c_float, work: *mut c_float, lwork: *const c_int, info: *mut c_int);
pub fn dggqrf_(n: *const c_int, m: *const c_int, p: *const c_int, a: *mut c_double,
               lda: *const c_int, taua: *mut c_double, b: *mut c_double, ldb: *const c_int,
               taub: *mut c_double, work: *mut c_double, lwork: *const c_int,
               info: *mut c_int);
pub fn cggqrf_(n: *const c_int, m: *const c_int, p: *const c_int, a: *mut complex_float,
               lda: *const c_int, taua: *mut complex_float, b: *mut complex_float,
               ldb: *const c_int, taub: *mut complex_float, work: *mut complex_float,
               lwork: *const c_int, info: *mut c_int);
pub fn zggqrf_(n: *const c_int, m: *const c_int, p: *const c_int, a: *mut complex_double,
               lda: *const c_int, taua: *mut complex_double, b: *mut complex_double,
               ldb: *const c_int, taub: *mut complex_double, work: *mut complex_double,
               lwork: *const c_int, info: *mut c_int);

pub fn sggrqf_(m: *const c_int, p: *const c_int, n: *const c_int, a: *mut c_float,
               lda: *const c_int, taua: *mut c_float, b: *mut c_float, ldb: *const c_int,
               taub: *mut c_float, work: *mut c_float, lwork: *const c_int, info: *mut c_int);
pub fn dggrqf_(m: *const c_int, p: *const c_int, n: *const c_int, a: *mut c_double,
               lda: *const c_int, taua: *mut c_double, b: *mut c_double, ldb: *const c_int,
               taub: *mut c_double, work: *mut c_double, lwork: *const c_int,
               info: *mut c_int);
pub fn cggrqf_(m: *const c_int, p: *const c_int, n: *const c_int, a: *mut complex_float,
               lda: *const c_int, taua: *mut complex_float, b: *mut complex_float,
               ldb: *const c_int, taub: *mut complex_float, work: *mut complex_float,
               lwork: *const c_int, info: *mut c_int);
pub fn zggrqf_(m: *const c_int, p: *const c_int, n: *const c_int, a: *mut complex_double,
               lda: *const c_int, taua: *mut complex_double, b: *mut complex_double,
               ldb: *const c_int, taub: *mut complex_double, work: *mut complex_double,
               lwork: *const c_int, info: *mut c_int);

pub fn sgebrd_(m: *const c_int, n: *const c_int, a: *mut c_float, lda: *const c_int,
               d: *mut c_float, e: *mut c_float, tauq: *mut c_float, taup: *mut c_float,
               work: *mut c_float, lwork: *const c_int, info: *mut c_int);
pub fn dgebrd_(m: *const c_int, n: *const c_int, a: *mut c_double, lda: *const c_int,
               d: *mut c_double, e: *mut c_double, tauq: *mut c_double, taup: *mut c_double,
               work: *mut c_double, lwork: *const c_int, info: *mut c_int);
pub fn cgebrd_(m: *const c_int, n: *const c_int, a: *mut complex_float, lda: *const c_int,
               d: *mut c_float, e: *mut c_float, tauq: *mut complex_float,
               taup: *mut complex_float, work: *mut complex_float, lwork: *const c_int,
               info: *mut c_int);
pub fn zgebrd_(m: *const c_int, n: *const c_int, a: *mut complex_double, lda: *const c_int,
               d: *mut c_double, e: *mut c_double, tauq: *mut complex_double,
               taup: *mut complex_double, work: *mut complex_double, lwork: *const c_int,
               info: *mut c_int);

pub fn sgbbrd_(vect: *const c_char, m: *const c_int, n: *const c_int, ncc: *const c_int,
               kl: *const c_int, ku: *const c_int, ab: *mut c_float, ldab: *const c_int,
               d: *mut c_float, e: *mut c_float, q: *mut c_float, ldq: *const c_int,
               pt: *mut c_float, ldpt: *const c_int, c: *mut c_float, ldc: *const c_int,
               work: *mut c_float, info: *mut c_int);
pub fn dgbbrd_(vect: *const c_char, m: *const c_int, n: *const c_int, ncc: *const c_int,
               kl: *const c_int, ku: *const c_int, ab: *mut c_double, ldab: *const c_int,
               d: *mut c_double, e: *mut c_double, q: *mut c_double, ldq: *const c_int,
               pt: *mut c_double, ldpt: *const c_int, c: *mut c_double, ldc: *const c_int,
               work: *mut c_double, info: *mut c_int);
pub fn cgbbrd_(vect: *const c_char, m: *const c_int, n: *const c_int, ncc: *const c_int,
               kl: *const c_int, ku: *const c_int, ab: *mut complex_float, ldab: *const c_int,
               d: *mut c_float, e: *mut c_float, q: *mut complex_float, ldq: *const c_int,
               pt: *mut complex_float, ldpt: *const c_int, c: *mut complex_float,
               ldc: *const c_int, work: *mut complex_float, rwork: *mut c_float,
               info: *mut c_int);
pub fn zgbbrd_(vect: *const c_char, m: *const c_int, n: *const c_int, ncc: *const c_int,
               kl: *const c_int, ku: *const c_int, ab: *mut complex_double, ldab: *const c_int,
               d: *mut c_double, e: *mut c_double, q: *mut complex_double, ldq: *const c_int,
               pt: *mut complex_double, ldpt: *const c_int, c: *mut complex_double,
               ldc: *const c_int, work: *mut complex_double, rwork: *mut c_double,
               info: *mut c_int);

pub fn sorgbr_(vect: *const c_char, m: *const c_int, n: *const c_int, k: *const c_int,
               a: *mut c_float, lda: *const c_int, tau: *const c_float, work: *mut c_float,
               lwork: *const c_int, info: *mut c_int);
pub fn dorgbr_(vect: *const c_char, m: *const c_int, n: *const c_int, k: *const c_int,
               a: *mut c_double, lda: *const c_int, tau: *const c_double, work: *mut c_double,
               lwork: *const c_int, info: *mut c_int);

pub fn sormbr_(vect: *const c_char, side: *const c_char, trans: *const c_char, m: *const c_int,
               n: *const c_int, k: *const c_int, a: *const c_float, lda: *const c_int,
               tau: *const c_float, c: *mut c_float, ldc: *const c_int, work: *mut c_float,
               lwork: *const c_int, info: *mut c_int);
pub fn dormbr_(vect: *const c_char, side: *const c_char, trans: *const c_char, m: *const c_int,
               n: *const c_int, k: *const c_int, a: *const c_double, lda: *const c_int,
               tau: *const c_double, c: *mut c_double, ldc: *const c_int, work: *mut c_double,
               lwork: *const c_int, info: *mut c_int);

pub fn cungbr_(vect: *const c_char, m: *const c_int, n: *const c_int, k: *const c_int,
               a: *mut complex_float, lda: *const c_int, tau: *const complex_float,
               work: *mut complex_float, lwork: *const c_int, info: *mut c_int);
pub fn zungbr_(vect: *const c_char, m: *const c_int, n: *const c_int, k: *const c_int,
               a: *mut complex_double, lda: *const c_int, tau: *const complex_double,
               work: *mut complex_double, lwork: *const c_int, info: *mut c_int);

pub fn cunmbr_(vect: *const c_char, side: *const c_char, trans: *const c_char, m: *const c_int,
               n: *const c_int, k: *const c_int, a: *const complex_float, lda: *const c_int,
               tau: *const complex_float, c: *mut complex_float, ldc: *const c_int,
               work: *mut complex_float, lwork: *const c_int, info: *mut c_int);
pub fn zunmbr_(vect: *const c_char, side: *const c_char, trans: *const c_char, m: *const c_int,
               n: *const c_int, k: *const c_int, a: *const complex_double, lda: *const c_int,
               tau: *const complex_double, c: *mut complex_double, ldc: *const c_int,
               work: *mut complex_double, lwork: *const c_int, info: *mut c_int);

pub fn sbdsqr_(uplo: *const c_char, n: *const c_int, ncvt: *const c_int, nru: *const c_int,
               ncc: *const c_int, d: *mut c_float, e: *mut c_float, vt: *mut c_float,
               ldvt: *const c_int, u: *mut c_float, ldu: *const c_int, c: *mut c_float,
               ldc: *const c_int, work: *mut c_float, info: *mut c_int);
pub fn dbdsqr_(uplo: *const c_char, n: *const c_int, ncvt: *const c_int, nru: *const c_int,
               ncc: *const c_int, d: *mut c_double, e: *mut c_double, vt: *mut c_double,
               ldvt: *const c_int, u: *mut c_double, ldu: *const c_int, c: *mut c_double,
               ldc: *const c_int, work: *mut c_double, info: *mut c_int);
pub fn cbdsqr_(uplo: *const c_char, n: *const c_int, ncvt: *const c_int, nru: *const c_int,
               ncc: *const c_int, d: *mut c_float, e: *mut c_float, vt: *mut complex_float,
               ldvt: *const c_int, u: *mut complex_float, ldu: *const c_int,
               c: *mut complex_float, ldc: *const c_int, rwork: *mut c_float,
               info: *mut c_int);
pub fn zbdsqr_(uplo: *const c_char, n: *const c_int, ncvt: *const c_int, nru: *const c_int,
               ncc: *const c_int, d: *mut c_double, e: *mut c_double, vt: *mut complex_double,
               ldvt: *const c_int, u: *mut complex_double, ldu: *const c_int,
               c: *mut complex_double, ldc: *const c_int, rwork: *mut c_double,
               info: *mut c_int);

pub fn sbdsdc_(uplo: *const c_char, compq: *const c_char, n: *const c_int, d: *mut c_float,
               e: *mut c_float, u: *mut c_float, ldu: *const c_int, vt: *mut c_float,
               ldvt: *const c_int, q: *mut c_float, iq: *mut c_int, work: *mut c_float,
               iwork: *mut c_int, info: *mut c_int);
pub fn dbdsdc_(uplo: *const c_char, compq: *const c_char, n: *const c_int, d: *mut c_double,
               e: *mut c_double, u: *mut c_double, ldu: *const c_int, vt: *mut c_double,
               ldvt: *const c_int, q: *mut c_double, iq: *mut c_int, work: *mut c_double,
               iwork: *mut c_int, info: *mut c_int);

pub fn ssytrd_(uplo: *const c_char, n: *const c_int, a: *mut c_float, lda: *const c_int,
               d: *mut c_float, e: *mut c_float, tau: *mut c_float, work: *mut c_float,
               lwork: *const c_int, info: *mut c_int);
pub fn dsytrd_(uplo: *const c_char, n: *const c_int, a: *mut c_double, lda: *const c_int,
               d: *mut c_double, e: *mut c_double, tau: *mut c_double, work: *mut c_double,
               lwork: *const c_int, info: *mut c_int);

pub fn sorgtr_(uplo: *const c_char, n: *const c_int, a: *mut c_float, lda: *const c_int,
               tau: *const c_float, work: *mut c_float, lwork: *const c_int, info: *mut c_int);
pub fn dorgtr_(uplo: *const c_char, n: *const c_int, a: *mut c_double, lda: *const c_int,
               tau: *const c_double, work: *mut c_double, lwork: *const c_int,
               info: *mut c_int);

pub fn sormtr_(side: *const c_char, uplo: *const c_char, trans: *const c_char, m: *const c_int,
               n: *const c_int, a: *const c_float, lda: *const c_int, tau: *const c_float,
               c: *mut c_float, ldc: *const c_int, work: *mut c_float, lwork: *const c_int,
               info: *mut c_int);
pub fn dormtr_(side: *const c_char, uplo: *const c_char, trans: *const c_char, m: *const c_int,
               n: *const c_int, a: *const c_double, lda: *const c_int, tau: *const c_double,
               c: *mut c_double, ldc: *const c_int, work: *mut c_double, lwork: *const c_int,
               info: *mut c_int);

pub fn chetrd_(uplo: *const c_char, n: *const c_int, a: *mut complex_float, lda: *const c_int,
               d: *mut c_float, e: *mut c_float, tau: *mut complex_float,
               work: *mut complex_float, lwork: *const c_int, info: *mut c_int);
pub fn zhetrd_(uplo: *const c_char, n: *const c_int, a: *mut complex_double, lda: *const c_int,
               d: *mut c_double, e: *mut c_double, tau: *mut complex_double,
               work: *mut complex_double, lwork: *const c_int, info: *mut c_int);

pub fn cungtr_(uplo: *const c_char, n: *const c_int, a: *mut complex_float, lda: *const c_int,
               tau: *const complex_float, work: *mut complex_float, lwork: *const c_int,
               info: *mut c_int);
pub fn zungtr_(uplo: *const c_char, n: *const c_int, a: *mut complex_double, lda: *const c_int,
               tau: *const complex_double, work: *mut complex_double, lwork: *const c_int,
               info: *mut c_int);

pub fn cunmtr_(side: *const c_char, uplo: *const c_char, trans: *const c_char, m: *const c_int,
               n: *const c_int, a: *const complex_float, lda: *const c_int,
               tau: *const complex_float, c: *mut complex_float, ldc: *const c_int,
               work: *mut complex_float, lwork: *const c_int, info: *mut c_int);
pub fn zunmtr_(side: *const c_char, uplo: *const c_char, trans: *const c_char, m: *const c_int,
               n: *const c_int, a: *const complex_double, lda: *const c_int,
               tau: *const complex_double, c: *mut complex_double, ldc: *const c_int,
               work: *mut complex_double, lwork: *const c_int, info: *mut c_int);

pub fn ssptrd_(uplo: *const c_char, n: *const c_int, ap: *mut c_float, d: *mut c_float,
               e: *mut c_float, tau: *mut c_float, info: *mut c_int);
pub fn dsptrd_(uplo: *const c_char, n: *const c_int, ap: *mut c_double, d: *mut c_double,
               e: *mut c_double, tau: *mut c_double, info: *mut c_int);

pub fn sopgtr_(uplo: *const c_char, n: *const c_int, ap: *const c_float, tau: *const c_float,
               q: *mut c_float, ldq: *const c_int, work: *mut c_float, info: *mut c_int);
pub fn dopgtr_(uplo: *const c_char, n: *const c_int, ap: *const c_double, tau: *const c_double,
               q: *mut c_double, ldq: *const c_int, work: *mut c_double, info: *mut c_int);

pub fn sopmtr_(side: *const c_char, uplo: *const c_char, trans: *const c_char, m: *const c_int,
               n: *const c_int, ap: *const c_float, tau: *const c_float, c: *mut c_float,
               ldc: *const c_int, work: *mut c_float, info: *mut c_int);
pub fn dopmtr_(side: *const c_char, uplo: *const c_char, trans: *const c_char, m: *const c_int,
               n: *const c_int, ap: *const c_double, tau: *const c_double, c: *mut c_double,
               ldc: *const c_int, work: *mut c_double, info: *mut c_int);

pub fn chptrd_(uplo: *const c_char, n: *const c_int, ap: *mut complex_float, d: *mut c_float,
               e: *mut c_float, tau: *mut complex_float, info: *mut c_int);
pub fn zhptrd_(uplo: *const c_char, n: *const c_int, ap: *mut complex_double, d: *mut c_double,
               e: *mut c_double, tau: *mut complex_double, info: *mut c_int);

pub fn cupgtr_(uplo: *const c_char, n: *const c_int, ap: *const complex_float,
               tau: *const complex_float, q: *mut complex_float, ldq: *const c_int,
               work: *mut complex_float, info: *mut c_int);
pub fn zupgtr_(uplo: *const c_char, n: *const c_int, ap: *const complex_double,
               tau: *const complex_double, q: *mut complex_double, ldq: *const c_int,
               work: *mut complex_double, info: *mut c_int);

pub fn cupmtr_(side: *const c_char, uplo: *const c_char, trans: *const c_char, m: *const c_int,
               n: *const c_int, ap: *const complex_float, tau: *const complex_float,
               c: *mut complex_float, ldc: *const c_int, work: *mut complex_float,
               info: *mut c_int);
pub fn zupmtr_(side: *const c_char, uplo: *const c_char, trans: *const c_char, m: *const c_int,
               n: *const c_int, ap: *const complex_double, tau: *const complex_double,
               c: *mut complex_double, ldc: *const c_int, work: *mut complex_double,
               info: *mut c_int);

pub fn ssbtrd_(vect: *const c_char, uplo: *const c_char, n: *const c_int, kd: *const c_int,
               ab: *mut c_float, ldab: *const c_int, d: *mut c_float, e: *mut c_float,
               q: *mut c_float, ldq: *const c_int, work: *mut c_float, info: *mut c_int);
pub fn dsbtrd_(vect: *const c_char, uplo: *const c_char, n: *const c_int, kd: *const c_int,
               ab: *mut c_double, ldab: *const c_int, d: *mut c_double, e: *mut c_double,
               q: *mut c_double, ldq: *const c_int, work: *mut c_double, info: *mut c_int);
pub fn chbtrd_(vect: *const c_char, uplo: *const c_char, n: *const c_int, kd: *const c_int,
               ab: *mut complex_float, ldab: *const c_int, d: *mut c_float, e: *mut c_float,
               q: *mut complex_float, ldq: *const c_int, work: *mut complex_float,
               info: *mut c_int);
pub fn zhbtrd_(vect: *const c_char, uplo: *const c_char, n: *const c_int, kd: *const c_int,
               ab: *mut complex_double, ldab: *const c_int, d: *mut c_double, e: *mut c_double,
               q: *mut complex_double, ldq: *const c_int, work: *mut complex_double,
               info: *mut c_int);

pub fn ssterf_(n: *const c_int, d: *mut c_float, e: *mut c_float, info: *mut c_int);
pub fn dsterf_(n: *const c_int, d: *mut c_double, e: *mut c_double, info: *mut c_int);

pub fn ssteqr_(compz: *const c_char, n: *const c_int, d: *mut c_float, e: *mut c_float,
               z: *mut c_float, ldz: *const c_int, work: *mut c_float, info: *mut c_int);
pub fn dsteqr_(compz: *const c_char, n: *const c_int, d: *mut c_double, e: *mut c_double,
               z: *mut c_double, ldz: *const c_int, work: *mut c_double, info: *mut c_int);
pub fn csteqr_(compz: *const c_char, n: *const c_int, d: *mut c_float, e: *mut c_float,
               z: *mut complex_float, ldz: *const c_int, work: *mut c_float, info: *mut c_int);
pub fn zsteqr_(compz: *const c_char, n: *const c_int, d: *mut c_double, e: *mut c_double,
               z: *mut complex_double, ldz: *const c_int, work: *mut c_double,
               info: *mut c_int);

pub fn sstemr_(jobz: *const c_char, range: *const c_char, n: *const c_int, d: *mut c_float,
               e: *mut c_float, vl: *const c_float, vu: *const c_float, il: *const c_int,
               iu: *const c_int, m: *mut c_int, w: *mut c_float, z: *mut c_float,
               ldz: *const c_int, nzc: *const c_int, isuppz: *mut c_int, tryrac: *mut c_int,
               work: *mut c_float, lwork: *const c_int, iwork: *mut c_int,
               liwork: *const c_int, info: *mut c_int);
pub fn dstemr_(jobz: *const c_char, range: *const c_char, n: *const c_int, d: *mut c_double,
               e: *mut c_double, vl: *const c_double, vu: *const c_double, il: *const c_int,
               iu: *const c_int, m: *mut c_int, w: *mut c_double, z: *mut c_double,
               ldz: *const c_int, nzc: *const c_int, isuppz: *mut c_int, tryrac: *mut c_int,
               work: *mut c_double, lwork: *const c_int, iwork: *mut c_int,
               liwork: *const c_int, info: *mut c_int);
pub fn cstemr_(jobz: *const c_char, range: *const c_char, n: *const c_int, d: *mut c_float,
               e: *mut c_float, vl: *const c_float, vu: *const c_float, il: *const c_int,
               iu: *const c_int, m: *mut c_int, w: *mut c_float, z: *mut complex_float,
               ldz: *const c_int, nzc: *const c_int, isuppz: *mut c_int, tryrac: *mut c_int,
               work: *mut c_float, lwork: *const c_int, iwork: *mut c_int,
               liwork: *const c_int, info: *mut c_int);
pub fn zstemr_(jobz: *const c_char, range: *const c_char, n: *const c_int, d: *mut c_double,
               e: *mut c_double, vl: *const c_double, vu: *const c_double, il: *const c_int,
               iu: *const c_int, m: *mut c_int, w: *mut c_double, z: *mut complex_double,
               ldz: *const c_int, nzc: *const c_int, isuppz: *mut c_int, tryrac: *mut c_int,
               work: *mut c_double, lwork: *const c_int, iwork: *mut c_int,
               liwork: *const c_int, info: *mut c_int);

pub fn sstedc_(compz: *const c_char, n: *const c_int, d: *mut c_float, e: *mut c_float,
               z: *mut c_float, ldz: *const c_int, work: *mut c_float, lwork: *const c_int,
               iwork: *mut c_int, liwork: *const c_int, info: *mut c_int);
pub fn dstedc_(compz: *const c_char, n: *const c_int, d: *mut c_double, e: *mut c_double,
               z: *mut c_double, ldz: *const c_int, work: *mut c_double, lwork: *const c_int,
               iwork: *mut c_int, liwork: *const c_int, info: *mut c_int);
pub fn cstedc_(compz: *const c_char, n: *const c_int, d: *mut c_float, e: *mut c_float,
               z: *mut complex_float, ldz: *const c_int, work: *mut complex_float,
               lwork: *const c_int, rwork: *mut c_float, lrwork: *const c_int,
               iwork: *mut c_int, liwork: *const c_int, info: *mut c_int);
pub fn zstedc_(compz: *const c_char, n: *const c_int, d: *mut c_double, e: *mut c_double,
               z: *mut complex_double, ldz: *const c_int, work: *mut complex_double,
               lwork: *const c_int, rwork: *mut c_double, lrwork: *const c_int,
               iwork: *mut c_int, liwork: *const c_int, info: *mut c_int);

pub fn sstegr_(jobz: *const c_char, range: *const c_char, n: *const c_int, d: *mut c_float,
               e: *mut c_float, vl: *const c_float, vu: *const c_float, il: *const c_int,
               iu: *const c_int, abstol: *const c_float, m: *mut c_int, w: *mut c_float,
               z: *mut c_float, ldz: *const c_int, isuppz: *mut c_int, work: *mut c_float,
               lwork: *const c_int, iwork: *mut c_int, liwork: *const c_int, info: *mut c_int);
pub fn dstegr_(jobz: *const c_char, range: *const c_char, n: *const c_int, d: *mut c_double,
               e: *mut c_double, vl: *const c_double, vu: *const c_double, il: *const c_int,
               iu: *const c_int, abstol: *const c_double, m: *mut c_int, w: *mut c_double,
               z: *mut c_double, ldz: *const c_int, isuppz: *mut c_int, work: *mut c_double,
               lwork: *const c_int, iwork: *mut c_int, liwork: *const c_int, info: *mut c_int);
pub fn cstegr_(jobz: *const c_char, range: *const c_char, n: *const c_int, d: *mut c_float,
               e: *mut c_float, vl: *const c_float, vu: *const c_float, il: *const c_int,
               iu: *const c_int, abstol: *const c_float, m: *mut c_int, w: *mut c_float,
               z: *mut complex_float, ldz: *const c_int, isuppz: *mut c_int,
               work: *mut c_float, lwork: *const c_int, iwork: *mut c_int,
               liwork: *const c_int, info: *mut c_int);
pub fn zstegr_(jobz: *const c_char, range: *const c_char, n: *const c_int, d: *mut c_double,
               e: *mut c_double, vl: *const c_double, vu: *const c_double, il: *const c_int,
               iu: *const c_int, abstol: *const c_double, m: *mut c_int, w: *mut c_double,
               z: *mut complex_double, ldz: *const c_int, isuppz: *mut c_int,
               work: *mut c_double, lwork: *const c_int, iwork: *mut c_int,
               liwork: *const c_int, info: *mut c_int);

pub fn spteqr_(compz: *const c_char, n: *const c_int, d: *mut c_float, e: *mut c_float,
               z: *mut c_float, ldz: *const c_int, work: *mut c_float, info: *mut c_int);
pub fn dpteqr_(compz: *const c_char, n: *const c_int, d: *mut c_double, e: *mut c_double,
               z: *mut c_double, ldz: *const c_int, work: *mut c_double, info: *mut c_int);
pub fn cpteqr_(compz: *const c_char, n: *const c_int, d: *mut c_float, e: *mut c_float,
               z: *mut complex_float, ldz: *const c_int, work: *mut c_float, info: *mut c_int);
pub fn zpteqr_(compz: *const c_char, n: *const c_int, d: *mut c_double, e: *mut c_double,
               z: *mut complex_double, ldz: *const c_int, work: *mut c_double,
               info: *mut c_int);

pub fn sstebz_(range: *const c_char, order: *const c_char, n: *const c_int, vl: *const c_float,
               vu: *const c_float, il: *const c_int, iu: *const c_int, abstol: *const c_float,
               d: *const c_float, e: *const c_float, m: *mut c_int, nsplit: *mut c_int,
               w: *mut c_float, iblock: *mut c_int, isplit: *mut c_int, work: *mut c_float,
               iwork: *mut c_int, info: *mut c_int);
pub fn dstebz_(range: *const c_char, order: *const c_char, n: *const c_int,
               vl: *const c_double, vu: *const c_double, il: *const c_int, iu: *const c_int,
               abstol: *const c_double, d: *const c_double, e: *const c_double, m: *mut c_int,
               nsplit: *mut c_int, w: *mut c_double, iblock: *mut c_int, isplit: *mut c_int,
               work: *mut c_double, iwork: *mut c_int, info: *mut c_int);

pub fn sstein_(n: *const c_int, d: *const c_float, e: *const c_float, m: *const c_int,
               w: *const c_float, iblock: *const c_int, isplit: *const c_int, z: *mut c_float,
               ldz: *const c_int, work: *mut c_float, iwork: *mut c_int, ifail: *mut c_int,
               info: *mut c_int);
pub fn dstein_(n: *const c_int, d: *const c_double, e: *const c_double, m: *const c_int,
               w: *const c_double, iblock: *const c_int, isplit: *const c_int,
               z: *mut c_double, ldz: *const c_int, work: *mut c_double, iwork: *mut c_int,
               ifail: *mut c_int, info: *mut c_int);
pub fn cstein_(n: *const c_int, d: *const c_float, e: *const c_float, m: *const c_int,
               w: *const c_float, iblock: *const c_int, isplit: *const c_int,
               z: *mut complex_float, ldz: *const c_int, work: *mut c_float, iwork: *mut c_int,
               ifail: *mut c_int, info: *mut c_int);
pub fn zstein_(n: *const c_int, d: *const c_double, e: *const c_double, m: *const c_int,
               w: *const c_double, iblock: *const c_int, isplit: *const c_int,
               z: *mut complex_double, ldz: *const c_int, work: *mut c_double,
               iwork: *mut c_int, ifail: *mut c_int, info: *mut c_int);

pub fn sdisna_(job: *const c_char, m: *const c_int, n: *const c_int, d: *const c_float,
               sep: *mut c_float, info: *mut c_int);
pub fn ddisna_(job: *const c_char, m: *const c_int, n: *const c_int, d: *const c_double,
               sep: *mut c_double, info: *mut c_int);

pub fn ssygst_(itype: *const c_int, uplo: *const c_char, n: *const c_int, a: *mut c_float,
               lda: *const c_int, b: *const c_float, ldb: *const c_int, info: *mut c_int);
pub fn dsygst_(itype: *const c_int, uplo: *const c_char, n: *const c_int, a: *mut c_double,
               lda: *const c_int, b: *const c_double, ldb: *const c_int, info: *mut c_int);
pub fn chegst_(itype: *const c_int, uplo: *const c_char, n: *const c_int,
               a: *mut complex_float, lda: *const c_int, b: *const complex_float,
               ldb: *const c_int, info: *mut c_int);
pub fn zhegst_(itype: *const c_int, uplo: *const c_char, n: *const c_int,
               a: *mut complex_double, lda: *const c_int, b: *const complex_double,
               ldb: *const c_int, info: *mut c_int);

pub fn sspgst_(itype: *const c_int, uplo: *const c_char, n: *const c_int, ap: *mut c_float,
               bp: *const c_float, info: *mut c_int);
pub fn dspgst_(itype: *const c_int, uplo: *const c_char, n: *const c_int, ap: *mut c_double,
               bp: *const c_double, info: *mut c_int);
pub fn chpgst_(itype: *const c_int, uplo: *const c_char, n: *const c_int,
               ap: *mut complex_float, bp: *const complex_float, info: *mut c_int);
pub fn zhpgst_(itype: *const c_int, uplo: *const c_char, n: *const c_int,
               ap: *mut complex_double, bp: *const complex_double, info: *mut c_int);

pub fn ssbgst_(vect: *const c_char, uplo: *const c_char, n: *const c_int, ka: *const c_int,
               kb: *const c_int, ab: *mut c_float, ldab: *const c_int, bb: *const c_float,
               ldbb: *const c_int, x: *mut c_float, ldx: *const c_int, work: *mut c_float,
               info: *mut c_int);
pub fn dsbgst_(vect: *const c_char, uplo: *const c_char, n: *const c_int, ka: *const c_int,
               kb: *const c_int, ab: *mut c_double, ldab: *const c_int, bb: *const c_double,
               ldbb: *const c_int, x: *mut c_double, ldx: *const c_int, work: *mut c_double,
               info: *mut c_int);
pub fn chbgst_(vect: *const c_char, uplo: *const c_char, n: *const c_int, ka: *const c_int,
               kb: *const c_int, ab: *mut complex_float, ldab: *const c_int,
               bb: *const complex_float, ldbb: *const c_int, x: *mut complex_float,
               ldx: *const c_int, work: *mut complex_float, rwork: *mut c_float,
               info: *mut c_int);
pub fn zhbgst_(vect: *const c_char, uplo: *const c_char, n: *const c_int, ka: *const c_int,
               kb: *const c_int, ab: *mut complex_double, ldab: *const c_int,
               bb: *const complex_double, ldbb: *const c_int, x: *mut complex_double,
               ldx: *const c_int, work: *mut complex_double, rwork: *mut c_double,
               info: *mut c_int);

pub fn spbstf_(uplo: *const c_char, n: *const c_int, kd: *const c_int, ab: *mut c_float,
               ldab: *const c_int, info: *mut c_int);
pub fn dpbstf_(uplo: *const c_char, n: *const c_int, kd: *const c_int, ab: *mut c_double,
               ldab: *const c_int, info: *mut c_int);
pub fn cpbstf_(uplo: *const c_char, n: *const c_int, kd: *const c_int, ab: *mut complex_float,
               ldab: *const c_int, info: *mut c_int);
pub fn zpbstf_(uplo: *const c_char, n: *const c_int, kd: *const c_int, ab: *mut complex_double,
               ldab: *const c_int, info: *mut c_int);

pub fn sgehrd_(n: *const c_int, ilo: *const c_int, ihi: *const c_int, a: *mut c_float,
               lda: *const c_int, tau: *mut c_float, work: *mut c_float, lwork: *const c_int,
               info: *mut c_int);
pub fn dgehrd_(n: *const c_int, ilo: *const c_int, ihi: *const c_int, a: *mut c_double,
               lda: *const c_int, tau: *mut c_double, work: *mut c_double, lwork: *const c_int,
               info: *mut c_int);
pub fn cgehrd_(n: *const c_int, ilo: *const c_int, ihi: *const c_int, a: *mut complex_float,
               lda: *const c_int, tau: *mut complex_float, work: *mut complex_float,
               lwork: *const c_int, info: *mut c_int);
pub fn zgehrd_(n: *const c_int, ilo: *const c_int, ihi: *const c_int, a: *mut complex_double,
               lda: *const c_int, tau: *mut complex_double, work: *mut complex_double,
               lwork: *const c_int, info: *mut c_int);

pub fn sorghr_(n: *const c_int, ilo: *const c_int, ihi: *const c_int, a: *mut c_float,
               lda: *const c_int, tau: *const c_float, work: *mut c_float, lwork: *const c_int,
               info: *mut c_int);
pub fn dorghr_(n: *const c_int, ilo: *const c_int, ihi: *const c_int, a: *mut c_double,
               lda: *const c_int, tau: *const c_double, work: *mut c_double,
               lwork: *const c_int, info: *mut c_int);

pub fn sormhr_(side: *const c_char, trans: *const c_char, m: *const c_int, n: *const c_int,
               ilo: *const c_int, ihi: *const c_int, a: *const c_float, lda: *const c_int,
               tau: *const c_float, c: *mut c_float, ldc: *const c_int, work: *mut c_float,
               lwork: *const c_int, info: *mut c_int);
pub fn dormhr_(side: *const c_char, trans: *const c_char, m: *const c_int, n: *const c_int,
               ilo: *const c_int, ihi: *const c_int, a: *const c_double, lda: *const c_int,
               tau: *const c_double, c: *mut c_double, ldc: *const c_int, work: *mut c_double,
               lwork: *const c_int, info: *mut c_int);

pub fn cunghr_(n: *const c_int, ilo: *const c_int, ihi: *const c_int, a: *mut complex_float,
               lda: *const c_int, tau: *const complex_float, work: *mut complex_float,
               lwork: *const c_int, info: *mut c_int);
pub fn zunghr_(n: *const c_int, ilo: *const c_int, ihi: *const c_int, a: *mut complex_double,
               lda: *const c_int, tau: *const complex_double, work: *mut complex_double,
               lwork: *const c_int, info: *mut c_int);

pub fn cunmhr_(side: *const c_char, trans: *const c_char, m: *const c_int, n: *const c_int,
               ilo: *const c_int, ihi: *const c_int, a: *const complex_float,
               lda: *const c_int, tau: *const complex_float, c: *mut complex_float,
               ldc: *const c_int, work: *mut complex_float, lwork: *const c_int,
               info: *mut c_int);
pub fn zunmhr_(side: *const c_char, trans: *const c_char, m: *const c_int, n: *const c_int,
               ilo: *const c_int, ihi: *const c_int, a: *const complex_double,
               lda: *const c_int, tau: *const complex_double, c: *mut complex_double,
               ldc: *const c_int, work: *mut complex_double, lwork: *const c_int,
               info: *mut c_int);

pub fn sgebal_(job: *const c_char, n: *const c_int, a: *mut c_float, lda: *const c_int,
               ilo: *mut c_int, ihi: *mut c_int, scale: *mut c_float, info: *mut c_int);
pub fn dgebal_(job: *const c_char, n: *const c_int, a: *mut c_double, lda: *const c_int,
               ilo: *mut c_int, ihi: *mut c_int, scale: *mut c_double, info: *mut c_int);
pub fn cgebal_(job: *const c_char, n: *const c_int, a: *mut complex_float, lda: *const c_int,
               ilo: *mut c_int, ihi: *mut c_int, scale: *mut c_float, info: *mut c_int);
pub fn zgebal_(job: *const c_char, n: *const c_int, a: *mut complex_double, lda: *const c_int,
               ilo: *mut c_int, ihi: *mut c_int, scale: *mut c_double, info: *mut c_int);

pub fn sgebak_(job: *const c_char, side: *const c_char, n: *const c_int, ilo: *const c_int,
               ihi: *const c_int, scale: *const c_float, m: *const c_int, v: *mut c_float,
               ldv: *const c_int, info: *mut c_int);
pub fn dgebak_(job: *const c_char, side: *const c_char, n: *const c_int, ilo: *const c_int,
               ihi: *const c_int, scale: *const c_double, m: *const c_int, v: *mut c_double,
               ldv: *const c_int, info: *mut c_int);
pub fn cgebak_(job: *const c_char, side: *const c_char, n: *const c_int, ilo: *const c_int,
               ihi: *const c_int, scale: *const c_float, m: *const c_int,
               v: *mut complex_float, ldv: *const c_int, info: *mut c_int);
pub fn zgebak_(job: *const c_char, side: *const c_char, n: *const c_int, ilo: *const c_int,
               ihi: *const c_int, scale: *const c_double, m: *const c_int,
               v: *mut complex_double, ldv: *const c_int, info: *mut c_int);

pub fn shseqr_(job: *const c_char, compz: *const c_char, n: *const c_int, ilo: *const c_int,
               ihi: *const c_int, h: *mut c_float, ldh: *const c_int, wr: *mut c_float,
               wi: *mut c_float, z: *mut c_float, ldz: *const c_int, work: *mut c_float,
               lwork: *const c_int, info: *mut c_int);
pub fn dhseqr_(job: *const c_char, compz: *const c_char, n: *const c_int, ilo: *const c_int,
               ihi: *const c_int, h: *mut c_double, ldh: *const c_int, wr: *mut c_double,
               wi: *mut c_double, z: *mut c_double, ldz: *const c_int, work: *mut c_double,
               lwork: *const c_int, info: *mut c_int);
pub fn chseqr_(job: *const c_char, compz: *const c_char, n: *const c_int, ilo: *const c_int,
               ihi: *const c_int, h: *mut complex_float, ldh: *const c_int,
               w: *mut complex_float, z: *mut complex_float, ldz: *const c_int,
               work: *mut complex_float, lwork: *const c_int, info: *mut c_int);
pub fn zhseqr_(job: *const c_char, compz: *const c_char, n: *const c_int, ilo: *const c_int,
               ihi: *const c_int, h: *mut complex_double, ldh: *const c_int,
               w: *mut complex_double, z: *mut complex_double, ldz: *const c_int,
               work: *mut complex_double, lwork: *const c_int, info: *mut c_int);

pub fn shsein_(side: *const c_char, eigsrc: *const c_char, initv: *const c_char,
               select: *mut c_int, n: *const c_int, h: *const c_float, ldh: *const c_int,
               wr: *mut c_float, wi: *const c_float, vl: *mut c_float, ldvl: *const c_int,
               vr: *mut c_float, ldvr: *const c_int, mm: *const c_int, m: *mut c_int,
               work: *mut c_float, ifaill: *mut c_int, ifailr: *mut c_int, info: *mut c_int);
pub fn dhsein_(side: *const c_char, eigsrc: *const c_char, initv: *const c_char,
               select: *mut c_int, n: *const c_int, h: *const c_double, ldh: *const c_int,
               wr: *mut c_double, wi: *const c_double, vl: *mut c_double, ldvl: *const c_int,
               vr: *mut c_double, ldvr: *const c_int, mm: *const c_int, m: *mut c_int,
               work: *mut c_double, ifaill: *mut c_int, ifailr: *mut c_int, info: *mut c_int);
pub fn chsein_(side: *const c_char, eigsrc: *const c_char, initv: *const c_char,
               select: *const c_int, n: *const c_int, h: *const complex_float,
               ldh: *const c_int, w: *mut complex_float, vl: *mut complex_float,
               ldvl: *const c_int, vr: *mut complex_float, ldvr: *const c_int,
               mm: *const c_int, m: *mut c_int, work: *mut complex_float, rwork: *mut c_float,
               ifaill: *mut c_int, ifailr: *mut c_int, info: *mut c_int);
pub fn zhsein_(side: *const c_char, eigsrc: *const c_char, initv: *const c_char,
               select: *const c_int, n: *const c_int, h: *const complex_double,
               ldh: *const c_int, w: *mut complex_double, vl: *mut complex_double,
               ldvl: *const c_int, vr: *mut complex_double, ldvr: *const c_int,
               mm: *const c_int, m: *mut c_int, work: *mut complex_double,
               rwork: *mut c_double, ifaill: *mut c_int, ifailr: *mut c_int, info: *mut c_int);

pub fn strevc_(side: *const c_char, howmny: *const c_char, select: *mut c_int, n: *const c_int,
               t: *const c_float, ldt: *const c_int, vl: *mut c_float, ldvl: *const c_int,
               vr: *mut c_float, ldvr: *const c_int, mm: *const c_int, m: *mut c_int,
               work: *mut c_float, info: *mut c_int);
pub fn dtrevc_(side: *const c_char, howmny: *const c_char, select: *mut c_int, n: *const c_int,
               t: *const c_double, ldt: *const c_int, vl: *mut c_double, ldvl: *const c_int,
               vr: *mut c_double, ldvr: *const c_int, mm: *const c_int, m: *mut c_int,
               work: *mut c_double, info: *mut c_int);
pub fn ctrevc_(side: *const c_char, howmny: *const c_char, select: *const c_int,
               n: *const c_int, t: *mut complex_float, ldt: *const c_int,
               vl: *mut complex_float, ldvl: *const c_int, vr: *mut complex_float,
               ldvr: *const c_int, mm: *const c_int, m: *mut c_int, work: *mut complex_float,
               rwork: *mut c_float, info: *mut c_int);
pub fn ztrevc_(side: *const c_char, howmny: *const c_char, select: *const c_int,
               n: *const c_int, t: *mut complex_double, ldt: *const c_int,
               vl: *mut complex_double, ldvl: *const c_int, vr: *mut complex_double,
               ldvr: *const c_int, mm: *const c_int, m: *mut c_int, work: *mut complex_double,
               rwork: *mut c_double, info: *mut c_int);

pub fn strsna_(job: *const c_char, howmny: *const c_char, select: *const c_int,
               n: *const c_int, t: *const c_float, ldt: *const c_int, vl: *const c_float,
               ldvl: *const c_int, vr: *const c_float, ldvr: *const c_int, s: *mut c_float,
               sep: *mut c_float, mm: *const c_int, m: *mut c_int, work: *mut c_float,
               ldwork: *const c_int, iwork: *mut c_int, info: *mut c_int);
pub fn dtrsna_(job: *const c_char, howmny: *const c_char, select: *const c_int,
               n: *const c_int, t: *const c_double, ldt: *const c_int, vl: *const c_double,
               ldvl: *const c_int, vr: *const c_double, ldvr: *const c_int, s: *mut c_double,
               sep: *mut c_double, mm: *const c_int, m: *mut c_int, work: *mut c_double,
               ldwork: *const c_int, iwork: *mut c_int, info: *mut c_int);
pub fn ctrsna_(job: *const c_char, howmny: *const c_char, select: *const c_int,
               n: *const c_int, t: *const complex_float, ldt: *const c_int,
               vl: *const complex_float, ldvl: *const c_int, vr: *const complex_float,
               ldvr: *const c_int, s: *mut c_float, sep: *mut c_float, mm: *const c_int,
               m: *mut c_int, work: *mut complex_float, ldwork: *const c_int,
               rwork: *mut c_float, info: *mut c_int);
pub fn ztrsna_(job: *const c_char, howmny: *const c_char, select: *const c_int,
               n: *const c_int, t: *const complex_double, ldt: *const c_int,
               vl: *const complex_double, ldvl: *const c_int, vr: *const complex_double,
               ldvr: *const c_int, s: *mut c_double, sep: *mut c_double, mm: *const c_int,
               m: *mut c_int, work: *mut complex_double, ldwork: *const c_int,
               rwork: *mut c_double, info: *mut c_int);

pub fn strexc_(compq: *const c_char, n: *const c_int, t: *mut c_float, ldt: *const c_int,
               q: *mut c_float, ldq: *const c_int, ifst: *mut c_int, ilst: *mut c_int,
               work: *mut c_float, info: *mut c_int);
pub fn dtrexc_(compq: *const c_char, n: *const c_int, t: *mut c_double, ldt: *const c_int,
               q: *mut c_double, ldq: *const c_int, ifst: *mut c_int, ilst: *mut c_int,
               work: *mut c_double, info: *mut c_int);
pub fn ctrexc_(compq: *const c_char, n: *const c_int, t: *mut complex_float, ldt: *const c_int,
               q: *mut complex_float, ldq: *const c_int, ifst: *const c_int,
               ilst: *const c_int, info: *mut c_int);
pub fn ztrexc_(compq: *const c_char, n: *const c_int, t: *mut complex_double,
               ldt: *const c_int, q: *mut complex_double, ldq: *const c_int,
               ifst: *const c_int, ilst: *const c_int, info: *mut c_int);

pub fn strsen_(job: *const c_char, compq: *const c_char, select: *const c_int, n: *const c_int,
               t: *mut c_float, ldt: *const c_int, q: *mut c_float, ldq: *const c_int,
               wr: *mut c_float, wi: *mut c_float, m: *mut c_int, s: *mut c_float,
               sep: *mut c_float, work: *mut c_float, lwork: *const c_int, iwork: *mut c_int,
               liwork: *const c_int, info: *mut c_int);
pub fn dtrsen_(job: *const c_char, compq: *const c_char, select: *const c_int, n: *const c_int,
               t: *mut c_double, ldt: *const c_int, q: *mut c_double, ldq: *const c_int,
               wr: *mut c_double, wi: *mut c_double, m: *mut c_int, s: *mut c_double,
               sep: *mut c_double, work: *mut c_double, lwork: *const c_int, iwork: *mut c_int,
               liwork: *const c_int, info: *mut c_int);
pub fn ctrsen_(job: *const c_char, compq: *const c_char, select: *const c_int, n: *const c_int,
               t: *mut complex_float, ldt: *const c_int, q: *mut complex_float,
               ldq: *const c_int, w: *mut complex_float, m: *mut c_int, s: *mut c_float,
               sep: *mut c_float, work: *mut complex_float, lwork: *const c_int,
               info: *mut c_int);
pub fn ztrsen_(job: *const c_char, compq: *const c_char, select: *const c_int, n: *const c_int,
               t: *mut complex_double, ldt: *const c_int, q: *mut complex_double,
               ldq: *const c_int, w: *mut complex_double, m: *mut c_int, s: *mut c_double,
               sep: *mut c_double, work: *mut complex_double, lwork: *const c_int,
               info: *mut c_int);

pub fn strsyl_(trana: *const c_char, tranb: *const c_char, isgn: *const c_int, m: *const c_int,
               n: *const c_int, a: *const c_float, lda: *const c_int, b: *const c_float,
               ldb: *const c_int, c: *mut c_float, ldc: *const c_int, scale: *mut c_float,
               info: *mut c_int);
pub fn dtrsyl_(trana: *const c_char, tranb: *const c_char, isgn: *const c_int, m: *const c_int,
               n: *const c_int, a: *const c_double, lda: *const c_int, b: *const c_double,
               ldb: *const c_int, c: *mut c_double, ldc: *const c_int, scale: *mut c_double,
               info: *mut c_int);
pub fn ctrsyl_(trana: *const c_char, tranb: *const c_char, isgn: *const c_int, m: *const c_int,
               n: *const c_int, a: *const complex_float, lda: *const c_int,
               b: *const complex_float, ldb: *const c_int, c: *mut complex_float,
               ldc: *const c_int, scale: *mut c_float, info: *mut c_int);
pub fn ztrsyl_(trana: *const c_char, tranb: *const c_char, isgn: *const c_int, m: *const c_int,
               n: *const c_int, a: *const complex_double, lda: *const c_int,
               b: *const complex_double, ldb: *const c_int, c: *mut complex_double,
               ldc: *const c_int, scale: *mut c_double, info: *mut c_int);

pub fn sgghrd_(compq: *const c_char, compz: *const c_char, n: *const c_int, ilo: *const c_int,
               ihi: *const c_int, a: *mut c_float, lda: *const c_int, b: *mut c_float,
               ldb: *const c_int, q: *mut c_float, ldq: *const c_int, z: *mut c_float,
               ldz: *const c_int, info: *mut c_int);
pub fn dgghrd_(compq: *const c_char, compz: *const c_char, n: *const c_int, ilo: *const c_int,
               ihi: *const c_int, a: *mut c_double, lda: *const c_int, b: *mut c_double,
               ldb: *const c_int, q: *mut c_double, ldq: *const c_int, z: *mut c_double,
               ldz: *const c_int, info: *mut c_int);
pub fn cgghrd_(compq: *const c_char, compz: *const c_char, n: *const c_int, ilo: *const c_int,
               ihi: *const c_int, a: *mut complex_float, lda: *const c_int,
               b: *mut complex_float, ldb: *const c_int, q: *mut complex_float,
               ldq: *const c_int, z: *mut complex_float, ldz: *const c_int, info: *mut c_int);
pub fn zgghrd_(compq: *const c_char, compz: *const c_char, n: *const c_int, ilo: *const c_int,
               ihi: *const c_int, a: *mut complex_double, lda: *const c_int,
               b: *mut complex_double, ldb: *const c_int, q: *mut complex_double,
               ldq: *const c_int, z: *mut complex_double, ldz: *const c_int, info: *mut c_int);

pub fn sggbal_(job: *const c_char, n: *const c_int, a: *mut c_float, lda: *const c_int,
               b: *mut c_float, ldb: *const c_int, ilo: *mut c_int, ihi: *mut c_int,
               lscale: *mut c_float, rscale: *mut c_float, work: *mut c_float,
               info: *mut c_int);
pub fn dggbal_(job: *const c_char, n: *const c_int, a: *mut c_double, lda: *const c_int,
               b: *mut c_double, ldb: *const c_int, ilo: *mut c_int, ihi: *mut c_int,
               lscale: *mut c_double, rscale: *mut c_double, work: *mut c_double,
               info: *mut c_int);
pub fn cggbal_(job: *const c_char, n: *const c_int, a: *mut complex_float, lda: *const c_int,
               b: *mut complex_float, ldb: *const c_int, ilo: *mut c_int, ihi: *mut c_int,
               lscale: *mut c_float, rscale: *mut c_float, work: *mut c_float,
               info: *mut c_int);
pub fn zggbal_(job: *const c_char, n: *const c_int, a: *mut complex_double, lda: *const c_int,
               b: *mut complex_double, ldb: *const c_int, ilo: *mut c_int, ihi: *mut c_int,
               lscale: *mut c_double, rscale: *mut c_double, work: *mut c_double,
               info: *mut c_int);

pub fn sggbak_(job: *const c_char, side: *const c_char, n: *const c_int, ilo: *const c_int,
               ihi: *const c_int, lscale: *const c_float, rscale: *const c_float,
               m: *const c_int, v: *mut c_float, ldv: *const c_int, info: *mut c_int);
pub fn dggbak_(job: *const c_char, side: *const c_char, n: *const c_int, ilo: *const c_int,
               ihi: *const c_int, lscale: *const c_double, rscale: *const c_double,
               m: *const c_int, v: *mut c_double, ldv: *const c_int, info: *mut c_int);
pub fn cggbak_(job: *const c_char, side: *const c_char, n: *const c_int, ilo: *const c_int,
               ihi: *const c_int, lscale: *const c_float, rscale: *const c_float,
               m: *const c_int, v: *mut complex_float, ldv: *const c_int, info: *mut c_int);
pub fn zggbak_(job: *const c_char, side: *const c_char, n: *const c_int, ilo: *const c_int,
               ihi: *const c_int, lscale: *const c_double, rscale: *const c_double,
               m: *const c_int, v: *mut complex_double, ldv: *const c_int, info: *mut c_int);

pub fn shgeqz_(job: *const c_char, compq: *const c_char, compz: *const c_char, n: *const c_int,
               ilo: *const c_int, ihi: *const c_int, h: *mut c_float, ldh: *const c_int,
               t: *mut c_float, ldt: *const c_int, alphar: *mut c_float, alphai: *mut c_float,
               beta: *mut c_float, q: *mut c_float, ldq: *const c_int, z: *mut c_float,
               ldz: *const c_int, work: *mut c_float, lwork: *const c_int, info: *mut c_int);
pub fn dhgeqz_(job: *const c_char, compq: *const c_char, compz: *const c_char, n: *const c_int,
               ilo: *const c_int, ihi: *const c_int, h: *mut c_double, ldh: *const c_int,
               t: *mut c_double, ldt: *const c_int, alphar: *mut c_double,
               alphai: *mut c_double, beta: *mut c_double, q: *mut c_double, ldq: *const c_int,
               z: *mut c_double, ldz: *const c_int, work: *mut c_double, lwork: *const c_int,
               info: *mut c_int);
pub fn chgeqz_(job: *const c_char, compq: *const c_char, compz: *const c_char, n: *const c_int,
               ilo: *const c_int, ihi: *const c_int, h: *mut complex_float, ldh: *const c_int,
               t: *mut complex_float, ldt: *const c_int, alpha: *mut complex_float,
               beta: *mut complex_float, q: *mut complex_float, ldq: *const c_int,
               z: *mut complex_float, ldz: *const c_int, work: *mut complex_float,
               lwork: *const c_int, rwork: *mut c_float, info: *mut c_int);
pub fn zhgeqz_(job: *const c_char, compq: *const c_char, compz: *const c_char, n: *const c_int,
               ilo: *const c_int, ihi: *const c_int, h: *mut complex_double, ldh: *const c_int,
               t: *mut complex_double, ldt: *const c_int, alpha: *mut complex_double,
               beta: *mut complex_double, q: *mut complex_double, ldq: *const c_int,
               z: *mut complex_double, ldz: *const c_int, work: *mut complex_double,
               lwork: *const c_int, rwork: *mut c_double, info: *mut c_int);

pub fn stgevc_(side: *const c_char, howmny: *const c_char, select: *const c_int,
               n: *const c_int, s: *const c_float, lds: *const c_int, p: *const c_float,
               ldp: *const c_int, vl: *mut c_float, ldvl: *const c_int, vr: *mut c_float,
               ldvr: *const c_int, mm: *const c_int, m: *mut c_int, work: *mut c_float,
               info: *mut c_int);
pub fn dtgevc_(side: *const c_char, howmny: *const c_char, select: *const c_int,
               n: *const c_int, s: *const c_double, lds: *const c_int, p: *const c_double,
               ldp: *const c_int, vl: *mut c_double, ldvl: *const c_int, vr: *mut c_double,
               ldvr: *const c_int, mm: *const c_int, m: *mut c_int, work: *mut c_double,
               info: *mut c_int);
pub fn ctgevc_(side: *const c_char, howmny: *const c_char, select: *const c_int,
               n: *const c_int, s: *const complex_float, lds: *const c_int,
               p: *const complex_float, ldp: *const c_int, vl: *mut complex_float,
               ldvl: *const c_int, vr: *mut complex_float, ldvr: *const c_int,
               mm: *const c_int, m: *mut c_int, work: *mut complex_float, rwork: *mut c_float,
               info: *mut c_int);
pub fn ztgevc_(side: *const c_char, howmny: *const c_char, select: *const c_int,
               n: *const c_int, s: *const complex_double, lds: *const c_int,
               p: *const complex_double, ldp: *const c_int, vl: *mut complex_double,
               ldvl: *const c_int, vr: *mut complex_double, ldvr: *const c_int,
               mm: *const c_int, m: *mut c_int, work: *mut complex_double,
               rwork: *mut c_double, info: *mut c_int);

pub fn stgexc_(wantq: *const c_int, wantz: *const c_int, n: *const c_int, a: *mut c_float,
               lda: *const c_int, b: *mut c_float, ldb: *const c_int, q: *mut c_float,
               ldq: *const c_int, z: *mut c_float, ldz: *const c_int, ifst: *mut c_int,
               ilst: *mut c_int, work: *mut c_float, lwork: *const c_int, info: *mut c_int);
pub fn dtgexc_(wantq: *const c_int, wantz: *const c_int, n: *const c_int, a: *mut c_double,
               lda: *const c_int, b: *mut c_double, ldb: *const c_int, q: *mut c_double,
               ldq: *const c_int, z: *mut c_double, ldz: *const c_int, ifst: *mut c_int,
               ilst: *mut c_int, work: *mut c_double, lwork: *const c_int, info: *mut c_int);
pub fn ctgexc_(wantq: *const c_int, wantz: *const c_int, n: *const c_int,
               a: *mut complex_float, lda: *const c_int, b: *mut complex_float,
               ldb: *const c_int, q: *mut complex_float, ldq: *const c_int,
               z: *mut complex_float, ldz: *const c_int, ifst: *const c_int, ilst: *mut c_int,
               info: *mut c_int);
pub fn ztgexc_(wantq: *const c_int, wantz: *const c_int, n: *const c_int,
               a: *mut complex_double, lda: *const c_int, b: *mut complex_double,
               ldb: *const c_int, q: *mut complex_double, ldq: *const c_int,
               z: *mut complex_double, ldz: *const c_int, ifst: *const c_int, ilst: *mut c_int,
               info: *mut c_int);

pub fn stgsen_(ijob: *const c_int, wantq: *const c_int, wantz: *const c_int,
               select: *const c_int, n: *const c_int, a: *mut c_float, lda: *const c_int,
               b: *mut c_float, ldb: *const c_int, alphar: *mut c_float, alphai: *mut c_float,
               beta: *mut c_float, q: *mut c_float, ldq: *const c_int, z: *mut c_float,
               ldz: *const c_int, m: *mut c_int, pl: *mut c_float, pr: *mut c_float,
               dif: *mut c_float, work: *mut c_float, lwork: *const c_int, iwork: *mut c_int,
               liwork: *const c_int, info: *mut c_int);
pub fn dtgsen_(ijob: *const c_int, wantq: *const c_int, wantz: *const c_int,
               select: *const c_int, n: *const c_int, a: *mut c_double, lda: *const c_int,
               b: *mut c_double, ldb: *const c_int, alphar: *mut c_double,
               alphai: *mut c_double, beta: *mut c_double, q: *mut c_double, ldq: *const c_int,
               z: *mut c_double, ldz: *const c_int, m: *mut c_int, pl: *mut c_double,
               pr: *mut c_double, dif: *mut c_double, work: *mut c_double, lwork: *const c_int,
               iwork: *mut c_int, liwork: *const c_int, info: *mut c_int);
pub fn ctgsen_(ijob: *const c_int, wantq: *const c_int, wantz: *const c_int,
               select: *const c_int, n: *const c_int, a: *mut complex_float, lda: *const c_int,
               b: *mut complex_float, ldb: *const c_int, alpha: *mut complex_float,
               beta: *mut complex_float, q: *mut complex_float, ldq: *const c_int,
               z: *mut complex_float, ldz: *const c_int, m: *mut c_int, pl: *mut c_float,
               pr: *mut c_float, dif: *mut c_float, work: *mut complex_float,
               lwork: *const c_int, iwork: *mut c_int, liwork: *const c_int, info: *mut c_int);
pub fn ztgsen_(ijob: *const c_int, wantq: *const c_int, wantz: *const c_int,
               select: *const c_int, n: *const c_int, a: *mut complex_double,
               lda: *const c_int, b: *mut complex_double, ldb: *const c_int,
               alpha: *mut complex_double, beta: *mut complex_double, q: *mut complex_double,
               ldq: *const c_int, z: *mut complex_double, ldz: *const c_int, m: *mut c_int,
               pl: *mut c_double, pr: *mut c_double, dif: *mut c_double,
               work: *mut complex_double, lwork: *const c_int, iwork: *mut c_int,
               liwork: *const c_int, info: *mut c_int);

pub fn stgsyl_(trans: *const c_char, ijob: *const c_int, m: *const c_int, n: *const c_int,
               a: *const c_float, lda: *const c_int, b: *const c_float, ldb: *const c_int,
               c: *mut c_float, ldc: *const c_int, d: *const c_float, ldd: *const c_int,
               e: *const c_float, lde: *const c_int, f: *mut c_float, ldf: *const c_int,
               scale: *mut c_float, dif: *mut c_float, work: *mut c_float, lwork: *const c_int,
               iwork: *mut c_int, info: *mut c_int);
pub fn dtgsyl_(trans: *const c_char, ijob: *const c_int, m: *const c_int, n: *const c_int,
               a: *const c_double, lda: *const c_int, b: *const c_double, ldb: *const c_int,
               c: *mut c_double, ldc: *const c_int, d: *const c_double, ldd: *const c_int,
               e: *const c_double, lde: *const c_int, f: *mut c_double, ldf: *const c_int,
               scale: *mut c_double, dif: *mut c_double, work: *mut c_double,
               lwork: *const c_int, iwork: *mut c_int, info: *mut c_int);
pub fn ctgsyl_(trans: *const c_char, ijob: *const c_int, m: *const c_int, n: *const c_int,
               a: *const complex_float, lda: *const c_int, b: *const complex_float,
               ldb: *const c_int, c: *mut complex_float, ldc: *const c_int,
               d: *const complex_float, ldd: *const c_int, e: *const complex_float,
               lde: *const c_int, f: *mut complex_float, ldf: *const c_int,
               scale: *mut c_float, dif: *mut c_float, work: *mut complex_float,
               lwork: *const c_int, iwork: *mut c_int, info: *mut c_int);
pub fn ztgsyl_(trans: *const c_char, ijob: *const c_int, m: *const c_int, n: *const c_int,
               a: *const complex_double, lda: *const c_int, b: *const complex_double,
               ldb: *const c_int, c: *mut complex_double, ldc: *const c_int,
               d: *const complex_double, ldd: *const c_int, e: *const complex_double,
               lde: *const c_int, f: *mut complex_double, ldf: *const c_int,
               scale: *mut c_double, dif: *mut c_double, work: *mut complex_double,
               lwork: *const c_int, iwork: *mut c_int, info: *mut c_int);

pub fn stgsna_(job: *const c_char, howmny: *const c_char, select: *const c_int,
               n: *const c_int, a: *const c_float, lda: *const c_int, b: *const c_float,
               ldb: *const c_int, vl: *const c_float, ldvl: *const c_int, vr: *const c_float,
               ldvr: *const c_int, s: *mut c_float, dif: *mut c_float, mm: *const c_int,
               m: *mut c_int, work: *mut c_float, lwork: *const c_int, iwork: *mut c_int,
               info: *mut c_int);
pub fn dtgsna_(job: *const c_char, howmny: *const c_char, select: *const c_int,
               n: *const c_int, a: *const c_double, lda: *const c_int, b: *const c_double,
               ldb: *const c_int, vl: *const c_double, ldvl: *const c_int, vr: *const c_double,
               ldvr: *const c_int, s: *mut c_double, dif: *mut c_double, mm: *const c_int,
               m: *mut c_int, work: *mut c_double, lwork: *const c_int, iwork: *mut c_int,
               info: *mut c_int);
pub fn ctgsna_(job: *const c_char, howmny: *const c_char, select: *const c_int,
               n: *const c_int, a: *const complex_float, lda: *const c_int,
               b: *const complex_float, ldb: *const c_int, vl: *const complex_float,
               ldvl: *const c_int, vr: *const complex_float, ldvr: *const c_int,
               s: *mut c_float, dif: *mut c_float, mm: *const c_int, m: *mut c_int,
               work: *mut complex_float, lwork: *const c_int, iwork: *mut c_int,
               info: *mut c_int);
pub fn ztgsna_(job: *const c_char, howmny: *const c_char, select: *const c_int,
               n: *const c_int, a: *const complex_double, lda: *const c_int,
               b: *const complex_double, ldb: *const c_int, vl: *const complex_double,
               ldvl: *const c_int, vr: *const complex_double, ldvr: *const c_int,
               s: *mut c_double, dif: *mut c_double, mm: *const c_int, m: *mut c_int,
               work: *mut complex_double, lwork: *const c_int, iwork: *mut c_int,
               info: *mut c_int);

pub fn sggsvp_(jobu: *const c_char, jobv: *const c_char, jobq: *const c_char, m: *const c_int,
               p: *const c_int, n: *const c_int, a: *mut c_float, lda: *const c_int,
               b: *mut c_float, ldb: *const c_int, tola: *const c_float, tolb: *const c_float,
               k: *mut c_int, l: *mut c_int, u: *mut c_float, ldu: *const c_int,
               v: *mut c_float, ldv: *const c_int, q: *mut c_float, ldq: *const c_int,
               iwork: *mut c_int, tau: *mut c_float, work: *mut c_float, info: *mut c_int);
pub fn dggsvp_(jobu: *const c_char, jobv: *const c_char, jobq: *const c_char, m: *const c_int,
               p: *const c_int, n: *const c_int, a: *mut c_double, lda: *const c_int,
               b: *mut c_double, ldb: *const c_int, tola: *const c_double,
               tolb: *const c_double, k: *mut c_int, l: *mut c_int, u: *mut c_double,
               ldu: *const c_int, v: *mut c_double, ldv: *const c_int, q: *mut c_double,
               ldq: *const c_int, iwork: *mut c_int, tau: *mut c_double, work: *mut c_double,
               info: *mut c_int);
pub fn cggsvp_(jobu: *const c_char, jobv: *const c_char, jobq: *const c_char, m: *const c_int,
               p: *const c_int, n: *const c_int, a: *mut complex_float, lda: *const c_int,
               b: *mut complex_float, ldb: *const c_int, tola: *const c_float,
               tolb: *const c_float, k: *mut c_int, l: *mut c_int, u: *mut complex_float,
               ldu: *const c_int, v: *mut complex_float, ldv: *const c_int,
               q: *mut complex_float, ldq: *const c_int, iwork: *mut c_int,
               rwork: *mut c_float, tau: *mut complex_float, work: *mut complex_float,
               info: *mut c_int);
pub fn zggsvp_(jobu: *const c_char, jobv: *const c_char, jobq: *const c_char, m: *const c_int,
               p: *const c_int, n: *const c_int, a: *mut complex_double, lda: *const c_int,
               b: *mut complex_double, ldb: *const c_int, tola: *const c_double,
               tolb: *const c_double, k: *mut c_int, l: *mut c_int, u: *mut complex_double,
               ldu: *const c_int, v: *mut complex_double, ldv: *const c_int,
               q: *mut complex_double, ldq: *const c_int, iwork: *mut c_int,
               rwork: *mut c_double, tau: *mut complex_double, work: *mut complex_double,
               info: *mut c_int);

pub fn stgsja_(jobu: *const c_char, jobv: *const c_char, jobq: *const c_char, m: *const c_int,
               p: *const c_int, n: *const c_int, k: *const c_int, l: *const c_int,
               a: *mut c_float, lda: *const c_int, b: *mut c_float, ldb: *const c_int,
               tola: *const c_float, tolb: *const c_float, alpha: *mut c_float,
               beta: *mut c_float, u: *mut c_float, ldu: *const c_int, v: *mut c_float,
               ldv: *const c_int, q: *mut c_float, ldq: *const c_int, work: *mut c_float,
               ncycle: *mut c_int, info: *mut c_int);
pub fn dtgsja_(jobu: *const c_char, jobv: *const c_char, jobq: *const c_char, m: *const c_int,
               p: *const c_int, n: *const c_int, k: *const c_int, l: *const c_int,
               a: *mut c_double, lda: *const c_int, b: *mut c_double, ldb: *const c_int,
               tola: *const c_double, tolb: *const c_double, alpha: *mut c_double,
               beta: *mut c_double, u: *mut c_double, ldu: *const c_int, v: *mut c_double,
               ldv: *const c_int, q: *mut c_double, ldq: *const c_int, work: *mut c_double,
               ncycle: *mut c_int, info: *mut c_int);
pub fn ctgsja_(jobu: *const c_char, jobv: *const c_char, jobq: *const c_char, m: *const c_int,
               p: *const c_int, n: *const c_int, k: *const c_int, l: *const c_int,
               a: *mut complex_float, lda: *const c_int, b: *mut complex_float,
               ldb: *const c_int, tola: *const c_float, tolb: *const c_float,
               alpha: *mut c_float, beta: *mut c_float, u: *mut complex_float,
               ldu: *const c_int, v: *mut complex_float, ldv: *const c_int,
               q: *mut complex_float, ldq: *const c_int, work: *mut complex_float,
               ncycle: *mut c_int, info: *mut c_int);
pub fn ztgsja_(jobu: *const c_char, jobv: *const c_char, jobq: *const c_char, m: *const c_int,
               p: *const c_int, n: *const c_int, k: *const c_int, l: *const c_int,
               a: *mut complex_double, lda: *const c_int, b: *mut complex_double,
               ldb: *const c_int, tola: *const c_double, tolb: *const c_double,
               alpha: *mut c_double, beta: *mut c_double, u: *mut complex_double,
               ldu: *const c_int, v: *mut complex_double, ldv: *const c_int,
               q: *mut complex_double, ldq: *const c_int, work: *mut complex_double,
               ncycle: *mut c_int, info: *mut c_int);

pub fn sgels_(trans: *const c_char, m: *const c_int, n: *const c_int, nrhs: *const c_int,
              a: *mut c_float, lda: *const c_int, b: *mut c_float, ldb: *const c_int,
              work: *mut c_float, lwork: *const c_int, info: *mut c_int);
pub fn dgels_(trans: *const c_char, m: *const c_int, n: *const c_int, nrhs: *const c_int,
              a: *mut c_double, lda: *const c_int, b: *mut c_double, ldb: *const c_int,
              work: *mut c_double, lwork: *const c_int, info: *mut c_int);
pub fn cgels_(trans: *const c_char, m: *const c_int, n: *const c_int, nrhs: *const c_int,
              a: *mut complex_float, lda: *const c_int, b: *mut complex_float,
              ldb: *const c_int, work: *mut complex_float, lwork: *const c_int,
              info: *mut c_int);
pub fn zgels_(trans: *const c_char, m: *const c_int, n: *const c_int, nrhs: *const c_int,
              a: *mut complex_double, lda: *const c_int, b: *mut complex_double,
              ldb: *const c_int, work: *mut complex_double, lwork: *const c_int,
              info: *mut c_int);

pub fn sgelsy_(m: *const c_int, n: *const c_int, nrhs: *const c_int, a: *mut c_float,
               lda: *const c_int, b: *mut c_float, ldb: *const c_int, jpvt: *mut c_int,
               rcond: *const c_float, rank: *mut c_int, work: *mut c_float,
               lwork: *const c_int, info: *mut c_int);
pub fn dgelsy_(m: *const c_int, n: *const c_int, nrhs: *const c_int, a: *mut c_double,
               lda: *const c_int, b: *mut c_double, ldb: *const c_int, jpvt: *mut c_int,
               rcond: *const c_double, rank: *mut c_int, work: *mut c_double,
               lwork: *const c_int, info: *mut c_int);
pub fn cgelsy_(m: *const c_int, n: *const c_int, nrhs: *const c_int, a: *mut complex_float,
               lda: *const c_int, b: *mut complex_float, ldb: *const c_int, jpvt: *mut c_int,
               rcond: *const c_float, rank: *mut c_int, work: *mut complex_float,
               lwork: *const c_int, rwork: *mut c_float, info: *mut c_int);
pub fn zgelsy_(m: *const c_int, n: *const c_int, nrhs: *const c_int, a: *mut complex_double,
               lda: *const c_int, b: *mut complex_double, ldb: *const c_int, jpvt: *mut c_int,
               rcond: *const c_double, rank: *mut c_int, work: *mut complex_double,
               lwork: *const c_int, rwork: *mut c_double, info: *mut c_int);

pub fn sgelss_(m: *const c_int, n: *const c_int, nrhs: *const c_int, a: *mut c_float,
               lda: *const c_int, b: *mut c_float, ldb: *const c_int, s: *mut c_float,
               rcond: *const c_float, rank: *mut c_int, work: *mut c_float,
               lwork: *const c_int, info: *mut c_int);
pub fn dgelss_(m: *const c_int, n: *const c_int, nrhs: *const c_int, a: *mut c_double,
               lda: *const c_int, b: *mut c_double, ldb: *const c_int, s: *mut c_double,
               rcond: *const c_double, rank: *mut c_int, work: *mut c_double,
               lwork: *const c_int, info: *mut c_int);
pub fn cgelss_(m: *const c_int, n: *const c_int, nrhs: *const c_int, a: *mut complex_float,
               lda: *const c_int, b: *mut complex_float, ldb: *const c_int, s: *mut c_float,
               rcond: *const c_float, rank: *mut c_int, work: *mut complex_float,
               lwork: *const c_int, rwork: *mut c_float, info: *mut c_int);
pub fn zgelss_(m: *const c_int, n: *const c_int, nrhs: *const c_int, a: *mut complex_double,
               lda: *const c_int, b: *mut complex_double, ldb: *const c_int, s: *mut c_double,
               rcond: *const c_double, rank: *mut c_int, work: *mut complex_double,
               lwork: *const c_int, rwork: *mut c_double, info: *mut c_int);

pub fn sgelsd_(m: *const c_int, n: *const c_int, nrhs: *const c_int, a: *const c_float,
               lda: *const c_int, b: *mut c_float, ldb: *const c_int, s: *mut c_float,
               rcond: *const c_float, rank: *mut c_int, work: *mut c_float,
               lwork: *const c_int, iwork: *mut c_int, info: *mut c_int);
pub fn dgelsd_(m: *const c_int, n: *const c_int, nrhs: *const c_int, a: *const c_double,
               lda: *const c_int, b: *mut c_double, ldb: *const c_int, s: *mut c_double,
               rcond: *const c_double, rank: *mut c_int, work: *mut c_double,
               lwork: *const c_int, iwork: *mut c_int, info: *mut c_int);
pub fn cgelsd_(m: *const c_int, n: *const c_int, nrhs: *const c_int, a: *mut complex_float,
               lda: *const c_int, b: *mut complex_float, ldb: *const c_int, s: *mut c_float,
               rcond: *const c_float, rank: *mut c_int, work: *mut complex_float,
               lwork: *const c_int, rwork: *mut c_float, iwork: *mut c_int, info: *mut c_int);
pub fn zgelsd_(m: *const c_int, n: *const c_int, nrhs: *const c_int, a: *const complex_double,
               lda: *const c_int, b: *mut complex_double, ldb: *const c_int, s: *mut c_double,
               rcond: *const c_double, rank: *mut c_int, work: *mut complex_double,
               lwork: *const c_int, rwork: *mut c_double, iwork: *mut c_int, info: *mut c_int);

pub fn sgglse_(m: *const c_int, n: *const c_int, p: *const c_int, a: *mut c_float,
               lda: *const c_int, b: *mut c_float, ldb: *const c_int, c: *mut c_float,
               d: *mut c_float, x: *mut c_float, work: *mut c_float, lwork: *const c_int,
               info: *mut c_int);
pub fn dgglse_(m: *const c_int, n: *const c_int, p: *const c_int, a: *mut c_double,
               lda: *const c_int, b: *mut c_double, ldb: *const c_int, c: *mut c_double,
               d: *mut c_double, x: *mut c_double, work: *mut c_double, lwork: *const c_int,
               info: *mut c_int);
pub fn cgglse_(m: *const c_int, n: *const c_int, p: *const c_int, a: *mut complex_float,
               lda: *const c_int, b: *mut complex_float, ldb: *const c_int,
               c: *mut complex_float, d: *mut complex_float, x: *mut complex_float,
               work: *mut complex_float, lwork: *const c_int, info: *mut c_int);
pub fn zgglse_(m: *const c_int, n: *const c_int, p: *const c_int, a: *mut complex_double,
               lda: *const c_int, b: *mut complex_double, ldb: *const c_int,
               c: *mut complex_double, d: *mut complex_double, x: *mut complex_double,
               work: *mut complex_double, lwork: *const c_int, info: *mut c_int);

pub fn sggglm_(n: *const c_int, m: *const c_int, p: *const c_int, a: *mut c_float,
               lda: *const c_int, b: *mut c_float, ldb: *const c_int, d: *mut c_float,
               x: *mut c_float, y: *mut c_float, work: *mut c_float, lwork: *const c_int,
               info: *mut c_int);
pub fn dggglm_(n: *const c_int, m: *const c_int, p: *const c_int, a: *mut c_double,
               lda: *const c_int, b: *mut c_double, ldb: *const c_int, d: *mut c_double,
               x: *mut c_double, y: *mut c_double, work: *mut c_double, lwork: *const c_int,
               info: *mut c_int);
pub fn cggglm_(n: *const c_int, m: *const c_int, p: *const c_int, a: *mut complex_float,
               lda: *const c_int, b: *mut complex_float, ldb: *const c_int,
               d: *mut complex_float, x: *mut complex_float, y: *mut complex_float,
               work: *mut complex_float, lwork: *const c_int, info: *mut c_int);
pub fn zggglm_(n: *const c_int, m: *const c_int, p: *const c_int, a: *mut complex_double,
               lda: *const c_int, b: *mut complex_double, ldb: *const c_int,
               d: *mut complex_double, x: *mut complex_double, y: *mut complex_double,
               work: *mut complex_double, lwork: *const c_int, info: *mut c_int);

pub fn ssyev_(jobz: *const c_char, uplo: *const c_char, n: *const c_int, a: *mut c_float,
              lda: *const c_int, w: *mut c_float, work: *mut c_float, lwork: *const c_int,
              info: *mut c_int);
pub fn dsyev_(jobz: *const c_char, uplo: *const c_char, n: *const c_int, a: *mut c_double,
              lda: *const c_int, w: *mut c_double, work: *mut c_double, lwork: *const c_int,
              info: *mut c_int);
pub fn cheev_(jobz: *const c_char, uplo: *const c_char, n: *const c_int, a: *mut complex_float,
              lda: *const c_int, w: *mut c_float, work: *mut complex_float,
              lwork: *const c_int, rwork: *mut c_float, info: *mut c_int);
pub fn zheev_(jobz: *const c_char, uplo: *const c_char, n: *const c_int,
              a: *mut complex_double, lda: *const c_int, w: *mut c_double,
              work: *mut complex_double, lwork: *const c_int, rwork: *mut c_double,
              info: *mut c_int);

pub fn ssyevd_(jobz: *const c_char, uplo: *const c_char, n: *const c_int, a: *mut c_float,
               lda: *const c_int, w: *mut c_float, work: *mut c_float, lwork: *const c_int,
               iwork: *mut c_int, liwork: *const c_int, info: *mut c_int);
pub fn dsyevd_(jobz: *const c_char, uplo: *const c_char, n: *const c_int, a: *mut c_double,
               lda: *const c_int, w: *mut c_double, work: *mut c_double, lwork: *const c_int,
               iwork: *mut c_int, liwork: *const c_int, info: *mut c_int);
pub fn cheevd_(jobz: *const c_char, uplo: *const c_char, n: *const c_int,
               a: *mut complex_float, lda: *const c_int, w: *mut c_float,
               work: *mut complex_float, lwork: *const c_int, rwork: *mut c_float,
               lrwork: *const c_int, iwork: *mut c_int, liwork: *const c_int,
               info: *mut c_int);
pub fn zheevd_(jobz: *const c_char, uplo: *const c_char, n: *const c_int,
               a: *mut complex_double, lda: *const c_int, w: *mut c_double,
               work: *mut complex_double, lwork: *const c_int, rwork: *mut c_double,
               lrwork: *const c_int, iwork: *mut c_int, liwork: *const c_int,
               info: *mut c_int);

pub fn ssyevx_(jobz: *const c_char, range: *const c_char, uplo: *const c_char, n: *const c_int,
               a: *mut c_float, lda: *const c_int, vl: *const c_float, vu: *const c_float,
               il: *const c_int, iu: *const c_int, abstol: *const c_float, m: *mut c_int,
               w: *mut c_float, z: *mut c_float, ldz: *const c_int, work: *mut c_float,
               lwork: *const c_int, iwork: *mut c_int, ifail: *mut c_int, info: *mut c_int);
pub fn dsyevx_(jobz: *const c_char, range: *const c_char, uplo: *const c_char, n: *const c_int,
               a: *mut c_double, lda: *const c_int, vl: *const c_double, vu: *const c_double,
               il: *const c_int, iu: *const c_int, abstol: *const c_double, m: *mut c_int,
               w: *mut c_double, z: *mut c_double, ldz: *const c_int, work: *mut c_double,
               lwork: *const c_int, iwork: *mut c_int, ifail: *mut c_int, info: *mut c_int);
pub fn cheevx_(jobz: *const c_char, range: *const c_char, uplo: *const c_char, n: *const c_int,
               a: *mut complex_float, lda: *const c_int, vl: *const c_float,
               vu: *const c_float, il: *const c_int, iu: *const c_int, abstol: *const c_float,
               m: *mut c_int, w: *mut c_float, z: *mut complex_float, ldz: *const c_int,
               work: *mut complex_float, lwork: *const c_int, rwork: *mut c_float,
               iwork: *mut c_int, ifail: *mut c_int, info: *mut c_int);
pub fn zheevx_(jobz: *const c_char, range: *const c_char, uplo: *const c_char, n: *const c_int,
               a: *mut complex_double, lda: *const c_int, vl: *const c_double,
               vu: *const c_double, il: *const c_int, iu: *const c_int,
               abstol: *const c_double, m: *mut c_int, w: *mut c_double,
               z: *mut complex_double, ldz: *const c_int, work: *mut complex_double,
               lwork: *const c_int, rwork: *mut c_double, iwork: *mut c_int, ifail: *mut c_int,
               info: *mut c_int);

pub fn ssyevr_(jobz: *const c_char, range: *const c_char, uplo: *const c_char, n: *const c_int,
               a: *mut c_float, lda: *const c_int, vl: *const c_float, vu: *const c_float,
               il: *const c_int, iu: *const c_int, abstol: *const c_float, m: *mut c_int,
               w: *mut c_float, z: *mut c_float, ldz: *const c_int, isuppz: *mut c_int,
               work: *mut c_float, lwork: *const c_int, iwork: *mut c_int,
               liwork: *const c_int, info: *mut c_int);
pub fn dsyevr_(jobz: *const c_char, range: *const c_char, uplo: *const c_char, n: *const c_int,
               a: *mut c_double, lda: *const c_int, vl: *const c_double, vu: *const c_double,
               il: *const c_int, iu: *const c_int, abstol: *const c_double, m: *mut c_int,
               w: *mut c_double, z: *mut c_double, ldz: *const c_int, isuppz: *mut c_int,
               work: *mut c_double, lwork: *const c_int, iwork: *mut c_int,
               liwork: *const c_int, info: *mut c_int);
pub fn cheevr_(jobz: *const c_char, range: *const c_char, uplo: *const c_char, n: *const c_int,
               a: *mut complex_float, lda: *const c_int, vl: *const c_float,
               vu: *const c_float, il: *const c_int, iu: *const c_int, abstol: *const c_float,
               m: *mut c_int, w: *mut c_float, z: *mut complex_float, ldz: *const c_int,
               isuppz: *mut c_int, work: *mut complex_float, lwork: *const c_int,
               rwork: *mut c_float, lrwork: *const c_int, iwork: *mut c_int,
               liwork: *const c_int, info: *mut c_int);
pub fn zheevr_(jobz: *const c_char, range: *const c_char, uplo: *const c_char, n: *const c_int,
               a: *mut complex_double, lda: *const c_int, vl: *const c_double,
               vu: *const c_double, il: *const c_int, iu: *const c_int,
               abstol: *const c_double, m: *mut c_int, w: *mut c_double,
               z: *mut complex_double, ldz: *const c_int, isuppz: *mut c_int,
               work: *mut complex_double, lwork: *const c_int, rwork: *mut c_double,
               lrwork: *const c_int, iwork: *mut c_int, liwork: *const c_int,
               info: *mut c_int);

pub fn sspev_(jobz: *const c_char, uplo: *const c_char, n: *const c_int, ap: *mut c_float,
              w: *mut c_float, z: *mut c_float, ldz: *const c_int, work: *mut c_float,
              info: *mut c_int);
pub fn dspev_(jobz: *const c_char, uplo: *const c_char, n: *const c_int, ap: *mut c_double,
              w: *mut c_double, z: *mut c_double, ldz: *const c_int, work: *mut c_double,
              info: *mut c_int);
pub fn chpev_(jobz: *const c_char, uplo: *const c_char, n: *const c_int,
              ap: *mut complex_float, w: *mut c_float, z: *mut complex_float,
              ldz: *const c_int, work: *mut complex_float, rwork: *mut c_float,
              info: *mut c_int);
pub fn zhpev_(jobz: *const c_char, uplo: *const c_char, n: *const c_int,
              ap: *mut complex_double, w: *mut c_double, z: *mut complex_double,
              ldz: *const c_int, work: *mut complex_double, rwork: *mut c_double,
              info: *mut c_int);

pub fn sspevd_(jobz: *const c_char, uplo: *const c_char, n: *const c_int, ap: *mut c_float,
               w: *mut c_float, z: *mut c_float, ldz: *const c_int, work: *mut c_float,
               lwork: *const c_int, iwork: *mut c_int, liwork: *const c_int, info: *mut c_int);
pub fn dspevd_(jobz: *const c_char, uplo: *const c_char, n: *const c_int, ap: *mut c_double,
               w: *mut c_double, z: *mut c_double, ldz: *const c_int, work: *mut c_double,
               lwork: *const c_int, iwork: *mut c_int, liwork: *const c_int, info: *mut c_int);
pub fn chpevd_(jobz: *const c_char, uplo: *const c_char, n: *const c_int,
               ap: *mut complex_float, w: *mut c_float, z: *mut complex_float,
               ldz: *const c_int, work: *mut complex_float, lwork: *const c_int,
               rwork: *mut c_float, lrwork: *const c_int, iwork: *mut c_int,
               liwork: *const c_int, info: *mut c_int);
pub fn zhpevd_(jobz: *const c_char, uplo: *const c_char, n: *const c_int,
               ap: *mut complex_double, w: *mut c_double, z: *mut complex_double,
               ldz: *const c_int, work: *mut complex_double, lwork: *const c_int,
               rwork: *mut c_double, lrwork: *const c_int, iwork: *mut c_int,
               liwork: *const c_int, info: *mut c_int);

pub fn sspevx_(jobz: *const c_char, range: *const c_char, uplo: *const c_char, n: *const c_int,
               ap: *mut c_float, vl: *const c_float, vu: *const c_float, il: *const c_int,
               iu: *const c_int, abstol: *const c_float, m: *mut c_int, w: *mut c_float,
               z: *mut c_float, ldz: *const c_int, work: *mut c_float, iwork: *mut c_int,
               ifail: *mut c_int, info: *mut c_int);
pub fn dspevx_(jobz: *const c_char, range: *const c_char, uplo: *const c_char, n: *const c_int,
               ap: *mut c_double, vl: *const c_double, vu: *const c_double, il: *const c_int,
               iu: *const c_int, abstol: *const c_double, m: *mut c_int, w: *mut c_double,
               z: *mut c_double, ldz: *const c_int, work: *mut c_double, iwork: *mut c_int,
               ifail: *mut c_int, info: *mut c_int);
pub fn chpevx_(jobz: *const c_char, range: *const c_char, uplo: *const c_char, n: *const c_int,
               ap: *mut complex_float, vl: *const c_float, vu: *const c_float,
               il: *const c_int, iu: *const c_int, abstol: *const c_float, m: *mut c_int,
               w: *mut c_float, z: *mut complex_float, ldz: *const c_int,
               work: *mut complex_float, rwork: *mut c_float, iwork: *mut c_int,
               ifail: *mut c_int, info: *mut c_int);
pub fn zhpevx_(jobz: *const c_char, range: *const c_char, uplo: *const c_char, n: *const c_int,
               ap: *mut complex_double, vl: *const c_double, vu: *const c_double,
               il: *const c_int, iu: *const c_int, abstol: *const c_double, m: *mut c_int,
               w: *mut c_double, z: *mut complex_double, ldz: *const c_int,
               work: *mut complex_double, rwork: *mut c_double, iwork: *mut c_int,
               ifail: *mut c_int, info: *mut c_int);

pub fn ssbev_(jobz: *const c_char, uplo: *const c_char, n: *const c_int, kd: *const c_int,
              ab: *mut c_float, ldab: *const c_int, w: *mut c_float, z: *mut c_float,
              ldz: *const c_int, work: *mut c_float, info: *mut c_int);
pub fn dsbev_(jobz: *const c_char, uplo: *const c_char, n: *const c_int, kd: *const c_int,
              ab: *mut c_double, ldab: *const c_int, w: *mut c_double, z: *mut c_double,
              ldz: *const c_int, work: *mut c_double, info: *mut c_int);
pub fn chbev_(jobz: *const c_char, uplo: *const c_char, n: *const c_int, kd: *const c_int,
              ab: *mut complex_float, ldab: *const c_int, w: *mut c_float,
              z: *mut complex_float, ldz: *const c_int, work: *mut complex_float,
              rwork: *mut c_float, info: *mut c_int);
pub fn zhbev_(jobz: *const c_char, uplo: *const c_char, n: *const c_int, kd: *const c_int,
              ab: *mut complex_double, ldab: *const c_int, w: *mut c_double,
              z: *mut complex_double, ldz: *const c_int, work: *mut complex_double,
              rwork: *mut c_double, info: *mut c_int);

pub fn ssbevd_(jobz: *const c_char, uplo: *const c_char, n: *const c_int, kd: *const c_int,
               ab: *mut c_float, ldab: *const c_int, w: *mut c_float, z: *mut c_float,
               ldz: *const c_int, work: *mut c_float, lwork: *const c_int, iwork: *mut c_int,
               liwork: *const c_int, info: *mut c_int);
pub fn dsbevd_(jobz: *const c_char, uplo: *const c_char, n: *const c_int, kd: *const c_int,
               ab: *mut c_double, ldab: *const c_int, w: *mut c_double, z: *mut c_double,
               ldz: *const c_int, work: *mut c_double, lwork: *const c_int, iwork: *mut c_int,
               liwork: *const c_int, info: *mut c_int);
pub fn chbevd_(jobz: *const c_char, uplo: *const c_char, n: *const c_int, kd: *const c_int,
               ab: *mut complex_float, ldab: *const c_int, w: *mut c_float,
               z: *mut complex_float, ldz: *const c_int, work: *mut complex_float,
               lwork: *const c_int, rwork: *mut c_float, lrwork: *const c_int,
               iwork: *mut c_int, liwork: *const c_int, info: *mut c_int);
pub fn zhbevd_(jobz: *const c_char, uplo: *const c_char, n: *const c_int, kd: *const c_int,
               ab: *mut complex_double, ldab: *const c_int, w: *mut c_double,
               z: *mut complex_double, ldz: *const c_int, work: *mut complex_double,
               lwork: *const c_int, rwork: *mut c_double, lrwork: *const c_int,
               iwork: *mut c_int, liwork: *const c_int, info: *mut c_int);

pub fn ssbevx_(jobz: *const c_char, range: *const c_char, uplo: *const c_char, n: *const c_int,
               kd: *const c_int, ab: *mut c_float, ldab: *const c_int, q: *mut c_float,
               ldq: *const c_int, vl: *const c_float, vu: *const c_float, il: *const c_int,
               iu: *const c_int, abstol: *const c_float, m: *mut c_int, w: *mut c_float,
               z: *mut c_float, ldz: *const c_int, work: *mut c_float, iwork: *mut c_int,
               ifail: *mut c_int, info: *mut c_int);
pub fn dsbevx_(jobz: *const c_char, range: *const c_char, uplo: *const c_char, n: *const c_int,
               kd: *const c_int, ab: *mut c_double, ldab: *const c_int, q: *mut c_double,
               ldq: *const c_int, vl: *const c_double, vu: *const c_double, il: *const c_int,
               iu: *const c_int, abstol: *const c_double, m: *mut c_int, w: *mut c_double,
               z: *mut c_double, ldz: *const c_int, work: *mut c_double, iwork: *mut c_int,
               ifail: *mut c_int, info: *mut c_int);
pub fn chbevx_(jobz: *const c_char, range: *const c_char, uplo: *const c_char, n: *const c_int,
               kd: *const c_int, ab: *mut complex_float, ldab: *const c_int,
               q: *mut complex_float, ldq: *const c_int, vl: *const c_float,
               vu: *const c_float, il: *const c_int, iu: *const c_int, abstol: *const c_float,
               m: *mut c_int, w: *mut c_float, z: *mut complex_float, ldz: *const c_int,
               work: *mut complex_float, rwork: *mut c_float, iwork: *mut c_int,
               ifail: *mut c_int, info: *mut c_int);
pub fn zhbevx_(jobz: *const c_char, range: *const c_char, uplo: *const c_char, n: *const c_int,
               kd: *const c_int, ab: *mut complex_double, ldab: *const c_int,
               q: *mut complex_double, ldq: *const c_int, vl: *const c_double,
               vu: *const c_double, il: *const c_int, iu: *const c_int,
               abstol: *const c_double, m: *mut c_int, w: *mut c_double,
               z: *mut complex_double, ldz: *const c_int, work: *mut complex_double,
               rwork: *mut c_double, iwork: *mut c_int, ifail: *mut c_int, info: *mut c_int);

pub fn sstev_(jobz: *const c_char, n: *const c_int, d: *mut c_float, e: *mut c_float,
              z: *mut c_float, ldz: *const c_int, work: *mut c_float, info: *mut c_int);
pub fn dstev_(jobz: *const c_char, n: *const c_int, d: *mut c_double, e: *mut c_double,
              z: *mut c_double, ldz: *const c_int, work: *mut c_double, info: *mut c_int);

pub fn sstevd_(jobz: *const c_char, n: *const c_int, d: *mut c_float, e: *mut c_float,
               z: *mut c_float, ldz: *const c_int, work: *mut c_float, lwork: *const c_int,
               iwork: *mut c_int, liwork: *const c_int, info: *mut c_int);
pub fn dstevd_(jobz: *const c_char, n: *const c_int, d: *mut c_double, e: *mut c_double,
               z: *mut c_double, ldz: *const c_int, work: *mut c_double, lwork: *const c_int,
               iwork: *mut c_int, liwork: *const c_int, info: *mut c_int);

pub fn sstevx_(jobz: *const c_char, range: *const c_char, n: *const c_int, d: *mut c_float,
               e: *mut c_float, vl: *const c_float, vu: *const c_float, il: *const c_int,
               iu: *const c_int, abstol: *const c_float, m: *mut c_int, w: *mut c_float,
               z: *mut c_float, ldz: *const c_int, work: *mut c_float, iwork: *mut c_int,
               ifail: *mut c_int, info: *mut c_int);
pub fn dstevx_(jobz: *const c_char, range: *const c_char, n: *const c_int, d: *mut c_double,
               e: *mut c_double, vl: *const c_double, vu: *const c_double, il: *const c_int,
               iu: *const c_int, abstol: *const c_double, m: *mut c_int, w: *mut c_double,
               z: *mut c_double, ldz: *const c_int, work: *mut c_double, iwork: *mut c_int,
               ifail: *mut c_int, info: *mut c_int);

pub fn sstevr_(jobz: *const c_char, range: *const c_char, n: *const c_int, d: *mut c_float,
               e: *mut c_float, vl: *const c_float, vu: *const c_float, il: *const c_int,
               iu: *const c_int, abstol: *const c_float, m: *mut c_int, w: *mut c_float,
               z: *mut c_float, ldz: *const c_int, isuppz: *mut c_int, work: *mut c_float,
               lwork: *const c_int, iwork: *mut c_int, liwork: *const c_int, info: *mut c_int);
pub fn dstevr_(jobz: *const c_char, range: *const c_char, n: *const c_int, d: *mut c_double,
               e: *mut c_double, vl: *const c_double, vu: *const c_double, il: *const c_int,
               iu: *const c_int, abstol: *const c_double, m: *mut c_int, w: *mut c_double,
               z: *mut c_double, ldz: *const c_int, isuppz: *mut c_int, work: *mut c_double,
               lwork: *const c_int, iwork: *mut c_int, liwork: *const c_int, info: *mut c_int);

pub fn sgees_(jobvs: *const c_char, sort: *const c_char, select: LAPACK_S_SELECT2,
              n: *const c_int, a: *mut c_float, lda: *const c_int, sdim: *mut c_int,
              wr: *mut c_float, wi: *mut c_float, vs: *mut c_float, ldvs: *const c_int,
              work: *mut c_float, lwork: *const c_int, bwork: *mut c_int, info: *mut c_int);
pub fn dgees_(jobvs: *const c_char, sort: *const c_char, select: LAPACK_D_SELECT2,
              n: *const c_int, a: *mut c_double, lda: *const c_int, sdim: *mut c_int,
              wr: *mut c_double, wi: *mut c_double, vs: *mut c_double, ldvs: *const c_int,
              work: *mut c_double, lwork: *const c_int, bwork: *mut c_int, info: *mut c_int);
pub fn cgees_(jobvs: *const c_char, sort: *const c_char, select: LAPACK_C_SELECT1,
              n: *const c_int, a: *mut complex_float, lda: *const c_int, sdim: *mut c_int,
              w: *mut complex_float, vs: *mut complex_float, ldvs: *const c_int,
              work: *mut complex_float, lwork: *const c_int, rwork: *mut c_float,
              bwork: *mut c_int, info: *mut c_int);
pub fn zgees_(jobvs: *const c_char, sort: *const c_char, select: LAPACK_Z_SELECT1,
              n: *const c_int, a: *mut complex_double, lda: *const c_int, sdim: *mut c_int,
              w: *mut complex_double, vs: *mut complex_double, ldvs: *const c_int,
              work: *mut complex_double, lwork: *const c_int, rwork: *mut c_double,
              bwork: *mut c_int, info: *mut c_int);

pub fn sgeesx_(jobvs: *const c_char, sort: *const c_char, select: LAPACK_S_SELECT2,
               sense: *const c_char, n: *const c_int, a: *mut c_float, lda: *const c_int,
               sdim: *mut c_int, wr: *mut c_float, wi: *mut c_float, vs: *mut c_float,
               ldvs: *const c_int, rconde: *mut c_float, rcondv: *mut c_float,
               work: *mut c_float, lwork: *const c_int, iwork: *mut c_int,
               liwork: *const c_int, bwork: *mut c_int, info: *mut c_int);
pub fn dgeesx_(jobvs: *const c_char, sort: *const c_char, select: LAPACK_D_SELECT2,
               sense: *const c_char, n: *const c_int, a: *mut c_double, lda: *const c_int,
               sdim: *mut c_int, wr: *mut c_double, wi: *mut c_double, vs: *mut c_double,
               ldvs: *const c_int, rconde: *mut c_double, rcondv: *mut c_double,
               work: *mut c_double, lwork: *const c_int, iwork: *mut c_int,
               liwork: *const c_int, bwork: *mut c_int, info: *mut c_int);
pub fn cgeesx_(jobvs: *const c_char, sort: *const c_char, select: LAPACK_C_SELECT1,
               sense: *const c_char, n: *const c_int, a: *mut complex_float, lda: *const c_int,
               sdim: *mut c_int, w: *mut complex_float, vs: *mut complex_float,
               ldvs: *const c_int, rconde: *mut c_float, rcondv: *mut c_float,
               work: *mut complex_float, lwork: *const c_int, rwork: *mut c_float,
               bwork: *mut c_int, info: *mut c_int);
pub fn zgeesx_(jobvs: *const c_char, sort: *const c_char, select: LAPACK_Z_SELECT1,
               sense: *const c_char, n: *const c_int, a: *mut complex_double,
               lda: *const c_int, sdim: *mut c_int, w: *mut complex_double,
               vs: *mut complex_double, ldvs: *const c_int, rconde: *mut c_double,
               rcondv: *mut c_double, work: *mut complex_double, lwork: *const c_int,
               rwork: *mut c_double, bwork: *mut c_int, info: *mut c_int);

pub fn sgeev_(jobvl: *const c_char, jobvr: *const c_char, n: *const c_int, a: *mut c_float,
              lda: *const c_int, wr: *mut c_float, wi: *mut c_float, vl: *mut c_float,
              ldvl: *const c_int, vr: *mut c_float, ldvr: *const c_int, work: *mut c_float,
              lwork: *const c_int, info: *mut c_int);
pub fn dgeev_(jobvl: *const c_char, jobvr: *const c_char, n: *const c_int, a: *mut c_double,
              lda: *const c_int, wr: *mut c_double, wi: *mut c_double, vl: *mut c_double,
              ldvl: *const c_int, vr: *mut c_double, ldvr: *const c_int, work: *mut c_double,
              lwork: *const c_int, info: *mut c_int);
pub fn cgeev_(jobvl: *const c_char, jobvr: *const c_char, n: *const c_int,
              a: *mut complex_float, lda: *const c_int, w: *mut complex_float,
              vl: *mut complex_float, ldvl: *const c_int, vr: *mut complex_float,
              ldvr: *const c_int, work: *mut complex_float, lwork: *const c_int,
              rwork: *mut c_float, info: *mut c_int);
pub fn zgeev_(jobvl: *const c_char, jobvr: *const c_char, n: *const c_int,
              a: *mut complex_double, lda: *const c_int, w: *mut complex_double,
              vl: *mut complex_double, ldvl: *const c_int, vr: *mut complex_double,
              ldvr: *const c_int, work: *mut complex_double, lwork: *const c_int,
              rwork: *mut c_double, info: *mut c_int);

pub fn sgeevx_(balanc: *const c_char, jobvl: *const c_char, jobvr: *const c_char,
               sense: *const c_char, n: *const c_int, a: *mut c_float, lda: *const c_int,
               wr: *mut c_float, wi: *mut c_float, vl: *mut c_float, ldvl: *const c_int,
               vr: *mut c_float, ldvr: *const c_int, ilo: *mut c_int, ihi: *mut c_int,
               scale: *mut c_float, abnrm: *mut c_float, rconde: *mut c_float,
               rcondv: *mut c_float, work: *mut c_float, lwork: *const c_int,
               iwork: *mut c_int, info: *mut c_int);
pub fn dgeevx_(balanc: *const c_char, jobvl: *const c_char, jobvr: *const c_char,
               sense: *const c_char, n: *const c_int, a: *mut c_double, lda: *const c_int,
               wr: *mut c_double, wi: *mut c_double, vl: *mut c_double, ldvl: *const c_int,
               vr: *mut c_double, ldvr: *const c_int, ilo: *mut c_int, ihi: *mut c_int,
               scale: *mut c_double, abnrm: *mut c_double, rconde: *mut c_double,
               rcondv: *mut c_double, work: *mut c_double, lwork: *const c_int,
               iwork: *mut c_int, info: *mut c_int);
pub fn cgeevx_(balanc: *const c_char, jobvl: *const c_char, jobvr: *const c_char,
               sense: *const c_char, n: *const c_int, a: *mut complex_float, lda: *const c_int,
               w: *mut complex_float, vl: *mut complex_float, ldvl: *const c_int,
               vr: *mut complex_float, ldvr: *const c_int, ilo: *mut c_int, ihi: *mut c_int,
               scale: *mut c_float, abnrm: *mut c_float, rconde: *mut c_float,
               rcondv: *mut c_float, work: *mut complex_float, lwork: *const c_int,
               rwork: *mut c_float, info: *mut c_int);
pub fn zgeevx_(balanc: *const c_char, jobvl: *const c_char, jobvr: *const c_char,
               sense: *const c_char, n: *const c_int, a: *mut complex_double,
               lda: *const c_int, w: *mut complex_double, vl: *mut complex_double,
               ldvl: *const c_int, vr: *mut complex_double, ldvr: *const c_int,
               ilo: *mut c_int, ihi: *mut c_int, scale: *mut c_double, abnrm: *mut c_double,
               rconde: *mut c_double, rcondv: *mut c_double, work: *mut complex_double,
               lwork: *const c_int, rwork: *mut c_double, info: *mut c_int);

pub fn sgesvd_(jobu: *const c_char, jobvt: *const c_char, m: *const c_int, n: *const c_int,
               a: *mut c_float, lda: *const c_int, s: *mut c_float, u: *mut c_float,
               ldu: *const c_int, vt: *mut c_float, ldvt: *const c_int, work: *mut c_float,
               lwork: *const c_int, info: *mut c_int);
pub fn dgesvd_(jobu: *const c_char, jobvt: *const c_char, m: *const c_int, n: *const c_int,
               a: *mut c_double, lda: *const c_int, s: *mut c_double, u: *mut c_double,
               ldu: *const c_int, vt: *mut c_double, ldvt: *const c_int, work: *mut c_double,
               lwork: *const c_int, info: *mut c_int);
pub fn cgesvd_(jobu: *const c_char, jobvt: *const c_char, m: *const c_int, n: *const c_int,
               a: *mut complex_float, lda: *const c_int, s: *mut c_float,
               u: *mut complex_float, ldu: *const c_int, vt: *mut complex_float,
               ldvt: *const c_int, work: *mut complex_float, lwork: *const c_int,
               rwork: *mut c_float, info: *mut c_int);
pub fn zgesvd_(jobu: *const c_char, jobvt: *const c_char, m: *const c_int, n: *const c_int,
               a: *mut complex_double, lda: *const c_int, s: *mut c_double,
               u: *mut complex_double, ldu: *const c_int, vt: *mut complex_double,
               ldvt: *const c_int, work: *mut complex_double, lwork: *const c_int,
               rwork: *mut c_double, info: *mut c_int);

pub fn sgesdd_(jobz: *const c_char, m: *const c_int, n: *const c_int, a: *mut c_float,
               lda: *const c_int, s: *mut c_float, u: *mut c_float, ldu: *const c_int,
               vt: *mut c_float, ldvt: *const c_int, work: *mut c_float, lwork: *const c_int,
               iwork: *mut c_int, info: *mut c_int);
pub fn dgesdd_(jobz: *const c_char, m: *const c_int, n: *const c_int, a: *mut c_double,
               lda: *const c_int, s: *mut c_double, u: *mut c_double, ldu: *const c_int,
               vt: *mut c_double, ldvt: *const c_int, work: *mut c_double, lwork: *const c_int,
               iwork: *mut c_int, info: *mut c_int);
pub fn cgesdd_(jobz: *const c_char, m: *const c_int, n: *const c_int, a: *mut complex_float,
               lda: *const c_int, s: *mut c_float, u: *mut complex_float, ldu: *const c_int,
               vt: *mut complex_float, ldvt: *const c_int, work: *mut complex_float,
               lwork: *const c_int, rwork: *mut c_float, iwork: *mut c_int, info: *mut c_int);
pub fn zgesdd_(jobz: *const c_char, m: *const c_int, n: *const c_int, a: *mut complex_double,
               lda: *const c_int, s: *mut c_double, u: *mut complex_double, ldu: *const c_int,
               vt: *mut complex_double, ldvt: *const c_int, work: *mut complex_double,
               lwork: *const c_int, rwork: *mut c_double, iwork: *mut c_int, info: *mut c_int);

pub fn dgejsv_(joba: *mut c_char, jobu: *mut c_char, jobv: *mut c_char, jobr: *mut c_char,
               jobt: *mut c_char, jobp: *mut c_char, m: *const c_int, n: *const c_int,
               a: *mut c_double, lda: *const c_int, sva: *mut c_double, u: *mut c_double,
               ldu: *const c_int, v: *mut c_double, ldv: *const c_int, work: *mut c_double,
               lwork: *const c_int, iwork: *mut c_int, info: *mut c_int);
pub fn sgejsv_(joba: *mut c_char, jobu: *mut c_char, jobv: *mut c_char, jobr: *mut c_char,
               jobt: *mut c_char, jobp: *mut c_char, m: *const c_int, n: *const c_int,
               a: *mut c_float, lda: *const c_int, sva: *mut c_float, u: *mut c_float,
               ldu: *const c_int, v: *mut c_float, ldv: *const c_int, work: *mut c_float,
               lwork: *const c_int, iwork: *mut c_int, info: *mut c_int);

pub fn dgesvj_(joba: *const c_char, jobu: *const c_char, jobv: *const c_char, m: *const c_int,
               n: *const c_int, a: *mut c_double, lda: *const c_int, sva: *mut c_double,
               mv: *const c_int, v: *mut c_double, ldv: *const c_int, work: *mut c_double,
               lwork: *const c_int, info: *mut c_int);
pub fn sgesvj_(joba: *const c_char, jobu: *const c_char, jobv: *const c_char, m: *const c_int,
               n: *const c_int, a: *mut c_float, lda: *const c_int, sva: *mut c_float,
               mv: *const c_int, v: *mut c_float, ldv: *const c_int, work: *mut c_float,
               lwork: *const c_int, info: *mut c_int);

pub fn sggsvd_(jobu: *const c_char, jobv: *const c_char, jobq: *const c_char, m: *const c_int,
               n: *const c_int, p: *const c_int, k: *mut c_int, l: *mut c_int, a: *mut c_float,
               lda: *const c_int, b: *mut c_float, ldb: *const c_int, alpha: *mut c_float,
               beta: *mut c_float, u: *mut c_float, ldu: *const c_int, v: *mut c_float,
               ldv: *const c_int, q: *mut c_float, ldq: *const c_int, work: *mut c_float,
               iwork: *mut c_int, info: *mut c_int);
pub fn dggsvd_(jobu: *const c_char, jobv: *const c_char, jobq: *const c_char, m: *const c_int,
               n: *const c_int, p: *const c_int, k: *mut c_int, l: *mut c_int,
               a: *mut c_double, lda: *const c_int, b: *mut c_double, ldb: *const c_int,
               alpha: *mut c_double, beta: *mut c_double, u: *mut c_double, ldu: *const c_int,
               v: *mut c_double, ldv: *const c_int, q: *mut c_double, ldq: *const c_int,
               work: *mut c_double, iwork: *mut c_int, info: *mut c_int);
pub fn cggsvd_(jobu: *const c_char, jobv: *const c_char, jobq: *const c_char, m: *const c_int,
               n: *const c_int, p: *const c_int, k: *mut c_int, l: *mut c_int,
               a: *mut complex_float, lda: *const c_int, b: *mut complex_float,
               ldb: *const c_int, alpha: *mut c_float, beta: *mut c_float,
               u: *mut complex_float, ldu: *const c_int, v: *mut complex_float,
               ldv: *const c_int, q: *mut complex_float, ldq: *const c_int,
               work: *mut complex_float, rwork: *mut c_float, iwork: *mut c_int,
               info: *mut c_int);
pub fn zggsvd_(jobu: *const c_char, jobv: *const c_char, jobq: *const c_char, m: *const c_int,
               n: *const c_int, p: *const c_int, k: *mut c_int, l: *mut c_int,
               a: *mut complex_double, lda: *const c_int, b: *mut complex_double,
               ldb: *const c_int, alpha: *mut c_double, beta: *mut c_double,
               u: *mut complex_double, ldu: *const c_int, v: *mut complex_double,
               ldv: *const c_int, q: *mut complex_double, ldq: *const c_int,
               work: *mut complex_double, rwork: *mut c_double, iwork: *mut c_int,
               info: *mut c_int);

pub fn ssygv_(itype: *const c_int, jobz: *const c_char, uplo: *const c_char, n: *const c_int,
              a: *mut c_float, lda: *const c_int, b: *mut c_float, ldb: *const c_int,
              w: *mut c_float, work: *mut c_float, lwork: *const c_int, info: *mut c_int);
pub fn dsygv_(itype: *const c_int, jobz: *const c_char, uplo: *const c_char, n: *const c_int,
              a: *mut c_double, lda: *const c_int, b: *mut c_double, ldb: *const c_int,
              w: *mut c_double, work: *mut c_double, lwork: *const c_int, info: *mut c_int);
pub fn chegv_(itype: *const c_int, jobz: *const c_char, uplo: *const c_char, n: *const c_int,
              a: *mut complex_float, lda: *const c_int, b: *mut complex_float,
              ldb: *const c_int, w: *mut c_float, work: *mut complex_float,
              lwork: *const c_int, rwork: *mut c_float, info: *mut c_int);
pub fn zhegv_(itype: *const c_int, jobz: *const c_char, uplo: *const c_char, n: *const c_int,
              a: *mut complex_double, lda: *const c_int, b: *mut complex_double,
              ldb: *const c_int, w: *mut c_double, work: *mut complex_double,
              lwork: *const c_int, rwork: *mut c_double, info: *mut c_int);

pub fn ssygvd_(itype: *const c_int, jobz: *const c_char, uplo: *const c_char, n: *const c_int,
               a: *mut c_float, lda: *const c_int, b: *mut c_float, ldb: *const c_int,
               w: *mut c_float, work: *mut c_float, lwork: *const c_int, iwork: *mut c_int,
               liwork: *const c_int, info: *mut c_int);
pub fn dsygvd_(itype: *const c_int, jobz: *const c_char, uplo: *const c_char, n: *const c_int,
               a: *mut c_double, lda: *const c_int, b: *mut c_double, ldb: *const c_int,
               w: *mut c_double, work: *mut c_double, lwork: *const c_int, iwork: *mut c_int,
               liwork: *const c_int, info: *mut c_int);
pub fn chegvd_(itype: *const c_int, jobz: *const c_char, uplo: *const c_char, n: *const c_int,
               a: *mut complex_float, lda: *const c_int, b: *mut complex_float,
               ldb: *const c_int, w: *mut c_float, work: *mut complex_float,
               lwork: *const c_int, rwork: *mut c_float, lrwork: *const c_int,
               iwork: *mut c_int, liwork: *const c_int, info: *mut c_int);
pub fn zhegvd_(itype: *const c_int, jobz: *const c_char, uplo: *const c_char, n: *const c_int,
               a: *mut complex_double, lda: *const c_int, b: *mut complex_double,
               ldb: *const c_int, w: *mut c_double, work: *mut complex_double,
               lwork: *const c_int, rwork: *mut c_double, lrwork: *const c_int,
               iwork: *mut c_int, liwork: *const c_int, info: *mut c_int);

pub fn ssygvx_(itype: *const c_int, jobz: *const c_char, range: *const c_char,
               uplo: *const c_char, n: *const c_int, a: *mut c_float, lda: *const c_int,
               b: *mut c_float, ldb: *const c_int, vl: *const c_float, vu: *const c_float,
               il: *const c_int, iu: *const c_int, abstol: *const c_float, m: *mut c_int,
               w: *mut c_float, z: *mut c_float, ldz: *const c_int, work: *mut c_float,
               lwork: *const c_int, iwork: *mut c_int, ifail: *mut c_int, info: *mut c_int);
pub fn dsygvx_(itype: *const c_int, jobz: *const c_char, range: *const c_char,
               uplo: *const c_char, n: *const c_int, a: *mut c_double, lda: *const c_int,
               b: *mut c_double, ldb: *const c_int, vl: *const c_double, vu: *const c_double,
               il: *const c_int, iu: *const c_int, abstol: *const c_double, m: *mut c_int,
               w: *mut c_double, z: *mut c_double, ldz: *const c_int, work: *mut c_double,
               lwork: *const c_int, iwork: *mut c_int, ifail: *mut c_int, info: *mut c_int);
pub fn chegvx_(itype: *const c_int, jobz: *const c_char, range: *const c_char,
               uplo: *const c_char, n: *const c_int, a: *mut complex_float, lda: *const c_int,
               b: *mut complex_float, ldb: *const c_int, vl: *const c_float,
               vu: *const c_float, il: *const c_int, iu: *const c_int, abstol: *const c_float,
               m: *mut c_int, w: *mut c_float, z: *mut complex_float, ldz: *const c_int,
               work: *mut complex_float, lwork: *const c_int, rwork: *mut c_float,
               iwork: *mut c_int, ifail: *mut c_int, info: *mut c_int);
pub fn zhegvx_(itype: *const c_int, jobz: *const c_char, range: *const c_char,
               uplo: *const c_char, n: *const c_int, a: *mut complex_double, lda: *const c_int,
               b: *mut complex_double, ldb: *const c_int, vl: *const c_double,
               vu: *const c_double, il: *const c_int, iu: *const c_int,
               abstol: *const c_double, m: *mut c_int, w: *mut c_double,
               z: *mut complex_double, ldz: *const c_int, work: *mut complex_double,
               lwork: *const c_int, rwork: *mut c_double, iwork: *mut c_int, ifail: *mut c_int,
               info: *mut c_int);

pub fn sspgv_(itype: *const c_int, jobz: *const c_char, uplo: *const c_char, n: *const c_int,
              ap: *mut c_float, bp: *mut c_float, w: *mut c_float, z: *mut c_float,
              ldz: *const c_int, work: *mut c_float, info: *mut c_int);
pub fn dspgv_(itype: *const c_int, jobz: *const c_char, uplo: *const c_char, n: *const c_int,
              ap: *mut c_double, bp: *mut c_double, w: *mut c_double, z: *mut c_double,
              ldz: *const c_int, work: *mut c_double, info: *mut c_int);
pub fn chpgv_(itype: *const c_int, jobz: *const c_char, uplo: *const c_char, n: *const c_int,
              ap: *mut complex_float, bp: *mut complex_float, w: *mut c_float,
              z: *mut complex_float, ldz: *const c_int, work: *mut complex_float,
              rwork: *mut c_float, info: *mut c_int);
pub fn zhpgv_(itype: *const c_int, jobz: *const c_char, uplo: *const c_char, n: *const c_int,
              ap: *mut complex_double, bp: *mut complex_double, w: *mut c_double,
              z: *mut complex_double, ldz: *const c_int, work: *mut complex_double,
              rwork: *mut c_double, info: *mut c_int);

pub fn sspgvd_(itype: *const c_int, jobz: *const c_char, uplo: *const c_char, n: *const c_int,
               ap: *mut c_float, bp: *mut c_float, w: *mut c_float, z: *mut c_float,
               ldz: *const c_int, work: *mut c_float, lwork: *const c_int, iwork: *mut c_int,
               liwork: *const c_int, info: *mut c_int);
pub fn dspgvd_(itype: *const c_int, jobz: *const c_char, uplo: *const c_char, n: *const c_int,
               ap: *mut c_double, bp: *mut c_double, w: *mut c_double, z: *mut c_double,
               ldz: *const c_int, work: *mut c_double, lwork: *const c_int, iwork: *mut c_int,
               liwork: *const c_int, info: *mut c_int);
pub fn chpgvd_(itype: *const c_int, jobz: *const c_char, uplo: *const c_char, n: *const c_int,
               ap: *mut complex_float, bp: *mut complex_float, w: *mut c_float,
               z: *mut complex_float, ldz: *const c_int, work: *mut complex_float,
               lwork: *const c_int, rwork: *mut c_float, lrwork: *const c_int,
               iwork: *mut c_int, liwork: *const c_int, info: *mut c_int);
pub fn zhpgvd_(itype: *const c_int, jobz: *const c_char, uplo: *const c_char, n: *const c_int,
               ap: *mut complex_double, bp: *mut complex_double, w: *mut c_double,
               z: *mut complex_double, ldz: *const c_int, work: *mut complex_double,
               lwork: *const c_int, rwork: *mut c_double, lrwork: *const c_int,
               iwork: *mut c_int, liwork: *const c_int, info: *mut c_int);

pub fn sspgvx_(itype: *const c_int, jobz: *const c_char, range: *const c_char,
               uplo: *const c_char, n: *const c_int, ap: *mut c_float, bp: *mut c_float,
               vl: *const c_float, vu: *const c_float, il: *const c_int, iu: *const c_int,
               abstol: *const c_float, m: *mut c_int, w: *mut c_float, z: *mut c_float,
               ldz: *const c_int, work: *mut c_float, iwork: *mut c_int, ifail: *mut c_int,
               info: *mut c_int);
pub fn dspgvx_(itype: *const c_int, jobz: *const c_char, range: *const c_char,
               uplo: *const c_char, n: *const c_int, ap: *mut c_double, bp: *mut c_double,
               vl: *const c_double, vu: *const c_double, il: *const c_int, iu: *const c_int,
               abstol: *const c_double, m: *mut c_int, w: *mut c_double, z: *mut c_double,
               ldz: *const c_int, work: *mut c_double, iwork: *mut c_int, ifail: *mut c_int,
               info: *mut c_int);
pub fn chpgvx_(itype: *const c_int, jobz: *const c_char, range: *const c_char,
               uplo: *const c_char, n: *const c_int, ap: *mut complex_float,
               bp: *mut complex_float, vl: *const c_float, vu: *const c_float,
               il: *const c_int, iu: *const c_int, abstol: *const c_float, m: *mut c_int,
               w: *mut c_float, z: *mut complex_float, ldz: *const c_int,
               work: *mut complex_float, rwork: *mut c_float, iwork: *mut c_int,
               ifail: *mut c_int, info: *mut c_int);
pub fn zhpgvx_(itype: *const c_int, jobz: *const c_char, range: *const c_char,
               uplo: *const c_char, n: *const c_int, ap: *mut complex_double,
               bp: *mut complex_double, vl: *const c_double, vu: *const c_double,
               il: *const c_int, iu: *const c_int, abstol: *const c_double, m: *mut c_int,
               w: *mut c_double, z: *mut complex_double, ldz: *const c_int,
               work: *mut complex_double, rwork: *mut c_double, iwork: *mut c_int,
               ifail: *mut c_int, info: *mut c_int);

pub fn ssbgv_(jobz: *const c_char, uplo: *const c_char, n: *const c_int, ka: *const c_int,
              kb: *const c_int, ab: *mut c_float, ldab: *const c_int, bb: *mut c_float,
              ldbb: *const c_int, w: *mut c_float, z: *mut c_float, ldz: *const c_int,
              work: *mut c_float, info: *mut c_int);
pub fn dsbgv_(jobz: *const c_char, uplo: *const c_char, n: *const c_int, ka: *const c_int,
              kb: *const c_int, ab: *mut c_double, ldab: *const c_int, bb: *mut c_double,
              ldbb: *const c_int, w: *mut c_double, z: *mut c_double, ldz: *const c_int,
              work: *mut c_double, info: *mut c_int);
pub fn chbgv_(jobz: *const c_char, uplo: *const c_char, n: *const c_int, ka: *const c_int,
              kb: *const c_int, ab: *mut complex_float, ldab: *const c_int,
              bb: *mut complex_float, ldbb: *const c_int, w: *mut c_float,
              z: *mut complex_float, ldz: *const c_int, work: *mut complex_float,
              rwork: *mut c_float, info: *mut c_int);
pub fn zhbgv_(jobz: *const c_char, uplo: *const c_char, n: *const c_int, ka: *const c_int,
              kb: *const c_int, ab: *mut complex_double, ldab: *const c_int,
              bb: *mut complex_double, ldbb: *const c_int, w: *mut c_double,
              z: *mut complex_double, ldz: *const c_int, work: *mut complex_double,
              rwork: *mut c_double, info: *mut c_int);

pub fn ssbgvd_(jobz: *const c_char, uplo: *const c_char, n: *const c_int, ka: *const c_int,
               kb: *const c_int, ab: *mut c_float, ldab: *const c_int, bb: *mut c_float,
               ldbb: *const c_int, w: *mut c_float, z: *mut c_float, ldz: *const c_int,
               work: *mut c_float, lwork: *const c_int, iwork: *mut c_int,
               liwork: *const c_int, info: *mut c_int);
pub fn dsbgvd_(jobz: *const c_char, uplo: *const c_char, n: *const c_int, ka: *const c_int,
               kb: *const c_int, ab: *mut c_double, ldab: *const c_int, bb: *mut c_double,
               ldbb: *const c_int, w: *mut c_double, z: *mut c_double, ldz: *const c_int,
               work: *mut c_double, lwork: *const c_int, iwork: *mut c_int,
               liwork: *const c_int, info: *mut c_int);
pub fn chbgvd_(jobz: *const c_char, uplo: *const c_char, n: *const c_int, ka: *const c_int,
               kb: *const c_int, ab: *mut complex_float, ldab: *const c_int,
               bb: *mut complex_float, ldbb: *const c_int, w: *mut c_float,
               z: *mut complex_float, ldz: *const c_int, work: *mut complex_float,
               lwork: *const c_int, rwork: *mut c_float, lrwork: *const c_int,
               iwork: *mut c_int, liwork: *const c_int, info: *mut c_int);
pub fn zhbgvd_(jobz: *const c_char, uplo: *const c_char, n: *const c_int, ka: *const c_int,
               kb: *const c_int, ab: *mut complex_double, ldab: *const c_int,
               bb: *mut complex_double, ldbb: *const c_int, w: *mut c_double,
               z: *mut complex_double, ldz: *const c_int, work: *mut complex_double,
               lwork: *const c_int, rwork: *mut c_double, lrwork: *const c_int,
               iwork: *mut c_int, liwork: *const c_int, info: *mut c_int);

pub fn ssbgvx_(jobz: *const c_char, range: *const c_char, uplo: *const c_char, n: *const c_int,
               ka: *const c_int, kb: *const c_int, ab: *mut c_float, ldab: *const c_int,
               bb: *mut c_float, ldbb: *const c_int, q: *mut c_float, ldq: *const c_int,
               vl: *const c_float, vu: *const c_float, il: *const c_int, iu: *const c_int,
               abstol: *const c_float, m: *mut c_int, w: *mut c_float, z: *mut c_float,
               ldz: *const c_int, work: *mut c_float, iwork: *mut c_int, ifail: *mut c_int,
               info: *mut c_int);
pub fn dsbgvx_(jobz: *const c_char, range: *const c_char, uplo: *const c_char, n: *const c_int,
               ka: *const c_int, kb: *const c_int, ab: *mut c_double, ldab: *const c_int,
               bb: *mut c_double, ldbb: *const c_int, q: *mut c_double, ldq: *const c_int,
               vl: *const c_double, vu: *const c_double, il: *const c_int, iu: *const c_int,
               abstol: *const c_double, m: *mut c_int, w: *mut c_double, z: *mut c_double,
               ldz: *const c_int, work: *mut c_double, iwork: *mut c_int, ifail: *mut c_int,
               info: *mut c_int);
pub fn chbgvx_(jobz: *const c_char, range: *const c_char, uplo: *const c_char, n: *const c_int,
               ka: *const c_int, kb: *const c_int, ab: *mut complex_float, ldab: *const c_int,
               bb: *mut complex_float, ldbb: *const c_int, q: *mut complex_float,
               ldq: *const c_int, vl: *const c_float, vu: *const c_float, il: *const c_int,
               iu: *const c_int, abstol: *const c_float, m: *mut c_int, w: *mut c_float,
               z: *mut complex_float, ldz: *const c_int, work: *mut complex_float,
               rwork: *mut c_float, iwork: *mut c_int, ifail: *mut c_int, info: *mut c_int);
pub fn zhbgvx_(jobz: *const c_char, range: *const c_char, uplo: *const c_char, n: *const c_int,
               ka: *const c_int, kb: *const c_int, ab: *mut complex_double, ldab: *const c_int,
               bb: *mut complex_double, ldbb: *const c_int, q: *mut complex_double,
               ldq: *const c_int, vl: *const c_double, vu: *const c_double, il: *const c_int,
               iu: *const c_int, abstol: *const c_double, m: *mut c_int, w: *mut c_double,
               z: *mut complex_double, ldz: *const c_int, work: *mut complex_double,
               rwork: *mut c_double, iwork: *mut c_int, ifail: *mut c_int, info: *mut c_int);

pub fn sgges_(jobvsl: *const c_char, jobvsr: *const c_char, sort: *const c_char, selctg:
              LAPACK_S_SELECT3, n: *const c_int, a: *mut c_float, lda: *const c_int,
              b: *mut c_float, ldb: *const c_int, sdim: *mut c_int, alphar: *mut c_float,
              alphai: *mut c_float, beta: *mut c_float, vsl: *mut c_float, ldvsl: *const c_int,
              vsr: *mut c_float, ldvsr: *const c_int, work: *mut c_float, lwork: *const c_int,
              bwork: *mut c_int, info: *mut c_int);
pub fn dgges_(jobvsl: *const c_char, jobvsr: *const c_char, sort: *const c_char, selctg:
              LAPACK_D_SELECT3, n: *const c_int, a: *mut c_double, lda: *const c_int,
              b: *mut c_double, ldb: *const c_int, sdim: *mut c_int, alphar: *mut c_double,
              alphai: *mut c_double, beta: *mut c_double, vsl: *mut c_double,
              ldvsl: *const c_int, vsr: *mut c_double, ldvsr: *const c_int,
              work: *mut c_double, lwork: *const c_int, bwork: *mut c_int, info: *mut c_int);
pub fn cgges_(jobvsl: *const c_char, jobvsr: *const c_char, sort: *const c_char, selctg:
              LAPACK_C_SELECT2, n: *const c_int, a: *mut complex_float, lda: *const c_int,
              b: *mut complex_float, ldb: *const c_int, sdim: *mut c_int,
              alpha: *mut complex_float, beta: *mut complex_float, vsl: *mut complex_float,
              ldvsl: *const c_int, vsr: *mut complex_float, ldvsr: *const c_int,
              work: *mut complex_float, lwork: *const c_int, rwork: *mut c_float,
              bwork: *mut c_int, info: *mut c_int);
pub fn zgges_(jobvsl: *const c_char, jobvsr: *const c_char, sort: *const c_char, selctg:
              LAPACK_Z_SELECT2, n: *const c_int, a: *mut complex_double, lda: *const c_int,
              b: *mut complex_double, ldb: *const c_int, sdim: *mut c_int,
              alpha: *mut complex_double, beta: *mut complex_double, vsl: *mut complex_double,
              ldvsl: *const c_int, vsr: *mut complex_double, ldvsr: *const c_int,
              work: *mut complex_double, lwork: *const c_int, rwork: *mut c_double,
              bwork: *mut c_int, info: *mut c_int);

pub fn sggesx_(jobvsl: *const c_char, jobvsr: *const c_char, sort: *const c_char, selctg:
               LAPACK_S_SELECT3, sense: *const c_char, n: *const c_int, a: *mut c_float,
               lda: *const c_int, b: *mut c_float, ldb: *const c_int, sdim: *mut c_int,
               alphar: *mut c_float, alphai: *mut c_float, beta: *mut c_float,
               vsl: *mut c_float, ldvsl: *const c_int, vsr: *mut c_float, ldvsr: *const c_int,
               rconde: *mut c_float, rcondv: *mut c_float, work: *mut c_float,
               lwork: *const c_int, iwork: *mut c_int, liwork: *const c_int, bwork: *mut c_int,
               info: *mut c_int);
pub fn dggesx_(jobvsl: *const c_char, jobvsr: *const c_char, sort: *const c_char, selctg:
               LAPACK_D_SELECT3, sense: *const c_char, n: *const c_int, a: *mut c_double,
               lda: *const c_int, b: *mut c_double, ldb: *const c_int, sdim: *mut c_int,
               alphar: *mut c_double, alphai: *mut c_double, beta: *mut c_double,
               vsl: *mut c_double, ldvsl: *const c_int, vsr: *mut c_double,
               ldvsr: *const c_int, rconde: *mut c_double, rcondv: *mut c_double,
               work: *mut c_double, lwork: *const c_int, iwork: *mut c_int,
               liwork: *const c_int, bwork: *mut c_int, info: *mut c_int);
pub fn cggesx_(jobvsl: *const c_char, jobvsr: *const c_char, sort: *const c_char, selctg:
               LAPACK_C_SELECT2, sense: *const c_char, n: *const c_int, a: *mut complex_float,
               lda: *const c_int, b: *mut complex_float, ldb: *const c_int, sdim: *mut c_int,
               alpha: *mut complex_float, beta: *mut complex_float, vsl: *mut complex_float,
               ldvsl: *const c_int, vsr: *mut complex_float, ldvsr: *const c_int,
               rconde: *mut c_float, rcondv: *mut c_float, work: *mut complex_float,
               lwork: *const c_int, rwork: *mut c_float, iwork: *mut c_int,
               liwork: *const c_int, bwork: *mut c_int, info: *mut c_int);
pub fn zggesx_(jobvsl: *const c_char, jobvsr: *const c_char, sort: *const c_char, selctg:
               LAPACK_Z_SELECT2, sense: *const c_char, n: *const c_int, a: *mut complex_double,
               lda: *const c_int, b: *mut complex_double, ldb: *const c_int, sdim: *mut c_int,
               alpha: *mut complex_double, beta: *mut complex_double, vsl: *mut complex_double,
               ldvsl: *const c_int, vsr: *mut complex_double, ldvsr: *const c_int,
               rconde: *mut c_double, rcondv: *mut c_double, work: *mut complex_double,
               lwork: *const c_int, rwork: *mut c_double, iwork: *mut c_int,
               liwork: *const c_int, bwork: *mut c_int, info: *mut c_int);

pub fn sggev_(jobvl: *const c_char, jobvr: *const c_char, n: *const c_int, a: *mut c_float,
              lda: *const c_int, b: *mut c_float, ldb: *const c_int, alphar: *mut c_float,
              alphai: *mut c_float, beta: *mut c_float, vl: *mut c_float, ldvl: *const c_int,
              vr: *mut c_float, ldvr: *const c_int, work: *mut c_float, lwork: *const c_int,
              info: *mut c_int);
pub fn dggev_(jobvl: *const c_char, jobvr: *const c_char, n: *const c_int, a: *mut c_double,
              lda: *const c_int, b: *mut c_double, ldb: *const c_int, alphar: *mut c_double,
              alphai: *mut c_double, beta: *mut c_double, vl: *mut c_double,
              ldvl: *const c_int, vr: *mut c_double, ldvr: *const c_int, work: *mut c_double,
              lwork: *const c_int, info: *mut c_int);
pub fn cggev_(jobvl: *const c_char, jobvr: *const c_char, n: *const c_int,
              a: *mut complex_float, lda: *const c_int, b: *mut complex_float,
              ldb: *const c_int, alpha: *mut complex_float, beta: *mut complex_float,
              vl: *mut complex_float, ldvl: *const c_int, vr: *mut complex_float,
              ldvr: *const c_int, work: *mut complex_float, lwork: *const c_int,
              rwork: *mut c_float, info: *mut c_int);
pub fn zggev_(jobvl: *const c_char, jobvr: *const c_char, n: *const c_int,
              a: *mut complex_double, lda: *const c_int, b: *mut complex_double,
              ldb: *const c_int, alpha: *mut complex_double, beta: *mut complex_double,
              vl: *mut complex_double, ldvl: *const c_int, vr: *mut complex_double,
              ldvr: *const c_int, work: *mut complex_double, lwork: *const c_int,
              rwork: *mut c_double, info: *mut c_int);

pub fn sggevx_(balanc: *const c_char, jobvl: *const c_char, jobvr: *const c_char,
               sense: *const c_char, n: *const c_int, a: *mut c_float, lda: *const c_int,
               b: *mut c_float, ldb: *const c_int, alphar: *mut c_float, alphai: *mut c_float,
               beta: *mut c_float, vl: *mut c_float, ldvl: *const c_int, vr: *mut c_float,
               ldvr: *const c_int, ilo: *mut c_int, ihi: *mut c_int, lscale: *mut c_float,
               rscale: *mut c_float, abnrm: *mut c_float, bbnrm: *mut c_float,
               rconde: *mut c_float, rcondv: *mut c_float, work: *mut c_float,
               lwork: *const c_int, iwork: *mut c_int, bwork: *mut c_int, info: *mut c_int);
pub fn dggevx_(balanc: *const c_char, jobvl: *const c_char, jobvr: *const c_char,
               sense: *const c_char, n: *const c_int, a: *mut c_double, lda: *const c_int,
               b: *mut c_double, ldb: *const c_int, alphar: *mut c_double,
               alphai: *mut c_double, beta: *mut c_double, vl: *mut c_double,
               ldvl: *const c_int, vr: *mut c_double, ldvr: *const c_int, ilo: *mut c_int,
               ihi: *mut c_int, lscale: *mut c_double, rscale: *mut c_double,
               abnrm: *mut c_double, bbnrm: *mut c_double, rconde: *mut c_double,
               rcondv: *mut c_double, work: *mut c_double, lwork: *const c_int,
               iwork: *mut c_int, bwork: *mut c_int, info: *mut c_int);
pub fn cggevx_(balanc: *const c_char, jobvl: *const c_char, jobvr: *const c_char,
               sense: *const c_char, n: *const c_int, a: *mut complex_float, lda: *const c_int,
               b: *mut complex_float, ldb: *const c_int, alpha: *mut complex_float,
               beta: *mut complex_float, vl: *mut complex_float, ldvl: *const c_int,
               vr: *mut complex_float, ldvr: *const c_int, ilo: *mut c_int, ihi: *mut c_int,
               lscale: *mut c_float, rscale: *mut c_float, abnrm: *mut c_float,
               bbnrm: *mut c_float, rconde: *mut c_float, rcondv: *mut c_float,
               work: *mut complex_float, lwork: *const c_int, rwork: *mut c_float,
               iwork: *mut c_int, bwork: *mut c_int, info: *mut c_int);
pub fn zggevx_(balanc: *const c_char, jobvl: *const c_char, jobvr: *const c_char,
               sense: *const c_char, n: *const c_int, a: *mut complex_double,
               lda: *const c_int, b: *mut complex_double, ldb: *const c_int,
               alpha: *mut complex_double, beta: *mut complex_double, vl: *mut complex_double,
               ldvl: *const c_int, vr: *mut complex_double, ldvr: *const c_int,
               ilo: *mut c_int, ihi: *mut c_int, lscale: *mut c_double, rscale: *mut c_double,
               abnrm: *mut c_double, bbnrm: *mut c_double, rconde: *mut c_double,
               rcondv: *mut c_double, work: *mut complex_double, lwork: *const c_int,
               rwork: *mut c_double, iwork: *mut c_int, bwork: *mut c_int, info: *mut c_int);

pub fn ssfrk_(transr: *const c_char, uplo: *const c_char, trans: *const c_char,
              n: *const c_int, k: *const c_int, alpha: *const c_float, a: *const c_float,
              lda: *const c_int, beta: *const c_float, c: *mut c_float);
pub fn dsfrk_(transr: *const c_char, uplo: *const c_char, trans: *const c_char,
              n: *const c_int, k: *const c_int, alpha: *const c_double, a: *const c_double,
              lda: *const c_int, beta: *const c_double, c: *mut c_double);
pub fn chfrk_(transr: *const c_char, uplo: *const c_char, trans: *const c_char,
              n: *const c_int, k: *const c_int, alpha: *const c_float, a: *const complex_float,
              lda: *const c_int, beta: *const c_float, c: *mut complex_float);
pub fn zhfrk_(transr: *const c_char, uplo: *const c_char, trans: *const c_char,
              n: *const c_int, k: *const c_int, alpha: *const c_double,
              a: *const complex_double, lda: *const c_int, beta: *const c_double,
              c: *mut complex_double);

pub fn stfsm_(transr: *const c_char, side: *const c_char, uplo: *const c_char,
              trans: *const c_char, diag: *const c_char, m: *const c_int, n: *const c_int,
              alpha: *const c_float, a: *const c_float, b: *mut c_float, ldb: *const c_int);
pub fn dtfsm_(transr: *const c_char, side: *const c_char, uplo: *const c_char,
              trans: *const c_char, diag: *const c_char, m: *const c_int, n: *const c_int,
              alpha: *const c_double, a: *const c_double, b: *mut c_double, ldb: *const c_int);
pub fn ctfsm_(transr: *const c_char, side: *const c_char, uplo: *const c_char,
              trans: *const c_char, diag: *const c_char, m: *const c_int, n: *const c_int,
              alpha: *const complex_float, a: *const complex_float, b: *mut complex_float,
              ldb: *const c_int);
pub fn ztfsm_(transr: *const c_char, side: *const c_char, uplo: *const c_char,
              trans: *const c_char, diag: *const c_char, m: *const c_int, n: *const c_int,
              alpha: *const complex_double, a: *const complex_double, b: *mut complex_double,
              ldb: *const c_int);

pub fn stfttp_(transr: *const c_char, uplo: *const c_char, n: *const c_int,
               arf: *const c_float, ap: *mut c_float, info: *mut c_int);
pub fn dtfttp_(transr: *const c_char, uplo: *const c_char, n: *const c_int,
               arf: *const c_double, ap: *mut c_double, info: *mut c_int);
pub fn ctfttp_(transr: *const c_char, uplo: *const c_char, n: *const c_int,
               arf: *const complex_float, ap: *mut complex_float, info: *mut c_int);
pub fn ztfttp_(transr: *const c_char, uplo: *const c_char, n: *const c_int,
               arf: *const complex_double, ap: *mut complex_double, info: *mut c_int);

pub fn stfttr_(transr: *const c_char, uplo: *const c_char, n: *const c_int,
               arf: *const c_float, a: *mut c_float, lda: *const c_int, info: *mut c_int);
pub fn dtfttr_(transr: *const c_char, uplo: *const c_char, n: *const c_int,
               arf: *const c_double, a: *mut c_double, lda: *const c_int, info: *mut c_int);
pub fn ctfttr_(transr: *const c_char, uplo: *const c_char, n: *const c_int,
               arf: *const complex_float, a: *mut complex_float, lda: *const c_int,
               info: *mut c_int);
pub fn ztfttr_(transr: *const c_char, uplo: *const c_char, n: *const c_int,
               arf: *const complex_double, a: *mut complex_double, lda: *const c_int,
               info: *mut c_int);

pub fn stpttf_(transr: *const c_char, uplo: *const c_char, n: *const c_int, ap: *const c_float,
               arf: *mut c_float, info: *mut c_int);
pub fn dtpttf_(transr: *const c_char, uplo: *const c_char, n: *const c_int,
               ap: *const c_double, arf: *mut c_double, info: *mut c_int);
pub fn ctpttf_(transr: *const c_char, uplo: *const c_char, n: *const c_int,
               ap: *const complex_float, arf: *mut complex_float, info: *mut c_int);
pub fn ztpttf_(transr: *const c_char, uplo: *const c_char, n: *const c_int,
               ap: *const complex_double, arf: *mut complex_double, info: *mut c_int);

pub fn stpttr_(uplo: *const c_char, n: *const c_int, ap: *const c_float, a: *mut c_float,
               lda: *const c_int, info: *mut c_int);
pub fn dtpttr_(uplo: *const c_char, n: *const c_int, ap: *const c_double, a: *mut c_double,
               lda: *const c_int, info: *mut c_int);
pub fn ctpttr_(uplo: *const c_char, n: *const c_int, ap: *const complex_float,
               a: *mut complex_float, lda: *const c_int, info: *mut c_int);
pub fn ztpttr_(uplo: *const c_char, n: *const c_int, ap: *const complex_double,
               a: *mut complex_double, lda: *const c_int, info: *mut c_int);

pub fn strttf_(transr: *const c_char, uplo: *const c_char, n: *const c_int, a: *const c_float,
               lda: *const c_int, arf: *mut c_float, info: *mut c_int);
pub fn dtrttf_(transr: *const c_char, uplo: *const c_char, n: *const c_int, a: *const c_double,
               lda: *const c_int, arf: *mut c_double, info: *mut c_int);
pub fn ctrttf_(transr: *const c_char, uplo: *const c_char, n: *const c_int,
               a: *const complex_float, lda: *const c_int, arf: *mut complex_float,
               info: *mut c_int);
pub fn ztrttf_(transr: *const c_char, uplo: *const c_char, n: *const c_int,
               a: *const complex_double, lda: *const c_int, arf: *mut complex_double,
               info: *mut c_int);

pub fn strttp_(uplo: *const c_char, n: *const c_int, a: *const c_float, lda: *const c_int,
               ap: *mut c_float, info: *mut c_int);
pub fn dtrttp_(uplo: *const c_char, n: *const c_int, a: *const c_double, lda: *const c_int,
               ap: *mut c_double, info: *mut c_int);
pub fn ctrttp_(uplo: *const c_char, n: *const c_int, a: *const complex_float,
               lda: *const c_int, ap: *mut complex_float, info: *mut c_int);
pub fn ztrttp_(uplo: *const c_char, n: *const c_int, a: *const complex_double,
               lda: *const c_int, ap: *mut complex_double, info: *mut c_int);

pub fn sgeqrfp_(m: *const c_int, n: *const c_int, a: *mut c_float, lda: *const c_int,
                tau: *mut c_float, work: *mut c_float, lwork: *const c_int, info: *mut c_int);
pub fn dgeqrfp_(m: *const c_int, n: *const c_int, a: *mut c_double, lda: *const c_int,
                tau: *mut c_double, work: *mut c_double, lwork: *const c_int,
                info: *mut c_int);
pub fn cgeqrfp_(m: *const c_int, n: *const c_int, a: *mut complex_float, lda: *const c_int,
                tau: *mut complex_float, work: *mut complex_float, lwork: *const c_int,
                info: *mut c_int);
pub fn zgeqrfp_(m: *const c_int, n: *const c_int, a: *mut complex_double, lda: *const c_int,
                tau: *mut complex_double, work: *mut complex_double, lwork: *const c_int,
                info: *mut c_int);

pub fn clacgv_(n: *const c_int, x: *mut complex_float, incx: *const c_int);
pub fn zlacgv_(n: *const c_int, x: *mut complex_double, incx: *const c_int);

pub fn slarnv_(idist: *const c_int, iseed: *mut c_int, n: *const c_int, x: *mut c_float);
pub fn dlarnv_(idist: *const c_int, iseed: *mut c_int, n: *const c_int, x: *mut c_double);
pub fn clarnv_(idist: *const c_int, iseed: *mut c_int, n: *const c_int, x: *mut complex_float);
pub fn zlarnv_(idist: *const c_int, iseed: *mut c_int, n: *const c_int,
               x: *mut complex_double);

pub fn sgeqr2_(m: *const c_int, n: *const c_int, a: *mut c_float, lda: *const c_int,
               tau: *mut c_float, work: *mut c_float, info: *mut c_int);
pub fn dgeqr2_(m: *const c_int, n: *const c_int, a: *mut c_double, lda: *const c_int,
               tau: *mut c_double, work: *mut c_double, info: *mut c_int);
pub fn cgeqr2_(m: *const c_int, n: *const c_int, a: *mut complex_float, lda: *const c_int,
               tau: *mut complex_float, work: *mut complex_float, info: *mut c_int);
pub fn zgeqr2_(m: *const c_int, n: *const c_int, a: *mut complex_double, lda: *const c_int,
               tau: *mut complex_double, work: *mut complex_double, info: *mut c_int);

pub fn slacn2_(n: *const c_int, v: *mut c_float, x: *mut c_float, isgn: *mut c_int,
               est: *mut c_float, kase: *mut c_int, isave: *mut c_int);
pub fn dlacn2_(n: *const c_int, v: *mut c_double, x: *mut c_double, isgn: *mut c_int,
               est: *mut c_double, kase: *mut c_int, isave: *mut c_int);
pub fn clacn2_(n: *const c_int, v: *mut complex_float, x: *mut complex_float,
               est: *mut c_float, kase: *mut c_int, isave: *mut c_int);
pub fn zlacn2_(n: *const c_int, v: *mut complex_double, x: *mut complex_double,
               est: *mut c_double, kase: *mut c_int, isave: *mut c_int);

pub fn slacpy_(uplo: *const c_char, m: *const c_int, n: *const c_int, a: *const c_float,
               lda: *const c_int, b: *mut c_float, ldb: *const c_int);
pub fn dlacpy_(uplo: *const c_char, m: *const c_int, n: *const c_int, a: *const c_double,
               lda: *const c_int, b: *mut c_double, ldb: *const c_int);
pub fn clacpy_(uplo: *const c_char, m: *const c_int, n: *const c_int, a: *const complex_float,
               lda: *const c_int, b: *mut complex_float, ldb: *const c_int);
pub fn zlacpy_(uplo: *const c_char, m: *const c_int, n: *const c_int, a: *const complex_double,
               lda: *const c_int, b: *mut complex_double, ldb: *const c_int);

pub fn clacp2_(uplo: *const c_char, m: *const c_int, n: *const c_int, a: *const c_float,
               lda: *const c_int, b: *mut complex_float, ldb: *const c_int);
pub fn zlacp2_(uplo: *const c_char, m: *const c_int, n: *const c_int, a: *const c_double,
               lda: *const c_int, b: *mut complex_double, ldb: *const c_int);

pub fn sgetf2_(m: *const c_int, n: *const c_int, a: *mut c_float, lda: *const c_int,
               ipiv: *mut c_int, info: *mut c_int);
pub fn dgetf2_(m: *const c_int, n: *const c_int, a: *mut c_double, lda: *const c_int,
               ipiv: *mut c_int, info: *mut c_int);
pub fn cgetf2_(m: *const c_int, n: *const c_int, a: *mut complex_float, lda: *const c_int,
               ipiv: *mut c_int, info: *mut c_int);
pub fn zgetf2_(m: *const c_int, n: *const c_int, a: *mut complex_double, lda: *const c_int,
               ipiv: *mut c_int, info: *mut c_int);

pub fn slaswp_(n: *const c_int, a: *mut c_float, lda: *const c_int, k1: *const c_int,
               k2: *const c_int, ipiv: *const c_int, incx: *const c_int);
pub fn dlaswp_(n: *const c_int, a: *mut c_double, lda: *const c_int, k1: *const c_int,
               k2: *const c_int, ipiv: *const c_int, incx: *const c_int);
pub fn claswp_(n: *const c_int, a: *mut complex_float, lda: *const c_int, k1: *const c_int,
               k2: *const c_int, ipiv: *const c_int, incx: *const c_int);
pub fn zlaswp_(n: *const c_int, a: *mut complex_double, lda: *const c_int, k1: *const c_int,
               k2: *const c_int, ipiv: *const c_int, incx: *const c_int);

pub fn slange_(norm: *const c_char, m: *const c_int, n: *const c_int, a: *const c_float,
               lda: *const c_int, work: *mut c_float) -> c_float;
pub fn dlange_(norm: *const c_char, m: *const c_int, n: *const c_int, a: *const c_double,
               lda: *const c_int, work: *mut c_double) -> c_double;
pub fn clange_(norm: *const c_char, m: *const c_int, n: *const c_int, a: *const complex_float,
               lda: *const c_int, work: *mut c_float) -> c_float;
pub fn zlange_(norm: *const c_char, m: *const c_int, n: *const c_int, a: *const complex_double,
               lda: *const c_int, work: *mut c_double) -> c_double;

pub fn clanhe_(norm: *const c_char, uplo: *const c_char, n: *const c_int,
               a: *const complex_float, lda: *const c_int, work: *mut c_float) -> c_float;
pub fn zlanhe_(norm: *const c_char, uplo: *const c_char, n: *const c_int,
               a: *const complex_double, lda: *const c_int, work: *mut c_double) -> c_double;

pub fn slansy_(norm: *const c_char, uplo: *const c_char, n: *const c_int, a: *const c_float,
               lda: *const c_int, work: *mut c_float) -> c_float;
pub fn dlansy_(norm: *const c_char, uplo: *const c_char, n: *const c_int, a: *const c_double,
               lda: *const c_int, work: *mut c_double) -> c_double;
pub fn clansy_(norm: *const c_char, uplo: *const c_char, n: *const c_int,
               a: *const complex_float, lda: *const c_int, work: *mut c_float) -> c_float;
pub fn zlansy_(norm: *const c_char, uplo: *const c_char, n: *const c_int,
               a: *const complex_double, lda: *const c_int, work: *mut c_double) -> c_double;

pub fn slantr_(norm: *const c_char, uplo: *const c_char, diag: *const c_char, m: *const c_int,
               n: *const c_int, a: *const c_float, lda: *const c_int,
               work: *mut c_float) -> c_float;
pub fn dlantr_(norm: *const c_char, uplo: *const c_char, diag: *const c_char, m: *const c_int,
               n: *const c_int, a: *const c_double, lda: *const c_int,
               work: *mut c_double) -> c_double;
pub fn clantr_(norm: *const c_char, uplo: *const c_char, diag: *const c_char, m: *const c_int,
               n: *const c_int, a: *const complex_float, lda: *const c_int,
               work: *mut c_float) -> c_float;
pub fn zlantr_(norm: *const c_char, uplo: *const c_char, diag: *const c_char, m: *const c_int,
               n: *const c_int, a: *const complex_double, lda: *const c_int,
               work: *mut c_double) -> c_double;

pub fn slamch_(cmach: *const c_char) -> c_float;
pub fn dlamch_(cmach: *const c_char) -> c_double;

pub fn sgelq2_(m: *const c_int, n: *const c_int, a: *mut c_float, lda: *const c_int,
               tau: *mut c_float, work: *mut c_float, info: *mut c_int);
pub fn dgelq2_(m: *const c_int, n: *const c_int, a: *mut c_double, lda: *const c_int,
               tau: *mut c_double, work: *mut c_double, info: *mut c_int);
pub fn cgelq2_(m: *const c_int, n: *const c_int, a: *mut complex_float, lda: *const c_int,
               tau: *mut complex_float, work: *mut complex_float, info: *mut c_int);
pub fn zgelq2_(m: *const c_int, n: *const c_int, a: *mut complex_double, lda: *const c_int,
               tau: *mut complex_double, work: *mut complex_double, info: *mut c_int);

pub fn slarfb_(side: *const c_char, trans: *const c_char, direct: *const c_char,
               storev: *const c_char, m: *const c_int, n: *const c_int, k: *const c_int,
               v: *const c_float, ldv: *const c_int, t: *const c_float, ldt: *const c_int,
               c: *mut c_float, ldc: *const c_int, work: *mut c_float, ldwork: *const c_int);
pub fn dlarfb_(side: *const c_char, trans: *const c_char, direct: *const c_char,
               storev: *const c_char, m: *const c_int, n: *const c_int, k: *const c_int,
               v: *const c_double, ldv: *const c_int, t: *const c_double, ldt: *const c_int,
               c: *mut c_double, ldc: *const c_int, work: *mut c_double, ldwork: *const c_int);
pub fn clarfb_(side: *const c_char, trans: *const c_char, direct: *const c_char,
               storev: *const c_char, m: *const c_int, n: *const c_int, k: *const c_int,
               v: *const complex_float, ldv: *const c_int, t: *const complex_float,
               ldt: *const c_int, c: *mut complex_float, ldc: *const c_int,
               work: *mut complex_float, ldwork: *const c_int);
pub fn zlarfb_(side: *const c_char, trans: *const c_char, direct: *const c_char,
               storev: *const c_char, m: *const c_int, n: *const c_int, k: *const c_int,
               v: *const complex_double, ldv: *const c_int, t: *const complex_double,
               ldt: *const c_int, c: *mut complex_double, ldc: *const c_int,
               work: *mut complex_double, ldwork: *const c_int);

pub fn slarfg_(n: *const c_int, alpha: *mut c_float, x: *mut c_float, incx: *const c_int,
               tau: *mut c_float);
pub fn dlarfg_(n: *const c_int, alpha: *mut c_double, x: *mut c_double, incx: *const c_int,
               tau: *mut c_double);
pub fn clarfg_(n: *const c_int, alpha: *mut complex_float, x: *mut complex_float,
               incx: *const c_int, tau: *mut complex_float);
pub fn zlarfg_(n: *const c_int, alpha: *mut complex_double, x: *mut complex_double,
               incx: *const c_int, tau: *mut complex_double);

pub fn slarft_(direct: *const c_char, storev: *const c_char, n: *const c_int, k: *const c_int,
               v: *mut c_float, ldv: *const c_int, tau: *const c_float, t: *mut c_float,
               ldt: *const c_int);
pub fn dlarft_(direct: *const c_char, storev: *const c_char, n: *const c_int, k: *const c_int,
               v: *mut c_double, ldv: *const c_int, tau: *const c_double, t: *mut c_double,
               ldt: *const c_int);
pub fn clarft_(direct: *const c_char, storev: *const c_char, n: *const c_int, k: *const c_int,
               v: *mut complex_float, ldv: *const c_int, tau: *const complex_float,
               t: *mut complex_float, ldt: *const c_int);
pub fn zlarft_(direct: *const c_char, storev: *const c_char, n: *const c_int, k: *const c_int,
               v: *mut complex_double, ldv: *const c_int, tau: *const complex_double,
               t: *mut complex_double, ldt: *const c_int);

pub fn slarfx_(side: *const c_char, m: *const c_int, n: *const c_int, v: *const c_float,
               tau: *const c_float, c: *mut c_float, ldc: *const c_int, work: *mut c_float);
pub fn dlarfx_(side: *const c_char, m: *const c_int, n: *const c_int, v: *const c_double,
               tau: *const c_double, c: *mut c_double, ldc: *const c_int, work: *mut c_double);
pub fn clarfx_(side: *const c_char, m: *const c_int, n: *const c_int, v: *const complex_float,
               tau: *const complex_float, c: *mut complex_float, ldc: *const c_int,
               work: *mut complex_float);
pub fn zlarfx_(side: *const c_char, m: *const c_int, n: *const c_int, v: *const complex_double,
               tau: *const complex_double, c: *mut complex_double, ldc: *const c_int,
               work: *mut complex_double);

pub fn slatms_(m: *const c_int, n: *const c_int, dist: *const c_char, iseed: *mut c_int,
               sym: *const c_char, d: *mut c_float, mode: *const c_int, cond: *const c_float,
               dmax: *const c_float, kl: *const c_int, ku: *const c_int, pack: *const c_char,
               a: *mut c_float, lda: *const c_int, work: *mut c_float, info: *mut c_int);
pub fn dlatms_(m: *const c_int, n: *const c_int, dist: *const c_char, iseed: *mut c_int,
               sym: *const c_char, d: *mut c_double, mode: *const c_int, cond: *const c_double,
               dmax: *const c_double, kl: *const c_int, ku: *const c_int, pack: *const c_char,
               a: *mut c_double, lda: *const c_int, work: *mut c_double, info: *mut c_int);
pub fn clatms_(m: *const c_int, n: *const c_int, dist: *const c_char, iseed: *mut c_int,
               sym: *const c_char, d: *mut c_float, mode: *const c_int, cond: *const c_float,
               dmax: *const c_float, kl: *const c_int, ku: *const c_int, pack: *const c_char,
               a: *mut complex_float, lda: *const c_int, work: *mut complex_float,
               info: *mut c_int);
pub fn zlatms_(m: *const c_int, n: *const c_int, dist: *const c_char, iseed: *mut c_int,
               sym: *const c_char, d: *mut c_double, mode: *const c_int, cond: *const c_double,
               dmax: *const c_double, kl: *const c_int, ku: *const c_int, pack: *const c_char,
               a: *mut complex_double, lda: *const c_int, work: *mut complex_double,
               info: *mut c_int);

pub fn dlag2s_(m: *const c_int, n: *const c_int, a: *const c_double, lda: *const c_int,
               sa: *mut c_float, ldsa: *const c_int, info: *mut c_int);

pub fn slag2d_(m: *const c_int, n: *const c_int, sa: *const c_float, ldsa: *const c_int,
               a: *mut c_double, lda: *const c_int, info: *mut c_int);

pub fn zlag2c_(m: *const c_int, n: *const c_int, a: *const complex_double, lda: *const c_int,
               sa: *mut complex_float, ldsa: *const c_int, info: *mut c_int);

pub fn clag2z_(m: *const c_int, n: *const c_int, sa: *const complex_float, ldsa: *const c_int,
               a: *mut complex_double, lda: *const c_int, info: *mut c_int);

pub fn slauum_(uplo: *const c_char, n: *const c_int, a: *mut c_float, lda: *const c_int,
               info: *mut c_int);
pub fn dlauum_(uplo: *const c_char, n: *const c_int, a: *mut c_double, lda: *const c_int,
               info: *mut c_int);
pub fn clauum_(uplo: *const c_char, n: *const c_int, a: *mut complex_float, lda: *const c_int,
               info: *mut c_int);
pub fn zlauum_(uplo: *const c_char, n: *const c_int, a: *mut complex_double, lda: *const c_int,
               info: *mut c_int);

pub fn slagge_(m: *const c_int, n: *const c_int, kl: *const c_int, ku: *const c_int,
               d: *const c_float, a: *mut c_float, lda: *const c_int, iseed: *mut c_int,
               work: *mut c_float, info: *mut c_int);
pub fn dlagge_(m: *const c_int, n: *const c_int, kl: *const c_int, ku: *const c_int,
               d: *const c_double, a: *mut c_double, lda: *const c_int, iseed: *mut c_int,
               work: *mut c_double, info: *mut c_int);
pub fn clagge_(m: *const c_int, n: *const c_int, kl: *const c_int, ku: *const c_int,
               d: *const c_float, a: *mut complex_float, lda: *const c_int, iseed: *mut c_int,
               work: *mut complex_float, info: *mut c_int);
pub fn zlagge_(m: *const c_int, n: *const c_int, kl: *const c_int, ku: *const c_int,
               d: *const c_double, a: *mut complex_double, lda: *const c_int,
               iseed: *mut c_int, work: *mut complex_double, info: *mut c_int);

pub fn slaset_(uplo: *const c_char, m: *const c_int, n: *const c_int, alpha: *const c_float,
               beta: *const c_float, a: *mut c_float, lda: *const c_int);
pub fn dlaset_(uplo: *const c_char, m: *const c_int, n: *const c_int, alpha: *const c_double,
               beta: *const c_double, a: *mut c_double, lda: *const c_int);
pub fn claset_(uplo: *const c_char, m: *const c_int, n: *const c_int,
               alpha: *const complex_float, beta: *const complex_float, a: *mut complex_float,
               lda: *const c_int);
pub fn zlaset_(uplo: *const c_char, m: *const c_int, n: *const c_int,
               alpha: *const complex_double, beta: *const complex_double,
               a: *mut complex_double, lda: *const c_int);

pub fn slasrt_(id: *const c_char, n: *const c_int, d: *mut c_float, info: *mut c_int);
pub fn dlasrt_(id: *const c_char, n: *const c_int, d: *mut c_double, info: *mut c_int);

pub fn claghe_(n: *const c_int, k: *const c_int, d: *const c_float, a: *mut complex_float,
               lda: *const c_int, iseed: *mut c_int, work: *mut complex_float,
               info: *mut c_int);
pub fn zlaghe_(n: *const c_int, k: *const c_int, d: *const c_double, a: *mut complex_double,
               lda: *const c_int, iseed: *mut c_int, work: *mut complex_double,
               info: *mut c_int);

pub fn slagsy_(n: *const c_int, k: *const c_int, d: *const c_float, a: *mut c_float,
               lda: *const c_int, iseed: *mut c_int, work: *mut c_float, info: *mut c_int);
pub fn dlagsy_(n: *const c_int, k: *const c_int, d: *const c_double, a: *mut c_double,
               lda: *const c_int, iseed: *mut c_int, work: *mut c_double, info: *mut c_int);
pub fn clagsy_(n: *const c_int, k: *const c_int, d: *const c_float, a: *mut complex_float,
               lda: *const c_int, iseed: *mut c_int, work: *mut complex_float,
               info: *mut c_int);
pub fn zlagsy_(n: *const c_int, k: *const c_int, d: *const c_double, a: *mut complex_double,
               lda: *const c_int, iseed: *mut c_int, work: *mut complex_double,
               info: *mut c_int);

pub fn slapmr_(forwrd: *const c_int, m: *const c_int, n: *const c_int, x: *mut c_float,
               ldx: *const c_int, k: *mut c_int);
pub fn dlapmr_(forwrd: *const c_int, m: *const c_int, n: *const c_int, x: *mut c_double,
               ldx: *const c_int, k: *mut c_int);
pub fn clapmr_(forwrd: *const c_int, m: *const c_int, n: *const c_int, x: *mut complex_float,
               ldx: *const c_int, k: *mut c_int);
pub fn zlapmr_(forwrd: *const c_int, m: *const c_int, n: *const c_int, x: *mut complex_double,
               ldx: *const c_int, k: *mut c_int);

pub fn slapy2_(x: *const c_float, y: *const c_float) -> c_float;
pub fn dlapy2_(x: *const c_double, y: *const c_double) -> c_double;

pub fn slapy3_(x: *const c_float, y: *const c_float, z: *const c_float) -> c_float;
pub fn dlapy3_(x: *const c_double, y: *const c_double, z: *const c_double) -> c_double;

pub fn slartgp_(f: *const c_float, g: *const c_float, cs: *mut c_float, sn: *mut c_float,
                r: *mut c_float);
pub fn dlartgp_(f: *const c_double, g: *const c_double, cs: *mut c_double, sn: *mut c_double,
                r: *mut c_double);

pub fn slartgs_(x: *const c_float, y: *const c_float, sigma: *const c_float, cs: *mut c_float,
                sn: *mut c_float);
pub fn dlartgs_(x: *const c_double, y: *const c_double, sigma: *const c_double,
                cs: *mut c_double, sn: *mut c_double);

pub fn sbbcsd_(jobu1: *const c_char, jobu2: *const c_char, jobv1t: *const c_char,
               jobv2t: *const c_char, trans: *const c_char, m: *const c_int, p: *const c_int,
               q: *const c_int, theta: *mut c_float, phi: *mut c_float, u1: *mut c_float,
               ldu1: *const c_int, u2: *mut c_float, ldu2: *const c_int, v1t: *mut c_float,
               ldv1t: *const c_int, v2t: *mut c_float, ldv2t: *const c_int, b11d: *mut c_float,
               b11e: *mut c_float, b12d: *mut c_float, b12e: *mut c_float, b21d: *mut c_float,
               b21e: *mut c_float, b22d: *mut c_float, b22e: *mut c_float, work: *mut c_float,
               lwork: *const c_int, info: *mut c_int);
pub fn dbbcsd_(jobu1: *const c_char, jobu2: *const c_char, jobv1t: *const c_char,
               jobv2t: *const c_char, trans: *const c_char, m: *const c_int, p: *const c_int,
               q: *const c_int, theta: *mut c_double, phi: *mut c_double, u1: *mut c_double,
               ldu1: *const c_int, u2: *mut c_double, ldu2: *const c_int, v1t: *mut c_double,
               ldv1t: *const c_int, v2t: *mut c_double, ldv2t: *const c_int,
               b11d: *mut c_double, b11e: *mut c_double, b12d: *mut c_double,
               b12e: *mut c_double, b21d: *mut c_double, b21e: *mut c_double,
               b22d: *mut c_double, b22e: *mut c_double, work: *mut c_double,
               lwork: *const c_int, info: *mut c_int);
pub fn cbbcsd_(jobu1: *const c_char, jobu2: *const c_char, jobv1t: *const c_char,
               jobv2t: *const c_char, trans: *const c_char, m: *const c_int, p: *const c_int,
               q: *const c_int, theta: *mut c_float, phi: *mut c_float, u1: *mut complex_float,
               ldu1: *const c_int, u2: *mut complex_float, ldu2: *const c_int,
               v1t: *mut complex_float, ldv1t: *const c_int, v2t: *mut complex_float,
               ldv2t: *const c_int, b11d: *mut c_float, b11e: *mut c_float, b12d: *mut c_float,
               b12e: *mut c_float, b21d: *mut c_float, b21e: *mut c_float, b22d: *mut c_float,
               b22e: *mut c_float, rwork: *mut c_float, lrwork: *const c_int,
               info: *mut c_int);
pub fn zbbcsd_(jobu1: *const c_char, jobu2: *const c_char, jobv1t: *const c_char,
               jobv2t: *const c_char, trans: *const c_char, m: *const c_int, p: *const c_int,
               q: *const c_int, theta: *mut c_double, phi: *mut c_double,
               u1: *mut complex_double, ldu1: *const c_int, u2: *mut complex_double,
               ldu2: *const c_int, v1t: *mut complex_double, ldv1t: *const c_int,
               v2t: *mut complex_double, ldv2t: *const c_int, b11d: *mut c_double,
               b11e: *mut c_double, b12d: *mut c_double, b12e: *mut c_double,
               b21d: *mut c_double, b21e: *mut c_double, b22d: *mut c_double,
               b22e: *mut c_double, rwork: *mut c_double, lrwork: *const c_int,
               info: *mut c_int);

pub fn cheswapr_(uplo: *const c_char, n: *const c_int, a: *mut complex_float, i1: *const c_int,
                 i2: *const c_int);
pub fn zheswapr_(uplo: *const c_char, n: *const c_int, a: *mut complex_double,
                 i1: *const c_int, i2: *const c_int);

pub fn chetri2_(uplo: *const c_char, n: *const c_int, a: *mut complex_float, lda: *const c_int,
                ipiv: *const c_int, work: *mut complex_float, lwork: *const c_int,
                info: *mut c_int);
pub fn zhetri2_(uplo: *const c_char, n: *const c_int, a: *mut complex_double,
                lda: *const c_int, ipiv: *const c_int, work: *mut complex_double,
                lwork: *const c_int, info: *mut c_int);

pub fn chetri2x_(uplo: *const c_char, n: *const c_int, a: *mut complex_float,
                 lda: *const c_int, ipiv: *const c_int, work: *mut complex_float,
                 nb: *const c_int, info: *mut c_int);
pub fn zhetri2x_(uplo: *const c_char, n: *const c_int, a: *mut complex_double,
                 lda: *const c_int, ipiv: *const c_int, work: *mut complex_double,
                 nb: *const c_int, info: *mut c_int);

pub fn chetrs2_(uplo: *const c_char, n: *const c_int, nrhs: *const c_int,
                a: *const complex_float, lda: *const c_int, ipiv: *const c_int,
                b: *mut complex_float, ldb: *const c_int, work: *mut complex_float,
                info: *mut c_int);
pub fn zhetrs2_(uplo: *const c_char, n: *const c_int, nrhs: *const c_int,
                a: *const complex_double, lda: *const c_int, ipiv: *const c_int,
                b: *mut complex_double, ldb: *const c_int, work: *mut complex_double,
                info: *mut c_int);

pub fn ssyconv_(uplo: *const c_char, way: *const c_char, n: *const c_int, a: *mut c_float,
                lda: *const c_int, ipiv: *const c_int, work: *mut c_float, info: *mut c_int);
pub fn dsyconv_(uplo: *const c_char, way: *const c_char, n: *const c_int, a: *mut c_double,
                lda: *const c_int, ipiv: *const c_int, work: *mut c_double, info: *mut c_int);
pub fn csyconv_(uplo: *const c_char, way: *const c_char, n: *const c_int,
                a: *mut complex_float, lda: *const c_int, ipiv: *const c_int,
                work: *mut complex_float, info: *mut c_int);
pub fn zsyconv_(uplo: *const c_char, way: *const c_char, n: *const c_int,
                a: *mut complex_double, lda: *const c_int, ipiv: *const c_int,
                work: *mut complex_double, info: *mut c_int);

pub fn ssyswapr_(uplo: *const c_char, n: *const c_int, a: *mut c_float, i1: *const c_int,
                 i2: *const c_int);
pub fn dsyswapr_(uplo: *const c_char, n: *const c_int, a: *mut c_double, i1: *const c_int,
                 i2: *const c_int);
pub fn csyswapr_(uplo: *const c_char, n: *const c_int, a: *mut complex_float, i1: *const c_int,
                 i2: *const c_int);
pub fn zsyswapr_(uplo: *const c_char, n: *const c_int, a: *mut complex_double,
                 i1: *const c_int, i2: *const c_int);

pub fn ssytri2_(uplo: *const c_char, n: *const c_int, a: *mut c_float, lda: *const c_int,
                ipiv: *const c_int, work: *mut complex_float, lwork: *const c_int,
                info: *mut c_int);
pub fn dsytri2_(uplo: *const c_char, n: *const c_int, a: *mut c_double, lda: *const c_int,
                ipiv: *const c_int, work: *mut complex_double, lwork: *const c_int,
                info: *mut c_int);
pub fn csytri2_(uplo: *const c_char, n: *const c_int, a: *mut complex_float, lda: *const c_int,
                ipiv: *const c_int, work: *mut complex_float, lwork: *const c_int,
                info: *mut c_int);
pub fn zsytri2_(uplo: *const c_char, n: *const c_int, a: *mut complex_double,
                lda: *const c_int, ipiv: *const c_int, work: *mut complex_double,
                lwork: *const c_int, info: *mut c_int);

pub fn ssytri2x_(uplo: *const c_char, n: *const c_int, a: *mut c_float, lda: *const c_int,
                 ipiv: *const c_int, work: *mut c_float, nb: *const c_int, info: *mut c_int);
pub fn dsytri2x_(uplo: *const c_char, n: *const c_int, a: *mut c_double, lda: *const c_int,
                 ipiv: *const c_int, work: *mut c_double, nb: *const c_int, info: *mut c_int);
pub fn csytri2x_(uplo: *const c_char, n: *const c_int, a: *mut complex_float,
                 lda: *const c_int, ipiv: *const c_int, work: *mut complex_float,
                 nb: *const c_int, info: *mut c_int);
pub fn zsytri2x_(uplo: *const c_char, n: *const c_int, a: *mut complex_double,
                 lda: *const c_int, ipiv: *const c_int, work: *mut complex_double,
                 nb: *const c_int, info: *mut c_int);

pub fn ssytrs2_(uplo: *const c_char, n: *const c_int, nrhs: *const c_int, a: *const c_float,
                lda: *const c_int, ipiv: *const c_int, b: *mut c_float, ldb: *const c_int,
                work: *mut c_float, info: *mut c_int);
pub fn dsytrs2_(uplo: *const c_char, n: *const c_int, nrhs: *const c_int, a: *const c_double,
                lda: *const c_int, ipiv: *const c_int, b: *mut c_double, ldb: *const c_int,
                work: *mut c_double, info: *mut c_int);
pub fn csytrs2_(uplo: *const c_char, n: *const c_int, nrhs: *const c_int,
                a: *const complex_float, lda: *const c_int, ipiv: *const c_int,
                b: *mut complex_float, ldb: *const c_int, work: *mut complex_float,
                info: *mut c_int);
pub fn zsytrs2_(uplo: *const c_char, n: *const c_int, nrhs: *const c_int,
                a: *const complex_double, lda: *const c_int, ipiv: *const c_int,
                b: *mut complex_double, ldb: *const c_int, work: *mut complex_double,
                info: *mut c_int);

pub fn cunbdb_(trans: *const c_char, signs: *const c_char, m: *const c_int, p: *const c_int,
               q: *const c_int, x11: *mut complex_float, ldx11: *const c_int,
               x12: *mut complex_float, ldx12: *const c_int, x21: *mut complex_float,
               ldx21: *const c_int, x22: *mut complex_float, ldx22: *const c_int,
               theta: *mut c_float, phi: *mut c_float, taup1: *mut complex_float,
               taup2: *mut complex_float, tauq1: *mut complex_float, tauq2: *mut complex_float,
               work: *mut complex_float, lwork: *const c_int, info: *mut c_int);
pub fn zunbdb_(trans: *const c_char, signs: *const c_char, m: *const c_int, p: *const c_int,
               q: *const c_int, x11: *mut complex_double, ldx11: *const c_int,
               x12: *mut complex_double, ldx12: *const c_int, x21: *mut complex_double,
               ldx21: *const c_int, x22: *mut complex_double, ldx22: *const c_int,
               theta: *mut c_double, phi: *mut c_double, taup1: *mut complex_double,
               taup2: *mut complex_double, tauq1: *mut complex_double,
               tauq2: *mut complex_double, work: *mut complex_double, lwork: *const c_int,
               info: *mut c_int);

pub fn cuncsd_(jobu1: *const c_char, jobu2: *const c_char, jobv1t: *const c_char,
               jobv2t: *const c_char, trans: *const c_char, signs: *const c_char,
               m: *const c_int, p: *const c_int, q: *const c_int, x11: *mut complex_float,
               ldx11: *mut c_int, x12: *mut complex_float, ldx12: *mut c_int,
               x21: *mut complex_float, ldx21: *mut c_int, x22: *mut complex_float,
               ldx22: *mut c_int, theta: *mut c_float, u1: *mut complex_float,
               ldu1: *const c_int, u2: *mut complex_float, ldu2: *const c_int,
               v1t: *mut complex_float, ldv1t: *const c_int, v2t: *mut complex_float,
               ldv2t: *const c_int, work: *mut complex_float, lwork: *const c_int,
               rwork: *mut c_float, lrwork: *const c_int, iwork: *mut c_int, info: *mut c_int);
pub fn zuncsd_(jobu1: *const c_char, jobu2: *const c_char, jobv1t: *const c_char,
               jobv2t: *const c_char, trans: *const c_char, signs: *const c_char,
               m: *const c_int, p: *const c_int, q: *const c_int, x11: *mut complex_double,
               ldx11: *mut c_int, x12: *mut complex_double, ldx12: *mut c_int,
               x21: *mut complex_double, ldx21: *mut c_int, x22: *mut complex_double,
               ldx22: *mut c_int, theta: *mut c_double, u1: *mut complex_double,
               ldu1: *const c_int, u2: *mut complex_double, ldu2: *const c_int,
               v1t: *mut complex_double, ldv1t: *const c_int, v2t: *mut complex_double,
               ldv2t: *const c_int, work: *mut complex_double, lwork: *const c_int,
               rwork: *mut c_double, lrwork: *const c_int, iwork: *mut c_int,
               info: *mut c_int);

pub fn sorbdb_(trans: *const c_char, signs: *const c_char, m: *const c_int, p: *const c_int,
               q: *const c_int, x11: *mut c_float, ldx11: *const c_int, x12: *mut c_float,
               ldx12: *const c_int, x21: *mut c_float, ldx21: *const c_int, x22: *mut c_float,
               ldx22: *const c_int, theta: *mut c_float, phi: *mut c_float,
               taup1: *mut c_float, taup2: *mut c_float, tauq1: *mut c_float,
               tauq2: *mut c_float, work: *mut c_float, lwork: *const c_int, info: *mut c_int);
pub fn dorbdb_(trans: *const c_char, signs: *const c_char, m: *const c_int, p: *const c_int,
               q: *const c_int, x11: *mut c_double, ldx11: *const c_int, x12: *mut c_double,
               ldx12: *const c_int, x21: *mut c_double, ldx21: *const c_int,
               x22: *mut c_double, ldx22: *const c_int, theta: *mut c_double,
               phi: *mut c_double, taup1: *mut c_double, taup2: *mut c_double,
               tauq1: *mut c_double, tauq2: *mut c_double, work: *mut c_double,
               lwork: *const c_int, info: *mut c_int);

pub fn sorcsd_(jobu1: *const c_char, jobu2: *const c_char, jobv1t: *const c_char,
               jobv2t: *const c_char, trans: *const c_char, signs: *const c_char,
               m: *const c_int, p: *const c_int, q: *const c_int, x11: *mut c_float,
               ldx11: *const c_int, x12: *mut c_float, ldx12: *const c_int, x21: *mut c_float,
               ldx21: *const c_int, x22: *mut c_float, ldx22: *const c_int,
               theta: *mut c_float, u1: *mut c_float, ldu1: *const c_int, u2: *mut c_float,
               ldu2: *const c_int, v1t: *mut c_float, ldv1t: *const c_int, v2t: *mut c_float,
               ldv2t: *const c_int, work: *mut c_float, lwork: *const c_int, iwork: *mut c_int,
               info: *mut c_int);
pub fn dorcsd_(jobu1: *const c_char, jobu2: *const c_char, jobv1t: *const c_char,
               jobv2t: *const c_char, trans: *const c_char, signs: *const c_char,
               m: *const c_int, p: *const c_int, q: *const c_int, x11: *mut c_double,
               ldx11: *const c_int, x12: *mut c_double, ldx12: *const c_int,
               x21: *mut c_double, ldx21: *const c_int, x22: *mut c_double,
               ldx22: *const c_int, theta: *mut c_double, u1: *mut c_double,
               ldu1: *const c_int, u2: *mut c_double, ldu2: *const c_int, v1t: *mut c_double,
               ldv1t: *const c_int, v2t: *mut c_double, ldv2t: *const c_int,
               work: *mut c_double, lwork: *const c_int, iwork: *mut c_int, info: *mut c_int);

pub fn sgemqrt_(side: *const c_char, trans: *const c_char, m: *const c_int, n: *const c_int,
                k: *const c_int, nb: *const c_int, v: *const c_float, ldv: *const c_int,
                t: *const c_float, ldt: *const c_int, c: *mut c_float, ldc: *const c_int,
                work: *mut c_float, info: *mut c_int);
pub fn dgemqrt_(side: *const c_char, trans: *const c_char, m: *const c_int, n: *const c_int,
                k: *const c_int, nb: *const c_int, v: *const c_double, ldv: *const c_int,
                t: *const c_double, ldt: *const c_int, c: *mut c_double, ldc: *const c_int,
                work: *mut c_double, info: *mut c_int);
pub fn cgemqrt_(side: *const c_char, trans: *const c_char, m: *const c_int, n: *const c_int,
                k: *const c_int, nb: *const c_int, v: *const complex_float, ldv: *const c_int,
                t: *const complex_float, ldt: *const c_int, c: *mut complex_float,
                ldc: *const c_int, work: *mut complex_float, info: *mut c_int);
pub fn zgemqrt_(side: *const c_char, trans: *const c_char, m: *const c_int, n: *const c_int,
                k: *const c_int, nb: *const c_int, v: *const complex_double, ldv: *const c_int,
                t: *const complex_double, ldt: *const c_int, c: *mut complex_double,
                ldc: *const c_int, work: *mut complex_double, info: *mut c_int);

pub fn sgeqrt_(m: *const c_int, n: *const c_int, nb: *const c_int, a: *mut c_float,
               lda: *const c_int, t: *mut c_float, ldt: *const c_int, work: *mut c_float,
               info: *mut c_int);
pub fn dgeqrt_(m: *const c_int, n: *const c_int, nb: *const c_int, a: *mut c_double,
               lda: *const c_int, t: *mut c_double, ldt: *const c_int, work: *mut c_double,
               info: *mut c_int);
pub fn cgeqrt_(m: *const c_int, n: *const c_int, nb: *const c_int, a: *mut complex_float,
               lda: *const c_int, t: *mut complex_float, ldt: *const c_int,
               work: *mut complex_float, info: *mut c_int);
pub fn zgeqrt_(m: *const c_int, n: *const c_int, nb: *const c_int, a: *mut complex_double,
               lda: *const c_int, t: *mut complex_double, ldt: *const c_int,
               work: *mut complex_double, info: *mut c_int);

pub fn sgeqrt2_(m: *const c_int, n: *const c_int, a: *mut c_float, lda: *const c_int,
                t: *mut c_float, ldt: *const c_int, info: *mut c_int);
pub fn dgeqrt2_(m: *const c_int, n: *const c_int, a: *mut c_double, lda: *const c_int,
                t: *mut c_double, ldt: *const c_int, info: *mut c_int);
pub fn cgeqrt2_(m: *const c_int, n: *const c_int, a: *mut complex_float, lda: *const c_int,
                t: *mut complex_float, ldt: *const c_int, info: *mut c_int);
pub fn zgeqrt2_(m: *const c_int, n: *const c_int, a: *mut complex_double, lda: *const c_int,
                t: *mut complex_double, ldt: *const c_int, info: *mut c_int);

pub fn sgeqrt3_(m: *const c_int, n: *const c_int, a: *mut c_float, lda: *const c_int,
                t: *mut c_float, ldt: *const c_int, info: *mut c_int);
pub fn dgeqrt3_(m: *const c_int, n: *const c_int, a: *mut c_double, lda: *const c_int,
                t: *mut c_double, ldt: *const c_int, info: *mut c_int);
pub fn cgeqrt3_(m: *const c_int, n: *const c_int, a: *mut complex_float, lda: *const c_int,
                t: *mut complex_float, ldt: *const c_int, info: *mut c_int);
pub fn zgeqrt3_(m: *const c_int, n: *const c_int, a: *mut complex_double, lda: *const c_int,
                t: *mut complex_double, ldt: *const c_int, info: *mut c_int);

pub fn stpmqrt_(side: *const c_char, trans: *const c_char, m: *const c_int, n: *const c_int,
                k: *const c_int, l: *const c_int, nb: *const c_int, v: *const c_float,
                ldv: *const c_int, t: *const c_float, ldt: *const c_int, a: *mut c_float,
                lda: *const c_int, b: *mut c_float, ldb: *const c_int, work: *mut c_float,
                info: *mut c_int);
pub fn dtpmqrt_(side: *const c_char, trans: *const c_char, m: *const c_int, n: *const c_int,
                k: *const c_int, l: *const c_int, nb: *const c_int, v: *const c_double,
                ldv: *const c_int, t: *const c_double, ldt: *const c_int, a: *mut c_double,
                lda: *const c_int, b: *mut c_double, ldb: *const c_int, work: *mut c_double,
                info: *mut c_int);
pub fn ctpmqrt_(side: *const c_char, trans: *const c_char, m: *const c_int, n: *const c_int,
                k: *const c_int, l: *const c_int, nb: *const c_int, v: *const complex_float,
                ldv: *const c_int, t: *const complex_float, ldt: *const c_int,
                a: *mut complex_float, lda: *const c_int, b: *mut complex_float,
                ldb: *const c_int, work: *mut complex_float, info: *mut c_int);
pub fn ztpmqrt_(side: *const c_char, trans: *const c_char, m: *const c_int, n: *const c_int,
                k: *const c_int, l: *const c_int, nb: *const c_int, v: *const complex_double,
                ldv: *const c_int, t: *const complex_double, ldt: *const c_int,
                a: *mut complex_double, lda: *const c_int, b: *mut complex_double,
                ldb: *const c_int, work: *mut complex_double, info: *mut c_int);

pub fn stpqrt_(m: *const c_int, n: *const c_int, l: *const c_int, nb: *const c_int,
               a: *mut c_float, lda: *const c_int, b: *mut c_float, ldb: *const c_int,
               t: *mut c_float, ldt: *const c_int, work: *mut c_float, info: *mut c_int);
pub fn dtpqrt_(m: *const c_int, n: *const c_int, l: *const c_int, nb: *const c_int,
               a: *mut c_double, lda: *const c_int, b: *mut c_double, ldb: *const c_int,
               t: *mut c_double, ldt: *const c_int, work: *mut c_double, info: *mut c_int);
pub fn ctpqrt_(m: *const c_int, n: *const c_int, l: *const c_int, nb: *const c_int,
               a: *mut complex_float, lda: *const c_int, b: *mut complex_float,
               ldb: *const c_int, t: *mut complex_float, ldt: *const c_int,
               work: *mut complex_float, info: *mut c_int);
pub fn ztpqrt_(m: *const c_int, n: *const c_int, l: *const c_int, nb: *const c_int,
               a: *mut complex_double, lda: *const c_int, b: *mut complex_double,
               ldb: *const c_int, t: *mut complex_double, ldt: *const c_int,
               work: *mut complex_double, info: *mut c_int);

pub fn stpqrt2_(m: *const c_int, n: *const c_int, l: *const c_int, a: *mut c_float,
                lda: *const c_int, b: *mut c_float, ldb: *const c_int, t: *mut c_float,
                ldt: *const c_int, info: *mut c_int);
pub fn dtpqrt2_(m: *const c_int, n: *const c_int, l: *const c_int, a: *mut c_double,
                lda: *const c_int, b: *mut c_double, ldb: *const c_int, t: *mut c_double,
                ldt: *const c_int, info: *mut c_int);
pub fn ctpqrt2_(m: *const c_int, n: *const c_int, l: *const c_int, a: *mut complex_float,
                lda: *const c_int, b: *mut complex_float, ldb: *const c_int,
                t: *mut complex_float, ldt: *const c_int, info: *mut c_int);
pub fn ztpqrt2_(m: *const c_int, n: *const c_int, l: *const c_int, a: *mut complex_double,
                lda: *const c_int, b: *mut complex_double, ldb: *const c_int,
                t: *mut complex_double, ldt: *const c_int, info: *mut c_int);

pub fn stprfb_(side: *const c_char, trans: *const c_char, direct: *const c_char,
               storev: *const c_char, m: *const c_int, n: *const c_int, k: *const c_int,
               l: *const c_int, v: *const c_float, ldv: *const c_int, t: *const c_float,
               ldt: *const c_int, a: *mut c_float, lda: *const c_int, b: *mut c_float,
               ldb: *const c_int, work: *const c_float, ldwork: *const c_int);
pub fn dtprfb_(side: *const c_char, trans: *const c_char, direct: *const c_char,
               storev: *const c_char, m: *const c_int, n: *const c_int, k: *const c_int,
               l: *const c_int, v: *const c_double, ldv: *const c_int, t: *const c_double,
               ldt: *const c_int, a: *mut c_double, lda: *const c_int, b: *mut c_double,
               ldb: *const c_int, work: *const c_double, ldwork: *const c_int);
pub fn ctprfb_(side: *const c_char, trans: *const c_char, direct: *const c_char,
               storev: *const c_char, m: *const c_int, n: *const c_int, k: *const c_int,
               l: *const c_int, v: *const complex_float, ldv: *const c_int,
               t: *const complex_float, ldt: *const c_int, a: *mut complex_float,
               lda: *const c_int, b: *mut complex_float, ldb: *const c_int,
               work: *const c_float, ldwork: *const c_int);
pub fn ztprfb_(side: *const c_char, trans: *const c_char, direct: *const c_char,
               storev: *const c_char, m: *const c_int, n: *const c_int, k: *const c_int,
               l: *const c_int, v: *const complex_double, ldv: *const c_int,
               t: *const complex_double, ldt: *const c_int, a: *mut complex_double,
               lda: *const c_int, b: *mut complex_double, ldb: *const c_int,
               work: *const c_double, ldwork: *const c_int);

pub fn ssysv_rook_(uplo: *const c_char, n: *const c_int, nrhs: *const c_int, a: *mut c_float,
                   lda: *const c_int, ipiv: *mut c_int, b: *mut c_float, ldb: *const c_int,
                   work: *mut c_float, lwork: *const c_int, info: *mut c_int);
pub fn dsysv_rook_(uplo: *const c_char, n: *const c_int, nrhs: *const c_int, a: *mut c_double,
                   lda: *const c_int, ipiv: *mut c_int, b: *mut c_double, ldb: *const c_int,
                   work: *mut c_double, lwork: *const c_int, info: *mut c_int);
pub fn csysv_rook_(uplo: *const c_char, n: *const c_int, nrhs: *const c_int,
                   a: *mut complex_float, lda: *const c_int, ipiv: *mut c_int,
                   b: *mut complex_float, ldb: *const c_int, work: *mut complex_float,
                   lwork: *const c_int, info: *mut c_int);
pub fn zsysv_rook_(uplo: *const c_char, n: *const c_int, nrhs: *const c_int,
                   a: *mut complex_double, lda: *const c_int, ipiv: *mut c_int,
                   b: *mut complex_double, ldb: *const c_int, work: *mut complex_double,
                   lwork: *const c_int, info: *mut c_int);

pub fn csyr_(uplo: *const c_char, n: *const c_int, alpha: *const complex_float,
             x: *const complex_float, incx: *const c_int, a: *mut complex_float,
             lda: *const c_int);
pub fn zsyr_(uplo: *const c_char, n: *const c_int, alpha: *const complex_double,
             x: *const complex_double, incx: *const c_int, a: *mut complex_double,
             lda: *const c_int);

pub fn ilaver_(vers_major: *mut c_int, vers_minor: *mut c_int, vers_patch: *mut c_int);
"""

name_re = re.compile("pub fn (\w+)_")
argument_re = re.compile("(\w+): ([^,]*)(,|\))")
return_re = re.compile("(?:\s*->\s*([^;]+))?");
select_re = re.compile("LAPACK_(\w)_SELECT(\d)")

def pull_name(s):
    m = name_re.match(s)
    assert(m is not None)
    return m.group(1), s[m.end(1)+1:]

def pull_argument(s):
    m = argument_re.match(s)
    if m is None:
        return None, None, s
    return m.group(1), m.group(2), s[m.end(3):]

def pull_return(s):
    m = return_re.match(s)
    if m is None:
        return None, s
    return m.group(1), s[m.end(1):]

def chew(s, c):
    assert s[0] == c
    return s[1:]

class Func(object):
    def __init__(self, name, args, ret):
        self.name = name
        self.args = args
        self.ret = ret

    @staticmethod
    def parse(line):
        name, line = pull_name(line)
        if name is None:
            return None

        line = chew(line, '(')
        args = []
        while True:
            arg, aty, line = pull_argument(line)
            if arg is None:
                break
            args.append((arg, aty))
            line = line.strip()

        ret, line = pull_return(line)

        return Func(name, args, ret)

def is_const(name, cty):
    return "*const" in cty

def is_letter(name, cty):
    return "c_char" in cty

def is_natural(name, cty):
    return "c_int" in cty and (
        name in ["l", "lwork", "m", "mm", "n", "nb", "nrhs", "p", "q", "rank"] or
        name.startswith("k") or
        name.startswith("ld") or
        name.startswith("inc") or
        name.startswith("vers_")
    )

def is_mut(name, cty):
    return "*mut" in cty

def is_scalar(name, cty, f):
    return name in ["info"]

def translate_argument(name, cty, f):
    if is_natural(name, cty):
        if is_const(name, cty):
            return "usize"
        elif is_mut(name, cty):
            return "&mut u32"
    elif is_letter(name, cty):
        if is_const(name, cty):
            return "u8"
        elif is_mut(name, cty):
            return "&mut u8"
    elif is_const(name, cty):
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
    if "complex_double" in cty:
        return "c64"
    elif "complex_float" in cty:
        return "c32"
    elif "c_double" in cty:
        return "f64"
    elif "c_float" in cty:
        return "f32"
    elif "c_int" in cty:
        return "i32"

    assert False, "cannot translate `{}` in `{}`".format(cty, f.name)

def translate_body_argument(name, rty):
    if rty == "u8":
        return "&({} as c_char)".format(name)

    elif rty == "&mut u8":
        return "{} as *mut _ as *mut _".format(name)

    elif rty == "usize":
        return "&({} as c_int)".format(name)
    elif rty == "&mut u32":
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
    if cty == "c_int":
        return "isize"
    elif cty == "c_float":
        return "f32"
    elif cty == "c_double":
        return "f64"

    assert False, "cannot translate `{}`".format(cty)

def format_header(f):
    args = format_header_arguments(f)
    ret = "" if f.ret is None else " -> {}".format(translate_return_type(f.ret))
    header = "pub fn {}({}){} {{".format(f.name, args, ret)

    s = []
    indent = 7 + len(f.name) + 1
    while True:
        if len(header) <= 99:
            s.append(header)
            break
        k = 98 - header[98::-1].index(',')
        if k < 0:
            s.append(header)
            break
        s.append(header[:k+1])
        header = "{}{}".format(" " * indent, header[k+2:])

    if len(s) > 1:
        s.append("")

    return "\n".join(s)

def format_body(f):
    a = format_body_arguments(f)
    if f.ret is None:
        a = "{})".format(a)
    if f.ret is not None:
        a = "{}) as {}".format(a, translate_return_type(f.ret))

    s = []
    s.append(" " * 4)
    s.append("unsafe {\n")
    s.append(" " * 8)
    s.append("ffi::{}_(".format(f.name))

    a = "".join(a)
    indent = 8 + 5 + len(f.name) + 2
    while len(a) > 0:
        if len(a) + indent > 99:
            k = a.find(",")
            if k < 0 or k > 98:
                assert False, "cannot format `{}`".format(f.name)
            while True:
                l = a.find(",", k + 1)
                if l < 0 or l + indent > 98: break
                k = l
            s.append(a[0:k+1])
            s.append("\n")
            s.append(" " * indent)
            a = a[k+2:]
        else:
            s.append(a)
            a = ""

    s.append("\n")
    s.append(" " * 4)
    s.append("}")

    return "".join(s)

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
    lines = re.sub(r'\s+', ' ', "".join(code.split('\n'))).strip().split(';')
    lines = filter(lambda line: not re.match(r'^\s*$', line), lines)
    return [Func.parse(line) for line in lines]

def do(funcs):
    for f in funcs:
        print("#[inline]")
        print(format_header(f))
        print(format_body(f))
        print("}\n")

do(prepare(definitions))
