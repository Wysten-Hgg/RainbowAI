pub mod auth;
pub mod user;
pub mod ai;
pub mod coupon;
pub mod points;
pub mod store;
pub mod invite;
pub mod admin;

use axum::{
    Router,
    routing::{post, get},
    middleware,
};

use crate::db::Database;
use crate::middleware::auth::auth_middleware;

pub fn create_routes(db: Database) -> Router {
    let auth_routes = Router::new()
        .route("/register", post(auth::register))
        .route("/login", post(auth::login))
        .route("/refresh", post(auth::refresh_token))
        .route("/verify-email", post(auth::verify_email));

    let user_routes = Router::new()
        .route("/profile", get(user::get_profile))
        .route("/stats", get(user::get_stats))
        .route("/apply-for-promoter", post(user::apply_for_promoter))
        .route("/set_vip_config", post(user::set_vip_config))
        .layer(middleware::map_response(auth_middleware))
        .with_state(db.clone());

    let ai_routes = Router::new()
        .route("/initiate", post(ai::initiate_ai))
        .route("/check-vip-status", post(ai::check_vip_status))
        .layer(middleware::map_response(auth_middleware))
        .with_state(db.clone());

    let coupon_routes = Router::new()
        .route("/my", get(coupon::get_my_coupons))
        .route("/redeem", post(coupon::redeem_coupon))
        .route("/transfer", post(coupon::transfer_coupon))
        .route("/issue/admin", post(coupon::issue_coupon_admin))
        .layer(middleware::map_response(auth_middleware))
        .with_state(db.clone());

    let points_routes = Router::new()
        .route("/daily-checkin", post(points::daily_checkin))
        .route("/wallet/transactions", get(points::get_wallet_transactions))
        .route("/wallet/balance", get(points::get_wallet_info))
        .route("/gift/send", post(points::send_gift))
        .route("/gift/available", get(points::get_available_gifts))
        .route("/gift/sent", get(points::get_sent_gifts))
        .route("/gift/received/:ai_id", get(points::get_ai_received_gifts))
        .route("/lucky-card/use/:id", post(points::use_lucky_card))
        .route("/lucky-card/my", get(points::get_valid_lucky_cards))
        .nest("/points", points::points_routes())
        .layer(middleware::map_response(auth_middleware))
        .with_state(db.clone());

    // 添加商城路由
    let store_routes = Router::new()
        .merge(store::create_store_routes())
        .layer(middleware::map_response(auth_middleware))
        .with_state(db.clone());

    // 添加管理员商城路由
    let admin_store_routes = Router::new()
        .merge(store::create_admin_store_routes())
        .layer(middleware::map_response(auth_middleware))
        .with_state(db.clone());

    let invite_routes = Router::new()
        .route("/create", post(invite::create_invite))
        .route("/use", post(invite::use_invite));

    let admin_routes = Router::new()
        .route("/user/role", post(admin::update_user_role))
        .route("/audit-logs", get(admin::view_audit_logs));

    Router::new()
        .nest("/auth", auth_routes)
        .nest("/user", user_routes)
        .nest("/ai", ai_routes)
        .nest("/coupon", coupon_routes)
        .nest("/points", points_routes)
        .nest("/store", store_routes)
        .nest("/admin/store", admin_store_routes)
        .nest("/invite", invite_routes)
        .nest("/admin", admin_routes)
        .with_state(db)
}
