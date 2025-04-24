use axum::{
    extract::{State, Path},
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

// ==================== 礼物管理接口 ====================

use crate::models::{Gift, GiftEffectType, GiftCategory, GiftFeedbackTemplate};
use crate::db::points::PointsService;

#[derive(Serialize)]
pub struct GiftListResponse {
    gifts: Vec<Gift>,
}

#[derive(Deserialize)]
pub struct CreateGiftPayload {
    name: String,
    description: Option<String>,
    price_lc: u32,
    emotional_value: u32,
    effect_type: GiftEffectType,
    category: GiftCategory,
    image_url: Option<String>,
    is_limited: bool,
    available_until: Option<i64>,
    boost_value: Option<u32>,
}

#[derive(Deserialize)]
pub struct UpdateGiftPayload {
    id: String,
    name: String,
    description: Option<String>,
    price_lc: u32,
    emotional_value: u32,
    effect_type: GiftEffectType,
    category: GiftCategory,
    image_url: Option<String>,
    is_limited: bool,
    available_until: Option<i64>,
    boost_value: Option<u32>,
    is_active: bool,
}

#[derive(Serialize)]
pub struct GiftResponse {
    success: bool,
    gift: Option<Gift>,
}

// 获取所有礼物（管理员用）
#[axum::debug_handler]
pub async fn admin_get_all_gifts(
    State(db): State<Database>,
    auth_user: AuthenticatedUser,
) -> Result<Json<GiftListResponse>, StatusCode> {
    // 验证管理员权限
    let admin = db.get_user_by_id(&auth_user.user_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::UNAUTHORIZED)?;

    if !admin.is_admin() {
        return Err(StatusCode::FORBIDDEN);
    }

    // 获取所有礼物
    let points_service = PointsService::new(db);
    let gifts = points_service.get_all_gifts()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(GiftListResponse { gifts }))
}

// 创建礼物
#[axum::debug_handler]
pub async fn admin_create_gift(
    State(db): State<Database>,
    auth_user: AuthenticatedUser,
    Json(payload): Json<CreateGiftPayload>,
) -> Result<Json<GiftResponse>, StatusCode> {
    // 验证管理员权限
    let admin = db.get_user_by_id(&auth_user.user_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::UNAUTHORIZED)?;

    if !admin.is_admin() {
        return Err(StatusCode::FORBIDDEN);
    }

    // 创建礼物
    let gift = Gift::new(
        payload.name,
        payload.description,
        payload.price_lc,
        payload.emotional_value,
        payload.effect_type,
        payload.category,
        payload.image_url,
        payload.is_limited,
        payload.available_until,
        payload.boost_value,
    );

    // 保存礼物
    let points_service = PointsService::new(db.clone());
    points_service.create_gift(&gift)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // 记录审计日志
    let log = AuditLog::new(
        admin.id,
        AuditAction::AdminAction,
        format!("Created gift: {}", gift.name),
        "".to_string(),
        "".to_string(),
    );
    db.create_audit_log(&log)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(GiftResponse { success: true, gift: Some(gift) }))
}

// 更新礼物
#[axum::debug_handler]
pub async fn admin_update_gift(
    State(db): State<Database>,
    auth_user: AuthenticatedUser,
    Json(payload): Json<UpdateGiftPayload>,
) -> Result<Json<GiftResponse>, StatusCode> {
    // 验证管理员权限
    let admin = db.get_user_by_id(&auth_user.user_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::UNAUTHORIZED)?;

    if !admin.is_admin() {
        return Err(StatusCode::FORBIDDEN);
    }

    // 获取礼物
    let points_service = PointsService::new(db.clone());
    let gift_result = points_service.get_gift_by_id(&payload.id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let gift = match gift_result {
        Some(gift) => gift,
        None => return Err(StatusCode::NOT_FOUND),
    };

    // 更新礼物
    let updated_gift = Gift {
        id: gift.id,
        name: payload.name,
        description: payload.description,
        price_lc: payload.price_lc,
        emotional_value: payload.emotional_value,
        effect_type: payload.effect_type,
        category: payload.category,
        image_url: payload.image_url,
        created_at: gift.created_at,
        is_limited: payload.is_limited,
        available_until: payload.available_until,
        boost_value: payload.boost_value,
        is_active: payload.is_active,
    };

    // 保存更新
    points_service.update_gift(&updated_gift)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // 记录审计日志
    let log = AuditLog::new(
        admin.id,
        AuditAction::AdminAction,
        format!("Updated gift: {}", updated_gift.name),
        "".to_string(),
        "".to_string(),
    );
    db.create_audit_log(&log)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(GiftResponse { success: true, gift: Some(updated_gift) }))
}

