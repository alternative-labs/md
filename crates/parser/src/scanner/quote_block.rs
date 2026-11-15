use crate::cursor::Cursor;
use crate::scanner::{Outcome, Scanned, TokenScanner};
use crate::token::QuoteBlockToken;

pub(crate) struct QuoteBlockScanner;
impl TokenScanner<QuoteBlockToken> for QuoteBlockScanner {
    fn scan(cursor: &Cursor<'_>) -> Outcome<QuoteBlockToken> {
        let mut local_cursor = cursor.clone();
        let span_start = cursor.position();

        if local_cursor.peek() == Some('>') {
            local_cursor.bump();

            Outcome::Accept(Scanned {
                token: QuoteBlockToken,
                span: span_start..local_cursor.position(),
            })
        } else {
            Outcome::Reject
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scans_quote_marker_without_consuming_the_source_cursor() {
        let cursor = Cursor::new("> quote");

        match QuoteBlockScanner::scan(&cursor) {
            Outcome::Accept(scanned) => assert_eq!(scanned.span, 0..1),
            Outcome::Reject => panic!("expected a quote marker to be accepted"),
        }

        assert_eq!(cursor.position(), 0);
    }

    #[test]
    fn rejects_non_quote_characters() {
        assert!(matches!(
            QuoteBlockScanner::scan(&Cursor::new("text")),
            Outcome::Reject
        ));
    }
}
