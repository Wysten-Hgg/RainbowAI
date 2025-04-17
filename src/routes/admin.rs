use axum::{
    extract::State,
    Json,
    http::StatusCode,
};
use serde::{Deserialize, Serialize};

use crate::{
    db::Database,
    middleware::auth::AuthenticatedUser,
    models::{User, AuditLog, AuditAction},
    models::user::BackendUserRole,
};

#[derive(Deserialize)]
pub struct UpdateUserRolePayload {
    user_id: String,
    new_role: String,
}

#[axum::debug_handler]
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
    if !admin.is_admin() {
        return Err(StatusCode::FORBIDDEN);
    }

    // 获取目标用户
    let mut user = db.get_user_by_id(&payload.user_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    // 只有超级管理员可以修改管理员权限
    if (payload.new_role == "Admin" || payload.new_role == "SuperAdmin") 
        && !admin.backend_roles.contains(&BackendUserRole::SuperAdmin) {
        return Err(StatusCode::FORBIDDEN);
    }

    // 更新用户角色
    // 将字符串转换为 BackendUserRole
    let new_role = match payload.new_role.as_str() {
        "Admin" => BackendUserRole::Admin,
        "SuperAdmin" => BackendUserRole::SuperAdmin,
        "Moderator" => BackendUserRole::Moderator,
        "Editor" => BackendUserRole::Editor,
        "Viewer" => BackendUserRole::Viewer,
        _ => return Err(StatusCode::BAD_REQUEST),
    };
    user.backend_roles = vec![new_role];
    
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

#[axum::debug_handler]
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
    if !admin.backend_roles.iter().any(|role| matches!(role, BackendUserRole::SuperAdmin | BackendUserRole::Admin | BackendUserRole::Moderator)) {
        return Err(StatusCode::FORBIDDEN);
    }

    // 获取审计日志
    let logs = db.get_user_audit_logs(&auth_user.user_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(AuditLogResponse { logs }))
}
