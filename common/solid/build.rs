use std::env;

fn main() -> miette::Result<()> {
    println!("cargo:rerun-if-env-changed=BUILD_INCLUDE_DIRS");
    let include_dirs = env::var("BUILD_INCLUDE_DIRS")
        .unwrap_or_else(|_| report_missing_build_vars_and_exit("BUILD_INCLUDE_DIRS"));
    let mut include_dirs: Vec<_> = include_dirs.split(";").collect();

    println!("cargo:rerun-if-env-changed=BUILD_CFLAGS");
    let flags = env::var("BUILD_CFLAGS")
        .unwrap_or_else(|_| report_missing_build_vars_and_exit("BUILD_CFLAGS"));
    dbg!(&flags);
    let flags = flags.split_ascii_whitespace();

    let this_path = env::var("CARGO_MANIFEST_DIR").unwrap();
    include_dirs.insert(0, this_path.as_str());

    let mut b = autocxx_build::Builder::new("src/abi.rs", &include_dirs)
        .extra_clang_args(&flags.clone().collect::<Vec<_>>())
        .build()?;

    for flag in flags {
        b.flag(flag);
    }
    b.flag_if_supported("-std=c++14").compile("rustapp-ffi");

    println!("cargo:rerun-if-changed=src/abi.rs");
    println!("cargo:rerun-if-changed=src/abi.hpp");
    Ok(())
}

fn report_missing_build_vars_and_exit(name: &str) -> ! {
    eprint!(
        r#"error: environment variable ${name} is not provided
 
This package needs to know the C++ compiler flags of your program to
automatically generate low-level Rust bindings.
 
You have to configure your SOLID-Rust project to pass C++ compiler flags to
Rust's build system. Open your '.ptrsproj' file by a text editor and add the
following code next to the 'ProjectGuid' element (make sure to replace
'< PROJECT >' with the name of a C++ project in your SOLID-IDE solution):
 
    <CargoEnvironmentVariables>
        BUILD_INCLUDE_DIRS=$expand:{{"projectName":"< PROJECT >", "type": "property", "query": "IncludePath"}}
        BUILD_CFLAGS=$expand:{{"projectName":"< PROJECT >", "type": "property", "query": "GCCSW"}}
    </CargoEnvironmentVariables>
 "#
    );
    std::process::exit(1);
}
