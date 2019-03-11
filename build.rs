extern crate cc;

fn main() {
    cc::Build::new().object("src/kdb/m64/e.o").compile("libkdb.a");
}