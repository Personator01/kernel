use std::env;
use std::process::Command;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    Command::new("gcc").args(["-c", "-o", format!("{}/boot.o", out_dir).as_str(), "boot.S"]);
    
}
