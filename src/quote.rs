//! Quote parser as struct implementation.

use std::convert::From;
use std::str::FromStr;

use crate::{
    error::ParseError,
    parser::{parse, Style},
};

#[derive(Debug, Default, PartialEq)]
pub struct Quote(f64);

impl FromStr for Quote {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Quote, Self::Err> {
        Ok(Self(parse(s, Style::default())?))
    }
}

/// Parses a string into a Quote.
///
/// If parsing succeeds, returns the value inside `Ok`, otherwise
/// when the string is ill-formatted, returns an error specific to
/// the implementation of the trait.
///
/// # Examples
///
/// Basic usage with `Quote`, a type that implements `FromStr`:
///
/// ```
/// use quoteparser::prelude::*;
///
/// let expected = 126.78125;
/// let result: f64 = "126'25".parse::<Quote>().unwrap().into();
/// assert_eq!(result, expected);
/// ````
impl From<Quote> for f64 {
    fn from(val: Quote) -> Self {
        val.0
    }
}

impl From<&str> for Quote {
    fn from(value: &str) -> Self {
        value.parse().unwrap()
    }
}

impl From<f64> for Quote {
    fn from(value: f64) -> Self {
        Self(value)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn it_should_apply_parse_on_a_str() {
        let expected = 126.78125;
        let result: f64 = "126'25".parse::<Quote>().unwrap().into();
        assert_eq!(expected, result);
    }

    #[test]
    fn it_should_apply_parse_on_a_string() {
        let expected = 126.78125;
        let quote: Quote = "126'25".parse().unwrap();
        let result: f64 = quote.into();
        assert_eq!(expected, result);
    }

    #[test]
    fn it_should_throw_an_error_if_string_is_invalid() {
        let expected: Result<_, ParseError> = Err(ParseError::InvalidQuote);
        let result = "".parse::<Quote>();
        assert_eq!(expected, result);
    }

    #[test]
    fn it_shoud_support_the_from_trait() {
        let result: f64 = Quote::from("126'25").into();
        assert_eq!(126.78125, result);
    }
}
