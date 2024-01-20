//
// error.rs
//
#[derive(Debug)]
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