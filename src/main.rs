use make_public_macro::public;

#[public(exclude(fourth, third))]
struct Example {
    first: String,
    pub second: u32,
    third: bool,
    fourth: String,
}

impl Example {
    pub fn new() -> Self {
        Example {
            first: "example".to_string(),
            second: 42,
            third: true,
            fourth: "fourth".to_string(),
        }
    }
}


fn main() {
    let s = Example::new();
    println!("Hello, world, {}, {}", s.first, s.second);
}
