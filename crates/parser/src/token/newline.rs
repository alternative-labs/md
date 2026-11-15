use std::fmt::Display;

pub(crate) struct NewlineToken;

impl Display for NewlineToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(n)")
    }
}
