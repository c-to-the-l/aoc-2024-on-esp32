use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    println!("cargo:rustc-link-arg-bins=-Tlinkall.x");
    println!("cargo:rustc-link-arg-bins=-Trom-functions.x");
    println!("cargo:rustc-link-arg-bins=-Tdefmt.x");
    if let Ok(file) = File::open(".env") {
        println!("cargo:rerun-if-changed=.env");
        let mut lines = BufReader::new(file).lines();
        while let Some(Ok(l)) = lines.next() {
            if !l.contains('=') {
                continue;
            }
            println!("cargo:rustc-env={l}");
        }
    }
}
