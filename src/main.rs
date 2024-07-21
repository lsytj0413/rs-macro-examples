use function_like_compose_macro::compose;

fn add_one(n: i32) -> i32 {
    n + 1
}

fn stringify(n: i32) -> String {
    n.to_string()
}

fn main() {
    let composed = compose!(
        add_one . add_one . stringify
    );
    println!("{:?}", composed(5))
}
