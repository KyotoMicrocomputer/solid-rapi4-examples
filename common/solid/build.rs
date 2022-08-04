use std::env;

fn main() -> miette::Result<()> {
    println!("cargo:rerun-if-env-changed=BUILD_INCLUDE_DIRS");
    let include_dirs = env::var("BUILD_INCLUDE_DIRS").unwrap();
    let mut include_dirs: Vec<_> = include_dirs.split(";").collect();

    let this_path = env::var("CARGO_MANIFEST_DIR").unwrap();
    include_dirs.insert(0, this_path.as_str());

    let mut b = autocxx_build::Builder::new("src/abi.rs", &include_dirs).build()?;

    println!("cargo:rerun-if-env-changed=BUILD_CFLAGS");
    let flags = env::var("BUILD_CFLAGS").unwrap();
    for flag in flags.split_ascii_whitespace() {
        b.flag(flag);
    }
    b.flag_if_supported("-std=c++14").compile("rustapp-ffi");

    println!("cargo:rerun-if-changed=src/abi.rs");
    println!("cargo:rerun-if-changed=src/abi.hpp");
    Ok(())
}
