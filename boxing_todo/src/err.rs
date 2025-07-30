use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum ParseErr {
    // expected public fields
    Empty,
    Malformed(Box<dyn Error>),
}

impl Display for ParseErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", "Failed to parse todo file")
    }
}

impl Error for ParseErr {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
           ParseErr::Empty => None, 
           ParseErr::Malformed(e)=> Some(e.as_ref())
        }
    }
}

#[derive(Debug)]
pub struct ReadErr {
    // expected public fields
    pub  child_err: Box<dyn Error>,
}

impl Display for ReadErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.child_err)
    }
}

impl Error for ReadErr {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(self.child_err.as_ref())
    }
}
