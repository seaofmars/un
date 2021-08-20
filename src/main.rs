mod args;

use clap::Clap;

use std::convert::TryFrom;

use libun::{crypto::Identity, message::Message, ExposeSecret};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: args::Opts = args::Opts::parse();

    if args.new {
        let identity = Identity::new();

        println!("ID: {}", identity.id);
        println!("Private key: {}", identity.priv_key.unwrap().expose_secret());

        return Ok(())
    }

    // ID: age1hzxjkcl7vlnnfszzt9tarprcnh45fentuj7h2v9zu5kcl6mru4rsun775h
    // Private key: AGE-SECRET-KEY-1WQVPN8PXAER3M4C36KDFUDW4ZP8MP5XQGLQ0KJ2SM4RYK0UTM4VQ5NNEPX
    let identity = std::env::var("UN_PRIVATE_KEY")?;
    let un = libun::Un::new(Identity::try_from(identity)?);

    // ID: age12yqrte7xaxnnhe5eqjxvfy8l5qr0zdk3mdwqh38snxg8w9ez34aqyrgtn0
    // Private key: AGE-SECRET-KEY-1WZC063RJTHA3L9D820RX429HYLDNUFJTDMFQCQRZ8GSH09A2C47QN5DSUX
    let to = Identity::try_from(
        String::from("AGE-SECRET-KEY-1WZC063RJTHA3L9D820RX429HYLDNUFJTDMFQCQRZ8GSH09A2C47QN5DSUX"))?;

    let encrypted_message = Message::new(un.me, to)
        .message_content(String::from("test"))
        .encrypt();

    Ok(())
}