use appbiotic_code_error::Error;

pub fn main() {
    eprintln!("NOTE: Not implemented yet!");
    eprintln!("Error: {}", Error::internal("Unimplemented"));
    std::process::exit(1);
}
