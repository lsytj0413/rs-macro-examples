use make_public_macro::public;

#[public]
struct Example {
    first: String,
    pub second: u32,
}

fn main() {
    let s = Example {
        first: "example".to_string(),
        second: 42,
    };
    println!("Hello, world, {}, {}", s.first, s.second);
}
