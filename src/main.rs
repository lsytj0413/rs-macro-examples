use make_public_macro::public;

#[public]
struct Example {}

fn main() {
    let s = "Hello, world!";
    println!("Hello, world, {}", s);
}
