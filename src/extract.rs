use async_trait::async_trait;
use axum::http::request::Parts;
use axum_core::extract::FromRequestParts;
use crate::gate::{AuthCreds, AuthGate};

struct AuthManager<T>
    where
        T: AuthGate
{
    gate: T,
    pub user: Option<T::User>
}

impl<T> AuthManager<T>
    where
        T: AuthGate
{
    pub(crate) fn new() -> Self {
        todo!()
    }

    fn validate_header(&self) {
    }

    async fn authenticate(&self) -> Result<Option<T::User>, ()> {
        let creds = T::Credentials::from_header();

        self.gate.authenticate(creds)
            .await
    }
}

#[async_trait]
impl<S, T> FromRequestParts<S> for AuthManager<T>
    where
        T: AuthGate,
        S: Send + Sync,
{
    type Rejection = ();

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        todo!()
    }
}