//! # The Quote Parser
//! Module that contains the parse() function.

// #[cfg(feature = "cinterf")]
pub mod cparse;

mod macros;
mod style;
mod styleparsers;

use macros::extract_capture;
use macros::re;

// Export Style.
pub use self::style::Style;
// Use all style parsers.
use self::styleparsers::*;
// Use ParseError from Error module.
use super::error::ParseError;

const FRACTION_BOND: [f64; 8] = [0.0, 0.125, 0.25, 0.375, 0.5, 0.625, 0.75, 0.875];
const FRACTION_SHORT_TERM_NOTE: [f64; 9] = [0.0, 0.125, 0.25, 0.375, 0.5, 0.5, 0.625, 0.75, 0.875];
//--------------------------------0----1----2-----3----4---5-----6----7--
const FRACTION_NOTE: [f64; 8] = [0.0, 0.0, 0.25, 0.0, 0.5, 0.5, 0.0, 0.75];

const RE: &str = concat!(
    r"(?P<number>^\d+)(?P<delimiter_frac>[\.\-\'])?",
    r"(?P<fraction>\d{2})?(?P<delimiter32>\'?)(?P<fraction32>\d+|\+)?"
);

/**
This function allow to parse tresury bond quotes and treasury
future quotes.

If parsing succeeds, return the value inside [`Ok`], otherwise
when the string is ill-formatted return an error specific to the
inside [`Err`]. The error type is specific to the implementation of the trait.

# Examples

Basic usage with [`parse`] that resturn an [`f64`] if successful.

```
use quoteparser::prelude::*;

let s = "103-04+";
let quote = parse(s, Style::Bond).unwrap();

assert_eq!(103.140625, quote);
```
*/
pub fn parse(s: &str, quotestyle: Style) -> Result<f64, ParseError> {
    // First try to parse a simple float.
    if let Ok(price) = s.parse::<f64>() {
        return Ok(price);
    };

    // Guard
    let Some(captures) = re!(RE).captures(s) else {
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
        Style::Bond => parse_quote(number, fraction, fraction32, &FRACTION_BOND),
        Style::BondFuture => parse_quote(number, fraction, fraction32, &FRACTION_NOTE),
        Style::NoteFuture => parse_quote(number, fraction, fraction32, &FRACTION_NOTE),
        Style::ShortNoteFuture => parse_quote(number, fraction, fraction32, &FRACTION_SHORT_TERM_NOTE)
        _ => return Err(ParseError::InvalidStyle),
    }?;

    Ok(price)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_parse_a_decimal_value() {
        let result = parse("123.45", Style::default()).unwrap();
        let expected = 123.45;
        assert_eq!(result, expected);
    }

    #[test]
    fn it_should_return_an_error_for_an_unqualified_str() {
        let parsed_price = parse("tum4", Style::default());
        assert!(parsed_price.is_err());
    }

    #[test]
    fn it_should_parse_a_bond_quote() {
        let expected = 103.125;
        let result = parse("103-04", Style::default()).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn it_should_parse_a_bond_quote_denoted_as_half() {
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
    fn it_should_parse_a_bond_quote_with_a_fraction() {
        let expected = 103.792968750;
        let result = parse("103-253", Style::default()).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn it_should_should_return_an_error_with_an_invalid_fraction() {
        let expected = Err(ParseError::InvalidFraction32);
        let result = parse("103'25'9", Style::NoteFuture);
        assert_eq!(expected, result);
    }

    #[test]
    fn it_should_parse_a_bond_future_quote() {
        let expected = 126.78125;
        let result = parse("126'25", Style::default()).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn it_should_parse_a_ten_year_future_note() {
        let expected = 111.359375;
        let result = parse("111'11'5", Style::default()).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn it_should_parse_a_quote_with_an_invalid_null_prefix_in_fraction32() {
        let result = parse("123'04'05", Style::default()).unwrap();
        let expected = parse("123'04'5", Style::NoteFuture).unwrap();
        assert_eq!(result, expected);
    }
}
