use crate::cursor::Cursor;
use crate::scanner::{
    HeadingScanner, ListScanner, NewlineScanner, Outcome, QuoteBlockScanner, TokenScanner,
};

// Markdown Parser
pub struct Parser<'a> {
    source: &'a str,
    cursor: Cursor<'a>,
}
impl<'a> Parser<'a> {
    pub fn parse(&mut self) {
        while let Some(ch) = self.cursor.peek() {
            match ch {
                '#' => match HeadingScanner::scan(&self.cursor) {
                    Outcome::Accept(scanned) => {
                        self.cursor += scanned.span;
                        print!("{}", scanned.token);
                    }
                    Outcome::Reject => {
                        print!("{}", ch);
                        self.cursor.bump();
                    }
                },
                '\r' | '\n' => match NewlineScanner::scan(&self.cursor) {
                    Outcome::Accept(scanned) => {
                        self.cursor += scanned.span;
                        print!("{}", scanned.token);
                    }
                    Outcome::Reject => {
                        print!("{}", ch);
                        self.cursor.bump();
                    }
                },
                '>' => match QuoteBlockScanner::scan(&self.cursor) {
                    Outcome::Accept(scanned) => {
                        self.cursor += scanned.span;
                        print!("{}", scanned.token);
                    }
                    Outcome::Reject => {
                        print!("{}", ch);
                        self.cursor.bump();
                    }
                },
                '-' | '*' | '+' => match ListScanner::scan(&self.cursor) {
                    Outcome::Accept(scanned) => {
                        self.cursor += scanned.span;
                        print!("{}", scanned.token);
                    }
                    Outcome::Reject => {
                        print!("{}", ch);
                        self.cursor.bump();
                    }
                },
                ch if ch.is_ascii_digit() => match ListScanner::scan(&self.cursor) {
                    Outcome::Accept(scanned) => {
                        self.cursor += scanned.span;
                        print!("{}", scanned.token);
                    }
                    Outcome::Reject => {
                        print!("{}", ch);
                        self.cursor.bump();
                    }
                },
                _ => {
                    print!("{}", ch);
                    self.cursor.bump();
                }
            }
        }
    }
}

/**
 * Parser
 *
 * Blocks:
 * - [ ] Block start
 * - [ ] Block end
 *
 * Inline:
 * - [ ] Modifiers
 *
 * Block types:
 * - [x] Headings
 * - [ ] Paragraphs
 * - [x] QuoteBlocks
 * - [x] Lists
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
 * - [ ] Frontmatter parsing
 * - [ ] HTML
 * - [ ] JSX
 *
 *
 * API Features:
 * - [ ] Full-string parsing
 * - [ ] Block parsing
 * - [ ] Iterator (block-level)
 */
impl<'a> Parser<'a> {
    pub fn new(source: &'a str) -> Parser<'a> {
        let cursor = Cursor::new(source);

        Parser { source, cursor }
    }
}
