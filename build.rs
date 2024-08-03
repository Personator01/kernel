extern crate cc;

fn main() {
    println!("cargo:rerun-if-changed=src/init/boot.S");
    println!("cargo:rerun-if-changed=link.ld");
    // Command::new("gcc").args(["-c", "-o", format!("{}/boot.o", out_dir).as_str(), "boot.S"]);
    cc::Build::new()
    .file("src/init/boot.S")
    .compile("boot");

    
}
