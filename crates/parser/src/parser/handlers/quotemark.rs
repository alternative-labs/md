use crate::scanners::{Outcome, QuoteBlockScanner, TokenScanner};

use crate::parser::Parser;

impl<'a> Parser<'a> {
    pub(in crate::parser) fn handle_quotemark(&mut self) {
        match QuoteBlockScanner::run(&self.cursor) {
            Outcome::Accept(scanned) => {
                self.cursor += scanned.span;
                print!("{}", scanned.token);
            }
            Outcome::Reject => {
                if let Some(ch) = self.cursor.peek() {
                    print!("{}", ch);
                }

                self.cursor.bump();
            }
        }
    }
}
