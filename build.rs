fn main() {
    println!("cargo:rustc-link-lib=secrets");
    println!("cargo:rustc-link-search=.");
}
