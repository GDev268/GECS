use std::env;
use std::fs;
use std::io::Write;

fn main() {
    // Dynamically define a value (e.g., from config or computation)
    let type_count = env::var("TYPE_COUNT").unwrap_or_else(|_| "10".to_string());

    // Make the value available to the Rust compiler via an environment variable
    println!("cargo:rustc-env=TYPE_COUNT={}", type_count);

    // Optionally, write a generated file for additional consts if needed
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = std::path::Path::new(&out_dir).join("generated_constants.rs");
    let mut file = fs::File::create(&dest_path).unwrap();
    writeln!(
        file,
        "/// Dynamically generated constant
pub const TYPE_COUNT: usize = {};",
        type_count
    )
    .unwrap();
}