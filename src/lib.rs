#![cfg_attr(feature = "dev", allow(unstable_features))]
#![cfg_attr(feature = "dev", feature(plugin))]
#![cfg_attr(feature = "dev", plugin(clippy))]
#![crate_type = "lib"]
#[allow(dead_code)]

use std::fmt;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::str::FromStr;

const IPV4_BITS: u8 = 32;
const IPV6_BITS: u8 = 128;

// A network
#[derive(Debug)]
pub enum IpNetwork {
    V4(Ipv4Network),
    V6(Ipv6Network),
}

pub struct Ipv4Network {
    addr: Ipv4Addr,
    prefix: u8,
}

pub struct Ipv6Network {
    addr: Ipv6Addr,
    prefix: u8,
}

#[derive(Debug)]
pub enum IpNetworkError {
    InvalidAddr(String),
    InvalidPrefix,
    InvalidCidrFormat(String),
}

impl Ipv4Network {
    /// Constructs a new `Ipv4Network` from any `Ipv4Addr` and a prefix denoting the network size.
    /// If the prefix is larger than 32 this will return an `IpNetworkError::InvalidPrefix`.
    pub fn new(addr: Ipv4Addr, prefix: u8) -> Result<Ipv4Network, IpNetworkError> {
        if prefix > IPV4_BITS {
            Err(IpNetworkError::InvalidPrefix)
        } else {
            Ok(Ipv4Network {
                addr: addr,
                prefix: prefix,
            })
        }
    }

    pub fn from_cidr(cidr: &str) -> Result<Ipv4Network, IpNetworkError> {
        let (addr_str, prefix_str) = try!(cidr_parts(cidr));
        let addr = try!(Self::parse_addr(addr_str));
        let prefix = try!(parse_prefix(prefix_str, IPV4_BITS));
        let new = try!(Self::new(addr, prefix));
        let (net, _) = new.network();
        if addr != net {
            Err(IpNetworkError::InvalidCidrFormat(format!("Host bits must be zero")))
        } else {
            Ok(new)
        }
    }

    pub fn ip(&self) -> Ipv4Addr {
        self.addr
    }

    pub fn prefix(&self) -> u8 {
        self.prefix
    }

    pub fn mask(&self) -> (Ipv4Addr, u32) {
        let prefix = self.prefix;
        let mask = !(0xffffffff as u64 >> prefix) as u32;
        (Ipv4Addr::from(mask), mask)
    }

    pub fn network(&self) -> (Ipv4Addr, u32) {
        let (_, mask) = self.mask();
        let ip = u32::from(self.addr) & mask;
        (Ipv4Addr::from(ip), ip)
    }

    pub fn broadcast(&self) -> (Ipv4Addr, u32) {
        let (_, network) = self.network();
        let (_, mask) = self.mask();
        let broadcast = network | !mask;
        (Ipv4Addr::from(broadcast), broadcast)
    }

    pub fn contains(&self, ip: Ipv4Addr) -> bool {
        let (_, net) = self.network();
        let (_, mask) = self.mask();
        (u32::from(ip) & mask) == net
    }

    /// Returns number of possible host addresses in this `Ipv4Network`.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::net::Ipv4Addr;
    /// use ipnetwork::Ipv4Network;
    ///
    /// let net = Ipv4Network::from_cidr("10.1.0.0/16").unwrap();
    /// assert_eq!(net.size(), 65536);
    ///
    /// let tinynet = Ipv4Network::from_cidr("0.0.0.0/32").unwrap();
    /// assert_eq!(tinynet.size(), 1);
    /// ```
    pub fn size(&self) -> u64 {
        let host_bits = (IPV4_BITS - self.prefix) as u32;
        (2 as u64).pow(host_bits)
    }

