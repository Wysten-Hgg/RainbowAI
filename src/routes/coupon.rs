
use axum::{
    Json,
    http::StatusCode,
    extract::State,
};

use crate::models::coupon::{Coupon, RedeemCouponPayload, TransferCouponPayload, IssueCouponPayload};
use crate::db::Database;
use std::sync::Arc;
use crate::middleware::auth::AuthenticatedUser;
use crate::models::user::{VipStatus, BackendUserRole};
use time::OffsetDateTime;

// 获取当前时间戳的辅助函数
fn now() -> i64 {
    OffsetDateTime::now_utc().unix_timestamp()
}

pub async fn get_my_coupons(
    State(db): State<Database>,
    auth_user: AuthenticatedUser
) -> Result<Json<Vec<Coupon>>, StatusCode> {
    // 查询用户的未过期卡券
    let coupons = db.get_user_coupons(&auth_user.user_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(coupons))
}

pub async fn redeem_coupon(
    State(db): State<Database>,
    auth_user: AuthenticatedUser,
    Json(payload): Json<RedeemCouponPayload>
) -> Result<StatusCode, StatusCode> {
    // 验证卡券并应用
    let mut coupon = db.get_coupon(&payload.coupon_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    if coupon.owner_id != auth_user.user_id || coupon.status != "active" {
        return Err(StatusCode::FORBIDDEN);
    }

    // 应用卡券逻辑（如体验券升级VIP）
    apply_coupon_logic(&mut coupon, &auth_user, db.clone()).await?;

    // 更新卡券状态
    coupon.status = "used".to_string();
    db.update_coupon(&coupon)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::OK)
}

pub async fn transfer_coupon(
    State(db): State<Database>,
    auth_user: AuthenticatedUser,
    Json(payload): Json<TransferCouponPayload>
) -> Result<StatusCode, StatusCode> {
    // 转赠卡券
    let mut coupon = db.get_coupon(&payload.coupon_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    if coupon.owner_id != auth_user.user_id || !coupon.is_transferable {
        return Err(StatusCode::FORBIDDEN);
    }

    coupon.owner_id = payload.new_owner_id.clone();
    db.update_coupon(&coupon)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::OK)
}

pub async fn issue_coupon_admin(
    State(db): State<Database>,
    auth_user: AuthenticatedUser,
    Json(payload): Json<IssueCouponPayload>
) -> Result<StatusCode, StatusCode> {
    // 验证管理员权限
    let admin = db.get_user_by_id(&auth_user.user_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::UNAUTHORIZED)?;

    if !admin.is_admin() {
        return Err(StatusCode::FORBIDDEN);
    }

    // 批量发放卡券
    for coupon_data in payload.coupons {
        let coupon = Coupon::new(
            coupon_data.id,
            coupon_data.coupon_type,
            coupon_data.sub_type,
            coupon_data.value,
            coupon_data.duration_days,
            coupon_data.owner_id,
            coupon_data.issued_at,
            coupon_data.expires_at,
            coupon_data.is_transferable,
        );
        db.create_coupon(&coupon)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }

    Ok(StatusCode::OK)
}

pub async fn apply_coupon_logic(coupon: &mut Coupon, auth_user: &AuthenticatedUser, db: Database) -> Result<(), StatusCode> {
    // 示例：根据卡券类型应用不同的逻辑
    match coupon.coupon_type.as_str() {
        "experience" => {
            // 应用体验券逻辑
            let mut user = db.get_user_by_id(&auth_user.user_id)
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
                .ok_or(StatusCode::NOT_FOUND)?;

            user.vip_schedule.push(VipStatus {
                level: coupon.sub_type.clone(),
                start: now(),
                end: now() + coupon.duration_days.unwrap_or(0) as i64,
            });

            db.update_user(&user)
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        }
        "discount" => {
            // 应用折扣券逻辑
        }
        "cash" => {
            // 应用现金券逻辑
        }
        _ => return Err(StatusCode::BAD_REQUEST),
    }
    Ok(())
}
