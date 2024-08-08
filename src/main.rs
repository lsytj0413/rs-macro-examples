use make_public_macro::public;

#[public]
#[derive(Debug)]
struct Example {
    first: String,
    pub second: u32,
}

#[public]
#[derive(Debug)]
struct ExampleUnnamed(pub String, pub u32);

#[public]
#[derive(Debug)]
enum ExampleEnum {
    First(String),
    Second(u32),
}

fn main() {
    let s = Example {
        first: "example".to_string(),
        second: 42,
    };
    println!("Hello, world, {}, {}", s.first, s.second);
    let s = ExampleUnnamed("example_unnamed".to_string(), 43);
    println!("Hello, world, {}, {}", s.0, s.1);
}
