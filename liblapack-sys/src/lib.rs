extern crate libc;

pub use blas::dgemm_ as dgemm;
pub use blas::dgemv_ as dgemv;
pub use lapack::dsyev_ as dsyev;

mod blas;
mod lapack;

#[link(name = "gfortran")]
extern {}
