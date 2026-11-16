use crate::cursor::Cursor;
use crate::scanner::{
    HeadingScanner, ListScanner, NewlineScanner, Outcome, QuoteBlockScanner, TokenScanner,
};

// Markdown Parser
pub struct Parser<'a> {
    _source: &'a str,
    cursor: Cursor<'a>,
}

impl<'a> Parser<'a> {
    pub fn parse_hash(&mut self) {
        match HeadingScanner::run(&self.cursor) {
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

    pub fn parse_quotemark(&mut self) {
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

    pub fn parse_hyphen(&mut self) {
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

    pub fn parse_asterisk(&mut self) {
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

    pub fn parse_plus(&mut self) {
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

    pub fn parse_digit(&mut self) {
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

    pub fn parse_newline(&mut self) {
        match NewlineScanner::run(&self.cursor) {
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

    pub fn parse_backtick(&mut self) {
        self.cursor.bump();
    }

    pub fn parse_tilde(&mut self) {
        self.cursor.bump();
    }

    pub fn parse_lbracket(&mut self) {
        self.cursor.bump();
    }

    pub fn parse_rbracket(&mut self) {
        self.cursor.bump();
    }

    pub fn parse_lparen(&mut self) {
        self.cursor.bump();
    }

    pub fn parse_rparen(&mut self) {
        self.cursor.bump();
    }

    pub fn parse_exclamation(&mut self) {
        self.cursor.bump();
    }

    pub fn parse_underscore(&mut self) {
        self.cursor.bump();
    }

    pub fn parse_pipe(&mut self) {
        self.cursor.bump();
    }

    pub fn parse(&mut self) {
        while let Some(ch) = self.cursor.peek() {
            match ch {
                '#' => self.parse_hash(),
                '>' => self.parse_quotemark(),
                '-' => self.parse_hyphen(),
                '*' => self.parse_asterisk(),
                '+' => self.parse_plus(),
                '\n' => self.parse_newline(),
                '\r' => self.parse_newline(),
                '`' => self.parse_backtick(),
                '~' => self.parse_tilde(),
                '[' => self.parse_lbracket(),
                ']' => self.parse_rbracket(),
                '(' => self.parse_lparen(),
                ')' => self.parse_rparen(),
                '!' => self.parse_exclamation(),
                '_' => self.parse_underscore(),
                '|' => self.parse_pipe(),
                ch if ch.is_ascii_digit() => self.parse_digit(),
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
 * - [x] Paragraphs
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
 * - [ ] HTML
 * - [ ] JSX
 * - [ ] P2: Frontmatter parsing
 *
 *
 * API Features:
 * - [ ] Full-string parsing
 * - [ ] Block parsing
 * - [ ] Iterator (block-level)
 */
impl<'a> Parser<'a> {
    pub fn new(source: &'a str) -> Parser<'a> {
        let cursor: Cursor<'_> = Cursor::new(source);

        Parser {
            cursor,
            _source: source,
        }
    }
}
