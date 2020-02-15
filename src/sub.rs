use crate::{Ipv4Network, Ipv6Network};
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
