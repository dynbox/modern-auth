use std::task::{Context, Poll};
use tower_service::Service;
use crate::gate::AuthGate;

pub struct AuthService<S, T> {
    inner: S,
    gate: T,
}

impl<S, T> AuthService<S, T> {
    pub(crate) fn new(inner: S, gate: T) -> Self {
        Self {
            inner,
            gate,
        }
    }
}

impl<S, T> Service<S> for AuthService<S, T>
    where
        T: AuthGate
{
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