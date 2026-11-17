/**
 * Scanners recognize token prefixes at the cursor and report their spans
 * without consuming input.
 */
use std::ops::Range;

mod heading;
mod list;
mod newline;
mod quote_block;

pub(crate) use heading::HeadingScanner;
pub(crate) use list::ListScanner;
pub(crate) use newline::NewlineScanner;
pub(crate) use quote_block::QuoteBlockScanner;

pub(crate) enum Outcome<T> {
    Reject,
    Accept(Scanned<T>),
}

pub(crate) struct Scanned<T> {
    pub(crate) token: T,
    pub(crate) span: Range<usize>,
}

pub(crate) trait TokenScanner<T> {
    fn run(cursor: &Cursor<'_>) -> Outcome<T>;
}

use crate::cursor::Cursor;
