//
// Quote Parser
//
use std::str::FromStr;
use std::collections::HashMap;
use crate::regex::Regex;
use crate::quotestyle::QuoteStyle;
use crate::error::ParseError;

// type QuoteParser = for<'a, 'b, 'c> fn(&'a str, &'b str, &'c str) -> Result<Quote, ParseError>;

#[derive(Debug, Default)]
struct Quote {
    price: f64,
    styleparser: HashMap<QuoteStyle, String>,
}

impl PartialEq for Quote {
    fn eq(&self, other: &Self) -> bool {
        self.price == other.price
    }
}
impl FromStr for Quote {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Quote::parse(s, QuoteStyle::Detect)
    }
}
impl Quote {
    fn parse_tresury_price(number: &str, fraction: &str, fraction32: &str) -> Result<Self, ParseError> {
        Ok(Quote::default())
    }

    fn new() -> Self {
        let mut styleparser = HashMap::new();

        styleparser.insert(QuoteStyle::Bond, String::from("Bond"));

        Quote { price: 0.0, styleparser }
    }
    fn parse(s: &str, quotestyle: QuoteStyle) -> Result<Self, ParseError> {
        if let Ok(result) = s.parse::<f64>() {
            return Ok(Quote::default());
        }

        let re = Regex::new(r"(?P<number>^\d+)(?P<delimiter_frac>[\.\-\'])?(?P<fraction>\d{2})?(?P<delimiter32>\'?)(?P<fraction32>[\d+,\+])?").unwrap();

        let Some(captures) = re.captures(s) else {
            return Err(ParseError::UnexpectedToken);
        };

        let number: i32 = match captures.name("number") {
            Some(number_str) => number_str.as_str().parse::<i32>().unwrap(),
            None => 0,
        };

        let delimiter_frac: &str = match captures.name("delimiter_frac") {
            Some(delimiter_frac_str) => delimiter_frac_str.as_str(),
            None => "",
        };

        let fraction: i32 = match captures.name("fraction") {
            Some(fraction_str) => fraction_str.as_str().parse::<i32>().unwrap(),
            None => 0,
        };

        let delimiter32 = match captures.name("delimiter32") {
            Some(delimiter32_str) => delimiter32_str.as_str(),
            None => "",
        };

        let fraction32: &str = match captures.name("fraction32") {
            Some(fraction32_str) => fraction32_str.as_str(),
            None => ""
        };

        let style = if quotestyle == QuoteStyle::Detect { QuoteStyle::detect(fraction32, delimiter_frac, delimiter32) } else { quotestyle };


        // if style == QuoteStyle::Bond {
        //     let mut price = _number as f64;
        //     price += _fraction as f64 / 100.0;
        //     price += _fraction32.replace(",", ".").parse::<f64>().unwrap() / 100.0;
        //     return Ok(Quote { price: price });
        // }

        // if style == QuoteStyle::Stock {
        //     let mut price = _number as f64;
        //     price += _fraction as f64 / 100.0;
        //     price += _fraction32.replace(",", ".").parse::<f64>().unwrap() / 100.0;
        //     return Ok(Quote { price: price });
        // }

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