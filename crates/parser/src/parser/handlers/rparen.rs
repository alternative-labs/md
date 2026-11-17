use crate::parser::Parser;

impl<'a> Parser<'a> {
    pub(in crate::parser) fn handle_rparen(&mut self) {
        self.cursor.bump();
    }
}
