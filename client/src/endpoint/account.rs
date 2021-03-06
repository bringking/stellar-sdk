//! Contains endpoints for accessing accounts and related information.
use error::Result;
use std::str::FromStr;
use stellar_resources::Account;
use super::{Body, EndPoint};
use http::{Request, Uri};

/// An endpoint that accesses a single accounts details.
#[derive(Debug)]
pub struct Details {
    id: String,
}

impl Details {
    /// Returns a new end point for account details. Hand this to the client in order to request
    /// details about an account.
    pub fn new(id: &str) -> Self {
        Self { id: id.to_string() }
    }
}

impl EndPoint for Details {
    type Response = Account;

    fn into_request(self, host: &str) -> Result<Request<Body>> {
        let uri = Uri::from_str(&format!("{}/accounts/{}", host, self.id))?;
        let request = Request::get(uri).body(Body::None)?;
        Ok(request)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_can_make_an_account_uri() {
        let details = Details::new("abc123");
        let request = details
            .into_request("https://horizon-testnet.stellar.org")
            .unwrap();
        assert_eq!(request.uri().host().unwrap(), "horizon-testnet.stellar.org");
        assert_eq!(request.uri().path(), "/accounts/abc123");
    }
}
