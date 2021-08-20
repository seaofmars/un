use std::convert::TryFrom;

use serde::{Serialize, Deserialize};

use crate::error;
use crate::crypto::Identity;
use crate::crypto::EncryptedMessage;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Message {
    destination: Identity,

    sender: Identity,
    message_content: String,
}

impl Message {
    pub fn new(sender: Identity, destination: Identity) -> Self {
        Message {
            sender,
            destination,
            message_content: String::new(),
        }
    }

    pub fn destination(mut self, d: Identity) -> Self {
        self.destination = d;

        self
    }

    pub fn message_content(mut self, mc: String) -> Self {
        self.message_content = mc;

        self
    }

    pub fn encrypt(&self) -> error::Result<EncryptedMessage> {
        EncryptedMessage::try_from(self)
    }
}