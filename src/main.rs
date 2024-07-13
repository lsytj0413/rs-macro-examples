use make_public_macro::delete;

#[delete]
struct EmptyStruct {}

fn main() {
    let s = "delete";
    println!("Hello, world, {}", s);
}
