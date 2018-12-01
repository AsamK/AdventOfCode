use std::fmt;

pub struct Error {
    repr: String,
}

impl Error {
    pub fn new(message: String) -> Self {
        Error { repr: message }
    }

    pub fn new_str(message: &str) -> Self {
        Error {
            repr: message.to_string(),
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self.repr, f)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.repr, f)
    }
}

impl std::error::Error for Error {}

pub type ACResult<T> = Result<T, Error>;
