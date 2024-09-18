use std::error;
use std::io;
use std::fmt;
use std::net;
use std::fs::File;
use std::net::Ipv6Addr;

#[derive(Debug)]
enum UpStreamError {
    IO(io::Error),
    Parsing(net::AddrParseError),
}

impl fmt::Display for UpStreamError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl error::Error for UpStreamError {}

impl From<io::Error> for UpStreamError {
    fn from(err: io::Error) -> Self {
        UpStreamError::IO(err)
    }
}

impl From<net::AddrParseError> for UpStreamError {
    fn from(err: net::AddrParseError) -> Self {
        UpStreamError::Parsing(err)
    }
}

fn main() -> Result<(), UpStreamError> {
    let _f = File::open("invisible.txt")?;
    let _localhost = "::1".parse::<Ipv6Addr>()?;

    Ok(())
}
