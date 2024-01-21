//!Crate prelude
//
#![allow(unused)]
use crate::error::{Error};
pub type Result<T> = core::result::Result<T, Error>;

// Gerneric Wrapper tuple struct for newtype pattern, mostly for Result.
pub struct W<T>(pub T);

// Personal preference
pub use std::format as f;