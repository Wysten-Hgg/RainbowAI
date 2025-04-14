use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    headers::{Authorization, HeaderMapExt},
};

use crate::utils::jwt;

pub struct AuthenticatedUser {
    pub user_id: String,
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthenticatedUser
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        // 从header中获取token
        let auth_header = parts
            .headers
            .typed_get::<Authorization<String>>()
            .ok_or(StatusCode::UNAUTHORIZED)?;

        // 验证token
        let claims = jwt::verify_token(auth_header.as_str())
            .map_err(|_| StatusCode::UNAUTHORIZED)?;

        Ok(Self {
            user_id: claims.sub,
        })
    }
}
