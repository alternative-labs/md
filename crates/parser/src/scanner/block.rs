use std::ops::Range;

pub(crate) enum Kind {
    Heading,
    Paragraph,
    Frontmatter,
}

pub(crate) struct Block {
    kind: Kind,
    span: Range<usize>,
}

impl Block {
    pub fn new(start: usize, kind: Kind) -> Block {
        Block {
            kind: kind,
            span: start..start,
        }
    }
}
