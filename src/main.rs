use private_macro::private;

private!(ExampleStruct);
struct ExampleStruct {}

fn main() {
    let e = ExampleStruct {    };
    
    e.hello_world();
}
