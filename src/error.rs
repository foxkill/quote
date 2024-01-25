//!crate error

#![allow(unused)] // for starting project only, remove later.

#[allow(clippy::enum_variant_names)]
#[derive(Debug, PartialEq)]
pub enum ParseError {
    InvalidQuote,
    InvalidNumber,
    InvalidFraction,
    InvalidStyle,
}

// impl std::error::Error for ParseError {
//     fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
//         None
//     }

//     fn description(&self) -> &str {
//         "description() is deprecated; use Display"
//     }

//     fn cause(&self) -> Option<&dyn std::error::Error> {
//         self.source()
//     }

//     fn provide<'a>(&'a self, request: &mut std::error::Request<'a>) {}
// }