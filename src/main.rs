#[macro_use]
extern crate hello_world_macro;

#[derive(Hello)]
struct Example;

fn main() {
    let s = "Hello, world!";
    println!("Hello, world, {}", s);
}
