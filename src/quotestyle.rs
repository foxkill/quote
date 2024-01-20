//
// QuoteStyle
//

#[derive(Debug, PartialEq)]
pub enum QuoteStyle {
    Detect,
    Bond,
    ShortNoteFuture,
    BondFuture,
}

impl QuoteStyle {
    pub fn detect(fraction32: &str, delimiter_frac: &str, delimiter32: &str) -> QuoteStyle {
        if fraction32.contains(".") {
            QuoteStyle::Bond
        } else if fraction32.contains(" ") {
            QuoteStyle::ShortNoteFuture
        } else {
            QuoteStyle::BondFuture
        }
    }
}

mod tests {
    #[test]
    fn test_detect() {
        use super::QuoteStyle;
        assert_eq!(QuoteStyle::detect("1/1", "/", ""), QuoteStyle::Bond);
        assert_eq!(QuoteStyle::detect("1/1", "/", " "), QuoteStyle::ShortNoteFuture);
        assert_eq!(QuoteStyle::detect("1/1", "/", "="), QuoteStyle::BondFuture);
    }
}
