use super::prelude::*;
use proc_macro::Context;

pub trait Has<T> {
    fn get(&self) -> &T;
    fn get_mut(&mut self) -> &mut T;
}


/// Context, created for each Client instance.
#[derive(Clone, Debug, Context)]
pub struct ClientContext {
    #[has]
    pub timeout: RequestTimeout,
    #[has]
    pub auth_data: Option<Authorization<Basic>>,
    #[has]
    pub xspan: XSpanIdString,
}

/// Context wrapper, to bind an API with a context.
#[derive(Debug, Clone)]
pub struct ContextWrapper<T, C> {
    api: T,
    context: C,
}

impl<T, C> ContextWrapper<T, C> {
    /// Create a new `ContextWrapper`, binding the API and context.
    pub const fn new(api: T, context: C) -> Self {
        Self { api, context }
    }

    /// Borrows the API.
    pub const fn api(&self) -> &T {
        &self.api
    }

    /// Borrows the context.
    pub const fn context(&self) -> &C {
        &self.context
    }
}

/// Middleware that wraps a `hyper::service::Service` and drops any contextual information
/// on the request.
#[derive(Debug, Clone)]
pub struct DropContextService<T, C>
where
    C: Send + 'static,
{
    inner: T,
    marker: PhantomData<C>,
}

impl<T, C> DropContextService<T, C>
where
    C: Send + 'static,
{
    /// Create a new `DropContextService` struct wrapping a value
    pub const fn new(inner: T) -> Self {
        Self {
            inner,
            marker: PhantomData,
        }
    }
}

impl<Inner, Body, Context> hyper::service::Service<(Request<Body>, Context)>
    for DropContextService<Inner, Context>
where
    Context: Send + 'static,
    Inner: hyper::service::Service<Request<Body>>,
{
    type Response = Inner::Response;
    type Error = Inner::Error;
    type Future = Inner::Future;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<std::result::Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, (req, _): (Request<Body>, Context)) -> Self::Future {
        self.inner.call(req)
    }
}
