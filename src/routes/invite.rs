use axum::{
    extract::State,
    Json,
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use time::{OffsetDateTime};

use crate::{
    db::Database,
    middleware::AuthenticatedUser,
    models::{Invite, User, VipLevel},
};

#[derive(Serialize)]
pub struct CreateInviteResponse {
    invite: Invite,
}

pub async fn create_invite(
    State(db): State<Database>,
    auth_user: AuthenticatedUser,
) -> Result<Json<CreateInviteResponse>, StatusCode> {
    // 获取用户信息
    let user = db.get_user_by_id(&auth_user.user_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    // 根据VIP等级设置不同的使用上限
    let usage_limit = match user.vip_level {
        VipLevel::Free => 10,
        VipLevel::Pro => 30,
        VipLevel::Premium => 50,
        VipLevel::Ultimate => 100,
        VipLevel::Team => 100, // 单账号每周100次
    };

    // 创建邀请码
    let invite = Invite::new(user.id, usage_limit);
    db.create_invite(&invite)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(CreateInviteResponse { invite }))
}

#[derive(Deserialize)]
pub struct UseInvitePayload {
    code: String,
}

pub async fn use_invite(
    State(db): State<Database>,
    auth_user: AuthenticatedUser,
    Json(payload): Json<UseInvitePayload>,
) -> Result<StatusCode, StatusCode> {
    // 获取邀请码信息
    let mut invite = db.get_invite(&payload.code)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    // 检查邀请码是否可用
    if !invite.can_be_used() {
        return Err(StatusCode::GONE);
    }

    // 检查用户是否已经使用过这个邀请码
    if invite.used_by.contains(&auth_user.user_id) {
        return Err(StatusCode::CONFLICT);
    }

    // 更新用户的Pro体验
    let mut user = db.get_user_by_id(&auth_user.user_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;
    user.vip_level = VipLevel::Pro;
    user.pro_experience_expiration = Some(OffsetDateTime::now_utc().unix_timestamp() + 7 * 24 * 60 * 60);
    db.update_user(&user).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // 更新邀请码使用记录
    invite.used_by.push(auth_user.user_id);
    db.update_invite(&invite)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::OK)
}
