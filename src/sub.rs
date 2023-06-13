use crate::{IpNetwork, Ipv4Network, Ipv6Network};
use std::{iter, ops::Sub};

impl Sub for Ipv4Network {
    type Output = Ipv4NetworkSubResult;

    fn sub(self, other: Self) -> Self::Output {
        let subtrahend: u32 = self.network().into();
        let minuend: u32 = other.network().into();
        let mask: u32 = self.mask().into();

        if minuend & mask == subtrahend {
            let max_bit_position = 32 - self.prefix();
            let bit_position = 32 - other.prefix();

            Ipv4NetworkSubResult::MultipleNetworks(Ipv4NetworkSubSet {
                network: minuend,
                bit_position,
                max_bit_position,
            })
        } else {
            let other_mask: u32 = other.mask().into();

            if subtrahend & other_mask == minuend {
                Ipv4NetworkSubResult::Empty
            } else {
                Ipv4NetworkSubResult::SingleNetwork(self)
            }
        }
    }
}

impl Sub for Ipv6Network {
    type Output = Ipv6NetworkSubResult;

    fn sub(self, other: Self) -> Self::Output {
        let subtrahend: u128 = self.network().into();
        let minuend: u128 = other.network().into();
        let mask: u128 = self.mask().into();

        if minuend & mask == subtrahend {
            let max_bit_position = 32 - self.prefix();
            let bit_position = 32 - other.prefix();

            Ipv6NetworkSubResult::MultipleNetworks(Ipv6NetworkSubSet {
                network: minuend,
                bit_position,
                max_bit_position,
            })
        } else {
            let other_mask: u128 = other.mask().into();

            if subtrahend & other_mask == minuend {
                Ipv6NetworkSubResult::Empty
            } else {
                Ipv6NetworkSubResult::SingleNetwork(self)
            }
        }
    }
}

impl Sub for IpNetwork {
    type Output = IpNetworkSubResult;

