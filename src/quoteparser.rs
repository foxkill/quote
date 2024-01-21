//
// Quote Parser
//
#![allow(unused)] // for starting project only

use std::str::FromStr;
use std::collections::HashMap;
use crate::regex::Regex;
use crate::quotestyle::QuoteStyle;
use crate::error::ParseError;
use crate::styleparsers::{parse_treasury_price, parse_short_term_note_future_price, parse_bond_future_price};

// type QuoteParser = for<'a, 'b, 'c> fn(&'a str, &'b str, &'c str) -> Result<Quote, ParseError>;

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

    fn parse(s: &str, quotestyle: QuoteStyle) -> Result<Self, ParseError> {
        if let Ok(price) = s.parse::<f64>() {
            return Ok(Quote { price });
        };

        // It's ok if it is not parsable. We'Ll continue to parse it via an regular expression.
        let re = Regex::new(concat!(
                r"(?P<number>^\d+)(?P<delimiter_frac>[\.\-\'])?",
                r"(?P<fraction>\d{2})?(?P<delimiter32>\'?)(?P<fraction32>[\d+,\+])?"
            )).unwrap();

        let Some(captures) = re.captures(s) else {
            return Err(ParseError::InvalidString);
        };

        let Some(number) = captures.name("number") else {
            return Err(ParseError::InvalidNumber);
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

        return match if quotestyle == QuoteStyle::Detect {
            QuoteStyle::detect(fraction32, delimiter_frac, delimiter32)
        } else {
            quotestyle
        } {
            QuoteStyle::Bond => Ok(Quote {
                 price: parse_treasury_price(number.as_str(), fraction, fraction32).unwrap(),
            }),
            QuoteStyle::BondFuture => Ok(Quote {
                price: parse_bond_future_price(number.as_str(), fraction, fraction32).unwrap(),
            }),
            QuoteStyle::ShortNoteFuture => Ok(Quote {
                price: parse_short_term_note_future_price(number.as_str(), fraction, fraction32).unwrap(),
            }),
            _ => Err(ParseError::UnexpectedToken),
        };
    }
}



#[cfg(test)]
mod tests {
    use crate::quotestyle::QuoteStyle;

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
        let parsed_price = Quote::parse("104,04", QuoteStyle::Detect);
        assert_eq!(parsed_price.unwrap().price, 104.04);
    }
}