use axum::{
    extract::State,
    Json,
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    models::{AI, AIType, User},
    db::Database,
    middleware::auth::AuthenticatedUser,
};

#[derive(Deserialize)]
pub struct InitiateAIPayload {
    ai_type: AIType,
    name: Option<String>,
}

#[derive(Serialize)]
pub struct InitiateAIResponse {
    ai_id: String,
}

pub async fn initiate_ai(
    State(db): State<Database>,
    auth_user: AuthenticatedUser,
    Json(payload): Json<InitiateAIPayload>,
) -> Result<Json<InitiateAIResponse>, StatusCode> {
    let mut user = db.get_user_by_id(&auth_user.user_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    // 检查用户是否可以初始化特定类型的AI
    if !user.can_initiate_ai(&payload.ai_type, &db).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)? {
        return Err(StatusCode::FORBIDDEN);
    }

    // 生成AI名称
    let ai_name = payload.name.unwrap_or_else(|| 
        format!("AI-{}", Uuid::new_v4().to_string().split('-').next().unwrap())
    );

    // 初始化AI逻辑
    let ai = AI::new(
        ai_name,
        payload.ai_type.clone(),
        auth_user.user_id.clone(),
    );
    
    db.create_ai(&ai)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // 更新用户AI计数
    user.update_ai_count(&payload.ai_type);
    db.update_user(&user)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(InitiateAIResponse { ai_id: ai.id }))
}

pub async fn check_vip_status(
    State(db): State<Database>,
    auth_user: AuthenticatedUser,
) -> Result<StatusCode, StatusCode> {
    let mut user = db.get_user_by_id(&auth_user.user_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    // 处理VIP过期逻辑
    user.handle_vip_expiration();
    db.update_user(&user)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::OK)
}
