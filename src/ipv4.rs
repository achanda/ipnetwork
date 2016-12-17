use std::fmt;
use std::net::Ipv4Addr;
use std::str::FromStr;

use common::{IpNetworkError, cidr_parts, parse_prefix, parse_addr};

const IPV4_BITS: u8 = 32;

#[derive(Debug,Clone,Copy,Hash,PartialEq,Eq)]
pub struct Ipv4Network {
    addr: Ipv4Addr,
    prefix: u8,
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

    /// Returns an iterator over `Ipv4Network`. Each call to `next` will return the next
    /// `Ipv4Addr` in the given network. `None` will be returned when there are no more
    /// addresses.
    pub fn iter(&self) -> Ipv4NetworkIterator {
        let start = u32::from(self.network()) as u64;
        let end = start + self.size();
        Ipv4NetworkIterator {
            next: start,
            end: end,
        }
    }

    pub fn ip(&self) -> Ipv4Addr {
        self.addr
    }

    pub fn prefix(&self) -> u8 {
        self.prefix
    }

    /// Returns the mask for this `Ipv4Network`.
    /// That means the `prefix` most significant bits will be 1 and the rest 0
    ///
    /// # Examples
    ///
    /// ```
    /// use std::net::Ipv4Addr;
    /// use ipnetwork::Ipv4Network;
    ///
    /// let net: Ipv4Network = "127.0.0.0/16".parse().unwrap();
    /// assert_eq!(net.mask(), Ipv4Addr::new(255, 255, 0, 0));
    /// ```
    pub fn mask(&self) -> Ipv4Addr {
        let prefix = self.prefix;
        let mask = !(0xffffffff as u64 >> prefix) as u32;
        Ipv4Addr::from(mask)
    }

    /// Returns the address of the network denoted by this `Ipv4Network`.
    /// This means the lowest possible IPv4 address inside of the network.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::net::Ipv4Addr;
    /// use ipnetwork::Ipv4Network;
    ///
    /// let net: Ipv4Network = "10.1.9.32/16".parse().unwrap();
    /// assert_eq!(net.network(), Ipv4Addr::new(10, 1, 0, 0));
    /// ```
    pub fn network(&self) -> Ipv4Addr {
        let mask = u32::from(self.mask());
        let ip = u32::from(self.addr) & mask;
        Ipv4Addr::from(ip)
    }

    /// Returns the broadcasting address of this `Ipv4Network`.
    /// This means the highest possible IPv4 address inside of the network.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::net::Ipv4Addr;
    /// use ipnetwork::Ipv4Network;
    ///
    /// let net: Ipv4Network = "10.9.0.32/16".parse().unwrap();
    /// assert_eq!(net.broadcast(), Ipv4Addr::new(10, 9, 255, 255));
    /// ```
    pub fn broadcast(&self) -> Ipv4Addr {
        let mask = u32::from(self.mask());
        let broadcast = u32::from(self.addr) | !mask;
        Ipv4Addr::from(broadcast)
    }

    /// Checks if a given `Ipv4Addr` is in this `Ipv4Network`
    ///
    /// # Examples
    ///
    /// ```
    /// use std::net::Ipv4Addr;
    /// use ipnetwork::Ipv4Network;
    ///
    /// let net: Ipv4Network = "127.0.0.0/24".parse().unwrap();
    /// assert!(net.contains(Ipv4Addr::new(127, 0, 0, 70)));
    /// assert!(!net.contains(Ipv4Addr::new(127, 0, 1, 70)));
    /// ```
    pub fn contains(&self, ip: Ipv4Addr) -> bool {
        let net = u32::from(self.network());
        let mask = u32::from(self.mask());
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
    /// let net: Ipv4Network = "10.1.0.0/16".parse().unwrap();
    /// assert_eq!(net.size(), 65536);
    ///
    /// let tinynet: Ipv4Network = "0.0.0.0/32".parse().unwrap();
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
    /// let net: Ipv4Network = "192.168.0.0/24".parse().unwrap();
    /// assert_eq!(net.nth(0).unwrap(), Ipv4Addr::new(192, 168, 0, 0));
    /// assert_eq!(net.nth(15).unwrap(), Ipv4Addr::new(192, 168, 0, 15));
    /// assert!(net.nth(256).is_none());
    ///
    /// let net2: Ipv4Network = "10.0.0.0/16".parse().unwrap();
    /// assert_eq!(net2.nth(256).unwrap(), Ipv4Addr::new(10, 0, 1, 0));
    /// ```
    pub fn nth(&self, n: u32) -> Option<Ipv4Addr> {
        if (n as u64) < self.size() {
            let net = u32::from(self.network());
            Some(Ipv4Addr::from(net + n))
        } else {
            None
        }
    }
}

impl fmt::Display for Ipv4Network {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}/{}", self.ip(), self.prefix())
    }
}


