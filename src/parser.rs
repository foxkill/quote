//! # The Quote Parser
//
//
#![allow(unuse)] // for starting projects only

mod style;
mod styleparsers;

use lazy_static::lazy_static;
use regex::Regex;

// Export Style.
pub use self::style::Style;
// Use all style parsers.
use self::styleparsers::*;
// Use ParseError from Error module.
use crate::error::ParseError;

// consider using lazy_regex::regex; Consider lazy regex insted of lazy_static?

macro_rules! extract_capture {
    ($caps:expr, $($name:ident), *) => {
        $(let $name: &str = $caps.name(stringify!($name)).map_or("", |f| f.as_str());)*
    };
}
lazy_static! {
    static ref QUOTE_EXPRESSION_RE: Regex = Regex::new(concat!(
        r"(?P<number>^\d+)(?P<delimiter_frac>[\.\-\'])?",
        r"(?P<fraction>\d{2})?(?P<delimiter32>\'?)(?P<fraction32>[\d+,\+])?"
    ))
    .unwrap();
}

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
        let parsed_price = parse("tum4", Style::Detect);
        assert!(parsed_price.is_err());
    }

    #[test]
    fn parse_bond_quote() {
        let expected = 103.125;
        let result = parse("103-04", Style::Detect).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn parse_default_bond_quote() {
        let expected = 104.140625;
        let result = parse("104-04+", Style::Detect).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn parse_quote_with_comma() {
        let result = parse("104,04", Style::default());
        assert!(matches!(result, Err(ParseError::InvalidNumber)));
    }
}
