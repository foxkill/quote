//
//! # Quote Parser
//!
//! The `quote parser` crate provides a parser for treasury future quotes.
//
mod error;
mod parser;
mod quote;

pub mod prelude {
    pub use crate::error::ParseError;
    pub use crate::parser::Style;
    pub use crate::parser::parse;
    pub use crate::parser::cparse;
    pub use crate::quote::Quote as Quote;
}