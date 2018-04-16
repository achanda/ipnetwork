#[cfg(feature = "with-serde")]
extern crate serde;
#[cfg(feature = "with-serde")]
extern crate serde_json;
#[cfg(feature = "with-serde")]
#[macro_use]
extern crate serde_derive;

extern crate ipnetwork;


#[cfg(test)]
mod tests {

    use ipnetwork::{IpNetwork, Ipv4Network, Ipv6Network};
    use std::net::{Ipv4Addr, Ipv6Addr};

    #[cfg(feature = "with-serde")]
    #[test]
    fn test_ipv4_json() {
        let json_string = r#"{"ipnetwork":{"addr":"127.1.0.0","prefix":24}}"#;

        #[derive(Serialize, Deserialize)]
        struct MyStruct {
            ipnetwork: Ipv4Network,
        }

        let mystruct: MyStruct = ::serde_json::from_str(json_string).unwrap();

        assert_eq!(mystruct.ipnetwork.ip(), Ipv4Addr::new(127, 1, 0, 0));
        assert_eq!(mystruct.ipnetwork.prefix(), 24);

        assert_eq!(::serde_json::to_string(&mystruct).unwrap(), json_string);
    }

    #[cfg(feature = "with-serde")]
    #[test]
    fn test_ipv6_json() {
        let json_string = r#"{"ipnetwork":{"addr":"::1","prefix":0}}"#;

        #[derive(Serialize, Deserialize)]
        struct MyStruct {
            ipnetwork: Ipv6Network,
        }

        let mystruct: MyStruct = ::serde_json::from_str(json_string).unwrap();

        assert_eq!(mystruct.ipnetwork.ip(), Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1));
        assert_eq!(mystruct.ipnetwork.prefix(), 0);

        assert_eq!(::serde_json::to_string(&mystruct).unwrap(), json_string);
    }

    #[cfg(feature = "with-serde")]
    #[test]
    fn test_ipnetwork_json() {
        let json_string = r#"{"ipnetwork":[{"V4":{"addr":"127.1.0.0","prefix":24}},{"V6":{"addr":"::1","prefix":0}}]}"#;

        #[derive(Serialize, Deserialize)]
        struct MyStruct {
            ipnetwork: Vec<IpNetwork>,
        }

        let mystruct: MyStruct = ::serde_json::from_str(json_string).unwrap();

        assert_eq!(mystruct.ipnetwork[0].ip(), Ipv4Addr::new(127, 1, 0, 0));
        assert_eq!(mystruct.ipnetwork[0].prefix(), 24);
        assert_eq!(mystruct.ipnetwork[1].ip(), Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1));
        assert_eq!(mystruct.ipnetwork[1].prefix(), 0);

        assert_eq!(::serde_json::to_string(&mystruct).unwrap(), json_string);
    }
}