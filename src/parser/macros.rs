//! Contains all macros for quote parser library.
//!
//! 
#![allow(unused_macros)] 

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

macro_rules! re {
    ($re:ident $(,)?) => {{
        static ONCE: std::sync::OnceLock<regex::Regex> = std::sync::OnceLock::new();
        ONCE.get_or_init(|| regex::Regex::new($re).unwrap())
    }};
}

pub(crate) use re;
pub(crate) use extract_capture;