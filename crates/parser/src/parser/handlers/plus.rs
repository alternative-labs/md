use crate::scanners::{ListScanner, Outcome, TokenScanner};

use crate::parser::Parser;

impl<'a> Parser<'a> {
    pub(in crate::parser) fn handle_plus(&mut self) {
        match ListScanner::run(&self.cursor) {
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
