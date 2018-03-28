use std::cmp;
use std::fmt;
use std::net::Ipv6Addr;
use std::str::FromStr;

use common::{cidr_parts, parse_prefix, IpNetworkError};

const IPV6_BITS: u8 = 128;
const IPV6_SEGMENT_BITS: u8 = 16;

/// Represents a network range where the IP addresses are of v6
#[cfg_attr(feature = "with-serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
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

    /// Returns an iterator over `Ipv6Network`. Each call to `next` will return the next
    /// `Ipv6Addr` in the given network. `None` will be returned when there are no more
    /// addresses.
    #[cfg(feature = "ipv6-iterator")]
    pub fn iter(&self) -> Ipv6NetworkIterator {
        let dec = u128::from(self.addr);
        let max = u128::max_value();
        let prefix = self.prefix;

        let mask = max.checked_shl((IPV6_BITS - prefix) as u32).unwrap_or(0);
        let start: u128 = dec & mask;

        let mask = max.checked_shr(prefix as u32).unwrap_or(0);
        let end: u128 = dec | mask;

        Ipv6NetworkIterator {
            next: start,
            end: end,
        }
    }

    /// Returns the address of the network denoted by this `Ipv6Network`.
    /// This means the lowest possible IPv6 address inside of the network.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::net::Ipv6Addr;
    /// use ipnetwork::Ipv6Network;
    ///
    /// let net: Ipv6Network = "2001:db8::/96".parse().unwrap();
    /// assert_eq!(net.network(), Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 0));
    /// ```
    #[cfg(feature = "ipv6-methods")]
    pub fn network(&self) -> Ipv6Addr {
        let mask = u128::from(self.mask());
        let ip = u128::from(self.addr) & mask;
        Ipv6Addr::from(ip)
    }

    /// Returns the broadcast address of this `Ipv6Network`.
    /// This means the highest possible IPv4 address inside of the network.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::net::Ipv6Addr;
    /// use ipnetwork::Ipv6Network;
    ///
    /// let net: Ipv6Network = "2001:db8::/96".parse().unwrap();
    /// assert_eq!(net.broadcast(), Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0xffff, 0xffff));
    /// ```
    #[cfg(feature = "ipv6-methods")]
    pub fn broadcast(&self) -> Ipv6Addr {
        let mask = u128::from(self.mask());
        let broadcast = u128::from(self.addr) | !mask;
        Ipv6Addr::from(broadcast)
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
    /// let net: Ipv6Network = "ff01::0".parse().unwrap();
    /// assert_eq!(net.mask(), Ipv6Addr::new(0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff));
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
        self.mask()
            .segments()
            .iter()
            .zip(addrs)
            .all(|(mask, (a, b))| a & mask == b & mask)
    }

    /// Returns number of possible host addresses in this `Ipv6Network`.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::net::Ipv6Addr;
    /// use ipnetwork::Ipv6Network;
    ///
    /// let net: Ipv6Network = "ff01::0/32".parse().unwrap();
    /// assert_eq!(net.size(), 79228162514264337593543950336);
    ///
    /// let tinynet: Ipv6Network = "ff01::0/128".parse().unwrap();
    /// assert_eq!(tinynet.size(), 1);
    /// ```
    #[cfg(feature = "ipv6-methods")]
    pub fn size(&self) -> u128 {
        let host_bits = (IPV6_BITS - self.prefix) as u32;
        (2 as u128).pow(host_bits)
    }
}

impl FromStr for Ipv6Network {
    type Err = IpNetworkError;
    fn from_str(s: &str) -> Result<Ipv6Network, IpNetworkError> {
        let (addr_str, prefix_str) = cidr_parts(s)?;
        let addr = Ipv6Addr::from_str(addr_str)
            .map_err(|_| IpNetworkError::InvalidAddr(addr_str.to_string()))?;
        let prefix = if prefix_str.is_empty() {
            IPV6_BITS
        } else {
            parse_prefix(prefix_str, IPV6_BITS)?
        };
        Ipv6Network::new(addr, prefix)
    }
}

