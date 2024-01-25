//
//!# Style parsers
//!The module that contains the style parsers.
//! 
//
#![allow(unused)] 

use std::collections::HashMap;
use lazy_static::lazy_static;

use crate::error::ParseError;

type FractionMap = HashMap<char, f64>;

macro_rules! hashmap {
    ($($key:expr => $value:expr),*) => ({
        let mut map = HashMap::new();
        $(map.insert($key, $value);)*
        map
    });
}

lazy_static! {
    static ref FRACTION_BOND: FractionMap = hashmap!(
        '0' => 0.0,
        '1' => 0.125,
        '2' => 0.25,
        '3' => 0.375,
        '4' => 0.5,
        '+' => 0.5,
        '5' => 0.625,
        '6' => 0.75,
        '7' => 0.875
    );

    static ref FRACTION32_SHORT_TERM_NOTE: HashMap<char, f64> = hashmap!(
        '0' => 0.0,
        '1' => 0.125,
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
        '2' => 0.25,
        '5' => 0.5,
        '7' => 0.75
    );
}

fn parse_quote2(number: &str, fraction: &str, fraction32: &str, m: &FractionMap) -> Result<f64, ParseError> {
    let price = number.parse::<f64>().map_err(|_| ParseError::InvalidNumber)?;
    let fraction = fraction.parse::<f64>().map_err(|_| ParseError::InvalidNumber)?;
    let fr32 = m.get(&fraction32.chars().next().unwrap_or('0')).unwrap_or(&0.0);
    Ok(price + (fraction + fr32) / 32.0)
}
/// The work horse for parsing
fn parse_quote(number: &str, fraction: &str, fraction32: &str, m: &FractionMap) -> Result<f64, ParseError> {
    let Ok(price) = number.parse::<f64>() else {
        return Err(ParseError::InvalidNumber);
    };

    let Ok(fraction) = fraction.parse::<f64>() else {
        return Err(ParseError::InvalidFraction);
    };

    let fr32 = m.get(
        &fraction32.chars().next().unwrap_or('0')
    ).unwrap_or(&0.0);

    Ok(price + ((fraction + fr32)/32.0))
}

pub(crate) fn parse_treasury_price(number: &str, fraction: &str, fraction32: &str) -> Result<f64, ParseError> {
    parse_quote(number, fraction, fraction32, &FRACTION_BOND)
}

pub(crate) fn parse_short_term_note_future_price(number: &str, fraction: &str, fraction32: &str) -> Result<f64, ParseError> {
    parse_quote(number, fraction, fraction32, &FRACTION32_SHORT_TERM_NOTE)
}

/// Parse the price of a note future, ZF, ZN, TN
pub(crate) fn parse_note_future_price(number: &str, fraction: &str, fraction32: &str) -> Result<f64, ParseError> {
    parse_quote(number, fraction, fraction32, &FRACTION32_NOTE)
}

/// Parse treasury futures >20+ (TWE, ZB, UB)
pub(crate) fn parse_bond_future_price(number: &str, fraction: &str, fraction32: &str) -> Result<f64, ParseError> {
    parse_quote(number, fraction, "0", &FRACTION32_NOTE)
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
        let expected = 110.3671875;
        let result = parse_note_future_price("110", "11", "7").unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_bond_future_price() {
        let expected = 110.34375;
        let result = parse_bond_future_price("110", "11", "").unwrap();
        assert_eq!(result, expected);
    }
}