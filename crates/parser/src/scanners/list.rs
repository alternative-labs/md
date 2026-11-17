use crate::cursor::Cursor;
use crate::scanners::{Outcome, Scanned, TokenScanner};
use crate::token::ListToken;

pub(crate) struct ListScanner;

impl TokenScanner<ListToken> for ListScanner {
    fn run(cursor: &Cursor<'_>) -> Outcome<ListToken> {
        let mut local_cursor = cursor.clone();
        let span_start = local_cursor.position();

        match local_cursor.peek() {
            Some('-' | '*' | '+') => {
                local_cursor.bump();

                if matches!(local_cursor.peek(), Some(' ' | '\t')) {
                    local_cursor.bump();
                    return Outcome::Accept(Scanned {
                        token: ListToken,
                        span: span_start..local_cursor.position(),
                    });
                }
            }
            Some(ch) if ch.is_ascii_digit() => {
                let mut digits = 0usize;

                while let Some(digit) = local_cursor.peek() {
                    if digit.is_ascii_digit() {
                        digits += 1;
                        local_cursor.bump();
                    } else {
                        break;
                    }
                }

                if digits > 0 && matches!(local_cursor.peek(), Some('.') | Some(')')) {
                    local_cursor.bump();

                    if matches!(local_cursor.peek(), Some(' ' | '\t')) {
                        local_cursor.bump();
                        return Outcome::Accept(Scanned {
                            token: ListToken,
                            span: span_start..local_cursor.position(),
                        });
                    }
                }
            }
            _ => {}
        }

        Outcome::Reject
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scans_bullet_list_markers() {
        for source in ["- item", "* item", "+ item"] {
            let cursor = Cursor::new(source);

            match ListScanner::run(&cursor) {
                Outcome::Accept(scanned) => assert_eq!(scanned.span, 0..2),
                Outcome::Reject => panic!("expected {source:?} to be accepted"),
            }

            assert_eq!(cursor.position(), 0);
        }
    }

    #[test]
    fn scans_ordered_list_markers() {
        let source = "1. item";
        let cursor = Cursor::new(source);

        match ListScanner::run(&cursor) {
            Outcome::Accept(scanned) => assert_eq!(scanned.span, 0..3),
            Outcome::Reject => panic!("expected {source:?} to be accepted"),
        }

        assert_eq!(cursor.position(), 0);
    }

    #[test]
    fn scans_multi_digit_ordered_list_markers() {
        for (source, marker_len) in [("10. item", 4), ("42) item", 4), ("123.\titem", 5)] {
            let cursor = Cursor::new(source);

            match ListScanner::run(&cursor) {
                Outcome::Accept(scanned) => assert_eq!(scanned.span, 0..marker_len),
                Outcome::Reject => panic!("expected {source:?} to be accepted"),
            }

            assert_eq!(cursor.position(), 0);
        }
    }

    #[test]
    fn rejects_non_list_markers() {
        for source in ["text", "-item", "1 item", "1a. item", "a. item"] {
            assert!(
                matches!(ListScanner::run(&Cursor::new(source)), Outcome::Reject),
                "expected {source:?} to be rejected"
            );
        }
    }
}
