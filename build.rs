fn main() {
    println!("cargo:rustc-env=LD_LIBRARY_PATH=./libs");
    println!("cargo:rustc-link-search=./libs");
}
