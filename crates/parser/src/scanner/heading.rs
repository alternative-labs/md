use crate::cursor::Cursor;
use crate::scanner::{Outcome, Scanned, TokenScanner};
use crate::token::HeadingToken;

pub(crate) struct HeadingScanner;
impl TokenScanner<HeadingToken> for HeadingScanner {
    fn scan(cursor: &Cursor) -> Outcome<HeadingToken> {
        let mut cursor: Cursor<'_> = cursor.clone();
        let span_start: usize = cursor.position();

        if cursor.peek() != Some('#') {
            return Outcome::Reject;
        }

        let mut level: u8 = 0;

        while let Some('#') = cursor.peek() {
            if level == 6 {
                return Outcome::Reject;
            }

            level += 1;
            cursor.bump();
        }

        if cursor.peek() == Some(' ') {
            cursor.bump();
        } else {
            return Outcome::Reject;
        }

        let span_end = cursor.position();
        let span = span_start..span_end;

        Outcome::Accept(Scanned {
            span,
            token: HeadingToken { level },
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scans_heading_levels_one_through_six() {
        for level in 1_u8..=6 {
            let source = format!("{} title", "#".repeat(usize::from(level)));
            let cursor = Cursor::new(&source);

            match HeadingScanner::scan(&cursor) {
                Outcome::Accept(scanned) => {
                    assert_eq!(scanned.token.level, level);
                    assert_eq!(scanned.span, 0..usize::from(level) + 1);
                }
                Outcome::Reject => panic!("expected an H{level} heading to be accepted"),
            }

            assert_eq!(cursor.position(), 0);
        }
    }

    #[test]
    fn scans_only_the_heading_prefix() {
        let source = "# title with 🦀";
        let cursor = Cursor::new(source);

        match HeadingScanner::scan(&cursor) {
            Outcome::Accept(scanned) => {
                assert_eq!(scanned.token.level, 1);
                assert_eq!(scanned.span, 0..2);
            }
            Outcome::Reject => panic!("expected a heading to be accepted"),
        }
    }

    #[test]
    fn rejects_invalid_heading_prefixes() {
        for source in ["text", "#heading", "#\ttab", "####### too many"] {
            assert!(
                matches!(HeadingScanner::scan(&Cursor::new(source)), Outcome::Reject),
                "expected {source:?} to be rejected"
            );
        }
    }
}
