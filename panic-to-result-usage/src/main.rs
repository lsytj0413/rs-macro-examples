use panic_to_result_macro::panic_to_result;


#[derive(Debug)]
pub struct Person {
    name: String,
    age: u32,
}

#[panic_to_result]
fn create_person(name: String, age: u32) -> Person {
    if age > 30 {
        panic!("I hope I die before I get old");
    }

    Person {
        name,
        age,
    }
}

#[panic_to_result]
fn create_person_with_empty_panic(name: String, age: u32) -> Person {
    if age > 30 {
        panic!();
    }

    Person {
        name,
        age,
    }
}

#[panic_to_result]
fn create_person_with_result(name: String, age: u32) -> Result<Person, String> {
    if age > 30 {
        return Err("I hope I die before I get old".to_string());
    }

    Ok(Person {
        name,
        age,
    })
}

#[panic_to_result]
fn create_person_with_result_and_empty_panic(name: String, age: u32) -> Result<Person, String> {
    if age > 30 {
        panic!();
    }

    Ok(Person {
        name,
        age,
    })
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn happy_path() {
        let actual = create_person("Sam".to_string(), 22).unwrap();

        assert_eq!(actual.name, "Sam".to_string());
        assert_eq!(actual.age, 22);
    }

    #[test]
    fn should_panic_on_invalid_age() {
        let actual = create_person("S".to_string(), 33);

        assert_eq!(
            actual.expect_err("this should be an err"),
            "I hope I die before I get old".to_string(),
        )
    }
}