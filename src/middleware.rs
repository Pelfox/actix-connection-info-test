use std::{
    future::{ready, Ready},
    rc::Rc,
};

use actix_web::{
    body::EitherBody,
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpResponse,
};
use futures::{future::LocalBoxFuture, FutureExt};

pub struct ExampleMiddlewareFactory {}

impl<S, B> Transform<S, ServiceRequest> for ExampleMiddlewareFactory
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Transform = ExampleMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(ExampleMiddleware {
            service: Rc::new(service),
        }))
    }
}

pub struct ExampleMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for ExampleMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let body = format!("{:#?}", req.connection_info());
        async move {
            Ok(req
                .into_response(HttpResponse::Ok().body(body))
                .map_into_right_body())
        }
        .boxed_local()
    }
}
