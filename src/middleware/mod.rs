pub mod auth;

use axum::{
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use crate::db::Database;

// 身份验证中间件
pub async fn auth_middleware<B>(
    db: Database,
    req: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    // 简单实现，只检查用户是否通过了 AuthenticatedUser 提取器
    // 实际项目中可能需要更复杂的逻辑，如检查用户角色、权限等
    let response = next.run(req).await;
    Ok(response)
}