/// Creates an `Ipv4Network` from parsing a string in CIDR notation.
///
/// # Examples
///
/// ```
/// use std::net::Ipv4Addr;
/// use ipnetwork::Ipv4Network;
///
/// let new = Ipv4Network::new(Ipv4Addr::new(10, 1, 9, 32), 16).unwrap();
/// let from_cidr: Ipv4Network = "10.1.9.32/16".parse().unwrap();
/// assert_eq!(new.ip(), from_cidr.ip());
/// assert_eq!(new.prefix(), from_cidr.prefix());
/// ```
impl FromStr for Ipv4Network {
    type Err = IpNetworkError;
    fn from_str(s: &str) -> Result<Ipv4Network, IpNetworkError> {
        let (addr_str, prefix_str) = cidr_parts(s)?;
        let addr = parse_addr(addr_str)?;
        let prefix = parse_prefix(prefix_str, IPV4_BITS)?;
        Ipv4Network::new(addr, prefix)
    }
}

pub struct Ipv4NetworkIterator {
    next: u64,
    end: u64,
}

impl Iterator for Ipv4NetworkIterator {
    type Item = Ipv4Addr;

    fn next(&mut self) -> Option<Ipv4Addr> {
        if self.next < self.end {
            let next = Ipv4Addr::from(self.next as u32);
            self.next += 1;
            Some(next)
        } else {
            None
        }
    }
}

/// Converts a `Ipv4Addr` network mask into a prefix.
/// If the mask is invalid this will return an `IpNetworkError::InvalidPrefix`.
pub fn ipv4_mask_to_prefix(mask: Ipv4Addr) -> Result<u8, IpNetworkError> {
    let mask = u32::from(mask);

    let prefix = (!mask).leading_zeros() as u8;
    if ((mask as u64) << prefix) & 0xffffffff != 0 {
        Err(IpNetworkError::InvalidPrefix)
    } else {
        Ok(prefix)
    }
}

