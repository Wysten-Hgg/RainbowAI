pub mod auth;
pub mod user;
pub mod ai;
pub mod invite;
pub mod admin;
pub mod coupon;
pub mod points;

use axum::{
    Router,
    routing::{post, get},
};


use crate::db::Database;

pub fn create_routes(db:Database) -> Router {
    Router::new()
        .route("/auth/register", post(auth::register))
        .route("/auth/login", post(auth::login))
        .route("/auth/refresh", post(auth::refresh_token))
        .route("/auth/verify-email", post(auth::verify_email))
        .route("/user/profile", get(user::get_profile))
        .route("/user/stats", get(user::get_stats))
        .route("/user/apply-promoter", post(user::apply_for_promoter))
        .route("/admin/set_vip_config", post(user::set_vip_config))
        .route("/ai/initiate", post(ai::initiate_ai))
        .route("/ai/check-vip-status", post(ai::check_vip_status))
        .route("/invite/create", post(invite::create_invite))
        .route("/invite/use", post(invite::use_invite))
        .route("/admin/user/role", post(admin::update_user_role))
        .route("/admin/audit-logs", get(admin::view_audit_logs))
        .route("/coupon/my", get(coupon::get_my_coupons))
        .route("/coupon/redeem", post(coupon::redeem_coupon))
        .route("/coupon/transfer", post(coupon::transfer_coupon))
        .route("/coupon/issue/admin", post(coupon::issue_coupon_admin))
        .nest("/points", points::points_routes())
        .with_state(db)
}
