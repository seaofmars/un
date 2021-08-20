use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Identity {
    pub id: String,
    #[serde(skip_serializing, skip_deserializing)]
    pub priv_key: Option<secrecy::Secret<String>>,
}

pub trait Generate {
    fn new() -> Self;
}