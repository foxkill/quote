//
//!crate styleparsers
//
// use crate::prelude::*;

pub fn parse_tresury_price(number: &str, fraction: &str, fraction32: &str) -> Result<f64> {
    let price = number.parse::<f64>() else {
        return Err(Error::InvalidNumber);
    };
}