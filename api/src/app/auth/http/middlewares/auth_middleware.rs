use axum::{
    body::Body, extract::Request, http::{header,  StatusCode}, response::Response
};
use futures_util::future::BoxFuture;
use tower::{Service, Layer};
use std::{convert::Infallible, task::{Context, Poll}};

use crate::app::auth::strategies::jwt::JwtStrategy;

#[derive(Clone)]
pub struct AuthLayer;

impl<S> Layer<S> for AuthLayer {
    type Service = AuthMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        AuthMiddleware { inner }
    }
}

#[derive(Clone)]
pub struct AuthMiddleware<S> {
    inner: S,
}

impl<S> Service<Request> for AuthMiddleware<S>
where
    S: Service<Request<Body>, Response = Response<Body>, Error = Infallible> + Clone + Send + 'static,
    S::Future: Send + 'static,
    S::Error: Into<Response> + Send,
{
    type Response = Response;
    type Error = S::Error;
    type Future = BoxFuture<'static, Result<Response, S::Error>>;
    

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request) -> Self::Future {
        if req.uri().path().contains("auth") {
            let fut = self.inner.call(req);

            return Box::pin(fut);
        }


        let auth_header = req.headers()
            .get(header::AUTHORIZATION)
            .and_then(|h| h.to_str().ok());

        let token = match auth_header {
            Some(token) => token.to_string(),
            None => {
                return Box::pin(async {
                    Ok(Response::builder()
                        .status(StatusCode::UNAUTHORIZED)
                        .body(Body::from("Missing auth header"))
                        .unwrap())
                });
            }
        };


            let fut = self.inner.call(req);

            match JwtStrategy::verify(&token) {
                Ok(_) => 
        Box::pin(async move {
            
            let response = fut.await?;

                Ok(response)
        }),
                Err(_) => Box::pin(async move {
            
            let response = fut.await?;

                Ok(response)
        })
        }
    }
}
