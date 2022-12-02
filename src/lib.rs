use std::fs::read_to_string;

#[macro_export]
macro_rules! function {
    ($f:ident) => {{
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of($f);
        &name[15..]
    }}
}

pub fn get_input(name: &str) -> String {
    read_to_string(&format!("inputs/{name}")).expect("File doesn't exist.")
}

#[macro_export]
macro_rules! input {
    ($f:ident) => {
        get_input(function!($f))
    }
}

#[cfg(test)]
mod y2022 {
    mod day1;
    mod day2;
}
