use crate::parser::Parser;

impl<'a> Parser<'a> {
    pub(in crate::parser) fn handle_tilde(&mut self) {
        self.cursor.bump();
    }
}
