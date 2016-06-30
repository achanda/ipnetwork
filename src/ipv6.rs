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
}
