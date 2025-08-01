// src/handlers/admin_middleware.rs
use actix_web::{dev::{Service, ServiceRequest, ServiceResponse, Transform}, Error, HttpMessage, HttpResponse};
use futures::future::{ok, Ready};
use crate::{errors::AppError, models::user::Claims};
//use std::{future::{Future, ReadyOrNot}, pin::Pin};

pub struct AdminAuth;

// ... (这里需要完整的中间件实现代码，为了简洁，我们直接展示最终版本)
// ... (它会复用Auth中间件的逻辑，但增加一步 is_admin 的检查)

// 为了简化，我们直接在 handler 中检查，但中间件是更专业的做法。
// 我们先继续，如果需要，再回头实现完整的中间件。