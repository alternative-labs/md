use std::ops::{AddAssign, Range};

// Base pointer, representing the current pointer position in the source.
pub(crate) struct Cursor<'a> {
    ptr: usize,
    source: &'a str,
}
impl<'a> Cursor<'a> {
    pub fn new(source: &'a str) -> Cursor<'a> {
        Self { ptr: 0, source }
    }

    pub fn bump(&mut self) -> Option<char> {
        // This will early return None if the source is exhausted (with '?' operator). NEAT!
        let ch: char = self.peek()?;

        self.ptr += ch.len_utf8();

        Some(ch)
    }

    pub fn peek(&self) -> Option<char> {
        self.source.get(self.ptr..)?.chars().next()
    }

    pub fn position(&self) -> usize {
        self.ptr
    }
}

impl Clone for Cursor<'_> {
    fn clone(&self) -> Self {
        Cursor {
            ptr: self.ptr,
            source: self.source,
        }
    }
}

impl AddAssign<Range<usize>> for Cursor<'_> {
    fn add_assign(&mut self, span: Range<usize>) {
        assert_eq!(
            self.ptr, span.start,
            "Cursor position must match the start of the range being added."
        );
        assert!(
            span.end <= self.source.len(),
            "Range end must not exceed the source length."
        );
        assert!(
            self.source.is_char_boundary(span.end),
            "Range end must be at a valid character boundary."
        );

        self.ptr = span.end;
    }
}
