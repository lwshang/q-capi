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

    // Translate some Rust types to C
    cfg.type_name(|ty, _, _| {
    match ty {
        "Sconst" => "S".to_string(),
        "k0" => "struct k0".to_string(),
        _ => ty.to_string(),
    }
});

    // Generate the tests, passing the path to the `*-sys` library as well as
    // the module to generate.
    cfg.generate("../src/lib.rs", "all.rs");
}