fn main() {
    println!("cargo::rustc-link-search=../staticlib-lib/target/debug");
    println!("cargo::rustc-link-lib=static=staticlib_lib");
}
