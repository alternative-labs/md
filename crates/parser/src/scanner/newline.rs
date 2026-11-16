use crate::cursor::Cursor;
use crate::scanner::{Outcome, Scanned, TokenScanner};
use crate::token::NewlineToken;

pub(crate) struct NewlineScanner;
impl TokenScanner<NewlineToken> for NewlineScanner {
    fn run(cursor: &Cursor) -> Outcome<NewlineToken> {
        let mut scan_cursor: Cursor<'_> = cursor.clone();
        let span_start = scan_cursor.position();

        let next_char: Option<char> = scan_cursor.peek();
        if matches!(next_char, Some('\n' | '\r')) {
            scan_cursor.bump();

            if next_char == Some('\r') && scan_cursor.peek() == Some('\n') {
                scan_cursor.bump();
            }

            Outcome::Accept(Scanned {
                token: NewlineToken,
                span: span_start..scan_cursor.position(),
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
    fn scans_line_feeds_and_carriage_returns() {
        for (source, expected_span) in [("\n", 0..1), ("\r", 0..1), ("\r\n", 0..2)] {
            let cursor: Cursor<'_> = Cursor::new(source);

            match NewlineScanner::run(&cursor) {
                Outcome::Accept(scanned) => assert_eq!(scanned.span, expected_span),
                Outcome::Reject => panic!("expected {source:?} to be accepted"),
            }

            assert_eq!(cursor.position(), 0);
        }
    }

    #[test]
    fn rejects_non_newline_characters() {
        assert!(matches!(
            NewlineScanner::run(&Cursor::new("text")),
            Outcome::Reject
        ));
    }
}
