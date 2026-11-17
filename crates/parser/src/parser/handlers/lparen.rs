use crate::parser::Parser;

impl<'a> Parser<'a> {
    pub(in crate::parser) fn handle_lparen(&mut self) {
        self.cursor.bump();
    }
}
