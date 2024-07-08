use lazy_static::lazy_static;

lazy_static! {
    static ref GREETING: String = "lazy string".to_string();
}

fn main() {
    println!("Hello, world, {}", *GREETING);
}
