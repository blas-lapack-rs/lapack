#!/usr/bin/env python

import re

functions = """
    pub fn LAPACKE_sbdsdc(matrix_layout: c_int, uplo: c_char, compq: c_char, n: lapack_int,
                          d: *mut c_float, e: *mut c_float, u: *mut c_float, ldu: lapack_int,
                          vt: *mut c_float, ldvt: lapack_int, q: *mut c_float, iq: *mut lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_dbdsdc(matrix_layout: c_int, uplo: c_char, compq: c_char, n: lapack_int,
                          d: *mut c_double, e: *mut c_double, u: *mut c_double, ldu: lapack_int,
                          vt: *mut c_double, ldvt: lapack_int, q: *mut c_double,
                          iq: *mut lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_sbdsqr(matrix_layout: c_int, uplo: c_char, n: lapack_int, ncvt: lapack_int,
                          nru: lapack_int, ncc: lapack_int, d: *mut c_float, e: *mut c_float,
                          vt: *mut c_float, ldvt: lapack_int, u: *mut c_float, ldu: lapack_int,
                          c: *mut c_float, ldc: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_dbdsqr(matrix_layout: c_int, uplo: c_char, n: lapack_int, ncvt: lapack_int,
                          nru: lapack_int, ncc: lapack_int, d: *mut c_double, e: *mut c_double,
                          vt: *mut c_double, ldvt: lapack_int, u: *mut c_double, ldu: lapack_int,
                          c: *mut c_double, ldc: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_cbdsqr(matrix_layout: c_int, uplo: c_char, n: lapack_int, ncvt: lapack_int,
                          nru: lapack_int, ncc: lapack_int, d: *mut c_float, e: *mut c_float,
                          vt: *mut lapack_complex_float, ldvt: lapack_int,
                          u: *mut lapack_complex_float, ldu: lapack_int,
                          c: *mut lapack_complex_float, ldc: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_zbdsqr(matrix_layout: c_int, uplo: c_char, n: lapack_int, ncvt: lapack_int,
                          nru: lapack_int, ncc: lapack_int, d: *mut c_double, e: *mut c_double,
                          vt: *mut lapack_complex_double, ldvt: lapack_int,
                          u: *mut lapack_complex_double, ldu: lapack_int,
                          c: *mut lapack_complex_double, ldc: lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_sbdsvdx(matrix_layout: c_int, uplo: c_char, jobz: c_char, range: c_char,
                           n: lapack_int, d: *mut c_float, e: *mut c_float, vl: c_float,
                           vu: c_float, il: lapack_int, iu: lapack_int, ns: lapack_int,
                           s: *mut c_float, z: *mut c_float, ldz: lapack_int,
                           superb: *mut lapack_int)
                           -> lapack_int;
    pub fn LAPACKE_dbdsvdx(matrix_layout: c_int, uplo: c_char, jobz: c_char, range: c_char,
                           n: lapack_int, d: *mut c_double, e: *mut c_double, vl: c_double,
                           vu: c_double, il: lapack_int, iu: lapack_int, ns: lapack_int,
                           s: *mut c_double, z: *mut c_double, ldz: lapack_int,
                           superb: *mut lapack_int)
                           -> lapack_int;

    pub fn LAPACKE_sdisna(job: c_char, m: lapack_int, n: lapack_int, d: *const c_float,
                          sep: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_ddisna(job: c_char, m: lapack_int, n: lapack_int, d: *const c_double,
                          sep: *mut c_double)
                          -> lapack_int;

    pub fn LAPACKE_sgbbrd(matrix_layout: c_int, vect: c_char, m: lapack_int, n: lapack_int,
                          ncc: lapack_int, kl: lapack_int, ku: lapack_int, ab: *mut c_float,
                          ldab: lapack_int, d: *mut c_float, e: *mut c_float, q: *mut c_float,
                          ldq: lapack_int, pt: *mut c_float, ldpt: lapack_int, c: *mut c_float,
                          ldc: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_dgbbrd(matrix_layout: c_int, vect: c_char, m: lapack_int, n: lapack_int,
                          ncc: lapack_int, kl: lapack_int, ku: lapack_int, ab: *mut c_double,
                          ldab: lapack_int, d: *mut c_double, e: *mut c_double, q: *mut c_double,
                          ldq: lapack_int, pt: *mut c_double, ldpt: lapack_int, c: *mut c_double,
                          ldc: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_cgbbrd(matrix_layout: c_int, vect: c_char, m: lapack_int, n: lapack_int,
                          ncc: lapack_int, kl: lapack_int, ku: lapack_int,
                          ab: *mut lapack_complex_float, ldab: lapack_int, d: *mut c_float,
                          e: *mut c_float, q: *mut lapack_complex_float, ldq: lapack_int,
                          pt: *mut lapack_complex_float, ldpt: lapack_int,
                          c: *mut lapack_complex_float, ldc: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_zgbbrd(matrix_layout: c_int, vect: c_char, m: lapack_int, n: lapack_int,
                          ncc: lapack_int, kl: lapack_int, ku: lapack_int,
                          ab: *mut lapack_complex_double, ldab: lapack_int, d: *mut c_double,
                          e: *mut c_double, q: *mut lapack_complex_double, ldq: lapack_int,
                          pt: *mut lapack_complex_double, ldpt: lapack_int,
                          c: *mut lapack_complex_double, ldc: lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_sgbcon(matrix_layout: c_int, norm: c_char, n: lapack_int, kl: lapack_int,
                          ku: lapack_int, ab: *const c_float, ldab: lapack_int,
                          ipiv: *const lapack_int, anorm: c_float, rcond: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_dgbcon(matrix_layout: c_int, norm: c_char, n: lapack_int, kl: lapack_int,
                          ku: lapack_int, ab: *const c_double, ldab: lapack_int,
                          ipiv: *const lapack_int, anorm: c_double, rcond: *mut c_double)
                          -> lapack_int;
    pub fn LAPACKE_cgbcon(matrix_layout: c_int, norm: c_char, n: lapack_int, kl: lapack_int,
                          ku: lapack_int, ab: *const lapack_complex_float, ldab: lapack_int,
                          ipiv: *const lapack_int, anorm: c_float, rcond: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_zgbcon(matrix_layout: c_int, norm: c_char, n: lapack_int, kl: lapack_int,
                          ku: lapack_int, ab: *const lapack_complex_double, ldab: lapack_int,
                          ipiv: *const lapack_int, anorm: c_double, rcond: *mut c_double)
                          -> lapack_int;

    pub fn LAPACKE_sgbequ(matrix_layout: c_int, m: lapack_int, n: lapack_int, kl: lapack_int,
                          ku: lapack_int, ab: *const c_float, ldab: lapack_int, r: *mut c_float,
                          c: *mut c_float, rowcnd: *mut c_float, colcnd: *mut c_float,
                          amax: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_dgbequ(matrix_layout: c_int, m: lapack_int, n: lapack_int, kl: lapack_int,
                          ku: lapack_int, ab: *const c_double, ldab: lapack_int, r: *mut c_double,
                          c: *mut c_double, rowcnd: *mut c_double, colcnd: *mut c_double,
                          amax: *mut c_double)
                          -> lapack_int;
    pub fn LAPACKE_cgbequ(matrix_layout: c_int, m: lapack_int, n: lapack_int, kl: lapack_int,
                          ku: lapack_int, ab: *const lapack_complex_float, ldab: lapack_int,
                          r: *mut c_float, c: *mut c_float, rowcnd: *mut c_float,
                          colcnd: *mut c_float, amax: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_zgbequ(matrix_layout: c_int, m: lapack_int, n: lapack_int, kl: lapack_int,
                          ku: lapack_int, ab: *const lapack_complex_double, ldab: lapack_int,
                          r: *mut c_double, c: *mut c_double, rowcnd: *mut c_double,
                          colcnd: *mut c_double, amax: *mut c_double)
                          -> lapack_int;

    pub fn LAPACKE_sgbequb(matrix_layout: c_int, m: lapack_int, n: lapack_int, kl: lapack_int,
                           ku: lapack_int, ab: *const c_float, ldab: lapack_int, r: *mut c_float,
                           c: *mut c_float, rowcnd: *mut c_float, colcnd: *mut c_float,
                           amax: *mut c_float)
                           -> lapack_int;
    pub fn LAPACKE_dgbequb(matrix_layout: c_int, m: lapack_int, n: lapack_int, kl: lapack_int,
                           ku: lapack_int, ab: *const c_double, ldab: lapack_int, r: *mut c_double,
                           c: *mut c_double, rowcnd: *mut c_double, colcnd: *mut c_double,
                           amax: *mut c_double)
                           -> lapack_int;
    pub fn LAPACKE_cgbequb(matrix_layout: c_int, m: lapack_int, n: lapack_int, kl: lapack_int,
                           ku: lapack_int, ab: *const lapack_complex_float, ldab: lapack_int,
                           r: *mut c_float, c: *mut c_float, rowcnd: *mut c_float,
                           colcnd: *mut c_float, amax: *mut c_float)
                           -> lapack_int;
    pub fn LAPACKE_zgbequb(matrix_layout: c_int, m: lapack_int, n: lapack_int, kl: lapack_int,
                           ku: lapack_int, ab: *const lapack_complex_double, ldab: lapack_int,
                           r: *mut c_double, c: *mut c_double, rowcnd: *mut c_double,
                           colcnd: *mut c_double, amax: *mut c_double)
                           -> lapack_int;

    pub fn LAPACKE_sgbrfs(matrix_layout: c_int, trans: c_char, n: lapack_int, kl: lapack_int,
                          ku: lapack_int, nrhs: lapack_int, ab: *const c_float, ldab: lapack_int,
                          afb: *const c_float, ldafb: lapack_int, ipiv: *const lapack_int,
                          b: *const c_float, ldb: lapack_int, x: *mut c_float, ldx: lapack_int,
                          ferr: *mut c_float, berr: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_dgbrfs(matrix_layout: c_int, trans: c_char, n: lapack_int, kl: lapack_int,
                          ku: lapack_int, nrhs: lapack_int, ab: *const c_double, ldab: lapack_int,
                          afb: *const c_double, ldafb: lapack_int, ipiv: *const lapack_int,
                          b: *const c_double, ldb: lapack_int, x: *mut c_double, ldx: lapack_int,
                          ferr: *mut c_double, berr: *mut c_double)
                          -> lapack_int;
    pub fn LAPACKE_cgbrfs(matrix_layout: c_int, trans: c_char, n: lapack_int, kl: lapack_int,
                          ku: lapack_int, nrhs: lapack_int, ab: *const lapack_complex_float,
                          ldab: lapack_int, afb: *const lapack_complex_float, ldafb: lapack_int,
                          ipiv: *const lapack_int, b: *const lapack_complex_float, ldb: lapack_int,
                          x: *mut lapack_complex_float, ldx: lapack_int, ferr: *mut c_float,
                          berr: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_zgbrfs(matrix_layout: c_int, trans: c_char, n: lapack_int, kl: lapack_int,
                          ku: lapack_int, nrhs: lapack_int, ab: *const lapack_complex_double,
                          ldab: lapack_int, afb: *const lapack_complex_double, ldafb: lapack_int,
                          ipiv: *const lapack_int, b: *const lapack_complex_double,
                          ldb: lapack_int, x: *mut lapack_complex_double, ldx: lapack_int,
                          ferr: *mut c_double, berr: *mut c_double)
                          -> lapack_int;

    pub fn LAPACKE_sgbrfsx(matrix_layout: c_int, trans: c_char, equed: c_char, n: lapack_int,
                           kl: lapack_int, ku: lapack_int, nrhs: lapack_int, ab: *const c_float,
                           ldab: lapack_int, afb: *const c_float, ldafb: lapack_int,
                           ipiv: *const lapack_int, r: *const c_float, c: *const c_float,
                           b: *const c_float, ldb: lapack_int, x: *mut c_float, ldx: lapack_int,
                           rcond: *mut c_float, berr: *mut c_float, n_err_bnds: lapack_int,
                           err_bnds_norm: *mut c_float, err_bnds_comp: *mut c_float,
                           nparams: lapack_int, params: *mut c_float)
                           -> lapack_int;
    pub fn LAPACKE_dgbrfsx(matrix_layout: c_int, trans: c_char, equed: c_char, n: lapack_int,
                           kl: lapack_int, ku: lapack_int, nrhs: lapack_int, ab: *const c_double,
                           ldab: lapack_int, afb: *const c_double, ldafb: lapack_int,
                           ipiv: *const lapack_int, r: *const c_double, c: *const c_double,
                           b: *const c_double, ldb: lapack_int, x: *mut c_double, ldx: lapack_int,
                           rcond: *mut c_double, berr: *mut c_double, n_err_bnds: lapack_int,
                           err_bnds_norm: *mut c_double, err_bnds_comp: *mut c_double,
                           nparams: lapack_int, params: *mut c_double)
                           -> lapack_int;
    pub fn LAPACKE_cgbrfsx(matrix_layout: c_int, trans: c_char, equed: c_char, n: lapack_int,
                           kl: lapack_int, ku: lapack_int, nrhs: lapack_int,
                           ab: *const lapack_complex_float, ldab: lapack_int,
                           afb: *const lapack_complex_float, ldafb: lapack_int,
                           ipiv: *const lapack_int, r: *const c_float, c: *const c_float,
                           b: *const lapack_complex_float, ldb: lapack_int,
                           x: *mut lapack_complex_float, ldx: lapack_int, rcond: *mut c_float,
                           berr: *mut c_float, n_err_bnds: lapack_int, err_bnds_norm: *mut c_float,
                           err_bnds_comp: *mut c_float, nparams: lapack_int, params: *mut c_float)
                           -> lapack_int;
    pub fn LAPACKE_zgbrfsx(matrix_layout: c_int, trans: c_char, equed: c_char, n: lapack_int,
                           kl: lapack_int, ku: lapack_int, nrhs: lapack_int,
                           ab: *const lapack_complex_double, ldab: lapack_int,
                           afb: *const lapack_complex_double, ldafb: lapack_int,
                           ipiv: *const lapack_int, r: *const c_double, c: *const c_double,
                           b: *const lapack_complex_double, ldb: lapack_int,
                           x: *mut lapack_complex_double, ldx: lapack_int, rcond: *mut c_double,
                           berr: *mut c_double, n_err_bnds: lapack_int,
                           err_bnds_norm: *mut c_double, err_bnds_comp: *mut c_double,
                           nparams: lapack_int, params: *mut c_double)
                           -> lapack_int;

    pub fn LAPACKE_sgbsv(matrix_layout: c_int, n: lapack_int, kl: lapack_int, ku: lapack_int,
                         nrhs: lapack_int, ab: *mut c_float, ldab: lapack_int,
                         ipiv: *mut lapack_int, b: *mut c_float, ldb: lapack_int)
                         -> lapack_int;
    pub fn LAPACKE_dgbsv(matrix_layout: c_int, n: lapack_int, kl: lapack_int, ku: lapack_int,
                         nrhs: lapack_int, ab: *mut c_double, ldab: lapack_int,
                         ipiv: *mut lapack_int, b: *mut c_double, ldb: lapack_int)
                         -> lapack_int;
    pub fn LAPACKE_cgbsv(matrix_layout: c_int, n: lapack_int, kl: lapack_int, ku: lapack_int,
                         nrhs: lapack_int, ab: *mut lapack_complex_float, ldab: lapack_int,
                         ipiv: *mut lapack_int, b: *mut lapack_complex_float, ldb: lapack_int)
                         -> lapack_int;
    pub fn LAPACKE_zgbsv(matrix_layout: c_int, n: lapack_int, kl: lapack_int, ku: lapack_int,
                         nrhs: lapack_int, ab: *mut lapack_complex_double, ldab: lapack_int,
                         ipiv: *mut lapack_int, b: *mut lapack_complex_double, ldb: lapack_int)
                         -> lapack_int;

    pub fn LAPACKE_sgbsvx(matrix_layout: c_int, fact: c_char, trans: c_char, n: lapack_int,
                          kl: lapack_int, ku: lapack_int, nrhs: lapack_int, ab: *mut c_float,
                          ldab: lapack_int, afb: *mut c_float, ldafb: lapack_int,
                          ipiv: *mut lapack_int, equed: *mut c_char, r: *mut c_float,
                          c: *mut c_float, b: *mut c_float, ldb: lapack_int, x: *mut c_float,
                          ldx: lapack_int, rcond: *mut c_float, ferr: *mut c_float,
                          berr: *mut c_float, rpivot: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_dgbsvx(matrix_layout: c_int, fact: c_char, trans: c_char, n: lapack_int,
                          kl: lapack_int, ku: lapack_int, nrhs: lapack_int, ab: *mut c_double,
                          ldab: lapack_int, afb: *mut c_double, ldafb: lapack_int,
                          ipiv: *mut lapack_int, equed: *mut c_char, r: *mut c_double,
                          c: *mut c_double, b: *mut c_double, ldb: lapack_int, x: *mut c_double,
                          ldx: lapack_int, rcond: *mut c_double, ferr: *mut c_double,
                          berr: *mut c_double, rpivot: *mut c_double)
                          -> lapack_int;
    pub fn LAPACKE_cgbsvx(matrix_layout: c_int, fact: c_char, trans: c_char, n: lapack_int,
                          kl: lapack_int, ku: lapack_int, nrhs: lapack_int,
                          ab: *mut lapack_complex_float, ldab: lapack_int,
                          afb: *mut lapack_complex_float, ldafb: lapack_int, ipiv: *mut lapack_int,
                          equed: *mut c_char, r: *mut c_float, c: *mut c_float,
                          b: *mut lapack_complex_float, ldb: lapack_int,
                          x: *mut lapack_complex_float, ldx: lapack_int, rcond: *mut c_float,
                          ferr: *mut c_float, berr: *mut c_float, rpivot: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_zgbsvx(matrix_layout: c_int, fact: c_char, trans: c_char, n: lapack_int,
                          kl: lapack_int, ku: lapack_int, nrhs: lapack_int,
                          ab: *mut lapack_complex_double, ldab: lapack_int,
                          afb: *mut lapack_complex_double, ldafb: lapack_int,
                          ipiv: *mut lapack_int, equed: *mut c_char, r: *mut c_double,
                          c: *mut c_double, b: *mut lapack_complex_double, ldb: lapack_int,
                          x: *mut lapack_complex_double, ldx: lapack_int, rcond: *mut c_double,
                          ferr: *mut c_double, berr: *mut c_double, rpivot: *mut c_double)
                          -> lapack_int;

    pub fn LAPACKE_sgbsvxx(matrix_layout: c_int, fact: c_char, trans: c_char, n: lapack_int,
                           kl: lapack_int, ku: lapack_int, nrhs: lapack_int, ab: *mut c_float,
                           ldab: lapack_int, afb: *mut c_float, ldafb: lapack_int,
                           ipiv: *mut lapack_int, equed: *mut c_char, r: *mut c_float,
                           c: *mut c_float, b: *mut c_float, ldb: lapack_int, x: *mut c_float,
                           ldx: lapack_int, rcond: *mut c_float, rpvgrw: *mut c_float,
                           berr: *mut c_float, n_err_bnds: lapack_int, err_bnds_norm: *mut c_float,
                           err_bnds_comp: *mut c_float, nparams: lapack_int, params: *mut c_float)
                           -> lapack_int;
    pub fn LAPACKE_dgbsvxx(matrix_layout: c_int, fact: c_char, trans: c_char, n: lapack_int,
                           kl: lapack_int, ku: lapack_int, nrhs: lapack_int, ab: *mut c_double,
                           ldab: lapack_int, afb: *mut c_double, ldafb: lapack_int,
                           ipiv: *mut lapack_int, equed: *mut c_char, r: *mut c_double,
                           c: *mut c_double, b: *mut c_double, ldb: lapack_int, x: *mut c_double,
                           ldx: lapack_int, rcond: *mut c_double, rpvgrw: *mut c_double,
                           berr: *mut c_double, n_err_bnds: lapack_int,
                           err_bnds_norm: *mut c_double, err_bnds_comp: *mut c_double,
                           nparams: lapack_int, params: *mut c_double)
                           -> lapack_int;
    pub fn LAPACKE_cgbsvxx(matrix_layout: c_int, fact: c_char, trans: c_char, n: lapack_int,
                           kl: lapack_int, ku: lapack_int, nrhs: lapack_int,
                           ab: *mut lapack_complex_float, ldab: lapack_int,
                           afb: *mut lapack_complex_float, ldafb: lapack_int,
                           ipiv: *mut lapack_int, equed: *mut c_char, r: *mut c_float,
                           c: *mut c_float, b: *mut lapack_complex_float, ldb: lapack_int,
                           x: *mut lapack_complex_float, ldx: lapack_int, rcond: *mut c_float,
                           rpvgrw: *mut c_float, berr: *mut c_float, n_err_bnds: lapack_int,
                           err_bnds_norm: *mut c_float, err_bnds_comp: *mut c_float,
                           nparams: lapack_int, params: *mut c_float)
                           -> lapack_int;
    pub fn LAPACKE_zgbsvxx(matrix_layout: c_int, fact: c_char, trans: c_char, n: lapack_int,
                           kl: lapack_int, ku: lapack_int, nrhs: lapack_int,
                           ab: *mut lapack_complex_double, ldab: lapack_int,
                           afb: *mut lapack_complex_double, ldafb: lapack_int,
                           ipiv: *mut lapack_int, equed: *mut c_char, r: *mut c_double,
                           c: *mut c_double, b: *mut lapack_complex_double, ldb: lapack_int,
                           x: *mut lapack_complex_double, ldx: lapack_int, rcond: *mut c_double,
                           rpvgrw: *mut c_double, berr: *mut c_double, n_err_bnds: lapack_int,
                           err_bnds_norm: *mut c_double, err_bnds_comp: *mut c_double,
                           nparams: lapack_int, params: *mut c_double)
                           -> lapack_int;

    pub fn LAPACKE_sgbtrf(matrix_layout: c_int, m: lapack_int, n: lapack_int, kl: lapack_int,
                          ku: lapack_int, ab: *mut c_float, ldab: lapack_int,
                          ipiv: *mut lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_dgbtrf(matrix_layout: c_int, m: lapack_int, n: lapack_int, kl: lapack_int,
                          ku: lapack_int, ab: *mut c_double, ldab: lapack_int,
                          ipiv: *mut lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_cgbtrf(matrix_layout: c_int, m: lapack_int, n: lapack_int, kl: lapack_int,
                          ku: lapack_int, ab: *mut lapack_complex_float, ldab: lapack_int,
                          ipiv: *mut lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_zgbtrf(matrix_layout: c_int, m: lapack_int, n: lapack_int, kl: lapack_int,
                          ku: lapack_int, ab: *mut lapack_complex_double, ldab: lapack_int,
                          ipiv: *mut lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_sgbtrs(matrix_layout: c_int, trans: c_char, n: lapack_int, kl: lapack_int,
                          ku: lapack_int, nrhs: lapack_int, ab: *const c_float, ldab: lapack_int,
                          ipiv: *const lapack_int, b: *mut c_float, ldb: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_dgbtrs(matrix_layout: c_int, trans: c_char, n: lapack_int, kl: lapack_int,
                          ku: lapack_int, nrhs: lapack_int, ab: *const c_double, ldab: lapack_int,
                          ipiv: *const lapack_int, b: *mut c_double, ldb: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_cgbtrs(matrix_layout: c_int, trans: c_char, n: lapack_int, kl: lapack_int,
                          ku: lapack_int, nrhs: lapack_int, ab: *const lapack_complex_float,
                          ldab: lapack_int, ipiv: *const lapack_int, b: *mut lapack_complex_float,
                          ldb: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_zgbtrs(matrix_layout: c_int, trans: c_char, n: lapack_int, kl: lapack_int,
                          ku: lapack_int, nrhs: lapack_int, ab: *const lapack_complex_double,
                          ldab: lapack_int, ipiv: *const lapack_int, b: *mut lapack_complex_double,
                          ldb: lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_sgebak(matrix_layout: c_int, job: c_char, side: c_char, n: lapack_int,
                          ilo: lapack_int, ihi: lapack_int, scale: *const c_float, m: lapack_int,
                          v: *mut c_float, ldv: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_dgebak(matrix_layout: c_int, job: c_char, side: c_char, n: lapack_int,
                          ilo: lapack_int, ihi: lapack_int, scale: *const c_double, m: lapack_int,
                          v: *mut c_double, ldv: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_cgebak(matrix_layout: c_int, job: c_char, side: c_char, n: lapack_int,
                          ilo: lapack_int, ihi: lapack_int, scale: *const c_float, m: lapack_int,
                          v: *mut lapack_complex_float, ldv: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_zgebak(matrix_layout: c_int, job: c_char, side: c_char, n: lapack_int,
                          ilo: lapack_int, ihi: lapack_int, scale: *const c_double, m: lapack_int,
                          v: *mut lapack_complex_double, ldv: lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_sgebal(matrix_layout: c_int, job: c_char, n: lapack_int, a: *mut c_float,
                          lda: lapack_int, ilo: *mut lapack_int, ihi: *mut lapack_int,
                          scale: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_dgebal(matrix_layout: c_int, job: c_char, n: lapack_int, a: *mut c_double,
                          lda: lapack_int, ilo: *mut lapack_int, ihi: *mut lapack_int,
                          scale: *mut c_double)
                          -> lapack_int;
    pub fn LAPACKE_cgebal(matrix_layout: c_int, job: c_char, n: lapack_int,
                          a: *mut lapack_complex_float, lda: lapack_int, ilo: *mut lapack_int,
                          ihi: *mut lapack_int, scale: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_zgebal(matrix_layout: c_int, job: c_char, n: lapack_int,
                          a: *mut lapack_complex_double, lda: lapack_int, ilo: *mut lapack_int,
                          ihi: *mut lapack_int, scale: *mut c_double)
                          -> lapack_int;

    pub fn LAPACKE_sgebrd(matrix_layout: c_int, m: lapack_int, n: lapack_int, a: *mut c_float,
                          lda: lapack_int, d: *mut c_float, e: *mut c_float, tauq: *mut c_float,
                          taup: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_dgebrd(matrix_layout: c_int, m: lapack_int, n: lapack_int, a: *mut c_double,
                          lda: lapack_int, d: *mut c_double, e: *mut c_double, tauq: *mut c_double,
                          taup: *mut c_double)
                          -> lapack_int;
    pub fn LAPACKE_cgebrd(matrix_layout: c_int, m: lapack_int, n: lapack_int,
                          a: *mut lapack_complex_float, lda: lapack_int, d: *mut c_float,
                          e: *mut c_float, tauq: *mut lapack_complex_float,
                          taup: *mut lapack_complex_float)
                          -> lapack_int;
    pub fn LAPACKE_zgebrd(matrix_layout: c_int, m: lapack_int, n: lapack_int,
                          a: *mut lapack_complex_double, lda: lapack_int, d: *mut c_double,
                          e: *mut c_double, tauq: *mut lapack_complex_double,
                          taup: *mut lapack_complex_double)
                          -> lapack_int;

    pub fn LAPACKE_sgecon(matrix_layout: c_int, norm: c_char, n: lapack_int, a: *const c_float,
                          lda: lapack_int, anorm: c_float, rcond: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_dgecon(matrix_layout: c_int, norm: c_char, n: lapack_int, a: *const c_double,
                          lda: lapack_int, anorm: c_double, rcond: *mut c_double)
                          -> lapack_int;
    pub fn LAPACKE_cgecon(matrix_layout: c_int, norm: c_char, n: lapack_int,
                          a: *const lapack_complex_float, lda: lapack_int, anorm: c_float,
                          rcond: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_zgecon(matrix_layout: c_int, norm: c_char, n: lapack_int,
                          a: *const lapack_complex_double, lda: lapack_int, anorm: c_double,
                          rcond: *mut c_double)
                          -> lapack_int;

    pub fn LAPACKE_sgeequ(matrix_layout: c_int, m: lapack_int, n: lapack_int, a: *const c_float,
                          lda: lapack_int, r: *mut c_float, c: *mut c_float, rowcnd: *mut c_float,
                          colcnd: *mut c_float, amax: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_dgeequ(matrix_layout: c_int, m: lapack_int, n: lapack_int, a: *const c_double,
                          lda: lapack_int, r: *mut c_double, c: *mut c_double,
                          rowcnd: *mut c_double, colcnd: *mut c_double, amax: *mut c_double)
                          -> lapack_int;
    pub fn LAPACKE_cgeequ(matrix_layout: c_int, m: lapack_int, n: lapack_int,
                          a: *const lapack_complex_float, lda: lapack_int, r: *mut c_float,
                          c: *mut c_float, rowcnd: *mut c_float, colcnd: *mut c_float,
                          amax: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_zgeequ(matrix_layout: c_int, m: lapack_int, n: lapack_int,
                          a: *const lapack_complex_double, lda: lapack_int, r: *mut c_double,
                          c: *mut c_double, rowcnd: *mut c_double, colcnd: *mut c_double,
                          amax: *mut c_double)
                          -> lapack_int;

    pub fn LAPACKE_sgeequb(matrix_layout: c_int, m: lapack_int, n: lapack_int, a: *const c_float,
                           lda: lapack_int, r: *mut c_float, c: *mut c_float, rowcnd: *mut c_float,
                           colcnd: *mut c_float, amax: *mut c_float)
                           -> lapack_int;
    pub fn LAPACKE_dgeequb(matrix_layout: c_int, m: lapack_int, n: lapack_int, a: *const c_double,
                           lda: lapack_int, r: *mut c_double, c: *mut c_double,
                           rowcnd: *mut c_double, colcnd: *mut c_double, amax: *mut c_double)
                           -> lapack_int;
    pub fn LAPACKE_cgeequb(matrix_layout: c_int, m: lapack_int, n: lapack_int,
                           a: *const lapack_complex_float, lda: lapack_int, r: *mut c_float,
                           c: *mut c_float, rowcnd: *mut c_float, colcnd: *mut c_float,
                           amax: *mut c_float)
                           -> lapack_int;
    pub fn LAPACKE_zgeequb(matrix_layout: c_int, m: lapack_int, n: lapack_int,
                           a: *const lapack_complex_double, lda: lapack_int, r: *mut c_double,
                           c: *mut c_double, rowcnd: *mut c_double, colcnd: *mut c_double,
                           amax: *mut c_double)
                           -> lapack_int;

    pub fn LAPACKE_sgees(matrix_layout: c_int, jobvs: c_char, sort: c_char,
                         select: LAPACK_S_SELECT2, n: lapack_int, a: *mut c_float, lda: lapack_int,
                         sdim: *mut lapack_int, wr: *mut c_float, wi: *mut c_float,
                         vs: *mut c_float, ldvs: lapack_int)
                         -> lapack_int;
    pub fn LAPACKE_dgees(matrix_layout: c_int, jobvs: c_char, sort: c_char,
                         select: LAPACK_D_SELECT2, n: lapack_int, a: *mut c_double,
                         lda: lapack_int, sdim: *mut lapack_int, wr: *mut c_double,
                         wi: *mut c_double, vs: *mut c_double, ldvs: lapack_int)
                         -> lapack_int;
    pub fn LAPACKE_cgees(matrix_layout: c_int, jobvs: c_char, sort: c_char,
                         select: LAPACK_C_SELECT1, n: lapack_int, a: *mut lapack_complex_float,
                         lda: lapack_int, sdim: *mut lapack_int, w: *mut lapack_complex_float,
                         vs: *mut lapack_complex_float, ldvs: lapack_int)
                         -> lapack_int;
    pub fn LAPACKE_zgees(matrix_layout: c_int, jobvs: c_char, sort: c_char,
                         select: LAPACK_Z_SELECT1, n: lapack_int, a: *mut lapack_complex_double,
                         lda: lapack_int, sdim: *mut lapack_int, w: *mut lapack_complex_double,
                         vs: *mut lapack_complex_double, ldvs: lapack_int)
                         -> lapack_int;

    pub fn LAPACKE_sgeesx(matrix_layout: c_int, jobvs: c_char, sort: c_char,
                          select: LAPACK_S_SELECT2, sense: c_char, n: lapack_int, a: *mut c_float,
                          lda: lapack_int, sdim: *mut lapack_int, wr: *mut c_float,
                          wi: *mut c_float, vs: *mut c_float, ldvs: lapack_int,
                          rconde: *mut c_float, rcondv: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_dgeesx(matrix_layout: c_int, jobvs: c_char, sort: c_char,
                          select: LAPACK_D_SELECT2, sense: c_char, n: lapack_int, a: *mut c_double,
                          lda: lapack_int, sdim: *mut lapack_int, wr: *mut c_double,
                          wi: *mut c_double, vs: *mut c_double, ldvs: lapack_int,
                          rconde: *mut c_double, rcondv: *mut c_double)
                          -> lapack_int;
    pub fn LAPACKE_cgeesx(matrix_layout: c_int, jobvs: c_char, sort: c_char,
                          select: LAPACK_C_SELECT1, sense: c_char, n: lapack_int,
                          a: *mut lapack_complex_float, lda: lapack_int, sdim: *mut lapack_int,
                          w: *mut lapack_complex_float, vs: *mut lapack_complex_float,
                          ldvs: lapack_int, rconde: *mut c_float, rcondv: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_zgeesx(matrix_layout: c_int, jobvs: c_char, sort: c_char,
                          select: LAPACK_Z_SELECT1, sense: c_char, n: lapack_int,
                          a: *mut lapack_complex_double, lda: lapack_int, sdim: *mut lapack_int,
                          w: *mut lapack_complex_double, vs: *mut lapack_complex_double,
                          ldvs: lapack_int, rconde: *mut c_double, rcondv: *mut c_double)
                          -> lapack_int;

    pub fn LAPACKE_sgeev(matrix_layout: c_int, jobvl: c_char, jobvr: c_char, n: lapack_int,
                         a: *mut c_float, lda: lapack_int, wr: *mut c_float, wi: *mut c_float,
                         vl: *mut c_float, ldvl: lapack_int, vr: *mut c_float, ldvr: lapack_int)
                         -> lapack_int;
    pub fn LAPACKE_dgeev(matrix_layout: c_int, jobvl: c_char, jobvr: c_char, n: lapack_int,
                         a: *mut c_double, lda: lapack_int, wr: *mut c_double, wi: *mut c_double,
                         vl: *mut c_double, ldvl: lapack_int, vr: *mut c_double, ldvr: lapack_int)
                         -> lapack_int;
    pub fn LAPACKE_cgeev(matrix_layout: c_int, jobvl: c_char, jobvr: c_char, n: lapack_int,
                         a: *mut lapack_complex_float, lda: lapack_int,
                         w: *mut lapack_complex_float, vl: *mut lapack_complex_float,
                         ldvl: lapack_int, vr: *mut lapack_complex_float, ldvr: lapack_int)
                         -> lapack_int;
    pub fn LAPACKE_zgeev(matrix_layout: c_int, jobvl: c_char, jobvr: c_char, n: lapack_int,
                         a: *mut lapack_complex_double, lda: lapack_int,
                         w: *mut lapack_complex_double, vl: *mut lapack_complex_double,
                         ldvl: lapack_int, vr: *mut lapack_complex_double, ldvr: lapack_int)
                         -> lapack_int;

    pub fn LAPACKE_sgeevx(matrix_layout: c_int, balanc: c_char, jobvl: c_char, jobvr: c_char,
                          sense: c_char, n: lapack_int, a: *mut c_float, lda: lapack_int,
                          wr: *mut c_float, wi: *mut c_float, vl: *mut c_float, ldvl: lapack_int,
                          vr: *mut c_float, ldvr: lapack_int, ilo: *mut lapack_int,
                          ihi: *mut lapack_int, scale: *mut c_float, abnrm: *mut c_float,
                          rconde: *mut c_float, rcondv: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_dgeevx(matrix_layout: c_int, balanc: c_char, jobvl: c_char, jobvr: c_char,
                          sense: c_char, n: lapack_int, a: *mut c_double, lda: lapack_int,
                          wr: *mut c_double, wi: *mut c_double, vl: *mut c_double,
                          ldvl: lapack_int, vr: *mut c_double, ldvr: lapack_int,
                          ilo: *mut lapack_int, ihi: *mut lapack_int, scale: *mut c_double,
                          abnrm: *mut c_double, rconde: *mut c_double, rcondv: *mut c_double)
                          -> lapack_int;
    pub fn LAPACKE_cgeevx(matrix_layout: c_int, balanc: c_char, jobvl: c_char, jobvr: c_char,
                          sense: c_char, n: lapack_int, a: *mut lapack_complex_float,
                          lda: lapack_int, w: *mut lapack_complex_float,
                          vl: *mut lapack_complex_float, ldvl: lapack_int,
                          vr: *mut lapack_complex_float, ldvr: lapack_int, ilo: *mut lapack_int,
                          ihi: *mut lapack_int, scale: *mut c_float, abnrm: *mut c_float,
                          rconde: *mut c_float, rcondv: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_zgeevx(matrix_layout: c_int, balanc: c_char, jobvl: c_char, jobvr: c_char,
                          sense: c_char, n: lapack_int, a: *mut lapack_complex_double,
                          lda: lapack_int, w: *mut lapack_complex_double,
                          vl: *mut lapack_complex_double, ldvl: lapack_int,
                          vr: *mut lapack_complex_double, ldvr: lapack_int, ilo: *mut lapack_int,
                          ihi: *mut lapack_int, scale: *mut c_double, abnrm: *mut c_double,
                          rconde: *mut c_double, rcondv: *mut c_double)
                          -> lapack_int;

    pub fn LAPACKE_sgehrd(matrix_layout: c_int, n: lapack_int, ilo: lapack_int, ihi: lapack_int,
                          a: *mut c_float, lda: lapack_int, tau: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_dgehrd(matrix_layout: c_int, n: lapack_int, ilo: lapack_int, ihi: lapack_int,
                          a: *mut c_double, lda: lapack_int, tau: *mut c_double)
                          -> lapack_int;
    pub fn LAPACKE_cgehrd(matrix_layout: c_int, n: lapack_int, ilo: lapack_int, ihi: lapack_int,
                          a: *mut lapack_complex_float, lda: lapack_int,
                          tau: *mut lapack_complex_float)
                          -> lapack_int;
    pub fn LAPACKE_zgehrd(matrix_layout: c_int, n: lapack_int, ilo: lapack_int, ihi: lapack_int,
                          a: *mut lapack_complex_double, lda: lapack_int,
                          tau: *mut lapack_complex_double)
                          -> lapack_int;

    pub fn LAPACKE_sgejsv(matrix_layout: c_int, joba: c_char, jobu: c_char, jobv: c_char,
                          jobr: c_char, jobt: c_char, jobp: c_char, m: lapack_int, n: lapack_int,
                          a: *mut c_float, lda: lapack_int, sva: *mut c_float, u: *mut c_float,
                          ldu: lapack_int, v: *mut c_float, ldv: lapack_int, stat: *mut c_float,
                          istat: *mut lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_dgejsv(matrix_layout: c_int, joba: c_char, jobu: c_char, jobv: c_char,
                          jobr: c_char, jobt: c_char, jobp: c_char, m: lapack_int, n: lapack_int,
                          a: *mut c_double, lda: lapack_int, sva: *mut c_double, u: *mut c_double,
                          ldu: lapack_int, v: *mut c_double, ldv: lapack_int, stat: *mut c_double,
                          istat: *mut lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_cgejsv(matrix_layout: c_int, joba: c_char, jobu: c_char, jobv: c_char,
                          jobr: c_char, jobt: c_char, jobp: c_char, m: lapack_int, n: lapack_int,
                          a: *mut lapack_complex_float, lda: lapack_int, sva: *mut c_float,
                          u: *mut lapack_complex_float, ldu: lapack_int,
                          v: *mut lapack_complex_float, ldv: lapack_int, stat: *mut c_float,
                          istat: *mut lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_zgejsv(matrix_layout: c_int, joba: c_char, jobu: c_char, jobv: c_char,
                          jobr: c_char, jobt: c_char, jobp: c_char, m: lapack_int, n: lapack_int,
                          a: *mut lapack_complex_double, lda: lapack_int, sva: *mut c_double,
                          u: *mut lapack_complex_double, ldu: lapack_int,
                          v: *mut lapack_complex_double, ldv: lapack_int, stat: *mut c_double,
                          istat: *mut lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_sgelq2(matrix_layout: c_int, m: lapack_int, n: lapack_int, a: *mut c_float,
                          lda: lapack_int, tau: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_dgelq2(matrix_layout: c_int, m: lapack_int, n: lapack_int, a: *mut c_double,
                          lda: lapack_int, tau: *mut c_double)
                          -> lapack_int;
    pub fn LAPACKE_cgelq2(matrix_layout: c_int, m: lapack_int, n: lapack_int,
                          a: *mut lapack_complex_float, lda: lapack_int,
                          tau: *mut lapack_complex_float)
                          -> lapack_int;
    pub fn LAPACKE_zgelq2(matrix_layout: c_int, m: lapack_int, n: lapack_int,
                          a: *mut lapack_complex_double, lda: lapack_int,
                          tau: *mut lapack_complex_double)
                          -> lapack_int;

    pub fn LAPACKE_sgelqf(matrix_layout: c_int, m: lapack_int, n: lapack_int, a: *mut c_float,
                          lda: lapack_int, tau: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_dgelqf(matrix_layout: c_int, m: lapack_int, n: lapack_int, a: *mut c_double,
                          lda: lapack_int, tau: *mut c_double)
                          -> lapack_int;
    pub fn LAPACKE_cgelqf(matrix_layout: c_int, m: lapack_int, n: lapack_int,
                          a: *mut lapack_complex_float, lda: lapack_int,
                          tau: *mut lapack_complex_float)
                          -> lapack_int;
    pub fn LAPACKE_zgelqf(matrix_layout: c_int, m: lapack_int, n: lapack_int,
                          a: *mut lapack_complex_double, lda: lapack_int,
                          tau: *mut lapack_complex_double)
                          -> lapack_int;

    pub fn LAPACKE_sgels(matrix_layout: c_int, trans: c_char, m: lapack_int, n: lapack_int,
                         nrhs: lapack_int, a: *mut c_float, lda: lapack_int, b: *mut c_float,
                         ldb: lapack_int)
                         -> lapack_int;
    pub fn LAPACKE_dgels(matrix_layout: c_int, trans: c_char, m: lapack_int, n: lapack_int,
                         nrhs: lapack_int, a: *mut c_double, lda: lapack_int, b: *mut c_double,
                         ldb: lapack_int)
                         -> lapack_int;
    pub fn LAPACKE_cgels(matrix_layout: c_int, trans: c_char, m: lapack_int, n: lapack_int,
                         nrhs: lapack_int, a: *mut lapack_complex_float, lda: lapack_int,
                         b: *mut lapack_complex_float, ldb: lapack_int)
                         -> lapack_int;
    pub fn LAPACKE_zgels(matrix_layout: c_int, trans: c_char, m: lapack_int, n: lapack_int,
                         nrhs: lapack_int, a: *mut lapack_complex_double, lda: lapack_int,
                         b: *mut lapack_complex_double, ldb: lapack_int)
                         -> lapack_int;

    pub fn LAPACKE_sgelsd(matrix_layout: c_int, m: lapack_int, n: lapack_int, nrhs: lapack_int,
                          a: *mut c_float, lda: lapack_int, b: *mut c_float, ldb: lapack_int,
                          s: *mut c_float, rcond: c_float, rank: *mut lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_dgelsd(matrix_layout: c_int, m: lapack_int, n: lapack_int, nrhs: lapack_int,
                          a: *mut c_double, lda: lapack_int, b: *mut c_double, ldb: lapack_int,
                          s: *mut c_double, rcond: c_double, rank: *mut lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_cgelsd(matrix_layout: c_int, m: lapack_int, n: lapack_int, nrhs: lapack_int,
                          a: *mut lapack_complex_float, lda: lapack_int,
                          b: *mut lapack_complex_float, ldb: lapack_int, s: *mut c_float,
                          rcond: c_float, rank: *mut lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_zgelsd(matrix_layout: c_int, m: lapack_int, n: lapack_int, nrhs: lapack_int,
                          a: *mut lapack_complex_double, lda: lapack_int,
                          b: *mut lapack_complex_double, ldb: lapack_int, s: *mut c_double,
                          rcond: c_double, rank: *mut lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_sgelss(matrix_layout: c_int, m: lapack_int, n: lapack_int, nrhs: lapack_int,
                          a: *mut c_float, lda: lapack_int, b: *mut c_float, ldb: lapack_int,
                          s: *mut c_float, rcond: c_float, rank: *mut lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_dgelss(matrix_layout: c_int, m: lapack_int, n: lapack_int, nrhs: lapack_int,
                          a: *mut c_double, lda: lapack_int, b: *mut c_double, ldb: lapack_int,
                          s: *mut c_double, rcond: c_double, rank: *mut lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_cgelss(matrix_layout: c_int, m: lapack_int, n: lapack_int, nrhs: lapack_int,
                          a: *mut lapack_complex_float, lda: lapack_int,
                          b: *mut lapack_complex_float, ldb: lapack_int, s: *mut c_float,
                          rcond: c_float, rank: *mut lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_zgelss(matrix_layout: c_int, m: lapack_int, n: lapack_int, nrhs: lapack_int,
                          a: *mut lapack_complex_double, lda: lapack_int,
                          b: *mut lapack_complex_double, ldb: lapack_int, s: *mut c_double,
                          rcond: c_double, rank: *mut lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_sgelsy(matrix_layout: c_int, m: lapack_int, n: lapack_int, nrhs: lapack_int,
                          a: *mut c_float, lda: lapack_int, b: *mut c_float, ldb: lapack_int,
                          jpvt: *mut lapack_int, rcond: c_float, rank: *mut lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_dgelsy(matrix_layout: c_int, m: lapack_int, n: lapack_int, nrhs: lapack_int,
                          a: *mut c_double, lda: lapack_int, b: *mut c_double, ldb: lapack_int,
                          jpvt: *mut lapack_int, rcond: c_double, rank: *mut lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_cgelsy(matrix_layout: c_int, m: lapack_int, n: lapack_int, nrhs: lapack_int,
                          a: *mut lapack_complex_float, lda: lapack_int,
                          b: *mut lapack_complex_float, ldb: lapack_int, jpvt: *mut lapack_int,
                          rcond: c_float, rank: *mut lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_zgelsy(matrix_layout: c_int, m: lapack_int, n: lapack_int, nrhs: lapack_int,
                          a: *mut lapack_complex_double, lda: lapack_int,
                          b: *mut lapack_complex_double, ldb: lapack_int, jpvt: *mut lapack_int,
                          rcond: c_double, rank: *mut lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_sgeqlf(matrix_layout: c_int, m: lapack_int, n: lapack_int, a: *mut c_float,
                          lda: lapack_int, tau: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_dgeqlf(matrix_layout: c_int, m: lapack_int, n: lapack_int, a: *mut c_double,
                          lda: lapack_int, tau: *mut c_double)
                          -> lapack_int;
    pub fn LAPACKE_cgeqlf(matrix_layout: c_int, m: lapack_int, n: lapack_int,
                          a: *mut lapack_complex_float, lda: lapack_int,
                          tau: *mut lapack_complex_float)
                          -> lapack_int;
    pub fn LAPACKE_zgeqlf(matrix_layout: c_int, m: lapack_int, n: lapack_int,
                          a: *mut lapack_complex_double, lda: lapack_int,
                          tau: *mut lapack_complex_double)
                          -> lapack_int;

    pub fn LAPACKE_sgeqp3(matrix_layout: c_int, m: lapack_int, n: lapack_int, a: *mut c_float,
                          lda: lapack_int, jpvt: *mut lapack_int, tau: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_dgeqp3(matrix_layout: c_int, m: lapack_int, n: lapack_int, a: *mut c_double,
                          lda: lapack_int, jpvt: *mut lapack_int, tau: *mut c_double)
                          -> lapack_int;
    pub fn LAPACKE_cgeqp3(matrix_layout: c_int, m: lapack_int, n: lapack_int,
                          a: *mut lapack_complex_float, lda: lapack_int, jpvt: *mut lapack_int,
                          tau: *mut lapack_complex_float)
                          -> lapack_int;
    pub fn LAPACKE_zgeqp3(matrix_layout: c_int, m: lapack_int, n: lapack_int,
                          a: *mut lapack_complex_double, lda: lapack_int, jpvt: *mut lapack_int,
                          tau: *mut lapack_complex_double)
                          -> lapack_int;

    pub fn LAPACKE_sgeqpf(matrix_layout: c_int, m: lapack_int, n: lapack_int, a: *mut c_float,
                          lda: lapack_int, jpvt: *mut lapack_int, tau: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_dgeqpf(matrix_layout: c_int, m: lapack_int, n: lapack_int, a: *mut c_double,
                          lda: lapack_int, jpvt: *mut lapack_int, tau: *mut c_double)
                          -> lapack_int;
    pub fn LAPACKE_cgeqpf(matrix_layout: c_int, m: lapack_int, n: lapack_int,
                          a: *mut lapack_complex_float, lda: lapack_int, jpvt: *mut lapack_int,
                          tau: *mut lapack_complex_float)
                          -> lapack_int;
    pub fn LAPACKE_zgeqpf(matrix_layout: c_int, m: lapack_int, n: lapack_int,
                          a: *mut lapack_complex_double, lda: lapack_int, jpvt: *mut lapack_int,
                          tau: *mut lapack_complex_double)
                          -> lapack_int;

    pub fn LAPACKE_sgeqr2(matrix_layout: c_int, m: lapack_int, n: lapack_int, a: *mut c_float,
                          lda: lapack_int, tau: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_dgeqr2(matrix_layout: c_int, m: lapack_int, n: lapack_int, a: *mut c_double,
                          lda: lapack_int, tau: *mut c_double)
                          -> lapack_int;
    pub fn LAPACKE_cgeqr2(matrix_layout: c_int, m: lapack_int, n: lapack_int,
                          a: *mut lapack_complex_float, lda: lapack_int,
                          tau: *mut lapack_complex_float)
                          -> lapack_int;
    pub fn LAPACKE_zgeqr2(matrix_layout: c_int, m: lapack_int, n: lapack_int,
                          a: *mut lapack_complex_double, lda: lapack_int,
                          tau: *mut lapack_complex_double)
                          -> lapack_int;

    pub fn LAPACKE_sgeqrf(matrix_layout: c_int, m: lapack_int, n: lapack_int, a: *mut c_float,
                          lda: lapack_int, tau: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_dgeqrf(matrix_layout: c_int, m: lapack_int, n: lapack_int, a: *mut c_double,
                          lda: lapack_int, tau: *mut c_double)
                          -> lapack_int;
    pub fn LAPACKE_cgeqrf(matrix_layout: c_int, m: lapack_int, n: lapack_int,
                          a: *mut lapack_complex_float, lda: lapack_int,
                          tau: *mut lapack_complex_float)
                          -> lapack_int;
    pub fn LAPACKE_zgeqrf(matrix_layout: c_int, m: lapack_int, n: lapack_int,
                          a: *mut lapack_complex_double, lda: lapack_int,
                          tau: *mut lapack_complex_double)
                          -> lapack_int;

    pub fn LAPACKE_sgeqrfp(matrix_layout: c_int, m: lapack_int, n: lapack_int, a: *mut c_float,
                           lda: lapack_int, tau: *mut c_float)
                           -> lapack_int;
    pub fn LAPACKE_dgeqrfp(matrix_layout: c_int, m: lapack_int, n: lapack_int, a: *mut c_double,
                           lda: lapack_int, tau: *mut c_double)
                           -> lapack_int;
    pub fn LAPACKE_cgeqrfp(matrix_layout: c_int, m: lapack_int, n: lapack_int,
                           a: *mut lapack_complex_float, lda: lapack_int,
                           tau: *mut lapack_complex_float)
                           -> lapack_int;
    pub fn LAPACKE_zgeqrfp(matrix_layout: c_int, m: lapack_int, n: lapack_int,
                           a: *mut lapack_complex_double, lda: lapack_int,
                           tau: *mut lapack_complex_double)
                           -> lapack_int;

    pub fn LAPACKE_sgerfs(matrix_layout: c_int, trans: c_char, n: lapack_int, nrhs: lapack_int,
                          a: *const c_float, lda: lapack_int, af: *const c_float, ldaf: lapack_int,
                          ipiv: *const lapack_int, b: *const c_float, ldb: lapack_int,
                          x: *mut c_float, ldx: lapack_int, ferr: *mut c_float, berr: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_dgerfs(matrix_layout: c_int, trans: c_char, n: lapack_int, nrhs: lapack_int,
                          a: *const c_double, lda: lapack_int, af: *const c_double,
                          ldaf: lapack_int, ipiv: *const lapack_int, b: *const c_double,
                          ldb: lapack_int, x: *mut c_double, ldx: lapack_int, ferr: *mut c_double,
                          berr: *mut c_double)
                          -> lapack_int;
    pub fn LAPACKE_cgerfs(matrix_layout: c_int, trans: c_char, n: lapack_int, nrhs: lapack_int,
                          a: *const lapack_complex_float, lda: lapack_int,
                          af: *const lapack_complex_float, ldaf: lapack_int,
                          ipiv: *const lapack_int, b: *const lapack_complex_float, ldb: lapack_int,
                          x: *mut lapack_complex_float, ldx: lapack_int, ferr: *mut c_float,
                          berr: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_zgerfs(matrix_layout: c_int, trans: c_char, n: lapack_int, nrhs: lapack_int,
                          a: *const lapack_complex_double, lda: lapack_int,
                          af: *const lapack_complex_double, ldaf: lapack_int,
                          ipiv: *const lapack_int, b: *const lapack_complex_double,
                          ldb: lapack_int, x: *mut lapack_complex_double, ldx: lapack_int,
                          ferr: *mut c_double, berr: *mut c_double)
                          -> lapack_int;

    pub fn LAPACKE_sgerfsx(matrix_layout: c_int, trans: c_char, equed: c_char, n: lapack_int,
                           nrhs: lapack_int, a: *const c_float, lda: lapack_int,
                           af: *const c_float, ldaf: lapack_int, ipiv: *const lapack_int,
                           r: *const c_float, c: *const c_float, b: *const c_float,
                           ldb: lapack_int, x: *mut c_float, ldx: lapack_int, rcond: *mut c_float,
                           berr: *mut c_float, n_err_bnds: lapack_int, err_bnds_norm: *mut c_float,
                           err_bnds_comp: *mut c_float, nparams: lapack_int, params: *mut c_float)
                           -> lapack_int;
    pub fn LAPACKE_dgerfsx(matrix_layout: c_int, trans: c_char, equed: c_char, n: lapack_int,
                           nrhs: lapack_int, a: *const c_double, lda: lapack_int,
                           af: *const c_double, ldaf: lapack_int, ipiv: *const lapack_int,
                           r: *const c_double, c: *const c_double, b: *const c_double,
                           ldb: lapack_int, x: *mut c_double, ldx: lapack_int,
                           rcond: *mut c_double, berr: *mut c_double, n_err_bnds: lapack_int,
                           err_bnds_norm: *mut c_double, err_bnds_comp: *mut c_double,
                           nparams: lapack_int, params: *mut c_double)
                           -> lapack_int;
    pub fn LAPACKE_cgerfsx(matrix_layout: c_int, trans: c_char, equed: c_char, n: lapack_int,
                           nrhs: lapack_int, a: *const lapack_complex_float, lda: lapack_int,
                           af: *const lapack_complex_float, ldaf: lapack_int,
                           ipiv: *const lapack_int, r: *const c_float, c: *const c_float,
                           b: *const lapack_complex_float, ldb: lapack_int,
                           x: *mut lapack_complex_float, ldx: lapack_int, rcond: *mut c_float,
                           berr: *mut c_float, n_err_bnds: lapack_int, err_bnds_norm: *mut c_float,
                           err_bnds_comp: *mut c_float, nparams: lapack_int, params: *mut c_float)
                           -> lapack_int;
    pub fn LAPACKE_zgerfsx(matrix_layout: c_int, trans: c_char, equed: c_char, n: lapack_int,
                           nrhs: lapack_int, a: *const lapack_complex_double, lda: lapack_int,
                           af: *const lapack_complex_double, ldaf: lapack_int,
                           ipiv: *const lapack_int, r: *const c_double, c: *const c_double,
                           b: *const lapack_complex_double, ldb: lapack_int,
                           x: *mut lapack_complex_double, ldx: lapack_int, rcond: *mut c_double,
                           berr: *mut c_double, n_err_bnds: lapack_int,
                           err_bnds_norm: *mut c_double, err_bnds_comp: *mut c_double,
                           nparams: lapack_int, params: *mut c_double)
                           -> lapack_int;

    pub fn LAPACKE_sgerqf(matrix_layout: c_int, m: lapack_int, n: lapack_int, a: *mut c_float,
                          lda: lapack_int, tau: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_dgerqf(matrix_layout: c_int, m: lapack_int, n: lapack_int, a: *mut c_double,
                          lda: lapack_int, tau: *mut c_double)
                          -> lapack_int;
    pub fn LAPACKE_cgerqf(matrix_layout: c_int, m: lapack_int, n: lapack_int,
                          a: *mut lapack_complex_float, lda: lapack_int,
                          tau: *mut lapack_complex_float)
                          -> lapack_int;
    pub fn LAPACKE_zgerqf(matrix_layout: c_int, m: lapack_int, n: lapack_int,
                          a: *mut lapack_complex_double, lda: lapack_int,
                          tau: *mut lapack_complex_double)
                          -> lapack_int;

    pub fn LAPACKE_sgesdd(matrix_layout: c_int, jobz: c_char, m: lapack_int, n: lapack_int,
                          a: *mut c_float, lda: lapack_int, s: *mut c_float, u: *mut c_float,
                          ldu: lapack_int, vt: *mut c_float, ldvt: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_dgesdd(matrix_layout: c_int, jobz: c_char, m: lapack_int, n: lapack_int,
                          a: *mut c_double, lda: lapack_int, s: *mut c_double, u: *mut c_double,
                          ldu: lapack_int, vt: *mut c_double, ldvt: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_cgesdd(matrix_layout: c_int, jobz: c_char, m: lapack_int, n: lapack_int,
                          a: *mut lapack_complex_float, lda: lapack_int, s: *mut c_float,
                          u: *mut lapack_complex_float, ldu: lapack_int,
                          vt: *mut lapack_complex_float, ldvt: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_zgesdd(matrix_layout: c_int, jobz: c_char, m: lapack_int, n: lapack_int,
                          a: *mut lapack_complex_double, lda: lapack_int, s: *mut c_double,
                          u: *mut lapack_complex_double, ldu: lapack_int,
                          vt: *mut lapack_complex_double, ldvt: lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_sgesv(matrix_layout: c_int, n: lapack_int, nrhs: lapack_int, a: *mut c_float,
                         lda: lapack_int, ipiv: *mut lapack_int, b: *mut c_float, ldb: lapack_int)
                         -> lapack_int;
    pub fn LAPACKE_dgesv(matrix_layout: c_int, n: lapack_int, nrhs: lapack_int, a: *mut c_double,
                         lda: lapack_int, ipiv: *mut lapack_int, b: *mut c_double, ldb: lapack_int)
                         -> lapack_int;
    pub fn LAPACKE_cgesv(matrix_layout: c_int, n: lapack_int, nrhs: lapack_int,
                         a: *mut lapack_complex_float, lda: lapack_int, ipiv: *mut lapack_int,
                         b: *mut lapack_complex_float, ldb: lapack_int)
                         -> lapack_int;
    pub fn LAPACKE_zgesv(matrix_layout: c_int, n: lapack_int, nrhs: lapack_int,
                         a: *mut lapack_complex_double, lda: lapack_int, ipiv: *mut lapack_int,
                         b: *mut lapack_complex_double, ldb: lapack_int)
                         -> lapack_int;
    pub fn LAPACKE_dsgesv(matrix_layout: c_int, n: lapack_int, nrhs: lapack_int, a: *mut c_double,
                          lda: lapack_int, ipiv: *mut lapack_int, b: *mut c_double,
                          ldb: lapack_int, x: *mut c_double, ldx: lapack_int,
                          iter: *mut lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_zcgesv(matrix_layout: c_int, n: lapack_int, nrhs: lapack_int,
                          a: *mut lapack_complex_double, lda: lapack_int, ipiv: *mut lapack_int,
                          b: *mut lapack_complex_double, ldb: lapack_int,
                          x: *mut lapack_complex_double, ldx: lapack_int, iter: *mut lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_sgesvd(matrix_layout: c_int, jobu: c_char, jobvt: c_char, m: lapack_int,
                          n: lapack_int, a: *mut c_float, lda: lapack_int, s: *mut c_float,
                          u: *mut c_float, ldu: lapack_int, vt: *mut c_float, ldvt: lapack_int,
                          superb: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_dgesvd(matrix_layout: c_int, jobu: c_char, jobvt: c_char, m: lapack_int,
                          n: lapack_int, a: *mut c_double, lda: lapack_int, s: *mut c_double,
                          u: *mut c_double, ldu: lapack_int, vt: *mut c_double, ldvt: lapack_int,
                          superb: *mut c_double)
                          -> lapack_int;
    pub fn LAPACKE_cgesvd(matrix_layout: c_int, jobu: c_char, jobvt: c_char, m: lapack_int,
                          n: lapack_int, a: *mut lapack_complex_float, lda: lapack_int,
                          s: *mut c_float, u: *mut lapack_complex_float, ldu: lapack_int,
                          vt: *mut lapack_complex_float, ldvt: lapack_int, superb: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_zgesvd(matrix_layout: c_int, jobu: c_char, jobvt: c_char, m: lapack_int,
                          n: lapack_int, a: *mut lapack_complex_double, lda: lapack_int,
                          s: *mut c_double, u: *mut lapack_complex_double, ldu: lapack_int,
                          vt: *mut lapack_complex_double, ldvt: lapack_int, superb: *mut c_double)
                          -> lapack_int;

    pub fn LAPACKE_sgesvdx(matrix_layout: c_int, jobu: c_char, jobvt: c_char, range: c_char,
                           m: lapack_int, n: lapack_int, a: *mut c_float, lda: lapack_int,
                           vl: c_float, vu: c_float, il: lapack_int, iu: lapack_int,
                           ns: lapack_int, s: *mut c_float, u: *mut c_float, ldu: lapack_int,
                           vt: *mut c_float, ldvt: lapack_int, superb: *mut lapack_int)
                           -> lapack_int;
    pub fn LAPACKE_dgesvdx(matrix_layout: c_int, jobu: c_char, jobvt: c_char, range: c_char,
                           m: lapack_int, n: lapack_int, a: *mut c_double, lda: lapack_int,
                           vl: c_double, vu: c_double, il: lapack_int, iu: lapack_int,
                           ns: lapack_int, s: *mut c_double, u: *mut c_double, ldu: lapack_int,
                           vt: *mut c_double, ldvt: lapack_int, superb: *mut lapack_int)
                           -> lapack_int;
    pub fn LAPACKE_cgesvdx(matrix_layout: c_int, jobu: c_char, jobvt: c_char, range: c_char,
                           m: lapack_int, n: lapack_int, a: *mut lapack_complex_float,
                           lda: lapack_int, vl: c_float, vu: c_float, il: lapack_int,
                           iu: lapack_int, ns: lapack_int, s: *mut c_float,
                           u: *mut lapack_complex_float, ldu: lapack_int,
                           vt: *mut lapack_complex_float, ldvt: lapack_int,
                           superb: *mut lapack_int)
                           -> lapack_int;
    pub fn LAPACKE_zgesvdx(matrix_layout: c_int, jobu: c_char, jobvt: c_char, range: c_char,
                           m: lapack_int, n: lapack_int, a: *mut lapack_complex_double,
                           lda: lapack_int, vl: c_double, vu: c_double, il: lapack_int,
                           iu: lapack_int, ns: lapack_int, s: *mut c_double,
                           u: *mut lapack_complex_double, ldu: lapack_int,
                           vt: *mut lapack_complex_double, ldvt: lapack_int,
                           superb: *mut lapack_int)
                           -> lapack_int;

    pub fn LAPACKE_sgesvj(matrix_layout: c_int, joba: c_char, jobu: c_char, jobv: c_char,
                          m: lapack_int, n: lapack_int, a: *mut c_float, lda: lapack_int,
                          sva: *mut c_float, mv: lapack_int, v: *mut c_float, ldv: lapack_int,
                          stat: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_dgesvj(matrix_layout: c_int, joba: c_char, jobu: c_char, jobv: c_char,
                          m: lapack_int, n: lapack_int, a: *mut c_double, lda: lapack_int,
                          sva: *mut c_double, mv: lapack_int, v: *mut c_double, ldv: lapack_int,
                          stat: *mut c_double)
                          -> lapack_int;
    pub fn LAPACKE_cgesvj(matrix_layout: c_int, joba: c_char, jobu: c_char, jobv: c_char,
                          m: lapack_int, n: lapack_int, a: *mut lapack_complex_float,
                          lda: lapack_int, sva: *mut c_float, mv: lapack_int,
                          v: *mut lapack_complex_float, ldv: lapack_int, stat: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_zgesvj(matrix_layout: c_int, joba: c_char, jobu: c_char, jobv: c_char,
                          m: lapack_int, n: lapack_int, a: *mut lapack_complex_double,
                          lda: lapack_int, sva: *mut c_double, mv: lapack_int,
                          v: *mut lapack_complex_double, ldv: lapack_int, stat: *mut c_double)
                          -> lapack_int;

    pub fn LAPACKE_sgesvx(matrix_layout: c_int, fact: c_char, trans: c_char, n: lapack_int,
                          nrhs: lapack_int, a: *mut c_float, lda: lapack_int, af: *mut c_float,
                          ldaf: lapack_int, ipiv: *mut lapack_int, equed: *mut c_char,
                          r: *mut c_float, c: *mut c_float, b: *mut c_float, ldb: lapack_int,
                          x: *mut c_float, ldx: lapack_int, rcond: *mut c_float,
                          ferr: *mut c_float, berr: *mut c_float, rpivot: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_dgesvx(matrix_layout: c_int, fact: c_char, trans: c_char, n: lapack_int,
                          nrhs: lapack_int, a: *mut c_double, lda: lapack_int, af: *mut c_double,
                          ldaf: lapack_int, ipiv: *mut lapack_int, equed: *mut c_char,
                          r: *mut c_double, c: *mut c_double, b: *mut c_double, ldb: lapack_int,
                          x: *mut c_double, ldx: lapack_int, rcond: *mut c_double,
                          ferr: *mut c_double, berr: *mut c_double, rpivot: *mut c_double)
                          -> lapack_int;
    pub fn LAPACKE_cgesvx(matrix_layout: c_int, fact: c_char, trans: c_char, n: lapack_int,
                          nrhs: lapack_int, a: *mut lapack_complex_float, lda: lapack_int,
                          af: *mut lapack_complex_float, ldaf: lapack_int, ipiv: *mut lapack_int,
                          equed: *mut c_char, r: *mut c_float, c: *mut c_float,
                          b: *mut lapack_complex_float, ldb: lapack_int,
                          x: *mut lapack_complex_float, ldx: lapack_int, rcond: *mut c_float,
                          ferr: *mut c_float, berr: *mut c_float, rpivot: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_zgesvx(matrix_layout: c_int, fact: c_char, trans: c_char, n: lapack_int,
                          nrhs: lapack_int, a: *mut lapack_complex_double, lda: lapack_int,
                          af: *mut lapack_complex_double, ldaf: lapack_int, ipiv: *mut lapack_int,
                          equed: *mut c_char, r: *mut c_double, c: *mut c_double,
                          b: *mut lapack_complex_double, ldb: lapack_int,
                          x: *mut lapack_complex_double, ldx: lapack_int, rcond: *mut c_double,
                          ferr: *mut c_double, berr: *mut c_double, rpivot: *mut c_double)
                          -> lapack_int;

    pub fn LAPACKE_sgesvxx(matrix_layout: c_int, fact: c_char, trans: c_char, n: lapack_int,
                           nrhs: lapack_int, a: *mut c_float, lda: lapack_int, af: *mut c_float,
                           ldaf: lapack_int, ipiv: *mut lapack_int, equed: *mut c_char,
                           r: *mut c_float, c: *mut c_float, b: *mut c_float, ldb: lapack_int,
                           x: *mut c_float, ldx: lapack_int, rcond: *mut c_float,
                           rpvgrw: *mut c_float, berr: *mut c_float, n_err_bnds: lapack_int,
                           err_bnds_norm: *mut c_float, err_bnds_comp: *mut c_float,
                           nparams: lapack_int, params: *mut c_float)
                           -> lapack_int;
    pub fn LAPACKE_dgesvxx(matrix_layout: c_int, fact: c_char, trans: c_char, n: lapack_int,
                           nrhs: lapack_int, a: *mut c_double, lda: lapack_int, af: *mut c_double,
                           ldaf: lapack_int, ipiv: *mut lapack_int, equed: *mut c_char,
                           r: *mut c_double, c: *mut c_double, b: *mut c_double, ldb: lapack_int,
                           x: *mut c_double, ldx: lapack_int, rcond: *mut c_double,
                           rpvgrw: *mut c_double, berr: *mut c_double, n_err_bnds: lapack_int,
                           err_bnds_norm: *mut c_double, err_bnds_comp: *mut c_double,
                           nparams: lapack_int, params: *mut c_double)
                           -> lapack_int;
    pub fn LAPACKE_cgesvxx(matrix_layout: c_int, fact: c_char, trans: c_char, n: lapack_int,
                           nrhs: lapack_int, a: *mut lapack_complex_float, lda: lapack_int,
                           af: *mut lapack_complex_float, ldaf: lapack_int, ipiv: *mut lapack_int,
                           equed: *mut c_char, r: *mut c_float, c: *mut c_float,
                           b: *mut lapack_complex_float, ldb: lapack_int,
                           x: *mut lapack_complex_float, ldx: lapack_int, rcond: *mut c_float,
                           rpvgrw: *mut c_float, berr: *mut c_float, n_err_bnds: lapack_int,
                           err_bnds_norm: *mut c_float, err_bnds_comp: *mut c_float,
                           nparams: lapack_int, params: *mut c_float)
                           -> lapack_int;
    pub fn LAPACKE_zgesvxx(matrix_layout: c_int, fact: c_char, trans: c_char, n: lapack_int,
                           nrhs: lapack_int, a: *mut lapack_complex_double, lda: lapack_int,
                           af: *mut lapack_complex_double, ldaf: lapack_int, ipiv: *mut lapack_int,
                           equed: *mut c_char, r: *mut c_double, c: *mut c_double,
                           b: *mut lapack_complex_double, ldb: lapack_int,
                           x: *mut lapack_complex_double, ldx: lapack_int, rcond: *mut c_double,
                           rpvgrw: *mut c_double, berr: *mut c_double, n_err_bnds: lapack_int,
                           err_bnds_norm: *mut c_double, err_bnds_comp: *mut c_double,
                           nparams: lapack_int, params: *mut c_double)
                           -> lapack_int;

    pub fn LAPACKE_sgetf2(matrix_layout: c_int, m: lapack_int, n: lapack_int, a: *mut c_float,
                          lda: lapack_int, ipiv: *mut lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_dgetf2(matrix_layout: c_int, m: lapack_int, n: lapack_int, a: *mut c_double,
                          lda: lapack_int, ipiv: *mut lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_cgetf2(matrix_layout: c_int, m: lapack_int, n: lapack_int,
                          a: *mut lapack_complex_float, lda: lapack_int, ipiv: *mut lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_zgetf2(matrix_layout: c_int, m: lapack_int, n: lapack_int,
                          a: *mut lapack_complex_double, lda: lapack_int, ipiv: *mut lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_sgetrf(matrix_layout: c_int, m: lapack_int, n: lapack_int, a: *mut c_float,
                          lda: lapack_int, ipiv: *mut lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_dgetrf(matrix_layout: c_int, m: lapack_int, n: lapack_int, a: *mut c_double,
                          lda: lapack_int, ipiv: *mut lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_cgetrf(matrix_layout: c_int, m: lapack_int, n: lapack_int,
                          a: *mut lapack_complex_float, lda: lapack_int, ipiv: *mut lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_zgetrf(matrix_layout: c_int, m: lapack_int, n: lapack_int,
                          a: *mut lapack_complex_double, lda: lapack_int, ipiv: *mut lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_sgetrf2(matrix_layout: c_int, m: lapack_int, n: lapack_int, a: *mut c_float,
                           lda: lapack_int, ipiv: *mut lapack_int)
                           -> lapack_int;
    pub fn LAPACKE_dgetrf2(matrix_layout: c_int, m: lapack_int, n: lapack_int, a: *mut c_double,
                           lda: lapack_int, ipiv: *mut lapack_int)
                           -> lapack_int;
    pub fn LAPACKE_cgetrf2(matrix_layout: c_int, m: lapack_int, n: lapack_int,
                           a: *mut lapack_complex_float, lda: lapack_int, ipiv: *mut lapack_int)
                           -> lapack_int;
    pub fn LAPACKE_zgetrf2(matrix_layout: c_int, m: lapack_int, n: lapack_int,
                           a: *mut lapack_complex_double, lda: lapack_int, ipiv: *mut lapack_int)
                           -> lapack_int;

    pub fn LAPACKE_sgetri(matrix_layout: c_int, n: lapack_int, a: *mut c_float, lda: lapack_int,
                          ipiv: *const lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_dgetri(matrix_layout: c_int, n: lapack_int, a: *mut c_double, lda: lapack_int,
                          ipiv: *const lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_cgetri(matrix_layout: c_int, n: lapack_int, a: *mut lapack_complex_float,
                          lda: lapack_int, ipiv: *const lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_zgetri(matrix_layout: c_int, n: lapack_int, a: *mut lapack_complex_double,
                          lda: lapack_int, ipiv: *const lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_sgetrs(matrix_layout: c_int, trans: c_char, n: lapack_int, nrhs: lapack_int,
                          a: *const c_float, lda: lapack_int, ipiv: *const lapack_int,
                          b: *mut c_float, ldb: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_dgetrs(matrix_layout: c_int, trans: c_char, n: lapack_int, nrhs: lapack_int,
                          a: *const c_double, lda: lapack_int, ipiv: *const lapack_int,
                          b: *mut c_double, ldb: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_cgetrs(matrix_layout: c_int, trans: c_char, n: lapack_int, nrhs: lapack_int,
                          a: *const lapack_complex_float, lda: lapack_int, ipiv: *const lapack_int,
                          b: *mut lapack_complex_float, ldb: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_zgetrs(matrix_layout: c_int, trans: c_char, n: lapack_int, nrhs: lapack_int,
                          a: *const lapack_complex_double, lda: lapack_int,
                          ipiv: *const lapack_int, b: *mut lapack_complex_double, ldb: lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_sggbak(matrix_layout: c_int, job: c_char, side: c_char, n: lapack_int,
                          ilo: lapack_int, ihi: lapack_int, lscale: *const c_float,
                          rscale: *const c_float, m: lapack_int, v: *mut c_float, ldv: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_dggbak(matrix_layout: c_int, job: c_char, side: c_char, n: lapack_int,
                          ilo: lapack_int, ihi: lapack_int, lscale: *const c_double,
                          rscale: *const c_double, m: lapack_int, v: *mut c_double,
                          ldv: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_cggbak(matrix_layout: c_int, job: c_char, side: c_char, n: lapack_int,
                          ilo: lapack_int, ihi: lapack_int, lscale: *const c_float,
                          rscale: *const c_float, m: lapack_int, v: *mut lapack_complex_float,
                          ldv: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_zggbak(matrix_layout: c_int, job: c_char, side: c_char, n: lapack_int,
                          ilo: lapack_int, ihi: lapack_int, lscale: *const c_double,
                          rscale: *const c_double, m: lapack_int, v: *mut lapack_complex_double,
                          ldv: lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_sggbal(matrix_layout: c_int, job: c_char, n: lapack_int, a: *mut c_float,
                          lda: lapack_int, b: *mut c_float, ldb: lapack_int, ilo: *mut lapack_int,
                          ihi: *mut lapack_int, lscale: *mut c_float, rscale: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_dggbal(matrix_layout: c_int, job: c_char, n: lapack_int, a: *mut c_double,
                          lda: lapack_int, b: *mut c_double, ldb: lapack_int, ilo: *mut lapack_int,
                          ihi: *mut lapack_int, lscale: *mut c_double, rscale: *mut c_double)
                          -> lapack_int;
    pub fn LAPACKE_cggbal(matrix_layout: c_int, job: c_char, n: lapack_int,
                          a: *mut lapack_complex_float, lda: lapack_int,
                          b: *mut lapack_complex_float, ldb: lapack_int, ilo: *mut lapack_int,
                          ihi: *mut lapack_int, lscale: *mut c_float, rscale: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_zggbal(matrix_layout: c_int, job: c_char, n: lapack_int,
                          a: *mut lapack_complex_double, lda: lapack_int,
                          b: *mut lapack_complex_double, ldb: lapack_int, ilo: *mut lapack_int,
                          ihi: *mut lapack_int, lscale: *mut c_double, rscale: *mut c_double)
                          -> lapack_int;

    pub fn LAPACKE_sgges(matrix_layout: c_int, jobvsl: c_char, jobvsr: c_char, sort: c_char,
                         selctg: LAPACK_S_SELECT3, n: lapack_int, a: *mut c_float, lda: lapack_int,
                         b: *mut c_float, ldb: lapack_int, sdim: *mut lapack_int,
                         alphar: *mut c_float, alphai: *mut c_float, beta: *mut c_float,
                         vsl: *mut c_float, ldvsl: lapack_int, vsr: *mut c_float,
                         ldvsr: lapack_int)
                         -> lapack_int;
    pub fn LAPACKE_dgges(matrix_layout: c_int, jobvsl: c_char, jobvsr: c_char, sort: c_char,
                         selctg: LAPACK_D_SELECT3, n: lapack_int, a: *mut c_double,
                         lda: lapack_int, b: *mut c_double, ldb: lapack_int, sdim: *mut lapack_int,
                         alphar: *mut c_double, alphai: *mut c_double, beta: *mut c_double,
                         vsl: *mut c_double, ldvsl: lapack_int, vsr: *mut c_double,
                         ldvsr: lapack_int)
                         -> lapack_int;
    pub fn LAPACKE_cgges(matrix_layout: c_int, jobvsl: c_char, jobvsr: c_char, sort: c_char,
                         selctg: LAPACK_C_SELECT2, n: lapack_int, a: *mut lapack_complex_float,
                         lda: lapack_int, b: *mut lapack_complex_float, ldb: lapack_int,
                         sdim: *mut lapack_int, alpha: *mut lapack_complex_float,
                         beta: *mut lapack_complex_float, vsl: *mut lapack_complex_float,
                         ldvsl: lapack_int, vsr: *mut lapack_complex_float, ldvsr: lapack_int)
                         -> lapack_int;
    pub fn LAPACKE_zgges(matrix_layout: c_int, jobvsl: c_char, jobvsr: c_char, sort: c_char,
                         selctg: LAPACK_Z_SELECT2, n: lapack_int, a: *mut lapack_complex_double,
                         lda: lapack_int, b: *mut lapack_complex_double, ldb: lapack_int,
                         sdim: *mut lapack_int, alpha: *mut lapack_complex_double,
                         beta: *mut lapack_complex_double, vsl: *mut lapack_complex_double,
                         ldvsl: lapack_int, vsr: *mut lapack_complex_double, ldvsr: lapack_int)
                         -> lapack_int;

    pub fn LAPACKE_sgges3(matrix_layout: c_int, jobvsl: c_char, jobvsr: c_char, sort: c_char,
                          selctg: LAPACK_S_SELECT3, n: lapack_int, a: *mut c_float,
                          lda: lapack_int, b: *mut c_float, ldb: lapack_int, sdim: *mut lapack_int,
                          alphar: *mut c_float, alphai: *mut c_float, beta: *mut c_float,
                          vsl: *mut c_float, ldvsl: lapack_int, vsr: *mut c_float,
                          ldvsr: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_dgges3(matrix_layout: c_int, jobvsl: c_char, jobvsr: c_char, sort: c_char,
                          selctg: LAPACK_D_SELECT3, n: lapack_int, a: *mut c_double,
                          lda: lapack_int, b: *mut c_double, ldb: lapack_int,
                          sdim: *mut lapack_int, alphar: *mut c_double, alphai: *mut c_double,
                          beta: *mut c_double, vsl: *mut c_double, ldvsl: lapack_int,
                          vsr: *mut c_double, ldvsr: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_cgges3(matrix_layout: c_int, jobvsl: c_char, jobvsr: c_char, sort: c_char,
                          selctg: LAPACK_C_SELECT2, n: lapack_int, a: *mut lapack_complex_float,
                          lda: lapack_int, b: *mut lapack_complex_float, ldb: lapack_int,
                          sdim: *mut lapack_int, alpha: *mut lapack_complex_float,
                          beta: *mut lapack_complex_float, vsl: *mut lapack_complex_float,
                          ldvsl: lapack_int, vsr: *mut lapack_complex_float, ldvsr: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_zgges3(matrix_layout: c_int, jobvsl: c_char, jobvsr: c_char, sort: c_char,
                          selctg: LAPACK_Z_SELECT2, n: lapack_int, a: *mut lapack_complex_double,
                          lda: lapack_int, b: *mut lapack_complex_double, ldb: lapack_int,
                          sdim: *mut lapack_int, alpha: *mut lapack_complex_double,
                          beta: *mut lapack_complex_double, vsl: *mut lapack_complex_double,
                          ldvsl: lapack_int, vsr: *mut lapack_complex_double, ldvsr: lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_sggesx(matrix_layout: c_int, jobvsl: c_char, jobvsr: c_char, sort: c_char,
                          selctg: LAPACK_S_SELECT3, sense: c_char, n: lapack_int, a: *mut c_float,
                          lda: lapack_int, b: *mut c_float, ldb: lapack_int, sdim: *mut lapack_int,
                          alphar: *mut c_float, alphai: *mut c_float, beta: *mut c_float,
                          vsl: *mut c_float, ldvsl: lapack_int, vsr: *mut c_float,
                          ldvsr: lapack_int, rconde: *mut c_float, rcondv: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_dggesx(matrix_layout: c_int, jobvsl: c_char, jobvsr: c_char, sort: c_char,
                          selctg: LAPACK_D_SELECT3, sense: c_char, n: lapack_int, a: *mut c_double,
                          lda: lapack_int, b: *mut c_double, ldb: lapack_int,
                          sdim: *mut lapack_int, alphar: *mut c_double, alphai: *mut c_double,
                          beta: *mut c_double, vsl: *mut c_double, ldvsl: lapack_int,
                          vsr: *mut c_double, ldvsr: lapack_int, rconde: *mut c_double,
                          rcondv: *mut c_double)
                          -> lapack_int;
    pub fn LAPACKE_cggesx(matrix_layout: c_int, jobvsl: c_char, jobvsr: c_char, sort: c_char,
                          selctg: LAPACK_C_SELECT2, sense: c_char, n: lapack_int,
                          a: *mut lapack_complex_float, lda: lapack_int,
                          b: *mut lapack_complex_float, ldb: lapack_int, sdim: *mut lapack_int,
                          alpha: *mut lapack_complex_float, beta: *mut lapack_complex_float,
                          vsl: *mut lapack_complex_float, ldvsl: lapack_int,
                          vsr: *mut lapack_complex_float, ldvsr: lapack_int, rconde: *mut c_float,
                          rcondv: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_zggesx(matrix_layout: c_int, jobvsl: c_char, jobvsr: c_char, sort: c_char,
                          selctg: LAPACK_Z_SELECT2, sense: c_char, n: lapack_int,
                          a: *mut lapack_complex_double, lda: lapack_int,
                          b: *mut lapack_complex_double, ldb: lapack_int, sdim: *mut lapack_int,
                          alpha: *mut lapack_complex_double, beta: *mut lapack_complex_double,
                          vsl: *mut lapack_complex_double, ldvsl: lapack_int,
                          vsr: *mut lapack_complex_double, ldvsr: lapack_int,
                          rconde: *mut c_double, rcondv: *mut c_double)
                          -> lapack_int;

    pub fn LAPACKE_sggev(matrix_layout: c_int, jobvl: c_char, jobvr: c_char, n: lapack_int,
                         a: *mut c_float, lda: lapack_int, b: *mut c_float, ldb: lapack_int,
                         alphar: *mut c_float, alphai: *mut c_float, beta: *mut c_float,
                         vl: *mut c_float, ldvl: lapack_int, vr: *mut c_float, ldvr: lapack_int)
                         -> lapack_int;
    pub fn LAPACKE_dggev(matrix_layout: c_int, jobvl: c_char, jobvr: c_char, n: lapack_int,
                         a: *mut c_double, lda: lapack_int, b: *mut c_double, ldb: lapack_int,
                         alphar: *mut c_double, alphai: *mut c_double, beta: *mut c_double,
                         vl: *mut c_double, ldvl: lapack_int, vr: *mut c_double, ldvr: lapack_int)
                         -> lapack_int;
    pub fn LAPACKE_cggev(matrix_layout: c_int, jobvl: c_char, jobvr: c_char, n: lapack_int,
                         a: *mut lapack_complex_float, lda: lapack_int,
                         b: *mut lapack_complex_float, ldb: lapack_int,
                         alpha: *mut lapack_complex_float, beta: *mut lapack_complex_float,
                         vl: *mut lapack_complex_float, ldvl: lapack_int,
                         vr: *mut lapack_complex_float, ldvr: lapack_int)
                         -> lapack_int;
    pub fn LAPACKE_zggev(matrix_layout: c_int, jobvl: c_char, jobvr: c_char, n: lapack_int,
                         a: *mut lapack_complex_double, lda: lapack_int,
                         b: *mut lapack_complex_double, ldb: lapack_int,
                         alpha: *mut lapack_complex_double, beta: *mut lapack_complex_double,
                         vl: *mut lapack_complex_double, ldvl: lapack_int,
                         vr: *mut lapack_complex_double, ldvr: lapack_int)
                         -> lapack_int;

    pub fn LAPACKE_sggev3(matrix_layout: c_int, jobvl: c_char, jobvr: c_char, n: lapack_int,
                          a: *mut c_float, lda: lapack_int, b: *mut c_float, ldb: lapack_int,
                          alphar: *mut c_float, alphai: *mut c_float, beta: *mut c_float,
                          vl: *mut c_float, ldvl: lapack_int, vr: *mut c_float, ldvr: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_dggev3(matrix_layout: c_int, jobvl: c_char, jobvr: c_char, n: lapack_int,
                          a: *mut c_double, lda: lapack_int, b: *mut c_double, ldb: lapack_int,
                          alphar: *mut c_double, alphai: *mut c_double, beta: *mut c_double,
                          vl: *mut c_double, ldvl: lapack_int, vr: *mut c_double, ldvr: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_cggev3(matrix_layout: c_int, jobvl: c_char, jobvr: c_char, n: lapack_int,
                          a: *mut lapack_complex_float, lda: lapack_int,
                          b: *mut lapack_complex_float, ldb: lapack_int,
                          alpha: *mut lapack_complex_float, beta: *mut lapack_complex_float,
                          vl: *mut lapack_complex_float, ldvl: lapack_int,
                          vr: *mut lapack_complex_float, ldvr: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_zggev3(matrix_layout: c_int, jobvl: c_char, jobvr: c_char, n: lapack_int,
                          a: *mut lapack_complex_double, lda: lapack_int,
                          b: *mut lapack_complex_double, ldb: lapack_int,
                          alpha: *mut lapack_complex_double, beta: *mut lapack_complex_double,
                          vl: *mut lapack_complex_double, ldvl: lapack_int,
                          vr: *mut lapack_complex_double, ldvr: lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_sggevx(matrix_layout: c_int, balanc: c_char, jobvl: c_char, jobvr: c_char,
                          sense: c_char, n: lapack_int, a: *mut c_float, lda: lapack_int,
                          b: *mut c_float, ldb: lapack_int, alphar: *mut c_float,
                          alphai: *mut c_float, beta: *mut c_float, vl: *mut c_float,
                          ldvl: lapack_int, vr: *mut c_float, ldvr: lapack_int,
                          ilo: *mut lapack_int, ihi: *mut lapack_int, lscale: *mut c_float,
                          rscale: *mut c_float, abnrm: *mut c_float, bbnrm: *mut c_float,
                          rconde: *mut c_float, rcondv: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_dggevx(matrix_layout: c_int, balanc: c_char, jobvl: c_char, jobvr: c_char,
                          sense: c_char, n: lapack_int, a: *mut c_double, lda: lapack_int,
                          b: *mut c_double, ldb: lapack_int, alphar: *mut c_double,
                          alphai: *mut c_double, beta: *mut c_double, vl: *mut c_double,
                          ldvl: lapack_int, vr: *mut c_double, ldvr: lapack_int,
                          ilo: *mut lapack_int, ihi: *mut lapack_int, lscale: *mut c_double,
                          rscale: *mut c_double, abnrm: *mut c_double, bbnrm: *mut c_double,
                          rconde: *mut c_double, rcondv: *mut c_double)
                          -> lapack_int;
    pub fn LAPACKE_cggevx(matrix_layout: c_int, balanc: c_char, jobvl: c_char, jobvr: c_char,
                          sense: c_char, n: lapack_int, a: *mut lapack_complex_float,
                          lda: lapack_int, b: *mut lapack_complex_float, ldb: lapack_int,
                          alpha: *mut lapack_complex_float, beta: *mut lapack_complex_float,
                          vl: *mut lapack_complex_float, ldvl: lapack_int,
                          vr: *mut lapack_complex_float, ldvr: lapack_int, ilo: *mut lapack_int,
                          ihi: *mut lapack_int, lscale: *mut c_float, rscale: *mut c_float,
                          abnrm: *mut c_float, bbnrm: *mut c_float, rconde: *mut c_float,
                          rcondv: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_zggevx(matrix_layout: c_int, balanc: c_char, jobvl: c_char, jobvr: c_char,
                          sense: c_char, n: lapack_int, a: *mut lapack_complex_double,
                          lda: lapack_int, b: *mut lapack_complex_double, ldb: lapack_int,
                          alpha: *mut lapack_complex_double, beta: *mut lapack_complex_double,
                          vl: *mut lapack_complex_double, ldvl: lapack_int,
                          vr: *mut lapack_complex_double, ldvr: lapack_int, ilo: *mut lapack_int,
                          ihi: *mut lapack_int, lscale: *mut c_double, rscale: *mut c_double,
                          abnrm: *mut c_double, bbnrm: *mut c_double, rconde: *mut c_double,
                          rcondv: *mut c_double)
                          -> lapack_int;

    pub fn LAPACKE_sggglm(matrix_layout: c_int, n: lapack_int, m: lapack_int, p: lapack_int,
                          a: *mut c_float, lda: lapack_int, b: *mut c_float, ldb: lapack_int,
                          d: *mut c_float, x: *mut c_float, y: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_dggglm(matrix_layout: c_int, n: lapack_int, m: lapack_int, p: lapack_int,
                          a: *mut c_double, lda: lapack_int, b: *mut c_double, ldb: lapack_int,
                          d: *mut c_double, x: *mut c_double, y: *mut c_double)
                          -> lapack_int;
    pub fn LAPACKE_cggglm(matrix_layout: c_int, n: lapack_int, m: lapack_int, p: lapack_int,
                          a: *mut lapack_complex_float, lda: lapack_int,
                          b: *mut lapack_complex_float, ldb: lapack_int,
                          d: *mut lapack_complex_float, x: *mut lapack_complex_float,
                          y: *mut lapack_complex_float)
                          -> lapack_int;
    pub fn LAPACKE_zggglm(matrix_layout: c_int, n: lapack_int, m: lapack_int, p: lapack_int,
                          a: *mut lapack_complex_double, lda: lapack_int,
                          b: *mut lapack_complex_double, ldb: lapack_int,
                          d: *mut lapack_complex_double, x: *mut lapack_complex_double,
                          y: *mut lapack_complex_double)
                          -> lapack_int;

    pub fn LAPACKE_sgghrd(matrix_layout: c_int, compq: c_char, compz: c_char, n: lapack_int,
                          ilo: lapack_int, ihi: lapack_int, a: *mut c_float, lda: lapack_int,
                          b: *mut c_float, ldb: lapack_int, q: *mut c_float, ldq: lapack_int,
                          z: *mut c_float, ldz: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_dgghrd(matrix_layout: c_int, compq: c_char, compz: c_char, n: lapack_int,
                          ilo: lapack_int, ihi: lapack_int, a: *mut c_double, lda: lapack_int,
                          b: *mut c_double, ldb: lapack_int, q: *mut c_double, ldq: lapack_int,
                          z: *mut c_double, ldz: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_cgghrd(matrix_layout: c_int, compq: c_char, compz: c_char, n: lapack_int,
                          ilo: lapack_int, ihi: lapack_int, a: *mut lapack_complex_float,
                          lda: lapack_int, b: *mut lapack_complex_float, ldb: lapack_int,
                          q: *mut lapack_complex_float, ldq: lapack_int,
                          z: *mut lapack_complex_float, ldz: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_zgghrd(matrix_layout: c_int, compq: c_char, compz: c_char, n: lapack_int,
                          ilo: lapack_int, ihi: lapack_int, a: *mut lapack_complex_double,
                          lda: lapack_int, b: *mut lapack_complex_double, ldb: lapack_int,
                          q: *mut lapack_complex_double, ldq: lapack_int,
                          z: *mut lapack_complex_double, ldz: lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_sgghd3(matrix_layout: c_int, compq: c_char, compz: c_char, n: lapack_int,
                          ilo: lapack_int, ihi: lapack_int, a: *mut c_float, lda: lapack_int,
                          b: *mut c_float, ldb: lapack_int, q: *mut c_float, ldq: lapack_int,
                          z: *mut c_float, ldz: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_dgghd3(matrix_layout: c_int, compq: c_char, compz: c_char, n: lapack_int,
                          ilo: lapack_int, ihi: lapack_int, a: *mut c_double, lda: lapack_int,
                          b: *mut c_double, ldb: lapack_int, q: *mut c_double, ldq: lapack_int,
                          z: *mut c_double, ldz: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_cgghd3(matrix_layout: c_int, compq: c_char, compz: c_char, n: lapack_int,
                          ilo: lapack_int, ihi: lapack_int, a: *mut lapack_complex_float,
                          lda: lapack_int, b: *mut lapack_complex_float, ldb: lapack_int,
                          q: *mut lapack_complex_float, ldq: lapack_int,
                          z: *mut lapack_complex_float, ldz: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_zgghd3(matrix_layout: c_int, compq: c_char, compz: c_char, n: lapack_int,
                          ilo: lapack_int, ihi: lapack_int, a: *mut lapack_complex_double,
                          lda: lapack_int, b: *mut lapack_complex_double, ldb: lapack_int,
                          q: *mut lapack_complex_double, ldq: lapack_int,
                          z: *mut lapack_complex_double, ldz: lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_sgglse(matrix_layout: c_int, m: lapack_int, n: lapack_int, p: lapack_int,
                          a: *mut c_float, lda: lapack_int, b: *mut c_float, ldb: lapack_int,
                          c: *mut c_float, d: *mut c_float, x: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_dgglse(matrix_layout: c_int, m: lapack_int, n: lapack_int, p: lapack_int,
                          a: *mut c_double, lda: lapack_int, b: *mut c_double, ldb: lapack_int,
                          c: *mut c_double, d: *mut c_double, x: *mut c_double)
                          -> lapack_int;
    pub fn LAPACKE_cgglse(matrix_layout: c_int, m: lapack_int, n: lapack_int, p: lapack_int,
                          a: *mut lapack_complex_float, lda: lapack_int,
                          b: *mut lapack_complex_float, ldb: lapack_int,
                          c: *mut lapack_complex_float, d: *mut lapack_complex_float,
                          x: *mut lapack_complex_float)
                          -> lapack_int;
    pub fn LAPACKE_zgglse(matrix_layout: c_int, m: lapack_int, n: lapack_int, p: lapack_int,
                          a: *mut lapack_complex_double, lda: lapack_int,
                          b: *mut lapack_complex_double, ldb: lapack_int,
                          c: *mut lapack_complex_double, d: *mut lapack_complex_double,
                          x: *mut lapack_complex_double)
                          -> lapack_int;

    pub fn LAPACKE_sggqrf(matrix_layout: c_int, n: lapack_int, m: lapack_int, p: lapack_int,
                          a: *mut c_float, lda: lapack_int, taua: *mut c_float, b: *mut c_float,
                          ldb: lapack_int, taub: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_dggqrf(matrix_layout: c_int, n: lapack_int, m: lapack_int, p: lapack_int,
                          a: *mut c_double, lda: lapack_int, taua: *mut c_double, b: *mut c_double,
                          ldb: lapack_int, taub: *mut c_double)
                          -> lapack_int;
    pub fn LAPACKE_cggqrf(matrix_layout: c_int, n: lapack_int, m: lapack_int, p: lapack_int,
                          a: *mut lapack_complex_float, lda: lapack_int,
                          taua: *mut lapack_complex_float, b: *mut lapack_complex_float,
                          ldb: lapack_int, taub: *mut lapack_complex_float)
                          -> lapack_int;
    pub fn LAPACKE_zggqrf(matrix_layout: c_int, n: lapack_int, m: lapack_int, p: lapack_int,
                          a: *mut lapack_complex_double, lda: lapack_int,
                          taua: *mut lapack_complex_double, b: *mut lapack_complex_double,
                          ldb: lapack_int, taub: *mut lapack_complex_double)
                          -> lapack_int;

    pub fn LAPACKE_sggrqf(matrix_layout: c_int, m: lapack_int, p: lapack_int, n: lapack_int,
                          a: *mut c_float, lda: lapack_int, taua: *mut c_float, b: *mut c_float,
                          ldb: lapack_int, taub: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_dggrqf(matrix_layout: c_int, m: lapack_int, p: lapack_int, n: lapack_int,
                          a: *mut c_double, lda: lapack_int, taua: *mut c_double, b: *mut c_double,
                          ldb: lapack_int, taub: *mut c_double)
                          -> lapack_int;
    pub fn LAPACKE_cggrqf(matrix_layout: c_int, m: lapack_int, p: lapack_int, n: lapack_int,
                          a: *mut lapack_complex_float, lda: lapack_int,
                          taua: *mut lapack_complex_float, b: *mut lapack_complex_float,
                          ldb: lapack_int, taub: *mut lapack_complex_float)
                          -> lapack_int;
    pub fn LAPACKE_zggrqf(matrix_layout: c_int, m: lapack_int, p: lapack_int, n: lapack_int,
                          a: *mut lapack_complex_double, lda: lapack_int,
                          taua: *mut lapack_complex_double, b: *mut lapack_complex_double,
                          ldb: lapack_int, taub: *mut lapack_complex_double)
                          -> lapack_int;

    pub fn LAPACKE_sggsvd(matrix_layout: c_int, jobu: c_char, jobv: c_char, jobq: c_char,
                          m: lapack_int, n: lapack_int, p: lapack_int, k: *mut lapack_int,
                          l: *mut lapack_int, a: *mut c_float, lda: lapack_int, b: *mut c_float,
                          ldb: lapack_int, alpha: *mut c_float, beta: *mut c_float,
                          u: *mut c_float, ldu: lapack_int, v: *mut c_float, ldv: lapack_int,
                          q: *mut c_float, ldq: lapack_int, iwork: *mut lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_dggsvd(matrix_layout: c_int, jobu: c_char, jobv: c_char, jobq: c_char,
                          m: lapack_int, n: lapack_int, p: lapack_int, k: *mut lapack_int,
                          l: *mut lapack_int, a: *mut c_double, lda: lapack_int, b: *mut c_double,
                          ldb: lapack_int, alpha: *mut c_double, beta: *mut c_double,
                          u: *mut c_double, ldu: lapack_int, v: *mut c_double, ldv: lapack_int,
                          q: *mut c_double, ldq: lapack_int, iwork: *mut lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_cggsvd(matrix_layout: c_int, jobu: c_char, jobv: c_char, jobq: c_char,
                          m: lapack_int, n: lapack_int, p: lapack_int, k: *mut lapack_int,
                          l: *mut lapack_int, a: *mut lapack_complex_float, lda: lapack_int,
                          b: *mut lapack_complex_float, ldb: lapack_int, alpha: *mut c_float,
                          beta: *mut c_float, u: *mut lapack_complex_float, ldu: lapack_int,
                          v: *mut lapack_complex_float, ldv: lapack_int,
                          q: *mut lapack_complex_float, ldq: lapack_int, iwork: *mut lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_zggsvd(matrix_layout: c_int, jobu: c_char, jobv: c_char, jobq: c_char,
                          m: lapack_int, n: lapack_int, p: lapack_int, k: *mut lapack_int,
                          l: *mut lapack_int, a: *mut lapack_complex_double, lda: lapack_int,
                          b: *mut lapack_complex_double, ldb: lapack_int, alpha: *mut c_double,
                          beta: *mut c_double, u: *mut lapack_complex_double, ldu: lapack_int,
                          v: *mut lapack_complex_double, ldv: lapack_int,
                          q: *mut lapack_complex_double, ldq: lapack_int, iwork: *mut lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_sggsvd3(matrix_layout: c_int, jobu: c_char, jobv: c_char, jobq: c_char,
                           m: lapack_int, n: lapack_int, p: lapack_int, k: *mut lapack_int,
                           l: *mut lapack_int, a: *mut c_float, lda: lapack_int, b: *mut c_float,
                           ldb: lapack_int, alpha: *mut c_float, beta: *mut c_float,
                           u: *mut c_float, ldu: lapack_int, v: *mut c_float, ldv: lapack_int,
                           q: *mut c_float, ldq: lapack_int, iwork: *mut lapack_int)
                           -> lapack_int;
    pub fn LAPACKE_dggsvd3(matrix_layout: c_int, jobu: c_char, jobv: c_char, jobq: c_char,
                           m: lapack_int, n: lapack_int, p: lapack_int, k: *mut lapack_int,
                           l: *mut lapack_int, a: *mut c_double, lda: lapack_int, b: *mut c_double,
                           ldb: lapack_int, alpha: *mut c_double, beta: *mut c_double,
                           u: *mut c_double, ldu: lapack_int, v: *mut c_double, ldv: lapack_int,
                           q: *mut c_double, ldq: lapack_int, iwork: *mut lapack_int)
                           -> lapack_int;
    pub fn LAPACKE_cggsvd3(matrix_layout: c_int, jobu: c_char, jobv: c_char, jobq: c_char,
                           m: lapack_int, n: lapack_int, p: lapack_int, k: *mut lapack_int,
                           l: *mut lapack_int, a: *mut lapack_complex_float, lda: lapack_int,
                           b: *mut lapack_complex_float, ldb: lapack_int, alpha: *mut c_float,
                           beta: *mut c_float, u: *mut lapack_complex_float, ldu: lapack_int,
                           v: *mut lapack_complex_float, ldv: lapack_int,
                           q: *mut lapack_complex_float, ldq: lapack_int, iwork: *mut lapack_int)
                           -> lapack_int;
    pub fn LAPACKE_zggsvd3(matrix_layout: c_int, jobu: c_char, jobv: c_char, jobq: c_char,
                           m: lapack_int, n: lapack_int, p: lapack_int, k: *mut lapack_int,
                           l: *mut lapack_int, a: *mut lapack_complex_double, lda: lapack_int,
                           b: *mut lapack_complex_double, ldb: lapack_int, alpha: *mut c_double,
                           beta: *mut c_double, u: *mut lapack_complex_double, ldu: lapack_int,
                           v: *mut lapack_complex_double, ldv: lapack_int,
                           q: *mut lapack_complex_double, ldq: lapack_int, iwork: *mut lapack_int)
                           -> lapack_int;

    pub fn LAPACKE_sggsvp(matrix_layout: c_int, jobu: c_char, jobv: c_char, jobq: c_char,
                          m: lapack_int, p: lapack_int, n: lapack_int, a: *mut c_float,
                          lda: lapack_int, b: *mut c_float, ldb: lapack_int, tola: c_float,
                          tolb: c_float, k: *mut lapack_int, l: *mut lapack_int, u: *mut c_float,
                          ldu: lapack_int, v: *mut c_float, ldv: lapack_int, q: *mut c_float,
                          ldq: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_dggsvp(matrix_layout: c_int, jobu: c_char, jobv: c_char, jobq: c_char,
                          m: lapack_int, p: lapack_int, n: lapack_int, a: *mut c_double,
                          lda: lapack_int, b: *mut c_double, ldb: lapack_int, tola: c_double,
                          tolb: c_double, k: *mut lapack_int, l: *mut lapack_int, u: *mut c_double,
                          ldu: lapack_int, v: *mut c_double, ldv: lapack_int, q: *mut c_double,
                          ldq: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_cggsvp(matrix_layout: c_int, jobu: c_char, jobv: c_char, jobq: c_char,
                          m: lapack_int, p: lapack_int, n: lapack_int,
                          a: *mut lapack_complex_float, lda: lapack_int,
                          b: *mut lapack_complex_float, ldb: lapack_int, tola: c_float,
                          tolb: c_float, k: *mut lapack_int, l: *mut lapack_int,
                          u: *mut lapack_complex_float, ldu: lapack_int,
                          v: *mut lapack_complex_float, ldv: lapack_int,
                          q: *mut lapack_complex_float, ldq: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_zggsvp(matrix_layout: c_int, jobu: c_char, jobv: c_char, jobq: c_char,
                          m: lapack_int, p: lapack_int, n: lapack_int,
                          a: *mut lapack_complex_double, lda: lapack_int,
                          b: *mut lapack_complex_double, ldb: lapack_int, tola: c_double,
                          tolb: c_double, k: *mut lapack_int, l: *mut lapack_int,
                          u: *mut lapack_complex_double, ldu: lapack_int,
                          v: *mut lapack_complex_double, ldv: lapack_int,
                          q: *mut lapack_complex_double, ldq: lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_sggsvp3(matrix_layout: c_int, jobu: c_char, jobv: c_char, jobq: c_char,
                           m: lapack_int, p: lapack_int, n: lapack_int, a: *mut c_float,
                           lda: lapack_int, b: *mut c_float, ldb: lapack_int, tola: c_float,
                           tolb: c_float, k: *mut lapack_int, l: *mut lapack_int, u: *mut c_float,
                           ldu: lapack_int, v: *mut c_float, ldv: lapack_int, q: *mut c_float,
                           ldq: lapack_int)
                           -> lapack_int;
    pub fn LAPACKE_dggsvp3(matrix_layout: c_int, jobu: c_char, jobv: c_char, jobq: c_char,
                           m: lapack_int, p: lapack_int, n: lapack_int, a: *mut c_double,
                           lda: lapack_int, b: *mut c_double, ldb: lapack_int, tola: c_double,
                           tolb: c_double, k: *mut lapack_int, l: *mut lapack_int,
                           u: *mut c_double, ldu: lapack_int, v: *mut c_double, ldv: lapack_int,
                           q: *mut c_double, ldq: lapack_int)
                           -> lapack_int;
    pub fn LAPACKE_cggsvp3(matrix_layout: c_int, jobu: c_char, jobv: c_char, jobq: c_char,
                           m: lapack_int, p: lapack_int, n: lapack_int,
                           a: *mut lapack_complex_float, lda: lapack_int,
                           b: *mut lapack_complex_float, ldb: lapack_int, tola: c_float,
                           tolb: c_float, k: *mut lapack_int, l: *mut lapack_int,
                           u: *mut lapack_complex_float, ldu: lapack_int,
                           v: *mut lapack_complex_float, ldv: lapack_int,
                           q: *mut lapack_complex_float, ldq: lapack_int)
                           -> lapack_int;
    pub fn LAPACKE_zggsvp3(matrix_layout: c_int, jobu: c_char, jobv: c_char, jobq: c_char,
                           m: lapack_int, p: lapack_int, n: lapack_int,
                           a: *mut lapack_complex_double, lda: lapack_int,
                           b: *mut lapack_complex_double, ldb: lapack_int, tola: c_double,
                           tolb: c_double, k: *mut lapack_int, l: *mut lapack_int,
                           u: *mut lapack_complex_double, ldu: lapack_int,
                           v: *mut lapack_complex_double, ldv: lapack_int,
                           q: *mut lapack_complex_double, ldq: lapack_int)
                           -> lapack_int;

    pub fn LAPACKE_sgtcon(norm: c_char, n: lapack_int, dl: *const c_float, d: *const c_float,
                          du: *const c_float, du2: *const c_float, ipiv: *const lapack_int,
                          anorm: c_float, rcond: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_dgtcon(norm: c_char, n: lapack_int, dl: *const c_double, d: *const c_double,
                          du: *const c_double, du2: *const c_double, ipiv: *const lapack_int,
                          anorm: c_double, rcond: *mut c_double)
                          -> lapack_int;
    pub fn LAPACKE_cgtcon(norm: c_char, n: lapack_int, dl: *const lapack_complex_float,
                          d: *const lapack_complex_float, du: *const lapack_complex_float,
                          du2: *const lapack_complex_float, ipiv: *const lapack_int,
                          anorm: c_float, rcond: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_zgtcon(norm: c_char, n: lapack_int, dl: *const lapack_complex_double,
                          d: *const lapack_complex_double, du: *const lapack_complex_double,
                          du2: *const lapack_complex_double, ipiv: *const lapack_int,
                          anorm: c_double, rcond: *mut c_double)
                          -> lapack_int;

    pub fn LAPACKE_sgtrfs(matrix_layout: c_int, trans: c_char, n: lapack_int, nrhs: lapack_int,
                          dl: *const c_float, d: *const c_float, du: *const c_float,
                          dlf: *const c_float, df: *const c_float, duf: *const c_float,
                          du2: *const c_float, ipiv: *const lapack_int, b: *const c_float,
                          ldb: lapack_int, x: *mut c_float, ldx: lapack_int, ferr: *mut c_float,
                          berr: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_dgtrfs(matrix_layout: c_int, trans: c_char, n: lapack_int, nrhs: lapack_int,
                          dl: *const c_double, d: *const c_double, du: *const c_double,
                          dlf: *const c_double, df: *const c_double, duf: *const c_double,
                          du2: *const c_double, ipiv: *const lapack_int, b: *const c_double,
                          ldb: lapack_int, x: *mut c_double, ldx: lapack_int, ferr: *mut c_double,
                          berr: *mut c_double)
                          -> lapack_int;
    pub fn LAPACKE_cgtrfs(matrix_layout: c_int, trans: c_char, n: lapack_int, nrhs: lapack_int,
                          dl: *const lapack_complex_float, d: *const lapack_complex_float,
                          du: *const lapack_complex_float, dlf: *const lapack_complex_float,
                          df: *const lapack_complex_float, duf: *const lapack_complex_float,
                          du2: *const lapack_complex_float, ipiv: *const lapack_int,
                          b: *const lapack_complex_float, ldb: lapack_int,
                          x: *mut lapack_complex_float, ldx: lapack_int, ferr: *mut c_float,
                          berr: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_zgtrfs(matrix_layout: c_int, trans: c_char, n: lapack_int, nrhs: lapack_int,
                          dl: *const lapack_complex_double, d: *const lapack_complex_double,
                          du: *const lapack_complex_double, dlf: *const lapack_complex_double,
                          df: *const lapack_complex_double, duf: *const lapack_complex_double,
                          du2: *const lapack_complex_double, ipiv: *const lapack_int,
                          b: *const lapack_complex_double, ldb: lapack_int,
                          x: *mut lapack_complex_double, ldx: lapack_int, ferr: *mut c_double,
                          berr: *mut c_double)
                          -> lapack_int;

    pub fn LAPACKE_sgtsv(matrix_layout: c_int, n: lapack_int, nrhs: lapack_int, dl: *mut c_float,
                         d: *mut c_float, du: *mut c_float, b: *mut c_float, ldb: lapack_int)
                         -> lapack_int;
    pub fn LAPACKE_dgtsv(matrix_layout: c_int, n: lapack_int, nrhs: lapack_int, dl: *mut c_double,
                         d: *mut c_double, du: *mut c_double, b: *mut c_double, ldb: lapack_int)
                         -> lapack_int;
    pub fn LAPACKE_cgtsv(matrix_layout: c_int, n: lapack_int, nrhs: lapack_int,
                         dl: *mut lapack_complex_float, d: *mut lapack_complex_float,
                         du: *mut lapack_complex_float, b: *mut lapack_complex_float,
                         ldb: lapack_int)
                         -> lapack_int;
    pub fn LAPACKE_zgtsv(matrix_layout: c_int, n: lapack_int, nrhs: lapack_int,
                         dl: *mut lapack_complex_double, d: *mut lapack_complex_double,
                         du: *mut lapack_complex_double, b: *mut lapack_complex_double,
                         ldb: lapack_int)
                         -> lapack_int;

    pub fn LAPACKE_sgtsvx(matrix_layout: c_int, fact: c_char, trans: c_char, n: lapack_int,
                          nrhs: lapack_int, dl: *const c_float, d: *const c_float,
                          du: *const c_float, dlf: *mut c_float, df: *mut c_float,
                          duf: *mut c_float, du2: *mut c_float, ipiv: *mut lapack_int,
                          b: *const c_float, ldb: lapack_int, x: *mut c_float, ldx: lapack_int,
                          rcond: *mut c_float, ferr: *mut c_float, berr: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_dgtsvx(matrix_layout: c_int, fact: c_char, trans: c_char, n: lapack_int,
                          nrhs: lapack_int, dl: *const c_double, d: *const c_double,
                          du: *const c_double, dlf: *mut c_double, df: *mut c_double,
                          duf: *mut c_double, du2: *mut c_double, ipiv: *mut lapack_int,
                          b: *const c_double, ldb: lapack_int, x: *mut c_double, ldx: lapack_int,
                          rcond: *mut c_double, ferr: *mut c_double, berr: *mut c_double)
                          -> lapack_int;
    pub fn LAPACKE_cgtsvx(matrix_layout: c_int, fact: c_char, trans: c_char, n: lapack_int,
                          nrhs: lapack_int, dl: *const lapack_complex_float,
                          d: *const lapack_complex_float, du: *const lapack_complex_float,
                          dlf: *mut lapack_complex_float, df: *mut lapack_complex_float,
                          duf: *mut lapack_complex_float, du2: *mut lapack_complex_float,
                          ipiv: *mut lapack_int, b: *const lapack_complex_float, ldb: lapack_int,
                          x: *mut lapack_complex_float, ldx: lapack_int, rcond: *mut c_float,
                          ferr: *mut c_float, berr: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_zgtsvx(matrix_layout: c_int, fact: c_char, trans: c_char, n: lapack_int,
                          nrhs: lapack_int, dl: *const lapack_complex_double,
                          d: *const lapack_complex_double, du: *const lapack_complex_double,
                          dlf: *mut lapack_complex_double, df: *mut lapack_complex_double,
                          duf: *mut lapack_complex_double, du2: *mut lapack_complex_double,
                          ipiv: *mut lapack_int, b: *const lapack_complex_double, ldb: lapack_int,
                          x: *mut lapack_complex_double, ldx: lapack_int, rcond: *mut c_double,
                          ferr: *mut c_double, berr: *mut c_double)
                          -> lapack_int;

    pub fn LAPACKE_sgttrf(n: lapack_int, dl: *mut c_float, d: *mut c_float, du: *mut c_float,
                          du2: *mut c_float, ipiv: *mut lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_dgttrf(n: lapack_int, dl: *mut c_double, d: *mut c_double, du: *mut c_double,
                          du2: *mut c_double, ipiv: *mut lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_cgttrf(n: lapack_int, dl: *mut lapack_complex_float,
                          d: *mut lapack_complex_float, du: *mut lapack_complex_float,
                          du2: *mut lapack_complex_float, ipiv: *mut lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_zgttrf(n: lapack_int, dl: *mut lapack_complex_double,
                          d: *mut lapack_complex_double, du: *mut lapack_complex_double,
                          du2: *mut lapack_complex_double, ipiv: *mut lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_sgttrs(matrix_layout: c_int, trans: c_char, n: lapack_int, nrhs: lapack_int,
                          dl: *const c_float, d: *const c_float, du: *const c_float,
                          du2: *const c_float, ipiv: *const lapack_int, b: *mut c_float,
                          ldb: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_dgttrs(matrix_layout: c_int, trans: c_char, n: lapack_int, nrhs: lapack_int,
                          dl: *const c_double, d: *const c_double, du: *const c_double,
                          du2: *const c_double, ipiv: *const lapack_int, b: *mut c_double,
                          ldb: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_cgttrs(matrix_layout: c_int, trans: c_char, n: lapack_int, nrhs: lapack_int,
                          dl: *const lapack_complex_float, d: *const lapack_complex_float,
                          du: *const lapack_complex_float, du2: *const lapack_complex_float,
                          ipiv: *const lapack_int, b: *mut lapack_complex_float, ldb: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_zgttrs(matrix_layout: c_int, trans: c_char, n: lapack_int, nrhs: lapack_int,
                          dl: *const lapack_complex_double, d: *const lapack_complex_double,
                          du: *const lapack_complex_double, du2: *const lapack_complex_double,
                          ipiv: *const lapack_int, b: *mut lapack_complex_double, ldb: lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_chbev(matrix_layout: c_int, jobz: c_char, uplo: c_char, n: lapack_int,
                         kd: lapack_int, ab: *mut lapack_complex_float, ldab: lapack_int,
                         w: *mut c_float, z: *mut lapack_complex_float, ldz: lapack_int)
                         -> lapack_int;
    pub fn LAPACKE_zhbev(matrix_layout: c_int, jobz: c_char, uplo: c_char, n: lapack_int,
                         kd: lapack_int, ab: *mut lapack_complex_double, ldab: lapack_int,
                         w: *mut c_double, z: *mut lapack_complex_double, ldz: lapack_int)
                         -> lapack_int;

    pub fn LAPACKE_chbevd(matrix_layout: c_int, jobz: c_char, uplo: c_char, n: lapack_int,
                          kd: lapack_int, ab: *mut lapack_complex_float, ldab: lapack_int,
                          w: *mut c_float, z: *mut lapack_complex_float, ldz: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_zhbevd(matrix_layout: c_int, jobz: c_char, uplo: c_char, n: lapack_int,
                          kd: lapack_int, ab: *mut lapack_complex_double, ldab: lapack_int,
                          w: *mut c_double, z: *mut lapack_complex_double, ldz: lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_chbevx(matrix_layout: c_int, jobz: c_char, range: c_char, uplo: c_char,
                          n: lapack_int, kd: lapack_int, ab: *mut lapack_complex_float,
                          ldab: lapack_int, q: *mut lapack_complex_float, ldq: lapack_int,
                          vl: c_float, vu: c_float, il: lapack_int, iu: lapack_int,
                          abstol: c_float, m: *mut lapack_int, w: *mut c_float,
                          z: *mut lapack_complex_float, ldz: lapack_int, ifail: *mut lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_zhbevx(matrix_layout: c_int, jobz: c_char, range: c_char, uplo: c_char,
                          n: lapack_int, kd: lapack_int, ab: *mut lapack_complex_double,
                          ldab: lapack_int, q: *mut lapack_complex_double, ldq: lapack_int,
                          vl: c_double, vu: c_double, il: lapack_int, iu: lapack_int,
                          abstol: c_double, m: *mut lapack_int, w: *mut c_double,
                          z: *mut lapack_complex_double, ldz: lapack_int, ifail: *mut lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_chbgst(matrix_layout: c_int, vect: c_char, uplo: c_char, n: lapack_int,
                          ka: lapack_int, kb: lapack_int, ab: *mut lapack_complex_float,
                          ldab: lapack_int, bb: *const lapack_complex_float, ldbb: lapack_int,
                          x: *mut lapack_complex_float, ldx: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_zhbgst(matrix_layout: c_int, vect: c_char, uplo: c_char, n: lapack_int,
                          ka: lapack_int, kb: lapack_int, ab: *mut lapack_complex_double,
                          ldab: lapack_int, bb: *const lapack_complex_double, ldbb: lapack_int,
                          x: *mut lapack_complex_double, ldx: lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_chbgv(matrix_layout: c_int, jobz: c_char, uplo: c_char, n: lapack_int,
                         ka: lapack_int, kb: lapack_int, ab: *mut lapack_complex_float,
                         ldab: lapack_int, bb: *mut lapack_complex_float, ldbb: lapack_int,
                         w: *mut c_float, z: *mut lapack_complex_float, ldz: lapack_int)
                         -> lapack_int;
    pub fn LAPACKE_zhbgv(matrix_layout: c_int, jobz: c_char, uplo: c_char, n: lapack_int,
                         ka: lapack_int, kb: lapack_int, ab: *mut lapack_complex_double,
                         ldab: lapack_int, bb: *mut lapack_complex_double, ldbb: lapack_int,
                         w: *mut c_double, z: *mut lapack_complex_double, ldz: lapack_int)
                         -> lapack_int;

    pub fn LAPACKE_chbgvd(matrix_layout: c_int, jobz: c_char, uplo: c_char, n: lapack_int,
                          ka: lapack_int, kb: lapack_int, ab: *mut lapack_complex_float,
                          ldab: lapack_int, bb: *mut lapack_complex_float, ldbb: lapack_int,
                          w: *mut c_float, z: *mut lapack_complex_float, ldz: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_zhbgvd(matrix_layout: c_int, jobz: c_char, uplo: c_char, n: lapack_int,
                          ka: lapack_int, kb: lapack_int, ab: *mut lapack_complex_double,
                          ldab: lapack_int, bb: *mut lapack_complex_double, ldbb: lapack_int,
                          w: *mut c_double, z: *mut lapack_complex_double, ldz: lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_chbgvx(matrix_layout: c_int, jobz: c_char, range: c_char, uplo: c_char,
                          n: lapack_int, ka: lapack_int, kb: lapack_int,
                          ab: *mut lapack_complex_float, ldab: lapack_int,
                          bb: *mut lapack_complex_float, ldbb: lapack_int,
                          q: *mut lapack_complex_float, ldq: lapack_int, vl: c_float, vu: c_float,
                          il: lapack_int, iu: lapack_int, abstol: c_float, m: *mut lapack_int,
                          w: *mut c_float, z: *mut lapack_complex_float, ldz: lapack_int,
                          ifail: *mut lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_zhbgvx(matrix_layout: c_int, jobz: c_char, range: c_char, uplo: c_char,
                          n: lapack_int, ka: lapack_int, kb: lapack_int,
                          ab: *mut lapack_complex_double, ldab: lapack_int,
                          bb: *mut lapack_complex_double, ldbb: lapack_int,
                          q: *mut lapack_complex_double, ldq: lapack_int, vl: c_double,
                          vu: c_double, il: lapack_int, iu: lapack_int, abstol: c_double,
                          m: *mut lapack_int, w: *mut c_double, z: *mut lapack_complex_double,
                          ldz: lapack_int, ifail: *mut lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_chbtrd(matrix_layout: c_int, vect: c_char, uplo: c_char, n: lapack_int,
                          kd: lapack_int, ab: *mut lapack_complex_float, ldab: lapack_int,
                          d: *mut c_float, e: *mut c_float, q: *mut lapack_complex_float,
                          ldq: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_zhbtrd(matrix_layout: c_int, vect: c_char, uplo: c_char, n: lapack_int,
                          kd: lapack_int, ab: *mut lapack_complex_double, ldab: lapack_int,
                          d: *mut c_double, e: *mut c_double, q: *mut lapack_complex_double,
                          ldq: lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_checon(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                          a: *const lapack_complex_float, lda: lapack_int, ipiv: *const lapack_int,
                          anorm: c_float, rcond: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_zhecon(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                          a: *const lapack_complex_double, lda: lapack_int,
                          ipiv: *const lapack_int, anorm: c_double, rcond: *mut c_double)
                          -> lapack_int;

    pub fn LAPACKE_cheequb(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                           a: *const lapack_complex_float, lda: lapack_int, s: *mut c_float,
                           scond: *mut c_float, amax: *mut c_float)
                           -> lapack_int;
    pub fn LAPACKE_zheequb(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                           a: *const lapack_complex_double, lda: lapack_int, s: *mut c_double,
                           scond: *mut c_double, amax: *mut c_double)
                           -> lapack_int;

    pub fn LAPACKE_cheev(matrix_layout: c_int, jobz: c_char, uplo: c_char, n: lapack_int,
                         a: *mut lapack_complex_float, lda: lapack_int, w: *mut c_float)
                         -> lapack_int;
    pub fn LAPACKE_zheev(matrix_layout: c_int, jobz: c_char, uplo: c_char, n: lapack_int,
                         a: *mut lapack_complex_double, lda: lapack_int, w: *mut c_double)
                         -> lapack_int;

    pub fn LAPACKE_cheevd(matrix_layout: c_int, jobz: c_char, uplo: c_char, n: lapack_int,
                          a: *mut lapack_complex_float, lda: lapack_int, w: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_zheevd(matrix_layout: c_int, jobz: c_char, uplo: c_char, n: lapack_int,
                          a: *mut lapack_complex_double, lda: lapack_int, w: *mut c_double)
                          -> lapack_int;

    pub fn LAPACKE_cheevr(matrix_layout: c_int, jobz: c_char, range: c_char, uplo: c_char,
                          n: lapack_int, a: *mut lapack_complex_float, lda: lapack_int,
                          vl: c_float, vu: c_float, il: lapack_int, iu: lapack_int,
                          abstol: c_float, m: *mut lapack_int, w: *mut c_float,
                          z: *mut lapack_complex_float, ldz: lapack_int, isuppz: *mut lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_zheevr(matrix_layout: c_int, jobz: c_char, range: c_char, uplo: c_char,
                          n: lapack_int, a: *mut lapack_complex_double, lda: lapack_int,
                          vl: c_double, vu: c_double, il: lapack_int, iu: lapack_int,
                          abstol: c_double, m: *mut lapack_int, w: *mut c_double,
                          z: *mut lapack_complex_double, ldz: lapack_int, isuppz: *mut lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_cheevx(matrix_layout: c_int, jobz: c_char, range: c_char, uplo: c_char,
                          n: lapack_int, a: *mut lapack_complex_float, lda: lapack_int,
                          vl: c_float, vu: c_float, il: lapack_int, iu: lapack_int,
                          abstol: c_float, m: *mut lapack_int, w: *mut c_float,
                          z: *mut lapack_complex_float, ldz: lapack_int, ifail: *mut lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_zheevx(matrix_layout: c_int, jobz: c_char, range: c_char, uplo: c_char,
                          n: lapack_int, a: *mut lapack_complex_double, lda: lapack_int,
                          vl: c_double, vu: c_double, il: lapack_int, iu: lapack_int,
                          abstol: c_double, m: *mut lapack_int, w: *mut c_double,
                          z: *mut lapack_complex_double, ldz: lapack_int, ifail: *mut lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_chegst(matrix_layout: c_int, itype: lapack_int, uplo: c_char, n: lapack_int,
                          a: *mut lapack_complex_float, lda: lapack_int,
                          b: *const lapack_complex_float, ldb: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_zhegst(matrix_layout: c_int, itype: lapack_int, uplo: c_char, n: lapack_int,
                          a: *mut lapack_complex_double, lda: lapack_int,
                          b: *const lapack_complex_double, ldb: lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_chegv(matrix_layout: c_int, itype: lapack_int, jobz: c_char, uplo: c_char,
                         n: lapack_int, a: *mut lapack_complex_float, lda: lapack_int,
                         b: *mut lapack_complex_float, ldb: lapack_int, w: *mut c_float)
                         -> lapack_int;
    pub fn LAPACKE_zhegv(matrix_layout: c_int, itype: lapack_int, jobz: c_char, uplo: c_char,
                         n: lapack_int, a: *mut lapack_complex_double, lda: lapack_int,
                         b: *mut lapack_complex_double, ldb: lapack_int, w: *mut c_double)
                         -> lapack_int;

    pub fn LAPACKE_chegvd(matrix_layout: c_int, itype: lapack_int, jobz: c_char, uplo: c_char,
                          n: lapack_int, a: *mut lapack_complex_float, lda: lapack_int,
                          b: *mut lapack_complex_float, ldb: lapack_int, w: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_zhegvd(matrix_layout: c_int, itype: lapack_int, jobz: c_char, uplo: c_char,
                          n: lapack_int, a: *mut lapack_complex_double, lda: lapack_int,
                          b: *mut lapack_complex_double, ldb: lapack_int, w: *mut c_double)
                          -> lapack_int;

    pub fn LAPACKE_chegvx(matrix_layout: c_int, itype: lapack_int, jobz: c_char, range: c_char,
                          uplo: c_char, n: lapack_int, a: *mut lapack_complex_float,
                          lda: lapack_int, b: *mut lapack_complex_float, ldb: lapack_int,
                          vl: c_float, vu: c_float, il: lapack_int, iu: lapack_int,
                          abstol: c_float, m: *mut lapack_int, w: *mut c_float,
                          z: *mut lapack_complex_float, ldz: lapack_int, ifail: *mut lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_zhegvx(matrix_layout: c_int, itype: lapack_int, jobz: c_char, range: c_char,
                          uplo: c_char, n: lapack_int, a: *mut lapack_complex_double,
                          lda: lapack_int, b: *mut lapack_complex_double, ldb: lapack_int,
                          vl: c_double, vu: c_double, il: lapack_int, iu: lapack_int,
                          abstol: c_double, m: *mut lapack_int, w: *mut c_double,
                          z: *mut lapack_complex_double, ldz: lapack_int, ifail: *mut lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_cherfs(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                          a: *const lapack_complex_float, lda: lapack_int,
                          af: *const lapack_complex_float, ldaf: lapack_int,
                          ipiv: *const lapack_int, b: *const lapack_complex_float, ldb: lapack_int,
                          x: *mut lapack_complex_float, ldx: lapack_int, ferr: *mut c_float,
                          berr: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_zherfs(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                          a: *const lapack_complex_double, lda: lapack_int,
                          af: *const lapack_complex_double, ldaf: lapack_int,
                          ipiv: *const lapack_int, b: *const lapack_complex_double,
                          ldb: lapack_int, x: *mut lapack_complex_double, ldx: lapack_int,
                          ferr: *mut c_double, berr: *mut c_double)
                          -> lapack_int;

    pub fn LAPACKE_cherfsx(matrix_layout: c_int, uplo: c_char, equed: c_char, n: lapack_int,
                           nrhs: lapack_int, a: *const lapack_complex_float, lda: lapack_int,
                           af: *const lapack_complex_float, ldaf: lapack_int,
                           ipiv: *const lapack_int, s: *const c_float,
                           b: *const lapack_complex_float, ldb: lapack_int,
                           x: *mut lapack_complex_float, ldx: lapack_int, rcond: *mut c_float,
                           berr: *mut c_float, n_err_bnds: lapack_int, err_bnds_norm: *mut c_float,
                           err_bnds_comp: *mut c_float, nparams: lapack_int, params: *mut c_float)
                           -> lapack_int;
    pub fn LAPACKE_zherfsx(matrix_layout: c_int, uplo: c_char, equed: c_char, n: lapack_int,
                           nrhs: lapack_int, a: *const lapack_complex_double, lda: lapack_int,
                           af: *const lapack_complex_double, ldaf: lapack_int,
                           ipiv: *const lapack_int, s: *const c_double,
                           b: *const lapack_complex_double, ldb: lapack_int,
                           x: *mut lapack_complex_double, ldx: lapack_int, rcond: *mut c_double,
                           berr: *mut c_double, n_err_bnds: lapack_int,
                           err_bnds_norm: *mut c_double, err_bnds_comp: *mut c_double,
                           nparams: lapack_int, params: *mut c_double)
                           -> lapack_int;

    pub fn LAPACKE_chesv(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                         a: *mut lapack_complex_float, lda: lapack_int, ipiv: *mut lapack_int,
                         b: *mut lapack_complex_float, ldb: lapack_int)
                         -> lapack_int;
    pub fn LAPACKE_zhesv(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                         a: *mut lapack_complex_double, lda: lapack_int, ipiv: *mut lapack_int,
                         b: *mut lapack_complex_double, ldb: lapack_int)
                         -> lapack_int;

    pub fn LAPACKE_chesvx(matrix_layout: c_int, fact: c_char, uplo: c_char, n: lapack_int,
                          nrhs: lapack_int, a: *const lapack_complex_float, lda: lapack_int,
                          af: *mut lapack_complex_float, ldaf: lapack_int, ipiv: *mut lapack_int,
                          b: *const lapack_complex_float, ldb: lapack_int,
                          x: *mut lapack_complex_float, ldx: lapack_int, rcond: *mut c_float,
                          ferr: *mut c_float, berr: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_zhesvx(matrix_layout: c_int, fact: c_char, uplo: c_char, n: lapack_int,
                          nrhs: lapack_int, a: *const lapack_complex_double, lda: lapack_int,
                          af: *mut lapack_complex_double, ldaf: lapack_int, ipiv: *mut lapack_int,
                          b: *const lapack_complex_double, ldb: lapack_int,
                          x: *mut lapack_complex_double, ldx: lapack_int, rcond: *mut c_double,
                          ferr: *mut c_double, berr: *mut c_double)
                          -> lapack_int;

    pub fn LAPACKE_chesvxx(matrix_layout: c_int, fact: c_char, uplo: c_char, n: lapack_int,
                           nrhs: lapack_int, a: *mut lapack_complex_float, lda: lapack_int,
                           af: *mut lapack_complex_float, ldaf: lapack_int, ipiv: *mut lapack_int,
                           equed: *mut c_char, s: *mut c_float, b: *mut lapack_complex_float,
                           ldb: lapack_int, x: *mut lapack_complex_float, ldx: lapack_int,
                           rcond: *mut c_float, rpvgrw: *mut c_float, berr: *mut c_float,
                           n_err_bnds: lapack_int, err_bnds_norm: *mut c_float,
                           err_bnds_comp: *mut c_float, nparams: lapack_int, params: *mut c_float)
                           -> lapack_int;
    pub fn LAPACKE_zhesvxx(matrix_layout: c_int, fact: c_char, uplo: c_char, n: lapack_int,
                           nrhs: lapack_int, a: *mut lapack_complex_double, lda: lapack_int,
                           af: *mut lapack_complex_double, ldaf: lapack_int, ipiv: *mut lapack_int,
                           equed: *mut c_char, s: *mut c_double, b: *mut lapack_complex_double,
                           ldb: lapack_int, x: *mut lapack_complex_double, ldx: lapack_int,
                           rcond: *mut c_double, rpvgrw: *mut c_double, berr: *mut c_double,
                           n_err_bnds: lapack_int, err_bnds_norm: *mut c_double,
                           err_bnds_comp: *mut c_double, nparams: lapack_int,
                           params: *mut c_double)
                           -> lapack_int;

    pub fn LAPACKE_chetrd(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                          a: *mut lapack_complex_float, lda: lapack_int, d: *mut c_float,
                          e: *mut c_float, tau: *mut lapack_complex_float)
                          -> lapack_int;
    pub fn LAPACKE_zhetrd(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                          a: *mut lapack_complex_double, lda: lapack_int, d: *mut c_double,
                          e: *mut c_double, tau: *mut lapack_complex_double)
                          -> lapack_int;

    pub fn LAPACKE_chetrf(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                          a: *mut lapack_complex_float, lda: lapack_int, ipiv: *mut lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_zhetrf(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                          a: *mut lapack_complex_double, lda: lapack_int, ipiv: *mut lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_chetri(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                          a: *mut lapack_complex_float, lda: lapack_int, ipiv: *const lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_zhetri(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                          a: *mut lapack_complex_double, lda: lapack_int, ipiv: *const lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_chetrs(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                          a: *const lapack_complex_float, lda: lapack_int, ipiv: *const lapack_int,
                          b: *mut lapack_complex_float, ldb: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_zhetrs(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                          a: *const lapack_complex_double, lda: lapack_int,
                          ipiv: *const lapack_int, b: *mut lapack_complex_double, ldb: lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_chfrk(matrix_layout: c_int, transr: c_char, uplo: c_char, trans: c_char,
                         n: lapack_int, k: lapack_int, alpha: c_float,
                         a: *const lapack_complex_float, lda: lapack_int, beta: c_float,
                         c: *mut lapack_complex_float)
                         -> lapack_int;
    pub fn LAPACKE_zhfrk(matrix_layout: c_int, transr: c_char, uplo: c_char, trans: c_char,
                         n: lapack_int, k: lapack_int, alpha: c_double,
                         a: *const lapack_complex_double, lda: lapack_int, beta: c_double,
                         c: *mut lapack_complex_double)
                         -> lapack_int;

    pub fn LAPACKE_shgeqz(matrix_layout: c_int, job: c_char, compq: c_char, compz: c_char,
                          n: lapack_int, ilo: lapack_int, ihi: lapack_int, h: *mut c_float,
                          ldh: lapack_int, t: *mut c_float, ldt: lapack_int, alphar: *mut c_float,
                          alphai: *mut c_float, beta: *mut c_float, q: *mut c_float,
                          ldq: lapack_int, z: *mut c_float, ldz: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_dhgeqz(matrix_layout: c_int, job: c_char, compq: c_char, compz: c_char,
                          n: lapack_int, ilo: lapack_int, ihi: lapack_int, h: *mut c_double,
                          ldh: lapack_int, t: *mut c_double, ldt: lapack_int,
                          alphar: *mut c_double, alphai: *mut c_double, beta: *mut c_double,
                          q: *mut c_double, ldq: lapack_int, z: *mut c_double, ldz: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_chgeqz(matrix_layout: c_int, job: c_char, compq: c_char, compz: c_char,
                          n: lapack_int, ilo: lapack_int, ihi: lapack_int,
                          h: *mut lapack_complex_float, ldh: lapack_int,
                          t: *mut lapack_complex_float, ldt: lapack_int,
                          alpha: *mut lapack_complex_float, beta: *mut lapack_complex_float,
                          q: *mut lapack_complex_float, ldq: lapack_int,
                          z: *mut lapack_complex_float, ldz: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_zhgeqz(matrix_layout: c_int, job: c_char, compq: c_char, compz: c_char,
                          n: lapack_int, ilo: lapack_int, ihi: lapack_int,
                          h: *mut lapack_complex_double, ldh: lapack_int,
                          t: *mut lapack_complex_double, ldt: lapack_int,
                          alpha: *mut lapack_complex_double, beta: *mut lapack_complex_double,
                          q: *mut lapack_complex_double, ldq: lapack_int,
                          z: *mut lapack_complex_double, ldz: lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_chpcon(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                          ap: *const lapack_complex_float, ipiv: *const lapack_int, anorm: c_float,
                          rcond: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_zhpcon(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                          ap: *const lapack_complex_double, ipiv: *const lapack_int,
                          anorm: c_double, rcond: *mut c_double)
                          -> lapack_int;

    pub fn LAPACKE_chpev(matrix_layout: c_int, jobz: c_char, uplo: c_char, n: lapack_int,
                         ap: *mut lapack_complex_float, w: *mut c_float,
                         z: *mut lapack_complex_float, ldz: lapack_int)
                         -> lapack_int;
    pub fn LAPACKE_zhpev(matrix_layout: c_int, jobz: c_char, uplo: c_char, n: lapack_int,
                         ap: *mut lapack_complex_double, w: *mut c_double,
                         z: *mut lapack_complex_double, ldz: lapack_int)
                         -> lapack_int;

    pub fn LAPACKE_chpevd(matrix_layout: c_int, jobz: c_char, uplo: c_char, n: lapack_int,
                          ap: *mut lapack_complex_float, w: *mut c_float,
                          z: *mut lapack_complex_float, ldz: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_zhpevd(matrix_layout: c_int, jobz: c_char, uplo: c_char, n: lapack_int,
                          ap: *mut lapack_complex_double, w: *mut c_double,
                          z: *mut lapack_complex_double, ldz: lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_chpevx(matrix_layout: c_int, jobz: c_char, range: c_char, uplo: c_char,
                          n: lapack_int, ap: *mut lapack_complex_float, vl: c_float, vu: c_float,
                          il: lapack_int, iu: lapack_int, abstol: c_float, m: *mut lapack_int,
                          w: *mut c_float, z: *mut lapack_complex_float, ldz: lapack_int,
                          ifail: *mut lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_zhpevx(matrix_layout: c_int, jobz: c_char, range: c_char, uplo: c_char,
                          n: lapack_int, ap: *mut lapack_complex_double, vl: c_double,
                          vu: c_double, il: lapack_int, iu: lapack_int, abstol: c_double,
                          m: *mut lapack_int, w: *mut c_double, z: *mut lapack_complex_double,
                          ldz: lapack_int, ifail: *mut lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_chpgst(matrix_layout: c_int, itype: lapack_int, uplo: c_char, n: lapack_int,
                          ap: *mut lapack_complex_float, bp: *const lapack_complex_float)
                          -> lapack_int;
    pub fn LAPACKE_zhpgst(matrix_layout: c_int, itype: lapack_int, uplo: c_char, n: lapack_int,
                          ap: *mut lapack_complex_double, bp: *const lapack_complex_double)
                          -> lapack_int;

    pub fn LAPACKE_chpgv(matrix_layout: c_int, itype: lapack_int, jobz: c_char, uplo: c_char,
                         n: lapack_int, ap: *mut lapack_complex_float,
                         bp: *mut lapack_complex_float, w: *mut c_float,
                         z: *mut lapack_complex_float, ldz: lapack_int)
                         -> lapack_int;
    pub fn LAPACKE_zhpgv(matrix_layout: c_int, itype: lapack_int, jobz: c_char, uplo: c_char,
                         n: lapack_int, ap: *mut lapack_complex_double,
                         bp: *mut lapack_complex_double, w: *mut c_double,
                         z: *mut lapack_complex_double, ldz: lapack_int)
                         -> lapack_int;

    pub fn LAPACKE_chpgvd(matrix_layout: c_int, itype: lapack_int, jobz: c_char, uplo: c_char,
                          n: lapack_int, ap: *mut lapack_complex_float,
                          bp: *mut lapack_complex_float, w: *mut c_float,
                          z: *mut lapack_complex_float, ldz: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_zhpgvd(matrix_layout: c_int, itype: lapack_int, jobz: c_char, uplo: c_char,
                          n: lapack_int, ap: *mut lapack_complex_double,
                          bp: *mut lapack_complex_double, w: *mut c_double,
                          z: *mut lapack_complex_double, ldz: lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_chpgvx(matrix_layout: c_int, itype: lapack_int, jobz: c_char, range: c_char,
                          uplo: c_char, n: lapack_int, ap: *mut lapack_complex_float,
                          bp: *mut lapack_complex_float, vl: c_float, vu: c_float, il: lapack_int,
                          iu: lapack_int, abstol: c_float, m: *mut lapack_int, w: *mut c_float,
                          z: *mut lapack_complex_float, ldz: lapack_int, ifail: *mut lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_zhpgvx(matrix_layout: c_int, itype: lapack_int, jobz: c_char, range: c_char,
                          uplo: c_char, n: lapack_int, ap: *mut lapack_complex_double,
                          bp: *mut lapack_complex_double, vl: c_double, vu: c_double,
                          il: lapack_int, iu: lapack_int, abstol: c_double, m: *mut lapack_int,
                          w: *mut c_double, z: *mut lapack_complex_double, ldz: lapack_int,
                          ifail: *mut lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_chprfs(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                          ap: *const lapack_complex_float, afp: *const lapack_complex_float,
                          ipiv: *const lapack_int, b: *const lapack_complex_float, ldb: lapack_int,
                          x: *mut lapack_complex_float, ldx: lapack_int, ferr: *mut c_float,
                          berr: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_zhprfs(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                          ap: *const lapack_complex_double, afp: *const lapack_complex_double,
                          ipiv: *const lapack_int, b: *const lapack_complex_double,
                          ldb: lapack_int, x: *mut lapack_complex_double, ldx: lapack_int,
                          ferr: *mut c_double, berr: *mut c_double)
                          -> lapack_int;

    pub fn LAPACKE_chpsv(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                         ap: *mut lapack_complex_float, ipiv: *mut lapack_int,
                         b: *mut lapack_complex_float, ldb: lapack_int)
                         -> lapack_int;
    pub fn LAPACKE_zhpsv(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                         ap: *mut lapack_complex_double, ipiv: *mut lapack_int,
                         b: *mut lapack_complex_double, ldb: lapack_int)
                         -> lapack_int;

    pub fn LAPACKE_chpsvx(matrix_layout: c_int, fact: c_char, uplo: c_char, n: lapack_int,
                          nrhs: lapack_int, ap: *const lapack_complex_float,
                          afp: *mut lapack_complex_float, ipiv: *mut lapack_int,
                          b: *const lapack_complex_float, ldb: lapack_int,
                          x: *mut lapack_complex_float, ldx: lapack_int, rcond: *mut c_float,
                          ferr: *mut c_float, berr: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_zhpsvx(matrix_layout: c_int, fact: c_char, uplo: c_char, n: lapack_int,
                          nrhs: lapack_int, ap: *const lapack_complex_double,
                          afp: *mut lapack_complex_double, ipiv: *mut lapack_int,
                          b: *const lapack_complex_double, ldb: lapack_int,
                          x: *mut lapack_complex_double, ldx: lapack_int, rcond: *mut c_double,
                          ferr: *mut c_double, berr: *mut c_double)
                          -> lapack_int;

    pub fn LAPACKE_chptrd(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                          ap: *mut lapack_complex_float, d: *mut c_float, e: *mut c_float,
                          tau: *mut lapack_complex_float)
                          -> lapack_int;
    pub fn LAPACKE_zhptrd(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                          ap: *mut lapack_complex_double, d: *mut c_double, e: *mut c_double,
                          tau: *mut lapack_complex_double)
                          -> lapack_int;

    pub fn LAPACKE_chptrf(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                          ap: *mut lapack_complex_float, ipiv: *mut lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_zhptrf(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                          ap: *mut lapack_complex_double, ipiv: *mut lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_chptri(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                          ap: *mut lapack_complex_float, ipiv: *const lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_zhptri(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                          ap: *mut lapack_complex_double, ipiv: *const lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_chptrs(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                          ap: *const lapack_complex_float, ipiv: *const lapack_int,
                          b: *mut lapack_complex_float, ldb: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_zhptrs(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                          ap: *const lapack_complex_double, ipiv: *const lapack_int,
                          b: *mut lapack_complex_double, ldb: lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_shsein(matrix_layout: c_int, job: c_char, eigsrc: c_char, initv: c_char,
                          select: *mut lapack_logical, n: lapack_int, h: *const c_float,
                          ldh: lapack_int, wr: *mut c_float, wi: *const c_float, vl: *mut c_float,
                          ldvl: lapack_int, vr: *mut c_float, ldvr: lapack_int, mm: lapack_int,
                          m: *mut lapack_int, ifaill: *mut lapack_int, ifailr: *mut lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_dhsein(matrix_layout: c_int, job: c_char, eigsrc: c_char, initv: c_char,
                          select: *mut lapack_logical, n: lapack_int, h: *const c_double,
                          ldh: lapack_int, wr: *mut c_double, wi: *const c_double,
                          vl: *mut c_double, ldvl: lapack_int, vr: *mut c_double, ldvr: lapack_int,
                          mm: lapack_int, m: *mut lapack_int, ifaill: *mut lapack_int,
                          ifailr: *mut lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_chsein(matrix_layout: c_int, job: c_char, eigsrc: c_char, initv: c_char,
                          select: *const lapack_logical, n: lapack_int,
                          h: *const lapack_complex_float, ldh: lapack_int,
                          w: *mut lapack_complex_float, vl: *mut lapack_complex_float,
                          ldvl: lapack_int, vr: *mut lapack_complex_float, ldvr: lapack_int,
                          mm: lapack_int, m: *mut lapack_int, ifaill: *mut lapack_int,
                          ifailr: *mut lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_zhsein(matrix_layout: c_int, job: c_char, eigsrc: c_char, initv: c_char,
                          select: *const lapack_logical, n: lapack_int,
                          h: *const lapack_complex_double, ldh: lapack_int,
                          w: *mut lapack_complex_double, vl: *mut lapack_complex_double,
                          ldvl: lapack_int, vr: *mut lapack_complex_double, ldvr: lapack_int,
                          mm: lapack_int, m: *mut lapack_int, ifaill: *mut lapack_int,
                          ifailr: *mut lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_shseqr(matrix_layout: c_int, job: c_char, compz: c_char, n: lapack_int,
                          ilo: lapack_int, ihi: lapack_int, h: *mut c_float, ldh: lapack_int,
                          wr: *mut c_float, wi: *mut c_float, z: *mut c_float, ldz: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_dhseqr(matrix_layout: c_int, job: c_char, compz: c_char, n: lapack_int,
                          ilo: lapack_int, ihi: lapack_int, h: *mut c_double, ldh: lapack_int,
                          wr: *mut c_double, wi: *mut c_double, z: *mut c_double, ldz: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_chseqr(matrix_layout: c_int, job: c_char, compz: c_char, n: lapack_int,
                          ilo: lapack_int, ihi: lapack_int, h: *mut lapack_complex_float,
                          ldh: lapack_int, w: *mut lapack_complex_float,
                          z: *mut lapack_complex_float, ldz: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_zhseqr(matrix_layout: c_int, job: c_char, compz: c_char, n: lapack_int,
                          ilo: lapack_int, ihi: lapack_int, h: *mut lapack_complex_double,
                          ldh: lapack_int, w: *mut lapack_complex_double,
                          z: *mut lapack_complex_double, ldz: lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_clacgv(n: lapack_int, x: *mut lapack_complex_float, incx: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_zlacgv(n: lapack_int, x: *mut lapack_complex_double, incx: lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_slacn2(n: lapack_int, v: *mut c_float, x: *mut c_float, isgn: *mut lapack_int,
                          est: *mut c_float, kase: *mut lapack_int, isave: *mut lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_dlacn2(n: lapack_int, v: *mut c_double, x: *mut c_double, isgn: *mut lapack_int,
                          est: *mut c_double, kase: *mut lapack_int, isave: *mut lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_clacn2(n: lapack_int, v: *mut lapack_complex_float,
                          x: *mut lapack_complex_float, est: *mut c_float, kase: *mut lapack_int,
                          isave: *mut lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_zlacn2(n: lapack_int, v: *mut lapack_complex_double,
                          x: *mut lapack_complex_double, est: *mut c_double, kase: *mut lapack_int,
                          isave: *mut lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_slacpy(matrix_layout: c_int, uplo: c_char, m: lapack_int, n: lapack_int,
                          a: *const c_float, lda: lapack_int, b: *mut c_float, ldb: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_dlacpy(matrix_layout: c_int, uplo: c_char, m: lapack_int, n: lapack_int,
                          a: *const c_double, lda: lapack_int, b: *mut c_double, ldb: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_clacpy(matrix_layout: c_int, uplo: c_char, m: lapack_int, n: lapack_int,
                          a: *const lapack_complex_float, lda: lapack_int,
                          b: *mut lapack_complex_float, ldb: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_zlacpy(matrix_layout: c_int, uplo: c_char, m: lapack_int, n: lapack_int,
                          a: *const lapack_complex_double, lda: lapack_int,
                          b: *mut lapack_complex_double, ldb: lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_clacp2(matrix_layout: c_int, uplo: c_char, m: lapack_int, n: lapack_int,
                          a: *const c_float, lda: lapack_int, b: *mut lapack_complex_float,
                          ldb: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_zlacp2(matrix_layout: c_int, uplo: c_char, m: lapack_int, n: lapack_int,
                          a: *const c_double, lda: lapack_int, b: *mut lapack_complex_double,
                          ldb: lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_zlag2c(matrix_layout: c_int, m: lapack_int, n: lapack_int,
                          a: *const lapack_complex_double, lda: lapack_int,
                          sa: *mut lapack_complex_float, ldsa: lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_slag2d(matrix_layout: c_int, m: lapack_int, n: lapack_int, sa: *const c_float,
                          ldsa: lapack_int, a: *mut c_double, lda: lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_dlag2s(matrix_layout: c_int, m: lapack_int, n: lapack_int, a: *const c_double,
                          lda: lapack_int, sa: *mut c_float, ldsa: lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_clag2z(matrix_layout: c_int, m: lapack_int, n: lapack_int,
                          sa: *const lapack_complex_float, ldsa: lapack_int,
                          a: *mut lapack_complex_double, lda: lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_slagge(matrix_layout: c_int, m: lapack_int, n: lapack_int, kl: lapack_int,
                          ku: lapack_int, d: *const c_float, a: *mut c_float, lda: lapack_int,
                          iseed: *mut lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_dlagge(matrix_layout: c_int, m: lapack_int, n: lapack_int, kl: lapack_int,
                          ku: lapack_int, d: *const c_double, a: *mut c_double, lda: lapack_int,
                          iseed: *mut lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_clagge(matrix_layout: c_int, m: lapack_int, n: lapack_int, kl: lapack_int,
                          ku: lapack_int, d: *const c_float, a: *mut lapack_complex_float,
                          lda: lapack_int, iseed: *mut lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_zlagge(matrix_layout: c_int, m: lapack_int, n: lapack_int, kl: lapack_int,
                          ku: lapack_int, d: *const c_double, a: *mut lapack_complex_double,
                          lda: lapack_int, iseed: *mut lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_slamch(cmach: c_char) -> c_float;
    pub fn LAPACKE_dlamch(cmach: c_char) -> c_double;

    pub fn LAPACKE_slange(matrix_layout: c_int, norm: c_char, m: lapack_int, n: lapack_int,
                          a: *const c_float, lda: lapack_int)
                          -> c_float;
    pub fn LAPACKE_dlange(matrix_layout: c_int, norm: c_char, m: lapack_int, n: lapack_int,
                          a: *const c_double, lda: lapack_int)
                          -> c_double;
    pub fn LAPACKE_clange(matrix_layout: c_int, norm: c_char, m: lapack_int, n: lapack_int,
                          a: *const lapack_complex_float, lda: lapack_int)
                          -> c_float;
    pub fn LAPACKE_zlange(matrix_layout: c_int, norm: c_char, m: lapack_int, n: lapack_int,
                          a: *const lapack_complex_double, lda: lapack_int)
                          -> c_double;

    pub fn LAPACKE_clanhe(matrix_layout: c_int, norm: c_char, uplo: c_char, n: lapack_int,
                          a: *const lapack_complex_float, lda: lapack_int)
                          -> c_float;
    pub fn LAPACKE_zlanhe(matrix_layout: c_int, norm: c_char, uplo: c_char, n: lapack_int,
                          a: *const lapack_complex_double, lda: lapack_int)
                          -> c_double;

    pub fn LAPACKE_slansy(matrix_layout: c_int, norm: c_char, uplo: c_char, n: lapack_int,
                          a: *const c_float, lda: lapack_int)
                          -> c_float;
    pub fn LAPACKE_dlansy(matrix_layout: c_int, norm: c_char, uplo: c_char, n: lapack_int,
                          a: *const c_double, lda: lapack_int)
                          -> c_double;
    pub fn LAPACKE_clansy(matrix_layout: c_int, norm: c_char, uplo: c_char, n: lapack_int,
                          a: *const lapack_complex_float, lda: lapack_int)
                          -> c_float;
    pub fn LAPACKE_zlansy(matrix_layout: c_int, norm: c_char, uplo: c_char, n: lapack_int,
                          a: *const lapack_complex_double, lda: lapack_int)
                          -> c_double;

    pub fn LAPACKE_slantr(matrix_layout: c_int, norm: c_char, uplo: c_char, diag: c_char,
                          m: lapack_int, n: lapack_int, a: *const c_float, lda: lapack_int)
                          -> c_float;
    pub fn LAPACKE_dlantr(matrix_layout: c_int, norm: c_char, uplo: c_char, diag: c_char,
                          m: lapack_int, n: lapack_int, a: *const c_double, lda: lapack_int)
                          -> c_double;
    pub fn LAPACKE_clantr(matrix_layout: c_int, norm: c_char, uplo: c_char, diag: c_char,
                          m: lapack_int, n: lapack_int, a: *const lapack_complex_float,
                          lda: lapack_int)
                          -> c_float;
    pub fn LAPACKE_zlantr(matrix_layout: c_int, norm: c_char, uplo: c_char, diag: c_char,
                          m: lapack_int, n: lapack_int, a: *const lapack_complex_double,
                          lda: lapack_int)
                          -> c_double;

    pub fn LAPACKE_slarfb(matrix_layout: c_int, side: c_char, trans: c_char, direct: c_char,
                          storev: c_char, m: lapack_int, n: lapack_int, k: lapack_int,
                          v: *const c_float, ldv: lapack_int, t: *const c_float, ldt: lapack_int,
                          c: *mut c_float, ldc: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_dlarfb(matrix_layout: c_int, side: c_char, trans: c_char, direct: c_char,
                          storev: c_char, m: lapack_int, n: lapack_int, k: lapack_int,
                          v: *const c_double, ldv: lapack_int, t: *const c_double, ldt: lapack_int,
                          c: *mut c_double, ldc: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_clarfb(matrix_layout: c_int, side: c_char, trans: c_char, direct: c_char,
                          storev: c_char, m: lapack_int, n: lapack_int, k: lapack_int,
                          v: *const lapack_complex_float, ldv: lapack_int,
                          t: *const lapack_complex_float, ldt: lapack_int,
                          c: *mut lapack_complex_float, ldc: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_zlarfb(matrix_layout: c_int, side: c_char, trans: c_char, direct: c_char,
                          storev: c_char, m: lapack_int, n: lapack_int, k: lapack_int,
                          v: *const lapack_complex_double, ldv: lapack_int,
                          t: *const lapack_complex_double, ldt: lapack_int,
                          c: *mut lapack_complex_double, ldc: lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_slarfg(n: lapack_int, alpha: *mut c_float, x: *mut c_float, incx: lapack_int,
                          tau: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_dlarfg(n: lapack_int, alpha: *mut c_double, x: *mut c_double, incx: lapack_int,
                          tau: *mut c_double)
                          -> lapack_int;
    pub fn LAPACKE_clarfg(n: lapack_int, alpha: *mut lapack_complex_float,
                          x: *mut lapack_complex_float, incx: lapack_int,
                          tau: *mut lapack_complex_float)
                          -> lapack_int;
    pub fn LAPACKE_zlarfg(n: lapack_int, alpha: *mut lapack_complex_double,
                          x: *mut lapack_complex_double, incx: lapack_int,
                          tau: *mut lapack_complex_double)
                          -> lapack_int;

    pub fn LAPACKE_slarft(matrix_layout: c_int, direct: c_char, storev: c_char, n: lapack_int,
                          k: lapack_int, v: *const c_float, ldv: lapack_int, tau: *const c_float,
                          t: *mut c_float, ldt: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_dlarft(matrix_layout: c_int, direct: c_char, storev: c_char, n: lapack_int,
                          k: lapack_int, v: *const c_double, ldv: lapack_int, tau: *const c_double,
                          t: *mut c_double, ldt: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_clarft(matrix_layout: c_int, direct: c_char, storev: c_char, n: lapack_int,
                          k: lapack_int, v: *const lapack_complex_float, ldv: lapack_int,
                          tau: *const lapack_complex_float, t: *mut lapack_complex_float,
                          ldt: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_zlarft(matrix_layout: c_int, direct: c_char, storev: c_char, n: lapack_int,
                          k: lapack_int, v: *const lapack_complex_double, ldv: lapack_int,
                          tau: *const lapack_complex_double, t: *mut lapack_complex_double,
                          ldt: lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_slarfx(matrix_layout: c_int, side: c_char, m: lapack_int, n: lapack_int,
                          v: *const c_float, tau: c_float, c: *mut c_float, ldc: lapack_int,
                          work: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_dlarfx(matrix_layout: c_int, side: c_char, m: lapack_int, n: lapack_int,
                          v: *const c_double, tau: c_double, c: *mut c_double, ldc: lapack_int,
                          work: *mut c_double)
                          -> lapack_int;
    pub fn LAPACKE_clarfx(matrix_layout: c_int, side: c_char, m: lapack_int, n: lapack_int,
                          v: *const lapack_complex_float, tau: lapack_complex_float,
                          c: *mut lapack_complex_float, ldc: lapack_int,
                          work: *mut lapack_complex_float)
                          -> lapack_int;
    pub fn LAPACKE_zlarfx(matrix_layout: c_int, side: c_char, m: lapack_int, n: lapack_int,
                          v: *const lapack_complex_double, tau: lapack_complex_double,
                          c: *mut lapack_complex_double, ldc: lapack_int,
                          work: *mut lapack_complex_double)
                          -> lapack_int;

    pub fn LAPACKE_slarnv(idist: lapack_int, iseed: *mut lapack_int, n: lapack_int,
                          x: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_dlarnv(idist: lapack_int, iseed: *mut lapack_int, n: lapack_int,
                          x: *mut c_double)
                          -> lapack_int;
    pub fn LAPACKE_clarnv(idist: lapack_int, iseed: *mut lapack_int, n: lapack_int,
                          x: *mut lapack_complex_float)
                          -> lapack_int;
    pub fn LAPACKE_zlarnv(idist: lapack_int, iseed: *mut lapack_int, n: lapack_int,
                          x: *mut lapack_complex_double)
                          -> lapack_int;

    pub fn LAPACKE_slascl(matrix_layout: c_int, _type: c_char, kl: lapack_int, ku: lapack_int,
                          cfrom: c_float, cto: c_float, m: lapack_int, n: lapack_int,
                          a: *mut c_float, lda: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_dlascl(matrix_layout: c_int, _type: c_char, kl: lapack_int, ku: lapack_int,
                          cfrom: c_double, cto: c_double, m: lapack_int, n: lapack_int,
                          a: *mut c_double, lda: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_clascl(matrix_layout: c_int, _type: c_char, kl: lapack_int, ku: lapack_int,
                          cfrom: c_float, cto: c_float, m: lapack_int, n: lapack_int,
                          a: *mut lapack_complex_float, lda: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_zlascl(matrix_layout: c_int, _type: c_char, kl: lapack_int, ku: lapack_int,
                          cfrom: c_double, cto: c_double, m: lapack_int, n: lapack_int,
                          a: *mut lapack_complex_double, lda: lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_slaset(matrix_layout: c_int, uplo: c_char, m: lapack_int, n: lapack_int,
                          alpha: c_float, beta: c_float, a: *mut c_float, lda: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_dlaset(matrix_layout: c_int, uplo: c_char, m: lapack_int, n: lapack_int,
                          alpha: c_double, beta: c_double, a: *mut c_double, lda: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_claset(matrix_layout: c_int, uplo: c_char, m: lapack_int, n: lapack_int,
                          alpha: lapack_complex_float, beta: lapack_complex_float,
                          a: *mut lapack_complex_float, lda: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_zlaset(matrix_layout: c_int, uplo: c_char, m: lapack_int, n: lapack_int,
                          alpha: lapack_complex_double, beta: lapack_complex_double,
                          a: *mut lapack_complex_double, lda: lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_slasrt(id: c_char, n: lapack_int, d: *mut c_float) -> lapack_int;
    pub fn LAPACKE_dlasrt(id: c_char, n: lapack_int, d: *mut c_double) -> lapack_int;

    pub fn LAPACKE_slaswp(matrix_layout: c_int, n: lapack_int, a: *mut c_float, lda: lapack_int,
                          k1: lapack_int, k2: lapack_int, ipiv: *const lapack_int,
                          incx: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_dlaswp(matrix_layout: c_int, n: lapack_int, a: *mut c_double, lda: lapack_int,
                          k1: lapack_int, k2: lapack_int, ipiv: *const lapack_int,
                          incx: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_claswp(matrix_layout: c_int, n: lapack_int, a: *mut lapack_complex_float,
                          lda: lapack_int, k1: lapack_int, k2: lapack_int, ipiv: *const lapack_int,
                          incx: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_zlaswp(matrix_layout: c_int, n: lapack_int, a: *mut lapack_complex_double,
                          lda: lapack_int, k1: lapack_int, k2: lapack_int, ipiv: *const lapack_int,
                          incx: lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_slatms(matrix_layout: c_int, m: lapack_int, n: lapack_int, dist: c_char,
                          iseed: *mut lapack_int, sym: c_char, d: *mut c_float, mode: lapack_int,
                          cond: c_float, dmax: c_float, kl: lapack_int, ku: lapack_int,
                          pack: c_char, a: *mut c_float, lda: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_dlatms(matrix_layout: c_int, m: lapack_int, n: lapack_int, dist: c_char,
                          iseed: *mut lapack_int, sym: c_char, d: *mut c_double, mode: lapack_int,
                          cond: c_double, dmax: c_double, kl: lapack_int, ku: lapack_int,
                          pack: c_char, a: *mut c_double, lda: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_clatms(matrix_layout: c_int, m: lapack_int, n: lapack_int, dist: c_char,
                          iseed: *mut lapack_int, sym: c_char, d: *mut c_float, mode: lapack_int,
                          cond: c_float, dmax: c_float, kl: lapack_int, ku: lapack_int,
                          pack: c_char, a: *mut lapack_complex_float, lda: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_zlatms(matrix_layout: c_int, m: lapack_int, n: lapack_int, dist: c_char,
                          iseed: *mut lapack_int, sym: c_char, d: *mut c_double, mode: lapack_int,
                          cond: c_double, dmax: c_double, kl: lapack_int, ku: lapack_int,
                          pack: c_char, a: *mut lapack_complex_double, lda: lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_slauum(matrix_layout: c_int, uplo: c_char, n: lapack_int, a: *mut c_float,
                          lda: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_dlauum(matrix_layout: c_int, uplo: c_char, n: lapack_int, a: *mut c_double,
                          lda: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_clauum(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                          a: *mut lapack_complex_float, lda: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_zlauum(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                          a: *mut lapack_complex_double, lda: lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_sopgtr(matrix_layout: c_int, uplo: c_char, n: lapack_int, ap: *const c_float,
                          tau: *const c_float, q: *mut c_float, ldq: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_dopgtr(matrix_layout: c_int, uplo: c_char, n: lapack_int, ap: *const c_double,
                          tau: *const c_double, q: *mut c_double, ldq: lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_sopmtr(matrix_layout: c_int, side: c_char, uplo: c_char, trans: c_char,
                          m: lapack_int, n: lapack_int, ap: *const c_float, tau: *const c_float,
                          c: *mut c_float, ldc: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_dopmtr(matrix_layout: c_int, side: c_char, uplo: c_char, trans: c_char,
                          m: lapack_int, n: lapack_int, ap: *const c_double, tau: *const c_double,
                          c: *mut c_double, ldc: lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_sorgbr(matrix_layout: c_int, vect: c_char, m: lapack_int, n: lapack_int,
                          k: lapack_int, a: *mut c_float, lda: lapack_int, tau: *const c_float)
                          -> lapack_int;
    pub fn LAPACKE_dorgbr(matrix_layout: c_int, vect: c_char, m: lapack_int, n: lapack_int,
                          k: lapack_int, a: *mut c_double, lda: lapack_int, tau: *const c_double)
                          -> lapack_int;

    pub fn LAPACKE_sorghr(matrix_layout: c_int, n: lapack_int, ilo: lapack_int, ihi: lapack_int,
                          a: *mut c_float, lda: lapack_int, tau: *const c_float)
                          -> lapack_int;
    pub fn LAPACKE_dorghr(matrix_layout: c_int, n: lapack_int, ilo: lapack_int, ihi: lapack_int,
                          a: *mut c_double, lda: lapack_int, tau: *const c_double)
                          -> lapack_int;

    pub fn LAPACKE_sorglq(matrix_layout: c_int, m: lapack_int, n: lapack_int, k: lapack_int,
                          a: *mut c_float, lda: lapack_int, tau: *const c_float)
                          -> lapack_int;
    pub fn LAPACKE_dorglq(matrix_layout: c_int, m: lapack_int, n: lapack_int, k: lapack_int,
                          a: *mut c_double, lda: lapack_int, tau: *const c_double)
                          -> lapack_int;

    pub fn LAPACKE_sorgql(matrix_layout: c_int, m: lapack_int, n: lapack_int, k: lapack_int,
                          a: *mut c_float, lda: lapack_int, tau: *const c_float)
                          -> lapack_int;
    pub fn LAPACKE_dorgql(matrix_layout: c_int, m: lapack_int, n: lapack_int, k: lapack_int,
                          a: *mut c_double, lda: lapack_int, tau: *const c_double)
                          -> lapack_int;

    pub fn LAPACKE_sorgqr(matrix_layout: c_int, m: lapack_int, n: lapack_int, k: lapack_int,
                          a: *mut c_float, lda: lapack_int, tau: *const c_float)
                          -> lapack_int;
    pub fn LAPACKE_dorgqr(matrix_layout: c_int, m: lapack_int, n: lapack_int, k: lapack_int,
                          a: *mut c_double, lda: lapack_int, tau: *const c_double)
                          -> lapack_int;

    pub fn LAPACKE_sorgrq(matrix_layout: c_int, m: lapack_int, n: lapack_int, k: lapack_int,
                          a: *mut c_float, lda: lapack_int, tau: *const c_float)
                          -> lapack_int;
    pub fn LAPACKE_dorgrq(matrix_layout: c_int, m: lapack_int, n: lapack_int, k: lapack_int,
                          a: *mut c_double, lda: lapack_int, tau: *const c_double)
                          -> lapack_int;

    pub fn LAPACKE_sorgtr(matrix_layout: c_int, uplo: c_char, n: lapack_int, a: *mut c_float,
                          lda: lapack_int, tau: *const c_float)
                          -> lapack_int;
    pub fn LAPACKE_dorgtr(matrix_layout: c_int, uplo: c_char, n: lapack_int, a: *mut c_double,
                          lda: lapack_int, tau: *const c_double)
                          -> lapack_int;

    pub fn LAPACKE_sormbr(matrix_layout: c_int, vect: c_char, side: c_char, trans: c_char,
                          m: lapack_int, n: lapack_int, k: lapack_int, a: *const c_float,
                          lda: lapack_int, tau: *const c_float, c: *mut c_float, ldc: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_dormbr(matrix_layout: c_int, vect: c_char, side: c_char, trans: c_char,
                          m: lapack_int, n: lapack_int, k: lapack_int, a: *const c_double,
                          lda: lapack_int, tau: *const c_double, c: *mut c_double, ldc: lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_sormhr(matrix_layout: c_int, side: c_char, trans: c_char, m: lapack_int,
                          n: lapack_int, ilo: lapack_int, ihi: lapack_int, a: *const c_float,
                          lda: lapack_int, tau: *const c_float, c: *mut c_float, ldc: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_dormhr(matrix_layout: c_int, side: c_char, trans: c_char, m: lapack_int,
                          n: lapack_int, ilo: lapack_int, ihi: lapack_int, a: *const c_double,
                          lda: lapack_int, tau: *const c_double, c: *mut c_double, ldc: lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_sormlq(matrix_layout: c_int, side: c_char, trans: c_char, m: lapack_int,
                          n: lapack_int, k: lapack_int, a: *const c_float, lda: lapack_int,
                          tau: *const c_float, c: *mut c_float, ldc: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_dormlq(matrix_layout: c_int, side: c_char, trans: c_char, m: lapack_int,
                          n: lapack_int, k: lapack_int, a: *const c_double, lda: lapack_int,
                          tau: *const c_double, c: *mut c_double, ldc: lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_sormql(matrix_layout: c_int, side: c_char, trans: c_char, m: lapack_int,
                          n: lapack_int, k: lapack_int, a: *const c_float, lda: lapack_int,
                          tau: *const c_float, c: *mut c_float, ldc: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_dormql(matrix_layout: c_int, side: c_char, trans: c_char, m: lapack_int,
                          n: lapack_int, k: lapack_int, a: *const c_double, lda: lapack_int,
                          tau: *const c_double, c: *mut c_double, ldc: lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_sormqr(matrix_layout: c_int, side: c_char, trans: c_char, m: lapack_int,
                          n: lapack_int, k: lapack_int, a: *const c_float, lda: lapack_int,
                          tau: *const c_float, c: *mut c_float, ldc: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_dormqr(matrix_layout: c_int, side: c_char, trans: c_char, m: lapack_int,
                          n: lapack_int, k: lapack_int, a: *const c_double, lda: lapack_int,
                          tau: *const c_double, c: *mut c_double, ldc: lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_sormrq(matrix_layout: c_int, side: c_char, trans: c_char, m: lapack_int,
                          n: lapack_int, k: lapack_int, a: *const c_float, lda: lapack_int,
                          tau: *const c_float, c: *mut c_float, ldc: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_dormrq(matrix_layout: c_int, side: c_char, trans: c_char, m: lapack_int,
                          n: lapack_int, k: lapack_int, a: *const c_double, lda: lapack_int,
                          tau: *const c_double, c: *mut c_double, ldc: lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_sormrz(matrix_layout: c_int, side: c_char, trans: c_char, m: lapack_int,
                          n: lapack_int, k: lapack_int, l: lapack_int, a: *const c_float,
                          lda: lapack_int, tau: *const c_float, c: *mut c_float, ldc: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_dormrz(matrix_layout: c_int, side: c_char, trans: c_char, m: lapack_int,
                          n: lapack_int, k: lapack_int, l: lapack_int, a: *const c_double,
                          lda: lapack_int, tau: *const c_double, c: *mut c_double, ldc: lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_sormtr(matrix_layout: c_int, side: c_char, uplo: c_char, trans: c_char,
                          m: lapack_int, n: lapack_int, a: *const c_float, lda: lapack_int,
                          tau: *const c_float, c: *mut c_float, ldc: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_dormtr(matrix_layout: c_int, side: c_char, uplo: c_char, trans: c_char,
                          m: lapack_int, n: lapack_int, a: *const c_double, lda: lapack_int,
                          tau: *const c_double, c: *mut c_double, ldc: lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_spbcon(matrix_layout: c_int, uplo: c_char, n: lapack_int, kd: lapack_int,
                          ab: *const c_float, ldab: lapack_int, anorm: c_float,
                          rcond: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_dpbcon(matrix_layout: c_int, uplo: c_char, n: lapack_int, kd: lapack_int,
                          ab: *const c_double, ldab: lapack_int, anorm: c_double,
                          rcond: *mut c_double)
                          -> lapack_int;
    pub fn LAPACKE_cpbcon(matrix_layout: c_int, uplo: c_char, n: lapack_int, kd: lapack_int,
                          ab: *const lapack_complex_float, ldab: lapack_int, anorm: c_float,
                          rcond: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_zpbcon(matrix_layout: c_int, uplo: c_char, n: lapack_int, kd: lapack_int,
                          ab: *const lapack_complex_double, ldab: lapack_int, anorm: c_double,
                          rcond: *mut c_double)
                          -> lapack_int;

    pub fn LAPACKE_spbequ(matrix_layout: c_int, uplo: c_char, n: lapack_int, kd: lapack_int,
                          ab: *const c_float, ldab: lapack_int, s: *mut c_float,
                          scond: *mut c_float, amax: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_dpbequ(matrix_layout: c_int, uplo: c_char, n: lapack_int, kd: lapack_int,
                          ab: *const c_double, ldab: lapack_int, s: *mut c_double,
                          scond: *mut c_double, amax: *mut c_double)
                          -> lapack_int;
    pub fn LAPACKE_cpbequ(matrix_layout: c_int, uplo: c_char, n: lapack_int, kd: lapack_int,
                          ab: *const lapack_complex_float, ldab: lapack_int, s: *mut c_float,
                          scond: *mut c_float, amax: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_zpbequ(matrix_layout: c_int, uplo: c_char, n: lapack_int, kd: lapack_int,
                          ab: *const lapack_complex_double, ldab: lapack_int, s: *mut c_double,
                          scond: *mut c_double, amax: *mut c_double)
                          -> lapack_int;

    pub fn LAPACKE_spbrfs(matrix_layout: c_int, uplo: c_char, n: lapack_int, kd: lapack_int,
                          nrhs: lapack_int, ab: *const c_float, ldab: lapack_int,
                          afb: *const c_float, ldafb: lapack_int, b: *const c_float,
                          ldb: lapack_int, x: *mut c_float, ldx: lapack_int, ferr: *mut c_float,
                          berr: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_dpbrfs(matrix_layout: c_int, uplo: c_char, n: lapack_int, kd: lapack_int,
                          nrhs: lapack_int, ab: *const c_double, ldab: lapack_int,
                          afb: *const c_double, ldafb: lapack_int, b: *const c_double,
                          ldb: lapack_int, x: *mut c_double, ldx: lapack_int, ferr: *mut c_double,
                          berr: *mut c_double)
                          -> lapack_int;
    pub fn LAPACKE_cpbrfs(matrix_layout: c_int, uplo: c_char, n: lapack_int, kd: lapack_int,
                          nrhs: lapack_int, ab: *const lapack_complex_float, ldab: lapack_int,
                          afb: *const lapack_complex_float, ldafb: lapack_int,
                          b: *const lapack_complex_float, ldb: lapack_int,
                          x: *mut lapack_complex_float, ldx: lapack_int, ferr: *mut c_float,
                          berr: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_zpbrfs(matrix_layout: c_int, uplo: c_char, n: lapack_int, kd: lapack_int,
                          nrhs: lapack_int, ab: *const lapack_complex_double, ldab: lapack_int,
                          afb: *const lapack_complex_double, ldafb: lapack_int,
                          b: *const lapack_complex_double, ldb: lapack_int,
                          x: *mut lapack_complex_double, ldx: lapack_int, ferr: *mut c_double,
                          berr: *mut c_double)
                          -> lapack_int;

    pub fn LAPACKE_spbstf(matrix_layout: c_int, uplo: c_char, n: lapack_int, kb: lapack_int,
                          bb: *mut c_float, ldbb: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_dpbstf(matrix_layout: c_int, uplo: c_char, n: lapack_int, kb: lapack_int,
                          bb: *mut c_double, ldbb: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_cpbstf(matrix_layout: c_int, uplo: c_char, n: lapack_int, kb: lapack_int,
                          bb: *mut lapack_complex_float, ldbb: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_zpbstf(matrix_layout: c_int, uplo: c_char, n: lapack_int, kb: lapack_int,
                          bb: *mut lapack_complex_double, ldbb: lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_spbsv(matrix_layout: c_int, uplo: c_char, n: lapack_int, kd: lapack_int,
                         nrhs: lapack_int, ab: *mut c_float, ldab: lapack_int, b: *mut c_float,
                         ldb: lapack_int)
                         -> lapack_int;
    pub fn LAPACKE_dpbsv(matrix_layout: c_int, uplo: c_char, n: lapack_int, kd: lapack_int,
                         nrhs: lapack_int, ab: *mut c_double, ldab: lapack_int, b: *mut c_double,
                         ldb: lapack_int)
                         -> lapack_int;
    pub fn LAPACKE_cpbsv(matrix_layout: c_int, uplo: c_char, n: lapack_int, kd: lapack_int,
                         nrhs: lapack_int, ab: *mut lapack_complex_float, ldab: lapack_int,
                         b: *mut lapack_complex_float, ldb: lapack_int)
                         -> lapack_int;
    pub fn LAPACKE_zpbsv(matrix_layout: c_int, uplo: c_char, n: lapack_int, kd: lapack_int,
                         nrhs: lapack_int, ab: *mut lapack_complex_double, ldab: lapack_int,
                         b: *mut lapack_complex_double, ldb: lapack_int)
                         -> lapack_int;

    pub fn LAPACKE_spbsvx(matrix_layout: c_int, fact: c_char, uplo: c_char, n: lapack_int,
                          kd: lapack_int, nrhs: lapack_int, ab: *mut c_float, ldab: lapack_int,
                          afb: *mut c_float, ldafb: lapack_int, equed: *mut c_char,
                          s: *mut c_float, b: *mut c_float, ldb: lapack_int, x: *mut c_float,
                          ldx: lapack_int, rcond: *mut c_float, ferr: *mut c_float,
                          berr: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_dpbsvx(matrix_layout: c_int, fact: c_char, uplo: c_char, n: lapack_int,
                          kd: lapack_int, nrhs: lapack_int, ab: *mut c_double, ldab: lapack_int,
                          afb: *mut c_double, ldafb: lapack_int, equed: *mut c_char,
                          s: *mut c_double, b: *mut c_double, ldb: lapack_int, x: *mut c_double,
                          ldx: lapack_int, rcond: *mut c_double, ferr: *mut c_double,
                          berr: *mut c_double)
                          -> lapack_int;
    pub fn LAPACKE_cpbsvx(matrix_layout: c_int, fact: c_char, uplo: c_char, n: lapack_int,
                          kd: lapack_int, nrhs: lapack_int, ab: *mut lapack_complex_float,
                          ldab: lapack_int, afb: *mut lapack_complex_float, ldafb: lapack_int,
                          equed: *mut c_char, s: *mut c_float, b: *mut lapack_complex_float,
                          ldb: lapack_int, x: *mut lapack_complex_float, ldx: lapack_int,
                          rcond: *mut c_float, ferr: *mut c_float, berr: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_zpbsvx(matrix_layout: c_int, fact: c_char, uplo: c_char, n: lapack_int,
                          kd: lapack_int, nrhs: lapack_int, ab: *mut lapack_complex_double,
                          ldab: lapack_int, afb: *mut lapack_complex_double, ldafb: lapack_int,
                          equed: *mut c_char, s: *mut c_double, b: *mut lapack_complex_double,
                          ldb: lapack_int, x: *mut lapack_complex_double, ldx: lapack_int,
                          rcond: *mut c_double, ferr: *mut c_double, berr: *mut c_double)
                          -> lapack_int;

    pub fn LAPACKE_spbtrf(matrix_layout: c_int, uplo: c_char, n: lapack_int, kd: lapack_int,
                          ab: *mut c_float, ldab: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_dpbtrf(matrix_layout: c_int, uplo: c_char, n: lapack_int, kd: lapack_int,
                          ab: *mut c_double, ldab: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_cpbtrf(matrix_layout: c_int, uplo: c_char, n: lapack_int, kd: lapack_int,
                          ab: *mut lapack_complex_float, ldab: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_zpbtrf(matrix_layout: c_int, uplo: c_char, n: lapack_int, kd: lapack_int,
                          ab: *mut lapack_complex_double, ldab: lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_spbtrs(matrix_layout: c_int, uplo: c_char, n: lapack_int, kd: lapack_int,
                          nrhs: lapack_int, ab: *const c_float, ldab: lapack_int, b: *mut c_float,
                          ldb: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_dpbtrs(matrix_layout: c_int, uplo: c_char, n: lapack_int, kd: lapack_int,
                          nrhs: lapack_int, ab: *const c_double, ldab: lapack_int,
                          b: *mut c_double, ldb: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_cpbtrs(matrix_layout: c_int, uplo: c_char, n: lapack_int, kd: lapack_int,
                          nrhs: lapack_int, ab: *const lapack_complex_float, ldab: lapack_int,
                          b: *mut lapack_complex_float, ldb: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_zpbtrs(matrix_layout: c_int, uplo: c_char, n: lapack_int, kd: lapack_int,
                          nrhs: lapack_int, ab: *const lapack_complex_double, ldab: lapack_int,
                          b: *mut lapack_complex_double, ldb: lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_spftrf(matrix_layout: c_int, transr: c_char, uplo: c_char, n: lapack_int,
                          a: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_dpftrf(matrix_layout: c_int, transr: c_char, uplo: c_char, n: lapack_int,
                          a: *mut c_double)
                          -> lapack_int;
    pub fn LAPACKE_cpftrf(matrix_layout: c_int, transr: c_char, uplo: c_char, n: lapack_int,
                          a: *mut lapack_complex_float)
                          -> lapack_int;
    pub fn LAPACKE_zpftrf(matrix_layout: c_int, transr: c_char, uplo: c_char, n: lapack_int,
                          a: *mut lapack_complex_double)
                          -> lapack_int;

    pub fn LAPACKE_spftri(matrix_layout: c_int, transr: c_char, uplo: c_char, n: lapack_int,
                          a: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_dpftri(matrix_layout: c_int, transr: c_char, uplo: c_char, n: lapack_int,
                          a: *mut c_double)
                          -> lapack_int;
    pub fn LAPACKE_cpftri(matrix_layout: c_int, transr: c_char, uplo: c_char, n: lapack_int,
                          a: *mut lapack_complex_float)
                          -> lapack_int;
    pub fn LAPACKE_zpftri(matrix_layout: c_int, transr: c_char, uplo: c_char, n: lapack_int,
                          a: *mut lapack_complex_double)
                          -> lapack_int;

    pub fn LAPACKE_spftrs(matrix_layout: c_int, transr: c_char, uplo: c_char, n: lapack_int,
                          nrhs: lapack_int, a: *const c_float, b: *mut c_float, ldb: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_dpftrs(matrix_layout: c_int, transr: c_char, uplo: c_char, n: lapack_int,
                          nrhs: lapack_int, a: *const c_double, b: *mut c_double, ldb: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_cpftrs(matrix_layout: c_int, transr: c_char, uplo: c_char, n: lapack_int,
                          nrhs: lapack_int, a: *const lapack_complex_float,
                          b: *mut lapack_complex_float, ldb: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_zpftrs(matrix_layout: c_int, transr: c_char, uplo: c_char, n: lapack_int,
                          nrhs: lapack_int, a: *const lapack_complex_double,
                          b: *mut lapack_complex_double, ldb: lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_spocon(matrix_layout: c_int, uplo: c_char, n: lapack_int, a: *const c_float,
                          lda: lapack_int, anorm: c_float, rcond: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_dpocon(matrix_layout: c_int, uplo: c_char, n: lapack_int, a: *const c_double,
                          lda: lapack_int, anorm: c_double, rcond: *mut c_double)
                          -> lapack_int;
    pub fn LAPACKE_cpocon(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                          a: *const lapack_complex_float, lda: lapack_int, anorm: c_float,
                          rcond: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_zpocon(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                          a: *const lapack_complex_double, lda: lapack_int, anorm: c_double,
                          rcond: *mut c_double)
                          -> lapack_int;

    pub fn LAPACKE_spoequ(matrix_layout: c_int, n: lapack_int, a: *const c_float, lda: lapack_int,
                          s: *mut c_float, scond: *mut c_float, amax: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_dpoequ(matrix_layout: c_int, n: lapack_int, a: *const c_double, lda: lapack_int,
                          s: *mut c_double, scond: *mut c_double, amax: *mut c_double)
                          -> lapack_int;
    pub fn LAPACKE_cpoequ(matrix_layout: c_int, n: lapack_int, a: *const lapack_complex_float,
                          lda: lapack_int, s: *mut c_float, scond: *mut c_float,
                          amax: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_zpoequ(matrix_layout: c_int, n: lapack_int, a: *const lapack_complex_double,
                          lda: lapack_int, s: *mut c_double, scond: *mut c_double,
                          amax: *mut c_double)
                          -> lapack_int;

    pub fn LAPACKE_spoequb(matrix_layout: c_int, n: lapack_int, a: *const c_float, lda: lapack_int,
                           s: *mut c_float, scond: *mut c_float, amax: *mut c_float)
                           -> lapack_int;
    pub fn LAPACKE_dpoequb(matrix_layout: c_int, n: lapack_int, a: *const c_double,
                           lda: lapack_int, s: *mut c_double, scond: *mut c_double,
                           amax: *mut c_double)
                           -> lapack_int;
    pub fn LAPACKE_cpoequb(matrix_layout: c_int, n: lapack_int, a: *const lapack_complex_float,
                           lda: lapack_int, s: *mut c_float, scond: *mut c_float,
                           amax: *mut c_float)
                           -> lapack_int;
    pub fn LAPACKE_zpoequb(matrix_layout: c_int, n: lapack_int, a: *const lapack_complex_double,
                           lda: lapack_int, s: *mut c_double, scond: *mut c_double,
                           amax: *mut c_double)
                           -> lapack_int;

    pub fn LAPACKE_sporfs(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                          a: *const c_float, lda: lapack_int, af: *const c_float, ldaf: lapack_int,
                          b: *const c_float, ldb: lapack_int, x: *mut c_float, ldx: lapack_int,
                          ferr: *mut c_float, berr: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_dporfs(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                          a: *const c_double, lda: lapack_int, af: *const c_double,
                          ldaf: lapack_int, b: *const c_double, ldb: lapack_int, x: *mut c_double,
                          ldx: lapack_int, ferr: *mut c_double, berr: *mut c_double)
                          -> lapack_int;
    pub fn LAPACKE_cporfs(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                          a: *const lapack_complex_float, lda: lapack_int,
                          af: *const lapack_complex_float, ldaf: lapack_int,
                          b: *const lapack_complex_float, ldb: lapack_int,
                          x: *mut lapack_complex_float, ldx: lapack_int, ferr: *mut c_float,
                          berr: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_zporfs(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                          a: *const lapack_complex_double, lda: lapack_int,
                          af: *const lapack_complex_double, ldaf: lapack_int,
                          b: *const lapack_complex_double, ldb: lapack_int,
                          x: *mut lapack_complex_double, ldx: lapack_int, ferr: *mut c_double,
                          berr: *mut c_double)
                          -> lapack_int;

    pub fn LAPACKE_sporfsx(matrix_layout: c_int, uplo: c_char, equed: c_char, n: lapack_int,
                           nrhs: lapack_int, a: *const c_float, lda: lapack_int,
                           af: *const c_float, ldaf: lapack_int, s: *const c_float,
                           b: *const c_float, ldb: lapack_int, x: *mut c_float, ldx: lapack_int,
                           rcond: *mut c_float, berr: *mut c_float, n_err_bnds: lapack_int,
                           err_bnds_norm: *mut c_float, err_bnds_comp: *mut c_float,
                           nparams: lapack_int, params: *mut c_float)
                           -> lapack_int;
    pub fn LAPACKE_dporfsx(matrix_layout: c_int, uplo: c_char, equed: c_char, n: lapack_int,
                           nrhs: lapack_int, a: *const c_double, lda: lapack_int,
                           af: *const c_double, ldaf: lapack_int, s: *const c_double,
                           b: *const c_double, ldb: lapack_int, x: *mut c_double, ldx: lapack_int,
                           rcond: *mut c_double, berr: *mut c_double, n_err_bnds: lapack_int,
                           err_bnds_norm: *mut c_double, err_bnds_comp: *mut c_double,
                           nparams: lapack_int, params: *mut c_double)
                           -> lapack_int;
    pub fn LAPACKE_cporfsx(matrix_layout: c_int, uplo: c_char, equed: c_char, n: lapack_int,
                           nrhs: lapack_int, a: *const lapack_complex_float, lda: lapack_int,
                           af: *const lapack_complex_float, ldaf: lapack_int, s: *const c_float,
                           b: *const lapack_complex_float, ldb: lapack_int,
                           x: *mut lapack_complex_float, ldx: lapack_int, rcond: *mut c_float,
                           berr: *mut c_float, n_err_bnds: lapack_int, err_bnds_norm: *mut c_float,
                           err_bnds_comp: *mut c_float, nparams: lapack_int, params: *mut c_float)
                           -> lapack_int;
    pub fn LAPACKE_zporfsx(matrix_layout: c_int, uplo: c_char, equed: c_char, n: lapack_int,
                           nrhs: lapack_int, a: *const lapack_complex_double, lda: lapack_int,
                           af: *const lapack_complex_double, ldaf: lapack_int, s: *const c_double,
                           b: *const lapack_complex_double, ldb: lapack_int,
                           x: *mut lapack_complex_double, ldx: lapack_int, rcond: *mut c_double,
                           berr: *mut c_double, n_err_bnds: lapack_int,
                           err_bnds_norm: *mut c_double, err_bnds_comp: *mut c_double,
                           nparams: lapack_int, params: *mut c_double)
                           -> lapack_int;

    pub fn LAPACKE_sposv(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                         a: *mut c_float, lda: lapack_int, b: *mut c_float, ldb: lapack_int)
                         -> lapack_int;
    pub fn LAPACKE_dposv(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                         a: *mut c_double, lda: lapack_int, b: *mut c_double, ldb: lapack_int)
                         -> lapack_int;
    pub fn LAPACKE_cposv(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                         a: *mut lapack_complex_float, lda: lapack_int,
                         b: *mut lapack_complex_float, ldb: lapack_int)
                         -> lapack_int;
    pub fn LAPACKE_zposv(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                         a: *mut lapack_complex_double, lda: lapack_int,
                         b: *mut lapack_complex_double, ldb: lapack_int)
                         -> lapack_int;
    pub fn LAPACKE_dsposv(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                          a: *mut c_double, lda: lapack_int, b: *mut c_double, ldb: lapack_int,
                          x: *mut c_double, ldx: lapack_int, iter: *mut lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_zcposv(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                          a: *mut lapack_complex_double, lda: lapack_int,
                          b: *mut lapack_complex_double, ldb: lapack_int,
                          x: *mut lapack_complex_double, ldx: lapack_int, iter: *mut lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_sposvx(matrix_layout: c_int, fact: c_char, uplo: c_char, n: lapack_int,
                          nrhs: lapack_int, a: *mut c_float, lda: lapack_int, af: *mut c_float,
                          ldaf: lapack_int, equed: *mut c_char, s: *mut c_float, b: *mut c_float,
                          ldb: lapack_int, x: *mut c_float, ldx: lapack_int, rcond: *mut c_float,
                          ferr: *mut c_float, berr: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_dposvx(matrix_layout: c_int, fact: c_char, uplo: c_char, n: lapack_int,
                          nrhs: lapack_int, a: *mut c_double, lda: lapack_int, af: *mut c_double,
                          ldaf: lapack_int, equed: *mut c_char, s: *mut c_double, b: *mut c_double,
                          ldb: lapack_int, x: *mut c_double, ldx: lapack_int, rcond: *mut c_double,
                          ferr: *mut c_double, berr: *mut c_double)
                          -> lapack_int;
    pub fn LAPACKE_cposvx(matrix_layout: c_int, fact: c_char, uplo: c_char, n: lapack_int,
                          nrhs: lapack_int, a: *mut lapack_complex_float, lda: lapack_int,
                          af: *mut lapack_complex_float, ldaf: lapack_int, equed: *mut c_char,
                          s: *mut c_float, b: *mut lapack_complex_float, ldb: lapack_int,
                          x: *mut lapack_complex_float, ldx: lapack_int, rcond: *mut c_float,
                          ferr: *mut c_float, berr: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_zposvx(matrix_layout: c_int, fact: c_char, uplo: c_char, n: lapack_int,
                          nrhs: lapack_int, a: *mut lapack_complex_double, lda: lapack_int,
                          af: *mut lapack_complex_double, ldaf: lapack_int, equed: *mut c_char,
                          s: *mut c_double, b: *mut lapack_complex_double, ldb: lapack_int,
                          x: *mut lapack_complex_double, ldx: lapack_int, rcond: *mut c_double,
                          ferr: *mut c_double, berr: *mut c_double)
                          -> lapack_int;

    pub fn LAPACKE_sposvxx(matrix_layout: c_int, fact: c_char, uplo: c_char, n: lapack_int,
                           nrhs: lapack_int, a: *mut c_float, lda: lapack_int, af: *mut c_float,
                           ldaf: lapack_int, equed: *mut c_char, s: *mut c_float, b: *mut c_float,
                           ldb: lapack_int, x: *mut c_float, ldx: lapack_int, rcond: *mut c_float,
                           rpvgrw: *mut c_float, berr: *mut c_float, n_err_bnds: lapack_int,
                           err_bnds_norm: *mut c_float, err_bnds_comp: *mut c_float,
                           nparams: lapack_int, params: *mut c_float)
                           -> lapack_int;
    pub fn LAPACKE_dposvxx(matrix_layout: c_int, fact: c_char, uplo: c_char, n: lapack_int,
                           nrhs: lapack_int, a: *mut c_double, lda: lapack_int, af: *mut c_double,
                           ldaf: lapack_int, equed: *mut c_char, s: *mut c_double,
                           b: *mut c_double, ldb: lapack_int, x: *mut c_double, ldx: lapack_int,
                           rcond: *mut c_double, rpvgrw: *mut c_double, berr: *mut c_double,
                           n_err_bnds: lapack_int, err_bnds_norm: *mut c_double,
                           err_bnds_comp: *mut c_double, nparams: lapack_int,
                           params: *mut c_double)
                           -> lapack_int;
    pub fn LAPACKE_cposvxx(matrix_layout: c_int, fact: c_char, uplo: c_char, n: lapack_int,
                           nrhs: lapack_int, a: *mut lapack_complex_float, lda: lapack_int,
                           af: *mut lapack_complex_float, ldaf: lapack_int, equed: *mut c_char,
                           s: *mut c_float, b: *mut lapack_complex_float, ldb: lapack_int,
                           x: *mut lapack_complex_float, ldx: lapack_int, rcond: *mut c_float,
                           rpvgrw: *mut c_float, berr: *mut c_float, n_err_bnds: lapack_int,
                           err_bnds_norm: *mut c_float, err_bnds_comp: *mut c_float,
                           nparams: lapack_int, params: *mut c_float)
                           -> lapack_int;
    pub fn LAPACKE_zposvxx(matrix_layout: c_int, fact: c_char, uplo: c_char, n: lapack_int,
                           nrhs: lapack_int, a: *mut lapack_complex_double, lda: lapack_int,
                           af: *mut lapack_complex_double, ldaf: lapack_int, equed: *mut c_char,
                           s: *mut c_double, b: *mut lapack_complex_double, ldb: lapack_int,
                           x: *mut lapack_complex_double, ldx: lapack_int, rcond: *mut c_double,
                           rpvgrw: *mut c_double, berr: *mut c_double, n_err_bnds: lapack_int,
                           err_bnds_norm: *mut c_double, err_bnds_comp: *mut c_double,
                           nparams: lapack_int, params: *mut c_double)
                           -> lapack_int;

    pub fn LAPACKE_spotrf2(matrix_layout: c_int, uplo: c_char, n: lapack_int, a: *mut c_float,
                           lda: lapack_int)
                           -> lapack_int;
    pub fn LAPACKE_dpotrf2(matrix_layout: c_int, uplo: c_char, n: lapack_int, a: *mut c_double,
                           lda: lapack_int)
                           -> lapack_int;
    pub fn LAPACKE_cpotrf2(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                           a: *mut lapack_complex_float, lda: lapack_int)
                           -> lapack_int;
    pub fn LAPACKE_zpotrf2(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                           a: *mut lapack_complex_double, lda: lapack_int)
                           -> lapack_int;

    pub fn LAPACKE_spotrf(matrix_layout: c_int, uplo: c_char, n: lapack_int, a: *mut c_float,
                          lda: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_dpotrf(matrix_layout: c_int, uplo: c_char, n: lapack_int, a: *mut c_double,
                          lda: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_cpotrf(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                          a: *mut lapack_complex_float, lda: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_zpotrf(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                          a: *mut lapack_complex_double, lda: lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_spotri(matrix_layout: c_int, uplo: c_char, n: lapack_int, a: *mut c_float,
                          lda: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_dpotri(matrix_layout: c_int, uplo: c_char, n: lapack_int, a: *mut c_double,
                          lda: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_cpotri(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                          a: *mut lapack_complex_float, lda: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_zpotri(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                          a: *mut lapack_complex_double, lda: lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_spotrs(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                          a: *const c_float, lda: lapack_int, b: *mut c_float, ldb: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_dpotrs(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                          a: *const c_double, lda: lapack_int, b: *mut c_double, ldb: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_cpotrs(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                          a: *const lapack_complex_float, lda: lapack_int,
                          b: *mut lapack_complex_float, ldb: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_zpotrs(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                          a: *const lapack_complex_double, lda: lapack_int,
                          b: *mut lapack_complex_double, ldb: lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_sppcon(matrix_layout: c_int, uplo: c_char, n: lapack_int, ap: *const c_float,
                          anorm: c_float, rcond: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_dppcon(matrix_layout: c_int, uplo: c_char, n: lapack_int, ap: *const c_double,
                          anorm: c_double, rcond: *mut c_double)
                          -> lapack_int;
    pub fn LAPACKE_cppcon(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                          ap: *const lapack_complex_float, anorm: c_float, rcond: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_zppcon(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                          ap: *const lapack_complex_double, anorm: c_double, rcond: *mut c_double)
                          -> lapack_int;

    pub fn LAPACKE_sppequ(matrix_layout: c_int, uplo: c_char, n: lapack_int, ap: *const c_float,
                          s: *mut c_float, scond: *mut c_float, amax: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_dppequ(matrix_layout: c_int, uplo: c_char, n: lapack_int, ap: *const c_double,
                          s: *mut c_double, scond: *mut c_double, amax: *mut c_double)
                          -> lapack_int;
    pub fn LAPACKE_cppequ(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                          ap: *const lapack_complex_float, s: *mut c_float, scond: *mut c_float,
                          amax: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_zppequ(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                          ap: *const lapack_complex_double, s: *mut c_double, scond: *mut c_double,
                          amax: *mut c_double)
                          -> lapack_int;

    pub fn LAPACKE_spprfs(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                          ap: *const c_float, afp: *const c_float, b: *const c_float,
                          ldb: lapack_int, x: *mut c_float, ldx: lapack_int, ferr: *mut c_float,
                          berr: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_dpprfs(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                          ap: *const c_double, afp: *const c_double, b: *const c_double,
                          ldb: lapack_int, x: *mut c_double, ldx: lapack_int, ferr: *mut c_double,
                          berr: *mut c_double)
                          -> lapack_int;
    pub fn LAPACKE_cpprfs(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                          ap: *const lapack_complex_float, afp: *const lapack_complex_float,
                          b: *const lapack_complex_float, ldb: lapack_int,
                          x: *mut lapack_complex_float, ldx: lapack_int, ferr: *mut c_float,
                          berr: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_zpprfs(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                          ap: *const lapack_complex_double, afp: *const lapack_complex_double,
                          b: *const lapack_complex_double, ldb: lapack_int,
                          x: *mut lapack_complex_double, ldx: lapack_int, ferr: *mut c_double,
                          berr: *mut c_double)
                          -> lapack_int;

    pub fn LAPACKE_sppsv(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                         ap: *mut c_float, b: *mut c_float, ldb: lapack_int)
                         -> lapack_int;
    pub fn LAPACKE_dppsv(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                         ap: *mut c_double, b: *mut c_double, ldb: lapack_int)
                         -> lapack_int;
    pub fn LAPACKE_cppsv(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                         ap: *mut lapack_complex_float, b: *mut lapack_complex_float,
                         ldb: lapack_int)
                         -> lapack_int;
    pub fn LAPACKE_zppsv(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                         ap: *mut lapack_complex_double, b: *mut lapack_complex_double,
                         ldb: lapack_int)
                         -> lapack_int;

    pub fn LAPACKE_sppsvx(matrix_layout: c_int, fact: c_char, uplo: c_char, n: lapack_int,
                          nrhs: lapack_int, ap: *mut c_float, afp: *mut c_float,
                          equed: *mut c_char, s: *mut c_float, b: *mut c_float, ldb: lapack_int,
                          x: *mut c_float, ldx: lapack_int, rcond: *mut c_float,
                          ferr: *mut c_float, berr: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_dppsvx(matrix_layout: c_int, fact: c_char, uplo: c_char, n: lapack_int,
                          nrhs: lapack_int, ap: *mut c_double, afp: *mut c_double,
                          equed: *mut c_char, s: *mut c_double, b: *mut c_double, ldb: lapack_int,
                          x: *mut c_double, ldx: lapack_int, rcond: *mut c_double,
                          ferr: *mut c_double, berr: *mut c_double)
                          -> lapack_int;
    pub fn LAPACKE_cppsvx(matrix_layout: c_int, fact: c_char, uplo: c_char, n: lapack_int,
                          nrhs: lapack_int, ap: *mut lapack_complex_float,
                          afp: *mut lapack_complex_float, equed: *mut c_char, s: *mut c_float,
                          b: *mut lapack_complex_float, ldb: lapack_int,
                          x: *mut lapack_complex_float, ldx: lapack_int, rcond: *mut c_float,
                          ferr: *mut c_float, berr: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_zppsvx(matrix_layout: c_int, fact: c_char, uplo: c_char, n: lapack_int,
                          nrhs: lapack_int, ap: *mut lapack_complex_double,
                          afp: *mut lapack_complex_double, equed: *mut c_char, s: *mut c_double,
                          b: *mut lapack_complex_double, ldb: lapack_int,
                          x: *mut lapack_complex_double, ldx: lapack_int, rcond: *mut c_double,
                          ferr: *mut c_double, berr: *mut c_double)
                          -> lapack_int;

    pub fn LAPACKE_spptrf(matrix_layout: c_int, uplo: c_char, n: lapack_int, ap: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_dpptrf(matrix_layout: c_int, uplo: c_char, n: lapack_int, ap: *mut c_double)
                          -> lapack_int;
    pub fn LAPACKE_cpptrf(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                          ap: *mut lapack_complex_float)
                          -> lapack_int;
    pub fn LAPACKE_zpptrf(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                          ap: *mut lapack_complex_double)
                          -> lapack_int;

    pub fn LAPACKE_spptri(matrix_layout: c_int, uplo: c_char, n: lapack_int, ap: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_dpptri(matrix_layout: c_int, uplo: c_char, n: lapack_int, ap: *mut c_double)
                          -> lapack_int;
    pub fn LAPACKE_cpptri(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                          ap: *mut lapack_complex_float)
                          -> lapack_int;
    pub fn LAPACKE_zpptri(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                          ap: *mut lapack_complex_double)
                          -> lapack_int;

    pub fn LAPACKE_spptrs(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                          ap: *const c_float, b: *mut c_float, ldb: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_dpptrs(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                          ap: *const c_double, b: *mut c_double, ldb: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_cpptrs(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                          ap: *const lapack_complex_float, b: *mut lapack_complex_float,
                          ldb: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_zpptrs(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                          ap: *const lapack_complex_double, b: *mut lapack_complex_double,
                          ldb: lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_spstrf(matrix_layout: c_int, uplo: c_char, n: lapack_int, a: *mut c_float,
                          lda: lapack_int, piv: *mut lapack_int, rank: *mut lapack_int,
                          tol: c_float)
                          -> lapack_int;
    pub fn LAPACKE_dpstrf(matrix_layout: c_int, uplo: c_char, n: lapack_int, a: *mut c_double,
                          lda: lapack_int, piv: *mut lapack_int, rank: *mut lapack_int,
                          tol: c_double)
                          -> lapack_int;
    pub fn LAPACKE_cpstrf(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                          a: *mut lapack_complex_float, lda: lapack_int, piv: *mut lapack_int,
                          rank: *mut lapack_int, tol: c_float)
                          -> lapack_int;
    pub fn LAPACKE_zpstrf(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                          a: *mut lapack_complex_double, lda: lapack_int, piv: *mut lapack_int,
                          rank: *mut lapack_int, tol: c_double)
                          -> lapack_int;

    pub fn LAPACKE_sptcon(n: lapack_int, d: *const c_float, e: *const c_float, anorm: c_float,
                          rcond: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_dptcon(n: lapack_int, d: *const c_double, e: *const c_double, anorm: c_double,
                          rcond: *mut c_double)
                          -> lapack_int;
    pub fn LAPACKE_cptcon(n: lapack_int, d: *const c_float, e: *const lapack_complex_float,
                          anorm: c_float, rcond: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_zptcon(n: lapack_int, d: *const c_double, e: *const lapack_complex_double,
                          anorm: c_double, rcond: *mut c_double)
                          -> lapack_int;

    pub fn LAPACKE_spteqr(matrix_layout: c_int, compz: c_char, n: lapack_int, d: *mut c_float,
                          e: *mut c_float, z: *mut c_float, ldz: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_dpteqr(matrix_layout: c_int, compz: c_char, n: lapack_int, d: *mut c_double,
                          e: *mut c_double, z: *mut c_double, ldz: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_cpteqr(matrix_layout: c_int, compz: c_char, n: lapack_int, d: *mut c_float,
                          e: *mut c_float, z: *mut lapack_complex_float, ldz: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_zpteqr(matrix_layout: c_int, compz: c_char, n: lapack_int, d: *mut c_double,
                          e: *mut c_double, z: *mut lapack_complex_double, ldz: lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_sptrfs(matrix_layout: c_int, n: lapack_int, nrhs: lapack_int, d: *const c_float,
                          e: *const c_float, df: *const c_float, ef: *const c_float,
                          b: *const c_float, ldb: lapack_int, x: *mut c_float, ldx: lapack_int,
                          ferr: *mut c_float, berr: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_dptrfs(matrix_layout: c_int, n: lapack_int, nrhs: lapack_int,
                          d: *const c_double, e: *const c_double, df: *const c_double,
                          ef: *const c_double, b: *const c_double, ldb: lapack_int,
                          x: *mut c_double, ldx: lapack_int, ferr: *mut c_double,
                          berr: *mut c_double)
                          -> lapack_int;
    pub fn LAPACKE_cptrfs(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                          d: *const c_float, e: *const lapack_complex_float, df: *const c_float,
                          ef: *const lapack_complex_float, b: *const lapack_complex_float,
                          ldb: lapack_int, x: *mut lapack_complex_float, ldx: lapack_int,
                          ferr: *mut c_float, berr: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_zptrfs(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                          d: *const c_double, e: *const lapack_complex_double, df: *const c_double,
                          ef: *const lapack_complex_double, b: *const lapack_complex_double,
                          ldb: lapack_int, x: *mut lapack_complex_double, ldx: lapack_int,
                          ferr: *mut c_double, berr: *mut c_double)
                          -> lapack_int;

    pub fn LAPACKE_sptsv(matrix_layout: c_int, n: lapack_int, nrhs: lapack_int, d: *mut c_float,
                         e: *mut c_float, b: *mut c_float, ldb: lapack_int)
                         -> lapack_int;
    pub fn LAPACKE_dptsv(matrix_layout: c_int, n: lapack_int, nrhs: lapack_int, d: *mut c_double,
                         e: *mut c_double, b: *mut c_double, ldb: lapack_int)
                         -> lapack_int;
    pub fn LAPACKE_cptsv(matrix_layout: c_int, n: lapack_int, nrhs: lapack_int, d: *mut c_float,
                         e: *mut lapack_complex_float, b: *mut lapack_complex_float,
                         ldb: lapack_int)
                         -> lapack_int;
    pub fn LAPACKE_zptsv(matrix_layout: c_int, n: lapack_int, nrhs: lapack_int, d: *mut c_double,
                         e: *mut lapack_complex_double, b: *mut lapack_complex_double,
                         ldb: lapack_int)
                         -> lapack_int;

    pub fn LAPACKE_sptsvx(matrix_layout: c_int, fact: c_char, n: lapack_int, nrhs: lapack_int,
                          d: *const c_float, e: *const c_float, df: *mut c_float, ef: *mut c_float,
                          b: *const c_float, ldb: lapack_int, x: *mut c_float, ldx: lapack_int,
                          rcond: *mut c_float, ferr: *mut c_float, berr: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_dptsvx(matrix_layout: c_int, fact: c_char, n: lapack_int, nrhs: lapack_int,
                          d: *const c_double, e: *const c_double, df: *mut c_double,
                          ef: *mut c_double, b: *const c_double, ldb: lapack_int, x: *mut c_double,
                          ldx: lapack_int, rcond: *mut c_double, ferr: *mut c_double,
                          berr: *mut c_double)
                          -> lapack_int;
    pub fn LAPACKE_cptsvx(matrix_layout: c_int, fact: c_char, n: lapack_int, nrhs: lapack_int,
                          d: *const c_float, e: *const lapack_complex_float, df: *mut c_float,
                          ef: *mut lapack_complex_float, b: *const lapack_complex_float,
                          ldb: lapack_int, x: *mut lapack_complex_float, ldx: lapack_int,
                          rcond: *mut c_float, ferr: *mut c_float, berr: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_zptsvx(matrix_layout: c_int, fact: c_char, n: lapack_int, nrhs: lapack_int,
                          d: *const c_double, e: *const lapack_complex_double, df: *mut c_double,
                          ef: *mut lapack_complex_double, b: *const lapack_complex_double,
                          ldb: lapack_int, x: *mut lapack_complex_double, ldx: lapack_int,
                          rcond: *mut c_double, ferr: *mut c_double, berr: *mut c_double)
                          -> lapack_int;

    pub fn LAPACKE_spttrf(n: lapack_int, d: *mut c_float, e: *mut c_float) -> lapack_int;
    pub fn LAPACKE_dpttrf(n: lapack_int, d: *mut c_double, e: *mut c_double) -> lapack_int;
    pub fn LAPACKE_cpttrf(n: lapack_int, d: *mut c_float, e: *mut lapack_complex_float)
                          -> lapack_int;
    pub fn LAPACKE_zpttrf(n: lapack_int, d: *mut c_double, e: *mut lapack_complex_double)
                          -> lapack_int;

    pub fn LAPACKE_spttrs(matrix_layout: c_int, n: lapack_int, nrhs: lapack_int, d: *const c_float,
                          e: *const c_float, b: *mut c_float, ldb: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_dpttrs(matrix_layout: c_int, n: lapack_int, nrhs: lapack_int,
                          d: *const c_double, e: *const c_double, b: *mut c_double,
                          ldb: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_cpttrs(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                          d: *const c_float, e: *const lapack_complex_float,
                          b: *mut lapack_complex_float, ldb: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_zpttrs(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                          d: *const c_double, e: *const lapack_complex_double,
                          b: *mut lapack_complex_double, ldb: lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_ssbev(matrix_layout: c_int, jobz: c_char, uplo: c_char, n: lapack_int,
                         kd: lapack_int, ab: *mut c_float, ldab: lapack_int, w: *mut c_float,
                         z: *mut c_float, ldz: lapack_int)
                         -> lapack_int;
    pub fn LAPACKE_dsbev(matrix_layout: c_int, jobz: c_char, uplo: c_char, n: lapack_int,
                         kd: lapack_int, ab: *mut c_double, ldab: lapack_int, w: *mut c_double,
                         z: *mut c_double, ldz: lapack_int)
                         -> lapack_int;

    pub fn LAPACKE_ssbevd(matrix_layout: c_int, jobz: c_char, uplo: c_char, n: lapack_int,
                          kd: lapack_int, ab: *mut c_float, ldab: lapack_int, w: *mut c_float,
                          z: *mut c_float, ldz: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_dsbevd(matrix_layout: c_int, jobz: c_char, uplo: c_char, n: lapack_int,
                          kd: lapack_int, ab: *mut c_double, ldab: lapack_int, w: *mut c_double,
                          z: *mut c_double, ldz: lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_ssbevx(matrix_layout: c_int, jobz: c_char, range: c_char, uplo: c_char,
                          n: lapack_int, kd: lapack_int, ab: *mut c_float, ldab: lapack_int,
                          q: *mut c_float, ldq: lapack_int, vl: c_float, vu: c_float,
                          il: lapack_int, iu: lapack_int, abstol: c_float, m: *mut lapack_int,
                          w: *mut c_float, z: *mut c_float, ldz: lapack_int,
                          ifail: *mut lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_dsbevx(matrix_layout: c_int, jobz: c_char, range: c_char, uplo: c_char,
                          n: lapack_int, kd: lapack_int, ab: *mut c_double, ldab: lapack_int,
                          q: *mut c_double, ldq: lapack_int, vl: c_double, vu: c_double,
                          il: lapack_int, iu: lapack_int, abstol: c_double, m: *mut lapack_int,
                          w: *mut c_double, z: *mut c_double, ldz: lapack_int,
                          ifail: *mut lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_ssbgst(matrix_layout: c_int, vect: c_char, uplo: c_char, n: lapack_int,
                          ka: lapack_int, kb: lapack_int, ab: *mut c_float, ldab: lapack_int,
                          bb: *const c_float, ldbb: lapack_int, x: *mut c_float, ldx: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_dsbgst(matrix_layout: c_int, vect: c_char, uplo: c_char, n: lapack_int,
                          ka: lapack_int, kb: lapack_int, ab: *mut c_double, ldab: lapack_int,
                          bb: *const c_double, ldbb: lapack_int, x: *mut c_double, ldx: lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_ssbgv(matrix_layout: c_int, jobz: c_char, uplo: c_char, n: lapack_int,
                         ka: lapack_int, kb: lapack_int, ab: *mut c_float, ldab: lapack_int,
                         bb: *mut c_float, ldbb: lapack_int, w: *mut c_float, z: *mut c_float,
                         ldz: lapack_int)
                         -> lapack_int;
    pub fn LAPACKE_dsbgv(matrix_layout: c_int, jobz: c_char, uplo: c_char, n: lapack_int,
                         ka: lapack_int, kb: lapack_int, ab: *mut c_double, ldab: lapack_int,
                         bb: *mut c_double, ldbb: lapack_int, w: *mut c_double, z: *mut c_double,
                         ldz: lapack_int)
                         -> lapack_int;

    pub fn LAPACKE_ssbgvd(matrix_layout: c_int, jobz: c_char, uplo: c_char, n: lapack_int,
                          ka: lapack_int, kb: lapack_int, ab: *mut c_float, ldab: lapack_int,
                          bb: *mut c_float, ldbb: lapack_int, w: *mut c_float, z: *mut c_float,
                          ldz: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_dsbgvd(matrix_layout: c_int, jobz: c_char, uplo: c_char, n: lapack_int,
                          ka: lapack_int, kb: lapack_int, ab: *mut c_double, ldab: lapack_int,
                          bb: *mut c_double, ldbb: lapack_int, w: *mut c_double, z: *mut c_double,
                          ldz: lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_ssbgvx(matrix_layout: c_int, jobz: c_char, range: c_char, uplo: c_char,
                          n: lapack_int, ka: lapack_int, kb: lapack_int, ab: *mut c_float,
                          ldab: lapack_int, bb: *mut c_float, ldbb: lapack_int, q: *mut c_float,
                          ldq: lapack_int, vl: c_float, vu: c_float, il: lapack_int,
                          iu: lapack_int, abstol: c_float, m: *mut lapack_int, w: *mut c_float,
                          z: *mut c_float, ldz: lapack_int, ifail: *mut lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_dsbgvx(matrix_layout: c_int, jobz: c_char, range: c_char, uplo: c_char,
                          n: lapack_int, ka: lapack_int, kb: lapack_int, ab: *mut c_double,
                          ldab: lapack_int, bb: *mut c_double, ldbb: lapack_int, q: *mut c_double,
                          ldq: lapack_int, vl: c_double, vu: c_double, il: lapack_int,
                          iu: lapack_int, abstol: c_double, m: *mut lapack_int, w: *mut c_double,
                          z: *mut c_double, ldz: lapack_int, ifail: *mut lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_ssbtrd(matrix_layout: c_int, vect: c_char, uplo: c_char, n: lapack_int,
                          kd: lapack_int, ab: *mut c_float, ldab: lapack_int, d: *mut c_float,
                          e: *mut c_float, q: *mut c_float, ldq: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_dsbtrd(matrix_layout: c_int, vect: c_char, uplo: c_char, n: lapack_int,
                          kd: lapack_int, ab: *mut c_double, ldab: lapack_int, d: *mut c_double,
                          e: *mut c_double, q: *mut c_double, ldq: lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_ssfrk(matrix_layout: c_int, transr: c_char, uplo: c_char, trans: c_char,
                         n: lapack_int, k: lapack_int, alpha: c_float, a: *const c_float,
                         lda: lapack_int, beta: c_float, c: *mut c_float)
                         -> lapack_int;
    pub fn LAPACKE_dsfrk(matrix_layout: c_int, transr: c_char, uplo: c_char, trans: c_char,
                         n: lapack_int, k: lapack_int, alpha: c_double, a: *const c_double,
                         lda: lapack_int, beta: c_double, c: *mut c_double)
                         -> lapack_int;

    pub fn LAPACKE_sspcon(matrix_layout: c_int, uplo: c_char, n: lapack_int, ap: *const c_float,
                          ipiv: *const lapack_int, anorm: c_float, rcond: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_dspcon(matrix_layout: c_int, uplo: c_char, n: lapack_int, ap: *const c_double,
                          ipiv: *const lapack_int, anorm: c_double, rcond: *mut c_double)
                          -> lapack_int;
    pub fn LAPACKE_cspcon(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                          ap: *const lapack_complex_float, ipiv: *const lapack_int, anorm: c_float,
                          rcond: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_zspcon(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                          ap: *const lapack_complex_double, ipiv: *const lapack_int,
                          anorm: c_double, rcond: *mut c_double)
                          -> lapack_int;

    pub fn LAPACKE_sspev(matrix_layout: c_int, jobz: c_char, uplo: c_char, n: lapack_int,
                         ap: *mut c_float, w: *mut c_float, z: *mut c_float, ldz: lapack_int)
                         -> lapack_int;
    pub fn LAPACKE_dspev(matrix_layout: c_int, jobz: c_char, uplo: c_char, n: lapack_int,
                         ap: *mut c_double, w: *mut c_double, z: *mut c_double, ldz: lapack_int)
                         -> lapack_int;

    pub fn LAPACKE_sspevd(matrix_layout: c_int, jobz: c_char, uplo: c_char, n: lapack_int,
                          ap: *mut c_float, w: *mut c_float, z: *mut c_float, ldz: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_dspevd(matrix_layout: c_int, jobz: c_char, uplo: c_char, n: lapack_int,
                          ap: *mut c_double, w: *mut c_double, z: *mut c_double, ldz: lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_sspevx(matrix_layout: c_int, jobz: c_char, range: c_char, uplo: c_char,
                          n: lapack_int, ap: *mut c_float, vl: c_float, vu: c_float,
                          il: lapack_int, iu: lapack_int, abstol: c_float, m: *mut lapack_int,
                          w: *mut c_float, z: *mut c_float, ldz: lapack_int,
                          ifail: *mut lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_dspevx(matrix_layout: c_int, jobz: c_char, range: c_char, uplo: c_char,
                          n: lapack_int, ap: *mut c_double, vl: c_double, vu: c_double,
                          il: lapack_int, iu: lapack_int, abstol: c_double, m: *mut lapack_int,
                          w: *mut c_double, z: *mut c_double, ldz: lapack_int,
                          ifail: *mut lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_sspgst(matrix_layout: c_int, itype: lapack_int, uplo: c_char, n: lapack_int,
                          ap: *mut c_float, bp: *const c_float)
                          -> lapack_int;
    pub fn LAPACKE_dspgst(matrix_layout: c_int, itype: lapack_int, uplo: c_char, n: lapack_int,
                          ap: *mut c_double, bp: *const c_double)
                          -> lapack_int;

    pub fn LAPACKE_sspgv(matrix_layout: c_int, itype: lapack_int, jobz: c_char, uplo: c_char,
                         n: lapack_int, ap: *mut c_float, bp: *mut c_float, w: *mut c_float,
                         z: *mut c_float, ldz: lapack_int)
                         -> lapack_int;
    pub fn LAPACKE_dspgv(matrix_layout: c_int, itype: lapack_int, jobz: c_char, uplo: c_char,
                         n: lapack_int, ap: *mut c_double, bp: *mut c_double, w: *mut c_double,
                         z: *mut c_double, ldz: lapack_int)
                         -> lapack_int;

    pub fn LAPACKE_sspgvd(matrix_layout: c_int, itype: lapack_int, jobz: c_char, uplo: c_char,
                          n: lapack_int, ap: *mut c_float, bp: *mut c_float, w: *mut c_float,
                          z: *mut c_float, ldz: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_dspgvd(matrix_layout: c_int, itype: lapack_int, jobz: c_char, uplo: c_char,
                          n: lapack_int, ap: *mut c_double, bp: *mut c_double, w: *mut c_double,
                          z: *mut c_double, ldz: lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_sspgvx(matrix_layout: c_int, itype: lapack_int, jobz: c_char, range: c_char,
                          uplo: c_char, n: lapack_int, ap: *mut c_float, bp: *mut c_float,
                          vl: c_float, vu: c_float, il: lapack_int, iu: lapack_int,
                          abstol: c_float, m: *mut lapack_int, w: *mut c_float, z: *mut c_float,
                          ldz: lapack_int, ifail: *mut lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_dspgvx(matrix_layout: c_int, itype: lapack_int, jobz: c_char, range: c_char,
                          uplo: c_char, n: lapack_int, ap: *mut c_double, bp: *mut c_double,
                          vl: c_double, vu: c_double, il: lapack_int, iu: lapack_int,
                          abstol: c_double, m: *mut lapack_int, w: *mut c_double, z: *mut c_double,
                          ldz: lapack_int, ifail: *mut lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_ssprfs(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                          ap: *const c_float, afp: *const c_float, ipiv: *const lapack_int,
                          b: *const c_float, ldb: lapack_int, x: *mut c_float, ldx: lapack_int,
                          ferr: *mut c_float, berr: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_dsprfs(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                          ap: *const c_double, afp: *const c_double, ipiv: *const lapack_int,
                          b: *const c_double, ldb: lapack_int, x: *mut c_double, ldx: lapack_int,
                          ferr: *mut c_double, berr: *mut c_double)
                          -> lapack_int;
    pub fn LAPACKE_csprfs(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                          ap: *const lapack_complex_float, afp: *const lapack_complex_float,
                          ipiv: *const lapack_int, b: *const lapack_complex_float, ldb: lapack_int,
                          x: *mut lapack_complex_float, ldx: lapack_int, ferr: *mut c_float,
                          berr: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_zsprfs(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                          ap: *const lapack_complex_double, afp: *const lapack_complex_double,
                          ipiv: *const lapack_int, b: *const lapack_complex_double,
                          ldb: lapack_int, x: *mut lapack_complex_double, ldx: lapack_int,
                          ferr: *mut c_double, berr: *mut c_double)
                          -> lapack_int;

    pub fn LAPACKE_sspsv(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                         ap: *mut c_float, ipiv: *mut lapack_int, b: *mut c_float, ldb: lapack_int)
                         -> lapack_int;
    pub fn LAPACKE_dspsv(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                         ap: *mut c_double, ipiv: *mut lapack_int, b: *mut c_double,
                         ldb: lapack_int)
                         -> lapack_int;
    pub fn LAPACKE_cspsv(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                         ap: *mut lapack_complex_float, ipiv: *mut lapack_int,
                         b: *mut lapack_complex_float, ldb: lapack_int)
                         -> lapack_int;
    pub fn LAPACKE_zspsv(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                         ap: *mut lapack_complex_double, ipiv: *mut lapack_int,
                         b: *mut lapack_complex_double, ldb: lapack_int)
                         -> lapack_int;

    pub fn LAPACKE_sspsvx(matrix_layout: c_int, fact: c_char, uplo: c_char, n: lapack_int,
                          nrhs: lapack_int, ap: *const c_float, afp: *mut c_float,
                          ipiv: *mut lapack_int, b: *const c_float, ldb: lapack_int,
                          x: *mut c_float, ldx: lapack_int, rcond: *mut c_float,
                          ferr: *mut c_float, berr: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_dspsvx(matrix_layout: c_int, fact: c_char, uplo: c_char, n: lapack_int,
                          nrhs: lapack_int, ap: *const c_double, afp: *mut c_double,
                          ipiv: *mut lapack_int, b: *const c_double, ldb: lapack_int,
                          x: *mut c_double, ldx: lapack_int, rcond: *mut c_double,
                          ferr: *mut c_double, berr: *mut c_double)
                          -> lapack_int;
    pub fn LAPACKE_cspsvx(matrix_layout: c_int, fact: c_char, uplo: c_char, n: lapack_int,
                          nrhs: lapack_int, ap: *const lapack_complex_float,
                          afp: *mut lapack_complex_float, ipiv: *mut lapack_int,
                          b: *const lapack_complex_float, ldb: lapack_int,
                          x: *mut lapack_complex_float, ldx: lapack_int, rcond: *mut c_float,
                          ferr: *mut c_float, berr: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_zspsvx(matrix_layout: c_int, fact: c_char, uplo: c_char, n: lapack_int,
                          nrhs: lapack_int, ap: *const lapack_complex_double,
                          afp: *mut lapack_complex_double, ipiv: *mut lapack_int,
                          b: *const lapack_complex_double, ldb: lapack_int,
                          x: *mut lapack_complex_double, ldx: lapack_int, rcond: *mut c_double,
                          ferr: *mut c_double, berr: *mut c_double)
                          -> lapack_int;

    pub fn LAPACKE_ssptrd(matrix_layout: c_int, uplo: c_char, n: lapack_int, ap: *mut c_float,
                          d: *mut c_float, e: *mut c_float, tau: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_dsptrd(matrix_layout: c_int, uplo: c_char, n: lapack_int, ap: *mut c_double,
                          d: *mut c_double, e: *mut c_double, tau: *mut c_double)
                          -> lapack_int;

    pub fn LAPACKE_ssptrf(matrix_layout: c_int, uplo: c_char, n: lapack_int, ap: *mut c_float,
                          ipiv: *mut lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_dsptrf(matrix_layout: c_int, uplo: c_char, n: lapack_int, ap: *mut c_double,
                          ipiv: *mut lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_csptrf(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                          ap: *mut lapack_complex_float, ipiv: *mut lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_zsptrf(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                          ap: *mut lapack_complex_double, ipiv: *mut lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_ssptri(matrix_layout: c_int, uplo: c_char, n: lapack_int, ap: *mut c_float,
                          ipiv: *const lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_dsptri(matrix_layout: c_int, uplo: c_char, n: lapack_int, ap: *mut c_double,
                          ipiv: *const lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_csptri(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                          ap: *mut lapack_complex_float, ipiv: *const lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_zsptri(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                          ap: *mut lapack_complex_double, ipiv: *const lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_ssptrs(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                          ap: *const c_float, ipiv: *const lapack_int, b: *mut c_float,
                          ldb: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_dsptrs(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                          ap: *const c_double, ipiv: *const lapack_int, b: *mut c_double,
                          ldb: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_csptrs(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                          ap: *const lapack_complex_float, ipiv: *const lapack_int,
                          b: *mut lapack_complex_float, ldb: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_zsptrs(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                          ap: *const lapack_complex_double, ipiv: *const lapack_int,
                          b: *mut lapack_complex_double, ldb: lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_sstebz(range: c_char, order: c_char, n: lapack_int, vl: c_float, vu: c_float,
                          il: lapack_int, iu: lapack_int, abstol: c_float, d: *const c_float,
                          e: *const c_float, m: *mut lapack_int, nsplit: *mut lapack_int,
                          w: *mut c_float, iblock: *mut lapack_int, isplit: *mut lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_dstebz(range: c_char, order: c_char, n: lapack_int, vl: c_double, vu: c_double,
                          il: lapack_int, iu: lapack_int, abstol: c_double, d: *const c_double,
                          e: *const c_double, m: *mut lapack_int, nsplit: *mut lapack_int,
                          w: *mut c_double, iblock: *mut lapack_int, isplit: *mut lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_sstedc(matrix_layout: c_int, compz: c_char, n: lapack_int, d: *mut c_float,
                          e: *mut c_float, z: *mut c_float, ldz: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_dstedc(matrix_layout: c_int, compz: c_char, n: lapack_int, d: *mut c_double,
                          e: *mut c_double, z: *mut c_double, ldz: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_cstedc(matrix_layout: c_int, compz: c_char, n: lapack_int, d: *mut c_float,
                          e: *mut c_float, z: *mut lapack_complex_float, ldz: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_zstedc(matrix_layout: c_int, compz: c_char, n: lapack_int, d: *mut c_double,
                          e: *mut c_double, z: *mut lapack_complex_double, ldz: lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_sstegr(matrix_layout: c_int, jobz: c_char, range: c_char, n: lapack_int,
                          d: *mut c_float, e: *mut c_float, vl: c_float, vu: c_float,
                          il: lapack_int, iu: lapack_int, abstol: c_float, m: *mut lapack_int,
                          w: *mut c_float, z: *mut c_float, ldz: lapack_int,
                          isuppz: *mut lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_dstegr(matrix_layout: c_int, jobz: c_char, range: c_char, n: lapack_int,
                          d: *mut c_double, e: *mut c_double, vl: c_double, vu: c_double,
                          il: lapack_int, iu: lapack_int, abstol: c_double, m: *mut lapack_int,
                          w: *mut c_double, z: *mut c_double, ldz: lapack_int,
                          isuppz: *mut lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_cstegr(matrix_layout: c_int, jobz: c_char, range: c_char, n: lapack_int,
                          d: *mut c_float, e: *mut c_float, vl: c_float, vu: c_float,
                          il: lapack_int, iu: lapack_int, abstol: c_float, m: *mut lapack_int,
                          w: *mut c_float, z: *mut lapack_complex_float, ldz: lapack_int,
                          isuppz: *mut lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_zstegr(matrix_layout: c_int, jobz: c_char, range: c_char, n: lapack_int,
                          d: *mut c_double, e: *mut c_double, vl: c_double, vu: c_double,
                          il: lapack_int, iu: lapack_int, abstol: c_double, m: *mut lapack_int,
                          w: *mut c_double, z: *mut lapack_complex_double, ldz: lapack_int,
                          isuppz: *mut lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_sstein(matrix_layout: c_int, n: lapack_int, d: *const c_float,
                          e: *const c_float, m: lapack_int, w: *const c_float,
                          iblock: *const lapack_int, isplit: *const lapack_int, z: *mut c_float,
                          ldz: lapack_int, ifailv: *mut lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_dstein(matrix_layout: c_int, n: lapack_int, d: *const c_double,
                          e: *const c_double, m: lapack_int, w: *const c_double,
                          iblock: *const lapack_int, isplit: *const lapack_int, z: *mut c_double,
                          ldz: lapack_int, ifailv: *mut lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_cstein(matrix_layout: c_int, n: lapack_int, d: *const c_float,
                          e: *const c_float, m: lapack_int, w: *const c_float,
                          iblock: *const lapack_int, isplit: *const lapack_int,
                          z: *mut lapack_complex_float, ldz: lapack_int, ifailv: *mut lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_zstein(matrix_layout: c_int, n: lapack_int, d: *const c_double,
                          e: *const c_double, m: lapack_int, w: *const c_double,
                          iblock: *const lapack_int, isplit: *const lapack_int,
                          z: *mut lapack_complex_double, ldz: lapack_int, ifailv: *mut lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_sstemr(matrix_layout: c_int, jobz: c_char, range: c_char, n: lapack_int,
                          d: *mut c_float, e: *mut c_float, vl: c_float, vu: c_float,
                          il: lapack_int, iu: lapack_int, m: *mut lapack_int, w: *mut c_float,
                          z: *mut c_float, ldz: lapack_int, nzc: lapack_int,
                          isuppz: *mut lapack_int, tryrac: *mut lapack_logical)
                          -> lapack_int;
    pub fn LAPACKE_dstemr(matrix_layout: c_int, jobz: c_char, range: c_char, n: lapack_int,
                          d: *mut c_double, e: *mut c_double, vl: c_double, vu: c_double,
                          il: lapack_int, iu: lapack_int, m: *mut lapack_int, w: *mut c_double,
                          z: *mut c_double, ldz: lapack_int, nzc: lapack_int,
                          isuppz: *mut lapack_int, tryrac: *mut lapack_logical)
                          -> lapack_int;
    pub fn LAPACKE_cstemr(matrix_layout: c_int, jobz: c_char, range: c_char, n: lapack_int,
                          d: *mut c_float, e: *mut c_float, vl: c_float, vu: c_float,
                          il: lapack_int, iu: lapack_int, m: *mut lapack_int, w: *mut c_float,
                          z: *mut lapack_complex_float, ldz: lapack_int, nzc: lapack_int,
                          isuppz: *mut lapack_int, tryrac: *mut lapack_logical)
                          -> lapack_int;
    pub fn LAPACKE_zstemr(matrix_layout: c_int, jobz: c_char, range: c_char, n: lapack_int,
                          d: *mut c_double, e: *mut c_double, vl: c_double, vu: c_double,
                          il: lapack_int, iu: lapack_int, m: *mut lapack_int, w: *mut c_double,
                          z: *mut lapack_complex_double, ldz: lapack_int, nzc: lapack_int,
                          isuppz: *mut lapack_int, tryrac: *mut lapack_logical)
                          -> lapack_int;

    pub fn LAPACKE_ssteqr(matrix_layout: c_int, compz: c_char, n: lapack_int, d: *mut c_float,
                          e: *mut c_float, z: *mut c_float, ldz: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_dsteqr(matrix_layout: c_int, compz: c_char, n: lapack_int, d: *mut c_double,
                          e: *mut c_double, z: *mut c_double, ldz: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_csteqr(matrix_layout: c_int, compz: c_char, n: lapack_int, d: *mut c_float,
                          e: *mut c_float, z: *mut lapack_complex_float, ldz: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_zsteqr(matrix_layout: c_int, compz: c_char, n: lapack_int, d: *mut c_double,
                          e: *mut c_double, z: *mut lapack_complex_double, ldz: lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_ssterf(n: lapack_int, d: *mut c_float, e: *mut c_float) -> lapack_int;
    pub fn LAPACKE_dsterf(n: lapack_int, d: *mut c_double, e: *mut c_double) -> lapack_int;

    pub fn LAPACKE_sstev(matrix_layout: c_int, jobz: c_char, n: lapack_int, d: *mut c_float,
                         e: *mut c_float, z: *mut c_float, ldz: lapack_int)
                         -> lapack_int;
    pub fn LAPACKE_dstev(matrix_layout: c_int, jobz: c_char, n: lapack_int, d: *mut c_double,
                         e: *mut c_double, z: *mut c_double, ldz: lapack_int)
                         -> lapack_int;

    pub fn LAPACKE_sstevd(matrix_layout: c_int, jobz: c_char, n: lapack_int, d: *mut c_float,
                          e: *mut c_float, z: *mut c_float, ldz: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_dstevd(matrix_layout: c_int, jobz: c_char, n: lapack_int, d: *mut c_double,
                          e: *mut c_double, z: *mut c_double, ldz: lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_sstevr(matrix_layout: c_int, jobz: c_char, range: c_char, n: lapack_int,
                          d: *mut c_float, e: *mut c_float, vl: c_float, vu: c_float,
                          il: lapack_int, iu: lapack_int, abstol: c_float, m: *mut lapack_int,
                          w: *mut c_float, z: *mut c_float, ldz: lapack_int,
                          isuppz: *mut lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_dstevr(matrix_layout: c_int, jobz: c_char, range: c_char, n: lapack_int,
                          d: *mut c_double, e: *mut c_double, vl: c_double, vu: c_double,
                          il: lapack_int, iu: lapack_int, abstol: c_double, m: *mut lapack_int,
                          w: *mut c_double, z: *mut c_double, ldz: lapack_int,
                          isuppz: *mut lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_sstevx(matrix_layout: c_int, jobz: c_char, range: c_char, n: lapack_int,
                          d: *mut c_float, e: *mut c_float, vl: c_float, vu: c_float,
                          il: lapack_int, iu: lapack_int, abstol: c_float, m: *mut lapack_int,
                          w: *mut c_float, z: *mut c_float, ldz: lapack_int,
                          ifail: *mut lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_dstevx(matrix_layout: c_int, jobz: c_char, range: c_char, n: lapack_int,
                          d: *mut c_double, e: *mut c_double, vl: c_double, vu: c_double,
                          il: lapack_int, iu: lapack_int, abstol: c_double, m: *mut lapack_int,
                          w: *mut c_double, z: *mut c_double, ldz: lapack_int,
                          ifail: *mut lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_ssycon(matrix_layout: c_int, uplo: c_char, n: lapack_int, a: *const c_float,
                          lda: lapack_int, ipiv: *const lapack_int, anorm: c_float,
                          rcond: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_dsycon(matrix_layout: c_int, uplo: c_char, n: lapack_int, a: *const c_double,
                          lda: lapack_int, ipiv: *const lapack_int, anorm: c_double,
                          rcond: *mut c_double)
                          -> lapack_int;
    pub fn LAPACKE_csycon(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                          a: *const lapack_complex_float, lda: lapack_int, ipiv: *const lapack_int,
                          anorm: c_float, rcond: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_zsycon(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                          a: *const lapack_complex_double, lda: lapack_int,
                          ipiv: *const lapack_int, anorm: c_double, rcond: *mut c_double)
                          -> lapack_int;

    pub fn LAPACKE_ssyequb(matrix_layout: c_int, uplo: c_char, n: lapack_int, a: *const c_float,
                           lda: lapack_int, s: *mut c_float, scond: *mut c_float,
                           amax: *mut c_float)
                           -> lapack_int;
    pub fn LAPACKE_dsyequb(matrix_layout: c_int, uplo: c_char, n: lapack_int, a: *const c_double,
                           lda: lapack_int, s: *mut c_double, scond: *mut c_double,
                           amax: *mut c_double)
                           -> lapack_int;
    pub fn LAPACKE_csyequb(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                           a: *const lapack_complex_float, lda: lapack_int, s: *mut c_float,
                           scond: *mut c_float, amax: *mut c_float)
                           -> lapack_int;
    pub fn LAPACKE_zsyequb(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                           a: *const lapack_complex_double, lda: lapack_int, s: *mut c_double,
                           scond: *mut c_double, amax: *mut c_double)
                           -> lapack_int;

    pub fn LAPACKE_ssyev(matrix_layout: c_int, jobz: c_char, uplo: c_char, n: lapack_int,
                         a: *mut c_float, lda: lapack_int, w: *mut c_float)
                         -> lapack_int;
    pub fn LAPACKE_dsyev(matrix_layout: c_int, jobz: c_char, uplo: c_char, n: lapack_int,
                         a: *mut c_double, lda: lapack_int, w: *mut c_double)
                         -> lapack_int;

    pub fn LAPACKE_ssyevd(matrix_layout: c_int, jobz: c_char, uplo: c_char, n: lapack_int,
                          a: *mut c_float, lda: lapack_int, w: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_dsyevd(matrix_layout: c_int, jobz: c_char, uplo: c_char, n: lapack_int,
                          a: *mut c_double, lda: lapack_int, w: *mut c_double)
                          -> lapack_int;

    pub fn LAPACKE_ssyevr(matrix_layout: c_int, jobz: c_char, range: c_char, uplo: c_char,
                          n: lapack_int, a: *mut c_float, lda: lapack_int, vl: c_float,
                          vu: c_float, il: lapack_int, iu: lapack_int, abstol: c_float,
                          m: *mut lapack_int, w: *mut c_float, z: *mut c_float, ldz: lapack_int,
                          isuppz: *mut lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_dsyevr(matrix_layout: c_int, jobz: c_char, range: c_char, uplo: c_char,
                          n: lapack_int, a: *mut c_double, lda: lapack_int, vl: c_double,
                          vu: c_double, il: lapack_int, iu: lapack_int, abstol: c_double,
                          m: *mut lapack_int, w: *mut c_double, z: *mut c_double, ldz: lapack_int,
                          isuppz: *mut lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_ssyevx(matrix_layout: c_int, jobz: c_char, range: c_char, uplo: c_char,
                          n: lapack_int, a: *mut c_float, lda: lapack_int, vl: c_float,
                          vu: c_float, il: lapack_int, iu: lapack_int, abstol: c_float,
                          m: *mut lapack_int, w: *mut c_float, z: *mut c_float, ldz: lapack_int,
                          ifail: *mut lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_dsyevx(matrix_layout: c_int, jobz: c_char, range: c_char, uplo: c_char,
                          n: lapack_int, a: *mut c_double, lda: lapack_int, vl: c_double,
                          vu: c_double, il: lapack_int, iu: lapack_int, abstol: c_double,
                          m: *mut lapack_int, w: *mut c_double, z: *mut c_double, ldz: lapack_int,
                          ifail: *mut lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_ssygst(matrix_layout: c_int, itype: lapack_int, uplo: c_char, n: lapack_int,
                          a: *mut c_float, lda: lapack_int, b: *const c_float, ldb: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_dsygst(matrix_layout: c_int, itype: lapack_int, uplo: c_char, n: lapack_int,
                          a: *mut c_double, lda: lapack_int, b: *const c_double, ldb: lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_ssygv(matrix_layout: c_int, itype: lapack_int, jobz: c_char, uplo: c_char,
                         n: lapack_int, a: *mut c_float, lda: lapack_int, b: *mut c_float,
                         ldb: lapack_int, w: *mut c_float)
                         -> lapack_int;
    pub fn LAPACKE_dsygv(matrix_layout: c_int, itype: lapack_int, jobz: c_char, uplo: c_char,
                         n: lapack_int, a: *mut c_double, lda: lapack_int, b: *mut c_double,
                         ldb: lapack_int, w: *mut c_double)
                         -> lapack_int;

    pub fn LAPACKE_ssygvd(matrix_layout: c_int, itype: lapack_int, jobz: c_char, uplo: c_char,
                          n: lapack_int, a: *mut c_float, lda: lapack_int, b: *mut c_float,
                          ldb: lapack_int, w: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_dsygvd(matrix_layout: c_int, itype: lapack_int, jobz: c_char, uplo: c_char,
                          n: lapack_int, a: *mut c_double, lda: lapack_int, b: *mut c_double,
                          ldb: lapack_int, w: *mut c_double)
                          -> lapack_int;

    pub fn LAPACKE_ssygvx(matrix_layout: c_int, itype: lapack_int, jobz: c_char, range: c_char,
                          uplo: c_char, n: lapack_int, a: *mut c_float, lda: lapack_int,
                          b: *mut c_float, ldb: lapack_int, vl: c_float, vu: c_float,
                          il: lapack_int, iu: lapack_int, abstol: c_float, m: *mut lapack_int,
                          w: *mut c_float, z: *mut c_float, ldz: lapack_int,
                          ifail: *mut lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_dsygvx(matrix_layout: c_int, itype: lapack_int, jobz: c_char, range: c_char,
                          uplo: c_char, n: lapack_int, a: *mut c_double, lda: lapack_int,
                          b: *mut c_double, ldb: lapack_int, vl: c_double, vu: c_double,
                          il: lapack_int, iu: lapack_int, abstol: c_double, m: *mut lapack_int,
                          w: *mut c_double, z: *mut c_double, ldz: lapack_int,
                          ifail: *mut lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_ssyrfs(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                          a: *const c_float, lda: lapack_int, af: *const c_float, ldaf: lapack_int,
                          ipiv: *const lapack_int, b: *const c_float, ldb: lapack_int,
                          x: *mut c_float, ldx: lapack_int, ferr: *mut c_float, berr: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_dsyrfs(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                          a: *const c_double, lda: lapack_int, af: *const c_double,
                          ldaf: lapack_int, ipiv: *const lapack_int, b: *const c_double,
                          ldb: lapack_int, x: *mut c_double, ldx: lapack_int, ferr: *mut c_double,
                          berr: *mut c_double)
                          -> lapack_int;
    pub fn LAPACKE_csyrfs(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                          a: *const lapack_complex_float, lda: lapack_int,
                          af: *const lapack_complex_float, ldaf: lapack_int,
                          ipiv: *const lapack_int, b: *const lapack_complex_float, ldb: lapack_int,
                          x: *mut lapack_complex_float, ldx: lapack_int, ferr: *mut c_float,
                          berr: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_zsyrfs(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                          a: *const lapack_complex_double, lda: lapack_int,
                          af: *const lapack_complex_double, ldaf: lapack_int,
                          ipiv: *const lapack_int, b: *const lapack_complex_double,
                          ldb: lapack_int, x: *mut lapack_complex_double, ldx: lapack_int,
                          ferr: *mut c_double, berr: *mut c_double)
                          -> lapack_int;

    pub fn LAPACKE_ssyrfsx(matrix_layout: c_int, uplo: c_char, equed: c_char, n: lapack_int,
                           nrhs: lapack_int, a: *const c_float, lda: lapack_int,
                           af: *const c_float, ldaf: lapack_int, ipiv: *const lapack_int,
                           s: *const c_float, b: *const c_float, ldb: lapack_int, x: *mut c_float,
                           ldx: lapack_int, rcond: *mut c_float, berr: *mut c_float,
                           n_err_bnds: lapack_int, err_bnds_norm: *mut c_float,
                           err_bnds_comp: *mut c_float, nparams: lapack_int, params: *mut c_float)
                           -> lapack_int;
    pub fn LAPACKE_dsyrfsx(matrix_layout: c_int, uplo: c_char, equed: c_char, n: lapack_int,
                           nrhs: lapack_int, a: *const c_double, lda: lapack_int,
                           af: *const c_double, ldaf: lapack_int, ipiv: *const lapack_int,
                           s: *const c_double, b: *const c_double, ldb: lapack_int,
                           x: *mut c_double, ldx: lapack_int, rcond: *mut c_double,
                           berr: *mut c_double, n_err_bnds: lapack_int,
                           err_bnds_norm: *mut c_double, err_bnds_comp: *mut c_double,
                           nparams: lapack_int, params: *mut c_double)
                           -> lapack_int;
    pub fn LAPACKE_csyrfsx(matrix_layout: c_int, uplo: c_char, equed: c_char, n: lapack_int,
                           nrhs: lapack_int, a: *const lapack_complex_float, lda: lapack_int,
                           af: *const lapack_complex_float, ldaf: lapack_int,
                           ipiv: *const lapack_int, s: *const c_float,
                           b: *const lapack_complex_float, ldb: lapack_int,
                           x: *mut lapack_complex_float, ldx: lapack_int, rcond: *mut c_float,
                           berr: *mut c_float, n_err_bnds: lapack_int, err_bnds_norm: *mut c_float,
                           err_bnds_comp: *mut c_float, nparams: lapack_int, params: *mut c_float)
                           -> lapack_int;
    pub fn LAPACKE_zsyrfsx(matrix_layout: c_int, uplo: c_char, equed: c_char, n: lapack_int,
                           nrhs: lapack_int, a: *const lapack_complex_double, lda: lapack_int,
                           af: *const lapack_complex_double, ldaf: lapack_int,
                           ipiv: *const lapack_int, s: *const c_double,
                           b: *const lapack_complex_double, ldb: lapack_int,
                           x: *mut lapack_complex_double, ldx: lapack_int, rcond: *mut c_double,
                           berr: *mut c_double, n_err_bnds: lapack_int,
                           err_bnds_norm: *mut c_double, err_bnds_comp: *mut c_double,
                           nparams: lapack_int, params: *mut c_double)
                           -> lapack_int;

    pub fn LAPACKE_ssysv(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                         a: *mut c_float, lda: lapack_int, ipiv: *mut lapack_int, b: *mut c_float,
                         ldb: lapack_int)
                         -> lapack_int;
    pub fn LAPACKE_dsysv(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                         a: *mut c_double, lda: lapack_int, ipiv: *mut lapack_int,
                         b: *mut c_double, ldb: lapack_int)
                         -> lapack_int;
    pub fn LAPACKE_csysv(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                         a: *mut lapack_complex_float, lda: lapack_int, ipiv: *mut lapack_int,
                         b: *mut lapack_complex_float, ldb: lapack_int)
                         -> lapack_int;
    pub fn LAPACKE_zsysv(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                         a: *mut lapack_complex_double, lda: lapack_int, ipiv: *mut lapack_int,
                         b: *mut lapack_complex_double, ldb: lapack_int)
                         -> lapack_int;

    pub fn LAPACKE_ssysvx(matrix_layout: c_int, fact: c_char, uplo: c_char, n: lapack_int,
                          nrhs: lapack_int, a: *const c_float, lda: lapack_int, af: *mut c_float,
                          ldaf: lapack_int, ipiv: *mut lapack_int, b: *const c_float,
                          ldb: lapack_int, x: *mut c_float, ldx: lapack_int, rcond: *mut c_float,
                          ferr: *mut c_float, berr: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_dsysvx(matrix_layout: c_int, fact: c_char, uplo: c_char, n: lapack_int,
                          nrhs: lapack_int, a: *const c_double, lda: lapack_int, af: *mut c_double,
                          ldaf: lapack_int, ipiv: *mut lapack_int, b: *const c_double,
                          ldb: lapack_int, x: *mut c_double, ldx: lapack_int, rcond: *mut c_double,
                          ferr: *mut c_double, berr: *mut c_double)
                          -> lapack_int;
    pub fn LAPACKE_csysvx(matrix_layout: c_int, fact: c_char, uplo: c_char, n: lapack_int,
                          nrhs: lapack_int, a: *const lapack_complex_float, lda: lapack_int,
                          af: *mut lapack_complex_float, ldaf: lapack_int, ipiv: *mut lapack_int,
                          b: *const lapack_complex_float, ldb: lapack_int,
                          x: *mut lapack_complex_float, ldx: lapack_int, rcond: *mut c_float,
                          ferr: *mut c_float, berr: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_zsysvx(matrix_layout: c_int, fact: c_char, uplo: c_char, n: lapack_int,
                          nrhs: lapack_int, a: *const lapack_complex_double, lda: lapack_int,
                          af: *mut lapack_complex_double, ldaf: lapack_int, ipiv: *mut lapack_int,
                          b: *const lapack_complex_double, ldb: lapack_int,
                          x: *mut lapack_complex_double, ldx: lapack_int, rcond: *mut c_double,
                          ferr: *mut c_double, berr: *mut c_double)
                          -> lapack_int;

    pub fn LAPACKE_ssysvxx(matrix_layout: c_int, fact: c_char, uplo: c_char, n: lapack_int,
                           nrhs: lapack_int, a: *mut c_float, lda: lapack_int, af: *mut c_float,
                           ldaf: lapack_int, ipiv: *mut lapack_int, equed: *mut c_char,
                           s: *mut c_float, b: *mut c_float, ldb: lapack_int, x: *mut c_float,
                           ldx: lapack_int, rcond: *mut c_float, rpvgrw: *mut c_float,
                           berr: *mut c_float, n_err_bnds: lapack_int, err_bnds_norm: *mut c_float,
                           err_bnds_comp: *mut c_float, nparams: lapack_int, params: *mut c_float)
                           -> lapack_int;
    pub fn LAPACKE_dsysvxx(matrix_layout: c_int, fact: c_char, uplo: c_char, n: lapack_int,
                           nrhs: lapack_int, a: *mut c_double, lda: lapack_int, af: *mut c_double,
                           ldaf: lapack_int, ipiv: *mut lapack_int, equed: *mut c_char,
                           s: *mut c_double, b: *mut c_double, ldb: lapack_int, x: *mut c_double,
                           ldx: lapack_int, rcond: *mut c_double, rpvgrw: *mut c_double,
                           berr: *mut c_double, n_err_bnds: lapack_int,
                           err_bnds_norm: *mut c_double, err_bnds_comp: *mut c_double,
                           nparams: lapack_int, params: *mut c_double)
                           -> lapack_int;
    pub fn LAPACKE_csysvxx(matrix_layout: c_int, fact: c_char, uplo: c_char, n: lapack_int,
                           nrhs: lapack_int, a: *mut lapack_complex_float, lda: lapack_int,
                           af: *mut lapack_complex_float, ldaf: lapack_int, ipiv: *mut lapack_int,
                           equed: *mut c_char, s: *mut c_float, b: *mut lapack_complex_float,
                           ldb: lapack_int, x: *mut lapack_complex_float, ldx: lapack_int,
                           rcond: *mut c_float, rpvgrw: *mut c_float, berr: *mut c_float,
                           n_err_bnds: lapack_int, err_bnds_norm: *mut c_float,
                           err_bnds_comp: *mut c_float, nparams: lapack_int, params: *mut c_float)
                           -> lapack_int;
    pub fn LAPACKE_zsysvxx(matrix_layout: c_int, fact: c_char, uplo: c_char, n: lapack_int,
                           nrhs: lapack_int, a: *mut lapack_complex_double, lda: lapack_int,
                           af: *mut lapack_complex_double, ldaf: lapack_int, ipiv: *mut lapack_int,
                           equed: *mut c_char, s: *mut c_double, b: *mut lapack_complex_double,
                           ldb: lapack_int, x: *mut lapack_complex_double, ldx: lapack_int,
                           rcond: *mut c_double, rpvgrw: *mut c_double, berr: *mut c_double,
                           n_err_bnds: lapack_int, err_bnds_norm: *mut c_double,
                           err_bnds_comp: *mut c_double, nparams: lapack_int,
                           params: *mut c_double)
                           -> lapack_int;

    pub fn LAPACKE_ssytrd(matrix_layout: c_int, uplo: c_char, n: lapack_int, a: *mut c_float,
                          lda: lapack_int, d: *mut c_float, e: *mut c_float, tau: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_dsytrd(matrix_layout: c_int, uplo: c_char, n: lapack_int, a: *mut c_double,
                          lda: lapack_int, d: *mut c_double, e: *mut c_double, tau: *mut c_double)
                          -> lapack_int;

    pub fn LAPACKE_ssytrf(matrix_layout: c_int, uplo: c_char, n: lapack_int, a: *mut c_float,
                          lda: lapack_int, ipiv: *mut lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_dsytrf(matrix_layout: c_int, uplo: c_char, n: lapack_int, a: *mut c_double,
                          lda: lapack_int, ipiv: *mut lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_csytrf(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                          a: *mut lapack_complex_float, lda: lapack_int, ipiv: *mut lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_zsytrf(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                          a: *mut lapack_complex_double, lda: lapack_int, ipiv: *mut lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_ssytri(matrix_layout: c_int, uplo: c_char, n: lapack_int, a: *mut c_float,
                          lda: lapack_int, ipiv: *const lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_dsytri(matrix_layout: c_int, uplo: c_char, n: lapack_int, a: *mut c_double,
                          lda: lapack_int, ipiv: *const lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_csytri(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                          a: *mut lapack_complex_float, lda: lapack_int, ipiv: *const lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_zsytri(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                          a: *mut lapack_complex_double, lda: lapack_int, ipiv: *const lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_ssytrs(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                          a: *const c_float, lda: lapack_int, ipiv: *const lapack_int,
                          b: *mut c_float, ldb: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_dsytrs(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                          a: *const c_double, lda: lapack_int, ipiv: *const lapack_int,
                          b: *mut c_double, ldb: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_csytrs(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                          a: *const lapack_complex_float, lda: lapack_int, ipiv: *const lapack_int,
                          b: *mut lapack_complex_float, ldb: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_zsytrs(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                          a: *const lapack_complex_double, lda: lapack_int,
                          ipiv: *const lapack_int, b: *mut lapack_complex_double, ldb: lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_stbcon(matrix_layout: c_int, norm: c_char, uplo: c_char, diag: c_char,
                          n: lapack_int, kd: lapack_int, ab: *const c_float, ldab: lapack_int,
                          rcond: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_dtbcon(matrix_layout: c_int, norm: c_char, uplo: c_char, diag: c_char,
                          n: lapack_int, kd: lapack_int, ab: *const c_double, ldab: lapack_int,
                          rcond: *mut c_double)
                          -> lapack_int;
    pub fn LAPACKE_ctbcon(matrix_layout: c_int, norm: c_char, uplo: c_char, diag: c_char,
                          n: lapack_int, kd: lapack_int, ab: *const lapack_complex_float,
                          ldab: lapack_int, rcond: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_ztbcon(matrix_layout: c_int, norm: c_char, uplo: c_char, diag: c_char,
                          n: lapack_int, kd: lapack_int, ab: *const lapack_complex_double,
                          ldab: lapack_int, rcond: *mut c_double)
                          -> lapack_int;

    pub fn LAPACKE_stbrfs(matrix_layout: c_int, uplo: c_char, trans: c_char, diag: c_char,
                          n: lapack_int, kd: lapack_int, nrhs: lapack_int, ab: *const c_float,
                          ldab: lapack_int, b: *const c_float, ldb: lapack_int, x: *const c_float,
                          ldx: lapack_int, ferr: *mut c_float, berr: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_dtbrfs(matrix_layout: c_int, uplo: c_char, trans: c_char, diag: c_char,
                          n: lapack_int, kd: lapack_int, nrhs: lapack_int, ab: *const c_double,
                          ldab: lapack_int, b: *const c_double, ldb: lapack_int,
                          x: *const c_double, ldx: lapack_int, ferr: *mut c_double,
                          berr: *mut c_double)
                          -> lapack_int;
    pub fn LAPACKE_ctbrfs(matrix_layout: c_int, uplo: c_char, trans: c_char, diag: c_char,
                          n: lapack_int, kd: lapack_int, nrhs: lapack_int,
                          ab: *const lapack_complex_float, ldab: lapack_int,
                          b: *const lapack_complex_float, ldb: lapack_int,
                          x: *const lapack_complex_float, ldx: lapack_int, ferr: *mut c_float,
                          berr: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_ztbrfs(matrix_layout: c_int, uplo: c_char, trans: c_char, diag: c_char,
                          n: lapack_int, kd: lapack_int, nrhs: lapack_int,
                          ab: *const lapack_complex_double, ldab: lapack_int,
                          b: *const lapack_complex_double, ldb: lapack_int,
                          x: *const lapack_complex_double, ldx: lapack_int, ferr: *mut c_double,
                          berr: *mut c_double)
                          -> lapack_int;

    pub fn LAPACKE_stbtrs(matrix_layout: c_int, uplo: c_char, trans: c_char, diag: c_char,
                          n: lapack_int, kd: lapack_int, nrhs: lapack_int, ab: *const c_float,
                          ldab: lapack_int, b: *mut c_float, ldb: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_dtbtrs(matrix_layout: c_int, uplo: c_char, trans: c_char, diag: c_char,
                          n: lapack_int, kd: lapack_int, nrhs: lapack_int, ab: *const c_double,
                          ldab: lapack_int, b: *mut c_double, ldb: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_ctbtrs(matrix_layout: c_int, uplo: c_char, trans: c_char, diag: c_char,
                          n: lapack_int, kd: lapack_int, nrhs: lapack_int,
                          ab: *const lapack_complex_float, ldab: lapack_int,
                          b: *mut lapack_complex_float, ldb: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_ztbtrs(matrix_layout: c_int, uplo: c_char, trans: c_char, diag: c_char,
                          n: lapack_int, kd: lapack_int, nrhs: lapack_int,
                          ab: *const lapack_complex_double, ldab: lapack_int,
                          b: *mut lapack_complex_double, ldb: lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_stfsm(matrix_layout: c_int, transr: c_char, side: c_char, uplo: c_char,
                         trans: c_char, diag: c_char, m: lapack_int, n: lapack_int, alpha: c_float,
                         a: *const c_float, b: *mut c_float, ldb: lapack_int)
                         -> lapack_int;
    pub fn LAPACKE_dtfsm(matrix_layout: c_int, transr: c_char, side: c_char, uplo: c_char,
                         trans: c_char, diag: c_char, m: lapack_int, n: lapack_int,
                         alpha: c_double, a: *const c_double, b: *mut c_double, ldb: lapack_int)
                         -> lapack_int;
    pub fn LAPACKE_ctfsm(matrix_layout: c_int, transr: c_char, side: c_char, uplo: c_char,
                         trans: c_char, diag: c_char, m: lapack_int, n: lapack_int,
                         alpha: lapack_complex_float, a: *const lapack_complex_float,
                         b: *mut lapack_complex_float, ldb: lapack_int)
                         -> lapack_int;
    pub fn LAPACKE_ztfsm(matrix_layout: c_int, transr: c_char, side: c_char, uplo: c_char,
                         trans: c_char, diag: c_char, m: lapack_int, n: lapack_int,
                         alpha: lapack_complex_double, a: *const lapack_complex_double,
                         b: *mut lapack_complex_double, ldb: lapack_int)
                         -> lapack_int;

    pub fn LAPACKE_stftri(matrix_layout: c_int, transr: c_char, uplo: c_char, diag: c_char,
                          n: lapack_int, a: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_dtftri(matrix_layout: c_int, transr: c_char, uplo: c_char, diag: c_char,
                          n: lapack_int, a: *mut c_double)
                          -> lapack_int;
    pub fn LAPACKE_ctftri(matrix_layout: c_int, transr: c_char, uplo: c_char, diag: c_char,
                          n: lapack_int, a: *mut lapack_complex_float)
                          -> lapack_int;
    pub fn LAPACKE_ztftri(matrix_layout: c_int, transr: c_char, uplo: c_char, diag: c_char,
                          n: lapack_int, a: *mut lapack_complex_double)
                          -> lapack_int;

    pub fn LAPACKE_stfttp(matrix_layout: c_int, transr: c_char, uplo: c_char, n: lapack_int,
                          arf: *const c_float, ap: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_dtfttp(matrix_layout: c_int, transr: c_char, uplo: c_char, n: lapack_int,
                          arf: *const c_double, ap: *mut c_double)
                          -> lapack_int;
    pub fn LAPACKE_ctfttp(matrix_layout: c_int, transr: c_char, uplo: c_char, n: lapack_int,
                          arf: *const lapack_complex_float, ap: *mut lapack_complex_float)
                          -> lapack_int;
    pub fn LAPACKE_ztfttp(matrix_layout: c_int, transr: c_char, uplo: c_char, n: lapack_int,
                          arf: *const lapack_complex_double, ap: *mut lapack_complex_double)
                          -> lapack_int;

    pub fn LAPACKE_stfttr(matrix_layout: c_int, transr: c_char, uplo: c_char, n: lapack_int,
                          arf: *const c_float, a: *mut c_float, lda: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_dtfttr(matrix_layout: c_int, transr: c_char, uplo: c_char, n: lapack_int,
                          arf: *const c_double, a: *mut c_double, lda: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_ctfttr(matrix_layout: c_int, transr: c_char, uplo: c_char, n: lapack_int,
                          arf: *const lapack_complex_float, a: *mut lapack_complex_float,
                          lda: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_ztfttr(matrix_layout: c_int, transr: c_char, uplo: c_char, n: lapack_int,
                          arf: *const lapack_complex_double, a: *mut lapack_complex_double,
                          lda: lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_stgevc(matrix_layout: c_int, side: c_char, howmny: c_char,
                          select: *const lapack_logical, n: lapack_int, s: *const c_float,
                          lds: lapack_int, p: *const c_float, ldp: lapack_int, vl: *mut c_float,
                          ldvl: lapack_int, vr: *mut c_float, ldvr: lapack_int, mm: lapack_int,
                          m: *mut lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_dtgevc(matrix_layout: c_int, side: c_char, howmny: c_char,
                          select: *const lapack_logical, n: lapack_int, s: *const c_double,
                          lds: lapack_int, p: *const c_double, ldp: lapack_int, vl: *mut c_double,
                          ldvl: lapack_int, vr: *mut c_double, ldvr: lapack_int, mm: lapack_int,
                          m: *mut lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_ctgevc(matrix_layout: c_int, side: c_char, howmny: c_char,
                          select: *const lapack_logical, n: lapack_int,
                          s: *const lapack_complex_float, lds: lapack_int,
                          p: *const lapack_complex_float, ldp: lapack_int,
                          vl: *mut lapack_complex_float, ldvl: lapack_int,
                          vr: *mut lapack_complex_float, ldvr: lapack_int, mm: lapack_int,
                          m: *mut lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_ztgevc(matrix_layout: c_int, side: c_char, howmny: c_char,
                          select: *const lapack_logical, n: lapack_int,
                          s: *const lapack_complex_double, lds: lapack_int,
                          p: *const lapack_complex_double, ldp: lapack_int,
                          vl: *mut lapack_complex_double, ldvl: lapack_int,
                          vr: *mut lapack_complex_double, ldvr: lapack_int, mm: lapack_int,
                          m: *mut lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_stgexc(matrix_layout: c_int, wantq: lapack_logical, wantz: lapack_logical,
                          n: lapack_int, a: *mut c_float, lda: lapack_int, b: *mut c_float,
                          ldb: lapack_int, q: *mut c_float, ldq: lapack_int, z: *mut c_float,
                          ldz: lapack_int, ifst: *mut lapack_int, ilst: *mut lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_dtgexc(matrix_layout: c_int, wantq: lapack_logical, wantz: lapack_logical,
                          n: lapack_int, a: *mut c_double, lda: lapack_int, b: *mut c_double,
                          ldb: lapack_int, q: *mut c_double, ldq: lapack_int, z: *mut c_double,
                          ldz: lapack_int, ifst: *mut lapack_int, ilst: *mut lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_ctgexc(matrix_layout: c_int, wantq: lapack_logical, wantz: lapack_logical,
                          n: lapack_int, a: *mut lapack_complex_float, lda: lapack_int,
                          b: *mut lapack_complex_float, ldb: lapack_int,
                          q: *mut lapack_complex_float, ldq: lapack_int,
                          z: *mut lapack_complex_float, ldz: lapack_int, ifst: lapack_int,
                          ilst: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_ztgexc(matrix_layout: c_int, wantq: lapack_logical, wantz: lapack_logical,
                          n: lapack_int, a: *mut lapack_complex_double, lda: lapack_int,
                          b: *mut lapack_complex_double, ldb: lapack_int,
                          q: *mut lapack_complex_double, ldq: lapack_int,
                          z: *mut lapack_complex_double, ldz: lapack_int, ifst: lapack_int,
                          ilst: lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_stgsen(matrix_layout: c_int, ijob: lapack_int, wantq: lapack_logical,
                          wantz: lapack_logical, select: *const lapack_logical, n: lapack_int,
                          a: *mut c_float, lda: lapack_int, b: *mut c_float, ldb: lapack_int,
                          alphar: *mut c_float, alphai: *mut c_float, beta: *mut c_float,
                          q: *mut c_float, ldq: lapack_int, z: *mut c_float, ldz: lapack_int,
                          m: *mut lapack_int, pl: *mut c_float, pr: *mut c_float,
                          dif: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_dtgsen(matrix_layout: c_int, ijob: lapack_int, wantq: lapack_logical,
                          wantz: lapack_logical, select: *const lapack_logical, n: lapack_int,
                          a: *mut c_double, lda: lapack_int, b: *mut c_double, ldb: lapack_int,
                          alphar: *mut c_double, alphai: *mut c_double, beta: *mut c_double,
                          q: *mut c_double, ldq: lapack_int, z: *mut c_double, ldz: lapack_int,
                          m: *mut lapack_int, pl: *mut c_double, pr: *mut c_double,
                          dif: *mut c_double)
                          -> lapack_int;
    pub fn LAPACKE_ctgsen(matrix_layout: c_int, ijob: lapack_int, wantq: lapack_logical,
                          wantz: lapack_logical, select: *const lapack_logical, n: lapack_int,
                          a: *mut lapack_complex_float, lda: lapack_int,
                          b: *mut lapack_complex_float, ldb: lapack_int,
                          alpha: *mut lapack_complex_float, beta: *mut lapack_complex_float,
                          q: *mut lapack_complex_float, ldq: lapack_int,
                          z: *mut lapack_complex_float, ldz: lapack_int, m: *mut lapack_int,
                          pl: *mut c_float, pr: *mut c_float, dif: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_ztgsen(matrix_layout: c_int, ijob: lapack_int, wantq: lapack_logical,
                          wantz: lapack_logical, select: *const lapack_logical, n: lapack_int,
                          a: *mut lapack_complex_double, lda: lapack_int,
                          b: *mut lapack_complex_double, ldb: lapack_int,
                          alpha: *mut lapack_complex_double, beta: *mut lapack_complex_double,
                          q: *mut lapack_complex_double, ldq: lapack_int,
                          z: *mut lapack_complex_double, ldz: lapack_int, m: *mut lapack_int,
                          pl: *mut c_double, pr: *mut c_double, dif: *mut c_double)
                          -> lapack_int;

    pub fn LAPACKE_stgsja(matrix_layout: c_int, jobu: c_char, jobv: c_char, jobq: c_char,
                          m: lapack_int, p: lapack_int, n: lapack_int, k: lapack_int,
                          l: lapack_int, a: *mut c_float, lda: lapack_int, b: *mut c_float,
                          ldb: lapack_int, tola: c_float, tolb: c_float, alpha: *mut c_float,
                          beta: *mut c_float, u: *mut c_float, ldu: lapack_int, v: *mut c_float,
                          ldv: lapack_int, q: *mut c_float, ldq: lapack_int,
                          ncycle: *mut lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_dtgsja(matrix_layout: c_int, jobu: c_char, jobv: c_char, jobq: c_char,
                          m: lapack_int, p: lapack_int, n: lapack_int, k: lapack_int,
                          l: lapack_int, a: *mut c_double, lda: lapack_int, b: *mut c_double,
                          ldb: lapack_int, tola: c_double, tolb: c_double, alpha: *mut c_double,
                          beta: *mut c_double, u: *mut c_double, ldu: lapack_int, v: *mut c_double,
                          ldv: lapack_int, q: *mut c_double, ldq: lapack_int,
                          ncycle: *mut lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_ctgsja(matrix_layout: c_int, jobu: c_char, jobv: c_char, jobq: c_char,
                          m: lapack_int, p: lapack_int, n: lapack_int, k: lapack_int,
                          l: lapack_int, a: *mut lapack_complex_float, lda: lapack_int,
                          b: *mut lapack_complex_float, ldb: lapack_int, tola: c_float,
                          tolb: c_float, alpha: *mut c_float, beta: *mut c_float,
                          u: *mut lapack_complex_float, ldu: lapack_int,
                          v: *mut lapack_complex_float, ldv: lapack_int,
                          q: *mut lapack_complex_float, ldq: lapack_int, ncycle: *mut lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_ztgsja(matrix_layout: c_int, jobu: c_char, jobv: c_char, jobq: c_char,
                          m: lapack_int, p: lapack_int, n: lapack_int, k: lapack_int,
                          l: lapack_int, a: *mut lapack_complex_double, lda: lapack_int,
                          b: *mut lapack_complex_double, ldb: lapack_int, tola: c_double,
                          tolb: c_double, alpha: *mut c_double, beta: *mut c_double,
                          u: *mut lapack_complex_double, ldu: lapack_int,
                          v: *mut lapack_complex_double, ldv: lapack_int,
                          q: *mut lapack_complex_double, ldq: lapack_int, ncycle: *mut lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_stgsna(matrix_layout: c_int, job: c_char, howmny: c_char,
                          select: *const lapack_logical, n: lapack_int, a: *const c_float,
                          lda: lapack_int, b: *const c_float, ldb: lapack_int, vl: *const c_float,
                          ldvl: lapack_int, vr: *const c_float, ldvr: lapack_int, s: *mut c_float,
                          dif: *mut c_float, mm: lapack_int, m: *mut lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_dtgsna(matrix_layout: c_int, job: c_char, howmny: c_char,
                          select: *const lapack_logical, n: lapack_int, a: *const c_double,
                          lda: lapack_int, b: *const c_double, ldb: lapack_int,
                          vl: *const c_double, ldvl: lapack_int, vr: *const c_double,
                          ldvr: lapack_int, s: *mut c_double, dif: *mut c_double, mm: lapack_int,
                          m: *mut lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_ctgsna(matrix_layout: c_int, job: c_char, howmny: c_char,
                          select: *const lapack_logical, n: lapack_int,
                          a: *const lapack_complex_float, lda: lapack_int,
                          b: *const lapack_complex_float, ldb: lapack_int,
                          vl: *const lapack_complex_float, ldvl: lapack_int,
                          vr: *const lapack_complex_float, ldvr: lapack_int, s: *mut c_float,
                          dif: *mut c_float, mm: lapack_int, m: *mut lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_ztgsna(matrix_layout: c_int, job: c_char, howmny: c_char,
                          select: *const lapack_logical, n: lapack_int,
                          a: *const lapack_complex_double, lda: lapack_int,
                          b: *const lapack_complex_double, ldb: lapack_int,
                          vl: *const lapack_complex_double, ldvl: lapack_int,
                          vr: *const lapack_complex_double, ldvr: lapack_int, s: *mut c_double,
                          dif: *mut c_double, mm: lapack_int, m: *mut lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_stgsyl(matrix_layout: c_int, trans: c_char, ijob: lapack_int, m: lapack_int,
                          n: lapack_int, a: *const c_float, lda: lapack_int, b: *const c_float,
                          ldb: lapack_int, c: *mut c_float, ldc: lapack_int, d: *const c_float,
                          ldd: lapack_int, e: *const c_float, lde: lapack_int, f: *mut c_float,
                          ldf: lapack_int, scale: *mut c_float, dif: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_dtgsyl(matrix_layout: c_int, trans: c_char, ijob: lapack_int, m: lapack_int,
                          n: lapack_int, a: *const c_double, lda: lapack_int, b: *const c_double,
                          ldb: lapack_int, c: *mut c_double, ldc: lapack_int, d: *const c_double,
                          ldd: lapack_int, e: *const c_double, lde: lapack_int, f: *mut c_double,
                          ldf: lapack_int, scale: *mut c_double, dif: *mut c_double)
                          -> lapack_int;
    pub fn LAPACKE_ctgsyl(matrix_layout: c_int, trans: c_char, ijob: lapack_int, m: lapack_int,
                          n: lapack_int, a: *const lapack_complex_float, lda: lapack_int,
                          b: *const lapack_complex_float, ldb: lapack_int,
                          c: *mut lapack_complex_float, ldc: lapack_int,
                          d: *const lapack_complex_float, ldd: lapack_int,
                          e: *const lapack_complex_float, lde: lapack_int,
                          f: *mut lapack_complex_float, ldf: lapack_int, scale: *mut c_float,
                          dif: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_ztgsyl(matrix_layout: c_int, trans: c_char, ijob: lapack_int, m: lapack_int,
                          n: lapack_int, a: *const lapack_complex_double, lda: lapack_int,
                          b: *const lapack_complex_double, ldb: lapack_int,
                          c: *mut lapack_complex_double, ldc: lapack_int,
                          d: *const lapack_complex_double, ldd: lapack_int,
                          e: *const lapack_complex_double, lde: lapack_int,
                          f: *mut lapack_complex_double, ldf: lapack_int, scale: *mut c_double,
                          dif: *mut c_double)
                          -> lapack_int;

    pub fn LAPACKE_stpcon(matrix_layout: c_int, norm: c_char, uplo: c_char, diag: c_char,
                          n: lapack_int, ap: *const c_float, rcond: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_dtpcon(matrix_layout: c_int, norm: c_char, uplo: c_char, diag: c_char,
                          n: lapack_int, ap: *const c_double, rcond: *mut c_double)
                          -> lapack_int;
    pub fn LAPACKE_ctpcon(matrix_layout: c_int, norm: c_char, uplo: c_char, diag: c_char,
                          n: lapack_int, ap: *const lapack_complex_float, rcond: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_ztpcon(matrix_layout: c_int, norm: c_char, uplo: c_char, diag: c_char,
                          n: lapack_int, ap: *const lapack_complex_double, rcond: *mut c_double)
                          -> lapack_int;

    pub fn LAPACKE_stprfs(matrix_layout: c_int, uplo: c_char, trans: c_char, diag: c_char,
                          n: lapack_int, nrhs: lapack_int, ap: *const c_float, b: *const c_float,
                          ldb: lapack_int, x: *const c_float, ldx: lapack_int, ferr: *mut c_float,
                          berr: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_dtprfs(matrix_layout: c_int, uplo: c_char, trans: c_char, diag: c_char,
                          n: lapack_int, nrhs: lapack_int, ap: *const c_double, b: *const c_double,
                          ldb: lapack_int, x: *const c_double, ldx: lapack_int,
                          ferr: *mut c_double, berr: *mut c_double)
                          -> lapack_int;
    pub fn LAPACKE_ctprfs(matrix_layout: c_int, uplo: c_char, trans: c_char, diag: c_char,
                          n: lapack_int, nrhs: lapack_int, ap: *const lapack_complex_float,
                          b: *const lapack_complex_float, ldb: lapack_int,
                          x: *const lapack_complex_float, ldx: lapack_int, ferr: *mut c_float,
                          berr: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_ztprfs(matrix_layout: c_int, uplo: c_char, trans: c_char, diag: c_char,
                          n: lapack_int, nrhs: lapack_int, ap: *const lapack_complex_double,
                          b: *const lapack_complex_double, ldb: lapack_int,
                          x: *const lapack_complex_double, ldx: lapack_int, ferr: *mut c_double,
                          berr: *mut c_double)
                          -> lapack_int;

    pub fn LAPACKE_stptri(matrix_layout: c_int, uplo: c_char, diag: c_char, n: lapack_int,
                          ap: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_dtptri(matrix_layout: c_int, uplo: c_char, diag: c_char, n: lapack_int,
                          ap: *mut c_double)
                          -> lapack_int;
    pub fn LAPACKE_ctptri(matrix_layout: c_int, uplo: c_char, diag: c_char, n: lapack_int,
                          ap: *mut lapack_complex_float)
                          -> lapack_int;
    pub fn LAPACKE_ztptri(matrix_layout: c_int, uplo: c_char, diag: c_char, n: lapack_int,
                          ap: *mut lapack_complex_double)
                          -> lapack_int;

    pub fn LAPACKE_stptrs(matrix_layout: c_int, uplo: c_char, trans: c_char, diag: c_char,
                          n: lapack_int, nrhs: lapack_int, ap: *const c_float, b: *mut c_float,
                          ldb: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_dtptrs(matrix_layout: c_int, uplo: c_char, trans: c_char, diag: c_char,
                          n: lapack_int, nrhs: lapack_int, ap: *const c_double, b: *mut c_double,
                          ldb: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_ctptrs(matrix_layout: c_int, uplo: c_char, trans: c_char, diag: c_char,
                          n: lapack_int, nrhs: lapack_int, ap: *const lapack_complex_float,
                          b: *mut lapack_complex_float, ldb: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_ztptrs(matrix_layout: c_int, uplo: c_char, trans: c_char, diag: c_char,
                          n: lapack_int, nrhs: lapack_int, ap: *const lapack_complex_double,
                          b: *mut lapack_complex_double, ldb: lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_stpttf(matrix_layout: c_int, transr: c_char, uplo: c_char, n: lapack_int,
                          ap: *const c_float, arf: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_dtpttf(matrix_layout: c_int, transr: c_char, uplo: c_char, n: lapack_int,
                          ap: *const c_double, arf: *mut c_double)
                          -> lapack_int;
    pub fn LAPACKE_ctpttf(matrix_layout: c_int, transr: c_char, uplo: c_char, n: lapack_int,
                          ap: *const lapack_complex_float, arf: *mut lapack_complex_float)
                          -> lapack_int;
    pub fn LAPACKE_ztpttf(matrix_layout: c_int, transr: c_char, uplo: c_char, n: lapack_int,
                          ap: *const lapack_complex_double, arf: *mut lapack_complex_double)
                          -> lapack_int;

    pub fn LAPACKE_stpttr(matrix_layout: c_int, uplo: c_char, n: lapack_int, ap: *const c_float,
                          a: *mut c_float, lda: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_dtpttr(matrix_layout: c_int, uplo: c_char, n: lapack_int, ap: *const c_double,
                          a: *mut c_double, lda: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_ctpttr(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                          ap: *const lapack_complex_float, a: *mut lapack_complex_float,
                          lda: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_ztpttr(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                          ap: *const lapack_complex_double, a: *mut lapack_complex_double,
                          lda: lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_strcon(matrix_layout: c_int, norm: c_char, uplo: c_char, diag: c_char,
                          n: lapack_int, a: *const c_float, lda: lapack_int, rcond: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_dtrcon(matrix_layout: c_int, norm: c_char, uplo: c_char, diag: c_char,
                          n: lapack_int, a: *const c_double, lda: lapack_int, rcond: *mut c_double)
                          -> lapack_int;
    pub fn LAPACKE_ctrcon(matrix_layout: c_int, norm: c_char, uplo: c_char, diag: c_char,
                          n: lapack_int, a: *const lapack_complex_float, lda: lapack_int,
                          rcond: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_ztrcon(matrix_layout: c_int, norm: c_char, uplo: c_char, diag: c_char,
                          n: lapack_int, a: *const lapack_complex_double, lda: lapack_int,
                          rcond: *mut c_double)
                          -> lapack_int;

    pub fn LAPACKE_strevc(matrix_layout: c_int, side: c_char, howmny: c_char,
                          select: *mut lapack_logical, n: lapack_int, t: *const c_float,
                          ldt: lapack_int, vl: *mut c_float, ldvl: lapack_int, vr: *mut c_float,
                          ldvr: lapack_int, mm: lapack_int, m: *mut lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_dtrevc(matrix_layout: c_int, side: c_char, howmny: c_char,
                          select: *mut lapack_logical, n: lapack_int, t: *const c_double,
                          ldt: lapack_int, vl: *mut c_double, ldvl: lapack_int, vr: *mut c_double,
                          ldvr: lapack_int, mm: lapack_int, m: *mut lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_ctrevc(matrix_layout: c_int, side: c_char, howmny: c_char,
                          select: *const lapack_logical, n: lapack_int,
                          t: *mut lapack_complex_float, ldt: lapack_int,
                          vl: *mut lapack_complex_float, ldvl: lapack_int,
                          vr: *mut lapack_complex_float, ldvr: lapack_int, mm: lapack_int,
                          m: *mut lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_ztrevc(matrix_layout: c_int, side: c_char, howmny: c_char,
                          select: *const lapack_logical, n: lapack_int,
                          t: *mut lapack_complex_double, ldt: lapack_int,
                          vl: *mut lapack_complex_double, ldvl: lapack_int,
                          vr: *mut lapack_complex_double, ldvr: lapack_int, mm: lapack_int,
                          m: *mut lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_strexc(matrix_layout: c_int, compq: c_char, n: lapack_int, t: *mut c_float,
                          ldt: lapack_int, q: *mut c_float, ldq: lapack_int, ifst: *mut lapack_int,
                          ilst: *mut lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_dtrexc(matrix_layout: c_int, compq: c_char, n: lapack_int, t: *mut c_double,
                          ldt: lapack_int, q: *mut c_double, ldq: lapack_int,
                          ifst: *mut lapack_int, ilst: *mut lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_ctrexc(matrix_layout: c_int, compq: c_char, n: lapack_int,
                          t: *mut lapack_complex_float, ldt: lapack_int,
                          q: *mut lapack_complex_float, ldq: lapack_int, ifst: lapack_int,
                          ilst: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_ztrexc(matrix_layout: c_int, compq: c_char, n: lapack_int,
                          t: *mut lapack_complex_double, ldt: lapack_int,
                          q: *mut lapack_complex_double, ldq: lapack_int, ifst: lapack_int,
                          ilst: lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_strrfs(matrix_layout: c_int, uplo: c_char, trans: c_char, diag: c_char,
                          n: lapack_int, nrhs: lapack_int, a: *const c_float, lda: lapack_int,
                          b: *const c_float, ldb: lapack_int, x: *const c_float, ldx: lapack_int,
                          ferr: *mut c_float, berr: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_dtrrfs(matrix_layout: c_int, uplo: c_char, trans: c_char, diag: c_char,
                          n: lapack_int, nrhs: lapack_int, a: *const c_double, lda: lapack_int,
                          b: *const c_double, ldb: lapack_int, x: *const c_double, ldx: lapack_int,
                          ferr: *mut c_double, berr: *mut c_double)
                          -> lapack_int;
    pub fn LAPACKE_ctrrfs(matrix_layout: c_int, uplo: c_char, trans: c_char, diag: c_char,
                          n: lapack_int, nrhs: lapack_int, a: *const lapack_complex_float,
                          lda: lapack_int, b: *const lapack_complex_float, ldb: lapack_int,
                          x: *const lapack_complex_float, ldx: lapack_int, ferr: *mut c_float,
                          berr: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_ztrrfs(matrix_layout: c_int, uplo: c_char, trans: c_char, diag: c_char,
                          n: lapack_int, nrhs: lapack_int, a: *const lapack_complex_double,
                          lda: lapack_int, b: *const lapack_complex_double, ldb: lapack_int,
                          x: *const lapack_complex_double, ldx: lapack_int, ferr: *mut c_double,
                          berr: *mut c_double)
                          -> lapack_int;

    pub fn LAPACKE_strsen(matrix_layout: c_int, job: c_char, compq: c_char,
                          select: *const lapack_logical, n: lapack_int, t: *mut c_float,
                          ldt: lapack_int, q: *mut c_float, ldq: lapack_int, wr: *mut c_float,
                          wi: *mut c_float, m: *mut lapack_int, s: *mut c_float, sep: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_dtrsen(matrix_layout: c_int, job: c_char, compq: c_char,
                          select: *const lapack_logical, n: lapack_int, t: *mut c_double,
                          ldt: lapack_int, q: *mut c_double, ldq: lapack_int, wr: *mut c_double,
                          wi: *mut c_double, m: *mut lapack_int, s: *mut c_double,
                          sep: *mut c_double)
                          -> lapack_int;
    pub fn LAPACKE_ctrsen(matrix_layout: c_int, job: c_char, compq: c_char,
                          select: *const lapack_logical, n: lapack_int,
                          t: *mut lapack_complex_float, ldt: lapack_int,
                          q: *mut lapack_complex_float, ldq: lapack_int,
                          w: *mut lapack_complex_float, m: *mut lapack_int, s: *mut c_float,
                          sep: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_ztrsen(matrix_layout: c_int, job: c_char, compq: c_char,
                          select: *const lapack_logical, n: lapack_int,
                          t: *mut lapack_complex_double, ldt: lapack_int,
                          q: *mut lapack_complex_double, ldq: lapack_int,
                          w: *mut lapack_complex_double, m: *mut lapack_int, s: *mut c_double,
                          sep: *mut c_double)
                          -> lapack_int;

    pub fn LAPACKE_strsna(matrix_layout: c_int, job: c_char, howmny: c_char,
                          select: *const lapack_logical, n: lapack_int, t: *const c_float,
                          ldt: lapack_int, vl: *const c_float, ldvl: lapack_int,
                          vr: *const c_float, ldvr: lapack_int, s: *mut c_float, sep: *mut c_float,
                          mm: lapack_int, m: *mut lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_dtrsna(matrix_layout: c_int, job: c_char, howmny: c_char,
                          select: *const lapack_logical, n: lapack_int, t: *const c_double,
                          ldt: lapack_int, vl: *const c_double, ldvl: lapack_int,
                          vr: *const c_double, ldvr: lapack_int, s: *mut c_double,
                          sep: *mut c_double, mm: lapack_int, m: *mut lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_ctrsna(matrix_layout: c_int, job: c_char, howmny: c_char,
                          select: *const lapack_logical, n: lapack_int,
                          t: *const lapack_complex_float, ldt: lapack_int,
                          vl: *const lapack_complex_float, ldvl: lapack_int,
                          vr: *const lapack_complex_float, ldvr: lapack_int, s: *mut c_float,
                          sep: *mut c_float, mm: lapack_int, m: *mut lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_ztrsna(matrix_layout: c_int, job: c_char, howmny: c_char,
                          select: *const lapack_logical, n: lapack_int,
                          t: *const lapack_complex_double, ldt: lapack_int,
                          vl: *const lapack_complex_double, ldvl: lapack_int,
                          vr: *const lapack_complex_double, ldvr: lapack_int, s: *mut c_double,
                          sep: *mut c_double, mm: lapack_int, m: *mut lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_strsyl(matrix_layout: c_int, trana: c_char, tranb: c_char, isgn: lapack_int,
                          m: lapack_int, n: lapack_int, a: *const c_float, lda: lapack_int,
                          b: *const c_float, ldb: lapack_int, c: *mut c_float, ldc: lapack_int,
                          scale: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_dtrsyl(matrix_layout: c_int, trana: c_char, tranb: c_char, isgn: lapack_int,
                          m: lapack_int, n: lapack_int, a: *const c_double, lda: lapack_int,
                          b: *const c_double, ldb: lapack_int, c: *mut c_double, ldc: lapack_int,
                          scale: *mut c_double)
                          -> lapack_int;
    pub fn LAPACKE_ctrsyl(matrix_layout: c_int, trana: c_char, tranb: c_char, isgn: lapack_int,
                          m: lapack_int, n: lapack_int, a: *const lapack_complex_float,
                          lda: lapack_int, b: *const lapack_complex_float, ldb: lapack_int,
                          c: *mut lapack_complex_float, ldc: lapack_int, scale: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_ztrsyl(matrix_layout: c_int, trana: c_char, tranb: c_char, isgn: lapack_int,
                          m: lapack_int, n: lapack_int, a: *const lapack_complex_double,
                          lda: lapack_int, b: *const lapack_complex_double, ldb: lapack_int,
                          c: *mut lapack_complex_double, ldc: lapack_int, scale: *mut c_double)
                          -> lapack_int;

    pub fn LAPACKE_strtri(matrix_layout: c_int, uplo: c_char, diag: c_char, n: lapack_int,
                          a: *mut c_float, lda: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_dtrtri(matrix_layout: c_int, uplo: c_char, diag: c_char, n: lapack_int,
                          a: *mut c_double, lda: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_ctrtri(matrix_layout: c_int, uplo: c_char, diag: c_char, n: lapack_int,
                          a: *mut lapack_complex_float, lda: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_ztrtri(matrix_layout: c_int, uplo: c_char, diag: c_char, n: lapack_int,
                          a: *mut lapack_complex_double, lda: lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_strtrs(matrix_layout: c_int, uplo: c_char, trans: c_char, diag: c_char,
                          n: lapack_int, nrhs: lapack_int, a: *const c_float, lda: lapack_int,
                          b: *mut c_float, ldb: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_dtrtrs(matrix_layout: c_int, uplo: c_char, trans: c_char, diag: c_char,
                          n: lapack_int, nrhs: lapack_int, a: *const c_double, lda: lapack_int,
                          b: *mut c_double, ldb: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_ctrtrs(matrix_layout: c_int, uplo: c_char, trans: c_char, diag: c_char,
                          n: lapack_int, nrhs: lapack_int, a: *const lapack_complex_float,
                          lda: lapack_int, b: *mut lapack_complex_float, ldb: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_ztrtrs(matrix_layout: c_int, uplo: c_char, trans: c_char, diag: c_char,
                          n: lapack_int, nrhs: lapack_int, a: *const lapack_complex_double,
                          lda: lapack_int, b: *mut lapack_complex_double, ldb: lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_strttf(matrix_layout: c_int, transr: c_char, uplo: c_char, n: lapack_int,
                          a: *const c_float, lda: lapack_int, arf: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_dtrttf(matrix_layout: c_int, transr: c_char, uplo: c_char, n: lapack_int,
                          a: *const c_double, lda: lapack_int, arf: *mut c_double)
                          -> lapack_int;
    pub fn LAPACKE_ctrttf(matrix_layout: c_int, transr: c_char, uplo: c_char, n: lapack_int,
                          a: *const lapack_complex_float, lda: lapack_int,
                          arf: *mut lapack_complex_float)
                          -> lapack_int;
    pub fn LAPACKE_ztrttf(matrix_layout: c_int, transr: c_char, uplo: c_char, n: lapack_int,
                          a: *const lapack_complex_double, lda: lapack_int,
                          arf: *mut lapack_complex_double)
                          -> lapack_int;

    pub fn LAPACKE_strttp(matrix_layout: c_int, uplo: c_char, n: lapack_int, a: *const c_float,
                          lda: lapack_int, ap: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_dtrttp(matrix_layout: c_int, uplo: c_char, n: lapack_int, a: *const c_double,
                          lda: lapack_int, ap: *mut c_double)
                          -> lapack_int;
    pub fn LAPACKE_ctrttp(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                          a: *const lapack_complex_float, lda: lapack_int,
                          ap: *mut lapack_complex_float)
                          -> lapack_int;
    pub fn LAPACKE_ztrttp(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                          a: *const lapack_complex_double, lda: lapack_int,
                          ap: *mut lapack_complex_double)
                          -> lapack_int;

    pub fn LAPACKE_stzrzf(matrix_layout: c_int, m: lapack_int, n: lapack_int, a: *mut c_float,
                          lda: lapack_int, tau: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_dtzrzf(matrix_layout: c_int, m: lapack_int, n: lapack_int, a: *mut c_double,
                          lda: lapack_int, tau: *mut c_double)
                          -> lapack_int;
    pub fn LAPACKE_ctzrzf(matrix_layout: c_int, m: lapack_int, n: lapack_int,
                          a: *mut lapack_complex_float, lda: lapack_int,
                          tau: *mut lapack_complex_float)
                          -> lapack_int;
    pub fn LAPACKE_ztzrzf(matrix_layout: c_int, m: lapack_int, n: lapack_int,
                          a: *mut lapack_complex_double, lda: lapack_int,
                          tau: *mut lapack_complex_double)
                          -> lapack_int;

    pub fn LAPACKE_cungbr(matrix_layout: c_int, vect: c_char, m: lapack_int, n: lapack_int,
                          k: lapack_int, a: *mut lapack_complex_float, lda: lapack_int,
                          tau: *const lapack_complex_float)
                          -> lapack_int;
    pub fn LAPACKE_zungbr(matrix_layout: c_int, vect: c_char, m: lapack_int, n: lapack_int,
                          k: lapack_int, a: *mut lapack_complex_double, lda: lapack_int,
                          tau: *const lapack_complex_double)
                          -> lapack_int;

    pub fn LAPACKE_cunghr(matrix_layout: c_int, n: lapack_int, ilo: lapack_int, ihi: lapack_int,
                          a: *mut lapack_complex_float, lda: lapack_int,
                          tau: *const lapack_complex_float)
                          -> lapack_int;
    pub fn LAPACKE_zunghr(matrix_layout: c_int, n: lapack_int, ilo: lapack_int, ihi: lapack_int,
                          a: *mut lapack_complex_double, lda: lapack_int,
                          tau: *const lapack_complex_double)
                          -> lapack_int;

    pub fn LAPACKE_cunglq(matrix_layout: c_int, m: lapack_int, n: lapack_int, k: lapack_int,
                          a: *mut lapack_complex_float, lda: lapack_int,
                          tau: *const lapack_complex_float)
                          -> lapack_int;
    pub fn LAPACKE_zunglq(matrix_layout: c_int, m: lapack_int, n: lapack_int, k: lapack_int,
                          a: *mut lapack_complex_double, lda: lapack_int,
                          tau: *const lapack_complex_double)
                          -> lapack_int;

    pub fn LAPACKE_cungql(matrix_layout: c_int, m: lapack_int, n: lapack_int, k: lapack_int,
                          a: *mut lapack_complex_float, lda: lapack_int,
                          tau: *const lapack_complex_float)
                          -> lapack_int;
    pub fn LAPACKE_zungql(matrix_layout: c_int, m: lapack_int, n: lapack_int, k: lapack_int,
                          a: *mut lapack_complex_double, lda: lapack_int,
                          tau: *const lapack_complex_double)
                          -> lapack_int;

    pub fn LAPACKE_cungqr(matrix_layout: c_int, m: lapack_int, n: lapack_int, k: lapack_int,
                          a: *mut lapack_complex_float, lda: lapack_int,
                          tau: *const lapack_complex_float)
                          -> lapack_int;
    pub fn LAPACKE_zungqr(matrix_layout: c_int, m: lapack_int, n: lapack_int, k: lapack_int,
                          a: *mut lapack_complex_double, lda: lapack_int,
                          tau: *const lapack_complex_double)
                          -> lapack_int;

    pub fn LAPACKE_cungrq(matrix_layout: c_int, m: lapack_int, n: lapack_int, k: lapack_int,
                          a: *mut lapack_complex_float, lda: lapack_int,
                          tau: *const lapack_complex_float)
                          -> lapack_int;
    pub fn LAPACKE_zungrq(matrix_layout: c_int, m: lapack_int, n: lapack_int, k: lapack_int,
                          a: *mut lapack_complex_double, lda: lapack_int,
                          tau: *const lapack_complex_double)
                          -> lapack_int;

    pub fn LAPACKE_cungtr(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                          a: *mut lapack_complex_float, lda: lapack_int,
                          tau: *const lapack_complex_float)
                          -> lapack_int;
    pub fn LAPACKE_zungtr(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                          a: *mut lapack_complex_double, lda: lapack_int,
                          tau: *const lapack_complex_double)
                          -> lapack_int;

    pub fn LAPACKE_cunmbr(matrix_layout: c_int, vect: c_char, side: c_char, trans: c_char,
                          m: lapack_int, n: lapack_int, k: lapack_int,
                          a: *const lapack_complex_float, lda: lapack_int,
                          tau: *const lapack_complex_float, c: *mut lapack_complex_float,
                          ldc: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_zunmbr(matrix_layout: c_int, vect: c_char, side: c_char, trans: c_char,
                          m: lapack_int, n: lapack_int, k: lapack_int,
                          a: *const lapack_complex_double, lda: lapack_int,
                          tau: *const lapack_complex_double, c: *mut lapack_complex_double,
                          ldc: lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_cunmhr(matrix_layout: c_int, side: c_char, trans: c_char, m: lapack_int,
                          n: lapack_int, ilo: lapack_int, ihi: lapack_int,
                          a: *const lapack_complex_float, lda: lapack_int,
                          tau: *const lapack_complex_float, c: *mut lapack_complex_float,
                          ldc: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_zunmhr(matrix_layout: c_int, side: c_char, trans: c_char, m: lapack_int,
                          n: lapack_int, ilo: lapack_int, ihi: lapack_int,
                          a: *const lapack_complex_double, lda: lapack_int,
                          tau: *const lapack_complex_double, c: *mut lapack_complex_double,
                          ldc: lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_cunmlq(matrix_layout: c_int, side: c_char, trans: c_char, m: lapack_int,
                          n: lapack_int, k: lapack_int, a: *const lapack_complex_float,
                          lda: lapack_int, tau: *const lapack_complex_float,
                          c: *mut lapack_complex_float, ldc: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_zunmlq(matrix_layout: c_int, side: c_char, trans: c_char, m: lapack_int,
                          n: lapack_int, k: lapack_int, a: *const lapack_complex_double,
                          lda: lapack_int, tau: *const lapack_complex_double,
                          c: *mut lapack_complex_double, ldc: lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_cunmql(matrix_layout: c_int, side: c_char, trans: c_char, m: lapack_int,
                          n: lapack_int, k: lapack_int, a: *const lapack_complex_float,
                          lda: lapack_int, tau: *const lapack_complex_float,
                          c: *mut lapack_complex_float, ldc: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_zunmql(matrix_layout: c_int, side: c_char, trans: c_char, m: lapack_int,
                          n: lapack_int, k: lapack_int, a: *const lapack_complex_double,
                          lda: lapack_int, tau: *const lapack_complex_double,
                          c: *mut lapack_complex_double, ldc: lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_cunmqr(matrix_layout: c_int, side: c_char, trans: c_char, m: lapack_int,
                          n: lapack_int, k: lapack_int, a: *const lapack_complex_float,
                          lda: lapack_int, tau: *const lapack_complex_float,
                          c: *mut lapack_complex_float, ldc: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_zunmqr(matrix_layout: c_int, side: c_char, trans: c_char, m: lapack_int,
                          n: lapack_int, k: lapack_int, a: *const lapack_complex_double,
                          lda: lapack_int, tau: *const lapack_complex_double,
                          c: *mut lapack_complex_double, ldc: lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_cunmrq(matrix_layout: c_int, side: c_char, trans: c_char, m: lapack_int,
                          n: lapack_int, k: lapack_int, a: *const lapack_complex_float,
                          lda: lapack_int, tau: *const lapack_complex_float,
                          c: *mut lapack_complex_float, ldc: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_zunmrq(matrix_layout: c_int, side: c_char, trans: c_char, m: lapack_int,
                          n: lapack_int, k: lapack_int, a: *const lapack_complex_double,
                          lda: lapack_int, tau: *const lapack_complex_double,
                          c: *mut lapack_complex_double, ldc: lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_cunmrz(matrix_layout: c_int, side: c_char, trans: c_char, m: lapack_int,
                          n: lapack_int, k: lapack_int, l: lapack_int,
                          a: *const lapack_complex_float, lda: lapack_int,
                          tau: *const lapack_complex_float, c: *mut lapack_complex_float,
                          ldc: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_zunmrz(matrix_layout: c_int, side: c_char, trans: c_char, m: lapack_int,
                          n: lapack_int, k: lapack_int, l: lapack_int,
                          a: *const lapack_complex_double, lda: lapack_int,
                          tau: *const lapack_complex_double, c: *mut lapack_complex_double,
                          ldc: lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_cunmtr(matrix_layout: c_int, side: c_char, uplo: c_char, trans: c_char,
                          m: lapack_int, n: lapack_int, a: *const lapack_complex_float,
                          lda: lapack_int, tau: *const lapack_complex_float,
                          c: *mut lapack_complex_float, ldc: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_zunmtr(matrix_layout: c_int, side: c_char, uplo: c_char, trans: c_char,
                          m: lapack_int, n: lapack_int, a: *const lapack_complex_double,
                          lda: lapack_int, tau: *const lapack_complex_double,
                          c: *mut lapack_complex_double, ldc: lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_cupgtr(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                          ap: *const lapack_complex_float, tau: *const lapack_complex_float,
                          q: *mut lapack_complex_float, ldq: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_zupgtr(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                          ap: *const lapack_complex_double, tau: *const lapack_complex_double,
                          q: *mut lapack_complex_double, ldq: lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_cupmtr(matrix_layout: c_int, side: c_char, uplo: c_char, trans: c_char,
                          m: lapack_int, n: lapack_int, ap: *const lapack_complex_float,
                          tau: *const lapack_complex_float, c: *mut lapack_complex_float,
                          ldc: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_zupmtr(matrix_layout: c_int, side: c_char, uplo: c_char, trans: c_char,
                          m: lapack_int, n: lapack_int, ap: *const lapack_complex_double,
                          tau: *const lapack_complex_double, c: *mut lapack_complex_double,
                          ldc: lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_sbdsdc_work(matrix_layout: c_int, uplo: c_char, compq: c_char, n: lapack_int,
                               d: *mut c_float, e: *mut c_float, u: *mut c_float, ldu: lapack_int,
                               vt: *mut c_float, ldvt: lapack_int, q: *mut c_float,
                               iq: *mut lapack_int, work: *mut c_float, iwork: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dbdsdc_work(matrix_layout: c_int, uplo: c_char, compq: c_char, n: lapack_int,
                               d: *mut c_double, e: *mut c_double, u: *mut c_double,
                               ldu: lapack_int, vt: *mut c_double, ldvt: lapack_int,
                               q: *mut c_double, iq: *mut lapack_int, work: *mut c_double,
                               iwork: *mut lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_sbdsvdx_work(matrix_layout: c_int, uplo: c_char, jobz: c_char, range: c_char,
                                n: lapack_int, d: *mut c_float, e: *mut c_float, vl: c_float,
                                vu: c_float, il: lapack_int, iu: lapack_int, ns: lapack_int,
                                s: *mut c_float, z: *mut c_float, ldz: lapack_int,
                                work: *mut c_float, iwork: *mut lapack_int)
                                -> lapack_int;
    pub fn LAPACKE_dbdsvdx_work(matrix_layout: c_int, uplo: c_char, jobz: c_char, range: c_char,
                                n: lapack_int, d: *mut c_double, e: *mut c_double, vl: c_double,
                                vu: c_double, il: lapack_int, iu: lapack_int, ns: lapack_int,
                                s: *mut c_double, z: *mut c_double, ldz: lapack_int,
                                work: *mut c_double, iwork: *mut lapack_int)
                                -> lapack_int;

    pub fn LAPACKE_sbdsqr_work(matrix_layout: c_int, uplo: c_char, n: lapack_int, ncvt: lapack_int,
                               nru: lapack_int, ncc: lapack_int, d: *mut c_float, e: *mut c_float,
                               vt: *mut c_float, ldvt: lapack_int, u: *mut c_float,
                               ldu: lapack_int, c: *mut c_float, ldc: lapack_int,
                               work: *mut c_float)
                               -> lapack_int;
    pub fn LAPACKE_dbdsqr_work(matrix_layout: c_int, uplo: c_char, n: lapack_int, ncvt: lapack_int,
                               nru: lapack_int, ncc: lapack_int, d: *mut c_double,
                               e: *mut c_double, vt: *mut c_double, ldvt: lapack_int,
                               u: *mut c_double, ldu: lapack_int, c: *mut c_double,
                               ldc: lapack_int, work: *mut c_double)
                               -> lapack_int;
    pub fn LAPACKE_cbdsqr_work(matrix_layout: c_int, uplo: c_char, n: lapack_int, ncvt: lapack_int,
                               nru: lapack_int, ncc: lapack_int, d: *mut c_float, e: *mut c_float,
                               vt: *mut lapack_complex_float, ldvt: lapack_int,
                               u: *mut lapack_complex_float, ldu: lapack_int,
                               c: *mut lapack_complex_float, ldc: lapack_int, work: *mut c_float)
                               -> lapack_int;
    pub fn LAPACKE_zbdsqr_work(matrix_layout: c_int, uplo: c_char, n: lapack_int, ncvt: lapack_int,
                               nru: lapack_int, ncc: lapack_int, d: *mut c_double,
                               e: *mut c_double, vt: *mut lapack_complex_double, ldvt: lapack_int,
                               u: *mut lapack_complex_double, ldu: lapack_int,
                               c: *mut lapack_complex_double, ldc: lapack_int, work: *mut c_double)
                               -> lapack_int;

    pub fn LAPACKE_sdisna_work(job: c_char, m: lapack_int, n: lapack_int, d: *const c_float,
                               sep: *mut c_float)
                               -> lapack_int;
    pub fn LAPACKE_ddisna_work(job: c_char, m: lapack_int, n: lapack_int, d: *const c_double,
                               sep: *mut c_double)
                               -> lapack_int;

    pub fn LAPACKE_sgbbrd_work(matrix_layout: c_int, vect: c_char, m: lapack_int, n: lapack_int,
                               ncc: lapack_int, kl: lapack_int, ku: lapack_int, ab: *mut c_float,
                               ldab: lapack_int, d: *mut c_float, e: *mut c_float, q: *mut c_float,
                               ldq: lapack_int, pt: *mut c_float, ldpt: lapack_int,
                               c: *mut c_float, ldc: lapack_int, work: *mut c_float)
                               -> lapack_int;
    pub fn LAPACKE_dgbbrd_work(matrix_layout: c_int, vect: c_char, m: lapack_int, n: lapack_int,
                               ncc: lapack_int, kl: lapack_int, ku: lapack_int, ab: *mut c_double,
                               ldab: lapack_int, d: *mut c_double, e: *mut c_double,
                               q: *mut c_double, ldq: lapack_int, pt: *mut c_double,
                               ldpt: lapack_int, c: *mut c_double, ldc: lapack_int,
                               work: *mut c_double)
                               -> lapack_int;
    pub fn LAPACKE_cgbbrd_work(matrix_layout: c_int, vect: c_char, m: lapack_int, n: lapack_int,
                               ncc: lapack_int, kl: lapack_int, ku: lapack_int,
                               ab: *mut lapack_complex_float, ldab: lapack_int, d: *mut c_float,
                               e: *mut c_float, q: *mut lapack_complex_float, ldq: lapack_int,
                               pt: *mut lapack_complex_float, ldpt: lapack_int,
                               c: *mut lapack_complex_float, ldc: lapack_int,
                               work: *mut lapack_complex_float, rwork: *mut c_float)
                               -> lapack_int;
    pub fn LAPACKE_zgbbrd_work(matrix_layout: c_int, vect: c_char, m: lapack_int, n: lapack_int,
                               ncc: lapack_int, kl: lapack_int, ku: lapack_int,
                               ab: *mut lapack_complex_double, ldab: lapack_int, d: *mut c_double,
                               e: *mut c_double, q: *mut lapack_complex_double, ldq: lapack_int,
                               pt: *mut lapack_complex_double, ldpt: lapack_int,
                               c: *mut lapack_complex_double, ldc: lapack_int,
                               work: *mut lapack_complex_double, rwork: *mut c_double)
                               -> lapack_int;

    pub fn LAPACKE_sgbcon_work(matrix_layout: c_int, norm: c_char, n: lapack_int, kl: lapack_int,
                               ku: lapack_int, ab: *const c_float, ldab: lapack_int,
                               ipiv: *const lapack_int, anorm: c_float, rcond: *mut c_float,
                               work: *mut c_float, iwork: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dgbcon_work(matrix_layout: c_int, norm: c_char, n: lapack_int, kl: lapack_int,
                               ku: lapack_int, ab: *const c_double, ldab: lapack_int,
                               ipiv: *const lapack_int, anorm: c_double, rcond: *mut c_double,
                               work: *mut c_double, iwork: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_cgbcon_work(matrix_layout: c_int, norm: c_char, n: lapack_int, kl: lapack_int,
                               ku: lapack_int, ab: *const lapack_complex_float, ldab: lapack_int,
                               ipiv: *const lapack_int, anorm: c_float, rcond: *mut c_float,
                               work: *mut lapack_complex_float, rwork: *mut c_float)
                               -> lapack_int;
    pub fn LAPACKE_zgbcon_work(matrix_layout: c_int, norm: c_char, n: lapack_int, kl: lapack_int,
                               ku: lapack_int, ab: *const lapack_complex_double, ldab: lapack_int,
                               ipiv: *const lapack_int, anorm: c_double, rcond: *mut c_double,
                               work: *mut lapack_complex_double, rwork: *mut c_double)
                               -> lapack_int;

    pub fn LAPACKE_sgbequ_work(matrix_layout: c_int, m: lapack_int, n: lapack_int, kl: lapack_int,
                               ku: lapack_int, ab: *const c_float, ldab: lapack_int,
                               r: *mut c_float, c: *mut c_float, rowcnd: *mut c_float,
                               colcnd: *mut c_float, amax: *mut c_float)
                               -> lapack_int;
    pub fn LAPACKE_dgbequ_work(matrix_layout: c_int, m: lapack_int, n: lapack_int, kl: lapack_int,
                               ku: lapack_int, ab: *const c_double, ldab: lapack_int,
                               r: *mut c_double, c: *mut c_double, rowcnd: *mut c_double,
                               colcnd: *mut c_double, amax: *mut c_double)
                               -> lapack_int;
    pub fn LAPACKE_cgbequ_work(matrix_layout: c_int, m: lapack_int, n: lapack_int, kl: lapack_int,
                               ku: lapack_int, ab: *const lapack_complex_float, ldab: lapack_int,
                               r: *mut c_float, c: *mut c_float, rowcnd: *mut c_float,
                               colcnd: *mut c_float, amax: *mut c_float)
                               -> lapack_int;
    pub fn LAPACKE_zgbequ_work(matrix_layout: c_int, m: lapack_int, n: lapack_int, kl: lapack_int,
                               ku: lapack_int, ab: *const lapack_complex_double, ldab: lapack_int,
                               r: *mut c_double, c: *mut c_double, rowcnd: *mut c_double,
                               colcnd: *mut c_double, amax: *mut c_double)
                               -> lapack_int;

    pub fn LAPACKE_sgbequb_work(matrix_layout: c_int, m: lapack_int, n: lapack_int, kl: lapack_int,
                                ku: lapack_int, ab: *const c_float, ldab: lapack_int,
                                r: *mut c_float, c: *mut c_float, rowcnd: *mut c_float,
                                colcnd: *mut c_float, amax: *mut c_float)
                                -> lapack_int;
    pub fn LAPACKE_dgbequb_work(matrix_layout: c_int, m: lapack_int, n: lapack_int, kl: lapack_int,
                                ku: lapack_int, ab: *const c_double, ldab: lapack_int,
                                r: *mut c_double, c: *mut c_double, rowcnd: *mut c_double,
                                colcnd: *mut c_double, amax: *mut c_double)
                                -> lapack_int;
    pub fn LAPACKE_cgbequb_work(matrix_layout: c_int, m: lapack_int, n: lapack_int, kl: lapack_int,
                                ku: lapack_int, ab: *const lapack_complex_float, ldab: lapack_int,
                                r: *mut c_float, c: *mut c_float, rowcnd: *mut c_float,
                                colcnd: *mut c_float, amax: *mut c_float)
                                -> lapack_int;
    pub fn LAPACKE_zgbequb_work(matrix_layout: c_int, m: lapack_int, n: lapack_int, kl: lapack_int,
                                ku: lapack_int, ab: *const lapack_complex_double, ldab: lapack_int,
                                r: *mut c_double, c: *mut c_double, rowcnd: *mut c_double,
                                colcnd: *mut c_double, amax: *mut c_double)
                                -> lapack_int;

    pub fn LAPACKE_sgbrfs_work(matrix_layout: c_int, trans: c_char, n: lapack_int, kl: lapack_int,
                               ku: lapack_int, nrhs: lapack_int, ab: *const c_float,
                               ldab: lapack_int, afb: *const c_float, ldafb: lapack_int,
                               ipiv: *const lapack_int, b: *const c_float, ldb: lapack_int,
                               x: *mut c_float, ldx: lapack_int, ferr: *mut c_float,
                               berr: *mut c_float, work: *mut c_float, iwork: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dgbrfs_work(matrix_layout: c_int, trans: c_char, n: lapack_int, kl: lapack_int,
                               ku: lapack_int, nrhs: lapack_int, ab: *const c_double,
                               ldab: lapack_int, afb: *const c_double, ldafb: lapack_int,
                               ipiv: *const lapack_int, b: *const c_double, ldb: lapack_int,
                               x: *mut c_double, ldx: lapack_int, ferr: *mut c_double,
                               berr: *mut c_double, work: *mut c_double, iwork: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_cgbrfs_work(matrix_layout: c_int, trans: c_char, n: lapack_int, kl: lapack_int,
                               ku: lapack_int, nrhs: lapack_int, ab: *const lapack_complex_float,
                               ldab: lapack_int, afb: *const lapack_complex_float,
                               ldafb: lapack_int, ipiv: *const lapack_int,
                               b: *const lapack_complex_float, ldb: lapack_int,
                               x: *mut lapack_complex_float, ldx: lapack_int, ferr: *mut c_float,
                               berr: *mut c_float, work: *mut lapack_complex_float,
                               rwork: *mut c_float)
                               -> lapack_int;
    pub fn LAPACKE_zgbrfs_work(matrix_layout: c_int, trans: c_char, n: lapack_int, kl: lapack_int,
                               ku: lapack_int, nrhs: lapack_int, ab: *const lapack_complex_double,
                               ldab: lapack_int, afb: *const lapack_complex_double,
                               ldafb: lapack_int, ipiv: *const lapack_int,
                               b: *const lapack_complex_double, ldb: lapack_int,
                               x: *mut lapack_complex_double, ldx: lapack_int, ferr: *mut c_double,
                               berr: *mut c_double, work: *mut lapack_complex_double,
                               rwork: *mut c_double)
                               -> lapack_int;

    pub fn LAPACKE_sgbrfsx_work(matrix_layout: c_int, trans: c_char, equed: c_char, n: lapack_int,
                                kl: lapack_int, ku: lapack_int, nrhs: lapack_int,
                                ab: *const c_float, ldab: lapack_int, afb: *const c_float,
                                ldafb: lapack_int, ipiv: *const lapack_int, r: *const c_float,
                                c: *const c_float, b: *const c_float, ldb: lapack_int,
                                x: *mut c_float, ldx: lapack_int, rcond: *mut c_float,
                                berr: *mut c_float, n_err_bnds: lapack_int,
                                err_bnds_norm: *mut c_float, err_bnds_comp: *mut c_float,
                                nparams: lapack_int, params: *mut c_float, work: *mut c_float,
                                iwork: *mut lapack_int)
                                -> lapack_int;
    pub fn LAPACKE_dgbrfsx_work(matrix_layout: c_int, trans: c_char, equed: c_char, n: lapack_int,
                                kl: lapack_int, ku: lapack_int, nrhs: lapack_int,
                                ab: *const c_double, ldab: lapack_int, afb: *const c_double,
                                ldafb: lapack_int, ipiv: *const lapack_int, r: *const c_double,
                                c: *const c_double, b: *const c_double, ldb: lapack_int,
                                x: *mut c_double, ldx: lapack_int, rcond: *mut c_double,
                                berr: *mut c_double, n_err_bnds: lapack_int,
                                err_bnds_norm: *mut c_double, err_bnds_comp: *mut c_double,
                                nparams: lapack_int, params: *mut c_double, work: *mut c_double,
                                iwork: *mut lapack_int)
                                -> lapack_int;
    pub fn LAPACKE_cgbrfsx_work(matrix_layout: c_int, trans: c_char, equed: c_char, n: lapack_int,
                                kl: lapack_int, ku: lapack_int, nrhs: lapack_int,
                                ab: *const lapack_complex_float, ldab: lapack_int,
                                afb: *const lapack_complex_float, ldafb: lapack_int,
                                ipiv: *const lapack_int, r: *const c_float, c: *const c_float,
                                b: *const lapack_complex_float, ldb: lapack_int,
                                x: *mut lapack_complex_float, ldx: lapack_int, rcond: *mut c_float,
                                berr: *mut c_float, n_err_bnds: lapack_int,
                                err_bnds_norm: *mut c_float, err_bnds_comp: *mut c_float,
                                nparams: lapack_int, params: *mut c_float,
                                work: *mut lapack_complex_float, rwork: *mut c_float)
                                -> lapack_int;
    pub fn LAPACKE_zgbrfsx_work(matrix_layout: c_int, trans: c_char, equed: c_char, n: lapack_int,
                                kl: lapack_int, ku: lapack_int, nrhs: lapack_int,
                                ab: *const lapack_complex_double, ldab: lapack_int,
                                afb: *const lapack_complex_double, ldafb: lapack_int,
                                ipiv: *const lapack_int, r: *const c_double, c: *const c_double,
                                b: *const lapack_complex_double, ldb: lapack_int,
                                x: *mut lapack_complex_double, ldx: lapack_int,
                                rcond: *mut c_double, berr: *mut c_double, n_err_bnds: lapack_int,
                                err_bnds_norm: *mut c_double, err_bnds_comp: *mut c_double,
                                nparams: lapack_int, params: *mut c_double,
                                work: *mut lapack_complex_double, rwork: *mut c_double)
                                -> lapack_int;

    pub fn LAPACKE_sgbsv_work(matrix_layout: c_int, n: lapack_int, kl: lapack_int, ku: lapack_int,
                              nrhs: lapack_int, ab: *mut c_float, ldab: lapack_int,
                              ipiv: *mut lapack_int, b: *mut c_float, ldb: lapack_int)
                              -> lapack_int;
    pub fn LAPACKE_dgbsv_work(matrix_layout: c_int, n: lapack_int, kl: lapack_int, ku: lapack_int,
                              nrhs: lapack_int, ab: *mut c_double, ldab: lapack_int,
                              ipiv: *mut lapack_int, b: *mut c_double, ldb: lapack_int)
                              -> lapack_int;
    pub fn LAPACKE_cgbsv_work(matrix_layout: c_int, n: lapack_int, kl: lapack_int, ku: lapack_int,
                              nrhs: lapack_int, ab: *mut lapack_complex_float, ldab: lapack_int,
                              ipiv: *mut lapack_int, b: *mut lapack_complex_float, ldb: lapack_int)
                              -> lapack_int;
    pub fn LAPACKE_zgbsv_work(matrix_layout: c_int, n: lapack_int, kl: lapack_int, ku: lapack_int,
                              nrhs: lapack_int, ab: *mut lapack_complex_double, ldab: lapack_int,
                              ipiv: *mut lapack_int, b: *mut lapack_complex_double,
                              ldb: lapack_int)
                              -> lapack_int;

    pub fn LAPACKE_sgbsvx_work(matrix_layout: c_int, fact: c_char, trans: c_char, n: lapack_int,
                               kl: lapack_int, ku: lapack_int, nrhs: lapack_int, ab: *mut c_float,
                               ldab: lapack_int, afb: *mut c_float, ldafb: lapack_int,
                               ipiv: *mut lapack_int, equed: *mut c_char, r: *mut c_float,
                               c: *mut c_float, b: *mut c_float, ldb: lapack_int, x: *mut c_float,
                               ldx: lapack_int, rcond: *mut c_float, ferr: *mut c_float,
                               berr: *mut c_float, work: *mut c_float, iwork: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dgbsvx_work(matrix_layout: c_int, fact: c_char, trans: c_char, n: lapack_int,
                               kl: lapack_int, ku: lapack_int, nrhs: lapack_int, ab: *mut c_double,
                               ldab: lapack_int, afb: *mut c_double, ldafb: lapack_int,
                               ipiv: *mut lapack_int, equed: *mut c_char, r: *mut c_double,
                               c: *mut c_double, b: *mut c_double, ldb: lapack_int,
                               x: *mut c_double, ldx: lapack_int, rcond: *mut c_double,
                               ferr: *mut c_double, berr: *mut c_double, work: *mut c_double,
                               iwork: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_cgbsvx_work(matrix_layout: c_int, fact: c_char, trans: c_char, n: lapack_int,
                               kl: lapack_int, ku: lapack_int, nrhs: lapack_int,
                               ab: *mut lapack_complex_float, ldab: lapack_int,
                               afb: *mut lapack_complex_float, ldafb: lapack_int,
                               ipiv: *mut lapack_int, equed: *mut c_char, r: *mut c_float,
                               c: *mut c_float, b: *mut lapack_complex_float, ldb: lapack_int,
                               x: *mut lapack_complex_float, ldx: lapack_int, rcond: *mut c_float,
                               ferr: *mut c_float, berr: *mut c_float,
                               work: *mut lapack_complex_float, rwork: *mut c_float)
                               -> lapack_int;
    pub fn LAPACKE_zgbsvx_work(matrix_layout: c_int, fact: c_char, trans: c_char, n: lapack_int,
                               kl: lapack_int, ku: lapack_int, nrhs: lapack_int,
                               ab: *mut lapack_complex_double, ldab: lapack_int,
                               afb: *mut lapack_complex_double, ldafb: lapack_int,
                               ipiv: *mut lapack_int, equed: *mut c_char, r: *mut c_double,
                               c: *mut c_double, b: *mut lapack_complex_double, ldb: lapack_int,
                               x: *mut lapack_complex_double, ldx: lapack_int,
                               rcond: *mut c_double, ferr: *mut c_double, berr: *mut c_double,
                               work: *mut lapack_complex_double, rwork: *mut c_double)
                               -> lapack_int;

    pub fn LAPACKE_sgbsvxx_work(matrix_layout: c_int, fact: c_char, trans: c_char, n: lapack_int,
                                kl: lapack_int, ku: lapack_int, nrhs: lapack_int, ab: *mut c_float,
                                ldab: lapack_int, afb: *mut c_float, ldafb: lapack_int,
                                ipiv: *mut lapack_int, equed: *mut c_char, r: *mut c_float,
                                c: *mut c_float, b: *mut c_float, ldb: lapack_int, x: *mut c_float,
                                ldx: lapack_int, rcond: *mut c_float, rpvgrw: *mut c_float,
                                berr: *mut c_float, n_err_bnds: lapack_int,
                                err_bnds_norm: *mut c_float, err_bnds_comp: *mut c_float,
                                nparams: lapack_int, params: *mut c_float, work: *mut c_float,
                                iwork: *mut lapack_int)
                                -> lapack_int;
    pub fn LAPACKE_dgbsvxx_work(matrix_layout: c_int, fact: c_char, trans: c_char, n: lapack_int,
                                kl: lapack_int, ku: lapack_int, nrhs: lapack_int,
                                ab: *mut c_double, ldab: lapack_int, afb: *mut c_double,
                                ldafb: lapack_int, ipiv: *mut lapack_int, equed: *mut c_char,
                                r: *mut c_double, c: *mut c_double, b: *mut c_double,
                                ldb: lapack_int, x: *mut c_double, ldx: lapack_int,
                                rcond: *mut c_double, rpvgrw: *mut c_double, berr: *mut c_double,
                                n_err_bnds: lapack_int, err_bnds_norm: *mut c_double,
                                err_bnds_comp: *mut c_double, nparams: lapack_int,
                                params: *mut c_double, work: *mut c_double, iwork: *mut lapack_int)
                                -> lapack_int;
    pub fn LAPACKE_cgbsvxx_work(matrix_layout: c_int, fact: c_char, trans: c_char, n: lapack_int,
                                kl: lapack_int, ku: lapack_int, nrhs: lapack_int,
                                ab: *mut lapack_complex_float, ldab: lapack_int,
                                afb: *mut lapack_complex_float, ldafb: lapack_int,
                                ipiv: *mut lapack_int, equed: *mut c_char, r: *mut c_float,
                                c: *mut c_float, b: *mut lapack_complex_float, ldb: lapack_int,
                                x: *mut lapack_complex_float, ldx: lapack_int, rcond: *mut c_float,
                                rpvgrw: *mut c_float, berr: *mut c_float, n_err_bnds: lapack_int,
                                err_bnds_norm: *mut c_float, err_bnds_comp: *mut c_float,
                                nparams: lapack_int, params: *mut c_float,
                                work: *mut lapack_complex_float, rwork: *mut c_float)
                                -> lapack_int;
    pub fn LAPACKE_zgbsvxx_work(matrix_layout: c_int, fact: c_char, trans: c_char, n: lapack_int,
                                kl: lapack_int, ku: lapack_int, nrhs: lapack_int,
                                ab: *mut lapack_complex_double, ldab: lapack_int,
                                afb: *mut lapack_complex_double, ldafb: lapack_int,
                                ipiv: *mut lapack_int, equed: *mut c_char, r: *mut c_double,
                                c: *mut c_double, b: *mut lapack_complex_double, ldb: lapack_int,
                                x: *mut lapack_complex_double, ldx: lapack_int,
                                rcond: *mut c_double, rpvgrw: *mut c_double, berr: *mut c_double,
                                n_err_bnds: lapack_int, err_bnds_norm: *mut c_double,
                                err_bnds_comp: *mut c_double, nparams: lapack_int,
                                params: *mut c_double, work: *mut lapack_complex_double,
                                rwork: *mut c_double)
                                -> lapack_int;

    pub fn LAPACKE_sgbtrf_work(matrix_layout: c_int, m: lapack_int, n: lapack_int, kl: lapack_int,
                               ku: lapack_int, ab: *mut c_float, ldab: lapack_int,
                               ipiv: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dgbtrf_work(matrix_layout: c_int, m: lapack_int, n: lapack_int, kl: lapack_int,
                               ku: lapack_int, ab: *mut c_double, ldab: lapack_int,
                               ipiv: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_cgbtrf_work(matrix_layout: c_int, m: lapack_int, n: lapack_int, kl: lapack_int,
                               ku: lapack_int, ab: *mut lapack_complex_float, ldab: lapack_int,
                               ipiv: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_zgbtrf_work(matrix_layout: c_int, m: lapack_int, n: lapack_int, kl: lapack_int,
                               ku: lapack_int, ab: *mut lapack_complex_double, ldab: lapack_int,
                               ipiv: *mut lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_sgbtrs_work(matrix_layout: c_int, trans: c_char, n: lapack_int, kl: lapack_int,
                               ku: lapack_int, nrhs: lapack_int, ab: *const c_float,
                               ldab: lapack_int, ipiv: *const lapack_int, b: *mut c_float,
                               ldb: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dgbtrs_work(matrix_layout: c_int, trans: c_char, n: lapack_int, kl: lapack_int,
                               ku: lapack_int, nrhs: lapack_int, ab: *const c_double,
                               ldab: lapack_int, ipiv: *const lapack_int, b: *mut c_double,
                               ldb: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_cgbtrs_work(matrix_layout: c_int, trans: c_char, n: lapack_int, kl: lapack_int,
                               ku: lapack_int, nrhs: lapack_int, ab: *const lapack_complex_float,
                               ldab: lapack_int, ipiv: *const lapack_int,
                               b: *mut lapack_complex_float, ldb: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_zgbtrs_work(matrix_layout: c_int, trans: c_char, n: lapack_int, kl: lapack_int,
                               ku: lapack_int, nrhs: lapack_int, ab: *const lapack_complex_double,
                               ldab: lapack_int, ipiv: *const lapack_int,
                               b: *mut lapack_complex_double, ldb: lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_sgebak_work(matrix_layout: c_int, job: c_char, side: c_char, n: lapack_int,
                               ilo: lapack_int, ihi: lapack_int, scale: *const c_float,
                               m: lapack_int, v: *mut c_float, ldv: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dgebak_work(matrix_layout: c_int, job: c_char, side: c_char, n: lapack_int,
                               ilo: lapack_int, ihi: lapack_int, scale: *const c_double,
                               m: lapack_int, v: *mut c_double, ldv: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_cgebak_work(matrix_layout: c_int, job: c_char, side: c_char, n: lapack_int,
                               ilo: lapack_int, ihi: lapack_int, scale: *const c_float,
                               m: lapack_int, v: *mut lapack_complex_float, ldv: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_zgebak_work(matrix_layout: c_int, job: c_char, side: c_char, n: lapack_int,
                               ilo: lapack_int, ihi: lapack_int, scale: *const c_double,
                               m: lapack_int, v: *mut lapack_complex_double, ldv: lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_sgebal_work(matrix_layout: c_int, job: c_char, n: lapack_int, a: *mut c_float,
                               lda: lapack_int, ilo: *mut lapack_int, ihi: *mut lapack_int,
                               scale: *mut c_float)
                               -> lapack_int;
    pub fn LAPACKE_dgebal_work(matrix_layout: c_int, job: c_char, n: lapack_int, a: *mut c_double,
                               lda: lapack_int, ilo: *mut lapack_int, ihi: *mut lapack_int,
                               scale: *mut c_double)
                               -> lapack_int;
    pub fn LAPACKE_cgebal_work(matrix_layout: c_int, job: c_char, n: lapack_int,
                               a: *mut lapack_complex_float, lda: lapack_int, ilo: *mut lapack_int,
                               ihi: *mut lapack_int, scale: *mut c_float)
                               -> lapack_int;
    pub fn LAPACKE_zgebal_work(matrix_layout: c_int, job: c_char, n: lapack_int,
                               a: *mut lapack_complex_double, lda: lapack_int,
                               ilo: *mut lapack_int, ihi: *mut lapack_int, scale: *mut c_double)
                               -> lapack_int;

    pub fn LAPACKE_sgebrd_work(matrix_layout: c_int, m: lapack_int, n: lapack_int, a: *mut c_float,
                               lda: lapack_int, d: *mut c_float, e: *mut c_float,
                               tauq: *mut c_float, taup: *mut c_float, work: *mut c_float,
                               lwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dgebrd_work(matrix_layout: c_int, m: lapack_int, n: lapack_int,
                               a: *mut c_double, lda: lapack_int, d: *mut c_double,
                               e: *mut c_double, tauq: *mut c_double, taup: *mut c_double,
                               work: *mut c_double, lwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_cgebrd_work(matrix_layout: c_int, m: lapack_int, n: lapack_int,
                               a: *mut lapack_complex_float, lda: lapack_int, d: *mut c_float,
                               e: *mut c_float, tauq: *mut lapack_complex_float,
                               taup: *mut lapack_complex_float, work: *mut lapack_complex_float,
                               lwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_zgebrd_work(matrix_layout: c_int, m: lapack_int, n: lapack_int,
                               a: *mut lapack_complex_double, lda: lapack_int, d: *mut c_double,
                               e: *mut c_double, tauq: *mut lapack_complex_double,
                               taup: *mut lapack_complex_double, work: *mut lapack_complex_double,
                               lwork: lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_sgecon_work(matrix_layout: c_int, norm: c_char, n: lapack_int,
                               a: *const c_float, lda: lapack_int, anorm: c_float,
                               rcond: *mut c_float, work: *mut c_float, iwork: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dgecon_work(matrix_layout: c_int, norm: c_char, n: lapack_int,
                               a: *const c_double, lda: lapack_int, anorm: c_double,
                               rcond: *mut c_double, work: *mut c_double, iwork: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_cgecon_work(matrix_layout: c_int, norm: c_char, n: lapack_int,
                               a: *const lapack_complex_float, lda: lapack_int, anorm: c_float,
                               rcond: *mut c_float, work: *mut lapack_complex_float,
                               rwork: *mut c_float)
                               -> lapack_int;
    pub fn LAPACKE_zgecon_work(matrix_layout: c_int, norm: c_char, n: lapack_int,
                               a: *const lapack_complex_double, lda: lapack_int, anorm: c_double,
                               rcond: *mut c_double, work: *mut lapack_complex_double,
                               rwork: *mut c_double)
                               -> lapack_int;

    pub fn LAPACKE_sgeequ_work(matrix_layout: c_int, m: lapack_int, n: lapack_int,
                               a: *const c_float, lda: lapack_int, r: *mut c_float,
                               c: *mut c_float, rowcnd: *mut c_float, colcnd: *mut c_float,
                               amax: *mut c_float)
                               -> lapack_int;
    pub fn LAPACKE_dgeequ_work(matrix_layout: c_int, m: lapack_int, n: lapack_int,
                               a: *const c_double, lda: lapack_int, r: *mut c_double,
                               c: *mut c_double, rowcnd: *mut c_double, colcnd: *mut c_double,
                               amax: *mut c_double)
                               -> lapack_int;
    pub fn LAPACKE_cgeequ_work(matrix_layout: c_int, m: lapack_int, n: lapack_int,
                               a: *const lapack_complex_float, lda: lapack_int, r: *mut c_float,
                               c: *mut c_float, rowcnd: *mut c_float, colcnd: *mut c_float,
                               amax: *mut c_float)
                               -> lapack_int;
    pub fn LAPACKE_zgeequ_work(matrix_layout: c_int, m: lapack_int, n: lapack_int,
                               a: *const lapack_complex_double, lda: lapack_int, r: *mut c_double,
                               c: *mut c_double, rowcnd: *mut c_double, colcnd: *mut c_double,
                               amax: *mut c_double)
                               -> lapack_int;

    pub fn LAPACKE_sgeequb_work(matrix_layout: c_int, m: lapack_int, n: lapack_int,
                                a: *const c_float, lda: lapack_int, r: *mut c_float,
                                c: *mut c_float, rowcnd: *mut c_float, colcnd: *mut c_float,
                                amax: *mut c_float)
                                -> lapack_int;
    pub fn LAPACKE_dgeequb_work(matrix_layout: c_int, m: lapack_int, n: lapack_int,
                                a: *const c_double, lda: lapack_int, r: *mut c_double,
                                c: *mut c_double, rowcnd: *mut c_double, colcnd: *mut c_double,
                                amax: *mut c_double)
                                -> lapack_int;
    pub fn LAPACKE_cgeequb_work(matrix_layout: c_int, m: lapack_int, n: lapack_int,
                                a: *const lapack_complex_float, lda: lapack_int, r: *mut c_float,
                                c: *mut c_float, rowcnd: *mut c_float, colcnd: *mut c_float,
                                amax: *mut c_float)
                                -> lapack_int;
    pub fn LAPACKE_zgeequb_work(matrix_layout: c_int, m: lapack_int, n: lapack_int,
                                a: *const lapack_complex_double, lda: lapack_int, r: *mut c_double,
                                c: *mut c_double, rowcnd: *mut c_double, colcnd: *mut c_double,
                                amax: *mut c_double)
                                -> lapack_int;

    pub fn LAPACKE_sgees_work(matrix_layout: c_int, jobvs: c_char, sort: c_char,
                              select: LAPACK_S_SELECT2, n: lapack_int, a: *mut c_float,
                              lda: lapack_int, sdim: *mut lapack_int, wr: *mut c_float,
                              wi: *mut c_float, vs: *mut c_float, ldvs: lapack_int,
                              work: *mut c_float, lwork: lapack_int, bwork: *mut lapack_logical)
                              -> lapack_int;
    pub fn LAPACKE_dgees_work(matrix_layout: c_int, jobvs: c_char, sort: c_char,
                              select: LAPACK_D_SELECT2, n: lapack_int, a: *mut c_double,
                              lda: lapack_int, sdim: *mut lapack_int, wr: *mut c_double,
                              wi: *mut c_double, vs: *mut c_double, ldvs: lapack_int,
                              work: *mut c_double, lwork: lapack_int, bwork: *mut lapack_logical)
                              -> lapack_int;
    pub fn LAPACKE_cgees_work(matrix_layout: c_int, jobvs: c_char, sort: c_char,
                              select: LAPACK_C_SELECT1, n: lapack_int,
                              a: *mut lapack_complex_float, lda: lapack_int, sdim: *mut lapack_int,
                              w: *mut lapack_complex_float, vs: *mut lapack_complex_float,
                              ldvs: lapack_int, work: *mut lapack_complex_float, lwork: lapack_int,
                              rwork: *mut c_float, bwork: *mut lapack_logical)
                              -> lapack_int;
    pub fn LAPACKE_zgees_work(matrix_layout: c_int, jobvs: c_char, sort: c_char,
                              select: LAPACK_Z_SELECT1, n: lapack_int,
                              a: *mut lapack_complex_double, lda: lapack_int,
                              sdim: *mut lapack_int, w: *mut lapack_complex_double,
                              vs: *mut lapack_complex_double, ldvs: lapack_int,
                              work: *mut lapack_complex_double, lwork: lapack_int,
                              rwork: *mut c_double, bwork: *mut lapack_logical)
                              -> lapack_int;

    pub fn LAPACKE_sgeesx_work(matrix_layout: c_int, jobvs: c_char, sort: c_char,
                               select: LAPACK_S_SELECT2, sense: c_char, n: lapack_int,
                               a: *mut c_float, lda: lapack_int, sdim: *mut lapack_int,
                               wr: *mut c_float, wi: *mut c_float, vs: *mut c_float,
                               ldvs: lapack_int, rconde: *mut c_float, rcondv: *mut c_float,
                               work: *mut c_float, lwork: lapack_int, iwork: *mut lapack_int,
                               liwork: lapack_int, bwork: *mut lapack_logical)
                               -> lapack_int;
    pub fn LAPACKE_dgeesx_work(matrix_layout: c_int, jobvs: c_char, sort: c_char,
                               select: LAPACK_D_SELECT2, sense: c_char, n: lapack_int,
                               a: *mut c_double, lda: lapack_int, sdim: *mut lapack_int,
                               wr: *mut c_double, wi: *mut c_double, vs: *mut c_double,
                               ldvs: lapack_int, rconde: *mut c_double, rcondv: *mut c_double,
                               work: *mut c_double, lwork: lapack_int, iwork: *mut lapack_int,
                               liwork: lapack_int, bwork: *mut lapack_logical)
                               -> lapack_int;
    pub fn LAPACKE_cgeesx_work(matrix_layout: c_int, jobvs: c_char, sort: c_char,
                               select: LAPACK_C_SELECT1, sense: c_char, n: lapack_int,
                               a: *mut lapack_complex_float, lda: lapack_int,
                               sdim: *mut lapack_int, w: *mut lapack_complex_float,
                               vs: *mut lapack_complex_float, ldvs: lapack_int,
                               rconde: *mut c_float, rcondv: *mut c_float,
                               work: *mut lapack_complex_float, lwork: lapack_int,
                               rwork: *mut c_float, bwork: *mut lapack_logical)
                               -> lapack_int;
    pub fn LAPACKE_zgeesx_work(matrix_layout: c_int, jobvs: c_char, sort: c_char,
                               select: LAPACK_Z_SELECT1, sense: c_char, n: lapack_int,
                               a: *mut lapack_complex_double, lda: lapack_int,
                               sdim: *mut lapack_int, w: *mut lapack_complex_double,
                               vs: *mut lapack_complex_double, ldvs: lapack_int,
                               rconde: *mut c_double, rcondv: *mut c_double,
                               work: *mut lapack_complex_double, lwork: lapack_int,
                               rwork: *mut c_double, bwork: *mut lapack_logical)
                               -> lapack_int;

    pub fn LAPACKE_sgeev_work(matrix_layout: c_int, jobvl: c_char, jobvr: c_char, n: lapack_int,
                              a: *mut c_float, lda: lapack_int, wr: *mut c_float, wi: *mut c_float,
                              vl: *mut c_float, ldvl: lapack_int, vr: *mut c_float,
                              ldvr: lapack_int, work: *mut c_float, lwork: lapack_int)
                              -> lapack_int;
    pub fn LAPACKE_dgeev_work(matrix_layout: c_int, jobvl: c_char, jobvr: c_char, n: lapack_int,
                              a: *mut c_double, lda: lapack_int, wr: *mut c_double,
                              wi: *mut c_double, vl: *mut c_double, ldvl: lapack_int,
                              vr: *mut c_double, ldvr: lapack_int, work: *mut c_double,
                              lwork: lapack_int)
                              -> lapack_int;
    pub fn LAPACKE_cgeev_work(matrix_layout: c_int, jobvl: c_char, jobvr: c_char, n: lapack_int,
                              a: *mut lapack_complex_float, lda: lapack_int,
                              w: *mut lapack_complex_float, vl: *mut lapack_complex_float,
                              ldvl: lapack_int, vr: *mut lapack_complex_float, ldvr: lapack_int,
                              work: *mut lapack_complex_float, lwork: lapack_int,
                              rwork: *mut c_float)
                              -> lapack_int;
    pub fn LAPACKE_zgeev_work(matrix_layout: c_int, jobvl: c_char, jobvr: c_char, n: lapack_int,
                              a: *mut lapack_complex_double, lda: lapack_int,
                              w: *mut lapack_complex_double, vl: *mut lapack_complex_double,
                              ldvl: lapack_int, vr: *mut lapack_complex_double, ldvr: lapack_int,
                              work: *mut lapack_complex_double, lwork: lapack_int,
                              rwork: *mut c_double)
                              -> lapack_int;

    pub fn LAPACKE_sgeevx_work(matrix_layout: c_int, balanc: c_char, jobvl: c_char, jobvr: c_char,
                               sense: c_char, n: lapack_int, a: *mut c_float, lda: lapack_int,
                               wr: *mut c_float, wi: *mut c_float, vl: *mut c_float,
                               ldvl: lapack_int, vr: *mut c_float, ldvr: lapack_int,
                               ilo: *mut lapack_int, ihi: *mut lapack_int, scale: *mut c_float,
                               abnrm: *mut c_float, rconde: *mut c_float, rcondv: *mut c_float,
                               work: *mut c_float, lwork: lapack_int, iwork: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dgeevx_work(matrix_layout: c_int, balanc: c_char, jobvl: c_char, jobvr: c_char,
                               sense: c_char, n: lapack_int, a: *mut c_double, lda: lapack_int,
                               wr: *mut c_double, wi: *mut c_double, vl: *mut c_double,
                               ldvl: lapack_int, vr: *mut c_double, ldvr: lapack_int,
                               ilo: *mut lapack_int, ihi: *mut lapack_int, scale: *mut c_double,
                               abnrm: *mut c_double, rconde: *mut c_double, rcondv: *mut c_double,
                               work: *mut c_double, lwork: lapack_int, iwork: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_cgeevx_work(matrix_layout: c_int, balanc: c_char, jobvl: c_char, jobvr: c_char,
                               sense: c_char, n: lapack_int, a: *mut lapack_complex_float,
                               lda: lapack_int, w: *mut lapack_complex_float,
                               vl: *mut lapack_complex_float, ldvl: lapack_int,
                               vr: *mut lapack_complex_float, ldvr: lapack_int,
                               ilo: *mut lapack_int, ihi: *mut lapack_int, scale: *mut c_float,
                               abnrm: *mut c_float, rconde: *mut c_float, rcondv: *mut c_float,
                               work: *mut lapack_complex_float, lwork: lapack_int,
                               rwork: *mut c_float)
                               -> lapack_int;
    pub fn LAPACKE_zgeevx_work(matrix_layout: c_int, balanc: c_char, jobvl: c_char, jobvr: c_char,
                               sense: c_char, n: lapack_int, a: *mut lapack_complex_double,
                               lda: lapack_int, w: *mut lapack_complex_double,
                               vl: *mut lapack_complex_double, ldvl: lapack_int,
                               vr: *mut lapack_complex_double, ldvr: lapack_int,
                               ilo: *mut lapack_int, ihi: *mut lapack_int, scale: *mut c_double,
                               abnrm: *mut c_double, rconde: *mut c_double, rcondv: *mut c_double,
                               work: *mut lapack_complex_double, lwork: lapack_int,
                               rwork: *mut c_double)
                               -> lapack_int;

    pub fn LAPACKE_sgehrd_work(matrix_layout: c_int, n: lapack_int, ilo: lapack_int,
                               ihi: lapack_int, a: *mut c_float, lda: lapack_int,
                               tau: *mut c_float, work: *mut c_float, lwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dgehrd_work(matrix_layout: c_int, n: lapack_int, ilo: lapack_int,
                               ihi: lapack_int, a: *mut c_double, lda: lapack_int,
                               tau: *mut c_double, work: *mut c_double, lwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_cgehrd_work(matrix_layout: c_int, n: lapack_int, ilo: lapack_int,
                               ihi: lapack_int, a: *mut lapack_complex_float, lda: lapack_int,
                               tau: *mut lapack_complex_float, work: *mut lapack_complex_float,
                               lwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_zgehrd_work(matrix_layout: c_int, n: lapack_int, ilo: lapack_int,
                               ihi: lapack_int, a: *mut lapack_complex_double, lda: lapack_int,
                               tau: *mut lapack_complex_double, work: *mut lapack_complex_double,
                               lwork: lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_sgejsv_work(matrix_layout: c_int, joba: c_char, jobu: c_char, jobv: c_char,
                               jobr: c_char, jobt: c_char, jobp: c_char, m: lapack_int,
                               n: lapack_int, a: *mut c_float, lda: lapack_int, sva: *mut c_float,
                               u: *mut c_float, ldu: lapack_int, v: *mut c_float, ldv: lapack_int,
                               work: *mut c_float, lwork: lapack_int, iwork: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dgejsv_work(matrix_layout: c_int, joba: c_char, jobu: c_char, jobv: c_char,
                               jobr: c_char, jobt: c_char, jobp: c_char, m: lapack_int,
                               n: lapack_int, a: *mut c_double, lda: lapack_int,
                               sva: *mut c_double, u: *mut c_double, ldu: lapack_int,
                               v: *mut c_double, ldv: lapack_int, work: *mut c_double,
                               lwork: lapack_int, iwork: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_cgejsv_work(matrix_layout: c_int, joba: c_char, jobu: c_char, jobv: c_char,
                               jobr: c_char, jobt: c_char, jobp: c_char, m: lapack_int,
                               n: lapack_int, a: *mut lapack_complex_float, lda: lapack_int,
                               sva: *mut c_float, u: *mut lapack_complex_float, ldu: lapack_int,
                               v: *mut lapack_complex_float, ldv: lapack_int,
                               cwork: *mut lapack_complex_float, lwork: lapack_int,
                               work: *mut c_float, lrwork: lapack_int, iwork: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_zgejsv_work(matrix_layout: c_int, joba: c_char, jobu: c_char, jobv: c_char,
                               jobr: c_char, jobt: c_char, jobp: c_char, m: lapack_int,
                               n: lapack_int, a: *mut lapack_complex_double, lda: lapack_int,
                               sva: *mut c_double, u: *mut lapack_complex_double, ldu: lapack_int,
                               v: *mut lapack_complex_double, ldv: lapack_int,
                               cwork: *mut lapack_complex_double, lwork: lapack_int,
                               work: *mut c_double, lrwork: lapack_int, iwork: *mut lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_sgelq2_work(matrix_layout: c_int, m: lapack_int, n: lapack_int, a: *mut c_float,
                               lda: lapack_int, tau: *mut c_float, work: *mut c_float)
                               -> lapack_int;
    pub fn LAPACKE_dgelq2_work(matrix_layout: c_int, m: lapack_int, n: lapack_int,
                               a: *mut c_double, lda: lapack_int, tau: *mut c_double,
                               work: *mut c_double)
                               -> lapack_int;
    pub fn LAPACKE_cgelq2_work(matrix_layout: c_int, m: lapack_int, n: lapack_int,
                               a: *mut lapack_complex_float, lda: lapack_int,
                               tau: *mut lapack_complex_float, work: *mut lapack_complex_float)
                               -> lapack_int;
    pub fn LAPACKE_zgelq2_work(matrix_layout: c_int, m: lapack_int, n: lapack_int,
                               a: *mut lapack_complex_double, lda: lapack_int,
                               tau: *mut lapack_complex_double, work: *mut lapack_complex_double)
                               -> lapack_int;

    pub fn LAPACKE_sgelqf_work(matrix_layout: c_int, m: lapack_int, n: lapack_int, a: *mut c_float,
                               lda: lapack_int, tau: *mut c_float, work: *mut c_float,
                               lwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dgelqf_work(matrix_layout: c_int, m: lapack_int, n: lapack_int,
                               a: *mut c_double, lda: lapack_int, tau: *mut c_double,
                               work: *mut c_double, lwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_cgelqf_work(matrix_layout: c_int, m: lapack_int, n: lapack_int,
                               a: *mut lapack_complex_float, lda: lapack_int,
                               tau: *mut lapack_complex_float, work: *mut lapack_complex_float,
                               lwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_zgelqf_work(matrix_layout: c_int, m: lapack_int, n: lapack_int,
                               a: *mut lapack_complex_double, lda: lapack_int,
                               tau: *mut lapack_complex_double, work: *mut lapack_complex_double,
                               lwork: lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_sgels_work(matrix_layout: c_int, trans: c_char, m: lapack_int, n: lapack_int,
                              nrhs: lapack_int, a: *mut c_float, lda: lapack_int, b: *mut c_float,
                              ldb: lapack_int, work: *mut c_float, lwork: lapack_int)
                              -> lapack_int;
    pub fn LAPACKE_dgels_work(matrix_layout: c_int, trans: c_char, m: lapack_int, n: lapack_int,
                              nrhs: lapack_int, a: *mut c_double, lda: lapack_int,
                              b: *mut c_double, ldb: lapack_int, work: *mut c_double,
                              lwork: lapack_int)
                              -> lapack_int;
    pub fn LAPACKE_cgels_work(matrix_layout: c_int, trans: c_char, m: lapack_int, n: lapack_int,
                              nrhs: lapack_int, a: *mut lapack_complex_float, lda: lapack_int,
                              b: *mut lapack_complex_float, ldb: lapack_int,
                              work: *mut lapack_complex_float, lwork: lapack_int)
                              -> lapack_int;
    pub fn LAPACKE_zgels_work(matrix_layout: c_int, trans: c_char, m: lapack_int, n: lapack_int,
                              nrhs: lapack_int, a: *mut lapack_complex_double, lda: lapack_int,
                              b: *mut lapack_complex_double, ldb: lapack_int,
                              work: *mut lapack_complex_double, lwork: lapack_int)
                              -> lapack_int;

    pub fn LAPACKE_sgelsd_work(matrix_layout: c_int, m: lapack_int, n: lapack_int,
                               nrhs: lapack_int, a: *mut c_float, lda: lapack_int, b: *mut c_float,
                               ldb: lapack_int, s: *mut c_float, rcond: c_float,
                               rank: *mut lapack_int, work: *mut c_float, lwork: lapack_int,
                               iwork: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dgelsd_work(matrix_layout: c_int, m: lapack_int, n: lapack_int,
                               nrhs: lapack_int, a: *mut c_double, lda: lapack_int,
                               b: *mut c_double, ldb: lapack_int, s: *mut c_double,
                               rcond: c_double, rank: *mut lapack_int, work: *mut c_double,
                               lwork: lapack_int, iwork: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_cgelsd_work(matrix_layout: c_int, m: lapack_int, n: lapack_int,
                               nrhs: lapack_int, a: *mut lapack_complex_float, lda: lapack_int,
                               b: *mut lapack_complex_float, ldb: lapack_int, s: *mut c_float,
                               rcond: c_float, rank: *mut lapack_int,
                               work: *mut lapack_complex_float, lwork: lapack_int,
                               rwork: *mut c_float, iwork: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_zgelsd_work(matrix_layout: c_int, m: lapack_int, n: lapack_int,
                               nrhs: lapack_int, a: *mut lapack_complex_double, lda: lapack_int,
                               b: *mut lapack_complex_double, ldb: lapack_int, s: *mut c_double,
                               rcond: c_double, rank: *mut lapack_int,
                               work: *mut lapack_complex_double, lwork: lapack_int,
                               rwork: *mut c_double, iwork: *mut lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_sgelss_work(matrix_layout: c_int, m: lapack_int, n: lapack_int,
                               nrhs: lapack_int, a: *mut c_float, lda: lapack_int, b: *mut c_float,
                               ldb: lapack_int, s: *mut c_float, rcond: c_float,
                               rank: *mut lapack_int, work: *mut c_float, lwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dgelss_work(matrix_layout: c_int, m: lapack_int, n: lapack_int,
                               nrhs: lapack_int, a: *mut c_double, lda: lapack_int,
                               b: *mut c_double, ldb: lapack_int, s: *mut c_double,
                               rcond: c_double, rank: *mut lapack_int, work: *mut c_double,
                               lwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_cgelss_work(matrix_layout: c_int, m: lapack_int, n: lapack_int,
                               nrhs: lapack_int, a: *mut lapack_complex_float, lda: lapack_int,
                               b: *mut lapack_complex_float, ldb: lapack_int, s: *mut c_float,
                               rcond: c_float, rank: *mut lapack_int,
                               work: *mut lapack_complex_float, lwork: lapack_int,
                               rwork: *mut c_float)
                               -> lapack_int;
    pub fn LAPACKE_zgelss_work(matrix_layout: c_int, m: lapack_int, n: lapack_int,
                               nrhs: lapack_int, a: *mut lapack_complex_double, lda: lapack_int,
                               b: *mut lapack_complex_double, ldb: lapack_int, s: *mut c_double,
                               rcond: c_double, rank: *mut lapack_int,
                               work: *mut lapack_complex_double, lwork: lapack_int,
                               rwork: *mut c_double)
                               -> lapack_int;

    pub fn LAPACKE_sgelsy_work(matrix_layout: c_int, m: lapack_int, n: lapack_int,
                               nrhs: lapack_int, a: *mut c_float, lda: lapack_int, b: *mut c_float,
                               ldb: lapack_int, jpvt: *mut lapack_int, rcond: c_float,
                               rank: *mut lapack_int, work: *mut c_float, lwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dgelsy_work(matrix_layout: c_int, m: lapack_int, n: lapack_int,
                               nrhs: lapack_int, a: *mut c_double, lda: lapack_int,
                               b: *mut c_double, ldb: lapack_int, jpvt: *mut lapack_int,
                               rcond: c_double, rank: *mut lapack_int, work: *mut c_double,
                               lwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_cgelsy_work(matrix_layout: c_int, m: lapack_int, n: lapack_int,
                               nrhs: lapack_int, a: *mut lapack_complex_float, lda: lapack_int,
                               b: *mut lapack_complex_float, ldb: lapack_int,
                               jpvt: *mut lapack_int, rcond: c_float, rank: *mut lapack_int,
                               work: *mut lapack_complex_float, lwork: lapack_int,
                               rwork: *mut c_float)
                               -> lapack_int;
    pub fn LAPACKE_zgelsy_work(matrix_layout: c_int, m: lapack_int, n: lapack_int,
                               nrhs: lapack_int, a: *mut lapack_complex_double, lda: lapack_int,
                               b: *mut lapack_complex_double, ldb: lapack_int,
                               jpvt: *mut lapack_int, rcond: c_double, rank: *mut lapack_int,
                               work: *mut lapack_complex_double, lwork: lapack_int,
                               rwork: *mut c_double)
                               -> lapack_int;

    pub fn LAPACKE_sgeqlf_work(matrix_layout: c_int, m: lapack_int, n: lapack_int, a: *mut c_float,
                               lda: lapack_int, tau: *mut c_float, work: *mut c_float,
                               lwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dgeqlf_work(matrix_layout: c_int, m: lapack_int, n: lapack_int,
                               a: *mut c_double, lda: lapack_int, tau: *mut c_double,
                               work: *mut c_double, lwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_cgeqlf_work(matrix_layout: c_int, m: lapack_int, n: lapack_int,
                               a: *mut lapack_complex_float, lda: lapack_int,
                               tau: *mut lapack_complex_float, work: *mut lapack_complex_float,
                               lwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_zgeqlf_work(matrix_layout: c_int, m: lapack_int, n: lapack_int,
                               a: *mut lapack_complex_double, lda: lapack_int,
                               tau: *mut lapack_complex_double, work: *mut lapack_complex_double,
                               lwork: lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_sgeqp3_work(matrix_layout: c_int, m: lapack_int, n: lapack_int, a: *mut c_float,
                               lda: lapack_int, jpvt: *mut lapack_int, tau: *mut c_float,
                               work: *mut c_float, lwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dgeqp3_work(matrix_layout: c_int, m: lapack_int, n: lapack_int,
                               a: *mut c_double, lda: lapack_int, jpvt: *mut lapack_int,
                               tau: *mut c_double, work: *mut c_double, lwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_cgeqp3_work(matrix_layout: c_int, m: lapack_int, n: lapack_int,
                               a: *mut lapack_complex_float, lda: lapack_int,
                               jpvt: *mut lapack_int, tau: *mut lapack_complex_float,
                               work: *mut lapack_complex_float, lwork: lapack_int,
                               rwork: *mut c_float)
                               -> lapack_int;
    pub fn LAPACKE_zgeqp3_work(matrix_layout: c_int, m: lapack_int, n: lapack_int,
                               a: *mut lapack_complex_double, lda: lapack_int,
                               jpvt: *mut lapack_int, tau: *mut lapack_complex_double,
                               work: *mut lapack_complex_double, lwork: lapack_int,
                               rwork: *mut c_double)
                               -> lapack_int;

    pub fn LAPACKE_sgeqpf_work(matrix_layout: c_int, m: lapack_int, n: lapack_int, a: *mut c_float,
                               lda: lapack_int, jpvt: *mut lapack_int, tau: *mut c_float,
                               work: *mut c_float)
                               -> lapack_int;
    pub fn LAPACKE_dgeqpf_work(matrix_layout: c_int, m: lapack_int, n: lapack_int,
                               a: *mut c_double, lda: lapack_int, jpvt: *mut lapack_int,
                               tau: *mut c_double, work: *mut c_double)
                               -> lapack_int;
    pub fn LAPACKE_cgeqpf_work(matrix_layout: c_int, m: lapack_int, n: lapack_int,
                               a: *mut lapack_complex_float, lda: lapack_int,
                               jpvt: *mut lapack_int, tau: *mut lapack_complex_float,
                               work: *mut lapack_complex_float, rwork: *mut c_float)
                               -> lapack_int;
    pub fn LAPACKE_zgeqpf_work(matrix_layout: c_int, m: lapack_int, n: lapack_int,
                               a: *mut lapack_complex_double, lda: lapack_int,
                               jpvt: *mut lapack_int, tau: *mut lapack_complex_double,
                               work: *mut lapack_complex_double, rwork: *mut c_double)
                               -> lapack_int;

    pub fn LAPACKE_sgeqr2_work(matrix_layout: c_int, m: lapack_int, n: lapack_int, a: *mut c_float,
                               lda: lapack_int, tau: *mut c_float, work: *mut c_float)
                               -> lapack_int;
    pub fn LAPACKE_dgeqr2_work(matrix_layout: c_int, m: lapack_int, n: lapack_int,
                               a: *mut c_double, lda: lapack_int, tau: *mut c_double,
                               work: *mut c_double)
                               -> lapack_int;
    pub fn LAPACKE_cgeqr2_work(matrix_layout: c_int, m: lapack_int, n: lapack_int,
                               a: *mut lapack_complex_float, lda: lapack_int,
                               tau: *mut lapack_complex_float, work: *mut lapack_complex_float)
                               -> lapack_int;
    pub fn LAPACKE_zgeqr2_work(matrix_layout: c_int, m: lapack_int, n: lapack_int,
                               a: *mut lapack_complex_double, lda: lapack_int,
                               tau: *mut lapack_complex_double, work: *mut lapack_complex_double)
                               -> lapack_int;

    pub fn LAPACKE_sgeqrf_work(matrix_layout: c_int, m: lapack_int, n: lapack_int, a: *mut c_float,
                               lda: lapack_int, tau: *mut c_float, work: *mut c_float,
                               lwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dgeqrf_work(matrix_layout: c_int, m: lapack_int, n: lapack_int,
                               a: *mut c_double, lda: lapack_int, tau: *mut c_double,
                               work: *mut c_double, lwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_cgeqrf_work(matrix_layout: c_int, m: lapack_int, n: lapack_int,
                               a: *mut lapack_complex_float, lda: lapack_int,
                               tau: *mut lapack_complex_float, work: *mut lapack_complex_float,
                               lwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_zgeqrf_work(matrix_layout: c_int, m: lapack_int, n: lapack_int,
                               a: *mut lapack_complex_double, lda: lapack_int,
                               tau: *mut lapack_complex_double, work: *mut lapack_complex_double,
                               lwork: lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_sgeqrfp_work(matrix_layout: c_int, m: lapack_int, n: lapack_int,
                                a: *mut c_float, lda: lapack_int, tau: *mut c_float,
                                work: *mut c_float, lwork: lapack_int)
                                -> lapack_int;
    pub fn LAPACKE_dgeqrfp_work(matrix_layout: c_int, m: lapack_int, n: lapack_int,
                                a: *mut c_double, lda: lapack_int, tau: *mut c_double,
                                work: *mut c_double, lwork: lapack_int)
                                -> lapack_int;
    pub fn LAPACKE_cgeqrfp_work(matrix_layout: c_int, m: lapack_int, n: lapack_int,
                                a: *mut lapack_complex_float, lda: lapack_int,
                                tau: *mut lapack_complex_float, work: *mut lapack_complex_float,
                                lwork: lapack_int)
                                -> lapack_int;
    pub fn LAPACKE_zgeqrfp_work(matrix_layout: c_int, m: lapack_int, n: lapack_int,
                                a: *mut lapack_complex_double, lda: lapack_int,
                                tau: *mut lapack_complex_double, work: *mut lapack_complex_double,
                                lwork: lapack_int)
                                -> lapack_int;

    pub fn LAPACKE_sgerfs_work(matrix_layout: c_int, trans: c_char, n: lapack_int,
                               nrhs: lapack_int, a: *const c_float, lda: lapack_int,
                               af: *const c_float, ldaf: lapack_int, ipiv: *const lapack_int,
                               b: *const c_float, ldb: lapack_int, x: *mut c_float,
                               ldx: lapack_int, ferr: *mut c_float, berr: *mut c_float,
                               work: *mut c_float, iwork: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dgerfs_work(matrix_layout: c_int, trans: c_char, n: lapack_int,
                               nrhs: lapack_int, a: *const c_double, lda: lapack_int,
                               af: *const c_double, ldaf: lapack_int, ipiv: *const lapack_int,
                               b: *const c_double, ldb: lapack_int, x: *mut c_double,
                               ldx: lapack_int, ferr: *mut c_double, berr: *mut c_double,
                               work: *mut c_double, iwork: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_cgerfs_work(matrix_layout: c_int, trans: c_char, n: lapack_int,
                               nrhs: lapack_int, a: *const lapack_complex_float, lda: lapack_int,
                               af: *const lapack_complex_float, ldaf: lapack_int,
                               ipiv: *const lapack_int, b: *const lapack_complex_float,
                               ldb: lapack_int, x: *mut lapack_complex_float, ldx: lapack_int,
                               ferr: *mut c_float, berr: *mut c_float,
                               work: *mut lapack_complex_float, rwork: *mut c_float)
                               -> lapack_int;
    pub fn LAPACKE_zgerfs_work(matrix_layout: c_int, trans: c_char, n: lapack_int,
                               nrhs: lapack_int, a: *const lapack_complex_double, lda: lapack_int,
                               af: *const lapack_complex_double, ldaf: lapack_int,
                               ipiv: *const lapack_int, b: *const lapack_complex_double,
                               ldb: lapack_int, x: *mut lapack_complex_double, ldx: lapack_int,
                               ferr: *mut c_double, berr: *mut c_double,
                               work: *mut lapack_complex_double, rwork: *mut c_double)
                               -> lapack_int;

    pub fn LAPACKE_sgerfsx_work(matrix_layout: c_int, trans: c_char, equed: c_char, n: lapack_int,
                                nrhs: lapack_int, a: *const c_float, lda: lapack_int,
                                af: *const c_float, ldaf: lapack_int, ipiv: *const lapack_int,
                                r: *const c_float, c: *const c_float, b: *const c_float,
                                ldb: lapack_int, x: *mut c_float, ldx: lapack_int,
                                rcond: *mut c_float, berr: *mut c_float, n_err_bnds: lapack_int,
                                err_bnds_norm: *mut c_float, err_bnds_comp: *mut c_float,
                                nparams: lapack_int, params: *mut c_float, work: *mut c_float,
                                iwork: *mut lapack_int)
                                -> lapack_int;
    pub fn LAPACKE_dgerfsx_work(matrix_layout: c_int, trans: c_char, equed: c_char, n: lapack_int,
                                nrhs: lapack_int, a: *const c_double, lda: lapack_int,
                                af: *const c_double, ldaf: lapack_int, ipiv: *const lapack_int,
                                r: *const c_double, c: *const c_double, b: *const c_double,
                                ldb: lapack_int, x: *mut c_double, ldx: lapack_int,
                                rcond: *mut c_double, berr: *mut c_double, n_err_bnds: lapack_int,
                                err_bnds_norm: *mut c_double, err_bnds_comp: *mut c_double,
                                nparams: lapack_int, params: *mut c_double, work: *mut c_double,
                                iwork: *mut lapack_int)
                                -> lapack_int;
    pub fn LAPACKE_cgerfsx_work(matrix_layout: c_int, trans: c_char, equed: c_char, n: lapack_int,
                                nrhs: lapack_int, a: *const lapack_complex_float, lda: lapack_int,
                                af: *const lapack_complex_float, ldaf: lapack_int,
                                ipiv: *const lapack_int, r: *const c_float, c: *const c_float,
                                b: *const lapack_complex_float, ldb: lapack_int,
                                x: *mut lapack_complex_float, ldx: lapack_int, rcond: *mut c_float,
                                berr: *mut c_float, n_err_bnds: lapack_int,
                                err_bnds_norm: *mut c_float, err_bnds_comp: *mut c_float,
                                nparams: lapack_int, params: *mut c_float,
                                work: *mut lapack_complex_float, rwork: *mut c_float)
                                -> lapack_int;
    pub fn LAPACKE_zgerfsx_work(matrix_layout: c_int, trans: c_char, equed: c_char, n: lapack_int,
                                nrhs: lapack_int, a: *const lapack_complex_double, lda: lapack_int,
                                af: *const lapack_complex_double, ldaf: lapack_int,
                                ipiv: *const lapack_int, r: *const c_double, c: *const c_double,
                                b: *const lapack_complex_double, ldb: lapack_int,
                                x: *mut lapack_complex_double, ldx: lapack_int,
                                rcond: *mut c_double, berr: *mut c_double, n_err_bnds: lapack_int,
                                err_bnds_norm: *mut c_double, err_bnds_comp: *mut c_double,
                                nparams: lapack_int, params: *mut c_double,
                                work: *mut lapack_complex_double, rwork: *mut c_double)
                                -> lapack_int;

    pub fn LAPACKE_sgerqf_work(matrix_layout: c_int, m: lapack_int, n: lapack_int, a: *mut c_float,
                               lda: lapack_int, tau: *mut c_float, work: *mut c_float,
                               lwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dgerqf_work(matrix_layout: c_int, m: lapack_int, n: lapack_int,
                               a: *mut c_double, lda: lapack_int, tau: *mut c_double,
                               work: *mut c_double, lwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_cgerqf_work(matrix_layout: c_int, m: lapack_int, n: lapack_int,
                               a: *mut lapack_complex_float, lda: lapack_int,
                               tau: *mut lapack_complex_float, work: *mut lapack_complex_float,
                               lwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_zgerqf_work(matrix_layout: c_int, m: lapack_int, n: lapack_int,
                               a: *mut lapack_complex_double, lda: lapack_int,
                               tau: *mut lapack_complex_double, work: *mut lapack_complex_double,
                               lwork: lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_sgesdd_work(matrix_layout: c_int, jobz: c_char, m: lapack_int, n: lapack_int,
                               a: *mut c_float, lda: lapack_int, s: *mut c_float, u: *mut c_float,
                               ldu: lapack_int, vt: *mut c_float, ldvt: lapack_int,
                               work: *mut c_float, lwork: lapack_int, iwork: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dgesdd_work(matrix_layout: c_int, jobz: c_char, m: lapack_int, n: lapack_int,
                               a: *mut c_double, lda: lapack_int, s: *mut c_double,
                               u: *mut c_double, ldu: lapack_int, vt: *mut c_double,
                               ldvt: lapack_int, work: *mut c_double, lwork: lapack_int,
                               iwork: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_cgesdd_work(matrix_layout: c_int, jobz: c_char, m: lapack_int, n: lapack_int,
                               a: *mut lapack_complex_float, lda: lapack_int, s: *mut c_float,
                               u: *mut lapack_complex_float, ldu: lapack_int,
                               vt: *mut lapack_complex_float, ldvt: lapack_int,
                               work: *mut lapack_complex_float, lwork: lapack_int,
                               rwork: *mut c_float, iwork: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_zgesdd_work(matrix_layout: c_int, jobz: c_char, m: lapack_int, n: lapack_int,
                               a: *mut lapack_complex_double, lda: lapack_int, s: *mut c_double,
                               u: *mut lapack_complex_double, ldu: lapack_int,
                               vt: *mut lapack_complex_double, ldvt: lapack_int,
                               work: *mut lapack_complex_double, lwork: lapack_int,
                               rwork: *mut c_double, iwork: *mut lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_sgesv_work(matrix_layout: c_int, n: lapack_int, nrhs: lapack_int,
                              a: *mut c_float, lda: lapack_int, ipiv: *mut lapack_int,
                              b: *mut c_float, ldb: lapack_int)
                              -> lapack_int;
    pub fn LAPACKE_dgesv_work(matrix_layout: c_int, n: lapack_int, nrhs: lapack_int,
                              a: *mut c_double, lda: lapack_int, ipiv: *mut lapack_int,
                              b: *mut c_double, ldb: lapack_int)
                              -> lapack_int;
    pub fn LAPACKE_cgesv_work(matrix_layout: c_int, n: lapack_int, nrhs: lapack_int,
                              a: *mut lapack_complex_float, lda: lapack_int, ipiv: *mut lapack_int,
                              b: *mut lapack_complex_float, ldb: lapack_int)
                              -> lapack_int;
    pub fn LAPACKE_zgesv_work(matrix_layout: c_int, n: lapack_int, nrhs: lapack_int,
                              a: *mut lapack_complex_double, lda: lapack_int,
                              ipiv: *mut lapack_int, b: *mut lapack_complex_double,
                              ldb: lapack_int)
                              -> lapack_int;
    pub fn LAPACKE_dsgesv_work(matrix_layout: c_int, n: lapack_int, nrhs: lapack_int,
                               a: *mut c_double, lda: lapack_int, ipiv: *mut lapack_int,
                               b: *mut c_double, ldb: lapack_int, x: *mut c_double,
                               ldx: lapack_int, work: *mut c_double, swork: *mut c_float,
                               iter: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_zcgesv_work(matrix_layout: c_int, n: lapack_int, nrhs: lapack_int,
                               a: *mut lapack_complex_double, lda: lapack_int,
                               ipiv: *mut lapack_int, b: *mut lapack_complex_double,
                               ldb: lapack_int, x: *mut lapack_complex_double, ldx: lapack_int,
                               work: *mut lapack_complex_double, swork: *mut lapack_complex_float,
                               rwork: *mut c_double, iter: *mut lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_sgesvd_work(matrix_layout: c_int, jobu: c_char, jobvt: c_char, m: lapack_int,
                               n: lapack_int, a: *mut c_float, lda: lapack_int, s: *mut c_float,
                               u: *mut c_float, ldu: lapack_int, vt: *mut c_float,
                               ldvt: lapack_int, work: *mut c_float, lwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dgesvd_work(matrix_layout: c_int, jobu: c_char, jobvt: c_char, m: lapack_int,
                               n: lapack_int, a: *mut c_double, lda: lapack_int, s: *mut c_double,
                               u: *mut c_double, ldu: lapack_int, vt: *mut c_double,
                               ldvt: lapack_int, work: *mut c_double, lwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_cgesvd_work(matrix_layout: c_int, jobu: c_char, jobvt: c_char, m: lapack_int,
                               n: lapack_int, a: *mut lapack_complex_float, lda: lapack_int,
                               s: *mut c_float, u: *mut lapack_complex_float, ldu: lapack_int,
                               vt: *mut lapack_complex_float, ldvt: lapack_int,
                               work: *mut lapack_complex_float, lwork: lapack_int,
                               rwork: *mut c_float)
                               -> lapack_int;
    pub fn LAPACKE_zgesvd_work(matrix_layout: c_int, jobu: c_char, jobvt: c_char, m: lapack_int,
                               n: lapack_int, a: *mut lapack_complex_double, lda: lapack_int,
                               s: *mut c_double, u: *mut lapack_complex_double, ldu: lapack_int,
                               vt: *mut lapack_complex_double, ldvt: lapack_int,
                               work: *mut lapack_complex_double, lwork: lapack_int,
                               rwork: *mut c_double)
                               -> lapack_int;

    pub fn LAPACKE_sgesvdx_work(matrix_layout: c_int, jobu: c_char, jobvt: c_char, range: c_char,
                                m: lapack_int, n: lapack_int, a: *mut c_float, lda: lapack_int,
                                vl: c_float, vu: c_float, il: lapack_int, iu: lapack_int,
                                ns: lapack_int, s: *mut c_float, u: *mut c_float, ldu: lapack_int,
                                vt: *mut c_float, ldvt: lapack_int, work: *mut c_float,
                                lwork: lapack_int, iwork: *mut lapack_int)
                                -> lapack_int;
    pub fn LAPACKE_dgesvdx_work(matrix_layout: c_int, jobu: c_char, jobvt: c_char, range: c_char,
                                m: lapack_int, n: lapack_int, a: *mut c_double, lda: lapack_int,
                                vl: c_double, vu: c_double, il: lapack_int, iu: lapack_int,
                                ns: lapack_int, s: *mut c_double, u: *mut c_double,
                                ldu: lapack_int, vt: *mut c_double, ldvt: lapack_int,
                                work: *mut c_double, lwork: lapack_int, iwork: *mut lapack_int)
                                -> lapack_int;
    pub fn LAPACKE_cgesvdx_work(matrix_layout: c_int, jobu: c_char, jobvt: c_char, range: c_char,
                                m: lapack_int, n: lapack_int, a: *mut lapack_complex_float,
                                lda: lapack_int, vl: c_float, vu: c_float, il: lapack_int,
                                iu: lapack_int, ns: lapack_int, s: *mut c_float,
                                u: *mut lapack_complex_float, ldu: lapack_int,
                                vt: *mut lapack_complex_float, ldvt: lapack_int,
                                work: *mut lapack_complex_float, lwork: lapack_int,
                                rwork: *mut c_float, iwork: *mut lapack_int)
                                -> lapack_int;
    pub fn LAPACKE_zgesvdx_work(matrix_layout: c_int, jobu: c_char, jobvt: c_char, range: c_char,
                                m: lapack_int, n: lapack_int, a: *mut lapack_complex_double,
                                lda: lapack_int, vl: c_double, vu: c_double, il: lapack_int,
                                iu: lapack_int, ns: lapack_int, s: *mut c_double,
                                u: *mut lapack_complex_double, ldu: lapack_int,
                                vt: *mut lapack_complex_double, ldvt: lapack_int,
                                work: *mut lapack_complex_double, lwork: lapack_int,
                                rwork: *mut c_double, iwork: *mut lapack_int)
                                -> lapack_int;

    pub fn LAPACKE_sgesvj_work(matrix_layout: c_int, joba: c_char, jobu: c_char, jobv: c_char,
                               m: lapack_int, n: lapack_int, a: *mut c_float, lda: lapack_int,
                               sva: *mut c_float, mv: lapack_int, v: *mut c_float, ldv: lapack_int,
                               work: *mut c_float, lwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dgesvj_work(matrix_layout: c_int, joba: c_char, jobu: c_char, jobv: c_char,
                               m: lapack_int, n: lapack_int, a: *mut c_double, lda: lapack_int,
                               sva: *mut c_double, mv: lapack_int, v: *mut c_double,
                               ldv: lapack_int, work: *mut c_double, lwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_cgesvj_work(matrix_layout: c_int, joba: c_char, jobu: c_char, jobv: c_char,
                               m: lapack_int, n: lapack_int, a: *mut lapack_complex_float,
                               lda: lapack_int, sva: *mut c_float, mv: lapack_int,
                               v: *mut lapack_complex_float, ldv: lapack_int,
                               cwork: *mut lapack_complex_float, lwork: lapack_int,
                               rwork: *mut c_float, lrwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_zgesvj_work(matrix_layout: c_int, joba: c_char, jobu: c_char, jobv: c_char,
                               m: lapack_int, n: lapack_int, a: *mut lapack_complex_double,
                               lda: lapack_int, sva: *mut c_double, mv: lapack_int,
                               v: *mut lapack_complex_double, ldv: lapack_int,
                               cwork: *mut lapack_complex_double, lwork: lapack_int,
                               rwork: *mut c_double, lrwork: lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_sgesvx_work(matrix_layout: c_int, fact: c_char, trans: c_char, n: lapack_int,
                               nrhs: lapack_int, a: *mut c_float, lda: lapack_int,
                               af: *mut c_float, ldaf: lapack_int, ipiv: *mut lapack_int,
                               equed: *mut c_char, r: *mut c_float, c: *mut c_float,
                               b: *mut c_float, ldb: lapack_int, x: *mut c_float, ldx: lapack_int,
                               rcond: *mut c_float, ferr: *mut c_float, berr: *mut c_float,
                               work: *mut c_float, iwork: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dgesvx_work(matrix_layout: c_int, fact: c_char, trans: c_char, n: lapack_int,
                               nrhs: lapack_int, a: *mut c_double, lda: lapack_int,
                               af: *mut c_double, ldaf: lapack_int, ipiv: *mut lapack_int,
                               equed: *mut c_char, r: *mut c_double, c: *mut c_double,
                               b: *mut c_double, ldb: lapack_int, x: *mut c_double,
                               ldx: lapack_int, rcond: *mut c_double, ferr: *mut c_double,
                               berr: *mut c_double, work: *mut c_double, iwork: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_cgesvx_work(matrix_layout: c_int, fact: c_char, trans: c_char, n: lapack_int,
                               nrhs: lapack_int, a: *mut lapack_complex_float, lda: lapack_int,
                               af: *mut lapack_complex_float, ldaf: lapack_int,
                               ipiv: *mut lapack_int, equed: *mut c_char, r: *mut c_float,
                               c: *mut c_float, b: *mut lapack_complex_float, ldb: lapack_int,
                               x: *mut lapack_complex_float, ldx: lapack_int, rcond: *mut c_float,
                               ferr: *mut c_float, berr: *mut c_float,
                               work: *mut lapack_complex_float, rwork: *mut c_float)
                               -> lapack_int;
    pub fn LAPACKE_zgesvx_work(matrix_layout: c_int, fact: c_char, trans: c_char, n: lapack_int,
                               nrhs: lapack_int, a: *mut lapack_complex_double, lda: lapack_int,
                               af: *mut lapack_complex_double, ldaf: lapack_int,
                               ipiv: *mut lapack_int, equed: *mut c_char, r: *mut c_double,
                               c: *mut c_double, b: *mut lapack_complex_double, ldb: lapack_int,
                               x: *mut lapack_complex_double, ldx: lapack_int,
                               rcond: *mut c_double, ferr: *mut c_double, berr: *mut c_double,
                               work: *mut lapack_complex_double, rwork: *mut c_double)
                               -> lapack_int;

    pub fn LAPACKE_sgesvxx_work(matrix_layout: c_int, fact: c_char, trans: c_char, n: lapack_int,
                                nrhs: lapack_int, a: *mut c_float, lda: lapack_int,
                                af: *mut c_float, ldaf: lapack_int, ipiv: *mut lapack_int,
                                equed: *mut c_char, r: *mut c_float, c: *mut c_float,
                                b: *mut c_float, ldb: lapack_int, x: *mut c_float, ldx: lapack_int,
                                rcond: *mut c_float, rpvgrw: *mut c_float, berr: *mut c_float,
                                n_err_bnds: lapack_int, err_bnds_norm: *mut c_float,
                                err_bnds_comp: *mut c_float, nparams: lapack_int,
                                params: *mut c_float, work: *mut c_float, iwork: *mut lapack_int)
                                -> lapack_int;
    pub fn LAPACKE_dgesvxx_work(matrix_layout: c_int, fact: c_char, trans: c_char, n: lapack_int,
                                nrhs: lapack_int, a: *mut c_double, lda: lapack_int,
                                af: *mut c_double, ldaf: lapack_int, ipiv: *mut lapack_int,
                                equed: *mut c_char, r: *mut c_double, c: *mut c_double,
                                b: *mut c_double, ldb: lapack_int, x: *mut c_double,
                                ldx: lapack_int, rcond: *mut c_double, rpvgrw: *mut c_double,
                                berr: *mut c_double, n_err_bnds: lapack_int,
                                err_bnds_norm: *mut c_double, err_bnds_comp: *mut c_double,
                                nparams: lapack_int, params: *mut c_double, work: *mut c_double,
                                iwork: *mut lapack_int)
                                -> lapack_int;
    pub fn LAPACKE_cgesvxx_work(matrix_layout: c_int, fact: c_char, trans: c_char, n: lapack_int,
                                nrhs: lapack_int, a: *mut lapack_complex_float, lda: lapack_int,
                                af: *mut lapack_complex_float, ldaf: lapack_int,
                                ipiv: *mut lapack_int, equed: *mut c_char, r: *mut c_float,
                                c: *mut c_float, b: *mut lapack_complex_float, ldb: lapack_int,
                                x: *mut lapack_complex_float, ldx: lapack_int, rcond: *mut c_float,
                                rpvgrw: *mut c_float, berr: *mut c_float, n_err_bnds: lapack_int,
                                err_bnds_norm: *mut c_float, err_bnds_comp: *mut c_float,
                                nparams: lapack_int, params: *mut c_float,
                                work: *mut lapack_complex_float, rwork: *mut c_float)
                                -> lapack_int;
    pub fn LAPACKE_zgesvxx_work(matrix_layout: c_int, fact: c_char, trans: c_char, n: lapack_int,
                                nrhs: lapack_int, a: *mut lapack_complex_double, lda: lapack_int,
                                af: *mut lapack_complex_double, ldaf: lapack_int,
                                ipiv: *mut lapack_int, equed: *mut c_char, r: *mut c_double,
                                c: *mut c_double, b: *mut lapack_complex_double, ldb: lapack_int,
                                x: *mut lapack_complex_double, ldx: lapack_int,
                                rcond: *mut c_double, rpvgrw: *mut c_double, berr: *mut c_double,
                                n_err_bnds: lapack_int, err_bnds_norm: *mut c_double,
                                err_bnds_comp: *mut c_double, nparams: lapack_int,
                                params: *mut c_double, work: *mut lapack_complex_double,
                                rwork: *mut c_double)
                                -> lapack_int;

    pub fn LAPACKE_sgetf2_work(matrix_layout: c_int, m: lapack_int, n: lapack_int, a: *mut c_float,
                               lda: lapack_int, ipiv: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dgetf2_work(matrix_layout: c_int, m: lapack_int, n: lapack_int,
                               a: *mut c_double, lda: lapack_int, ipiv: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_cgetf2_work(matrix_layout: c_int, m: lapack_int, n: lapack_int,
                               a: *mut lapack_complex_float, lda: lapack_int,
                               ipiv: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_zgetf2_work(matrix_layout: c_int, m: lapack_int, n: lapack_int,
                               a: *mut lapack_complex_double, lda: lapack_int,
                               ipiv: *mut lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_sgetrf_work(matrix_layout: c_int, m: lapack_int, n: lapack_int, a: *mut c_float,
                               lda: lapack_int, ipiv: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dgetrf_work(matrix_layout: c_int, m: lapack_int, n: lapack_int,
                               a: *mut c_double, lda: lapack_int, ipiv: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_cgetrf_work(matrix_layout: c_int, m: lapack_int, n: lapack_int,
                               a: *mut lapack_complex_float, lda: lapack_int,
                               ipiv: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_zgetrf_work(matrix_layout: c_int, m: lapack_int, n: lapack_int,
                               a: *mut lapack_complex_double, lda: lapack_int,
                               ipiv: *mut lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_sgetrf2_work(matrix_layout: c_int, m: lapack_int, n: lapack_int,
                                a: *mut c_float, lda: lapack_int, ipiv: *mut lapack_int)
                                -> lapack_int;
    pub fn LAPACKE_dgetrf2_work(matrix_layout: c_int, m: lapack_int, n: lapack_int,
                                a: *mut c_double, lda: lapack_int, ipiv: *mut lapack_int)
                                -> lapack_int;
    pub fn LAPACKE_cgetrf2_work(matrix_layout: c_int, m: lapack_int, n: lapack_int,
                                a: *mut lapack_complex_float, lda: lapack_int,
                                ipiv: *mut lapack_int)
                                -> lapack_int;
    pub fn LAPACKE_zgetrf2_work(matrix_layout: c_int, m: lapack_int, n: lapack_int,
                                a: *mut lapack_complex_double, lda: lapack_int,
                                ipiv: *mut lapack_int)
                                -> lapack_int;

    pub fn LAPACKE_sgetri_work(matrix_layout: c_int, n: lapack_int, a: *mut c_float,
                               lda: lapack_int, ipiv: *const lapack_int, work: *mut c_float,
                               lwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dgetri_work(matrix_layout: c_int, n: lapack_int, a: *mut c_double,
                               lda: lapack_int, ipiv: *const lapack_int, work: *mut c_double,
                               lwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_cgetri_work(matrix_layout: c_int, n: lapack_int, a: *mut lapack_complex_float,
                               lda: lapack_int, ipiv: *const lapack_int,
                               work: *mut lapack_complex_float, lwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_zgetri_work(matrix_layout: c_int, n: lapack_int, a: *mut lapack_complex_double,
                               lda: lapack_int, ipiv: *const lapack_int,
                               work: *mut lapack_complex_double, lwork: lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_sgetrs_work(matrix_layout: c_int, trans: c_char, n: lapack_int,
                               nrhs: lapack_int, a: *const c_float, lda: lapack_int,
                               ipiv: *const lapack_int, b: *mut c_float, ldb: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dgetrs_work(matrix_layout: c_int, trans: c_char, n: lapack_int,
                               nrhs: lapack_int, a: *const c_double, lda: lapack_int,
                               ipiv: *const lapack_int, b: *mut c_double, ldb: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_cgetrs_work(matrix_layout: c_int, trans: c_char, n: lapack_int,
                               nrhs: lapack_int, a: *const lapack_complex_float, lda: lapack_int,
                               ipiv: *const lapack_int, b: *mut lapack_complex_float,
                               ldb: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_zgetrs_work(matrix_layout: c_int, trans: c_char, n: lapack_int,
                               nrhs: lapack_int, a: *const lapack_complex_double, lda: lapack_int,
                               ipiv: *const lapack_int, b: *mut lapack_complex_double,
                               ldb: lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_sggbak_work(matrix_layout: c_int, job: c_char, side: c_char, n: lapack_int,
                               ilo: lapack_int, ihi: lapack_int, lscale: *const c_float,
                               rscale: *const c_float, m: lapack_int, v: *mut c_float,
                               ldv: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dggbak_work(matrix_layout: c_int, job: c_char, side: c_char, n: lapack_int,
                               ilo: lapack_int, ihi: lapack_int, lscale: *const c_double,
                               rscale: *const c_double, m: lapack_int, v: *mut c_double,
                               ldv: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_cggbak_work(matrix_layout: c_int, job: c_char, side: c_char, n: lapack_int,
                               ilo: lapack_int, ihi: lapack_int, lscale: *const c_float,
                               rscale: *const c_float, m: lapack_int, v: *mut lapack_complex_float,
                               ldv: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_zggbak_work(matrix_layout: c_int, job: c_char, side: c_char, n: lapack_int,
                               ilo: lapack_int, ihi: lapack_int, lscale: *const c_double,
                               rscale: *const c_double, m: lapack_int,
                               v: *mut lapack_complex_double, ldv: lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_sggbal_work(matrix_layout: c_int, job: c_char, n: lapack_int, a: *mut c_float,
                               lda: lapack_int, b: *mut c_float, ldb: lapack_int,
                               ilo: *mut lapack_int, ihi: *mut lapack_int, lscale: *mut c_float,
                               rscale: *mut c_float, work: *mut c_float)
                               -> lapack_int;
    pub fn LAPACKE_dggbal_work(matrix_layout: c_int, job: c_char, n: lapack_int, a: *mut c_double,
                               lda: lapack_int, b: *mut c_double, ldb: lapack_int,
                               ilo: *mut lapack_int, ihi: *mut lapack_int, lscale: *mut c_double,
                               rscale: *mut c_double, work: *mut c_double)
                               -> lapack_int;
    pub fn LAPACKE_cggbal_work(matrix_layout: c_int, job: c_char, n: lapack_int,
                               a: *mut lapack_complex_float, lda: lapack_int,
                               b: *mut lapack_complex_float, ldb: lapack_int, ilo: *mut lapack_int,
                               ihi: *mut lapack_int, lscale: *mut c_float, rscale: *mut c_float,
                               work: *mut c_float)
                               -> lapack_int;
    pub fn LAPACKE_zggbal_work(matrix_layout: c_int, job: c_char, n: lapack_int,
                               a: *mut lapack_complex_double, lda: lapack_int,
                               b: *mut lapack_complex_double, ldb: lapack_int,
                               ilo: *mut lapack_int, ihi: *mut lapack_int, lscale: *mut c_double,
                               rscale: *mut c_double, work: *mut c_double)
                               -> lapack_int;

    pub fn LAPACKE_sgges_work(matrix_layout: c_int, jobvsl: c_char, jobvsr: c_char, sort: c_char,
                              selctg: LAPACK_S_SELECT3, n: lapack_int, a: *mut c_float,
                              lda: lapack_int, b: *mut c_float, ldb: lapack_int,
                              sdim: *mut lapack_int, alphar: *mut c_float, alphai: *mut c_float,
                              beta: *mut c_float, vsl: *mut c_float, ldvsl: lapack_int,
                              vsr: *mut c_float, ldvsr: lapack_int, work: *mut c_float,
                              lwork: lapack_int, bwork: *mut lapack_logical)
                              -> lapack_int;
    pub fn LAPACKE_dgges_work(matrix_layout: c_int, jobvsl: c_char, jobvsr: c_char, sort: c_char,
                              selctg: LAPACK_D_SELECT3, n: lapack_int, a: *mut c_double,
                              lda: lapack_int, b: *mut c_double, ldb: lapack_int,
                              sdim: *mut lapack_int, alphar: *mut c_double, alphai: *mut c_double,
                              beta: *mut c_double, vsl: *mut c_double, ldvsl: lapack_int,
                              vsr: *mut c_double, ldvsr: lapack_int, work: *mut c_double,
                              lwork: lapack_int, bwork: *mut lapack_logical)
                              -> lapack_int;
    pub fn LAPACKE_cgges_work(matrix_layout: c_int, jobvsl: c_char, jobvsr: c_char, sort: c_char,
                              selctg: LAPACK_C_SELECT2, n: lapack_int,
                              a: *mut lapack_complex_float, lda: lapack_int,
                              b: *mut lapack_complex_float, ldb: lapack_int, sdim: *mut lapack_int,
                              alpha: *mut lapack_complex_float, beta: *mut lapack_complex_float,
                              vsl: *mut lapack_complex_float, ldvsl: lapack_int,
                              vsr: *mut lapack_complex_float, ldvsr: lapack_int,
                              work: *mut lapack_complex_float, lwork: lapack_int,
                              rwork: *mut c_float, bwork: *mut lapack_logical)
                              -> lapack_int;
    pub fn LAPACKE_zgges_work(matrix_layout: c_int, jobvsl: c_char, jobvsr: c_char, sort: c_char,
                              selctg: LAPACK_Z_SELECT2, n: lapack_int,
                              a: *mut lapack_complex_double, lda: lapack_int,
                              b: *mut lapack_complex_double, ldb: lapack_int,
                              sdim: *mut lapack_int, alpha: *mut lapack_complex_double,
                              beta: *mut lapack_complex_double, vsl: *mut lapack_complex_double,
                              ldvsl: lapack_int, vsr: *mut lapack_complex_double,
                              ldvsr: lapack_int, work: *mut lapack_complex_double,
                              lwork: lapack_int, rwork: *mut c_double, bwork: *mut lapack_logical)
                              -> lapack_int;

    pub fn LAPACKE_sgges3_work(matrix_layout: c_int, jobvsl: c_char, jobvsr: c_char, sort: c_char,
                               selctg: LAPACK_S_SELECT3, n: lapack_int, a: *mut c_float,
                               lda: lapack_int, b: *mut c_float, ldb: lapack_int,
                               sdim: *mut lapack_int, alphar: *mut c_float, alphai: *mut c_float,
                               beta: *mut c_float, vsl: *mut c_float, ldvsl: lapack_int,
                               vsr: *mut c_float, ldvsr: lapack_int, work: *mut c_float,
                               lwork: lapack_int, bwork: *mut lapack_logical)
                               -> lapack_int;
    pub fn LAPACKE_dgges3_work(matrix_layout: c_int, jobvsl: c_char, jobvsr: c_char, sort: c_char,
                               selctg: LAPACK_D_SELECT3, n: lapack_int, a: *mut c_double,
                               lda: lapack_int, b: *mut c_double, ldb: lapack_int,
                               sdim: *mut lapack_int, alphar: *mut c_double, alphai: *mut c_double,
                               beta: *mut c_double, vsl: *mut c_double, ldvsl: lapack_int,
                               vsr: *mut c_double, ldvsr: lapack_int, work: *mut c_double,
                               lwork: lapack_int, bwork: *mut lapack_logical)
                               -> lapack_int;
    pub fn LAPACKE_cgges3_work(matrix_layout: c_int, jobvsl: c_char, jobvsr: c_char, sort: c_char,
                               selctg: LAPACK_C_SELECT2, n: lapack_int,
                               a: *mut lapack_complex_float, lda: lapack_int,
                               b: *mut lapack_complex_float, ldb: lapack_int,
                               sdim: *mut lapack_int, alpha: *mut lapack_complex_float,
                               beta: *mut lapack_complex_float, vsl: *mut lapack_complex_float,
                               ldvsl: lapack_int, vsr: *mut lapack_complex_float,
                               ldvsr: lapack_int, work: *mut lapack_complex_float,
                               lwork: lapack_int, rwork: *mut c_float, bwork: *mut lapack_logical)
                               -> lapack_int;
    pub fn LAPACKE_zgges3_work(matrix_layout: c_int, jobvsl: c_char, jobvsr: c_char, sort: c_char,
                               selctg: LAPACK_Z_SELECT2, n: lapack_int,
                               a: *mut lapack_complex_double, lda: lapack_int,
                               b: *mut lapack_complex_double, ldb: lapack_int,
                               sdim: *mut lapack_int, alpha: *mut lapack_complex_double,
                               beta: *mut lapack_complex_double, vsl: *mut lapack_complex_double,
                               ldvsl: lapack_int, vsr: *mut lapack_complex_double,
                               ldvsr: lapack_int, work: *mut lapack_complex_double,
                               lwork: lapack_int, rwork: *mut c_double, bwork: *mut lapack_logical)
                               -> lapack_int;

    pub fn LAPACKE_sggesx_work(matrix_layout: c_int, jobvsl: c_char, jobvsr: c_char, sort: c_char,
                               selctg: LAPACK_S_SELECT3, sense: c_char, n: lapack_int,
                               a: *mut c_float, lda: lapack_int, b: *mut c_float, ldb: lapack_int,
                               sdim: *mut lapack_int, alphar: *mut c_float, alphai: *mut c_float,
                               beta: *mut c_float, vsl: *mut c_float, ldvsl: lapack_int,
                               vsr: *mut c_float, ldvsr: lapack_int, rconde: *mut c_float,
                               rcondv: *mut c_float, work: *mut c_float, lwork: lapack_int,
                               iwork: *mut lapack_int, liwork: lapack_int,
                               bwork: *mut lapack_logical)
                               -> lapack_int;
    pub fn LAPACKE_dggesx_work(matrix_layout: c_int, jobvsl: c_char, jobvsr: c_char, sort: c_char,
                               selctg: LAPACK_D_SELECT3, sense: c_char, n: lapack_int,
                               a: *mut c_double, lda: lapack_int, b: *mut c_double,
                               ldb: lapack_int, sdim: *mut lapack_int, alphar: *mut c_double,
                               alphai: *mut c_double, beta: *mut c_double, vsl: *mut c_double,
                               ldvsl: lapack_int, vsr: *mut c_double, ldvsr: lapack_int,
                               rconde: *mut c_double, rcondv: *mut c_double, work: *mut c_double,
                               lwork: lapack_int, iwork: *mut lapack_int, liwork: lapack_int,
                               bwork: *mut lapack_logical)
                               -> lapack_int;
    pub fn LAPACKE_cggesx_work(matrix_layout: c_int, jobvsl: c_char, jobvsr: c_char, sort: c_char,
                               selctg: LAPACK_C_SELECT2, sense: c_char, n: lapack_int,
                               a: *mut lapack_complex_float, lda: lapack_int,
                               b: *mut lapack_complex_float, ldb: lapack_int,
                               sdim: *mut lapack_int, alpha: *mut lapack_complex_float,
                               beta: *mut lapack_complex_float, vsl: *mut lapack_complex_float,
                               ldvsl: lapack_int, vsr: *mut lapack_complex_float,
                               ldvsr: lapack_int, rconde: *mut c_float, rcondv: *mut c_float,
                               work: *mut lapack_complex_float, lwork: lapack_int,
                               rwork: *mut c_float, iwork: *mut lapack_int, liwork: lapack_int,
                               bwork: *mut lapack_logical)
                               -> lapack_int;
    pub fn LAPACKE_zggesx_work(matrix_layout: c_int, jobvsl: c_char, jobvsr: c_char, sort: c_char,
                               selctg: LAPACK_Z_SELECT2, sense: c_char, n: lapack_int,
                               a: *mut lapack_complex_double, lda: lapack_int,
                               b: *mut lapack_complex_double, ldb: lapack_int,
                               sdim: *mut lapack_int, alpha: *mut lapack_complex_double,
                               beta: *mut lapack_complex_double, vsl: *mut lapack_complex_double,
                               ldvsl: lapack_int, vsr: *mut lapack_complex_double,
                               ldvsr: lapack_int, rconde: *mut c_double, rcondv: *mut c_double,
                               work: *mut lapack_complex_double, lwork: lapack_int,
                               rwork: *mut c_double, iwork: *mut lapack_int, liwork: lapack_int,
                               bwork: *mut lapack_logical)
                               -> lapack_int;

    pub fn LAPACKE_sggev_work(matrix_layout: c_int, jobvl: c_char, jobvr: c_char, n: lapack_int,
                              a: *mut c_float, lda: lapack_int, b: *mut c_float, ldb: lapack_int,
                              alphar: *mut c_float, alphai: *mut c_float, beta: *mut c_float,
                              vl: *mut c_float, ldvl: lapack_int, vr: *mut c_float,
                              ldvr: lapack_int, work: *mut c_float, lwork: lapack_int)
                              -> lapack_int;
    pub fn LAPACKE_dggev_work(matrix_layout: c_int, jobvl: c_char, jobvr: c_char, n: lapack_int,
                              a: *mut c_double, lda: lapack_int, b: *mut c_double, ldb: lapack_int,
                              alphar: *mut c_double, alphai: *mut c_double, beta: *mut c_double,
                              vl: *mut c_double, ldvl: lapack_int, vr: *mut c_double,
                              ldvr: lapack_int, work: *mut c_double, lwork: lapack_int)
                              -> lapack_int;
    pub fn LAPACKE_cggev_work(matrix_layout: c_int, jobvl: c_char, jobvr: c_char, n: lapack_int,
                              a: *mut lapack_complex_float, lda: lapack_int,
                              b: *mut lapack_complex_float, ldb: lapack_int,
                              alpha: *mut lapack_complex_float, beta: *mut lapack_complex_float,
                              vl: *mut lapack_complex_float, ldvl: lapack_int,
                              vr: *mut lapack_complex_float, ldvr: lapack_int,
                              work: *mut lapack_complex_float, lwork: lapack_int,
                              rwork: *mut c_float)
                              -> lapack_int;
    pub fn LAPACKE_zggev_work(matrix_layout: c_int, jobvl: c_char, jobvr: c_char, n: lapack_int,
                              a: *mut lapack_complex_double, lda: lapack_int,
                              b: *mut lapack_complex_double, ldb: lapack_int,
                              alpha: *mut lapack_complex_double, beta: *mut lapack_complex_double,
                              vl: *mut lapack_complex_double, ldvl: lapack_int,
                              vr: *mut lapack_complex_double, ldvr: lapack_int,
                              work: *mut lapack_complex_double, lwork: lapack_int,
                              rwork: *mut c_double)
                              -> lapack_int;

    pub fn LAPACKE_sggev3_work(matrix_layout: c_int, jobvl: c_char, jobvr: c_char, n: lapack_int,
                               a: *mut c_float, lda: lapack_int, b: *mut c_float, ldb: lapack_int,
                               alphar: *mut c_float, alphai: *mut c_float, beta: *mut c_float,
                               vl: *mut c_float, ldvl: lapack_int, vr: *mut c_float,
                               ldvr: lapack_int, work: *mut c_float, lwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dggev3_work(matrix_layout: c_int, jobvl: c_char, jobvr: c_char, n: lapack_int,
                               a: *mut c_double, lda: lapack_int, b: *mut c_double,
                               ldb: lapack_int, alphar: *mut c_double, alphai: *mut c_double,
                               beta: *mut c_double, vl: *mut c_double, ldvl: lapack_int,
                               vr: *mut c_double, ldvr: lapack_int, work: *mut c_double,
                               lwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_cggev3_work(matrix_layout: c_int, jobvl: c_char, jobvr: c_char, n: lapack_int,
                               a: *mut lapack_complex_float, lda: lapack_int,
                               b: *mut lapack_complex_float, ldb: lapack_int,
                               alpha: *mut lapack_complex_float, beta: *mut lapack_complex_float,
                               vl: *mut lapack_complex_float, ldvl: lapack_int,
                               vr: *mut lapack_complex_float, ldvr: lapack_int,
                               work: *mut lapack_complex_float, lwork: lapack_int,
                               rwork: *mut c_float)
                               -> lapack_int;
    pub fn LAPACKE_zggev3_work(matrix_layout: c_int, jobvl: c_char, jobvr: c_char, n: lapack_int,
                               a: *mut lapack_complex_double, lda: lapack_int,
                               b: *mut lapack_complex_double, ldb: lapack_int,
                               alpha: *mut lapack_complex_double, beta: *mut lapack_complex_double,
                               vl: *mut lapack_complex_double, ldvl: lapack_int,
                               vr: *mut lapack_complex_double, ldvr: lapack_int,
                               work: *mut lapack_complex_double, lwork: lapack_int,
                               rwork: *mut c_double)
                               -> lapack_int;

    pub fn LAPACKE_sggevx_work(matrix_layout: c_int, balanc: c_char, jobvl: c_char, jobvr: c_char,
                               sense: c_char, n: lapack_int, a: *mut c_float, lda: lapack_int,
                               b: *mut c_float, ldb: lapack_int, alphar: *mut c_float,
                               alphai: *mut c_float, beta: *mut c_float, vl: *mut c_float,
                               ldvl: lapack_int, vr: *mut c_float, ldvr: lapack_int,
                               ilo: *mut lapack_int, ihi: *mut lapack_int, lscale: *mut c_float,
                               rscale: *mut c_float, abnrm: *mut c_float, bbnrm: *mut c_float,
                               rconde: *mut c_float, rcondv: *mut c_float, work: *mut c_float,
                               lwork: lapack_int, iwork: *mut lapack_int,
                               bwork: *mut lapack_logical)
                               -> lapack_int;
    pub fn LAPACKE_dggevx_work(matrix_layout: c_int, balanc: c_char, jobvl: c_char, jobvr: c_char,
                               sense: c_char, n: lapack_int, a: *mut c_double, lda: lapack_int,
                               b: *mut c_double, ldb: lapack_int, alphar: *mut c_double,
                               alphai: *mut c_double, beta: *mut c_double, vl: *mut c_double,
                               ldvl: lapack_int, vr: *mut c_double, ldvr: lapack_int,
                               ilo: *mut lapack_int, ihi: *mut lapack_int, lscale: *mut c_double,
                               rscale: *mut c_double, abnrm: *mut c_double, bbnrm: *mut c_double,
                               rconde: *mut c_double, rcondv: *mut c_double, work: *mut c_double,
                               lwork: lapack_int, iwork: *mut lapack_int,
                               bwork: *mut lapack_logical)
                               -> lapack_int;
    pub fn LAPACKE_cggevx_work(matrix_layout: c_int, balanc: c_char, jobvl: c_char, jobvr: c_char,
                               sense: c_char, n: lapack_int, a: *mut lapack_complex_float,
                               lda: lapack_int, b: *mut lapack_complex_float, ldb: lapack_int,
                               alpha: *mut lapack_complex_float, beta: *mut lapack_complex_float,
                               vl: *mut lapack_complex_float, ldvl: lapack_int,
                               vr: *mut lapack_complex_float, ldvr: lapack_int,
                               ilo: *mut lapack_int, ihi: *mut lapack_int, lscale: *mut c_float,
                               rscale: *mut c_float, abnrm: *mut c_float, bbnrm: *mut c_float,
                               rconde: *mut c_float, rcondv: *mut c_float,
                               work: *mut lapack_complex_float, lwork: lapack_int,
                               rwork: *mut c_float, iwork: *mut lapack_int,
                               bwork: *mut lapack_logical)
                               -> lapack_int;
    pub fn LAPACKE_zggevx_work(matrix_layout: c_int, balanc: c_char, jobvl: c_char, jobvr: c_char,
                               sense: c_char, n: lapack_int, a: *mut lapack_complex_double,
                               lda: lapack_int, b: *mut lapack_complex_double, ldb: lapack_int,
                               alpha: *mut lapack_complex_double, beta: *mut lapack_complex_double,
                               vl: *mut lapack_complex_double, ldvl: lapack_int,
                               vr: *mut lapack_complex_double, ldvr: lapack_int,
                               ilo: *mut lapack_int, ihi: *mut lapack_int, lscale: *mut c_double,
                               rscale: *mut c_double, abnrm: *mut c_double, bbnrm: *mut c_double,
                               rconde: *mut c_double, rcondv: *mut c_double,
                               work: *mut lapack_complex_double, lwork: lapack_int,
                               rwork: *mut c_double, iwork: *mut lapack_int,
                               bwork: *mut lapack_logical)
                               -> lapack_int;

    pub fn LAPACKE_sggglm_work(matrix_layout: c_int, n: lapack_int, m: lapack_int, p: lapack_int,
                               a: *mut c_float, lda: lapack_int, b: *mut c_float, ldb: lapack_int,
                               d: *mut c_float, x: *mut c_float, y: *mut c_float,
                               work: *mut c_float, lwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dggglm_work(matrix_layout: c_int, n: lapack_int, m: lapack_int, p: lapack_int,
                               a: *mut c_double, lda: lapack_int, b: *mut c_double,
                               ldb: lapack_int, d: *mut c_double, x: *mut c_double,
                               y: *mut c_double, work: *mut c_double, lwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_cggglm_work(matrix_layout: c_int, n: lapack_int, m: lapack_int, p: lapack_int,
                               a: *mut lapack_complex_float, lda: lapack_int,
                               b: *mut lapack_complex_float, ldb: lapack_int,
                               d: *mut lapack_complex_float, x: *mut lapack_complex_float,
                               y: *mut lapack_complex_float, work: *mut lapack_complex_float,
                               lwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_zggglm_work(matrix_layout: c_int, n: lapack_int, m: lapack_int, p: lapack_int,
                               a: *mut lapack_complex_double, lda: lapack_int,
                               b: *mut lapack_complex_double, ldb: lapack_int,
                               d: *mut lapack_complex_double, x: *mut lapack_complex_double,
                               y: *mut lapack_complex_double, work: *mut lapack_complex_double,
                               lwork: lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_sgghrd_work(matrix_layout: c_int, compq: c_char, compz: c_char, n: lapack_int,
                               ilo: lapack_int, ihi: lapack_int, a: *mut c_float, lda: lapack_int,
                               b: *mut c_float, ldb: lapack_int, q: *mut c_float, ldq: lapack_int,
                               z: *mut c_float, ldz: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dgghrd_work(matrix_layout: c_int, compq: c_char, compz: c_char, n: lapack_int,
                               ilo: lapack_int, ihi: lapack_int, a: *mut c_double, lda: lapack_int,
                               b: *mut c_double, ldb: lapack_int, q: *mut c_double,
                               ldq: lapack_int, z: *mut c_double, ldz: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_cgghrd_work(matrix_layout: c_int, compq: c_char, compz: c_char, n: lapack_int,
                               ilo: lapack_int, ihi: lapack_int, a: *mut lapack_complex_float,
                               lda: lapack_int, b: *mut lapack_complex_float, ldb: lapack_int,
                               q: *mut lapack_complex_float, ldq: lapack_int,
                               z: *mut lapack_complex_float, ldz: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_zgghrd_work(matrix_layout: c_int, compq: c_char, compz: c_char, n: lapack_int,
                               ilo: lapack_int, ihi: lapack_int, a: *mut lapack_complex_double,
                               lda: lapack_int, b: *mut lapack_complex_double, ldb: lapack_int,
                               q: *mut lapack_complex_double, ldq: lapack_int,
                               z: *mut lapack_complex_double, ldz: lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_sgghd3_work(matrix_layout: c_int, compq: c_char, compz: c_char, n: lapack_int,
                               ilo: lapack_int, ihi: lapack_int, a: *mut c_float, lda: lapack_int,
                               b: *mut c_float, ldb: lapack_int, q: *mut c_float, ldq: lapack_int,
                               z: *mut c_float, ldz: lapack_int, work: *mut c_float,
                               lwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dgghd3_work(matrix_layout: c_int, compq: c_char, compz: c_char, n: lapack_int,
                               ilo: lapack_int, ihi: lapack_int, a: *mut c_double, lda: lapack_int,
                               b: *mut c_double, ldb: lapack_int, q: *mut c_double,
                               ldq: lapack_int, z: *mut c_double, ldz: lapack_int,
                               work: *mut c_double, lwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_cgghd3_work(matrix_layout: c_int, compq: c_char, compz: c_char, n: lapack_int,
                               ilo: lapack_int, ihi: lapack_int, a: *mut lapack_complex_float,
                               lda: lapack_int, b: *mut lapack_complex_float, ldb: lapack_int,
                               q: *mut lapack_complex_float, ldq: lapack_int,
                               z: *mut lapack_complex_float, ldz: lapack_int,
                               work: *mut lapack_complex_float, lwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_zgghd3_work(matrix_layout: c_int, compq: c_char, compz: c_char, n: lapack_int,
                               ilo: lapack_int, ihi: lapack_int, a: *mut lapack_complex_double,
                               lda: lapack_int, b: *mut lapack_complex_double, ldb: lapack_int,
                               q: *mut lapack_complex_double, ldq: lapack_int,
                               z: *mut lapack_complex_double, ldz: lapack_int,
                               work: *mut lapack_complex_double, lwork: lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_sgglse_work(matrix_layout: c_int, m: lapack_int, n: lapack_int, p: lapack_int,
                               a: *mut c_float, lda: lapack_int, b: *mut c_float, ldb: lapack_int,
                               c: *mut c_float, d: *mut c_float, x: *mut c_float,
                               work: *mut c_float, lwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dgglse_work(matrix_layout: c_int, m: lapack_int, n: lapack_int, p: lapack_int,
                               a: *mut c_double, lda: lapack_int, b: *mut c_double,
                               ldb: lapack_int, c: *mut c_double, d: *mut c_double,
                               x: *mut c_double, work: *mut c_double, lwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_cgglse_work(matrix_layout: c_int, m: lapack_int, n: lapack_int, p: lapack_int,
                               a: *mut lapack_complex_float, lda: lapack_int,
                               b: *mut lapack_complex_float, ldb: lapack_int,
                               c: *mut lapack_complex_float, d: *mut lapack_complex_float,
                               x: *mut lapack_complex_float, work: *mut lapack_complex_float,
                               lwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_zgglse_work(matrix_layout: c_int, m: lapack_int, n: lapack_int, p: lapack_int,
                               a: *mut lapack_complex_double, lda: lapack_int,
                               b: *mut lapack_complex_double, ldb: lapack_int,
                               c: *mut lapack_complex_double, d: *mut lapack_complex_double,
                               x: *mut lapack_complex_double, work: *mut lapack_complex_double,
                               lwork: lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_sggqrf_work(matrix_layout: c_int, n: lapack_int, m: lapack_int, p: lapack_int,
                               a: *mut c_float, lda: lapack_int, taua: *mut c_float,
                               b: *mut c_float, ldb: lapack_int, taub: *mut c_float,
                               work: *mut c_float, lwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dggqrf_work(matrix_layout: c_int, n: lapack_int, m: lapack_int, p: lapack_int,
                               a: *mut c_double, lda: lapack_int, taua: *mut c_double,
                               b: *mut c_double, ldb: lapack_int, taub: *mut c_double,
                               work: *mut c_double, lwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_cggqrf_work(matrix_layout: c_int, n: lapack_int, m: lapack_int, p: lapack_int,
                               a: *mut lapack_complex_float, lda: lapack_int,
                               taua: *mut lapack_complex_float, b: *mut lapack_complex_float,
                               ldb: lapack_int, taub: *mut lapack_complex_float,
                               work: *mut lapack_complex_float, lwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_zggqrf_work(matrix_layout: c_int, n: lapack_int, m: lapack_int, p: lapack_int,
                               a: *mut lapack_complex_double, lda: lapack_int,
                               taua: *mut lapack_complex_double, b: *mut lapack_complex_double,
                               ldb: lapack_int, taub: *mut lapack_complex_double,
                               work: *mut lapack_complex_double, lwork: lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_sggrqf_work(matrix_layout: c_int, m: lapack_int, p: lapack_int, n: lapack_int,
                               a: *mut c_float, lda: lapack_int, taua: *mut c_float,
                               b: *mut c_float, ldb: lapack_int, taub: *mut c_float,
                               work: *mut c_float, lwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dggrqf_work(matrix_layout: c_int, m: lapack_int, p: lapack_int, n: lapack_int,
                               a: *mut c_double, lda: lapack_int, taua: *mut c_double,
                               b: *mut c_double, ldb: lapack_int, taub: *mut c_double,
                               work: *mut c_double, lwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_cggrqf_work(matrix_layout: c_int, m: lapack_int, p: lapack_int, n: lapack_int,
                               a: *mut lapack_complex_float, lda: lapack_int,
                               taua: *mut lapack_complex_float, b: *mut lapack_complex_float,
                               ldb: lapack_int, taub: *mut lapack_complex_float,
                               work: *mut lapack_complex_float, lwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_zggrqf_work(matrix_layout: c_int, m: lapack_int, p: lapack_int, n: lapack_int,
                               a: *mut lapack_complex_double, lda: lapack_int,
                               taua: *mut lapack_complex_double, b: *mut lapack_complex_double,
                               ldb: lapack_int, taub: *mut lapack_complex_double,
                               work: *mut lapack_complex_double, lwork: lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_sggsvd_work(matrix_layout: c_int, jobu: c_char, jobv: c_char, jobq: c_char,
                               m: lapack_int, n: lapack_int, p: lapack_int, k: *mut lapack_int,
                               l: *mut lapack_int, a: *mut c_float, lda: lapack_int,
                               b: *mut c_float, ldb: lapack_int, alpha: *mut c_float,
                               beta: *mut c_float, u: *mut c_float, ldu: lapack_int,
                               v: *mut c_float, ldv: lapack_int, q: *mut c_float, ldq: lapack_int,
                               work: *mut c_float, iwork: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dggsvd_work(matrix_layout: c_int, jobu: c_char, jobv: c_char, jobq: c_char,
                               m: lapack_int, n: lapack_int, p: lapack_int, k: *mut lapack_int,
                               l: *mut lapack_int, a: *mut c_double, lda: lapack_int,
                               b: *mut c_double, ldb: lapack_int, alpha: *mut c_double,
                               beta: *mut c_double, u: *mut c_double, ldu: lapack_int,
                               v: *mut c_double, ldv: lapack_int, q: *mut c_double,
                               ldq: lapack_int, work: *mut c_double, iwork: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_cggsvd_work(matrix_layout: c_int, jobu: c_char, jobv: c_char, jobq: c_char,
                               m: lapack_int, n: lapack_int, p: lapack_int, k: *mut lapack_int,
                               l: *mut lapack_int, a: *mut lapack_complex_float, lda: lapack_int,
                               b: *mut lapack_complex_float, ldb: lapack_int, alpha: *mut c_float,
                               beta: *mut c_float, u: *mut lapack_complex_float, ldu: lapack_int,
                               v: *mut lapack_complex_float, ldv: lapack_int,
                               q: *mut lapack_complex_float, ldq: lapack_int,
                               work: *mut lapack_complex_float, rwork: *mut c_float,
                               iwork: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_zggsvd_work(matrix_layout: c_int, jobu: c_char, jobv: c_char, jobq: c_char,
                               m: lapack_int, n: lapack_int, p: lapack_int, k: *mut lapack_int,
                               l: *mut lapack_int, a: *mut lapack_complex_double, lda: lapack_int,
                               b: *mut lapack_complex_double, ldb: lapack_int,
                               alpha: *mut c_double, beta: *mut c_double,
                               u: *mut lapack_complex_double, ldu: lapack_int,
                               v: *mut lapack_complex_double, ldv: lapack_int,
                               q: *mut lapack_complex_double, ldq: lapack_int,
                               work: *mut lapack_complex_double, rwork: *mut c_double,
                               iwork: *mut lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_sggsvd3_work(matrix_layout: c_int, jobu: c_char, jobv: c_char, jobq: c_char,
                                m: lapack_int, n: lapack_int, p: lapack_int, k: *mut lapack_int,
                                l: *mut lapack_int, a: *mut c_float, lda: lapack_int,
                                b: *mut c_float, ldb: lapack_int, alpha: *mut c_float,
                                beta: *mut c_float, u: *mut c_float, ldu: lapack_int,
                                v: *mut c_float, ldv: lapack_int, q: *mut c_float, ldq: lapack_int,
                                work: *mut c_float, lwork: lapack_int, iwork: *mut lapack_int)
                                -> lapack_int;
    pub fn LAPACKE_dggsvd3_work(matrix_layout: c_int, jobu: c_char, jobv: c_char, jobq: c_char,
                                m: lapack_int, n: lapack_int, p: lapack_int, k: *mut lapack_int,
                                l: *mut lapack_int, a: *mut c_double, lda: lapack_int,
                                b: *mut c_double, ldb: lapack_int, alpha: *mut c_double,
                                beta: *mut c_double, u: *mut c_double, ldu: lapack_int,
                                v: *mut c_double, ldv: lapack_int, q: *mut c_double,
                                ldq: lapack_int, work: *mut c_double, lwork: lapack_int,
                                iwork: *mut lapack_int)
                                -> lapack_int;
    pub fn LAPACKE_cggsvd3_work(matrix_layout: c_int, jobu: c_char, jobv: c_char, jobq: c_char,
                                m: lapack_int, n: lapack_int, p: lapack_int, k: *mut lapack_int,
                                l: *mut lapack_int, a: *mut lapack_complex_float, lda: lapack_int,
                                b: *mut lapack_complex_float, ldb: lapack_int, alpha: *mut c_float,
                                beta: *mut c_float, u: *mut lapack_complex_float, ldu: lapack_int,
                                v: *mut lapack_complex_float, ldv: lapack_int,
                                q: *mut lapack_complex_float, ldq: lapack_int,
                                work: *mut lapack_complex_float, lwork: lapack_int,
                                rwork: *mut c_float, iwork: *mut lapack_int)
                                -> lapack_int;
    pub fn LAPACKE_zggsvd3_work(matrix_layout: c_int, jobu: c_char, jobv: c_char, jobq: c_char,
                                m: lapack_int, n: lapack_int, p: lapack_int, k: *mut lapack_int,
                                l: *mut lapack_int, a: *mut lapack_complex_double, lda: lapack_int,
                                b: *mut lapack_complex_double, ldb: lapack_int,
                                alpha: *mut c_double, beta: *mut c_double,
                                u: *mut lapack_complex_double, ldu: lapack_int,
                                v: *mut lapack_complex_double, ldv: lapack_int,
                                q: *mut lapack_complex_double, ldq: lapack_int,
                                work: *mut lapack_complex_double, lwork: lapack_int,
                                rwork: *mut c_double, iwork: *mut lapack_int)
                                -> lapack_int;

    pub fn LAPACKE_sggsvp_work(matrix_layout: c_int, jobu: c_char, jobv: c_char, jobq: c_char,
                               m: lapack_int, p: lapack_int, n: lapack_int, a: *mut c_float,
                               lda: lapack_int, b: *mut c_float, ldb: lapack_int, tola: c_float,
                               tolb: c_float, k: *mut lapack_int, l: *mut lapack_int,
                               u: *mut c_float, ldu: lapack_int, v: *mut c_float, ldv: lapack_int,
                               q: *mut c_float, ldq: lapack_int, iwork: *mut lapack_int,
                               tau: *mut c_float, work: *mut c_float)
                               -> lapack_int;
    pub fn LAPACKE_dggsvp_work(matrix_layout: c_int, jobu: c_char, jobv: c_char, jobq: c_char,
                               m: lapack_int, p: lapack_int, n: lapack_int, a: *mut c_double,
                               lda: lapack_int, b: *mut c_double, ldb: lapack_int, tola: c_double,
                               tolb: c_double, k: *mut lapack_int, l: *mut lapack_int,
                               u: *mut c_double, ldu: lapack_int, v: *mut c_double,
                               ldv: lapack_int, q: *mut c_double, ldq: lapack_int,
                               iwork: *mut lapack_int, tau: *mut c_double, work: *mut c_double)
                               -> lapack_int;
    pub fn LAPACKE_cggsvp_work(matrix_layout: c_int, jobu: c_char, jobv: c_char, jobq: c_char,
                               m: lapack_int, p: lapack_int, n: lapack_int,
                               a: *mut lapack_complex_float, lda: lapack_int,
                               b: *mut lapack_complex_float, ldb: lapack_int, tola: c_float,
                               tolb: c_float, k: *mut lapack_int, l: *mut lapack_int,
                               u: *mut lapack_complex_float, ldu: lapack_int,
                               v: *mut lapack_complex_float, ldv: lapack_int,
                               q: *mut lapack_complex_float, ldq: lapack_int,
                               iwork: *mut lapack_int, rwork: *mut c_float,
                               tau: *mut lapack_complex_float, work: *mut lapack_complex_float)
                               -> lapack_int;
    pub fn LAPACKE_zggsvp_work(matrix_layout: c_int, jobu: c_char, jobv: c_char, jobq: c_char,
                               m: lapack_int, p: lapack_int, n: lapack_int,
                               a: *mut lapack_complex_double, lda: lapack_int,
                               b: *mut lapack_complex_double, ldb: lapack_int, tola: c_double,
                               tolb: c_double, k: *mut lapack_int, l: *mut lapack_int,
                               u: *mut lapack_complex_double, ldu: lapack_int,
                               v: *mut lapack_complex_double, ldv: lapack_int,
                               q: *mut lapack_complex_double, ldq: lapack_int,
                               iwork: *mut lapack_int, rwork: *mut c_double,
                               tau: *mut lapack_complex_double, work: *mut lapack_complex_double)
                               -> lapack_int;

    pub fn LAPACKE_sggsvp3_work(matrix_layout: c_int, jobu: c_char, jobv: c_char, jobq: c_char,
                                m: lapack_int, p: lapack_int, n: lapack_int, a: *mut c_float,
                                lda: lapack_int, b: *mut c_float, ldb: lapack_int, tola: c_float,
                                tolb: c_float, k: *mut lapack_int, l: *mut lapack_int,
                                u: *mut c_float, ldu: lapack_int, v: *mut c_float, ldv: lapack_int,
                                q: *mut c_float, ldq: lapack_int, iwork: *mut lapack_int,
                                tau: *mut c_float, work: *mut c_float, lwork: lapack_int)
                                -> lapack_int;
    pub fn LAPACKE_dggsvp3_work(matrix_layout: c_int, jobu: c_char, jobv: c_char, jobq: c_char,
                                m: lapack_int, p: lapack_int, n: lapack_int, a: *mut c_double,
                                lda: lapack_int, b: *mut c_double, ldb: lapack_int, tola: c_double,
                                tolb: c_double, k: *mut lapack_int, l: *mut lapack_int,
                                u: *mut c_double, ldu: lapack_int, v: *mut c_double,
                                ldv: lapack_int, q: *mut c_double, ldq: lapack_int,
                                iwork: *mut lapack_int, tau: *mut c_double, work: *mut c_double,
                                lwork: lapack_int)
                                -> lapack_int;
    pub fn LAPACKE_cggsvp3_work(matrix_layout: c_int, jobu: c_char, jobv: c_char, jobq: c_char,
                                m: lapack_int, p: lapack_int, n: lapack_int,
                                a: *mut lapack_complex_float, lda: lapack_int,
                                b: *mut lapack_complex_float, ldb: lapack_int, tola: c_float,
                                tolb: c_float, k: *mut lapack_int, l: *mut lapack_int,
                                u: *mut lapack_complex_float, ldu: lapack_int,
                                v: *mut lapack_complex_float, ldv: lapack_int,
                                q: *mut lapack_complex_float, ldq: lapack_int,
                                iwork: *mut lapack_int, rwork: *mut c_float,
                                tau: *mut lapack_complex_float, work: *mut lapack_complex_float,
                                lwork: lapack_int)
                                -> lapack_int;
    pub fn LAPACKE_zggsvp3_work(matrix_layout: c_int, jobu: c_char, jobv: c_char, jobq: c_char,
                                m: lapack_int, p: lapack_int, n: lapack_int,
                                a: *mut lapack_complex_double, lda: lapack_int,
                                b: *mut lapack_complex_double, ldb: lapack_int, tola: c_double,
                                tolb: c_double, k: *mut lapack_int, l: *mut lapack_int,
                                u: *mut lapack_complex_double, ldu: lapack_int,
                                v: *mut lapack_complex_double, ldv: lapack_int,
                                q: *mut lapack_complex_double, ldq: lapack_int,
                                iwork: *mut lapack_int, rwork: *mut c_double,
                                tau: *mut lapack_complex_double, work: *mut lapack_complex_double,
                                lwork: lapack_int)
                                -> lapack_int;

    pub fn LAPACKE_sgtcon_work(norm: c_char, n: lapack_int, dl: *const c_float, d: *const c_float,
                               du: *const c_float, du2: *const c_float, ipiv: *const lapack_int,
                               anorm: c_float, rcond: *mut c_float, work: *mut c_float,
                               iwork: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dgtcon_work(norm: c_char, n: lapack_int, dl: *const c_double,
                               d: *const c_double, du: *const c_double, du2: *const c_double,
                               ipiv: *const lapack_int, anorm: c_double, rcond: *mut c_double,
                               work: *mut c_double, iwork: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_cgtcon_work(norm: c_char, n: lapack_int, dl: *const lapack_complex_float,
                               d: *const lapack_complex_float, du: *const lapack_complex_float,
                               du2: *const lapack_complex_float, ipiv: *const lapack_int,
                               anorm: c_float, rcond: *mut c_float,
                               work: *mut lapack_complex_float)
                               -> lapack_int;
    pub fn LAPACKE_zgtcon_work(norm: c_char, n: lapack_int, dl: *const lapack_complex_double,
                               d: *const lapack_complex_double, du: *const lapack_complex_double,
                               du2: *const lapack_complex_double, ipiv: *const lapack_int,
                               anorm: c_double, rcond: *mut c_double,
                               work: *mut lapack_complex_double)
                               -> lapack_int;

    pub fn LAPACKE_sgtrfs_work(matrix_layout: c_int, trans: c_char, n: lapack_int,
                               nrhs: lapack_int, dl: *const c_float, d: *const c_float,
                               du: *const c_float, dlf: *const c_float, df: *const c_float,
                               duf: *const c_float, du2: *const c_float, ipiv: *const lapack_int,
                               b: *const c_float, ldb: lapack_int, x: *mut c_float,
                               ldx: lapack_int, ferr: *mut c_float, berr: *mut c_float,
                               work: *mut c_float, iwork: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dgtrfs_work(matrix_layout: c_int, trans: c_char, n: lapack_int,
                               nrhs: lapack_int, dl: *const c_double, d: *const c_double,
                               du: *const c_double, dlf: *const c_double, df: *const c_double,
                               duf: *const c_double, du2: *const c_double, ipiv: *const lapack_int,
                               b: *const c_double, ldb: lapack_int, x: *mut c_double,
                               ldx: lapack_int, ferr: *mut c_double, berr: *mut c_double,
                               work: *mut c_double, iwork: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_cgtrfs_work(matrix_layout: c_int, trans: c_char, n: lapack_int,
                               nrhs: lapack_int, dl: *const lapack_complex_float,
                               d: *const lapack_complex_float, du: *const lapack_complex_float,
                               dlf: *const lapack_complex_float, df: *const lapack_complex_float,
                               duf: *const lapack_complex_float, du2: *const lapack_complex_float,
                               ipiv: *const lapack_int, b: *const lapack_complex_float,
                               ldb: lapack_int, x: *mut lapack_complex_float, ldx: lapack_int,
                               ferr: *mut c_float, berr: *mut c_float,
                               work: *mut lapack_complex_float, rwork: *mut c_float)
                               -> lapack_int;
    pub fn LAPACKE_zgtrfs_work(matrix_layout: c_int, trans: c_char, n: lapack_int,
                               nrhs: lapack_int, dl: *const lapack_complex_double,
                               d: *const lapack_complex_double, du: *const lapack_complex_double,
                               dlf: *const lapack_complex_double, df: *const lapack_complex_double,
                               duf: *const lapack_complex_double,
                               du2: *const lapack_complex_double, ipiv: *const lapack_int,
                               b: *const lapack_complex_double, ldb: lapack_int,
                               x: *mut lapack_complex_double, ldx: lapack_int, ferr: *mut c_double,
                               berr: *mut c_double, work: *mut lapack_complex_double,
                               rwork: *mut c_double)
                               -> lapack_int;

    pub fn LAPACKE_sgtsv_work(matrix_layout: c_int, n: lapack_int, nrhs: lapack_int,
                              dl: *mut c_float, d: *mut c_float, du: *mut c_float, b: *mut c_float,
                              ldb: lapack_int)
                              -> lapack_int;
    pub fn LAPACKE_dgtsv_work(matrix_layout: c_int, n: lapack_int, nrhs: lapack_int,
                              dl: *mut c_double, d: *mut c_double, du: *mut c_double,
                              b: *mut c_double, ldb: lapack_int)
                              -> lapack_int;
    pub fn LAPACKE_cgtsv_work(matrix_layout: c_int, n: lapack_int, nrhs: lapack_int,
                              dl: *mut lapack_complex_float, d: *mut lapack_complex_float,
                              du: *mut lapack_complex_float, b: *mut lapack_complex_float,
                              ldb: lapack_int)
                              -> lapack_int;
    pub fn LAPACKE_zgtsv_work(matrix_layout: c_int, n: lapack_int, nrhs: lapack_int,
                              dl: *mut lapack_complex_double, d: *mut lapack_complex_double,
                              du: *mut lapack_complex_double, b: *mut lapack_complex_double,
                              ldb: lapack_int)
                              -> lapack_int;

    pub fn LAPACKE_sgtsvx_work(matrix_layout: c_int, fact: c_char, trans: c_char, n: lapack_int,
                               nrhs: lapack_int, dl: *const c_float, d: *const c_float,
                               du: *const c_float, dlf: *mut c_float, df: *mut c_float,
                               duf: *mut c_float, du2: *mut c_float, ipiv: *mut lapack_int,
                               b: *const c_float, ldb: lapack_int, x: *mut c_float,
                               ldx: lapack_int, rcond: *mut c_float, ferr: *mut c_float,
                               berr: *mut c_float, work: *mut c_float, iwork: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dgtsvx_work(matrix_layout: c_int, fact: c_char, trans: c_char, n: lapack_int,
                               nrhs: lapack_int, dl: *const c_double, d: *const c_double,
                               du: *const c_double, dlf: *mut c_double, df: *mut c_double,
                               duf: *mut c_double, du2: *mut c_double, ipiv: *mut lapack_int,
                               b: *const c_double, ldb: lapack_int, x: *mut c_double,
                               ldx: lapack_int, rcond: *mut c_double, ferr: *mut c_double,
                               berr: *mut c_double, work: *mut c_double, iwork: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_cgtsvx_work(matrix_layout: c_int, fact: c_char, trans: c_char, n: lapack_int,
                               nrhs: lapack_int, dl: *const lapack_complex_float,
                               d: *const lapack_complex_float, du: *const lapack_complex_float,
                               dlf: *mut lapack_complex_float, df: *mut lapack_complex_float,
                               duf: *mut lapack_complex_float, du2: *mut lapack_complex_float,
                               ipiv: *mut lapack_int, b: *const lapack_complex_float,
                               ldb: lapack_int, x: *mut lapack_complex_float, ldx: lapack_int,
                               rcond: *mut c_float, ferr: *mut c_float, berr: *mut c_float,
                               work: *mut lapack_complex_float, rwork: *mut c_float)
                               -> lapack_int;
    pub fn LAPACKE_zgtsvx_work(matrix_layout: c_int, fact: c_char, trans: c_char, n: lapack_int,
                               nrhs: lapack_int, dl: *const lapack_complex_double,
                               d: *const lapack_complex_double, du: *const lapack_complex_double,
                               dlf: *mut lapack_complex_double, df: *mut lapack_complex_double,
                               duf: *mut lapack_complex_double, du2: *mut lapack_complex_double,
                               ipiv: *mut lapack_int, b: *const lapack_complex_double,
                               ldb: lapack_int, x: *mut lapack_complex_double, ldx: lapack_int,
                               rcond: *mut c_double, ferr: *mut c_double, berr: *mut c_double,
                               work: *mut lapack_complex_double, rwork: *mut c_double)
                               -> lapack_int;

    pub fn LAPACKE_sgttrf_work(n: lapack_int, dl: *mut c_float, d: *mut c_float, du: *mut c_float,
                               du2: *mut c_float, ipiv: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dgttrf_work(n: lapack_int, dl: *mut c_double, d: *mut c_double,
                               du: *mut c_double, du2: *mut c_double, ipiv: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_cgttrf_work(n: lapack_int, dl: *mut lapack_complex_float,
                               d: *mut lapack_complex_float, du: *mut lapack_complex_float,
                               du2: *mut lapack_complex_float, ipiv: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_zgttrf_work(n: lapack_int, dl: *mut lapack_complex_double,
                               d: *mut lapack_complex_double, du: *mut lapack_complex_double,
                               du2: *mut lapack_complex_double, ipiv: *mut lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_sgttrs_work(matrix_layout: c_int, trans: c_char, n: lapack_int,
                               nrhs: lapack_int, dl: *const c_float, d: *const c_float,
                               du: *const c_float, du2: *const c_float, ipiv: *const lapack_int,
                               b: *mut c_float, ldb: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dgttrs_work(matrix_layout: c_int, trans: c_char, n: lapack_int,
                               nrhs: lapack_int, dl: *const c_double, d: *const c_double,
                               du: *const c_double, du2: *const c_double, ipiv: *const lapack_int,
                               b: *mut c_double, ldb: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_cgttrs_work(matrix_layout: c_int, trans: c_char, n: lapack_int,
                               nrhs: lapack_int, dl: *const lapack_complex_float,
                               d: *const lapack_complex_float, du: *const lapack_complex_float,
                               du2: *const lapack_complex_float, ipiv: *const lapack_int,
                               b: *mut lapack_complex_float, ldb: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_zgttrs_work(matrix_layout: c_int, trans: c_char, n: lapack_int,
                               nrhs: lapack_int, dl: *const lapack_complex_double,
                               d: *const lapack_complex_double, du: *const lapack_complex_double,
                               du2: *const lapack_complex_double, ipiv: *const lapack_int,
                               b: *mut lapack_complex_double, ldb: lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_chbev_work(matrix_layout: c_int, jobz: c_char, uplo: c_char, n: lapack_int,
                              kd: lapack_int, ab: *mut lapack_complex_float, ldab: lapack_int,
                              w: *mut c_float, z: *mut lapack_complex_float, ldz: lapack_int,
                              work: *mut lapack_complex_float, rwork: *mut c_float)
                              -> lapack_int;
    pub fn LAPACKE_zhbev_work(matrix_layout: c_int, jobz: c_char, uplo: c_char, n: lapack_int,
                              kd: lapack_int, ab: *mut lapack_complex_double, ldab: lapack_int,
                              w: *mut c_double, z: *mut lapack_complex_double, ldz: lapack_int,
                              work: *mut lapack_complex_double, rwork: *mut c_double)
                              -> lapack_int;

    pub fn LAPACKE_chbevd_work(matrix_layout: c_int, jobz: c_char, uplo: c_char, n: lapack_int,
                               kd: lapack_int, ab: *mut lapack_complex_float, ldab: lapack_int,
                               w: *mut c_float, z: *mut lapack_complex_float, ldz: lapack_int,
                               work: *mut lapack_complex_float, lwork: lapack_int,
                               rwork: *mut c_float, lrwork: lapack_int, iwork: *mut lapack_int,
                               liwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_zhbevd_work(matrix_layout: c_int, jobz: c_char, uplo: c_char, n: lapack_int,
                               kd: lapack_int, ab: *mut lapack_complex_double, ldab: lapack_int,
                               w: *mut c_double, z: *mut lapack_complex_double, ldz: lapack_int,
                               work: *mut lapack_complex_double, lwork: lapack_int,
                               rwork: *mut c_double, lrwork: lapack_int, iwork: *mut lapack_int,
                               liwork: lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_chbevx_work(matrix_layout: c_int, jobz: c_char, range: c_char, uplo: c_char,
                               n: lapack_int, kd: lapack_int, ab: *mut lapack_complex_float,
                               ldab: lapack_int, q: *mut lapack_complex_float, ldq: lapack_int,
                               vl: c_float, vu: c_float, il: lapack_int, iu: lapack_int,
                               abstol: c_float, m: *mut lapack_int, w: *mut c_float,
                               z: *mut lapack_complex_float, ldz: lapack_int,
                               work: *mut lapack_complex_float, rwork: *mut c_float,
                               iwork: *mut lapack_int, ifail: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_zhbevx_work(matrix_layout: c_int, jobz: c_char, range: c_char, uplo: c_char,
                               n: lapack_int, kd: lapack_int, ab: *mut lapack_complex_double,
                               ldab: lapack_int, q: *mut lapack_complex_double, ldq: lapack_int,
                               vl: c_double, vu: c_double, il: lapack_int, iu: lapack_int,
                               abstol: c_double, m: *mut lapack_int, w: *mut c_double,
                               z: *mut lapack_complex_double, ldz: lapack_int,
                               work: *mut lapack_complex_double, rwork: *mut c_double,
                               iwork: *mut lapack_int, ifail: *mut lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_chbgst_work(matrix_layout: c_int, vect: c_char, uplo: c_char, n: lapack_int,
                               ka: lapack_int, kb: lapack_int, ab: *mut lapack_complex_float,
                               ldab: lapack_int, bb: *const lapack_complex_float, ldbb: lapack_int,
                               x: *mut lapack_complex_float, ldx: lapack_int,
                               work: *mut lapack_complex_float, rwork: *mut c_float)
                               -> lapack_int;
    pub fn LAPACKE_zhbgst_work(matrix_layout: c_int, vect: c_char, uplo: c_char, n: lapack_int,
                               ka: lapack_int, kb: lapack_int, ab: *mut lapack_complex_double,
                               ldab: lapack_int, bb: *const lapack_complex_double,
                               ldbb: lapack_int, x: *mut lapack_complex_double, ldx: lapack_int,
                               work: *mut lapack_complex_double, rwork: *mut c_double)
                               -> lapack_int;

    pub fn LAPACKE_chbgv_work(matrix_layout: c_int, jobz: c_char, uplo: c_char, n: lapack_int,
                              ka: lapack_int, kb: lapack_int, ab: *mut lapack_complex_float,
                              ldab: lapack_int, bb: *mut lapack_complex_float, ldbb: lapack_int,
                              w: *mut c_float, z: *mut lapack_complex_float, ldz: lapack_int,
                              work: *mut lapack_complex_float, rwork: *mut c_float)
                              -> lapack_int;
    pub fn LAPACKE_zhbgv_work(matrix_layout: c_int, jobz: c_char, uplo: c_char, n: lapack_int,
                              ka: lapack_int, kb: lapack_int, ab: *mut lapack_complex_double,
                              ldab: lapack_int, bb: *mut lapack_complex_double, ldbb: lapack_int,
                              w: *mut c_double, z: *mut lapack_complex_double, ldz: lapack_int,
                              work: *mut lapack_complex_double, rwork: *mut c_double)
                              -> lapack_int;

    pub fn LAPACKE_chbgvd_work(matrix_layout: c_int, jobz: c_char, uplo: c_char, n: lapack_int,
                               ka: lapack_int, kb: lapack_int, ab: *mut lapack_complex_float,
                               ldab: lapack_int, bb: *mut lapack_complex_float, ldbb: lapack_int,
                               w: *mut c_float, z: *mut lapack_complex_float, ldz: lapack_int,
                               work: *mut lapack_complex_float, lwork: lapack_int,
                               rwork: *mut c_float, lrwork: lapack_int, iwork: *mut lapack_int,
                               liwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_zhbgvd_work(matrix_layout: c_int, jobz: c_char, uplo: c_char, n: lapack_int,
                               ka: lapack_int, kb: lapack_int, ab: *mut lapack_complex_double,
                               ldab: lapack_int, bb: *mut lapack_complex_double, ldbb: lapack_int,
                               w: *mut c_double, z: *mut lapack_complex_double, ldz: lapack_int,
                               work: *mut lapack_complex_double, lwork: lapack_int,
                               rwork: *mut c_double, lrwork: lapack_int, iwork: *mut lapack_int,
                               liwork: lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_chbgvx_work(matrix_layout: c_int, jobz: c_char, range: c_char, uplo: c_char,
                               n: lapack_int, ka: lapack_int, kb: lapack_int,
                               ab: *mut lapack_complex_float, ldab: lapack_int,
                               bb: *mut lapack_complex_float, ldbb: lapack_int,
                               q: *mut lapack_complex_float, ldq: lapack_int, vl: c_float,
                               vu: c_float, il: lapack_int, iu: lapack_int, abstol: c_float,
                               m: *mut lapack_int, w: *mut c_float, z: *mut lapack_complex_float,
                               ldz: lapack_int, work: *mut lapack_complex_float,
                               rwork: *mut c_float, iwork: *mut lapack_int, ifail: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_zhbgvx_work(matrix_layout: c_int, jobz: c_char, range: c_char, uplo: c_char,
                               n: lapack_int, ka: lapack_int, kb: lapack_int,
                               ab: *mut lapack_complex_double, ldab: lapack_int,
                               bb: *mut lapack_complex_double, ldbb: lapack_int,
                               q: *mut lapack_complex_double, ldq: lapack_int, vl: c_double,
                               vu: c_double, il: lapack_int, iu: lapack_int, abstol: c_double,
                               m: *mut lapack_int, w: *mut c_double, z: *mut lapack_complex_double,
                               ldz: lapack_int, work: *mut lapack_complex_double,
                               rwork: *mut c_double, iwork: *mut lapack_int,
                               ifail: *mut lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_chbtrd_work(matrix_layout: c_int, vect: c_char, uplo: c_char, n: lapack_int,
                               kd: lapack_int, ab: *mut lapack_complex_float, ldab: lapack_int,
                               d: *mut c_float, e: *mut c_float, q: *mut lapack_complex_float,
                               ldq: lapack_int, work: *mut lapack_complex_float)
                               -> lapack_int;
    pub fn LAPACKE_zhbtrd_work(matrix_layout: c_int, vect: c_char, uplo: c_char, n: lapack_int,
                               kd: lapack_int, ab: *mut lapack_complex_double, ldab: lapack_int,
                               d: *mut c_double, e: *mut c_double, q: *mut lapack_complex_double,
                               ldq: lapack_int, work: *mut lapack_complex_double)
                               -> lapack_int;

    pub fn LAPACKE_checon_work(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                               a: *const lapack_complex_float, lda: lapack_int,
                               ipiv: *const lapack_int, anorm: c_float, rcond: *mut c_float,
                               work: *mut lapack_complex_float)
                               -> lapack_int;
    pub fn LAPACKE_zhecon_work(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                               a: *const lapack_complex_double, lda: lapack_int,
                               ipiv: *const lapack_int, anorm: c_double, rcond: *mut c_double,
                               work: *mut lapack_complex_double)
                               -> lapack_int;

    pub fn LAPACKE_cheequb_work(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                                a: *const lapack_complex_float, lda: lapack_int, s: *mut c_float,
                                scond: *mut c_float, amax: *mut c_float,
                                work: *mut lapack_complex_float)
                                -> lapack_int;
    pub fn LAPACKE_zheequb_work(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                                a: *const lapack_complex_double, lda: lapack_int, s: *mut c_double,
                                scond: *mut c_double, amax: *mut c_double,
                                work: *mut lapack_complex_double)
                                -> lapack_int;

    pub fn LAPACKE_cheev_work(matrix_layout: c_int, jobz: c_char, uplo: c_char, n: lapack_int,
                              a: *mut lapack_complex_float, lda: lapack_int, w: *mut c_float,
                              work: *mut lapack_complex_float, lwork: lapack_int,
                              rwork: *mut c_float)
                              -> lapack_int;
    pub fn LAPACKE_zheev_work(matrix_layout: c_int, jobz: c_char, uplo: c_char, n: lapack_int,
                              a: *mut lapack_complex_double, lda: lapack_int, w: *mut c_double,
                              work: *mut lapack_complex_double, lwork: lapack_int,
                              rwork: *mut c_double)
                              -> lapack_int;

    pub fn LAPACKE_cheevd_work(matrix_layout: c_int, jobz: c_char, uplo: c_char, n: lapack_int,
                               a: *mut lapack_complex_float, lda: lapack_int, w: *mut c_float,
                               work: *mut lapack_complex_float, lwork: lapack_int,
                               rwork: *mut c_float, lrwork: lapack_int, iwork: *mut lapack_int,
                               liwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_zheevd_work(matrix_layout: c_int, jobz: c_char, uplo: c_char, n: lapack_int,
                               a: *mut lapack_complex_double, lda: lapack_int, w: *mut c_double,
                               work: *mut lapack_complex_double, lwork: lapack_int,
                               rwork: *mut c_double, lrwork: lapack_int, iwork: *mut lapack_int,
                               liwork: lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_cheevr_work(matrix_layout: c_int, jobz: c_char, range: c_char, uplo: c_char,
                               n: lapack_int, a: *mut lapack_complex_float, lda: lapack_int,
                               vl: c_float, vu: c_float, il: lapack_int, iu: lapack_int,
                               abstol: c_float, m: *mut lapack_int, w: *mut c_float,
                               z: *mut lapack_complex_float, ldz: lapack_int,
                               isuppz: *mut lapack_int, work: *mut lapack_complex_float,
                               lwork: lapack_int, rwork: *mut c_float, lrwork: lapack_int,
                               iwork: *mut lapack_int, liwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_zheevr_work(matrix_layout: c_int, jobz: c_char, range: c_char, uplo: c_char,
                               n: lapack_int, a: *mut lapack_complex_double, lda: lapack_int,
                               vl: c_double, vu: c_double, il: lapack_int, iu: lapack_int,
                               abstol: c_double, m: *mut lapack_int, w: *mut c_double,
                               z: *mut lapack_complex_double, ldz: lapack_int,
                               isuppz: *mut lapack_int, work: *mut lapack_complex_double,
                               lwork: lapack_int, rwork: *mut c_double, lrwork: lapack_int,
                               iwork: *mut lapack_int, liwork: lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_cheevx_work(matrix_layout: c_int, jobz: c_char, range: c_char, uplo: c_char,
                               n: lapack_int, a: *mut lapack_complex_float, lda: lapack_int,
                               vl: c_float, vu: c_float, il: lapack_int, iu: lapack_int,
                               abstol: c_float, m: *mut lapack_int, w: *mut c_float,
                               z: *mut lapack_complex_float, ldz: lapack_int,
                               work: *mut lapack_complex_float, lwork: lapack_int,
                               rwork: *mut c_float, iwork: *mut lapack_int, ifail: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_zheevx_work(matrix_layout: c_int, jobz: c_char, range: c_char, uplo: c_char,
                               n: lapack_int, a: *mut lapack_complex_double, lda: lapack_int,
                               vl: c_double, vu: c_double, il: lapack_int, iu: lapack_int,
                               abstol: c_double, m: *mut lapack_int, w: *mut c_double,
                               z: *mut lapack_complex_double, ldz: lapack_int,
                               work: *mut lapack_complex_double, lwork: lapack_int,
                               rwork: *mut c_double, iwork: *mut lapack_int,
                               ifail: *mut lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_chegst_work(matrix_layout: c_int, itype: lapack_int, uplo: c_char,
                               n: lapack_int, a: *mut lapack_complex_float, lda: lapack_int,
                               b: *const lapack_complex_float, ldb: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_zhegst_work(matrix_layout: c_int, itype: lapack_int, uplo: c_char,
                               n: lapack_int, a: *mut lapack_complex_double, lda: lapack_int,
                               b: *const lapack_complex_double, ldb: lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_chegv_work(matrix_layout: c_int, itype: lapack_int, jobz: c_char, uplo: c_char,
                              n: lapack_int, a: *mut lapack_complex_float, lda: lapack_int,
                              b: *mut lapack_complex_float, ldb: lapack_int, w: *mut c_float,
                              work: *mut lapack_complex_float, lwork: lapack_int,
                              rwork: *mut c_float)
                              -> lapack_int;
    pub fn LAPACKE_zhegv_work(matrix_layout: c_int, itype: lapack_int, jobz: c_char, uplo: c_char,
                              n: lapack_int, a: *mut lapack_complex_double, lda: lapack_int,
                              b: *mut lapack_complex_double, ldb: lapack_int, w: *mut c_double,
                              work: *mut lapack_complex_double, lwork: lapack_int,
                              rwork: *mut c_double)
                              -> lapack_int;

    pub fn LAPACKE_chegvd_work(matrix_layout: c_int, itype: lapack_int, jobz: c_char, uplo: c_char,
                               n: lapack_int, a: *mut lapack_complex_float, lda: lapack_int,
                               b: *mut lapack_complex_float, ldb: lapack_int, w: *mut c_float,
                               work: *mut lapack_complex_float, lwork: lapack_int,
                               rwork: *mut c_float, lrwork: lapack_int, iwork: *mut lapack_int,
                               liwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_zhegvd_work(matrix_layout: c_int, itype: lapack_int, jobz: c_char, uplo: c_char,
                               n: lapack_int, a: *mut lapack_complex_double, lda: lapack_int,
                               b: *mut lapack_complex_double, ldb: lapack_int, w: *mut c_double,
                               work: *mut lapack_complex_double, lwork: lapack_int,
                               rwork: *mut c_double, lrwork: lapack_int, iwork: *mut lapack_int,
                               liwork: lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_chegvx_work(matrix_layout: c_int, itype: lapack_int, jobz: c_char,
                               range: c_char, uplo: c_char, n: lapack_int,
                               a: *mut lapack_complex_float, lda: lapack_int,
                               b: *mut lapack_complex_float, ldb: lapack_int, vl: c_float,
                               vu: c_float, il: lapack_int, iu: lapack_int, abstol: c_float,
                               m: *mut lapack_int, w: *mut c_float, z: *mut lapack_complex_float,
                               ldz: lapack_int, work: *mut lapack_complex_float, lwork: lapack_int,
                               rwork: *mut c_float, iwork: *mut lapack_int, ifail: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_zhegvx_work(matrix_layout: c_int, itype: lapack_int, jobz: c_char,
                               range: c_char, uplo: c_char, n: lapack_int,
                               a: *mut lapack_complex_double, lda: lapack_int,
                               b: *mut lapack_complex_double, ldb: lapack_int, vl: c_double,
                               vu: c_double, il: lapack_int, iu: lapack_int, abstol: c_double,
                               m: *mut lapack_int, w: *mut c_double, z: *mut lapack_complex_double,
                               ldz: lapack_int, work: *mut lapack_complex_double,
                               lwork: lapack_int, rwork: *mut c_double, iwork: *mut lapack_int,
                               ifail: *mut lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_cherfs_work(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                               a: *const lapack_complex_float, lda: lapack_int,
                               af: *const lapack_complex_float, ldaf: lapack_int,
                               ipiv: *const lapack_int, b: *const lapack_complex_float,
                               ldb: lapack_int, x: *mut lapack_complex_float, ldx: lapack_int,
                               ferr: *mut c_float, berr: *mut c_float,
                               work: *mut lapack_complex_float, rwork: *mut c_float)
                               -> lapack_int;
    pub fn LAPACKE_zherfs_work(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                               a: *const lapack_complex_double, lda: lapack_int,
                               af: *const lapack_complex_double, ldaf: lapack_int,
                               ipiv: *const lapack_int, b: *const lapack_complex_double,
                               ldb: lapack_int, x: *mut lapack_complex_double, ldx: lapack_int,
                               ferr: *mut c_double, berr: *mut c_double,
                               work: *mut lapack_complex_double, rwork: *mut c_double)
                               -> lapack_int;

    pub fn LAPACKE_cherfsx_work(matrix_layout: c_int, uplo: c_char, equed: c_char, n: lapack_int,
                                nrhs: lapack_int, a: *const lapack_complex_float, lda: lapack_int,
                                af: *const lapack_complex_float, ldaf: lapack_int,
                                ipiv: *const lapack_int, s: *const c_float,
                                b: *const lapack_complex_float, ldb: lapack_int,
                                x: *mut lapack_complex_float, ldx: lapack_int, rcond: *mut c_float,
                                berr: *mut c_float, n_err_bnds: lapack_int,
                                err_bnds_norm: *mut c_float, err_bnds_comp: *mut c_float,
                                nparams: lapack_int, params: *mut c_float,
                                work: *mut lapack_complex_float, rwork: *mut c_float)
                                -> lapack_int;
    pub fn LAPACKE_zherfsx_work(matrix_layout: c_int, uplo: c_char, equed: c_char, n: lapack_int,
                                nrhs: lapack_int, a: *const lapack_complex_double, lda: lapack_int,
                                af: *const lapack_complex_double, ldaf: lapack_int,
                                ipiv: *const lapack_int, s: *const c_double,
                                b: *const lapack_complex_double, ldb: lapack_int,
                                x: *mut lapack_complex_double, ldx: lapack_int,
                                rcond: *mut c_double, berr: *mut c_double, n_err_bnds: lapack_int,
                                err_bnds_norm: *mut c_double, err_bnds_comp: *mut c_double,
                                nparams: lapack_int, params: *mut c_double,
                                work: *mut lapack_complex_double, rwork: *mut c_double)
                                -> lapack_int;

    pub fn LAPACKE_chesv_work(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                              a: *mut lapack_complex_float, lda: lapack_int, ipiv: *mut lapack_int,
                              b: *mut lapack_complex_float, ldb: lapack_int,
                              work: *mut lapack_complex_float, lwork: lapack_int)
                              -> lapack_int;
    pub fn LAPACKE_zhesv_work(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                              a: *mut lapack_complex_double, lda: lapack_int,
                              ipiv: *mut lapack_int, b: *mut lapack_complex_double,
                              ldb: lapack_int, work: *mut lapack_complex_double, lwork: lapack_int)
                              -> lapack_int;

    pub fn LAPACKE_chesvx_work(matrix_layout: c_int, fact: c_char, uplo: c_char, n: lapack_int,
                               nrhs: lapack_int, a: *const lapack_complex_float, lda: lapack_int,
                               af: *mut lapack_complex_float, ldaf: lapack_int,
                               ipiv: *mut lapack_int, b: *const lapack_complex_float,
                               ldb: lapack_int, x: *mut lapack_complex_float, ldx: lapack_int,
                               rcond: *mut c_float, ferr: *mut c_float, berr: *mut c_float,
                               work: *mut lapack_complex_float, lwork: lapack_int,
                               rwork: *mut c_float)
                               -> lapack_int;
    pub fn LAPACKE_zhesvx_work(matrix_layout: c_int, fact: c_char, uplo: c_char, n: lapack_int,
                               nrhs: lapack_int, a: *const lapack_complex_double, lda: lapack_int,
                               af: *mut lapack_complex_double, ldaf: lapack_int,
                               ipiv: *mut lapack_int, b: *const lapack_complex_double,
                               ldb: lapack_int, x: *mut lapack_complex_double, ldx: lapack_int,
                               rcond: *mut c_double, ferr: *mut c_double, berr: *mut c_double,
                               work: *mut lapack_complex_double, lwork: lapack_int,
                               rwork: *mut c_double)
                               -> lapack_int;

    pub fn LAPACKE_chesvxx_work(matrix_layout: c_int, fact: c_char, uplo: c_char, n: lapack_int,
                                nrhs: lapack_int, a: *mut lapack_complex_float, lda: lapack_int,
                                af: *mut lapack_complex_float, ldaf: lapack_int,
                                ipiv: *mut lapack_int, equed: *mut c_char, s: *mut c_float,
                                b: *mut lapack_complex_float, ldb: lapack_int,
                                x: *mut lapack_complex_float, ldx: lapack_int, rcond: *mut c_float,
                                rpvgrw: *mut c_float, berr: *mut c_float, n_err_bnds: lapack_int,
                                err_bnds_norm: *mut c_float, err_bnds_comp: *mut c_float,
                                nparams: lapack_int, params: *mut c_float,
                                work: *mut lapack_complex_float, rwork: *mut c_float)
                                -> lapack_int;
    pub fn LAPACKE_zhesvxx_work(matrix_layout: c_int, fact: c_char, uplo: c_char, n: lapack_int,
                                nrhs: lapack_int, a: *mut lapack_complex_double, lda: lapack_int,
                                af: *mut lapack_complex_double, ldaf: lapack_int,
                                ipiv: *mut lapack_int, equed: *mut c_char, s: *mut c_double,
                                b: *mut lapack_complex_double, ldb: lapack_int,
                                x: *mut lapack_complex_double, ldx: lapack_int,
                                rcond: *mut c_double, rpvgrw: *mut c_double, berr: *mut c_double,
                                n_err_bnds: lapack_int, err_bnds_norm: *mut c_double,
                                err_bnds_comp: *mut c_double, nparams: lapack_int,
                                params: *mut c_double, work: *mut lapack_complex_double,
                                rwork: *mut c_double)
                                -> lapack_int;

    pub fn LAPACKE_chetrd_work(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                               a: *mut lapack_complex_float, lda: lapack_int, d: *mut c_float,
                               e: *mut c_float, tau: *mut lapack_complex_float,
                               work: *mut lapack_complex_float, lwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_zhetrd_work(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                               a: *mut lapack_complex_double, lda: lapack_int, d: *mut c_double,
                               e: *mut c_double, tau: *mut lapack_complex_double,
                               work: *mut lapack_complex_double, lwork: lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_chetrf_work(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                               a: *mut lapack_complex_float, lda: lapack_int,
                               ipiv: *mut lapack_int, work: *mut lapack_complex_float,
                               lwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_zhetrf_work(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                               a: *mut lapack_complex_double, lda: lapack_int,
                               ipiv: *mut lapack_int, work: *mut lapack_complex_double,
                               lwork: lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_chetri_work(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                               a: *mut lapack_complex_float, lda: lapack_int,
                               ipiv: *const lapack_int, work: *mut lapack_complex_float)
                               -> lapack_int;
    pub fn LAPACKE_zhetri_work(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                               a: *mut lapack_complex_double, lda: lapack_int,
                               ipiv: *const lapack_int, work: *mut lapack_complex_double)
                               -> lapack_int;

    pub fn LAPACKE_chetrs_work(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                               a: *const lapack_complex_float, lda: lapack_int,
                               ipiv: *const lapack_int, b: *mut lapack_complex_float,
                               ldb: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_zhetrs_work(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                               a: *const lapack_complex_double, lda: lapack_int,
                               ipiv: *const lapack_int, b: *mut lapack_complex_double,
                               ldb: lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_chfrk_work(matrix_layout: c_int, transr: c_char, uplo: c_char, trans: c_char,
                              n: lapack_int, k: lapack_int, alpha: c_float,
                              a: *const lapack_complex_float, lda: lapack_int, beta: c_float,
                              c: *mut lapack_complex_float)
                              -> lapack_int;
    pub fn LAPACKE_zhfrk_work(matrix_layout: c_int, transr: c_char, uplo: c_char, trans: c_char,
                              n: lapack_int, k: lapack_int, alpha: c_double,
                              a: *const lapack_complex_double, lda: lapack_int, beta: c_double,
                              c: *mut lapack_complex_double)
                              -> lapack_int;

    pub fn LAPACKE_shgeqz_work(matrix_layout: c_int, job: c_char, compq: c_char, compz: c_char,
                               n: lapack_int, ilo: lapack_int, ihi: lapack_int, h: *mut c_float,
                               ldh: lapack_int, t: *mut c_float, ldt: lapack_int,
                               alphar: *mut c_float, alphai: *mut c_float, beta: *mut c_float,
                               q: *mut c_float, ldq: lapack_int, z: *mut c_float, ldz: lapack_int,
                               work: *mut c_float, lwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dhgeqz_work(matrix_layout: c_int, job: c_char, compq: c_char, compz: c_char,
                               n: lapack_int, ilo: lapack_int, ihi: lapack_int, h: *mut c_double,
                               ldh: lapack_int, t: *mut c_double, ldt: lapack_int,
                               alphar: *mut c_double, alphai: *mut c_double, beta: *mut c_double,
                               q: *mut c_double, ldq: lapack_int, z: *mut c_double,
                               ldz: lapack_int, work: *mut c_double, lwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_chgeqz_work(matrix_layout: c_int, job: c_char, compq: c_char, compz: c_char,
                               n: lapack_int, ilo: lapack_int, ihi: lapack_int,
                               h: *mut lapack_complex_float, ldh: lapack_int,
                               t: *mut lapack_complex_float, ldt: lapack_int,
                               alpha: *mut lapack_complex_float, beta: *mut lapack_complex_float,
                               q: *mut lapack_complex_float, ldq: lapack_int,
                               z: *mut lapack_complex_float, ldz: lapack_int,
                               work: *mut lapack_complex_float, lwork: lapack_int,
                               rwork: *mut c_float)
                               -> lapack_int;
    pub fn LAPACKE_zhgeqz_work(matrix_layout: c_int, job: c_char, compq: c_char, compz: c_char,
                               n: lapack_int, ilo: lapack_int, ihi: lapack_int,
                               h: *mut lapack_complex_double, ldh: lapack_int,
                               t: *mut lapack_complex_double, ldt: lapack_int,
                               alpha: *mut lapack_complex_double, beta: *mut lapack_complex_double,
                               q: *mut lapack_complex_double, ldq: lapack_int,
                               z: *mut lapack_complex_double, ldz: lapack_int,
                               work: *mut lapack_complex_double, lwork: lapack_int,
                               rwork: *mut c_double)
                               -> lapack_int;

    pub fn LAPACKE_chpcon_work(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                               ap: *const lapack_complex_float, ipiv: *const lapack_int,
                               anorm: c_float, rcond: *mut c_float,
                               work: *mut lapack_complex_float)
                               -> lapack_int;
    pub fn LAPACKE_zhpcon_work(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                               ap: *const lapack_complex_double, ipiv: *const lapack_int,
                               anorm: c_double, rcond: *mut c_double,
                               work: *mut lapack_complex_double)
                               -> lapack_int;

    pub fn LAPACKE_chpev_work(matrix_layout: c_int, jobz: c_char, uplo: c_char, n: lapack_int,
                              ap: *mut lapack_complex_float, w: *mut c_float,
                              z: *mut lapack_complex_float, ldz: lapack_int,
                              work: *mut lapack_complex_float, rwork: *mut c_float)
                              -> lapack_int;
    pub fn LAPACKE_zhpev_work(matrix_layout: c_int, jobz: c_char, uplo: c_char, n: lapack_int,
                              ap: *mut lapack_complex_double, w: *mut c_double,
                              z: *mut lapack_complex_double, ldz: lapack_int,
                              work: *mut lapack_complex_double, rwork: *mut c_double)
                              -> lapack_int;

    pub fn LAPACKE_chpevd_work(matrix_layout: c_int, jobz: c_char, uplo: c_char, n: lapack_int,
                               ap: *mut lapack_complex_float, w: *mut c_float,
                               z: *mut lapack_complex_float, ldz: lapack_int,
                               work: *mut lapack_complex_float, lwork: lapack_int,
                               rwork: *mut c_float, lrwork: lapack_int, iwork: *mut lapack_int,
                               liwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_zhpevd_work(matrix_layout: c_int, jobz: c_char, uplo: c_char, n: lapack_int,
                               ap: *mut lapack_complex_double, w: *mut c_double,
                               z: *mut lapack_complex_double, ldz: lapack_int,
                               work: *mut lapack_complex_double, lwork: lapack_int,
                               rwork: *mut c_double, lrwork: lapack_int, iwork: *mut lapack_int,
                               liwork: lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_chpevx_work(matrix_layout: c_int, jobz: c_char, range: c_char, uplo: c_char,
                               n: lapack_int, ap: *mut lapack_complex_float, vl: c_float,
                               vu: c_float, il: lapack_int, iu: lapack_int, abstol: c_float,
                               m: *mut lapack_int, w: *mut c_float, z: *mut lapack_complex_float,
                               ldz: lapack_int, work: *mut lapack_complex_float,
                               rwork: *mut c_float, iwork: *mut lapack_int, ifail: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_zhpevx_work(matrix_layout: c_int, jobz: c_char, range: c_char, uplo: c_char,
                               n: lapack_int, ap: *mut lapack_complex_double, vl: c_double,
                               vu: c_double, il: lapack_int, iu: lapack_int, abstol: c_double,
                               m: *mut lapack_int, w: *mut c_double, z: *mut lapack_complex_double,
                               ldz: lapack_int, work: *mut lapack_complex_double,
                               rwork: *mut c_double, iwork: *mut lapack_int,
                               ifail: *mut lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_chpgst_work(matrix_layout: c_int, itype: lapack_int, uplo: c_char,
                               n: lapack_int, ap: *mut lapack_complex_float,
                               bp: *const lapack_complex_float)
                               -> lapack_int;
    pub fn LAPACKE_zhpgst_work(matrix_layout: c_int, itype: lapack_int, uplo: c_char,
                               n: lapack_int, ap: *mut lapack_complex_double,
                               bp: *const lapack_complex_double)
                               -> lapack_int;

    pub fn LAPACKE_chpgv_work(matrix_layout: c_int, itype: lapack_int, jobz: c_char, uplo: c_char,
                              n: lapack_int, ap: *mut lapack_complex_float,
                              bp: *mut lapack_complex_float, w: *mut c_float,
                              z: *mut lapack_complex_float, ldz: lapack_int,
                              work: *mut lapack_complex_float, rwork: *mut c_float)
                              -> lapack_int;
    pub fn LAPACKE_zhpgv_work(matrix_layout: c_int, itype: lapack_int, jobz: c_char, uplo: c_char,
                              n: lapack_int, ap: *mut lapack_complex_double,
                              bp: *mut lapack_complex_double, w: *mut c_double,
                              z: *mut lapack_complex_double, ldz: lapack_int,
                              work: *mut lapack_complex_double, rwork: *mut c_double)
                              -> lapack_int;

    pub fn LAPACKE_chpgvd_work(matrix_layout: c_int, itype: lapack_int, jobz: c_char, uplo: c_char,
                               n: lapack_int, ap: *mut lapack_complex_float,
                               bp: *mut lapack_complex_float, w: *mut c_float,
                               z: *mut lapack_complex_float, ldz: lapack_int,
                               work: *mut lapack_complex_float, lwork: lapack_int,
                               rwork: *mut c_float, lrwork: lapack_int, iwork: *mut lapack_int,
                               liwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_zhpgvd_work(matrix_layout: c_int, itype: lapack_int, jobz: c_char, uplo: c_char,
                               n: lapack_int, ap: *mut lapack_complex_double,
                               bp: *mut lapack_complex_double, w: *mut c_double,
                               z: *mut lapack_complex_double, ldz: lapack_int,
                               work: *mut lapack_complex_double, lwork: lapack_int,
                               rwork: *mut c_double, lrwork: lapack_int, iwork: *mut lapack_int,
                               liwork: lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_chpgvx_work(matrix_layout: c_int, itype: lapack_int, jobz: c_char,
                               range: c_char, uplo: c_char, n: lapack_int,
                               ap: *mut lapack_complex_float, bp: *mut lapack_complex_float,
                               vl: c_float, vu: c_float, il: lapack_int, iu: lapack_int,
                               abstol: c_float, m: *mut lapack_int, w: *mut c_float,
                               z: *mut lapack_complex_float, ldz: lapack_int,
                               work: *mut lapack_complex_float, rwork: *mut c_float,
                               iwork: *mut lapack_int, ifail: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_zhpgvx_work(matrix_layout: c_int, itype: lapack_int, jobz: c_char,
                               range: c_char, uplo: c_char, n: lapack_int,
                               ap: *mut lapack_complex_double, bp: *mut lapack_complex_double,
                               vl: c_double, vu: c_double, il: lapack_int, iu: lapack_int,
                               abstol: c_double, m: *mut lapack_int, w: *mut c_double,
                               z: *mut lapack_complex_double, ldz: lapack_int,
                               work: *mut lapack_complex_double, rwork: *mut c_double,
                               iwork: *mut lapack_int, ifail: *mut lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_chprfs_work(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                               ap: *const lapack_complex_float, afp: *const lapack_complex_float,
                               ipiv: *const lapack_int, b: *const lapack_complex_float,
                               ldb: lapack_int, x: *mut lapack_complex_float, ldx: lapack_int,
                               ferr: *mut c_float, berr: *mut c_float,
                               work: *mut lapack_complex_float, rwork: *mut c_float)
                               -> lapack_int;
    pub fn LAPACKE_zhprfs_work(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                               ap: *const lapack_complex_double, afp: *const lapack_complex_double,
                               ipiv: *const lapack_int, b: *const lapack_complex_double,
                               ldb: lapack_int, x: *mut lapack_complex_double, ldx: lapack_int,
                               ferr: *mut c_double, berr: *mut c_double,
                               work: *mut lapack_complex_double, rwork: *mut c_double)
                               -> lapack_int;

    pub fn LAPACKE_chpsv_work(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                              ap: *mut lapack_complex_float, ipiv: *mut lapack_int,
                              b: *mut lapack_complex_float, ldb: lapack_int)
                              -> lapack_int;
    pub fn LAPACKE_zhpsv_work(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                              ap: *mut lapack_complex_double, ipiv: *mut lapack_int,
                              b: *mut lapack_complex_double, ldb: lapack_int)
                              -> lapack_int;

    pub fn LAPACKE_chpsvx_work(matrix_layout: c_int, fact: c_char, uplo: c_char, n: lapack_int,
                               nrhs: lapack_int, ap: *const lapack_complex_float,
                               afp: *mut lapack_complex_float, ipiv: *mut lapack_int,
                               b: *const lapack_complex_float, ldb: lapack_int,
                               x: *mut lapack_complex_float, ldx: lapack_int, rcond: *mut c_float,
                               ferr: *mut c_float, berr: *mut c_float,
                               work: *mut lapack_complex_float, rwork: *mut c_float)
                               -> lapack_int;
    pub fn LAPACKE_zhpsvx_work(matrix_layout: c_int, fact: c_char, uplo: c_char, n: lapack_int,
                               nrhs: lapack_int, ap: *const lapack_complex_double,
                               afp: *mut lapack_complex_double, ipiv: *mut lapack_int,
                               b: *const lapack_complex_double, ldb: lapack_int,
                               x: *mut lapack_complex_double, ldx: lapack_int,
                               rcond: *mut c_double, ferr: *mut c_double, berr: *mut c_double,
                               work: *mut lapack_complex_double, rwork: *mut c_double)
                               -> lapack_int;

    pub fn LAPACKE_chptrd_work(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                               ap: *mut lapack_complex_float, d: *mut c_float, e: *mut c_float,
                               tau: *mut lapack_complex_float)
                               -> lapack_int;
    pub fn LAPACKE_zhptrd_work(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                               ap: *mut lapack_complex_double, d: *mut c_double, e: *mut c_double,
                               tau: *mut lapack_complex_double)
                               -> lapack_int;

    pub fn LAPACKE_chptrf_work(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                               ap: *mut lapack_complex_float, ipiv: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_zhptrf_work(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                               ap: *mut lapack_complex_double, ipiv: *mut lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_chptri_work(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                               ap: *mut lapack_complex_float, ipiv: *const lapack_int,
                               work: *mut lapack_complex_float)
                               -> lapack_int;
    pub fn LAPACKE_zhptri_work(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                               ap: *mut lapack_complex_double, ipiv: *const lapack_int,
                               work: *mut lapack_complex_double)
                               -> lapack_int;

    pub fn LAPACKE_chptrs_work(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                               ap: *const lapack_complex_float, ipiv: *const lapack_int,
                               b: *mut lapack_complex_float, ldb: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_zhptrs_work(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                               ap: *const lapack_complex_double, ipiv: *const lapack_int,
                               b: *mut lapack_complex_double, ldb: lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_shsein_work(matrix_layout: c_int, job: c_char, eigsrc: c_char, initv: c_char,
                               select: *mut lapack_logical, n: lapack_int, h: *const c_float,
                               ldh: lapack_int, wr: *mut c_float, wi: *const c_float,
                               vl: *mut c_float, ldvl: lapack_int, vr: *mut c_float,
                               ldvr: lapack_int, mm: lapack_int, m: *mut lapack_int,
                               work: *mut c_float, ifaill: *mut lapack_int,
                               ifailr: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dhsein_work(matrix_layout: c_int, job: c_char, eigsrc: c_char, initv: c_char,
                               select: *mut lapack_logical, n: lapack_int, h: *const c_double,
                               ldh: lapack_int, wr: *mut c_double, wi: *const c_double,
                               vl: *mut c_double, ldvl: lapack_int, vr: *mut c_double,
                               ldvr: lapack_int, mm: lapack_int, m: *mut lapack_int,
                               work: *mut c_double, ifaill: *mut lapack_int,
                               ifailr: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_chsein_work(matrix_layout: c_int, job: c_char, eigsrc: c_char, initv: c_char,
                               select: *const lapack_logical, n: lapack_int,
                               h: *const lapack_complex_float, ldh: lapack_int,
                               w: *mut lapack_complex_float, vl: *mut lapack_complex_float,
                               ldvl: lapack_int, vr: *mut lapack_complex_float, ldvr: lapack_int,
                               mm: lapack_int, m: *mut lapack_int, work: *mut lapack_complex_float,
                               rwork: *mut c_float, ifaill: *mut lapack_int,
                               ifailr: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_zhsein_work(matrix_layout: c_int, job: c_char, eigsrc: c_char, initv: c_char,
                               select: *const lapack_logical, n: lapack_int,
                               h: *const lapack_complex_double, ldh: lapack_int,
                               w: *mut lapack_complex_double, vl: *mut lapack_complex_double,
                               ldvl: lapack_int, vr: *mut lapack_complex_double, ldvr: lapack_int,
                               mm: lapack_int, m: *mut lapack_int,
                               work: *mut lapack_complex_double, rwork: *mut c_double,
                               ifaill: *mut lapack_int, ifailr: *mut lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_shseqr_work(matrix_layout: c_int, job: c_char, compz: c_char, n: lapack_int,
                               ilo: lapack_int, ihi: lapack_int, h: *mut c_float, ldh: lapack_int,
                               wr: *mut c_float, wi: *mut c_float, z: *mut c_float,
                               ldz: lapack_int, work: *mut c_float, lwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dhseqr_work(matrix_layout: c_int, job: c_char, compz: c_char, n: lapack_int,
                               ilo: lapack_int, ihi: lapack_int, h: *mut c_double, ldh: lapack_int,
                               wr: *mut c_double, wi: *mut c_double, z: *mut c_double,
                               ldz: lapack_int, work: *mut c_double, lwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_chseqr_work(matrix_layout: c_int, job: c_char, compz: c_char, n: lapack_int,
                               ilo: lapack_int, ihi: lapack_int, h: *mut lapack_complex_float,
                               ldh: lapack_int, w: *mut lapack_complex_float,
                               z: *mut lapack_complex_float, ldz: lapack_int,
                               work: *mut lapack_complex_float, lwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_zhseqr_work(matrix_layout: c_int, job: c_char, compz: c_char, n: lapack_int,
                               ilo: lapack_int, ihi: lapack_int, h: *mut lapack_complex_double,
                               ldh: lapack_int, w: *mut lapack_complex_double,
                               z: *mut lapack_complex_double, ldz: lapack_int,
                               work: *mut lapack_complex_double, lwork: lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_clacgv_work(n: lapack_int, x: *mut lapack_complex_float, incx: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_zlacgv_work(n: lapack_int, x: *mut lapack_complex_double, incx: lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_slacn2_work(n: lapack_int, v: *mut c_float, x: *mut c_float,
                               isgn: *mut lapack_int, est: *mut c_float, kase: *mut lapack_int,
                               isave: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dlacn2_work(n: lapack_int, v: *mut c_double, x: *mut c_double,
                               isgn: *mut lapack_int, est: *mut c_double, kase: *mut lapack_int,
                               isave: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_clacn2_work(n: lapack_int, v: *mut lapack_complex_float,
                               x: *mut lapack_complex_float, est: *mut c_float,
                               kase: *mut lapack_int, isave: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_zlacn2_work(n: lapack_int, v: *mut lapack_complex_double,
                               x: *mut lapack_complex_double, est: *mut c_double,
                               kase: *mut lapack_int, isave: *mut lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_slacpy_work(matrix_layout: c_int, uplo: c_char, m: lapack_int, n: lapack_int,
                               a: *const c_float, lda: lapack_int, b: *mut c_float,
                               ldb: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dlacpy_work(matrix_layout: c_int, uplo: c_char, m: lapack_int, n: lapack_int,
                               a: *const c_double, lda: lapack_int, b: *mut c_double,
                               ldb: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_clacpy_work(matrix_layout: c_int, uplo: c_char, m: lapack_int, n: lapack_int,
                               a: *const lapack_complex_float, lda: lapack_int,
                               b: *mut lapack_complex_float, ldb: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_zlacpy_work(matrix_layout: c_int, uplo: c_char, m: lapack_int, n: lapack_int,
                               a: *const lapack_complex_double, lda: lapack_int,
                               b: *mut lapack_complex_double, ldb: lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_clacp2_work(matrix_layout: c_int, uplo: c_char, m: lapack_int, n: lapack_int,
                               a: *const c_float, lda: lapack_int, b: *mut lapack_complex_float,
                               ldb: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_zlacp2_work(matrix_layout: c_int, uplo: c_char, m: lapack_int, n: lapack_int,
                               a: *const c_double, lda: lapack_int, b: *mut lapack_complex_double,
                               ldb: lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_zlag2c_work(matrix_layout: c_int, m: lapack_int, n: lapack_int,
                               a: *const lapack_complex_double, lda: lapack_int,
                               sa: *mut lapack_complex_float, ldsa: lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_slag2d_work(matrix_layout: c_int, m: lapack_int, n: lapack_int,
                               sa: *const c_float, ldsa: lapack_int, a: *mut c_double,
                               lda: lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_dlag2s_work(matrix_layout: c_int, m: lapack_int, n: lapack_int,
                               a: *const c_double, lda: lapack_int, sa: *mut c_float,
                               ldsa: lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_clag2z_work(matrix_layout: c_int, m: lapack_int, n: lapack_int,
                               sa: *const lapack_complex_float, ldsa: lapack_int,
                               a: *mut lapack_complex_double, lda: lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_slagge_work(matrix_layout: c_int, m: lapack_int, n: lapack_int, kl: lapack_int,
                               ku: lapack_int, d: *const c_float, a: *mut c_float, lda: lapack_int,
                               iseed: *mut lapack_int, work: *mut c_float)
                               -> lapack_int;
    pub fn LAPACKE_dlagge_work(matrix_layout: c_int, m: lapack_int, n: lapack_int, kl: lapack_int,
                               ku: lapack_int, d: *const c_double, a: *mut c_double,
                               lda: lapack_int, iseed: *mut lapack_int, work: *mut c_double)
                               -> lapack_int;
    pub fn LAPACKE_clagge_work(matrix_layout: c_int, m: lapack_int, n: lapack_int, kl: lapack_int,
                               ku: lapack_int, d: *const c_float, a: *mut lapack_complex_float,
                               lda: lapack_int, iseed: *mut lapack_int,
                               work: *mut lapack_complex_float)
                               -> lapack_int;
    pub fn LAPACKE_zlagge_work(matrix_layout: c_int, m: lapack_int, n: lapack_int, kl: lapack_int,
                               ku: lapack_int, d: *const c_double, a: *mut lapack_complex_double,
                               lda: lapack_int, iseed: *mut lapack_int,
                               work: *mut lapack_complex_double)
                               -> lapack_int;

    pub fn LAPACKE_claghe_work(matrix_layout: c_int, n: lapack_int, k: lapack_int,
                               d: *const c_float, a: *mut lapack_complex_float, lda: lapack_int,
                               iseed: *mut lapack_int, work: *mut lapack_complex_float)
                               -> lapack_int;
    pub fn LAPACKE_zlaghe_work(matrix_layout: c_int, n: lapack_int, k: lapack_int,
                               d: *const c_double, a: *mut lapack_complex_double, lda: lapack_int,
                               iseed: *mut lapack_int, work: *mut lapack_complex_double)
                               -> lapack_int;

    pub fn LAPACKE_slagsy_work(matrix_layout: c_int, n: lapack_int, k: lapack_int,
                               d: *const c_float, a: *mut c_float, lda: lapack_int,
                               iseed: *mut lapack_int, work: *mut c_float)
                               -> lapack_int;
    pub fn LAPACKE_dlagsy_work(matrix_layout: c_int, n: lapack_int, k: lapack_int,
                               d: *const c_double, a: *mut c_double, lda: lapack_int,
                               iseed: *mut lapack_int, work: *mut c_double)
                               -> lapack_int;
    pub fn LAPACKE_clagsy_work(matrix_layout: c_int, n: lapack_int, k: lapack_int,
                               d: *const c_float, a: *mut lapack_complex_float, lda: lapack_int,
                               iseed: *mut lapack_int, work: *mut lapack_complex_float)
                               -> lapack_int;
    pub fn LAPACKE_zlagsy_work(matrix_layout: c_int, n: lapack_int, k: lapack_int,
                               d: *const c_double, a: *mut lapack_complex_double, lda: lapack_int,
                               iseed: *mut lapack_int, work: *mut lapack_complex_double)
                               -> lapack_int;

    pub fn LAPACKE_slapmr_work(matrix_layout: c_int, forwrd: lapack_logical, m: lapack_int,
                               n: lapack_int, x: *mut c_float, ldx: lapack_int, k: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dlapmr_work(matrix_layout: c_int, forwrd: lapack_logical, m: lapack_int,
                               n: lapack_int, x: *mut c_double, ldx: lapack_int,
                               k: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_clapmr_work(matrix_layout: c_int, forwrd: lapack_logical, m: lapack_int,
                               n: lapack_int, x: *mut lapack_complex_float, ldx: lapack_int,
                               k: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_zlapmr_work(matrix_layout: c_int, forwrd: lapack_logical, m: lapack_int,
                               n: lapack_int, x: *mut lapack_complex_double, ldx: lapack_int,
                               k: *mut lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_slapmt_work(matrix_layout: c_int, forwrd: lapack_logical, m: lapack_int,
                               n: lapack_int, x: *mut c_float, ldx: lapack_int, k: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dlapmt_work(matrix_layout: c_int, forwrd: lapack_logical, m: lapack_int,
                               n: lapack_int, x: *mut c_double, ldx: lapack_int,
                               k: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_clapmt_work(matrix_layout: c_int, forwrd: lapack_logical, m: lapack_int,
                               n: lapack_int, x: *mut lapack_complex_float, ldx: lapack_int,
                               k: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_zlapmt_work(matrix_layout: c_int, forwrd: lapack_logical, m: lapack_int,
                               n: lapack_int, x: *mut lapack_complex_double, ldx: lapack_int,
                               k: *mut lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_slartgp_work(f: c_float, g: c_float, cs: *mut c_float, sn: *mut c_float,
                                r: *mut c_float)
                                -> lapack_int;
    pub fn LAPACKE_dlartgp_work(f: c_double, g: c_double, cs: *mut c_double, sn: *mut c_double,
                                r: *mut c_double)
                                -> lapack_int;

    pub fn LAPACKE_slartgs_work(x: c_float, y: c_float, sigma: c_float, cs: *mut c_float,
                                sn: *mut c_float)
                                -> lapack_int;
    pub fn LAPACKE_dlartgs_work(x: c_double, y: c_double, sigma: c_double, cs: *mut c_double,
                                sn: *mut c_double)
                                -> lapack_int;

    pub fn LAPACKE_slapy2_work(x: c_float, y: c_float) -> c_float;
    pub fn LAPACKE_dlapy2_work(x: c_double, y: c_double) -> c_double;

    pub fn LAPACKE_slapy3_work(x: c_float, y: c_float, z: c_float) -> c_float;
    pub fn LAPACKE_dlapy3_work(x: c_double, y: c_double, z: c_double) -> c_double;

    pub fn LAPACKE_slamch_work(cmach: c_char) -> c_float;
    pub fn LAPACKE_dlamch_work(cmach: c_char) -> c_double;

    pub fn LAPACKE_slange_work(matrix_layout: c_int, norm: c_char, m: lapack_int, n: lapack_int,
                               a: *const c_float, lda: lapack_int, work: *mut c_float)
                               -> c_float;
    pub fn LAPACKE_dlange_work(matrix_layout: c_int, norm: c_char, m: lapack_int, n: lapack_int,
                               a: *const c_double, lda: lapack_int, work: *mut c_double)
                               -> c_double;
    pub fn LAPACKE_clange_work(matrix_layout: c_int, norm: c_char, m: lapack_int, n: lapack_int,
                               a: *const lapack_complex_float, lda: lapack_int, work: *mut c_float)
                               -> c_float;
    pub fn LAPACKE_zlange_work(matrix_layout: c_int, norm: c_char, m: lapack_int, n: lapack_int,
                               a: *const lapack_complex_double, lda: lapack_int,
                               work: *mut c_double)
                               -> c_double;

    pub fn LAPACKE_clanhe_work(matrix_layout: c_int, norm: c_char, uplo: c_char, n: lapack_int,
                               a: *const lapack_complex_float, lda: lapack_int, work: *mut c_float)
                               -> c_float;
    pub fn LAPACKE_zlanhe_work(matrix_layout: c_int, norm: c_char, uplo: c_char, n: lapack_int,
                               a: *const lapack_complex_double, lda: lapack_int,
                               work: *mut c_double)
                               -> c_double;

    pub fn LAPACKE_slansy_work(matrix_layout: c_int, norm: c_char, uplo: c_char, n: lapack_int,
                               a: *const c_float, lda: lapack_int, work: *mut c_float)
                               -> c_float;
    pub fn LAPACKE_dlansy_work(matrix_layout: c_int, norm: c_char, uplo: c_char, n: lapack_int,
                               a: *const c_double, lda: lapack_int, work: *mut c_double)
                               -> c_double;
    pub fn LAPACKE_clansy_work(matrix_layout: c_int, norm: c_char, uplo: c_char, n: lapack_int,
                               a: *const lapack_complex_float, lda: lapack_int, work: *mut c_float)
                               -> c_float;
    pub fn LAPACKE_zlansy_work(matrix_layout: c_int, norm: c_char, uplo: c_char, n: lapack_int,
                               a: *const lapack_complex_double, lda: lapack_int,
                               work: *mut c_double)
                               -> c_double;

    pub fn LAPACKE_slantr_work(matrix_layout: c_int, norm: c_char, uplo: c_char, diag: c_char,
                               m: lapack_int, n: lapack_int, a: *const c_float, lda: lapack_int,
                               work: *mut c_float)
                               -> c_float;
    pub fn LAPACKE_dlantr_work(matrix_layout: c_int, norm: c_char, uplo: c_char, diag: c_char,
                               m: lapack_int, n: lapack_int, a: *const c_double, lda: lapack_int,
                               work: *mut c_double)
                               -> c_double;
    pub fn LAPACKE_clantr_work(matrix_layout: c_int, norm: c_char, uplo: c_char, diag: c_char,
                               m: lapack_int, n: lapack_int, a: *const lapack_complex_float,
                               lda: lapack_int, work: *mut c_float)
                               -> c_float;
    pub fn LAPACKE_zlantr_work(matrix_layout: c_int, norm: c_char, uplo: c_char, diag: c_char,
                               m: lapack_int, n: lapack_int, a: *const lapack_complex_double,
                               lda: lapack_int, work: *mut c_double)
                               -> c_double;

    pub fn LAPACKE_slarfb_work(matrix_layout: c_int, side: c_char, trans: c_char, direct: c_char,
                               storev: c_char, m: lapack_int, n: lapack_int, k: lapack_int,
                               v: *const c_float, ldv: lapack_int, t: *const c_float,
                               ldt: lapack_int, c: *mut c_float, ldc: lapack_int,
                               work: *mut c_float, ldwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dlarfb_work(matrix_layout: c_int, side: c_char, trans: c_char, direct: c_char,
                               storev: c_char, m: lapack_int, n: lapack_int, k: lapack_int,
                               v: *const c_double, ldv: lapack_int, t: *const c_double,
                               ldt: lapack_int, c: *mut c_double, ldc: lapack_int,
                               work: *mut c_double, ldwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_clarfb_work(matrix_layout: c_int, side: c_char, trans: c_char, direct: c_char,
                               storev: c_char, m: lapack_int, n: lapack_int, k: lapack_int,
                               v: *const lapack_complex_float, ldv: lapack_int,
                               t: *const lapack_complex_float, ldt: lapack_int,
                               c: *mut lapack_complex_float, ldc: lapack_int,
                               work: *mut lapack_complex_float, ldwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_zlarfb_work(matrix_layout: c_int, side: c_char, trans: c_char, direct: c_char,
                               storev: c_char, m: lapack_int, n: lapack_int, k: lapack_int,
                               v: *const lapack_complex_double, ldv: lapack_int,
                               t: *const lapack_complex_double, ldt: lapack_int,
                               c: *mut lapack_complex_double, ldc: lapack_int,
                               work: *mut lapack_complex_double, ldwork: lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_slarfg_work(n: lapack_int, alpha: *mut c_float, x: *mut c_float,
                               incx: lapack_int, tau: *mut c_float)
                               -> lapack_int;
    pub fn LAPACKE_dlarfg_work(n: lapack_int, alpha: *mut c_double, x: *mut c_double,
                               incx: lapack_int, tau: *mut c_double)
                               -> lapack_int;
    pub fn LAPACKE_clarfg_work(n: lapack_int, alpha: *mut lapack_complex_float,
                               x: *mut lapack_complex_float, incx: lapack_int,
                               tau: *mut lapack_complex_float)
                               -> lapack_int;
    pub fn LAPACKE_zlarfg_work(n: lapack_int, alpha: *mut lapack_complex_double,
                               x: *mut lapack_complex_double, incx: lapack_int,
                               tau: *mut lapack_complex_double)
                               -> lapack_int;

    pub fn LAPACKE_slarft_work(matrix_layout: c_int, direct: c_char, storev: c_char, n: lapack_int,
                               k: lapack_int, v: *const c_float, ldv: lapack_int,
                               tau: *const c_float, t: *mut c_float, ldt: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dlarft_work(matrix_layout: c_int, direct: c_char, storev: c_char, n: lapack_int,
                               k: lapack_int, v: *const c_double, ldv: lapack_int,
                               tau: *const c_double, t: *mut c_double, ldt: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_clarft_work(matrix_layout: c_int, direct: c_char, storev: c_char, n: lapack_int,
                               k: lapack_int, v: *const lapack_complex_float, ldv: lapack_int,
                               tau: *const lapack_complex_float, t: *mut lapack_complex_float,
                               ldt: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_zlarft_work(matrix_layout: c_int, direct: c_char, storev: c_char, n: lapack_int,
                               k: lapack_int, v: *const lapack_complex_double, ldv: lapack_int,
                               tau: *const lapack_complex_double, t: *mut lapack_complex_double,
                               ldt: lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_slarfx_work(matrix_layout: c_int, side: c_char, m: lapack_int, n: lapack_int,
                               v: *const c_float, tau: c_float, c: *mut c_float, ldc: lapack_int,
                               work: *mut c_float)
                               -> lapack_int;
    pub fn LAPACKE_dlarfx_work(matrix_layout: c_int, side: c_char, m: lapack_int, n: lapack_int,
                               v: *const c_double, tau: c_double, c: *mut c_double,
                               ldc: lapack_int, work: *mut c_double)
                               -> lapack_int;
    pub fn LAPACKE_clarfx_work(matrix_layout: c_int, side: c_char, m: lapack_int, n: lapack_int,
                               v: *const lapack_complex_float, tau: lapack_complex_float,
                               c: *mut lapack_complex_float, ldc: lapack_int,
                               work: *mut lapack_complex_float)
                               -> lapack_int;
    pub fn LAPACKE_zlarfx_work(matrix_layout: c_int, side: c_char, m: lapack_int, n: lapack_int,
                               v: *const lapack_complex_double, tau: lapack_complex_double,
                               c: *mut lapack_complex_double, ldc: lapack_int,
                               work: *mut lapack_complex_double)
                               -> lapack_int;

    pub fn LAPACKE_slarnv_work(idist: lapack_int, iseed: *mut lapack_int, n: lapack_int,
                               x: *mut c_float)
                               -> lapack_int;
    pub fn LAPACKE_dlarnv_work(idist: lapack_int, iseed: *mut lapack_int, n: lapack_int,
                               x: *mut c_double)
                               -> lapack_int;
    pub fn LAPACKE_clarnv_work(idist: lapack_int, iseed: *mut lapack_int, n: lapack_int,
                               x: *mut lapack_complex_float)
                               -> lapack_int;
    pub fn LAPACKE_zlarnv_work(idist: lapack_int, iseed: *mut lapack_int, n: lapack_int,
                               x: *mut lapack_complex_double)
                               -> lapack_int;

    pub fn LAPACKE_slascl_work(matrix_layout: c_int, _type: c_char, kl: lapack_int, ku: lapack_int,
                               cfrom: c_float, cto: c_float, m: lapack_int, n: lapack_int,
                               a: *mut c_float, lda: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dlascl_work(matrix_layout: c_int, _type: c_char, kl: lapack_int, ku: lapack_int,
                               cfrom: c_double, cto: c_double, m: lapack_int, n: lapack_int,
                               a: *mut c_double, lda: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_clascl_work(matrix_layout: c_int, _type: c_char, kl: lapack_int, ku: lapack_int,
                               cfrom: c_float, cto: c_float, m: lapack_int, n: lapack_int,
                               a: *mut lapack_complex_float, lda: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_zlascl_work(matrix_layout: c_int, _type: c_char, kl: lapack_int, ku: lapack_int,
                               cfrom: c_double, cto: c_double, m: lapack_int, n: lapack_int,
                               a: *mut lapack_complex_double, lda: lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_slaset_work(matrix_layout: c_int, uplo: c_char, m: lapack_int, n: lapack_int,
                               alpha: c_float, beta: c_float, a: *mut c_float, lda: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dlaset_work(matrix_layout: c_int, uplo: c_char, m: lapack_int, n: lapack_int,
                               alpha: c_double, beta: c_double, a: *mut c_double, lda: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_claset_work(matrix_layout: c_int, uplo: c_char, m: lapack_int, n: lapack_int,
                               alpha: lapack_complex_float, beta: lapack_complex_float,
                               a: *mut lapack_complex_float, lda: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_zlaset_work(matrix_layout: c_int, uplo: c_char, m: lapack_int, n: lapack_int,
                               alpha: lapack_complex_double, beta: lapack_complex_double,
                               a: *mut lapack_complex_double, lda: lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_slasrt_work(id: c_char, n: lapack_int, d: *mut c_float) -> lapack_int;
    pub fn LAPACKE_dlasrt_work(id: c_char, n: lapack_int, d: *mut c_double) -> lapack_int;

    pub fn LAPACKE_slaswp_work(matrix_layout: c_int, n: lapack_int, a: *mut c_float,
                               lda: lapack_int, k1: lapack_int, k2: lapack_int,
                               ipiv: *const lapack_int, incx: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dlaswp_work(matrix_layout: c_int, n: lapack_int, a: *mut c_double,
                               lda: lapack_int, k1: lapack_int, k2: lapack_int,
                               ipiv: *const lapack_int, incx: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_claswp_work(matrix_layout: c_int, n: lapack_int, a: *mut lapack_complex_float,
                               lda: lapack_int, k1: lapack_int, k2: lapack_int,
                               ipiv: *const lapack_int, incx: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_zlaswp_work(matrix_layout: c_int, n: lapack_int, a: *mut lapack_complex_double,
                               lda: lapack_int, k1: lapack_int, k2: lapack_int,
                               ipiv: *const lapack_int, incx: lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_slatms_work(matrix_layout: c_int, m: lapack_int, n: lapack_int, dist: c_char,
                               iseed: *mut lapack_int, sym: c_char, d: *mut c_float,
                               mode: lapack_int, cond: c_float, dmax: c_float, kl: lapack_int,
                               ku: lapack_int, pack: c_char, a: *mut c_float, lda: lapack_int,
                               work: *mut c_float)
                               -> lapack_int;
    pub fn LAPACKE_dlatms_work(matrix_layout: c_int, m: lapack_int, n: lapack_int, dist: c_char,
                               iseed: *mut lapack_int, sym: c_char, d: *mut c_double,
                               mode: lapack_int, cond: c_double, dmax: c_double, kl: lapack_int,
                               ku: lapack_int, pack: c_char, a: *mut c_double, lda: lapack_int,
                               work: *mut c_double)
                               -> lapack_int;
    pub fn LAPACKE_clatms_work(matrix_layout: c_int, m: lapack_int, n: lapack_int, dist: c_char,
                               iseed: *mut lapack_int, sym: c_char, d: *mut c_float,
                               mode: lapack_int, cond: c_float, dmax: c_float, kl: lapack_int,
                               ku: lapack_int, pack: c_char, a: *mut lapack_complex_float,
                               lda: lapack_int, work: *mut lapack_complex_float)
                               -> lapack_int;
    pub fn LAPACKE_zlatms_work(matrix_layout: c_int, m: lapack_int, n: lapack_int, dist: c_char,
                               iseed: *mut lapack_int, sym: c_char, d: *mut c_double,
                               mode: lapack_int, cond: c_double, dmax: c_double, kl: lapack_int,
                               ku: lapack_int, pack: c_char, a: *mut lapack_complex_double,
                               lda: lapack_int, work: *mut lapack_complex_double)
                               -> lapack_int;

    pub fn LAPACKE_slauum_work(matrix_layout: c_int, uplo: c_char, n: lapack_int, a: *mut c_float,
                               lda: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dlauum_work(matrix_layout: c_int, uplo: c_char, n: lapack_int, a: *mut c_double,
                               lda: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_clauum_work(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                               a: *mut lapack_complex_float, lda: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_zlauum_work(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                               a: *mut lapack_complex_double, lda: lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_sopgtr_work(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                               ap: *const c_float, tau: *const c_float, q: *mut c_float,
                               ldq: lapack_int, work: *mut c_float)
                               -> lapack_int;
    pub fn LAPACKE_dopgtr_work(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                               ap: *const c_double, tau: *const c_double, q: *mut c_double,
                               ldq: lapack_int, work: *mut c_double)
                               -> lapack_int;

    pub fn LAPACKE_sopmtr_work(matrix_layout: c_int, side: c_char, uplo: c_char, trans: c_char,
                               m: lapack_int, n: lapack_int, ap: *const c_float,
                               tau: *const c_float, c: *mut c_float, ldc: lapack_int,
                               work: *mut c_float)
                               -> lapack_int;
    pub fn LAPACKE_dopmtr_work(matrix_layout: c_int, side: c_char, uplo: c_char, trans: c_char,
                               m: lapack_int, n: lapack_int, ap: *const c_double,
                               tau: *const c_double, c: *mut c_double, ldc: lapack_int,
                               work: *mut c_double)
                               -> lapack_int;

    pub fn LAPACKE_sorgbr_work(matrix_layout: c_int, vect: c_char, m: lapack_int, n: lapack_int,
                               k: lapack_int, a: *mut c_float, lda: lapack_int,
                               tau: *const c_float, work: *mut c_float, lwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dorgbr_work(matrix_layout: c_int, vect: c_char, m: lapack_int, n: lapack_int,
                               k: lapack_int, a: *mut c_double, lda: lapack_int,
                               tau: *const c_double, work: *mut c_double, lwork: lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_sorghr_work(matrix_layout: c_int, n: lapack_int, ilo: lapack_int,
                               ihi: lapack_int, a: *mut c_float, lda: lapack_int,
                               tau: *const c_float, work: *mut c_float, lwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dorghr_work(matrix_layout: c_int, n: lapack_int, ilo: lapack_int,
                               ihi: lapack_int, a: *mut c_double, lda: lapack_int,
                               tau: *const c_double, work: *mut c_double, lwork: lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_sorglq_work(matrix_layout: c_int, m: lapack_int, n: lapack_int, k: lapack_int,
                               a: *mut c_float, lda: lapack_int, tau: *const c_float,
                               work: *mut c_float, lwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dorglq_work(matrix_layout: c_int, m: lapack_int, n: lapack_int, k: lapack_int,
                               a: *mut c_double, lda: lapack_int, tau: *const c_double,
                               work: *mut c_double, lwork: lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_sorgql_work(matrix_layout: c_int, m: lapack_int, n: lapack_int, k: lapack_int,
                               a: *mut c_float, lda: lapack_int, tau: *const c_float,
                               work: *mut c_float, lwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dorgql_work(matrix_layout: c_int, m: lapack_int, n: lapack_int, k: lapack_int,
                               a: *mut c_double, lda: lapack_int, tau: *const c_double,
                               work: *mut c_double, lwork: lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_sorgqr_work(matrix_layout: c_int, m: lapack_int, n: lapack_int, k: lapack_int,
                               a: *mut c_float, lda: lapack_int, tau: *const c_float,
                               work: *mut c_float, lwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dorgqr_work(matrix_layout: c_int, m: lapack_int, n: lapack_int, k: lapack_int,
                               a: *mut c_double, lda: lapack_int, tau: *const c_double,
                               work: *mut c_double, lwork: lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_sorgrq_work(matrix_layout: c_int, m: lapack_int, n: lapack_int, k: lapack_int,
                               a: *mut c_float, lda: lapack_int, tau: *const c_float,
                               work: *mut c_float, lwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dorgrq_work(matrix_layout: c_int, m: lapack_int, n: lapack_int, k: lapack_int,
                               a: *mut c_double, lda: lapack_int, tau: *const c_double,
                               work: *mut c_double, lwork: lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_sorgtr_work(matrix_layout: c_int, uplo: c_char, n: lapack_int, a: *mut c_float,
                               lda: lapack_int, tau: *const c_float, work: *mut c_float,
                               lwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dorgtr_work(matrix_layout: c_int, uplo: c_char, n: lapack_int, a: *mut c_double,
                               lda: lapack_int, tau: *const c_double, work: *mut c_double,
                               lwork: lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_sormbr_work(matrix_layout: c_int, vect: c_char, side: c_char, trans: c_char,
                               m: lapack_int, n: lapack_int, k: lapack_int, a: *const c_float,
                               lda: lapack_int, tau: *const c_float, c: *mut c_float,
                               ldc: lapack_int, work: *mut c_float, lwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dormbr_work(matrix_layout: c_int, vect: c_char, side: c_char, trans: c_char,
                               m: lapack_int, n: lapack_int, k: lapack_int, a: *const c_double,
                               lda: lapack_int, tau: *const c_double, c: *mut c_double,
                               ldc: lapack_int, work: *mut c_double, lwork: lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_sormhr_work(matrix_layout: c_int, side: c_char, trans: c_char, m: lapack_int,
                               n: lapack_int, ilo: lapack_int, ihi: lapack_int, a: *const c_float,
                               lda: lapack_int, tau: *const c_float, c: *mut c_float,
                               ldc: lapack_int, work: *mut c_float, lwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dormhr_work(matrix_layout: c_int, side: c_char, trans: c_char, m: lapack_int,
                               n: lapack_int, ilo: lapack_int, ihi: lapack_int, a: *const c_double,
                               lda: lapack_int, tau: *const c_double, c: *mut c_double,
                               ldc: lapack_int, work: *mut c_double, lwork: lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_sormlq_work(matrix_layout: c_int, side: c_char, trans: c_char, m: lapack_int,
                               n: lapack_int, k: lapack_int, a: *const c_float, lda: lapack_int,
                               tau: *const c_float, c: *mut c_float, ldc: lapack_int,
                               work: *mut c_float, lwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dormlq_work(matrix_layout: c_int, side: c_char, trans: c_char, m: lapack_int,
                               n: lapack_int, k: lapack_int, a: *const c_double, lda: lapack_int,
                               tau: *const c_double, c: *mut c_double, ldc: lapack_int,
                               work: *mut c_double, lwork: lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_sormql_work(matrix_layout: c_int, side: c_char, trans: c_char, m: lapack_int,
                               n: lapack_int, k: lapack_int, a: *const c_float, lda: lapack_int,
                               tau: *const c_float, c: *mut c_float, ldc: lapack_int,
                               work: *mut c_float, lwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dormql_work(matrix_layout: c_int, side: c_char, trans: c_char, m: lapack_int,
                               n: lapack_int, k: lapack_int, a: *const c_double, lda: lapack_int,
                               tau: *const c_double, c: *mut c_double, ldc: lapack_int,
                               work: *mut c_double, lwork: lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_sormqr_work(matrix_layout: c_int, side: c_char, trans: c_char, m: lapack_int,
                               n: lapack_int, k: lapack_int, a: *const c_float, lda: lapack_int,
                               tau: *const c_float, c: *mut c_float, ldc: lapack_int,
                               work: *mut c_float, lwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dormqr_work(matrix_layout: c_int, side: c_char, trans: c_char, m: lapack_int,
                               n: lapack_int, k: lapack_int, a: *const c_double, lda: lapack_int,
                               tau: *const c_double, c: *mut c_double, ldc: lapack_int,
                               work: *mut c_double, lwork: lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_sormrq_work(matrix_layout: c_int, side: c_char, trans: c_char, m: lapack_int,
                               n: lapack_int, k: lapack_int, a: *const c_float, lda: lapack_int,
                               tau: *const c_float, c: *mut c_float, ldc: lapack_int,
                               work: *mut c_float, lwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dormrq_work(matrix_layout: c_int, side: c_char, trans: c_char, m: lapack_int,
                               n: lapack_int, k: lapack_int, a: *const c_double, lda: lapack_int,
                               tau: *const c_double, c: *mut c_double, ldc: lapack_int,
                               work: *mut c_double, lwork: lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_sormrz_work(matrix_layout: c_int, side: c_char, trans: c_char, m: lapack_int,
                               n: lapack_int, k: lapack_int, l: lapack_int, a: *const c_float,
                               lda: lapack_int, tau: *const c_float, c: *mut c_float,
                               ldc: lapack_int, work: *mut c_float, lwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dormrz_work(matrix_layout: c_int, side: c_char, trans: c_char, m: lapack_int,
                               n: lapack_int, k: lapack_int, l: lapack_int, a: *const c_double,
                               lda: lapack_int, tau: *const c_double, c: *mut c_double,
                               ldc: lapack_int, work: *mut c_double, lwork: lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_sormtr_work(matrix_layout: c_int, side: c_char, uplo: c_char, trans: c_char,
                               m: lapack_int, n: lapack_int, a: *const c_float, lda: lapack_int,
                               tau: *const c_float, c: *mut c_float, ldc: lapack_int,
                               work: *mut c_float, lwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dormtr_work(matrix_layout: c_int, side: c_char, uplo: c_char, trans: c_char,
                               m: lapack_int, n: lapack_int, a: *const c_double, lda: lapack_int,
                               tau: *const c_double, c: *mut c_double, ldc: lapack_int,
                               work: *mut c_double, lwork: lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_spbcon_work(matrix_layout: c_int, uplo: c_char, n: lapack_int, kd: lapack_int,
                               ab: *const c_float, ldab: lapack_int, anorm: c_float,
                               rcond: *mut c_float, work: *mut c_float, iwork: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dpbcon_work(matrix_layout: c_int, uplo: c_char, n: lapack_int, kd: lapack_int,
                               ab: *const c_double, ldab: lapack_int, anorm: c_double,
                               rcond: *mut c_double, work: *mut c_double, iwork: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_cpbcon_work(matrix_layout: c_int, uplo: c_char, n: lapack_int, kd: lapack_int,
                               ab: *const lapack_complex_float, ldab: lapack_int, anorm: c_float,
                               rcond: *mut c_float, work: *mut lapack_complex_float,
                               rwork: *mut c_float)
                               -> lapack_int;
    pub fn LAPACKE_zpbcon_work(matrix_layout: c_int, uplo: c_char, n: lapack_int, kd: lapack_int,
                               ab: *const lapack_complex_double, ldab: lapack_int, anorm: c_double,
                               rcond: *mut c_double, work: *mut lapack_complex_double,
                               rwork: *mut c_double)
                               -> lapack_int;

    pub fn LAPACKE_spbequ_work(matrix_layout: c_int, uplo: c_char, n: lapack_int, kd: lapack_int,
                               ab: *const c_float, ldab: lapack_int, s: *mut c_float,
                               scond: *mut c_float, amax: *mut c_float)
                               -> lapack_int;
    pub fn LAPACKE_dpbequ_work(matrix_layout: c_int, uplo: c_char, n: lapack_int, kd: lapack_int,
                               ab: *const c_double, ldab: lapack_int, s: *mut c_double,
                               scond: *mut c_double, amax: *mut c_double)
                               -> lapack_int;
    pub fn LAPACKE_cpbequ_work(matrix_layout: c_int, uplo: c_char, n: lapack_int, kd: lapack_int,
                               ab: *const lapack_complex_float, ldab: lapack_int, s: *mut c_float,
                               scond: *mut c_float, amax: *mut c_float)
                               -> lapack_int;
    pub fn LAPACKE_zpbequ_work(matrix_layout: c_int, uplo: c_char, n: lapack_int, kd: lapack_int,
                               ab: *const lapack_complex_double, ldab: lapack_int,
                               s: *mut c_double, scond: *mut c_double, amax: *mut c_double)
                               -> lapack_int;

    pub fn LAPACKE_spbrfs_work(matrix_layout: c_int, uplo: c_char, n: lapack_int, kd: lapack_int,
                               nrhs: lapack_int, ab: *const c_float, ldab: lapack_int,
                               afb: *const c_float, ldafb: lapack_int, b: *const c_float,
                               ldb: lapack_int, x: *mut c_float, ldx: lapack_int,
                               ferr: *mut c_float, berr: *mut c_float, work: *mut c_float,
                               iwork: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dpbrfs_work(matrix_layout: c_int, uplo: c_char, n: lapack_int, kd: lapack_int,
                               nrhs: lapack_int, ab: *const c_double, ldab: lapack_int,
                               afb: *const c_double, ldafb: lapack_int, b: *const c_double,
                               ldb: lapack_int, x: *mut c_double, ldx: lapack_int,
                               ferr: *mut c_double, berr: *mut c_double, work: *mut c_double,
                               iwork: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_cpbrfs_work(matrix_layout: c_int, uplo: c_char, n: lapack_int, kd: lapack_int,
                               nrhs: lapack_int, ab: *const lapack_complex_float, ldab: lapack_int,
                               afb: *const lapack_complex_float, ldafb: lapack_int,
                               b: *const lapack_complex_float, ldb: lapack_int,
                               x: *mut lapack_complex_float, ldx: lapack_int, ferr: *mut c_float,
                               berr: *mut c_float, work: *mut lapack_complex_float,
                               rwork: *mut c_float)
                               -> lapack_int;
    pub fn LAPACKE_zpbrfs_work(matrix_layout: c_int, uplo: c_char, n: lapack_int, kd: lapack_int,
                               nrhs: lapack_int, ab: *const lapack_complex_double,
                               ldab: lapack_int, afb: *const lapack_complex_double,
                               ldafb: lapack_int, b: *const lapack_complex_double, ldb: lapack_int,
                               x: *mut lapack_complex_double, ldx: lapack_int, ferr: *mut c_double,
                               berr: *mut c_double, work: *mut lapack_complex_double,
                               rwork: *mut c_double)
                               -> lapack_int;

    pub fn LAPACKE_spbstf_work(matrix_layout: c_int, uplo: c_char, n: lapack_int, kb: lapack_int,
                               bb: *mut c_float, ldbb: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dpbstf_work(matrix_layout: c_int, uplo: c_char, n: lapack_int, kb: lapack_int,
                               bb: *mut c_double, ldbb: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_cpbstf_work(matrix_layout: c_int, uplo: c_char, n: lapack_int, kb: lapack_int,
                               bb: *mut lapack_complex_float, ldbb: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_zpbstf_work(matrix_layout: c_int, uplo: c_char, n: lapack_int, kb: lapack_int,
                               bb: *mut lapack_complex_double, ldbb: lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_spbsv_work(matrix_layout: c_int, uplo: c_char, n: lapack_int, kd: lapack_int,
                              nrhs: lapack_int, ab: *mut c_float, ldab: lapack_int,
                              b: *mut c_float, ldb: lapack_int)
                              -> lapack_int;
    pub fn LAPACKE_dpbsv_work(matrix_layout: c_int, uplo: c_char, n: lapack_int, kd: lapack_int,
                              nrhs: lapack_int, ab: *mut c_double, ldab: lapack_int,
                              b: *mut c_double, ldb: lapack_int)
                              -> lapack_int;
    pub fn LAPACKE_cpbsv_work(matrix_layout: c_int, uplo: c_char, n: lapack_int, kd: lapack_int,
                              nrhs: lapack_int, ab: *mut lapack_complex_float, ldab: lapack_int,
                              b: *mut lapack_complex_float, ldb: lapack_int)
                              -> lapack_int;
    pub fn LAPACKE_zpbsv_work(matrix_layout: c_int, uplo: c_char, n: lapack_int, kd: lapack_int,
                              nrhs: lapack_int, ab: *mut lapack_complex_double, ldab: lapack_int,
                              b: *mut lapack_complex_double, ldb: lapack_int)
                              -> lapack_int;

    pub fn LAPACKE_spbsvx_work(matrix_layout: c_int, fact: c_char, uplo: c_char, n: lapack_int,
                               kd: lapack_int, nrhs: lapack_int, ab: *mut c_float,
                               ldab: lapack_int, afb: *mut c_float, ldafb: lapack_int,
                               equed: *mut c_char, s: *mut c_float, b: *mut c_float,
                               ldb: lapack_int, x: *mut c_float, ldx: lapack_int,
                               rcond: *mut c_float, ferr: *mut c_float, berr: *mut c_float,
                               work: *mut c_float, iwork: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dpbsvx_work(matrix_layout: c_int, fact: c_char, uplo: c_char, n: lapack_int,
                               kd: lapack_int, nrhs: lapack_int, ab: *mut c_double,
                               ldab: lapack_int, afb: *mut c_double, ldafb: lapack_int,
                               equed: *mut c_char, s: *mut c_double, b: *mut c_double,
                               ldb: lapack_int, x: *mut c_double, ldx: lapack_int,
                               rcond: *mut c_double, ferr: *mut c_double, berr: *mut c_double,
                               work: *mut c_double, iwork: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_cpbsvx_work(matrix_layout: c_int, fact: c_char, uplo: c_char, n: lapack_int,
                               kd: lapack_int, nrhs: lapack_int, ab: *mut lapack_complex_float,
                               ldab: lapack_int, afb: *mut lapack_complex_float, ldafb: lapack_int,
                               equed: *mut c_char, s: *mut c_float, b: *mut lapack_complex_float,
                               ldb: lapack_int, x: *mut lapack_complex_float, ldx: lapack_int,
                               rcond: *mut c_float, ferr: *mut c_float, berr: *mut c_float,
                               work: *mut lapack_complex_float, rwork: *mut c_float)
                               -> lapack_int;
    pub fn LAPACKE_zpbsvx_work(matrix_layout: c_int, fact: c_char, uplo: c_char, n: lapack_int,
                               kd: lapack_int, nrhs: lapack_int, ab: *mut lapack_complex_double,
                               ldab: lapack_int, afb: *mut lapack_complex_double,
                               ldafb: lapack_int, equed: *mut c_char, s: *mut c_double,
                               b: *mut lapack_complex_double, ldb: lapack_int,
                               x: *mut lapack_complex_double, ldx: lapack_int,
                               rcond: *mut c_double, ferr: *mut c_double, berr: *mut c_double,
                               work: *mut lapack_complex_double, rwork: *mut c_double)
                               -> lapack_int;

    pub fn LAPACKE_spbtrf_work(matrix_layout: c_int, uplo: c_char, n: lapack_int, kd: lapack_int,
                               ab: *mut c_float, ldab: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dpbtrf_work(matrix_layout: c_int, uplo: c_char, n: lapack_int, kd: lapack_int,
                               ab: *mut c_double, ldab: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_cpbtrf_work(matrix_layout: c_int, uplo: c_char, n: lapack_int, kd: lapack_int,
                               ab: *mut lapack_complex_float, ldab: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_zpbtrf_work(matrix_layout: c_int, uplo: c_char, n: lapack_int, kd: lapack_int,
                               ab: *mut lapack_complex_double, ldab: lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_spbtrs_work(matrix_layout: c_int, uplo: c_char, n: lapack_int, kd: lapack_int,
                               nrhs: lapack_int, ab: *const c_float, ldab: lapack_int,
                               b: *mut c_float, ldb: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dpbtrs_work(matrix_layout: c_int, uplo: c_char, n: lapack_int, kd: lapack_int,
                               nrhs: lapack_int, ab: *const c_double, ldab: lapack_int,
                               b: *mut c_double, ldb: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_cpbtrs_work(matrix_layout: c_int, uplo: c_char, n: lapack_int, kd: lapack_int,
                               nrhs: lapack_int, ab: *const lapack_complex_float, ldab: lapack_int,
                               b: *mut lapack_complex_float, ldb: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_zpbtrs_work(matrix_layout: c_int, uplo: c_char, n: lapack_int, kd: lapack_int,
                               nrhs: lapack_int, ab: *const lapack_complex_double,
                               ldab: lapack_int, b: *mut lapack_complex_double, ldb: lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_spftrf_work(matrix_layout: c_int, transr: c_char, uplo: c_char, n: lapack_int,
                               a: *mut c_float)
                               -> lapack_int;
    pub fn LAPACKE_dpftrf_work(matrix_layout: c_int, transr: c_char, uplo: c_char, n: lapack_int,
                               a: *mut c_double)
                               -> lapack_int;
    pub fn LAPACKE_cpftrf_work(matrix_layout: c_int, transr: c_char, uplo: c_char, n: lapack_int,
                               a: *mut lapack_complex_float)
                               -> lapack_int;
    pub fn LAPACKE_zpftrf_work(matrix_layout: c_int, transr: c_char, uplo: c_char, n: lapack_int,
                               a: *mut lapack_complex_double)
                               -> lapack_int;

    pub fn LAPACKE_spftri_work(matrix_layout: c_int, transr: c_char, uplo: c_char, n: lapack_int,
                               a: *mut c_float)
                               -> lapack_int;
    pub fn LAPACKE_dpftri_work(matrix_layout: c_int, transr: c_char, uplo: c_char, n: lapack_int,
                               a: *mut c_double)
                               -> lapack_int;
    pub fn LAPACKE_cpftri_work(matrix_layout: c_int, transr: c_char, uplo: c_char, n: lapack_int,
                               a: *mut lapack_complex_float)
                               -> lapack_int;
    pub fn LAPACKE_zpftri_work(matrix_layout: c_int, transr: c_char, uplo: c_char, n: lapack_int,
                               a: *mut lapack_complex_double)
                               -> lapack_int;

    pub fn LAPACKE_spftrs_work(matrix_layout: c_int, transr: c_char, uplo: c_char, n: lapack_int,
                               nrhs: lapack_int, a: *const c_float, b: *mut c_float,
                               ldb: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dpftrs_work(matrix_layout: c_int, transr: c_char, uplo: c_char, n: lapack_int,
                               nrhs: lapack_int, a: *const c_double, b: *mut c_double,
                               ldb: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_cpftrs_work(matrix_layout: c_int, transr: c_char, uplo: c_char, n: lapack_int,
                               nrhs: lapack_int, a: *const lapack_complex_float,
                               b: *mut lapack_complex_float, ldb: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_zpftrs_work(matrix_layout: c_int, transr: c_char, uplo: c_char, n: lapack_int,
                               nrhs: lapack_int, a: *const lapack_complex_double,
                               b: *mut lapack_complex_double, ldb: lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_spocon_work(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                               a: *const c_float, lda: lapack_int, anorm: c_float,
                               rcond: *mut c_float, work: *mut c_float, iwork: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dpocon_work(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                               a: *const c_double, lda: lapack_int, anorm: c_double,
                               rcond: *mut c_double, work: *mut c_double, iwork: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_cpocon_work(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                               a: *const lapack_complex_float, lda: lapack_int, anorm: c_float,
                               rcond: *mut c_float, work: *mut lapack_complex_float,
                               rwork: *mut c_float)
                               -> lapack_int;
    pub fn LAPACKE_zpocon_work(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                               a: *const lapack_complex_double, lda: lapack_int, anorm: c_double,
                               rcond: *mut c_double, work: *mut lapack_complex_double,
                               rwork: *mut c_double)
                               -> lapack_int;

    pub fn LAPACKE_spoequ_work(matrix_layout: c_int, n: lapack_int, a: *const c_float,
                               lda: lapack_int, s: *mut c_float, scond: *mut c_float,
                               amax: *mut c_float)
                               -> lapack_int;
    pub fn LAPACKE_dpoequ_work(matrix_layout: c_int, n: lapack_int, a: *const c_double,
                               lda: lapack_int, s: *mut c_double, scond: *mut c_double,
                               amax: *mut c_double)
                               -> lapack_int;
    pub fn LAPACKE_cpoequ_work(matrix_layout: c_int, n: lapack_int, a: *const lapack_complex_float,
                               lda: lapack_int, s: *mut c_float, scond: *mut c_float,
                               amax: *mut c_float)
                               -> lapack_int;
    pub fn LAPACKE_zpoequ_work(matrix_layout: c_int, n: lapack_int,
                               a: *const lapack_complex_double, lda: lapack_int, s: *mut c_double,
                               scond: *mut c_double, amax: *mut c_double)
                               -> lapack_int;

    pub fn LAPACKE_spoequb_work(matrix_layout: c_int, n: lapack_int, a: *const c_float,
                                lda: lapack_int, s: *mut c_float, scond: *mut c_float,
                                amax: *mut c_float)
                                -> lapack_int;
    pub fn LAPACKE_dpoequb_work(matrix_layout: c_int, n: lapack_int, a: *const c_double,
                                lda: lapack_int, s: *mut c_double, scond: *mut c_double,
                                amax: *mut c_double)
                                -> lapack_int;
    pub fn LAPACKE_cpoequb_work(matrix_layout: c_int, n: lapack_int,
                                a: *const lapack_complex_float, lda: lapack_int, s: *mut c_float,
                                scond: *mut c_float, amax: *mut c_float)
                                -> lapack_int;
    pub fn LAPACKE_zpoequb_work(matrix_layout: c_int, n: lapack_int,
                                a: *const lapack_complex_double, lda: lapack_int, s: *mut c_double,
                                scond: *mut c_double, amax: *mut c_double)
                                -> lapack_int;

    pub fn LAPACKE_sporfs_work(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                               a: *const c_float, lda: lapack_int, af: *const c_float,
                               ldaf: lapack_int, b: *const c_float, ldb: lapack_int,
                               x: *mut c_float, ldx: lapack_int, ferr: *mut c_float,
                               berr: *mut c_float, work: *mut c_float, iwork: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dporfs_work(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                               a: *const c_double, lda: lapack_int, af: *const c_double,
                               ldaf: lapack_int, b: *const c_double, ldb: lapack_int,
                               x: *mut c_double, ldx: lapack_int, ferr: *mut c_double,
                               berr: *mut c_double, work: *mut c_double, iwork: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_cporfs_work(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                               a: *const lapack_complex_float, lda: lapack_int,
                               af: *const lapack_complex_float, ldaf: lapack_int,
                               b: *const lapack_complex_float, ldb: lapack_int,
                               x: *mut lapack_complex_float, ldx: lapack_int, ferr: *mut c_float,
                               berr: *mut c_float, work: *mut lapack_complex_float,
                               rwork: *mut c_float)
                               -> lapack_int;
    pub fn LAPACKE_zporfs_work(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                               a: *const lapack_complex_double, lda: lapack_int,
                               af: *const lapack_complex_double, ldaf: lapack_int,
                               b: *const lapack_complex_double, ldb: lapack_int,
                               x: *mut lapack_complex_double, ldx: lapack_int, ferr: *mut c_double,
                               berr: *mut c_double, work: *mut lapack_complex_double,
                               rwork: *mut c_double)
                               -> lapack_int;

    pub fn LAPACKE_sporfsx_work(matrix_layout: c_int, uplo: c_char, equed: c_char, n: lapack_int,
                                nrhs: lapack_int, a: *const c_float, lda: lapack_int,
                                af: *const c_float, ldaf: lapack_int, s: *const c_float,
                                b: *const c_float, ldb: lapack_int, x: *mut c_float,
                                ldx: lapack_int, rcond: *mut c_float, berr: *mut c_float,
                                n_err_bnds: lapack_int, err_bnds_norm: *mut c_float,
                                err_bnds_comp: *mut c_float, nparams: lapack_int,
                                params: *mut c_float, work: *mut c_float, iwork: *mut lapack_int)
                                -> lapack_int;
    pub fn LAPACKE_dporfsx_work(matrix_layout: c_int, uplo: c_char, equed: c_char, n: lapack_int,
                                nrhs: lapack_int, a: *const c_double, lda: lapack_int,
                                af: *const c_double, ldaf: lapack_int, s: *const c_double,
                                b: *const c_double, ldb: lapack_int, x: *mut c_double,
                                ldx: lapack_int, rcond: *mut c_double, berr: *mut c_double,
                                n_err_bnds: lapack_int, err_bnds_norm: *mut c_double,
                                err_bnds_comp: *mut c_double, nparams: lapack_int,
                                params: *mut c_double, work: *mut c_double, iwork: *mut lapack_int)
                                -> lapack_int;
    pub fn LAPACKE_cporfsx_work(matrix_layout: c_int, uplo: c_char, equed: c_char, n: lapack_int,
                                nrhs: lapack_int, a: *const lapack_complex_float, lda: lapack_int,
                                af: *const lapack_complex_float, ldaf: lapack_int,
                                s: *const c_float, b: *const lapack_complex_float, ldb: lapack_int,
                                x: *mut lapack_complex_float, ldx: lapack_int, rcond: *mut c_float,
                                berr: *mut c_float, n_err_bnds: lapack_int,
                                err_bnds_norm: *mut c_float, err_bnds_comp: *mut c_float,
                                nparams: lapack_int, params: *mut c_float,
                                work: *mut lapack_complex_float, rwork: *mut c_float)
                                -> lapack_int;
    pub fn LAPACKE_zporfsx_work(matrix_layout: c_int, uplo: c_char, equed: c_char, n: lapack_int,
                                nrhs: lapack_int, a: *const lapack_complex_double, lda: lapack_int,
                                af: *const lapack_complex_double, ldaf: lapack_int,
                                s: *const c_double, b: *const lapack_complex_double,
                                ldb: lapack_int, x: *mut lapack_complex_double, ldx: lapack_int,
                                rcond: *mut c_double, berr: *mut c_double, n_err_bnds: lapack_int,
                                err_bnds_norm: *mut c_double, err_bnds_comp: *mut c_double,
                                nparams: lapack_int, params: *mut c_double,
                                work: *mut lapack_complex_double, rwork: *mut c_double)
                                -> lapack_int;

    pub fn LAPACKE_sposv_work(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                              a: *mut c_float, lda: lapack_int, b: *mut c_float, ldb: lapack_int)
                              -> lapack_int;
    pub fn LAPACKE_dposv_work(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                              a: *mut c_double, lda: lapack_int, b: *mut c_double, ldb: lapack_int)
                              -> lapack_int;
    pub fn LAPACKE_cposv_work(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                              a: *mut lapack_complex_float, lda: lapack_int,
                              b: *mut lapack_complex_float, ldb: lapack_int)
                              -> lapack_int;
    pub fn LAPACKE_zposv_work(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                              a: *mut lapack_complex_double, lda: lapack_int,
                              b: *mut lapack_complex_double, ldb: lapack_int)
                              -> lapack_int;
    pub fn LAPACKE_dsposv_work(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                               a: *mut c_double, lda: lapack_int, b: *mut c_double,
                               ldb: lapack_int, x: *mut c_double, ldx: lapack_int,
                               work: *mut c_double, swork: *mut c_float, iter: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_zcposv_work(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                               a: *mut lapack_complex_double, lda: lapack_int,
                               b: *mut lapack_complex_double, ldb: lapack_int,
                               x: *mut lapack_complex_double, ldx: lapack_int,
                               work: *mut lapack_complex_double, swork: *mut lapack_complex_float,
                               rwork: *mut c_double, iter: *mut lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_sposvx_work(matrix_layout: c_int, fact: c_char, uplo: c_char, n: lapack_int,
                               nrhs: lapack_int, a: *mut c_float, lda: lapack_int,
                               af: *mut c_float, ldaf: lapack_int, equed: *mut c_char,
                               s: *mut c_float, b: *mut c_float, ldb: lapack_int, x: *mut c_float,
                               ldx: lapack_int, rcond: *mut c_float, ferr: *mut c_float,
                               berr: *mut c_float, work: *mut c_float, iwork: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dposvx_work(matrix_layout: c_int, fact: c_char, uplo: c_char, n: lapack_int,
                               nrhs: lapack_int, a: *mut c_double, lda: lapack_int,
                               af: *mut c_double, ldaf: lapack_int, equed: *mut c_char,
                               s: *mut c_double, b: *mut c_double, ldb: lapack_int,
                               x: *mut c_double, ldx: lapack_int, rcond: *mut c_double,
                               ferr: *mut c_double, berr: *mut c_double, work: *mut c_double,
                               iwork: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_cposvx_work(matrix_layout: c_int, fact: c_char, uplo: c_char, n: lapack_int,
                               nrhs: lapack_int, a: *mut lapack_complex_float, lda: lapack_int,
                               af: *mut lapack_complex_float, ldaf: lapack_int, equed: *mut c_char,
                               s: *mut c_float, b: *mut lapack_complex_float, ldb: lapack_int,
                               x: *mut lapack_complex_float, ldx: lapack_int, rcond: *mut c_float,
                               ferr: *mut c_float, berr: *mut c_float,
                               work: *mut lapack_complex_float, rwork: *mut c_float)
                               -> lapack_int;
    pub fn LAPACKE_zposvx_work(matrix_layout: c_int, fact: c_char, uplo: c_char, n: lapack_int,
                               nrhs: lapack_int, a: *mut lapack_complex_double, lda: lapack_int,
                               af: *mut lapack_complex_double, ldaf: lapack_int,
                               equed: *mut c_char, s: *mut c_double, b: *mut lapack_complex_double,
                               ldb: lapack_int, x: *mut lapack_complex_double, ldx: lapack_int,
                               rcond: *mut c_double, ferr: *mut c_double, berr: *mut c_double,
                               work: *mut lapack_complex_double, rwork: *mut c_double)
                               -> lapack_int;

    pub fn LAPACKE_sposvxx_work(matrix_layout: c_int, fact: c_char, uplo: c_char, n: lapack_int,
                                nrhs: lapack_int, a: *mut c_float, lda: lapack_int,
                                af: *mut c_float, ldaf: lapack_int, equed: *mut c_char,
                                s: *mut c_float, b: *mut c_float, ldb: lapack_int, x: *mut c_float,
                                ldx: lapack_int, rcond: *mut c_float, rpvgrw: *mut c_float,
                                berr: *mut c_float, n_err_bnds: lapack_int,
                                err_bnds_norm: *mut c_float, err_bnds_comp: *mut c_float,
                                nparams: lapack_int, params: *mut c_float, work: *mut c_float,
                                iwork: *mut lapack_int)
                                -> lapack_int;
    pub fn LAPACKE_dposvxx_work(matrix_layout: c_int, fact: c_char, uplo: c_char, n: lapack_int,
                                nrhs: lapack_int, a: *mut c_double, lda: lapack_int,
                                af: *mut c_double, ldaf: lapack_int, equed: *mut c_char,
                                s: *mut c_double, b: *mut c_double, ldb: lapack_int,
                                x: *mut c_double, ldx: lapack_int, rcond: *mut c_double,
                                rpvgrw: *mut c_double, berr: *mut c_double, n_err_bnds: lapack_int,
                                err_bnds_norm: *mut c_double, err_bnds_comp: *mut c_double,
                                nparams: lapack_int, params: *mut c_double, work: *mut c_double,
                                iwork: *mut lapack_int)
                                -> lapack_int;
    pub fn LAPACKE_cposvxx_work(matrix_layout: c_int, fact: c_char, uplo: c_char, n: lapack_int,
                                nrhs: lapack_int, a: *mut lapack_complex_float, lda: lapack_int,
                                af: *mut lapack_complex_float, ldaf: lapack_int,
                                equed: *mut c_char, s: *mut c_float, b: *mut lapack_complex_float,
                                ldb: lapack_int, x: *mut lapack_complex_float, ldx: lapack_int,
                                rcond: *mut c_float, rpvgrw: *mut c_float, berr: *mut c_float,
                                n_err_bnds: lapack_int, err_bnds_norm: *mut c_float,
                                err_bnds_comp: *mut c_float, nparams: lapack_int,
                                params: *mut c_float, work: *mut lapack_complex_float,
                                rwork: *mut c_float)
                                -> lapack_int;
    pub fn LAPACKE_zposvxx_work(matrix_layout: c_int, fact: c_char, uplo: c_char, n: lapack_int,
                                nrhs: lapack_int, a: *mut lapack_complex_double, lda: lapack_int,
                                af: *mut lapack_complex_double, ldaf: lapack_int,
                                equed: *mut c_char, s: *mut c_double,
                                b: *mut lapack_complex_double, ldb: lapack_int,
                                x: *mut lapack_complex_double, ldx: lapack_int,
                                rcond: *mut c_double, rpvgrw: *mut c_double, berr: *mut c_double,
                                n_err_bnds: lapack_int, err_bnds_norm: *mut c_double,
                                err_bnds_comp: *mut c_double, nparams: lapack_int,
                                params: *mut c_double, work: *mut lapack_complex_double,
                                rwork: *mut c_double)
                                -> lapack_int;

    pub fn LAPACKE_spotrf2_work(matrix_layout: c_int, uplo: c_char, n: lapack_int, a: *mut c_float,
                                lda: lapack_int)
                                -> lapack_int;
    pub fn LAPACKE_dpotrf2_work(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                                a: *mut c_double, lda: lapack_int)
                                -> lapack_int;
    pub fn LAPACKE_cpotrf2_work(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                                a: *mut lapack_complex_float, lda: lapack_int)
                                -> lapack_int;
    pub fn LAPACKE_zpotrf2_work(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                                a: *mut lapack_complex_double, lda: lapack_int)
                                -> lapack_int;

    pub fn LAPACKE_spotrf_work(matrix_layout: c_int, uplo: c_char, n: lapack_int, a: *mut c_float,
                               lda: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dpotrf_work(matrix_layout: c_int, uplo: c_char, n: lapack_int, a: *mut c_double,
                               lda: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_cpotrf_work(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                               a: *mut lapack_complex_float, lda: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_zpotrf_work(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                               a: *mut lapack_complex_double, lda: lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_spotri_work(matrix_layout: c_int, uplo: c_char, n: lapack_int, a: *mut c_float,
                               lda: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dpotri_work(matrix_layout: c_int, uplo: c_char, n: lapack_int, a: *mut c_double,
                               lda: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_cpotri_work(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                               a: *mut lapack_complex_float, lda: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_zpotri_work(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                               a: *mut lapack_complex_double, lda: lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_spotrs_work(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                               a: *const c_float, lda: lapack_int, b: *mut c_float,
                               ldb: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dpotrs_work(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                               a: *const c_double, lda: lapack_int, b: *mut c_double,
                               ldb: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_cpotrs_work(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                               a: *const lapack_complex_float, lda: lapack_int,
                               b: *mut lapack_complex_float, ldb: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_zpotrs_work(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                               a: *const lapack_complex_double, lda: lapack_int,
                               b: *mut lapack_complex_double, ldb: lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_sppcon_work(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                               ap: *const c_float, anorm: c_float, rcond: *mut c_float,
                               work: *mut c_float, iwork: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dppcon_work(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                               ap: *const c_double, anorm: c_double, rcond: *mut c_double,
                               work: *mut c_double, iwork: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_cppcon_work(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                               ap: *const lapack_complex_float, anorm: c_float,
                               rcond: *mut c_float, work: *mut lapack_complex_float,
                               rwork: *mut c_float)
                               -> lapack_int;
    pub fn LAPACKE_zppcon_work(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                               ap: *const lapack_complex_double, anorm: c_double,
                               rcond: *mut c_double, work: *mut lapack_complex_double,
                               rwork: *mut c_double)
                               -> lapack_int;

    pub fn LAPACKE_sppequ_work(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                               ap: *const c_float, s: *mut c_float, scond: *mut c_float,
                               amax: *mut c_float)
                               -> lapack_int;
    pub fn LAPACKE_dppequ_work(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                               ap: *const c_double, s: *mut c_double, scond: *mut c_double,
                               amax: *mut c_double)
                               -> lapack_int;
    pub fn LAPACKE_cppequ_work(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                               ap: *const lapack_complex_float, s: *mut c_float,
                               scond: *mut c_float, amax: *mut c_float)
                               -> lapack_int;
    pub fn LAPACKE_zppequ_work(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                               ap: *const lapack_complex_double, s: *mut c_double,
                               scond: *mut c_double, amax: *mut c_double)
                               -> lapack_int;

    pub fn LAPACKE_spprfs_work(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                               ap: *const c_float, afp: *const c_float, b: *const c_float,
                               ldb: lapack_int, x: *mut c_float, ldx: lapack_int,
                               ferr: *mut c_float, berr: *mut c_float, work: *mut c_float,
                               iwork: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dpprfs_work(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                               ap: *const c_double, afp: *const c_double, b: *const c_double,
                               ldb: lapack_int, x: *mut c_double, ldx: lapack_int,
                               ferr: *mut c_double, berr: *mut c_double, work: *mut c_double,
                               iwork: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_cpprfs_work(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                               ap: *const lapack_complex_float, afp: *const lapack_complex_float,
                               b: *const lapack_complex_float, ldb: lapack_int,
                               x: *mut lapack_complex_float, ldx: lapack_int, ferr: *mut c_float,
                               berr: *mut c_float, work: *mut lapack_complex_float,
                               rwork: *mut c_float)
                               -> lapack_int;
    pub fn LAPACKE_zpprfs_work(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                               ap: *const lapack_complex_double, afp: *const lapack_complex_double,
                               b: *const lapack_complex_double, ldb: lapack_int,
                               x: *mut lapack_complex_double, ldx: lapack_int, ferr: *mut c_double,
                               berr: *mut c_double, work: *mut lapack_complex_double,
                               rwork: *mut c_double)
                               -> lapack_int;

    pub fn LAPACKE_sppsv_work(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                              ap: *mut c_float, b: *mut c_float, ldb: lapack_int)
                              -> lapack_int;
    pub fn LAPACKE_dppsv_work(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                              ap: *mut c_double, b: *mut c_double, ldb: lapack_int)
                              -> lapack_int;
    pub fn LAPACKE_cppsv_work(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                              ap: *mut lapack_complex_float, b: *mut lapack_complex_float,
                              ldb: lapack_int)
                              -> lapack_int;
    pub fn LAPACKE_zppsv_work(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                              ap: *mut lapack_complex_double, b: *mut lapack_complex_double,
                              ldb: lapack_int)
                              -> lapack_int;

    pub fn LAPACKE_sppsvx_work(matrix_layout: c_int, fact: c_char, uplo: c_char, n: lapack_int,
                               nrhs: lapack_int, ap: *mut c_float, afp: *mut c_float,
                               equed: *mut c_char, s: *mut c_float, b: *mut c_float,
                               ldb: lapack_int, x: *mut c_float, ldx: lapack_int,
                               rcond: *mut c_float, ferr: *mut c_float, berr: *mut c_float,
                               work: *mut c_float, iwork: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dppsvx_work(matrix_layout: c_int, fact: c_char, uplo: c_char, n: lapack_int,
                               nrhs: lapack_int, ap: *mut c_double, afp: *mut c_double,
                               equed: *mut c_char, s: *mut c_double, b: *mut c_double,
                               ldb: lapack_int, x: *mut c_double, ldx: lapack_int,
                               rcond: *mut c_double, ferr: *mut c_double, berr: *mut c_double,
                               work: *mut c_double, iwork: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_cppsvx_work(matrix_layout: c_int, fact: c_char, uplo: c_char, n: lapack_int,
                               nrhs: lapack_int, ap: *mut lapack_complex_float,
                               afp: *mut lapack_complex_float, equed: *mut c_char, s: *mut c_float,
                               b: *mut lapack_complex_float, ldb: lapack_int,
                               x: *mut lapack_complex_float, ldx: lapack_int, rcond: *mut c_float,
                               ferr: *mut c_float, berr: *mut c_float,
                               work: *mut lapack_complex_float, rwork: *mut c_float)
                               -> lapack_int;
    pub fn LAPACKE_zppsvx_work(matrix_layout: c_int, fact: c_char, uplo: c_char, n: lapack_int,
                               nrhs: lapack_int, ap: *mut lapack_complex_double,
                               afp: *mut lapack_complex_double, equed: *mut c_char,
                               s: *mut c_double, b: *mut lapack_complex_double, ldb: lapack_int,
                               x: *mut lapack_complex_double, ldx: lapack_int,
                               rcond: *mut c_double, ferr: *mut c_double, berr: *mut c_double,
                               work: *mut lapack_complex_double, rwork: *mut c_double)
                               -> lapack_int;

    pub fn LAPACKE_spptrf_work(matrix_layout: c_int, uplo: c_char, n: lapack_int, ap: *mut c_float)
                               -> lapack_int;
    pub fn LAPACKE_dpptrf_work(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                               ap: *mut c_double)
                               -> lapack_int;
    pub fn LAPACKE_cpptrf_work(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                               ap: *mut lapack_complex_float)
                               -> lapack_int;
    pub fn LAPACKE_zpptrf_work(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                               ap: *mut lapack_complex_double)
                               -> lapack_int;

    pub fn LAPACKE_spptri_work(matrix_layout: c_int, uplo: c_char, n: lapack_int, ap: *mut c_float)
                               -> lapack_int;
    pub fn LAPACKE_dpptri_work(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                               ap: *mut c_double)
                               -> lapack_int;
    pub fn LAPACKE_cpptri_work(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                               ap: *mut lapack_complex_float)
                               -> lapack_int;
    pub fn LAPACKE_zpptri_work(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                               ap: *mut lapack_complex_double)
                               -> lapack_int;

    pub fn LAPACKE_spptrs_work(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                               ap: *const c_float, b: *mut c_float, ldb: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dpptrs_work(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                               ap: *const c_double, b: *mut c_double, ldb: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_cpptrs_work(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                               ap: *const lapack_complex_float, b: *mut lapack_complex_float,
                               ldb: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_zpptrs_work(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                               ap: *const lapack_complex_double, b: *mut lapack_complex_double,
                               ldb: lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_spstrf_work(matrix_layout: c_int, uplo: c_char, n: lapack_int, a: *mut c_float,
                               lda: lapack_int, piv: *mut lapack_int, rank: *mut lapack_int,
                               tol: c_float, work: *mut c_float)
                               -> lapack_int;
    pub fn LAPACKE_dpstrf_work(matrix_layout: c_int, uplo: c_char, n: lapack_int, a: *mut c_double,
                               lda: lapack_int, piv: *mut lapack_int, rank: *mut lapack_int,
                               tol: c_double, work: *mut c_double)
                               -> lapack_int;
    pub fn LAPACKE_cpstrf_work(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                               a: *mut lapack_complex_float, lda: lapack_int, piv: *mut lapack_int,
                               rank: *mut lapack_int, tol: c_float, work: *mut c_float)
                               -> lapack_int;
    pub fn LAPACKE_zpstrf_work(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                               a: *mut lapack_complex_double, lda: lapack_int,
                               piv: *mut lapack_int, rank: *mut lapack_int, tol: c_double,
                               work: *mut c_double)
                               -> lapack_int;

    pub fn LAPACKE_sptcon_work(n: lapack_int, d: *const c_float, e: *const c_float, anorm: c_float,
                               rcond: *mut c_float, work: *mut c_float)
                               -> lapack_int;
    pub fn LAPACKE_dptcon_work(n: lapack_int, d: *const c_double, e: *const c_double,
                               anorm: c_double, rcond: *mut c_double, work: *mut c_double)
                               -> lapack_int;
    pub fn LAPACKE_cptcon_work(n: lapack_int, d: *const c_float, e: *const lapack_complex_float,
                               anorm: c_float, rcond: *mut c_float, work: *mut c_float)
                               -> lapack_int;
    pub fn LAPACKE_zptcon_work(n: lapack_int, d: *const c_double, e: *const lapack_complex_double,
                               anorm: c_double, rcond: *mut c_double, work: *mut c_double)
                               -> lapack_int;

    pub fn LAPACKE_spteqr_work(matrix_layout: c_int, compz: c_char, n: lapack_int, d: *mut c_float,
                               e: *mut c_float, z: *mut c_float, ldz: lapack_int,
                               work: *mut c_float)
                               -> lapack_int;
    pub fn LAPACKE_dpteqr_work(matrix_layout: c_int, compz: c_char, n: lapack_int,
                               d: *mut c_double, e: *mut c_double, z: *mut c_double,
                               ldz: lapack_int, work: *mut c_double)
                               -> lapack_int;
    pub fn LAPACKE_cpteqr_work(matrix_layout: c_int, compz: c_char, n: lapack_int, d: *mut c_float,
                               e: *mut c_float, z: *mut lapack_complex_float, ldz: lapack_int,
                               work: *mut c_float)
                               -> lapack_int;
    pub fn LAPACKE_zpteqr_work(matrix_layout: c_int, compz: c_char, n: lapack_int,
                               d: *mut c_double, e: *mut c_double, z: *mut lapack_complex_double,
                               ldz: lapack_int, work: *mut c_double)
                               -> lapack_int;

    pub fn LAPACKE_sptrfs_work(matrix_layout: c_int, n: lapack_int, nrhs: lapack_int,
                               d: *const c_float, e: *const c_float, df: *const c_float,
                               ef: *const c_float, b: *const c_float, ldb: lapack_int,
                               x: *mut c_float, ldx: lapack_int, ferr: *mut c_float,
                               berr: *mut c_float, work: *mut c_float)
                               -> lapack_int;
    pub fn LAPACKE_dptrfs_work(matrix_layout: c_int, n: lapack_int, nrhs: lapack_int,
                               d: *const c_double, e: *const c_double, df: *const c_double,
                               ef: *const c_double, b: *const c_double, ldb: lapack_int,
                               x: *mut c_double, ldx: lapack_int, ferr: *mut c_double,
                               berr: *mut c_double, work: *mut c_double)
                               -> lapack_int;
    pub fn LAPACKE_cptrfs_work(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                               d: *const c_float, e: *const lapack_complex_float,
                               df: *const c_float, ef: *const lapack_complex_float,
                               b: *const lapack_complex_float, ldb: lapack_int,
                               x: *mut lapack_complex_float, ldx: lapack_int, ferr: *mut c_float,
                               berr: *mut c_float, work: *mut lapack_complex_float,
                               rwork: *mut c_float)
                               -> lapack_int;
    pub fn LAPACKE_zptrfs_work(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                               d: *const c_double, e: *const lapack_complex_double,
                               df: *const c_double, ef: *const lapack_complex_double,
                               b: *const lapack_complex_double, ldb: lapack_int,
                               x: *mut lapack_complex_double, ldx: lapack_int, ferr: *mut c_double,
                               berr: *mut c_double, work: *mut lapack_complex_double,
                               rwork: *mut c_double)
                               -> lapack_int;

    pub fn LAPACKE_sptsv_work(matrix_layout: c_int, n: lapack_int, nrhs: lapack_int,
                              d: *mut c_float, e: *mut c_float, b: *mut c_float, ldb: lapack_int)
                              -> lapack_int;
    pub fn LAPACKE_dptsv_work(matrix_layout: c_int, n: lapack_int, nrhs: lapack_int,
                              d: *mut c_double, e: *mut c_double, b: *mut c_double,
                              ldb: lapack_int)
                              -> lapack_int;
    pub fn LAPACKE_cptsv_work(matrix_layout: c_int, n: lapack_int, nrhs: lapack_int,
                              d: *mut c_float, e: *mut lapack_complex_float,
                              b: *mut lapack_complex_float, ldb: lapack_int)
                              -> lapack_int;
    pub fn LAPACKE_zptsv_work(matrix_layout: c_int, n: lapack_int, nrhs: lapack_int,
                              d: *mut c_double, e: *mut lapack_complex_double,
                              b: *mut lapack_complex_double, ldb: lapack_int)
                              -> lapack_int;

    pub fn LAPACKE_sptsvx_work(matrix_layout: c_int, fact: c_char, n: lapack_int, nrhs: lapack_int,
                               d: *const c_float, e: *const c_float, df: *mut c_float,
                               ef: *mut c_float, b: *const c_float, ldb: lapack_int,
                               x: *mut c_float, ldx: lapack_int, rcond: *mut c_float,
                               ferr: *mut c_float, berr: *mut c_float, work: *mut c_float)
                               -> lapack_int;
    pub fn LAPACKE_dptsvx_work(matrix_layout: c_int, fact: c_char, n: lapack_int, nrhs: lapack_int,
                               d: *const c_double, e: *const c_double, df: *mut c_double,
                               ef: *mut c_double, b: *const c_double, ldb: lapack_int,
                               x: *mut c_double, ldx: lapack_int, rcond: *mut c_double,
                               ferr: *mut c_double, berr: *mut c_double, work: *mut c_double)
                               -> lapack_int;
    pub fn LAPACKE_cptsvx_work(matrix_layout: c_int, fact: c_char, n: lapack_int, nrhs: lapack_int,
                               d: *const c_float, e: *const lapack_complex_float, df: *mut c_float,
                               ef: *mut lapack_complex_float, b: *const lapack_complex_float,
                               ldb: lapack_int, x: *mut lapack_complex_float, ldx: lapack_int,
                               rcond: *mut c_float, ferr: *mut c_float, berr: *mut c_float,
                               work: *mut lapack_complex_float, rwork: *mut c_float)
                               -> lapack_int;
    pub fn LAPACKE_zptsvx_work(matrix_layout: c_int, fact: c_char, n: lapack_int, nrhs: lapack_int,
                               d: *const c_double, e: *const lapack_complex_double,
                               df: *mut c_double, ef: *mut lapack_complex_double,
                               b: *const lapack_complex_double, ldb: lapack_int,
                               x: *mut lapack_complex_double, ldx: lapack_int,
                               rcond: *mut c_double, ferr: *mut c_double, berr: *mut c_double,
                               work: *mut lapack_complex_double, rwork: *mut c_double)
                               -> lapack_int;

    pub fn LAPACKE_spttrf_work(n: lapack_int, d: *mut c_float, e: *mut c_float) -> lapack_int;
    pub fn LAPACKE_dpttrf_work(n: lapack_int, d: *mut c_double, e: *mut c_double) -> lapack_int;
    pub fn LAPACKE_cpttrf_work(n: lapack_int, d: *mut c_float, e: *mut lapack_complex_float)
                               -> lapack_int;
    pub fn LAPACKE_zpttrf_work(n: lapack_int, d: *mut c_double, e: *mut lapack_complex_double)
                               -> lapack_int;

    pub fn LAPACKE_spttrs_work(matrix_layout: c_int, n: lapack_int, nrhs: lapack_int,
                               d: *const c_float, e: *const c_float, b: *mut c_float,
                               ldb: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dpttrs_work(matrix_layout: c_int, n: lapack_int, nrhs: lapack_int,
                               d: *const c_double, e: *const c_double, b: *mut c_double,
                               ldb: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_cpttrs_work(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                               d: *const c_float, e: *const lapack_complex_float,
                               b: *mut lapack_complex_float, ldb: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_zpttrs_work(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                               d: *const c_double, e: *const lapack_complex_double,
                               b: *mut lapack_complex_double, ldb: lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_ssbev_work(matrix_layout: c_int, jobz: c_char, uplo: c_char, n: lapack_int,
                              kd: lapack_int, ab: *mut c_float, ldab: lapack_int, w: *mut c_float,
                              z: *mut c_float, ldz: lapack_int, work: *mut c_float)
                              -> lapack_int;
    pub fn LAPACKE_dsbev_work(matrix_layout: c_int, jobz: c_char, uplo: c_char, n: lapack_int,
                              kd: lapack_int, ab: *mut c_double, ldab: lapack_int,
                              w: *mut c_double, z: *mut c_double, ldz: lapack_int,
                              work: *mut c_double)
                              -> lapack_int;

    pub fn LAPACKE_ssbevd_work(matrix_layout: c_int, jobz: c_char, uplo: c_char, n: lapack_int,
                               kd: lapack_int, ab: *mut c_float, ldab: lapack_int, w: *mut c_float,
                               z: *mut c_float, ldz: lapack_int, work: *mut c_float,
                               lwork: lapack_int, iwork: *mut lapack_int, liwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dsbevd_work(matrix_layout: c_int, jobz: c_char, uplo: c_char, n: lapack_int,
                               kd: lapack_int, ab: *mut c_double, ldab: lapack_int,
                               w: *mut c_double, z: *mut c_double, ldz: lapack_int,
                               work: *mut c_double, lwork: lapack_int, iwork: *mut lapack_int,
                               liwork: lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_ssbevx_work(matrix_layout: c_int, jobz: c_char, range: c_char, uplo: c_char,
                               n: lapack_int, kd: lapack_int, ab: *mut c_float, ldab: lapack_int,
                               q: *mut c_float, ldq: lapack_int, vl: c_float, vu: c_float,
                               il: lapack_int, iu: lapack_int, abstol: c_float, m: *mut lapack_int,
                               w: *mut c_float, z: *mut c_float, ldz: lapack_int,
                               work: *mut c_float, iwork: *mut lapack_int, ifail: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dsbevx_work(matrix_layout: c_int, jobz: c_char, range: c_char, uplo: c_char,
                               n: lapack_int, kd: lapack_int, ab: *mut c_double, ldab: lapack_int,
                               q: *mut c_double, ldq: lapack_int, vl: c_double, vu: c_double,
                               il: lapack_int, iu: lapack_int, abstol: c_double,
                               m: *mut lapack_int, w: *mut c_double, z: *mut c_double,
                               ldz: lapack_int, work: *mut c_double, iwork: *mut lapack_int,
                               ifail: *mut lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_ssbgst_work(matrix_layout: c_int, vect: c_char, uplo: c_char, n: lapack_int,
                               ka: lapack_int, kb: lapack_int, ab: *mut c_float, ldab: lapack_int,
                               bb: *const c_float, ldbb: lapack_int, x: *mut c_float,
                               ldx: lapack_int, work: *mut c_float)
                               -> lapack_int;
    pub fn LAPACKE_dsbgst_work(matrix_layout: c_int, vect: c_char, uplo: c_char, n: lapack_int,
                               ka: lapack_int, kb: lapack_int, ab: *mut c_double, ldab: lapack_int,
                               bb: *const c_double, ldbb: lapack_int, x: *mut c_double,
                               ldx: lapack_int, work: *mut c_double)
                               -> lapack_int;

    pub fn LAPACKE_ssbgv_work(matrix_layout: c_int, jobz: c_char, uplo: c_char, n: lapack_int,
                              ka: lapack_int, kb: lapack_int, ab: *mut c_float, ldab: lapack_int,
                              bb: *mut c_float, ldbb: lapack_int, w: *mut c_float, z: *mut c_float,
                              ldz: lapack_int, work: *mut c_float)
                              -> lapack_int;
    pub fn LAPACKE_dsbgv_work(matrix_layout: c_int, jobz: c_char, uplo: c_char, n: lapack_int,
                              ka: lapack_int, kb: lapack_int, ab: *mut c_double, ldab: lapack_int,
                              bb: *mut c_double, ldbb: lapack_int, w: *mut c_double,
                              z: *mut c_double, ldz: lapack_int, work: *mut c_double)
                              -> lapack_int;

    pub fn LAPACKE_ssbgvd_work(matrix_layout: c_int, jobz: c_char, uplo: c_char, n: lapack_int,
                               ka: lapack_int, kb: lapack_int, ab: *mut c_float, ldab: lapack_int,
                               bb: *mut c_float, ldbb: lapack_int, w: *mut c_float,
                               z: *mut c_float, ldz: lapack_int, work: *mut c_float,
                               lwork: lapack_int, iwork: *mut lapack_int, liwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dsbgvd_work(matrix_layout: c_int, jobz: c_char, uplo: c_char, n: lapack_int,
                               ka: lapack_int, kb: lapack_int, ab: *mut c_double, ldab: lapack_int,
                               bb: *mut c_double, ldbb: lapack_int, w: *mut c_double,
                               z: *mut c_double, ldz: lapack_int, work: *mut c_double,
                               lwork: lapack_int, iwork: *mut lapack_int, liwork: lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_ssbgvx_work(matrix_layout: c_int, jobz: c_char, range: c_char, uplo: c_char,
                               n: lapack_int, ka: lapack_int, kb: lapack_int, ab: *mut c_float,
                               ldab: lapack_int, bb: *mut c_float, ldbb: lapack_int,
                               q: *mut c_float, ldq: lapack_int, vl: c_float, vu: c_float,
                               il: lapack_int, iu: lapack_int, abstol: c_float, m: *mut lapack_int,
                               w: *mut c_float, z: *mut c_float, ldz: lapack_int,
                               work: *mut c_float, iwork: *mut lapack_int, ifail: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dsbgvx_work(matrix_layout: c_int, jobz: c_char, range: c_char, uplo: c_char,
                               n: lapack_int, ka: lapack_int, kb: lapack_int, ab: *mut c_double,
                               ldab: lapack_int, bb: *mut c_double, ldbb: lapack_int,
                               q: *mut c_double, ldq: lapack_int, vl: c_double, vu: c_double,
                               il: lapack_int, iu: lapack_int, abstol: c_double,
                               m: *mut lapack_int, w: *mut c_double, z: *mut c_double,
                               ldz: lapack_int, work: *mut c_double, iwork: *mut lapack_int,
                               ifail: *mut lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_ssbtrd_work(matrix_layout: c_int, vect: c_char, uplo: c_char, n: lapack_int,
                               kd: lapack_int, ab: *mut c_float, ldab: lapack_int, d: *mut c_float,
                               e: *mut c_float, q: *mut c_float, ldq: lapack_int,
                               work: *mut c_float)
                               -> lapack_int;
    pub fn LAPACKE_dsbtrd_work(matrix_layout: c_int, vect: c_char, uplo: c_char, n: lapack_int,
                               kd: lapack_int, ab: *mut c_double, ldab: lapack_int,
                               d: *mut c_double, e: *mut c_double, q: *mut c_double,
                               ldq: lapack_int, work: *mut c_double)
                               -> lapack_int;

    pub fn LAPACKE_ssfrk_work(matrix_layout: c_int, transr: c_char, uplo: c_char, trans: c_char,
                              n: lapack_int, k: lapack_int, alpha: c_float, a: *const c_float,
                              lda: lapack_int, beta: c_float, c: *mut c_float)
                              -> lapack_int;
    pub fn LAPACKE_dsfrk_work(matrix_layout: c_int, transr: c_char, uplo: c_char, trans: c_char,
                              n: lapack_int, k: lapack_int, alpha: c_double, a: *const c_double,
                              lda: lapack_int, beta: c_double, c: *mut c_double)
                              -> lapack_int;

    pub fn LAPACKE_sspcon_work(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                               ap: *const c_float, ipiv: *const lapack_int, anorm: c_float,
                               rcond: *mut c_float, work: *mut c_float, iwork: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dspcon_work(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                               ap: *const c_double, ipiv: *const lapack_int, anorm: c_double,
                               rcond: *mut c_double, work: *mut c_double, iwork: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_cspcon_work(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                               ap: *const lapack_complex_float, ipiv: *const lapack_int,
                               anorm: c_float, rcond: *mut c_float,
                               work: *mut lapack_complex_float)
                               -> lapack_int;
    pub fn LAPACKE_zspcon_work(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                               ap: *const lapack_complex_double, ipiv: *const lapack_int,
                               anorm: c_double, rcond: *mut c_double,
                               work: *mut lapack_complex_double)
                               -> lapack_int;

    pub fn LAPACKE_sspev_work(matrix_layout: c_int, jobz: c_char, uplo: c_char, n: lapack_int,
                              ap: *mut c_float, w: *mut c_float, z: *mut c_float, ldz: lapack_int,
                              work: *mut c_float)
                              -> lapack_int;
    pub fn LAPACKE_dspev_work(matrix_layout: c_int, jobz: c_char, uplo: c_char, n: lapack_int,
                              ap: *mut c_double, w: *mut c_double, z: *mut c_double,
                              ldz: lapack_int, work: *mut c_double)
                              -> lapack_int;

    pub fn LAPACKE_sspevd_work(matrix_layout: c_int, jobz: c_char, uplo: c_char, n: lapack_int,
                               ap: *mut c_float, w: *mut c_float, z: *mut c_float, ldz: lapack_int,
                               work: *mut c_float, lwork: lapack_int, iwork: *mut lapack_int,
                               liwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dspevd_work(matrix_layout: c_int, jobz: c_char, uplo: c_char, n: lapack_int,
                               ap: *mut c_double, w: *mut c_double, z: *mut c_double,
                               ldz: lapack_int, work: *mut c_double, lwork: lapack_int,
                               iwork: *mut lapack_int, liwork: lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_sspevx_work(matrix_layout: c_int, jobz: c_char, range: c_char, uplo: c_char,
                               n: lapack_int, ap: *mut c_float, vl: c_float, vu: c_float,
                               il: lapack_int, iu: lapack_int, abstol: c_float, m: *mut lapack_int,
                               w: *mut c_float, z: *mut c_float, ldz: lapack_int,
                               work: *mut c_float, iwork: *mut lapack_int, ifail: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dspevx_work(matrix_layout: c_int, jobz: c_char, range: c_char, uplo: c_char,
                               n: lapack_int, ap: *mut c_double, vl: c_double, vu: c_double,
                               il: lapack_int, iu: lapack_int, abstol: c_double,
                               m: *mut lapack_int, w: *mut c_double, z: *mut c_double,
                               ldz: lapack_int, work: *mut c_double, iwork: *mut lapack_int,
                               ifail: *mut lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_sspgst_work(matrix_layout: c_int, itype: lapack_int, uplo: c_char,
                               n: lapack_int, ap: *mut c_float, bp: *const c_float)
                               -> lapack_int;
    pub fn LAPACKE_dspgst_work(matrix_layout: c_int, itype: lapack_int, uplo: c_char,
                               n: lapack_int, ap: *mut c_double, bp: *const c_double)
                               -> lapack_int;

    pub fn LAPACKE_sspgv_work(matrix_layout: c_int, itype: lapack_int, jobz: c_char, uplo: c_char,
                              n: lapack_int, ap: *mut c_float, bp: *mut c_float, w: *mut c_float,
                              z: *mut c_float, ldz: lapack_int, work: *mut c_float)
                              -> lapack_int;
    pub fn LAPACKE_dspgv_work(matrix_layout: c_int, itype: lapack_int, jobz: c_char, uplo: c_char,
                              n: lapack_int, ap: *mut c_double, bp: *mut c_double,
                              w: *mut c_double, z: *mut c_double, ldz: lapack_int,
                              work: *mut c_double)
                              -> lapack_int;

    pub fn LAPACKE_sspgvd_work(matrix_layout: c_int, itype: lapack_int, jobz: c_char, uplo: c_char,
                               n: lapack_int, ap: *mut c_float, bp: *mut c_float, w: *mut c_float,
                               z: *mut c_float, ldz: lapack_int, work: *mut c_float,
                               lwork: lapack_int, iwork: *mut lapack_int, liwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dspgvd_work(matrix_layout: c_int, itype: lapack_int, jobz: c_char, uplo: c_char,
                               n: lapack_int, ap: *mut c_double, bp: *mut c_double,
                               w: *mut c_double, z: *mut c_double, ldz: lapack_int,
                               work: *mut c_double, lwork: lapack_int, iwork: *mut lapack_int,
                               liwork: lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_sspgvx_work(matrix_layout: c_int, itype: lapack_int, jobz: c_char,
                               range: c_char, uplo: c_char, n: lapack_int, ap: *mut c_float,
                               bp: *mut c_float, vl: c_float, vu: c_float, il: lapack_int,
                               iu: lapack_int, abstol: c_float, m: *mut lapack_int,
                               w: *mut c_float, z: *mut c_float, ldz: lapack_int,
                               work: *mut c_float, iwork: *mut lapack_int, ifail: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dspgvx_work(matrix_layout: c_int, itype: lapack_int, jobz: c_char,
                               range: c_char, uplo: c_char, n: lapack_int, ap: *mut c_double,
                               bp: *mut c_double, vl: c_double, vu: c_double, il: lapack_int,
                               iu: lapack_int, abstol: c_double, m: *mut lapack_int,
                               w: *mut c_double, z: *mut c_double, ldz: lapack_int,
                               work: *mut c_double, iwork: *mut lapack_int, ifail: *mut lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_ssprfs_work(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                               ap: *const c_float, afp: *const c_float, ipiv: *const lapack_int,
                               b: *const c_float, ldb: lapack_int, x: *mut c_float,
                               ldx: lapack_int, ferr: *mut c_float, berr: *mut c_float,
                               work: *mut c_float, iwork: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dsprfs_work(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                               ap: *const c_double, afp: *const c_double, ipiv: *const lapack_int,
                               b: *const c_double, ldb: lapack_int, x: *mut c_double,
                               ldx: lapack_int, ferr: *mut c_double, berr: *mut c_double,
                               work: *mut c_double, iwork: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_csprfs_work(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                               ap: *const lapack_complex_float, afp: *const lapack_complex_float,
                               ipiv: *const lapack_int, b: *const lapack_complex_float,
                               ldb: lapack_int, x: *mut lapack_complex_float, ldx: lapack_int,
                               ferr: *mut c_float, berr: *mut c_float,
                               work: *mut lapack_complex_float, rwork: *mut c_float)
                               -> lapack_int;
    pub fn LAPACKE_zsprfs_work(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                               ap: *const lapack_complex_double, afp: *const lapack_complex_double,
                               ipiv: *const lapack_int, b: *const lapack_complex_double,
                               ldb: lapack_int, x: *mut lapack_complex_double, ldx: lapack_int,
                               ferr: *mut c_double, berr: *mut c_double,
                               work: *mut lapack_complex_double, rwork: *mut c_double)
                               -> lapack_int;

    pub fn LAPACKE_sspsv_work(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                              ap: *mut c_float, ipiv: *mut lapack_int, b: *mut c_float,
                              ldb: lapack_int)
                              -> lapack_int;
    pub fn LAPACKE_dspsv_work(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                              ap: *mut c_double, ipiv: *mut lapack_int, b: *mut c_double,
                              ldb: lapack_int)
                              -> lapack_int;
    pub fn LAPACKE_cspsv_work(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                              ap: *mut lapack_complex_float, ipiv: *mut lapack_int,
                              b: *mut lapack_complex_float, ldb: lapack_int)
                              -> lapack_int;
    pub fn LAPACKE_zspsv_work(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                              ap: *mut lapack_complex_double, ipiv: *mut lapack_int,
                              b: *mut lapack_complex_double, ldb: lapack_int)
                              -> lapack_int;

    pub fn LAPACKE_sspsvx_work(matrix_layout: c_int, fact: c_char, uplo: c_char, n: lapack_int,
                               nrhs: lapack_int, ap: *const c_float, afp: *mut c_float,
                               ipiv: *mut lapack_int, b: *const c_float, ldb: lapack_int,
                               x: *mut c_float, ldx: lapack_int, rcond: *mut c_float,
                               ferr: *mut c_float, berr: *mut c_float, work: *mut c_float,
                               iwork: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dspsvx_work(matrix_layout: c_int, fact: c_char, uplo: c_char, n: lapack_int,
                               nrhs: lapack_int, ap: *const c_double, afp: *mut c_double,
                               ipiv: *mut lapack_int, b: *const c_double, ldb: lapack_int,
                               x: *mut c_double, ldx: lapack_int, rcond: *mut c_double,
                               ferr: *mut c_double, berr: *mut c_double, work: *mut c_double,
                               iwork: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_cspsvx_work(matrix_layout: c_int, fact: c_char, uplo: c_char, n: lapack_int,
                               nrhs: lapack_int, ap: *const lapack_complex_float,
                               afp: *mut lapack_complex_float, ipiv: *mut lapack_int,
                               b: *const lapack_complex_float, ldb: lapack_int,
                               x: *mut lapack_complex_float, ldx: lapack_int, rcond: *mut c_float,
                               ferr: *mut c_float, berr: *mut c_float,
                               work: *mut lapack_complex_float, rwork: *mut c_float)
                               -> lapack_int;
    pub fn LAPACKE_zspsvx_work(matrix_layout: c_int, fact: c_char, uplo: c_char, n: lapack_int,
                               nrhs: lapack_int, ap: *const lapack_complex_double,
                               afp: *mut lapack_complex_double, ipiv: *mut lapack_int,
                               b: *const lapack_complex_double, ldb: lapack_int,
                               x: *mut lapack_complex_double, ldx: lapack_int,
                               rcond: *mut c_double, ferr: *mut c_double, berr: *mut c_double,
                               work: *mut lapack_complex_double, rwork: *mut c_double)
                               -> lapack_int;

    pub fn LAPACKE_ssptrd_work(matrix_layout: c_int, uplo: c_char, n: lapack_int, ap: *mut c_float,
                               d: *mut c_float, e: *mut c_float, tau: *mut c_float)
                               -> lapack_int;
    pub fn LAPACKE_dsptrd_work(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                               ap: *mut c_double, d: *mut c_double, e: *mut c_double,
                               tau: *mut c_double)
                               -> lapack_int;

    pub fn LAPACKE_ssptrf_work(matrix_layout: c_int, uplo: c_char, n: lapack_int, ap: *mut c_float,
                               ipiv: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dsptrf_work(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                               ap: *mut c_double, ipiv: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_csptrf_work(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                               ap: *mut lapack_complex_float, ipiv: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_zsptrf_work(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                               ap: *mut lapack_complex_double, ipiv: *mut lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_ssptri_work(matrix_layout: c_int, uplo: c_char, n: lapack_int, ap: *mut c_float,
                               ipiv: *const lapack_int, work: *mut c_float)
                               -> lapack_int;
    pub fn LAPACKE_dsptri_work(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                               ap: *mut c_double, ipiv: *const lapack_int, work: *mut c_double)
                               -> lapack_int;
    pub fn LAPACKE_csptri_work(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                               ap: *mut lapack_complex_float, ipiv: *const lapack_int,
                               work: *mut lapack_complex_float)
                               -> lapack_int;
    pub fn LAPACKE_zsptri_work(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                               ap: *mut lapack_complex_double, ipiv: *const lapack_int,
                               work: *mut lapack_complex_double)
                               -> lapack_int;

    pub fn LAPACKE_ssptrs_work(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                               ap: *const c_float, ipiv: *const lapack_int, b: *mut c_float,
                               ldb: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dsptrs_work(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                               ap: *const c_double, ipiv: *const lapack_int, b: *mut c_double,
                               ldb: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_csptrs_work(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                               ap: *const lapack_complex_float, ipiv: *const lapack_int,
                               b: *mut lapack_complex_float, ldb: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_zsptrs_work(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                               ap: *const lapack_complex_double, ipiv: *const lapack_int,
                               b: *mut lapack_complex_double, ldb: lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_sstebz_work(range: c_char, order: c_char, n: lapack_int, vl: c_float,
                               vu: c_float, il: lapack_int, iu: lapack_int, abstol: c_float,
                               d: *const c_float, e: *const c_float, m: *mut lapack_int,
                               nsplit: *mut lapack_int, w: *mut c_float, iblock: *mut lapack_int,
                               isplit: *mut lapack_int, work: *mut c_float, iwork: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dstebz_work(range: c_char, order: c_char, n: lapack_int, vl: c_double,
                               vu: c_double, il: lapack_int, iu: lapack_int, abstol: c_double,
                               d: *const c_double, e: *const c_double, m: *mut lapack_int,
                               nsplit: *mut lapack_int, w: *mut c_double, iblock: *mut lapack_int,
                               isplit: *mut lapack_int, work: *mut c_double,
                               iwork: *mut lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_sstedc_work(matrix_layout: c_int, compz: c_char, n: lapack_int, d: *mut c_float,
                               e: *mut c_float, z: *mut c_float, ldz: lapack_int,
                               work: *mut c_float, lwork: lapack_int, iwork: *mut lapack_int,
                               liwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dstedc_work(matrix_layout: c_int, compz: c_char, n: lapack_int,
                               d: *mut c_double, e: *mut c_double, z: *mut c_double,
                               ldz: lapack_int, work: *mut c_double, lwork: lapack_int,
                               iwork: *mut lapack_int, liwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_cstedc_work(matrix_layout: c_int, compz: c_char, n: lapack_int, d: *mut c_float,
                               e: *mut c_float, z: *mut lapack_complex_float, ldz: lapack_int,
                               work: *mut lapack_complex_float, lwork: lapack_int,
                               rwork: *mut c_float, lrwork: lapack_int, iwork: *mut lapack_int,
                               liwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_zstedc_work(matrix_layout: c_int, compz: c_char, n: lapack_int,
                               d: *mut c_double, e: *mut c_double, z: *mut lapack_complex_double,
                               ldz: lapack_int, work: *mut lapack_complex_double,
                               lwork: lapack_int, rwork: *mut c_double, lrwork: lapack_int,
                               iwork: *mut lapack_int, liwork: lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_sstegr_work(matrix_layout: c_int, jobz: c_char, range: c_char, n: lapack_int,
                               d: *mut c_float, e: *mut c_float, vl: c_float, vu: c_float,
                               il: lapack_int, iu: lapack_int, abstol: c_float, m: *mut lapack_int,
                               w: *mut c_float, z: *mut c_float, ldz: lapack_int,
                               isuppz: *mut lapack_int, work: *mut c_float, lwork: lapack_int,
                               iwork: *mut lapack_int, liwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dstegr_work(matrix_layout: c_int, jobz: c_char, range: c_char, n: lapack_int,
                               d: *mut c_double, e: *mut c_double, vl: c_double, vu: c_double,
                               il: lapack_int, iu: lapack_int, abstol: c_double,
                               m: *mut lapack_int, w: *mut c_double, z: *mut c_double,
                               ldz: lapack_int, isuppz: *mut lapack_int, work: *mut c_double,
                               lwork: lapack_int, iwork: *mut lapack_int, liwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_cstegr_work(matrix_layout: c_int, jobz: c_char, range: c_char, n: lapack_int,
                               d: *mut c_float, e: *mut c_float, vl: c_float, vu: c_float,
                               il: lapack_int, iu: lapack_int, abstol: c_float, m: *mut lapack_int,
                               w: *mut c_float, z: *mut lapack_complex_float, ldz: lapack_int,
                               isuppz: *mut lapack_int, work: *mut c_float, lwork: lapack_int,
                               iwork: *mut lapack_int, liwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_zstegr_work(matrix_layout: c_int, jobz: c_char, range: c_char, n: lapack_int,
                               d: *mut c_double, e: *mut c_double, vl: c_double, vu: c_double,
                               il: lapack_int, iu: lapack_int, abstol: c_double,
                               m: *mut lapack_int, w: *mut c_double, z: *mut lapack_complex_double,
                               ldz: lapack_int, isuppz: *mut lapack_int, work: *mut c_double,
                               lwork: lapack_int, iwork: *mut lapack_int, liwork: lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_sstein_work(matrix_layout: c_int, n: lapack_int, d: *const c_float,
                               e: *const c_float, m: lapack_int, w: *const c_float,
                               iblock: *const lapack_int, isplit: *const lapack_int,
                               z: *mut c_float, ldz: lapack_int, work: *mut c_float,
                               iwork: *mut lapack_int, ifailv: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dstein_work(matrix_layout: c_int, n: lapack_int, d: *const c_double,
                               e: *const c_double, m: lapack_int, w: *const c_double,
                               iblock: *const lapack_int, isplit: *const lapack_int,
                               z: *mut c_double, ldz: lapack_int, work: *mut c_double,
                               iwork: *mut lapack_int, ifailv: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_cstein_work(matrix_layout: c_int, n: lapack_int, d: *const c_float,
                               e: *const c_float, m: lapack_int, w: *const c_float,
                               iblock: *const lapack_int, isplit: *const lapack_int,
                               z: *mut lapack_complex_float, ldz: lapack_int, work: *mut c_float,
                               iwork: *mut lapack_int, ifailv: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_zstein_work(matrix_layout: c_int, n: lapack_int, d: *const c_double,
                               e: *const c_double, m: lapack_int, w: *const c_double,
                               iblock: *const lapack_int, isplit: *const lapack_int,
                               z: *mut lapack_complex_double, ldz: lapack_int, work: *mut c_double,
                               iwork: *mut lapack_int, ifailv: *mut lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_sstemr_work(matrix_layout: c_int, jobz: c_char, range: c_char, n: lapack_int,
                               d: *mut c_float, e: *mut c_float, vl: c_float, vu: c_float,
                               il: lapack_int, iu: lapack_int, m: *mut lapack_int, w: *mut c_float,
                               z: *mut c_float, ldz: lapack_int, nzc: lapack_int,
                               isuppz: *mut lapack_int, tryrac: *mut lapack_logical,
                               work: *mut c_float, lwork: lapack_int, iwork: *mut lapack_int,
                               liwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dstemr_work(matrix_layout: c_int, jobz: c_char, range: c_char, n: lapack_int,
                               d: *mut c_double, e: *mut c_double, vl: c_double, vu: c_double,
                               il: lapack_int, iu: lapack_int, m: *mut lapack_int,
                               w: *mut c_double, z: *mut c_double, ldz: lapack_int,
                               nzc: lapack_int, isuppz: *mut lapack_int,
                               tryrac: *mut lapack_logical, work: *mut c_double, lwork: lapack_int,
                               iwork: *mut lapack_int, liwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_cstemr_work(matrix_layout: c_int, jobz: c_char, range: c_char, n: lapack_int,
                               d: *mut c_float, e: *mut c_float, vl: c_float, vu: c_float,
                               il: lapack_int, iu: lapack_int, m: *mut lapack_int, w: *mut c_float,
                               z: *mut lapack_complex_float, ldz: lapack_int, nzc: lapack_int,
                               isuppz: *mut lapack_int, tryrac: *mut lapack_logical,
                               work: *mut c_float, lwork: lapack_int, iwork: *mut lapack_int,
                               liwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_zstemr_work(matrix_layout: c_int, jobz: c_char, range: c_char, n: lapack_int,
                               d: *mut c_double, e: *mut c_double, vl: c_double, vu: c_double,
                               il: lapack_int, iu: lapack_int, m: *mut lapack_int,
                               w: *mut c_double, z: *mut lapack_complex_double, ldz: lapack_int,
                               nzc: lapack_int, isuppz: *mut lapack_int,
                               tryrac: *mut lapack_logical, work: *mut c_double, lwork: lapack_int,
                               iwork: *mut lapack_int, liwork: lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_ssteqr_work(matrix_layout: c_int, compz: c_char, n: lapack_int, d: *mut c_float,
                               e: *mut c_float, z: *mut c_float, ldz: lapack_int,
                               work: *mut c_float)
                               -> lapack_int;
    pub fn LAPACKE_dsteqr_work(matrix_layout: c_int, compz: c_char, n: lapack_int,
                               d: *mut c_double, e: *mut c_double, z: *mut c_double,
                               ldz: lapack_int, work: *mut c_double)
                               -> lapack_int;
    pub fn LAPACKE_csteqr_work(matrix_layout: c_int, compz: c_char, n: lapack_int, d: *mut c_float,
                               e: *mut c_float, z: *mut lapack_complex_float, ldz: lapack_int,
                               work: *mut c_float)
                               -> lapack_int;
    pub fn LAPACKE_zsteqr_work(matrix_layout: c_int, compz: c_char, n: lapack_int,
                               d: *mut c_double, e: *mut c_double, z: *mut lapack_complex_double,
                               ldz: lapack_int, work: *mut c_double)
                               -> lapack_int;

    pub fn LAPACKE_ssterf_work(n: lapack_int, d: *mut c_float, e: *mut c_float) -> lapack_int;
    pub fn LAPACKE_dsterf_work(n: lapack_int, d: *mut c_double, e: *mut c_double) -> lapack_int;

    pub fn LAPACKE_sstev_work(matrix_layout: c_int, jobz: c_char, n: lapack_int, d: *mut c_float,
                              e: *mut c_float, z: *mut c_float, ldz: lapack_int,
                              work: *mut c_float)
                              -> lapack_int;
    pub fn LAPACKE_dstev_work(matrix_layout: c_int, jobz: c_char, n: lapack_int, d: *mut c_double,
                              e: *mut c_double, z: *mut c_double, ldz: lapack_int,
                              work: *mut c_double)
                              -> lapack_int;

    pub fn LAPACKE_sstevd_work(matrix_layout: c_int, jobz: c_char, n: lapack_int, d: *mut c_float,
                               e: *mut c_float, z: *mut c_float, ldz: lapack_int,
                               work: *mut c_float, lwork: lapack_int, iwork: *mut lapack_int,
                               liwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dstevd_work(matrix_layout: c_int, jobz: c_char, n: lapack_int, d: *mut c_double,
                               e: *mut c_double, z: *mut c_double, ldz: lapack_int,
                               work: *mut c_double, lwork: lapack_int, iwork: *mut lapack_int,
                               liwork: lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_sstevr_work(matrix_layout: c_int, jobz: c_char, range: c_char, n: lapack_int,
                               d: *mut c_float, e: *mut c_float, vl: c_float, vu: c_float,
                               il: lapack_int, iu: lapack_int, abstol: c_float, m: *mut lapack_int,
                               w: *mut c_float, z: *mut c_float, ldz: lapack_int,
                               isuppz: *mut lapack_int, work: *mut c_float, lwork: lapack_int,
                               iwork: *mut lapack_int, liwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dstevr_work(matrix_layout: c_int, jobz: c_char, range: c_char, n: lapack_int,
                               d: *mut c_double, e: *mut c_double, vl: c_double, vu: c_double,
                               il: lapack_int, iu: lapack_int, abstol: c_double,
                               m: *mut lapack_int, w: *mut c_double, z: *mut c_double,
                               ldz: lapack_int, isuppz: *mut lapack_int, work: *mut c_double,
                               lwork: lapack_int, iwork: *mut lapack_int, liwork: lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_sstevx_work(matrix_layout: c_int, jobz: c_char, range: c_char, n: lapack_int,
                               d: *mut c_float, e: *mut c_float, vl: c_float, vu: c_float,
                               il: lapack_int, iu: lapack_int, abstol: c_float, m: *mut lapack_int,
                               w: *mut c_float, z: *mut c_float, ldz: lapack_int,
                               work: *mut c_float, iwork: *mut lapack_int, ifail: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dstevx_work(matrix_layout: c_int, jobz: c_char, range: c_char, n: lapack_int,
                               d: *mut c_double, e: *mut c_double, vl: c_double, vu: c_double,
                               il: lapack_int, iu: lapack_int, abstol: c_double,
                               m: *mut lapack_int, w: *mut c_double, z: *mut c_double,
                               ldz: lapack_int, work: *mut c_double, iwork: *mut lapack_int,
                               ifail: *mut lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_ssycon_work(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                               a: *const c_float, lda: lapack_int, ipiv: *const lapack_int,
                               anorm: c_float, rcond: *mut c_float, work: *mut c_float,
                               iwork: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dsycon_work(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                               a: *const c_double, lda: lapack_int, ipiv: *const lapack_int,
                               anorm: c_double, rcond: *mut c_double, work: *mut c_double,
                               iwork: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_csycon_work(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                               a: *const lapack_complex_float, lda: lapack_int,
                               ipiv: *const lapack_int, anorm: c_float, rcond: *mut c_float,
                               work: *mut lapack_complex_float)
                               -> lapack_int;
    pub fn LAPACKE_zsycon_work(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                               a: *const lapack_complex_double, lda: lapack_int,
                               ipiv: *const lapack_int, anorm: c_double, rcond: *mut c_double,
                               work: *mut lapack_complex_double)
                               -> lapack_int;

    pub fn LAPACKE_ssyequb_work(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                                a: *const c_float, lda: lapack_int, s: *mut c_float,
                                scond: *mut c_float, amax: *mut c_float, work: *mut c_float)
                                -> lapack_int;
    pub fn LAPACKE_dsyequb_work(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                                a: *const c_double, lda: lapack_int, s: *mut c_double,
                                scond: *mut c_double, amax: *mut c_double, work: *mut c_double)
                                -> lapack_int;
    pub fn LAPACKE_csyequb_work(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                                a: *const lapack_complex_float, lda: lapack_int, s: *mut c_float,
                                scond: *mut c_float, amax: *mut c_float,
                                work: *mut lapack_complex_float)
                                -> lapack_int;
    pub fn LAPACKE_zsyequb_work(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                                a: *const lapack_complex_double, lda: lapack_int, s: *mut c_double,
                                scond: *mut c_double, amax: *mut c_double,
                                work: *mut lapack_complex_double)
                                -> lapack_int;

    pub fn LAPACKE_ssyev_work(matrix_layout: c_int, jobz: c_char, uplo: c_char, n: lapack_int,
                              a: *mut c_float, lda: lapack_int, w: *mut c_float,
                              work: *mut c_float, lwork: lapack_int)
                              -> lapack_int;
    pub fn LAPACKE_dsyev_work(matrix_layout: c_int, jobz: c_char, uplo: c_char, n: lapack_int,
                              a: *mut c_double, lda: lapack_int, w: *mut c_double,
                              work: *mut c_double, lwork: lapack_int)
                              -> lapack_int;

    pub fn LAPACKE_ssyevd_work(matrix_layout: c_int, jobz: c_char, uplo: c_char, n: lapack_int,
                               a: *mut c_float, lda: lapack_int, w: *mut c_float,
                               work: *mut c_float, lwork: lapack_int, iwork: *mut lapack_int,
                               liwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dsyevd_work(matrix_layout: c_int, jobz: c_char, uplo: c_char, n: lapack_int,
                               a: *mut c_double, lda: lapack_int, w: *mut c_double,
                               work: *mut c_double, lwork: lapack_int, iwork: *mut lapack_int,
                               liwork: lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_ssyevr_work(matrix_layout: c_int, jobz: c_char, range: c_char, uplo: c_char,
                               n: lapack_int, a: *mut c_float, lda: lapack_int, vl: c_float,
                               vu: c_float, il: lapack_int, iu: lapack_int, abstol: c_float,
                               m: *mut lapack_int, w: *mut c_float, z: *mut c_float,
                               ldz: lapack_int, isuppz: *mut lapack_int, work: *mut c_float,
                               lwork: lapack_int, iwork: *mut lapack_int, liwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dsyevr_work(matrix_layout: c_int, jobz: c_char, range: c_char, uplo: c_char,
                               n: lapack_int, a: *mut c_double, lda: lapack_int, vl: c_double,
                               vu: c_double, il: lapack_int, iu: lapack_int, abstol: c_double,
                               m: *mut lapack_int, w: *mut c_double, z: *mut c_double,
                               ldz: lapack_int, isuppz: *mut lapack_int, work: *mut c_double,
                               lwork: lapack_int, iwork: *mut lapack_int, liwork: lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_ssyevx_work(matrix_layout: c_int, jobz: c_char, range: c_char, uplo: c_char,
                               n: lapack_int, a: *mut c_float, lda: lapack_int, vl: c_float,
                               vu: c_float, il: lapack_int, iu: lapack_int, abstol: c_float,
                               m: *mut lapack_int, w: *mut c_float, z: *mut c_float,
                               ldz: lapack_int, work: *mut c_float, lwork: lapack_int,
                               iwork: *mut lapack_int, ifail: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dsyevx_work(matrix_layout: c_int, jobz: c_char, range: c_char, uplo: c_char,
                               n: lapack_int, a: *mut c_double, lda: lapack_int, vl: c_double,
                               vu: c_double, il: lapack_int, iu: lapack_int, abstol: c_double,
                               m: *mut lapack_int, w: *mut c_double, z: *mut c_double,
                               ldz: lapack_int, work: *mut c_double, lwork: lapack_int,
                               iwork: *mut lapack_int, ifail: *mut lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_ssygst_work(matrix_layout: c_int, itype: lapack_int, uplo: c_char,
                               n: lapack_int, a: *mut c_float, lda: lapack_int, b: *const c_float,
                               ldb: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dsygst_work(matrix_layout: c_int, itype: lapack_int, uplo: c_char,
                               n: lapack_int, a: *mut c_double, lda: lapack_int,
                               b: *const c_double, ldb: lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_ssygv_work(matrix_layout: c_int, itype: lapack_int, jobz: c_char, uplo: c_char,
                              n: lapack_int, a: *mut c_float, lda: lapack_int, b: *mut c_float,
                              ldb: lapack_int, w: *mut c_float, work: *mut c_float,
                              lwork: lapack_int)
                              -> lapack_int;
    pub fn LAPACKE_dsygv_work(matrix_layout: c_int, itype: lapack_int, jobz: c_char, uplo: c_char,
                              n: lapack_int, a: *mut c_double, lda: lapack_int, b: *mut c_double,
                              ldb: lapack_int, w: *mut c_double, work: *mut c_double,
                              lwork: lapack_int)
                              -> lapack_int;

    pub fn LAPACKE_ssygvd_work(matrix_layout: c_int, itype: lapack_int, jobz: c_char, uplo: c_char,
                               n: lapack_int, a: *mut c_float, lda: lapack_int, b: *mut c_float,
                               ldb: lapack_int, w: *mut c_float, work: *mut c_float,
                               lwork: lapack_int, iwork: *mut lapack_int, liwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dsygvd_work(matrix_layout: c_int, itype: lapack_int, jobz: c_char, uplo: c_char,
                               n: lapack_int, a: *mut c_double, lda: lapack_int, b: *mut c_double,
                               ldb: lapack_int, w: *mut c_double, work: *mut c_double,
                               lwork: lapack_int, iwork: *mut lapack_int, liwork: lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_ssygvx_work(matrix_layout: c_int, itype: lapack_int, jobz: c_char,
                               range: c_char, uplo: c_char, n: lapack_int, a: *mut c_float,
                               lda: lapack_int, b: *mut c_float, ldb: lapack_int, vl: c_float,
                               vu: c_float, il: lapack_int, iu: lapack_int, abstol: c_float,
                               m: *mut lapack_int, w: *mut c_float, z: *mut c_float,
                               ldz: lapack_int, work: *mut c_float, lwork: lapack_int,
                               iwork: *mut lapack_int, ifail: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dsygvx_work(matrix_layout: c_int, itype: lapack_int, jobz: c_char,
                               range: c_char, uplo: c_char, n: lapack_int, a: *mut c_double,
                               lda: lapack_int, b: *mut c_double, ldb: lapack_int, vl: c_double,
                               vu: c_double, il: lapack_int, iu: lapack_int, abstol: c_double,
                               m: *mut lapack_int, w: *mut c_double, z: *mut c_double,
                               ldz: lapack_int, work: *mut c_double, lwork: lapack_int,
                               iwork: *mut lapack_int, ifail: *mut lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_ssyrfs_work(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                               a: *const c_float, lda: lapack_int, af: *const c_float,
                               ldaf: lapack_int, ipiv: *const lapack_int, b: *const c_float,
                               ldb: lapack_int, x: *mut c_float, ldx: lapack_int,
                               ferr: *mut c_float, berr: *mut c_float, work: *mut c_float,
                               iwork: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dsyrfs_work(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                               a: *const c_double, lda: lapack_int, af: *const c_double,
                               ldaf: lapack_int, ipiv: *const lapack_int, b: *const c_double,
                               ldb: lapack_int, x: *mut c_double, ldx: lapack_int,
                               ferr: *mut c_double, berr: *mut c_double, work: *mut c_double,
                               iwork: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_csyrfs_work(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                               a: *const lapack_complex_float, lda: lapack_int,
                               af: *const lapack_complex_float, ldaf: lapack_int,
                               ipiv: *const lapack_int, b: *const lapack_complex_float,
                               ldb: lapack_int, x: *mut lapack_complex_float, ldx: lapack_int,
                               ferr: *mut c_float, berr: *mut c_float,
                               work: *mut lapack_complex_float, rwork: *mut c_float)
                               -> lapack_int;
    pub fn LAPACKE_zsyrfs_work(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                               a: *const lapack_complex_double, lda: lapack_int,
                               af: *const lapack_complex_double, ldaf: lapack_int,
                               ipiv: *const lapack_int, b: *const lapack_complex_double,
                               ldb: lapack_int, x: *mut lapack_complex_double, ldx: lapack_int,
                               ferr: *mut c_double, berr: *mut c_double,
                               work: *mut lapack_complex_double, rwork: *mut c_double)
                               -> lapack_int;

    pub fn LAPACKE_ssyrfsx_work(matrix_layout: c_int, uplo: c_char, equed: c_char, n: lapack_int,
                                nrhs: lapack_int, a: *const c_float, lda: lapack_int,
                                af: *const c_float, ldaf: lapack_int, ipiv: *const lapack_int,
                                s: *const c_float, b: *const c_float, ldb: lapack_int,
                                x: *mut c_float, ldx: lapack_int, rcond: *mut c_float,
                                berr: *mut c_float, n_err_bnds: lapack_int,
                                err_bnds_norm: *mut c_float, err_bnds_comp: *mut c_float,
                                nparams: lapack_int, params: *mut c_float, work: *mut c_float,
                                iwork: *mut lapack_int)
                                -> lapack_int;
    pub fn LAPACKE_dsyrfsx_work(matrix_layout: c_int, uplo: c_char, equed: c_char, n: lapack_int,
                                nrhs: lapack_int, a: *const c_double, lda: lapack_int,
                                af: *const c_double, ldaf: lapack_int, ipiv: *const lapack_int,
                                s: *const c_double, b: *const c_double, ldb: lapack_int,
                                x: *mut c_double, ldx: lapack_int, rcond: *mut c_double,
                                berr: *mut c_double, n_err_bnds: lapack_int,
                                err_bnds_norm: *mut c_double, err_bnds_comp: *mut c_double,
                                nparams: lapack_int, params: *mut c_double, work: *mut c_double,
                                iwork: *mut lapack_int)
                                -> lapack_int;
    pub fn LAPACKE_csyrfsx_work(matrix_layout: c_int, uplo: c_char, equed: c_char, n: lapack_int,
                                nrhs: lapack_int, a: *const lapack_complex_float, lda: lapack_int,
                                af: *const lapack_complex_float, ldaf: lapack_int,
                                ipiv: *const lapack_int, s: *const c_float,
                                b: *const lapack_complex_float, ldb: lapack_int,
                                x: *mut lapack_complex_float, ldx: lapack_int, rcond: *mut c_float,
                                berr: *mut c_float, n_err_bnds: lapack_int,
                                err_bnds_norm: *mut c_float, err_bnds_comp: *mut c_float,
                                nparams: lapack_int, params: *mut c_float,
                                work: *mut lapack_complex_float, rwork: *mut c_float)
                                -> lapack_int;
    pub fn LAPACKE_zsyrfsx_work(matrix_layout: c_int, uplo: c_char, equed: c_char, n: lapack_int,
                                nrhs: lapack_int, a: *const lapack_complex_double, lda: lapack_int,
                                af: *const lapack_complex_double, ldaf: lapack_int,
                                ipiv: *const lapack_int, s: *const c_double,
                                b: *const lapack_complex_double, ldb: lapack_int,
                                x: *mut lapack_complex_double, ldx: lapack_int,
                                rcond: *mut c_double, berr: *mut c_double, n_err_bnds: lapack_int,
                                err_bnds_norm: *mut c_double, err_bnds_comp: *mut c_double,
                                nparams: lapack_int, params: *mut c_double,
                                work: *mut lapack_complex_double, rwork: *mut c_double)
                                -> lapack_int;

    pub fn LAPACKE_ssysv_work(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                              a: *mut c_float, lda: lapack_int, ipiv: *mut lapack_int,
                              b: *mut c_float, ldb: lapack_int, work: *mut c_float,
                              lwork: lapack_int)
                              -> lapack_int;
    pub fn LAPACKE_dsysv_work(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                              a: *mut c_double, lda: lapack_int, ipiv: *mut lapack_int,
                              b: *mut c_double, ldb: lapack_int, work: *mut c_double,
                              lwork: lapack_int)
                              -> lapack_int;
    pub fn LAPACKE_csysv_work(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                              a: *mut lapack_complex_float, lda: lapack_int, ipiv: *mut lapack_int,
                              b: *mut lapack_complex_float, ldb: lapack_int,
                              work: *mut lapack_complex_float, lwork: lapack_int)
                              -> lapack_int;
    pub fn LAPACKE_zsysv_work(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                              a: *mut lapack_complex_double, lda: lapack_int,
                              ipiv: *mut lapack_int, b: *mut lapack_complex_double,
                              ldb: lapack_int, work: *mut lapack_complex_double, lwork: lapack_int)
                              -> lapack_int;

    pub fn LAPACKE_ssysvx_work(matrix_layout: c_int, fact: c_char, uplo: c_char, n: lapack_int,
                               nrhs: lapack_int, a: *const c_float, lda: lapack_int,
                               af: *mut c_float, ldaf: lapack_int, ipiv: *mut lapack_int,
                               b: *const c_float, ldb: lapack_int, x: *mut c_float,
                               ldx: lapack_int, rcond: *mut c_float, ferr: *mut c_float,
                               berr: *mut c_float, work: *mut c_float, lwork: lapack_int,
                               iwork: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dsysvx_work(matrix_layout: c_int, fact: c_char, uplo: c_char, n: lapack_int,
                               nrhs: lapack_int, a: *const c_double, lda: lapack_int,
                               af: *mut c_double, ldaf: lapack_int, ipiv: *mut lapack_int,
                               b: *const c_double, ldb: lapack_int, x: *mut c_double,
                               ldx: lapack_int, rcond: *mut c_double, ferr: *mut c_double,
                               berr: *mut c_double, work: *mut c_double, lwork: lapack_int,
                               iwork: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_csysvx_work(matrix_layout: c_int, fact: c_char, uplo: c_char, n: lapack_int,
                               nrhs: lapack_int, a: *const lapack_complex_float, lda: lapack_int,
                               af: *mut lapack_complex_float, ldaf: lapack_int,
                               ipiv: *mut lapack_int, b: *const lapack_complex_float,
                               ldb: lapack_int, x: *mut lapack_complex_float, ldx: lapack_int,
                               rcond: *mut c_float, ferr: *mut c_float, berr: *mut c_float,
                               work: *mut lapack_complex_float, lwork: lapack_int,
                               rwork: *mut c_float)
                               -> lapack_int;
    pub fn LAPACKE_zsysvx_work(matrix_layout: c_int, fact: c_char, uplo: c_char, n: lapack_int,
                               nrhs: lapack_int, a: *const lapack_complex_double, lda: lapack_int,
                               af: *mut lapack_complex_double, ldaf: lapack_int,
                               ipiv: *mut lapack_int, b: *const lapack_complex_double,
                               ldb: lapack_int, x: *mut lapack_complex_double, ldx: lapack_int,
                               rcond: *mut c_double, ferr: *mut c_double, berr: *mut c_double,
                               work: *mut lapack_complex_double, lwork: lapack_int,
                               rwork: *mut c_double)
                               -> lapack_int;

    pub fn LAPACKE_ssysvxx_work(matrix_layout: c_int, fact: c_char, uplo: c_char, n: lapack_int,
                                nrhs: lapack_int, a: *mut c_float, lda: lapack_int,
                                af: *mut c_float, ldaf: lapack_int, ipiv: *mut lapack_int,
                                equed: *mut c_char, s: *mut c_float, b: *mut c_float,
                                ldb: lapack_int, x: *mut c_float, ldx: lapack_int,
                                rcond: *mut c_float, rpvgrw: *mut c_float, berr: *mut c_float,
                                n_err_bnds: lapack_int, err_bnds_norm: *mut c_float,
                                err_bnds_comp: *mut c_float, nparams: lapack_int,
                                params: *mut c_float, work: *mut c_float, iwork: *mut lapack_int)
                                -> lapack_int;
    pub fn LAPACKE_dsysvxx_work(matrix_layout: c_int, fact: c_char, uplo: c_char, n: lapack_int,
                                nrhs: lapack_int, a: *mut c_double, lda: lapack_int,
                                af: *mut c_double, ldaf: lapack_int, ipiv: *mut lapack_int,
                                equed: *mut c_char, s: *mut c_double, b: *mut c_double,
                                ldb: lapack_int, x: *mut c_double, ldx: lapack_int,
                                rcond: *mut c_double, rpvgrw: *mut c_double, berr: *mut c_double,
                                n_err_bnds: lapack_int, err_bnds_norm: *mut c_double,
                                err_bnds_comp: *mut c_double, nparams: lapack_int,
                                params: *mut c_double, work: *mut c_double, iwork: *mut lapack_int)
                                -> lapack_int;
    pub fn LAPACKE_csysvxx_work(matrix_layout: c_int, fact: c_char, uplo: c_char, n: lapack_int,
                                nrhs: lapack_int, a: *mut lapack_complex_float, lda: lapack_int,
                                af: *mut lapack_complex_float, ldaf: lapack_int,
                                ipiv: *mut lapack_int, equed: *mut c_char, s: *mut c_float,
                                b: *mut lapack_complex_float, ldb: lapack_int,
                                x: *mut lapack_complex_float, ldx: lapack_int, rcond: *mut c_float,
                                rpvgrw: *mut c_float, berr: *mut c_float, n_err_bnds: lapack_int,
                                err_bnds_norm: *mut c_float, err_bnds_comp: *mut c_float,
                                nparams: lapack_int, params: *mut c_float,
                                work: *mut lapack_complex_float, rwork: *mut c_float)
                                -> lapack_int;
    pub fn LAPACKE_zsysvxx_work(matrix_layout: c_int, fact: c_char, uplo: c_char, n: lapack_int,
                                nrhs: lapack_int, a: *mut lapack_complex_double, lda: lapack_int,
                                af: *mut lapack_complex_double, ldaf: lapack_int,
                                ipiv: *mut lapack_int, equed: *mut c_char, s: *mut c_double,
                                b: *mut lapack_complex_double, ldb: lapack_int,
                                x: *mut lapack_complex_double, ldx: lapack_int,
                                rcond: *mut c_double, rpvgrw: *mut c_double, berr: *mut c_double,
                                n_err_bnds: lapack_int, err_bnds_norm: *mut c_double,
                                err_bnds_comp: *mut c_double, nparams: lapack_int,
                                params: *mut c_double, work: *mut lapack_complex_double,
                                rwork: *mut c_double)
                                -> lapack_int;

    pub fn LAPACKE_ssytrd_work(matrix_layout: c_int, uplo: c_char, n: lapack_int, a: *mut c_float,
                               lda: lapack_int, d: *mut c_float, e: *mut c_float,
                               tau: *mut c_float, work: *mut c_float, lwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dsytrd_work(matrix_layout: c_int, uplo: c_char, n: lapack_int, a: *mut c_double,
                               lda: lapack_int, d: *mut c_double, e: *mut c_double,
                               tau: *mut c_double, work: *mut c_double, lwork: lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_ssytrf_work(matrix_layout: c_int, uplo: c_char, n: lapack_int, a: *mut c_float,
                               lda: lapack_int, ipiv: *mut lapack_int, work: *mut c_float,
                               lwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dsytrf_work(matrix_layout: c_int, uplo: c_char, n: lapack_int, a: *mut c_double,
                               lda: lapack_int, ipiv: *mut lapack_int, work: *mut c_double,
                               lwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_csytrf_work(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                               a: *mut lapack_complex_float, lda: lapack_int,
                               ipiv: *mut lapack_int, work: *mut lapack_complex_float,
                               lwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_zsytrf_work(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                               a: *mut lapack_complex_double, lda: lapack_int,
                               ipiv: *mut lapack_int, work: *mut lapack_complex_double,
                               lwork: lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_ssytri_work(matrix_layout: c_int, uplo: c_char, n: lapack_int, a: *mut c_float,
                               lda: lapack_int, ipiv: *const lapack_int, work: *mut c_float)
                               -> lapack_int;
    pub fn LAPACKE_dsytri_work(matrix_layout: c_int, uplo: c_char, n: lapack_int, a: *mut c_double,
                               lda: lapack_int, ipiv: *const lapack_int, work: *mut c_double)
                               -> lapack_int;
    pub fn LAPACKE_csytri_work(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                               a: *mut lapack_complex_float, lda: lapack_int,
                               ipiv: *const lapack_int, work: *mut lapack_complex_float)
                               -> lapack_int;
    pub fn LAPACKE_zsytri_work(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                               a: *mut lapack_complex_double, lda: lapack_int,
                               ipiv: *const lapack_int, work: *mut lapack_complex_double)
                               -> lapack_int;

    pub fn LAPACKE_ssytrs_work(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                               a: *const c_float, lda: lapack_int, ipiv: *const lapack_int,
                               b: *mut c_float, ldb: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dsytrs_work(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                               a: *const c_double, lda: lapack_int, ipiv: *const lapack_int,
                               b: *mut c_double, ldb: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_csytrs_work(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                               a: *const lapack_complex_float, lda: lapack_int,
                               ipiv: *const lapack_int, b: *mut lapack_complex_float,
                               ldb: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_zsytrs_work(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                               a: *const lapack_complex_double, lda: lapack_int,
                               ipiv: *const lapack_int, b: *mut lapack_complex_double,
                               ldb: lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_stbcon_work(matrix_layout: c_int, norm: c_char, uplo: c_char, diag: c_char,
                               n: lapack_int, kd: lapack_int, ab: *const c_float, ldab: lapack_int,
                               rcond: *mut c_float, work: *mut c_float, iwork: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dtbcon_work(matrix_layout: c_int, norm: c_char, uplo: c_char, diag: c_char,
                               n: lapack_int, kd: lapack_int, ab: *const c_double,
                               ldab: lapack_int, rcond: *mut c_double, work: *mut c_double,
                               iwork: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_ctbcon_work(matrix_layout: c_int, norm: c_char, uplo: c_char, diag: c_char,
                               n: lapack_int, kd: lapack_int, ab: *const lapack_complex_float,
                               ldab: lapack_int, rcond: *mut c_float,
                               work: *mut lapack_complex_float, rwork: *mut c_float)
                               -> lapack_int;
    pub fn LAPACKE_ztbcon_work(matrix_layout: c_int, norm: c_char, uplo: c_char, diag: c_char,
                               n: lapack_int, kd: lapack_int, ab: *const lapack_complex_double,
                               ldab: lapack_int, rcond: *mut c_double,
                               work: *mut lapack_complex_double, rwork: *mut c_double)
                               -> lapack_int;

    pub fn LAPACKE_stbrfs_work(matrix_layout: c_int, uplo: c_char, trans: c_char, diag: c_char,
                               n: lapack_int, kd: lapack_int, nrhs: lapack_int, ab: *const c_float,
                               ldab: lapack_int, b: *const c_float, ldb: lapack_int,
                               x: *const c_float, ldx: lapack_int, ferr: *mut c_float,
                               berr: *mut c_float, work: *mut c_float, iwork: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dtbrfs_work(matrix_layout: c_int, uplo: c_char, trans: c_char, diag: c_char,
                               n: lapack_int, kd: lapack_int, nrhs: lapack_int,
                               ab: *const c_double, ldab: lapack_int, b: *const c_double,
                               ldb: lapack_int, x: *const c_double, ldx: lapack_int,
                               ferr: *mut c_double, berr: *mut c_double, work: *mut c_double,
                               iwork: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_ctbrfs_work(matrix_layout: c_int, uplo: c_char, trans: c_char, diag: c_char,
                               n: lapack_int, kd: lapack_int, nrhs: lapack_int,
                               ab: *const lapack_complex_float, ldab: lapack_int,
                               b: *const lapack_complex_float, ldb: lapack_int,
                               x: *const lapack_complex_float, ldx: lapack_int, ferr: *mut c_float,
                               berr: *mut c_float, work: *mut lapack_complex_float,
                               rwork: *mut c_float)
                               -> lapack_int;
    pub fn LAPACKE_ztbrfs_work(matrix_layout: c_int, uplo: c_char, trans: c_char, diag: c_char,
                               n: lapack_int, kd: lapack_int, nrhs: lapack_int,
                               ab: *const lapack_complex_double, ldab: lapack_int,
                               b: *const lapack_complex_double, ldb: lapack_int,
                               x: *const lapack_complex_double, ldx: lapack_int,
                               ferr: *mut c_double, berr: *mut c_double,
                               work: *mut lapack_complex_double, rwork: *mut c_double)
                               -> lapack_int;

    pub fn LAPACKE_stbtrs_work(matrix_layout: c_int, uplo: c_char, trans: c_char, diag: c_char,
                               n: lapack_int, kd: lapack_int, nrhs: lapack_int, ab: *const c_float,
                               ldab: lapack_int, b: *mut c_float, ldb: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dtbtrs_work(matrix_layout: c_int, uplo: c_char, trans: c_char, diag: c_char,
                               n: lapack_int, kd: lapack_int, nrhs: lapack_int,
                               ab: *const c_double, ldab: lapack_int, b: *mut c_double,
                               ldb: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_ctbtrs_work(matrix_layout: c_int, uplo: c_char, trans: c_char, diag: c_char,
                               n: lapack_int, kd: lapack_int, nrhs: lapack_int,
                               ab: *const lapack_complex_float, ldab: lapack_int,
                               b: *mut lapack_complex_float, ldb: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_ztbtrs_work(matrix_layout: c_int, uplo: c_char, trans: c_char, diag: c_char,
                               n: lapack_int, kd: lapack_int, nrhs: lapack_int,
                               ab: *const lapack_complex_double, ldab: lapack_int,
                               b: *mut lapack_complex_double, ldb: lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_stfsm_work(matrix_layout: c_int, transr: c_char, side: c_char, uplo: c_char,
                              trans: c_char, diag: c_char, m: lapack_int, n: lapack_int,
                              alpha: c_float, a: *const c_float, b: *mut c_float, ldb: lapack_int)
                              -> lapack_int;
    pub fn LAPACKE_dtfsm_work(matrix_layout: c_int, transr: c_char, side: c_char, uplo: c_char,
                              trans: c_char, diag: c_char, m: lapack_int, n: lapack_int,
                              alpha: c_double, a: *const c_double, b: *mut c_double,
                              ldb: lapack_int)
                              -> lapack_int;
    pub fn LAPACKE_ctfsm_work(matrix_layout: c_int, transr: c_char, side: c_char, uplo: c_char,
                              trans: c_char, diag: c_char, m: lapack_int, n: lapack_int,
                              alpha: lapack_complex_float, a: *const lapack_complex_float,
                              b: *mut lapack_complex_float, ldb: lapack_int)
                              -> lapack_int;
    pub fn LAPACKE_ztfsm_work(matrix_layout: c_int, transr: c_char, side: c_char, uplo: c_char,
                              trans: c_char, diag: c_char, m: lapack_int, n: lapack_int,
                              alpha: lapack_complex_double, a: *const lapack_complex_double,
                              b: *mut lapack_complex_double, ldb: lapack_int)
                              -> lapack_int;

    pub fn LAPACKE_stftri_work(matrix_layout: c_int, transr: c_char, uplo: c_char, diag: c_char,
                               n: lapack_int, a: *mut c_float)
                               -> lapack_int;
    pub fn LAPACKE_dtftri_work(matrix_layout: c_int, transr: c_char, uplo: c_char, diag: c_char,
                               n: lapack_int, a: *mut c_double)
                               -> lapack_int;
    pub fn LAPACKE_ctftri_work(matrix_layout: c_int, transr: c_char, uplo: c_char, diag: c_char,
                               n: lapack_int, a: *mut lapack_complex_float)
                               -> lapack_int;
    pub fn LAPACKE_ztftri_work(matrix_layout: c_int, transr: c_char, uplo: c_char, diag: c_char,
                               n: lapack_int, a: *mut lapack_complex_double)
                               -> lapack_int;

    pub fn LAPACKE_stfttp_work(matrix_layout: c_int, transr: c_char, uplo: c_char, n: lapack_int,
                               arf: *const c_float, ap: *mut c_float)
                               -> lapack_int;
    pub fn LAPACKE_dtfttp_work(matrix_layout: c_int, transr: c_char, uplo: c_char, n: lapack_int,
                               arf: *const c_double, ap: *mut c_double)
                               -> lapack_int;
    pub fn LAPACKE_ctfttp_work(matrix_layout: c_int, transr: c_char, uplo: c_char, n: lapack_int,
                               arf: *const lapack_complex_float, ap: *mut lapack_complex_float)
                               -> lapack_int;
    pub fn LAPACKE_ztfttp_work(matrix_layout: c_int, transr: c_char, uplo: c_char, n: lapack_int,
                               arf: *const lapack_complex_double, ap: *mut lapack_complex_double)
                               -> lapack_int;

    pub fn LAPACKE_stfttr_work(matrix_layout: c_int, transr: c_char, uplo: c_char, n: lapack_int,
                               arf: *const c_float, a: *mut c_float, lda: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dtfttr_work(matrix_layout: c_int, transr: c_char, uplo: c_char, n: lapack_int,
                               arf: *const c_double, a: *mut c_double, lda: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_ctfttr_work(matrix_layout: c_int, transr: c_char, uplo: c_char, n: lapack_int,
                               arf: *const lapack_complex_float, a: *mut lapack_complex_float,
                               lda: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_ztfttr_work(matrix_layout: c_int, transr: c_char, uplo: c_char, n: lapack_int,
                               arf: *const lapack_complex_double, a: *mut lapack_complex_double,
                               lda: lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_stgevc_work(matrix_layout: c_int, side: c_char, howmny: c_char,
                               select: *const lapack_logical, n: lapack_int, s: *const c_float,
                               lds: lapack_int, p: *const c_float, ldp: lapack_int,
                               vl: *mut c_float, ldvl: lapack_int, vr: *mut c_float,
                               ldvr: lapack_int, mm: lapack_int, m: *mut lapack_int,
                               work: *mut c_float)
                               -> lapack_int;
    pub fn LAPACKE_dtgevc_work(matrix_layout: c_int, side: c_char, howmny: c_char,
                               select: *const lapack_logical, n: lapack_int, s: *const c_double,
                               lds: lapack_int, p: *const c_double, ldp: lapack_int,
                               vl: *mut c_double, ldvl: lapack_int, vr: *mut c_double,
                               ldvr: lapack_int, mm: lapack_int, m: *mut lapack_int,
                               work: *mut c_double)
                               -> lapack_int;
    pub fn LAPACKE_ctgevc_work(matrix_layout: c_int, side: c_char, howmny: c_char,
                               select: *const lapack_logical, n: lapack_int,
                               s: *const lapack_complex_float, lds: lapack_int,
                               p: *const lapack_complex_float, ldp: lapack_int,
                               vl: *mut lapack_complex_float, ldvl: lapack_int,
                               vr: *mut lapack_complex_float, ldvr: lapack_int, mm: lapack_int,
                               m: *mut lapack_int, work: *mut lapack_complex_float,
                               rwork: *mut c_float)
                               -> lapack_int;
    pub fn LAPACKE_ztgevc_work(matrix_layout: c_int, side: c_char, howmny: c_char,
                               select: *const lapack_logical, n: lapack_int,
                               s: *const lapack_complex_double, lds: lapack_int,
                               p: *const lapack_complex_double, ldp: lapack_int,
                               vl: *mut lapack_complex_double, ldvl: lapack_int,
                               vr: *mut lapack_complex_double, ldvr: lapack_int, mm: lapack_int,
                               m: *mut lapack_int, work: *mut lapack_complex_double,
                               rwork: *mut c_double)
                               -> lapack_int;

    pub fn LAPACKE_stgexc_work(matrix_layout: c_int, wantq: lapack_logical, wantz: lapack_logical,
                               n: lapack_int, a: *mut c_float, lda: lapack_int, b: *mut c_float,
                               ldb: lapack_int, q: *mut c_float, ldq: lapack_int, z: *mut c_float,
                               ldz: lapack_int, ifst: *mut lapack_int, ilst: *mut lapack_int,
                               work: *mut c_float, lwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dtgexc_work(matrix_layout: c_int, wantq: lapack_logical, wantz: lapack_logical,
                               n: lapack_int, a: *mut c_double, lda: lapack_int, b: *mut c_double,
                               ldb: lapack_int, q: *mut c_double, ldq: lapack_int,
                               z: *mut c_double, ldz: lapack_int, ifst: *mut lapack_int,
                               ilst: *mut lapack_int, work: *mut c_double, lwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_ctgexc_work(matrix_layout: c_int, wantq: lapack_logical, wantz: lapack_logical,
                               n: lapack_int, a: *mut lapack_complex_float, lda: lapack_int,
                               b: *mut lapack_complex_float, ldb: lapack_int,
                               q: *mut lapack_complex_float, ldq: lapack_int,
                               z: *mut lapack_complex_float, ldz: lapack_int, ifst: lapack_int,
                               ilst: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_ztgexc_work(matrix_layout: c_int, wantq: lapack_logical, wantz: lapack_logical,
                               n: lapack_int, a: *mut lapack_complex_double, lda: lapack_int,
                               b: *mut lapack_complex_double, ldb: lapack_int,
                               q: *mut lapack_complex_double, ldq: lapack_int,
                               z: *mut lapack_complex_double, ldz: lapack_int, ifst: lapack_int,
                               ilst: lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_stgsen_work(matrix_layout: c_int, ijob: lapack_int, wantq: lapack_logical,
                               wantz: lapack_logical, select: *const lapack_logical, n: lapack_int,
                               a: *mut c_float, lda: lapack_int, b: *mut c_float, ldb: lapack_int,
                               alphar: *mut c_float, alphai: *mut c_float, beta: *mut c_float,
                               q: *mut c_float, ldq: lapack_int, z: *mut c_float, ldz: lapack_int,
                               m: *mut lapack_int, pl: *mut c_float, pr: *mut c_float,
                               dif: *mut c_float, work: *mut c_float, lwork: lapack_int,
                               iwork: *mut lapack_int, liwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dtgsen_work(matrix_layout: c_int, ijob: lapack_int, wantq: lapack_logical,
                               wantz: lapack_logical, select: *const lapack_logical, n: lapack_int,
                               a: *mut c_double, lda: lapack_int, b: *mut c_double,
                               ldb: lapack_int, alphar: *mut c_double, alphai: *mut c_double,
                               beta: *mut c_double, q: *mut c_double, ldq: lapack_int,
                               z: *mut c_double, ldz: lapack_int, m: *mut lapack_int,
                               pl: *mut c_double, pr: *mut c_double, dif: *mut c_double,
                               work: *mut c_double, lwork: lapack_int, iwork: *mut lapack_int,
                               liwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_ctgsen_work(matrix_layout: c_int, ijob: lapack_int, wantq: lapack_logical,
                               wantz: lapack_logical, select: *const lapack_logical, n: lapack_int,
                               a: *mut lapack_complex_float, lda: lapack_int,
                               b: *mut lapack_complex_float, ldb: lapack_int,
                               alpha: *mut lapack_complex_float, beta: *mut lapack_complex_float,
                               q: *mut lapack_complex_float, ldq: lapack_int,
                               z: *mut lapack_complex_float, ldz: lapack_int, m: *mut lapack_int,
                               pl: *mut c_float, pr: *mut c_float, dif: *mut c_float,
                               work: *mut lapack_complex_float, lwork: lapack_int,
                               iwork: *mut lapack_int, liwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_ztgsen_work(matrix_layout: c_int, ijob: lapack_int, wantq: lapack_logical,
                               wantz: lapack_logical, select: *const lapack_logical, n: lapack_int,
                               a: *mut lapack_complex_double, lda: lapack_int,
                               b: *mut lapack_complex_double, ldb: lapack_int,
                               alpha: *mut lapack_complex_double, beta: *mut lapack_complex_double,
                               q: *mut lapack_complex_double, ldq: lapack_int,
                               z: *mut lapack_complex_double, ldz: lapack_int, m: *mut lapack_int,
                               pl: *mut c_double, pr: *mut c_double, dif: *mut c_double,
                               work: *mut lapack_complex_double, lwork: lapack_int,
                               iwork: *mut lapack_int, liwork: lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_stgsja_work(matrix_layout: c_int, jobu: c_char, jobv: c_char, jobq: c_char,
                               m: lapack_int, p: lapack_int, n: lapack_int, k: lapack_int,
                               l: lapack_int, a: *mut c_float, lda: lapack_int, b: *mut c_float,
                               ldb: lapack_int, tola: c_float, tolb: c_float, alpha: *mut c_float,
                               beta: *mut c_float, u: *mut c_float, ldu: lapack_int,
                               v: *mut c_float, ldv: lapack_int, q: *mut c_float, ldq: lapack_int,
                               work: *mut c_float, ncycle: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dtgsja_work(matrix_layout: c_int, jobu: c_char, jobv: c_char, jobq: c_char,
                               m: lapack_int, p: lapack_int, n: lapack_int, k: lapack_int,
                               l: lapack_int, a: *mut c_double, lda: lapack_int, b: *mut c_double,
                               ldb: lapack_int, tola: c_double, tolb: c_double,
                               alpha: *mut c_double, beta: *mut c_double, u: *mut c_double,
                               ldu: lapack_int, v: *mut c_double, ldv: lapack_int,
                               q: *mut c_double, ldq: lapack_int, work: *mut c_double,
                               ncycle: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_ctgsja_work(matrix_layout: c_int, jobu: c_char, jobv: c_char, jobq: c_char,
                               m: lapack_int, p: lapack_int, n: lapack_int, k: lapack_int,
                               l: lapack_int, a: *mut lapack_complex_float, lda: lapack_int,
                               b: *mut lapack_complex_float, ldb: lapack_int, tola: c_float,
                               tolb: c_float, alpha: *mut c_float, beta: *mut c_float,
                               u: *mut lapack_complex_float, ldu: lapack_int,
                               v: *mut lapack_complex_float, ldv: lapack_int,
                               q: *mut lapack_complex_float, ldq: lapack_int,
                               work: *mut lapack_complex_float, ncycle: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_ztgsja_work(matrix_layout: c_int, jobu: c_char, jobv: c_char, jobq: c_char,
                               m: lapack_int, p: lapack_int, n: lapack_int, k: lapack_int,
                               l: lapack_int, a: *mut lapack_complex_double, lda: lapack_int,
                               b: *mut lapack_complex_double, ldb: lapack_int, tola: c_double,
                               tolb: c_double, alpha: *mut c_double, beta: *mut c_double,
                               u: *mut lapack_complex_double, ldu: lapack_int,
                               v: *mut lapack_complex_double, ldv: lapack_int,
                               q: *mut lapack_complex_double, ldq: lapack_int,
                               work: *mut lapack_complex_double, ncycle: *mut lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_stgsna_work(matrix_layout: c_int, job: c_char, howmny: c_char,
                               select: *const lapack_logical, n: lapack_int, a: *const c_float,
                               lda: lapack_int, b: *const c_float, ldb: lapack_int,
                               vl: *const c_float, ldvl: lapack_int, vr: *const c_float,
                               ldvr: lapack_int, s: *mut c_float, dif: *mut c_float,
                               mm: lapack_int, m: *mut lapack_int, work: *mut c_float,
                               lwork: lapack_int, iwork: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dtgsna_work(matrix_layout: c_int, job: c_char, howmny: c_char,
                               select: *const lapack_logical, n: lapack_int, a: *const c_double,
                               lda: lapack_int, b: *const c_double, ldb: lapack_int,
                               vl: *const c_double, ldvl: lapack_int, vr: *const c_double,
                               ldvr: lapack_int, s: *mut c_double, dif: *mut c_double,
                               mm: lapack_int, m: *mut lapack_int, work: *mut c_double,
                               lwork: lapack_int, iwork: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_ctgsna_work(matrix_layout: c_int, job: c_char, howmny: c_char,
                               select: *const lapack_logical, n: lapack_int,
                               a: *const lapack_complex_float, lda: lapack_int,
                               b: *const lapack_complex_float, ldb: lapack_int,
                               vl: *const lapack_complex_float, ldvl: lapack_int,
                               vr: *const lapack_complex_float, ldvr: lapack_int, s: *mut c_float,
                               dif: *mut c_float, mm: lapack_int, m: *mut lapack_int,
                               work: *mut lapack_complex_float, lwork: lapack_int,
                               iwork: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_ztgsna_work(matrix_layout: c_int, job: c_char, howmny: c_char,
                               select: *const lapack_logical, n: lapack_int,
                               a: *const lapack_complex_double, lda: lapack_int,
                               b: *const lapack_complex_double, ldb: lapack_int,
                               vl: *const lapack_complex_double, ldvl: lapack_int,
                               vr: *const lapack_complex_double, ldvr: lapack_int,
                               s: *mut c_double, dif: *mut c_double, mm: lapack_int,
                               m: *mut lapack_int, work: *mut lapack_complex_double,
                               lwork: lapack_int, iwork: *mut lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_stgsyl_work(matrix_layout: c_int, trans: c_char, ijob: lapack_int,
                               m: lapack_int, n: lapack_int, a: *const c_float, lda: lapack_int,
                               b: *const c_float, ldb: lapack_int, c: *mut c_float,
                               ldc: lapack_int, d: *const c_float, ldd: lapack_int,
                               e: *const c_float, lde: lapack_int, f: *mut c_float,
                               ldf: lapack_int, scale: *mut c_float, dif: *mut c_float,
                               work: *mut c_float, lwork: lapack_int, iwork: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dtgsyl_work(matrix_layout: c_int, trans: c_char, ijob: lapack_int,
                               m: lapack_int, n: lapack_int, a: *const c_double, lda: lapack_int,
                               b: *const c_double, ldb: lapack_int, c: *mut c_double,
                               ldc: lapack_int, d: *const c_double, ldd: lapack_int,
                               e: *const c_double, lde: lapack_int, f: *mut c_double,
                               ldf: lapack_int, scale: *mut c_double, dif: *mut c_double,
                               work: *mut c_double, lwork: lapack_int, iwork: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_ctgsyl_work(matrix_layout: c_int, trans: c_char, ijob: lapack_int,
                               m: lapack_int, n: lapack_int, a: *const lapack_complex_float,
                               lda: lapack_int, b: *const lapack_complex_float, ldb: lapack_int,
                               c: *mut lapack_complex_float, ldc: lapack_int,
                               d: *const lapack_complex_float, ldd: lapack_int,
                               e: *const lapack_complex_float, lde: lapack_int,
                               f: *mut lapack_complex_float, ldf: lapack_int, scale: *mut c_float,
                               dif: *mut c_float, work: *mut lapack_complex_float,
                               lwork: lapack_int, iwork: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_ztgsyl_work(matrix_layout: c_int, trans: c_char, ijob: lapack_int,
                               m: lapack_int, n: lapack_int, a: *const lapack_complex_double,
                               lda: lapack_int, b: *const lapack_complex_double, ldb: lapack_int,
                               c: *mut lapack_complex_double, ldc: lapack_int,
                               d: *const lapack_complex_double, ldd: lapack_int,
                               e: *const lapack_complex_double, lde: lapack_int,
                               f: *mut lapack_complex_double, ldf: lapack_int,
                               scale: *mut c_double, dif: *mut c_double,
                               work: *mut lapack_complex_double, lwork: lapack_int,
                               iwork: *mut lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_stpcon_work(matrix_layout: c_int, norm: c_char, uplo: c_char, diag: c_char,
                               n: lapack_int, ap: *const c_float, rcond: *mut c_float,
                               work: *mut c_float, iwork: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dtpcon_work(matrix_layout: c_int, norm: c_char, uplo: c_char, diag: c_char,
                               n: lapack_int, ap: *const c_double, rcond: *mut c_double,
                               work: *mut c_double, iwork: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_ctpcon_work(matrix_layout: c_int, norm: c_char, uplo: c_char, diag: c_char,
                               n: lapack_int, ap: *const lapack_complex_float, rcond: *mut c_float,
                               work: *mut lapack_complex_float, rwork: *mut c_float)
                               -> lapack_int;
    pub fn LAPACKE_ztpcon_work(matrix_layout: c_int, norm: c_char, uplo: c_char, diag: c_char,
                               n: lapack_int, ap: *const lapack_complex_double,
                               rcond: *mut c_double, work: *mut lapack_complex_double,
                               rwork: *mut c_double)
                               -> lapack_int;

    pub fn LAPACKE_stprfs_work(matrix_layout: c_int, uplo: c_char, trans: c_char, diag: c_char,
                               n: lapack_int, nrhs: lapack_int, ap: *const c_float,
                               b: *const c_float, ldb: lapack_int, x: *const c_float,
                               ldx: lapack_int, ferr: *mut c_float, berr: *mut c_float,
                               work: *mut c_float, iwork: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dtprfs_work(matrix_layout: c_int, uplo: c_char, trans: c_char, diag: c_char,
                               n: lapack_int, nrhs: lapack_int, ap: *const c_double,
                               b: *const c_double, ldb: lapack_int, x: *const c_double,
                               ldx: lapack_int, ferr: *mut c_double, berr: *mut c_double,
                               work: *mut c_double, iwork: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_ctprfs_work(matrix_layout: c_int, uplo: c_char, trans: c_char, diag: c_char,
                               n: lapack_int, nrhs: lapack_int, ap: *const lapack_complex_float,
                               b: *const lapack_complex_float, ldb: lapack_int,
                               x: *const lapack_complex_float, ldx: lapack_int, ferr: *mut c_float,
                               berr: *mut c_float, work: *mut lapack_complex_float,
                               rwork: *mut c_float)
                               -> lapack_int;
    pub fn LAPACKE_ztprfs_work(matrix_layout: c_int, uplo: c_char, trans: c_char, diag: c_char,
                               n: lapack_int, nrhs: lapack_int, ap: *const lapack_complex_double,
                               b: *const lapack_complex_double, ldb: lapack_int,
                               x: *const lapack_complex_double, ldx: lapack_int,
                               ferr: *mut c_double, berr: *mut c_double,
                               work: *mut lapack_complex_double, rwork: *mut c_double)
                               -> lapack_int;

    pub fn LAPACKE_stptri_work(matrix_layout: c_int, uplo: c_char, diag: c_char, n: lapack_int,
                               ap: *mut c_float)
                               -> lapack_int;
    pub fn LAPACKE_dtptri_work(matrix_layout: c_int, uplo: c_char, diag: c_char, n: lapack_int,
                               ap: *mut c_double)
                               -> lapack_int;
    pub fn LAPACKE_ctptri_work(matrix_layout: c_int, uplo: c_char, diag: c_char, n: lapack_int,
                               ap: *mut lapack_complex_float)
                               -> lapack_int;
    pub fn LAPACKE_ztptri_work(matrix_layout: c_int, uplo: c_char, diag: c_char, n: lapack_int,
                               ap: *mut lapack_complex_double)
                               -> lapack_int;

    pub fn LAPACKE_stptrs_work(matrix_layout: c_int, uplo: c_char, trans: c_char, diag: c_char,
                               n: lapack_int, nrhs: lapack_int, ap: *const c_float,
                               b: *mut c_float, ldb: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dtptrs_work(matrix_layout: c_int, uplo: c_char, trans: c_char, diag: c_char,
                               n: lapack_int, nrhs: lapack_int, ap: *const c_double,
                               b: *mut c_double, ldb: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_ctptrs_work(matrix_layout: c_int, uplo: c_char, trans: c_char, diag: c_char,
                               n: lapack_int, nrhs: lapack_int, ap: *const lapack_complex_float,
                               b: *mut lapack_complex_float, ldb: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_ztptrs_work(matrix_layout: c_int, uplo: c_char, trans: c_char, diag: c_char,
                               n: lapack_int, nrhs: lapack_int, ap: *const lapack_complex_double,
                               b: *mut lapack_complex_double, ldb: lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_stpttf_work(matrix_layout: c_int, transr: c_char, uplo: c_char, n: lapack_int,
                               ap: *const c_float, arf: *mut c_float)
                               -> lapack_int;
    pub fn LAPACKE_dtpttf_work(matrix_layout: c_int, transr: c_char, uplo: c_char, n: lapack_int,
                               ap: *const c_double, arf: *mut c_double)
                               -> lapack_int;
    pub fn LAPACKE_ctpttf_work(matrix_layout: c_int, transr: c_char, uplo: c_char, n: lapack_int,
                               ap: *const lapack_complex_float, arf: *mut lapack_complex_float)
                               -> lapack_int;
    pub fn LAPACKE_ztpttf_work(matrix_layout: c_int, transr: c_char, uplo: c_char, n: lapack_int,
                               ap: *const lapack_complex_double, arf: *mut lapack_complex_double)
                               -> lapack_int;

    pub fn LAPACKE_stpttr_work(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                               ap: *const c_float, a: *mut c_float, lda: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dtpttr_work(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                               ap: *const c_double, a: *mut c_double, lda: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_ctpttr_work(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                               ap: *const lapack_complex_float, a: *mut lapack_complex_float,
                               lda: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_ztpttr_work(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                               ap: *const lapack_complex_double, a: *mut lapack_complex_double,
                               lda: lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_strcon_work(matrix_layout: c_int, norm: c_char, uplo: c_char, diag: c_char,
                               n: lapack_int, a: *const c_float, lda: lapack_int,
                               rcond: *mut c_float, work: *mut c_float, iwork: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dtrcon_work(matrix_layout: c_int, norm: c_char, uplo: c_char, diag: c_char,
                               n: lapack_int, a: *const c_double, lda: lapack_int,
                               rcond: *mut c_double, work: *mut c_double, iwork: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_ctrcon_work(matrix_layout: c_int, norm: c_char, uplo: c_char, diag: c_char,
                               n: lapack_int, a: *const lapack_complex_float, lda: lapack_int,
                               rcond: *mut c_float, work: *mut lapack_complex_float,
                               rwork: *mut c_float)
                               -> lapack_int;
    pub fn LAPACKE_ztrcon_work(matrix_layout: c_int, norm: c_char, uplo: c_char, diag: c_char,
                               n: lapack_int, a: *const lapack_complex_double, lda: lapack_int,
                               rcond: *mut c_double, work: *mut lapack_complex_double,
                               rwork: *mut c_double)
                               -> lapack_int;

    pub fn LAPACKE_strevc_work(matrix_layout: c_int, side: c_char, howmny: c_char,
                               select: *mut lapack_logical, n: lapack_int, t: *const c_float,
                               ldt: lapack_int, vl: *mut c_float, ldvl: lapack_int,
                               vr: *mut c_float, ldvr: lapack_int, mm: lapack_int,
                               m: *mut lapack_int, work: *mut c_float)
                               -> lapack_int;
    pub fn LAPACKE_dtrevc_work(matrix_layout: c_int, side: c_char, howmny: c_char,
                               select: *mut lapack_logical, n: lapack_int, t: *const c_double,
                               ldt: lapack_int, vl: *mut c_double, ldvl: lapack_int,
                               vr: *mut c_double, ldvr: lapack_int, mm: lapack_int,
                               m: *mut lapack_int, work: *mut c_double)
                               -> lapack_int;
    pub fn LAPACKE_ctrevc_work(matrix_layout: c_int, side: c_char, howmny: c_char,
                               select: *const lapack_logical, n: lapack_int,
                               t: *mut lapack_complex_float, ldt: lapack_int,
                               vl: *mut lapack_complex_float, ldvl: lapack_int,
                               vr: *mut lapack_complex_float, ldvr: lapack_int, mm: lapack_int,
                               m: *mut lapack_int, work: *mut lapack_complex_float,
                               rwork: *mut c_float)
                               -> lapack_int;
    pub fn LAPACKE_ztrevc_work(matrix_layout: c_int, side: c_char, howmny: c_char,
                               select: *const lapack_logical, n: lapack_int,
                               t: *mut lapack_complex_double, ldt: lapack_int,
                               vl: *mut lapack_complex_double, ldvl: lapack_int,
                               vr: *mut lapack_complex_double, ldvr: lapack_int, mm: lapack_int,
                               m: *mut lapack_int, work: *mut lapack_complex_double,
                               rwork: *mut c_double)
                               -> lapack_int;

    pub fn LAPACKE_strexc_work(matrix_layout: c_int, compq: c_char, n: lapack_int, t: *mut c_float,
                               ldt: lapack_int, q: *mut c_float, ldq: lapack_int,
                               ifst: *mut lapack_int, ilst: *mut lapack_int, work: *mut c_float)
                               -> lapack_int;
    pub fn LAPACKE_dtrexc_work(matrix_layout: c_int, compq: c_char, n: lapack_int,
                               t: *mut c_double, ldt: lapack_int, q: *mut c_double,
                               ldq: lapack_int, ifst: *mut lapack_int, ilst: *mut lapack_int,
                               work: *mut c_double)
                               -> lapack_int;
    pub fn LAPACKE_ctrexc_work(matrix_layout: c_int, compq: c_char, n: lapack_int,
                               t: *mut lapack_complex_float, ldt: lapack_int,
                               q: *mut lapack_complex_float, ldq: lapack_int, ifst: lapack_int,
                               ilst: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_ztrexc_work(matrix_layout: c_int, compq: c_char, n: lapack_int,
                               t: *mut lapack_complex_double, ldt: lapack_int,
                               q: *mut lapack_complex_double, ldq: lapack_int, ifst: lapack_int,
                               ilst: lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_strrfs_work(matrix_layout: c_int, uplo: c_char, trans: c_char, diag: c_char,
                               n: lapack_int, nrhs: lapack_int, a: *const c_float, lda: lapack_int,
                               b: *const c_float, ldb: lapack_int, x: *const c_float,
                               ldx: lapack_int, ferr: *mut c_float, berr: *mut c_float,
                               work: *mut c_float, iwork: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dtrrfs_work(matrix_layout: c_int, uplo: c_char, trans: c_char, diag: c_char,
                               n: lapack_int, nrhs: lapack_int, a: *const c_double,
                               lda: lapack_int, b: *const c_double, ldb: lapack_int,
                               x: *const c_double, ldx: lapack_int, ferr: *mut c_double,
                               berr: *mut c_double, work: *mut c_double, iwork: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_ctrrfs_work(matrix_layout: c_int, uplo: c_char, trans: c_char, diag: c_char,
                               n: lapack_int, nrhs: lapack_int, a: *const lapack_complex_float,
                               lda: lapack_int, b: *const lapack_complex_float, ldb: lapack_int,
                               x: *const lapack_complex_float, ldx: lapack_int, ferr: *mut c_float,
                               berr: *mut c_float, work: *mut lapack_complex_float,
                               rwork: *mut c_float)
                               -> lapack_int;
    pub fn LAPACKE_ztrrfs_work(matrix_layout: c_int, uplo: c_char, trans: c_char, diag: c_char,
                               n: lapack_int, nrhs: lapack_int, a: *const lapack_complex_double,
                               lda: lapack_int, b: *const lapack_complex_double, ldb: lapack_int,
                               x: *const lapack_complex_double, ldx: lapack_int,
                               ferr: *mut c_double, berr: *mut c_double,
                               work: *mut lapack_complex_double, rwork: *mut c_double)
                               -> lapack_int;

    pub fn LAPACKE_strsen_work(matrix_layout: c_int, job: c_char, compq: c_char,
                               select: *const lapack_logical, n: lapack_int, t: *mut c_float,
                               ldt: lapack_int, q: *mut c_float, ldq: lapack_int, wr: *mut c_float,
                               wi: *mut c_float, m: *mut lapack_int, s: *mut c_float,
                               sep: *mut c_float, work: *mut c_float, lwork: lapack_int,
                               iwork: *mut lapack_int, liwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dtrsen_work(matrix_layout: c_int, job: c_char, compq: c_char,
                               select: *const lapack_logical, n: lapack_int, t: *mut c_double,
                               ldt: lapack_int, q: *mut c_double, ldq: lapack_int,
                               wr: *mut c_double, wi: *mut c_double, m: *mut lapack_int,
                               s: *mut c_double, sep: *mut c_double, work: *mut c_double,
                               lwork: lapack_int, iwork: *mut lapack_int, liwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_ctrsen_work(matrix_layout: c_int, job: c_char, compq: c_char,
                               select: *const lapack_logical, n: lapack_int,
                               t: *mut lapack_complex_float, ldt: lapack_int,
                               q: *mut lapack_complex_float, ldq: lapack_int,
                               w: *mut lapack_complex_float, m: *mut lapack_int, s: *mut c_float,
                               sep: *mut c_float, work: *mut lapack_complex_float,
                               lwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_ztrsen_work(matrix_layout: c_int, job: c_char, compq: c_char,
                               select: *const lapack_logical, n: lapack_int,
                               t: *mut lapack_complex_double, ldt: lapack_int,
                               q: *mut lapack_complex_double, ldq: lapack_int,
                               w: *mut lapack_complex_double, m: *mut lapack_int, s: *mut c_double,
                               sep: *mut c_double, work: *mut lapack_complex_double,
                               lwork: lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_strsna_work(matrix_layout: c_int, job: c_char, howmny: c_char,
                               select: *const lapack_logical, n: lapack_int, t: *const c_float,
                               ldt: lapack_int, vl: *const c_float, ldvl: lapack_int,
                               vr: *const c_float, ldvr: lapack_int, s: *mut c_float,
                               sep: *mut c_float, mm: lapack_int, m: *mut lapack_int,
                               work: *mut c_float, ldwork: lapack_int, iwork: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dtrsna_work(matrix_layout: c_int, job: c_char, howmny: c_char,
                               select: *const lapack_logical, n: lapack_int, t: *const c_double,
                               ldt: lapack_int, vl: *const c_double, ldvl: lapack_int,
                               vr: *const c_double, ldvr: lapack_int, s: *mut c_double,
                               sep: *mut c_double, mm: lapack_int, m: *mut lapack_int,
                               work: *mut c_double, ldwork: lapack_int, iwork: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_ctrsna_work(matrix_layout: c_int, job: c_char, howmny: c_char,
                               select: *const lapack_logical, n: lapack_int,
                               t: *const lapack_complex_float, ldt: lapack_int,
                               vl: *const lapack_complex_float, ldvl: lapack_int,
                               vr: *const lapack_complex_float, ldvr: lapack_int, s: *mut c_float,
                               sep: *mut c_float, mm: lapack_int, m: *mut lapack_int,
                               work: *mut lapack_complex_float, ldwork: lapack_int,
                               rwork: *mut c_float)
                               -> lapack_int;
    pub fn LAPACKE_ztrsna_work(matrix_layout: c_int, job: c_char, howmny: c_char,
                               select: *const lapack_logical, n: lapack_int,
                               t: *const lapack_complex_double, ldt: lapack_int,
                               vl: *const lapack_complex_double, ldvl: lapack_int,
                               vr: *const lapack_complex_double, ldvr: lapack_int,
                               s: *mut c_double, sep: *mut c_double, mm: lapack_int,
                               m: *mut lapack_int, work: *mut lapack_complex_double,
                               ldwork: lapack_int, rwork: *mut c_double)
                               -> lapack_int;

    pub fn LAPACKE_strsyl_work(matrix_layout: c_int, trana: c_char, tranb: c_char,
                               isgn: lapack_int, m: lapack_int, n: lapack_int, a: *const c_float,
                               lda: lapack_int, b: *const c_float, ldb: lapack_int,
                               c: *mut c_float, ldc: lapack_int, scale: *mut c_float)
                               -> lapack_int;
    pub fn LAPACKE_dtrsyl_work(matrix_layout: c_int, trana: c_char, tranb: c_char,
                               isgn: lapack_int, m: lapack_int, n: lapack_int, a: *const c_double,
                               lda: lapack_int, b: *const c_double, ldb: lapack_int,
                               c: *mut c_double, ldc: lapack_int, scale: *mut c_double)
                               -> lapack_int;
    pub fn LAPACKE_ctrsyl_work(matrix_layout: c_int, trana: c_char, tranb: c_char,
                               isgn: lapack_int, m: lapack_int, n: lapack_int,
                               a: *const lapack_complex_float, lda: lapack_int,
                               b: *const lapack_complex_float, ldb: lapack_int,
                               c: *mut lapack_complex_float, ldc: lapack_int, scale: *mut c_float)
                               -> lapack_int;
    pub fn LAPACKE_ztrsyl_work(matrix_layout: c_int, trana: c_char, tranb: c_char,
                               isgn: lapack_int, m: lapack_int, n: lapack_int,
                               a: *const lapack_complex_double, lda: lapack_int,
                               b: *const lapack_complex_double, ldb: lapack_int,
                               c: *mut lapack_complex_double, ldc: lapack_int,
                               scale: *mut c_double)
                               -> lapack_int;

    pub fn LAPACKE_strtri_work(matrix_layout: c_int, uplo: c_char, diag: c_char, n: lapack_int,
                               a: *mut c_float, lda: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dtrtri_work(matrix_layout: c_int, uplo: c_char, diag: c_char, n: lapack_int,
                               a: *mut c_double, lda: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_ctrtri_work(matrix_layout: c_int, uplo: c_char, diag: c_char, n: lapack_int,
                               a: *mut lapack_complex_float, lda: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_ztrtri_work(matrix_layout: c_int, uplo: c_char, diag: c_char, n: lapack_int,
                               a: *mut lapack_complex_double, lda: lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_strtrs_work(matrix_layout: c_int, uplo: c_char, trans: c_char, diag: c_char,
                               n: lapack_int, nrhs: lapack_int, a: *const c_float, lda: lapack_int,
                               b: *mut c_float, ldb: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dtrtrs_work(matrix_layout: c_int, uplo: c_char, trans: c_char, diag: c_char,
                               n: lapack_int, nrhs: lapack_int, a: *const c_double,
                               lda: lapack_int, b: *mut c_double, ldb: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_ctrtrs_work(matrix_layout: c_int, uplo: c_char, trans: c_char, diag: c_char,
                               n: lapack_int, nrhs: lapack_int, a: *const lapack_complex_float,
                               lda: lapack_int, b: *mut lapack_complex_float, ldb: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_ztrtrs_work(matrix_layout: c_int, uplo: c_char, trans: c_char, diag: c_char,
                               n: lapack_int, nrhs: lapack_int, a: *const lapack_complex_double,
                               lda: lapack_int, b: *mut lapack_complex_double, ldb: lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_strttf_work(matrix_layout: c_int, transr: c_char, uplo: c_char, n: lapack_int,
                               a: *const c_float, lda: lapack_int, arf: *mut c_float)
                               -> lapack_int;
    pub fn LAPACKE_dtrttf_work(matrix_layout: c_int, transr: c_char, uplo: c_char, n: lapack_int,
                               a: *const c_double, lda: lapack_int, arf: *mut c_double)
                               -> lapack_int;
    pub fn LAPACKE_ctrttf_work(matrix_layout: c_int, transr: c_char, uplo: c_char, n: lapack_int,
                               a: *const lapack_complex_float, lda: lapack_int,
                               arf: *mut lapack_complex_float)
                               -> lapack_int;
    pub fn LAPACKE_ztrttf_work(matrix_layout: c_int, transr: c_char, uplo: c_char, n: lapack_int,
                               a: *const lapack_complex_double, lda: lapack_int,
                               arf: *mut lapack_complex_double)
                               -> lapack_int;

    pub fn LAPACKE_strttp_work(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                               a: *const c_float, lda: lapack_int, ap: *mut c_float)
                               -> lapack_int;
    pub fn LAPACKE_dtrttp_work(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                               a: *const c_double, lda: lapack_int, ap: *mut c_double)
                               -> lapack_int;
    pub fn LAPACKE_ctrttp_work(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                               a: *const lapack_complex_float, lda: lapack_int,
                               ap: *mut lapack_complex_float)
                               -> lapack_int;
    pub fn LAPACKE_ztrttp_work(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                               a: *const lapack_complex_double, lda: lapack_int,
                               ap: *mut lapack_complex_double)
                               -> lapack_int;

    pub fn LAPACKE_stzrzf_work(matrix_layout: c_int, m: lapack_int, n: lapack_int, a: *mut c_float,
                               lda: lapack_int, tau: *mut c_float, work: *mut c_float,
                               lwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dtzrzf_work(matrix_layout: c_int, m: lapack_int, n: lapack_int,
                               a: *mut c_double, lda: lapack_int, tau: *mut c_double,
                               work: *mut c_double, lwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_ctzrzf_work(matrix_layout: c_int, m: lapack_int, n: lapack_int,
                               a: *mut lapack_complex_float, lda: lapack_int,
                               tau: *mut lapack_complex_float, work: *mut lapack_complex_float,
                               lwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_ztzrzf_work(matrix_layout: c_int, m: lapack_int, n: lapack_int,
                               a: *mut lapack_complex_double, lda: lapack_int,
                               tau: *mut lapack_complex_double, work: *mut lapack_complex_double,
                               lwork: lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_cungbr_work(matrix_layout: c_int, vect: c_char, m: lapack_int, n: lapack_int,
                               k: lapack_int, a: *mut lapack_complex_float, lda: lapack_int,
                               tau: *const lapack_complex_float, work: *mut lapack_complex_float,
                               lwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_zungbr_work(matrix_layout: c_int, vect: c_char, m: lapack_int, n: lapack_int,
                               k: lapack_int, a: *mut lapack_complex_double, lda: lapack_int,
                               tau: *const lapack_complex_double, work: *mut lapack_complex_double,
                               lwork: lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_cunghr_work(matrix_layout: c_int, n: lapack_int, ilo: lapack_int,
                               ihi: lapack_int, a: *mut lapack_complex_float, lda: lapack_int,
                               tau: *const lapack_complex_float, work: *mut lapack_complex_float,
                               lwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_zunghr_work(matrix_layout: c_int, n: lapack_int, ilo: lapack_int,
                               ihi: lapack_int, a: *mut lapack_complex_double, lda: lapack_int,
                               tau: *const lapack_complex_double, work: *mut lapack_complex_double,
                               lwork: lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_cunglq_work(matrix_layout: c_int, m: lapack_int, n: lapack_int, k: lapack_int,
                               a: *mut lapack_complex_float, lda: lapack_int,
                               tau: *const lapack_complex_float, work: *mut lapack_complex_float,
                               lwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_zunglq_work(matrix_layout: c_int, m: lapack_int, n: lapack_int, k: lapack_int,
                               a: *mut lapack_complex_double, lda: lapack_int,
                               tau: *const lapack_complex_double, work: *mut lapack_complex_double,
                               lwork: lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_cungql_work(matrix_layout: c_int, m: lapack_int, n: lapack_int, k: lapack_int,
                               a: *mut lapack_complex_float, lda: lapack_int,
                               tau: *const lapack_complex_float, work: *mut lapack_complex_float,
                               lwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_zungql_work(matrix_layout: c_int, m: lapack_int, n: lapack_int, k: lapack_int,
                               a: *mut lapack_complex_double, lda: lapack_int,
                               tau: *const lapack_complex_double, work: *mut lapack_complex_double,
                               lwork: lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_cungqr_work(matrix_layout: c_int, m: lapack_int, n: lapack_int, k: lapack_int,
                               a: *mut lapack_complex_float, lda: lapack_int,
                               tau: *const lapack_complex_float, work: *mut lapack_complex_float,
                               lwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_zungqr_work(matrix_layout: c_int, m: lapack_int, n: lapack_int, k: lapack_int,
                               a: *mut lapack_complex_double, lda: lapack_int,
                               tau: *const lapack_complex_double, work: *mut lapack_complex_double,
                               lwork: lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_cungrq_work(matrix_layout: c_int, m: lapack_int, n: lapack_int, k: lapack_int,
                               a: *mut lapack_complex_float, lda: lapack_int,
                               tau: *const lapack_complex_float, work: *mut lapack_complex_float,
                               lwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_zungrq_work(matrix_layout: c_int, m: lapack_int, n: lapack_int, k: lapack_int,
                               a: *mut lapack_complex_double, lda: lapack_int,
                               tau: *const lapack_complex_double, work: *mut lapack_complex_double,
                               lwork: lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_cungtr_work(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                               a: *mut lapack_complex_float, lda: lapack_int,
                               tau: *const lapack_complex_float, work: *mut lapack_complex_float,
                               lwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_zungtr_work(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                               a: *mut lapack_complex_double, lda: lapack_int,
                               tau: *const lapack_complex_double, work: *mut lapack_complex_double,
                               lwork: lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_cunmbr_work(matrix_layout: c_int, vect: c_char, side: c_char, trans: c_char,
                               m: lapack_int, n: lapack_int, k: lapack_int,
                               a: *const lapack_complex_float, lda: lapack_int,
                               tau: *const lapack_complex_float, c: *mut lapack_complex_float,
                               ldc: lapack_int, work: *mut lapack_complex_float, lwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_zunmbr_work(matrix_layout: c_int, vect: c_char, side: c_char, trans: c_char,
                               m: lapack_int, n: lapack_int, k: lapack_int,
                               a: *const lapack_complex_double, lda: lapack_int,
                               tau: *const lapack_complex_double, c: *mut lapack_complex_double,
                               ldc: lapack_int, work: *mut lapack_complex_double,
                               lwork: lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_cunmhr_work(matrix_layout: c_int, side: c_char, trans: c_char, m: lapack_int,
                               n: lapack_int, ilo: lapack_int, ihi: lapack_int,
                               a: *const lapack_complex_float, lda: lapack_int,
                               tau: *const lapack_complex_float, c: *mut lapack_complex_float,
                               ldc: lapack_int, work: *mut lapack_complex_float, lwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_zunmhr_work(matrix_layout: c_int, side: c_char, trans: c_char, m: lapack_int,
                               n: lapack_int, ilo: lapack_int, ihi: lapack_int,
                               a: *const lapack_complex_double, lda: lapack_int,
                               tau: *const lapack_complex_double, c: *mut lapack_complex_double,
                               ldc: lapack_int, work: *mut lapack_complex_double,
                               lwork: lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_cunmlq_work(matrix_layout: c_int, side: c_char, trans: c_char, m: lapack_int,
                               n: lapack_int, k: lapack_int, a: *const lapack_complex_float,
                               lda: lapack_int, tau: *const lapack_complex_float,
                               c: *mut lapack_complex_float, ldc: lapack_int,
                               work: *mut lapack_complex_float, lwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_zunmlq_work(matrix_layout: c_int, side: c_char, trans: c_char, m: lapack_int,
                               n: lapack_int, k: lapack_int, a: *const lapack_complex_double,
                               lda: lapack_int, tau: *const lapack_complex_double,
                               c: *mut lapack_complex_double, ldc: lapack_int,
                               work: *mut lapack_complex_double, lwork: lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_cunmql_work(matrix_layout: c_int, side: c_char, trans: c_char, m: lapack_int,
                               n: lapack_int, k: lapack_int, a: *const lapack_complex_float,
                               lda: lapack_int, tau: *const lapack_complex_float,
                               c: *mut lapack_complex_float, ldc: lapack_int,
                               work: *mut lapack_complex_float, lwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_zunmql_work(matrix_layout: c_int, side: c_char, trans: c_char, m: lapack_int,
                               n: lapack_int, k: lapack_int, a: *const lapack_complex_double,
                               lda: lapack_int, tau: *const lapack_complex_double,
                               c: *mut lapack_complex_double, ldc: lapack_int,
                               work: *mut lapack_complex_double, lwork: lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_cunmqr_work(matrix_layout: c_int, side: c_char, trans: c_char, m: lapack_int,
                               n: lapack_int, k: lapack_int, a: *const lapack_complex_float,
                               lda: lapack_int, tau: *const lapack_complex_float,
                               c: *mut lapack_complex_float, ldc: lapack_int,
                               work: *mut lapack_complex_float, lwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_zunmqr_work(matrix_layout: c_int, side: c_char, trans: c_char, m: lapack_int,
                               n: lapack_int, k: lapack_int, a: *const lapack_complex_double,
                               lda: lapack_int, tau: *const lapack_complex_double,
                               c: *mut lapack_complex_double, ldc: lapack_int,
                               work: *mut lapack_complex_double, lwork: lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_cunmrq_work(matrix_layout: c_int, side: c_char, trans: c_char, m: lapack_int,
                               n: lapack_int, k: lapack_int, a: *const lapack_complex_float,
                               lda: lapack_int, tau: *const lapack_complex_float,
                               c: *mut lapack_complex_float, ldc: lapack_int,
                               work: *mut lapack_complex_float, lwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_zunmrq_work(matrix_layout: c_int, side: c_char, trans: c_char, m: lapack_int,
                               n: lapack_int, k: lapack_int, a: *const lapack_complex_double,
                               lda: lapack_int, tau: *const lapack_complex_double,
                               c: *mut lapack_complex_double, ldc: lapack_int,
                               work: *mut lapack_complex_double, lwork: lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_cunmrz_work(matrix_layout: c_int, side: c_char, trans: c_char, m: lapack_int,
                               n: lapack_int, k: lapack_int, l: lapack_int,
                               a: *const lapack_complex_float, lda: lapack_int,
                               tau: *const lapack_complex_float, c: *mut lapack_complex_float,
                               ldc: lapack_int, work: *mut lapack_complex_float, lwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_zunmrz_work(matrix_layout: c_int, side: c_char, trans: c_char, m: lapack_int,
                               n: lapack_int, k: lapack_int, l: lapack_int,
                               a: *const lapack_complex_double, lda: lapack_int,
                               tau: *const lapack_complex_double, c: *mut lapack_complex_double,
                               ldc: lapack_int, work: *mut lapack_complex_double,
                               lwork: lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_cunmtr_work(matrix_layout: c_int, side: c_char, uplo: c_char, trans: c_char,
                               m: lapack_int, n: lapack_int, a: *const lapack_complex_float,
                               lda: lapack_int, tau: *const lapack_complex_float,
                               c: *mut lapack_complex_float, ldc: lapack_int,
                               work: *mut lapack_complex_float, lwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_zunmtr_work(matrix_layout: c_int, side: c_char, uplo: c_char, trans: c_char,
                               m: lapack_int, n: lapack_int, a: *const lapack_complex_double,
                               lda: lapack_int, tau: *const lapack_complex_double,
                               c: *mut lapack_complex_double, ldc: lapack_int,
                               work: *mut lapack_complex_double, lwork: lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_cupgtr_work(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                               ap: *const lapack_complex_float, tau: *const lapack_complex_float,
                               q: *mut lapack_complex_float, ldq: lapack_int,
                               work: *mut lapack_complex_float)
                               -> lapack_int;
    pub fn LAPACKE_zupgtr_work(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                               ap: *const lapack_complex_double, tau: *const lapack_complex_double,
                               q: *mut lapack_complex_double, ldq: lapack_int,
                               work: *mut lapack_complex_double)
                               -> lapack_int;

    pub fn LAPACKE_cupmtr_work(matrix_layout: c_int, side: c_char, uplo: c_char, trans: c_char,
                               m: lapack_int, n: lapack_int, ap: *const lapack_complex_float,
                               tau: *const lapack_complex_float, c: *mut lapack_complex_float,
                               ldc: lapack_int, work: *mut lapack_complex_float)
                               -> lapack_int;
    pub fn LAPACKE_zupmtr_work(matrix_layout: c_int, side: c_char, uplo: c_char, trans: c_char,
                               m: lapack_int, n: lapack_int, ap: *const lapack_complex_double,
                               tau: *const lapack_complex_double, c: *mut lapack_complex_double,
                               ldc: lapack_int, work: *mut lapack_complex_double)
                               -> lapack_int;

    pub fn LAPACKE_claghe(matrix_layout: c_int, n: lapack_int, k: lapack_int, d: *const c_float,
                          a: *mut lapack_complex_float, lda: lapack_int, iseed: *mut lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_zlaghe(matrix_layout: c_int, n: lapack_int, k: lapack_int, d: *const c_double,
                          a: *mut lapack_complex_double, lda: lapack_int, iseed: *mut lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_slagsy(matrix_layout: c_int, n: lapack_int, k: lapack_int, d: *const c_float,
                          a: *mut c_float, lda: lapack_int, iseed: *mut lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_dlagsy(matrix_layout: c_int, n: lapack_int, k: lapack_int, d: *const c_double,
                          a: *mut c_double, lda: lapack_int, iseed: *mut lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_clagsy(matrix_layout: c_int, n: lapack_int, k: lapack_int, d: *const c_float,
                          a: *mut lapack_complex_float, lda: lapack_int, iseed: *mut lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_zlagsy(matrix_layout: c_int, n: lapack_int, k: lapack_int, d: *const c_double,
                          a: *mut lapack_complex_double, lda: lapack_int, iseed: *mut lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_slapmr(matrix_layout: c_int, forwrd: lapack_logical, m: lapack_int,
                          n: lapack_int, x: *mut c_float, ldx: lapack_int, k: *mut lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_dlapmr(matrix_layout: c_int, forwrd: lapack_logical, m: lapack_int,
                          n: lapack_int, x: *mut c_double, ldx: lapack_int, k: *mut lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_clapmr(matrix_layout: c_int, forwrd: lapack_logical, m: lapack_int,
                          n: lapack_int, x: *mut lapack_complex_float, ldx: lapack_int,
                          k: *mut lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_zlapmr(matrix_layout: c_int, forwrd: lapack_logical, m: lapack_int,
                          n: lapack_int, x: *mut lapack_complex_double, ldx: lapack_int,
                          k: *mut lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_slapmt(matrix_layout: c_int, forwrd: lapack_logical, m: lapack_int,
                          n: lapack_int, x: *mut c_float, ldx: lapack_int, k: *mut lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_dlapmt(matrix_layout: c_int, forwrd: lapack_logical, m: lapack_int,
                          n: lapack_int, x: *mut c_double, ldx: lapack_int, k: *mut lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_clapmt(matrix_layout: c_int, forwrd: lapack_logical, m: lapack_int,
                          n: lapack_int, x: *mut lapack_complex_float, ldx: lapack_int,
                          k: *mut lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_zlapmt(matrix_layout: c_int, forwrd: lapack_logical, m: lapack_int,
                          n: lapack_int, x: *mut lapack_complex_double, ldx: lapack_int,
                          k: *mut lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_slapy2(x: c_float, y: c_float) -> c_float;
    pub fn LAPACKE_dlapy2(x: c_double, y: c_double) -> c_double;

    pub fn LAPACKE_slapy3(x: c_float, y: c_float, z: c_float) -> c_float;
    pub fn LAPACKE_dlapy3(x: c_double, y: c_double, z: c_double) -> c_double;

    pub fn LAPACKE_slartgp(f: c_float, g: c_float, cs: *mut c_float, sn: *mut c_float,
                           r: *mut c_float)
                           -> lapack_int;
    pub fn LAPACKE_dlartgp(f: c_double, g: c_double, cs: *mut c_double, sn: *mut c_double,
                           r: *mut c_double)
                           -> lapack_int;

    pub fn LAPACKE_slartgs(x: c_float, y: c_float, sigma: c_float, cs: *mut c_float,
                           sn: *mut c_float)
                           -> lapack_int;
    pub fn LAPACKE_dlartgs(x: c_double, y: c_double, sigma: c_double, cs: *mut c_double,
                           sn: *mut c_double)
                           -> lapack_int;

    // Version 3.3.0
    pub fn LAPACKE_cbbcsd(matrix_layout: c_int, jobu1: c_char, jobu2: c_char, jobv1t: c_char,
                          jobv2t: c_char, trans: c_char, m: lapack_int, p: lapack_int,
                          q: lapack_int, theta: *mut c_float, phi: *mut c_float,
                          u1: *mut lapack_complex_float, ldu1: lapack_int,
                          u2: *mut lapack_complex_float, ldu2: lapack_int,
                          v1t: *mut lapack_complex_float, ldv1t: lapack_int,
                          v2t: *mut lapack_complex_float, ldv2t: lapack_int, b11d: *mut c_float,
                          b11e: *mut c_float, b12d: *mut c_float, b12e: *mut c_float,
                          b21d: *mut c_float, b21e: *mut c_float, b22d: *mut c_float,
                          b22e: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_cbbcsd_work(matrix_layout: c_int, jobu1: c_char, jobu2: c_char, jobv1t: c_char,
                               jobv2t: c_char, trans: c_char, m: lapack_int, p: lapack_int,
                               q: lapack_int, theta: *mut c_float, phi: *mut c_float,
                               u1: *mut lapack_complex_float, ldu1: lapack_int,
                               u2: *mut lapack_complex_float, ldu2: lapack_int,
                               v1t: *mut lapack_complex_float, ldv1t: lapack_int,
                               v2t: *mut lapack_complex_float, ldv2t: lapack_int,
                               b11d: *mut c_float, b11e: *mut c_float, b12d: *mut c_float,
                               b12e: *mut c_float, b21d: *mut c_float, b21e: *mut c_float,
                               b22d: *mut c_float, b22e: *mut c_float, rwork: *mut c_float,
                               lrwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_cheswapr(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                            a: *mut lapack_complex_float, i1: lapack_int, i2: lapack_int)
                            -> lapack_int;
    pub fn LAPACKE_cheswapr_work(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                                 a: *mut lapack_complex_float, i1: lapack_int, i2: lapack_int)
                                 -> lapack_int;
    pub fn LAPACKE_chetri2(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                           a: *mut lapack_complex_float, lda: lapack_int, ipiv: *const lapack_int)
                           -> lapack_int;
    pub fn LAPACKE_chetri2_work(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                                a: *mut lapack_complex_float, lda: lapack_int,
                                ipiv: *const lapack_int, work: *mut lapack_complex_float,
                                lwork: lapack_int)
                                -> lapack_int;
    pub fn LAPACKE_chetri2x(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                            a: *mut lapack_complex_float, lda: lapack_int, ipiv: *const lapack_int,
                            nb: lapack_int)
                            -> lapack_int;
    pub fn LAPACKE_chetri2x_work(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                                 a: *mut lapack_complex_float, lda: lapack_int,
                                 ipiv: *const lapack_int, work: *mut lapack_complex_float,
                                 nb: lapack_int)
                                 -> lapack_int;
    pub fn LAPACKE_chetrs2(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                           a: *const lapack_complex_float, lda: lapack_int,
                           ipiv: *const lapack_int, b: *mut lapack_complex_float, ldb: lapack_int)
                           -> lapack_int;
    pub fn LAPACKE_chetrs2_work(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                                nrhs: lapack_int, a: *const lapack_complex_float, lda: lapack_int,
                                ipiv: *const lapack_int, b: *mut lapack_complex_float,
                                ldb: lapack_int, work: *mut lapack_complex_float)
                                -> lapack_int;
    pub fn LAPACKE_csyconv(matrix_layout: c_int, uplo: c_char, way: c_char, n: lapack_int,
                           a: *mut lapack_complex_float, lda: lapack_int, ipiv: *const lapack_int,
                           work: *mut lapack_complex_float)
                           -> lapack_int;
    pub fn LAPACKE_csyconv_work(matrix_layout: c_int, uplo: c_char, way: c_char, n: lapack_int,
                                a: *mut lapack_complex_float, lda: lapack_int,
                                ipiv: *const lapack_int, work: *mut lapack_complex_float)
                                -> lapack_int;
    pub fn LAPACKE_csyswapr(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                            a: *mut lapack_complex_float, i1: lapack_int, i2: lapack_int)
                            -> lapack_int;
    pub fn LAPACKE_csyswapr_work(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                                 a: *mut lapack_complex_float, i1: lapack_int, i2: lapack_int)
                                 -> lapack_int;
    pub fn LAPACKE_csytri2(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                           a: *mut lapack_complex_float, lda: lapack_int, ipiv: *const lapack_int)
                           -> lapack_int;
    pub fn LAPACKE_csytri2_work(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                                a: *mut lapack_complex_float, lda: lapack_int,
                                ipiv: *const lapack_int, work: *mut lapack_complex_float,
                                lwork: lapack_int)
                                -> lapack_int;
    pub fn LAPACKE_csytri2x(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                            a: *mut lapack_complex_float, lda: lapack_int, ipiv: *const lapack_int,
                            nb: lapack_int)
                            -> lapack_int;
    pub fn LAPACKE_csytri2x_work(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                                 a: *mut lapack_complex_float, lda: lapack_int,
                                 ipiv: *const lapack_int, work: *mut lapack_complex_float,
                                 nb: lapack_int)
                                 -> lapack_int;
    pub fn LAPACKE_csytrs2(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                           a: *const lapack_complex_float, lda: lapack_int,
                           ipiv: *const lapack_int, b: *mut lapack_complex_float, ldb: lapack_int)
                           -> lapack_int;
    pub fn LAPACKE_csytrs2_work(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                                nrhs: lapack_int, a: *const lapack_complex_float, lda: lapack_int,
                                ipiv: *const lapack_int, b: *mut lapack_complex_float,
                                ldb: lapack_int, work: *mut lapack_complex_float)
                                -> lapack_int;
    pub fn LAPACKE_cunbdb(matrix_layout: c_int, trans: c_char, signs: c_char, m: lapack_int,
                          p: lapack_int, q: lapack_int, x11: *mut lapack_complex_float,
                          ldx11: lapack_int, x12: *mut lapack_complex_float, ldx12: lapack_int,
                          x21: *mut lapack_complex_float, ldx21: lapack_int,
                          x22: *mut lapack_complex_float, ldx22: lapack_int, theta: *mut c_float,
                          phi: *mut c_float, taup1: *mut lapack_complex_float,
                          taup2: *mut lapack_complex_float, tauq1: *mut lapack_complex_float,
                          tauq2: *mut lapack_complex_float)
                          -> lapack_int;
    pub fn LAPACKE_cunbdb_work(matrix_layout: c_int, trans: c_char, signs: c_char, m: lapack_int,
                               p: lapack_int, q: lapack_int, x11: *mut lapack_complex_float,
                               ldx11: lapack_int, x12: *mut lapack_complex_float,
                               ldx12: lapack_int, x21: *mut lapack_complex_float,
                               ldx21: lapack_int, x22: *mut lapack_complex_float,
                               ldx22: lapack_int, theta: *mut c_float, phi: *mut c_float,
                               taup1: *mut lapack_complex_float, taup2: *mut lapack_complex_float,
                               tauq1: *mut lapack_complex_float, tauq2: *mut lapack_complex_float,
                               work: *mut lapack_complex_float, lwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_cuncsd(matrix_layout: c_int, jobu1: c_char, jobu2: c_char, jobv1t: c_char,
                          jobv2t: c_char, trans: c_char, signs: c_char, m: lapack_int,
                          p: lapack_int, q: lapack_int, x11: *mut lapack_complex_float,
                          ldx11: lapack_int, x12: *mut lapack_complex_float, ldx12: lapack_int,
                          x21: *mut lapack_complex_float, ldx21: lapack_int,
                          x22: *mut lapack_complex_float, ldx22: lapack_int, theta: *mut c_float,
                          u1: *mut lapack_complex_float, ldu1: lapack_int,
                          u2: *mut lapack_complex_float, ldu2: lapack_int,
                          v1t: *mut lapack_complex_float, ldv1t: lapack_int,
                          v2t: *mut lapack_complex_float, ldv2t: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_cuncsd_work(matrix_layout: c_int, jobu1: c_char, jobu2: c_char, jobv1t: c_char,
                               jobv2t: c_char, trans: c_char, signs: c_char, m: lapack_int,
                               p: lapack_int, q: lapack_int, x11: *mut lapack_complex_float,
                               ldx11: lapack_int, x12: *mut lapack_complex_float,
                               ldx12: lapack_int, x21: *mut lapack_complex_float,
                               ldx21: lapack_int, x22: *mut lapack_complex_float,
                               ldx22: lapack_int, theta: *mut c_float,
                               u1: *mut lapack_complex_float, ldu1: lapack_int,
                               u2: *mut lapack_complex_float, ldu2: lapack_int,
                               v1t: *mut lapack_complex_float, ldv1t: lapack_int,
                               v2t: *mut lapack_complex_float, ldv2t: lapack_int,
                               work: *mut lapack_complex_float, lwork: lapack_int,
                               rwork: *mut c_float, lrwork: lapack_int, iwork: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_cuncsd2by1(matrix_layout: c_int, jobu1: c_char, jobu2: c_char, jobv1t: c_char,
                              m: lapack_int, p: lapack_int, q: lapack_int,
                              x11: *mut lapack_complex_float, ldx11: lapack_int,
                              x21: *mut lapack_complex_float, ldx21: lapack_int,
                              theta: *mut lapack_complex_float, u1: *mut lapack_complex_float,
                              ldu1: lapack_int, u2: *mut lapack_complex_float, ldu2: lapack_int,
                              v1t: *mut lapack_complex_float, ldv1t: lapack_int)
                              -> lapack_int;
    pub fn LAPACKE_cuncsd2by1_work(matrix_layout: c_int, jobu1: c_char, jobu2: c_char,
                                   jobv1t: c_char, m: lapack_int, p: lapack_int, q: lapack_int,
                                   x11: *mut lapack_complex_float, ldx11: lapack_int,
                                   x21: *mut lapack_complex_float, ldx21: lapack_int,
                                   theta: *mut lapack_complex_float, u1: *mut lapack_complex_float,
                                   ldu1: lapack_int, u2: *mut lapack_complex_float,
                                   ldu2: lapack_int, v1t: *mut lapack_complex_float,
                                   ldv1t: lapack_int, work: *mut lapack_complex_float,
                                   lwork: lapack_int, rwork: *mut c_float, lrwork: lapack_int,
                                   iwork: *mut lapack_int)
                                   -> lapack_int;
    pub fn LAPACKE_dbbcsd(matrix_layout: c_int, jobu1: c_char, jobu2: c_char, jobv1t: c_char,
                          jobv2t: c_char, trans: c_char, m: lapack_int, p: lapack_int,
                          q: lapack_int, theta: *mut c_double, phi: *mut c_double,
                          u1: *mut c_double, ldu1: lapack_int, u2: *mut c_double, ldu2: lapack_int,
                          v1t: *mut c_double, ldv1t: lapack_int, v2t: *mut c_double,
                          ldv2t: lapack_int, b11d: *mut c_double, b11e: *mut c_double,
                          b12d: *mut c_double, b12e: *mut c_double, b21d: *mut c_double,
                          b21e: *mut c_double, b22d: *mut c_double, b22e: *mut c_double)
                          -> lapack_int;
    pub fn LAPACKE_dbbcsd_work(matrix_layout: c_int, jobu1: c_char, jobu2: c_char, jobv1t: c_char,
                               jobv2t: c_char, trans: c_char, m: lapack_int, p: lapack_int,
                               q: lapack_int, theta: *mut c_double, phi: *mut c_double,
                               u1: *mut c_double, ldu1: lapack_int, u2: *mut c_double,
                               ldu2: lapack_int, v1t: *mut c_double, ldv1t: lapack_int,
                               v2t: *mut c_double, ldv2t: lapack_int, b11d: *mut c_double,
                               b11e: *mut c_double, b12d: *mut c_double, b12e: *mut c_double,
                               b21d: *mut c_double, b21e: *mut c_double, b22d: *mut c_double,
                               b22e: *mut c_double, work: *mut c_double, lwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dorbdb(matrix_layout: c_int, trans: c_char, signs: c_char, m: lapack_int,
                          p: lapack_int, q: lapack_int, x11: *mut c_double, ldx11: lapack_int,
                          x12: *mut c_double, ldx12: lapack_int, x21: *mut c_double,
                          ldx21: lapack_int, x22: *mut c_double, ldx22: lapack_int,
                          theta: *mut c_double, phi: *mut c_double, taup1: *mut c_double,
                          taup2: *mut c_double, tauq1: *mut c_double, tauq2: *mut c_double)
                          -> lapack_int;
    pub fn LAPACKE_dorbdb_work(matrix_layout: c_int, trans: c_char, signs: c_char, m: lapack_int,
                               p: lapack_int, q: lapack_int, x11: *mut c_double, ldx11: lapack_int,
                               x12: *mut c_double, ldx12: lapack_int, x21: *mut c_double,
                               ldx21: lapack_int, x22: *mut c_double, ldx22: lapack_int,
                               theta: *mut c_double, phi: *mut c_double, taup1: *mut c_double,
                               taup2: *mut c_double, tauq1: *mut c_double, tauq2: *mut c_double,
                               work: *mut c_double, lwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dorcsd(matrix_layout: c_int, jobu1: c_char, jobu2: c_char, jobv1t: c_char,
                          jobv2t: c_char, trans: c_char, signs: c_char, m: lapack_int,
                          p: lapack_int, q: lapack_int, x11: *mut c_double, ldx11: lapack_int,
                          x12: *mut c_double, ldx12: lapack_int, x21: *mut c_double,
                          ldx21: lapack_int, x22: *mut c_double, ldx22: lapack_int,
                          theta: *mut c_double, u1: *mut c_double, ldu1: lapack_int,
                          u2: *mut c_double, ldu2: lapack_int, v1t: *mut c_double,
                          ldv1t: lapack_int, v2t: *mut c_double, ldv2t: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_dorcsd_work(matrix_layout: c_int, jobu1: c_char, jobu2: c_char, jobv1t: c_char,
                               jobv2t: c_char, trans: c_char, signs: c_char, m: lapack_int,
                               p: lapack_int, q: lapack_int, x11: *mut c_double, ldx11: lapack_int,
                               x12: *mut c_double, ldx12: lapack_int, x21: *mut c_double,
                               ldx21: lapack_int, x22: *mut c_double, ldx22: lapack_int,
                               theta: *mut c_double, u1: *mut c_double, ldu1: lapack_int,
                               u2: *mut c_double, ldu2: lapack_int, v1t: *mut c_double,
                               ldv1t: lapack_int, v2t: *mut c_double, ldv2t: lapack_int,
                               work: *mut c_double, lwork: lapack_int, iwork: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dorcsd2by1(matrix_layout: c_int, jobu1: c_char, jobu2: c_char, jobv1t: c_char,
                              m: lapack_int, p: lapack_int, q: lapack_int, x11: *mut c_double,
                              ldx11: lapack_int, x21: *mut c_double, ldx21: lapack_int,
                              theta: *mut c_double, u1: *mut c_double, ldu1: lapack_int,
                              u2: *mut c_double, ldu2: lapack_int, v1t: *mut c_double,
                              ldv1t: lapack_int)
                              -> lapack_int;
    pub fn LAPACKE_dorcsd2by1_work(matrix_layout: c_int, jobu1: c_char, jobu2: c_char,
                                   jobv1t: c_char, m: lapack_int, p: lapack_int, q: lapack_int,
                                   x11: *mut c_double, ldx11: lapack_int, x21: *mut c_double,
                                   ldx21: lapack_int, theta: *mut c_double, u1: *mut c_double,
                                   ldu1: lapack_int, u2: *mut c_double, ldu2: lapack_int,
                                   v1t: *mut c_double, ldv1t: lapack_int, work: *mut c_double,
                                   lwork: lapack_int, iwork: *mut lapack_int)
                                   -> lapack_int;
    pub fn LAPACKE_dsyconv(matrix_layout: c_int, uplo: c_char, way: c_char, n: lapack_int,
                           a: *mut c_double, lda: lapack_int, ipiv: *const lapack_int,
                           work: *mut c_double)
                           -> lapack_int;
    pub fn LAPACKE_dsyconv_work(matrix_layout: c_int, uplo: c_char, way: c_char, n: lapack_int,
                                a: *mut c_double, lda: lapack_int, ipiv: *const lapack_int,
                                work: *mut c_double)
                                -> lapack_int;
    pub fn LAPACKE_dsyswapr(matrix_layout: c_int, uplo: c_char, n: lapack_int, a: *mut c_double,
                            i1: lapack_int, i2: lapack_int)
                            -> lapack_int;
    pub fn LAPACKE_dsyswapr_work(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                                 a: *mut c_double, i1: lapack_int, i2: lapack_int)
                                 -> lapack_int;
    pub fn LAPACKE_dsytri2(matrix_layout: c_int, uplo: c_char, n: lapack_int, a: *mut c_double,
                           lda: lapack_int, ipiv: *const lapack_int)
                           -> lapack_int;
    pub fn LAPACKE_dsytri2_work(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                                a: *mut c_double, lda: lapack_int, ipiv: *const lapack_int,
                                work: *mut lapack_complex_double, lwork: lapack_int)
                                -> lapack_int;
    pub fn LAPACKE_dsytri2x(matrix_layout: c_int, uplo: c_char, n: lapack_int, a: *mut c_double,
                            lda: lapack_int, ipiv: *const lapack_int, nb: lapack_int)
                            -> lapack_int;
    pub fn LAPACKE_dsytri2x_work(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                                 a: *mut c_double, lda: lapack_int, ipiv: *const lapack_int,
                                 work: *mut c_double, nb: lapack_int)
                                 -> lapack_int;
    pub fn LAPACKE_dsytrs2(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                           a: *const c_double, lda: lapack_int, ipiv: *const lapack_int,
                           b: *mut c_double, ldb: lapack_int)
                           -> lapack_int;
    pub fn LAPACKE_dsytrs2_work(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                                nrhs: lapack_int, a: *const c_double, lda: lapack_int,
                                ipiv: *const lapack_int, b: *mut c_double, ldb: lapack_int,
                                work: *mut c_double)
                                -> lapack_int;
    pub fn LAPACKE_sbbcsd(matrix_layout: c_int, jobu1: c_char, jobu2: c_char, jobv1t: c_char,
                          jobv2t: c_char, trans: c_char, m: lapack_int, p: lapack_int,
                          q: lapack_int, theta: *mut c_float, phi: *mut c_float, u1: *mut c_float,
                          ldu1: lapack_int, u2: *mut c_float, ldu2: lapack_int, v1t: *mut c_float,
                          ldv1t: lapack_int, v2t: *mut c_float, ldv2t: lapack_int,
                          b11d: *mut c_float, b11e: *mut c_float, b12d: *mut c_float,
                          b12e: *mut c_float, b21d: *mut c_float, b21e: *mut c_float,
                          b22d: *mut c_float, b22e: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_sbbcsd_work(matrix_layout: c_int, jobu1: c_char, jobu2: c_char, jobv1t: c_char,
                               jobv2t: c_char, trans: c_char, m: lapack_int, p: lapack_int,
                               q: lapack_int, theta: *mut c_float, phi: *mut c_float,
                               u1: *mut c_float, ldu1: lapack_int, u2: *mut c_float,
                               ldu2: lapack_int, v1t: *mut c_float, ldv1t: lapack_int,
                               v2t: *mut c_float, ldv2t: lapack_int, b11d: *mut c_float,
                               b11e: *mut c_float, b12d: *mut c_float, b12e: *mut c_float,
                               b21d: *mut c_float, b21e: *mut c_float, b22d: *mut c_float,
                               b22e: *mut c_float, work: *mut c_float, lwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_sorbdb(matrix_layout: c_int, trans: c_char, signs: c_char, m: lapack_int,
                          p: lapack_int, q: lapack_int, x11: *mut c_float, ldx11: lapack_int,
                          x12: *mut c_float, ldx12: lapack_int, x21: *mut c_float,
                          ldx21: lapack_int, x22: *mut c_float, ldx22: lapack_int,
                          theta: *mut c_float, phi: *mut c_float, taup1: *mut c_float,
                          taup2: *mut c_float, tauq1: *mut c_float, tauq2: *mut c_float)
                          -> lapack_int;
    pub fn LAPACKE_sorbdb_work(matrix_layout: c_int, trans: c_char, signs: c_char, m: lapack_int,
                               p: lapack_int, q: lapack_int, x11: *mut c_float, ldx11: lapack_int,
                               x12: *mut c_float, ldx12: lapack_int, x21: *mut c_float,
                               ldx21: lapack_int, x22: *mut c_float, ldx22: lapack_int,
                               theta: *mut c_float, phi: *mut c_float, taup1: *mut c_float,
                               taup2: *mut c_float, tauq1: *mut c_float, tauq2: *mut c_float,
                               work: *mut c_float, lwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_sorcsd(matrix_layout: c_int, jobu1: c_char, jobu2: c_char, jobv1t: c_char,
                          jobv2t: c_char, trans: c_char, signs: c_char, m: lapack_int,
                          p: lapack_int, q: lapack_int, x11: *mut c_float, ldx11: lapack_int,
                          x12: *mut c_float, ldx12: lapack_int, x21: *mut c_float,
                          ldx21: lapack_int, x22: *mut c_float, ldx22: lapack_int,
                          theta: *mut c_float, u1: *mut c_float, ldu1: lapack_int,
                          u2: *mut c_float, ldu2: lapack_int, v1t: *mut c_float, ldv1t: lapack_int,
                          v2t: *mut c_float, ldv2t: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_sorcsd_work(matrix_layout: c_int, jobu1: c_char, jobu2: c_char, jobv1t: c_char,
                               jobv2t: c_char, trans: c_char, signs: c_char, m: lapack_int,
                               p: lapack_int, q: lapack_int, x11: *mut c_float, ldx11: lapack_int,
                               x12: *mut c_float, ldx12: lapack_int, x21: *mut c_float,
                               ldx21: lapack_int, x22: *mut c_float, ldx22: lapack_int,
                               theta: *mut c_float, u1: *mut c_float, ldu1: lapack_int,
                               u2: *mut c_float, ldu2: lapack_int, v1t: *mut c_float,
                               ldv1t: lapack_int, v2t: *mut c_float, ldv2t: lapack_int,
                               work: *mut c_float, lwork: lapack_int, iwork: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_sorcsd2by1(matrix_layout: c_int, jobu1: c_char, jobu2: c_char, jobv1t: c_char,
                              m: lapack_int, p: lapack_int, q: lapack_int, x11: *mut c_float,
                              ldx11: lapack_int, x21: *mut c_float, ldx21: lapack_int,
                              theta: *mut c_float, u1: *mut c_float, ldu1: lapack_int,
                              u2: *mut c_float, ldu2: lapack_int, v1t: *mut c_float,
                              ldv1t: lapack_int)
                              -> lapack_int;
    pub fn LAPACKE_sorcsd2by1_work(matrix_layout: c_int, jobu1: c_char, jobu2: c_char,
                                   jobv1t: c_char, m: lapack_int, p: lapack_int, q: lapack_int,
                                   x11: *mut c_float, ldx11: lapack_int, x21: *mut c_float,
                                   ldx21: lapack_int, theta: *mut c_float, u1: *mut c_float,
                                   ldu1: lapack_int, u2: *mut c_float, ldu2: lapack_int,
                                   v1t: *mut c_float, ldv1t: lapack_int, work: *mut c_float,
                                   lwork: lapack_int, iwork: *mut lapack_int)
                                   -> lapack_int;
    pub fn LAPACKE_ssyconv(matrix_layout: c_int, uplo: c_char, way: c_char, n: lapack_int,
                           a: *mut c_float, lda: lapack_int, ipiv: *const lapack_int,
                           work: *mut c_float)
                           -> lapack_int;
    pub fn LAPACKE_ssyconv_work(matrix_layout: c_int, uplo: c_char, way: c_char, n: lapack_int,
                                a: *mut c_float, lda: lapack_int, ipiv: *const lapack_int,
                                work: *mut c_float)
                                -> lapack_int;
    pub fn LAPACKE_ssyswapr(matrix_layout: c_int, uplo: c_char, n: lapack_int, a: *mut c_float,
                            i1: lapack_int, i2: lapack_int)
                            -> lapack_int;
    pub fn LAPACKE_ssyswapr_work(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                                 a: *mut c_float, i1: lapack_int, i2: lapack_int)
                                 -> lapack_int;
    pub fn LAPACKE_ssytri2(matrix_layout: c_int, uplo: c_char, n: lapack_int, a: *mut c_float,
                           lda: lapack_int, ipiv: *const lapack_int)
                           -> lapack_int;
    pub fn LAPACKE_ssytri2_work(matrix_layout: c_int, uplo: c_char, n: lapack_int, a: *mut c_float,
                                lda: lapack_int, ipiv: *const lapack_int,
                                work: *mut lapack_complex_float, lwork: lapack_int)
                                -> lapack_int;
    pub fn LAPACKE_ssytri2x(matrix_layout: c_int, uplo: c_char, n: lapack_int, a: *mut c_float,
                            lda: lapack_int, ipiv: *const lapack_int, nb: lapack_int)
                            -> lapack_int;
    pub fn LAPACKE_ssytri2x_work(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                                 a: *mut c_float, lda: lapack_int, ipiv: *const lapack_int,
                                 work: *mut c_float, nb: lapack_int)
                                 -> lapack_int;
    pub fn LAPACKE_ssytrs2(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                           a: *const c_float, lda: lapack_int, ipiv: *const lapack_int,
                           b: *mut c_float, ldb: lapack_int)
                           -> lapack_int;
    pub fn LAPACKE_ssytrs2_work(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                                nrhs: lapack_int, a: *const c_float, lda: lapack_int,
                                ipiv: *const lapack_int, b: *mut c_float, ldb: lapack_int,
                                work: *mut c_float)
                                -> lapack_int;
    pub fn LAPACKE_zbbcsd(matrix_layout: c_int, jobu1: c_char, jobu2: c_char, jobv1t: c_char,
                          jobv2t: c_char, trans: c_char, m: lapack_int, p: lapack_int,
                          q: lapack_int, theta: *mut c_double, phi: *mut c_double,
                          u1: *mut lapack_complex_double, ldu1: lapack_int,
                          u2: *mut lapack_complex_double, ldu2: lapack_int,
                          v1t: *mut lapack_complex_double, ldv1t: lapack_int,
                          v2t: *mut lapack_complex_double, ldv2t: lapack_int, b11d: *mut c_double,
                          b11e: *mut c_double, b12d: *mut c_double, b12e: *mut c_double,
                          b21d: *mut c_double, b21e: *mut c_double, b22d: *mut c_double,
                          b22e: *mut c_double)
                          -> lapack_int;
    pub fn LAPACKE_zbbcsd_work(matrix_layout: c_int, jobu1: c_char, jobu2: c_char, jobv1t: c_char,
                               jobv2t: c_char, trans: c_char, m: lapack_int, p: lapack_int,
                               q: lapack_int, theta: *mut c_double, phi: *mut c_double,
                               u1: *mut lapack_complex_double, ldu1: lapack_int,
                               u2: *mut lapack_complex_double, ldu2: lapack_int,
                               v1t: *mut lapack_complex_double, ldv1t: lapack_int,
                               v2t: *mut lapack_complex_double, ldv2t: lapack_int,
                               b11d: *mut c_double, b11e: *mut c_double, b12d: *mut c_double,
                               b12e: *mut c_double, b21d: *mut c_double, b21e: *mut c_double,
                               b22d: *mut c_double, b22e: *mut c_double, rwork: *mut c_double,
                               lrwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_zheswapr(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                            a: *mut lapack_complex_double, i1: lapack_int, i2: lapack_int)
                            -> lapack_int;
    pub fn LAPACKE_zheswapr_work(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                                 a: *mut lapack_complex_double, i1: lapack_int, i2: lapack_int)
                                 -> lapack_int;
    pub fn LAPACKE_zhetri2(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                           a: *mut lapack_complex_double, lda: lapack_int, ipiv: *const lapack_int)
                           -> lapack_int;
    pub fn LAPACKE_zhetri2_work(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                                a: *mut lapack_complex_double, lda: lapack_int,
                                ipiv: *const lapack_int, work: *mut lapack_complex_double,
                                lwork: lapack_int)
                                -> lapack_int;
    pub fn LAPACKE_zhetri2x(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                            a: *mut lapack_complex_double, lda: lapack_int,
                            ipiv: *const lapack_int, nb: lapack_int)
                            -> lapack_int;
    pub fn LAPACKE_zhetri2x_work(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                                 a: *mut lapack_complex_double, lda: lapack_int,
                                 ipiv: *const lapack_int, work: *mut lapack_complex_double,
                                 nb: lapack_int)
                                 -> lapack_int;
    pub fn LAPACKE_zhetrs2(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                           a: *const lapack_complex_double, lda: lapack_int,
                           ipiv: *const lapack_int, b: *mut lapack_complex_double, ldb: lapack_int)
                           -> lapack_int;
    pub fn LAPACKE_zhetrs2_work(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                                nrhs: lapack_int, a: *const lapack_complex_double, lda: lapack_int,
                                ipiv: *const lapack_int, b: *mut lapack_complex_double,
                                ldb: lapack_int, work: *mut lapack_complex_double)
                                -> lapack_int;
    pub fn LAPACKE_zsyconv(matrix_layout: c_int, uplo: c_char, way: c_char, n: lapack_int,
                           a: *mut lapack_complex_double, lda: lapack_int, ipiv: *const lapack_int,
                           work: *mut lapack_complex_double)
                           -> lapack_int;
    pub fn LAPACKE_zsyconv_work(matrix_layout: c_int, uplo: c_char, way: c_char, n: lapack_int,
                                a: *mut lapack_complex_double, lda: lapack_int,
                                ipiv: *const lapack_int, work: *mut lapack_complex_double)
                                -> lapack_int;
    pub fn LAPACKE_zsyswapr(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                            a: *mut lapack_complex_double, i1: lapack_int, i2: lapack_int)
                            -> lapack_int;
    pub fn LAPACKE_zsyswapr_work(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                                 a: *mut lapack_complex_double, i1: lapack_int, i2: lapack_int)
                                 -> lapack_int;
    pub fn LAPACKE_zsytri2(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                           a: *mut lapack_complex_double, lda: lapack_int, ipiv: *const lapack_int)
                           -> lapack_int;
    pub fn LAPACKE_zsytri2_work(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                                a: *mut lapack_complex_double, lda: lapack_int,
                                ipiv: *const lapack_int, work: *mut lapack_complex_double,
                                lwork: lapack_int)
                                -> lapack_int;
    pub fn LAPACKE_zsytri2x(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                            a: *mut lapack_complex_double, lda: lapack_int,
                            ipiv: *const lapack_int, nb: lapack_int)
                            -> lapack_int;
    pub fn LAPACKE_zsytri2x_work(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                                 a: *mut lapack_complex_double, lda: lapack_int,
                                 ipiv: *const lapack_int, work: *mut lapack_complex_double,
                                 nb: lapack_int)
                                 -> lapack_int;
    pub fn LAPACKE_zsytrs2(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                           a: *const lapack_complex_double, lda: lapack_int,
                           ipiv: *const lapack_int, b: *mut lapack_complex_double, ldb: lapack_int)
                           -> lapack_int;
    pub fn LAPACKE_zsytrs2_work(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                                nrhs: lapack_int, a: *const lapack_complex_double, lda: lapack_int,
                                ipiv: *const lapack_int, b: *mut lapack_complex_double,
                                ldb: lapack_int, work: *mut lapack_complex_double)
                                -> lapack_int;
    pub fn LAPACKE_zunbdb(matrix_layout: c_int, trans: c_char, signs: c_char, m: lapack_int,
                          p: lapack_int, q: lapack_int, x11: *mut lapack_complex_double,
                          ldx11: lapack_int, x12: *mut lapack_complex_double, ldx12: lapack_int,
                          x21: *mut lapack_complex_double, ldx21: lapack_int,
                          x22: *mut lapack_complex_double, ldx22: lapack_int, theta: *mut c_double,
                          phi: *mut c_double, taup1: *mut lapack_complex_double,
                          taup2: *mut lapack_complex_double, tauq1: *mut lapack_complex_double,
                          tauq2: *mut lapack_complex_double)
                          -> lapack_int;
    pub fn LAPACKE_zunbdb_work(matrix_layout: c_int, trans: c_char, signs: c_char, m: lapack_int,
                               p: lapack_int, q: lapack_int, x11: *mut lapack_complex_double,
                               ldx11: lapack_int, x12: *mut lapack_complex_double,
                               ldx12: lapack_int, x21: *mut lapack_complex_double,
                               ldx21: lapack_int, x22: *mut lapack_complex_double,
                               ldx22: lapack_int, theta: *mut c_double, phi: *mut c_double,
                               taup1: *mut lapack_complex_double,
                               taup2: *mut lapack_complex_double,
                               tauq1: *mut lapack_complex_double,
                               tauq2: *mut lapack_complex_double, work: *mut lapack_complex_double,
                               lwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_zuncsd(matrix_layout: c_int, jobu1: c_char, jobu2: c_char, jobv1t: c_char,
                          jobv2t: c_char, trans: c_char, signs: c_char, m: lapack_int,
                          p: lapack_int, q: lapack_int, x11: *mut lapack_complex_double,
                          ldx11: lapack_int, x12: *mut lapack_complex_double, ldx12: lapack_int,
                          x21: *mut lapack_complex_double, ldx21: lapack_int,
                          x22: *mut lapack_complex_double, ldx22: lapack_int, theta: *mut c_double,
                          u1: *mut lapack_complex_double, ldu1: lapack_int,
                          u2: *mut lapack_complex_double, ldu2: lapack_int,
                          v1t: *mut lapack_complex_double, ldv1t: lapack_int,
                          v2t: *mut lapack_complex_double, ldv2t: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_zuncsd_work(matrix_layout: c_int, jobu1: c_char, jobu2: c_char, jobv1t: c_char,
                               jobv2t: c_char, trans: c_char, signs: c_char, m: lapack_int,
                               p: lapack_int, q: lapack_int, x11: *mut lapack_complex_double,
                               ldx11: lapack_int, x12: *mut lapack_complex_double,
                               ldx12: lapack_int, x21: *mut lapack_complex_double,
                               ldx21: lapack_int, x22: *mut lapack_complex_double,
                               ldx22: lapack_int, theta: *mut c_double,
                               u1: *mut lapack_complex_double, ldu1: lapack_int,
                               u2: *mut lapack_complex_double, ldu2: lapack_int,
                               v1t: *mut lapack_complex_double, ldv1t: lapack_int,
                               v2t: *mut lapack_complex_double, ldv2t: lapack_int,
                               work: *mut lapack_complex_double, lwork: lapack_int,
                               rwork: *mut c_double, lrwork: lapack_int, iwork: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_zuncsd2by1(matrix_layout: c_int, jobu1: c_char, jobu2: c_char, jobv1t: c_char,
                              m: lapack_int, p: lapack_int, q: lapack_int,
                              x11: *mut lapack_complex_double, ldx11: lapack_int,
                              x21: *mut lapack_complex_double, ldx21: lapack_int,
                              theta: *mut lapack_complex_double, u1: *mut lapack_complex_double,
                              ldu1: lapack_int, u2: *mut lapack_complex_double, ldu2: lapack_int,
                              v1t: *mut lapack_complex_double, ldv1t: lapack_int)
                              -> lapack_int;
    pub fn LAPACKE_zuncsd2by1_work(matrix_layout: c_int, jobu1: c_char, jobu2: c_char,
                                   jobv1t: c_char, m: lapack_int, p: lapack_int, q: lapack_int,
                                   x11: *mut lapack_complex_double, ldx11: lapack_int,
                                   x21: *mut lapack_complex_double, ldx21: lapack_int,
                                   theta: *mut lapack_complex_double,
                                   u1: *mut lapack_complex_double, ldu1: lapack_int,
                                   u2: *mut lapack_complex_double, ldu2: lapack_int,
                                   v1t: *mut lapack_complex_double, ldv1t: lapack_int,
                                   work: *mut lapack_complex_double, lwork: lapack_int,
                                   rwork: *mut c_double, lrwork: lapack_int,
                                   iwork: *mut lapack_int)
                                   -> lapack_int;

    // Version 3.4.0
    pub fn LAPACKE_sgemqrt(matrix_layout: c_int, side: c_char, trans: c_char, m: lapack_int,
                           n: lapack_int, k: lapack_int, nb: lapack_int, v: *const c_float,
                           ldv: lapack_int, t: *const c_float, ldt: lapack_int, c: *mut c_float,
                           ldc: lapack_int)
                           -> lapack_int;
    pub fn LAPACKE_dgemqrt(matrix_layout: c_int, side: c_char, trans: c_char, m: lapack_int,
                           n: lapack_int, k: lapack_int, nb: lapack_int, v: *const c_double,
                           ldv: lapack_int, t: *const c_double, ldt: lapack_int, c: *mut c_double,
                           ldc: lapack_int)
                           -> lapack_int;
    pub fn LAPACKE_cgemqrt(matrix_layout: c_int, side: c_char, trans: c_char, m: lapack_int,
                           n: lapack_int, k: lapack_int, nb: lapack_int,
                           v: *const lapack_complex_float, ldv: lapack_int,
                           t: *const lapack_complex_float, ldt: lapack_int,
                           c: *mut lapack_complex_float, ldc: lapack_int)
                           -> lapack_int;
    pub fn LAPACKE_zgemqrt(matrix_layout: c_int, side: c_char, trans: c_char, m: lapack_int,
                           n: lapack_int, k: lapack_int, nb: lapack_int,
                           v: *const lapack_complex_double, ldv: lapack_int,
                           t: *const lapack_complex_double, ldt: lapack_int,
                           c: *mut lapack_complex_double, ldc: lapack_int)
                           -> lapack_int;

    pub fn LAPACKE_sgeqrt(matrix_layout: c_int, m: lapack_int, n: lapack_int, nb: lapack_int,
                          a: *mut c_float, lda: lapack_int, t: *mut c_float, ldt: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_dgeqrt(matrix_layout: c_int, m: lapack_int, n: lapack_int, nb: lapack_int,
                          a: *mut c_double, lda: lapack_int, t: *mut c_double, ldt: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_cgeqrt(matrix_layout: c_int, m: lapack_int, n: lapack_int, nb: lapack_int,
                          a: *mut lapack_complex_float, lda: lapack_int,
                          t: *mut lapack_complex_float, ldt: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_zgeqrt(matrix_layout: c_int, m: lapack_int, n: lapack_int, nb: lapack_int,
                          a: *mut lapack_complex_double, lda: lapack_int,
                          t: *mut lapack_complex_double, ldt: lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_sgeqrt2(matrix_layout: c_int, m: lapack_int, n: lapack_int, a: *mut c_float,
                           lda: lapack_int, t: *mut c_float, ldt: lapack_int)
                           -> lapack_int;
    pub fn LAPACKE_dgeqrt2(matrix_layout: c_int, m: lapack_int, n: lapack_int, a: *mut c_double,
                           lda: lapack_int, t: *mut c_double, ldt: lapack_int)
                           -> lapack_int;
    pub fn LAPACKE_cgeqrt2(matrix_layout: c_int, m: lapack_int, n: lapack_int,
                           a: *mut lapack_complex_float, lda: lapack_int,
                           t: *mut lapack_complex_float, ldt: lapack_int)
                           -> lapack_int;
    pub fn LAPACKE_zgeqrt2(matrix_layout: c_int, m: lapack_int, n: lapack_int,
                           a: *mut lapack_complex_double, lda: lapack_int,
                           t: *mut lapack_complex_double, ldt: lapack_int)
                           -> lapack_int;

    pub fn LAPACKE_sgeqrt3(matrix_layout: c_int, m: lapack_int, n: lapack_int, a: *mut c_float,
                           lda: lapack_int, t: *mut c_float, ldt: lapack_int)
                           -> lapack_int;
    pub fn LAPACKE_dgeqrt3(matrix_layout: c_int, m: lapack_int, n: lapack_int, a: *mut c_double,
                           lda: lapack_int, t: *mut c_double, ldt: lapack_int)
                           -> lapack_int;
    pub fn LAPACKE_cgeqrt3(matrix_layout: c_int, m: lapack_int, n: lapack_int,
                           a: *mut lapack_complex_float, lda: lapack_int,
                           t: *mut lapack_complex_float, ldt: lapack_int)
                           -> lapack_int;
    pub fn LAPACKE_zgeqrt3(matrix_layout: c_int, m: lapack_int, n: lapack_int,
                           a: *mut lapack_complex_double, lda: lapack_int,
                           t: *mut lapack_complex_double, ldt: lapack_int)
                           -> lapack_int;

    pub fn LAPACKE_stpmqrt(matrix_layout: c_int, side: c_char, trans: c_char, m: lapack_int,
                           n: lapack_int, k: lapack_int, l: lapack_int, nb: lapack_int,
                           v: *const c_float, ldv: lapack_int, t: *const c_float, ldt: lapack_int,
                           a: *mut c_float, lda: lapack_int, b: *mut c_float, ldb: lapack_int)
                           -> lapack_int;
    pub fn LAPACKE_dtpmqrt(matrix_layout: c_int, side: c_char, trans: c_char, m: lapack_int,
                           n: lapack_int, k: lapack_int, l: lapack_int, nb: lapack_int,
                           v: *const c_double, ldv: lapack_int, t: *const c_double,
                           ldt: lapack_int, a: *mut c_double, lda: lapack_int, b: *mut c_double,
                           ldb: lapack_int)
                           -> lapack_int;
    pub fn LAPACKE_ctpmqrt(matrix_layout: c_int, side: c_char, trans: c_char, m: lapack_int,
                           n: lapack_int, k: lapack_int, l: lapack_int, nb: lapack_int,
                           v: *const lapack_complex_float, ldv: lapack_int,
                           t: *const lapack_complex_float, ldt: lapack_int,
                           a: *mut lapack_complex_float, lda: lapack_int,
                           b: *mut lapack_complex_float, ldb: lapack_int)
                           -> lapack_int;
    pub fn LAPACKE_ztpmqrt(matrix_layout: c_int, side: c_char, trans: c_char, m: lapack_int,
                           n: lapack_int, k: lapack_int, l: lapack_int, nb: lapack_int,
                           v: *const lapack_complex_double, ldv: lapack_int,
                           t: *const lapack_complex_double, ldt: lapack_int,
                           a: *mut lapack_complex_double, lda: lapack_int,
                           b: *mut lapack_complex_double, ldb: lapack_int)
                           -> lapack_int;

    pub fn LAPACKE_stpqrt(matrix_layout: c_int, m: lapack_int, n: lapack_int, l: lapack_int,
                          nb: lapack_int, a: *mut c_float, lda: lapack_int, b: *mut c_float,
                          ldb: lapack_int, t: *mut c_float, ldt: lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_dtpqrt(matrix_layout: c_int, m: lapack_int, n: lapack_int, l: lapack_int,
                          nb: lapack_int, a: *mut c_double, lda: lapack_int, b: *mut c_double,
                          ldb: lapack_int, t: *mut c_double, ldt: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_ctpqrt(matrix_layout: c_int, m: lapack_int, n: lapack_int, l: lapack_int,
                          nb: lapack_int, a: *mut lapack_complex_float, lda: lapack_int,
                          b: *mut lapack_complex_float, ldb: lapack_int,
                          t: *mut lapack_complex_float, ldt: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_ztpqrt(matrix_layout: c_int, m: lapack_int, n: lapack_int, l: lapack_int,
                          nb: lapack_int, a: *mut lapack_complex_double, lda: lapack_int,
                          b: *mut lapack_complex_double, ldb: lapack_int,
                          t: *mut lapack_complex_double, ldt: lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_stpqrt2(matrix_layout: c_int, m: lapack_int, n: lapack_int, l: lapack_int,
                           a: *mut c_float, lda: lapack_int, b: *mut c_float, ldb: lapack_int,
                           t: *mut c_float, ldt: lapack_int)
                           -> lapack_int;
    pub fn LAPACKE_dtpqrt2(matrix_layout: c_int, m: lapack_int, n: lapack_int, l: lapack_int,
                           a: *mut c_double, lda: lapack_int, b: *mut c_double, ldb: lapack_int,
                           t: *mut c_double, ldt: lapack_int)
                           -> lapack_int;
    pub fn LAPACKE_ctpqrt2(matrix_layout: c_int, m: lapack_int, n: lapack_int, l: lapack_int,
                           a: *mut lapack_complex_float, lda: lapack_int,
                           b: *mut lapack_complex_float, ldb: lapack_int,
                           t: *mut lapack_complex_float, ldt: lapack_int)
                           -> lapack_int;
    pub fn LAPACKE_ztpqrt2(matrix_layout: c_int, m: lapack_int, n: lapack_int, l: lapack_int,
                           a: *mut lapack_complex_double, lda: lapack_int,
                           b: *mut lapack_complex_double, ldb: lapack_int,
                           t: *mut lapack_complex_double, ldt: lapack_int)
                           -> lapack_int;

    pub fn LAPACKE_stprfb(matrix_layout: c_int, side: c_char, trans: c_char, direct: c_char,
                          storev: c_char, m: lapack_int, n: lapack_int, k: lapack_int,
                          l: lapack_int, v: *const c_float, ldv: lapack_int, t: *const c_float,
                          ldt: lapack_int, a: *mut c_float, lda: lapack_int, b: *mut c_float,
                          ldb: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_dtprfb(matrix_layout: c_int, side: c_char, trans: c_char, direct: c_char,
                          storev: c_char, m: lapack_int, n: lapack_int, k: lapack_int,
                          l: lapack_int, v: *const c_double, ldv: lapack_int, t: *const c_double,
                          ldt: lapack_int, a: *mut c_double, lda: lapack_int, b: *mut c_double,
                          ldb: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_ctprfb(matrix_layout: c_int, side: c_char, trans: c_char, direct: c_char,
                          storev: c_char, m: lapack_int, n: lapack_int, k: lapack_int,
                          l: lapack_int, v: *const lapack_complex_float, ldv: lapack_int,
                          t: *const lapack_complex_float, ldt: lapack_int,
                          a: *mut lapack_complex_float, lda: lapack_int,
                          b: *mut lapack_complex_float, ldb: lapack_int)
                          -> lapack_int;
    pub fn LAPACKE_ztprfb(matrix_layout: c_int, side: c_char, trans: c_char, direct: c_char,
                          storev: c_char, m: lapack_int, n: lapack_int, k: lapack_int,
                          l: lapack_int, v: *const lapack_complex_double, ldv: lapack_int,
                          t: *const lapack_complex_double, ldt: lapack_int,
                          a: *mut lapack_complex_double, lda: lapack_int,
                          b: *mut lapack_complex_double, ldb: lapack_int)
                          -> lapack_int;

    pub fn LAPACKE_sgemqrt_work(matrix_layout: c_int, side: c_char, trans: c_char, m: lapack_int,
                                n: lapack_int, k: lapack_int, nb: lapack_int, v: *const c_float,
                                ldv: lapack_int, t: *const c_float, ldt: lapack_int,
                                c: *mut c_float, ldc: lapack_int, work: *mut c_float)
                                -> lapack_int;
    pub fn LAPACKE_dgemqrt_work(matrix_layout: c_int, side: c_char, trans: c_char, m: lapack_int,
                                n: lapack_int, k: lapack_int, nb: lapack_int, v: *const c_double,
                                ldv: lapack_int, t: *const c_double, ldt: lapack_int,
                                c: *mut c_double, ldc: lapack_int, work: *mut c_double)
                                -> lapack_int;
    pub fn LAPACKE_cgemqrt_work(matrix_layout: c_int, side: c_char, trans: c_char, m: lapack_int,
                                n: lapack_int, k: lapack_int, nb: lapack_int,
                                v: *const lapack_complex_float, ldv: lapack_int,
                                t: *const lapack_complex_float, ldt: lapack_int,
                                c: *mut lapack_complex_float, ldc: lapack_int,
                                work: *mut lapack_complex_float)
                                -> lapack_int;
    pub fn LAPACKE_zgemqrt_work(matrix_layout: c_int, side: c_char, trans: c_char, m: lapack_int,
                                n: lapack_int, k: lapack_int, nb: lapack_int,
                                v: *const lapack_complex_double, ldv: lapack_int,
                                t: *const lapack_complex_double, ldt: lapack_int,
                                c: *mut lapack_complex_double, ldc: lapack_int,
                                work: *mut lapack_complex_double)
                                -> lapack_int;

    pub fn LAPACKE_sgeqrt_work(matrix_layout: c_int, m: lapack_int, n: lapack_int, nb: lapack_int,
                               a: *mut c_float, lda: lapack_int, t: *mut c_float, ldt: lapack_int,
                               work: *mut c_float)
                               -> lapack_int;
    pub fn LAPACKE_dgeqrt_work(matrix_layout: c_int, m: lapack_int, n: lapack_int, nb: lapack_int,
                               a: *mut c_double, lda: lapack_int, t: *mut c_double,
                               ldt: lapack_int, work: *mut c_double)
                               -> lapack_int;
    pub fn LAPACKE_cgeqrt_work(matrix_layout: c_int, m: lapack_int, n: lapack_int, nb: lapack_int,
                               a: *mut lapack_complex_float, lda: lapack_int,
                               t: *mut lapack_complex_float, ldt: lapack_int,
                               work: *mut lapack_complex_float)
                               -> lapack_int;
    pub fn LAPACKE_zgeqrt_work(matrix_layout: c_int, m: lapack_int, n: lapack_int, nb: lapack_int,
                               a: *mut lapack_complex_double, lda: lapack_int,
                               t: *mut lapack_complex_double, ldt: lapack_int,
                               work: *mut lapack_complex_double)
                               -> lapack_int;

    pub fn LAPACKE_sgeqrt2_work(matrix_layout: c_int, m: lapack_int, n: lapack_int,
                                a: *mut c_float, lda: lapack_int, t: *mut c_float, ldt: lapack_int)
                                -> lapack_int;
    pub fn LAPACKE_dgeqrt2_work(matrix_layout: c_int, m: lapack_int, n: lapack_int,
                                a: *mut c_double, lda: lapack_int, t: *mut c_double,
                                ldt: lapack_int)
                                -> lapack_int;
    pub fn LAPACKE_cgeqrt2_work(matrix_layout: c_int, m: lapack_int, n: lapack_int,
                                a: *mut lapack_complex_float, lda: lapack_int,
                                t: *mut lapack_complex_float, ldt: lapack_int)
                                -> lapack_int;
    pub fn LAPACKE_zgeqrt2_work(matrix_layout: c_int, m: lapack_int, n: lapack_int,
                                a: *mut lapack_complex_double, lda: lapack_int,
                                t: *mut lapack_complex_double, ldt: lapack_int)
                                -> lapack_int;

    pub fn LAPACKE_sgeqrt3_work(matrix_layout: c_int, m: lapack_int, n: lapack_int,
                                a: *mut c_float, lda: lapack_int, t: *mut c_float, ldt: lapack_int)
                                -> lapack_int;
    pub fn LAPACKE_dgeqrt3_work(matrix_layout: c_int, m: lapack_int, n: lapack_int,
                                a: *mut c_double, lda: lapack_int, t: *mut c_double,
                                ldt: lapack_int)
                                -> lapack_int;
    pub fn LAPACKE_cgeqrt3_work(matrix_layout: c_int, m: lapack_int, n: lapack_int,
                                a: *mut lapack_complex_float, lda: lapack_int,
                                t: *mut lapack_complex_float, ldt: lapack_int)
                                -> lapack_int;
    pub fn LAPACKE_zgeqrt3_work(matrix_layout: c_int, m: lapack_int, n: lapack_int,
                                a: *mut lapack_complex_double, lda: lapack_int,
                                t: *mut lapack_complex_double, ldt: lapack_int)
                                -> lapack_int;

    pub fn LAPACKE_stpmqrt_work(matrix_layout: c_int, side: c_char, trans: c_char, m: lapack_int,
                                n: lapack_int, k: lapack_int, l: lapack_int, nb: lapack_int,
                                v: *const c_float, ldv: lapack_int, t: *const c_float,
                                ldt: lapack_int, a: *mut c_float, lda: lapack_int, b: *mut c_float,
                                ldb: lapack_int, work: *mut c_float)
                                -> lapack_int;
    pub fn LAPACKE_dtpmqrt_work(matrix_layout: c_int, side: c_char, trans: c_char, m: lapack_int,
                                n: lapack_int, k: lapack_int, l: lapack_int, nb: lapack_int,
                                v: *const c_double, ldv: lapack_int, t: *const c_double,
                                ldt: lapack_int, a: *mut c_double, lda: lapack_int,
                                b: *mut c_double, ldb: lapack_int, work: *mut c_double)
                                -> lapack_int;
    pub fn LAPACKE_ctpmqrt_work(matrix_layout: c_int, side: c_char, trans: c_char, m: lapack_int,
                                n: lapack_int, k: lapack_int, l: lapack_int, nb: lapack_int,
                                v: *const lapack_complex_float, ldv: lapack_int,
                                t: *const lapack_complex_float, ldt: lapack_int,
                                a: *mut lapack_complex_float, lda: lapack_int,
                                b: *mut lapack_complex_float, ldb: lapack_int,
                                work: *mut lapack_complex_float)
                                -> lapack_int;
    pub fn LAPACKE_ztpmqrt_work(matrix_layout: c_int, side: c_char, trans: c_char, m: lapack_int,
                                n: lapack_int, k: lapack_int, l: lapack_int, nb: lapack_int,
                                v: *const lapack_complex_double, ldv: lapack_int,
                                t: *const lapack_complex_double, ldt: lapack_int,
                                a: *mut lapack_complex_double, lda: lapack_int,
                                b: *mut lapack_complex_double, ldb: lapack_int,
                                work: *mut lapack_complex_double)
                                -> lapack_int;

    pub fn LAPACKE_stpqrt_work(matrix_layout: c_int, m: lapack_int, n: lapack_int, l: lapack_int,
                               nb: lapack_int, a: *mut c_float, lda: lapack_int, b: *mut c_float,
                               ldb: lapack_int, t: *mut c_float, ldt: lapack_int,
                               work: *mut c_float)
                               -> lapack_int;
    pub fn LAPACKE_dtpqrt_work(matrix_layout: c_int, m: lapack_int, n: lapack_int, l: lapack_int,
                               nb: lapack_int, a: *mut c_double, lda: lapack_int, b: *mut c_double,
                               ldb: lapack_int, t: *mut c_double, ldt: lapack_int,
                               work: *mut c_double)
                               -> lapack_int;
    pub fn LAPACKE_ctpqrt_work(matrix_layout: c_int, m: lapack_int, n: lapack_int, l: lapack_int,
                               nb: lapack_int, a: *mut lapack_complex_float, lda: lapack_int,
                               b: *mut lapack_complex_float, ldb: lapack_int,
                               t: *mut lapack_complex_float, ldt: lapack_int,
                               work: *mut lapack_complex_float)
                               -> lapack_int;
    pub fn LAPACKE_ztpqrt_work(matrix_layout: c_int, m: lapack_int, n: lapack_int, l: lapack_int,
                               nb: lapack_int, a: *mut lapack_complex_double, lda: lapack_int,
                               b: *mut lapack_complex_double, ldb: lapack_int,
                               t: *mut lapack_complex_double, ldt: lapack_int,
                               work: *mut lapack_complex_double)
                               -> lapack_int;

    pub fn LAPACKE_stpqrt2_work(matrix_layout: c_int, m: lapack_int, n: lapack_int, l: lapack_int,
                                a: *mut c_float, lda: lapack_int, b: *mut c_float, ldb: lapack_int,
                                t: *mut c_float, ldt: lapack_int)
                                -> lapack_int;
    pub fn LAPACKE_dtpqrt2_work(matrix_layout: c_int, m: lapack_int, n: lapack_int, l: lapack_int,
                                a: *mut c_double, lda: lapack_int, b: *mut c_double,
                                ldb: lapack_int, t: *mut c_double, ldt: lapack_int)
                                -> lapack_int;
    pub fn LAPACKE_ctpqrt2_work(matrix_layout: c_int, m: lapack_int, n: lapack_int, l: lapack_int,
                                a: *mut lapack_complex_float, lda: lapack_int,
                                b: *mut lapack_complex_float, ldb: lapack_int,
                                t: *mut lapack_complex_float, ldt: lapack_int)
                                -> lapack_int;
    pub fn LAPACKE_ztpqrt2_work(matrix_layout: c_int, m: lapack_int, n: lapack_int, l: lapack_int,
                                a: *mut lapack_complex_double, lda: lapack_int,
                                b: *mut lapack_complex_double, ldb: lapack_int,
                                t: *mut lapack_complex_double, ldt: lapack_int)
                                -> lapack_int;

    pub fn LAPACKE_stprfb_work(matrix_layout: c_int, side: c_char, trans: c_char, direct: c_char,
                               storev: c_char, m: lapack_int, n: lapack_int, k: lapack_int,
                               l: lapack_int, v: *const c_float, ldv: lapack_int,
                               t: *const c_float, ldt: lapack_int, a: *mut c_float,
                               lda: lapack_int, b: *mut c_float, ldb: lapack_int,
                               work: *const c_float, ldwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dtprfb_work(matrix_layout: c_int, side: c_char, trans: c_char, direct: c_char,
                               storev: c_char, m: lapack_int, n: lapack_int, k: lapack_int,
                               l: lapack_int, v: *const c_double, ldv: lapack_int,
                               t: *const c_double, ldt: lapack_int, a: *mut c_double,
                               lda: lapack_int, b: *mut c_double, ldb: lapack_int,
                               work: *const c_double, ldwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_ctprfb_work(matrix_layout: c_int, side: c_char, trans: c_char, direct: c_char,
                               storev: c_char, m: lapack_int, n: lapack_int, k: lapack_int,
                               l: lapack_int, v: *const lapack_complex_float, ldv: lapack_int,
                               t: *const lapack_complex_float, ldt: lapack_int,
                               a: *mut lapack_complex_float, lda: lapack_int,
                               b: *mut lapack_complex_float, ldb: lapack_int,
                               work: *mut lapack_complex_float, ldwork: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_ztprfb_work(matrix_layout: c_int, side: c_char, trans: c_char, direct: c_char,
                               storev: c_char, m: lapack_int, n: lapack_int, k: lapack_int,
                               l: lapack_int, v: *const lapack_complex_double, ldv: lapack_int,
                               t: *const lapack_complex_double, ldt: lapack_int,
                               a: *mut lapack_complex_double, lda: lapack_int,
                               b: *mut lapack_complex_double, ldb: lapack_int,
                               work: *mut lapack_complex_double, ldwork: lapack_int)
                               -> lapack_int;
    // Version 3.5.0
    pub fn LAPACKE_ssysv_rook(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                              a: *mut c_float, lda: lapack_int, ipiv: *mut lapack_int,
                              b: *mut c_float, ldb: lapack_int)
                              -> lapack_int;
    pub fn LAPACKE_dsysv_rook(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                              a: *mut c_double, lda: lapack_int, ipiv: *mut lapack_int,
                              b: *mut c_double, ldb: lapack_int)
                              -> lapack_int;
    pub fn LAPACKE_csysv_rook(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                              a: *mut lapack_complex_float, lda: lapack_int, ipiv: *mut lapack_int,
                              b: *mut lapack_complex_float, ldb: lapack_int)
                              -> lapack_int;
    pub fn LAPACKE_zsysv_rook(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                              a: *mut lapack_complex_double, lda: lapack_int,
                              ipiv: *mut lapack_int, b: *mut lapack_complex_double,
                              ldb: lapack_int)
                              -> lapack_int;

    pub fn LAPACKE_ssytrf_rook(matrix_layout: c_int, uplo: c_char, n: lapack_int, a: *mut c_float,
                               lda: lapack_int, ipiv: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dsytrf_rook(matrix_layout: c_int, uplo: c_char, n: lapack_int, a: *mut c_double,
                               lda: lapack_int, ipiv: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_csytrf_rook(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                               a: *mut lapack_complex_float, lda: lapack_int,
                               ipiv: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_zsytrf_rook(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                               a: *mut lapack_complex_double, lda: lapack_int,
                               ipiv: *mut lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_ssytrs_rook(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                               a: *const c_float, lda: lapack_int, ipiv: *const lapack_int,
                               b: *mut c_float, ldb: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_dsytrs_rook(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                               a: *const c_double, lda: lapack_int, ipiv: *const lapack_int,
                               b: *mut c_double, ldb: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_csytrs_rook(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                               a: *const lapack_complex_float, lda: lapack_int,
                               ipiv: *const lapack_int, b: *mut lapack_complex_float,
                               ldb: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_zsytrs_rook(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                               a: *const lapack_complex_double, lda: lapack_int,
                               ipiv: *const lapack_int, b: *mut lapack_complex_double,
                               ldb: lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_chetrf_rook(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                               a: *mut lapack_complex_float, lda: lapack_int,
                               ipiv: *mut lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_zhetrf_rook(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                               a: *mut lapack_complex_double, lda: lapack_int,
                               ipiv: *mut lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_chetrs_rook(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                               a: *const lapack_complex_float, lda: lapack_int,
                               ipiv: *const lapack_int, b: *mut lapack_complex_float,
                               ldb: lapack_int)
                               -> lapack_int;
    pub fn LAPACKE_zhetrs_rook(matrix_layout: c_int, uplo: c_char, n: lapack_int, nrhs: lapack_int,
                               a: *const lapack_complex_double, lda: lapack_int,
                               ipiv: *const lapack_int, b: *mut lapack_complex_double,
                               ldb: lapack_int)
                               -> lapack_int;

    pub fn LAPACKE_csyr(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                        alpha: lapack_complex_float, x: *const lapack_complex_float,
                        incx: lapack_int, a: *mut lapack_complex_float, lda: lapack_int)
                        -> lapack_int;
    pub fn LAPACKE_zsyr(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                        alpha: lapack_complex_double, x: *const lapack_complex_double,
                        incx: lapack_int, a: *mut lapack_complex_double, lda: lapack_int)
                        -> lapack_int;

    pub fn LAPACKE_ssysv_rook_work(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                                   nrhs: lapack_int, a: *mut c_float, lda: lapack_int,
                                   ipiv: *mut lapack_int, b: *mut c_float, ldb: lapack_int,
                                   work: *mut c_float, lwork: lapack_int)
                                   -> lapack_int;
    pub fn LAPACKE_dsysv_rook_work(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                                   nrhs: lapack_int, a: *mut c_double, lda: lapack_int,
                                   ipiv: *mut lapack_int, b: *mut c_double, ldb: lapack_int,
                                   work: *mut c_double, lwork: lapack_int)
                                   -> lapack_int;
    pub fn LAPACKE_csysv_rook_work(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                                   nrhs: lapack_int, a: *mut lapack_complex_float, lda: lapack_int,
                                   ipiv: *mut lapack_int, b: *mut lapack_complex_float,
                                   ldb: lapack_int, work: *mut lapack_complex_float,
                                   lwork: lapack_int)
                                   -> lapack_int;
    pub fn LAPACKE_zsysv_rook_work(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                                   nrhs: lapack_int, a: *mut lapack_complex_double,
                                   lda: lapack_int, ipiv: *mut lapack_int,
                                   b: *mut lapack_complex_double, ldb: lapack_int,
                                   work: *mut lapack_complex_double, lwork: lapack_int)
                                   -> lapack_int;

    pub fn LAPACKE_ssytrf_rook_work(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                                    a: *mut c_float, lda: lapack_int, ipiv: *mut lapack_int,
                                    work: *mut c_float, lwork: lapack_int)
                                    -> lapack_int;
    pub fn LAPACKE_dsytrf_rook_work(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                                    a: *mut c_double, lda: lapack_int, ipiv: *mut lapack_int,
                                    work: *mut c_double, lwork: lapack_int)
                                    -> lapack_int;
    pub fn LAPACKE_csytrf_rook_work(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                                    a: *mut lapack_complex_float, lda: lapack_int,
                                    ipiv: *mut lapack_int, work: *mut lapack_complex_float,
                                    lwork: lapack_int)
                                    -> lapack_int;
    pub fn LAPACKE_zsytrf_rook_work(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                                    a: *mut lapack_complex_double, lda: lapack_int,
                                    ipiv: *mut lapack_int, work: *mut lapack_complex_double,
                                    lwork: lapack_int)
                                    -> lapack_int;

    pub fn LAPACKE_ssytrs_rook_work(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                                    nrhs: lapack_int, a: *const c_float, lda: lapack_int,
                                    ipiv: *const lapack_int, b: *mut c_float, ldb: lapack_int)
                                    -> lapack_int;
    pub fn LAPACKE_dsytrs_rook_work(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                                    nrhs: lapack_int, a: *const c_double, lda: lapack_int,
                                    ipiv: *const lapack_int, b: *mut c_double, ldb: lapack_int)
                                    -> lapack_int;
    pub fn LAPACKE_csytrs_rook_work(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                                    nrhs: lapack_int, a: *const lapack_complex_float,
                                    lda: lapack_int, ipiv: *const lapack_int,
                                    b: *mut lapack_complex_float, ldb: lapack_int)
                                    -> lapack_int;
    pub fn LAPACKE_zsytrs_rook_work(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                                    nrhs: lapack_int, a: *const lapack_complex_double,
                                    lda: lapack_int, ipiv: *const lapack_int,
                                    b: *mut lapack_complex_double, ldb: lapack_int)
                                    -> lapack_int;

    pub fn LAPACKE_chetrf_rook_work(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                                    a: *mut lapack_complex_float, lda: lapack_int,
                                    ipiv: *mut lapack_int, work: *mut lapack_complex_float,
                                    lwork: lapack_int)
                                    -> lapack_int;
    pub fn LAPACKE_zhetrf_rook_work(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                                    a: *mut lapack_complex_double, lda: lapack_int,
                                    ipiv: *mut lapack_int, work: *mut lapack_complex_double,
                                    lwork: lapack_int)
                                    -> lapack_int;

    pub fn LAPACKE_chetrs_rook_work(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                                    nrhs: lapack_int, a: *const lapack_complex_float,
                                    lda: lapack_int, ipiv: *const lapack_int,
                                    b: *mut lapack_complex_float, ldb: lapack_int)
                                    -> lapack_int;
    pub fn LAPACKE_zhetrs_rook_work(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                                    nrhs: lapack_int, a: *const lapack_complex_double,
                                    lda: lapack_int, ipiv: *const lapack_int,
                                    b: *mut lapack_complex_double, ldb: lapack_int)
                                    -> lapack_int;

    pub fn LAPACKE_csyr_work(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                             alpha: lapack_complex_float, x: *const lapack_complex_float,
                             incx: lapack_int, a: *mut lapack_complex_float, lda: lapack_int)
                             -> lapack_int;
    pub fn LAPACKE_zsyr_work(matrix_layout: c_int, uplo: c_char, n: lapack_int,
                             alpha: lapack_complex_double, x: *const lapack_complex_double,
                             incx: lapack_int, a: *mut lapack_complex_double, lda: lapack_int)
                             -> lapack_int;

    pub fn LAPACKE_ilaver(vers_major: *mut lapack_int, vers_minor: *mut lapack_int,
                          vers_patch: *mut lapack_int);
"""

name_re = re.compile("\s*pub fn LAPACKE_(\w+)")
argument_re = re.compile("(\w+): ([^,]*)(,|\))")
return_re = re.compile("(?:\s*->\s*([^;]+))?");
select_re = re.compile("LAPACK_(\w)_SELECT(\d)")

def pull_name(s):
    match = name_re.match(s)
    assert(match is not None)
    return match.group(1), s[match.end(1):]

def pull_argument(s):
    match = argument_re.match(s)
    if match is None:
        return None, None, s
    return match.group(1), match.group(2), s[match.end(3):]

def pull_return(s):
    match = return_re.match(s)
    if match is None:
        return None, s
    return match.group(1), s[match.end(1):]

def chew(s, c):
    assert(s[0] == c)
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

def is_scalar(name, cty, f):
    return (
        "c_char" in cty or
        name in [
            "abstol",
            "anorm",
            "ihi",
            "il",
            "ilo",
            "info",
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
            "tryrac",
            "vu",
        ] or
        not ('tgsna' in f.name or 'trsna' in f.name) and name in [
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

    base = translate_type_base(cty)
    if "*const" in cty:
        if is_scalar(name, cty, f):
            return base
        else:
            return "&[{}]".format(base)
    elif "*mut" in cty:
        if is_scalar(name, cty, f):
            return "&mut {}".format(base)
        else:
            return "&mut [{}]".format(base)

    return base

def translate_type_base(cty):
    if "c_char" in cty:
        return "u8"
    elif "c_int" in cty or "lapack_int" in cty or "lapack_logical" in cty:
        return "i32"
    elif "lapack_complex_double" in cty:
        return "c64"
    elif "lapack_complex_float" in cty:
        return "c32"
    elif "c_double" in cty:
        return "f64"
    elif "c_float" in cty:
        return "f32"

    assert False, "cannot translate `{}`".format(cty)

def translate_body_argument(name, rty):
    if rty == "u8":
        return "{} as c_char".format(name)
    elif rty == "&mut u8":
        return "{} as *mut _ as *mut _".format(name)

    elif rty in ["i32", "&mut i32"]:
        return name
    elif rty == "&[i32]":
        return "{}.as_ptr()".format(name)
    elif rty == "&mut [i32]":
        return "{}.as_mut_ptr()".format(name)

    elif rty.startswith("f"):
        return name
    elif rty.startswith("&mut f"):
        return name
    elif rty.startswith("&[f"):
        return "{}.as_ptr()".format(name)
    elif rty.startswith("&mut [f"):
        return "{}.as_mut_ptr()".format(name)

    elif rty.startswith("c"):
        return "transmute({})".format(name)
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
    if cty == "lapack_int":
        return "i32"
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
        i = 98 - header[98::-1].index(',')
        if i < 0:
            s.append(header)
            break
        s.append(header[:i+1])
        header = "{}{}".format(" " * indent, header[i+2:])

    if len(s) > 1:
        s.append("")

    return "\n".join(s)

def format_body(f):
    a = format_body_arguments(f)
    tail = "{})".format(a)

    s = []
    s.append(" " * 4)
    s.append("unsafe {\n")
    s.append(" " * 8)
    s.append("ffi::LAPACKE_{}(".format(f.name))

    indent = 8 + 13 + len(f.name) + 1
    while len(tail) > 0:
        if len(tail) + indent > 99:
            i = tail.find(",")
            if i < 0 or i > 98:
                assert False, "cannot format `{}`".format(f.name)
            while True:
                l = tail.find(",", i + 1)
                if l < 0 or l + indent > 98: break
                i = l
            s.append(tail[0:i+1])
            s.append("\n")
            s.append(" " * indent)
            tail = tail[i+2:]
        else:
            s.append(tail)
            tail = ""

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
    lines = filter(lambda line: not re.match(r'^\s*//.*', line), code.split('\n'))
    lines = re.sub(r'\s+', ' ', "".join(lines)).strip().split(';')
    lines = filter(lambda line: not re.match(r'^\s*$', line), lines)
    return [Func.parse(line) for line in lines]

def do(funcs):
    for f in funcs:
        print("#[inline]")
        print(format_header(f))
        print(format_body(f))
        print("}\n")

do(prepare(functions))
