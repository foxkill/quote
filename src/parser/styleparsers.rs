//
//!# Style parsers
//!The module that contains the style parsers.
//! 
//
// #[allow(unused)] 
use crate::error::ParseError;

/// The work horse for parsing
pub fn parse_quote(number: &str, fraction: &str, fraction32: &str, m: &[f64]) -> Result<f64, ParseError> {
    let price = number.parse::<f64>().map_err(|_| ParseError::InvalidNumber)?;
    let fr = fraction.parse::<f64>().map_err(|_| ParseError::InvalidNumber)?;

    let index = match fraction32 {
        s if s.starts_with('+') => "4",
        "" => "0",
        _ => fraction32
    }.parse::<usize>().map_err(|_| ParseError::InvalidFraction32)?;
    
    let Some(fr32) = m.get(index) else {
        return Err(ParseError::InvalidFraction32);
    };

    Ok(price + (fr + fr32) / 32.0)
}