use axum::{
    extract::State,
    Json,
    http::StatusCode,
};
use serde::{Deserialize, Serialize};

use crate::{
    db::Database,
    middleware::AuthenticatedUser,
    models::{User, UserRole, AuditLog, AuditAction},
};

#[derive(Deserialize)]
pub struct UpdateUserRolePayload {
    user_id: String,
    new_role: UserRole,
}

pub async fn update_user_role(
    State(db): State<Database>,
    auth_user: AuthenticatedUser,
    Json(payload): Json<UpdateUserRolePayload>,
) -> Result<StatusCode, StatusCode> {
    // 获取管理员信息
    let admin = db.get_user_by_id(&auth_user.user_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // 验证管理员权限
    if !admin.roles.contains(&UserRole::SuperAdmin) && !admin.roles.contains(&UserRole::Admin) {
        return Err(StatusCode::FORBIDDEN);
    }

    // 获取目标用户
    let mut user = db.get_user_by_id(&payload.user_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    // 只有超级管理员可以修改管理员权限
    if (payload.new_role == UserRole::Admin || payload.new_role == UserRole::SuperAdmin) 
        && !admin.roles.contains(&UserRole::SuperAdmin) {
        return Err(StatusCode::FORBIDDEN);
    }

    // 更新用户角色
    user.roles = vec![payload.new_role];
    db.update_user(&user)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // 记录审计日志
    let log = AuditLog::new(
        admin.id,
        AuditAction::AdminAction,
        format!("Updated user {} role to {:?}", user.id, payload.new_role),
        "".to_string(), // TODO: 从请求中获取IP
        "".to_string(), // TODO: 从请求中获取User-Agent
    );
    db.create_audit_log(&log)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::OK)
}

#[derive(Serialize)]
pub struct AuditLogResponse {
    logs: Vec<AuditLog>,
}

pub async fn view_audit_logs(
    State(db): State<Database>,
    auth_user: AuthenticatedUser,
) -> Result<Json<AuditLogResponse>, StatusCode> {
    // 获取管理员信息
    let admin = db.get_user_by_id(&auth_user.user_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // 验证管理员权限
    if !admin.roles.iter().any(|role| matches!(role, UserRole::SuperAdmin | UserRole::Admin | UserRole::Moderator)) {
        return Err(StatusCode::FORBIDDEN);
    }

    // 获取审计日志
    let logs = db.get_user_audit_logs(&auth_user.user_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(AuditLogResponse { logs }))
}
