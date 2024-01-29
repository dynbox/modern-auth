use std::task::{Context, Poll};
use tower_service::Service;

pub struct AuthService<S> {
    inner: S
}

impl<S> AuthService<S> {
    pub(crate) fn new(inner: S) -> Self {
        Self {
            inner
        }
    }
}

impl<S> Service<S> for AuthService<S> {
    type Response = ();
    type Error = ();
    type Future = ();

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        todo!()
    }

    fn call(&mut self, req: S) -> Self::Future {
        todo!()
    }
}