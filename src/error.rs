//!crate error

// #![allow(unused)] // for starting project only, remove later.

#[derive(Debug, PartialEq)]
pub enum ParseError {
    InvalidCharacter,
    InvalidNumber,
    InvalidString,
    InvalidSymbol,
    InvalidToken,
    InvalidType,
    MissingToken,
    UnexpectedToken,
}