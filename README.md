# LAPACK [![Package][package-img]][package-url] [![Documentation][documentation-img]][documentation-url] [![Build][build-img]][build-url]

The package provides wrappers for [LAPACK] (Fortran).

The usage of the package is explained [here][usage].

## Example

```rust
use lapack::*;

let n = 3;
let mut a = vec![3.0, 1.0, 1.0, 1.0, 3.0, 1.0, 1.0, 1.0, 3.0];
let mut w = vec![0.0; n as usize];
let mut work = vec![0.0; 4 * n as usize];
let lwork = 4 * n;
let mut info = 0;

unsafe {
    dsyev(b'V', b'U', n, &mut a, n, &mut w, &mut work, lwork, &mut info);
}

assert!(info == 0);
for (one, another) in w.iter().zip(&[2.0, 2.0, 5.0]) {
    assert!((one - another).abs() < 1e-14);
}
```

## Contribution

Your contribution is highly appreciated. Do not hesitate to open an issue or a
pull request. Note that any contribution submitted for inclusion in the project
will be licensed according to the terms given in [LICENSE.md](LICENSE.md).

[lapack]: https://en.wikipedia.org/wiki/LAPACK
[usage]: https://blas-lapack-rs.github.io/usage

[build-img]: https://travis-ci.org/blas-lapack-rs/lapack.svg?branch=master
[build-url]: https://travis-ci.org/blas-lapack-rs/lapack
[documentation-img]: https://docs.rs/lapack/badge.svg
[documentation-url]: https://docs.rs/lapack
[package-img]: https://img.shields.io/crates/v/lapack.svg
[package-url]: https://crates.io/crates/lapack
