//
//!# Style parsers
//!crate styleparsers
//! 
//

#![allow(unused)] 

use std::collections::HashMap;
use crate::error::ParseError;
use lazy_static::lazy_static;

macro_rules! hashmap {
    ($($key:expr => $value:expr),*) => ({
        let mut map = HashMap::new();
        $(map.insert($key, $value);)*
        map
    });
}
lazy_static! {
    static ref FRACTION_BOND: HashMap<char, f64> = hashmap!(
        '0' => 0.0,
        '1' => 1.0/8.0,
        '2' => 1.0/4.0,
        '3' => 3.0/8.0,
        '4' => 1.0/2.0,
        '+' => 1.0/2.0,
        '5' => 5.0/8.0,
        '6' => 3.0/4.0,
        '7' => 7.0/8.0
    );

    static ref FRACTION32_SHORT_TERM_NOTE: HashMap<char, f64> = hashmap!(
        '0' => 0.0,
        '1' => 0.0125,
        '2' => 0.25,
        '3' => 0.375,
        '5' => 0.5,
        '+' => 0.5,
        '6' => 0.625,
        '7' => 0.75,
        '8' => 0.875
    );

    static ref FRACTION32_NOTE: HashMap<char, f64> = hashmap!(
        '+' => 0.0,
        '0' => 0.0,
        '1' => 0.25,
        '2' => 0.5,
        '3' => 0.75
    );
}

pub fn parse_treasury_price(number: &str, fraction: &str, fraction32: &str) -> Result<f64, ParseError> {
    let Ok(price) = number.parse::<f64>() else {
        return Err(ParseError::InvalidNumber);
    };

    let Ok(fraction) = fraction.parse::<f64>() else {
        return Err(ParseError::InvalidNumber);
    };

    let fr32 = FRACTION_BOND.get(
        &fraction32.chars().next().unwrap_or('0')
    ).unwrap_or(&0.0);

    Ok(price + ((fraction + fr32)/32.0))
}

pub fn parse_short_term_note_future_price(number: &str, fraction: &str, fraction32: &str) -> Result<f64, ParseError> {
    let Ok(price) = number.parse::<f64>() else {
        return Err(ParseError::InvalidNumber);
    };

    let Ok(fraction) = fraction.parse::<f64>() else {
        return Err(ParseError::InvalidNumber);
    };

    let c = fraction32.chars().next().unwrap_or('0');
    let fr32 = FRACTION32_SHORT_TERM_NOTE.get(&c).unwrap_or(&0.0);

    Ok(price + ((fraction + fr32)/32.0))
}

/// Parse the price of a note future, ZF, ZN, TN
pub fn parse_note_future_price(number: &str, fraction: &str, fraction32: &str) -> Result<f64, ParseError> {
    let Ok(price) = number.parse::<f64>() else {
        return Err(ParseError::InvalidNumber);
    };

    let Ok(fraction) = fraction.parse::<f64>() else {
        return Err(ParseError::InvalidNumber);
    };

    let c = fraction32.chars().next().unwrap_or('0');
    let fr32 = FRACTION32_NOTE.get(&c).unwrap_or(&0.0);

    Ok(price + ((fraction + fr32)/32.0))
}


pub fn parse_bond_future_price(number: &str, fraction: &str, fraction32: &str) -> Result<f64, ParseError> {
    let Ok(price) = number.parse::<f64>() else {
        return Err(ParseError::InvalidNumber);
    };

    let Ok(fraction) = fraction.parse::<f64>() else {
        return Err(ParseError::InvalidNumber);
    };

    let _ = fraction32;

    Ok(price + (fraction/32.0))
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_tresury_price() {
        let expected = 103.140625;	
        let result = parse_treasury_price("103", "04", "+").unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_short_term_note_future_price() {
        let expected = 110.35546875;
        let result = parse_short_term_note_future_price("110", "11", "3").unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_note_future_price() {
        todo!();
    }

    #[test]
    fn test_parse_bond_future_price() {
        let expected = 110.34375;
        let result = parse_bond_future_price("110", "11", "").unwrap();
        assert_eq!(result, expected);
    }
}