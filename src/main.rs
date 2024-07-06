
struct FirstName {
    value: String,
}

struct LastName {
    value: String,
}

struct Age {
    value: i32,
}

struct Pay {
    value: i32,
}

macro_rules! generate_get_value {
    ($struct_type:ident) => {
        generate_get_value!($struct_type, String);
    };
    ($struct_type:ident, $return_type:ident) => {
        impl $struct_type {
            pub fn get_value(&self) -> &$return_type {
                &self.value
            }
        }
    }
}

generate_get_value!(FirstName);
generate_get_value!(LastName);
generate_get_value!(Age, i32);
generate_get_value!(Pay, i32);

fn calculate_raise(first_name: FirstName, last_name: LastName, age: Age, pay: Pay) -> Pay {
    if first_name.get_value() == "Sam" {
        Pay{
            value: pay.get_value() + 32
        }
    } else {
        pay
    }
}

fn main() {
    let s = calculate_raise(FirstName{
        value: "Sam".to_string(),
    }, LastName{
        value: "Smith".to_string(),
    }, Age{
        value: 32,
    }, Pay{
        value: 100,
    });
    println!("Hello, world, {}", s.get_value());
}
