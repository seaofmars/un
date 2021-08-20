use clap::Clap;

#[derive(Clap)]
#[clap(version = "0.1.0", author = "Mars <seaofmars@protonmail.ch>")]
pub struct Opts {
    #[clap(short, long)]
    pub new: bool,
}