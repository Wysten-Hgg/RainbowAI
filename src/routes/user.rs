use axum::{
    extract::State,
    Json,
    http::StatusCode,
};
use serde::Serialize;

use crate::{
    db::Database,
    middleware::AuthenticatedUser,
    models::{User, PromoterType, FrontendUserRole},
};

#[derive(Serialize)]
pub struct ProfileResponse {
    user: User,
}

pub async fn get_profile(
    State(db): State<Database>,
    auth_user: AuthenticatedUser,
) -> Result<Json<ProfileResponse>, StatusCode> {
    let user = db.get_user_by_id(&auth_user.user_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(ProfileResponse { user }))
}

#[derive(Serialize)]
pub struct UserStatsResponse {
    total_ais: usize,
    available_slots: usize,
}

pub async fn get_stats(
    State(db): State<Database>,
    auth_user: AuthenticatedUser,
) -> Result<Json<UserStatsResponse>, StatusCode> {
    let user = db.get_user_by_id(&auth_user.user_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(UserStatsResponse {
        total_ais: user.awakened_ais.len(),
        available_slots: user.ai_slots as usize - user.awakened_ais.len(),
    }))
}

pub async fn apply_for_promoter(
    State(db): State<Database>,
    auth_user: AuthenticatedUser,
    Json(payload): Json<PromoterType>,
) -> Result<StatusCode, StatusCode> {
    let mut user = db.get_user_by_id(&auth_user.user_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    if user.frontend_roles.contains(&FrontendUserRole::Promoter) {
        return Err(StatusCode::CONFLICT);
    }

    user.apply_for_promoter(payload)
        .map_err(|_| StatusCode::FORBIDDEN)?;

    db.update_user(&user)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::OK)
}
