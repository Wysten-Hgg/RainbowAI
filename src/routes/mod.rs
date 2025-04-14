pub mod auth;
pub mod user;
pub mod ai;
pub mod invite;
pub mod admin;

use axum::{
    Router,
    routing::{post, get},
};

use crate::db::Database;

pub fn create_router(db: Database) -> Router {
    Router::new()
        .route("/auth/register", post(auth::register))
        .route("/auth/login", post(auth::login))
        .route("/auth/refresh", post(auth::refresh_token))
        .route("/user/profile", get(user::get_profile))
        .route("/user/stats", get(user::get_stats))
        .route("/user/apply-promoter", post(user::apply_for_promoter))
        .route("/ai/initiate", post(ai::initiate_ai))
        .route("/ai/check-vip-status", post(ai::check_vip_status))
        .route("/invite/create", post(invite::create_invite))
        .route("/invite/use", post(invite::use_invite))
        .route("/admin/user/role", post(admin::update_user_role))
        .route("/admin/audit-logs", get(admin::view_audit_logs))
        .with_state(db)
}
