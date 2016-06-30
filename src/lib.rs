#![cfg_attr(feature = "dev", allow(unstable_features))]
#![cfg_attr(feature = "dev", feature(plugin))]
#![cfg_attr(feature = "dev", plugin(clippy))]
#![crate_type = "lib"]
#[allow(dead_code)]

use std::net::IpAddr;

mod ipv4;
mod ipv6;
mod common;

pub use ipv4::Ipv4Network;
pub use ipv6::Ipv6Network;
pub use common::IpNetworkError;

// A network
#[derive(Debug,Clone,Copy,Hash,PartialEq,Eq)]
pub enum IpNetwork {
    V4(Ipv4Network),
    V6(Ipv6Network),
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
