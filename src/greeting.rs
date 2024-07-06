pub fn base_greeting_fn(name: &str, greeting: &str) -> String {
    format!("{}, {}!", greeting, name)
}

#[macro_export]
macro_rules! greeting {
    ($name:literal) => {
        {
            // use $crate::greeting::base_greeting_fn;
            log_syntax!("The name passed is ", $name);
            base_greeting_fn($name, "Hello")
        }
    };
    ($name:literal, $greeting:literal) => {
        {
            // use $crate::greeting::base_greeting_fn;
            base_greeting_fn($name, $greeting)
        }
    };
}