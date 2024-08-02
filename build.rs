use std::env;
use std::process::Command;
extern crate cc;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    println!("cargo:rerun-if-changed=src/init/boot.S");
    // Command::new("gcc").args(["-c", "-o", format!("{}/boot.o", out_dir).as_str(), "boot.S"]);
    cc::Build::new()
    .file("src/init/boot.S")
    .compile("boot");

    
}
