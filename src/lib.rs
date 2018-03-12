//! The `ipnetwork` crate provides a set of APIs to work with IP CIDRs in
//! Rust. Implementation for IPv4 is more or less stable, IPv6 implementation
//! is still WIP.
#![cfg_attr(feature = "dev", feature(plugin))]
#![cfg_attr(feature = "dev", plugin(clippy))]
#![cfg_attr(all(feature = "ipv6-iterator", not(feature = "i128-extprim")), feature(i128_type))]
#![cfg_attr(all(feature = "ipv6-methods", not(feature = "i128-extprim")), feature(i128_type))]
#![crate_type = "lib"]
#![doc(html_root_url = "https://docs.rs/ipnetwork/0.12.7")]

#[cfg(all(any(feature = "ipv6-methods", feature = "ipv6-iterator"), feature = "i128-extprim"))]
extern crate extprim;

use std::fmt;
use std::net::IpAddr;

mod ipv4;
mod ipv6;
mod common;

use std::str::FromStr;

pub use ipv4::{Ipv4Network, ipv4_mask_to_prefix};
pub use ipv6::{Ipv6Network, ipv6_mask_to_prefix};
pub use common::IpNetworkError;

/// Represents a generic network range. This type can have two variants:
/// the v4 and the v6 case.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum IpNetwork {
    V4(Ipv4Network),
    V6(Ipv6Network),
}

impl IpNetwork {
    /// Constructs a new `IpNetwork` from a given `IpAddr` and a prefix denoting the
    /// network size. If the prefix is larger than 32 (for IPv4) or 128 (for IPv6), this
    /// will raise an `IpNetworkError::InvalidPrefix` error. Support for IPv6 is not
    /// complete yet.
    pub fn new(ip: IpAddr, prefix: u8) -> Result<IpNetwork, IpNetworkError> {
        match ip {
            IpAddr::V4(a) => Ok(IpNetwork::V4(Ipv4Network::new(a, prefix)?)),
            IpAddr::V6(a) => Ok(IpNetwork::V6(Ipv6Network::new(a, prefix)?)),
        }
    }

    /// Returns the IP part of a given `IpNetwork`
    pub fn ip(&self) -> IpAddr {
        match *self {
            IpNetwork::V4(ref a) => IpAddr::V4(a.ip()),
            IpNetwork::V6(ref a) => IpAddr::V6(a.ip()),
        }
    }

    /// Returns the prefix of the given `IpNetwork`
    ///
    /// # Example
    /// ```
    /// use ipnetwork::IpNetwork;
    ///
    /// assert_eq!(IpNetwork::V4("10.9.0.32/16".parse().unwrap()).prefix(), 16u8);
    /// assert_eq!(IpNetwork::V6("ff01::0/32".parse().unwrap()).prefix(), 32u8);
    /// ```
    pub fn prefix(&self) -> u8 {
        match *self {
            IpNetwork::V4(ref a) => a.prefix(),
            IpNetwork::V6(ref a) => a.prefix(),
        }
    }

    /// Returns the mask for this `IpNetwork`.
    /// That means the `prefix` most significant bits will be 1 and the rest 0
    ///
    /// # Example
    /// ```
    /// use ipnetwork::IpNetwork;
    /// use std::net::{Ipv4Addr, Ipv6Addr};
    ///
    /// let v4_net: IpNetwork = "10.9.0.32/16".parse().unwrap();
    /// assert_eq!(v4_net.mask(), Ipv4Addr::new(255, 255, 0, 0));
    /// let v6_net: IpNetwork = "ff01::0/32".parse().unwrap();
    /// assert_eq!(v6_net.mask(), Ipv6Addr::new(0xffff, 0xffff, 0, 0, 0, 0, 0, 0));
    ///```
    pub fn mask(&self) -> IpAddr {
        match *self {
            IpNetwork::V4(ref a) => IpAddr::V4(a.mask()),
            IpNetwork::V6(ref a) => IpAddr::V6(a.mask()),
        }
    }

    /// Returns true if the IP in this `IpNetwork` is a valid IPv4 address,
    /// false if it's a valid IPv6 address.
    ///
    /// # Example
    ///
    ///```
    /// use ipnetwork::IpNetwork;
    ///
    /// let v4: IpNetwork = IpNetwork::V4("10.9.0.32/16".parse().unwrap());
    /// assert_eq!(v4.is_ipv4(), true);
    /// assert_eq!(v4.is_ipv6(), false);
    ///```
    pub fn is_ipv4(&self) -> bool {
        match *self {
            IpNetwork::V4(_) => true,
            IpNetwork::V6(_) => false,
        }
    }

