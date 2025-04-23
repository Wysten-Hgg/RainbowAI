use serde::{Serialize, Deserialize};
use uuid::Uuid;
use time::{OffsetDateTime, Duration};

#[derive(Serialize, Deserialize)]
pub struct Coupon {
    pub id: String,
    pub coupon_type: String,       // "experience", "discount", "cash"
    pub sub_type: String,          // 如 "pro_2d", "premium_7d", "95_discount", "cash_10"
    pub value: f32,                // 折扣或金额
    pub duration_days: Option<u32>,// 体验时长
    pub status: String,            // "active", "used", "expired"
    pub owner_id: String,          // 所属用户
    pub issued_at: String,
    pub expires_at: String,
    pub is_transferable: bool,
}

impl Coupon {
    pub fn new(id: String, coupon_type: String, sub_type: String, value: f32, duration_days: Option<u32>, owner_id: String, issued_at: String, expires_at: String, is_transferable: bool) -> Self {
        Self {
            id,
            coupon_type,
            sub_type,
            value,
            duration_days,
            status: "active".to_string(),
            owner_id,
            issued_at,
            expires_at,
            is_transferable,
        }
    }
    
    // 从模板创建卡券
    pub fn new_from_template(template: CouponTemplate, owner_id: String) -> Self {
        let now = OffsetDateTime::now_utc();
        let issued_at = now.to_string();
        
        // 计算过期时间
        let expires_at = match template.duration_days {
            Some(days) => {
                let expire_time = now + Duration::days(days as i64);
                expire_time.to_string()
            },
            None => "9999-12-31T23:59:59Z".to_string() // 无限期
        };
        
        Self {
            id: Uuid::new_v4().to_string(),
            coupon_type: template.coupon_type,
            sub_type: template.sub_type,
            value: template.value,
            duration_days: template.duration_days,
            status: "active".to_string(),
            owner_id,
            issued_at,
            expires_at,
            is_transferable: template.is_transferable,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct RedeemCouponPayload {
    pub coupon_id: String,
    pub user_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct TransferCouponPayload {
    pub coupon_id: String,
    pub new_owner_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct IssueCouponPayload {
    pub coupons: Vec<CouponData>,
}

#[derive(Serialize, Deserialize)]
pub struct CouponData {
    pub id: String,
    pub coupon_type: String,
    pub sub_type: String,
    pub value: f32,
    pub duration_days: Option<u32>,
    pub owner_id: String,
    pub issued_at: String,
    pub expires_at: String,
    pub is_transferable: bool,
}

// 卡券模板
#[derive(Serialize, Deserialize)]
pub struct CouponTemplate {
    pub id: String,
    pub name: String,
    pub description: String,
    pub coupon_type: String,
    pub sub_type: String,
    pub value: f32,
    pub duration_days: Option<u32>,
    pub is_transferable: bool,
    pub created_at: String,
    pub is_active: bool,
}
