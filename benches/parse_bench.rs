#[macro_use]
extern crate criterion;
extern crate ipnetwork;

use ipnetwork::{Ipv4Network, Ipv6Network};
use criterion::Criterion;

use std::net::{Ipv4Addr, Ipv6Addr};

fn parse_ipv4_benchmark(c: &mut Criterion) {
    c.bench_function("parse ipv4", |b| b.iter(|| {
        "127.1.0.0/24".parse::<Ipv4Network>().unwrap()
    }));
}

fn parse_ipv6_benchmark(c: &mut Criterion) {
    c.bench_function("parse ipv6", |b| b.iter(|| {
        "FF01:0:0:17:0:0:0:2/64".parse::<Ipv6Network>().unwrap()
    }));
}

fn contains_ipv4_benchmark(c: &mut Criterion) {
    c.bench_function("contains ipv4", |b| b.iter(|| {
        let cidr = "74.125.227.0/25".parse::<Ipv4Network>().unwrap();
        cidr.contains(Ipv4Addr::new(74, 125, 227, 4))
    }));
}

fn contains_ipv6_benchmark(c: &mut Criterion) {
    c.bench_function("contains ipv6", |b| b.iter(|| {
        let cidr = "FF01:0:0:17:0:0:0:2/65".parse::<Ipv6Network>().unwrap();
        cidr.contains(Ipv6Addr::new(0xff01, 0, 0, 0x17, 0x7fff, 0, 0, 0x2))
    }));
}

criterion_group!(benches, parse_ipv4_benchmark, parse_ipv6_benchmark, contains_ipv4_benchmark, contains_ipv6_benchmark);
criterion_main!(benches);
