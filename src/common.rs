use std::error::Error;
use std::fmt;
use std::net::Ipv4Addr;

/// Represents a bunch of errors that can occur while working with a `IpNetwork`
#[derive(Debug, Clone, PartialEq, Eq)]
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
            InvalidPrefix => write!(f, "invalid prefix"),
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

pub fn cidr_parts(cidr: &str) -> Result<(&str, Option<&str>), IpNetworkError> {
    let parts = cidr.split('/').collect::<Vec<&str>>();
    if parts.len() == 1 {
        Ok((parts[0], None))
    } else if parts.len() == 2 {
        Ok((parts[0], Some(parts[1])))
    } else {
        Err(IpNetworkError::InvalidCidrFormat(format!(
            "CIDR must contain a single '/': {}",
            cidr
        )))
    }
}

pub fn parse_prefix(prefix: &str, max: u8) -> Result<u8, IpNetworkError> {
    let mask = prefix
        .parse::<u8>()
        .map_err(|_| IpNetworkError::InvalidPrefix)?;
    if mask > max {
        Err(IpNetworkError::InvalidPrefix)
    } else {
        Ok(mask)
    }
}
