macro_rules! hello_world {
    ($T:ident) => {
        impl $T {
            fn hello_world(&self) {
                println!("Hello world")
            }
        }
    };
}

struct Example {}
hello_world!(Example);

fn main() {
    let s = Example {};
    s.hello_world();
}
