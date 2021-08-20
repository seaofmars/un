use std::fmt::{self, Display};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    BadIdentity,
    #[cfg(feature = "json")]
    Serialisation(serde_json::Error),
}

impl Display for Error {
    fn fmt(&self, w: &mut fmt::Formatter) -> fmt::Result {
        use Error::*;
        
        write!(w, "{}", match self {
            BadIdentity => String::from("bad identity"),
            #[cfg(feature = "json")]
            Serialisation(e) => e.to_string(),
        })
    }
}

impl std::error::Error for Error {}

#[cfg(feature = "json")]
impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Self::Serialisation(e)
    }
}