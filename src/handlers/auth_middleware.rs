// src/handlers/auth_middleware.rs

use crate::{errors::AppError, utils::auth_utils};
use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage,
};
use futures::future::{ok, Ready};
use std::future::Future;
use std::pin::Pin;

// 中间件工厂
pub struct Auth;

impl<S, B> Transform<S, ServiceRequest> for Auth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AuthMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthMiddleware { service })
    }
}

// 中间件服务
pub struct AuthMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    actix_web::dev::forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // 从请求头中提取 "Authorization"
        let auth_header = req.headers().get("Authorization");

        // 验证token
        match auth_header {
            Some(header_value) => {
                // 期望的格式是 "Bearer <token>"
                if let Ok(auth_str) = header_value.to_str() {
                    if auth_str.starts_with("Bearer ") {
                        let token = &auth_str[7..];
                        match auth_utils::validate_jwt(token) {
                            Ok(claims) => {
                                // 验证成功，将用户信息 (claims) 存入请求的扩展中，方便后续处理器使用
                                req.extensions_mut().insert(claims);
                            }
                            Err(_) => {
                                // token无效或过期
                                let err = AppError::AuthError;
                                return Box::pin(async move { Err(err.into()) });
                            }
                        }
                    } else {
                        // Header格式不正确
                        let err = AppError::AuthError;
                        return Box::pin(async move { Err(err.into()) });
                    }
                }
            }
            None => {
                // 缺少 Authorization header
                let err = AppError::AuthError;
                return Box::pin(async move { Err(err.into()) });
            }
        }

        // 将请求传递给下一个服务
        let fut = self.service.call(req);
        Box::pin(async move {
            let res = fut.await?;
            Ok(res)
        })
    }
}