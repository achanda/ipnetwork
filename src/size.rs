use std::{cmp::Ordering, error::Error, fmt::Display};

/// Represents a generic network size. For IPv4, the max size is a u32 and for IPv6, it is a u128
#[derive(Debug, Clone, Copy, Hash)]
pub enum NetworkSize {
    V4(u32),
    V6(u128),
}
use NetworkSize::*;

// Conversions

impl From<u128> for NetworkSize {
    fn from(value: u128) -> Self {
        V6(value)
    }
}

impl From<u32> for NetworkSize {
    fn from(value: u32) -> Self {
        V4(value)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
/// Cannot convert an IPv6 network size to a u32 as it is a 128-bit value.
pub struct NetworkIsTooLargeError;

impl Display for NetworkIsTooLargeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Network is too large to fit into an unsigned 32-bit integer!")
    }
}
impl Error for NetworkIsTooLargeError {}

impl TryInto<u32> for NetworkSize {
    type Error = NetworkIsTooLargeError;
    fn try_into(self) -> Result<u32, Self::Error> {
        match self {
            V4(a) => Ok(a),
            V6(_) => Err(NetworkIsTooLargeError),
        }
    }
}

impl Into<u128> for NetworkSize {
    fn into(self) -> u128 {
        match self {
            V4(a) => a as u128,
            V6(a) => a,
        }
    }
}

// Equality/comparisons

impl PartialEq for NetworkSize {
    fn eq(&self, other: &Self) -> bool {
        let a: u128 = (*self).into();
        let b: u128 = (*other).into();
        a == b
    }
}

impl Ord for NetworkSize {
    fn cmp(&self, other: &Self) -> Ordering {
        let a: u128 = (*self).into();
        let b: u128 = (*other).into();
        return a.cmp(&b);
    }
}

impl PartialOrd for NetworkSize {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for NetworkSize {}
