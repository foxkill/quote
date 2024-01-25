//! Contains all macros for quote parser library.
//!
//! 
 
#[macro_export]
macro_rules! extract_capture {
    ($caps:expr, $($name:ident), *) => {
        $(let $name: &str = $caps.name(stringify!($name)).map_or("", |m| m.as_str());)*
    };
}