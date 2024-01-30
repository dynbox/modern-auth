use async_trait::async_trait;

#[async_trait]
pub trait AuthGate {
    type Credentials: AuthCreds;
    type User;

    async fn authenticate(
        &self,
        creds: Self::Credentials
    ) -> Result<Option<Self::User>, ()>;
}

pub trait AuthCreds {
    fn from_header() -> Self;
}