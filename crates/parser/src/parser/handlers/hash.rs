use crate::scanners::{HeadingScanner, Outcome, TokenScanner};

use crate::parser::Parser;

impl<'a> Parser<'a> {
    pub(in crate::parser) fn handle_hash(&mut self) {
        // TODO: If not at the beginning of the block
        //       Treat hash as a regular character

        // Otherwise, perform the token run
        match HeadingScanner::run(&self.cursor) {
            Outcome::Accept(scanned) => {
                // TODO: Update the block kind to "heading"

                // Advance the cursor
                self.cursor += scanned.span;
            }
            Outcome::Reject => {
                // Safeguard, skip the character
                self.cursor.bump();
            }
        }
    }
}
