use tower_layer::Layer;
use crate::service::AuthService;

pub struct AuthLayer<T> {
    gate: T
}

impl<T> AuthLayer<T> {
    pub fn new(gate: T) -> Self {
        Self {
            gate
        }
    }
}

impl<S, T> Layer<S> for AuthLayer<T>
where
    T: Clone,
{
    type Service = AuthService<S, T>;

    fn layer(&self, inner: S) -> Self::Service {
        AuthService::new(inner, self.gate.clone())
    }
}