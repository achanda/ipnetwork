This is a library to work with IPv4 and v6 CIDRs in rust
The IPv4 implementation is stable, IPv6 implementation is not done yet.

[![Build Status](https://travis-ci.org/achanda/ipnetwork.svg?branch=master)](https://travis-ci.org/achanda/ipnetwork)
[![Merit Badge](http://meritbadge.herokuapp.com/ipnetwork)](https://crates.io/crates/ipnetwork)

Run Clippy by doing
```
cargo test --features "dev"
```

Installation
=============
This crate works with Cargo. Assuming you have Rust and Cargo installed, simply check out the source and run tests:
```
git clone https://github.com/achanda/ipnetwork
cd ipnetwork
cargo test
```

You can also add `ipnetwork` as a dependency to your project's `Cargo.toml`:
```
[dependencies.ipnetwork]
ipnetwork = "*"
```
