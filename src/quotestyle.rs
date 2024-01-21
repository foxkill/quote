//
// QuoteStyle
//

#[derive(Debug, PartialEq)]
pub enum QuoteStyle {
    Detect,
    Decimal,
    Bond,
    ShortNoteFuture,
    NoteFuture,
    BondFuture,
}

impl QuoteStyle {
    pub fn detect(fraction32: &str, delimiter_frac: &str, delimiter32: &str) -> QuoteStyle {
        if fraction32.contains('.') {
            QuoteStyle::Decimal
        } else if fraction32.contains('+') {
            QuoteStyle::Bond
        } else if delimiter32.contains('\'') && delimiter32.is_empty() {
            QuoteStyle::BondFuture
        } else if delimiter32.contains('\'') && delimiter32.contains('\'') {
            if fraction32.contains('1') || fraction32.contains('6') || fraction32.contains('8') {
                return QuoteStyle::ShortNoteFuture  
            } else if fraction32.contains('2') || fraction32.contains('5') || fraction32.contains('7') {
                return QuoteStyle::NoteFuture;
            } else {
                return QuoteStyle::Bond;
            }
        } else {
            QuoteStyle::Bond
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
        use super::QuoteStyle;
        assert_eq!(QuoteStyle::detect("", "", ""), QuoteStyle::Bond);
        assert_eq!(QuoteStyle::detect("325", ".", ""), QuoteStyle::Decimal);
        assert_eq!(QuoteStyle::detect("+", "", ""), QuoteStyle::Bond);
        assert_eq!(QuoteStyle::detect("2", "'", ""), QuoteStyle::BondFuture);
        assert_eq!(QuoteStyle::detect("1", "'", "'"), QuoteStyle::ShortNoteFuture);
    }
}
