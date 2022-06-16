use std::fmt;

#[derive(Debug)]
pub enum CodegenError {
    BadPtrGen,
}

impl fmt::Display for CodegenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CodegenError::BadPtrGen => write!(f, "Bad ptr gen"),
        }
    }
}