    fn sub(self, other: Self) -> Self::Output {
        match (self, other) {
            (IpNetwork::V4(subtrahend), IpNetwork::V4(minuend)) => {
                IpNetworkSubResult::V4(subtrahend - minuend)
            }
            (IpNetwork::V6(subtrahend), IpNetwork::V6(minuend)) => {
                IpNetworkSubResult::V6(subtrahend - minuend)
            }
            (IpNetwork::V4(_), IpNetwork::V6(_)) => {
                panic!("Can't subtract IPv6 network from IPv4 network")
            }
            (IpNetwork::V6(_), IpNetwork::V4(_)) => {
                panic!("Can't subtract IPv4 network from IPv6 network")
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Ipv4NetworkSubResult {
    Empty,
    SingleNetwork(Ipv4Network),
    MultipleNetworks(Ipv4NetworkSubSet),
}

impl Iterator for Ipv4NetworkSubResult {
    type Item = Ipv4Network;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Ipv4NetworkSubResult::Empty => None,
            &mut Ipv4NetworkSubResult::SingleNetwork(network) => {
                *self = Ipv4NetworkSubResult::Empty;
                Some(network)
            }
            Ipv4NetworkSubResult::MultipleNetworks(range) => {
                if let Some(item) = range.next() {
                    Some(item)
                } else {
                    *self = Ipv4NetworkSubResult::Empty;
                    None
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Ipv6NetworkSubResult {
    Empty,
    SingleNetwork(Ipv6Network),
    MultipleNetworks(Ipv6NetworkSubSet),
}

impl Iterator for Ipv6NetworkSubResult {
    type Item = Ipv6Network;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Ipv6NetworkSubResult::Empty => None,
            &mut Ipv6NetworkSubResult::SingleNetwork(network) => {
                *self = Ipv6NetworkSubResult::Empty;
                Some(network)
            }
            Ipv6NetworkSubResult::MultipleNetworks(range) => {
                if let Some(item) = range.next() {
                    Some(item)
                } else {
                    *self = Ipv6NetworkSubResult::Empty;
                    None
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum IpNetworkSubResult {
    V4(Ipv4NetworkSubResult),
    V6(Ipv6NetworkSubResult),
}

impl Iterator for IpNetworkSubResult {
    type Item = IpNetwork;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            IpNetworkSubResult::V4(result) => result.next().map(IpNetwork::from),
            IpNetworkSubResult::V6(result) => result.next().map(IpNetwork::from),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Ipv4NetworkSubSet {
    network: u32,
    bit_position: u8,
    max_bit_position: u8,
}

impl Iterator for Ipv4NetworkSubSet {
    type Item = Ipv4Network;

    fn next(&mut self) -> Option<Self::Item> {
        if self.bit_position < self.max_bit_position {
            let bit_mask = 1 << self.bit_position;
            let prefix_mask = !(bit_mask - 1);
            let address = (self.network ^ bit_mask) & prefix_mask;
            let prefix = 32 - self.bit_position;

            self.bit_position += 1;

            Some(Ipv4Network::new(address.into(), prefix).expect("Invalid IPv4 network prefix"))
        } else {
            None
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Ipv6NetworkSubSet {
    network: u128,
    bit_position: u8,
    max_bit_position: u8,
}

impl Iterator for Ipv6NetworkSubSet {
    type Item = Ipv6Network;

    fn next(&mut self) -> Option<Self::Item> {
        if self.bit_position < self.max_bit_position {
            let bit_mask = 1 << self.bit_position;
            let prefix_mask = !(bit_mask - 1);
            let address = (self.network ^ bit_mask) & prefix_mask;
            let prefix = 128 - self.bit_position;

            self.bit_position += 1;

            Some(Ipv6Network::new(address.into(), prefix).expect("Invalid IPv6 network prefix"))
        } else {
            None
        }
    }
}

impl<T> Sub<T> for Ipv4Network
where
    T: IntoIterator<Item = Ipv4Network>,
{
    type Output = Box<dyn Iterator<Item = Ipv4Network>>;

    fn sub(self, minuends: T) -> Self::Output {
        let mut result: Box<dyn Iterator<Item = Self>> = Box::new(iter::once(self));

        for minuend in minuends {
            result = Box::new(result.flat_map(move |partial_result| partial_result - minuend));
        }

        result
    }
}

impl<T> Sub<T> for Ipv6Network
where
    T: IntoIterator<Item = Ipv6Network>,
{
    type Output = Box<dyn Iterator<Item = Ipv6Network>>;

    fn sub(self, minuends: T) -> Self::Output {
        let mut result: Box<dyn Iterator<Item = Self>> = Box::new(iter::once(self));

        for minuend in minuends {
            result = Box::new(result.flat_map(move |partial_result| partial_result - minuend));
        }

        result
    }
}

impl<T> Sub<T> for IpNetwork
where
    T: IntoIterator<Item = IpNetwork>,
{
    type Output = Box<dyn Iterator<Item = IpNetwork>>;

    fn sub(self, minuends: T) -> Self::Output {
        let mut result: Box<dyn Iterator<Item = Self>> = Box::new(iter::once(self));

        for minuend in minuends {
            result = Box::new(result.flat_map(move |partial_result| partial_result - minuend));
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::{
        collections::HashSet,
        net::{IpAddr, Ipv4Addr},
    };

    #[test]
    fn subtract_out_of_range() {
        let minuend = IpNetwork::new(IpAddr::V4(Ipv4Addr::new(25, 0, 0, 0)), 8).unwrap();
        let subtrahend = IpNetwork::new(IpAddr::V4(Ipv4Addr::new(125, 92, 4, 0)), 24).unwrap();

        let difference: Vec<_> = minuend.sub(subtrahend).collect();

        let expected = vec![minuend];

        assert_eq!(difference, expected);
    }

    #[test]
    fn subtract_whole_range() {
        let minuend = IpNetwork::new(IpAddr::V4(Ipv4Addr::new(25, 0, 0, 0)), 8).unwrap();
        let subtrahend = IpNetwork::new(IpAddr::V4(Ipv4Addr::new(16, 0, 0, 0)), 4).unwrap();

        let difference: Vec<_> = minuend.sub(subtrahend).collect();

        assert!(difference.is_empty());
    }

    #[test]
    fn subtract_inner_range() {
        let minuend = IpNetwork::new(IpAddr::V4(Ipv4Addr::new(10, 0, 0, 0)), 8).unwrap();
        let subtrahend = IpNetwork::new(IpAddr::V4(Ipv4Addr::new(10, 10, 10, 0)), 24).unwrap();

        let difference: HashSet<_> = minuend.sub(subtrahend).collect();

        let expected = vec![
            ([10, 0, 0, 0], 13),
            ([10, 8, 0, 0], 15),
            ([10, 10, 0, 0], 21),
            ([10, 10, 8, 0], 23),
            ([10, 10, 11, 0], 24),
            ([10, 10, 12, 0], 22),
            ([10, 10, 16, 0], 20),
            ([10, 10, 32, 0], 19),
            ([10, 10, 64, 0], 18),
            ([10, 10, 128, 0], 17),
            ([10, 11, 0, 0], 16),
            ([10, 12, 0, 0], 14),
            ([10, 16, 0, 0], 12),
            ([10, 32, 0, 0], 11),
            ([10, 64, 0, 0], 10),
            ([10, 128, 0, 0], 9),
        ];

        let expected: HashSet<_> = expected
            .into_iter()
            .map(|(octets, prefix)| IpNetwork::new(IpAddr::V4(octets.into()), prefix).unwrap())
            .collect();

        assert_eq!(difference, expected);
    }

    #[test]
    fn subtract_single_address() {
        let minuend = IpNetwork::new(IpAddr::V4(Ipv4Addr::new(10, 64, 0, 0)), 10).unwrap();
        let subtrahend = IpNetwork::new(IpAddr::V4(Ipv4Addr::new(10, 64, 0, 0)), 32).unwrap();

        let difference: HashSet<_> = minuend.sub(subtrahend).collect();

        let expected = vec![
            ([10, 64, 0, 1], 32),
            ([10, 64, 0, 2], 31),
            ([10, 64, 0, 4], 30),
            ([10, 64, 0, 8], 29),
            ([10, 64, 0, 16], 28),
            ([10, 64, 0, 32], 27),
            ([10, 64, 0, 64], 26),
            ([10, 64, 0, 128], 25),
            ([10, 64, 1, 0], 24),
            ([10, 64, 2, 0], 23),
            ([10, 64, 4, 0], 22),
            ([10, 64, 8, 0], 21),
            ([10, 64, 16, 0], 20),
            ([10, 64, 32, 0], 19),
            ([10, 64, 64, 0], 18),
            ([10, 64, 128, 0], 17),
            ([10, 65, 0, 0], 16),
            ([10, 66, 0, 0], 15),
            ([10, 68, 0, 0], 14),
            ([10, 72, 0, 0], 13),
            ([10, 80, 0, 0], 12),
            ([10, 96, 0, 0], 11),
        ];

        let expected: HashSet<_> = expected
            .into_iter()
            .map(|(octets, prefix)| IpNetwork::new(IpAddr::V4(octets.into()), prefix).unwrap())
            .collect();

        assert_eq!(difference, expected);
    }

    #[test]
    fn subtract_multiple() {
        let minuend = IpNetwork::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 0).unwrap();
        let subtrahend_1 = IpNetwork::new(IpAddr::V4(Ipv4Addr::new(10, 0, 0, 0)), 8).unwrap();
        let subtrahend_2 = IpNetwork::new(IpAddr::V4(Ipv4Addr::new(172, 16, 0, 0)), 12).unwrap();
        let subtrahend_3 = IpNetwork::new(IpAddr::V4(Ipv4Addr::new(192, 168, 0, 0)), 16).unwrap();
        let subtrahend_4 = IpNetwork::new(IpAddr::V4(Ipv4Addr::new(169, 254, 0, 0)), 16).unwrap();
        let subtrahend_5 = IpNetwork::new(IpAddr::V4(Ipv4Addr::new(224, 0, 0, 0)), 24).unwrap();
        let subtrahend_6 =
            IpNetwork::new(IpAddr::V4(Ipv4Addr::new(239, 255, 255, 250)), 32).unwrap();
        let subtrahend_7 =
            IpNetwork::new(IpAddr::V4(Ipv4Addr::new(239, 255, 255, 251)), 32).unwrap();

        let difference: HashSet<_> = (minuend
            - vec![
                subtrahend_1,
                subtrahend_2,
                subtrahend_3,
                subtrahend_4,
                subtrahend_5,
                subtrahend_6,
                subtrahend_7,
            ])
        .collect();

        let expected = vec![
            ([0, 0, 0, 0], 5),
            ([8, 0, 0, 0], 7),
            ([11, 0, 0, 0], 8),
            ([12, 0, 0, 0], 6),
            ([16, 0, 0, 0], 4),
            ([32, 0, 0, 0], 3),
            ([64, 0, 0, 0], 2),
            ([128, 0, 0, 0], 3),
            ([160, 0, 0, 0], 5),
            ([168, 0, 0, 0], 8),
            ([169, 0, 0, 0], 9),
            ([169, 128, 0, 0], 10),
            ([169, 192, 0, 0], 11),
            ([169, 224, 0, 0], 12),
            ([169, 240, 0, 0], 13),
            ([169, 248, 0, 0], 14),
            ([169, 252, 0, 0], 15),
            ([169, 255, 0, 0], 16),
            ([170, 0, 0, 0], 7),
            ([172, 0, 0, 0], 12),
            ([172, 32, 0, 0], 11),
            ([172, 64, 0, 0], 10),
            ([172, 128, 0, 0], 9),
            ([173, 0, 0, 0], 8),
            ([174, 0, 0, 0], 7),
            ([176, 0, 0, 0], 4),
            ([192, 0, 0, 0], 9),
            ([192, 128, 0, 0], 11),
            ([192, 160, 0, 0], 13),
            ([192, 169, 0, 0], 16),
            ([192, 170, 0, 0], 15),
            ([192, 172, 0, 0], 14),
            ([192, 176, 0, 0], 12),
            ([192, 192, 0, 0], 10),
            ([193, 0, 0, 0], 8),
            ([194, 0, 0, 0], 7),
            ([196, 0, 0, 0], 6),
            ([200, 0, 0, 0], 5),
            ([208, 0, 0, 0], 4),
            ([224, 0, 1, 0], 24),
            ([224, 0, 2, 0], 23),
            ([224, 0, 4, 0], 22),
            ([224, 0, 8, 0], 21),
            ([224, 0, 16, 0], 20),
            ([224, 0, 32, 0], 19),
            ([224, 0, 64, 0], 18),
            ([224, 0, 128, 0], 17),
            ([224, 1, 0, 0], 16),
            ([224, 2, 0, 0], 15),
            ([224, 4, 0, 0], 14),
            ([224, 8, 0, 0], 13),
            ([224, 16, 0, 0], 12),
            ([224, 32, 0, 0], 11),
            ([224, 64, 0, 0], 10),
            ([224, 128, 0, 0], 9),
            ([225, 0, 0, 0], 8),
            ([226, 0, 0, 0], 7),
            ([228, 0, 0, 0], 6),
            ([232, 0, 0, 0], 6),
            ([236, 0, 0, 0], 7),
            ([238, 0, 0, 0], 8),
            ([239, 0, 0, 0], 9),
            ([239, 128, 0, 0], 10),
            ([239, 192, 0, 0], 11),
            ([239, 224, 0, 0], 12),
            ([239, 240, 0, 0], 13),
            ([239, 248, 0, 0], 14),
            ([239, 252, 0, 0], 15),
            ([239, 254, 0, 0], 16),
            ([239, 255, 0, 0], 17),
            ([239, 255, 128, 0], 18),
            ([239, 255, 192, 0], 19),
            ([239, 255, 224, 0], 20),
            ([239, 255, 240, 0], 21),
            ([239, 255, 248, 0], 22),
            ([239, 255, 252, 0], 23),
            ([239, 255, 254, 0], 24),
            ([239, 255, 255, 0], 25),
            ([239, 255, 255, 128], 26),
            ([239, 255, 255, 192], 27),
            ([239, 255, 255, 224], 28),
            ([239, 255, 255, 240], 29),
            ([239, 255, 255, 248], 31),
            ([239, 255, 255, 252], 30),
            ([240, 0, 0, 0], 4),
        ];

        let expected: HashSet<_> = expected
            .into_iter()
            .map(|(octets, prefix)| IpNetwork::new(IpAddr::V4(octets.into()), prefix).unwrap())
            .collect();

        assert_eq!(difference, expected);
    }
}