    /// Returns the `n`:th address within this network.
    /// The adresses are indexed from 0 and `n` must be smaller than the size of the network.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::net::Ipv4Addr;
    /// use ipnetwork::Ipv4Network;
    ///
    /// let net = Ipv4Network::from_cidr("192.168.0.0/24").unwrap();
    /// assert_eq!(net.nth(0).unwrap(), Ipv4Addr::new(192, 168, 0, 0));
    /// assert_eq!(net.nth(15).unwrap(), Ipv4Addr::new(192, 168, 0, 15));
    /// assert!(net.nth(256).is_none());
    ///
    /// let net2 = Ipv4Network::from_cidr("10.0.0.0/16").unwrap();
    /// assert_eq!(net2.nth(256).unwrap(), Ipv4Addr::new(10, 0, 1, 0));
    /// ```
    pub fn nth(&self, n: u32) -> Option<Ipv4Addr> {
        if (n as u64) < self.size() {
            let (_, net) = self.network();
            Some(Ipv4Addr::from(net + n))
        } else {
            None
        }
    }

    fn parse_addr(addr: &str) -> Result<Ipv4Addr, IpNetworkError> {
        let addr_parts = addr.split('.').map(|b| b.parse::<u8>());
        let mut bytes = [0; 4];
        for (i, byte) in addr_parts.enumerate() {
            if i >= 4 {
                return Err(IpNetworkError::InvalidAddr(format!("More than 4 bytes: {}", addr)));
            }
            bytes[i] = try!(byte.map_err(|_| {
                IpNetworkError::InvalidAddr(format!("All bytes not 0-255: {}", addr))
            }));
        }
        Ok(Ipv4Addr::new(bytes[0], bytes[1], bytes[2], bytes[3]))
    }
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
        let addr = try!(Ipv6Addr::from_str(addr_str).map_err(|_| {
            IpNetworkError::InvalidAddr(format!("{}", addr_str))
        }));
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

impl IpNetwork {
    pub fn new(ip: IpAddr, prefix: u8) -> Result<IpNetwork, IpNetworkError> {
        match ip {
            IpAddr::V4(a) => Ok(IpNetwork::V4(try!(Ipv4Network::new(a, prefix)))),
            IpAddr::V6(a) => Ok(IpNetwork::V6(try!(Ipv6Network::new(a, prefix)))),
        }
    }

    pub fn ip(&self) -> IpAddr {
        match *self {
            IpNetwork::V4(ref a) => IpAddr::V4(a.ip()),
            IpNetwork::V6(ref a) => IpAddr::V6(a.ip()),
        }
    }

    pub fn prefix(&self) -> u8 {
        match *self {
            IpNetwork::V4(ref a) => a.prefix(),
            IpNetwork::V6(ref a) => a.prefix(),
        }
    }
}

impl fmt::Debug for Ipv4Network {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}/{}", self.ip(), self.prefix())
    }
}

impl fmt::Debug for Ipv6Network {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}/{}", self.ip(), self.prefix())
    }
}

fn cidr_parts<'a>(cidr: &'a str) -> Result<(&'a str, &'a str), IpNetworkError> {
    let parts = cidr.split('/').collect::<Vec<&str>>();
    if parts.len() == 2 {
        Ok((parts[0], parts[1]))
    } else {
        Err(IpNetworkError::InvalidCidrFormat(format!("CIDR must contain '/': {}", cidr)))
    }
}

fn parse_prefix(prefix: &str, max: u8) -> Result<u8, IpNetworkError> {
    let mask = try!(prefix.parse::<u8>().map_err(|_| IpNetworkError::InvalidPrefix));
    if mask > max {
        Err(IpNetworkError::InvalidPrefix)
    } else {
        Ok(mask)
    }
}


#[cfg(test)]
mod test {
    use std::net::{Ipv4Addr, Ipv6Addr};
    use super::*;

    #[test]
    fn create_v4() {
        let cidr = Ipv4Network::new(Ipv4Addr::new(77, 88, 21, 11), 24).unwrap();
        assert_eq!(cidr.prefix(), 24);
    }

    #[test]
    fn create_v4_invalid_prefix() {
        let net = Ipv4Network::new(Ipv4Addr::new(0, 0, 0, 0), 33);
        assert!(net.is_err());
    }

    #[test]
    fn parse_v4_0bit() {
        let cidr = Ipv4Network::from_cidr("0/0").unwrap();
        assert_eq!(cidr.ip(), Ipv4Addr::new(0, 0, 0, 0));
        assert_eq!(cidr.prefix(), 0);
    }

