use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let mut source_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    source_path.push("resources");
    source_path.push("tests");
    source_path.push("lorem.txt");

    let mut dest_path = PathBuf::from(env::var("OUT_DIR").expect("can't get OUT_DIR"));
    dest_path.push("lorem.txt");
    fs::copy(
        source_path.to_str().expect("can't convert path to string"),
        dest_path.to_str().expect("can't convert path to string"),
    )
    .expect("can't copy file");
}
