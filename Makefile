# Prefix as spaces
.RECIPEPREFIX += $(.RECIPEPREFIX) 

kernel: 
    RUST_TARGET_PATH=$(pwd) cargo build --profile=release


.PHONY: clean 
clean:
    cargo clean

.PHONY: iso
iso: kernel
    isobuild/isobuild

.PHONY: run
run: iso
    qemu-system-x86_64 -cdrom isobuild/os.iso