    #[test]
    fn parse_v4_24bit() {
        let cidr = Ipv4Network::from_cidr("127.1.0.0/24").unwrap();
        assert_eq!(cidr.ip(), Ipv4Addr::new(127, 1, 0, 0));
        assert_eq!(cidr.prefix(), 24);
    }

    #[test]
    fn parse_v4_32bit() {
        let cidr = Ipv4Network::from_cidr("127.0.0.0/32").unwrap();
        assert_eq!(cidr.ip(), Ipv4Addr::new(127, 0, 0, 0));
        assert_eq!(cidr.prefix(), 32);
    }

    #[test]
    fn parse_v4_fail_addr() {
        let cidr = Ipv4Network::from_cidr("10.a.b/8");
        assert!(cidr.is_err());
    }

    #[test]
    fn parse_v4_fail_addr2() {
        let cidr = Ipv4Network::from_cidr("10.1.1.1.0/8");
        assert!(cidr.is_err());
    }

    #[test]
    fn parse_v4_fail_addr3() {
        let cidr = Ipv4Network::from_cidr("256/8");
        assert!(cidr.is_err());
    }

    #[test]
    fn parse_v4_fail_prefix() {
        let cidr = Ipv4Network::from_cidr("0/39");
        assert!(cidr.is_err());
    }

    #[test]
    fn size_v4_24bit() {
        let net = Ipv4Network::from_cidr("0/24").unwrap();
        assert_eq!(net.size(), 256);
    }

    #[test]
    fn size_v4_1bit() {
        let net = Ipv4Network::from_cidr("0/31").unwrap();
        assert_eq!(net.size(), 2);
    }

    #[test]
    fn size_v4_max() {
        let net = Ipv4Network::from_cidr("0/0").unwrap();
        assert_eq!(net.size(), 4_294_967_296);
    }

    #[test]
    fn size_v4_min() {
        let net = Ipv4Network::from_cidr("0/32").unwrap();
        assert_eq!(net.size(), 1);
    }

    #[test]
    fn nth_v4() {
        let net = Ipv4Network::new(Ipv4Addr::new(127, 0, 0, 0), 24).unwrap();
        assert_eq!(net.nth(0).unwrap(), Ipv4Addr::new(127, 0, 0, 0));
        assert_eq!(net.nth(1).unwrap(), Ipv4Addr::new(127, 0, 0, 1));
        assert_eq!(net.nth(255).unwrap(), Ipv4Addr::new(127, 0, 0, 255));
        assert!(net.nth(256).is_none());
    }

    #[test]
    fn nth_v4_fail() {
        let net = Ipv4Network::new(Ipv4Addr::new(10, 0, 0, 0), 32).unwrap();
        assert!(net.nth(1).is_none());
    }

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
    fn mask_v4() {
        let cidr = Ipv4Network::new(Ipv4Addr::new(74, 125, 227, 0), 29).unwrap();
        let (ip, int) = cidr.mask();
        assert_eq!(ip, Ipv4Addr::new(255, 255, 255, 248));
        assert_eq!(int, 4294967288);
    }

    #[test]
    fn network_v4() {
        let cidr = Ipv4Network::new(Ipv4Addr::new(10, 10, 1, 97), 23).unwrap();
        let (ip, int) = cidr.network();
        assert_eq!(ip, Ipv4Addr::new(10, 10, 0, 0));
        assert_eq!(int, 168427520);
    }

    #[test]
    fn broadcast_v4() {
        let cidr = Ipv4Network::new(Ipv4Addr::new(10, 10, 1, 97), 23).unwrap();
        let (ip, int) = cidr.broadcast();
        assert_eq!(ip, Ipv4Addr::new(10, 10, 1, 255));
        assert_eq!(int, 168428031);
    }

    #[test]
    fn contains_v4() {
        let cidr = Ipv4Network::new(Ipv4Addr::new(74, 125, 227, 0), 25).unwrap();
        let ip = Ipv4Addr::new(74, 125, 227, 4);
        assert!(cidr.contains(ip));
    }

    #[test]
    fn not_contains_v4() {
        let cidr = Ipv4Network::new(Ipv4Addr::new(10, 0, 0, 50), 24).unwrap();
        let ip = Ipv4Addr::new(10, 1, 0, 1);
        assert!(!cidr.contains(ip));
    }
}
