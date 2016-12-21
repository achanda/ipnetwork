use std::cmp;
use std::fmt;
use std::net::Ipv6Addr;
use std::str::FromStr;

use common::{IpNetworkError, cidr_parts, parse_prefix};

const IPV6_BITS: u8 = 128;
const IPV6_SEGMENT_BITS: u8 = 16;

/// Represents a network range where the IP addresses are of v6
#[derive(Debug,Clone,Copy,Hash,PartialEq,Eq)]
pub struct Ipv6Network {
    addr: Ipv6Addr,
    prefix: u8,
}

impl Ipv6Network {
    /// Constructs a new `Ipv6Network` from any `Ipv6Addr` and a prefix denoting the network size.
    /// If the prefix is larger than 128 this will return an `IpNetworkError::InvalidPrefix`.
    pub fn new(addr: Ipv6Addr, prefix: u8) -> Result<Ipv6Network, IpNetworkError> {
        if prefix > IPV6_BITS {
            Err(IpNetworkError::InvalidPrefix)
        } else {
            Ok(Ipv6Network {
                addr: addr,
                prefix: prefix,
            })
        }
    }

    pub fn ip(&self) -> Ipv6Addr {
        self.addr
    }

    pub fn prefix(&self) -> u8 {
        self.prefix
    }

    /// Returns the mask for this `Ipv6Network`.
    /// That means the `prefix` most significant bits will be 1 and the rest 0
    ///
    /// # Examples
    ///
    /// ```
    /// use std::net::Ipv6Addr;
    /// use ipnetwork::Ipv6Network;
    ///
    /// let net: Ipv6Network = "ff01::0/32".parse().unwrap();
    /// assert_eq!(net.mask(), Ipv6Addr::new(0xffff, 0xffff, 0, 0, 0, 0, 0, 0));
    /// ```
    pub fn mask(&self) -> Ipv6Addr {
        // Ipv6Addr::from is only implemented for [u8; 16]
        let mut segments = [0; 16];
        for (i, segment) in segments.iter_mut().enumerate() {
            let bits_remaining = self.prefix.saturating_sub(i as u8 * 8);
            let set_bits = cmp::min(bits_remaining, 8);
            *segment = !(0xff as u16 >> set_bits) as u8;
        }
        Ipv6Addr::from(segments)
    }

    /// Checks if a given `Ipv6Addr` is in this `Ipv6Network`
    ///
    /// # Examples
    ///
    /// ```
    /// use std::net::Ipv6Addr;
    /// use ipnetwork::Ipv6Network;
    ///
    /// let net: Ipv6Network = "ff01::0/32".parse().unwrap();
    /// assert!(net.contains(Ipv6Addr::new(0xff01, 0, 0, 0, 0, 0, 0, 0x1)));
    /// assert!(!net.contains(Ipv6Addr::new(0xffff, 0, 0, 0, 0, 0, 0, 0x1)));
    /// ```
    pub fn contains(&self, ip: Ipv6Addr) -> bool {
        let a = self.addr.segments();
        let b = ip.segments();
        let addrs = Iterator::zip(a.iter(), b.iter());
        self.mask().segments().iter().zip(addrs).all(|(mask, (a, b))| a & mask == b & mask)
    }
}

impl FromStr for Ipv6Network {
    type Err = IpNetworkError;
    fn from_str(s: &str) -> Result<Ipv6Network, IpNetworkError> {
        let (addr_str, prefix_str) = cidr_parts(s)?;
        let addr = Ipv6Addr::from_str(addr_str).map_err(|_| IpNetworkError::InvalidAddr(addr_str.to_string()))?;
        let prefix = parse_prefix(prefix_str, IPV6_BITS)?;
        Ipv6Network::new(addr, prefix)
    }
}

impl fmt::Display for Ipv6Network {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}/{}", self.ip(), self.prefix())
    }
}

