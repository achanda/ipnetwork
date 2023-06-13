use crate::error::IpNetworkError;

pub fn cidr_parts(cidr: &str) -> Result<(&str, Option<&str>), IpNetworkError> {
    // Try to find a single slash
    if let Some(sep) = cidr.find('/') {
        let (ip, prefix) = cidr.split_at(sep);
        // Error if cidr has multiple slashes
        if prefix[1..].find('/').is_some() {
            Err(IpNetworkError::InvalidCidrFormat(format!(
                "CIDR must contain a single '/': {cidr}"
            )))
        } else {
            // Handle the case when cidr has exactly one slash
            Ok((ip, Some(&prefix[1..])))
        }
    } else {
        // Handle the case when cidr does not have a slash
        Ok((cidr, None))
    }
}

pub fn parse_prefix(prefix: &str, max: u8) -> Result<u8, IpNetworkError> {
    prefix
        .parse()
        .map_err(|_| IpNetworkError::InvalidPrefix)
        .and_then(|mask| {
            if mask > max {
                Err(IpNetworkError::InvalidPrefix)
            } else {
                Ok(mask)
            }
        })
}
