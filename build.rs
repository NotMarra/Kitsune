use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let mut dll_src = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    dll_src.push("libs");
    dll_src.push("libmpv-2.dll");

    let mut dest = PathBuf::from(env::var("OUT_DIR").unwrap());
    dest.pop();
    dest.pop();
    dest.pop();
    dest.push("libmpv-2.dll");

    if dll_src.exists() {
        fs::copy(dll_src, dest).ok();
    }
}