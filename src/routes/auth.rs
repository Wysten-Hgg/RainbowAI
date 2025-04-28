use axum::{
    Json,
    http::StatusCode,
    extract::State,
};
use serde::{Deserialize, Serialize};
use bcrypt::{hash, DEFAULT_COST};
use time::{OffsetDateTime};

use crate::{models::{User, VipLevel, EmailVerification, VerificationType}, db::Database, utils::jwt, services::{EmailService, PromoterService}};

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

#[derive(Deserialize)]
pub struct VerifyEmailPayload {
    id: String,
    code: String,
}

#[derive(Serialize)]
pub struct VerifyEmailResponse {
    success: bool,
    message: String,
}

#[derive(Serialize)]
pub struct RegisterResponse {
    success: bool,
    message: String,
}

pub async fn register(
    State(db): State<Database>,
    Json(payload): Json<RegisterPayload>,
) -> Result<Json<RegisterResponse>, StatusCode> {
    // 检查邮箱是否已存在
    if db.get_user_by_email(&payload.email).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?.is_some() {
        return Err(StatusCode::CONFLICT);
    }
    
    // 密码加密
    let password_hash = hash(payload.password.as_bytes(), DEFAULT_COST)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    // 创建邮箱验证记录
    let verification = EmailVerification::new(
        payload.email.clone(),
        VerificationType::Registration
    );
    
    // 保存验证记录到数据库
    db.create_verification(&verification).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    // 创建新用户（未激活状态）
    let mut user = User::new(payload.email.clone(), password_hash);
    user.vip_level = VipLevel::Free; // 用户验证邮箱后才会升级为Pro
    user.is_email_verified = false;  // 标记为未验证
    
    // 如果有邀请码，设置邀请人信息并处理推广记录
    if let Some(invite_code) = payload.invite_code.clone() {
        user.invited_by = Some(invite_code.clone());
        
        // 保存用户到数据库
        db.create_user(&user).await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        
        // 处理推广记录
        let promoter_service = PromoterService::new(db.clone());
        let _ = promoter_service.process_invite_code(&user.id, &invite_code).await;
    } else {
        // 保存用户到数据库
        db.create_user(&user).await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }
    
    // 发送验证邮件
    match EmailService::new() {
        Ok(email_service) => {
            if let Err(_) = email_service.send_verification_email(&verification).await {
                // 邮件发送失败，但用户已创建，返回成功但提示邮件发送失败
                return Ok(Json(RegisterResponse {
                    success: true,
                    message: "用户注册成功，但验证邮件发送失败，请稍后重试".to_string(),
                }));
            }
        },
        Err(_) => {
            // 邮件服务初始化失败，但用户已创建，返回成功但提示邮件发送失败
            return Ok(Json(RegisterResponse {
                success: true,
                message: "用户注册成功，但验证邮件发送失败，请联系管理员".to_string(),
            }));
        }
    }
    
    Ok(Json(RegisterResponse {
        success: true,
        message: "用户注册成功，请查收验证邮件完成注册".to_string(),
    }))
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
    
    // 检查邮箱是否已验证
    if !user.is_email_verified {
        return Err(StatusCode::FORBIDDEN);
    }
    
    // 生成token
    let token = jwt::create_token(&user.id)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(AuthResponse { token, user }))
}

pub async fn verify_email(
    State(db): State<Database>,
    Json(payload): Json<VerifyEmailPayload>,
) -> Result<Json<VerifyEmailResponse>, StatusCode> {
    // 获取验证记录
    let verification = db.get_verification_by_id(&payload.id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;
    
    // 检查验证码是否匹配
    if verification.code != payload.code {
        return Ok(Json(VerifyEmailResponse {
            success: false,
            message: "验证码不正确".to_string(),
        }));
    }
    
    // 检查验证记录是否有效
    if !verification.is_valid() {
        return Ok(Json(VerifyEmailResponse {
            success: false,
            message: "验证链接已过期，请重新注册".to_string(),
        }));
    }
    
    // 检查验证类型
    if let VerificationType::Registration = verification.verification_type {
        // 查找用户
        let mut user = db.get_user_by_email(&verification.email)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            .ok_or(StatusCode::NOT_FOUND)?;
        
        // 更新用户状态为已验证，并设置7天Pro体验
        user.is_email_verified = true;
        user.vip_level = VipLevel::Pro;
        user.pro_experience_expiration = Some(OffsetDateTime::now_utc().unix_timestamp() + 7 * 24 * 60 * 60);
        
        // 保存用户更新
        db.update_user(&user)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        
        // 标记验证记录为已使用
        db.mark_verification_used(&verification.id)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        
        return Ok(Json(VerifyEmailResponse {
            success: true,
            message: "邮箱验证成功，您已获得7天Pro会员体验".to_string(),
        }));
    }
    
    Ok(Json(VerifyEmailResponse {
        success: false,
        message: "无效的验证类型".to_string(),
    }))
}

pub async fn refresh_token(
    Json(payload): Json<RefreshTokenPayload>,
) -> Result<Json<RefreshTokenResponse>, StatusCode> {
    let access_token = jwt::refresh_access_token(&payload.refresh_token)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;
    
    Ok(Json(RefreshTokenResponse { access_token }))
}
