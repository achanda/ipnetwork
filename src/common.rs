use std::net::Ipv4Addr;

#[derive(Debug,Clone,PartialEq,Eq)]
pub enum IpNetworkError {
    InvalidAddr(String),
    InvalidPrefix,
    InvalidCidrFormat(String),
}

pub fn cidr_parts<'a>(cidr: &'a str) -> Result<(&'a str, &'a str), IpNetworkError> {
    let parts = cidr.split('/').collect::<Vec<&str>>();
    if parts.len() == 2 {
        Ok((parts[0], parts[1]))
    } else {
        Err(IpNetworkError::InvalidCidrFormat(format!("CIDR must contain '/': {}", cidr)))
    }
}

pub fn parse_prefix(prefix: &str, max: u8) -> Result<u8, IpNetworkError> {
    let mask = try!(prefix.parse::<u8>().map_err(|_| IpNetworkError::InvalidPrefix));
    if mask > max {
        Err(IpNetworkError::InvalidPrefix)
    } else {
        Ok(mask)
    }
}

pub fn parse_addr(addr: &str) -> Result<Ipv4Addr, IpNetworkError> {
    let addr_parts = addr.split('.').map(|b| b.parse::<u8>());
    let mut bytes = [0; 4];
    for (i, byte) in addr_parts.enumerate() {
        if i >= 4 {
            return Err(IpNetworkError::InvalidAddr(format!("More than 4 bytes: {}", addr)));
        }
        bytes[i] = try!(byte.map_err(|_| {
                IpNetworkError::InvalidAddr(format!("All bytes not 0-255: {}", addr))
            }));
    }
    Ok(Ipv4Addr::new(bytes[0], bytes[1], bytes[2], bytes[3]))
}
