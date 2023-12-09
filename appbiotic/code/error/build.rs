use std::env;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    cbindgen::generate(crate_dir)
        .expect("Unable to generate bindings")
        .write_to_file("error.h");

    let content = std::fs::read("error.h").unwrap();
    let mut text = String::from_utf8(content).unwrap();
    text = text.replace("\ntypedef int32_t AppbioticErrorCode;", "\n");
    text = text.replace(
        "enum AppbioticErrorCode {",
        "typedef CF_ENUM(int32_t, AppbioticErrorCode) {",
    );
    std::fs::write("error.h", text).unwrap();
}
