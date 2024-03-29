//! #Quote Style
//!
//!crate:quotestyle
//! pub enum Stye - Enum which contains all possible kinds of parsing methods.
//

use std::fmt;

#[repr(C)]
#[derive(Debug, PartialEq, Default, Copy, Clone)]
pub enum Style {
    #[default]
    Detect,
    Decimal,
    Bond,
    ShortNoteFuture,
    NoteFuture,
    BondFuture,
}

impl fmt::Display for Style {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Style::Detect => write!(f, "Detect style of the quote"),
            Style::Decimal => write!(f, "Parse a decimal number"),
            Style::Bond => write!(f, "Bond"),
            Style::ShortNoteFuture => write!(f, "Short Note Future (2yr, 3yr, 5yr, 7yr)"),
            Style::NoteFuture => write!(f, "Note Future (10yr+)"),
            Style::BondFuture => write!(f, "Bond Future (20yr, 30yr)"),
        }
    }
}

impl Style {
    pub fn detect(fraction32: &str, delimiter_frac: &str, delimiter32: &str) -> Style {
        match (
            delimiter_frac.contains('.'),
            fraction32.contains('+'),
            delimiter_frac.contains('\''),
            delimiter32.contains('\''),
            !delimiter32.is_empty(),
        ) {
            (true, _, _, _, _) => Style::Decimal,
            (_, true, _, _, _) => Style::Bond,
            (_, _, true, false, _) => Style::BondFuture,
            (_, _, true, true, true) => match fraction32 {
                s if s.contains('2') || s.contains('5') || s.contains('7') => Style::NoteFuture,
                s if s.contains('1') || s.contains('3') || s.contains('6') || s.contains('8') => {
                    Style::ShortNoteFuture
                }
                _ => Style::Bond,
            },
            _ => Style::Bond,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_detect() {
        // (None, None, None, QuoteStyle.BOND), # Default
        // ('', '', '', QuoteStyle.BOND), # Default
        // ('325', '.', None, QuoteStyle.DECIMAL), # Decimal
        // ('+', '', '', QuoteStyle.BOND), # 108-04+
        // ('2', '\'', '', QuoteStyle.BOND_FUTURE), # 108'182
        use super::Style;
        assert_eq!(Style::detect("", "", ""), Style::Bond);
        assert_eq!(Style::detect("325", ".", ""), Style::Decimal);
        assert_eq!(Style::detect("+", "", ""), Style::Bond);
        assert_eq!(Style::detect("2", "'", ""), Style::BondFuture);
        assert_eq!(Style::detect("7", "'", "'"), Style::NoteFuture);
        assert_eq!(Style::detect("8", "'", "'"), Style::ShortNoteFuture);
    }
}