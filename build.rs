extern crate cc;

use std::env;

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
            .object("src/kdb/m64/emutls.c.o")
            .compile("libkdb.a");

        #[cfg(target_os = "linux")]
        cc::Build::new()
            .object("src/kdb/l64/e.o")
            .compile("libkdb.a");
    }
}
