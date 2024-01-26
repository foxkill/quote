//! Contains all macros for quote parser library.
//!
//! 
#![allow(unused_macros)] 

#[macro_export]
macro_rules! extract_capture {
    ($caps:expr, $($name:ident), *) => {
        $(let $name: &str = $caps.name(stringify!($name)).map_or("", |m| m.as_str());)*
    };
}

macro_rules! hashmap {
    ($($key:expr => $value:expr),*) => ({
        let mut map = HashMap::new();
        $(map.insert($key, $value);)*
        map
    });
}