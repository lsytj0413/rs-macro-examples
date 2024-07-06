#![feature(trace_macros)]
#![feature(log_syntax)]

use crate::greeting::base_greeting_fn;

// #[macro_use]  // 如果没有这一行，会报错找不到 greeting!
mod greeting;

fn main() {
    trace_macros!(true);
    let greet = greeting!("Sam", "Heya");
    println!("{}", greet);
    let greet_with_default = greeting!("Sam");
    println!("{}", greet_with_default);
    trace_macros!(false);
}
