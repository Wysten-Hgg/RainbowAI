use serde::{Serialize, Deserialize};
use uuid::Uuid;
use time::OffsetDateTime;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum GiftEffectType {
    Boost,      // 提升AI关系值
    Memory,     // 增强记忆点
    Exclusive,  // 解锁专属内容
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Gift {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub price_lc: u32,               // 光币价格
    pub emotional_value: u32,        // 情感价值
    pub effect_type: GiftEffectType,
    pub image_url: Option<String>,   // 礼物图片URL
    pub created_at: i64,
    pub is_limited: bool,            // 是否限定礼物
    pub available_until: Option<i64>, // 限定礼物的有效期
}

impl Gift {
    pub fn new(
        name: String,
        description: Option<String>,
        price_lc: u32,
        emotional_value: u32,
        effect_type: GiftEffectType,
        image_url: Option<String>,
        is_limited: bool,
        available_until: Option<i64>,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            description,
            price_lc,
            emotional_value,
            effect_type,
            image_url,
            created_at: OffsetDateTime::now_utc().unix_timestamp(),
            is_limited,
            available_until,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GiftRecord {
    pub id: String,
    pub gift_id: String,
    pub sender_id: String,
    pub receiver_ai_id: String,
    pub sent_at: i64,
    pub message: Option<String>,     // 赠送礼物时的留言
}

impl GiftRecord {
    pub fn new(
        gift_id: String,
        sender_id: String,
        receiver_ai_id: String,
        message: Option<String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            gift_id,
            sender_id,
            receiver_ai_id,
            sent_at: OffsetDateTime::now_utc().unix_timestamp(),
            message,
        }
    }
}
