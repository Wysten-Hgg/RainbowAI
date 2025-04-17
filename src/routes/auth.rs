use axum::{
    Json,
    http::StatusCode,
    extract::State,
};
use serde::{Deserialize, Serialize};
use bcrypt::{hash, DEFAULT_COST};
use time::{OffsetDateTime};

use crate::{models::{User, VipLevel}, db::Database, utils::jwt};

#[derive(Deserialize)]
pub struct RegisterPayload {
    email: String,
    password: String,
    invite_code: Option<String>,
}

#[derive(Deserialize)]
pub struct LoginPayload {
    email: String,
    password: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    token: String,
    user: User,
}

#[derive(Serialize)]
pub struct TokenResponse {
    access_token: String,
    refresh_token: String,
}

#[derive(Deserialize)]
pub struct RefreshTokenPayload {
    refresh_token: String,
}

#[derive(Serialize)]
pub struct RefreshTokenResponse {
    access_token: String,
}

pub async fn register(
    State(db): State<Database>,
    Json(payload): Json<RegisterPayload>,
) -> Result<Json<AuthResponse>, StatusCode> {
    // 检查邮箱是否已存在
    if db.get_user_by_email(&payload.email).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?.is_some() {
        return Err(StatusCode::CONFLICT);
    }

    // 密码加密
    let password_hash = hash(payload.password.as_bytes(), DEFAULT_COST)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // 创建新用户并设置7天Pro体验
    let mut user = User::new(payload.email, password_hash);
    user.vip_level = VipLevel::Pro;
    user.pro_experience_expiration = Some(OffsetDateTime::now_utc().unix_timestamp() + 7 * 24 * 60 * 60);
    db.create_user(&user).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // 生成token
    let token = jwt::create_token(&user.id)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(AuthResponse { token, user }))
}

pub async fn login(
    State(db): State<Database>,
    Json(payload): Json<LoginPayload>,
) -> Result<Json<AuthResponse>, StatusCode> {
    // 查找用户
    let user = db.get_user_by_email(&payload.email)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // 验证密码
    if !bcrypt::verify(payload.password.as_bytes(), &user.password_hash)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)? {
        return Err(StatusCode::UNAUTHORIZED);
    }

    // 生成token
    let token = jwt::create_token(&user.id)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(AuthResponse { token, user }))
}

pub async fn refresh_token(
    Json(payload): Json<RefreshTokenPayload>,
) -> Result<Json<RefreshTokenResponse>, StatusCode> {
    let access_token = jwt::refresh_access_token(&payload.refresh_token)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    Ok(Json(RefreshTokenResponse { access_token }))
}
