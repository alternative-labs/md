use crate::scanners::{NewlineScanner, Outcome, TokenScanner};

use crate::parser::Parser;

impl<'a> Parser<'a> {
    pub(in crate::parser) fn handle_newline(&mut self) {
        match NewlineScanner::run(&self.cursor) {
            Outcome::Accept(scanned) => {
                self.cursor += scanned.span;
            }
            Outcome::Reject => {
                self.cursor.bump();
            }
        }
    }
}
