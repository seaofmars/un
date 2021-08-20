use crate::error;
use crate::message::Message;

pub fn serialise_message(message: &Message) -> error::Result<String> {
    Ok(serde_json::to_string(message)?)
}