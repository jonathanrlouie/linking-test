fn main() {
    println!("cargo::rustc-link-search=../cdylib-lib/target/debug");
    println!("cargo::rustc-link-lib=dylib=cdylib_lib");
}
