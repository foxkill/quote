//!crate error

#[allow(clippy::enum_variant_names)]
#[derive(Debug, PartialEq)]
pub enum ParseError {
    InvalidQuote,
    InvalidNumber,
    InvalidFraction,
    InvalidFraction32,
    InvalidStyle,
}