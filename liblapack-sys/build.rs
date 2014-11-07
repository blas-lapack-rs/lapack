#![feature(macro_rules)]

macro_rules! cmd(
    ($name:expr) => (::std::io::process::Command::new($name));
)

macro_rules! fmt(
    ($($arg:tt)*) => (format_args!(::std::fmt::format, $($arg)*).as_slice());
)

macro_rules! get(
    ($name:expr) => (::std::os::getenv($name).unwrap());
)

macro_rules! run(
    ($command:expr) => (
        assert!($command.stdout(::std::io::process::InheritFd(1))
                        .stderr(::std::io::process::InheritFd(2))
                        .status().unwrap().success());
    );
)

fn main() {
    let from = Path::new(get!("CARGO_MANIFEST_DIR")).join("liblapack");
    let into = Path::new(get!("OUT_DIR"));

    run!(cmd!("cmake").cwd(&into).arg(&from).arg("-DCMAKE_Fortran_FLAGS=-fPIC"));
    run!(cmd!("make").cwd(&into).arg(fmt!("-j{}", get!("NUM_JOBS"))));

    println!("cargo:rustc-flags=-L {}", into.join("lib").display());
    println!("cargo:rustc-flags=-l blas:static");
    println!("cargo:rustc-flags=-l lapack:static");
}
