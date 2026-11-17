mod handlers;

use crate::{cursor::Cursor, state::State};

// Markdown Parser
pub struct Parser<'a> {
    cursor: Cursor<'a>,
    state: State,
}

impl<'a> Parser<'a> {
    pub fn handle_next_char(&mut self, ch: char) {
        match ch {
            '#' => self.handle_hash(),
            '>' => self.handle_quotemark(),
            '-' => self.handle_hyphen(),
            '*' => self.handle_asterisk(),
            '+' => self.handle_plus(),
            '\n' => self.handle_newline(),
            '\r' => self.handle_newline(),
            '`' => self.handle_backtick(),
            '~' => self.handle_tilde(),
            '[' => self.handle_lbracket(),
            ']' => self.handle_rbracket(),
            '(' => self.handle_lparen(),
            ')' => self.handle_rparen(),
            '!' => self.handle_exclamation(),
            '_' => self.handle_underscore(),
            '|' => self.handle_pipe(),
            ch if ch.is_ascii_digit() => self.handle_digit(),
            _ => {
                print!("{}", ch);
                self.cursor.bump();
            }
        }
    }

    pub fn parse(&mut self) {
        while let Some(ch) = self.cursor.peek() {
            self.handle_next_char(ch);
        }
    }
}

/**
 * Parser - Streams markdown processing events from raw source (string/stream).
 * Note, parser doesn't build the AST, this will be a responsibility of a future "ast" crate.
 *
 * Blocks:
 * - [ ] Block start event
 * - [ ] Block end event
 *
 * Inline:
 * - [ ] Modifier start event
 * - [ ] Modifier end event
 *
 * Block types:
 * - [ ] Headings
 * - [ ] Paragraphs
 * - [ ] QuoteBlocks
 * - [ ] Lists
 * - [ ] Code Blocks (```, ~~~)
 * - [ ] Thematic break (---, ***, ___)
 * - [ ] P2: Callouts/Alerts
 *
 * Tables:
 * - [ ] Headers
 * - [ ] Dividers
 * - [ ] Body
 *
 * Inline types:
 * - [ ] Links
 * - [ ] Images
 * - [ ] Bold
 * - [ ] Italic
 * - [ ] Underline
 * - [ ] Strikethrough
 *
 * Extras:
 * - [ ] Frontmatter
 * - [ ] HTML
 * - [ ] JSX
 * - [ ] P2: Frontmatter parsing
 *
 *
 * API Features:
 * - [ ] Stream parsing
 * - [ ] Source string parsing
 * - [ ] Async event streaming
 */
impl<'a> Parser<'a> {
    pub fn from_string(source: &'a str) -> Parser<'a> {
        let cursor: Cursor<'_> = Cursor::new(source);
        let state: State = State {};

        Parser { cursor, state }
    }

    pub fn from_stream() /* -> Parser<'a> */
    {
        // TODO: Implement
    }
}