#[cfg(test)]
mod test {
    use std::mem;
    use std::collections::HashMap;
    use std::net::Ipv4Addr;
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
        let cidr: Ipv4Network = "0/0".parse().unwrap();
        assert_eq!(cidr.ip(), Ipv4Addr::new(0, 0, 0, 0));
        assert_eq!(cidr.prefix(), 0);
    }

    #[test]
    fn parse_v4_24bit() {
        let cidr: Ipv4Network = "127.1.0.0/24".parse().unwrap();
        assert_eq!(cidr.ip(), Ipv4Addr::new(127, 1, 0, 0));
        assert_eq!(cidr.prefix(), 24);
    }

    #[test]
    fn parse_v4_32bit() {
        let cidr: Ipv4Network = "127.0.0.0/32".parse().unwrap();
        assert_eq!(cidr.ip(), Ipv4Addr::new(127, 0, 0, 0));
        assert_eq!(cidr.prefix(), 32);
    }

    #[test]
    fn parse_v4_fail_addr() {
        let cidr: Option<Ipv4Network> = "10.a.b/8".parse().ok();
        assert_eq!(None, cidr);
    }

    #[test]
    fn parse_v4_fail_addr2() {
        let cidr: Option<Ipv4Network> = "10.1.1.1.0/8".parse().ok();
        assert_eq!(None, cidr);
    }

    #[test]
    fn parse_v4_fail_addr3() {
        let cidr: Option<Ipv4Network> = "256/8".parse().ok();
        assert_eq!(None, cidr);
    }

    #[test]
    fn parse_v4_non_zero_host_bits() {
        let cidr: Ipv4Network = "10.1.1.1/24".parse().unwrap();
        assert_eq!(cidr.ip(), Ipv4Addr::new(10, 1, 1, 1));
        assert_eq!(cidr.prefix(), 24);
    }

    #[test]
    fn parse_v4_fail_prefix() {
        let cidr: Option<Ipv4Network> = "0/39".parse().ok();
        assert_eq!(None, cidr);
    }

    #[test]
    fn size_v4_24bit() {
        let net: Ipv4Network = "0/24".parse().unwrap();
        assert_eq!(net.size(), 256);
    }

    #[test]
    fn size_v4_1bit() {
        let net: Ipv4Network = "0/31".parse().unwrap();
        assert_eq!(net.size(), 2);
    }

    #[test]
    fn size_v4_max() {
        let net: Ipv4Network = "0/0".parse().unwrap();
        assert_eq!(net.size(), 4_294_967_296);
    }

    #[test]
    fn size_v4_min() {
        let net: Ipv4Network = "0/32".parse().unwrap();
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
    fn hash_eq_compatibility_v4() {
        let mut map = HashMap::new();
        let net = Ipv4Network::new(Ipv4Addr::new(127, 0, 0, 1), 16).unwrap();
        map.insert(net, 137);
        let out = map.get(&net).unwrap();
        assert_eq!(137, *out);
    }

    #[test]
    fn copy_compatibility_v4() {
        let net = Ipv4Network::new(Ipv4Addr::new(127, 0, 0, 1), 16).unwrap();
        mem::drop(net);
        assert_eq!(16, net.prefix());
    }

    #[test]
    fn mask_v4() {
        let cidr = Ipv4Network::new(Ipv4Addr::new(74, 125, 227, 0), 29).unwrap();
        let mask = cidr.mask();
        assert_eq!(mask, Ipv4Addr::new(255, 255, 255, 248));
    }

    #[test]
    fn network_v4() {
        let cidr = Ipv4Network::new(Ipv4Addr::new(10, 10, 1, 97), 23).unwrap();
        let net = cidr.network();
        assert_eq!(net, Ipv4Addr::new(10, 10, 0, 0));
    }

    #[test]
    fn broadcast_v4() {
        let cidr = Ipv4Network::new(Ipv4Addr::new(10, 10, 1, 97), 23).unwrap();
        let bcast = cidr.broadcast();
        assert_eq!(bcast, Ipv4Addr::new(10, 10, 1, 255));
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

    #[test]
    fn iterator_v4() {
        let cidr: Ipv4Network = "192.168.122.0/30".parse().unwrap();
        let mut iter = cidr.iter();
        assert_eq!(Ipv4Addr::new(192, 168, 122, 0), iter.next().unwrap());
        assert_eq!(Ipv4Addr::new(192, 168, 122, 1), iter.next().unwrap());
        assert_eq!(Ipv4Addr::new(192, 168, 122, 2), iter.next().unwrap());
        assert_eq!(Ipv4Addr::new(192, 168, 122, 3), iter.next().unwrap());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn iterator_v4_tiny() {
        let cidr: Ipv4Network = "10/32".parse().unwrap();
        let mut iter = cidr.iter();
        assert_eq!(Ipv4Addr::new(10, 0, 0, 0), iter.next().unwrap());
        assert_eq!(None, iter.next());
    }

    // Tests the entire IPv4 space to see if the iterator will stop at the correct place
    // and not overflow or wrap around. Ignored since it takes a long time to run.
    #[test]
    #[ignore]
    fn iterator_v4_huge() {
        let cidr: Ipv4Network = "0/0".parse().unwrap();
        let mut iter = cidr.iter();
        for i in 0..(u32::max_value() as u64 + 1) {
            assert_eq!(i as u32, u32::from(iter.next().unwrap()));
        }
        assert_eq!(None, iter.next());
    }

    #[test]
    fn v4_mask_to_prefix() {
        let mask = Ipv4Addr::new(255, 255, 255, 128);
        let prefix = ipv4_mask_to_prefix(mask).unwrap();
        assert_eq!(prefix, 25);
    }

    #[test]
    fn invalid_v4_mask_to_prefix() {
        let mask = Ipv4Addr::new(255, 0, 255, 0);
        let prefix = ipv4_mask_to_prefix(mask);
        assert!(prefix.is_err());
    }
}
