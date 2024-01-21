//
//!crate styleparsers
use std::collections::HashMap;
use crate::error::ParseError;
use lazy_static::lazy_static;

lazy_static! {
    static ref FRACTION_BOND: HashMap<&'static str, f64> = {
        let mut map = HashMap::new();
        map.insert("0", 0.0);
        map.insert("1", 1.0/8.0);
        map.insert("2", 1.0/4.0);
        map.insert("3", 3.0/8.0);
        map.insert("4", 1.0/2.0);
        map.insert("+", 1.0/2.0);
        map.insert("5", 5.0/8.0);
        map.insert("6", 3.0/4.0);
        map.insert("7", 7.0/8.0);
        map
    };
}

pub fn parse_tresury_price(number: &str, fraction: &str, fraction32: &str) -> Result<f64, ParseError> {
    let Ok(price) = number.parse::<f64>() else {
        return Err(ParseError::InvalidString);
    };

    let Ok(fraction) = fraction.parse::<f64>() else {
        return Err(ParseError::InvalidString);
    };

    let fr32 = FRACTION_BOND.get(fraction32).unwrap_or(&0.0);
    
    Ok(price + ((fraction + fr32)/32.0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_tresury_price() {
        let expected = 103.140625;	
        let result = parse_tresury_price("103", "04", "+").unwrap();
        assert_eq!(result, expected);
    }
}