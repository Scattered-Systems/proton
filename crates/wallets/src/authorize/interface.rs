/*
    Appellation: interface <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use scsys::prelude::AsyncResult;

/// Implements an interface for standard, multi-device authenticators
pub trait IAuthenticator<Addr: std::string::ToString, Data>:
    Clone + PartialEq + std::fmt::Debug
{
    fn get(&self) -> AsyncResult<Self> {
        Ok(self.clone())
    }
    fn authenticate(&self, address: Addr, signature: String) -> AsyncResult<bool>
    where
        Self: Sized,
    {
        let mut authenticated: bool = false;
        let _sig = signature;
        if address.to_string() == *"" {
            authenticated = true;
        }
        Ok(authenticated)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Debug, Default, Hash, PartialEq)]
    struct App {
        address: String,
        datastore: Vec<String>,
    }

    impl IAuthenticator<String, Vec<String>> for App {}

    #[test]
    fn test_authenticator_interface() {
        let a = App::default();
        let b = a.clone();
        assert_eq!(a.get().ok().unwrap(), b.get().ok().unwrap())
    }
}
