//! # The Quote Parser
//! Module that contains the parse() function.
//
// #[allow(unused)] // for starting projects only
use lazy_static::lazy_static;
use regex::Regex;

mod style;
mod macros;
mod styleparsers;

use crate::extract_capture;

// Export Style.
pub use self::style::Style;
// Use all style parsers.
use self::styleparsers::*;
// Use ParseError from Error module.
use crate::error::ParseError;

// consider using lazy_regex::regex; Consider lazy regex insted of lazy_static?


lazy_static! {
    static ref QUOTE_EXPRESSION_RE: Regex = Regex::new(concat!(
        r"(?P<number>^\d+)(?P<delimiter_frac>[\.\-\'])?",
        r"(?P<fraction>\d{2})?(?P<delimiter32>\'?)(?P<fraction32>[\d+,\+])?"
    ))
    .unwrap();
}

/**
This function allow to parse tresury bond quotes and treasury
future quotes.

If parsing succeeds, return the value inside [`Ok`], otherwise
when the string is ill-formatted return an error specific to the
inside [`Err`]. The error type is specific to the implementation of the trait.

# Examples

Basic usage with [`parse`] that resturn an [`f64`] if successful.

```
use quote::parser::parse;
use quote::parser::Style;

let s = "103-04+";
let x = parse(s, Style::Bond).unwrap();

assert_eq!(103.140625, x);
```
*/
pub fn parse(s: &str, quotestyle: Style) -> Result<f64, ParseError> {
    // First try parse a simple float.
    if let Ok(price) = s.parse::<f64>() {
        return Ok(price);
    };

    // Guard
    let Some(captures) = QUOTE_EXPRESSION_RE.captures(s) else {
        return Err(ParseError::InvalidQuote);
    };

    extract_capture!(
        captures,
        number,
        delimiter_frac,
        fraction,
        delimiter32,
        fraction32
    );

    let style = if quotestyle == Style::Detect {
        Style::detect(fraction32, delimiter_frac, delimiter32)
    } else {
        quotestyle
    };

    let price = match style {
        Style::Bond => parse_treasury_price(number, fraction, fraction32),
        Style::BondFuture => parse_bond_future_price(number, fraction, fraction32),
        Style::NoteFuture => parse_note_future_price(number, fraction, fraction32),
        Style::ShortNoteFuture => parse_short_term_note_future_price(number, fraction, fraction32),
        _ => return Err(ParseError::InvalidStyle),
    }?;

    Ok(price)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_decimal() {
        let result = parse("123.45", Style::default()).unwrap();
        let expected = 123.45;
        assert_eq!(result, expected);
    }

    #[test]
    fn parse_unqualified_str() {
        let parsed_price = parse("tum4", Style::default());
        assert!(parsed_price.is_err());
    }

    #[test]
    fn parse_bond_quote() {
        let expected = 103.125;
        let result = parse("103-04", Style::default()).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn parse_default_bond_quote() {
        let expected = 104.140625;
        let result = parse("104-04+", Style::default()).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn parse_quote_with_comma() {
        let result = parse("104,04", Style::default());
        assert!(matches!(result, Err(ParseError::InvalidNumber)));
    }

    #[test]
    /// If you want to parse a short term future quote like a price from /ZT
    /// future, you have to be specific about the parse style.
    fn it_should_parse_short_term_note_future_quotes() {
        let expected = 102.578125;
        let result = parse("102'18'5", Style::ShortNoteFuture).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn it_should_parse_bond_quotes() {
        let expected = 103.792968750;
        let result = parse("103-253", Style::default()).unwrap();
        assert_eq!(result, expected);
    }

}
