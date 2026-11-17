use crate::parser::Parser;

impl<'a> Parser<'a> {
    pub(in crate::parser) fn handle_backtick(&mut self) {
        self.cursor.bump();
    }
}
