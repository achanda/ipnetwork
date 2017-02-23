use std::net::Ipv4Addr;
use std::fmt;
use std::error::Error;

/// Represents a bunch of errors that can occur while working with a `IpNetwork`
#[derive(Debug,Clone,PartialEq,Eq)]
pub enum IpNetworkError {
    InvalidAddr(String),
    InvalidPrefix,
    InvalidCidrFormat(String),
}

impl fmt::Display for IpNetworkError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use IpNetworkError::*;
        match *self {
            InvalidAddr(ref s) => write!(f, "invalid address: {}", s),
            InvalidPrefix => write!(f, "invalid prifex"),
            InvalidCidrFormat(ref s) => write!(f, "invalid cidr format: {}", s),
        }
    }
}

impl Error for IpNetworkError {
    fn description(&self) -> &str {
        use IpNetworkError::*;
        match *self {
            InvalidAddr(_) => "address is invalid",
            InvalidPrefix => "prefix is invalid",
            InvalidCidrFormat(_) => "cidr is invalid",
        }

    }
}

pub fn cidr_parts(cidr: &str) -> Result<(&str, &str), IpNetworkError> {
    let parts = cidr.split('/').collect::<Vec<&str>>();
    if parts.len() == 2 {
        Ok((parts[0], parts[1]))
    } else {
        Err(IpNetworkError::InvalidCidrFormat(format!("CIDR must contain '/': {}", cidr)))
    }
}

pub fn parse_prefix(prefix: &str, max: u8) -> Result<u8, IpNetworkError> {
    let mask = prefix.parse::<u8>().map_err(|_| IpNetworkError::InvalidPrefix)?;
    if mask > max {
        Err(IpNetworkError::InvalidPrefix)
    } else {
        Ok(mask)
    }
}

pub fn parse_addr(addr: &str) -> Result<Ipv4Addr, IpNetworkError> {
    let addr_parts = addr.split('.').map(|b| b.parse::<u8>());
    let mut bytes = [0; 4];
    for (i, byte) in addr_parts.enumerate() {
        if i >= 4 {
            return Err(IpNetworkError::InvalidAddr(format!("More than 4 bytes: {}", addr)));
        }
        bytes[i] = byte.map_err(|_| {
                IpNetworkError::InvalidAddr(format!("All bytes not 0-255: {}", addr))
            })?;
    }
    Ok(Ipv4Addr::new(bytes[0], bytes[1], bytes[2], bytes[3]))
}
