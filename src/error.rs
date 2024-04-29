use std::{error::Error, fmt, net::AddrParseError};

use crate::error::IpNetworkError::*;

/// Represents a bunch of errors that can occur while working with a `IpNetwork`
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum IpNetworkError {
    InvalidAddr(String),
    InvalidPrefix,
    InvalidCidrFormat(String),
    NetworkSizeError(NetworkSizeError),
}

impl fmt::Display for IpNetworkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            InvalidAddr(ref s) => write!(f, "invalid address: {s}"),
            InvalidPrefix => write!(f, "invalid prefix"),
            InvalidCidrFormat(ref s) => write!(f, "invalid cidr format: {s}"),
            NetworkSizeError(ref e) => write!(f, "network size error: {e}"),
        }
    }
}

impl Error for IpNetworkError {
    fn description(&self) -> &str {
        match *self {
            InvalidAddr(_) => "address is invalid",
            InvalidPrefix => "prefix is invalid",
            InvalidCidrFormat(_) => "cidr is invalid",
            NetworkSizeError(_) => "network size error",
        }
    }
}

impl From<AddrParseError> for IpNetworkError {
    fn from(e: AddrParseError) -> Self {
        InvalidAddr(e.to_string())
    }
}

/// Cannot convert an IPv6 network size to a u32 as it is a 128-bit value.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum NetworkSizeError {
    NetworkIsTooLarge,
}

impl fmt::Display for NetworkSizeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Network is too large to fit into an unsigned 32-bit integer!")
    }
}

impl Error for NetworkSizeError {}
