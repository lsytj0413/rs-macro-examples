use builder_macro::Builder;

#[derive(Builder)]
struct Gleipnir {
    roots_of: String,
    breath_of_a_fish: u8,
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use builder_macro::Builder;

    #[test]
    fn should_work_with_correct_order() {
        #[derive(Builder)]
        struct Gleipnir {
            roots_of: String,
            breath_of_a_fish: u8,
            anything_else: bool,
        }

        let gleipnir = Gleipnir::builder()
            .roots_of("mountains".to_string())
            .breath_of_a_fish(1)
            .anything_else(true)
            .build();
        assert_eq!(gleipnir.roots_of, "mountains");
        assert_eq!(gleipnir.breath_of_a_fish, 1);
        assert_eq!(gleipnir.anything_else, true);
    }
}
