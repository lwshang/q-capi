extern crate ctest;

fn main() {
    let mut cfg = ctest::TestGenerator::new();

    // Include the header files where the C APIs are defined
    cfg.header("k.h");
    cfg.flag("-DKXVER=3");

    // Include the directory where the header files are defined
    cfg.include("src");

    // Skip some tests
    cfg.skip_struct(|ty| {
        match ty {
            "U" => true, // TODO: figure out how to test opauqe struct type U
            _ => false,
        }
    });

    cfg.skip_type(|s| match s {
        "V" => true,
        _ => false,
    });

    // following functions are not defined in e.o, so skip here
    cfg.skip_fn(|x| match x {
        "dl" | "dot" | "sd0" | "sd0x" | "sd1" => true,
        _ => false,
    });

    // Translate some Rust types to C
    cfg.type_name(|ty, _, _| match ty {
        "Sconst" => "S".to_string(),
        "k0" => "struct k0".to_string(),
        _ => ty.to_string(),
    });

    // Skip roundtrip tests
    cfg.skip_roundtrip(|_| true);

    // Generate the tests, passing the path to the `*-sys` library as well as
    // the module to generate.
    cfg.generate("../src/lib.rs", "all.rs");
}
