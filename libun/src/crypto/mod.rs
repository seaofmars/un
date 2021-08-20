#[cfg(feature = "age")]
mod rage;
#[cfg(feature = "age")]
use rage::encrypt_message;

mod encrypted_message;
pub use encrypted_message::EncryptedMessage;

mod identity;
pub use identity::{Identity, Generate};