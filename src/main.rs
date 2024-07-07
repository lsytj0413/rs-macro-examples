macro_rules! curry {
    (|$first_arg:ident $(, $arg:ident )*| $function_body:expr) => {
        move |$first_arg| $(move |$arg|)* {
            $function_body
        }
    };
}

fn main() {
    let s = curry!(|a, b, c| a + b + c);
    println!("Hello, world, {}", s(1)(2)(3));
}
