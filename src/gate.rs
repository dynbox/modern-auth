pub trait AuthGate {
    type Credentials;
    type User;

    async fn authenticate(
        &self,
        creds: Self::Credentials
    ) -> Result<Option<Self::User>, ()>;
}
