use axum::{
    async_trait,
    extract::{FromRequestParts, State},
    http::{request::Parts, StatusCode, Request},
    middleware::Next,
    response::Response,
    body::Body,
};
use axum::http::header::{AUTHORIZATION};

use crate::utils::jwt;
use crate::db::Database;

pub struct AuthenticatedUser {
    pub user_id: String,
}

impl AuthenticatedUser {
    pub fn is_admin(&self) -> bool {
        // 简单实现，后续可以从数据库或JWT中获取用户角色
        // 这里暂时返回false，实际项目中需要根据用户角色判断
        false
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthenticatedUser
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        // 从header中获取token
        let auth_header = parts.headers.get(AUTHORIZATION).ok_or(StatusCode::UNAUTHORIZED)?;

        // 验证token
        let claims = jwt::verify_token(auth_header.to_str().map_err(|_| StatusCode::UNAUTHORIZED)?)
            .map_err(|_| StatusCode::UNAUTHORIZED)?;

        Ok(Self {
            user_id: claims.sub,
        })
    }
}

// 身份验证中间件
pub async fn auth_middleware<B>(response: Response<B>) -> Response<B> {
    // 简单实现，只返回原始响应
    // 实际项目中可能需要更复杂的逻辑，如检查用户角色、权限等
    response
}
