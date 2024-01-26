//
//!# Style parsers
//!The module that contains the style parsers.
//! 
//
// #[allow(unused)] 

use crate::error::ParseError;

const FRACTION_BOND: [f64; 8]= [0.0, 0.125, 0.25, 0.375, 0.5, 0.625, 0.75, 0.875];
const FRACTION_SHORT_TERM_NOTE: [f64; 9]= [0.0, 0.125, 0.25, 0.375, 0.5, 0.5, 0.625, 0.75, 0.875];
const FRACTION_NOTE: [f64; 7] = [0.0, 0.0, 0.25, 0.0, 0.5, 0.0, 0.75];

/// The work horse for parsing
fn parse_quote(number: &str, fraction: &str, fraction32: &str, m: &[f64]) -> Result<f64, ParseError> {
    let price = number.parse::<f64>().map_err(|_| ParseError::InvalidNumber)?;
    let fraction = fraction.parse::<f64>().map_err(|_| ParseError::InvalidNumber)?;

    let index = if fraction32.contains('+') { "4" } else { fraction32 };
    
    let i = index.parse::<usize>().map_err(|_| ParseError::InvalidFraction)?;

    let Some(fr32) = m.get(i) else {
        return Err(ParseError::InvalidFraction);
    };

    Ok(price + (fraction + fr32) / 32.0)
}

pub(crate) fn parse_treasury_price(number: &str, fraction: &str, fraction32: &str) -> Result<f64, ParseError> {
    parse_quote(number, fraction, fraction32, &FRACTION_BOND)
}

pub(crate) fn parse_short_term_note_future_price(number: &str, fraction: &str, fraction32: &str) -> Result<f64, ParseError> {
    parse_quote(number, fraction, fraction32, &FRACTION_SHORT_TERM_NOTE)
}

/// Parse the price of a note future, ZF, ZN, TN
pub(crate) fn parse_note_future_price(number: &str, fraction: &str, fraction32: &str) -> Result<f64, ParseError> {
    parse_quote(number, fraction, fraction32, &FRACTION_NOTE)
}

/// Parse treasury futures >20+ (TWE, ZB, UB)
pub(crate) fn parse_bond_future_price(number: &str, fraction: &str, fraction32: &str) -> Result<f64, ParseError> {
    let _ = fraction32;
    parse_quote(number, fraction, "0", &FRACTION_NOTE)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_should_correctly_handle_out_of_bounds_fractions() {
        let result = parse_quote("111", "04", "7", &FRACTION_NOTE);
        assert!(matches!(result, Err(ParseError::InvalidFraction)));
    }

    #[test]
    fn it_should_correctly_handle_invalid_fractions() {
        let result = parse_quote("111", "04", "-7", &FRACTION_NOTE);
        assert!(matches!(result, Err(ParseError::InvalidFraction)));


        let result = parse_quote("111", "04", "/", &FRACTION_NOTE);
        assert!(matches!(result, Err(ParseError::InvalidFraction)));
    }
}