impl From<Ipv6Addr> for Ipv6Network {
    fn from(a: Ipv6Addr) -> Ipv6Network {
        Ipv6Network {
            addr: a,
            prefix: 128,
        }
    }
}

#[cfg(feature = "ipv6-iterator")]
pub struct Ipv6NetworkIterator {
    next: u128,
    end: u128,
}

#[cfg(feature = "ipv6-iterator")]
impl Iterator for Ipv6NetworkIterator {
    type Item = Ipv6Addr;

    fn next(&mut self) -> Option<Ipv6Addr> {
        if self.next <= self.end {
            let next = Ipv6Addr::from(self.next);
            self.next += 1;
            Some(next)
        } else {
            None
        }
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
    fn parse_v6_noprefix() {
        let cidr: Ipv6Network = "::1".parse().unwrap();
        assert_eq!(cidr.ip(), Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1));
        assert_eq!(cidr.prefix(), 128);
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
    fn parse_v6_fail_two_slashes() {
        let cidr: Option<Ipv6Network> = "::1/24/".parse().ok();
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

    #[test]
    #[cfg(feature = "ipv6-iterator")]
    fn iterator_v6() {
        let cidr: Ipv6Network = "2001:db8::/126".parse().unwrap();
        let mut iter = cidr.iter();
        assert_eq!(
            Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 0),
            iter.next().unwrap()
        );
        assert_eq!(
            Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 1),
            iter.next().unwrap()
        );
        assert_eq!(
            Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 2),
            iter.next().unwrap()
        );
        assert_eq!(
            Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 3),
            iter.next().unwrap()
        );
        assert_eq!(None, iter.next());
    }

    #[test]
    #[cfg(feature = "ipv6-iterator")]
    fn iterator_v6_tiny() {
        let cidr: Ipv6Network = "2001:db8::/128".parse().unwrap();
        let mut iter = cidr.iter();
        assert_eq!(
            Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 0),
            iter.next().unwrap()
        );
        assert_eq!(None, iter.next());
    }

    #[test]
    #[cfg(feature = "ipv6-iterator")]
    fn iterator_v6_huge() {
        let cidr: Ipv6Network = "2001:db8::/0".parse().unwrap();
        let mut iter = cidr.iter();
        assert_eq!(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0), iter.next().unwrap());
        assert_eq!(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1), iter.next().unwrap());
        assert_eq!(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 2), iter.next().unwrap());
    }

    #[test]
    #[cfg(feature = "ipv6-methods")]
    fn network_v6() {
        let cidr: Ipv6Network = "2001:db8::0/96".parse().unwrap();
        let net = cidr.network();
        let expected: Ipv6Addr = "2001:db8::".parse().unwrap();
        assert_eq!(net, expected);
    }

    #[test]
    #[cfg(feature = "ipv6-methods")]
    fn broadcast_v6() {
        let cidr: Ipv6Network = "2001:db8::0/96".parse().unwrap();
        let net = cidr.broadcast();
        let expected: Ipv6Addr = "2001:db8::ffff:ffff".parse().unwrap();
        assert_eq!(net, expected);
    }

    #[test]
    #[cfg(feature = "ipv6-methods")]
    fn size_v6() {
        let cidr: Ipv6Network = "2001:db8::0/96".parse().unwrap();
        assert_eq!(cidr.size(), 4294967296);
    }

    #[test]
    fn ipv6network_from_ipv6addr() {
        let net = Ipv6Network::from(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1));
        let expected = Ipv6Network::new(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1), 128).unwrap();
        assert_eq!(net, expected);
    }

    #[test]
    fn test_send() {
        fn assert_send<T: Send>() {}
        assert_send::<Ipv6Network>();
    }

    #[test]
    fn test_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<Ipv6Network>();
    }
}
