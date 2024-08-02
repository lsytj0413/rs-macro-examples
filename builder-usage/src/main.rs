use std::marker::PhantomData;

use builder_macro::Builder;

#[derive(Builder)]
struct ExampleStruct {
}

trait Light {}

struct Green {}
impl Light for Green {}
struct Red {}
impl Light for Red {}

struct TrafficLight<T: Light> {
    marker: PhantomData<T>
}

impl TrafficLight<Green> {
    fn turn_red(&self) -> TrafficLight<Red> {
        TrafficLight { marker: Default::default() }
    }
}

impl TrafficLight<Red> {
    fn turn_green(&self) -> TrafficLight<Green> {
        TrafficLight { marker: Default::default() }
    }
}

fn main() {
    let light = TrafficLight{ marker: Default::default() };
    light.turn_red().turn_green();
    let light2 = TrafficLight{ marker: Default::default() };
    light2.turn_green().turn_red();

    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use builder_macro::Builder;

    #[test]
    fn should_generate_builder_for_struct_with_no_properties() {
        #[derive(Builder)]
        struct ExampleStructNoFields {}

        let _: ExampleStructNoFields = ExampleStructNoFields::builder().build();
    }

    #[test]
    fn should_generate_builder_for_struct_with_one_property() {
        #[derive(Builder)]
        struct Gleipnir {
            roots_of: String,
        }

        let gleipnir = Gleipnir::builder().roots_of("mountains".to_string()).build();
        assert_eq!(gleipnir.roots_of, "mountains");
    }

    #[test]
    fn should_generate_builder_for_struct_with_two_properties() {
        #[derive(Builder)]
        struct Gleipnir {
            roots_of: String,
            breath_of_a_fish: u8
        }

        let gleipnir = Gleipnir::builder()
            .roots_of("mountains".to_string())
            .breath_of_a_fish(1)
            .build();
        assert_eq!(gleipnir.roots_of, "mountains");
        assert_eq!(gleipnir.breath_of_a_fish, 1);
    }

    #[test]
    fn should_generate_builder_for_struct_with_multiple_properties() {
        #[derive(Builder)]
        struct Gleipnir {
            roots_of: String,
            breath_of_a_fish: u8,
            other_attrs: Vec<String>,
        }

        let gleipnir = Gleipnir::builder()
            .roots_of("mountains".to_string())
            .breath_of_a_fish(1)
            .other_attrs(vec!["a".to_string(), "b".to_string()])
            .build();
        assert_eq!(gleipnir.roots_of, "mountains");
        assert_eq!(gleipnir.breath_of_a_fish, 1);
        assert_eq!(gleipnir.other_attrs, vec!["a".to_string(), "b".to_string()]);
    }

    #[test]
    #[should_panic]
    fn should_panic_when_field_is_missing() {
        #[derive(Builder)]
        struct Gleipnir {
            _roots_of: String,
        }

        Gleipnir::builder().build();
    }

    #[test]
    fn should_generate_builder_for_struct_with_one_renamed_property() {
        #[derive(Builder)]
        struct Gleipnir {
            #[rename("tops_of")]
            roots_of: String,
        }

        let gleipnir = Gleipnir::builder().tops_of("mountains".to_string()).build();
        assert_eq!(gleipnir.roots_of, "mountains");
    }

    #[test]
    fn should_generate_builder_for_struct_with_one_renamed_namedvalue_property() {
        #[derive(Builder)]
        struct Gleipnir {
            #[rename = "tops_of"]
            roots_of: String,
        }

        let gleipnir = Gleipnir::builder().tops_of("mountains".to_string()).build();
        assert_eq!(gleipnir.roots_of, "mountains");
    }

    #[test]
    fn should_use_defaults_when_attribute_is_present() {
        #[derive(Builder)]
        #[builder_defaults]
        struct ExampleStructTwoFields {
            string_value: String,
            int_value: i32,
        }

        let example = ExampleStructTwoFields::builder().build();
        assert_eq!(example.string_value, String::default());
        assert_eq!(example.int_value, i32::default());
    }
}
