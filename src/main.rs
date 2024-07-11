#[macro_use]
extern crate hello_world_macro;

#[derive(Hello)]
struct ExampleStruct;

fn main() {
    let e = ExampleStruct{};
    e.hello_world();
}
