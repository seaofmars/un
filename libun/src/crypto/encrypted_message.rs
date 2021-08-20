use std::convert::TryFrom;

use crate::crypto;
use crate::serialise;
use crate::error;
use crate::message::Message;

pub struct EncryptedMessage(Vec<u8>);

impl TryFrom<&Message> for EncryptedMessage {
    type Error = error::Error;

    fn try_from(message: &Message) -> Result<EncryptedMessage, Self::Error> {
        Ok(Self(crypto::encrypt_message(message)?))
    }
}