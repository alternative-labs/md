use std::fmt::Display;

pub(crate) struct HeadingToken {
    pub level: u8,
}

impl Display for HeadingToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(H{})", self.level)
    }
}
