use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum AdminErrorKind {
    AwsError,
    UnknownError,
    InputError,
}

#[derive(Debug)]
pub struct AdminError {
    pub kind: AdminErrorKind,
    pub msg: Option<String>,
}

impl fmt::Display for AdminError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string = self
            .msg
            .clone()
            .unwrap_or("Error with no description".to_string());
        write!(f, "{}", string)
    }
}

impl Error for AdminError {}
