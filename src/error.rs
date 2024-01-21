//!crate error

#![allow(unused)] // for starting project only, remove later.

#[derive(Debug, PartialEq)]
pub enum ParseError {
    InvalidNumber,
    InvalidString,
    InvalidSymbol,
    InvalidType,
    UnexpectedToken,
}