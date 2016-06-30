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
