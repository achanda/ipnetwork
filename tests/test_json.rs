#![cfg(feature = "serde")]

#[cfg(test)]
mod tests {
    use ipnetwork::{IpNetwork, Ipv4Network, Ipv6Network};
    use serde::{Deserialize, Serialize};
    use std::net::{Ipv4Addr, Ipv6Addr};

    #[test]
    fn test_ipv4_json() {
        let json_string = r#"{"ipnetwork":"127.1.0.0/24"}"#;

        #[derive(Serialize, Deserialize)]
        #[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
        struct MyStruct {
            ipnetwork: Ipv4Network,
        }

        let mystruct: MyStruct = ::serde_json::from_str(json_string).unwrap();

        assert_eq!(mystruct.ipnetwork.ip(), Ipv4Addr::new(127, 1, 0, 0));
        assert_eq!(mystruct.ipnetwork.prefix(), 24);

        assert_eq!(::serde_json::to_string(&mystruct).unwrap(), json_string);

        #[cfg(feature = "schemars")]
        {
            // Validate that we can generate a schema and it contains expected properties
            let schema = schemars::schema_for!(MyStruct);
            let schema_value = serde_json::to_value(&schema).unwrap();
            
            // Verify the schema has the expected structure
            assert!(schema_value.get("properties").is_some());
            assert!(schema_value["properties"].get("ipnetwork").is_some());
            
            // Verify our struct can be serialized to JSON that matches the schema expectations
            let json_value = serde_json::to_value(&mystruct).unwrap();
            assert!(json_value.get("ipnetwork").is_some());
            // For single IpNetwork values, the JSON value should be a string
            assert!(json_value["ipnetwork"].is_string());
        }
    }

    #[test]
    fn test_ipv6_json() {
        let json_string = r#"{"ipnetwork":"::1/0"}"#;

        #[derive(Serialize, Deserialize)]
        #[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
        struct MyStruct {
            ipnetwork: Ipv6Network,
        }

        let mystruct: MyStruct = ::serde_json::from_str(json_string).unwrap();

        assert_eq!(
            mystruct.ipnetwork.ip(),
            Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1)
        );
        assert_eq!(mystruct.ipnetwork.prefix(), 0);

        assert_eq!(::serde_json::to_string(&mystruct).unwrap(), json_string);

        #[cfg(feature = "schemars")]
        {
            // Validate that we can generate a schema and it contains expected properties
            let schema = schemars::schema_for!(MyStruct);
            let schema_value = serde_json::to_value(&schema).unwrap();
            
            // Verify the schema has the expected structure
            assert!(schema_value.get("properties").is_some());
            assert!(schema_value["properties"].get("ipnetwork").is_some());
            
            // Verify our struct can be serialized to JSON that matches the schema expectations
            let json_value = serde_json::to_value(&mystruct).unwrap();
            assert!(json_value.get("ipnetwork").is_some());
            // For single IpNetwork values, the JSON value should be a string
            assert!(json_value["ipnetwork"].is_string());
        }
    }

    #[test]
    fn test_ipnetwork_json() {
        let json_string = r#"{"ipnetwork":["127.1.0.0/24","::1/0"]}"#;

        #[derive(Serialize, Deserialize)]
        #[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
        struct MyStruct {
            ipnetwork: Vec<IpNetwork>,
        }

        let mystruct: MyStruct = ::serde_json::from_str(json_string).unwrap();

        assert_eq!(mystruct.ipnetwork[0].ip(), Ipv4Addr::new(127, 1, 0, 0));
        assert_eq!(mystruct.ipnetwork[0].prefix(), 24);
        assert_eq!(
            mystruct.ipnetwork[1].ip(),
            Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1)
        );
        assert_eq!(mystruct.ipnetwork[1].prefix(), 0);

        assert_eq!(::serde_json::to_string(&mystruct).unwrap(), json_string);

        #[cfg(feature = "schemars")]
        {
            // Validate that we can generate a schema and it contains expected properties
            let schema = schemars::schema_for!(MyStruct);
            let schema_value = serde_json::to_value(&schema).unwrap();
            
            // Verify the schema has the expected structure
            assert!(schema_value.get("properties").is_some());
            assert!(schema_value["properties"].get("ipnetwork").is_some());
            
            // Verify our struct can be serialized to JSON that matches the schema expectations
            let json_value = serde_json::to_value(&mystruct).unwrap();
            assert!(json_value.get("ipnetwork").is_some());
            // For Vec<IpNetwork>, the JSON value should be an array
            assert!(json_value["ipnetwork"].is_array());
        }
    }


    #[test]
    fn test_ipnetwork_size_with_prefix_0() {
        let network: Ipv4Network = "0.0.0.0/0".parse().unwrap();
        let size = network.size();
        assert_eq!(size, u32::MAX);
    }

    #[test]
    #[cfg(feature = "schemars")]
    fn test_schema_generation() {
        // Test that we can generate schemas for all network types
        let ipv4_schema = schemars::schema_for!(Ipv4Network);
        let ipv6_schema = schemars::schema_for!(Ipv6Network);
        let ip_schema = schemars::schema_for!(IpNetwork);
        
        // Convert to JSON to verify structure
        let ipv4_json = serde_json::to_value(&ipv4_schema).unwrap();
        let ipv6_json = serde_json::to_value(&ipv6_schema).unwrap();
        let ip_json = serde_json::to_value(&ip_schema).unwrap();
        
        // Verify IPv4 schema has string type and pattern
        assert_eq!(ipv4_json["type"], "string");
        assert!(ipv4_json.get("pattern").is_some());
        assert_eq!(ipv4_json["x-rust-type"], "ipnetwork::Ipv4Network");
        
        // Verify IPv6 schema has string type and pattern  
        assert_eq!(ipv6_json["type"], "string");
        assert!(ipv6_json.get("pattern").is_some());
        assert_eq!(ipv6_json["x-rust-type"], "ipnetwork::Ipv6Network");
        
        // Verify IpNetwork schema has oneOf structure
        assert!(ip_json.get("oneOf").is_some());
        assert_eq!(ip_json["x-rust-type"], "ipnetwork::IpNetwork");
        
        let one_of = ip_json["oneOf"].as_array().unwrap();
        assert_eq!(one_of.len(), 2);
        assert_eq!(one_of[0]["title"], "v4");
        assert_eq!(one_of[1]["title"], "v6");
        
        // Verify that the schemas follow the schemars 1.0 migration guide patterns
        // The Schema should be a wrapper around serde_json::Value
        assert!(ipv4_json.is_object());
        assert!(ipv6_json.is_object());
        assert!(ip_json.is_object());
        
        // Print schemas for manual verification (useful for debugging)
        println!("IPv4 Schema: {}", serde_json::to_string_pretty(&ipv4_json).unwrap());
        println!("IPv6 Schema: {}", serde_json::to_string_pretty(&ipv6_json).unwrap());
        println!("IpNetwork Schema: {}", serde_json::to_string_pretty(&ip_json).unwrap());
    }
}
