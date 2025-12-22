fn main() {
    println!("RUSTC_LINT BUILD");
    for (key, value) in std::env::vars() {
        println!("{key}: {value}");
    }
}