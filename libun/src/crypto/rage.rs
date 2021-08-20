use crate::crypto;
use crate::error;
use crate::message::Message;
use crate::serialise;

use age::x25519;

use std::convert::TryFrom;
use std::str::FromStr;

impl crypto::Identity {
    pub fn new() -> Self {
        let key = x25519::Identity::generate();

        Self {
            priv_key: Some(key.to_string()),
            id: key.to_public().to_string(),
        }
    }
}

impl TryFrom<String> for crypto::Identity {
    type Error = error::Error;

    fn try_from(from: String) -> Result<crypto::Identity, Self::Error> {
        let key = match x25519::Identity::from_str(&from) {
            Ok(k) => Ok(k),
            Err(_) => Err(Self::Error::BadIdentity),
        }?;

        Ok(Self {
            priv_key: Some(key.to_string()),
            id: key.to_public().to_string(),
        })
    }
}

pub fn encrypt_message(message: &Message) -> error::Result<Vec<u8>> {
    let serialised_message = serialise::serialise_message(message);
}