    /// Returns true if the IP in this `IpNetwork` is a valid IPv6 address,
    /// false if it's a valid IPv4 address.
    ///
    /// # Example
    ///
    ///```
    /// use ipnetwork::IpNetwork;
    ///
    /// let v6: IpNetwork = IpNetwork::V6("ff01::0/32".parse().unwrap());
    /// assert_eq!(v6.is_ipv6(), true);
    /// assert_eq!(v6.is_ipv4(), false);
    ///```
    pub fn is_ipv6(&self) -> bool {
        match *self {
            IpNetwork::V4(_) => false,
            IpNetwork::V6(_) => true,
        }
    }

    /// Checks if a given `IpAddr` is in this `IpNetwork`
    ///
    /// # Examples
    ///
    /// ```
    /// use std::net::IpAddr;
    /// use ipnetwork::IpNetwork;
    ///
    /// let net: IpNetwork = "127.0.0.0/24".parse().unwrap();
    /// let ip1: IpAddr = "127.0.0.1".parse().unwrap();
    /// let ip2: IpAddr = "172.0.0.1".parse().unwrap();
    /// let ip4: IpAddr = "::1".parse().unwrap();
    /// assert!(net.contains(ip1));
    /// assert!(!net.contains(ip2));
    /// assert!(!net.contains(ip4));
    /// ```
    pub fn contains(&self, ip: IpAddr) -> bool {
        match (*self, ip) {
            (IpNetwork::V4(net), IpAddr::V4(ip)) => net.contains(ip),
            (IpNetwork::V6(net), IpAddr::V6(ip)) => net.contains(ip),
            _ => false,
        }
    }
}

/// Tries to parse the given string into a `IpNetwork`. Will first try to parse
/// it as an `Ipv4Network` and if that fails as an `Ipv6Network`. If both
/// fails it will return an `InvalidAddr` error.
///
/// # Examples
///
/// ```
/// use std::net::Ipv4Addr;
/// use ipnetwork::{IpNetwork, Ipv4Network};
///
/// let expected = IpNetwork::V4(Ipv4Network::new(Ipv4Addr::new(10, 1, 9, 32), 16).unwrap());
/// let from_cidr: IpNetwork = "10.1.9.32/16".parse().unwrap();
/// assert_eq!(expected, from_cidr);
/// ```
impl FromStr for IpNetwork {
    type Err = IpNetworkError;
    fn from_str(s: &str) -> Result<IpNetwork, IpNetworkError> {
        if let Ok(net) = Ipv4Network::from_str(s) {
            Ok(IpNetwork::V4(net))
        } else if let Ok(net) = Ipv6Network::from_str(s) {
            Ok(IpNetwork::V6(net))
        } else {
            Err(IpNetworkError::InvalidAddr(s.to_string()))
        }
    }
}

impl From<Ipv4Network> for IpNetwork {
    fn from(v4: Ipv4Network) -> IpNetwork {
        IpNetwork::V4(v4)
    }
}

impl From<Ipv6Network> for IpNetwork {
    fn from(v6: Ipv6Network) -> IpNetwork {
        IpNetwork::V6(v6)
    }
}

impl From<IpAddr> for IpNetwork {
    fn from(addr: IpAddr) -> IpNetwork {
        match addr {
            IpAddr::V4(a) => IpNetwork::V4(Ipv4Network::from(a)),
            IpAddr::V6(a) => IpNetwork::V6(Ipv6Network::from(a)),
        }
    }
}

impl fmt::Display for IpNetwork {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            IpNetwork::V4(net) => net.fmt(f),
            IpNetwork::V6(net) => net.fmt(f),
        }
    }
}

/// Converts a `IpAddr` network mask into a prefix.
/// If the mask is invalid this will return an `IpNetworkError::InvalidPrefix`.
pub fn ip_mask_to_prefix(mask: IpAddr) -> Result<u8, IpNetworkError> {
    match mask {
        IpAddr::V4(mask) => ipv4_mask_to_prefix(mask),
        IpAddr::V6(mask) => ipv6_mask_to_prefix(mask),
    }
}
