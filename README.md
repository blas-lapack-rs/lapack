# LAPACK [![Version][version-img]][version-url] [![Status][status-img]][status-url]

The package provides an interface to the [Linear Algebra PACKage][1].

## [Documentation][docs]

## Example (C)

```rust
use lapack::c::*;

let n = 3;
let mut a = vec![3.0, 1.0, 1.0, 1.0, 3.0, 1.0, 1.0, 1.0, 3.0];
let mut w = vec![0.0; n as usize];

let info = dsyev(Layout::ColumnMajor, b'V', b'U', n, &mut a, n, &mut w);
assert_eq!(info, 0);

for (one, another) in w.iter().zip(&[2.0, 2.0, 5.0]) {
    assert!((one - another).abs() < 1e-14);
}
```

## Example (Fortran)

```rust
use lapack::fortran::*;

let n = 3;
let mut a = vec![3.0, 1.0, 1.0, 1.0, 3.0, 1.0, 1.0, 1.0, 3.0];
let mut w = vec![0.0; n as usize];
let mut work = vec![0.0; 4 * n as usize];
let lwork = 4 * n;
let mut info = 0;

dsyev(b'V', b'U', n, &mut a, n, &mut w, &mut work, lwork, &mut info);
assert_eq!(info, 0);

for (one, another) in w.iter().zip(&[2.0, 2.0, 5.0]) {
    assert!((one - another).abs() < 1e-14);
}
```

## Contribution

Your contribution is highly appreciated. Do not hesitate to open an issue or a
pull request. Note that any contribution submitted for inclusion in the project
will be licensed according to the terms given in [LICENSE.md](LICENSE.md).

[1]: http://en.wikipedia.org/wiki/LAPACK

[version-img]: https://img.shields.io/crates/v/lapack.svg
[version-url]: https://crates.io/crates/lapack
[status-img]: https://travis-ci.org/stainless-steel/lapack.svg?branch=master
[status-url]: https://travis-ci.org/stainless-steel/lapack
[docs]: https://stainless-steel.github.io/lapack
