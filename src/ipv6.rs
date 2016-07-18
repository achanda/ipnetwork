use std::cmp;
use std::fmt;
use std::net::Ipv6Addr;
use std::str::FromStr;

use common::{IpNetworkError, cidr_parts, parse_prefix};

const IPV6_BITS: u8 = 128;

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

    pub fn from_cidr(cidr: &str) -> Result<Ipv6Network, IpNetworkError> {
        let (addr_str, prefix_str) = try!(cidr_parts(cidr));
        let addr = try!(Ipv6Addr::from_str(addr_str)
            .map_err(|_| IpNetworkError::InvalidAddr(format!("{}", addr_str))));
        let prefix = try!(parse_prefix(prefix_str, IPV6_BITS));
        Self::new(addr, prefix)
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
    /// let net = Ipv6Network::from_cidr("ff01::0/32").unwrap();
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
    /// let net = Ipv6Network::from_cidr("ff01::0/32").unwrap();
    /// assert!(net.contains(Ipv6Addr::new(0xff01, 0, 0, 0, 0, 0, 0, 0x1)));
    /// assert!(!net.contains(Ipv6Addr::new(0xffff, 0, 0, 0, 0, 0, 0, 0x1)));
    /// ```
    pub fn contains(&self, ip: Ipv6Addr) -> bool {
        let a = self.addr.segments();
        let b = ip.segments();
        let addrs = Iterator::zip(a.iter(), b.iter());
        self.mask().segments().iter().zip(addrs).all(|(mask, (a, b))|
            a & mask == b & mask
        )
    }
}

impl fmt::Display for Ipv6Network {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}/{}", self.ip(), self.prefix())
    }
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
        let cidr = Ipv6Network::from_cidr("::1/0").unwrap();
        assert_eq!(cidr.ip(), Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1));
        assert_eq!(cidr.prefix(), 0);
    }

    #[test]
    fn parse_v6_2() {
        let cidr = Ipv6Network::from_cidr("FF01:0:0:17:0:0:0:2/64").unwrap();
        assert_eq!(cidr.ip(), Ipv6Addr::new(0xff01, 0, 0, 0x17, 0, 0, 0, 0x2));
        assert_eq!(cidr.prefix(), 64);
    }

    #[test]
    fn parse_v6_fail_addr() {
        let cidr = Ipv6Network::from_cidr("2001::1::/8");
        assert!(cidr.is_err());
    }

    #[test]
    fn parse_v6_fail_prefix() {
        let cidr = Ipv6Network::from_cidr("::1/129");
        assert!(cidr.is_err());
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
}
