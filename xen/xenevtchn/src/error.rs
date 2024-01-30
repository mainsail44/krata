use std::io;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("kernel error")]
    Kernel(#[from] nix::errno::Errno),
    #[error("io issue encountered")]
    Io(#[from] io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
