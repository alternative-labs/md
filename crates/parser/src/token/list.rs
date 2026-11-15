use std::fmt::Display;

pub(crate) struct ListToken;

impl Display for ListToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(list)")
    }
}
