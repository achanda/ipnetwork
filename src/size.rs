use std::{
    cmp::Ordering,
    fmt::Display,
    hash::{Hash, Hasher},
};

use crate::error::NetworkSizeError;
use NetworkSize::*;

/// Represents a generic network size.
///
/// IPv4 network sizes are represented as `u32` values, while IPv6 network sizes are represented as `u128` values.
///
/// # Comparisons
///
/// Network sizes are compared by _value_, not by type.
///
/// ```
/// use ipnetwork::NetworkSize;
///
/// let ns1 = NetworkSize::V4(100);
/// let ns2 = NetworkSize::V6(100);
///
/// assert_eq!(ns1, ns2);
/// ```
#[derive(Debug, Clone, Copy)]
pub enum NetworkSize {
    V4(u32),
    V6(u128),
}

impl NetworkSize {
    /// Returns the size of the network as a `u128`
    fn as_u128(&self) -> u128 {
        match *self {
            V4(a) => a as u128,
            V6(a) => a,
        }
    }
}

impl From<u32> for NetworkSize {
    fn from(value: u32) -> Self {
        V4(value)
    }
}

impl From<u128> for NetworkSize {
    fn from(value: u128) -> Self {
        V6(value)
    }
}

impl TryFrom<NetworkSize> for u32 {
    type Error = NetworkSizeError;
    fn try_from(value: NetworkSize) -> Result<Self, Self::Error> {
        match value {
            V4(a) => Ok(a),
            V6(_) => Err(NetworkSizeError::NetworkIsTooLarge),
        }
    }
}

impl From<NetworkSize> for u128 {
    fn from(val: NetworkSize) -> Self {
        val.as_u128()
    }
}

impl PartialEq for NetworkSize {
    fn eq(&self, other: &Self) -> bool {
        let a = self.as_u128();
        let b = other.as_u128();
        a == b
    }
}

impl Eq for NetworkSize {}

impl Hash for NetworkSize {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let a = self.as_u128();
        a.hash(state);
    }
}

impl Ord for NetworkSize {
    fn cmp(&self, other: &Self) -> Ordering {
        let a = self.as_u128();
        let b = other.as_u128();
        a.cmp(&b)
    }
}

impl PartialOrd for NetworkSize {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Display for NetworkSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_u128())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_u128() {
        let value: u128 = 100;
        let ns = NetworkSize::from(value);
        assert_eq!(ns, V6(100));
    }

    #[test]
    fn test_from_u32() {
        let value: u32 = 100;
        let ns = NetworkSize::from(value);
        assert_eq!(ns, V4(100));
    }

    #[test]
    fn test_try_into_u32() {
        let value: u32 = 100;
        let ns = V4(value);
        let result: Result<u32, _> = ns.try_into();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), value);
    }

    #[test]
    fn test_try_into_u32_error() {
        let value: u128 = u32::MAX as u128 + 1;
        let ns = V6(value);
        let result: Result<u32, _> = ns.try_into();
        assert!(result.is_err());
    }

    #[test]
    fn test_into_u128() {
        let value: u32 = 100;
        let ns = V4(value);
        let result: u128 = ns.into();
        assert_eq!(result, value as u128);
    }

    #[test]
    fn test_eq() {
        let ns1 = V4(100);
        let ns2 = V4(100);
        assert_eq!(ns1, ns2);

        let ns1 = V6(100);
        let ns2 = V6(100);
        assert_eq!(ns1, ns2);

        let ns1 = V4(100);
        let ns2 = V6(100);
        assert_eq!(ns1, ns2);
    }

    #[test]
    fn test_cmp() {
        let ns1 = V4(100);
        let ns2 = V4(200);
        assert!(ns1 < ns2);

        let ns1 = V6(200);
        let ns2 = V6(100);
        assert!(ns1 > ns2);

        let ns1 = V4(100);
        let ns2 = V6(200);
        assert!(ns1 < ns2);
    }

    #[test]
    fn test_display() {
        let ns1 = V4(u32::MAX);
        let ns2 = V6(ns1.into());
        assert_eq!(ns1.to_string(), ns2.to_string());
    }

    // Verify that [`std::hash::Hash`] and [`std::cmp::PartialEq`] are consistent
    #[test]
    fn test_hash() {
        let a = NetworkSize::V4(100);
        let b = NetworkSize::V6(100);

        // Calculate the hash of the two values
        let mut hasher = std::hash::DefaultHasher::default();
        a.hash(&mut hasher);
        let hash_a = hasher.finish();

        let mut hasher = std::hash::DefaultHasher::default();
        b.hash(&mut hasher);
        let hash_b = hasher.finish();

        // a == b
        assert_eq!(a, b);
        // implies hash(a) == hash(b)
        assert_eq!(hash_a, hash_b);
    }
}
