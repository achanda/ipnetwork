use crate::Ipv4Network;

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
