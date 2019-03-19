extern crate cc;

use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let target = env::var("TARGET").unwrap();
    println!("cargo:rustc-link-lib=static=kdb");

    if target.contains("apple") {
        cc::Build::new()
            .object("src/kdb/m64/e.o")
            .compile("libkdb.a");
        let dst = PathBuf::from(env::var_os("OUT_DIR").unwrap());
        Command::new("lipo")
                    .arg("/Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/lib/clang/10.0.0/lib/darwin/libclang_rt.osx.a")
                    .arg("-thin").arg("x86_64").arg("-output")
                    .arg(dst.join("libcrt.a"))
                    .output()
                    .expect("failed to extract thin library");
        println!("cargo:rustc-link-lib=static=crt");
    } else if target.contains("gnu") {
        cc::Build::new()
            .object("src/kdb/l64/e.o")
            .compile("libkdb.a");
    }
}
