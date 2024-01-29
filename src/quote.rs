//! Quote parser as struct implementation.

use std::str::FromStr;

use crate::{error::ParseError, parser::{parse, Style}};

#[derive(Debug, Default, PartialEq)]
pub struct Quote(f64); 

impl FromStr for Quote {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Quote, Self::Err> {
        Ok(Self(parse(s, Style::default())?))
    }
}

/// This allows to parse the string by calling the parse method directly.
/// See the example.
///
/// If parsing succeeds, return the value inside [`Ok`], otherwise
/// when the string is ill-formatted return an error specific to the
/// inside [`Err`]. The error type is specific to the implementation of the trait.
///
/// # Examples
///
/// Basic usage with [`Quote`], a type that implements `FromStr`:
///
/// ```
/// let expected = 126.78125;
/// let result: f64 = "126'25".parse::<Quote>().unwrap().into(); 
/// assert_eq!(result, expected);
/// ```
impl From<Quote> for f64 {
    fn from(val: Quote) -> Self {
        val.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_apply_parse_on_a_str() {
        let expected = 126.78125;
        let result: f64 = "126'25".parse::<Quote>().unwrap().into(); 
        assert_eq!(result, expected);
    } 
}
