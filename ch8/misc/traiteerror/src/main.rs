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

fn main() -> Result<(), UpStreamError> {
    let _f = File::open("invisible.txt")
        .map_err(UpStreamError::IO)?;

    let _localhost = "::1"
        .parse::<Ipv6Addr>()
        .map_err(UpStreamError::Parsing)?;


    Ok(())
}
