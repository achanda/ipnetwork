#![crate_type = "lib"]
#![feature(ip_addr)]
#[allow(dead_code)]

use std::fmt;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

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

impl Ipv4Network {
    pub fn new(addr: Ipv4Addr, prefix: u8) -> Ipv4Network {
        Ipv4Network { addr: addr, prefix: prefix }
    }

    pub fn ip(&self) -> &Ipv4Addr {
        &(self.addr)
    }

    pub fn prefix(&self) -> u8 {
        self.prefix
    }

    pub fn mask_int(&self) -> u32 {
        let prefix = self.prefix;
        !(0xffffffff >> prefix)
    }

    pub fn mask(&self) -> (Ipv4Addr, u32) {
        let prefix = self.prefix;
        let mask = !(0xffffffff >> prefix);
        return (Ipv4Addr::from(mask), mask);
    }

    pub fn network(&self) -> (Ipv4Addr, u32) {
        return (self.addr, u32::from(self.addr));
    }

    pub fn contains(&self, ip: Ipv4Addr) -> bool {
        let (_, net) = self.network();
        return (u32::from(ip) & net) == net
    }
}

impl Ipv6Network {
    pub fn new(addr: Ipv6Addr, prefix: u8) -> Ipv6Network {
        Ipv6Network { addr: addr, prefix: prefix }
    }

    pub fn ip(&self) -> &Ipv6Addr {
        &(self.addr)
    }

    pub fn prefix(&self) -> u8 {
        self.prefix
    }
}

impl IpNetwork {
    pub fn new(ip: IpAddr, prefix: u8) -> IpNetwork {
        match ip {
            IpAddr::V4(a) => IpNetwork::V4(Ipv4Network::new(a, prefix)),
            IpAddr::V6(a) => IpNetwork::V6(Ipv6Network::new(a, prefix)),
        }
    }

    pub fn ip(&self) -> IpAddr {
        match *self {
            IpNetwork::V4(ref a) => IpAddr::V4(*a.ip()),
            IpNetwork::V6(ref a) => IpAddr::V6(*a.ip()),
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

#[cfg(test)]
mod test {
    use std::net::{Ipv4Addr, Ipv6Addr};
    use super::*;

    #[test]
    fn create_v4() {
        let cidr = Ipv4Network::new(Ipv4Addr::new(77, 88, 21, 11), 24);
        assert_eq!(cidr.prefix(), 24);
    }

    #[test]
    fn create_v6() {
        let cidr = Ipv6Network::new(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1), 24);
        assert_eq!(cidr.prefix(), 24);
    }

    #[test]
    fn mask_v4() {
        let cidr = Ipv4Network::new(Ipv4Addr::new(74, 125, 227, 0), 29);
        let (ip, int) = cidr.mask();
        assert_eq!(ip, Ipv4Addr::new(255, 255, 255, 248));
        assert_eq!(int, 4294967288);
    }

    #[test]
    fn network_v4() {
        let cidr = Ipv4Network::new(Ipv4Addr::new(74, 125, 227, 0), 25);
        let (ip, int) = cidr.network();
        assert_eq!(ip, Ipv4Addr::new(74, 125, 227, 0));
        assert_eq!(int, 1249764096);
    }

    #[test]
    fn contains_v4() {
        let cidr = Ipv4Network::new(Ipv4Addr::new(74, 125, 227, 0), 25);
        let ip = Ipv4Addr::new(74,125,227,4);
        assert!(cidr.contains(ip));
    }
}
