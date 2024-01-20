use std::convert::Infallible;
//
// Quote Parser
//
use std::str::FromStr;
use crate::regex::Regex;
use crate::quotestyle::QuoteStyle;
use crate::error::ParseError;

#[derive(Debug, PartialEq, Default)]
struct Quote {
    price: f64,
}
impl FromStr for Quote {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Quote::parse(s, QuoteStyle::Detect)
    }
}
impl Quote {
    fn parse(s: &str, quotestyle: QuoteStyle) -> Result<Self, ParseError> {
        if let Ok(result) = s.parse::<f64>() {
            return Ok(Quote { price: result });
        }

        let re = Regex::new(r"(?P<number>^\d+)(?P<delimiter_frac>[\.\-\'])?(?P<fraction>\d{2})?(?P<delimiter32>\'?)(?P<fraction32>[\d+,\+])?").unwrap();

        let Some(captures) = re.captures(s) else {
            return Err(ParseError::UnexpectedToken);
        };

        let _number: i32 = match captures.name("number") {
            Some(number_str) => number_str.as_str().parse::<i32>().unwrap(),
            None => 0,
        };

        let _delimiter32 = match captures.name("delimiter32") {
            Some(delimiter32_str) => delimiter32_str.as_str(),
            None => "",
        };

        let _fraction: i32 = match captures.name("fraction") {
            Some(fraction_str) => fraction_str.as_str().parse::<i32>().unwrap(),
            None => 0,
        };

        let _fraction32: &str = match captures.name("fraction32") {
            Some(fraction32_str) => fraction32_str.as_str(),
            None => ""
        };

        let _delimiter32: &str = match captures.name("delimiter32") {
            Some(delimiter32_str) => delimiter32_str.as_str(),
            None => "",
        };

        let _delimiter_frac: &str = match captures.name("delimiter_frac") {
            Some(delimiter_frac_str) => delimiter_frac_str.as_str(),
            None => "",
        };

        return Ok(Quote::default());
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