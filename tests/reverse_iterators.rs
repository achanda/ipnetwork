use std::collections::VecDeque;
use std::fmt::Debug;
use std::net::{Ipv4Addr, Ipv6Addr};

use ipnetwork::{IpNetwork, Ipv4Network, Ipv6Network};

fn check_both_ends<I>(iter: I)
where
    I: DoubleEndedIterator + Clone,
    I::Item: Eq + Debug,
{
    let expected: Vec<_> = iter.clone().collect();
    assert_eq!(
        iter.clone().rev().collect::<Vec<_>>(),
        expected.into_iter().rev().collect::<Vec<_>>()
    );

    // Exercise every front/back consumption pattern for this four-address network.
    for pattern in 0..16 {
        let mut remaining: VecDeque<_> = iter.clone().collect();
        let mut actual = iter.clone();
        for bit in 0..4 {
            assert_eq!(actual.size_hint(), (remaining.len(), Some(remaining.len())));
            let (got, want) = if pattern & (1 << bit) == 0 {
                (actual.next(), remaining.pop_front())
            } else {
                (actual.next_back(), remaining.pop_back())
            };
            assert_eq!(got, want);
        }
        assert_eq!(actual.next(), None);
        assert_eq!(actual.next_back(), None);
        assert_eq!(actual.next(), None);
    }
}

#[test]
fn reverse_and_interleave_ipv4() {
    let net: Ipv4Network = "192.0.2.1/30".parse().unwrap();
    check_both_ends(net.iter());
}

#[test]
fn reverse_and_interleave_ipv6() {
    let net: Ipv6Network = "2001:db8::1/126".parse().unwrap();
    check_both_ends(net.iter());
}

#[test]
fn reverse_and_interleave_ip_network() {
    for cidr in ["192.0.2.1/30", "2001:db8::1/126"] {
        let net: IpNetwork = cidr.parse().unwrap();
        check_both_ends(net.iter());
    }
}

#[test]
fn singleton_networks_do_not_underflow_or_repeat() {
    for addr in [Ipv4Addr::UNSPECIFIED, Ipv4Addr::BROADCAST] {
        let mut iter = Ipv4Network::from(addr).iter();
        assert_eq!(iter.next_back(), Some(addr));
        assert_eq!(iter.next_back(), None);
        assert_eq!(iter.next(), None);
    }
    for addr in [Ipv6Addr::UNSPECIFIED, Ipv6Addr::from(u128::MAX)] {
        let mut iter = Ipv6Network::from(addr).iter();
        assert_eq!(iter.next_back(), Some(addr));
        assert_eq!(iter.next_back(), None);
        assert_eq!(iter.next(), None);
    }
}

#[test]
fn reverse_from_highest_address() {
    let net: Ipv4Network = "255.255.255.255/31".parse().unwrap();
    let mut iter = net.iter();
    assert_eq!(iter.next_back(), Some(Ipv4Addr::BROADCAST));
    assert_eq!(iter.next(), Some(Ipv4Addr::from(u32::MAX - 1)));
    assert_eq!(iter.next_back(), None);

    let net: Ipv6Network = "::/0".parse().unwrap();
    let mut iter = net.iter();
    assert_eq!(iter.next_back(), Some(Ipv6Addr::from(u128::MAX)));
    assert_eq!(iter.next(), Some(Ipv6Addr::UNSPECIFIED));
    assert_eq!(iter.next_back(), Some(Ipv6Addr::from(u128::MAX - 1)));
    assert_eq!(iter.next(), Some(Ipv6Addr::LOCALHOST));
}