// 删除礼物
#[axum::debug_handler]
pub async fn admin_delete_gift(
    State(db): State<Database>,
    auth_user: AuthenticatedUser,
    Path(gift_id): Path<String>,
) -> Result<StatusCode, StatusCode> {
    // 验证管理员权限
    let admin = db.get_user_by_id(&auth_user.user_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::UNAUTHORIZED)?;

    if !admin.is_admin() {
        return Err(StatusCode::FORBIDDEN);
    }

    // 获取礼物
    let points_service = PointsService::new(db.clone());
    let gift_result = points_service.get_gift_by_id(&gift_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let gift = match gift_result {
        Some(gift) => gift,
        None => return Err(StatusCode::NOT_FOUND),
    };

    // 删除礼物
    points_service.delete_gift(&gift_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // 记录审计日志
    let log = AuditLog::new(
        admin.id,
        AuditAction::AdminAction,
        format!("Deleted gift: {}", gift.name),
        "".to_string(),
        "".to_string(),
    );
    db.create_audit_log(&log)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::OK)
}

// ==================== 礼物反馈模板管理 ====================

#[derive(Deserialize)]
pub struct CreateFeedbackTemplatePayload {
    gift_category: GiftCategory,
    feedback_templates: Vec<String>,
}

#[derive(Serialize)]
pub struct FeedbackTemplateResponse {
    success: bool,
}

#[derive(Serialize)]
pub struct FeedbackTemplateListResponse {
    templates: Vec<GiftFeedbackTemplate>,
}

// 创建礼物反馈模板
#[axum::debug_handler]
pub async fn admin_create_feedback_template(
    State(db): State<Database>,
    auth_user: AuthenticatedUser,
    Json(payload): Json<CreateFeedbackTemplatePayload>,
) -> Result<Json<FeedbackTemplateResponse>, StatusCode> {
    // 验证管理员权限
    let admin = db.get_user_by_id(&auth_user.user_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::UNAUTHORIZED)?;

    if !admin.is_admin() {
        return Err(StatusCode::FORBIDDEN);
    }

    // 创建反馈模板
    let template = GiftFeedbackTemplate::new(
        payload.gift_category,
        payload.feedback_templates,
    );

    // 保存模板
    let points_service = PointsService::new(db.clone());
    points_service.create_gift_feedback_template(&template)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // 记录审计日志
    let log = AuditLog::new(
        admin.id,
        AuditAction::AdminAction,
        format!("Created gift feedback template for category: {:?}", template.gift_category),
        "".to_string(),
        "".to_string(),
    );
    db.create_audit_log(&log)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(FeedbackTemplateResponse { success: true }))
}

// 获取礼物反馈模板
#[axum::debug_handler]
pub async fn admin_get_feedback_templates(
    State(db): State<Database>,
    auth_user: AuthenticatedUser,
    Path(category): Path<GiftCategory>,
) -> Result<Json<FeedbackTemplateListResponse>, StatusCode> {
    // 验证管理员权限
    let admin = db.get_user_by_id(&auth_user.user_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::UNAUTHORIZED)?;

    if !admin.is_admin() {
        return Err(StatusCode::FORBIDDEN);
    }

    // 获取反馈模板
    let points_service = PointsService::new(db);
    let templates = points_service.get_gift_feedback_templates(&category)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(FeedbackTemplateListResponse { templates }))
}