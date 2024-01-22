//
// Quote Parser
//
#![allow(unused)] // for starting projects only

use crate::error::ParseError;
use crate::regex::Regex;
use crate::style::Style;
use crate::styleparsers::{
    parse_bond_future_price, parse_note_future_price, parse_short_term_note_future_price,
    parse_treasury_price,
};
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::str::FromStr;
// macro_rules! concat {
//         ($($e:expr),* $(,)?) => {{ /* compiler built-in */ }};
//     }
// use lazy_regex::regex; Consider lazy regex insted of lazy_static?
macro_rules! cvar {
    ($s:ident, $caps:expr) => {
        let $s: &str = $caps.name(stringify!($s)).map_or("", |m| m.as_str());
    };
}
lazy_static! {
    static ref QUOTE_EXPRESSION_RE: Regex = Regex::new(concat!(
        r"(?P<number>^\d+)(?P<delimiter_frac>[\.\-\'])?",
        r"(?P<fraction>\d{2})?(?P<delimiter32>\'?)(?P<fraction32>[\d+,\+])?"
    ))
    .unwrap();
}

#[derive(Debug, Default)]
struct Quote {
    price: f64,
}

impl PartialEq for Quote {
    fn eq(&self, other: &Self) -> bool {
        self.price == other.price
    }
}
/// Parses a string `s` to return a decimal representation of
/// the string value.
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
/// use quote::Quote
///
/// let s = "103-04+";
/// let x = Quote::from_str(s).unwrap();
///
/// assert_eq!(103.140625, x);
/// ```
impl FromStr for Quote {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, ParseError> {
        Quote::parse(s, Style::Detect)
    }
}
impl Quote {
    fn new() -> Self {
        // .ok_or_else(|| ParseError::InvalidString)
        Quote::default()
    }

    /// Try to parse a quote.
    pub fn parse(s: &str, quotestyle: Style) -> Result<Self, ParseError> {
        // First try parse a simple float.
        if let Ok(price) = s.parse::<f64>() {
            return Ok(Quote { price });
        };

        let Some(captures) = QUOTE_EXPRESSION_RE.captures(s) else {
            return Err(ParseError::Quote);
        };

        cvar!(number, captures);
        cvar!(delimiter_frac, captures);
        cvar!(fraction, captures);
        cvar!(delimiter32, captures);
        cvar!(fraction32, captures);

        match if quotestyle == Style::Detect {
            Style::detect(fraction32, delimiter_frac, delimiter32)
        } else {
            quotestyle
        } {
            Style::Bond => Ok(Quote {
                price: parse_treasury_price(number, fraction, fraction32)?,
            }),
            Style::BondFuture => Ok(Quote {
                price: parse_bond_future_price(number, fraction, fraction32)?,
            }),
            Style::NoteFuture => Ok(Quote {
                price: parse_note_future_price(number, fraction, fraction32)?,
            }),
            Style::ShortNoteFuture => Ok(Quote {
                price: parse_short_term_note_future_price(number, fraction, fraction32)?,
            }),
            _ => Err(ParseError::Style),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::{error::ParseError, style::Style};

    use super::Quote;

    #[test]
    fn parse_decimal() {
        let result = Quote::from_str("123.45").unwrap().price;
        let expected = 123.45;
        assert_eq!(result, 123.45);
    }

    #[test]
    fn parse_unqualified_str() {
        let parsed_price = Quote::parse("tum4", Style::Detect);
        assert!(parsed_price.is_err());
    }

    #[test]
    fn parse_bond_quote() {
        let expected = 103.125;
        let result = Quote::from_str("103-04").unwrap().price;
        assert_eq!(result, expected);
    }

    #[test]
    fn parse_default_bond_quote() {
        let expected = 104.140625;
        let result = Quote::from_str("104-04+").unwrap().price;
        assert_eq!(result, expected);
    }

    #[test]
    fn parse_quote_with_comma() {
        let result = Quote::from_str("104,04");
        let e = Err(ParseError::Number);
        assert_eq!(e, result);
    }
}
