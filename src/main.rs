use upper_case_macro::UpperCaseName;

#[derive(UpperCaseName)]
struct Example;

fn main() {
    let s = Example {};
    s.uppercase();
}
