extern crate cc;

fn main() {
    println!("cargo:rerun-if-changed=src/arch/amd64/boot.S");
    println!("cargo:rerun-if-changed=link.ld");
    cc::Build::new()
    .file("src/arch/amd64/boot.S")
    .compile("boot");

    
}
