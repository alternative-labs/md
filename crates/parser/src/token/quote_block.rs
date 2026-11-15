use std::fmt::Display;

pub(crate) struct QuoteBlockToken;

impl Display for QuoteBlockToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(>)")
    }
}
