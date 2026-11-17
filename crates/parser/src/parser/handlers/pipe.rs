use crate::parser::Parser;

impl<'a> Parser<'a> {
    pub(in crate::parser) fn handle_pipe(&mut self) {
        self.cursor.bump();
    }
}
