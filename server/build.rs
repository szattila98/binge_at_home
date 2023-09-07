fn main() {
    println!("cargo:rerun-if-changed=config");
    println!("cargo:rerun-if-changed=resources");
    println!("cargo:rerun-if-changed=migrations");
}
