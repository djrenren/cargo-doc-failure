#[cfg(all(not(doc)))]
std::compile_error!("Library only builds as doc");

fn main() {
    println!("Hello, world!");
}
