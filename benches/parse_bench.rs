#[macro_use]
extern crate criterion;
extern crate ipnetwork;

use ipnetwork::{Ipv4Network, Ipv6Network};
use criterion::Criterion;

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

criterion_group!(benches, parse_ipv4_benchmark, parse_ipv6_benchmark);
criterion_main!(benches);
