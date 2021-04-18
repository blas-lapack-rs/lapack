#[inline]
pub unsafe fn cbbcsd(
    jobu1: u8,
    jobu2: u8,
    jobv1t: u8,
    jobv2t: u8,
    trans: u8,
    m: i32,
    p: i32,
    q: &[i32],
    theta: &mut [f32],
    phi: &mut [f32],
    u1: &mut [c32],
    ldu1: i32,
    u2: &mut [c32],
    ldu2: i32,
    v1t: &mut [c32],
    ldv1t: i32,
    v2t: &mut [c32],
    ldv2t: i32,
    b11d: &mut [f32],
    b11e: &mut [f32],
    b12d: &mut [f32],
    b12e: &mut [f32],
    b21d: &mut [f32],
    b21e: &mut [f32],
    b22d: &mut [f32],
    b22e: &mut [f32],
    rwork: &mut [f32],
    lrwork: i32,
    info: &mut i32,
) {
    ffi::cbbcsd_(
        &(jobu1 as c_char),
        &(jobu2 as c_char),
        &(jobv1t as c_char),
        &(jobv2t as c_char),
        &(trans as c_char),
        &m,
        &p,
        q.as_ptr(),
        theta.as_mut_ptr(),
        phi.as_mut_ptr(),
        u1.as_mut_ptr() as *mut _,
        &ldu1,
        u2.as_mut_ptr() as *mut _,
        &ldu2,
        v1t.as_mut_ptr() as *mut _,
        &ldv1t,
        v2t.as_mut_ptr() as *mut _,
        &ldv2t,
        b11d.as_mut_ptr(),
        b11e.as_mut_ptr(),
        b12d.as_mut_ptr(),
        b12e.as_mut_ptr(),
        b21d.as_mut_ptr(),
        b21e.as_mut_ptr(),
        b22d.as_mut_ptr(),
        b22e.as_mut_ptr(),
        rwork.as_mut_ptr(),
        &lrwork,
        info,
    )
}

#[inline]
pub unsafe fn dbbcsd(
    jobu1: u8,
    jobu2: u8,
    jobv1t: u8,
    jobv2t: u8,
    trans: u8,
    m: i32,
    p: i32,
    q: &[i32],
    theta: &mut [f64],
    phi: &mut [f64],
    u1: &mut [f64],
    ldu1: i32,
    u2: &mut [f64],
    ldu2: i32,
    v1t: &mut [f64],
    ldv1t: i32,
    v2t: &mut [f64],
    ldv2t: i32,
    b11d: &mut [f64],
    b11e: &mut [f64],
    b12d: &mut [f64],
    b12e: &mut [f64],
    b21d: &mut [f64],
    b21e: &mut [f64],
    b22d: &mut [f64],
    b22e: &mut [f64],
    work: &mut [f64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::dbbcsd_(
        &(jobu1 as c_char),
        &(jobu2 as c_char),
        &(jobv1t as c_char),
        &(jobv2t as c_char),
        &(trans as c_char),
        &m,
        &p,
        q.as_ptr(),
        theta.as_mut_ptr(),
        phi.as_mut_ptr(),
        u1.as_mut_ptr(),
        &ldu1,
        u2.as_mut_ptr(),
        &ldu2,
        v1t.as_mut_ptr(),
        &ldv1t,
        v2t.as_mut_ptr(),
        &ldv2t,
        b11d.as_mut_ptr(),
        b11e.as_mut_ptr(),
        b12d.as_mut_ptr(),
        b12e.as_mut_ptr(),
        b21d.as_mut_ptr(),
        b21e.as_mut_ptr(),
        b22d.as_mut_ptr(),
        b22e.as_mut_ptr(),
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn sbbcsd(
    jobu1: u8,
    jobu2: u8,
    jobv1t: u8,
    jobv2t: u8,
    trans: u8,
    m: i32,
    p: i32,
    q: &[i32],
    theta: &mut [f32],
    phi: &mut [f32],
    u1: &mut [f32],
    ldu1: i32,
    u2: &mut [f32],
    ldu2: i32,
    v1t: &mut [f32],
    ldv1t: i32,
    v2t: &mut [f32],
    ldv2t: i32,
    b11d: &mut [f32],
    b11e: &mut [f32],
    b12d: &mut [f32],
    b12e: &mut [f32],
    b21d: &mut [f32],
    b21e: &mut [f32],
    b22d: &mut [f32],
    b22e: &mut [f32],
    work: &mut [f32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::sbbcsd_(
        &(jobu1 as c_char),
        &(jobu2 as c_char),
        &(jobv1t as c_char),
        &(jobv2t as c_char),
        &(trans as c_char),
        &m,
        &p,
        q.as_ptr(),
        theta.as_mut_ptr(),
        phi.as_mut_ptr(),
        u1.as_mut_ptr(),
        &ldu1,
        u2.as_mut_ptr(),
        &ldu2,
        v1t.as_mut_ptr(),
        &ldv1t,
        v2t.as_mut_ptr(),
        &ldv2t,
        b11d.as_mut_ptr(),
        b11e.as_mut_ptr(),
        b12d.as_mut_ptr(),
        b12e.as_mut_ptr(),
        b21d.as_mut_ptr(),
        b21e.as_mut_ptr(),
        b22d.as_mut_ptr(),
        b22e.as_mut_ptr(),
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn zbbcsd(
    jobu1: u8,
    jobu2: u8,
    jobv1t: u8,
    jobv2t: u8,
    trans: u8,
    m: i32,
    p: i32,
    q: &[i32],
    theta: &mut [f64],
    phi: &mut [f64],
    u1: &mut [c64],
    ldu1: i32,
    u2: &mut [c64],
    ldu2: i32,
    v1t: &mut [c64],
    ldv1t: i32,
    v2t: &mut [c64],
    ldv2t: i32,
    b11d: &mut [f64],
    b11e: &mut [f64],
    b12d: &mut [f64],
    b12e: &mut [f64],
    b21d: &mut [f64],
    b21e: &mut [f64],
    b22d: &mut [f64],
    b22e: &mut [f64],
    rwork: &mut [f64],
    lrwork: i32,
    info: &mut i32,
) {
    ffi::zbbcsd_(
        &(jobu1 as c_char),
        &(jobu2 as c_char),
        &(jobv1t as c_char),
        &(jobv2t as c_char),
        &(trans as c_char),
        &m,
        &p,
        q.as_ptr(),
        theta.as_mut_ptr(),
        phi.as_mut_ptr(),
        u1.as_mut_ptr() as *mut _,
        &ldu1,
        u2.as_mut_ptr() as *mut _,
        &ldu2,
        v1t.as_mut_ptr() as *mut _,
        &ldv1t,
        v2t.as_mut_ptr() as *mut _,
        &ldv2t,
        b11d.as_mut_ptr(),
        b11e.as_mut_ptr(),
        b12d.as_mut_ptr(),
        b12e.as_mut_ptr(),
        b21d.as_mut_ptr(),
        b21e.as_mut_ptr(),
        b22d.as_mut_ptr(),
        b22e.as_mut_ptr(),
        rwork.as_mut_ptr(),
        &lrwork,
        info,
    )
}

#[inline]
pub unsafe fn dbdsdc(
    uplo: u8,
    compq: u8,
    n: i32,
    d: &mut [f64],
    e: &mut [f64],
    u: &mut [f64],
    ldu: i32,
    vt: &mut [f64],
    ldvt: i32,
    q: &mut [f64],
    iq: &mut [i32],
    work: &mut [f64],
    iwork: &mut [i32],
    info: &mut i32,
) {
    ffi::dbdsdc_(
        &(uplo as c_char),
        &(compq as c_char),
        &n,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        u.as_mut_ptr(),
        &ldu,
        vt.as_mut_ptr(),
        &ldvt,
        q.as_mut_ptr(),
        iq.as_mut_ptr(),
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn sbdsdc(
    uplo: u8,
    compq: u8,
    n: i32,
    d: &mut [f32],
    e: &mut [f32],
    u: &mut [f32],
    ldu: i32,
    vt: &mut [f32],
    ldvt: i32,
    q: &mut [f32],
    iq: &mut [i32],
    work: &mut [f32],
    iwork: &mut [i32],
    info: &mut i32,
) {
    ffi::sbdsdc_(
        &(uplo as c_char),
        &(compq as c_char),
        &n,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        u.as_mut_ptr(),
        &ldu,
        vt.as_mut_ptr(),
        &ldvt,
        q.as_mut_ptr(),
        iq.as_mut_ptr(),
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn cbdsqr(
    uplo: u8,
    n: i32,
    ncvt: &[i32],
    nru: &[i32],
    ncc: &[i32],
    d: &mut [f32],
    e: &mut [f32],
    vt: &mut [c32],
    ldvt: i32,
    u: &mut [c32],
    ldu: i32,
    c: &mut [c32],
    ldc: i32,
    rwork: &mut [f32],
    info: &mut i32,
) {
    ffi::cbdsqr_(
        &(uplo as c_char),
        &n,
        ncvt.as_ptr(),
        nru.as_ptr(),
        ncc.as_ptr(),
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        vt.as_mut_ptr() as *mut _,
        &ldvt,
        u.as_mut_ptr() as *mut _,
        &ldu,
        c.as_mut_ptr() as *mut _,
        &ldc,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn dbdsqr(
    uplo: u8,
    n: i32,
    ncvt: &[i32],
    nru: &[i32],
    ncc: &[i32],
    d: &mut [f64],
    e: &mut [f64],
    vt: &mut [f64],
    ldvt: i32,
    u: &mut [f64],
    ldu: i32,
    c: &mut [f64],
    ldc: i32,
    work: &mut [f64],
    info: &mut i32,
) {
    ffi::dbdsqr_(
        &(uplo as c_char),
        &n,
        ncvt.as_ptr(),
        nru.as_ptr(),
        ncc.as_ptr(),
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        vt.as_mut_ptr(),
        &ldvt,
        u.as_mut_ptr(),
        &ldu,
        c.as_mut_ptr(),
        &ldc,
        work.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn sbdsqr(
    uplo: u8,
    n: i32,
    ncvt: &[i32],
    nru: &[i32],
    ncc: &[i32],
    d: &mut [f32],
    e: &mut [f32],
    vt: &mut [f32],
    ldvt: i32,
    u: &mut [f32],
    ldu: i32,
    c: &mut [f32],
    ldc: i32,
    work: &mut [f32],
    info: &mut i32,
) {
    ffi::sbdsqr_(
        &(uplo as c_char),
        &n,
        ncvt.as_ptr(),
        nru.as_ptr(),
        ncc.as_ptr(),
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        vt.as_mut_ptr(),
        &ldvt,
        u.as_mut_ptr(),
        &ldu,
        c.as_mut_ptr(),
        &ldc,
        work.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn zbdsqr(
    uplo: u8,
    n: i32,
    ncvt: &[i32],
    nru: &[i32],
    ncc: &[i32],
    d: &mut [f64],
    e: &mut [f64],
    vt: &mut [c64],
    ldvt: i32,
    u: &mut [c64],
    ldu: i32,
    c: &mut [c64],
    ldc: i32,
    rwork: &mut [f64],
    info: &mut i32,
) {
    ffi::zbdsqr_(
        &(uplo as c_char),
        &n,
        ncvt.as_ptr(),
        nru.as_ptr(),
        ncc.as_ptr(),
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        vt.as_mut_ptr() as *mut _,
        &ldvt,
        u.as_mut_ptr() as *mut _,
        &ldu,
        c.as_mut_ptr() as *mut _,
        &ldc,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn dbdsvdx(
    uplo: u8,
    jobz: u8,
    range: u8,
    n: i32,
    d: &[f64],
    e: &[f64],
    vl: f64,
    vu: f64,
    il: i32,
    iu: i32,
    ns: &mut [i32],
    s: &mut [f64],
    z: &mut [f64],
    ldz: i32,
    work: &mut [f64],
    iwork: &mut [i32],
    info: &mut i32,
) {
    ffi::dbdsvdx_(
        &(uplo as c_char),
        &(jobz as c_char),
        &(range as c_char),
        &n,
        d.as_ptr(),
        e.as_ptr(),
        &vl,
        &vu,
        &il,
        &iu,
        ns.as_mut_ptr(),
        s.as_mut_ptr(),
        z.as_mut_ptr(),
        &ldz,
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn sbdsvdx(
    uplo: u8,
    jobz: u8,
    range: u8,
    n: i32,
    d: &[f32],
    e: &[f32],
    vl: f32,
    vu: f32,
    il: i32,
    iu: i32,
    ns: &mut [i32],
    s: &mut [f32],
    z: &mut [f32],
    ldz: i32,
    work: &mut [f32],
    iwork: &mut [i32],
    info: &mut i32,
) {
    ffi::sbdsvdx_(
        &(uplo as c_char),
        &(jobz as c_char),
        &(range as c_char),
        &n,
        d.as_ptr(),
        e.as_ptr(),
        &vl,
        &vu,
        &il,
        &iu,
        ns.as_mut_ptr(),
        s.as_mut_ptr(),
        z.as_mut_ptr(),
        &ldz,
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn ddisna(job: u8, m: i32, n: i32, d: &[f64], sep: &mut [f64], info: &mut i32) {
    ffi::ddisna_(&(job as c_char), &m, &n, d.as_ptr(), sep.as_mut_ptr(), info)
}

#[inline]
pub unsafe fn sdisna(job: u8, m: i32, n: i32, d: &[f32], sep: &mut [f32], info: &mut i32) {
    ffi::sdisna_(&(job as c_char), &m, &n, d.as_ptr(), sep.as_mut_ptr(), info)
}

#[inline]
pub unsafe fn cgbbrd(
    vect: u8,
    m: i32,
    n: i32,
    ncc: &[i32],
    kl: i32,
    ku: i32,
    ab: &mut [c32],
    ldab: i32,
    d: &mut [f32],
    e: &mut [f32],
    q: &mut [c32],
    ldq: i32,
    pt: &mut [c32],
    ldpt: i32,
    c: &mut [c32],
    ldc: i32,
    work: &mut [c32],
    rwork: &mut [f32],
    info: &mut i32,
) {
    ffi::cgbbrd_(
        &(vect as c_char),
        &m,
        &n,
        ncc.as_ptr(),
        &kl,
        &ku,
        ab.as_mut_ptr() as *mut _,
        &ldab,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        q.as_mut_ptr() as *mut _,
        &ldq,
        pt.as_mut_ptr() as *mut _,
        &ldpt,
        c.as_mut_ptr() as *mut _,
        &ldc,
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn dgbbrd(
    vect: u8,
    m: i32,
    n: i32,
    ncc: &[i32],
    kl: i32,
    ku: i32,
    ab: &mut [f64],
    ldab: i32,
    d: &mut [f64],
    e: &mut [f64],
    q: &mut [f64],
    ldq: i32,
    pt: &mut [f64],
    ldpt: i32,
    c: &mut [f64],
    ldc: i32,
    work: &mut [f64],
    info: &mut i32,
) {
    ffi::dgbbrd_(
        &(vect as c_char),
        &m,
        &n,
        ncc.as_ptr(),
        &kl,
        &ku,
        ab.as_mut_ptr(),
        &ldab,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        q.as_mut_ptr(),
        &ldq,
        pt.as_mut_ptr(),
        &ldpt,
        c.as_mut_ptr(),
        &ldc,
        work.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn sgbbrd(
    vect: u8,
    m: i32,
    n: i32,
    ncc: &[i32],
    kl: i32,
    ku: i32,
    ab: &mut [f32],
    ldab: i32,
    d: &mut [f32],
    e: &mut [f32],
    q: &mut [f32],
    ldq: i32,
    pt: &mut [f32],
    ldpt: i32,
    c: &mut [f32],
    ldc: i32,
    work: &mut [f32],
    info: &mut i32,
) {
    ffi::sgbbrd_(
        &(vect as c_char),
        &m,
        &n,
        ncc.as_ptr(),
        &kl,
        &ku,
        ab.as_mut_ptr(),
        &ldab,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        q.as_mut_ptr(),
        &ldq,
        pt.as_mut_ptr(),
        &ldpt,
        c.as_mut_ptr(),
        &ldc,
        work.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn zgbbrd(
    vect: u8,
    m: i32,
    n: i32,
    ncc: &[i32],
    kl: i32,
    ku: i32,
    ab: &mut [c64],
    ldab: i32,
    d: &mut [f64],
    e: &mut [f64],
    q: &mut [c64],
    ldq: i32,
    pt: &mut [c64],
    ldpt: i32,
    c: &mut [c64],
    ldc: i32,
    work: &mut [c64],
    rwork: &mut [f64],
    info: &mut i32,
) {
    ffi::zgbbrd_(
        &(vect as c_char),
        &m,
        &n,
        ncc.as_ptr(),
        &kl,
        &ku,
        ab.as_mut_ptr() as *mut _,
        &ldab,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        q.as_mut_ptr() as *mut _,
        &ldq,
        pt.as_mut_ptr() as *mut _,
        &ldpt,
        c.as_mut_ptr() as *mut _,
        &ldc,
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn cgbcon(
    norm: u8,
    n: i32,
    kl: i32,
    ku: i32,
    ab: &[c32],
    ldab: i32,
    ipiv: &[i32],
    anorm: f32,
    rcond: &mut f32,
    work: &mut [c32],
    rwork: &mut [f32],
    info: &mut i32,
) {
    ffi::cgbcon_(
        &(norm as c_char),
        &n,
        &kl,
        &ku,
        ab.as_ptr() as *const _,
        &ldab,
        ipiv.as_ptr(),
        &anorm,
        rcond,
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn dgbcon(
    norm: u8,
    n: i32,
    kl: i32,
    ku: i32,
    ab: &[f64],
    ldab: i32,
    ipiv: &[i32],
    anorm: f64,
    rcond: &mut f64,
    work: &mut [f64],
    iwork: &mut [i32],
    info: &mut i32,
) {
    ffi::dgbcon_(
        &(norm as c_char),
        &n,
        &kl,
        &ku,
        ab.as_ptr(),
        &ldab,
        ipiv.as_ptr(),
        &anorm,
        rcond,
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn sgbcon(
    norm: u8,
    n: i32,
    kl: i32,
    ku: i32,
    ab: &[f32],
    ldab: i32,
    ipiv: &[i32],
    anorm: f32,
    rcond: &mut f32,
    work: &mut [f32],
    iwork: &mut [i32],
    info: &mut i32,
) {
    ffi::sgbcon_(
        &(norm as c_char),
        &n,
        &kl,
        &ku,
        ab.as_ptr(),
        &ldab,
        ipiv.as_ptr(),
        &anorm,
        rcond,
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn zgbcon(
    norm: u8,
    n: i32,
    kl: i32,
    ku: i32,
    ab: &[c64],
    ldab: i32,
    ipiv: &[i32],
    anorm: f64,
    rcond: &mut f64,
    work: &mut [c64],
    rwork: &mut [f64],
    info: &mut i32,
) {
    ffi::zgbcon_(
        &(norm as c_char),
        &n,
        &kl,
        &ku,
        ab.as_ptr() as *const _,
        &ldab,
        ipiv.as_ptr(),
        &anorm,
        rcond,
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn cgbequ(
    m: i32,
    n: i32,
    kl: i32,
    ku: i32,
    ab: &[c32],
    ldab: i32,
    r: &mut [f32],
    c: &mut [f32],
    rowcnd: &mut f32,
    colcnd: &mut f32,
    amax: &mut f32,
    info: &mut i32,
) {
    ffi::cgbequ_(
        &m,
        &n,
        &kl,
        &ku,
        ab.as_ptr() as *const _,
        &ldab,
        r.as_mut_ptr(),
        c.as_mut_ptr(),
        rowcnd,
        colcnd,
        amax,
        info,
    )
}

#[inline]
pub unsafe fn dgbequ(
    m: i32,
    n: i32,
    kl: i32,
    ku: i32,
    ab: &[f64],
    ldab: i32,
    r: &mut [f64],
    c: &mut [f64],
    rowcnd: &mut f64,
    colcnd: &mut f64,
    amax: &mut f64,
    info: &mut i32,
) {
    ffi::dgbequ_(
        &m,
        &n,
        &kl,
        &ku,
        ab.as_ptr(),
        &ldab,
        r.as_mut_ptr(),
        c.as_mut_ptr(),
        rowcnd,
        colcnd,
        amax,
        info,
    )
}

#[inline]
pub unsafe fn sgbequ(
    m: i32,
    n: i32,
    kl: i32,
    ku: i32,
    ab: &[f32],
    ldab: i32,
    r: &mut [f32],
    c: &mut [f32],
    rowcnd: &mut f32,
    colcnd: &mut f32,
    amax: &mut f32,
    info: &mut i32,
) {
    ffi::sgbequ_(
        &m,
        &n,
        &kl,
        &ku,
        ab.as_ptr(),
        &ldab,
        r.as_mut_ptr(),
        c.as_mut_ptr(),
        rowcnd,
        colcnd,
        amax,
        info,
    )
}

#[inline]
pub unsafe fn zgbequ(
    m: i32,
    n: i32,
    kl: i32,
    ku: i32,
    ab: &[c64],
    ldab: i32,
    r: &mut [f64],
    c: &mut [f64],
    rowcnd: &mut f64,
    colcnd: &mut f64,
    amax: &mut f64,
    info: &mut i32,
) {
    ffi::zgbequ_(
        &m,
        &n,
        &kl,
        &ku,
        ab.as_ptr() as *const _,
        &ldab,
        r.as_mut_ptr(),
        c.as_mut_ptr(),
        rowcnd,
        colcnd,
        amax,
        info,
    )
}

#[inline]
pub unsafe fn cgbequb(
    m: i32,
    n: i32,
    kl: i32,
    ku: i32,
    ab: &[c32],
    ldab: i32,
    r: &mut [f32],
    c: &mut [f32],
    rowcnd: &mut f32,
    colcnd: &mut f32,
    amax: &mut f32,
    info: &mut i32,
) {
    ffi::cgbequb_(
        &m,
        &n,
        &kl,
        &ku,
        ab.as_ptr() as *const _,
        &ldab,
        r.as_mut_ptr(),
        c.as_mut_ptr(),
        rowcnd,
        colcnd,
        amax,
        info,
    )
}

#[inline]
pub unsafe fn dgbequb(
    m: i32,
    n: i32,
    kl: i32,
    ku: i32,
    ab: &[f64],
    ldab: i32,
    r: &mut [f64],
    c: &mut [f64],
    rowcnd: &mut f64,
    colcnd: &mut f64,
    amax: &mut f64,
    info: &mut i32,
) {
    ffi::dgbequb_(
        &m,
        &n,
        &kl,
        &ku,
        ab.as_ptr(),
        &ldab,
        r.as_mut_ptr(),
        c.as_mut_ptr(),
        rowcnd,
        colcnd,
        amax,
        info,
    )
}

#[inline]
pub unsafe fn sgbequb(
    m: i32,
    n: i32,
    kl: i32,
    ku: i32,
    ab: &[f32],
    ldab: i32,
    r: &mut [f32],
    c: &mut [f32],
    rowcnd: &mut f32,
    colcnd: &mut f32,
    amax: &mut f32,
    info: &mut i32,
) {
    ffi::sgbequb_(
        &m,
        &n,
        &kl,
        &ku,
        ab.as_ptr(),
        &ldab,
        r.as_mut_ptr(),
        c.as_mut_ptr(),
        rowcnd,
        colcnd,
        amax,
        info,
    )
}

#[inline]
pub unsafe fn zgbequb(
    m: i32,
    n: i32,
    kl: i32,
    ku: i32,
    ab: &[c64],
    ldab: i32,
    r: &mut [f64],
    c: &mut [f64],
    rowcnd: &mut f64,
    colcnd: &mut f64,
    amax: &mut f64,
    info: &mut i32,
) {
    ffi::zgbequb_(
        &m,
        &n,
        &kl,
        &ku,
        ab.as_ptr() as *const _,
        &ldab,
        r.as_mut_ptr(),
        c.as_mut_ptr(),
        rowcnd,
        colcnd,
        amax,
        info,
    )
}

#[inline]
pub unsafe fn cgbrfs(
    trans: u8,
    n: i32,
    kl: i32,
    ku: i32,
    nrhs: i32,
    ab: &[c32],
    ldab: i32,
    afb: &[c32],
    ldafb: i32,
    ipiv: &[i32],
    b: &[c32],
    ldb: i32,
    x: &mut [c32],
    ldx: i32,
    ferr: &mut [f32],
    berr: &mut [f32],
    work: &mut [c32],
    rwork: &mut [f32],
    info: &mut i32,
) {
    ffi::cgbrfs_(
        &(trans as c_char),
        &n,
        &kl,
        &ku,
        &nrhs,
        ab.as_ptr() as *const _,
        &ldab,
        afb.as_ptr() as *const _,
        &ldafb,
        ipiv.as_ptr(),
        b.as_ptr() as *const _,
        &ldb,
        x.as_mut_ptr() as *mut _,
        &ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn dgbrfs(
    trans: u8,
    n: i32,
    kl: i32,
    ku: i32,
    nrhs: i32,
    ab: &[f64],
    ldab: i32,
    afb: &[f64],
    ldafb: i32,
    ipiv: &[i32],
    b: &[f64],
    ldb: i32,
    x: &mut [f64],
    ldx: i32,
    ferr: &mut [f64],
    berr: &mut [f64],
    work: &mut [f64],
    iwork: &mut [i32],
    info: &mut i32,
) {
    ffi::dgbrfs_(
        &(trans as c_char),
        &n,
        &kl,
        &ku,
        &nrhs,
        ab.as_ptr(),
        &ldab,
        afb.as_ptr(),
        &ldafb,
        ipiv.as_ptr(),
        b.as_ptr(),
        &ldb,
        x.as_mut_ptr(),
        &ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn sgbrfs(
    trans: u8,
    n: i32,
    kl: i32,
    ku: i32,
    nrhs: i32,
    ab: &[f32],
    ldab: i32,
    afb: &[f32],
    ldafb: i32,
    ipiv: &[i32],
    b: &[f32],
    ldb: i32,
    x: &mut [f32],
    ldx: i32,
    ferr: &mut [f32],
    berr: &mut [f32],
    work: &mut [f32],
    iwork: &mut [i32],
    info: &mut i32,
) {
    ffi::sgbrfs_(
        &(trans as c_char),
        &n,
        &kl,
        &ku,
        &nrhs,
        ab.as_ptr(),
        &ldab,
        afb.as_ptr(),
        &ldafb,
        ipiv.as_ptr(),
        b.as_ptr(),
        &ldb,
        x.as_mut_ptr(),
        &ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn zgbrfs(
    trans: u8,
    n: i32,
    kl: i32,
    ku: i32,
    nrhs: i32,
    ab: &[c64],
    ldab: i32,
    afb: &[c64],
    ldafb: i32,
    ipiv: &[i32],
    b: &[c64],
    ldb: i32,
    x: &mut [c64],
    ldx: i32,
    ferr: &mut [f64],
    berr: &mut [f64],
    work: &mut [c64],
    rwork: &mut [f64],
    info: &mut i32,
) {
    ffi::zgbrfs_(
        &(trans as c_char),
        &n,
        &kl,
        &ku,
        &nrhs,
        ab.as_ptr() as *const _,
        &ldab,
        afb.as_ptr() as *const _,
        &ldafb,
        ipiv.as_ptr(),
        b.as_ptr() as *const _,
        &ldb,
        x.as_mut_ptr() as *mut _,
        &ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn cgbrfsx(
    trans: u8,
    equed: u8,
    n: i32,
    kl: i32,
    ku: i32,
    nrhs: i32,
    ab: &[c32],
    ldab: i32,
    afb: &[c32],
    ldafb: i32,
    ipiv: &[i32],
    r: &mut [f32],
    c: &mut [f32],
    b: &[c32],
    ldb: i32,
    x: &mut [c32],
    ldx: i32,
    rcond: &mut f32,
    berr: &mut [f32],
    n_err_bnds: i32,
    err_bnds_norm: &mut [f32],
    err_bnds_comp: &mut [f32],
    nparams: &[i32],
    params: &mut [f32],
    work: &mut [c32],
    rwork: &mut [f32],
    info: &mut i32,
) {
    ffi::cgbrfsx_(
        &(trans as c_char),
        &(equed as c_char),
        &n,
        &kl,
        &ku,
        &nrhs,
        ab.as_ptr() as *const _,
        &ldab,
        afb.as_ptr() as *const _,
        &ldafb,
        ipiv.as_ptr(),
        r.as_mut_ptr(),
        c.as_mut_ptr(),
        b.as_ptr() as *const _,
        &ldb,
        x.as_mut_ptr() as *mut _,
        &ldx,
        rcond,
        berr.as_mut_ptr(),
        &n_err_bnds,
        err_bnds_norm.as_mut_ptr(),
        err_bnds_comp.as_mut_ptr(),
        nparams.as_ptr(),
        params.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn dgbrfsx(
    trans: u8,
    equed: u8,
    n: i32,
    kl: i32,
    ku: i32,
    nrhs: i32,
    ab: &[f64],
    ldab: i32,
    afb: &[f64],
    ldafb: i32,
    ipiv: &[i32],
    r: &mut [f64],
    c: &mut [f64],
    b: &[f64],
    ldb: i32,
    x: &mut [f64],
    ldx: i32,
    rcond: &mut f64,
    berr: &mut [f64],
    n_err_bnds: i32,
    err_bnds_norm: &mut [f64],
    err_bnds_comp: &mut [f64],
    nparams: &[i32],
    params: &mut [f64],
    work: &mut [f64],
    iwork: &mut [i32],
    info: &mut i32,
) {
    ffi::dgbrfsx_(
        &(trans as c_char),
        &(equed as c_char),
        &n,
        &kl,
        &ku,
        &nrhs,
        ab.as_ptr(),
        &ldab,
        afb.as_ptr(),
        &ldafb,
        ipiv.as_ptr(),
        r.as_mut_ptr(),
        c.as_mut_ptr(),
        b.as_ptr(),
        &ldb,
        x.as_mut_ptr(),
        &ldx,
        rcond,
        berr.as_mut_ptr(),
        &n_err_bnds,
        err_bnds_norm.as_mut_ptr(),
        err_bnds_comp.as_mut_ptr(),
        nparams.as_ptr(),
        params.as_mut_ptr(),
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn sgbrfsx(
    trans: u8,
    equed: u8,
    n: i32,
    kl: i32,
    ku: i32,
    nrhs: i32,
    ab: &[f32],
    ldab: i32,
    afb: &[f32],
    ldafb: i32,
    ipiv: &[i32],
    r: &mut [f32],
    c: &mut [f32],
    b: &[f32],
    ldb: i32,
    x: &mut [f32],
    ldx: i32,
    rcond: &mut f32,
    berr: &mut [f32],
    n_err_bnds: i32,
    err_bnds_norm: &mut [f32],
    err_bnds_comp: &mut [f32],
    nparams: &[i32],
    params: &mut [f32],
    work: &mut [f32],
    iwork: &mut [i32],
    info: &mut i32,
) {
    ffi::sgbrfsx_(
        &(trans as c_char),
        &(equed as c_char),
        &n,
        &kl,
        &ku,
        &nrhs,
        ab.as_ptr(),
        &ldab,
        afb.as_ptr(),
        &ldafb,
        ipiv.as_ptr(),
        r.as_mut_ptr(),
        c.as_mut_ptr(),
        b.as_ptr(),
        &ldb,
        x.as_mut_ptr(),
        &ldx,
        rcond,
        berr.as_mut_ptr(),
        &n_err_bnds,
        err_bnds_norm.as_mut_ptr(),
        err_bnds_comp.as_mut_ptr(),
        nparams.as_ptr(),
        params.as_mut_ptr(),
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn zgbrfsx(
    trans: u8,
    equed: u8,
    n: i32,
    kl: i32,
    ku: i32,
    nrhs: i32,
    ab: &[c64],
    ldab: i32,
    afb: &[c64],
    ldafb: i32,
    ipiv: &[i32],
    r: &mut [f64],
    c: &mut [f64],
    b: &[c64],
    ldb: i32,
    x: &mut [c64],
    ldx: i32,
    rcond: &mut f64,
    berr: &mut [f64],
    n_err_bnds: i32,
    err_bnds_norm: &mut [f64],
    err_bnds_comp: &mut [f64],
    nparams: &[i32],
    params: &mut [f64],
    work: &mut [c64],
    rwork: &mut [f64],
    info: &mut i32,
) {
    ffi::zgbrfsx_(
        &(trans as c_char),
        &(equed as c_char),
        &n,
        &kl,
        &ku,
        &nrhs,
        ab.as_ptr() as *const _,
        &ldab,
        afb.as_ptr() as *const _,
        &ldafb,
        ipiv.as_ptr(),
        r.as_mut_ptr(),
        c.as_mut_ptr(),
        b.as_ptr() as *const _,
        &ldb,
        x.as_mut_ptr() as *mut _,
        &ldx,
        rcond,
        berr.as_mut_ptr(),
        &n_err_bnds,
        err_bnds_norm.as_mut_ptr(),
        err_bnds_comp.as_mut_ptr(),
        nparams.as_ptr(),
        params.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn cgbsv(
    n: i32,
    kl: i32,
    ku: i32,
    nrhs: i32,
    ab: &mut [c32],
    ldab: i32,
    ipiv: &mut [i32],
    b: &mut [c32],
    ldb: i32,
    info: &mut i32,
) {
    ffi::cgbsv_(
        &n,
        &kl,
        &ku,
        &nrhs,
        ab.as_mut_ptr() as *mut _,
        &ldab,
        ipiv.as_mut_ptr(),
        b.as_mut_ptr() as *mut _,
        &ldb,
        info,
    )
}

#[inline]
pub unsafe fn dgbsv(
    n: i32,
    kl: i32,
    ku: i32,
    nrhs: i32,
    ab: &mut [f64],
    ldab: i32,
    ipiv: &mut [i32],
    b: &mut [f64],
    ldb: i32,
    info: &mut i32,
) {
    ffi::dgbsv_(
        &n,
        &kl,
        &ku,
        &nrhs,
        ab.as_mut_ptr(),
        &ldab,
        ipiv.as_mut_ptr(),
        b.as_mut_ptr(),
        &ldb,
        info,
    )
}

#[inline]
pub unsafe fn sgbsv(
    n: i32,
    kl: i32,
    ku: i32,
    nrhs: i32,
    ab: &mut [f32],
    ldab: i32,
    ipiv: &mut [i32],
    b: &mut [f32],
    ldb: i32,
    info: &mut i32,
) {
    ffi::sgbsv_(
        &n,
        &kl,
        &ku,
        &nrhs,
        ab.as_mut_ptr(),
        &ldab,
        ipiv.as_mut_ptr(),
        b.as_mut_ptr(),
        &ldb,
        info,
    )
}

#[inline]
pub unsafe fn zgbsv(
    n: i32,
    kl: i32,
    ku: i32,
    nrhs: i32,
    ab: &mut [c64],
    ldab: i32,
    ipiv: &mut [i32],
    b: &mut [c64],
    ldb: i32,
    info: &mut i32,
) {
    ffi::zgbsv_(
        &n,
        &kl,
        &ku,
        &nrhs,
        ab.as_mut_ptr() as *mut _,
        &ldab,
        ipiv.as_mut_ptr(),
        b.as_mut_ptr() as *mut _,
        &ldb,
        info,
    )
}

#[inline]
pub unsafe fn cgbsvx(
    fact: u8,
    trans: u8,
    n: i32,
    kl: i32,
    ku: i32,
    nrhs: i32,
    ab: &mut [c32],
    ldab: i32,
    afb: &mut [c32],
    ldafb: i32,
    ipiv: &mut [i32],
    equed: &mut u8,
    r: &mut [f32],
    c: &mut [f32],
    b: &mut [c32],
    ldb: i32,
    x: &mut [c32],
    ldx: i32,
    rcond: &mut f32,
    ferr: &mut [f32],
    berr: &mut [f32],
    work: &mut [c32],
    rwork: &mut [f32],
    info: &mut i32,
) {
    ffi::cgbsvx_(
        &(fact as c_char),
        &(trans as c_char),
        &n,
        &kl,
        &ku,
        &nrhs,
        ab.as_mut_ptr() as *mut _,
        &ldab,
        afb.as_mut_ptr() as *mut _,
        &ldafb,
        ipiv.as_mut_ptr(),
        equed as *mut _ as *mut _,
        r.as_mut_ptr(),
        c.as_mut_ptr(),
        b.as_mut_ptr() as *mut _,
        &ldb,
        x.as_mut_ptr() as *mut _,
        &ldx,
        rcond,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn dgbsvx(
    fact: u8,
    trans: u8,
    n: i32,
    kl: i32,
    ku: i32,
    nrhs: i32,
    ab: &mut [f64],
    ldab: i32,
    afb: &mut [f64],
    ldafb: i32,
    ipiv: &mut [i32],
    equed: &mut u8,
    r: &mut [f64],
    c: &mut [f64],
    b: &mut [f64],
    ldb: i32,
    x: &mut [f64],
    ldx: i32,
    rcond: &mut f64,
    ferr: &mut [f64],
    berr: &mut [f64],
    work: &mut [f64],
    iwork: &mut [i32],
    info: &mut i32,
) {
    ffi::dgbsvx_(
        &(fact as c_char),
        &(trans as c_char),
        &n,
        &kl,
        &ku,
        &nrhs,
        ab.as_mut_ptr(),
        &ldab,
        afb.as_mut_ptr(),
        &ldafb,
        ipiv.as_mut_ptr(),
        equed as *mut _ as *mut _,
        r.as_mut_ptr(),
        c.as_mut_ptr(),
        b.as_mut_ptr(),
        &ldb,
        x.as_mut_ptr(),
        &ldx,
        rcond,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn sgbsvx(
    fact: u8,
    trans: u8,
    n: i32,
    kl: i32,
    ku: i32,
    nrhs: i32,
    ab: &mut [f32],
    ldab: i32,
    afb: &mut [f32],
    ldafb: i32,
    ipiv: &mut [i32],
    equed: &mut u8,
    r: &mut [f32],
    c: &mut [f32],
    b: &mut [f32],
    ldb: i32,
    x: &mut [f32],
    ldx: i32,
    rcond: &mut f32,
    ferr: &mut [f32],
    berr: &mut [f32],
    work: &mut [f32],
    iwork: &mut [i32],
    info: &mut i32,
) {
    ffi::sgbsvx_(
        &(fact as c_char),
        &(trans as c_char),
        &n,
        &kl,
        &ku,
        &nrhs,
        ab.as_mut_ptr(),
        &ldab,
        afb.as_mut_ptr(),
        &ldafb,
        ipiv.as_mut_ptr(),
        equed as *mut _ as *mut _,
        r.as_mut_ptr(),
        c.as_mut_ptr(),
        b.as_mut_ptr(),
        &ldb,
        x.as_mut_ptr(),
        &ldx,
        rcond,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn zgbsvx(
    fact: u8,
    trans: u8,
    n: i32,
    kl: i32,
    ku: i32,
    nrhs: i32,
    ab: &mut [c64],
    ldab: i32,
    afb: &mut [c64],
    ldafb: i32,
    ipiv: &mut [i32],
    equed: &mut u8,
    r: &mut [f64],
    c: &mut [f64],
    b: &mut [c64],
    ldb: i32,
    x: &mut [c64],
    ldx: i32,
    rcond: &mut f64,
    ferr: &mut [f64],
    berr: &mut [f64],
    work: &mut [c64],
    rwork: &mut [f64],
    info: &mut i32,
) {
    ffi::zgbsvx_(
        &(fact as c_char),
        &(trans as c_char),
        &n,
        &kl,
        &ku,
        &nrhs,
        ab.as_mut_ptr() as *mut _,
        &ldab,
        afb.as_mut_ptr() as *mut _,
        &ldafb,
        ipiv.as_mut_ptr(),
        equed as *mut _ as *mut _,
        r.as_mut_ptr(),
        c.as_mut_ptr(),
        b.as_mut_ptr() as *mut _,
        &ldb,
        x.as_mut_ptr() as *mut _,
        &ldx,
        rcond,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn cgbsvxx(
    fact: u8,
    trans: u8,
    n: i32,
    kl: i32,
    ku: i32,
    nrhs: i32,
    ab: &mut [c32],
    ldab: i32,
    afb: &mut [c32],
    ldafb: i32,
    ipiv: &mut [i32],
    equed: &mut u8,
    r: &mut [f32],
    c: &mut [f32],
    b: &mut [c32],
    ldb: i32,
    x: &mut [c32],
    ldx: i32,
    rcond: &mut f32,
    rpvgrw: &mut f32,
    berr: &mut [f32],
    n_err_bnds: i32,
    err_bnds_norm: &mut [f32],
    err_bnds_comp: &mut [f32],
    nparams: &[i32],
    params: &mut [f32],
    work: &mut [c32],
    rwork: &mut [f32],
    info: &mut i32,
) {
    ffi::cgbsvxx_(
        &(fact as c_char),
        &(trans as c_char),
        &n,
        &kl,
        &ku,
        &nrhs,
        ab.as_mut_ptr() as *mut _,
        &ldab,
        afb.as_mut_ptr() as *mut _,
        &ldafb,
        ipiv.as_mut_ptr(),
        equed as *mut _ as *mut _,
        r.as_mut_ptr(),
        c.as_mut_ptr(),
        b.as_mut_ptr() as *mut _,
        &ldb,
        x.as_mut_ptr() as *mut _,
        &ldx,
        rcond,
        rpvgrw,
        berr.as_mut_ptr(),
        &n_err_bnds,
        err_bnds_norm.as_mut_ptr(),
        err_bnds_comp.as_mut_ptr(),
        nparams.as_ptr(),
        params.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn dgbsvxx(
    fact: u8,
    trans: u8,
    n: i32,
    kl: i32,
    ku: i32,
    nrhs: i32,
    ab: &mut [f64],
    ldab: i32,
    afb: &mut [f64],
    ldafb: i32,
    ipiv: &mut [i32],
    equed: &mut u8,
    r: &mut [f64],
    c: &mut [f64],
    b: &mut [f64],
    ldb: i32,
    x: &mut [f64],
    ldx: i32,
    rcond: &mut f64,
    rpvgrw: &mut f64,
    berr: &mut [f64],
    n_err_bnds: i32,
    err_bnds_norm: &mut [f64],
    err_bnds_comp: &mut [f64],
    nparams: &[i32],
    params: &mut [f64],
    work: &mut [f64],
    iwork: &mut [i32],
    info: &mut i32,
) {
    ffi::dgbsvxx_(
        &(fact as c_char),
        &(trans as c_char),
        &n,
        &kl,
        &ku,
        &nrhs,
        ab.as_mut_ptr(),
        &ldab,
        afb.as_mut_ptr(),
        &ldafb,
        ipiv.as_mut_ptr(),
        equed as *mut _ as *mut _,
        r.as_mut_ptr(),
        c.as_mut_ptr(),
        b.as_mut_ptr(),
        &ldb,
        x.as_mut_ptr(),
        &ldx,
        rcond,
        rpvgrw,
        berr.as_mut_ptr(),
        &n_err_bnds,
        err_bnds_norm.as_mut_ptr(),
        err_bnds_comp.as_mut_ptr(),
        nparams.as_ptr(),
        params.as_mut_ptr(),
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn sgbsvxx(
    fact: u8,
    trans: u8,
    n: i32,
    kl: i32,
    ku: i32,
    nrhs: i32,
    ab: &mut [f32],
    ldab: i32,
    afb: &mut [f32],
    ldafb: i32,
    ipiv: &mut [i32],
    equed: &mut u8,
    r: &mut [f32],
    c: &mut [f32],
    b: &mut [f32],
    ldb: i32,
    x: &mut [f32],
    ldx: i32,
    rcond: &mut f32,
    rpvgrw: &mut f32,
    berr: &mut [f32],
    n_err_bnds: i32,
    err_bnds_norm: &mut [f32],
    err_bnds_comp: &mut [f32],
    nparams: &[i32],
    params: &mut [f32],
    work: &mut [f32],
    iwork: &mut [i32],
    info: &mut i32,
) {
    ffi::sgbsvxx_(
        &(fact as c_char),
        &(trans as c_char),
        &n,
        &kl,
        &ku,
        &nrhs,
        ab.as_mut_ptr(),
        &ldab,
        afb.as_mut_ptr(),
        &ldafb,
        ipiv.as_mut_ptr(),
        equed as *mut _ as *mut _,
        r.as_mut_ptr(),
        c.as_mut_ptr(),
        b.as_mut_ptr(),
        &ldb,
        x.as_mut_ptr(),
        &ldx,
        rcond,
        rpvgrw,
        berr.as_mut_ptr(),
        &n_err_bnds,
        err_bnds_norm.as_mut_ptr(),
        err_bnds_comp.as_mut_ptr(),
        nparams.as_ptr(),
        params.as_mut_ptr(),
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn zgbsvxx(
    fact: u8,
    trans: u8,
    n: i32,
    kl: i32,
    ku: i32,
    nrhs: i32,
    ab: &mut [c64],
    ldab: i32,
    afb: &mut [c64],
    ldafb: i32,
    ipiv: &mut [i32],
    equed: &mut u8,
    r: &mut [f64],
    c: &mut [f64],
    b: &mut [c64],
    ldb: i32,
    x: &mut [c64],
    ldx: i32,
    rcond: &mut f64,
    rpvgrw: &mut f64,
    berr: &mut [f64],
    n_err_bnds: i32,
    err_bnds_norm: &mut [f64],
    err_bnds_comp: &mut [f64],
    nparams: &[i32],
    params: &mut [f64],
    work: &mut [c64],
    rwork: &mut [f64],
    info: &mut i32,
) {
    ffi::zgbsvxx_(
        &(fact as c_char),
        &(trans as c_char),
        &n,
        &kl,
        &ku,
        &nrhs,
        ab.as_mut_ptr() as *mut _,
        &ldab,
        afb.as_mut_ptr() as *mut _,
        &ldafb,
        ipiv.as_mut_ptr(),
        equed as *mut _ as *mut _,
        r.as_mut_ptr(),
        c.as_mut_ptr(),
        b.as_mut_ptr() as *mut _,
        &ldb,
        x.as_mut_ptr() as *mut _,
        &ldx,
        rcond,
        rpvgrw,
        berr.as_mut_ptr(),
        &n_err_bnds,
        err_bnds_norm.as_mut_ptr(),
        err_bnds_comp.as_mut_ptr(),
        nparams.as_ptr(),
        params.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn cgbtrf(
    m: i32,
    n: i32,
    kl: i32,
    ku: i32,
    ab: &mut [c32],
    ldab: i32,
    ipiv: &mut [i32],
    info: &mut i32,
) {
    ffi::cgbtrf_(
        &m,
        &n,
        &kl,
        &ku,
        ab.as_mut_ptr() as *mut _,
        &ldab,
        ipiv.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn dgbtrf(
    m: i32,
    n: i32,
    kl: i32,
    ku: i32,
    ab: &mut [f64],
    ldab: i32,
    ipiv: &mut [i32],
    info: &mut i32,
) {
    ffi::dgbtrf_(
        &m,
        &n,
        &kl,
        &ku,
        ab.as_mut_ptr(),
        &ldab,
        ipiv.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn sgbtrf(
    m: i32,
    n: i32,
    kl: i32,
    ku: i32,
    ab: &mut [f32],
    ldab: i32,
    ipiv: &mut [i32],
    info: &mut i32,
) {
    ffi::sgbtrf_(
        &m,
        &n,
        &kl,
        &ku,
        ab.as_mut_ptr(),
        &ldab,
        ipiv.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn zgbtrf(
    m: i32,
    n: i32,
    kl: i32,
    ku: i32,
    ab: &mut [c64],
    ldab: i32,
    ipiv: &mut [i32],
    info: &mut i32,
) {
    ffi::zgbtrf_(
        &m,
        &n,
        &kl,
        &ku,
        ab.as_mut_ptr() as *mut _,
        &ldab,
        ipiv.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn cgbtrs(
    trans: u8,
    n: i32,
    kl: i32,
    ku: i32,
    nrhs: i32,
    ab: &[c32],
    ldab: i32,
    ipiv: &[i32],
    b: &mut [c32],
    ldb: i32,
    info: &mut i32,
) {
    ffi::cgbtrs_(
        &(trans as c_char),
        &n,
        &kl,
        &ku,
        &nrhs,
        ab.as_ptr() as *const _,
        &ldab,
        ipiv.as_ptr(),
        b.as_mut_ptr() as *mut _,
        &ldb,
        info,
    )
}

#[inline]
pub unsafe fn dgbtrs(
    trans: u8,
    n: i32,
    kl: i32,
    ku: i32,
    nrhs: i32,
    ab: &[f64],
    ldab: i32,
    ipiv: &[i32],
    b: &mut [f64],
    ldb: i32,
    info: &mut i32,
) {
    ffi::dgbtrs_(
        &(trans as c_char),
        &n,
        &kl,
        &ku,
        &nrhs,
        ab.as_ptr(),
        &ldab,
        ipiv.as_ptr(),
        b.as_mut_ptr(),
        &ldb,
        info,
    )
}

#[inline]
pub unsafe fn sgbtrs(
    trans: u8,
    n: i32,
    kl: i32,
    ku: i32,
    nrhs: i32,
    ab: &[f32],
    ldab: i32,
    ipiv: &[i32],
    b: &mut [f32],
    ldb: i32,
    info: &mut i32,
) {
    ffi::sgbtrs_(
        &(trans as c_char),
        &n,
        &kl,
        &ku,
        &nrhs,
        ab.as_ptr(),
        &ldab,
        ipiv.as_ptr(),
        b.as_mut_ptr(),
        &ldb,
        info,
    )
}

#[inline]
pub unsafe fn zgbtrs(
    trans: u8,
    n: i32,
    kl: i32,
    ku: i32,
    nrhs: i32,
    ab: &[c64],
    ldab: i32,
    ipiv: &[i32],
    b: &mut [c64],
    ldb: i32,
    info: &mut i32,
) {
    ffi::zgbtrs_(
        &(trans as c_char),
        &n,
        &kl,
        &ku,
        &nrhs,
        ab.as_ptr() as *const _,
        &ldab,
        ipiv.as_ptr(),
        b.as_mut_ptr() as *mut _,
        &ldb,
        info,
    )
}

#[inline]
pub unsafe fn cgebak(
    job: u8,
    side: u8,
    n: i32,
    ilo: i32,
    ihi: i32,
    scale: &[f32],
    m: i32,
    v: &mut [c32],
    ldv: i32,
    info: &mut i32,
) {
    ffi::cgebak_(
        &(job as c_char),
        &(side as c_char),
        &n,
        &ilo,
        &ihi,
        scale.as_ptr(),
        &m,
        v.as_mut_ptr() as *mut _,
        &ldv,
        info,
    )
}

#[inline]
pub unsafe fn dgebak(
    job: u8,
    side: u8,
    n: i32,
    ilo: i32,
    ihi: i32,
    scale: &[f64],
    m: i32,
    v: &mut [f64],
    ldv: i32,
    info: &mut i32,
) {
    ffi::dgebak_(
        &(job as c_char),
        &(side as c_char),
        &n,
        &ilo,
        &ihi,
        scale.as_ptr(),
        &m,
        v.as_mut_ptr(),
        &ldv,
        info,
    )
}

#[inline]
pub unsafe fn sgebak(
    job: u8,
    side: u8,
    n: i32,
    ilo: i32,
    ihi: i32,
    scale: &[f32],
    m: i32,
    v: &mut [f32],
    ldv: i32,
    info: &mut i32,
) {
    ffi::sgebak_(
        &(job as c_char),
        &(side as c_char),
        &n,
        &ilo,
        &ihi,
        scale.as_ptr(),
        &m,
        v.as_mut_ptr(),
        &ldv,
        info,
    )
}

#[inline]
pub unsafe fn zgebak(
    job: u8,
    side: u8,
    n: i32,
    ilo: i32,
    ihi: i32,
    scale: &[f64],
    m: i32,
    v: &mut [c64],
    ldv: i32,
    info: &mut i32,
) {
    ffi::zgebak_(
        &(job as c_char),
        &(side as c_char),
        &n,
        &ilo,
        &ihi,
        scale.as_ptr(),
        &m,
        v.as_mut_ptr() as *mut _,
        &ldv,
        info,
    )
}

#[inline]
pub unsafe fn cgebal(
    job: u8,
    n: i32,
    a: &mut [c32],
    lda: i32,
    ilo: &mut i32,
    ihi: &mut i32,
    scale: &mut [f32],
    info: &mut i32,
) {
    ffi::cgebal_(
        &(job as c_char),
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        ilo,
        ihi,
        scale.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn dgebal(
    job: u8,
    n: i32,
    a: &mut [f64],
    lda: i32,
    ilo: &mut i32,
    ihi: &mut i32,
    scale: &mut [f64],
    info: &mut i32,
) {
    ffi::dgebal_(
        &(job as c_char),
        &n,
        a.as_mut_ptr(),
        &lda,
        ilo,
        ihi,
        scale.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn sgebal(
    job: u8,
    n: i32,
    a: &mut [f32],
    lda: i32,
    ilo: &mut i32,
    ihi: &mut i32,
    scale: &mut [f32],
    info: &mut i32,
) {
    ffi::sgebal_(
        &(job as c_char),
        &n,
        a.as_mut_ptr(),
        &lda,
        ilo,
        ihi,
        scale.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn zgebal(
    job: u8,
    n: i32,
    a: &mut [c64],
    lda: i32,
    ilo: &mut i32,
    ihi: &mut i32,
    scale: &mut [f64],
    info: &mut i32,
) {
    ffi::zgebal_(
        &(job as c_char),
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        ilo,
        ihi,
        scale.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn cgebrd(
    m: i32,
    n: i32,
    a: &mut [c32],
    lda: i32,
    d: &mut [f32],
    e: &mut [f32],
    tauq: &mut [c32],
    taup: &mut [c32],
    work: &mut [c32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::cgebrd_(
        &m,
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        tauq.as_mut_ptr() as *mut _,
        taup.as_mut_ptr() as *mut _,
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn dgebrd(
    m: i32,
    n: i32,
    a: &mut [f64],
    lda: i32,
    d: &mut [f64],
    e: &mut [f64],
    tauq: &mut [f64],
    taup: &mut [f64],
    work: &mut [f64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::dgebrd_(
        &m,
        &n,
        a.as_mut_ptr(),
        &lda,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        tauq.as_mut_ptr(),
        taup.as_mut_ptr(),
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn sgebrd(
    m: i32,
    n: i32,
    a: &mut [f32],
    lda: i32,
    d: &mut [f32],
    e: &mut [f32],
    tauq: &mut [f32],
    taup: &mut [f32],
    work: &mut [f32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::sgebrd_(
        &m,
        &n,
        a.as_mut_ptr(),
        &lda,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        tauq.as_mut_ptr(),
        taup.as_mut_ptr(),
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn zgebrd(
    m: i32,
    n: i32,
    a: &mut [c64],
    lda: i32,
    d: &mut [f64],
    e: &mut [f64],
    tauq: &mut [c64],
    taup: &mut [c64],
    work: &mut [c64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::zgebrd_(
        &m,
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        tauq.as_mut_ptr() as *mut _,
        taup.as_mut_ptr() as *mut _,
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn cgecon(
    norm: u8,
    n: i32,
    a: &[c32],
    lda: i32,
    anorm: f32,
    rcond: &mut f32,
    work: &mut [c32],
    rwork: &mut [f32],
    info: &mut i32,
) {
    ffi::cgecon_(
        &(norm as c_char),
        &n,
        a.as_ptr() as *const _,
        &lda,
        &anorm,
        rcond,
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn dgecon(
    norm: u8,
    n: i32,
    a: &[f64],
    lda: i32,
    anorm: f64,
    rcond: &mut f64,
    work: &mut [f64],
    iwork: &mut [i32],
    info: &mut i32,
) {
    ffi::dgecon_(
        &(norm as c_char),
        &n,
        a.as_ptr(),
        &lda,
        &anorm,
        rcond,
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn sgecon(
    norm: u8,
    n: i32,
    a: &[f32],
    lda: i32,
    anorm: f32,
    rcond: &mut f32,
    work: &mut [f32],
    iwork: &mut [i32],
    info: &mut i32,
) {
    ffi::sgecon_(
        &(norm as c_char),
        &n,
        a.as_ptr(),
        &lda,
        &anorm,
        rcond,
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn zgecon(
    norm: u8,
    n: i32,
    a: &[c64],
    lda: i32,
    anorm: f64,
    rcond: &mut f64,
    work: &mut [c64],
    rwork: &mut [f64],
    info: &mut i32,
) {
    ffi::zgecon_(
        &(norm as c_char),
        &n,
        a.as_ptr() as *const _,
        &lda,
        &anorm,
        rcond,
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn cgeequ(
    m: i32,
    n: i32,
    a: &[c32],
    lda: i32,
    r: &mut [f32],
    c: &mut [f32],
    rowcnd: &mut f32,
    colcnd: &mut f32,
    amax: &mut f32,
    info: &mut i32,
) {
    ffi::cgeequ_(
        &m,
        &n,
        a.as_ptr() as *const _,
        &lda,
        r.as_mut_ptr(),
        c.as_mut_ptr(),
        rowcnd,
        colcnd,
        amax,
        info,
    )
}

#[inline]
pub unsafe fn dgeequ(
    m: i32,
    n: i32,
    a: &[f64],
    lda: i32,
    r: &mut [f64],
    c: &mut [f64],
    rowcnd: &mut f64,
    colcnd: &mut f64,
    amax: &mut f64,
    info: &mut i32,
) {
    ffi::dgeequ_(
        &m,
        &n,
        a.as_ptr(),
        &lda,
        r.as_mut_ptr(),
        c.as_mut_ptr(),
        rowcnd,
        colcnd,
        amax,
        info,
    )
}

#[inline]
pub unsafe fn sgeequ(
    m: i32,
    n: i32,
    a: &[f32],
    lda: i32,
    r: &mut [f32],
    c: &mut [f32],
    rowcnd: &mut f32,
    colcnd: &mut f32,
    amax: &mut f32,
    info: &mut i32,
) {
    ffi::sgeequ_(
        &m,
        &n,
        a.as_ptr(),
        &lda,
        r.as_mut_ptr(),
        c.as_mut_ptr(),
        rowcnd,
        colcnd,
        amax,
        info,
    )
}

#[inline]
pub unsafe fn zgeequ(
    m: i32,
    n: i32,
    a: &[c64],
    lda: i32,
    r: &mut [f64],
    c: &mut [f64],
    rowcnd: &mut f64,
    colcnd: &mut f64,
    amax: &mut f64,
    info: &mut i32,
) {
    ffi::zgeequ_(
        &m,
        &n,
        a.as_ptr() as *const _,
        &lda,
        r.as_mut_ptr(),
        c.as_mut_ptr(),
        rowcnd,
        colcnd,
        amax,
        info,
    )
}

#[inline]
pub unsafe fn cgeequb(
    m: i32,
    n: i32,
    a: &[c32],
    lda: i32,
    r: &mut [f32],
    c: &mut [f32],
    rowcnd: &mut f32,
    colcnd: &mut f32,
    amax: &mut f32,
    info: &mut i32,
) {
    ffi::cgeequb_(
        &m,
        &n,
        a.as_ptr() as *const _,
        &lda,
        r.as_mut_ptr(),
        c.as_mut_ptr(),
        rowcnd,
        colcnd,
        amax,
        info,
    )
}

#[inline]
pub unsafe fn dgeequb(
    m: i32,
    n: i32,
    a: &[f64],
    lda: i32,
    r: &mut [f64],
    c: &mut [f64],
    rowcnd: &mut f64,
    colcnd: &mut f64,
    amax: &mut f64,
    info: &mut i32,
) {
    ffi::dgeequb_(
        &m,
        &n,
        a.as_ptr(),
        &lda,
        r.as_mut_ptr(),
        c.as_mut_ptr(),
        rowcnd,
        colcnd,
        amax,
        info,
    )
}

#[inline]
pub unsafe fn sgeequb(
    m: i32,
    n: i32,
    a: &[f32],
    lda: i32,
    r: &mut [f32],
    c: &mut [f32],
    rowcnd: &mut f32,
    colcnd: &mut f32,
    amax: &mut f32,
    info: &mut i32,
) {
    ffi::sgeequb_(
        &m,
        &n,
        a.as_ptr(),
        &lda,
        r.as_mut_ptr(),
        c.as_mut_ptr(),
        rowcnd,
        colcnd,
        amax,
        info,
    )
}

#[inline]
pub unsafe fn zgeequb(
    m: i32,
    n: i32,
    a: &[c64],
    lda: i32,
    r: &mut [f64],
    c: &mut [f64],
    rowcnd: &mut f64,
    colcnd: &mut f64,
    amax: &mut f64,
    info: &mut i32,
) {
    ffi::zgeequb_(
        &m,
        &n,
        a.as_ptr() as *const _,
        &lda,
        r.as_mut_ptr(),
        c.as_mut_ptr(),
        rowcnd,
        colcnd,
        amax,
        info,
    )
}

#[inline]
pub unsafe fn cgees(
    jobvs: u8,
    sort: u8,
    select: Select1C32,
    n: i32,
    a: &mut [c32],
    lda: i32,
    sdim: &mut i32,
    w: &mut [c32],
    vs: &mut [c32],
    ldvs: i32,
    work: &mut [c32],
    lwork: i32,
    rwork: &mut [f32],
    bwork: &mut [i32],
    info: &mut i32,
) {
    ffi::cgees_(
        &(jobvs as c_char),
        &(sort as c_char),
        transmute(select),
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        sdim,
        w.as_mut_ptr() as *mut _,
        vs.as_mut_ptr() as *mut _,
        &ldvs,
        work.as_mut_ptr() as *mut _,
        &lwork,
        rwork.as_mut_ptr(),
        bwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn dgees(
    jobvs: u8,
    sort: u8,
    select: Select2F64,
    n: i32,
    a: &mut [f64],
    lda: i32,
    sdim: &mut i32,
    wr: &mut [f64],
    wi: &mut [f64],
    vs: &mut [f64],
    ldvs: i32,
    work: &mut [f64],
    lwork: i32,
    bwork: &mut [i32],
    info: &mut i32,
) {
    ffi::dgees_(
        &(jobvs as c_char),
        &(sort as c_char),
        transmute(select),
        &n,
        a.as_mut_ptr(),
        &lda,
        sdim,
        wr.as_mut_ptr(),
        wi.as_mut_ptr(),
        vs.as_mut_ptr(),
        &ldvs,
        work.as_mut_ptr(),
        &lwork,
        bwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn sgees(
    jobvs: u8,
    sort: u8,
    select: Select2F32,
    n: i32,
    a: &mut [f32],
    lda: i32,
    sdim: &mut i32,
    wr: &mut [f32],
    wi: &mut [f32],
    vs: &mut [f32],
    ldvs: i32,
    work: &mut [f32],
    lwork: i32,
    bwork: &mut [i32],
    info: &mut i32,
) {
    ffi::sgees_(
        &(jobvs as c_char),
        &(sort as c_char),
        transmute(select),
        &n,
        a.as_mut_ptr(),
        &lda,
        sdim,
        wr.as_mut_ptr(),
        wi.as_mut_ptr(),
        vs.as_mut_ptr(),
        &ldvs,
        work.as_mut_ptr(),
        &lwork,
        bwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn zgees(
    jobvs: u8,
    sort: u8,
    select: Select1C64,
    n: i32,
    a: &mut [c64],
    lda: i32,
    sdim: &mut i32,
    w: &mut [c64],
    vs: &mut [c64],
    ldvs: i32,
    work: &mut [c64],
    lwork: i32,
    rwork: &mut [f64],
    bwork: &mut [i32],
    info: &mut i32,
) {
    ffi::zgees_(
        &(jobvs as c_char),
        &(sort as c_char),
        transmute(select),
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        sdim,
        w.as_mut_ptr() as *mut _,
        vs.as_mut_ptr() as *mut _,
        &ldvs,
        work.as_mut_ptr() as *mut _,
        &lwork,
        rwork.as_mut_ptr(),
        bwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn cgeesx(
    jobvs: u8,
    sort: u8,
    select: Select1C32,
    sense: u8,
    n: i32,
    a: &mut [c32],
    lda: i32,
    sdim: &mut i32,
    w: &mut [c32],
    vs: &mut [c32],
    ldvs: i32,
    rconde: &mut [f32],
    rcondv: &mut [f32],
    work: &mut [c32],
    lwork: i32,
    rwork: &mut [f32],
    bwork: &mut [i32],
    info: &mut i32,
) {
    ffi::cgeesx_(
        &(jobvs as c_char),
        &(sort as c_char),
        transmute(select),
        &(sense as c_char),
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        sdim,
        w.as_mut_ptr() as *mut _,
        vs.as_mut_ptr() as *mut _,
        &ldvs,
        rconde.as_mut_ptr(),
        rcondv.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        &lwork,
        rwork.as_mut_ptr(),
        bwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn dgeesx(
    jobvs: u8,
    sort: u8,
    select: Select2F64,
    sense: u8,
    n: i32,
    a: &mut [f64],
    lda: i32,
    sdim: &mut i32,
    wr: &mut [f64],
    wi: &mut [f64],
    vs: &mut [f64],
    ldvs: i32,
    rconde: &mut [f64],
    rcondv: &mut [f64],
    work: &mut [f64],
    lwork: i32,
    iwork: &mut [i32],
    liwork: i32,
    bwork: &mut [i32],
    info: &mut i32,
) {
    ffi::dgeesx_(
        &(jobvs as c_char),
        &(sort as c_char),
        transmute(select),
        &(sense as c_char),
        &n,
        a.as_mut_ptr(),
        &lda,
        sdim,
        wr.as_mut_ptr(),
        wi.as_mut_ptr(),
        vs.as_mut_ptr(),
        &ldvs,
        rconde.as_mut_ptr(),
        rcondv.as_mut_ptr(),
        work.as_mut_ptr(),
        &lwork,
        iwork.as_mut_ptr(),
        &liwork,
        bwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn sgeesx(
    jobvs: u8,
    sort: u8,
    select: Select2F32,
    sense: u8,
    n: i32,
    a: &mut [f32],
    lda: i32,
    sdim: &mut i32,
    wr: &mut [f32],
    wi: &mut [f32],
    vs: &mut [f32],
    ldvs: i32,
    rconde: &mut [f32],
    rcondv: &mut [f32],
    work: &mut [f32],
    lwork: i32,
    iwork: &mut [i32],
    liwork: i32,
    bwork: &mut [i32],
    info: &mut i32,
) {
    ffi::sgeesx_(
        &(jobvs as c_char),
        &(sort as c_char),
        transmute(select),
        &(sense as c_char),
        &n,
        a.as_mut_ptr(),
        &lda,
        sdim,
        wr.as_mut_ptr(),
        wi.as_mut_ptr(),
        vs.as_mut_ptr(),
        &ldvs,
        rconde.as_mut_ptr(),
        rcondv.as_mut_ptr(),
        work.as_mut_ptr(),
        &lwork,
        iwork.as_mut_ptr(),
        &liwork,
        bwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn zgeesx(
    jobvs: u8,
    sort: u8,
    select: Select1C64,
    sense: u8,
    n: i32,
    a: &mut [c64],
    lda: i32,
    sdim: &mut i32,
    w: &mut [c64],
    vs: &mut [c64],
    ldvs: i32,
    rconde: &mut [f64],
    rcondv: &mut [f64],
    work: &mut [c64],
    lwork: i32,
    rwork: &mut [f64],
    bwork: &mut [i32],
    info: &mut i32,
) {
    ffi::zgeesx_(
        &(jobvs as c_char),
        &(sort as c_char),
        transmute(select),
        &(sense as c_char),
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        sdim,
        w.as_mut_ptr() as *mut _,
        vs.as_mut_ptr() as *mut _,
        &ldvs,
        rconde.as_mut_ptr(),
        rcondv.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        &lwork,
        rwork.as_mut_ptr(),
        bwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn cgeev(
    jobvl: u8,
    jobvr: u8,
    n: i32,
    a: &mut [c32],
    lda: i32,
    w: &mut [c32],
    vl: &mut [c32],
    ldvl: i32,
    vr: &mut [c32],
    ldvr: i32,
    work: &mut [c32],
    lwork: i32,
    rwork: &mut [f32],
    info: &mut i32,
) {
    ffi::cgeev_(
        &(jobvl as c_char),
        &(jobvr as c_char),
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        w.as_mut_ptr() as *mut _,
        vl.as_mut_ptr() as *mut _,
        &ldvl,
        vr.as_mut_ptr() as *mut _,
        &ldvr,
        work.as_mut_ptr() as *mut _,
        &lwork,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn dgeev(
    jobvl: u8,
    jobvr: u8,
    n: i32,
    a: &mut [f64],
    lda: i32,
    wr: &mut [f64],
    wi: &mut [f64],
    vl: &mut [f64],
    ldvl: i32,
    vr: &mut [f64],
    ldvr: i32,
    work: &mut [f64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::dgeev_(
        &(jobvl as c_char),
        &(jobvr as c_char),
        &n,
        a.as_mut_ptr(),
        &lda,
        wr.as_mut_ptr(),
        wi.as_mut_ptr(),
        vl.as_mut_ptr(),
        &ldvl,
        vr.as_mut_ptr(),
        &ldvr,
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn sgeev(
    jobvl: u8,
    jobvr: u8,
    n: i32,
    a: &mut [f32],
    lda: i32,
    wr: &mut [f32],
    wi: &mut [f32],
    vl: &mut [f32],
    ldvl: i32,
    vr: &mut [f32],
    ldvr: i32,
    work: &mut [f32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::sgeev_(
        &(jobvl as c_char),
        &(jobvr as c_char),
        &n,
        a.as_mut_ptr(),
        &lda,
        wr.as_mut_ptr(),
        wi.as_mut_ptr(),
        vl.as_mut_ptr(),
        &ldvl,
        vr.as_mut_ptr(),
        &ldvr,
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn zgeev(
    jobvl: u8,
    jobvr: u8,
    n: i32,
    a: &mut [c64],
    lda: i32,
    w: &mut [c64],
    vl: &mut [c64],
    ldvl: i32,
    vr: &mut [c64],
    ldvr: i32,
    work: &mut [c64],
    lwork: i32,
    rwork: &mut [f64],
    info: &mut i32,
) {
    ffi::zgeev_(
        &(jobvl as c_char),
        &(jobvr as c_char),
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        w.as_mut_ptr() as *mut _,
        vl.as_mut_ptr() as *mut _,
        &ldvl,
        vr.as_mut_ptr() as *mut _,
        &ldvr,
        work.as_mut_ptr() as *mut _,
        &lwork,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn cgeevx(
    balanc: u8,
    jobvl: u8,
    jobvr: u8,
    sense: u8,
    n: i32,
    a: &mut [c32],
    lda: i32,
    w: &mut [c32],
    vl: &mut [c32],
    ldvl: i32,
    vr: &mut [c32],
    ldvr: i32,
    ilo: &mut i32,
    ihi: &mut i32,
    scale: &mut [f32],
    abnrm: &mut f32,
    rconde: &mut [f32],
    rcondv: &mut [f32],
    work: &mut [c32],
    lwork: i32,
    rwork: &mut [f32],
    info: &mut i32,
) {
    ffi::cgeevx_(
        &(balanc as c_char),
        &(jobvl as c_char),
        &(jobvr as c_char),
        &(sense as c_char),
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        w.as_mut_ptr() as *mut _,
        vl.as_mut_ptr() as *mut _,
        &ldvl,
        vr.as_mut_ptr() as *mut _,
        &ldvr,
        ilo,
        ihi,
        scale.as_mut_ptr(),
        abnrm,
        rconde.as_mut_ptr(),
        rcondv.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        &lwork,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn dgeevx(
    balanc: u8,
    jobvl: u8,
    jobvr: u8,
    sense: u8,
    n: i32,
    a: &mut [f64],
    lda: i32,
    wr: &mut [f64],
    wi: &mut [f64],
    vl: &mut [f64],
    ldvl: i32,
    vr: &mut [f64],
    ldvr: i32,
    ilo: &mut i32,
    ihi: &mut i32,
    scale: &mut [f64],
    abnrm: &mut f64,
    rconde: &mut [f64],
    rcondv: &mut [f64],
    work: &mut [f64],
    lwork: i32,
    iwork: &mut [i32],
    info: &mut i32,
) {
    ffi::dgeevx_(
        &(balanc as c_char),
        &(jobvl as c_char),
        &(jobvr as c_char),
        &(sense as c_char),
        &n,
        a.as_mut_ptr(),
        &lda,
        wr.as_mut_ptr(),
        wi.as_mut_ptr(),
        vl.as_mut_ptr(),
        &ldvl,
        vr.as_mut_ptr(),
        &ldvr,
        ilo,
        ihi,
        scale.as_mut_ptr(),
        abnrm,
        rconde.as_mut_ptr(),
        rcondv.as_mut_ptr(),
        work.as_mut_ptr(),
        &lwork,
        iwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn sgeevx(
    balanc: u8,
    jobvl: u8,
    jobvr: u8,
    sense: u8,
    n: i32,
    a: &mut [f32],
    lda: i32,
    wr: &mut [f32],
    wi: &mut [f32],
    vl: &mut [f32],
    ldvl: i32,
    vr: &mut [f32],
    ldvr: i32,
    ilo: &mut i32,
    ihi: &mut i32,
    scale: &mut [f32],
    abnrm: &mut f32,
    rconde: &mut [f32],
    rcondv: &mut [f32],
    work: &mut [f32],
    lwork: i32,
    iwork: &mut [i32],
    info: &mut i32,
) {
    ffi::sgeevx_(
        &(balanc as c_char),
        &(jobvl as c_char),
        &(jobvr as c_char),
        &(sense as c_char),
        &n,
        a.as_mut_ptr(),
        &lda,
        wr.as_mut_ptr(),
        wi.as_mut_ptr(),
        vl.as_mut_ptr(),
        &ldvl,
        vr.as_mut_ptr(),
        &ldvr,
        ilo,
        ihi,
        scale.as_mut_ptr(),
        abnrm,
        rconde.as_mut_ptr(),
        rcondv.as_mut_ptr(),
        work.as_mut_ptr(),
        &lwork,
        iwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn zgeevx(
    balanc: u8,
    jobvl: u8,
    jobvr: u8,
    sense: u8,
    n: i32,
    a: &mut [c64],
    lda: i32,
    w: &mut [c64],
    vl: &mut [c64],
    ldvl: i32,
    vr: &mut [c64],
    ldvr: i32,
    ilo: &mut i32,
    ihi: &mut i32,
    scale: &mut [f64],
    abnrm: &mut f64,
    rconde: &mut [f64],
    rcondv: &mut [f64],
    work: &mut [c64],
    lwork: i32,
    rwork: &mut [f64],
    info: &mut i32,
) {
    ffi::zgeevx_(
        &(balanc as c_char),
        &(jobvl as c_char),
        &(jobvr as c_char),
        &(sense as c_char),
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        w.as_mut_ptr() as *mut _,
        vl.as_mut_ptr() as *mut _,
        &ldvl,
        vr.as_mut_ptr() as *mut _,
        &ldvr,
        ilo,
        ihi,
        scale.as_mut_ptr(),
        abnrm,
        rconde.as_mut_ptr(),
        rcondv.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        &lwork,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn cgehrd(
    n: i32,
    ilo: i32,
    ihi: i32,
    a: &mut [c32],
    lda: i32,
    tau: &mut [c32],
    work: &mut [c32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::cgehrd_(
        &n,
        &ilo,
        &ihi,
        a.as_mut_ptr() as *mut _,
        &lda,
        tau.as_mut_ptr() as *mut _,
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn dgehrd(
    n: i32,
    ilo: i32,
    ihi: i32,
    a: &mut [f64],
    lda: i32,
    tau: &mut [f64],
    work: &mut [f64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::dgehrd_(
        &n,
        &ilo,
        &ihi,
        a.as_mut_ptr(),
        &lda,
        tau.as_mut_ptr(),
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn sgehrd(
    n: i32,
    ilo: i32,
    ihi: i32,
    a: &mut [f32],
    lda: i32,
    tau: &mut [f32],
    work: &mut [f32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::sgehrd_(
        &n,
        &ilo,
        &ihi,
        a.as_mut_ptr(),
        &lda,
        tau.as_mut_ptr(),
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn zgehrd(
    n: i32,
    ilo: i32,
    ihi: i32,
    a: &mut [c64],
    lda: i32,
    tau: &mut [c64],
    work: &mut [c64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::zgehrd_(
        &n,
        &ilo,
        &ihi,
        a.as_mut_ptr() as *mut _,
        &lda,
        tau.as_mut_ptr() as *mut _,
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn cgejsv(
    joba: u8,
    jobu: u8,
    jobv: u8,
    jobr: u8,
    jobt: u8,
    jobp: u8,
    m: i32,
    n: i32,
    a: &mut [c32],
    lda: i32,
    sva: &mut [f32],
    u: &mut [c32],
    ldu: i32,
    v: &mut [c32],
    ldv: i32,
    cwork: &mut [c32],
    lwork: i32,
    rwork: &mut [f32],
    lrwork: i32,
    iwork: &mut [i32],
    info: &mut i32,
) {
    ffi::cgejsv_(
        &(joba as c_char),
        &(jobu as c_char),
        &(jobv as c_char),
        &(jobr as c_char),
        &(jobt as c_char),
        &(jobp as c_char),
        &m,
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        sva.as_mut_ptr(),
        u.as_mut_ptr() as *mut _,
        &ldu,
        v.as_mut_ptr() as *mut _,
        &ldv,
        cwork.as_mut_ptr() as *mut _,
        &lwork,
        rwork.as_mut_ptr(),
        &lrwork,
        iwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn dgejsv(
    joba: u8,
    jobu: u8,
    jobv: u8,
    jobr: u8,
    jobt: u8,
    jobp: u8,
    m: i32,
    n: i32,
    a: &mut [f64],
    lda: i32,
    sva: &mut [f64],
    u: &mut [f64],
    ldu: i32,
    v: &mut [f64],
    ldv: i32,
    work: &mut [f64],
    lwork: i32,
    iwork: &mut [i32],
    info: &mut i32,
) {
    ffi::dgejsv_(
        &(joba as c_char),
        &(jobu as c_char),
        &(jobv as c_char),
        &(jobr as c_char),
        &(jobt as c_char),
        &(jobp as c_char),
        &m,
        &n,
        a.as_mut_ptr(),
        &lda,
        sva.as_mut_ptr(),
        u.as_mut_ptr(),
        &ldu,
        v.as_mut_ptr(),
        &ldv,
        work.as_mut_ptr(),
        &lwork,
        iwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn sgejsv(
    joba: u8,
    jobu: u8,
    jobv: u8,
    jobr: u8,
    jobt: u8,
    jobp: u8,
    m: i32,
    n: i32,
    a: &mut [f32],
    lda: i32,
    sva: &mut [f32],
    u: &mut [f32],
    ldu: i32,
    v: &mut [f32],
    ldv: i32,
    work: &mut [f32],
    lwork: i32,
    iwork: &mut [i32],
    info: &mut i32,
) {
    ffi::sgejsv_(
        &(joba as c_char),
        &(jobu as c_char),
        &(jobv as c_char),
        &(jobr as c_char),
        &(jobt as c_char),
        &(jobp as c_char),
        &m,
        &n,
        a.as_mut_ptr(),
        &lda,
        sva.as_mut_ptr(),
        u.as_mut_ptr(),
        &ldu,
        v.as_mut_ptr(),
        &ldv,
        work.as_mut_ptr(),
        &lwork,
        iwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn zgejsv(
    joba: u8,
    jobu: u8,
    jobv: u8,
    jobr: u8,
    jobt: u8,
    jobp: u8,
    m: i32,
    n: i32,
    a: &mut [c64],
    lda: i32,
    sva: &mut [f64],
    u: &mut [c64],
    ldu: i32,
    v: &mut [c64],
    ldv: i32,
    cwork: &mut [c64],
    lwork: i32,
    rwork: &mut [f64],
    lrwork: i32,
    iwork: &mut [i32],
    info: &mut i32,
) {
    ffi::zgejsv_(
        &(joba as c_char),
        &(jobu as c_char),
        &(jobv as c_char),
        &(jobr as c_char),
        &(jobt as c_char),
        &(jobp as c_char),
        &m,
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        sva.as_mut_ptr(),
        u.as_mut_ptr() as *mut _,
        &ldu,
        v.as_mut_ptr() as *mut _,
        &ldv,
        cwork.as_mut_ptr() as *mut _,
        &lwork,
        rwork.as_mut_ptr(),
        &lrwork,
        iwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn cgelq(
    m: i32,
    n: i32,
    a: &mut [c32],
    lda: i32,
    t: &mut [c32],
    tsize: &[i32],
    work: &mut [c32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::cgelq_(
        &m,
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        t.as_mut_ptr() as *mut _,
        tsize.as_ptr(),
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn dgelq(
    m: i32,
    n: i32,
    a: &mut [f64],
    lda: i32,
    t: &mut [f64],
    tsize: &[i32],
    work: &mut [f64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::dgelq_(
        &m,
        &n,
        a.as_mut_ptr(),
        &lda,
        t.as_mut_ptr(),
        tsize.as_ptr(),
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn sgelq(
    m: i32,
    n: i32,
    a: &mut [f32],
    lda: i32,
    t: &mut [f32],
    tsize: &[i32],
    work: &mut [f32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::sgelq_(
        &m,
        &n,
        a.as_mut_ptr(),
        &lda,
        t.as_mut_ptr(),
        tsize.as_ptr(),
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn zgelq(
    m: i32,
    n: i32,
    a: &mut [c64],
    lda: i32,
    t: &mut [c64],
    tsize: &[i32],
    work: &mut [c64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::zgelq_(
        &m,
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        t.as_mut_ptr() as *mut _,
        tsize.as_ptr(),
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn cgelq2(
    m: i32,
    n: i32,
    a: &mut [c32],
    lda: i32,
    tau: &mut [c32],
    work: &mut [c32],
    info: &mut i32,
) {
    ffi::cgelq2_(
        &m,
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        tau.as_mut_ptr() as *mut _,
        work.as_mut_ptr() as *mut _,
        info,
    )
}

#[inline]
pub unsafe fn dgelq2(
    m: i32,
    n: i32,
    a: &mut [f64],
    lda: i32,
    tau: &mut [f64],
    work: &mut [f64],
    info: &mut i32,
) {
    ffi::dgelq2_(
        &m,
        &n,
        a.as_mut_ptr(),
        &lda,
        tau.as_mut_ptr(),
        work.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn sgelq2(
    m: i32,
    n: i32,
    a: &mut [f32],
    lda: i32,
    tau: &mut [f32],
    work: &mut [f32],
    info: &mut i32,
) {
    ffi::sgelq2_(
        &m,
        &n,
        a.as_mut_ptr(),
        &lda,
        tau.as_mut_ptr(),
        work.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn zgelq2(
    m: i32,
    n: i32,
    a: &mut [c64],
    lda: i32,
    tau: &mut [c64],
    work: &mut [c64],
    info: &mut i32,
) {
    ffi::zgelq2_(
        &m,
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        tau.as_mut_ptr() as *mut _,
        work.as_mut_ptr() as *mut _,
        info,
    )
}

#[inline]
pub unsafe fn cgelqf(
    m: i32,
    n: i32,
    a: &mut [c32],
    lda: i32,
    tau: &mut [c32],
    work: &mut [c32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::cgelqf_(
        &m,
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        tau.as_mut_ptr() as *mut _,
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn dgelqf(
    m: i32,
    n: i32,
    a: &mut [f64],
    lda: i32,
    tau: &mut [f64],
    work: &mut [f64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::dgelqf_(
        &m,
        &n,
        a.as_mut_ptr(),
        &lda,
        tau.as_mut_ptr(),
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn sgelqf(
    m: i32,
    n: i32,
    a: &mut [f32],
    lda: i32,
    tau: &mut [f32],
    work: &mut [f32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::sgelqf_(
        &m,
        &n,
        a.as_mut_ptr(),
        &lda,
        tau.as_mut_ptr(),
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn zgelqf(
    m: i32,
    n: i32,
    a: &mut [c64],
    lda: i32,
    tau: &mut [c64],
    work: &mut [c64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::zgelqf_(
        &m,
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        tau.as_mut_ptr() as *mut _,
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn cgels(
    trans: u8,
    m: i32,
    n: i32,
    nrhs: i32,
    a: &mut [c32],
    lda: i32,
    b: &mut [c32],
    ldb: i32,
    work: &mut [c32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::cgels_(
        &(trans as c_char),
        &m,
        &n,
        &nrhs,
        a.as_mut_ptr() as *mut _,
        &lda,
        b.as_mut_ptr() as *mut _,
        &ldb,
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn dgels(
    trans: u8,
    m: i32,
    n: i32,
    nrhs: i32,
    a: &mut [f64],
    lda: i32,
    b: &mut [f64],
    ldb: i32,
    work: &mut [f64],
    lwork: i32,
    info: &mut i32,
    arg1: size_t,
) {
    ffi::dgels_(
        &(trans as c_char),
        &m,
        &n,
        &nrhs,
        a.as_mut_ptr(),
        &lda,
        b.as_mut_ptr(),
        &ldb,
        work.as_mut_ptr(),
        &lwork,
        info,
        arg1,
    )
}

#[inline]
pub unsafe fn sgels(
    trans: u8,
    m: i32,
    n: i32,
    nrhs: i32,
    a: &mut [f32],
    lda: i32,
    b: &mut [f32],
    ldb: i32,
    work: &mut [f32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::sgels_(
        &(trans as c_char),
        &m,
        &n,
        &nrhs,
        a.as_mut_ptr(),
        &lda,
        b.as_mut_ptr(),
        &ldb,
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn zgels(
    trans: u8,
    m: i32,
    n: i32,
    nrhs: i32,
    a: &mut [c64],
    lda: i32,
    b: &mut [c64],
    ldb: i32,
    work: &mut [c64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::zgels_(
        &(trans as c_char),
        &m,
        &n,
        &nrhs,
        a.as_mut_ptr() as *mut _,
        &lda,
        b.as_mut_ptr() as *mut _,
        &ldb,
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn cgelsd(
    m: i32,
    n: i32,
    nrhs: i32,
    a: &mut [c32],
    lda: i32,
    b: &mut [c32],
    ldb: i32,
    s: &mut [f32],
    rcond: f32,
    rank: &mut i32,
    work: &mut [c32],
    lwork: i32,
    rwork: &mut [f32],
    iwork: &mut [i32],
    info: &mut i32,
) {
    ffi::cgelsd_(
        &m,
        &n,
        &nrhs,
        a.as_mut_ptr() as *mut _,
        &lda,
        b.as_mut_ptr() as *mut _,
        &ldb,
        s.as_mut_ptr(),
        &rcond,
        rank,
        work.as_mut_ptr() as *mut _,
        &lwork,
        rwork.as_mut_ptr(),
        iwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn dgelsd(
    m: i32,
    n: i32,
    nrhs: i32,
    a: &mut [f64],
    lda: i32,
    b: &mut [f64],
    ldb: i32,
    s: &mut [f64],
    rcond: f64,
    rank: &mut i32,
    work: &mut [f64],
    lwork: i32,
    iwork: &mut [i32],
    info: &mut i32,
) {
    ffi::dgelsd_(
        &m,
        &n,
        &nrhs,
        a.as_mut_ptr(),
        &lda,
        b.as_mut_ptr(),
        &ldb,
        s.as_mut_ptr(),
        &rcond,
        rank,
        work.as_mut_ptr(),
        &lwork,
        iwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn sgelsd(
    m: i32,
    n: i32,
    nrhs: i32,
    a: &mut [f32],
    lda: i32,
    b: &mut [f32],
    ldb: i32,
    s: &mut [f32],
    rcond: f32,
    rank: &mut i32,
    work: &mut [f32],
    lwork: i32,
    iwork: &mut [i32],
    info: &mut i32,
) {
    ffi::sgelsd_(
        &m,
        &n,
        &nrhs,
        a.as_mut_ptr(),
        &lda,
        b.as_mut_ptr(),
        &ldb,
        s.as_mut_ptr(),
        &rcond,
        rank,
        work.as_mut_ptr(),
        &lwork,
        iwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn zgelsd(
    m: i32,
    n: i32,
    nrhs: i32,
    a: &mut [c64],
    lda: i32,
    b: &mut [c64],
    ldb: i32,
    s: &mut [f64],
    rcond: f64,
    rank: &mut i32,
    work: &mut [c64],
    lwork: i32,
    rwork: &mut [f64],
    iwork: &mut [i32],
    info: &mut i32,
) {
    ffi::zgelsd_(
        &m,
        &n,
        &nrhs,
        a.as_mut_ptr() as *mut _,
        &lda,
        b.as_mut_ptr() as *mut _,
        &ldb,
        s.as_mut_ptr(),
        &rcond,
        rank,
        work.as_mut_ptr() as *mut _,
        &lwork,
        rwork.as_mut_ptr(),
        iwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn cgelss(
    m: i32,
    n: i32,
    nrhs: i32,
    a: &mut [c32],
    lda: i32,
    b: &mut [c32],
    ldb: i32,
    s: &mut [f32],
    rcond: f32,
    rank: &mut i32,
    work: &mut [c32],
    lwork: i32,
    rwork: &mut [f32],
    info: &mut i32,
) {
    ffi::cgelss_(
        &m,
        &n,
        &nrhs,
        a.as_mut_ptr() as *mut _,
        &lda,
        b.as_mut_ptr() as *mut _,
        &ldb,
        s.as_mut_ptr(),
        &rcond,
        rank,
        work.as_mut_ptr() as *mut _,
        &lwork,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn dgelss(
    m: i32,
    n: i32,
    nrhs: i32,
    a: &mut [f64],
    lda: i32,
    b: &mut [f64],
    ldb: i32,
    s: &mut [f64],
    rcond: f64,
    rank: &mut i32,
    work: &mut [f64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::dgelss_(
        &m,
        &n,
        &nrhs,
        a.as_mut_ptr(),
        &lda,
        b.as_mut_ptr(),
        &ldb,
        s.as_mut_ptr(),
        &rcond,
        rank,
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn sgelss(
    m: i32,
    n: i32,
    nrhs: i32,
    a: &mut [f32],
    lda: i32,
    b: &mut [f32],
    ldb: i32,
    s: &mut [f32],
    rcond: f32,
    rank: &mut i32,
    work: &mut [f32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::sgelss_(
        &m,
        &n,
        &nrhs,
        a.as_mut_ptr(),
        &lda,
        b.as_mut_ptr(),
        &ldb,
        s.as_mut_ptr(),
        &rcond,
        rank,
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn zgelss(
    m: i32,
    n: i32,
    nrhs: i32,
    a: &mut [c64],
    lda: i32,
    b: &mut [c64],
    ldb: i32,
    s: &mut [f64],
    rcond: f64,
    rank: &mut i32,
    work: &mut [c64],
    lwork: i32,
    rwork: &mut [f64],
    info: &mut i32,
) {
    ffi::zgelss_(
        &m,
        &n,
        &nrhs,
        a.as_mut_ptr() as *mut _,
        &lda,
        b.as_mut_ptr() as *mut _,
        &ldb,
        s.as_mut_ptr(),
        &rcond,
        rank,
        work.as_mut_ptr() as *mut _,
        &lwork,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn cgelsy(
    m: i32,
    n: i32,
    nrhs: i32,
    a: &mut [c32],
    lda: i32,
    b: &mut [c32],
    ldb: i32,
    jpvt: &mut [i32],
    rcond: f32,
    rank: &mut i32,
    work: &mut [c32],
    lwork: i32,
    rwork: &mut [f32],
    info: &mut i32,
) {
    ffi::cgelsy_(
        &m,
        &n,
        &nrhs,
        a.as_mut_ptr() as *mut _,
        &lda,
        b.as_mut_ptr() as *mut _,
        &ldb,
        jpvt.as_mut_ptr(),
        &rcond,
        rank,
        work.as_mut_ptr() as *mut _,
        &lwork,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn dgelsy(
    m: i32,
    n: i32,
    nrhs: i32,
    a: &mut [f64],
    lda: i32,
    b: &mut [f64],
    ldb: i32,
    jpvt: &mut [i32],
    rcond: f64,
    rank: &mut i32,
    work: &mut [f64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::dgelsy_(
        &m,
        &n,
        &nrhs,
        a.as_mut_ptr(),
        &lda,
        b.as_mut_ptr(),
        &ldb,
        jpvt.as_mut_ptr(),
        &rcond,
        rank,
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn sgelsy(
    m: i32,
    n: i32,
    nrhs: i32,
    a: &mut [f32],
    lda: i32,
    b: &mut [f32],
    ldb: i32,
    jpvt: &mut [i32],
    rcond: f32,
    rank: &mut i32,
    work: &mut [f32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::sgelsy_(
        &m,
        &n,
        &nrhs,
        a.as_mut_ptr(),
        &lda,
        b.as_mut_ptr(),
        &ldb,
        jpvt.as_mut_ptr(),
        &rcond,
        rank,
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn zgelsy(
    m: i32,
    n: i32,
    nrhs: i32,
    a: &mut [c64],
    lda: i32,
    b: &mut [c64],
    ldb: i32,
    jpvt: &mut [i32],
    rcond: f64,
    rank: &mut i32,
    work: &mut [c64],
    lwork: i32,
    rwork: &mut [f64],
    info: &mut i32,
) {
    ffi::zgelsy_(
        &m,
        &n,
        &nrhs,
        a.as_mut_ptr() as *mut _,
        &lda,
        b.as_mut_ptr() as *mut _,
        &ldb,
        jpvt.as_mut_ptr(),
        &rcond,
        rank,
        work.as_mut_ptr() as *mut _,
        &lwork,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn cgemlq(
    side: u8,
    trans: u8,
    m: i32,
    n: i32,
    k: i32,
    a: &[c32],
    lda: i32,
    t: &[c32],
    tsize: &[i32],
    c: &mut [c32],
    ldc: i32,
    work: &mut [c32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::cgemlq_(
        &(side as c_char),
        &(trans as c_char),
        &m,
        &n,
        &k,
        a.as_ptr() as *const _,
        &lda,
        t.as_ptr() as *const _,
        tsize.as_ptr(),
        c.as_mut_ptr() as *mut _,
        &ldc,
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn dgemlq(
    side: u8,
    trans: u8,
    m: i32,
    n: i32,
    k: i32,
    a: &[f64],
    lda: i32,
    t: &[f64],
    tsize: &[i32],
    c: &mut [f64],
    ldc: i32,
    work: &mut [f64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::dgemlq_(
        &(side as c_char),
        &(trans as c_char),
        &m,
        &n,
        &k,
        a.as_ptr(),
        &lda,
        t.as_ptr(),
        tsize.as_ptr(),
        c.as_mut_ptr(),
        &ldc,
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn sgemlq(
    side: u8,
    trans: u8,
    m: i32,
    n: i32,
    k: i32,
    a: &[f32],
    lda: i32,
    t: &[f32],
    tsize: &[i32],
    c: &mut [f32],
    ldc: i32,
    work: &mut [f32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::sgemlq_(
        &(side as c_char),
        &(trans as c_char),
        &m,
        &n,
        &k,
        a.as_ptr(),
        &lda,
        t.as_ptr(),
        tsize.as_ptr(),
        c.as_mut_ptr(),
        &ldc,
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn zgemlq(
    side: u8,
    trans: u8,
    m: i32,
    n: i32,
    k: i32,
    a: &[c64],
    lda: i32,
    t: &[c64],
    tsize: &[i32],
    c: &mut [c64],
    ldc: i32,
    work: &mut [c64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::zgemlq_(
        &(side as c_char),
        &(trans as c_char),
        &m,
        &n,
        &k,
        a.as_ptr() as *const _,
        &lda,
        t.as_ptr() as *const _,
        tsize.as_ptr(),
        c.as_mut_ptr() as *mut _,
        &ldc,
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn cgemqr(
    side: u8,
    trans: u8,
    m: i32,
    n: i32,
    k: i32,
    a: &[c32],
    lda: i32,
    t: &[c32],
    tsize: &[i32],
    c: &mut [c32],
    ldc: i32,
    work: &mut [c32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::cgemqr_(
        &(side as c_char),
        &(trans as c_char),
        &m,
        &n,
        &k,
        a.as_ptr() as *const _,
        &lda,
        t.as_ptr() as *const _,
        tsize.as_ptr(),
        c.as_mut_ptr() as *mut _,
        &ldc,
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn dgemqr(
    side: u8,
    trans: u8,
    m: i32,
    n: i32,
    k: i32,
    a: &[f64],
    lda: i32,
    t: &[f64],
    tsize: &[i32],
    c: &mut [f64],
    ldc: i32,
    work: &mut [f64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::dgemqr_(
        &(side as c_char),
        &(trans as c_char),
        &m,
        &n,
        &k,
        a.as_ptr(),
        &lda,
        t.as_ptr(),
        tsize.as_ptr(),
        c.as_mut_ptr(),
        &ldc,
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn sgemqr(
    side: u8,
    trans: u8,
    m: i32,
    n: i32,
    k: i32,
    a: &[f32],
    lda: i32,
    t: &[f32],
    tsize: &[i32],
    c: &mut [f32],
    ldc: i32,
    work: &mut [f32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::sgemqr_(
        &(side as c_char),
        &(trans as c_char),
        &m,
        &n,
        &k,
        a.as_ptr(),
        &lda,
        t.as_ptr(),
        tsize.as_ptr(),
        c.as_mut_ptr(),
        &ldc,
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn zgemqr(
    side: u8,
    trans: u8,
    m: i32,
    n: i32,
    k: i32,
    a: &[c64],
    lda: i32,
    t: &[c64],
    tsize: &[i32],
    c: &mut [c64],
    ldc: i32,
    work: &mut [c64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::zgemqr_(
        &(side as c_char),
        &(trans as c_char),
        &m,
        &n,
        &k,
        a.as_ptr() as *const _,
        &lda,
        t.as_ptr() as *const _,
        tsize.as_ptr(),
        c.as_mut_ptr() as *mut _,
        &ldc,
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn cgemqrt(
    side: u8,
    trans: u8,
    m: i32,
    n: i32,
    k: i32,
    nb: i32,
    v: &[c32],
    ldv: i32,
    t: &[c32],
    ldt: i32,
    c: &mut [c32],
    ldc: i32,
    work: &mut [c32],
    info: &mut i32,
) {
    ffi::cgemqrt_(
        &(side as c_char),
        &(trans as c_char),
        &m,
        &n,
        &k,
        &nb,
        v.as_ptr() as *const _,
        &ldv,
        t.as_ptr() as *const _,
        &ldt,
        c.as_mut_ptr() as *mut _,
        &ldc,
        work.as_mut_ptr() as *mut _,
        info,
    )
}

#[inline]
pub unsafe fn dgemqrt(
    side: u8,
    trans: u8,
    m: i32,
    n: i32,
    k: i32,
    nb: i32,
    v: &[f64],
    ldv: i32,
    t: &[f64],
    ldt: i32,
    c: &mut [f64],
    ldc: i32,
    work: &mut [f64],
    info: &mut i32,
) {
    ffi::dgemqrt_(
        &(side as c_char),
        &(trans as c_char),
        &m,
        &n,
        &k,
        &nb,
        v.as_ptr(),
        &ldv,
        t.as_ptr(),
        &ldt,
        c.as_mut_ptr(),
        &ldc,
        work.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn sgemqrt(
    side: u8,
    trans: u8,
    m: i32,
    n: i32,
    k: i32,
    nb: i32,
    v: &[f32],
    ldv: i32,
    t: &[f32],
    ldt: i32,
    c: &mut [f32],
    ldc: i32,
    work: &mut [f32],
    info: &mut i32,
) {
    ffi::sgemqrt_(
        &(side as c_char),
        &(trans as c_char),
        &m,
        &n,
        &k,
        &nb,
        v.as_ptr(),
        &ldv,
        t.as_ptr(),
        &ldt,
        c.as_mut_ptr(),
        &ldc,
        work.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn zgemqrt(
    side: u8,
    trans: u8,
    m: i32,
    n: i32,
    k: i32,
    nb: i32,
    v: &[c64],
    ldv: i32,
    t: &[c64],
    ldt: i32,
    c: &mut [c64],
    ldc: i32,
    work: &mut [c64],
    info: &mut i32,
) {
    ffi::zgemqrt_(
        &(side as c_char),
        &(trans as c_char),
        &m,
        &n,
        &k,
        &nb,
        v.as_ptr() as *const _,
        &ldv,
        t.as_ptr() as *const _,
        &ldt,
        c.as_mut_ptr() as *mut _,
        &ldc,
        work.as_mut_ptr() as *mut _,
        info,
    )
}

#[inline]
pub unsafe fn cgeql2(
    m: i32,
    n: i32,
    a: &mut [c32],
    lda: i32,
    tau: &mut [c32],
    work: &mut [c32],
    info: &mut i32,
) {
    ffi::cgeql2_(
        &m,
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        tau.as_mut_ptr() as *mut _,
        work.as_mut_ptr() as *mut _,
        info,
    )
}

#[inline]
pub unsafe fn dgeql2(
    m: i32,
    n: i32,
    a: &mut [f64],
    lda: i32,
    tau: &mut [f64],
    work: &mut [f64],
    info: &mut i32,
) {
    ffi::dgeql2_(
        &m,
        &n,
        a.as_mut_ptr(),
        &lda,
        tau.as_mut_ptr(),
        work.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn sgeql2(
    m: i32,
    n: i32,
    a: &mut [f32],
    lda: i32,
    tau: &mut [f32],
    work: &mut [f32],
    info: &mut i32,
) {
    ffi::sgeql2_(
        &m,
        &n,
        a.as_mut_ptr(),
        &lda,
        tau.as_mut_ptr(),
        work.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn zgeql2(
    m: i32,
    n: i32,
    a: &mut [c64],
    lda: i32,
    tau: &mut [c64],
    work: &mut [c64],
    info: &mut i32,
) {
    ffi::zgeql2_(
        &m,
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        tau.as_mut_ptr() as *mut _,
        work.as_mut_ptr() as *mut _,
        info,
    )
}

#[inline]
pub unsafe fn cgeqlf(
    m: i32,
    n: i32,
    a: &mut [c32],
    lda: i32,
    tau: &mut [c32],
    work: &mut [c32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::cgeqlf_(
        &m,
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        tau.as_mut_ptr() as *mut _,
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn dgeqlf(
    m: i32,
    n: i32,
    a: &mut [f64],
    lda: i32,
    tau: &mut [f64],
    work: &mut [f64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::dgeqlf_(
        &m,
        &n,
        a.as_mut_ptr(),
        &lda,
        tau.as_mut_ptr(),
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn sgeqlf(
    m: i32,
    n: i32,
    a: &mut [f32],
    lda: i32,
    tau: &mut [f32],
    work: &mut [f32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::sgeqlf_(
        &m,
        &n,
        a.as_mut_ptr(),
        &lda,
        tau.as_mut_ptr(),
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn zgeqlf(
    m: i32,
    n: i32,
    a: &mut [c64],
    lda: i32,
    tau: &mut [c64],
    work: &mut [c64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::zgeqlf_(
        &m,
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        tau.as_mut_ptr() as *mut _,
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn sgeqpf(
    m: &mut i32,
    n: &mut i32,
    a: &mut [f32],
    lda: &mut i32,
    jpvt: &mut [i32],
    tau: &mut [f32],
    work: &mut [f32],
    info: &mut i32,
) {
    ffi::sgeqpf_(
        m,
        n,
        a.as_mut_ptr(),
        lda,
        jpvt.as_mut_ptr(),
        tau.as_mut_ptr(),
        work.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn dgeqpf(
    m: &mut i32,
    n: &mut i32,
    a: &mut [f64],
    lda: &mut i32,
    jpvt: &mut [i32],
    tau: &mut [f64],
    work: &mut [f64],
    info: &mut i32,
) {
    ffi::dgeqpf_(
        m,
        n,
        a.as_mut_ptr(),
        lda,
        jpvt.as_mut_ptr(),
        tau.as_mut_ptr(),
        work.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn cgeqpf(
    m: &mut i32,
    n: &mut i32,
    a: &mut [c32],
    lda: &mut i32,
    jpvt: &mut [i32],
    tau: &mut [c32],
    work: &mut [c32],
    rwork: &mut [f32],
    info: &mut i32,
) {
    ffi::cgeqpf_(
        m,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        jpvt.as_mut_ptr(),
        tau.as_mut_ptr() as *mut _,
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn zgeqpf(
    m: &mut i32,
    n: &mut i32,
    a: &mut [c64],
    lda: &mut i32,
    jpvt: &mut [i32],
    tau: &mut [c64],
    work: &mut [c64],
    rwork: &mut [f64],
    info: &mut i32,
) {
    ffi::zgeqpf_(
        m,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        jpvt.as_mut_ptr(),
        tau.as_mut_ptr() as *mut _,
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn cgeqp3(
    m: i32,
    n: i32,
    a: &mut [c32],
    lda: i32,
    jpvt: &mut [i32],
    tau: &mut [c32],
    work: &mut [c32],
    lwork: i32,
    rwork: &mut [f32],
    info: &mut i32,
) {
    ffi::cgeqp3_(
        &m,
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        jpvt.as_mut_ptr(),
        tau.as_mut_ptr() as *mut _,
        work.as_mut_ptr() as *mut _,
        &lwork,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn dgeqp3(
    m: i32,
    n: i32,
    a: &mut [f64],
    lda: i32,
    jpvt: &mut [i32],
    tau: &mut [f64],
    work: &mut [f64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::dgeqp3_(
        &m,
        &n,
        a.as_mut_ptr(),
        &lda,
        jpvt.as_mut_ptr(),
        tau.as_mut_ptr(),
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn sgeqp3(
    m: i32,
    n: i32,
    a: &mut [f32],
    lda: i32,
    jpvt: &mut [i32],
    tau: &mut [f32],
    work: &mut [f32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::sgeqp3_(
        &m,
        &n,
        a.as_mut_ptr(),
        &lda,
        jpvt.as_mut_ptr(),
        tau.as_mut_ptr(),
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn zgeqp3(
    m: i32,
    n: i32,
    a: &mut [c64],
    lda: i32,
    jpvt: &mut [i32],
    tau: &mut [c64],
    work: &mut [c64],
    lwork: i32,
    rwork: &mut [f64],
    info: &mut i32,
) {
    ffi::zgeqp3_(
        &m,
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        jpvt.as_mut_ptr(),
        tau.as_mut_ptr() as *mut _,
        work.as_mut_ptr() as *mut _,
        &lwork,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn cgeqr(
    m: i32,
    n: i32,
    a: &mut [c32],
    lda: i32,
    t: &mut [c32],
    tsize: &[i32],
    work: &mut [c32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::cgeqr_(
        &m,
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        t.as_mut_ptr() as *mut _,
        tsize.as_ptr(),
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn dgeqr(
    m: i32,
    n: i32,
    a: &mut [f64],
    lda: i32,
    t: &mut [f64],
    tsize: &[i32],
    work: &mut [f64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::dgeqr_(
        &m,
        &n,
        a.as_mut_ptr(),
        &lda,
        t.as_mut_ptr(),
        tsize.as_ptr(),
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn sgeqr(
    m: i32,
    n: i32,
    a: &mut [f32],
    lda: i32,
    t: &mut [f32],
    tsize: &[i32],
    work: &mut [f32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::sgeqr_(
        &m,
        &n,
        a.as_mut_ptr(),
        &lda,
        t.as_mut_ptr(),
        tsize.as_ptr(),
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn zgeqr(
    m: i32,
    n: i32,
    a: &mut [c64],
    lda: i32,
    t: &mut [c64],
    tsize: &[i32],
    work: &mut [c64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::zgeqr_(
        &m,
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        t.as_mut_ptr() as *mut _,
        tsize.as_ptr(),
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn cgeqr2(
    m: i32,
    n: i32,
    a: &mut [c32],
    lda: i32,
    tau: &mut [c32],
    work: &mut [c32],
    info: &mut i32,
) {
    ffi::cgeqr2_(
        &m,
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        tau.as_mut_ptr() as *mut _,
        work.as_mut_ptr() as *mut _,
        info,
    )
}

#[inline]
pub unsafe fn dgeqr2(
    m: i32,
    n: i32,
    a: &mut [f64],
    lda: i32,
    tau: &mut [f64],
    work: &mut [f64],
    info: &mut i32,
) {
    ffi::dgeqr2_(
        &m,
        &n,
        a.as_mut_ptr(),
        &lda,
        tau.as_mut_ptr(),
        work.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn sgeqr2(
    m: i32,
    n: i32,
    a: &mut [f32],
    lda: i32,
    tau: &mut [f32],
    work: &mut [f32],
    info: &mut i32,
) {
    ffi::sgeqr2_(
        &m,
        &n,
        a.as_mut_ptr(),
        &lda,
        tau.as_mut_ptr(),
        work.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn zgeqr2(
    m: i32,
    n: i32,
    a: &mut [c64],
    lda: i32,
    tau: &mut [c64],
    work: &mut [c64],
    info: &mut i32,
) {
    ffi::zgeqr2_(
        &m,
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        tau.as_mut_ptr() as *mut _,
        work.as_mut_ptr() as *mut _,
        info,
    )
}

#[inline]
pub unsafe fn cgeqrf(
    m: i32,
    n: i32,
    a: &mut [c32],
    lda: i32,
    tau: &mut [c32],
    work: &mut [c32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::cgeqrf_(
        &m,
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        tau.as_mut_ptr() as *mut _,
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn dgeqrf(
    m: i32,
    n: i32,
    a: &mut [f64],
    lda: i32,
    tau: &mut [f64],
    work: &mut [f64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::dgeqrf_(
        &m,
        &n,
        a.as_mut_ptr(),
        &lda,
        tau.as_mut_ptr(),
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn sgeqrf(
    m: i32,
    n: i32,
    a: &mut [f32],
    lda: i32,
    tau: &mut [f32],
    work: &mut [f32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::sgeqrf_(
        &m,
        &n,
        a.as_mut_ptr(),
        &lda,
        tau.as_mut_ptr(),
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn zgeqrf(
    m: i32,
    n: i32,
    a: &mut [c64],
    lda: i32,
    tau: &mut [c64],
    work: &mut [c64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::zgeqrf_(
        &m,
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        tau.as_mut_ptr() as *mut _,
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn cgeqrfp(
    m: i32,
    n: i32,
    a: &mut [c32],
    lda: i32,
    tau: &mut [c32],
    work: &mut [c32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::cgeqrfp_(
        &m,
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        tau.as_mut_ptr() as *mut _,
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn dgeqrfp(
    m: i32,
    n: i32,
    a: &mut [f64],
    lda: i32,
    tau: &mut [f64],
    work: &mut [f64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::dgeqrfp_(
        &m,
        &n,
        a.as_mut_ptr(),
        &lda,
        tau.as_mut_ptr(),
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn sgeqrfp(
    m: i32,
    n: i32,
    a: &mut [f32],
    lda: i32,
    tau: &mut [f32],
    work: &mut [f32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::sgeqrfp_(
        &m,
        &n,
        a.as_mut_ptr(),
        &lda,
        tau.as_mut_ptr(),
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn zgeqrfp(
    m: i32,
    n: i32,
    a: &mut [c64],
    lda: i32,
    tau: &mut [c64],
    work: &mut [c64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::zgeqrfp_(
        &m,
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        tau.as_mut_ptr() as *mut _,
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn cgeqrt(
    m: i32,
    n: i32,
    nb: i32,
    a: &mut [c32],
    lda: i32,
    t: &mut [c32],
    ldt: i32,
    work: &mut [c32],
    info: &mut i32,
) {
    ffi::cgeqrt_(
        &m,
        &n,
        &nb,
        a.as_mut_ptr() as *mut _,
        &lda,
        t.as_mut_ptr() as *mut _,
        &ldt,
        work.as_mut_ptr() as *mut _,
        info,
    )
}

#[inline]
pub unsafe fn dgeqrt(
    m: i32,
    n: i32,
    nb: i32,
    a: &mut [f64],
    lda: i32,
    t: &mut [f64],
    ldt: i32,
    work: &mut [f64],
    info: &mut i32,
) {
    ffi::dgeqrt_(
        &m,
        &n,
        &nb,
        a.as_mut_ptr(),
        &lda,
        t.as_mut_ptr(),
        &ldt,
        work.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn sgeqrt(
    m: i32,
    n: i32,
    nb: i32,
    a: &mut [f32],
    lda: i32,
    t: &mut [f32],
    ldt: i32,
    work: &mut [f32],
    info: &mut i32,
) {
    ffi::sgeqrt_(
        &m,
        &n,
        &nb,
        a.as_mut_ptr(),
        &lda,
        t.as_mut_ptr(),
        &ldt,
        work.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn zgeqrt(
    m: i32,
    n: i32,
    nb: i32,
    a: &mut [c64],
    lda: i32,
    t: &mut [c64],
    ldt: i32,
    work: &mut [c64],
    info: &mut i32,
) {
    ffi::zgeqrt_(
        &m,
        &n,
        &nb,
        a.as_mut_ptr() as *mut _,
        &lda,
        t.as_mut_ptr() as *mut _,
        &ldt,
        work.as_mut_ptr() as *mut _,
        info,
    )
}

#[inline]
pub unsafe fn cgeqrt2(
    m: i32,
    n: i32,
    a: &mut [c32],
    lda: i32,
    t: &mut [c32],
    ldt: i32,
    info: &mut i32,
) {
    ffi::cgeqrt2_(
        &m,
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        t.as_mut_ptr() as *mut _,
        &ldt,
        info,
    )
}

#[inline]
pub unsafe fn dgeqrt2(
    m: i32,
    n: i32,
    a: &mut [f64],
    lda: i32,
    t: &mut [f64],
    ldt: i32,
    info: &mut i32,
) {
    ffi::dgeqrt2_(&m, &n, a.as_mut_ptr(), &lda, t.as_mut_ptr(), &ldt, info)
}

#[inline]
pub unsafe fn sgeqrt2(
    m: i32,
    n: i32,
    a: &mut [f32],
    lda: i32,
    t: &mut [f32],
    ldt: i32,
    info: &mut i32,
) {
    ffi::sgeqrt2_(&m, &n, a.as_mut_ptr(), &lda, t.as_mut_ptr(), &ldt, info)
}

#[inline]
pub unsafe fn zgeqrt2(
    m: i32,
    n: i32,
    a: &mut [c64],
    lda: i32,
    t: &mut [c64],
    ldt: i32,
    info: &mut i32,
) {
    ffi::zgeqrt2_(
        &m,
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        t.as_mut_ptr() as *mut _,
        &ldt,
        info,
    )
}

#[inline]
pub unsafe fn cgeqrt3(
    m: i32,
    n: i32,
    a: &mut [c32],
    lda: i32,
    t: &mut [c32],
    ldt: i32,
    info: &mut i32,
) {
    ffi::cgeqrt3_(
        &m,
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        t.as_mut_ptr() as *mut _,
        &ldt,
        info,
    )
}

#[inline]
pub unsafe fn dgeqrt3(
    m: i32,
    n: i32,
    a: &mut [f64],
    lda: i32,
    t: &mut [f64],
    ldt: i32,
    info: &mut i32,
) {
    ffi::dgeqrt3_(&m, &n, a.as_mut_ptr(), &lda, t.as_mut_ptr(), &ldt, info)
}

#[inline]
pub unsafe fn sgeqrt3(
    m: i32,
    n: i32,
    a: &mut [f32],
    lda: i32,
    t: &mut [f32],
    ldt: i32,
    info: &mut i32,
) {
    ffi::sgeqrt3_(&m, &n, a.as_mut_ptr(), &lda, t.as_mut_ptr(), &ldt, info)
}

#[inline]
pub unsafe fn zgeqrt3(
    m: i32,
    n: i32,
    a: &mut [c64],
    lda: i32,
    t: &mut [c64],
    ldt: i32,
    info: &mut i32,
) {
    ffi::zgeqrt3_(
        &m,
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        t.as_mut_ptr() as *mut _,
        &ldt,
        info,
    )
}

#[inline]
pub unsafe fn cgerfs(
    trans: u8,
    n: i32,
    nrhs: i32,
    a: &[c32],
    lda: i32,
    af: &[c32],
    ldaf: i32,
    ipiv: &[i32],
    b: &[c32],
    ldb: i32,
    x: &mut [c32],
    ldx: i32,
    ferr: &mut [f32],
    berr: &mut [f32],
    work: &mut [c32],
    rwork: &mut [f32],
    info: &mut i32,
) {
    ffi::cgerfs_(
        &(trans as c_char),
        &n,
        &nrhs,
        a.as_ptr() as *const _,
        &lda,
        af.as_ptr() as *const _,
        &ldaf,
        ipiv.as_ptr(),
        b.as_ptr() as *const _,
        &ldb,
        x.as_mut_ptr() as *mut _,
        &ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn dgerfs(
    trans: u8,
    n: i32,
    nrhs: i32,
    a: &[f64],
    lda: i32,
    af: &[f64],
    ldaf: i32,
    ipiv: &[i32],
    b: &[f64],
    ldb: i32,
    x: &mut [f64],
    ldx: i32,
    ferr: &mut [f64],
    berr: &mut [f64],
    work: &mut [f64],
    iwork: &mut [i32],
    info: &mut i32,
) {
    ffi::dgerfs_(
        &(trans as c_char),
        &n,
        &nrhs,
        a.as_ptr(),
        &lda,
        af.as_ptr(),
        &ldaf,
        ipiv.as_ptr(),
        b.as_ptr(),
        &ldb,
        x.as_mut_ptr(),
        &ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn sgerfs(
    trans: u8,
    n: i32,
    nrhs: i32,
    a: &[f32],
    lda: i32,
    af: &[f32],
    ldaf: i32,
    ipiv: &[i32],
    b: &[f32],
    ldb: i32,
    x: &mut [f32],
    ldx: i32,
    ferr: &mut [f32],
    berr: &mut [f32],
    work: &mut [f32],
    iwork: &mut [i32],
    info: &mut i32,
) {
    ffi::sgerfs_(
        &(trans as c_char),
        &n,
        &nrhs,
        a.as_ptr(),
        &lda,
        af.as_ptr(),
        &ldaf,
        ipiv.as_ptr(),
        b.as_ptr(),
        &ldb,
        x.as_mut_ptr(),
        &ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn zgerfs(
    trans: u8,
    n: i32,
    nrhs: i32,
    a: &[c64],
    lda: i32,
    af: &[c64],
    ldaf: i32,
    ipiv: &[i32],
    b: &[c64],
    ldb: i32,
    x: &mut [c64],
    ldx: i32,
    ferr: &mut [f64],
    berr: &mut [f64],
    work: &mut [c64],
    rwork: &mut [f64],
    info: &mut i32,
) {
    ffi::zgerfs_(
        &(trans as c_char),
        &n,
        &nrhs,
        a.as_ptr() as *const _,
        &lda,
        af.as_ptr() as *const _,
        &ldaf,
        ipiv.as_ptr(),
        b.as_ptr() as *const _,
        &ldb,
        x.as_mut_ptr() as *mut _,
        &ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn cgerfsx(
    trans: u8,
    equed: u8,
    n: i32,
    nrhs: i32,
    a: &[c32],
    lda: i32,
    af: &[c32],
    ldaf: i32,
    ipiv: &[i32],
    r: &[f32],
    c: &[f32],
    b: &[c32],
    ldb: i32,
    x: &mut [c32],
    ldx: i32,
    rcond: &mut f32,
    berr: &mut [f32],
    n_err_bnds: i32,
    err_bnds_norm: &mut [f32],
    err_bnds_comp: &mut [f32],
    nparams: &[i32],
    params: &mut [f32],
    work: &mut [c32],
    rwork: &mut [f32],
    info: &mut i32,
) {
    ffi::cgerfsx_(
        &(trans as c_char),
        &(equed as c_char),
        &n,
        &nrhs,
        a.as_ptr() as *const _,
        &lda,
        af.as_ptr() as *const _,
        &ldaf,
        ipiv.as_ptr(),
        r.as_ptr(),
        c.as_ptr(),
        b.as_ptr() as *const _,
        &ldb,
        x.as_mut_ptr() as *mut _,
        &ldx,
        rcond,
        berr.as_mut_ptr(),
        &n_err_bnds,
        err_bnds_norm.as_mut_ptr(),
        err_bnds_comp.as_mut_ptr(),
        nparams.as_ptr(),
        params.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn dgerfsx(
    trans: u8,
    equed: u8,
    n: i32,
    nrhs: i32,
    a: &[f64],
    lda: i32,
    af: &[f64],
    ldaf: i32,
    ipiv: &[i32],
    r: &[f64],
    c: &[f64],
    b: &[f64],
    ldb: i32,
    x: &mut [f64],
    ldx: i32,
    rcond: &mut f64,
    berr: &mut [f64],
    n_err_bnds: i32,
    err_bnds_norm: &mut [f64],
    err_bnds_comp: &mut [f64],
    nparams: &[i32],
    params: &mut [f64],
    work: &mut [f64],
    iwork: &mut [i32],
    info: &mut i32,
) {
    ffi::dgerfsx_(
        &(trans as c_char),
        &(equed as c_char),
        &n,
        &nrhs,
        a.as_ptr(),
        &lda,
        af.as_ptr(),
        &ldaf,
        ipiv.as_ptr(),
        r.as_ptr(),
        c.as_ptr(),
        b.as_ptr(),
        &ldb,
        x.as_mut_ptr(),
        &ldx,
        rcond,
        berr.as_mut_ptr(),
        &n_err_bnds,
        err_bnds_norm.as_mut_ptr(),
        err_bnds_comp.as_mut_ptr(),
        nparams.as_ptr(),
        params.as_mut_ptr(),
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn sgerfsx(
    trans: u8,
    equed: u8,
    n: i32,
    nrhs: i32,
    a: &[f32],
    lda: i32,
    af: &[f32],
    ldaf: i32,
    ipiv: &[i32],
    r: &[f32],
    c: &[f32],
    b: &[f32],
    ldb: i32,
    x: &mut [f32],
    ldx: i32,
    rcond: &mut f32,
    berr: &mut [f32],
    n_err_bnds: i32,
    err_bnds_norm: &mut [f32],
    err_bnds_comp: &mut [f32],
    nparams: &[i32],
    params: &mut [f32],
    work: &mut [f32],
    iwork: &mut [i32],
    info: &mut i32,
) {
    ffi::sgerfsx_(
        &(trans as c_char),
        &(equed as c_char),
        &n,
        &nrhs,
        a.as_ptr(),
        &lda,
        af.as_ptr(),
        &ldaf,
        ipiv.as_ptr(),
        r.as_ptr(),
        c.as_ptr(),
        b.as_ptr(),
        &ldb,
        x.as_mut_ptr(),
        &ldx,
        rcond,
        berr.as_mut_ptr(),
        &n_err_bnds,
        err_bnds_norm.as_mut_ptr(),
        err_bnds_comp.as_mut_ptr(),
        nparams.as_ptr(),
        params.as_mut_ptr(),
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn zgerfsx(
    trans: u8,
    equed: u8,
    n: i32,
    nrhs: i32,
    a: &[c64],
    lda: i32,
    af: &[c64],
    ldaf: i32,
    ipiv: &[i32],
    r: &[f64],
    c: &[f64],
    b: &[c64],
    ldb: i32,
    x: &mut [c64],
    ldx: i32,
    rcond: &mut f64,
    berr: &mut [f64],
    n_err_bnds: i32,
    err_bnds_norm: &mut [f64],
    err_bnds_comp: &mut [f64],
    nparams: &[i32],
    params: &mut [f64],
    work: &mut [c64],
    rwork: &mut [f64],
    info: &mut i32,
) {
    ffi::zgerfsx_(
        &(trans as c_char),
        &(equed as c_char),
        &n,
        &nrhs,
        a.as_ptr() as *const _,
        &lda,
        af.as_ptr() as *const _,
        &ldaf,
        ipiv.as_ptr(),
        r.as_ptr(),
        c.as_ptr(),
        b.as_ptr() as *const _,
        &ldb,
        x.as_mut_ptr() as *mut _,
        &ldx,
        rcond,
        berr.as_mut_ptr(),
        &n_err_bnds,
        err_bnds_norm.as_mut_ptr(),
        err_bnds_comp.as_mut_ptr(),
        nparams.as_ptr(),
        params.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn cgerq2(
    m: i32,
    n: i32,
    a: &mut [c32],
    lda: i32,
    tau: &mut [c32],
    work: &mut [c32],
    info: &mut i32,
) {
    ffi::cgerq2_(
        &m,
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        tau.as_mut_ptr() as *mut _,
        work.as_mut_ptr() as *mut _,
        info,
    )
}

#[inline]
pub unsafe fn dgerq2(
    m: i32,
    n: i32,
    a: &mut [f64],
    lda: i32,
    tau: &mut [f64],
    work: &mut [f64],
    info: &mut i32,
) {
    ffi::dgerq2_(
        &m,
        &n,
        a.as_mut_ptr(),
        &lda,
        tau.as_mut_ptr(),
        work.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn sgerq2(
    m: i32,
    n: i32,
    a: &mut [f32],
    lda: i32,
    tau: &mut [f32],
    work: &mut [f32],
    info: &mut i32,
) {
    ffi::sgerq2_(
        &m,
        &n,
        a.as_mut_ptr(),
        &lda,
        tau.as_mut_ptr(),
        work.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn zgerq2(
    m: i32,
    n: i32,
    a: &mut [c64],
    lda: i32,
    tau: &mut [c64],
    work: &mut [c64],
    info: &mut i32,
) {
    ffi::zgerq2_(
        &m,
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        tau.as_mut_ptr() as *mut _,
        work.as_mut_ptr() as *mut _,
        info,
    )
}

#[inline]
pub unsafe fn cgerqf(
    m: i32,
    n: i32,
    a: &mut [c32],
    lda: i32,
    tau: &mut [c32],
    work: &mut [c32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::cgerqf_(
        &m,
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        tau.as_mut_ptr() as *mut _,
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn dgerqf(
    m: i32,
    n: i32,
    a: &mut [f64],
    lda: i32,
    tau: &mut [f64],
    work: &mut [f64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::dgerqf_(
        &m,
        &n,
        a.as_mut_ptr(),
        &lda,
        tau.as_mut_ptr(),
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn sgerqf(
    m: i32,
    n: i32,
    a: &mut [f32],
    lda: i32,
    tau: &mut [f32],
    work: &mut [f32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::sgerqf_(
        &m,
        &n,
        a.as_mut_ptr(),
        &lda,
        tau.as_mut_ptr(),
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn zgerqf(
    m: i32,
    n: i32,
    a: &mut [c64],
    lda: i32,
    tau: &mut [c64],
    work: &mut [c64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::zgerqf_(
        &m,
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        tau.as_mut_ptr() as *mut _,
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn cgesdd(
    jobz: u8,
    m: i32,
    n: i32,
    a: &mut [c32],
    lda: i32,
    s: &mut [f32],
    u: &mut [c32],
    ldu: i32,
    vt: &mut [c32],
    ldvt: i32,
    work: &mut [c32],
    lwork: i32,
    rwork: &mut [f32],
    iwork: &mut [i32],
    info: &mut i32,
) {
    ffi::cgesdd_(
        &(jobz as c_char),
        &m,
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        s.as_mut_ptr(),
        u.as_mut_ptr() as *mut _,
        &ldu,
        vt.as_mut_ptr() as *mut _,
        &ldvt,
        work.as_mut_ptr() as *mut _,
        &lwork,
        rwork.as_mut_ptr(),
        iwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn dgesdd(
    jobz: u8,
    m: i32,
    n: i32,
    a: &mut [f64],
    lda: i32,
    s: &mut [f64],
    u: &mut [f64],
    ldu: i32,
    vt: &mut [f64],
    ldvt: i32,
    work: &mut [f64],
    lwork: i32,
    iwork: &mut [i32],
    info: &mut i32,
) {
    ffi::dgesdd_(
        &(jobz as c_char),
        &m,
        &n,
        a.as_mut_ptr(),
        &lda,
        s.as_mut_ptr(),
        u.as_mut_ptr(),
        &ldu,
        vt.as_mut_ptr(),
        &ldvt,
        work.as_mut_ptr(),
        &lwork,
        iwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn sgesdd(
    jobz: u8,
    m: i32,
    n: i32,
    a: &mut [f32],
    lda: i32,
    s: &mut [f32],
    u: &mut [f32],
    ldu: i32,
    vt: &mut [f32],
    ldvt: i32,
    work: &mut [f32],
    lwork: i32,
    iwork: &mut [i32],
    info: &mut i32,
) {
    ffi::sgesdd_(
        &(jobz as c_char),
        &m,
        &n,
        a.as_mut_ptr(),
        &lda,
        s.as_mut_ptr(),
        u.as_mut_ptr(),
        &ldu,
        vt.as_mut_ptr(),
        &ldvt,
        work.as_mut_ptr(),
        &lwork,
        iwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn zgesdd(
    jobz: u8,
    m: i32,
    n: i32,
    a: &mut [c64],
    lda: i32,
    s: &mut [f64],
    u: &mut [c64],
    ldu: i32,
    vt: &mut [c64],
    ldvt: i32,
    work: &mut [c64],
    lwork: i32,
    rwork: &mut [f64],
    iwork: &mut [i32],
    info: &mut i32,
) {
    ffi::zgesdd_(
        &(jobz as c_char),
        &m,
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        s.as_mut_ptr(),
        u.as_mut_ptr() as *mut _,
        &ldu,
        vt.as_mut_ptr() as *mut _,
        &ldvt,
        work.as_mut_ptr() as *mut _,
        &lwork,
        rwork.as_mut_ptr(),
        iwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn cgesv(
    n: i32,
    nrhs: i32,
    a: &mut [c32],
    lda: i32,
    ipiv: &mut [i32],
    b: &mut [c32],
    ldb: i32,
    info: &mut i32,
) {
    ffi::cgesv_(
        &n,
        &nrhs,
        a.as_mut_ptr() as *mut _,
        &lda,
        ipiv.as_mut_ptr(),
        b.as_mut_ptr() as *mut _,
        &ldb,
        info,
    )
}

#[inline]
pub unsafe fn dgesv(
    n: i32,
    nrhs: i32,
    a: &mut [f64],
    lda: i32,
    ipiv: &mut [i32],
    b: &mut [f64],
    ldb: i32,
    info: &mut i32,
) {
    ffi::dgesv_(
        &n,
        &nrhs,
        a.as_mut_ptr(),
        &lda,
        ipiv.as_mut_ptr(),
        b.as_mut_ptr(),
        &ldb,
        info,
    )
}

#[inline]
pub unsafe fn sgesv(
    n: i32,
    nrhs: i32,
    a: &mut [f32],
    lda: i32,
    ipiv: &mut [i32],
    b: &mut [f32],
    ldb: i32,
    info: &mut i32,
) {
    ffi::sgesv_(
        &n,
        &nrhs,
        a.as_mut_ptr(),
        &lda,
        ipiv.as_mut_ptr(),
        b.as_mut_ptr(),
        &ldb,
        info,
    )
}

#[inline]
pub unsafe fn zgesv(
    n: i32,
    nrhs: i32,
    a: &mut [c64],
    lda: i32,
    ipiv: &mut [i32],
    b: &mut [c64],
    ldb: i32,
    info: &mut i32,
) {
    ffi::zgesv_(
        &n,
        &nrhs,
        a.as_mut_ptr() as *mut _,
        &lda,
        ipiv.as_mut_ptr(),
        b.as_mut_ptr() as *mut _,
        &ldb,
        info,
    )
}

#[inline]
pub unsafe fn dsgesv(
    n: i32,
    nrhs: i32,
    a: &mut [f64],
    lda: i32,
    ipiv: &mut [i32],
    b: &[f64],
    ldb: i32,
    x: &mut [f64],
    ldx: i32,
    work: &mut [f64],
    swork: &mut [f32],
    iter: &mut i32,
    info: &mut i32,
) {
    ffi::dsgesv_(
        &n,
        &nrhs,
        a.as_mut_ptr(),
        &lda,
        ipiv.as_mut_ptr(),
        b.as_ptr(),
        &ldb,
        x.as_mut_ptr(),
        &ldx,
        work.as_mut_ptr(),
        swork.as_mut_ptr(),
        iter,
        info,
    )
}

#[inline]
pub unsafe fn zcgesv(
    n: i32,
    nrhs: i32,
    a: &mut [c64],
    lda: i32,
    ipiv: &mut [i32],
    b: &[c64],
    ldb: i32,
    x: &mut [c64],
    ldx: i32,
    work: &mut [c64],
    swork: &mut [c32],
    rwork: &mut [f64],
    iter: &mut i32,
    info: &mut i32,
) {
    ffi::zcgesv_(
        &n,
        &nrhs,
        a.as_mut_ptr() as *mut _,
        &lda,
        ipiv.as_mut_ptr(),
        b.as_ptr() as *const _,
        &ldb,
        x.as_mut_ptr() as *mut _,
        &ldx,
        work.as_mut_ptr() as *mut _,
        swork.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
        iter,
        info,
    )
}

#[inline]
pub unsafe fn cgesvd(
    jobu: u8,
    jobvt: u8,
    m: i32,
    n: i32,
    a: &mut [c32],
    lda: i32,
    s: &mut [f32],
    u: &mut [c32],
    ldu: i32,
    vt: &mut [c32],
    ldvt: i32,
    work: &mut [c32],
    lwork: i32,
    rwork: &mut [f32],
    info: &mut i32,
) {
    ffi::cgesvd_(
        &(jobu as c_char),
        &(jobvt as c_char),
        &m,
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        s.as_mut_ptr(),
        u.as_mut_ptr() as *mut _,
        &ldu,
        vt.as_mut_ptr() as *mut _,
        &ldvt,
        work.as_mut_ptr() as *mut _,
        &lwork,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn dgesvd(
    jobu: u8,
    jobvt: u8,
    m: i32,
    n: i32,
    a: &mut [f64],
    lda: i32,
    s: &mut [f64],
    u: &mut [f64],
    ldu: i32,
    vt: &mut [f64],
    ldvt: i32,
    work: &mut [f64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::dgesvd_(
        &(jobu as c_char),
        &(jobvt as c_char),
        &m,
        &n,
        a.as_mut_ptr(),
        &lda,
        s.as_mut_ptr(),
        u.as_mut_ptr(),
        &ldu,
        vt.as_mut_ptr(),
        &ldvt,
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn sgesvd(
    jobu: u8,
    jobvt: u8,
    m: i32,
    n: i32,
    a: &mut [f32],
    lda: i32,
    s: &mut [f32],
    u: &mut [f32],
    ldu: i32,
    vt: &mut [f32],
    ldvt: i32,
    work: &mut [f32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::sgesvd_(
        &(jobu as c_char),
        &(jobvt as c_char),
        &m,
        &n,
        a.as_mut_ptr(),
        &lda,
        s.as_mut_ptr(),
        u.as_mut_ptr(),
        &ldu,
        vt.as_mut_ptr(),
        &ldvt,
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn zgesvd(
    jobu: u8,
    jobvt: u8,
    m: i32,
    n: i32,
    a: &mut [c64],
    lda: i32,
    s: &mut [f64],
    u: &mut [c64],
    ldu: i32,
    vt: &mut [c64],
    ldvt: i32,
    work: &mut [c64],
    lwork: i32,
    rwork: &mut [f64],
    info: &mut i32,
) {
    ffi::zgesvd_(
        &(jobu as c_char),
        &(jobvt as c_char),
        &m,
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        s.as_mut_ptr(),
        u.as_mut_ptr() as *mut _,
        &ldu,
        vt.as_mut_ptr() as *mut _,
        &ldvt,
        work.as_mut_ptr() as *mut _,
        &lwork,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn cgesvdq(
    joba: u8,
    jobp: u8,
    jobr: u8,
    jobu: u8,
    jobv: u8,
    m: i32,
    n: i32,
    a: &mut [c32],
    lda: i32,
    s: &mut [f32],
    u: &mut [c32],
    ldu: i32,
    v: &mut [c32],
    ldv: i32,
    numrank: &mut [i32],
    iwork: &mut [i32],
    liwork: i32,
    cwork: &mut [c32],
    lcwork: &mut [i32],
    rwork: &mut [f32],
    lrwork: i32,
    info: &mut i32,
) {
    ffi::cgesvdq_(
        &(joba as c_char),
        &(jobp as c_char),
        &(jobr as c_char),
        &(jobu as c_char),
        &(jobv as c_char),
        &m,
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        s.as_mut_ptr(),
        u.as_mut_ptr() as *mut _,
        &ldu,
        v.as_mut_ptr() as *mut _,
        &ldv,
        numrank.as_mut_ptr(),
        iwork.as_mut_ptr(),
        &liwork,
        cwork.as_mut_ptr() as *mut _,
        lcwork.as_mut_ptr(),
        rwork.as_mut_ptr(),
        &lrwork,
        info,
    )
}

#[inline]
pub unsafe fn dgesvdq(
    joba: u8,
    jobp: u8,
    jobr: u8,
    jobu: u8,
    jobv: u8,
    m: i32,
    n: i32,
    a: &mut [f64],
    lda: i32,
    s: &mut [f64],
    u: &mut [f64],
    ldu: i32,
    v: &mut [f64],
    ldv: i32,
    numrank: &mut [i32],
    iwork: &mut [i32],
    liwork: i32,
    work: &mut [f64],
    lwork: &mut i32,
    rwork: &mut [f64],
    lrwork: i32,
    info: &mut i32,
) {
    ffi::dgesvdq_(
        &(joba as c_char),
        &(jobp as c_char),
        &(jobr as c_char),
        &(jobu as c_char),
        &(jobv as c_char),
        &m,
        &n,
        a.as_mut_ptr(),
        &lda,
        s.as_mut_ptr(),
        u.as_mut_ptr(),
        &ldu,
        v.as_mut_ptr(),
        &ldv,
        numrank.as_mut_ptr(),
        iwork.as_mut_ptr(),
        &liwork,
        work.as_mut_ptr(),
        lwork,
        rwork.as_mut_ptr(),
        &lrwork,
        info,
    )
}

#[inline]
pub unsafe fn sgesvdq(
    joba: u8,
    jobp: u8,
    jobr: u8,
    jobu: u8,
    jobv: u8,
    m: i32,
    n: i32,
    a: &mut [f32],
    lda: i32,
    s: &mut [f32],
    u: &mut [f32],
    ldu: i32,
    v: &mut [f32],
    ldv: i32,
    numrank: &mut [i32],
    iwork: &mut [i32],
    liwork: i32,
    work: &mut [f32],
    lwork: &mut i32,
    rwork: &mut [f32],
    lrwork: i32,
    info: &mut i32,
) {
    ffi::sgesvdq_(
        &(joba as c_char),
        &(jobp as c_char),
        &(jobr as c_char),
        &(jobu as c_char),
        &(jobv as c_char),
        &m,
        &n,
        a.as_mut_ptr(),
        &lda,
        s.as_mut_ptr(),
        u.as_mut_ptr(),
        &ldu,
        v.as_mut_ptr(),
        &ldv,
        numrank.as_mut_ptr(),
        iwork.as_mut_ptr(),
        &liwork,
        work.as_mut_ptr(),
        lwork,
        rwork.as_mut_ptr(),
        &lrwork,
        info,
    )
}

#[inline]
pub unsafe fn zgesvdq(
    joba: u8,
    jobp: u8,
    jobr: u8,
    jobu: u8,
    jobv: u8,
    m: i32,
    n: i32,
    a: &mut [c64],
    lda: i32,
    s: &mut [f64],
    u: &mut [c64],
    ldu: i32,
    v: &mut [c64],
    ldv: i32,
    numrank: &mut [i32],
    iwork: &mut [i32],
    liwork: i32,
    cwork: &mut [c64],
    lcwork: &mut [i32],
    rwork: &mut [f64],
    lrwork: i32,
    info: &mut i32,
) {
    ffi::zgesvdq_(
        &(joba as c_char),
        &(jobp as c_char),
        &(jobr as c_char),
        &(jobu as c_char),
        &(jobv as c_char),
        &m,
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        s.as_mut_ptr(),
        u.as_mut_ptr() as *mut _,
        &ldu,
        v.as_mut_ptr() as *mut _,
        &ldv,
        numrank.as_mut_ptr(),
        iwork.as_mut_ptr(),
        &liwork,
        cwork.as_mut_ptr() as *mut _,
        lcwork.as_mut_ptr(),
        rwork.as_mut_ptr(),
        &lrwork,
        info,
    )
}

#[inline]
pub unsafe fn cgesvdx(
    jobu: u8,
    jobvt: u8,
    range: u8,
    m: i32,
    n: i32,
    a: &mut [c32],
    lda: i32,
    vl: f32,
    vu: f32,
    il: i32,
    iu: i32,
    ns: &mut [i32],
    s: &mut [f32],
    u: &mut [c32],
    ldu: i32,
    vt: &mut [c32],
    ldvt: i32,
    work: &mut [c32],
    lwork: i32,
    rwork: &mut [f32],
    iwork: &mut [i32],
    info: &mut i32,
) {
    ffi::cgesvdx_(
        &(jobu as c_char),
        &(jobvt as c_char),
        &(range as c_char),
        &m,
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        &vl,
        &vu,
        &il,
        &iu,
        ns.as_mut_ptr(),
        s.as_mut_ptr(),
        u.as_mut_ptr() as *mut _,
        &ldu,
        vt.as_mut_ptr() as *mut _,
        &ldvt,
        work.as_mut_ptr() as *mut _,
        &lwork,
        rwork.as_mut_ptr(),
        iwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn dgesvdx(
    jobu: u8,
    jobvt: u8,
    range: u8,
    m: i32,
    n: i32,
    a: &mut [f64],
    lda: i32,
    vl: f64,
    vu: f64,
    il: i32,
    iu: i32,
    ns: &mut [i32],
    s: &mut [f64],
    u: &mut [f64],
    ldu: i32,
    vt: &mut [f64],
    ldvt: i32,
    work: &mut [f64],
    lwork: i32,
    iwork: &mut [i32],
    info: &mut i32,
) {
    ffi::dgesvdx_(
        &(jobu as c_char),
        &(jobvt as c_char),
        &(range as c_char),
        &m,
        &n,
        a.as_mut_ptr(),
        &lda,
        &vl,
        &vu,
        &il,
        &iu,
        ns.as_mut_ptr(),
        s.as_mut_ptr(),
        u.as_mut_ptr(),
        &ldu,
        vt.as_mut_ptr(),
        &ldvt,
        work.as_mut_ptr(),
        &lwork,
        iwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn sgesvdx(
    jobu: u8,
    jobvt: u8,
    range: u8,
    m: i32,
    n: i32,
    a: &mut [f32],
    lda: i32,
    vl: f32,
    vu: f32,
    il: i32,
    iu: i32,
    ns: &mut [i32],
    s: &mut [f32],
    u: &mut [f32],
    ldu: i32,
    vt: &mut [f32],
    ldvt: i32,
    work: &mut [f32],
    lwork: i32,
    iwork: &mut [i32],
    info: &mut i32,
) {
    ffi::sgesvdx_(
        &(jobu as c_char),
        &(jobvt as c_char),
        &(range as c_char),
        &m,
        &n,
        a.as_mut_ptr(),
        &lda,
        &vl,
        &vu,
        &il,
        &iu,
        ns.as_mut_ptr(),
        s.as_mut_ptr(),
        u.as_mut_ptr(),
        &ldu,
        vt.as_mut_ptr(),
        &ldvt,
        work.as_mut_ptr(),
        &lwork,
        iwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn zgesvdx(
    jobu: u8,
    jobvt: u8,
    range: u8,
    m: i32,
    n: i32,
    a: &mut [c64],
    lda: i32,
    vl: f64,
    vu: f64,
    il: i32,
    iu: i32,
    ns: &mut [i32],
    s: &mut [f64],
    u: &mut [c64],
    ldu: i32,
    vt: &mut [c64],
    ldvt: i32,
    work: &mut [c64],
    lwork: i32,
    rwork: &mut [f64],
    iwork: &mut [i32],
    info: &mut i32,
) {
    ffi::zgesvdx_(
        &(jobu as c_char),
        &(jobvt as c_char),
        &(range as c_char),
        &m,
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        &vl,
        &vu,
        &il,
        &iu,
        ns.as_mut_ptr(),
        s.as_mut_ptr(),
        u.as_mut_ptr() as *mut _,
        &ldu,
        vt.as_mut_ptr() as *mut _,
        &ldvt,
        work.as_mut_ptr() as *mut _,
        &lwork,
        rwork.as_mut_ptr(),
        iwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn cgesvj(
    joba: u8,
    jobu: u8,
    jobv: u8,
    m: i32,
    n: i32,
    a: &mut [c32],
    lda: i32,
    sva: &mut [f32],
    mv: &[i32],
    v: &mut [c32],
    ldv: i32,
    cwork: &mut [c32],
    lwork: i32,
    rwork: &mut [f32],
    lrwork: i32,
    info: &mut i32,
) {
    ffi::cgesvj_(
        &(joba as c_char),
        &(jobu as c_char),
        &(jobv as c_char),
        &m,
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        sva.as_mut_ptr(),
        mv.as_ptr(),
        v.as_mut_ptr() as *mut _,
        &ldv,
        cwork.as_mut_ptr() as *mut _,
        &lwork,
        rwork.as_mut_ptr(),
        &lrwork,
        info,
    )
}

#[inline]
pub unsafe fn dgesvj(
    joba: u8,
    jobu: u8,
    jobv: u8,
    m: i32,
    n: i32,
    a: &mut [f64],
    lda: i32,
    sva: &mut [f64],
    mv: &[i32],
    v: &mut [f64],
    ldv: i32,
    work: &mut [f64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::dgesvj_(
        &(joba as c_char),
        &(jobu as c_char),
        &(jobv as c_char),
        &m,
        &n,
        a.as_mut_ptr(),
        &lda,
        sva.as_mut_ptr(),
        mv.as_ptr(),
        v.as_mut_ptr(),
        &ldv,
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn sgesvj(
    joba: u8,
    jobu: u8,
    jobv: u8,
    m: i32,
    n: i32,
    a: &mut [f32],
    lda: i32,
    sva: &mut [f32],
    mv: &[i32],
    v: &mut [f32],
    ldv: i32,
    work: &mut [f32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::sgesvj_(
        &(joba as c_char),
        &(jobu as c_char),
        &(jobv as c_char),
        &m,
        &n,
        a.as_mut_ptr(),
        &lda,
        sva.as_mut_ptr(),
        mv.as_ptr(),
        v.as_mut_ptr(),
        &ldv,
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn zgesvj(
    joba: u8,
    jobu: u8,
    jobv: u8,
    m: i32,
    n: i32,
    a: &mut [c64],
    lda: i32,
    sva: &mut [f64],
    mv: &[i32],
    v: &mut [c64],
    ldv: i32,
    cwork: &mut [c64],
    lwork: i32,
    rwork: &mut [f64],
    lrwork: i32,
    info: &mut i32,
) {
    ffi::zgesvj_(
        &(joba as c_char),
        &(jobu as c_char),
        &(jobv as c_char),
        &m,
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        sva.as_mut_ptr(),
        mv.as_ptr(),
        v.as_mut_ptr() as *mut _,
        &ldv,
        cwork.as_mut_ptr() as *mut _,
        &lwork,
        rwork.as_mut_ptr(),
        &lrwork,
        info,
    )
}

#[inline]
pub unsafe fn cgesvx(
    fact: u8,
    trans: u8,
    n: i32,
    nrhs: i32,
    a: &mut [c32],
    lda: i32,
    af: &mut [c32],
    ldaf: i32,
    ipiv: &mut [i32],
    equed: &mut u8,
    r: &mut [f32],
    c: &mut [f32],
    b: &mut [c32],
    ldb: i32,
    x: &mut [c32],
    ldx: i32,
    rcond: &mut f32,
    ferr: &mut [f32],
    berr: &mut [f32],
    work: &mut [c32],
    rwork: &mut [f32],
    info: &mut i32,
) {
    ffi::cgesvx_(
        &(fact as c_char),
        &(trans as c_char),
        &n,
        &nrhs,
        a.as_mut_ptr() as *mut _,
        &lda,
        af.as_mut_ptr() as *mut _,
        &ldaf,
        ipiv.as_mut_ptr(),
        equed as *mut _ as *mut _,
        r.as_mut_ptr(),
        c.as_mut_ptr(),
        b.as_mut_ptr() as *mut _,
        &ldb,
        x.as_mut_ptr() as *mut _,
        &ldx,
        rcond,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn dgesvx(
    fact: u8,
    trans: u8,
    n: i32,
    nrhs: i32,
    a: &mut [f64],
    lda: i32,
    af: &mut [f64],
    ldaf: i32,
    ipiv: &mut [i32],
    equed: &mut u8,
    r: &mut [f64],
    c: &mut [f64],
    b: &mut [f64],
    ldb: i32,
    x: &mut [f64],
    ldx: i32,
    rcond: &mut f64,
    ferr: &mut [f64],
    berr: &mut [f64],
    work: &mut [f64],
    iwork: &mut [i32],
    info: &mut i32,
) {
    ffi::dgesvx_(
        &(fact as c_char),
        &(trans as c_char),
        &n,
        &nrhs,
        a.as_mut_ptr(),
        &lda,
        af.as_mut_ptr(),
        &ldaf,
        ipiv.as_mut_ptr(),
        equed as *mut _ as *mut _,
        r.as_mut_ptr(),
        c.as_mut_ptr(),
        b.as_mut_ptr(),
        &ldb,
        x.as_mut_ptr(),
        &ldx,
        rcond,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn sgesvx(
    fact: u8,
    trans: u8,
    n: i32,
    nrhs: i32,
    a: &mut [f32],
    lda: i32,
    af: &mut [f32],
    ldaf: i32,
    ipiv: &mut [i32],
    equed: &mut u8,
    r: &mut [f32],
    c: &mut [f32],
    b: &mut [f32],
    ldb: i32,
    x: &mut [f32],
    ldx: i32,
    rcond: &mut f32,
    ferr: &mut [f32],
    berr: &mut [f32],
    work: &mut [f32],
    iwork: &mut [i32],
    info: &mut i32,
) {
    ffi::sgesvx_(
        &(fact as c_char),
        &(trans as c_char),
        &n,
        &nrhs,
        a.as_mut_ptr(),
        &lda,
        af.as_mut_ptr(),
        &ldaf,
        ipiv.as_mut_ptr(),
        equed as *mut _ as *mut _,
        r.as_mut_ptr(),
        c.as_mut_ptr(),
        b.as_mut_ptr(),
        &ldb,
        x.as_mut_ptr(),
        &ldx,
        rcond,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn zgesvx(
    fact: u8,
    trans: u8,
    n: i32,
    nrhs: i32,
    a: &mut [c64],
    lda: i32,
    af: &mut [c64],
    ldaf: i32,
    ipiv: &mut [i32],
    equed: &mut u8,
    r: &mut [f64],
    c: &mut [f64],
    b: &mut [c64],
    ldb: i32,
    x: &mut [c64],
    ldx: i32,
    rcond: &mut f64,
    ferr: &mut [f64],
    berr: &mut [f64],
    work: &mut [c64],
    rwork: &mut [f64],
    info: &mut i32,
) {
    ffi::zgesvx_(
        &(fact as c_char),
        &(trans as c_char),
        &n,
        &nrhs,
        a.as_mut_ptr() as *mut _,
        &lda,
        af.as_mut_ptr() as *mut _,
        &ldaf,
        ipiv.as_mut_ptr(),
        equed as *mut _ as *mut _,
        r.as_mut_ptr(),
        c.as_mut_ptr(),
        b.as_mut_ptr() as *mut _,
        &ldb,
        x.as_mut_ptr() as *mut _,
        &ldx,
        rcond,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn cgesvxx(
    fact: u8,
    trans: u8,
    n: i32,
    nrhs: i32,
    a: &mut [c32],
    lda: i32,
    af: &mut [c32],
    ldaf: i32,
    ipiv: &mut [i32],
    equed: &mut u8,
    r: &mut [f32],
    c: &mut [f32],
    b: &mut [c32],
    ldb: i32,
    x: &mut [c32],
    ldx: i32,
    rcond: &mut f32,
    rpvgrw: &mut f32,
    berr: &mut [f32],
    n_err_bnds: i32,
    err_bnds_norm: &mut [f32],
    err_bnds_comp: &mut [f32],
    nparams: &[i32],
    params: &mut [f32],
    work: &mut [c32],
    rwork: &mut [f32],
    info: &mut i32,
) {
    ffi::cgesvxx_(
        &(fact as c_char),
        &(trans as c_char),
        &n,
        &nrhs,
        a.as_mut_ptr() as *mut _,
        &lda,
        af.as_mut_ptr() as *mut _,
        &ldaf,
        ipiv.as_mut_ptr(),
        equed as *mut _ as *mut _,
        r.as_mut_ptr(),
        c.as_mut_ptr(),
        b.as_mut_ptr() as *mut _,
        &ldb,
        x.as_mut_ptr() as *mut _,
        &ldx,
        rcond,
        rpvgrw,
        berr.as_mut_ptr(),
        &n_err_bnds,
        err_bnds_norm.as_mut_ptr(),
        err_bnds_comp.as_mut_ptr(),
        nparams.as_ptr(),
        params.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn dgesvxx(
    fact: u8,
    trans: u8,
    n: i32,
    nrhs: i32,
    a: &mut [f64],
    lda: i32,
    af: &mut [f64],
    ldaf: i32,
    ipiv: &mut [i32],
    equed: &mut u8,
    r: &mut [f64],
    c: &mut [f64],
    b: &mut [f64],
    ldb: i32,
    x: &mut [f64],
    ldx: i32,
    rcond: &mut f64,
    rpvgrw: &mut f64,
    berr: &mut [f64],
    n_err_bnds: i32,
    err_bnds_norm: &mut [f64],
    err_bnds_comp: &mut [f64],
    nparams: &[i32],
    params: &mut [f64],
    work: &mut [f64],
    iwork: &mut [i32],
    info: &mut i32,
) {
    ffi::dgesvxx_(
        &(fact as c_char),
        &(trans as c_char),
        &n,
        &nrhs,
        a.as_mut_ptr(),
        &lda,
        af.as_mut_ptr(),
        &ldaf,
        ipiv.as_mut_ptr(),
        equed as *mut _ as *mut _,
        r.as_mut_ptr(),
        c.as_mut_ptr(),
        b.as_mut_ptr(),
        &ldb,
        x.as_mut_ptr(),
        &ldx,
        rcond,
        rpvgrw,
        berr.as_mut_ptr(),
        &n_err_bnds,
        err_bnds_norm.as_mut_ptr(),
        err_bnds_comp.as_mut_ptr(),
        nparams.as_ptr(),
        params.as_mut_ptr(),
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn sgesvxx(
    fact: u8,
    trans: u8,
    n: i32,
    nrhs: i32,
    a: &mut [f32],
    lda: i32,
    af: &mut [f32],
    ldaf: i32,
    ipiv: &mut [i32],
    equed: &mut u8,
    r: &mut [f32],
    c: &mut [f32],
    b: &mut [f32],
    ldb: i32,
    x: &mut [f32],
    ldx: i32,
    rcond: &mut f32,
    rpvgrw: &mut f32,
    berr: &mut [f32],
    n_err_bnds: i32,
    err_bnds_norm: &mut [f32],
    err_bnds_comp: &mut [f32],
    nparams: &[i32],
    params: &mut [f32],
    work: &mut [f32],
    iwork: &mut [i32],
    info: &mut i32,
) {
    ffi::sgesvxx_(
        &(fact as c_char),
        &(trans as c_char),
        &n,
        &nrhs,
        a.as_mut_ptr(),
        &lda,
        af.as_mut_ptr(),
        &ldaf,
        ipiv.as_mut_ptr(),
        equed as *mut _ as *mut _,
        r.as_mut_ptr(),
        c.as_mut_ptr(),
        b.as_mut_ptr(),
        &ldb,
        x.as_mut_ptr(),
        &ldx,
        rcond,
        rpvgrw,
        berr.as_mut_ptr(),
        &n_err_bnds,
        err_bnds_norm.as_mut_ptr(),
        err_bnds_comp.as_mut_ptr(),
        nparams.as_ptr(),
        params.as_mut_ptr(),
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn zgesvxx(
    fact: u8,
    trans: u8,
    n: i32,
    nrhs: i32,
    a: &mut [c64],
    lda: i32,
    af: &mut [c64],
    ldaf: i32,
    ipiv: &mut [i32],
    equed: &mut u8,
    r: &mut [f64],
    c: &mut [f64],
    b: &mut [c64],
    ldb: i32,
    x: &mut [c64],
    ldx: i32,
    rcond: &mut f64,
    rpvgrw: &mut f64,
    berr: &mut [f64],
    n_err_bnds: i32,
    err_bnds_norm: &mut [f64],
    err_bnds_comp: &mut [f64],
    nparams: &[i32],
    params: &mut [f64],
    work: &mut [c64],
    rwork: &mut [f64],
    info: &mut i32,
) {
    ffi::zgesvxx_(
        &(fact as c_char),
        &(trans as c_char),
        &n,
        &nrhs,
        a.as_mut_ptr() as *mut _,
        &lda,
        af.as_mut_ptr() as *mut _,
        &ldaf,
        ipiv.as_mut_ptr(),
        equed as *mut _ as *mut _,
        r.as_mut_ptr(),
        c.as_mut_ptr(),
        b.as_mut_ptr() as *mut _,
        &ldb,
        x.as_mut_ptr() as *mut _,
        &ldx,
        rcond,
        rpvgrw,
        berr.as_mut_ptr(),
        &n_err_bnds,
        err_bnds_norm.as_mut_ptr(),
        err_bnds_comp.as_mut_ptr(),
        nparams.as_ptr(),
        params.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn cgetf2(m: i32, n: i32, a: &mut [c32], lda: i32, ipiv: &mut [i32], info: &mut i32) {
    ffi::cgetf2_(
        &m,
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        ipiv.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn dgetf2(m: i32, n: i32, a: &mut [f64], lda: i32, ipiv: &mut [i32], info: &mut i32) {
    ffi::dgetf2_(&m, &n, a.as_mut_ptr(), &lda, ipiv.as_mut_ptr(), info)
}

#[inline]
pub unsafe fn sgetf2(m: i32, n: i32, a: &mut [f32], lda: i32, ipiv: &mut [i32], info: &mut i32) {
    ffi::sgetf2_(&m, &n, a.as_mut_ptr(), &lda, ipiv.as_mut_ptr(), info)
}

#[inline]
pub unsafe fn zgetf2(m: i32, n: i32, a: &mut [c64], lda: i32, ipiv: &mut [i32], info: &mut i32) {
    ffi::zgetf2_(
        &m,
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        ipiv.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn cgetrf(m: i32, n: i32, a: &mut [c32], lda: i32, ipiv: &mut [i32], info: &mut i32) {
    ffi::cgetrf_(
        &m,
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        ipiv.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn dgetrf(m: i32, n: i32, a: &mut [f64], lda: i32, ipiv: &mut [i32], info: &mut i32) {
    ffi::dgetrf_(&m, &n, a.as_mut_ptr(), &lda, ipiv.as_mut_ptr(), info)
}

#[inline]
pub unsafe fn sgetrf(m: i32, n: i32, a: &mut [f32], lda: i32, ipiv: &mut [i32], info: &mut i32) {
    ffi::sgetrf_(&m, &n, a.as_mut_ptr(), &lda, ipiv.as_mut_ptr(), info)
}

#[inline]
pub unsafe fn zgetrf(m: i32, n: i32, a: &mut [c64], lda: i32, ipiv: &mut [i32], info: &mut i32) {
    ffi::zgetrf_(
        &m,
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        ipiv.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn cgetrf2(m: i32, n: i32, a: &mut [c32], lda: i32, ipiv: &mut [i32], info: &mut i32) {
    ffi::cgetrf2_(
        &m,
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        ipiv.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn dgetrf2(m: i32, n: i32, a: &mut [f64], lda: i32, ipiv: &mut [i32], info: &mut i32) {
    ffi::dgetrf2_(&m, &n, a.as_mut_ptr(), &lda, ipiv.as_mut_ptr(), info)
}

#[inline]
pub unsafe fn sgetrf2(m: i32, n: i32, a: &mut [f32], lda: i32, ipiv: &mut [i32], info: &mut i32) {
    ffi::sgetrf2_(&m, &n, a.as_mut_ptr(), &lda, ipiv.as_mut_ptr(), info)
}

#[inline]
pub unsafe fn zgetrf2(m: i32, n: i32, a: &mut [c64], lda: i32, ipiv: &mut [i32], info: &mut i32) {
    ffi::zgetrf2_(
        &m,
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        ipiv.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn cgetri(
    n: i32,
    a: &mut [c32],
    lda: i32,
    ipiv: &[i32],
    work: &mut [c32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::cgetri_(
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        ipiv.as_ptr(),
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn dgetri(
    n: i32,
    a: &mut [f64],
    lda: i32,
    ipiv: &[i32],
    work: &mut [f64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::dgetri_(
        &n,
        a.as_mut_ptr(),
        &lda,
        ipiv.as_ptr(),
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn sgetri(
    n: i32,
    a: &mut [f32],
    lda: i32,
    ipiv: &[i32],
    work: &mut [f32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::sgetri_(
        &n,
        a.as_mut_ptr(),
        &lda,
        ipiv.as_ptr(),
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn zgetri(
    n: i32,
    a: &mut [c64],
    lda: i32,
    ipiv: &[i32],
    work: &mut [c64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::zgetri_(
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        ipiv.as_ptr(),
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn cgetrs(
    trans: u8,
    n: i32,
    nrhs: i32,
    a: &[c32],
    lda: i32,
    ipiv: &[i32],
    b: &mut [c32],
    ldb: i32,
    info: &mut i32,
) {
    ffi::cgetrs_(
        &(trans as c_char),
        &n,
        &nrhs,
        a.as_ptr() as *const _,
        &lda,
        ipiv.as_ptr(),
        b.as_mut_ptr() as *mut _,
        &ldb,
        info,
    )
}

#[inline]
pub unsafe fn dgetrs(
    trans: u8,
    n: i32,
    nrhs: i32,
    a: &[f64],
    lda: i32,
    ipiv: &[i32],
    b: &mut [f64],
    ldb: i32,
    info: &mut i32,
) {
    ffi::dgetrs_(
        &(trans as c_char),
        &n,
        &nrhs,
        a.as_ptr(),
        &lda,
        ipiv.as_ptr(),
        b.as_mut_ptr(),
        &ldb,
        info,
    )
}

#[inline]
pub unsafe fn sgetrs(
    trans: u8,
    n: i32,
    nrhs: i32,
    a: &[f32],
    lda: i32,
    ipiv: &[i32],
    b: &mut [f32],
    ldb: i32,
    info: &mut i32,
) {
    ffi::sgetrs_(
        &(trans as c_char),
        &n,
        &nrhs,
        a.as_ptr(),
        &lda,
        ipiv.as_ptr(),
        b.as_mut_ptr(),
        &ldb,
        info,
    )
}

#[inline]
pub unsafe fn zgetrs(
    trans: u8,
    n: i32,
    nrhs: i32,
    a: &[c64],
    lda: i32,
    ipiv: &[i32],
    b: &mut [c64],
    ldb: i32,
    info: &mut i32,
) {
    ffi::zgetrs_(
        &(trans as c_char),
        &n,
        &nrhs,
        a.as_ptr() as *const _,
        &lda,
        ipiv.as_ptr(),
        b.as_mut_ptr() as *mut _,
        &ldb,
        info,
    )
}

#[inline]
pub unsafe fn cgetsls(
    trans: u8,
    m: i32,
    n: i32,
    nrhs: i32,
    a: &mut [c32],
    lda: i32,
    b: &mut [c32],
    ldb: i32,
    work: &mut [c32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::cgetsls_(
        &(trans as c_char),
        &m,
        &n,
        &nrhs,
        a.as_mut_ptr() as *mut _,
        &lda,
        b.as_mut_ptr() as *mut _,
        &ldb,
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn dgetsls(
    trans: u8,
    m: i32,
    n: i32,
    nrhs: i32,
    a: &mut [f64],
    lda: i32,
    b: &mut [f64],
    ldb: i32,
    work: &mut [f64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::dgetsls_(
        &(trans as c_char),
        &m,
        &n,
        &nrhs,
        a.as_mut_ptr(),
        &lda,
        b.as_mut_ptr(),
        &ldb,
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn sgetsls(
    trans: u8,
    m: i32,
    n: i32,
    nrhs: i32,
    a: &mut [f32],
    lda: i32,
    b: &mut [f32],
    ldb: i32,
    work: &mut [f32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::sgetsls_(
        &(trans as c_char),
        &m,
        &n,
        &nrhs,
        a.as_mut_ptr(),
        &lda,
        b.as_mut_ptr(),
        &ldb,
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn zgetsls(
    trans: u8,
    m: i32,
    n: i32,
    nrhs: i32,
    a: &mut [c64],
    lda: i32,
    b: &mut [c64],
    ldb: i32,
    work: &mut [c64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::zgetsls_(
        &(trans as c_char),
        &m,
        &n,
        &nrhs,
        a.as_mut_ptr() as *mut _,
        &lda,
        b.as_mut_ptr() as *mut _,
        &ldb,
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn cgetsqrhrt(
    m: i32,
    n: i32,
    mb1: &[i32],
    nb1: &[i32],
    nb2: &[i32],
    a: &mut [c32],
    lda: i32,
    t: &mut [c32],
    ldt: i32,
    work: &mut [c32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::cgetsqrhrt_(
        &m,
        &n,
        mb1.as_ptr(),
        nb1.as_ptr(),
        nb2.as_ptr(),
        a.as_mut_ptr() as *mut _,
        &lda,
        t.as_mut_ptr() as *mut _,
        &ldt,
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn dgetsqrhrt(
    m: i32,
    n: i32,
    mb1: &[i32],
    nb1: &[i32],
    nb2: &[i32],
    a: &mut [f64],
    lda: i32,
    t: &mut [f64],
    ldt: i32,
    work: &mut [f64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::dgetsqrhrt_(
        &m,
        &n,
        mb1.as_ptr(),
        nb1.as_ptr(),
        nb2.as_ptr(),
        a.as_mut_ptr(),
        &lda,
        t.as_mut_ptr(),
        &ldt,
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn sgetsqrhrt(
    m: i32,
    n: i32,
    mb1: &[i32],
    nb1: &[i32],
    nb2: &[i32],
    a: &mut [f32],
    lda: i32,
    t: &mut [f32],
    ldt: i32,
    work: &mut [f32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::sgetsqrhrt_(
        &m,
        &n,
        mb1.as_ptr(),
        nb1.as_ptr(),
        nb2.as_ptr(),
        a.as_mut_ptr(),
        &lda,
        t.as_mut_ptr(),
        &ldt,
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn zgetsqrhrt(
    m: i32,
    n: i32,
    mb1: &[i32],
    nb1: &[i32],
    nb2: &[i32],
    a: &mut [c64],
    lda: i32,
    t: &mut [c64],
    ldt: i32,
    work: &mut [c64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::zgetsqrhrt_(
        &m,
        &n,
        mb1.as_ptr(),
        nb1.as_ptr(),
        nb2.as_ptr(),
        a.as_mut_ptr() as *mut _,
        &lda,
        t.as_mut_ptr() as *mut _,
        &ldt,
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn cggbak(
    job: u8,
    side: u8,
    n: i32,
    ilo: i32,
    ihi: i32,
    lscale: &[f32],
    rscale: &[f32],
    m: i32,
    v: &mut [c32],
    ldv: i32,
    info: &mut i32,
) {
    ffi::cggbak_(
        &(job as c_char),
        &(side as c_char),
        &n,
        &ilo,
        &ihi,
        lscale.as_ptr(),
        rscale.as_ptr(),
        &m,
        v.as_mut_ptr() as *mut _,
        &ldv,
        info,
    )
}

#[inline]
pub unsafe fn dggbak(
    job: u8,
    side: u8,
    n: i32,
    ilo: i32,
    ihi: i32,
    lscale: &[f64],
    rscale: &[f64],
    m: i32,
    v: &mut [f64],
    ldv: i32,
    info: &mut i32,
) {
    ffi::dggbak_(
        &(job as c_char),
        &(side as c_char),
        &n,
        &ilo,
        &ihi,
        lscale.as_ptr(),
        rscale.as_ptr(),
        &m,
        v.as_mut_ptr(),
        &ldv,
        info,
    )
}

#[inline]
pub unsafe fn sggbak(
    job: u8,
    side: u8,
    n: i32,
    ilo: i32,
    ihi: i32,
    lscale: &[f32],
    rscale: &[f32],
    m: i32,
    v: &mut [f32],
    ldv: i32,
    info: &mut i32,
) {
    ffi::sggbak_(
        &(job as c_char),
        &(side as c_char),
        &n,
        &ilo,
        &ihi,
        lscale.as_ptr(),
        rscale.as_ptr(),
        &m,
        v.as_mut_ptr(),
        &ldv,
        info,
    )
}

#[inline]
pub unsafe fn zggbak(
    job: u8,
    side: u8,
    n: i32,
    ilo: i32,
    ihi: i32,
    lscale: &[f64],
    rscale: &[f64],
    m: i32,
    v: &mut [c64],
    ldv: i32,
    info: &mut i32,
) {
    ffi::zggbak_(
        &(job as c_char),
        &(side as c_char),
        &n,
        &ilo,
        &ihi,
        lscale.as_ptr(),
        rscale.as_ptr(),
        &m,
        v.as_mut_ptr() as *mut _,
        &ldv,
        info,
    )
}

#[inline]
pub unsafe fn cggbal(
    job: u8,
    n: i32,
    a: &mut [c32],
    lda: i32,
    b: &mut [c32],
    ldb: i32,
    ilo: &mut i32,
    ihi: &mut i32,
    lscale: &mut [f32],
    rscale: &mut [f32],
    work: &mut [f32],
    info: &mut i32,
) {
    ffi::cggbal_(
        &(job as c_char),
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        b.as_mut_ptr() as *mut _,
        &ldb,
        ilo,
        ihi,
        lscale.as_mut_ptr(),
        rscale.as_mut_ptr(),
        work.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn dggbal(
    job: u8,
    n: i32,
    a: &mut [f64],
    lda: i32,
    b: &mut [f64],
    ldb: i32,
    ilo: &mut i32,
    ihi: &mut i32,
    lscale: &mut [f64],
    rscale: &mut [f64],
    work: &mut [f64],
    info: &mut i32,
) {
    ffi::dggbal_(
        &(job as c_char),
        &n,
        a.as_mut_ptr(),
        &lda,
        b.as_mut_ptr(),
        &ldb,
        ilo,
        ihi,
        lscale.as_mut_ptr(),
        rscale.as_mut_ptr(),
        work.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn sggbal(
    job: u8,
    n: i32,
    a: &mut [f32],
    lda: i32,
    b: &mut [f32],
    ldb: i32,
    ilo: &mut i32,
    ihi: &mut i32,
    lscale: &mut [f32],
    rscale: &mut [f32],
    work: &mut [f32],
    info: &mut i32,
) {
    ffi::sggbal_(
        &(job as c_char),
        &n,
        a.as_mut_ptr(),
        &lda,
        b.as_mut_ptr(),
        &ldb,
        ilo,
        ihi,
        lscale.as_mut_ptr(),
        rscale.as_mut_ptr(),
        work.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn zggbal(
    job: u8,
    n: i32,
    a: &mut [c64],
    lda: i32,
    b: &mut [c64],
    ldb: i32,
    ilo: &mut i32,
    ihi: &mut i32,
    lscale: &mut [f64],
    rscale: &mut [f64],
    work: &mut [f64],
    info: &mut i32,
) {
    ffi::zggbal_(
        &(job as c_char),
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        b.as_mut_ptr() as *mut _,
        &ldb,
        ilo,
        ihi,
        lscale.as_mut_ptr(),
        rscale.as_mut_ptr(),
        work.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn cgges(
    jobvsl: u8,
    jobvsr: u8,
    sort: u8,
    selctg: Select2C32,
    n: i32,
    a: &mut [c32],
    lda: i32,
    b: &mut [c32],
    ldb: i32,
    sdim: &mut i32,
    alpha: &mut [c32],
    beta: &mut [c32],
    vsl: &mut [c32],
    ldvsl: i32,
    vsr: &mut [c32],
    ldvsr: i32,
    work: &mut [c32],
    lwork: i32,
    rwork: &mut [f32],
    bwork: &mut [i32],
    info: &mut i32,
) {
    ffi::cgges_(
        &(jobvsl as c_char),
        &(jobvsr as c_char),
        &(sort as c_char),
        transmute(selctg),
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        b.as_mut_ptr() as *mut _,
        &ldb,
        sdim,
        alpha.as_mut_ptr() as *mut _,
        beta.as_mut_ptr() as *mut _,
        vsl.as_mut_ptr() as *mut _,
        &ldvsl,
        vsr.as_mut_ptr() as *mut _,
        &ldvsr,
        work.as_mut_ptr() as *mut _,
        &lwork,
        rwork.as_mut_ptr(),
        bwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn dgges(
    jobvsl: u8,
    jobvsr: u8,
    sort: u8,
    selctg: Select3F64,
    n: i32,
    a: &mut [f64],
    lda: i32,
    b: &mut [f64],
    ldb: i32,
    sdim: &mut i32,
    alphar: &mut [f64],
    alphai: &mut [f64],
    beta: &mut [f64],
    vsl: &mut [f64],
    ldvsl: i32,
    vsr: &mut [f64],
    ldvsr: i32,
    work: &mut [f64],
    lwork: i32,
    bwork: &mut [i32],
    info: &mut i32,
) {
    ffi::dgges_(
        &(jobvsl as c_char),
        &(jobvsr as c_char),
        &(sort as c_char),
        transmute(selctg),
        &n,
        a.as_mut_ptr(),
        &lda,
        b.as_mut_ptr(),
        &ldb,
        sdim,
        alphar.as_mut_ptr(),
        alphai.as_mut_ptr(),
        beta.as_mut_ptr(),
        vsl.as_mut_ptr(),
        &ldvsl,
        vsr.as_mut_ptr(),
        &ldvsr,
        work.as_mut_ptr(),
        &lwork,
        bwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn sgges(
    jobvsl: u8,
    jobvsr: u8,
    sort: u8,
    selctg: Select3F32,
    n: i32,
    a: &mut [f32],
    lda: i32,
    b: &mut [f32],
    ldb: i32,
    sdim: &mut i32,
    alphar: &mut [f32],
    alphai: &mut [f32],
    beta: &mut [f32],
    vsl: &mut [f32],
    ldvsl: i32,
    vsr: &mut [f32],
    ldvsr: i32,
    work: &mut [f32],
    lwork: i32,
    bwork: &mut [i32],
    info: &mut i32,
) {
    ffi::sgges_(
        &(jobvsl as c_char),
        &(jobvsr as c_char),
        &(sort as c_char),
        transmute(selctg),
        &n,
        a.as_mut_ptr(),
        &lda,
        b.as_mut_ptr(),
        &ldb,
        sdim,
        alphar.as_mut_ptr(),
        alphai.as_mut_ptr(),
        beta.as_mut_ptr(),
        vsl.as_mut_ptr(),
        &ldvsl,
        vsr.as_mut_ptr(),
        &ldvsr,
        work.as_mut_ptr(),
        &lwork,
        bwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn zgges(
    jobvsl: u8,
    jobvsr: u8,
    sort: u8,
    selctg: Select2C64,
    n: i32,
    a: &mut [c64],
    lda: i32,
    b: &mut [c64],
    ldb: i32,
    sdim: &mut i32,
    alpha: &mut [c64],
    beta: &mut [c64],
    vsl: &mut [c64],
    ldvsl: i32,
    vsr: &mut [c64],
    ldvsr: i32,
    work: &mut [c64],
    lwork: i32,
    rwork: &mut [f64],
    bwork: &mut [i32],
    info: &mut i32,
) {
    ffi::zgges_(
        &(jobvsl as c_char),
        &(jobvsr as c_char),
        &(sort as c_char),
        transmute(selctg),
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        b.as_mut_ptr() as *mut _,
        &ldb,
        sdim,
        alpha.as_mut_ptr() as *mut _,
        beta.as_mut_ptr() as *mut _,
        vsl.as_mut_ptr() as *mut _,
        &ldvsl,
        vsr.as_mut_ptr() as *mut _,
        &ldvsr,
        work.as_mut_ptr() as *mut _,
        &lwork,
        rwork.as_mut_ptr(),
        bwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn cgges3(
    jobvsl: u8,
    jobvsr: u8,
    sort: u8,
    selctg: Select2C32,
    n: i32,
    a: &mut [c32],
    lda: i32,
    b: &mut [c32],
    ldb: i32,
    sdim: &mut i32,
    alpha: &mut [c32],
    beta: &mut [c32],
    vsl: &mut [c32],
    ldvsl: i32,
    vsr: &mut [c32],
    ldvsr: i32,
    work: &mut [c32],
    lwork: i32,
    rwork: &mut [f32],
    bwork: &mut [i32],
    info: &mut i32,
) {
    ffi::cgges3_(
        &(jobvsl as c_char),
        &(jobvsr as c_char),
        &(sort as c_char),
        transmute(selctg),
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        b.as_mut_ptr() as *mut _,
        &ldb,
        sdim,
        alpha.as_mut_ptr() as *mut _,
        beta.as_mut_ptr() as *mut _,
        vsl.as_mut_ptr() as *mut _,
        &ldvsl,
        vsr.as_mut_ptr() as *mut _,
        &ldvsr,
        work.as_mut_ptr() as *mut _,
        &lwork,
        rwork.as_mut_ptr(),
        bwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn dgges3(
    jobvsl: u8,
    jobvsr: u8,
    sort: u8,
    selctg: Select3F64,
    n: i32,
    a: &mut [f64],
    lda: i32,
    b: &mut [f64],
    ldb: i32,
    sdim: &mut i32,
    alphar: &mut [f64],
    alphai: &mut [f64],
    beta: &mut [f64],
    vsl: &mut [f64],
    ldvsl: i32,
    vsr: &mut [f64],
    ldvsr: i32,
    work: &mut [f64],
    lwork: i32,
    bwork: &mut [i32],
    info: &mut i32,
) {
    ffi::dgges3_(
        &(jobvsl as c_char),
        &(jobvsr as c_char),
        &(sort as c_char),
        transmute(selctg),
        &n,
        a.as_mut_ptr(),
        &lda,
        b.as_mut_ptr(),
        &ldb,
        sdim,
        alphar.as_mut_ptr(),
        alphai.as_mut_ptr(),
        beta.as_mut_ptr(),
        vsl.as_mut_ptr(),
        &ldvsl,
        vsr.as_mut_ptr(),
        &ldvsr,
        work.as_mut_ptr(),
        &lwork,
        bwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn sgges3(
    jobvsl: u8,
    jobvsr: u8,
    sort: u8,
    selctg: Select3F32,
    n: i32,
    a: &mut [f32],
    lda: i32,
    b: &mut [f32],
    ldb: i32,
    sdim: &mut i32,
    alphar: &mut [f32],
    alphai: &mut [f32],
    beta: &mut [f32],
    vsl: &mut [f32],
    ldvsl: i32,
    vsr: &mut [f32],
    ldvsr: i32,
    work: &mut [f32],
    lwork: i32,
    bwork: &mut [i32],
    info: &mut i32,
) {
    ffi::sgges3_(
        &(jobvsl as c_char),
        &(jobvsr as c_char),
        &(sort as c_char),
        transmute(selctg),
        &n,
        a.as_mut_ptr(),
        &lda,
        b.as_mut_ptr(),
        &ldb,
        sdim,
        alphar.as_mut_ptr(),
        alphai.as_mut_ptr(),
        beta.as_mut_ptr(),
        vsl.as_mut_ptr(),
        &ldvsl,
        vsr.as_mut_ptr(),
        &ldvsr,
        work.as_mut_ptr(),
        &lwork,
        bwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn zgges3(
    jobvsl: u8,
    jobvsr: u8,
    sort: u8,
    selctg: Select2C64,
    n: i32,
    a: &mut [c64],
    lda: i32,
    b: &mut [c64],
    ldb: i32,
    sdim: &mut i32,
    alpha: &mut [c64],
    beta: &mut [c64],
    vsl: &mut [c64],
    ldvsl: i32,
    vsr: &mut [c64],
    ldvsr: i32,
    work: &mut [c64],
    lwork: i32,
    rwork: &mut [f64],
    bwork: &mut [i32],
    info: &mut i32,
) {
    ffi::zgges3_(
        &(jobvsl as c_char),
        &(jobvsr as c_char),
        &(sort as c_char),
        transmute(selctg),
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        b.as_mut_ptr() as *mut _,
        &ldb,
        sdim,
        alpha.as_mut_ptr() as *mut _,
        beta.as_mut_ptr() as *mut _,
        vsl.as_mut_ptr() as *mut _,
        &ldvsl,
        vsr.as_mut_ptr() as *mut _,
        &ldvsr,
        work.as_mut_ptr() as *mut _,
        &lwork,
        rwork.as_mut_ptr(),
        bwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn cggesx(
    jobvsl: u8,
    jobvsr: u8,
    sort: u8,
    selctg: Select2C32,
    sense: u8,
    n: i32,
    a: &mut [c32],
    lda: i32,
    b: &mut [c32],
    ldb: i32,
    sdim: &mut i32,
    alpha: &mut [c32],
    beta: &mut [c32],
    vsl: &mut [c32],
    ldvsl: i32,
    vsr: &mut [c32],
    ldvsr: i32,
    rconde: &mut [f32],
    rcondv: &mut [f32],
    work: &mut [c32],
    lwork: i32,
    rwork: &mut [f32],
    iwork: &mut [i32],
    liwork: i32,
    bwork: &mut [i32],
    info: &mut i32,
) {
    ffi::cggesx_(
        &(jobvsl as c_char),
        &(jobvsr as c_char),
        &(sort as c_char),
        transmute(selctg),
        &(sense as c_char),
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        b.as_mut_ptr() as *mut _,
        &ldb,
        sdim,
        alpha.as_mut_ptr() as *mut _,
        beta.as_mut_ptr() as *mut _,
        vsl.as_mut_ptr() as *mut _,
        &ldvsl,
        vsr.as_mut_ptr() as *mut _,
        &ldvsr,
        rconde.as_mut_ptr(),
        rcondv.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        &lwork,
        rwork.as_mut_ptr(),
        iwork.as_mut_ptr(),
        &liwork,
        bwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn dggesx(
    jobvsl: u8,
    jobvsr: u8,
    sort: u8,
    selctg: Select3F64,
    sense: u8,
    n: i32,
    a: &mut [f64],
    lda: i32,
    b: &mut [f64],
    ldb: i32,
    sdim: &mut i32,
    alphar: &mut [f64],
    alphai: &mut [f64],
    beta: &mut [f64],
    vsl: &mut [f64],
    ldvsl: i32,
    vsr: &mut [f64],
    ldvsr: i32,
    rconde: &mut [f64],
    rcondv: &mut [f64],
    work: &mut [f64],
    lwork: i32,
    iwork: &mut [i32],
    liwork: i32,
    bwork: &mut [i32],
    info: &mut i32,
) {
    ffi::dggesx_(
        &(jobvsl as c_char),
        &(jobvsr as c_char),
        &(sort as c_char),
        transmute(selctg),
        &(sense as c_char),
        &n,
        a.as_mut_ptr(),
        &lda,
        b.as_mut_ptr(),
        &ldb,
        sdim,
        alphar.as_mut_ptr(),
        alphai.as_mut_ptr(),
        beta.as_mut_ptr(),
        vsl.as_mut_ptr(),
        &ldvsl,
        vsr.as_mut_ptr(),
        &ldvsr,
        rconde.as_mut_ptr(),
        rcondv.as_mut_ptr(),
        work.as_mut_ptr(),
        &lwork,
        iwork.as_mut_ptr(),
        &liwork,
        bwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn sggesx(
    jobvsl: u8,
    jobvsr: u8,
    sort: u8,
    selctg: Select3F32,
    sense: u8,
    n: i32,
    a: &mut [f32],
    lda: i32,
    b: &mut [f32],
    ldb: i32,
    sdim: &mut i32,
    alphar: &mut [f32],
    alphai: &mut [f32],
    beta: &mut [f32],
    vsl: &mut [f32],
    ldvsl: i32,
    vsr: &mut [f32],
    ldvsr: i32,
    rconde: &mut [f32],
    rcondv: &mut [f32],
    work: &mut [f32],
    lwork: i32,
    iwork: &mut [i32],
    liwork: i32,
    bwork: &mut [i32],
    info: &mut i32,
) {
    ffi::sggesx_(
        &(jobvsl as c_char),
        &(jobvsr as c_char),
        &(sort as c_char),
        transmute(selctg),
        &(sense as c_char),
        &n,
        a.as_mut_ptr(),
        &lda,
        b.as_mut_ptr(),
        &ldb,
        sdim,
        alphar.as_mut_ptr(),
        alphai.as_mut_ptr(),
        beta.as_mut_ptr(),
        vsl.as_mut_ptr(),
        &ldvsl,
        vsr.as_mut_ptr(),
        &ldvsr,
        rconde.as_mut_ptr(),
        rcondv.as_mut_ptr(),
        work.as_mut_ptr(),
        &lwork,
        iwork.as_mut_ptr(),
        &liwork,
        bwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn zggesx(
    jobvsl: u8,
    jobvsr: u8,
    sort: u8,
    selctg: Select2C64,
    sense: u8,
    n: i32,
    a: &mut [c64],
    lda: i32,
    b: &mut [c64],
    ldb: i32,
    sdim: &mut i32,
    alpha: &mut [c64],
    beta: &mut [c64],
    vsl: &mut [c64],
    ldvsl: i32,
    vsr: &mut [c64],
    ldvsr: i32,
    rconde: &mut [f64],
    rcondv: &mut [f64],
    work: &mut [c64],
    lwork: i32,
    rwork: &mut [f64],
    iwork: &mut [i32],
    liwork: i32,
    bwork: &mut [i32],
    info: &mut i32,
) {
    ffi::zggesx_(
        &(jobvsl as c_char),
        &(jobvsr as c_char),
        &(sort as c_char),
        transmute(selctg),
        &(sense as c_char),
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        b.as_mut_ptr() as *mut _,
        &ldb,
        sdim,
        alpha.as_mut_ptr() as *mut _,
        beta.as_mut_ptr() as *mut _,
        vsl.as_mut_ptr() as *mut _,
        &ldvsl,
        vsr.as_mut_ptr() as *mut _,
        &ldvsr,
        rconde.as_mut_ptr(),
        rcondv.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        &lwork,
        rwork.as_mut_ptr(),
        iwork.as_mut_ptr(),
        &liwork,
        bwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn cggev(
    jobvl: u8,
    jobvr: u8,
    n: i32,
    a: &mut [c32],
    lda: i32,
    b: &mut [c32],
    ldb: i32,
    alpha: &mut [c32],
    beta: &mut [c32],
    vl: &mut [c32],
    ldvl: i32,
    vr: &mut [c32],
    ldvr: i32,
    work: &mut [c32],
    lwork: i32,
    rwork: &mut [f32],
    info: &mut i32,
) {
    ffi::cggev_(
        &(jobvl as c_char),
        &(jobvr as c_char),
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        b.as_mut_ptr() as *mut _,
        &ldb,
        alpha.as_mut_ptr() as *mut _,
        beta.as_mut_ptr() as *mut _,
        vl.as_mut_ptr() as *mut _,
        &ldvl,
        vr.as_mut_ptr() as *mut _,
        &ldvr,
        work.as_mut_ptr() as *mut _,
        &lwork,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn dggev(
    jobvl: u8,
    jobvr: u8,
    n: i32,
    a: &mut [f64],
    lda: i32,
    b: &mut [f64],
    ldb: i32,
    alphar: &mut [f64],
    alphai: &mut [f64],
    beta: &mut [f64],
    vl: &mut [f64],
    ldvl: i32,
    vr: &mut [f64],
    ldvr: i32,
    work: &mut [f64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::dggev_(
        &(jobvl as c_char),
        &(jobvr as c_char),
        &n,
        a.as_mut_ptr(),
        &lda,
        b.as_mut_ptr(),
        &ldb,
        alphar.as_mut_ptr(),
        alphai.as_mut_ptr(),
        beta.as_mut_ptr(),
        vl.as_mut_ptr(),
        &ldvl,
        vr.as_mut_ptr(),
        &ldvr,
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn sggev(
    jobvl: u8,
    jobvr: u8,
    n: i32,
    a: &mut [f32],
    lda: i32,
    b: &mut [f32],
    ldb: i32,
    alphar: &mut [f32],
    alphai: &mut [f32],
    beta: &mut [f32],
    vl: &mut [f32],
    ldvl: i32,
    vr: &mut [f32],
    ldvr: i32,
    work: &mut [f32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::sggev_(
        &(jobvl as c_char),
        &(jobvr as c_char),
        &n,
        a.as_mut_ptr(),
        &lda,
        b.as_mut_ptr(),
        &ldb,
        alphar.as_mut_ptr(),
        alphai.as_mut_ptr(),
        beta.as_mut_ptr(),
        vl.as_mut_ptr(),
        &ldvl,
        vr.as_mut_ptr(),
        &ldvr,
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn zggev(
    jobvl: u8,
    jobvr: u8,
    n: i32,
    a: &mut [c64],
    lda: i32,
    b: &mut [c64],
    ldb: i32,
    alpha: &mut [c64],
    beta: &mut [c64],
    vl: &mut [c64],
    ldvl: i32,
    vr: &mut [c64],
    ldvr: i32,
    work: &mut [c64],
    lwork: i32,
    rwork: &mut [f64],
    info: &mut i32,
) {
    ffi::zggev_(
        &(jobvl as c_char),
        &(jobvr as c_char),
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        b.as_mut_ptr() as *mut _,
        &ldb,
        alpha.as_mut_ptr() as *mut _,
        beta.as_mut_ptr() as *mut _,
        vl.as_mut_ptr() as *mut _,
        &ldvl,
        vr.as_mut_ptr() as *mut _,
        &ldvr,
        work.as_mut_ptr() as *mut _,
        &lwork,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn cggev3(
    jobvl: u8,
    jobvr: u8,
    n: i32,
    a: &mut [c32],
    lda: i32,
    b: &mut [c32],
    ldb: i32,
    alpha: &mut [c32],
    beta: &mut [c32],
    vl: &mut [c32],
    ldvl: i32,
    vr: &mut [c32],
    ldvr: i32,
    work: &mut [c32],
    lwork: i32,
    rwork: &mut [f32],
    info: &mut i32,
) {
    ffi::cggev3_(
        &(jobvl as c_char),
        &(jobvr as c_char),
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        b.as_mut_ptr() as *mut _,
        &ldb,
        alpha.as_mut_ptr() as *mut _,
        beta.as_mut_ptr() as *mut _,
        vl.as_mut_ptr() as *mut _,
        &ldvl,
        vr.as_mut_ptr() as *mut _,
        &ldvr,
        work.as_mut_ptr() as *mut _,
        &lwork,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn dggev3(
    jobvl: u8,
    jobvr: u8,
    n: i32,
    a: &mut [f64],
    lda: i32,
    b: &mut [f64],
    ldb: i32,
    alphar: &mut [f64],
    alphai: &mut [f64],
    beta: &mut [f64],
    vl: &mut [f64],
    ldvl: i32,
    vr: &mut [f64],
    ldvr: i32,
    work: &mut [f64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::dggev3_(
        &(jobvl as c_char),
        &(jobvr as c_char),
        &n,
        a.as_mut_ptr(),
        &lda,
        b.as_mut_ptr(),
        &ldb,
        alphar.as_mut_ptr(),
        alphai.as_mut_ptr(),
        beta.as_mut_ptr(),
        vl.as_mut_ptr(),
        &ldvl,
        vr.as_mut_ptr(),
        &ldvr,
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn sggev3(
    jobvl: u8,
    jobvr: u8,
    n: i32,
    a: &mut [f32],
    lda: i32,
    b: &mut [f32],
    ldb: i32,
    alphar: &mut [f32],
    alphai: &mut [f32],
    beta: &mut [f32],
    vl: &mut [f32],
    ldvl: i32,
    vr: &mut [f32],
    ldvr: i32,
    work: &mut [f32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::sggev3_(
        &(jobvl as c_char),
        &(jobvr as c_char),
        &n,
        a.as_mut_ptr(),
        &lda,
        b.as_mut_ptr(),
        &ldb,
        alphar.as_mut_ptr(),
        alphai.as_mut_ptr(),
        beta.as_mut_ptr(),
        vl.as_mut_ptr(),
        &ldvl,
        vr.as_mut_ptr(),
        &ldvr,
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn zggev3(
    jobvl: u8,
    jobvr: u8,
    n: i32,
    a: &mut [c64],
    lda: i32,
    b: &mut [c64],
    ldb: i32,
    alpha: &mut [c64],
    beta: &mut [c64],
    vl: &mut [c64],
    ldvl: i32,
    vr: &mut [c64],
    ldvr: i32,
    work: &mut [c64],
    lwork: i32,
    rwork: &mut [f64],
    info: &mut i32,
) {
    ffi::zggev3_(
        &(jobvl as c_char),
        &(jobvr as c_char),
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        b.as_mut_ptr() as *mut _,
        &ldb,
        alpha.as_mut_ptr() as *mut _,
        beta.as_mut_ptr() as *mut _,
        vl.as_mut_ptr() as *mut _,
        &ldvl,
        vr.as_mut_ptr() as *mut _,
        &ldvr,
        work.as_mut_ptr() as *mut _,
        &lwork,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn cggevx(
    balanc: u8,
    jobvl: u8,
    jobvr: u8,
    sense: u8,
    n: i32,
    a: &mut [c32],
    lda: i32,
    b: &mut [c32],
    ldb: i32,
    alpha: &mut [c32],
    beta: &mut [c32],
    vl: &mut [c32],
    ldvl: i32,
    vr: &mut [c32],
    ldvr: i32,
    ilo: &mut i32,
    ihi: &mut i32,
    lscale: &mut [f32],
    rscale: &mut [f32],
    abnrm: &mut f32,
    bbnrm: &mut f32,
    rconde: &mut [f32],
    rcondv: &mut [f32],
    work: &mut [c32],
    lwork: i32,
    rwork: &mut [f32],
    iwork: &mut [i32],
    bwork: &mut [i32],
    info: &mut i32,
) {
    ffi::cggevx_(
        &(balanc as c_char),
        &(jobvl as c_char),
        &(jobvr as c_char),
        &(sense as c_char),
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        b.as_mut_ptr() as *mut _,
        &ldb,
        alpha.as_mut_ptr() as *mut _,
        beta.as_mut_ptr() as *mut _,
        vl.as_mut_ptr() as *mut _,
        &ldvl,
        vr.as_mut_ptr() as *mut _,
        &ldvr,
        ilo,
        ihi,
        lscale.as_mut_ptr(),
        rscale.as_mut_ptr(),
        abnrm,
        bbnrm,
        rconde.as_mut_ptr(),
        rcondv.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        &lwork,
        rwork.as_mut_ptr(),
        iwork.as_mut_ptr(),
        bwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn dggevx(
    balanc: u8,
    jobvl: u8,
    jobvr: u8,
    sense: u8,
    n: i32,
    a: &mut [f64],
    lda: i32,
    b: &mut [f64],
    ldb: i32,
    alphar: &mut [f64],
    alphai: &mut [f64],
    beta: &mut [f64],
    vl: &mut [f64],
    ldvl: i32,
    vr: &mut [f64],
    ldvr: i32,
    ilo: &mut i32,
    ihi: &mut i32,
    lscale: &mut [f64],
    rscale: &mut [f64],
    abnrm: &mut f64,
    bbnrm: &mut f64,
    rconde: &mut [f64],
    rcondv: &mut [f64],
    work: &mut [f64],
    lwork: i32,
    iwork: &mut [i32],
    bwork: &mut [i32],
    info: &mut i32,
) {
    ffi::dggevx_(
        &(balanc as c_char),
        &(jobvl as c_char),
        &(jobvr as c_char),
        &(sense as c_char),
        &n,
        a.as_mut_ptr(),
        &lda,
        b.as_mut_ptr(),
        &ldb,
        alphar.as_mut_ptr(),
        alphai.as_mut_ptr(),
        beta.as_mut_ptr(),
        vl.as_mut_ptr(),
        &ldvl,
        vr.as_mut_ptr(),
        &ldvr,
        ilo,
        ihi,
        lscale.as_mut_ptr(),
        rscale.as_mut_ptr(),
        abnrm,
        bbnrm,
        rconde.as_mut_ptr(),
        rcondv.as_mut_ptr(),
        work.as_mut_ptr(),
        &lwork,
        iwork.as_mut_ptr(),
        bwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn sggevx(
    balanc: u8,
    jobvl: u8,
    jobvr: u8,
    sense: u8,
    n: i32,
    a: &mut [f32],
    lda: i32,
    b: &mut [f32],
    ldb: i32,
    alphar: &mut [f32],
    alphai: &mut [f32],
    beta: &mut [f32],
    vl: &mut [f32],
    ldvl: i32,
    vr: &mut [f32],
    ldvr: i32,
    ilo: &mut i32,
    ihi: &mut i32,
    lscale: &mut [f32],
    rscale: &mut [f32],
    abnrm: &mut f32,
    bbnrm: &mut f32,
    rconde: &mut [f32],
    rcondv: &mut [f32],
    work: &mut [f32],
    lwork: i32,
    iwork: &mut [i32],
    bwork: &mut [i32],
    info: &mut i32,
) {
    ffi::sggevx_(
        &(balanc as c_char),
        &(jobvl as c_char),
        &(jobvr as c_char),
        &(sense as c_char),
        &n,
        a.as_mut_ptr(),
        &lda,
        b.as_mut_ptr(),
        &ldb,
        alphar.as_mut_ptr(),
        alphai.as_mut_ptr(),
        beta.as_mut_ptr(),
        vl.as_mut_ptr(),
        &ldvl,
        vr.as_mut_ptr(),
        &ldvr,
        ilo,
        ihi,
        lscale.as_mut_ptr(),
        rscale.as_mut_ptr(),
        abnrm,
        bbnrm,
        rconde.as_mut_ptr(),
        rcondv.as_mut_ptr(),
        work.as_mut_ptr(),
        &lwork,
        iwork.as_mut_ptr(),
        bwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn zggevx(
    balanc: u8,
    jobvl: u8,
    jobvr: u8,
    sense: u8,
    n: i32,
    a: &mut [c64],
    lda: i32,
    b: &mut [c64],
    ldb: i32,
    alpha: &mut [c64],
    beta: &mut [c64],
    vl: &mut [c64],
    ldvl: i32,
    vr: &mut [c64],
    ldvr: i32,
    ilo: &mut i32,
    ihi: &mut i32,
    lscale: &mut [f64],
    rscale: &mut [f64],
    abnrm: &mut f64,
    bbnrm: &mut f64,
    rconde: &mut [f64],
    rcondv: &mut [f64],
    work: &mut [c64],
    lwork: i32,
    rwork: &mut [f64],
    iwork: &mut [i32],
    bwork: &mut [i32],
    info: &mut i32,
) {
    ffi::zggevx_(
        &(balanc as c_char),
        &(jobvl as c_char),
        &(jobvr as c_char),
        &(sense as c_char),
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        b.as_mut_ptr() as *mut _,
        &ldb,
        alpha.as_mut_ptr() as *mut _,
        beta.as_mut_ptr() as *mut _,
        vl.as_mut_ptr() as *mut _,
        &ldvl,
        vr.as_mut_ptr() as *mut _,
        &ldvr,
        ilo,
        ihi,
        lscale.as_mut_ptr(),
        rscale.as_mut_ptr(),
        abnrm,
        bbnrm,
        rconde.as_mut_ptr(),
        rcondv.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        &lwork,
        rwork.as_mut_ptr(),
        iwork.as_mut_ptr(),
        bwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn cggglm(
    n: i32,
    m: i32,
    p: i32,
    a: &mut [c32],
    lda: i32,
    b: &mut [c32],
    ldb: i32,
    d: &mut [c32],
    x: &mut [c32],
    y: &mut [c32],
    work: &mut [c32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::cggglm_(
        &n,
        &m,
        &p,
        a.as_mut_ptr() as *mut _,
        &lda,
        b.as_mut_ptr() as *mut _,
        &ldb,
        d.as_mut_ptr() as *mut _,
        x.as_mut_ptr() as *mut _,
        y.as_mut_ptr() as *mut _,
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn dggglm(
    n: i32,
    m: i32,
    p: i32,
    a: &mut [f64],
    lda: i32,
    b: &mut [f64],
    ldb: i32,
    d: &mut [f64],
    x: &mut [f64],
    y: &mut [f64],
    work: &mut [f64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::dggglm_(
        &n,
        &m,
        &p,
        a.as_mut_ptr(),
        &lda,
        b.as_mut_ptr(),
        &ldb,
        d.as_mut_ptr(),
        x.as_mut_ptr(),
        y.as_mut_ptr(),
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn sggglm(
    n: i32,
    m: i32,
    p: i32,
    a: &mut [f32],
    lda: i32,
    b: &mut [f32],
    ldb: i32,
    d: &mut [f32],
    x: &mut [f32],
    y: &mut [f32],
    work: &mut [f32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::sggglm_(
        &n,
        &m,
        &p,
        a.as_mut_ptr(),
        &lda,
        b.as_mut_ptr(),
        &ldb,
        d.as_mut_ptr(),
        x.as_mut_ptr(),
        y.as_mut_ptr(),
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn zggglm(
    n: i32,
    m: i32,
    p: i32,
    a: &mut [c64],
    lda: i32,
    b: &mut [c64],
    ldb: i32,
    d: &mut [c64],
    x: &mut [c64],
    y: &mut [c64],
    work: &mut [c64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::zggglm_(
        &n,
        &m,
        &p,
        a.as_mut_ptr() as *mut _,
        &lda,
        b.as_mut_ptr() as *mut _,
        &ldb,
        d.as_mut_ptr() as *mut _,
        x.as_mut_ptr() as *mut _,
        y.as_mut_ptr() as *mut _,
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn cgghd3(
    compq: u8,
    compz: u8,
    n: i32,
    ilo: i32,
    ihi: i32,
    a: &mut [c32],
    lda: i32,
    b: &mut [c32],
    ldb: i32,
    q: &mut [c32],
    ldq: i32,
    z: &mut [c32],
    ldz: i32,
    work: &mut [c32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::cgghd3_(
        &(compq as c_char),
        &(compz as c_char),
        &n,
        &ilo,
        &ihi,
        a.as_mut_ptr() as *mut _,
        &lda,
        b.as_mut_ptr() as *mut _,
        &ldb,
        q.as_mut_ptr() as *mut _,
        &ldq,
        z.as_mut_ptr() as *mut _,
        &ldz,
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn dgghd3(
    compq: u8,
    compz: u8,
    n: i32,
    ilo: i32,
    ihi: i32,
    a: &mut [f64],
    lda: i32,
    b: &mut [f64],
    ldb: i32,
    q: &mut [f64],
    ldq: i32,
    z: &mut [f64],
    ldz: i32,
    work: &mut [f64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::dgghd3_(
        &(compq as c_char),
        &(compz as c_char),
        &n,
        &ilo,
        &ihi,
        a.as_mut_ptr(),
        &lda,
        b.as_mut_ptr(),
        &ldb,
        q.as_mut_ptr(),
        &ldq,
        z.as_mut_ptr(),
        &ldz,
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn sgghd3(
    compq: u8,
    compz: u8,
    n: i32,
    ilo: i32,
    ihi: i32,
    a: &mut [f32],
    lda: i32,
    b: &mut [f32],
    ldb: i32,
    q: &mut [f32],
    ldq: i32,
    z: &mut [f32],
    ldz: i32,
    work: &mut [f32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::sgghd3_(
        &(compq as c_char),
        &(compz as c_char),
        &n,
        &ilo,
        &ihi,
        a.as_mut_ptr(),
        &lda,
        b.as_mut_ptr(),
        &ldb,
        q.as_mut_ptr(),
        &ldq,
        z.as_mut_ptr(),
        &ldz,
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn zgghd3(
    compq: u8,
    compz: u8,
    n: i32,
    ilo: i32,
    ihi: i32,
    a: &mut [c64],
    lda: i32,
    b: &mut [c64],
    ldb: i32,
    q: &mut [c64],
    ldq: i32,
    z: &mut [c64],
    ldz: i32,
    work: &mut [c64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::zgghd3_(
        &(compq as c_char),
        &(compz as c_char),
        &n,
        &ilo,
        &ihi,
        a.as_mut_ptr() as *mut _,
        &lda,
        b.as_mut_ptr() as *mut _,
        &ldb,
        q.as_mut_ptr() as *mut _,
        &ldq,
        z.as_mut_ptr() as *mut _,
        &ldz,
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn cgghrd(
    compq: u8,
    compz: u8,
    n: i32,
    ilo: i32,
    ihi: i32,
    a: &mut [c32],
    lda: i32,
    b: &mut [c32],
    ldb: i32,
    q: &mut [c32],
    ldq: i32,
    z: &mut [c32],
    ldz: i32,
    info: &mut i32,
) {
    ffi::cgghrd_(
        &(compq as c_char),
        &(compz as c_char),
        &n,
        &ilo,
        &ihi,
        a.as_mut_ptr() as *mut _,
        &lda,
        b.as_mut_ptr() as *mut _,
        &ldb,
        q.as_mut_ptr() as *mut _,
        &ldq,
        z.as_mut_ptr() as *mut _,
        &ldz,
        info,
    )
}

#[inline]
pub unsafe fn dgghrd(
    compq: u8,
    compz: u8,
    n: i32,
    ilo: i32,
    ihi: i32,
    a: &mut [f64],
    lda: i32,
    b: &mut [f64],
    ldb: i32,
    q: &mut [f64],
    ldq: i32,
    z: &mut [f64],
    ldz: i32,
    info: &mut i32,
) {
    ffi::dgghrd_(
        &(compq as c_char),
        &(compz as c_char),
        &n,
        &ilo,
        &ihi,
        a.as_mut_ptr(),
        &lda,
        b.as_mut_ptr(),
        &ldb,
        q.as_mut_ptr(),
        &ldq,
        z.as_mut_ptr(),
        &ldz,
        info,
    )
}

#[inline]
pub unsafe fn sgghrd(
    compq: u8,
    compz: u8,
    n: i32,
    ilo: i32,
    ihi: i32,
    a: &mut [f32],
    lda: i32,
    b: &mut [f32],
    ldb: i32,
    q: &mut [f32],
    ldq: i32,
    z: &mut [f32],
    ldz: i32,
    info: &mut i32,
) {
    ffi::sgghrd_(
        &(compq as c_char),
        &(compz as c_char),
        &n,
        &ilo,
        &ihi,
        a.as_mut_ptr(),
        &lda,
        b.as_mut_ptr(),
        &ldb,
        q.as_mut_ptr(),
        &ldq,
        z.as_mut_ptr(),
        &ldz,
        info,
    )
}

#[inline]
pub unsafe fn zgghrd(
    compq: u8,
    compz: u8,
    n: i32,
    ilo: i32,
    ihi: i32,
    a: &mut [c64],
    lda: i32,
    b: &mut [c64],
    ldb: i32,
    q: &mut [c64],
    ldq: i32,
    z: &mut [c64],
    ldz: i32,
    info: &mut i32,
) {
    ffi::zgghrd_(
        &(compq as c_char),
        &(compz as c_char),
        &n,
        &ilo,
        &ihi,
        a.as_mut_ptr() as *mut _,
        &lda,
        b.as_mut_ptr() as *mut _,
        &ldb,
        q.as_mut_ptr() as *mut _,
        &ldq,
        z.as_mut_ptr() as *mut _,
        &ldz,
        info,
    )
}

#[inline]
pub unsafe fn cgglse(
    m: i32,
    n: i32,
    p: i32,
    a: &mut [c32],
    lda: i32,
    b: &mut [c32],
    ldb: i32,
    c: &mut [c32],
    d: &mut [c32],
    x: &mut [c32],
    work: &mut [c32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::cgglse_(
        &m,
        &n,
        &p,
        a.as_mut_ptr() as *mut _,
        &lda,
        b.as_mut_ptr() as *mut _,
        &ldb,
        c.as_mut_ptr() as *mut _,
        d.as_mut_ptr() as *mut _,
        x.as_mut_ptr() as *mut _,
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn dgglse(
    m: i32,
    n: i32,
    p: i32,
    a: &mut [f64],
    lda: i32,
    b: &mut [f64],
    ldb: i32,
    c: &mut [f64],
    d: &mut [f64],
    x: &mut [f64],
    work: &mut [f64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::dgglse_(
        &m,
        &n,
        &p,
        a.as_mut_ptr(),
        &lda,
        b.as_mut_ptr(),
        &ldb,
        c.as_mut_ptr(),
        d.as_mut_ptr(),
        x.as_mut_ptr(),
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn sgglse(
    m: i32,
    n: i32,
    p: i32,
    a: &mut [f32],
    lda: i32,
    b: &mut [f32],
    ldb: i32,
    c: &mut [f32],
    d: &mut [f32],
    x: &mut [f32],
    work: &mut [f32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::sgglse_(
        &m,
        &n,
        &p,
        a.as_mut_ptr(),
        &lda,
        b.as_mut_ptr(),
        &ldb,
        c.as_mut_ptr(),
        d.as_mut_ptr(),
        x.as_mut_ptr(),
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn zgglse(
    m: i32,
    n: i32,
    p: i32,
    a: &mut [c64],
    lda: i32,
    b: &mut [c64],
    ldb: i32,
    c: &mut [c64],
    d: &mut [c64],
    x: &mut [c64],
    work: &mut [c64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::zgglse_(
        &m,
        &n,
        &p,
        a.as_mut_ptr() as *mut _,
        &lda,
        b.as_mut_ptr() as *mut _,
        &ldb,
        c.as_mut_ptr() as *mut _,
        d.as_mut_ptr() as *mut _,
        x.as_mut_ptr() as *mut _,
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn cggqrf(
    n: i32,
    m: i32,
    p: i32,
    a: &mut [c32],
    lda: i32,
    taua: &mut [c32],
    b: &mut [c32],
    ldb: i32,
    taub: &mut [c32],
    work: &mut [c32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::cggqrf_(
        &n,
        &m,
        &p,
        a.as_mut_ptr() as *mut _,
        &lda,
        taua.as_mut_ptr() as *mut _,
        b.as_mut_ptr() as *mut _,
        &ldb,
        taub.as_mut_ptr() as *mut _,
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn dggqrf(
    n: i32,
    m: i32,
    p: i32,
    a: &mut [f64],
    lda: i32,
    taua: &mut [f64],
    b: &mut [f64],
    ldb: i32,
    taub: &mut [f64],
    work: &mut [f64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::dggqrf_(
        &n,
        &m,
        &p,
        a.as_mut_ptr(),
        &lda,
        taua.as_mut_ptr(),
        b.as_mut_ptr(),
        &ldb,
        taub.as_mut_ptr(),
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn sggqrf(
    n: i32,
    m: i32,
    p: i32,
    a: &mut [f32],
    lda: i32,
    taua: &mut [f32],
    b: &mut [f32],
    ldb: i32,
    taub: &mut [f32],
    work: &mut [f32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::sggqrf_(
        &n,
        &m,
        &p,
        a.as_mut_ptr(),
        &lda,
        taua.as_mut_ptr(),
        b.as_mut_ptr(),
        &ldb,
        taub.as_mut_ptr(),
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn zggqrf(
    n: i32,
    m: i32,
    p: i32,
    a: &mut [c64],
    lda: i32,
    taua: &mut [c64],
    b: &mut [c64],
    ldb: i32,
    taub: &mut [c64],
    work: &mut [c64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::zggqrf_(
        &n,
        &m,
        &p,
        a.as_mut_ptr() as *mut _,
        &lda,
        taua.as_mut_ptr() as *mut _,
        b.as_mut_ptr() as *mut _,
        &ldb,
        taub.as_mut_ptr() as *mut _,
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn cggrqf(
    m: i32,
    p: i32,
    n: i32,
    a: &mut [c32],
    lda: i32,
    taua: &mut [c32],
    b: &mut [c32],
    ldb: i32,
    taub: &mut [c32],
    work: &mut [c32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::cggrqf_(
        &m,
        &p,
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        taua.as_mut_ptr() as *mut _,
        b.as_mut_ptr() as *mut _,
        &ldb,
        taub.as_mut_ptr() as *mut _,
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn dggrqf(
    m: i32,
    p: i32,
    n: i32,
    a: &mut [f64],
    lda: i32,
    taua: &mut [f64],
    b: &mut [f64],
    ldb: i32,
    taub: &mut [f64],
    work: &mut [f64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::dggrqf_(
        &m,
        &p,
        &n,
        a.as_mut_ptr(),
        &lda,
        taua.as_mut_ptr(),
        b.as_mut_ptr(),
        &ldb,
        taub.as_mut_ptr(),
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn sggrqf(
    m: i32,
    p: i32,
    n: i32,
    a: &mut [f32],
    lda: i32,
    taua: &mut [f32],
    b: &mut [f32],
    ldb: i32,
    taub: &mut [f32],
    work: &mut [f32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::sggrqf_(
        &m,
        &p,
        &n,
        a.as_mut_ptr(),
        &lda,
        taua.as_mut_ptr(),
        b.as_mut_ptr(),
        &ldb,
        taub.as_mut_ptr(),
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn zggrqf(
    m: i32,
    p: i32,
    n: i32,
    a: &mut [c64],
    lda: i32,
    taua: &mut [c64],
    b: &mut [c64],
    ldb: i32,
    taub: &mut [c64],
    work: &mut [c64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::zggrqf_(
        &m,
        &p,
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        taua.as_mut_ptr() as *mut _,
        b.as_mut_ptr() as *mut _,
        &ldb,
        taub.as_mut_ptr() as *mut _,
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn sggsvd(
    jobu: u8,
    jobv: u8,
    jobq: u8,
    m: i32,
    n: i32,
    p: i32,
    k: &mut i32,
    l: &mut i32,
    a: &mut [f32],
    lda: i32,
    b: &mut [f32],
    ldb: i32,
    alpha: &mut [f32],
    beta: &mut [f32],
    u: &mut [f32],
    ldu: i32,
    v: &mut [f32],
    ldv: i32,
    q: &mut [f32],
    ldq: i32,
    work: &mut [f32],
    iwork: &mut [i32],
    info: &mut i32,
) -> i32 {
    ffi::sggsvd_(
        &(jobu as c_char),
        &(jobv as c_char),
        &(jobq as c_char),
        &m,
        &n,
        &p,
        k,
        l,
        a.as_mut_ptr(),
        &lda,
        b.as_mut_ptr(),
        &ldb,
        alpha.as_mut_ptr(),
        beta.as_mut_ptr(),
        u.as_mut_ptr(),
        &ldu,
        v.as_mut_ptr(),
        &ldv,
        q.as_mut_ptr(),
        &ldq,
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn dggsvd(
    jobu: u8,
    jobv: u8,
    jobq: u8,
    m: i32,
    n: i32,
    p: i32,
    k: &mut i32,
    l: &mut i32,
    a: &mut [f64],
    lda: i32,
    b: &mut [f64],
    ldb: i32,
    alpha: &mut [f64],
    beta: &mut [f64],
    u: &mut [f64],
    ldu: i32,
    v: &mut [f64],
    ldv: i32,
    q: &mut [f64],
    ldq: i32,
    work: &mut [f64],
    iwork: &mut [i32],
    info: &mut i32,
) -> i32 {
    ffi::dggsvd_(
        &(jobu as c_char),
        &(jobv as c_char),
        &(jobq as c_char),
        &m,
        &n,
        &p,
        k,
        l,
        a.as_mut_ptr(),
        &lda,
        b.as_mut_ptr(),
        &ldb,
        alpha.as_mut_ptr(),
        beta.as_mut_ptr(),
        u.as_mut_ptr(),
        &ldu,
        v.as_mut_ptr(),
        &ldv,
        q.as_mut_ptr(),
        &ldq,
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn cggsvd(
    jobu: u8,
    jobv: u8,
    jobq: u8,
    m: i32,
    n: i32,
    p: i32,
    k: &mut i32,
    l: &mut i32,
    a: &mut [c32],
    lda: i32,
    b: &mut [c32],
    ldb: i32,
    alpha: &mut [f32],
    beta: &mut [f32],
    u: &mut [c32],
    ldu: i32,
    v: &mut [c32],
    ldv: i32,
    q: &mut [c32],
    ldq: i32,
    work: &mut [c32],
    rwork: &mut [f32],
    iwork: &mut [i32],
    info: &mut i32,
) -> i32 {
    ffi::cggsvd_(
        &(jobu as c_char),
        &(jobv as c_char),
        &(jobq as c_char),
        &m,
        &n,
        &p,
        k,
        l,
        a.as_mut_ptr() as *mut _,
        &lda,
        b.as_mut_ptr() as *mut _,
        &ldb,
        alpha.as_mut_ptr(),
        beta.as_mut_ptr(),
        u.as_mut_ptr() as *mut _,
        &ldu,
        v.as_mut_ptr() as *mut _,
        &ldv,
        q.as_mut_ptr() as *mut _,
        &ldq,
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
        iwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn zggsvd(
    jobu: u8,
    jobv: u8,
    jobq: u8,
    m: i32,
    n: i32,
    p: i32,
    k: &mut i32,
    l: &mut i32,
    a: &mut [c64],
    lda: i32,
    b: &mut [c64],
    ldb: i32,
    alpha: &mut [f64],
    beta: &mut [f64],
    u: &mut [c64],
    ldu: i32,
    v: &mut [c64],
    ldv: i32,
    q: &mut [c64],
    ldq: i32,
    work: &mut [c64],
    rwork: &mut [f64],
    iwork: &mut [i32],
    info: &mut i32,
) -> i32 {
    ffi::zggsvd_(
        &(jobu as c_char),
        &(jobv as c_char),
        &(jobq as c_char),
        &m,
        &n,
        &p,
        k,
        l,
        a.as_mut_ptr() as *mut _,
        &lda,
        b.as_mut_ptr() as *mut _,
        &ldb,
        alpha.as_mut_ptr(),
        beta.as_mut_ptr(),
        u.as_mut_ptr() as *mut _,
        &ldu,
        v.as_mut_ptr() as *mut _,
        &ldv,
        q.as_mut_ptr() as *mut _,
        &ldq,
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
        iwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn cggsvd3(
    jobu: u8,
    jobv: u8,
    jobq: u8,
    m: i32,
    n: i32,
    p: i32,
    k: &mut i32,
    l: &mut i32,
    a: &mut [c32],
    lda: i32,
    b: &mut [c32],
    ldb: i32,
    alpha: &mut [f32],
    beta: &mut [f32],
    u: &mut [c32],
    ldu: i32,
    v: &mut [c32],
    ldv: i32,
    q: &mut [c32],
    ldq: i32,
    work: &mut [c32],
    lwork: i32,
    rwork: &mut [f32],
    iwork: &mut [i32],
    info: &mut i32,
) {
    ffi::cggsvd3_(
        &(jobu as c_char),
        &(jobv as c_char),
        &(jobq as c_char),
        &m,
        &n,
        &p,
        k,
        l,
        a.as_mut_ptr() as *mut _,
        &lda,
        b.as_mut_ptr() as *mut _,
        &ldb,
        alpha.as_mut_ptr(),
        beta.as_mut_ptr(),
        u.as_mut_ptr() as *mut _,
        &ldu,
        v.as_mut_ptr() as *mut _,
        &ldv,
        q.as_mut_ptr() as *mut _,
        &ldq,
        work.as_mut_ptr() as *mut _,
        &lwork,
        rwork.as_mut_ptr(),
        iwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn dggsvd3(
    jobu: u8,
    jobv: u8,
    jobq: u8,
    m: i32,
    n: i32,
    p: i32,
    k: &mut i32,
    l: &mut i32,
    a: &mut [f64],
    lda: i32,
    b: &mut [f64],
    ldb: i32,
    alpha: &mut [f64],
    beta: &mut [f64],
    u: &mut [f64],
    ldu: i32,
    v: &mut [f64],
    ldv: i32,
    q: &mut [f64],
    ldq: i32,
    work: &mut [f64],
    lwork: i32,
    iwork: &mut [i32],
    info: &mut i32,
) {
    ffi::dggsvd3_(
        &(jobu as c_char),
        &(jobv as c_char),
        &(jobq as c_char),
        &m,
        &n,
        &p,
        k,
        l,
        a.as_mut_ptr(),
        &lda,
        b.as_mut_ptr(),
        &ldb,
        alpha.as_mut_ptr(),
        beta.as_mut_ptr(),
        u.as_mut_ptr(),
        &ldu,
        v.as_mut_ptr(),
        &ldv,
        q.as_mut_ptr(),
        &ldq,
        work.as_mut_ptr(),
        &lwork,
        iwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn sggsvd3(
    jobu: u8,
    jobv: u8,
    jobq: u8,
    m: i32,
    n: i32,
    p: i32,
    k: &mut i32,
    l: &mut i32,
    a: &mut [f32],
    lda: i32,
    b: &mut [f32],
    ldb: i32,
    alpha: &mut [f32],
    beta: &mut [f32],
    u: &mut [f32],
    ldu: i32,
    v: &mut [f32],
    ldv: i32,
    q: &mut [f32],
    ldq: i32,
    work: &mut [f32],
    lwork: i32,
    iwork: &mut [i32],
    info: &mut i32,
) {
    ffi::sggsvd3_(
        &(jobu as c_char),
        &(jobv as c_char),
        &(jobq as c_char),
        &m,
        &n,
        &p,
        k,
        l,
        a.as_mut_ptr(),
        &lda,
        b.as_mut_ptr(),
        &ldb,
        alpha.as_mut_ptr(),
        beta.as_mut_ptr(),
        u.as_mut_ptr(),
        &ldu,
        v.as_mut_ptr(),
        &ldv,
        q.as_mut_ptr(),
        &ldq,
        work.as_mut_ptr(),
        &lwork,
        iwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn zggsvd3(
    jobu: u8,
    jobv: u8,
    jobq: u8,
    m: i32,
    n: i32,
    p: i32,
    k: &mut i32,
    l: &mut i32,
    a: &mut [c64],
    lda: i32,
    b: &mut [c64],
    ldb: i32,
    alpha: &mut [f64],
    beta: &mut [f64],
    u: &mut [c64],
    ldu: i32,
    v: &mut [c64],
    ldv: i32,
    q: &mut [c64],
    ldq: i32,
    work: &mut [c64],
    lwork: i32,
    rwork: &mut [f64],
    iwork: &mut [i32],
    info: &mut i32,
) {
    ffi::zggsvd3_(
        &(jobu as c_char),
        &(jobv as c_char),
        &(jobq as c_char),
        &m,
        &n,
        &p,
        k,
        l,
        a.as_mut_ptr() as *mut _,
        &lda,
        b.as_mut_ptr() as *mut _,
        &ldb,
        alpha.as_mut_ptr(),
        beta.as_mut_ptr(),
        u.as_mut_ptr() as *mut _,
        &ldu,
        v.as_mut_ptr() as *mut _,
        &ldv,
        q.as_mut_ptr() as *mut _,
        &ldq,
        work.as_mut_ptr() as *mut _,
        &lwork,
        rwork.as_mut_ptr(),
        iwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn sggsvp(
    jobu: u8,
    jobv: u8,
    jobq: u8,
    m: i32,
    p: i32,
    n: i32,
    a: &mut [f32],
    lda: i32,
    b: &mut [f32],
    ldb: i32,
    tola: &mut f32,
    tolb: &mut f32,
    k: &mut i32,
    l: &mut i32,
    u: &mut [f32],
    ldu: i32,
    v: &mut [f32],
    ldv: i32,
    q: &mut [f32],
    ldq: i32,
    iwork: &mut [i32],
    tau: &mut [f32],
    work: &mut [f32],
    info: &mut i32,
) -> i32 {
    ffi::sggsvp_(
        &(jobu as c_char),
        &(jobv as c_char),
        &(jobq as c_char),
        &m,
        &p,
        &n,
        a.as_mut_ptr(),
        &lda,
        b.as_mut_ptr(),
        &ldb,
        tola,
        tolb,
        k,
        l,
        u.as_mut_ptr(),
        &ldu,
        v.as_mut_ptr(),
        &ldv,
        q.as_mut_ptr(),
        &ldq,
        iwork.as_mut_ptr(),
        tau.as_mut_ptr(),
        work.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn dggsvp(
    jobu: u8,
    jobv: u8,
    jobq: u8,
    m: i32,
    p: i32,
    n: i32,
    a: &mut [f64],
    lda: i32,
    b: &mut [f64],
    ldb: i32,
    tola: &mut f64,
    tolb: &mut f64,
    k: &mut i32,
    l: &mut i32,
    u: &mut [f64],
    ldu: i32,
    v: &mut [f64],
    ldv: i32,
    q: &mut [f64],
    ldq: i32,
    iwork: &mut [i32],
    tau: &mut [f64],
    work: &mut [f64],
    info: &mut i32,
) -> i32 {
    ffi::dggsvp_(
        &(jobu as c_char),
        &(jobv as c_char),
        &(jobq as c_char),
        &m,
        &p,
        &n,
        a.as_mut_ptr(),
        &lda,
        b.as_mut_ptr(),
        &ldb,
        tola,
        tolb,
        k,
        l,
        u.as_mut_ptr(),
        &ldu,
        v.as_mut_ptr(),
        &ldv,
        q.as_mut_ptr(),
        &ldq,
        iwork.as_mut_ptr(),
        tau.as_mut_ptr(),
        work.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn cggsvp(
    jobu: u8,
    jobv: u8,
    jobq: u8,
    m: i32,
    p: i32,
    n: i32,
    a: &mut [c32],
    lda: i32,
    b: &mut [c32],
    ldb: i32,
    tola: &mut f32,
    tolb: &mut f32,
    k: &mut i32,
    l: &mut i32,
    u: &mut [c32],
    ldu: i32,
    v: &mut [c32],
    ldv: i32,
    q: &mut [c32],
    ldq: i32,
    iwork: &mut [i32],
    rwork: &mut [f32],
    tau: &mut [c32],
    work: &mut [c32],
    info: &mut i32,
) -> i32 {
    ffi::cggsvp_(
        &(jobu as c_char),
        &(jobv as c_char),
        &(jobq as c_char),
        &m,
        &p,
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        b.as_mut_ptr() as *mut _,
        &ldb,
        tola,
        tolb,
        k,
        l,
        u.as_mut_ptr() as *mut _,
        &ldu,
        v.as_mut_ptr() as *mut _,
        &ldv,
        q.as_mut_ptr() as *mut _,
        &ldq,
        iwork.as_mut_ptr(),
        rwork.as_mut_ptr(),
        tau.as_mut_ptr() as *mut _,
        work.as_mut_ptr() as *mut _,
        info,
    )
}

#[inline]
pub unsafe fn zggsvp(
    jobu: u8,
    jobv: u8,
    jobq: u8,
    m: i32,
    p: i32,
    n: i32,
    a: &mut [c64],
    lda: i32,
    b: &mut [c64],
    ldb: i32,
    tola: &mut f64,
    tolb: &mut f64,
    k: &mut i32,
    l: &mut i32,
    u: &mut [c64],
    ldu: i32,
    v: &mut [c64],
    ldv: i32,
    q: &mut [c64],
    ldq: i32,
    iwork: &mut [i32],
    rwork: &mut [f64],
    tau: &mut [c64],
    work: &mut [c64],
    info: &mut i32,
) -> i32 {
    ffi::zggsvp_(
        &(jobu as c_char),
        &(jobv as c_char),
        &(jobq as c_char),
        &m,
        &p,
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        b.as_mut_ptr() as *mut _,
        &ldb,
        tola,
        tolb,
        k,
        l,
        u.as_mut_ptr() as *mut _,
        &ldu,
        v.as_mut_ptr() as *mut _,
        &ldv,
        q.as_mut_ptr() as *mut _,
        &ldq,
        iwork.as_mut_ptr(),
        rwork.as_mut_ptr(),
        tau.as_mut_ptr() as *mut _,
        work.as_mut_ptr() as *mut _,
        info,
    )
}

#[inline]
pub unsafe fn cggsvp3(
    jobu: u8,
    jobv: u8,
    jobq: u8,
    m: i32,
    p: i32,
    n: i32,
    a: &mut [c32],
    lda: i32,
    b: &mut [c32],
    ldb: i32,
    tola: f32,
    tolb: f32,
    k: &mut i32,
    l: &mut i32,
    u: &mut [c32],
    ldu: i32,
    v: &mut [c32],
    ldv: i32,
    q: &mut [c32],
    ldq: i32,
    iwork: &mut [i32],
    rwork: &mut [f32],
    tau: &mut [c32],
    work: &mut [c32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::cggsvp3_(
        &(jobu as c_char),
        &(jobv as c_char),
        &(jobq as c_char),
        &m,
        &p,
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        b.as_mut_ptr() as *mut _,
        &ldb,
        &tola,
        &tolb,
        k,
        l,
        u.as_mut_ptr() as *mut _,
        &ldu,
        v.as_mut_ptr() as *mut _,
        &ldv,
        q.as_mut_ptr() as *mut _,
        &ldq,
        iwork.as_mut_ptr(),
        rwork.as_mut_ptr(),
        tau.as_mut_ptr() as *mut _,
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn dggsvp3(
    jobu: u8,
    jobv: u8,
    jobq: u8,
    m: i32,
    p: i32,
    n: i32,
    a: &mut [f64],
    lda: i32,
    b: &mut [f64],
    ldb: i32,
    tola: f64,
    tolb: f64,
    k: &mut i32,
    l: &mut i32,
    u: &mut [f64],
    ldu: i32,
    v: &mut [f64],
    ldv: i32,
    q: &mut [f64],
    ldq: i32,
    iwork: &mut [i32],
    tau: &mut [f64],
    work: &mut [f64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::dggsvp3_(
        &(jobu as c_char),
        &(jobv as c_char),
        &(jobq as c_char),
        &m,
        &p,
        &n,
        a.as_mut_ptr(),
        &lda,
        b.as_mut_ptr(),
        &ldb,
        &tola,
        &tolb,
        k,
        l,
        u.as_mut_ptr(),
        &ldu,
        v.as_mut_ptr(),
        &ldv,
        q.as_mut_ptr(),
        &ldq,
        iwork.as_mut_ptr(),
        tau.as_mut_ptr(),
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn sggsvp3(
    jobu: u8,
    jobv: u8,
    jobq: u8,
    m: i32,
    p: i32,
    n: i32,
    a: &mut [f32],
    lda: i32,
    b: &mut [f32],
    ldb: i32,
    tola: f32,
    tolb: f32,
    k: &mut i32,
    l: &mut i32,
    u: &mut [f32],
    ldu: i32,
    v: &mut [f32],
    ldv: i32,
    q: &mut [f32],
    ldq: i32,
    iwork: &mut [i32],
    tau: &mut [f32],
    work: &mut [f32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::sggsvp3_(
        &(jobu as c_char),
        &(jobv as c_char),
        &(jobq as c_char),
        &m,
        &p,
        &n,
        a.as_mut_ptr(),
        &lda,
        b.as_mut_ptr(),
        &ldb,
        &tola,
        &tolb,
        k,
        l,
        u.as_mut_ptr(),
        &ldu,
        v.as_mut_ptr(),
        &ldv,
        q.as_mut_ptr(),
        &ldq,
        iwork.as_mut_ptr(),
        tau.as_mut_ptr(),
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn zggsvp3(
    jobu: u8,
    jobv: u8,
    jobq: u8,
    m: i32,
    p: i32,
    n: i32,
    a: &mut [c64],
    lda: i32,
    b: &mut [c64],
    ldb: i32,
    tola: f64,
    tolb: f64,
    k: &mut i32,
    l: &mut i32,
    u: &mut [c64],
    ldu: i32,
    v: &mut [c64],
    ldv: i32,
    q: &mut [c64],
    ldq: i32,
    iwork: &mut [i32],
    rwork: &mut [f64],
    tau: &mut [c64],
    work: &mut [c64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::zggsvp3_(
        &(jobu as c_char),
        &(jobv as c_char),
        &(jobq as c_char),
        &m,
        &p,
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        b.as_mut_ptr() as *mut _,
        &ldb,
        &tola,
        &tolb,
        k,
        l,
        u.as_mut_ptr() as *mut _,
        &ldu,
        v.as_mut_ptr() as *mut _,
        &ldv,
        q.as_mut_ptr() as *mut _,
        &ldq,
        iwork.as_mut_ptr(),
        rwork.as_mut_ptr(),
        tau.as_mut_ptr() as *mut _,
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn cgtcon(
    norm: u8,
    n: i32,
    dl: &[c32],
    d: &[c32],
    du: &[c32],
    du2: &[c32],
    ipiv: &[i32],
    anorm: f32,
    rcond: &mut f32,
    work: &mut [c32],
    info: &mut i32,
) {
    ffi::cgtcon_(
        &(norm as c_char),
        &n,
        dl.as_ptr() as *const _,
        d.as_ptr() as *const _,
        du.as_ptr() as *const _,
        du2.as_ptr() as *const _,
        ipiv.as_ptr(),
        &anorm,
        rcond,
        work.as_mut_ptr() as *mut _,
        info,
    )
}

#[inline]
pub unsafe fn dgtcon(
    norm: u8,
    n: i32,
    dl: &[f64],
    d: &[f64],
    du: &[f64],
    du2: &[f64],
    ipiv: &[i32],
    anorm: f64,
    rcond: &mut f64,
    work: &mut [f64],
    iwork: &mut [i32],
    info: &mut i32,
) {
    ffi::dgtcon_(
        &(norm as c_char),
        &n,
        dl.as_ptr(),
        d.as_ptr(),
        du.as_ptr(),
        du2.as_ptr(),
        ipiv.as_ptr(),
        &anorm,
        rcond,
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn sgtcon(
    norm: u8,
    n: i32,
    dl: &[f32],
    d: &[f32],
    du: &[f32],
    du2: &[f32],
    ipiv: &[i32],
    anorm: f32,
    rcond: &mut f32,
    work: &mut [f32],
    iwork: &mut [i32],
    info: &mut i32,
) {
    ffi::sgtcon_(
        &(norm as c_char),
        &n,
        dl.as_ptr(),
        d.as_ptr(),
        du.as_ptr(),
        du2.as_ptr(),
        ipiv.as_ptr(),
        &anorm,
        rcond,
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn zgtcon(
    norm: u8,
    n: i32,
    dl: &[c64],
    d: &[c64],
    du: &[c64],
    du2: &[c64],
    ipiv: &[i32],
    anorm: f64,
    rcond: &mut f64,
    work: &mut [c64],
    info: &mut i32,
) {
    ffi::zgtcon_(
        &(norm as c_char),
        &n,
        dl.as_ptr() as *const _,
        d.as_ptr() as *const _,
        du.as_ptr() as *const _,
        du2.as_ptr() as *const _,
        ipiv.as_ptr(),
        &anorm,
        rcond,
        work.as_mut_ptr() as *mut _,
        info,
    )
}

#[inline]
pub unsafe fn cgtrfs(
    trans: u8,
    n: i32,
    nrhs: i32,
    dl: &[c32],
    d: &[c32],
    du: &[c32],
    dlf: &[c32],
    df: &[c32],
    duf: &[c32],
    du2: &[c32],
    ipiv: &[i32],
    b: &[c32],
    ldb: i32,
    x: &mut [c32],
    ldx: i32,
    ferr: &mut [f32],
    berr: &mut [f32],
    work: &mut [c32],
    rwork: &mut [f32],
    info: &mut i32,
) {
    ffi::cgtrfs_(
        &(trans as c_char),
        &n,
        &nrhs,
        dl.as_ptr() as *const _,
        d.as_ptr() as *const _,
        du.as_ptr() as *const _,
        dlf.as_ptr() as *const _,
        df.as_ptr() as *const _,
        duf.as_ptr() as *const _,
        du2.as_ptr() as *const _,
        ipiv.as_ptr(),
        b.as_ptr() as *const _,
        &ldb,
        x.as_mut_ptr() as *mut _,
        &ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn dgtrfs(
    trans: u8,
    n: i32,
    nrhs: i32,
    dl: &[f64],
    d: &[f64],
    du: &[f64],
    dlf: &[f64],
    df: &[f64],
    duf: &[f64],
    du2: &[f64],
    ipiv: &[i32],
    b: &[f64],
    ldb: i32,
    x: &mut [f64],
    ldx: i32,
    ferr: &mut [f64],
    berr: &mut [f64],
    work: &mut [f64],
    iwork: &mut [i32],
    info: &mut i32,
) {
    ffi::dgtrfs_(
        &(trans as c_char),
        &n,
        &nrhs,
        dl.as_ptr(),
        d.as_ptr(),
        du.as_ptr(),
        dlf.as_ptr(),
        df.as_ptr(),
        duf.as_ptr(),
        du2.as_ptr(),
        ipiv.as_ptr(),
        b.as_ptr(),
        &ldb,
        x.as_mut_ptr(),
        &ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn sgtrfs(
    trans: u8,
    n: i32,
    nrhs: i32,
    dl: &[f32],
    d: &[f32],
    du: &[f32],
    dlf: &[f32],
    df: &[f32],
    duf: &[f32],
    du2: &[f32],
    ipiv: &[i32],
    b: &[f32],
    ldb: i32,
    x: &mut [f32],
    ldx: i32,
    ferr: &mut [f32],
    berr: &mut [f32],
    work: &mut [f32],
    iwork: &mut [i32],
    info: &mut i32,
) {
    ffi::sgtrfs_(
        &(trans as c_char),
        &n,
        &nrhs,
        dl.as_ptr(),
        d.as_ptr(),
        du.as_ptr(),
        dlf.as_ptr(),
        df.as_ptr(),
        duf.as_ptr(),
        du2.as_ptr(),
        ipiv.as_ptr(),
        b.as_ptr(),
        &ldb,
        x.as_mut_ptr(),
        &ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn zgtrfs(
    trans: u8,
    n: i32,
    nrhs: i32,
    dl: &[c64],
    d: &[c64],
    du: &[c64],
    dlf: &[c64],
    df: &[c64],
    duf: &[c64],
    du2: &[c64],
    ipiv: &[i32],
    b: &[c64],
    ldb: i32,
    x: &mut [c64],
    ldx: i32,
    ferr: &mut [f64],
    berr: &mut [f64],
    work: &mut [c64],
    rwork: &mut [f64],
    info: &mut i32,
) {
    ffi::zgtrfs_(
        &(trans as c_char),
        &n,
        &nrhs,
        dl.as_ptr() as *const _,
        d.as_ptr() as *const _,
        du.as_ptr() as *const _,
        dlf.as_ptr() as *const _,
        df.as_ptr() as *const _,
        duf.as_ptr() as *const _,
        du2.as_ptr() as *const _,
        ipiv.as_ptr(),
        b.as_ptr() as *const _,
        &ldb,
        x.as_mut_ptr() as *mut _,
        &ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn cgtsv(
    n: i32,
    nrhs: i32,
    dl: &mut [c32],
    d: &mut [c32],
    du: &mut [c32],
    b: &mut [c32],
    ldb: i32,
    info: &mut i32,
) {
    ffi::cgtsv_(
        &n,
        &nrhs,
        dl.as_mut_ptr() as *mut _,
        d.as_mut_ptr() as *mut _,
        du.as_mut_ptr() as *mut _,
        b.as_mut_ptr() as *mut _,
        &ldb,
        info,
    )
}

#[inline]
pub unsafe fn dgtsv(
    n: i32,
    nrhs: i32,
    dl: &mut [f64],
    d: &mut [f64],
    du: &mut [f64],
    b: &mut [f64],
    ldb: i32,
    info: &mut i32,
) {
    ffi::dgtsv_(
        &n,
        &nrhs,
        dl.as_mut_ptr(),
        d.as_mut_ptr(),
        du.as_mut_ptr(),
        b.as_mut_ptr(),
        &ldb,
        info,
    )
}

#[inline]
pub unsafe fn sgtsv(
    n: i32,
    nrhs: i32,
    dl: &mut [f32],
    d: &mut [f32],
    du: &mut [f32],
    b: &mut [f32],
    ldb: i32,
    info: &mut i32,
) {
    ffi::sgtsv_(
        &n,
        &nrhs,
        dl.as_mut_ptr(),
        d.as_mut_ptr(),
        du.as_mut_ptr(),
        b.as_mut_ptr(),
        &ldb,
        info,
    )
}

#[inline]
pub unsafe fn zgtsv(
    n: i32,
    nrhs: i32,
    dl: &mut [c64],
    d: &mut [c64],
    du: &mut [c64],
    b: &mut [c64],
    ldb: i32,
    info: &mut i32,
) {
    ffi::zgtsv_(
        &n,
        &nrhs,
        dl.as_mut_ptr() as *mut _,
        d.as_mut_ptr() as *mut _,
        du.as_mut_ptr() as *mut _,
        b.as_mut_ptr() as *mut _,
        &ldb,
        info,
    )
}

#[inline]
pub unsafe fn cgtsvx(
    fact: u8,
    trans: u8,
    n: i32,
    nrhs: i32,
    dl: &[c32],
    d: &[c32],
    du: &[c32],
    dlf: &mut [c32],
    df: &mut [c32],
    duf: &mut [c32],
    du2: &mut [c32],
    ipiv: &mut [i32],
    b: &[c32],
    ldb: i32,
    x: &mut [c32],
    ldx: i32,
    rcond: &mut f32,
    ferr: &mut [f32],
    berr: &mut [f32],
    work: &mut [c32],
    rwork: &mut [f32],
    info: &mut i32,
) {
    ffi::cgtsvx_(
        &(fact as c_char),
        &(trans as c_char),
        &n,
        &nrhs,
        dl.as_ptr() as *const _,
        d.as_ptr() as *const _,
        du.as_ptr() as *const _,
        dlf.as_mut_ptr() as *mut _,
        df.as_mut_ptr() as *mut _,
        duf.as_mut_ptr() as *mut _,
        du2.as_mut_ptr() as *mut _,
        ipiv.as_mut_ptr(),
        b.as_ptr() as *const _,
        &ldb,
        x.as_mut_ptr() as *mut _,
        &ldx,
        rcond,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn dgtsvx(
    fact: u8,
    trans: u8,
    n: i32,
    nrhs: i32,
    dl: &[f64],
    d: &[f64],
    du: &[f64],
    dlf: &mut [f64],
    df: &mut [f64],
    duf: &mut [f64],
    du2: &mut [f64],
    ipiv: &mut [i32],
    b: &[f64],
    ldb: i32,
    x: &mut [f64],
    ldx: i32,
    rcond: &mut f64,
    ferr: &mut [f64],
    berr: &mut [f64],
    work: &mut [f64],
    iwork: &mut [i32],
    info: &mut i32,
) {
    ffi::dgtsvx_(
        &(fact as c_char),
        &(trans as c_char),
        &n,
        &nrhs,
        dl.as_ptr(),
        d.as_ptr(),
        du.as_ptr(),
        dlf.as_mut_ptr(),
        df.as_mut_ptr(),
        duf.as_mut_ptr(),
        du2.as_mut_ptr(),
        ipiv.as_mut_ptr(),
        b.as_ptr(),
        &ldb,
        x.as_mut_ptr(),
        &ldx,
        rcond,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn sgtsvx(
    fact: u8,
    trans: u8,
    n: i32,
    nrhs: i32,
    dl: &[f32],
    d: &[f32],
    du: &[f32],
    dlf: &mut [f32],
    df: &mut [f32],
    duf: &mut [f32],
    du2: &mut [f32],
    ipiv: &mut [i32],
    b: &[f32],
    ldb: i32,
    x: &mut [f32],
    ldx: i32,
    rcond: &mut f32,
    ferr: &mut [f32],
    berr: &mut [f32],
    work: &mut [f32],
    iwork: &mut [i32],
    info: &mut i32,
) {
    ffi::sgtsvx_(
        &(fact as c_char),
        &(trans as c_char),
        &n,
        &nrhs,
        dl.as_ptr(),
        d.as_ptr(),
        du.as_ptr(),
        dlf.as_mut_ptr(),
        df.as_mut_ptr(),
        duf.as_mut_ptr(),
        du2.as_mut_ptr(),
        ipiv.as_mut_ptr(),
        b.as_ptr(),
        &ldb,
        x.as_mut_ptr(),
        &ldx,
        rcond,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn zgtsvx(
    fact: u8,
    trans: u8,
    n: i32,
    nrhs: i32,
    dl: &[c64],
    d: &[c64],
    du: &[c64],
    dlf: &mut [c64],
    df: &mut [c64],
    duf: &mut [c64],
    du2: &mut [c64],
    ipiv: &mut [i32],
    b: &[c64],
    ldb: i32,
    x: &mut [c64],
    ldx: i32,
    rcond: &mut f64,
    ferr: &mut [f64],
    berr: &mut [f64],
    work: &mut [c64],
    rwork: &mut [f64],
    info: &mut i32,
) {
    ffi::zgtsvx_(
        &(fact as c_char),
        &(trans as c_char),
        &n,
        &nrhs,
        dl.as_ptr() as *const _,
        d.as_ptr() as *const _,
        du.as_ptr() as *const _,
        dlf.as_mut_ptr() as *mut _,
        df.as_mut_ptr() as *mut _,
        duf.as_mut_ptr() as *mut _,
        du2.as_mut_ptr() as *mut _,
        ipiv.as_mut_ptr(),
        b.as_ptr() as *const _,
        &ldb,
        x.as_mut_ptr() as *mut _,
        &ldx,
        rcond,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn cgttrf(
    n: i32,
    dl: &mut [c32],
    d: &mut [c32],
    du: &mut [c32],
    du2: &mut [c32],
    ipiv: &mut [i32],
    info: &mut i32,
) {
    ffi::cgttrf_(
        &n,
        dl.as_mut_ptr() as *mut _,
        d.as_mut_ptr() as *mut _,
        du.as_mut_ptr() as *mut _,
        du2.as_mut_ptr() as *mut _,
        ipiv.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn dgttrf(
    n: i32,
    dl: &mut [f64],
    d: &mut [f64],
    du: &mut [f64],
    du2: &mut [f64],
    ipiv: &mut [i32],
    info: &mut i32,
) {
    ffi::dgttrf_(
        &n,
        dl.as_mut_ptr(),
        d.as_mut_ptr(),
        du.as_mut_ptr(),
        du2.as_mut_ptr(),
        ipiv.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn sgttrf(
    n: i32,
    dl: &mut [f32],
    d: &mut [f32],
    du: &mut [f32],
    du2: &mut [f32],
    ipiv: &mut [i32],
    info: &mut i32,
) {
    ffi::sgttrf_(
        &n,
        dl.as_mut_ptr(),
        d.as_mut_ptr(),
        du.as_mut_ptr(),
        du2.as_mut_ptr(),
        ipiv.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn zgttrf(
    n: i32,
    dl: &mut [c64],
    d: &mut [c64],
    du: &mut [c64],
    du2: &mut [c64],
    ipiv: &mut [i32],
    info: &mut i32,
) {
    ffi::zgttrf_(
        &n,
        dl.as_mut_ptr() as *mut _,
        d.as_mut_ptr() as *mut _,
        du.as_mut_ptr() as *mut _,
        du2.as_mut_ptr() as *mut _,
        ipiv.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn cgttrs(
    trans: u8,
    n: i32,
    nrhs: i32,
    dl: &[c32],
    d: &[c32],
    du: &[c32],
    du2: &[c32],
    ipiv: &[i32],
    b: &mut [c32],
    ldb: i32,
    info: &mut i32,
) {
    ffi::cgttrs_(
        &(trans as c_char),
        &n,
        &nrhs,
        dl.as_ptr() as *const _,
        d.as_ptr() as *const _,
        du.as_ptr() as *const _,
        du2.as_ptr() as *const _,
        ipiv.as_ptr(),
        b.as_mut_ptr() as *mut _,
        &ldb,
        info,
    )
}

#[inline]
pub unsafe fn dgttrs(
    trans: u8,
    n: i32,
    nrhs: i32,
    dl: &[f64],
    d: &[f64],
    du: &[f64],
    du2: &[f64],
    ipiv: &[i32],
    b: &mut [f64],
    ldb: i32,
    info: &mut i32,
) {
    ffi::dgttrs_(
        &(trans as c_char),
        &n,
        &nrhs,
        dl.as_ptr(),
        d.as_ptr(),
        du.as_ptr(),
        du2.as_ptr(),
        ipiv.as_ptr(),
        b.as_mut_ptr(),
        &ldb,
        info,
    )
}

#[inline]
pub unsafe fn sgttrs(
    trans: u8,
    n: i32,
    nrhs: i32,
    dl: &[f32],
    d: &[f32],
    du: &[f32],
    du2: &[f32],
    ipiv: &[i32],
    b: &mut [f32],
    ldb: i32,
    info: &mut i32,
) {
    ffi::sgttrs_(
        &(trans as c_char),
        &n,
        &nrhs,
        dl.as_ptr(),
        d.as_ptr(),
        du.as_ptr(),
        du2.as_ptr(),
        ipiv.as_ptr(),
        b.as_mut_ptr(),
        &ldb,
        info,
    )
}

#[inline]
pub unsafe fn zgttrs(
    trans: u8,
    n: i32,
    nrhs: i32,
    dl: &[c64],
    d: &[c64],
    du: &[c64],
    du2: &[c64],
    ipiv: &[i32],
    b: &mut [c64],
    ldb: i32,
    info: &mut i32,
) {
    ffi::zgttrs_(
        &(trans as c_char),
        &n,
        &nrhs,
        dl.as_ptr() as *const _,
        d.as_ptr() as *const _,
        du.as_ptr() as *const _,
        du2.as_ptr() as *const _,
        ipiv.as_ptr(),
        b.as_mut_ptr() as *mut _,
        &ldb,
        info,
    )
}

#[inline]
pub unsafe fn chbev(
    jobz: u8,
    uplo: u8,
    n: i32,
    kd: i32,
    ab: &mut [c32],
    ldab: i32,
    w: &mut [f32],
    z: &mut [c32],
    ldz: i32,
    work: &mut [c32],
    rwork: &mut [f32],
    info: &mut i32,
) {
    ffi::chbev_(
        &(jobz as c_char),
        &(uplo as c_char),
        &n,
        &kd,
        ab.as_mut_ptr() as *mut _,
        &ldab,
        w.as_mut_ptr(),
        z.as_mut_ptr() as *mut _,
        &ldz,
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn zhbev(
    jobz: u8,
    uplo: u8,
    n: i32,
    kd: i32,
    ab: &mut [c64],
    ldab: i32,
    w: &mut [f64],
    z: &mut [c64],
    ldz: i32,
    work: &mut [c64],
    rwork: &mut [f64],
    info: &mut i32,
) {
    ffi::zhbev_(
        &(jobz as c_char),
        &(uplo as c_char),
        &n,
        &kd,
        ab.as_mut_ptr() as *mut _,
        &ldab,
        w.as_mut_ptr(),
        z.as_mut_ptr() as *mut _,
        &ldz,
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn chbev_2stage(
    jobz: u8,
    uplo: u8,
    n: i32,
    kd: i32,
    ab: &mut [c32],
    ldab: i32,
    w: &mut [f32],
    z: &mut [c32],
    ldz: i32,
    work: &mut [c32],
    lwork: i32,
    rwork: &mut [f32],
    info: &mut i32,
) {
    ffi::chbev_2stage_(
        &(jobz as c_char),
        &(uplo as c_char),
        &n,
        &kd,
        ab.as_mut_ptr() as *mut _,
        &ldab,
        w.as_mut_ptr(),
        z.as_mut_ptr() as *mut _,
        &ldz,
        work.as_mut_ptr() as *mut _,
        &lwork,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn zhbev_2stage(
    jobz: u8,
    uplo: u8,
    n: i32,
    kd: i32,
    ab: &mut [c64],
    ldab: i32,
    w: &mut [f64],
    z: &mut [c64],
    ldz: i32,
    work: &mut [c64],
    lwork: i32,
    rwork: &mut [f64],
    info: &mut i32,
) {
    ffi::zhbev_2stage_(
        &(jobz as c_char),
        &(uplo as c_char),
        &n,
        &kd,
        ab.as_mut_ptr() as *mut _,
        &ldab,
        w.as_mut_ptr(),
        z.as_mut_ptr() as *mut _,
        &ldz,
        work.as_mut_ptr() as *mut _,
        &lwork,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn chbevd(
    jobz: u8,
    uplo: u8,
    n: i32,
    kd: i32,
    ab: &mut [c32],
    ldab: i32,
    w: &mut [f32],
    z: &mut [c32],
    ldz: i32,
    work: &mut [c32],
    lwork: i32,
    rwork: &mut [f32],
    lrwork: i32,
    iwork: &mut [i32],
    liwork: i32,
    info: &mut i32,
) {
    ffi::chbevd_(
        &(jobz as c_char),
        &(uplo as c_char),
        &n,
        &kd,
        ab.as_mut_ptr() as *mut _,
        &ldab,
        w.as_mut_ptr(),
        z.as_mut_ptr() as *mut _,
        &ldz,
        work.as_mut_ptr() as *mut _,
        &lwork,
        rwork.as_mut_ptr(),
        &lrwork,
        iwork.as_mut_ptr(),
        &liwork,
        info,
    )
}

#[inline]
pub unsafe fn zhbevd(
    jobz: u8,
    uplo: u8,
    n: i32,
    kd: i32,
    ab: &mut [c64],
    ldab: i32,
    w: &mut [f64],
    z: &mut [c64],
    ldz: i32,
    work: &mut [c64],
    lwork: i32,
    rwork: &mut [f64],
    lrwork: i32,
    iwork: &mut [i32],
    liwork: i32,
    info: &mut i32,
) {
    ffi::zhbevd_(
        &(jobz as c_char),
        &(uplo as c_char),
        &n,
        &kd,
        ab.as_mut_ptr() as *mut _,
        &ldab,
        w.as_mut_ptr(),
        z.as_mut_ptr() as *mut _,
        &ldz,
        work.as_mut_ptr() as *mut _,
        &lwork,
        rwork.as_mut_ptr(),
        &lrwork,
        iwork.as_mut_ptr(),
        &liwork,
        info,
    )
}

#[inline]
pub unsafe fn chbevd_2stage(
    jobz: u8,
    uplo: u8,
    n: i32,
    kd: i32,
    ab: &mut [c32],
    ldab: i32,
    w: &mut [f32],
    z: &mut [c32],
    ldz: i32,
    work: &mut [c32],
    lwork: i32,
    rwork: &mut [f32],
    lrwork: i32,
    iwork: &mut [i32],
    liwork: i32,
    info: &mut i32,
) {
    ffi::chbevd_2stage_(
        &(jobz as c_char),
        &(uplo as c_char),
        &n,
        &kd,
        ab.as_mut_ptr() as *mut _,
        &ldab,
        w.as_mut_ptr(),
        z.as_mut_ptr() as *mut _,
        &ldz,
        work.as_mut_ptr() as *mut _,
        &lwork,
        rwork.as_mut_ptr(),
        &lrwork,
        iwork.as_mut_ptr(),
        &liwork,
        info,
    )
}

#[inline]
pub unsafe fn zhbevd_2stage(
    jobz: u8,
    uplo: u8,
    n: i32,
    kd: i32,
    ab: &mut [c64],
    ldab: i32,
    w: &mut [f64],
    z: &mut [c64],
    ldz: i32,
    work: &mut [c64],
    lwork: i32,
    rwork: &mut [f64],
    lrwork: i32,
    iwork: &mut [i32],
    liwork: i32,
    info: &mut i32,
) {
    ffi::zhbevd_2stage_(
        &(jobz as c_char),
        &(uplo as c_char),
        &n,
        &kd,
        ab.as_mut_ptr() as *mut _,
        &ldab,
        w.as_mut_ptr(),
        z.as_mut_ptr() as *mut _,
        &ldz,
        work.as_mut_ptr() as *mut _,
        &lwork,
        rwork.as_mut_ptr(),
        &lrwork,
        iwork.as_mut_ptr(),
        &liwork,
        info,
    )
}

#[inline]
pub unsafe fn chbevx(
    jobz: u8,
    range: u8,
    uplo: u8,
    n: i32,
    kd: i32,
    ab: &mut [c32],
    ldab: i32,
    q: &mut [c32],
    ldq: i32,
    vl: f32,
    vu: f32,
    il: i32,
    iu: i32,
    abstol: f32,
    m: &mut i32,
    w: &mut [f32],
    z: &mut [c32],
    ldz: i32,
    work: &mut [c32],
    rwork: &mut [f32],
    iwork: &mut [i32],
    ifail: &mut [i32],
    info: &mut i32,
) {
    ffi::chbevx_(
        &(jobz as c_char),
        &(range as c_char),
        &(uplo as c_char),
        &n,
        &kd,
        ab.as_mut_ptr() as *mut _,
        &ldab,
        q.as_mut_ptr() as *mut _,
        &ldq,
        &vl,
        &vu,
        &il,
        &iu,
        &abstol,
        m,
        w.as_mut_ptr(),
        z.as_mut_ptr() as *mut _,
        &ldz,
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
        iwork.as_mut_ptr(),
        ifail.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn zhbevx(
    jobz: u8,
    range: u8,
    uplo: u8,
    n: i32,
    kd: i32,
    ab: &mut [c64],
    ldab: i32,
    q: &mut [c64],
    ldq: i32,
    vl: f64,
    vu: f64,
    il: i32,
    iu: i32,
    abstol: f64,
    m: &mut i32,
    w: &mut [f64],
    z: &mut [c64],
    ldz: i32,
    work: &mut [c64],
    rwork: &mut [f64],
    iwork: &mut [i32],
    ifail: &mut [i32],
    info: &mut i32,
) {
    ffi::zhbevx_(
        &(jobz as c_char),
        &(range as c_char),
        &(uplo as c_char),
        &n,
        &kd,
        ab.as_mut_ptr() as *mut _,
        &ldab,
        q.as_mut_ptr() as *mut _,
        &ldq,
        &vl,
        &vu,
        &il,
        &iu,
        &abstol,
        m,
        w.as_mut_ptr(),
        z.as_mut_ptr() as *mut _,
        &ldz,
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
        iwork.as_mut_ptr(),
        ifail.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn chbevx_2stage(
    jobz: u8,
    range: u8,
    uplo: u8,
    n: i32,
    kd: i32,
    ab: &mut [c32],
    ldab: i32,
    q: &mut [c32],
    ldq: i32,
    vl: f32,
    vu: f32,
    il: i32,
    iu: i32,
    abstol: f32,
    m: &mut i32,
    w: &mut [f32],
    z: &mut [c32],
    ldz: i32,
    work: &mut [c32],
    lwork: i32,
    rwork: &mut [f32],
    iwork: &mut [i32],
    ifail: &mut [i32],
    info: &mut i32,
) {
    ffi::chbevx_2stage_(
        &(jobz as c_char),
        &(range as c_char),
        &(uplo as c_char),
        &n,
        &kd,
        ab.as_mut_ptr() as *mut _,
        &ldab,
        q.as_mut_ptr() as *mut _,
        &ldq,
        &vl,
        &vu,
        &il,
        &iu,
        &abstol,
        m,
        w.as_mut_ptr(),
        z.as_mut_ptr() as *mut _,
        &ldz,
        work.as_mut_ptr() as *mut _,
        &lwork,
        rwork.as_mut_ptr(),
        iwork.as_mut_ptr(),
        ifail.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn zhbevx_2stage(
    jobz: u8,
    range: u8,
    uplo: u8,
    n: i32,
    kd: i32,
    ab: &mut [c64],
    ldab: i32,
    q: &mut [c64],
    ldq: i32,
    vl: f64,
    vu: f64,
    il: i32,
    iu: i32,
    abstol: f64,
    m: &mut i32,
    w: &mut [f64],
    z: &mut [c64],
    ldz: i32,
    work: &mut [c64],
    lwork: i32,
    rwork: &mut [f64],
    iwork: &mut [i32],
    ifail: &mut [i32],
    info: &mut i32,
) {
    ffi::zhbevx_2stage_(
        &(jobz as c_char),
        &(range as c_char),
        &(uplo as c_char),
        &n,
        &kd,
        ab.as_mut_ptr() as *mut _,
        &ldab,
        q.as_mut_ptr() as *mut _,
        &ldq,
        &vl,
        &vu,
        &il,
        &iu,
        &abstol,
        m,
        w.as_mut_ptr(),
        z.as_mut_ptr() as *mut _,
        &ldz,
        work.as_mut_ptr() as *mut _,
        &lwork,
        rwork.as_mut_ptr(),
        iwork.as_mut_ptr(),
        ifail.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn chbgst(
    vect: u8,
    uplo: u8,
    n: i32,
    ka: i32,
    kb: i32,
    ab: &mut [c32],
    ldab: i32,
    bb: &[c32],
    ldbb: i32,
    x: &mut [c32],
    ldx: i32,
    work: &mut [c32],
    rwork: &mut [f32],
    info: &mut i32,
) {
    ffi::chbgst_(
        &(vect as c_char),
        &(uplo as c_char),
        &n,
        &ka,
        &kb,
        ab.as_mut_ptr() as *mut _,
        &ldab,
        bb.as_ptr() as *const _,
        &ldbb,
        x.as_mut_ptr() as *mut _,
        &ldx,
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn zhbgst(
    vect: u8,
    uplo: u8,
    n: i32,
    ka: i32,
    kb: i32,
    ab: &mut [c64],
    ldab: i32,
    bb: &[c64],
    ldbb: i32,
    x: &mut [c64],
    ldx: i32,
    work: &mut [c64],
    rwork: &mut [f64],
    info: &mut i32,
) {
    ffi::zhbgst_(
        &(vect as c_char),
        &(uplo as c_char),
        &n,
        &ka,
        &kb,
        ab.as_mut_ptr() as *mut _,
        &ldab,
        bb.as_ptr() as *const _,
        &ldbb,
        x.as_mut_ptr() as *mut _,
        &ldx,
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn chbgv(
    jobz: u8,
    uplo: u8,
    n: i32,
    ka: i32,
    kb: i32,
    ab: &mut [c32],
    ldab: i32,
    bb: &mut [c32],
    ldbb: i32,
    w: &mut [f32],
    z: &mut [c32],
    ldz: i32,
    work: &mut [c32],
    rwork: &mut [f32],
    info: &mut i32,
) {
    ffi::chbgv_(
        &(jobz as c_char),
        &(uplo as c_char),
        &n,
        &ka,
        &kb,
        ab.as_mut_ptr() as *mut _,
        &ldab,
        bb.as_mut_ptr() as *mut _,
        &ldbb,
        w.as_mut_ptr(),
        z.as_mut_ptr() as *mut _,
        &ldz,
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn zhbgv(
    jobz: u8,
    uplo: u8,
    n: i32,
    ka: i32,
    kb: i32,
    ab: &mut [c64],
    ldab: i32,
    bb: &mut [c64],
    ldbb: i32,
    w: &mut [f64],
    z: &mut [c64],
    ldz: i32,
    work: &mut [c64],
    rwork: &mut [f64],
    info: &mut i32,
) {
    ffi::zhbgv_(
        &(jobz as c_char),
        &(uplo as c_char),
        &n,
        &ka,
        &kb,
        ab.as_mut_ptr() as *mut _,
        &ldab,
        bb.as_mut_ptr() as *mut _,
        &ldbb,
        w.as_mut_ptr(),
        z.as_mut_ptr() as *mut _,
        &ldz,
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn chbgvd(
    jobz: u8,
    uplo: u8,
    n: i32,
    ka: i32,
    kb: i32,
    ab: &mut [c32],
    ldab: i32,
    bb: &mut [c32],
    ldbb: i32,
    w: &mut [f32],
    z: &mut [c32],
    ldz: i32,
    work: &mut [c32],
    lwork: i32,
    rwork: &mut [f32],
    lrwork: i32,
    iwork: &mut [i32],
    liwork: i32,
    info: &mut i32,
) {
    ffi::chbgvd_(
        &(jobz as c_char),
        &(uplo as c_char),
        &n,
        &ka,
        &kb,
        ab.as_mut_ptr() as *mut _,
        &ldab,
        bb.as_mut_ptr() as *mut _,
        &ldbb,
        w.as_mut_ptr(),
        z.as_mut_ptr() as *mut _,
        &ldz,
        work.as_mut_ptr() as *mut _,
        &lwork,
        rwork.as_mut_ptr(),
        &lrwork,
        iwork.as_mut_ptr(),
        &liwork,
        info,
    )
}

#[inline]
pub unsafe fn zhbgvd(
    jobz: u8,
    uplo: u8,
    n: i32,
    ka: i32,
    kb: i32,
    ab: &mut [c64],
    ldab: i32,
    bb: &mut [c64],
    ldbb: i32,
    w: &mut [f64],
    z: &mut [c64],
    ldz: i32,
    work: &mut [c64],
    lwork: i32,
    rwork: &mut [f64],
    lrwork: i32,
    iwork: &mut [i32],
    liwork: i32,
    info: &mut i32,
) {
    ffi::zhbgvd_(
        &(jobz as c_char),
        &(uplo as c_char),
        &n,
        &ka,
        &kb,
        ab.as_mut_ptr() as *mut _,
        &ldab,
        bb.as_mut_ptr() as *mut _,
        &ldbb,
        w.as_mut_ptr(),
        z.as_mut_ptr() as *mut _,
        &ldz,
        work.as_mut_ptr() as *mut _,
        &lwork,
        rwork.as_mut_ptr(),
        &lrwork,
        iwork.as_mut_ptr(),
        &liwork,
        info,
    )
}

#[inline]
pub unsafe fn chbgvx(
    jobz: u8,
    range: u8,
    uplo: u8,
    n: i32,
    ka: i32,
    kb: i32,
    ab: &mut [c32],
    ldab: i32,
    bb: &mut [c32],
    ldbb: i32,
    q: &mut [c32],
    ldq: i32,
    vl: f32,
    vu: f32,
    il: i32,
    iu: i32,
    abstol: f32,
    m: &mut i32,
    w: &mut [f32],
    z: &mut [c32],
    ldz: i32,
    work: &mut [c32],
    rwork: &mut [f32],
    iwork: &mut [i32],
    ifail: &mut [i32],
    info: &mut i32,
) {
    ffi::chbgvx_(
        &(jobz as c_char),
        &(range as c_char),
        &(uplo as c_char),
        &n,
        &ka,
        &kb,
        ab.as_mut_ptr() as *mut _,
        &ldab,
        bb.as_mut_ptr() as *mut _,
        &ldbb,
        q.as_mut_ptr() as *mut _,
        &ldq,
        &vl,
        &vu,
        &il,
        &iu,
        &abstol,
        m,
        w.as_mut_ptr(),
        z.as_mut_ptr() as *mut _,
        &ldz,
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
        iwork.as_mut_ptr(),
        ifail.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn zhbgvx(
    jobz: u8,
    range: u8,
    uplo: u8,
    n: i32,
    ka: i32,
    kb: i32,
    ab: &mut [c64],
    ldab: i32,
    bb: &mut [c64],
    ldbb: i32,
    q: &mut [c64],
    ldq: i32,
    vl: f64,
    vu: f64,
    il: i32,
    iu: i32,
    abstol: f64,
    m: &mut i32,
    w: &mut [f64],
    z: &mut [c64],
    ldz: i32,
    work: &mut [c64],
    rwork: &mut [f64],
    iwork: &mut [i32],
    ifail: &mut [i32],
    info: &mut i32,
) {
    ffi::zhbgvx_(
        &(jobz as c_char),
        &(range as c_char),
        &(uplo as c_char),
        &n,
        &ka,
        &kb,
        ab.as_mut_ptr() as *mut _,
        &ldab,
        bb.as_mut_ptr() as *mut _,
        &ldbb,
        q.as_mut_ptr() as *mut _,
        &ldq,
        &vl,
        &vu,
        &il,
        &iu,
        &abstol,
        m,
        w.as_mut_ptr(),
        z.as_mut_ptr() as *mut _,
        &ldz,
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
        iwork.as_mut_ptr(),
        ifail.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn chbtrd(
    vect: u8,
    uplo: u8,
    n: i32,
    kd: i32,
    ab: &mut [c32],
    ldab: i32,
    d: &mut [f32],
    e: &mut [f32],
    q: &mut [c32],
    ldq: i32,
    work: &mut [c32],
    info: &mut i32,
) {
    ffi::chbtrd_(
        &(vect as c_char),
        &(uplo as c_char),
        &n,
        &kd,
        ab.as_mut_ptr() as *mut _,
        &ldab,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        q.as_mut_ptr() as *mut _,
        &ldq,
        work.as_mut_ptr() as *mut _,
        info,
    )
}

#[inline]
pub unsafe fn zhbtrd(
    vect: u8,
    uplo: u8,
    n: i32,
    kd: i32,
    ab: &mut [c64],
    ldab: i32,
    d: &mut [f64],
    e: &mut [f64],
    q: &mut [c64],
    ldq: i32,
    work: &mut [c64],
    info: &mut i32,
) {
    ffi::zhbtrd_(
        &(vect as c_char),
        &(uplo as c_char),
        &n,
        &kd,
        ab.as_mut_ptr() as *mut _,
        &ldab,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        q.as_mut_ptr() as *mut _,
        &ldq,
        work.as_mut_ptr() as *mut _,
        info,
    )
}

#[inline]
pub unsafe fn checon(
    uplo: u8,
    n: i32,
    a: &[c32],
    lda: i32,
    ipiv: &[i32],
    anorm: f32,
    rcond: &mut f32,
    work: &mut [c32],
    info: &mut i32,
) {
    ffi::checon_(
        &(uplo as c_char),
        &n,
        a.as_ptr() as *const _,
        &lda,
        ipiv.as_ptr(),
        &anorm,
        rcond,
        work.as_mut_ptr() as *mut _,
        info,
    )
}

#[inline]
pub unsafe fn zhecon(
    uplo: u8,
    n: i32,
    a: &[c64],
    lda: i32,
    ipiv: &[i32],
    anorm: f64,
    rcond: &mut f64,
    work: &mut [c64],
    info: &mut i32,
) {
    ffi::zhecon_(
        &(uplo as c_char),
        &n,
        a.as_ptr() as *const _,
        &lda,
        ipiv.as_ptr(),
        &anorm,
        rcond,
        work.as_mut_ptr() as *mut _,
        info,
    )
}

#[inline]
pub unsafe fn checon_3(
    uplo: u8,
    n: i32,
    a: &[c32],
    lda: i32,
    e: &[c32],
    ipiv: &[i32],
    anorm: f32,
    rcond: &mut f32,
    work: &mut [c32],
    info: &mut i32,
) {
    ffi::checon_3_(
        &(uplo as c_char),
        &n,
        a.as_ptr() as *const _,
        &lda,
        e.as_ptr() as *const _,
        ipiv.as_ptr(),
        &anorm,
        rcond,
        work.as_mut_ptr() as *mut _,
        info,
    )
}

#[inline]
pub unsafe fn zhecon_3(
    uplo: u8,
    n: i32,
    a: &[c64],
    lda: i32,
    e: &[c64],
    ipiv: &[i32],
    anorm: f64,
    rcond: &mut f64,
    work: &mut [c64],
    info: &mut i32,
) {
    ffi::zhecon_3_(
        &(uplo as c_char),
        &n,
        a.as_ptr() as *const _,
        &lda,
        e.as_ptr() as *const _,
        ipiv.as_ptr(),
        &anorm,
        rcond,
        work.as_mut_ptr() as *mut _,
        info,
    )
}

#[inline]
pub unsafe fn cheequb(
    uplo: u8,
    n: i32,
    a: &[c32],
    lda: i32,
    s: &mut [f32],
    scond: &mut [f32],
    amax: &mut f32,
    work: &mut [c32],
    info: &mut i32,
) {
    ffi::cheequb_(
        &(uplo as c_char),
        &n,
        a.as_ptr() as *const _,
        &lda,
        s.as_mut_ptr(),
        scond.as_mut_ptr(),
        amax,
        work.as_mut_ptr() as *mut _,
        info,
    )
}

#[inline]
pub unsafe fn zheequb(
    uplo: u8,
    n: i32,
    a: &[c64],
    lda: i32,
    s: &mut [f64],
    scond: &mut [f64],
    amax: &mut f64,
    work: &mut [c64],
    info: &mut i32,
) {
    ffi::zheequb_(
        &(uplo as c_char),
        &n,
        a.as_ptr() as *const _,
        &lda,
        s.as_mut_ptr(),
        scond.as_mut_ptr(),
        amax,
        work.as_mut_ptr() as *mut _,
        info,
    )
}

#[inline]
pub unsafe fn cheev(
    jobz: u8,
    uplo: u8,
    n: i32,
    a: &mut [c32],
    lda: i32,
    w: &mut [f32],
    work: &mut [c32],
    lwork: i32,
    rwork: &mut [f32],
    info: &mut i32,
) {
    ffi::cheev_(
        &(jobz as c_char),
        &(uplo as c_char),
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        w.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        &lwork,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn zheev(
    jobz: u8,
    uplo: u8,
    n: i32,
    a: &mut [c64],
    lda: i32,
    w: &mut [f64],
    work: &mut [c64],
    lwork: i32,
    rwork: &mut [f64],
    info: &mut i32,
) {
    ffi::zheev_(
        &(jobz as c_char),
        &(uplo as c_char),
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        w.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        &lwork,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn cheev_2stage(
    jobz: u8,
    uplo: u8,
    n: i32,
    a: &mut [c32],
    lda: i32,
    w: &mut [f32],
    work: &mut [c32],
    lwork: i32,
    rwork: &mut [f32],
    info: &mut i32,
) {
    ffi::cheev_2stage_(
        &(jobz as c_char),
        &(uplo as c_char),
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        w.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        &lwork,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn zheev_2stage(
    jobz: u8,
    uplo: u8,
    n: i32,
    a: &mut [c64],
    lda: i32,
    w: &mut [f64],
    work: &mut [c64],
    lwork: i32,
    rwork: &mut [f64],
    info: &mut i32,
) {
    ffi::zheev_2stage_(
        &(jobz as c_char),
        &(uplo as c_char),
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        w.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        &lwork,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn cheevd(
    jobz: u8,
    uplo: u8,
    n: i32,
    a: &mut [c32],
    lda: i32,
    w: &mut [f32],
    work: &mut [c32],
    lwork: i32,
    rwork: &mut [f32],
    lrwork: i32,
    iwork: &mut [i32],
    liwork: i32,
    info: &mut i32,
) {
    ffi::cheevd_(
        &(jobz as c_char),
        &(uplo as c_char),
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        w.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        &lwork,
        rwork.as_mut_ptr(),
        &lrwork,
        iwork.as_mut_ptr(),
        &liwork,
        info,
    )
}

#[inline]
pub unsafe fn zheevd(
    jobz: u8,
    uplo: u8,
    n: i32,
    a: &mut [c64],
    lda: i32,
    w: &mut [f64],
    work: &mut [c64],
    lwork: i32,
    rwork: &mut [f64],
    lrwork: i32,
    iwork: &mut [i32],
    liwork: i32,
    info: &mut i32,
) {
    ffi::zheevd_(
        &(jobz as c_char),
        &(uplo as c_char),
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        w.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        &lwork,
        rwork.as_mut_ptr(),
        &lrwork,
        iwork.as_mut_ptr(),
        &liwork,
        info,
    )
}

#[inline]
pub unsafe fn cheevd_2stage(
    jobz: u8,
    uplo: u8,
    n: i32,
    a: &mut [c32],
    lda: i32,
    w: &mut [f32],
    work: &mut [c32],
    lwork: i32,
    rwork: &mut [f32],
    lrwork: i32,
    iwork: &mut [i32],
    liwork: i32,
    info: &mut i32,
) {
    ffi::cheevd_2stage_(
        &(jobz as c_char),
        &(uplo as c_char),
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        w.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        &lwork,
        rwork.as_mut_ptr(),
        &lrwork,
        iwork.as_mut_ptr(),
        &liwork,
        info,
    )
}

#[inline]
pub unsafe fn zheevd_2stage(
    jobz: u8,
    uplo: u8,
    n: i32,
    a: &mut [c64],
    lda: i32,
    w: &mut [f64],
    work: &mut [c64],
    lwork: i32,
    rwork: &mut [f64],
    lrwork: i32,
    iwork: &mut [i32],
    liwork: i32,
    info: &mut i32,
) {
    ffi::zheevd_2stage_(
        &(jobz as c_char),
        &(uplo as c_char),
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        w.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        &lwork,
        rwork.as_mut_ptr(),
        &lrwork,
        iwork.as_mut_ptr(),
        &liwork,
        info,
    )
}

#[inline]
pub unsafe fn cheevr(
    jobz: u8,
    range: u8,
    uplo: u8,
    n: i32,
    a: &mut [c32],
    lda: i32,
    vl: f32,
    vu: f32,
    il: i32,
    iu: i32,
    abstol: f32,
    m: &mut i32,
    w: &mut [f32],
    z: &mut [c32],
    ldz: i32,
    isuppz: &mut [i32],
    work: &mut [c32],
    lwork: i32,
    rwork: &mut [f32],
    lrwork: i32,
    iwork: &mut [i32],
    liwork: i32,
    info: &mut i32,
) {
    ffi::cheevr_(
        &(jobz as c_char),
        &(range as c_char),
        &(uplo as c_char),
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        &vl,
        &vu,
        &il,
        &iu,
        &abstol,
        m,
        w.as_mut_ptr(),
        z.as_mut_ptr() as *mut _,
        &ldz,
        isuppz.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        &lwork,
        rwork.as_mut_ptr(),
        &lrwork,
        iwork.as_mut_ptr(),
        &liwork,
        info,
    )
}

#[inline]
pub unsafe fn zheevr(
    jobz: u8,
    range: u8,
    uplo: u8,
    n: i32,
    a: &mut [c64],
    lda: i32,
    vl: f64,
    vu: f64,
    il: i32,
    iu: i32,
    abstol: f64,
    m: &mut i32,
    w: &mut [f64],
    z: &mut [c64],
    ldz: i32,
    isuppz: &mut [i32],
    work: &mut [c64],
    lwork: i32,
    rwork: &mut [f64],
    lrwork: i32,
    iwork: &mut [i32],
    liwork: i32,
    info: &mut i32,
) {
    ffi::zheevr_(
        &(jobz as c_char),
        &(range as c_char),
        &(uplo as c_char),
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        &vl,
        &vu,
        &il,
        &iu,
        &abstol,
        m,
        w.as_mut_ptr(),
        z.as_mut_ptr() as *mut _,
        &ldz,
        isuppz.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        &lwork,
        rwork.as_mut_ptr(),
        &lrwork,
        iwork.as_mut_ptr(),
        &liwork,
        info,
    )
}

#[inline]
pub unsafe fn cheevr_2stage(
    jobz: u8,
    range: u8,
    uplo: u8,
    n: i32,
    a: &mut [c32],
    lda: i32,
    vl: f32,
    vu: f32,
    il: i32,
    iu: i32,
    abstol: f32,
    m: &mut i32,
    w: &mut [f32],
    z: &mut [c32],
    ldz: i32,
    isuppz: &mut [i32],
    work: &mut [c32],
    lwork: i32,
    rwork: &mut [f32],
    lrwork: i32,
    iwork: &mut [i32],
    liwork: i32,
    info: &mut i32,
) {
    ffi::cheevr_2stage_(
        &(jobz as c_char),
        &(range as c_char),
        &(uplo as c_char),
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        &vl,
        &vu,
        &il,
        &iu,
        &abstol,
        m,
        w.as_mut_ptr(),
        z.as_mut_ptr() as *mut _,
        &ldz,
        isuppz.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        &lwork,
        rwork.as_mut_ptr(),
        &lrwork,
        iwork.as_mut_ptr(),
        &liwork,
        info,
    )
}

#[inline]
pub unsafe fn zheevr_2stage(
    jobz: u8,
    range: u8,
    uplo: u8,
    n: i32,
    a: &mut [c64],
    lda: i32,
    vl: f64,
    vu: f64,
    il: i32,
    iu: i32,
    abstol: f64,
    m: &mut i32,
    w: &mut [f64],
    z: &mut [c64],
    ldz: i32,
    isuppz: &mut [i32],
    work: &mut [c64],
    lwork: i32,
    rwork: &mut [f64],
    lrwork: i32,
    iwork: &mut [i32],
    liwork: i32,
    info: &mut i32,
) {
    ffi::zheevr_2stage_(
        &(jobz as c_char),
        &(range as c_char),
        &(uplo as c_char),
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        &vl,
        &vu,
        &il,
        &iu,
        &abstol,
        m,
        w.as_mut_ptr(),
        z.as_mut_ptr() as *mut _,
        &ldz,
        isuppz.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        &lwork,
        rwork.as_mut_ptr(),
        &lrwork,
        iwork.as_mut_ptr(),
        &liwork,
        info,
    )
}

#[inline]
pub unsafe fn cheevx(
    jobz: u8,
    range: u8,
    uplo: u8,
    n: i32,
    a: &mut [c32],
    lda: i32,
    vl: f32,
    vu: f32,
    il: i32,
    iu: i32,
    abstol: f32,
    m: &mut i32,
    w: &mut [f32],
    z: &mut [c32],
    ldz: i32,
    work: &mut [c32],
    lwork: i32,
    rwork: &mut [f32],
    iwork: &mut [i32],
    ifail: &mut [i32],
    info: &mut i32,
) {
    ffi::cheevx_(
        &(jobz as c_char),
        &(range as c_char),
        &(uplo as c_char),
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        &vl,
        &vu,
        &il,
        &iu,
        &abstol,
        m,
        w.as_mut_ptr(),
        z.as_mut_ptr() as *mut _,
        &ldz,
        work.as_mut_ptr() as *mut _,
        &lwork,
        rwork.as_mut_ptr(),
        iwork.as_mut_ptr(),
        ifail.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn zheevx(
    jobz: u8,
    range: u8,
    uplo: u8,
    n: i32,
    a: &mut [c64],
    lda: i32,
    vl: f64,
    vu: f64,
    il: i32,
    iu: i32,
    abstol: f64,
    m: &mut i32,
    w: &mut [f64],
    z: &mut [c64],
    ldz: i32,
    work: &mut [c64],
    lwork: i32,
    rwork: &mut [f64],
    iwork: &mut [i32],
    ifail: &mut [i32],
    info: &mut i32,
) {
    ffi::zheevx_(
        &(jobz as c_char),
        &(range as c_char),
        &(uplo as c_char),
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        &vl,
        &vu,
        &il,
        &iu,
        &abstol,
        m,
        w.as_mut_ptr(),
        z.as_mut_ptr() as *mut _,
        &ldz,
        work.as_mut_ptr() as *mut _,
        &lwork,
        rwork.as_mut_ptr(),
        iwork.as_mut_ptr(),
        ifail.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn cheevx_2stage(
    jobz: u8,
    range: u8,
    uplo: u8,
    n: i32,
    a: &mut [c32],
    lda: i32,
    vl: f32,
    vu: f32,
    il: i32,
    iu: i32,
    abstol: f32,
    m: &mut i32,
    w: &mut [f32],
    z: &mut [c32],
    ldz: i32,
    work: &mut [c32],
    lwork: i32,
    rwork: &mut [f32],
    iwork: &mut [i32],
    ifail: &mut [i32],
    info: &mut i32,
) {
    ffi::cheevx_2stage_(
        &(jobz as c_char),
        &(range as c_char),
        &(uplo as c_char),
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        &vl,
        &vu,
        &il,
        &iu,
        &abstol,
        m,
        w.as_mut_ptr(),
        z.as_mut_ptr() as *mut _,
        &ldz,
        work.as_mut_ptr() as *mut _,
        &lwork,
        rwork.as_mut_ptr(),
        iwork.as_mut_ptr(),
        ifail.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn zheevx_2stage(
    jobz: u8,
    range: u8,
    uplo: u8,
    n: i32,
    a: &mut [c64],
    lda: i32,
    vl: f64,
    vu: f64,
    il: i32,
    iu: i32,
    abstol: f64,
    m: &mut i32,
    w: &mut [f64],
    z: &mut [c64],
    ldz: i32,
    work: &mut [c64],
    lwork: i32,
    rwork: &mut [f64],
    iwork: &mut [i32],
    ifail: &mut [i32],
    info: &mut i32,
) {
    ffi::zheevx_2stage_(
        &(jobz as c_char),
        &(range as c_char),
        &(uplo as c_char),
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        &vl,
        &vu,
        &il,
        &iu,
        &abstol,
        m,
        w.as_mut_ptr(),
        z.as_mut_ptr() as *mut _,
        &ldz,
        work.as_mut_ptr() as *mut _,
        &lwork,
        rwork.as_mut_ptr(),
        iwork.as_mut_ptr(),
        ifail.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn chegst(
    itype: &[i32],
    uplo: u8,
    n: i32,
    a: &mut [c32],
    lda: i32,
    b: &mut [c32],
    ldb: i32,
    info: &mut i32,
) {
    ffi::chegst_(
        itype.as_ptr(),
        &(uplo as c_char),
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        b.as_mut_ptr() as *mut _,
        &ldb,
        info,
    )
}

#[inline]
pub unsafe fn zhegst(
    itype: &[i32],
    uplo: u8,
    n: i32,
    a: &mut [c64],
    lda: i32,
    b: &mut [c64],
    ldb: i32,
    info: &mut i32,
) {
    ffi::zhegst_(
        itype.as_ptr(),
        &(uplo as c_char),
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        b.as_mut_ptr() as *mut _,
        &ldb,
        info,
    )
}

#[inline]
pub unsafe fn chegv(
    itype: &[i32],
    jobz: u8,
    uplo: u8,
    n: i32,
    a: &mut [c32],
    lda: i32,
    b: &mut [c32],
    ldb: i32,
    w: &mut [f32],
    work: &mut [c32],
    lwork: i32,
    rwork: &mut [f32],
    info: &mut i32,
) {
    ffi::chegv_(
        itype.as_ptr(),
        &(jobz as c_char),
        &(uplo as c_char),
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        b.as_mut_ptr() as *mut _,
        &ldb,
        w.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        &lwork,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn zhegv(
    itype: &[i32],
    jobz: u8,
    uplo: u8,
    n: i32,
    a: &mut [c64],
    lda: i32,
    b: &mut [c64],
    ldb: i32,
    w: &mut [f64],
    work: &mut [c64],
    lwork: i32,
    rwork: &mut [f64],
    info: &mut i32,
) {
    ffi::zhegv_(
        itype.as_ptr(),
        &(jobz as c_char),
        &(uplo as c_char),
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        b.as_mut_ptr() as *mut _,
        &ldb,
        w.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        &lwork,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn chegv_2stage(
    itype: &[i32],
    jobz: u8,
    uplo: u8,
    n: i32,
    a: &mut [c32],
    lda: i32,
    b: &mut [c32],
    ldb: i32,
    w: &mut [f32],
    work: &mut [c32],
    lwork: i32,
    rwork: &mut [f32],
    info: &mut i32,
) {
    ffi::chegv_2stage_(
        itype.as_ptr(),
        &(jobz as c_char),
        &(uplo as c_char),
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        b.as_mut_ptr() as *mut _,
        &ldb,
        w.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        &lwork,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn zhegv_2stage(
    itype: &[i32],
    jobz: u8,
    uplo: u8,
    n: i32,
    a: &mut [c64],
    lda: i32,
    b: &mut [c64],
    ldb: i32,
    w: &mut [f64],
    work: &mut [c64],
    lwork: i32,
    rwork: &mut [f64],
    info: &mut i32,
) {
    ffi::zhegv_2stage_(
        itype.as_ptr(),
        &(jobz as c_char),
        &(uplo as c_char),
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        b.as_mut_ptr() as *mut _,
        &ldb,
        w.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        &lwork,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn chegvd(
    itype: &[i32],
    jobz: u8,
    uplo: u8,
    n: i32,
    a: &mut [c32],
    lda: i32,
    b: &mut [c32],
    ldb: i32,
    w: &mut [f32],
    work: &mut [c32],
    lwork: i32,
    rwork: &mut [f32],
    lrwork: i32,
    iwork: &mut [i32],
    liwork: i32,
    info: &mut i32,
) {
    ffi::chegvd_(
        itype.as_ptr(),
        &(jobz as c_char),
        &(uplo as c_char),
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        b.as_mut_ptr() as *mut _,
        &ldb,
        w.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        &lwork,
        rwork.as_mut_ptr(),
        &lrwork,
        iwork.as_mut_ptr(),
        &liwork,
        info,
    )
}

#[inline]
pub unsafe fn zhegvd(
    itype: &[i32],
    jobz: u8,
    uplo: u8,
    n: i32,
    a: &mut [c64],
    lda: i32,
    b: &mut [c64],
    ldb: i32,
    w: &mut [f64],
    work: &mut [c64],
    lwork: i32,
    rwork: &mut [f64],
    lrwork: i32,
    iwork: &mut [i32],
    liwork: i32,
    info: &mut i32,
) {
    ffi::zhegvd_(
        itype.as_ptr(),
        &(jobz as c_char),
        &(uplo as c_char),
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        b.as_mut_ptr() as *mut _,
        &ldb,
        w.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        &lwork,
        rwork.as_mut_ptr(),
        &lrwork,
        iwork.as_mut_ptr(),
        &liwork,
        info,
    )
}

#[inline]
pub unsafe fn chegvx(
    itype: &[i32],
    jobz: u8,
    range: u8,
    uplo: u8,
    n: i32,
    a: &mut [c32],
    lda: i32,
    b: &mut [c32],
    ldb: i32,
    vl: f32,
    vu: f32,
    il: i32,
    iu: i32,
    abstol: f32,
    m: &mut i32,
    w: &mut [f32],
    z: &mut [c32],
    ldz: i32,
    work: &mut [c32],
    lwork: i32,
    rwork: &mut [f32],
    iwork: &mut [i32],
    ifail: &mut [i32],
    info: &mut i32,
) {
    ffi::chegvx_(
        itype.as_ptr(),
        &(jobz as c_char),
        &(range as c_char),
        &(uplo as c_char),
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        b.as_mut_ptr() as *mut _,
        &ldb,
        &vl,
        &vu,
        &il,
        &iu,
        &abstol,
        m,
        w.as_mut_ptr(),
        z.as_mut_ptr() as *mut _,
        &ldz,
        work.as_mut_ptr() as *mut _,
        &lwork,
        rwork.as_mut_ptr(),
        iwork.as_mut_ptr(),
        ifail.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn zhegvx(
    itype: &[i32],
    jobz: u8,
    range: u8,
    uplo: u8,
    n: i32,
    a: &mut [c64],
    lda: i32,
    b: &mut [c64],
    ldb: i32,
    vl: f64,
    vu: f64,
    il: i32,
    iu: i32,
    abstol: f64,
    m: &mut i32,
    w: &mut [f64],
    z: &mut [c64],
    ldz: i32,
    work: &mut [c64],
    lwork: i32,
    rwork: &mut [f64],
    iwork: &mut [i32],
    ifail: &mut [i32],
    info: &mut i32,
) {
    ffi::zhegvx_(
        itype.as_ptr(),
        &(jobz as c_char),
        &(range as c_char),
        &(uplo as c_char),
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        b.as_mut_ptr() as *mut _,
        &ldb,
        &vl,
        &vu,
        &il,
        &iu,
        &abstol,
        m,
        w.as_mut_ptr(),
        z.as_mut_ptr() as *mut _,
        &ldz,
        work.as_mut_ptr() as *mut _,
        &lwork,
        rwork.as_mut_ptr(),
        iwork.as_mut_ptr(),
        ifail.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn cherfs(
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &[c32],
    lda: i32,
    af: &[c32],
    ldaf: i32,
    ipiv: &[i32],
    b: &[c32],
    ldb: i32,
    x: &mut [c32],
    ldx: i32,
    ferr: &mut [f32],
    berr: &mut [f32],
    work: &mut [c32],
    rwork: &mut [f32],
    info: &mut i32,
) {
    ffi::cherfs_(
        &(uplo as c_char),
        &n,
        &nrhs,
        a.as_ptr() as *const _,
        &lda,
        af.as_ptr() as *const _,
        &ldaf,
        ipiv.as_ptr(),
        b.as_ptr() as *const _,
        &ldb,
        x.as_mut_ptr() as *mut _,
        &ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn zherfs(
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &[c64],
    lda: i32,
    af: &[c64],
    ldaf: i32,
    ipiv: &[i32],
    b: &[c64],
    ldb: i32,
    x: &mut [c64],
    ldx: i32,
    ferr: &mut [f64],
    berr: &mut [f64],
    work: &mut [c64],
    rwork: &mut [f64],
    info: &mut i32,
) {
    ffi::zherfs_(
        &(uplo as c_char),
        &n,
        &nrhs,
        a.as_ptr() as *const _,
        &lda,
        af.as_ptr() as *const _,
        &ldaf,
        ipiv.as_ptr(),
        b.as_ptr() as *const _,
        &ldb,
        x.as_mut_ptr() as *mut _,
        &ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn cherfsx(
    uplo: u8,
    equed: u8,
    n: i32,
    nrhs: i32,
    a: &[c32],
    lda: i32,
    af: &[c32],
    ldaf: i32,
    ipiv: &[i32],
    s: &mut [f32],
    b: &[c32],
    ldb: i32,
    x: &mut [c32],
    ldx: i32,
    rcond: &mut f32,
    berr: &mut [f32],
    n_err_bnds: i32,
    err_bnds_norm: &mut [f32],
    err_bnds_comp: &mut [f32],
    nparams: &[i32],
    params: &mut [f32],
    work: &mut [c32],
    rwork: &mut [f32],
    info: &mut i32,
) {
    ffi::cherfsx_(
        &(uplo as c_char),
        &(equed as c_char),
        &n,
        &nrhs,
        a.as_ptr() as *const _,
        &lda,
        af.as_ptr() as *const _,
        &ldaf,
        ipiv.as_ptr(),
        s.as_mut_ptr(),
        b.as_ptr() as *const _,
        &ldb,
        x.as_mut_ptr() as *mut _,
        &ldx,
        rcond,
        berr.as_mut_ptr(),
        &n_err_bnds,
        err_bnds_norm.as_mut_ptr(),
        err_bnds_comp.as_mut_ptr(),
        nparams.as_ptr(),
        params.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn zherfsx(
    uplo: u8,
    equed: u8,
    n: i32,
    nrhs: i32,
    a: &[c64],
    lda: i32,
    af: &[c64],
    ldaf: i32,
    ipiv: &[i32],
    s: &mut [f64],
    b: &[c64],
    ldb: i32,
    x: &mut [c64],
    ldx: i32,
    rcond: &mut f64,
    berr: &mut [f64],
    n_err_bnds: i32,
    err_bnds_norm: &mut [f64],
    err_bnds_comp: &mut [f64],
    nparams: &[i32],
    params: &mut [f64],
    work: &mut [c64],
    rwork: &mut [f64],
    info: &mut i32,
) {
    ffi::zherfsx_(
        &(uplo as c_char),
        &(equed as c_char),
        &n,
        &nrhs,
        a.as_ptr() as *const _,
        &lda,
        af.as_ptr() as *const _,
        &ldaf,
        ipiv.as_ptr(),
        s.as_mut_ptr(),
        b.as_ptr() as *const _,
        &ldb,
        x.as_mut_ptr() as *mut _,
        &ldx,
        rcond,
        berr.as_mut_ptr(),
        &n_err_bnds,
        err_bnds_norm.as_mut_ptr(),
        err_bnds_comp.as_mut_ptr(),
        nparams.as_ptr(),
        params.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn chesv(
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &mut [c32],
    lda: i32,
    ipiv: &mut [i32],
    b: &mut [c32],
    ldb: i32,
    work: &mut [c32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::chesv_(
        &(uplo as c_char),
        &n,
        &nrhs,
        a.as_mut_ptr() as *mut _,
        &lda,
        ipiv.as_mut_ptr(),
        b.as_mut_ptr() as *mut _,
        &ldb,
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn zhesv(
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &mut [c64],
    lda: i32,
    ipiv: &mut [i32],
    b: &mut [c64],
    ldb: i32,
    work: &mut [c64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::zhesv_(
        &(uplo as c_char),
        &n,
        &nrhs,
        a.as_mut_ptr() as *mut _,
        &lda,
        ipiv.as_mut_ptr(),
        b.as_mut_ptr() as *mut _,
        &ldb,
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn chesv_aa(
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &mut [c32],
    lda: i32,
    ipiv: &mut [i32],
    b: &mut [c32],
    ldb: i32,
    work: &mut [c32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::chesv_aa_(
        &(uplo as c_char),
        &n,
        &nrhs,
        a.as_mut_ptr() as *mut _,
        &lda,
        ipiv.as_mut_ptr(),
        b.as_mut_ptr() as *mut _,
        &ldb,
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn zhesv_aa(
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &mut [c64],
    lda: i32,
    ipiv: &mut [i32],
    b: &mut [c64],
    ldb: i32,
    work: &mut [c64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::zhesv_aa_(
        &(uplo as c_char),
        &n,
        &nrhs,
        a.as_mut_ptr() as *mut _,
        &lda,
        ipiv.as_mut_ptr(),
        b.as_mut_ptr() as *mut _,
        &ldb,
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn chesv_aa_2stage(
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &mut [c32],
    lda: i32,
    tb: &mut [c32],
    ltb: &[i32],
    ipiv: &mut [i32],
    ipiv2: &mut [i32],
    b: &mut [c32],
    ldb: i32,
    work: &mut [c32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::chesv_aa_2stage_(
        &(uplo as c_char),
        &n,
        &nrhs,
        a.as_mut_ptr() as *mut _,
        &lda,
        tb.as_mut_ptr() as *mut _,
        ltb.as_ptr(),
        ipiv.as_mut_ptr(),
        ipiv2.as_mut_ptr(),
        b.as_mut_ptr() as *mut _,
        &ldb,
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn zhesv_aa_2stage(
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &mut [c64],
    lda: i32,
    tb: &mut [c64],
    ltb: &[i32],
    ipiv: &mut [i32],
    ipiv2: &mut [i32],
    b: &mut [c64],
    ldb: i32,
    work: &mut [c64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::zhesv_aa_2stage_(
        &(uplo as c_char),
        &n,
        &nrhs,
        a.as_mut_ptr() as *mut _,
        &lda,
        tb.as_mut_ptr() as *mut _,
        ltb.as_ptr(),
        ipiv.as_mut_ptr(),
        ipiv2.as_mut_ptr(),
        b.as_mut_ptr() as *mut _,
        &ldb,
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn chesv_rk(
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &mut [c32],
    lda: i32,
    e: &mut [c32],
    ipiv: &mut [i32],
    b: &mut [c32],
    ldb: i32,
    work: &mut [c32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::chesv_rk_(
        &(uplo as c_char),
        &n,
        &nrhs,
        a.as_mut_ptr() as *mut _,
        &lda,
        e.as_mut_ptr() as *mut _,
        ipiv.as_mut_ptr(),
        b.as_mut_ptr() as *mut _,
        &ldb,
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn zhesv_rk(
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &mut [c64],
    lda: i32,
    e: &mut [c64],
    ipiv: &mut [i32],
    b: &mut [c64],
    ldb: i32,
    work: &mut [c64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::zhesv_rk_(
        &(uplo as c_char),
        &n,
        &nrhs,
        a.as_mut_ptr() as *mut _,
        &lda,
        e.as_mut_ptr() as *mut _,
        ipiv.as_mut_ptr(),
        b.as_mut_ptr() as *mut _,
        &ldb,
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn chesv_rook(
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &mut [c32],
    lda: i32,
    ipiv: &mut [i32],
    b: &mut [c32],
    ldb: i32,
    work: &mut [c32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::chesv_rook_(
        &(uplo as c_char),
        &n,
        &nrhs,
        a.as_mut_ptr() as *mut _,
        &lda,
        ipiv.as_mut_ptr(),
        b.as_mut_ptr() as *mut _,
        &ldb,
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn zhesv_rook(
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &mut [c64],
    lda: i32,
    ipiv: &mut [i32],
    b: &mut [c64],
    ldb: i32,
    work: &mut [c64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::zhesv_rook_(
        &(uplo as c_char),
        &n,
        &nrhs,
        a.as_mut_ptr() as *mut _,
        &lda,
        ipiv.as_mut_ptr(),
        b.as_mut_ptr() as *mut _,
        &ldb,
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn chesvx(
    fact: u8,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &[c32],
    lda: i32,
    af: &mut [c32],
    ldaf: i32,
    ipiv: &mut [i32],
    b: &[c32],
    ldb: i32,
    x: &mut [c32],
    ldx: i32,
    rcond: &mut f32,
    ferr: &mut [f32],
    berr: &mut [f32],
    work: &mut [c32],
    lwork: i32,
    rwork: &mut [f32],
    info: &mut i32,
) {
    ffi::chesvx_(
        &(fact as c_char),
        &(uplo as c_char),
        &n,
        &nrhs,
        a.as_ptr() as *const _,
        &lda,
        af.as_mut_ptr() as *mut _,
        &ldaf,
        ipiv.as_mut_ptr(),
        b.as_ptr() as *const _,
        &ldb,
        x.as_mut_ptr() as *mut _,
        &ldx,
        rcond,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        &lwork,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn zhesvx(
    fact: u8,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &[c64],
    lda: i32,
    af: &mut [c64],
    ldaf: i32,
    ipiv: &mut [i32],
    b: &[c64],
    ldb: i32,
    x: &mut [c64],
    ldx: i32,
    rcond: &mut f64,
    ferr: &mut [f64],
    berr: &mut [f64],
    work: &mut [c64],
    lwork: i32,
    rwork: &mut [f64],
    info: &mut i32,
) {
    ffi::zhesvx_(
        &(fact as c_char),
        &(uplo as c_char),
        &n,
        &nrhs,
        a.as_ptr() as *const _,
        &lda,
        af.as_mut_ptr() as *mut _,
        &ldaf,
        ipiv.as_mut_ptr(),
        b.as_ptr() as *const _,
        &ldb,
        x.as_mut_ptr() as *mut _,
        &ldx,
        rcond,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        &lwork,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn chesvxx(
    fact: u8,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &mut [c32],
    lda: i32,
    af: &mut [c32],
    ldaf: i32,
    ipiv: &mut [i32],
    equed: &mut u8,
    s: &mut [f32],
    b: &mut [c32],
    ldb: i32,
    x: &mut [c32],
    ldx: i32,
    rcond: &mut f32,
    rpvgrw: &mut f32,
    berr: &mut [f32],
    n_err_bnds: i32,
    err_bnds_norm: &mut [f32],
    err_bnds_comp: &mut [f32],
    nparams: &[i32],
    params: &mut [f32],
    work: &mut [c32],
    rwork: &mut [f32],
    info: &mut i32,
) {
    ffi::chesvxx_(
        &(fact as c_char),
        &(uplo as c_char),
        &n,
        &nrhs,
        a.as_mut_ptr() as *mut _,
        &lda,
        af.as_mut_ptr() as *mut _,
        &ldaf,
        ipiv.as_mut_ptr(),
        equed as *mut _ as *mut _,
        s.as_mut_ptr(),
        b.as_mut_ptr() as *mut _,
        &ldb,
        x.as_mut_ptr() as *mut _,
        &ldx,
        rcond,
        rpvgrw,
        berr.as_mut_ptr(),
        &n_err_bnds,
        err_bnds_norm.as_mut_ptr(),
        err_bnds_comp.as_mut_ptr(),
        nparams.as_ptr(),
        params.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn zhesvxx(
    fact: u8,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &mut [c64],
    lda: i32,
    af: &mut [c64],
    ldaf: i32,
    ipiv: &mut [i32],
    equed: &mut u8,
    s: &mut [f64],
    b: &mut [c64],
    ldb: i32,
    x: &mut [c64],
    ldx: i32,
    rcond: &mut f64,
    rpvgrw: &mut f64,
    berr: &mut [f64],
    n_err_bnds: i32,
    err_bnds_norm: &mut [f64],
    err_bnds_comp: &mut [f64],
    nparams: &[i32],
    params: &mut [f64],
    work: &mut [c64],
    rwork: &mut [f64],
    info: &mut i32,
) {
    ffi::zhesvxx_(
        &(fact as c_char),
        &(uplo as c_char),
        &n,
        &nrhs,
        a.as_mut_ptr() as *mut _,
        &lda,
        af.as_mut_ptr() as *mut _,
        &ldaf,
        ipiv.as_mut_ptr(),
        equed as *mut _ as *mut _,
        s.as_mut_ptr(),
        b.as_mut_ptr() as *mut _,
        &ldb,
        x.as_mut_ptr() as *mut _,
        &ldx,
        rcond,
        rpvgrw,
        berr.as_mut_ptr(),
        &n_err_bnds,
        err_bnds_norm.as_mut_ptr(),
        err_bnds_comp.as_mut_ptr(),
        nparams.as_ptr(),
        params.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn cheswapr(uplo: u8, n: i32, a: &mut [c32], lda: i32, i1: &[i32], i2: &[i32]) {
    ffi::cheswapr_(
        &(uplo as c_char),
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        i1.as_ptr(),
        i2.as_ptr(),
    )
}

#[inline]
pub unsafe fn zheswapr(uplo: u8, n: i32, a: &mut [c64], lda: i32, i1: &[i32], i2: &[i32]) {
    ffi::zheswapr_(
        &(uplo as c_char),
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        i1.as_ptr(),
        i2.as_ptr(),
    )
}

#[inline]
pub unsafe fn chetrd(
    uplo: u8,
    n: i32,
    a: &mut [c32],
    lda: i32,
    d: &mut [f32],
    e: &mut [f32],
    tau: &mut [c32],
    work: &mut [c32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::chetrd_(
        &(uplo as c_char),
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        tau.as_mut_ptr() as *mut _,
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn zhetrd(
    uplo: u8,
    n: i32,
    a: &mut [c64],
    lda: i32,
    d: &mut [f64],
    e: &mut [f64],
    tau: &mut [c64],
    work: &mut [c64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::zhetrd_(
        &(uplo as c_char),
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        tau.as_mut_ptr() as *mut _,
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn chetrd_2stage(
    vect: u8,
    uplo: u8,
    n: i32,
    a: &mut [c32],
    lda: i32,
    d: &mut [f32],
    e: &mut [f32],
    tau: &mut [c32],
    hous2: &mut [c32],
    lhous2: &[i32],
    work: &mut [c32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::chetrd_2stage_(
        &(vect as c_char),
        &(uplo as c_char),
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        tau.as_mut_ptr() as *mut _,
        hous2.as_mut_ptr() as *mut _,
        lhous2.as_ptr(),
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn zhetrd_2stage(
    vect: u8,
    uplo: u8,
    n: i32,
    a: &mut [c64],
    lda: i32,
    d: &mut [f64],
    e: &mut [f64],
    tau: &mut [c64],
    hous2: &mut [c64],
    lhous2: &[i32],
    work: &mut [c64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::zhetrd_2stage_(
        &(vect as c_char),
        &(uplo as c_char),
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        tau.as_mut_ptr() as *mut _,
        hous2.as_mut_ptr() as *mut _,
        lhous2.as_ptr(),
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn chetrf(
    uplo: u8,
    n: i32,
    a: &mut [c32],
    lda: i32,
    ipiv: &mut [i32],
    work: &mut [c32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::chetrf_(
        &(uplo as c_char),
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        ipiv.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn zhetrf(
    uplo: u8,
    n: i32,
    a: &mut [c64],
    lda: i32,
    ipiv: &mut [i32],
    work: &mut [c64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::zhetrf_(
        &(uplo as c_char),
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        ipiv.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn chetrf_aa(
    uplo: u8,
    n: i32,
    a: &mut [c32],
    lda: i32,
    ipiv: &mut [i32],
    work: &mut [c32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::chetrf_aa_(
        &(uplo as c_char),
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        ipiv.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn zhetrf_aa(
    uplo: u8,
    n: i32,
    a: &mut [c64],
    lda: i32,
    ipiv: &mut [i32],
    work: &mut [c64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::zhetrf_aa_(
        &(uplo as c_char),
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        ipiv.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn chetrf_aa_2stage(
    uplo: u8,
    n: i32,
    a: &mut [c32],
    lda: i32,
    tb: &mut [c32],
    ltb: &[i32],
    ipiv: &mut [i32],
    ipiv2: &mut [i32],
    work: &mut [c32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::chetrf_aa_2stage_(
        &(uplo as c_char),
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        tb.as_mut_ptr() as *mut _,
        ltb.as_ptr(),
        ipiv.as_mut_ptr(),
        ipiv2.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn zhetrf_aa_2stage(
    uplo: u8,
    n: i32,
    a: &mut [c64],
    lda: i32,
    tb: &mut [c64],
    ltb: &[i32],
    ipiv: &mut [i32],
    ipiv2: &mut [i32],
    work: &mut [c64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::zhetrf_aa_2stage_(
        &(uplo as c_char),
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        tb.as_mut_ptr() as *mut _,
        ltb.as_ptr(),
        ipiv.as_mut_ptr(),
        ipiv2.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn chetrf_rk(
    uplo: u8,
    n: i32,
    a: &mut [c32],
    lda: i32,
    e: &mut [c32],
    ipiv: &mut [i32],
    work: &mut [c32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::chetrf_rk_(
        &(uplo as c_char),
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        e.as_mut_ptr() as *mut _,
        ipiv.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn zhetrf_rk(
    uplo: u8,
    n: i32,
    a: &mut [c64],
    lda: i32,
    e: &mut [c64],
    ipiv: &mut [i32],
    work: &mut [c64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::zhetrf_rk_(
        &(uplo as c_char),
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        e.as_mut_ptr() as *mut _,
        ipiv.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn chetrf_rook(
    uplo: u8,
    n: i32,
    a: &mut [c32],
    lda: i32,
    ipiv: &mut [i32],
    work: &mut [c32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::chetrf_rook_(
        &(uplo as c_char),
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        ipiv.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn zhetrf_rook(
    uplo: u8,
    n: i32,
    a: &mut [c64],
    lda: i32,
    ipiv: &mut [i32],
    work: &mut [c64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::zhetrf_rook_(
        &(uplo as c_char),
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        ipiv.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn chetri(
    uplo: u8,
    n: i32,
    a: &mut [c32],
    lda: i32,
    ipiv: &[i32],
    work: &mut [c32],
    info: &mut i32,
) {
    ffi::chetri_(
        &(uplo as c_char),
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        ipiv.as_ptr(),
        work.as_mut_ptr() as *mut _,
        info,
    )
}

#[inline]
pub unsafe fn zhetri(
    uplo: u8,
    n: i32,
    a: &mut [c64],
    lda: i32,
    ipiv: &[i32],
    work: &mut [c64],
    info: &mut i32,
) {
    ffi::zhetri_(
        &(uplo as c_char),
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        ipiv.as_ptr(),
        work.as_mut_ptr() as *mut _,
        info,
    )
}

#[inline]
pub unsafe fn chetri2(
    uplo: u8,
    n: i32,
    a: &mut [c32],
    lda: i32,
    ipiv: &[i32],
    work: &mut [c32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::chetri2_(
        &(uplo as c_char),
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        ipiv.as_ptr(),
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn zhetri2(
    uplo: u8,
    n: i32,
    a: &mut [c64],
    lda: i32,
    ipiv: &[i32],
    work: &mut [c64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::zhetri2_(
        &(uplo as c_char),
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        ipiv.as_ptr(),
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn chetri2x(
    uplo: u8,
    n: i32,
    a: &mut [c32],
    lda: i32,
    ipiv: &[i32],
    work: &mut [c32],
    nb: i32,
    info: &mut i32,
) {
    ffi::chetri2x_(
        &(uplo as c_char),
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        ipiv.as_ptr(),
        work.as_mut_ptr() as *mut _,
        &nb,
        info,
    )
}

#[inline]
pub unsafe fn zhetri2x(
    uplo: u8,
    n: i32,
    a: &mut [c64],
    lda: i32,
    ipiv: &[i32],
    work: &mut [c64],
    nb: i32,
    info: &mut i32,
) {
    ffi::zhetri2x_(
        &(uplo as c_char),
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        ipiv.as_ptr(),
        work.as_mut_ptr() as *mut _,
        &nb,
        info,
    )
}

#[inline]
pub unsafe fn chetri_3(
    uplo: u8,
    n: i32,
    a: &mut [c32],
    lda: i32,
    e: &[c32],
    ipiv: &[i32],
    work: &mut [c32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::chetri_3_(
        &(uplo as c_char),
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        e.as_ptr() as *const _,
        ipiv.as_ptr(),
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn zhetri_3(
    uplo: u8,
    n: i32,
    a: &mut [c64],
    lda: i32,
    e: &[c64],
    ipiv: &[i32],
    work: &mut [c64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::zhetri_3_(
        &(uplo as c_char),
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        e.as_ptr() as *const _,
        ipiv.as_ptr(),
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn chetrs(
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &[c32],
    lda: i32,
    ipiv: &[i32],
    b: &mut [c32],
    ldb: i32,
    info: &mut i32,
) {
    ffi::chetrs_(
        &(uplo as c_char),
        &n,
        &nrhs,
        a.as_ptr() as *const _,
        &lda,
        ipiv.as_ptr(),
        b.as_mut_ptr() as *mut _,
        &ldb,
        info,
    )
}

#[inline]
pub unsafe fn zhetrs(
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &[c64],
    lda: i32,
    ipiv: &[i32],
    b: &mut [c64],
    ldb: i32,
    info: &mut i32,
) {
    ffi::zhetrs_(
        &(uplo as c_char),
        &n,
        &nrhs,
        a.as_ptr() as *const _,
        &lda,
        ipiv.as_ptr(),
        b.as_mut_ptr() as *mut _,
        &ldb,
        info,
    )
}

#[inline]
pub unsafe fn chetrs2(
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &[c32],
    lda: i32,
    ipiv: &[i32],
    b: &mut [c32],
    ldb: i32,
    work: &mut [c32],
    info: &mut i32,
) {
    ffi::chetrs2_(
        &(uplo as c_char),
        &n,
        &nrhs,
        a.as_ptr() as *const _,
        &lda,
        ipiv.as_ptr(),
        b.as_mut_ptr() as *mut _,
        &ldb,
        work.as_mut_ptr() as *mut _,
        info,
    )
}

#[inline]
pub unsafe fn zhetrs2(
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &[c64],
    lda: i32,
    ipiv: &[i32],
    b: &mut [c64],
    ldb: i32,
    work: &mut [c64],
    info: &mut i32,
) {
    ffi::zhetrs2_(
        &(uplo as c_char),
        &n,
        &nrhs,
        a.as_ptr() as *const _,
        &lda,
        ipiv.as_ptr(),
        b.as_mut_ptr() as *mut _,
        &ldb,
        work.as_mut_ptr() as *mut _,
        info,
    )
}

#[inline]
pub unsafe fn chetrs_3(
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &[c32],
    lda: i32,
    e: &[c32],
    ipiv: &[i32],
    b: &mut [c32],
    ldb: i32,
    info: &mut i32,
) {
    ffi::chetrs_3_(
        &(uplo as c_char),
        &n,
        &nrhs,
        a.as_ptr() as *const _,
        &lda,
        e.as_ptr() as *const _,
        ipiv.as_ptr(),
        b.as_mut_ptr() as *mut _,
        &ldb,
        info,
    )
}

#[inline]
pub unsafe fn zhetrs_3(
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &[c64],
    lda: i32,
    e: &[c64],
    ipiv: &[i32],
    b: &mut [c64],
    ldb: i32,
    info: &mut i32,
) {
    ffi::zhetrs_3_(
        &(uplo as c_char),
        &n,
        &nrhs,
        a.as_ptr() as *const _,
        &lda,
        e.as_ptr() as *const _,
        ipiv.as_ptr(),
        b.as_mut_ptr() as *mut _,
        &ldb,
        info,
    )
}

#[inline]
pub unsafe fn chetrs_aa(
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &[c32],
    lda: i32,
    ipiv: &[i32],
    b: &mut [c32],
    ldb: i32,
    work: &mut [c32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::chetrs_aa_(
        &(uplo as c_char),
        &n,
        &nrhs,
        a.as_ptr() as *const _,
        &lda,
        ipiv.as_ptr(),
        b.as_mut_ptr() as *mut _,
        &ldb,
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn zhetrs_aa(
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &[c64],
    lda: i32,
    ipiv: &[i32],
    b: &mut [c64],
    ldb: i32,
    work: &mut [c64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::zhetrs_aa_(
        &(uplo as c_char),
        &n,
        &nrhs,
        a.as_ptr() as *const _,
        &lda,
        ipiv.as_ptr(),
        b.as_mut_ptr() as *mut _,
        &ldb,
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn chetrs_aa_2stage(
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &[c32],
    lda: i32,
    tb: &mut [c32],
    ltb: &[i32],
    ipiv: &[i32],
    ipiv2: &[i32],
    b: &mut [c32],
    ldb: i32,
    info: &mut i32,
) {
    ffi::chetrs_aa_2stage_(
        &(uplo as c_char),
        &n,
        &nrhs,
        a.as_ptr() as *const _,
        &lda,
        tb.as_mut_ptr() as *mut _,
        ltb.as_ptr(),
        ipiv.as_ptr(),
        ipiv2.as_ptr(),
        b.as_mut_ptr() as *mut _,
        &ldb,
        info,
    )
}

#[inline]
pub unsafe fn zhetrs_aa_2stage(
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &[c64],
    lda: i32,
    tb: &mut [c64],
    ltb: &[i32],
    ipiv: &[i32],
    ipiv2: &[i32],
    b: &mut [c64],
    ldb: i32,
    info: &mut i32,
) {
    ffi::zhetrs_aa_2stage_(
        &(uplo as c_char),
        &n,
        &nrhs,
        a.as_ptr() as *const _,
        &lda,
        tb.as_mut_ptr() as *mut _,
        ltb.as_ptr(),
        ipiv.as_ptr(),
        ipiv2.as_ptr(),
        b.as_mut_ptr() as *mut _,
        &ldb,
        info,
    )
}

#[inline]
pub unsafe fn chetrs_rook(
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &[c32],
    lda: i32,
    ipiv: &[i32],
    b: &mut [c32],
    ldb: i32,
    info: &mut i32,
) {
    ffi::chetrs_rook_(
        &(uplo as c_char),
        &n,
        &nrhs,
        a.as_ptr() as *const _,
        &lda,
        ipiv.as_ptr(),
        b.as_mut_ptr() as *mut _,
        &ldb,
        info,
    )
}

#[inline]
pub unsafe fn zhetrs_rook(
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &[c64],
    lda: i32,
    ipiv: &[i32],
    b: &mut [c64],
    ldb: i32,
    info: &mut i32,
) {
    ffi::zhetrs_rook_(
        &(uplo as c_char),
        &n,
        &nrhs,
        a.as_ptr() as *const _,
        &lda,
        ipiv.as_ptr(),
        b.as_mut_ptr() as *mut _,
        &ldb,
        info,
    )
}

#[inline]
pub unsafe fn chfrk(
    transr: u8,
    uplo: u8,
    trans: u8,
    n: i32,
    k: i32,
    alpha: &[f32],
    a: &[c32],
    lda: i32,
    beta: &[f32],
    c: &mut [c32],
) {
    ffi::chfrk_(
        &(transr as c_char),
        &(uplo as c_char),
        &(trans as c_char),
        &n,
        &k,
        alpha.as_ptr(),
        a.as_ptr() as *const _,
        &lda,
        beta.as_ptr(),
        c.as_mut_ptr() as *mut _,
    )
}

#[inline]
pub unsafe fn zhfrk(
    transr: u8,
    uplo: u8,
    trans: u8,
    n: i32,
    k: i32,
    alpha: &[f64],
    a: &[c64],
    lda: i32,
    beta: &[f64],
    c: &mut [c64],
) {
    ffi::zhfrk_(
        &(transr as c_char),
        &(uplo as c_char),
        &(trans as c_char),
        &n,
        &k,
        alpha.as_ptr(),
        a.as_ptr() as *const _,
        &lda,
        beta.as_ptr(),
        c.as_mut_ptr() as *mut _,
    )
}

#[inline]
pub unsafe fn chgeqz(
    job: u8,
    compq: u8,
    compz: u8,
    n: i32,
    ilo: i32,
    ihi: i32,
    h: &mut [c32],
    ldh: i32,
    t: &mut [c32],
    ldt: i32,
    alpha: &mut [c32],
    beta: &mut [c32],
    q: &mut [c32],
    ldq: i32,
    z: &mut [c32],
    ldz: i32,
    work: &mut [c32],
    lwork: i32,
    rwork: &mut [f32],
    info: &mut i32,
) {
    ffi::chgeqz_(
        &(job as c_char),
        &(compq as c_char),
        &(compz as c_char),
        &n,
        &ilo,
        &ihi,
        h.as_mut_ptr() as *mut _,
        &ldh,
        t.as_mut_ptr() as *mut _,
        &ldt,
        alpha.as_mut_ptr() as *mut _,
        beta.as_mut_ptr() as *mut _,
        q.as_mut_ptr() as *mut _,
        &ldq,
        z.as_mut_ptr() as *mut _,
        &ldz,
        work.as_mut_ptr() as *mut _,
        &lwork,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn dhgeqz(
    job: u8,
    compq: u8,
    compz: u8,
    n: i32,
    ilo: i32,
    ihi: i32,
    h: &mut [f64],
    ldh: i32,
    t: &mut [f64],
    ldt: i32,
    alphar: &mut [f64],
    alphai: &mut [f64],
    beta: &mut [f64],
    q: &mut [f64],
    ldq: i32,
    z: &mut [f64],
    ldz: i32,
    work: &mut [f64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::dhgeqz_(
        &(job as c_char),
        &(compq as c_char),
        &(compz as c_char),
        &n,
        &ilo,
        &ihi,
        h.as_mut_ptr(),
        &ldh,
        t.as_mut_ptr(),
        &ldt,
        alphar.as_mut_ptr(),
        alphai.as_mut_ptr(),
        beta.as_mut_ptr(),
        q.as_mut_ptr(),
        &ldq,
        z.as_mut_ptr(),
        &ldz,
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn shgeqz(
    job: u8,
    compq: u8,
    compz: u8,
    n: i32,
    ilo: i32,
    ihi: i32,
    h: &mut [f32],
    ldh: i32,
    t: &mut [f32],
    ldt: i32,
    alphar: &mut [f32],
    alphai: &mut [f32],
    beta: &mut [f32],
    q: &mut [f32],
    ldq: i32,
    z: &mut [f32],
    ldz: i32,
    work: &mut [f32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::shgeqz_(
        &(job as c_char),
        &(compq as c_char),
        &(compz as c_char),
        &n,
        &ilo,
        &ihi,
        h.as_mut_ptr(),
        &ldh,
        t.as_mut_ptr(),
        &ldt,
        alphar.as_mut_ptr(),
        alphai.as_mut_ptr(),
        beta.as_mut_ptr(),
        q.as_mut_ptr(),
        &ldq,
        z.as_mut_ptr(),
        &ldz,
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn zhgeqz(
    job: u8,
    compq: u8,
    compz: u8,
    n: i32,
    ilo: i32,
    ihi: i32,
    h: &mut [c64],
    ldh: i32,
    t: &mut [c64],
    ldt: i32,
    alpha: &mut [c64],
    beta: &mut [c64],
    q: &mut [c64],
    ldq: i32,
    z: &mut [c64],
    ldz: i32,
    work: &mut [c64],
    lwork: i32,
    rwork: &mut [f64],
    info: &mut i32,
) {
    ffi::zhgeqz_(
        &(job as c_char),
        &(compq as c_char),
        &(compz as c_char),
        &n,
        &ilo,
        &ihi,
        h.as_mut_ptr() as *mut _,
        &ldh,
        t.as_mut_ptr() as *mut _,
        &ldt,
        alpha.as_mut_ptr() as *mut _,
        beta.as_mut_ptr() as *mut _,
        q.as_mut_ptr() as *mut _,
        &ldq,
        z.as_mut_ptr() as *mut _,
        &ldz,
        work.as_mut_ptr() as *mut _,
        &lwork,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn chpcon(
    uplo: u8,
    n: i32,
    ap: &[c32],
    ipiv: &[i32],
    anorm: f32,
    rcond: &mut f32,
    work: &mut [c32],
    info: &mut i32,
) {
    ffi::chpcon_(
        &(uplo as c_char),
        &n,
        ap.as_ptr() as *const _,
        ipiv.as_ptr(),
        &anorm,
        rcond,
        work.as_mut_ptr() as *mut _,
        info,
    )
}

#[inline]
pub unsafe fn zhpcon(
    uplo: u8,
    n: i32,
    ap: &[c64],
    ipiv: &[i32],
    anorm: f64,
    rcond: &mut f64,
    work: &mut [c64],
    info: &mut i32,
) {
    ffi::zhpcon_(
        &(uplo as c_char),
        &n,
        ap.as_ptr() as *const _,
        ipiv.as_ptr(),
        &anorm,
        rcond,
        work.as_mut_ptr() as *mut _,
        info,
    )
}

#[inline]
pub unsafe fn chpev(
    jobz: u8,
    uplo: u8,
    n: i32,
    ap: &mut [c32],
    w: &mut [f32],
    z: &mut [c32],
    ldz: i32,
    work: &mut [c32],
    rwork: &mut [f32],
    info: &mut i32,
) {
    ffi::chpev_(
        &(jobz as c_char),
        &(uplo as c_char),
        &n,
        ap.as_mut_ptr() as *mut _,
        w.as_mut_ptr(),
        z.as_mut_ptr() as *mut _,
        &ldz,
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn zhpev(
    jobz: u8,
    uplo: u8,
    n: i32,
    ap: &mut [c64],
    w: &mut [f64],
    z: &mut [c64],
    ldz: i32,
    work: &mut [c64],
    rwork: &mut [f64],
    info: &mut i32,
) {
    ffi::zhpev_(
        &(jobz as c_char),
        &(uplo as c_char),
        &n,
        ap.as_mut_ptr() as *mut _,
        w.as_mut_ptr(),
        z.as_mut_ptr() as *mut _,
        &ldz,
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn chpevd(
    jobz: u8,
    uplo: u8,
    n: i32,
    ap: &mut [c32],
    w: &mut [f32],
    z: &mut [c32],
    ldz: i32,
    work: &mut [c32],
    lwork: i32,
    rwork: &mut [f32],
    lrwork: i32,
    iwork: &mut [i32],
    liwork: i32,
    info: &mut i32,
) {
    ffi::chpevd_(
        &(jobz as c_char),
        &(uplo as c_char),
        &n,
        ap.as_mut_ptr() as *mut _,
        w.as_mut_ptr(),
        z.as_mut_ptr() as *mut _,
        &ldz,
        work.as_mut_ptr() as *mut _,
        &lwork,
        rwork.as_mut_ptr(),
        &lrwork,
        iwork.as_mut_ptr(),
        &liwork,
        info,
    )
}

#[inline]
pub unsafe fn zhpevd(
    jobz: u8,
    uplo: u8,
    n: i32,
    ap: &mut [c64],
    w: &mut [f64],
    z: &mut [c64],
    ldz: i32,
    work: &mut [c64],
    lwork: i32,
    rwork: &mut [f64],
    lrwork: i32,
    iwork: &mut [i32],
    liwork: i32,
    info: &mut i32,
) {
    ffi::zhpevd_(
        &(jobz as c_char),
        &(uplo as c_char),
        &n,
        ap.as_mut_ptr() as *mut _,
        w.as_mut_ptr(),
        z.as_mut_ptr() as *mut _,
        &ldz,
        work.as_mut_ptr() as *mut _,
        &lwork,
        rwork.as_mut_ptr(),
        &lrwork,
        iwork.as_mut_ptr(),
        &liwork,
        info,
    )
}

#[inline]
pub unsafe fn chpevx(
    jobz: u8,
    range: u8,
    uplo: u8,
    n: i32,
    ap: &mut [c32],
    vl: f32,
    vu: f32,
    il: i32,
    iu: i32,
    abstol: f32,
    m: &mut i32,
    w: &mut [f32],
    z: &mut [c32],
    ldz: i32,
    work: &mut [c32],
    rwork: &mut [f32],
    iwork: &mut [i32],
    ifail: &mut [i32],
    info: &mut i32,
) {
    ffi::chpevx_(
        &(jobz as c_char),
        &(range as c_char),
        &(uplo as c_char),
        &n,
        ap.as_mut_ptr() as *mut _,
        &vl,
        &vu,
        &il,
        &iu,
        &abstol,
        m,
        w.as_mut_ptr(),
        z.as_mut_ptr() as *mut _,
        &ldz,
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
        iwork.as_mut_ptr(),
        ifail.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn zhpevx(
    jobz: u8,
    range: u8,
    uplo: u8,
    n: i32,
    ap: &mut [c64],
    vl: f64,
    vu: f64,
    il: i32,
    iu: i32,
    abstol: f64,
    m: &mut i32,
    w: &mut [f64],
    z: &mut [c64],
    ldz: i32,
    work: &mut [c64],
    rwork: &mut [f64],
    iwork: &mut [i32],
    ifail: &mut [i32],
    info: &mut i32,
) {
    ffi::zhpevx_(
        &(jobz as c_char),
        &(range as c_char),
        &(uplo as c_char),
        &n,
        ap.as_mut_ptr() as *mut _,
        &vl,
        &vu,
        &il,
        &iu,
        &abstol,
        m,
        w.as_mut_ptr(),
        z.as_mut_ptr() as *mut _,
        &ldz,
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
        iwork.as_mut_ptr(),
        ifail.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn chpgst(itype: &[i32], uplo: u8, n: i32, ap: &mut [c32], bp: &[c32], info: &mut i32) {
    ffi::chpgst_(
        itype.as_ptr(),
        &(uplo as c_char),
        &n,
        ap.as_mut_ptr() as *mut _,
        bp.as_ptr() as *const _,
        info,
    )
}

#[inline]
pub unsafe fn zhpgst(itype: &[i32], uplo: u8, n: i32, ap: &mut [c64], bp: &[c64], info: &mut i32) {
    ffi::zhpgst_(
        itype.as_ptr(),
        &(uplo as c_char),
        &n,
        ap.as_mut_ptr() as *mut _,
        bp.as_ptr() as *const _,
        info,
    )
}

#[inline]
pub unsafe fn chpgv(
    itype: &[i32],
    jobz: u8,
    uplo: u8,
    n: i32,
    ap: &mut [c32],
    bp: &mut [c32],
    w: &mut [f32],
    z: &mut [c32],
    ldz: i32,
    work: &mut [c32],
    rwork: &mut [f32],
    info: &mut i32,
) {
    ffi::chpgv_(
        itype.as_ptr(),
        &(jobz as c_char),
        &(uplo as c_char),
        &n,
        ap.as_mut_ptr() as *mut _,
        bp.as_mut_ptr() as *mut _,
        w.as_mut_ptr(),
        z.as_mut_ptr() as *mut _,
        &ldz,
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn zhpgv(
    itype: &[i32],
    jobz: u8,
    uplo: u8,
    n: i32,
    ap: &mut [c64],
    bp: &mut [c64],
    w: &mut [f64],
    z: &mut [c64],
    ldz: i32,
    work: &mut [c64],
    rwork: &mut [f64],
    info: &mut i32,
) {
    ffi::zhpgv_(
        itype.as_ptr(),
        &(jobz as c_char),
        &(uplo as c_char),
        &n,
        ap.as_mut_ptr() as *mut _,
        bp.as_mut_ptr() as *mut _,
        w.as_mut_ptr(),
        z.as_mut_ptr() as *mut _,
        &ldz,
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn chpgvd(
    itype: &[i32],
    jobz: u8,
    uplo: u8,
    n: i32,
    ap: &mut [c32],
    bp: &mut [c32],
    w: &mut [f32],
    z: &mut [c32],
    ldz: i32,
    work: &mut [c32],
    lwork: i32,
    rwork: &mut [f32],
    lrwork: i32,
    iwork: &mut [i32],
    liwork: i32,
    info: &mut i32,
) {
    ffi::chpgvd_(
        itype.as_ptr(),
        &(jobz as c_char),
        &(uplo as c_char),
        &n,
        ap.as_mut_ptr() as *mut _,
        bp.as_mut_ptr() as *mut _,
        w.as_mut_ptr(),
        z.as_mut_ptr() as *mut _,
        &ldz,
        work.as_mut_ptr() as *mut _,
        &lwork,
        rwork.as_mut_ptr(),
        &lrwork,
        iwork.as_mut_ptr(),
        &liwork,
        info,
    )
}

#[inline]
pub unsafe fn zhpgvd(
    itype: &[i32],
    jobz: u8,
    uplo: u8,
    n: i32,
    ap: &mut [c64],
    bp: &mut [c64],
    w: &mut [f64],
    z: &mut [c64],
    ldz: i32,
    work: &mut [c64],
    lwork: i32,
    rwork: &mut [f64],
    lrwork: i32,
    iwork: &mut [i32],
    liwork: i32,
    info: &mut i32,
) {
    ffi::zhpgvd_(
        itype.as_ptr(),
        &(jobz as c_char),
        &(uplo as c_char),
        &n,
        ap.as_mut_ptr() as *mut _,
        bp.as_mut_ptr() as *mut _,
        w.as_mut_ptr(),
        z.as_mut_ptr() as *mut _,
        &ldz,
        work.as_mut_ptr() as *mut _,
        &lwork,
        rwork.as_mut_ptr(),
        &lrwork,
        iwork.as_mut_ptr(),
        &liwork,
        info,
    )
}

#[inline]
pub unsafe fn chpgvx(
    itype: &[i32],
    jobz: u8,
    range: u8,
    uplo: u8,
    n: i32,
    ap: &mut [c32],
    bp: &mut [c32],
    vl: f32,
    vu: f32,
    il: i32,
    iu: i32,
    abstol: f32,
    m: &mut i32,
    w: &mut [f32],
    z: &mut [c32],
    ldz: i32,
    work: &mut [c32],
    rwork: &mut [f32],
    iwork: &mut [i32],
    ifail: &mut [i32],
    info: &mut i32,
) {
    ffi::chpgvx_(
        itype.as_ptr(),
        &(jobz as c_char),
        &(range as c_char),
        &(uplo as c_char),
        &n,
        ap.as_mut_ptr() as *mut _,
        bp.as_mut_ptr() as *mut _,
        &vl,
        &vu,
        &il,
        &iu,
        &abstol,
        m,
        w.as_mut_ptr(),
        z.as_mut_ptr() as *mut _,
        &ldz,
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
        iwork.as_mut_ptr(),
        ifail.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn zhpgvx(
    itype: &[i32],
    jobz: u8,
    range: u8,
    uplo: u8,
    n: i32,
    ap: &mut [c64],
    bp: &mut [c64],
    vl: f64,
    vu: f64,
    il: i32,
    iu: i32,
    abstol: f64,
    m: &mut i32,
    w: &mut [f64],
    z: &mut [c64],
    ldz: i32,
    work: &mut [c64],
    rwork: &mut [f64],
    iwork: &mut [i32],
    ifail: &mut [i32],
    info: &mut i32,
) {
    ffi::zhpgvx_(
        itype.as_ptr(),
        &(jobz as c_char),
        &(range as c_char),
        &(uplo as c_char),
        &n,
        ap.as_mut_ptr() as *mut _,
        bp.as_mut_ptr() as *mut _,
        &vl,
        &vu,
        &il,
        &iu,
        &abstol,
        m,
        w.as_mut_ptr(),
        z.as_mut_ptr() as *mut _,
        &ldz,
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
        iwork.as_mut_ptr(),
        ifail.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn chprfs(
    uplo: u8,
    n: i32,
    nrhs: i32,
    ap: &[c32],
    afp: &[c32],
    ipiv: &[i32],
    b: &[c32],
    ldb: i32,
    x: &mut [c32],
    ldx: i32,
    ferr: &mut [f32],
    berr: &mut [f32],
    work: &mut [c32],
    rwork: &mut [f32],
    info: &mut i32,
) {
    ffi::chprfs_(
        &(uplo as c_char),
        &n,
        &nrhs,
        ap.as_ptr() as *const _,
        afp.as_ptr() as *const _,
        ipiv.as_ptr(),
        b.as_ptr() as *const _,
        &ldb,
        x.as_mut_ptr() as *mut _,
        &ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn zhprfs(
    uplo: u8,
    n: i32,
    nrhs: i32,
    ap: &[c64],
    afp: &[c64],
    ipiv: &[i32],
    b: &[c64],
    ldb: i32,
    x: &mut [c64],
    ldx: i32,
    ferr: &mut [f64],
    berr: &mut [f64],
    work: &mut [c64],
    rwork: &mut [f64],
    info: &mut i32,
) {
    ffi::zhprfs_(
        &(uplo as c_char),
        &n,
        &nrhs,
        ap.as_ptr() as *const _,
        afp.as_ptr() as *const _,
        ipiv.as_ptr(),
        b.as_ptr() as *const _,
        &ldb,
        x.as_mut_ptr() as *mut _,
        &ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn chpsv(
    uplo: u8,
    n: i32,
    nrhs: i32,
    ap: &mut [c32],
    ipiv: &mut [i32],
    b: &mut [c32],
    ldb: i32,
    info: &mut i32,
) {
    ffi::chpsv_(
        &(uplo as c_char),
        &n,
        &nrhs,
        ap.as_mut_ptr() as *mut _,
        ipiv.as_mut_ptr(),
        b.as_mut_ptr() as *mut _,
        &ldb,
        info,
    )
}

#[inline]
pub unsafe fn zhpsv(
    uplo: u8,
    n: i32,
    nrhs: i32,
    ap: &mut [c64],
    ipiv: &mut [i32],
    b: &mut [c64],
    ldb: i32,
    info: &mut i32,
) {
    ffi::zhpsv_(
        &(uplo as c_char),
        &n,
        &nrhs,
        ap.as_mut_ptr() as *mut _,
        ipiv.as_mut_ptr(),
        b.as_mut_ptr() as *mut _,
        &ldb,
        info,
    )
}

#[inline]
pub unsafe fn chpsvx(
    fact: u8,
    uplo: u8,
    n: i32,
    nrhs: i32,
    ap: &[c32],
    afp: &mut [c32],
    ipiv: &mut [i32],
    b: &[c32],
    ldb: i32,
    x: &mut [c32],
    ldx: i32,
    rcond: &mut f32,
    ferr: &mut [f32],
    berr: &mut [f32],
    work: &mut [c32],
    rwork: &mut [f32],
    info: &mut i32,
) {
    ffi::chpsvx_(
        &(fact as c_char),
        &(uplo as c_char),
        &n,
        &nrhs,
        ap.as_ptr() as *const _,
        afp.as_mut_ptr() as *mut _,
        ipiv.as_mut_ptr(),
        b.as_ptr() as *const _,
        &ldb,
        x.as_mut_ptr() as *mut _,
        &ldx,
        rcond,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn zhpsvx(
    fact: u8,
    uplo: u8,
    n: i32,
    nrhs: i32,
    ap: &[c64],
    afp: &mut [c64],
    ipiv: &mut [i32],
    b: &[c64],
    ldb: i32,
    x: &mut [c64],
    ldx: i32,
    rcond: &mut f64,
    ferr: &mut [f64],
    berr: &mut [f64],
    work: &mut [c64],
    rwork: &mut [f64],
    info: &mut i32,
) {
    ffi::zhpsvx_(
        &(fact as c_char),
        &(uplo as c_char),
        &n,
        &nrhs,
        ap.as_ptr() as *const _,
        afp.as_mut_ptr() as *mut _,
        ipiv.as_mut_ptr(),
        b.as_ptr() as *const _,
        &ldb,
        x.as_mut_ptr() as *mut _,
        &ldx,
        rcond,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn chptrd(
    uplo: u8,
    n: i32,
    ap: &mut [c32],
    d: &mut [f32],
    e: &mut [f32],
    tau: &mut [c32],
    info: &mut i32,
) {
    ffi::chptrd_(
        &(uplo as c_char),
        &n,
        ap.as_mut_ptr() as *mut _,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        tau.as_mut_ptr() as *mut _,
        info,
    )
}

#[inline]
pub unsafe fn zhptrd(
    uplo: u8,
    n: i32,
    ap: &mut [c64],
    d: &mut [f64],
    e: &mut [f64],
    tau: &mut [c64],
    info: &mut i32,
) {
    ffi::zhptrd_(
        &(uplo as c_char),
        &n,
        ap.as_mut_ptr() as *mut _,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        tau.as_mut_ptr() as *mut _,
        info,
    )
}

#[inline]
pub unsafe fn chptrf(uplo: u8, n: i32, ap: &mut [c32], ipiv: &mut [i32], info: &mut i32) {
    ffi::chptrf_(
        &(uplo as c_char),
        &n,
        ap.as_mut_ptr() as *mut _,
        ipiv.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn zhptrf(uplo: u8, n: i32, ap: &mut [c64], ipiv: &mut [i32], info: &mut i32) {
    ffi::zhptrf_(
        &(uplo as c_char),
        &n,
        ap.as_mut_ptr() as *mut _,
        ipiv.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn chptri(
    uplo: u8,
    n: i32,
    ap: &mut [c32],
    ipiv: &[i32],
    work: &mut [c32],
    info: &mut i32,
) {
    ffi::chptri_(
        &(uplo as c_char),
        &n,
        ap.as_mut_ptr() as *mut _,
        ipiv.as_ptr(),
        work.as_mut_ptr() as *mut _,
        info,
    )
}

#[inline]
pub unsafe fn zhptri(
    uplo: u8,
    n: i32,
    ap: &mut [c64],
    ipiv: &[i32],
    work: &mut [c64],
    info: &mut i32,
) {
    ffi::zhptri_(
        &(uplo as c_char),
        &n,
        ap.as_mut_ptr() as *mut _,
        ipiv.as_ptr(),
        work.as_mut_ptr() as *mut _,
        info,
    )
}

#[inline]
pub unsafe fn chptrs(
    uplo: u8,
    n: i32,
    nrhs: i32,
    ap: &[c32],
    ipiv: &[i32],
    b: &mut [c32],
    ldb: i32,
    info: &mut i32,
) {
    ffi::chptrs_(
        &(uplo as c_char),
        &n,
        &nrhs,
        ap.as_ptr() as *const _,
        ipiv.as_ptr(),
        b.as_mut_ptr() as *mut _,
        &ldb,
        info,
    )
}

#[inline]
pub unsafe fn zhptrs(
    uplo: u8,
    n: i32,
    nrhs: i32,
    ap: &[c64],
    ipiv: &[i32],
    b: &mut [c64],
    ldb: i32,
    info: &mut i32,
) {
    ffi::zhptrs_(
        &(uplo as c_char),
        &n,
        &nrhs,
        ap.as_ptr() as *const _,
        ipiv.as_ptr(),
        b.as_mut_ptr() as *mut _,
        &ldb,
        info,
    )
}

#[inline]
pub unsafe fn chsein(
    side: u8,
    eigsrc: u8,
    initv: u8,
    select: &[i32],
    n: i32,
    h: &[c32],
    ldh: i32,
    w: &mut [c32],
    vl: &mut [c32],
    ldvl: i32,
    vr: &mut [c32],
    ldvr: i32,
    mm: i32,
    m: &mut i32,
    work: &mut [c32],
    rwork: &mut [f32],
    ifaill: &mut [i32],
    ifailr: &mut [i32],
    info: &mut i32,
) {
    ffi::chsein_(
        &(side as c_char),
        &(eigsrc as c_char),
        &(initv as c_char),
        select.as_ptr(),
        &n,
        h.as_ptr() as *const _,
        &ldh,
        w.as_mut_ptr() as *mut _,
        vl.as_mut_ptr() as *mut _,
        &ldvl,
        vr.as_mut_ptr() as *mut _,
        &ldvr,
        &mm,
        m,
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
        ifaill.as_mut_ptr(),
        ifailr.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn dhsein(
    side: u8,
    eigsrc: u8,
    initv: u8,
    select: &mut [i32],
    n: i32,
    h: &[f64],
    ldh: i32,
    wr: &mut [f64],
    wi: &[f64],
    vl: &mut [f64],
    ldvl: i32,
    vr: &mut [f64],
    ldvr: i32,
    mm: i32,
    m: &mut i32,
    work: &mut [f64],
    ifaill: &mut [i32],
    ifailr: &mut [i32],
    info: &mut i32,
) {
    ffi::dhsein_(
        &(side as c_char),
        &(eigsrc as c_char),
        &(initv as c_char),
        select.as_mut_ptr(),
        &n,
        h.as_ptr(),
        &ldh,
        wr.as_mut_ptr(),
        wi.as_ptr(),
        vl.as_mut_ptr(),
        &ldvl,
        vr.as_mut_ptr(),
        &ldvr,
        &mm,
        m,
        work.as_mut_ptr(),
        ifaill.as_mut_ptr(),
        ifailr.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn shsein(
    side: u8,
    eigsrc: u8,
    initv: u8,
    select: &mut [i32],
    n: i32,
    h: &[f32],
    ldh: i32,
    wr: &mut [f32],
    wi: &[f32],
    vl: &mut [f32],
    ldvl: i32,
    vr: &mut [f32],
    ldvr: i32,
    mm: i32,
    m: &mut i32,
    work: &mut [f32],
    ifaill: &mut [i32],
    ifailr: &mut [i32],
    info: &mut i32,
) {
    ffi::shsein_(
        &(side as c_char),
        &(eigsrc as c_char),
        &(initv as c_char),
        select.as_mut_ptr(),
        &n,
        h.as_ptr(),
        &ldh,
        wr.as_mut_ptr(),
        wi.as_ptr(),
        vl.as_mut_ptr(),
        &ldvl,
        vr.as_mut_ptr(),
        &ldvr,
        &mm,
        m,
        work.as_mut_ptr(),
        ifaill.as_mut_ptr(),
        ifailr.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn zhsein(
    side: u8,
    eigsrc: u8,
    initv: u8,
    select: &[i32],
    n: i32,
    h: &[c64],
    ldh: i32,
    w: &mut [c64],
    vl: &mut [c64],
    ldvl: i32,
    vr: &mut [c64],
    ldvr: i32,
    mm: i32,
    m: &mut i32,
    work: &mut [c64],
    rwork: &mut [f64],
    ifaill: &mut [i32],
    ifailr: &mut [i32],
    info: &mut i32,
) {
    ffi::zhsein_(
        &(side as c_char),
        &(eigsrc as c_char),
        &(initv as c_char),
        select.as_ptr(),
        &n,
        h.as_ptr() as *const _,
        &ldh,
        w.as_mut_ptr() as *mut _,
        vl.as_mut_ptr() as *mut _,
        &ldvl,
        vr.as_mut_ptr() as *mut _,
        &ldvr,
        &mm,
        m,
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
        ifaill.as_mut_ptr(),
        ifailr.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn chseqr(
    job: u8,
    compz: u8,
    n: i32,
    ilo: i32,
    ihi: i32,
    h: &mut [c32],
    ldh: i32,
    w: &mut [c32],
    z: &mut [c32],
    ldz: i32,
    work: &mut [c32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::chseqr_(
        &(job as c_char),
        &(compz as c_char),
        &n,
        &ilo,
        &ihi,
        h.as_mut_ptr() as *mut _,
        &ldh,
        w.as_mut_ptr() as *mut _,
        z.as_mut_ptr() as *mut _,
        &ldz,
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn dhseqr(
    job: u8,
    compz: u8,
    n: i32,
    ilo: i32,
    ihi: i32,
    h: &mut [f64],
    ldh: i32,
    wr: &mut [f64],
    wi: &mut [f64],
    z: &mut [f64],
    ldz: i32,
    work: &mut [f64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::dhseqr_(
        &(job as c_char),
        &(compz as c_char),
        &n,
        &ilo,
        &ihi,
        h.as_mut_ptr(),
        &ldh,
        wr.as_mut_ptr(),
        wi.as_mut_ptr(),
        z.as_mut_ptr(),
        &ldz,
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn shseqr(
    job: u8,
    compz: u8,
    n: i32,
    ilo: i32,
    ihi: i32,
    h: &mut [f32],
    ldh: i32,
    wr: &mut [f32],
    wi: &mut [f32],
    z: &mut [f32],
    ldz: i32,
    work: &mut [f32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::shseqr_(
        &(job as c_char),
        &(compz as c_char),
        &n,
        &ilo,
        &ihi,
        h.as_mut_ptr(),
        &ldh,
        wr.as_mut_ptr(),
        wi.as_mut_ptr(),
        z.as_mut_ptr(),
        &ldz,
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn zhseqr(
    job: u8,
    compz: u8,
    n: i32,
    ilo: i32,
    ihi: i32,
    h: &mut [c64],
    ldh: i32,
    w: &mut [c64],
    z: &mut [c64],
    ldz: i32,
    work: &mut [c64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::zhseqr_(
        &(job as c_char),
        &(compz as c_char),
        &n,
        &ilo,
        &ihi,
        h.as_mut_ptr() as *mut _,
        &ldh,
        w.as_mut_ptr() as *mut _,
        z.as_mut_ptr() as *mut _,
        &ldz,
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn clacgv(n: i32, x: &mut [c32], incx: i32) {
    ffi::clacgv_(&n, x.as_mut_ptr() as *mut _, &incx)
}

#[inline]
pub unsafe fn zlacgv(n: i32, x: &mut [c64], incx: i32) {
    ffi::zlacgv_(&n, x.as_mut_ptr() as *mut _, &incx)
}

#[inline]
pub unsafe fn clacn2(
    n: i32,
    v: &mut [c32],
    x: &mut [c32],
    est: &mut [f32],
    kase: &mut i32,
    isave: &mut [i32],
) {
    ffi::clacn2_(
        &n,
        v.as_mut_ptr() as *mut _,
        x.as_mut_ptr() as *mut _,
        est.as_mut_ptr(),
        kase,
        isave.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dlacn2(
    n: i32,
    v: &mut [f64],
    x: &mut [f64],
    isgn: &mut [i32],
    est: &mut [f64],
    kase: &mut i32,
    isave: &mut [i32],
) {
    ffi::dlacn2_(
        &n,
        v.as_mut_ptr(),
        x.as_mut_ptr(),
        isgn.as_mut_ptr(),
        est.as_mut_ptr(),
        kase,
        isave.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn slacn2(
    n: i32,
    v: &mut [f32],
    x: &mut [f32],
    isgn: &mut [i32],
    est: &mut [f32],
    kase: &mut i32,
    isave: &mut [i32],
) {
    ffi::slacn2_(
        &n,
        v.as_mut_ptr(),
        x.as_mut_ptr(),
        isgn.as_mut_ptr(),
        est.as_mut_ptr(),
        kase,
        isave.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zlacn2(
    n: i32,
    v: &mut [c64],
    x: &mut [c64],
    est: &mut [f64],
    kase: &mut i32,
    isave: &mut [i32],
) {
    ffi::zlacn2_(
        &n,
        v.as_mut_ptr() as *mut _,
        x.as_mut_ptr() as *mut _,
        est.as_mut_ptr(),
        kase,
        isave.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn clacp2(uplo: u8, m: i32, n: i32, a: &[f32], lda: i32, b: &mut [c32], ldb: i32) {
    ffi::clacp2_(
        &(uplo as c_char),
        &m,
        &n,
        a.as_ptr(),
        &lda,
        b.as_mut_ptr() as *mut _,
        &ldb,
    )
}

#[inline]
pub unsafe fn zlacp2(uplo: u8, m: i32, n: i32, a: &[f64], lda: i32, b: &mut [c64], ldb: i32) {
    ffi::zlacp2_(
        &(uplo as c_char),
        &m,
        &n,
        a.as_ptr(),
        &lda,
        b.as_mut_ptr() as *mut _,
        &ldb,
    )
}

#[inline]
pub unsafe fn clacpy(uplo: u8, m: i32, n: i32, a: &[c32], lda: i32, b: &mut [c32], ldb: i32) {
    ffi::clacpy_(
        &(uplo as c_char),
        &m,
        &n,
        a.as_ptr() as *const _,
        &lda,
        b.as_mut_ptr() as *mut _,
        &ldb,
    )
}

#[inline]
pub unsafe fn dlacpy(uplo: u8, m: i32, n: i32, a: &[f64], lda: i32, b: &mut [f64], ldb: i32) {
    ffi::dlacpy_(
        &(uplo as c_char),
        &m,
        &n,
        a.as_ptr(),
        &lda,
        b.as_mut_ptr(),
        &ldb,
    )
}

#[inline]
pub unsafe fn slacpy(uplo: u8, m: i32, n: i32, a: &[f32], lda: i32, b: &mut [f32], ldb: i32) {
    ffi::slacpy_(
        &(uplo as c_char),
        &m,
        &n,
        a.as_ptr(),
        &lda,
        b.as_mut_ptr(),
        &ldb,
    )
}

#[inline]
pub unsafe fn zlacpy(uplo: u8, m: i32, n: i32, a: &[c64], lda: i32, b: &mut [c64], ldb: i32) {
    ffi::zlacpy_(
        &(uplo as c_char),
        &m,
        &n,
        a.as_ptr() as *const _,
        &lda,
        b.as_mut_ptr() as *mut _,
        &ldb,
    )
}

#[inline]
pub unsafe fn clacrm(
    m: i32,
    n: i32,
    a: &[c32],
    lda: i32,
    b: &[f32],
    ldb: i32,
    c: &mut [c32],
    ldc: i32,
    rwork: &mut [f32],
) {
    ffi::clacrm_(
        &m,
        &n,
        a.as_ptr() as *const _,
        &lda,
        b.as_ptr(),
        &ldb,
        c.as_mut_ptr() as *mut _,
        &ldc,
        rwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zlacrm(
    m: i32,
    n: i32,
    a: &[c64],
    lda: i32,
    b: &[f64],
    ldb: i32,
    c: &mut [c64],
    ldc: i32,
    rwork: &mut [f64],
) {
    ffi::zlacrm_(
        &m,
        &n,
        a.as_ptr() as *const _,
        &lda,
        b.as_ptr(),
        &ldb,
        c.as_mut_ptr() as *mut _,
        &ldc,
        rwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zlag2c(
    m: i32,
    n: i32,
    a: &[c64],
    lda: i32,
    sa: &mut [c32],
    ldsa: i32,
    info: &mut i32,
) {
    ffi::zlag2c_(
        &m,
        &n,
        a.as_ptr() as *const _,
        &lda,
        sa.as_mut_ptr() as *mut _,
        &ldsa,
        info,
    )
}

#[inline]
pub unsafe fn slag2d(
    m: i32,
    n: i32,
    sa: &[f32],
    ldsa: i32,
    a: &mut [f64],
    lda: i32,
    info: &mut i32,
) {
    ffi::slag2d_(&m, &n, sa.as_ptr(), &ldsa, a.as_mut_ptr(), &lda, info)
}

#[inline]
pub unsafe fn dlag2s(
    m: i32,
    n: i32,
    a: &[f64],
    lda: i32,
    sa: &mut [f32],
    ldsa: i32,
    info: &mut i32,
) {
    ffi::dlag2s_(&m, &n, a.as_ptr(), &lda, sa.as_mut_ptr(), &ldsa, info)
}

#[inline]
pub unsafe fn clag2z(
    m: i32,
    n: i32,
    sa: &[c32],
    ldsa: i32,
    a: &mut [c64],
    lda: i32,
    info: &mut i32,
) {
    ffi::clag2z_(
        &m,
        &n,
        sa.as_ptr() as *const _,
        &ldsa,
        a.as_mut_ptr() as *mut _,
        &lda,
        info,
    )
}

#[inline]
pub unsafe fn clagge(
    m: i32,
    n: i32,
    kl: i32,
    ku: i32,
    d: &[f32],
    a: &mut [c32],
    lda: i32,
    iseed: &mut [i32],
    work: &mut [c32],
    info: &mut i32,
) {
    ffi::clagge_(
        &m,
        &n,
        &kl,
        &ku,
        d.as_ptr(),
        a.as_mut_ptr() as *mut _,
        &lda,
        iseed.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        info,
    )
}

#[inline]
pub unsafe fn dlagge(
    m: i32,
    n: i32,
    kl: i32,
    ku: i32,
    d: &[f64],
    a: &mut [f64],
    lda: i32,
    iseed: &mut [i32],
    work: &mut [f64],
    info: &mut i32,
) {
    ffi::dlagge_(
        &m,
        &n,
        &kl,
        &ku,
        d.as_ptr(),
        a.as_mut_ptr(),
        &lda,
        iseed.as_mut_ptr(),
        work.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn slagge(
    m: i32,
    n: i32,
    kl: i32,
    ku: i32,
    d: &[f32],
    a: &mut [f32],
    lda: i32,
    iseed: &mut [i32],
    work: &mut [f32],
    info: &mut i32,
) {
    ffi::slagge_(
        &m,
        &n,
        &kl,
        &ku,
        d.as_ptr(),
        a.as_mut_ptr(),
        &lda,
        iseed.as_mut_ptr(),
        work.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn zlagge(
    m: i32,
    n: i32,
    kl: i32,
    ku: i32,
    d: &[f64],
    a: &mut [c64],
    lda: i32,
    iseed: &mut [i32],
    work: &mut [c64],
    info: &mut i32,
) {
    ffi::zlagge_(
        &m,
        &n,
        &kl,
        &ku,
        d.as_ptr(),
        a.as_mut_ptr() as *mut _,
        &lda,
        iseed.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        info,
    )
}

#[inline]
pub unsafe fn claghe(
    n: i32,
    k: i32,
    d: &[f32],
    a: &mut [c32],
    lda: i32,
    iseed: &mut [i32],
    work: &mut [c32],
    info: &mut i32,
) {
    ffi::claghe_(
        &n,
        &k,
        d.as_ptr(),
        a.as_mut_ptr() as *mut _,
        &lda,
        iseed.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        info,
    )
}

#[inline]
pub unsafe fn zlaghe(
    n: i32,
    k: i32,
    d: &[f64],
    a: &mut [c64],
    lda: i32,
    iseed: &mut [i32],
    work: &mut [c64],
    info: &mut i32,
) {
    ffi::zlaghe_(
        &n,
        &k,
        d.as_ptr(),
        a.as_mut_ptr() as *mut _,
        &lda,
        iseed.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        info,
    )
}

#[inline]
pub unsafe fn clagsy(
    n: i32,
    k: i32,
    d: &[f32],
    a: &mut [c32],
    lda: i32,
    iseed: &mut [i32],
    work: &mut [c32],
    info: &mut i32,
) {
    ffi::clagsy_(
        &n,
        &k,
        d.as_ptr(),
        a.as_mut_ptr() as *mut _,
        &lda,
        iseed.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        info,
    )
}

#[inline]
pub unsafe fn dlagsy(
    n: i32,
    k: i32,
    d: &[f64],
    a: &mut [f64],
    lda: i32,
    iseed: &mut [i32],
    work: &mut [f64],
    info: &mut i32,
) {
    ffi::dlagsy_(
        &n,
        &k,
        d.as_ptr(),
        a.as_mut_ptr(),
        &lda,
        iseed.as_mut_ptr(),
        work.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn slagsy(
    n: i32,
    k: i32,
    d: &[f32],
    a: &mut [f32],
    lda: i32,
    iseed: &mut [i32],
    work: &mut [f32],
    info: &mut i32,
) {
    ffi::slagsy_(
        &n,
        &k,
        d.as_ptr(),
        a.as_mut_ptr(),
        &lda,
        iseed.as_mut_ptr(),
        work.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn zlagsy(
    n: i32,
    k: i32,
    d: &[f64],
    a: &mut [c64],
    lda: i32,
    iseed: &mut [i32],
    work: &mut [c64],
    info: &mut i32,
) {
    ffi::zlagsy_(
        &n,
        &k,
        d.as_ptr(),
        a.as_mut_ptr() as *mut _,
        &lda,
        iseed.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        info,
    )
}

#[inline]
pub unsafe fn dlamch(cmach: u8) -> f64 {
    ffi::dlamch_(&(cmach as c_char))
}

#[inline]
pub unsafe fn slamch(cmach: u8) -> f32 {
    ffi::slamch_(&(cmach as c_char))
}

#[inline]
pub unsafe fn clangb(
    norm: u8,
    n: i32,
    kl: i32,
    ku: i32,
    ab: &[c32],
    ldab: i32,
    work: &mut [f32],
) -> f32 {
    ffi::clangb_(
        &(norm as c_char),
        &n,
        &kl,
        &ku,
        ab.as_ptr() as *const _,
        &ldab,
        work.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dlangb(
    norm: u8,
    n: i32,
    kl: i32,
    ku: i32,
    ab: &[f64],
    ldab: i32,
    work: &mut [f64],
) -> f64 {
    ffi::dlangb_(
        &(norm as c_char),
        &n,
        &kl,
        &ku,
        ab.as_ptr(),
        &ldab,
        work.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn slangb(
    norm: u8,
    n: i32,
    kl: i32,
    ku: i32,
    ab: &[f32],
    ldab: i32,
    work: &mut [f32],
) -> f32 {
    ffi::slangb_(
        &(norm as c_char),
        &n,
        &kl,
        &ku,
        ab.as_ptr(),
        &ldab,
        work.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zlangb(
    norm: u8,
    n: i32,
    kl: i32,
    ku: i32,
    ab: &[c64],
    ldab: i32,
    work: &mut [f64],
) -> f64 {
    ffi::zlangb_(
        &(norm as c_char),
        &n,
        &kl,
        &ku,
        ab.as_ptr() as *const _,
        &ldab,
        work.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn clange(norm: u8, m: i32, n: i32, a: &[c32], lda: i32, work: &mut [f32]) -> f32 {
    ffi::clange_(
        &(norm as c_char),
        &m,
        &n,
        a.as_ptr() as *const _,
        &lda,
        work.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dlange(norm: u8, m: i32, n: i32, a: &[f64], lda: i32, work: &mut [f64]) -> f64 {
    ffi::dlange_(
        &(norm as c_char),
        &m,
        &n,
        a.as_ptr(),
        &lda,
        work.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn slange(norm: u8, m: i32, n: i32, a: &[f32], lda: i32, work: &mut [f32]) -> f32 {
    ffi::slange_(
        &(norm as c_char),
        &m,
        &n,
        a.as_ptr(),
        &lda,
        work.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zlange(norm: u8, m: i32, n: i32, a: &[c64], lda: i32, work: &mut [f64]) -> f64 {
    ffi::zlange_(
        &(norm as c_char),
        &m,
        &n,
        a.as_ptr() as *const _,
        &lda,
        work.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn clangt(norm: u8, n: i32, dl: &[c32], d: &[c32], du: &[c32]) -> f32 {
    ffi::clangt_(
        &(norm as c_char),
        &n,
        dl.as_ptr() as *const _,
        d.as_ptr() as *const _,
        du.as_ptr() as *const _,
    )
}

#[inline]
pub unsafe fn dlangt(norm: u8, n: i32, dl: &[f64], d: &[f64], du: &[f64]) -> f64 {
    ffi::dlangt_(&(norm as c_char), &n, dl.as_ptr(), d.as_ptr(), du.as_ptr())
}

#[inline]
pub unsafe fn slangt(norm: u8, n: i32, dl: &[f32], d: &[f32], du: &[f32]) -> f32 {
    ffi::slangt_(&(norm as c_char), &n, dl.as_ptr(), d.as_ptr(), du.as_ptr())
}

#[inline]
pub unsafe fn zlangt(norm: u8, n: i32, dl: &[c64], d: &[c64], du: &[c64]) -> f64 {
    ffi::zlangt_(
        &(norm as c_char),
        &n,
        dl.as_ptr() as *const _,
        d.as_ptr() as *const _,
        du.as_ptr() as *const _,
    )
}

#[inline]
pub unsafe fn clanhb(
    norm: u8,
    uplo: u8,
    n: i32,
    k: i32,
    ab: &[c32],
    ldab: i32,
    work: &mut [f32],
) -> f32 {
    ffi::clanhb_(
        &(norm as c_char),
        &(uplo as c_char),
        &n,
        &k,
        ab.as_ptr() as *const _,
        &ldab,
        work.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zlanhb(
    norm: u8,
    uplo: u8,
    n: i32,
    k: i32,
    ab: &[c64],
    ldab: i32,
    work: &mut [f64],
) -> f64 {
    ffi::zlanhb_(
        &(norm as c_char),
        &(uplo as c_char),
        &n,
        &k,
        ab.as_ptr() as *const _,
        &ldab,
        work.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn clanhe(norm: u8, uplo: u8, n: i32, a: &[c32], lda: i32, work: &mut [f32]) -> f32 {
    ffi::clanhe_(
        &(norm as c_char),
        &(uplo as c_char),
        &n,
        a.as_ptr() as *const _,
        &lda,
        work.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zlanhe(norm: u8, uplo: u8, n: i32, a: &[c64], lda: i32, work: &mut [f64]) -> f64 {
    ffi::zlanhe_(
        &(norm as c_char),
        &(uplo as c_char),
        &n,
        a.as_ptr() as *const _,
        &lda,
        work.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn clanhp(norm: u8, uplo: u8, n: i32, ap: &[c32], work: &mut [f32]) -> f32 {
    ffi::clanhp_(
        &(norm as c_char),
        &(uplo as c_char),
        &n,
        ap.as_ptr() as *const _,
        work.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zlanhp(norm: u8, uplo: u8, n: i32, ap: &[c64], work: &mut [f64]) -> f64 {
    ffi::zlanhp_(
        &(norm as c_char),
        &(uplo as c_char),
        &n,
        ap.as_ptr() as *const _,
        work.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn clanhs(norm: u8, n: i32, a: &[c32], lda: i32, work: &mut [f32]) -> f32 {
    ffi::clanhs_(
        &(norm as c_char),
        &n,
        a.as_ptr() as *const _,
        &lda,
        work.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dlanhs(norm: u8, n: i32, a: &[f64], lda: i32, work: &mut [f64]) -> f64 {
    ffi::dlanhs_(&(norm as c_char), &n, a.as_ptr(), &lda, work.as_mut_ptr())
}

#[inline]
pub unsafe fn slanhs(norm: u8, n: i32, a: &[f32], lda: i32, work: &mut [f32]) -> f32 {
    ffi::slanhs_(&(norm as c_char), &n, a.as_ptr(), &lda, work.as_mut_ptr())
}

#[inline]
pub unsafe fn zlanhs(norm: u8, n: i32, a: &[c64], lda: i32, work: &mut [f64]) -> f64 {
    ffi::zlanhs_(
        &(norm as c_char),
        &n,
        a.as_ptr() as *const _,
        &lda,
        work.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn clanht(norm: u8, n: i32, d: &[f32], e: &[c32]) -> f32 {
    ffi::clanht_(&(norm as c_char), &n, d.as_ptr(), e.as_ptr() as *const _)
}

#[inline]
pub unsafe fn zlanht(norm: u8, n: i32, d: &[f64], e: &[c64]) -> f64 {
    ffi::zlanht_(&(norm as c_char), &n, d.as_ptr(), e.as_ptr() as *const _)
}

#[inline]
pub unsafe fn clansb(
    norm: u8,
    uplo: u8,
    n: i32,
    k: i32,
    ab: &[c32],
    ldab: i32,
    work: &mut [f32],
) -> f32 {
    ffi::clansb_(
        &(norm as c_char),
        &(uplo as c_char),
        &n,
        &k,
        ab.as_ptr() as *const _,
        &ldab,
        work.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dlansb(
    norm: u8,
    uplo: u8,
    n: i32,
    k: i32,
    ab: &[f64],
    ldab: i32,
    work: &mut [f64],
) -> f64 {
    ffi::dlansb_(
        &(norm as c_char),
        &(uplo as c_char),
        &n,
        &k,
        ab.as_ptr(),
        &ldab,
        work.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn slansb(
    norm: u8,
    uplo: u8,
    n: i32,
    k: i32,
    ab: &[f32],
    ldab: i32,
    work: &mut [f32],
) -> f32 {
    ffi::slansb_(
        &(norm as c_char),
        &(uplo as c_char),
        &n,
        &k,
        ab.as_ptr(),
        &ldab,
        work.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zlansb(
    norm: u8,
    uplo: u8,
    n: i32,
    k: i32,
    ab: &[c64],
    ldab: i32,
    work: &mut [f64],
) -> f64 {
    ffi::zlansb_(
        &(norm as c_char),
        &(uplo as c_char),
        &n,
        &k,
        ab.as_ptr() as *const _,
        &ldab,
        work.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn clansp(norm: u8, uplo: u8, n: i32, ap: &[c32], work: &mut [f32]) -> f32 {
    ffi::clansp_(
        &(norm as c_char),
        &(uplo as c_char),
        &n,
        ap.as_ptr() as *const _,
        work.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dlansp(norm: u8, uplo: u8, n: i32, ap: &[f64], work: &mut [f64]) -> f64 {
    ffi::dlansp_(
        &(norm as c_char),
        &(uplo as c_char),
        &n,
        ap.as_ptr(),
        work.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn slansp(norm: u8, uplo: u8, n: i32, ap: &[f32], work: &mut [f32]) -> f32 {
    ffi::slansp_(
        &(norm as c_char),
        &(uplo as c_char),
        &n,
        ap.as_ptr(),
        work.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zlansp(norm: u8, uplo: u8, n: i32, ap: &[c64], work: &mut [f64]) -> f64 {
    ffi::zlansp_(
        &(norm as c_char),
        &(uplo as c_char),
        &n,
        ap.as_ptr() as *const _,
        work.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dlanst(norm: u8, n: i32, d: &[f64], e: &[f64]) -> f64 {
    ffi::dlanst_(&(norm as c_char), &n, d.as_ptr(), e.as_ptr())
}

#[inline]
pub unsafe fn slanst(norm: u8, n: i32, d: &[f32], e: &[f32]) -> f32 {
    ffi::slanst_(&(norm as c_char), &n, d.as_ptr(), e.as_ptr())
}

#[inline]
pub unsafe fn clansy(norm: u8, uplo: u8, n: i32, a: &[c32], lda: i32, work: &mut [f32]) -> f32 {
    ffi::clansy_(
        &(norm as c_char),
        &(uplo as c_char),
        &n,
        a.as_ptr() as *const _,
        &lda,
        work.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dlansy(norm: u8, uplo: u8, n: i32, a: &[f64], lda: i32, work: &mut [f64]) -> f64 {
    ffi::dlansy_(
        &(norm as c_char),
        &(uplo as c_char),
        &n,
        a.as_ptr(),
        &lda,
        work.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn slansy(norm: u8, uplo: u8, n: i32, a: &[f32], lda: i32, work: &mut [f32]) -> f32 {
    ffi::slansy_(
        &(norm as c_char),
        &(uplo as c_char),
        &n,
        a.as_ptr(),
        &lda,
        work.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zlansy(norm: u8, uplo: u8, n: i32, a: &[c64], lda: i32, work: &mut [f64]) -> f64 {
    ffi::zlansy_(
        &(norm as c_char),
        &(uplo as c_char),
        &n,
        a.as_ptr() as *const _,
        &lda,
        work.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn clantb(
    norm: u8,
    uplo: u8,
    diag: u8,
    n: i32,
    k: i32,
    ab: &[c32],
    ldab: i32,
    work: &mut [f32],
) -> f32 {
    ffi::clantb_(
        &(norm as c_char),
        &(uplo as c_char),
        &(diag as c_char),
        &n,
        &k,
        ab.as_ptr() as *const _,
        &ldab,
        work.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dlantb(
    norm: u8,
    uplo: u8,
    diag: u8,
    n: i32,
    k: i32,
    ab: &[f64],
    ldab: i32,
    work: &mut [f64],
) -> f64 {
    ffi::dlantb_(
        &(norm as c_char),
        &(uplo as c_char),
        &(diag as c_char),
        &n,
        &k,
        ab.as_ptr(),
        &ldab,
        work.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn slantb(
    norm: u8,
    uplo: u8,
    diag: u8,
    n: i32,
    k: i32,
    ab: &[f32],
    ldab: i32,
    work: &mut [f32],
) -> f32 {
    ffi::slantb_(
        &(norm as c_char),
        &(uplo as c_char),
        &(diag as c_char),
        &n,
        &k,
        ab.as_ptr(),
        &ldab,
        work.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zlantb(
    norm: u8,
    uplo: u8,
    diag: u8,
    n: i32,
    k: i32,
    ab: &[c64],
    ldab: i32,
    work: &mut [f64],
) -> f64 {
    ffi::zlantb_(
        &(norm as c_char),
        &(uplo as c_char),
        &(diag as c_char),
        &n,
        &k,
        ab.as_ptr() as *const _,
        &ldab,
        work.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn clantp(norm: u8, uplo: u8, diag: u8, n: i32, ap: &[c32], work: &mut [f32]) -> f32 {
    ffi::clantp_(
        &(norm as c_char),
        &(uplo as c_char),
        &(diag as c_char),
        &n,
        ap.as_ptr() as *const _,
        work.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dlantp(norm: u8, uplo: u8, diag: u8, n: i32, ap: &[f64], work: &mut [f64]) -> f64 {
    ffi::dlantp_(
        &(norm as c_char),
        &(uplo as c_char),
        &(diag as c_char),
        &n,
        ap.as_ptr(),
        work.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn slantp(norm: u8, uplo: u8, diag: u8, n: i32, ap: &[f32], work: &mut [f32]) -> f32 {
    ffi::slantp_(
        &(norm as c_char),
        &(uplo as c_char),
        &(diag as c_char),
        &n,
        ap.as_ptr(),
        work.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zlantp(norm: u8, uplo: u8, diag: u8, n: i32, ap: &[c64], work: &mut [f64]) -> f64 {
    ffi::zlantp_(
        &(norm as c_char),
        &(uplo as c_char),
        &(diag as c_char),
        &n,
        ap.as_ptr() as *const _,
        work.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn clantr(
    norm: u8,
    uplo: u8,
    diag: u8,
    m: i32,
    n: i32,
    a: &[c32],
    lda: i32,
    work: &mut [f32],
) -> f32 {
    ffi::clantr_(
        &(norm as c_char),
        &(uplo as c_char),
        &(diag as c_char),
        &m,
        &n,
        a.as_ptr() as *const _,
        &lda,
        work.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dlantr(
    norm: u8,
    uplo: u8,
    diag: u8,
    m: i32,
    n: i32,
    a: &[f64],
    lda: i32,
    work: &mut [f64],
) -> f64 {
    ffi::dlantr_(
        &(norm as c_char),
        &(uplo as c_char),
        &(diag as c_char),
        &m,
        &n,
        a.as_ptr(),
        &lda,
        work.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn slantr(
    norm: u8,
    uplo: u8,
    diag: u8,
    m: i32,
    n: i32,
    a: &[f32],
    lda: i32,
    work: &mut [f32],
) -> f32 {
    ffi::slantr_(
        &(norm as c_char),
        &(uplo as c_char),
        &(diag as c_char),
        &m,
        &n,
        a.as_ptr(),
        &lda,
        work.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zlantr(
    norm: u8,
    uplo: u8,
    diag: u8,
    m: i32,
    n: i32,
    a: &[c64],
    lda: i32,
    work: &mut [f64],
) -> f64 {
    ffi::zlantr_(
        &(norm as c_char),
        &(uplo as c_char),
        &(diag as c_char),
        &m,
        &n,
        a.as_ptr() as *const _,
        &lda,
        work.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn clapmr(forwrd: &[i32], m: i32, n: i32, x: &mut [c32], ldx: i32, k: &mut [i32]) {
    ffi::clapmr_(
        forwrd.as_ptr(),
        &m,
        &n,
        x.as_mut_ptr() as *mut _,
        &ldx,
        k.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dlapmr(forwrd: &[i32], m: i32, n: i32, x: &mut [f64], ldx: i32, k: &mut [i32]) {
    ffi::dlapmr_(
        forwrd.as_ptr(),
        &m,
        &n,
        x.as_mut_ptr(),
        &ldx,
        k.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn slapmr(forwrd: &[i32], m: i32, n: i32, x: &mut [f32], ldx: i32, k: &mut [i32]) {
    ffi::slapmr_(
        forwrd.as_ptr(),
        &m,
        &n,
        x.as_mut_ptr(),
        &ldx,
        k.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zlapmr(forwrd: &[i32], m: i32, n: i32, x: &mut [c64], ldx: i32, k: &mut [i32]) {
    ffi::zlapmr_(
        forwrd.as_ptr(),
        &m,
        &n,
        x.as_mut_ptr() as *mut _,
        &ldx,
        k.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn clapmt(forwrd: &[i32], m: i32, n: i32, x: &mut [c32], ldx: i32, k: &mut [i32]) {
    ffi::clapmt_(
        forwrd.as_ptr(),
        &m,
        &n,
        x.as_mut_ptr() as *mut _,
        &ldx,
        k.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dlapmt(forwrd: &[i32], m: i32, n: i32, x: &mut [f64], ldx: i32, k: &mut [i32]) {
    ffi::dlapmt_(
        forwrd.as_ptr(),
        &m,
        &n,
        x.as_mut_ptr(),
        &ldx,
        k.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn slapmt(forwrd: &[i32], m: i32, n: i32, x: &mut [f32], ldx: i32, k: &mut [i32]) {
    ffi::slapmt_(
        forwrd.as_ptr(),
        &m,
        &n,
        x.as_mut_ptr(),
        &ldx,
        k.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zlapmt(forwrd: &[i32], m: i32, n: i32, x: &mut [c64], ldx: i32, k: &mut [i32]) {
    ffi::zlapmt_(
        forwrd.as_ptr(),
        &m,
        &n,
        x.as_mut_ptr() as *mut _,
        &ldx,
        k.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dlapy2(x: &[f64], y: &[f64]) -> f64 {
    ffi::dlapy2_(x.as_ptr(), y.as_ptr())
}

#[inline]
pub unsafe fn slapy2(x: &[f32], y: &[f32]) -> f32 {
    ffi::slapy2_(x.as_ptr(), y.as_ptr())
}

#[inline]
pub unsafe fn dlapy3(x: &[f64], y: &[f64], z: &[f64]) -> f64 {
    ffi::dlapy3_(x.as_ptr(), y.as_ptr(), z.as_ptr())
}

#[inline]
pub unsafe fn slapy3(x: &[f32], y: &[f32], z: &[f32]) -> f32 {
    ffi::slapy3_(x.as_ptr(), y.as_ptr(), z.as_ptr())
}

#[inline]
pub unsafe fn clarcm(
    m: i32,
    n: i32,
    a: &[f32],
    lda: i32,
    b: &[c32],
    ldb: i32,
    c: &mut [c32],
    ldc: i32,
    rwork: &mut [f32],
) {
    ffi::clarcm_(
        &m,
        &n,
        a.as_ptr(),
        &lda,
        b.as_ptr() as *const _,
        &ldb,
        c.as_mut_ptr() as *mut _,
        &ldc,
        rwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zlarcm(
    m: i32,
    n: i32,
    a: &[f64],
    lda: i32,
    b: &[c64],
    ldb: i32,
    c: &mut [c64],
    ldc: i32,
    rwork: &mut [f64],
) {
    ffi::zlarcm_(
        &m,
        &n,
        a.as_ptr(),
        &lda,
        b.as_ptr() as *const _,
        &ldb,
        c.as_mut_ptr() as *mut _,
        &ldc,
        rwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn clarf(
    side: u8,
    m: i32,
    n: i32,
    v: &[c32],
    incv: i32,
    tau: &[c32],
    c: &mut [c32],
    ldc: i32,
    work: &mut [c32],
) {
    ffi::clarf_(
        &(side as c_char),
        &m,
        &n,
        v.as_ptr() as *const _,
        &incv,
        tau.as_ptr() as *const _,
        c.as_mut_ptr() as *mut _,
        &ldc,
        work.as_mut_ptr() as *mut _,
    )
}

#[inline]
pub unsafe fn dlarf(
    side: u8,
    m: i32,
    n: i32,
    v: &[f64],
    incv: i32,
    tau: &[f64],
    c: &mut [f64],
    ldc: i32,
    work: &mut [f64],
) {
    ffi::dlarf_(
        &(side as c_char),
        &m,
        &n,
        v.as_ptr(),
        &incv,
        tau.as_ptr(),
        c.as_mut_ptr(),
        &ldc,
        work.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn slarf(
    side: u8,
    m: i32,
    n: i32,
    v: &[f32],
    incv: i32,
    tau: &[f32],
    c: &mut [f32],
    ldc: i32,
    work: &mut [f32],
) {
    ffi::slarf_(
        &(side as c_char),
        &m,
        &n,
        v.as_ptr(),
        &incv,
        tau.as_ptr(),
        c.as_mut_ptr(),
        &ldc,
        work.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zlarf(
    side: u8,
    m: i32,
    n: i32,
    v: &[c64],
    incv: i32,
    tau: &[c64],
    c: &mut [c64],
    ldc: i32,
    work: &mut [c64],
) {
    ffi::zlarf_(
        &(side as c_char),
        &m,
        &n,
        v.as_ptr() as *const _,
        &incv,
        tau.as_ptr() as *const _,
        c.as_mut_ptr() as *mut _,
        &ldc,
        work.as_mut_ptr() as *mut _,
    )
}

#[inline]
pub unsafe fn clarfb(
    side: u8,
    trans: u8,
    direct: u8,
    storev: u8,
    m: i32,
    n: i32,
    k: i32,
    v: &[c32],
    ldv: i32,
    t: &[c32],
    ldt: i32,
    c: &mut [c32],
    ldc: i32,
    work: &mut [c32],
    ldwork: i32,
) {
    ffi::clarfb_(
        &(side as c_char),
        &(trans as c_char),
        &(direct as c_char),
        &(storev as c_char),
        &m,
        &n,
        &k,
        v.as_ptr() as *const _,
        &ldv,
        t.as_ptr() as *const _,
        &ldt,
        c.as_mut_ptr() as *mut _,
        &ldc,
        work.as_mut_ptr() as *mut _,
        &ldwork,
    )
}

#[inline]
pub unsafe fn dlarfb(
    side: u8,
    trans: u8,
    direct: u8,
    storev: u8,
    m: i32,
    n: i32,
    k: i32,
    v: &[f64],
    ldv: i32,
    t: &[f64],
    ldt: i32,
    c: &mut [f64],
    ldc: i32,
    work: &mut [f64],
    ldwork: i32,
) {
    ffi::dlarfb_(
        &(side as c_char),
        &(trans as c_char),
        &(direct as c_char),
        &(storev as c_char),
        &m,
        &n,
        &k,
        v.as_ptr(),
        &ldv,
        t.as_ptr(),
        &ldt,
        c.as_mut_ptr(),
        &ldc,
        work.as_mut_ptr(),
        &ldwork,
    )
}

#[inline]
pub unsafe fn slarfb(
    side: u8,
    trans: u8,
    direct: u8,
    storev: u8,
    m: i32,
    n: i32,
    k: i32,
    v: &[f32],
    ldv: i32,
    t: &[f32],
    ldt: i32,
    c: &mut [f32],
    ldc: i32,
    work: &mut [f32],
    ldwork: i32,
) {
    ffi::slarfb_(
        &(side as c_char),
        &(trans as c_char),
        &(direct as c_char),
        &(storev as c_char),
        &m,
        &n,
        &k,
        v.as_ptr(),
        &ldv,
        t.as_ptr(),
        &ldt,
        c.as_mut_ptr(),
        &ldc,
        work.as_mut_ptr(),
        &ldwork,
    )
}

#[inline]
pub unsafe fn zlarfb(
    side: u8,
    trans: u8,
    direct: u8,
    storev: u8,
    m: i32,
    n: i32,
    k: i32,
    v: &[c64],
    ldv: i32,
    t: &[c64],
    ldt: i32,
    c: &mut [c64],
    ldc: i32,
    work: &mut [c64],
    ldwork: i32,
) {
    ffi::zlarfb_(
        &(side as c_char),
        &(trans as c_char),
        &(direct as c_char),
        &(storev as c_char),
        &m,
        &n,
        &k,
        v.as_ptr() as *const _,
        &ldv,
        t.as_ptr() as *const _,
        &ldt,
        c.as_mut_ptr() as *mut _,
        &ldc,
        work.as_mut_ptr() as *mut _,
        &ldwork,
    )
}

#[inline]
pub unsafe fn clarfg(n: i32, alpha: &mut c32, x: &mut [c32], incx: i32, tau: &mut [c32]) {
    ffi::clarfg_(
        &n,
        alpha as *mut _ as *mut _,
        x.as_mut_ptr() as *mut _,
        &incx,
        tau.as_mut_ptr() as *mut _,
    )
}

#[inline]
pub unsafe fn dlarfg(n: i32, alpha: &mut f64, x: &mut [f64], incx: i32, tau: &mut [f64]) {
    ffi::dlarfg_(&n, alpha, x.as_mut_ptr(), &incx, tau.as_mut_ptr())
}

#[inline]
pub unsafe fn slarfg(n: i32, alpha: &mut f32, x: &mut [f32], incx: i32, tau: &mut [f32]) {
    ffi::slarfg_(&n, alpha, x.as_mut_ptr(), &incx, tau.as_mut_ptr())
}

#[inline]
pub unsafe fn zlarfg(n: i32, alpha: &mut c64, x: &mut [c64], incx: i32, tau: &mut [c64]) {
    ffi::zlarfg_(
        &n,
        alpha as *mut _ as *mut _,
        x.as_mut_ptr() as *mut _,
        &incx,
        tau.as_mut_ptr() as *mut _,
    )
}

#[inline]
pub unsafe fn clarft(
    direct: u8,
    storev: u8,
    n: i32,
    k: i32,
    v: &[c32],
    ldv: i32,
    tau: &[c32],
    t: &mut [c32],
    ldt: i32,
) {
    ffi::clarft_(
        &(direct as c_char),
        &(storev as c_char),
        &n,
        &k,
        v.as_ptr() as *const _,
        &ldv,
        tau.as_ptr() as *const _,
        t.as_mut_ptr() as *mut _,
        &ldt,
    )
}

#[inline]
pub unsafe fn dlarft(
    direct: u8,
    storev: u8,
    n: i32,
    k: i32,
    v: &[f64],
    ldv: i32,
    tau: &[f64],
    t: &mut [f64],
    ldt: i32,
) {
    ffi::dlarft_(
        &(direct as c_char),
        &(storev as c_char),
        &n,
        &k,
        v.as_ptr(),
        &ldv,
        tau.as_ptr(),
        t.as_mut_ptr(),
        &ldt,
    )
}

#[inline]
pub unsafe fn slarft(
    direct: u8,
    storev: u8,
    n: i32,
    k: i32,
    v: &[f32],
    ldv: i32,
    tau: &[f32],
    t: &mut [f32],
    ldt: i32,
) {
    ffi::slarft_(
        &(direct as c_char),
        &(storev as c_char),
        &n,
        &k,
        v.as_ptr(),
        &ldv,
        tau.as_ptr(),
        t.as_mut_ptr(),
        &ldt,
    )
}

#[inline]
pub unsafe fn zlarft(
    direct: u8,
    storev: u8,
    n: i32,
    k: i32,
    v: &[c64],
    ldv: i32,
    tau: &[c64],
    t: &mut [c64],
    ldt: i32,
) {
    ffi::zlarft_(
        &(direct as c_char),
        &(storev as c_char),
        &n,
        &k,
        v.as_ptr() as *const _,
        &ldv,
        tau.as_ptr() as *const _,
        t.as_mut_ptr() as *mut _,
        &ldt,
    )
}

#[inline]
pub unsafe fn clarfx(
    side: u8,
    m: i32,
    n: i32,
    v: &[c32],
    tau: &[c32],
    c: &mut [c32],
    ldc: i32,
    work: &mut [c32],
) {
    ffi::clarfx_(
        &(side as c_char),
        &m,
        &n,
        v.as_ptr() as *const _,
        tau.as_ptr() as *const _,
        c.as_mut_ptr() as *mut _,
        &ldc,
        work.as_mut_ptr() as *mut _,
    )
}

#[inline]
pub unsafe fn dlarfx(
    side: u8,
    m: i32,
    n: i32,
    v: &[f64],
    tau: &[f64],
    c: &mut [f64],
    ldc: i32,
    work: &mut [f64],
) {
    ffi::dlarfx_(
        &(side as c_char),
        &m,
        &n,
        v.as_ptr(),
        tau.as_ptr(),
        c.as_mut_ptr(),
        &ldc,
        work.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn slarfx(
    side: u8,
    m: i32,
    n: i32,
    v: &[f32],
    tau: &[f32],
    c: &mut [f32],
    ldc: i32,
    work: &mut [f32],
) {
    ffi::slarfx_(
        &(side as c_char),
        &m,
        &n,
        v.as_ptr(),
        tau.as_ptr(),
        c.as_mut_ptr(),
        &ldc,
        work.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zlarfx(
    side: u8,
    m: i32,
    n: i32,
    v: &[c64],
    tau: &[c64],
    c: &mut [c64],
    ldc: i32,
    work: &mut [c64],
) {
    ffi::zlarfx_(
        &(side as c_char),
        &m,
        &n,
        v.as_ptr() as *const _,
        tau.as_ptr() as *const _,
        c.as_mut_ptr() as *mut _,
        &ldc,
        work.as_mut_ptr() as *mut _,
    )
}

#[inline]
pub unsafe fn clarnv(idist: &[i32], iseed: &mut [i32], n: i32, x: &mut [c32]) {
    ffi::clarnv_(
        idist.as_ptr(),
        iseed.as_mut_ptr(),
        &n,
        x.as_mut_ptr() as *mut _,
    )
}

#[inline]
pub unsafe fn dlarnv(idist: &[i32], iseed: &mut [i32], n: i32, x: &mut [f64]) {
    ffi::dlarnv_(idist.as_ptr(), iseed.as_mut_ptr(), &n, x.as_mut_ptr())
}

#[inline]
pub unsafe fn slarnv(idist: &[i32], iseed: &mut [i32], n: i32, x: &mut [f32]) {
    ffi::slarnv_(idist.as_ptr(), iseed.as_mut_ptr(), &n, x.as_mut_ptr())
}

#[inline]
pub unsafe fn zlarnv(idist: &[i32], iseed: &mut [i32], n: i32, x: &mut [c64]) {
    ffi::zlarnv_(
        idist.as_ptr(),
        iseed.as_mut_ptr(),
        &n,
        x.as_mut_ptr() as *mut _,
    )
}

#[inline]
pub unsafe fn dlartgp(f: &[f64], g: &[f64], cs: &mut [f64], sn: &mut [f64], r: &mut [f64]) {
    ffi::dlartgp_(
        f.as_ptr(),
        g.as_ptr(),
        cs.as_mut_ptr(),
        sn.as_mut_ptr(),
        r.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn slartgp(f: &[f32], g: &[f32], cs: &mut [f32], sn: &mut [f32], r: &mut [f32]) {
    ffi::slartgp_(
        f.as_ptr(),
        g.as_ptr(),
        cs.as_mut_ptr(),
        sn.as_mut_ptr(),
        r.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dlartgs(x: &[f64], y: &[f64], sigma: &[f64], cs: &mut [f64], sn: &mut [f64]) {
    ffi::dlartgs_(
        x.as_ptr(),
        y.as_ptr(),
        sigma.as_ptr(),
        cs.as_mut_ptr(),
        sn.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn slartgs(x: &[f32], y: &[f32], sigma: &[f32], cs: &mut [f32], sn: &mut [f32]) {
    ffi::slartgs_(
        x.as_ptr(),
        y.as_ptr(),
        sigma.as_ptr(),
        cs.as_mut_ptr(),
        sn.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn clascl(
    type_: u8,
    kl: i32,
    ku: i32,
    cfrom: &[f32],
    cto: &[f32],
    m: i32,
    n: i32,
    a: &mut [c32],
    lda: i32,
    info: &mut i32,
) {
    ffi::clascl_(
        &(type_ as c_char),
        &kl,
        &ku,
        cfrom.as_ptr(),
        cto.as_ptr(),
        &m,
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        info,
    )
}

#[inline]
pub unsafe fn dlascl(
    type_: u8,
    kl: i32,
    ku: i32,
    cfrom: &[f64],
    cto: &[f64],
    m: i32,
    n: i32,
    a: &mut [f64],
    lda: i32,
    info: &mut i32,
) {
    ffi::dlascl_(
        &(type_ as c_char),
        &kl,
        &ku,
        cfrom.as_ptr(),
        cto.as_ptr(),
        &m,
        &n,
        a.as_mut_ptr(),
        &lda,
        info,
    )
}

#[inline]
pub unsafe fn slascl(
    type_: u8,
    kl: i32,
    ku: i32,
    cfrom: &[f32],
    cto: &[f32],
    m: i32,
    n: i32,
    a: &mut [f32],
    lda: i32,
    info: &mut i32,
) {
    ffi::slascl_(
        &(type_ as c_char),
        &kl,
        &ku,
        cfrom.as_ptr(),
        cto.as_ptr(),
        &m,
        &n,
        a.as_mut_ptr(),
        &lda,
        info,
    )
}

#[inline]
pub unsafe fn zlascl(
    type_: u8,
    kl: i32,
    ku: i32,
    cfrom: &[f64],
    cto: &[f64],
    m: i32,
    n: i32,
    a: &mut [c64],
    lda: i32,
    info: &mut i32,
) {
    ffi::zlascl_(
        &(type_ as c_char),
        &kl,
        &ku,
        cfrom.as_ptr(),
        cto.as_ptr(),
        &m,
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        info,
    )
}

#[inline]
pub unsafe fn claset(
    uplo: u8,
    m: i32,
    n: i32,
    alpha: &[c32],
    beta: &[c32],
    a: &mut [c32],
    lda: i32,
) {
    ffi::claset_(
        &(uplo as c_char),
        &m,
        &n,
        alpha.as_ptr() as *const _,
        beta.as_ptr() as *const _,
        a.as_mut_ptr() as *mut _,
        &lda,
    )
}

#[inline]
pub unsafe fn dlaset(
    uplo: u8,
    m: i32,
    n: i32,
    alpha: &[f64],
    beta: &[f64],
    a: &mut [f64],
    lda: i32,
) {
    ffi::dlaset_(
        &(uplo as c_char),
        &m,
        &n,
        alpha.as_ptr(),
        beta.as_ptr(),
        a.as_mut_ptr(),
        &lda,
    )
}

#[inline]
pub unsafe fn slaset(
    uplo: u8,
    m: i32,
    n: i32,
    alpha: &[f32],
    beta: &[f32],
    a: &mut [f32],
    lda: i32,
) {
    ffi::slaset_(
        &(uplo as c_char),
        &m,
        &n,
        alpha.as_ptr(),
        beta.as_ptr(),
        a.as_mut_ptr(),
        &lda,
    )
}

#[inline]
pub unsafe fn zlaset(
    uplo: u8,
    m: i32,
    n: i32,
    alpha: &[c64],
    beta: &[c64],
    a: &mut [c64],
    lda: i32,
) {
    ffi::zlaset_(
        &(uplo as c_char),
        &m,
        &n,
        alpha.as_ptr() as *const _,
        beta.as_ptr() as *const _,
        a.as_mut_ptr() as *mut _,
        &lda,
    )
}

#[inline]
pub unsafe fn dlasrt(id: u8, n: i32, d: &mut [f64], info: &mut i32) {
    ffi::dlasrt_(&(id as c_char), &n, d.as_mut_ptr(), info)
}

#[inline]
pub unsafe fn slasrt(id: u8, n: i32, d: &mut [f32], info: &mut i32) {
    ffi::slasrt_(&(id as c_char), &n, d.as_mut_ptr(), info)
}

#[inline]
pub unsafe fn classq(n: i32, x: &[c32], incx: i32, scale: &mut [f32], sumsq: &mut [f32]) {
    ffi::classq_(
        &n,
        x.as_ptr() as *const _,
        &incx,
        scale.as_mut_ptr(),
        sumsq.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dlassq(n: i32, x: &[f64], incx: i32, scale: &mut [f64], sumsq: &mut [f64]) {
    ffi::dlassq_(
        &n,
        x.as_ptr(),
        &incx,
        scale.as_mut_ptr(),
        sumsq.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn slassq(n: i32, x: &[f32], incx: i32, scale: &mut [f32], sumsq: &mut [f32]) {
    ffi::slassq_(
        &n,
        x.as_ptr(),
        &incx,
        scale.as_mut_ptr(),
        sumsq.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zlassq(n: i32, x: &[c64], incx: i32, scale: &mut [f64], sumsq: &mut [f64]) {
    ffi::zlassq_(
        &n,
        x.as_ptr() as *const _,
        &incx,
        scale.as_mut_ptr(),
        sumsq.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn claswp(n: i32, a: &mut [c32], lda: i32, k1: i32, k2: i32, ipiv: &[i32], incx: i32) {
    ffi::claswp_(
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        &k1,
        &k2,
        ipiv.as_ptr(),
        &incx,
    )
}

#[inline]
pub unsafe fn dlaswp(n: i32, a: &mut [f64], lda: i32, k1: i32, k2: i32, ipiv: &[i32], incx: i32) {
    ffi::dlaswp_(&n, a.as_mut_ptr(), &lda, &k1, &k2, ipiv.as_ptr(), &incx)
}

#[inline]
pub unsafe fn slaswp(n: i32, a: &mut [f32], lda: i32, k1: i32, k2: i32, ipiv: &[i32], incx: i32) {
    ffi::slaswp_(&n, a.as_mut_ptr(), &lda, &k1, &k2, ipiv.as_ptr(), &incx)
}

#[inline]
pub unsafe fn zlaswp(n: i32, a: &mut [c64], lda: i32, k1: i32, k2: i32, ipiv: &[i32], incx: i32) {
    ffi::zlaswp_(
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        &k1,
        &k2,
        ipiv.as_ptr(),
        &incx,
    )
}

#[inline]
pub unsafe fn clatms(
    m: i32,
    n: i32,
    dist: u8,
    iseed: &mut [i32],
    sym: u8,
    d: &mut [f32],
    mode: &[i32],
    cond: &[f32],
    dmax: &[f32],
    kl: i32,
    ku: i32,
    pack: u8,
    a: &mut [c32],
    lda: i32,
    work: &mut [c32],
    info: &mut i32,
) {
    ffi::clatms_(
        &m,
        &n,
        &(dist as c_char),
        iseed.as_mut_ptr(),
        &(sym as c_char),
        d.as_mut_ptr(),
        mode.as_ptr(),
        cond.as_ptr(),
        dmax.as_ptr(),
        &kl,
        &ku,
        &(pack as c_char),
        a.as_mut_ptr() as *mut _,
        &lda,
        work.as_mut_ptr() as *mut _,
        info,
    )
}

#[inline]
pub unsafe fn dlatms(
    m: i32,
    n: i32,
    dist: u8,
    iseed: &mut [i32],
    sym: u8,
    d: &mut [f64],
    mode: &[i32],
    cond: &[f64],
    dmax: &[f64],
    kl: i32,
    ku: i32,
    pack: u8,
    a: &mut [f64],
    lda: i32,
    work: &mut [f64],
    info: &mut i32,
) {
    ffi::dlatms_(
        &m,
        &n,
        &(dist as c_char),
        iseed.as_mut_ptr(),
        &(sym as c_char),
        d.as_mut_ptr(),
        mode.as_ptr(),
        cond.as_ptr(),
        dmax.as_ptr(),
        &kl,
        &ku,
        &(pack as c_char),
        a.as_mut_ptr(),
        &lda,
        work.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn slatms(
    m: i32,
    n: i32,
    dist: u8,
    iseed: &mut [i32],
    sym: u8,
    d: &mut [f32],
    mode: &[i32],
    cond: &[f32],
    dmax: &[f32],
    kl: i32,
    ku: i32,
    pack: u8,
    a: &mut [f32],
    lda: i32,
    work: &mut [f32],
    info: &mut i32,
) {
    ffi::slatms_(
        &m,
        &n,
        &(dist as c_char),
        iseed.as_mut_ptr(),
        &(sym as c_char),
        d.as_mut_ptr(),
        mode.as_ptr(),
        cond.as_ptr(),
        dmax.as_ptr(),
        &kl,
        &ku,
        &(pack as c_char),
        a.as_mut_ptr(),
        &lda,
        work.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn zlatms(
    m: i32,
    n: i32,
    dist: u8,
    iseed: &mut [i32],
    sym: u8,
    d: &mut [f64],
    mode: &[i32],
    cond: &[f64],
    dmax: &[f64],
    kl: i32,
    ku: i32,
    pack: u8,
    a: &mut [c64],
    lda: i32,
    work: &mut [c64],
    info: &mut i32,
) {
    ffi::zlatms_(
        &m,
        &n,
        &(dist as c_char),
        iseed.as_mut_ptr(),
        &(sym as c_char),
        d.as_mut_ptr(),
        mode.as_ptr(),
        cond.as_ptr(),
        dmax.as_ptr(),
        &kl,
        &ku,
        &(pack as c_char),
        a.as_mut_ptr() as *mut _,
        &lda,
        work.as_mut_ptr() as *mut _,
        info,
    )
}

#[inline]
pub unsafe fn clauum(uplo: u8, n: i32, a: &mut [c32], lda: i32, info: &mut i32) {
    ffi::clauum_(&(uplo as c_char), &n, a.as_mut_ptr() as *mut _, &lda, info)
}

#[inline]
pub unsafe fn dlauum(uplo: u8, n: i32, a: &mut [f64], lda: i32, info: &mut i32) {
    ffi::dlauum_(&(uplo as c_char), &n, a.as_mut_ptr(), &lda, info)
}

#[inline]
pub unsafe fn slauum(uplo: u8, n: i32, a: &mut [f32], lda: i32, info: &mut i32) {
    ffi::slauum_(&(uplo as c_char), &n, a.as_mut_ptr(), &lda, info)
}

#[inline]
pub unsafe fn zlauum(uplo: u8, n: i32, a: &mut [c64], lda: i32, info: &mut i32) {
    ffi::zlauum_(&(uplo as c_char), &n, a.as_mut_ptr() as *mut _, &lda, info)
}

#[inline]
pub unsafe fn ilaver(vers_major: &mut i32, vers_minor: &mut i32, vers_patch: &mut i32) {
    ffi::ilaver_(vers_major, vers_minor, vers_patch)
}

#[inline]
pub unsafe fn dopgtr(
    uplo: u8,
    n: i32,
    ap: &[f64],
    tau: &[f64],
    q: &mut [f64],
    ldq: i32,
    work: &mut [f64],
    info: &mut i32,
) {
    ffi::dopgtr_(
        &(uplo as c_char),
        &n,
        ap.as_ptr(),
        tau.as_ptr(),
        q.as_mut_ptr(),
        &ldq,
        work.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn sopgtr(
    uplo: u8,
    n: i32,
    ap: &[f32],
    tau: &[f32],
    q: &mut [f32],
    ldq: i32,
    work: &mut [f32],
    info: &mut i32,
) {
    ffi::sopgtr_(
        &(uplo as c_char),
        &n,
        ap.as_ptr(),
        tau.as_ptr(),
        q.as_mut_ptr(),
        &ldq,
        work.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn dopmtr(
    side: u8,
    uplo: u8,
    trans: u8,
    m: i32,
    n: i32,
    ap: &[f64],
    tau: &[f64],
    c: &mut [f64],
    ldc: i32,
    work: &mut [f64],
    info: &mut i32,
) {
    ffi::dopmtr_(
        &(side as c_char),
        &(uplo as c_char),
        &(trans as c_char),
        &m,
        &n,
        ap.as_ptr(),
        tau.as_ptr(),
        c.as_mut_ptr(),
        &ldc,
        work.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn sopmtr(
    side: u8,
    uplo: u8,
    trans: u8,
    m: i32,
    n: i32,
    ap: &[f32],
    tau: &[f32],
    c: &mut [f32],
    ldc: i32,
    work: &mut [f32],
    info: &mut i32,
) {
    ffi::sopmtr_(
        &(side as c_char),
        &(uplo as c_char),
        &(trans as c_char),
        &m,
        &n,
        ap.as_ptr(),
        tau.as_ptr(),
        c.as_mut_ptr(),
        &ldc,
        work.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn dorbdb(
    trans: u8,
    signs: u8,
    m: i32,
    p: i32,
    q: &[i32],
    x11: &mut [f64],
    ldx11: i32,
    x12: &mut [f64],
    ldx12: i32,
    x21: &mut [f64],
    ldx21: i32,
    x22: &mut [f64],
    ldx22: i32,
    theta: &mut [f64],
    phi: &mut [f64],
    taup1: &mut [f64],
    taup2: &mut [f64],
    tauq1: &mut [f64],
    tauq2: &mut [f64],
    work: &mut [f64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::dorbdb_(
        &(trans as c_char),
        &(signs as c_char),
        &m,
        &p,
        q.as_ptr(),
        x11.as_mut_ptr(),
        &ldx11,
        x12.as_mut_ptr(),
        &ldx12,
        x21.as_mut_ptr(),
        &ldx21,
        x22.as_mut_ptr(),
        &ldx22,
        theta.as_mut_ptr(),
        phi.as_mut_ptr(),
        taup1.as_mut_ptr(),
        taup2.as_mut_ptr(),
        tauq1.as_mut_ptr(),
        tauq2.as_mut_ptr(),
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn sorbdb(
    trans: u8,
    signs: u8,
    m: i32,
    p: i32,
    q: &[i32],
    x11: &mut [f32],
    ldx11: i32,
    x12: &mut [f32],
    ldx12: i32,
    x21: &mut [f32],
    ldx21: i32,
    x22: &mut [f32],
    ldx22: i32,
    theta: &mut [f32],
    phi: &mut [f32],
    taup1: &mut [f32],
    taup2: &mut [f32],
    tauq1: &mut [f32],
    tauq2: &mut [f32],
    work: &mut [f32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::sorbdb_(
        &(trans as c_char),
        &(signs as c_char),
        &m,
        &p,
        q.as_ptr(),
        x11.as_mut_ptr(),
        &ldx11,
        x12.as_mut_ptr(),
        &ldx12,
        x21.as_mut_ptr(),
        &ldx21,
        x22.as_mut_ptr(),
        &ldx22,
        theta.as_mut_ptr(),
        phi.as_mut_ptr(),
        taup1.as_mut_ptr(),
        taup2.as_mut_ptr(),
        tauq1.as_mut_ptr(),
        tauq2.as_mut_ptr(),
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn dorcsd(
    jobu1: u8,
    jobu2: u8,
    jobv1t: u8,
    jobv2t: u8,
    trans: u8,
    signs: u8,
    m: i32,
    p: i32,
    q: &[i32],
    x11: &mut [f64],
    ldx11: i32,
    x12: &mut [f64],
    ldx12: i32,
    x21: &mut [f64],
    ldx21: i32,
    x22: &mut [f64],
    ldx22: i32,
    theta: &mut [f64],
    u1: &mut [f64],
    ldu1: i32,
    u2: &mut [f64],
    ldu2: i32,
    v1t: &mut [f64],
    ldv1t: i32,
    v2t: &mut [f64],
    ldv2t: i32,
    work: &mut [f64],
    lwork: i32,
    iwork: &mut [i32],
    info: &mut i32,
) {
    ffi::dorcsd_(
        &(jobu1 as c_char),
        &(jobu2 as c_char),
        &(jobv1t as c_char),
        &(jobv2t as c_char),
        &(trans as c_char),
        &(signs as c_char),
        &m,
        &p,
        q.as_ptr(),
        x11.as_mut_ptr(),
        &ldx11,
        x12.as_mut_ptr(),
        &ldx12,
        x21.as_mut_ptr(),
        &ldx21,
        x22.as_mut_ptr(),
        &ldx22,
        theta.as_mut_ptr(),
        u1.as_mut_ptr(),
        &ldu1,
        u2.as_mut_ptr(),
        &ldu2,
        v1t.as_mut_ptr(),
        &ldv1t,
        v2t.as_mut_ptr(),
        &ldv2t,
        work.as_mut_ptr(),
        &lwork,
        iwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn sorcsd(
    jobu1: u8,
    jobu2: u8,
    jobv1t: u8,
    jobv2t: u8,
    trans: u8,
    signs: u8,
    m: i32,
    p: i32,
    q: &[i32],
    x11: &mut [f32],
    ldx11: i32,
    x12: &mut [f32],
    ldx12: i32,
    x21: &mut [f32],
    ldx21: i32,
    x22: &mut [f32],
    ldx22: i32,
    theta: &mut [f32],
    u1: &mut [f32],
    ldu1: i32,
    u2: &mut [f32],
    ldu2: i32,
    v1t: &mut [f32],
    ldv1t: i32,
    v2t: &mut [f32],
    ldv2t: i32,
    work: &mut [f32],
    lwork: i32,
    iwork: &mut [i32],
    info: &mut i32,
) {
    ffi::sorcsd_(
        &(jobu1 as c_char),
        &(jobu2 as c_char),
        &(jobv1t as c_char),
        &(jobv2t as c_char),
        &(trans as c_char),
        &(signs as c_char),
        &m,
        &p,
        q.as_ptr(),
        x11.as_mut_ptr(),
        &ldx11,
        x12.as_mut_ptr(),
        &ldx12,
        x21.as_mut_ptr(),
        &ldx21,
        x22.as_mut_ptr(),
        &ldx22,
        theta.as_mut_ptr(),
        u1.as_mut_ptr(),
        &ldu1,
        u2.as_mut_ptr(),
        &ldu2,
        v1t.as_mut_ptr(),
        &ldv1t,
        v2t.as_mut_ptr(),
        &ldv2t,
        work.as_mut_ptr(),
        &lwork,
        iwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn dorcsd2by1(
    jobu1: u8,
    jobu2: u8,
    jobv1t: u8,
    m: i32,
    p: i32,
    q: &[i32],
    x11: &mut [f64],
    ldx11: i32,
    x21: &mut [f64],
    ldx21: i32,
    theta: &mut [f64],
    u1: &mut [f64],
    ldu1: i32,
    u2: &mut [f64],
    ldu2: i32,
    v1t: &mut [f64],
    ldv1t: i32,
    work: &mut [f64],
    lwork: i32,
    iwork: &mut [i32],
    info: &mut i32,
) {
    ffi::dorcsd2by1_(
        &(jobu1 as c_char),
        &(jobu2 as c_char),
        &(jobv1t as c_char),
        &m,
        &p,
        q.as_ptr(),
        x11.as_mut_ptr(),
        &ldx11,
        x21.as_mut_ptr(),
        &ldx21,
        theta.as_mut_ptr(),
        u1.as_mut_ptr(),
        &ldu1,
        u2.as_mut_ptr(),
        &ldu2,
        v1t.as_mut_ptr(),
        &ldv1t,
        work.as_mut_ptr(),
        &lwork,
        iwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn sorcsd2by1(
    jobu1: u8,
    jobu2: u8,
    jobv1t: u8,
    m: i32,
    p: i32,
    q: &[i32],
    x11: &mut [f32],
    ldx11: i32,
    x21: &mut [f32],
    ldx21: i32,
    theta: &mut [f32],
    u1: &mut [f32],
    ldu1: i32,
    u2: &mut [f32],
    ldu2: i32,
    v1t: &mut [f32],
    ldv1t: i32,
    work: &mut [f32],
    lwork: i32,
    iwork: &mut [i32],
    info: &mut i32,
) {
    ffi::sorcsd2by1_(
        &(jobu1 as c_char),
        &(jobu2 as c_char),
        &(jobv1t as c_char),
        &m,
        &p,
        q.as_ptr(),
        x11.as_mut_ptr(),
        &ldx11,
        x21.as_mut_ptr(),
        &ldx21,
        theta.as_mut_ptr(),
        u1.as_mut_ptr(),
        &ldu1,
        u2.as_mut_ptr(),
        &ldu2,
        v1t.as_mut_ptr(),
        &ldv1t,
        work.as_mut_ptr(),
        &lwork,
        iwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn dorgbr(
    vect: u8,
    m: i32,
    n: i32,
    k: i32,
    a: &mut [f64],
    lda: i32,
    tau: &[f64],
    work: &mut [f64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::dorgbr_(
        &(vect as c_char),
        &m,
        &n,
        &k,
        a.as_mut_ptr(),
        &lda,
        tau.as_ptr(),
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn sorgbr(
    vect: u8,
    m: i32,
    n: i32,
    k: i32,
    a: &mut [f32],
    lda: i32,
    tau: &[f32],
    work: &mut [f32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::sorgbr_(
        &(vect as c_char),
        &m,
        &n,
        &k,
        a.as_mut_ptr(),
        &lda,
        tau.as_ptr(),
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn dorghr(
    n: i32,
    ilo: i32,
    ihi: i32,
    a: &mut [f64],
    lda: i32,
    tau: &[f64],
    work: &mut [f64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::dorghr_(
        &n,
        &ilo,
        &ihi,
        a.as_mut_ptr(),
        &lda,
        tau.as_ptr(),
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn sorghr(
    n: i32,
    ilo: i32,
    ihi: i32,
    a: &mut [f32],
    lda: i32,
    tau: &[f32],
    work: &mut [f32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::sorghr_(
        &n,
        &ilo,
        &ihi,
        a.as_mut_ptr(),
        &lda,
        tau.as_ptr(),
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn dorglq(
    m: i32,
    n: i32,
    k: i32,
    a: &mut [f64],
    lda: i32,
    tau: &[f64],
    work: &mut [f64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::dorglq_(
        &m,
        &n,
        &k,
        a.as_mut_ptr(),
        &lda,
        tau.as_ptr(),
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn sorglq(
    m: i32,
    n: i32,
    k: i32,
    a: &mut [f32],
    lda: i32,
    tau: &[f32],
    work: &mut [f32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::sorglq_(
        &m,
        &n,
        &k,
        a.as_mut_ptr(),
        &lda,
        tau.as_ptr(),
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn dorgql(
    m: i32,
    n: i32,
    k: i32,
    a: &mut [f64],
    lda: i32,
    tau: &[f64],
    work: &mut [f64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::dorgql_(
        &m,
        &n,
        &k,
        a.as_mut_ptr(),
        &lda,
        tau.as_ptr(),
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn sorgql(
    m: i32,
    n: i32,
    k: i32,
    a: &mut [f32],
    lda: i32,
    tau: &[f32],
    work: &mut [f32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::sorgql_(
        &m,
        &n,
        &k,
        a.as_mut_ptr(),
        &lda,
        tau.as_ptr(),
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn dorgqr(
    m: i32,
    n: i32,
    k: i32,
    a: &mut [f64],
    lda: i32,
    tau: &[f64],
    work: &mut [f64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::dorgqr_(
        &m,
        &n,
        &k,
        a.as_mut_ptr(),
        &lda,
        tau.as_ptr(),
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn sorgqr(
    m: i32,
    n: i32,
    k: i32,
    a: &mut [f32],
    lda: i32,
    tau: &[f32],
    work: &mut [f32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::sorgqr_(
        &m,
        &n,
        &k,
        a.as_mut_ptr(),
        &lda,
        tau.as_ptr(),
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn dorgrq(
    m: i32,
    n: i32,
    k: i32,
    a: &mut [f64],
    lda: i32,
    tau: &[f64],
    work: &mut [f64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::dorgrq_(
        &m,
        &n,
        &k,
        a.as_mut_ptr(),
        &lda,
        tau.as_ptr(),
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn sorgrq(
    m: i32,
    n: i32,
    k: i32,
    a: &mut [f32],
    lda: i32,
    tau: &[f32],
    work: &mut [f32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::sorgrq_(
        &m,
        &n,
        &k,
        a.as_mut_ptr(),
        &lda,
        tau.as_ptr(),
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn dorgtr(
    uplo: u8,
    n: i32,
    a: &mut [f64],
    lda: i32,
    tau: &[f64],
    work: &mut [f64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::dorgtr_(
        &(uplo as c_char),
        &n,
        a.as_mut_ptr(),
        &lda,
        tau.as_ptr(),
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn sorgtr(
    uplo: u8,
    n: i32,
    a: &mut [f32],
    lda: i32,
    tau: &[f32],
    work: &mut [f32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::sorgtr_(
        &(uplo as c_char),
        &n,
        a.as_mut_ptr(),
        &lda,
        tau.as_ptr(),
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn dorgtsqr_row(
    m: i32,
    n: i32,
    mb: &[i32],
    nb: i32,
    a: &mut [f64],
    lda: i32,
    t: &[f64],
    ldt: i32,
    work: &mut [f64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::dorgtsqr_row_(
        &m,
        &n,
        mb.as_ptr(),
        &nb,
        a.as_mut_ptr(),
        &lda,
        t.as_ptr(),
        &ldt,
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn sorgtsqr_row(
    m: i32,
    n: i32,
    mb: &[i32],
    nb: i32,
    a: &mut [f32],
    lda: i32,
    t: &[f32],
    ldt: i32,
    work: &mut [f32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::sorgtsqr_row_(
        &m,
        &n,
        mb.as_ptr(),
        &nb,
        a.as_mut_ptr(),
        &lda,
        t.as_ptr(),
        &ldt,
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn dormbr(
    vect: u8,
    side: u8,
    trans: u8,
    m: i32,
    n: i32,
    k: i32,
    a: &[f64],
    lda: i32,
    tau: &[f64],
    c: &mut [f64],
    ldc: i32,
    work: &mut [f64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::dormbr_(
        &(vect as c_char),
        &(side as c_char),
        &(trans as c_char),
        &m,
        &n,
        &k,
        a.as_ptr(),
        &lda,
        tau.as_ptr(),
        c.as_mut_ptr(),
        &ldc,
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn sormbr(
    vect: u8,
    side: u8,
    trans: u8,
    m: i32,
    n: i32,
    k: i32,
    a: &[f32],
    lda: i32,
    tau: &[f32],
    c: &mut [f32],
    ldc: i32,
    work: &mut [f32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::sormbr_(
        &(vect as c_char),
        &(side as c_char),
        &(trans as c_char),
        &m,
        &n,
        &k,
        a.as_ptr(),
        &lda,
        tau.as_ptr(),
        c.as_mut_ptr(),
        &ldc,
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn dormhr(
    side: u8,
    trans: u8,
    m: i32,
    n: i32,
    ilo: i32,
    ihi: i32,
    a: &[f64],
    lda: i32,
    tau: &[f64],
    c: &mut [f64],
    ldc: i32,
    work: &mut [f64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::dormhr_(
        &(side as c_char),
        &(trans as c_char),
        &m,
        &n,
        &ilo,
        &ihi,
        a.as_ptr(),
        &lda,
        tau.as_ptr(),
        c.as_mut_ptr(),
        &ldc,
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn sormhr(
    side: u8,
    trans: u8,
    m: i32,
    n: i32,
    ilo: i32,
    ihi: i32,
    a: &[f32],
    lda: i32,
    tau: &[f32],
    c: &mut [f32],
    ldc: i32,
    work: &mut [f32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::sormhr_(
        &(side as c_char),
        &(trans as c_char),
        &m,
        &n,
        &ilo,
        &ihi,
        a.as_ptr(),
        &lda,
        tau.as_ptr(),
        c.as_mut_ptr(),
        &ldc,
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn dormlq(
    side: u8,
    trans: u8,
    m: i32,
    n: i32,
    k: i32,
    a: &[f64],
    lda: i32,
    tau: &[f64],
    c: &mut [f64],
    ldc: i32,
    work: &mut [f64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::dormlq_(
        &(side as c_char),
        &(trans as c_char),
        &m,
        &n,
        &k,
        a.as_ptr(),
        &lda,
        tau.as_ptr(),
        c.as_mut_ptr(),
        &ldc,
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn sormlq(
    side: u8,
    trans: u8,
    m: i32,
    n: i32,
    k: i32,
    a: &[f32],
    lda: i32,
    tau: &[f32],
    c: &mut [f32],
    ldc: i32,
    work: &mut [f32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::sormlq_(
        &(side as c_char),
        &(trans as c_char),
        &m,
        &n,
        &k,
        a.as_ptr(),
        &lda,
        tau.as_ptr(),
        c.as_mut_ptr(),
        &ldc,
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn dormql(
    side: u8,
    trans: u8,
    m: i32,
    n: i32,
    k: i32,
    a: &[f64],
    lda: i32,
    tau: &[f64],
    c: &mut [f64],
    ldc: i32,
    work: &mut [f64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::dormql_(
        &(side as c_char),
        &(trans as c_char),
        &m,
        &n,
        &k,
        a.as_ptr(),
        &lda,
        tau.as_ptr(),
        c.as_mut_ptr(),
        &ldc,
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn sormql(
    side: u8,
    trans: u8,
    m: i32,
    n: i32,
    k: i32,
    a: &[f32],
    lda: i32,
    tau: &[f32],
    c: &mut [f32],
    ldc: i32,
    work: &mut [f32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::sormql_(
        &(side as c_char),
        &(trans as c_char),
        &m,
        &n,
        &k,
        a.as_ptr(),
        &lda,
        tau.as_ptr(),
        c.as_mut_ptr(),
        &ldc,
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn dormqr(
    side: u8,
    trans: u8,
    m: i32,
    n: i32,
    k: i32,
    a: &[f64],
    lda: i32,
    tau: &[f64],
    c: &mut [f64],
    ldc: i32,
    work: &mut [f64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::dormqr_(
        &(side as c_char),
        &(trans as c_char),
        &m,
        &n,
        &k,
        a.as_ptr(),
        &lda,
        tau.as_ptr(),
        c.as_mut_ptr(),
        &ldc,
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn sormqr(
    side: u8,
    trans: u8,
    m: i32,
    n: i32,
    k: i32,
    a: &[f32],
    lda: i32,
    tau: &[f32],
    c: &mut [f32],
    ldc: i32,
    work: &mut [f32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::sormqr_(
        &(side as c_char),
        &(trans as c_char),
        &m,
        &n,
        &k,
        a.as_ptr(),
        &lda,
        tau.as_ptr(),
        c.as_mut_ptr(),
        &ldc,
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn dormrq(
    side: u8,
    trans: u8,
    m: i32,
    n: i32,
    k: i32,
    a: &[f64],
    lda: i32,
    tau: &[f64],
    c: &mut [f64],
    ldc: i32,
    work: &mut [f64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::dormrq_(
        &(side as c_char),
        &(trans as c_char),
        &m,
        &n,
        &k,
        a.as_ptr(),
        &lda,
        tau.as_ptr(),
        c.as_mut_ptr(),
        &ldc,
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn sormrq(
    side: u8,
    trans: u8,
    m: i32,
    n: i32,
    k: i32,
    a: &[f32],
    lda: i32,
    tau: &[f32],
    c: &mut [f32],
    ldc: i32,
    work: &mut [f32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::sormrq_(
        &(side as c_char),
        &(trans as c_char),
        &m,
        &n,
        &k,
        a.as_ptr(),
        &lda,
        tau.as_ptr(),
        c.as_mut_ptr(),
        &ldc,
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn dormrz(
    side: u8,
    trans: u8,
    m: i32,
    n: i32,
    k: i32,
    l: i32,
    a: &[f64],
    lda: i32,
    tau: &[f64],
    c: &mut [f64],
    ldc: i32,
    work: &mut [f64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::dormrz_(
        &(side as c_char),
        &(trans as c_char),
        &m,
        &n,
        &k,
        &l,
        a.as_ptr(),
        &lda,
        tau.as_ptr(),
        c.as_mut_ptr(),
        &ldc,
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn sormrz(
    side: u8,
    trans: u8,
    m: i32,
    n: i32,
    k: i32,
    l: i32,
    a: &[f32],
    lda: i32,
    tau: &[f32],
    c: &mut [f32],
    ldc: i32,
    work: &mut [f32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::sormrz_(
        &(side as c_char),
        &(trans as c_char),
        &m,
        &n,
        &k,
        &l,
        a.as_ptr(),
        &lda,
        tau.as_ptr(),
        c.as_mut_ptr(),
        &ldc,
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn dormtr(
    side: u8,
    uplo: u8,
    trans: u8,
    m: i32,
    n: i32,
    a: &[f64],
    lda: i32,
    tau: &[f64],
    c: &mut [f64],
    ldc: i32,
    work: &mut [f64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::dormtr_(
        &(side as c_char),
        &(uplo as c_char),
        &(trans as c_char),
        &m,
        &n,
        a.as_ptr(),
        &lda,
        tau.as_ptr(),
        c.as_mut_ptr(),
        &ldc,
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn sormtr(
    side: u8,
    uplo: u8,
    trans: u8,
    m: i32,
    n: i32,
    a: &[f32],
    lda: i32,
    tau: &[f32],
    c: &mut [f32],
    ldc: i32,
    work: &mut [f32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::sormtr_(
        &(side as c_char),
        &(uplo as c_char),
        &(trans as c_char),
        &m,
        &n,
        a.as_ptr(),
        &lda,
        tau.as_ptr(),
        c.as_mut_ptr(),
        &ldc,
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn cpbcon(
    uplo: u8,
    n: i32,
    kd: i32,
    ab: &[c32],
    ldab: i32,
    anorm: f32,
    rcond: &mut f32,
    work: &mut [c32],
    rwork: &mut [f32],
    info: &mut i32,
) {
    ffi::cpbcon_(
        &(uplo as c_char),
        &n,
        &kd,
        ab.as_ptr() as *const _,
        &ldab,
        &anorm,
        rcond,
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn dpbcon(
    uplo: u8,
    n: i32,
    kd: i32,
    ab: &[f64],
    ldab: i32,
    anorm: f64,
    rcond: &mut f64,
    work: &mut [f64],
    iwork: &mut [i32],
    info: &mut i32,
) {
    ffi::dpbcon_(
        &(uplo as c_char),
        &n,
        &kd,
        ab.as_ptr(),
        &ldab,
        &anorm,
        rcond,
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn spbcon(
    uplo: u8,
    n: i32,
    kd: i32,
    ab: &[f32],
    ldab: i32,
    anorm: f32,
    rcond: &mut f32,
    work: &mut [f32],
    iwork: &mut [i32],
    info: &mut i32,
) {
    ffi::spbcon_(
        &(uplo as c_char),
        &n,
        &kd,
        ab.as_ptr(),
        &ldab,
        &anorm,
        rcond,
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn zpbcon(
    uplo: u8,
    n: i32,
    kd: i32,
    ab: &[c64],
    ldab: i32,
    anorm: f64,
    rcond: &mut f64,
    work: &mut [c64],
    rwork: &mut [f64],
    info: &mut i32,
) {
    ffi::zpbcon_(
        &(uplo as c_char),
        &n,
        &kd,
        ab.as_ptr() as *const _,
        &ldab,
        &anorm,
        rcond,
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn cpbequ(
    uplo: u8,
    n: i32,
    kd: i32,
    ab: &[c32],
    ldab: i32,
    s: &mut [f32],
    scond: &mut [f32],
    amax: &mut f32,
    info: &mut i32,
) {
    ffi::cpbequ_(
        &(uplo as c_char),
        &n,
        &kd,
        ab.as_ptr() as *const _,
        &ldab,
        s.as_mut_ptr(),
        scond.as_mut_ptr(),
        amax,
        info,
    )
}

#[inline]
pub unsafe fn dpbequ(
    uplo: u8,
    n: i32,
    kd: i32,
    ab: &[f64],
    ldab: i32,
    s: &mut [f64],
    scond: &mut [f64],
    amax: &mut f64,
    info: &mut i32,
) {
    ffi::dpbequ_(
        &(uplo as c_char),
        &n,
        &kd,
        ab.as_ptr(),
        &ldab,
        s.as_mut_ptr(),
        scond.as_mut_ptr(),
        amax,
        info,
    )
}

#[inline]
pub unsafe fn spbequ(
    uplo: u8,
    n: i32,
    kd: i32,
    ab: &[f32],
    ldab: i32,
    s: &mut [f32],
    scond: &mut [f32],
    amax: &mut f32,
    info: &mut i32,
) {
    ffi::spbequ_(
        &(uplo as c_char),
        &n,
        &kd,
        ab.as_ptr(),
        &ldab,
        s.as_mut_ptr(),
        scond.as_mut_ptr(),
        amax,
        info,
    )
}

#[inline]
pub unsafe fn zpbequ(
    uplo: u8,
    n: i32,
    kd: i32,
    ab: &[c64],
    ldab: i32,
    s: &mut [f64],
    scond: &mut [f64],
    amax: &mut f64,
    info: &mut i32,
) {
    ffi::zpbequ_(
        &(uplo as c_char),
        &n,
        &kd,
        ab.as_ptr() as *const _,
        &ldab,
        s.as_mut_ptr(),
        scond.as_mut_ptr(),
        amax,
        info,
    )
}

#[inline]
pub unsafe fn cpbrfs(
    uplo: u8,
    n: i32,
    kd: i32,
    nrhs: i32,
    ab: &[c32],
    ldab: i32,
    afb: &[c32],
    ldafb: i32,
    b: &[c32],
    ldb: i32,
    x: &mut [c32],
    ldx: i32,
    ferr: &mut [f32],
    berr: &mut [f32],
    work: &mut [c32],
    rwork: &mut [f32],
    info: &mut i32,
) {
    ffi::cpbrfs_(
        &(uplo as c_char),
        &n,
        &kd,
        &nrhs,
        ab.as_ptr() as *const _,
        &ldab,
        afb.as_ptr() as *const _,
        &ldafb,
        b.as_ptr() as *const _,
        &ldb,
        x.as_mut_ptr() as *mut _,
        &ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn dpbrfs(
    uplo: u8,
    n: i32,
    kd: i32,
    nrhs: i32,
    ab: &[f64],
    ldab: i32,
    afb: &[f64],
    ldafb: i32,
    b: &[f64],
    ldb: i32,
    x: &mut [f64],
    ldx: i32,
    ferr: &mut [f64],
    berr: &mut [f64],
    work: &mut [f64],
    iwork: &mut [i32],
    info: &mut i32,
) {
    ffi::dpbrfs_(
        &(uplo as c_char),
        &n,
        &kd,
        &nrhs,
        ab.as_ptr(),
        &ldab,
        afb.as_ptr(),
        &ldafb,
        b.as_ptr(),
        &ldb,
        x.as_mut_ptr(),
        &ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn spbrfs(
    uplo: u8,
    n: i32,
    kd: i32,
    nrhs: i32,
    ab: &[f32],
    ldab: i32,
    afb: &[f32],
    ldafb: i32,
    b: &[f32],
    ldb: i32,
    x: &mut [f32],
    ldx: i32,
    ferr: &mut [f32],
    berr: &mut [f32],
    work: &mut [f32],
    iwork: &mut [i32],
    info: &mut i32,
) {
    ffi::spbrfs_(
        &(uplo as c_char),
        &n,
        &kd,
        &nrhs,
        ab.as_ptr(),
        &ldab,
        afb.as_ptr(),
        &ldafb,
        b.as_ptr(),
        &ldb,
        x.as_mut_ptr(),
        &ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn zpbrfs(
    uplo: u8,
    n: i32,
    kd: i32,
    nrhs: i32,
    ab: &[c64],
    ldab: i32,
    afb: &[c64],
    ldafb: i32,
    b: &[c64],
    ldb: i32,
    x: &mut [c64],
    ldx: i32,
    ferr: &mut [f64],
    berr: &mut [f64],
    work: &mut [c64],
    rwork: &mut [f64],
    info: &mut i32,
) {
    ffi::zpbrfs_(
        &(uplo as c_char),
        &n,
        &kd,
        &nrhs,
        ab.as_ptr() as *const _,
        &ldab,
        afb.as_ptr() as *const _,
        &ldafb,
        b.as_ptr() as *const _,
        &ldb,
        x.as_mut_ptr() as *mut _,
        &ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn cpbstf(uplo: u8, n: i32, kd: i32, ab: &mut [c32], ldab: i32, info: &mut i32) {
    ffi::cpbstf_(
        &(uplo as c_char),
        &n,
        &kd,
        ab.as_mut_ptr() as *mut _,
        &ldab,
        info,
    )
}

#[inline]
pub unsafe fn dpbstf(uplo: u8, n: i32, kd: i32, ab: &mut [f64], ldab: i32, info: &mut i32) {
    ffi::dpbstf_(&(uplo as c_char), &n, &kd, ab.as_mut_ptr(), &ldab, info)
}

#[inline]
pub unsafe fn spbstf(uplo: u8, n: i32, kd: i32, ab: &mut [f32], ldab: i32, info: &mut i32) {
    ffi::spbstf_(&(uplo as c_char), &n, &kd, ab.as_mut_ptr(), &ldab, info)
}

#[inline]
pub unsafe fn zpbstf(uplo: u8, n: i32, kd: i32, ab: &mut [c64], ldab: i32, info: &mut i32) {
    ffi::zpbstf_(
        &(uplo as c_char),
        &n,
        &kd,
        ab.as_mut_ptr() as *mut _,
        &ldab,
        info,
    )
}

#[inline]
pub unsafe fn cpbsv(
    uplo: u8,
    n: i32,
    kd: i32,
    nrhs: i32,
    ab: &mut [c32],
    ldab: i32,
    b: &mut [c32],
    ldb: i32,
    info: &mut i32,
) {
    ffi::cpbsv_(
        &(uplo as c_char),
        &n,
        &kd,
        &nrhs,
        ab.as_mut_ptr() as *mut _,
        &ldab,
        b.as_mut_ptr() as *mut _,
        &ldb,
        info,
    )
}

#[inline]
pub unsafe fn dpbsv(
    uplo: u8,
    n: i32,
    kd: i32,
    nrhs: i32,
    ab: &mut [f64],
    ldab: i32,
    b: &mut [f64],
    ldb: i32,
    info: &mut i32,
) {
    ffi::dpbsv_(
        &(uplo as c_char),
        &n,
        &kd,
        &nrhs,
        ab.as_mut_ptr(),
        &ldab,
        b.as_mut_ptr(),
        &ldb,
        info,
    )
}

#[inline]
pub unsafe fn spbsv(
    uplo: u8,
    n: i32,
    kd: i32,
    nrhs: i32,
    ab: &mut [f32],
    ldab: i32,
    b: &mut [f32],
    ldb: i32,
    info: &mut i32,
) {
    ffi::spbsv_(
        &(uplo as c_char),
        &n,
        &kd,
        &nrhs,
        ab.as_mut_ptr(),
        &ldab,
        b.as_mut_ptr(),
        &ldb,
        info,
    )
}

#[inline]
pub unsafe fn zpbsv(
    uplo: u8,
    n: i32,
    kd: i32,
    nrhs: i32,
    ab: &mut [c64],
    ldab: i32,
    b: &mut [c64],
    ldb: i32,
    info: &mut i32,
) {
    ffi::zpbsv_(
        &(uplo as c_char),
        &n,
        &kd,
        &nrhs,
        ab.as_mut_ptr() as *mut _,
        &ldab,
        b.as_mut_ptr() as *mut _,
        &ldb,
        info,
    )
}

#[inline]
pub unsafe fn cpbsvx(
    fact: u8,
    uplo: u8,
    n: i32,
    kd: i32,
    nrhs: i32,
    ab: &mut [c32],
    ldab: i32,
    afb: &mut [c32],
    ldafb: i32,
    equed: &mut u8,
    s: &mut [f32],
    b: &mut [c32],
    ldb: i32,
    x: &mut [c32],
    ldx: i32,
    rcond: &mut f32,
    ferr: &mut [f32],
    berr: &mut [f32],
    work: &mut [c32],
    rwork: &mut [f32],
    info: &mut i32,
) {
    ffi::cpbsvx_(
        &(fact as c_char),
        &(uplo as c_char),
        &n,
        &kd,
        &nrhs,
        ab.as_mut_ptr() as *mut _,
        &ldab,
        afb.as_mut_ptr() as *mut _,
        &ldafb,
        equed as *mut _ as *mut _,
        s.as_mut_ptr(),
        b.as_mut_ptr() as *mut _,
        &ldb,
        x.as_mut_ptr() as *mut _,
        &ldx,
        rcond,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn dpbsvx(
    fact: u8,
    uplo: u8,
    n: i32,
    kd: i32,
    nrhs: i32,
    ab: &mut [f64],
    ldab: i32,
    afb: &mut [f64],
    ldafb: i32,
    equed: &mut u8,
    s: &mut [f64],
    b: &mut [f64],
    ldb: i32,
    x: &mut [f64],
    ldx: i32,
    rcond: &mut f64,
    ferr: &mut [f64],
    berr: &mut [f64],
    work: &mut [f64],
    iwork: &mut [i32],
    info: &mut i32,
) {
    ffi::dpbsvx_(
        &(fact as c_char),
        &(uplo as c_char),
        &n,
        &kd,
        &nrhs,
        ab.as_mut_ptr(),
        &ldab,
        afb.as_mut_ptr(),
        &ldafb,
        equed as *mut _ as *mut _,
        s.as_mut_ptr(),
        b.as_mut_ptr(),
        &ldb,
        x.as_mut_ptr(),
        &ldx,
        rcond,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn spbsvx(
    fact: u8,
    uplo: u8,
    n: i32,
    kd: i32,
    nrhs: i32,
    ab: &mut [f32],
    ldab: i32,
    afb: &mut [f32],
    ldafb: i32,
    equed: &mut u8,
    s: &mut [f32],
    b: &mut [f32],
    ldb: i32,
    x: &mut [f32],
    ldx: i32,
    rcond: &mut f32,
    ferr: &mut [f32],
    berr: &mut [f32],
    work: &mut [f32],
    iwork: &mut [i32],
    info: &mut i32,
) {
    ffi::spbsvx_(
        &(fact as c_char),
        &(uplo as c_char),
        &n,
        &kd,
        &nrhs,
        ab.as_mut_ptr(),
        &ldab,
        afb.as_mut_ptr(),
        &ldafb,
        equed as *mut _ as *mut _,
        s.as_mut_ptr(),
        b.as_mut_ptr(),
        &ldb,
        x.as_mut_ptr(),
        &ldx,
        rcond,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn zpbsvx(
    fact: u8,
    uplo: u8,
    n: i32,
    kd: i32,
    nrhs: i32,
    ab: &mut [c64],
    ldab: i32,
    afb: &mut [c64],
    ldafb: i32,
    equed: &mut u8,
    s: &mut [f64],
    b: &mut [c64],
    ldb: i32,
    x: &mut [c64],
    ldx: i32,
    rcond: &mut f64,
    ferr: &mut [f64],
    berr: &mut [f64],
    work: &mut [c64],
    rwork: &mut [f64],
    info: &mut i32,
) {
    ffi::zpbsvx_(
        &(fact as c_char),
        &(uplo as c_char),
        &n,
        &kd,
        &nrhs,
        ab.as_mut_ptr() as *mut _,
        &ldab,
        afb.as_mut_ptr() as *mut _,
        &ldafb,
        equed as *mut _ as *mut _,
        s.as_mut_ptr(),
        b.as_mut_ptr() as *mut _,
        &ldb,
        x.as_mut_ptr() as *mut _,
        &ldx,
        rcond,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn cpbtrf(uplo: u8, n: i32, kd: i32, ab: &mut [c32], ldab: i32, info: &mut i32) {
    ffi::cpbtrf_(
        &(uplo as c_char),
        &n,
        &kd,
        ab.as_mut_ptr() as *mut _,
        &ldab,
        info,
    )
}

#[inline]
pub unsafe fn dpbtrf(uplo: u8, n: i32, kd: i32, ab: &mut [f64], ldab: i32, info: &mut i32) {
    ffi::dpbtrf_(&(uplo as c_char), &n, &kd, ab.as_mut_ptr(), &ldab, info)
}

#[inline]
pub unsafe fn spbtrf(uplo: u8, n: i32, kd: i32, ab: &mut [f32], ldab: i32, info: &mut i32) {
    ffi::spbtrf_(&(uplo as c_char), &n, &kd, ab.as_mut_ptr(), &ldab, info)
}

#[inline]
pub unsafe fn zpbtrf(uplo: u8, n: i32, kd: i32, ab: &mut [c64], ldab: i32, info: &mut i32) {
    ffi::zpbtrf_(
        &(uplo as c_char),
        &n,
        &kd,
        ab.as_mut_ptr() as *mut _,
        &ldab,
        info,
    )
}

#[inline]
pub unsafe fn cpbtrs(
    uplo: u8,
    n: i32,
    kd: i32,
    nrhs: i32,
    ab: &[c32],
    ldab: i32,
    b: &mut [c32],
    ldb: i32,
    info: &mut i32,
) {
    ffi::cpbtrs_(
        &(uplo as c_char),
        &n,
        &kd,
        &nrhs,
        ab.as_ptr() as *const _,
        &ldab,
        b.as_mut_ptr() as *mut _,
        &ldb,
        info,
    )
}

#[inline]
pub unsafe fn dpbtrs(
    uplo: u8,
    n: i32,
    kd: i32,
    nrhs: i32,
    ab: &[f64],
    ldab: i32,
    b: &mut [f64],
    ldb: i32,
    info: &mut i32,
) {
    ffi::dpbtrs_(
        &(uplo as c_char),
        &n,
        &kd,
        &nrhs,
        ab.as_ptr(),
        &ldab,
        b.as_mut_ptr(),
        &ldb,
        info,
    )
}

#[inline]
pub unsafe fn spbtrs(
    uplo: u8,
    n: i32,
    kd: i32,
    nrhs: i32,
    ab: &[f32],
    ldab: i32,
    b: &mut [f32],
    ldb: i32,
    info: &mut i32,
) {
    ffi::spbtrs_(
        &(uplo as c_char),
        &n,
        &kd,
        &nrhs,
        ab.as_ptr(),
        &ldab,
        b.as_mut_ptr(),
        &ldb,
        info,
    )
}

#[inline]
pub unsafe fn zpbtrs(
    uplo: u8,
    n: i32,
    kd: i32,
    nrhs: i32,
    ab: &[c64],
    ldab: i32,
    b: &mut [c64],
    ldb: i32,
    info: &mut i32,
) {
    ffi::zpbtrs_(
        &(uplo as c_char),
        &n,
        &kd,
        &nrhs,
        ab.as_ptr() as *const _,
        &ldab,
        b.as_mut_ptr() as *mut _,
        &ldb,
        info,
    )
}

#[inline]
pub unsafe fn cpftrf(transr: u8, uplo: u8, n: i32, a: &mut [c32], info: &mut i32) {
    ffi::cpftrf_(
        &(transr as c_char),
        &(uplo as c_char),
        &n,
        a.as_mut_ptr() as *mut _,
        info,
    )
}

#[inline]
pub unsafe fn dpftrf(transr: u8, uplo: u8, n: i32, a: &mut [f64], info: &mut i32) {
    ffi::dpftrf_(
        &(transr as c_char),
        &(uplo as c_char),
        &n,
        a.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn spftrf(transr: u8, uplo: u8, n: i32, a: &mut [f32], info: &mut i32) {
    ffi::spftrf_(
        &(transr as c_char),
        &(uplo as c_char),
        &n,
        a.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn zpftrf(transr: u8, uplo: u8, n: i32, a: &mut [c64], info: &mut i32) {
    ffi::zpftrf_(
        &(transr as c_char),
        &(uplo as c_char),
        &n,
        a.as_mut_ptr() as *mut _,
        info,
    )
}

#[inline]
pub unsafe fn cpftri(transr: u8, uplo: u8, n: i32, a: &mut [c32], info: &mut i32) {
    ffi::cpftri_(
        &(transr as c_char),
        &(uplo as c_char),
        &n,
        a.as_mut_ptr() as *mut _,
        info,
    )
}

#[inline]
pub unsafe fn dpftri(transr: u8, uplo: u8, n: i32, a: &mut [f64], info: &mut i32) {
    ffi::dpftri_(
        &(transr as c_char),
        &(uplo as c_char),
        &n,
        a.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn spftri(transr: u8, uplo: u8, n: i32, a: &mut [f32], info: &mut i32) {
    ffi::spftri_(
        &(transr as c_char),
        &(uplo as c_char),
        &n,
        a.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn zpftri(transr: u8, uplo: u8, n: i32, a: &mut [c64], info: &mut i32) {
    ffi::zpftri_(
        &(transr as c_char),
        &(uplo as c_char),
        &n,
        a.as_mut_ptr() as *mut _,
        info,
    )
}

#[inline]
pub unsafe fn cpftrs(
    transr: u8,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &[c32],
    b: &mut [c32],
    ldb: i32,
    info: &mut i32,
) {
    ffi::cpftrs_(
        &(transr as c_char),
        &(uplo as c_char),
        &n,
        &nrhs,
        a.as_ptr() as *const _,
        b.as_mut_ptr() as *mut _,
        &ldb,
        info,
    )
}

#[inline]
pub unsafe fn dpftrs(
    transr: u8,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &[f64],
    b: &mut [f64],
    ldb: i32,
    info: &mut i32,
) {
    ffi::dpftrs_(
        &(transr as c_char),
        &(uplo as c_char),
        &n,
        &nrhs,
        a.as_ptr(),
        b.as_mut_ptr(),
        &ldb,
        info,
    )
}

#[inline]
pub unsafe fn spftrs(
    transr: u8,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &[f32],
    b: &mut [f32],
    ldb: i32,
    info: &mut i32,
) {
    ffi::spftrs_(
        &(transr as c_char),
        &(uplo as c_char),
        &n,
        &nrhs,
        a.as_ptr(),
        b.as_mut_ptr(),
        &ldb,
        info,
    )
}

#[inline]
pub unsafe fn zpftrs(
    transr: u8,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &[c64],
    b: &mut [c64],
    ldb: i32,
    info: &mut i32,
) {
    ffi::zpftrs_(
        &(transr as c_char),
        &(uplo as c_char),
        &n,
        &nrhs,
        a.as_ptr() as *const _,
        b.as_mut_ptr() as *mut _,
        &ldb,
        info,
    )
}

#[inline]
pub unsafe fn cpocon(
    uplo: u8,
    n: i32,
    a: &[c32],
    lda: i32,
    anorm: f32,
    rcond: &mut f32,
    work: &mut [c32],
    rwork: &mut [f32],
    info: &mut i32,
) {
    ffi::cpocon_(
        &(uplo as c_char),
        &n,
        a.as_ptr() as *const _,
        &lda,
        &anorm,
        rcond,
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn dpocon(
    uplo: u8,
    n: i32,
    a: &[f64],
    lda: i32,
    anorm: f64,
    rcond: &mut f64,
    work: &mut [f64],
    iwork: &mut [i32],
    info: &mut i32,
) {
    ffi::dpocon_(
        &(uplo as c_char),
        &n,
        a.as_ptr(),
        &lda,
        &anorm,
        rcond,
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn spocon(
    uplo: u8,
    n: i32,
    a: &[f32],
    lda: i32,
    anorm: f32,
    rcond: &mut f32,
    work: &mut [f32],
    iwork: &mut [i32],
    info: &mut i32,
) {
    ffi::spocon_(
        &(uplo as c_char),
        &n,
        a.as_ptr(),
        &lda,
        &anorm,
        rcond,
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn zpocon(
    uplo: u8,
    n: i32,
    a: &[c64],
    lda: i32,
    anorm: f64,
    rcond: &mut f64,
    work: &mut [c64],
    rwork: &mut [f64],
    info: &mut i32,
) {
    ffi::zpocon_(
        &(uplo as c_char),
        &n,
        a.as_ptr() as *const _,
        &lda,
        &anorm,
        rcond,
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn cpoequ(
    n: i32,
    a: &[c32],
    lda: i32,
    s: &mut [f32],
    scond: &mut [f32],
    amax: &mut f32,
    info: &mut i32,
) {
    ffi::cpoequ_(
        &n,
        a.as_ptr() as *const _,
        &lda,
        s.as_mut_ptr(),
        scond.as_mut_ptr(),
        amax,
        info,
    )
}

#[inline]
pub unsafe fn dpoequ(
    n: i32,
    a: &[f64],
    lda: i32,
    s: &mut [f64],
    scond: &mut [f64],
    amax: &mut f64,
    info: &mut i32,
) {
    ffi::dpoequ_(
        &n,
        a.as_ptr(),
        &lda,
        s.as_mut_ptr(),
        scond.as_mut_ptr(),
        amax,
        info,
    )
}

#[inline]
pub unsafe fn spoequ(
    n: i32,
    a: &[f32],
    lda: i32,
    s: &mut [f32],
    scond: &mut [f32],
    amax: &mut f32,
    info: &mut i32,
) {
    ffi::spoequ_(
        &n,
        a.as_ptr(),
        &lda,
        s.as_mut_ptr(),
        scond.as_mut_ptr(),
        amax,
        info,
    )
}

#[inline]
pub unsafe fn zpoequ(
    n: i32,
    a: &[c64],
    lda: i32,
    s: &mut [f64],
    scond: &mut [f64],
    amax: &mut f64,
    info: &mut i32,
) {
    ffi::zpoequ_(
        &n,
        a.as_ptr() as *const _,
        &lda,
        s.as_mut_ptr(),
        scond.as_mut_ptr(),
        amax,
        info,
    )
}

#[inline]
pub unsafe fn cpoequb(
    n: i32,
    a: &[c32],
    lda: i32,
    s: &mut [f32],
    scond: &mut [f32],
    amax: &mut f32,
    info: &mut i32,
) {
    ffi::cpoequb_(
        &n,
        a.as_ptr() as *const _,
        &lda,
        s.as_mut_ptr(),
        scond.as_mut_ptr(),
        amax,
        info,
    )
}

#[inline]
pub unsafe fn dpoequb(
    n: i32,
    a: &[f64],
    lda: i32,
    s: &mut [f64],
    scond: &mut [f64],
    amax: &mut f64,
    info: &mut i32,
) {
    ffi::dpoequb_(
        &n,
        a.as_ptr(),
        &lda,
        s.as_mut_ptr(),
        scond.as_mut_ptr(),
        amax,
        info,
    )
}

#[inline]
pub unsafe fn spoequb(
    n: i32,
    a: &[f32],
    lda: i32,
    s: &mut [f32],
    scond: &mut [f32],
    amax: &mut f32,
    info: &mut i32,
) {
    ffi::spoequb_(
        &n,
        a.as_ptr(),
        &lda,
        s.as_mut_ptr(),
        scond.as_mut_ptr(),
        amax,
        info,
    )
}

#[inline]
pub unsafe fn zpoequb(
    n: i32,
    a: &[c64],
    lda: i32,
    s: &mut [f64],
    scond: &mut [f64],
    amax: &mut f64,
    info: &mut i32,
) {
    ffi::zpoequb_(
        &n,
        a.as_ptr() as *const _,
        &lda,
        s.as_mut_ptr(),
        scond.as_mut_ptr(),
        amax,
        info,
    )
}

#[inline]
pub unsafe fn cporfs(
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &[c32],
    lda: i32,
    af: &[c32],
    ldaf: i32,
    b: &[c32],
    ldb: i32,
    x: &mut [c32],
    ldx: i32,
    ferr: &mut [f32],
    berr: &mut [f32],
    work: &mut [c32],
    rwork: &mut [f32],
    info: &mut i32,
) {
    ffi::cporfs_(
        &(uplo as c_char),
        &n,
        &nrhs,
        a.as_ptr() as *const _,
        &lda,
        af.as_ptr() as *const _,
        &ldaf,
        b.as_ptr() as *const _,
        &ldb,
        x.as_mut_ptr() as *mut _,
        &ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn dporfs(
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &[f64],
    lda: i32,
    af: &[f64],
    ldaf: i32,
    b: &[f64],
    ldb: i32,
    x: &mut [f64],
    ldx: i32,
    ferr: &mut [f64],
    berr: &mut [f64],
    work: &mut [f64],
    iwork: &mut [i32],
    info: &mut i32,
) {
    ffi::dporfs_(
        &(uplo as c_char),
        &n,
        &nrhs,
        a.as_ptr(),
        &lda,
        af.as_ptr(),
        &ldaf,
        b.as_ptr(),
        &ldb,
        x.as_mut_ptr(),
        &ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn sporfs(
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &[f32],
    lda: i32,
    af: &[f32],
    ldaf: i32,
    b: &[f32],
    ldb: i32,
    x: &mut [f32],
    ldx: i32,
    ferr: &mut [f32],
    berr: &mut [f32],
    work: &mut [f32],
    iwork: &mut [i32],
    info: &mut i32,
) {
    ffi::sporfs_(
        &(uplo as c_char),
        &n,
        &nrhs,
        a.as_ptr(),
        &lda,
        af.as_ptr(),
        &ldaf,
        b.as_ptr(),
        &ldb,
        x.as_mut_ptr(),
        &ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn zporfs(
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &[c64],
    lda: i32,
    af: &[c64],
    ldaf: i32,
    b: &[c64],
    ldb: i32,
    x: &mut [c64],
    ldx: i32,
    ferr: &mut [f64],
    berr: &mut [f64],
    work: &mut [c64],
    rwork: &mut [f64],
    info: &mut i32,
) {
    ffi::zporfs_(
        &(uplo as c_char),
        &n,
        &nrhs,
        a.as_ptr() as *const _,
        &lda,
        af.as_ptr() as *const _,
        &ldaf,
        b.as_ptr() as *const _,
        &ldb,
        x.as_mut_ptr() as *mut _,
        &ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn cporfsx(
    uplo: u8,
    equed: u8,
    n: i32,
    nrhs: i32,
    a: &[c32],
    lda: i32,
    af: &[c32],
    ldaf: i32,
    s: &mut [f32],
    b: &[c32],
    ldb: i32,
    x: &mut [c32],
    ldx: i32,
    rcond: &mut f32,
    berr: &mut [f32],
    n_err_bnds: i32,
    err_bnds_norm: &mut [f32],
    err_bnds_comp: &mut [f32],
    nparams: &[i32],
    params: &mut [f32],
    work: &mut [c32],
    rwork: &mut [f32],
    info: &mut i32,
) {
    ffi::cporfsx_(
        &(uplo as c_char),
        &(equed as c_char),
        &n,
        &nrhs,
        a.as_ptr() as *const _,
        &lda,
        af.as_ptr() as *const _,
        &ldaf,
        s.as_mut_ptr(),
        b.as_ptr() as *const _,
        &ldb,
        x.as_mut_ptr() as *mut _,
        &ldx,
        rcond,
        berr.as_mut_ptr(),
        &n_err_bnds,
        err_bnds_norm.as_mut_ptr(),
        err_bnds_comp.as_mut_ptr(),
        nparams.as_ptr(),
        params.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn dporfsx(
    uplo: u8,
    equed: u8,
    n: i32,
    nrhs: i32,
    a: &[f64],
    lda: i32,
    af: &[f64],
    ldaf: i32,
    s: &mut [f64],
    b: &[f64],
    ldb: i32,
    x: &mut [f64],
    ldx: i32,
    rcond: &mut f64,
    berr: &mut [f64],
    n_err_bnds: i32,
    err_bnds_norm: &mut [f64],
    err_bnds_comp: &mut [f64],
    nparams: &[i32],
    params: &mut [f64],
    work: &mut [f64],
    iwork: &mut [i32],
    info: &mut i32,
) {
    ffi::dporfsx_(
        &(uplo as c_char),
        &(equed as c_char),
        &n,
        &nrhs,
        a.as_ptr(),
        &lda,
        af.as_ptr(),
        &ldaf,
        s.as_mut_ptr(),
        b.as_ptr(),
        &ldb,
        x.as_mut_ptr(),
        &ldx,
        rcond,
        berr.as_mut_ptr(),
        &n_err_bnds,
        err_bnds_norm.as_mut_ptr(),
        err_bnds_comp.as_mut_ptr(),
        nparams.as_ptr(),
        params.as_mut_ptr(),
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn sporfsx(
    uplo: u8,
    equed: u8,
    n: i32,
    nrhs: i32,
    a: &[f32],
    lda: i32,
    af: &[f32],
    ldaf: i32,
    s: &mut [f32],
    b: &[f32],
    ldb: i32,
    x: &mut [f32],
    ldx: i32,
    rcond: &mut f32,
    berr: &mut [f32],
    n_err_bnds: i32,
    err_bnds_norm: &mut [f32],
    err_bnds_comp: &mut [f32],
    nparams: &[i32],
    params: &mut [f32],
    work: &mut [f32],
    iwork: &mut [i32],
    info: &mut i32,
) {
    ffi::sporfsx_(
        &(uplo as c_char),
        &(equed as c_char),
        &n,
        &nrhs,
        a.as_ptr(),
        &lda,
        af.as_ptr(),
        &ldaf,
        s.as_mut_ptr(),
        b.as_ptr(),
        &ldb,
        x.as_mut_ptr(),
        &ldx,
        rcond,
        berr.as_mut_ptr(),
        &n_err_bnds,
        err_bnds_norm.as_mut_ptr(),
        err_bnds_comp.as_mut_ptr(),
        nparams.as_ptr(),
        params.as_mut_ptr(),
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn zporfsx(
    uplo: u8,
    equed: u8,
    n: i32,
    nrhs: i32,
    a: &[c64],
    lda: i32,
    af: &[c64],
    ldaf: i32,
    s: &mut [f64],
    b: &[c64],
    ldb: i32,
    x: &mut [c64],
    ldx: i32,
    rcond: &mut f64,
    berr: &mut [f64],
    n_err_bnds: i32,
    err_bnds_norm: &mut [f64],
    err_bnds_comp: &mut [f64],
    nparams: &[i32],
    params: &mut [f64],
    work: &mut [c64],
    rwork: &mut [f64],
    info: &mut i32,
) {
    ffi::zporfsx_(
        &(uplo as c_char),
        &(equed as c_char),
        &n,
        &nrhs,
        a.as_ptr() as *const _,
        &lda,
        af.as_ptr() as *const _,
        &ldaf,
        s.as_mut_ptr(),
        b.as_ptr() as *const _,
        &ldb,
        x.as_mut_ptr() as *mut _,
        &ldx,
        rcond,
        berr.as_mut_ptr(),
        &n_err_bnds,
        err_bnds_norm.as_mut_ptr(),
        err_bnds_comp.as_mut_ptr(),
        nparams.as_ptr(),
        params.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn cposv(
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &mut [c32],
    lda: i32,
    b: &mut [c32],
    ldb: i32,
    info: &mut i32,
) {
    ffi::cposv_(
        &(uplo as c_char),
        &n,
        &nrhs,
        a.as_mut_ptr() as *mut _,
        &lda,
        b.as_mut_ptr() as *mut _,
        &ldb,
        info,
    )
}

#[inline]
pub unsafe fn dposv(
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &mut [f64],
    lda: i32,
    b: &mut [f64],
    ldb: i32,
    info: &mut i32,
) {
    ffi::dposv_(
        &(uplo as c_char),
        &n,
        &nrhs,
        a.as_mut_ptr(),
        &lda,
        b.as_mut_ptr(),
        &ldb,
        info,
    )
}

#[inline]
pub unsafe fn sposv(
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &mut [f32],
    lda: i32,
    b: &mut [f32],
    ldb: i32,
    info: &mut i32,
) {
    ffi::sposv_(
        &(uplo as c_char),
        &n,
        &nrhs,
        a.as_mut_ptr(),
        &lda,
        b.as_mut_ptr(),
        &ldb,
        info,
    )
}

#[inline]
pub unsafe fn zposv(
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &mut [c64],
    lda: i32,
    b: &mut [c64],
    ldb: i32,
    info: &mut i32,
) {
    ffi::zposv_(
        &(uplo as c_char),
        &n,
        &nrhs,
        a.as_mut_ptr() as *mut _,
        &lda,
        b.as_mut_ptr() as *mut _,
        &ldb,
        info,
    )
}

#[inline]
pub unsafe fn dsposv(
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &mut [f64],
    lda: i32,
    b: &[f64],
    ldb: i32,
    x: &mut [f64],
    ldx: i32,
    work: &mut [f64],
    swork: &mut [f32],
    iter: &mut i32,
    info: &mut i32,
) {
    ffi::dsposv_(
        &(uplo as c_char),
        &n,
        &nrhs,
        a.as_mut_ptr(),
        &lda,
        b.as_ptr(),
        &ldb,
        x.as_mut_ptr(),
        &ldx,
        work.as_mut_ptr(),
        swork.as_mut_ptr(),
        iter,
        info,
    )
}

#[inline]
pub unsafe fn zcposv(
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &mut [c64],
    lda: i32,
    b: &[c64],
    ldb: i32,
    x: &mut [c64],
    ldx: i32,
    work: &mut [c64],
    swork: &mut [c32],
    rwork: &mut [f64],
    iter: &mut i32,
    info: &mut i32,
) {
    ffi::zcposv_(
        &(uplo as c_char),
        &n,
        &nrhs,
        a.as_mut_ptr() as *mut _,
        &lda,
        b.as_ptr() as *const _,
        &ldb,
        x.as_mut_ptr() as *mut _,
        &ldx,
        work.as_mut_ptr() as *mut _,
        swork.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
        iter,
        info,
    )
}

#[inline]
pub unsafe fn cposvx(
    fact: u8,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &mut [c32],
    lda: i32,
    af: &mut [c32],
    ldaf: i32,
    equed: &mut u8,
    s: &mut [f32],
    b: &mut [c32],
    ldb: i32,
    x: &mut [c32],
    ldx: i32,
    rcond: &mut f32,
    ferr: &mut [f32],
    berr: &mut [f32],
    work: &mut [c32],
    rwork: &mut [f32],
    info: &mut i32,
) {
    ffi::cposvx_(
        &(fact as c_char),
        &(uplo as c_char),
        &n,
        &nrhs,
        a.as_mut_ptr() as *mut _,
        &lda,
        af.as_mut_ptr() as *mut _,
        &ldaf,
        equed as *mut _ as *mut _,
        s.as_mut_ptr(),
        b.as_mut_ptr() as *mut _,
        &ldb,
        x.as_mut_ptr() as *mut _,
        &ldx,
        rcond,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn dposvx(
    fact: u8,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &mut [f64],
    lda: i32,
    af: &mut [f64],
    ldaf: i32,
    equed: &mut u8,
    s: &mut [f64],
    b: &mut [f64],
    ldb: i32,
    x: &mut [f64],
    ldx: i32,
    rcond: &mut f64,
    ferr: &mut [f64],
    berr: &mut [f64],
    work: &mut [f64],
    iwork: &mut [i32],
    info: &mut i32,
) {
    ffi::dposvx_(
        &(fact as c_char),
        &(uplo as c_char),
        &n,
        &nrhs,
        a.as_mut_ptr(),
        &lda,
        af.as_mut_ptr(),
        &ldaf,
        equed as *mut _ as *mut _,
        s.as_mut_ptr(),
        b.as_mut_ptr(),
        &ldb,
        x.as_mut_ptr(),
        &ldx,
        rcond,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn sposvx(
    fact: u8,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &mut [f32],
    lda: i32,
    af: &mut [f32],
    ldaf: i32,
    equed: &mut u8,
    s: &mut [f32],
    b: &mut [f32],
    ldb: i32,
    x: &mut [f32],
    ldx: i32,
    rcond: &mut f32,
    ferr: &mut [f32],
    berr: &mut [f32],
    work: &mut [f32],
    iwork: &mut [i32],
    info: &mut i32,
) {
    ffi::sposvx_(
        &(fact as c_char),
        &(uplo as c_char),
        &n,
        &nrhs,
        a.as_mut_ptr(),
        &lda,
        af.as_mut_ptr(),
        &ldaf,
        equed as *mut _ as *mut _,
        s.as_mut_ptr(),
        b.as_mut_ptr(),
        &ldb,
        x.as_mut_ptr(),
        &ldx,
        rcond,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn zposvx(
    fact: u8,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &mut [c64],
    lda: i32,
    af: &mut [c64],
    ldaf: i32,
    equed: &mut u8,
    s: &mut [f64],
    b: &mut [c64],
    ldb: i32,
    x: &mut [c64],
    ldx: i32,
    rcond: &mut f64,
    ferr: &mut [f64],
    berr: &mut [f64],
    work: &mut [c64],
    rwork: &mut [f64],
    info: &mut i32,
) {
    ffi::zposvx_(
        &(fact as c_char),
        &(uplo as c_char),
        &n,
        &nrhs,
        a.as_mut_ptr() as *mut _,
        &lda,
        af.as_mut_ptr() as *mut _,
        &ldaf,
        equed as *mut _ as *mut _,
        s.as_mut_ptr(),
        b.as_mut_ptr() as *mut _,
        &ldb,
        x.as_mut_ptr() as *mut _,
        &ldx,
        rcond,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn cposvxx(
    fact: u8,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &mut [c32],
    lda: i32,
    af: &mut [c32],
    ldaf: i32,
    equed: &mut u8,
    s: &mut [f32],
    b: &mut [c32],
    ldb: i32,
    x: &mut [c32],
    ldx: i32,
    rcond: &mut f32,
    rpvgrw: &mut f32,
    berr: &mut [f32],
    n_err_bnds: i32,
    err_bnds_norm: &mut [f32],
    err_bnds_comp: &mut [f32],
    nparams: &[i32],
    params: &mut [f32],
    work: &mut [c32],
    rwork: &mut [f32],
    info: &mut i32,
) {
    ffi::cposvxx_(
        &(fact as c_char),
        &(uplo as c_char),
        &n,
        &nrhs,
        a.as_mut_ptr() as *mut _,
        &lda,
        af.as_mut_ptr() as *mut _,
        &ldaf,
        equed as *mut _ as *mut _,
        s.as_mut_ptr(),
        b.as_mut_ptr() as *mut _,
        &ldb,
        x.as_mut_ptr() as *mut _,
        &ldx,
        rcond,
        rpvgrw,
        berr.as_mut_ptr(),
        &n_err_bnds,
        err_bnds_norm.as_mut_ptr(),
        err_bnds_comp.as_mut_ptr(),
        nparams.as_ptr(),
        params.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn dposvxx(
    fact: u8,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &mut [f64],
    lda: i32,
    af: &mut [f64],
    ldaf: i32,
    equed: &mut u8,
    s: &mut [f64],
    b: &mut [f64],
    ldb: i32,
    x: &mut [f64],
    ldx: i32,
    rcond: &mut f64,
    rpvgrw: &mut f64,
    berr: &mut [f64],
    n_err_bnds: i32,
    err_bnds_norm: &mut [f64],
    err_bnds_comp: &mut [f64],
    nparams: &[i32],
    params: &mut [f64],
    work: &mut [f64],
    iwork: &mut [i32],
    info: &mut i32,
) {
    ffi::dposvxx_(
        &(fact as c_char),
        &(uplo as c_char),
        &n,
        &nrhs,
        a.as_mut_ptr(),
        &lda,
        af.as_mut_ptr(),
        &ldaf,
        equed as *mut _ as *mut _,
        s.as_mut_ptr(),
        b.as_mut_ptr(),
        &ldb,
        x.as_mut_ptr(),
        &ldx,
        rcond,
        rpvgrw,
        berr.as_mut_ptr(),
        &n_err_bnds,
        err_bnds_norm.as_mut_ptr(),
        err_bnds_comp.as_mut_ptr(),
        nparams.as_ptr(),
        params.as_mut_ptr(),
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn sposvxx(
    fact: u8,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &mut [f32],
    lda: i32,
    af: &mut [f32],
    ldaf: i32,
    equed: &mut u8,
    s: &mut [f32],
    b: &mut [f32],
    ldb: i32,
    x: &mut [f32],
    ldx: i32,
    rcond: &mut f32,
    rpvgrw: &mut f32,
    berr: &mut [f32],
    n_err_bnds: i32,
    err_bnds_norm: &mut [f32],
    err_bnds_comp: &mut [f32],
    nparams: &[i32],
    params: &mut [f32],
    work: &mut [f32],
    iwork: &mut [i32],
    info: &mut i32,
) {
    ffi::sposvxx_(
        &(fact as c_char),
        &(uplo as c_char),
        &n,
        &nrhs,
        a.as_mut_ptr(),
        &lda,
        af.as_mut_ptr(),
        &ldaf,
        equed as *mut _ as *mut _,
        s.as_mut_ptr(),
        b.as_mut_ptr(),
        &ldb,
        x.as_mut_ptr(),
        &ldx,
        rcond,
        rpvgrw,
        berr.as_mut_ptr(),
        &n_err_bnds,
        err_bnds_norm.as_mut_ptr(),
        err_bnds_comp.as_mut_ptr(),
        nparams.as_ptr(),
        params.as_mut_ptr(),
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn zposvxx(
    fact: u8,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &mut [c64],
    lda: i32,
    af: &mut [c64],
    ldaf: i32,
    equed: &mut u8,
    s: &mut [f64],
    b: &mut [c64],
    ldb: i32,
    x: &mut [c64],
    ldx: i32,
    rcond: &mut f64,
    rpvgrw: &mut f64,
    berr: &mut [f64],
    n_err_bnds: i32,
    err_bnds_norm: &mut [f64],
    err_bnds_comp: &mut [f64],
    nparams: &[i32],
    params: &mut [f64],
    work: &mut [c64],
    rwork: &mut [f64],
    info: &mut i32,
) {
    ffi::zposvxx_(
        &(fact as c_char),
        &(uplo as c_char),
        &n,
        &nrhs,
        a.as_mut_ptr() as *mut _,
        &lda,
        af.as_mut_ptr() as *mut _,
        &ldaf,
        equed as *mut _ as *mut _,
        s.as_mut_ptr(),
        b.as_mut_ptr() as *mut _,
        &ldb,
        x.as_mut_ptr() as *mut _,
        &ldx,
        rcond,
        rpvgrw,
        berr.as_mut_ptr(),
        &n_err_bnds,
        err_bnds_norm.as_mut_ptr(),
        err_bnds_comp.as_mut_ptr(),
        nparams.as_ptr(),
        params.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn cpotf2(uplo: u8, n: i32, a: &mut [c32], lda: i32, info: &mut i32) {
    ffi::cpotf2_(&(uplo as c_char), &n, a.as_mut_ptr() as *mut _, &lda, info)
}

#[inline]
pub unsafe fn dpotf2(uplo: u8, n: i32, a: &mut [f64], lda: i32, info: &mut i32) {
    ffi::dpotf2_(&(uplo as c_char), &n, a.as_mut_ptr(), &lda, info)
}

#[inline]
pub unsafe fn spotf2(uplo: u8, n: i32, a: &mut [f32], lda: i32, info: &mut i32) {
    ffi::spotf2_(&(uplo as c_char), &n, a.as_mut_ptr(), &lda, info)
}

#[inline]
pub unsafe fn zpotf2(uplo: u8, n: i32, a: &mut [c64], lda: i32, info: &mut i32) {
    ffi::zpotf2_(&(uplo as c_char), &n, a.as_mut_ptr() as *mut _, &lda, info)
}

#[inline]
pub unsafe fn cpotrf(uplo: u8, n: i32, a: &mut [c32], lda: i32, info: &mut i32) {
    ffi::cpotrf_(&(uplo as c_char), &n, a.as_mut_ptr() as *mut _, &lda, info)
}

#[inline]
pub unsafe fn dpotrf(uplo: u8, n: i32, a: &mut [f64], lda: i32, info: &mut i32) {
    ffi::dpotrf_(&(uplo as c_char), &n, a.as_mut_ptr(), &lda, info)
}

#[inline]
pub unsafe fn spotrf(uplo: u8, n: i32, a: &mut [f32], lda: i32, info: &mut i32) {
    ffi::spotrf_(&(uplo as c_char), &n, a.as_mut_ptr(), &lda, info)
}

#[inline]
pub unsafe fn zpotrf(uplo: u8, n: i32, a: &mut [c64], lda: i32, info: &mut i32) {
    ffi::zpotrf_(&(uplo as c_char), &n, a.as_mut_ptr() as *mut _, &lda, info)
}

#[inline]
pub unsafe fn cpotrf2(uplo: u8, n: i32, a: &mut [c32], lda: i32, info: &mut i32) {
    ffi::cpotrf2_(&(uplo as c_char), &n, a.as_mut_ptr() as *mut _, &lda, info)
}

#[inline]
pub unsafe fn dpotrf2(uplo: u8, n: i32, a: &mut [f64], lda: i32, info: &mut i32) {
    ffi::dpotrf2_(&(uplo as c_char), &n, a.as_mut_ptr(), &lda, info)
}

#[inline]
pub unsafe fn spotrf2(uplo: u8, n: i32, a: &mut [f32], lda: i32, info: &mut i32) {
    ffi::spotrf2_(&(uplo as c_char), &n, a.as_mut_ptr(), &lda, info)
}

#[inline]
pub unsafe fn zpotrf2(uplo: u8, n: i32, a: &mut [c64], lda: i32, info: &mut i32) {
    ffi::zpotrf2_(&(uplo as c_char), &n, a.as_mut_ptr() as *mut _, &lda, info)
}

#[inline]
pub unsafe fn cpotri(uplo: u8, n: i32, a: &mut [c32], lda: i32, info: &mut i32) {
    ffi::cpotri_(&(uplo as c_char), &n, a.as_mut_ptr() as *mut _, &lda, info)
}

#[inline]
pub unsafe fn dpotri(uplo: u8, n: i32, a: &mut [f64], lda: i32, info: &mut i32) {
    ffi::dpotri_(&(uplo as c_char), &n, a.as_mut_ptr(), &lda, info)
}

#[inline]
pub unsafe fn spotri(uplo: u8, n: i32, a: &mut [f32], lda: i32, info: &mut i32) {
    ffi::spotri_(&(uplo as c_char), &n, a.as_mut_ptr(), &lda, info)
}

#[inline]
pub unsafe fn zpotri(uplo: u8, n: i32, a: &mut [c64], lda: i32, info: &mut i32) {
    ffi::zpotri_(&(uplo as c_char), &n, a.as_mut_ptr() as *mut _, &lda, info)
}

#[inline]
pub unsafe fn cpotrs(
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &[c32],
    lda: i32,
    b: &mut [c32],
    ldb: i32,
    info: &mut i32,
) {
    ffi::cpotrs_(
        &(uplo as c_char),
        &n,
        &nrhs,
        a.as_ptr() as *const _,
        &lda,
        b.as_mut_ptr() as *mut _,
        &ldb,
        info,
    )
}

#[inline]
pub unsafe fn dpotrs(
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &[f64],
    lda: i32,
    b: &mut [f64],
    ldb: i32,
    info: &mut i32,
) {
    ffi::dpotrs_(
        &(uplo as c_char),
        &n,
        &nrhs,
        a.as_ptr(),
        &lda,
        b.as_mut_ptr(),
        &ldb,
        info,
    )
}

#[inline]
pub unsafe fn spotrs(
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &[f32],
    lda: i32,
    b: &mut [f32],
    ldb: i32,
    info: &mut i32,
) {
    ffi::spotrs_(
        &(uplo as c_char),
        &n,
        &nrhs,
        a.as_ptr(),
        &lda,
        b.as_mut_ptr(),
        &ldb,
        info,
    )
}

#[inline]
pub unsafe fn zpotrs(
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &[c64],
    lda: i32,
    b: &mut [c64],
    ldb: i32,
    info: &mut i32,
) {
    ffi::zpotrs_(
        &(uplo as c_char),
        &n,
        &nrhs,
        a.as_ptr() as *const _,
        &lda,
        b.as_mut_ptr() as *mut _,
        &ldb,
        info,
    )
}

#[inline]
pub unsafe fn cppcon(
    uplo: u8,
    n: i32,
    ap: &[c32],
    anorm: f32,
    rcond: &mut f32,
    work: &mut [c32],
    rwork: &mut [f32],
    info: &mut i32,
) {
    ffi::cppcon_(
        &(uplo as c_char),
        &n,
        ap.as_ptr() as *const _,
        &anorm,
        rcond,
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn dppcon(
    uplo: u8,
    n: i32,
    ap: &[f64],
    anorm: f64,
    rcond: &mut f64,
    work: &mut [f64],
    iwork: &mut [i32],
    info: &mut i32,
) {
    ffi::dppcon_(
        &(uplo as c_char),
        &n,
        ap.as_ptr(),
        &anorm,
        rcond,
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn sppcon(
    uplo: u8,
    n: i32,
    ap: &[f32],
    anorm: f32,
    rcond: &mut f32,
    work: &mut [f32],
    iwork: &mut [i32],
    info: &mut i32,
) {
    ffi::sppcon_(
        &(uplo as c_char),
        &n,
        ap.as_ptr(),
        &anorm,
        rcond,
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn zppcon(
    uplo: u8,
    n: i32,
    ap: &[c64],
    anorm: f64,
    rcond: &mut f64,
    work: &mut [c64],
    rwork: &mut [f64],
    info: &mut i32,
) {
    ffi::zppcon_(
        &(uplo as c_char),
        &n,
        ap.as_ptr() as *const _,
        &anorm,
        rcond,
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn cppequ(
    uplo: u8,
    n: i32,
    ap: &[c32],
    s: &mut [f32],
    scond: &mut [f32],
    amax: &mut f32,
    info: &mut i32,
) {
    ffi::cppequ_(
        &(uplo as c_char),
        &n,
        ap.as_ptr() as *const _,
        s.as_mut_ptr(),
        scond.as_mut_ptr(),
        amax,
        info,
    )
}

#[inline]
pub unsafe fn dppequ(
    uplo: u8,
    n: i32,
    ap: &[f64],
    s: &mut [f64],
    scond: &mut [f64],
    amax: &mut f64,
    info: &mut i32,
) {
    ffi::dppequ_(
        &(uplo as c_char),
        &n,
        ap.as_ptr(),
        s.as_mut_ptr(),
        scond.as_mut_ptr(),
        amax,
        info,
    )
}

#[inline]
pub unsafe fn sppequ(
    uplo: u8,
    n: i32,
    ap: &[f32],
    s: &mut [f32],
    scond: &mut [f32],
    amax: &mut f32,
    info: &mut i32,
) {
    ffi::sppequ_(
        &(uplo as c_char),
        &n,
        ap.as_ptr(),
        s.as_mut_ptr(),
        scond.as_mut_ptr(),
        amax,
        info,
    )
}

#[inline]
pub unsafe fn zppequ(
    uplo: u8,
    n: i32,
    ap: &[c64],
    s: &mut [f64],
    scond: &mut [f64],
    amax: &mut f64,
    info: &mut i32,
) {
    ffi::zppequ_(
        &(uplo as c_char),
        &n,
        ap.as_ptr() as *const _,
        s.as_mut_ptr(),
        scond.as_mut_ptr(),
        amax,
        info,
    )
}

#[inline]
pub unsafe fn cpprfs(
    uplo: u8,
    n: i32,
    nrhs: i32,
    ap: &[c32],
    afp: &[c32],
    b: &[c32],
    ldb: i32,
    x: &mut [c32],
    ldx: i32,
    ferr: &mut [f32],
    berr: &mut [f32],
    work: &mut [c32],
    rwork: &mut [f32],
    info: &mut i32,
) {
    ffi::cpprfs_(
        &(uplo as c_char),
        &n,
        &nrhs,
        ap.as_ptr() as *const _,
        afp.as_ptr() as *const _,
        b.as_ptr() as *const _,
        &ldb,
        x.as_mut_ptr() as *mut _,
        &ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn dpprfs(
    uplo: u8,
    n: i32,
    nrhs: i32,
    ap: &[f64],
    afp: &[f64],
    b: &[f64],
    ldb: i32,
    x: &mut [f64],
    ldx: i32,
    ferr: &mut [f64],
    berr: &mut [f64],
    work: &mut [f64],
    iwork: &mut [i32],
    info: &mut i32,
) {
    ffi::dpprfs_(
        &(uplo as c_char),
        &n,
        &nrhs,
        ap.as_ptr(),
        afp.as_ptr(),
        b.as_ptr(),
        &ldb,
        x.as_mut_ptr(),
        &ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn spprfs(
    uplo: u8,
    n: i32,
    nrhs: i32,
    ap: &[f32],
    afp: &[f32],
    b: &[f32],
    ldb: i32,
    x: &mut [f32],
    ldx: i32,
    ferr: &mut [f32],
    berr: &mut [f32],
    work: &mut [f32],
    iwork: &mut [i32],
    info: &mut i32,
) {
    ffi::spprfs_(
        &(uplo as c_char),
        &n,
        &nrhs,
        ap.as_ptr(),
        afp.as_ptr(),
        b.as_ptr(),
        &ldb,
        x.as_mut_ptr(),
        &ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn zpprfs(
    uplo: u8,
    n: i32,
    nrhs: i32,
    ap: &[c64],
    afp: &[c64],
    b: &[c64],
    ldb: i32,
    x: &mut [c64],
    ldx: i32,
    ferr: &mut [f64],
    berr: &mut [f64],
    work: &mut [c64],
    rwork: &mut [f64],
    info: &mut i32,
) {
    ffi::zpprfs_(
        &(uplo as c_char),
        &n,
        &nrhs,
        ap.as_ptr() as *const _,
        afp.as_ptr() as *const _,
        b.as_ptr() as *const _,
        &ldb,
        x.as_mut_ptr() as *mut _,
        &ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn cppsv(
    uplo: u8,
    n: i32,
    nrhs: i32,
    ap: &mut [c32],
    b: &mut [c32],
    ldb: i32,
    info: &mut i32,
) {
    ffi::cppsv_(
        &(uplo as c_char),
        &n,
        &nrhs,
        ap.as_mut_ptr() as *mut _,
        b.as_mut_ptr() as *mut _,
        &ldb,
        info,
    )
}

#[inline]
pub unsafe fn dppsv(
    uplo: u8,
    n: i32,
    nrhs: i32,
    ap: &mut [f64],
    b: &mut [f64],
    ldb: i32,
    info: &mut i32,
) {
    ffi::dppsv_(
        &(uplo as c_char),
        &n,
        &nrhs,
        ap.as_mut_ptr(),
        b.as_mut_ptr(),
        &ldb,
        info,
    )
}

#[inline]
pub unsafe fn sppsv(
    uplo: u8,
    n: i32,
    nrhs: i32,
    ap: &mut [f32],
    b: &mut [f32],
    ldb: i32,
    info: &mut i32,
) {
    ffi::sppsv_(
        &(uplo as c_char),
        &n,
        &nrhs,
        ap.as_mut_ptr(),
        b.as_mut_ptr(),
        &ldb,
        info,
    )
}

#[inline]
pub unsafe fn zppsv(
    uplo: u8,
    n: i32,
    nrhs: i32,
    ap: &mut [c64],
    b: &mut [c64],
    ldb: i32,
    info: &mut i32,
) {
    ffi::zppsv_(
        &(uplo as c_char),
        &n,
        &nrhs,
        ap.as_mut_ptr() as *mut _,
        b.as_mut_ptr() as *mut _,
        &ldb,
        info,
    )
}

#[inline]
pub unsafe fn cppsvx(
    fact: u8,
    uplo: u8,
    n: i32,
    nrhs: i32,
    ap: &mut [c32],
    afp: &mut [c32],
    equed: &mut u8,
    s: &mut [f32],
    b: &mut [c32],
    ldb: i32,
    x: &mut [c32],
    ldx: i32,
    rcond: &mut f32,
    ferr: &mut [f32],
    berr: &mut [f32],
    work: &mut [c32],
    rwork: &mut [f32],
    info: &mut i32,
) {
    ffi::cppsvx_(
        &(fact as c_char),
        &(uplo as c_char),
        &n,
        &nrhs,
        ap.as_mut_ptr() as *mut _,
        afp.as_mut_ptr() as *mut _,
        equed as *mut _ as *mut _,
        s.as_mut_ptr(),
        b.as_mut_ptr() as *mut _,
        &ldb,
        x.as_mut_ptr() as *mut _,
        &ldx,
        rcond,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn dppsvx(
    fact: u8,
    uplo: u8,
    n: i32,
    nrhs: i32,
    ap: &mut [f64],
    afp: &mut [f64],
    equed: &mut u8,
    s: &mut [f64],
    b: &mut [f64],
    ldb: i32,
    x: &mut [f64],
    ldx: i32,
    rcond: &mut f64,
    ferr: &mut [f64],
    berr: &mut [f64],
    work: &mut [f64],
    iwork: &mut [i32],
    info: &mut i32,
) {
    ffi::dppsvx_(
        &(fact as c_char),
        &(uplo as c_char),
        &n,
        &nrhs,
        ap.as_mut_ptr(),
        afp.as_mut_ptr(),
        equed as *mut _ as *mut _,
        s.as_mut_ptr(),
        b.as_mut_ptr(),
        &ldb,
        x.as_mut_ptr(),
        &ldx,
        rcond,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn sppsvx(
    fact: u8,
    uplo: u8,
    n: i32,
    nrhs: i32,
    ap: &mut [f32],
    afp: &mut [f32],
    equed: &mut u8,
    s: &mut [f32],
    b: &mut [f32],
    ldb: i32,
    x: &mut [f32],
    ldx: i32,
    rcond: &mut f32,
    ferr: &mut [f32],
    berr: &mut [f32],
    work: &mut [f32],
    iwork: &mut [i32],
    info: &mut i32,
) {
    ffi::sppsvx_(
        &(fact as c_char),
        &(uplo as c_char),
        &n,
        &nrhs,
        ap.as_mut_ptr(),
        afp.as_mut_ptr(),
        equed as *mut _ as *mut _,
        s.as_mut_ptr(),
        b.as_mut_ptr(),
        &ldb,
        x.as_mut_ptr(),
        &ldx,
        rcond,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn zppsvx(
    fact: u8,
    uplo: u8,
    n: i32,
    nrhs: i32,
    ap: &mut [c64],
    afp: &mut [c64],
    equed: &mut u8,
    s: &mut [f64],
    b: &mut [c64],
    ldb: i32,
    x: &mut [c64],
    ldx: i32,
    rcond: &mut f64,
    ferr: &mut [f64],
    berr: &mut [f64],
    work: &mut [c64],
    rwork: &mut [f64],
    info: &mut i32,
) {
    ffi::zppsvx_(
        &(fact as c_char),
        &(uplo as c_char),
        &n,
        &nrhs,
        ap.as_mut_ptr() as *mut _,
        afp.as_mut_ptr() as *mut _,
        equed as *mut _ as *mut _,
        s.as_mut_ptr(),
        b.as_mut_ptr() as *mut _,
        &ldb,
        x.as_mut_ptr() as *mut _,
        &ldx,
        rcond,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn cpptrf(uplo: u8, n: i32, ap: &mut [c32], info: &mut i32) {
    ffi::cpptrf_(&(uplo as c_char), &n, ap.as_mut_ptr() as *mut _, info)
}

#[inline]
pub unsafe fn dpptrf(uplo: u8, n: i32, ap: &mut [f64], info: &mut i32) {
    ffi::dpptrf_(&(uplo as c_char), &n, ap.as_mut_ptr(), info)
}

#[inline]
pub unsafe fn spptrf(uplo: u8, n: i32, ap: &mut [f32], info: &mut i32) {
    ffi::spptrf_(&(uplo as c_char), &n, ap.as_mut_ptr(), info)
}

#[inline]
pub unsafe fn zpptrf(uplo: u8, n: i32, ap: &mut [c64], info: &mut i32) {
    ffi::zpptrf_(&(uplo as c_char), &n, ap.as_mut_ptr() as *mut _, info)
}

#[inline]
pub unsafe fn cpptri(uplo: u8, n: i32, ap: &mut [c32], info: &mut i32) {
    ffi::cpptri_(&(uplo as c_char), &n, ap.as_mut_ptr() as *mut _, info)
}

#[inline]
pub unsafe fn dpptri(uplo: u8, n: i32, ap: &mut [f64], info: &mut i32) {
    ffi::dpptri_(&(uplo as c_char), &n, ap.as_mut_ptr(), info)
}

#[inline]
pub unsafe fn spptri(uplo: u8, n: i32, ap: &mut [f32], info: &mut i32) {
    ffi::spptri_(&(uplo as c_char), &n, ap.as_mut_ptr(), info)
}

#[inline]
pub unsafe fn zpptri(uplo: u8, n: i32, ap: &mut [c64], info: &mut i32) {
    ffi::zpptri_(&(uplo as c_char), &n, ap.as_mut_ptr() as *mut _, info)
}

#[inline]
pub unsafe fn cpptrs(
    uplo: u8,
    n: i32,
    nrhs: i32,
    ap: &[c32],
    b: &mut [c32],
    ldb: i32,
    info: &mut i32,
) {
    ffi::cpptrs_(
        &(uplo as c_char),
        &n,
        &nrhs,
        ap.as_ptr() as *const _,
        b.as_mut_ptr() as *mut _,
        &ldb,
        info,
    )
}

#[inline]
pub unsafe fn dpptrs(
    uplo: u8,
    n: i32,
    nrhs: i32,
    ap: &[f64],
    b: &mut [f64],
    ldb: i32,
    info: &mut i32,
) {
    ffi::dpptrs_(
        &(uplo as c_char),
        &n,
        &nrhs,
        ap.as_ptr(),
        b.as_mut_ptr(),
        &ldb,
        info,
    )
}

#[inline]
pub unsafe fn spptrs(
    uplo: u8,
    n: i32,
    nrhs: i32,
    ap: &[f32],
    b: &mut [f32],
    ldb: i32,
    info: &mut i32,
) {
    ffi::spptrs_(
        &(uplo as c_char),
        &n,
        &nrhs,
        ap.as_ptr(),
        b.as_mut_ptr(),
        &ldb,
        info,
    )
}

#[inline]
pub unsafe fn zpptrs(
    uplo: u8,
    n: i32,
    nrhs: i32,
    ap: &[c64],
    b: &mut [c64],
    ldb: i32,
    info: &mut i32,
) {
    ffi::zpptrs_(
        &(uplo as c_char),
        &n,
        &nrhs,
        ap.as_ptr() as *const _,
        b.as_mut_ptr() as *mut _,
        &ldb,
        info,
    )
}

#[inline]
pub unsafe fn cpstrf(
    uplo: u8,
    n: i32,
    a: &mut [c32],
    lda: i32,
    piv: &mut [i32],
    rank: &mut i32,
    tol: f32,
    work: &mut [f32],
    info: &mut i32,
) {
    ffi::cpstrf_(
        &(uplo as c_char),
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        piv.as_mut_ptr(),
        rank,
        &tol,
        work.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn dpstrf(
    uplo: u8,
    n: i32,
    a: &mut [f64],
    lda: i32,
    piv: &mut [i32],
    rank: &mut i32,
    tol: f64,
    work: &mut [f64],
    info: &mut i32,
) {
    ffi::dpstrf_(
        &(uplo as c_char),
        &n,
        a.as_mut_ptr(),
        &lda,
        piv.as_mut_ptr(),
        rank,
        &tol,
        work.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn spstrf(
    uplo: u8,
    n: i32,
    a: &mut [f32],
    lda: i32,
    piv: &mut [i32],
    rank: &mut i32,
    tol: f32,
    work: &mut [f32],
    info: &mut i32,
) {
    ffi::spstrf_(
        &(uplo as c_char),
        &n,
        a.as_mut_ptr(),
        &lda,
        piv.as_mut_ptr(),
        rank,
        &tol,
        work.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn zpstrf(
    uplo: u8,
    n: i32,
    a: &mut [c64],
    lda: i32,
    piv: &mut [i32],
    rank: &mut i32,
    tol: f64,
    work: &mut [f64],
    info: &mut i32,
) {
    ffi::zpstrf_(
        &(uplo as c_char),
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        piv.as_mut_ptr(),
        rank,
        &tol,
        work.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn cptcon(
    n: i32,
    d: &[f32],
    e: &[c32],
    anorm: f32,
    rcond: &mut f32,
    rwork: &mut [f32],
    info: &mut i32,
) {
    ffi::cptcon_(
        &n,
        d.as_ptr(),
        e.as_ptr() as *const _,
        &anorm,
        rcond,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn dptcon(
    n: i32,
    d: &[f64],
    e: &[f64],
    anorm: f64,
    rcond: &mut f64,
    work: &mut [f64],
    info: &mut i32,
) {
    ffi::dptcon_(
        &n,
        d.as_ptr(),
        e.as_ptr(),
        &anorm,
        rcond,
        work.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn sptcon(
    n: i32,
    d: &[f32],
    e: &[f32],
    anorm: f32,
    rcond: &mut f32,
    work: &mut [f32],
    info: &mut i32,
) {
    ffi::sptcon_(
        &n,
        d.as_ptr(),
        e.as_ptr(),
        &anorm,
        rcond,
        work.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn zptcon(
    n: i32,
    d: &[f64],
    e: &[c64],
    anorm: f64,
    rcond: &mut f64,
    rwork: &mut [f64],
    info: &mut i32,
) {
    ffi::zptcon_(
        &n,
        d.as_ptr(),
        e.as_ptr() as *const _,
        &anorm,
        rcond,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn cpteqr(
    compz: u8,
    n: i32,
    d: &mut [f32],
    e: &mut [f32],
    z: &mut [c32],
    ldz: i32,
    work: &mut [f32],
    info: &mut i32,
) {
    ffi::cpteqr_(
        &(compz as c_char),
        &n,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        z.as_mut_ptr() as *mut _,
        &ldz,
        work.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn dpteqr(
    compz: u8,
    n: i32,
    d: &mut [f64],
    e: &mut [f64],
    z: &mut [f64],
    ldz: i32,
    work: &mut [f64],
    info: &mut i32,
) {
    ffi::dpteqr_(
        &(compz as c_char),
        &n,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        z.as_mut_ptr(),
        &ldz,
        work.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn spteqr(
    compz: u8,
    n: i32,
    d: &mut [f32],
    e: &mut [f32],
    z: &mut [f32],
    ldz: i32,
    work: &mut [f32],
    info: &mut i32,
) {
    ffi::spteqr_(
        &(compz as c_char),
        &n,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        z.as_mut_ptr(),
        &ldz,
        work.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn zpteqr(
    compz: u8,
    n: i32,
    d: &mut [f64],
    e: &mut [f64],
    z: &mut [c64],
    ldz: i32,
    work: &mut [f64],
    info: &mut i32,
) {
    ffi::zpteqr_(
        &(compz as c_char),
        &n,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        z.as_mut_ptr() as *mut _,
        &ldz,
        work.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn cptrfs(
    uplo: u8,
    n: i32,
    nrhs: i32,
    d: &[f32],
    e: &[c32],
    df: &[f32],
    ef: &[c32],
    b: &[c32],
    ldb: i32,
    x: &mut [c32],
    ldx: i32,
    ferr: &mut [f32],
    berr: &mut [f32],
    work: &mut [c32],
    rwork: &mut [f32],
    info: &mut i32,
) {
    ffi::cptrfs_(
        &(uplo as c_char),
        &n,
        &nrhs,
        d.as_ptr(),
        e.as_ptr() as *const _,
        df.as_ptr(),
        ef.as_ptr() as *const _,
        b.as_ptr() as *const _,
        &ldb,
        x.as_mut_ptr() as *mut _,
        &ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn dptrfs(
    n: i32,
    nrhs: i32,
    d: &[f64],
    e: &[f64],
    df: &[f64],
    ef: &[f64],
    b: &[f64],
    ldb: i32,
    x: &mut [f64],
    ldx: i32,
    ferr: &mut [f64],
    berr: &mut [f64],
    work: &mut [f64],
    info: &mut i32,
) {
    ffi::dptrfs_(
        &n,
        &nrhs,
        d.as_ptr(),
        e.as_ptr(),
        df.as_ptr(),
        ef.as_ptr(),
        b.as_ptr(),
        &ldb,
        x.as_mut_ptr(),
        &ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn sptrfs(
    n: i32,
    nrhs: i32,
    d: &[f32],
    e: &[f32],
    df: &[f32],
    ef: &[f32],
    b: &[f32],
    ldb: i32,
    x: &mut [f32],
    ldx: i32,
    ferr: &mut [f32],
    berr: &mut [f32],
    work: &mut [f32],
    info: &mut i32,
) {
    ffi::sptrfs_(
        &n,
        &nrhs,
        d.as_ptr(),
        e.as_ptr(),
        df.as_ptr(),
        ef.as_ptr(),
        b.as_ptr(),
        &ldb,
        x.as_mut_ptr(),
        &ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn zptrfs(
    uplo: u8,
    n: i32,
    nrhs: i32,
    d: &[f64],
    e: &[c64],
    df: &[f64],
    ef: &[c64],
    b: &[c64],
    ldb: i32,
    x: &mut [c64],
    ldx: i32,
    ferr: &mut [f64],
    berr: &mut [f64],
    work: &mut [c64],
    rwork: &mut [f64],
    info: &mut i32,
) {
    ffi::zptrfs_(
        &(uplo as c_char),
        &n,
        &nrhs,
        d.as_ptr(),
        e.as_ptr() as *const _,
        df.as_ptr(),
        ef.as_ptr() as *const _,
        b.as_ptr() as *const _,
        &ldb,
        x.as_mut_ptr() as *mut _,
        &ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn cptsv(
    n: i32,
    nrhs: i32,
    d: &mut [f32],
    e: &mut [c32],
    b: &mut [c32],
    ldb: i32,
    info: &mut i32,
) {
    ffi::cptsv_(
        &n,
        &nrhs,
        d.as_mut_ptr(),
        e.as_mut_ptr() as *mut _,
        b.as_mut_ptr() as *mut _,
        &ldb,
        info,
    )
}

#[inline]
pub unsafe fn dptsv(
    n: i32,
    nrhs: i32,
    d: &mut [f64],
    e: &mut [f64],
    b: &mut [f64],
    ldb: i32,
    info: &mut i32,
) {
    ffi::dptsv_(
        &n,
        &nrhs,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        b.as_mut_ptr(),
        &ldb,
        info,
    )
}

#[inline]
pub unsafe fn sptsv(
    n: i32,
    nrhs: i32,
    d: &mut [f32],
    e: &mut [f32],
    b: &mut [f32],
    ldb: i32,
    info: &mut i32,
) {
    ffi::sptsv_(
        &n,
        &nrhs,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        b.as_mut_ptr(),
        &ldb,
        info,
    )
}

#[inline]
pub unsafe fn zptsv(
    n: i32,
    nrhs: i32,
    d: &mut [f64],
    e: &mut [c64],
    b: &mut [c64],
    ldb: i32,
    info: &mut i32,
) {
    ffi::zptsv_(
        &n,
        &nrhs,
        d.as_mut_ptr(),
        e.as_mut_ptr() as *mut _,
        b.as_mut_ptr() as *mut _,
        &ldb,
        info,
    )
}

#[inline]
pub unsafe fn cptsvx(
    fact: u8,
    n: i32,
    nrhs: i32,
    d: &[f32],
    e: &[c32],
    df: &mut [f32],
    ef: &mut [c32],
    b: &[c32],
    ldb: i32,
    x: &mut [c32],
    ldx: i32,
    rcond: &mut f32,
    ferr: &mut [f32],
    berr: &mut [f32],
    work: &mut [c32],
    rwork: &mut [f32],
    info: &mut i32,
) {
    ffi::cptsvx_(
        &(fact as c_char),
        &n,
        &nrhs,
        d.as_ptr(),
        e.as_ptr() as *const _,
        df.as_mut_ptr(),
        ef.as_mut_ptr() as *mut _,
        b.as_ptr() as *const _,
        &ldb,
        x.as_mut_ptr() as *mut _,
        &ldx,
        rcond,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn dptsvx(
    fact: u8,
    n: i32,
    nrhs: i32,
    d: &[f64],
    e: &[f64],
    df: &mut [f64],
    ef: &mut [f64],
    b: &[f64],
    ldb: i32,
    x: &mut [f64],
    ldx: i32,
    rcond: &mut f64,
    ferr: &mut [f64],
    berr: &mut [f64],
    work: &mut [f64],
    info: &mut i32,
) {
    ffi::dptsvx_(
        &(fact as c_char),
        &n,
        &nrhs,
        d.as_ptr(),
        e.as_ptr(),
        df.as_mut_ptr(),
        ef.as_mut_ptr(),
        b.as_ptr(),
        &ldb,
        x.as_mut_ptr(),
        &ldx,
        rcond,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn sptsvx(
    fact: u8,
    n: i32,
    nrhs: i32,
    d: &[f32],
    e: &[f32],
    df: &mut [f32],
    ef: &mut [f32],
    b: &[f32],
    ldb: i32,
    x: &mut [f32],
    ldx: i32,
    rcond: &mut f32,
    ferr: &mut [f32],
    berr: &mut [f32],
    work: &mut [f32],
    info: &mut i32,
) {
    ffi::sptsvx_(
        &(fact as c_char),
        &n,
        &nrhs,
        d.as_ptr(),
        e.as_ptr(),
        df.as_mut_ptr(),
        ef.as_mut_ptr(),
        b.as_ptr(),
        &ldb,
        x.as_mut_ptr(),
        &ldx,
        rcond,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn zptsvx(
    fact: u8,
    n: i32,
    nrhs: i32,
    d: &[f64],
    e: &[c64],
    df: &mut [f64],
    ef: &mut [c64],
    b: &[c64],
    ldb: i32,
    x: &mut [c64],
    ldx: i32,
    rcond: &mut f64,
    ferr: &mut [f64],
    berr: &mut [f64],
    work: &mut [c64],
    rwork: &mut [f64],
    info: &mut i32,
) {
    ffi::zptsvx_(
        &(fact as c_char),
        &n,
        &nrhs,
        d.as_ptr(),
        e.as_ptr() as *const _,
        df.as_mut_ptr(),
        ef.as_mut_ptr() as *mut _,
        b.as_ptr() as *const _,
        &ldb,
        x.as_mut_ptr() as *mut _,
        &ldx,
        rcond,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn cpttrf(n: i32, d: &mut [f32], e: &mut [c32], info: &mut i32) {
    ffi::cpttrf_(&n, d.as_mut_ptr(), e.as_mut_ptr() as *mut _, info)
}

#[inline]
pub unsafe fn dpttrf(n: i32, d: &mut [f64], e: &mut [f64], info: &mut i32) {
    ffi::dpttrf_(&n, d.as_mut_ptr(), e.as_mut_ptr(), info)
}

#[inline]
pub unsafe fn spttrf(n: i32, d: &mut [f32], e: &mut [f32], info: &mut i32) {
    ffi::spttrf_(&n, d.as_mut_ptr(), e.as_mut_ptr(), info)
}

#[inline]
pub unsafe fn zpttrf(n: i32, d: &mut [f64], e: &mut [c64], info: &mut i32) {
    ffi::zpttrf_(&n, d.as_mut_ptr(), e.as_mut_ptr() as *mut _, info)
}

#[inline]
pub unsafe fn cpttrs(
    uplo: u8,
    n: i32,
    nrhs: i32,
    d: &[f32],
    e: &[c32],
    b: &mut [c32],
    ldb: i32,
    info: &mut i32,
) {
    ffi::cpttrs_(
        &(uplo as c_char),
        &n,
        &nrhs,
        d.as_ptr(),
        e.as_ptr() as *const _,
        b.as_mut_ptr() as *mut _,
        &ldb,
        info,
    )
}

#[inline]
pub unsafe fn dpttrs(
    n: i32,
    nrhs: i32,
    d: &[f64],
    e: &[f64],
    b: &mut [f64],
    ldb: i32,
    info: &mut i32,
) {
    ffi::dpttrs_(
        &n,
        &nrhs,
        d.as_ptr(),
        e.as_ptr(),
        b.as_mut_ptr(),
        &ldb,
        info,
    )
}

#[inline]
pub unsafe fn spttrs(
    n: i32,
    nrhs: i32,
    d: &[f32],
    e: &[f32],
    b: &mut [f32],
    ldb: i32,
    info: &mut i32,
) {
    ffi::spttrs_(
        &n,
        &nrhs,
        d.as_ptr(),
        e.as_ptr(),
        b.as_mut_ptr(),
        &ldb,
        info,
    )
}

#[inline]
pub unsafe fn zpttrs(
    uplo: u8,
    n: i32,
    nrhs: i32,
    d: &[f64],
    e: &[c64],
    b: &mut [c64],
    ldb: i32,
    info: &mut i32,
) {
    ffi::zpttrs_(
        &(uplo as c_char),
        &n,
        &nrhs,
        d.as_ptr(),
        e.as_ptr() as *const _,
        b.as_mut_ptr() as *mut _,
        &ldb,
        info,
    )
}

#[inline]
pub unsafe fn dsbev(
    jobz: u8,
    uplo: u8,
    n: i32,
    kd: i32,
    ab: &mut [f64],
    ldab: i32,
    w: &mut [f64],
    z: &mut [f64],
    ldz: i32,
    work: &mut [f64],
    info: &mut i32,
) {
    ffi::dsbev_(
        &(jobz as c_char),
        &(uplo as c_char),
        &n,
        &kd,
        ab.as_mut_ptr(),
        &ldab,
        w.as_mut_ptr(),
        z.as_mut_ptr(),
        &ldz,
        work.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn ssbev(
    jobz: u8,
    uplo: u8,
    n: i32,
    kd: i32,
    ab: &mut [f32],
    ldab: i32,
    w: &mut [f32],
    z: &mut [f32],
    ldz: i32,
    work: &mut [f32],
    info: &mut i32,
) {
    ffi::ssbev_(
        &(jobz as c_char),
        &(uplo as c_char),
        &n,
        &kd,
        ab.as_mut_ptr(),
        &ldab,
        w.as_mut_ptr(),
        z.as_mut_ptr(),
        &ldz,
        work.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn dsbev_2stage(
    jobz: u8,
    uplo: u8,
    n: i32,
    kd: i32,
    ab: &mut [f64],
    ldab: i32,
    w: &mut [f64],
    z: &mut [f64],
    ldz: i32,
    work: &mut [f64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::dsbev_2stage_(
        &(jobz as c_char),
        &(uplo as c_char),
        &n,
        &kd,
        ab.as_mut_ptr(),
        &ldab,
        w.as_mut_ptr(),
        z.as_mut_ptr(),
        &ldz,
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn ssbev_2stage(
    jobz: u8,
    uplo: u8,
    n: i32,
    kd: i32,
    ab: &mut [f32],
    ldab: i32,
    w: &mut [f32],
    z: &mut [f32],
    ldz: i32,
    work: &mut [f32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::ssbev_2stage_(
        &(jobz as c_char),
        &(uplo as c_char),
        &n,
        &kd,
        ab.as_mut_ptr(),
        &ldab,
        w.as_mut_ptr(),
        z.as_mut_ptr(),
        &ldz,
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn dsbevd(
    jobz: u8,
    uplo: u8,
    n: i32,
    kd: i32,
    ab: &mut [f64],
    ldab: i32,
    w: &mut [f64],
    z: &mut [f64],
    ldz: i32,
    work: &mut [f64],
    lwork: i32,
    iwork: &mut [i32],
    liwork: i32,
    info: &mut i32,
) {
    ffi::dsbevd_(
        &(jobz as c_char),
        &(uplo as c_char),
        &n,
        &kd,
        ab.as_mut_ptr(),
        &ldab,
        w.as_mut_ptr(),
        z.as_mut_ptr(),
        &ldz,
        work.as_mut_ptr(),
        &lwork,
        iwork.as_mut_ptr(),
        &liwork,
        info,
    )
}

#[inline]
pub unsafe fn ssbevd(
    jobz: u8,
    uplo: u8,
    n: i32,
    kd: i32,
    ab: &mut [f32],
    ldab: i32,
    w: &mut [f32],
    z: &mut [f32],
    ldz: i32,
    work: &mut [f32],
    lwork: i32,
    iwork: &mut [i32],
    liwork: i32,
    info: &mut i32,
) {
    ffi::ssbevd_(
        &(jobz as c_char),
        &(uplo as c_char),
        &n,
        &kd,
        ab.as_mut_ptr(),
        &ldab,
        w.as_mut_ptr(),
        z.as_mut_ptr(),
        &ldz,
        work.as_mut_ptr(),
        &lwork,
        iwork.as_mut_ptr(),
        &liwork,
        info,
    )
}

#[inline]
pub unsafe fn dsbevd_2stage(
    jobz: u8,
    uplo: u8,
    n: i32,
    kd: i32,
    ab: &mut [f64],
    ldab: i32,
    w: &mut [f64],
    z: &mut [f64],
    ldz: i32,
    work: &mut [f64],
    lwork: i32,
    iwork: &mut [i32],
    liwork: i32,
    info: &mut i32,
) {
    ffi::dsbevd_2stage_(
        &(jobz as c_char),
        &(uplo as c_char),
        &n,
        &kd,
        ab.as_mut_ptr(),
        &ldab,
        w.as_mut_ptr(),
        z.as_mut_ptr(),
        &ldz,
        work.as_mut_ptr(),
        &lwork,
        iwork.as_mut_ptr(),
        &liwork,
        info,
    )
}

#[inline]
pub unsafe fn ssbevd_2stage(
    jobz: u8,
    uplo: u8,
    n: i32,
    kd: i32,
    ab: &mut [f32],
    ldab: i32,
    w: &mut [f32],
    z: &mut [f32],
    ldz: i32,
    work: &mut [f32],
    lwork: i32,
    iwork: &mut [i32],
    liwork: i32,
    info: &mut i32,
) {
    ffi::ssbevd_2stage_(
        &(jobz as c_char),
        &(uplo as c_char),
        &n,
        &kd,
        ab.as_mut_ptr(),
        &ldab,
        w.as_mut_ptr(),
        z.as_mut_ptr(),
        &ldz,
        work.as_mut_ptr(),
        &lwork,
        iwork.as_mut_ptr(),
        &liwork,
        info,
    )
}

#[inline]
pub unsafe fn dsbevx(
    jobz: u8,
    range: u8,
    uplo: u8,
    n: i32,
    kd: i32,
    ab: &mut [f64],
    ldab: i32,
    q: &mut [f64],
    ldq: i32,
    vl: f64,
    vu: f64,
    il: i32,
    iu: i32,
    abstol: f64,
    m: &mut i32,
    w: &mut [f64],
    z: &mut [f64],
    ldz: i32,
    work: &mut [f64],
    iwork: &mut [i32],
    ifail: &mut [i32],
    info: &mut i32,
) {
    ffi::dsbevx_(
        &(jobz as c_char),
        &(range as c_char),
        &(uplo as c_char),
        &n,
        &kd,
        ab.as_mut_ptr(),
        &ldab,
        q.as_mut_ptr(),
        &ldq,
        &vl,
        &vu,
        &il,
        &iu,
        &abstol,
        m,
        w.as_mut_ptr(),
        z.as_mut_ptr(),
        &ldz,
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
        ifail.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn ssbevx(
    jobz: u8,
    range: u8,
    uplo: u8,
    n: i32,
    kd: i32,
    ab: &mut [f32],
    ldab: i32,
    q: &mut [f32],
    ldq: i32,
    vl: f32,
    vu: f32,
    il: i32,
    iu: i32,
    abstol: f32,
    m: &mut i32,
    w: &mut [f32],
    z: &mut [f32],
    ldz: i32,
    work: &mut [f32],
    iwork: &mut [i32],
    ifail: &mut [i32],
    info: &mut i32,
) {
    ffi::ssbevx_(
        &(jobz as c_char),
        &(range as c_char),
        &(uplo as c_char),
        &n,
        &kd,
        ab.as_mut_ptr(),
        &ldab,
        q.as_mut_ptr(),
        &ldq,
        &vl,
        &vu,
        &il,
        &iu,
        &abstol,
        m,
        w.as_mut_ptr(),
        z.as_mut_ptr(),
        &ldz,
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
        ifail.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn dsbevx_2stage(
    jobz: u8,
    range: u8,
    uplo: u8,
    n: i32,
    kd: i32,
    ab: &mut [f64],
    ldab: i32,
    q: &mut [f64],
    ldq: i32,
    vl: f64,
    vu: f64,
    il: i32,
    iu: i32,
    abstol: f64,
    m: &mut i32,
    w: &mut [f64],
    z: &mut [f64],
    ldz: i32,
    work: &mut [f64],
    lwork: i32,
    iwork: &mut [i32],
    ifail: &mut [i32],
    info: &mut i32,
) {
    ffi::dsbevx_2stage_(
        &(jobz as c_char),
        &(range as c_char),
        &(uplo as c_char),
        &n,
        &kd,
        ab.as_mut_ptr(),
        &ldab,
        q.as_mut_ptr(),
        &ldq,
        &vl,
        &vu,
        &il,
        &iu,
        &abstol,
        m,
        w.as_mut_ptr(),
        z.as_mut_ptr(),
        &ldz,
        work.as_mut_ptr(),
        &lwork,
        iwork.as_mut_ptr(),
        ifail.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn ssbevx_2stage(
    jobz: u8,
    range: u8,
    uplo: u8,
    n: i32,
    kd: i32,
    ab: &mut [f32],
    ldab: i32,
    q: &mut [f32],
    ldq: i32,
    vl: f32,
    vu: f32,
    il: i32,
    iu: i32,
    abstol: f32,
    m: &mut i32,
    w: &mut [f32],
    z: &mut [f32],
    ldz: i32,
    work: &mut [f32],
    lwork: i32,
    iwork: &mut [i32],
    ifail: &mut [i32],
    info: &mut i32,
) {
    ffi::ssbevx_2stage_(
        &(jobz as c_char),
        &(range as c_char),
        &(uplo as c_char),
        &n,
        &kd,
        ab.as_mut_ptr(),
        &ldab,
        q.as_mut_ptr(),
        &ldq,
        &vl,
        &vu,
        &il,
        &iu,
        &abstol,
        m,
        w.as_mut_ptr(),
        z.as_mut_ptr(),
        &ldz,
        work.as_mut_ptr(),
        &lwork,
        iwork.as_mut_ptr(),
        ifail.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn dsbgst(
    vect: u8,
    uplo: u8,
    n: i32,
    ka: i32,
    kb: i32,
    ab: &mut [f64],
    ldab: i32,
    bb: &[f64],
    ldbb: i32,
    x: &mut [f64],
    ldx: i32,
    work: &mut [f64],
    info: &mut i32,
) {
    ffi::dsbgst_(
        &(vect as c_char),
        &(uplo as c_char),
        &n,
        &ka,
        &kb,
        ab.as_mut_ptr(),
        &ldab,
        bb.as_ptr(),
        &ldbb,
        x.as_mut_ptr(),
        &ldx,
        work.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn ssbgst(
    vect: u8,
    uplo: u8,
    n: i32,
    ka: i32,
    kb: i32,
    ab: &mut [f32],
    ldab: i32,
    bb: &[f32],
    ldbb: i32,
    x: &mut [f32],
    ldx: i32,
    work: &mut [f32],
    info: &mut i32,
) {
    ffi::ssbgst_(
        &(vect as c_char),
        &(uplo as c_char),
        &n,
        &ka,
        &kb,
        ab.as_mut_ptr(),
        &ldab,
        bb.as_ptr(),
        &ldbb,
        x.as_mut_ptr(),
        &ldx,
        work.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn dsbgv(
    jobz: u8,
    uplo: u8,
    n: i32,
    ka: i32,
    kb: i32,
    ab: &mut [f64],
    ldab: i32,
    bb: &mut [f64],
    ldbb: i32,
    w: &mut [f64],
    z: &mut [f64],
    ldz: i32,
    work: &mut [f64],
    info: &mut i32,
) {
    ffi::dsbgv_(
        &(jobz as c_char),
        &(uplo as c_char),
        &n,
        &ka,
        &kb,
        ab.as_mut_ptr(),
        &ldab,
        bb.as_mut_ptr(),
        &ldbb,
        w.as_mut_ptr(),
        z.as_mut_ptr(),
        &ldz,
        work.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn ssbgv(
    jobz: u8,
    uplo: u8,
    n: i32,
    ka: i32,
    kb: i32,
    ab: &mut [f32],
    ldab: i32,
    bb: &mut [f32],
    ldbb: i32,
    w: &mut [f32],
    z: &mut [f32],
    ldz: i32,
    work: &mut [f32],
    info: &mut i32,
) {
    ffi::ssbgv_(
        &(jobz as c_char),
        &(uplo as c_char),
        &n,
        &ka,
        &kb,
        ab.as_mut_ptr(),
        &ldab,
        bb.as_mut_ptr(),
        &ldbb,
        w.as_mut_ptr(),
        z.as_mut_ptr(),
        &ldz,
        work.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn dsbgvd(
    jobz: u8,
    uplo: u8,
    n: i32,
    ka: i32,
    kb: i32,
    ab: &mut [f64],
    ldab: i32,
    bb: &mut [f64],
    ldbb: i32,
    w: &mut [f64],
    z: &mut [f64],
    ldz: i32,
    work: &mut [f64],
    lwork: i32,
    iwork: &mut [i32],
    liwork: i32,
    info: &mut i32,
) {
    ffi::dsbgvd_(
        &(jobz as c_char),
        &(uplo as c_char),
        &n,
        &ka,
        &kb,
        ab.as_mut_ptr(),
        &ldab,
        bb.as_mut_ptr(),
        &ldbb,
        w.as_mut_ptr(),
        z.as_mut_ptr(),
        &ldz,
        work.as_mut_ptr(),
        &lwork,
        iwork.as_mut_ptr(),
        &liwork,
        info,
    )
}

#[inline]
pub unsafe fn ssbgvd(
    jobz: u8,
    uplo: u8,
    n: i32,
    ka: i32,
    kb: i32,
    ab: &mut [f32],
    ldab: i32,
    bb: &mut [f32],
    ldbb: i32,
    w: &mut [f32],
    z: &mut [f32],
    ldz: i32,
    work: &mut [f32],
    lwork: i32,
    iwork: &mut [i32],
    liwork: i32,
    info: &mut i32,
) {
    ffi::ssbgvd_(
        &(jobz as c_char),
        &(uplo as c_char),
        &n,
        &ka,
        &kb,
        ab.as_mut_ptr(),
        &ldab,
        bb.as_mut_ptr(),
        &ldbb,
        w.as_mut_ptr(),
        z.as_mut_ptr(),
        &ldz,
        work.as_mut_ptr(),
        &lwork,
        iwork.as_mut_ptr(),
        &liwork,
        info,
    )
}

#[inline]
pub unsafe fn dsbgvx(
    jobz: u8,
    range: u8,
    uplo: u8,
    n: i32,
    ka: i32,
    kb: i32,
    ab: &mut [f64],
    ldab: i32,
    bb: &mut [f64],
    ldbb: i32,
    q: &mut [f64],
    ldq: i32,
    vl: f64,
    vu: f64,
    il: i32,
    iu: i32,
    abstol: f64,
    m: &mut i32,
    w: &mut [f64],
    z: &mut [f64],
    ldz: i32,
    work: &mut [f64],
    iwork: &mut [i32],
    ifail: &mut [i32],
    info: &mut i32,
) {
    ffi::dsbgvx_(
        &(jobz as c_char),
        &(range as c_char),
        &(uplo as c_char),
        &n,
        &ka,
        &kb,
        ab.as_mut_ptr(),
        &ldab,
        bb.as_mut_ptr(),
        &ldbb,
        q.as_mut_ptr(),
        &ldq,
        &vl,
        &vu,
        &il,
        &iu,
        &abstol,
        m,
        w.as_mut_ptr(),
        z.as_mut_ptr(),
        &ldz,
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
        ifail.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn ssbgvx(
    jobz: u8,
    range: u8,
    uplo: u8,
    n: i32,
    ka: i32,
    kb: i32,
    ab: &mut [f32],
    ldab: i32,
    bb: &mut [f32],
    ldbb: i32,
    q: &mut [f32],
    ldq: i32,
    vl: f32,
    vu: f32,
    il: i32,
    iu: i32,
    abstol: f32,
    m: &mut i32,
    w: &mut [f32],
    z: &mut [f32],
    ldz: i32,
    work: &mut [f32],
    iwork: &mut [i32],
    ifail: &mut [i32],
    info: &mut i32,
) {
    ffi::ssbgvx_(
        &(jobz as c_char),
        &(range as c_char),
        &(uplo as c_char),
        &n,
        &ka,
        &kb,
        ab.as_mut_ptr(),
        &ldab,
        bb.as_mut_ptr(),
        &ldbb,
        q.as_mut_ptr(),
        &ldq,
        &vl,
        &vu,
        &il,
        &iu,
        &abstol,
        m,
        w.as_mut_ptr(),
        z.as_mut_ptr(),
        &ldz,
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
        ifail.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn dsbtrd(
    vect: u8,
    uplo: u8,
    n: i32,
    kd: i32,
    ab: &mut [f64],
    ldab: i32,
    d: &mut [f64],
    e: &mut [f64],
    q: &mut [f64],
    ldq: i32,
    work: &mut [f64],
    info: &mut i32,
) {
    ffi::dsbtrd_(
        &(vect as c_char),
        &(uplo as c_char),
        &n,
        &kd,
        ab.as_mut_ptr(),
        &ldab,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        q.as_mut_ptr(),
        &ldq,
        work.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn ssbtrd(
    vect: u8,
    uplo: u8,
    n: i32,
    kd: i32,
    ab: &mut [f32],
    ldab: i32,
    d: &mut [f32],
    e: &mut [f32],
    q: &mut [f32],
    ldq: i32,
    work: &mut [f32],
    info: &mut i32,
) {
    ffi::ssbtrd_(
        &(vect as c_char),
        &(uplo as c_char),
        &n,
        &kd,
        ab.as_mut_ptr(),
        &ldab,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        q.as_mut_ptr(),
        &ldq,
        work.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn dsfrk(
    transr: u8,
    uplo: u8,
    trans: u8,
    n: i32,
    k: i32,
    alpha: &[f64],
    a: &[f64],
    lda: i32,
    beta: &[f64],
    c: &mut [f64],
) {
    ffi::dsfrk_(
        &(transr as c_char),
        &(uplo as c_char),
        &(trans as c_char),
        &n,
        &k,
        alpha.as_ptr(),
        a.as_ptr(),
        &lda,
        beta.as_ptr(),
        c.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn ssfrk(
    transr: u8,
    uplo: u8,
    trans: u8,
    n: i32,
    k: i32,
    alpha: &[f32],
    a: &[f32],
    lda: i32,
    beta: &[f32],
    c: &mut [f32],
) {
    ffi::ssfrk_(
        &(transr as c_char),
        &(uplo as c_char),
        &(trans as c_char),
        &n,
        &k,
        alpha.as_ptr(),
        a.as_ptr(),
        &lda,
        beta.as_ptr(),
        c.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn cspcon(
    uplo: u8,
    n: i32,
    ap: &[c32],
    ipiv: &[i32],
    anorm: f32,
    rcond: &mut f32,
    work: &mut [c32],
    info: &mut i32,
) {
    ffi::cspcon_(
        &(uplo as c_char),
        &n,
        ap.as_ptr() as *const _,
        ipiv.as_ptr(),
        &anorm,
        rcond,
        work.as_mut_ptr() as *mut _,
        info,
    )
}

#[inline]
pub unsafe fn dspcon(
    uplo: u8,
    n: i32,
    ap: &[f64],
    ipiv: &[i32],
    anorm: f64,
    rcond: &mut f64,
    work: &mut [f64],
    iwork: &mut [i32],
    info: &mut i32,
) {
    ffi::dspcon_(
        &(uplo as c_char),
        &n,
        ap.as_ptr(),
        ipiv.as_ptr(),
        &anorm,
        rcond,
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn sspcon(
    uplo: u8,
    n: i32,
    ap: &[f32],
    ipiv: &[i32],
    anorm: f32,
    rcond: &mut f32,
    work: &mut [f32],
    iwork: &mut [i32],
    info: &mut i32,
) {
    ffi::sspcon_(
        &(uplo as c_char),
        &n,
        ap.as_ptr(),
        ipiv.as_ptr(),
        &anorm,
        rcond,
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn zspcon(
    uplo: u8,
    n: i32,
    ap: &[c64],
    ipiv: &[i32],
    anorm: f64,
    rcond: &mut f64,
    work: &mut [c64],
    info: &mut i32,
) {
    ffi::zspcon_(
        &(uplo as c_char),
        &n,
        ap.as_ptr() as *const _,
        ipiv.as_ptr(),
        &anorm,
        rcond,
        work.as_mut_ptr() as *mut _,
        info,
    )
}

#[inline]
pub unsafe fn dspev(
    jobz: u8,
    uplo: u8,
    n: i32,
    ap: &mut [f64],
    w: &mut [f64],
    z: &mut [f64],
    ldz: i32,
    work: &mut [f64],
    info: &mut i32,
) {
    ffi::dspev_(
        &(jobz as c_char),
        &(uplo as c_char),
        &n,
        ap.as_mut_ptr(),
        w.as_mut_ptr(),
        z.as_mut_ptr(),
        &ldz,
        work.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn sspev(
    jobz: u8,
    uplo: u8,
    n: i32,
    ap: &mut [f32],
    w: &mut [f32],
    z: &mut [f32],
    ldz: i32,
    work: &mut [f32],
    info: &mut i32,
) {
    ffi::sspev_(
        &(jobz as c_char),
        &(uplo as c_char),
        &n,
        ap.as_mut_ptr(),
        w.as_mut_ptr(),
        z.as_mut_ptr(),
        &ldz,
        work.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn dspevd(
    jobz: u8,
    uplo: u8,
    n: i32,
    ap: &mut [f64],
    w: &mut [f64],
    z: &mut [f64],
    ldz: i32,
    work: &mut [f64],
    lwork: i32,
    iwork: &mut [i32],
    liwork: i32,
    info: &mut i32,
) {
    ffi::dspevd_(
        &(jobz as c_char),
        &(uplo as c_char),
        &n,
        ap.as_mut_ptr(),
        w.as_mut_ptr(),
        z.as_mut_ptr(),
        &ldz,
        work.as_mut_ptr(),
        &lwork,
        iwork.as_mut_ptr(),
        &liwork,
        info,
    )
}

#[inline]
pub unsafe fn sspevd(
    jobz: u8,
    uplo: u8,
    n: i32,
    ap: &mut [f32],
    w: &mut [f32],
    z: &mut [f32],
    ldz: i32,
    work: &mut [f32],
    lwork: i32,
    iwork: &mut [i32],
    liwork: i32,
    info: &mut i32,
) {
    ffi::sspevd_(
        &(jobz as c_char),
        &(uplo as c_char),
        &n,
        ap.as_mut_ptr(),
        w.as_mut_ptr(),
        z.as_mut_ptr(),
        &ldz,
        work.as_mut_ptr(),
        &lwork,
        iwork.as_mut_ptr(),
        &liwork,
        info,
    )
}

#[inline]
pub unsafe fn dspevx(
    jobz: u8,
    range: u8,
    uplo: u8,
    n: i32,
    ap: &mut [f64],
    vl: f64,
    vu: f64,
    il: i32,
    iu: i32,
    abstol: f64,
    m: &mut i32,
    w: &mut [f64],
    z: &mut [f64],
    ldz: i32,
    work: &mut [f64],
    iwork: &mut [i32],
    ifail: &mut [i32],
    info: &mut i32,
) {
    ffi::dspevx_(
        &(jobz as c_char),
        &(range as c_char),
        &(uplo as c_char),
        &n,
        ap.as_mut_ptr(),
        &vl,
        &vu,
        &il,
        &iu,
        &abstol,
        m,
        w.as_mut_ptr(),
        z.as_mut_ptr(),
        &ldz,
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
        ifail.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn sspevx(
    jobz: u8,
    range: u8,
    uplo: u8,
    n: i32,
    ap: &mut [f32],
    vl: f32,
    vu: f32,
    il: i32,
    iu: i32,
    abstol: f32,
    m: &mut i32,
    w: &mut [f32],
    z: &mut [f32],
    ldz: i32,
    work: &mut [f32],
    iwork: &mut [i32],
    ifail: &mut [i32],
    info: &mut i32,
) {
    ffi::sspevx_(
        &(jobz as c_char),
        &(range as c_char),
        &(uplo as c_char),
        &n,
        ap.as_mut_ptr(),
        &vl,
        &vu,
        &il,
        &iu,
        &abstol,
        m,
        w.as_mut_ptr(),
        z.as_mut_ptr(),
        &ldz,
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
        ifail.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn dspgst(itype: &[i32], uplo: u8, n: i32, ap: &mut [f64], bp: &[f64], info: &mut i32) {
    ffi::dspgst_(
        itype.as_ptr(),
        &(uplo as c_char),
        &n,
        ap.as_mut_ptr(),
        bp.as_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn sspgst(itype: &[i32], uplo: u8, n: i32, ap: &mut [f32], bp: &[f32], info: &mut i32) {
    ffi::sspgst_(
        itype.as_ptr(),
        &(uplo as c_char),
        &n,
        ap.as_mut_ptr(),
        bp.as_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn dspgv(
    itype: &[i32],
    jobz: u8,
    uplo: u8,
    n: i32,
    ap: &mut [f64],
    bp: &mut [f64],
    w: &mut [f64],
    z: &mut [f64],
    ldz: i32,
    work: &mut [f64],
    info: &mut i32,
) {
    ffi::dspgv_(
        itype.as_ptr(),
        &(jobz as c_char),
        &(uplo as c_char),
        &n,
        ap.as_mut_ptr(),
        bp.as_mut_ptr(),
        w.as_mut_ptr(),
        z.as_mut_ptr(),
        &ldz,
        work.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn sspgv(
    itype: &[i32],
    jobz: u8,
    uplo: u8,
    n: i32,
    ap: &mut [f32],
    bp: &mut [f32],
    w: &mut [f32],
    z: &mut [f32],
    ldz: i32,
    work: &mut [f32],
    info: &mut i32,
) {
    ffi::sspgv_(
        itype.as_ptr(),
        &(jobz as c_char),
        &(uplo as c_char),
        &n,
        ap.as_mut_ptr(),
        bp.as_mut_ptr(),
        w.as_mut_ptr(),
        z.as_mut_ptr(),
        &ldz,
        work.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn dspgvd(
    itype: &[i32],
    jobz: u8,
    uplo: u8,
    n: i32,
    ap: &mut [f64],
    bp: &mut [f64],
    w: &mut [f64],
    z: &mut [f64],
    ldz: i32,
    work: &mut [f64],
    lwork: i32,
    iwork: &mut [i32],
    liwork: i32,
    info: &mut i32,
) {
    ffi::dspgvd_(
        itype.as_ptr(),
        &(jobz as c_char),
        &(uplo as c_char),
        &n,
        ap.as_mut_ptr(),
        bp.as_mut_ptr(),
        w.as_mut_ptr(),
        z.as_mut_ptr(),
        &ldz,
        work.as_mut_ptr(),
        &lwork,
        iwork.as_mut_ptr(),
        &liwork,
        info,
    )
}

#[inline]
pub unsafe fn sspgvd(
    itype: &[i32],
    jobz: u8,
    uplo: u8,
    n: i32,
    ap: &mut [f32],
    bp: &mut [f32],
    w: &mut [f32],
    z: &mut [f32],
    ldz: i32,
    work: &mut [f32],
    lwork: i32,
    iwork: &mut [i32],
    liwork: i32,
    info: &mut i32,
) {
    ffi::sspgvd_(
        itype.as_ptr(),
        &(jobz as c_char),
        &(uplo as c_char),
        &n,
        ap.as_mut_ptr(),
        bp.as_mut_ptr(),
        w.as_mut_ptr(),
        z.as_mut_ptr(),
        &ldz,
        work.as_mut_ptr(),
        &lwork,
        iwork.as_mut_ptr(),
        &liwork,
        info,
    )
}

#[inline]
pub unsafe fn dspgvx(
    itype: &[i32],
    jobz: u8,
    range: u8,
    uplo: u8,
    n: i32,
    ap: &mut [f64],
    bp: &mut [f64],
    vl: f64,
    vu: f64,
    il: i32,
    iu: i32,
    abstol: f64,
    m: &mut i32,
    w: &mut [f64],
    z: &mut [f64],
    ldz: i32,
    work: &mut [f64],
    iwork: &mut [i32],
    ifail: &mut [i32],
    info: &mut i32,
) {
    ffi::dspgvx_(
        itype.as_ptr(),
        &(jobz as c_char),
        &(range as c_char),
        &(uplo as c_char),
        &n,
        ap.as_mut_ptr(),
        bp.as_mut_ptr(),
        &vl,
        &vu,
        &il,
        &iu,
        &abstol,
        m,
        w.as_mut_ptr(),
        z.as_mut_ptr(),
        &ldz,
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
        ifail.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn sspgvx(
    itype: &[i32],
    jobz: u8,
    range: u8,
    uplo: u8,
    n: i32,
    ap: &mut [f32],
    bp: &mut [f32],
    vl: f32,
    vu: f32,
    il: i32,
    iu: i32,
    abstol: f32,
    m: &mut i32,
    w: &mut [f32],
    z: &mut [f32],
    ldz: i32,
    work: &mut [f32],
    iwork: &mut [i32],
    ifail: &mut [i32],
    info: &mut i32,
) {
    ffi::sspgvx_(
        itype.as_ptr(),
        &(jobz as c_char),
        &(range as c_char),
        &(uplo as c_char),
        &n,
        ap.as_mut_ptr(),
        bp.as_mut_ptr(),
        &vl,
        &vu,
        &il,
        &iu,
        &abstol,
        m,
        w.as_mut_ptr(),
        z.as_mut_ptr(),
        &ldz,
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
        ifail.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn csprfs(
    uplo: u8,
    n: i32,
    nrhs: i32,
    ap: &[c32],
    afp: &[c32],
    ipiv: &[i32],
    b: &[c32],
    ldb: i32,
    x: &mut [c32],
    ldx: i32,
    ferr: &mut [f32],
    berr: &mut [f32],
    work: &mut [c32],
    rwork: &mut [f32],
    info: &mut i32,
) {
    ffi::csprfs_(
        &(uplo as c_char),
        &n,
        &nrhs,
        ap.as_ptr() as *const _,
        afp.as_ptr() as *const _,
        ipiv.as_ptr(),
        b.as_ptr() as *const _,
        &ldb,
        x.as_mut_ptr() as *mut _,
        &ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn dsprfs(
    uplo: u8,
    n: i32,
    nrhs: i32,
    ap: &[f64],
    afp: &[f64],
    ipiv: &[i32],
    b: &[f64],
    ldb: i32,
    x: &mut [f64],
    ldx: i32,
    ferr: &mut [f64],
    berr: &mut [f64],
    work: &mut [f64],
    iwork: &mut [i32],
    info: &mut i32,
) {
    ffi::dsprfs_(
        &(uplo as c_char),
        &n,
        &nrhs,
        ap.as_ptr(),
        afp.as_ptr(),
        ipiv.as_ptr(),
        b.as_ptr(),
        &ldb,
        x.as_mut_ptr(),
        &ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn ssprfs(
    uplo: u8,
    n: i32,
    nrhs: i32,
    ap: &[f32],
    afp: &[f32],
    ipiv: &[i32],
    b: &[f32],
    ldb: i32,
    x: &mut [f32],
    ldx: i32,
    ferr: &mut [f32],
    berr: &mut [f32],
    work: &mut [f32],
    iwork: &mut [i32],
    info: &mut i32,
) {
    ffi::ssprfs_(
        &(uplo as c_char),
        &n,
        &nrhs,
        ap.as_ptr(),
        afp.as_ptr(),
        ipiv.as_ptr(),
        b.as_ptr(),
        &ldb,
        x.as_mut_ptr(),
        &ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn zsprfs(
    uplo: u8,
    n: i32,
    nrhs: i32,
    ap: &[c64],
    afp: &[c64],
    ipiv: &[i32],
    b: &[c64],
    ldb: i32,
    x: &mut [c64],
    ldx: i32,
    ferr: &mut [f64],
    berr: &mut [f64],
    work: &mut [c64],
    rwork: &mut [f64],
    info: &mut i32,
) {
    ffi::zsprfs_(
        &(uplo as c_char),
        &n,
        &nrhs,
        ap.as_ptr() as *const _,
        afp.as_ptr() as *const _,
        ipiv.as_ptr(),
        b.as_ptr() as *const _,
        &ldb,
        x.as_mut_ptr() as *mut _,
        &ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn cspsv(
    uplo: u8,
    n: i32,
    nrhs: i32,
    ap: &mut [c32],
    ipiv: &mut [i32],
    b: &mut [c32],
    ldb: i32,
    info: &mut i32,
) {
    ffi::cspsv_(
        &(uplo as c_char),
        &n,
        &nrhs,
        ap.as_mut_ptr() as *mut _,
        ipiv.as_mut_ptr(),
        b.as_mut_ptr() as *mut _,
        &ldb,
        info,
    )
}

#[inline]
pub unsafe fn dspsv(
    uplo: u8,
    n: i32,
    nrhs: i32,
    ap: &mut [f64],
    ipiv: &mut [i32],
    b: &mut [f64],
    ldb: i32,
    info: &mut i32,
) {
    ffi::dspsv_(
        &(uplo as c_char),
        &n,
        &nrhs,
        ap.as_mut_ptr(),
        ipiv.as_mut_ptr(),
        b.as_mut_ptr(),
        &ldb,
        info,
    )
}

#[inline]
pub unsafe fn sspsv(
    uplo: u8,
    n: i32,
    nrhs: i32,
    ap: &mut [f32],
    ipiv: &mut [i32],
    b: &mut [f32],
    ldb: i32,
    info: &mut i32,
) {
    ffi::sspsv_(
        &(uplo as c_char),
        &n,
        &nrhs,
        ap.as_mut_ptr(),
        ipiv.as_mut_ptr(),
        b.as_mut_ptr(),
        &ldb,
        info,
    )
}

#[inline]
pub unsafe fn zspsv(
    uplo: u8,
    n: i32,
    nrhs: i32,
    ap: &mut [c64],
    ipiv: &mut [i32],
    b: &mut [c64],
    ldb: i32,
    info: &mut i32,
) {
    ffi::zspsv_(
        &(uplo as c_char),
        &n,
        &nrhs,
        ap.as_mut_ptr() as *mut _,
        ipiv.as_mut_ptr(),
        b.as_mut_ptr() as *mut _,
        &ldb,
        info,
    )
}

#[inline]
pub unsafe fn cspsvx(
    fact: u8,
    uplo: u8,
    n: i32,
    nrhs: i32,
    ap: &[c32],
    afp: &mut [c32],
    ipiv: &mut [i32],
    b: &[c32],
    ldb: i32,
    x: &mut [c32],
    ldx: i32,
    rcond: &mut f32,
    ferr: &mut [f32],
    berr: &mut [f32],
    work: &mut [c32],
    rwork: &mut [f32],
    info: &mut i32,
) {
    ffi::cspsvx_(
        &(fact as c_char),
        &(uplo as c_char),
        &n,
        &nrhs,
        ap.as_ptr() as *const _,
        afp.as_mut_ptr() as *mut _,
        ipiv.as_mut_ptr(),
        b.as_ptr() as *const _,
        &ldb,
        x.as_mut_ptr() as *mut _,
        &ldx,
        rcond,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn dspsvx(
    fact: u8,
    uplo: u8,
    n: i32,
    nrhs: i32,
    ap: &[f64],
    afp: &mut [f64],
    ipiv: &mut [i32],
    b: &[f64],
    ldb: i32,
    x: &mut [f64],
    ldx: i32,
    rcond: &mut f64,
    ferr: &mut [f64],
    berr: &mut [f64],
    work: &mut [f64],
    iwork: &mut [i32],
    info: &mut i32,
) {
    ffi::dspsvx_(
        &(fact as c_char),
        &(uplo as c_char),
        &n,
        &nrhs,
        ap.as_ptr(),
        afp.as_mut_ptr(),
        ipiv.as_mut_ptr(),
        b.as_ptr(),
        &ldb,
        x.as_mut_ptr(),
        &ldx,
        rcond,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn sspsvx(
    fact: u8,
    uplo: u8,
    n: i32,
    nrhs: i32,
    ap: &[f32],
    afp: &mut [f32],
    ipiv: &mut [i32],
    b: &[f32],
    ldb: i32,
    x: &mut [f32],
    ldx: i32,
    rcond: &mut f32,
    ferr: &mut [f32],
    berr: &mut [f32],
    work: &mut [f32],
    iwork: &mut [i32],
    info: &mut i32,
) {
    ffi::sspsvx_(
        &(fact as c_char),
        &(uplo as c_char),
        &n,
        &nrhs,
        ap.as_ptr(),
        afp.as_mut_ptr(),
        ipiv.as_mut_ptr(),
        b.as_ptr(),
        &ldb,
        x.as_mut_ptr(),
        &ldx,
        rcond,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn zspsvx(
    fact: u8,
    uplo: u8,
    n: i32,
    nrhs: i32,
    ap: &[c64],
    afp: &mut [c64],
    ipiv: &mut [i32],
    b: &[c64],
    ldb: i32,
    x: &mut [c64],
    ldx: i32,
    rcond: &mut f64,
    ferr: &mut [f64],
    berr: &mut [f64],
    work: &mut [c64],
    rwork: &mut [f64],
    info: &mut i32,
) {
    ffi::zspsvx_(
        &(fact as c_char),
        &(uplo as c_char),
        &n,
        &nrhs,
        ap.as_ptr() as *const _,
        afp.as_mut_ptr() as *mut _,
        ipiv.as_mut_ptr(),
        b.as_ptr() as *const _,
        &ldb,
        x.as_mut_ptr() as *mut _,
        &ldx,
        rcond,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn dsptrd(
    uplo: u8,
    n: i32,
    ap: &mut [f64],
    d: &mut [f64],
    e: &mut [f64],
    tau: &mut [f64],
    info: &mut i32,
) {
    ffi::dsptrd_(
        &(uplo as c_char),
        &n,
        ap.as_mut_ptr(),
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        tau.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn ssptrd(
    uplo: u8,
    n: i32,
    ap: &mut [f32],
    d: &mut [f32],
    e: &mut [f32],
    tau: &mut [f32],
    info: &mut i32,
) {
    ffi::ssptrd_(
        &(uplo as c_char),
        &n,
        ap.as_mut_ptr(),
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        tau.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn csptrf(uplo: u8, n: i32, ap: &mut [c32], ipiv: &mut [i32], info: &mut i32) {
    ffi::csptrf_(
        &(uplo as c_char),
        &n,
        ap.as_mut_ptr() as *mut _,
        ipiv.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn dsptrf(uplo: u8, n: i32, ap: &mut [f64], ipiv: &mut [i32], info: &mut i32) {
    ffi::dsptrf_(
        &(uplo as c_char),
        &n,
        ap.as_mut_ptr(),
        ipiv.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn ssptrf(uplo: u8, n: i32, ap: &mut [f32], ipiv: &mut [i32], info: &mut i32) {
    ffi::ssptrf_(
        &(uplo as c_char),
        &n,
        ap.as_mut_ptr(),
        ipiv.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn zsptrf(uplo: u8, n: i32, ap: &mut [c64], ipiv: &mut [i32], info: &mut i32) {
    ffi::zsptrf_(
        &(uplo as c_char),
        &n,
        ap.as_mut_ptr() as *mut _,
        ipiv.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn csptri(
    uplo: u8,
    n: i32,
    ap: &mut [c32],
    ipiv: &[i32],
    work: &mut [c32],
    info: &mut i32,
) {
    ffi::csptri_(
        &(uplo as c_char),
        &n,
        ap.as_mut_ptr() as *mut _,
        ipiv.as_ptr(),
        work.as_mut_ptr() as *mut _,
        info,
    )
}

#[inline]
pub unsafe fn dsptri(
    uplo: u8,
    n: i32,
    ap: &mut [f64],
    ipiv: &[i32],
    work: &mut [f64],
    info: &mut i32,
) {
    ffi::dsptri_(
        &(uplo as c_char),
        &n,
        ap.as_mut_ptr(),
        ipiv.as_ptr(),
        work.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn ssptri(
    uplo: u8,
    n: i32,
    ap: &mut [f32],
    ipiv: &[i32],
    work: &mut [f32],
    info: &mut i32,
) {
    ffi::ssptri_(
        &(uplo as c_char),
        &n,
        ap.as_mut_ptr(),
        ipiv.as_ptr(),
        work.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn zsptri(
    uplo: u8,
    n: i32,
    ap: &mut [c64],
    ipiv: &[i32],
    work: &mut [c64],
    info: &mut i32,
) {
    ffi::zsptri_(
        &(uplo as c_char),
        &n,
        ap.as_mut_ptr() as *mut _,
        ipiv.as_ptr(),
        work.as_mut_ptr() as *mut _,
        info,
    )
}

#[inline]
pub unsafe fn csptrs(
    uplo: u8,
    n: i32,
    nrhs: i32,
    ap: &[c32],
    ipiv: &[i32],
    b: &mut [c32],
    ldb: i32,
    info: &mut i32,
) {
    ffi::csptrs_(
        &(uplo as c_char),
        &n,
        &nrhs,
        ap.as_ptr() as *const _,
        ipiv.as_ptr(),
        b.as_mut_ptr() as *mut _,
        &ldb,
        info,
    )
}

#[inline]
pub unsafe fn dsptrs(
    uplo: u8,
    n: i32,
    nrhs: i32,
    ap: &[f64],
    ipiv: &[i32],
    b: &mut [f64],
    ldb: i32,
    info: &mut i32,
) {
    ffi::dsptrs_(
        &(uplo as c_char),
        &n,
        &nrhs,
        ap.as_ptr(),
        ipiv.as_ptr(),
        b.as_mut_ptr(),
        &ldb,
        info,
    )
}

#[inline]
pub unsafe fn ssptrs(
    uplo: u8,
    n: i32,
    nrhs: i32,
    ap: &[f32],
    ipiv: &[i32],
    b: &mut [f32],
    ldb: i32,
    info: &mut i32,
) {
    ffi::ssptrs_(
        &(uplo as c_char),
        &n,
        &nrhs,
        ap.as_ptr(),
        ipiv.as_ptr(),
        b.as_mut_ptr(),
        &ldb,
        info,
    )
}

#[inline]
pub unsafe fn zsptrs(
    uplo: u8,
    n: i32,
    nrhs: i32,
    ap: &[c64],
    ipiv: &[i32],
    b: &mut [c64],
    ldb: i32,
    info: &mut i32,
) {
    ffi::zsptrs_(
        &(uplo as c_char),
        &n,
        &nrhs,
        ap.as_ptr() as *const _,
        ipiv.as_ptr(),
        b.as_mut_ptr() as *mut _,
        &ldb,
        info,
    )
}

#[inline]
pub unsafe fn dstebz(
    range: u8,
    order: u8,
    n: i32,
    vl: f64,
    vu: f64,
    il: i32,
    iu: i32,
    abstol: f64,
    d: &[f64],
    e: &[f64],
    m: &mut i32,
    nsplit: &mut [i32],
    w: &mut [f64],
    iblock: &mut [i32],
    isplit: &mut [i32],
    work: &mut [f64],
    iwork: &mut [i32],
    info: &mut i32,
) {
    ffi::dstebz_(
        &(range as c_char),
        &(order as c_char),
        &n,
        &vl,
        &vu,
        &il,
        &iu,
        &abstol,
        d.as_ptr(),
        e.as_ptr(),
        m,
        nsplit.as_mut_ptr(),
        w.as_mut_ptr(),
        iblock.as_mut_ptr(),
        isplit.as_mut_ptr(),
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn sstebz(
    range: u8,
    order: u8,
    n: i32,
    vl: f32,
    vu: f32,
    il: i32,
    iu: i32,
    abstol: f32,
    d: &[f32],
    e: &[f32],
    m: &mut i32,
    nsplit: &mut [i32],
    w: &mut [f32],
    iblock: &mut [i32],
    isplit: &mut [i32],
    work: &mut [f32],
    iwork: &mut [i32],
    info: &mut i32,
) {
    ffi::sstebz_(
        &(range as c_char),
        &(order as c_char),
        &n,
        &vl,
        &vu,
        &il,
        &iu,
        &abstol,
        d.as_ptr(),
        e.as_ptr(),
        m,
        nsplit.as_mut_ptr(),
        w.as_mut_ptr(),
        iblock.as_mut_ptr(),
        isplit.as_mut_ptr(),
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn cstedc(
    compz: u8,
    n: i32,
    d: &mut [f32],
    e: &mut [f32],
    z: &mut [c32],
    ldz: i32,
    work: &mut [c32],
    lwork: i32,
    rwork: &mut [f32],
    lrwork: i32,
    iwork: &mut [i32],
    liwork: i32,
    info: &mut i32,
) {
    ffi::cstedc_(
        &(compz as c_char),
        &n,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        z.as_mut_ptr() as *mut _,
        &ldz,
        work.as_mut_ptr() as *mut _,
        &lwork,
        rwork.as_mut_ptr(),
        &lrwork,
        iwork.as_mut_ptr(),
        &liwork,
        info,
    )
}

#[inline]
pub unsafe fn dstedc(
    compz: u8,
    n: i32,
    d: &mut [f64],
    e: &mut [f64],
    z: &mut [f64],
    ldz: i32,
    work: &mut [f64],
    lwork: i32,
    iwork: &mut [i32],
    liwork: i32,
    info: &mut i32,
) {
    ffi::dstedc_(
        &(compz as c_char),
        &n,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        z.as_mut_ptr(),
        &ldz,
        work.as_mut_ptr(),
        &lwork,
        iwork.as_mut_ptr(),
        &liwork,
        info,
    )
}

#[inline]
pub unsafe fn sstedc(
    compz: u8,
    n: i32,
    d: &mut [f32],
    e: &mut [f32],
    z: &mut [f32],
    ldz: i32,
    work: &mut [f32],
    lwork: i32,
    iwork: &mut [i32],
    liwork: i32,
    info: &mut i32,
) {
    ffi::sstedc_(
        &(compz as c_char),
        &n,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        z.as_mut_ptr(),
        &ldz,
        work.as_mut_ptr(),
        &lwork,
        iwork.as_mut_ptr(),
        &liwork,
        info,
    )
}

#[inline]
pub unsafe fn zstedc(
    compz: u8,
    n: i32,
    d: &mut [f64],
    e: &mut [f64],
    z: &mut [c64],
    ldz: i32,
    work: &mut [c64],
    lwork: i32,
    rwork: &mut [f64],
    lrwork: i32,
    iwork: &mut [i32],
    liwork: i32,
    info: &mut i32,
) {
    ffi::zstedc_(
        &(compz as c_char),
        &n,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        z.as_mut_ptr() as *mut _,
        &ldz,
        work.as_mut_ptr() as *mut _,
        &lwork,
        rwork.as_mut_ptr(),
        &lrwork,
        iwork.as_mut_ptr(),
        &liwork,
        info,
    )
}

#[inline]
pub unsafe fn cstegr(
    jobz: u8,
    range: u8,
    n: i32,
    d: &mut [f32],
    e: &mut [f32],
    vl: f32,
    vu: f32,
    il: i32,
    iu: i32,
    abstol: f32,
    m: &mut i32,
    w: &mut [f32],
    z: &mut [c32],
    ldz: i32,
    isuppz: &mut [i32],
    work: &mut [f32],
    lwork: i32,
    iwork: &mut [i32],
    liwork: i32,
    info: &mut i32,
) {
    ffi::cstegr_(
        &(jobz as c_char),
        &(range as c_char),
        &n,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        &vl,
        &vu,
        &il,
        &iu,
        &abstol,
        m,
        w.as_mut_ptr(),
        z.as_mut_ptr() as *mut _,
        &ldz,
        isuppz.as_mut_ptr(),
        work.as_mut_ptr(),
        &lwork,
        iwork.as_mut_ptr(),
        &liwork,
        info,
    )
}

#[inline]
pub unsafe fn dstegr(
    jobz: u8,
    range: u8,
    n: i32,
    d: &mut [f64],
    e: &mut [f64],
    vl: f64,
    vu: f64,
    il: i32,
    iu: i32,
    abstol: f64,
    m: &mut i32,
    w: &mut [f64],
    z: &mut [f64],
    ldz: i32,
    isuppz: &mut [i32],
    work: &mut [f64],
    lwork: i32,
    iwork: &mut [i32],
    liwork: i32,
    info: &mut i32,
) {
    ffi::dstegr_(
        &(jobz as c_char),
        &(range as c_char),
        &n,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        &vl,
        &vu,
        &il,
        &iu,
        &abstol,
        m,
        w.as_mut_ptr(),
        z.as_mut_ptr(),
        &ldz,
        isuppz.as_mut_ptr(),
        work.as_mut_ptr(),
        &lwork,
        iwork.as_mut_ptr(),
        &liwork,
        info,
    )
}

#[inline]
pub unsafe fn sstegr(
    jobz: u8,
    range: u8,
    n: i32,
    d: &mut [f32],
    e: &mut [f32],
    vl: f32,
    vu: f32,
    il: i32,
    iu: i32,
    abstol: f32,
    m: &mut i32,
    w: &mut [f32],
    z: &mut [f32],
    ldz: i32,
    isuppz: &mut [i32],
    work: &mut [f32],
    lwork: i32,
    iwork: &mut [i32],
    liwork: i32,
    info: &mut i32,
) {
    ffi::sstegr_(
        &(jobz as c_char),
        &(range as c_char),
        &n,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        &vl,
        &vu,
        &il,
        &iu,
        &abstol,
        m,
        w.as_mut_ptr(),
        z.as_mut_ptr(),
        &ldz,
        isuppz.as_mut_ptr(),
        work.as_mut_ptr(),
        &lwork,
        iwork.as_mut_ptr(),
        &liwork,
        info,
    )
}

#[inline]
pub unsafe fn zstegr(
    jobz: u8,
    range: u8,
    n: i32,
    d: &mut [f64],
    e: &mut [f64],
    vl: f64,
    vu: f64,
    il: i32,
    iu: i32,
    abstol: f64,
    m: &mut i32,
    w: &mut [f64],
    z: &mut [c64],
    ldz: i32,
    isuppz: &mut [i32],
    work: &mut [f64],
    lwork: i32,
    iwork: &mut [i32],
    liwork: i32,
    info: &mut i32,
) {
    ffi::zstegr_(
        &(jobz as c_char),
        &(range as c_char),
        &n,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        &vl,
        &vu,
        &il,
        &iu,
        &abstol,
        m,
        w.as_mut_ptr(),
        z.as_mut_ptr() as *mut _,
        &ldz,
        isuppz.as_mut_ptr(),
        work.as_mut_ptr(),
        &lwork,
        iwork.as_mut_ptr(),
        &liwork,
        info,
    )
}

#[inline]
pub unsafe fn cstein(
    n: i32,
    d: &[f32],
    e: &[f32],
    m: i32,
    w: &[f32],
    iblock: &[i32],
    isplit: &[i32],
    z: &mut [c32],
    ldz: i32,
    work: &mut [f32],
    iwork: &mut [i32],
    ifail: &mut [i32],
    info: &mut i32,
) {
    ffi::cstein_(
        &n,
        d.as_ptr(),
        e.as_ptr(),
        &m,
        w.as_ptr(),
        iblock.as_ptr(),
        isplit.as_ptr(),
        z.as_mut_ptr() as *mut _,
        &ldz,
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
        ifail.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn dstein(
    n: i32,
    d: &[f64],
    e: &[f64],
    m: i32,
    w: &[f64],
    iblock: &[i32],
    isplit: &[i32],
    z: &mut [f64],
    ldz: i32,
    work: &mut [f64],
    iwork: &mut [i32],
    ifail: &mut [i32],
    info: &mut i32,
) {
    ffi::dstein_(
        &n,
        d.as_ptr(),
        e.as_ptr(),
        &m,
        w.as_ptr(),
        iblock.as_ptr(),
        isplit.as_ptr(),
        z.as_mut_ptr(),
        &ldz,
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
        ifail.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn sstein(
    n: i32,
    d: &[f32],
    e: &[f32],
    m: i32,
    w: &[f32],
    iblock: &[i32],
    isplit: &[i32],
    z: &mut [f32],
    ldz: i32,
    work: &mut [f32],
    iwork: &mut [i32],
    ifail: &mut [i32],
    info: &mut i32,
) {
    ffi::sstein_(
        &n,
        d.as_ptr(),
        e.as_ptr(),
        &m,
        w.as_ptr(),
        iblock.as_ptr(),
        isplit.as_ptr(),
        z.as_mut_ptr(),
        &ldz,
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
        ifail.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn zstein(
    n: i32,
    d: &[f64],
    e: &[f64],
    m: i32,
    w: &[f64],
    iblock: &[i32],
    isplit: &[i32],
    z: &mut [c64],
    ldz: i32,
    work: &mut [f64],
    iwork: &mut [i32],
    ifail: &mut [i32],
    info: &mut i32,
) {
    ffi::zstein_(
        &n,
        d.as_ptr(),
        e.as_ptr(),
        &m,
        w.as_ptr(),
        iblock.as_ptr(),
        isplit.as_ptr(),
        z.as_mut_ptr() as *mut _,
        &ldz,
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
        ifail.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn cstemr(
    jobz: u8,
    range: u8,
    n: i32,
    d: &mut [f32],
    e: &mut [f32],
    vl: f32,
    vu: f32,
    il: i32,
    iu: i32,
    m: &mut i32,
    w: &mut [f32],
    z: &mut [c32],
    ldz: i32,
    nzc: &[i32],
    isuppz: &mut [i32],
    tryrac: &mut i32,
    work: &mut [f32],
    lwork: i32,
    iwork: &mut [i32],
    liwork: i32,
    info: &mut i32,
) {
    ffi::cstemr_(
        &(jobz as c_char),
        &(range as c_char),
        &n,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        &vl,
        &vu,
        &il,
        &iu,
        m,
        w.as_mut_ptr(),
        z.as_mut_ptr() as *mut _,
        &ldz,
        nzc.as_ptr(),
        isuppz.as_mut_ptr(),
        tryrac,
        work.as_mut_ptr(),
        &lwork,
        iwork.as_mut_ptr(),
        &liwork,
        info,
    )
}

#[inline]
pub unsafe fn dstemr(
    jobz: u8,
    range: u8,
    n: i32,
    d: &mut [f64],
    e: &mut [f64],
    vl: f64,
    vu: f64,
    il: i32,
    iu: i32,
    m: &mut i32,
    w: &mut [f64],
    z: &mut [f64],
    ldz: i32,
    nzc: &[i32],
    isuppz: &mut [i32],
    tryrac: &mut i32,
    work: &mut [f64],
    lwork: i32,
    iwork: &mut [i32],
    liwork: i32,
    info: &mut i32,
) {
    ffi::dstemr_(
        &(jobz as c_char),
        &(range as c_char),
        &n,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        &vl,
        &vu,
        &il,
        &iu,
        m,
        w.as_mut_ptr(),
        z.as_mut_ptr(),
        &ldz,
        nzc.as_ptr(),
        isuppz.as_mut_ptr(),
        tryrac,
        work.as_mut_ptr(),
        &lwork,
        iwork.as_mut_ptr(),
        &liwork,
        info,
    )
}

#[inline]
pub unsafe fn sstemr(
    jobz: u8,
    range: u8,
    n: i32,
    d: &mut [f32],
    e: &mut [f32],
    vl: f32,
    vu: f32,
    il: i32,
    iu: i32,
    m: &mut i32,
    w: &mut [f32],
    z: &mut [f32],
    ldz: i32,
    nzc: &[i32],
    isuppz: &mut [i32],
    tryrac: &mut i32,
    work: &mut [f32],
    lwork: i32,
    iwork: &mut [i32],
    liwork: i32,
    info: &mut i32,
) {
    ffi::sstemr_(
        &(jobz as c_char),
        &(range as c_char),
        &n,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        &vl,
        &vu,
        &il,
        &iu,
        m,
        w.as_mut_ptr(),
        z.as_mut_ptr(),
        &ldz,
        nzc.as_ptr(),
        isuppz.as_mut_ptr(),
        tryrac,
        work.as_mut_ptr(),
        &lwork,
        iwork.as_mut_ptr(),
        &liwork,
        info,
    )
}

#[inline]
pub unsafe fn zstemr(
    jobz: u8,
    range: u8,
    n: i32,
    d: &mut [f64],
    e: &mut [f64],
    vl: f64,
    vu: f64,
    il: i32,
    iu: i32,
    m: &mut i32,
    w: &mut [f64],
    z: &mut [c64],
    ldz: i32,
    nzc: &[i32],
    isuppz: &mut [i32],
    tryrac: &mut i32,
    work: &mut [f64],
    lwork: i32,
    iwork: &mut [i32],
    liwork: i32,
    info: &mut i32,
) {
    ffi::zstemr_(
        &(jobz as c_char),
        &(range as c_char),
        &n,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        &vl,
        &vu,
        &il,
        &iu,
        m,
        w.as_mut_ptr(),
        z.as_mut_ptr() as *mut _,
        &ldz,
        nzc.as_ptr(),
        isuppz.as_mut_ptr(),
        tryrac,
        work.as_mut_ptr(),
        &lwork,
        iwork.as_mut_ptr(),
        &liwork,
        info,
    )
}

#[inline]
pub unsafe fn csteqr(
    compz: u8,
    n: i32,
    d: &mut [f32],
    e: &mut [f32],
    z: &mut [c32],
    ldz: i32,
    work: &mut [f32],
    info: &mut i32,
) {
    ffi::csteqr_(
        &(compz as c_char),
        &n,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        z.as_mut_ptr() as *mut _,
        &ldz,
        work.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn dsteqr(
    compz: u8,
    n: i32,
    d: &mut [f64],
    e: &mut [f64],
    z: &mut [f64],
    ldz: i32,
    work: &mut [f64],
    info: &mut i32,
) {
    ffi::dsteqr_(
        &(compz as c_char),
        &n,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        z.as_mut_ptr(),
        &ldz,
        work.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn ssteqr(
    compz: u8,
    n: i32,
    d: &mut [f32],
    e: &mut [f32],
    z: &mut [f32],
    ldz: i32,
    work: &mut [f32],
    info: &mut i32,
) {
    ffi::ssteqr_(
        &(compz as c_char),
        &n,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        z.as_mut_ptr(),
        &ldz,
        work.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn zsteqr(
    compz: u8,
    n: i32,
    d: &mut [f64],
    e: &mut [f64],
    z: &mut [c64],
    ldz: i32,
    work: &mut [f64],
    info: &mut i32,
) {
    ffi::zsteqr_(
        &(compz as c_char),
        &n,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        z.as_mut_ptr() as *mut _,
        &ldz,
        work.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn dsterf(n: i32, d: &mut [f64], e: &mut [f64], info: &mut i32) {
    ffi::dsterf_(&n, d.as_mut_ptr(), e.as_mut_ptr(), info)
}

#[inline]
pub unsafe fn ssterf(n: i32, d: &mut [f32], e: &mut [f32], info: &mut i32) {
    ffi::ssterf_(&n, d.as_mut_ptr(), e.as_mut_ptr(), info)
}

#[inline]
pub unsafe fn dstev(
    jobz: u8,
    n: i32,
    d: &mut [f64],
    e: &mut [f64],
    z: &mut [f64],
    ldz: i32,
    work: &mut [f64],
    info: &mut i32,
) {
    ffi::dstev_(
        &(jobz as c_char),
        &n,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        z.as_mut_ptr(),
        &ldz,
        work.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn sstev(
    jobz: u8,
    n: i32,
    d: &mut [f32],
    e: &mut [f32],
    z: &mut [f32],
    ldz: i32,
    work: &mut [f32],
    info: &mut i32,
) {
    ffi::sstev_(
        &(jobz as c_char),
        &n,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        z.as_mut_ptr(),
        &ldz,
        work.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn dstevd(
    jobz: u8,
    n: i32,
    d: &mut [f64],
    e: &mut [f64],
    z: &mut [f64],
    ldz: i32,
    work: &mut [f64],
    lwork: i32,
    iwork: &mut [i32],
    liwork: i32,
    info: &mut i32,
) {
    ffi::dstevd_(
        &(jobz as c_char),
        &n,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        z.as_mut_ptr(),
        &ldz,
        work.as_mut_ptr(),
        &lwork,
        iwork.as_mut_ptr(),
        &liwork,
        info,
    )
}

#[inline]
pub unsafe fn sstevd(
    jobz: u8,
    n: i32,
    d: &mut [f32],
    e: &mut [f32],
    z: &mut [f32],
    ldz: i32,
    work: &mut [f32],
    lwork: i32,
    iwork: &mut [i32],
    liwork: i32,
    info: &mut i32,
) {
    ffi::sstevd_(
        &(jobz as c_char),
        &n,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        z.as_mut_ptr(),
        &ldz,
        work.as_mut_ptr(),
        &lwork,
        iwork.as_mut_ptr(),
        &liwork,
        info,
    )
}

#[inline]
pub unsafe fn dstevr(
    jobz: u8,
    range: u8,
    n: i32,
    d: &mut [f64],
    e: &mut [f64],
    vl: f64,
    vu: f64,
    il: i32,
    iu: i32,
    abstol: f64,
    m: &mut i32,
    w: &mut [f64],
    z: &mut [f64],
    ldz: i32,
    isuppz: &mut [i32],
    work: &mut [f64],
    lwork: i32,
    iwork: &mut [i32],
    liwork: i32,
    info: &mut i32,
) {
    ffi::dstevr_(
        &(jobz as c_char),
        &(range as c_char),
        &n,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        &vl,
        &vu,
        &il,
        &iu,
        &abstol,
        m,
        w.as_mut_ptr(),
        z.as_mut_ptr(),
        &ldz,
        isuppz.as_mut_ptr(),
        work.as_mut_ptr(),
        &lwork,
        iwork.as_mut_ptr(),
        &liwork,
        info,
    )
}

#[inline]
pub unsafe fn sstevr(
    jobz: u8,
    range: u8,
    n: i32,
    d: &mut [f32],
    e: &mut [f32],
    vl: f32,
    vu: f32,
    il: i32,
    iu: i32,
    abstol: f32,
    m: &mut i32,
    w: &mut [f32],
    z: &mut [f32],
    ldz: i32,
    isuppz: &mut [i32],
    work: &mut [f32],
    lwork: i32,
    iwork: &mut [i32],
    liwork: i32,
    info: &mut i32,
) {
    ffi::sstevr_(
        &(jobz as c_char),
        &(range as c_char),
        &n,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        &vl,
        &vu,
        &il,
        &iu,
        &abstol,
        m,
        w.as_mut_ptr(),
        z.as_mut_ptr(),
        &ldz,
        isuppz.as_mut_ptr(),
        work.as_mut_ptr(),
        &lwork,
        iwork.as_mut_ptr(),
        &liwork,
        info,
    )
}

#[inline]
pub unsafe fn dstevx(
    jobz: u8,
    range: u8,
    n: i32,
    d: &mut [f64],
    e: &mut [f64],
    vl: f64,
    vu: f64,
    il: i32,
    iu: i32,
    abstol: f64,
    m: &mut i32,
    w: &mut [f64],
    z: &mut [f64],
    ldz: i32,
    work: &mut [f64],
    iwork: &mut [i32],
    ifail: &mut [i32],
    info: &mut i32,
) {
    ffi::dstevx_(
        &(jobz as c_char),
        &(range as c_char),
        &n,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        &vl,
        &vu,
        &il,
        &iu,
        &abstol,
        m,
        w.as_mut_ptr(),
        z.as_mut_ptr(),
        &ldz,
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
        ifail.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn sstevx(
    jobz: u8,
    range: u8,
    n: i32,
    d: &mut [f32],
    e: &mut [f32],
    vl: f32,
    vu: f32,
    il: i32,
    iu: i32,
    abstol: f32,
    m: &mut i32,
    w: &mut [f32],
    z: &mut [f32],
    ldz: i32,
    work: &mut [f32],
    iwork: &mut [i32],
    ifail: &mut [i32],
    info: &mut i32,
) {
    ffi::sstevx_(
        &(jobz as c_char),
        &(range as c_char),
        &n,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        &vl,
        &vu,
        &il,
        &iu,
        &abstol,
        m,
        w.as_mut_ptr(),
        z.as_mut_ptr(),
        &ldz,
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
        ifail.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn csycon(
    uplo: u8,
    n: i32,
    a: &[c32],
    lda: i32,
    ipiv: &[i32],
    anorm: f32,
    rcond: &mut f32,
    work: &mut [c32],
    info: &mut i32,
) {
    ffi::csycon_(
        &(uplo as c_char),
        &n,
        a.as_ptr() as *const _,
        &lda,
        ipiv.as_ptr(),
        &anorm,
        rcond,
        work.as_mut_ptr() as *mut _,
        info,
    )
}

#[inline]
pub unsafe fn dsycon(
    uplo: u8,
    n: i32,
    a: &[f64],
    lda: i32,
    ipiv: &[i32],
    anorm: f64,
    rcond: &mut f64,
    work: &mut [f64],
    iwork: &mut [i32],
    info: &mut i32,
) {
    ffi::dsycon_(
        &(uplo as c_char),
        &n,
        a.as_ptr(),
        &lda,
        ipiv.as_ptr(),
        &anorm,
        rcond,
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn ssycon(
    uplo: u8,
    n: i32,
    a: &[f32],
    lda: i32,
    ipiv: &[i32],
    anorm: f32,
    rcond: &mut f32,
    work: &mut [f32],
    iwork: &mut [i32],
    info: &mut i32,
) {
    ffi::ssycon_(
        &(uplo as c_char),
        &n,
        a.as_ptr(),
        &lda,
        ipiv.as_ptr(),
        &anorm,
        rcond,
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn zsycon(
    uplo: u8,
    n: i32,
    a: &[c64],
    lda: i32,
    ipiv: &[i32],
    anorm: f64,
    rcond: &mut f64,
    work: &mut [c64],
    info: &mut i32,
) {
    ffi::zsycon_(
        &(uplo as c_char),
        &n,
        a.as_ptr() as *const _,
        &lda,
        ipiv.as_ptr(),
        &anorm,
        rcond,
        work.as_mut_ptr() as *mut _,
        info,
    )
}

#[inline]
pub unsafe fn csycon_3(
    uplo: u8,
    n: i32,
    a: &[c32],
    lda: i32,
    e: &[c32],
    ipiv: &[i32],
    anorm: f32,
    rcond: &mut f32,
    work: &mut [c32],
    info: &mut i32,
) {
    ffi::csycon_3_(
        &(uplo as c_char),
        &n,
        a.as_ptr() as *const _,
        &lda,
        e.as_ptr() as *const _,
        ipiv.as_ptr(),
        &anorm,
        rcond,
        work.as_mut_ptr() as *mut _,
        info,
    )
}

#[inline]
pub unsafe fn dsycon_3(
    uplo: u8,
    n: i32,
    a: &[f64],
    lda: i32,
    e: &[f64],
    ipiv: &[i32],
    anorm: f64,
    rcond: &mut f64,
    work: &mut [f64],
    iwork: &mut [i32],
    info: &mut i32,
) {
    ffi::dsycon_3_(
        &(uplo as c_char),
        &n,
        a.as_ptr(),
        &lda,
        e.as_ptr(),
        ipiv.as_ptr(),
        &anorm,
        rcond,
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn ssycon_3(
    uplo: u8,
    n: i32,
    a: &[f32],
    lda: i32,
    e: &[f32],
    ipiv: &[i32],
    anorm: f32,
    rcond: &mut f32,
    work: &mut [f32],
    iwork: &mut [i32],
    info: &mut i32,
) {
    ffi::ssycon_3_(
        &(uplo as c_char),
        &n,
        a.as_ptr(),
        &lda,
        e.as_ptr(),
        ipiv.as_ptr(),
        &anorm,
        rcond,
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn zsycon_3(
    uplo: u8,
    n: i32,
    a: &[c64],
    lda: i32,
    e: &[c64],
    ipiv: &[i32],
    anorm: f64,
    rcond: &mut f64,
    work: &mut [c64],
    info: &mut i32,
) {
    ffi::zsycon_3_(
        &(uplo as c_char),
        &n,
        a.as_ptr() as *const _,
        &lda,
        e.as_ptr() as *const _,
        ipiv.as_ptr(),
        &anorm,
        rcond,
        work.as_mut_ptr() as *mut _,
        info,
    )
}

#[inline]
pub unsafe fn csyconv(
    uplo: u8,
    way: u8,
    n: i32,
    a: &mut [c32],
    lda: i32,
    ipiv: &[i32],
    e: &mut [c32],
    info: &mut i32,
) {
    ffi::csyconv_(
        &(uplo as c_char),
        &(way as c_char),
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        ipiv.as_ptr(),
        e.as_mut_ptr() as *mut _,
        info,
    )
}

#[inline]
pub unsafe fn dsyconv(
    uplo: u8,
    way: u8,
    n: i32,
    a: &mut [f64],
    lda: i32,
    ipiv: &[i32],
    e: &mut [f64],
    info: &mut i32,
) {
    ffi::dsyconv_(
        &(uplo as c_char),
        &(way as c_char),
        &n,
        a.as_mut_ptr(),
        &lda,
        ipiv.as_ptr(),
        e.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn ssyconv(
    uplo: u8,
    way: u8,
    n: i32,
    a: &mut [f32],
    lda: i32,
    ipiv: &[i32],
    e: &mut [f32],
    info: &mut i32,
) {
    ffi::ssyconv_(
        &(uplo as c_char),
        &(way as c_char),
        &n,
        a.as_mut_ptr(),
        &lda,
        ipiv.as_ptr(),
        e.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn zsyconv(
    uplo: u8,
    way: u8,
    n: i32,
    a: &mut [c64],
    lda: i32,
    ipiv: &[i32],
    e: &mut [c64],
    info: &mut i32,
) {
    ffi::zsyconv_(
        &(uplo as c_char),
        &(way as c_char),
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        ipiv.as_ptr(),
        e.as_mut_ptr() as *mut _,
        info,
    )
}

#[inline]
pub unsafe fn csyequb(
    uplo: u8,
    n: i32,
    a: &[c32],
    lda: i32,
    s: &mut [f32],
    scond: &mut [f32],
    amax: &mut f32,
    work: &mut [c32],
    info: &mut i32,
) {
    ffi::csyequb_(
        &(uplo as c_char),
        &n,
        a.as_ptr() as *const _,
        &lda,
        s.as_mut_ptr(),
        scond.as_mut_ptr(),
        amax,
        work.as_mut_ptr() as *mut _,
        info,
    )
}

#[inline]
pub unsafe fn dsyequb(
    uplo: u8,
    n: i32,
    a: &[f64],
    lda: i32,
    s: &mut [f64],
    scond: &mut [f64],
    amax: &mut f64,
    work: &mut [f64],
    info: &mut i32,
) {
    ffi::dsyequb_(
        &(uplo as c_char),
        &n,
        a.as_ptr(),
        &lda,
        s.as_mut_ptr(),
        scond.as_mut_ptr(),
        amax,
        work.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn ssyequb(
    uplo: u8,
    n: i32,
    a: &[f32],
    lda: i32,
    s: &mut [f32],
    scond: &mut [f32],
    amax: &mut f32,
    work: &mut [f32],
    info: &mut i32,
) {
    ffi::ssyequb_(
        &(uplo as c_char),
        &n,
        a.as_ptr(),
        &lda,
        s.as_mut_ptr(),
        scond.as_mut_ptr(),
        amax,
        work.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn zsyequb(
    uplo: u8,
    n: i32,
    a: &[c64],
    lda: i32,
    s: &mut [f64],
    scond: &mut [f64],
    amax: &mut f64,
    work: &mut [c64],
    info: &mut i32,
) {
    ffi::zsyequb_(
        &(uplo as c_char),
        &n,
        a.as_ptr() as *const _,
        &lda,
        s.as_mut_ptr(),
        scond.as_mut_ptr(),
        amax,
        work.as_mut_ptr() as *mut _,
        info,
    )
}

#[inline]
pub unsafe fn dsyev(
    jobz: u8,
    uplo: u8,
    n: i32,
    a: &mut [f64],
    lda: i32,
    w: &mut [f64],
    work: &mut [f64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::dsyev_(
        &(jobz as c_char),
        &(uplo as c_char),
        &n,
        a.as_mut_ptr(),
        &lda,
        w.as_mut_ptr(),
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn ssyev(
    jobz: u8,
    uplo: u8,
    n: i32,
    a: &mut [f32],
    lda: i32,
    w: &mut [f32],
    work: &mut [f32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::ssyev_(
        &(jobz as c_char),
        &(uplo as c_char),
        &n,
        a.as_mut_ptr(),
        &lda,
        w.as_mut_ptr(),
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn dsyev_2stage(
    jobz: u8,
    uplo: u8,
    n: i32,
    a: &mut [f64],
    lda: i32,
    w: &mut [f64],
    work: &mut [f64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::dsyev_2stage_(
        &(jobz as c_char),
        &(uplo as c_char),
        &n,
        a.as_mut_ptr(),
        &lda,
        w.as_mut_ptr(),
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn ssyev_2stage(
    jobz: u8,
    uplo: u8,
    n: i32,
    a: &mut [f32],
    lda: i32,
    w: &mut [f32],
    work: &mut [f32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::ssyev_2stage_(
        &(jobz as c_char),
        &(uplo as c_char),
        &n,
        a.as_mut_ptr(),
        &lda,
        w.as_mut_ptr(),
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn dsyevd(
    jobz: u8,
    uplo: u8,
    n: i32,
    a: &mut [f64],
    lda: i32,
    w: &mut [f64],
    work: &mut [f64],
    lwork: i32,
    iwork: &mut [i32],
    liwork: i32,
    info: &mut i32,
) {
    ffi::dsyevd_(
        &(jobz as c_char),
        &(uplo as c_char),
        &n,
        a.as_mut_ptr(),
        &lda,
        w.as_mut_ptr(),
        work.as_mut_ptr(),
        &lwork,
        iwork.as_mut_ptr(),
        &liwork,
        info,
    )
}

#[inline]
pub unsafe fn ssyevd(
    jobz: u8,
    uplo: u8,
    n: i32,
    a: &mut [f32],
    lda: i32,
    w: &mut [f32],
    work: &mut [f32],
    lwork: i32,
    iwork: &mut [i32],
    liwork: i32,
    info: &mut i32,
) {
    ffi::ssyevd_(
        &(jobz as c_char),
        &(uplo as c_char),
        &n,
        a.as_mut_ptr(),
        &lda,
        w.as_mut_ptr(),
        work.as_mut_ptr(),
        &lwork,
        iwork.as_mut_ptr(),
        &liwork,
        info,
    )
}

#[inline]
pub unsafe fn dsyevd_2stage(
    jobz: u8,
    uplo: u8,
    n: i32,
    a: &mut [f64],
    lda: i32,
    w: &mut [f64],
    work: &mut [f64],
    lwork: i32,
    iwork: &mut [i32],
    liwork: i32,
    info: &mut i32,
) {
    ffi::dsyevd_2stage_(
        &(jobz as c_char),
        &(uplo as c_char),
        &n,
        a.as_mut_ptr(),
        &lda,
        w.as_mut_ptr(),
        work.as_mut_ptr(),
        &lwork,
        iwork.as_mut_ptr(),
        &liwork,
        info,
    )
}

#[inline]
pub unsafe fn ssyevd_2stage(
    jobz: u8,
    uplo: u8,
    n: i32,
    a: &mut [f32],
    lda: i32,
    w: &mut [f32],
    work: &mut [f32],
    lwork: i32,
    iwork: &mut [i32],
    liwork: i32,
    info: &mut i32,
) {
    ffi::ssyevd_2stage_(
        &(jobz as c_char),
        &(uplo as c_char),
        &n,
        a.as_mut_ptr(),
        &lda,
        w.as_mut_ptr(),
        work.as_mut_ptr(),
        &lwork,
        iwork.as_mut_ptr(),
        &liwork,
        info,
    )
}

#[inline]
pub unsafe fn dsyevr(
    jobz: u8,
    range: u8,
    uplo: u8,
    n: i32,
    a: &mut [f64],
    lda: i32,
    vl: f64,
    vu: f64,
    il: i32,
    iu: i32,
    abstol: f64,
    m: &mut i32,
    w: &mut [f64],
    z: &mut [f64],
    ldz: i32,
    isuppz: &mut [i32],
    work: &mut [f64],
    lwork: i32,
    iwork: &mut [i32],
    liwork: i32,
    info: &mut i32,
) {
    ffi::dsyevr_(
        &(jobz as c_char),
        &(range as c_char),
        &(uplo as c_char),
        &n,
        a.as_mut_ptr(),
        &lda,
        &vl,
        &vu,
        &il,
        &iu,
        &abstol,
        m,
        w.as_mut_ptr(),
        z.as_mut_ptr(),
        &ldz,
        isuppz.as_mut_ptr(),
        work.as_mut_ptr(),
        &lwork,
        iwork.as_mut_ptr(),
        &liwork,
        info,
    )
}

#[inline]
pub unsafe fn ssyevr(
    jobz: u8,
    range: u8,
    uplo: u8,
    n: i32,
    a: &mut [f32],
    lda: i32,
    vl: f32,
    vu: f32,
    il: i32,
    iu: i32,
    abstol: f32,
    m: &mut i32,
    w: &mut [f32],
    z: &mut [f32],
    ldz: i32,
    isuppz: &mut [i32],
    work: &mut [f32],
    lwork: i32,
    iwork: &mut [i32],
    liwork: i32,
    info: &mut i32,
) {
    ffi::ssyevr_(
        &(jobz as c_char),
        &(range as c_char),
        &(uplo as c_char),
        &n,
        a.as_mut_ptr(),
        &lda,
        &vl,
        &vu,
        &il,
        &iu,
        &abstol,
        m,
        w.as_mut_ptr(),
        z.as_mut_ptr(),
        &ldz,
        isuppz.as_mut_ptr(),
        work.as_mut_ptr(),
        &lwork,
        iwork.as_mut_ptr(),
        &liwork,
        info,
    )
}

#[inline]
pub unsafe fn dsyevr_2stage(
    jobz: u8,
    range: u8,
    uplo: u8,
    n: i32,
    a: &mut [f64],
    lda: i32,
    vl: f64,
    vu: f64,
    il: i32,
    iu: i32,
    abstol: f64,
    m: &mut i32,
    w: &mut [f64],
    z: &mut [f64],
    ldz: i32,
    isuppz: &mut [i32],
    work: &mut [f64],
    lwork: i32,
    iwork: &mut [i32],
    liwork: i32,
    info: &mut i32,
) {
    ffi::dsyevr_2stage_(
        &(jobz as c_char),
        &(range as c_char),
        &(uplo as c_char),
        &n,
        a.as_mut_ptr(),
        &lda,
        &vl,
        &vu,
        &il,
        &iu,
        &abstol,
        m,
        w.as_mut_ptr(),
        z.as_mut_ptr(),
        &ldz,
        isuppz.as_mut_ptr(),
        work.as_mut_ptr(),
        &lwork,
        iwork.as_mut_ptr(),
        &liwork,
        info,
    )
}

#[inline]
pub unsafe fn ssyevr_2stage(
    jobz: u8,
    range: u8,
    uplo: u8,
    n: i32,
    a: &mut [f32],
    lda: i32,
    vl: f32,
    vu: f32,
    il: i32,
    iu: i32,
    abstol: f32,
    m: &mut i32,
    w: &mut [f32],
    z: &mut [f32],
    ldz: i32,
    isuppz: &mut [i32],
    work: &mut [f32],
    lwork: i32,
    iwork: &mut [i32],
    liwork: i32,
    info: &mut i32,
) {
    ffi::ssyevr_2stage_(
        &(jobz as c_char),
        &(range as c_char),
        &(uplo as c_char),
        &n,
        a.as_mut_ptr(),
        &lda,
        &vl,
        &vu,
        &il,
        &iu,
        &abstol,
        m,
        w.as_mut_ptr(),
        z.as_mut_ptr(),
        &ldz,
        isuppz.as_mut_ptr(),
        work.as_mut_ptr(),
        &lwork,
        iwork.as_mut_ptr(),
        &liwork,
        info,
    )
}

#[inline]
pub unsafe fn dsyevx(
    jobz: u8,
    range: u8,
    uplo: u8,
    n: i32,
    a: &mut [f64],
    lda: i32,
    vl: f64,
    vu: f64,
    il: i32,
    iu: i32,
    abstol: f64,
    m: &mut i32,
    w: &mut [f64],
    z: &mut [f64],
    ldz: i32,
    work: &mut [f64],
    lwork: i32,
    iwork: &mut [i32],
    ifail: &mut [i32],
    info: &mut i32,
) {
    ffi::dsyevx_(
        &(jobz as c_char),
        &(range as c_char),
        &(uplo as c_char),
        &n,
        a.as_mut_ptr(),
        &lda,
        &vl,
        &vu,
        &il,
        &iu,
        &abstol,
        m,
        w.as_mut_ptr(),
        z.as_mut_ptr(),
        &ldz,
        work.as_mut_ptr(),
        &lwork,
        iwork.as_mut_ptr(),
        ifail.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn ssyevx(
    jobz: u8,
    range: u8,
    uplo: u8,
    n: i32,
    a: &mut [f32],
    lda: i32,
    vl: f32,
    vu: f32,
    il: i32,
    iu: i32,
    abstol: f32,
    m: &mut i32,
    w: &mut [f32],
    z: &mut [f32],
    ldz: i32,
    work: &mut [f32],
    lwork: i32,
    iwork: &mut [i32],
    ifail: &mut [i32],
    info: &mut i32,
) {
    ffi::ssyevx_(
        &(jobz as c_char),
        &(range as c_char),
        &(uplo as c_char),
        &n,
        a.as_mut_ptr(),
        &lda,
        &vl,
        &vu,
        &il,
        &iu,
        &abstol,
        m,
        w.as_mut_ptr(),
        z.as_mut_ptr(),
        &ldz,
        work.as_mut_ptr(),
        &lwork,
        iwork.as_mut_ptr(),
        ifail.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn dsyevx_2stage(
    jobz: u8,
    range: u8,
    uplo: u8,
    n: i32,
    a: &mut [f64],
    lda: i32,
    vl: f64,
    vu: f64,
    il: i32,
    iu: i32,
    abstol: f64,
    m: &mut i32,
    w: &mut [f64],
    z: &mut [f64],
    ldz: i32,
    work: &mut [f64],
    lwork: i32,
    iwork: &mut [i32],
    ifail: &mut [i32],
    info: &mut i32,
) {
    ffi::dsyevx_2stage_(
        &(jobz as c_char),
        &(range as c_char),
        &(uplo as c_char),
        &n,
        a.as_mut_ptr(),
        &lda,
        &vl,
        &vu,
        &il,
        &iu,
        &abstol,
        m,
        w.as_mut_ptr(),
        z.as_mut_ptr(),
        &ldz,
        work.as_mut_ptr(),
        &lwork,
        iwork.as_mut_ptr(),
        ifail.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn ssyevx_2stage(
    jobz: u8,
    range: u8,
    uplo: u8,
    n: i32,
    a: &mut [f32],
    lda: i32,
    vl: f32,
    vu: f32,
    il: i32,
    iu: i32,
    abstol: f32,
    m: &mut i32,
    w: &mut [f32],
    z: &mut [f32],
    ldz: i32,
    work: &mut [f32],
    lwork: i32,
    iwork: &mut [i32],
    ifail: &mut [i32],
    info: &mut i32,
) {
    ffi::ssyevx_2stage_(
        &(jobz as c_char),
        &(range as c_char),
        &(uplo as c_char),
        &n,
        a.as_mut_ptr(),
        &lda,
        &vl,
        &vu,
        &il,
        &iu,
        &abstol,
        m,
        w.as_mut_ptr(),
        z.as_mut_ptr(),
        &ldz,
        work.as_mut_ptr(),
        &lwork,
        iwork.as_mut_ptr(),
        ifail.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn dsygst(
    itype: &[i32],
    uplo: u8,
    n: i32,
    a: &mut [f64],
    lda: i32,
    b: &[f64],
    ldb: i32,
    info: &mut i32,
) {
    ffi::dsygst_(
        itype.as_ptr(),
        &(uplo as c_char),
        &n,
        a.as_mut_ptr(),
        &lda,
        b.as_ptr(),
        &ldb,
        info,
    )
}

#[inline]
pub unsafe fn ssygst(
    itype: &[i32],
    uplo: u8,
    n: i32,
    a: &mut [f32],
    lda: i32,
    b: &[f32],
    ldb: i32,
    info: &mut i32,
) {
    ffi::ssygst_(
        itype.as_ptr(),
        &(uplo as c_char),
        &n,
        a.as_mut_ptr(),
        &lda,
        b.as_ptr(),
        &ldb,
        info,
    )
}

#[inline]
pub unsafe fn dsygv(
    itype: &[i32],
    jobz: u8,
    uplo: u8,
    n: i32,
    a: &mut [f64],
    lda: i32,
    b: &mut [f64],
    ldb: i32,
    w: &mut [f64],
    work: &mut [f64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::dsygv_(
        itype.as_ptr(),
        &(jobz as c_char),
        &(uplo as c_char),
        &n,
        a.as_mut_ptr(),
        &lda,
        b.as_mut_ptr(),
        &ldb,
        w.as_mut_ptr(),
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn ssygv(
    itype: &[i32],
    jobz: u8,
    uplo: u8,
    n: i32,
    a: &mut [f32],
    lda: i32,
    b: &mut [f32],
    ldb: i32,
    w: &mut [f32],
    work: &mut [f32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::ssygv_(
        itype.as_ptr(),
        &(jobz as c_char),
        &(uplo as c_char),
        &n,
        a.as_mut_ptr(),
        &lda,
        b.as_mut_ptr(),
        &ldb,
        w.as_mut_ptr(),
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn dsygv_2stage(
    itype: &[i32],
    jobz: u8,
    uplo: u8,
    n: i32,
    a: &mut [f64],
    lda: i32,
    b: &mut [f64],
    ldb: i32,
    w: &mut [f64],
    work: &mut [f64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::dsygv_2stage_(
        itype.as_ptr(),
        &(jobz as c_char),
        &(uplo as c_char),
        &n,
        a.as_mut_ptr(),
        &lda,
        b.as_mut_ptr(),
        &ldb,
        w.as_mut_ptr(),
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn ssygv_2stage(
    itype: &[i32],
    jobz: u8,
    uplo: u8,
    n: i32,
    a: &mut [f32],
    lda: i32,
    b: &mut [f32],
    ldb: i32,
    w: &mut [f32],
    work: &mut [f32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::ssygv_2stage_(
        itype.as_ptr(),
        &(jobz as c_char),
        &(uplo as c_char),
        &n,
        a.as_mut_ptr(),
        &lda,
        b.as_mut_ptr(),
        &ldb,
        w.as_mut_ptr(),
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn dsygvd(
    itype: &[i32],
    jobz: u8,
    uplo: u8,
    n: i32,
    a: &mut [f64],
    lda: i32,
    b: &mut [f64],
    ldb: i32,
    w: &mut [f64],
    work: &mut [f64],
    lwork: i32,
    iwork: &mut [i32],
    liwork: i32,
    info: &mut i32,
) {
    ffi::dsygvd_(
        itype.as_ptr(),
        &(jobz as c_char),
        &(uplo as c_char),
        &n,
        a.as_mut_ptr(),
        &lda,
        b.as_mut_ptr(),
        &ldb,
        w.as_mut_ptr(),
        work.as_mut_ptr(),
        &lwork,
        iwork.as_mut_ptr(),
        &liwork,
        info,
    )
}

#[inline]
pub unsafe fn ssygvd(
    itype: &[i32],
    jobz: u8,
    uplo: u8,
    n: i32,
    a: &mut [f32],
    lda: i32,
    b: &mut [f32],
    ldb: i32,
    w: &mut [f32],
    work: &mut [f32],
    lwork: i32,
    iwork: &mut [i32],
    liwork: i32,
    info: &mut i32,
) {
    ffi::ssygvd_(
        itype.as_ptr(),
        &(jobz as c_char),
        &(uplo as c_char),
        &n,
        a.as_mut_ptr(),
        &lda,
        b.as_mut_ptr(),
        &ldb,
        w.as_mut_ptr(),
        work.as_mut_ptr(),
        &lwork,
        iwork.as_mut_ptr(),
        &liwork,
        info,
    )
}

#[inline]
pub unsafe fn dsygvx(
    itype: &[i32],
    jobz: u8,
    range: u8,
    uplo: u8,
    n: i32,
    a: &mut [f64],
    lda: i32,
    b: &mut [f64],
    ldb: i32,
    vl: f64,
    vu: f64,
    il: i32,
    iu: i32,
    abstol: f64,
    m: &mut i32,
    w: &mut [f64],
    z: &mut [f64],
    ldz: i32,
    work: &mut [f64],
    lwork: i32,
    iwork: &mut [i32],
    ifail: &mut [i32],
    info: &mut i32,
) {
    ffi::dsygvx_(
        itype.as_ptr(),
        &(jobz as c_char),
        &(range as c_char),
        &(uplo as c_char),
        &n,
        a.as_mut_ptr(),
        &lda,
        b.as_mut_ptr(),
        &ldb,
        &vl,
        &vu,
        &il,
        &iu,
        &abstol,
        m,
        w.as_mut_ptr(),
        z.as_mut_ptr(),
        &ldz,
        work.as_mut_ptr(),
        &lwork,
        iwork.as_mut_ptr(),
        ifail.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn ssygvx(
    itype: &[i32],
    jobz: u8,
    range: u8,
    uplo: u8,
    n: i32,
    a: &mut [f32],
    lda: i32,
    b: &mut [f32],
    ldb: i32,
    vl: f32,
    vu: f32,
    il: i32,
    iu: i32,
    abstol: f32,
    m: &mut i32,
    w: &mut [f32],
    z: &mut [f32],
    ldz: i32,
    work: &mut [f32],
    lwork: i32,
    iwork: &mut [i32],
    ifail: &mut [i32],
    info: &mut i32,
) {
    ffi::ssygvx_(
        itype.as_ptr(),
        &(jobz as c_char),
        &(range as c_char),
        &(uplo as c_char),
        &n,
        a.as_mut_ptr(),
        &lda,
        b.as_mut_ptr(),
        &ldb,
        &vl,
        &vu,
        &il,
        &iu,
        &abstol,
        m,
        w.as_mut_ptr(),
        z.as_mut_ptr(),
        &ldz,
        work.as_mut_ptr(),
        &lwork,
        iwork.as_mut_ptr(),
        ifail.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn csyr(uplo: u8, n: i32, alpha: &[c32], x: &[c32], incx: i32, a: &mut [c32], lda: i32) {
    ffi::csyr_(
        &(uplo as c_char),
        &n,
        alpha.as_ptr() as *const _,
        x.as_ptr() as *const _,
        &incx,
        a.as_mut_ptr() as *mut _,
        &lda,
    )
}

#[inline]
pub unsafe fn zsyr(uplo: u8, n: i32, alpha: &[c64], x: &[c64], incx: i32, a: &mut [c64], lda: i32) {
    ffi::zsyr_(
        &(uplo as c_char),
        &n,
        alpha.as_ptr() as *const _,
        x.as_ptr() as *const _,
        &incx,
        a.as_mut_ptr() as *mut _,
        &lda,
    )
}

#[inline]
pub unsafe fn csyrfs(
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &[c32],
    lda: i32,
    af: &[c32],
    ldaf: i32,
    ipiv: &[i32],
    b: &[c32],
    ldb: i32,
    x: &mut [c32],
    ldx: i32,
    ferr: &mut [f32],
    berr: &mut [f32],
    work: &mut [c32],
    rwork: &mut [f32],
    info: &mut i32,
) {
    ffi::csyrfs_(
        &(uplo as c_char),
        &n,
        &nrhs,
        a.as_ptr() as *const _,
        &lda,
        af.as_ptr() as *const _,
        &ldaf,
        ipiv.as_ptr(),
        b.as_ptr() as *const _,
        &ldb,
        x.as_mut_ptr() as *mut _,
        &ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn dsyrfs(
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &[f64],
    lda: i32,
    af: &[f64],
    ldaf: i32,
    ipiv: &[i32],
    b: &[f64],
    ldb: i32,
    x: &mut [f64],
    ldx: i32,
    ferr: &mut [f64],
    berr: &mut [f64],
    work: &mut [f64],
    iwork: &mut [i32],
    info: &mut i32,
) {
    ffi::dsyrfs_(
        &(uplo as c_char),
        &n,
        &nrhs,
        a.as_ptr(),
        &lda,
        af.as_ptr(),
        &ldaf,
        ipiv.as_ptr(),
        b.as_ptr(),
        &ldb,
        x.as_mut_ptr(),
        &ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn ssyrfs(
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &[f32],
    lda: i32,
    af: &[f32],
    ldaf: i32,
    ipiv: &[i32],
    b: &[f32],
    ldb: i32,
    x: &mut [f32],
    ldx: i32,
    ferr: &mut [f32],
    berr: &mut [f32],
    work: &mut [f32],
    iwork: &mut [i32],
    info: &mut i32,
) {
    ffi::ssyrfs_(
        &(uplo as c_char),
        &n,
        &nrhs,
        a.as_ptr(),
        &lda,
        af.as_ptr(),
        &ldaf,
        ipiv.as_ptr(),
        b.as_ptr(),
        &ldb,
        x.as_mut_ptr(),
        &ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn zsyrfs(
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &[c64],
    lda: i32,
    af: &[c64],
    ldaf: i32,
    ipiv: &[i32],
    b: &[c64],
    ldb: i32,
    x: &mut [c64],
    ldx: i32,
    ferr: &mut [f64],
    berr: &mut [f64],
    work: &mut [c64],
    rwork: &mut [f64],
    info: &mut i32,
) {
    ffi::zsyrfs_(
        &(uplo as c_char),
        &n,
        &nrhs,
        a.as_ptr() as *const _,
        &lda,
        af.as_ptr() as *const _,
        &ldaf,
        ipiv.as_ptr(),
        b.as_ptr() as *const _,
        &ldb,
        x.as_mut_ptr() as *mut _,
        &ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn csyrfsx(
    uplo: u8,
    equed: u8,
    n: i32,
    nrhs: i32,
    a: &[c32],
    lda: i32,
    af: &[c32],
    ldaf: i32,
    ipiv: &[i32],
    s: &mut [f32],
    b: &[c32],
    ldb: i32,
    x: &mut [c32],
    ldx: i32,
    rcond: &mut f32,
    berr: &mut [f32],
    n_err_bnds: i32,
    err_bnds_norm: &mut [f32],
    err_bnds_comp: &mut [f32],
    nparams: &[i32],
    params: &mut [f32],
    work: &mut [c32],
    rwork: &mut [f32],
    info: &mut i32,
) {
    ffi::csyrfsx_(
        &(uplo as c_char),
        &(equed as c_char),
        &n,
        &nrhs,
        a.as_ptr() as *const _,
        &lda,
        af.as_ptr() as *const _,
        &ldaf,
        ipiv.as_ptr(),
        s.as_mut_ptr(),
        b.as_ptr() as *const _,
        &ldb,
        x.as_mut_ptr() as *mut _,
        &ldx,
        rcond,
        berr.as_mut_ptr(),
        &n_err_bnds,
        err_bnds_norm.as_mut_ptr(),
        err_bnds_comp.as_mut_ptr(),
        nparams.as_ptr(),
        params.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn dsyrfsx(
    uplo: u8,
    equed: u8,
    n: i32,
    nrhs: i32,
    a: &[f64],
    lda: i32,
    af: &[f64],
    ldaf: i32,
    ipiv: &[i32],
    s: &mut [f64],
    b: &[f64],
    ldb: i32,
    x: &mut [f64],
    ldx: i32,
    rcond: &mut f64,
    berr: &mut [f64],
    n_err_bnds: i32,
    err_bnds_norm: &mut [f64],
    err_bnds_comp: &mut [f64],
    nparams: &[i32],
    params: &mut [f64],
    work: &mut [f64],
    iwork: &mut [i32],
    info: &mut i32,
) {
    ffi::dsyrfsx_(
        &(uplo as c_char),
        &(equed as c_char),
        &n,
        &nrhs,
        a.as_ptr(),
        &lda,
        af.as_ptr(),
        &ldaf,
        ipiv.as_ptr(),
        s.as_mut_ptr(),
        b.as_ptr(),
        &ldb,
        x.as_mut_ptr(),
        &ldx,
        rcond,
        berr.as_mut_ptr(),
        &n_err_bnds,
        err_bnds_norm.as_mut_ptr(),
        err_bnds_comp.as_mut_ptr(),
        nparams.as_ptr(),
        params.as_mut_ptr(),
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn ssyrfsx(
    uplo: u8,
    equed: u8,
    n: i32,
    nrhs: i32,
    a: &[f32],
    lda: i32,
    af: &[f32],
    ldaf: i32,
    ipiv: &[i32],
    s: &mut [f32],
    b: &[f32],
    ldb: i32,
    x: &mut [f32],
    ldx: i32,
    rcond: &mut f32,
    berr: &mut [f32],
    n_err_bnds: i32,
    err_bnds_norm: &mut [f32],
    err_bnds_comp: &mut [f32],
    nparams: &[i32],
    params: &mut [f32],
    work: &mut [f32],
    iwork: &mut [i32],
    info: &mut i32,
) {
    ffi::ssyrfsx_(
        &(uplo as c_char),
        &(equed as c_char),
        &n,
        &nrhs,
        a.as_ptr(),
        &lda,
        af.as_ptr(),
        &ldaf,
        ipiv.as_ptr(),
        s.as_mut_ptr(),
        b.as_ptr(),
        &ldb,
        x.as_mut_ptr(),
        &ldx,
        rcond,
        berr.as_mut_ptr(),
        &n_err_bnds,
        err_bnds_norm.as_mut_ptr(),
        err_bnds_comp.as_mut_ptr(),
        nparams.as_ptr(),
        params.as_mut_ptr(),
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn zsyrfsx(
    uplo: u8,
    equed: u8,
    n: i32,
    nrhs: i32,
    a: &[c64],
    lda: i32,
    af: &[c64],
    ldaf: i32,
    ipiv: &[i32],
    s: &mut [f64],
    b: &[c64],
    ldb: i32,
    x: &mut [c64],
    ldx: i32,
    rcond: &mut f64,
    berr: &mut [f64],
    n_err_bnds: i32,
    err_bnds_norm: &mut [f64],
    err_bnds_comp: &mut [f64],
    nparams: &[i32],
    params: &mut [f64],
    work: &mut [c64],
    rwork: &mut [f64],
    info: &mut i32,
) {
    ffi::zsyrfsx_(
        &(uplo as c_char),
        &(equed as c_char),
        &n,
        &nrhs,
        a.as_ptr() as *const _,
        &lda,
        af.as_ptr() as *const _,
        &ldaf,
        ipiv.as_ptr(),
        s.as_mut_ptr(),
        b.as_ptr() as *const _,
        &ldb,
        x.as_mut_ptr() as *mut _,
        &ldx,
        rcond,
        berr.as_mut_ptr(),
        &n_err_bnds,
        err_bnds_norm.as_mut_ptr(),
        err_bnds_comp.as_mut_ptr(),
        nparams.as_ptr(),
        params.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn csysv(
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &mut [c32],
    lda: i32,
    ipiv: &mut [i32],
    b: &mut [c32],
    ldb: i32,
    work: &mut [c32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::csysv_(
        &(uplo as c_char),
        &n,
        &nrhs,
        a.as_mut_ptr() as *mut _,
        &lda,
        ipiv.as_mut_ptr(),
        b.as_mut_ptr() as *mut _,
        &ldb,
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn dsysv(
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &mut [f64],
    lda: i32,
    ipiv: &mut [i32],
    b: &mut [f64],
    ldb: i32,
    work: &mut [f64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::dsysv_(
        &(uplo as c_char),
        &n,
        &nrhs,
        a.as_mut_ptr(),
        &lda,
        ipiv.as_mut_ptr(),
        b.as_mut_ptr(),
        &ldb,
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn ssysv(
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &mut [f32],
    lda: i32,
    ipiv: &mut [i32],
    b: &mut [f32],
    ldb: i32,
    work: &mut [f32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::ssysv_(
        &(uplo as c_char),
        &n,
        &nrhs,
        a.as_mut_ptr(),
        &lda,
        ipiv.as_mut_ptr(),
        b.as_mut_ptr(),
        &ldb,
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn zsysv(
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &mut [c64],
    lda: i32,
    ipiv: &mut [i32],
    b: &mut [c64],
    ldb: i32,
    work: &mut [c64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::zsysv_(
        &(uplo as c_char),
        &n,
        &nrhs,
        a.as_mut_ptr() as *mut _,
        &lda,
        ipiv.as_mut_ptr(),
        b.as_mut_ptr() as *mut _,
        &ldb,
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn csysv_aa(
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &mut [c32],
    lda: i32,
    ipiv: &mut [i32],
    b: &mut [c32],
    ldb: i32,
    work: &mut [c32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::csysv_aa_(
        &(uplo as c_char),
        &n,
        &nrhs,
        a.as_mut_ptr() as *mut _,
        &lda,
        ipiv.as_mut_ptr(),
        b.as_mut_ptr() as *mut _,
        &ldb,
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn dsysv_aa(
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &mut [f64],
    lda: i32,
    ipiv: &mut [i32],
    b: &mut [f64],
    ldb: i32,
    work: &mut [f64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::dsysv_aa_(
        &(uplo as c_char),
        &n,
        &nrhs,
        a.as_mut_ptr(),
        &lda,
        ipiv.as_mut_ptr(),
        b.as_mut_ptr(),
        &ldb,
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn ssysv_aa(
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &mut [f32],
    lda: i32,
    ipiv: &mut [i32],
    b: &mut [f32],
    ldb: i32,
    work: &mut [f32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::ssysv_aa_(
        &(uplo as c_char),
        &n,
        &nrhs,
        a.as_mut_ptr(),
        &lda,
        ipiv.as_mut_ptr(),
        b.as_mut_ptr(),
        &ldb,
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn zsysv_aa(
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &mut [c64],
    lda: i32,
    ipiv: &mut [i32],
    b: &mut [c64],
    ldb: i32,
    work: &mut [c64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::zsysv_aa_(
        &(uplo as c_char),
        &n,
        &nrhs,
        a.as_mut_ptr() as *mut _,
        &lda,
        ipiv.as_mut_ptr(),
        b.as_mut_ptr() as *mut _,
        &ldb,
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn csysv_aa_2stage(
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &mut [c32],
    lda: i32,
    tb: &mut [c32],
    ltb: &[i32],
    ipiv: &mut [i32],
    ipiv2: &mut [i32],
    b: &mut [c32],
    ldb: i32,
    work: &mut [c32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::csysv_aa_2stage_(
        &(uplo as c_char),
        &n,
        &nrhs,
        a.as_mut_ptr() as *mut _,
        &lda,
        tb.as_mut_ptr() as *mut _,
        ltb.as_ptr(),
        ipiv.as_mut_ptr(),
        ipiv2.as_mut_ptr(),
        b.as_mut_ptr() as *mut _,
        &ldb,
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn dsysv_aa_2stage(
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &mut [f64],
    lda: i32,
    tb: &mut [f64],
    ltb: &[i32],
    ipiv: &mut [i32],
    ipiv2: &mut [i32],
    b: &mut [f64],
    ldb: i32,
    work: &mut [f64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::dsysv_aa_2stage_(
        &(uplo as c_char),
        &n,
        &nrhs,
        a.as_mut_ptr(),
        &lda,
        tb.as_mut_ptr(),
        ltb.as_ptr(),
        ipiv.as_mut_ptr(),
        ipiv2.as_mut_ptr(),
        b.as_mut_ptr(),
        &ldb,
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn ssysv_aa_2stage(
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &mut [f32],
    lda: i32,
    tb: &mut [f32],
    ltb: &[i32],
    ipiv: &mut [i32],
    ipiv2: &mut [i32],
    b: &mut [f32],
    ldb: i32,
    work: &mut [f32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::ssysv_aa_2stage_(
        &(uplo as c_char),
        &n,
        &nrhs,
        a.as_mut_ptr(),
        &lda,
        tb.as_mut_ptr(),
        ltb.as_ptr(),
        ipiv.as_mut_ptr(),
        ipiv2.as_mut_ptr(),
        b.as_mut_ptr(),
        &ldb,
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn zsysv_aa_2stage(
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &mut [c64],
    lda: i32,
    tb: &mut [c64],
    ltb: &[i32],
    ipiv: &mut [i32],
    ipiv2: &mut [i32],
    b: &mut [c64],
    ldb: i32,
    work: &mut [c64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::zsysv_aa_2stage_(
        &(uplo as c_char),
        &n,
        &nrhs,
        a.as_mut_ptr() as *mut _,
        &lda,
        tb.as_mut_ptr() as *mut _,
        ltb.as_ptr(),
        ipiv.as_mut_ptr(),
        ipiv2.as_mut_ptr(),
        b.as_mut_ptr() as *mut _,
        &ldb,
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn csysv_rk(
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &mut [c32],
    lda: i32,
    e: &mut [c32],
    ipiv: &mut [i32],
    b: &mut [c32],
    ldb: i32,
    work: &mut [c32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::csysv_rk_(
        &(uplo as c_char),
        &n,
        &nrhs,
        a.as_mut_ptr() as *mut _,
        &lda,
        e.as_mut_ptr() as *mut _,
        ipiv.as_mut_ptr(),
        b.as_mut_ptr() as *mut _,
        &ldb,
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn dsysv_rk(
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &mut [f64],
    lda: i32,
    e: &mut [f64],
    ipiv: &mut [i32],
    b: &mut [f64],
    ldb: i32,
    work: &mut [f64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::dsysv_rk_(
        &(uplo as c_char),
        &n,
        &nrhs,
        a.as_mut_ptr(),
        &lda,
        e.as_mut_ptr(),
        ipiv.as_mut_ptr(),
        b.as_mut_ptr(),
        &ldb,
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn ssysv_rk(
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &mut [f32],
    lda: i32,
    e: &mut [f32],
    ipiv: &mut [i32],
    b: &mut [f32],
    ldb: i32,
    work: &mut [f32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::ssysv_rk_(
        &(uplo as c_char),
        &n,
        &nrhs,
        a.as_mut_ptr(),
        &lda,
        e.as_mut_ptr(),
        ipiv.as_mut_ptr(),
        b.as_mut_ptr(),
        &ldb,
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn zsysv_rk(
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &mut [c64],
    lda: i32,
    e: &mut [c64],
    ipiv: &mut [i32],
    b: &mut [c64],
    ldb: i32,
    work: &mut [c64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::zsysv_rk_(
        &(uplo as c_char),
        &n,
        &nrhs,
        a.as_mut_ptr() as *mut _,
        &lda,
        e.as_mut_ptr() as *mut _,
        ipiv.as_mut_ptr(),
        b.as_mut_ptr() as *mut _,
        &ldb,
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn csysv_rook(
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &mut [c32],
    lda: i32,
    ipiv: &mut [i32],
    b: &mut [c32],
    ldb: i32,
    work: &mut [c32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::csysv_rook_(
        &(uplo as c_char),
        &n,
        &nrhs,
        a.as_mut_ptr() as *mut _,
        &lda,
        ipiv.as_mut_ptr(),
        b.as_mut_ptr() as *mut _,
        &ldb,
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn dsysv_rook(
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &mut [f64],
    lda: i32,
    ipiv: &mut [i32],
    b: &mut [f64],
    ldb: i32,
    work: &mut [f64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::dsysv_rook_(
        &(uplo as c_char),
        &n,
        &nrhs,
        a.as_mut_ptr(),
        &lda,
        ipiv.as_mut_ptr(),
        b.as_mut_ptr(),
        &ldb,
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn ssysv_rook(
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &mut [f32],
    lda: i32,
    ipiv: &mut [i32],
    b: &mut [f32],
    ldb: i32,
    work: &mut [f32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::ssysv_rook_(
        &(uplo as c_char),
        &n,
        &nrhs,
        a.as_mut_ptr(),
        &lda,
        ipiv.as_mut_ptr(),
        b.as_mut_ptr(),
        &ldb,
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn zsysv_rook(
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &mut [c64],
    lda: i32,
    ipiv: &mut [i32],
    b: &mut [c64],
    ldb: i32,
    work: &mut [c64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::zsysv_rook_(
        &(uplo as c_char),
        &n,
        &nrhs,
        a.as_mut_ptr() as *mut _,
        &lda,
        ipiv.as_mut_ptr(),
        b.as_mut_ptr() as *mut _,
        &ldb,
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn csysvx(
    fact: u8,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &[c32],
    lda: i32,
    af: &mut [c32],
    ldaf: i32,
    ipiv: &mut [i32],
    b: &[c32],
    ldb: i32,
    x: &mut [c32],
    ldx: i32,
    rcond: &mut f32,
    ferr: &mut [f32],
    berr: &mut [f32],
    work: &mut [c32],
    lwork: i32,
    rwork: &mut [f32],
    info: &mut i32,
) {
    ffi::csysvx_(
        &(fact as c_char),
        &(uplo as c_char),
        &n,
        &nrhs,
        a.as_ptr() as *const _,
        &lda,
        af.as_mut_ptr() as *mut _,
        &ldaf,
        ipiv.as_mut_ptr(),
        b.as_ptr() as *const _,
        &ldb,
        x.as_mut_ptr() as *mut _,
        &ldx,
        rcond,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        &lwork,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn dsysvx(
    fact: u8,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &[f64],
    lda: i32,
    af: &mut [f64],
    ldaf: i32,
    ipiv: &mut [i32],
    b: &[f64],
    ldb: i32,
    x: &mut [f64],
    ldx: i32,
    rcond: &mut f64,
    ferr: &mut [f64],
    berr: &mut [f64],
    work: &mut [f64],
    lwork: i32,
    iwork: &mut [i32],
    info: &mut i32,
) {
    ffi::dsysvx_(
        &(fact as c_char),
        &(uplo as c_char),
        &n,
        &nrhs,
        a.as_ptr(),
        &lda,
        af.as_mut_ptr(),
        &ldaf,
        ipiv.as_mut_ptr(),
        b.as_ptr(),
        &ldb,
        x.as_mut_ptr(),
        &ldx,
        rcond,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr(),
        &lwork,
        iwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn ssysvx(
    fact: u8,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &[f32],
    lda: i32,
    af: &mut [f32],
    ldaf: i32,
    ipiv: &mut [i32],
    b: &[f32],
    ldb: i32,
    x: &mut [f32],
    ldx: i32,
    rcond: &mut f32,
    ferr: &mut [f32],
    berr: &mut [f32],
    work: &mut [f32],
    lwork: i32,
    iwork: &mut [i32],
    info: &mut i32,
) {
    ffi::ssysvx_(
        &(fact as c_char),
        &(uplo as c_char),
        &n,
        &nrhs,
        a.as_ptr(),
        &lda,
        af.as_mut_ptr(),
        &ldaf,
        ipiv.as_mut_ptr(),
        b.as_ptr(),
        &ldb,
        x.as_mut_ptr(),
        &ldx,
        rcond,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr(),
        &lwork,
        iwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn zsysvx(
    fact: u8,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &[c64],
    lda: i32,
    af: &mut [c64],
    ldaf: i32,
    ipiv: &mut [i32],
    b: &[c64],
    ldb: i32,
    x: &mut [c64],
    ldx: i32,
    rcond: &mut f64,
    ferr: &mut [f64],
    berr: &mut [f64],
    work: &mut [c64],
    lwork: i32,
    rwork: &mut [f64],
    info: &mut i32,
) {
    ffi::zsysvx_(
        &(fact as c_char),
        &(uplo as c_char),
        &n,
        &nrhs,
        a.as_ptr() as *const _,
        &lda,
        af.as_mut_ptr() as *mut _,
        &ldaf,
        ipiv.as_mut_ptr(),
        b.as_ptr() as *const _,
        &ldb,
        x.as_mut_ptr() as *mut _,
        &ldx,
        rcond,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        &lwork,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn csysvxx(
    fact: u8,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &mut [c32],
    lda: i32,
    af: &mut [c32],
    ldaf: i32,
    ipiv: &mut [i32],
    equed: &mut u8,
    s: &mut [f32],
    b: &mut [c32],
    ldb: i32,
    x: &mut [c32],
    ldx: i32,
    rcond: &mut f32,
    rpvgrw: &mut f32,
    berr: &mut [f32],
    n_err_bnds: i32,
    err_bnds_norm: &mut [f32],
    err_bnds_comp: &mut [f32],
    nparams: &[i32],
    params: &mut [f32],
    work: &mut [c32],
    rwork: &mut [f32],
    info: &mut i32,
) {
    ffi::csysvxx_(
        &(fact as c_char),
        &(uplo as c_char),
        &n,
        &nrhs,
        a.as_mut_ptr() as *mut _,
        &lda,
        af.as_mut_ptr() as *mut _,
        &ldaf,
        ipiv.as_mut_ptr(),
        equed as *mut _ as *mut _,
        s.as_mut_ptr(),
        b.as_mut_ptr() as *mut _,
        &ldb,
        x.as_mut_ptr() as *mut _,
        &ldx,
        rcond,
        rpvgrw,
        berr.as_mut_ptr(),
        &n_err_bnds,
        err_bnds_norm.as_mut_ptr(),
        err_bnds_comp.as_mut_ptr(),
        nparams.as_ptr(),
        params.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn dsysvxx(
    fact: u8,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &mut [f64],
    lda: i32,
    af: &mut [f64],
    ldaf: i32,
    ipiv: &mut [i32],
    equed: &mut u8,
    s: &mut [f64],
    b: &mut [f64],
    ldb: i32,
    x: &mut [f64],
    ldx: i32,
    rcond: &mut f64,
    rpvgrw: &mut f64,
    berr: &mut [f64],
    n_err_bnds: i32,
    err_bnds_norm: &mut [f64],
    err_bnds_comp: &mut [f64],
    nparams: &[i32],
    params: &mut [f64],
    work: &mut [f64],
    iwork: &mut [i32],
    info: &mut i32,
) {
    ffi::dsysvxx_(
        &(fact as c_char),
        &(uplo as c_char),
        &n,
        &nrhs,
        a.as_mut_ptr(),
        &lda,
        af.as_mut_ptr(),
        &ldaf,
        ipiv.as_mut_ptr(),
        equed as *mut _ as *mut _,
        s.as_mut_ptr(),
        b.as_mut_ptr(),
        &ldb,
        x.as_mut_ptr(),
        &ldx,
        rcond,
        rpvgrw,
        berr.as_mut_ptr(),
        &n_err_bnds,
        err_bnds_norm.as_mut_ptr(),
        err_bnds_comp.as_mut_ptr(),
        nparams.as_ptr(),
        params.as_mut_ptr(),
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn ssysvxx(
    fact: u8,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &mut [f32],
    lda: i32,
    af: &mut [f32],
    ldaf: i32,
    ipiv: &mut [i32],
    equed: &mut u8,
    s: &mut [f32],
    b: &mut [f32],
    ldb: i32,
    x: &mut [f32],
    ldx: i32,
    rcond: &mut f32,
    rpvgrw: &mut f32,
    berr: &mut [f32],
    n_err_bnds: i32,
    err_bnds_norm: &mut [f32],
    err_bnds_comp: &mut [f32],
    nparams: &[i32],
    params: &mut [f32],
    work: &mut [f32],
    iwork: &mut [i32],
    info: &mut i32,
) {
    ffi::ssysvxx_(
        &(fact as c_char),
        &(uplo as c_char),
        &n,
        &nrhs,
        a.as_mut_ptr(),
        &lda,
        af.as_mut_ptr(),
        &ldaf,
        ipiv.as_mut_ptr(),
        equed as *mut _ as *mut _,
        s.as_mut_ptr(),
        b.as_mut_ptr(),
        &ldb,
        x.as_mut_ptr(),
        &ldx,
        rcond,
        rpvgrw,
        berr.as_mut_ptr(),
        &n_err_bnds,
        err_bnds_norm.as_mut_ptr(),
        err_bnds_comp.as_mut_ptr(),
        nparams.as_ptr(),
        params.as_mut_ptr(),
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn zsysvxx(
    fact: u8,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &mut [c64],
    lda: i32,
    af: &mut [c64],
    ldaf: i32,
    ipiv: &mut [i32],
    equed: &mut u8,
    s: &mut [f64],
    b: &mut [c64],
    ldb: i32,
    x: &mut [c64],
    ldx: i32,
    rcond: &mut f64,
    rpvgrw: &mut f64,
    berr: &mut [f64],
    n_err_bnds: i32,
    err_bnds_norm: &mut [f64],
    err_bnds_comp: &mut [f64],
    nparams: &[i32],
    params: &mut [f64],
    work: &mut [c64],
    rwork: &mut [f64],
    info: &mut i32,
) {
    ffi::zsysvxx_(
        &(fact as c_char),
        &(uplo as c_char),
        &n,
        &nrhs,
        a.as_mut_ptr() as *mut _,
        &lda,
        af.as_mut_ptr() as *mut _,
        &ldaf,
        ipiv.as_mut_ptr(),
        equed as *mut _ as *mut _,
        s.as_mut_ptr(),
        b.as_mut_ptr() as *mut _,
        &ldb,
        x.as_mut_ptr() as *mut _,
        &ldx,
        rcond,
        rpvgrw,
        berr.as_mut_ptr(),
        &n_err_bnds,
        err_bnds_norm.as_mut_ptr(),
        err_bnds_comp.as_mut_ptr(),
        nparams.as_ptr(),
        params.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn csyswapr(uplo: u8, n: i32, a: &mut [c32], lda: i32, i1: &[i32], i2: &[i32]) {
    ffi::csyswapr_(
        &(uplo as c_char),
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        i1.as_ptr(),
        i2.as_ptr(),
    )
}

#[inline]
pub unsafe fn dsyswapr(uplo: u8, n: i32, a: &mut [f64], lda: i32, i1: &[i32], i2: &[i32]) {
    ffi::dsyswapr_(
        &(uplo as c_char),
        &n,
        a.as_mut_ptr(),
        &lda,
        i1.as_ptr(),
        i2.as_ptr(),
    )
}

#[inline]
pub unsafe fn ssyswapr(uplo: u8, n: i32, a: &mut [f32], lda: i32, i1: &[i32], i2: &[i32]) {
    ffi::ssyswapr_(
        &(uplo as c_char),
        &n,
        a.as_mut_ptr(),
        &lda,
        i1.as_ptr(),
        i2.as_ptr(),
    )
}

#[inline]
pub unsafe fn zsyswapr(uplo: u8, n: i32, a: &mut [c64], lda: i32, i1: &[i32], i2: &[i32]) {
    ffi::zsyswapr_(
        &(uplo as c_char),
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        i1.as_ptr(),
        i2.as_ptr(),
    )
}

#[inline]
pub unsafe fn dsytrd(
    uplo: u8,
    n: i32,
    a: &mut [f64],
    lda: i32,
    d: &mut [f64],
    e: &mut [f64],
    tau: &mut [f64],
    work: &mut [f64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::dsytrd_(
        &(uplo as c_char),
        &n,
        a.as_mut_ptr(),
        &lda,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        tau.as_mut_ptr(),
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn ssytrd(
    uplo: u8,
    n: i32,
    a: &mut [f32],
    lda: i32,
    d: &mut [f32],
    e: &mut [f32],
    tau: &mut [f32],
    work: &mut [f32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::ssytrd_(
        &(uplo as c_char),
        &n,
        a.as_mut_ptr(),
        &lda,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        tau.as_mut_ptr(),
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn dsytrd_2stage(
    vect: u8,
    uplo: u8,
    n: i32,
    a: &mut [f64],
    lda: i32,
    d: &mut [f64],
    e: &mut [f64],
    tau: &mut [f64],
    hous2: &mut [f64],
    lhous2: &[i32],
    work: &mut [f64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::dsytrd_2stage_(
        &(vect as c_char),
        &(uplo as c_char),
        &n,
        a.as_mut_ptr(),
        &lda,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        tau.as_mut_ptr(),
        hous2.as_mut_ptr(),
        lhous2.as_ptr(),
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn ssytrd_2stage(
    vect: u8,
    uplo: u8,
    n: i32,
    a: &mut [f32],
    lda: i32,
    d: &mut [f32],
    e: &mut [f32],
    tau: &mut [f32],
    hous2: &mut [f32],
    lhous2: &[i32],
    work: &mut [f32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::ssytrd_2stage_(
        &(vect as c_char),
        &(uplo as c_char),
        &n,
        a.as_mut_ptr(),
        &lda,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        tau.as_mut_ptr(),
        hous2.as_mut_ptr(),
        lhous2.as_ptr(),
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn csytrf(
    uplo: u8,
    n: i32,
    a: &mut [c32],
    lda: i32,
    ipiv: &mut [i32],
    work: &mut [c32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::csytrf_(
        &(uplo as c_char),
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        ipiv.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn dsytrf(
    uplo: u8,
    n: i32,
    a: &mut [f64],
    lda: i32,
    ipiv: &mut [i32],
    work: &mut [f64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::dsytrf_(
        &(uplo as c_char),
        &n,
        a.as_mut_ptr(),
        &lda,
        ipiv.as_mut_ptr(),
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn ssytrf(
    uplo: u8,
    n: i32,
    a: &mut [f32],
    lda: i32,
    ipiv: &mut [i32],
    work: &mut [f32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::ssytrf_(
        &(uplo as c_char),
        &n,
        a.as_mut_ptr(),
        &lda,
        ipiv.as_mut_ptr(),
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn zsytrf(
    uplo: u8,
    n: i32,
    a: &mut [c64],
    lda: i32,
    ipiv: &mut [i32],
    work: &mut [c64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::zsytrf_(
        &(uplo as c_char),
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        ipiv.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn csytrf_aa(
    uplo: u8,
    n: i32,
    a: &mut [c32],
    lda: i32,
    ipiv: &mut [i32],
    work: &mut [c32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::csytrf_aa_(
        &(uplo as c_char),
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        ipiv.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn dsytrf_aa(
    uplo: u8,
    n: i32,
    a: &mut [f64],
    lda: i32,
    ipiv: &mut [i32],
    work: &mut [f64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::dsytrf_aa_(
        &(uplo as c_char),
        &n,
        a.as_mut_ptr(),
        &lda,
        ipiv.as_mut_ptr(),
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn ssytrf_aa(
    uplo: u8,
    n: i32,
    a: &mut [f32],
    lda: i32,
    ipiv: &mut [i32],
    work: &mut [f32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::ssytrf_aa_(
        &(uplo as c_char),
        &n,
        a.as_mut_ptr(),
        &lda,
        ipiv.as_mut_ptr(),
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn zsytrf_aa(
    uplo: u8,
    n: i32,
    a: &mut [c64],
    lda: i32,
    ipiv: &mut [i32],
    work: &mut [c64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::zsytrf_aa_(
        &(uplo as c_char),
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        ipiv.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn csytrf_aa_2stage(
    uplo: u8,
    n: i32,
    a: &mut [c32],
    lda: i32,
    tb: &mut [c32],
    ltb: &[i32],
    ipiv: &mut [i32],
    ipiv2: &mut [i32],
    work: &mut [c32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::csytrf_aa_2stage_(
        &(uplo as c_char),
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        tb.as_mut_ptr() as *mut _,
        ltb.as_ptr(),
        ipiv.as_mut_ptr(),
        ipiv2.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn dsytrf_aa_2stage(
    uplo: u8,
    n: i32,
    a: &mut [f64],
    lda: i32,
    tb: &mut [f64],
    ltb: &[i32],
    ipiv: &mut [i32],
    ipiv2: &mut [i32],
    work: &mut [f64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::dsytrf_aa_2stage_(
        &(uplo as c_char),
        &n,
        a.as_mut_ptr(),
        &lda,
        tb.as_mut_ptr(),
        ltb.as_ptr(),
        ipiv.as_mut_ptr(),
        ipiv2.as_mut_ptr(),
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn ssytrf_aa_2stage(
    uplo: u8,
    n: i32,
    a: &mut [f32],
    lda: i32,
    tb: &mut [f32],
    ltb: &[i32],
    ipiv: &mut [i32],
    ipiv2: &mut [i32],
    work: &mut [f32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::ssytrf_aa_2stage_(
        &(uplo as c_char),
        &n,
        a.as_mut_ptr(),
        &lda,
        tb.as_mut_ptr(),
        ltb.as_ptr(),
        ipiv.as_mut_ptr(),
        ipiv2.as_mut_ptr(),
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn zsytrf_aa_2stage(
    uplo: u8,
    n: i32,
    a: &mut [c64],
    lda: i32,
    tb: &mut [c64],
    ltb: &[i32],
    ipiv: &mut [i32],
    ipiv2: &mut [i32],
    work: &mut [c64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::zsytrf_aa_2stage_(
        &(uplo as c_char),
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        tb.as_mut_ptr() as *mut _,
        ltb.as_ptr(),
        ipiv.as_mut_ptr(),
        ipiv2.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn csytrf_rk(
    uplo: u8,
    n: i32,
    a: &mut [c32],
    lda: i32,
    e: &mut [c32],
    ipiv: &mut [i32],
    work: &mut [c32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::csytrf_rk_(
        &(uplo as c_char),
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        e.as_mut_ptr() as *mut _,
        ipiv.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn dsytrf_rk(
    uplo: u8,
    n: i32,
    a: &mut [f64],
    lda: i32,
    e: &mut [f64],
    ipiv: &mut [i32],
    work: &mut [f64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::dsytrf_rk_(
        &(uplo as c_char),
        &n,
        a.as_mut_ptr(),
        &lda,
        e.as_mut_ptr(),
        ipiv.as_mut_ptr(),
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn ssytrf_rk(
    uplo: u8,
    n: i32,
    a: &mut [f32],
    lda: i32,
    e: &mut [f32],
    ipiv: &mut [i32],
    work: &mut [f32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::ssytrf_rk_(
        &(uplo as c_char),
        &n,
        a.as_mut_ptr(),
        &lda,
        e.as_mut_ptr(),
        ipiv.as_mut_ptr(),
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn zsytrf_rk(
    uplo: u8,
    n: i32,
    a: &mut [c64],
    lda: i32,
    e: &mut [c64],
    ipiv: &mut [i32],
    work: &mut [c64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::zsytrf_rk_(
        &(uplo as c_char),
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        e.as_mut_ptr() as *mut _,
        ipiv.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn csytrf_rook(
    uplo: u8,
    n: i32,
    a: &mut [c32],
    lda: i32,
    ipiv: &mut [i32],
    work: &mut [c32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::csytrf_rook_(
        &(uplo as c_char),
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        ipiv.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn dsytrf_rook(
    uplo: u8,
    n: i32,
    a: &mut [f64],
    lda: i32,
    ipiv: &mut [i32],
    work: &mut [f64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::dsytrf_rook_(
        &(uplo as c_char),
        &n,
        a.as_mut_ptr(),
        &lda,
        ipiv.as_mut_ptr(),
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn ssytrf_rook(
    uplo: u8,
    n: i32,
    a: &mut [f32],
    lda: i32,
    ipiv: &mut [i32],
    work: &mut [f32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::ssytrf_rook_(
        &(uplo as c_char),
        &n,
        a.as_mut_ptr(),
        &lda,
        ipiv.as_mut_ptr(),
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn zsytrf_rook(
    uplo: u8,
    n: i32,
    a: &mut [c64],
    lda: i32,
    ipiv: &mut [i32],
    work: &mut [c64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::zsytrf_rook_(
        &(uplo as c_char),
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        ipiv.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn csytri(
    uplo: u8,
    n: i32,
    a: &mut [c32],
    lda: i32,
    ipiv: &[i32],
    work: &mut [c32],
    info: &mut i32,
) {
    ffi::csytri_(
        &(uplo as c_char),
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        ipiv.as_ptr(),
        work.as_mut_ptr() as *mut _,
        info,
    )
}

#[inline]
pub unsafe fn dsytri(
    uplo: u8,
    n: i32,
    a: &mut [f64],
    lda: i32,
    ipiv: &[i32],
    work: &mut [f64],
    info: &mut i32,
) {
    ffi::dsytri_(
        &(uplo as c_char),
        &n,
        a.as_mut_ptr(),
        &lda,
        ipiv.as_ptr(),
        work.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn ssytri(
    uplo: u8,
    n: i32,
    a: &mut [f32],
    lda: i32,
    ipiv: &[i32],
    work: &mut [f32],
    info: &mut i32,
) {
    ffi::ssytri_(
        &(uplo as c_char),
        &n,
        a.as_mut_ptr(),
        &lda,
        ipiv.as_ptr(),
        work.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn zsytri(
    uplo: u8,
    n: i32,
    a: &mut [c64],
    lda: i32,
    ipiv: &[i32],
    work: &mut [c64],
    info: &mut i32,
) {
    ffi::zsytri_(
        &(uplo as c_char),
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        ipiv.as_ptr(),
        work.as_mut_ptr() as *mut _,
        info,
    )
}

#[inline]
pub unsafe fn csytri2(
    uplo: u8,
    n: i32,
    a: &mut [c32],
    lda: i32,
    ipiv: &[i32],
    work: &mut [c32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::csytri2_(
        &(uplo as c_char),
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        ipiv.as_ptr(),
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn dsytri2(
    uplo: u8,
    n: i32,
    a: &mut [f64],
    lda: i32,
    ipiv: &[i32],
    work: &mut [f64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::dsytri2_(
        &(uplo as c_char),
        &n,
        a.as_mut_ptr(),
        &lda,
        ipiv.as_ptr(),
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn ssytri2(
    uplo: u8,
    n: i32,
    a: &mut [f32],
    lda: i32,
    ipiv: &[i32],
    work: &mut [f32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::ssytri2_(
        &(uplo as c_char),
        &n,
        a.as_mut_ptr(),
        &lda,
        ipiv.as_ptr(),
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn zsytri2(
    uplo: u8,
    n: i32,
    a: &mut [c64],
    lda: i32,
    ipiv: &[i32],
    work: &mut [c64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::zsytri2_(
        &(uplo as c_char),
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        ipiv.as_ptr(),
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn csytri2x(
    uplo: u8,
    n: i32,
    a: &mut [c32],
    lda: i32,
    ipiv: &[i32],
    work: &mut [c32],
    nb: i32,
    info: &mut i32,
) {
    ffi::csytri2x_(
        &(uplo as c_char),
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        ipiv.as_ptr(),
        work.as_mut_ptr() as *mut _,
        &nb,
        info,
    )
}

#[inline]
pub unsafe fn dsytri2x(
    uplo: u8,
    n: i32,
    a: &mut [f64],
    lda: i32,
    ipiv: &[i32],
    work: &mut [f64],
    nb: i32,
    info: &mut i32,
) {
    ffi::dsytri2x_(
        &(uplo as c_char),
        &n,
        a.as_mut_ptr(),
        &lda,
        ipiv.as_ptr(),
        work.as_mut_ptr(),
        &nb,
        info,
    )
}

#[inline]
pub unsafe fn ssytri2x(
    uplo: u8,
    n: i32,
    a: &mut [f32],
    lda: i32,
    ipiv: &[i32],
    work: &mut [f32],
    nb: i32,
    info: &mut i32,
) {
    ffi::ssytri2x_(
        &(uplo as c_char),
        &n,
        a.as_mut_ptr(),
        &lda,
        ipiv.as_ptr(),
        work.as_mut_ptr(),
        &nb,
        info,
    )
}

#[inline]
pub unsafe fn zsytri2x(
    uplo: u8,
    n: i32,
    a: &mut [c64],
    lda: i32,
    ipiv: &[i32],
    work: &mut [c64],
    nb: i32,
    info: &mut i32,
) {
    ffi::zsytri2x_(
        &(uplo as c_char),
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        ipiv.as_ptr(),
        work.as_mut_ptr() as *mut _,
        &nb,
        info,
    )
}

#[inline]
pub unsafe fn csytri_3(
    uplo: u8,
    n: i32,
    a: &mut [c32],
    lda: i32,
    e: &[c32],
    ipiv: &[i32],
    work: &mut [c32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::csytri_3_(
        &(uplo as c_char),
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        e.as_ptr() as *const _,
        ipiv.as_ptr(),
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn dsytri_3(
    uplo: u8,
    n: i32,
    a: &mut [f64],
    lda: i32,
    e: &[f64],
    ipiv: &[i32],
    work: &mut [f64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::dsytri_3_(
        &(uplo as c_char),
        &n,
        a.as_mut_ptr(),
        &lda,
        e.as_ptr(),
        ipiv.as_ptr(),
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn ssytri_3(
    uplo: u8,
    n: i32,
    a: &mut [f32],
    lda: i32,
    e: &[f32],
    ipiv: &[i32],
    work: &mut [f32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::ssytri_3_(
        &(uplo as c_char),
        &n,
        a.as_mut_ptr(),
        &lda,
        e.as_ptr(),
        ipiv.as_ptr(),
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn zsytri_3(
    uplo: u8,
    n: i32,
    a: &mut [c64],
    lda: i32,
    e: &[c64],
    ipiv: &[i32],
    work: &mut [c64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::zsytri_3_(
        &(uplo as c_char),
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        e.as_ptr() as *const _,
        ipiv.as_ptr(),
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn csytrs(
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &[c32],
    lda: i32,
    ipiv: &[i32],
    b: &mut [c32],
    ldb: i32,
    info: &mut i32,
) {
    ffi::csytrs_(
        &(uplo as c_char),
        &n,
        &nrhs,
        a.as_ptr() as *const _,
        &lda,
        ipiv.as_ptr(),
        b.as_mut_ptr() as *mut _,
        &ldb,
        info,
    )
}

#[inline]
pub unsafe fn dsytrs(
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &[f64],
    lda: i32,
    ipiv: &[i32],
    b: &mut [f64],
    ldb: i32,
    info: &mut i32,
) {
    ffi::dsytrs_(
        &(uplo as c_char),
        &n,
        &nrhs,
        a.as_ptr(),
        &lda,
        ipiv.as_ptr(),
        b.as_mut_ptr(),
        &ldb,
        info,
    )
}

#[inline]
pub unsafe fn ssytrs(
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &[f32],
    lda: i32,
    ipiv: &[i32],
    b: &mut [f32],
    ldb: i32,
    info: &mut i32,
) {
    ffi::ssytrs_(
        &(uplo as c_char),
        &n,
        &nrhs,
        a.as_ptr(),
        &lda,
        ipiv.as_ptr(),
        b.as_mut_ptr(),
        &ldb,
        info,
    )
}

#[inline]
pub unsafe fn zsytrs(
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &[c64],
    lda: i32,
    ipiv: &[i32],
    b: &mut [c64],
    ldb: i32,
    info: &mut i32,
) {
    ffi::zsytrs_(
        &(uplo as c_char),
        &n,
        &nrhs,
        a.as_ptr() as *const _,
        &lda,
        ipiv.as_ptr(),
        b.as_mut_ptr() as *mut _,
        &ldb,
        info,
    )
}

#[inline]
pub unsafe fn csytrs2(
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &mut [c32],
    lda: i32,
    ipiv: &[i32],
    b: &mut [c32],
    ldb: i32,
    work: &mut [c32],
    info: &mut i32,
) {
    ffi::csytrs2_(
        &(uplo as c_char),
        &n,
        &nrhs,
        a.as_mut_ptr() as *mut _,
        &lda,
        ipiv.as_ptr(),
        b.as_mut_ptr() as *mut _,
        &ldb,
        work.as_mut_ptr() as *mut _,
        info,
    )
}

#[inline]
pub unsafe fn dsytrs2(
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &mut [f64],
    lda: i32,
    ipiv: &[i32],
    b: &mut [f64],
    ldb: i32,
    work: &mut [f64],
    info: &mut i32,
) {
    ffi::dsytrs2_(
        &(uplo as c_char),
        &n,
        &nrhs,
        a.as_mut_ptr(),
        &lda,
        ipiv.as_ptr(),
        b.as_mut_ptr(),
        &ldb,
        work.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn ssytrs2(
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &mut [f32],
    lda: i32,
    ipiv: &[i32],
    b: &mut [f32],
    ldb: i32,
    work: &mut [f32],
    info: &mut i32,
) {
    ffi::ssytrs2_(
        &(uplo as c_char),
        &n,
        &nrhs,
        a.as_mut_ptr(),
        &lda,
        ipiv.as_ptr(),
        b.as_mut_ptr(),
        &ldb,
        work.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn zsytrs2(
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &mut [c64],
    lda: i32,
    ipiv: &[i32],
    b: &mut [c64],
    ldb: i32,
    work: &mut [c64],
    info: &mut i32,
) {
    ffi::zsytrs2_(
        &(uplo as c_char),
        &n,
        &nrhs,
        a.as_mut_ptr() as *mut _,
        &lda,
        ipiv.as_ptr(),
        b.as_mut_ptr() as *mut _,
        &ldb,
        work.as_mut_ptr() as *mut _,
        info,
    )
}

#[inline]
pub unsafe fn csytrs_3(
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &[c32],
    lda: i32,
    e: &[c32],
    ipiv: &[i32],
    b: &mut [c32],
    ldb: i32,
    info: &mut i32,
) {
    ffi::csytrs_3_(
        &(uplo as c_char),
        &n,
        &nrhs,
        a.as_ptr() as *const _,
        &lda,
        e.as_ptr() as *const _,
        ipiv.as_ptr(),
        b.as_mut_ptr() as *mut _,
        &ldb,
        info,
    )
}

#[inline]
pub unsafe fn dsytrs_3(
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &[f64],
    lda: i32,
    e: &[f64],
    ipiv: &[i32],
    b: &mut [f64],
    ldb: i32,
    info: &mut i32,
) {
    ffi::dsytrs_3_(
        &(uplo as c_char),
        &n,
        &nrhs,
        a.as_ptr(),
        &lda,
        e.as_ptr(),
        ipiv.as_ptr(),
        b.as_mut_ptr(),
        &ldb,
        info,
    )
}

#[inline]
pub unsafe fn ssytrs_3(
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &[f32],
    lda: i32,
    e: &[f32],
    ipiv: &[i32],
    b: &mut [f32],
    ldb: i32,
    info: &mut i32,
) {
    ffi::ssytrs_3_(
        &(uplo as c_char),
        &n,
        &nrhs,
        a.as_ptr(),
        &lda,
        e.as_ptr(),
        ipiv.as_ptr(),
        b.as_mut_ptr(),
        &ldb,
        info,
    )
}

#[inline]
pub unsafe fn zsytrs_3(
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &[c64],
    lda: i32,
    e: &[c64],
    ipiv: &[i32],
    b: &mut [c64],
    ldb: i32,
    info: &mut i32,
) {
    ffi::zsytrs_3_(
        &(uplo as c_char),
        &n,
        &nrhs,
        a.as_ptr() as *const _,
        &lda,
        e.as_ptr() as *const _,
        ipiv.as_ptr(),
        b.as_mut_ptr() as *mut _,
        &ldb,
        info,
    )
}

#[inline]
pub unsafe fn csytrs_aa(
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &[c32],
    lda: i32,
    ipiv: &[i32],
    b: &mut [c32],
    ldb: i32,
    work: &mut [c32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::csytrs_aa_(
        &(uplo as c_char),
        &n,
        &nrhs,
        a.as_ptr() as *const _,
        &lda,
        ipiv.as_ptr(),
        b.as_mut_ptr() as *mut _,
        &ldb,
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn dsytrs_aa(
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &[f64],
    lda: i32,
    ipiv: &[i32],
    b: &mut [f64],
    ldb: i32,
    work: &mut [f64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::dsytrs_aa_(
        &(uplo as c_char),
        &n,
        &nrhs,
        a.as_ptr(),
        &lda,
        ipiv.as_ptr(),
        b.as_mut_ptr(),
        &ldb,
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn ssytrs_aa(
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &[f32],
    lda: i32,
    ipiv: &[i32],
    b: &mut [f32],
    ldb: i32,
    work: &mut [f32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::ssytrs_aa_(
        &(uplo as c_char),
        &n,
        &nrhs,
        a.as_ptr(),
        &lda,
        ipiv.as_ptr(),
        b.as_mut_ptr(),
        &ldb,
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn zsytrs_aa(
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &[c64],
    lda: i32,
    ipiv: &[i32],
    b: &mut [c64],
    ldb: i32,
    work: &mut [c64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::zsytrs_aa_(
        &(uplo as c_char),
        &n,
        &nrhs,
        a.as_ptr() as *const _,
        &lda,
        ipiv.as_ptr(),
        b.as_mut_ptr() as *mut _,
        &ldb,
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn csytrs_aa_2stage(
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &[c32],
    lda: i32,
    tb: &mut [c32],
    ltb: &[i32],
    ipiv: &[i32],
    ipiv2: &[i32],
    b: &mut [c32],
    ldb: i32,
    info: &mut i32,
) {
    ffi::csytrs_aa_2stage_(
        &(uplo as c_char),
        &n,
        &nrhs,
        a.as_ptr() as *const _,
        &lda,
        tb.as_mut_ptr() as *mut _,
        ltb.as_ptr(),
        ipiv.as_ptr(),
        ipiv2.as_ptr(),
        b.as_mut_ptr() as *mut _,
        &ldb,
        info,
    )
}

#[inline]
pub unsafe fn dsytrs_aa_2stage(
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &[f64],
    lda: i32,
    tb: &mut [f64],
    ltb: &[i32],
    ipiv: &[i32],
    ipiv2: &[i32],
    b: &mut [f64],
    ldb: i32,
    info: &mut i32,
) {
    ffi::dsytrs_aa_2stage_(
        &(uplo as c_char),
        &n,
        &nrhs,
        a.as_ptr(),
        &lda,
        tb.as_mut_ptr(),
        ltb.as_ptr(),
        ipiv.as_ptr(),
        ipiv2.as_ptr(),
        b.as_mut_ptr(),
        &ldb,
        info,
    )
}

#[inline]
pub unsafe fn ssytrs_aa_2stage(
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &[f32],
    lda: i32,
    tb: &mut [f32],
    ltb: &[i32],
    ipiv: &[i32],
    ipiv2: &[i32],
    b: &mut [f32],
    ldb: i32,
    info: &mut i32,
) {
    ffi::ssytrs_aa_2stage_(
        &(uplo as c_char),
        &n,
        &nrhs,
        a.as_ptr(),
        &lda,
        tb.as_mut_ptr(),
        ltb.as_ptr(),
        ipiv.as_ptr(),
        ipiv2.as_ptr(),
        b.as_mut_ptr(),
        &ldb,
        info,
    )
}

#[inline]
pub unsafe fn zsytrs_aa_2stage(
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &[c64],
    lda: i32,
    tb: &mut [c64],
    ltb: &[i32],
    ipiv: &[i32],
    ipiv2: &[i32],
    b: &mut [c64],
    ldb: i32,
    info: &mut i32,
) {
    ffi::zsytrs_aa_2stage_(
        &(uplo as c_char),
        &n,
        &nrhs,
        a.as_ptr() as *const _,
        &lda,
        tb.as_mut_ptr() as *mut _,
        ltb.as_ptr(),
        ipiv.as_ptr(),
        ipiv2.as_ptr(),
        b.as_mut_ptr() as *mut _,
        &ldb,
        info,
    )
}

#[inline]
pub unsafe fn csytrs_rook(
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &[c32],
    lda: i32,
    ipiv: &[i32],
    b: &mut [c32],
    ldb: i32,
    info: &mut i32,
) {
    ffi::csytrs_rook_(
        &(uplo as c_char),
        &n,
        &nrhs,
        a.as_ptr() as *const _,
        &lda,
        ipiv.as_ptr(),
        b.as_mut_ptr() as *mut _,
        &ldb,
        info,
    )
}

#[inline]
pub unsafe fn dsytrs_rook(
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &[f64],
    lda: i32,
    ipiv: &[i32],
    b: &mut [f64],
    ldb: i32,
    info: &mut i32,
) {
    ffi::dsytrs_rook_(
        &(uplo as c_char),
        &n,
        &nrhs,
        a.as_ptr(),
        &lda,
        ipiv.as_ptr(),
        b.as_mut_ptr(),
        &ldb,
        info,
    )
}

#[inline]
pub unsafe fn ssytrs_rook(
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &[f32],
    lda: i32,
    ipiv: &[i32],
    b: &mut [f32],
    ldb: i32,
    info: &mut i32,
) {
    ffi::ssytrs_rook_(
        &(uplo as c_char),
        &n,
        &nrhs,
        a.as_ptr(),
        &lda,
        ipiv.as_ptr(),
        b.as_mut_ptr(),
        &ldb,
        info,
    )
}

#[inline]
pub unsafe fn zsytrs_rook(
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &[c64],
    lda: i32,
    ipiv: &[i32],
    b: &mut [c64],
    ldb: i32,
    info: &mut i32,
) {
    ffi::zsytrs_rook_(
        &(uplo as c_char),
        &n,
        &nrhs,
        a.as_ptr() as *const _,
        &lda,
        ipiv.as_ptr(),
        b.as_mut_ptr() as *mut _,
        &ldb,
        info,
    )
}

#[inline]
pub unsafe fn ctbcon(
    norm: u8,
    uplo: u8,
    diag: u8,
    n: i32,
    kd: i32,
    ab: &[c32],
    ldab: i32,
    rcond: &mut f32,
    work: &mut [c32],
    rwork: &mut [f32],
    info: &mut i32,
) {
    ffi::ctbcon_(
        &(norm as c_char),
        &(uplo as c_char),
        &(diag as c_char),
        &n,
        &kd,
        ab.as_ptr() as *const _,
        &ldab,
        rcond,
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn dtbcon(
    norm: u8,
    uplo: u8,
    diag: u8,
    n: i32,
    kd: i32,
    ab: &[f64],
    ldab: i32,
    rcond: &mut f64,
    work: &mut [f64],
    iwork: &mut [i32],
    info: &mut i32,
) {
    ffi::dtbcon_(
        &(norm as c_char),
        &(uplo as c_char),
        &(diag as c_char),
        &n,
        &kd,
        ab.as_ptr(),
        &ldab,
        rcond,
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn stbcon(
    norm: u8,
    uplo: u8,
    diag: u8,
    n: i32,
    kd: i32,
    ab: &[f32],
    ldab: i32,
    rcond: &mut f32,
    work: &mut [f32],
    iwork: &mut [i32],
    info: &mut i32,
) {
    ffi::stbcon_(
        &(norm as c_char),
        &(uplo as c_char),
        &(diag as c_char),
        &n,
        &kd,
        ab.as_ptr(),
        &ldab,
        rcond,
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn ztbcon(
    norm: u8,
    uplo: u8,
    diag: u8,
    n: i32,
    kd: i32,
    ab: &[c64],
    ldab: i32,
    rcond: &mut f64,
    work: &mut [c64],
    rwork: &mut [f64],
    info: &mut i32,
) {
    ffi::ztbcon_(
        &(norm as c_char),
        &(uplo as c_char),
        &(diag as c_char),
        &n,
        &kd,
        ab.as_ptr() as *const _,
        &ldab,
        rcond,
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn ctbrfs(
    uplo: u8,
    trans: u8,
    diag: u8,
    n: i32,
    kd: i32,
    nrhs: i32,
    ab: &[c32],
    ldab: i32,
    b: &[c32],
    ldb: i32,
    x: &[c32],
    ldx: i32,
    ferr: &mut [f32],
    berr: &mut [f32],
    work: &mut [c32],
    rwork: &mut [f32],
    info: &mut i32,
) {
    ffi::ctbrfs_(
        &(uplo as c_char),
        &(trans as c_char),
        &(diag as c_char),
        &n,
        &kd,
        &nrhs,
        ab.as_ptr() as *const _,
        &ldab,
        b.as_ptr() as *const _,
        &ldb,
        x.as_ptr() as *const _,
        &ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn dtbrfs(
    uplo: u8,
    trans: u8,
    diag: u8,
    n: i32,
    kd: i32,
    nrhs: i32,
    ab: &[f64],
    ldab: i32,
    b: &[f64],
    ldb: i32,
    x: &[f64],
    ldx: i32,
    ferr: &mut [f64],
    berr: &mut [f64],
    work: &mut [f64],
    iwork: &mut [i32],
    info: &mut i32,
) {
    ffi::dtbrfs_(
        &(uplo as c_char),
        &(trans as c_char),
        &(diag as c_char),
        &n,
        &kd,
        &nrhs,
        ab.as_ptr(),
        &ldab,
        b.as_ptr(),
        &ldb,
        x.as_ptr(),
        &ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn stbrfs(
    uplo: u8,
    trans: u8,
    diag: u8,
    n: i32,
    kd: i32,
    nrhs: i32,
    ab: &[f32],
    ldab: i32,
    b: &[f32],
    ldb: i32,
    x: &[f32],
    ldx: i32,
    ferr: &mut [f32],
    berr: &mut [f32],
    work: &mut [f32],
    iwork: &mut [i32],
    info: &mut i32,
) {
    ffi::stbrfs_(
        &(uplo as c_char),
        &(trans as c_char),
        &(diag as c_char),
        &n,
        &kd,
        &nrhs,
        ab.as_ptr(),
        &ldab,
        b.as_ptr(),
        &ldb,
        x.as_ptr(),
        &ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn ztbrfs(
    uplo: u8,
    trans: u8,
    diag: u8,
    n: i32,
    kd: i32,
    nrhs: i32,
    ab: &[c64],
    ldab: i32,
    b: &[c64],
    ldb: i32,
    x: &[c64],
    ldx: i32,
    ferr: &mut [f64],
    berr: &mut [f64],
    work: &mut [c64],
    rwork: &mut [f64],
    info: &mut i32,
) {
    ffi::ztbrfs_(
        &(uplo as c_char),
        &(trans as c_char),
        &(diag as c_char),
        &n,
        &kd,
        &nrhs,
        ab.as_ptr() as *const _,
        &ldab,
        b.as_ptr() as *const _,
        &ldb,
        x.as_ptr() as *const _,
        &ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn ctbtrs(
    uplo: u8,
    trans: u8,
    diag: u8,
    n: i32,
    kd: i32,
    nrhs: i32,
    ab: &[c32],
    ldab: i32,
    b: &mut [c32],
    ldb: i32,
    info: &mut i32,
) {
    ffi::ctbtrs_(
        &(uplo as c_char),
        &(trans as c_char),
        &(diag as c_char),
        &n,
        &kd,
        &nrhs,
        ab.as_ptr() as *const _,
        &ldab,
        b.as_mut_ptr() as *mut _,
        &ldb,
        info,
    )
}

#[inline]
pub unsafe fn dtbtrs(
    uplo: u8,
    trans: u8,
    diag: u8,
    n: i32,
    kd: i32,
    nrhs: i32,
    ab: &[f64],
    ldab: i32,
    b: &mut [f64],
    ldb: i32,
    info: &mut i32,
) {
    ffi::dtbtrs_(
        &(uplo as c_char),
        &(trans as c_char),
        &(diag as c_char),
        &n,
        &kd,
        &nrhs,
        ab.as_ptr(),
        &ldab,
        b.as_mut_ptr(),
        &ldb,
        info,
    )
}

#[inline]
pub unsafe fn stbtrs(
    uplo: u8,
    trans: u8,
    diag: u8,
    n: i32,
    kd: i32,
    nrhs: i32,
    ab: &[f32],
    ldab: i32,
    b: &mut [f32],
    ldb: i32,
    info: &mut i32,
) {
    ffi::stbtrs_(
        &(uplo as c_char),
        &(trans as c_char),
        &(diag as c_char),
        &n,
        &kd,
        &nrhs,
        ab.as_ptr(),
        &ldab,
        b.as_mut_ptr(),
        &ldb,
        info,
    )
}

#[inline]
pub unsafe fn ztbtrs(
    uplo: u8,
    trans: u8,
    diag: u8,
    n: i32,
    kd: i32,
    nrhs: i32,
    ab: &[c64],
    ldab: i32,
    b: &mut [c64],
    ldb: i32,
    info: &mut i32,
) {
    ffi::ztbtrs_(
        &(uplo as c_char),
        &(trans as c_char),
        &(diag as c_char),
        &n,
        &kd,
        &nrhs,
        ab.as_ptr() as *const _,
        &ldab,
        b.as_mut_ptr() as *mut _,
        &ldb,
        info,
    )
}

#[inline]
pub unsafe fn ctfsm(
    transr: u8,
    side: u8,
    uplo: u8,
    trans: u8,
    diag: u8,
    m: i32,
    n: i32,
    alpha: &[c32],
    a: &[c32],
    b: &mut [c32],
    ldb: i32,
) {
    ffi::ctfsm_(
        &(transr as c_char),
        &(side as c_char),
        &(uplo as c_char),
        &(trans as c_char),
        &(diag as c_char),
        &m,
        &n,
        alpha.as_ptr() as *const _,
        a.as_ptr() as *const _,
        b.as_mut_ptr() as *mut _,
        &ldb,
    )
}

#[inline]
pub unsafe fn dtfsm(
    transr: u8,
    side: u8,
    uplo: u8,
    trans: u8,
    diag: u8,
    m: i32,
    n: i32,
    alpha: &[f64],
    a: &[f64],
    b: &mut [f64],
    ldb: i32,
) {
    ffi::dtfsm_(
        &(transr as c_char),
        &(side as c_char),
        &(uplo as c_char),
        &(trans as c_char),
        &(diag as c_char),
        &m,
        &n,
        alpha.as_ptr(),
        a.as_ptr(),
        b.as_mut_ptr(),
        &ldb,
    )
}

#[inline]
pub unsafe fn stfsm(
    transr: u8,
    side: u8,
    uplo: u8,
    trans: u8,
    diag: u8,
    m: i32,
    n: i32,
    alpha: &[f32],
    a: &[f32],
    b: &mut [f32],
    ldb: i32,
) {
    ffi::stfsm_(
        &(transr as c_char),
        &(side as c_char),
        &(uplo as c_char),
        &(trans as c_char),
        &(diag as c_char),
        &m,
        &n,
        alpha.as_ptr(),
        a.as_ptr(),
        b.as_mut_ptr(),
        &ldb,
    )
}

#[inline]
pub unsafe fn ztfsm(
    transr: u8,
    side: u8,
    uplo: u8,
    trans: u8,
    diag: u8,
    m: i32,
    n: i32,
    alpha: &[c64],
    a: &[c64],
    b: &mut [c64],
    ldb: i32,
) {
    ffi::ztfsm_(
        &(transr as c_char),
        &(side as c_char),
        &(uplo as c_char),
        &(trans as c_char),
        &(diag as c_char),
        &m,
        &n,
        alpha.as_ptr() as *const _,
        a.as_ptr() as *const _,
        b.as_mut_ptr() as *mut _,
        &ldb,
    )
}

#[inline]
pub unsafe fn ctftri(transr: u8, uplo: u8, diag: u8, n: i32, a: &mut [c32], info: &mut i32) {
    ffi::ctftri_(
        &(transr as c_char),
        &(uplo as c_char),
        &(diag as c_char),
        &n,
        a.as_mut_ptr() as *mut _,
        info,
    )
}

#[inline]
pub unsafe fn dtftri(transr: u8, uplo: u8, diag: u8, n: i32, a: &mut [f64], info: &mut i32) {
    ffi::dtftri_(
        &(transr as c_char),
        &(uplo as c_char),
        &(diag as c_char),
        &n,
        a.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn stftri(transr: u8, uplo: u8, diag: u8, n: i32, a: &mut [f32], info: &mut i32) {
    ffi::stftri_(
        &(transr as c_char),
        &(uplo as c_char),
        &(diag as c_char),
        &n,
        a.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn ztftri(transr: u8, uplo: u8, diag: u8, n: i32, a: &mut [c64], info: &mut i32) {
    ffi::ztftri_(
        &(transr as c_char),
        &(uplo as c_char),
        &(diag as c_char),
        &n,
        a.as_mut_ptr() as *mut _,
        info,
    )
}

#[inline]
pub unsafe fn ctfttp(transr: u8, uplo: u8, n: i32, arf: &[c32], ap: &mut [c32], info: &mut i32) {
    ffi::ctfttp_(
        &(transr as c_char),
        &(uplo as c_char),
        &n,
        arf.as_ptr() as *const _,
        ap.as_mut_ptr() as *mut _,
        info,
    )
}

#[inline]
pub unsafe fn dtfttp(transr: u8, uplo: u8, n: i32, arf: &[f64], ap: &mut [f64], info: &mut i32) {
    ffi::dtfttp_(
        &(transr as c_char),
        &(uplo as c_char),
        &n,
        arf.as_ptr(),
        ap.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn stfttp(transr: u8, uplo: u8, n: i32, arf: &[f32], ap: &mut [f32], info: &mut i32) {
    ffi::stfttp_(
        &(transr as c_char),
        &(uplo as c_char),
        &n,
        arf.as_ptr(),
        ap.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn ztfttp(transr: u8, uplo: u8, n: i32, arf: &[c64], ap: &mut [c64], info: &mut i32) {
    ffi::ztfttp_(
        &(transr as c_char),
        &(uplo as c_char),
        &n,
        arf.as_ptr() as *const _,
        ap.as_mut_ptr() as *mut _,
        info,
    )
}

#[inline]
pub unsafe fn ctfttr(
    transr: u8,
    uplo: u8,
    n: i32,
    arf: &[c32],
    a: &mut [c32],
    lda: i32,
    info: &mut i32,
) {
    ffi::ctfttr_(
        &(transr as c_char),
        &(uplo as c_char),
        &n,
        arf.as_ptr() as *const _,
        a.as_mut_ptr() as *mut _,
        &lda,
        info,
    )
}

#[inline]
pub unsafe fn dtfttr(
    transr: u8,
    uplo: u8,
    n: i32,
    arf: &[f64],
    a: &mut [f64],
    lda: i32,
    info: &mut i32,
) {
    ffi::dtfttr_(
        &(transr as c_char),
        &(uplo as c_char),
        &n,
        arf.as_ptr(),
        a.as_mut_ptr(),
        &lda,
        info,
    )
}

#[inline]
pub unsafe fn stfttr(
    transr: u8,
    uplo: u8,
    n: i32,
    arf: &[f32],
    a: &mut [f32],
    lda: i32,
    info: &mut i32,
) {
    ffi::stfttr_(
        &(transr as c_char),
        &(uplo as c_char),
        &n,
        arf.as_ptr(),
        a.as_mut_ptr(),
        &lda,
        info,
    )
}

#[inline]
pub unsafe fn ztfttr(
    transr: u8,
    uplo: u8,
    n: i32,
    arf: &[c64],
    a: &mut [c64],
    lda: i32,
    info: &mut i32,
) {
    ffi::ztfttr_(
        &(transr as c_char),
        &(uplo as c_char),
        &n,
        arf.as_ptr() as *const _,
        a.as_mut_ptr() as *mut _,
        &lda,
        info,
    )
}

#[inline]
pub unsafe fn ctgevc(
    side: u8,
    howmny: u8,
    select: &[i32],
    n: i32,
    s: &[c32],
    lds: i32,
    p: &[c32],
    ldp: i32,
    vl: &mut [c32],
    ldvl: i32,
    vr: &mut [c32],
    ldvr: i32,
    mm: i32,
    m: &mut i32,
    work: &mut [c32],
    rwork: &mut [f32],
    info: &mut i32,
) {
    ffi::ctgevc_(
        &(side as c_char),
        &(howmny as c_char),
        select.as_ptr(),
        &n,
        s.as_ptr() as *const _,
        &lds,
        p.as_ptr() as *const _,
        &ldp,
        vl.as_mut_ptr() as *mut _,
        &ldvl,
        vr.as_mut_ptr() as *mut _,
        &ldvr,
        &mm,
        m,
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn dtgevc(
    side: u8,
    howmny: u8,
    select: &[i32],
    n: i32,
    s: &[f64],
    lds: i32,
    p: &[f64],
    ldp: i32,
    vl: &mut [f64],
    ldvl: i32,
    vr: &mut [f64],
    ldvr: i32,
    mm: i32,
    m: &mut i32,
    work: &mut [f64],
    info: &mut i32,
) {
    ffi::dtgevc_(
        &(side as c_char),
        &(howmny as c_char),
        select.as_ptr(),
        &n,
        s.as_ptr(),
        &lds,
        p.as_ptr(),
        &ldp,
        vl.as_mut_ptr(),
        &ldvl,
        vr.as_mut_ptr(),
        &ldvr,
        &mm,
        m,
        work.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn stgevc(
    side: u8,
    howmny: u8,
    select: &[i32],
    n: i32,
    s: &[f32],
    lds: i32,
    p: &[f32],
    ldp: i32,
    vl: &mut [f32],
    ldvl: i32,
    vr: &mut [f32],
    ldvr: i32,
    mm: i32,
    m: &mut i32,
    work: &mut [f32],
    info: &mut i32,
) {
    ffi::stgevc_(
        &(side as c_char),
        &(howmny as c_char),
        select.as_ptr(),
        &n,
        s.as_ptr(),
        &lds,
        p.as_ptr(),
        &ldp,
        vl.as_mut_ptr(),
        &ldvl,
        vr.as_mut_ptr(),
        &ldvr,
        &mm,
        m,
        work.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn ztgevc(
    side: u8,
    howmny: u8,
    select: &[i32],
    n: i32,
    s: &[c64],
    lds: i32,
    p: &[c64],
    ldp: i32,
    vl: &mut [c64],
    ldvl: i32,
    vr: &mut [c64],
    ldvr: i32,
    mm: i32,
    m: &mut i32,
    work: &mut [c64],
    rwork: &mut [f64],
    info: &mut i32,
) {
    ffi::ztgevc_(
        &(side as c_char),
        &(howmny as c_char),
        select.as_ptr(),
        &n,
        s.as_ptr() as *const _,
        &lds,
        p.as_ptr() as *const _,
        &ldp,
        vl.as_mut_ptr() as *mut _,
        &ldvl,
        vr.as_mut_ptr() as *mut _,
        &ldvr,
        &mm,
        m,
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn ctgexc(
    wantq: &[i32],
    wantz: &[i32],
    n: i32,
    a: &mut [c32],
    lda: i32,
    b: &mut [c32],
    ldb: i32,
    q: &mut [c32],
    ldq: i32,
    z: &mut [c32],
    ldz: i32,
    ifst: &[i32],
    ilst: &mut [i32],
    info: &mut i32,
) {
    ffi::ctgexc_(
        wantq.as_ptr(),
        wantz.as_ptr(),
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        b.as_mut_ptr() as *mut _,
        &ldb,
        q.as_mut_ptr() as *mut _,
        &ldq,
        z.as_mut_ptr() as *mut _,
        &ldz,
        ifst.as_ptr(),
        ilst.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn dtgexc(
    wantq: &[i32],
    wantz: &[i32],
    n: i32,
    a: &mut [f64],
    lda: i32,
    b: &mut [f64],
    ldb: i32,
    q: &mut [f64],
    ldq: i32,
    z: &mut [f64],
    ldz: i32,
    ifst: &mut [i32],
    ilst: &mut [i32],
    work: &mut [f64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::dtgexc_(
        wantq.as_ptr(),
        wantz.as_ptr(),
        &n,
        a.as_mut_ptr(),
        &lda,
        b.as_mut_ptr(),
        &ldb,
        q.as_mut_ptr(),
        &ldq,
        z.as_mut_ptr(),
        &ldz,
        ifst.as_mut_ptr(),
        ilst.as_mut_ptr(),
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn stgexc(
    wantq: &[i32],
    wantz: &[i32],
    n: i32,
    a: &mut [f32],
    lda: i32,
    b: &mut [f32],
    ldb: i32,
    q: &mut [f32],
    ldq: i32,
    z: &mut [f32],
    ldz: i32,
    ifst: &mut [i32],
    ilst: &mut [i32],
    work: &mut [f32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::stgexc_(
        wantq.as_ptr(),
        wantz.as_ptr(),
        &n,
        a.as_mut_ptr(),
        &lda,
        b.as_mut_ptr(),
        &ldb,
        q.as_mut_ptr(),
        &ldq,
        z.as_mut_ptr(),
        &ldz,
        ifst.as_mut_ptr(),
        ilst.as_mut_ptr(),
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn ztgexc(
    wantq: &[i32],
    wantz: &[i32],
    n: i32,
    a: &mut [c64],
    lda: i32,
    b: &mut [c64],
    ldb: i32,
    q: &mut [c64],
    ldq: i32,
    z: &mut [c64],
    ldz: i32,
    ifst: &[i32],
    ilst: &mut [i32],
    info: &mut i32,
) {
    ffi::ztgexc_(
        wantq.as_ptr(),
        wantz.as_ptr(),
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        b.as_mut_ptr() as *mut _,
        &ldb,
        q.as_mut_ptr() as *mut _,
        &ldq,
        z.as_mut_ptr() as *mut _,
        &ldz,
        ifst.as_ptr(),
        ilst.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn ctgsen(
    ijob: &[i32],
    wantq: &[i32],
    wantz: &[i32],
    select: &[i32],
    n: i32,
    a: &mut [c32],
    lda: i32,
    b: &mut [c32],
    ldb: i32,
    alpha: &mut [c32],
    beta: &mut [c32],
    q: &mut [c32],
    ldq: i32,
    z: &mut [c32],
    ldz: i32,
    m: &mut i32,
    pl: &mut [f32],
    pr: &mut [f32],
    dif: &mut [f32],
    work: &mut [c32],
    lwork: i32,
    iwork: &mut [i32],
    liwork: i32,
    info: &mut i32,
) {
    ffi::ctgsen_(
        ijob.as_ptr(),
        wantq.as_ptr(),
        wantz.as_ptr(),
        select.as_ptr(),
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        b.as_mut_ptr() as *mut _,
        &ldb,
        alpha.as_mut_ptr() as *mut _,
        beta.as_mut_ptr() as *mut _,
        q.as_mut_ptr() as *mut _,
        &ldq,
        z.as_mut_ptr() as *mut _,
        &ldz,
        m,
        pl.as_mut_ptr(),
        pr.as_mut_ptr(),
        dif.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        &lwork,
        iwork.as_mut_ptr(),
        &liwork,
        info,
    )
}

#[inline]
pub unsafe fn dtgsen(
    ijob: &[i32],
    wantq: &[i32],
    wantz: &[i32],
    select: &[i32],
    n: i32,
    a: &mut [f64],
    lda: i32,
    b: &mut [f64],
    ldb: i32,
    alphar: &mut [f64],
    alphai: &mut [f64],
    beta: &mut [f64],
    q: &mut [f64],
    ldq: i32,
    z: &mut [f64],
    ldz: i32,
    m: &mut i32,
    pl: &mut [f64],
    pr: &mut [f64],
    dif: &mut [f64],
    work: &mut [f64],
    lwork: i32,
    iwork: &mut [i32],
    liwork: i32,
    info: &mut i32,
) {
    ffi::dtgsen_(
        ijob.as_ptr(),
        wantq.as_ptr(),
        wantz.as_ptr(),
        select.as_ptr(),
        &n,
        a.as_mut_ptr(),
        &lda,
        b.as_mut_ptr(),
        &ldb,
        alphar.as_mut_ptr(),
        alphai.as_mut_ptr(),
        beta.as_mut_ptr(),
        q.as_mut_ptr(),
        &ldq,
        z.as_mut_ptr(),
        &ldz,
        m,
        pl.as_mut_ptr(),
        pr.as_mut_ptr(),
        dif.as_mut_ptr(),
        work.as_mut_ptr(),
        &lwork,
        iwork.as_mut_ptr(),
        &liwork,
        info,
    )
}

#[inline]
pub unsafe fn stgsen(
    ijob: &[i32],
    wantq: &[i32],
    wantz: &[i32],
    select: &[i32],
    n: i32,
    a: &mut [f32],
    lda: i32,
    b: &mut [f32],
    ldb: i32,
    alphar: &mut [f32],
    alphai: &mut [f32],
    beta: &mut [f32],
    q: &mut [f32],
    ldq: i32,
    z: &mut [f32],
    ldz: i32,
    m: &mut i32,
    pl: &mut [f32],
    pr: &mut [f32],
    dif: &mut [f32],
    work: &mut [f32],
    lwork: i32,
    iwork: &mut [i32],
    liwork: i32,
    info: &mut i32,
) {
    ffi::stgsen_(
        ijob.as_ptr(),
        wantq.as_ptr(),
        wantz.as_ptr(),
        select.as_ptr(),
        &n,
        a.as_mut_ptr(),
        &lda,
        b.as_mut_ptr(),
        &ldb,
        alphar.as_mut_ptr(),
        alphai.as_mut_ptr(),
        beta.as_mut_ptr(),
        q.as_mut_ptr(),
        &ldq,
        z.as_mut_ptr(),
        &ldz,
        m,
        pl.as_mut_ptr(),
        pr.as_mut_ptr(),
        dif.as_mut_ptr(),
        work.as_mut_ptr(),
        &lwork,
        iwork.as_mut_ptr(),
        &liwork,
        info,
    )
}

#[inline]
pub unsafe fn ztgsen(
    ijob: &[i32],
    wantq: &[i32],
    wantz: &[i32],
    select: &[i32],
    n: i32,
    a: &mut [c64],
    lda: i32,
    b: &mut [c64],
    ldb: i32,
    alpha: &mut [c64],
    beta: &mut [c64],
    q: &mut [c64],
    ldq: i32,
    z: &mut [c64],
    ldz: i32,
    m: &mut i32,
    pl: &mut [f64],
    pr: &mut [f64],
    dif: &mut [f64],
    work: &mut [c64],
    lwork: i32,
    iwork: &mut [i32],
    liwork: i32,
    info: &mut i32,
) {
    ffi::ztgsen_(
        ijob.as_ptr(),
        wantq.as_ptr(),
        wantz.as_ptr(),
        select.as_ptr(),
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        b.as_mut_ptr() as *mut _,
        &ldb,
        alpha.as_mut_ptr() as *mut _,
        beta.as_mut_ptr() as *mut _,
        q.as_mut_ptr() as *mut _,
        &ldq,
        z.as_mut_ptr() as *mut _,
        &ldz,
        m,
        pl.as_mut_ptr(),
        pr.as_mut_ptr(),
        dif.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        &lwork,
        iwork.as_mut_ptr(),
        &liwork,
        info,
    )
}

#[inline]
pub unsafe fn ctgsja(
    jobu: u8,
    jobv: u8,
    jobq: u8,
    m: i32,
    p: i32,
    n: i32,
    k: i32,
    l: i32,
    a: &mut [c32],
    lda: i32,
    b: &mut [c32],
    ldb: i32,
    tola: f32,
    tolb: f32,
    alpha: &mut [f32],
    beta: &mut [f32],
    u: &mut [c32],
    ldu: i32,
    v: &mut [c32],
    ldv: i32,
    q: &mut [c32],
    ldq: i32,
    work: &mut [c32],
    ncycle: &mut [i32],
    info: &mut i32,
) {
    ffi::ctgsja_(
        &(jobu as c_char),
        &(jobv as c_char),
        &(jobq as c_char),
        &m,
        &p,
        &n,
        &k,
        &l,
        a.as_mut_ptr() as *mut _,
        &lda,
        b.as_mut_ptr() as *mut _,
        &ldb,
        &tola,
        &tolb,
        alpha.as_mut_ptr(),
        beta.as_mut_ptr(),
        u.as_mut_ptr() as *mut _,
        &ldu,
        v.as_mut_ptr() as *mut _,
        &ldv,
        q.as_mut_ptr() as *mut _,
        &ldq,
        work.as_mut_ptr() as *mut _,
        ncycle.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn dtgsja(
    jobu: u8,
    jobv: u8,
    jobq: u8,
    m: i32,
    p: i32,
    n: i32,
    k: i32,
    l: i32,
    a: &mut [f64],
    lda: i32,
    b: &mut [f64],
    ldb: i32,
    tola: f64,
    tolb: f64,
    alpha: &mut [f64],
    beta: &mut [f64],
    u: &mut [f64],
    ldu: i32,
    v: &mut [f64],
    ldv: i32,
    q: &mut [f64],
    ldq: i32,
    work: &mut [f64],
    ncycle: &mut [i32],
    info: &mut i32,
) {
    ffi::dtgsja_(
        &(jobu as c_char),
        &(jobv as c_char),
        &(jobq as c_char),
        &m,
        &p,
        &n,
        &k,
        &l,
        a.as_mut_ptr(),
        &lda,
        b.as_mut_ptr(),
        &ldb,
        &tola,
        &tolb,
        alpha.as_mut_ptr(),
        beta.as_mut_ptr(),
        u.as_mut_ptr(),
        &ldu,
        v.as_mut_ptr(),
        &ldv,
        q.as_mut_ptr(),
        &ldq,
        work.as_mut_ptr(),
        ncycle.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn stgsja(
    jobu: u8,
    jobv: u8,
    jobq: u8,
    m: i32,
    p: i32,
    n: i32,
    k: i32,
    l: i32,
    a: &mut [f32],
    lda: i32,
    b: &mut [f32],
    ldb: i32,
    tola: f32,
    tolb: f32,
    alpha: &mut [f32],
    beta: &mut [f32],
    u: &mut [f32],
    ldu: i32,
    v: &mut [f32],
    ldv: i32,
    q: &mut [f32],
    ldq: i32,
    work: &mut [f32],
    ncycle: &mut [i32],
    info: &mut i32,
) {
    ffi::stgsja_(
        &(jobu as c_char),
        &(jobv as c_char),
        &(jobq as c_char),
        &m,
        &p,
        &n,
        &k,
        &l,
        a.as_mut_ptr(),
        &lda,
        b.as_mut_ptr(),
        &ldb,
        &tola,
        &tolb,
        alpha.as_mut_ptr(),
        beta.as_mut_ptr(),
        u.as_mut_ptr(),
        &ldu,
        v.as_mut_ptr(),
        &ldv,
        q.as_mut_ptr(),
        &ldq,
        work.as_mut_ptr(),
        ncycle.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn ztgsja(
    jobu: u8,
    jobv: u8,
    jobq: u8,
    m: i32,
    p: i32,
    n: i32,
    k: i32,
    l: i32,
    a: &mut [c64],
    lda: i32,
    b: &mut [c64],
    ldb: i32,
    tola: f64,
    tolb: f64,
    alpha: &mut [f64],
    beta: &mut [f64],
    u: &mut [c64],
    ldu: i32,
    v: &mut [c64],
    ldv: i32,
    q: &mut [c64],
    ldq: i32,
    work: &mut [c64],
    ncycle: &mut [i32],
    info: &mut i32,
) {
    ffi::ztgsja_(
        &(jobu as c_char),
        &(jobv as c_char),
        &(jobq as c_char),
        &m,
        &p,
        &n,
        &k,
        &l,
        a.as_mut_ptr() as *mut _,
        &lda,
        b.as_mut_ptr() as *mut _,
        &ldb,
        &tola,
        &tolb,
        alpha.as_mut_ptr(),
        beta.as_mut_ptr(),
        u.as_mut_ptr() as *mut _,
        &ldu,
        v.as_mut_ptr() as *mut _,
        &ldv,
        q.as_mut_ptr() as *mut _,
        &ldq,
        work.as_mut_ptr() as *mut _,
        ncycle.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn ctgsna(
    job: u8,
    howmny: u8,
    select: &[i32],
    n: i32,
    a: &[c32],
    lda: i32,
    b: &[c32],
    ldb: i32,
    vl: &[c32],
    ldvl: i32,
    vr: &[c32],
    ldvr: i32,
    s: &mut [f32],
    dif: &mut [f32],
    mm: i32,
    m: &mut i32,
    work: &mut [c32],
    lwork: i32,
    iwork: &mut [i32],
    info: &mut i32,
) {
    ffi::ctgsna_(
        &(job as c_char),
        &(howmny as c_char),
        select.as_ptr(),
        &n,
        a.as_ptr() as *const _,
        &lda,
        b.as_ptr() as *const _,
        &ldb,
        vl.as_ptr() as *const _,
        &ldvl,
        vr.as_ptr() as *const _,
        &ldvr,
        s.as_mut_ptr(),
        dif.as_mut_ptr(),
        &mm,
        m,
        work.as_mut_ptr() as *mut _,
        &lwork,
        iwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn dtgsna(
    job: u8,
    howmny: u8,
    select: &[i32],
    n: i32,
    a: &[f64],
    lda: i32,
    b: &[f64],
    ldb: i32,
    vl: &[f64],
    ldvl: i32,
    vr: &[f64],
    ldvr: i32,
    s: &mut [f64],
    dif: &mut [f64],
    mm: i32,
    m: &mut i32,
    work: &mut [f64],
    lwork: i32,
    iwork: &mut [i32],
    info: &mut i32,
) {
    ffi::dtgsna_(
        &(job as c_char),
        &(howmny as c_char),
        select.as_ptr(),
        &n,
        a.as_ptr(),
        &lda,
        b.as_ptr(),
        &ldb,
        vl.as_ptr(),
        &ldvl,
        vr.as_ptr(),
        &ldvr,
        s.as_mut_ptr(),
        dif.as_mut_ptr(),
        &mm,
        m,
        work.as_mut_ptr(),
        &lwork,
        iwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn stgsna(
    job: u8,
    howmny: u8,
    select: &[i32],
    n: i32,
    a: &[f32],
    lda: i32,
    b: &[f32],
    ldb: i32,
    vl: &[f32],
    ldvl: i32,
    vr: &[f32],
    ldvr: i32,
    s: &mut [f32],
    dif: &mut [f32],
    mm: i32,
    m: &mut i32,
    work: &mut [f32],
    lwork: i32,
    iwork: &mut [i32],
    info: &mut i32,
) {
    ffi::stgsna_(
        &(job as c_char),
        &(howmny as c_char),
        select.as_ptr(),
        &n,
        a.as_ptr(),
        &lda,
        b.as_ptr(),
        &ldb,
        vl.as_ptr(),
        &ldvl,
        vr.as_ptr(),
        &ldvr,
        s.as_mut_ptr(),
        dif.as_mut_ptr(),
        &mm,
        m,
        work.as_mut_ptr(),
        &lwork,
        iwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn ztgsna(
    job: u8,
    howmny: u8,
    select: &[i32],
    n: i32,
    a: &[c64],
    lda: i32,
    b: &[c64],
    ldb: i32,
    vl: &[c64],
    ldvl: i32,
    vr: &[c64],
    ldvr: i32,
    s: &mut [f64],
    dif: &mut [f64],
    mm: i32,
    m: &mut i32,
    work: &mut [c64],
    lwork: i32,
    iwork: &mut [i32],
    info: &mut i32,
) {
    ffi::ztgsna_(
        &(job as c_char),
        &(howmny as c_char),
        select.as_ptr(),
        &n,
        a.as_ptr() as *const _,
        &lda,
        b.as_ptr() as *const _,
        &ldb,
        vl.as_ptr() as *const _,
        &ldvl,
        vr.as_ptr() as *const _,
        &ldvr,
        s.as_mut_ptr(),
        dif.as_mut_ptr(),
        &mm,
        m,
        work.as_mut_ptr() as *mut _,
        &lwork,
        iwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn ctgsyl(
    trans: u8,
    ijob: &[i32],
    m: i32,
    n: i32,
    a: &[c32],
    lda: i32,
    b: &[c32],
    ldb: i32,
    c: &mut [c32],
    ldc: i32,
    d: &[c32],
    ldd: i32,
    e: &[c32],
    lde: i32,
    f: &mut [c32],
    ldf: i32,
    dif: &mut f32,
    scale: &mut [f32],
    work: &mut [c32],
    lwork: i32,
    iwork: &mut [i32],
    info: &mut i32,
) {
    ffi::ctgsyl_(
        &(trans as c_char),
        ijob.as_ptr(),
        &m,
        &n,
        a.as_ptr() as *const _,
        &lda,
        b.as_ptr() as *const _,
        &ldb,
        c.as_mut_ptr() as *mut _,
        &ldc,
        d.as_ptr() as *const _,
        &ldd,
        e.as_ptr() as *const _,
        &lde,
        f.as_mut_ptr() as *mut _,
        &ldf,
        dif,
        scale.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        &lwork,
        iwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn dtgsyl(
    trans: u8,
    ijob: &[i32],
    m: i32,
    n: i32,
    a: &[f64],
    lda: i32,
    b: &[f64],
    ldb: i32,
    c: &mut [f64],
    ldc: i32,
    d: &[f64],
    ldd: i32,
    e: &[f64],
    lde: i32,
    f: &mut [f64],
    ldf: i32,
    dif: &mut f64,
    scale: &mut [f64],
    work: &mut [f64],
    lwork: i32,
    iwork: &mut [i32],
    info: &mut i32,
) {
    ffi::dtgsyl_(
        &(trans as c_char),
        ijob.as_ptr(),
        &m,
        &n,
        a.as_ptr(),
        &lda,
        b.as_ptr(),
        &ldb,
        c.as_mut_ptr(),
        &ldc,
        d.as_ptr(),
        &ldd,
        e.as_ptr(),
        &lde,
        f.as_mut_ptr(),
        &ldf,
        dif,
        scale.as_mut_ptr(),
        work.as_mut_ptr(),
        &lwork,
        iwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn stgsyl(
    trans: u8,
    ijob: &[i32],
    m: i32,
    n: i32,
    a: &[f32],
    lda: i32,
    b: &[f32],
    ldb: i32,
    c: &mut [f32],
    ldc: i32,
    d: &[f32],
    ldd: i32,
    e: &[f32],
    lde: i32,
    f: &mut [f32],
    ldf: i32,
    dif: &mut f32,
    scale: &mut [f32],
    work: &mut [f32],
    lwork: i32,
    iwork: &mut [i32],
    info: &mut i32,
) {
    ffi::stgsyl_(
        &(trans as c_char),
        ijob.as_ptr(),
        &m,
        &n,
        a.as_ptr(),
        &lda,
        b.as_ptr(),
        &ldb,
        c.as_mut_ptr(),
        &ldc,
        d.as_ptr(),
        &ldd,
        e.as_ptr(),
        &lde,
        f.as_mut_ptr(),
        &ldf,
        dif,
        scale.as_mut_ptr(),
        work.as_mut_ptr(),
        &lwork,
        iwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn ztgsyl(
    trans: u8,
    ijob: &[i32],
    m: i32,
    n: i32,
    a: &[c64],
    lda: i32,
    b: &[c64],
    ldb: i32,
    c: &mut [c64],
    ldc: i32,
    d: &[c64],
    ldd: i32,
    e: &[c64],
    lde: i32,
    f: &mut [c64],
    ldf: i32,
    dif: &mut f64,
    scale: &mut [f64],
    work: &mut [c64],
    lwork: i32,
    iwork: &mut [i32],
    info: &mut i32,
) {
    ffi::ztgsyl_(
        &(trans as c_char),
        ijob.as_ptr(),
        &m,
        &n,
        a.as_ptr() as *const _,
        &lda,
        b.as_ptr() as *const _,
        &ldb,
        c.as_mut_ptr() as *mut _,
        &ldc,
        d.as_ptr() as *const _,
        &ldd,
        e.as_ptr() as *const _,
        &lde,
        f.as_mut_ptr() as *mut _,
        &ldf,
        dif,
        scale.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        &lwork,
        iwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn ctpcon(
    norm: u8,
    uplo: u8,
    diag: u8,
    n: i32,
    ap: &[c32],
    rcond: &mut f32,
    work: &mut [c32],
    rwork: &mut [f32],
    info: &mut i32,
) {
    ffi::ctpcon_(
        &(norm as c_char),
        &(uplo as c_char),
        &(diag as c_char),
        &n,
        ap.as_ptr() as *const _,
        rcond,
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn dtpcon(
    norm: u8,
    uplo: u8,
    diag: u8,
    n: i32,
    ap: &[f64],
    rcond: &mut f64,
    work: &mut [f64],
    iwork: &mut [i32],
    info: &mut i32,
) {
    ffi::dtpcon_(
        &(norm as c_char),
        &(uplo as c_char),
        &(diag as c_char),
        &n,
        ap.as_ptr(),
        rcond,
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn stpcon(
    norm: u8,
    uplo: u8,
    diag: u8,
    n: i32,
    ap: &[f32],
    rcond: &mut f32,
    work: &mut [f32],
    iwork: &mut [i32],
    info: &mut i32,
) {
    ffi::stpcon_(
        &(norm as c_char),
        &(uplo as c_char),
        &(diag as c_char),
        &n,
        ap.as_ptr(),
        rcond,
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn ztpcon(
    norm: u8,
    uplo: u8,
    diag: u8,
    n: i32,
    ap: &[c64],
    rcond: &mut f64,
    work: &mut [c64],
    rwork: &mut [f64],
    info: &mut i32,
) {
    ffi::ztpcon_(
        &(norm as c_char),
        &(uplo as c_char),
        &(diag as c_char),
        &n,
        ap.as_ptr() as *const _,
        rcond,
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn ctplqt(
    m: i32,
    n: i32,
    l: i32,
    mb: &[i32],
    a: &mut [c32],
    lda: i32,
    b: &mut [c32],
    ldb: i32,
    t: &mut [c32],
    ldt: i32,
    work: &mut [c32],
    info: &mut i32,
) {
    ffi::ctplqt_(
        &m,
        &n,
        &l,
        mb.as_ptr(),
        a.as_mut_ptr() as *mut _,
        &lda,
        b.as_mut_ptr() as *mut _,
        &ldb,
        t.as_mut_ptr() as *mut _,
        &ldt,
        work.as_mut_ptr() as *mut _,
        info,
    )
}

#[inline]
pub unsafe fn dtplqt(
    m: i32,
    n: i32,
    l: i32,
    mb: &[i32],
    a: &mut [f64],
    lda: i32,
    b: &mut [f64],
    ldb: i32,
    t: &mut [f64],
    ldt: i32,
    work: &mut [f64],
    info: &mut i32,
) {
    ffi::dtplqt_(
        &m,
        &n,
        &l,
        mb.as_ptr(),
        a.as_mut_ptr(),
        &lda,
        b.as_mut_ptr(),
        &ldb,
        t.as_mut_ptr(),
        &ldt,
        work.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn stplqt(
    m: i32,
    n: i32,
    l: i32,
    mb: &[i32],
    a: &mut [f32],
    lda: i32,
    b: &mut [f32],
    ldb: i32,
    t: &mut [f32],
    ldt: i32,
    work: &mut [f32],
    info: &mut i32,
) {
    ffi::stplqt_(
        &m,
        &n,
        &l,
        mb.as_ptr(),
        a.as_mut_ptr(),
        &lda,
        b.as_mut_ptr(),
        &ldb,
        t.as_mut_ptr(),
        &ldt,
        work.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn ztplqt(
    m: i32,
    n: i32,
    l: i32,
    mb: &[i32],
    a: &mut [c64],
    lda: i32,
    b: &mut [c64],
    ldb: i32,
    t: &mut [c64],
    ldt: i32,
    work: &mut [c64],
    info: &mut i32,
) {
    ffi::ztplqt_(
        &m,
        &n,
        &l,
        mb.as_ptr(),
        a.as_mut_ptr() as *mut _,
        &lda,
        b.as_mut_ptr() as *mut _,
        &ldb,
        t.as_mut_ptr() as *mut _,
        &ldt,
        work.as_mut_ptr() as *mut _,
        info,
    )
}

#[inline]
pub unsafe fn ctplqt2(
    m: i32,
    n: i32,
    l: i32,
    a: &mut [c32],
    lda: i32,
    b: &mut [c32],
    ldb: i32,
    t: &mut [c32],
    ldt: i32,
    info: &mut i32,
) {
    ffi::ctplqt2_(
        &m,
        &n,
        &l,
        a.as_mut_ptr() as *mut _,
        &lda,
        b.as_mut_ptr() as *mut _,
        &ldb,
        t.as_mut_ptr() as *mut _,
        &ldt,
        info,
    )
}

#[inline]
pub unsafe fn dtplqt2(
    m: i32,
    n: i32,
    l: i32,
    a: &mut [f64],
    lda: i32,
    b: &mut [f64],
    ldb: i32,
    t: &mut [f64],
    ldt: i32,
    info: &mut i32,
) {
    ffi::dtplqt2_(
        &m,
        &n,
        &l,
        a.as_mut_ptr(),
        &lda,
        b.as_mut_ptr(),
        &ldb,
        t.as_mut_ptr(),
        &ldt,
        info,
    )
}

#[inline]
pub unsafe fn stplqt2(
    m: i32,
    n: i32,
    l: i32,
    a: &mut [f32],
    lda: i32,
    b: &mut [f32],
    ldb: i32,
    t: &mut [f32],
    ldt: i32,
    info: &mut i32,
) {
    ffi::stplqt2_(
        &m,
        &n,
        &l,
        a.as_mut_ptr(),
        &lda,
        b.as_mut_ptr(),
        &ldb,
        t.as_mut_ptr(),
        &ldt,
        info,
    )
}

#[inline]
pub unsafe fn ztplqt2(
    m: i32,
    n: i32,
    l: i32,
    a: &mut [c64],
    lda: i32,
    b: &mut [c64],
    ldb: i32,
    t: &mut [c64],
    ldt: i32,
    info: &mut i32,
) {
    ffi::ztplqt2_(
        &m,
        &n,
        &l,
        a.as_mut_ptr() as *mut _,
        &lda,
        b.as_mut_ptr() as *mut _,
        &ldb,
        t.as_mut_ptr() as *mut _,
        &ldt,
        info,
    )
}

#[inline]
pub unsafe fn ctpmlqt(
    side: u8,
    trans: u8,
    m: i32,
    n: i32,
    k: i32,
    l: i32,
    mb: &[i32],
    v: &[c32],
    ldv: i32,
    t: &[c32],
    ldt: i32,
    a: &mut [c32],
    lda: i32,
    b: &mut [c32],
    ldb: i32,
    work: &mut [c32],
    info: &mut i32,
) {
    ffi::ctpmlqt_(
        &(side as c_char),
        &(trans as c_char),
        &m,
        &n,
        &k,
        &l,
        mb.as_ptr(),
        v.as_ptr() as *const _,
        &ldv,
        t.as_ptr() as *const _,
        &ldt,
        a.as_mut_ptr() as *mut _,
        &lda,
        b.as_mut_ptr() as *mut _,
        &ldb,
        work.as_mut_ptr() as *mut _,
        info,
    )
}

#[inline]
pub unsafe fn dtpmlqt(
    side: u8,
    trans: u8,
    m: i32,
    n: i32,
    k: i32,
    l: i32,
    mb: &[i32],
    v: &[f64],
    ldv: i32,
    t: &[f64],
    ldt: i32,
    a: &mut [f64],
    lda: i32,
    b: &mut [f64],
    ldb: i32,
    work: &mut [f64],
    info: &mut i32,
) {
    ffi::dtpmlqt_(
        &(side as c_char),
        &(trans as c_char),
        &m,
        &n,
        &k,
        &l,
        mb.as_ptr(),
        v.as_ptr(),
        &ldv,
        t.as_ptr(),
        &ldt,
        a.as_mut_ptr(),
        &lda,
        b.as_mut_ptr(),
        &ldb,
        work.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn stpmlqt(
    side: u8,
    trans: u8,
    m: i32,
    n: i32,
    k: i32,
    l: i32,
    mb: &[i32],
    v: &[f32],
    ldv: i32,
    t: &[f32],
    ldt: i32,
    a: &mut [f32],
    lda: i32,
    b: &mut [f32],
    ldb: i32,
    work: &mut [f32],
    info: &mut i32,
) {
    ffi::stpmlqt_(
        &(side as c_char),
        &(trans as c_char),
        &m,
        &n,
        &k,
        &l,
        mb.as_ptr(),
        v.as_ptr(),
        &ldv,
        t.as_ptr(),
        &ldt,
        a.as_mut_ptr(),
        &lda,
        b.as_mut_ptr(),
        &ldb,
        work.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn ztpmlqt(
    side: u8,
    trans: u8,
    m: i32,
    n: i32,
    k: i32,
    l: i32,
    mb: &[i32],
    v: &[c64],
    ldv: i32,
    t: &[c64],
    ldt: i32,
    a: &mut [c64],
    lda: i32,
    b: &mut [c64],
    ldb: i32,
    work: &mut [c64],
    info: &mut i32,
) {
    ffi::ztpmlqt_(
        &(side as c_char),
        &(trans as c_char),
        &m,
        &n,
        &k,
        &l,
        mb.as_ptr(),
        v.as_ptr() as *const _,
        &ldv,
        t.as_ptr() as *const _,
        &ldt,
        a.as_mut_ptr() as *mut _,
        &lda,
        b.as_mut_ptr() as *mut _,
        &ldb,
        work.as_mut_ptr() as *mut _,
        info,
    )
}

#[inline]
pub unsafe fn ctpmqrt(
    side: u8,
    trans: u8,
    m: i32,
    n: i32,
    k: i32,
    l: i32,
    nb: i32,
    v: &[c32],
    ldv: i32,
    t: &[c32],
    ldt: i32,
    a: &mut [c32],
    lda: i32,
    b: &mut [c32],
    ldb: i32,
    work: &mut [c32],
    info: &mut i32,
) {
    ffi::ctpmqrt_(
        &(side as c_char),
        &(trans as c_char),
        &m,
        &n,
        &k,
        &l,
        &nb,
        v.as_ptr() as *const _,
        &ldv,
        t.as_ptr() as *const _,
        &ldt,
        a.as_mut_ptr() as *mut _,
        &lda,
        b.as_mut_ptr() as *mut _,
        &ldb,
        work.as_mut_ptr() as *mut _,
        info,
    )
}

#[inline]
pub unsafe fn dtpmqrt(
    side: u8,
    trans: u8,
    m: i32,
    n: i32,
    k: i32,
    l: i32,
    nb: i32,
    v: &[f64],
    ldv: i32,
    t: &[f64],
    ldt: i32,
    a: &mut [f64],
    lda: i32,
    b: &mut [f64],
    ldb: i32,
    work: &mut [f64],
    info: &mut i32,
) {
    ffi::dtpmqrt_(
        &(side as c_char),
        &(trans as c_char),
        &m,
        &n,
        &k,
        &l,
        &nb,
        v.as_ptr(),
        &ldv,
        t.as_ptr(),
        &ldt,
        a.as_mut_ptr(),
        &lda,
        b.as_mut_ptr(),
        &ldb,
        work.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn stpmqrt(
    side: u8,
    trans: u8,
    m: i32,
    n: i32,
    k: i32,
    l: i32,
    nb: i32,
    v: &[f32],
    ldv: i32,
    t: &[f32],
    ldt: i32,
    a: &mut [f32],
    lda: i32,
    b: &mut [f32],
    ldb: i32,
    work: &mut [f32],
    info: &mut i32,
) {
    ffi::stpmqrt_(
        &(side as c_char),
        &(trans as c_char),
        &m,
        &n,
        &k,
        &l,
        &nb,
        v.as_ptr(),
        &ldv,
        t.as_ptr(),
        &ldt,
        a.as_mut_ptr(),
        &lda,
        b.as_mut_ptr(),
        &ldb,
        work.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn ztpmqrt(
    side: u8,
    trans: u8,
    m: i32,
    n: i32,
    k: i32,
    l: i32,
    nb: i32,
    v: &[c64],
    ldv: i32,
    t: &[c64],
    ldt: i32,
    a: &mut [c64],
    lda: i32,
    b: &mut [c64],
    ldb: i32,
    work: &mut [c64],
    info: &mut i32,
) {
    ffi::ztpmqrt_(
        &(side as c_char),
        &(trans as c_char),
        &m,
        &n,
        &k,
        &l,
        &nb,
        v.as_ptr() as *const _,
        &ldv,
        t.as_ptr() as *const _,
        &ldt,
        a.as_mut_ptr() as *mut _,
        &lda,
        b.as_mut_ptr() as *mut _,
        &ldb,
        work.as_mut_ptr() as *mut _,
        info,
    )
}

#[inline]
pub unsafe fn ctpqrt(
    m: i32,
    n: i32,
    l: i32,
    nb: i32,
    a: &mut [c32],
    lda: i32,
    b: &mut [c32],
    ldb: i32,
    t: &mut [c32],
    ldt: i32,
    work: &mut [c32],
    info: &mut i32,
) {
    ffi::ctpqrt_(
        &m,
        &n,
        &l,
        &nb,
        a.as_mut_ptr() as *mut _,
        &lda,
        b.as_mut_ptr() as *mut _,
        &ldb,
        t.as_mut_ptr() as *mut _,
        &ldt,
        work.as_mut_ptr() as *mut _,
        info,
    )
}

#[inline]
pub unsafe fn dtpqrt(
    m: i32,
    n: i32,
    l: i32,
    nb: i32,
    a: &mut [f64],
    lda: i32,
    b: &mut [f64],
    ldb: i32,
    t: &mut [f64],
    ldt: i32,
    work: &mut [f64],
    info: &mut i32,
) {
    ffi::dtpqrt_(
        &m,
        &n,
        &l,
        &nb,
        a.as_mut_ptr(),
        &lda,
        b.as_mut_ptr(),
        &ldb,
        t.as_mut_ptr(),
        &ldt,
        work.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn stpqrt(
    m: i32,
    n: i32,
    l: i32,
    nb: i32,
    a: &mut [f32],
    lda: i32,
    b: &mut [f32],
    ldb: i32,
    t: &mut [f32],
    ldt: i32,
    work: &mut [f32],
    info: &mut i32,
) {
    ffi::stpqrt_(
        &m,
        &n,
        &l,
        &nb,
        a.as_mut_ptr(),
        &lda,
        b.as_mut_ptr(),
        &ldb,
        t.as_mut_ptr(),
        &ldt,
        work.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn ztpqrt(
    m: i32,
    n: i32,
    l: i32,
    nb: i32,
    a: &mut [c64],
    lda: i32,
    b: &mut [c64],
    ldb: i32,
    t: &mut [c64],
    ldt: i32,
    work: &mut [c64],
    info: &mut i32,
) {
    ffi::ztpqrt_(
        &m,
        &n,
        &l,
        &nb,
        a.as_mut_ptr() as *mut _,
        &lda,
        b.as_mut_ptr() as *mut _,
        &ldb,
        t.as_mut_ptr() as *mut _,
        &ldt,
        work.as_mut_ptr() as *mut _,
        info,
    )
}

#[inline]
pub unsafe fn ctpqrt2(
    m: i32,
    n: i32,
    l: i32,
    a: &mut [c32],
    lda: i32,
    b: &mut [c32],
    ldb: i32,
    t: &mut [c32],
    ldt: i32,
    info: &mut i32,
) {
    ffi::ctpqrt2_(
        &m,
        &n,
        &l,
        a.as_mut_ptr() as *mut _,
        &lda,
        b.as_mut_ptr() as *mut _,
        &ldb,
        t.as_mut_ptr() as *mut _,
        &ldt,
        info,
    )
}

#[inline]
pub unsafe fn dtpqrt2(
    m: i32,
    n: i32,
    l: i32,
    a: &mut [f64],
    lda: i32,
    b: &mut [f64],
    ldb: i32,
    t: &mut [f64],
    ldt: i32,
    info: &mut i32,
) {
    ffi::dtpqrt2_(
        &m,
        &n,
        &l,
        a.as_mut_ptr(),
        &lda,
        b.as_mut_ptr(),
        &ldb,
        t.as_mut_ptr(),
        &ldt,
        info,
    )
}

#[inline]
pub unsafe fn stpqrt2(
    m: i32,
    n: i32,
    l: i32,
    a: &mut [f32],
    lda: i32,
    b: &mut [f32],
    ldb: i32,
    t: &mut [f32],
    ldt: i32,
    info: &mut i32,
) {
    ffi::stpqrt2_(
        &m,
        &n,
        &l,
        a.as_mut_ptr(),
        &lda,
        b.as_mut_ptr(),
        &ldb,
        t.as_mut_ptr(),
        &ldt,
        info,
    )
}

#[inline]
pub unsafe fn ztpqrt2(
    m: i32,
    n: i32,
    l: i32,
    a: &mut [c64],
    lda: i32,
    b: &mut [c64],
    ldb: i32,
    t: &mut [c64],
    ldt: i32,
    info: &mut i32,
) {
    ffi::ztpqrt2_(
        &m,
        &n,
        &l,
        a.as_mut_ptr() as *mut _,
        &lda,
        b.as_mut_ptr() as *mut _,
        &ldb,
        t.as_mut_ptr() as *mut _,
        &ldt,
        info,
    )
}

#[inline]
pub unsafe fn ctprfb(
    side: u8,
    trans: u8,
    direct: u8,
    storev: u8,
    m: i32,
    n: i32,
    k: i32,
    l: i32,
    v: &[c32],
    ldv: i32,
    t: &[c32],
    ldt: i32,
    a: &mut [c32],
    lda: i32,
    b: &mut [c32],
    ldb: i32,
    work: &mut [c32],
    ldwork: i32,
) {
    ffi::ctprfb_(
        &(side as c_char),
        &(trans as c_char),
        &(direct as c_char),
        &(storev as c_char),
        &m,
        &n,
        &k,
        &l,
        v.as_ptr() as *const _,
        &ldv,
        t.as_ptr() as *const _,
        &ldt,
        a.as_mut_ptr() as *mut _,
        &lda,
        b.as_mut_ptr() as *mut _,
        &ldb,
        work.as_mut_ptr() as *mut _,
        &ldwork,
    )
}

#[inline]
pub unsafe fn dtprfb(
    side: u8,
    trans: u8,
    direct: u8,
    storev: u8,
    m: i32,
    n: i32,
    k: i32,
    l: i32,
    v: &[f64],
    ldv: i32,
    t: &[f64],
    ldt: i32,
    a: &mut [f64],
    lda: i32,
    b: &mut [f64],
    ldb: i32,
    work: &mut [f64],
    ldwork: i32,
) {
    ffi::dtprfb_(
        &(side as c_char),
        &(trans as c_char),
        &(direct as c_char),
        &(storev as c_char),
        &m,
        &n,
        &k,
        &l,
        v.as_ptr(),
        &ldv,
        t.as_ptr(),
        &ldt,
        a.as_mut_ptr(),
        &lda,
        b.as_mut_ptr(),
        &ldb,
        work.as_mut_ptr(),
        &ldwork,
    )
}

#[inline]
pub unsafe fn stprfb(
    side: u8,
    trans: u8,
    direct: u8,
    storev: u8,
    m: i32,
    n: i32,
    k: i32,
    l: i32,
    v: &[f32],
    ldv: i32,
    t: &[f32],
    ldt: i32,
    a: &mut [f32],
    lda: i32,
    b: &mut [f32],
    ldb: i32,
    work: &mut [f32],
    ldwork: i32,
) {
    ffi::stprfb_(
        &(side as c_char),
        &(trans as c_char),
        &(direct as c_char),
        &(storev as c_char),
        &m,
        &n,
        &k,
        &l,
        v.as_ptr(),
        &ldv,
        t.as_ptr(),
        &ldt,
        a.as_mut_ptr(),
        &lda,
        b.as_mut_ptr(),
        &ldb,
        work.as_mut_ptr(),
        &ldwork,
    )
}

#[inline]
pub unsafe fn ztprfb(
    side: u8,
    trans: u8,
    direct: u8,
    storev: u8,
    m: i32,
    n: i32,
    k: i32,
    l: i32,
    v: &[c64],
    ldv: i32,
    t: &[c64],
    ldt: i32,
    a: &mut [c64],
    lda: i32,
    b: &mut [c64],
    ldb: i32,
    work: &mut [c64],
    ldwork: i32,
) {
    ffi::ztprfb_(
        &(side as c_char),
        &(trans as c_char),
        &(direct as c_char),
        &(storev as c_char),
        &m,
        &n,
        &k,
        &l,
        v.as_ptr() as *const _,
        &ldv,
        t.as_ptr() as *const _,
        &ldt,
        a.as_mut_ptr() as *mut _,
        &lda,
        b.as_mut_ptr() as *mut _,
        &ldb,
        work.as_mut_ptr() as *mut _,
        &ldwork,
    )
}

#[inline]
pub unsafe fn ctprfs(
    uplo: u8,
    trans: u8,
    diag: u8,
    n: i32,
    nrhs: i32,
    ap: &[c32],
    b: &[c32],
    ldb: i32,
    x: &[c32],
    ldx: i32,
    ferr: &mut [f32],
    berr: &mut [f32],
    work: &mut [c32],
    rwork: &mut [f32],
    info: &mut i32,
) {
    ffi::ctprfs_(
        &(uplo as c_char),
        &(trans as c_char),
        &(diag as c_char),
        &n,
        &nrhs,
        ap.as_ptr() as *const _,
        b.as_ptr() as *const _,
        &ldb,
        x.as_ptr() as *const _,
        &ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn dtprfs(
    uplo: u8,
    trans: u8,
    diag: u8,
    n: i32,
    nrhs: i32,
    ap: &[f64],
    b: &[f64],
    ldb: i32,
    x: &[f64],
    ldx: i32,
    ferr: &mut [f64],
    berr: &mut [f64],
    work: &mut [f64],
    iwork: &mut [i32],
    info: &mut i32,
) {
    ffi::dtprfs_(
        &(uplo as c_char),
        &(trans as c_char),
        &(diag as c_char),
        &n,
        &nrhs,
        ap.as_ptr(),
        b.as_ptr(),
        &ldb,
        x.as_ptr(),
        &ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn stprfs(
    uplo: u8,
    trans: u8,
    diag: u8,
    n: i32,
    nrhs: i32,
    ap: &[f32],
    b: &[f32],
    ldb: i32,
    x: &[f32],
    ldx: i32,
    ferr: &mut [f32],
    berr: &mut [f32],
    work: &mut [f32],
    iwork: &mut [i32],
    info: &mut i32,
) {
    ffi::stprfs_(
        &(uplo as c_char),
        &(trans as c_char),
        &(diag as c_char),
        &n,
        &nrhs,
        ap.as_ptr(),
        b.as_ptr(),
        &ldb,
        x.as_ptr(),
        &ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn ztprfs(
    uplo: u8,
    trans: u8,
    diag: u8,
    n: i32,
    nrhs: i32,
    ap: &[c64],
    b: &[c64],
    ldb: i32,
    x: &[c64],
    ldx: i32,
    ferr: &mut [f64],
    berr: &mut [f64],
    work: &mut [c64],
    rwork: &mut [f64],
    info: &mut i32,
) {
    ffi::ztprfs_(
        &(uplo as c_char),
        &(trans as c_char),
        &(diag as c_char),
        &n,
        &nrhs,
        ap.as_ptr() as *const _,
        b.as_ptr() as *const _,
        &ldb,
        x.as_ptr() as *const _,
        &ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn ctptri(uplo: u8, diag: u8, n: i32, ap: &mut [c32], info: &mut i32) {
    ffi::ctptri_(
        &(uplo as c_char),
        &(diag as c_char),
        &n,
        ap.as_mut_ptr() as *mut _,
        info,
    )
}

#[inline]
pub unsafe fn dtptri(uplo: u8, diag: u8, n: i32, ap: &mut [f64], info: &mut i32) {
    ffi::dtptri_(
        &(uplo as c_char),
        &(diag as c_char),
        &n,
        ap.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn stptri(uplo: u8, diag: u8, n: i32, ap: &mut [f32], info: &mut i32) {
    ffi::stptri_(
        &(uplo as c_char),
        &(diag as c_char),
        &n,
        ap.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn ztptri(uplo: u8, diag: u8, n: i32, ap: &mut [c64], info: &mut i32) {
    ffi::ztptri_(
        &(uplo as c_char),
        &(diag as c_char),
        &n,
        ap.as_mut_ptr() as *mut _,
        info,
    )
}

#[inline]
pub unsafe fn ctptrs(
    uplo: u8,
    trans: u8,
    diag: u8,
    n: i32,
    nrhs: i32,
    ap: &[c32],
    b: &mut [c32],
    ldb: i32,
    info: &mut i32,
) {
    ffi::ctptrs_(
        &(uplo as c_char),
        &(trans as c_char),
        &(diag as c_char),
        &n,
        &nrhs,
        ap.as_ptr() as *const _,
        b.as_mut_ptr() as *mut _,
        &ldb,
        info,
    )
}

#[inline]
pub unsafe fn dtptrs(
    uplo: u8,
    trans: u8,
    diag: u8,
    n: i32,
    nrhs: i32,
    ap: &[f64],
    b: &mut [f64],
    ldb: i32,
    info: &mut i32,
) {
    ffi::dtptrs_(
        &(uplo as c_char),
        &(trans as c_char),
        &(diag as c_char),
        &n,
        &nrhs,
        ap.as_ptr(),
        b.as_mut_ptr(),
        &ldb,
        info,
    )
}

#[inline]
pub unsafe fn stptrs(
    uplo: u8,
    trans: u8,
    diag: u8,
    n: i32,
    nrhs: i32,
    ap: &[f32],
    b: &mut [f32],
    ldb: i32,
    info: &mut i32,
) {
    ffi::stptrs_(
        &(uplo as c_char),
        &(trans as c_char),
        &(diag as c_char),
        &n,
        &nrhs,
        ap.as_ptr(),
        b.as_mut_ptr(),
        &ldb,
        info,
    )
}

#[inline]
pub unsafe fn ztptrs(
    uplo: u8,
    trans: u8,
    diag: u8,
    n: i32,
    nrhs: i32,
    ap: &[c64],
    b: &mut [c64],
    ldb: i32,
    info: &mut i32,
) {
    ffi::ztptrs_(
        &(uplo as c_char),
        &(trans as c_char),
        &(diag as c_char),
        &n,
        &nrhs,
        ap.as_ptr() as *const _,
        b.as_mut_ptr() as *mut _,
        &ldb,
        info,
    )
}

#[inline]
pub unsafe fn ctpttf(transr: u8, uplo: u8, n: i32, ap: &[c32], arf: &mut [c32], info: &mut i32) {
    ffi::ctpttf_(
        &(transr as c_char),
        &(uplo as c_char),
        &n,
        ap.as_ptr() as *const _,
        arf.as_mut_ptr() as *mut _,
        info,
    )
}

#[inline]
pub unsafe fn dtpttf(transr: u8, uplo: u8, n: i32, ap: &[f64], arf: &mut [f64], info: &mut i32) {
    ffi::dtpttf_(
        &(transr as c_char),
        &(uplo as c_char),
        &n,
        ap.as_ptr(),
        arf.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn stpttf(transr: u8, uplo: u8, n: i32, ap: &[f32], arf: &mut [f32], info: &mut i32) {
    ffi::stpttf_(
        &(transr as c_char),
        &(uplo as c_char),
        &n,
        ap.as_ptr(),
        arf.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn ztpttf(transr: u8, uplo: u8, n: i32, ap: &[c64], arf: &mut [c64], info: &mut i32) {
    ffi::ztpttf_(
        &(transr as c_char),
        &(uplo as c_char),
        &n,
        ap.as_ptr() as *const _,
        arf.as_mut_ptr() as *mut _,
        info,
    )
}

#[inline]
pub unsafe fn ctpttr(uplo: u8, n: i32, ap: &[c32], a: &mut [c32], lda: i32, info: &mut i32) {
    ffi::ctpttr_(
        &(uplo as c_char),
        &n,
        ap.as_ptr() as *const _,
        a.as_mut_ptr() as *mut _,
        &lda,
        info,
    )
}

#[inline]
pub unsafe fn dtpttr(uplo: u8, n: i32, ap: &[f64], a: &mut [f64], lda: i32, info: &mut i32) {
    ffi::dtpttr_(
        &(uplo as c_char),
        &n,
        ap.as_ptr(),
        a.as_mut_ptr(),
        &lda,
        info,
    )
}

#[inline]
pub unsafe fn stpttr(uplo: u8, n: i32, ap: &[f32], a: &mut [f32], lda: i32, info: &mut i32) {
    ffi::stpttr_(
        &(uplo as c_char),
        &n,
        ap.as_ptr(),
        a.as_mut_ptr(),
        &lda,
        info,
    )
}

#[inline]
pub unsafe fn ztpttr(uplo: u8, n: i32, ap: &[c64], a: &mut [c64], lda: i32, info: &mut i32) {
    ffi::ztpttr_(
        &(uplo as c_char),
        &n,
        ap.as_ptr() as *const _,
        a.as_mut_ptr() as *mut _,
        &lda,
        info,
    )
}

#[inline]
pub unsafe fn ctrcon(
    norm: u8,
    uplo: u8,
    diag: u8,
    n: i32,
    a: &[c32],
    lda: i32,
    rcond: &mut f32,
    work: &mut [c32],
    rwork: &mut [f32],
    info: &mut i32,
) {
    ffi::ctrcon_(
        &(norm as c_char),
        &(uplo as c_char),
        &(diag as c_char),
        &n,
        a.as_ptr() as *const _,
        &lda,
        rcond,
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn dtrcon(
    norm: u8,
    uplo: u8,
    diag: u8,
    n: i32,
    a: &[f64],
    lda: i32,
    rcond: &mut f64,
    work: &mut [f64],
    iwork: &mut [i32],
    info: &mut i32,
) {
    ffi::dtrcon_(
        &(norm as c_char),
        &(uplo as c_char),
        &(diag as c_char),
        &n,
        a.as_ptr(),
        &lda,
        rcond,
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn strcon(
    norm: u8,
    uplo: u8,
    diag: u8,
    n: i32,
    a: &[f32],
    lda: i32,
    rcond: &mut f32,
    work: &mut [f32],
    iwork: &mut [i32],
    info: &mut i32,
) {
    ffi::strcon_(
        &(norm as c_char),
        &(uplo as c_char),
        &(diag as c_char),
        &n,
        a.as_ptr(),
        &lda,
        rcond,
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn ztrcon(
    norm: u8,
    uplo: u8,
    diag: u8,
    n: i32,
    a: &[c64],
    lda: i32,
    rcond: &mut f64,
    work: &mut [c64],
    rwork: &mut [f64],
    info: &mut i32,
) {
    ffi::ztrcon_(
        &(norm as c_char),
        &(uplo as c_char),
        &(diag as c_char),
        &n,
        a.as_ptr() as *const _,
        &lda,
        rcond,
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn ctrevc(
    side: u8,
    howmny: u8,
    select: &[i32],
    n: i32,
    t: &mut [c32],
    ldt: i32,
    vl: &mut [c32],
    ldvl: i32,
    vr: &mut [c32],
    ldvr: i32,
    mm: i32,
    m: &mut i32,
    work: &mut [c32],
    rwork: &mut [f32],
    info: &mut i32,
) {
    ffi::ctrevc_(
        &(side as c_char),
        &(howmny as c_char),
        select.as_ptr(),
        &n,
        t.as_mut_ptr() as *mut _,
        &ldt,
        vl.as_mut_ptr() as *mut _,
        &ldvl,
        vr.as_mut_ptr() as *mut _,
        &ldvr,
        &mm,
        m,
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn dtrevc(
    side: u8,
    howmny: u8,
    select: &mut [i32],
    n: i32,
    t: &[f64],
    ldt: i32,
    vl: &mut [f64],
    ldvl: i32,
    vr: &mut [f64],
    ldvr: i32,
    mm: i32,
    m: &mut i32,
    work: &mut [f64],
    info: &mut i32,
) {
    ffi::dtrevc_(
        &(side as c_char),
        &(howmny as c_char),
        select.as_mut_ptr(),
        &n,
        t.as_ptr(),
        &ldt,
        vl.as_mut_ptr(),
        &ldvl,
        vr.as_mut_ptr(),
        &ldvr,
        &mm,
        m,
        work.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn strevc(
    side: u8,
    howmny: u8,
    select: &mut [i32],
    n: i32,
    t: &[f32],
    ldt: i32,
    vl: &mut [f32],
    ldvl: i32,
    vr: &mut [f32],
    ldvr: i32,
    mm: i32,
    m: &mut i32,
    work: &mut [f32],
    info: &mut i32,
) {
    ffi::strevc_(
        &(side as c_char),
        &(howmny as c_char),
        select.as_mut_ptr(),
        &n,
        t.as_ptr(),
        &ldt,
        vl.as_mut_ptr(),
        &ldvl,
        vr.as_mut_ptr(),
        &ldvr,
        &mm,
        m,
        work.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn ztrevc(
    side: u8,
    howmny: u8,
    select: &[i32],
    n: i32,
    t: &mut [c64],
    ldt: i32,
    vl: &mut [c64],
    ldvl: i32,
    vr: &mut [c64],
    ldvr: i32,
    mm: i32,
    m: &mut i32,
    work: &mut [c64],
    rwork: &mut [f64],
    info: &mut i32,
) {
    ffi::ztrevc_(
        &(side as c_char),
        &(howmny as c_char),
        select.as_ptr(),
        &n,
        t.as_mut_ptr() as *mut _,
        &ldt,
        vl.as_mut_ptr() as *mut _,
        &ldvl,
        vr.as_mut_ptr() as *mut _,
        &ldvr,
        &mm,
        m,
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn ctrevc3(
    side: u8,
    howmny: u8,
    select: &[i32],
    n: i32,
    t: &mut [c32],
    ldt: i32,
    vl: &mut [c32],
    ldvl: i32,
    vr: &mut [c32],
    ldvr: i32,
    mm: i32,
    m: &mut i32,
    work: &mut [c32],
    lwork: i32,
    rwork: &mut [f32],
    lrwork: i32,
    info: &mut i32,
) {
    ffi::ctrevc3_(
        &(side as c_char),
        &(howmny as c_char),
        select.as_ptr(),
        &n,
        t.as_mut_ptr() as *mut _,
        &ldt,
        vl.as_mut_ptr() as *mut _,
        &ldvl,
        vr.as_mut_ptr() as *mut _,
        &ldvr,
        &mm,
        m,
        work.as_mut_ptr() as *mut _,
        &lwork,
        rwork.as_mut_ptr(),
        &lrwork,
        info,
    )
}

#[inline]
pub unsafe fn dtrevc3(
    side: u8,
    howmny: u8,
    select: &mut [i32],
    n: i32,
    t: &[f64],
    ldt: i32,
    vl: &mut [f64],
    ldvl: i32,
    vr: &mut [f64],
    ldvr: i32,
    mm: i32,
    m: &mut i32,
    work: &mut [f64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::dtrevc3_(
        &(side as c_char),
        &(howmny as c_char),
        select.as_mut_ptr(),
        &n,
        t.as_ptr(),
        &ldt,
        vl.as_mut_ptr(),
        &ldvl,
        vr.as_mut_ptr(),
        &ldvr,
        &mm,
        m,
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn strevc3(
    side: u8,
    howmny: u8,
    select: &mut [i32],
    n: i32,
    t: &[f32],
    ldt: i32,
    vl: &mut [f32],
    ldvl: i32,
    vr: &mut [f32],
    ldvr: i32,
    mm: i32,
    m: &mut i32,
    work: &mut [f32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::strevc3_(
        &(side as c_char),
        &(howmny as c_char),
        select.as_mut_ptr(),
        &n,
        t.as_ptr(),
        &ldt,
        vl.as_mut_ptr(),
        &ldvl,
        vr.as_mut_ptr(),
        &ldvr,
        &mm,
        m,
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn ztrevc3(
    side: u8,
    howmny: u8,
    select: &[i32],
    n: i32,
    t: &mut [c64],
    ldt: i32,
    vl: &mut [c64],
    ldvl: i32,
    vr: &mut [c64],
    ldvr: i32,
    mm: i32,
    m: &mut i32,
    work: &mut [c64],
    lwork: i32,
    rwork: &mut [f64],
    lrwork: i32,
    info: &mut i32,
) {
    ffi::ztrevc3_(
        &(side as c_char),
        &(howmny as c_char),
        select.as_ptr(),
        &n,
        t.as_mut_ptr() as *mut _,
        &ldt,
        vl.as_mut_ptr() as *mut _,
        &ldvl,
        vr.as_mut_ptr() as *mut _,
        &ldvr,
        &mm,
        m,
        work.as_mut_ptr() as *mut _,
        &lwork,
        rwork.as_mut_ptr(),
        &lrwork,
        info,
    )
}

#[inline]
pub unsafe fn ctrexc(
    compq: u8,
    n: i32,
    t: &mut [c32],
    ldt: i32,
    q: &mut [c32],
    ldq: i32,
    ifst: &[i32],
    ilst: &[i32],
    info: &mut i32,
) {
    ffi::ctrexc_(
        &(compq as c_char),
        &n,
        t.as_mut_ptr() as *mut _,
        &ldt,
        q.as_mut_ptr() as *mut _,
        &ldq,
        ifst.as_ptr(),
        ilst.as_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn dtrexc(
    compq: u8,
    n: i32,
    t: &mut [f64],
    ldt: i32,
    q: &mut [f64],
    ldq: i32,
    ifst: &mut [i32],
    ilst: &mut [i32],
    work: &mut [f64],
    info: &mut i32,
) {
    ffi::dtrexc_(
        &(compq as c_char),
        &n,
        t.as_mut_ptr(),
        &ldt,
        q.as_mut_ptr(),
        &ldq,
        ifst.as_mut_ptr(),
        ilst.as_mut_ptr(),
        work.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn strexc(
    compq: u8,
    n: i32,
    t: &mut [f32],
    ldt: i32,
    q: &mut [f32],
    ldq: i32,
    ifst: &mut [i32],
    ilst: &mut [i32],
    work: &mut [f32],
    info: &mut i32,
) {
    ffi::strexc_(
        &(compq as c_char),
        &n,
        t.as_mut_ptr(),
        &ldt,
        q.as_mut_ptr(),
        &ldq,
        ifst.as_mut_ptr(),
        ilst.as_mut_ptr(),
        work.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn ztrexc(
    compq: u8,
    n: i32,
    t: &mut [c64],
    ldt: i32,
    q: &mut [c64],
    ldq: i32,
    ifst: &[i32],
    ilst: &[i32],
    info: &mut i32,
) {
    ffi::ztrexc_(
        &(compq as c_char),
        &n,
        t.as_mut_ptr() as *mut _,
        &ldt,
        q.as_mut_ptr() as *mut _,
        &ldq,
        ifst.as_ptr(),
        ilst.as_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn ctrrfs(
    uplo: u8,
    trans: u8,
    diag: u8,
    n: i32,
    nrhs: i32,
    a: &[c32],
    lda: i32,
    b: &[c32],
    ldb: i32,
    x: &[c32],
    ldx: i32,
    ferr: &mut [f32],
    berr: &mut [f32],
    work: &mut [c32],
    rwork: &mut [f32],
    info: &mut i32,
) {
    ffi::ctrrfs_(
        &(uplo as c_char),
        &(trans as c_char),
        &(diag as c_char),
        &n,
        &nrhs,
        a.as_ptr() as *const _,
        &lda,
        b.as_ptr() as *const _,
        &ldb,
        x.as_ptr() as *const _,
        &ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn dtrrfs(
    uplo: u8,
    trans: u8,
    diag: u8,
    n: i32,
    nrhs: i32,
    a: &[f64],
    lda: i32,
    b: &[f64],
    ldb: i32,
    x: &[f64],
    ldx: i32,
    ferr: &mut [f64],
    berr: &mut [f64],
    work: &mut [f64],
    iwork: &mut [i32],
    info: &mut i32,
) {
    ffi::dtrrfs_(
        &(uplo as c_char),
        &(trans as c_char),
        &(diag as c_char),
        &n,
        &nrhs,
        a.as_ptr(),
        &lda,
        b.as_ptr(),
        &ldb,
        x.as_ptr(),
        &ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn strrfs(
    uplo: u8,
    trans: u8,
    diag: u8,
    n: i32,
    nrhs: i32,
    a: &[f32],
    lda: i32,
    b: &[f32],
    ldb: i32,
    x: &[f32],
    ldx: i32,
    ferr: &mut [f32],
    berr: &mut [f32],
    work: &mut [f32],
    iwork: &mut [i32],
    info: &mut i32,
) {
    ffi::strrfs_(
        &(uplo as c_char),
        &(trans as c_char),
        &(diag as c_char),
        &n,
        &nrhs,
        a.as_ptr(),
        &lda,
        b.as_ptr(),
        &ldb,
        x.as_ptr(),
        &ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn ztrrfs(
    uplo: u8,
    trans: u8,
    diag: u8,
    n: i32,
    nrhs: i32,
    a: &[c64],
    lda: i32,
    b: &[c64],
    ldb: i32,
    x: &[c64],
    ldx: i32,
    ferr: &mut [f64],
    berr: &mut [f64],
    work: &mut [c64],
    rwork: &mut [f64],
    info: &mut i32,
) {
    ffi::ztrrfs_(
        &(uplo as c_char),
        &(trans as c_char),
        &(diag as c_char),
        &n,
        &nrhs,
        a.as_ptr() as *const _,
        &lda,
        b.as_ptr() as *const _,
        &ldb,
        x.as_ptr() as *const _,
        &ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn ctrsen(
    job: u8,
    compq: u8,
    select: &[i32],
    n: i32,
    t: &mut [c32],
    ldt: i32,
    q: &mut [c32],
    ldq: i32,
    w: &mut [c32],
    m: &mut i32,
    s: &mut [f32],
    sep: &mut [f32],
    work: &mut [c32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::ctrsen_(
        &(job as c_char),
        &(compq as c_char),
        select.as_ptr(),
        &n,
        t.as_mut_ptr() as *mut _,
        &ldt,
        q.as_mut_ptr() as *mut _,
        &ldq,
        w.as_mut_ptr() as *mut _,
        m,
        s.as_mut_ptr(),
        sep.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn dtrsen(
    job: u8,
    compq: u8,
    select: &[i32],
    n: i32,
    t: &mut [f64],
    ldt: i32,
    q: &mut [f64],
    ldq: i32,
    wr: &mut [f64],
    wi: &mut [f64],
    m: &mut i32,
    s: &mut [f64],
    sep: &mut [f64],
    work: &mut [f64],
    lwork: i32,
    iwork: &mut [i32],
    liwork: i32,
    info: &mut i32,
) {
    ffi::dtrsen_(
        &(job as c_char),
        &(compq as c_char),
        select.as_ptr(),
        &n,
        t.as_mut_ptr(),
        &ldt,
        q.as_mut_ptr(),
        &ldq,
        wr.as_mut_ptr(),
        wi.as_mut_ptr(),
        m,
        s.as_mut_ptr(),
        sep.as_mut_ptr(),
        work.as_mut_ptr(),
        &lwork,
        iwork.as_mut_ptr(),
        &liwork,
        info,
    )
}

#[inline]
pub unsafe fn strsen(
    job: u8,
    compq: u8,
    select: &[i32],
    n: i32,
    t: &mut [f32],
    ldt: i32,
    q: &mut [f32],
    ldq: i32,
    wr: &mut [f32],
    wi: &mut [f32],
    m: &mut i32,
    s: &mut [f32],
    sep: &mut [f32],
    work: &mut [f32],
    lwork: i32,
    iwork: &mut [i32],
    liwork: i32,
    info: &mut i32,
) {
    ffi::strsen_(
        &(job as c_char),
        &(compq as c_char),
        select.as_ptr(),
        &n,
        t.as_mut_ptr(),
        &ldt,
        q.as_mut_ptr(),
        &ldq,
        wr.as_mut_ptr(),
        wi.as_mut_ptr(),
        m,
        s.as_mut_ptr(),
        sep.as_mut_ptr(),
        work.as_mut_ptr(),
        &lwork,
        iwork.as_mut_ptr(),
        &liwork,
        info,
    )
}

#[inline]
pub unsafe fn ztrsen(
    job: u8,
    compq: u8,
    select: &[i32],
    n: i32,
    t: &mut [c64],
    ldt: i32,
    q: &mut [c64],
    ldq: i32,
    w: &mut [c64],
    m: &mut i32,
    s: &mut [f64],
    sep: &mut [f64],
    work: &mut [c64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::ztrsen_(
        &(job as c_char),
        &(compq as c_char),
        select.as_ptr(),
        &n,
        t.as_mut_ptr() as *mut _,
        &ldt,
        q.as_mut_ptr() as *mut _,
        &ldq,
        w.as_mut_ptr() as *mut _,
        m,
        s.as_mut_ptr(),
        sep.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn ctrsna(
    job: u8,
    howmny: u8,
    select: &[i32],
    n: i32,
    t: &[c32],
    ldt: i32,
    vl: &[c32],
    ldvl: i32,
    vr: &[c32],
    ldvr: i32,
    s: &mut [f32],
    sep: &mut [f32],
    mm: i32,
    m: &mut i32,
    work: &mut [c32],
    ldwork: i32,
    rwork: &mut [f32],
    info: &mut i32,
) {
    ffi::ctrsna_(
        &(job as c_char),
        &(howmny as c_char),
        select.as_ptr(),
        &n,
        t.as_ptr() as *const _,
        &ldt,
        vl.as_ptr() as *const _,
        &ldvl,
        vr.as_ptr() as *const _,
        &ldvr,
        s.as_mut_ptr(),
        sep.as_mut_ptr(),
        &mm,
        m,
        work.as_mut_ptr() as *mut _,
        &ldwork,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn dtrsna(
    job: u8,
    howmny: u8,
    select: &[i32],
    n: i32,
    t: &[f64],
    ldt: i32,
    vl: &[f64],
    ldvl: i32,
    vr: &[f64],
    ldvr: i32,
    s: &mut [f64],
    sep: &mut [f64],
    mm: i32,
    m: &mut i32,
    work: &mut [f64],
    ldwork: i32,
    iwork: &mut [i32],
    info: &mut i32,
) {
    ffi::dtrsna_(
        &(job as c_char),
        &(howmny as c_char),
        select.as_ptr(),
        &n,
        t.as_ptr(),
        &ldt,
        vl.as_ptr(),
        &ldvl,
        vr.as_ptr(),
        &ldvr,
        s.as_mut_ptr(),
        sep.as_mut_ptr(),
        &mm,
        m,
        work.as_mut_ptr(),
        &ldwork,
        iwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn strsna(
    job: u8,
    howmny: u8,
    select: &[i32],
    n: i32,
    t: &[f32],
    ldt: i32,
    vl: &[f32],
    ldvl: i32,
    vr: &[f32],
    ldvr: i32,
    s: &mut [f32],
    sep: &mut [f32],
    mm: i32,
    m: &mut i32,
    work: &mut [f32],
    ldwork: i32,
    iwork: &mut [i32],
    info: &mut i32,
) {
    ffi::strsna_(
        &(job as c_char),
        &(howmny as c_char),
        select.as_ptr(),
        &n,
        t.as_ptr(),
        &ldt,
        vl.as_ptr(),
        &ldvl,
        vr.as_ptr(),
        &ldvr,
        s.as_mut_ptr(),
        sep.as_mut_ptr(),
        &mm,
        m,
        work.as_mut_ptr(),
        &ldwork,
        iwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn ztrsna(
    job: u8,
    howmny: u8,
    select: &[i32],
    n: i32,
    t: &[c64],
    ldt: i32,
    vl: &[c64],
    ldvl: i32,
    vr: &[c64],
    ldvr: i32,
    s: &mut [f64],
    sep: &mut [f64],
    mm: i32,
    m: &mut i32,
    work: &mut [c64],
    ldwork: i32,
    rwork: &mut [f64],
    info: &mut i32,
) {
    ffi::ztrsna_(
        &(job as c_char),
        &(howmny as c_char),
        select.as_ptr(),
        &n,
        t.as_ptr() as *const _,
        &ldt,
        vl.as_ptr() as *const _,
        &ldvl,
        vr.as_ptr() as *const _,
        &ldvr,
        s.as_mut_ptr(),
        sep.as_mut_ptr(),
        &mm,
        m,
        work.as_mut_ptr() as *mut _,
        &ldwork,
        rwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn ctrsyl(
    trana: u8,
    tranb: u8,
    isgn: &[i32],
    m: i32,
    n: i32,
    a: &[c32],
    lda: i32,
    b: &[c32],
    ldb: i32,
    c: &mut [c32],
    ldc: i32,
    scale: &mut [f32],
    info: &mut i32,
) {
    ffi::ctrsyl_(
        &(trana as c_char),
        &(tranb as c_char),
        isgn.as_ptr(),
        &m,
        &n,
        a.as_ptr() as *const _,
        &lda,
        b.as_ptr() as *const _,
        &ldb,
        c.as_mut_ptr() as *mut _,
        &ldc,
        scale.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn dtrsyl(
    trana: u8,
    tranb: u8,
    isgn: &[i32],
    m: i32,
    n: i32,
    a: &[f64],
    lda: i32,
    b: &[f64],
    ldb: i32,
    c: &mut [f64],
    ldc: i32,
    scale: &mut [f64],
    info: &mut i32,
) {
    ffi::dtrsyl_(
        &(trana as c_char),
        &(tranb as c_char),
        isgn.as_ptr(),
        &m,
        &n,
        a.as_ptr(),
        &lda,
        b.as_ptr(),
        &ldb,
        c.as_mut_ptr(),
        &ldc,
        scale.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn strsyl(
    trana: u8,
    tranb: u8,
    isgn: &[i32],
    m: i32,
    n: i32,
    a: &[f32],
    lda: i32,
    b: &[f32],
    ldb: i32,
    c: &mut [f32],
    ldc: i32,
    scale: &mut [f32],
    info: &mut i32,
) {
    ffi::strsyl_(
        &(trana as c_char),
        &(tranb as c_char),
        isgn.as_ptr(),
        &m,
        &n,
        a.as_ptr(),
        &lda,
        b.as_ptr(),
        &ldb,
        c.as_mut_ptr(),
        &ldc,
        scale.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn ztrsyl(
    trana: u8,
    tranb: u8,
    isgn: &[i32],
    m: i32,
    n: i32,
    a: &[c64],
    lda: i32,
    b: &[c64],
    ldb: i32,
    c: &mut [c64],
    ldc: i32,
    scale: &mut [f64],
    info: &mut i32,
) {
    ffi::ztrsyl_(
        &(trana as c_char),
        &(tranb as c_char),
        isgn.as_ptr(),
        &m,
        &n,
        a.as_ptr() as *const _,
        &lda,
        b.as_ptr() as *const _,
        &ldb,
        c.as_mut_ptr() as *mut _,
        &ldc,
        scale.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn ctrtri(uplo: u8, diag: u8, n: i32, a: &mut [c32], lda: i32, info: &mut i32) {
    ffi::ctrtri_(
        &(uplo as c_char),
        &(diag as c_char),
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        info,
    )
}

#[inline]
pub unsafe fn dtrtri(uplo: u8, diag: u8, n: i32, a: &mut [f64], lda: i32, info: &mut i32) {
    ffi::dtrtri_(
        &(uplo as c_char),
        &(diag as c_char),
        &n,
        a.as_mut_ptr(),
        &lda,
        info,
    )
}

#[inline]
pub unsafe fn strtri(uplo: u8, diag: u8, n: i32, a: &mut [f32], lda: i32, info: &mut i32) {
    ffi::strtri_(
        &(uplo as c_char),
        &(diag as c_char),
        &n,
        a.as_mut_ptr(),
        &lda,
        info,
    )
}

#[inline]
pub unsafe fn ztrtri(uplo: u8, diag: u8, n: i32, a: &mut [c64], lda: i32, info: &mut i32) {
    ffi::ztrtri_(
        &(uplo as c_char),
        &(diag as c_char),
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        info,
    )
}

#[inline]
pub unsafe fn ctrtrs(
    uplo: u8,
    trans: u8,
    diag: u8,
    n: i32,
    nrhs: i32,
    a: &[c32],
    lda: i32,
    b: &mut [c32],
    ldb: i32,
    info: &mut i32,
) {
    ffi::ctrtrs_(
        &(uplo as c_char),
        &(trans as c_char),
        &(diag as c_char),
        &n,
        &nrhs,
        a.as_ptr() as *const _,
        &lda,
        b.as_mut_ptr() as *mut _,
        &ldb,
        info,
    )
}

#[inline]
pub unsafe fn dtrtrs(
    uplo: u8,
    trans: u8,
    diag: u8,
    n: i32,
    nrhs: i32,
    a: &[f64],
    lda: i32,
    b: &mut [f64],
    ldb: i32,
    info: &mut i32,
) {
    ffi::dtrtrs_(
        &(uplo as c_char),
        &(trans as c_char),
        &(diag as c_char),
        &n,
        &nrhs,
        a.as_ptr(),
        &lda,
        b.as_mut_ptr(),
        &ldb,
        info,
    )
}

#[inline]
pub unsafe fn strtrs(
    uplo: u8,
    trans: u8,
    diag: u8,
    n: i32,
    nrhs: i32,
    a: &[f32],
    lda: i32,
    b: &mut [f32],
    ldb: i32,
    info: &mut i32,
) {
    ffi::strtrs_(
        &(uplo as c_char),
        &(trans as c_char),
        &(diag as c_char),
        &n,
        &nrhs,
        a.as_ptr(),
        &lda,
        b.as_mut_ptr(),
        &ldb,
        info,
    )
}

#[inline]
pub unsafe fn ztrtrs(
    uplo: u8,
    trans: u8,
    diag: u8,
    n: i32,
    nrhs: i32,
    a: &[c64],
    lda: i32,
    b: &mut [c64],
    ldb: i32,
    info: &mut i32,
) {
    ffi::ztrtrs_(
        &(uplo as c_char),
        &(trans as c_char),
        &(diag as c_char),
        &n,
        &nrhs,
        a.as_ptr() as *const _,
        &lda,
        b.as_mut_ptr() as *mut _,
        &ldb,
        info,
    )
}

#[inline]
pub unsafe fn ctrttf(
    transr: u8,
    uplo: u8,
    n: i32,
    a: &[c32],
    lda: i32,
    arf: &mut [c32],
    info: &mut i32,
) {
    ffi::ctrttf_(
        &(transr as c_char),
        &(uplo as c_char),
        &n,
        a.as_ptr() as *const _,
        &lda,
        arf.as_mut_ptr() as *mut _,
        info,
    )
}

#[inline]
pub unsafe fn dtrttf(
    transr: u8,
    uplo: u8,
    n: i32,
    a: &[f64],
    lda: i32,
    arf: &mut [f64],
    info: &mut i32,
) {
    ffi::dtrttf_(
        &(transr as c_char),
        &(uplo as c_char),
        &n,
        a.as_ptr(),
        &lda,
        arf.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn strttf(
    transr: u8,
    uplo: u8,
    n: i32,
    a: &[f32],
    lda: i32,
    arf: &mut [f32],
    info: &mut i32,
) {
    ffi::strttf_(
        &(transr as c_char),
        &(uplo as c_char),
        &n,
        a.as_ptr(),
        &lda,
        arf.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn ztrttf(
    transr: u8,
    uplo: u8,
    n: i32,
    a: &[c64],
    lda: i32,
    arf: &mut [c64],
    info: &mut i32,
) {
    ffi::ztrttf_(
        &(transr as c_char),
        &(uplo as c_char),
        &n,
        a.as_ptr() as *const _,
        &lda,
        arf.as_mut_ptr() as *mut _,
        info,
    )
}

#[inline]
pub unsafe fn ctrttp(uplo: u8, n: i32, a: &[c32], lda: i32, ap: &mut [c32], info: &mut i32) {
    ffi::ctrttp_(
        &(uplo as c_char),
        &n,
        a.as_ptr() as *const _,
        &lda,
        ap.as_mut_ptr() as *mut _,
        info,
    )
}

#[inline]
pub unsafe fn dtrttp(uplo: u8, n: i32, a: &[f64], lda: i32, ap: &mut [f64], info: &mut i32) {
    ffi::dtrttp_(
        &(uplo as c_char),
        &n,
        a.as_ptr(),
        &lda,
        ap.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn strttp(uplo: u8, n: i32, a: &[f32], lda: i32, ap: &mut [f32], info: &mut i32) {
    ffi::strttp_(
        &(uplo as c_char),
        &n,
        a.as_ptr(),
        &lda,
        ap.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn ztrttp(uplo: u8, n: i32, a: &[c64], lda: i32, ap: &mut [c64], info: &mut i32) {
    ffi::ztrttp_(
        &(uplo as c_char),
        &n,
        a.as_ptr() as *const _,
        &lda,
        ap.as_mut_ptr() as *mut _,
        info,
    )
}

#[inline]
pub unsafe fn ctzrzf(
    m: i32,
    n: i32,
    a: &mut [c32],
    lda: i32,
    tau: &mut [c32],
    work: &mut [c32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::ctzrzf_(
        &m,
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        tau.as_mut_ptr() as *mut _,
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn dtzrzf(
    m: i32,
    n: i32,
    a: &mut [f64],
    lda: i32,
    tau: &mut [f64],
    work: &mut [f64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::dtzrzf_(
        &m,
        &n,
        a.as_mut_ptr(),
        &lda,
        tau.as_mut_ptr(),
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn stzrzf(
    m: i32,
    n: i32,
    a: &mut [f32],
    lda: i32,
    tau: &mut [f32],
    work: &mut [f32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::stzrzf_(
        &m,
        &n,
        a.as_mut_ptr(),
        &lda,
        tau.as_mut_ptr(),
        work.as_mut_ptr(),
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn ztzrzf(
    m: i32,
    n: i32,
    a: &mut [c64],
    lda: i32,
    tau: &mut [c64],
    work: &mut [c64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::ztzrzf_(
        &m,
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        tau.as_mut_ptr() as *mut _,
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn cunbdb(
    trans: u8,
    signs: u8,
    m: i32,
    p: i32,
    q: &[i32],
    x11: &mut [c32],
    ldx11: i32,
    x12: &mut [c32],
    ldx12: i32,
    x21: &mut [c32],
    ldx21: i32,
    x22: &mut [c32],
    ldx22: i32,
    theta: &mut [f32],
    phi: &mut [f32],
    taup1: &mut [c32],
    taup2: &mut [c32],
    tauq1: &mut [c32],
    tauq2: &mut [c32],
    work: &mut [c32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::cunbdb_(
        &(trans as c_char),
        &(signs as c_char),
        &m,
        &p,
        q.as_ptr(),
        x11.as_mut_ptr() as *mut _,
        &ldx11,
        x12.as_mut_ptr() as *mut _,
        &ldx12,
        x21.as_mut_ptr() as *mut _,
        &ldx21,
        x22.as_mut_ptr() as *mut _,
        &ldx22,
        theta.as_mut_ptr(),
        phi.as_mut_ptr(),
        taup1.as_mut_ptr() as *mut _,
        taup2.as_mut_ptr() as *mut _,
        tauq1.as_mut_ptr() as *mut _,
        tauq2.as_mut_ptr() as *mut _,
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn zunbdb(
    trans: u8,
    signs: u8,
    m: i32,
    p: i32,
    q: &[i32],
    x11: &mut [c64],
    ldx11: i32,
    x12: &mut [c64],
    ldx12: i32,
    x21: &mut [c64],
    ldx21: i32,
    x22: &mut [c64],
    ldx22: i32,
    theta: &mut [f64],
    phi: &mut [f64],
    taup1: &mut [c64],
    taup2: &mut [c64],
    tauq1: &mut [c64],
    tauq2: &mut [c64],
    work: &mut [c64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::zunbdb_(
        &(trans as c_char),
        &(signs as c_char),
        &m,
        &p,
        q.as_ptr(),
        x11.as_mut_ptr() as *mut _,
        &ldx11,
        x12.as_mut_ptr() as *mut _,
        &ldx12,
        x21.as_mut_ptr() as *mut _,
        &ldx21,
        x22.as_mut_ptr() as *mut _,
        &ldx22,
        theta.as_mut_ptr(),
        phi.as_mut_ptr(),
        taup1.as_mut_ptr() as *mut _,
        taup2.as_mut_ptr() as *mut _,
        tauq1.as_mut_ptr() as *mut _,
        tauq2.as_mut_ptr() as *mut _,
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn cuncsd(
    jobu1: u8,
    jobu2: u8,
    jobv1t: u8,
    jobv2t: u8,
    trans: u8,
    signs: u8,
    m: i32,
    p: i32,
    q: &[i32],
    x11: &mut [c32],
    ldx11: i32,
    x12: &mut [c32],
    ldx12: i32,
    x21: &mut [c32],
    ldx21: i32,
    x22: &mut [c32],
    ldx22: i32,
    theta: &mut [f32],
    u1: &mut [c32],
    ldu1: i32,
    u2: &mut [c32],
    ldu2: i32,
    v1t: &mut [c32],
    ldv1t: i32,
    v2t: &mut [c32],
    ldv2t: i32,
    work: &mut [c32],
    lwork: i32,
    rwork: &mut [f32],
    lrwork: i32,
    iwork: &mut [i32],
    info: &mut i32,
) {
    ffi::cuncsd_(
        &(jobu1 as c_char),
        &(jobu2 as c_char),
        &(jobv1t as c_char),
        &(jobv2t as c_char),
        &(trans as c_char),
        &(signs as c_char),
        &m,
        &p,
        q.as_ptr(),
        x11.as_mut_ptr() as *mut _,
        &ldx11,
        x12.as_mut_ptr() as *mut _,
        &ldx12,
        x21.as_mut_ptr() as *mut _,
        &ldx21,
        x22.as_mut_ptr() as *mut _,
        &ldx22,
        theta.as_mut_ptr(),
        u1.as_mut_ptr() as *mut _,
        &ldu1,
        u2.as_mut_ptr() as *mut _,
        &ldu2,
        v1t.as_mut_ptr() as *mut _,
        &ldv1t,
        v2t.as_mut_ptr() as *mut _,
        &ldv2t,
        work.as_mut_ptr() as *mut _,
        &lwork,
        rwork.as_mut_ptr(),
        &lrwork,
        iwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn zuncsd(
    jobu1: u8,
    jobu2: u8,
    jobv1t: u8,
    jobv2t: u8,
    trans: u8,
    signs: u8,
    m: i32,
    p: i32,
    q: &[i32],
    x11: &mut [c64],
    ldx11: i32,
    x12: &mut [c64],
    ldx12: i32,
    x21: &mut [c64],
    ldx21: i32,
    x22: &mut [c64],
    ldx22: i32,
    theta: &mut [f64],
    u1: &mut [c64],
    ldu1: i32,
    u2: &mut [c64],
    ldu2: i32,
    v1t: &mut [c64],
    ldv1t: i32,
    v2t: &mut [c64],
    ldv2t: i32,
    work: &mut [c64],
    lwork: i32,
    rwork: &mut [f64],
    lrwork: i32,
    iwork: &mut [i32],
    info: &mut i32,
) {
    ffi::zuncsd_(
        &(jobu1 as c_char),
        &(jobu2 as c_char),
        &(jobv1t as c_char),
        &(jobv2t as c_char),
        &(trans as c_char),
        &(signs as c_char),
        &m,
        &p,
        q.as_ptr(),
        x11.as_mut_ptr() as *mut _,
        &ldx11,
        x12.as_mut_ptr() as *mut _,
        &ldx12,
        x21.as_mut_ptr() as *mut _,
        &ldx21,
        x22.as_mut_ptr() as *mut _,
        &ldx22,
        theta.as_mut_ptr(),
        u1.as_mut_ptr() as *mut _,
        &ldu1,
        u2.as_mut_ptr() as *mut _,
        &ldu2,
        v1t.as_mut_ptr() as *mut _,
        &ldv1t,
        v2t.as_mut_ptr() as *mut _,
        &ldv2t,
        work.as_mut_ptr() as *mut _,
        &lwork,
        rwork.as_mut_ptr(),
        &lrwork,
        iwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn cuncsd2by1(
    jobu1: u8,
    jobu2: u8,
    jobv1t: u8,
    m: i32,
    p: i32,
    q: &[i32],
    x11: &mut [c32],
    ldx11: i32,
    x21: &mut [c32],
    ldx21: i32,
    theta: &mut [f32],
    u1: &mut [c32],
    ldu1: i32,
    u2: &mut [c32],
    ldu2: i32,
    v1t: &mut [c32],
    ldv1t: i32,
    work: &mut [c32],
    lwork: i32,
    rwork: &mut [f32],
    lrwork: i32,
    iwork: &mut [i32],
    info: &mut i32,
) {
    ffi::cuncsd2by1_(
        &(jobu1 as c_char),
        &(jobu2 as c_char),
        &(jobv1t as c_char),
        &m,
        &p,
        q.as_ptr(),
        x11.as_mut_ptr() as *mut _,
        &ldx11,
        x21.as_mut_ptr() as *mut _,
        &ldx21,
        theta.as_mut_ptr(),
        u1.as_mut_ptr() as *mut _,
        &ldu1,
        u2.as_mut_ptr() as *mut _,
        &ldu2,
        v1t.as_mut_ptr() as *mut _,
        &ldv1t,
        work.as_mut_ptr() as *mut _,
        &lwork,
        rwork.as_mut_ptr(),
        &lrwork,
        iwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn zuncsd2by1(
    jobu1: u8,
    jobu2: u8,
    jobv1t: u8,
    m: i32,
    p: i32,
    q: &[i32],
    x11: &mut [c64],
    ldx11: i32,
    x21: &mut [c64],
    ldx21: i32,
    theta: &mut [f64],
    u1: &mut [c64],
    ldu1: i32,
    u2: &mut [c64],
    ldu2: i32,
    v1t: &mut [c64],
    ldv1t: i32,
    work: &mut [c64],
    lwork: i32,
    rwork: &mut [f64],
    lrwork: i32,
    iwork: &mut [i32],
    info: &mut i32,
) {
    ffi::zuncsd2by1_(
        &(jobu1 as c_char),
        &(jobu2 as c_char),
        &(jobv1t as c_char),
        &m,
        &p,
        q.as_ptr(),
        x11.as_mut_ptr() as *mut _,
        &ldx11,
        x21.as_mut_ptr() as *mut _,
        &ldx21,
        theta.as_mut_ptr(),
        u1.as_mut_ptr() as *mut _,
        &ldu1,
        u2.as_mut_ptr() as *mut _,
        &ldu2,
        v1t.as_mut_ptr() as *mut _,
        &ldv1t,
        work.as_mut_ptr() as *mut _,
        &lwork,
        rwork.as_mut_ptr(),
        &lrwork,
        iwork.as_mut_ptr(),
        info,
    )
}

#[inline]
pub unsafe fn cungbr(
    vect: u8,
    m: i32,
    n: i32,
    k: i32,
    a: &mut [c32],
    lda: i32,
    tau: &[c32],
    work: &mut [c32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::cungbr_(
        &(vect as c_char),
        &m,
        &n,
        &k,
        a.as_mut_ptr() as *mut _,
        &lda,
        tau.as_ptr() as *const _,
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn zungbr(
    vect: u8,
    m: i32,
    n: i32,
    k: i32,
    a: &mut [c64],
    lda: i32,
    tau: &[c64],
    work: &mut [c64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::zungbr_(
        &(vect as c_char),
        &m,
        &n,
        &k,
        a.as_mut_ptr() as *mut _,
        &lda,
        tau.as_ptr() as *const _,
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn cunghr(
    n: i32,
    ilo: i32,
    ihi: i32,
    a: &mut [c32],
    lda: i32,
    tau: &[c32],
    work: &mut [c32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::cunghr_(
        &n,
        &ilo,
        &ihi,
        a.as_mut_ptr() as *mut _,
        &lda,
        tau.as_ptr() as *const _,
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn zunghr(
    n: i32,
    ilo: i32,
    ihi: i32,
    a: &mut [c64],
    lda: i32,
    tau: &[c64],
    work: &mut [c64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::zunghr_(
        &n,
        &ilo,
        &ihi,
        a.as_mut_ptr() as *mut _,
        &lda,
        tau.as_ptr() as *const _,
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn cunglq(
    m: i32,
    n: i32,
    k: i32,
    a: &mut [c32],
    lda: i32,
    tau: &[c32],
    work: &mut [c32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::cunglq_(
        &m,
        &n,
        &k,
        a.as_mut_ptr() as *mut _,
        &lda,
        tau.as_ptr() as *const _,
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn zunglq(
    m: i32,
    n: i32,
    k: i32,
    a: &mut [c64],
    lda: i32,
    tau: &[c64],
    work: &mut [c64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::zunglq_(
        &m,
        &n,
        &k,
        a.as_mut_ptr() as *mut _,
        &lda,
        tau.as_ptr() as *const _,
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn cungql(
    m: i32,
    n: i32,
    k: i32,
    a: &mut [c32],
    lda: i32,
    tau: &[c32],
    work: &mut [c32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::cungql_(
        &m,
        &n,
        &k,
        a.as_mut_ptr() as *mut _,
        &lda,
        tau.as_ptr() as *const _,
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn zungql(
    m: i32,
    n: i32,
    k: i32,
    a: &mut [c64],
    lda: i32,
    tau: &[c64],
    work: &mut [c64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::zungql_(
        &m,
        &n,
        &k,
        a.as_mut_ptr() as *mut _,
        &lda,
        tau.as_ptr() as *const _,
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn cungqr(
    m: i32,
    n: i32,
    k: i32,
    a: &mut [c32],
    lda: i32,
    tau: &[c32],
    work: &mut [c32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::cungqr_(
        &m,
        &n,
        &k,
        a.as_mut_ptr() as *mut _,
        &lda,
        tau.as_ptr() as *const _,
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn zungqr(
    m: i32,
    n: i32,
    k: i32,
    a: &mut [c64],
    lda: i32,
    tau: &[c64],
    work: &mut [c64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::zungqr_(
        &m,
        &n,
        &k,
        a.as_mut_ptr() as *mut _,
        &lda,
        tau.as_ptr() as *const _,
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn cungrq(
    m: i32,
    n: i32,
    k: i32,
    a: &mut [c32],
    lda: i32,
    tau: &[c32],
    work: &mut [c32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::cungrq_(
        &m,
        &n,
        &k,
        a.as_mut_ptr() as *mut _,
        &lda,
        tau.as_ptr() as *const _,
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn zungrq(
    m: i32,
    n: i32,
    k: i32,
    a: &mut [c64],
    lda: i32,
    tau: &[c64],
    work: &mut [c64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::zungrq_(
        &m,
        &n,
        &k,
        a.as_mut_ptr() as *mut _,
        &lda,
        tau.as_ptr() as *const _,
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn cungtr(
    uplo: u8,
    n: i32,
    a: &mut [c32],
    lda: i32,
    tau: &[c32],
    work: &mut [c32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::cungtr_(
        &(uplo as c_char),
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        tau.as_ptr() as *const _,
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn zungtr(
    uplo: u8,
    n: i32,
    a: &mut [c64],
    lda: i32,
    tau: &[c64],
    work: &mut [c64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::zungtr_(
        &(uplo as c_char),
        &n,
        a.as_mut_ptr() as *mut _,
        &lda,
        tau.as_ptr() as *const _,
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn cungtsqr_row(
    m: i32,
    n: i32,
    mb: &[i32],
    nb: i32,
    a: &mut [c32],
    lda: i32,
    t: &[c32],
    ldt: i32,
    work: &mut [c32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::cungtsqr_row_(
        &m,
        &n,
        mb.as_ptr(),
        &nb,
        a.as_mut_ptr() as *mut _,
        &lda,
        t.as_ptr() as *const _,
        &ldt,
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn zungtsqr_row(
    m: i32,
    n: i32,
    mb: &[i32],
    nb: i32,
    a: &mut [c64],
    lda: i32,
    t: &[c64],
    ldt: i32,
    work: &mut [c64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::zungtsqr_row_(
        &m,
        &n,
        mb.as_ptr(),
        &nb,
        a.as_mut_ptr() as *mut _,
        &lda,
        t.as_ptr() as *const _,
        &ldt,
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn cunmbr(
    vect: u8,
    side: u8,
    trans: u8,
    m: i32,
    n: i32,
    k: i32,
    a: &[c32],
    lda: i32,
    tau: &[c32],
    c: &mut [c32],
    ldc: i32,
    work: &mut [c32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::cunmbr_(
        &(vect as c_char),
        &(side as c_char),
        &(trans as c_char),
        &m,
        &n,
        &k,
        a.as_ptr() as *const _,
        &lda,
        tau.as_ptr() as *const _,
        c.as_mut_ptr() as *mut _,
        &ldc,
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn zunmbr(
    vect: u8,
    side: u8,
    trans: u8,
    m: i32,
    n: i32,
    k: i32,
    a: &[c64],
    lda: i32,
    tau: &[c64],
    c: &mut [c64],
    ldc: i32,
    work: &mut [c64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::zunmbr_(
        &(vect as c_char),
        &(side as c_char),
        &(trans as c_char),
        &m,
        &n,
        &k,
        a.as_ptr() as *const _,
        &lda,
        tau.as_ptr() as *const _,
        c.as_mut_ptr() as *mut _,
        &ldc,
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn cunmhr(
    side: u8,
    trans: u8,
    m: i32,
    n: i32,
    ilo: i32,
    ihi: i32,
    a: &[c32],
    lda: i32,
    tau: &[c32],
    c: &mut [c32],
    ldc: i32,
    work: &mut [c32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::cunmhr_(
        &(side as c_char),
        &(trans as c_char),
        &m,
        &n,
        &ilo,
        &ihi,
        a.as_ptr() as *const _,
        &lda,
        tau.as_ptr() as *const _,
        c.as_mut_ptr() as *mut _,
        &ldc,
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn zunmhr(
    side: u8,
    trans: u8,
    m: i32,
    n: i32,
    ilo: i32,
    ihi: i32,
    a: &[c64],
    lda: i32,
    tau: &[c64],
    c: &mut [c64],
    ldc: i32,
    work: &mut [c64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::zunmhr_(
        &(side as c_char),
        &(trans as c_char),
        &m,
        &n,
        &ilo,
        &ihi,
        a.as_ptr() as *const _,
        &lda,
        tau.as_ptr() as *const _,
        c.as_mut_ptr() as *mut _,
        &ldc,
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn cunmlq(
    side: u8,
    trans: u8,
    m: i32,
    n: i32,
    k: i32,
    a: &[c32],
    lda: i32,
    tau: &[c32],
    c: &mut [c32],
    ldc: i32,
    work: &mut [c32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::cunmlq_(
        &(side as c_char),
        &(trans as c_char),
        &m,
        &n,
        &k,
        a.as_ptr() as *const _,
        &lda,
        tau.as_ptr() as *const _,
        c.as_mut_ptr() as *mut _,
        &ldc,
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn zunmlq(
    side: u8,
    trans: u8,
    m: i32,
    n: i32,
    k: i32,
    a: &[c64],
    lda: i32,
    tau: &[c64],
    c: &mut [c64],
    ldc: i32,
    work: &mut [c64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::zunmlq_(
        &(side as c_char),
        &(trans as c_char),
        &m,
        &n,
        &k,
        a.as_ptr() as *const _,
        &lda,
        tau.as_ptr() as *const _,
        c.as_mut_ptr() as *mut _,
        &ldc,
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn cunmql(
    side: u8,
    trans: u8,
    m: i32,
    n: i32,
    k: i32,
    a: &[c32],
    lda: i32,
    tau: &[c32],
    c: &mut [c32],
    ldc: i32,
    work: &mut [c32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::cunmql_(
        &(side as c_char),
        &(trans as c_char),
        &m,
        &n,
        &k,
        a.as_ptr() as *const _,
        &lda,
        tau.as_ptr() as *const _,
        c.as_mut_ptr() as *mut _,
        &ldc,
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn zunmql(
    side: u8,
    trans: u8,
    m: i32,
    n: i32,
    k: i32,
    a: &[c64],
    lda: i32,
    tau: &[c64],
    c: &mut [c64],
    ldc: i32,
    work: &mut [c64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::zunmql_(
        &(side as c_char),
        &(trans as c_char),
        &m,
        &n,
        &k,
        a.as_ptr() as *const _,
        &lda,
        tau.as_ptr() as *const _,
        c.as_mut_ptr() as *mut _,
        &ldc,
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn cunmqr(
    side: u8,
    trans: u8,
    m: i32,
    n: i32,
    k: i32,
    a: &[c32],
    lda: i32,
    tau: &[c32],
    c: &mut [c32],
    ldc: i32,
    work: &mut [c32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::cunmqr_(
        &(side as c_char),
        &(trans as c_char),
        &m,
        &n,
        &k,
        a.as_ptr() as *const _,
        &lda,
        tau.as_ptr() as *const _,
        c.as_mut_ptr() as *mut _,
        &ldc,
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn zunmqr(
    side: u8,
    trans: u8,
    m: i32,
    n: i32,
    k: i32,
    a: &[c64],
    lda: i32,
    tau: &[c64],
    c: &mut [c64],
    ldc: i32,
    work: &mut [c64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::zunmqr_(
        &(side as c_char),
        &(trans as c_char),
        &m,
        &n,
        &k,
        a.as_ptr() as *const _,
        &lda,
        tau.as_ptr() as *const _,
        c.as_mut_ptr() as *mut _,
        &ldc,
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn cunmrq(
    side: u8,
    trans: u8,
    m: i32,
    n: i32,
    k: i32,
    a: &[c32],
    lda: i32,
    tau: &[c32],
    c: &mut [c32],
    ldc: i32,
    work: &mut [c32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::cunmrq_(
        &(side as c_char),
        &(trans as c_char),
        &m,
        &n,
        &k,
        a.as_ptr() as *const _,
        &lda,
        tau.as_ptr() as *const _,
        c.as_mut_ptr() as *mut _,
        &ldc,
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn zunmrq(
    side: u8,
    trans: u8,
    m: i32,
    n: i32,
    k: i32,
    a: &[c64],
    lda: i32,
    tau: &[c64],
    c: &mut [c64],
    ldc: i32,
    work: &mut [c64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::zunmrq_(
        &(side as c_char),
        &(trans as c_char),
        &m,
        &n,
        &k,
        a.as_ptr() as *const _,
        &lda,
        tau.as_ptr() as *const _,
        c.as_mut_ptr() as *mut _,
        &ldc,
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn cunmrz(
    side: u8,
    trans: u8,
    m: i32,
    n: i32,
    k: i32,
    l: i32,
    a: &[c32],
    lda: i32,
    tau: &[c32],
    c: &mut [c32],
    ldc: i32,
    work: &mut [c32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::cunmrz_(
        &(side as c_char),
        &(trans as c_char),
        &m,
        &n,
        &k,
        &l,
        a.as_ptr() as *const _,
        &lda,
        tau.as_ptr() as *const _,
        c.as_mut_ptr() as *mut _,
        &ldc,
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn zunmrz(
    side: u8,
    trans: u8,
    m: i32,
    n: i32,
    k: i32,
    l: i32,
    a: &[c64],
    lda: i32,
    tau: &[c64],
    c: &mut [c64],
    ldc: i32,
    work: &mut [c64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::zunmrz_(
        &(side as c_char),
        &(trans as c_char),
        &m,
        &n,
        &k,
        &l,
        a.as_ptr() as *const _,
        &lda,
        tau.as_ptr() as *const _,
        c.as_mut_ptr() as *mut _,
        &ldc,
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn cunmtr(
    side: u8,
    uplo: u8,
    trans: u8,
    m: i32,
    n: i32,
    a: &[c32],
    lda: i32,
    tau: &[c32],
    c: &mut [c32],
    ldc: i32,
    work: &mut [c32],
    lwork: i32,
    info: &mut i32,
) {
    ffi::cunmtr_(
        &(side as c_char),
        &(uplo as c_char),
        &(trans as c_char),
        &m,
        &n,
        a.as_ptr() as *const _,
        &lda,
        tau.as_ptr() as *const _,
        c.as_mut_ptr() as *mut _,
        &ldc,
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn zunmtr(
    side: u8,
    uplo: u8,
    trans: u8,
    m: i32,
    n: i32,
    a: &[c64],
    lda: i32,
    tau: &[c64],
    c: &mut [c64],
    ldc: i32,
    work: &mut [c64],
    lwork: i32,
    info: &mut i32,
) {
    ffi::zunmtr_(
        &(side as c_char),
        &(uplo as c_char),
        &(trans as c_char),
        &m,
        &n,
        a.as_ptr() as *const _,
        &lda,
        tau.as_ptr() as *const _,
        c.as_mut_ptr() as *mut _,
        &ldc,
        work.as_mut_ptr() as *mut _,
        &lwork,
        info,
    )
}

#[inline]
pub unsafe fn cupgtr(
    uplo: u8,
    n: i32,
    ap: &[c32],
    tau: &[c32],
    q: &mut [c32],
    ldq: i32,
    work: &mut [c32],
    info: &mut i32,
) {
    ffi::cupgtr_(
        &(uplo as c_char),
        &n,
        ap.as_ptr() as *const _,
        tau.as_ptr() as *const _,
        q.as_mut_ptr() as *mut _,
        &ldq,
        work.as_mut_ptr() as *mut _,
        info,
    )
}

#[inline]
pub unsafe fn zupgtr(
    uplo: u8,
    n: i32,
    ap: &[c64],
    tau: &[c64],
    q: &mut [c64],
    ldq: i32,
    work: &mut [c64],
    info: &mut i32,
) {
    ffi::zupgtr_(
        &(uplo as c_char),
        &n,
        ap.as_ptr() as *const _,
        tau.as_ptr() as *const _,
        q.as_mut_ptr() as *mut _,
        &ldq,
        work.as_mut_ptr() as *mut _,
        info,
    )
}

#[inline]
pub unsafe fn cupmtr(
    side: u8,
    uplo: u8,
    trans: u8,
    m: i32,
    n: i32,
    ap: &[c32],
    tau: &[c32],
    c: &mut [c32],
    ldc: i32,
    work: &mut [c32],
    info: &mut i32,
) {
    ffi::cupmtr_(
        &(side as c_char),
        &(uplo as c_char),
        &(trans as c_char),
        &m,
        &n,
        ap.as_ptr() as *const _,
        tau.as_ptr() as *const _,
        c.as_mut_ptr() as *mut _,
        &ldc,
        work.as_mut_ptr() as *mut _,
        info,
    )
}

#[inline]
pub unsafe fn zupmtr(
    side: u8,
    uplo: u8,
    trans: u8,
    m: i32,
    n: i32,
    ap: &[c64],
    tau: &[c64],
    c: &mut [c64],
    ldc: i32,
    work: &mut [c64],
    info: &mut i32,
) {
    ffi::zupmtr_(
        &(side as c_char),
        &(uplo as c_char),
        &(trans as c_char),
        &m,
        &n,
        ap.as_ptr() as *const _,
        tau.as_ptr() as *const _,
        c.as_mut_ptr() as *mut _,
        &ldc,
        work.as_mut_ptr() as *mut _,
        info,
    )
}
