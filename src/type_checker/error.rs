use std::fmt;

#[derive(Debug)]
pub enum TypeCheckerError {}

impl fmt::Display for TypeCheckerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "type checker error")
    }
}
