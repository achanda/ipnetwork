# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.21.1](https://github.com/achanda/ipnetwork/compare/v0.21.0...v0.21.1) - 2025-01-07

### Other

- Fix for 0::/0 network ([#205](https://github.com/achanda/ipnetwork/pull/205))

## [0.21.0](https://github.com/achanda/ipnetwork/compare/v0.20.0...v0.21.0) - 2025-01-06

### Fixed

- fix for 0.0.0.0/0 network ([#199](https://github.com/achanda/ipnetwork/pull/199))
- *(deps)* update rust crate serde to 1.0.200 (#196)
- *(deps)* update rust crate serde to 1.0.199 (#194)
- use associated constants (#191)

### Other

- Update .gitignore
- Update publish.yml
- Update publish.yml
- Replace `Ipv{4,6}Network::new_unchecked` with `Ipv{4,6}Network::new_checked` ([#203](https://github.com/achanda/ipnetwork/pull/203))
- Make the serde feature opt-in instead of opt-out ([#200](https://github.com/achanda/ipnetwork/pull/200))
- Fix typo ([#198](https://github.com/achanda/ipnetwork/pull/198))
- Update publish.yml ([#195](https://github.com/achanda/ipnetwork/pull/195))
- *(deps)* update rust crate serde_json to 1.0.116 (#193)
- setup release plz (#192)
- Update Rust crate criterion to 0.5.1 ([#172](https://github.com/achanda/ipnetwork/pull/172))
- Update actions/checkout action to v4 ([#182](https://github.com/achanda/ipnetwork/pull/182))
- rewrite core ipv6 methods to operate on u128s  (#187)
- move to dtolnay/rust-toolchain and run clippy (#189)
- Hash implementation to match PartialEq (#186)
- Update Rust crate schemars to 0.8.17 ([#184](https://github.com/achanda/ipnetwork/pull/184))
- Add const unsafe `new_unchecked` to `Ipv4Network` & `Ipv6Network` ([#185](https://github.com/achanda/ipnetwork/pull/185))
- Update Rust crate schemars to 0.8.15 ([#183](https://github.com/achanda/ipnetwork/pull/183))
- Update Rust crate schemars to 0.8.13 ([#181](https://github.com/achanda/ipnetwork/pull/181))
- Add `Ipv6Network::nth` to get the nth address (take two) ([#176](https://github.com/achanda/ipnetwork/pull/176))
- Added needed traits to `NetworkSize` ([#175](https://github.com/achanda/ipnetwork/pull/175))
- Update criterion requirement from 0.4.0 to 0.5.0
- Update katyo/publish-crates action to v2
- Update actions/checkout action to v3
- Update Rust crate schemars to 0.8.12
- Add renovate.json
- Replace assert_eq with assert for bool comparison
- Use cargo clippy --fix to autofix code
- Add a reference where missing
- Cleanup mask for Ipv4Addr
- Shrink the enumerate call on mask
- Cleanup both size functions
- Simplify FromStr for Ipv6Network
- Make parse_prefix more idiomatic
- Update criterion requirement from 0.3.4 to 0.4.0 ([#162](https://github.com/achanda/ipnetwork/pull/162))
- Update does-it-json requirement from 0.0.3 to 0.0.4 ([#161](https://github.com/achanda/ipnetwork/pull/161))
