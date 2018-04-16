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

    use ipnetwork::Ipv4Network;
    use std::net::Ipv4Addr;

    #[cfg(feature = "with-serde")]
    #[test]
    fn test_ipv4_json() {
        let json_string = "{\"ipnetwork\":{\"addr\":\"127.1.0.0\",\"prefix\":24}}";

        #[derive(Serialize, Deserialize)]
        struct MyStruct {
            ipnetwork: Ipv4Network,
        }

        let mystruct: MyStruct = ::serde_json::from_str(json_string).unwrap();

        assert_eq!(mystruct.ipnetwork.ip(), Ipv4Addr::new(127, 1, 0, 0));
        assert_eq!(mystruct.ipnetwork.prefix(), 24);

        assert_eq!(::serde_json::to_string(&mystruct).unwrap(), json_string);
    }
}