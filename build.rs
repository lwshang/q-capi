extern crate cc;

use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let host = env::var("HOST").unwrap();
    let target = env::var("TARGET").unwrap();

    if host != target {
        panic!("Cross compilation not supported");
    }

    if cfg!(feature = "ipc") {
        println!("cargo:rustc-link-lib=static=kdb");
        
        #[cfg(target_os = "macos")]
        cc::Build::new()
            .object("src/kdb/m64/e.o")
            .compile("libkdb.a");
        let dst = PathBuf::from(env::var_os("OUT_DIR").unwrap());
        Command::new("lipo")
            .arg("/Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/lib/clang/10.0.0/lib/darwin/libclang_rt.osx.a")
            .arg("-thin").arg("x86_64").arg("-output")
            .arg(dst.join("libclang_rt.osx.a"))
            .output()
            .expect("failed to extract thin library");
        println!("cargo:rustc-link-lib=static=clang_rt.osx");

        #[cfg(target_os = "linux")]
        cc::Build::new()
            .object("src/kdb/l64/e.o")
            .compile("libkdb.a");
    }
}
