// extern crate cc;
use std::process::Command;
use std::env;
use std::path::PathBuf;
fn main() {
    // let pwd = Command::new("pwd").output().expect("no pwd!");
    // println!("{:?}",pwd);
    let dst = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    Command::new("ar")
        .arg("rcs")
        .arg(dst.join("libkdb.a"))
        .arg("src/kdb/m64/e.o")
        .output()
        .expect("failed to produce static lib");
}