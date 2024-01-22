//
// Quote Parser
//
#![allow(unused)] // for starting projects only

use std::str::FromStr;
use std::collections::HashMap;
use crate::regex::Regex;
use crate::style::QuoteStyle;
use crate::error::ParseError;
use crate::styleparsers::{parse_treasury_price, parse_short_term_note_future_price, parse_bond_future_price, parse_note_future_price};
use lazy_static::lazy_static;

// type QuoteParser = for<'a, 'b, 'c> fn(&'a str, &'b str, &'c str) -> Result<Quote, ParseError>;

lazy_static! {
    static ref QUOTE_EXPRESSION_RE: Regex = Regex::new(concat!(
        r"(?P<number>^\d+)(?P<delimiter_frac>[\.\-\'])?",
        r"(?P<fraction>\d{2})?(?P<delimiter32>\'?)(?P<fraction32>[\d+,\+])?"
    )).unwrap();
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
impl FromStr for Quote {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, ParseError> {
        Quote::parse(s, QuoteStyle::Detect)
    }
}
impl Quote {
    fn new() -> Self {
        // .ok_or_else(|| ParseError::InvalidString)
        Quote::default()
    }

    /// Try to parse a quote.
    pub fn parse(s: &str, quotestyle: QuoteStyle) -> Result<Self, ParseError> {
        // First try if it is just a simple float.
        if let Ok(price) = s.parse::<f64>() {
            return Ok(Quote { price });
        };

        let Some(captures) = QUOTE_EXPRESSION_RE.captures(s) else {
            return Err(ParseError::Quote);
        };

        let Some(number) = captures.name("number") else {
            return Err(ParseError::Number);
        };

        let delimiter_frac: &str = match captures.name("delimiter_frac") {
            Some(delimiter_frac_str) => delimiter_frac_str.as_str(),
            None => "",
        };

        let fraction: &str = match captures.name("fraction") {
            Some(fraction_str) => fraction_str.as_str(),
            None => "",
        };

        let delimiter32 = match captures.name("delimiter32") {
            Some(delimiter32_str) => delimiter32_str.as_str(),
            None => "",
        };

        let fraction32: &str = captures.name("fraction32").map_or("", |f| f.as_str());

        match if quotestyle == QuoteStyle::Detect {
            QuoteStyle::detect(fraction32, delimiter_frac, delimiter32)
        } else {
            quotestyle
        } {
            QuoteStyle::Bond => Ok(Quote {
                 price: parse_treasury_price(number.as_str(), fraction, fraction32)?,
            }),
            QuoteStyle::BondFuture => Ok(Quote {
                price: parse_bond_future_price(number.as_str(), fraction, fraction32)?,
            }),
            QuoteStyle::NoteFuture => Ok(Quote {
                price: parse_note_future_price(number.as_str(), fraction, fraction32)?,
            }),
            QuoteStyle::ShortNoteFuture => Ok(Quote {
                price: parse_short_term_note_future_price(number.as_str(), fraction, fraction32)?,
            }),
            _ => Err(ParseError::Style),
        }
    }
}



#[cfg(test)]
mod tests {
    use crate::{style::QuoteStyle, error::ParseError};

    use super::Quote;

    #[test]
    fn parse_decimal() {
        let parsed_price = Quote::parse("123.45", QuoteStyle::Detect);
        assert_eq!(parsed_price.unwrap().price, 123.45);
    }

    #[test]
    fn parse_unqualified_str() {
        let parsed_price = Quote::parse("tum4", QuoteStyle::Detect);
        assert!(parsed_price.is_err());
    }

    #[test]
    fn parse_default_bond_quote() {
        let parsed_price = Quote::parse("104-04+", QuoteStyle::Detect);
        assert_eq!(parsed_price.unwrap().price, 104.125);
    }

    #[test]
    fn parse_default_stock_quote_with_comma() {
        let result = Quote::parse("104,04", QuoteStyle::Detect);
        assert_eq!(Err(ParseError::Number), result);
    }
}