/// Converts a `Ipv6Addr` network mask into a prefix.
/// If the mask is invalid this will return an `IpNetworkError::InvalidPrefix`.
pub fn ipv6_mask_to_prefix(mask: Ipv6Addr) -> Result<u8, IpNetworkError> {
    let mask = mask.segments();
    let mut mask_iter = mask.into_iter();

    // Count the number of set bits from the start of the address
    let mut prefix = 0;
    for &segment in &mut mask_iter {
        if segment == 0xffff {
            prefix += IPV6_SEGMENT_BITS;
        } else if segment == 0 {
            // Prefix finishes on a segment boundary
            break;
        } else {
            let prefix_bits = (!segment).leading_zeros() as u8;
            // Check that the remainder of the bits are all unset
            if segment << prefix_bits != 0 {
                return Err(IpNetworkError::InvalidPrefix);
            }
            prefix += prefix_bits;
            break;
        }
    }

    // Now check all the remaining bits are unset
    for &segment in mask_iter {
        if segment != 0 {
            return Err(IpNetworkError::InvalidPrefix);
        }
    }

    Ok(prefix)
}

#[cfg(test)]
mod test {
    use std::net::Ipv6Addr;
    use super::*;

    #[test]
    fn create_v6() {
        let cidr = Ipv6Network::new(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1), 24).unwrap();
        assert_eq!(cidr.prefix(), 24);
    }

    #[test]
    fn create_v6_invalid_prefix() {
        let cidr = Ipv6Network::new(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1), 129);
        assert!(cidr.is_err());
    }

    #[test]
    fn parse_v6() {
        let cidr: Ipv6Network = "::1/0".parse().unwrap();
        assert_eq!(cidr.ip(), Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1));
        assert_eq!(cidr.prefix(), 0);
    }

    #[test]
    fn parse_v6_2() {
        let cidr: Ipv6Network = "FF01:0:0:17:0:0:0:2/64".parse().unwrap();
        assert_eq!(cidr.ip(), Ipv6Addr::new(0xff01, 0, 0, 0x17, 0, 0, 0, 0x2));
        assert_eq!(cidr.prefix(), 64);
    }

    #[test]
    fn parse_v6_fail_addr() {
        let cidr: Option<Ipv6Network> = "2001::1::/8".parse().ok();
        assert_eq!(None, cidr);
    }

    #[test]
    fn parse_v6_fail_prefix() {
        let cidr: Option<Ipv6Network> = "::1/129".parse().ok();
        assert_eq!(None, cidr);
    }

    #[test]
    fn mask_v6() {
        let cidr = Ipv6Network::new(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0), 40).unwrap();
        let mask = cidr.mask();
        assert_eq!(mask, Ipv6Addr::new(0xffff, 0xffff, 0xff00, 0, 0, 0, 0, 0));
    }

    #[test]
    fn contains_v6() {
        let cidr = Ipv6Network::new(Ipv6Addr::new(0xff01, 0, 0, 0x17, 0, 0, 0, 0x2), 65).unwrap();
        let ip = Ipv6Addr::new(0xff01, 0, 0, 0x17, 0x7fff, 0, 0, 0x2);
        assert!(cidr.contains(ip));
    }

    #[test]
    fn not_contains_v6() {
        let cidr = Ipv6Network::new(Ipv6Addr::new(0xff01, 0, 0, 0x17, 0, 0, 0, 0x2), 65).unwrap();
        let ip = Ipv6Addr::new(0xff01, 0, 0, 0x17, 0xffff, 0, 0, 0x2);
        assert!(!cidr.contains(ip));
    }

    #[test]
    fn v6_mask_to_prefix() {
        let mask = Ipv6Addr::new(0xffff, 0xffff, 0xffff, 0, 0, 0, 0, 0);
        let prefix = ipv6_mask_to_prefix(mask).unwrap();
        assert_eq!(prefix, 48);
    }

    #[test]
    fn invalid_v6_mask_to_prefix() {
        let mask = Ipv6Addr::new(0, 0, 0xffff, 0xffff, 0, 0, 0, 0);
        let prefix = ipv6_mask_to_prefix(mask);
        assert!(prefix.is_err());
    }
}
