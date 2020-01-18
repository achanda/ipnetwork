#![cfg(feature = "serde")]

#[cfg(test)]
mod tests {
    use ipnetwork::{IpNetwork, Ipv4Network, Ipv6Network};
    use serde_derive::{Deserialize, Serialize};
    use std::net::{Ipv4Addr, Ipv6Addr};

    #[test]
    fn test_ipv4_json() {
        let json_string = r#"{"ipnetwork":"127.1.0.0/24"}"#;

        #[derive(Serialize, Deserialize)]
        struct MyStruct {
            ipnetwork: Ipv4Network,
        }

        let mystruct: MyStruct = ::serde_json::from_str(json_string).unwrap();

        assert_eq!(mystruct.ipnetwork.network(), Ipv4Addr::new(127, 1, 0, 0));
        assert_eq!(mystruct.ipnetwork.prefix(), 24);

        assert_eq!(::serde_json::to_string(&mystruct).unwrap(), json_string);
    }

    #[test]
    fn test_ipv6_json() {
        let json_string = r#"{"ipnetwork":"::/0"}"#;

        #[derive(Serialize, Deserialize)]
        struct MyStruct {
            ipnetwork: Ipv6Network,
        }

        let mystruct: MyStruct = ::serde_json::from_str(json_string).unwrap();

        assert_eq!(
            mystruct.ipnetwork.network(),
            Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0)
        );
        assert_eq!(mystruct.ipnetwork.prefix(), 0);

        assert_eq!(::serde_json::to_string(&mystruct).unwrap(), json_string);
    }

    #[test]
    fn test_ipnetwork_json() {
        let json_string = r#"{"ipnetwork":["127.1.0.0/24","::/0"]}"#;

        #[derive(Serialize, Deserialize)]
        struct MyStruct {
            ipnetwork: Vec<IpNetwork>,
        }

        let mystruct: MyStruct = ::serde_json::from_str(json_string).unwrap();

        assert_eq!(mystruct.ipnetwork[0].network(), Ipv4Addr::new(127, 1, 0, 0));
        assert_eq!(mystruct.ipnetwork[0].prefix(), 24);
        assert_eq!(
            mystruct.ipnetwork[1].network(),
            Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0)
        );
        assert_eq!(mystruct.ipnetwork[1].prefix(), 0);

        assert_eq!(::serde_json::to_string(&mystruct).unwrap(), json_string);
    }
}
