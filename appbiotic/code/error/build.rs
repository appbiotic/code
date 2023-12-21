fn main() {
    let crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();

    cbindgen::generate(crate_dir)
        .expect("Unable to generate bindings")
        .write_to_file("error.h");
    println!("cargo:rerun-if-changed=src/lib.rs");

    let content = std::fs::read("error.h").unwrap();
    let mut text = String::from_utf8(content).unwrap();
    text = text.replace("\ntypedef int32_t appbiotic_code_error_Code;", "\n");
    text = text.replace(
        "enum appbiotic_code_error_Code {",
        "typedef CF_ENUM(int32_t, appbiotic_code_error_Code) {",
    );
    std::fs::write("error.h", text).unwrap();
}
