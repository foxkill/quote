//! Quote parser as struct implementation.
#[derive(Debug, Default)]
pub struct Quote {
    price: f64,
}

impl PartialEq for Quote {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}

impl PartialEq<f64> for Quote {
    fn eq(&self, other: &f64) -> bool {
        self.price() == *other
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
/// use quote::parser::Quote;
/// use quote::style::Style;
///
/// let s = "103-04+";
/// let x = Quote::parse(s, Style::Bond).unwrap();
///
/// assert_eq!(103.140625, x.price());
/// ```
impl FromStr for Quote {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, ParseError> {
        Quote::parse(s, Style::Detect)
    }
}

impl Quote {
    /// Create a new instance of Quote.
    pub fn new() -> Self {
        // .ok_or_else(|| ParseError::InvalidString)
        Quote::default()
    }

    /// Try to parse a quote.
    pub fn parse(s: &str, quotestyle: Style) -> Result<Self, ParseError> {
        // First try parse a simple float.
        if let Ok(price) = s.parse::<f64>() {
            return Ok(Quote { price });
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
            Style::ShortNoteFuture => {
                parse_short_term_note_future_price(number, fraction, fraction32)
            }
            _ => return Err(ParseError::InvalidStyle),
        }?;

        Ok(Quote { price })
    }

    /// Return the price that was parsed.
    pub fn price(&self) -> f64 {
        self.price
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

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
        let e = Err(ParseError::InvalidNumber);
        assert_eq!(e, result);
    }

    #[test]
    fn can_compare() {
        let expected = 104.140625;
        let result = Quote::from_str("104-04+").unwrap();
        assert_eq!(result, expected);
    }
}
