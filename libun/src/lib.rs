pub mod crypto;
pub mod serialise;
pub mod message;
pub mod error;

pub use secrecy::ExposeSecret;

use message::Message;
use error::Error;
use crypto::Identity;

pub struct Un {
    pub me: Identity,
}

impl Un {
    pub fn new(id: Identity) -> Un {
        Un { me: id }
    }

    pub fn recv() -> Message {
        unimplemented!()
    }
    
    pub fn send(message: Message) -> Result<(), Error> {
        unimplemented!()
    }
}