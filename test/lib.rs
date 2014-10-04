#![crate_name = "lapack"]

extern crate lapack;
extern crate test;

use lapack::dgemv;
use test::Bencher;

#[bench]
fn bench_dgemv_few_large(b: &mut Bencher) {
    #[allow(non_uppercase_statics)]
    static m: uint = 1000;

    let a = box [0.0, ..m*m];
    let x = box [0.0, ..m*1];
    let mut y = box [0.0, ..m*1];

    b.iter(|| {
        dgemv('N' as i8, m as i32, m as i32, 1.0, &*a,
            m as i32, &*x, 1, 1.0, &mut *y, 1)
    })
}

#[bench]
fn bench_dgemv_many_small(b: &mut Bencher) {
    #[allow(non_uppercase_statics)]
    static m: uint = 20;

    let a = box [0.0, ..m*m];
    let x = box [0.0, ..m*1];
    let mut y = box [0.0, ..m*1];

    b.iter(|| {
        for _ in range(0u, 20000) {
            dgemv('N' as i8, m as i32, m as i32, 1.0, &*a,
                m as i32, &*x, 1, 1.0, &mut *y, 1);
        }
    })
}
